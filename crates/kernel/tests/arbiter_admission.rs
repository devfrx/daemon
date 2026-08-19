//! What the compiler cannot hold about the GRANT CYCLE. Two things at task 4, and the
//! ADMISSION itself from task 5: the LEGAL side of `I2 · §5.3`, the positive side of `V4`
//! -- that `Admission` truly answers THREE ways -- and then that the sum of every grant
//! never exceeds the total, that releasing gives back EXACTLY the reservation, and that an
//! expired grant does not stay allocated.
//!
//! ⚠️ THE PROBES LIVE HERE AND NOT IN `arbiter_resource.rs`, and the split is by subject
//! rather than by convenience: that file holds the vocabulary of the RESOURCE -- `Mib`, the
//! three lanes, the grace time -- and neither `Activity` nor `Admission` is a resource.
//! `Activity` is what a held grant is DOING; `Admission` is the answer a request gets.
//!
//! ⚠️ AND THIS FILE ARRIVED ONE TASK EARLY. The plan's file table assigns it to tasks 5-7;
//! it was born at task 4 because two rules would otherwise have been held in only one
//! direction -- `tests/compile_fail/revoking_a_non_preemptible_grant.rs` proves only that the
//! ILLEGAL state cannot be spelled, and `tests/compile_fail/admission_is_not_two_ways.rs`
//! proves only that a two-armed match does not compile -- and a rule proved in one direction
//! only is not admissible (§7.1.1 rule 3). ⛔ TASK 5 ADDED TO THIS FILE rather than creating
//! it: its step said "create", the file was already here, and this module comment was MERGED
//! instead of overwritten -- otherwise the only thing holding the second direction of
//! `I2 · §5.3` would have disappeared. The same still goes for tasks 6 and 7.
//!
//! ⚠️ TWO DIFFERENT RULES ABOUT COMPARISON LIVE HERE, DELIBERATELY. R2 -- probes MATCH and
//! never compare -- is about `Admission`, which carries a `Grant` and therefore has neither
//! `Debug` nor `PartialEq`: giving them to it for the convenience of this file is the trade
//! `ports::process` refused, so every probe below uses `matches!` and `let … else`, never
//! `assert_eq!` on an `Admission`. `Activity` carries no grant and derives both, so
//! `assert_ne!` on it directly is fine and is what the first probe uses.

use kernel::arbiter::{
    Activity, Admission, Arbiter, ComputeClass, Mib, Preemption, PreemptibleState,
    ResourceProfile,
};
use kernel::parameters::Parameters;
use kernel::time::{Millis, Monotonic};

const TURN_LIMIT: u64 = 10_000;
const TOTAL: Mib = Mib::new(16_384);

fn profile(name: &'static str, vram: u64, lane: ComputeClass) -> ResourceProfile {
    ResourceProfile {
        name,
        reserved_vram: Mib::new(vram),
        compute_class: lane,
        preemption: Preemption::Never,
    }
}

fn arbiter(total: Mib) -> Arbiter {
    Arbiter::new(Parameters::new(TURN_LIMIT, total))
}

/// The window every probe in this file uses when the value does not matter.
const LONG: Millis = Millis::new(1_000_000);

/// The direction the compile-fail case CANNOT prove: the revocation is constructible
/// exactly where §5.3 point 3 allows it.
///
/// ⛔ WITHOUT THIS, `I2 · §5.3` WOULD BE HELD IN ONE DIRECTION BY A CASE AND IN THE OTHER BY
/// A MUTATION -- and a mutation disappears when it is reverted. A direction held by
/// something that does not stay is not held (§7.1.1 rule 3). It is also what tells the
/// negative case apart from an accident: it proves the compiler refuses the NESTING, not
/// the syntax of `Revoking { deadline }`.
#[test]
fn a_revocation_is_constructible_on_the_preemptible_side() {
    let revoking = Activity::Preemptible(PreemptibleState::Revoking {
        deadline: Monotonic::from_millis(1_000),
    });

    assert_ne!(
        revoking,
        Activity::Preemptible(PreemptibleState::Running),
        "a revocation that compared equal to a running grant would make the state useless"
    );
    assert_ne!(
        revoking,
        Activity::NonPreemptible,
        "the two sides of the nesting must not collapse into one another"
    );
}

/// The direction the compile-fail case CANNOT prove: `Admission` really is distinguishable
/// THREE ways, not two -- a `match` naming all three variants compiles from outside the
/// crate, which is exactly the shape `admission_is_not_two_ways.rs` proves a two-armed
/// match does NOT have.
///
/// ⛔ WITHOUT THIS, `V4`'s positive direction WOULD BE HELD BY A MUTATION -- add the third
/// arm to the negative case's `match` and watch it start compiling -- and a mutation
/// disappears when it is reverted. A direction held by something that does not stay is not
/// held (§7.1.1 rule 3).
///
/// ⚠️ ONLY `Refused` IS CONSTRUCTIBLE FROM OUT HERE. `Granted` carries a `Grant`, which has
/// no public constructor (`grant_has_no_constructor.rs`); `Queued` carries a `TicketId`,
/// whose only field is private too. The other two arms therefore bind and discard -- their
/// weight is that the match TYPE-CHECKS against all three shapes. R2-compliant: this
/// matches `Admission` instead of comparing it, exactly as the module comment requires.
///
/// 📌 THIS PROBE'S FORCE IS AT COMPILE TIME ONLY, and it is worth saying plainly instead of
/// implying otherwise. Nothing here computes anything -- it constructs the answer itself --
/// so the `Mib` values only ever echo what this test passed in. Removing or renaming a
/// variant of `Admission` makes this REFUSE TO COMPILE; no mutation of the production code
/// can turn the `assert_eq!` calls below false while the type still has three variants named
/// the way this file names them. ⚠️ THAT IS UNCHANGED BY TASK 5: `admit` computes now, but
/// it computes for the probes BELOW -- this one still builds its own `Refused`, because it
/// is the only variant a bench can construct.
#[test]
fn an_admission_is_distinguishable_three_ways() {
    let outcome = Admission::Refused {
        asked: Mib::new(512),
        ceiling: Mib::new(256),
    };

    match outcome {
        Admission::Granted(_) => panic!("this probe never constructs a Granted admission"),
        Admission::Queued(_) => panic!("this probe never constructs a Queued admission"),
        Admission::Refused { asked, ceiling } => {
            assert_eq!(asked, Mib::new(512));
            assert_eq!(ceiling, Mib::new(256));
        }
    }
}

/// ⛔ THE ASSERTION IS THE NUMBER, NOT THE VARIANT. "It granted" is satisfied by an arbiter
/// that grants everything; what says the budget is real is that `allocated` MOVED BY THE
/// RESERVATION.
#[test]
fn a_grant_takes_exactly_its_reservation_out_of_the_budget() {
    let mut arbiter = arbiter(TOTAL);
    assert_eq!(arbiter.allocated(), Mib::ZERO);

    let outcome = arbiter.admit(
        &profile("asr-realtime", 1_024, ComputeClass::Realtime),
        LONG,
        Monotonic::ORIGIN,
    );
    assert!(matches!(outcome, Admission::Granted(_)));
    assert_eq!(arbiter.allocated(), Mib::new(1_024));
}

/// ⛔ THE OTHER HALF OF THE PROPERTY, and it is the arbiter half of §5.7 properties 2 and 3
/// -- the only half milestone 5 can hold, and it is ONE and not two: the arbiter does not
/// need to know WHO held a grant, only that releasing puts the reservation back.
#[test]
fn releasing_gives_back_exactly_the_reservation() {
    let mut arbiter = arbiter(TOTAL);
    let Admission::Granted(grant) = arbiter.admit(
        &profile("trellis2-512-lean", 6_144, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("6144 of 16384 fits");
    };
    assert_eq!(arbiter.allocated(), Mib::new(6_144));

    let returned = arbiter
        .release(grant, Monotonic::ORIGIN)
        .expect("the arbiter issued this grant");

    assert_eq!(returned, Mib::new(6_144));
    assert_eq!(arbiter.allocated(), Mib::ZERO);
}

/// ⛔ THE `Err` OF `release` IS REACHABLE, which is what keeps it from being the dead
/// surface this repository removed from `Record::encode` and refused to `Ipc::accept`. Two
/// arbiters, a grant from the first handed to the second.
///
/// ⚠️ WHAT IT PROVES IS "IT IS NOT IN MY BOOKS", NOT "I TELL MINE FROM SOMEBODY ELSE'S", and
/// the declared limit beside `ReleaseError` says why: the second arbiter here is EMPTY.
#[test]
fn a_grant_released_on_the_wrong_arbiter_is_an_error_and_not_a_silent_credit() {
    let mut first = arbiter(TOTAL);
    let mut second = arbiter(TOTAL);

    let Admission::Granted(grant) = first.admit(
        &profile("asr-realtime", 1_024, ComputeClass::Realtime),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("1024 of 16384 fits");
    };

    assert!(second.release(grant, Monotonic::ORIGIN).is_err());
    assert_eq!(second.allocated(), Mib::ZERO, "no silent credit");
}

/// ⛔ THE INVARIANT, ASSERTED ON THE NUMBER: the sum of ALL grants never exceeds the total.
/// The third request does not fit and comes back `Refused` with the two numbers design/02
/// asks for.
#[test]
fn the_sum_of_the_grants_never_exceeds_the_total() {
    let mut arbiter = arbiter(Mib::new(8_192));
    for name in ["a", "b"] {
        let outcome = arbiter.admit(
            &profile(name, 4_096, ComputeClass::Batch),
            LONG,
            Monotonic::ORIGIN,
        );
        assert!(matches!(outcome, Admission::Granted(_)));
    }
    assert_eq!(arbiter.allocated(), Mib::new(8_192));

    let Admission::Refused { asked, ceiling } = arbiter.admit(
        &profile("c", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("the budget is full");
    };
    assert_eq!(asked, Mib::new(4_096));
    assert_eq!(ceiling, Mib::new(8_192));
    assert_eq!(
        arbiter.allocated(),
        Mib::new(8_192),
        "nothing was over-admitted"
    );
}

/// ⛔ AN IMPOSSIBLE CONFIGURATION IS VISIBLE INSTEAD OF SILENT, and this is the probe that
/// pays for the design's divergence from §5.1. With the two quotas SUBTRACTED from the
/// total, a total smaller than their sum would give a budget of zero WITHOUT A WORD. As two
/// permanent grants, the second one comes back `Refused` and names both numbers.
#[test]
fn a_total_smaller_than_the_two_permanent_quotas_refuses_the_second_one() {
    let mut arbiter = arbiter(Mib::new(1_500));

    let audio = arbiter.admit(
        &profile("audio-reserved", 1_024, ComputeClass::Realtime),
        LONG,
        Monotonic::ORIGIN,
    );
    assert!(matches!(audio, Admission::Granted(_)));

    let Admission::Refused { asked, ceiling } = arbiter.admit(
        &profile("presentation-reserved", 1_024, ComputeClass::Realtime),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("1024 + 1024 does not fit in 1500");
    };
    assert_eq!(asked, Mib::new(1_024));
    assert_eq!(ceiling, Mib::new(1_500));
}

/// ⛔ THE LAZY COLLECTION, AND THE PROPERTY IS WRITTEN SO IT IS OBSERVABLE. Between two
/// operations an expired grant stays in the books -- it denies nothing to nobody, there IS
/// nobody -- and at the first one who looks it is already freed. §5.7 property 5.
#[test]
fn an_expired_grant_does_not_stay_allocated() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let outcome = arbiter.admit(
        &profile("short-lived", 4_096, ComputeClass::Batch),
        Millis::new(5_000),
        Monotonic::ORIGIN,
    );
    assert!(matches!(outcome, Admission::Granted(_)));

    // The same request, after the window: the first one is collected and this one fits.
    let after = arbiter.admit(
        &profile("the-next-one", 4_096, ComputeClass::Batch),
        Millis::new(5_000),
        Monotonic::from_millis(5_001),
    );
    assert!(
        matches!(after, Admission::Granted(_)),
        "without the collection this is Refused"
    );
    assert_eq!(arbiter.allocated(), Mib::new(4_096), "one grant, not two");
}

/// The counter-probe of the one above, and it is the direction that is skipped: a grant
/// that has NOT expired is not collected. Without this, "collect everything always" passes.
#[test]
fn a_grant_still_inside_its_window_is_not_collected() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let outcome = arbiter.admit(
        &profile("still-running", 4_096, ComputeClass::Batch),
        Millis::new(5_000),
        Monotonic::ORIGIN,
    );
    assert!(matches!(outcome, Admission::Granted(_)));

    let after = arbiter.admit(
        &profile("the-next-one", 4_096, ComputeClass::Batch),
        Millis::new(5_000),
        Monotonic::from_millis(4_999),
    );
    assert!(
        matches!(after, Admission::Refused { .. }),
        "the window has not closed yet"
    );
}

/// ⛔ A REQUEST BIGGER THAN THE WHOLE MACHINE IS `Refused` AND NEVER `Queued`: no release
/// will ever make room for it, and a ticket that can never be served is a leak that looks
/// like patience.
#[test]
fn a_request_larger_than_the_total_is_refused_and_not_queued() {
    let mut arbiter = arbiter(Mib::new(8_192));
    let Admission::Refused { asked, ceiling } = arbiter.admit(
        &profile("too-big", 32_768, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("32768 never fits in 8192");
    };
    assert_eq!(asked, Mib::new(32_768));
    assert_eq!(ceiling, Mib::new(8_192));
}
