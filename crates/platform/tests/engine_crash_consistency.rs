//! Level 2 of the two crash levels (ADR-0032, §4.6): the subject under test is not the kernel
//! but `redb` ITSELF, driven through the backend we control — *does the engine leave a
//! recoverable archive?*
//!
//! ⛔ IT LIVES IN A TEST BINARY AND NOT IN `platform/src/`, and that is the point rather than
//! tidiness: what task 8 of milestone 3 bought is that the `StorageBackend` boundary is
//! reachable FROM OUTSIDE THE CRATE (gotcha #46). A failing backend written inside `platform`
//! would prove nothing about that. It sits beside `CountingBackend` in
//! `crates/platform/tests/file_journal.rs`, whose comment named this milestone in advance.

use std::io;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use kernel::ports::journal::{Journal, JournalError, StepId};
use platform::journal::FileJournal;
use redb::StorageBackend;

/// The bytes of the archive, held OUTSIDE the backend that serves them.
///
/// ⛔ THIS IS WHY `redb::InMemoryBackend` IS NOT USED, and it was measured rather than assumed:
/// that type is `InMemoryBackend(RwLock<Vec<u8>>)` with PRIVATE guards, so the bytes die with
/// the object and the archive CANNOT BE REOPENED — and reopening is the entire question level 2
/// asks. ADR-0032 measured with a backend of its own for the same reason.
type Archive = Arc<Mutex<Vec<u8>>>;

fn empty_archive() -> Archive {
    Arc::new(Mutex::new(Vec::new()))
}

/// A `redb::StorageBackend` that STOPS SERVING at an operation chosen by the caller.
///
/// ⚠️ `close` NEVER FAILS, and it is an exception with a reason: `redb` calls it exactly once
/// when the `Database` is dropped, and a failure there would fire during unwinding rather than
/// at the injection point the test is about.
#[derive(Debug)]
struct CrashingBackend {
    archive: Archive,
    falls_at: u64,
    operations: Arc<AtomicU64>,
    syncs: Arc<AtomicU64>,
    fallen: Arc<AtomicBool>,
}

/// What the test keeps after handing the backend over BY VALUE to `FileJournal::with_backend`.
#[derive(Clone)]
struct Handles {
    archive: Archive,
    operations: Arc<AtomicU64>,
    syncs: Arc<AtomicU64>,
    fallen: Arc<AtomicBool>,
}

fn backend(archive: &Archive, falls_at: u64) -> (CrashingBackend, Handles) {
    let handles = Handles {
        archive: Arc::clone(archive),
        operations: Arc::new(AtomicU64::new(0)),
        syncs: Arc::new(AtomicU64::new(0)),
        fallen: Arc::new(AtomicBool::new(false)),
    };
    let backend = CrashingBackend {
        archive: Arc::clone(archive),
        falls_at,
        operations: Arc::clone(&handles.operations),
        syncs: Arc::clone(&handles.syncs),
        fallen: Arc::clone(&handles.fallen),
    };
    (backend, handles)
}

impl CrashingBackend {
    /// Whether this operation may proceed, MARKING the fall when it may not. Once fallen, the
    /// backend never serves again — the same permanence the level-1 journal has, and for the
    /// same reason: a backend that refused once and worked again would model a bad disk.
    fn may_serve(&self) -> bool {
        if self.fallen.load(Ordering::Relaxed) {
            return false;
        }
        if self.operations.fetch_add(1, Ordering::Relaxed) == self.falls_at {
            self.fallen.store(true, Ordering::Relaxed);
            return false;
        }
        true
    }

    fn gone() -> io::Error {
        io::Error::new(io::ErrorKind::Other, "the process is gone")
    }
}

impl StorageBackend for CrashingBackend {
    fn len(&self) -> Result<u64, io::Error> {
        if !self.may_serve() {
            return Err(Self::gone());
        }
        Ok(self.archive.lock().expect("archive").len() as u64)
    }

    fn read(&self, offset: u64, out: &mut [u8]) -> Result<(), io::Error> {
        if !self.may_serve() {
            return Err(Self::gone());
        }
        let guard = self.archive.lock().expect("archive");
        let offset = usize::try_from(offset).map_err(|_| Self::gone())?;
        if offset + out.len() > guard.len() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "out of range"));
        }
        out.copy_from_slice(&guard[offset..offset + out.len()]);
        Ok(())
    }

    fn set_len(&self, len: u64) -> Result<(), io::Error> {
        if !self.may_serve() {
            return Err(Self::gone());
        }
        let len = usize::try_from(len).map_err(|_| Self::gone())?;
        // ⛔ ZERO-FILLED, because `redb`'s own trait says so: "New positions in the storage must
        // be initialized to zero". `Vec::resize` with 0 is exactly that, and getting it wrong
        // would corrupt the archive for a reason that has nothing to do with the injection.
        self.archive.lock().expect("archive").resize(len, 0);
        Ok(())
    }

    /// ⚠️ THE COUNTER MOVES BEFORE THE GUARD, and it is the ONLY method here that does. It is
    /// deliberate: what the level-2 campaign will ask of this counter is *«was the engine ever
    /// asked to make its writes durable?»* — the oracle that closes gotcha #51, because with
    /// `Durability::None` `redb` never calls this at all. An attempt that the fall refuses is
    /// still an attempt, and counting it after the guard would hide exactly the call the
    /// campaign is looking for. `CountingBackend` counts in the same place, for the same reason.
    fn sync_data(&self) -> Result<(), io::Error> {
        self.syncs.fetch_add(1, Ordering::Relaxed);
        if !self.may_serve() {
            return Err(Self::gone());
        }
        Ok(())
    }

    fn write(&self, offset: u64, data: &[u8]) -> Result<(), io::Error> {
        if !self.may_serve() {
            return Err(Self::gone());
        }
        let mut guard = self.archive.lock().expect("archive");
        let offset = usize::try_from(offset).map_err(|_| Self::gone())?;
        if offset + data.len() > guard.len() {
            guard.resize(offset + data.len(), 0);
        }
        guard[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }

    fn close(&self) -> Result<(), io::Error> {
        Ok(())
    }
}

/// How many backend operations ONE CLEAN OPENING costs, from the first to the last.
///
/// ⛔ MEASURED, NOT CALCULATED, and it could not have been calculated: `with_backend` COMMITS on
/// every open — it creates the table there so that every later read finds it — so an opening is
/// a whole write transaction, and how many operations that costs is `redb`'s business and not
/// ours. The figure comes from running this file's backend with `falls_at` at `u64::MAX`,
/// opening, and reading the counter before writing anything: **23**, against redb 4.1.0.
///
/// ⚠️ AND THE POINT OF HAVING IT IS TO INJECT PAST IT — gotcha #17. Measured on the way here: at
/// `falls_at` 3 the fall fires INSIDE `with_backend`, which then answers `Err` and leaves behind
/// an archive that cannot even be reopened (`Engine(Io(Kind(InvalidData)))`). That is a test
/// about opening, not about writing.
///
/// ⚠️ PINNED RATHER THAN LOOSENED TO A `>=`. If this number moves, the engine's I/O pattern moved
/// underneath a crash campaign whose injection points are counted in exactly these units, and
/// the campaign of task 7 would go on injecting somewhere else without saying so. It is the same
/// reason `redb` names its minor in `Cargo.toml`.
const OPERATIONS_TO_OPEN: u64 = 23;

#[test]
fn without_a_crash_the_archive_reopens_with_everything_in_it() {
    // ⛔ THE OTHER DIRECTION FIRST (rule 3 of §7.1.1): if this failed, every red below would be
    // about the backend rather than about the injection.
    let archive = empty_archive();
    // ⚠️ NOT NAMED `backend`: the helper of that name is called again below, and a local binding
    // shadows a function in the value namespace — the second call would be `expected function,
    // found CrashingBackend`.
    let (first_backend, handles) = backend(&archive, u64::MAX);

    {
        let mut journal = FileJournal::with_backend(first_backend).expect("open");
        journal.intent(StepId::new(1), b"one").expect("intent");
        journal
            .outcome(StepId::new(1), b"one done")
            .expect("outcome");
    }

    let (reopened_backend, _) = backend(&handles.archive, u64::MAX);
    let reopened = FileJournal::with_backend(reopened_backend).expect("reopen");
    assert_eq!(
        reopened.replay().expect("replay"),
        vec![
            (StepId::new(1), b"one".to_vec()),
            (StepId::new(1), b"one done".to_vec()),
        ]
    );
}

#[test]
fn the_backend_falls_at_the_operation_it_was_told_to() {
    // ⛔ WITHOUT THIS TEST THE FALL IS DEAD CODE, and nothing would say so: the compiler cannot
    // see it. `backend` reads `operations`, `syncs` and `fallen` to build the `CrashingBackend`,
    // so the fields count as read and `dead_code` stays quiet even when NO TEST ever makes the
    // backend fall — measured, not assumed. The only signal is the missing probe itself.
    let archive = empty_archive();
    let (falling, handles) = backend(&archive, OPERATIONS_TO_OPEN);

    // The opening happens BEFORE the injection point and must therefore survive it whole.
    let mut journal = FileJournal::with_backend(falling).expect("the opening must survive");
    assert_eq!(
        handles.operations.load(Ordering::Relaxed),
        OPERATIONS_TO_OPEN,
        "the opening did not cost what it was measured to cost"
    );
    assert!(
        !handles.fallen.load(Ordering::Relaxed),
        "the fall fired inside the opening, so this test is about opening"
    );

    // ⚠️ AND THE ENGINE ASKED FOR DURABILITY WHILE DOING IT — the oracle task 6 needs to close
    // gotcha #51, proved alive here for the price of one line. With `Durability::None` `redb`
    // never calls `sync_data` AT ALL, so a zero here would mean the level-2 campaign is watching
    // a journal that never syncs and cannot tell. Measured: **7** syncs in one clean opening.
    assert!(
        handles.syncs.load(Ordering::Relaxed) > 0,
        "a clean opening never asked the backend to make anything durable"
    );

    // And the very next operation is the one refused.
    assert_eq!(
        journal.intent(StepId::new(1), b"one"),
        Err(JournalError::NotDurable),
        "the write went through an operation that was told to fall"
    );
    assert!(
        handles.fallen.load(Ordering::Relaxed),
        "the write failed, but not because of the fall"
    );
}

#[test]
fn after_the_fall_the_backend_never_serves_again() {
    // ⛔ DRIVEN DIRECTLY AND NOT THROUGH A JOURNAL. Permanence is a property of the backend, and
    // `redb` is entitled to give up after the first refusal — with a journal in the way this
    // would stay green by never making the second call, which is the vacuous shape of the test.
    let archive = empty_archive();
    let (falling, handles) = backend(&archive, 0);

    assert!(
        falling.len().is_err(),
        "the very first operation was told to fall"
    );
    assert!(
        falling.len().is_err(),
        "the backend served again after it had fallen"
    );
    assert!(handles.fallen.load(Ordering::Relaxed));

    // ⚠️ AND THE COUNTER FROZE AT THE FALL, because the refusal returns before the `fetch_add`.
    // A backend still counting would be a backend still weighing each operation — the same
    // defect seen from the other side.
    assert_eq!(handles.operations.load(Ordering::Relaxed), 1);
}
