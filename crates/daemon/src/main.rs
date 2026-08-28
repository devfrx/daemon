//! The production wiring: it mounts `platform`, opens the journal, builds the arbiter with the
//! two permanent grants of ADR-0033, builds the executor and runs it.
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
//! Nothing is spawned, and that is not a placeholder: there is no work to do yet. What the
//! run claims is THE WHOLE GRAPH ASSEMBLES — the real `Rng`, the real `Reactor`, the real
//! `Journal` on the disk, the arbiter holding the two permanent grants of ADR-0033, the
//! delivered `Parameters` and the executor's `Sleep` cell fit together, and the executor runs
//! to completion.
//!
//! ⚠️ RECALL OF 2026-08-21, MILESTONE 5 TASK 10. This heading said "it is the ONE claim this
//! binary can make today", and this task is what made that false: the start-up gained failures
//! it can say out loud and did not have — the journal will not open, and either permanent quota
//! of ADR-0033 does not get in — and each of them is a claim of its own. The sentence is
//! REWRITTEN and not answered beside itself, which is finding A-2 of this project's audit.

use std::path::Path;

use kernel::arbiter::{
    Admission, Arbiter, ComputeClass, Grant, Mib, Preemption, RemotePolicy, ResourceProfile,
    VramPolicy,
};
use kernel::executor::{Executor, RunError, Sleep};
use kernel::parameters::Parameters;
use kernel::time::{Millis, Monotonic};
use platform::journal::{FileJournal, OpenError};
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

/// How much VRAM the machine has, in whole MiB (§5.1).
///
/// # ⛔ It is DECLARED here, because the kernel has no way to ask
///
/// Querying the GPU is an OS call, which I3 forbids the kernel, and none of the six port
/// families supplies hardware capacity. So the total is a DELIVERED parameter like every
/// other, and this binary is the somebody who resolves it — constraint 11 of §11, the same
/// boundary `EXECUTOR_TURN_LIMIT` sits on.
///
/// # Where the number comes from
///
/// 16384 MiB is the single RTX 5080 this project is built around — the resource constraint
/// that ADR-0005 calls dominant. ⚠️ IT IS NOT MEASURED FROM THE DEVICE and cannot be from
/// here: a wrong total produces OVER-ADMISSION, which §5.1 declares as the cost of
/// delivering it rather than asking for it. That makes a systematic discrepancy a defect of
/// this line, visible and variable, instead of an incident nobody can locate.
///
/// # What reads it, since this task
///
/// ⚠️ RECALL OF 2026-08-21, MILESTONE 5 TASK 10 — THIS PARAGRAPH SAID THE OPPOSITE, and the
/// sentence it replaces was a DEADLINE IN PROSE that this task is what makes come due
/// (gotcha #77). It read "NOTHING IN THIS BINARY READS IT YET beyond handing it over. No
/// arbiter is wired here: the production wiring of the arbiter, with the two permanent grants
/// of §4.3, is a later task of milestone 5". That task is THIS one. The whole paragraph is
/// rewritten rather than contradicted underneath itself — finding A-2 — because the half that
/// stayed true ("the value is CHOSEN IN `daemon` and travels through `Parameters`") reads as
/// an excuse when it is left standing beside a denial.
///
/// It travels through `Parameters` and reaches TWO consumers now: `Executor::new`, which
/// carries it without reading it, and `Arbiter::new`, which uses it as the ceiling every
/// admission is measured against. There is no second road for it to arrive by.
const TOTAL_VRAM: Mib = Mib::new(16_384);

/// Where the journal file lives, in production.
///
/// ⛔ A LITERAL, ON THE SAME BOUNDARY AS `TOTAL_VRAM` and for the same reason: the value has
/// to be chosen somewhere until the parameter store arrives, and a literal in `daemon` is
/// visible and can be varied. ⚠️ AND IT IS RELATIVE TO THE WORKING DIRECTORY, which is
/// declared rather than defended: where a per-user data directory should be is a decision no
/// ADR has taken, and inventing one here would be that decision taken by whoever typed the
/// path. Every test passes its OWN path, so nothing in the gate depends on this constant.
const JOURNAL_PATH: &str = "journal.redb";

/// The audio quota, and the presentation quota of ADR-0033.
///
/// ⛔ THEY ARE NOT SUBTRACTIONS, THEY ARE TWO PERMANENT GRANTS, and the difference is I2. A
/// quota subtracted from the budget WITHOUT A HOLDER leaves I2 false for that consumer --
/// "the subtraction is not an exemption" (ADR-0005, gotcha #4) -- whereas a grant HAS a
/// holder by construction. ADR-0033 says it in those words: "the core REQUESTS a permanent,
/// non-preemptible presentation grant at start-up".
const AUDIO_QUOTA: Mib = Mib::new(1_024);
const PRESENTATION_QUOTA: Mib = Mib::new(768);

/// ⛔ "PERMANENT" IS NOT A TYPE -- it is "nobody calls release". The window is saturated on
/// purpose: `Monotonic::saturating_add` does not wrap, so there is no special case inside the
/// arbiter for a grant that never expires, and none is wanted.
///
/// ⚠️ AND IT IS NOT LITERALLY "NEVER", WHICH WAS MEASURED RATHER THAN REASONED. This comment
/// was dictated saying "a deadline this far out NEVER ARRIVES", and it does arrive, at exactly
/// one instant: `Monotonic::ORIGIN.saturating_add(FOR_EVER)` saturates AT `u64::MAX`, and
/// `Arbiter::collect_expired` compares `expires_at <= now`, so a sweep at the last
/// representable millisecond of the axis collects both quotas. ✅ MEASURED, not deduced --
/// `allocated()` comes back `Mib(0)` instead of `Mib(1792)` there. 📌 What the saturation
/// buys is unchanged: about 584 million years of monotonic time, on a clock that starts at
/// process start-up.
const FOR_EVER: Millis = Millis::new(u64::MAX);

/// The two profiles the composition root reserves at start-up.
///
/// ⛔ THE ARBITER DOES NOT KNOW THESE ARE CALLED "audio" AND "presentation". It sees two
/// permanent grants like any other -- which is ADR-0001: no capability has privileged access.
/// Wiring the two names inside the arbiter would be two special cases in a mechanism that has
/// to be even-handed.
///
/// ⚠️ THEY ARE CONSTANTS AND NOT TWO LITERALS INSIDE THE WIRING, and the reason is that the
/// probes name them: a profile built twice is a profile that can drift, and the probe would
/// then be checking its own copy.
const AUDIO_RESERVATION: ResourceProfile = ResourceProfile {
    name: "audio-reserved",
    reserved_vram: AUDIO_QUOTA,
    compute_class: ComputeClass::Realtime,
    preemption: Preemption::Never,
};

const PRESENTATION_RESERVATION: ResourceProfile = ResourceProfile {
    name: "presentation-reserved",
    reserved_vram: PRESENTATION_QUOTA,
    compute_class: ComputeClass::Realtime,
    preemption: Preemption::Never,
};

/// Why the start-up did not complete.
///
/// ⛔ THREE VARIANTS, AND THE THIRD IS THE ONE THAT CLOSES `E41`. An impossible VRAM
/// configuration stopped announcing itself the day the arbiter grew queues: the second
/// permanent quota comes back `Queued` instead of `Refused`, and nobody will ever serve it,
/// because releasing a permanent grant is exactly what nobody does. The arbiter cannot repair
/// that -- "permanence is not a type, it is nobody calls release", so it cannot tell a ticket
/// that WILL be served from one that never will -- and a ticket that waits for ever is the
/// silent degradation ADR-0005 and ADR-0019 forbid. Here it is not silent: the start-up stops
/// and NAMES the quota.
///
/// ⚠️ NO `PartialEq`, AND IT IS FORCED RATHER THAN CHOSEN: `OpenError` derives `Debug` alone,
/// so an `assert_eq!` on this type does not compile and the probes match instead. `Debug` is
/// what the probes and `main` both need, and it is the only thing `OpenError` gives.
#[derive(Debug)]
enum StartupError {
    /// The journal file would not open. Two things a human has to tell apart live inside
    /// `OpenError`: a wrong path, and a file another journal already holds.
    Journal(OpenError),
    /// A permanent quota of ADR-0033 did not get in. The name is the profile's own.
    ReservedQuota { name: &'static str },
    /// The run stopped without finishing.
    Run(RunError),
}

/// Builds the production graph and runs the executor, handing back what the start-up said.
///
/// ⚠️ IT IS A FUNCTION RATHER THAN THE BODY OF `main` SO THAT A TEST CAN CALL IT. The
/// quality gate runs `cargo build` and `cargo test`, never `cargo run`, so a wiring that
/// only `main` touches would be the one part of this milestone that no check exercises —
/// and a principle nobody can check is an intention. `main` keeps the process-level job,
/// what to print and what to exit with, and nothing else.
///
/// ⛔ THE PATH IS AN ARGUMENT, AND THAT IS NOT CAUTION. Handed a `FileJournal`, the test that
/// already existed starts writing a REAL FILE; a fixed path in a shared directory is gotcha
/// #52, and on Windows the clean-up of an open file fails silently, so the red would come out
/// on Linux — the project's second system.
fn run_the_production_graph(journal_path: &Path) -> Result<(), StartupError> {
    run_the_graph(
        Parameters::new(EXECUTOR_TURN_LIMIT, TOTAL_VRAM),
        journal_path,
    )
}

/// The graph itself, on parameters it is HANDED rather than reads.
///
/// ⛔ IT EXISTS SO THE TWO PERMANENT QUOTAS CAN BE PROVEN TO STOP THE START-UP, and that is
/// worth the extra function: with the total taken from `TOTAL_VRAM` inside the body, a probe
/// could not build a machine too small to hold them, and the error branch that closes `E41`
/// would be reachable by no check at all. It is also the shape ADR-0034 already imposes
/// everywhere else — the value is DELIVERED at construction, and here it is delivered one
/// level further down.
fn run_the_graph(parameters: Parameters, journal_path: &Path) -> Result<(), StartupError> {
    // ⚠️ THE JOURNAL HAS NO CONSUMER IN THIS BINARY YET, and that is what this task delivers
    // rather than a placeholder: it is OPENED, so the file exists, the exclusive lock is
    // taken and a bad path stops the start-up here instead of at the first write. The day
    // something journals, it journals into this one.
    let _journal = FileJournal::open(journal_path).map_err(StartupError::Journal)?;

    let _arbiter = build_the_arbiter(parameters)?;

    // ⚠️ THE CELL IS DECLARED FIRST, and the order is load-bearing: `Executor` borrows it for
    // `'a`, and locals drop in reverse order of declaration, so the executor goes before the
    // cell it points at. Swapping these two lines does not compile.
    let sleep = Sleep::new();

    let mut executor = Executor::new(
        SequentialRng::new(),
        SystemReactor::new(),
        parameters,
        &sleep,
    );

    executor.run().map_err(StartupError::Run)
}

/// Builds the arbiter and takes the two permanent quotas of ADR-0033 out of its budget.
///
/// ⛔ IT HANDS THE ARBITER BACK INSTEAD OF KEEPING IT, and that is what makes the BOOKS
/// checkable: `run_the_graph` answers `Result<(), StartupError>` and nothing else, so a probe
/// that wants to ask `allocated()` or `policy()` has to be given the object. A probe that
/// assembled its own would be a second copy of the wiring, green on the day the two drift.
///
/// ⛔ THE ORDER IS AUDIO FIRST, AND IT IS NOT ARBITRARY: both profiles sit in
/// `ComputeClass::Realtime`, so nothing inside the arbiter breaks the tie between two requests
/// of one lane except arrival. On a machine too small for both, whichever is asked for SECOND
/// is the one that does not get in -- which is what the two probes name.
///
/// ⚠️ THE TWO GRANTS ARE DROPPED HERE AND THE RESERVATIONS ARE NOT, and that is the point
/// rather than an oversight: "permanent" is not a type, it is "nobody calls release". The
/// arbiter keeps both in its books until somebody hands a grant back, and nobody ever will.
fn build_the_arbiter(parameters: Parameters) -> Result<Arbiter, StartupError> {
    let mut arbiter = Arbiter::new(
        parameters,
        // ⛔ REMOTE is the default of ADR-0006, and reopening that turns a coordinated swap
        // from an exception into the normal case.
        VramPolicy::Remote(RemotePolicy),
    );

    let _audio = reserve(&mut arbiter, &AUDIO_RESERVATION)?;
    let _presentation = reserve(&mut arbiter, &PRESENTATION_RESERVATION)?;

    Ok(arbiter)
}

/// Turns an `Admission` into a start-up decision.
///
/// ⛔ THE TWO FAILING ANSWERS ARE THE SAME FAILURE HERE, AND THEY ARE NOT THE SAME EVERYWHERE.
/// `Refused` means "bigger than the whole machine" and `Queued` means "bigger than what is
/// free right now"; for an ordinary request those call for different behaviour, which is why
/// `Admission` has no `is_granted()`. For a PERMANENT quota they collapse: nobody releases a
/// permanent grant, so a queued one waits for ever, and waiting for ever at start-up is a
/// misconfiguration exactly like asking for more than the machine has.
///
/// ⚠️ AND THE COLLAPSE IS MADE HERE AND NOT INSIDE THE ARBITER, which is `E41` in one line:
/// the arbiter cannot tell a ticket that will be served from one that never will, and a rule
/// it cannot evaluate is not a rule it can enforce. The composition root can, because it is
/// the one that knows nobody is ever going to release these two.
///
/// ⛔ DECLARED LIVE MUTANT -- `Monotonic::ORIGIN` IS HELD BY NOTHING, said here rather than
/// left to be found beside the residuals that ARE declared. `FOR_EVER` saturates, so
/// `now.saturating_add(FOR_EVER)` is `u64::MAX` whatever `now` is, and every starting instant
/// gives the same `expires_at`. ✅ MEASURED on 2026-08-21, not deduced: with this argument
/// changed to `Monotonic::from_millis(1)` NOTHING in the whole workspace goes red. The count
/// is row 15 of the mutation campaign in `docs/porta-di-qualita.md`.
///
/// ⚖️ AND IT IS DECLARED AND NOT PINNED, because there is no claim behind it to defend: the
/// indifference is an ARITHMETIC CONSEQUENCE of the saturation and not a decision somebody
/// took. The day the window stops saturating, the starting instant starts mattering.
fn reserve(arbiter: &mut Arbiter, profile: &ResourceProfile) -> Result<Grant, StartupError> {
    match arbiter.admit(profile, FOR_EVER, Monotonic::ORIGIN) {
        Admission::Granted(grant) => Ok(grant),
        Admission::Queued(_) | Admission::Refused { .. } => {
            Err(StartupError::ReservedQuota { name: profile.name })
        }
    }
}

/// ⛔ DECLARED RESIDUAL — THE ERROR BRANCHES BELOW ARE COVERED BY NOTHING, and saying so is
/// the point. The wiring was pulled out into a function precisely because the gate runs
/// `build` and `test` and never `run`; this is the half that stayed behind. No check observes
/// that a failed start-up writes to stderr, leaves stdout empty, and exits 1.
///
/// ⚠️ AND WHICH BRANCHES WERE WALKED BY HAND IS NAMED, because "verified by hand" over four
/// arms is a claim about three of them nobody made. Walked on 2026-08-21, in a scratch
/// directory outside the repository: the `Ok` arm — exit 0, the sentence on stdout, stderr
/// EMPTY, and a `journal.redb` of 1 056 768 bytes left behind — and the `Journal` arm, provoked
/// by putting a DIRECTORY where the file should be: exit 1, stdout EMPTY, and
/// `File(Os { code: 5, kind: PermissionDenied, … })` on stderr. ⛔ `ReservedQuota` AND `Run`
/// WERE NOT WALKED: neither can be provoked from outside without editing this file, which is
/// what the mutation campaign does and a hand-run cannot. And a verification by hand is a
/// moment in time in any case, not a control.
///
/// ⚠️ AND IT IS NOT WORTH THE PRICE, which has to be said rather than implied. Covering it
/// means spawning the built binary as a CHILD PROCESS and reading back its two streams and
/// its exit status, in order to hold lines that make no decision. `platform`'s `wait_until`
/// declares a residual for the same shape of reason: a control that is absent and DECLARED
/// beats one that is contorted. The trade stops being fair the day these branches grow a
/// decision of their own.
///
/// ⛔ THE THREE FAILURES ARE SPELT OUT ONE BY ONE AND NOT FOLDED INTO ONE `{error:?}`, and it
/// is FORCED rather than a preference: `#[derive(Debug)]` does not count as a read for the
/// dead-code analysis, so a single arm left `Journal`'s and `Run`'s payloads flagged as
/// "field `0` is never read" — MEASURED, two warnings on `cargo test -p daemon`. Silencing
/// that with an `#[allow]` is a prohibition switched off (gotcha #13), and emptying the two
/// payloads would throw away the only thing that says WHICH file and WHICH failure. Reading
/// them is what the fix had to be, and the operator gets three different sentences out of it.
fn main() {
    match run_the_production_graph(Path::new(JOURNAL_PATH)) {
        Ok(()) => println!(
            "daemon: the graph is wired, the two reserved quotas are held, and the executor ran \
             with no activities."
        ),
        // ⛔ stderr and exit 1 on every failing branch: a start-up that did not complete must
        // be distinguishable by a caller that reads neither stream.
        Err(StartupError::Journal(error)) => {
            stop(&format!(
                "the journal at {JOURNAL_PATH} would not open: {error:?}"
            ));
        }
        Err(StartupError::ReservedQuota { name }) => {
            stop(&format!(
                "the reserved quota {name} did not get in: this machine is too small for the \
                 quotas of ADR-0033"
            ));
        }
        Err(StartupError::Run(error)) => {
            stop(&format!(
                "the executor stopped without finishing: {error:?}"
            ));
        }
    }
}

/// Says why the start-up stopped, on stderr, and leaves exit code 1 behind.
///
/// ⚠️ `!` AND NOT `()`, so that the three call sites above do not each need a statement saying
/// nothing follows. It is the return type `std::process::exit` already has.
fn stop(reason: &str) -> ! {
    eprintln!("daemon: {reason}.");
    std::process::exit(1)
}

#[cfg(test)]
mod tests {
    //! ⚠️ A UNIT TEST MODULE IN `src/`, where this repository otherwise puts tests in
    //! `tests/` — and here the deviation is NOT a preference, it is FORCED. The functions
    //! under test are private in a `bin` target, and an integration test is a crate of its own
    //! that can link only a LIBRARY. No file under `tests/` can reach
    //! `run_the_production_graph`, so moving this module out would not relocate the tests, it
    //! would delete them.
    //!
    //! ⚠️ HOW MANY SUCH MODULES THERE ARE, AND WHERE, COMES FROM THE COMMAND AND NOT FROM THIS
    //! LINE — `grep -rn --include='*.rs' 'mod tests {' crates/*/src/`. What differs between them
    //! is the REASON, and each states its own: here it is forced, elsewhere it IS a choice.
    //!
    //! ⛔ RECALL OF 2026-08-28, FINDING AUD-060 — THIS SAID "one of THREE" AND NAMED THE OTHER
    //! TWO. The figure was RIGHT and is removed anyway: the same count sat in
    //! `crates/platform/src/rng.rs` saying TWO, and correcting only the false house would keep
    //! the count in two places, which is what let them diverge (gotcha #68).

    use super::*;

    // ⚠️ IMPORTED HERE AND NOT AT THE TOP OF THE FILE, and it is MEASURED rather than tidy:
    // `MakeRoom` is the trait `VramPolicy::name` lives on, nothing outside these tests calls it,
    // and at the top it made `cargo build --locked --workspace` say `unused import` — a warning
    // this repository does not switch off with an `#[allow]` (gotcha #13).
    use kernel::arbiter::MakeRoom;

    /// ⛔ A DIRECTORY OF ITS OWN PER CALL SITE, from `line!()`, and it is not caution: a
    /// fixed path in a shared directory is gotcha #52, measured at milestone 3. Windows
    /// refuses to delete a file that is open, so the removal FAILS SILENTLY there and the
    /// red comes out on Linux -- the project's second system.
    ///
    /// ⚠️ AND THE PREFIX IS DIFFERENT from the two benches of `platform`, because a line
    /// number is unique inside ONE file only and the binaries run together.
    fn private_dir_for_line(line: u32) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("daemon-production-graph-{line}"));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("a fresh directory for this call site");
        dir
    }

    /// The arbiter of the PRODUCTION parameters, or a red that NAMES the quota that fell.
    ///
    /// ⛔ IT HOLDS SOMETHING INSTEAD OF ONLY SHORTENING TWO CALL SITES, which is the shape a
    /// bench helper has to have here -- task 8 already paid once for one that held nothing
    /// while its doc said otherwise. What it holds is that on `TOTAL_VRAM` BOTH quotas of
    /// ADR-0033 get in: ✅ MEASURED, with `TOTAL_VRAM` cut to `Mib::new(1_000)` every caller
    /// goes red through this `panic!`, and row 2 of the campaign is that measurement.
    ///
    /// ⚠️ `match` AND NOT `.expect(…)`, and it is forced rather than chosen: `Arbiter` has no
    /// `Debug`, so the `Result` cannot be formatted as a whole. Taking the error out first is
    /// what lets the failure say which quota fell.
    fn the_production_arbiter() -> Arbiter {
        match build_the_arbiter(Parameters::new(EXECUTOR_TURN_LIMIT, TOTAL_VRAM)) {
            Ok(arbiter) => arbiter,
            Err(error) => panic!("a permanent quota of ADR-0033 must be granted: {error:?}"),
        }
    }

    /// What this buys, stated exactly: THE GRAPH ASSEMBLES AND RUNS. Not that it DOES
    /// anything -- no activity is spawned, and there is nothing to do yet -- but that the real
    /// `SequentialRng`, the real `SystemReactor`, the real `FileJournal`, the arbiter with the
    /// two permanent grants of ADR-0033, the delivered `Parameters` and the `Sleep` cell fit
    /// together and the executor comes back saying the run finished.
    ///
    /// ⚠️ IT CALLS THE SAME FUNCTION `main` CALLS, which is why that function exists. A test
    /// that rebuilt the wiring itself would be a second copy, and on the day the two drifted
    /// apart this one would go on passing about a graph nobody ships.
    ///
    /// ⚠️ `assert!` AND NOT `assert_eq!`, and the reason is measured rather than stylistic:
    /// `StartupError` carries an `OpenError`, which derives `Debug` ALONE, so the enum cannot
    /// derive `PartialEq` and `assert_eq!(…, Ok(()))` does not compile. The `Debug` goes INTO
    /// THE MESSAGE, because a bare `is_ok()` would not say which of the three branches fired.
    ///
    /// ⛔ DECLARED RESIDUAL -- IT DOES NOT COVER THE VALUE OF `EXECUTOR_TURN_LIMIT`, and the
    /// two directions were measured rather than assumed:
    ///
    /// - setting the constant to `0` leaves this test GREEN. `Executor::run` is
    ///   `while !self.tasks.is_empty()`, so with nothing spawned the body never runs and the
    ///   counter is never compared with the limit. Any value whatsoever passes here;
    /// - spawning a never-ready activity turns it RED with `Err(TurnLimitReached)`, which is
    ///   what says the assertion is not unconditionally true and that the delivered limit
    ///   really does reach the executor.
    ///
    /// So what this test holds is the WIRING -- that the graph assembles and the run
    /// terminates -- and not the sizing of the number. The number gets its own check when
    /// something is spawned to exercise it.
    #[test]
    fn the_production_graph_assembles_and_the_executor_runs_to_completion() {
        let dir = private_dir_for_line(line!());

        let outcome = run_the_production_graph(&dir.join("journal.redb"));

        assert!(
            outcome.is_ok(),
            "the production graph must assemble: {outcome:?}"
        );
    }

    /// ⛔ WHAT THIS BUYS THAT THE ASSEMBLY TEST DOES NOT: that the journal is really OPENED.
    /// A wiring that had simply dropped the `FileJournal::open` line would assemble and run to
    /// completion exactly as before -- nothing in this binary reads the journal yet -- so the
    /// test above would stay green over a graph with no durable store in it at all. The file
    /// on the disk is the only thing that tells the two apart, and it is there because
    /// `FileJournal::open` COMMITS on every open, which is written down beside that function.
    #[test]
    fn the_production_graph_leaves_its_journal_on_the_disk() {
        let dir = private_dir_for_line(line!());
        let path = dir.join("journal.redb");

        let outcome = run_the_production_graph(&path);

        assert!(
            outcome.is_ok(),
            "the production graph must assemble: {outcome:?}"
        );
        assert!(
            path.is_file(),
            "the journal must be a real file, and this is what says `open` was reached"
        );
    }

    /// The other direction of the same rule (§7.1.1 rule 3): a journal that CANNOT be opened
    /// must stop the start-up instead of being carried on without.
    ///
    /// ⚠️ THE FAILURE IS PROVOKED BY A DIRECTORY THAT IS NOT THERE, which is the one way to
    /// make `open` fail that needs no privileges and behaves the same on both of the project's
    /// systems -- a locked file would need a second process on Linux, and a read-only path
    /// would need a mode change Windows spells differently.
    #[test]
    fn a_journal_that_cannot_be_opened_stops_the_start_up() {
        let dir = private_dir_for_line(line!());

        let outcome = run_the_production_graph(&dir.join("no-such-directory").join("journal.redb"));

        match outcome {
            Err(StartupError::Journal(_)) => {}
            other => panic!("a journal that will not open must stop the start-up: {other:?}"),
        }
    }

    /// ⛔ WHAT THIS BUYS THAT THE ASSEMBLY TEST DOES NOT: that the two quotas are HELD, not
    /// subtracted. An arbiter that had merely lowered its ceiling would pass the test above
    /// and leave I2 false for the two consumers -- gotcha #4, and it is the whole reason the
    /// design diverges from the letter of §5.1.
    ///
    /// ⚠️ IT CALLS THE PRODUCTION BUILDER AND DOES NOT REBUILD THE ARBITER, which is the whole
    /// reason that builder is a function of its own: `allocated()` is not reachable through
    /// `run_the_graph`, which hands back a `Result<(), StartupError>` and nothing else, so a
    /// probe of the BOOKS has to be handed the arbiter itself. An arbiter assembled here would
    /// be a second copy, and the day the two drifted apart this would go on passing about a
    /// graph nobody ships.
    ///
    /// ⛔ AND IT PINS THE POLICY, WHICH IS NOT DECORATION: `VramPolicy::Remote` is the DEFAULT
    /// of ADR-0006 and the comment beside that line says so, and an assertion with no guard is
    /// gotcha #14. ✅ MEASURED, not feared: with the wiring swapped to
    /// `VramPolicy::Local(LocalPolicy)` and this line absent, the WHOLE WORKSPACE stayed green
    /// -- 253 passed, 0 failed. The mutant was alive.
    ///
    /// ⚠️ `match` AND NOT `assert!(… .is_ok())`, and it is forced rather than chosen:
    /// `build_the_arbiter` hands back an `Arbiter`, which has no `Debug`, so the `Result`
    /// cannot be formatted. Taking the error out first is what lets a failure say which quota
    /// fell.
    #[test]
    fn the_two_reserved_quotas_are_held_by_grants_and_not_subtracted() {
        let arbiter = the_production_arbiter();

        assert_eq!(
            arbiter.allocated(),
            AUDIO_QUOTA.saturating_add(PRESENTATION_QUOTA),
            "the quotas are SPOKEN FOR, which is what a subtraction would not show"
        );
        assert_eq!(
            arbiter.policy().name(),
            "remote",
            "the composition root runs the DEFAULT policy of ADR-0006"
        );
    }

    /// ⛔ WHAT HOLDS `FOR_EVER`, and without it the constant was an ASSERTION WITH NO GUARD --
    /// gotcha #14. ✅ MEASURED: with `FOR_EVER` cut to `Millis::new(1)` and this probe absent,
    /// the WHOLE WORKSPACE stayed green, 253 passed and 0 failed. Nothing else in this binary
    /// ever advances the clock, so nothing else can see a validity window at all.
    ///
    /// ⚠️ `promote` AND NOT `allocated()`, and the choice is the arbiter's own doc rather than
    /// taste: `allocated` DELIBERATELY COLLECTS NOTHING, so asking it alone cannot tell "the
    /// sweep happened" from "the number looks right anyway". `promote` sweeps first.
    ///
    /// ⛔ AND IT WALKS THE BOUNDARY IN BOTH DIRECTIONS (§7.1.1 rule 3), which is the half that
    /// gets forgotten. `Monotonic::ORIGIN.saturating_add(FOR_EVER)` saturates AT `u64::MAX`, and
    /// `collect_expired` compares `expires_at <= now`, so the last representable instant on the
    /// axis is exactly the one at which a "permanent" grant IS swept. INSIDE the window --
    /// `u64::MAX - 1` -- both quotas survive; AT `u64::MAX` both are collected and `allocated()`
    /// comes back `Mib::ZERO`. The window is half-open at both ends, which is one rule and not
    /// two.
    ///
    /// ⚠️ AND THE OUTER SIDE IS WHAT THE COMMENT BESIDE `FOR_EVER` ASSERTS. It was written to
    /// record a measurement and then held by nothing, which is gotcha #14 inside the paragraph
    /// that exists to answer it; the second sweep below is what makes it a control.
    #[test]
    fn a_permanent_grant_survives_to_the_last_instant_of_the_axis_and_is_swept_at_it() {
        let mut arbiter = the_production_arbiter();

        let promoted = arbiter.promote(Monotonic::from_millis(u64::MAX - 1));

        assert!(
            promoted.is_empty(),
            "nothing was ever queued, so nothing can come out of a queue"
        );
        assert_eq!(
            arbiter.allocated(),
            AUDIO_QUOTA.saturating_add(PRESENTATION_QUOTA),
            "a permanent grant is still held after a sweep 584 million years out"
        );

        let swept = arbiter.promote(Monotonic::from_millis(u64::MAX));

        assert!(
            swept.is_empty(),
            "still nothing was ever queued, so still nothing comes out of a queue"
        );
        assert_eq!(
            arbiter.allocated(),
            Mib::ZERO,
            "the OTHER side of the boundary: at the last instant a permanent grant IS swept"
        );
    }

    /// ⛔ WHAT HOLDS THE TWO FIELDS OF THE TWO RESERVATIONS, and without it each of the four
    /// was an assertion with no guard -- gotcha #14. These two constants are the only place in
    /// production code that CHOOSES `Preemption::Never`, and `ComputeClass::Realtime` is the
    /// PREMISE of the sentence beside `build_the_arbiter`: both profiles sit in one lane, so
    /// nothing inside the arbiter breaks the tie except arrival.
    /// ✅ MEASURED on 2026-08-21: with any ONE of the four changed and this probe absent, the
    /// WHOLE WORKSPACE stayed green -- 35 targets, 254 passed, 0 failed, 2 ignored, the
    /// baseline exactly. Four live mutants.
    ///
    /// ⛔ IT PINS THE VALUE AND NOT THE CONSEQUENCE, AND THE PRICE IS WRITTEN DOWN INSTEAD OF
    /// IMPLIED. This repository prefers the probe that attacks the MECHANISM, so that one was
    /// BUILT AND MEASURED first: an arbiter under `VramPolicy::Local`, where `may_make_room`
    /// answers yes, asked for more than is free and then swept past every grace. ⚠️ IT KILLS
    /// NEITHER FIELD, because inside `Arbiter::ask_back` the two stand behind ONE ANOTHER'S
    /// guard. `held.lane <= below` drops every `Realtime` grant -- `Realtime` is the top lane,
    /// so a `Realtime` grant is never BELOW the asking lane, whoever asks -- and that guard runs
    /// BEFORE the one that reads the grace, so `preemption` is never reached; with the lane
    /// changed alone, `Preemption::Never` gives no grace and the grant falls at the second
    /// guard instead. A probe no single mutation can kill is the vacuous probe, so it was not
    /// kept. What that road measured lives in the register, beside the mutation campaign.
    ///
    /// ⚖️ WHAT THIS ONE THEREFORE DOES NOT PROVE, said out loud: that either field changes
    /// anything the arbiter DOES. It says ADR-0033's word and the tie-break premise are still
    /// the ones written down.
    #[test]
    fn the_two_reservations_declare_no_preemption_and_one_lane() {
        for reservation in [&AUDIO_RESERVATION, &PRESENTATION_RESERVATION] {
            assert_eq!(
                reservation.preemption,
                Preemption::Never,
                "ADR-0033 asks for a NON-PREEMPTIBLE grant, and {} is where that is CHOSEN",
                reservation.name
            );
            assert_eq!(
                reservation.compute_class,
                ComputeClass::Realtime,
                "one lane for both is what leaves ARRIVAL the only tie-break, and {} left it",
                reservation.name
            );
        }
    }

    /// ⛔ THE SCENARIO OF `E41` EXACTLY, AND IT IS A PERMANENT PROBE AND NOT A MUTATION. A
    /// direction of proof held by a mutation is held by NOTHING: the mutation is reverted and
    /// the record is left saying the line is closed -- gotcha #72.
    ///
    /// `E41` says an impossible configuration stopped ANNOUNCING ITSELF when the queues
    /// arrived: the second permanent quota comes back `Queued`, and nobody will ever serve it
    /// because releasing a permanent grant is exactly what nobody does. The arbiter cannot
    /// repair that -- it cannot tell a ticket that WILL be served from one that never will --
    /// so the visibility belongs HERE, in the composition root, which asks for the two grants
    /// itself and can say so out loud.
    ///
    /// ⚠️ IT GOES THROUGH THE WHOLE GRAPH and not through a hand-built arbiter, which is what
    /// makes it hold the WIRING and not just `reserve`: with the two reservations taken out of
    /// `run_the_graph` this probe goes red.
    #[test]
    fn a_permanent_quota_that_only_queues_stops_the_start_up() {
        let dir = private_dir_for_line(line!());

        // 1024 fits in 1500; 1024 + 768 does not, and under `RemotePolicy` -- which may not
        // make room -- a request that fits the machine but not the moment is QUEUED.
        let outcome = run_the_graph(
            Parameters::new(EXECUTOR_TURN_LIMIT, Mib::new(1_500)),
            &dir.join("journal.redb"),
        );

        match outcome {
            Err(StartupError::ReservedQuota { name }) => assert_eq!(
                name, "presentation-reserved",
                "the failure must NAME the quota that did not get in"
            ),
            other => panic!("a permanent quota that only queues must stop the start-up: {other:?}"),
        }
    }

    /// The SECOND way the same failure arrives, and it is a second probe rather than a second
    /// assertion because the two travel by different roads inside `admit` (gotcha #65):
    /// "bigger than what is free right now" is `Queued`, "bigger than the whole machine" is
    /// `Refused`. One probe would leave whichever road it does not take uncovered.
    #[test]
    fn a_permanent_quota_bigger_than_the_machine_stops_the_start_up() {
        let dir = private_dir_for_line(line!());

        // 1024 is more than the whole machine, so no release will ever make room for it.
        let outcome = run_the_graph(
            Parameters::new(EXECUTOR_TURN_LIMIT, Mib::new(500)),
            &dir.join("journal.redb"),
        );

        match outcome {
            Err(StartupError::ReservedQuota { name }) => assert_eq!(
                name, "audio-reserved",
                "the failure must NAME the quota that did not get in"
            ),
            other => {
                panic!(
                    "a permanent quota bigger than the machine must stop the start-up: {other:?}"
                )
            }
        }
    }
}
