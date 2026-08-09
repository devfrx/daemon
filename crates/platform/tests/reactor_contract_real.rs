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
// file's `#[test]` functions along with it, so `the_fake_reactor_honours_the_contract` and
// `a_reactor_that_lies_about_a_null_advance_is_caught` RUN A SECOND TIME in this binary. They
// exercise the fake and the liar, neither of which sleeps, so it costs a few milliseconds — and
// it buys the single copy of the assertions, which is not a trade worth reversing.

include!("../../kernel/tests/reactor_contract.rs");

#[test]
fn the_real_reactor_honours_the_contract() {
    // ⚠️ THIS ONE REALLY SLEEPS — three waits of `CONTRACT_MARGIN_MS` each, so about 150 ms.
    // That is the price of measuring the actual port instead of a description of it.
    assert_reactor_contract(platform::reactor::SystemReactor::new);
}
