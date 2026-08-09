//! The production wiring: it mounts `platform`, builds the executor and runs it.
//!
//! # It PRODUCES the parameters, the kernel only RECEIVES them
//!
//! The kernel reads no configuration — §2.8, ADR-0034 — so somebody has to resolve the
//! values and DELIVER them at construction. In production that somebody is this binary.
//!
//! In this sub-project the resolved values are LITERALS RIGHT HERE: constraint 11 of §11.
//! That is the correct boundary and not a shortcut — the parameter store arrives later with
//! an interface of its own, and until it does the value has to be chosen somewhere. What
//! makes it acceptable is that it is WRITTEN DOWN rather than hidden: a literal in `daemon`
//! is visible and can vary from one call site to the next, whereas the same number written
//! inside the kernel would appear in no list and could not be made to vary at all
//! (gotcha #28).
//!
//! # ⛔ It does NOT mount `simulator`
//!
//! `Cargo.toml` does not depend on it, with the reason written where the line is missing.
//! The daemon is the PRODUCTION wiring: it mounts `platform`. In simulation the wiring is
//! the test bench's job, and the bench receives the resolved parameters exactly as this file
//! produces them.
//!
//! # What a run with NO activities proves
//!
//! Nothing is spawned, and that is not a placeholder. It is the ONE claim this binary can
//! make today: THE WHOLE GRAPH ASSEMBLES — the real `Rng`, the real `Reactor`, the delivered
//! `Parameters` and the executor's `Sleep` cell fit together, and the executor runs to
//! completion. There is no work to do yet, so there is nothing else to claim.

use kernel::executor::{Executor, RunError, Sleep};
use kernel::parameters::Parameters;
use platform::reactor::SystemReactor;
use platform::rng::SequentialRng;

/// How many turns the executor may take before it declares a block (§3.2.1).
///
/// # ⛔ It is a COUNT OF TURNS, not a ceiling on wall-clock time
///
/// The distinction is not pedantry, and an earlier version of this comment got it wrong by
/// asserting that a turn "performs no I/O". IT CAN. A turn is one iteration of
/// `Executor::run`, and that iteration may contain `reactor.wait_until` — which on
/// `SystemReactor`, the reactor THIS FILE wires, is a real `std::thread::sleep`. So the wall
/// time of a turn is whatever that turn waits for, and no number chosen here bounds it.
///
/// Measured on this graph, which is what settles it:
///
/// | Case                                          | Cost                   |
/// |-----------------------------------------------|------------------------|
/// | the whole ceiling spent polling, no waits     | 100 000 turns ≈ 15 ms  |
/// | ONE run whose turns contain a 2000 ms wait    | 2.0004 s               |
///
/// # What the value therefore buys, stated exactly
///
/// - ABOVE anything legitimate. The reference scenario — three activities of four steps
///   each — takes NINE turns, so the limit clears it by FOUR orders of magnitude.
/// - It catches a block that DOES NOT WAIT in far less than a second: the top row is the
///   whole ceiling in about fifteen milliseconds. Those are the two failures
///   `RunError::TurnLimitReached` documents — an activity that yields for ever, and one that
///   re-registers an elapsed deadline. Both spin, so both land there.
/// - ⚠️ AND IT DOES NOT BOUND THE CLOCK for an activity that keeps going back to sleep on
///   deadlines still in the FUTURE. That run is not spinning, it is waiting; it still ends,
///   because the turns still run out, but at whatever wall time its waits add up to. The
///   guarantee is TERMINATION, not promptness.
///
/// # Where the nine comes from
///
/// 📌 MEASURED, not carried over. The plan said "fewer than forty", and that figure was
/// never checked — an expectation written before the measurement is a hypothesis, which is
/// gotcha #15, named at the top of `crates/kernel/tests/executor_determinism.rs` for this
/// exact reason. The instrument is the limit itself: `run` fails as soon as `turns > limit`,
/// so the SMALLEST limit that still returns `Ok(())` IS the count. It is nine, and the same
/// nine on all 200 seeds of that file — the seed changes the ORDER within a turn, not the
/// NUMBER of turns. Eight fails, which is what makes nine a boundary rather than a guess.
const EXECUTOR_TURN_LIMIT: u64 = 100_000;

/// Builds the production graph and runs the executor, handing back what the run said.
///
/// ⚠️ IT IS A FUNCTION RATHER THAN THE BODY OF `main` SO THAT A TEST CAN CALL IT. The
/// quality gate runs `cargo build` and `cargo test`, never `cargo run`, so a wiring that
/// only `main` touches would be the one part of this milestone that no check exercises —
/// and a principle nobody can check is an intention. `main` keeps the process-level job,
/// what to print and what to exit with, and nothing else.
fn run_the_production_graph() -> Result<(), RunError> {
    // ⚠️ THE CELL IS DECLARED FIRST, and the order is load-bearing: `Executor` borrows it for
    // `'a`, and locals drop in reverse order of declaration, so the executor goes before the
    // cell it points at. Swapping these two lines does not compile.
    let sleep = Sleep::new();

    let mut executor = Executor::new(
        SequentialRng::new(),
        SystemReactor::new(),
        Parameters::new(EXECUTOR_TURN_LIMIT),
        &sleep,
    );

    executor.run()
}

/// ⛔ DECLARED RESIDUAL — THE ERROR BRANCH BELOW IS COVERED BY NOTHING, and saying so is the
/// point. The wiring was pulled out into a function precisely because the gate runs `build`
/// and `test` and never `run`; this is the half that stayed behind. No check observes that a
/// failed run writes to stderr, leaves stdout empty, and exits 1. It was verified BY HAND and
/// does all three — but a verification by hand is a moment in time, not a control.
///
/// ⚠️ AND IT IS NOT WORTH THE PRICE, which has to be said rather than implied. Covering it
/// means spawning the built binary as a CHILD PROCESS and reading back its two streams and
/// its exit status, in order to hold three lines that make no decision. `platform`'s
/// `wait_until` declares a residual for the same shape of reason: a control that is absent
/// and DECLARED beats one that is contorted. The trade stops being fair the day this branch
/// grows a decision of its own.
fn main() {
    match run_the_production_graph() {
        Ok(()) => println!("daemon: the graph is wired, and the executor ran with no activities."),
        Err(error) => {
            // ⛔ stderr and exit 1: a run that stopped without finishing must be
            // distinguishable by a caller that reads neither stream.
            eprintln!("daemon: the executor stopped without finishing: {error:?}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    //! ⚠️ A UNIT TEST MODULE IN `src/`, where this repository otherwise puts tests in
    //! `tests/` — and here the deviation is NOT a preference, it is FORCED. The function
    //! under test is private in a `bin` target, and an integration test is a crate of its own
    //! that can link only a LIBRARY. No file under `tests/` can reach
    //! `run_the_production_graph`, so moving this module out would not relocate the test, it
    //! would delete it.
    //!
    //! ⚠️ It is one of TWO such modules in the workspace. The other is in
    //! `crates/platform/src/rng.rs`, where the reason is different and IS a choice.

    use super::*;

    /// What this buys, stated exactly: THE GRAPH ASSEMBLES AND RUNS. Not that it DOES
    /// anything — no activity is spawned, and there is nothing to do yet — but that the real
    /// `SequentialRng`, the real `SystemReactor`, the delivered `Parameters` and the `Sleep`
    /// cell fit together and the executor comes back saying the run finished.
    ///
    /// ⚠️ IT CALLS THE SAME FUNCTION `main` CALLS, which is why that function exists. A test
    /// that rebuilt the wiring itself would be a second copy, and on the day the two drifted
    /// apart this one would go on passing about a graph nobody ships.
    ///
    /// ⛔ DECLARED RESIDUAL — IT DOES NOT COVER THE VALUE OF `EXECUTOR_TURN_LIMIT`, and the
    /// two directions were measured rather than assumed:
    ///
    /// - setting the constant to `0` leaves this test GREEN. `Executor::run` is
    ///   `while !self.tasks.is_empty()`, so with nothing spawned the body never runs and the
    ///   counter is never compared with the limit. Any value whatsoever passes here;
    /// - spawning a never-ready activity turns it RED with `Err(TurnLimitReached)`, which is
    ///   what says the assertion is not unconditionally true and that the delivered limit
    ///   really does reach the executor.
    ///
    /// So what this test holds is the WIRING — that the graph assembles and the run
    /// terminates — and not the sizing of the number. The number gets its own check when
    /// something is spawned to exercise it.
    #[test]
    fn the_production_graph_assembles_and_the_executor_runs_to_completion() {
        assert_eq!(run_the_production_graph(), Ok(()));
    }
}
