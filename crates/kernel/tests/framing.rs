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
