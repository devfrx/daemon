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

    fn prune(&mut self, _step: StepId) -> Result<(), JournalError> {
        // ⛔ NOT IMPLEMENTED, AND IT ANSWERS `Missing` FOR A STEP THAT IS DEMONSTRABLY THERE.
        // Read on its own against the port's own words — "the read found nothing under that
        // identity" — this line says something FALSE, so the reason stands next to it instead
        // of only in the test bench. Retention is out of this milestone: the fingerprint a
        // pruned record carries demands a hash function, and in the kernel that is a NEW ENTRY
        // IN THE LIST OF ADR-0031 — a deliberate act wanting a measurement nobody has made.
        //
        // ⚠️ Refusing is only half of it: it must also NOT PRUNE. An irreversible operation
        // that half happened and then reported failure is worse than one nobody wrote. Both
        // halves are held by `prune_refuses_and_leaves_the_record_where_it_was`.
        Err(JournalError::Missing)
    }
}
