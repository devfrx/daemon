// Catalogue §7.4.1 block C, row `V29 · §2.1`, rule B: there is no `From`/`Into` path
// between the two times — the other direction.
//
// ⛔ Rule A — that one type cannot be passed where the other is expected — is guarded by
// `monotonic_as_wall.rs`. That guard is real but INDIRECT: with an `impl From` present
// the call site still fails with E0308, and what turns it red is the `help: call
// Into::into` lines that rustc appends and the oracle does not carry. Measured on the
// sibling direction. So its power rests entirely on the oracle never being regenerated
// in bulk — gotcha #25, in a form nobody had written down.
//
// This case is the DIRECT guard: with an `impl From<Monotonic> for WallTime` present it
// COMPILES, and trybuild trips with "Expected test case to fail to compile, but it
// succeeded". No dependency on suggestion text.
//
// ⛔ Names `kernel::` and declares no attributes of its own — gotcha #39.

use kernel::time::{Monotonic, WallTime};

fn main() {
    let deadline = Monotonic::from_millis(5_000);
    let _stamp: WallTime = deadline.into();
}
