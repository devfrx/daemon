// Catalogue §7.4.1 block C, row `V2`: an admission WITHOUT a resource profile must NOT
// compile. `Arbiter::admit` takes the `&ResourceProfile` positionally, so leaving it out is
// an arity error rather than an admission that decides on nothing. Here is the rule firing.
//
// ⛔ IT IS THE SAME MECHANISM AS `executor_without_parameters.rs`, deliberately: no
// `Default` for `ResourceProfile`, no builder, no optional argument, no second `admit` with
// fewer arguments. Nothing else in the gate would see such a route appear —
// `gate-attributes.sh` reads attributes, `gate-deps.sh` reads the dependency graph,
// `gate-no-os.sh` builds for a target without an OS, `check-docs.sh` does not read code at
// all — and `cargo build` compiles a two-argument admission happily, because it is valid
// Rust.
//
// ⛔ And it trips as an `error`, not as a `mismatch`: the day somebody adds an overload-like
// route this case starts COMPILING, and trybuild says so outright instead of noticing
// through its oracle. A bulk regeneration of the `.stderr` files therefore cannot disarm it.
// Gotcha #42.
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
        kernel::parameters::Parameters::new(10_000, kernel::arbiter::Mib::new(16_384), kernel::arbiter::ArbiterId::new(1)),
        kernel::arbiter::VramPolicy::Remote(kernel::arbiter::RemotePolicy),
    );
    // The profile is what the arbiter decides ON: it has to be handed over.
    let _outcome = arbiter.admit(
        kernel::time::Millis::new(1_000),
        kernel::time::Monotonic::ORIGIN,
    );
}
