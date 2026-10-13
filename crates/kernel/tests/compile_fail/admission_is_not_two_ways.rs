// `V4`: the admission answers THREE ways. §5.3 point 1 -- "refused" and "queued" are
// DISTINCT outcomes, so whoever calls is obliged to tell them apart, and treating the
// answer as a yes/no does not compile: `E0004`, non-exhaustive patterns.
//
// ⚠️ IT NEEDS NO ARBITER, DELIBERATELY. This case is about the SHAPE of the answer, and the
// shape exists from the moment the enum does. The second half of the row -- no boolean shortcut
// on the answer -- is `admission_has_no_is_granted.rs` and its two siblings, one road each.
fn two_ways(outcome: kernel::arbiter::Admission) -> bool {
    match outcome {
        kernel::arbiter::Admission::Granted(_) => true,
        kernel::arbiter::Admission::Refused { .. } => false,
    }
}

fn main() {
    let _ = two_ways;
}

// ⚠️ RECALL OF 2026-10-13 -- audit of 2026-09-30, AUD-1285, AUD-1286: the second half of the row
// named at the top is written, three cases beside this one. The note sits down here because the
// oracle next door quotes the lines of the `match`.
