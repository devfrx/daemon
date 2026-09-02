//! The in-memory journal (§4.1). One of the two implementations the conformance suite runs
//! against; the other is `redb` in `platform`.
//!
//! ⛔ THIS IS NOT THE FALLING DOUBLE. Failing at a write chosen by the seed is FAULT
//! INJECTION — §3.3, milestone 4 — and it needs the campaign to be worth anything. Here a
//! journal that works; there one that breaks.
//!
//! ⚠️ UNTIL 2026-08-11 THE PARAGRAPH ABOVE WAS THE WHOLE TRUTH, and it is dated rather than
//! deleted because it held for as long as the falling double did not exist: "there" was a
//! milestone away, and two documents — the milestone-4 design and `docs/riferimenti.md` —
//! quote that sentence as evidence that it did not. It does now, and "there" is HERE, further
//! down this file: `CrashingJournal`.
//!
//! ⚠️ WHY IN THIS FILE AND NOT IN ONE OF ITS OWN — decision D1. The falling double WRAPS
//! `MemoryJournal` instead of reimplementing it, so what survives a crash is the very archive
//! the working journal keeps. Two in-memory stores would be two truths to hold in step, and
//! nothing would go red on the day they drifted apart.

use alloc::vec::Vec;

use crate::rng::SeededRng;
use kernel::ports::journal::{Journal, JournalError, StepId};
use kernel::rng::RngExt;

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
///
/// ⛔ RECALL OF 2026-09-02: THE TWO CLAIMS ABOVE ARE FALSE AGAINST THE PORT THEY DESCRIBE.
/// `replay` owes the WHOLE write order and not "the order ACROSS steps" -- `ports/journal.rs`
/// says "Re-reads EVERYTHING, in write order" and "WRITE ORDER IS PART OF THE PROMISE" -- and
/// conformance DOES reach inside a step: promise 8(c) asserts `[(step, intent), (step, note)]`,
/// two records of one step in write order. What the suite does not exercise is TWO NOTES upon
/// ONE step: measured, of the four `.note` calls in `assert_journal_contract` two are asserted
/// refused and the two that succeed sit on different steps. That untested case is the one
/// `kernel::degradation::degradation_now` rests on, and it is declared beside its loop.
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
enum EntryKind {
    Intent,
    Outcome,
    Note,
}

impl EntryKind {
    /// ⛔ EXHAUSTIVE `match`ES AND NOT THE `==` THE TWO QUESTIONS CARRIED — the cure of
    /// `kernel::permission::Operation::is_write`, for the same reason. A fourth kind fell out of
    /// both questions as "neither", which here is the conservative direction — it is treated as a
    /// note — but it fell in silence: measured on 2026-09-01, ninth review round, with a variant
    /// added `cargo check --locked --workspace --all-targets` stayed at ZERO errors. Now it is
    /// `error[E0004]` here. Errata `E124`.
    ///
    /// ⚠️ AND THE `Outcome` ARM IS HELD BY NOTHING, for a reason that is a property of this
    /// journal and not a gap: `outcome` refuses a step without an intent (`OutOfOrder`), so no
    /// step ever carries an `Outcome` entry without an `Intent` one, and answering `true` for
    /// `Outcome` changes no answer. Measured on 2026-09-01, tenth review round: the whole
    /// workspace stayed green, `43 · 326 · 0 · 2`. The `Note` arm's equivalence is declared on
    /// the enum above; this one was not, and a reader mutating it would have read a gap
    /// (errata `E135`).
    fn is_intent(&self) -> bool {
        match self {
            EntryKind::Intent => true,
            EntryKind::Outcome | EntryKind::Note => false,
        }
    }

    fn is_outcome(&self) -> bool {
        match self {
            EntryKind::Outcome => true,
            EntryKind::Intent | EntryKind::Note => false,
        }
    }
}

// ⛔ NO `impl Default`, AND ITS ABSENCE IS THE DECISION — the same one, for the same reason, as
// `SystemReactor` and `VirtualReactor`: nothing calls it, and this repository removes such items
// rather than keeping them for symmetry. `cargo clippy` asks for one on all three
// (`new_without_default`); the warning is ACCEPTED and NOT silenced, because §7.4.3 gives clippy
// no voice in the gate and an `#[allow]` would hide the next occurrence too. The argument is
// written out once, in `crates/platform/src/reactor.rs`, and this comment points at it rather
// than restating it. Added 2026-08-21: two of the three sites carried it and this one did not.
impl MemoryJournal {
    pub const fn new() -> Self {
        MemoryJournal {
            entries: Vec::new(),
        }
    }

    fn has_intent(&self, step: StepId) -> bool {
        self.entries
            .iter()
            .any(|e| e.step == step && e.kind.is_intent())
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
        // contradicting this line. ⛔ AND THE TWO CAN DIVERGE IN THE DIRECTION THAT AUTHORISES
        // PRUNING — an outcome whose bytes the kernel cannot decode is closed HERE and in doubt
        // THERE, measured on 2026-08-27 on both implementations. It is a LIMIT and not a nuance,
        // and it is declared once, on `Journal::prune`, together with the obligation it puts on
        // whoever calls. ⚠️ AND THE NOTE IS NOT AN OUTCOME: a step whose only company
        // for its intent is a note is still in doubt, which is the whole reason this asks for
        // `EntryKind::Outcome` by name instead of counting records.
        let closed = self
            .entries
            .iter()
            .any(|e| e.step == step && e.kind.is_outcome());
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

/// A journal that STOPS EXISTING at a write chosen by the seed — §3.3, and level 1 of the two
/// crash levels of ADR-0032.
///
/// ⛔ IT IS NOT AN ERROR CHANNEL, AND THAT DIFFERENCE IS THE WHOLE POINT. A journal that
/// answered `NotDurable` once and worked again afterwards would model A BAD DISK, not a crash:
/// a dead process does not come back. So the first refusal is PERMANENT, and every later write
/// is refused too — which is what makes all the interleaved activities of the campaign stop,
/// and not only the one that happened to touch the boundary.
///
/// ⚠️ DECLARED LIMIT, so this doc promises no more than it delivers, and it is about the three
/// operations the sentence above does NOT cover. `read_back` and `replay` DELEGATE to the
/// surviving archive rather than refusing: the campaign never calls them — it calls
/// `into_survivor`, which models reopening the archive after the restart — and they are here
/// because `Journal` requires them. `prune` is different from both, because it MUTATES: it is
/// refused after the fall, like every write, but it takes no part in the count the crash point
/// is drawn against. Its own reasoning is on the method.
///
/// ⚠️ IT IS NOT HELD TO THE CONFORMANCE SUITE, and that is deliberate rather than an omission:
/// this type is a LIAR by construction, and gotcha #50 says a fake may break a contract when
/// the test around it speaks about the breaking. Its own promises live in
/// `crates/simulator/tests/crashing_journal.rs`.
pub struct CrashingJournal {
    inner: MemoryJournal,
    falls_at: u64,
    writes: u64,
    fallen: bool,
}

impl CrashingJournal {
    /// Falls at the write with this index, counting from zero.
    pub const fn falling_at(write: u64) -> Self {
        CrashingJournal {
            inner: MemoryJournal::new(),
            falls_at: write,
            writes: 0,
            fallen: false,
        }
    }

    /// Falls at a write DRAWN from the seed, inside `0..expected_writes`.
    ///
    /// ⛔ `expected_writes` IS HOW MANY WRITES THE SCENARIO REALLY PERFORMS, counted rather
    /// than guessed. Gotcha #17: a point drawn past the last write never fires, and a campaign
    /// whose fault never arrives reports green for having done nothing.
    ///
    /// ⛔ COUNTED IN WHICH RUN, because the number is not the same in all of them: in a run
    /// WITHOUT A CRASH, which is what `without_crash` is for. A count taken from a run that
    /// already crashed stops at the crash and would draw every later point out of reach.
    ///
    /// ⛔ THE SEED MUST BE DERIVED, AND DIFFERENT FROM THE ONE DRIVING THE INTERLEAVING —
    /// decision D4, and the obvious wiring is the wrong one. Two `SeededRng` built from the
    /// same number give the SAME sequence, so passing the campaign's seed straight through
    /// ties the crash point to the interleaving: the campaign would then explore a DIAGONAL of
    /// the space instead of the space. Nothing on this type can enforce it — the caller holds
    /// the seed — so it is written here, where the caller is looking, rather than left in a
    /// plan nobody rereads (gotcha #36).
    ///
    /// ⛔ AND `expected_writes` MUST NOT BE ZERO. `RngExt::below` answers 0 for a bound of 0,
    /// and 0 is not inside `0..0` — the range is empty — so the point would be one that can
    /// never arrive: exactly the vacuity the paragraph above exists to prevent. A scenario
    /// that performs no writes has no write to fall at, and asking for one is a defect in the
    /// caller rather than a case to serve.
    pub fn from_seed(seed: u64, expected_writes: u64) -> Self {
        debug_assert!(
            expected_writes > 0,
            "a scenario that performs no writes has no write to fall at: `below` would answer \
             0, which is outside the empty range 0..0, and the crash would never fire"
        );
        let mut rng = SeededRng::new(seed);
        Self::falling_at(rng.below(expected_writes))
    }

    /// Never falls. It is what `C7a` — no crash, no false doubt — is measured against.
    pub const fn without_crash() -> Self {
        Self::falling_at(u64::MAX)
    }

    /// The write it will fall at.
    pub const fn falls_at(&self) -> u64 {
        self.falls_at
    }

    /// Whether it HAS fallen. ⛔ The campaign's non-vacuity oracle: without it, "this run left
    /// no doubt" and "the crash never fired" are the same green.
    pub const fn has_fallen(&self) -> bool {
        self.fallen
    }

    /// How many writes reached the archive.
    pub const fn writes_done(&self) -> u64 {
        self.writes
    }

    /// The archive that survived, as a journal that works. It models REOPENING after the
    /// restart, which is the only way the reconciliation ever meets a crashed archive.
    pub fn into_survivor(self) -> MemoryJournal {
        self.inner
    }

    /// Whether this write may proceed, MARKING the fall when it may not.
    ///
    /// ⚠️ It is asked BEFORE delegating, and the counter moves only on an `Ok` from the inner
    /// journal: a write the write-ahead protocol refuses (`OutOfOrder`) never reached storage,
    /// so it must not consume a position in the count the crash point is drawn against. Held
    /// by `a_write_the_protocol_refuses_does_not_consume_a_crash_position`.
    fn may_write(&mut self) -> bool {
        if self.fallen {
            return false;
        }
        if self.writes == self.falls_at {
            self.fallen = true;
            return false;
        }
        true
    }
}

impl Journal for CrashingJournal {
    fn intent(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        if !self.may_write() {
            return Err(JournalError::NotDurable);
        }
        let written = self.inner.intent(step, record);
        if written.is_ok() {
            self.writes += 1;
        }
        written
    }

    fn outcome(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        if !self.may_write() {
            return Err(JournalError::NotDurable);
        }
        let written = self.inner.outcome(step, record);
        if written.is_ok() {
            self.writes += 1;
        }
        written
    }

    fn note(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        if !self.may_write() {
            return Err(JournalError::NotDurable);
        }
        let written = self.inner.note(step, record);
        if written.is_ok() {
            self.writes += 1;
        }
        written
    }

    fn read_back(&self, step: StepId) -> Result<Vec<u8>, JournalError> {
        self.inner.read_back(step)
    }

    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError> {
        self.inner.replay()
    }

    fn prune(&mut self, step: StepId) -> Result<(), JournalError> {
        // ⛔ IT IS THE ONLY MUTATING OPERATION THAT DOES NOT GO THROUGH `may_write`, AND THAT
        // IS DELIBERATE RATHER THAN AN OVERSIGHT. Refused after the fall, yes — a dead process
        // prunes nothing, and an archive pruned after the crash is one no real crash can
        // produce. But it must not ARM or CONSUME the fall: the crash point is drawn against
        // the writes of the scenario, which are `intent`, `outcome` and `note` — see
        // `from_seed`. A prune that moved the counter would shift the fall away from the drawn
        // point, which is gotcha #17 by another route.
        if self.fallen {
            return Err(JournalError::NotDurable);
        }
        self.inner.prune(step)
    }
}
