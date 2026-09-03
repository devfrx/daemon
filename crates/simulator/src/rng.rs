//! The seeded generator: this crate's implementation of the `Rng` port.

use kernel::rng::Rng;

/// xorshift64, from a scrambled seed. Small and cheap, and enough for what randomness is
/// for here — exploring interleavings and injecting faults, not sampling.
pub struct SeededRng(u64);

impl SeededRng {
    pub const fn new(seed: u64) -> Self {
        let scrambled = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        // ⛔ xorshift STAYS STUCK ON ZERO: zero is a fixed point of the three shifts, so a
        // generator that starts there returns zero forever. A campaign that explores
        // nothing LOOKS like a campaign that found nothing. Gotcha #10, measured in SP-5.
        //
        // ⚠️ HOW OFTEN IT FIRES — worked out, not assumed, because a guard whose comment
        // claims more than it does is worse than no comment. The multiplier is odd, hence
        // invertible modulo 2^64, so `seed -> seed * M + 1` is a BIJECTION on the 2^64
        // seeds: EXACTLY ONE of them maps to zero, and it is 4_568_919_932_995_229_531.
        // Seed 0 is NOT that one — it scrambles to 1 on its own, so it never arrives here.
        //
        // So: reachable, and its reach is one seed out of 2^64. It stays because it is
        // cheap insurance on a state known to be fatal, and it is not claimed to be more
        // than that. The probe that actually exercises it — and shows that it fires, which
        // `seed_zero_does_not_produce_a_dead_generator` does not — is
        // `the_one_seed_that_scrambles_to_zero_does_not_produce_a_dead_generator`.
        SeededRng(if scrambled == 0 { 1 } else { scrambled })
    }
}

impl Rng for SeededRng {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}
