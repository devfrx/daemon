// The other half of the block B row "starting a worker <- a grant": the token cannot be
// FORGED. `Grant`'s only field is private and its module is `kernel::arbiter`, so from out
// here there is no way to build one. ⚠️ THE ERROR CARRIES NO CODE: the oracle beside this
// file reads, bare, "cannot construct `Grant` with struct literal syntax due to private
// fields" -- `E0422`/`E0423` was a guess written before the measure (gotcha #15).
// ⚠️ THE DECLARED LIMIT: trybuild compiles its cases as SEPARATE CRATES, so what is proved
// is the direction FROM OUTSIDE. Nothing here stops a `pub(crate)` constructor tomorrow --
// that would be a new catalogue row, and the catalogue is spec. Registered in §12.
fn main() {
    let _forged = kernel::arbiter::Grant {};
}
