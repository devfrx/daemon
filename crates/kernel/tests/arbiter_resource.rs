//! What the compiler cannot hold about the resource model: the DIRECTION in which the
//! arithmetic saturates, and the explicit lane order.

use kernel::arbiter::{ComputeClass, Mib, Preemption};
use kernel::time::Millis;

/// ⛔ THE DIRECTION IS THE ASSERTION, not the fact that it does not panic. A wrapping add
/// would give a SMALLER number than the ceiling and admit a request that does not fit.
///
/// ⚠️ TWO VALUES AND NOT ONE (gotcha #48): a single pair can agree with the mutation by
/// accident. The second pair overflows by a different amount.
#[test]
fn an_overflowing_sum_saturates_upwards_so_a_request_is_refused() {
    let ceiling = Mib::new(16_384);

    let first = Mib::new(u64::MAX).saturating_add(Mib::new(1));
    assert_eq!(first, Mib::new(u64::MAX));
    assert!(
        first > ceiling,
        "a wrapped sum would land BELOW the ceiling and be admitted"
    );

    let second = Mib::new(u64::MAX - 5).saturating_add(Mib::new(9));
    assert_eq!(second, Mib::new(u64::MAX));
    assert!(second > ceiling);
}

/// The floor, and its own failure: a wrapped subtraction would give some 18 quintillion
/// MiB of free budget -- over-admission by the other road.
#[test]
fn a_subtraction_below_zero_saturates_to_zero_and_not_to_an_enormous_budget() {
    assert_eq!(Mib::new(3).saturating_sub(Mib::new(4)), Mib::ZERO);
    assert_eq!(Mib::ZERO.saturating_sub(Mib::new(1)), Mib::ZERO);
}

/// The ordinary path, so the two probes above are not the only thing this type is held by.
#[test]
fn the_ordinary_arithmetic_is_exact() {
    assert_eq!(Mib::new(4096).saturating_add(Mib::new(2048)), Mib::new(6144));
    assert_eq!(Mib::new(4096).saturating_sub(Mib::new(2048)), Mib::new(2048));
    assert_eq!(Mib::new(4096).get(), 4096);
}

/// ⛔ THE ORDER IS FIXED BY NAME, and what that buys is worth stating. `Ord` DERIVED
/// follows DECLARATION order, so reordering the variants -- a tidy-up, a rename, an
/// alphabetisation -- would silently change the arbiter's priorities and NOTHING WOULD GO
/// RED. The order lives in an explicit key (`priority`), and this probe pins it by name.
///
/// ⚠️ SO REORDERING THE VARIANTS LEAVES THIS PROBE GREEN, deliberately: the trap has been
/// REMOVED rather than watched. What turns it red is changing the key -- which is the only
/// place the order is stated.
#[test]
fn the_lane_order_is_pinned_by_name_and_realtime_comes_first() {
    assert!(ComputeClass::Realtime < ComputeClass::Interactive);
    assert!(ComputeClass::Interactive < ComputeClass::Batch);
    assert!(ComputeClass::Realtime < ComputeClass::Batch);

    // The key itself, so a reader does not have to infer it from three inequalities.
    assert_eq!(ComputeClass::Realtime.priority(), 0);
    assert_eq!(ComputeClass::Interactive.priority(), 1);
    assert_eq!(ComputeClass::Batch.priority(), 2);
}

/// ⛔ THE COUNTER-PROBE OF THE ONE ABOVE, and it is the half that is easy to skip: the
/// ordering has to be TOTAL, so a `BTreeMap` keyed on a lane gets one bucket per lane and
/// not two that compare equal.
#[test]
fn the_three_lanes_are_distinct_and_the_ordering_is_total() {
    let lanes = [
        ComputeClass::Realtime,
        ComputeClass::Interactive,
        ComputeClass::Batch,
    ];
    for (index, left) in lanes.iter().enumerate() {
        for (other, right) in lanes.iter().enumerate() {
            assert_eq!(index == other, left == right);
            assert_eq!(index.cmp(&other), left.cmp(right));
        }
    }
}

/// ⛔ WHAT THE TYPE MAKES UNSAYABLE, and it is TWO illegal states and not one: a
/// non-preemptible profile CANNOT CARRY a grace time, and a preemptible one CANNOT LACK
/// one. A boolean plus a separate duration expresses both.
#[test]
fn a_grace_time_exists_exactly_when_the_profile_is_preemptible() {
    assert_eq!(Preemption::Never.grace(), None);
    assert_eq!(
        Preemption::After(Millis::new(250)).grace(),
        Some(Millis::new(250))
    );
}
