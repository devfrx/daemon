//! What the compiler cannot hold about the resource model: the DIRECTION in which the
//! arithmetic saturates, and the explicit lane order.

use kernel::arbiter::Mib;

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
