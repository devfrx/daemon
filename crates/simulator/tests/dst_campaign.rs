//! The DST campaign — level 1 of the two crash levels (ADR-0032): the subject under test is
//! THE KERNEL'S RECONCILIATION, and nothing here touches a disk.
//!
//! ⚠️ C1, C2, C3 and non-vacuity are NOT here: they are permanent tests since milestone 2, in
//! `crates/kernel/tests/executor_determinism.rs`. This milestone brings the FAULT.

use core::cell::RefCell;
use std::collections::BTreeSet;

use kernel::arbiter::Mib;
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

/// ⚠️ A LITERAL OF THIS BENCH, and inert to this campaign: nothing here admits anything, and
/// the seed does not touch it. It is written because `Parameters` carries every delivered
/// value positionally — §2.8.5's friction — and because §2.8.2 rule 2 forbids the kernel to
/// name a default in its place.
const TOTAL_VRAM: Mib = Mib::new(16_384);

const ACTIVITIES: usize = 3;
const STEPS: usize = 4;

/// How many writes the scenario performs when nothing falls: two per step — the intent and the
/// outcome — which is the cost ADR-0007 accepts for the write-ahead discipline.
///
/// ⛔ IT IS PINNED BY A TEST rather than trusted, because the crash point is drawn BELOW this
/// number: were the scenario to perform fewer writes, the tail of the range would never fire
/// and those seeds would be silent no-ops. Gotcha #17.
const WRITES_PER_RUN: u64 = (ACTIVITIES * STEPS * 2) as u64;

/// How many seeds the SHORT level-1 campaign sweeps — `C7a` without a crash, `campaign` with one.
/// Constraint 7 of §11: the number is fixed here and versioned with the file, not drawn from the
/// clock or from an environment variable, so two runs of the gate sweep the same seeds.
///
/// ⛔ ONE NUMBER FOR THE TWO, AND IT IS A PREMISE AND NOT TIDINESS. `campaign` asserts that EVERY
/// seed reaches its crash point, which is sound only because the write count does not depend on
/// the seed — and the only thing holding that is `C7a`'s `writes_done() == WRITES_PER_RUN`. When
/// `C7a` swept a bare `0..50` and this one was 200, three quarters of the crash campaign rested
/// on a premise nobody checked for them. Measured on 2026-08-11: it held, zero deviations over
/// fifty thousand seeds — so this is a premise brought back under guard, not a defect repaired.
/// ⚠️ The day this number is raised, the support rises with it instead of falling silently.
///
/// ⛔ THE BUDGET IS MEASURED IN `debug` AND NOT IN `--release`, and the difference is not a
/// rounding. `gate.sh` runs `cargo test --workspace` with no `--release`, so debug is the profile
/// that pays. Measured on 2026-08-11, one crashing run: 4.4 µs optimised against 18.9 µs
/// unoptimised — a factor of 4.3. A budget set on the release figure overruns by that factor
/// exactly where the gate collects it.
///
/// ⛔ AND THE BUDGET IS ON THE WHOLE BINARY, NOT ON A LOOP, because more than one loop scales
/// with this number. ⚠️ The obvious `2 x seeds x cost` model is wrong by better than a third: a
/// crashing run STOPS at its point, so it does about half the writes and costs 18.9 µs, while
/// `C7a` runs the scenario to the end and costs 32.7 µs. The binary costs about `seeds x 51.6 µs`
/// — the interleaving probe breaks early and the pin uses one seed — which puts the one-second
/// ceiling near 19 000 seeds, not near 26 000.
///
/// ⛔ AND IT IS NOT SET TO THAT CEILING. A ceiling is a constraint and not a target, and past a
/// point more seeds buy nothing this campaign ASSERTS. Measured on 2026-08-11 over 20 000 seeds:
///
/// | seeds  | crash points hit | least hit | largest doubt set | distinct doubt sets |
/// |--------|------------------|-----------|-------------------|---------------------|
/// | 200    | 24/24            | 4         | 3                 | 92                  |
/// | 500    | 24/24            | 14        | 3                 | 105                 |
/// | 1 000  | 24/24            | 31        | 3                 | 108                 |
/// | 2 000  | 24/24            | 64        | 3                 | 109                 |
/// | 20 000 | 24/24            | 807       | 3                 | 109                 |
///
/// Every crash point was already covered at 200 seeds. `largest` saturates at 3 because its
/// ceiling is STRUCTURAL and equals `ACTIVITIES` — an activity is a sequential loop and holds at
/// most one step open. And the doubt vectors this campaign can ever compare are a FINITE set of
/// 109: the last one first appeared at seed 1038, and 19 000 further seeds produced not one more.
/// So this number closes that space, raises the thinnest crash point from 4 samples to 64, and
/// costs about 103 ms — a tenth of the ceiling, which is the margin for a machine slower than
/// the one that measured it.
///
/// ⚠️ AND THE MARGIN IS THINNER THAN "1.9x" MADE IT SOUND, which is why that phrasing is gone:
/// seed 1038 is the 52nd percentile of the sweep, not a comfortable early finish. Measured over
/// six mixing constants (see `EXPECTED_DOUBT_SETS`) the space closes at seeds 539, 610, 697, 802,
/// 1038 and 1166 — so the WORST observed close is at 58% of the sweep and the real margin is
/// 1.7x, not 1.9x.
///
/// ⚠️ WHAT MORE SEEDS DO STILL BUY, said so the paragraph above is not read as "the space is
/// exhausted": the distinct ARCHIVES handed to `steps_in_doubt` never saturate — 1 206 here,
/// 16 360 at a hundred times this. Interleaving diversity keeps growing; only the OUTCOMES it
/// produces stop. That growth is the whole reason `the_deep_campaign` exists, and its table is
/// on `DEEP_CAMPAIGN_SEEDS`.
const SHORT_CAMPAIGN_SEEDS: u64 = 2_000;

/// How many DISTINCT doubt vectors this scenario can produce at all. ⛔ IT IS THE ONE NUMBER THAT
/// TURNS THE CHOICE OF `SHORT_CAMPAIGN_SEEDS` INTO SOMETHING HELD, and before 2026-08-11 nothing
/// held it. The seed count was chosen because 109 distinct vectors appear and the last arrives at
/// seed 1038 — but that justification lived only in a doc comment. Raise `ACTIVITIES` to four and
/// the space grows, 2 000 seeds may no longer close it, and EVERY TEST IN THIS FILE STAYS GREEN
/// while the reason for the number quietly dies. `WRITES_PER_RUN` is pinned by a test for exactly
/// this reason; this was not.
///
/// ⛔ SO IT IS A CHANGE DETECTOR ON THE SHAPE OF THE SCENARIO, in the same posture as the frozen
/// bytes of `crates/kernel/tests/frozen_bytes.rs`: it fires precisely when the coverage claim
/// stops being true, and it fires in BOTH directions — a scenario that grew, and a seed count
/// that no longer reaches the end of the space.
///
/// ⛔ AND IT WAS NOT ADOPTED UNTIL IT WAS MEASURED NOT TO FIRE WHERE IT SHOULD NOT, which is
/// gotcha #24 and the reason a wrong version of this constant would be worse than no constant:
/// a check that cries on an innocent change teaches everyone to ignore the audit. The question
/// was whether 109 is a property of the SCENARIO or of the seeds that sample it. Measured on
/// 2026-08-11 by replacing the mixing constant of `crash_seed` with six different values, each
/// over `SHORT_CAMPAIGN_SEEDS` seeds:
///
/// | mixing constant      | distinct doubt sets | last new at seed |
/// |----------------------|---------------------|------------------|
/// | `0x9E3779B97F4A7C15` | 109                 | 1038             |
/// | `0xBF58476D1CE4E5B9` | 109                 | 610              |
/// | `0x94D049BB133111EB` | 109                 | 539              |
/// | `0xD6E8FEB86659FD93` | 109                 | 802              |
/// | `0xA0761D6478BD642F` | 109                 | 697              |
/// | `0x8EBC6AF09C88C6E3` | 109                 | 1166             |
///
/// The COUNT is invariant across all six; only WHEN the space closes moves. So 109 belongs to
/// `ACTIVITIES`, `STEPS` and the write-ahead shape, and a reshuffle does not disturb it — which
/// is what makes the assertion below a check rather than a bet.
///
/// ⚠️ WHAT INVALIDATES IT, named so a red is read as a decision and not as a defect: a change to
/// `ACTIVITIES`, to `STEPS`, or to the scenario's write pattern. When that red arrives the right
/// move is to re-measure the space and re-choose `SHORT_CAMPAIGN_SEEDS` against it, not to edit
/// this number until the bar turns green.
const EXPECTED_DOUBT_SETS: usize = 109;

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
/// answer is a single expected value — which is what lets the campaign assert the RESOLUTION and
/// not only the set. ⚠️ This sentence said "what WILL LET task 3" until 2026-08-11, and it is
/// dated rather than rewritten away: the assertion exists now, inside
/// `c7b_a_crash_leaves_exactly_the_steps_the_scenario_left_open`, and it is what makes this
/// single class load-bearing instead of merely convenient.
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
/// ⛔ IT COMES FROM THE SCENARIO AND NOT FROM THE ARCHIVE, and that is what keeps `C7b` from
/// being a tautology: `steps_in_doubt` walks the DECODED archive, this walks what the activities
/// were told went through. A journal that dropped a record, or a decode that misread `kind`,
/// makes the two disagree.
///
/// ✅ AND SINCE 2026-08-11 IT IS A MEASURE AND NO LONGER A PROMISE — this paragraph read "what
/// WILL keep", which was an argument. Measured: `expected_doubt`'s outcome arm was made blind, so
/// that the trace declared open every step it had ever opened, and the comparison went RED —
/// `left: [3]` from the archive against a right-hand side carrying all twelve steps. The two
/// sides really do come from different places.
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
/// reader deducing it. Nothing in THIS function goes red if the activities run one after the
/// other: the write count is the same, and with no crash there is no doubt either way. It is
/// held further down this same file, by `a_crash_leaves_more_than_one_step_in_doubt_on_at_least_one_seed`
/// — measured against a sequential counterfactual on 2026-08-11, where the largest doubt set
/// drops from THREE to ONE and that probe goes red. ⚠️ The declared price: if that probe ever
/// fails, the diagnosis has two candidates — no interleaving, or a wrong reconciliation.
///
/// ⚠️ TWO THINGS THIS PARAGRAPH GOT WRONG UNTIL 2026-08-11, dated rather than quietly fixed. It
/// said "one task away", and that task is this commit. And it called that probe "`c7b`'s", which
/// it never was: it is a SIBLING `#[test]`, not a helper of `c7b`, and the difference matters
/// because `c7b` now asserts the same predicate for its own reasons — a reader sent looking for
/// a part of `c7b` would find the wrong assertion and draw the wrong conclusion from its red.
fn run(seed: u64, journal: CrashingJournal) -> (CrashingJournal, Trace) {
    let journal = RefCell::new(journal);
    let trace: RefCell<Trace> = RefCell::new(Vec::new());
    let sleep = Sleep::new();
    let mut executor = Executor::new(
        SeededRng::new(seed),
        VirtualReactor::new(),
        Parameters::new(TURN_LIMIT, TOTAL_VRAM),
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
    //
    // ⛔ AND IT IS A `const` BLOCK AND NOT A RUNTIME `assert!`, since 2026-08-21: the operand is
    // a CONSTANT, so the guard belongs to the compiler and a run of this test cannot be what
    // holds it. It is §7.1.2's own preference — a rule that can rise to level 1 rises — and here
    // it costs one word. `clippy::assertions_on_constants` is what pointed at it.
    const {
        assert!(
            WRITES_PER_RUN > 0,
            "a scenario with no writes has nothing to fall at"
        )
    };

    let (journal, trace) = run(20_260_806, CrashingJournal::without_crash());
    assert_eq!(journal.writes_done(), WRITES_PER_RUN);

    // ⚠️ AND THIS ONE IS NOT A SECOND OPINION ON THE LINE ABOVE: it holds the trace against the
    // archive at the COUNT, where `expected_doubt` holds it at the CONTENT.
    //
    // ⚠️ ITS DECLARED REASON EXPIRED ON 2026-08-11, and the line is re-argued rather than left
    // standing on a dead one. It read "it is the ONLY thing anchoring the trace in this commit
    // … `C7b`'s independent oracle arrives at task 3, so until then nothing else would notice";
    // that oracle has arrived, and it notices far more than a count. What keeps this assertion
    // is a different argument: `C7b` only ever sees traces from runs that CRASHED, so a scenario
    // that recorded nothing in the tail of a complete run would still be invisible there.
    assert_eq!(trace.len() as u64, WRITES_PER_RUN);
}

#[test]
fn c7a_without_a_crash_no_step_is_in_doubt() {
    // ⛔ NO FALSE POSITIVES. It is the half that is easy to skip, and the one that says the
    // doubt reported by C7b means something.
    //
    // ⛔ THE SAME SEEDS AS `C7b`, AND THE SHARED CONSTANT IS LOAD-BEARING: the write count pinned
    // below is the premise `campaign`'s `crashes == seeds` rests on, so a range narrower than the
    // crash campaign's would leave most of that campaign's seeds unsupported. It read `0..50u64`
    // until the review of 2026-08-11. See `SHORT_CAMPAIGN_SEEDS`.
    //
    // ⚠️ THE SUPPORT IS FOR THE SHORT CAMPAIGN ONLY, AND THE GAP IS DECLARED RATHER THAN LEFT TO
    // BE FOUND: `the_deep_campaign` sweeps a hundred times these seeds and this loop does not
    // follow it there. ⚠️ BUT THE GAP IS NOT AN INFERENCE, and calling it one over-declared it —
    // the range was MEASURED on 2026-08-11 over the exact interval `the_deep_campaign` uses:
    // `writes_done() == WRITES_PER_RUN` for two hundred thousand seeds out of two hundred
    // thousand, zero deviations. What is missing is the guard on every future run, not the
    // evidence. The price of closing it would be a `C7a`-shaped loop inside the deep campaign,
    // which doubles the cost of the long cycle to re-check a claim measured twice.
    //
    // ⚠️ AND NO FALSE GREEN CAN HIDE IN THE GAP, which is what makes that trade acceptable rather
    // than merely cheap. If some seed did write fewer than `WRITES_PER_RUN` times, `expected_doubt`
    // would still be computed from the REAL trace of that run, so `found == expected` would go on
    // holding. What the deep campaign would lose is COVERAGE — a crash point drawn beyond the last
    // write never fires, gotcha #17 — and not CORRECTNESS.
    for seed in 0..SHORT_CAMPAIGN_SEEDS {
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
/// ⚠️ THE SECOND-INTENT CASE IS UNREACHABLE IN THIS SCENARIO, and it is ASSERTED rather than
/// absorbed. It was a `contains` guard until the review of 2026-08-11, justified as mirroring
/// what `enter` does on the other side — and that justification was the one place this oracle
/// was openly derived from READING THE IMPLEMENTATION, four paragraphs under a claim of
/// independence. The reasoning is on the arm below.
fn expected_doubt(trace: &Trace) -> Vec<u64> {
    let mut open: Vec<u64> = Vec::new();
    for (step, kind) in trace {
        match kind {
            RecordKind::Intent => {
                // ⛔ NOT A DEDUPLICATION BUT AN ASSERTED INVARIANT, and the difference is the
                // independence this oracle claims four paragraphs up. No step in this scenario
                // is ever given two intents — the ids are distinct per activity, and a second
                // one would be refused by the port before it could reach the trace. Were the
                // scenario to grow into that case, the obvious fix would be to mirror what
                // `enter` does, and from that moment the oracle would agree with the
                // implementation BY CONSTRUCTION — including when the implementation is wrong.
                // So the case fails loudly instead of being absorbed quietly.
                //
                // ⚠️ THE PRICE, DECLARED: the scenario can no longer grow into that case without
                // a red. That is the point and not a side effect — the red is a decision being
                // asked for, not a defect being reported.
                assert!(
                    !open.contains(step),
                    "step {step} was given two intents: this oracle is no longer independent \
                     of `enter` for that case, and the campaign must decide before it grows"
                );
                open.push(*step);
            }
            RecordKind::Outcome => open.retain(|s| s != step),
            RecordKind::Note => {}
        }
    }
    open
}

/// The campaign itself, over `seeds` seeds, answering `(how many crashed, the largest doubt set)`.
///
/// ⛔ THE PER-SEED ASSERTIONS LIVE IN HERE AND NOT IN THE CALLER, because they are WHAT THE
/// CAMPAIGN VERIFIES and not a garnish on it. What comes back out are the two NON-VACUITY
/// oracles — the numbers that say the sweep did work — and a caller that got only those and
/// asserted the rest itself would leave the deep campaign checking nothing per seed.
///
/// ⛔ AND IT IS A FUNCTION AND NOT A SECOND `#[test]`. The draft of this task added a `campaign`
/// ALONGSIDE `c7b_…`, which would have duplicated its body minus the resolution loop and minus
/// both non-vacuity oracles — and the weaker of the two copies is the one that would have been
/// called "the campaign that really runs". There is one body, and both entry points use it.
///
/// ⚠️ IT PRINTS ITS WALL TIME, AND THE GATE DOES NOT YET COLLECT IT. `gate.sh` runs
/// `cargo test --workspace` with no `--nocapture`, so today this line is visible only to whoever
/// runs the binary by hand. It is written for the step the gate does not have yet, not to
/// describe one it has.
fn campaign(seeds: u64) -> (u64, usize) {
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
    let started = std::time::Instant::now();
    let mut crashes = 0u64;
    let mut largest = 0usize;
    let mut seen: BTreeSet<Vec<u64>> = BTreeSet::new();

    for seed in 0..seeds {
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
        seen.insert(found);

        // ⛔ EVERY STEP OF THIS SCENARIO DECLARES `Idempotent`, so the resolution is decided and
        // not incidental. Without this the campaign would hold WHICH steps are in doubt and say
        // nothing about WHAT TO DO with them, which is the half ADR-0007 exists for.
        //
        // ⛔ AND IT LOOKS REDUNDANT AFTER THE SET, WHICH IS WHY THE PROOF IS WRITTEN HERE rather
        // than left to be re-derived (gotcha #45). No defect that breaks the SET can demonstrate
        // this block's worth: it would panic on the line above and this loop would never run.
        // Its independent value was established by one measurement and only one — MUTATION B of
        // 2026-08-11, `resolution_of` collapsed to a constant `SuspendAndAsk`, which leaves the
        // set exactly right and fires HERE: `left: SuspendAndAsk, right: RunAgain`, seed 0, step
        // 3. Delete this block and that mutation survives in silence.
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

    // A MEASUREMENT, printed rather than guessed — run with `-- --nocapture`, and NOT visible
    // under `gate.sh`, which captures it.
    //
    // ⚠️ WHAT `largest` IS AND WHAT IT IS NOT, because this line claimed to be what the seed list
    // of task 8 and the campaign size of task 4 are chosen against, and it cannot be either.
    // `largest` is bounded STRUCTURALLY by `ACTIVITIES` — an activity is a sequential loop and
    // holds at most one step open — so it saturates within the first handful of seeds and says
    // nothing about how many more would be worth sweeping. What it is: evidence that the
    // scenario interleaves, and the CEILING that evidence reaches. What task 4 was chosen
    // against instead is on `SHORT_CAMPAIGN_SEEDS`.
    //
    // ⛔ AND THE WALL TIME IS ON THIS LINE BECAUSE IT IS THE ONLY NUMBER HERE THAT NOBODY CAN
    // DERIVE. The budget on `SHORT_CAMPAIGN_SEEDS` is a measurement taken once, on one machine,
    // and a measurement taken once decays; printing it every run is what turns it into something
    // a reader can contradict.
    // ⛔ THE COVERAGE CLAIM, ASSERTED WHERE THE SWEEP HAPPENS. It is what keeps the choice of
    // `SHORT_CAMPAIGN_SEEDS` honest: the reason for that number is "this many seeds see the whole
    // space of doubt vectors", and without this line that reason is a sentence in a comment that
    // no run can contradict. Reasoning, the six-mixer stability measurement, and what a red here
    // means are all on `EXPECTED_DOUBT_SETS`.
    //
    // ⛔ IT LIVES IN `campaign` AND NOT IN A CALLER, WHICH IS A PRECONDITION ON THE ARGUMENT and
    // is stated rather than left to be tripped over: `seeds` must be large enough to saturate the
    // space, and both callers are. Calling `campaign` with a short list would fire HERE — which
    // is the intent and not a trap, since a sweep too short to close the space is exactly the
    // thing this constant exists to report. Measured in both directions on 2026-08-11: at 500
    // seeds it fires with `left: 105, right: 109`, at 2 000 it is silent.
    assert_eq!(
        seen.len(),
        EXPECTED_DOUBT_SETS,
        "the campaign saw {} of the {EXPECTED_DOUBT_SETS} doubt vectors this scenario can produce \
         — either the scenario changed shape, or a sweep of {seeds} seeds no longer reaches the \
         end of the space and the count must be re-measured",
        seen.len()
    );

    let elapsed = started.elapsed();
    println!(
        "DST L1 campaign: {crashes}/{seeds} seeds crashed, largest doubt set {largest}, \
         {} distinct doubt sets, {elapsed:?}",
        seen.len()
    );

    (crashes, largest)
}

/// C7b, and it IS the short campaign — one sweep per commit, not a second one beside it.
///
/// ⛔ CONSTRAINT 7 OF §11 IS DISCHARGED IN PART HERE, AND THE PART MATTERS. The constraint has
/// two halves. The first — the seed count is FIXED AND VERSIONED, not drawn from the clock or the
/// environment — is discharged: it is `SHORT_CAMPAIGN_SEEDS`, in this file, under review. The
/// second — the wall time is printed EVERY RUN, *so that the slowdown becomes visible* — splits
/// once more, into PRODUCING the number and SHOWING it.
///
/// ⚠️ PRODUCING IT IS DONE SINCE 2026-08-11, AND AT BOTH LEVELS: `campaign` prints it here, and
/// task 7 put the same line on the level-2 sweep in
/// `crates/platform/tests/engine_crash_consistency.rs`, which until that day printed its counts and
/// not its time — so the constraint was produced at one level of two and this paragraph did not
/// know it. SHOWING it is what is still missing: `gate.sh` runs `cargo test --workspace` with no
/// `--nocapture`, so on every commit BOTH lines go into a buffer nobody reads. **TASK 9 adds the
/// gate step that shows them.**
///
/// ⚠️ WHY THE DISTINCTION IS WORTH SEVEN LINES rather than a tidier "discharged": the closing
/// audit of this milestone reads these claims. A file saying "discharged" here would mark the
/// constraint closed and Task 9 redundant, and what would be left uncovered is the constraint's
/// entire PURPOSE — not that a number exists, but that somebody SEES the campaign getting slower.
/// The file also said "the gate does not yet collect it" eighty-five lines up, so before this
/// correction it asserted both things at once.
///
/// ⛔ AND DECISION D6: the campaign is a TEST and the gate does not grow a seventh check for it.
/// `gate.sh` already runs `cargo test --workspace`, so this sweep is on every commit by being an
/// ordinary `#[test]` — which is also why the count above is chosen against a wall-clock budget
/// instead of against how much sweeping would be nice.
#[test]
fn c7b_a_crash_leaves_exactly_the_steps_the_scenario_left_open() {
    let (crashes, largest) = campaign(SHORT_CAMPAIGN_SEEDS);

    // ⛔ THE NON-VACUITY, AND IT IS THE POINT OF THE WHOLE TEST — but it is asserted as an
    // EQUALITY and not as `> 0`, and the difference is not pedantry. The point is drawn inside
    // `0..WRITES_PER_RUN` and the scenario performs exactly `WRITES_PER_RUN` writes when nothing
    // falls, so EVERY seed must reach its point: a single seed that did not crash would mean the
    // scenario performed fewer writes than the number the point was drawn against, which is
    // precisely the silent no-op of gotcha #17. `> 0` would let all but one seed go quiet.
    assert_eq!(
        crashes, SHORT_CAMPAIGN_SEEDS,
        "a seed did not reach its crash point: the scenario wrote fewer times than {WRITES_PER_RUN}"
    );

    // ⛔ AND THIS IS C7b's OWN NON-VACUITY, WHICH THE LINE ABOVE IS NOT. That one says the fault
    // FIRED; this one says the fault left something to reconcile. They come apart, measured:
    // with a journal that falls at write 0 every seed crashes and every comparison is `[] == []`,
    // and with a single activity every crash leaves one step at most — in both cases the
    // assertion above is satisfied and this campaign has verified nothing. ⚠️ NINETY of these
    // seeds already compare two empty sets on their own merits — the line read "six of these two
    // hundred" until 2026-08-11, and it was the campaign size that moved, not the fact: six of
    // the first two hundred seeds re-measures exactly, and the rate walks 3.00%, 5.00%, 5.00%,
    // 4.50%, 4.33% at 200, 500, 1 000, 2 000 and 20 000 seeds — sampling noise around ~4.3%.
    //
    // ⚠️ IT IS THE SAME DEFECT `C7a` CARRIED UNTIL TWO TASKS AGO, re-imported: there it was
    // "nothing was written to be in doubt about", here it is "nothing was left in doubt to
    // disagree about". A campaign needs an oracle saying it did work, not only one saying the
    // injection went off.
    assert!(
        largest > 1,
        "no seed left more than one step in doubt: the campaign compared empty sets"
    );
}

/// The deep campaign. ⛔ `#[ignore]` rather than a shorter list: constraint 8 of §11 puts the
/// deep DST on a LONG CYCLE, and a campaign that made every commit slower would be turned off
/// by whoever waits for it.
///
/// ⚠️ WHAT IT BUYS OVER THE SHORT ONE IS ONE THING, AND IT IS NOT THE OBVIOUS ONES: not
/// crash-point coverage, complete at 200 seeds; not a bigger doubt set, whose ceiling is
/// `ACTIVITIES`; and not more distinct doubt vectors, since `EXPECTED_DOUBT_SETS` is closed by
/// the short campaign already. It is the count of DISTINCT ARCHIVES handed to `steps_in_doubt`.
///
/// ⛔ AND THE MULTIPLIER IS MEASURED, NOT EXTRAPOLATED. This doc declared an extrapolation until
/// 2026-08-11 — honest, but this repository measures, and the measurement cost two seconds:
///
/// | seeds   | distinct archives |
/// |---------|-------------------|
/// | 2 000   | 1 206             |
/// | 20 000  | 6 168             |
/// | 50 000  | 10 082            |
/// | 100 000 | 13 348            |
/// | 200 000 | 16 360            |
///
/// The curve is still rising at the far end — the last new archive arrived at seed 199 898, of
/// 200 000 — and it rises SUBLINEARLY, about `n^0.42`. That is the number this constant is
/// chosen against: `x100` buys **2.65 times** the archives of `x10`, not ten times, and 13.6
/// times the archives of the short campaign. The price is 3.9 s in debug, which is what makes
/// 2.65x worth having on a long cycle and not on a commit.
const DEEP_CAMPAIGN_SEEDS: u64 = SHORT_CAMPAIGN_SEEDS * 100;

#[test]
#[ignore = "the deep campaign belongs to the long cycle — constraint 8 of §11"]
fn the_deep_campaign() {
    let (crashes, largest) = campaign(DEEP_CAMPAIGN_SEEDS);

    // ⛔ THE SAME TWO ORACLES AS `c7b_…` AND NOT WEAKER ONES, because a campaign a hundred times
    // longer is a hundred times more expensive to run vacuously. Gotcha #17 does not become
    // less likely with more seeds — it becomes more silent.
    assert_eq!(
        crashes, DEEP_CAMPAIGN_SEEDS,
        "a seed did not reach its crash point: the scenario wrote fewer times than {WRITES_PER_RUN}"
    );
    assert!(
        largest > 1,
        "no seed left more than one step in doubt: the campaign compared empty sets"
    );
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
    //
    // ⚠️ THE OVERLAP IS DECLARED RATHER THAN LEFT TO BE DISCOVERED: since the review of
    // 2026-08-11 `c7b` asserts THE SAME PREDICATE at the end of its own sweep, as its
    // non-vacuity. So this probe buys no coverage `c7b` has not already bought, and it stays for
    // two things a merged assertion would lose — its NAME, which the doc of `fn run`,
    // `docs/HANDOFF.md` and `docs/porta-di-qualita.md` cite as the holder of the interleaving,
    // and its DIAGNOSIS: a red here reads "the scenario stopped interleaving", a red there reads
    // "the campaign got weaker", and the two want different repairs. ⚠️ It is also why this one
    // stops early — the maximum over the whole campaign is `c7b`'s business now.
    let mut best = 0usize;
    let mut swept = 0u64;
    for seed in 0..SHORT_CAMPAIGN_SEEDS {
        swept += 1;
        let (journal, _) = run(
            seed,
            CrashingJournal::from_seed(crash_seed(seed), WRITES_PER_RUN),
        );
        let survivor = journal.into_survivor();
        best = best.max(steps_in_doubt(&survivor).expect("replay").len());
        // ⛔ "ON AT LEAST ONE SEED" IS THE WHOLE CLAIM, and stopping here is what makes the name
        // true instead of merely satisfied. The maximum over the WHOLE campaign is `c7b`'s
        // business now that it asserts it; this probe exists for its name and for the diagnosis
        // it gives — a red here says THE SCENARIO STOPPED INTERLEAVING, which is what the doc of
        // `run` promises somebody holds, while a red there says the campaign got weaker.
        if best > 1 {
            break;
        }
    }
    assert!(best > 1, "no seed left more than one step in doubt: {best}");

    // ⚠️ IT REPORTS `swept` AND NOT THE CONSTANT, and the distinction was a real lie for the
    // length of one review: this line read "over {CAMPAIGN_SEEDS} seeds" and the `break` above
    // made it false the moment it was added — the probe stops at the FIRST seed that satisfies
    // it. How soon it stops is the interesting half anyway: it is how rare the interleaving is.
    //
    // ⚠️ AND THIS IS WHY RAISING THE SEED COUNT DID NOT MAKE THIS PROBE COST MORE: `swept` is
    // what it pays, and `swept` is decided by the scenario and not by the bound.
    println!(
        "DST L1 interleaving: doubt set {best} reached after {swept} of {SHORT_CAMPAIGN_SEEDS} seeds"
    );
}
