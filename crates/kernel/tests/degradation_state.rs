//! §6.7 — the degradation state is DERIVED and recomputed, never authoritative of itself
//! (ADR-0019).
//!
//! ⛔ EVERY PROBE THAT DISPATCHES OPENS THE STEP FIRST, AND THAT IS NOT SETUP NOISE. `dispatch`
//! writes a NOTE, and `Journal::note` says of itself that "a note for a step with NO INTENT is
//! `OutOfOrder`. A note is an annotation UPON something, and a step nobody opened is not
//! something" — `MemoryJournal` enforces it. The dictated probes started from an empty journal
//! and could not pass: measured before this file was written, `dispatch: OutOfOrder`. The helper
//! is the one `tests/gateway_decisor.rs` and `tests/permission_triple.rs` already carry, and the
//! asymmetry is the same: the step belongs to the CALLER, never to the mechanism annotating it.
//!
//! ⛔ AND `vram_exhausted` GETS A PROBE OF ITS OWN, which the dictated three did not give it.
//! Three of them exercise `routing_degraded` and NOTHING exercised the other half of the type —
//! nailed to `false` it would have been a live mutant on the whole file. The saturated arbiter
//! below is what makes that mutation red.
//!
//! ⛔ AND "THE LAST ROUTING" IS HELD BY A PROBE AND NOT BY A SENTENCE. With one dispatch per
//! journal, "the last one wins" and "any one wins" are the same answer, so the rule the
//! production comment states would have been an intention. The probe that dispatches TWICE upon
//! one open step is what separates them.

use kernel::arbiter::{
    Admission, Arbiter, ArbiterId, ComputeClass, Mib, Preemption, RemotePolicy, ResourceProfile,
    VramPolicy,
};
use kernel::degradation::{DegradationError, degradation_now};
use kernel::gateway::{Candidate, Constraint, dispatch, resolve};
use kernel::parameters::Parameters;
use kernel::ports::journal::{Journal, JournalError, StepId};
use kernel::record::{EffectClass, Record, RecordError, RecordV1, Trust};
use kernel::time::{Millis, Monotonic};
use simulator::journal::MemoryJournal;

const TURN_LIMIT: u64 = 10_000;

/// A machine with room to spare, for every probe whose subject is the ROUTING half.
const ROOMY: Mib = Mib::new(16_384);

/// The window every probe here uses: none of them is about expiry.
const LONG: Millis = Millis::new(1_000_000);

/// ⛔ THE ONE CANDIDATE EVERY ROUTING PROBE RESOLVES, and it is dear on purpose: the price is
/// what the quality constraint below relaxes, and a quality constraint relaxed is the DECLARED
/// degradation of ADR-0012.
const REMOTE_DEAR: Candidate = Candidate {
    model: "remote",
    local: false,
    retains: true,
    price: 100,
};

/// ⛔ IT HANDS OVER `RemotePolicy`, THE DEFAULT OF ADR-0006 AND THE ONE THAT MAKES NO ROOM, for
/// the reason `tests/arbiter_admission.rs` writes out: under `LocalPolicy` a full arbiter would
/// start marking victims before it answers, and the saturated probe below would be about the
/// revocation instead of about the ceiling.
fn arbiter(total: Mib) -> Arbiter {
    Arbiter::new(
        Parameters::new(TURN_LIMIT, total, ArbiterId::new(1)),
        VramPolicy::Remote(RemotePolicy),
    )
}

fn profile(name: &'static str, vram: u64) -> ResourceProfile {
    ResourceProfile {
        name,
        reserved_vram: Mib::new(vram),
        compute_class: ComputeClass::Batch,
        preemption: Preemption::Never,
    }
}

/// Opens the step the routing is written upon, and answers nothing.
///
/// ⛔ `dispatch` DOES NOT DO THIS, and the asymmetry is the point: the gateway is handed a
/// `StepId` and has no allocator for one, so a mechanism that minted the intent of a step it does
/// not own would be inventing an identity the port assigns.
fn open_the_step(journal: &mut MemoryJournal, step: StepId) {
    let intent = Record::V1(RecordV1::intent(
        EffectClass::Idempotent,
        Trust::Instruction,
        Vec::new(),
        "the step whose routing is resolved",
    ))
    .encode();
    journal.intent(step, &intent).expect("intent");
}

/// Resolves the one candidate WITH a price ceiling it exceeds: the data constraints are empty, so
/// the second road is taken and the routing is degraded.
fn dispatch_a_degraded_routing(journal: &mut MemoryJournal, step: StepId) {
    let chain = [REMOTE_DEAR];
    let resolved = resolve(&chain, &[Constraint::PriceCeiling(10)]).expect("resolve");
    assert!(
        resolved.was_degraded(),
        "the fixture resolved a clean routing"
    );
    dispatch(resolved, step, journal).expect("dispatch");
}

/// The same candidate under a ceiling it MEETS: the first road, and nothing is degraded.
fn dispatch_a_clean_routing(journal: &mut MemoryJournal, step: StepId) {
    let chain = [REMOTE_DEAR];
    let resolved = resolve(&chain, &[Constraint::PriceCeiling(1_000)]).expect("resolve");
    assert!(
        !resolved.was_degraded(),
        "the fixture resolved a degraded routing"
    );
    dispatch(resolved, step, journal).expect("dispatch");
}

/// A journal whose `replay` refuses outright, for the road into `DegradationError::Journal`.
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

#[test]
fn an_idle_machine_declares_nothing() {
    // ⛔ THE NON-VACUITY PROBE OF BOTH FIELDS AT ONCE. Without it a derivation answering `true` to
    // everything passes every other probe in this file that asserts a degradation, and "what is
    // degraded now" would be a question with one answer.
    let journal = MemoryJournal::new();
    let arbiter = arbiter(ROOMY);

    let state = degradation_now(&arbiter, &journal).expect("derive");

    assert!(!state.vram_exhausted);
    assert!(!state.routing_degraded);
}

#[test]
fn a_degraded_routing_shows_up_in_the_state() {
    // ⛔ ADR-0012 says a quality constraint relaxed is a DECLARED degradation. This is where the
    // declaration becomes observable: without it, "declared" means "written in a record nobody
    // reads", which is the silent degradation ADR-0019 forbids.
    let mut journal = MemoryJournal::new();
    let step = StepId::new(1);
    open_the_step(&mut journal, step);
    dispatch_a_degraded_routing(&mut journal, step);

    let arbiter = arbiter(ROOMY);
    let state = degradation_now(&arbiter, &journal).expect("derive");

    assert!(state.routing_degraded);
    // ⚠️ AND THE OTHER FIELD IS ASSERTED TOO, because the two are derived from two independent
    // worlds: a derivation that set them together would pass the assertion above and be wrong
    // about a machine with 16 GiB free.
    assert!(!state.vram_exhausted);
}

#[test]
#[allow(non_snake_case)]
fn it_is_RECOMPUTED_and_not_cached() {
    // ⛔ THIS IS THE PROBE WITHOUT WHICH "recomputed" IS A CLAIM HELD BY NOTHING. Ask once, change
    // the world, ask again: a cached answer would repeat itself, and the whole reason §6.7
    // recomputes is that a cache makes "never authoritative of itself" a matter of discipline
    // instead of construction.
    //
    // ⚠️ THE ARBITER IS THE SAME VALUE ACROSS BOTH QUESTIONS, deliberately: what moves between
    // them is the JOURNAL alone, so the change the second answer reports can only have come from
    // re-reading it.
    let mut journal = MemoryJournal::new();
    let step = StepId::new(1);
    open_the_step(&mut journal, step);
    let arbiter = arbiter(ROOMY);

    let before = degradation_now(&arbiter, &journal).expect("derive");
    assert!(!before.routing_degraded);

    dispatch_a_degraded_routing(&mut journal, step);

    let after = degradation_now(&arbiter, &journal).expect("derive");
    assert!(after.routing_degraded);
}

#[test]
#[allow(non_snake_case)]
fn the_LAST_routing_wins_and_not_any_routing() {
    // ⛔ THE RULE `degradation_now` STATES IN ITS OWN BODY, and until this probe nothing held it:
    // with one routing per journal, "the last one" and "any one" give the same answer, and a loop
    // that stopped at the FIRST record — or one that accumulated with `|=` — passed every other
    // probe in this file. A degradation that happened and was then RESOLVED is a fact about
    // history, not about what the user can do now (§7.5).
    //
    // ⚠️ BOTH DIRECTIONS ARE IN ONE PROBE, and that is what keeps either half from being vacuous:
    // "always false" fails the first assertion, "always true" and "the first one wins" fail the
    // second. Two probes asserting one end each could each be satisfied by a constant.
    //
    // ⚠️ TWO NOTES UPON ONE STEP, which the port allows and says so: `Journal::note` places no
    // limit on how many notes a step carries, and only the INTENT is once-only. ⛔ THE PORT OWES
    // THEIR ORDER — `replay` promises write order for EVERYTHING — but the conformance suite
    // never writes two notes upon one step, and this probe does.
    let mut journal = MemoryJournal::new();
    let step = StepId::new(1);
    open_the_step(&mut journal, step);
    let arbiter = arbiter(ROOMY);

    dispatch_a_degraded_routing(&mut journal, step);
    assert!(
        degradation_now(&arbiter, &journal)
            .expect("derive")
            .routing_degraded
    );

    dispatch_a_clean_routing(&mut journal, step);

    assert!(
        !degradation_now(&arbiter, &journal)
            .expect("derive")
            .routing_degraded,
        "a degradation that was resolved is still being reported as the state NOW"
    );
}

#[test]
fn a_full_arbiter_declares_its_vram_exhausted() {
    // ⛔ THE OTHER HALF OF THE TYPE, AND THE DICTATED PROBES GAVE IT NOTHING. `vram_exhausted`
    // nailed to `false` is invisible to every routing probe above, so without this one half the
    // struct would be held by the compiler alone.
    //
    // ⚠️ THE MACHINE IS FILLED EXACTLY, not over-filled, and that is the boundary the comparison
    // sits on: `allocated() >= ceiling()` must be `true` when the two are EQUAL, because a
    // machine with nothing left to admit is exhausted whether or not anything overflowed. A `>`
    // in place of the `>=` answers "fine" to a full machine.
    let mut arbiter = arbiter(Mib::new(4_096));
    let journal = MemoryJournal::new();

    let Admission::Granted(_resident) =
        arbiter.admit(&profile("resident", 4_096), LONG, Monotonic::ORIGIN)
    else {
        panic!("it fills the machine exactly");
    };
    assert_eq!(
        arbiter.allocated(),
        Mib::new(4_096),
        "the fixture did not fill the machine"
    );

    let state = degradation_now(&arbiter, &journal).expect("derive");

    assert!(state.vram_exhausted);
    // ⚠️ AND THE JOURNAL IS EMPTY, so this probe says the two fields are independent from the
    // other side: a full machine degrades no routing.
    assert!(!state.routing_degraded);
}

#[test]
fn a_journal_that_will_not_replay_is_not_an_answer_of_nothing_degraded() {
    // ⛔ THE ROAD INTO `DegradationError::Journal`, and it is the reason that variant exists. A
    // derivation that answered `Ok` with both fields `false` when it could not read the archive
    // would report "nothing is degraded" for "I do not know" — the silent degradation ADR-0019
    // forbids outright — and every probe above would stay green, because they all replay.
    let arbiter = arbiter(ROOMY);
    let journal = ReplayRefusingJournal;

    assert_eq!(
        degradation_now(&arbiter, &journal),
        Err(DegradationError::Journal(JournalError::NotDurable))
    );
}

#[test]
fn a_record_that_will_not_decode_is_not_an_answer_of_nothing_degraded() {
    // ⛔ THE FIRST ROAD INTO `DegradationError::Record`. A `continue` in place of the `?` on
    // `Record::decode` passes every other probe in this file, and it would let a corrupt archive
    // read as a clean "nothing is degraded" while a degraded routing sits in the very bytes that
    // would not decode.
    //
    // ⚠️ THE BYTES ARE WRITTEN THROUGH THE PORT, which takes `&[u8]` and validates nothing — road
    // A4 of `kernel::boundary`, which already declares that nothing requires every write to the
    // journal to be a `Record`.
    let mut journal = MemoryJournal::new();
    let step = StepId::new(1);
    open_the_step(&mut journal, step);
    journal
        .note(step, b"not a record of any version")
        .expect("note");

    let arbiter = arbiter(ROOMY);

    assert_eq!(
        degradation_now(&arbiter, &journal),
        Err(DegradationError::Record(RecordError::Malformed))
    );
}

#[test]
fn a_routing_record_without_its_detail_is_not_an_answer_of_nothing_degraded() {
    // ⛔ THE SECOND ROAD INTO `DegradationError::Record`, and it is REACHABLE rather than declared
    // impossible. In SOURCE it is unpronounceable — `RecordV1::routing` takes its `RoutingDetail`
    // by value, so the `kind`/`detail` pair cannot be split. From BYTES it is one byte's work,
    // which is road A4 of `kernel::boundary` again: `Record::decode` is `pub` and privacy does
    // not watch the derived `Decode`.
    //
    // ⛔ AND THE ANSWER IS AN ERROR AND NOT "nothing is degraded", which is the whole of what the
    // sister `permission::is_granted` decided for its own species: such a record claims to be a
    // routing while naming no model and no degradation, so the only honest answer is that this
    // build cannot read it. An `if let … else { continue }` here would fold a corrupt archive
    // into a clean state.
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
    // `every_field_sits_at_the_offset_the_map_gives_it`. `04` is `RecordKind::Routing`.
    bytes[4] = 4;
    journal.note(step, &bytes).expect("note");

    let arbiter = arbiter(ROOMY);

    assert_eq!(
        degradation_now(&arbiter, &journal),
        Err(DegradationError::Record(RecordError::Malformed))
    );
}
