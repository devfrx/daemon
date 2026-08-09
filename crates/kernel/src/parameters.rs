//! The resolved decision parameters, DELIVERED to the kernel at construction (§2.8,
//! ADR-0034).
//!
//! > No kernel decision reads a parameter that was not delivered to it.
//!
//! A parameter that is not delivered ends up as a CONSTANT written inside the kernel, and
//! a constant is the worst violation possible here because it is INVISIBLE: it appears in
//! no list, fires no check in the §7 catalogue, and shows up only when somebody tries to
//! make it vary in a campaign and cannot. Gotcha #28.
//!
//! ⛔ NEGATIVE PERIMETER — this is NOT a configuration system. No format, no schema, no
//! validation, no hot reload. It is NOT a string-keyed registry, which would put the
//! kernel back in the position of ASKING. And it does not decide the format of any store.
//! The full list is in ADR-0034.
//!
//! Who PRODUCES this value is `daemon`: from the store via `platform` in production, from
//! the test bench in simulation. In sub-project 1 the defaults are LITERALS IN `daemon` —
//! the correct boundary, and written down rather than hidden.
//!
//! ⚠️ Adding a parameter changes the signature of `new`, and every caller breaks. That
//! friction is the point: §2.8.5 declares it, and it is what stops a parameter from
//! quietly re-entering as a constant.

/// The parameters the kernel has been configured with.
///
/// ⛔ THERE IS NO `Default` IMPL, and that is the decision rather than an omission. A
/// default is one of the three things §2.8.2 rule 2 says the kernel cannot name — file,
/// key, default — and `impl Default for Parameters` would name one: the value would be
/// CHOSEN HERE instead of being delivered, which is the violation the whole section
/// exists to remove.
///
/// ⛔ And the ban is not left to this comment. `tests/compile_fail/parameters_have_no_default.rs`
/// calls `Parameters::default()`: today it does not compile, and the day somebody writes
/// the impl it starts COMPILING, which trybuild reports outright instead of through its
/// oracle. That is the `error` shape, and a bulk regeneration of the `.stderr` files
/// cannot disarm it — gotcha #42.
///
/// ⚠️ Declared limit, so this comment promises no more than it delivers: the guard covers
/// the `Default` route only. A fallback written inside `new` is a default too, and the
/// compiler cannot forbid it — §2.8.4 says so outright. That hole is held by a test,
/// `the_constructor_substitutes_nothing_for_the_value_it_is_handed`, which is level 2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Parameters {
    executor_turn_limit: u64,
}

impl Parameters {
    /// Every field, positionally.
    ///
    /// ⛔ It substitutes NOTHING for what it is handed: no clamp, no floor, no fallback on
    /// a value it dislikes. A guard along the lines of `if executor_turn_limit == 0 { … }`
    /// would put a number chosen inside the kernel just as surely as an `impl Default`
    /// would, and it is cheaper to write. Validation is outside the perimeter above for
    /// this reason, not for laziness.
    pub const fn new(executor_turn_limit: u64) -> Self {
        Parameters {
            executor_turn_limit,
        }
    }

    /// How many turns the executor may take before declaring a block.
    ///
    /// A block must show up as an error, never as an infinite wait: a test that never ends
    /// says nothing (§3.2.1).
    pub const fn executor_turn_limit(self) -> u64 {
        self.executor_turn_limit
    }
}
