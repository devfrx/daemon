// `V4`, second half, and the second of its three shortcuts: `Admission` has no `is_ok()`.
//
// ⛔ A CASE OF ITS OWN AND NOT A SECOND LINE IN `admission_has_no_is_granted.rs`, BECAUSE A CASE
// NAMES ONE ROAD. With `is_ok` added that case stays `ok`, and one case naming both methods would
// stay red while EITHER was still missing — it would hold neither on its own. The design of
// milestone 5 forbids three shortcuts — `is_ok()`, `is_granted()` and a conversion to a boolean —
// so the cases are three: this one, `admission_has_no_is_granted.rs` and
// `admission_is_not_a_bool.rs`. Audit of 2026-09-30, AUD-695.
//
// ⚠️ IT NEEDS NO ARBITER, DELIBERATELY — the shape of `admission_is_not_two_ways.rs`: a shortcut
// is a property of the TYPE, so the case takes an `Admission` as an argument instead of obtaining
// one from `admit`, and a change to `Parameters::new` or to `admit` cannot make it fail for a
// reason that is not its own.
fn shortcut(outcome: kernel::arbiter::Admission) -> bool {
    outcome.is_ok()
}

fn main() {
    let _ = shortcut;
}

// ⛔ IT NAMES A METHOD THAT DOES NOT EXIST, ON PURPOSE, and it reports by COMPILING, which is the
// strong shape (gotcha #42): add `is_ok` to `Admission` and this file compiles, with trybuild
// reporting `error` outright instead of through its oracle. A bulk `TRYBUILD=overwrite` cannot
// disarm it. Measured on 2026-10-02 in both directions: with `fn is_ok(&self) -> bool` added this
// case is `error` and every other case stays `ok`; without it, `ok`.
//
// ⛔ Names `kernel::` and declares no attributes of its own — gotcha #39.
//
// ⛔ THE NOTE IS DOWN HERE ON PURPOSE: the oracle quotes the line of the call, so a paragraph added
// at the top would move the code and break it. Whoever writes here appends.
