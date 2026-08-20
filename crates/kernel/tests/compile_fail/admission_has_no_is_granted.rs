// `V4`, second half: there is no boolean shortcut on the answer either. `Admission` has no
// `is_granted()`, no `is_ok()`, and no conversion to `bool`.
//
// ⛔ IT NAMES A METHOD THAT DOES NOT EXIST, ON PURPOSE. Today that is `E0599`. The day
// somebody adds it this case starts COMPILING and trybuild reports it as `error` rather than
// through its oracle -- gotcha #42, the shape a bulk regeneration cannot disarm. The first
// half, `admission_is_not_two_ways.rs`, fires as `E0004` and DOES rest on its oracle: the
// two halves are complementary and neither is redundant.
//
// ⛔ Names `kernel::` and declares no attributes of its own -- gotcha #39.
fn main() {
    let mut arbiter = kernel::arbiter::Arbiter::new(
        kernel::parameters::Parameters::new(10_000, kernel::arbiter::Mib::new(16_384)),
        kernel::arbiter::VramPolicy::Remote(kernel::arbiter::RemotePolicy),
    );
    let outcome = arbiter.admit(
        &kernel::arbiter::ResourceProfile {
            name: "asr-realtime",
            reserved_vram: kernel::arbiter::Mib::new(1_024),
            compute_class: kernel::arbiter::ComputeClass::Realtime,
            preemption: kernel::arbiter::Preemption::Never,
        },
        kernel::time::Millis::new(1_000),
        kernel::time::Monotonic::ORIGIN,
    );
    if outcome.is_granted() {
        // nothing: the point is that this line must not compile
    }
}
