//! Catalogue §7.4.1 block C, row `Q9 · I6 · V20 · §4.9` — a durable record WITHOUT its trust
//! label is not expressible. The field is mandatory: it is not an `Option`, it carries no
//! `#[cbor(default)]`, and `Trust` implements no `Default`.

fn main() {
    // Three arguments out of four. `trust` is the one left out, and leaving it out is the point.
    let _record = kernel::record::Record::V1(kernel::record::RecordV1::intent(
        kernel::record::EffectClass::Idempotent,
        Vec::new(),
        "why this step exists",
    ));
}

// ⛔ THE MECHANISM IS THE ABSENCE OF A WAY TO OMIT THE FIELD, and this case is what keeps that
// absence from being a comment. Nothing else in the gate would see the field go: `gate-deps.sh`
// reads the dependency graph, `gate-attributes.sh` reads attributes, `gate-no-os.sh` builds for
// a target without an OS, `check-docs.sh` does not read code at all — and a `RecordV1` with one
// field fewer is valid Rust that `cargo build` compiles happily.
//
// ⛔ IT FIRES AS `error`, NOT AS `mismatch`, and it was measured rather than hoped: with the
// `trust` field deleted from `RecordV1` in `src/record.rs`, trybuild prints `test ..
// record_without_trust_label.rs ... error` and `Expected test case to fail to compile, but it
// succeeded.` That is the good answer to gotcha #42 — `TRYBUILD=overwrite` rewrites `.stderr`
// files and nothing else, so a bulk regeneration CANNOT silence a case that fires by compiling.
//
// ⚠️ REMOVING THE FIELD IS THE ONLY MUTATION THAT DISARMS THIS ONE. A `Default` on a field's
// TYPE does not make that field omissible in a struct literal, so `impl Default for Trust`
// leaves this case green — which is why the row's other half needs its own case.
// `trust_has_no_default.rs` is that case, and it carries the whole argument, the two-line
// recipe and what neither case covers. Read it there rather than twice.
//
// ⚠️ THE COUNTER-PROBE IS NOT HERE, AND IT ALREADY EXISTED: the catalogue row's "a record that
// declares its own label compiles, in both values" is
// `every_trust_label_survives_the_round_trip_and_the_two_differ_in_the_bytes` in
// `tests/record_shape.rs`, which writes both values and compares their bytes.
//
// ⛔ Names `kernel::` and declares no attributes of its own — gotcha #39.
//
// ⛔ THE NOTE IS DOWN HERE ON PURPOSE: the oracle quotes LINE 7 of this file, so a paragraph
// added at the top would move the code and break it. Whoever writes here appends.
//
// ⚠️ `reason` WAS ADDED TO THE LITERAL ON 2026-08-10 AND THE CASE DID NOT MOVE, which is the
// point of touching it. Index 4 is mandatory, so without that line this case would fail with
// `missing fields `reason` and `trust`` — still an error, still `ok`, and no longer a case
// about the trust label at all. A negative case that fires for a second reason is a case that
// stops proving the first.

// ⛔ DATED RECALL, 2026-09-01 — AUD-050. This case used to leave `trust` out of a STRUCT
// LITERAL and expect `error[E0063]: missing field`. Since AUD-050 was shut, `RecordV1` has no
// public field, so the literal is refused for a SECOND REASON — `cannot construct with struct
// literal syntax due to private fields` — and the paragraph above says what that means:
// "a negative case that fires for a second reason is a case that stops proving the first".
// ✅ SO THE CASE MOVED TO THE ROAD THAT EXISTS: the species constructor, whose signature takes
// the label. Leaving it out is now an ARITY error, and the property is unchanged — a record
// cannot be built without saying what its TRUST LABEL is. The `E0061` shape is already used by the
// gate for the same purpose in `reading_without_a_receipt.rs`.
//
// ⚠️ AND THE MUTATION THAT DISARMS IT MOVED WITH IT: it is no longer "delete the field" but
// "give `intent` a default for `trust`", which Rust cannot express — so what this case now
// holds is the SIGNATURE. Deleting the field from `RecordV1` breaks the constructor's body and
// the whole crate stops compiling, which is a louder red than this case.
