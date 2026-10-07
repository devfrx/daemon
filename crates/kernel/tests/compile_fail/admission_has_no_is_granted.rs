// `V4`, second half: there is no boolean shortcut on the answer either. This case holds the
// `is_granted()` road; `is_ok()` and the conversion to `bool` have a case each, beside it.
//
// ⛔ IT NAMES A METHOD THAT DOES NOT EXIST, ON PURPOSE. Today that is `E0599`. The day
// somebody adds it this case starts COMPILING and trybuild reports it as `error` rather than
// through its oracle -- gotcha #42, the shape a bulk regeneration cannot disarm. The first half, `admission_is_not_two_ways.rs`,
// fires by HOW the rule breaks: `error` without `Queued`, `mismatch` (`E0599`) without `Refused` or `Granted` -- and under all
// three `tests/arbiter_admission.rs` stops compiling (docs/porta-di-qualita.md). Audit of 2026-09-30, AUD-705.
//
// ⛔ Names `kernel::` and declares no attributes of its own -- gotcha #39.
fn main() {
    let mut arbiter = kernel::arbiter::Arbiter::new(
        kernel::parameters::Parameters::new(10_000, kernel::arbiter::Mib::new(16_384), kernel::arbiter::ArbiterId::new(1), kernel::time::Millis::new(0)),
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

// ⛔ THE `Parameters::new` CALL ABOVE RUNS LONG ON PURPOSE, AND THIS NOTE SITS AT THE
// BOTTOM FOR THE SAME REASON: anything inserted above it shifts the asserted error below
// and rewrites this case's `.stderr` line numbers for nothing. `cargo fmt` does not reach
// this directory, so nothing else objects to the width.
//
// ⚠️ ONE ROAD OF THREE, AND THE OTHER TWO ARE NOT SEEN FROM HERE: with `is_ok` added to
// `Admission`, or with `impl From<Admission> for bool`, this case stays `ok` — measured on
// 2026-10-02. The design of milestone 5 forbids all three shortcuts, so the other two are held by
// `admission_has_no_is_ok.rs` and `admission_is_not_a_bool.rs`. Audit of 2026-09-30, AUD-695.
