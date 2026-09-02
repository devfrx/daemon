//! Which grant belongs to which IPC client, and what happens when one disconnects.
//!
//! ⛔ WHY THIS EXISTS AT ALL, and it is the finding P-16: §5.7 asks that the sum return to
//! baseline when a GUI dies holding a discretionary grant, and ADR-0033 names the mechanism —
//! "the core notices from the IPC DISCONNECTION and reconciles". Nothing performed that
//! reconciliation, and no file in the plan's map had a place for it.
//!
//! ⛔ THIS IS NOT THE ORCHESTRATION LOOP, and the distinction matters because `E50` and `E51`
//! wait for one. That loop decides WHEN `promote` runs relative to `admit`; this decides nothing
//! of the sort — it answers one event with one release. The two open voices stay open.
//!
//! ⛔ WHY IT IS NOT INSIDE `Arbiter`: the arbiter knows reservations, lanes and grants, and
//! nothing about clients. Giving it a `ClientId` would put a notion of the `ipc` port inside the
//! type that ADR-0005 keeps about RESOURCE — and I3's shape of argument applies within the crate
//! too: a boundary is worth what it refuses to know.
//!
//! ⛔ AND IT DOES NOT TOUCH THE PORT EITHER. Nothing here calls `Ipc`, and that is what keeps
//! the reconciliation usable: the DEATH is seen by whoever polls the port — `ipc` has no
//! disconnection event, only `Err(IpcError::Disconnected)` on `send` or `receive` — and this
//! module answers it. A type that asked the port whether a client was still there would hold
//! both halves and be testable only against a fake that agreed with it.

use alloc::vec::Vec;

use crate::arbiter::{Arbiter, Grant, ReleaseError, Released};
use crate::ports::ipc::ClientId;
use crate::time::Monotonic;

/// The pairs: who holds what.
///
/// ⛔ A `Vec` OF PAIRS AND NOT A MAP, and it is a consequence of two decisions taken elsewhere
/// rather than a preference. `HashMap` is forbidden outright — gotcha #12, and
/// `tests/compile_fail/hashmap_in_kernel.rs` is what makes it unspeakable — and `BTreeMap` wants
/// `Ord`, which `ClientId` deliberately does not derive: `ports::ipc` pruned `PartialOrd`, `Ord`
/// and `Hash` one at a time, and the argument it pruned them with is that "a table plus `==`
/// works, as every other fake here does". This is that table. ⛔ ADDING THE DERIVE TO GET A
/// `BTreeMap` WOULD REOPEN THAT DECISION for the convenience of this file, which is the trade
/// `ports::process` refused when `Grant` was asked for a `Debug`.
///
/// ⛔ ONE CLIENT MAY HOLD SEVERAL, which is why the pairs are not a client-keyed anything: a gui
/// that asked twice and was granted twice holds two, and `on_disconnect` owes both back.
///
/// ⚠️ NO `Default`, and its absence is the decision this repository has already taken three
/// times — `SystemReactor`, `VirtualReactor`, `MemoryJournal`: nothing calls it. The argument,
/// including why the `clippy::new_without_default` warning is accepted rather than silenced, is
/// written out once in `crates/platform/src/reactor.rs`.
pub struct ClientGrants {
    held: Vec<(ClientId, Grant)>,
}

impl ClientGrants {
    pub const fn new() -> Self {
        ClientGrants { held: Vec::new() }
    }

    /// Records that this client holds this grant.
    ///
    /// ⛔ IT TAKES THE GRANT BY VALUE because there is no other way: `Grant` is neither `Clone`
    /// nor `Copy`, deliberately, and a register that borrowed one would leave the caller holding
    /// the only copy of a capability this type is supposed to be answerable for.
    pub fn register(&mut self, client: ClientId, grant: Grant) {
        self.held.push((client, grant));
    }

    /// The reconciliation: every grant of THIS client, handed back to the arbiter.
    ///
    /// ⛔ IT ANSWERS `Ok(empty)` FOR A CLIENT THAT HOLDS NOTHING, AND THAT IS THE SHAPE AND NOT
    /// A LENIENCY. A gui may die before it ever asked — it may die before it was ever accepted —
    /// and reporting an ordinary event as a fault would make every caller learn to ignore this
    /// `Result`. It is the same reading `IpcError::Disconnected` already carries: "this client
    /// died" and "this identifier was never issued" are one thing seen from the core.
    ///
    /// ⛔ SO THE `Err` IS THE CALLER'S DEFECT AND NOTHING ELSE. `Arbiter::release` answers
    /// `Err(UnknownGrant)` for exactly one cause since 2026-08-30 — a grant ANOTHER arbiter
    /// issued — so an `Err` here says this register was handed the wrong arbiter, never that the
    /// clock ran on. A grant whose window has closed is `Ok(AlreadyCollected)`, which is why
    /// those two words are in the answer instead of being flattened away.
    ///
    /// ⛔ THE ANSWER IS THE `Released`S THEMSELVES AND NOT A COUNT, which is `E30` delivered
    /// rather than restated: `Now(Mib)` and `AlreadyCollected` are a difference the caller has
    /// something to do with, and a `usize` would collapse them. It is also what makes a probe
    /// able to tell "the reconciliation freed the reservation" from "the sweep had already".
    ///
    /// ⛔ IT CANNOT PANIC. No `unwrap`, no `expect`, no indexing: `position` answers an `Option`
    /// the `while let` consumes, and `remove` is reached only with an index `position` just
    /// returned. A reconciliation that panicked would turn the death of a sacrificial process
    /// into the death of the core, which is I1 upside down.
    ///
    /// ⚠️ DECLARED LIMIT, measured rather than reasoned: on an `Err` the grant that PRODUCED it
    /// is gone — `Arbiter::release` consumes what it is given, and it consumed that one before
    /// answering. What this loop buys is the REST: it takes one pair at a time, so every grant
    /// this client still holds is still registered when the `?` returns, and a caller that comes
    /// back with the right arbiter loses none of them. Draining first and releasing afterwards
    /// would have dropped all of them on that same `Err`.
    pub fn on_disconnect(
        &mut self,
        client: ClientId,
        arbiter: &mut Arbiter,
        now: Monotonic,
    ) -> Result<Vec<Released>, ReleaseError> {
        let mut released = Vec::new();
        // ⛔ THE SEARCH IS ON THE IDENTIFIER, and that is the whole of "only that client's". A
        // loop that took the pairs in order and released them all would satisfy "the sum comes
        // back to baseline" for the client that died AND take the reservation of every client
        // that did not.
        while let Some(index) = self.held.iter().position(|(holder, _)| *holder == client) {
            let (_, grant) = self.held.remove(index);
            released.push(arbiter.release(grant, now)?);
        }
        Ok(released)
    }
}
