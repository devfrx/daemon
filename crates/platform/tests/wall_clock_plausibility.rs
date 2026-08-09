//! `SystemReactor::wall_time()` reads a PLAUSIBLE instant — the one thing about the real wall
//! clock that can be checked without a second source of time, and without flakiness.
//!
//! ⛔ WHY IT IS NOT IN THE CONFORMANCE SUITE. `crates/kernel/tests/reactor_contract.rs` holds
//! what THE PORT promises and runs against both implementations, and the port promises nothing
//! about wall time that a real system clock could be held to: assertion 5 there deliberately
//! proves only that `wall_time()` is callable. The hole that leaves is an IMPLEMENTATION's
//! hole, so it is closed against the implementation. The fake's own wall-clock behaviour is
//! pinned in the same spirit, and just as separately, in `crates/simulator/tests/virtual_clock.rs`.
//!
//! ⚠️ AND WHY IT IS AN INTEGRATION TEST rather than a `#[cfg(test)] mod tests` in `src/`: it
//! needs nothing private. The repository puts its tests in `tests/`, and the single unit-test
//! module in this crate — in `src/rng.rs` — is there only because it reaches a private field.

use kernel::ports::reactor::Reactor;
use platform::reactor::SystemReactor;

/// 2020-01-01T00:00:00Z, in milliseconds since the Unix epoch.
///
/// ⛔ A DATE ALREADY GONE, and that is the entire reason it can serve as a bound: an instant
/// fixed in the past stays true for ever FORWARDS, so this constant never ages into a false red
/// the way a bound near the present would. Comparing against "now" instead would be a test
/// measuring itself — it would read the very clock it is checking and agree with whatever that
/// clock happened to say, including zero.
const A_DATE_ALREADY_PAST_MS: u64 = 1_577_836_800_000;

#[test]
fn wall_time_reads_a_plausible_instant() {
    // ⚠️ WHAT THIS BUYS, exactly: it catches a wall clock STUCK AT ZERO OR AT THE EPOCH. That
    // is the mutation that survived everything else — replacing the whole body of `wall_time`
    // with `WallTime::from_millis_since_epoch(0)` left the entire suite green — and it is also
    // what the `unwrap_or(0)` branch yields on a machine whose clock sits before the epoch.
    //
    // ⛔ AND WHAT IT DOES NOT: it says NOTHING about the value being CORRECT. Establishing that
    // would need a second, independent source of time, and there is none here. It is a
    // plausibility bound, not a measurement.
    let reactor = SystemReactor::new();
    let stamp = reactor.wall_time().as_millis_since_epoch();
    assert!(
        stamp > A_DATE_ALREADY_PAST_MS,
        "wall_time() gave {stamp} ms since the epoch, which is not even after 2020-01-01 \
         ({A_DATE_ALREADY_PAST_MS} ms): the clock is stuck at zero or at the epoch"
    );
}
