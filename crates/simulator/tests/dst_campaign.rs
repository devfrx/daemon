//! The DST campaign — level 1 of the two crash levels (ADR-0032): the subject under test is
//! THE KERNEL'S RECONCILIATION, and nothing here touches a disk.
//!
//! ⚠️ C1, C2, C3 and non-vacuity are NOT here: they are permanent tests since milestone 2, in
//! `crates/kernel/tests/executor_determinism.rs`. This milestone brings the FAULT.

use core::cell::RefCell;

use kernel::executor::{Executor, Sleep};
use kernel::parameters::Parameters;
use kernel::ports::journal::{Journal, StepId};
use kernel::reconcile::{Resolution, steps_in_doubt};
use kernel::record::{EffectClass, Record, RecordKind, RecordV1, Trust};
use kernel::time::Monotonic;
use simulator::journal::CrashingJournal;
use simulator::reactor::VirtualReactor;
use simulator::rng::SeededRng;

const TURN_LIMIT: u64 = 10_000;
const ACTIVITIES: usize = 3;
const STEPS: usize = 4;

/// How many writes the scenario performs when nothing falls: two per step — the intent and the
/// outcome — which is the cost ADR-0007 accepts for the write-ahead discipline.
///
/// ⛔ IT IS PINNED BY A TEST rather than trusted, because the crash point is drawn BELOW this
/// number: were the scenario to perform fewer writes, the tail of the range would never fire
/// and those seeds would be silent no-ops. Gotcha #17.
const WRITES_PER_RUN: u64 = (ACTIVITIES * STEPS * 2) as u64;

/// How many seeds the crash campaign sweeps.
const CAMPAIGN_SEEDS: u64 = 200;

/// ⛔ THE CRASH POINT IS DRAWN FROM A DIFFERENT GENERATOR THAN THE INTERLEAVING, and from a
/// seed DERIVED from this one rather than from the same number. Two `SeededRng` built from the
/// same seed produce the SAME sequence, so the crash point would be a function of the first
/// shuffle and the campaign would explore a DIAGONAL of the space instead of the space.
fn crash_seed(seed: u64) -> u64 {
    seed ^ 0x9E37_79B9_7F4A_7C15
}

/// A record of the shape every step of this scenario writes.
///
/// ⚠️ The class is `Idempotent` for every step, so the resolution the reconciliation must
/// answer is a single expected value — which is what will let task 3 assert the RESOLUTION and
/// not only the set.
fn record(kind: RecordKind) -> Vec<u8> {
    Record::V1(RecordV1 {
        kind,
        effect: EffectClass::Idempotent,
        trust: Trust::Instruction,
        payload: Vec::new(),
        reason: String::from("a step of the DST scenario"),
    })
    .encode()
}

/// What the scenario SUCCEEDED in writing, in order. It is the independent oracle of `C7b`.
///
/// ⛔ IT COMES FROM THE SCENARIO AND NOT FROM THE ARCHIVE, and that is what will keep `C7b`
/// from being a tautology: `steps_in_doubt` walks the DECODED archive, this walks what the
/// activities were told went through. A journal that dropped a record, or a decode that misread
/// `kind`, makes the two disagree.
type Trace = Vec<(u64, RecordKind)>;

/// The M-2 scenario, now journalled: `ACTIVITIES` activities x `STEPS` steps, each step writing
/// its intent, waiting on a deadline of the VIRTUAL clock, then writing its outcome.
///
/// ⚠️ THE SHAPE IS NAMED AND NOT SPELLED OUT — this line read "3 activities x 4 steps" and
/// "5000 VIRTUAL milliseconds" until the review of 2026-08-11. Prose that restates a constant
/// goes false IN SILENCE the day the constant moves, which is gotcha #31, and the numbers here
/// are `ACTIVITIES`, `STEPS` and the literal nine lines below. ⚠️ The same sentence in
/// `crates/kernel/tests/executor_determinism.rs` is CORRECT as it stands, and the difference is
/// worth the line: there the numbers are the literals of the code itself, with no constant to
/// drift away from.
///
/// ⛔ THAT THIS SCENARIO REALLY INTERLEAVES IS NOT HELD HERE, and saying so is cheaper than a
/// reader deducing it. Nothing in this file goes red if the activities run one after the other:
/// the write count is the same, and with no crash there is no doubt either way. It is held one
/// task away, by `c7b`'s `a_crash_leaves_more_than_one_step_in_doubt_on_at_least_one_seed` —
/// measured against a sequential counterfactual on 2026-08-11, where the largest doubt set drops
/// from THREE to ONE and that probe goes red. ⚠️ The declared price: if that probe ever fails,
/// the diagnosis has two candidates — no interleaving, or a wrong reconciliation.
fn run(seed: u64, journal: CrashingJournal) -> (CrashingJournal, Trace) {
    let journal = RefCell::new(journal);
    let trace: RefCell<Trace> = RefCell::new(Vec::new());
    let sleep = Sleep::new();
    let mut executor = Executor::new(
        SeededRng::new(seed),
        VirtualReactor::new(),
        Parameters::new(TURN_LIMIT),
        &sleep,
    );

    for activity in 0..ACTIVITIES {
        let journal = &journal;
        let trace = &trace;
        let sleep = &sleep;
        executor.spawn(async move {
            for step in 0..STEPS {
                let id = (activity * STEPS + step) as u64;

                // ⛔ AN ERROR FROM THE JOURNAL IS THE PROCESS DYING, not a case to handle.
                // The activity returns and writes nothing more — and since the journal refuses
                // everything after the first fall (decision D2), so do all the others.
                if journal
                    .borrow_mut()
                    .intent(StepId::new(id), &record(RecordKind::Intent))
                    .is_err()
                {
                    return;
                }
                trace.borrow_mut().push((id, RecordKind::Intent));

                // Suspend on a PORT: the reactor is the only thing that can bring this activity
                // back. §2.4.1.
                sleep.until(Monotonic::from_millis(((step as u64) + 1) * 5_000));
                Yield::once().await;

                if journal
                    .borrow_mut()
                    .outcome(StepId::new(id), &record(RecordKind::Outcome))
                    .is_err()
                {
                    return;
                }
                trace.borrow_mut().push((id, RecordKind::Outcome));
            }
        });
    }

    executor.run().expect("the scenario terminates");
    // ⛔ Dropped EXPLICITLY: the tasks hold boxed futures that borrow the two cells, and a boxed
    // trait object carries drop glue, so `into_inner` would not compile otherwise.
    drop(executor);
    (journal.into_inner(), trace.into_inner())
}

/// A future that returns `Pending` exactly once. It is how an activity hands control back to
/// the executor after declaring a suspension.
///
/// ⚠️ DUPLICATED WORD FOR WORD from `crates/kernel/tests/executor_determinism.rs`, and it is
/// declared rather than left to be discovered: TEST CODE DOES NOT CROSS CRATE BOUNDARIES, so
/// there is no place both benches could reach. Whoever is tempted to unify the two would have
/// to promote it into a shipped crate, which would put a test helper on the wire of `kernel`
/// for the convenience of not repeating the lines below.
struct Yield(bool);

impl Yield {
    fn once() -> Self {
        Yield(false)
    }
}

impl core::future::Future for Yield {
    type Output = ();
    fn poll(
        mut self: core::pin::Pin<&mut Self>,
        _context: &mut core::task::Context<'_>,
    ) -> core::task::Poll<()> {
        if self.0 {
            core::task::Poll::Ready(())
        } else {
            self.0 = true;
            core::task::Poll::Pending
        }
    }
}

#[test]
fn the_scenario_really_writes_what_the_campaign_assumes() {
    // ⛔ GOTCHA #17, and it is asserted rather than commented: the crash point is drawn below
    // `WRITES_PER_RUN`, so if the scenario performed fewer writes the tail of the range would
    // be silent and those seeds would prove nothing while passing.
    //
    // ⛔ THE PIN BELOW FIXES A RELATION, NOT A NUMBER, and a relation is satisfied by zero. With
    // `ACTIVITIES` or `STEPS` at zero the constant is 0, both assertions read `0 == 0` and stay
    // green — while task 3 would hand that 0 to `from_seed`, whose own guard against it is a
    // `debug_assert!` and is compiled away in release.
    assert!(
        WRITES_PER_RUN > 0,
        "a scenario with no writes has nothing to fall at"
    );

    let (journal, trace) = run(20_260_806, CrashingJournal::without_crash());
    assert_eq!(journal.writes_done(), WRITES_PER_RUN);

    // ⚠️ AND THIS ONE IS NOT A SECOND OPINION ON THE LINE ABOVE: it is the ONLY thing anchoring
    // the trace in this commit. `Trace` exists for `C7b`'s independent oracle, which arrives at
    // task 3, so until then nothing else would notice a scenario that journalled correctly and
    // recorded nothing — or the reverse.
    assert_eq!(trace.len() as u64, WRITES_PER_RUN);
}

#[test]
fn c7a_without_a_crash_no_step_is_in_doubt() {
    // ⛔ NO FALSE POSITIVES. It is the half that is easy to skip, and the one that says the
    // doubt reported by C7b means something.
    for seed in 0..50u64 {
        let (journal, _) = run(seed, CrashingJournal::without_crash());

        // ⛔ C7a's NON-VACUITY ORACLE, and it is the mirror of `has_fallen()` on the other half
        // of the campaign. "No step is in doubt" and "the scenario wrote nothing to be in doubt
        // about" are THE SAME GREEN — measured, not argued: run this loop with a journal that
        // falls at write 0 and the archive is empty, the trace is empty, and the assertion below
        // is satisfied by a run that did nothing at all. Without this line that distinction
        // lives in a mutation somebody ran once, which is a note, and a note is read and
        // forgotten (gotcha #36).
        assert_eq!(
            journal.writes_done(),
            WRITES_PER_RUN,
            "seed {seed}: nothing was written to be in doubt about"
        );

        let survivor = journal.into_survivor();
        assert_eq!(
            steps_in_doubt(&survivor).expect("replay"),
            Vec::new(),
            "seed {seed} left a doubt with no crash"
        );
    }
}

/// The steps left with an intent and no outcome, computed FROM THE SCENARIO'S TRACE.
///
/// ⛔ WHY THIS IS NOT A TAUTOLOGY, said here because it looks like one: the algorithm is the
/// same shape as `steps_in_doubt`, but the INPUT is not. This walks what the activities were
/// told went through; `steps_in_doubt` walks the bytes that came back out of the archive, after
/// decoding. A journal that lost a record, an encode that dropped a field, a decode that
/// misread `kind` — each makes the two disagree, and none of them would show if the expectation
/// were computed from the archive.
///
/// ⚠️ THE `contains` GUARD IS UNREACHABLE IN THIS SCENARIO, and it stays — declared rather than
/// removed. No step here is ever given two intents: the ids are distinct per activity, and a
/// second intent would be refused by the port, so it would never reach the trace at all. It
/// mirrors what `enter` really does on the other side — a step enters the doubt AT MOST ONCE
/// and keeps the place it first took — and dropping it would make the two algorithms diverge
/// for a case the scenario could grow into. It is insurance with its reach written down, not
/// coverage.
fn expected_doubt(trace: &Trace) -> Vec<u64> {
    let mut open: Vec<u64> = Vec::new();
    for (step, kind) in trace {
        match kind {
            RecordKind::Intent => {
                if !open.contains(step) {
                    open.push(*step);
                }
            }
            RecordKind::Outcome => open.retain(|s| s != step),
            RecordKind::Note => {}
        }
    }
    open
}

#[test]
fn c7b_a_crash_leaves_exactly_the_steps_the_scenario_left_open() {
    // ⛔ THE SET AND NOT ITS SIZE. Measured on the spike, seed 99 left `[3, 7]`: with
    // interleaved execution one crash leaves SEVERAL steps in doubt together, and a bench that
    // compared only how many would pass on the wrong ones. Gotcha #30, and #20.
    //
    // ⛔ AND THE COMPARISON IS ORDERED, NOT SET-WISE, WHICH IS FREE HERE AND MUST STAY. Between
    // the write to the journal and the push onto the trace there is no `await`, so archive and
    // trace are in lockstep and the two orders agree — measured over these seeds. Weakening it
    // to a set "for prudence" would give away the defence against the class of defect that has
    // already cost this repository three vacuous probes: a comparison of bare identities passes
    // against a liar that reverses the order, because `1, 2, 1` is a palindrome.
    let mut crashes = 0u64;
    let mut largest = 0usize;

    for seed in 0..CAMPAIGN_SEEDS {
        let (journal, trace) = run(
            seed,
            CrashingJournal::from_seed(crash_seed(seed), WRITES_PER_RUN),
        );
        let fell = journal.has_fallen();
        let point = journal.falls_at();
        let survivor = journal.into_survivor();

        let expected = expected_doubt(&trace);
        let doubts = steps_in_doubt(&survivor).expect("replay");
        let found: Vec<u64> = doubts.iter().map(|d| d.step.get()).collect();

        assert_eq!(found, expected, "seed {seed}, crash at write {point}");

        // ⛔ EVERY STEP OF THIS SCENARIO DECLARES `Idempotent`, so the resolution is decided and
        // not incidental. Without this the campaign would hold WHICH steps are in doubt and say
        // nothing about WHAT TO DO with them, which is the half ADR-0007 exists for.
        for doubt in &doubts {
            assert_eq!(
                doubt.resolution,
                Resolution::RunAgain,
                "seed {seed}, step {}",
                doubt.step.get()
            );
        }

        if fell {
            crashes += 1;
        }
        largest = largest.max(doubts.len());
    }

    // ⛔ THE NON-VACUITY, AND IT IS THE POINT OF THE WHOLE TEST — but it is asserted as an
    // EQUALITY and not as `> 0`, and the difference is not pedantry. The point is drawn inside
    // `0..WRITES_PER_RUN` and the scenario performs exactly `WRITES_PER_RUN` writes when nothing
    // falls, so EVERY seed must reach its point: a single seed that did not crash would mean the
    // scenario performed fewer writes than the number the point was drawn against, which is
    // precisely the silent no-op of gotcha #17. `> 0` would let 199 out of 200 go quiet.
    assert_eq!(
        crashes, CAMPAIGN_SEEDS,
        "a seed did not reach its crash point: the scenario wrote fewer times than {WRITES_PER_RUN}"
    );

    // A MEASUREMENT, printed rather than guessed — run with `-- --nocapture`. It is what the
    // seed list of task 8 and the campaign size of task 4 are chosen against.
    println!("DST L1 c7b: {crashes}/{CAMPAIGN_SEEDS} seeds crashed, largest doubt set {largest}");
}

#[test]
fn a_crash_leaves_more_than_one_step_in_doubt_on_at_least_one_seed() {
    // ⛔ FINDING 2 OF §3.6.1, held on the campaign rather than on a hand-built archive. The
    // in-tree probe `a_crash_leaves_more_than_one_step_in_doubt` in
    // `crates/kernel/tests/reconciliation.rs` builds the state BY HAND; this one gets there
    // through the executor, which is the only way to know the interleaving really produces it.
    //
    // ⛔ AND IT IS THE ONLY THING HOLDING THAT THIS SCENARIO INTERLEAVES AT ALL — the doc of
    // `run` says so and names this test, so the name is a commitment. Measured on 2026-08-11
    // against a sequential counterfactual: the largest doubt set drops from THREE to ONE, so
    // this probe really does go red if the activities stop overlapping.
    let mut best = 0usize;
    for seed in 0..CAMPAIGN_SEEDS {
        let (journal, _) = run(
            seed,
            CrashingJournal::from_seed(crash_seed(seed), WRITES_PER_RUN),
        );
        let survivor = journal.into_survivor();
        best = best.max(steps_in_doubt(&survivor).expect("replay").len());
    }
    assert!(best > 1, "no seed left more than one step in doubt: {best}");
    println!("DST L1 interleaving: largest doubt set over {CAMPAIGN_SEEDS} seeds is {best}");
}
