// Catalogue §7.4.1 block C, row `V29 · §2.1`, rule B: there is no `From`/`Into` path
// between the two times.
//
// ⛔ Rule A — that one type cannot be passed where the other is expected — is guarded by
// `wall_as_monotonic.rs`. That guard is real but INDIRECT: with an `impl From` present
// the call site still fails with E0308, and what turns it red is the four lines of
// `help: call Into::into` that rustc appends and the oracle does not carry. Measured. So
// its power rests entirely on the oracle never being regenerated in bulk — gotcha #25,
// in a form nobody had written down.
//
// This case is the DIRECT guard: with an `impl From<WallTime> for Monotonic` present it
// COMPILES, and trybuild trips with "Expected test case to fail to compile, but it
// succeeded". No dependency on suggestion text.
//
// ⛔ Names `kernel::` and declares no attributes of its own — gotcha #39.

use kernel::time::{Monotonic, WallTime};

fn main() {
    let now_in_the_world = WallTime::from_millis_since_epoch(1_775_000_000_000);
    let _deadline: Monotonic = now_in_the_world.into();
}
