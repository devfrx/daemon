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
///
/// ⛔ AND A SCENARIO SATURATES, WHICH IS WHAT A CAMPAIGN HAS TO KNOW BEFORE IT PICKS ITS RANGE.
/// Measured on the shape task 7 will run — open, three writes, drop: the whole thing spends
/// **58** operations (23 to open, 23 for the three writes, 12 for the `Drop`), so a `falls_at`
/// of 58 or more NEVER FIRES and the run is indistinguishable from a run with no injection at
/// all. Of the forty points in `23..63`, **35 fire and 5 do not**; the highest that fires is 57.
/// Sweeping past saturation buys runs that explore no state and costs the same as the ones that
/// do.
#[derive(Debug)]
struct CrashingBackend {
    archive: Archive,
    falls_at: u64,
    operations: Arc<AtomicU64>,
    syncs: Arc<AtomicU64>,
    fallen: Arc<AtomicBool>,
}

/// What the test keeps after handing the backend over BY VALUE to `FileJournal::with_backend`.
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
        io::Error::other("the process is gone")
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

/// How many backend operations ONE CLEAN OPENING OF AN EMPTY ARCHIVE costs.
///
/// ⛔ OF AN EMPTY ONE, AND THE WORD IS LOAD-BEARING: reopening a POPULATED archive is a different
/// number, and not even a stable one. Measured over eight successive sessions on the same
/// archive: **18, 19, 19, 27, 18, 18, 18, 18**. Anything that wants to inject past a REOPENING
/// has to measure its own scenario — this constant does not cover it.
///
/// ⛔ MEASURED, NOT CALCULATED, and it could not have been calculated: `with_backend` COMMITS on
/// every open — it creates the table there so that every later read finds it — so an opening is
/// a whole write transaction, and how many operations that costs is `redb`'s business and not
/// ours. The figure comes from running this file's backend with `falls_at` at `u64::MAX`,
/// opening, and reading the counter before writing anything: **23**, against redb 4.1.0. Of
/// those, **19** are `create_with_backend` alone; the opening's own commit adds the other four.
///
/// ⚠️ AND THE POINT OF HAVING IT IS TO INJECT PAST IT — gotcha #17. Measured on the way here: at
/// `falls_at` 3 the fall fires INSIDE `with_backend`, which then answers `Err` and leaves behind
/// an archive that cannot even be reopened (`Engine(Io(Kind(InvalidData)))`). That is a test
/// about opening, not about writing.
///
/// ⚠️ PINNED RATHER THAN LOOSENED TO A `>=`, and it earns more than it looks. If the number moves,
/// the engine's I/O pattern moved underneath a crash campaign whose injection points are counted
/// in exactly these units, and the campaign of task 7 would go on injecting somewhere else
/// without saying so — the same reason `redb` names its minor in `Cargo.toml`. ⛔ AND IT IS ALSO
/// WHAT CATCHES A BACKEND THAT STOPPED GUARDING: dropping the `may_serve` call from any ONE of
/// the five operations is caught, and FOUR OF THE FIVE only through this counter. It does the
/// work of four guards.
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

        // ⚠️ THE SYNC DELTA THAT STOOD HERE MOVED OUT ON 2026-08-11, into
        // `the_engine_really_syncs_and_that_is_what_closes_gotcha_51`, and the line is dated
        // rather than deleted because the oracle did not change — only its address. It was right
        // here for a real reason, which went with it: the delta needs a test where the writes
        // SUCCEED. What it was NOT is this test's business — a probe named after reopening should
        // fail for reasons about reopening, and one claim asserted in two places is the
        // duplication this milestone has already taken out once.
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
    //
    // ⚠️ THE CONSTANT IS NAMED IN THE MESSAGE ON PURPOSE — gotcha #9, a red for the wrong reason.
    // A stale constant fails ASYMMETRICALLY: too high and the counter assertion below says so
    // plainly, too low and the fall lands inside the opening and this `expect` reports «the
    // process is gone», which reads as «the archive will not open» rather than «the number
    // pinned here is out of date». Naming it is what tells the two apart.
    let mut journal = FileJournal::with_backend(falling)
        .expect("the opening must survive: is OPERATIONS_TO_OPEN stale?");

    // ⛔ AND THIS ONE ASSERTION OWNS BOTH TRUTHS: that the opening cost what it was measured to
    // cost, AND that the fall did not fire inside it. A separate `!fallen` check stood here and
    // was DEAD — a fall implies `operations >= falls_at + 1`, so this line fires first in every
    // case that could reach it. Measured: sweeping `falls_at` across 0..80, the set of runs where
    // the opening survives AND the fall has fired is empty.
    assert_eq!(
        handles.operations.load(Ordering::Relaxed),
        OPERATIONS_TO_OPEN,
        "the opening did not cost what it was measured to cost; if it cost more, the fall fired \
         INSIDE it and this became a test about opening"
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

/// The highest injection point the scenario can still reach. ⛔ MEASURED and not derived:
/// opening costs `OPERATIONS_TO_OPEN`, the three writes cost as much again, and the `Drop` of
/// `Database` costs twelve more — see the doc of `CrashingBackend`. Past this the fall NEVER
/// FIRES and the run is indistinguishable from one with no injection at all, so sweeping past it
/// buys runs that explore no state (gotcha #17).
const OPERATIONS_TO_SATURATION: u64 = 58;

/// Writes three records through a backend that falls at `falls_at`, then reopens the archive and
/// returns what came back — or WHY IT COULD NOT BE READ, which is itself an answer.
fn crash_then_reopen(falls_at: u64) -> (Handles, Result<Vec<(StepId, Vec<u8>)>, String>) {
    let archive = empty_archive();
    let (crashing, handles) = backend(&archive, falls_at);

    // ⚠️ THE UNWIND IS CAUGHT AND THEN ASSERTED NOT TO HAVE HAPPENED, which is not the same as
    // swallowing it. A crashed engine would be entitled to panic on the way down; MEASURED over
    // every point from 0 to 59 on 2026-08-11, `redb` never does — its `Drop` runs a whole write
    // transaction under `if !thread::panicking()` and takes the error in silence. Catching and
    // discarding would make the day that changes look like an archive that simply came back
    // empty; catching and asserting makes it say so.
    let ran = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // ⚠️ THE `expect` NAMES THE CONSTANT — gotcha #9, which the sister probe has already paid
        // for once. A range starting BELOW `OPERATIONS_TO_OPEN` puts the fall inside the opening,
        // and this is the line that panics when it does.
        let mut journal = FileJournal::with_backend(crashing)
            .expect("the opening must survive: is OPERATIONS_TO_OPEN stale?");
        let _ = journal.intent(StepId::new(1), b"one");
        let _ = journal.outcome(StepId::new(1), b"one done");
        let _ = journal.intent(StepId::new(2), b"two");
    }));
    // ⛔ AND THE MESSAGE NAMES BOTH CANDIDATES RATHER THAN BLAMING THE ENGINE, which is what it
    // did until mutation B exhibited the false diagnosis on 2026-08-11: by far the likelier panic
    // is the `expect` just above — OURS — and reporting it as «the engine panicked» sends the
    // reader into `redb` to look for a stale constant of ours.
    assert!(
        ran.is_ok(),
        "injection at {falls_at}: something panicked on the way down — EITHER the opening did not \
         survive, and then the range starts below OPERATIONS_TO_OPEN, OR the engine panicked, \
         which it was measured never to do"
    );

    // The reopening goes through a backend that NEVER falls: what is under test is the state the
    // first fall left behind, not a second injection.
    let (fresh, _) = backend(&handles.archive, u64::MAX);

    // ⚠️ THE SAME POSTURE ON THIS SIDE — caught, and then declared not to have happened — and here
    // the reason is stronger than on the way down: this reads an archive left TORN by the fall,
    // which is exactly where a decoder is likeliest to trip. Measured over every point of the
    // sweep on 2026-08-11, neither the opening nor the replay ever panics.
    let reopened = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // ⛔ THE TWO FAILURES ARE TOLD APART, AND THE ENGINE'S OWN WORD IS CARRIED OUT WITH THEM —
        // the convention `crates/platform/tests/file_journal.rs` already writes down. An `.ok()`
        // on both calls folds «it would not open» and «it opened and would not replay» into one
        // sentence and throws the reason away; across thirty-five points that is a red with no
        // cause in it.
        match FileJournal::with_backend(fresh) {
            Err(error) => Err(format!("the archive did not reopen: {error:?}")),
            Ok(journal) => match journal.replay() {
                Err(error) => Err(format!(
                    "the archive reopened but would not replay: {error:?}"
                )),
                Ok(records) => Ok(records),
            },
        }
    }));
    let reopened = match reopened {
        Ok(answer) => answer,
        Err(_) => {
            panic!("injection at {falls_at}: reopening panicked, which it was measured never to do")
        }
    };

    (handles, reopened)
}

#[test]
fn a_crashed_archive_reopens_in_a_coherent_state() {
    // ⛔ THE QUESTION LEVEL 2 ASKS, and the answer is not "everything survived": it is that what
    // comes back is a PREFIX of what was written — either the records confirmed before the fall,
    // or all of them, NEVER a partial record and never a scrambled one. ADR-0032 measured twelve
    // injection points and twelve coherent reopenings; this holds it at every commit.
    let written: Vec<(StepId, Vec<u8>)> = vec![
        (StepId::new(1), b"one".to_vec()),
        (StepId::new(1), b"one done".to_vec()),
        (StepId::new(2), b"two".to_vec()),
    ];

    let mut points = 0u64;
    let mut fired = 0u64;
    let mut truncated = 0u64;
    let mut partial = 0u64;

    for falls_at in OPERATIONS_TO_OPEN..OPERATIONS_TO_SATURATION {
        points += 1;
        let (handles, reopened) = crash_then_reopen(falls_at);
        if handles.fallen.load(Ordering::Relaxed) {
            fired += 1;
        }
        let records = match reopened {
            Ok(records) => records,
            // An archive that cannot be read back at all is a FAILURE of this promise, and it is
            // named — with the engine's own reason inside it — rather than skipped.
            Err(why) => panic!("injection at {falls_at}: {why}"),
        };
        assert!(
            written.starts_with(&records),
            "injection at {falls_at}: what came back is not a prefix of what was written: {records:?}"
        );
        if records.len() < written.len() {
            truncated += 1;
        }
        // A STEP OF THE STAIRCASE: something came back, and not everything. See `partial > 0`.
        if !records.is_empty() && records.len() < written.len() {
            partial += 1;
        }
    }

    // ⚠️ LEFT IN ON PURPOSE AND NOT DEBUGGING LEFTOVER: how many points a sweep of this shape
    // reaches, how many fire and how many actually cost the archive something is the measurement
    // task 7 picks ITS range from, and a number reported once in a commit message is a number
    // nobody can re-read. `cargo test … -- --nocapture` shows it.
    //
    // ⛔ AND WHAT IT COUNTED ON 2026-08-11 IS A MONOTONE STAIRCASE, which says more than the
    // prefix check can state on its own: injecting at 23..=27 gives back NO record, 28..=33 gives
    // back one, 34..=44 two, and 45 upwards all three. Records reappear ONE WHOLE RECORD AT A
    // TIME, in write order, each at a well-defined operation. ⚠️ THE BOUNDS ARE PROSE AND PROSE
    // AGES (gotcha #31): what HOLDS the shape is the four assertions below, and these numbers are
    // here to be read, not to be trusted.
    println!("points={points} fired={fired} truncated={truncated} partial={partial}");

    // ⛔ THE FIRST NON-VACUITY, AND IT IS AN EQUALITY. The range stops at saturation precisely so
    // that every point reaches its operation: one that did not would be a run indistinguishable
    // from a run with no injection, and `> 0` would let all but one of them go quiet.
    //
    // ⚠️ AND THE MESSAGE NAMES BOTH CAUSES IN THE ORDER WORTH CHECKING — measured on 2026-08-11,
    // when `Durability::None` turned this red and the message blamed a drift while NOTHING had
    // drifted: fewer syncs make the whole scenario cheaper, so saturation falls from 58 to about
    // 51 and the tail of the range stops firing. The engine's I/O pattern is the cause this file
    // exists for; a stale constant is the boring one.
    assert_eq!(
        fired, points,
        "an injection point never fired: EITHER the engine now does LESS I/O than was measured — \
         check durability FIRST — OR OPERATIONS_TO_SATURATION is stale"
    );

    // ⛔ THE SECOND, AND WITHOUT IT THE FIRST IS NOT ENOUGH — the same lesson level 1 paid for.
    // "The injection went off" and "the injection cost the archive something" are two claims: if
    // every point left the archive whole, `records == written`, the prefix check above is
    // TRIVIALLY TRUE and this campaign is green having never lost a byte. The injection point is
    // drawn across all FIVE guarded operations of the backend — `close` does not pass through
    // `may_serve` — and of those, `read` and `len` take nothing away.
    assert!(
        truncated > 0,
        "no injection left the archive shorter than what was written: the prefix check proved nothing"
    );

    // ⛔ THE THIRD IS THE OPPOSITE DIRECTION OF THE SECOND, AND IT WAS MISSING — gotcha #24.
    // `fired == points` catches the scenario getting CHEAPER; nothing caught it getting DEARER.
    // If the cost grows, the top of the staircase slides out of the range, every point comes back
    // short, and the sweep goes on being green having quietly stopped exercising the case where
    // everything survives — which is the case a journal is FOR.
    assert!(
        truncated < points,
        "every injection came back short: the top of the staircase has slid out of the range, so \
         the scenario now costs MORE than OPERATIONS_TO_SATURATION says"
    );

    // ⛔ THE FOURTH IS WHAT MAKES THIS SWEEP A SECOND WITNESS TO GOTCHA #51, and without it the
    // whole promise rests on ONE probe. MEASURED on 2026-08-11 and not supposed: under
    // `set_durability(Durability::None)` — WITH the saturation corrected to 51, so that the three
    // assertions above are all green — the staircase COLLAPSES TO ALL-OR-NOTHING. Nothing is
    // durable until the `Drop` of `Database` commits, so a point either falls before that and
    // gives back zero records, or after it and gives back three; the steps in between disappear.
    // ⚠️ AND THAT IS WHY THIS ONE IS WORTH MORE THAN THE OTHER THREE: it detects a lost durability
    // guarantee WITHOUT depending on `OPERATIONS_TO_SATURATION`, the fragile constant — and it is
    // what turns the staircase from a sentence in a comment into a check.
    assert!(
        partial > 0,
        "the archive came back all-or-nothing, with no step in between: records have stopped \
         becoming durable ONE AT A TIME, which is what a lost durability guarantee looks like \
         from here"
    );
}

#[test]
fn the_engine_really_syncs_and_that_is_what_closes_gotcha_51() {
    // ⛔ THE PROMISE NOBODY HELD UNTIL NOW. `FileJournal` promises a write survives the death of
    // the process, and putting `set_durability(Durability::None)` into it leaves ALL SIX tests of
    // `file_journal.rs` GREEN — they reopen the file inside a LIVING process, so the writes are
    // in the operating system's hands either way. Gotcha #51.
    //
    // ⛔ AND WHAT MAKES IT OBSERVABLE IS A DELTA, NOT A COUNT — measured, because the obvious form
    // is blind. Opening alone produces seven `sync_data` calls, six of them from
    // `create_with_backend` before any journal exists and the seventh from the commit the opening
    // itself makes: `syncs > 0` is satisfied by an engine that syncs NO WRITE AT ALL, and stays
    // green under the very mutation this test exists to catch. What the write must move is the
    // count ACROSS itself, and a write is worth exactly one sync — MEASURED as the ladder 6 bare,
    // 7 open, 8 after the intent, 9 after the outcome.
    //
    // ⚠️ AND IT INJECTS NOTHING — `u64::MAX` — because the oracle needs a write that SUCCEEDS.
    // Beside a fall it would be worthless: there the write fails for the fall's own reason, and
    // the count stops moving whether the engine wanted durability or not.
    let archive = empty_archive();
    let (crashing, handles) = backend(&archive, u64::MAX);

    let mut journal = FileJournal::with_backend(crashing).expect("open");
    let after_open = handles.syncs.load(Ordering::Relaxed);
    journal.intent(StepId::new(1), b"one").expect("intent");

    assert!(
        handles.syncs.load(Ordering::Relaxed) > after_open,
        "the write did not ask the engine to make anything durable: the durability guarantee is not being asked for"
    );
}
