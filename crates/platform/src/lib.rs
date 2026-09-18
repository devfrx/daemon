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
//! ⚠️ DATED RECALL, 2026-09-17 -- THE OPENING SENTENCE, the one that lists `Journal`, `Reactor` and `Rng`, IS
//! FALSE FROM THIS TASK: `ipc::LocalSocketIpc` is the fourth. The `grep` above DOES find it -- `Ipc` is one of the names it
//! enumerates -- so the two halves of that sentence now contradict each other, which is why the prose is
//! dated here rather than left to be believed. The count is deliberately NOT rewritten into the sentence:
//! the command answers it, and a figure inside prose is gotcha #31.
//!
//! ⛔ DATED RECALL, 2026-09-18 -- THE SENTENCE ABOVE CALLED ITSELF "not written here as a fixed set" AND IT WAS
//! ONE. The `grep` it hands over ENUMERATES SEVEN TRAIT NAMES, so it can never answer with a
//! family added later: `Custody` -- the seventh port, §2 of the GUI north star -- was invisible
//! to it the moment `custody::FileCustody` existed. Measured, not reasoned. ⛔ AND THE CURE IS
//! NOT AN OPEN REGEX, which was measured too: `^impl [A-Za-z_]+ for ` catches `StorageBackend for
//! FileBackend` and `Default for SequentialRng`, neither of which is a port -- noise mistaken for
//! coverage, the opposite error and just as silent. So the list IS an enumeration, it is now said
//! to be one, and WHOEVER ADDS A PORT ADDS ITS NAME HERE, because nobody else can know.
//! ⚠️ AND THE THIRD FALSEHOOD IN THE SAME SENTENCE WAS A DEADLINE IN PROSE (gotcha #77):
//! "because milestone 6 adds to it". Milestone 6 CLOSED on 2026-09-02 and added nothing. It is
//! replaced by a fact instead of by another deadline.
//!
//! ⛔ The list comes from
//! `grep -rEn "^impl (Custody|Journal|Reactor|Rng|Filesystem|Network|Process|Ipc) for " crates/platform/src/`,
//! WHICH IS AN ENUMERATION OF THE PORT TRAITS AND NOT A DISCOVERY.
//!
//! ⛔ This crate USES `std` and WILL USE `unsafe` for FFI, and that is deliberate: it is
//! the place where I/O has to live (ADR-0031, perimeter). The functions below exist as
//! COUNTER-PROBES — they prove that the kernel's prohibitions do not fire where they
//! must not, which is the direction one forgets (§7.1.1 rule 3, gotcha #24). Do not
//! delete them until real code exists that demonstrates the same two things.

pub mod journal;

pub mod reactor;

pub mod rng;

pub mod ipc;

pub mod custody;

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
