//! The schema of the channel towards the workers. ⛔ OUTSIDE THE CRATE, like `framing.rs`.

use kernel::arbiter::Mib;
use kernel::framing::WireError;
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
    // SIZE, and §6.10.4 measured it: a 4096 B audio fragment costs 4101 bytes as a byte
    // string and 7813 as an array of numbers, i.e. 1.91x. Both compile, both round-trip,
    // both are correct; one costs double the traffic in silence.
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
    // there the ratio is §6.10.4's 1.91x. The 4106 decomposes exactly: 4 envelope, `82 00`
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
    let body = &good[4..];
    let mut junked = body.to_vec();
    junked.push(0xFF);
    let bytes = kernel::framing::frame(&junked).expect("frame");
    assert_eq!(FromWorker::decode(&bytes), Err(WireError::Malformed));
}
