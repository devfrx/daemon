//! The in-memory journal (§4.1). One of the two implementations the conformance suite runs
//! against; the other is `redb` in `platform`.
//!
//! ⛔ THIS IS NOT THE FALLING DOUBLE. Failing at a write chosen by the seed is FAULT
//! INJECTION — §3.3, milestone 4 — and it needs the campaign to be worth anything. Here a
//! journal that works; there one that breaks.

use alloc::vec::Vec;
use kernel::ports::journal::{Journal, JournalError, StepId};

/// A journal that keeps everything in memory, in write order.
///
/// ⚠️ A `Vec` of pairs and not a map, and the reason is a rule of this crate: `HashMap` is
/// forbidden in a deterministic world because `RandomState` is seeded per process and the
/// iteration order is not reproducible (gotcha #12). A `Vec` also gives WRITE ORDER for free.
///
/// ⚠️ AND SINCE 2026-08-10 SOMETHING DOES ASK FOR THAT ORDER: `replay` is on the port, and
/// what it owes is the order ACROSS steps. This sentence used to say the opposite — "nothing
/// asks for that order today" — and it is dated rather than deleted, because the reason the
/// `Vec` was chosen has not changed and the reader deserves to see when it stopped being free.
/// The order is now held from outside by `crates/kernel/tests/journal_contract.rs`;
/// `memory_journal.rs` continues to hold the order WITHIN a step, which conformance does not
/// reach.
pub struct MemoryJournal {
    entries: Vec<Entry>,
}

struct Entry {
    step: StepId,
    kind: EntryKind,
    bytes: Vec<u8>,
}

/// ⚠️ THE JOURNAL'S OWN BOOKKEEPING, and NOT the record's `RecordKind` — the two are separate
/// on purpose and the separation is the answer to the question `crate`'s reconciliation had
/// open until 2026-08-10. This one exists so `has_intent` can be asked; the record's `kind` is
/// what the KERNEL reads, and the kernel is the authority. They are kept in step by the caller,
/// not by a type.
///
/// ⚠️ `Note` IS HERE RATHER THAN FILED UNDER ONE OF THE OTHER TWO, AND NOTHING HOLDS IT — which
/// was MEASURED and not supposed. Filing a note under `EntryKind::Intent` leaves the ENTIRE
/// workspace green, and the reason is sound rather than a hole in the bench: `note` refuses a
/// step with no intent, so a note-only step cannot be built through the port at all, and
/// `has_intent` can only be made true by a note on a step that already had one. The distinction
/// is real inside this file and invisible outside it.
///
/// ⚠️ SO WHY IT IS HERE, since no test would notice its going: `has_intent` asks a question
/// about `Intent`, and a third state that answers "neither" is the shape that stays right when
/// somebody writes `has_outcome` — which nobody has yet. It is an argument, not a red, and it is
/// labelled as one. A test was written to hold it and REMOVED when the mutation survived; the
/// note on that removal is in `crates/simulator/tests/memory_journal.rs`.
#[derive(PartialEq, Eq)]
enum EntryKind {
    Intent,
    Outcome,
    Note,
}

impl MemoryJournal {
    pub const fn new() -> Self {
        MemoryJournal {
            entries: Vec::new(),
        }
    }

    fn has_intent(&self, step: StepId) -> bool {
        self.entries
            .iter()
            .any(|e| e.step == step && e.kind == EntryKind::Intent)
    }
}

impl Journal for MemoryJournal {
    fn intent(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        // ⛔ THE GUARD ARRIVED ON 2026-08-10, AND THIS COMMENT USED TO SAY THERE WAS NONE. It
        // was an OPEN QUESTION and not a decision: a second intent was accepted in silence and
        // `read_back` answered with the first of the two. The question bound BOTH
        // implementations, so it was settled where it binds both — the conformance suite,
        // promise 6 — and this line only obeys.
        //
        // ⚠️ WHY IT IS NOT MERELY THIS FILE'S BUSINESS: promise 2 already forces an
        // implementation to keep more than one record per step, hence a key finer than the
        // step's identity, and with such a key "the first intent wins" falls out for free. But
        // that is an accord by ACCIDENT OF THE KEY DESIGN. Keyed on the step — the natural
        // choice — the two implementations would diverge with nothing going red.
        if self.has_intent(step) {
            return Err(JournalError::OutOfOrder);
        }
        self.entries.push(Entry {
            step,
            kind: EntryKind::Intent,
            bytes: record.to_vec(),
        });
        Ok(())
    }

    fn outcome(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        if !self.has_intent(step) {
            return Err(JournalError::OutOfOrder);
        }
        self.entries.push(Entry {
            step,
            kind: EntryKind::Outcome,
            bytes: record.to_vec(),
        });
        Ok(())
    }

    fn note(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        // ⛔ THE SAME GUARD AS `outcome` AND FOR THE SAME REASON: a note is an annotation UPON
        // something, and a step nobody opened is not something. What it does NOT share is
        // `intent`'s guard — there is no limit on how many notes a step carries, because
        // nothing says how many times one interaction with the world may consult external
        // content. The argument is written out on `Journal::note`.
        if !self.has_intent(step) {
            return Err(JournalError::OutOfOrder);
        }
        self.entries.push(Entry {
            step,
            kind: EntryKind::Note,
            bytes: record.to_vec(),
        });
        Ok(())
    }

    fn read_back(&self, step: StepId) -> Result<Vec<u8>, JournalError> {
        // ⛔ THE FIRST RECORD OF THE STEP, WHICH IS THE INTENT, AND IT IS A DECISION RATHER
        // THAN A PROPERTY OF `find`. `read_back` exists for reconciliation, which re-reads a
        // step IN DOUBT — one that carries an intent and no outcome — and on such a step the
        // first record and the last are the same record, so the choice only shows itself on a
        // COMPLETE step. There it shows in favour of the intent: the intent is the record that
        // says WHAT THE STEP WAS, and hiding it behind its own outcome would leave a resumed
        // run able to read what happened and no longer able to read what it had set out to do.
        //
        // ⚠️ AND THE SECOND IMPLEMENTATION WILL NOT MEET THIS BY ITSELF: a `redb` table keyed
        // by step identity returns — or worse, keeps — the LAST write. ✅ THE CONFORMANCE
        // SUITE NOW HOLDS BOTH TO THE SAME ANSWER — `crates/kernel/tests/journal_contract.rs`,
        // promise 2, with `LastWriteWinsJournal` as the liar that proves it bites. This
        // paragraph used to end "and it does not exist yet: until it does, this comment is the
        // only thing telling whoever writes the second one", which was true when it was
        // written and false from the commit that wrote the suite. Whoever writes the `redb`
        // one does not need this comment to be told: they get a red.
        //
        // ✅ WRITTEN ON 2026-08-10, AND THE PREDICTION HELD WITHOUT COSTING A RED. The `redb`
        // one keys on a PROGRESSIVE OF THE WRITE and not on the step — which the paragraph
        // above is why — so the intent survives its outcome and comes back first, exactly as
        // here. `crates/platform/src/journal.rs` carries the argument on its table definition.
        self.entries
            .iter()
            .find(|e| e.step == step)
            .map(|e| e.bytes.clone())
            .ok_or(JournalError::Missing)
    }

    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError> {
        // ⛔ WRITE ORDER COMES FROM THE `Vec` AND NOWHERE ELSE, which is the reason the store
        // is a `Vec` of pairs rather than a map — see the type's own doc. Nothing here sorts,
        // groups or deduplicates: an intent and its outcome come back as two entries under the
        // same identity, in the order they were written, and telling them apart is the
        // kernel's job because the port exchanges BYTES (ADR-0036).
        Ok(self
            .entries
            .iter()
            .map(|e| (e.step, e.bytes.clone()))
            .collect())
    }

    fn prune(&mut self, step: StepId) -> Result<(), JournalError> {
        // ⚠️ THIS METHOD REFUSED EVERYTHING UNTIL 2026-08-10, and the paragraph that stood here
        // is replaced rather than deleted because it explained a state that was real: it
        // answered `Missing` for a step that was demonstrably there, which is FALSE read against
        // the port's own words, and it satisfied the conformance suite's promise 7 WITHOUT ever
        // consulting whether the step was in doubt. Promise 7b is what took that away.
        if !self.entries.iter().any(|e| e.step == step) {
            return Err(JournalError::Missing);
        }

        // ⛔ ADR-0018, NOT NEGOTIABLE: a step with an intent and no outcome is IN DOUBT, and
        // pruning it destroys the only trace of something that MAY have happened.
        //
        // ⚠️ "IN DOUBT" HERE IS THE PORT'S NOTION AND NOT §4.3's, and the two must not be
        // confused: this asks which OPERATIONS were called — an `intent` with no `outcome` —
        // while the kernel's asks what the RECORDS say, by decoding them. The port cannot
        // decode anything (ADR-0036), which is exactly why `EntryKind` above exists and why
        // `Journal::replay` can say "the port does not know what in doubt means" without
        // contradicting this line. ⚠️ AND THE NOTE IS NOT AN OUTCOME: a step whose only company
        // for its intent is a note is still in doubt, which is the whole reason this asks for
        // `EntryKind::Outcome` by name instead of counting records.
        let closed = self
            .entries
            .iter()
            .any(|e| e.step == step && e.kind == EntryKind::Outcome);
        if !closed {
            return Err(JournalError::StepInDoubt);
        }

        // ⛔ DECLARED LIMIT, AND IT IS A RULE OF ADR-0018 THAT THIS LINE DOES NOT KEEP: "a
        // payload that is absent and one that was never recorded must not be indistinguishable".
        // Removing the entries makes them exactly that — MEASURED on 2026-08-10, not argued: a
        // pruned step and a step nobody ever wrote both answer `Err(Missing)` to `read_back`,
        // and both are absent from `replay`. The full distinction needs the FINGERPRINT and the
        // SIZE that ADR-0018 asks a pruned record to carry, and a fingerprint needs a hash
        // function, which in the kernel is a NEW ENTRY IN THE LIST OF ADR-0031 — a deliberate
        // act no measurement has prepared. It belongs to the milestone that brings retention
        // (decision D7 of the milestone-3 plan), and it is carried as an OPEN ENTRY in
        // `docs/porta-di-qualita.md` rather than as this comment alone, because a note is read
        // and forgotten (gotcha #36).
        self.entries.retain(|e| e.step != step);
        Ok(())
    }
}
