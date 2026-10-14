//! The real journal, checked on WHAT ONLY IT PROMISES. What both implementations promise is
//! the conformance suite's business — `crates/kernel/tests/journal_contract.rs` — which is run
//! against this type by `crates/platform/tests/journal_contract_real.rs` since 2026-08-10; this
//! line said "at task 9" until that file existed. Asserting the promises below inside that suite
//! would turn the in-memory double red, and the double is CORRECT not to make them: gotcha #44.
//!
//! ⚠️ AND NO TWO FILES SHARE A TEMPORARY DIRECTORY, which the `line!()` scheme below does not
//! cover on its own: a line number is unique within ONE file, and `temp_dir()` is one for every
//! process of the account. So every bench that makes its directories this way has a prefix of its
//! own -- `daemon-file-journal-` here, `daemon-journal-contract-` in `journal_contract_real.rs` -- and
//! `grep -rhoE 'daemon-[a-z-]+' crates/ | sort -u` lists the ones in use. ⛔ WHAT THE PREFIX KEEPS
//! APART IS TWO PROCESSES AT ONCE, NOT THE BINARIES OF ONE `cargo test`: that command runs its
//! test binaries one after the other -- "run serially", `man cargo-test` of the toolchain this
//! repository pins -- so inside one invocation two files never meet. Two processes do: two
//! `cargo test` invocations that overlap on one machine, or a runner that starts the binaries
//! together. ⚠️ RECALL OF 2026-10-13 -- audit of 2026-09-30, AUD-1383, AUD-1388, AUD-1390.
//! ⛔ AND THE PROCESS ID KEEPS APART TWO RUNS OF THE SAME FILE, which the prefix cannot: every
//! directory name carries it, as `channel_for_line` of `crates/daemon/src/main.rs` carries it in a
//! channel's, because `temp_dir()` is shared by every process of the account and two overlapping
//! runs of one bench would otherwise share every directory -- the second emptying it under the
//! first, the shape of gotcha #52. ⛔ AND A GUARD REMOVES THE DIRECTORY WHEN THE PROBE IS DONE:
//! with the process id in its name no later run reuses it, so one left behind would stay for good.
//! The same pair -- the process id and the guard -- is in every bench that makes its directories
//! this way, `crates/daemon/src/main.rs` included, each with a copy of its own: test code does not
//! cross a crate boundary. ⚠️ RECALL OF 2026-10-14 -- same audit, AUD-2127.

use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use kernel::ports::journal::{Journal, JournalError, StepId};
use platform::journal::{FileBackend, FileJournal, OpenError};
use redb::StorageBackend;

/// A directory of this test's own, fresh on entry and removed by its guard when the test is done.
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
/// ⚠️ Deliberately not a crate: `tempfile` would be a new entry for a helper of a few lines, and
/// `platform` already names `std`. ⚠️ RECALL OF 2026-10-13 -- audit of 2026-09-30, AUD-1391.
///
/// ⚠️ AND IT IS DEFINED UP HERE, NOT NEXT TO THE OTHER HELPERS AT THE BOTTOM: a `macro_rules!`
/// is visible only AFTER its definition in the file, so a macro written below its callers does
/// not compile.
macro_rules! private_dir {
    () => {
        private_dir_for_line(line!())
    };
}

fn private_dir_for_line(line: u32) -> PrivateDir {
    let dir =
        std::env::temp_dir().join(format!("daemon-file-journal-{}-{line}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create the test directory");
    PrivateDir(dir)
}

/// The directory `private_dir_for_line` hands out, REMOVED WHEN IT FALLS -- on a red too, since a
/// panicking test drops its locals as it unwinds. Why the guard exists is at the head of this file.
/// ⚠️ A REMOVAL THAT FAILS IS IGNORED: on Windows a file still open refuses to go, and a guard that
/// panicked in `drop` would turn a green probe red for a reason that is not the probe's.
struct PrivateDir(std::path::PathBuf);

impl std::ops::Deref for PrivateDir {
    type Target = std::path::Path;

    fn deref(&self) -> &std::path::Path {
        &self.0
    }
}

impl Drop for PrivateDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// ⛔ ADR-0023 PROMISES "PROTECTED AS MUCH AS YOUR SYSTEM ACCOUNT IS", AND 0644 IS LESS.
///
/// Finding PL-1 of the 2026-08-11 audit. `OpenOptions::create(true)` asks the OS for
/// `0o666 & !umask`; on a stock Linux that is **0644**, world-readable. The journal is the one
/// archive ADR-0022 marks as encrypted-and-backed-up, so a world-readable one contradicts the
/// sentence the ADR requires to be shown IN THE INTERFACE.
///
/// ⚠️ THE ASSERTION IS "NOBODY BUT THE OWNER", NOT "EXACTLY 0600", and the difference is
/// umask. `mode()` sets what is HANDED to `open(2)`, and the kernel still masks it: with a
/// hostile umask an exact-equality assertion would go red on a system that is MORE closed than
/// asked, that is, a red where the promise is kept. `mode & 0o077 == 0` is the promise itself.
///
/// ⚠️ AND IT ONLY BITES ON CREATION. `mode()` is ignored when the file already exists, so a
/// journal created before this line keeps 0644 for ever. That is a MIGRATION, not a bug in this
/// probe, and it is declared here because nobody would find it later: the fixture below deletes
/// the directory on entry, so the probe always sees a fresh file and CANNOT see the old case.
///
/// ⚠️ `cfg(unix)`: on Windows there is no Unix mode, which is exactly why the defect could not
/// appear on the development host. This runs on the Linux CI — gotcha #52, same shape.
#[cfg(unix)]
#[test]
fn the_journal_file_is_not_world_readable() {
    use std::os::unix::fs::PermissionsExt;

    let dir = private_dir!();
    let path = dir.join("journal.redb");

    let mut journal = FileJournal::open(&path).expect("open");
    journal.intent(StepId::new(1), b"private").expect("intent");
    drop(journal);

    let mode = std::fs::metadata(&path)
        .expect("stat the journal file")
        .permissions()
        .mode();

    assert_eq!(
        mode & 0o077,
        0,
        "ADR-0023 promises the journal is protected as much as the account: mode is {:o}, \
         which grants group or other. Finding PL-1.",
        mode & 0o777
    );
    assert_eq!(
        mode & 0o600,
        0o600,
        "the owner must still be able to read and write it: mode is {:o}",
        mode & 0o777
    );
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
    // in a test binary, and `FileJournal` runs on it unchanged. This one only counts.
    //
    // ⚠️ AND THE FAILING ONE ARRIVED ON 2026-08-11 (§4.6, ADR-0032 requirement 4): it is
    // `CrashingBackend`, in `crates/platform/tests/engine_crash_consistency.rs`, a test binary of
    // its own — the same place, for the same reason. This line said "milestone 4 WILL put a
    // failing one in the same place" until it did. ⛔ IT IS DATED RATHER THAN DELETED because the
    // sentence it replaces is the whole reason that file sits in `tests/` and not in `src/`: a
    // backend written inside `platform` would prove nothing about a boundary being reachable from
    // outside it, and a reader who found the crashing backend over there without this line would
    // be entitled to think the placement was tidiness.
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
    // silence — two writers on one journal would be two cores on one authoritative state, which
    // ADR-0004's single instance of the core rules out. ⚠️ RECALL OF 2026-10-13 -- audit of
    // 2026-09-30, AUD-786.
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

#[test]
fn a_journal_under_a_directory_that_is_not_there_is_refused_as_a_file_error() {
    // ⛔ THE OTHER HALF OF WHY `OpenError` EXISTS: "a wrong path and a locked file are exactly
    // the two things a human has to be able to tell apart", and the locked file is the probe
    // above. A change that folded the variants, or sent the failure of the open down another arm,
    // would leave that probe green and turn this one red.
    //
    // ⚠️ A DIRECTORY THAT IS NOT THERE, because it fails the same way on both of the project's
    // systems and needs no privileges -- the road `crates/daemon/src/main.rs` takes to stop its
    // start-up on a journal that will not open.
    let dir = private_dir!();

    match FileJournal::open(&dir.join("no-such-directory").join("journal.redb")) {
        Err(OpenError::File(_)) => {}
        Err(other) => panic!("refused, but not as a file that would not open: {other:?}"),
        Ok(_) => panic!("a journal under a directory that is not there was opened"),
    }
}

#[test]
fn a_file_that_is_not_a_journal_is_refused_by_the_engine() {
    // ⛔ THE THIRD VARIANT, AND THE ONE NO PROBE PROVOKED: the file opens and its lock is taken,
    // and what refuses is `redb`, on reading what the file holds. Folded into `File`, a human
    // would hunt a path that is right; folded into `AlreadyOpen`, a second core that is not there.
    //
    // ⚠️ THE BYTES ARE NONE OF `redb`'s, and there are enough of them: an EMPTY file is not this
    // case at all -- the engine takes it for a new archive and writes one into it.
    let dir = private_dir!();
    let path = dir.join("journal.redb");
    std::fs::write(&path, [0xA5_u8; 4_096]).expect("a file that holds no archive");

    match FileJournal::open(&path) {
        Err(OpenError::Engine(_)) => {}
        Err(other) => panic!("refused, but not by the engine: {other:?}"),
        Ok(_) => panic!("a file that is not a journal was opened as one"),
    }
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
