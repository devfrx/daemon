// Rule: no `unsafe` in the kernel. Mechanism: `#![forbid(unsafe_code)]`.
// Force: compiler. Defends: constraint 2 of §11 — §7.4.1 block A.
#![no_std]
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
// ⚠️ IT FIRES ONLY AS `mismatch`, declared rather than left to be discovered (gotcha #42). The
// case is a `#![no_std]` BINARY with no panic handler, so its oracle carries two errors that come
// from that shape and not from the rule — `#[panic_handler]` required, unwinding without std —
// and nothing done to the rule can make it compile: with `#![forbid(unsafe_code)]` taken away
// it still fails, on those two alone, and trybuild answers `mismatch`, the verdict a bulk
// `TRYBUILD=overwrite` blesses in silence. Measured on 2026-10-13, as was the way out: the same
// case WITHOUT `#![no_std]` fails on the lint alone and, with the attribute taken away, COMPILES
// — `error`. Whether the case keeps the kernel's `#![no_std]` or takes that strong shape is not
// decided here. Until then its oracle is read by hand at every regeneration (gotcha #25), and
// `scripts/gate-attributes.sh` holds the attribute itself in the two `lib.rs`, at level 2.
// ⚠️ RECALL OF 2026-10-13 -- audit of 2026-09-30, AUD-2142.
