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
use alloc::vec::Vec;

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
///
/// ⚠️ A SECOND DECLARED LIMIT, AND IT IS THE ONE THAT BITES FIRST. `release` calls
/// `collect_expired` BEFORE it looks, so a grant THIS ARBITER ISSUED whose window has closed
/// is no longer in the books: `remove` answers `None` and the caller gets `UnknownGrant`.
/// ✅ MEASURED on a throwaway probe, not deduced: admitted for 5_000 ms, released at 5_001 ->
/// `Err(UnknownGrant)`; released at 4_999 -> `Ok(Mib(4096))`; released at 5_000 EXACTLY ->
/// `Err(UnknownGrant)` too, because the window is half-open, `[start, expiry)`.
///
/// ⛔ SO TWO CASES ARE CONFLATED IN ONE VARIANT, and THE NAME STILL STATES THE STRONGER OF
/// THE TWO -- which of an expired grant is simply FALSE. That is why the doc line below no
/// longer repeats it. What the guard buys is "that is not in my books, so I will not credit
/// it", and THAT holds in both cases: it is the whole of the over-admission protection. What
/// it does not buy is telling the caller WHICH of the two happened.
///
/// ⚠️ TODAY IT COSTS NOTHING, and the reason is a measurement rather than a hope: `release`
/// has TWO callers in this repository and both are in `tests/arbiter_admission.rs` -- no
/// production consumer exists. It starts costing at milestone 6, where `Worker::kill` hands
/// the grant back when the work FINISHES, which can perfectly well be after the window; there
/// "your release failed" and "it was already done for you" are different news. A second
/// variant `Expired` is the known remedy and it is a DESIGN decision, so it is RECORDED FOR
/// THE OWNER in the plan's errata (`E30`) instead of being taken by the task that noticed it.
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
    /// Not in this arbiter's books. ⚠️ TWO CAUSES, ONE ANSWER -- see the second declared
    /// limit above: the grant came from ANOTHER arbiter, or it was this arbiter's and its
    /// window had already closed.
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
/// and not merely a number it needs -- see the two guards there, which are deliberately about
/// two different questions.
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
    pub const fn new(parameters: Parameters) -> Self {
        Arbiter {
            parameters,
            next_grant: 0,
            next_ticket: 0,
            held: BTreeMap::new(),
            queues: BTreeMap::new(),
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
    /// to a waiting ticket is an ORCHESTRATION decision -- who calls `promote`, and when --
    /// which is task 10's, and closing it here would mean an `admit` that can refuse room that
    /// exists. REGISTERED FOR THE OWNER in the plan's errata, where it sits beside the
    /// permanent-quota voice it interacts with.
    ///
    /// ⛔ AND NOTHING IN THIS REPOSITORY HOLDS THE PARAGRAPH ABOVE -- said here rather than
    /// left to be discovered, because a claim about behaviour that no probe exercises is the
    /// species this task has already paid for once. ✅ MEASURED on 2026-08-19, not deduced:
    /// with `!self.queues.is_empty() ||` added to the second guard below -- so a latecomer
    /// QUEUES BEHIND the waiters instead of jumping them -- NOTHING goes red: 19 passed, 0
    /// failed in `tests/arbiter_admission.rs`, and 34 targets, 222 passed, 0 failed, 2 ignored
    /// across the workspace. THE MUTANT IS ALIVE, so the day task 10 decides the other way this
    /// paragraph becomes FALSE IN SILENCE with nothing going red to say so. ⚖️ AND IT IS NOT
    /// PINNED, ON PURPOSE AND ON THE MERITS: a probe asserting `Granted` for the latecomer would
    /// freeze exactly the choice the errata voice `E51` puts in front of the owner, and a probe
    /// that has to be deleted to take a decision is a vote against taking it -- the precedent is
    /// `E39`, which refused for the same reason to pin the three measured `release` values.
    /// Registered as `E53`.
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
            // ⛔ IT FITS THE MACHINE AND NOT THE MOMENT, so it WAITS. Refusing here would
            // make the answer depend on the instant the request happened to arrive, and
            // §5.3.1 wants the request kept IN ITS OWN LANE instead. The guard above is what
            // keeps "for ever" out of this branch: a request bigger than the whole machine
            // never reaches it.
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
            return Admission::Queued(ticket);
        }

        Admission::Granted(self.issue(profile, valid_for, now))
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
    /// 2 ignored across the workspace. THE MUTANT IS ALIVE, so the day task 7 or task 10 changes
    /// the order across lanes this paragraph becomes FALSE IN SILENCE. ⚖️ AND A PROBE IS NOT
    /// THE REMEDY: pinning the fall-through would freeze the very policy the errata voice `E50`
    /// asks the owner to choose. Same reasoning as `E39`; registered as `E53`.
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
    /// ⛔ WORST LANE FIRST: the cheapest thing to interrupt goes first. What grounds it is the
    /// lane table of design/02, which says of `ComputeClass::Interactive` that it is served
    /// before `ComputeClass::Batch`, and of `Batch` that it may wait indefinitely. What HOLDS it
    /// is `asking_back_takes_the_worst_lane_first`: before that probe the sentence was in a
    /// comment and in nothing else.
    ///
    /// ⛔ IT COLLECTS THE EXPIRED FIRST, like every other operation. With this one there are
    /// FOUR, and the property "the arbiter collects before it decides" is why `collect_expired`
    /// is private rather than a step somebody remembers to take.
    ///
    /// ⚠️ `pub(crate)` BECAUSE ITS ONLY CALLER IS THE ADMISSION UNDER THE LOCAL POLICY. It is
    /// not a public operation: making room is a consequence of a request, never a thing
    /// somebody asks for. ⛔ THAT CALLER IS TASK 8 AND DOES NOT EXIST YET, which would make this
    /// a method with no consumer -- gotcha #46 from the wrong side. What answers it here is the
    /// answer this repository already uses: the consumer is a BENCH, and the ten probes of the
    /// `#[cfg(test)] mod tests` at the foot of this file are it. The `pub(crate)` is what puts
    /// them there instead of in `tests/`.
    pub(crate) fn ask_back(&mut self, needed: Mib, below: ComputeClass, now: Monotonic) -> Mib {
        self.collect_expired(now);

        // ⛔ THE LANES COME FROM THE BOOKS AND THE ORDER FROM `ComputeClass` ITSELF, and neither
        // is a list of the three lanes written out here. `BTreeSet` iterates in key order --
        // which for `ComputeClass` is the hand-written `priority()` of §5.1 -- so `.rev()` IS
        // "worst lane first", and the order stays stated in ONE place. `promote` refuses a
        // hand-written list for exactly this reason and gets its order from its own map for
        // free; `held` is keyed by `GrantId` and not by lane, so this is what buys the same
        // thing here.
        let lanes: BTreeSet<ComputeClass> = self.held.values().map(|held| held.lane).collect();

        let mut covered = Mib::ZERO;
        for lane in lanes.iter().rev() {
            if *lane <= below {
                // Not BELOW the asking lane: a Realtime job is not evicted for an Interactive
                // one, however preemptible its profile says it is.
                continue;
            }
            for held in self.held.values_mut() {
                if covered >= needed {
                    return covered;
                }
                if held.lane != *lane {
                    continue;
                }
                // ⛔ ALREADY ON ITS WAY OUT: leave it alone. Marking it again would hand its
                // holder a FRESH grace for having been asked earlier, and would count its
                // reservation into `covered` a second time -- room the arbiter does not have,
                // which is over-admission by the back door.
                if matches!(
                    held.activity,
                    Activity::Preemptible(PreemptibleState::Revoking { .. })
                ) {
                    continue;
                }
                // ⛔ NO GRACE MEANS NEVER REVOKED, and that is `Preemption::grace`'s own word
                // for it -- `None` "is the statement that this profile is never revoked". So
                // this is the guard that keeps `I2 · §5.3` at runtime, and it is the ONLY one
                // that reads it: the guard above asks a DIFFERENT question, "is it already on
                // its way out".
                //
                // ⚠️ WRITTEN THIS WAY INSTEAD OF THE WAY THE PLAN DICTATES, and the reason is a
                // MEASUREMENT rather than a taste. The dictated body wraps all of this in
                // `if let Activity::Preemptible(PreemptibleState::Running)` and reads the grace
                // with `match held.grace { Some(g) => .., None => continue }`, so BOTH guards
                // answer for the non-preemptible case and each masks the other's mutation. ✅
                // Measured on that very shape, which is green on all thirty probes: deleting
                // the `Running` guard kills `asking_back_twice_does_not_buy_the_room_twice` and
                // ONLY it -- `a_non_preemptible_grant_is_never_asked_back` stays green, saved by
                // the `None` arm -- and turning `None => continue` into `None => now` kills
                // NOTHING AT ALL, 10 passed and 20 passed, because the `Running` guard means
                // that arm is never reached. The non-preemptible direction was therefore held by
                // two guards and provable through neither. In this shape each guard has its own
                // probe and its own SOLE killer. Registered as `E62`.
                let Some(grace) = held.grace else {
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

    fn arbiter(total: Mib) -> Arbiter {
        Arbiter::new(Parameters::new(TURN_LIMIT, total))
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

        let asked_back =
            arbiter.ask_back(Mib::new(4_096), ComputeClass::Interactive, Monotonic::ORIGIN);

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
        let _ = arbiter.ask_back(Mib::new(4_096), ComputeClass::Interactive, Monotonic::ORIGIN);

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
    /// steps over. Without it a sweep that freed a revoked reservation AT ONCE would satisfy
    /// every other probe here: `allocated()` collects nothing by design, and neither `ask_back`
    /// nor `revoking()` runs a sweep -- so an assertion written straight after an `ask_back`
    /// passes through no sweep at all and cannot fail. The only way to catch it is to make
    /// somebody SWEEP while the grace is still running, and `admit` is that somebody.
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
        let _ = arbiter.ask_back(Mib::new(4_096), ComputeClass::Interactive, Monotonic::ORIGIN);

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
        let _ = arbiter.ask_back(Mib::new(4_096), ComputeClass::Interactive, Monotonic::ORIGIN);

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

        let asked_back =
            arbiter.ask_back(Mib::new(4_096), ComputeClass::Interactive, Monotonic::ORIGIN);

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

        let asked_back =
            arbiter.ask_back(Mib::new(4_096), ComputeClass::Interactive, Monotonic::ORIGIN);

        assert_eq!(asked_back, Mib::ZERO, "Realtime is not below Interactive");
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

        let asked_back =
            arbiter.ask_back(Mib::new(4_096), ComputeClass::Interactive, Monotonic::ORIGIN);

        assert_eq!(asked_back, Mib::new(4_096));
        assert_eq!(arbiter.revoking(), 1, "one was enough");
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
            &preemptible("interactive-resident", 4_096, ComputeClass::Interactive, 500),
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
        let first = arbiter.ask_back(Mib::new(4_096), ComputeClass::Interactive, Monotonic::ORIGIN);
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
