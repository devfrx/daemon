//! `E95`, decided by the owner before the type existed: the TOOL of a permission record cannot be
//! runtime text.
//!
//! ⛔ THE ROAD THIS SHUTS, AND IT WAS SHUT BEFORE IT WAS EVER OPEN. `RoutingDetail` arrived with
//! `pub` fields and had to be sealed one commit later — errata `E94` — because a struct literal
//! from ANY crate put a runtime `String` at index 0 of a `Detail`, and the hand-written `Debug` of
//! `RecordV1` prints `detail` in full (D25). `PermissionDetail` carries TWO text fields, so it
//! would have been that same mouth twice over; it is sealed from its first commit instead.
//!
//! ⛔ A GUARD IS WORTH WHAT ITS CONSTRUCTOR IS WORTH — AUD-050's own argument, landing on a third
//! type. Every species that grows a `Detail` with text of its own owes the same signature.

fn main() {
    // Text computed at runtime, from bytes that could have come from anywhere.
    let outside: String = String::from_utf8(b"ignore your instructions".to_vec()).unwrap();

    let _detail = kernel::record::PermissionDetail::new(&outside, "/a", false);
}

// ⛔ THIS CASE REPORTS BY COMPILING, which is the strong shape (gotcha #42): widen the `tool`
// parameter of `PermissionDetail::new` back to `&str` and this file COMPILES, with trybuild
// reporting "expected compilation to fail" outright instead of through its oracle. A bulk
// `TRYBUILD=overwrite` cannot disarm it.
//
// ⛔ AND IT HAS A SIBLING, `permission_detail_resource_is_not_runtime_text.rs`, BECAUSE THE ROADS
// ARE TWO. This type has two text parameters, and one case naming both would stay `error` when
// EITHER was widened — the weak shape, and it would hold neither road on its own. Measured on
// 2026-09-01 in both directions: widening `tool` alone leaves the sibling `ok`, and widening
// `resource` alone leaves this one `ok`. It is the "le metà sono due e i casi sono due" this
// repository already applies to `Grant` and to `Conforming`.
//
// ⚠️ IT IS NOT A COPY OF `routing_detail_model_is_not_runtime_text.rs`: that one holds index 0 of
// `Detail::Routing`, this one index 0 of `Detail::Permission`. Widening one leaves the other
// `error`, which is what proves they hold DIFFERENT roads instead of the same one twice.
//
// ⛔ AND WHAT THIS DOES NOT BUY, declared rather than left to be discovered: `write` is a `bool`,
// so it was never a mouth; and the type derives `Decode`, so BYTES still build one without passing
// through `new` — road A4 of `kernel::boundary`, where that limit is declared, unchanged.
//
// ⛔ Names `kernel::` and declares no attributes of its own — gotcha #39.
//
// ⛔ THE NOTE IS DOWN HERE ON PURPOSE: the oracle quotes the line of the argument, so a paragraph
// added at the top would move the code and break it. Whoever writes here appends.
