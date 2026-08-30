// Catalogue §7.4.1 block C, row `V3`: a SECOND ACTIVE POLICY. The delivered value carries ONE,
// so an arbiter cannot be handed both -- `Arbiter::new` takes the policy POSITIONALLY, and a
// second one is an arity error. Here is the rule firing.
//
// ⛔ THE PLAN DICTATED A DIFFERENT CASE AND IT WAS MEASURED AND REJECTED, which is written here
// because the rejected one looks fine. It wrote `VramPolicy::Remote(..) | VramPolicy::Local(..)`
// and proved that `VramPolicy` does not implement `BitOr` -- `error[E0369]`, measured. ✅ AND
// UNDER THE MUTATION `V3` ACTUALLY FEARS -- an `Arbiter::new` that ACCEPTS two policies -- THAT
// CASE STAYS `ok`: it never names `Arbiter::new`, so the regression is invisible to it. This one
// goes to `error`. Measured on 2026-08-20, both in the same run; registered as `E89`.
//
// ⛔ And it trips as an `error`, not as a `mismatch`: the day somebody widens the constructor
// this case starts COMPILING, and trybuild says so outright instead of noticing through its
// oracle. A bulk regeneration of the `.stderr` files therefore cannot disarm it. Gotcha #42.
//
// ⚠️ THE LIMIT, declared before anyone discovers it, and MEASURED rather than assumed: this pins
// the ARITY OF `new`, not the absence of every route to two policies. ✅ With a second
// constructor added -- `pub const fn new_with_two(parameters, a, _b)` -- this case stays `ok` and
// the whole suite stays green, measured on 2026-08-20. What closes that road is that nobody
// writes such a constructor, which is review and not the compiler. The same species of limit
// `admission_without_profile.rs` declares for itself.
//
// ⚠️ AND THE ORACLE NEXT DOOR WILL GO `mismatch` THE DAY `Arbiter::new` GAINS OR LOSES AN
// ARGUMENT: rustc quotes the signature verbatim, exactly as it does for
// `parameters_have_no_default.stderr`. That regeneration is LEGITIMATE and it disarms nothing,
// because the rule trips as `error` and never through the oracle. Regenerate by the documented
// route -- delete the stale `.stderr`, re-run, `diff -u` the old against the `wip/` one, move by
// hand -- and NEVER with `TRYBUILD=overwrite`, which would take the other oracles with it.
// Gotcha #25.
//
// ⚠️ THE COUNTER-PROBE -- "with ONE policy it compiles" -- is not a case but the whole of
// `crates/kernel/tests/arbiter_policy.rs`, which builds an arbiter with each of the two from
// outside the crate. §7.1.1 rule 3 wants both directions and that is where the other lives. The
// second half of the row's counter-probe -- "and the transition stays a journalled step (§5.4)"
// -- is task 9's.
//
// ⛔ Names `kernel::` and declares no attributes of its own -- gotcha #39.
fn main() {
    let _arbiter = kernel::arbiter::Arbiter::new(
        kernel::parameters::Parameters::new(10_000, kernel::arbiter::Mib::new(16_384), kernel::arbiter::ArbiterId::new(1)),
        kernel::arbiter::VramPolicy::Remote(kernel::arbiter::RemotePolicy),
        kernel::arbiter::VramPolicy::Local(kernel::arbiter::LocalPolicy),
    );
}
