//! The other half of block B row `Q13`: the token cannot be FORGED. `Conforming`'s fields are
//! private and its module is `kernel::gateway`, so from out here there is no way to build one.
//! ⛔ THE MINTER IS `resolve`, AND IT IS THE ONLY ONE — the same shape as `Arbiter::issue`
//! for `Grant` (§5.6), and for the same reason: a token whose producer lives INSIDE the crate
//! that defines it is not forgeable (§4.1 of the design).
//!
//! ⚠️ THE ERROR TEXT IS NOT GUESSED: `grant_has_no_constructor.stderr` shows that this shape of
//! error carries NO code — bare "cannot construct ... with struct literal syntax due to private
//! fields" — and that fact was itself a correction of a guess (gotcha #15). Measure it here too
//! rather than copying that file's oracle: the two types have a different number of fields, and
//! the `note:` line names them.
fn main() {
    let _forged = kernel::gateway::Conforming {};
}

// ⚠️ THE DECLARED LIMIT, and it is the same one `grant_has_no_constructor.rs` declares: trybuild
// compiles its cases as SEPARATE CRATES, so what is proved is the direction FROM OUTSIDE.
// Nothing here stops a `pub(crate)` constructor tomorrow — that would be a new catalogue row,
// and the catalogue is spec.
