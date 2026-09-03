//! The falling double of the `ipc` port: a gui that STOPS EXISTING at an operation chosen by
//! the seed — §3.3, and level 1 of the two crash levels of ADR-0032.
//!
//! ⛔ IT IS THE SHAPE OF `CrashingJournal` AND NOT A NEW ONE, and the reason is that type's own:
//! a client that refused once and answered again afterwards would model A BAD PIPE, not a death.
//! The gui is sacrificial (I1, ADR-0004) and a sacrificial process that came back would make the
//! one event ADR-0033 gives the core to reconcile on unobservable. So the first refusal is
//! PERMANENT, on `send` and on `receive` alike, and `accept` does not present the client again.
//!
//! ⛔ THE DEATH IS VISIBLE ONLY AS `Err(IpcError::Disconnected)`, and that is a property of the
//! PORT rather than a choice of this file: `Ipc` has `accept`, `send` and `receive` and no
//! disconnection event at all. `has_died` exists for the bench that holds this type's own
//! promises — `crates/simulator/tests/dying_gui.rs` — and a campaign that asked it instead of
//! reading the port would keep the reconciliation inside the bench, which is the defect the
//! finding P-16 names.
//!
//! ⚠️ IT IS NOT HELD TO ANY CONFORMANCE SUITE, deliberately, for the reason `CrashingJournal` is
//! not: this type is a LIAR by construction, and gotcha #50 says a fake may break a contract when
//! the test around it speaks about the breaking. There is no `ipc` conformance suite in any case
//! — the port has no real transport yet.

use alloc::vec::Vec;

use crate::rng::SeededRng;
use kernel::ports::ipc::{ClientId, Ipc, IpcError};
use kernel::rng::RngExt;
use kernel::wire::ipc::{GrantRequest, IpcMessage};

/// A gui that connects once, asks once, and dies at the operation it was told to die at.
///
/// ⛔ WHAT COUNTS AS AN OPERATION IS `send` AND `receive`, AND `accept` IS OUT. The count the
/// death point is drawn against has to be a count of things that CAN report the death, and
/// `accept` answers `Option<ClientId>` with no error channel: a death drawn at an `accept` would
/// be a point the core could never observe, which is gotcha #17 wearing the port's own asymmetry
/// as a disguise.
pub struct DyingGui {
    /// The one thing this gui has to say. ⛔ IT IS HANDED IN RATHER THAN DRAWN HERE, and the
    /// split is the same one `CrashingJournal` makes: this type models a DEATH, and what a client
    /// asks for is a property of the world the campaign is building.
    request: GrantRequest,
    client: ClientId,
    dies_at: u64,
    operations: u64,
    died: bool,
    accepted: bool,
    delivered: bool,
}

impl DyingGui {
    /// Dies at the operation with this index, counting from zero.
    pub const fn dying_at(client: ClientId, request: GrantRequest, operation: u64) -> Self {
        DyingGui {
            request,
            client,
            dies_at: operation,
            operations: 0,
            died: false,
            accepted: false,
            delivered: false,
        }
    }

    /// Dies at an operation DRAWN from the seed, inside `0..expected_operations`.
    ///
    /// ⛔ `expected_operations` IS HOW MANY OPERATIONS THE PATH REALLY PERFORMS, counted rather
    /// than guessed. Gotcha #17: a point drawn past the last one never fires, and a campaign
    /// whose fault never arrives reports green for having done nothing.
    ///
    /// ⛔ THE SEED MUST BE DERIVED, AND DIFFERENT FROM THE ONE DRIVING THE INTERLEAVING —
    /// decision D2 of the milestone 4 plan, and the obvious wiring is the wrong one. Two
    /// `SeededRng` built from the same number give the SAME sequence, so passing a campaign's
    /// seed straight through ties the death point to everything else that seed decides: the
    /// campaign would then explore a DIAGONAL of the space instead of the space. Nothing on this
    /// type can enforce it — the caller holds the seed — so it is written here, where the caller
    /// is looking, rather than left in a plan nobody rereads (gotcha #36).
    ///
    /// ⛔ AND `expected_operations` MUST NOT BE ZERO, for `CrashingJournal::from_seed`'s reason:
    /// `RngExt::below` answers 0 for a bound of 0, and 0 is not inside the empty range `0..0`, so
    /// the point would be one that can never arrive.
    pub fn from_seed(
        client: ClientId,
        request: GrantRequest,
        seed: u64,
        expected_operations: u64,
    ) -> Self {
        debug_assert!(
            expected_operations > 0,
            "a path that performs no operation has none to die at: `below` would answer 0, \
             which is outside the empty range 0..0, and the death would never fire"
        );
        let mut rng = SeededRng::new(seed);
        Self::dying_at(client, request, rng.below(expected_operations))
    }

    /// Never dies. It is what the counter-direction is measured against — a run in which the
    /// core is never given a disconnection to reconcile on.
    pub const fn immortal(client: ClientId, request: GrantRequest) -> Self {
        Self::dying_at(client, request, u64::MAX)
    }

    /// The operation it will die at.
    pub const fn dies_at(&self) -> u64 {
        self.dies_at
    }

    /// Whether it HAS died. ⛔ FOR THE BENCH THAT HOLDS THIS TYPE, AND NOT FOR THE CAMPAIGN:
    /// the campaign has to learn of the death the way the core does, from the port.
    pub const fn has_died(&self) -> bool {
        self.died
    }

    /// How many operations went through before the death.
    pub const fn operations_done(&self) -> u64 {
        self.operations
    }

    /// Whether this operation may proceed, MARKING the death when it may not.
    ///
    /// ⚠️ IT IS ASKED AFTER THE IDENTITY CHECK AND BEFORE THE WORK, which is `CrashingJournal`'s
    /// order and for its reason: an operation named on a client this gui is not never reached
    /// anything, so it must not consume a position in the count the death point is drawn against.
    fn may_operate(&mut self) -> bool {
        if self.died {
            return false;
        }
        if self.operations == self.dies_at {
            self.died = true;
            return false;
        }
        true
    }

    /// Whether this identifier is the one this gui was accepted under.
    ///
    /// ⛔ AN IDENTIFIER THIS GUI NEVER HAD IS `Disconnected` AND NOT A THIRD ANSWER, which is the
    /// port's own reading rather than this fake's invention: `IpcError::Disconnected` is "one word
    /// for two facts", and "this identifier was never issued" is the second of them.
    fn is_mine(&self, client: ClientId) -> bool {
        client == self.client
    }
}

impl Ipc for DyingGui {
    fn accept(&mut self) -> Option<ClientId> {
        // ⛔ IT DOES NOT PRESENT THE CLIENT AGAIN AFTER THE DEATH, and that is the difference
        // between a crash and a reconnection. A core that accepted the same gui a second time
        // would be handed a live identifier for a process that is gone, and the reconciliation
        // it had just performed would be undone by the fake rather than by the world.
        if self.died || self.accepted {
            return None;
        }
        self.accepted = true;
        Some(self.client)
    }

    fn send(&mut self, client: ClientId, _message: &[u8]) -> Result<(), IpcError> {
        if !self.is_mine(client) {
            return Err(IpcError::Disconnected);
        }
        if !self.may_operate() {
            return Err(IpcError::Disconnected);
        }
        self.operations += 1;
        // ⛔ THE VERDICT IS ACCEPTED AND NOT KEPT. This fake is a client that DIES; what it was
        // told is the gui's business, and a field holding the last verdict would be state no
        // assertion reads — the item this repository removes rather than keeps for symmetry.
        Ok(())
    }

    fn receive(&mut self, client: ClientId) -> Result<Option<Vec<u8>>, IpcError> {
        if !self.is_mine(client) {
            return Err(IpcError::Disconnected);
        }
        if !self.may_operate() {
            return Err(IpcError::Disconnected);
        }
        self.operations += 1;
        if self.delivered {
            // ⚠️ `Ok(None)` IS NOT AN ERROR and it is not the death either: an idle client and a
            // dead one must not give the same answer, or the core could not poll this port.
            return Ok(None);
        }
        self.delivered = true;
        // ⛔ IT CROSSES AS BYTES, like `journal` and `process`, so the campaign really exercises
        // the encoding instead of going around it. ⚠️ AND A FAILED ENCODING BECOMES AN EMPTY
        // BODY rather than a panic — the containment `IpcMessage::encode` already argues, and
        // `unwrap_or_default` here is a `Vec` and not a value this file invents: an empty body
        // does not decode (`an_empty_body_in_an_honest_envelope_does_not_decode`), so the core
        // would see a malformed message rather than a message nobody wrote.
        let message = IpcMessage::Request(self.request.clone());
        Ok(Some(message.encode().unwrap_or_default()))
    }
}
