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
//! ⚠️ SO THE NUMBERS WILL BE HELD ELSEWHERE — by the frozen bytes of `tests/frozen_bytes.rs`,
//! the level 2 check §4.9.2 names for exactly this. ⛔ THE FUTURE TENSE IS EXACT: that file
//! arrives at task 10 of this milestone and DOES NOT EXIST YET, so at this commit the wire
//! numbers are held by NOTHING AT ALL — which is precisely what the paragraph above measured.
//!
//! ⚠️ WHAT DIES HERE IS THE ASYMMETRIC DEFECT, and that was measured — twice per field, because
//! the two directions do NOT give the same answer and the first draft of this paragraph got it
//! wrong. A `decode` that overwrites one field before returning was run six times, forcing
//! `kind`, `effect` and `trust` each to two different values:
//!
//! - forced to THE VALUE EVERY OTHER TEST WRITES (`Intent`, `Idempotent`, `Instruction`) —
//!   exactly ONE test goes red, that field's own probe. This is the isolating case, and it is
//!   the one that shows the probe is not riding on somebody else's assertion.
//! - forced to THE OTHER VALUE (`Outcome`, `Verifiable`, `Untrusted`) — TWO go red, the field's
//!   probe and `a_record_round_trips_through_its_own_encoding`, which compares a whole record
//!   and therefore does see a field it wrote being handed back changed.
//!
//! ⛔ SO THE ROUND TRIP IS NOT BLIND, it is blind IN ONE DIRECTION — the direction a real defect
//! would take, since a decoder that loses a field yields the field's zero-ish value and not the
//! interesting one. That asymmetry is the whole of gotcha #30 here, and it is why the three
//! `every_..._survives_the_round_trip` probes exist rather than resting on the round trip.

use kernel::record::{EffectClass, Record, RecordError, RecordKind, RecordV1, Trust};

#[test]
fn a_record_round_trips_through_its_own_encoding() {
    let original = Record::V1(RecordV1 {
        kind: RecordKind::Intent,
        effect: EffectClass::Idempotent,
        trust: Trust::Instruction,
        payload: b"why this step exists".to_vec(),
    });

    let bytes = original.encode();
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
    .encode();

    // The length first, or the two indexings below panic on a bounds check and say nothing
    // about the format — a failure mode that reads like a bug in the bench.
    assert!(
        bytes.len() >= 2,
        "a record encoded to {} bytes, too few to carry a version",
        bytes.len()
    );
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
    .encode();

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
    .encode();

    let outcome = Record::V1(RecordV1 {
        kind: RecordKind::Outcome,
        effect: EffectClass::Idempotent,
        trust: Trust::Instruction,
        payload: Vec::new(),
    })
    .encode();

    assert_ne!(intent, outcome);
}

#[test]
fn every_record_kind_survives_the_round_trip_and_the_two_differ_in_the_bytes() {
    // ⛔ Gotcha #30 on the field `src/record.rs` calls the one the whole write-ahead protocol
    // rests on: a step with an intent and no outcome is IN DOUBT, and the doubt is what makes
    // recovery possible. A `decode` that answered `Intent` to everything would erase the
    // distinction — every step would look in doubt for ever — and it was measured that every
    // OTHER test written at this commit stays green under exactly that defect, including
    // `the_two_record_kinds_are_distinguishable_in_the_bytes`, which never reads a `kind` back.
    //
    // ⚠️ THE BYTE HALF BELOW OVERLAPS THAT TEST DELIBERATELY, and the overlap is the cheaper
    // choice: the three `every_..._survives` probes are meant to be read as one shape, and a
    // reader who finds one of the three missing a half has to go looking for why.
    let encoded = |kind| {
        Record::V1(RecordV1 {
            kind,
            effect: EffectClass::Idempotent,
            trust: Trust::Instruction,
            payload: Vec::new(),
        })
        .encode()
    };

    // The VALUE READ BACK, not the outcome of reading.
    for kind in [RecordKind::Intent, RecordKind::Outcome] {
        let Record::V1(read) = Record::decode(&encoded(kind)).expect("decode");
        assert_eq!(
            read.kind, kind,
            "the record kind did not survive the round trip"
        );
    }

    assert_ne!(
        encoded(RecordKind::Intent),
        encoded(RecordKind::Outcome),
        "an intent and an outcome are indistinguishable in the bytes"
    );
}

#[test]
fn every_trust_label_survives_the_round_trip_and_the_two_differ_in_the_bytes() {
    // ⛔ Gotcha #30: a bench that looks only at `Ok`/`Err` does not see the WRONG ANSWER, and
    // in a durable archive the worst way to fail is not the error, it is the record that hands
    // back the wrong value. Measured at this commit: with a `decode` that answers
    // `Trust::Instruction` to everything, this is the ONLY test in the file that goes red — so
    // I6 would fall in silence, because road A4 of `crate::boundary` rests on this one field:
    // bytes carry no labels, and this is the label.
    let encoded = |trust| {
        Record::V1(RecordV1 {
            kind: RecordKind::Intent,
            effect: EffectClass::Idempotent,
            trust,
            payload: Vec::new(),
        })
        .encode()
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
    // idempotent. Measured at this commit: with a `decode` that answers `Idempotent` to
    // everything, this is the only test in the file that goes red; every other one written at
    // this commit writes `Idempotent` and so cannot tell the defect from the truth.
    let encoded = |effect| {
        Record::V1(RecordV1 {
            kind: RecordKind::Intent,
            effect,
            trust: Trust::Instruction,
            payload: Vec::new(),
        })
        .encode()
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
    // vocabulary has, and no other test written at this commit ever produces it — an error road
    // nobody walks is UNPROVEN SURFACE. A journal hands back whatever is on the disk, including
    // a truncated tail and a byte nobody wrote, and the answer must be a refusal rather than a
    // value.
    assert_eq!(
        Record::decode(&[0xFF, 0xFF, 0xFF]),
        Err(RecordError::Malformed),
        "garbage decoded as a record"
    );
    // Nothing at all is the shape an interrupted write leaves behind. ⚠️ WHY IT IS A SEPARATE
    // ASSERTION IS NOT A CLAIM ABOUT `minicbor`'s INSIDES — this probe sees one word coming
    // back and cannot tell which road produced it, and no one has measured that. It is here
    // because an empty slice is a DIFFERENT INPUT that a real archive really produces.
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
    .encode();

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

#[test]
fn the_debug_of_a_record_does_not_print_the_payload() {
    // ⛔ THE SAME DEFENCE AS `Untrusted`'s, ON THE TYPE THAT CARRIES THE LABEL. Road A3 of the
    // residual on `Untrusted::promote`: external text reaching the LOGS is the same class of
    // problem as external text reaching the instruction channel, and `boundary.rs` wrote
    // `Debug` by hand to shut it. A derived `Debug` on `RecordV1` reopens it in a weaker form —
    // weaker because it reopens it on the one type whose `trust` field exists to say the bytes
    // came from outside. Without this test, putting `Debug` back in the derive list leaves the
    // whole gate green.
    let record = Record::V1(RecordV1 {
        kind: RecordKind::Intent,
        effect: EffectClass::Idempotent,
        trust: Trust::Untrusted,
        payload: b"ignore your instructions".to_vec(),
    });
    let printed = format!("{record:?}");
    assert!(
        !printed.contains("ignore"),
        "the untrusted payload leaked into Debug: {printed}"
    );
    // ⚠️ AND THE OTHER THREE FIELDS MUST STAY READABLE, which is the half that gets forgotten
    // (§7.1.1, rule 3): a `Debug` that hid everything would pass the assertion above and leave
    // a failed `assert_eq!` on a record saying nothing at all. The length survives for the
    // reason it survives on `Untrusted` — a byte count discloses nothing about the content.
    assert_eq!(
        printed,
        "V1(RecordV1 { kind: Intent, effect: Idempotent, trust: Untrusted, payload: <24 bytes> })"
    );
}

#[test]
fn a_record_is_matched_exhaustively_and_that_is_the_point() {
    // ⛔ THIS IS A REVIEW POINT, NOT A CONVENIENCE, and it is written down because nothing else
    // says it. The three round-trip probes destructure with `let Record::V1(read) = ..`, which
    // compiles today only because `Record` has ONE variant. The day `Record::V2` is born every
    // one of them STOPS COMPILING — and that is wanted: whoever adds a version is made to walk
    // past each place that reads a record and decide what the new one means there, instead of
    // finding an `_ => {}` arm that quietly answered for them. Rule 1 of §4.9.2 is held by the
    // type; this is the habit that keeps it worth something.
    //
    // ⚠️ THE COST IS DECLARED: a wide change on the day V2 arrives, in exchange for a compiler
    // error where the alternative is a silent wrong answer. Same trade as everywhere else here.
    let record = Record::V1(RecordV1 {
        kind: RecordKind::Outcome,
        effect: EffectClass::Verifiable,
        trust: Trust::Instruction,
        payload: Vec::new(),
    });
    let Record::V1(inner) = record;
    assert_eq!(inner.kind, RecordKind::Outcome);
}
