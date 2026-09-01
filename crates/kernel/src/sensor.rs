//! The sensor contract (§6.4, ADR-0009), and the ring that carries a verdict back.
//!
//! ⛔ THE CONTRACT IS DELIBERATELY POOR, and that is a decision and not an omission: ADR-0009
//! writes it `(artefact) -> (verdict, detail, cost)` and says a minimal contract can be widened
//! while a rich and wrong one cannot. RK-5 is already accepted: it gets revisited after the
//! SECOND real sensor in different areas, and if it does not stretch it BREAKS rather than
//! bends.

use crate::boundary::Untrusted;
use crate::ports::journal::{Journal, JournalError, StepId};
use crate::record::{EffectClass, Record, RecordV1, Trust, VerdictDetail};
use crate::time::Millis;

/// What a sensor costs BEFORE it runs (§6.4.1). ⛔ IT IS A CLASS AND NOT A NUMBER, and the
/// reason is what V11 actually asks: "inferential sensors stay OUT of the tight ring". That is
/// a partition, not a threshold — a number would invite a cutoff nobody has chosen.
///
/// ⚠️ THE SECOND VARIANT HAS NO IMPLEMENTOR TODAY, and that is written rather than hidden:
/// §8.3 keeps V11 at `parziale` precisely because no inferential sensor exists (trigger C4).
/// What exists here is the MECHANISM that will admit or refuse one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CostClass {
    /// Runs on the CPU and answers from the artefact alone — schema validation, a linter, a
    /// test. Admitted to the tight ring.
    Computational,
    /// Calls a model. ⛔ REFUSED BY THE TIGHT RING (V11), and the refusal is the point: an
    /// inferential sensor in the tight loop turns every step into two inferences.
    Inferential,
}

/// Whether the artefact passed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerdictOutcome {
    Pass,
    /// ⛔ A NEGATIVE VERDICT IS NOT AN ERROR (ADR-0013): it is an ordinary answer that re-enters
    /// the ring as feedback, and the correction is A NEW STEP. Modelling it as an `Err` would
    /// make schema non-conformance an exception, which is the thing ADR-0013 exists to refuse.
    Fail,
}

/// What a sensor answers. The triple of ADR-0009, with the two costs of §6.4.1 kept apart:
/// the DECLARED one is on the trait and is read before running, the SPENT one is here.
///
/// ⛔ `detail` IS `Untrusted` AND THAT IS FORCED, not defensive. ADR-0014 makes the label
/// HEREDITARY — "extracting, summarising, translating or concatenating still produces untrusted
/// content" — and a detail is computed FROM the artefact. For an inferential sensor it is model
/// output outright. So it travels in the record's `payload`, which is the box whose doc says it
/// holds "somebody else's" bytes, and never in `reason`.
///
/// ⚠️ WHAT THE SPENT COST IS AND IS NOT: it is what the SENSOR reports, not what the ring
/// measured. A sensor that lies about it is not caught here — sensors are kernel-side code and
/// the content comes from capacities (ADR-0009). The limit is declared rather than defended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
    pub outcome: VerdictOutcome,
    pub detail: Untrusted,
    pub spent: Millis,
}

/// The contract. ⛔ `observe` TAKES THE ARTEFACT BY SHARED REFERENCE AND THAT IS `V10` — a
/// sensor observes and changes nothing; correcting is ring 1's job (§6.4.2). The negative case
/// is `tests/compile_fail/sensor_modifies_the_artefact.rs`.
pub trait Sensor {
    /// Read BEFORE running, and it decides admission to the tight ring (§6.4.1, V11).
    fn declared_cost(&self) -> CostClass;

    /// Observe, and answer. ⛔ THE ARTEFACT IS `&`, NEVER `&mut`.
    fn observe(&self, artefact: &Untrusted) -> Verdict;
}

/// Runs one sensor over one artefact and carries the answer back into the journal.
///
/// ⛔ A FREE FUNCTION THAT TAKES THE PORT, like `reconcile::steps_in_doubt` — the project already
/// has this shape for "read the journal and derive", and this one writes as well. A struct
/// holding the journal would give the ring state, and I5 keeps state in one place.
///
/// ⛔ `next` IS DELIVERED AND NOT ALLOCATED, and that is not laziness: `StepId` HAS NO ALLOCATOR,
/// `ports/journal.rs` says so beside the type, and whether one arrives is the owner's — registered
/// and not taken since 2026-08-21. Inventing one here would take that decision by writing it.
///
/// ⛔ AND THE STEP MUST ALREADY BE OPEN: the verdict is written with `Journal::note`, whose
/// contract is that "a note for a step with NO INTENT is `OutOfOrder`". So this function answers
/// `Err(JournalError::OutOfOrder)` when `step` was never opened, and that is the write-ahead
/// discipline of ADR-0007 rather than a limitation of this function — an artefact belongs to a
/// step that exists, and the one who opened it is the caller. ⚠️ WRITTEN DOWN BECAUSE IT WAS
/// MEASURED THE HARD WAY: the dictated probes of this task judged a step nobody had opened, and
/// two of the three could not pass. Errata `E45`.
///
/// ⛔ AND `correction_effect` IS DELIVERED FOR THE SAME REASON `next` IS. The ring opens a step
/// it will not execute, and the class of an effect says how a DOUBT about it is reconciled
/// (ADR-0007) — so naming it here would classify an effect nobody has written yet. ⚠️ IT USED TO
/// BE A LITERAL `Idempotent` UNTIL 2026-08-31, held by nothing: turned to `Unrepeatable` the whole
/// workspace stayed green, 41 targets and 297 passed, identical to the baseline. ⛔ AND THE
/// DEFAULT ADR-0007 GIVES AN UNCLASSIFIED EFFECT IS THE OPPOSITE ONE — "an effect with no declared
/// class is treated as UNREPEATABLE: facing a doubt it cannot resolve the system STOPS, it does
/// not guess" — so the literal was not merely unheld, it guessed in the permissive direction.
/// Errata `E55`, decided by the owner on 2026-08-31.
///
/// Returns the id of the step it opened, or `None` when nothing was opened — either the verdict
/// passed, or the sensor was refused by the tight ring.
pub fn run_the_ring<S: Sensor, J: Journal>(
    sensor: &S,
    artefact: &Untrusted,
    step: StepId,
    next: StepId,
    correction_effect: EffectClass,
    journal: &mut J,
) -> Result<Option<StepId>, JournalError> {
    // ⛔ THE DECLARED COST IS READ BEFORE `observe` IS CALLED, and that ordering IS V11: a cost
    // that came back with the verdict would arrive after the expense (§6.4.1). Nothing is
    // journalled on this road — a sensor that never ran produced no verdict, and writing one
    // would be the record of an event that did not happen.
    if sensor.declared_cost() == CostClass::Inferential {
        return Ok(None);
    }

    let verdict = sensor.observe(artefact);

    // The verdict, upon the step whose artefact was judged. ⛔ `Verifiable` AND NOT
    // `Unrepeatable`: the class describes how a DOUBT about this record's effect would be
    // reconciled, and a verdict has no effect on the world — re-running the sensor over the same
    // artefact answers the same thing. ⚠️ It is never actually reconciled, because a `Verdict`
    // record opens no doubt (see `crate::reconcile`); the field is mandatory and must still be
    // true.
    let record = Record::V1(RecordV1::verdict(
        EffectClass::Verifiable,
        Trust::Untrusted,
        verdict.detail.as_str().as_bytes().to_vec(),
        "a sensor judged the artefact of this step",
        VerdictDetail {
            passed: verdict.outcome == VerdictOutcome::Pass,
            spent_millis: verdict.spent.get(),
        },
    ))
    .encode();
    journal.note(step, &record)?;

    if verdict.outcome == VerdictOutcome::Pass {
        return Ok(None);
    }

    // ⛔ A NEGATIVE VERDICT RE-ENTERS AS A NEW STEP (V14), AND NOBODY IS ASKED (Q10). The intent
    // carries the same untrusted detail as the feedback the next attempt has to answer.
    //
    // ⛔ AND THE CLASS IS THE CALLER'S, NOT A LITERAL. Unlike the verdict's class above — which
    // `reconcile` provably never reads — THIS one IS read:
    // `RecordKind::Intent => enter(.., resolution_of(body.effect))`, so it decides how every
    // corrective step this ring opens gets reconciled after a crash. And this record is the
    // step's ONLY intent (`E19` refuses a second one), so whatever is written here is fixed for
    // that step forever. The ring does not know what the correction will do; the caller does.
    // The reason it is delivered rather than invented is on the signature, with the measure.
    let feedback = Record::V1(RecordV1::intent(
        correction_effect,
        Trust::Untrusted,
        verdict.detail.as_str().as_bytes().to_vec(),
        "a sensor verdict re-entered the ring as a new step",
    ))
    .encode();
    journal.intent(next, &feedback)?;

    Ok(Some(next))
}
