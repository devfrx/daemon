//! The randomness port, and the list of who consumes it (§2.2).
//!
//! Every source of randomness is a point from which a trace can diverge. So the port
//! exists, the list of who draws from it is WRITTEN DOWN, and it stays short by choice.
//! Two decisions keep it short, and both were taken by declining randomness rather than by
//! injecting it:
//!
//! | What could have been random     | What it is instead                    | Why                                                                     |
//! |---------------------------------|---------------------------------------|-------------------------------------------------------------------------|
//! | identity of a run and of a step | PROGRESSIVE, assigned by the journal  | the journal already orders them; a drawn identifier is a draw to reproduce for nothing |
//! | the wait between two retries    | NO jitter                             | jitter fights contention between MANY clients, and here the client is ONE |
//!
//! ⛔ **Consumers in the kernel's decision logic: NONE.** Declaring the list empty is
//! information — it says the logic is reproducible without a seed at all. A generic port
//! carried "because it will be needed later" is the opposite: it says nothing about what
//! actually draws, and every later reader has to re-derive the answer. Randomness serves
//! `simulator` — exploring interleavings, injecting faults — not the logic.
//!
//! The only consumer inside this crate is the EXECUTOR, and it RECEIVES an `Rng` instead of
//! owning one: it draws to choose the order in which it polls. That is substrate, not
//! decision logic, which is why the list above stays empty rather than gaining a first row.

/// A source of raw draws. This is the whole port: one method, so that everything derived
/// from it is derived in ONE place — see `RngExt`.
pub trait Rng {
    fn next_u64(&mut self) -> u64;
}

/// The reduction. It lives on a SEPARATE trait for exactly one reason: so that no
/// implementation can reduce differently.
///
/// ⛔ A default method on `Rng` would have been an INTENTION, not a rule. It can be
/// overridden, and two implementations that reduce differently produce different traces
/// from the same seed with nothing on the surface to show for it — the invisible divergence
/// this sub-project exists to remove. Here the blanket impl below already covers every
/// `Rng`, so a hand-written `impl RngExt for MyRng { fn below(..) }` collides with it and
/// does NOT compile (E0119, conflicting implementations).
///
/// The negative test is `tests/compile_fail/override_below.rs`. It trips as an `error` and
/// not as a `mismatch`: defeat the rule by deleting the blanket impl and the case starts
/// COMPILING, which trybuild reports outright instead of through its oracle. Gotcha #42.
///
/// ⚠️ Declared costs, and there are two:
///
/// - callers import two names — `use kernel::rng::{Rng, RngExt};` — to write `rng.below(n)`;
/// - the guard forbids ANY hand-written `impl RngExt`, not only one that overrides `below`.
///   That is broader than the rule it enforces, and it is the price of the mechanism.
///
/// ⛔ Declared residual, so that this comment does not promise more than it delivers: an
/// INHERENT `fn below` on a concrete type would shadow this one at call sites holding that
/// concrete type, and no mechanism in Rust forbids it — it cannot become a level 1 rule.
/// It does not reach the executor, which holds its source behind the trait and so resolves
/// through `RngExt` regardless. The guard covers the consumer that matters; it is not total.
pub trait RngExt: Rng {
    /// A draw reduced into `0..n`.
    ///
    /// ⚠️ MODULO CARRIES A BIAS for every bound that does not divide 2^64: the raw range is
    /// not a whole number of cycles, so the low residues get one preimage more than the
    /// high ones. ACCEPTED AND DECLARED — the purpose here is exploring interleavings, not
    /// statistical soundness. ⛔ A consumer that ever needs uniformity gets ITS OWN method
    /// in this module and does not reduce on its own: two reductions living in two places
    /// is the thing the trait split has just paid to prevent.
    ///
    /// Returns 0 for `n == 0` rather than panicking. The callers are index choices over a
    /// collection, and an empty collection has no index; a panic would sit in the
    /// executor's hot path for a case the executor already excludes.
    ///
    /// ⛔ The degenerate bound still DRAWS — one call, one draw, whatever `n` is. Returning
    /// early would make the number of draws depend on the SIZE of the collection being
    /// indexed, so two runs of one seed differing by a single empty step would diverge from
    /// that point on. Pinned by `every_call_to_below_consumes_exactly_one_draw`.
    fn below(&mut self, n: u64) -> u64 {
        let raw = self.next_u64();
        if n == 0 { 0 } else { raw % n }
    }
}

/// ⛔ THE BLANKET IMPL IS THE GUARD, not a convenience: it is what makes `below` final.
///
/// `?Sized` so that an erased `&mut dyn Rng` receives it as well — the executor may well
/// hold its source that way, and dropping the bound would still compile everything else in
/// this crate. Checked by `below_is_reachable_through_a_trait_object`.
impl<R: Rng + ?Sized> RngExt for R {}
