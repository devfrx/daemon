//! The DST campaign — level 1 of the two crash levels (ADR-0032): the subject under test is
//! THE KERNEL'S RECONCILIATION, and nothing here touches a disk.
//!
//! ⚠️ C1, C2, C3 and non-vacuity are NOT here: they are permanent tests since milestone 2, in
//! `crates/kernel/tests/executor_determinism.rs`. This milestone brings the FAULT.

use core::cell::RefCell;

use kernel::executor::{Executor, Sleep};
use kernel::parameters::Parameters;
use kernel::ports::journal::{Journal, StepId};
use kernel::reconcile::steps_in_doubt;
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
