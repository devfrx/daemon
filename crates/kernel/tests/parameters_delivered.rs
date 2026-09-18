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

use kernel::arbiter::{ArbiterId, Mib};
use kernel::parameters::Parameters;
use kernel::time::Millis;

/// A literal of this bench, and the value is arbitrary on purpose: nothing here admits
/// anything, and no probe below depends on the number being plausible.
const TOTAL_VRAM: Mib = Mib::new(16_384);

/// The gui tick this bench delivers. ⚠️ A LITERAL OF THIS BENCH, and it is inert here on
/// purpose: nothing in this file runs `kernel::serving::serve`, so nobody reads it -- but
/// `Parameters` carries every delivered value positionally, and §2.8.2 rule 2 forbids the kernel
/// to name a default.
///
/// ⛔ ZERO IS NOT A NEUTRAL VALUE WHERE IT IS READ: a zero tick makes `kernel::executor::nap`
/// behave as a yield (`Sleep::until`'s own rule), so a bench that really serves hands its own --
/// `crates/kernel/tests/serving.rs` does.
const GUI_TICK: Millis = Millis::new(0);

#[test]
fn the_value_carries_the_resolved_parameters() {
    let parameters = Parameters::new(10_000, TOTAL_VRAM, ArbiterId::new(1), GUI_TICK);
    assert_eq!(parameters.executor_turn_limit(), 10_000);
    assert_eq!(parameters.total_vram(), TOTAL_VRAM);
    assert_eq!(parameters.gui_tick(), GUI_TICK);
}

#[test]
fn parameters_are_comparable_so_a_substitution_is_observable() {
    // §2.8.2 rule 4: substituting a parameter is a journalled step. Before it can be
    // journalled, "it changed" has to be expressible.
    assert_ne!(
        Parameters::new(10_000, TOTAL_VRAM, ArbiterId::new(1), GUI_TICK),
        Parameters::new(20_000, TOTAL_VRAM, ArbiterId::new(1), GUI_TICK)
    );
    // And the same for the second delivered value, differing in IT ALONE. Without this
    // line a comparison that looked only at `executor_turn_limit` would pass every probe
    // in this file, and substituting a total would be unobservable — the very thing rule 4
    // needs expressible.
    assert_ne!(
        Parameters::new(10_000, TOTAL_VRAM, ArbiterId::new(1), GUI_TICK),
        Parameters::new(10_000, Mib::new(8_192), ArbiterId::new(1), GUI_TICK)
    );
    // And the third, for the same reason again -- differing in the IDENTITY alone.
    // ✅ Measured 2026-08-30: with `PartialEq` written by hand over the other two fields
    // only, the whole workspace stayed green without this line. "Substituting the arbiter
    // this decision belongs to is observable" was held by nothing.
    assert_ne!(
        Parameters::new(10_000, TOTAL_VRAM, ArbiterId::new(1), GUI_TICK),
        Parameters::new(10_000, TOTAL_VRAM, ArbiterId::new(2), GUI_TICK)
    );
    // And the fourth, differing in THE TICK alone -- the value sub-project 2 delivered last.
    // Without this line a comparison written by hand over the other three would pass every
    // probe in this file, which is exactly what the measurement above found for the identity.
    assert_ne!(
        Parameters::new(10_000, TOTAL_VRAM, ArbiterId::new(1), Millis::new(0)),
        Parameters::new(10_000, TOTAL_VRAM, ArbiterId::new(1), Millis::new(50))
    );
}

#[test]
fn equal_parameters_do_not_report_a_substitution_that_never_happened() {
    // The other direction of the test above, and it is the one that gets forgotten
    // (§7.1.1, rule 3). On its own `assert_ne!` is satisfied by a comparison that answers
    // "different" to everything — which would journal a substitution at every step. A
    // check that fires where it must not is worse than one that is absent: gotcha #24.
    assert_eq!(
        Parameters::new(10_000, TOTAL_VRAM, ArbiterId::new(1), GUI_TICK),
        Parameters::new(10_000, TOTAL_VRAM, ArbiterId::new(1), GUI_TICK)
    );
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
    assert_eq!(
        Parameters::new(0, TOTAL_VRAM, ArbiterId::new(1), GUI_TICK).executor_turn_limit(),
        0
    );
    assert_eq!(
        Parameters::new(u64::MAX, TOTAL_VRAM, ArbiterId::new(1), GUI_TICK).executor_turn_limit(),
        u64::MAX
    );
}

#[test]
fn the_constructor_substitutes_nothing_for_the_total_it_is_handed() {
    // The other half of the test above, for the second delivered value — and it is a HALF
    // and not a repetition: the rule of §2.8.4 is about the CONSTRUCTOR, and a constructor
    // that leaves one field alone and quietly floors the other satisfies the first probe
    // and violates the rule. `total_vram` is the likelier of the two to tempt a guard,
    // because "a machine with zero VRAM is absurd" reads like a reason to write one.
    //
    // ⛔ It is not. A floor here would be a number CHOSEN INSIDE THE KERNEL — gotcha #28,
    // on no list, firing no check — and it would hide precisely the configuration error
    // §5.1 declares as the cost of DELIVERING the total instead of asking for it: a wrong
    // total must show up as over-admission that can be traced to the parameter, never as a
    // budget the kernel invented.
    assert_eq!(
        Parameters::new(10_000, Mib::ZERO, ArbiterId::new(1), GUI_TICK).total_vram(),
        Mib::ZERO
    );
    assert_eq!(
        Parameters::new(10_000, Mib::new(u64::MAX), ArbiterId::new(1), GUI_TICK).total_vram(),
        Mib::new(u64::MAX)
    );
}

#[test]
fn the_arbiter_identity_is_delivered_and_not_invented() {
    // ⛔ THE POINT IS THE ABSENCE OF A DEFAULT. §6.1.3 forbids the kernel to MINT an
    // identifier, and ADR-0034 says a decision reads only what it was handed: an arbiter
    // that chose its own id would be doing both.
    //
    // ⛔ TWO VALUES AND NOT ONE, for the reason the sisters above carry: a reader that
    // answered a CONSTANT would satisfy a single-value probe, so one value tests the
    // constructor's arity and not what it delivers.
    assert_eq!(
        Parameters::new(64, Mib::new(8_192), ArbiterId::new(7), GUI_TICK).arbiter_id(),
        ArbiterId::new(7)
    );
    assert_eq!(
        Parameters::new(64, Mib::new(8_192), ArbiterId::new(u64::MAX), GUI_TICK).arbiter_id(),
        ArbiterId::new(u64::MAX)
    );
}

#[test]
fn the_constructor_substitutes_nothing_for_the_tick_it_is_handed() {
    // The same half once more, for the value delivered last -- and of the four it is the one
    // a guard would look most defensible on: a ZERO tick makes `kernel::executor::nap` behave
    // as a yield (`Sleep::until`'s own rule), so `if gui_tick == 0 { … }` reads like prudence.
    //
    // ⛔ It is gotcha #28 all the same. The number would be chosen INSIDE the kernel -- on no
    // list, firing no check -- and a campaign that wanted a core which never waits could not
    // ask for one, which is the single thing §2.8.4's declared limit leaves to a test.
    assert_eq!(
        Parameters::new(10_000, TOTAL_VRAM, ArbiterId::new(1), Millis::new(0)).gui_tick(),
        Millis::new(0)
    );
    assert_eq!(
        Parameters::new(10_000, TOTAL_VRAM, ArbiterId::new(1), Millis::new(50)).gui_tick(),
        Millis::new(50)
    );
}
