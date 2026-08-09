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
    TurnLimitReached,
    /// No activity can advance and the reactor has nothing to advance to. Distinct from
    /// `TurnLimitReached`: this one is a deadlock, that one is a slow loop.
    ///
    /// ⚠️ IT ALSO COVERS A CASE THAT IS NOT A DEADLOCK, and saying so here is cheaper than
    /// letting somebody find it: an activity suspended on a deadline that is NOT STRICTLY IN
    /// THE FUTURE — the current instant included, so a zero-length sleep — stops the whole
    /// run with this error instead of being polled again. See `Sleep::until`, which carries
    /// the caller's half of the contract.
    Stalled,
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
/// - the request is read and DRAINED after every poll, so it always belongs to the activity
///   that has just run — never to the previous one;
/// - within a single poll the LAST write wins. An activity that suspended on two deadlines
///   at once would keep only the second, and it would be the wrong one whenever the first is
///   earlier. Nothing in this milestone can do that — there is no combinator that polls two
///   suspending futures — and the day one exists it does not get to reuse this cell.
pub struct Sleep {
    until: core::cell::Cell<Option<Monotonic>>,
}

impl Sleep {
    pub const fn new() -> Self {
        Sleep {
            until: core::cell::Cell::new(None),
        }
    }

    /// Declare that the calling activity is suspended until `deadline`.
    ///
    /// ⛔ THE CALLER'S HALF OF THE CONTRACT: `deadline` must be STRICTLY LATER than the
    /// instant the run is at. A deadline already reached cannot move the clock, so the
    /// executor refuses to treat it as a wait and stops with `RunError::Stalled` — it does
    /// not promote the activity, and it does not spin. That is §3.2.1's rule taken at its
    /// word: a null advance is never reported as a successful one.
    ///
    /// ⚠️ THE COST IS REAL AND IT IS PAID BY THE CALLER: a future cannot read the clock — it
    /// holds no reactor — so an activity computing an absolute deadline has to obtain it
    /// from `Executor::now` at a point where it can still be sure the instant lies ahead. A
    /// zero-length sleep is NOT a no-op here: it stops the run.
    ///
    /// ⚠️ This is a runtime rule and this comment does not pretend otherwise. No compiler
    /// stops a caller from passing a past instant; what holds it is the level 2 case
    /// `a_deadlock_is_stalled_and_not_a_null_advance`, which proves the executor REPORTS the
    /// situation rather than spinning on it.
    pub fn until(&self, deadline: Monotonic) {
        self.until.set(Some(deadline));
    }

    fn take(&self) -> Option<Monotonic> {
        self.until.take()
    }
}

impl Default for Sleep {
    fn default() -> Self {
        Sleep::new()
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

            // Nobody can work. Find the earliest deadline STRICTLY IN THE FUTURE and let the
            // reactor take us there.
            //
            // ⛔ "Strictly": the first draft of the spike took the minimum of ALL registered
            // deadlines, including those of finished tasks, so the minimum fell in the past,
            // the clock did not move, and the function declared success anyway. The executor
            // spun forever. §3.2.1.
            let now = self.reactor.now();
            let earliest = self
                .tasks
                .iter()
                .filter_map(|task| match task.state {
                    TaskState::Sleeping(deadline) if deadline > now => Some(deadline),
                    _ => None,
                })
                .min();

            let Some(deadline) = earliest else {
                return Err(RunError::Stalled);
            };
            let Some(reached) = self.reactor.wait_until(deadline) else {
                return Err(RunError::Stalled);
            };

            // ⚠️ `reached` and not `deadline`: the port may come back EARLY, because an
            // external event beat the deadline. Promoting by the instant actually reached is
            // what keeps that honest — an early return wakes nobody, and the next turn asks
            // again. Nothing produces external events in this milestone, so today every wait
            // that returns `Some` ran to its deadline.
            for task in &mut self.tasks {
                if let TaskState::Sleeping(until) = task.state
                    && until <= reached
                {
                    task.state = TaskState::Runnable;
                }
            }
        }

        Ok(())
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
