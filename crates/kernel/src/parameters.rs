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

use crate::arbiter::Mib;

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
    total_vram: Mib,
}

impl Parameters {
    /// Every field, positionally.
    ///
    /// ⛔ It substitutes NOTHING for what it is handed: no clamp, no floor, no fallback on
    /// a value it dislikes. A guard along the lines of `if executor_turn_limit == 0 { … }`
    /// would put a number chosen inside the kernel just as surely as an `impl Default`
    /// would, and it is cheaper to write. Validation is outside the perimeter above for
    /// this reason, not for laziness.
    pub const fn new(executor_turn_limit: u64, total_vram: Mib) -> Self {
        Parameters {
            executor_turn_limit,
            total_vram,
        }
    }

    /// How many turns the executor may take before declaring a block.
    ///
    /// A block must show up as an error, never as an infinite wait: a test that never ends
    /// says nothing (§3.2.1).
    pub const fn executor_turn_limit(self) -> u64 {
        self.executor_turn_limit
    }

    /// How much VRAM the machine has, in whole MiB.
    ///
    /// ⛔ IT IS DELIVERED AND NOT ASKED FOR, and §5.1 spent a dated recall on exactly this:
    /// the formula for the allocatable budget appears identically in three documents and
    /// NONE of them said where `total` comes from. Querying the GPU is an OS call, which I3
    /// forbids the kernel, and none of the six port families supplies hardware capacity. So
    /// it is DECLARED, like the reservation of ADR-0005, and a systematic discrepancy is a
    /// defect of the PARAMETER rather than an accident.
    ///
    /// ⚠️ THE COST, DECLARED BY §5.1 ITSELF: a wrong total produces over-admission -- Q2
    /// giving way through a configuration error rather than a code one. The mitigation is
    /// the measured peak of §5.2.2, not an a-priori check that does not exist here.
    ///
    /// ⛔ IT IS THE ONLY ONE OF THE THREE ADDENDS THAT IS DELIVERED, and that is a declared
    /// divergence from the letter of §5.1 rather than an omission. The audio quota and the
    /// presentation quota are NOT subtracted here: they are the reservations of two
    /// PERMANENT GRANTS asked for by the composition root. A subtraction without a holder
    /// leaves I2 false for those two consumers -- "the subtraction is not an exemption",
    /// ADR-0005 and gotcha #4 -- and two fields no kernel decision reads would be dead
    /// surface inside the kernel.
    pub const fn total_vram(self) -> Mib {
        self.total_vram
    }
}
