// The other half of the block B row "starting a worker <- a grant": the token cannot be
// FORGED. `Grant`'s fields, `id` and `issuer`, are private and its module is `kernel::arbiter`,
// so from out here there is no way to build one. ⚠️ THE ERROR CARRIES NO CODE: the oracle
// beside this file reads, bare, "cannot construct `Grant` with struct literal syntax due to
// private fields" -- `E0422`/`E0423` was a guess written before the measure (gotcha #15).
// ⚠️ THE DECLARED LIMIT: trybuild compiles its cases as SEPARATE CRATES, so what is proved is
// the direction FROM OUTSIDE. A `pub(crate)` constructor is seen through the oracle alone, or not
// at all -- measured, at the foot of this file. Registered in §12 of the milestone 5 design.
fn main() {
    let _forged = kernel::arbiter::Grant {};
}

// ⚠️ RECALL OF 2026-10-13 -- audit of 2026-09-30, AUD-1289, AUD-1290: `Grant` has two private
// fields since milestone 6 task 1, and `issuer` is the whole of the guard of `Arbiter::release`.
// The note sits down here because the oracle next door quotes the line of the literal.
//
// ⚠️ WHAT A `pub(crate)` CONSTRUCTOR DOES TO THIS CASE, measured on 2026-10-14 as it was for
// `conforming_has_no_constructor.rs`: with `pub(crate) fn new() -> Self` in `impl Grant`, rustc
// appends "help: you might have meant to use the `new` associated function" and the case goes
// `mismatch`, the shape a bulk regeneration blesses in silence (gotcha #42); with a `pub(crate)`
// free function of `kernel::arbiter` that returns one, the case stays `ok`. Stopping either for
// good would be a new catalogue row, and the catalogue is spec. Read every regeneration of the
// oracle next door by hand (gotcha #25). ⚠️ RECALL OF 2026-10-14 -- audit of 2026-09-30.
