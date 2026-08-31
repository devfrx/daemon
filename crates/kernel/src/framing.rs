//! The envelope of the two private channels: a declared length, then the body.
//!
//! ⛔ WHAT IS SHARED IS NEITHER THE TRANSPORT NOR THE SCHEMA -- it is envelope bytes.
//! ADR-0035 reads I4 as "one transport and one schema PER PRIVATE CHANNEL", and ADR-0037
//! distrusts arguments of SYMMETRY between the two channels. Neither is contradicted: both
//! peers must read a length prefix whatever the body format is, so the problem here is
//! LITERALLY the same one and not a symmetric one. §6.3 of the milestone 6 design says so,
//! and whether that reading holds is open item 8 for the owner.
//!
//! ⛔ THE WIDTH IS DECIDED HERE AND NOWHERE ELSE (§6.3). Four bytes, big-endian.

use alloc::vec::Vec;

/// The width of the declared length, in bytes.
///
/// ⚠️ PUBLIC SINCE 2026-08-31, AND IT IS A DEDUPLICATION RATHER THAN AN API WIDENING:
/// `crates/kernel/tests/worker_wire.rs` cuts a body off a framed message and spelled the `4`
/// itself, so the width the module doc claims is "decided here and nowhere else" had a second
/// site. ⛔ READING THIS CONSTANT PROVES NOTHING, and no probe may use it as an oracle: the
/// oracle is `the_declared_length_is_four_bytes_big_endian`, which asserts the prefix as a
/// LITERAL. A probe that compared against this constant would assert the code against itself.
pub const LENGTH_WIDTH: usize = 4;

/// The longest body this envelope can declare.
///
/// ⚠️ THE GUARD ABOVE IT IS DECLARED AND NOT EXERCISED, and saying so is the point: reaching
/// it needs a body of four gibibytes, so no probe in this repository can produce one. What is
/// held instead is the WIDTH, by `the_declared_length_is_four_bytes_big_endian`: while the
/// prefix is four bytes this constant cannot be anything else.
pub const MAX_BODY_LEN: usize = u32::MAX as usize;

/// What can go wrong reading an envelope.
///
/// ⚠️ NO VARIANT CARRIES A PAYLOAD, and that is the shape of the project rather than an
/// omission: no error in this repository carries the value it consumed. The caller that wants
/// the numbers has the bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireError {
    /// Fewer bytes than the frame declares -- the prefix itself may be missing.
    Incomplete,
    /// More bytes than the frame declares.
    TrailingBytes,
    /// The body is longer than a declared length can express. See `MAX_BODY_LEN`.
    TooLong,
    /// The body did not decode as a message of this channel, or it carried a complete
    /// element followed by something else. ⚠️ PRODUCED BY THE SCHEMAS, NOT BY THIS MODULE:
    /// the envelope knows how many bytes there are, never what they mean.
    Malformed,
}

/// Wraps a body in its envelope.
pub fn frame(body: &[u8]) -> Result<Vec<u8>, WireError> {
    if body.len() > MAX_BODY_LEN {
        return Err(WireError::TooLong);
    }
    let mut bytes = Vec::with_capacity(LENGTH_WIDTH + body.len());
    bytes.extend_from_slice(&(body.len() as u32).to_be_bytes());
    bytes.extend_from_slice(body);
    Ok(bytes)
}

/// Reads a body out of its envelope.
///
/// ⛔ THE TWO FAILURES ARE NOT THE SAME FAULT, and §3.2 of the design puts them in one table:
/// a TAIL is caught by a decoder that checks its own position, a TRUNCATION is caught by
/// NOTHING BUT a declared length -- the tail is not there, and the CBOR can be complete all
/// the same. That is why this function exists on top of the body decoder and not instead of
/// it.
///
/// ⛔ AND IT IS NOT A STREAM READER, WHICH IS A DECLARED LIMIT AND NOT AN OVERSIGHT. Refusing
/// every tail means this takes ONE frame and exactly one: it cannot walk a byte stream that
/// carries several frames back to back, and it cannot be asked where the next one begins. The
/// catalogue line `Q4 · I5 · §6.10` is what imposes it -- the bytes consumed must equal the
/// declared length, and a reader that handed back the rest would be free to consume fewer.
/// ⚠️ A REAL TRANSPORT WILL WANT A SECOND ENTRY POINT, one that answers "this frame, and where
/// the next starts", and it is a NEW function beside this one rather than a loosening of this
/// one -- `TrailingBytes` is the whole of what `a_frame_with_a_tail_is_refused` holds. Written
/// here so the transport does not rediscover it, the way `MAX_BODY_LEN` declares its own limit.
pub fn unframe(bytes: &[u8]) -> Result<&[u8], WireError> {
    if bytes.len() < LENGTH_WIDTH {
        return Err(WireError::Incomplete);
    }
    let (prefix, body) = bytes.split_at(LENGTH_WIDTH);
    let mut declared = [0u8; LENGTH_WIDTH];
    declared.copy_from_slice(prefix);
    let declared = u32::from_be_bytes(declared) as usize;
    if body.len() < declared {
        return Err(WireError::Incomplete);
    }
    if body.len() > declared {
        return Err(WireError::TrailingBytes);
    }
    Ok(body)
}
