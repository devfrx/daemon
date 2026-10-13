// ⛔ THE CASE THAT JUSTIFIES `forbid` INSTEAD OF `deny`: without it, constraint 2 of §11
// stays a preference. With `deny` the `#[allow]` wins and the `unsafe` is no longer
// flagged (it still does not compile, but because of the codegen noise); `forbid`
// rejects it: E0453.
#![no_std]
#![forbid(unsafe_code)]

#[allow(unsafe_code)]
fn dereference() -> u8 {
    let x: u8 = 1;
    let p = &raw const x;
    unsafe { *p }
}

fn main() {}

// ⚠️ IT FIRES ONLY AS `mismatch`, and the "codegen noise" of the header is why (gotcha #42): the
// case is a `#![no_std]` BINARY with no panic handler, so its oracle carries two errors that come
// from that shape and not from the rule, and nothing done to the rule can make it compile. With
// `deny` in place of `forbid` the `#[allow]` wins, the case fails on the noise alone, and
// trybuild answers `mismatch`, the verdict a bulk `TRYBUILD=overwrite` blesses in silence.
// Measured on 2026-10-13, as was the way out: WITHOUT `#![no_std]` the same swap makes the case
// COMPILE — `error`. Whether the case keeps the kernel's `#![no_std]` or takes that strong shape
// is not decided here; `unsafe_in_kernel.rs` carries the same limit. ⚠️ RECALL OF 2026-10-13 --
// audit of 2026-09-30, AUD-2142.
