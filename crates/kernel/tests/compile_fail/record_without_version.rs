//! Catalogue §7.4.1 block C, row `Q14 · §4.9` — a durable record WITHOUT a version is not
//! expressible. The type is a version enum, so the inner value cannot stand alone where a
//! record is expected.

fn main() {
    let inner = kernel::record::RecordV1 {
        kind: kernel::record::RecordKind::Intent,
        effect: kernel::record::EffectClass::Idempotent,
        trust: kernel::record::Trust::Instruction,
        payload: Vec::new(),
        reason: String::new(),
        detail: None,
    };

    // The bare V1 body is NOT a record: only `Record::V1(..)` is.
    let _bytes = inner.encode();
}

// ⚠️ THE ORACLE OF THIS CASE IS COUPLED TO `minicbor`, declared rather than left to be found
// out. `RecordV1` derives `minicbor::Encode`, whose trait method is ALSO called `encode`, so
// the `.stderr` does not stop at "no method named `encode`": it goes on to quote
// `src/encode.rs` of `minicbor`, the trait method's own signature line, and the suggestion
// `use minicbor::encode::Encode;`. Whoever bumps `minicbor` reads this file first. ⚠️ THE
// VERSION ITSELF IS NOT IN THE COUPLING — `trybuild` normalises the path to
// `$CARGO/minicbor-$VERSION/..` — so what breaks the oracle is a MOVED OR RESHAPED trait, not
// a version bump on its own.
//
// ⚠️ AND WHAT THIS CASE PROVES IS THEREFORE NARROWER THAN ITS NAME, which is worth writing
// down before it is trusted for more: `RecordV1` DOES have an `encode`, the trait one, merely
// out of scope here. What is pinned is that the INHERENT `encode` — the one that produces the
// bytes the journal exchanges — lives on `Record` and not on the bare V1 body, which is rule 1
// of §4.9.2. It does not pin that no other road reaches those bytes.
//
// ⛔ THIS CASE FIRES AS `error`, NOT AS `mismatch`, and it was measured rather than hoped:
// with an inherent `encode` added to `RecordV1` in `src/record.rs`, `trybuild` prints
// `test .. record_without_version.rs ... error` and `Expected test case to fail to compile,
// but it succeeded.` That is the answer gotcha #42 asks for, and it is the GOOD one:
// `TRYBUILD=overwrite` rewrites `.stderr` files and nothing else, so a bulk regeneration
// CANNOT silence a case that fires by compiling. No second case of a different shape is owed
// here — unlike a rule watched only by `mismatch`, which a regeneration switches off quietly.
//
// ⛔ THE NOTE IS DOWN HERE ON PURPOSE: the oracle quotes the LINE OF THE CALL, so a paragraph
// added at the top would move the code and break it. Whoever writes here appends.
//
// ⛔ AND THE LINE NUMBER MOVED ON 2026-08-10, WHICH IS THE HAZARD THIS FILE ALREADY WARNS
// ABOUT FROM THE OTHER END. The note above says a paragraph added at the TOP would break the
// oracle; index 4 of the record added a line INSIDE the literal, which does the same thing —
// `inner.encode()` went from line 14 to line 15 and the `.stderr` was corrected BY HAND to
// match. Not regenerated: `TRYBUILD=overwrite` would have rewritten the oracle wholesale and a
// real change hiding in it would have gone in unread.
//
// ⛔ AND IT MOVED AGAIN WHEN INDEX 5 ARRIVED, THE SAME WAY AND FOR THE SAME REASON: `detail`
// added a line inside the literal, so `inner.encode()` went from line 15 to line
// 16, and the `.stderr` was corrected BY HAND once more. ⚠️ THE NOTE ABOVE SAID "LINE 14"
// AND WAS ALREADY ONE BEHIND when this happened, which is worth more than the number: the
// paragraph warning about the hazard had itself gone stale, and nothing turns that red. So
// the numeral is TAKEN OUT of it rather than realigned to 16 -- the oracle quotes whatever
// line the call is on, and that line lives in the `.stderr`, which is the one house that
// cannot drift out of step with the file it describes.
