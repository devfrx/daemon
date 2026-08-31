//! The schema of the channel towards the workers. ⛔ OUTSIDE THE CRATE, like `framing.rs`.

use kernel::arbiter::Mib;
use kernel::framing::{self, LENGTH_WIDTH, WireError};
use kernel::wire::worker::FromWorker;

#[test]
fn a_fragment_survives_the_round_trip() {
    let message = FromWorker::Fragment(vec![9, 8, 7]);
    let bytes = message.encode().expect("encode");
    assert_eq!(FromWorker::decode(&bytes), Ok(message));
}

#[test]
fn a_vram_peak_survives_the_round_trip() {
    let message = FromWorker::VramPeak(Mib::new(1536));
    let bytes = message.encode().expect("encode");
    assert_eq!(FromWorker::decode(&bytes), Ok(message));
}

#[test]
fn the_byte_string_annotation_is_measured_and_not_asserted() {
    // ⛔ READING THE ATTRIBUTE IN THE SOURCE PROVES NOTHING -- what the annotation buys is a
    // SIZE. Both shapes compile, both round-trip, both are correct; one costs close to double
    // the traffic in silence. §6.10.4 measured that ratio on a 4096 B audio fragment and is
    // ITS house -- the figures are not copied here, and this comment is the house of the ones
    // below instead (gotcha #31; they stood in two more places until 2026-08-31).
    //
    // ⛔ THE BODY MUST NOT BE ALL ZEROS, AND THAT IS THE WHOLE OF WHY THIS PROBE IS NOT
    // VACUOUS. CBOR encodes 0..=23 in a SINGLE byte, so an array of 4096 zeros and a byte
    // string of 4096 zeros cost THE SAME and the annotation buys nothing measurable.
    //
    // ⛔ MEASURED ON 2026-08-31 from outside the crate, on a throwaway probe compiled, run
    // and deleted in the same run -- against a mirror type identical but for the attribute:
    //
    //   body of 4096 zeros          annotated 4106 framed bytes, bare 4106  -> 1.00x
    //   body of `(i % 256) as u8`   annotated 4106 framed bytes, bare 7818  -> 1.90x
    //
    // With zeros BOTH forms sit under the bound below, so the probe would pass whether the
    // annotation were there or not. The bytes chosen below leave the one-byte range, and
    // there the ratio is the one §6.10.4 measured. The 4106 decomposes exactly: 4 envelope, `82 00`
    // for the enum, `81` for the variant's field array, `59 10 00` for the byte-string
    // header, then the 4096.
    //
    // ⚠️ THE ASSERTION IS A BOUND AND NOT AN EQUALITY, and that is deliberate: an exact
    // number would go red the day the envelope or the variant index changes by a byte, i.e.
    // where the promise is KEPT (gotcha #24, the precedent is PL-1 and its `0600`).
    let body: Vec<u8> = (0..4096).map(|i| (i % 256) as u8).collect();
    let bytes = FromWorker::Fragment(body).encode().expect("encode");
    assert!(bytes.len() < 4096 + 64, "encoded {} bytes", bytes.len());
}

#[test]
fn a_frame_with_a_tail_does_not_decode() {
    let mut bytes = FromWorker::VramPeak(Mib::new(1)).encode().expect("encode");
    bytes.push(0xFF);
    assert_eq!(FromWorker::decode(&bytes), Err(WireError::TrailingBytes));
}

#[test]
fn junk_inside_the_declared_length_does_not_decode() {
    // ⛔ THE OTHER HALF, AND IT IS A DIFFERENT CHECK: here the ENVELOPE is honest -- the
    // declared length matches the body exactly -- and it is the body that carries a complete
    // CBOR element followed by a byte. `unframe` cannot see it; `position() != len()` can.
    // Remove either check and one of these two probes survives on its own merits.
    let good = FromWorker::VramPeak(Mib::new(1)).encode().expect("encode");
    let body = &good[LENGTH_WIDTH..];
    let mut junked = body.to_vec();
    junked.push(0xFF);
    let bytes = framing::frame(&junked).expect("frame");
    assert_eq!(FromWorker::decode(&bytes), Err(WireError::Malformed));
}

#[test]
fn an_empty_body_in_an_honest_envelope_does_not_decode() {
    // ⛔ THE HALF OF `FromWorker::encode`'s BORROWED ARGUMENT THAT NOTHING HELD. That method
    // drops the `Result` of `minicbor::encode` on the precedent of `Record::encode`, and the
    // precedent does NOT say "the error is unreachable, so throw it away". It says the
    // impossible case is CONTAINED: an encoder that stopped early leaves bytes TRUNCATED OR
    // EMPTY, and the decoder answers `Malformed` to BOTH. In `record.rs` that containment is a
    // NAMED probe, `bytes_that_are_not_a_record_decode_to_malformed`, which holds both of them
    // among its four inputs. `FromWorker` had no analogue until 2026-08-31, so the argument had a
    // half nothing held and the `map_err` in `FromWorker::decode` was reached by NO probe in
    // this workspace -- mutation W9, which left the ENTIRE workspace green.
    //
    // ⛔ THE ENVELOPE IS HONEST HERE, and that is what makes this a probe of the SCHEMA rather
    // than of the envelope: `frame(&[])` is `00 00 00 00`, `unframe` answers `Ok(&[])`, and the
    // refusal comes from the CBOR decoder alone. Break `framing`'s two checks and this survives.
    let bytes = framing::frame(&[]).expect("frame");
    assert_eq!(bytes, [0x00, 0x00, 0x00, 0x00]);
    assert_eq!(FromWorker::decode(&bytes), Err(WireError::Malformed));
}

#[test]
fn a_truncated_body_in_an_honest_envelope_does_not_decode() {
    // ⚠️ THE OTHER INPUT OF THE PAIR, AND IT IS A SEPARATE `#[test]` ON PURPOSE -- gotcha #14.
    // One probe holding both would stop at the first failing assertion and never exercise the
    // second input, so a red would hide half of what it was written to see. Measured, not
    // argued: under W9 BOTH of these go red, which is the evidence that each reaches the branch
    // on its own merits.
    //
    // "Truncated" is the shape a stopped encoder really leaves behind -- the prefix is well
    // formed as far as it goes. Cut from a real message rather than written as a literal, the
    // way the twin cuts a real record in half. ⚠️ NOT the same fault as
    // `a_frame_with_a_tail_does_not_decode`: there the envelope LIES, here it tells the truth
    // about a body that is genuinely short.
    let good = FromWorker::VramPeak(Mib::new(1536))
        .encode()
        .expect("encode");
    let body = &good[LENGTH_WIDTH..];
    let bytes = framing::frame(&body[..body.len() / 2]).expect("frame");
    assert_eq!(FromWorker::decode(&bytes), Err(WireError::Malformed));
}
