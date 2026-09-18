// THE SAME CONFORMANCE SUITE, RUN AGAINST THE REAL CUSTODY (§2 of the GUI north star).
//
// ⛔ THE ASSERTIONS ARE NOT REPEATED HERE, and that is the whole point of this file being short.
// They live in ONE place -- `crates/kernel/tests/custody_contract.rs` -- and are reached from
// here textually, because two copies would drift and THE FIRST ONE TO DRIFT WOULD LIE IN
// SILENCE. `include!` is the mechanism because an integration test is A CRATE OF ITS OWN: it
// cannot `use` the items of another test target. The path is relative to this file's directory,
// which is why it climbs out of `crates/platform/tests/`.
//
// ⛔ AND THIS FILE HAS NO `use` OF ITS OWN, which is the mechanism's constraint and not a
// preference: the included file brings `use kernel::ports::custody::{Custody, CustodyError,
// CustodyKey}` along with it, and naming any of those three again here is `E0252`. Everything
// below is spelt out in full for that reason -- `journal_contract_real.rs` has no imports for
// the same one.
//
// ⚠️ DECLARED COST, accepted rather than unnoticed: `include!` brings the included file's
// `#[test]` functions with it, so the suite's tests RUN A SECOND TIME inside this binary. None
// of them touches the disk, so it costs a few milliseconds and it buys the single copy of the
// assertions. ⚠️ HOW MANY IS NOT WRITTEN HERE, and that is deliberate: a figure inside a
// sentence that stays true is gotcha #31, and `grep -c '^#\[test\]'` on the two files answers it
// whenever it is asked. Only `the_real_custody_honours_the_contract` below reaches a file at
// all, which is what makes a red in this binary readable.

include!("../../kernel/tests/custody_contract.rs");

#[test]
fn the_real_custody_honours_the_contract() {
    // ⛔ A FILE OF ITS OWN FOR EVERY CALL OF THE FACTORY. The suite takes a factory precisely
    // because several promises need an archive that has never been written to -- promise 3 asks
    // for `Ok(None)`, promise 5 asks about an EMPTY package -- so a factory handing back the same
    // archive twice would let promise 1's bytes be found by promise 3, and the suite would go red
    // ON THE BENCH instead of on the implementation.
    //
    // ⛔ AND `FileBackend` TAKES AN EXCLUSIVE LOCK. A factory reusing one path would be one
    // refactor away from `OpenError::AlreadyOpen`, which is a failure of the bench wearing the
    // mask of a failure of the port.
    //
    // ⛔ A NAME THAT HAS NEVER EXISTED RATHER THAN A DELETION -- gotcha #52 avoided instead of met
    // again. `remove_file` before each open FAILS SILENTLY on Windows while the file is still
    // open, so the factory would reopen the OLD DATA with nothing saying so.
    //
    // ⚠️ `AtomicU64` AND NOT A PLAIN COUNTER: `assert_custody_contract` takes `F: Fn() -> C` and
    // not `FnMut`, so the closure cannot mutate what it captures.
    let dir = private_dir_for_line(line!());
    let calls = std::sync::atomic::AtomicU64::new(0);

    assert_custody_contract(|| {
        let nth = calls.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = dir.join(format!("custody-{nth}.redb"));
        platform::custody::FileCustody::open(&path).expect("open")
    });
}

/// A directory of this test's own, emptied on entry -- the same mechanism
/// `crates/platform/tests/file_journal.rs` uses, and for the same reason.
///
/// ⛔ ONE DIRECTORY PER CALL SITE, AND THE LINE NUMBER IS WHAT MAKES IT ONE. `cargo test` runs the
/// test BINARIES in parallel and libtest runs the tests inside one binary on parallel threads, so
/// a directory emptied on entry is only safe if nothing else can be inside it. Two call sites
/// cannot share a line number, so the directories are distinct BY CONSTRUCTION.
///
/// ⛔ AND THE PREFIX IS UNIQUE TO THIS FILE, which is the half a line number alone does not cover:
/// line 40 of this file and line 40 of `file_journal.rs`, `journal_contract_real.rs` or
/// `file_custody.rs` would name the SAME directory, and those binaries run at the same time. No
/// other bench may share this prefix; `grep -rhoE 'daemon-[a-z-]+' crates/ | sort -u` lists the
/// ones in use.
fn private_dir_for_line(line: u32) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("daemon-custody-contract-{line}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create the test directory");
    dir
}
