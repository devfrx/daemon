//! The frozen bytes of the durable record — level 2 check of §4.9.4, catalogue row
//! `Q14 · §4.9`.
//!
//! ⛔ FROM THIS FILE ONWARDS THE FORMAT IS FROZEN, and that is the sentence whoever comes
//! next is meant to hit first. Three things become forbidden that were free until the commit
//! that created these bytes:
//!
//! 1. A FIELD ADDED TO `RecordV1` MUST BE `Option<..>` AT A NEW INDEX, and is written
//!    `#[cbor(default)]` by convention. `reason` came in at index 4 as a mandatory field
//!    precisely because no archive existed yet; that exemption is spent.
//!    ⚠️ WHICH HALF CARRIES IT WAS MEASURED ON 2026-08-31, AND IT IS THE `Option`: with the
//!    annotation removed from `detail` the whole workspace stays green — 41 targets, 298
//!    passed — INCLUDING the backward direction, because the 21-byte files still decode to
//!    `detail: None`. `minicbor` already treats a missing `Option` field as `None`, so on an
//!    `Option` the annotation is belt AND braces rather than the belt. ⛔ SAID OUT LOUD BECAUSE
//!    THIS LINE CLAIMED IT WAS LOAD-BEARING AND NOTHING HELD IT: a field arriving without it
//!    would turn nothing red, so a reader must not mistake the convention for the defence.
//!    Errata `E72`.
//! 2. THE MEANING OF AN EXISTING INDEX NEVER CHANGES. Index 3 changed role once, from "the
//!    reason" to "the untrusted content", and it was free. Rule 4 of §4.9.2 now forbids it,
//!    and the reuse was MEASURED: it decodes to the WRONG SILENCE, not to an error.
//! 3. A VARIANT ADDED TO ONE OF THE THREE `index_only` ENUMS makes every record carrying it
//!    UNDECODABLE to an older build. The direction is safe — `Malformed` reconciles as
//!    `SuspendAndAsk`, so an old build stops instead of guessing — but it was free only
//!    while the archive did not exist.
//!
//! ⛔ THESE BYTES ARE NOT REGENERATED. If they change it is not an update, it is a CHANGE OF
//! FORMAT, and a NEW VERSION must be opened. Regenerating them in bulk erases the oracle —
//! gotcha #25, moved here from the `.stderr` files of `trybuild`.
//!
//! ⛔ AND THERE IS DELIBERATELY NO WAY TO REGENERATE THEM FROM HERE. No flag, no environment
//! variable, no `--bless`. That is exactly how `trybuild` gets disarmed: an oracle with a
//! regeneration path is an oracle one keystroke away from being a tautology. The three
//! `.cbor` files were TYPED BY HAND from the hexadecimal output of a throwaway probe, and the
//! probe was deleted in the same commit.
//!
//! ⚠️ WHAT THIS CATCHES THAT THE COMPILER CANNOT: an index REUSED or RENUMBERED. The compiler
//! sees types, not the numbers written inside an annotation — rule 4 of §4.9.2 is a
//! DISCIPLINE, and this file is what holds it. `tests/record_shape.rs` says the same thing
//! from the other side and MEASURED it: a variant moved onto a free index leaves every probe
//! there green, because the derive renumbers encoding and decoding together and no round trip
//! can see a symmetric change.
//!
//! ⛔ MORE THAN ONE RECORD, AND HOW MANY IS FORCED BY WHAT THERE IS TO PIN — NOT CHOSEN. A
//! record carries exactly ONE variant of each `index_only` enum, so a single frozen record
//! pins one index per enum and leaves every other variant held by nothing at all, which is the
//! state `record_shape.rs` describes and which this file exists to end. Two rules follow, and
//! both are rules rather than counts: the set must cover EVERY variant of every one of those
//! enums, and it therefore cannot be smaller than the WIDEST of them. The effects and the
//! kinds are laid out as a Latin square, so no pair of fields can be swapped without moving at
//! least one file.
//!
//! ⛔ RECALL OF 2026-08-31 — THIS PARAGRAPH CARRIED THREE NUMERALS AND ALL THREE HAD GONE
//! FALSE: "THREE RECORDS", "EIGHT variants — `RecordKind` three", and "Three records are the
//! FEWEST that cover all eight". `Verdict` made the kinds four and the total nine, and the
//! fourth record arrived in the same commit that left this head untouched — the probes below
//! were renamed for exactly this reason on that day, and the paragraph justifying them was
//! not. ⚠️ TAKEN OUT AND NOT REALIGNED, because the population grows with the format: how many
//! records there are is `the_frozen_records()` below, whose return type carries the count the
//! compiler checks, and how many variants there are is the enums themselves. Errata `E70`.
//!
//! ⚠️ AND THE RECORDS DIFFER IN AS LITTLE AS THE FORMAT ALLOWS, deliberately: same payload,
//! same reason, same framing. Two consequences that are both wanted — the ones carrying NO
//! `detail` differ from each other only inside bytes 4, 5 and 6, so the map's claim about
//! where the enums sit is legible by inspection; and the same six characters travel as a CBOR
//! BYTE STRING at index 3 and as CBOR TEXT at index 4, one nibble apart in the frozen bytes,
//! so the asymmetry `reason` exists for is visible in the artefact instead of only in the
//! source. ⛔ A RECORD THAT CARRIES A `detail` IS LONGER AND ALSO MOVES BYTE 3, the field-array
//! header — measured, `85` against `86` — so the "bytes 4, 5 and 6" reading holds WITHIN the
//! detail-less ones and nowhere else. It said "any pair of the three files" until 2026-08-31,
//! which was true when only three existed and false the moment the fourth arrived.
//!
//! ⚠️ THE FRAMING IS FOUR BYTES AND NOT THREE, and it is written here because the obvious
//! reading gets it wrong: `82 00` is the version enum and its variant index, and then comes
//! `81` — the ONE-ELEMENT ARRAY of the variant's body — before `85`, the five fields. So the
//! fields start at byte 4 and not at byte 3, and a record is 21 bytes and not 20. Measured off
//! the real output rather than deduced; `record_shape.rs` says the same four bytes from the
//! other side, in `82 00 81 85 00 01 00 40 60`.

use kernel::record::{Detail, EffectClass, Record, RecordKind, RecordV1, Trust, VerdictDetail};

/// The bytes on disk. ⛔ `include_bytes!` AND NOT A READ AT RUN TIME: the artefact enters the
/// test binary, so a `.cbor` deleted or renamed is a COMPILE error and not a check that
/// quietly stops checking — gotcha #26, which this repository has already met three times.
const INTENT_BYTES: &[u8] = include_bytes!("frozen/record_v1_intent.cbor");
const OUTCOME_BYTES: &[u8] = include_bytes!("frozen/record_v1_outcome.cbor");
const NOTE_BYTES: &[u8] = include_bytes!("frozen/record_v1_note.cbor");
const VERDICT_BYTES: &[u8] = include_bytes!("frozen/record_v1_verdict.cbor");

/// The map, included for the same reason and READ BACK by
/// `the_map_lists_the_bytes_that_are_really_frozen` instead of being believed.
const MAP: &str = include_str!("frozen/record_v1.map");

/// ⛔ THE SAME SIX CHARACTERS IN BOTH, AND IT IS THE POINT RATHER THAN LAZINESS. `payload` is
/// a CBOR byte string and `reason` is CBOR text, so every frozen file carries
/// `46 66 72 6f 7a 65 6e` immediately followed by `66 66 72 6f 7a 65 6e`: the same content
/// under two major types, differing in one nibble of one header byte. An encoder that swapped
/// the two indices would be caught HERE even though a round trip through this type cannot see
/// it — which is exactly what `the_reason_survives_the_round_trip_and_travels_beside_the_payload`
/// says in `record_shape.rs`.
const FROZEN_PAYLOAD: &[u8] = b"frozen";
const FROZEN_REASON: &str = "frozen";

/// What the oracle says when it fires. ⛔ A CONSTANT because three assertions carry it, and a
/// message written three times is a message that ages twice — gotcha #31.
const FORMAT_CHANGED: &str = "\n⛔ THE DURABLE FORMAT CHANGED.\n\
     This is not a test to update: it is the oracle of §4.9.4. If a field was added it must \
     be OPTIONAL, WITH `#[cbor(default)]` AND A NEW INDEX, and these bytes must be UNCHANGED. \
     If they are not, an index was reused or renumbered — rule 4 of §4.9.2 — and what is \
     needed is A NEW VERSION of the record, not a new oracle.\n\
     There is deliberately no way to regenerate these files: read the head of this one.\n\
     The map of `offset -> bytes -> field` is in tests/frozen/record_v1.map.\n";

/// Builds a record with the frozen payload and the frozen reason. ⛔ ONE CONSTRUCTOR for ALL
/// the frozen records AND for the mutants of `every_field_sits_at_the_offset_the_map_gives_it`:
/// a second constructor would be a second place to keep aligned, and the first one to stop
/// being updated lies in silence (§7.4.4).
fn record(kind: RecordKind, effect: EffectClass, trust: Trust, detail: Option<Detail>) -> Record {
    Record::V1(RecordV1 {
        kind,
        effect,
        trust,
        payload: FROZEN_PAYLOAD.to_vec(),
        reason: String::from(FROZEN_REASON),
        detail,
    })
}

/// The frozen records, each beside the name of the file that holds its bytes. ⚠️ HOW MANY
/// THERE ARE IS THE RETURN TYPE, one line below, which the compiler checks — it is not written
/// in this sentence, which said "The three frozen records" while the signature already said
/// four (errata `E70`).
///
/// ⛔ CHANGING ANY OF THESE VALUES CHANGES THE BYTES: this function and the `.cbor` files are
/// ONE artefact, and its pieces are only ever read together.
///
/// ⚠️ THE ORDER IS THE MAP'S ORDER, and `the_map_lists_the_bytes_that_are_really_frozen`
/// compares the names pairwise, so the two cannot drift apart in silence.
fn the_frozen_records() -> [(&'static str, &'static [u8], Record); 4] {
    [
        (
            "record_v1_intent.cbor",
            INTENT_BYTES,
            record(
                RecordKind::Intent,
                EffectClass::Idempotent,
                Trust::Untrusted,
                None,
            ),
        ),
        (
            "record_v1_outcome.cbor",
            OUTCOME_BYTES,
            record(
                RecordKind::Outcome,
                EffectClass::Unrepeatable,
                Trust::Instruction,
                None,
            ),
        ),
        (
            "record_v1_note.cbor",
            NOTE_BYTES,
            record(
                RecordKind::Note,
                EffectClass::Verifiable,
                Trust::Untrusted,
                None,
            ),
        ),
        // ⛔ THE FOURTH CARRIES BOTH THINGS AT ONCE (D21), and `None` here would pin NOTHING of
        // index 5: a trailing `None` is not written, measured. So this record is the only place
        // that holds the new variant index AND the new field's position on the wire.
        //
        // ⚠️ `passed: false` AND NOT `true`: `false` encodes `f4` and `true` `f5`, so the byte
        // exists either way — but a NEGATIVE verdict is the one the ring feeds back, which is
        // the case the species exists for. And `spent_millis: 7` is not zero, because `00` is
        // also half the variant indices in this table and a byte that resembles too many things
        // makes the map harder to read than it needs to be.
        (
            "record_v1_verdict.cbor",
            VERDICT_BYTES,
            record(
                RecordKind::Verdict,
                EffectClass::Verifiable,
                Trust::Untrusted,
                Some(Detail::Verdict(VerdictDetail {
                    passed: false,
                    spent_millis: 7,
                })),
            ),
        ),
    ]
}

#[test]
fn every_frozen_record_still_encodes_to_its_frozen_bytes() {
    for (name, frozen, record) in the_frozen_records() {
        assert_eq!(
            record.encode().as_slice(),
            frozen,
            "{FORMAT_CHANGED}The record whose bytes moved is `{name}`.\n"
        );
    }
}

#[test]
fn the_frozen_bytes_still_decode_to_their_records() {
    // The other direction, and it is not the same one: the first test catches a format that
    // moved, this one catches A BUILD THAT LOST THE ABILITY TO READ ITS OWN ARCHIVE. A
    // decoder narrowed to refuse what it used to accept leaves the encoder untouched and the
    // test above green.
    for (name, frozen, record) in the_frozen_records() {
        let read = Record::decode(frozen).unwrap_or_else(|e| {
            panic!("{FORMAT_CHANGED}The frozen bytes of `{name}` no longer decode: {e:?}\n")
        });
        assert_eq!(
            read, record,
            "{FORMAT_CHANGED}The frozen bytes of `{name}` decoded to a DIFFERENT record.\n"
        );
    }
}

#[test]
fn the_frozen_records_are_distinguishable_in_the_bytes() {
    // ⛔ TWO FROZEN RECORDS THAT ENCODED ALIKE WOULD PIN ONE THING BETWEEN THEM, and the file
    // count would flatter the coverage: three artefacts, two indices held. The pairs are all
    // compared, not the adjacent ones — two colliding while the third stays apart is exactly
    // the shape a partial comparison lets through, and it is the shape
    // `the_record_kinds_are_distinguishable_in_the_bytes` was widened for.
    //
    // ⚠️ THE NAME SAID `the_three_…` UNTIL THE FOURTH FROZEN RECORD ARRIVED, AND IT IS RENAMED
    // RATHER THAN LEFT: a name that counts its own subjects is a count like any other. The
    // pairwise comparison itself never mentioned three.
    let frozen = the_frozen_records();
    for (i, (left_name, left, _)) in frozen.iter().enumerate() {
        for (right_name, right, _) in frozen.iter().skip(i + 1) {
            assert_ne!(
                left, right,
                "`{left_name}` and `{right_name}` encode alike: between them they pin one \
                 variant and not two"
            );

            // ⚠️ AND THE DIFFERENCE IS WHERE THE MAP SAYS IT IS — FOR THE PAIRS THAT CARRY THE
            // SAME FIELDS. The records of one arity were chosen to differ in the three enum
            // fields and in NOTHING else, so a byte moving outside 4..7 means either the
            // constructors above drifted apart or the framing changed.
            //
            // ⛔ A PAIR OF DIFFERENT ARITY IS SKIPPED HERE, AND IT IS NOT A HOLE — it is the
            // D21 showing through: the fourth record exists precisely BECAUSE it carries a
            // field the other three do not, so it differs at the array header (`85` -> `86`)
            // and in the whole tail, by construction. Asserting equal length across arities
            // would assert that the D21 did not happen. ⚠️ WHAT WOULD BE A HOLE is dropping
            // the pair entirely: the `assert_ne!` above runs on EVERY pair, arity included, so
            // "two frozen records that encode alike" is still caught between any two of them.
            // What is skipped is only the WHERE of the difference, which has no meaning across
            // two different shapes.
            if left.len() != right.len() {
                continue;
            }
            let moved: Vec<usize> = (0..left.len()).filter(|&i| left[i] != right[i]).collect();
            assert!(
                moved.iter().all(|&i| (4..7).contains(&i)),
                "`{left_name}` and `{right_name}` differ at bytes {moved:?}, and frozen records \
                 of the same arity are meant to differ only at 4, 5 and 6"
            );
        }
    }
}

#[test]
fn every_variant_of_the_wire_enums_is_pinned_by_a_frozen_record() {
    // ⛔ THIS IS WHY THERE IS MORE THAN ONE FILE. Pinning `Intent`, `Idempotent` and
    // `Untrusted` alone would hold ONE index per enum and leave EVERY OTHER variant of the
    // three held by nothing — and `record_shape.rs` MEASURED that every other probe survives a
    // symmetric renumbering, so "nothing" is exact. ⚠️ THE LEFTOVERS ARE NOT LISTED BY NAME
    // HERE, and that is the fix rather than the shortcut: this sentence used to enumerate them
    // and to say "THREE indices out of eight", and the list went one short and the total one
    // low the day `Verdict` arrived. What enumerates them is the assertion below, which reads
    // the enums instead of quoting them (errata `E70`).
    //
    // ⚠️ RECALL OF 2026-08-31 — THE NAME SAID `..._of_the_three_enums_...` AND IT COUNTED ITS
    // OWN SUBJECTS. Renamed and not realigned to four, on the precedent this same file set for
    // `the_three_frozen_records_are_distinguishable_in_the_bytes`: a name that carries a count
    // is a count like any other, and this population grows with the format.
    let frozen = the_frozen_records();
    let kinds: Vec<RecordKind> = frozen.iter().map(|(_, _, Record::V1(r))| r.kind).collect();
    let effects: Vec<EffectClass> = frozen
        .iter()
        .map(|(_, _, Record::V1(r))| r.effect)
        .collect();
    let trusts: Vec<Trust> = frozen.iter().map(|(_, _, Record::V1(r))| r.trust).collect();

    for kind in [
        RecordKind::Intent,
        RecordKind::Outcome,
        RecordKind::Note,
        RecordKind::Verdict,
    ] {
        // ⛔ THE EXHAUSTIVE `match` IS THE HALF THAT DOES NOT AGE: a variant added to
        // `RecordKind` STOPS THIS FILE COMPILING, and the author lands on the list beside it.
        // ⚠️ DECLARED LIMIT, because half of it is held by a reader and not by the compiler:
        // extending the arm without extending the array above still compiles. What makes that
        // acceptable is that a new variant of these wire enums is A FORMAT CHANGE by the head
        // of this file, so it can never be a quiet addition.
        match kind {
            RecordKind::Intent | RecordKind::Outcome | RecordKind::Note | RecordKind::Verdict => {}
        }
        assert!(
            kinds.contains(&kind),
            "no frozen record carries {kind:?}: its wire index is held by nothing"
        );
    }

    for effect in [
        EffectClass::Verifiable,
        EffectClass::Idempotent,
        EffectClass::Unrepeatable,
    ] {
        match effect {
            EffectClass::Verifiable | EffectClass::Idempotent | EffectClass::Unrepeatable => {}
        }
        assert!(
            effects.contains(&effect),
            "no frozen record carries {effect:?}: its wire index is held by nothing"
        );
    }

    for trust in [Trust::Instruction, Trust::Untrusted] {
        match trust {
            Trust::Instruction | Trust::Untrusted => {}
        }
        assert!(
            trusts.contains(&trust),
            "no frozen record carries {trust:?}: its wire index is held by nothing"
        );
    }

    // ⛔ AND `Detail` IS THE FOURTH WIRE ENUM, WHICH THIS TEST DID NOT HOLD UNTIL 2026-08-31.
    // Its indices never retire either (rule 4 of §4.9.2), so the guarantee written above was
    // true of three enums while the head of this file claims it for the format. ⚠️ MEASURED IN
    // BOTH DIRECTIONS on 2026-08-31, reviewing the task that added the species: a second variant
    // on `Detail` left the WHOLE WORKSPACE green — 41 targets, 297 passed, 0 failed, 2 ignored,
    // identical to the baseline figure for figure — while the same addition to `RecordKind` is
    // `error[E0004]: non-exhaustive patterns`.
    //
    // ⚠️ THE SHAPE DIFFERS FROM THE THREE ABOVE BECAUSE THE ENUM CARRIES DATA: a variant cannot
    // go in an array literal without inventing a value, and comparing values would assert the
    // frozen CONTENT instead of the species. So the exhaustive `match` runs over the species the
    // frozen records really carry, and the assertion above it is what stops it from being a
    // `match` over an empty list. ⛔ THE DECLARED LIMIT IS THE SAME ONE ITS THREE SIBLINGS
    // CARRY, deliberately: extending the arm without freezing a record still compiles. Making
    // this one stronger alone would be a second convention for one property (§7.4.4).
    let details: Vec<&Detail> = frozen
        .iter()
        .filter_map(|(_, _, Record::V1(r))| r.detail.as_ref())
        .collect();
    assert!(
        !details.is_empty(),
        "no frozen record carries a `detail`: index 5 and its species are held by nothing"
    );
    for detail in details {
        match detail {
            Detail::Verdict(_) => {}
        }
    }
}

/// Asserts that changing one field moved the encoding INSIDE the offsets the map gives that
/// field, and nowhere else. Two directions in one helper (§7.1.1, rule 3): that something
/// moved at all, and that nothing moved outside.
fn only_inside(field: &str, base: &[u8], changed: &[u8], from: usize, to: usize) {
    assert_eq!(
        base.len(),
        changed.len(),
        "the two records must differ in a field and not in a length, or this proves nothing"
    );
    let moved: Vec<usize> = (0..base.len()).filter(|&i| base[i] != changed[i]).collect();
    assert!(
        !moved.is_empty(),
        "changing `{field}` changed no byte at all: the field is not in the archive"
    );
    assert!(
        moved.iter().all(|&i| (from..to).contains(&i)),
        "`{field}` moved the bytes {moved:?}, and the map puts it at {from}..{to}"
    );
}

#[test]
fn every_field_sits_at_the_offset_the_map_gives_it() {
    // ⛔ THIS IS WHAT MAKES THE MAP MORE THAN PROSE. A byte-for-byte comparison holds the
    // bytes and says nothing about WHICH BYTE BELONGS TO WHICH FIELD — and that second claim
    // is the whole of what a reader takes from the map. It is held here by changing ONE field
    // of a frozen record and demanding that the encoding move at the declared offsets and
    // NOWHERE ELSE.
    //
    // ⚠️ THE MUTANTS KEEP THE LENGTH, deliberately: an upper-case payload of the same six
    // bytes and a reason of the same six characters. A shorter value would move every byte
    // after it and the assertion would degrade into "something changed".
    let base = record(
        RecordKind::Intent,
        EffectClass::Idempotent,
        Trust::Untrusted,
        None,
    )
    .encode();

    let moved_kind = record(
        RecordKind::Outcome,
        EffectClass::Idempotent,
        Trust::Untrusted,
        None,
    )
    .encode();
    only_inside("kind", &base, &moved_kind, 4, 5);

    let moved_effect = record(
        RecordKind::Intent,
        EffectClass::Unrepeatable,
        Trust::Untrusted,
        None,
    )
    .encode();
    only_inside("effect", &base, &moved_effect, 5, 6);

    let moved_trust = record(
        RecordKind::Intent,
        EffectClass::Idempotent,
        Trust::Instruction,
        None,
    )
    .encode();
    only_inside("trust", &base, &moved_trust, 6, 7);

    let moved_payload = Record::V1(RecordV1 {
        kind: RecordKind::Intent,
        effect: EffectClass::Idempotent,
        trust: Trust::Untrusted,
        payload: b"FROZEN".to_vec(),
        reason: String::from(FROZEN_REASON),
        detail: None,
    })
    .encode();
    only_inside("payload", &base, &moved_payload, 7, 14);

    let moved_reason = Record::V1(RecordV1 {
        kind: RecordKind::Intent,
        effect: EffectClass::Idempotent,
        trust: Trust::Untrusted,
        payload: FROZEN_PAYLOAD.to_vec(),
        reason: String::from("FROZEN"),
        detail: None,
    })
    .encode();
    only_inside("reason", &base, &moved_reason, 14, 21);
}

/// Reads the map back — `offset | hex bytes | prose`, one section per `.cbor` file, a section
/// opening on a line that is only a file name.
///
/// ⚠️ A PARSER INSIDE A TEST IS A COST AND IT IS PAID ON PURPOSE: the alternative is a map
/// that nothing reads, which is the second place to keep aligned that §7.4.4 refuses. It is
/// deliberately UNFORGIVING — a row it cannot read is a panic and not a row skipped, because a
/// parser that skips is a check that empties itself in silence (gotcha #26).
fn the_map_read_back() -> Vec<(String, Vec<u8>)> {
    let mut sections: Vec<(String, Vec<u8>)> = Vec::new();

    for (n, raw) in MAP.lines().enumerate() {
        let line = raw.trim();
        let at = n + 1;
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((offset, rest)) = line.split_once('|') else {
            assert!(
                line.ends_with(".cbor"),
                "map line {at}: a line without columns opens a section and must be a file \
                 name, and this one is `{line}`"
            );
            sections.push((String::from(line), Vec::new()));
            continue;
        };

        let (hex, _prose) = rest.split_once('|').unwrap_or_else(|| {
            panic!("map line {at}: a row has three columns — `offset | hex | prose` — not two")
        });
        let section = sections
            .last_mut()
            .unwrap_or_else(|| panic!("map line {at}: a row before any file name was named"));

        let declared: usize = offset.trim().parse().unwrap_or_else(|e| {
            panic!("map line {at}: the offset column is not a number ({e}): `{line}`")
        });
        assert_eq!(
            declared,
            section.1.len(),
            "map line {at}: the row declares offset {declared}, and the rows before it end at \
             {}",
            section.1.len()
        );

        for token in hex.split_whitespace() {
            // ⚠️ EXACTLY TWO DIGITS, so `6` cannot pass for `06`: a map read one way and
            // written another is the drift this parser exists to make impossible.
            assert_eq!(
                token.len(),
                2,
                "map line {at}: `{token}` is not a two-digit hexadecimal byte"
            );
            let byte = u8::from_str_radix(token, 16).unwrap_or_else(|e| {
                panic!("map line {at}: `{token}` is not a hexadecimal byte ({e})")
            });
            section.1.push(byte);
        }
    }

    sections
}

#[test]
fn the_map_lists_the_bytes_that_are_really_frozen() {
    // ⛔ THE MAP IS CHECKED AND NOT TRUSTED. §7.4.4 refuses a second place to keep aligned for
    // one property, because the first one to stop being updated then lies with authority; a
    // map that no check reads is precisely that. The OFFSET and HEX columns of every row are
    // read back here and must rebuild the `.cbor` file byte for byte.
    //
    // ⛔ AND THIS IS WHAT KILLS THE PLACEHOLDER — gotcha #43, which this repository has paid
    // for once. A `<fill in>` left in the hex column does not parse, and a byte typed wrong
    // does not match: a map that lies cannot be committed green.
    //
    // ⚠️ THE PROSE COLUMN IS NOT CHECKED, and the map says so in its own header. Declaring it
    // is what keeps the file from being read as verified throughout — the middle ground the
    // check was written to leave.
    let sections = the_map_read_back();

    // Non-vacuity guard, gotcha #26: a map emptied of its rows would rebuild nothing and every
    // comparison below would be between two empty things.
    assert_eq!(
        sections.len(),
        the_frozen_records().len(),
        "the map describes {} records and there are {} frozen files",
        sections.len(),
        the_frozen_records().len()
    );

    for ((name, bytes), (file, frozen, _)) in sections.iter().zip(the_frozen_records()) {
        assert_eq!(
            name.as_str(),
            file,
            "the map's sections are in a different order from the frozen records"
        );
        assert!(
            !bytes.is_empty(),
            "the map's section for `{file}` has no rows at all"
        );
        assert_eq!(
            bytes.as_slice(),
            frozen,
            "{FORMAT_CHANGED}The map of `{file}` and its bytes disagree.\n"
        );
    }
}
