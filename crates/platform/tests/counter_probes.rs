//! The counter-probes of §7.1.1 rule 3: the kernel's prohibitions do NOT fire where they
//! must not. Without these, a rule that is too broad would pass for a rule that works —
//! it is the direction one forgets, and in M-3 the decisive probe was exactly this one.

#[test]
fn platform_names_std_and_compiles() {
    assert!(platform::counter_probe_std_compiles());
}

#[test]
fn platform_uses_unsafe_and_compiles() {
    assert_eq!(platform::counter_probe_unsafe_compiles(), 42);
}
