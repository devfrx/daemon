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

use alloc::collections::BTreeMap;

use crate::parameters::Parameters;
use crate::time::{Millis, Monotonic};

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
/// ⛔ There is deliberately NO public constructor, and since Task 5 there IS an issuer:
/// `Arbiter::admit`, the only function in this crate that builds one. That is the whole of
/// §5.6 -- whoever writes "start the worker" without going through it does not compile.
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
/// arrives in milestone 5" in the FUTURE, and "today the type has no issuer": the FIRST is
/// spent -- the arbiter module is this one. ⚠️ AND THE SECOND IS SPENT TOO SINCE TASK 5,
/// which is why this sentence is rewritten instead of left standing: at task 4 it read
/// "nothing constructs one yet, `admit` arrives at Task 5", and `admit` is now below it in
/// this file. It also recorded a DIVERGENCE -- the field was a private UNIT, `Grant(())`,
/// because the named field the plan dictated then bought nothing that the unit field did not
/// buy for free and cost an `#[allow(dead_code)]`, which this repository treats as a
/// prohibition switched off (gotcha #13). ⛔ THAT PARAGRAPH IS NOT COPIED, because the shape
/// it described no longer exists: the named field is back, dictated again by the milestone 5
/// design, and it is `id` that lets `Arbiter::release` tell a grant of THIS arbiter from a
/// grant of another one -- with the limit of that written beside `ReleaseError`.
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
/// The negative case -- `tests/compile_fail/admission_has_no_is_granted.rs`, written at
/// Task 5 once there was an `admit` to obtain a real `Admission` from -- names a method that
/// does not exist, so the day somebody adds it the case starts COMPILING and trybuild reports
/// `error`, which no bulk regeneration disarms (gotcha #42, strong form). ✅ MEASURED, not
/// asserted: with `is_granted` added, that case comes back `error` and the other twenty-six
/// stay `ok`.
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

/// What can go wrong when handing a grant back.
///
/// ⛔ ONE VARIANT, AND IT IS REACHABLE -- which is what keeps this `Result` from being the
/// dead surface this repository removed from `Record::encode`. Two arbiters can exist in
/// one process (a bench builds several), and a grant issued by one is meaningless to the
/// other. Crediting it silently would corrupt the budget of an arbiter that never issued
/// it, which is over-admission arriving by the back door.
///
/// ⚠️ DECLARED LIMIT, AND IT IS WRITTEN HERE BECAUSE THE PROBE CANNOT SAY IT. What
/// `release` actually answers is "THAT IS NOT IN MY BOOKS", not "I can tell my grants from
/// somebody else's". `GrantId` is a counter that restarts at zero for every `Arbiter`, so
/// two arbiters that have BOTH issued grants share the same id space, and the second one
/// would credit the first one's grant as if it were its own. The bench catches the case it
/// can -- an empty second arbiter -- and no more.
///
/// ⛔ AND THE DESIGN IS NOT CHANGED TO CLOSE IT, because closing it means giving an
/// `Arbiter` an IDENTITY, and that is a decision for the owner rather than for the task
/// that noticed. What buys the protection today is that one process has one arbiter: the
/// several that exist at once exist in BENCHES. The day a second real arbiter is wired, this
/// paragraph is the debt to settle, and it is written where the type is instead of in a
/// document nobody opens beside the code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseError {
    /// This arbiter never issued that grant.
    UnknownGrant,
}

/// What the arbiter remembers about a grant it has issued.
///
/// ⛔ TWO FIELDS, AND THE OTHER TWO ARRIVE WITH THEIR OWN READERS. The milestone 5 design
/// gives `Held` a `lane: ComputeClass` and an `activity: Activity` as well; neither has a
/// reader in this task, so both would compile as `dead_code` warnings, and this repository
/// does not switch a warning off with `#[allow]` (gotcha #13). `lane` comes with the queues
/// at task 6 -- a ticket has to know which lane it is waiting in -- and `activity` with the
/// revocation at task 7, which is the first thing that reads what a grant is DOING. A field
/// born with its consumer is a field somebody uses.
struct Held {
    reserved: Mib,
    /// The validity window, on the MONOTONIC axis (§5.3 point 2).
    expires_at: Monotonic,
}

/// The GPU arbiter: admission on VRAM, lanes on compute (ADR-0005).
///
/// ⛔ `BTreeMap` AND `Vec`, AND IT IS NOT A PREFERENCE: `HashMap` lives in `std`, which this
/// crate does not name, so gotcha #12 -- iteration order seeded PER PROCESS, which V29
/// forbids -- is closed here by the compiler and for free (§5.1). It also closes M-6.
pub struct Arbiter {
    parameters: Parameters,
    next_grant: u64,
    held: BTreeMap<GrantId, Held>,
}

impl Arbiter {
    /// ⛔ IT TAKES `Parameters` AND NOT A BARE `Mib`. That is the shape `Executor::new`
    /// already has and the one the catalogue row `V29 · §2.8 · ADR-0034` names -- "building
    /// a decision without the delivered parameters". A bare number would have the
    /// composition root read the total and hand it over OUTSIDE the mechanism ADR-0034
    /// exists to impose.
    pub const fn new(parameters: Parameters) -> Self {
        Arbiter {
            parameters,
            next_grant: 0,
            held: BTreeMap::new(),
        }
    }

    /// How much VRAM is spoken for right now. ⛔ IT COLLECTS NOTHING: it reports the books
    /// as they are, so a probe can tell "the collection happened" from "the number looks
    /// right anyway".
    pub fn allocated(&self) -> Mib {
        self.held
            .values()
            .fold(Mib::ZERO, |sum, held| sum.saturating_add(held.reserved))
    }

    /// Admission (§5.3). THREE ways out, and the caller must face all three.
    ///
    /// ⛔ IT COLLECTS THE EXPIRED FIRST, and the declared limit of that is written where the
    /// property is: between two operations an expired grant stays in the books. It denies
    /// nothing to nobody -- there is nobody -- and at the first one who looks it is already
    /// freed. The property holds AT EVERY POINT WHERE IT IS OBSERVABLE, which is why the
    /// probe advances the clock and then ASKS.
    ///
    /// ⛔ AND IT DOES NOT RECEIVE A `WorkDescriptor`. `cold_start` is not reachable from
    /// here, and that is `Q8 · §5.2.1` held by the shape of this signature rather than by a
    /// rule in a document.
    pub fn admit(
        &mut self,
        profile: &ResourceProfile,
        valid_for: Millis,
        now: Monotonic,
    ) -> Admission {
        self.collect_expired(now);

        let ceiling = self.parameters.total_vram();
        let asked = profile.reserved_vram;

        if asked > ceiling {
            // ⛔ Bigger than the whole machine: no release will ever make room, so a ticket
            // here would be a leak that looks like patience.
            return Admission::Refused { asked, ceiling };
        }

        if self.allocated().saturating_add(asked) > ceiling {
            return Admission::Refused { asked, ceiling };
        }

        let id = GrantId(self.next_grant);
        self.next_grant += 1;
        self.held.insert(
            id,
            Held {
                reserved: asked,
                expires_at: now.saturating_add(valid_for),
            },
        );
        Admission::Granted(Grant { id })
    }

    /// Hands a grant back, and answers with the reservation that returned to the budget.
    ///
    /// ⛔ IT CONSUMES THE GRANT: releasing twice DOES NOT COMPILE, which is level 1 and
    /// cheaper than any runtime guard. The consequence for milestone 6 is written beside
    /// `Grant`.
    pub fn release(&mut self, grant: Grant, now: Monotonic) -> Result<Mib, ReleaseError> {
        self.collect_expired(now);
        match self.held.remove(&grant.id) {
            Some(held) => Ok(held.reserved),
            None => Err(ReleaseError::UnknownGrant),
        }
    }

    /// ⛔ PRIVATE, AND DELIBERATELY. A public `collect` would be a SECOND way of advancing
    /// this state -- one no probe covers and no caller has to reach -- while "the arbiter
    /// collects before it decides" is a property of every operation rather than a step
    /// somebody remembers to take.
    fn collect_expired(&mut self, now: Monotonic) {
        self.held.retain(|_, held| held.expires_at > now);
    }
}
