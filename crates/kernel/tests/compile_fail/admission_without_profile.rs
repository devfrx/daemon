// Catalogue §7.4.1 block C, row `V2`: an admission WITHOUT a resource profile must NOT
// compile. `Arbiter::admit` takes the `&ResourceProfile` positionally, so leaving it out is
// an arity error rather than an admission that decides on nothing. Here is the rule firing.
//
// ⛔ IT IS THE SAME MECHANISM AS `executor_without_parameters.rs`, deliberately: no
// `Default` for `ResourceProfile`, no builder, no optional argument, no second `admit` with
// fewer arguments. Of those roads THIS CASE SEES ONE, the arity of `admit` -- the foot of the
// file says what it does not see -- and nothing else in the gate sees any: `gate-attributes.sh`
// reads attributes, `gate-deps.sh` the dependency graph, `gate-no-os.sh` builds for a target
// without an OS, `check-docs.sh` does not read code at all — and `cargo build` compiles a
// two-argument admission happily, because it is valid Rust.
//
// ⛔ And on that road it trips as an `error`, not as a `mismatch`: the day the profile leaves the
// signature of `admit`, the rest kept, this case starts COMPILING, and trybuild says so outright
// instead of noticing through its oracle. A bulk regeneration of the `.stderr` files therefore
// cannot disarm it. Gotcha #42.
//
// ⚠️ THE LIMIT, declared before anyone discovers it: this proves that the admission RECEIVES
// a profile, not that the profile it receives is the RIGHT one, and not that the numbers in
// it are calibrated. Calibration is SP-1 and a parameter — the spec says so on the row
// itself.
//
// ⚠️ THE COUNTER-PROBE — "with the profile it compiles" — is not a case but the whole of
// `crates/kernel/tests/arbiter_admission.rs`: every probe there calls `admit` WITH one, from
// outside the crate. §7.1.1 rule 3 wants both directions and that is where the other lives.
//
// ⚠️ AND THE ORACLE NEXT DOOR WILL GO `mismatch` THE DAY `admit` GAINS OR LOSES AN ARGUMENT:
// rustc quotes the arity and the signature verbatim. That regeneration is LEGITIMATE and it
// disarms nothing, because the rule trips as `error` and never through the oracle. Regenerate
// by the documented route — delete the stale `.stderr`, re-run, `diff -u` the old against the
// `wip/` one, move by hand — and NEVER with `TRYBUILD=overwrite`, which would take the other
// oracles with it. Gotcha #25.
//
// ⛔ Names `kernel::` and declares no attributes of its own — gotcha #39.
fn main() {
    let mut arbiter = kernel::arbiter::Arbiter::new(
        kernel::parameters::Parameters::new(10_000, kernel::arbiter::Mib::new(16_384), kernel::arbiter::ArbiterId::new(1), kernel::time::Millis::new(0)),
        kernel::arbiter::VramPolicy::Remote(kernel::arbiter::RemotePolicy),
    );
    // The profile is what the arbiter decides ON: it has to be handed over.
    let _outcome = arbiter.admit(
        kernel::time::Millis::new(1_000),
        kernel::time::Monotonic::ORIGIN,
    );
}

// ⛔ THE `Parameters::new` CALL ABOVE RUNS LONG ON PURPOSE, AND THIS NOTE SITS AT THE
// BOTTOM FOR THE SAME REASON: anything inserted above it shifts the asserted error below
// and rewrites this case's `.stderr` line numbers for nothing. `cargo fmt` does not reach
// this directory, so nothing else objects to the width.
//
// ⚠️ WHAT IT DOES NOT SEE, declared rather than left to be discovered — the species of limit
// `two_policies_at_once.rs` measured for `Arbiter::new`, and the one
// `executor_without_parameters.rs` declares for its own constructor. A `Default` or a builder for
// `ResourceProfile`, or a second admission under another name, is not called here and leaves
// this case `ok`; an optional profile, `Option<&ResourceProfile>`, keeps the arity, so the case
// goes on failing with another note — `mismatch`, measured on 2026-10-13, the shape a bulk
// regeneration blesses. The review closes those roads, not the compiler. ⚠️ RECALL OF 2026-10-13
// -- audit of 2026-09-30, AUD-2140.
