//! The real `reactor`: the operating system's two clocks, and THE WAIT that actually blocks.
//!
//! The division of labour is the one that makes the simulator possible (§1.3): the kernel owns
//! the DECISION of which activity to advance, this file owns the WAIT. Nothing here decides
//! anything, which is why the same executor runs unchanged against `VirtualReactor`.
//!
//! What holds this implementation and the fake to the same promises is the conformance suite
//! in `crates/kernel/tests/reactor_contract.rs`, run against this type from
//! `crates/platform/tests/reactor_contract_real.rs`. It is written once and run twice on
//! purpose: the fake is not the real one, and a campaign against the fake is worth exactly as
//! much as the evidence that the two answer the same contract.

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use kernel::ports::reactor::Reactor;
use kernel::time::{Monotonic, WallTime};

/// The `reactor` port against the real machine.
pub struct SystemReactor {
    /// The instant this reactor was built, and THE ONLY ORIGIN IT HAS. `Instant` deliberately
    /// exposes no epoch — it is opaque by design, precisely so that nobody reads absolute
    /// meaning into a monotonic value — so the mapping onto `Monotonic::ORIGIN` has to be
    /// anchored to a moment this type chooses itself. `Monotonic`'s own documentation says the
    /// same thing from the other side: "the origin is arbitrary and carries no meaning — only
    /// differences do".
    origin: Instant,
}

impl SystemReactor {
    pub fn new() -> Self {
        SystemReactor {
            origin: Instant::now(),
        }
    }
}

// ⛔ NO `impl Default`, AND ITS ABSENCE IS THE DECISION. It had no caller anywhere in the
// workspace, and this repository removes such items rather than keeping them for symmetry —
// `Millis::ZERO`, `Monotonic::as_millis` and a `?Sized` bound on `RngExt` all went the same
// way, and `tests/compile_fail/parameters_have_no_default.rs` exists to keep a `Default` OFF
// another type. Add it back on the day something calls it.
//
// ⚠️ CLIPPY DOES ASK FOR ONE, and the warning is accepted rather than silenced — measured, not
// assumed: `cargo clippy --workspace --all-targets` emits `new_without_default` for this type.
// It emits the SAME warning for `simulator`'s `VirtualReactor`, which has never had a `Default`
// either, so removing this one makes the two reactors consistent instead of singling this one
// out. §7.4.3 settles the tie: clippy "has NO voice here — no V depends on it", so it does not
// get to reintroduce an item with no callers. No `#[allow]` is added: a suppressed warning
// hides the next occurrence too, and `VirtualReactor` does not suppress it.
//
// ⚠️ `SequentialRng` in this same crate DOES keep its `Default`, and that is not an
// inconsistency: it has a caller — `a_fresh_generator_starts_from_zero` pins it, with the
// reason written next to it.

impl Reactor for SystemReactor {
    fn now(&self) -> Monotonic {
        // ⚠️ THE CAST IS A TRUNCATION, and it is declared rather than hidden: `as_millis`
        // yields a `u128`, and `as u64` would WRAP — a monotonic clock jumping backwards,
        // which is the one thing this type may not do. It needs the process to have been up
        // for 2^64 milliseconds, some 584 million years, so it is unreachable rather than
        // guarded: a `saturating` conversion here would be a branch no run can take.
        Monotonic::from_millis(self.origin.elapsed().as_millis() as u64)
    }

    fn wall_time(&self) -> WallTime {
        // ⛔ ON ERROR IT RETURNS THE EPOCH INSTEAD OF PANICKING, and that is a deliberate
        // trade: a record carrying a wrong stamp can be recovered afterwards, a core that dies
        // while stamping cannot. `duration_since` fails only if the system clock sits BEFORE
        // the Unix epoch, which is a machine misconfigured rather than a program in error.
        //
        // ⚠️ AND THIS CLOCK CAN GO BACKWARDS — NTP, daylight saving, the user setting it — so
        // two readings taken in order can compare in the opposite order. That is EXACTLY why
        // §2.1 lets no kernel decision read it: what it feeds is the record (journal stamps,
        // Q14), never a deadline, a grant window or a timeout. Those read `now()`, above.
        //
        // ⚠️ AND `as u64` IS THE SAME `u128` TRUNCATION `now()` declares above — said here too
        // rather than left to the reader to notice the symmetry. It is just as unreachable:
        // 2^64 milliseconds after 1970 is the year 584 million, so no wrap can occur before
        // then, and a saturating conversion would be a branch no run can take.
        WallTime::from_millis_since_epoch(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|since_epoch| since_epoch.as_millis() as u64)
                .unwrap_or(0),
        )
    }

    fn wait_until(&mut self, deadline: Monotonic) -> Option<Monotonic> {
        // ⛔ STRICTLY IN THE FUTURE, and `None` otherwise. A null advance reported as a
        // successful one is an infinite loop one level up: the executor would take the same
        // already-past deadline for ever and never let anything progress (§3.2.1, gotcha #19).
        let now = self.now();
        if deadline <= now {
            return None;
        }

        // `thread::sleep` is documented never to sleep LESS than asked — it may sleep longer.
        // That is the direction this contract needs: the wait must reach at least the
        // deadline, and overshooting is allowed.
        //
        // ⚠️ Readiness is the other half of the port's contract and has no producer yet:
        // nothing in this milestone generates external events, so today every wait that
        // returns `Some` ran to its deadline. §0.4.3 declares where such a source will enter.
        std::thread::sleep(Duration::from_millis(deadline.saturating_since(now).get()));

        // The clock is read AGAIN rather than `deadline` being handed back: what the caller
        // needs is where the clock REALLY is, not the instant it asked to arrive at.
        //
        // ⛔ DECLARED RESIDUAL, so that this comment does not promise more than it delivers:
        // NOTHING CHECKS IT. Replacing `self.now()` with `deadline` leaves every test in the
        // repository green, and the conformance suite cannot catch it either — ON THE FAKE THE
        // TWO EXPRESSIONS COINCIDE, because a virtual clock lands exactly on the deadline it
        // was handed. So the suite that runs against both cannot tell them apart on either.
        //
        // ⚠️ AND NO TEST WAS WRITTEN FOR IT, ON PURPOSE. The difference is observable only
        // through the OVERSHOOT of the operating system's sleep, whose granularity no platform
        // guarantees — on Windows it is around 15 ms by custom, not by contract. A probe built
        // on it would be green by luck and red by bad luck, and a control that fires where it
        // must not is worse than one that is absent (§7.1.1 rule 3). This repository prefers a
        // residual that is DECLARED to a check that is non-deterministic.
        Some(self.now())
    }
}
