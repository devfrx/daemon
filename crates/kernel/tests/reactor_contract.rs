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
// the three tests below RUN A SECOND TIME inside `platform`'s binary. It buys the single copy
// of the assertions and costs a few milliseconds — the fake and the two liars sleep for
// nothing.

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
            "reactor contract violated: now() went backwards, {first:?} then {second:?}"
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
            "reactor contract violated: the wait came back at {first_reached:?}, \
             short of the deadline {first_deadline:?}"
        );
        assert!(
            reactor.now() >= first_deadline,
            "reactor contract violated: the wait returned {first_reached:?} but now() is \
             {:?}, behind the deadline {first_deadline:?}",
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
            "reactor contract violated: the second wait came back at {second_reached:?}, \
             short of the deadline {second_deadline:?}"
        );
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
