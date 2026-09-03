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
