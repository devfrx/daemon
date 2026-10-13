// The other half of the block B row "starting a worker <- a grant": the token cannot be
// FORGED. `Grant`'s fields, `id` and `issuer`, are private and its module is `kernel::arbiter`,
// so from out here there is no way to build one. ⚠️ THE ERROR CARRIES NO CODE: the oracle
// beside this file reads, bare, "cannot construct `Grant` with struct literal syntax due to
// private fields" -- `E0422`/`E0423` was a guess written before the measure (gotcha #15).
// ⚠️ THE DECLARED LIMIT: trybuild compiles its cases as SEPARATE CRATES, so what is proved is
// the direction FROM OUTSIDE. Nothing here stops a `pub(crate)` constructor tomorrow -- that would
// be a new catalogue row, and the catalogue is spec. Registered in §12 of the milestone 5 design.
fn main() {
    let _forged = kernel::arbiter::Grant {};
}

// ⚠️ RECALL OF 2026-10-13 -- audit of 2026-09-30, AUD-1289, AUD-1290: `Grant` has two private
// fields since milestone 6 task 1, and `issuer` is the whole of the guard of `Arbiter::release`.
// The note sits down here because the oracle next door quotes the line of the literal.
