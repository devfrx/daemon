// `V4`, second half, and the conversion to `bool` BY REFERENCE: there is no
// `impl From<&Admission> for bool` either.
//
// ⛔ A CASE OF ITS OWN, BECAUSE `admission_is_not_a_bool.rs` SEES THIS ROAD THROUGH ITS ORACLE
// ALONE: its call takes `outcome` by value, so with that impl it keeps failing, with
// `help: consider borrowing here` appended -- `mismatch`. The call below borrows, so the same impl
// makes THIS file compile. It goes through `.into()` for the reason its sibling gives: a
// hand-written `impl Into<bool> for &Admission` reaches it too, where `bool::from(&outcome)` would
// see only the first road. Audit of 2026-09-30, AUD-2141, AUD-2235.
//
// ⚠️ IT NEEDS NO ARBITER, DELIBERATELY, for the reason `admission_has_no_is_ok.rs` gives: the
// conversion is a property of the TYPE.
fn shortcut(outcome: kernel::arbiter::Admission) -> bool {
    (&outcome).into()
}

fn main() {
    let _ = shortcut;
}

// ⛔ ON THE TWO ROADS BY REFERENCE IT REPORTS BY COMPILING, which is the strong shape (gotcha
// #42), and a bulk `TRYBUILD=overwrite` cannot disarm it. Measured on 2026-10-14 in both
// directions: with `impl From<&Admission> for bool` this case is `error`, with a hand-written
// `impl Into<bool> for &Admission` it is `error` too; without either, `ok`. ⚠️ RECALL OF
// 2026-10-14 -- audit of 2026-09-30, AUD-2141, AUD-2235.
//
// ⛔ Names `kernel::` and declares no attributes of its own — gotcha #39.
//
// ⛔ THE NOTE IS DOWN HERE ON PURPOSE: the oracle quotes the line of the call, so a paragraph added
// at the top would move the code and break it. Whoever writes here appends.
