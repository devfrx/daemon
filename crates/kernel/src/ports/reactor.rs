//! The `reactor` port: "what is ready", and THE WAIT (§2.4).
//!
//! The division of labour is the one that makes the simulator possible (§1.3):
//!
//! | Who        | What it owns                                                |
//! |------------|-------------------------------------------------------------|
//! | `kernel`   | THE DECISION of which concurrent activity to advance        |
//! | `platform` | THE WAIT for something to be ready, i.e. the call to the OS |
//!
//! Separating them is what lets the simulator be deterministic WITHOUT REIMPLEMENTING THE
//! LOGIC: it makes the wait instantaneous while the decision stays the real one. In
//! simulation "what is ready" and "how much time passes" are both decided BY THE SEED, and
//! that is how time becomes virtual (C3). Everything the campaign exercises on this side of
//! the boundary is therefore the production code, not a rehearsal of it.
//!
//! ⚠️ THE TWO CLOCKS LIVE ON THIS PORT, AND THAT DOES NOT CREATE A SEVENTH FAMILY (decision
//! D2 of the milestone 2 plan). Reading a clock IS I/O, and `reactor` is the port of time
//! and readiness — §3.1 already assigns it "moves the virtual clock forward". A separate
//! `clock` family would split one source of virtual time across two ports the simulator
//! would then have to keep in step, which is more machinery for less determinism.
//!
//! ⛔ Which of the two a caller may read is NOT this port's rule to make, and this comment
//! does not restate it as though it were. §2.1 — "no kernel decision depends on wall time" —
//! is held at level 1 by the type separation in `crate::time`, and by the four cases in
//! `tests/compile_fail/` that guard it in both directions. Those bite at EVERY site where a
//! `Monotonic` is expected, `wait_until`'s deadline included: handing out both clocks from
//! one trait adds a call site to a mechanism that is already general, not a hole in it.

use crate::time::{Monotonic, WallTime};

/// What a wait resumed on.
///
/// ⛔ ONE VARIANT, AND IT IS A DECISION RATHER THAN AN OVERSIGHT. The milestone 2 plan
/// drafted a second one — `EventReady(Monotonic)`, for a wait cut short by an external
/// event — and it was removed before it was written. Three reasons, and the third settles
/// it:
///
/// - nothing in this milestone can produce it. No port generates external events yet, which
///   the plan itself lists among what the milestone leaves open. An API item no caller
///   consumes does not enter here — the same rule that already removed `Millis::ZERO`,
///   `Monotonic::as_millis` and a `?Sized` bound from this crate;
/// - what regola B of §0.4.3 buys is written there in its own words: *"here we declare WHERE
///   a source enters, not HOW it works"*, and *"what this section buys is that the day it is
///   built, no new port is born"*. The port is the thing that had to exist now, and it does.
///   The shape of what the wait returns is the "how", which that section excludes explicitly
///   — adding that the conformance suite "does not cover an operation nobody calls";
/// - ⛔ AND THE VARIANT AS DRAFTED COULD NOT HAVE BEEN ACTED ON. The executor holds two task
///   states, `Runnable` and `Sleeping(deadline)`, and on resuming it promotes the activities
///   whose deadline the instant has passed. A variant carrying only an INSTANT identifies
///   nothing, so it could not promote the activity the event was for: the day events exist
///   it will have to carry WHICH REGISTRATION became ready. Declaring it now would freeze a
///   shape already known to be the wrong one — ADR-0009's rule, that a minimal contract can
///   be widened and a rich wrong one cannot.
///
/// ⚠️ And the argument for declaring it early inverts its own sign. "Adding a variant later
/// turns every `match` into a compile error" describes THE MECHANISM THAT FINDS EVERY SITE
/// THAT MUST DECIDE, not a hazard to be pre-empted; `at` below keeps its `match` so that the
/// second variant is refused HERE first. The analogous rule that does exist — ADR-0036 rule
/// 3, a new field is optional and takes a new index — governs DURABLE records, where bytes
/// already written cannot be recompiled. Nothing on this port is durable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wakeup {
    /// The wait reached the requested instant. Nothing external happened.
    DeadlineReached(Monotonic),
}

impl Wakeup {
    /// The instant the wait resumed at.
    pub const fn at(self) -> Monotonic {
        match self {
            Wakeup::DeadlineReached(instant) => instant,
        }
    }
}

/// The port of time and readiness. `platform` implements it against the OS; `simulator`
/// implements the same trait against a virtual clock governed by the seed (§2.4). The
/// conformance suite of §7.4.6 runs against BOTH, and is the reason the trait says what it
/// says instead of describing one of them.
pub trait Reactor {
    /// The current monotonic instant. This is what DECISIONS read.
    fn now(&self) -> Monotonic;

    /// The current time in the world. ONLY the record reads it — Q14, journal stamps.
    fn wall_time(&self) -> WallTime;

    /// Wait until `deadline`, or until an external event is ready, whichever comes first.
    ///
    /// ⛔ Returns `None` when there is NOTHING TO WAIT FOR — the deadline is not strictly in
    /// the future, and no event is pending. A NULL ADVANCE MUST NEVER BE REPORTED AS A
    /// SUCCESSFUL ONE. That is the trap §3.2.1 was found by walking into: the first draft
    /// took the minimum of ALL registered deadlines, those of finished activities included,
    /// so the minimum fell in the PAST, the clock did not move, and the function declared
    /// success anyway. The executor spun forever.
    ///
    /// ⚠️ THAT RULE IS LEVEL 2, AND THIS COMMENT CLAIMS NO MORE THAN THAT. No compiler stops
    /// an implementation from returning `Some(Wakeup::DeadlineReached(self.now()))` without
    /// having moved: the signature admits it, and only a run can tell the two apart. What
    /// holds it is the conformance suite that Tasks 6 and 7 run against both implementations
    /// — §7.4.6 calls the `reactor` one "the most important: the validity of the DST rests
    /// there" — and, one level up, the executor's own turn limit, so that a block shows up
    /// as an error and never as an endless wait.
    ///
    /// ⚠️ Readiness is the OTHER HALF of this port's contract, and it has no producer yet:
    /// nothing in this milestone generates external events, so today every wait that returns
    /// `Some` ran to its deadline. §0.4.3 declares WHERE scheduling and file watching will
    /// enter — here, and not on `filesystem`, because what has to be deterministic is WHEN
    /// the notification arrives and not which path it names — and leaves the HOW to the
    /// milestone that builds them.
    fn wait_until(&mut self, deadline: Monotonic) -> Option<Wakeup>;
}
