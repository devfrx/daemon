//! The two VRAM policies (ADR-0006), and what tells them apart is ONE DECISION INSIDE THE
//! ADMISSION PATH: "a request does not fit. Can room be made?"
//!
//! ⛔ AND NO MODEL IS NEEDED FOR THAT, which is what makes this provable at milestone 5.
//! "Evicting a resident" IS "revoking a preemptible grant" -- a mechanism task 7 built
//! anyway -- so the two policies are exercised with synthetic grants declared by the bench.
//! Zero speculation.
//!
//! ⚠️ `MakeRoom` IS IMPORTED AND THE PLAN'S `use` LIST DID NOT HAVE IT. `name()` lives ONLY on
//! the trait -- the four `fn name(` in `crates/kernel/src/` are its declaration and its three
//! impls, and there is no inherent one -- so calling it needs the trait in scope. ✅ MEASURED by
//! taking the import out again once the module existed: four times
//! `` error[E0599]: no method named `name` found ``, with
//! `` help: items from traits can only be used if the trait is in scope ``. Registered as `E88`.

use kernel::arbiter::{
    Admission, Arbiter, ComputeClass, LocalPolicy, MakeRoom, Mib, Preemption, RemotePolicy,
    ResourceProfile, VramPolicy,
};
use kernel::parameters::Parameters;
use kernel::time::{Millis, Monotonic};

const TURN_LIMIT: u64 = 10_000;
const LONG: Millis = Millis::new(1_000_000);

fn preemptible(name: &'static str, vram: u64, lane: ComputeClass) -> ResourceProfile {
    ResourceProfile {
        name,
        reserved_vram: Mib::new(vram),
        compute_class: lane,
        preemption: Preemption::After(Millis::new(500)),
    }
}

fn arbiter(total: u64, policy: VramPolicy) -> Arbiter {
    Arbiter::new(Parameters::new(TURN_LIMIT, Mib::new(total)), policy)
}

/// ⛔ THE DEFAULT, AND IT IS NOT A DETAIL: ADR-0006 makes REMOTE the default, and reopening
/// that turns a coordinated swap from an exception into the normal case.
#[test]
fn the_remote_policy_does_not_make_room_it_queues() {
    let mut arbiter = arbiter(4_096, VramPolicy::Remote(RemotePolicy));
    let Admission::Granted(_resident) = arbiter.admit(
        &preemptible("resident", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine");
    };

    let outcome = arbiter.admit(
        &preemptible("newcomer", 4_096, ComputeClass::Interactive),
        LONG,
        Monotonic::ORIGIN,
    );

    assert!(matches!(outcome, Admission::Queued(_)));
    assert_eq!(arbiter.revoking(), 0, "REMOTE revokes nothing to make room");
    // ⚠️ THE FINAL ASSERTION OF THIS PROBE -- AND OF EVERY PROBE BELOW THAT CARRIES THE SAME
    // NOTE -- IS DOMINATED INSIDE ITS OWN PROBE, AND IT IS KEPT: the species of `E37` and `E79`.
    // It is not vacuous -- ISOLATED (the two assertions above deleted) it fires under a mutation
    // that stops `admit` checking the ceiling, `left: Mib(8192), right: Mib(4096)` -- but AT FULL
    // STRENGTH an assertion above it fires first in every row of the campaign, so it never
    // decides an outcome. ✅ MEASURED on 2026-08-20 by isolation, not reasoned, and the sample is
    // named because an exclusivity measured on a partial one reads as a guarantee (`E82`): five
    // mutations were run isolated -- the two policy answers, the sweep collecting a `Running`
    // grant, `promote` ignoring its capacity check, and `admit` ignoring its ceiling. What it
    // declares is the intent the probe's name carries: the books did not move. As `E93`.
    //
    // ⛔ AND THE ONE `allocated()` ASSERTION OF THIS FILE THAT IS *NOT* DOMINATED IS THE ONE THAT
    // IS NOT FINAL: the PRECONDITION of the partly-full probe at the foot of this file, which
    // under the sweep mutation is the assertion that fires -- the only one on `allocated()` in
    // this bench that ever decides an outcome. Registered as `E108`.
    assert_eq!(arbiter.allocated(), Mib::new(4_096));
}

/// ⛔ THE OTHER OBJECT, AND THE SAME CALL SITE. This is the difference ADR-0006 says must
/// NOT be an `if` planted in the middle of the admission: two objects with one interface
/// keep it in one place, where a conditional would drift invisibly.
#[test]
fn the_local_policy_asks_the_lower_lanes_back() {
    let mut arbiter = arbiter(4_096, VramPolicy::Local(LocalPolicy));
    let Admission::Granted(_resident) = arbiter.admit(
        &preemptible("resident", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine");
    };

    let outcome = arbiter.admit(
        &preemptible("newcomer", 4_096, ComputeClass::Interactive),
        LONG,
        Monotonic::ORIGIN,
    );

    // ⛔ STILL QUEUED, and that is the honest answer: the room is not free until the holder
    // hands it over. What LOCAL changed is that somebody was ASKED.
    assert!(matches!(outcome, Admission::Queued(_)));
    assert_eq!(arbiter.revoking(), 1, "LOCAL asked the Batch resident back");
    // Dominated inside this probe, and kept: the reason is written ONCE, beside the same
    // assertion in `the_remote_policy_does_not_make_room_it_queues`.
    assert_eq!(arbiter.allocated(), Mib::new(4_096), "nothing freed yet");
}

/// ⛔ AND WHAT THE ASKING BUYS, END TO END: past the grace the queued request is served.
/// Without this the probe above proves a marking that leads nowhere.
#[test]
fn under_the_local_policy_the_queued_request_is_served_past_the_grace() {
    let mut arbiter = arbiter(4_096, VramPolicy::Local(LocalPolicy));
    let Admission::Granted(_resident) = arbiter.admit(
        &preemptible("resident", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine");
    };
    let Admission::Queued(ticket) = arbiter.admit(
        &preemptible("newcomer", 4_096, ComputeClass::Interactive),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("queued");
    };

    let promoted = arbiter.promote(Monotonic::from_millis(501));

    assert_eq!(promoted.len(), 1);
    assert_eq!(promoted[0].ticket, ticket);
    // Dominated inside this probe, and kept: same reason, written once above.
    assert_eq!(arbiter.allocated(), Mib::new(4_096), "one grant, not two");
}

/// The counter-probe of the one above: under REMOTE the same clock advance serves NOBODY,
/// because nobody was ever asked back.
#[test]
fn under_the_remote_policy_the_same_clock_advance_serves_nobody() {
    let mut arbiter = arbiter(4_096, VramPolicy::Remote(RemotePolicy));
    let Admission::Granted(_resident) = arbiter.admit(
        &preemptible("resident", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine");
    };
    let Admission::Queued(_) = arbiter.admit(
        &preemptible("newcomer", 4_096, ComputeClass::Interactive),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("queued");
    };

    let promoted = arbiter.promote(Monotonic::from_millis(501));

    assert!(promoted.is_empty(), "REMOTE asked nobody, so nothing came free");
    // Dominated inside this probe, and kept: same reason, written once above.
    assert_eq!(arbiter.allocated(), Mib::new(4_096));
}

/// The name, so a journalled transition has something true to write down.
///
/// ⛔ AND IT IS READ THROUGH THE ARBITER TOO, WHICH IS A SECOND FACT: `Arbiter::policy` would
/// otherwise be born with no consumer at all -- and being `pub`, no warning would say so
/// (gotcha #46 from the wrong side). Reading the name off the arbiter proves what no other
/// probe here proves: that the arbiter KEPT the policy it was built with, instead of
/// defaulting to one of them. ✅ MEASURED on 2026-08-20, not argued: with `policy()` returning
/// a fresh `VramPolicy::Remote(RemotePolicy)` the ARBITER-LOCAL assertion below is the ONLY red
/// in the workspace, and without the two lines that read through the arbiter that mutation
/// would have been a live mutant. ⚠️ THE TWO HALVES SIT ON DIFFERENT AXES, which is why both
/// are here: collapsing the `VramPolicy` name dispatch instead kills the ENUM-LOCAL assertion
/// above, also alone.
#[test]
fn each_policy_names_itself() {
    assert_eq!(VramPolicy::Remote(RemotePolicy).name(), "remote");
    assert_eq!(VramPolicy::Local(LocalPolicy).name(), "local");

    assert_eq!(
        arbiter(4_096, VramPolicy::Remote(RemotePolicy)).policy().name(),
        "remote"
    );
    assert_eq!(
        arbiter(4_096, VramPolicy::Local(LocalPolicy)).policy().name(),
        "local"
    );
}

/// ⛔ THE MACHINE IS ONLY PARTLY FULL, AND THAT IS THE WHOLE POINT OF THIS PROBE. In the five
/// above, one resident of `4_096` fills a machine of `4_096` and the newcomer asks for `4_096`
/// too, so the four sizes the admission juggles -- `ceiling`, `allocated()`, `asked` and the
/// `needed` it COMPUTES -- all coincide. With all four equal, `needed` is indistinguishable
/// from any of the other three, and the first of the two arguments `admit` hands to `ask_back`
/// is held by nothing.
///
/// ✅ MEASURED AND NOT ARGUED, on 2026-08-20: with `ask_back(asked, ...)` in place of
/// `ask_back(needed, ...)` the WHOLE WORKSPACE stayed green -- 241 passed, 0 failed -- and that
/// mutant is not cosmetic. `ask_back` uses its first argument as the THRESHOLD
/// (`if reclaimable < needed { return Mib::ZERO }`), so asking for the whole request instead of
/// the shortfall makes `LocalPolicy` mark NOBODY, and degrade to `RemotePolicy` in silence,
/// every time the machine is only PARTLY full. Here the four sizes are `4_096`, `3_072`,
/// `2_048` and `1_024`: all different. Registered as `E97`.
#[test]
fn a_partly_full_machine_asks_back_the_need_and_not_the_whole_request() {
    let mut arbiter = arbiter(4_096, VramPolicy::Local(LocalPolicy));
    // ⛔ REALTIME, AND PREEMPTIBLE LIKE EVERY OTHER RESIDENT OF THIS FILE: what keeps this half
    // out of the reclaimable set is the LANE alone -- `askable` returns on `held.lane <= below`
    // before it ever reads `preemption`. And what makes the SHORTFALL smaller than the REQUEST
    // is a different fact again: the machine is only PARTLY full, so `allocated + asked -
    // ceiling` is 1_024 while `asked` is 2_048.
    //
    // ⚠️ IT WAS `Preemption::Never`, THROUGH A HELPER OF ITS OWN, UNTIL 2026-08-20, and that
    // helper was a live mutant the correction wave itself introduced -- measured, see `E105`.
    let Admission::Granted(_realtime) = arbiter.admit(
        &preemptible("realtime", 2_048, ComputeClass::Realtime),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("the realtime resident is seated");
    };
    let Admission::Granted(_batch) = arbiter.admit(
        &preemptible("batch", 1_024, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("the batch resident is seated");
    };
    assert_eq!(
        arbiter.allocated(),
        Mib::new(3_072),
        "PARTLY full: 3_072 of 4_096"
    );

    let outcome = arbiter.admit(
        &preemptible("newcomer", 2_048, ComputeClass::Interactive),
        LONG,
        Monotonic::ORIGIN,
    );

    assert!(matches!(outcome, Admission::Queued(_)));
    // ⛔ THE ASSERTION THAT TELLS `needed` FROM `asked`: the reclaimable is `1_024`, which
    // COVERS the shortfall of `1_024` and does NOT cover the request of `2_048`.
    assert_eq!(
        arbiter.revoking(),
        1,
        "the Batch resident is asked back for the SHORTFALL"
    );
}

/// ⛔ AND THE SECOND ARGUMENT `admit` COMPUTES IS THE LANE, which the five probes above cannot
/// hold either: with one resident and one newcomer the boundary never decides anything.
/// `askable` drops a holder when `held.lane <= below`, and the order is
/// `Realtime(0) < Interactive(1) < Batch(2)`, so passing a BETTER lane WIDENS the set of
/// victims. Wiring `ComputeClass::Realtime` in place of the requester's own lane would have the
/// arbiter evict an `Interactive` PEER for an `Interactive` request -- exactly what
/// `a_grant_in_the_asking_lane_itself_is_not_asked_back` exists to forbid.
///
/// ⚠️ AND THAT PROBE DOES NOT SEE IT, WHICH IS WHY THIS ONE IS HERE: it calls `ask_back`
/// DIRECTLY with explicit lanes, so it holds the boundary inside the mechanism and says nothing
/// about the WIRING outside it. ✅ MEASURED on 2026-08-20 with
/// `ask_back(needed, ComputeClass::Realtime, now)` in `admit`: the whole workspace stayed green,
/// 241 passed, 0 failed. Registered as `E97`.
///
/// ⛔ IT IS THE "DOES NOT FIRE WHERE IT MUST NOT" DIRECTION of the probe above, and the pair is
/// deliberate: no single scenario can hold both arguments, and the reason is measured rather
/// than asserted -- see `E98`.
#[test]
fn the_admission_asks_back_below_its_own_lane_and_spares_a_peer() {
    let mut arbiter = arbiter(4_096, VramPolicy::Local(LocalPolicy));
    // The Realtime resident, and it is PREEMPTIBLE for the same reason the peer below is: only
    // the LANE keeps either of them out of the reclaimable set (`E105`).
    let Admission::Granted(_realtime) = arbiter.admit(
        &preemptible("realtime", 1_024, ComputeClass::Realtime),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("the realtime resident is seated");
    };
    // ⛔ THE PEER, AND IT IS PREEMPTIBLE: nothing but the lane boundary protects it. A
    // non-preemptible peer would be spared for the wrong reason and the probe would pass
    // whatever `admit` passes as `below`.
    let Admission::Granted(_peer) = arbiter.admit(
        &preemptible("peer", 2_048, ComputeClass::Interactive),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("the peer is seated");
    };

    let outcome = arbiter.admit(
        &preemptible("newcomer", 2_048, ComputeClass::Interactive),
        LONG,
        Monotonic::ORIGIN,
    );

    // ⛔ NOBODY IS ASKED BACK, AND THE POLICY SAID YES. `LocalPolicy::may_make_room` answered
    // `true` and `admit` did call `ask_back`; what stopped it is the boundary -- the only
    // candidate is a PEER, so the reclaimable is `Mib::ZERO` and the read-only pass marks
    // nothing. Queued is the honest answer, not a degradation.
    assert!(matches!(outcome, Admission::Queued(_)));
    assert_eq!(
        arbiter.revoking(),
        0,
        "an Interactive peer is not evicted for an Interactive request"
    );
    // Dominated inside this probe, and kept: same reason, written once above.
    assert_eq!(arbiter.allocated(), Mib::new(3_072), "the books did not move");
}
