//! PROPERTY 3 of §5.7: the gui dies holding a discretionary grant, and the sum comes back to the
//! baseline. The injection is the `ipc` port — `DyingGui`, which stops answering at an operation
//! chosen by the seed — and the reconciliation under test is `kernel::client::ClientGrants`,
//! finding P-16.
//!
//! ⛔ THE DEATH IS READ THROUGH THE PORT AND NOWHERE ELSE. `Ipc` has no disconnection event —
//! `accept`, `send`, `receive`, and nothing more — so what the core gets is
//! `Err(IpcError::Disconnected)` on `send` or on `receive`, and that `Err` is the ONLY thing that
//! triggers `on_disconnect` here. `DyingGui::has_died` is never called by this file, deliberately:
//! asking the fake whether it died would keep the property inside the bench, which is exactly the
//! defect P-16 names.
//!
//! ✅ MEASURED RATHER THAN PROMISED, AND WITH THE COMMAND THAT MEASURES THE CLAIM:
//! `grep -cE '\.has_died\(' ` on this file returns 0 — CALLS, not mentions. ⛔ THE OBVIOUS
//! COMMAND IS THE WRONG ONE: a bare `grep -c has_died` counts the paragraphs that SAY the fake is
//! not asked, this one included, so it answers a number greater than zero for a file that never
//! calls it. ⚠️ AND NO TALLY OF THOSE MENTIONS IS WRITTEN HERE, deliberately: a line that said
//! how many there are would be made false by adding it, which is what happened when this
//! paragraph first tried (gotcha #31). ⛔ AND THE OTHER DIRECTION, without which a zero measures
//! nothing: the anchored command returns 6 on `crates/simulator/tests/dying_gui.rs`, which is
//! where asking the fake is the right thing to do.
//!
//! ⛔ AND THE BASELINE IS NOT ZERO, which is the second half nothing else would catch. "The sum
//! comes back to the baseline" is green for a reconciliation that releases EVERYTHING when the
//! baseline is zero — that is `M2` of the commit 9a applied to a campaign — so two grants are in
//! the books before the dying gui says anything, and they are DIFFERENT KINDS of witness on
//! purpose:
//!
//! - the CORE's own presentation quota (ADR-0033, and `daemon` mounts two), which is never
//!   registered in `ClientGrants` at all: no disconnection can reach it, and every run redeems it
//!   afterwards to say so;
//! - a SECOND CLIENT that is registered and does NOT die. ⛔ IT IS THE ONE THE MUTATION CAN
//!   REACH, and that is why it is here: a reconciliation that released every pair it holds
//!   instead of that client's would leave the core's quota alone and take THIS one, so without it
//!   the campaign would carry an assertion no reachable defect could make red. Measured: with
//!   `on_disconnect`'s identity test removed, this campaign goes red and does so on the sum.
//!
//! ⚠️ WHAT THIS CAMPAIGN DOES NOT HOLD, said rather than left to be assumed. It runs at ONE
//! INSTANT: nothing here advances a clock, so a gui that dies AFTER its own window — where the
//! reconciliation meets `Released::AlreadyCollected` rather than `Now` — is not a world this
//! sweep contains. That road is held by
//! `crates/kernel/tests/client_grants.rs::a_disconnect_after_the_window_reports_already_collected`.
//! Adding a time axis here would change no assertion below: the sum returns to the baseline
//! whether the reservation came back now or had already been swept.

use std::collections::BTreeSet;

use kernel::arbiter::{
    Admission, Arbiter, ArbiterId, ComputeClass, Grant, Mib, Preemption, ReleaseError, Released,
    RemotePolicy, ResourceProfile, VramPolicy,
};
use kernel::client::ClientGrants;
use kernel::parameters::Parameters;
use kernel::ports::ipc::{ClientId, Ipc, IpcError};
use kernel::time::{Millis, Monotonic};
use kernel::wire::ipc::{GrantRequest, IpcMessage, Verdict};
use simulator::ipc::DyingGui;

const TURN_LIMIT: u64 = 10_000;

/// The whole machine.
const TOTAL: Mib = Mib::new(8_192);

/// The core's own quota. ⛔ IT IS NEVER REGISTERED in `ClientGrants`, so no disconnection can
/// reach it whatever the reconciliation does — which is what makes redeeming it afterwards a
/// statement about the ARBITER rather than about the register.
const CORE_QUOTA: Mib = Mib::new(1_024);

/// What a second, still-connected client holds. ⛔ IT IS REGISTERED, and it is the witness a
/// mutation can reach: a reconciliation that gave back every pair instead of the dead client's
/// takes this one and the sum stops matching the baseline.
const STANDING_QUOTA: Mib = Mib::new(1_024);

/// The two together: what the books hold before the dying gui asks for anything, and what they
/// must hold again after it is gone.
const BASELINE: Mib = Mib::new(CORE_QUOTA.get() + STANDING_QUOTA.get());

/// The window the two standing grants declare — long enough that nothing here ever collects them.
const FOREVER: Millis = Millis::new(1_000_000);

/// The window a gui declares.
const GUI_WINDOW: Millis = Millis::new(5_000);

const GUI: ClientId = ClientId::new(1);

/// The client that connects, is granted, and is still there when the other one dies.
const STANDING: ClientId = ClientId::new(2);

/// How many operations the path below performs on the port: `receive`, `send`, `receive`. ⛔ IT
/// IS THE PREMISE THE DEATH POINT IS DRAWN AGAINST, and it is held rather than trusted: the
/// oracle below asserts that EVERY seed reaches its point, so a path that performed fewer would
/// leave the tail of the range firing on nothing (gotcha #17).
const OPERATIONS: u64 = 3;

/// How many seeds the SHORT campaign sweeps. ⛔ FIXED AND VERSIONED WITH THIS FILE, never drawn
/// from the clock or from an environment variable: constraint 7 of §11, so two runs of the gate
/// sweep the same seeds. It is the figure `arbiter_campaign.rs` already uses, and the space here
/// is far smaller than that one's.
const SHORT_CAMPAIGN_SEEDS: u64 = 2_000;

/// What a gui may ask for. ⛔ THE FOUR ARE SIZED AGAINST `TOTAL` AND `CORE_QUOTA` SO THAT THE
/// ADMISSION REALLY DECIDES — gotcha #17 — and each buys a different arm: 1 024 fits with room to
/// spare, 6 144 fits EXACTLY (2 048 + 6 144 = 8 192, so the guard `allocated + asked > ceiling` is
/// exercised at equality and not only well inside it), 7 168 fits the machine but not the moment
/// and is QUEUED, and 12 288 is bigger than the whole machine and is REFUSED.
const ASKS: [Mib; 4] = [
    Mib::new(1_024),
    Mib::new(6_144),
    Mib::new(7_168),
    Mib::new(12_288),
];

/// How many DISTINCT worlds this scenario can produce at all.
///
/// ⛔ IT IS THE NON-VACUITY CONSTANT, and it is a CHANGE DETECTOR on the shape of the scenario —
/// the posture of `EXPECTED_OUTCOMES` in `arbiter_campaign.rs` and of `EXPECTED_DOUBT_SETS` in
/// `dst_campaign.rs`. Without it, the reason for `SHORT_CAMPAIGN_SEEDS` — "this many seeds see
/// the whole space" — would be a sentence in a comment that no run can contradict.
///
/// ⛔ HOW ONE REMEDIES A RED HERE, and it is the instruction and not a note: RE-MEASURE THE SPACE
/// AND RE-CHOOSE THESE NUMBERS. Editing this constant until the bar goes green is gotcha #25, and
/// it destroys the only thing the constant buys. What invalidates it, named so that a red is read
/// as a decision rather than as a defect: a change to `ASKS`, to `OPERATIONS`, to `CORE_QUOTA` or
/// to `TOTAL`.
///
/// ✅ MEASURED ON 2026-09-02, not deduced: the space is `ASKS.len()` asks × `OPERATIONS` death
/// points = 12, and the sweep reaches all twelve. The verdict is a function of the ask, so it
/// adds no dimension of its own; the death point does, because a gui killed at operation 0 never
/// asked at all and holds nothing to give back.
const EXPECTED_WORLDS: usize = 12;

/// The answer the core reached, flattened so that a world can be counted.
///
/// ⚠️ A LOCAL ENUM AND NOT `Verdict`, and it is not duplication for its own sake: `Verdict`
/// derives neither `Ord` nor `PartialOrd`, a `BTreeSet` wants both, and adding a derive to a
/// shipped type for the convenience of a bench is the trade `ports::process` refused when `Grant`
/// was asked for a `Debug`. `NeverAsked` is the state `Verdict` cannot spell at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Answer {
    /// The gui died before its request ever crossed the port.
    NeverAsked,
    Granted,
    Queued,
    Refused,
}

/// What one run observed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Observed {
    /// What the gui asked for, whether or not it lived to ask.
    asked: u64,
    answer: Answer,
    /// The operation the death landed on, as the fake was TOLD — not as the port reported it.
    /// ⚠️ IT IS NOT AN ORACLE: what it feeds is the world count, and the DIAGNOSTIC of the
    /// per-seed assertion below, which names it when the port never reported a death. Nothing
    /// asserts ON it, because "the fake was told to die at 2" is a fact about the fake.
    death_at: u64,
    /// How many grants the reconciliation handed back.
    reconciled: usize,
    /// Whether the death reached the core AS AN `Err(Disconnected)` FROM THE PORT, written in
    /// the `Err(IpcError::Disconnected)` arms and nowhere else.
    ///
    /// ⛔ IT IS `true` ON EVERY `Observed` THAT LEAVES `run`, AND THAT IS NOT A DEFECT — it is
    /// what the per-seed assertion below MAKES true by refusing anything else. So the second half
    /// of `E156` ①'s oracle — "the death was seen from the port" — is held THERE, on every seed,
    /// and testing it again after the fact would be testing the assertion rather than the world.
    /// ⚠️ THE CONSEQUENCE IS WRITTEN DOWN BECAUSE IT IS EASY TO GET WRONG TWICE: any aggregate
    /// that reads this field is reading a constant, and a conjunct on it is dead weight. One
    /// stood in `property_3` until the second review round.
    death_seen_from_port: bool,
}

/// The seed the death point is drawn from. ⛔ IT IS DERIVED, and it is a DIFFERENT mixing from
/// the one that chooses the ask: two draws taken from the same number move together, and the
/// campaign would then explore a DIAGONAL of the space instead of the space (decision D2 of the
/// milestone 4 plan). ✅ Held rather than argued — with the two mixings made equal the world
/// count collapses and `EXPECTED_WORLDS` goes red.
fn death_seed(seed: u64) -> u64 {
    seed.wrapping_mul(0xBF58_476D_1CE4_E5B9)
}

/// What this seed's gui asks for, from the OTHER mixing.
fn ask(seed: u64) -> Mib {
    ASKS[((seed.wrapping_mul(0x94D0_49BB_1331_11EB) >> 33) as usize) % ASKS.len()]
}

fn new_arbiter() -> Arbiter {
    Arbiter::new(
        Parameters::new(TURN_LIMIT, TOTAL, ArbiterId::new(1)),
        VramPolicy::Remote(RemotePolicy),
    )
}

/// A standing grant: one of the two the books hold before the dying gui says anything.
///
/// ⚠️ `Admission` has no `Debug`, so the `let … else` is not a style: `expect` does not exist
/// on it.
fn standing_grant(arbiter: &mut Arbiter, name: &'static str, reserved: Mib) -> Grant {
    let profile = ResourceProfile {
        name,
        reserved_vram: reserved,
        compute_class: ComputeClass::Realtime,
        preemption: Preemption::Never,
    };
    let Admission::Granted(grant) = arbiter.admit(&profile, FOREVER, Monotonic::ORIGIN) else {
        panic!("the standing quota {name} of {reserved:?} fits an empty machine of {TOTAL:?}");
    };
    grant
}

/// One world: the core talks to a gui that will die, and reconciles when the port says so.
///
/// ⛔ THE CORE NAMES THE PROFILE AND THE GUI DOES NOT — decision D16, and `GrantRequest` has no
/// `name` field precisely so that it cannot. What crosses the wire is the RESOURCE the gui wants;
/// what the arbiter's books record is a name the core chose.
fn run(seed: u64) -> Observed {
    let now = Monotonic::ORIGIN;
    let mut arbiter = new_arbiter();
    let mut clients = ClientGrants::new();

    let core = standing_grant(&mut arbiter, "core-presentation", CORE_QUOTA);
    clients.register(
        STANDING,
        standing_grant(&mut arbiter, "gui-standing", STANDING_QUOTA),
    );
    let baseline = arbiter.allocated();
    assert_eq!(
        baseline, BASELINE,
        "seed {seed}: the baseline is not what the two standing grants asked for"
    );

    let asked = ask(seed);
    let request = GrantRequest {
        reserved_vram: asked,
        // ⚠️ THE LANE AND THE PREEMPTION ARE FIXED, and the preemption is INERT rather than
        // uniform by oversight: this campaign runs under `VramPolicy::Remote`, whose
        // `may_make_room()` answers `false`, so nothing is ever asked back and no grant reaches
        // `Revoking`. It is `After` and not `Never` because §5.7 says DISCRETIONARY: what the
        // property is about is a grant the core could have taken back, as against the permanent
        // quota the baseline is.
        compute_class: ComputeClass::Interactive,
        preemption: Preemption::After(Millis::new(500)),
    };
    let mut gui = DyingGui::from_seed(GUI, request, death_seed(seed), OPERATIONS);

    let mut observed = Observed {
        asked: asked.get(),
        answer: Answer::NeverAsked,
        death_at: gui.dies_at(),
        reconciled: 0,
        death_seen_from_port: false,
    };

    let client = gui.accept().unwrap_or_else(|| {
        panic!("seed {seed}: the gui never connected, so nothing below happened at all")
    });

    // ⛔ THE WHOLE OF THE CORE'S SIDE, and every arm that meets a `Disconnected` reconciles and
    // stops. A `Disconnected` is PERMANENT (the fake's own bench holds that), so carrying on
    // would be the core talking to a process it has just buried.
    //
    // ⛔ AND THE STATE IS `observed.death_seen_from_port` ITSELF, not a local copied into it
    // afterwards: the field has to be written where the `Err` is READ, or it stops being evidence
    // of anything.

    // Operation 0: what the gui has to say.
    match gui.receive(client) {
        Ok(Some(bytes)) => {
            let Ok(IpcMessage::Request(asked_for)) = IpcMessage::decode(&bytes) else {
                panic!("seed {seed}: the gui's request did not decode as a request");
            };
            let profile = ResourceProfile {
                name: "gui-request",
                reserved_vram: asked_for.reserved_vram,
                compute_class: asked_for.compute_class,
                preemption: asked_for.preemption,
            };
            let verdict = match arbiter.admit(&profile, GUI_WINDOW, now) {
                Admission::Granted(grant) => {
                    // ⛔ REGISTERED BEFORE THE VERDICT IS SENT, and the order is the answer and
                    // not a preference: the grant exists from the instant `admit` returns it, so
                    // a gui that died ON the `send` would otherwise hold a reservation nobody
                    // could give back. That world is a third of this campaign.
                    clients.register(client, grant);
                    observed.answer = Answer::Granted;
                    Verdict::Granted
                }
                Admission::Queued(_) => {
                    observed.answer = Answer::Queued;
                    Verdict::Queued
                }
                Admission::Refused { asked, ceiling } => {
                    observed.answer = Answer::Refused;
                    Verdict::Refused { asked, ceiling }
                }
            };
            let Ok(bytes) = IpcMessage::Verdict(verdict).encode() else {
                panic!("seed {seed}: the verdict did not encode");
            };
            // Operation 1: the answer goes back.
            if gui.send(client, &bytes) == Err(IpcError::Disconnected) {
                observed.death_seen_from_port = true;
            }
        }
        Ok(None) => panic!("seed {seed}: the gui was accepted and had nothing to say"),
        Err(IpcError::Disconnected) => observed.death_seen_from_port = true,
        Err(IpcError::MalformedMessage) => {
            panic!("seed {seed}: this fake never sends bytes it did not encode itself")
        }
    }

    // Operation 2: the core polls again, which is where a gui that survived the first two dies.
    if !observed.death_seen_from_port {
        match gui.receive(client) {
            Ok(_) => {}
            Err(IpcError::Disconnected) => observed.death_seen_from_port = true,
            Err(IpcError::MalformedMessage) => {
                panic!("seed {seed}: this fake never sends bytes it did not encode itself")
            }
        }
    }

    // ⛔ ORACLE ONE OF §5.7.1 — THE INJECTION FIRED — AND IT IS ASSERTED PER SEED, which is
    // stronger than any total taken afterwards: it holds on EVERY seed instead of on the sum, and
    // it is where the red of the "the fake never dies" mutation comes from.
    assert!(
        observed.death_seen_from_port,
        "seed {seed}: the gui was told to die at operation {} of {OPERATIONS} and the port never \
         said so, so this run injected nothing",
        observed.death_at
    );

    // ⛔ AND THIS IS THE RECONCILIATION, triggered by the `Err` above and by nothing else.
    let released = clients
        .on_disconnect(client, &mut arbiter, now)
        .unwrap_or_else(|error: ReleaseError| {
            panic!(
                "seed {seed}: the one arbiter of this scenario disowned a grant it issued itself \
                 ({error:?})"
            )
        });
    observed.reconciled = released.len();

    // ⛔ PROPERTY 3, ASSERTED ON EVERY SEED. The seed is in the message because §5.7.1 wants the
    // campaign to NAME THE SEED when it fails.
    assert_eq!(
        arbiter.allocated(),
        baseline,
        "seed {seed}: after the gui died the sum is {:?} and the baseline was {baseline:?}",
        arbiter.allocated()
    );

    // ⛔ AND THE STANDING CLIENT IS STILL REGISTERED, which the sum alone does not say: a
    // reconciliation that threw its PAIR away while releasing only the dead client's grants would
    // leave the books exactly right and the reservation unreachable for ever. It disconnects in
    // its turn and the grant comes home.
    assert_eq!(
        clients.on_disconnect(STANDING, &mut arbiter, now),
        Ok(vec![Released::Now(STANDING_QUOTA)]),
        "seed {seed}: the client that did not die lost its grant when the other one went"
    );

    // ⛔ AND THE CORE'S QUOTA, WHICH NO REGISTER EVER HELD, is untouched — the statement about
    // the arbiter rather than about the register. `AlreadyCollected` here would mean it had left
    // the books.
    assert_eq!(
        arbiter.release(core, now),
        Ok(Released::Now(CORE_QUOTA)),
        "seed {seed}: the core's own quota did not survive the reconciliation"
    );
    assert_eq!(
        arbiter.allocated(),
        Mib::ZERO,
        "seed {seed}: the books are not empty once every holder has given its grant back"
    );

    observed
}

/// ⛔ ORACLE ONE — THE INJECTION FIRED — IS NOT IN THIS FUNCTION, and saying so is the point.
/// It is the per-seed `assert!` inside `run`: every seed must see the port report the gui gone.
/// ⚠️ AN AGGREGATE `deaths == SHORT_CAMPAIGN_SEEDS` STOOD HERE AND WAS REMOVED, not reworded,
/// and a `deaths` counter went with it in the second round: both read a flag the per-seed
/// assertion has already refused to let be false, so neither could ever have gone red. Gotcha
/// #76 — subtract rather than rewrite better.
///
/// ⛔ ORACLE TWO — THERE WAS SOMETHING TO VERIFY, and it is a DIFFERENT claim. This is the lesson
/// milestone 4 learned THREE times, each time after closing the previous one: a campaign that
/// kills clients which never asked for anything compares EMPTY SETS, and "the sum came back to
/// the baseline" is green because it never moved.
///
/// ⛔ `E156` ① WANTS IT WRITTEN WHOLE — the gui obtained `Granted` before dying AND the death was
/// seen from the port — AND IT IS, BY TWO CHECKS RATHER THAN ONE CONJUNCTION. The second half is
/// the per-seed assertion in `run`, which is STRONGER than a conjunct here: it holds on EVERY
/// seed instead of on at least one, and it is where the red of the "the fake never dies" mutation
/// comes from. What is left for this function is the FIRST half.
/// ⚠️ A CONJUNCT ON `death_seen_from_port` STOOD HERE AND WAS REMOVED: with the per-seed
/// assertion above it, the field is constant `true` in every value this loop sees, so the
/// conjunction reduced to its left operand and read as a guarantee it was not giving.
#[test]
fn property_3_a_gui_that_dies_holding_a_grant_gives_it_back() {
    let started = std::time::Instant::now();
    let mut held_a_grant_and_died = 0u64;
    let mut reconciled = 0usize;

    for seed in 0..SHORT_CAMPAIGN_SEEDS {
        let observed = run(seed);
        // ⛔ NO CONJUNCT ON `death_seen_from_port`: `run` returns only values on which it is
        // `true`, so it would add nothing here. That half is held per seed, inside `run`.
        if observed.answer == Answer::Granted {
            held_a_grant_and_died += 1;
        }
        reconciled += observed.reconciled;
    }
    let elapsed = started.elapsed();

    assert!(
        held_a_grant_and_died > 0,
        "on no seed did the gui hold a grant when the port reported it gone: every one of the \
         {SHORT_CAMPAIGN_SEEDS} seeds died before it was granted anything, so the sum was \
         compared against a baseline it never left"
    );

    println!(
        "DST gui death: {held_a_grant_and_died} of {SHORT_CAMPAIGN_SEEDS} seeds had the gui \
         holding a grant when the port reported it gone, {reconciled} grants handed back, \
         {elapsed:?}"
    );
}

/// ⛔ THE SCENARIO REALLY MAKES THE ADMISSION DECIDE, and it is a third claim again: without it a
/// campaign whose every request fitted would sweep a world where the answer is always the same,
/// and the reconciliation would only ever be exercised on one kind of grant.
///
/// ⚠️ WHAT IT MEASURES IS THE WORLD SPACE: how many DISTINCT `Observed` the sweep produces. One
/// distinct world across two thousand seeds would mean the two mixings move together and the
/// campaign is one run repeated.
#[test]
fn the_campaign_sweeps_every_world_this_scenario_has() {
    let mut distinct = BTreeSet::new();
    let mut answers = BTreeSet::new();
    for seed in 0..SHORT_CAMPAIGN_SEEDS {
        let observed = run(seed);
        answers.insert(observed.answer);
        distinct.insert(observed);
    }

    assert!(
        distinct.len() > 1,
        "every seed produced the SAME world: the two mixings are moving together, so this \
         campaign is one run repeated {SHORT_CAMPAIGN_SEEDS} times"
    );
    assert_eq!(
        distinct.len(),
        EXPECTED_WORLDS,
        "the campaign saw {} of the {EXPECTED_WORLDS} worlds this scenario can produce — either \
         the scenario changed shape, or a sweep of {SHORT_CAMPAIGN_SEEDS} seeds no longer reaches \
         the end of the space, and BOTH numbers must be re-measured rather than this one edited",
        distinct.len()
    );
    assert_eq!(
        answers.len(),
        4,
        "the admission reached {answers:?} and not all four of NeverAsked, Granted, Queued and \
         Refused: some arm of `admit` is not being exercised at all"
    );
}
