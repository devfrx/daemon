//! The real implementations of the ports the kernel declares. Today they are `Journal`
//! (`journal::FileJournal`), `Reactor` (`reactor::SystemReactor`) and `Rng`
//! (`rng::SequentialRng`) -- the list is not written here as a fixed set, because milestone 6
//! adds to it; it comes from
//! `grep -rEn "^impl (Journal|Reactor|Rng|Filesystem|Network|Process|Ipc) for " crates/platform/src/`,
//! which answered those three on 2026-08-28. `Rng` is a port of its own and NOT one of the six
//! families (§2.2): the kernel declares it in `kernel::rng`, outside `kernel::ports`. In
//! production its randomness is FAKE, deliberately: see `rng::SequentialRng`.
//!
//! ⛔ RECALL OF 2026-08-28, FINDING AUD-022 -- THIS SAID "filesystem, clock, network, processes,
//! confinement", AND EVERY ONE OF THE FIVE WAS WRONG IN ONE OF TWO WAYS. `clock` and
//! `confinement` ARE NOT KERNEL PORTS AT ALL -- `grep -rniE "pub trait (Clock|Confinement)"
//! crates/` answers ZERO -- and the other three are ports the kernel does declare but that THIS
//! CRATE DOES NOT IMPLEMENT. Meanwhile the two it does implement, `Journal` and `Reactor`, were
//! missing from the list entirely. ⛔ AND `clock` IS NOT A TYPO: a separate `clock` family was
//! EVALUATED AND REJECTED -- decision D2 of the milestone 2 plan, recorded in
//! `kernel::ports::reactor`, "a separate `clock` family would split one source of virtual time
//! across two ports" -- so naming it here contradicts a decision instead of merely being stale.
//! The sentence was a literal translation of the spec's §1.2 cell, written when the crate layout
//! was decided; the six families were fixed AFTERWARDS in §2.3, and the line was never reread
//! against them. `kernel::ports` declares six modules: filesystem, ipc, journal, network,
//! process, reactor.
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
