// The rule of §2.2: the reduction in `kernel::rng::RngExt::below` is THE SAME for every
// implementation. Two implementations that reduced differently would produce different
// traces from the same seed, and nothing on the surface would show it — which is the class
// of defect this sub-project exists to remove. Here is the rule firing.
//
// ⛔ The mechanism, and it is why `below` is not a default method on `Rng`: a default
// method can be OVERRIDDEN, so the rule would have been a comment. `below` lives on the
// separate trait `RngExt`, which a blanket impl already implements for every `Rng`; a
// hand-written impl therefore collides with it. E0119, conflicting implementations.
//
// ⛔ And it trips as an `error`, not as a `mismatch`: delete the blanket impl in
// `crates/kernel/src/rng.rs` — the only way to defeat the rule — and this case starts
// COMPILING, which trybuild reports outright instead of through its oracle. So a bulk
// regeneration cannot silently disarm it. Gotcha #42.
//
// ⛔ Names `kernel::` and declares no attributes of its own — gotcha #39.

use kernel::rng::{Rng, RngExt};

struct Loaded(u64);

impl Rng for Loaded {
    fn next_u64(&mut self) -> u64 {
        self.0
    }
}

impl RngExt for Loaded {
    fn below(&mut self, _n: u64) -> u64 {
        0
    }
}

fn main() {
    let mut rng = Loaded(7);
    let _ = rng.below(3);
}
