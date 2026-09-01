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
/// ⚠️ PUBLIC SINCE 2026-08-31, AND IT IS A DEDUPLICATION AND AN API WIDENING BOTH:
/// `crates/kernel/tests/worker_wire.rs` cuts a body off a framed message and spelled the `4`
/// itself, so the width the module doc claims is "decided here and nowhere else" had a second
/// site. ⚠️ THE WIDENING IS REAL: outside this file the only caller is that bench.
/// ⛔ READING THIS CONSTANT PROVES NOTHING, and no probe may use it as an oracle: the
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
/// ⚠️ NO VARIANT HERE CARRIES A PAYLOAD, and that is deliberate rather than an omission: this
/// error does not carry the value it consumed, and the caller that wants the numbers has the
/// bytes.
///
/// ⛔ RECALL OF 2026-09-01 — THIS SAID "and that is THE SHAPE OF THE PROJECT: no error in THIS
/// REPOSITORY carries the value it consumed", AND THE GENERALISATION WAS FALSE WHEN IT WAS
/// WRITTEN. Measured with `git log -S` rather than remembered: `platform::journal::OpenError` has
/// carried `File(io::Error)` and `Engine(redb::Error)` since `bb2e440`, 2026-08-10, and this
/// sentence landed on 2026-08-31 — three weeks later. What was true was the narrower claim, which
/// is what is left standing above: that no error in `kernel` carried one, and that none carries
/// the value it CONSUMED. The second half still holds everywhere; the first is now dated too.
///
/// ⛔ AND FROM 2026-09-01 THE `kernel` HALF IS SPENT AS WELL: `permission::PermissionError`
/// composes `JournalError` and `RecordError`, because `permission::is_granted` returns a `bool`
/// and a `bool` has no room for "I do not know". Those payloads are CAUSES and not consumed
/// values, they keep the type `Copy`, and the reasoning lives on that type — one house. The
/// sentence is corrected rather than answered underneath itself: a true line appended under a
/// false one leaves the false one standing, which is finding A-2 of this project's own audit.
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
