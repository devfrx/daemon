// Rule: no `unsafe` in the kernel. Mechanism: `#![forbid(unsafe_code)]`.
// Force: compiler. Defends: constraint 2 of §11 — §7.4.1 block A.
#![forbid(unsafe_code)]

fn dereference() -> u8 {
    let x: u8 = 1;
    let p = &raw const x;
    unsafe { *p }
}

fn main() {}

// ⚠️ RECALL OF 2026-10-13 -- audit of 2026-09-30, AUD-1298: the constraint the header names is
// row 2 of the table of §11 of the compendium, whose source column cites ADR-0026 — the form
// `allow_overrides_forbid.rs` and `scripts/gate-attributes.sh` use.
//
// ⛔ IT DECLARES NO `#![no_std]`, AND THAT IS WHAT MAKES IT FIRE AS `error`, THE STRONG SHAPE
// (gotcha #42). A `#![no_std]` binary has no panic handler, so its oracle would carry errors of
// that shape and not of the rule -- `#[panic_handler]` required, unwinding without std -- and
// with `#![forbid(unsafe_code)]` taken away it would still fail, on those alone: `mismatch`, the
// verdict a bulk `TRYBUILD=overwrite` blesses in silence. Without it the case fails on the lint
// alone and, with the attribute taken away, COMPILES: trybuild answers `error`. Measured on
// 2026-10-14 in both directions. What holds the attribute in the two `lib.rs` is
// `scripts/gate-attributes.sh`, at level 2; this case holds the MECHANISM, on a crate of its own
// (gotcha #39). ⚠️ RECALL OF 2026-10-14 -- audit of 2026-09-30, AUD-2141, AUD-2142.
