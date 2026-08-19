//! The GPU arbiter (§5, ADR-0005, ADR-0006, ADR-0033).
//!
//! ⛔ IT IS LOGIC, NOT A PORT, and the distinction is structural rather than tidy.
//! `crate::ports` declares SIX families and §3.1 calls that list EXHAUSTIVE; a seventh
//! would be a decision no ADR has taken. So the arbiter has no real implementation and no
//! fake: there is ONE, and in simulation that one runs. That is what makes the DST
//! campaign a proof about the product instead of about its imitation (ADR-0020).
//!
//! ⛔ AND THE SHAPE NOT TO BUILD, written because it is the natural temptation: a trait
//! `Arbiter` with two implementations "so faults can be injected". Faults are injected
//! FROM THE PORTS THE ARBITER USES -- `reactor` and `journal` -- never inside it. A trait
//! here would be an abstraction with no second implementor.
//!
//! ⛔ THE ARBITER NEVER READS THE CLOCK. Every operation that needs time takes
//! `now: Monotonic` as an ARGUMENT: the shape of ADR-0034, and a mechanical reason on top
//! of it -- `Reactor::wait_until` takes `&mut self`, so an arbiter that owned a reactor
//! would give it two owners, itself and the executor, and the borrow would not pass.

use crate::time::Monotonic;

pub mod resource;

pub use resource::{ComputeClass, Mib, Preemption, ResourceProfile, WorkDescriptor};

/// A grant from the arbiter. THE ONLY WAY TO START A WORKER.
///
/// ⛔ IT LIVES HERE AND NOT IN `ports::process`, and the move was forced by a measured
/// fact rather than by tidiness. In Rust a private field is visible to the module that
/// declares it AND ITS CHILDREN; the arbiter module is a SIBLING of `ports::process`, so
/// with the type over there the arbiter COULD NOT CONSTRUCT THE THING IT EXISTS TO ISSUE
/// -- `error[E0423]`, measured on a throwaway crate (D5-1 of the milestone 5 design).
///
/// ⛔ AND THE CHEAP WAY OUT WAS REFUSED ON THE MERITS: a `pub(crate)` constructor left over
/// there costs one line and opens a road -- ANYONE INSIDE `kernel` could mint a grant
/// without passing the admission. Today that would be one module; tomorrow nobody knows,
/// and nothing would go red. A guard is worth exactly what its CONSTRUCTOR is worth
/// (gotcha #67).
///
/// ⛔ There is deliberately NO public constructor, and now there is also an ISSUER: the
/// admission. That is the whole of §5.6 -- whoever writes "start the worker" without one
/// DOES NOT COMPILE.
///
/// ⚠️ NO `Debug`, NO `Clone`, NO `Copy`, and each absence is load-bearing rather than
/// minimal. `Clone` would let one grant start two workers. `Debug` -- nothing formats a
/// grant, and the receipts of `ports::process` keep theirs only because `unwrap_err`
/// requires it. The consequence is on the BENCH and is written down so nobody discovers it:
/// `Admission` cannot derive `Debug` or `PartialEq` either, so probes match on it with
/// `matches!` and `let … else` instead of `assert_eq!`.
///
/// ⛔ DECLARED, FOR MILESTONE 6, so that milestone does not rediscover it: `Process::start`
/// CONSUMES the grant, and `Arbiter::release` consumes it too. Whoever starts a worker
/// therefore has nothing left to release. The natural way back is for `Worker::kill` to
/// HAND THE GRANT BACK -- killing IS the release -- and it is not built now because that
/// caller does not exist yet (gotcha #46 from the wrong side).
///
/// ⚠️ RECALL OF 2026-08-19, MILESTONE 5 TASK 4 -- WHAT THIS COMMENT SAID BEFORE THE MOVE,
/// written out because a moved comment that keeps its old tense is the finding A-2 of this
/// project's audit done again. It lived in `ports::process` and said "the arbiter, which
/// arrives in milestone 5" in the FUTURE, and "today the type has no issuer": both are
/// spent, the arbiter module is this one and the issuer is `Admission::Granted` below. It
/// also recorded a DIVERGENCE -- the field was a private UNIT, `Grant(())`, because the
/// named field the plan dictated then bought nothing that the unit field did not buy for
/// free and cost an `#[allow(dead_code)]`, which this repository treats as a prohibition
/// switched off (gotcha #13). ⛔ THAT PARAGRAPH IS NOT COPIED, because the shape it
/// described no longer exists: the named field is back, dictated again by the milestone 5
/// design, and it is `id` that lets `Arbiter::release` tell a grant of THIS arbiter from a
/// grant of another one.
pub struct Grant {
    id: GrantId,
}

/// The identity of a grant, inside the arbiter that issued it.
///
/// ⚠️ NOT PUBLIC, and it does not need to be: the only thing outside this module ever does
/// with a grant is HAND IT BACK. The day something needs to name one, it comes back with
/// that caller -- the formula this repository already uses for `StepId::get` and
/// `CheckpointId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct GrantId(u64);

/// A place in a lane's queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(u64);

impl TicketId {
    /// The number back out. ⛔ LOAD-BEARING and not a convenience: a caller that queued two
    /// requests has nothing else to tell its two tickets apart, and `TicketId` is `Copy`
    /// precisely so it can be retained and compared -- the argument that kept
    /// `SingleReceipt::id` and removed `CheckpointId::get`, landing here on the keeping side.
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// What the admission answers. THREE WAYS, and the compiler makes the caller face all
/// three (§5.3 point 1, `V4`).
///
/// ⛔ THERE IS NO `is_ok()`, NO `is_granted()`, AND NO CONVERSION TO A BOOLEAN. That is how
/// `V4` becomes a SIGNATURE instead of a recommendation: "refused" and "queued" are
/// different answers that call for different behaviour, and a boolean would collapse them.
/// The negative case names a method that does not exist -- so the day somebody adds it the
/// case starts COMPILING and trybuild reports `error`, which no bulk regeneration disarms
/// (gotcha #42, strong form).
///
/// ⛔ `Refused` CARRIES TWO NUMBERS AND NOT A SENTENCE. design/02 wants "why it does not fit,
/// and the workable alternative": the alternative is built by the interface, the kernel
/// hands it the material. Suggesting another profile would be L2 logic inside the kernel
/// (ADR-0020).
///
/// ⚠️ NO `Debug` AND NO `PartialEq`, because `Granted` carries a `Grant` and that type has
/// neither, deliberately. Deriving them here would mean giving `Grant` a `Debug` FOR THE
/// CONVENIENCE OF THE BENCH -- exactly what `ports::process` refused. Probes match instead.
#[must_use]
pub enum Admission {
    /// It fits. The grant is the only way to start a worker.
    Granted(Grant),
    /// It does not fit now, and the request is waiting IN ITS OWN LANE (§5.3.1).
    Queued(TicketId),
    /// It does not fit and it never will under this budget.
    Refused { asked: Mib, ceiling: Mib },
}

/// What a held grant is doing. ⛔ IT NESTS RATHER THAN FLATTENS, and the nesting IS the
/// rule: §5.3 point 3 wants `Revoking` to be NOT REPRESENTABLE for a non-preemptible
/// profile. `NonPreemptible` HAS NOWHERE TO PUT ONE -- it is a unit variant -- so the
/// illegal state is not forbidden at runtime, it cannot be spelled.
///
/// ⚠️ `NonPreemptible` AND NOT `Permanent`, and the difference is real: a job that cannot
/// be interrupted still FINISHES and releases. Permanence is not a type -- it is "nobody
/// calls release", which is exactly how the two permanent grants of the composition root
/// stay held (§4.3 of the design).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Activity {
    NonPreemptible,
    Preemptible(PreemptibleState),
}

/// The two states only a preemptible grant can be in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreemptibleState {
    Running,
    /// The arbiter has asked for the resource back. `deadline` is on the MONOTONIC axis --
    /// never wall time: a clock that steps backwards cannot expire a grant (§5.3 point 2).
    Revoking { deadline: Monotonic },
}
