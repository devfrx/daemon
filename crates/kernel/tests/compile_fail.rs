//! The compile-fail tests: the level 1 rules, seen firing.
//!
//! ⛔ GOTCHA #25 — THE `.stderr` FILES ARE NOT REGENERATED IN BULK.
//! `trybuild` offers `TRYBUILD=overwrite` to rewrite them all over the current output.
//! It is needed when the compiler messages change legitimately. Used without reading
//! them, every case becomes "the expected error is whatever came out" and the suite
//! passes forever. Regeneration is a deliberate act and **is read in the diff**.
//!
//! ⚠️ A compile-fail test has level 1 force and level 2 visibility (§7.1.3): deleting it
//! does NOT reopen the violation, it makes it invisible.

#[test]
fn level_1_rules_do_not_compile() {
    non_vacuity_guard();
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
}

/// ⛔ The bench with NO cases must be red. Without this guard it is GREEN, and that is
/// measured on `trybuild` 1.0.120 — not deduced:
///
/// - `expand.rs:20` — with a pattern containing `*`, `glob()` returns `Err` **only** if
///   it is the pattern that is malformed, not if it matches nothing: zero matches give
///   zero expanded cases;
/// - `run.rs:74` — with zero cases it prints a yellow warning, leaves `failures` at zero
///   and raises nothing. That is, it exits green.
///
/// ⚠️ The asymmetry that makes the guard necessary: a **literal** path that does not
/// exist does turn red, because it goes through `check_exists`. Only the glob swallows
/// the void, and nobody reconstructs that by reading `t.compile_fail(...)`.
///
/// ⛔ No expected number, for the reason of §8.6.2: a fixed count would turn red the day
/// the bench grows for a legitimate reason — gotcha #9 applied to the guard. What is
/// checked is that the cases are **more than zero**.
fn non_vacuity_guard() {
    let dir = "tests/compile_fail";
    let cases = match std::fs::read_dir(dir) {
        Ok(entries) => entries
            .flatten()
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "rs"))
            .count(),
        // A directory that does not open is worth zero cases: the two ways of being empty
        // are the same fault, and the message below names them both.
        Err(_) => 0,
    };
    assert!(
        cases > 0,
        "empty bench: `{dir}/` does not exist or contains no `.rs`. It is not \"nothing \
         to do\": it is the level 1 gate that is proving NOTHING, and without this guard \
         it would come out GREEN."
    );
}
