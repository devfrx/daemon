//! What the compiler cannot hold about the GRANT CYCLE. Two things at task 4, and the
//! ADMISSION itself from task 5: the LEGAL side of `I2 · §5.3`, the positive side of `V4`
//! -- that `Admission` truly answers THREE ways -- and then that the sum of every grant
//! never exceeds the total, that releasing gives back EXACTLY the reservation, and that an
//! expired grant does not stay allocated.
//!
//! ⛔ AND THE QUEUES FROM TASK 6, which is a SUBJECT and not one more assertion about the
//! previous one: that a request which fits the machine but not the moment is QUEUED instead of
//! refused, that `promote` serves the queue BY LANE and, inside one lane, in arrival order,
//! that it serves EVERY request that fits and skips ahead to none, that it collects the expired
//! before it serves, and that what comes out of the queue is a grant like any other.
//!
//! ⛔ AND THE REVOCATION FROM TASK 7, WHICH IS A THIRD SUBJECT AND WHOSE PROBES ARE ALMOST ALL
//! SOMEWHERE ELSE -- said here because a reader who counted the revocation probes in this file
//! would find ONE and conclude the subject is barely held. `Arbiter::ask_back` is `pub(crate)`,
//! and a `pub(crate)` is unreachable from an integration test, which is a crate of its own
//! (`error[E0624]`, measured). So the THIRTEEN probes that CALL it live in the `#[cfg(test)] mod
//! tests` of `crates/kernel/src/arbiter/mod.rs`, with the deviation declared beside them: that
//! asking back MARKS and does not free, that the grace is collected when it runs out and not
//! before, the instant it runs out, that a non-preemptible grant and a lane that is not below
//! the asking one are never touched, that a PEER in the very lane that is asking is not touched
//! either -- `below` is exclusive -- that it stops as soon as the need is covered and takes the
//! WORST lane first, that when the worst lane does not cover the need ON ITS OWN the marking
//! carries on into the next one, that it marks NOTHING when what can be reclaimed does not cover
//! the need, that asking twice does not buy the room twice, and that it collects the expired
//! before it marks.
//!
//! ⚠️ THIS COUNT IS REWRITTEN AND NEVER ANNOTATED, AND IT HAS BEEN REWRITTEN TWICE: "TEN" until
//! 2026-08-20, "TWELVE" for the first wave of corrections of that day, THIRTEEN since the second.
//! A count that is quietly wrong is worse than one that is loudly missing (gotcha #31). The two
//! of the first wave were the LANE BOUNDARY, which no probe stood on, and the CAPACITY of what can
//! actually be reclaimed; the one of the second is the marking pass CROSSING out of the worst lane
//! when that lane does not cover the need alone.
//!
//! ⛔ AND THIS IS THE ONLY PLACE IN THIS FILE WHERE THAT NUMBER IS WRITTEN, since 2026-08-20 and
//! deliberately. There was a second copy in the doc of the probe below; it said "ten" while this
//! line said "twelve", which is gotcha #31 in one file and gotcha #68 on top -- the second copy
//! sat right under a claim that the number had just been rewritten. A figure that lives in two
//! places gets taken out of one, not realigned in both (`E77`).
//!
//! ⛔ WHAT STAYS HERE IS THE ONE PROBE THAT NEEDS NOTHING PRIVATE, and only privacy moved the
//! others: `a_grant_that_is_neither_expired_nor_revoking_survives_the_sweep` is about the
//! SWEEP, which is where the two deadlines of task 7 meet the validity window of task 5, and it
//! belongs beside the other probes of that window.
//!
//! ⛔ AND TASK 8 ADDED NO PROBE HERE AND STILL CHANGED WHAT THIS FILE PROVES, which is why it
//! is written down instead of left in the diff. `Arbiter::new` now takes a `VramPolicy`, and the
//! helper below hands over `RemotePolicy` -- the DEFAULT of ADR-0006, the one that makes NO room
//! -- so EVERY probe in this file is now scoped to the remote policy. Under `LocalPolicy` the
//! admission would ask the lower lanes back before it queued, and a dozen probes about the
//! queues and the sweep would silently be about the revocation instead. ⚠️ THE OTHER HALF OF
//! THAT, MEASURED AND NOT ASSUMED: nothing here holds the new branch. With the policy question
//! deleted from `admit` in EITHER direction -- nobody asks back, or everybody does -- this file
//! stays at 20 passed, 0 failed and the `#[cfg(test)] mod tests` of the lib at 13, 0. The branch
//! is held by `tests/arbiter_policy.rs` and by nothing else, and the extraction of `enqueue`
//! that came with it IS held here: filing every request in the `Batch` lane, or freezing the
//! ticket counter, each turns probes of this file red. Registered as `E96`.
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
//! ⚠️ DATED RECALL, 2026-08-19: TASK 6 SKIPPED THE INSTRUCTION THE LINE ABOVE NAMES IT IN.
//! It added eight probes on a NEW subject -- the queues, `promote`, `queued` -- and left this
//! comment enumerating tasks 4 and 5 only, which is a rule that does not bind the document
//! hosting it (gotcha #68). Repaired in review by EXTENDING the enumeration above, not by
//! overwriting it. TASK 7 IS STILL BOUND BY IT.
//!
//! ⚠️ TWO DIFFERENT RULES ABOUT COMPARISON LIVE HERE, DELIBERATELY. R2 -- probes MATCH and
//! never compare -- is about `Admission`, which carries a `Grant` and therefore has neither
//! `Debug` nor `PartialEq`: giving them to it for the convenience of this file is the trade
//! `ports::process` refused, so every probe below uses `matches!` and `let … else`, never
//! `assert_eq!` on an `Admission`. `Activity` carries no grant and derives both, so
//! `assert_ne!` on it directly is fine and is what the first probe uses.

use kernel::arbiter::{
    Activity, Admission, Arbiter, ComputeClass, Mib, PreemptibleState, Preemption, Promotion,
    RemotePolicy, ResourceProfile, VramPolicy,
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

/// A profile the arbiter MAY ask back, with the grace its holder then gets.
///
/// ⚠️ NO `use` CAME WITH IT. The plan's step opened with `use kernel::arbiter::{Activity,
/// PreemptibleState};` and `use kernel::time::Millis as Grace;`, and all three names are
/// ALREADY imported at the top of this file -- `error[E0252]: the name ... is defined multiple
/// times`, measured. Merged instead of added, which is what the module comment above has been
/// telling every task since task 5.
fn preemptible(name: &'static str, vram: u64, lane: ComputeClass, grace: u64) -> ResourceProfile {
    ResourceProfile {
        name,
        reserved_vram: Mib::new(vram),
        compute_class: lane,
        preemption: Preemption::After(Millis::new(grace)),
    }
}

/// ⛔ IT HANDS OVER `RemotePolicy`, AND THAT IS NOT A FILLER ARGUMENT. Remote is the DEFAULT of
/// ADR-0006 and the one that makes NO room, so every probe in this file keeps the subject it was
/// written with: under `LocalPolicy` the admission would start MARKING victims before it queues,
/// and probes about the queues and the sweep would silently be about the revocation instead. The
/// two policies have a bench of their own, `tests/arbiter_policy.rs`.
fn arbiter(total: Mib) -> Arbiter {
    Arbiter::new(
        Parameters::new(TURN_LIMIT, total),
        VramPolicy::Remote(RemotePolicy),
    )
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
///
/// ⛔ AND THE SECOND ASSERTION CANNOT FAIL, WRITTEN DOWN INSTEAD OF QUIETLY KEPT -- the
/// species this milestone has already paid for as `E17`. `release` can only REMOVE from the
/// map, never insert, so `allocated()` on an arbiter that was born empty is `Mib::ZERO`
/// whatever the production code does: no mutation of `release` turns that line red. IT IS
/// THE LINE ABOVE, `is_err()`, THAT HOLDS THE PROBE. The assertion stays because it states
/// the intent the name carries -- "and not a silent credit" -- and because the day `release`
/// grows a path that can INSERT, it stops being free and starts being the guard it reads
/// like.
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
/// The third request does not fit, and whatever the arbiter answers it does NOT admit it.
///
/// ⚠️ RECALL OF 2026-08-19, MILESTONE 5 TASK 6 -- WHAT THIS PROBE MATCHED BEFORE THE QUEUES,
/// rewritten instead of extended so no reader takes the old shape for the current one. The
/// third request used to come back `Refused` and this probe pinned both of its numbers. Since
/// task 6 "it fits the machine but not the moment" is `Queued`, so the VARIANT assertion
/// moved. ⛔ THE ASSERTION THAT CARRIES THE PROPERTY DID NOT MOVE: it is `allocated()` still
/// at `8_192`, which is the whole of "nothing was over-admitted", and it is asserted here
/// exactly as it was. The two numbers of `Refused` are still pinned, by
/// `a_request_larger_than_the_total_is_refused_and_not_queued`.
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

    let Admission::Queued(_) = arbiter.admit(
        &profile("c", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("the budget is full, and 4096 fits the machine -- so the third one waits");
    };
    assert_eq!(
        arbiter.allocated(),
        Mib::new(8_192),
        "nothing was over-admitted"
    );
}

/// ⛔ AN IMPOSSIBLE CONFIGURATION AND THE SECOND QUOTA IS NOT GRANTED, and this is the probe
/// that pays for the design's divergence from §5.1. With the two quotas SUBTRACTED from the
/// total, a total smaller than their sum would give a budget of zero WITHOUT A WORD. As two
/// permanent grants, the second one takes nothing the machine does not have.
///
/// ⛔ RECALL OF 2026-08-19, MILESTONE 5 TASK 6 -- THIS PROBE HAS LOST THE PROPERTY IT WAS
/// WRITTEN FOR, and the old sentence is REPLACED rather than left standing above a correction,
/// because a comment that keeps promising VISIBILITY while the answer became patience is the
/// finding A-2 of this project's audit done again. It was called
/// `a_total_smaller_than_the_two_permanent_quotas_refuses_the_second_one` and asserted a
/// `Refused` carrying both numbers: the impossible configuration ANNOUNCED ITSELF. Since the
/// queues the second quota is `Queued` -- and nobody will ever serve it, because releasing a
/// permanent grant is exactly what nobody does. A ticket that waits for ever is the silent
/// degradation ADR-0005 and ADR-0019 forbid.
///
/// ⛔ AND THE ARBITER CANNOT REPAIR IT, WHICH IS WHY NOTHING HERE TRIES. The reason is already
/// written beside `Activity`: "Permanence is not a type -- it is nobody calls release". The
/// arbiter therefore cannot tell a ticket that WILL be served from one that never will, and a
/// rule it cannot evaluate is not a rule it can enforce. ✅ THE VISIBILITY LIVES IN THE
/// COMPOSITION ROOT SINCE 2026-08-21, MILESTONE 5 TASK 10, and this sentence promised it in the
/// FUTURE until then: `crates/daemon/src/main.rs` asks for the two permanent grants itself, and
/// anything but `Granted` becomes `StartupError::ReservedQuota`, which NAMES the quota that did
/// not get in and stops the start-up. ⛔ TWO PROBES THERE HOLD BOTH ROADS — `Queued` on a machine
/// of 1500 MiB, `Refused` on one of 500 — because a direction of proof held by a mutation is
/// held by nothing (gotcha #72), and the two roads fail differently (gotcha #65).
///
/// ⚠️ WHAT IS LEFT HERE IS STILL WORTH HOLDING, and it is stated so nobody reads more into the
/// name: the second quota is NOT GRANTED. `allocated()` stays at the first one, so an
/// impossible configuration does not become OVER-ADMISSION. It has only stopped being loud.
#[test]
fn a_total_smaller_than_the_two_permanent_quotas_does_not_grant_the_second_one() {
    let mut arbiter = arbiter(Mib::new(1_500));

    let audio = arbiter.admit(
        &profile("audio-reserved", 1_024, ComputeClass::Realtime),
        LONG,
        Monotonic::ORIGIN,
    );
    assert!(matches!(audio, Admission::Granted(_)));

    let Admission::Queued(_) = arbiter.admit(
        &profile("presentation-reserved", 1_024, ComputeClass::Realtime),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("1024 + 1024 does not fit in 1500");
    };
    assert_eq!(
        arbiter.allocated(),
        Mib::new(1_024),
        "the second quota took nothing the machine does not have"
    );
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
///
/// ⚠️ RECALL OF 2026-08-19, MILESTONE 5 TASK 6 -- WHAT THIS PROBE MATCHED BEFORE THE QUEUES.
/// The second request used to come back `Refused` and the assertion read
/// `matches!(after, Admission::Refused { .. })`. Since task 6 the answer is `Queued`, because
/// `4_096` fits the machine and only the moment is wrong, so the match moved.
///
/// ⛔ WHAT HOLDS THE COLLECTION DIRECTION IS THE `let … else`, NOT THE NUMBER, and it is said
/// out loud rather than left to be discovered: with "collect everything always" the first
/// grant would be gone at `4_999`, the second request would be `Granted`, and this `let … else`
/// would panic. ⚠️ `allocated()` WOULD STILL READ `4_096` IN THAT WORLD -- one grant, the new
/// one -- so the number below is NOT what catches the mutation. What the number does catch is
/// the other defect: a queued request that RESERVED anyway, which would read `8_192`.
#[test]
fn a_grant_still_inside_its_window_is_not_collected() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let outcome = arbiter.admit(
        &profile("still-running", 4_096, ComputeClass::Batch),
        Millis::new(5_000),
        Monotonic::ORIGIN,
    );
    assert!(matches!(outcome, Admission::Granted(_)));

    let Admission::Queued(_) = arbiter.admit(
        &profile("the-next-one", 4_096, ComputeClass::Batch),
        Millis::new(5_000),
        Monotonic::from_millis(4_999),
    ) else {
        panic!("the window has not closed yet, so the second request waits");
    };
    assert_eq!(
        arbiter.allocated(),
        Mib::new(4_096),
        "a queued request reserves nothing"
    );
}

/// THE BOUNDARY THE TWO PROBES ABOVE STEP OVER. One asks at `5_001` and the other at
/// `4_999`; `5_000` itself -- the instant the window closes -- was asked by nobody, so `>`
/// mutated to `>=` inside `collect_expired` survived the whole suite, on the very function
/// those two exist to hold.
///
/// ⛔ AND IT WRITES DOWN WHICH SEMANTICS IS THE CHOSEN ONE, because a boundary nobody names
/// is a boundary somebody later "fixes". `retain(|_, held| held.expires_at > now)` means
/// that at `now == expires_at` the grant IS ALREADY COLLECTED: the window is HALF-OPEN,
/// `[start, expiry)`, and a grant is valid up to -- and not including -- the instant it
/// expires. A choice, not an accident.
///
/// ⚠️ THE CONSEQUENCE THAT IS NOT OBVIOUS, and it is why the choice is worth naming:
/// `release` collects before it looks, so at `now == expires_at` handing the grant back
/// answers `Err(ReleaseError::UnknownGrant)` -- measured. The limit written beside
/// `ReleaseError` is where that lives.
#[test]
fn a_grant_is_collected_at_the_instant_its_window_closes() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let outcome = arbiter.admit(
        &profile("short-lived", 4_096, ComputeClass::Batch),
        Millis::new(5_000),
        Monotonic::ORIGIN,
    );
    assert!(matches!(outcome, Admission::Granted(_)));

    let after = arbiter.admit(
        &profile("the-next-one", 4_096, ComputeClass::Batch),
        Millis::new(5_000),
        Monotonic::from_millis(5_000),
    );
    assert!(
        matches!(after, Admission::Granted(_)),
        "at now == expires_at the window is already shut: [start, expiry)"
    );
    assert_eq!(arbiter.allocated(), Mib::new(4_096), "one grant, not two");
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

/// A request that does not fit NOW but could fit later is queued, not refused.
#[test]
fn a_request_that_fits_the_machine_but_not_the_moment_is_queued() {
    let mut arbiter = arbiter(Mib::new(8_192));
    let Admission::Granted(resident) = arbiter.admit(
        &profile("resident", 8_192, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine exactly");
    };

    let Admission::Queued(ticket) = arbiter.admit(
        &profile("waiting", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("4096 fits the machine, just not right now");
    };
    assert_eq!(arbiter.queued(), 1);

    let _ = arbiter
        .release(resident, Monotonic::ORIGIN)
        .expect("this arbiter issued it");
    let promoted = arbiter.promote(Monotonic::ORIGIN);

    assert_eq!(promoted.len(), 1);
    assert_eq!(promoted[0].ticket, ticket);
    assert_eq!(arbiter.allocated(), Mib::new(4_096));
    assert_eq!(arbiter.queued(), 0);
}

/// ⛔ THE ASSERTION THAT KEEPS M-7's NUMBERS VALID, and it is the whole reason the queue is
/// per lane. The three waiters arrive in the WORST order there is -- `Batch` first,
/// `Interactive` second, `Realtime` last -- and they come out exactly REVERSED. A global FIFO
/// would serve them as they arrived and this probe would go red.
///
/// ⚠️ RECALL OF 2026-08-19, MILESTONE 5 TASK 6, IN REVIEW -- THIS PROBE PITTED TWO LANES AND
/// NOT THREE, and it is REWRITTEN rather than joined by a second one because what it says is
/// the same thing, said completely. The rewrite is a MEASUREMENT and not a tidy-up: with
/// `Interactive` against `Batch` only, NOTHING IN THE WORKSPACE EVER PROMOTED A WAITER IN THE
/// `Realtime` LANE -- ✅ measured, `if *lane_key == ComputeClass::Realtime { continue; }`
/// inside the lane loop of `promote` left ALL 34 TARGETS GREEN. "Best lane first" was
/// therefore proved on the SECOND-best lane, and `Realtime` is the very lane §5.3.1, M-7 and
/// ADR-0033's permanent quotas are about.
///
/// ⚠️ THE SIZES ARE WHAT MAKE THIS A THREE-WAY ORDER INSTEAD OF A ONE-WAY ONE. Exactly TWO of
/// the three fit the room that comes back, so one probe states three things: `Realtime`
/// before `Interactive`, `Interactive` before `Batch`, and `Batch` LEFT WAITING while the
/// room it arrived first for goes to the two lanes above it. Three waiters of `4_096` would
/// have served one and said nothing about the other two.
///
/// ⚠️ AND THE `assert_ne!` IS NOT FREE, which is worth saying because the shape looks it (the
/// species this milestone paid for as `E17`). It is what catches a `next_ticket` that never
/// advances: with the increment deleted every ticket is `TicketId(0)`, the two `assert_eq!`
/// above still pass, and that line does not.
#[test]
fn the_queue_promotes_by_lane_and_not_in_arrival_order() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let Admission::Granted(resident) = arbiter.admit(
        &profile("resident", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine exactly");
    };

    let Admission::Queued(batch) = arbiter.admit(
        &profile("batch-first", 2_048, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("queued");
    };
    let Admission::Queued(interactive) = arbiter.admit(
        &profile("interactive-second", 2_048, ComputeClass::Interactive),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("queued");
    };
    let Admission::Queued(realtime) = arbiter.admit(
        &profile("realtime-last", 2_048, ComputeClass::Realtime),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("queued");
    };
    assert_eq!(arbiter.queued(), 3);

    let _ = arbiter
        .release(resident, Monotonic::ORIGIN)
        .expect("this arbiter issued it");
    let promoted = arbiter.promote(Monotonic::ORIGIN);

    assert_eq!(promoted.len(), 2, "2048 + 2048 is exactly the room freed");
    assert_eq!(
        promoted[0].ticket, realtime,
        "the best lane first, and it arrived LAST"
    );
    assert_eq!(
        promoted[1].ticket, interactive,
        "then the next lane down, and it arrived second"
    );
    assert_ne!(promoted[0].ticket, batch);
    assert_eq!(
        arbiter.queued(),
        1,
        "the worst lane arrived FIRST and is the one left waiting"
    );
}

/// Within ONE lane the order is arrival order, and this is what says the lane rule above is
/// not "any order at all".
#[test]
fn inside_one_lane_the_order_is_the_order_of_arrival() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let Admission::Granted(resident) = arbiter.admit(
        &profile("resident", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("full");
    };

    let Admission::Queued(first) = arbiter.admit(
        &profile("first", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("queued");
    };
    let Admission::Queued(second) = arbiter.admit(
        &profile("second", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("queued");
    };

    let _ = arbiter
        .release(resident, Monotonic::ORIGIN)
        .expect("issued here");
    let promoted = arbiter.promote(Monotonic::ORIGIN);
    assert_eq!(promoted.len(), 1);
    assert_eq!(promoted[0].ticket, first);
    assert_ne!(promoted[0].ticket, second);
}

/// ⛔ THE COUNTER-PROBE, and it is the one that says `promote` is not "grant everything in
/// the queue": with no room freed it promotes NOTHING and the books do not move.
#[test]
fn promote_with_no_room_freed_promotes_nothing() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let Admission::Granted(_resident) = arbiter.admit(
        &profile("resident", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("full");
    };
    let Admission::Queued(_) = arbiter.admit(
        &profile("waiting", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("queued");
    };

    let promoted = arbiter.promote(Monotonic::ORIGIN);
    assert!(promoted.is_empty());
    assert_eq!(arbiter.allocated(), Mib::new(4_096));
    assert_eq!(arbiter.queued(), 1, "the ticket is still waiting");
}

/// ⛔ AND A PROMOTION IS A GRANT LIKE ANY OTHER: what comes out of the queue can be released
/// and gives back exactly its reservation. Without this the queue could hand out grants the
/// books never learned about.
#[test]
fn a_promoted_grant_is_a_grant_like_any_other() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let Admission::Granted(resident) = arbiter.admit(
        &profile("resident", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("full");
    };
    let Admission::Queued(_) = arbiter.admit(
        &profile("waiting", 2_048, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("queued");
    };
    let _ = arbiter
        .release(resident, Monotonic::ORIGIN)
        .expect("issued here");

    let mut promoted: Vec<Promotion> = arbiter.promote(Monotonic::ORIGIN);
    assert_eq!(promoted.len(), 1);
    let promotion = promoted.remove(0);
    assert_eq!(arbiter.allocated(), Mib::new(2_048));

    let returned = arbiter
        .release(promotion.grant, Monotonic::ORIGIN)
        .expect("the promotion came from this arbiter");
    assert_eq!(returned, Mib::new(2_048));
    assert_eq!(arbiter.allocated(), Mib::ZERO);
}

/// ⛔ THE RULE THE `promote` DOC STATES AND THAT NOTHING ELSE HOLDS: it STOPS at the first
/// request that does not fit WITHIN A LANE, and does not skip ahead to a smaller one that
/// would. A rule written in a comment and held by nothing is an intention (gotcha #42), and
/// §7.1.4 wants two directions per rule. This is the direction that gets skipped: the small
/// request IS servable and is NOT served, because a bigger one is in front of it in its own
/// lane.
///
/// ⚠️ RECALL OF 2026-08-19, MILESTONE 5 TASK 6, IN REVIEW -- "A `promote` THAT SKIPPED AHEAD
/// PASSES EVERY OTHER PROBE IN THIS FILE" WAS WRITTEN AS A FACT AND HAD NEVER BEEN MEASURED,
/// which is exactly the shape this repository calls an intention when it finds it in
/// production code. The campaign substituted the mutation that DELETES the fit check, which
/// kills four probes and therefore isolates nothing. ✅ NOW MEASURED, with the skip-ahead
/// mutation itself -- the `break` on the first non-fitting waiter replaced by a scan for the
/// first entry that fits: this probe goes red and it is SOLE, 18 passed 1 failed. The
/// sentence is kept because it turned out to be true, not because it was written down.
///
/// ⚠️ THE ROOM FREED IS EXACTLY THE SMALL ONE'S, and that is the whole construction: `1_024`
/// comes back, the head of the lane asks `4_096` and does not fit, the tail asks `1_024` and
/// would. Skipping ahead is a scheduling policy nobody decided, and it would let a large
/// request in a busy lane wait for ever behind small ones.
///
/// ⚠️ BOTH WAITERS ARE IN THE SAME LANE, DELIBERATELY. Put them in different lanes and the
/// probe would be testing the lane order instead, which
/// `the_queue_promotes_by_lane_and_not_in_arrival_order` already holds.
#[test]
fn promote_does_not_skip_ahead_to_a_smaller_request_behind_a_bigger_one() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let Admission::Granted(_bulk) = arbiter.admit(
        &profile("bulk", 3_072, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("3072 of 4096 fits");
    };
    let Admission::Granted(small_resident) = arbiter.admit(
        &profile("small-resident", 1_024, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("3072 + 1024 is exactly 4096");
    };

    let Admission::Queued(_big) = arbiter.admit(
        &profile("big-waiter", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("4096 fits the machine, just not right now");
    };
    let Admission::Queued(_small) = arbiter.admit(
        &profile("small-waiter", 1_024, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("queued behind the big one, in the same lane");
    };
    assert_eq!(arbiter.queued(), 2);

    let returned = arbiter
        .release(small_resident, Monotonic::ORIGIN)
        .expect("this arbiter issued it");
    assert_eq!(
        returned,
        Mib::new(1_024),
        "exactly the small waiter's room, and not the big one's"
    );

    let promoted = arbiter.promote(Monotonic::ORIGIN);

    assert!(
        promoted.is_empty(),
        "the head of the lane does not fit, so the lane stops there"
    );
    assert_eq!(arbiter.queued(), 2, "both are still waiting");
    assert_eq!(
        arbiter.allocated(),
        Mib::new(3_072),
        "the freed room stayed free instead of going to the one behind"
    );
}

/// ⛔ `promote` COLLECTS THE EXPIRED BEFORE IT DECIDES, and this is the only probe that
/// exercises that line. "The arbiter collects before it decides" is a property of EVERY
/// operation -- it is why `collect_expired` is private. Deleting `self.collect_expired(now)`
/// from this one has to make something red, and before this probe nothing went red.
///
/// ⛔ RECALL OF 2026-08-28, FINDING AUD-018 -- THIS SAID "and with `promote` there are now THREE
/// of them", AND THE COUNT IS REMOVED FROM ALL THREE OF ITS HOUSES RATHER THAN REALIGNED TO FOUR.
/// It was written at task 6 with the right figure and task 7 added `ask_back` without this line
/// being reread. ⛔ THE FINDING NAMED TWO HOUSES AND THERE WERE THREE: this one, the doc of
/// `ask_back` in `src/arbiter/mod.rs`, and the doc of `ask_back_collects_the_expired_before_it_
/// marks` in the same file -- so realigning would have left a figure standing in three places
/// that the NEXT operation falsifies again. What stays is the half that cannot rot, "a property
/// of EVERY operation", which each of the three already said next to the number. If the figure
/// is ever wanted: `grep -c 'self\.collect_expired(now);' crates/kernel/src/arbiter/mod.rs`, which
/// answered 4 on 2026-08-28.
///
/// ⚖️ AND THE OTHER TWO HOUSES WERE NOT WRONG, WHICH IS WHY THIS IS NOT A TYPO FIX: both already
/// said FOUR. The defect was one property counted in three places, so that a correct edit to two
/// of them left the third lying -- gotcha #68, and the reason the cure is subtraction.
///
/// ⚠️ THERE IS NO `release` CALL HERE, DELIBERATELY, and that is what makes the probe about
/// `promote` instead of about the collection in general: the only thing that can free the room
/// the promotion needs is the collection INSIDE `promote`. The resident's window closes at
/// `5_000` and the promotion is asked at `5_001`.
#[test]
fn promote_collects_the_expired_before_it_serves_the_queue() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let Admission::Granted(_short_lived) = arbiter.admit(
        &profile("short-lived", 4_096, ComputeClass::Batch),
        Millis::new(5_000),
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine exactly");
    };
    let Admission::Queued(ticket) = arbiter.admit(
        &profile("waiting", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("4096 fits the machine, just not right now");
    };

    let promoted = arbiter.promote(Monotonic::from_millis(5_001));

    assert_eq!(
        promoted.len(),
        1,
        "without the collection inside promote there is no room and nothing moves"
    );
    assert_eq!(promoted[0].ticket, ticket);
    assert_eq!(arbiter.allocated(), Mib::new(4_096), "one grant, not two");
    assert_eq!(arbiter.queued(), 0);
}

/// ⛔ `promote` SERVES EVERY REQUEST THAT FITS, NOT JUST THE FIRST ONE, and this probe is
/// here because the mutation that stops after one SURVIVED the whole suite. Not dictated by
/// the plan: added on 2026-08-19 while measuring task 6, on the same grounds as `E29` --
/// `promote`'s doc says it "serves the queue with whatever room there is now", the plural was
/// stated in a comment and held by NOTHING, and a rule held by nothing is an intention
/// (gotcha #42). ✅ Measured, not deduced: with a `break` after the first promotion of a lane,
/// all eighteen other probes stayed green.
///
/// ⚠️ THE TWO WAITERS ARE IN THE SAME LANE, and that is what makes the probe about "how many"
/// rather than about "which lane": one per LANE would still pass if they were split.
///
/// ⚠️ AND THE ORDER IS ASSERTED TOO, on both slots. The probe would be satisfied by a
/// promotion that served them backwards if it only counted, and counting is the thing
/// gotcha #30 warns about.
#[test]
fn promote_serves_every_request_that_fits_and_not_just_the_first() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let Admission::Granted(resident) = arbiter.admit(
        &profile("resident", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine exactly");
    };
    let Admission::Queued(first) = arbiter.admit(
        &profile("first", 2_048, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("queued");
    };
    let Admission::Queued(second) = arbiter.admit(
        &profile("second", 2_048, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("queued behind the first, in the same lane");
    };
    assert_eq!(arbiter.queued(), 2);

    let _ = arbiter
        .release(resident, Monotonic::ORIGIN)
        .expect("this arbiter issued it");
    let promoted = arbiter.promote(Monotonic::ORIGIN);

    assert_eq!(promoted.len(), 2, "2048 + 2048 is exactly the room freed");
    assert_eq!(promoted[0].ticket, first);
    assert_eq!(promoted[1].ticket, second);
    assert_eq!(arbiter.allocated(), Mib::new(4_096));
    assert_eq!(arbiter.queued(), 0, "the lane emptied");
}

/// ⛔ THE PROBE THAT PROTECTS THE ONES FROM TASK 5 FROM BECOMING VACUOUS (gotcha #66): with
/// forced reclamation now living inside the same sweep, a grant that is neither expired nor
/// revoking must still survive it. Without it, a `collect_expired` whose SECOND arm answered
/// "collect" would empty the books of everything that was never asked back, and the whole of
/// the validity window above would still be green.
///
/// ⚠️ THE ANSWER IS `Queued` AND NOT `Refused`, AND THE PLAN'S STEP SAID `Refused`. That was
/// written before task 6 gave `admit` its queueing branch: `4_096` is not bigger than the whole
/// machine, so "it fits the machine but not the moment" is a ticket. Corrected against the code
/// of today rather than against the plan, exactly as the three probes task 6 rewrote.
///
/// ⚠️ IT STAYS IN THIS FILE while the OTHER probes of task 7 are in
/// `crates/kernel/src/arbiter/mod.rs`, and the reason is the one the module comment gives: this
/// one never calls `ask_back`, so nothing private is out of its reach.
///
/// ⚠️ RECALL OF 2026-08-20, SECOND REVIEW OF THIS TASK -- THIS SAID "THE OTHER TEN PROBES", AND
/// THE FIGURE IS TAKEN OUT RATHER THAN RECORRECTED. They were TWELVE when it was read, and the
/// module comment of THIS SAME FILE said so thirty lines up -- while claiming, in the paragraph
/// right after, to have just rewritten that very number. ⛔ TWO PRESENT-TENSE COUNTS
/// CONTRADICTING EACH OTHER IN ONE FILE IS GOTCHA #31, AND A CLAIM OF COMPLETENESS THE SAME FILE
/// FALSIFIES IS GOTCHA #68 -- the rule that fails to bind the document hosting it, which weighs
/// more than the figure. ⛔ SO THE COUNT NOW LIVES IN EXACTLY ONE PLACE, the module comment, and
/// this sentence points at it instead of restating it: the second copy is what let the first one
/// rot unseen. Registered as `E77`.
#[test]
fn a_grant_that_is_neither_expired_nor_revoking_survives_the_sweep() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let Admission::Granted(_resident) = arbiter.admit(
        &preemptible("resident", 4_096, ComputeClass::Batch, 500),
        Millis::new(5_000),
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine");
    };

    let Admission::Queued(_) = arbiter.admit(
        &profile("late-comer", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::from_millis(4_999),
    ) else {
        panic!("nothing has expired and nothing was asked back, so the late-comer waits");
    };
    assert_eq!(arbiter.allocated(), Mib::new(4_096));
}
