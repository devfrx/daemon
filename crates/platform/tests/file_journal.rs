//! The real journal, checked on WHAT ONLY IT PROMISES. What both implementations promise is
//! the conformance suite's business — `crates/kernel/tests/journal_contract.rs` — which is run
//! against this type at task 9. Asserting the promises below inside that suite would turn the
//! in-memory double red, and the double is CORRECT not to make them: gotcha #44.

use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use kernel::ports::journal::{Journal, JournalError, StepId};
use platform::journal::{FileBackend, FileJournal, OpenError};
use redb::StorageBackend;

/// A directory of this test's own, emptied on entry.
///
/// ⛔ ONE DIRECTORY PER CALL SITE, AND THE LINE NUMBER IS WHAT MAKES IT ONE. The obvious
/// arrangement — one shared directory emptied on entry, with a different FILE NAME per test —
/// cannot work, and the file names are not the problem: `cargo test` runs these on parallel
/// threads, so a test entering while another is mid-flight deletes the other one's file too.
/// What comes of it is an INTERMITTENT red, which is worse than a red.
///
/// ⚠️ AND `line!()` RATHER THAN A NAME WRITTEN BY HAND, which is the difference between a
/// property and a discipline: two call sites cannot share a line number, so the directories
/// are distinct BY CONSTRUCTION and nobody has to remember to keep the names apart.
///
/// ⚠️ Deliberately not a crate: `tempfile` would be a new entry for eight lines, and
/// `platform` already names `std`.
///
/// ⚠️ AND IT IS DEFINED UP HERE, NOT NEXT TO THE OTHER HELPERS AT THE BOTTOM: a `macro_rules!`
/// is visible only AFTER its definition in the file, so a macro written below its callers does
/// not compile.
macro_rules! private_dir {
    () => {
        private_dir_for_line(line!())
    };
}

fn private_dir_for_line(line: u32) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("daemon-file-journal-{line}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create the test directory");
    dir
}

#[test]
fn what_was_written_survives_reopening_the_file() {
    // ⛔ THE PROMISE THE IN-MEMORY DOUBLE CANNOT MAKE, and the reason this test is here and not
    // in the conformance suite.
    let dir = private_dir!();
    let path = dir.join("journal.redb");

    {
        let mut journal = FileJournal::open(&path).expect("open");
        journal.intent(StepId::new(1), b"durable").expect("intent");
    }

    let reopened = FileJournal::open(&path).expect("reopen");
    assert_eq!(
        reopened.read_back(StepId::new(1)).expect("read back"),
        b"durable".to_vec()
    );
}

#[test]
fn an_unconfirmed_transaction_leaves_nothing_behind() {
    // Requirement 1 of §10.6: after reopening, the CONFIRMED records are there and the one from
    // a transaction never committed is NOT.
    let dir = private_dir!();
    let path = dir.join("journal.redb");

    {
        let mut journal = FileJournal::open(&path).expect("open");
        journal
            .intent(StepId::new(1), b"committed")
            .expect("intent");
        // ⛔ THE `expect` IS WHAT KEEPS THIS TEST FROM PASSING VACUOUSLY — and it only became
        // true after a mutation said it was not. This comment used to stop at that sentence
        // while the method merely staged and dropped, so a version that STAGED NOTHING answered
        // `Ok(())` and this test stayed green: the assertion below cannot tell an abandoned
        // record from one that never existed, which is precisely what the method promises about
        // it. The check now lives inside `abandon_without_commit`, where the transaction is
        // still open, and the `expect` is what carries it out here. Gotcha #30.
        journal
            .abandon_without_commit(StepId::new(2), b"lost")
            .expect("the record must reach the transaction, or this test measures nothing");
    }

    let reopened = FileJournal::open(&path).expect("reopen");
    assert!(reopened.read_back(StepId::new(1)).is_ok());
    assert_eq!(
        reopened.read_back(StepId::new(2)),
        Err(JournalError::Missing)
    );
}

#[test]
fn writes_after_reopening_are_appended_and_not_overwritten() {
    // ⛔ THE KEY IS A PROGRESSIVE OF THE WRITE AND NOT THE STEP, so a journal that reopened
    // with its counter back at zero would OVERWRITE the records of the previous session — in
    // silence, because the second write succeeds and the first record simply stops existing.
    // ⚠️ NEITHER OF THE TWO TESTS ABOVE SEES IT: both write during one session only, and the
    // conformance suite never reopens anything. It is this implementation's own promise.
    let dir = private_dir!();
    let path = dir.join("journal.redb");

    {
        let mut journal = FileJournal::open(&path).expect("open");
        journal.intent(StepId::new(1), b"before").expect("intent 1");
    }

    let mut reopened = FileJournal::open(&path).expect("reopen");
    reopened
        .intent(StepId::new(2), b"after the restart")
        .expect("intent 2");

    // ⛔ THE RECORDS AND NOT THE IDENTITIES, for the reason promise 4 of the conformance suite
    // learned the hard way: comparing identities alone leaves the block blind to a store that
    // kept the right names and the wrong bytes.
    let replayed = reopened.replay().expect("replay");
    let records: Vec<(StepId, &[u8])> = replayed
        .iter()
        .map(|(step, bytes)| (*step, bytes.as_slice()))
        .collect();
    assert_eq!(
        records,
        vec![
            (StepId::new(1), b"before".as_slice()),
            (StepId::new(2), b"after the restart".as_slice()),
        ]
    );
}

#[test]
fn a_second_intent_is_refused_across_a_reopening() {
    // ⛔ THE GUARD READS THE ARCHIVE AND NOT A MEMORY OF THIS SESSION. Promise 6 of the
    // conformance suite builds a fresh journal every time and never reopens one, so a guard
    // kept in a field of the struct would satisfy it and let the SAME step be opened twice
    // across a restart — which is precisely the moment a write-ahead journal is consulted.
    let dir = private_dir!();
    let path = dir.join("journal.redb");

    {
        let mut journal = FileJournal::open(&path).expect("open");
        journal
            .intent(StepId::new(1), b"the first intent")
            .expect("intent");
    }

    let mut reopened = FileJournal::open(&path).expect("reopen");
    assert_eq!(
        reopened.intent(StepId::new(1), b"the second intent"),
        Err(JournalError::OutOfOrder)
    );
}

#[test]
fn the_storage_backend_is_substitutable_from_outside() {
    // ⛔ THE ONLY REASON `FileBackend` IS A TYPE OF ITS OWN, and a boundary declared in advance
    // HAS NO CALLERS BY CONSTRUCTION — gotcha #46. What proves it real is an implementation
    // written from OUTSIDE the crate, which is what this test is: `CountingBackend` lives here,
    // in a test binary, and `FileJournal` runs on it unchanged. Milestone 4 will put a FAILING
    // one in the same place (§4.6, ADR-0032 requirement 4); this one only counts.
    let dir = private_dir!();
    let path = dir.join("journal.redb");

    let writes = Arc::new(AtomicU64::new(0));
    let syncs = Arc::new(AtomicU64::new(0));
    let backend = CountingBackend {
        inner: FileBackend::open(&path).expect("the real backend"),
        writes: Arc::clone(&writes),
        syncs: Arc::clone(&syncs),
    };

    let mut journal = FileJournal::with_backend(backend).expect("open over a foreign backend");
    journal
        .intent(StepId::new(1), b"what it set out to do")
        .expect("intent");
    journal
        .outcome(StepId::new(1), b"what came of it")
        .expect("outcome");

    // The journal behaves exactly as it does on its own backend.
    assert_eq!(
        journal.read_back(StepId::new(1)).expect("read back"),
        b"what it set out to do".to_vec()
    );

    // ⛔ AND THE I/O REALLY WENT THROUGH IT. Without these two lines the test would stay green
    // against a `FileJournal` that accepted the backend and then wrote somewhere else — which
    // is the shape milestone 4's injection would fail silently against.
    assert!(
        writes.load(Ordering::Relaxed) > 0,
        "no write reached the backend"
    );
    assert!(
        syncs.load(Ordering::Relaxed) > 0,
        "no sync reached the backend"
    );
}

#[test]
fn a_second_journal_on_the_same_file_is_refused_while_the_first_is_open() {
    // ⛔ THE GUARANTEE OUR BACKEND WOULD OTHERWISE HAVE LOST. `redb`'s own file backend takes
    // an exclusive lock, and replacing it with one that does not would drop that protection in
    // silence — two writers on one write-ahead journal is the corruption ADR-0007 exists to
    // make impossible.
    //
    // ⚠️ THE OTHER DIRECTION IS HELD BY `what_was_written_survives_reopening_the_file`
    // (§7.1.1 rule 3): once the first journal is dropped the lock goes with it, and reopening
    // the same path SUCCEEDS. A lock that never released would pass this test and fail that one.
    let dir = private_dir!();
    let path = dir.join("journal.redb");

    let first = FileJournal::open(&path).expect("open");

    // ⚠️ A `match` AND NOT `assert!(matches!(..))`, so that the two wrong outcomes are told
    // apart: a journal that opened anyway, and one that refused for another reason entirely.
    // It also keeps `Debug` off `FileJournal`, which is not tidiness — a `Debug` on a type
    // holding an archive is one careless `{:?}` away from printing what the archive holds.
    match FileJournal::open(&path) {
        Err(OpenError::AlreadyOpen) => {}
        Err(other) => panic!("refused, but not as already open: {other:?}"),
        Ok(_) => panic!("a second journal on an already open file was accepted"),
    }
    drop(first);
}

/// A `redb::StorageBackend` WRITTEN FROM OUTSIDE `platform`, wrapped around the real one and
/// counting what passes through it.
///
/// ⚠️ IT BREAKS NOTHING, and that is deliberate — gotcha #50. The test around it speaks about
/// ORDINARY behaviour, so a fake that violated a promise would make it green for a reason that
/// has nothing to do with what it claims. A backend that FAILS belongs in milestone 4, in a
/// test that speaks about the failure.
#[derive(Debug)]
struct CountingBackend {
    inner: FileBackend,
    /// ⚠️ `Arc` and not a plain field: `FileJournal::with_backend` takes the backend BY VALUE,
    /// so the counters have to outlive the handing over or the test cannot read them.
    writes: Arc<AtomicU64>,
    syncs: Arc<AtomicU64>,
}

impl StorageBackend for CountingBackend {
    fn len(&self) -> Result<u64, io::Error> {
        self.inner.len()
    }

    fn read(&self, offset: u64, out: &mut [u8]) -> Result<(), io::Error> {
        self.inner.read(offset, out)
    }

    fn set_len(&self, len: u64) -> Result<(), io::Error> {
        self.inner.set_len(len)
    }

    fn sync_data(&self) -> Result<(), io::Error> {
        self.syncs.fetch_add(1, Ordering::Relaxed);
        self.inner.sync_data()
    }

    fn write(&self, offset: u64, data: &[u8]) -> Result<(), io::Error> {
        self.writes.fetch_add(1, Ordering::Relaxed);
        self.inner.write(offset, data)
    }
}
