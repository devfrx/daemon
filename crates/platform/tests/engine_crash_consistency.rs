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
/// Measured on the shape the short campaign runs — open, three writes, drop: the whole thing spends
/// **58** operations (23 to open, 23 for the three writes, 12 for the `Drop`), so a `falls_at`
/// of 58 or more NEVER FIRES and the run is indistinguishable from a run with no injection at
/// all. Of the forty points in `23..63`, **35 fire and 5 do not**; the highest that fires is 57.
/// Sweeping past saturation buys runs that explore no state and costs the same as the ones that
/// do.
///
/// ⚠️ AND IT SATURATES *PER DEPTH*, WHICH THIS DOC DID NOT SAY WHILE THERE WAS ONLY ONE DEPTH: a
/// scenario that writes more records spends more operations and so saturates FURTHER OUT, which is
/// the whole mechanism the deep campaign of 2026-08-11 is built on. The measured curve, and why
/// widening the range instead buys nothing, are on `DEEP_RECORDS`.
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

/// How many records the SHORT campaign writes. ⛔ THREE, AND THE ODD ONE IS THE POINT: two
/// records close a step and the third opens one, so the scenario ends with a step IN DOUBT —
/// which is the state a crash is interesting for at all.
const SHORT_RECORDS: u64 = 3;

/// The highest injection point the SHORT scenario can still reach. ⛔ MEASURED and not derived:
/// opening costs `OPERATIONS_TO_OPEN`, the three writes cost as much again, and the `Drop` of
/// `Database` costs twelve more — see the doc of `CrashingBackend`. Past this the fall NEVER
/// FIRES and the run is indistinguishable from one with no injection at all, so sweeping past it
/// buys runs that explore no state (gotcha #17).
///
/// ⛔ A SATURATION BELONGS TO ONE DEPTH, WHICH IS WHY THE CONSTANT NAMES ITS OWN — and the two
/// ways of getting the pairing wrong are caught by TWO DIFFERENT ORACLES. Measured on 2026-08-11,
/// not argued:
///
/// | the saturation is… | what goes wrong                              | which check fires    |
/// |--------------------|----------------------------------------------|----------------------|
/// | TOO HIGH for the depth | the tail of the range has no operation to land on | `fired == points`    |
/// | TOO LOW for the depth  | every point falls before the scenario ends        | `truncated < points` |
///
/// ⚠️ AND THIS PARAGRAPH CLAIMED THE SECOND ROW WAS SILENT — *"it simply stops the sweep early
/// while every assertion stays green"* — WHICH THE MUTATION DENIED: handing this constant to the
/// 30-record scenario gives `points=35 fired=35 truncated=35 partial=30`, and `truncated < points`
/// fires. That check went in at task 6 for gotcha #24, against a scenario growing DEARER; this is
/// a second defect it turns out to catch, and it is written down because the sentence it replaces
/// was plausible enough to have been written once already.
const SHORT_OPERATIONS_TO_SATURATION: u64 = 58;

/// The records a scenario of `records` records ATTEMPTS TO WRITE — GENERATED, not listed.
///
/// ⛔ GENERATING THEM IS WHAT MAKES A DEEPER SCENARIO POSSIBLE AT ALL, and it is also what keeps
/// the oracles honest. Every check a campaign makes speaks about *what was written*; a literal
/// list beside the writing loop is a second place saying the same thing, and the day the two
/// disagree the campaign asserts a prefix of the WRONG list and says nothing about it.
///
/// ⛔ THE PAIRING IS THE PORT'S AND NOT A CHOICE OF STYLE. `intent` refuses a step that already
/// carries one and `outcome` refuses a step that carries none — see `crates/platform/src/journal.rs`
/// — so a generator that put N records on ONE step would have every write after the first refused
/// BY THE JOURNAL, and the deep campaign would explore nothing while looking busy. Records go two
/// to a step, intent then outcome; an ODD count ends on a lone intent, which is exactly the shape
/// the three records of the short campaign have always had.
///
/// ⚠️ THE PAYLOADS ARE UNIFORM AND SHORT, and the substitution was MEASURED rather than waved
/// through: `b"one"`, `b"one done"`, `b"two"` became `record 0`, `record 1`, `record 2` and the
/// short campaign's five numbers did not move — 23 to open, saturation 58, `points=35 fired=35
/// truncated=22 partial=17`. Records this small live inside one page either way, so what a record
/// COSTS the backend is not its length.
fn scenario(records: u64) -> Vec<(StepId, Vec<u8>)> {
    (0..records)
        .map(|index| {
            (
                StepId::new(index / 2 + 1),
                format!("record {index}").into_bytes(),
            )
        })
        .collect()
}

/// Writes `records` records through a backend that falls at `falls_at`, then reopens the archive
/// and returns what came back — or WHY IT COULD NOT BE READ, which is itself an answer.
///
/// ⛔ IT HANDS BACK WHAT IT TRIED TO WRITE, and that is not a convenience: the prefix check needs
/// both sides, and the caller inventing the "written" side is exactly the drift `scenario` exists
/// to prevent. One call, one scenario, both halves of the comparison from the same place.
fn crash_then_reopen(
    records: u64,
    falls_at: u64,
) -> (
    Handles,
    Vec<(StepId, Vec<u8>)>,
    Result<Vec<(StepId, Vec<u8>)>, String>,
) {
    let archive = empty_archive();
    let (crashing, handles) = backend(&archive, falls_at);
    let written = scenario(records);

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
        for (index, (step, record)) in written.iter().enumerate() {
            // ⚠️ THE OPERATION FOLLOWS THE INDEX AND IS NOT CARRIED IN THE RECORD, for the reason
            // written on `scenario`: even records open a step, odd ones close it. `let _` because
            // after the fall every call answers `Err` — which is the case under test and not a
            // failure of it.
            let _ = if index % 2 == 0 {
                journal.intent(*step, record)
            } else {
                journal.outcome(*step, record)
            };
        }
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

    (handles, written, reopened)
}

/// ONE injection campaign: every point from the end of the opening to `saturation`, against a
/// scenario of `records` records, with the five checks that make a sweep of this shape mean
/// something.
///
/// ⛔ IT IS A FUNCTION AND NOT A SECOND `#[test]` BESIDE THE FIRST, which is the decision
/// `crates/simulator/tests/dst_campaign.rs` took at level 1 and the one the plan for this task got
/// wrong. A second test would have carried its own copy of the sweep with ONE oracle instead of
/// five — and the WEAKER of the two copies is the one that would have run on every commit under
/// the name "the campaign". There is one body, and both entry points use it.
///
/// ⚠️ THE SATURATION IS A PARAMETER AND NOT A CONSTANT READ IN HERE, because it belongs to the
/// DEPTH and not to the sweep: a deeper scenario costs more operations, so its last reachable point
/// is further out. Pairing the two wrong is caught in BOTH directions, and by two different checks
/// — the table is on `SHORT_OPERATIONS_TO_SATURATION`.
fn campaign(name: &str, records: u64, saturation: u64) {
    // ⛔ THE QUESTION LEVEL 2 ASKS, and the answer is not "everything survived": it is that what
    // comes back is a PREFIX of what was written — either the records confirmed before the fall,
    // or all of them, NEVER a partial record and never a scrambled one. ADR-0032 measured twelve
    // injection points and twelve coherent reopenings; this holds it at every commit.
    let started = std::time::Instant::now();
    let mut points = 0u64;
    let mut fired = 0u64;
    let mut truncated = 0u64;
    let mut partial = 0u64;
    // Every DISTINCT prefix length the sweep recovered — the rungs of the staircase. See S-5,
    // at the point where it is filled.
    let mut rungs: std::collections::BTreeSet<usize> = std::collections::BTreeSet::new();

    for falls_at in OPERATIONS_TO_OPEN..saturation {
        points += 1;
        let (handles, written, reopened) = crash_then_reopen(records, falls_at);
        if handles.fallen.load(Ordering::Relaxed) {
            fired += 1;
        }
        let back = match reopened {
            Ok(back) => back,
            // An archive that cannot be read back at all is a FAILURE of this promise, and it is
            // named — with the engine's own reason inside it — rather than skipped.
            Err(why) => panic!("{name}: injection at {falls_at}: {why}"),
        };
        assert!(
            written.starts_with(&back),
            "{name}: injection at {falls_at}: what came back is not a prefix of what was written: {back:?}"
        );
        if back.len() < written.len() {
            truncated += 1;
        }
        // A STEP OF THE STAIRCASE: something came back, and not everything. See `partial > 0`.
        if !back.is_empty() && back.len() < written.len() {
            partial += 1;
        }
        // ⛔ THE RUNGS THEMSELVES — finding S-5 of the 2026-08-11 audit, closed on 2026-08-18.
        // `partial > 0` is satisfied by ONE step, while the entire argument for a DEEPER
        // scenario rather than a WIDER sweep is that `records` records give `records + 1`
        // DISTINCT recoverable archives AND that the sweep reaches all of them. That claim
        // lived in the doc comment on `DEEP_RECORDS`, with its table of 4/4, 11/11, 21/21,
        // 31/31, 41/41 — and nothing counted it. The audit measured the price: a world reaching
        // THREE rungs out of thirty-one passes all five assertions below.
        //
        // ⚠️ A `BTreeSet` and not a hash set: the kernel's rule about `RandomState` (gotcha #12)
        // does not bind a test in `platform`, but a set whose iteration order depends on a
        // per-process seed has no place in a campaign whose whole purpose is reproducibility.
        rungs.insert(back.len());
    }

    // ⚠️ LEFT IN ON PURPOSE AND NOT DEBUGGING LEFTOVER: how many points a sweep of this shape
    // reaches, how many fire and how many actually cost the archive something is the measurement
    // a deeper campaign picks ITS range from, and a number reported once in a commit message is a
    // number nobody can re-read. `cargo test … -- --nocapture` shows it.
    //
    // ⛔ AND WHAT THE SHORT SWEEP COUNTED ON 2026-08-11 IS A MONOTONE STAIRCASE, which says more
    // than the prefix check can state on its own: injecting at 23..=27 gives back NO record,
    // 28..=33 gives back one, 34..=44 two, and 45 upwards all three. Records reappear ONE WHOLE
    // RECORD AT A TIME, in write order, each at a well-defined operation. ⚠️ THE BOUNDS ARE PROSE
    // AND PROSE AGES (gotcha #31): what HOLDS the shape is the assertions below, and these numbers
    // are here to be read, not to be trusted.
    //
    // ⛔ AND THE WALL TIME IS ON THIS LINE BECAUSE CONSTRAINT 7 OF §11 ASKS FOR IT — *printed on
    // every run, so that the slowdown becomes visible before it becomes a temptation*. It is the
    // only number here nobody can derive: the budget this campaign is sized against is one
    // measurement on one machine, and a measurement taken once decays. `gate.sh` does not yet
    // SHOW it — see `a_crashed_archive_reopens_in_a_coherent_state`.
    let elapsed = started.elapsed();
    println!(
        "{name}: records={records} points={points} fired={fired} truncated={truncated} \
         partial={partial} rungs={}/{} {elapsed:?}",
        rungs.len(),
        records + 1
    );

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
        "{name}: an injection point never fired: EITHER the engine now does LESS I/O than was \
         measured — check durability FIRST — OR the saturation this campaign was given \
         ({saturation}) is stale"
    );

    // ⛔ THE SECOND, AND WITHOUT IT THE FIRST IS NOT ENOUGH — the same lesson level 1 paid for.
    // "The injection went off" and "the injection cost the archive something" are two claims: if
    // every point left the archive whole, `records == written`, the prefix check above is
    // TRIVIALLY TRUE and this campaign is green having never lost a byte. The injection point is
    // drawn across all FIVE guarded operations of the backend — `close` does not pass through
    // `may_serve` — and of those, `read` and `len` take nothing away.
    assert!(
        truncated > 0,
        "{name}: no injection left the archive shorter than what was written: the prefix check \
         proved nothing"
    );

    // ⛔ THE THIRD IS THE OPPOSITE DIRECTION OF THE SECOND, AND IT WAS MISSING — gotcha #24.
    // `fired == points` catches the scenario getting CHEAPER; nothing caught it getting DEARER.
    // If the cost grows, the top of the staircase slides out of the range, every point comes back
    // short, and the sweep goes on being green having quietly stopped exercising the case where
    // everything survives — which is the case a journal is FOR.
    assert!(
        truncated < points,
        "{name}: every injection came back short: the top of the staircase has slid out of the \
         range, so the scenario now costs MORE than the saturation this campaign was given \
         ({saturation}) says"
    );

    // ⛔ THE FOURTH IS WHAT MAKES THIS SWEEP A SECOND WITNESS TO GOTCHA #51, and without it the
    // whole promise rests on ONE probe. MEASURED on 2026-08-11 and not supposed: under
    // `set_durability(Durability::None)` — WITH the saturation corrected to 51, so that the three
    // assertions above are all green — the staircase COLLAPSES TO ALL-OR-NOTHING. Nothing is
    // durable until the `Drop` of `Database` commits, so a point either falls before that and
    // gives back zero records, or after it and gives back three; the steps in between disappear.
    // ⚠️ AND THAT IS WHY THIS ONE IS WORTH MORE THAN THE OTHER THREE: it detects a lost durability
    // guarantee WITHOUT depending on the saturation, the fragile constant — and it is what turns
    // the staircase from a sentence in a comment into a check.
    //
    // ⛔ AND SINCE 2026-08-11 IT IS A WITNESS AT TWO DEPTHS, which is more than the same claim
    // twice. Re-measured under `Durability::None` on the 30-record scenario — with ITS saturation
    // corrected to 186, the same courtesy the short one got — the collapse has exactly the shape it
    // has at three records, 163 points deep: `partial=0`, nothing back or all thirty, never a step
    // between, and THIS is the assertion that fires. A lost durability guarantee is not an artefact
    // of a scenario too shallow to have steps in the first place.
    assert!(
        partial > 0,
        "{name}: the archive came back all-or-nothing, with no step in between: records have \
         stopped becoming durable ONE AT A TIME, which is what a lost durability guarantee looks \
         like from here"
    );

    // ⛔ THE FIFTH ASSERTION, AND IT IS FINDING S-5 — closed on 2026-08-18. The four above are
    // satisfied by a staircase with ONE step: `partial > 0` counts POINTS that landed mid-way,
    // not the DISTINCT archives they landed on, so a sweep in which every partial recovery
    // returned the same two records would pass all of them. The audit measured the price of
    // that gap: a world reaching THREE rungs out of thirty-one passes the whole campaign.
    //
    // ⛔ AND WHAT IT DEFENDS IS THE REASON THIS CAMPAIGN IS DEEP INSTEAD OF WIDE. The doc on
    // `DEEP_RECORDS` argues that a wider sweep buys nothing — 800 points, still 35 falls — while
    // more RECORDS buy states, and its evidence is a table of rungs: 4/4, 11/11, 21/21, 31/31,
    // 41/41, "every rung appears at every depth". That was the load-bearing claim of the whole
    // design AND IT WAS PROSE. This is the line that makes it a check.
    //
    // ⚠️ AN EQUALITY AND NOT `> 1`, deliberately: `records + 1` is exactly what the argument
    // claims — the empty archive plus one rung per record — so anything less means the sweep no
    // longer reaches every recoverable state and the depth is being paid for without being used.
    assert_eq!(
        rungs.len() as u64,
        records + 1,
        "{name}: the sweep reached {} distinct recoverable archives out of the {} this depth \
         is supposed to have. The rungs are the ENTIRE argument for a deeper scenario over a \
         wider sweep — if they no longer all appear, the depth is being paid for and not used. \
         Rungs seen: {rungs:?}",
        rungs.len(),
        records + 1
    );
}

#[test]
fn a_crashed_archive_reopens_in_a_coherent_state() {
    // ⛔ AND THIS IS THE SHORT CAMPAIGN — the one sweep of level 2 that runs on every commit, not
    // a cheap rehearsal beside a real one. There is no weaker copy: `campaign` holds all five
    // checks and both entry points call it.
    //
    // ⚠️ CONSTRAINT 7 OF §11, THE HALF THIS FILE OWNS: the wall time is PRINTED on every run — see
    // the `println!` in `campaign`. The other half, a gate step that SHOWS it, is task 9's:
    // `gate.sh` runs `cargo test --workspace` with no `--nocapture`, so today the line goes into a
    // buffer nobody reads. The same sentence, for the same reason, is on
    // `crates/simulator/tests/dst_campaign.rs`.
    //
    // ⛔ AND THE BUDGET IS IN `debug` AND NOT IN `--release`, which is where the plan for this task
    // had it. `gate.sh` is what pays for this file, and `gate.sh` runs `cargo build --workspace`
    // and `cargo test --workspace` — BOTH UNOPTIMISED. The two profiles are not close enough for
    // the distinction to be pedantry, measured on 2026-08-11: one injection point of this scenario
    // costs **3.5 ms in debug** against **0.29 ms in `--release`**, a factor of TWELVE. A ceiling
    // set on the fast profile is a ceiling nobody pays and nobody checks.
    //
    // ⛔ THE RULE, THEREFORE: the whole `engine_crash_consistency` binary stays under ONE SECOND of
    // wall time in `debug` — all five ordinary tests together, of which this sweep is by far the
    // dearest. Measured the same day, running the binary itself rather than `cargo`: **0.14 s**,
    // seven times inside the ceiling. ⚠️ THE CEILING IS ON THE BINARY AND NOT ON THIS SWEEP, which is what
    // stops the budget being spent twice — a sixth probe costs against the same second.
    // ⚠️ THE `DST` PREFIX MATCHES LEVEL 1's, and it is not decoration: `scripts/gate.sh` runs
    // both binaries with `--nocapture` so that constraint 7 of §11 has its wall time, and the two
    // lines exist to be READ AS A PAIR. Two prefixes would make whoever scans that output know
    // two spellings to find one thing.
    campaign(
        "DST L2 short",
        SHORT_RECORDS,
        SHORT_OPERATIONS_TO_SATURATION,
    );
}

/// How many records the DEEP campaign writes.
///
/// ⛔ IT IS THE **SCENARIO** THAT IS DEEPER AND NOT THE SWEEP, AND THAT IS THE WHOLE DESIGN OF
/// THIS CAMPAIGN. The obvious deep campaign — the same three records over an injection range
/// twenty times wider — explores EXACTLY ZERO extra states, and this was measured rather than
/// argued on 2026-08-11: past saturation the fall has no operation left to land on, so the run is
/// indistinguishable from one with no injection at all.
///
/// | sweep of the 3-record scenario | points | fired |
/// |--------------------------------|--------|-------|
/// | `23..58`, to saturation        | 35     | 35    |
/// | `23..100`                      | 77     | 35    |
/// | `23..300`                      | 277    | 35    |
/// | `23..823`                      | 800    | 35    |
///
/// Eight hundred points, still thirty-five falls: twenty-three times the cost for nothing at all.
/// Gotcha #17, and the reason a "deep" campaign of that shape would be a lie.
///
/// ⛔ WHAT DOES BUY STATES IS WRITING MORE, because every record costs the backend operations and
/// so pushes saturation OUT — a wider range that still fires. Measured the same day, counting the
/// RUNGS OF THE STAIRCASE, which is how many distinct prefix lengths came back over the sweep:
///
/// | records | saturation | points | rungs | debug  |
/// |---------|------------|--------|-------|--------|
/// | 3       | 58         | 35     | 4/4   | 0.13 s |
/// | 10      | 100        | 77     | 11/11 | 0.34 s |
/// | 20      | 160        | 137    | 21/21 | 0.78 s |
/// | 30      | 220        | 197    | 31/31 | 1.41 s |
/// | 40      | 280        | 257    | 41/41 | 2.23 s |
///
/// EVERY rung appears at every depth — `records` records give `records + 1` distinct recoverable
/// archives, all of them reached — so the deep sweep really does hand `crash_then_reopen` states
/// the short one never sees, which is the thing a wider range does not do.
///
/// ⚠️ AND THE NUMBER IS A DECLARED COST AND NOT A KNEE IN A CURVE, said plainly because the
/// level-1 deep campaign chose ITS constant against a SUBLINEAR curve and a reader will look for
/// the same reasoning here. There is none to find: rungs grow one per record, exactly, so no depth
/// is a natural stopping point and what decides is wall time. Thirty costs 1.4 s in debug,
/// comfortably under the 3.9 s the level-1 deep campaign already spends on the same long cycle.
const DEEP_RECORDS: u64 = 30;

/// The highest injection point the DEEP scenario can still reach.
///
/// ⛔ MEASURED, AND IT COULD NOT HAVE BEEN DERIVED — which the measurement itself showed, because
/// the writes do NOT cost a flat amount each. Counted on 2026-08-11 with `falls_at` at `u64::MAX`:
/// the first record costs 6 operations, the second 7, the THIRD TEN, the fourth and fifth 6 again,
/// and from there every record costs exactly 6. The `Drop` of `Database` costs 12 — except at two
/// records, where it costs 14. Extrapolating 220 from the short scenario's 58 would have landed
/// somewhere else, and the sweep would have stopped early or fired short without saying so.
///
/// ⛔ AND NEITHER DIRECTION OF A WRONG PAIRING TURNS OUT TO BE SILENT — measured, and the table is
/// on `SHORT_OPERATIONS_TO_SATURATION`. So what these names buy is not the red itself but a red a
/// reader can ACT on: both messages carry the saturation they were handed, which is the number to
/// go and look at.
const DEEP_OPERATIONS_TO_SATURATION: u64 = 220;

#[test]
#[ignore = "the deep campaign belongs to the long cycle — constraint 8 of §11"]
fn the_deep_injection_campaign() {
    // ⛔ THE SAME FIVE CHECKS AS THE SHORT ONE AND NOT WEAKER ONES, and it costs nothing to say so
    // because `campaign` holds them and there is no second copy to weaken. A sweep five times
    // longer is five times more expensive to run vacuously: gotcha #17 does not become less likely
    // with a longer range, it becomes more silent.
    campaign("DST L2 deep", DEEP_RECORDS, DEEP_OPERATIONS_TO_SATURATION);
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
