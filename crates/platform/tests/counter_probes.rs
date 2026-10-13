//! The counter-probes of §7.1.1 rule 3: the kernel's prohibitions do NOT fire where they
//! must not. Without these, a rule that is too broad would pass for a rule that works —
//! it is the direction one forgets, and in M-3 the decisive probe was exactly this one.
//!
//! ⚠️ WHAT GOES RED IS THE COMPILER, AND NOT THE TWO ASSERTIONS BELOW. Each is true by
//! construction -- a size that is never zero, a value read back from where it was just written --
//! so no change that leaves the crate compiling can turn either of them red. A prohibition of the
//! kernel that reached this crate -- a `forbid(unsafe_code)` or a `no_std` -- would stop
//! `crates/platform/src/lib.rs` compiling, in every `cargo build` and before any test runs. What
//! this file adds is the anchor: deleting either function breaks this binary, so the probes cannot
//! vanish in silence. Whether they stay, now that real code does both things, is the choice the
//! crate doc of `platform` records. ⚠️ RECALL OF 2026-10-13 -- audit of 2026-09-30, AUD-2164.

#[test]
fn platform_names_std_and_compiles() {
    assert!(platform::counter_probe_std_compiles());
}

#[test]
fn platform_uses_unsafe_and_compiles() {
    assert_eq!(platform::counter_probe_unsafe_compiles(), 42);
}
