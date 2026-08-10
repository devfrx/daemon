//! Catalogue §7.4.1 block C, row `Q9 · I6 · V20 · §4.9`, SECOND HALF — the trust label has no
//! default. `record_without_trust_label.rs` holds "the field exists"; this one holds "and
//! nothing defaults it", which is the half that decides what a SHORT ARCHIVE decodes to.

fn main() {
    let _label = <kernel::record::Trust as Default>::default();
}

// ⛔ THIS IS THE SECOND CASE OF ONE CATALOGUE ROW, NOT A ROW OF ITS OWN, and it is the remedy
// gotcha #42 prescribes: a rule watched by a single case is watched in a single shape. The
// local precedent is next door — `monotonic_as_wall.rs` and `wall_as_monotonic.rs` are two
// cases under one row, and so are the two `no_conversion_*` pairs. The block C counts do not
// move because of this file.
//
// ⛔ MEASURED IN BOTH DIRECTIONS, AND THE WORD IS `error`. Baseline: the case fails to compile
// with the oracle next to it, so trybuild prints `ok`. With `impl Default for Trust` added to
// `src/record.rs` it prints `error` and `Expected test case to fail to compile, but it
// succeeded.`, and removing the impl brings it back. Firing by COMPILING is the good answer:
// `TRYBUILD=overwrite` rewrites `.stderr` files and nothing else, so no bulk regeneration can
// silence this rule.
//
// ⛔ AND HERE IS THE PART THAT WAS FIRST WRITTEN IN A FORM NOBODY COULD REPRODUCE, corrected by
// measuring instead of by reasoning — gotcha #15 turned on its author. `#[cbor(default)]` ON
// THE FIELD DOES NOT COMPILE ON ITS OWN:
//
//     error[E0277]: the trait bound `Trust: Default` is not satisfied
//     help: the trait `Default` is not implemented for `Trust`
//
// The minicbor derive demands `Default` for the attribute, so the recipe is TWO LINES — the
// impl and the attribute — and never one. An earlier note here claimed the whole suite stayed
// green "with `#[cbor(default)]` present", which is true only alongside the impl; followed as
// written it yields a crate that does not build.
//
// ⛔ WHICH TURNS THE CONCLUSION AROUND, IN THE GUARD'S FAVOUR: the row's second half —
// "and it has no default" — is NOT uncovered. `Trust: Default` is the doorway every defaulting
// road must pass through, this case stands in it, and the two-line mutation trips it too
// (measured: with impl plus attribute, this case is `error`).
//
// ⚠️ THE TWO CASES ARE COMPLEMENTARY AND NEITHER IS REDUNDANT, measured rather than argued: with
// `impl Default for Trust` present, `record_without_trust_label.rs` stays `ok` and only this one
// goes red; with the FIELD REMOVED, the reverse. Each is blind to what the other catches.
//
// ⚠️ WHAT NEITHER CASE COVERS, declared here rather than discovered later: a default written BY
// HAND inside a bespoke `Decode` impl needs no `Default` at all, and no compile-fail case can
// see it. It is the same declared limit §2.8.4 carries for `Parameters::new`, and it is held at
// level 2 — for the record, by the frozen bytes of task 10.
//
// ⛔ Names `kernel::` and declares no attributes of its own — gotcha #39.
//
// ⛔ THE NOTE IS DOWN HERE ON PURPOSE: the oracle quotes LINE 6 of this file, so a paragraph
// added at the top would move the code and break it. Whoever writes here appends.
