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

use alloc::vec::Vec;
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
/// ⚠️ The `irripetibile` default of ADR-0007 is NOT gone: it lives where it is actually
/// useful, on records READ BACK from a journal written before the class existed. Under
/// ADR-0036 that is not a special case — it is the ordinary case of a field absent in an
/// earlier version.
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
/// ⛔ EVERY FIELD CARRIES AN EXPLICIT INDEX, and the indices follow four rules that no
/// compiler enforces (§4.9.2): a new field is OPTIONAL and takes a NEW index; an index is
/// RETIRED AND NEVER REUSED — the gap stays; a non-additive change opens a NEW VERSION.
/// What holds them is the frozen bytes of `tests/frozen_bytes.rs`, a level 2 check.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
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

/// The durable record. ⛔ A RECORD WITHOUT A VERSION IS NOT EXPRESSIBLE — rule 1 of §4.9.2,
/// held at level 1 by the type itself.
///
/// ⚠️ ONE VARIANT TODAY, AND IT IS NOT CEREMONY — written down because a YAGNI pass would
/// remove it and would be wrong. `minicbor` encodes an enum as a two-element array: variant
/// index, then value. So the version TRAVELS IN THE BYTES. Removing the enum would not
/// remove a level of indirection, it would remove a byte from the format — and that byte is
/// the whole of rule 1. Contrast with `Wakeup`, deleted at milestone 2 (errata E9): that one
/// wrapped a value and bought no error anywhere; this one is written to the archive.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum Record {
    #[n(0)]
    V1(#[n(0)] RecordV1),
}

/// What can go wrong encoding or decoding a record.
///
/// ⚠️ Deliberately poor, and for the reason `JournalError` is: a rich error invites the
/// kernel to branch on the reason, and there is exactly one thing to do with a record that
/// will not decode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordError {
    /// The bytes are not a record of any version this build knows.
    Malformed,
}

impl Record {
    /// Encodes to the bytes the `journal` port exchanges.
    ///
    /// ⛔ DECLARED OPEN QUESTION, AND IT IS NOT RESOLVED HERE: THIS `Result` CANNOT BE `Err`
    /// TODAY. The repository already has the sentence for that shape, on `Ipc::accept` — a
    /// `Result` that can never be `Err` is DEAD SURFACE, of the kind that port pruned three
    /// derives and a getter for. Measured on the types rather than deduced: `minicbor::encode`
    /// returns `Result<(), minicbor::encode::Error<W::Error>>`, and `Vec<u8>` implements
    /// `minicbor::encode::Write` with `type Error = core::convert::Infallible`, so the WRITE
    /// road of that error is uninhabited here. Its other two roads — a message and a custom
    /// error — have exactly two producers in `minicbor` 2.3.0, `SystemTime` and a non-UTF-8
    /// `Path`, and NEITHER IS IN THIS TYPE'S GRAPH: three `index_only` enums and a byte
    /// string. So the compiler cannot see it, but nothing can produce it.
    ///
    /// ⚠️ AND THE PRICE OF CLOSING IT IS THE SIGNATURE, not a variant of `RecordError` — the
    /// same shape as `Ipc::accept`'s residue, and the same trap: widening the enum would not
    /// touch this. Closing it means returning `Vec<u8>` bare, which costs an edit at every
    /// call site the day a later version encodes something that CAN fail, and hides the
    /// asymmetry with `decode`, which really can. Left as it is deliberately: milestone 3 has
    /// further tasks that consume this signature, and changing it on incomplete information
    /// is churn.
    pub fn encode(&self) -> Result<Vec<u8>, RecordError> {
        let mut bytes = Vec::new();
        minicbor::encode(self, &mut bytes).map_err(|_| RecordError::Malformed)?;
        Ok(bytes)
    }

    /// Decodes from the bytes the `journal` port hands back.
    pub fn decode(bytes: &[u8]) -> Result<Self, RecordError> {
        minicbor::decode(bytes).map_err(|_| RecordError::Malformed)
    }
}
