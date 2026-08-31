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
/// ✅ THE QUESTION OF THE TWO TRUTHS IS DECIDED — 2026-08-10, BY THE OWNER — AND THE ANSWER IS
/// THE FIELD. It stood here open: which is the authority on "intent or outcome", the `kind`
/// field of the record or the PORT OPERATION that wrote the entry? `Journal::replay` hands back
/// `(StepId, bytes)` and does not say which of its write methods produced each entry — yet the
/// journal knows: `MemoryJournal` keeps an internal kind per entry, and
/// `JournalError::OutOfOrder` is DEFINED in terms of the operations. Two independent truths
/// about the same thing, and only one of them reaches this function.
///
/// ⛔ THE DECISION: `replay` DOES NOT CHANGE AND `kind` STAYS IN THE RECORD. Telling an intent
/// from an outcome is SEMANTICS OF THE KERNEL, and moving it into the port would contradict
/// `replay`'s own doc — "an operation like `steps_in_doubt()` would move a decision of the
/// kernel inside whoever implements the port" — and the rule that the durable form is the
/// kernel's property (ADR-0036). The record keeps the authority; this walk keeps trusting the
/// field, and now does so BY DECISION rather than by default.
///
/// ⚠️ THE DISAGREEMENT IS CLOSED BY WHOEVER WRITES, AND THERE ARE TWO OF THEM.
/// `Untrusted::promote` writes through `Journal::note` a record whose `kind` is
/// `RecordKind::Note`; `Arbiter::set_policy`, since milestone 5 task 9, writes through
/// `intent` and `outcome` records whose `kind` matches each. Each writer carries its OWN
/// probe that pins the agreement — `the_promotion_writes_through_note_and_the_record_says_note`
/// in `crates/kernel/tests/boundary_promotion.rs`, and
/// `a_policy_transition_writes_its_intent_before_its_outcome` in
/// `crates/kernel/tests/arbiter_policy.rs`, which asserts the two `kind` IN ORDER against the
/// archive.
///
/// ⚠️ RECALL OF 2026-08-21 — THIS PARAGRAPH SAID "TODAY THAT IS ONE FUNCTION" AND CARRIED A
/// TRIGGER THAT HAD ALREADY FIRED: "the helper is born with the SECOND writer". That writer
/// landed on 2026-08-20 and NOTHING WENT RED to say so — a deadline written in prose has no
/// mechanism behind it, unlike the `dead_code` deadlines of `E10` and `E67`, which the compiler
/// remembers. REWRITTEN and not annotated, which is finding A-2's rule.
/// ⛔ WHETHER TO BUILD THE HELPER IS THE OWNER'S and it is REGISTERED, NOT TAKEN: it changes
/// the shape of code with two call sites, and the two probes hold the agreement meanwhile.
///
/// ⚠️ MEASURED, BOTH DIRECTIONS, and the two do not fail alike — kept because it is the evidence
/// that the probe above is worth its line:
///
/// - a record written with `intent()` whose `kind` says `Outcome` → THE STEP IS NOT REPORTED.
///   A true doubt is dropped in silence, which is the one failure ADR-0007 exists to prevent:
///   the doubt is supposed to be always detectable.
/// - a record written with `outcome()` whose `kind` says `Intent` → the step IS reported though
///   it finished; before `enter` below made this a set, it was reported TWICE.
///
/// ⚠️ AND WHAT IS *NOT* BOUGHT IS SAID PLAINLY, because "decided" reads like "held": nothing at
/// level 1 stops a future writer from calling `outcome()` with a record whose `kind` says
/// `Intent`. Each probe covers its own writer. This sentence used to read "it is not
/// a defect today, because nothing in the kernel writes a record yet"; that reason expired on
/// 2026-08-10, and it is replaced rather than left standing.
pub fn steps_in_doubt<J: Journal>(journal: &J) -> Result<Vec<InDoubt>, JournalError> {
    let entries = journal.replay()?;

    let mut open: Vec<InDoubt> = Vec::new();
    for (step, bytes) in entries {
        match Record::decode(&bytes) {
            Ok(Record::V1(body)) => match body.kind {
                RecordKind::Intent => enter(&mut open, step, resolution_of(body.effect)),
                RecordKind::Outcome => leave(&mut open, step),
                // ⛔ A NOTE NEITHER OPENS A DOUBT NOR CLOSES ONE, AND THE EMPTY ARM IS THE
                // WHOLE OF IT. A note says something happened WITHIN a step — today, that
                // untrusted content crossed the boundary — and says nothing about whether the
                // step's effect reached the world. Both other answers were MEASURED before this
                // one was written, and both are defects:
                //
                // - treated as an intent, `enter` REPLACES the caller's resolution with the
                //   note's: a step the caller declared `Idempotent` came back `SuspendAndAsk`.
                //   The note would silently downgrade a step it does not own.
                // - treated as an outcome, `leave` takes the step out of the doubt although
                //   nothing executed: `steps_in_doubt` answered `[]`. A true doubt vanishing in
                //   silence is the one failure ADR-0007 exists to prevent.
                //
                // ⚠️ SO THE `effect` FIELD OF A NOTE IS NEVER READ, and that is said here rather
                // than left for a reader to deduce from an empty arm. `Untrusted::promote`
                // writes `Unrepeatable` into it, and the reason is on that call — it is what an
                // inert field is filled with here, not a class this function consults.
                //
                // Held in BOTH directions (§7.1.1 rule 3) by
                // `a_note_does_not_put_a_step_in_doubt` and
                // `a_note_leaves_the_doubt_and_its_resolution_exactly_as_it_found_them`.
                RecordKind::Note => {}
                // ⛔ A VERDICT NEITHER OPENS A DOUBT NOR CLOSES ONE, and the empty arm was
                // MEASURED for this variant rather than inherited from `Note`'s. The doubt of
                // ADR-0007 is about an EFFECT that may or may not have reached the world; a
                // verdict is a fact recorded ABOUT a step's artefact, and the step it names
                // already owes its own outcome. Both other answers were tried on 2026-09-01,
                // one at a time, each reverted from a byte-exact copy:
                //
                // - `enter` makes a step that already has an intent RE-ENTER the doubt with the
                //   verdict's own class, so a step whose outcome had already closed it comes
                //   back open FOREVER — the ring writes a verdict on every artefact it judges.
                // - `leave` closes a doubt that no effect resolved: the ring's verdict would
                //   take a step OUT of the doubt although nothing executed, which is the silent
                //   loss ADR-0007 exists to prevent.
                //
                // ⚠️ SO THE `effect` FIELD OF A VERDICT IS NEVER READ EITHER, and `run_the_ring`
                // writes `Verifiable` into it with its reason on that call.
                //
                // ⛔ AND WHAT HOLDS THIS ARM IS DECLARED RATHER THAN ASSUMED: see
                // `a_verdict_does_not_put_a_step_in_doubt` and
                // `a_verdict_leaves_the_doubt_and_its_resolution_exactly_as_it_found_them` in `tests/reconciliation.rs`, written
                // in BOTH directions (§7.1.1 rule 3) exactly as `Note`'s pair is.
                RecordKind::Verdict => {}
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
