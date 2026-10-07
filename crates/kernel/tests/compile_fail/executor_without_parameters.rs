// Catalogue §7.4.1 block C, row `V29 · §2.8 · ADR-0034`: building a decision WITHOUT the
// delivered parameters must NOT compile. The executor's turn limit is a parameter, and
// `Executor::new` takes `Parameters` positionally so that leaving it out is an arity error
// rather than a silently defaulted value. Here is the rule firing.
//
// ⛔ The mechanism is the ABSENCE of any route that builds an executor without them: no
// `Default`, no builder, no optional argument. Nothing else in the gate would see such a
// route appear — `gate-attributes.sh` reads attributes, `gate-deps.sh` reads the dependency
// graph, `gate-no-os.sh` builds for a target without an OS, `check-docs.sh` does not read
// code at all — and `cargo build` compiles a constructor without `parameters` happily, because
// it is valid Rust.
//
// ⛔ And it trips as an `error`, not as a `mismatch`: the day `parameters` leaves the signature
// of `new` — the turn limit written inside as a constant, the rest of the signature kept — this
// case starts COMPILING, and trybuild says so outright instead of noticing through its oracle.
// A bulk regeneration of the `.stderr` files therefore cannot disarm it. Gotcha #42.
//
// ⛔ EVERY OTHER ARGUMENT IS HANDED OVER, ON PURPOSE, and it is what makes the paragraph above
// true: a call that also left out `&Sleep` would go on failing when `parameters` alone leaves the
// signature — for arity, a `mismatch` against this oracle — and the rule would rest on the
// `.stderr`. Measured on 2026-10-02 in both directions: with `parameters` dropped and `sleep`
// kept this case is `error`, with the signature as it is it is `ok`. Audit of 2026-09-30,
// AUD-697.
//
// ⚠️ WHAT IT DOES NOT SEE: a `Default` for `Parameters` is `parameters_have_no_default.rs`'s, not
// this case's; and a SECOND constructor under another name is not called here, so this case
// cannot see it — the review closes that road, as it does for `two_policies_at_once.rs`.
//
// ⚠️ THE LIMIT, declared before anyone discovers it: this proves that the executor RECEIVES its
// parameters, not that it has no others hidden inside as constants. The compiler cannot forbid a
// constant, and no level 2 check of §2.8.4 sees one either: no campaign varies a parameter, and
// example tests pin only some values. ⚠️ RECALL OF 2026-10-03 -- audit of 2026-09-30, AUD-711.
//
// ⚠️ AND THE ORACLE NEXT DOOR WILL GO `mismatch` THE DAY `Executor::new` GAINS AN ARGUMENT OR
// LOSES ONE OTHER THAN `parameters`: rustc quotes the arity and the signature verbatim. That
// regeneration is LEGITIMATE and it disarms nothing, because losing `parameters` — the one loss
// the rule is about — trips as `error` and never through the oracle. Regenerate by the
// documented route — delete the stale `.stderr`, re-run, `diff -u` the old against the `wip/`
// one, move by hand — and NEVER with `TRYBUILD=overwrite`, which would take the other oracles
// with it. Gotcha #25.
//
// ⛔ Names `kernel::` and declares no attributes of its own — gotcha #39.

use kernel::executor::{Executor, Sleep};
use kernel::ports::reactor::Reactor;
use kernel::rng::Rng;
use kernel::time::{Monotonic, WallTime};

struct StubRng;
impl Rng for StubRng {
    fn next_u64(&mut self) -> u64 {
        0
    }
}

struct StubReactor;
impl Reactor for StubReactor {
    fn now(&self) -> Monotonic {
        Monotonic::ORIGIN
    }
    fn wall_time(&self) -> WallTime {
        WallTime::from_millis_since_epoch(0)
    }
    fn wait_until(&mut self, _deadline: Monotonic) -> Option<Monotonic> {
        None
    }
}

fn main() {
    // The turn limit is a parameter, not a constant: it has to be handed over. Everything else
    // `new` wants IS handed over, so the one argument missing is the parameters.
    let sleep = Sleep::new();
    let _executor = Executor::new(StubRng, StubReactor, &sleep);
}
