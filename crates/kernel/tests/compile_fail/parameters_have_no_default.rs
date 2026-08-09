// The rule of §2.8.2, number 2: the kernel cannot name a file, a key, or a DEFAULT — none
// of the three is expressible inside it. `Parameters::default()` names the third: the value
// would be CHOSEN in the kernel instead of being delivered to it, which is the whole of what
// ADR-0034 removes. Here is the rule firing.
//
// ⛔ The mechanism is the ABSENCE of an `impl Default for Parameters`, and this case is what
// keeps that absence from being a comment. Nothing else in the gate would see the impl
// appear: `gate-attributes.sh` reads attributes, `gate-deps.sh` reads the dependency graph,
// `gate-no-os.sh` builds for a target without an OS, `check-docs.sh` does not read code at
// all — and `cargo build` compiles a `Default` impl happily, because it is valid Rust.
//
// ⛔ And it trips as an `error`, not as a `mismatch`: the day somebody writes the impl this
// case starts COMPILING, and trybuild says so outright — "Expected test case to fail to
// compile, but it succeeded" — instead of noticing through its oracle. A bulk regeneration
// of the `.stderr` files therefore cannot disarm it. Gotcha #42.
//
// ⚠️ What it does NOT cover, declared here rather than discovered later: a fallback written
// inside `Parameters::new` is a default too, and §2.8.4 says the compiler cannot forbid it.
// That half is held at level 2, by `tests/parameters_delivered.rs`.
//
// ⚠️ AND THE ORACLE NEXT DOOR WILL GO `mismatch` THE DAY A SECOND PARAMETER IS ADDED —
// measured, not feared: rustc closes this error with a note quoting the signature of
// `Parameters::new` verbatim, so §2.8.5's promised friction reaches the `.stderr` too. That
// regeneration is LEGITIMATE and it disarms nothing, because the rule trips as `error` and
// never through the oracle. Regenerate by the documented route — delete the stale `.stderr`,
// re-run, `diff -u` the old against the `wip/` one, move by hand — and NEVER with
// `TRYBUILD=overwrite`, which would take the other nine oracles with it. Gotcha #25.
//
// ⛔ Names `kernel::` and declares no attributes of its own — gotcha #39.

use kernel::parameters::Parameters;

fn main() {
    let _parameters = Parameters::default();
}
