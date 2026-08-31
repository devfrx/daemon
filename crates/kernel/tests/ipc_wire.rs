//! The schema of the `ipc` channel. ⛔ OUTSIDE THE CRATE, like `framing` and `worker_wire`.

use kernel::arbiter::{ComputeClass, Mib, Preemption};
use kernel::framing::{self, LENGTH_WIDTH, WireError};
use kernel::time::Millis;
use kernel::wire::ipc::{GrantRequest, IpcMessage, Verdict};

fn a_request() -> GrantRequest {
    GrantRequest {
        reserved_vram: Mib::new(2048),
        compute_class: ComputeClass::Interactive,
        preemption: Preemption::After(Millis::new(500)),
    }
}

#[test]
fn a_grant_request_survives_the_round_trip() {
    let message = IpcMessage::Request(a_request());
    let bytes = message.encode().expect("encode");
    assert_eq!(IpcMessage::decode(&bytes), Ok(message));
}

#[test]
fn a_verdict_survives_the_round_trip() {
    // ⛔ THIS IS THE PROBE THAT EXERCISES THE DISCRIMINANT, and it is why §6.7 asks for TWO
    // messages rather than one: with a single message type the tag never varies, and a bug in
    // how it is written or read would be invisible. Same shape as the journal freezing THREE
    // records instead of one.
    let message = IpcMessage::Verdict(Verdict::Refused {
        asked: Mib::new(4096),
        ceiling: Mib::new(1024),
    });
    let bytes = message.encode().expect("encode");
    assert_eq!(IpcMessage::decode(&bytes), Ok(message));
}

#[test]
fn a_queued_verdict_survives_the_round_trip() {
    // ⛔ THE THIRD WAY OF THE VERDICT, AND IT WAS THE ONE NOTHING TOUCHED. `Refused` makes the
    // round trip above and `Granted` is encoded by the two probes below, so `Queued` was the
    // only variant of this enum that never reached the wire at all -- a wrong tag on it would
    // have been read by no assertion in the workspace. Mutation G5 measures the gap it closes.
    //
    // ⚠️ ITS OWN `#[test]` AND NOT A SECOND ASSERTION IN `a_verdict_survives_the_round_trip`,
    // for the reason written on the pair at the bottom of this file -- gotcha #14. That probe
    // asserts on `Refused` FIRST, so a red there would stop it before this input ran, and the
    // one variant with no other exercise anywhere is exactly the one that must not depend on
    // another assertion passing.
    let message = IpcMessage::Verdict(Verdict::Queued);
    let bytes = message.encode().expect("encode");
    assert_eq!(IpcMessage::decode(&bytes), Ok(message));
}

#[test]
fn a_message_with_a_tail_does_not_decode() {
    let mut bytes = IpcMessage::Verdict(Verdict::Granted)
        .encode()
        .expect("encode");
    bytes.push(0xFF);
    assert_eq!(IpcMessage::decode(&bytes), Err(WireError::TrailingBytes));
}

#[test]
fn junk_inside_the_declared_length_does_not_decode() {
    // ⛔ THE ENVELOPE IS HONEST HERE and the body is not -- the other half of the pair, and it
    // is a DIFFERENT check. See the same probe in `worker_wire.rs`.
    let good = IpcMessage::Verdict(Verdict::Granted)
        .encode()
        .expect("encode");
    let mut junked = good[LENGTH_WIDTH..].to_vec();
    junked.push(0xFF);
    let bytes = framing::frame(&junked).expect("frame");
    assert_eq!(IpcMessage::decode(&bytes), Err(WireError::Malformed));
}

#[test]
fn an_empty_body_in_an_honest_envelope_does_not_decode() {
    // ⛔ THE HALF THAT NOTHING WOULD HOLD WITHOUT IT, AND IT IS NOT A LATE ADDITION HERE: the
    // twin bench got these two on 2026-08-31, THE DAY AFTER the plan that dictates this file
    // was written, so the four probes this file was asked for would have left the
    // `map_err(|_| WireError::Malformed)` of `IpcMessage::decode` reached by NOTHING -- which
    // is exactly what mutation W9 measured on `wire::worker`, where it left the ENTIRE
    // workspace green. The four cannot reach it: both round trips DECODE, the tail outside the
    // envelope dies inside `framing::unframe`, and the junk inside the declared length decodes
    // successfully and falls on the consumed-bytes comparison, which is a different check.
    //
    // ⛔ THE EMPTY BODY IS ALSO WHAT `IpcMessage::encode` CONTAINS. That method takes the
    // `Vec` of a failed encoding as EMPTY rather than propagating an error, on the argument
    // written beside it, and the argument is only worth what this probe buys: an empty body
    // does not decode, so a stopped encoder produces a frame the peer REFUSES.
    //
    // ⛔ THE ENVELOPE IS HONEST HERE, and that is what makes this a probe of the SCHEMA rather
    // than of the envelope: `frame(&[])` is `00 00 00 00`, `unframe` answers `Ok(&[])`, and the
    // refusal comes from the body decoder alone. Break `framing`'s two checks and this survives.
    let bytes = framing::frame(&[]).expect("frame");
    assert_eq!(bytes, [0x00, 0x00, 0x00, 0x00]);
    assert_eq!(IpcMessage::decode(&bytes), Err(WireError::Malformed));
}

#[test]
fn a_truncated_body_in_an_honest_envelope_does_not_decode() {
    // ⚠️ THE OTHER INPUT OF THE PAIR, AND IT IS A SEPARATE `#[test]` ON PURPOSE -- gotcha #14.
    // One probe holding both would stop at the first failing assertion and never exercise the
    // second input, so a red would hide half of what it was written to see. The twin measured
    // that both of its two go red under the mutation; the same is measured here, mutation G4.
    //
    // "Truncated" is the shape a stopped encoder really leaves behind -- the prefix is well
    // formed as far as it goes. Cut from a real message rather than written as a literal, and
    // from the LONGEST of the two, so the cut lands inside the request and not on its tag.
    // ⚠️ NOT the same fault as `a_message_with_a_tail_does_not_decode`: there the envelope
    // LIES, here it tells the truth about a body that is genuinely short.
    let good = IpcMessage::Request(a_request()).encode().expect("encode");
    let body = &good[LENGTH_WIDTH..];
    let bytes = framing::frame(&body[..body.len() / 2]).expect("frame");
    assert_eq!(IpcMessage::decode(&bytes), Err(WireError::Malformed));
}
