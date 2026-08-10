//! The real `journal` (§4.1): `redb` (ADR-0032), driven through a `StorageBackend` OF OUR OWN.
//!
//! The division of labour is the one `reactor.rs` already follows: the kernel owns the DECISION
//! of what to make durable and when, this file owns the bytes reaching the disk. Nothing here
//! decides anything — which is why the same conformance suite runs against the in-memory double.
//!
//! ⛔ THE BACKEND IS NOT A DETAIL OF `open`, and it is the reason this file is longer than a
//! wrapper would be. `FileBackend` is a TYPE, with `read`, `write`, `set_len` and `sync_data` on
//! it, because it is the boundary at which milestone 4 injects level-2 faults (§4.6, ADR-0032
//! requirement 4) — and a boundary that does not exist as a type cannot be substituted. That it
//! really can is not asserted here but PROVEN FROM OUTSIDE, by
//! `crates/platform/tests/file_journal.rs`, which writes a second implementation of its own and
//! runs `FileJournal` on it. A boundary declared in advance has no callers by construction
//! (gotcha #46), and the only thing that answers the doubt is a caller written from outside.
//!
//! ⚠️ THE TRAIT IS `redb`'s AND WE DO NOT MIRROR IT. A parallel trait of our own, with an
//! adapter onto this one, would buy nothing today: `redb::StorageBackend` already names exactly
//! the four operations §4.6 wants injectable, and the cost of the indirection would be paid on
//! every read. If a second engine ever arrives, the trait arrives with it.
//!
//! ⛔ WHAT BOTH IMPLEMENTATIONS PROMISE IS NOT WRITTEN HERE: it is in
//! `crates/kernel/tests/journal_contract.rs`, eight promises run against this type at task 9.
//! What only THIS one promises — that a write survives the process — is in
//! `crates/platform/tests/file_journal.rs`.

use std::fs::{File, TryLockError};
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::{Mutex, MutexGuard};

use kernel::ports::journal::{Journal, JournalError, StepId};
use redb::{Database, ReadableDatabase, ReadableTable, StorageBackend, TableDefinition};

/// The one table: the archive of every record ever written, in write order.
///
/// ⛔ THE KEY IS A PROGRESSIVE OF THE WRITE AND NOT THE STEP, AND THAT IS THE WHOLE DESIGN.
/// `redb` is a B-tree ORDERED BY KEY, so whatever the key is, that is the order `replay` comes
/// back in — and promise 4 of the conformance suite demands WRITE ORDER, which is a different
/// thing the moment two steps interleave. Keyed on the step, `replay` would answer sorted by
/// identity and the suite would say so; and worse, a step could hold only ONE record, so its
/// outcome would OVERWRITE its intent and promise 2 would fall with it.
///
/// ⚠️ `u64` IS COMPARED AS A NUMBER AND NOT AS BYTES — read in `redb`'s source rather than
/// assumed, because a big-endian byte comparison and a numeric one agree while a little-endian
/// one does not: `Key for u64` is `from_bytes(a).cmp(&from_bytes(b))`, so key 256 follows key
/// 255 as it must. A store that compared the raw little-endian bytes would hand `replay` back
/// scrambled after the 256th write, which no short test would ever reach.
///
/// The value carries the step's identity beside the bytes, because `replay` owes both and the
/// key no longer says which step a record belongs to.
const RECORDS: TableDefinition<u64, (u64, &[u8])> = TableDefinition::new("journal-records");

/// What can go wrong OPENING the journal.
///
/// ⛔ IT IS DELIBERATELY NOT A `JournalError`, and the decision is worth its lines because the
/// plan asked for the opposite. `JournalError` has three variants — `NotDurable`, `Missing`,
/// `OutOfOrder` — and NONE of them means "the file would not open": `NotDurable` says a write
/// did not land when nothing has been written yet, and `Missing` asserts an ABSENCE that a
/// failure to open cannot know about. Either mapping would put a false sentence in the one
/// place whose job is to be true.
///
/// ⛔ AND `open` IS NOT AN OPERATION OF THE PORT, which is what makes an error of its own
/// legitimate rather than an enrichment by stealth. `JournalError` is poor ON PURPOSE — "a rich
/// error type invites the kernel to branch on the reason" — and the kernel never opens
/// anything: the composition root does, once, and there a wrong path and a locked file are
/// exactly the two things a human has to be able to tell apart.
///
/// ⚠️ NO `Display` AND NO `std::error::Error`: nothing implements or consumes them here, and
/// this repository removes items with no callers rather than keeping them for symmetry — the
/// same rule that took `Default` off `SystemReactor`. They come back with the caller.
#[derive(Debug)]
pub enum OpenError {
    /// The file could not be opened or created.
    File(io::Error),
    /// Another journal already holds this file. ⛔ The refusal is deliberate and it is NOT how
    /// `redb`'s own backend behaves on a platform without locks, where it warns and carries on:
    /// two writers on one write-ahead journal is the corruption ADR-0007 exists to prevent, so
    /// this one stops instead of guessing.
    AlreadyOpen,
    /// The file opened and the engine refused it: not a journal, a version this build cannot
    /// read, or an I/O failure on the way to the tree.
    Engine(redb::Error),
}

impl From<io::Error> for OpenError {
    fn from(error: io::Error) -> Self {
        OpenError::File(error)
    }
}

/// Every `redb` error type converts into `redb::Error`, so the five that `open` can meet are
/// folded into one variant here instead of five.
fn engine(error: impl Into<redb::Error>) -> OpenError {
    OpenError::Engine(error.into())
}

/// The journal's file, and THE BOUNDARY AT WHICH MILESTONE 4 WILL INJECT FAILURES (§4.6,
/// ADR-0032 requirement 4). Every byte the engine reads or writes passes through the five
/// methods below; there is no other route to the disk.
///
/// ⚠️ `Mutex<File>` AND NOT POSITIONAL I/O, and the trade is declared rather than defended.
/// `StorageBackend` hands out `&self`, while `Seek`/`Read`/`Write` want `&mut File`, so
/// something has to reconcile them. The alternative is `FileExt::read_at`, which needs one
/// `#[cfg]` branch for Unix and another for Windows — and on this machine ONE OF THE TWO IS
/// NEVER COMPILED, which is a half of the file nothing checks. One portable path that every
/// platform exercises is worth more than a lock elided on the way to a spinning disk. `redb`'s
/// own fallback backend is written the same way, for the same reason.
#[derive(Debug)]
pub struct FileBackend {
    file: Mutex<File>,
}

impl FileBackend {
    /// Opens the file, creating it if it is not there, and TAKES AN EXCLUSIVE LOCK on it.
    ///
    /// ⛔ THE LOCK IS NOT AN EXTRA: `redb`'s own file backend takes one, so a replacement that
    /// did not would drop a guarantee IN SILENCE, which is the one way a substitution must not
    /// go. Both directions are held by `crates/platform/tests/file_journal.rs` — a second
    /// journal on an open file is refused, and one on a CLOSED file is not.
    ///
    /// ⚠️ `truncate(false)` IS THE DEFAULT WRITTEN OUT, and this line SAID THE OPPOSITE until it
    /// was measured — "load-bearing: with `create(true)` alone an existing journal would be
    /// EMPTIED". It would not: a probe that wrote sixteen bytes and reopened both ways read
    /// sixteen bytes back both times. It stays spelt out for decision D3's reason — a default
    /// that matters is written down — and the false sentence is replaced rather than deleted,
    /// because the claim was plausible and the next reader will think it too. `redb`'s own
    /// `Builder::create` spells it out the same way.
    pub fn open(path: &Path) -> Result<Self, OpenError> {
        let file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;

        match file.try_lock() {
            Ok(()) => Ok(FileBackend {
                file: Mutex::new(file),
            }),
            Err(TryLockError::WouldBlock) => Err(OpenError::AlreadyOpen),
            Err(TryLockError::Error(error)) => Err(OpenError::File(error)),
        }
    }

    /// ⚠️ A POISONED LOCK IS REPORTED AS AN I/O ERROR AND NOT PANICKED ON, which is the
    /// opposite of what `redb`'s fallback backend does with the same `Mutex`. A poisoned lock
    /// means another thread died holding the file mid-write; taking the whole process down on
    /// top of that loses the chance to fail the transaction and leave the archive consistent.
    /// ⚠️ DECLARED: no test reaches this branch — reaching it means arranging a panic inside an
    /// I/O call — and it is written this way because the alternative is worse, not because it
    /// is measured.
    fn locked(&self) -> Result<MutexGuard<'_, File>, io::Error> {
        self.file
            .lock()
            .map_err(|_| io::Error::other("the journal's file lock was poisoned by a panic"))
    }
}

impl StorageBackend for FileBackend {
    fn len(&self) -> Result<u64, io::Error> {
        Ok(self.locked()?.metadata()?.len())
    }

    fn read(&self, offset: u64, out: &mut [u8]) -> Result<(), io::Error> {
        let mut file = self.locked()?;
        file.seek(SeekFrom::Start(offset))?;
        file.read_exact(out)
    }

    fn set_len(&self, len: u64) -> Result<(), io::Error> {
        self.locked()?.set_len(len)
    }

    fn sync_data(&self) -> Result<(), io::Error> {
        self.locked()?.sync_data()
    }

    fn write(&self, offset: u64, data: &[u8]) -> Result<(), io::Error> {
        let mut file = self.locked()?;
        file.seek(SeekFrom::Start(offset))?;
        file.write_all(data)
    }
}

/// The `journal` port against a real file.
pub struct FileJournal {
    database: Database,
    /// The key the NEXT write will take. Resumed from the archive on open, never from zero —
    /// see `with_backend`.
    next_key: u64,
}

impl FileJournal {
    /// Opens the journal at `path`, creating it if it is not there.
    pub fn open(path: &Path) -> Result<Self, OpenError> {
        Self::with_backend(FileBackend::open(path)?)
    }

    /// Opens the journal over an ARBITRARY backend.
    ///
    /// ⛔ THIS IS THE ENTRY POINT THAT MAKES THE BOUNDARY REAL, and without it `FileBackend`
    /// would be a type nobody could replace — a boundary in name. Milestone 4 hands a failing
    /// backend in here; `the_storage_backend_is_substitutable_from_outside` hands in a counting
    /// one today, so that the day the failing one arrives it is not also the day this signature
    /// is discovered to be missing.
    pub fn with_backend(backend: impl StorageBackend) -> Result<Self, OpenError> {
        let database = Database::builder()
            .create_with_backend(backend)
            .map_err(engine)?;

        // ⛔ THE TABLE IS CREATED HERE, ON EVERY OPEN, so that every later READ finds it. A
        // `redb` table springs into existence when a WRITE transaction opens it; a read
        // transaction on a fresh file would answer `TableDoesNotExist`, and the port would have
        // to launder that into `Missing` — reporting an absent step where the truth is an
        // absent table. ⚠️ DECLARED COST: opening the journal always commits, so `open` writes
        // to the disk even when nothing is journalled afterwards.
        let transaction = database.begin_write().map_err(engine)?;
        let next_key = {
            let table = transaction.open_table(RECORDS).map_err(engine)?;
            // ⛔ THE COUNTER RESUMES FROM THE ARCHIVE. Starting from zero would make the second
            // session OVERWRITE the first one's records, key by key, in silence — the writes
            // succeed and the old records simply stop existing. Held by
            // `writes_after_reopening_are_appended_and_not_overwritten`.
            //
            // ⚠️ `+ 1` cannot wrap for the same reason `SystemReactor::now`'s cast cannot: it
            // needs 2^64 records to have been written, one transaction each.
            match table.last().map_err(engine)? {
                Some((key, _)) => key.value() + 1,
                None => 0,
            }
        };
        transaction.commit().map_err(engine)?;

        Ok(FileJournal { database, next_key })
    }

    /// Writes a record into a transaction and lets it fall WITHOUT confirming it.
    ///
    /// ⛔ THIS EXISTS FOR A TEST, AND IT IS PUBLIC SURFACE IN PRODUCTION CODE — said here
    /// rather than found out by whoever reads the type's methods looking for the port. It is
    /// the only way to prove requirement 1 of §10.6 — that an unconfirmed transaction leaves
    /// nothing behind — without killing the process, which a test cannot do to itself.
    ///
    /// ⛔ `Ok(())` MEANS THE RECORD REALLY REACHED THE TRANSACTION, AND THE METHOD CHECKS IT
    /// ITSELF — which it did not until a mutation said so. The first version simply staged and
    /// dropped, and the test that uses it leant on the `Ok` for its non-vacuity; a mutation that
    /// STAGED NOTHING and still answered `Ok(())` left all six tests GREEN. The test cannot see
    /// the difference from outside — an abandoned transaction is invisible by construction — so
    /// the check has to live in here, where the transaction is still open. Gotcha #30, measured.
    ///
    /// ⚠️ AND IT DOES NOT ADVANCE `next_key`, because nothing durable happened: the abandoned
    /// key is handed to the next real write.
    pub fn abandon_without_commit(
        &mut self,
        step: StepId,
        record: &[u8],
    ) -> Result<(), JournalError> {
        let transaction = self
            .database
            .begin_write()
            .map_err(|_| JournalError::NotDurable)?;
        Self::stage(&transaction, self.next_key, step, record)?;

        // ⛔ READ IT BACK INSIDE THE TRANSACTION THAT IS ABOUT TO DIE. This is the only vantage
        // point from which "staged, then abandoned" and "never staged" look different: from
        // outside, both leave the archive exactly as it was, which is what the method promises.
        // ⚠️ Opening the table a second time is legitimate only because `stage` dropped its own
        // handle before returning — two live handles on one table in one transaction is
        // `TableAlreadyOpen`.
        let staged = {
            let table = transaction
                .open_table(RECORDS)
                .map_err(|_| JournalError::NotDurable)?;
            table
                .get(self.next_key)
                .map_err(|_| JournalError::NotDurable)?
                .is_some()
        };
        if !staged {
            return Err(JournalError::NotDurable);
        }

        drop(transaction);
        Ok(())
    }

    /// Puts one record into an OPEN transaction. Whether it becomes durable is the caller's
    /// business — which is the whole difference between `append` and `abandon_without_commit`.
    fn stage(
        transaction: &redb::WriteTransaction,
        key: u64,
        step: StepId,
        record: &[u8],
    ) -> Result<(), JournalError> {
        let mut table = transaction
            .open_table(RECORDS)
            .map_err(|_| JournalError::NotDurable)?;
        table
            .insert(key, (step.get(), record))
            .map_err(|_| JournalError::NotDurable)?;
        Ok(())
    }

    /// Appends one record and MAKES IT DURABLE before returning.
    ///
    /// ⚠️ `redb` commits with `Durability::Immediate` unless told otherwise — read in its
    /// source, not assumed — so `commit()` returning IS the durability V6 asks for, and nothing
    /// here calls `set_durability`.
    ///
    /// ⛔ AND NOTHING IN THE BENCH HOLDS THAT, WHICH IS MEASURED AND NOT SUPPOSED: inserting
    /// `set_durability(Durability::None)` right here leaves ALL SIX tests of
    /// `crates/platform/tests/file_journal.rs` green. The reason is structural rather than a
    /// gap in the bench — the tests reopen the file inside a live process, so the writes are in
    /// the operating system's hands either way and the file is closed cleanly. Only a process
    /// that DIES tells the two apart, and killing itself is the one thing a test cannot do.
    /// That is level-2 fault injection, milestone 4, through the very backend below. Declared
    /// here rather than discovered there.
    fn append(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        let transaction = self
            .database
            .begin_write()
            .map_err(|_| JournalError::NotDurable)?;
        Self::stage(&transaction, self.next_key, step, record)?;
        transaction.commit().map_err(|_| JournalError::NotDurable)?;
        // ⛔ AFTER the commit and not before: a write that failed must not burn a key, or the
        // archive would grow holes that `replay` cannot tell from records it never saw.
        self.next_key += 1;
        Ok(())
    }

    /// Walks the archive IN WRITE ORDER and hands back the first thing `visit` recognises.
    ///
    /// ⛔ IT IS A SCAN, AND THE COST IS DECLARED RATHER THAN HIDDEN. Every guard below asks a
    /// question about a step, and a step's records are spread through a table keyed on the
    /// PROGRESSIVE — so answering costs a walk from the beginning. ⚠️ MEASURED, so that the cost
    /// is a number and not a worry: **56 ns per record** in a release build, linear, against a
    /// floor of **1.45 ms** for the `fsync` a write pays anyway — the scan only overtakes it past
    /// some 26 000 records. It is NOT optimised, because no measurement asks for it, and the
    /// remedy the day one does is the same CHECKPOINT that `Journal::replay` already declares it
    /// needs: two mechanisms for one measurement, when the measurement exists. The figures are in
    /// `docs/riferimenti.md`, section "Esecuzione del Traguardo 3 — il Task 8".
    fn find_first<T>(
        &self,
        mut visit: impl FnMut(u64, &[u8]) -> Option<T>,
    ) -> Result<Option<T>, JournalError> {
        // ⛔ `NotDurable` AND NOT `Missing` ON AN ENGINE FAILURE, and the choice is between two
        // wrong-looking answers. `Missing` asserts that the step IS NOT THERE, which a failed
        // read cannot know — and reconciliation would take the lie for an answer and let a step
        // in doubt out of the doubt. `NotDurable` is the port's word for "the durable store did
        // not answer", and that is exactly what happened.
        let transaction = self
            .database
            .begin_read()
            .map_err(|_| JournalError::NotDurable)?;
        let table = transaction
            .open_table(RECORDS)
            .map_err(|_| JournalError::NotDurable)?;

        for entry in table.iter().map_err(|_| JournalError::NotDurable)? {
            let (_, value) = entry.map_err(|_| JournalError::NotDurable)?;
            let (stored_step, record) = value.value();
            if let Some(found) = visit(stored_step, record) {
                return Ok(Some(found));
            }
        }
        Ok(None)
    }

    /// Whether `step` already carries an intent.
    ///
    /// ⛔ IT ASKS "HAS THIS STEP ANY RECORD AT ALL", AND THE TWO QUESTIONS ARE THE SAME ONE
    /// HERE — which is an invariant of this file and not a coincidence, so it is written down.
    /// `intent` refuses a step that already has one, `outcome` and `note` refuse a step that
    /// has none: therefore the FIRST record of any step is always its intent, and no step can
    /// hold records without holding an intent. `MemoryJournal` keeps an explicit kind for the
    /// same question; storing one here would mean reading it back and branching on a byte that
    /// is neither of the three, a case nothing in this file can produce.
    ///
    /// ⚠️ IT READS THE ARCHIVE AND NOT A FIELD, which is what makes the guard survive a
    /// restart. A guard cached in the struct would satisfy promise 6 of the conformance suite —
    /// which never reopens a journal — and let the same step be opened twice across the crash
    /// this whole port exists for. Held by `a_second_intent_is_refused_across_a_reopening`.
    fn has_intent(&self, step: StepId) -> Result<bool, JournalError> {
        Ok(self
            .find_first(|stored, _| (stored == step.get()).then_some(()))?
            .is_some())
    }
}

impl Journal for FileJournal {
    fn intent(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        if self.has_intent(step)? {
            return Err(JournalError::OutOfOrder);
        }
        self.append(step, record)
    }

    fn outcome(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        if !self.has_intent(step)? {
            return Err(JournalError::OutOfOrder);
        }
        self.append(step, record)
    }

    fn note(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        // The same guard as `outcome` and NOT `intent`'s: a note is an annotation upon
        // something, and there is no limit on how many a step carries. The argument is written
        // out on `Journal::note`.
        if !self.has_intent(step)? {
            return Err(JournalError::OutOfOrder);
        }
        self.append(step, record)
    }

    fn read_back(&self, step: StepId) -> Result<Vec<u8>, JournalError> {
        // ⛔ THE FIRST RECORD OF THE STEP, WHICH IS THE INTENT — promise 2 of the conformance
        // suite, and the promise `crates/simulator/src/journal.rs` predicted this
        // implementation would not meet by itself. It does meet it, and not by luck: the key is
        // the progressive of the write, so the intent both SURVIVES its outcome and comes
        // first. A table keyed on the step would have answered with the outcome, or kept only
        // it.
        self.find_first(|stored, record| (stored == step.get()).then(|| record.to_vec()))?
            .ok_or(JournalError::Missing)
    }

    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError> {
        // Write order comes from the KEY and nowhere else — see `RECORDS`. Nothing here groups
        // or deduplicates: an intent and its outcome come back as two entries under the same
        // identity, and telling them apart is the kernel's job because the port exchanges BYTES
        // (ADR-0036).
        let transaction = self
            .database
            .begin_read()
            .map_err(|_| JournalError::NotDurable)?;
        let table = transaction
            .open_table(RECORDS)
            .map_err(|_| JournalError::NotDurable)?;

        let mut all = Vec::new();
        for entry in table.iter().map_err(|_| JournalError::NotDurable)? {
            let (_, value) = entry.map_err(|_| JournalError::NotDurable)?;
            let (stored_step, record) = value.value();
            all.push((StepId::new(stored_step), record.to_vec()));
        }
        Ok(all)
    }

    fn prune(&mut self, _step: StepId) -> Result<(), JournalError> {
        // ⛔ NOT IMPLEMENTED, AND IT ANSWERS THE SAME WAY THE IN-MEMORY DOUBLE DOES, which is
        // the point: retention is out of this milestone (decision D7 — the fingerprint of a
        // pruned payload needs a hash function, and in the kernel that is a NEW ENTRY in the
        // list of ADR-0031), and two implementations that refused DIFFERENTLY would make the
        // conformance suite pass while the two behaved apart. `Missing` for a step that is
        // demonstrably there says something false read on its own; the reason stands here
        // instead of only in the test bench, exactly as it does in `simulator`.
        //
        // ⚠️ It satisfies promise 7 WITHOUT consulting whether the step is in doubt — the same
        // half-blind pass the suite declares on its own assertion. Task 11 is where both sides
        // learn to refuse a step in doubt instead of refusing everything.
        Err(JournalError::Missing)
    }
}
