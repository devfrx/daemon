// `Q8 · §5.2.1`: the profile the arbiter receives HAS NO `cold_start`, so a decision that
// wanted it has no way -- `E0609`.
//
// ⚠️ THIS CASE IS THE FIRST HALF. What the row forbids is not "a struct without a field",
// it is THE DECISION PATH reading it, and the decision path does not exist yet: `admit`
// arrives with task 5, and this case is rewritten there to name it. Until then the row is
// registered as PARTIALLY covered rather than closed -- a row proved in one direction only
// is not admissible (§7.1.1 rule 3).
fn main() {
    let profile = kernel::arbiter::ResourceProfile {
        name: "asr-realtime",
        reserved_vram: kernel::arbiter::Mib::new(1_024),
        compute_class: kernel::arbiter::ComputeClass::Realtime,
        preemption: kernel::arbiter::Preemption::Never,
    };
    let _warn_the_user = profile.cold_start;
}
