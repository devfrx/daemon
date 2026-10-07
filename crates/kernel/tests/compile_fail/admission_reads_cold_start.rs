// `Q8 · §5.2.1`: THE DECISION PATH cannot reach `cold_start`. `admit` receives a
// `ResourceProfile`, which has no such field -- `E0609`.
//
// ⚠️ WHAT THIS CASE HOLDS is that `ResourceProfile`, the type `admit` receives, has no field
// `cold_start`: §5.2.1 forbids THE DECISION PATH reading it, and that path receives this type
// and not `WorkDescriptor`, which carries the field. It does not hold that the call below reads
// anything -- the next paragraph -- and docs/porta-di-qualita.md says the same. ⚠️ RECALL OF
// 2026-10-07 -- audit of 2026-09-30, AUD-309.
//
// ⛔ THE CALL BELOW DOES NOT PARTICIPATE IN THE ERROR, and this paragraph used to claim it
// did. `E0609` comes from the LITERAL plus the field access, and is raised with the call
// DELETED -- ✅ measured 2026-08-19. What handing the profile to `admit` buys is a coupling to
// the SIGNATURE, and it is of grade `mismatch`, not of grade `error`: see the register.
//
// ⛔ Names `kernel::` and declares no attributes of its own -- gotcha #39.
fn main() {
    let mut arbiter = kernel::arbiter::Arbiter::new(
        kernel::parameters::Parameters::new(10_000, kernel::arbiter::Mib::new(16_384), kernel::arbiter::ArbiterId::new(1), kernel::time::Millis::new(0)),
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

// ⛔ THE `Parameters::new` CALL ABOVE RUNS LONG ON PURPOSE, AND THIS NOTE SITS AT THE
// BOTTOM FOR THE SAME REASON: anything inserted above it shifts the asserted error below
// and rewrites this case's `.stderr` line numbers for nothing. `cargo fmt` does not reach
// this directory, so nothing else objects to the width.
