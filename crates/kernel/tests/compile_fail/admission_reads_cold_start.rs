// `Q8 · §5.2.1`: THE DECISION PATH cannot reach `cold_start`. `admit` receives a
// `ResourceProfile`, which has no such field -- `E0609`.
//
// ⚠️ WHAT THIS CASE SAID AT TASK 3, written out because a comment that keeps its old tense
// lies with authority: it declared itself "the FIRST HALF" and said the row was only
// PARTIALLY covered, because what §5.2.1 forbids is not "a struct without a field" but THE
// DECISION PATH reading one -- and no decision path existed. It does now, it is the call
// below, and the row closes here.
//
// ⛔ THE CALL BELOW DOES NOT PARTICIPATE IN THE ERROR, and this paragraph used to claim it
// did. `E0609` comes from the LITERAL plus the field access, and is raised with the call
// DELETED -- ✅ measured 2026-08-19. What handing the profile to `admit` buys is a coupling to
// the SIGNATURE, and it is of grade `mismatch`, not of grade `error`: see the register.
//
// ⛔ Names `kernel::` and declares no attributes of its own -- gotcha #39.
fn main() {
    let mut arbiter = kernel::arbiter::Arbiter::new(
        kernel::parameters::Parameters::new(10_000, kernel::arbiter::Mib::new(16_384), kernel::arbiter::ArbiterId::new(1)),
        kernel::arbiter::VramPolicy::Remote(kernel::arbiter::RemotePolicy),
    );
    let profile = kernel::arbiter::ResourceProfile {
        name: "asr-realtime",
        reserved_vram: kernel::arbiter::Mib::new(1_024),
        compute_class: kernel::arbiter::ComputeClass::Realtime,
        preemption: kernel::arbiter::Preemption::Never,
    };
    let _decide_on_it = profile.cold_start;
    let _ = arbiter.admit(
        &profile,
        kernel::time::Millis::new(1_000),
        kernel::time::Monotonic::ORIGIN,
    );
}
