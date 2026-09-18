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
    Admission, Arbiter, ArbiterId, ComputeClass, LocalPolicy, MakeRoom, Mib, PolicyError,
    Preemption, RemotePolicy, ResourceProfile, VramPolicy, policy_now,
};
use kernel::parameters::Parameters;
use kernel::ports::journal::{Journal, JournalError, StepId};
use kernel::reconcile::{Resolution, steps_in_doubt};
use kernel::record::{
    Detail, EffectClass, PolicyDetail, Record, RecordError, RecordKind, RecordV1, Trust,
};
use kernel::time::{Millis, Monotonic};
use simulator::journal::{CrashingJournal, MemoryJournal};

const TURN_LIMIT: u64 = 10_000;

/// The gui tick this bench delivers. ⚠️ A LITERAL OF THIS BENCH, and it is inert here on
/// purpose: nothing in this file runs `kernel::serving::serve`, so nobody reads it -- but
/// `Parameters` carries every delivered value positionally, and §2.8.2 rule 2 forbids the kernel
/// to name a default.
///
/// ⛔ ZERO IS NOT A NEUTRAL VALUE WHERE IT IS READ: a zero tick makes `kernel::executor::nap`
/// behave as a yield (`Sleep::until`'s own rule), so a bench that really serves hands its own --
/// `crates/kernel/tests/serving.rs` does.
const GUI_TICK: Millis = Millis::new(0);
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
    Arbiter::new(
        Parameters::new(TURN_LIMIT, Mib::new(total), ArbiterId::new(1), GUI_TICK),
        policy,
    )
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

    assert!(
        promoted.is_empty(),
        "REMOTE asked nobody, so nothing came free"
    );
    // Dominated inside this probe, and kept: same reason, written once above.
    assert_eq!(arbiter.allocated(), Mib::new(4_096));
}

/// The name, so a journalled transition has something true to write down.
///
/// ⛔ AND IT IS READ THROUGH THE ARBITER TOO, WHICH IS A SECOND FACT: `Arbiter::policy` would
/// otherwise be born with no consumer at all -- and being `pub`, no warning would say so
/// (gotcha #46 from the wrong side). Reading the name off the arbiter proves that the arbiter
/// KEPT the policy it was built with, instead of defaulting to one of them. ✅ MEASURED on
/// 2026-08-20, not argued: with `policy()` returning a fresh `VramPolicy::Remote(RemotePolicy)`
/// the ARBITER-LOCAL assertion below goes red. ⚠️ THE TWO HALVES SIT ON DIFFERENT AXES, which
/// is why both are here: collapsing the `VramPolicy` name dispatch instead kills the ENUM-LOCAL
/// assertion above.
#[test]
fn each_policy_names_itself() {
    assert_eq!(VramPolicy::Remote(RemotePolicy).name(), "remote");
    assert_eq!(VramPolicy::Local(LocalPolicy).name(), "local");

    assert_eq!(
        arbiter(4_096, VramPolicy::Remote(RemotePolicy))
            .policy()
            .name(),
        "remote"
    );
    assert_eq!(
        arbiter(4_096, VramPolicy::Local(LocalPolicy))
            .policy()
            .name(),
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
    assert_eq!(
        arbiter.allocated(),
        Mib::new(3_072),
        "the books did not move"
    );
}

/// ⛔ THE ASSERTION IS ON THE ARCHIVE, NOT ON THE POLICY. "After the transition the policy is
/// the other one" is green with ZERO records written, and V6 is exactly the claim that
/// nothing happens before the intent is durable.
#[test]
fn a_policy_transition_writes_its_intent_before_its_outcome() {
    let mut journal = MemoryJournal::new();
    let mut arbiter = arbiter(4_096, VramPolicy::Remote(RemotePolicy));

    arbiter
        .set_policy(VramPolicy::Local(LocalPolicy), StepId::new(1), &mut journal)
        .expect("the journal accepts");

    assert_eq!(arbiter.policy().name(), "local");

    let entries = journal.replay().expect("the archive reads back");
    // ⛔ RECALL OF 2026-09-18 — IT WAS `2`, "an intent AND an outcome". `Arbiter::set_policy` writes a
    // THIRD record between them since the `Policy` species arrived: the structured half
    // `crate::arbiter::policy_now` reads back, which the `reason` could only say in prose.
    assert_eq!(entries.len(), 3, "an intent, the policy note, AND an outcome");

    let records: Vec<RecordV1> = entries
        .iter()
        .map(
            |(_, bytes)| match Record::decode(bytes).expect("our own bytes") {
                Record::V1(record) => record,
            },
        )
        .collect();

    assert_eq!(
        records.iter().map(|r| r.kind()).collect::<Vec<_>>(),
        vec![
            RecordKind::Intent,
            RecordKind::Policy,
            RecordKind::Outcome
        ]
    );
    // ⛔ THE NAME IS THE POINT, AND IT IS WHY `MakeRoom::name` EXISTS: a record that only said
    // "policy transition" would make the two directions indistinguishable in the archive, and
    // the archive is the only thing that survives. The OTHER direction is asserted in
    // `a_transition_names_the_policy_it_moves_to` -- one alone would be satisfied by a
    // hard-coded "local".
    assert!(records.iter().all(|r| r.reason() == "local"));
    // ⛔ THE LABEL AND THE PAYLOAD ARE CONTRACT, NOT DECORATION: the doc of `set_policy` says no
    // external byte reaches this record, so the payload is EMPTY and the label is
    // `Trust::Instruction`. ✅ MEASURED: mutating either field is caught HERE AND NOWHERE ELSE
    // -- rows 8 and 9 of the campaign, 247 passed / 1 failed each, this probe alone.
    assert!(records.iter().all(|r| r.trust() == Trust::Instruction));
    assert!(records.iter().all(|r| r.payload().is_empty()));
}

/// ⛔ THE OTHER DIRECTION, AND IT IS NOT SYMMETRY FOR ITS OWN SAKE: with only the probe above,
/// a `transition_record` that hard-coded "local" would stay green. Two directions kill every
/// constant.
#[test]
fn a_transition_names_the_policy_it_moves_to() {
    let mut journal = MemoryJournal::new();
    let mut arbiter = arbiter(4_096, VramPolicy::Local(LocalPolicy));

    arbiter
        .set_policy(
            VramPolicy::Remote(RemotePolicy),
            StepId::new(1),
            &mut journal,
        )
        .expect("the journal accepts");

    let entries = journal.replay().expect("the archive reads back");
    assert_eq!(entries.len(), 3);
    for (_, bytes) in &entries {
        let Record::V1(record) = Record::decode(bytes).expect("our own bytes");
        assert_eq!(record.reason(), "remote");
        // ⛔ RECALL OF 2026-09-18 — THIS ASSERTED `None` ON BOTH, saying "a policy transition declares
        // no structured species". It declares one now, and the sentence is REWRITTEN rather than
        // annotated beside itself (finding `A-2`). What the measurement behind it bought is KEPT
        // and made sharper: `E79` found that turning the detail of the intent or the outcome to
        // `Some(..)` left the whole workspace green, so the two that still carry `None` are still
        // held here — and the third is now REQUIRED to carry the right one.
        match record.kind() {
            RecordKind::Policy => assert_eq!(
                record.detail(),
                Some(&Detail::Policy(PolicyDetail { local: false })),
                "the note carries the policy it moved TO, structured"
            ),
            _ => assert_eq!(
                record.detail(),
                None,
                "the intent and the outcome declare no structured species: `E79`"
            ),
        }
    }
}

/// ⛔ THE HALF THAT V6 IS ACTUALLY ABOUT: a journal that refuses the intent means the
/// transition DOES NOT HAPPEN. Without this the write-ahead is decoration.
#[test]
fn a_refused_intent_leaves_the_policy_where_it_was() {
    let mut journal = CrashingJournal::falling_at(0);
    let mut arbiter = arbiter(4_096, VramPolicy::Remote(RemotePolicy));

    let outcome = arbiter.set_policy(VramPolicy::Local(LocalPolicy), StepId::new(1), &mut journal);

    assert!(outcome.is_err());
    assert_eq!(
        arbiter.policy().name(),
        "remote",
        "nothing executes before the intent is durable"
    );
}

/// ⛔ AND A TRANSITION CUT IN HALF LEAVES A RECONCILABLE STEP -- which is DST property 4,
/// asserted here on ONE constructed state so that the campaign of task 12 has a shape to
/// look for rather than a hope.
#[test]
fn a_transition_cut_between_intent_and_outcome_leaves_the_step_in_doubt() {
    let mut journal = CrashingJournal::falling_at(1);
    let mut arbiter = arbiter(4_096, VramPolicy::Remote(RemotePolicy));

    let outcome = arbiter.set_policy(VramPolicy::Local(LocalPolicy), StepId::new(7), &mut journal);
    assert!(outcome.is_err(), "the outcome never reached the archive");

    let survivor = journal.into_survivor();
    let doubts = steps_in_doubt(&survivor).expect("the archive reads back");

    assert_eq!(doubts.len(), 1);
    assert_eq!(doubts[0].step, StepId::new(7));
    assert_eq!(
        doubts[0].resolution,
        Resolution::RunAgain,
        "a policy transition is idempotent: re-running converges"
    );
}

/// The counter-probe of the one above, and it is the direction that is skipped: WITHOUT a
/// crash, no step is in doubt. Otherwise "there is a doubt" would be satisfied by an arbiter
/// that never writes an outcome at all.
///
/// ⚠️ DOMINATED IN EVERY ROW OF THE CAMPAIGN, and kept: the species of `E93`. It goes red only
/// in rows 1 and 5, where `a_policy_transition_writes_its_intent_before_its_outcome` is red too
/// (244 passed / 4 failed, both). It decides no row on its own; what it holds is the
/// direction.
#[test]
fn without_a_crash_a_transition_leaves_no_step_in_doubt() {
    let mut journal = MemoryJournal::new();
    let mut arbiter = arbiter(4_096, VramPolicy::Remote(RemotePolicy));

    arbiter
        .set_policy(VramPolicy::Local(LocalPolicy), StepId::new(7), &mut journal)
        .expect("the journal accepts");

    assert!(steps_in_doubt(&journal).expect("reads back").is_empty());
}

/// ⛔ AN ARCHIVE WITH NO TRANSITION NAMES NO POLICY, AND IT IS THE DIRECTION D27 EXISTS FOR: a
/// `policy_now` that answered `Remote` here would be naming the default inside the projection,
/// which is the one thing ADR-0034 forbids the kernel. It is asserted FIRST because it is the
/// state every archive starts in.
#[test]
fn an_archive_with_no_transition_names_no_policy() {
    let mut journal = MemoryJournal::new();

    journal
        .intent(
            StepId::new(1),
            &Record::V1(RecordV1::intent(
                EffectClass::Idempotent,
                Trust::Instruction,
                Vec::new(),
                "a step that is not a transition",
            ))
            .encode(),
        )
        .expect("the memory journal accepts");

    // ⚠️ THROUGH `.map(.. name())` LIKE ITS THREE SISTERS: `VramPolicy` derives neither `PartialEq`
    // nor `Debug` — its variants carry the policy objects themselves — and a derive added for a
    // bench would be a kernel type widened for a test.
    assert_eq!(
        policy_now(&journal)
            .expect("the archive reads back")
            .map(|policy| policy.name()),
        None,
        "no transition was written, and the default is not the kernel's to name"
    );
    // ⛔ AND THE ARCHIVE IS NOT EMPTY, which is what makes this probe about the FILTER rather than
    // about an empty loop: a `policy_now` that answered `None` because it never looked would pass
    // on an empty journal and fail here.
    assert_eq!(journal.replay().expect("the archive reads back").len(), 1);
}

/// ⛔ THE TRANSITION IS READ BACK, THROUGH THE REAL WRITER. It calls `set_policy` and not a
/// hand-built record, which is what makes the pair `policy_note`/`policy_now` one artefact: a probe
/// that wrote its own record would be a second copy of the format, green on the day the two drift.
#[test]
fn the_policy_the_archive_names_is_the_one_the_transition_moved_to() {
    let mut journal = MemoryJournal::new();
    let mut arbiter = arbiter(4_096, VramPolicy::Remote(RemotePolicy));

    arbiter
        .set_policy(VramPolicy::Local(LocalPolicy), StepId::new(1), &mut journal)
        .expect("the journal accepts");

    assert_eq!(
        policy_now(&journal)
            .expect("the archive reads back")
            .map(|policy| policy.name()),
        Some("local"),
        "the archive names the policy the transition moved to"
    );
}

/// ⛔ THE OTHER DIRECTION OF THE SAME RULE, and without it a `policy_now` that hard-coded `Local`
/// would stay green — the lesson `a_transition_names_the_policy_it_moves_to` already paid for. Two
/// directions kill every constant.
#[test]
fn a_transition_back_to_remote_is_read_back_as_remote() {
    let mut journal = MemoryJournal::new();
    let mut arbiter = arbiter(4_096, VramPolicy::Local(LocalPolicy));

    arbiter
        .set_policy(
            VramPolicy::Remote(RemotePolicy),
            StepId::new(1),
            &mut journal,
        )
        .expect("the journal accepts");

    assert_eq!(
        policy_now(&journal)
            .expect("the archive reads back")
            .map(|policy| policy.name()),
        Some("remote"),
    );
}

/// ⛔ THE LAST TRANSITION AND NOT THE FIRST, AND IT IS THE DIRECTION THAT GETS FORGOTTEN: with only
/// the three probes above, a `policy_now` that `break`s on the first `Policy` record it meets is
/// green on every one of them, because each writes exactly one transition. ✅ MEASURED rather than
/// feared: with `current = Some(..)` replaced by an early `return`, this probe alone goes red.
#[test]
fn the_archive_names_the_last_transition_and_not_the_first() {
    let mut journal = MemoryJournal::new();
    let mut arbiter = arbiter(4_096, VramPolicy::Remote(RemotePolicy));

    arbiter
        .set_policy(VramPolicy::Local(LocalPolicy), StepId::new(1), &mut journal)
        .expect("the journal accepts");
    arbiter
        .set_policy(
            VramPolicy::Remote(RemotePolicy),
            StepId::new(2),
            &mut journal,
        )
        .expect("the journal accepts");

    assert_eq!(
        policy_now(&journal)
            .expect("the archive reads back")
            .map(|policy| policy.name()),
        Some("remote"),
        "a policy set and then set back is not the state NOW"
    );
}

/// Opens the step the notes below sit upon. ⚠️ `Journal::note` REFUSES A STEP WITHOUT AN INTENT
/// (`OutOfOrder`), so every probe that writes a note writes this first — the shape the four benches
/// that carry an `open_the_step` already have, each defining its own.
fn open_the_step(journal: &mut MemoryJournal, step: StepId) {
    let intent = Record::V1(RecordV1::intent(
        EffectClass::Idempotent,
        Trust::Instruction,
        Vec::new(),
        "the step this record sits upon",
    ))
    .encode();
    journal.intent(step, &intent).expect("intent");
}

/// A journal whose `replay` refuses outright, for the road into `PolicyError::Journal`.
struct ReplayRefusingJournal;

impl Journal for ReplayRefusingJournal {
    fn intent(&mut self, _step: StepId, _record: &[u8]) -> Result<(), JournalError> {
        Ok(())
    }
    fn outcome(&mut self, _s: StepId, _r: &[u8]) -> Result<(), JournalError> {
        Ok(())
    }
    fn note(&mut self, _s: StepId, _r: &[u8]) -> Result<(), JournalError> {
        Ok(())
    }
    fn read_back(&self, _s: StepId) -> Result<Vec<u8>, JournalError> {
        Err(JournalError::Missing)
    }
    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError> {
        Err(JournalError::NotDurable)
    }
    fn prune(&mut self, _s: StepId) -> Result<(), JournalError> {
        Ok(())
    }
}

/// ⛔ THE ROAD INTO `PolicyError::Journal`, and it is the reason that variant exists. A `policy_now`
/// that folded a refusing archive into `Ok(None)` would tell the composition root "nobody ever
/// changed the policy" about an archive it simply could not read, and the daemon would start on the
/// default — which is the silent partial truth `PolicyError` exists to prevent.
#[test]
fn an_archive_that_will_not_be_read_is_an_error_and_not_an_absent_policy() {
    assert_eq!(
        policy_now(&ReplayRefusingJournal).map(|found| found.map(|policy| policy.name())),
        Err(PolicyError::Journal(JournalError::NotDurable))
    );
}

/// ⛔ THE FIRST ROAD INTO `PolicyError::Record`: bytes that are not a record at all. A `continue` in
/// place of the `?` on `Record::decode` passes every other probe in this file, and it would let a
/// corrupt archive read as a clean `None` while a transition sits in the very bytes that would not
/// decode.
///
/// ⚠️ THE BYTES ARE WRITTEN THROUGH THE PORT, which takes `&[u8]` and validates nothing — road A4 of
/// `kernel::boundary`, which already declares that nothing requires every write to the journal to
/// be a `Record`.
#[test]
fn a_record_this_build_cannot_read_stops_the_answer() {
    let mut journal = MemoryJournal::new();
    let step = StepId::new(1);
    open_the_step(&mut journal, step);
    journal
        .note(step, b"not a record of any version")
        .expect("note");

    assert_eq!(
        policy_now(&journal).map(|found| found.map(|policy| policy.name())),
        Err(PolicyError::Record(RecordError::Malformed))
    );
}

/// ⛔ THE SECOND ROAD INTO `PolicyError::Record`, AND IT IS REACHABLE rather than declared
/// impossible — the sentence that variant's doc carries, held by nothing until this probe. In
/// SOURCE it is unpronounceable: `RecordV1::policy` takes its `PolicyDetail` by value, so the
/// `kind`/`detail` pair cannot be split. From BYTES it is one byte's work.
///
/// ⛔ AND AN `else { continue }` IN PLACE OF THE `return` WOULD ANSWER `None` — "nobody ever changed
/// it" — about an archive that holds a transition record naming no policy. That is the one answer
/// `policy_now` must never give, because the caller starts on the default when it hears it.
///
/// ⚠️ AND IT DOES NOT PIN `RecordKind::Policy`'s WIRE INDEX, though the `07` below invites that
/// reading: the index is held by the FROZEN RECORD ALONE, which reads it back. ✅ MEASURED on
/// 2026-09-18 rather than argued — with `Policy` moved to `#[n(9)]` in `src/record.rs` this whole
/// bench stays GREEN and only `tests/frozen_bytes.rs` goes red, because once the index moves the
/// relabelled bytes decode as nothing at all and this probe arrives at its `Err` through the FIRST
/// road, `Record::decode` failing, instead of through its own. What this probe holds is the `else`
/// branch, and nothing else in the workspace holds it.
#[test]
fn a_policy_record_whose_detail_is_not_a_policy_is_an_error() {
    let mut journal = MemoryJournal::new();
    let step = StepId::new(1);
    open_the_step(&mut journal, step);

    let mut bytes = Record::V1(RecordV1::note(
        EffectClass::Idempotent,
        Trust::Instruction,
        Vec::new(),
        "a note, about to be relabelled from outside",
    ))
    .encode();
    // Index 0 of `RecordV1` sits at byte 4 — `tests/frozen/record_v1.map`, checked by
    // `every_field_sits_at_the_offset_the_map_gives_it`. `07` is `RecordKind::Policy`.
    bytes[4] = 7;
    journal.note(step, &bytes).expect("note");

    assert_eq!(
        policy_now(&journal).map(|found| found.map(|policy| policy.name())),
        Err(PolicyError::Record(RecordError::Malformed))
    );
}
