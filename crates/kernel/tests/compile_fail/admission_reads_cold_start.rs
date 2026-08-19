// `Q8 · §5.2.1`: THE DECISION PATH cannot reach `cold_start`. `admit` receives a
// `ResourceProfile`, which has no such field -- `E0609`.
//
// ⚠️ WHAT THIS CASE SAID AT TASK 3, written out because a comment that keeps its old tense
// lies with authority: it declared itself "the FIRST HALF" and said the row was only
// PARTIALLY covered, because what §5.2.1 forbids is not "a struct without a field" but THE
// DECISION PATH reading one -- and no decision path existed. It does now, it is the call
// below, and the row closes here.
//
// ⛔ THE PROFILE IS BUILT AND THEN HANDED TO `admit` IN THE SAME `main`, and that is the
// point rather than ceremony: it is the exact material a real admission runs on, so "the
// decision cannot see `cold_start`" is proved on the argument the decision actually takes,
// not on a type quoted out of context.
//
// ⛔ Names `kernel::` and declares no attributes of its own -- gotcha #39.
fn main() {
    let mut arbiter = kernel::arbiter::Arbiter::new(kernel::parameters::Parameters::new(
        10_000,
        kernel::arbiter::Mib::new(16_384),
    ));
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
