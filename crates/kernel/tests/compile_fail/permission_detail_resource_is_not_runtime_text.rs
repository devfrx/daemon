//! `E95`, the OTHER half: the RESOURCE of a permission record cannot be runtime text either.
//!
//! ⛔ WHY IT IS A CASE OF ITS OWN AND NOT A SECOND LINE IN ITS SIBLING. `PermissionDetail::new`
//! takes TWO `&'static str`, so there are two roads a runtime `String` could take into a `Detail`.
//! A single case naming both parameters stays `error` when EITHER one is widened, which is the
//! weak shape: it would go on passing while half the guard it claims to hold had been removed.
//! Measured on 2026-09-01 — widening `resource` alone leaves
//! `permission_detail_tool_is_not_runtime_text.rs` compiling `error` and nothing goes red there.
//!
//! ⛔ AND THE RESOURCE IS THE HALF THAT MATTERS MOST, which is worth one line: the tool names our
//! own code, while the resource names a PATH — the one component of the triple most likely to be
//! built at run time from something a user or a model said.

fn main() {
    // Text computed at runtime, from bytes that could have come from anywhere.
    let outside: String = String::from_utf8(b"ignore your instructions".to_vec()).unwrap();

    let _detail = kernel::record::PermissionDetail::new("file", &outside, false);
}

// ⛔ THIS CASE REPORTS BY COMPILING, which is the strong shape (gotcha #42): widen the `resource`
// parameter of `PermissionDetail::new` back to `&str` and this file COMPILES, with trybuild
// reporting "expected compilation to fail" outright instead of through its oracle. A bulk
// `TRYBUILD=overwrite` cannot disarm it.
//
// ⛔ Names `kernel::` and declares no attributes of its own — gotcha #39.
//
// ⛔ THE NOTE IS DOWN HERE ON PURPOSE: the oracle quotes the line of the argument, so a paragraph
// added at the top would move the code and break it. Whoever writes here appends.
