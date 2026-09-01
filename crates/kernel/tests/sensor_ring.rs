//! The ring: it collects a verdict, journals it, and a NEGATIVE one opens a new step (V14, Q10).
//!
//! ⛔ EVERY PROBE HERE OPENS THE JUDGED STEP FIRST, AND THAT IS NOT SETUP NOISE — it is the
//! write-ahead discipline of ADR-0007 showing through. `Journal::note` says of itself that "a
//! note for a step with NO INTENT is `OutOfOrder`. A note is an annotation UPON something, and a
//! step nobody opened is not something", and `MemoryJournal` enforces it. A verdict is an
//! annotation upon the artefact of a step that EXISTS, and the one who opened that step is the
//! caller — never the ring. ⚠️ The dictated probes started from an empty journal and could not
//! pass: measured, `Err(OutOfOrder)`. Errata `E45`.

use core::cell::Cell;

use kernel::boundary::Untrusted;
use kernel::ports::journal::{Journal, StepId};
use kernel::record::{Detail, EffectClass, Record, RecordKind, RecordV1, Trust};
use kernel::sensor::{CostClass, Sensor, Verdict, VerdictOutcome, run_the_ring};
use kernel::time::Millis;
use simulator::journal::MemoryJournal;

/// A sensor whose verdict the TEST chooses — §6.4.2 asks for exactly this double.
///
/// ⛔ `spent` IS CHOSEN BY THE TEST AND NOT FIXED, AND IT HAD TO BECOME SO. It was the literal
/// `Millis::new(7)` until 2026-08-31, which made `7` the ONLY cost that ever crossed the
/// conversion in the whole workspace — so `spent_millis: 7` written as a CONSTANT in
/// `run_the_ring` survived every test there is, 41 targets and 298 passed, identical to the
/// baseline. ⚠️ THE PROBES DELIBERATELY USE DIFFERENT VALUES (gotcha #48): a ring that ignored
/// the sensor and wrote either constant fails on the other, which is what a single value can
/// never ask. It is the shape of `E65` on a field `E65` did not reach, and of the two-value
/// probe this same file already uses for the corrective step's class. Errata `E67`.
struct ScriptedSensor {
    cost: CostClass,
    outcome: VerdictOutcome,
    spent: u64,
}

impl Sensor for ScriptedSensor {
    fn declared_cost(&self) -> CostClass {
        self.cost
    }

    fn observe(&self, _artefact: &Untrusted) -> Verdict {
        Verdict {
            outcome: self.outcome,
            detail: Untrusted::new("field `name` is missing".into()),
            spent: Millis::new(self.spent),
        }
    }
}

fn records(journal: &MemoryJournal) -> Vec<(StepId, Record)> {
    journal
        .replay()
        .expect("replay")
        .into_iter()
        .map(|(step, bytes)| (step, Record::decode(&bytes).expect("decode")))
        .collect()
}

/// Opens the step whose artefact is about to be judged, and answers what it wrote.
///
/// ⛔ THE RING DOES NOT DO THIS, AND THE ASYMMETRY IS THE POINT: `run_the_ring` refuses to
/// allocate the id of the step it opens on failure — `StepId` has no allocator and whether one
/// arrives is the owner's — so it would be incoherent for it to mint the intent of a step it did
/// not open either.
fn open_the_step(journal: &mut MemoryJournal, step: StepId) {
    let intent = Record::V1(RecordV1 {
        kind: RecordKind::Intent,
        effect: EffectClass::Idempotent,
        trust: Trust::Instruction,
        payload: Vec::new(),
        reason: String::from("the step whose artefact is judged"),
        detail: None,
    })
    .encode();
    journal.intent(step, &intent).expect("intent");
}

#[test]
fn a_passing_sensor_writes_a_verdict_and_opens_nothing() {
    // ⛔ THIS IS ALSO THE COUNTER-PROBE OF CATALOGUE ROW V10 — "observing it and returning a
    // verdict compiles". It lives here and not beside the compile_fail case: gotcha #49.
    let mut journal = MemoryJournal::new();
    let judged = StepId::new(1);
    open_the_step(&mut journal, judged);

    let sensor = ScriptedSensor {
        cost: CostClass::Computational,
        outcome: VerdictOutcome::Pass,
        spent: 3,
    };

    let opened = run_the_ring(
        &sensor,
        &Untrusted::new("the artefact".into()),
        judged,
        StepId::new(2),
        EffectClass::Idempotent,
        &mut journal,
    )
    .expect("the ring");

    assert_eq!(opened, None);

    let written = records(&journal);
    assert_eq!(
        written.len(),
        2,
        "the intent of the judged step, and the verdict"
    );

    let (step, Record::V1(body)) = &written[1];
    assert_eq!(*step, judged);
    assert_eq!(body.kind, RecordKind::Verdict);

    // ⛔ AND `passed` IS TRUE HERE, WHICH IS THE ONLY `true` IN THE WORKSPACE. Every other site
    // of that field carries `false` — the frozen record, `reconciliation.rs`, `record_shape.rs`
    // and the failing probe below — so until this line a ring that wrote the CONSTANT `false`
    // was indistinguishable from one that computed the outcome. Measured: `passed: false` in
    // `run_the_ring` survived the whole workspace, 41 targets and 298 passed, identical to the
    // baseline. It is the second direction of a two-valued field, and the second direction is
    // the one that gets forgotten. Errata `E65`.
    let Some(Detail::Verdict(detail)) = &body.detail else {
        panic!("the verdict of a passing sensor carries no structured detail");
    };
    assert!(
        detail.passed,
        "a passing sensor wrote a verdict that says it failed"
    );
    // ⛔ AND `3` AND NOT `7`, WHICH IS THE WHOLE OF THIS LINE. The failing probe below asserts
    // `7`, so the two together refuse a ring that writes EITHER constant — and until this pair
    // existed `7` was the only cost that ever crossed the conversion, so `spent_millis: 7`
    // written as a literal in `run_the_ring` survived the whole workspace. Same shape as
    // `passed` above, on the field beside it. Errata `E67`.
    assert_eq!(detail.spent_millis, 3);

    // ⛔ AND THE TWO FIELDS NOBODY READ. `effect` is argued at length beside the record — it is
    // `Verifiable` because re-running a sensor over the same artefact answers the same thing —
    // and `reason` says what the record is FOR. Both went on the wire held by nothing: turned to
    // `Unrepeatable` and to another string, each survived the whole workspace. ⚠️ PINNED AND NOT
    // DECLARED, which is the task 10 boundary of milestone 5: a doc that AFFIRMS a value gets a
    // probe, and only a value some OPEN decision could still change is left declared (#73). No
    // decision is open here — `reconcile` provably never reads a verdict's class, measured by
    // mutating its arm in both directions.
    assert_eq!(body.effect, EffectClass::Verifiable);
    assert_eq!(body.reason, "a sensor judged the artefact of this step");

    // ⚠️ AND THE VERDICT IS ON THE JUDGED STEP AND NOWHERE ELSE. ⛔ RECALL OF 2026-08-31: this
    // said "a ring that wrote its verdict against the NEXT step would satisfy every line above
    // except this one", and the measurement says otherwise — such a ring reaches NO line above,
    // because `note` on a step nobody opened is `Err(OutOfOrder)` and the probe dies on the
    // `.expect` at the call. Measured: `note(next, ..)` gives 41 targets, 294 passed, 4 FAILED,
    // panicking with `the ring: OutOfOrder`.
    //
    // ⚠️ SO THIS ASSERTION IS DOMINATED, AND IT STAYS ANYWAY — declared rather than deleted.
    // Entry 0 is the intent this test wrote itself and entry 1 is already pinned three lines
    // up, so nothing a single mutation of `run_the_ring` can do makes it the FIRST to fail. It
    // costs one line and states the property in the form a reader looks for; what it must not
    // do is claim a discriminating power it does not have. Errata `E68`.
    assert!(
        written.iter().all(|(s, _)| *s == judged),
        "the ring touched a step other than the one it judged"
    );
}

#[test]
fn a_failing_verdict_opens_a_new_step_and_carries_the_detail() {
    // V14: "a negative verdict re-entering the ring is a NEW STEP, journalled" -- and Q10:
    // it re-enters WITHOUT HUMAN INTERVENTION, which is why nothing here asks anybody anything.
    let mut journal = MemoryJournal::new();
    let judged = StepId::new(1);
    let next = StepId::new(2);
    open_the_step(&mut journal, judged);

    let sensor = ScriptedSensor {
        cost: CostClass::Computational,
        outcome: VerdictOutcome::Fail,
        spent: 7,
    };

    let opened = run_the_ring(
        &sensor,
        &Untrusted::new("the artefact".into()),
        judged,
        next,
        EffectClass::Idempotent,
        &mut journal,
    )
    .expect("the ring");

    assert_eq!(opened, Some(next));

    let written = records(&journal);
    assert_eq!(
        written.len(),
        3,
        "the intent of the judged step, the verdict, and the intent of the new one"
    );

    // The verdict, upon the step that was judged.
    let (step, Record::V1(verdict)) = &written[1];
    assert_eq!(*step, judged);
    assert_eq!(verdict.kind, RecordKind::Verdict);
    // ⛔ THE ASSERTION IS ON THE ARCHIVE, NOT ON THE RETURN VALUE, and that is what keeps it from
    // being vacuous -- the same choice task 9 of milestone 5 made for the policy transition.
    let Some(Detail::Verdict(detail)) = &verdict.detail else {
        panic!("the verdict record carries no structured detail");
    };
    assert!(!detail.passed);
    assert_eq!(detail.spent_millis, 7);
    // The untrusted half travelled in the payload, under the label that says so.
    assert_eq!(verdict.payload, b"field `name` is missing");
    assert_eq!(verdict.trust, Trust::Untrusted);

    // The new step's intent, carrying the feedback.
    let (opened_step, Record::V1(intent)) = &written[2];
    assert_eq!(*opened_step, next);
    assert_eq!(intent.kind, RecordKind::Intent);
    assert_eq!(intent.payload, b"field `name` is missing");

    // ⛔ AND ITS LABEL, WHICH IS THE ONE THAT MATTERS: the payload of this record is the sensor's
    // detail, and the sensor read an `Untrusted` artefact. ADR-0014 makes the label HEREDITARY —
    // extracting, summarising or concatenating still yields untrusted content — so a feedback
    // record labelled `Instruction` would carry outside content across the instruction boundary
    // that I6 exists to keep shut, and would carry it into the DURABLE format.
    //
    // ⚠️ IT WAS HELD BY NOTHING UNTIL 2026-08-31, and the shape of the gap is worth more than
    // the fix: the line thirteen rows above asserts exactly this field on the VERDICT record, so
    // the bench held one of the two records the ring writes and not the other. Turned to
    // `Trust::Instruction` here, the whole workspace stayed green — 41 targets, 298 passed,
    // identical to the baseline. Errata `E65`.
    assert_eq!(intent.trust, Trust::Untrusted);
    // ⚠️ AND `detail` IS `None` ON PURPOSE: the structured half belongs to the VERDICT
    // species, and an `Intent` carrying a `Detail::Verdict` would enter the durable format
    // with a pair `RecordKind` and `Detail` that nothing at level 1 forbids. What holds the
    // pair is that ONE function per species builds the record, and this line is what holds
    // THIS function to it. Turned to `Some(..)`, the whole workspace stayed green — 41
    // targets, 298 passed, identical to the baseline. Errata `E73`.
    assert_eq!(intent.detail, None);
    assert_eq!(
        intent.reason,
        "a sensor verdict re-entered the ring as a new step"
    );
}

#[test]
fn an_inferential_sensor_is_refused_by_the_tight_ring() {
    // V11's first half: the DECLARED cost decides admission, and it is read BEFORE running.
    // ⚠️ The row stays `parziale` all the same -- its second half has no subject while no
    // inferential sensor exists (§8.3, trigger C4). Condition 12 of the design: do NOT mark it.
    let mut journal = MemoryJournal::new();
    let judged = StepId::new(1);
    open_the_step(&mut journal, judged);

    let sensor = ScriptedSensor {
        cost: CostClass::Inferential,
        outcome: VerdictOutcome::Fail,
        spent: 99,
    };

    let opened = run_the_ring(
        &sensor,
        &Untrusted::new("the artefact".into()),
        judged,
        StepId::new(2),
        EffectClass::Idempotent,
        &mut journal,
    )
    .expect("the ring");

    assert_eq!(opened, None);

    // ⛔ AND NOTHING WAS WRITTEN BEYOND THE INTENT THAT WAS ALREADY THERE, which is the half that
    // would be missed: a ring that refused to OPEN a step but journalled the verdict anyway would
    // pass the assertion above. ⚠️ THE ORACLE IS NOT `is_empty()` — the journal is not empty here
    // and cannot be, because a step has to exist before its artefact can be judged (E45). What is
    // asserted is that the ring added NOTHING.
    let written = records(&journal);
    assert_eq!(
        written.len(),
        1,
        "the ring wrote although it refused the sensor"
    );
    let (_, Record::V1(only)) = &written[0];
    assert_eq!(only.kind, RecordKind::Intent);
}

#[test]
fn the_class_of_the_corrective_step_is_the_one_the_caller_delivered() {
    // ⛔ THE RING DOES NOT KNOW WHAT THE CORRECTION WILL DO, SO IT DOES NOT NAME ITS CLASS. Until
    // 2026-08-31 it wrote a literal `Idempotent` and NOTHING held it: turned to `Unrepeatable`
    // the whole workspace stayed green — 41 targets, 297 passed, identical to the baseline. It
    // was not among this task's five dictated mutations, which is why it was never run. Errata
    // `E55`.
    //
    // ⚠️ TWO VALUES AND NOT ONE (gotcha #48), AND NEITHER IS THE OLD LITERAL: a ring that
    // ignored the argument and kept writing `Idempotent` fails on BOTH, so the probe cannot pass
    // by resembling the code it replaced.
    //
    // ⚠️ AND WHAT IS ASSERTED IS THE ARCHIVE, NOT THE RESOLUTION. That the class then decides the
    // reconciliation is `reconciliation.rs`'s half, proved there on its own values; composing two
    // proved facts into a third probe would be a second house for one property (§7.4.4).
    for effect in [EffectClass::Unrepeatable, EffectClass::Verifiable] {
        let mut journal = MemoryJournal::new();
        let judged = StepId::new(1);
        let next = StepId::new(2);
        open_the_step(&mut journal, judged);

        let sensor = ScriptedSensor {
            cost: CostClass::Computational,
            outcome: VerdictOutcome::Fail,
            spent: 7,
        };

        run_the_ring(
            &sensor,
            &Untrusted::new("the artefact".into()),
            judged,
            next,
            effect,
            &mut journal,
        )
        .expect("the ring");

        let written = records(&journal);
        let (opened, Record::V1(intent)) = &written[2];
        assert_eq!(*opened, next);
        assert_eq!(
            intent.effect, effect,
            "the ring wrote a class of its own instead of the one it was handed"
        );
    }
}

/// A sensor that records whether it was ever RUN. ⛔ `Cell` AND NOT A COUNTER: what is asked is
/// "did `observe` happen at all", and a count would invite an assertion about how many times the
/// ring runs a sensor, which nothing decides today.
struct WatchfulSensor {
    cost: CostClass,
    observed: Cell<bool>,
}

impl Sensor for WatchfulSensor {
    fn declared_cost(&self) -> CostClass {
        self.cost
    }

    fn observe(&self, _artefact: &Untrusted) -> Verdict {
        self.observed.set(true);
        Verdict {
            outcome: VerdictOutcome::Pass,
            detail: Untrusted::new("observed".into()),
            spent: Millis::new(1),
        }
    }
}

#[test]
fn an_inferential_sensor_is_never_run_at_all() {
    // ⛔ THIS IS THE SECOND HALF OF V11, AND IT WAS ADDED BECAUSE A MUTATION SURVIVED. The doc of
    // `run_the_ring` says the declared cost is read BEFORE `observe` and that "that ordering IS
    // V11". Measured on 2026-09-01 by moving the cost check AFTER `observe`: the whole workspace
    // stayed GREEN — `an_inferential_sensor_is_refused_by_the_tight_ring` only ever looked at
    // what was WRITTEN, and a sensor that runs and is then discarded writes nothing either.
    //
    // ⛔ AND THE DIFFERENCE IS NOT COSMETIC: §6.4.1 refuses an inferential sensor from the tight
    // ring because running it "turns every step into two inferences". A ring that runs it and
    // throws the answer away has paid the whole price and bought nothing — which is the one
    // outcome the rule exists to prevent, and the one the old probe could not see.
    let mut journal = MemoryJournal::new();
    let judged = StepId::new(1);
    open_the_step(&mut journal, judged);

    let sensor = WatchfulSensor {
        cost: CostClass::Inferential,
        observed: Cell::new(false),
    };

    let opened = run_the_ring(
        &sensor,
        &Untrusted::new("the artefact".into()),
        judged,
        StepId::new(2),
        EffectClass::Idempotent,
        &mut journal,
    )
    .expect("the ring");

    assert_eq!(opened, None);
    assert!(
        !sensor.observed.get(),
        "the ring RAN an inferential sensor and then discarded its verdict: the expense \
         happened and V11 bought nothing"
    );

    // ⚠️ AND THE OTHER DIRECTION, without which the line above is green for a ring that runs
    // NOTHING: a computational sensor IS run.
    let admitted = WatchfulSensor {
        cost: CostClass::Computational,
        observed: Cell::new(false),
    };
    run_the_ring(
        &admitted,
        &Untrusted::new("the artefact".into()),
        judged,
        StepId::new(3),
        EffectClass::Idempotent,
        &mut journal,
    )
    .expect("the ring");
    assert!(
        admitted.observed.get(),
        "the ring did not run a computational sensor either: it refuses everything"
    );
}
