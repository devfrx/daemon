// Regola: niente `unsafe` nel kernel. Meccanismo: `#![forbid(unsafe_code)]`.
// Forza: compilatore. Difende: ADR-0026 vincolo 2 — §7.4.1 blocco A.
#![no_std]
#![forbid(unsafe_code)]

fn deferenzia() -> u8 {
    let x: u8 = 1;
    let p = &raw const x;
    unsafe { *p }
}

fn main() {}
