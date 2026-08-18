// THE CONFORMANCE SUITE OF THE `reactor` PORT (§7.4.6). It is the most important one in the
// project: THE VALIDITY OF THE DETERMINISTIC SIMULATION RESTS HERE, because the fake is not
// the real one. Every campaign run against `VirtualReactor` is worth exactly as much as the
// evidence that the two implementations answer the same contract.
//
// ⛔ REGULAR COMMENTS AND NOT `//!`, and that is not a slip. This file is `include!`d by
// `crates/platform/tests/reactor_contract_real.rs`, where it is expanded in item position:
// an inner attribute — which is what `//!` desugars to — is not permitted there.
//
// ⛔ THE ASSERTIONS LIVE HERE AND NOWHERE ELSE, and they are reached from the other crate by
// `include!` instead of by being copied. Two copies would diverge, and the first one that
// diverged would lie in silence — a conformance suite that no longer compares anything still
// prints `ok`. An integration test is a crate of its own and cannot import another test's
// items, so textual inclusion is the mechanism, not a shortcut.
//
// ⚠️ DECLARED COST: `include!` carries the `#[test]` functions of this file along with it, so
// the EIGHT tests below RUN A SECOND TIME inside `platform`'s binary. It buys the single copy of
// the assertions and costs a few milliseconds. ⚠️ THIS LINE SAID "THREE" UNTIL 2026-08-18, when
// finding B-2 brought five liars more — and the clause that followed it, "the fake and the two
// liars sleep for nothing", was wrong in BOTH halves even then: the SEVEN liars never sleep at
// all, because a liar is a fake clock and only the real implementation waits. Recounted on the
// source rather than remembered — gotcha #31.

use kernel::ports::reactor::Reactor;
use kernel::time::{Millis, Monotonic, WallTime};

/// The margin every wait in this suite asks for, and it is 50 ms rather than 5 FOR A REASON
/// THAT ONLY THE REAL IMPLEMENTATION HAS. THIS SUITE computes a deadline from `now()` and then
/// calls `wait_until`; if the machine is descheduled in between, the deadline is already past
/// by the time the port reads its own clock, the port correctly answers `None` — which is the
/// contract, not a violation — and the suite would go red for a reason that has nothing to do
/// with what it is measuring. A margin wide enough to survive an ordinary scheduling hiccup
/// removes that class of false red without weakening a single assertion.
const CONTRACT_MARGIN_MS: u64 = 50;

/// The message case 2a fails with — the deadline EQUAL to the current instant.
///
/// ⛔ TWO CONSTANTS AND NOT ONE, and the reason is the whole point of the negative tests below:
/// THE PAYLOAD HAS TO SAY WHICH HALF FIRED. With a single shared message the two cases would be
/// indistinguishable in exactly the place built to distinguish them — a broken reactor caught
/// by 2a would be indistinguishable from one caught by 2b, and a test claiming to pin the
/// second would be satisfied by the first. Public and stable because the tests match on them.
pub const DEADLINE_EQUAL_TO_NOW_MESSAGE: &str =
    "reactor contract violated: a deadline EQUAL to the current instant must give None";

/// The message case 2b fails with — the deadline STRICTLY BEFORE the current instant. See the
/// constant above for why the two are not one.
pub const DEADLINE_BEFORE_NOW_MESSAGE: &str =
    "reactor contract violated: a deadline STRICTLY BEFORE the current instant must give None";

// ⛔ THE FOUR CONSTANTS BELOW ARRIVED ON 2026-08-18 WITH FINDING B-2 of the 2026-08-11 audit,
// and they are constants for the reason 2a and 2b are: A NEGATIVE TEST HAS TO SAY WHICH
// ASSERTION FIRED. Until that day FOUR GROUPS OUT OF FIVE had no liar at all — deleting blocks
// 1, 3, 4 and 5 outright left the whole workspace green — so the suite asserted its own
// coverage instead of measuring it, which is gotcha #14 at the scale of a file.
//
// ⚠️ They carry only the STATIC HEAD of each message: the values are formatted in at the
// assertion and the tests match with `contains`, exactly as 2a and 2b already do.

/// Assertion 1 — the clock went backwards on its own.
pub const CLOCK_WENT_BACKWARDS_MESSAGE: &str = "reactor contract violated: now() went backwards";

/// Assertion 3a — the first wait came back short of the deadline it was given.
pub const FIRST_WAIT_SHORT_MESSAGE: &str =
    "reactor contract violated: the first wait came back short of its deadline";

/// Assertion 3b — the wait returned the right instant while the port's own clock stayed behind.
pub const CLOCK_LAGGED_BEHIND_MESSAGE: &str =
    "reactor contract violated: the wait returned its deadline but now() stayed behind";

/// Assertion 4a — the SECOND wait came back short. Distinct from 3a's, and it has to be: a test
/// pinning the second would otherwise be satisfied by the first one firing.
pub const SECOND_WAIT_SHORT_MESSAGE: &str =
    "reactor contract violated: the second wait came back short of its deadline";

/// Group 5 carries NO assertion — it proves `wall_time()` is callable and stops there. What
/// proves the block is REACHED is a port whose `wall_time` panics, and this is the text it
/// panics with.
pub const WALL_TIME_UNREACHED_MESSAGE: &str =
    "reactor contract violated: wall_time() was never called, so group 5 is dead code";

/// What every wait on a future deadline fails with, AND IT NAMES THE SECOND CAUSE ON PURPOSE.
///
/// ⚠️ This is the only message a reader will ever see in the one intermittence this suite has
/// left, and left knowingly: `CONTRACT_MARGIN_MS` buys room for an ordinary scheduling hiccup,
/// not for an unbounded one. A message naming only the port would send them hunting for a
/// defect that is not there — so it names both causes and says how to tell them apart.
const FUTURE_DEADLINE_MESSAGE: &str = "reactor contract violated: a deadline in the future \
     must be waited for, not refused.\n⚠️ OR the thread was preempted for the whole margin \
     (see CONTRACT_MARGIN_MS) between computing the deadline and entering wait_until — in \
     which case the deadline HAD already passed, `None` was the CORRECT answer, and the port \
     is innocent. Re-run before looking for a defect: a red that repeats is the port, a red \
     that does not come back was the scheduler.";

/// Every promise the `reactor` port makes, checked against ONE implementation.
///
/// It takes a FACTORY and not a reactor because some assertions need one that has never
/// advanced — the monotonic origin is the bottom of the scale, and once the clock has moved
/// there is no going back to it.
pub fn assert_reactor_contract<R: Reactor, F: Fn() -> R>(build: F) {
    // ── 1. `now()` does not go backwards on its own ──────────────────────────────────────
    // The weakest of the promises and the one everything else stands on. `>=` and not `>`:
    // the port owes monotonicity, not progress, and a clock read twice inside the same
    // millisecond legitimately gives the same answer.
    {
        let reactor = build();
        let first = reactor.now();
        let second = reactor.now();
        assert!(
            second >= first,
            "{CLOCK_WENT_BACKWARDS_MESSAGE}, {first:?} then {second:?}"
        );
    }

    // ── 2. A deadline that is NOT strictly in the future gives `None` ────────────────────
    // ⛔ THE RULE THE WHOLE PORT EXISTS TO HOLD: a null advance must never be reported as a
    // successful one. The first draft of the executor took the minimum of ALL registered
    // deadlines, finished activities included, so the minimum fell in the PAST, the clock did
    // not move, and the function declared success anyway — the executor spun for ever
    // (§3.2.1, gotcha #19).
    //
    // ⛔ TWO FORMS, AND NOT ONE. The branch under test is `deadline <= now`. Exercising only
    // `==` leaves the `<` half of it unvisited: an implementation that wrote `deadline == now`
    // would pass a suite that checked only the equal case, and would then happily "advance"
    // backwards. The two sub-cases below are the two halves.
    {
        // 2a — the deadline IS the current instant. A freshly built reactor is enough.
        let mut reactor = build();
        let now = reactor.now();
        assert_eq!(
            reactor.wait_until(now),
            None,
            "{}",
            DEADLINE_EQUAL_TO_NOW_MESSAGE
        );
    }
    {
        // 2b — the deadline lies STRICTLY BEFORE the current instant. It needs a reactor that
        // has already moved: `Monotonic::ORIGIN` is the bottom of the scale, so with a
        // never-advanced reactor there is no earlier instant to name.
        let mut reactor = build();
        let start = reactor.now();
        let ahead = start.saturating_add(Millis::new(CONTRACT_MARGIN_MS));
        reactor.wait_until(ahead).expect(FUTURE_DEADLINE_MESSAGE);
        assert!(
            start < reactor.now(),
            "the setup of case 2b did not move the clock, so the case would be vacuous"
        );
        assert_eq!(
            reactor.wait_until(start),
            None,
            "{}",
            DEADLINE_BEFORE_NOW_MESSAGE
        );
    }

    // ── 3. A deadline in the future reaches AT LEAST that instant ───────────────────────
    // ── 4. Two consecutive waits COMPOSE ────────────────────────────────────────────────
    // The two share one reactor, and deliberately: assertion 4 is about what a SECOND wait
    // does to a clock the first one has already moved, so building a fresh reactor for it
    // would throw away the very state it is asking about. It also halves what the real
    // implementation spends sleeping.
    {
        let mut reactor = build();

        // 3 — the wait arrives, and `now()` followed it. Both are checked: a port that
        // returned the right instant while leaving its own clock behind would pass on the
        // return value alone, and the executor reads `now()`.
        let start = reactor.now();
        let first_deadline = start.saturating_add(Millis::new(CONTRACT_MARGIN_MS));
        let first_reached = reactor
            .wait_until(first_deadline)
            .expect(FUTURE_DEADLINE_MESSAGE);
        assert!(
            first_reached >= first_deadline,
            "{FIRST_WAIT_SHORT_MESSAGE}: came back at {first_reached:?}, \
             deadline was {first_deadline:?}"
        );
        assert!(
            reactor.now() >= first_deadline,
            "{CLOCK_LAGGED_BEHIND_MESSAGE}: returned {first_reached:?} but now() is \
             {:?}, deadline was {first_deadline:?}",
            reactor.now()
        );

        // 4 — the clock does not restart. A port that reset its origin on every wait would
        // satisfy assertion 3 for ever and never make progress.
        let second_deadline = first_reached.saturating_add(Millis::new(CONTRACT_MARGIN_MS));
        let second_reached = reactor
            .wait_until(second_deadline)
            .expect(FUTURE_DEADLINE_MESSAGE);
        assert!(
            second_reached >= second_deadline,
            "{SECOND_WAIT_SHORT_MESSAGE}: came back at {second_reached:?}, \
             deadline was {second_deadline:?}"
        );

        // ⛔ THIS ASSERTION IS IMPLIED BY THE ONE ABOVE, and it is kept with the proof rather
        // than deleted or given a liar that cannot exist. Found on 2026-08-18 while writing
        // B-2's liars: `second_deadline` is computed from `first_reached`, so
        // `second_reached >= second_deadline = first_reached + MARGIN > first_reached` — the
        // assertion above ENTAILS this one, and no conforming-then-lying reactor can fire this
        // and not that. ⚠️ It is dead weight, NOT a vacuous check: it cannot be false where the
        // other is true, so it never lies; it just never speaks. Removing it, or rebasing
        // `second_deadline` on `start` so that the two become independent, is a change to a
        // SHARED PORT'S CONFORMANCE SUITE — registered as an open item in
        // `docs/porta-di-qualita.md` rather than taken here.
        assert!(
            second_reached > first_reached,
            "reactor contract violated: the clock restarted between two waits, \
             {first_reached:?} then {second_reached:?}"
        );
    }

    // ── 5. `wall_time()` is READABLE — and that is ALL conformance can say about it ──────
    // ⛔ AN HONEST LINE ABOUT A NARROW PROMISE. The port does NOT promise that the two clocks
    // move together: the real implementation serves `wall_time` from the system clock, which
    // NTP, daylight saving or the user can step BACKWARDS at any moment. Any assertion
    // relating it to `now()`, or even to its own previous reading, would be a property of one
    // implementation smuggled into a suite that runs against both — and it would be flaky
    // against the real one, which is the worst way to be wrong.
    //
    // ⚠️ SO THIS LINE PROVES CALLABILITY, NOT A BEHAVIOUR: reaching the end of the block means
    // `wall_time()` returned instead of panicking, and the coverage stops exactly there. It is
    // written with this paragraph rather than as a bare `let _ = …`, which would look like
    // coverage it does not give.
    //
    // The behaviour of the FAKE — its two clocks advancing by the same amount — is a property
    // of the fake and is pinned where it belongs, in `crates/simulator/tests/virtual_clock.rs`.
    {
        let reactor = build();
        let _stamp: WallTime = reactor.wall_time();
    }
}

#[test]
fn the_fake_reactor_honours_the_contract() {
    assert_reactor_contract(simulator::reactor::VirtualReactor::new);
}

// ⛔ THE DIRECTION ONE FORGETS (§7.1.1 rule 3): a suite never seen to fail is not a suite. The
// two tests below break the port's central rule in TWO DIFFERENT WAYS, one per half of the
// `deadline <= now` branch, and demand that the suite notices each.
//
// ⛔ AND WHY THERE ARE TWO, which is the lesson gotcha #14 keeps re-teaching. With only the
// first liar, CASE 2b WAS NEVER SEEN TO FIRE: `NullAdvanceLiar` dies on 2a — it is fresh at the
// origin, so the very first `assert_eq!` explodes — and execution never reaches 2b at all.
// Deleting the whole 2b block left every test in the repository green. The block's own comment
// argues at length that 2a alone is not enough, and that argument was TRUE but UNPROVEN: the
// file asserted its own non-vacuity instead of measuring it. `PastDeadlineLiar` is the measure.
//
// ⛔ AND EACH TEST CHECKS WHAT WAS CAUGHT, NOT MERELY THAT SOMETHING EXPLODED. A `catch_unwind`
// reading only `is_err()` would report success even if the panic had come from another
// assertion entirely — a true measurement of the wrong thing. That is also why 2a and 2b carry
// TWO DISTINCT MESSAGES: with one shared message these two tests could not tell the halves
// apart, and the second would be satisfied by the first firing.

#[test]
fn a_reactor_that_lies_about_a_null_advance_is_caught() {
    let message = message_the_suite_fails_with(NullAdvanceLiar::new).expect(
        "THE SUITE IS VACUOUS: a reactor that reports a null advance as a success passed it",
    );
    assert!(
        message.contains(DEADLINE_EQUAL_TO_NOW_MESSAGE),
        "the suite did fire, but NOT on case 2a.\n\
         expected to contain: {DEADLINE_EQUAL_TO_NOW_MESSAGE}\n\
         actual payload: {message}"
    );
}

#[test]
fn a_reactor_that_lies_only_about_a_past_deadline_is_caught() {
    // ⛔ THIS IS THE TEST THAT MAKES CASE 2b NON-VACUOUS, and it is the only thing in the
    // repository that does. Delete the 2b block and this test goes red — `PastDeadlineLiar`
    // would sail through the rest of the suite, `message_the_suite_fails_with` would give back
    // `None`, and the `expect` below would fire.
    let message = message_the_suite_fails_with(PastDeadlineLiar::new).expect(
        "THE 2b CASE IS VACUOUS: a reactor that honours `deadline == now` and lies about \
         `deadline < now` passed the suite, so the `<` half of the branch is unguarded",
    );
    assert!(
        message.contains(DEADLINE_BEFORE_NOW_MESSAGE),
        "the suite did fire, but NOT on case 2b — so 2b is still unproven.\n\
         expected to contain: {DEADLINE_BEFORE_NOW_MESSAGE}\n\
         actual payload: {message}"
    );
}

// ⛔ THE FIVE TESTS BELOW ARE FINDING B-2, closed on 2026-08-18. Before them the suite had TWO
// liars for ONE group, and the other four groups were held by nothing: the audit measured that
// deleting blocks 1, 3, 4 and 5 left `cargo test --workspace` entirely green.
//
// ⛔ ONE LIAR PER ASSERTION AND NOT PER GROUP, which is gotcha #65 applied here: the suite dies
// at the FIRST assertion that fires, so a liar broken in two places proves only the first. Group
// 3 has two assertions — the returned instant and the port's own clock — and they need a liar
// each; group 4's second assertion needs none, because it is ENTAILED by its first (see the
// comment on it).

#[test]
fn a_reactor_whose_clock_runs_backwards_is_caught() {
    let message = message_the_suite_fails_with(BackwardsClockLiar::new)
        .expect("THE SUITE IS VACUOUS ON GROUP 1: a clock that runs backwards passed it");
    assert!(
        message.contains(CLOCK_WENT_BACKWARDS_MESSAGE),
        "the suite did fire, but NOT on group 1.\n\
         expected to contain: {CLOCK_WENT_BACKWARDS_MESSAGE}\n\
         actual payload: {message}"
    );
}

#[test]
fn a_reactor_that_comes_back_short_of_its_deadline_is_caught() {
    let message = message_the_suite_fails_with(ShortWaitLiar::new)
        .expect("THE SUITE IS VACUOUS ON ASSERTION 3a: a wait that stops short passed it");
    assert!(
        message.contains(FIRST_WAIT_SHORT_MESSAGE),
        "the suite did fire, but NOT on assertion 3a.\n\
         expected to contain: {FIRST_WAIT_SHORT_MESSAGE}\n\
         actual payload: {message}"
    );
}

#[test]
fn a_reactor_that_leaves_its_own_clock_behind_is_caught() {
    // ⛔ THE ASSERTION THIS PINS IS THE ONE THE BLOCK'S COMMENT CALLS OUT: "a port that
    // returned the right instant while leaving its own clock behind would pass on the return
    // value alone, and the executor reads `now()`". That argument was TRUE and UNPROVEN.
    let message = message_the_suite_fails_with(LaggingClockLiar::new)
        .expect("THE SUITE IS VACUOUS ON ASSERTION 3b: a port whose clock lags passed it");
    assert!(
        message.contains(CLOCK_LAGGED_BEHIND_MESSAGE),
        "the suite did fire, but NOT on assertion 3b.\n\
         expected to contain: {CLOCK_LAGGED_BEHIND_MESSAGE}\n\
         actual payload: {message}"
    );
}

#[test]
fn a_reactor_correct_on_the_first_wait_and_short_on_the_second_is_caught() {
    // ⛔ IT HAS TO BE CORRECT ON THE FIRST ONE, or it never reaches group 4 — and that is the
    // whole reason this liar counts its calls instead of being broken outright.
    let message = message_the_suite_fails_with(SecondWaitShortLiar::new)
        .expect("THE SUITE IS VACUOUS ON ASSERTION 4a: a second wait that stops short passed it");
    assert!(
        message.contains(SECOND_WAIT_SHORT_MESSAGE),
        "the suite did fire, but NOT on assertion 4a.\n\
         expected to contain: {SECOND_WAIT_SHORT_MESSAGE}\n\
         actual payload: {message}"
    );
}

#[test]
fn the_suite_really_reaches_the_wall_time_block() {
    // ⛔ GROUP 5 HAS NO ASSERTION — it proves `wall_time()` is callable and says so. That makes
    // it the one block whose DELETION nothing could notice: there is no oracle to go red. A
    // port whose `wall_time` panics turns "the block exists" into "the block runs", which is
    // the only property that block can have.
    let message = message_the_suite_fails_with(PanickingWallClockLiar::new)
        .expect("GROUP 5 IS DEAD CODE: the suite never called wall_time()");
    assert!(
        message.contains(WALL_TIME_UNREACHED_MESSAGE),
        "the suite did fire, but NOT inside group 5.\n\
         expected to contain: {WALL_TIME_UNREACHED_MESSAGE}\n\
         actual payload: {message}"
    );
}

/// Runs the suite against a deliberately broken reactor and gives back THE MESSAGE IT FAILED
/// WITH, or `None` if the suite let the reactor through — the vacuous case, which each caller
/// reports in its own words because the two mean different things.
///
/// ⚠️ The panic hook is silenced for the duration of the call: the panic is EXPECTED, and its
/// backtrace in the test output would train the reader to ignore backtraces. It is restored
/// immediately, so any LATER panic prints normally.
///
/// ⛔ DECLARED LIMIT, because the hook is PROCESS-WIDE and libtest runs tests on parallel
/// threads: a panic raised in ANOTHER test that happens to land inside this window is reported
/// as `FAILED` with no stdout section — the failure is never hidden, only its message. In
/// `platform`'s binary a test that sleeps 150 ms really does run alongside this one. The window
/// is microseconds wide and the risk is negligible, which is why it is declared rather than
/// engineered away; saying "a later unexpected panic still prints" would have been a shade
/// wider than the truth.
fn message_the_suite_fails_with<R, F>(build: F) -> Option<String>
where
    R: Reactor,
    F: Fn() -> R + std::panic::UnwindSafe,
{
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let outcome = std::panic::catch_unwind(|| assert_reactor_contract(build));
    std::panic::set_hook(previous_hook);
    outcome.err().map(|payload| panic_message(&*payload))
}

/// A `Reactor` broken ON PURPOSE: `wait_until` moves the clock to the deadline and answers
/// `Some` WHATEVER the deadline is, past instants included. It is the shape of §3.2.1's trap.
/// Being fresh at the origin, it is caught by CASE 2a.
struct NullAdvanceLiar {
    now: Monotonic,
}

impl NullAdvanceLiar {
    fn new() -> Self {
        NullAdvanceLiar {
            now: Monotonic::ORIGIN,
        }
    }
}

impl Reactor for NullAdvanceLiar {
    fn now(&self) -> Monotonic {
        self.now
    }

    fn wall_time(&self) -> WallTime {
        WallTime::from_millis_since_epoch(0)
    }

    fn wait_until(&mut self, deadline: Monotonic) -> Option<Monotonic> {
        // ⛔ THE DEFECT: no check that the deadline lies strictly ahead. A conforming
        // implementation returns `None` here whenever `deadline <= self.now`.
        self.now = deadline;
        Some(deadline)
    }
}

/// A `Reactor` broken ON PURPOSE IN THE OTHER HALF: it honours `None` for a deadline EQUAL to
/// the current instant, and lies for one strictly BEFORE it. This is precisely the
/// `if deadline == self.now` implementation that case 2b's comment warns about, made real.
///
/// It walks further into the suite than `NullAdvanceLiar` does, and that is its purpose: it
/// passes assertion 1, passes case 2a, performs 2b's setup wait correctly — the clock really
/// moves — and dies on 2b's own assertion. It is the only reactor here that reaches that line.
struct PastDeadlineLiar {
    now: Monotonic,
}

impl PastDeadlineLiar {
    fn new() -> Self {
        PastDeadlineLiar {
            now: Monotonic::ORIGIN,
        }
    }
}

impl Reactor for PastDeadlineLiar {
    fn now(&self) -> Monotonic {
        self.now
    }

    fn wall_time(&self) -> WallTime {
        WallTime::from_millis_since_epoch(0)
    }

    fn wait_until(&mut self, deadline: Monotonic) -> Option<Monotonic> {
        // ⛔ THE DEFECT, and it is exactly half of the previous one: `==` where the contract
        // says `<=`. A deadline in the past slips past this guard and is reported as a
        // successful advance — backwards.
        if deadline == self.now {
            return None;
        }
        self.now = deadline;
        Some(deadline)
    }
}

/// One millisecond before an instant. ⚠️ `Monotonic` has no subtraction and deliberately so —
/// the only distance it exposes is `saturating_since`, which answers a `Millis` — so the trip
/// through the origin is how a liar names an instant just short of its deadline. Written once
/// and used by two of them, so that "short" cannot come to mean two different amounts.
fn one_millisecond_before(instant: Monotonic) -> Monotonic {
    let from_origin = instant.saturating_since(Monotonic::ORIGIN).get();
    Monotonic::from_millis(from_origin.saturating_sub(1))
}

/// A `Reactor` broken ON PURPOSE IN GROUP 1: `now()` walks BACKWARDS on its own, with no wait
/// in between. It starts above the origin because `Monotonic::ORIGIN` is the bottom of the
/// scale and there is nothing below it to walk down to.
struct BackwardsClockLiar {
    reads: core::cell::Cell<u64>,
}

impl BackwardsClockLiar {
    fn new() -> Self {
        BackwardsClockLiar {
            reads: core::cell::Cell::new(0),
        }
    }
}

impl Reactor for BackwardsClockLiar {
    fn now(&self) -> Monotonic {
        // ⛔ THE DEFECT: every read is earlier than the one before it.
        let read = self.reads.get();
        self.reads.set(read + 1);
        Monotonic::from_millis(1_000 - read.min(1_000))
    }

    fn wall_time(&self) -> WallTime {
        WallTime::from_millis_since_epoch(0)
    }

    fn wait_until(&mut self, _deadline: Monotonic) -> Option<Monotonic> {
        None
    }
}

/// A `Reactor` broken ON PURPOSE IN ASSERTION 3a: it moves its clock all the way to the
/// deadline — so it walks through groups 1 and 2 honestly, including 2b's setup wait — and then
/// REPORTS one millisecond less than it actually reached.
struct ShortWaitLiar {
    now: Monotonic,
}

impl ShortWaitLiar {
    fn new() -> Self {
        ShortWaitLiar {
            now: Monotonic::ORIGIN,
        }
    }
}

impl Reactor for ShortWaitLiar {
    fn now(&self) -> Monotonic {
        self.now
    }

    fn wall_time(&self) -> WallTime {
        WallTime::from_millis_since_epoch(0)
    }

    fn wait_until(&mut self, deadline: Monotonic) -> Option<Monotonic> {
        if deadline <= self.now {
            return None;
        }
        self.now = deadline;
        // ⛔ THE DEFECT: the clock is right, the ANSWER is short. A caller that trusts the
        // return value schedules the next deadline from an instant that never happened.
        Some(one_millisecond_before(deadline))
    }
}

/// A `Reactor` broken ON PURPOSE IN ASSERTION 3b: it answers with exactly the deadline it was
/// given — so the return value is impeccable — while its OWN clock stops halfway. It is the
/// port the block's comment describes and that nothing used to catch.
struct LaggingClockLiar {
    now: Monotonic,
}

impl LaggingClockLiar {
    fn new() -> Self {
        LaggingClockLiar {
            now: Monotonic::ORIGIN,
        }
    }
}

impl Reactor for LaggingClockLiar {
    fn now(&self) -> Monotonic {
        self.now
    }

    fn wall_time(&self) -> WallTime {
        WallTime::from_millis_since_epoch(0)
    }

    fn wait_until(&mut self, deadline: Monotonic) -> Option<Monotonic> {
        if deadline <= self.now {
            return None;
        }
        // ⛔ THE DEFECT: the clock moves, but only halfway. It has to MOVE, or 2b's setup
        // assertion fires first and this liar would be proving that one instead.
        let half = deadline.saturating_since(self.now).get() / 2;
        self.now = self.now.saturating_add(Millis::new(half.max(1)));
        Some(deadline)
    }
}

/// A `Reactor` broken ON PURPOSE IN ASSERTION 4a: correct on the FIRST successful wait of its
/// life and short on the SECOND. It has to be correct first, or it never reaches group 4 —
/// which is why it counts instead of being broken outright.
///
/// ⚠️ The count is per INSTANCE, and the suite builds a fresh reactor for each block, so the
/// two waits it sees are groups 3 and 4 — not 2b's setup, which happens on another instance.
struct SecondWaitShortLiar {
    now: Monotonic,
    successful_waits: u32,
}

impl SecondWaitShortLiar {
    fn new() -> Self {
        SecondWaitShortLiar {
            now: Monotonic::ORIGIN,
            successful_waits: 0,
        }
    }
}

impl Reactor for SecondWaitShortLiar {
    fn now(&self) -> Monotonic {
        self.now
    }

    fn wall_time(&self) -> WallTime {
        WallTime::from_millis_since_epoch(0)
    }

    fn wait_until(&mut self, deadline: Monotonic) -> Option<Monotonic> {
        if deadline <= self.now {
            return None;
        }
        self.successful_waits += 1;
        self.now = deadline;
        if self.successful_waits >= 2 {
            // ⛔ THE DEFECT, and only from the second wait on.
            return Some(one_millisecond_before(deadline));
        }
        Some(deadline)
    }
}

/// A `Reactor` that conforms in every group EXCEPT that `wall_time()` refuses to answer.
///
/// ⛔ IT IS NOT A LIAR ABOUT A BEHAVIOUR, because group 5 asserts none: it is the instrument
/// that turns "the block exists" into "the block RUNS". Group 5 is the one block whose deletion
/// no oracle could notice, and this is what notices.
struct PanickingWallClockLiar {
    now: Monotonic,
}

impl PanickingWallClockLiar {
    fn new() -> Self {
        PanickingWallClockLiar {
            now: Monotonic::ORIGIN,
        }
    }
}

impl Reactor for PanickingWallClockLiar {
    fn now(&self) -> Monotonic {
        self.now
    }

    fn wall_time(&self) -> WallTime {
        panic!("{}", WALL_TIME_UNREACHED_MESSAGE)
    }

    fn wait_until(&mut self, deadline: Monotonic) -> Option<Monotonic> {
        if deadline <= self.now {
            return None;
        }
        self.now = deadline;
        Some(deadline)
    }
}

/// The text of a panic, dug out of the payload. `assert!`/`assert_eq!` with a format argument
/// panic with a `String`; a `panic!("literal")` with no arguments carries a `&str` instead, and
/// both are handled so that this helper cannot report "unknown" for a message that is right
/// there.
fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(text) = payload.downcast_ref::<String>() {
        text.clone()
    } else if let Some(text) = payload.downcast_ref::<&str>() {
        (*text).to_string()
    } else {
        String::from("<panic payload that is neither String nor &str>")
    }
}
