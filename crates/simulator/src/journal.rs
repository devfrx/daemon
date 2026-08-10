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
/// iteration order is not reproducible (gotcha #12). A `Vec` also gives WRITE ORDER for
/// free, which `replay` owes.
pub struct MemoryJournal {
    entries: Vec<Entry>,
}

struct Entry {
    step: StepId,
    kind: EntryKind,
    bytes: Vec<u8>,
}

#[derive(PartialEq, Eq)]
enum EntryKind {
    Intent,
    Outcome,
}

impl MemoryJournal {
    pub fn new() -> Self {
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
        // by step identity returns — or worse, keeps — the LAST write. The conformance suite
        // is what holds both to the same answer; this comment is what tells whoever writes the
        // second one that the answer was chosen.
        self.entries
            .iter()
            .find(|e| e.step == step)
            .map(|e| e.bytes.clone())
            .ok_or(JournalError::Missing)
    }

    fn prune(&mut self, _step: StepId) -> Result<(), JournalError> {
        Err(JournalError::Missing)
    }
}
