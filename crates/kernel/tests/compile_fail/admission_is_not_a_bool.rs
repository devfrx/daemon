// `V4`, second half, and the third of its three shortcuts: there is no conversion from
// `Admission` to `bool`.
//
// ⛔ THE DOORWAY IS `Into<bool>`, AND THE CALL BELOW STANDS IN IT. `impl From<Admission> for bool`
// reaches `.into()` through the standard blanket impl, and a hand-written
// `impl Into<bool> for Admission` reaches it directly: either one makes this file compile, where
// `bool::from(outcome)` would see only the first. The siblings are `admission_has_no_is_granted.rs`
// and `admission_has_no_is_ok.rs`, one road each. Audit of 2026-09-30, AUD-695.
//
// ⚠️ IT NEEDS NO ARBITER, DELIBERATELY, for the reason `admission_has_no_is_ok.rs` gives: the
// conversion is a property of the TYPE.
fn shortcut(outcome: kernel::arbiter::Admission) -> bool {
    outcome.into()
}

fn main() {
    let _ = shortcut;
}

// ⛔ ON THE TWO ROADS BY VALUE IT REPORTS BY COMPILING, which is the strong shape (gotcha #42),
// and a bulk `TRYBUILD=overwrite` cannot disarm it. Measured on 2026-10-02 in both directions:
// with `impl From<Admission> for bool` this case is `error`, with a hand-written
// `impl Into<bool> for Admission` it is `error` too, and every other case stays `ok` under both;
// without either, `ok`.
//
// ⚠️ WHAT IT DOES NOT SEE, declared rather than left to be discovered: a FALLIBLE conversion,
// `impl TryFrom<Admission> for bool`, goes through another trait, and with it this case stays
// `ok` — measured the same day. That road is closed by the review, not by the compiler.
// ⚠️ AND A CONVERSION BY REFERENCE, `impl From<&Admission> for bool`, is seen only through the
// ORACLE: the call takes `outcome` by value, so rustc answers that `Admission: Into<_>` is not
// satisfied and appends `help: consider borrowing here`, and the case goes `mismatch` — the shape
// a bulk regeneration blesses in silence. Measured on 2026-10-13. ⚠️ RECALL OF 2026-10-13 --
// audit of 2026-09-30, AUD-2235.
//
// ⛔ Names `kernel::` and declares no attributes of its own — gotcha #39.
//
// ⛔ THE NOTE IS DOWN HERE ON PURPOSE: the oracle quotes the line of the call, so a paragraph added
// at the top would move the code and break it. Whoever writes here appends.
