//! Catalogue §7.4.1 block C, row `Q9 · I6 · V20 · §4.9` — a durable record WITHOUT its trust
//! label is not expressible. The field is mandatory: it is not an `Option`, it carries no
//! `#[cbor(default)]`, and `Trust` implements no `Default`.

fn main() {
    // Three fields out of four. `trust` is the one left out, and leaving it out is the point.
    let _record = kernel::record::Record::V1(kernel::record::RecordV1 {
        kind: kernel::record::RecordKind::Intent,
        effect: kernel::record::EffectClass::Idempotent,
        payload: Vec::new(),
    });
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
// No second case of a different shape is owed here.
//
// ⛔ AND WHAT DOES *NOT* DISARM IT WAS MEASURED TOO, because the obvious guess is wrong and a
// later reader would waste the same afternoon. `impl Default for Trust` leaves this case GREEN,
// and so does adding `#[cbor(default)]` to the field on top of it. A `Default` on a FIELD'S TYPE
// does not make that field omissible in a STRUCT LITERAL — only `..Default::default()` does, and
// that lives in the caller, not in `src/record.rs`. The only mutation that disarms this guard is
// REMOVING THE FIELD, which is exactly the defect the rule is about.
//
// ⚠️ SO THIS CASE HOLDS THE HALF "THE FIELD EXISTS", NOT THE HALF "IT HAS NO DEFAULT", and the
// difference is written here rather than left to be assumed from the catalogue row. With
// `#[cbor(default)]` present the WHOLE `kernel` suite stays green — ten benches, no red. What it
// does to the bytes was measured too, and the answer is nothing: the array is POSITIONAL and
// `trust` sits at index 2, so a short array slides the payload into the label's slot and
// decoding still comes out `Err(Malformed)`. The decode half arrives with the frozen bytes of
// task 10, which is a level 2 check.
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
