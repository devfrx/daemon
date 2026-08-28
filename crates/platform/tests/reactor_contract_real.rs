// THE SAME CONFORMANCE SUITE, RUN AGAINST THE REAL REACTOR (§7.4.6).
//
// ⛔ THE ASSERTIONS ARE NOT REPEATED HERE, and that is the whole point of this file being three
// lines long. They live in ONE place — `crates/kernel/tests/reactor_contract.rs` — and are
// reached from here textually, because two copies would drift apart and THE FIRST ONE TO DRIFT
// WOULD LIE IN SILENCE: a suite that no longer compares the two implementations still prints
// `ok`. What makes the deterministic simulation worth anything is precisely that the fake and
// the real one answer the same questions, so the questions must be one set of words.
//
// `include!` is the mechanism because an integration test is A CRATE OF ITS OWN: it cannot
// `use` the items of another test target, so there is no import that would do this. The path is
// relative to this file's directory, which is why it climbs out of `crates/platform/tests/`.
//
// ⚠️ DECLARED COST, and it is accepted rather than unnoticed: `include!` brings the included
// file's `#[test]` functions along with it, so EVERY `#[test]` OF `reactor_contract.rs` RUNS A
// SECOND TIME in this binary. None of them sleeps -- a liar is a fake clock and only the real
// implementation waits -- so it costs a few milliseconds, and it buys the single copy of the
// assertions, which is not a trade worth reversing. ⛔ HOW MANY THEY ARE IS NOT WRITTEN HERE: the
// included file owns that figure and states it beside the tests themselves;
// `grep -c '^#\[test\]' crates/kernel/tests/reactor_contract.rs` answered 8 on 2026-08-28.
//
// ⛔ RECALL OF 2026-08-28, FINDINGS AUD-030 AND AUD-059 (the same defect, filed twice) -- THIS
// NAMED TWO TESTS BY NAME, `the_fake_reactor_honours_the_contract` and
// `a_reactor_that_lies_about_a_null_advance_is_caught`, AND THERE WERE EIGHT. Finding B-2 brought
// five liars more on 2026-08-18 and this file was never reopened: whoever adds a test edits the
// file that HOSTS it, not the one that RUNS IT A SECOND TIME. ⛔ THE NAMES ARE REMOVED RATHER
// THAN EXTENDED TO EIGHT, and the count with them: this is the SECOND house of a figure whose
// first is the included file, and realigning it would leave the rule as the only defence again --
// gotcha #68, and the cure AUD-009 applied to the gate's `cargo` sites. What stays is the
// RELATION, which holds however many tests arrive. ⚠️ The clause "the fake and the liar, neither
// of which sleeps" was stale in BOTH halves for the same reason, and is replaced by the reason
// the included file already gives: a liar is a fake clock.
//
// ⚖️ REGISTERED AND NOT TAKEN: the twin pair carries the same shape -- `journal_contract_real.rs`
// states the suite's size that `journal_contract.rs` also owns, and has been RECOUNTED three
// times instead of unlinked. It is CORRECT today, so it is not touched here; whether the twin
// should lose its copy too is one edit on a file this finding does not name.

include!("../../kernel/tests/reactor_contract.rs");

#[test]
fn the_real_reactor_honours_the_contract() {
    // ⚠️ THIS ONE REALLY SLEEPS — three waits of `CONTRACT_MARGIN_MS` each, so about 150 ms.
    // That is the price of measuring the actual port instead of a description of it.
    assert_reactor_contract(platform::reactor::SystemReactor::new);
}
