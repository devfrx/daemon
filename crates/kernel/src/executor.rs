//! The executor (§2.4). It lives in `kernel`, and that is the whole point of tie-break #1
//! in ADR-0026: THE ORDER OF THE CONCURRENT UNITS IS DECIDED HERE, and it holds outside the
//! tests too.
//!
//! # The rule that makes the rest possible (§2.4.1)
//!
//! > An activity of the kernel suspends ONLY on a primitive of the executor or on a port.
//!
//! It is not a preference. A bespoke waker — the ticket saying "call me when I am ready" —
//! is NOT BUILDABLE inside the kernel: `Waker::from_raw` is an unsafe function and
//! `#![forbid(unsafe_code)]` refuses it. Measured in M-5: `E0133: call to unsafe function`.
//!
//! So the executor must know by itself who can advance, and it does because readiness has
//! exactly two sources:
//!
//! | Source                                         | Who knows it                  |
//! |------------------------------------------------|-------------------------------|
//! | internal — yields, queues, waits between tasks | the EXECUTOR, which owns them |
//! | external — I/O, timers, IPC, workers           | the `Reactor` port            |
//!
//! # One decision at a time
//!
//! Not a renunciation of parallelism: the heavy work lives in the WORKERS, which are
//! separate processes (ADR-0004), and the heavy-but-systemic operations live behind
//! `platform`'s ports, which may use threads of their own. What is bought is the removal of
//! a class of defect: ADR-0004 describes the arbiter as "a single process with a single
//! lock", and with one decision at a time THAT LOCK DOES NOT EXIST.

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};

use crate::parameters::Parameters;
use crate::ports::reactor::Reactor;
// ⛔ BOTH traits, and it is not tidiness: `next_u64` lives on `Rng` and the reduction
// `below` lives on `RngExt`, deliberately split so that no implementation can reduce
// differently. A method reached through a blanket impl needs its trait IN SCOPE, so
// importing `Rng` alone leaves `below` unresolved (E0599) even though `R: Rng` proves
// `R: RngExt`. The cost is declared in `crate::rng`: callers import two names.
use crate::rng::{Rng, RngExt};
use crate::time::Monotonic;

/// Why a run stopped without finishing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunError {
    /// The turn limit was reached. A BLOCK MUST SHOW UP AS AN ERROR, never as an infinite
    /// wait: a test that never ends says nothing (§3.2.1).
    ///
    /// It is the backstop for every way an activity can fail to progress under its own
    /// power — a loop that yields for ever, or one that keeps re-registering a deadline the
    /// clock has already passed. Both are slow loops, and both end here.
    TurnLimitReached,
    /// The reactor was asked to advance to an instant STRICTLY IN THE FUTURE and refused.
    /// The `reactor` contract forbids that: `wait_until` returns `None` only when there is
    /// nothing to wait for, and a deadline strictly ahead of `now` is something to wait for.
    /// The executor fails closed rather than spinning on a port that will not move.
    ///
    /// ⚠️ UNREACHABLE WITH A CONFORMING REACTOR, and writing which of the two it is beats
    /// leaving it to be guessed: this is a FAIL-CLOSED GUARD against an implementation that
    /// breaks its own contract, not a state a correct run can enter. It is a variant rather
    /// than a comment because it is TESTABLE — the conformance suite of §7.4.6 builds a
    /// deliberately broken reactor, and that is the only thing that can produce it.
    ///
    /// ⛔ IT IS NOT A DEADLOCK, and an earlier draft called it `Stalled` and meant exactly
    /// that. The rename came with the fix: an activity whose deadline has already passed is
    /// READY, not blocked, so it is promoted and polled. What is left here is only the port
    /// misbehaving.
    ReactorWillNotAdvance,
}

/// What an activity is doing, from the executor's point of view.
///
/// This is decision D3 of the milestone 2 plan, and it is what makes §3.2's rule
/// enforceable: without it, an activity that YIELDS is indistinguishable from one that
/// WAITS, and the clock would move forward while somebody could still work.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TaskState {
    /// Can be polled right now.
    Runnable,
    /// Suspended until an instant. Only the reactor can bring it back.
    Sleeping(Monotonic),
}

struct Task<'a> {
    future: Pin<Box<dyn Future<Output = ()> + 'a>>,
    state: TaskState,
}

/// The suspension request an activity leaves behind when it wants to sleep.
///
/// ⚠️ It is a shared cell and not a return value because a `Future` returning `Pending`
/// cannot say WHY. The activity writes here, the executor reads. This is the only channel,
/// and §2.4.1 is what keeps it the only one.
///
/// ⛔ ONE CELL FOR THE WHOLE EXECUTOR, and the two limits that follow are declared rather
/// than discovered:
///
/// - the cell is CLEARED BEFORE and READ AFTER every poll, so what the executor reads was
///   written during that poll — by the activity that has just run, and by nobody else.
///   ⚠️ DATED RECALL, 2026-08-18: this line used to say "read and DRAINED after every poll,
///   so it always belongs to the activity that has just run". The clause was FALSE — after
///   is enough to exclude the PREVIOUS activity (V31) and nothing else, so a write made
///   while no poll was running was honoured on whichever activity the seed polled next.
///   Finding K-1 of the 2026-08-11 audit; the clearing before the poll is what makes the
///   sentence true rather than aspirational;
/// - within a single poll the LAST write wins. An activity that suspended on two deadlines
///   at once would keep only the second, and it would be the wrong one whenever the first is
///   earlier. Nothing in this milestone can do that — there is no combinator that polls two
///   suspending futures — and the day one exists it does not get to reuse this cell.
pub struct Sleep {
    until: core::cell::Cell<Option<Monotonic>>,
}

// ⛔ NO `impl Default`, AND ITS ABSENCE IS THE DECISION — the same one, for the same reason, as
// `SystemReactor`, `VirtualReactor` and `MemoryJournal`: nothing calls it, and this repository
// removes such items rather than keeping them for symmetry. `cargo clippy` asks for one
// (`new_without_default`); the warning is ACCEPTED and NOT silenced, because §7.4.3 gives clippy
// no voice in the gate and an `#[allow]` would hide the next occurrence too. The argument is
// written out once, in `crates/platform/src/reactor.rs`, and this comment points at it rather
// than restating it.
//
// ⚠️ REMOVED 2026-08-27, finding AUD-048: the impl was here, `Sleep::default()` had no caller
// anywhere in the workspace, and it was the ONE site of the four where clippy had been obeyed —
// against the rule `crates/kernel/src/boundary.rs` states for this crate, "an API item with no
// caller is deleted in this repository". Four hand-written exceptions elsewhere say why THEY
// stay; this one had no reason, and an undeclared survivor is what makes the other four
// unreadable.
impl Sleep {
    pub const fn new() -> Self {
        Sleep {
            until: core::cell::Cell::new(None),
        }
    }

    /// Declare that the calling activity is suspended until `deadline`.
    ///
    /// ⛔ A DEADLINE ALREADY REACHED IS NOT AN ERROR — the wait is simply over. The executor
    /// promotes the activity and polls it again WITHOUT touching the clock, so a zero-length
    /// sleep behaves as a yield rather than stopping the run.
    ///
    /// ⚠️ WHY IT HAS TO WORK THIS WAY, and the reason is structural rather than a kindness:
    /// A FUTURE CANNOT READ THE CLOCK — it holds no reactor — so an activity computing an
    /// absolute deadline has no way of checking that the instant still lies ahead by the
    /// time it is polled. Under the opposite rule any deadline that elapsed while the other
    /// activities ran would kill the whole run. That is a trap, not a property.
    ///
    /// 📌 §3.2.1 GOVERNS THE REACTOR, NOT THIS, and conflating the two is how the opposite
    /// rule got written in the first place. What that section rules is that `advance()`
    /// filters strictly future deadlines and returns false when there are none, because A
    /// NULL ADVANCE MUST NEVER BE DECLARED SUCCESSFUL — a rule about the PORT refusing to
    /// lie about the clock. It is honoured at the call site of `wait_until`, which is never
    /// handed an instant that is not strictly ahead. It says nothing about what the executor
    /// owes an activity whose wait is already over.
    ///
    /// ⚠️ THE COST, declared: an activity that re-registers a past deadline on every poll
    /// never blocks the clock, but never progresses either. It ends as
    /// `RunError::TurnLimitReached` — "a slow loop", which is the accurate diagnosis and the
    /// reason the turn limit exists.
    pub fn until(&self, deadline: Monotonic) {
        self.until.set(Some(deadline));
    }

    fn take(&self) -> Option<Monotonic> {
        self.until.take()
    }
}

pub struct Executor<'a, R: Rng, C: Reactor> {
    tasks: Vec<Task<'a>>,
    rng: R,
    reactor: C,
    turn_limit: u64,
    sleep: &'a Sleep,
}

impl<'a, R: Rng, C: Reactor> Executor<'a, R, C> {
    /// ⛔ `parameters` is NOT optional and has no default: §2.8.2 rule 2 says the kernel
    /// cannot name a file, a key or a default. The negative test is
    /// `tests/compile_fail/executor_without_parameters.rs`.
    pub fn new(rng: R, reactor: C, parameters: Parameters, sleep: &'a Sleep) -> Self {
        Executor {
            tasks: Vec::new(),
            rng,
            reactor,
            turn_limit: parameters.executor_turn_limit(),
            sleep,
        }
    }

    pub fn spawn(&mut self, future: impl Future<Output = ()> + 'a) {
        self.tasks.push(Task {
            future: Box::pin(future),
            state: TaskState::Runnable,
        });
    }

    /// The instant the reactor is currently at. Handed to activities so that they can
    /// compute their own deadlines without reading a clock of their own.
    pub fn now(&self) -> Monotonic {
        self.reactor.now()
    }

    /// Run until every activity has finished.
    ///
    /// One TURN is: poll every `Runnable` activity exactly once, in an order chosen by the
    /// seed (decision D4). Only when NO activity is `Runnable` does the reactor get to move
    /// the clock — which is §3.2's rule verbatim: "while a ready activity exists, time
    /// stands still".
    pub fn run(&mut self) -> Result<(), RunError> {
        let mut turns: u64 = 0;

        while !self.tasks.is_empty() {
            turns += 1;
            if turns > self.turn_limit {
                return Err(RunError::TurnLimitReached);
            }

            if self.poll_one_turn() {
                continue;
            }

            // Nobody can work — every remaining activity is `Sleeping`.
            //
            // ⛔ FIRST, A WAIT THAT IS ALREADY OVER IS NOT A WAIT. An activity whose deadline
            // the clock has already reached is READY, so it is promoted and polled, and the
            // clock is not touched. Doing otherwise looks defensible and is a trap: a future
            // cannot read the clock, so it cannot avoid registering a deadline that elapses
            // while the others run, and the whole run would die of it. See `Sleep::until`.
            let now = self.reactor.now();
            if self.wake_those_due_at(now) {
                continue;
            }

            // Only now, with every sleeper strictly ahead of `now`, is there something to
            // wait FOR. Take the earliest and let the reactor move the clock to it.
            //
            // ⛔ "Strictly" is the precondition of `wait_until`, and it is §3.2.1's rule at
            // the one boundary where it binds: the port must never report a null advance as
            // a successful one — the trap the spike's first draft walked into by taking the
            // minimum of ALL registered deadlines, finished activities included, so that the
            // minimum fell in the past and the executor spun for ever.
            //
            // ⚠️ The filter cannot exclude anything today — the promotion above has already
            // removed every candidate it would reject — and it stays because it states that
            // precondition where the call is made rather than three lines away.
            let earliest = self
                .tasks
                .iter()
                .filter_map(|task| match task.state {
                    TaskState::Sleeping(deadline) if deadline > now => Some(deadline),
                    _ => None,
                })
                .min();

            // ⚠️ `earliest` is always `Some` here: the loop guard says the vector is not
            // empty, `poll_one_turn` returned false so nothing is `Runnable`, and the
            // promotion above returned false so every sleeper is strictly ahead. Were that
            // ever to stop holding, the turn limit ends the run with an error instead of
            // hanging it — which is why this is an `if let` and not a second error variant
            // that no reachable state produces.
            if let Some(deadline) = earliest {
                let Some(reached) = self.reactor.wait_until(deadline) else {
                    return Err(RunError::ReactorWillNotAdvance);
                };

                // ⚠️ `reached` and not `deadline`: the port may come back EARLY, because an
                // external event beat the deadline. Promoting by the instant actually
                // reached is what keeps that honest — an early return wakes nobody, and the
                // next turn asks again. Nothing produces external events in this milestone,
                // so today every wait that returns `Some` ran to its deadline.
                self.wake_those_due_at(reached);
            }
        }

        Ok(())
    }

    /// Promotes every activity whose deadline `instant` has reached, and says whether any
    /// was. THE ONLY PLACE a sleeper becomes runnable, called from the two points that mean
    /// different things: `now`, where the wait turns out to be already over, and the instant
    /// the reactor came back at, where the clock has just moved.
    fn wake_those_due_at(&mut self, instant: Monotonic) -> bool {
        let mut woken = false;
        for task in &mut self.tasks {
            if let TaskState::Sleeping(until) = task.state
                && until <= instant
            {
                task.state = TaskState::Runnable;
                woken = true;
            }
        }
        woken
    }

    /// Polls every `Runnable` activity once, in an order chosen by the seed. Returns whether
    /// at least one was polled — which is what "somebody could work" means.
    fn poll_one_turn(&mut self) -> bool {
        let order = self.runnable_order();
        if order.is_empty() {
            return false;
        }

        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        let mut finished: Vec<usize> = Vec::new();

        // Every index in `order` is still `Runnable` when it comes up, and no index appears
        // twice: nothing inside a poll can reach the executor (§2.4.1), the only state this
        // loop writes is the polled activity's own, and completed activities leave the vector
        // only after the loop. So no activity is polled twice in a turn, and none is polled
        // after finishing.
        for index in order {
            // ⛔ CLEARED BEFORE THE POLL TOO, and this is finding K-1 of the 2026-08-11
            // audit. Draining AFTER a poll only defends against the PREVIOUS ACTIVITY; it
            // says nothing about a write made while no poll was running, and there are two
            // such moments — before `run` (anything holding `&Sleep` may write) and after
            // this loop, where a finished task's DESTRUCTOR runs.
            //
            // 📌 What the line buys is not a third patch but a CHANGE OF INVARIANT: from
            // "nobody may ever write outside a poll", which nothing can enforce, to "only
            // what is written DURING this poll counts", which is enforced right here. Every
            // entry path closes at one point, including any not yet imagined.
            //
            // ⚠️ Draining at the entry of `run` was the remedy §8 of the audit proposed. It
            // was measured on 2026-08-18 and is NEITHER SUFFICIENT — the destructor path
            // still reached the clock — NOR well aimed.
            self.sleep.take();
            let outcome = self.tasks[index].future.as_mut().poll(&mut context);
            // ⛔ Drained after EVERY poll, the completed ones included. A request left behind
            // by an activity that then returned `Ready` would otherwise still be in the cell
            // when the NEXT activity is polled, and that activity would go to sleep on a
            // deadline that was never its own.
            let requested = self.sleep.take();
            match outcome {
                Poll::Ready(()) => finished.push(index),
                Poll::Pending => {
                    // If the activity asked to sleep, honour it. If it did not, it yielded:
                    // it stays `Runnable` and gets polled again next turn.
                    if let Some(deadline) = requested {
                        self.tasks[index].state = TaskState::Sleeping(deadline);
                    }
                }
            }
        }

        // Descending, so that removing does not shift the indices still to be removed.
        finished.sort_unstable();
        for index in finished.into_iter().rev() {
            self.tasks.remove(index);
        }

        true
    }

    /// The indices of the runnable activities, shuffled with the seed.
    ///
    /// A Fisher-Yates shuffle: every permutation is reachable, and the seed alone decides
    /// which. This is the single point at which the interleaving is chosen.
    fn runnable_order(&mut self) -> Vec<usize> {
        let mut order: Vec<usize> = self
            .tasks
            .iter()
            .enumerate()
            .filter(|(_, task)| task.state == TaskState::Runnable)
            .map(|(index, _)| index)
            .collect();

        let mut remaining = order.len();
        while remaining > 1 {
            let picked = self.rng.below(remaining as u64) as usize;
            remaining -= 1;
            order.swap(picked, remaining);
        }
        order
    }
}
