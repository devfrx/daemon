//! The DST campaign of the arbiter (§5.7). Three of the five properties: the two that need
//! `process` and `ipc` are milestone 6.
//!
//! ⛔ THE ARBITER UNDER TEST IS THE REAL ONE. There is no fake: the arbiter is logic, not a
//! port, so in simulation the shipped object runs. What is injected comes from elsewhere --
//! the INTERLEAVING from the seed, through `SeededRng` and the executor's shuffle; the
//! PASSAGE OF TIME from `VirtualReactor`, which the activities read through `SharedClock`;
//! the CRASH from `CrashingJournal`.
//!
//! ⛔ AND NON-VACUITY IS MANDATORY HERE, §5.7.1 says so in those words. Two claims, two
//! oracles: THE INJECTION FIRED, and THERE WAS SOMETHING TO VERIFY.
//!
//! ⛔ THE SCENARIO DIVERGES FROM THE ONE THE PLAN DICTATED, AND THE REASON IS A MEASUREMENT.
//! The dictated one derived `now` from the LOOP INDEX, gave all three activities the same
//! reservation -- telling them apart by LANE alone, which under `RemotePolicy` decides only
//! which queue a ticket lands in -- and never handed a grant back. Measured on 2026-08-25, it
//! produced ONE distinct outcome over twenty thousand seeds, `granted 4, queued 8, refused 0,
//! peak 6144`, which is precisely what `the_campaign_sweeps_more_than_one_world` exists to
//! shout about: three activities identical in everything the BOOKS can see are SYMMETRIC, so
//! permuting them changes WHO is served and not HOW MANY. The rewrite, and the figures either
//! side of it, are voice `E144` of the milestone 5 plan and the task 12 section of
//! `docs/porta-di-qualita.md`.
//!
//! ⚠️ WHAT THIS CAMPAIGN DOES NOT HOLD, said rather than left to be assumed. That releasing
//! gives back EXACTLY the reservation is `releasing_gives_back_exactly_the_reservation` in
//! `crates/kernel/tests/arbiter_admission.rs`: here `release` is exercised under the
//! interleaving, and what is asserted about it is the ceiling and not the amount. That a
//! request bigger than the whole machine is refused is
//! `a_request_larger_than_the_total_is_refused_and_not_queued` in the same bench: no profile
//! in `PARTIES` asks more than `TOTAL`, so this scenario does not reach `Admission::Refused`
//! -- and `the_scenario_really_makes_the_admission_decide` asserts that it does not, so the
//! sentence is held instead of merely written.

use core::cell::RefCell;
use std::collections::BTreeSet;

use kernel::arbiter::{
    Admission, Arbiter, ComputeClass, Grant, LocalPolicy, Mib, Preemption, ReleaseError,
    RemotePolicy, ResourceProfile, VramPolicy,
};
use kernel::executor::{Executor, Sleep};
use kernel::parameters::Parameters;
use kernel::ports::journal::StepId;
use kernel::ports::reactor::Reactor;
use kernel::reconcile::{Resolution, steps_in_doubt};
use kernel::time::{Millis, Monotonic, WallTime};
use simulator::journal::CrashingJournal;
use simulator::reactor::VirtualReactor;
use simulator::rng::SeededRng;

const TURN_LIMIT: u64 = 10_000;

/// How many requests each party makes.
///
/// ⚠️ FOUR, AND THE NUMBER WAS MEASURED RATHER THAN PICKED. A longer scenario buys almost
/// nothing here: measured on 2026-08-25 over `SHORT_CAMPAIGN_SEEDS` seeds, the outcome space
/// is 7 distinct outcomes at four requests, 8 at six and 9 at eight, while one sweep grows
/// from 369 ms to 578 ms to 750 ms. The space is bounded by the SHAPE of the scenario -- the
/// parties, their sizes and their rhythms -- and not by its length.
const REQUESTS: usize = 4;

/// The whole machine. ⛔ THE PARTIES ARE SIZED AGAINST IT SO THAT NOT EVERYTHING FITS, which
/// is what makes the admission decide instead of always saying yes -- gotcha #17. The four
/// reservations add up to 10 240 against a machine of 8 192, and `voice` + `chat` + `render`
/// is 8 192 EXACTLY -- so the ceiling is REACHABLE, and the guard `allocated + asked >
/// ceiling` is exercised at equality rather than only well inside or well outside it.
/// ⚠️ Which three fit is not uniform, and that is the point rather than an accident:
/// `chat` + `render` + `index` is 9 216 and does not.
const TOTAL: Mib = Mib::new(8_192);

/// How long the bench keeps a grant it took OUT OF THE QUEUE. ⚠️ It is one number for all
/// lanes because the promotion does not say whose ticket it was in a way this bench keeps:
/// `Promotion` carries the ticket and the grant, and the bench does not index the parties by
/// ticket. Registered rather than papered over -- it is a limit of the bench, not of
/// `promote`.
const PROMOTED_HOLD: Millis = Millis::new(1_000);

/// The step the policy transition is journalled under.
const TRANSITION_STEP: u64 = 900;

/// How many records one transition writes: the intent and the outcome -- the write-ahead
/// discipline of ADR-0007. ⛔ IT IS THE PREMISE `crash_point` RESTS ON, and it is held rather
/// than trusted: the fall point is drawn modulo this number, so a transition that wrote fewer
/// records would leave the tail of the range firing on nothing, and
/// `property_4_a_severed_transition_leaves_a_reconcilable_step` asserts that EVERY seed
/// reaches its point. It is the shape `WRITES_PER_RUN` has in `dst_campaign.rs`, for the same
/// reason.
const WRITES_PER_TRANSITION: u64 = 2;

/// How many seeds the SHORT campaign sweeps. ⛔ FIXED AND VERSIONED WITH THIS FILE, never
/// drawn from the clock or from an environment variable: constraint 7 of §11, so two runs of
/// the gate sweep the same seeds.
///
/// ⛔ THE CRITERION IS THE CLOSURE OF THE OUTCOME SPACE and not "the largest round number
/// under a ceiling", which chases a figure that saturates. Measured on 2026-08-25, in `debug`
/// and on one machine, sweeping `0..seeds`:
///
/// | seeds  | distinct outcomes | last new one at seed | wall time |
/// |--------|-------------------|----------------------|-----------|
/// | 200    | 7                 | 38                   | 36 ms     |
/// | 500    | 7                 | 38                   | 88 ms     |
/// | 1 000  | 7                 | 38                   | 189 ms    |
/// | 2 000  | 7                 | 38                   | 369 ms    |
/// | 20 000 | 7                 | 38                   | 3 690 ms  |
///
/// The space is closed by seed 38 and 19 962 further seeds produce nothing new. This number
/// closes it with a margin of about fifty times, and costs four sweeps.
///
/// ⛔ AND THE BUDGET IS MEASURED IN `debug`, NOT IN `--release`: `gate.sh` runs
/// `cargo test --workspace` with no `--release`, so debug is the profile that pays. One run
/// of the scenario costs ~185 µs there.
///
/// ⚠️ THE TWO COLUMNS ARE NOT WORTH THE SAME, and saying so beats letting the figures look
/// equally solid. The distinct count and the seed the last one arrives at came out IDENTICAL
/// on every repetition; the wall time did not. The whole binary, same command
/// (`--test-threads=1`), was measured at 0.63 s early in the session and 1.53 s later -- a
/// factor of 2.4 on the same machine, within one session, on the same bytes. So the times
/// here are an ORDER OF MAGNITUDE and not a constant, which is also why nothing asserts on
/// them: what the gate collects is the printed line, for a reader to compare against the run
/// before.
const SHORT_CAMPAIGN_SEEDS: u64 = 2_000;

/// How many DISTINCT outcomes this scenario can produce at all.
///
/// ⛔ IT IS THE ONE NUMBER THAT TURNS THE CHOICE OF `SHORT_CAMPAIGN_SEEDS` INTO SOMETHING
/// HELD. Without it the reason for that number -- "this many seeds see the whole space" --
/// would be a sentence in a comment that no run can contradict. It is a change detector on
/// the SHAPE of the scenario, and it fires in both directions: a scenario that grew, and a
/// seed count that no longer reaches the end of the space. Same posture as
/// `EXPECTED_DOUBT_SETS` in `dst_campaign.rs`.
///
/// ⛔ AND IT WAS NOT ADOPTED UNTIL IT WAS MEASURED NOT TO FIRE WHERE IT SHOULD NOT, which is
/// gotcha #24: a check that cries on an innocent change teaches everyone to ignore the gate.
/// The question is whether 7 belongs to the SCENARIO or to the seeds that sample it. Measured
/// on 2026-08-25 by drawing the 2 000 seeds through four different samplings:
///
/// | how the seed is drawn        | distinct outcomes |
/// |------------------------------|-------------------|
/// | `seed` itself                | 7                 |
/// | `seed * 0xBF58476D1CE4E5B9 >> 33` | 7            |
/// | `seed * 0x94D049BB133111EB >> 33` | 7            |
/// | `seed * 0x2545F4914F6CDD1D >> 33` | 7            |
///
/// ⚠️ WHAT INVALIDATES IT, named so that a red is read as a decision and not as a defect: a
/// change to `PARTIES`, to `REQUESTS` or to `TOTAL`. When that red arrives the right move is
/// to re-measure the space and re-choose these two numbers against it, not to edit this one
/// until the bar turns green.
///
/// ⛔ RECALL OF 2026-08-25, FIRST WAVE OF CORRECTIONS -- THAT LIST ENDED WITH "OR TO THE
/// ARBITER'S OWN BEHAVIOUR", AND THOSE FOUR WORDS ARE TAKEN OUT RATHER THAN REPHRASED. What
/// this constant compares is the CARDINALITY of the outcome space, so a change to the arbiter
/// that leaves the cardinality where it is walks straight past it. ✅ MEASURED and not reasoned:
/// with `Arbiter::promote` returning `Vec::new()` on every call,
/// `the_campaign_sweeps_more_than_one_world` PASSES and still prints seven distinct outcomes,
/// while the counter `property_5` prints in the same run MOVES -- the seeds on which the sweep
/// made room go from 1835 to 1833.
///
/// ⛔ WHAT IT DOES HOLD is the choice of `SHORT_CAMPAIGN_SEEDS` and the SHAPE of the scenario:
/// a sweep that collects nothing, a sweep the admission no longer performs, and four identical
/// parties. All three turn it red, and all three COLLAPSE the space instead of permuting it.
const EXPECTED_OUTCOMES: usize = 7;

/// The clock the executor advances AND the activities read.
///
/// ⛔ IT EXISTS BECAUSE `Executor::new` TAKES ITS REACTOR BY VALUE, and an activity holds no
/// reactor of its own -- `Sleep::until` says so in as many words. Without a shared handle the
/// only instant an activity can name is one it computes itself, which is what the dictated
/// scenario did and why its interleaving never reached the arbiter.
///
/// ⚠️ IT ADDS NO BEHAVIOUR, and that is worth stating: every method forwards to the
/// `VirtualReactor` inside, so the clock under test is still the shipped fake and not a
/// bespoke one. What the wrapper adds is a `RefCell` and nothing else.
struct SharedClock<'a> {
    inner: &'a RefCell<VirtualReactor>,
}

impl Reactor for SharedClock<'_> {
    fn now(&self) -> Monotonic {
        self.inner.borrow().now()
    }

    fn wall_time(&self) -> WallTime {
        self.inner.borrow().wall_time()
    }

    fn wait_until(&mut self, deadline: Monotonic) -> Option<Monotonic> {
        self.inner.borrow_mut().wait_until(deadline)
    }
}

/// One synthetic requester: what it asks for, and its rhythm.
///
/// ⛔ THE PARTIES DIFFER IN RESERVATION, LANE, VALIDITY WINDOW AND RHYTHM, and that is the
/// whole reason the interleaving reaches the arbiter. Identical parties are symmetric:
/// permuting them changes which one is served, never how many are, so the outcome is the same
/// on every seed.
///
/// ⚠️ `preemption` IS THE ONE FIELD THEY SHARE, and it is inert here rather than uniform by
/// oversight: the campaign runs under `VramPolicy::Remote`, whose `may_make_room()` answers
/// `false`, so `ask_back` never marks anybody and no grant reaches `Revoking` -- the only
/// state the grace time is read in. The revocation has its own probes in the `#[cfg(test)]`
/// module of `crates/kernel/src/arbiter/mod.rs`.
struct Party {
    profile: ResourceProfile,
    /// How long the requester declares it needs the reservation for.
    valid_for: Millis,
    /// How long the bench keeps the grant before handing it back. ⛔ IT IS DELIBERATELY
    /// LONGER THAN `valid_for` FOR SOME PARTIES: a holder that overruns its own window finds
    /// the reservation already collected, which is property 5 observed from the outside.
    hold: Millis,
    /// How long the requester waits after being granted, before asking again.
    after_grant: Millis,
    /// And after being told to wait. ⛔ THE TWO DIFFER, AND THAT IS WHAT MAKES THE CLOCK
    /// SEED-DEPENDENT: a party's timeline follows the answers it got, the answers follow the
    /// interleaving, and the interleaving follows the seed. With one pause for both, every
    /// instant would be a function of the loop index again.
    after_queue: Millis,
}

const fn party(
    name: &'static str,
    reserved: u64,
    lane: ComputeClass,
    valid_for: u64,
    hold: u64,
    after_grant: u64,
    after_queue: u64,
) -> Party {
    Party {
        profile: ResourceProfile {
            name,
            reserved_vram: Mib::new(reserved),
            compute_class: lane,
            preemption: Preemption::After(Millis::new(500)),
        },
        valid_for: Millis::new(valid_for),
        hold: Millis::new(hold),
        after_grant: Millis::new(after_grant),
        after_queue: Millis::new(after_queue),
    }
}

/// ⚠️ A `static` AND NOT A `const`, and it is not a taste: the activities capture
/// `&'static Party`, and `&SOME_CONST` reaches that lifetime only through rvalue static
/// promotion -- a rule about what the value contains rather than about what is written. A
/// `static` says it outright.
static PARTIES: [Party; 4] = [
    party("voice", 1_024, ComputeClass::Realtime, 600, 900, 700, 400),
    party(
        "chat",
        3_072,
        ComputeClass::Interactive,
        2_500,
        1_100,
        1_100,
        600,
    ),
    party(
        "render",
        4_096,
        ComputeClass::Batch,
        1_200,
        1_800,
        1_300,
        900,
    ),
    party("index", 2_048, ComputeClass::Batch, 1_400, 900, 900, 500),
];

/// What one run of the scenario observed. ⛔ IT IS THE INDEPENDENT ORACLE, and it comes from
/// the ACTIVITIES rather than from the arbiter: the properties below walk what the activities
/// were told happened. An arbiter that lied about its own totals would make the two disagree.
///
/// ⛔ IT IS ALSO THE POINT IN THE OUTCOME SPACE, which is why it derives `Ord`: the campaign
/// counts how many DISTINCT ones a sweep produces, and that count is what
/// `the_campaign_sweeps_more_than_one_world` reads.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Observed {
    /// The three answers of `admit`, counted separately so that "every request got exactly
    /// one answer" is an addition and not a hope.
    granted: usize,
    queued: usize,
    refused: usize,
    /// Grants issued OUT OF THE QUEUE. ⛔ It is a second place grants are born, with a
    /// ceiling guard of its own, so property 1 covers `promote` and not only `admit`.
    promoted: usize,
    /// Hand-backs the arbiter accepted.
    released: usize,
    /// Hand-backs that found the grant ALREADY OFF THE BOOKS -- `ReleaseError::UnknownGrant`.
    /// ⛔ IT IS A WITNESS OF PROPERTY 5 seen from the holder's side. This scenario builds ONE
    /// arbiter and hands each grant back at most once, so the reading this bench takes from
    /// that error is that the sweep had already removed the grant.
    already_collected: usize,
    /// Admissions that were GRANTED although the books, read in the same breath, had no room
    /// for them.
    ///
    /// ⛔ IT IS THE OTHER WITNESS OF PROPERTY 5, and the stronger one: `admit` collects the
    /// expired FIRST and then decides, so a request that did not fit `allocated()` and came
    /// back `Granted` was seated on room the sweep had just freed. ⚠️ THE SNAPSHOT IS EXACT
    /// because there is no `.await` between reading `allocated()` and calling `admit`, and
    /// §2.4.1 makes an await the only place another activity can run.
    room_from_expiry: usize,
    /// The largest `allocated()` seen at ANY point at which it was observable.
    peak: Mib,
}

/// ⚠️ WRITTEN OUT BECAUSE `Mib` HAS NO `Default`, AND THAT IS DELIBERATE ON `Mib`'s SIDE: the
/// resource is quantised and its identity is NAMED -- `Mib::ZERO`, "the identity of
/// `saturating_add`" -- so `#[derive(Default)]` here does not compile (`E0277`). The plan
/// dictated the derive; voice `E143`.
impl Default for Observed {
    fn default() -> Self {
        Observed {
            granted: 0,
            queued: 0,
            refused: 0,
            promoted: 0,
            released: 0,
            already_collected: 0,
            room_from_expiry: 0,
            peak: Mib::ZERO,
        }
    }
}

/// The scenario: the four parties of `PARTIES`, interleaved, each asking `REQUESTS` times.
///
/// One turn of a party is: hand back whatever the machine is done with, serve the queue with
/// the room that returned, ask for its own reservation, then sleep -- for one delay if it was
/// granted and a different one if it was told to wait.
///
/// ⛔ THE INSTANT COMES FROM THE VIRTUAL CLOCK AND NOT FROM THE LOOP INDEX, which is the
/// whole of `E144`. Because the two delays differ, a party's timeline follows the answers it
/// received, so the instant it hands to `admit` is a function of the seed and not of the
/// iteration it is on.
///
/// ⛔ EVERY BOOK-MOVING OPERATION IS FOLLOWED BY `watch`, and that is what makes property 1
/// an assertion about EVERY POINT AT WHICH THE SUM IS OBSERVABLE rather than about the end of
/// the run. A check only at the end is green for an arbiter that over-admits and then tidies
/// up.
///
/// ⛔ THE RUNNING SET IS THE BENCH'S AND NOT THE ARBITER'S, and the order inside a turn --
/// release, then promote, then admit -- is a CHOICE OF THIS BENCH. There is no orchestration
/// cycle in this repository to take it from: who calls `promote` relative to `admit` is the
/// open owner voice `E51`/`E53`, and nothing here asserts on that order, so it stays open.
///
/// ⛔ AND THE JOURNALLED TRANSITION HAPPENS AFTER THE INTERLEAVING, deliberately: it is a
/// decision of the composition root, not of an activity, and property 4 is about the CRASH
/// and not about the concurrency.
fn run(seed: u64, journal: CrashingJournal) -> (CrashingJournal, Observed) {
    let arbiter = RefCell::new(Arbiter::new(
        Parameters::new(TURN_LIMIT, TOTAL),
        VramPolicy::Remote(RemotePolicy),
    ));
    let observed = RefCell::new(Observed::default());
    let journal = RefCell::new(journal);
    let clock = RefCell::new(VirtualReactor::new());
    let running: RefCell<Vec<(Grant, Monotonic)>> = RefCell::new(Vec::new());
    let sleep = Sleep::new();

    let mut executor = Executor::new(
        SeededRng::new(seed),
        SharedClock { inner: &clock },
        Parameters::new(TURN_LIMIT, TOTAL),
        &sleep,
    );

    for party in &PARTIES {
        let arbiter = &arbiter;
        let observed = &observed;
        let clock = &clock;
        let running = &running;
        let sleep = &sleep;
        executor.spawn(async move {
            for _request in 0..REQUESTS {
                let now = clock.borrow().now();

                let due = take_due(running, now);
                let handed_back = !due.is_empty();
                for grant in due {
                    match arbiter.borrow_mut().release(grant, now) {
                        Ok(_) => observed.borrow_mut().released += 1,
                        Err(ReleaseError::UnknownGrant) => {
                            observed.borrow_mut().already_collected += 1;
                        }
                    }
                    watch(seed, arbiter, observed);
                }

                if handed_back {
                    let promotions = arbiter.borrow_mut().promote(now);
                    observed.borrow_mut().promoted += promotions.len();
                    watch(seed, arbiter, observed);
                    for promotion in promotions {
                        running
                            .borrow_mut()
                            .push((promotion.grant, now.saturating_add(PROMOTED_HOLD)));
                    }
                }

                let before = arbiter.borrow().allocated();
                let outcome = arbiter
                    .borrow_mut()
                    .admit(&party.profile, party.valid_for, now);
                let granted = matches!(outcome, Admission::Granted(_));
                {
                    let mut observed = observed.borrow_mut();
                    match outcome {
                        Admission::Granted(_) => {
                            observed.granted += 1;
                            if before.saturating_add(party.profile.reserved_vram) > TOTAL {
                                observed.room_from_expiry += 1;
                            }
                        }
                        Admission::Queued(_) => observed.queued += 1,
                        Admission::Refused { .. } => observed.refused += 1,
                    }
                }
                watch(seed, arbiter, observed);

                if let Admission::Granted(grant) = outcome {
                    running
                        .borrow_mut()
                        .push((grant, now.saturating_add(party.hold)));
                }

                sleep.until(now.saturating_add(if granted {
                    party.after_grant
                } else {
                    party.after_queue
                }));
                Yield::once().await;
            }
        });
    }

    executor.run().expect("the scenario terminates");
    drop(executor);

    let _ = arbiter.borrow_mut().set_policy(
        VramPolicy::Local(LocalPolicy),
        StepId::new(TRANSITION_STEP),
        &mut *journal.borrow_mut(),
    );

    (journal.into_inner(), observed.into_inner())
}

/// Takes out of the running set everything the machine is done with at `now`, and leaves the
/// rest. ⚠️ It returns the grants rather than releasing them itself because `release` wants
/// the arbiter, and holding a borrow of the running set across that call is what a `RefCell`
/// panics on.
fn take_due(running: &RefCell<Vec<(Grant, Monotonic)>>, now: Monotonic) -> Vec<Grant> {
    let mut running = running.borrow_mut();
    let mut due = Vec::new();
    let mut kept = Vec::new();
    for (grant, until) in running.drain(..) {
        if until <= now {
            due.push(grant);
        } else {
            kept.push((grant, until));
        }
    }
    *running = kept;
    due
}

/// ⛔ PROPERTY 1, ASSERTED WHERE IT IS OBSERVABLE, and it is the load-bearing assertion of
/// this file: `property_1_the_sum_never_exceeds_the_total_on_any_seed` is what sweeps the
/// seeds so that THIS line gets its chances. The seed is in the message because §5.7.1 wants
/// the campaign to NAME THE SEED when it fails.
fn watch(seed: u64, arbiter: &RefCell<Arbiter>, observed: &RefCell<Observed>) {
    let allocated = arbiter.borrow().allocated();
    assert!(
        allocated <= TOTAL,
        "seed {seed}: allocated {allocated:?} exceeds the total {TOTAL:?}"
    );
    let mut observed = observed.borrow_mut();
    if allocated > observed.peak {
        observed.peak = allocated;
    }
}

/// A future that returns `Pending` exactly once.
///
/// ⚠️ DUPLICATED WORD FOR WORD from `dst_campaign.rs`, and it is declared rather than left to
/// be discovered: TEST CODE DOES NOT CROSS CRATE BOUNDARIES, so there is no place both
/// benches could reach. Unifying them would put a test helper on the wire of a shipped crate.
struct Yield(bool);

impl Yield {
    fn once() -> Self {
        Yield(false)
    }
}

impl core::future::Future for Yield {
    type Output = ();
    fn poll(
        mut self: core::pin::Pin<&mut Self>,
        _context: &mut core::task::Context<'_>,
    ) -> core::task::Poll<()> {
        if self.0 {
            core::task::Poll::Ready(())
        } else {
            self.0 = true;
            core::task::Poll::Pending
        }
    }
}

/// The fall point for a seed. ⛔ `WRITES_PER_TRANSITION` WRITES PER RUN -- the intent and the
/// outcome -- so the space of fall points has cardinality two, and both members are
/// interesting: 0 falls before the intent and leaves no doubt, 1 falls between the two and
/// leaves one.
///
/// ⛔ IT COMES FROM A DERIVED MIXING AND NOT FROM THE SAME GENERATOR AS THE INTERLEAVING: two
/// `SeededRng` built from the same number give the SAME sequence, so the campaign would
/// explore a DIAGONAL of the space instead of the space (decision D2 of the milestone 4
/// plan).
fn crash_point(seed: u64) -> u64 {
    (seed.wrapping_mul(0xBF58_476D_1CE4_E5B9) >> 33) % WRITES_PER_TRANSITION
}

/// The smallest reservation any party asks for. Read from `PARTIES` rather than written down,
/// so that changing a party cannot leave this figure behind.
fn smallest_reservation() -> Mib {
    PARTIES
        .iter()
        .map(|party| party.profile.reserved_vram)
        .min()
        .expect("the scenario has at least one party")
}

/// ⛔ ORACLE ONE -- THE SCENARIO REALLY DOES WHAT THE CAMPAIGN ASSUMES. Without it, a scenario
/// whose requests all fitted would sweep a world where the admission never says no, and every
/// property below would be green having decided nothing. Gotcha #17.
///
/// ⛔ IT HOLDS THE WHOLE SHAPE ON ONE SEED, and each line is a different assumption the tests
/// below rest on: that every request is answered, that the admission both grants and refuses
/// room, that grants come back, that the queue is served, and that a holder can overrun its
/// own window. A sweep is not needed for any of them -- measured on 2026-08-25, all five hold
/// on every one of the 2 000 seeds.
#[test]
fn the_scenario_really_makes_the_admission_decide() {
    let (_journal, observed) = run(20_260_818, CrashingJournal::without_crash());

    assert_eq!(
        observed.granted + observed.queued + observed.refused,
        PARTIES.len() * REQUESTS,
        "every request got exactly one answer"
    );
    assert!(observed.granted > 0, "nothing was ever granted");
    assert!(
        observed.queued > 0,
        "everything fitted: the admission never had to say no, so this campaign proves nothing"
    );
    assert_eq!(
        observed.refused, 0,
        "a request was refused: no profile in PARTIES asks more than TOTAL, so either one now \
         does or the ceiling moved"
    );
    assert!(
        observed.released > 0,
        "no grant was ever handed back: `release` is not being exercised"
    );
    assert!(
        observed.promoted > 0,
        "nothing was ever taken out of the queue: `promote` is not being exercised"
    );
    assert!(
        observed.already_collected > 0,
        "no holder ever overran its window: the sweep is not being exercised from this side"
    );
    assert!(
        observed.peak > Mib::ZERO,
        "the books never moved: the arbiter was never asked anything"
    );
}

/// ⛔ ORACLE TWO -- THERE WAS SOMETHING TO VERIFY, and it is a DIFFERENT claim from oracle
/// one. This is the lesson milestone 4 learned three times, each time AFTER closing the
/// previous one: a campaign that holds only "the injection fired" is green having compared
/// empty sets.
///
/// ⚠️ WHAT IT MEASURES IS THE OUTCOME SPACE: how many DISTINCT `Observed` the sweep produces.
/// One distinct outcome across two thousand seeds means the interleaving changes nothing and
/// the campaign is one run repeated -- which is what the scenario the plan dictated actually
/// did, measured, before `E144` replaced it.
#[test]
fn the_campaign_sweeps_more_than_one_world() {
    let started = std::time::Instant::now();
    let mut distinct = BTreeSet::new();
    for seed in 0..SHORT_CAMPAIGN_SEEDS {
        let (_journal, observed) = run(seed, CrashingJournal::without_crash());
        distinct.insert(observed);
    }
    let elapsed = started.elapsed();

    assert!(
        distinct.len() > 1,
        "every seed produced the SAME outcome: the interleaving is not reaching the arbiter, \
         so this campaign is one run repeated {SHORT_CAMPAIGN_SEEDS} times"
    );
    assert_eq!(
        distinct.len(),
        EXPECTED_OUTCOMES,
        "the campaign saw {} of the {EXPECTED_OUTCOMES} outcomes this scenario can produce — \
         either the scenario changed shape, or a sweep of {SHORT_CAMPAIGN_SEEDS} seeds no \
         longer reaches the end of the space and both numbers must be re-measured",
        distinct.len()
    );

    println!(
        "DST arbiter worlds: {} distinct outcomes over {SHORT_CAMPAIGN_SEEDS} seeds, {elapsed:?}",
        distinct.len()
    );
}

/// ⛔ PROPERTY 1 (§5.7): the sum of ALL grants never exceeds the total, at every point at
/// which it is observable. The assertion lives in `watch`, fired after every operation that
/// moves the books -- `release`, `promote` and `admit` -- of every party; this test is what
/// sweeps the seeds and reports which one broke.
///
/// ⛔ AND THIS IS PROPERTY 1's OWN NON-VACUITY, which the sweep is not. A sweep in which the
/// books never came near the ceiling would check the sum against a machine with room to
/// spare, and be green for an arbiter that admits anything up to twice the total. What is
/// asserted is that on at least one seed the books got within LESS THAN THE SMALLEST
/// RESERVATION of the ceiling -- so the guard was actually load-bearing somewhere.
///
/// ⚠️ THE PLAN DICTATED TWO ASSERTIONS HERE AND NEITHER COULD FAIL: `peak <= TOTAL` reads a
/// value that `watch` has already compared against `TOTAL`, and `worlds == SHORT_CAMPAIGN_SEEDS`
/// counts the turns of a `for` over that range. Voice `E147`.
#[test]
fn property_1_the_sum_never_exceeds_the_total_on_any_seed() {
    let started = std::time::Instant::now();
    let mut highest = Mib::ZERO;
    for seed in 0..SHORT_CAMPAIGN_SEEDS {
        let (_journal, observed) = run(seed, CrashingJournal::without_crash());
        if observed.peak > highest {
            highest = observed.peak;
        }
    }
    let elapsed = started.elapsed();

    let smallest = smallest_reservation();
    assert!(
        highest.saturating_add(smallest) > TOTAL,
        "the books never came within {smallest:?} of {TOTAL:?} — highest was {highest:?}, so \
         the ceiling was never binding and the sum was compared against a machine with room \
         to spare"
    );

    println!(
        "DST arbiter ceiling: highest {highest:?} of {TOTAL:?} over {SHORT_CAMPAIGN_SEEDS} \
         seeds, {elapsed:?}"
    );
}

/// ⛔ PROPERTY 5 (§5.7): an expired grant does not stay allocated. The injection is the
/// VIRTUAL CLOCK -- `hold` outruns `valid_for` for two of the four parties, so grants expire
/// UNDER the scenario rather than in a constructed state.
///
/// ⛔ TWO WITNESSES AND NOT ONE, and they come apart. `already_collected` is the holder's
/// side: it came back for a grant and the arbiter no longer had it. `room_from_expiry` is the
/// requester's side: a request that did not fit the books as they stood was granted anyway,
/// which `admit` can only do because it collects before it decides. The first says the grant
/// LEFT; the second says the budget it freed was REUSED. A campaign holding only the first
/// would be green for an arbiter that forgot expired grants without giving the room back.
#[test]
fn property_5_expiry_frees_the_budget_under_the_scenario() {
    let started = std::time::Instant::now();
    let mut seeds_where_a_holder_was_too_late = 0usize;
    let mut seeds_where_the_sweep_made_room = 0usize;
    for seed in 0..SHORT_CAMPAIGN_SEEDS {
        let (_journal, observed) = run(seed, CrashingJournal::without_crash());
        if observed.already_collected > 0 {
            seeds_where_a_holder_was_too_late += 1;
        }
        if observed.room_from_expiry > 0 {
            seeds_where_the_sweep_made_room += 1;
        }
    }
    let elapsed = started.elapsed();

    assert!(
        seeds_where_a_holder_was_too_late > 0,
        "on no seed did a holder outrun its own window: the sweep never took a grant off the \
         books, so the expiry is not being exercised at all"
    );
    assert!(
        seeds_where_the_sweep_made_room > 0,
        "on no seed was a request seated on room the sweep had just freed: grants may be \
         leaving the books without their reservation coming back"
    );

    println!(
        "DST arbiter expiry: {seeds_where_a_holder_was_too_late} seeds with a late holder, \
         {seeds_where_the_sweep_made_room} where the sweep made room, {elapsed:?}"
    );
}

/// ⛔ PROPERTY 4 (§5.7): a policy transition cut in half leaves a step IN DOUBT, reconcilable
/// like every other. The injection is the `journal` port -- the crashing journal of milestone
/// 4 -- and the fall point is drawn from a generator DERIVED from the seed.
///
/// ⛔ TWO ORACLES AND NOT ONE: `crashes` says the injection fired, `doubted` says there was
/// something to compare. A run that fell BEFORE the intent leaves no doubt and no bug -- and a
/// campaign that counted only the falls would be green having compared empty sets.
///
/// ⛔ THE FIRST IS AN EQUALITY AND NOT `> 0`, which is the shape `c7b` uses in
/// `dst_campaign.rs` and for the same reason: the point is drawn inside
/// `0..WRITES_PER_TRANSITION` and the transition writes exactly that many records, so every
/// seed must reach its point. A single seed that did not crash would mean the transition
/// wrote fewer times than the number the point was drawn against, which is the silent no-op
/// of gotcha #17; `> 0` would let all but one seed go quiet.
#[test]
fn property_4_a_severed_transition_leaves_a_reconcilable_step() {
    let started = std::time::Instant::now();
    let mut crashes = 0u64;
    let mut doubted = 0usize;

    for seed in 0..SHORT_CAMPAIGN_SEEDS {
        let (journal, _observed) = run(seed, CrashingJournal::falling_at(crash_point(seed)));
        if !journal.has_fallen() {
            continue;
        }
        crashes += 1;

        let survivor = journal.into_survivor();
        let doubts = steps_in_doubt(&survivor).expect("the archive reads back");
        for doubt in &doubts {
            assert_eq!(
                doubt.step,
                StepId::new(TRANSITION_STEP),
                "seed {seed}: a step other than the transition is in doubt"
            );
            assert_eq!(
                doubt.resolution,
                Resolution::RunAgain,
                "seed {seed}: a policy transition is idempotent"
            );
        }
        if !doubts.is_empty() {
            doubted += 1;
        }
    }
    let elapsed = started.elapsed();

    assert_eq!(
        crashes, SHORT_CAMPAIGN_SEEDS,
        "a seed did not reach its crash point: the transition wrote fewer times than \
         {WRITES_PER_TRANSITION}"
    );
    assert!(
        doubted > 0,
        "every crash fell before the intent: {crashes} crashes and ZERO doubts, so the \
         comparison was between empty sets on every one of them"
    );

    println!(
        "DST arbiter crashes: {crashes}/{SHORT_CAMPAIGN_SEEDS} seeds crashed, {doubted} with a \
         step in doubt, {elapsed:?}"
    );
}
