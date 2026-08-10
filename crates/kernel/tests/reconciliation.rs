//! Reconciliation on a SET (§4.3), and the counter-probes of the rules it applies.

use kernel::ports::journal::{Journal, StepId};
use kernel::reconcile::{InDoubt, Resolution, steps_in_doubt};
use kernel::record::{EffectClass, Record, RecordKind, RecordV1, Trust};
use simulator::journal::MemoryJournal;

fn record(kind: RecordKind, effect: EffectClass) -> Vec<u8> {
    Record::V1(RecordV1 {
        kind,
        effect,
        trust: Trust::Instruction,
        payload: Vec::new(),
    })
    .encode()
}

#[test]
fn a_crash_leaves_more_than_one_step_in_doubt() {
    // ⛔ Gotcha #20, and it is why this function returns a SET. Measured on the spike: seed 99
    // left `[3, 7]`. A helper that returns ONE step gives a false negative under interleaving.
    let mut journal = MemoryJournal::new();
    for step in [1u64, 3, 7] {
        journal
            .intent(
                StepId::new(step),
                &record(RecordKind::Intent, EffectClass::Idempotent),
            )
            .expect("intent");
    }
    journal
        .outcome(
            StepId::new(1),
            &record(RecordKind::Outcome, EffectClass::Idempotent),
        )
        .expect("outcome");

    let in_doubt = steps_in_doubt(&journal).expect("reconcile");

    assert_eq!(
        in_doubt.iter().map(|d| d.step).collect::<Vec<_>>(),
        vec![StepId::new(3), StepId::new(7)]
    );
}

#[test]
fn a_step_with_both_intent_and_outcome_is_not_in_doubt() {
    // The direction that is forgotten (§7.1.1 rule 3): the check must NOT fire where it must
    // not. A reconciliation that reports everything is as useless as one that reports nothing.
    let mut journal = MemoryJournal::new();
    journal
        .intent(
            StepId::new(1),
            &record(RecordKind::Intent, EffectClass::Idempotent),
        )
        .expect("intent");
    journal
        .outcome(
            StepId::new(1),
            &record(RecordKind::Outcome, EffectClass::Idempotent),
        )
        .expect("outcome");

    assert!(steps_in_doubt(&journal).expect("reconcile").is_empty());
}

#[test]
fn the_class_decides_the_resolution() {
    let mut journal = MemoryJournal::new();
    journal
        .intent(
            StepId::new(1),
            &record(RecordKind::Intent, EffectClass::Verifiable),
        )
        .expect("intent");
    journal
        .intent(
            StepId::new(2),
            &record(RecordKind::Intent, EffectClass::Idempotent),
        )
        .expect("intent");
    journal
        .intent(
            StepId::new(3),
            &record(RecordKind::Intent, EffectClass::Unrepeatable),
        )
        .expect("intent");

    let resolutions: Vec<Resolution> = steps_in_doubt(&journal)
        .expect("reconcile")
        .iter()
        .map(|d| d.resolution)
        .collect();

    assert_eq!(
        resolutions,
        vec![
            Resolution::AskTheWorld,
            Resolution::RunAgain,
            Resolution::SuspendAndAsk
        ]
    );
}

#[test]
fn a_record_that_will_not_decode_is_treated_as_unrepeatable() {
    // ⛔ ADR-0007: an effect with no declared class is treated as `Unrepeatable` — in front of
    // a doubt it cannot resolve, THE SYSTEM STOPS, it does not guess. A record this build
    // cannot read is the strongest form of that case.
    let mut journal = MemoryJournal::new();
    journal
        .intent(StepId::new(1), b"not a record at all")
        .expect("intent");

    let in_doubt = steps_in_doubt(&journal).expect("reconcile");

    assert_eq!(in_doubt.len(), 1);
    assert_eq!(in_doubt[0].resolution, Resolution::SuspendAndAsk);
}

#[test]
fn an_empty_journal_leaves_nothing_in_doubt() {
    // ⚠️ THE CASE NO DICTATED TEST COVERED, and it is not a formality: this is the state after
    // the very first start, before anything has ever been written, and it is the one input a
    // real resume meets before any other. An implementation that answered `Missing` on an empty
    // journal — a plausible reading of `replay` — would make the FIRST BOOT an error.
    //
    // ✅ AND IT IS THE ONLY TEST HERE THAT SEES THAT, measured rather than assumed: making
    // `MemoryJournal::replay` refuse an empty journal turns THIS ONE RED AND NOTHING ELSE. Every
    // other test writes something first, so none of them ever meets the empty case.
    let journal = MemoryJournal::new();

    assert!(steps_in_doubt(&journal).expect("reconcile").is_empty());
}

#[test]
fn the_set_comes_back_in_write_order_and_not_in_step_order() {
    // ⛔ THE ORDER IS A PROMISE OF THE PORT — "in the order the journal wrote them" — AND THE
    // OTHER TEST HOLDS IT ONLY BY ACCIDENT. `a_crash_leaves_more_than_one_step_in_doubt`
    // expects `[3, 7]`, which is write order AND numeric order at the same time, so a
    // reconciliation that SORTED its answer would leave it green. That is the palindrome
    // defect of errata E12 in a new dress: a probe that fits the case instead of the mechanism.
    //
    // ✅ AND THAT SENTENCE WAS MEASURED, not argued: sorting the answer by step — which needs a
    // temporary `Ord` on `StepId`, so it is a mutation of two files — turns THIS ONE RED AND
    // NOTHING ELSE, `a_crash_leaves_more_than_one_step_in_doubt` included. Reversing the answer
    // turns both red, which is why reversing alone would not have proved anything.
    //
    // Here the two orders disagree on purpose: written 7, 3, 1 and expected back 7, 3.
    let mut journal = MemoryJournal::new();
    for step in [7u64, 3, 1] {
        journal
            .intent(
                StepId::new(step),
                &record(RecordKind::Intent, EffectClass::Idempotent),
            )
            .expect("intent");
    }
    journal
        .outcome(
            StepId::new(1),
            &record(RecordKind::Outcome, EffectClass::Idempotent),
        )
        .expect("outcome");

    let in_doubt = steps_in_doubt(&journal).expect("reconcile");

    assert_eq!(
        in_doubt.iter().map(|d| d.step).collect::<Vec<_>>(),
        vec![StepId::new(7), StepId::new(3)],
        "the set came back sorted, not in the order the journal wrote it"
    );
}

#[test]
fn a_step_is_in_doubt_at_most_once_however_many_records_it_carries() {
    // ⛔ MEASURED BEFORE IT WAS FIXED, and the number is the point: with the first walk this
    // scenario answered `[{5, RunAgain}, {5, SuspendAndAsk}]` — THE SAME STEP TWICE. A function
    // called "the steps in doubt" that hands the same step back twice is not a set, and a
    // caller acting on it would suspend one step two times.
    //
    // The scenario is reachable through the port as it stands: a step may carry an intent this
    // build can read and an outcome it cannot.
    //
    // ⚠️ AND ITS WEAKNESS IS DECLARED RATHER THAN LEFT TO BE FOUND: NO MUTATION KILLED THIS TEST
    // ALONE. Every one that reddens it — the `Err` arm ignoring, the `Err` arm resolving to
    // `RunAgain`, `enter` always pushing, `enter` keeping the first answer — also reddens
    // `a_step_that_re_enters_doubt_keeps_the_place_it_first_took`, whose assertion compares the
    // WHOLE vector and therefore catches a duplicate too. What this one adds is the SCENARIO
    // (one step, two records, the second unreadable), not a defect only it can see.
    let mut journal = MemoryJournal::new();
    journal
        .intent(
            StepId::new(5),
            &record(RecordKind::Intent, EffectClass::Idempotent),
        )
        .expect("intent");
    journal
        .outcome(StepId::new(5), b"not a record at all")
        .expect("outcome");

    let in_doubt = steps_in_doubt(&journal).expect("reconcile");

    assert_eq!(in_doubt.len(), 1, "the same step came back more than once");
    // ⛔ AND THE UNREADABLE RECORD WINS OVER WHAT WAS READ BEFORE IT. It is not "the last word
    // wins" as a convenience: a record this build cannot decode says NOTHING about the step,
    // including whether it closed, so the reconciliation cannot resolve the doubt and ADR-0007
    // says that means stop. Keeping `RunAgain` here would re-run a step that may already have
    // happened.
    assert_eq!(in_doubt[0].resolution, Resolution::SuspendAndAsk);
}

#[test]
fn a_step_that_re_enters_doubt_keeps_the_place_it_first_took() {
    // ⚠️ A DELIBERATE CHOICE AND NOT A PROPERTY OF THE CONTAINER, so it is pinned. When a later
    // record changes a step's resolution, the step did not stop being in doubt and start again:
    // only the answer changed. So it keeps the position it took when it ENTERED the doubt, and
    // the order the caller walks stays the order the doubts appeared.
    //
    // ✅ ISOLATED BY ITS OWN MUTATION: an `enter` that removes the old entry and pushes at the
    // end — which KEEPS the set property, so it is the alternative a reader would reach for —
    // turns this one red and nothing else.
    let mut journal = MemoryJournal::new();
    for step in [1u64, 2] {
        journal
            .intent(
                StepId::new(step),
                &record(RecordKind::Intent, EffectClass::Idempotent),
            )
            .expect("intent");
    }
    journal
        .outcome(StepId::new(1), b"not a record at all")
        .expect("outcome");

    let in_doubt = steps_in_doubt(&journal).expect("reconcile");

    assert_eq!(
        in_doubt,
        vec![
            InDoubt {
                step: StepId::new(1),
                resolution: Resolution::SuspendAndAsk
            },
            InDoubt {
                step: StepId::new(2),
                resolution: Resolution::RunAgain
            }
        ],
        "the re-entering step was moved to the end instead of keeping its place"
    );
}

#[test]
fn an_unreadable_record_after_a_readable_outcome_puts_the_step_back_in_doubt() {
    // ⚠️ THE TWIN OF THE CASE ABOVE, and it answers the other way round for the same reason.
    // The step HAD a readable outcome, so at that moment it was resolved; then something this
    // build cannot read was written about it. The doubt is not "did the effect happen" any
    // more, it is "what does that record say", and an unresolvable doubt suspends.
    //
    // ⚠️ THIS ONE WAS ALREADY GREEN BEFORE THE SET WAS FIXED, and saying so is the honest state
    // of it: it is here to pin behaviour the fix must not change, not to have driven it. What
    // proves it is not vacuous is the mutation table — an `Err` arm that ignores turns it red.
    let mut journal = MemoryJournal::new();
    journal
        .intent(
            StepId::new(5),
            &record(RecordKind::Intent, EffectClass::Idempotent),
        )
        .expect("intent");
    journal
        .outcome(
            StepId::new(5),
            &record(RecordKind::Outcome, EffectClass::Idempotent),
        )
        .expect("outcome");
    journal
        .outcome(StepId::new(5), b"not a record at all")
        .expect("outcome");

    let in_doubt = steps_in_doubt(&journal).expect("reconcile");

    assert_eq!(in_doubt.len(), 1, "the same step came back more than once");
    assert_eq!(in_doubt[0].step, StepId::new(5));
    assert_eq!(in_doubt[0].resolution, Resolution::SuspendAndAsk);
}
