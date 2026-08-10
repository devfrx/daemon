//! The durable record (§4.9). ⛔ EVERY DURABLE RECORD DECLARES ITS OWN VERSION, AND ITS
//! FIELDS ARE IDENTIFIED BY EXPLICIT INDEX — ADR-0036, and the six rules are in §4.9.2.
//!
//! ⛔ THE ENCODING LIVES HERE, IN `kernel`, and the `journal` port exchanges BYTES. Three
//! reasons, from §4.9.3: the data model is the kernel's property (§4.4); with bytes on the
//! port the SIMULATOR EXCHANGES BYTES TOO, so the DST campaign really exercises encoding and
//! decoding instead of going around them; and the measured cost is small.
//!
//! ⛔ ARRAY ENCODING, NOT MAP, AND IT IS WRITTEN OUT EVEN THOUGH IT IS THE DEFAULT.
//! Measured in ADR-0036: array 27 bytes (+4 %), map 33 (+27 %), positional 26. The ADR notes
//! that the earlier estimate "priced the map instead of the array" — so the number that
//! decided this is the array one. A default nobody wrote down is a default somebody changes.
//! ⚠️ AND THAT IT COSTS NOTHING TO WRITE IT WAS MEASURED, not assumed: with and without
//! `#[cbor(array)]` on the two types below, a record encodes to the same bytes down to the
//! length — `82 00 81 84 00 01 00 40` empty, 28 bytes with a 20-byte payload.
//!
//! ⚠️ AND THE ARRAY HAS A PRICE THE MAP DOES NOT, which belongs beside those numbers: a
//! RETIRED INDEX COSTS A NULL BYTE FOR EVER. The array is positional, so a gap has to be
//! written to keep the ones after it in place, whereas a map simply omits the key. The
//! comparison above is between the shapes as they are TODAY; every index the format retires
//! moves it by one byte per record, in the archive's favourite direction, which is bigger.
//!
//! ⛔ AND `#[cbor(index_only)]` ON THE THREE ENUMS CARRIES ITS OWN CONSTRAINT, declared here
//! because it binds a FUTURE change and nothing in the file would otherwise say so: it encodes
//! a variant as its bare index, with no room for a body, so a variant under it can NEVER GAIN A
//! FIELD. The day one of them needs to carry a value — a `Verifiable` that names what to ask
//! the world, say — the annotation comes off and EVERY RECORD EVER WRITTEN changes shape. That
//! is a new version of the record, not an edit to this line. The byte-string annotation below
//! declares its own stake; this one is the stake of the three above it.

use alloc::vec::Vec;
use core::fmt;
use minicbor::{Decode, Encode};

/// Is this the INTENTION of a step or its OUTCOME? The whole write-ahead protocol rests on
/// telling them apart: a step with an intent and no outcome is IN DOUBT (§4.2), and the
/// doubt is what makes recovery possible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cbor(index_only)]
pub enum RecordKind {
    #[n(0)]
    Intent,
    #[n(1)]
    Outcome,
}

/// How an effect may be reconciled after a crash (ADR-0007).
///
/// ⛔ THE CLASS IS A MANDATORY FIELD OF THE RECORD, and that is the point: §7.4.4 raised V5
/// to the compiler precisely so that "an effect without a declared class" IS NOT
/// EXPRESSIBLE. A defaulted class would put the decision back where the risk is — the
/// forgetfulness of whoever writes.
///
/// ⚠️ THE `Unrepeatable` DEFAULT OF ADR-0007 IS NOT GONE, BUT IT IS NOT REACHABLE HERE EITHER,
/// and the difference was measured on the types rather than assumed. `RecordV1::effect` is not
/// an `Option`, carries no `#[cbor(default)]`, and `EffectClass` implements no `Default`: a
/// record whose array is short does NOT decode to `Unrepeatable`, it decodes to
/// `RecordError::Malformed`. So there is no defaulting in this file, and reading one into it
/// would be reading a guarantee that is not here.
///
/// ⚠️ AND THE CASE THE DEFAULT IS FOR RUNS THE OTHER WAY ROUND. "Records written before the
/// class existed" is EMPTY BY CONSTRUCTION — V1 is the first version and the field has been
/// mandatory in it from the first byte ever written. The real case is a LATER version that
/// drops the field, and under ADR-0036 that is the ordinary shape of a field absent in another
/// version. ⚠️ THE FUTURE TENSE IS EXACT, AND TODAY NOTHING DEFAULTS ANYTHING: the version
/// that removes it will declare `Option<EffectClass>` with `#[cbor(default)]` and resolve
/// `None` to `Unrepeatable` — the safe reading, which suspends and asks — and until such a
/// version exists that mechanism is named here and implemented nowhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cbor(index_only)]
pub enum EffectClass {
    /// Ask the world what happened, then finish or re-plan.
    #[n(0)]
    Verifiable,
    /// Just run it again.
    #[n(1)]
    Idempotent,
    /// ⛔ Suspend and ask the user. Also what an undeclared class means.
    #[n(2)]
    Unrepeatable,
}

/// Whether the payload of this record crossed the untrusted boundary (I6, ADR-0014).
///
/// ⛔ THIS FIELD IS WHY IT IS HERE ON DAY ONE, and the reason is written where it was found:
/// road A4 of `crate::boundary`. Write external text into the journal, read it back as raw
/// bytes, and it comes out indistinguishable from an instruction — BYTES CARRY NO LABELS.
/// The record is where a label can live, and `boundary.rs` prices the alternative exactly:
/// "retrofitted later only by migrating the one irreproducible archive".
///
/// ⚠️ AND THE LIMIT IS THE TOKEN'S LIMIT, declared rather than discovered later: this proves
/// PROVENANCE, NOT CORRECTNESS (§6.3.2). Whoever writes a record can label it wrongly. What
/// it buys is that a reader can no longer LOSE the distinction, which is a different thing
/// from making it impossible to lie about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cbor(index_only)]
pub enum Trust {
    /// The payload may be used as an instruction.
    #[n(0)]
    Instruction,
    /// ⛔ The payload came from outside and stays outside (V20). Reading it back yields
    /// `Untrusted`, never a `String` that somebody may hand to the instruction channel.
    #[n(1)]
    Untrusted,
}

/// Version 1 of the durable record.
///
/// ⛔ EVERY FIELD CARRIES AN EXPLICIT INDEX, and the indices follow three rules that no
/// compiler enforces (§4.9.2): a new field is OPTIONAL and takes a NEW index; an index is
/// RETIRED AND NEVER REUSED — the gap stays; a non-additive change opens a NEW VERSION.
///
/// ⚠️ THE FUTURE TENSE IS EXACT, AND TODAY NOTHING HOLDS THEM. What WILL hold them is the
/// frozen bytes of `tests/frozen_bytes.rs`, a level 2 check that arrives at task 10 of this
/// milestone; that file does not exist yet. It is not a quibble about tense: measured at this
/// commit, moving a variant onto a FREE index leaves the whole bench green — the derive
/// renumbers encoding and decoding together, so no round trip can see it. Until the frozen
/// bytes land, the three rules above are a convention that a reader must keep, not a check.
///
/// ⚠️ `Clone` HAS NO CALLER IN THE CRATE AT THIS COMMIT, and is kept deliberately rather than
/// by inattention — the derive lists of `StepId` and `ClientId` are justified line by line and
/// this one owes the same. It is NOT removable by the #46 test, which asks what an outside
/// implementer is BLOCKED from doing: a record is the unit `journal` hands out and callers of
/// `read_back` will hold one while writing the next, and unlike `Ord` a missing `Clone` on a
/// struct with private-by-default construction elsewhere is not a one-line fix for them. It
/// costs nothing on the wire and nothing at run time unless called. ⚠️ NOT MEASURED BY
/// REMOVAL, and that is the honest state of it: `kernel` compiles without it today, so what is
/// written here is an argument and not a red.
#[derive(Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(array)]
pub struct RecordV1 {
    #[n(0)]
    pub kind: RecordKind,
    #[n(1)]
    pub effect: EffectClass,
    #[n(2)]
    pub trust: Trust,
    /// ⛔ THE BYTE-STRING ANNOTATION IS LOAD-BEARING, not decoration. Without it `minicbor`
    /// encodes a `Vec<u8>` as an ARRAY OF NUMBERS: it compiles, it round-trips, and it costs
    /// 1.91x — measured on 4096 B, 7813 against 4101. Gotcha #35.
    #[n(3)]
    #[cbor(with = "minicbor::bytes")]
    pub payload: Vec<u8>,
}

/// ⛔ THE PAYLOAD IS NOT PRINTED, and it is the same defence `Untrusted` carries, applied to
/// the type that holds the LABEL. Road A3 of the residual on `Untrusted::promote` says external
/// text reaching the logs is the same class of problem as external text reaching the
/// instruction channel; `boundary.rs` wrote `Debug` by hand to close it, and a DERIVED `Debug`
/// here reopens it in a weaker form — weaker because it reopens it on the one type whose
/// `trust` field exists to say the bytes came from outside. A `{:?}` in a log line, a panic
/// message, a failed `assert_eq!`, and the payload is out.
///
/// ⚠️ THE OTHER THREE FIELDS STAY READABLE, deliberately and for the reason the length stays on
/// `Untrusted`: a failed `assert_eq!` has to remain diagnostic. `kind`, `effect` and `trust`
/// are the kernel's own vocabulary — nobody outside chose them — and they are exactly what one
/// wants to read when a record comes back wrong. Only the payload is somebody else's.
///
/// ⚠️ Pinned by `the_debug_of_a_record_does_not_print_the_payload`, because a closed road that
/// no test holds is a road that reopens the day somebody puts `Debug` back in the derive list
/// above, with the gate staying green.
impl fmt::Debug for RecordV1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RecordV1 {{ kind: {:?}, effect: {:?}, trust: {:?}, payload: <{} bytes> }}",
            self.kind,
            self.effect,
            self.trust,
            self.payload.len()
        )
    }
}

/// The durable record. ⛔ A RECORD WITHOUT A VERSION IS NOT EXPRESSIBLE — rule 1 of §4.9.2,
/// held at level 1 by the type itself.
///
/// ⚠️ ONE VARIANT TODAY, AND IT IS NOT CEREMONY — written down because a YAGNI pass would
/// remove it and would be wrong. `minicbor` encodes an enum as a two-element array: variant
/// index, then value. So the version TRAVELS IN THE BYTES. Removing the enum would not
/// remove a level of indirection, it would remove a byte from the format — and that byte is
/// the whole of rule 1. Contrast with `Wakeup`, deleted at milestone 2 (errata E9): that one
/// wrapped a value and bought no error anywhere; this one is written to the archive.
///
/// ⚠️ `Debug` IS DERIVED HERE AND HAND-WRITTEN ON `RecordV1`, which is not an inconsistency:
/// the derive delegates to the inner impl, so `{:?}` on a whole record prints
/// `V1(RecordV1 { .. payload: <N bytes> })` and the payload stays shut. Nothing outside chose
/// the word `V1`. ⚠️ `Clone` carries the same note as `RecordV1`'s, and for the same reason.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(array)]
pub enum Record {
    #[n(0)]
    V1(#[n(0)] RecordV1),
}

/// What can go wrong DECODING a record.
///
/// ⚠️ Deliberately poor, and for the reason `JournalError` is: a rich error invites the
/// kernel to branch on the reason, and there is exactly one thing to do with a record that
/// will not decode.
///
/// ⚠️ THIS SENTENCE SAID "encoding or decoding" UNTIL 2026-08-10, and it is dated rather than
/// silently rewritten: `encode` returned a `Result` that could never be `Err`, and when that
/// signature went so did half of this type's job. Nothing is lost by the narrowing — decoding
/// is where the failure really lives, because the bytes come from an archive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordError {
    /// The bytes are not a record of any version this build knows.
    Malformed,
}

impl Record {
    /// Encodes to the bytes the `journal` port exchanges. ⛔ IT CANNOT FAIL, AND THE SIGNATURE
    /// SAYS SO — this returned `Result<Vec<u8>, RecordError>` until 2026-08-10, with the open
    /// question below beside it, and the question is now CLOSED.
    ///
    /// ⛔ WHY THE `Err` WAS UNREACHABLE, kept because it is the EVIDENCE that removing it is
    /// safe and not an opinion about it. Measured on the types rather than deduced:
    /// `minicbor::encode` returns `Result<(), minicbor::encode::Error<W::Error>>`, and `Vec<u8>`
    /// implements `minicbor::encode::Write` with `type Error = core::convert::Infallible`, so
    /// the WRITE road of that error is uninhabited here. Its other two roads — a message and a
    /// custom error — have exactly two producers in `minicbor` 2.3.0, `SystemTime` and a
    /// non-UTF-8 `Path`, and NEITHER IS IN THIS TYPE'S GRAPH: three `index_only` enums and a
    /// byte string. So the compiler could not see it, but nothing could produce it.
    ///
    /// ⛔ AND THE THREE REASONS FOR CLOSING IT NOW rather than at the version that first needs
    /// an error. The repository already holds this position and wrote it down for `Ipc::accept`:
    /// A `Result` THAT CAN NEVER BE `Err` IS DEAD SURFACE, of the kind that port pruned three
    /// derives and a getter for. `Untrusted::promote` will call this at task 7, and an `.expect`
    /// that cannot fire, sitting INSIDE THE CODE OF THE UNTRUSTED-DATA BOUNDARY, is debt and not
    /// prudence — a reader of that file has to establish that it cannot fire before trusting the
    /// line it is on. And the call sites are TWO today and will be many afterwards: the edit
    /// costs least now and most later.
    ///
    /// ⚠️ THE PRICE IS DECLARED, and it is the one the open question named: the day a later
    /// version encodes something that CAN fail, this signature changes and every call site with
    /// it. That is a compiler error at each of them, which is the direction this repository
    /// accepts everywhere else — see `a_record_is_matched_exhaustively_and_that_is_the_point`.
    /// ⚠️ AND IT HIDES THE ASYMMETRY WITH `decode`, which really can fail: `RecordError` STAYS
    /// for that reason and only that one, so the type is now `decode`'s alone.
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        // ⚠️ THE RESULT IS DROPPED AND NOT `expect`ed, AND THAT IS THE POINT OF THE CHANGE: an
        // `.expect` here would only move the dead branch one level down, from many call sites to
        // one. ⛔ AND THE IMPOSSIBLE CASE IS CONTAINED RATHER THAN IGNORED — measured on the
        // shape of the failure, not hoped: an encoder that stopped early would leave `bytes`
        // TRUNCATED OR EMPTY, and `Record::decode` answers `Malformed` to both
        // (`bytes_that_are_not_a_record_decode_to_malformed` holds exactly those two inputs).
        // Reconciliation reads a record it cannot decode as `SuspendAndAsk`, so the archive
        // would stop the system rather than hand it a wrong answer — ADR-0007's own rule.
        let _ = minicbor::encode(self, &mut bytes);
        bytes
    }

    /// Decodes from the bytes the `journal` port hands back.
    pub fn decode(bytes: &[u8]) -> Result<Self, RecordError> {
        minicbor::decode(bytes).map_err(|_| RecordError::Malformed)
    }
}
