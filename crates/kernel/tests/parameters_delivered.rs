//! The resolved parameters are DELIVERED (§2.8, ADR-0034).
//!
//! The probe that must fire is in `tests/compile_fail/executor_without_parameters.rs`,
//! written in Task 5 when there is an executor to build. This file is the other
//! direction: the value exists, carries what it says it carries, and nothing in the
//! kernel can name a file, a key or a default in order to obtain it.
//!
//! ⛔ The `default` half of that last clause has its own case next door —
//! `tests/compile_fail/parameters_have_no_default.rs` — because a `Default` impl is not
//! the only way a default gets in, and the two ways need two different guards. The
//! cheaper way in is a fallback written inside the constructor, and the compiler cannot
//! forbid it (§2.8.4, the declared limit). That one is pinned here, by the last test.

use kernel::parameters::Parameters;

#[test]
fn the_value_carries_the_resolved_parameters() {
    let parameters = Parameters::new(10_000);
    assert_eq!(parameters.executor_turn_limit(), 10_000);
}

#[test]
fn parameters_are_comparable_so_a_substitution_is_observable() {
    // §2.8.2 rule 4: substituting a parameter is a journalled step. Before it can be
    // journalled, "it changed" has to be expressible.
    assert_ne!(Parameters::new(10_000), Parameters::new(20_000));
}

#[test]
fn equal_parameters_do_not_report_a_substitution_that_never_happened() {
    // The other direction of the test above, and it is the one that gets forgotten
    // (§7.1.1, rule 3). On its own `assert_ne!` is satisfied by a comparison that answers
    // "different" to everything — which would journal a substitution at every step. A
    // check that fires where it must not is worse than one that is absent: gotcha #24.
    assert_eq!(Parameters::new(10_000), Parameters::new(10_000));
}

#[test]
fn the_constructor_substitutes_nothing_for_the_value_it_is_handed() {
    // §2.8.2 rule 2: the kernel cannot name a default. A default does not only arrive as
    // an `impl Default` — the cheaper way in is a guard inside the constructor, along the
    // lines of `if executor_turn_limit == 0 { … }`, and zero is the value most likely to
    // tempt one into writing it. That number would be a constant chosen INSIDE the
    // kernel, which is gotcha #28 to the letter: on no list, firing no check, and visible
    // only when a campaign tries to make it vary and cannot.
    //
    // The compile-fail case forbids the `Default` route; nothing but this test covers the
    // inline one, and §2.8.4 says outright that the compiler cannot.
    assert_eq!(Parameters::new(0).executor_turn_limit(), 0);
    assert_eq!(Parameters::new(u64::MAX).executor_turn_limit(), u64::MAX);
}
