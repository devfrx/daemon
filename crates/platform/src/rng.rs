//! The real `rng`, and IN PRODUCTION IT IS NOT RANDOM AT ALL.

use kernel::rng::Rng;

/// The `Rng` port in production: the draws are 0, 1, 2, … and nothing about them is random.
///
/// # Why the production randomness is fake
///
/// The only consumer of this port is the EXECUTOR, which draws to choose the order in which it
/// polls the runnable activities. That draw exists to EXPLORE INTERLEAVINGS, and exploring is
/// what a campaign does — under `simulator`, with `SeededRng`, one seed per run. In production
/// nobody explores anything: the interleaving is simply fixed, once. Saying that with a type is
/// more honest than shipping a real generator whose randomness no one uses, and it removes a
/// source of divergence from the shipped binary rather than merely leaving it unread.
///
/// ⚠️ DECLARED CONSEQUENCE, and it is a property rather than a defect: two production runs over
/// the same activities poll them IN THE SAME ORDER. A defect that depends on the interleaving
/// therefore reproduces instead of appearing once and vanishing — and the interleavings this
/// order does not visit are the campaign's job, not the daemon's.
///
/// # What the sequence buys, stated exactly
///
/// ⛔ The reduction is `below(n) = raw % n`, so 0, 1, 2, … walks the indices `0..n` IN TURN
/// ONLY FOR AS LONG AS `n` STAYS THE SAME BETWEEN CALLS. The moment the bound changes — and it
/// does: `n` is the number of runnable activities, and a Fisher-Yates shuffle shrinks it at
/// every step — the walk is no longer a clean cycle, and the same index can come up twice in a
/// row. That limit is pinned by `a_bound_that_changes_breaks_the_round_robin` below, so this
/// paragraph cannot quietly grow into a bigger claim than the code makes.
pub struct SequentialRng(u64);

impl SequentialRng {
    pub const fn new() -> Self {
        SequentialRng(0)
    }
}

impl Default for SequentialRng {
    fn default() -> Self {
        SequentialRng::new()
    }
}

impl Rng for SequentialRng {
    fn next_u64(&mut self) -> u64 {
        let value = self.0;
        // `wrapping_add` and not `+`: at the top of the range a plain add PANICS in a debug
        // build, and a panic in the executor's hot path is a failure mode nobody would come
        // looking for here. Wrapping starts the sequence over, which is the harmless answer.
        self.0 = self.0.wrapping_add(1);
        value
    }
}

// ⚠️ A UNIT TEST MODULE IN `src/`, WHERE THIS REPOSITORY OTHERWISE PUTS TESTS IN `tests/`, and
// the deviation is declared rather than left to be noticed. HOW MANY THERE ARE, AND WHERE, COMES
// FROM THE COMMAND AND NOT FROM THIS LINE — `grep -rn --include='*.rs' 'mod tests {' crates/*/src/`.
// What is worth reading is the REASON, which differs from one to the next and which each of them
// states for itself. Here it IS a choice, and the reason is
// `the_counter_wraps_instead_of_overflowing`: it builds `SequentialRng(u64::MAX)` directly, and
// the field is private, so from an integration test — a crate of its own — that value is
// unreachable without either 2^64 draws or a constructor existing solely to be tested. The
// other tests need nothing private and would sit perfectly well in `tests/`; they stay for the
// sake of one home per type rather than two.
//
// ⛔ RECALL OF 2026-08-28, FINDING AUD-060 — THIS SAID "It is ONE OF TWO in the workspace" AND
// NAMED THE OTHER. THE FIGURE IS REMOVED RATHER THAN REALIGNED TO THREE, and so is the one in
// `crates/daemon/src/main.rs`, which said THREE and was RIGHT: the same count sat in two houses,
// the task that added the third module (Milestone 5, `Arbiter::ask_back`) updated that one and
// not this one, and `crates/kernel/src/arbiter/mod.rs` names THIS file as its precedent — so the
// house the other two point at was the one telling the wrong number. Correcting only the false
// house would leave the count in two places, which is what made them diverge; gotcha #68.
#[cfg(test)]
mod tests {
    use super::*;
    use kernel::rng::RngExt;

    #[test]
    fn the_draws_are_zero_one_two_and_so_on() {
        let mut rng = SequentialRng::new();
        let drawn: [u64; 5] = core::array::from_fn(|_| rng.next_u64());
        assert_eq!(drawn, [0, 1, 2, 3, 4]);
    }

    #[test]
    fn a_fresh_generator_starts_from_zero() {
        // `Default` and `new` are the same generator, not two. Two entry points that drifted
        // apart would give two different production orders depending on which was called.
        assert_eq!(SequentialRng::default().next_u64(), 0);
        assert_eq!(SequentialRng::new().next_u64(), 0);
    }

    #[test]
    fn below_walks_the_indices_in_turn_while_the_bound_stays_the_same() {
        // The property the type is FOR, and the one the doc comment claims: with `n` held
        // still, every index comes up before any comes up again.
        let mut rng = SequentialRng::new();
        let drawn: [u64; 7] = core::array::from_fn(|_| rng.below(3));
        assert_eq!(drawn, [0, 1, 2, 0, 1, 2, 0]);
    }

    #[test]
    fn a_bound_that_changes_breaks_the_round_robin() {
        // ⚠️ THE DECLARED LIMIT, pinned rather than described. `below` is `raw % n`, so the
        // clean walk survives only a constant bound. Here the bound changes and index 0 comes
        // up TWICE IN A ROW — which a constant bound above 1 can never produce.
        let mut rng = SequentialRng::new();
        assert_eq!(rng.below(4), 0); // raw 0
        assert_eq!(rng.below(4), 1); // raw 1
        assert_eq!(rng.below(2), 0); // raw 2, and the bound has changed
        assert_eq!(rng.below(3), 0); // raw 3, index 0 again, back to back
    }

    #[test]
    fn the_counter_wraps_instead_of_overflowing() {
        // The behaviour declared on `next_u64`, at the only place it differs from a plain add.
        // Unreachable in any real run — 2^64 draws — and pinned because the alternative is a
        // debug-build panic rather than a wrong number.
        let mut rng = SequentialRng(u64::MAX);
        assert_eq!(rng.next_u64(), u64::MAX);
        assert_eq!(rng.next_u64(), 0);
        assert_eq!(rng.next_u64(), 1);
    }
}
