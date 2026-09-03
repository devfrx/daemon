//! Catalogue §7.4.1 block C, row `V5` — an effect WITHOUT its declared class is not
//! expressible. The class is the FIRST argument of the species constructor: leaving it out is
//! an ARITY error, because the field is not an `Option`, carries no `#[cbor(default)]`, and
//! `EffectClass` implements no `Default`.

fn main() {
    // Three arguments out of four. `effect` is the one left out, and leaving it out is the point.
    let _record = kernel::record::Record::V1(kernel::record::RecordV1::intent(
        kernel::record::Trust::Instruction,
        Vec::new(),
        "why this step exists",
    ));
}

// ⛔ THE MECHANISM IS THE ABSENCE OF A WAY TO OMIT THE CLASS, and this case is what keeps that
// absence from being a comment. Nothing else in the gate would see the class go: `gate-deps.sh`
// reads the dependency graph, `gate-attributes.sh` reads attributes, `gate-no-os.sh` builds for
// a target without an OS, `check-docs.sh` does not read code at all — and a `RecordV1` with one
// field fewer is valid Rust that `cargo build` compiles happily.
//
// ⛔ WHAT THIS CASE HOLDS IS THE SIGNATURE AND NOT THE FIELD, and the whole argument for that
// is written once next to its TWIN: `record_without_trust_label.rs` keeps the same contract for
// `trust`, on the same function, with the same `E0061` shape, and it carries the reasoning and
// the mutation that would disarm it. Read it there rather than twice.
//
// ⚠️ THE COUNTER-PROBE IS NOT HERE, AND IT ALREADY EXISTED: the catalogue row's "an effect with
// the class compiles" is
// `every_effect_class_survives_the_round_trip_and_the_three_differ_in_the_bytes` in
// `tests/record_shape.rs`, which builds a record for all THREE classes and compares their bytes.
//
// ⚠️ THE OTHER HALF OF V5 IS NOT WRITTEN HERE BECAUSE IT IS ALREADY HELD, and this case DECLARES
// it instead of repeating it. "An undeclared class counts as unrepeatable" lives in
// `crates/kernel/src/reconcile.rs`, which sends a record this build cannot decode to
// `Resolution::SuspendAndAsk`; the probes are
// `a_record_that_will_not_decode_is_treated_as_unrepeatable` and `the_class_decides_the_resolution`
// in `crates/kernel/tests/reconciliation.rs`. A second copy here would be a second place to keep
// in step, not a second proof.
//
// ⛔ Names `kernel::` and declares no attributes of its own — gotcha #39.
//
// ⛔ THE NOTE IS DOWN HERE ON PURPOSE: the oracle quotes LINE 8 of this file, so a paragraph
// added at the top would move the code and turn the case into a `mismatch`. Whoever writes here
// appends.
