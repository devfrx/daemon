// Rule: no `unsafe` in the kernel. Mechanism: `#![forbid(unsafe_code)]`.
// Force: compiler. Defends: ADR-0026 constraint 2 — §7.4.1 block A.
#![no_std]
#![forbid(unsafe_code)]

fn dereference() -> u8 {
    let x: u8 = 1;
    let p = &raw const x;
    unsafe { *p }
}

fn main() {}
