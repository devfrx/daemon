// `V4`: the admission answers THREE ways. §5.3 point 1 -- "refused" and "queued" are
// DISTINCT outcomes, so whoever calls is obliged to tell them apart, and treating the
// answer as a yes/no does not compile: `E0004`, non-exhaustive patterns.
//
// ⚠️ IT NEEDS NO ARBITER, DELIBERATELY. This case is about the SHAPE of the answer, and the
// shape exists from the moment the enum does. The second half of the row -- that there is no
// `is_granted()` shortcut either -- needs a real admission and arrives with it.
fn two_ways(outcome: kernel::arbiter::Admission) -> bool {
    match outcome {
        kernel::arbiter::Admission::Granted(_) => true,
        kernel::arbiter::Admission::Refused { .. } => false,
    }
}

fn main() {
    let _ = two_ways;
}
