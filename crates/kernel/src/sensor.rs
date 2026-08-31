//! The sensor contract (§6.4, ADR-0009), and the ring that carries a verdict back.
//!
//! ⛔ THE CONTRACT IS DELIBERATELY POOR, and that is a decision and not an omission: ADR-0009
//! writes it `(artefact) -> (verdict, detail, cost)` and says a minimal contract can be widened
//! while a rich and wrong one cannot. RK-5 is already accepted: it gets revisited after the
//! SECOND real sensor in different areas, and if it does not stretch it BREAKS rather than
//! bends.

use crate::boundary::Untrusted;
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
