//! The virtual clock's OWN behaviour: the two clocks move TOGETHER, by the same amount (§3.2).
//!
//! # ⛔ Why these tests live here and NOT in the conformance suite
//!
//! `crates/kernel/tests/reactor_contract.rs` holds every promise THE PORT makes, and it is run
//! against both implementations. What is pinned here is not one of those promises: it is a
//! property of THE FAKE, and the real implementation does not have it and must not be asked to.
//! `SystemReactor` serves `wall_time` from the system clock, which NTP, daylight saving or the
//! user can step backwards at any moment — so the very assertion below would be flaky against
//! the real reactor, and moving it into the shared suite would break a correct implementation.
//!
//! The conformance suite says so in its own words at assertion 5, where it deliberately proves
//! only that `wall_time()` is callable. THIS FILE IS THE OTHER HALF of that sentence: the
//! behaviour conformance cannot state still gets pinned, just not there.
//!
//! # What the behaviour is for
//!
//! A virtual wall clock frozen at the origin while the monotonic one jumps twenty seconds would
//! hand the journal a stamp that CONTRADICTS ITS OWN ORDERING — every record of a simulated run
//! carrying the same instant, in an order the stamps deny. §2.1 forbids a DECISION from reading
//! wall time; it does not excuse the record from being consistent with it.
//!
//! ⚠️ THAT READER IS §2.1's DESIGN AND NOT TODAY'S RECORD: no record carries a stamp yet, as
//! `Reactor::wall_time` says, so what these tests keep today is a fake consistent with itself,
//! ready for the reader that will come. ⚠️ RECALL OF 2026-10-13 -- audit of 2026-09-30, AUD-1434.

use kernel::ports::reactor::Reactor;
use kernel::time::{Millis, Monotonic};
use simulator::reactor::VirtualReactor;

#[test]
fn a_successful_wait_moves_both_clocks_by_the_same_amount() {
    let mut reactor = VirtualReactor::new();
    let before_monotonic = reactor.now();
    let before_wall = reactor.wall_time().as_millis_since_epoch();

    let deadline = before_monotonic.saturating_add(Millis::new(20_000));
    assert_eq!(reactor.wait_until(deadline), Some(deadline));

    // The DELTAS are compared, not the absolute values: the property is "the same amount",
    // and the two origins coinciding at zero is a fact about this fake's construction rather
    // than the thing under test.
    let monotonic_moved = reactor.now().saturating_since(before_monotonic);
    // ⚠️ `saturating_sub` AND NOT A BARE `-`, TO PROTECT THE DIAGNOSTIC RATHER THAN THE RED. If
    // the wall clock ever moved BACKWARDS the bare subtraction would panic with "attempt to
    // subtract with overflow" in a debug build, and in `--release` would wrap and print a
    // nonsense figure. The test fails either way — what would be lost is the constructed
    // message below, which is the part that says WHAT went wrong. Saturating to zero keeps it.
    let wall_moved = reactor
        .wall_time()
        .as_millis_since_epoch()
        .saturating_sub(before_wall);
    assert_eq!(
        monotonic_moved,
        Millis::new(20_000),
        "the monotonic clock did not reach the deadline"
    );
    assert_eq!(
        wall_moved,
        monotonic_moved.get(),
        "the two clocks parted company: monotonic moved {} ms, wall moved {wall_moved} ms",
        monotonic_moved.get()
    );
}

#[test]
fn a_refused_wait_moves_neither_clock() {
    let mut reactor = VirtualReactor::new();

    // A real advance first, so that an instant STRICTLY IN THE PAST exists to ask for at all:
    // `Monotonic::ORIGIN` is the bottom of the scale, and a never-advanced reactor sits on it.
    reactor
        .wait_until(Monotonic::from_millis(5_000))
        .expect("the setup did not advance, so this test would be vacuous");

    let monotonic = reactor.now();
    let wall = reactor.wall_time();

    // Both halves of the `<=` branch, for the same reason the conformance suite spends two
    // cases on it: checking only the equal one leaves the other half unvisited.
    assert_eq!(reactor.wait_until(monotonic), None, "deadline == now");
    assert_eq!(
        reactor.wait_until(Monotonic::ORIGIN),
        None,
        "deadline < now"
    );

    assert_eq!(
        reactor.now(),
        monotonic,
        "a refused wait moved the monotonic clock"
    );
    assert_eq!(
        reactor.wall_time(),
        wall,
        "a refused wait moved the wall clock"
    );
}

#[test]
fn consecutive_waits_accumulate_on_both_clocks() {
    // Neither clock restarts, and they stay in step across several waits — a per-wait reset
    // would satisfy the single-wait test above and still lose the total.
    //
    // ⚠️ THE TOTALS ARE DELTAS FROM WHERE EACH CLOCK STARTED, for the reason the first test of
    // this file gives: the two origins coinciding at zero is a fact about this fake's
    // construction, not the thing under test. Measured on 2026-10-13: with the fake's wall
    // origin fixed at another value this test stays green, and with a per-wait reset it goes
    // red (⚠️ RECALL OF 2026-10-13 -- audit of 2026-09-30, AUD-2175).
    let mut reactor = VirtualReactor::new();
    let start_monotonic = reactor.now();
    let start_wall = reactor.wall_time().as_millis_since_epoch();
    for deadline in [1_000, 1_500, 9_000] {
        reactor
            .wait_until(Monotonic::from_millis(deadline))
            .expect("each of these deadlines is strictly ahead of the one before");
    }

    let monotonic_total = reactor.now().saturating_since(start_monotonic);
    let wall_total = reactor
        .wall_time()
        .as_millis_since_epoch()
        .saturating_sub(start_wall);
    assert_eq!(
        monotonic_total,
        Millis::new(9_000),
        "the monotonic clock lost the total"
    );
    assert_eq!(
        wall_total,
        monotonic_total.get(),
        "the wall clock lost the total: monotonic moved {} ms in all, wall moved {wall_total} ms",
        monotonic_total.get()
    );
}
