// Catalogue §7.4.1 block C, row `V29 · §2.8 · ADR-0034`: building a decision WITHOUT the
// delivered parameters must NOT compile. The executor's turn limit is a parameter, and
// `Executor::new` takes `Parameters` positionally so that leaving it out is an arity error
// rather than a silently defaulted value. Here is the rule firing.
//
// ⛔ The mechanism is the ABSENCE of any route that builds an executor without them: no
// `Default`, no builder, no optional argument. Nothing else in the gate would see such a
// route appear — `gate-attributes.sh` reads attributes, `gate-deps.sh` reads the dependency
// graph, `gate-no-os.sh` builds for a target without an OS, `check-docs.sh` does not read
// code at all — and `cargo build` compiles a two-argument constructor happily, because it is
// valid Rust.
//
// ⛔ And it trips as an `error`, not as a `mismatch`: the day somebody adds an overload-like
// route — a `new` with fewer arguments, a `Default` for `Parameters` feeding a fallback —
// this case starts COMPILING, and trybuild says so outright instead of noticing through its
// oracle. A bulk regeneration of the `.stderr` files therefore cannot disarm it. Gotcha #42.
//
// ⚠️ THE LIMIT, declared before anyone discovers it: this proves that the executor RECEIVES
// its parameters, not that it has no others hidden inside as constants. The compiler cannot
// forbid a constant. That hole is covered — only for the parameters the campaign actually
// varies — by the level 2 check of §2.8.4, and it is NOT a proof of absence.
//
// ⚠️ AND THE ORACLE NEXT DOOR WILL GO `mismatch` THE DAY `Executor::new` GAINS OR LOSES AN
// ARGUMENT: rustc quotes the arity and the signature verbatim. That regeneration is
// LEGITIMATE and it disarms nothing, because the rule trips as `error` and never through the
// oracle. Regenerate by the documented route — delete the stale `.stderr`, re-run, `diff -u`
// the old against the `wip/` one, move by hand — and NEVER with `TRYBUILD=overwrite`, which
// would take the other oracles with it. Gotcha #25.
//
// ⛔ Names `kernel::` and declares no attributes of its own — gotcha #39.

use kernel::executor::Executor;
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
    // The turn limit is a parameter, not a constant: it has to be handed over.
    let _executor = Executor::new(StubRng, StubReactor);
}
