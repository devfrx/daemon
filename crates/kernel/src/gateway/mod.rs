//! The gateway decisor (§6.2) and the proof of conformance (§6.3).
//!
//! ⛔ NO MODEL IS INVOKED FROM HERE, AND NONE EVER WILL BE (ADR-0020): the kernel routes, filters
//! and journals; the provider adapters are staged out of this milestone by rule C of §0.4. What
//! that buys is written in ADR-0020 itself — the kernel is testable end to end with no model in
//! existence, and this file is where that stops being a slogan.
//!
//! ⛔ THE CHAIN IS DELIVERED PER CALL AND NOT HELD IN `Parameters`, and the choice is written
//! rather than left to be inferred. ADR-0034 forbids the kernel to READ a parameter it was not
//! handed; an argument IS being handed one. `Parameters` is the shape for what is fixed at
//! CONSTRUCTION, and a candidate chain is derived per request from policy and request (ADR-0011).
//! ⚠️ THE NEIGHBOURING OPEN VOICE IS `E94` OF THE MILESTONE 5 PLAN, which asks the mirror
//! question about the arbiter's policy — it is the owner's and it is open; this line does not
//! answer it. ⛔ THE PLAN IS NAMED BECAUSE THE NUMBER ALONE IS NOT A REFERENCE, and that is
//! measured rather than feared: an errata voice is numbered per plan, and on 2026-09-01 the
//! Milestone 6 errata grew an `E94` of its own — a different voice, on `RoutingDetail`, shut the
//! same day. How wide the class is, and that it is the owner's to settle, is registered in the
//! Milestone 6 errata; this line names its own plan so that it does not depend on the answer.

use alloc::vec::Vec;

use crate::ports::journal::{Journal, JournalError, StepId};
use crate::record::{EffectClass, Record, RecordV1, RoutingDetail, Trust};

/// One candidate of the chain. ⛔ THE NAME IS `&'static str` AND THAT IS `I6`: a name arriving
/// from outside would be untrusted text inside a type the kernel DECIDES with (ADR-0014). It is
/// the same reasoning P-9 applied to `ResourceProfile`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Candidate {
    pub model: &'static str,
    /// Runs on this machine.
    pub local: bool,
    /// The provider keeps the data.
    pub retains: bool,
    pub price: u64,
}

/// ⛔ THE TWO CLASSES OF ADR-0012, AND THEY FAIL DIFFERENTLY. It is not a taxonomy: it is the
/// only thing that decides what happens when the chain runs out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintClass {
    /// Data and confidentiality. ⛔ FAIL CLOSED: an error, no fallback.
    Data,
    /// Quality and cost. ⛔ DECLARED DEGRADATION: it proceeds, saying so.
    Quality,
}

/// What a request demands of a candidate.
///
/// ⛔ A CANDIDATE THAT VIOLATES A CONSTRAINT IS NOT A FALLBACK: IT IS A DIFFERENT REQUEST
/// (ADR-0012), discarded before evaluation. That is why the filter runs first and the choice
/// second, rather than scoring everything and picking a winner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Constraint {
    /// The request may not leave this machine.
    LocalOnly,
    /// The provider may not keep the data.
    NoRetention,
    /// ⚠️ QUALITY CLASS: exceeding it degrades, it does not refuse.
    PriceCeiling(u64),
}

impl Constraint {
    pub fn class(self) -> ConstraintClass {
        match self {
            Constraint::LocalOnly | Constraint::NoRetention => ConstraintClass::Data,
            Constraint::PriceCeiling(_) => ConstraintClass::Quality,
        }
    }

    fn satisfied_by(self, candidate: &Candidate) -> bool {
        match self {
            Constraint::LocalOnly => candidate.local,
            Constraint::NoRetention => !candidate.retains,
            Constraint::PriceCeiling(ceiling) => candidate.price <= ceiling,
        }
    }
}

/// ⛔ THE PROOF OF CONFORMANCE (§6.3.1). It cannot be forged: every field is private and the only
/// place that builds one is `resolve`, below. That makes `Q13` a PROPERTY and not a check — an
/// unfiltered candidate is not EXPRESSIBLE as the argument of `dispatch`.
///
/// ⚠️ AND THE LIMIT IS §6.3.2, repeated here because it is the half that gets forgotten: A TOKEN
/// PROVES PROVENANCE, NOT CORRECTNESS. If `resolve` has a defect it mints wrong tokens and the
/// compiler says nothing. It removes ONE class of error — "we forgot to filter" — not two.
///
/// ⛔ IT CARRIES THE WHOLE RESOLVED DECISION AND HAS NO GETTER FOR IT, which is deliberate: the
/// only consumer is `dispatch`, in this module, and a public getter would exist for nobody
/// (the "no caller, no item" rule `ProcessError` already carries).
///
/// ⛔ AND IT DERIVES `Debug` AND NOTHING ELSE, WHICH IS LOAD-BEARING TWICE OVER. `PartialEq`
/// would exist for the bench alone — the same item the rule above keeps off this type — so every
/// probe compares with `matches!`, exactly as `Admission` made every arbiter probe do (errata
/// `E58`). And the absence of `Copy` and `Clone` is what makes a token spendable ONCE.
#[derive(Debug)]
pub struct Conforming {
    model: &'static str,
    evaluated: u32,
    degraded: bool,
}

impl Conforming {
    /// ⛔ DEGRADATION IS DECLARED, NEVER SILENT (ADR-0012, ADR-0019). This is the one thing a
    /// caller can ask, and it exists because a caller that cannot tell would have no way to say
    /// so to the user — which is the whole of "si dichiara prima, non si fallisce dopo".
    pub fn was_degraded(&self) -> bool {
        self.degraded
    }
}

/// What can go wrong. ⛔ DELIBERATELY POOR, like `JournalError`: one variant, and it means the
/// one thing that has no road onwards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GatewayError {
    /// ⛔ FAIL CLOSED (ADR-0012): no candidate satisfies the DATA constraints. There is no
    /// degraded road for this class, and its absence is the guarantee.
    NoConformingCandidate,
}

/// Walks the chain and mints the proof for the first candidate that conforms.
///
/// ⛔ THE TWO CLASSES ARE READ IN A FIXED ORDER, AND THE ORDER IS THE DECISION: the data
/// constraints are a filter — a candidate that fails one is not in the running at all — and the
/// quality constraints are a PREFERENCE among those that are left. Reading them the other way
/// round would let a price ceiling keep a request on a machine its data was not allowed to
/// leave, which is exactly the silent failure ADR-0012 sorts the classes to prevent.
pub fn resolve(
    chain: &[Candidate],
    constraints: &[Constraint],
) -> Result<Conforming, GatewayError> {
    // ⛔ HOW MANY THE CHAIN OFFERED, AND NOT HOW MANY WERE WALKED. The reasoning is on
    // `RoutingDetail::evaluated`, where the field lives and where the wire meaning belongs; the
    // one line that belongs HERE is that this function may traverse the chain TWICE, so a count
    // following the traversal would depend on which of the two roads was taken. Errata `E59`.
    let evaluated = chain.len() as u32;

    let admissible = |candidate: &Candidate| {
        constraints
            .iter()
            .filter(|c| c.class() == ConstraintClass::Data)
            .all(|c| c.satisfied_by(candidate))
    };

    let preferred = |candidate: &Candidate| {
        constraints
            .iter()
            .filter(|c| c.class() == ConstraintClass::Quality)
            .all(|c| c.satisfied_by(candidate))
    };

    // First choice: everything satisfied, nothing degraded.
    if let Some(candidate) = chain.iter().find(|c| admissible(c) && preferred(c)) {
        return Ok(Conforming {
            model: candidate.model,
            evaluated,
            degraded: false,
        });
    }

    // Second choice: the data constraints hold and a quality one does not. ⛔ THIS ROAD PROCEEDS,
    // and `degraded` is how it says so.
    if let Some(candidate) = chain.iter().find(|c| admissible(c)) {
        return Ok(Conforming {
            model: candidate.model,
            evaluated,
            degraded: true,
        });
    }

    // ⛔ AND THERE IS NO THIRD ROAD. An empty chain arrives here too, and it is the same answer
    // for the same reason: nothing conforms.
    Err(GatewayError::NoConformingCandidate)
}

/// Writes the RESOLVED routing record upon the step (ADR-0011) — and it CONSUMES the proof, so
/// one resolution dispatches once. Same shape as `Process::start` consuming a `Grant`.
///
/// ⚠️ AND THAT SENTENCE IS HELD BY THE TYPE AND BY NO CASE, which is said rather than left to be
/// read as covered: `Conforming` derives neither `Copy` nor `Clone`, so a second `dispatch` with
/// the same token IS `error[E0382]` — but no `compile_fail` case says so. The twin question for
/// `Grant` — "a second `start` with the same grant" — was MEASURED AND NOT TAKEN in milestone 5
/// for the reason that decides this one too: a new catalogue row is §7.4, which is spec, and
/// spec is the owner's (global constraint 7). Registered here beside the code, which is the one
/// place a reader of this promise will look. Errata `E62`.
///
/// ⛔ THE STEP IS THE CALLER'S AND THIS FUNCTION OPENS NOTHING. `Journal::note` refuses a note
/// upon a step with no intent — `JournalError::OutOfOrder` — so a routing record is an
/// annotation upon a step that already EXISTS. The gateway has no allocator for a `StepId` and
/// would not be the one to mint an intent it does not own.
///
/// ⛔ WHAT IS NOT HERE, AND IT IS STAGED RATHER THAN MISSING: the call to a provider. The
/// adapters are rule C of §0.4 — there is no provider to call — and the trigger is written here
/// rather than in prose elsewhere: THE FIRST PROVIDER ADAPTER. ⚠️ A deadline written in prose
/// has nothing that makes it fire (gotcha #77), so this one is not a promise: what this function
/// does today is the whole of what it claims to do.
pub fn dispatch<J: Journal>(
    token: Conforming,
    step: StepId,
    journal: &mut J,
) -> Result<(), JournalError> {
    // ⛔ THE RECORD CAN ONLY BE BUILT FROM THE TOKEN, and that is the point: what gets journalled
    // cannot disagree with what was filtered, because there is nothing else to build it from.
    //
    // ⛔ AND IT GOES THROUGH THE SPECIES CONSTRUCTOR, NOT A STRUCT LITERAL: `RecordV1` has no
    // public field since AUD-050, and the privacy is the MODULE's — measured, a literal here is
    // `error[E0451]` even though `gateway` and `record` are siblings inside one crate. The
    // `reason` is therefore a `&'static str` chosen at authoring time and never runtime text,
    // which is the half of AUD-050 that matters: the literal was never the defect, the signature
    // that would accept anything else was.
    //
    // ⚠️ `Trust::Instruction` WITH AN EMPTY PAYLOAD, and the precedent is `Arbiter::set_policy`,
    // which does exactly this: no external byte enters this record, so the label is TRUE rather
    // than decorative. The model name comes from the chain we were handed, which is ours.
    //
    // ⚠️ `EffectClass::Idempotent` IS INERT HERE, and it is said rather than left to be deduced:
    // `crate::reconcile` gives `RecordKind::Routing` an empty arm and provably never reads the
    // field. It is filled with what is TRUE of this record — writing the same routing decision
    // again produces the same record — and not with a class this crate consults.
    let record = Record::V1(RecordV1::routing(
        EffectClass::Idempotent,
        Trust::Instruction,
        Vec::new(),
        "the gateway resolved the routing for this step",
        RoutingDetail::new(token.model, token.evaluated, token.degraded),
    ))
    .encode();

    journal.note(step, &record)
}
