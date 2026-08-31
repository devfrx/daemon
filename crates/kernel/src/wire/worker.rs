//! The schema of the channel towards the workers (§6.10, ADR-0037).
//!
//! ⛔ THIS TAKES THE MECHANICS OF `record.rs` AND NOT ITS DISCIPLINE, and §6.10.3 says it in
//! as many words: no version enum, no register of retired indices, NO FROZEN BYTES. I4 gives
//! up versioning, and what stands in its place is the build stamp of §6.1.2 -- which this
//! milestone deliberately does NOT build (§3.4). Until it exists, NOTHING REFUSES A STALE
//! PEER, and the trigger is the first real worker process (§0.2).
//!
//! ⛔ ONE DIRECTION ONLY, worker -> core, AND THE OTHER IS A DECLARED NON-CONSTRUCTION.
//! Nothing written imposes a core -> worker message today: `instruct_one` and
//! `instruct_stream` take an opaque `Frame` and no production caller exists. §6.10.4 imposes
//! exactly these two -- it measures the annotation ON AN AUDIO FRAGMENT and names the VRAM
//! peak as the field this channel puts into the journal. Inventing a downward vocabulary now
//! would freeze it against an imaginary consumer -- gotcha #46 from the wrong side, the same
//! reason §3.4 gives for the stamp. The trigger is the same one: the first real worker
//! process (§0.2), which is what gives a downward message its first caller.

use alloc::vec::Vec;
use minicbor::{Decode, Encode};

use crate::arbiter::Mib;
use crate::framing::{self, WireError};

/// What a worker sends up.
///
/// ⛔ EVERY BYTE THAT RISES IS COVERED BY A RECEIPT (§6.10.1). This enum says what is INSIDE
/// a frame; it never says that a frame may arrive unsolicited -- that one is a FAULT, and the
/// port already has the word for it, `ProcessError::UnsolicitedFrame`.
///
/// ⛔ `#[cbor(array)]` IS WRITTEN OUT EVEN THOUGH IT IS THE DEFAULT, exactly as `record.rs`
/// does, and for the reason written there: A DEFAULT NOBODY WROTE DOWN IS A DEFAULT SOMEBODY
/// CHANGES. ⚠️ THE ATTRIBUTE WAS ABSENT UNTIL 2026-08-31, on an argument that was wrong twice.
/// It made the FROZEN BYTES of the journal the REASON `record.rs` writes its default out, when
/// they are only what would CATCH the change afterwards; and it leaned on the build stamp of
/// §6.1.2 to say both ends of this wire are rebuilt together, four lines under a paragraph of
/// this same file saying the stamp is NOT BUILT and NOTHING REFUSES A STALE PEER.
///
/// ⛔ READ STRAIGHT, THE ASYMMETRY RUNS THE OTHER WAY. `record.rs` has an oracle that goes red
/// if the shape moves; this channel has NONE, §6.10.3 forbids it one, and both peers live
/// OUTSIDE this workspace. A silent switch to `map` would break them and no probe here would
/// see it -- which makes writing the choice down worth more here, not less.
///
/// ⚠️ AND THAT IT COSTS NOTHING WAS MEASURED, not assumed: with and without the attribute on
/// this type and on `Mib`, the framed bytes are identical -- `Fragment(vec![9, 8, 7])` is
/// `00 00 00 07 82 00 81 43 09 08 07` and `VramPeak(Mib::new(1536))` is
/// `00 00 00 07 82 01 81 81 19 06 00` under both shapes, and the whole workspace stays green.
/// Measured 2026-08-31 on a throwaway probe outside the crate, run and deleted in the same run.
/// ⛔ THOSE BYTES ARE A MEASUREMENT AND NOT AN ORACLE: nothing holds them, and nothing may.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(array)]
pub enum FromWorker {
    /// One fragment of an instructed answer -- an audio chunk, a piece of a stream.
    ///
    /// ⛔ THE BYTE-STRING ANNOTATION IS LOAD-BEARING, not decoration, and the same sentence
    /// sits on `RecordV1::payload`. Without it `minicbor` writes AN ARRAY OF NUMBERS: it
    /// compiles, it round-trips, and it is correct -- it costs close to DOUBLE the traffic in
    /// silence, which is why the probe that holds it asserts a SIZE and not the attribute.
    ///
    /// ⚠️ THE FIGURES ARE NOT COPIED HERE, AND THAT IS THE POINT. §6.10.4 measured the ratio on
    /// a 4096 B audio fragment and is its house; the measurement that holds THIS line lives in
    /// the comment of `the_byte_string_annotation_is_measured_and_not_asserted`, in
    /// `crates/kernel/tests/worker_wire.rs`. Both pairs stood verbatim in this doc until
    /// 2026-08-31 -- gotcha #31, and a number in three houses is corrected in none of them.
    #[n(0)]
    Fragment(
        #[n(0)]
        #[cbor(with = "minicbor::bytes")]
        Vec<u8>,
    ),

    /// The VRAM peak the work actually reached (§5.2.2).
    ///
    /// ⚠️ IT IS THE ONE FIELD THIS CHANNEL PUTS INTO THE JOURNAL, and there it is subject to
    /// §4.9 -- optional, new index. Here it is not: this schema has no version enum at all.
    #[n(1)]
    VramPeak(#[n(0)] Mib),
}

impl FromWorker {
    /// Encodes the message and wraps it in its envelope.
    pub fn encode(&self) -> Result<Vec<u8>, WireError> {
        let mut body = Vec::new();
        let _ = minicbor::encode(self, &mut body);
        framing::frame(&body)
    }

    /// Reads a message out of an envelope.
    ///
    /// ⛔ TWO CHECKS AND NOT ONE, and they catch different faults: `unframe` catches a frame
    /// whose length does not match, `position() != body.len()` catches a body that carries a
    /// complete element AND SOMETHING AFTER IT. A CBOR decoder stops at the first complete
    /// element; the second check is the line `Record::decode` already carries, and the reason
    /// is written there -- finding AUD-047.
    pub fn decode(bytes: &[u8]) -> Result<Self, WireError> {
        let body = framing::unframe(bytes)?;
        let mut decoder = minicbor::Decoder::new(body);
        let message = decoder.decode().map_err(|_| WireError::Malformed)?;
        if decoder.position() != body.len() {
            return Err(WireError::Malformed);
        }
        Ok(message)
    }
}
