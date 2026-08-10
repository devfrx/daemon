//! Counter-probes for the durable record (§4.9). The probe that must FIRE lives in
//! `tests/compile_fail/record_without_version.rs`; these are the other direction, the one
//! that is forgotten (§7.1.1, rule 3).
//!
//! ⛔ WHAT THESE PROBES HOLD IS THE MEANING, NOT THE WIRE NUMBERS, and it is written here
//! because it was MEASURED rather than assumed. Mutating a variant's `#[n(..)]` in
//! `src/record.rs` does NOT turn any of them red, and there are two reasons, both observed:
//! a mutation onto an index ALREADY IN USE never reaches the bench at all — `minicbor-derive`
//! refuses it at compile time with `error: duplicate index numbers` — and a mutation onto a
//! FREE index (`Trust::Untrusted` 1 → 7, `EffectClass::Unrepeatable` 2 → 5) leaves every test
//! here GREEN, because the derive renumbers ENCODING AND DECODING TOGETHER and a round trip
//! cannot see a change that is symmetric.
//!
//! ⚠️ SO THE NUMBERS ARE HELD ELSEWHERE — by the frozen bytes of `tests/frozen_bytes.rs`,
//! which is the level 2 check §4.9.2 names for exactly this. What dies here is the ASYMMETRIC
//! defect, and that was measured too: a `decode` that overwrites one field before returning
//! turns red the probe for that field AND NOTHING ELSE — six of seven stay green, including
//! the round trip above, which writes one label and one class and therefore cannot see it.
//! That is gotcha #30 in one line, and it is why the two probes below exist.

use kernel::record::{EffectClass, Record, RecordError, RecordKind, RecordV1, Trust};

#[test]
fn a_record_round_trips_through_its_own_encoding() {
    let original = Record::V1(RecordV1 {
        kind: RecordKind::Intent,
        effect: EffectClass::Idempotent,
        trust: Trust::Instruction,
        payload: b"why this step exists".to_vec(),
    });

    let bytes = original.encode().expect("encode");
    let read = Record::decode(&bytes).expect("decode");

    assert_eq!(read, original);
}

#[test]
fn the_version_is_in_the_bytes_and_not_only_in_the_type() {
    // `minicbor` encodes an enum as a two-element array: variant index, then value. So the
    // version travels WITH the record and a reader that has never seen the type can still
    // tell which version it is holding. Measured here rather than assumed: the first byte
    // of a two-element array is 0x82.
    let bytes = Record::V1(RecordV1 {
        kind: RecordKind::Intent,
        effect: EffectClass::Idempotent,
        trust: Trust::Instruction,
        payload: Vec::new(),
    })
    .encode()
    .expect("encode");

    assert_eq!(
        bytes[0], 0x82,
        "the record must encode as a 2-element array"
    );
    assert_eq!(bytes[1], 0x00, "the second item must be the version index");
}

#[test]
fn a_payload_is_a_byte_string_and_not_an_array_of_numbers() {
    // Gotcha #35: without the byte-string annotation `minicbor` encodes a `Vec<u8>` as an
    // ARRAY OF NUMBERS. It compiles, it round-trips, and it costs 1.91x — measured on 4096 B:
    // 7813 against 4101. The annotation is load-bearing, so a test holds it.
    let small = Record::V1(RecordV1 {
        kind: RecordKind::Intent,
        effect: EffectClass::Idempotent,
        trust: Trust::Instruction,
        payload: vec![0xAA; 64],
    })
    .encode()
    .expect("encode");

    // 64 bytes as a byte string cost 64 + 2 of header. As an array of numbers each value
    // above 0x17 costs TWO bytes, so the array form could not fit under 100.
    assert!(
        small.len() < 100,
        "payload encoded as an array of numbers, not a byte string: {} bytes",
        small.len()
    );
}

#[test]
fn the_two_record_kinds_are_distinguishable_in_the_bytes() {
    let intent = Record::V1(RecordV1 {
        kind: RecordKind::Intent,
        effect: EffectClass::Idempotent,
        trust: Trust::Instruction,
        payload: Vec::new(),
    })
    .encode()
    .expect("encode");

    let outcome = Record::V1(RecordV1 {
        kind: RecordKind::Outcome,
        effect: EffectClass::Idempotent,
        trust: Trust::Instruction,
        payload: Vec::new(),
    })
    .encode()
    .expect("encode");

    assert_ne!(intent, outcome);
}

#[test]
fn every_trust_label_survives_the_round_trip_and_the_two_differ_in_the_bytes() {
    // ⛔ Gotcha #30: a bench that looks only at `Ok`/`Err` does not see the WRONG ANSWER, and
    // in a durable archive the worst way to fail is not the error, it is the record that hands
    // back the wrong value. Every other probe in this file writes `Trust::Instruction`, so a
    // build that read EVERY label back as `Instruction` would be green on all of them — and I6
    // would fall in silence, because road A4 of `crate::boundary` rests on this one field:
    // bytes carry no labels, and this is the label.
    let encoded = |trust| {
        Record::V1(RecordV1 {
            kind: RecordKind::Intent,
            effect: EffectClass::Idempotent,
            trust,
            payload: Vec::new(),
        })
        .encode()
        .expect("encode")
    };

    // The VALUE READ BACK, not the outcome of reading: `is_ok()` here would prove nothing.
    for label in [Trust::Instruction, Trust::Untrusted] {
        let Record::V1(read) = Record::decode(&encoded(label)).expect("decode");
        assert_eq!(
            read.trust, label,
            "the trust label did not survive the round trip"
        );
    }

    // And the two must not collapse into the same bytes: an archive where they encode alike
    // cannot be told apart by any reader afterwards, however careful.
    assert_ne!(
        encoded(Trust::Instruction),
        encoded(Trust::Untrusted),
        "the two trust labels are indistinguishable in the bytes"
    );
}

#[test]
fn every_effect_class_survives_the_round_trip_and_the_three_differ_in_the_bytes() {
    // ⛔ Gotcha #30 again, and here it costs more than a label: this is the field
    // RECONCILIATION BRANCHES ON. A class that came back wrong would send a step down the
    // wrong road after a crash — re-run something unrepeatable, or suspend something
    // idempotent — and every probe above, which only ever writes `Idempotent`, would be green.
    let encoded = |effect| {
        Record::V1(RecordV1 {
            kind: RecordKind::Intent,
            effect,
            trust: Trust::Instruction,
            payload: Vec::new(),
        })
        .encode()
        .expect("encode")
    };

    for class in [
        EffectClass::Verifiable,
        EffectClass::Idempotent,
        EffectClass::Unrepeatable,
    ] {
        let Record::V1(read) = Record::decode(&encoded(class)).expect("decode");
        assert_eq!(
            read.effect, class,
            "the effect class did not survive the round trip"
        );
    }

    // ALL THREE PAIRS, not the two adjacent ones: two classes colliding while the third stays
    // apart is exactly the shape a partial comparison lets through.
    let verifiable = encoded(EffectClass::Verifiable);
    let idempotent = encoded(EffectClass::Idempotent);
    let unrepeatable = encoded(EffectClass::Unrepeatable);
    assert_ne!(
        verifiable, idempotent,
        "verifiable and idempotent encode alike"
    );
    assert_ne!(
        idempotent, unrepeatable,
        "idempotent and unrepeatable encode alike"
    );
    assert_ne!(
        verifiable, unrepeatable,
        "verifiable and unrepeatable encode alike"
    );
}

#[test]
fn bytes_that_are_not_a_record_decode_to_malformed() {
    // ⛔ Gotcha #30 from the other side: `RecordError::Malformed` is the only word this
    // vocabulary has, and no other probe here ever produces it — an error road nobody walks is
    // UNPROVEN SURFACE. A journal hands back whatever is on the disk, including a truncated
    // tail and a byte nobody wrote, and the answer must be a refusal rather than a value.
    assert_eq!(
        Record::decode(&[0xFF, 0xFF, 0xFF]),
        Err(RecordError::Malformed),
        "garbage decoded as a record"
    );
    // Nothing at all is the shape an interrupted write leaves behind, and it is a different
    // road inside the decoder: it runs out of input instead of meeting a wrong tag.
    assert_eq!(
        Record::decode(&[]),
        Err(RecordError::Malformed),
        "an empty slice decoded as a record"
    );

    let valid = Record::V1(RecordV1 {
        kind: RecordKind::Intent,
        effect: EffectClass::Idempotent,
        trust: Trust::Instruction,
        payload: b"why this step exists".to_vec(),
    })
    .encode()
    .expect("encode");

    // A record cut in half is the failure a real archive actually suffers, and it is neither
    // of the two above: the prefix is well formed as far as it goes.
    assert_eq!(
        Record::decode(&valid[..valid.len() / 2]),
        Err(RecordError::Malformed),
        "half a record decoded as a whole one"
    );
    // ⚠️ And the other direction, the one that is forgotten (§7.1.1, rule 3): a `decode` that
    // refused EVERYTHING would be green on all three assertions above.
    assert!(Record::decode(&valid).is_ok(), "a whole record was refused");
}
