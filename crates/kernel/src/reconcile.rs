//! Reconciliation (§4.3). ⛔ RESUMING IS RECONCILIATION, NOT BLIND REPLAY (ADR-0007): re-reading
//! the journal does not mean re-running, it means establishing, FOR EVERY STEP IN DOUBT, what
//! happened and what to do about it.

use alloc::vec::Vec;

use crate::ports::journal::{Journal, JournalError, StepId};
use crate::record::{EffectClass, Record, RecordKind};

/// What to do with one step in doubt. The class of the effect decides, and nothing else does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolution {
    /// `EffectClass::Verifiable` — ask the world what happened, then finish or re-plan.
    AskTheWorld,
    /// `EffectClass::Idempotent` — just run it again.
    RunAgain,
    /// ⛔ `EffectClass::Unrepeatable` — suspend and ask the user. ALSO what an undeclared or
    /// unreadable class means: in front of a doubt it cannot resolve, the system stops rather
    /// than guesses.
    SuspendAndAsk,
}

/// One step that has an intent and no outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InDoubt {
    pub step: StepId,
    pub resolution: Resolution,
}

/// Every step with an intent and no outcome, in the order the journal wrote them.
///
/// ⛔ IT RETURNS A SET AND NOT ONE STEP, and that is not defensive style. Measured on the
/// spike: with interleaved execution ONE CRASH LEAVES SEVERAL STEPS IN DOUBT TOGETHER — seed
/// 99 left `[3, 7]`. The spike helper returned one, assuming sequential execution, and gave a
/// FALSE NEGATIVE. Gotcha #20, and constraint 6 of §11 says it does not come up as it was.
///
/// ⛔ DECLARED OPEN QUESTION, RAISED HERE AND NOT DECIDED HERE: WHICH IS THE AUTHORITY ON
/// "INTENT OR OUTCOME" — THE `kind` FIELD OF THE RECORD, OR THE PORT OPERATION THAT WROTE THE
/// ENTRY? This walk trusts the FIELD, and only the field. `Journal::replay` hands back
/// `(StepId, bytes)` and does not say which of its two write methods produced each entry — yet
/// the journal knows: `MemoryJournal` keeps an internal kind per entry, and
/// `JournalError::OutOfOrder` is DEFINED in terms of the two operations. So there are two
/// independent truths about the same thing, and only one of them reaches this function.
///
/// ⚠️ MEASURED AT THIS COMMIT, BOTH DIRECTIONS, and the two do not fail alike:
///
/// - a record written with `intent()` whose `kind` says `Outcome` → THE STEP IS NOT REPORTED.
///   A true doubt is dropped in silence, which is the one failure ADR-0007 exists to prevent:
///   the doubt is supposed to be always detectable.
/// - a record written with `outcome()` whose `kind` says `Intent` → the step IS reported though
///   it finished; before `enter` below made this a set, it was reported TWICE.
///
/// ⚠️ IT IS NOT A DEFECT TODAY, and the reason is what makes it safe to leave open: nothing in
/// the kernel writes a record yet — `Untrusted::promote` gains that at task 7 — so the two
/// truths cannot disagree unless the kernel writes a record that contradicts the call it is
/// making. What is written here is that NOTHING PREVENTS IT and nothing would notice.
///
/// ⛔ AND THE REMEDY IS NOT ON THIS SIDE OF THE PORT. Closing it means `replay` handing back the
/// operation it performed, which touches the port, the conformance suite and both
/// implementations — a shared contract, so it is reported rather than taken while writing a
/// consumer. ⚠️ AND THE CONSEQUENCE THAT COMES WITH IT, written down so it is not discovered
/// afterwards: if the port becomes the authority, the record's `kind` field turns redundant —
/// and `crate::record` calls it the field "the whole write-ahead protocol rests on". Then it
/// either goes, which is a FORMAT change and the frozen bytes of task 10 are the deadline for
/// one, or it stays as a cross-check that something actually CHECKS. Redundancy nobody checks is
/// exactly what ADR-0036 refused when it kept one oracle instead of two.
pub fn steps_in_doubt<J: Journal>(journal: &J) -> Result<Vec<InDoubt>, JournalError> {
    let entries = journal.replay()?;

    let mut open: Vec<InDoubt> = Vec::new();
    for (step, bytes) in entries {
        match Record::decode(&bytes) {
            Ok(Record::V1(body)) => match body.kind {
                RecordKind::Intent => enter(&mut open, step, resolution_of(body.effect)),
                RecordKind::Outcome => leave(&mut open, step),
            },
            // ⛔ A record this build cannot read closes nothing and resolves nothing: it is the
            // strongest form of "no declared class", and ADR-0007 says that means stop. Note it
            // ENTERS rather than closes: an unreadable record does not say the step finished.
            Err(_) => enter(&mut open, step, Resolution::SuspendAndAsk),
        }
    }

    Ok(open)
}

/// Puts a step in doubt, or replaces the answer of one already in doubt WITHOUT MOVING IT.
///
/// ⛔ THIS IS WHAT MAKES THE RESULT A SET, and it was a real defect rather than a precaution.
/// Measured before it was written: a step carrying an intent this build can read and an outcome
/// it cannot came back as `[{5, RunAgain}, {5, SuspendAndAsk}]` — THE SAME STEP TWICE — and a
/// caller walking that would suspend one step two times. Held by
/// `a_step_is_in_doubt_at_most_once_however_many_records_it_carries`.
///
/// ⚠️ REPLACING IN PLACE IS A CHOICE AND NOT A PROPERTY OF THE `Vec`: a later record that changes
/// a step's resolution does not make the step leave the doubt and re-enter it, so it keeps the
/// position it took when the doubt began, and the order the caller walks stays the order the
/// doubts appeared. Held by `a_step_that_re_enters_doubt_keeps_the_place_it_first_took`.
fn enter(open: &mut Vec<InDoubt>, step: StepId, resolution: Resolution) {
    match open.iter_mut().find(|d| d.step == step) {
        Some(already) => already.resolution = resolution,
        None => open.push(InDoubt { step, resolution }),
    }
}

/// Takes a step out of the doubt. Absent is not an error: see the declared limit above, where a
/// record whose `kind` disagrees with the operation is exactly how a step can be closed here
/// without ever having been opened.
fn leave(open: &mut Vec<InDoubt>, step: StepId) {
    open.retain(|d| d.step != step);
}

fn resolution_of(class: EffectClass) -> Resolution {
    match class {
        EffectClass::Verifiable => Resolution::AskTheWorld,
        EffectClass::Idempotent => Resolution::RunAgain,
        EffectClass::Unrepeatable => Resolution::SuspendAndAsk,
    }
}
