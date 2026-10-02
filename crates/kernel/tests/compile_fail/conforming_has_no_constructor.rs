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

// ⚠️ THE DECLARED LIMIT, and it is WIDER than the one `grant_has_no_constructor.rs` declares —
// measured on 2026-10-02 rather than copied from that twin. trybuild compiles its cases as
// SEPARATE CRATES, so what is proved is the direction FROM OUTSIDE; and from outside, the two
// roads that would really forge the token are seen only through the ORACLE, never by compiling.
// With the three fields `pub` the empty literal fails for missing fields, `E0063`; with a `pub`
// function in `impl Conforming` that hands one out — measured with `new` and with `forge` — it
// fails as before and rustc appends "help: you might have meant to use the `..` associated
// function". Both are `mismatch`, the shape a bulk regeneration blesses in silence (gotcha #42).
// Unlike `Grant`, whose `id` is a `GrantId` no other crate can name, these fields are a
// `&'static str`, a `u32` and a `bool`: opened, they forge the token for real. A `pub` FREE
// function of the module that returns one is not seen at all: the case stays `ok`. And nothing here stops a `pub(crate)` constructor —
// that would be a new catalogue row, and the catalogue is spec. Read every regeneration of the
// oracle next door by hand, never with `TRYBUILD=overwrite` (gotcha #25). Audit of 2026-09-30,
// AUD-696.
//
// ⛔ THE NOTE IS DOWN HERE ON PURPOSE: the oracle quotes the line of the literal, so a paragraph
// added at the top would move the code and break it. Whoever writes here appends.
