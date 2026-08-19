//! What the compiler cannot hold about the GRANT CYCLE. Two things: the LEGAL side of
//! `I2 · §5.3`, and the positive side of `V4` -- that `Admission` truly answers THREE ways.
//!
//! ⚠️ THE PROBES LIVE HERE AND NOT IN `arbiter_resource.rs`, and the split is by subject
//! rather than by convenience: that file holds the vocabulary of the RESOURCE -- `Mib`, the
//! three lanes, the grace time -- and neither `Activity` nor `Admission` is a resource.
//! `Activity` is what a held grant is DOING; `Admission` is the answer a request gets.
//!
//! ⚠️ AND THIS FILE ARRIVES ONE TASK EARLY. The plan's file table assigns it to tasks 5-7;
//! it is born at task 4 because two rules would otherwise be held in only one direction --
//! `tests/compile_fail/revoking_a_non_preemptible_grant.rs` proves only that the ILLEGAL
//! state cannot be spelled, and `tests/compile_fail/admission_is_not_two_ways.rs` proves
//! only that a two-armed match does not compile -- and a rule proved in one direction only
//! is not admissible (§7.1.1 rule 3). ⛔ WHOEVER EXECUTES TASK 5 ADDS TO THIS FILE: it
//! already exists, so its step "create" is a MODIFY, and this module comment is merged
//! rather than overwritten.
//!
//! ⚠️ TWO DIFFERENT RULES ABOUT COMPARISON LIVE HERE, DELIBERATELY. R2 -- probes MATCH and
//! never compare -- is about `Admission`, which carries a `Grant` and therefore has neither
//! `Debug` nor `PartialEq`: the second probe below matches it and compares only the `Mib`
//! values it extracts. `Activity` carries no grant and derives both, so `assert_ne!` on it
//! directly is fine and is what the first probe uses.

use kernel::arbiter::{Activity, Admission, Mib, PreemptibleState};
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

/// The direction the compile-fail case CANNOT prove: `Admission` really is distinguishable
/// THREE ways, not two -- a `match` naming all three variants compiles from outside the
/// crate, which is exactly the shape `admission_is_not_two_ways.rs` proves a two-armed
/// match does NOT have.
///
/// ⛔ WITHOUT THIS, `V4`'s positive direction WOULD BE HELD BY A MUTATION -- add the third
/// arm to the negative case's `match` and watch it start compiling -- and a mutation
/// disappears when it is reverted. A direction held by something that does not stay is not
/// held (§7.1.1 rule 3).
///
/// ⚠️ ONLY `Refused` IS CONSTRUCTIBLE FROM OUT HERE. `Granted` carries a `Grant`, which has
/// no public constructor (`grant_has_no_constructor.rs`); `Queued` carries a `TicketId`,
/// whose only field is private too. The other two arms therefore bind and discard -- their
/// weight is that the match TYPE-CHECKS against all three shapes. R2-compliant: this
/// matches `Admission` instead of comparing it, exactly as the module comment requires.
///
/// 📌 THIS PROBE'S FORCE IS AT COMPILE TIME ONLY, and it is worth saying plainly instead of
/// implying otherwise. Nothing here computes anything -- `admit` does not exist until Task
/// 5 -- so the `Mib` values only ever echo what this test itself passed in. Removing or
/// renaming a variant of `Admission` makes this REFUSE TO COMPILE; no mutation of the
/// production code can turn the `assert_eq!` calls below false while the type still has
/// three variants named the way this file names them.
#[test]
fn an_admission_is_distinguishable_three_ways() {
    let outcome = Admission::Refused {
        asked: Mib::new(512),
        ceiling: Mib::new(256),
    };

    match outcome {
        Admission::Granted(_) => panic!("this probe never constructs a Granted admission"),
        Admission::Queued(_) => panic!("this probe never constructs a Queued admission"),
        Admission::Refused { asked, ceiling } => {
            assert_eq!(asked, Mib::new(512));
            assert_eq!(ceiling, Mib::new(256));
        }
    }
}
