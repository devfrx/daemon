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

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::String;
use alloc::vec::Vec;

use crate::parameters::Parameters;
use crate::ports::journal::{Journal, JournalError, StepId};
use crate::record::{EffectClass, Record, RecordKind, RecordV1, Trust};
use crate::time::{Millis, Monotonic};

pub mod policy;
pub mod resource;

pub use policy::{LocalPolicy, MakeRoom, RemotePolicy, VramPolicy};
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
/// ⛔ There is deliberately NO public constructor, and since Task 5 there IS an issuer.
///
/// ⚠️ RECALL OF 2026-08-19, MILESTONE 5 TASK 6 -- "THE ONLY FUNCTION IN THIS CRATE THAT
/// BUILDS ONE" WAS TRUE FOR ONE TASK, and it is REWRITTEN rather than qualified: an
/// exclusivity is read as a GUARANTEE, and a stale guarantee is worse than a stale count
/// (gotcha #31 on an adjective, the species of `E38`). It said `Arbiter::admit` was that
/// function; task 6 gave the queue its own door and `Arbiter::promote` hands out grants too.
///
/// ⛔ WHAT IS STILL TRUE, AND IT IS THE POINT OF §5.6: a grant is CONSTRUCTED in exactly ONE
/// place, the private `Arbiter::issue`, and both public doors go through it. A second
/// construction site would be a second way to obtain a grant, which is the thing §5.6 exists
/// to remove. From outside the crate the two ways to come by one are `admit` and `promote`,
/// and whoever writes "start the worker" without passing an admission does not compile.
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

/// A ticket that came out of the queue, with the grant it waited for.
///
/// ⚠️ IT IS A STRUCT AND NOT A TUPLE, and the reason is the same one design/02 gives for
/// `Refused`: whoever reads `promotion.ticket` should not have to remember which of two
/// unnamed slots held it. A tuple of two things one of which has no `Debug` is worse still.
///
/// ⚠️ NO `Debug` AND NO `PartialEq`, for the same reason `Admission` has neither: it carries
/// a `Grant`. The bench compares `promotion.ticket`, which is a `TicketId` and derives both.
pub struct Promotion {
    pub ticket: TicketId,
    pub grant: Grant,
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
    Revoking {
        deadline: Monotonic,
    },
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
///
/// ⚠️ A SECOND AND A THIRD DECLARED LIMIT, AND THEY ARE THE ONES THAT BITE FIRST. `release`
/// calls `collect_expired` BEFORE it looks, so anything that sweep has taken off the books is
/// gone by the time `remove` runs: it answers `None` and the caller gets `UnknownGrant`. Since
/// task 7 that sweep has TWO deadlines, so there are TWO ways for a grant THIS ARBITER ISSUED
/// to be missing from its own books -- its validity window closed, or its GRACE ran out after
/// the arbiter asked it back.
/// ✅ MEASURED on a throwaway probe, not deduced: admitted for 5_000 ms, released at 5_001 ->
/// `Err(UnknownGrant)`; released at 4_999 -> `Ok(Mib(4096))`; released at 5_000 EXACTLY ->
/// `Err(UnknownGrant)` too, because the window is half-open, `[start, expiry)`. The grace
/// deadline is half-open by the same rule -- one rule for both, see `collect_expired`.
/// ✅ AND THE THIRD CAUSE IS MEASURED TOO, on a throwaway probe deleted straight after and on
/// 2026-08-20: a grant asked back at `0` with a grace of `500` and released at `500` EXACTLY ->
/// `Err(UnknownGrant)`, and the same grant released at `499` -> `Ok(Mib(4096))`. So being under
/// revocation is NOT by itself an obstacle to handing the grant back -- only the sweep that
/// follows the deadline is.
///
/// ⚠️ RECALL OF 2026-08-20, MILESTONE 5 TASK 7, IN REVIEW -- THIS BLOCK SAID "TWO CAUSES" AND
/// SAID IT IN THREE PLACES, AND ALL THREE ARE REWRITTEN RATHER THAN ANNOTATED. A true sentence
/// added under a false one leaves the false one standing, which is finding A-2 of this project's
/// own audit done again. The count was right until this task gave `collect_expired` its second
/// deadline; from the moment forced reclamation landed, a revoked grant whose grace expired
/// answers `UnknownGrant` for a reason neither old paragraph named.
///
/// ⛔ SO THREE CASES ARE CONFLATED IN ONE VARIANT, and THE NAME STILL STATES THE STRONGEST OF
/// THE THREE -- which of an expired grant, and of a reclaimed one, is simply FALSE. That is why
/// the doc line below no longer repeats it. What the guard buys is "that is not in my books, so
/// I will not credit it", and THAT holds in all three cases: it is the whole of the
/// over-admission protection. What it does not buy is telling the caller WHICH of the three
/// happened.
///
/// ⚠️ TODAY IT COSTS NOTHING, and the reason is a measurement rather than a hope: every caller of
/// `release` in this repository is a PROBE, and they all live in `tests/arbiter_admission.rs` --
/// NO PRODUCTION CONSUMER EXISTS. It starts costing at milestone 6, where `Worker::kill` hands
/// the grant back when the work FINISHES, which can perfectly well be after the window; there
/// "your release failed", "it was already done for you" and "it was TAKEN from you" are THREE
/// different pieces of news, and the third is the one a caller can act on -- being preempted is
/// not the same event as outliving your own window. A second variant `Expired` was the known
/// remedy while there were two causes; with three the shape of the remedy is itself part of the
/// decision. It is a DESIGN decision either way, so it is RECORDED FOR THE OWNER in the plan's
/// errata (`E30`, widened on 2026-08-20 by `E72`) instead of being taken by the task that
/// noticed it.
///
/// ⚠️ RECALL OF 2026-08-20, SECOND REVIEW OF MILESTONE 5 TASK 7 -- THE PARAGRAPH ABOVE SAID
/// "`release` HAS TWO CALLERS", AND THE FIGURE IS REMOVED RATHER THAN RECORRECTED. It was true
/// when task 5 wrote it, and task 6 falsified it in the very next commit by giving the queue
/// probes their releases. ⛔ AND IT IS NOT REPLACED BY A BIGGER NUMBER, because the number was
/// never what the argument needed: "no production consumer exists" carries the whole of it, and
/// it is the half that does not rot. A figure that lives in more than one document gets taken
/// out, not realigned -- the rule of gotcha #68, applied to the document that hosts it.
/// ⛔ THE THIRD REVIEW FINISHED THAT JOB, because the second one had not: the RECOUNT that
/// replaced the figure was itself written into four places at once, this paragraph among them,
/// under a heading saying no bigger number replaces it. The recount now lives in the plan's
/// `E77` alone, and everything else -- here, `E30`, the register -- points there instead of
/// repeating it. Gotcha #68 is about a figure living in two places, not about which figure it is.
/// Registered as `E77`, closed by `E85`.
///
/// ⚠️ AND NO PROBE PINS THOSE THREE VALUES, WHICH IS A CHOICE RATHER THAN AN OVERSIGHT. A test
/// asserting `Err` at 5_001 would freeze the very behaviour `E30` puts in front of the owner:
/// the day the second variant arrives, that probe goes red FOR HAVING BEEN RIGHT, and a probe
/// that must be deleted to take a decision is a vote against taking it. So the measurement
/// lives here, beside the type, and moves when the decision does. ⛔ THE COST IS STATED
/// INSTEAD OF HIDDEN: moving `collect_expired` after the lookup -- one of the two roads `E30`
/// names -- turns nothing red, and this paragraph would become false in silence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseError {
    /// Not in this arbiter's books. ⚠️ THREE CAUSES, ONE ANSWER -- see the declared limits
    /// above: the grant came from ANOTHER arbiter, or it was this arbiter's and its validity
    /// window had already closed, or it was this arbiter's and the grace of a revocation had
    /// run out, so the sweep had already taken it back.
    UnknownGrant,
}

/// What the arbiter remembers about a grant it has issued.
///
/// ⚠️ RECALL OF 2026-08-20, MILESTONE 5 TASK 7 -- THIS COMMENT SAID "TWO FIELDS, AND THE OTHER
/// TWO ARRIVE WITH THEIR OWN READERS", AND IT IS REWRITTEN AND NOT ANNOTATED, because a true
/// sentence appended under a false one leaves the false one standing, which is finding A-2 of
/// this project's own audit done again. It carried two dated recalls of its own and both are
/// spent: task 5 held `lane` and `activity` back because neither had a reader, so both would
/// have compiled as `dead_code` warnings and this repository does not switch a warning off with
/// `#[allow]` (gotcha #13); task 6 corrected the PREMISE of the `lane` one -- a WAITING request
/// is a `Waiting`, which carries the whole `ResourceProfile` and therefore its lane, so
/// `promote` never looks at `held` -- and named `ask_back` at task 7 as the first reader of
/// BOTH. ⛔ TASK 7 IS THIS ONE, `ask_back` EXISTS BELOW, AND IT READS BOTH: it chooses its
/// victim by `lane`, and by `activity` it refuses to ask back what is already on its way out.
///
/// ⛔ FIVE FIELDS AND NOT FOUR, AND THE FIFTH IS THE ONE THE MILESTONE 5 DESIGN DOES NOT LIST.
/// `grace` is here because `Preemption` lives in the PROFILE and the profile is not kept: the
/// deadline of a revocation is `now + grace`, and without the field there is nothing to add.
/// ⛔ KEEPING THE WHOLE PROFILE WOULD HAVE BEEN EASIER AND DEARER: it would put `name` and
/// `compute_class` in two places at once, and a `&'static str` retained inside the arbiter is
/// state no decision reads.
///
/// ⚠️ `grace` IS AN `Option` AND THE `None` IS NOT A MISSING VALUE: it is `Preemption::grace`'s
/// own word for "this profile is never revoked". That is what makes it a GUARD `ask_back` reads
/// and not merely a number it needs -- see the THREE guards of the `askable` closure there, which
/// are deliberately about three different questions.
///
/// ⚠️ RECALL OF 2026-08-20, SECOND REVIEW OF MILESTONE 5 TASK 7 -- IT SAID "THE TWO GUARDS", AND
/// THE NUMBER IS REWRITTEN RATHER THAN ANNOTATED. It was true until the first wave of corrections
/// moved the guard on the LANE inside the admissibility test, which took the count from two to
/// three; from that moment this sentence and the closure's own "THREE QUESTIONS AND NOT ONE, AND
/// THEY STAY THREE" were two present-tense figures contradicting each other in ONE file, which is
/// gotcha #31 and worse than a figure that is merely missing. Registered as `E78`.
struct Held {
    reserved: Mib,
    /// The validity window, on the MONOTONIC axis (§5.3 point 2).
    expires_at: Monotonic,
    /// The lane the grant was issued in. `ask_back` picks its victims by it, worst lane first.
    lane: ComputeClass,
    /// What the grant is doing -- and, since task 7, whether it has been asked back.
    activity: Activity,
    /// How long its holder gets to hand the resource over, when it can be asked for at all.
    grace: Option<Millis>,
}

/// A request waiting in its lane.
///
/// ⚠️ IT KEEPS THE WHOLE `ResourceProfile` AND NOT JUST THE RESERVATION, and that is what
/// makes a promotion INDISTINGUISHABLE from an admission: `issue` receives the same profile
/// the requester handed to `admit`, so a grant that came out of the queue is built from the
/// same material as one that never waited. Keeping only `reserved_vram` would make `promote`
/// a second, poorer way of describing a request.
///
/// ⚠️ AND IT KEEPS `valid_for` AND NOT AN `expires_at`. A window that started counting when
/// the request was QUEUED would charge the waiting to the work: the ticket would be handed a
/// grant already part-spent, and one that waited longer than its own window would expire
/// before it ever ran. The window opens when the grant is ISSUED.
struct Waiting {
    ticket: TicketId,
    profile: ResourceProfile,
    valid_for: Millis,
}

/// The GPU arbiter: admission on VRAM, lanes on compute (ADR-0005).
///
/// ⛔ `BTreeMap` AND `Vec`, BOTH FROM `alloc`, AND `BTreeMap` IS NOT A PREFERENCE:
/// `HashMap` lives in `std`, which this crate does not name, so gotcha #12 -- iteration
/// order seeded PER PROCESS, which V29 forbids -- is closed here by the compiler and for
/// free (§5.1). It also closes M-6.
///
/// ⛔ AND FOR `queues` THE ORDER IS NOT MERELY DETERMINISTIC, IT IS THE POLICY. `promote`
/// walks the map with the map's OWN iteration, which is `ComputeClass`'s hand-written
/// priority order, so "best lane first" costs nothing and the order stays stated in ONE
/// place -- the `priority()` key. A list of the three lanes written out inside `promote`
/// would be that order stated a SECOND time, which is the trap `resource.rs` refused a
/// derived `Ord` in order to remove.
///
/// ⚠️ RECALL OF 2026-08-19, OPENED AND CLOSED THE SAME DAY. At task 5 this line said
/// "`BTreeMap` AND `Vec`" while the struct had no `Vec` -- the sentence had been copied from
/// the milestone 5 design, which was thinking of the lane queues -- so it was cut, with a
/// note saying the word would come back with the queues. It has: `queues` holds a
/// `Vec<Waiting>` per lane. The note is rewritten instead of left standing as a promise
/// already kept.
pub struct Arbiter {
    parameters: Parameters,
    /// The ONE active VRAM policy (ADR-0006). It is asked exactly once, in `admit`.
    policy: VramPolicy,
    next_grant: u64,
    next_ticket: u64,
    held: BTreeMap<GrantId, Held>,
    /// One queue per lane, in arrival order within the lane (§5.3.1).
    queues: BTreeMap<ComputeClass, Vec<Waiting>>,
}

impl Arbiter {
    /// ⛔ IT TAKES `Parameters` AND NOT A BARE `Mib`. That is the shape `Executor::new`
    /// already has and the one the catalogue row `V29 · §2.8 · ADR-0034` names -- "building
    /// a decision without the delivered parameters". A bare number would have the
    /// composition root read the total and hand it over OUTSIDE the mechanism ADR-0034
    /// exists to impose.
    ///
    /// ⛔ AND THE POLICY IS HANDED OVER TOO, WHICH IS `V3` AT LEVEL 1. `VramPolicy` is an
    /// enum, so the value that arrives here carries exactly ONE policy and "two active at
    /// once" is not expressible -- the rule §5.4 used to check with an example-based test,
    /// risen to the compiler (§2.8.2, catalogue §7.4.1 block C).
    ///
    /// ⚠️ IT IS A SECOND ARGUMENT AND NOT A FIELD OF `Parameters`, which is the plan's shape
    /// and diverges from the WORDING of §2.8.2 -- "the delivered value carries one policy",
    /// singular. It is not a weakening: the arbiter still receives the policy at
    /// construction and still cannot read a configuration. Registered for the owner rather
    /// than decided here, because moving it into `Parameters` changes a type §2.8 pins.
    pub const fn new(parameters: Parameters, policy: VramPolicy) -> Self {
        Arbiter {
            parameters,
            policy,
            next_grant: 0,
            next_ticket: 0,
            held: BTreeMap::new(),
            queues: BTreeMap::new(),
        }
    }

    /// The policy this arbiter is running.
    ///
    /// ⚠️ ITS CONSUMERS ARE BENCHES, AND THAT IS SAID RATHER THAN LEFT TO BE NOTICED: it is
    /// `pub`, so `dead_code` would not have mentioned it either way. What buys it, in
    /// `tests/arbiter_policy.rs`, is `each_policy_names_itself` and -- since task 9 --
    /// `a_policy_transition_writes_its_intent_before_its_outcome` and
    /// `a_refused_intent_leaves_the_policy_where_it_was`, which read the name THROUGH the
    /// arbiter, FROM OUTSIDE THE CRATE.
    ///
    /// ⚠️ RECALL OF 2026-08-20, MILESTONE 5 TASK 9: this doc ended on a PREDICTION about task 9
    /// (gotcha #57, written at task 8 about code that did not exist), and task 9 MEASURED IT
    /// FALSE -- `set_policy` reads `self.policy`, the FIELD. The sentence is REMOVED, not
    /// answered beside itself (gotcha #76), on the precedent of finding `A-7`.
    pub const fn policy(&self) -> &VramPolicy {
        &self.policy
    }

    /// Swaps the active policy, AS A JOURNALLED STEP (§5.4).
    ///
    /// ⛔ INTENT, THEN THE EFFECT, THEN THE OUTCOME -- and the order is V6 rather than
    /// tidiness. Changing policy has real effects on the world (evictions, reloads), and
    /// nothing executes before the intent is DURABLE. A transition cut in half leaves a step
    /// IN DOUBT, reconcilable like every other (§4.3).
    ///
    /// ⛔ THE JOURNAL COMES BY REFERENCE AND IS NOT OWNED, for the mechanical reason already
    /// written about the reactor: an arbiter that owned one would give it two owners the day
    /// a caller needs it too, and the borrow would not pass.
    ///
    /// ⛔ IT TAKES NO `now`, AND THAT DIVERGES FROM THE PLAN. Every operation that collects
    /// expired grants takes one; this one TOUCHES NEITHER BOOK -- not `held`, not `queues` --
    /// so it has nothing to collect, and it follows the precedent of `allocated`, which
    /// declares that it collects nothing. An ignored parameter is the dead surface this
    /// crate took off `Record::encode` and refused `Ipc::accept`. ⚠️ THE DAY THE TRANSITION
    /// TOUCHES THE BOOKS the argument comes back, and it comes back as a COMPILER ERROR at
    /// every call site, not as a silent regression.
    ///
    /// ⛔ `EffectClass::Idempotent`, AND IT IS ARGUED RATHER THAN PICKED. ADR-0007 treats an
    /// effect with no declared class as `Unrepeatable`, so the choice has to be earned:
    /// "make the active policy be X" CONVERGES when re-run, which is what `Idempotent`
    /// means. ⚠️ THE DECLARED LIMIT: what milestone 5 does here is swap an object. When the
    /// CONTENT of an eviction arrives (L2), this class has to be looked at again -- a reload
    /// is not free to repeat.
    ///
    /// ⚠️ `Trust::Instruction`, and the payload is EMPTY: no external byte reaches this
    /// record. The label is about the payload (`Trust`'s own doc), and an empty payload that
    /// came from nowhere is ours.
    pub fn set_policy<J: Journal>(
        &mut self,
        policy: VramPolicy,
        step: StepId,
        journal: &mut J,
    ) -> Result<(), JournalError> {
        journal.intent(step, &transition_record(RecordKind::Intent, policy.name()))?;
        self.policy = policy;
        journal.outcome(
            step,
            &transition_record(RecordKind::Outcome, self.policy.name()),
        )
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
    ///
    /// ⛔ IT NEVER CONSULTS THE QUEUE, AND A LATECOMER THEREFORE JUMPS IT. This function reads
    /// `held` and `parameters` and nothing else: if the room is there when it is called, it
    /// says yes, whatever is already waiting. ✅ MEASURED on a throwaway crate outside the
    /// repository, not deduced: on a full 4_096 machine with a `Realtime` ticket queued,
    /// releasing the resident and then admitting a NEW `Batch` request of 4_096 answers
    /// `Granted` at once, and the `promote` that follows comes back EMPTY with the `Realtime`
    /// ticket still waiting. ⚠️ SO THE PER-LANE ORDER IS A PROPERTY OF `promote` AND NOT OF
    /// THE ARBITER: it decides who gets served out of the queue, not who gets in front of it.
    /// ⚖️ WHOSE PROBLEM IT IS, and it is not this function's: whether an admission has to yield
    /// to a waiting ticket is an ORCHESTRATION decision -- who calls `promote`, and when.
    /// Closing it here would mean an `admit` that can refuse room that exists. ⚠️ RECALL OF
    /// 2026-08-21 -- THIS SAID "which is task 10's". Task 10 closed on 2026-08-21 as the
    /// composition root: it assembles the graph and starts the executor ONCE, with no loop,
    /// so it builds no orchestration cycle to decide this IN. The decision belongs to whoever
    /// builds the first one -- the cycle that decides when to call `promote` relative to
    /// `admit` -- and none exists yet in this repository. REGISTERED FOR THE OWNER in the
    /// plan's errata, where it sits beside the permanent-quota voice it interacts with.
    ///
    /// ⛔ AND NOTHING IN THIS REPOSITORY HOLDS THE PARAGRAPH ABOVE -- said here rather than
    /// left to be discovered, because a claim about behaviour that no probe exercises is the
    /// species this task has already paid for once. ✅ MEASURED on 2026-08-19, not deduced:
    /// with `!self.queues.is_empty() ||` added to the second guard below -- so a latecomer
    /// QUEUES BEHIND the waiters instead of jumping them -- NOTHING goes red: 19 passed, 0
    /// failed in `tests/arbiter_admission.rs`, and 34 targets, 222 passed, 0 failed, 2 ignored
    /// across the workspace. THE MUTANT IS ALIVE, so the day whoever builds the first
    /// orchestration cycle -- the one that decides when to call `promote` relative to `admit`
    /// -- decides the other way, this paragraph becomes FALSE IN SILENCE with nothing going
    /// red to say so. ⚠️ RECALL OF 2026-08-21 -- THIS SAID "the day task 10 decides the other
    /// way". Task 10 closed on 2026-08-21 as the composition root, and builds no such cycle:
    /// it assembles the graph and starts the executor ONCE, with no loop. None exists yet in
    /// this repository. ⚖️ AND IT IS NOT PINNED, ON PURPOSE AND ON THE MERITS: a probe
    /// asserting `Granted` for the latecomer would freeze exactly the choice the errata voice
    /// `E51` puts in front of the owner, and a probe that has to be deleted to take a decision
    /// is a vote against taking it -- the precedent is `E39`, which refused for the same reason
    /// to pin the three measured `release` values. Registered as `E53`.
    ///
    /// ⛔ AND SINCE TASK 8 THAT JUMP IS NO LONGER THEORETICAL, WHICH IS A CHANGE OF KIND AND NOT
    /// A DETAIL. Until task 8 `ask_back` had no production caller, so NO revocation ever happened
    /// outside a probe and the room a revocation frees did not exist: there was nothing for a
    /// latecomer to steal. Under `VramPolicy::Local` there is. `LocalPolicy` asks a resident back
    /// FOR a queued ticket, the sweep frees that reservation when the grace runs out, and the
    /// next `admit` -- which collects the expired first and then reads only `held` -- seats
    /// WHOEVER CALLS IT on room that was made for somebody else, while the ticket it was made for
    /// stays in its lane.
    ///
    /// ✅ MEASURED IN BOTH DIRECTIONS on 2026-08-20, on a throwaway probe deleted straight after,
    /// because a claim about behaviour that no probe exercises is the species this file has
    /// already paid for. ⛔ THE FIGURES ARE NOT REPEATED HERE, AND THAT IS THE POINT AND NOT
    /// BREVITY: they stood VERBATIM in THREE live documents at once -- this comment, the register
    /// and the errata -- and the copy that goes stale is the one in the source, which is the
    /// species `E99` had to rewrite in THIS file on the SAME day. They live in `E100` and in the
    /// task 8 section of `docs/porta-di-qualita.md`: two houses, not three. Registered as `E109`.
    ///
    /// ⚠️ AND NO PROBE HOLDS ANY OF THIS EITHER, said here for the same reason as the paragraph
    /// above: the behaviour is measured and NOT pinned, because pinning it would freeze the
    /// choice `E51` and `E53` put in front of the owner. ⚖️ THE CLOSER IS WHOEVER BUILDS THE
    /// FIRST ORCHESTRATION CYCLE -- the one that decides when to call `promote` relative to
    /// `admit` -- and it is still an ORCHESTRATION decision: what changed at task 8 is only
    /// that the cost of leaving it open is now paid in production and not on paper. ⚠️ RECALL
    /// OF 2026-08-21 -- THIS SAID "THE CLOSER IS STILL TASK 10". Task 10 closed on 2026-08-21
    /// as the composition root, and builds no orchestration cycle to be that closer: it
    /// assembles the graph and starts the executor ONCE, with no loop. None exists yet in this
    /// repository. REWRITTEN in place and not annotated below, which is finding A-2's rule.
    /// Registered as `E100`.
    ///
    /// ⛔ THE TWO GUARDS BELOW ANSWER DIFFERENTLY SINCE TASK 6, and until task 6 they did
    /// not. "Bigger than the whole machine" is `Refused` -- no release will ever make room,
    /// so a ticket there would be a leak that looks like patience -- while "bigger than what
    /// is free right now" is `Queued`. Before the queues both returned the same `Refused`
    /// with the same two numbers, which is why deleting the first one killed nothing
    /// (measured at task 5, errata `E28`).
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
            // ⛔ THE ONE PLACE THE TWO POLICIES DIFFER. ADR-0006 says exactly this is where
            // a conditional would have been planted, and why it is not one: the question is
            // asked ONCE, of an object, instead of being an `if` on the origin of the
            // inference that every future decision would grow another arm of.
            if self.policy.may_make_room() {
                let needed = self
                    .allocated()
                    .saturating_add(asked)
                    .saturating_sub(ceiling);
                // ⚠️ THE ANSWER IS DELIBERATELY NOT READ, and that is the honest shape rather
                // than a shortcut. Asking back MARKS: the reservation stays in the books for
                // the whole grace period, so even a fully covered need frees nothing NOW.
                // Acting on the number here would mean seating a request on VRAM its holder
                // is still using -- the very thing §5.3 point 4 gives the grace for. The room
                // arrives at the sweep, and `promote` is what hands it over.
                let _asked_back = self.ask_back(needed, profile.compute_class, now);
            }
            return self.enqueue(profile, valid_for);
        }

        Admission::Granted(self.issue(profile, valid_for, now))
    }

    /// The ONE place a request is put in its lane, and the answer that says so.
    ///
    /// ⛔ EXTRACTED AT TASK 8 AND NOT REWRITTEN. It was written inline inside `admit` at task
    /// 6; the policy branch above gave the queueing a second thing standing before it, and
    /// two statements in one branch read as one. The fields, their order and the counter are
    /// the task 6 ones exactly -- an extraction that changed behaviour would be a task 6
    /// rewrite wearing a task 8 label.
    ///
    /// ⛔ IT FITS THE MACHINE AND NOT THE MOMENT, so it WAITS. Refusing would make the answer
    /// depend on the instant the request happened to arrive, and §5.3.1 wants the request
    /// kept IN ITS OWN LANE instead. What keeps "for ever" out of here is `admit`'s first
    /// guard: a request bigger than the whole machine never reaches this function.
    fn enqueue(&mut self, profile: &ResourceProfile, valid_for: Millis) -> Admission {
        let ticket = TicketId(self.next_ticket);
        self.next_ticket += 1;
        self.queues
            .entry(profile.compute_class)
            .or_default()
            .push(Waiting {
                ticket,
                profile: *profile,
                valid_for,
            });
        Admission::Queued(ticket)
    }

    /// How many requests are waiting, across all lanes.
    pub fn queued(&self) -> usize {
        self.queues.values().map(Vec::len).sum()
    }

    /// Serves the queue with whatever room there is now, BEST LANE FIRST.
    ///
    /// ⛔ THE ORDER IS BY LANE AND NOT BY ARRIVAL, and it is a measurement rather than a
    /// taste: §5.3.1 says M-7's numbers stay valid AS AN UPPER BOUND precisely because the
    /// specified version keeps the order PER LANE. A single queue re-sorted on every
    /// release would invalidate that measurement, and it would have to be redone.
    ///
    /// ⛔ IT STOPS AT THE FIRST REQUEST THAT DOES NOT FIT, WITHIN A LANE, and does not skip
    /// ahead to a smaller one BEHIND IT IN ITS OWN LANE. Skipping there is a scheduling policy
    /// nobody decided, and it would let a large request wait for ever behind the small ones OF
    /// ITS OWN LANE.
    ///
    /// ⛔ ACROSS LANES THAT IS EXACTLY WHAT HAPPENS, and the sentence above is scoped rather
    /// than left to be read as a guarantee it does not give. A lane that stops FALLS THROUGH
    /// to the next one, so a small request in a WORSE lane is served while a big one in a
    /// BETTER lane waits -- a priority inversion, and starvation for as long as the small ones
    /// keep arriving. ✅ MEASURED on a throwaway crate outside the repository, not deduced: on
    /// a 4_096 machine holding `bulk` 3_072 and `small` 1_024, with a `Realtime` waiter of
    /// 4_096 queued BEFORE a `Batch` waiter of 1_024, releasing `small` promotes the `Batch`
    /// one and leaves the `Realtime` one waiting. ⚖️ IT IS WORK-CONSERVING AND DEFENSIBLE --
    /// the alternative holds the machine idle for a waiter that may never fit -- AND NOTHING
    /// DECIDES IT: §5.3, §5.3.1 and design/02 say nothing about the order across lanes. So it
    /// is REGISTERED FOR THE OWNER in the plan's errata instead of being chosen here.
    ///
    /// ⛔ AND NOTHING HOLDS THE PARAGRAPH ABOVE EITHER, which is declared here for the same
    /// reason as beside `admit`. ✅ MEASURED on 2026-08-19, not deduced: with the WHOLE PASS
    /// made to stop at the first lane whose head does not fit -- a flag on the inner `break`,
    /// then `break` on the outer loop, so no lane ever falls through -- NOTHING goes red: 19
    /// passed, 0 failed in `tests/arbiter_admission.rs`, and 34 targets, 222 passed, 0 failed,
    /// 2 ignored across the workspace. THE MUTANT IS ALIVE, so the day the first orchestration
    /// cycle -- the one that decides when to call `promote` relative to `admit` -- picks a
    /// different order across lanes, this paragraph becomes FALSE IN SILENCE. ⚠️ RECALL OF
    /// 2026-08-21 -- THIS SAID "task 7 or task 10 changes the order across lanes". Task 10 closed
    /// on 2026-08-21 as the composition root, and builds no orchestration cycle: it assembles
    /// the graph and starts the executor ONCE, with no loop. None exists yet in this repository.
    /// Task 7 closed without changing the order either: `ask_back` walks the lanes from the
    /// WORST, `lanes.iter().rev()`. ⚖️ AND A PROBE IS NOT THE REMEDY: pinning the fall-through
    /// would freeze the very policy the errata voice `E50` asks the owner to choose. Same
    /// reasoning as `E39`; registered as `E53`.
    ///
    /// ⚠️ `BTreeMap` ITERATES IN KEY ORDER, and `ComputeClass` orders by its explicit
    /// priority key -- so "best lane first" costs nothing here. TWO probes hold that coupling
    /// and they hold DIFFERENT halves of it, which is why both are named instead of one:
    /// `the_lane_order_is_pinned_by_name_and_realtime_comes_first` pins THE KEY -- its three
    /// values and the three `<` relations -- and stays GREEN if this function stops using the
    /// map's order altogether, while `the_queue_promotes_by_lane_and_not_in_arrival_order`
    /// pins THIS FUNCTION'S DEPENDENCE on that key, which is what mutation 1b turns red.
    pub fn promote(&mut self, now: Monotonic) -> Vec<Promotion> {
        self.collect_expired(now);

        let mut promoted = Vec::new();
        let ceiling = self.parameters.total_vram();

        // ⚠️ THE QUEUES ARE MOVED OUT FOR THE PASS AND PUT BACK, and the reason is the
        // borrow checker rather than cleverness: `issue` takes `&mut self`, so a borrow
        // reaching into `self.queues` cannot be alive across it. The swap costs one pointer
        // and keeps the map's OWN iteration -- the priority order of `ComputeClass` -- which
        // writing the three lanes out here would have restated a second time.
        //
        // ⛔ THE HAZARD, WRITTEN DOWN INSTEAD OF LEFT TO BE DISCOVERED: while this loop runs,
        // `self.queues` is EMPTY. It is sound only because the two things called inside --
        // `allocated` and `issue` -- read and write `held` and `next_grant` and nothing else.
        // Anything added to either that touches `queues` would see an empty map.
        let mut queues = core::mem::take(&mut self.queues);
        for queue in queues.values_mut() {
            while let Some(waiting) = queue.first() {
                let asked = waiting.profile.reserved_vram;
                if self.allocated().saturating_add(asked) > ceiling {
                    break;
                }
                let waiting = queue.remove(0);
                let grant = self.issue(&waiting.profile, waiting.valid_for, now);
                promoted.push(Promotion {
                    ticket: waiting.ticket,
                    grant,
                });
            }
        }
        self.queues = queues;

        promoted
    }

    /// How many grants have been asked back and have not handed over yet.
    ///
    /// ⛔ IT COLLECTS NOTHING, for the same reason `allocated` collects nothing: it reports the
    /// books as they are, so a probe can tell "the sweep happened" from "the number looks right
    /// anyway".
    pub fn revoking(&self) -> usize {
        self.held
            .values()
            .filter(|held| {
                matches!(
                    held.activity,
                    Activity::Preemptible(PreemptibleState::Revoking { .. })
                )
            })
            .count()
    }

    /// Asks back enough preemptible grants FROM LANES BELOW `below` to cover `needed`, and
    /// answers with how much was actually asked back.
    ///
    /// ⛔ IT MARKS, IT DOES NOT TAKE. The reservation stays in the books for the whole grace
    /// period: §5.3 point 4 gives the holder that long, and freeing the memory at once would
    /// seat a second consumer on VRAM the first is still using. The forced reclamation happens
    /// in the sweep, when the grace has run out.
    ///
    /// ⛔ IT STOPS AS SOON AS THE NEED IS COVERED. "It made room" is satisfied by revoking
    /// everything, which evicts two jobs to seat one.
    ///
    /// ⛔ AND IT MARKS NOTHING AT ALL WHEN THE NEED WILL NOT BE COVERED, which is the SAME
    /// damage by the other road: a pass that marked as it went could condemn a job -- the sweep
    /// takes a revoked grant off the books when its grace runs out -- and still answer less than
    /// was asked, so it would evict one and seat nobody. That is a mutation of the state left
    /// standing beside a reported failure, which is the silent degradation ADR-0005 and ADR-0019
    /// forbid. Hence TWO PASSES: a READ-ONLY one that adds up what could be reclaimed, and a
    /// marking one that runs ONLY if that total covers `needed`. Registered as `E69`.
    ///
    /// ⛔ WORST LANE FIRST: the cheapest thing to interrupt goes first. What grounds it is the
    /// lane table of design/02, which says of `ComputeClass::Interactive` that it is served
    /// before `ComputeClass::Batch`, and of `Batch` that it may wait indefinitely. What HOLDS it
    /// is `asking_back_takes_the_worst_lane_first`: before that probe the sentence was in a
    /// comment and in nothing else.
    ///
    /// ⚖️ WITHIN ONE LANE THE VICTIM IS THE OLDEST GRANT, AND NOTHING DECIDES THAT. The marking
    /// pass walks `held`, which is keyed by `GrantId`, so it meets the grants of a lane in the
    /// order they were issued and takes the first that fits the criterion -- with a `Batch` of
    /// `4_096` and a `Batch` of `512` and a need of `512` it marks the `4_096`, evicting 4 GiB to
    /// free half of one. The alternative -- the SMALLEST victim that suffices -- is just as
    /// defensible, and §5.3, §5.3.1 and design/02 say nothing about either: the lane table
    /// decides the order BETWEEN lanes and is silent WITHIN one. ⛔ SO IT IS DECLARED AND NOT
    /// PINNED, for the reason `E50`, `E51` and `E53` give: a probe asserting "the oldest" would
    /// freeze the very choice the errata puts in front of the owner, and a probe that must be
    /// deleted to take a decision is a vote against taking it. ✅ RE-MEASURED on 2026-08-20
    /// against the code of the second wave rather than carried over -- a figure quoted instead of
    /// redone is gotcha #31, and this suite grew by a probe in between: with the marking pass
    /// walking `self.held.values_mut().rev()` -- NEWEST first -- NOTHING goes red, 13 passed here
    /// and 20 in `tests/arbiter_admission.rs` (row 13 of the campaign). THE MUTANT IS STILL ALIVE,
    /// so the day somebody changes it this paragraph becomes false in silence with nothing to say
    /// so. Registered as `E70`, re-measured under `E80`.
    ///
    /// ⛔ IT COLLECTS THE EXPIRED FIRST, like every other operation. With this one there are
    /// FOUR, and the property "the arbiter collects before it decides" is why `collect_expired`
    /// is private rather than a step somebody remembers to take.
    ///
    /// ⚠️ `pub(crate)` BECAUSE ITS ONLY CALLER IS THE ADMISSION UNDER THE LOCAL POLICY, AND
    /// SINCE TASK 8 THAT CALLER EXISTS: it is `Arbiter::admit`, in the branch that cannot seat
    /// the request, immediately after `VramPolicy::may_make_room` has answered yes. Making room
    /// is a consequence of a request and never a thing somebody asks for, which is what the
    /// `pub(crate)` keeps true -- and the probes of the `#[cfg(test)] mod tests` at the foot of
    /// this file are what the `pub(crate)` puts there instead of in `tests/`.
    ///
    /// ⏳ RECALL OF 2026-08-20 -- A DEADLINE STOOD HERE AND TASK 8 IS WHAT MADE IT COME DUE.
    /// `E67` and `E74` left two `dead_code` warnings standing on purpose -- "fields lane and
    /// grace are never read" and "method ask_back is never used" -- rather than silence them
    /// with an `#[allow]`, which this repository treats as a prohibition switched off, and the
    /// falsifiable half was written right here in the form `E10` used at task 4: AT TASK 8 THOSE
    /// TWO WARNINGS MUST BE GONE, and IF THEY ARE STILL THERE THIS METHOD WAS NOT NEEDED AND IT
    /// IS REMOVED. ✅ IT CAME DUE AND IT WAS MET, measured and not deduced:
    /// `cargo build --locked --workspace` prints ZERO warnings, against the TWO the same command
    /// printed before the task. No `#[allow]`, no invented reader, no `pub` of convenience --
    /// none was needed. Registered as `E91`.
    ///
    /// ⛔ AND THE ORDER TO REMOVE THIS METHOD IS REWRITTEN AWAY RATHER THAN LEFT BESIDE THE
    /// FACT, because it was written in the PRESENT TENSE and this task turned it into the
    /// opposite of true: `ask_back` is what `LocalPolicy` is made of, and whoever read the
    /// paragraph as it stood would have been told to delete it. Registered as `E99`.
    pub(crate) fn ask_back(&mut self, needed: Mib, below: ComputeClass, now: Monotonic) -> Mib {
        self.collect_expired(now);

        // ⛔ THE ADMISSIBILITY TEST, WRITTEN ONCE AND ASKED BY BOTH PASSES. A verbatim copy of
        // it in the second pass would be the very defect the two passes exist to remove: the
        // reading pass and the marking pass would answer for different sets, and the arbiter
        // would promise room it then declines to take.
        //
        // ⚠️ A CLOSURE AND NOT A METHOD, AND THE MEASUREMENT THAT BOUGHT IT HAS EXPIRED --
        // rewritten on 2026-08-20 rather than left standing beside the fact that killed it. The
        // reason WAS `dead_code`: `ask_back` had no production caller, so anything it was the
        // only caller of was dead with it, and ✅ as an associated `Held::askable_by`
        // `cargo build --locked --workspace` printed a THIRD warning on top of the two the owner
        // had accepted (`E67`). ⛔ TASK 8 GAVE `ask_back` A PRODUCTION CALLER AND THE PREMISE
        // FELL WITH IT (`E91`). ✅ RE-MEASURED the same day instead of reasoned: with a private
        // `impl Held` helper reachable ONLY from inside this closure, the build prints ZERO
        // warnings -- `admit` reaches `ask_back`, so nothing behind it is dead any more.
        //
        // ⚠️ SO WHAT HOLDS THE CLOSURE TODAY IS NOT A MEASUREMENT AND IT IS SAID SO: it captures
        // `below`, and it keeps the admissibility test inside the body both passes read -- which
        // is the paragraph above and not this one. A `Held` method would now cost nothing and
        // would be a defensible change; it is simply not one this task took. Registered as
        // `E99`.
        //
        // ⛔ THREE QUESTIONS AND NOT ONE, AND THEY STAY THREE. Folding them into a single
        // `matches!(activity, Preemptible(Running))` would put the non-preemptible case behind
        // two guards that mask each other's mutation, which is what `E62` measured on the
        // dictated body and refused: each guard here has its OWN probe. ⚠️ Two of the three have
        // a SOLE killer as well -- the lane guard through mutations 2b and 2c, the "already on
        // its way out" one through mutation 7. The grace guard does not any more: mutation 8
        // kills its probe AND the one that holds the read-only pass, because that scenario has a
        // non-preemptible resident too. Written down in the register instead of left to be
        // rediscovered (`E73`).
        let askable = |held: &Held| -> Option<Millis> {
            if held.lane <= below {
                // ⛔ NOT BELOW THE ASKING LANE: a Realtime job is not evicted for an Interactive
                // one, however preemptible its profile says it is -- AND NEITHER IS A PEER.
                // `below` is EXCLUSIVE, so a grant in the asking lane itself is not a victim:
                // that is the boundary `a_grant_in_the_asking_lane_itself_is_not_asked_back`
                // stands on, and it is the case task 8 produces first.
                return None;
            }
            if matches!(
                held.activity,
                Activity::Preemptible(PreemptibleState::Revoking { .. })
            ) {
                // ⛔ ALREADY ON ITS WAY OUT: leave it alone. Marking it again would hand its
                // holder a FRESH grace for having been asked earlier, and would count its
                // reservation a second time -- room the arbiter does not have, which is
                // over-admission by the back door.
                return None;
            }
            // ⛔ NO GRACE MEANS NEVER REVOKED, and that is `Preemption::grace`'s own word for it
            // -- `None` "is the statement that this profile is never revoked". So this is the
            // guard that keeps `I2 · §5.3` at runtime, and it is the ONLY one that reads it: the
            // guard above asks a DIFFERENT question, "is it already on its way out".
            held.grace
        };

        // ⛔ FIRST PASS, AND IT WRITES NOTHING. If everything that may be asked back does not
        // add up to `needed`, the answer is "no room" and the books are left exactly as they
        // were -- no condemned holder, no reservation the asker cannot use.
        let reclaimable = self
            .held
            .values()
            .filter(|held| askable(held).is_some())
            .fold(Mib::ZERO, |sum, held| sum.saturating_add(held.reserved));
        if reclaimable < needed {
            return Mib::ZERO;
        }

        // ⛔ THE LANES COME FROM THE BOOKS AND THE ORDER FROM `ComputeClass` ITSELF, and neither
        // is a list of the three lanes written out here. `BTreeSet` iterates in key order --
        // which for `ComputeClass` is the hand-written `priority()` of §5.1 -- so `.rev()` IS
        // "worst lane first", and the order stays stated in ONE place. `promote` refuses a
        // hand-written list for exactly this reason and gets its order from its own map for
        // free; `held` is keyed by `GrantId` and not by lane, so this is what buys the same
        // thing here.
        //
        // ⛔ AND IT FILTERS BY `askable`, WHICH IS WHAT MAKES THE TWO PASSES SHARE ONE SET INSTEAD
        // OF TWO THAT MERELY AGREE. Without the filter this collects the lane of EVERY grant,
        // admissible or not, so a lane holding no candidate at all was walked for nothing --
        // harmless in itself, and that is not why it is filtered. It is filtered because the
        // distance between "the lanes in the books" and "the lanes the reading pass summed" is a
        // distance the comment below would otherwise have to PROMISE away; with the filter there
        // is nothing left to promise, the set IS the same set.
        //
        // ✅ MEASURED on 2026-08-20 IN BOTH DIRECTIONS and not argued, by returning `lanes.len()`
        // from here on two throwaway probes deleted straight after. IT DROPS WHAT IT MUST: with a
        // `Realtime` resident that `below = Interactive` excludes and one `Batch` candidate, the
        // pass walked TWO lanes before this filter and ONE after it. AND IT DROPS NOTHING ELSE:
        // with an `Interactive` and a `Batch` candidate both under `below = Realtime`, TWO before
        // and TWO after -- a lane that holds a candidate is never taken away from the marking pass.
        //
        // ⛔ AND IT IS A LIVE MUTANT, DECLARED HERE INSTEAD OF LEFT TO BE FOUND -- the form `E70`
        // uses for the order within a lane. A suite can only see BEHAVIOUR, and this filter has
        // none: the lanes it removes hold nothing the inner loop would have marked, because that
        // loop asks `askable` again itself. ✅ MEASURED the same day: with this one line deleted
        // the WHOLE WORKSPACE stays green -- 34 targets, 236 passed, 0 failed, 2 ignored, the
        // baseline exactly. ⚠️ SO THE SENTENCE ABOVE IS HELD BY NOTHING THAT CAN GO RED, and the
        // day somebody deletes the line for being dead weight it becomes false in silence. That is
        // the price of buying the property by CONSTRUCTION rather than by a probe, and it is
        // written down rather than discovered. Row 15 of the campaign, registered as `E83`.
        let lanes: BTreeSet<ComputeClass> = self
            .held
            .values()
            .filter(|held| askable(held).is_some())
            .map(|held| held.lane)
            .collect();

        // ⛔ SECOND PASS, AND IT MARKS. It cannot run out of candidates before `covered` reaches
        // `needed`, and since 2026-08-20 that is true BY CONSTRUCTION and not by promise: the set
        // it walks is the set the reading pass summed, because `askable` builds both.
        //
        // ⛔ AND IT RUNS TO THE END OF THE LANES, WHICH IS A SECOND FACT AND NOT THE SAME ONE. No
        // single lane has to cover `needed` on its own -- the reading pass added up ALL of them --
        // so a pass that stopped after the first lane would answer LESS than it had just promised,
        // with that lane's victims already marked: one evicted and nobody seated, which is the
        // damage `E69` exists to remove arriving by a third road. ⚠️ AND NOTHING HELD IT UNTIL
        // 2026-08-20, which is stated because it was measured and not deduced: `lanes.iter().rev()`
        // cut to `.take(1)` survived the WHOLE WORKSPACE -- 34 targets, 235 passed, 0 failed, 2
        // ignored. What holds it now is
        // `asking_back_crosses_into_the_next_lane_when_the_worst_one_is_not_enough` (`E75`).
        let mut covered = Mib::ZERO;
        for lane in lanes.iter().rev() {
            for held in self.held.values_mut() {
                if covered >= needed {
                    return covered;
                }
                if held.lane != *lane {
                    continue;
                }
                let Some(grace) = askable(held) else {
                    continue;
                };
                held.activity = Activity::Preemptible(PreemptibleState::Revoking {
                    deadline: now.saturating_add(grace),
                });
                covered = covered.saturating_add(held.reserved);
            }
        }

        covered
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

    /// The ONE place a grant is built, shared by `admit` and `promote`.
    ///
    /// ⛔ PRIVATE, AND THERE IS EXACTLY ONE OF IT. A second place that constructed a `Grant`
    /// would be a second way to obtain one, which is the thing §5.6 exists to remove -- and
    /// it would be a second place where the books are written, so the two could drift and
    /// nothing would say so. `admit` and `promote` differ in WHO they say yes to; what
    /// saying yes MEANS is here.
    ///
    /// ⛔ IT DOES NOT CHECK THE CEILING, and that is deliberate rather than forgotten: both
    /// callers have already decided that this request fits, and a guard repeated here would
    /// be a second statement of the admission rule. The reservation it books is the
    /// profile's own, so the number in the books and the number the requester declared
    /// cannot disagree.
    fn issue(&mut self, profile: &ResourceProfile, valid_for: Millis, now: Monotonic) -> Grant {
        let id = GrantId(self.next_grant);
        self.next_grant += 1;
        self.held.insert(
            id,
            Held {
                reserved: profile.reserved_vram,
                expires_at: now.saturating_add(valid_for),
                lane: profile.compute_class,
                // ⛔ `activity` AND `grace` COME FROM THE SAME FIELD, AND THAT IS WHAT MAKES
                // THEM AGREE. `Preemption` says BOTH whether the grant may be asked back and
                // how long its holder then gets, so a `Held` that was `Preemptible` with no
                // grace -- or `NonPreemptible` with one -- cannot be built from here. It is
                // §5.3 point 3 surviving the trip from the profile into the books.
                activity: match profile.preemption {
                    Preemption::Never => Activity::NonPreemptible,
                    Preemption::After(_) => Activity::Preemptible(PreemptibleState::Running),
                },
                grace: profile.preemption.grace(),
            },
        );
        Grant { id }
    }

    /// ⛔ PRIVATE, AND DELIBERATELY. A public `collect` would be a SECOND way of advancing
    /// this state -- one no probe covers and no caller has to reach -- while "the arbiter
    /// collects before it decides" is a property of every operation rather than a step
    /// somebody remembers to take.
    ///
    /// ⛔ TWO DEADLINES, ONE SWEEP, and they are genuinely different things: `expires_at` is the
    /// validity window the requester declared, `deadline` is the grace a revocation gave. Both
    /// are on the MONOTONIC axis, never wall time (§5.3 point 2).
    ///
    /// ⛔ AND BOTH WINDOWS ARE HALF-OPEN, WHICH IS ONE RULE AND NOT TWO. At `now == expires_at`
    /// the grant is already collected, and at `now == deadline` the grace is already over: both
    /// comparisons are `>`, and both boundaries have a probe standing exactly on them --
    /// `a_grant_is_collected_at_the_instant_its_window_closes` for the first,
    /// `the_grace_runs_out_at_the_instant_of_its_deadline` for the second. The second exists
    /// because the first had to be added at task 5 for the same reason: the two probes either
    /// side of a boundary step OVER it, and `>` mutated to `>=` then survives the whole suite
    /// (`E29`).
    fn collect_expired(&mut self, now: Monotonic) {
        self.held.retain(|_, held| {
            if held.expires_at <= now {
                return false;
            }
            match held.activity {
                Activity::Preemptible(PreemptibleState::Revoking { deadline }) => deadline > now,
                _ => true,
            }
        });
    }
}

/// The durable record of a policy transition.
///
/// ⚠️ `reason` CARRIES THE NAME OF THE POLICY, and that is why `MakeRoom::name` exists: a
/// record that said only "policy transition" would make the two directions indistinguishable
/// in the archive, and the archive is the only thing that survives. ✅ HELD IN BOTH
/// DIRECTIONS by `a_policy_transition_writes_its_intent_before_its_outcome` and
/// `a_transition_names_the_policy_it_moves_to`: one alone is satisfied by a constant.
fn transition_record(kind: RecordKind, policy: &'static str) -> Vec<u8> {
    Record::V1(RecordV1 {
        kind,
        effect: EffectClass::Idempotent,
        trust: Trust::Instruction,
        payload: Vec::new(),
        reason: String::from(policy),
    })
    .encode()
}

// ⚠️ A UNIT TEST MODULE IN `src/`, WHERE THIS CRATE OTHERWISE PUTS EVERY TEST IN `tests/`, and
// the deviation is declared rather than left to be noticed. It is the FIRST `#[cfg(test)]` of
// `kernel`, and it exists for ONE reason: `Arbiter::ask_back` is this crate's first
// `pub(crate) fn`, and a `pub(crate)` is NOT visible from an integration test -- which is a
// crate of its own. ✅ MEASURED and not assumed: calling it from `tests/arbiter_admission.rs`
// is `error[E0624]: method `ask_back` is private`. The precedent, and the wording, is
// `crates/platform/src/rng.rs`, which lives in `src/` for the same species of reason -- there
// it is a private FIELD that "from an integration test -- a crate of its own -- is
// unreachable".
//
// ⛔ ONLY PRIVACY MOVES A PROBE IN HERE, and the counter-example is named so the rule stays
// readable: `a_grant_that_is_neither_expired_nor_revoking_survives_the_sweep` belongs to this
// same task, needs nothing private, and therefore STAYS in `tests/arbiter_admission.rs` beside
// the other probes of the validity window. Moving it for company would split one subject
// across two files and buy nothing.
//
// ⛔ AND THE CHEAP WAY OUT WAS REFUSED ON THE MERITS: making `ask_back` `pub` so that every
// probe could live in `tests/` would publish an operation that is not one -- making room is a
// CONSEQUENCE of a request, never a thing somebody asks for. It is the same argument that
// keeps `collect_expired` private and that refused a `pub(crate)` constructor for `Grant`
// (gotcha #67).
#[cfg(test)]
mod tests {
    use super::*;

    const TURN_LIMIT: u64 = 10_000;

    /// The window every probe here uses when the value does not matter.
    const LONG: Millis = Millis::new(1_000_000);

    /// ⛔ IT HANDS OVER `RemotePolicy`, AND THAT IS NOT A FILLER ARGUMENT. Remote is the
    /// DEFAULT of ADR-0006 and the one that makes no room, so every probe below keeps the
    /// subject it was written with: with `LocalPolicy` the admission would start MARKING
    /// victims, and a dozen probes about the revocation would silently be about something
    /// else. The two policies have their own bench, `tests/arbiter_policy.rs`.
    fn arbiter(total: Mib) -> Arbiter {
        Arbiter::new(
            Parameters::new(TURN_LIMIT, total),
            VramPolicy::Remote(RemotePolicy),
        )
    }

    /// A profile the arbiter may NEVER ask back.
    fn profile(name: &'static str, vram: u64, lane: ComputeClass) -> ResourceProfile {
        ResourceProfile {
            name,
            reserved_vram: Mib::new(vram),
            compute_class: lane,
            preemption: Preemption::Never,
        }
    }

    /// A profile the arbiter may ask back, with the grace its holder then gets.
    fn preemptible(
        name: &'static str,
        vram: u64,
        lane: ComputeClass,
        grace: u64,
    ) -> ResourceProfile {
        ResourceProfile {
            name,
            reserved_vram: Mib::new(vram),
            compute_class: lane,
            preemption: Preemption::After(Millis::new(grace)),
        }
    }

    /// ⛔ ASKING BACK MARKS, IT DOES NOT TAKE. The reservation stays in the books for the whole
    /// grace period: §5.3 point 4 gives the holder that long to hand it over, and an arbiter
    /// that freed the memory at once would be admitting a second consumer onto VRAM the first
    /// one is still using.
    #[test]
    fn asking_a_grant_back_marks_it_and_does_not_free_it_yet() {
        let mut arbiter = arbiter(Mib::new(4_096));
        let Admission::Granted(_batch) = arbiter.admit(
            &preemptible("batch-resident", 4_096, ComputeClass::Batch, 500),
            LONG,
            Monotonic::ORIGIN,
        ) else {
            panic!("it fills the machine");
        };

        let asked_back = arbiter.ask_back(
            Mib::new(4_096),
            ComputeClass::Interactive,
            Monotonic::ORIGIN,
        );

        assert_eq!(asked_back, Mib::new(4_096), "one grant covers the need");
        assert_eq!(arbiter.revoking(), 1);
        assert_eq!(
            arbiter.allocated(),
            Mib::new(4_096),
            "the memory is still the holder's until the grace runs out"
        );
    }

    /// ⛔ THE GRACE IS COLLECTED, AND IT IS THE ARBITER HALF OF `Forzata` (§6.5 of the design).
    /// The other half -- actually killing the process -- needs `process` and is milestone 6.
    #[test]
    fn a_grace_that_ran_out_returns_the_reservation_to_the_budget() {
        let mut arbiter = arbiter(Mib::new(4_096));
        let Admission::Granted(_batch) = arbiter.admit(
            &preemptible("batch-resident", 4_096, ComputeClass::Batch, 500),
            LONG,
            Monotonic::ORIGIN,
        ) else {
            panic!("it fills the machine");
        };
        let _ = arbiter.ask_back(
            Mib::new(4_096),
            ComputeClass::Interactive,
            Monotonic::ORIGIN,
        );

        // Still inside the grace: nothing is free yet.
        assert_eq!(arbiter.allocated(), Mib::new(4_096));
        // Past it: the first one who looks finds the budget back.
        let after = arbiter.admit(
            &profile("the-interactive-one", 4_096, ComputeClass::Interactive),
            LONG,
            Monotonic::from_millis(501),
        );
        assert!(matches!(after, Admission::Granted(_)));
        assert_eq!(arbiter.allocated(), Mib::new(4_096), "one grant, not two");
        assert_eq!(arbiter.revoking(), 0);
    }

    /// ⛔ A GRANT INSIDE ITS GRACE IS NOT COLLECTED, and this is the direction the probe above
    /// steps over. `allocated()` collects nothing by design, and `revoking()` runs no sweep
    /// either -- so neither one, read on its own, exercises `collect_expired`. What catches a
    /// grant being freed INSIDE its grace is a sweep that runs while the grace is still
    /// running, and here that sweep is `admit`'s.
    ///
    /// ⛔ RECALL OF 2026-08-28, finding AUD-013 -- THIS PARAGRAPH SAID "neither `ask_back` nor
    /// `revoking()` runs a sweep", and the `ask_back` half was FALSE: `self.collect_expired(now)`
    /// is its first statement, which `ask_back_collects_the_expired_before_it_marks` below
    /// already says about this SAME function. `ask_back` still does not stand in for `admit`
    /// here, for a narrower reason than "it never sweeps": in THIS scenario it sweeps at the
    /// instant the resident is admitted, before anything is marked `Revoking` -- the mark is
    /// made by the REST of that same call, after its own leading sweep already ran, so that
    /// sweep has nothing yet to test the grace-arm against.
    ///
    /// ⛔ AND TWO EXCLUSIVITY CLAIMS WERE ALSO FALSE, MEASURED and not assumed -- "would
    /// satisfy every other probe here", and "the only way to catch it ... is `admit`".
    /// Mutating `collect_expired`'s `Revoking` arm to sweep unconditionally fails TWO probes
    /// under `cargo test --locked -p kernel --lib`, not one -- this one AND
    /// `asking_back_twice_does_not_buy_the_room_twice` (11 passed, 2 failed). The second one
    /// dies on `assert_eq!(arbiter.revoking(), 1)`, BEFORE the `admit` further down its body
    /// ever runs, and the sweep that gets it there is the leading one of its own SECOND
    /// `ask_back` -- at `200`, against a grace that runs to `500`. Both removed rather than
    /// replaced: an exclusivity claim is bought by remeasuring it, and this task did not.
    #[test]
    fn a_grant_inside_its_grace_keeps_its_reservation() {
        let mut arbiter = arbiter(Mib::new(4_096));
        let Admission::Granted(_batch) = arbiter.admit(
            &preemptible("batch-resident", 4_096, ComputeClass::Batch, 500),
            LONG,
            Monotonic::ORIGIN,
        ) else {
            panic!("it fills the machine");
        };
        let _ = arbiter.ask_back(
            Mib::new(4_096),
            ComputeClass::Interactive,
            Monotonic::ORIGIN,
        );

        let Admission::Queued(_) = arbiter.admit(
            &profile("the-interactive-one", 4_096, ComputeClass::Interactive),
            LONG,
            Monotonic::from_millis(499),
        ) else {
            panic!("the grace has not run out, so the memory is still the holder's");
        };
        assert_eq!(arbiter.allocated(), Mib::new(4_096));
        assert_eq!(arbiter.revoking(), 1, "still on its way out, not gone");
    }

    /// THE BOUNDARY THE TWO PROBES ABOVE STEP OVER, and it is the species `E29` found on the
    /// OTHER deadline: one asks at `501` and the other at `499`, so `500` itself -- the instant
    /// the grace runs out -- would be asked by nobody, and `deadline > now` mutated to `>=`
    /// would survive the whole suite on the very line those two exist to hold.
    ///
    /// ⛔ AND IT WRITES DOWN WHICH SEMANTICS IS THE CHOSEN ONE, because a boundary nobody names
    /// is a boundary somebody later "fixes". At `now == deadline` the grace IS ALREADY OVER and
    /// the reservation is back in the budget: the grace window is HALF-OPEN, `[asked, deadline)`,
    /// exactly like the validity window of `expires_at`. ONE rule for both deadlines, not two.
    #[test]
    fn the_grace_runs_out_at_the_instant_of_its_deadline() {
        let mut arbiter = arbiter(Mib::new(4_096));
        let Admission::Granted(_batch) = arbiter.admit(
            &preemptible("batch-resident", 4_096, ComputeClass::Batch, 500),
            LONG,
            Monotonic::ORIGIN,
        ) else {
            panic!("it fills the machine");
        };
        let _ = arbiter.ask_back(
            Mib::new(4_096),
            ComputeClass::Interactive,
            Monotonic::ORIGIN,
        );

        let after = arbiter.admit(
            &profile("the-interactive-one", 4_096, ComputeClass::Interactive),
            LONG,
            Monotonic::from_millis(500),
        );
        assert!(
            matches!(after, Admission::Granted(_)),
            "at now == deadline the grace is already over: [asked, deadline)"
        );
        assert_eq!(arbiter.allocated(), Mib::new(4_096), "one grant, not two");
        assert_eq!(arbiter.revoking(), 0);
    }

    /// ⛔ A NON-PREEMPTIBLE GRANT IS NEVER ASKED BACK, and this is `I2 · §5.3` seen from the
    /// runtime side -- the type already makes `Revoking` unspellable for it, this says the
    /// arbiter does not even try.
    ///
    /// ⛔ THE RESIDENT IS IN `Batch` AND NOT IN `Realtime`, AND THE LANE IS THE WHOLE PROBE.
    /// Under `Realtime` the guard on the LANE turns it away before its preemption is ever
    /// looked at, so the mechanism this name promises would never run and the probe would pass
    /// for the wrong reason (gotcha #74). `Batch` is strictly below the asking lane, so the
    /// only thing that can save it is the one thing this probe is about.
    #[test]
    fn a_non_preemptible_grant_is_never_asked_back() {
        let mut arbiter = arbiter(Mib::new(4_096));
        let Admission::Granted(_reserved) = arbiter.admit(
            &profile("batch-that-is-never-preempted", 4_096, ComputeClass::Batch),
            LONG,
            Monotonic::ORIGIN,
        ) else {
            panic!("it fills the machine");
        };

        let asked_back = arbiter.ask_back(
            Mib::new(4_096),
            ComputeClass::Interactive,
            Monotonic::ORIGIN,
        );

        assert_eq!(asked_back, Mib::ZERO, "nothing here can be taken back");
        assert_eq!(arbiter.revoking(), 0);
        assert_eq!(arbiter.allocated(), Mib::new(4_096));
    }

    /// ⛔ ONLY LOWER LANES, and it is the counter-probe of the one above by the other road: a
    /// `Realtime` job is not evicted to make room for an `Interactive` one, no matter how
    /// preemptible its profile says it is.
    #[test]
    fn only_lanes_below_the_asking_one_are_asked_back() {
        let mut arbiter = arbiter(Mib::new(4_096));
        let Admission::Granted(_realtime) = arbiter.admit(
            &preemptible("realtime-resident", 4_096, ComputeClass::Realtime, 500),
            LONG,
            Monotonic::ORIGIN,
        ) else {
            panic!("it fills the machine");
        };

        let asked_back = arbiter.ask_back(
            Mib::new(4_096),
            ComputeClass::Interactive,
            Monotonic::ORIGIN,
        );

        assert_eq!(asked_back, Mib::ZERO, "Realtime is not below Interactive");
        assert_eq!(arbiter.revoking(), 0);
    }

    /// ⛔ BELOW MEANS STRICTLY BELOW, AND THIS PROBE STANDS ON THE BOUNDARY ITSELF. Every other
    /// probe here puts its resident STRICTLY above or STRICTLY below the asking lane, so the
    /// boundary was asked by nobody and `lane <= below` mutated to `lane < below` survived the
    /// WHOLE WORKSPACE -- measured on 2026-08-20 against the suite AS IT WAS BEFORE this probe,
    /// ten here and twenty in `tests/arbiter_admission.rs`, with every other target green too.
    /// It is the species of `E29` on a third guard: the two probes either side of a boundary
    /// step OVER it.
    ///
    /// ⛔ AND IT IS THE CASE TASK 8 PRODUCES FIRST, which is why it is not a curiosity: the
    /// admission asks back BELOW ITS OWN LANE, so a peer in the very lane that is asking is the
    /// first thing `ask_back` will be handed. Evicting a peer for a peer is exactly what "only
    /// lanes BELOW" excludes. Registered as `E71`.
    #[test]
    fn a_grant_in_the_asking_lane_itself_is_not_asked_back() {
        let mut arbiter = arbiter(Mib::new(4_096));
        let Admission::Granted(_peer) = arbiter.admit(
            &preemptible(
                "interactive-resident",
                4_096,
                ComputeClass::Interactive,
                500,
            ),
            LONG,
            Monotonic::ORIGIN,
        ) else {
            panic!("it fills the machine");
        };

        let asked_back = arbiter.ask_back(
            Mib::new(4_096),
            ComputeClass::Interactive,
            Monotonic::ORIGIN,
        );

        assert_eq!(
            asked_back,
            Mib::ZERO,
            "Interactive is not BELOW Interactive: a peer is not a victim"
        );
        assert_eq!(arbiter.revoking(), 0);
    }

    /// ⛔ IT STOPS WHEN IT HAS ENOUGH, and the assertion is on the NUMBER: an arbiter that
    /// revoked everything preemptible would satisfy "it made room" and evict two jobs to seat
    /// one.
    #[test]
    fn asking_back_stops_as_soon_as_the_need_is_covered() {
        let mut arbiter = arbiter(Mib::new(8_192));
        for name in ["batch-a", "batch-b"] {
            let outcome = arbiter.admit(
                &preemptible(name, 4_096, ComputeClass::Batch, 500),
                LONG,
                Monotonic::ORIGIN,
            );
            assert!(matches!(outcome, Admission::Granted(_)));
        }

        let asked_back = arbiter.ask_back(
            Mib::new(4_096),
            ComputeClass::Interactive,
            Monotonic::ORIGIN,
        );

        assert_eq!(asked_back, Mib::new(4_096));
        assert_eq!(arbiter.revoking(), 1, "one was enough");
    }

    /// ⛔ IT MARKS NOTHING WHEN THE NEED WILL NEVER BE COVERED, and this is the OTHER road to
    /// the very damage the doc of `ask_back` says it is avoiding -- "it evicts two jobs to seat
    /// one" becomes "it evicts one and seats nobody". A pass that marked as it went condemned
    /// the `Batch` one -- at the end of its grace the sweep takes it off the books -- and still
    /// answered less than was asked, so the asker does not sit down either.
    ///
    /// ⚠️ THE IMMOVABLE ONE IS IN `Batch` TOO, DELIBERATELY: under `Realtime` the lane guard
    /// would turn it away and the probe would be about the lane instead of about the capacity
    /// (gotcha #74, the same trap `a_non_preemptible_grant_is_never_asked_back` fell into).
    /// `2_048` is all there is to reclaim, and `2_048` does not seat a `4_096`.
    #[test]
    fn asking_back_marks_nothing_when_the_reclaimable_does_not_cover_the_need() {
        let mut arbiter = arbiter(Mib::new(8_192));
        let Admission::Granted(_reclaimable) = arbiter.admit(
            &preemptible("batch-reclaimable", 2_048, ComputeClass::Batch, 500),
            LONG,
            Monotonic::ORIGIN,
        ) else {
            panic!("2048 of 8192 fits");
        };
        let Admission::Granted(_immovable) = arbiter.admit(
            &profile("batch-that-is-never-preempted", 6_144, ComputeClass::Batch),
            LONG,
            Monotonic::ORIGIN,
        ) else {
            panic!("2048 + 6144 fills the machine");
        };

        let asked_back = arbiter.ask_back(
            Mib::new(4_096),
            ComputeClass::Interactive,
            Monotonic::ORIGIN,
        );

        assert_eq!(
            asked_back,
            Mib::ZERO,
            "2_048 is all there is to reclaim, and it does not seat a 4_096"
        );
        assert_eq!(
            arbiter.revoking(),
            0,
            "nobody is condemned for a seat nobody gets"
        );
        // ⚠️ AND THE THIRD ONE CANNOT FAIL UNDER A MUTATION OF `ask_back`, WHICH IS WRITTEN HERE
        // INSTEAD OF BEING LEFT TO BE DISCOVERED -- AND IT IS KEPT RATHER THAN DELETED. That
        // function never takes anything off the books itself: only `collect_expired` does, and at
        // `ORIGIN` with `LONG` windows there is nothing to collect, so `allocated()` is `8_192`
        // for any implementation of the thing under test. ✅ MEASURED on 2026-08-20 in the THIRD
        // review, against the suite OF THAT MOMENT -- thirteen probes here, twenty in
        // `tests/arbiter_admission.rs` -- with the two assertions above removed and this one left
        // standing alone: it survives the reading pass being deleted (mutation 12, this probe's
        // own sole killer -- 13 passed, 0 failed, where at full strength it is 12/1), the grace
        // guard being dropped (mutation 8 -- 12/1, and the one that dies is somebody else), and
        // `ask_back` freeing what it marks (mutation 1 -- 6/7 isolated AND 6/7 at full strength,
        // this probe among the deaths in neither, so isolating it changes nothing there).
        //
        // ⛔ AND YET IT IS NOT THE TWO ABOVE THAT HOLD THIS PROBE, WHICH IS WHAT THE PARAGRAPH
        // STANDING HERE UNTIL THE THIRD REVIEW CLAIMED. This probe dies under mutations 5b, 5d, 8,
        // 11 and 12, and they do not all kill it through the same assertion. ✅ MEASURED on
        // 2026-08-20 by reading WHICH one panics rather than that the probe went red: under 8 and
        // 12 it is the first, the ANSWER (`Mib(8192)` and `Mib(2048)` against `Mib(0)`); under 11
        // the second, the STATE (`1` against `0`); and under 5b and 5d -- the two that mutate
        // `collect_expired` instead of `ask_back` -- the first two PASS and THIS ONE is the only
        // one that fires, `left: Mib(0)` and `left: Mib(6144)` against `right: Mib(8192)`. On
        // those two rows of the campaign it is this assertion that kills the probe, alone.
        //
        // ⚠️ THE CAUSE OF THE OLD CLAIM IS WORTH MORE THAN THE CLAIM. The isolation above samples
        // mutations of `ask_back` ONLY -- precisely the class under which this assertion cannot
        // fail -- and never ran the two rows on which it is load-bearing. An exclusivity measured
        // on a partial sample reads as a guarantee. ⛔ THE REMEDY WAS RIGHT AND THE REASON WRITTEN
        // BESIDE IT WAS WRONG, and wrong by UNDERSTATEMENT: keeping the assertion needs no appeal
        // to a day when `ask_back` might grow a road that touches the books, because the sweep it
        // calls first already has one. Registered as `E82`; the counts as `E81` (`E79`, corrected).
        assert_eq!(arbiter.allocated(), Mib::new(8_192));
    }

    /// ⛔ THE WORST LANE FIRST, and it is the rule the doc of `ask_back` STATES -- "the cheapest
    /// thing to interrupt goes first". What grounds it is design/02's lane table: `interattivo`
    /// is "served before `batch`", and `batch` "may wait indefinitely". A rule written in a
    /// comment and held by nothing is an intention (gotcha #42).
    ///
    /// ⚠️ THE TWO RESERVATIONS DIFFER, AND THAT IS THE WHOLE CONSTRUCTION. `revoking()` counts,
    /// it does not name, so two victims of the same size would be indistinguishable: with
    /// `2_048` in `Batch` and `4_096` in `Interactive` the ANSWER says which one was taken.
    #[test]
    fn asking_back_takes_the_worst_lane_first() {
        let mut arbiter = arbiter(Mib::new(8_192));
        let Admission::Granted(_interactive) = arbiter.admit(
            &preemptible(
                "interactive-resident",
                4_096,
                ComputeClass::Interactive,
                500,
            ),
            LONG,
            Monotonic::ORIGIN,
        ) else {
            panic!("4096 of 8192 fits");
        };
        let Admission::Granted(_batch) = arbiter.admit(
            &preemptible("batch-resident", 2_048, ComputeClass::Batch, 500),
            LONG,
            Monotonic::ORIGIN,
        ) else {
            panic!("4096 + 2048 fits 8192");
        };

        let asked_back =
            arbiter.ask_back(Mib::new(1_024), ComputeClass::Realtime, Monotonic::ORIGIN);

        assert_eq!(
            asked_back,
            Mib::new(2_048),
            "the Batch one, which is the cheapest to interrupt -- not the Interactive one"
        );
        assert_eq!(arbiter.revoking(), 1, "one was enough");
    }

    /// ⛔ THE MARKING PASS RUNS TO THE END OF THE LANES, and this is the direction the probe above
    /// steps over. That one asks for `1_024` against a `Batch` of `2_048`, so the WORST lane
    /// covers the need on its own and the pass never has to leave it; of every other probe here
    /// seven have a single resident, two have two in the SAME lane, and one leaves through
    /// `reclaimable < needed` before any lane is walked at all. So "it goes on into the next lane"
    /// was asked by NOBODY. ✅ MEASURED on 2026-08-20 and not deduced: with `lanes.iter().rev()`
    /// cut to `.take(1)` -- the outer loop stopped after the first lane -- the mutant survived the
    /// WHOLE WORKSPACE, 34 targets, 235 passed, 0 failed, 2 ignored. Registered as `E75`.
    ///
    /// ⛔ AND IT IS NOT A CURIOSITY, IT IS `E69` BY A THIRD ROAD. The reading pass adds up ALL the
    /// lanes, so it can promise `6_144` and let the marking pass hand back `2_048` with the `Batch`
    /// holder ALREADY CONDEMNED -- one evicted and nobody seated, the very damage the two passes
    /// were built to remove.
    ///
    /// ⚠️ AND IT PINS SOMETHING NOTHING ELSE STATES: THE MARKING MAY OVERSHOOT. `2_048` does not
    /// carry `4_096`, so the whole `Interactive` grant goes too and the answer is `6_144` for a
    /// need of `4_096`. The pass stops at the first grant that takes it OVER the line; it does not
    /// hunt for one that lands ON it. Refusing a victim for being too big would leave the asker
    /// unseated with the room standing right there.
    #[test]
    fn asking_back_crosses_into_the_next_lane_when_the_worst_one_is_not_enough() {
        let mut arbiter = arbiter(Mib::new(8_192));
        let Admission::Granted(_interactive) = arbiter.admit(
            &preemptible(
                "interactive-resident",
                4_096,
                ComputeClass::Interactive,
                500,
            ),
            LONG,
            Monotonic::ORIGIN,
        ) else {
            panic!("4096 of 8192 fits");
        };
        let Admission::Granted(_batch) = arbiter.admit(
            &preemptible("batch-resident", 2_048, ComputeClass::Batch, 500),
            LONG,
            Monotonic::ORIGIN,
        ) else {
            panic!("4096 + 2048 fits 8192");
        };

        // `below` is `Realtime`, so BOTH residents are admissible; the worst lane goes first and
        // its `2_048` does not cover the need, so the pass has to carry on into `Interactive`.
        let asked_back =
            arbiter.ask_back(Mib::new(4_096), ComputeClass::Realtime, Monotonic::ORIGIN);

        assert_eq!(
            asked_back,
            Mib::new(6_144),
            "the Batch one did not carry it, so the Interactive one went too"
        );
        assert_eq!(
            arbiter.revoking(),
            2,
            "both: the pass did not stop at the worst lane"
        );
    }

    /// ⛔ ASKING BACK TWICE DOES NOT BUY THE ROOM TWICE, AND DOES NOT EXTEND THE GRACE. This is
    /// what the guard on `activity` buys ON ITS OWN, and nothing else here holds it: a second
    /// pass that re-marked a grant already on its way out would count its reservation into
    /// `covered` a SECOND time -- room the arbiter does not have, which is over-admission by the
    /// back door -- and would push its deadline further out, so the holder would get MORE time
    /// for having been asked earlier.
    #[test]
    fn asking_back_twice_does_not_buy_the_room_twice() {
        let mut arbiter = arbiter(Mib::new(4_096));
        let Admission::Granted(_batch) = arbiter.admit(
            &preemptible("batch-resident", 4_096, ComputeClass::Batch, 500),
            LONG,
            Monotonic::ORIGIN,
        ) else {
            panic!("it fills the machine");
        };
        let first = arbiter.ask_back(
            Mib::new(4_096),
            ComputeClass::Interactive,
            Monotonic::ORIGIN,
        );
        assert_eq!(first, Mib::new(4_096));

        let second = arbiter.ask_back(
            Mib::new(4_096),
            ComputeClass::Interactive,
            Monotonic::from_millis(200),
        );

        assert_eq!(second, Mib::ZERO, "it is already on its way out");
        assert_eq!(arbiter.revoking(), 1, "still one, and not one more");
        // The deadline did not move: it is still `0 + 500`, not `200 + 500`.
        let after = arbiter.admit(
            &profile("the-interactive-one", 4_096, ComputeClass::Interactive),
            LONG,
            Monotonic::from_millis(501),
        );
        assert!(
            matches!(after, Admission::Granted(_)),
            "the second ask must not have handed the holder a fresh grace"
        );
    }

    /// ⛔ `ask_back` COLLECTS THE EXPIRED BEFORE IT MARKS, and "the arbiter collects before it
    /// decides" is a property of EVERY operation -- it is why `collect_expired` is private.
    /// With `ask_back` there are now FOUR of them, and this is the only probe that exercises
    /// this one's line.
    ///
    /// ⚠️ NOTHING IS RELEASED HERE, DELIBERATELY, exactly as in
    /// `promote_collects_the_expired_before_it_serves_the_queue`: the only thing that can empty
    /// the books is the collection INSIDE `ask_back`. Without it the resident is still there,
    /// in a lane below the asking one and perfectly preemptible, and the answer would be
    /// `4_096` instead of nothing.
    #[test]
    fn ask_back_collects_the_expired_before_it_marks() {
        let mut arbiter = arbiter(Mib::new(4_096));
        let Admission::Granted(_short_lived) = arbiter.admit(
            &preemptible("short-lived", 4_096, ComputeClass::Batch, 500),
            Millis::new(5_000),
            Monotonic::ORIGIN,
        ) else {
            panic!("it fills the machine");
        };

        let asked_back = arbiter.ask_back(
            Mib::new(4_096),
            ComputeClass::Interactive,
            Monotonic::from_millis(5_001),
        );

        assert_eq!(
            asked_back,
            Mib::ZERO,
            "there was nothing left to ask back: the window had closed"
        );
        assert_eq!(arbiter.revoking(), 0);
        assert_eq!(arbiter.allocated(), Mib::ZERO);
    }
}
