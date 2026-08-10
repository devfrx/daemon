//! The real implementations of the traits the kernel declares: filesystem, clock,
//! network, processes, confinement — and `rng`, which is a port of its own and not one
//! of the six families (§2.2). In production its randomness is FAKE, deliberately: see
//! `rng::SequentialRng`.
//!
//! ⛔ This crate USES `std` and WILL USE `unsafe` for FFI, and that is deliberate: it is
//! the place where I/O has to live (ADR-0031, perimeter). The functions below exist as
//! COUNTER-PROBES — they prove that the kernel's prohibitions do not fire where they
//! must not, which is the direction one forgets (§7.1.1 rule 3, gotcha #24). Do not
//! delete them until real code exists that demonstrates the same two things.

pub mod journal;

pub mod reactor;

pub mod rng;

/// Counter-probe of `no_std`: `platform` names `std::fs` and **compiles**.
pub fn counter_probe_std_compiles() -> bool {
    core::mem::size_of::<std::fs::File>() > 0
}

/// Counter-probe of `forbid(unsafe_code)`: `platform` uses `unsafe` and **compiles**.
///
/// If someone declared the prohibitions at workspace level, this function would stop
/// compiling — and that is exactly what the counter-probe has to intercept.
pub fn counter_probe_unsafe_compiles() -> usize {
    let x: u8 = 42;
    let p = &raw const x;
    // SAFETY: `p` derives from a reference to `x`, alive for the whole function.
    unsafe { *p as usize }
}
