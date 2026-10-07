//! Block B row `Q13`, second half: the token cannot be FORGED -- this case holds the road "open
//! fields" in the STRONG shape. The notes are at the foot, on purpose.
fn main() {
    let _forged = kernel::gateway::Conforming { model: "a-model", evaluated: 1, degraded: false };
}

// ⛔ WHAT IT HOLDS: the three fields of `Conforming` are private, so a literal that names them
// all is refused -- `E0451`, one "private field" label per field. ⛔ AND WITH THE THREE FIELDS
// `pub` THIS CASE COMPILES, so trybuild answers `error` -- the shape no regeneration of any
// oracle can silence, because it does not go through an oracle at all (gotcha #42). That is
// what `conforming_has_no_constructor.rs` cannot become without losing the constructors: its
// empty literal turns `E0063` with the fields open, a `mismatch`.
//
// ⚠️ THE TWO CASES ARE COMPLEMENTARY AND NEITHER IS REDUNDANT, the shape of `trust_has_no_default.rs`
// beside its twin -- one case per road. This one is BLIND TO A CONSTRUCTOR: with a `pub fn new`
// in `impl Conforming` and the fields private, it stays `ok`, while the twin sees that road as a
// `mismatch` (rustc appends a `help:` to its oracle). Which is why it is ADDED and replaces
// nothing. A `pub` free function of the module, or a `pub(crate)` constructor, neither case sees:
// the limit is declared in the twin.
//
// ⛔ Names `kernel::` and declares no attributes of its own -- gotcha #39. Audit of 2026-09-30,
// AUD-696.
//
// ⛔ THE NOTE IS DOWN HERE ON PURPOSE: the oracle quotes the line of the literal, so a paragraph
// added at the top would move the code and break it. Whoever writes here appends.
