// ⛔ THE CASE THAT JUSTIFIES `forbid` INSTEAD OF `deny`: without it, constraint 2 of §11
// stays a preference. With `deny` the `#[allow]` wins, the `unsafe` is no longer flagged and
// the case compiles; `forbid` rejects it: E0453.
#![forbid(unsafe_code)]

#[allow(unsafe_code)]
fn dereference() -> u8 {
    let x: u8 = 1;
    let p = &raw const x;
    unsafe { *p }
}

fn main() {}

// ⛔ IT FIRES AS `error`, THE STRONG SHAPE (gotcha #42), BECAUSE IT DECLARES NO `#![no_std]`: a
// `#![no_std]` binary has no panic handler, so its oracle would carry two errors of that shape
// and not of the rule, and with `deny` in place of `forbid` the case would still fail, on those
// alone -- `mismatch`, the verdict a bulk `TRYBUILD=overwrite` blesses in silence. Without it the
// same swap makes the case COMPILE: trybuild answers `error`. Measured on 2026-10-14 in both
// directions; `unsafe_in_kernel.rs` takes the same shape. ⚠️ RECALL OF 2026-10-14 -- audit of
// 2026-09-30, AUD-2141, AUD-2142.
