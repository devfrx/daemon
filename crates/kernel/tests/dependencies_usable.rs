//! That a version exists does not mean it works — gotcha #22.
//! `cargo add bincode` resolves to 3.0.0, whose ENTIRE SOURCE is a `compile_error!`.
//! This test does not prove the logic: it proves that the shipped entries compile and
//! can be used.
//!
//! ⛔ THIS FILE DOES NOT DECLARE `#![no_std]`, AND THAT IS NOT AN OVERSIGHT.
//! An integration test is a crate of its own, and the `#[test]` harness needs `std` to
//! run: with `#![no_std]` here, the file does not link and fails for the wrong reason —
//! gotcha #9. The proof that the dependencies hold up **without an operating system** is
//! not this test: it is `scripts/gate-no-os.sh` (Task 4), which compiles `kernel` for
//! `x86_64-unknown-none`. That is the mechanism; this is only the round-trip.

#[test]
fn bincode_2_round_trips_in_no_std() {
    let expected: u32 = 4096;
    let bytes: Vec<u8> =
        bincode::encode_to_vec(expected, bincode::config::standard()).expect("encode");
    assert!(!bytes.is_empty());
    let (read, consumed): (u32, usize) =
        bincode::decode_from_slice(&bytes, bincode::config::standard()).expect("decode");
    assert_eq!(read, expected);
    // The consumed bytes match the length: it is the rule that gotcha #34 imposes on the
    // framed channel, and it is worth exercising it from the start.
    assert_eq!(consumed, bytes.len());
}

#[test]
fn minicbor_round_trips_in_no_std() {
    let expected: u32 = 4096;
    let mut bytes: Vec<u8> = Vec::new();
    minicbor::encode(expected, &mut bytes).expect("encode");
    let read: u32 = minicbor::decode(&bytes).expect("decode");
    assert_eq!(read, expected);
}
