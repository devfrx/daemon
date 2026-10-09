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
//! simulation the clock jumps to the deadline it is asked for --
//! `simulator::reactor::VirtualReactor` holds no seed -- and the seed reaches the ORDER among the
//! ready activities through the executor's `Rng`: that is how time becomes virtual (C3).
//! External readiness has no producer yet, see `wait_until`. ⚠️ RECALL OF 2026-10-03 -- audit of
//! 2026-09-30, AUD-402. Everything the campaign exercises on this side of the boundary is
//! therefore the production code, not a rehearsal of it.
//!
//! ⚠️ THE TWO CLOCKS LIVE ON THIS PORT, AND THAT DOES NOT CREATE A FAMILY OF ITS OWN (decision
//! D2 of the milestone 2 plan). Reading a clock IS I/O, and `reactor` is the port of time
//! and readiness — §3.1 already assigns it "moves the virtual clock forward". A separate
//! `clock` family would split one source of virtual time across two ports the simulator
//! would then have to keep in step, which is more machinery for less determinism.
//! ⚠️ RECALL OF 2026-10-09 -- audit of 2026-09-30, AUD-1224, AUD-1229.
//!
//! ⛔ Which of the two a caller may read is NOT this port's rule to make, and this comment
//! does not restate it as though it were. What `crate::time` holds at level 1, with the four
//! cases in `tests/compile_fail/` that guard it in both directions, is that the two times CANNOT
//! BE SWAPPED and that no `From`/`Into` road joins them — the two rows `V29 · §2.1` of §7.4.1.
//! Those bite at EVERY site where a `Monotonic` is expected, `wait_until`'s deadline included:
//! handing out both clocks from one trait adds a call site to a mechanism that is already
//! general, not a hole in it. ⚠️ AND THAT IS NARROWER THAN §2.1's "no kernel decision
//! depends on wall time": a site that holds a `Reactor` can read `wall_time()` and branch on it
//! — `WallTime` derives `Ord` and `as_millis_since_epoch` is `pub` — and no check looks for it.
//! ⚠️ RECALL OF 2026-10-09 -- audit of 2026-09-30, AUD-759.

use crate::time::{Monotonic, WallTime};

/// The port of time and readiness. `platform` implements it against the OS; `simulator`
/// implements the same trait against a virtual clock that moves only when nobody can work, and
/// holds no seed -- the seed governs the executor's order among the ready, not this clock (§2.4,
/// §3.1). ⚠️ RECALL OF 2026-10-03 -- audit of 2026-09-30, AUD-402. The conformance suite of
/// §7.4.6 runs against BOTH, and is the reason the trait says what it says instead of
/// describing one of them.
pub trait Reactor {
    /// The current monotonic instant. This is what DECISIONS read.
    fn now(&self) -> Monotonic;

    /// The current time in the world.
    ///
    /// ⚠️ THE READER §2.1 NAMES IS THE RECORD — Q14, journal stamps — AND TODAY IT DOES NOT READ
    /// IT. This line said "ONLY the record reads it" in the PRESENT tense until 2026-08-27,
    /// finding AUD-052: none of `RecordV1`'s fields is a stamp, `WallTime` does not appear in
    /// `record.rs` at all, and under `crates/*/src/` nothing reads the wall time — the call to
    /// `wall_time()` there is `SharedClock`, in `crates/daemon/src/main.rs`, forwarding it to the
    /// `SystemReactor` it wraps (⚠️ RECALL OF 2026-10-09 -- audit of 2026-09-30, AUD-1226,
    /// AUD-1227, AUD-1230). The sentence described §2.1's DESIGN and read as a guarantee about this
    /// file — the same distinction `StepId` and `CheckpointId` already carry, below the very
    /// paragraph of this module that promises not to restate a rule as though it were ours.
    ///
    /// ⛔ AND ADDING THE FIELD IS NOT A TOUCH-UP: the frozen bytes exist, so the stamp takes a NEW
    /// INDEX under rule 3 of ADR-0036, never a reused one.
    fn wall_time(&self) -> WallTime;

    /// Wait until `deadline`, or until an external event is ready, whichever comes first,
    /// and return the instant the wait resumed at.
    ///
    /// ⛔ Returns `None` when there is NOTHING TO WAIT FOR — the deadline is not strictly in
    /// the future, and no event is pending. A NULL ADVANCE MUST NEVER BE REPORTED AS A
    /// SUCCESSFUL ONE. That is the trap §3.2.1 was found by walking into: the first draft
    /// took the minimum of ALL registered deadlines, those of finished activities included,
    /// so the minimum fell in the PAST, the clock did not move, and the function declared
    /// success anyway. The executor spun forever.
    ///
    /// ⚠️ THAT RULE IS LEVEL 2, AND THIS COMMENT CLAIMS NO MORE THAN THAT. No compiler stops
    /// an implementation from returning `Some(self.now())` without having moved: the
    /// signature admits it, and only a run can tell the two apart. What holds it is the
    /// conformance suite that Tasks 6 and 7 run against both implementations — §7.4.6 calls
    /// the `reactor` one "the most important: the validity of the DST rests there" — and,
    /// one level up, the executor's own turn limit, so that a block shows up as an error and
    /// never as an endless wait. ⚠️ THAT NET HOLDS WHERE THE LIMIT IS FINITE -- the benches and
    /// the campaigns -- AND NOT IN THE SHIPPED BINARY, which is handed `u64::MAX` (decision A of
    /// §5 of the sub-project 2 design): there the guard is the OS watchdog of sub-project 10, and
    /// `RunError::TurnLimitReached` says the rest. ⚠️ RECALL OF 2026-10-03 -- audit of
    /// 2026-09-30, AUD-058.
    ///
    /// ⚠️ Readiness is the OTHER HALF of this port's contract, and it has no producer yet:
    /// nothing in this milestone generates external events, so today every wait that returns
    /// `Some` ran to its deadline. §0.4.3 declares WHERE scheduling and file watching will
    /// enter — here, and not on `filesystem`, because what has to be deterministic is WHEN
    /// the notification arrives and not which path it names — and leaves the HOW to the
    /// milestone that builds them.
    ///
    /// ⛔ WHAT THIS RETURN TYPE DELIBERATELY IS NOT, recorded because the reasoning outlives
    /// the types it rejected. The milestone 2 plan wrapped the answer in an enum, `Wakeup`,
    /// with two variants — `DeadlineReached(Monotonic)` and `EventReady(Monotonic)` — plus a
    /// getter. Both the variant and the wrapper are gone, and the reasons stack:
    ///
    /// - `EventReady` HAD NO PRODUCER. Nothing in this milestone generates external events,
    ///   so no implementation could return it, and its only consumer would have been the
    ///   getter's own match arm — an item that exists to support itself. It is the rule that
    ///   already removed `Millis::ZERO`, `Monotonic::as_millis` and a `?Sized` bound here.
    /// - THE ARGUMENT FOR DECLARING IT EARLY DOES NOT REACH IT. §0.4.3 states what its staging
    ///   rule B (§0.3) buys, in its own words: "here one declares WHERE a source enters, not HOW
    ///   it works … what this section buys is that the day it is built, no new port is born".
    ///   The port had to exist now, and it does. The shape of what the wait returns is the
    ///   "how", which that section excludes outright — adding that the conformance suite
    ///   "does not cover an operation nobody calls".
    /// - ⛔ AND THE VARIANT COULD NOT HAVE BEEN ACTED ON. The executor holds two task states,
    ///   `Runnable` and `Sleeping(deadline)`, and on resuming it promotes the activities
    ///   whose deadline the instant has passed. A variant carrying only an INSTANT
    ///   identifies nothing, so it could not promote the activity the event was for: the day
    ///   events exist it will have to carry WHICH REGISTRATION became ready. Declaring it
    ///   now would freeze a shape already known to be the wrong one — ADR-0009's rule, that
    ///   a minimal contract can be widened and a rich wrong one cannot.
    /// - ⛔ AND THE WRAPPER WENT WITH IT: with one variant left it distinguished NOTHING that
    ///   the signature does not already say, and keeping it would have been the very act
    ///   refused one level down — pre-declaring a shape nobody knows yet.
    ///
    /// ⚠️ `Millis`, `Monotonic` and `WallTime` ARE NOT A COUNTER-EXAMPLE to that last line,
    /// and the difference is the whole of it: those three distinguish DIFFERENT THINGS THAT
    /// SHARE A REPRESENTATION, which is exactly why passing one where another is expected
    /// has to be a compile error, and why `tests/compile_fail/` spends four cases on it. A
    /// one-variant enum over a `Monotonic` distinguishes nothing from a `Monotonic`: it buys
    /// no error anywhere, only ceremony.
    ///
    /// ⚠️ And widening later is cheap BY THE PROJECT'S OWN CRITERION, not by hope: §7.4.5
    /// stages a piece by asking "is it retrofittable?", answering for the confinement token
    /// that "adding an argument to a signature with zero callers is mechanical — rule B
    /// does not apply, so C does", the staging rules of §0.3 (⚠️ RECALL OF 2026-10-09 --
    /// audit of 2026-09-30, AUD-758). This is that case, and what makes it so is a RELATION rather
    /// than a count: EVERY call site and EVERY implementation of `wait_until` lives inside this
    /// REPOSITORY -- in the five members of the root `Cargo.toml`, and in `gui/fake-core`, which
    /// that manifest excludes and `scripts/gate-gui.sh` builds and tests on a lockfile of its own
    /// -- so there is no external consumer, and widening the return is a COMPILE ERROR at each
    /// one and a silent change at none. ⚠️ RECALL OF 2026-10-03 -- audit of 2026-09-30, AUD-063.
    /// And NO DURABLE ARTEFACT carries its shape, which is the half the contrast is
    /// about: ADR-0036 rule 3, where a new field must be optional and take a new index
    /// precisely because bytes already written cannot be recompiled.
    ///
    /// ⛔ RECALL OF 2026-08-28, FINDING AUD-016 -- THIS SAID "three call sites inside the
    /// repository", AND THE FIGURE IS REMOVED RATHER THAN REALIGNED TO TWELVE. It was true when
    /// this file was written on 2026-08-09 and the executor, the conformance suite and two
    /// simulator suites have added call sites since, without this line being reread. A count of
    /// callers is the specimen case of a figure that rots at the next commit, so what replaces
    /// it is the relation that holds however many arrive -- the cure gotcha #68 asks for, and
    /// the one AUD-009 applied to the gate's `cargo` sites. On 2026-08-28, over `crates/` alone,
    /// `grep -rn '\.wait_until(' --include=*.rs crates/ | wc -l` and the same for
    /// `'impl Reactor for'` answered 12 and 11 — BEFORE this paragraph quoted them: rerun, each
    /// also counts the lines here that quote it (⚠️ RECALL OF 2026-10-09 -- audit of
    /// 2026-09-30, AUD-1223). The census that sees EVERY site walks `gui/` too,
    /// drops the lines that only quote it, and anchors the impl, generic and path-qualified ones
    /// included:
    /// `grep -rn '\.wait_until(' --include=*.rs crates/ gui/ | grep -vE '^[^:]+:[0-9]+:[[:space:]]*//'`
    /// and `grep -rnE '^ *impl(<[^>]*>)? *([A-Za-z_]+::)*Reactor for' --include=*.rs crates/ gui/`.
    /// ⚠️ RECALL OF 2026-10-03 -- audit of 2026-09-30, AUD-063.
    ///
    /// ⚖️ THE OTHER TWO CLAUSES WERE MEASURED, NOT ASSUMED, and both hold: the root manifest
    /// lists five members and EXCLUDES `gui` -- where the one site outside them lives,
    /// `gui/fake-core` -- and `spikes`, which holds none; and `Record` carries NO time field at
    /// all, so nothing durable is shaped by this return. ⛔ AND "MECHANICAL" IS NOW MEASURED TOO,
    /// which is what "not by hope" above demands: widening the return to `Option<(Monotonic,
    /// u32)>` and running `cargo check --locked --workspace --all-targets --keep-going` gives
    /// `error[E0308]` at `crates/kernel/src/executor.rs` and ZERO `unused`/`unreachable`
    /// warnings -- nothing degrades quietly. ⚠️ THE OTHER SITES ARE HIDDEN BEHIND THAT FIRST
    /// WALL and the run cannot count them, because dependents are not checked until the lib
    /// compiles: the breakage is LOUD, and its full width is not measurable in one pass.
    /// Mutation applied, checked, and revoked with `git diff` at zero lines. ⚠️ AND THAT RUN DOES
    /// NOT REACH `gui/fake-core`, which no `--workspace` command walks: its site breaks in
    /// `scripts/gate-gui.sh`, at `cargo test --locked --manifest-path gui/fake-core/Cargo.toml` --
    /// loud as well, in a gate of its own. ⚠️ RECALL OF 2026-10-03 -- audit of 2026-09-30, AUD-063.
    fn wait_until(&mut self, deadline: Monotonic) -> Option<Monotonic>;
}
