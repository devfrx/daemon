//! The seeded generator, and the guard that spike SP-5 paid for (gotcha #10).

use kernel::rng::{Rng, RngExt};
use simulator::rng::SeededRng;

#[test]
fn the_same_seed_gives_the_same_sequence() {
    let mut a = SeededRng::new(20_260_806);
    let mut b = SeededRng::new(20_260_806);
    let left: [u64; 8] = core::array::from_fn(|_| a.next_u64());
    let right: [u64; 8] = core::array::from_fn(|_| b.next_u64());
    assert_eq!(left, right);
}

#[test]
fn a_different_seed_gives_a_different_sequence() {
    let mut a = SeededRng::new(20_260_806);
    let mut b = SeededRng::new(20_260_807);
    assert_ne!(a.next_u64(), b.next_u64());
}

#[test]
fn seed_zero_does_not_produce_a_dead_generator() {
    // xorshift stays stuck on zero: without the guard on the initial state, certain
    // seeds produce an empty trace and the spike SEEMS to pass. Gotcha #10.
    let mut rng = SeededRng::new(0);
    let drawn: [u64; 4] = core::array::from_fn(|_| rng.next_u64());
    assert!(
        drawn.iter().all(|&value| value != 0),
        "dead generator: {drawn:?}"
    );
}

#[test]
fn the_one_seed_that_scrambles_to_zero_does_not_produce_a_dead_generator() {
    // ⛔ THE NON-VACUITY OF THE ZERO GUARD, and the test above does NOT provide it: seed 0
    // scrambles to 1 all by itself, so it never reaches the guard and would pass with the
    // guard deleted. The multiplier is odd, hence invertible modulo 2^64, so
    // `seed -> seed * M + 1` is a BIJECTION on the 2^64 seeds: exactly one of them maps to
    // zero, and this is it. Gotcha #10 has measure one here, but it is not measure zero.
    let mut rescued = SeededRng::new(4_568_919_932_995_229_531);
    let drawn: [u64; 4] = core::array::from_fn(|_| rescued.next_u64());
    assert!(
        drawn.iter().all(|&value| value != 0),
        "dead generator: {drawn:?}"
    );

    // ⛔ And that the GUARD is what rescued it, not luck — otherwise this probe injects a
    // fault where the code never arrives, which is gotcha #17 in the shape that looks like
    // a success. The state is private, so it is observed through its effect: the guard
    // replaces zero with 1, and 1 is also where seed 0 lands by ordinary scrambling
    // (`0 * M + 1 == 1`). The two sequences must therefore be IDENTICAL. Were the seed
    // above merely an ordinary one, the guard would not fire, the state would be some
    // other value, and this equality would not hold.
    //
    // ⚠️ Declared consequence: those two seeds COLLIDE — one pair out of 2^64, and any
    // other replacement value would merely move the pair. Changing the guard's replacement
    // value turns this red on purpose: it changes that seed's trace.
    let mut ordinary = SeededRng::new(0);
    let same: [u64; 4] = core::array::from_fn(|_| ordinary.next_u64());
    assert_eq!(drawn, same);
}

#[test]
fn below_stays_inside_the_bound() {
    let mut rng = SeededRng::new(7);
    for _ in 0..1_000 {
        assert!(rng.below(5) < 5);
    }
}

#[test]
fn below_zero_is_zero_and_does_not_panic() {
    // The callers are index choices over a collection, and an empty collection has no
    // index. A panic here would sit in the executor's hot path for a case the executor
    // already excludes.
    let mut rng = SeededRng::new(7);
    assert_eq!(rng.below(0), 0);
}

#[test]
fn every_call_to_below_consumes_exactly_one_draw() {
    // Including the degenerate bound. If `below(0)` returned early without drawing, the
    // number of draws would depend on the SIZE of the collection being indexed, and two
    // runs of the same seed differing only by one empty step would diverge from there on.
    // That is the invisible divergence this sub-project exists to remove, so the rule is
    // the uniform one — one call, one draw — and it is pinned rather than described.
    let mut through_below = SeededRng::new(11);
    let mut raw = SeededRng::new(11);
    through_below.below(0);
    through_below.below(3);
    let _ = raw.next_u64();
    let _ = raw.next_u64();
    assert_eq!(through_below.next_u64(), raw.next_u64());
}

#[test]
fn below_is_the_shared_modulo_reduction_bias_included() {
    // ⛔ The reduction DEFINES the trace: change it and every draw of every campaign moves.
    // So it is pinned with frozen literals rather than by re-deriving `%` in the assertion,
    // which would only restate the code. Same species of oracle as the journal's bytes.
    //
    // `SeededRng` cannot serve here — its raw values are opaque, so through it `below` is
    // observable only as "inside the bound". `ScriptedRng` supplies them by hand, and,
    // implementing `Rng` and nothing else, it also shows that `below` arrives for free.
    let mut rng = ScriptedRng::new(&[u64::MAX, 0, u64::MAX, 1_000_000_007]);
    assert_eq!(rng.below(10), 5);

    // ⚠️ THE DECLARED BIAS, in two draws. The raw range `[0, 2^64)` is not a whole number
    // of cycles of 3 — its first value and its LAST both reduce to 0 — so residue 0 has one
    // preimage more than 1 and 2 do. Accepted: `below` explores interleavings, it does not
    // sample uniformly. A consumer that ever needs uniformity gets its own method in
    // `kernel::rng`; this test is what makes changing THIS one loud instead of silent.
    assert_eq!(rng.below(3), 0);
    assert_eq!(rng.below(3), 0);

    assert_eq!(rng.below(1_000), 7);
}

#[test]
fn below_is_reachable_through_a_trait_object() {
    // The executor receives an `Rng`; if it receives it erased, the `?Sized` on the blanket
    // impl is the only thing keeping `below` reachable — and dropping it would still
    // compile everything else here. Checked, therefore, instead of asserted in a comment.
    let mut seeded = SeededRng::new(3);
    let erased: &mut dyn Rng = &mut seeded;
    assert!(erased.below(4) < 4);
}

/// An `Rng` whose raw values are chosen by hand, so that the reduction can be observed on
/// values `SeededRng` cannot be asked to produce. It implements `Rng` and NOTHING else:
/// `below` reaches it through the blanket impl.
struct ScriptedRng {
    values: &'static [u64],
    next: usize,
}

impl ScriptedRng {
    fn new(values: &'static [u64]) -> Self {
        ScriptedRng { values, next: 0 }
    }
}

impl Rng for ScriptedRng {
    /// Running off the end of the script panics, and that is wanted: a test that draws more
    /// than it scripted is asserting on values nobody chose.
    fn next_u64(&mut self) -> u64 {
        let value = self.values[self.next];
        self.next += 1;
        value
    }
}
