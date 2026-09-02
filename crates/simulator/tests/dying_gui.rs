//! `DyingGui`: what only IT promises. What every `ipc` implementation promises is nobody's
//! business yet — the port has no conformance suite and no real transport — and this type would
//! not be held to one anyway: a gui that stops answering is a LIAR by construction, and gotcha
//! #50 says a fake may break a contract when the test around it speaks about the breaking.
//!
//! ⛔ IT IS THE BENCH `CrashingJournal` HAS AND THE MAP OF THE TASK DID NOT LIST, finding of the
//! pre-check (`E156`). The campaign's own oracle proves the gui DIES; it does not prove it dies
//! AT THE DRAWN OPERATION, that it stays dead on `send` AND on `receive`, or that every operation
//! of the path can be the one. Those are promises of this type, and they live here.

use kernel::arbiter::{ComputeClass, Mib, Preemption};
use kernel::ports::ipc::{ClientId, Ipc, IpcError};
use kernel::wire::ipc::{GrantRequest, IpcMessage};
use simulator::ipc::DyingGui;

/// How many operations the campaign's path performs: `receive`, `send`, `receive`. ⚠️ It is a
/// constant rather than a literal because the number also appears in what the assertions SAY,
/// and a count duplicated between code and prose is gotcha #31 waiting for the day one of the
/// two is edited.
const OPERATIONS: u64 = 3;

/// How many seeds the probes over the generator sweep.
const SEEDS: u64 = 500;

const GUI: ClientId = ClientId::new(1);

fn a_request() -> GrantRequest {
    GrantRequest {
        reserved_vram: Mib::new(1_024),
        compute_class: ComputeClass::Interactive,
        preemption: Preemption::Never,
    }
}

#[test]
fn it_dies_at_the_operation_it_was_told_to_die_at() {
    // ⛔ NOT "it dies somewhere": at THE operation. The number is handed in rather than drawn, so
    // this probe does not depend on the generator — that is `the_same_seed_chooses_the_same_
    // operation`'s job.
    let mut gui = DyingGui::dying_at(GUI, a_request(), 2);

    assert_eq!(gui.accept(), Some(GUI));
    assert!(
        gui.receive(GUI)
            .expect("the first receive is before the death")
            .is_some()
    );
    assert_eq!(gui.send(GUI, b"a verdict"), Ok(()));
    assert_eq!(gui.receive(GUI), Err(IpcError::Disconnected));
}

#[test]
fn after_the_death_every_later_operation_is_refused_too() {
    // ⛔ THE DIFFERENCE BETWEEN A DEATH AND A BAD PIPE. A client that refused once and answered
    // again would let the core carry on talking to a process that is gone — and ADR-0033 gives
    // the core exactly one event to reconcile on, so an event that un-happens is worse than none.
    //
    // ⛔ BOTH METHODS, AND THAT IS THE HALF A SINGLE ASSERTION MISSES: `send` and `receive` keep
    // separate code in this fake, so a death marked on one and not on the other would leave the
    // core able to keep pumping the other direction.
    let mut gui = DyingGui::dying_at(GUI, a_request(), 0);

    assert_eq!(gui.receive(GUI), Err(IpcError::Disconnected));
    assert_eq!(gui.send(GUI, b"a verdict"), Err(IpcError::Disconnected));
    assert_eq!(gui.receive(GUI), Err(IpcError::Disconnected));
    assert_eq!(
        gui.send(GUI, b"another verdict"),
        Err(IpcError::Disconnected)
    );
}

#[test]
fn a_dead_client_is_not_presented_again() {
    // ⛔ `accept` IS THE ONE METHOD WITH NO ERROR CHANNEL, so the only thing it can do about a
    // death is stop offering the client. A fake that offered it again would hand the core a live
    // identifier for a process that is gone, and the reconciliation would be undone by the fake.
    let mut gui = DyingGui::dying_at(GUI, a_request(), 0);

    assert_eq!(gui.accept(), Some(GUI), "it connects once");
    assert_eq!(gui.accept(), None, "and only once");

    assert_eq!(gui.receive(GUI), Err(IpcError::Disconnected));
    assert!(gui.has_died());
    assert_eq!(gui.accept(), None, "a dead client is not offered again");
}

#[test]
fn what_the_gui_said_before_dying_is_a_real_encoded_request() {
    // ⛔ THE REQUEST CROSSES AS BYTES AND MUST DECODE, which is what makes the campaign exercise
    // the schema instead of going around it. A fake that handed over an empty body would leave
    // the campaign green having decoded nothing — `IpcMessage::encode` contains its own failure
    // as an empty body, and an empty body is exactly what does NOT decode.
    let mut gui = DyingGui::dying_at(GUI, a_request(), 2);

    let bytes = gui
        .receive(GUI)
        .expect("before the death")
        .expect("the gui had something to say");
    assert_eq!(
        IpcMessage::decode(&bytes),
        Ok(IpcMessage::Request(a_request()))
    );

    // And it says it ONCE: a client that repeated itself for ever would make "the path performs
    // OPERATIONS operations" false and put the drawn point out of reach.
    assert_eq!(gui.receive(GUI), Ok(None));
}

#[test]
fn an_operation_named_on_another_client_does_not_consume_a_death_position() {
    // ⛔ GOTCHA #17 FROM THE OTHER SIDE, and the shape `a_write_the_protocol_refuses_does_not_
    // consume_a_crash_position` already has. An identifier this gui never had reaches nothing, so
    // if it consumed a position the death would drift EARLIER than the drawn point — and with a
    // point near the last operation it would not fire at all.
    let stranger = ClientId::new(9);
    let mut gui = DyingGui::dying_at(GUI, a_request(), 1);

    assert_eq!(gui.receive(stranger), Err(IpcError::Disconnected));
    assert_eq!(
        gui.send(stranger, b"a verdict"),
        Err(IpcError::Disconnected)
    );
    assert!(!gui.has_died(), "a stranger is not the death");
    assert_eq!(
        gui.operations_done(),
        0,
        "an operation on another client reached nothing"
    );

    // So the death is still the second operation that really happens on THIS client.
    assert!(gui.receive(GUI).expect("the first operation").is_some());
    assert_eq!(gui.operations_done(), 1);
    assert_eq!(gui.send(GUI, b"a verdict"), Err(IpcError::Disconnected));
}

#[test]
fn a_gui_told_not_to_die_never_does() {
    // ⛔ THE OTHER DIRECTION (rule 3 of §7.1.1): a control that fires where it must not is worse
    // than one that is absent. Without this, a fake that died on every call would satisfy every
    // probe above that asserts a `Disconnected`.
    let mut gui = DyingGui::immortal(GUI, a_request());
    assert_eq!(gui.accept(), Some(GUI));
    for turn in 0..64u64 {
        assert!(gui.receive(GUI).is_ok(), "turn {turn}");
        assert_eq!(gui.send(GUI, b"a verdict"), Ok(()), "turn {turn}");
    }
    assert!(!gui.has_died());
    assert_eq!(gui.operations_done(), 128);
}

#[test]
fn the_same_seed_chooses_the_same_operation() {
    // ⚠️ UNFALSIFIABLE BY CONSTRUCTION, and declared so rather than removed (gotcha #44).
    // `from_seed` is a pure function of its arguments, so no implementation of it could make this
    // red; the real determinism — that the SEQUENCE repeats — is held where it can actually fail,
    // by `crates/simulator/tests/seeded_rng.rs::the_same_seed_gives_the_same_sequence`. This is a
    // NAMED STATEMENT of what this type promises its callers, not coverage, and it must not be
    // counted as either.
    let first = DyingGui::from_seed(GUI, a_request(), 99, OPERATIONS);
    let second = DyingGui::from_seed(GUI, a_request(), 99, OPERATIONS);
    assert_eq!(first.dies_at(), second.dies_at());
}

#[test]
fn the_drawn_point_lies_inside_the_operations_the_path_performs() {
    // ⛔ GOTCHA #17: injecting a fault where the code never arrives is a VACUOUS proof that looks
    // like a success. If the point could land past the last operation, some seeds would simply
    // never kill the gui and the campaign would report green for having done nothing.
    for seed in 0..SEEDS {
        let point = DyingGui::from_seed(GUI, a_request(), seed, OPERATIONS).dies_at();
        assert!(
            point < OPERATIONS,
            "seed {seed} drew {point}, outside 0..{OPERATIONS}"
        );
    }
}

#[test]
fn every_operation_of_the_path_can_be_the_one_that_kills_it() {
    // ⛔ THE OTHER HALF OF #17, AND `> 1` IS NOT ENOUGH FOR IT. A point that never moved would
    // make five hundred seeds one experiment repeated — but so would a generator that only ever
    // drew two of the three, and the positions are not worth the same: only the ones AFTER the
    // request has crossed leave the gui holding a grant when it dies, which is the whole subject
    // of property 3.
    //
    // ⚠️ DECLARED COUPLING: this holds because `SEEDS` is far above `OPERATIONS`, not by
    // construction of `below`.
    let mut seen = std::collections::BTreeSet::new();
    for seed in 0..SEEDS {
        seen.insert(DyingGui::from_seed(GUI, a_request(), seed, OPERATIONS).dies_at());
    }
    assert_eq!(
        seen.len(),
        OPERATIONS as usize,
        "{SEEDS} seeds reached only {seen:?} of the {OPERATIONS} operations"
    );
}

#[test]
fn has_died_says_no_until_it_dies() {
    // The bench-side reading of what the campaign may only learn from the port: without it,
    // "this run left the books alone" and "the death never fired" are the same green.
    let mut gui = DyingGui::dying_at(GUI, a_request(), 1);
    assert!(!gui.has_died());
    assert!(gui.receive(GUI).is_ok());
    assert!(!gui.has_died());
    assert_eq!(gui.send(GUI, b"a verdict"), Err(IpcError::Disconnected));
    assert!(gui.has_died());
}
