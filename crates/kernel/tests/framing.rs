//! The envelope shared by the two private channels. ⛔ THE PROBES LIVE OUTSIDE THE CRATE on
//! purpose: what they hold is that the envelope is USABLE from outside, which is the only
//! form of "the boundary is real" this repository accepts (milestone 3, task 8).

use kernel::framing::{self, WireError};

#[test]
fn a_framed_body_comes_back_exactly() {
    let body = [1u8, 2, 3, 4, 5];
    let framed = framing::frame(&body).expect("frame");
    assert_eq!(framing::unframe(&framed), Ok(&body[..]));
}

#[test]
fn the_declared_length_is_four_bytes_big_endian() {
    // ⛔ THE BYTE ORDER IS AN ASSERTION AND NOT A COMMENT. ADR-0037 chooses a wire format on
    // what the PEER can read: `DataView.getUint32(0)` in TypeScript and `struct.unpack(">I")`
    // in Python are the forms that need no flag. Flipped to little-endian, nothing else in
    // this workspace would go red -- both peers live outside it.
    let framed = framing::frame(&[0xAA]).expect("frame");
    assert_eq!(framed, [0x00, 0x00, 0x00, 0x01, 0xAA]);
}

#[test]
fn a_truncated_frame_is_refused() {
    // Declares five, carries two.
    let bytes = [0x00, 0x00, 0x00, 0x05, 0x01, 0x02];
    assert_eq!(framing::unframe(&bytes), Err(WireError::Incomplete));
}

#[test]
fn bytes_shorter_than_the_prefix_are_refused() {
    // ⚠️ NOT the same failure as the one above, and it is worth its own probe: here there is
    // no declared length AT ALL, so the code path that reads it must not be reached.
    assert_eq!(
        framing::unframe(&[0x00, 0x00, 0x00]),
        Err(WireError::Incomplete)
    );
}

#[test]
fn a_frame_with_a_tail_is_refused() {
    // Declares one, carries three. ⛔ THIS IS THE HALF A CBOR DECODER CANNOT SEE: it stops at
    // the first complete element and ignores what follows (gotcha #34, measured in §6.10.4).
    let bytes = [0x00, 0x00, 0x00, 0x01, 0x01, 0x02, 0x03];
    assert_eq!(framing::unframe(&bytes), Err(WireError::TrailingBytes));
}

#[test]
fn two_frames_back_to_back_come_out_one_at_a_time() {
    // ⛔ THE CASE `unframe` REFUSES BY DESIGN, and the reason this function exists: a stream
    // carries frames back to back, and `unframe` answers `TrailingBytes` to the whole buffer.
    let mut stream = framing::frame(b"first").expect("frame");
    stream.extend_from_slice(&framing::frame(b"second").expect("frame"));
    assert_eq!(
        framing::unframe(&stream),
        Err(WireError::TrailingBytes),
        "the one-frame reader still refuses a tail -- THAT is what must not change"
    );

    let (first, consumed) = framing::take_frame(&stream).expect("a whole frame is there");
    assert_eq!(first, b"first");
    let (second, _) = framing::take_frame(&stream[consumed..]).expect("and so is the next");
    assert_eq!(second, b"second");
}

#[test]
fn a_stream_shorter_than_its_prefix_is_not_yet_a_frame() {
    assert_eq!(framing::take_frame(&[0, 0]), None);
    assert_eq!(framing::declared_len(&[0, 0]), None);
}

#[test]
fn a_stream_shorter_than_the_declared_body_is_not_yet_a_frame() {
    let whole = framing::frame(b"a body long enough to cut").expect("frame");
    let half = &whole[..whole.len() - 3];
    assert_eq!(framing::take_frame(half), None, "NOT YET is not an error");
    assert_eq!(
        framing::declared_len(half),
        Some(b"a body long enough to cut".len()),
        "the length is readable BEFORE the body arrives -- which is what a cap needs"
    );
}

#[test]
fn an_empty_body_is_a_whole_frame_and_not_an_absence() {
    let whole = framing::frame(b"").expect("frame");
    assert_eq!(framing::take_frame(&whole), Some((&[][..], framing::LENGTH_WIDTH)));
}
