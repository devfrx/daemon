//! What the compiler cannot hold about the GRANT CYCLE. Today one thing only: the LEGAL
//! side of `I2 · §5.3`.
//!
//! ⚠️ THE PROBE LIVES HERE AND NOT IN `arbiter_resource.rs`, and the split is by subject
//! rather than by convenience: that file holds the vocabulary of the RESOURCE -- `Mib`, the
//! three lanes, the grace time -- and `Activity` is not a resource. It is what a held grant
//! is DOING.
//!
//! ⚠️ AND THIS FILE ARRIVES ONE TASK EARLY. The plan's file table assigns it to tasks 5-7;
//! it is born at task 4 because `tests/compile_fail/revoking_a_non_preemptible_grant.rs`
//! proves only that the ILLEGAL state cannot be spelled, and a rule proved in one direction
//! only is not admissible (§7.1.1 rule 3). ⛔ WHOEVER EXECUTES TASK 5 ADDS TO THIS FILE: it
//! already exists, so its step "create" is a MODIFY, and this module comment is merged
//! rather than overwritten.
//!
//! ⚠️ `assert_ne!` IS USABLE HERE, and the line is worth writing because the opposite rule
//! is about to land in the same file. R2 -- probes MATCH and never compare -- is about
//! `Admission`, which carries a `Grant` and therefore has neither `Debug` nor `PartialEq`.
//! `Activity` carries no grant and derives both.

use kernel::arbiter::{Activity, PreemptibleState};
use kernel::time::Monotonic;

/// The direction the compile-fail case CANNOT prove: the revocation is constructible
/// exactly where §5.3 point 3 allows it.
///
/// ⛔ WITHOUT THIS, `I2 · §5.3` WOULD BE HELD IN ONE DIRECTION BY A CASE AND IN THE OTHER BY
/// A MUTATION -- and a mutation disappears when it is reverted. A direction held by
/// something that does not stay is not held (§7.1.1 rule 3). It is also what tells the
/// negative case apart from an accident: it proves the compiler refuses the NESTING, not
/// the syntax of `Revoking { deadline }`.
#[test]
fn a_revocation_is_constructible_on_the_preemptible_side() {
    let revoking = Activity::Preemptible(PreemptibleState::Revoking {
        deadline: Monotonic::from_millis(1_000),
    });

    assert_ne!(
        revoking,
        Activity::Preemptible(PreemptibleState::Running),
        "a revocation that compared equal to a running grant would make the state useless"
    );
    assert_ne!(
        revoking,
        Activity::NonPreemptible,
        "the two sides of the nesting must not collapse into one another"
    );
}
