//! The schema of the `ipc` channel: the envelope of `crate::framing` carrying ONE enum.
//!
//! ⛔ ONE ENUM FOR BOTH DIRECTIONS, AND THE DIRECTION IS DOCUMENTED RATHER THAN TYPED. Two
//! enums of one variant each would leave BOTH discriminants unexercised, which is the very
//! thing §6.7 asks two messages for. And typing the direction would buy nothing at the port:
//! `send` takes `&[u8]` and `receive` returns `Vec<u8>`, so the boundary sees no type at all.
//! ⚠️ THE COST, stated: nothing stops a caller from encoding a `Verdict` and sending it UP.
//! Today there is no such caller -- the transport is staged out (open item 5) -- and the day
//! there is one, the guard that pays for itself is on the composition side, not here.
//!
//! ⛔ THE SCHEMA MINTS NO IDENTIFIERS BECAUSE IT CARRIES NONE, and saying it that way is the
//! point (§6.5). Writing "§6.1.3 is satisfied" would be green having compared empty sets. A
//! grant request is not a step of a run: it writes no record and carries neither `StepId` nor
//! `RunId`. The first message that carries an identifier is where the rule becomes real, and
//! where its probe is born.
//!
//! ⛔ NO VERSION ENUM, NO RETIRED-INDEX REGISTER, NO FROZEN BYTES -- I4 renounces versioning
//! (§6.4). What stands in its place is the BUILD STAMP of §6.1.2, WHICH THIS MILESTONE DOES
//! NOT BUILD (§3.4). Until it exists, NOTHING REFUSES A STALE GUI, and today that costs
//! nothing because there is no gui to refuse -- `grep -rn "impl Ipc" crates/` returns a bench
//! fake. The trigger is milestone 2 of the subproject, the one that brings the shell.
//!
//! ⛔ AND THE REVOCATION core -> gui IS A DECLARED NON-CONSTRUCTION. ADR-0033 names it -- "the
//! gui stops rendering the 3D and says so" -- and it is the first message this vocabulary will
//! gain. It is not here because no written line demands it today and because §5.7 row 3 speaks
//! of a gui that DIES, not one that is asked. ⚠️ THE COST IS REAL: until then a discretionary
//! grant is preemptible IN THE BOOKS and the gui never hears about it. Open item 7. ⛔ ITS
//! TRIGGER IS THE SAME SHELL, and it is written out rather than inherited from the paragraph
//! above, because the two non-constructions could have had different ones: a revocation needs
//! an ADDRESSEE, and until milestone 2 of the subproject there is nobody to tell. Inventing
//! the message before there is one would freeze a vocabulary against an imaginary consumer --
//! gotcha #46 from the wrong side, which is the same reason §3.4 gives for the stamp.

use alloc::vec::Vec;
use bincode::{Decode, Encode};

use crate::arbiter::{ComputeClass, Mib, Preemption};
use crate::framing::{self, WireError};

/// What the gui asks for: an ordinary grant beyond the presentation quota (ADR-0033).
///
/// ⛔ IT IS NOT A `ResourceProfile`, AND THE MISSING FIELD IS THE REASON. `ResourceProfile`
/// carries `name: &'static str`, which cannot be produced from arriving bytes without leaking
/// -- and what would be leaked is text CHOSEN BY THE GUI, i.e. untrusted content (ADR-0014)
/// inside a type the arbiter DECIDES with. The split here is the one ADR-0005 already
/// describes: THE REQUESTER DECLARES THE RESERVATION, and the core names the profile.
///
/// ⛔ AND CARRYING THE NAME AND RESOLVING IT IS NOT THE OTHER ROAD, measured rather than
/// argued: `grep -rn "ResourceProfile {" crates/ --include=*.rs` finds two constants in
/// `daemon` and two bench helpers, and NOTHING that maps a name onto a profile. Building such
/// a register would be a mechanism no written line asks for.
///
/// ⚠️ THE COST, stated: the core picks ONE profile for the gui, so the gui cannot ask for an
/// arbitrary one. That is what ADR-0033 describes -- a single consumer, the 3D viewer beyond
/// the quota -- and it stops being enough the day a second one exists.
///
/// ⚠️ THE DERIVES, MEASURED ONE AT A TIME ON 2026-08-31 RATHER THAN ARGUED, because task 11 of
/// milestone 5 spent a review pruning derives nobody could name a consumer for, and a list
/// asserted to be accounted for is exactly what that review found. Removed and rebuilt, each
/// on its own: `Debug` -- `E0277`, `IpcMessage` doesn't implement `Debug`, demanded by the
/// `assert_eq!`s of `crates/kernel/tests/ipc_wire.rs`. `PartialEq` -- `E0369`, `==` cannot be
/// applied. `Encode`/`Decode` are the schema itself.
/// ⛔ `Eq` AND `Clone` HAVE NO CONSUMER, AND THAT IS MEASURED, NOT SUSPECTED: dropped one at a
/// time, `cargo build --locked --workspace --tests` compiles with ZERO errors and ZERO
/// warnings. They are here because they are the shape of `crate::wire::worker::FromWorker`,
/// which E33 of this task's errata pointed at, and pruning them is REGISTERED AND NOT TAKEN --
/// the precedent that argues for pruning lives on `crate::ports::ipc::ClientId`, which refused
/// `Ord` because a derive addable later in one line "is a convenience, not the entry door of
/// whoever comes"; the precedent that argues for keeping is the one on the ports themselves,
/// where callers are empty by construction and the criterion cannot tell dead from not-yet.
/// This channel has no transport yet (open item 5), so both readings are live and the choice
/// is the owner's.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct GrantRequest {
    pub reserved_vram: Mib,
    pub compute_class: ComputeClass,
    pub preemption: Preemption,
}

/// The three-way outcome, WITHOUT the grant.
///
/// ⛔ `Granted` IS A UNIT VARIANT AND CARRIES NO `Grant`, which is the whole of this type. A
/// decodable `Grant` would be a capability MINTED FROM BYTES: §5.6 holds that the only site
/// that mints one is `Arbiter::issue`, and `tests/compile_fail/grant_has_no_constructor.rs`
/// exists to make it unspeakable from outside. It would be AUD-050 done again on the
/// strongest token in the project -- a guard is worth exactly what its constructor is worth.
/// ⚠️ AND THE GUI DOES NOT NEED ONE: ADR-0033 says the grant is STATE OF THE CORE (I1). What
/// crosses is the verdict.
///
/// ⛔ THAT PROPERTY IS HELD BY CONSTRUCTION AND NOT BY A NEGATIVE CASE, and the distinction is
/// written rather than left to be inferred: a `compile_fail` case saying "a verdict may not
/// carry a `Grant`" would be a NEW CATALOGUE ROW, i.e. §7.4, i.e. spec -- the owner's, global
/// constraint 7. What holds it today is that the variant is unit and that this paragraph says
/// why. Registered, not taken.
///
/// ⚠️ `Refused` CARRIES TWO NUMBERS AND `Queued` CARRIES NOTHING, and the asymmetry is
/// argued: design/02 wants "why it does not fit and the workable alternative", ADR-0020
/// forbids the kernel to suggest one, so THE INTERFACE BUILDS IT AND THE KERNEL HANDS OVER
/// THE MATERIAL -- the gui is the written consumer of those two. A ticket, by contrast, is
/// load-bearing only for a caller with TWO requests outstanding, and the gui has one.
///
/// ⚠️ Same derive accounting as `GrantRequest`, one house above.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum Verdict {
    Granted,
    Queued,
    Refused { asked: Mib, ceiling: Mib },
}

/// One message on the `ipc` wire.
///
/// ⚠️ Same derive accounting as `GrantRequest`.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum IpcMessage {
    /// gui -> core.
    Request(GrantRequest),
    /// core -> gui.
    Verdict(Verdict),
}

impl IpcMessage {
    /// Encodes the message and wraps it in its envelope.
    ///
    /// ⛔ A FAILED ENCODING BECOMES AN EMPTY BODY RATHER THAN AN ERROR, on the containment
    /// argument `Record::encode` makes and `crate::wire::worker::FromWorker::encode` borrows
    /// -- READ AGAINST THIS TYPE'S GRAPH RATHER THAN COPIED. That graph is `Mib(u64)`,
    /// `ComputeClass`, `Preemption`, `Millis(u64)` and unit variants; the writer is a `Vec`
    /// that grows. Of `EncodeError`'s variants reachable without `std`, `UnexpectedEnd` is a
    /// writer out of room, and the other two want a `RefCell` or a caller-supplied string --
    /// none of which this graph has.
    ///
    /// ⚠️ THE SHAPE DIFFERS FROM THE TWIN'S AND THE ARGUMENT DOES NOT. `minicbor::encode`
    /// writes into a `Vec` the caller already owns, so its `Result` is dropped with `let _`;
    /// `encode_to_vec` RETURNS the `Vec`, so dropping the error needs a value in its place --
    /// and the value is the EMPTY body the containment argument already covers.
    ///
    /// ⛔ AND THE CONTAINMENT IS HELD RATHER THAN ASSERTED:
    /// `an_empty_body_in_an_honest_envelope_does_not_decode` is what makes it true. If an empty
    /// body decoded, a stopped encoder would put a message NOBODY WROTE on the wire, and this
    /// paragraph would be a promise kept by nothing.
    pub fn encode(&self) -> Result<Vec<u8>, WireError> {
        let body = bincode::encode_to_vec(self, bincode::config::standard()).unwrap_or_default();
        framing::frame(&body)
    }

    /// Reads a message out of an envelope.
    ///
    /// ⛔ TWO CHECKS AND NOT ONE, and they catch different faults: `framing::unframe` catches a
    /// frame whose declared length does not match the bytes -- a TRUNCATION, which no decoder
    /// can see because the missing tail is simply not there -- while `used != body.len()`
    /// catches a body that carries a COMPLETE MESSAGE AND SOMETHING AFTER IT, inside a length
    /// that tells the truth. The decoder stops at the end of the value it understood and
    /// reports how far it got; without this comparison the leftover would pass unread. It is
    /// the line `Record::decode` already carries, finding AUD-047.
    ///
    /// ⚠️ `decode_from_slice` FIXES `Decode`'s CONTEXT PARAMETER TO `()`, which is why nothing
    /// in this signature names it: the trait is `Decode<Context>`, and choosing the context is
    /// the caller's only when the caller wants one.
    pub fn decode(bytes: &[u8]) -> Result<Self, WireError> {
        let body = framing::unframe(bytes)?;
        let (message, used) = bincode::decode_from_slice(body, bincode::config::standard())
            .map_err(|_| WireError::Malformed)?;
        if used != body.len() {
            return Err(WireError::Malformed);
        }
        Ok(message)
    }
}
