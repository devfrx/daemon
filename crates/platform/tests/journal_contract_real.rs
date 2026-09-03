// THE SAME CONFORMANCE SUITE, RUN AGAINST THE REAL JOURNAL (§7.4.6).
//
// ⛔ THE ASSERTIONS ARE NOT REPEATED HERE, and that is the whole point of this file being short.
// They live in ONE place — `crates/kernel/tests/journal_contract.rs` — and are reached from here
// textually, because two copies would drift apart and THE FIRST ONE TO DRIFT WOULD LIE IN
// SILENCE: a suite that no longer compares the two implementations still prints `ok`. What makes
// the deterministic simulation worth anything is precisely that the in-memory double and `redb`
// answer the same questions, so the questions have to be ONE SET OF WORDS.
//
// `include!` is the mechanism because an integration test is A CRATE OF ITS OWN: it cannot `use`
// the items of another test target, so there is no import that would do this. The path is
// relative to this file's directory, which is why it climbs out of `crates/platform/tests/`.
//
// ⛔ AND THIS FILE HAS NO `use` OF ITS OWN, which is the mechanism's constraint and not a
// preference: the included file brings `use kernel::ports::journal::{Journal, JournalError,
// StepId}` along with it, and naming any of those three again here is `E0252`. Everything below
// is spelt out in full for that reason — `reactor_contract_real.rs` has no imports for the same
// one.
//
// ⚠️ DECLARED COST, accepted rather than unnoticed: `include!` brings the included file's
// `#[test]` functions with it, so THE FIFTEEN TESTS OF THE SUITE — the double, the substring
// constraint and the thirteen liars — RUN A SECOND TIME inside this binary. None of them touches
// the disk or sleeps, so it costs a few milliseconds, and it buys the single copy of the
// assertions. ⚠️ Only `the_real_journal_honours_the_contract` below reaches a file at all, which
// is what makes a red in this binary readable: a broken `FileJournal` turns THAT test red and
// leaves the other fifteen green, because they never meet it.
// ⚠️ THE TWO FIGURES SAID "TEN" AND "EIGHT" UNTIL 2026-08-17, and were already stale before the
// three blind journals of that day arrived: promise 7b's liar had made them eleven and nine.
// They said "FOURTEEN" AND "TWELVE" until 2026-08-27, when the bystander of promise 3 brought a
// thirteenth liar (finding AUD-019). Recounted from the source rather than bumped — gotcha #31.

include!("../../kernel/tests/journal_contract.rs");

#[test]
fn the_real_journal_honours_the_contract() {
    // ⛔ A FILE OF ITS OWN FOR EVERY CALL OF THE FACTORY, AND THE SUITE CALLS IT TEN TIMES. The
    // suite takes a factory precisely because several promises need a journal that has never been
    // written to — promise 3 asks a step to be `Missing`, promise 4 compares the WHOLE archive
    // against three records — so a factory handing back the same archive twice would let promise
    // 1's records be counted by promise 4, and the suite would go red on the bench instead of on
    // the implementation.
    //
    // ⛔ AND `FileJournal` TAKES AN EXCLUSIVE LOCK ON ITS FILE. The ten journals are dropped one
    // at a time — each promise builds inside its own block — but a factory that reused one path
    // would be one refactor away from `OpenError::AlreadyOpen`, which is a failure of the bench
    // wearing the mask of a failure of the port.
    //
    // ⚠️ BOTH FIGURES SAID "NINE" AND WERE STALE BEFORE 2026-08-17 TOUCHED THIS FILE, which is why
    // they are dated here rather than quietly corrected: promise 8 has TWO blocks — a note upon a
    // step nobody opened, and a note upon an open one — so the count went to ten the day `note`
    // arrived, and nothing recounted it. `grep -c '= build();'` answers ten. The three journals of
    // 2026-08-17 added assertions INSIDE existing blocks and no block of their own, so they did
    // not move this number at all.
    //
    // ⛔ A NAME THAT HAS NEVER EXISTED RATHER THAN A DELETION, which is gotcha #52 avoided
    // instead of met a second time. The obvious `remove_file` before each open FAILS SILENTLY on
    // Windows while the file is still open, so the factory would reopen the OLD DATA and the
    // promises would run against a dirty archive with nothing saying so; on Linux it would
    // succeed and hide the difference. A fresh name cannot be dirty on either.
    //
    // ⚠️ `AtomicU64` AND NOT A PLAIN COUNTER, and it was found by COMPILING rather than by
    // reading the signature: `assert_journal_contract` takes `F: Fn() -> J` and not `FnMut`, so
    // the closure cannot mutate what it captures. Interior mutability is the only way to number
    // the calls.
    let dir = private_dir_for_line(line!());
    let calls = std::sync::atomic::AtomicU64::new(0);

    assert_journal_contract(|| {
        let nth = calls.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = dir.join(format!("journal-{nth}.redb"));
        platform::journal::FileJournal::open(&path).expect("open")
    });
}

/// A directory of this test's own, emptied on entry — the same mechanism
/// `crates/platform/tests/file_journal.rs` uses, and for the same reason.
///
/// ⛔ ONE DIRECTORY PER CALL SITE, AND THE LINE NUMBER IS WHAT MAKES IT ONE. `cargo test` runs
/// the test BINARIES in parallel and libtest runs the tests inside one binary on parallel
/// threads, so a directory emptied on entry is only safe if nothing else can be inside it: a
/// shared directory with a different file name per test still loses, because the entering test
/// deletes the running one's file too. Two call sites cannot share a line number, so the
/// directories are distinct BY CONSTRUCTION and not by anybody remembering.
///
/// ⛔ AND THE PREFIX IS NOT THE ONE `file_journal.rs` USES, which is the half a line number alone
/// does not cover: line 49 of that file and line 49 of this one would name the SAME directory,
/// and those two binaries run at the same time.
///
/// ⚠️ Deliberately not a crate: `tempfile` would be a new entry in the dependency list for six
/// lines, and `platform` already names `std`.
fn private_dir_for_line(line: u32) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("daemon-journal-contract-{line}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create the test directory");
    dir
}
