//! Counter-probes for the three time types (§2.1).
//!
//! The probes that must FIRE live in `tests/compile_fail/monotonic_as_wall.rs` and
//! `tests/compile_fail/wall_as_monotonic.rs`: these are the other direction — the one
//! that is forgotten (§7.1.1, rule 3).

use kernel::time::{Millis, Monotonic, WallTime};

#[test]
fn a_deadline_is_an_instant_plus_a_duration() {
    let start = Monotonic::from_millis(1_000);
    let deadline = start.saturating_add(Millis::new(5_000));
    assert_eq!(deadline, Monotonic::from_millis(6_000));
}

#[test]
fn monotonic_never_goes_backwards_even_on_overflow() {
    // Pins the ceiling: at the top of the scale the add is a no-op, not a wrap. Why it
    // must be so is written on `saturating_add` itself.
    let late = Monotonic::from_millis(u64::MAX);
    assert_eq!(late.saturating_add(Millis::new(1)), late);
}

#[test]
fn the_last_add_before_the_ceiling_does_not_saturate() {
    // The boundary from below: the ceiling test above passes even if saturation started
    // far too early, and nothing else here would notice.
    let almost = Monotonic::from_millis(u64::MAX - 1);
    assert_eq!(
        almost.saturating_add(Millis::new(1)),
        Monotonic::from_millis(u64::MAX)
    );
}

#[test]
fn the_distance_between_two_instants_is_a_duration() {
    let earlier = Monotonic::from_millis(1_000);
    let later = Monotonic::from_millis(4_500);
    assert_eq!(later.saturating_since(earlier), Millis::new(3_500));
    // Backwards: saturates to zero rather than underflowing.
    assert_eq!(earlier.saturating_since(later), Millis::new(0));
}

#[test]
fn the_origin_is_the_bottom_of_the_monotonic_scale() {
    assert_eq!(Monotonic::ORIGIN, Monotonic::from_millis(0));
    assert!(Monotonic::ORIGIN <= Monotonic::from_millis(1));
}

#[test]
fn wall_time_round_trips_the_epoch_in_milliseconds() {
    let stamp = WallTime::from_millis_since_epoch(1_775_000_000_000);
    assert_eq!(stamp.as_millis_since_epoch(), 1_775_000_000_000);
}

#[test]
fn instants_of_the_same_kind_compare() {
    assert!(Monotonic::from_millis(1) < Monotonic::from_millis(2));
    assert!(WallTime::from_millis_since_epoch(1) < WallTime::from_millis_since_epoch(2));
}
