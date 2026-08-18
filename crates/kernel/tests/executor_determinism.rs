//! C1, C2, C3 and NON-VACUITY, on the executor that ships — not on the spike's.
//!
//! ⚠️ The interleaving figure is MEASURED HERE and not carried over from SP-5: this
//! executor polls every runnable activity once per turn in a seeded order (decision D4),
//! while the spike picked one at random. Citing the spike's 13-out-of-17 would be an
//! expectation written before the measurement, which is gotcha #15.

use core::cell::{Cell, RefCell};

use kernel::executor::{Executor, RunError, Sleep};
use kernel::parameters::Parameters;
use kernel::time::Monotonic;
use simulator::reactor::VirtualReactor;
use simulator::rng::SeededRng;

const TURN_LIMIT: u64 = 10_000;

/// The scenario of M-2, reduced to what milestone 2 can express: 3 activities x 4 steps,
/// each step waiting 5000 VIRTUAL milliseconds. No journal, no faults — those are
/// milestones 3 and 4.
fn trace_of(seed: u64) -> Vec<String> {
    let trace: RefCell<Vec<String>> = RefCell::new(Vec::new());
    let sleep = Sleep::new();
    let mut executor = Executor::new(
        SeededRng::new(seed),
        VirtualReactor::new(),
        Parameters::new(TURN_LIMIT),
        &sleep,
    );

    for activity in 0..3usize {
        let trace = &trace;
        let sleep = &sleep;
        executor.spawn(async move {
            for step in 0..4usize {
                trace.borrow_mut().push(format!("a{activity} s{step}"));
                // Suspend on a PORT: the reactor is the only thing that can bring this
                // activity back. §2.4.1.
                let deadline = Monotonic::from_millis(((step as u64) + 1) * 5_000);
                sleep.until(deadline);
                Yield::once().await;
            }
        });
    }

    executor.run().expect("the scenario terminates");
    // ⛔ The executor is dropped EXPLICITLY before the trace is taken. Its tasks hold
    // `Box<dyn Future>` values that borrow `trace`, and a boxed trait object carries
    // drop glue, so the borrow would otherwise live to the end of the scope and
    // `into_inner` — which moves `trace` — would not compile.
    drop(executor);
    trace.into_inner()
}

/// How many consecutive pairs of the trace change activity, and how many pairs there are.
///
/// ⚠️ Written once and used twice, so that the assertion and the printed measurement can
/// never come to disagree about what "a switch" means.
fn switches_and_transitions(trace: &[String]) -> (usize, usize) {
    let switches = trace
        .windows(2)
        .filter(|pair| pair[0][..2] != pair[1][..2])
        .count();
    (switches, trace.len() - 1)
}

/// A future that returns `Pending` exactly once. It is how an activity hands control
/// back to the executor after declaring a suspension.
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

#[test]
fn c1_the_same_seed_gives_one_single_trace() {
    let reference = trace_of(20_260_806);
    for _ in 0..100 {
        assert_eq!(trace_of(20_260_806), reference);
    }
}

#[test]
fn c2_a_different_seed_gives_a_different_trace() {
    let mut seen = std::collections::BTreeSet::new();
    for seed in 0..200u64 {
        seen.insert(trace_of(seed));
    }
    assert!(
        seen.len() > 1,
        "the seed does not change the order: {} distinct",
        seen.len()
    );
}

#[test]
fn c3_virtual_time_does_not_wait() {
    // 3 activities x 4 waits of 5000 ms. If they were sequential the clock would reach
    // 60 000; concurrent, it reaches 20 000. The figure is the COUNTER-PROOF THAT THE
    // CONCURRENCY IS REAL, not just that the run is deterministic — finding 2 of §3.6.1.
    let sleep = Sleep::new();
    let mut executor = Executor::new(
        SeededRng::new(20_260_806),
        VirtualReactor::new(),
        Parameters::new(TURN_LIMIT),
        &sleep,
    );
    for _ in 0..3 {
        let sleep = &sleep;
        executor.spawn(async move {
            for step in 0..4usize {
                sleep.until(Monotonic::from_millis(((step as u64) + 1) * 5_000));
                Yield::once().await;
            }
        });
    }
    executor.run().expect("the scenario terminates");
    assert_eq!(executor.now(), Monotonic::from_millis(20_000));
}

#[test]
fn non_vacuity_the_interleaving_is_real() {
    // ⛔ THE PROBE THAT IS EASY TO GET WRONG. SP-5's first version counted "task0 twice
    // in a row", which happens by chance one time in three. Corrected by counting TASK
    // SWITCHES — and it is counted again here, on this executor, because the ordering
    // policy is not the spike's.
    let trace = trace_of(20_260_806);
    let (switches, transitions) = switches_and_transitions(&trace);

    // A sequential control: 3 activities run one after the other give exactly 2
    // switches over 11 transitions.
    assert!(
        switches > 2,
        "no real interleaving: {switches} switches over {transitions} transitions"
    );

    // ⛔ AND THE CONTROL IN THE OTHER DIRECTION, because that sentence was a comment and a
    // comment is an intention. The assertion above is VACUOUS if the counter can only
    // answer "every pair differs": one comparing whole entries instead of the two-character
    // activity prefix would return 11 for the sequential trace too, and the test would go
    // green under any executor whatsoever. So the sequential trace is built and fed to the
    // same counter, which must say 2.
    let sequential: Vec<String> = (0..3)
        .flat_map(|activity| (0..4).map(move |step| format!("a{activity} s{step}")))
        .collect();
    assert_eq!(switches_and_transitions(&sequential), (2, 11));
}

#[test]
fn a_block_becomes_an_error_and_not_an_infinite_wait() {
    // The turn guard, §3.2.1: an activity that never finishes and never sleeps must
    // exhaust the limit rather than hang the test.
    let sleep = Sleep::new();
    let mut executor = Executor::new(
        SeededRng::new(1),
        VirtualReactor::new(),
        Parameters::new(50),
        &sleep,
    );
    executor.spawn(async {
        loop {
            Yield::once().await;
        }
    });
    assert_eq!(executor.run(), Err(RunError::TurnLimitReached));
}

#[test]
fn the_delivered_turn_limit_is_honoured_by_its_value() {
    // ⛔ FINDING B-1 OF THE 2026-08-11 AUDIT. The test above proves the guard FIRES; it does
    // not prove the executor used the number it was HANDED. Measured on 2026-08-18:
    // replacing the field with `turn_limit: { let _ = parameters; 10_000 }` left the whole
    // workspace at 32 targets and 177 passing — a parameter that is delivered and not read
    // is a constant just the same, which is gotcha #28 from the other side.
    //
    // ⚠️ TWO VALUES AND NOT ONE, because a single one is satisfied by any implementation
    // whose constant happens to equal it — gotcha #48, "for every mutation on a value prove
    // TWO". Both are far from any plausible hard-coded default.
    //
    // 📌 The oracle is the POLL COUNT and not the error: `run` takes exactly one turn per
    // poll for an activity that only yields, and stops when `turns > limit`. So the
    // activity is polled `limit` times, which is the only observable that carries the value.
    for limit in [7u64, 13] {
        let polls = Cell::new(0u64);
        let sleep = Sleep::new();
        let mut executor = Executor::new(
            SeededRng::new(1),
            VirtualReactor::new(),
            Parameters::new(limit),
            &sleep,
        );
        executor.spawn(async {
            loop {
                polls.set(polls.get() + 1);
                Yield::once().await;
            }
        });
        assert_eq!(
            executor.run(),
            Err(RunError::TurnLimitReached),
            "limit {limit}"
        );
        // The activity borrows `polls`, and a boxed future carries drop glue: the executor
        // goes first, exactly as in `trace_of`.
        drop(executor);
        assert_eq!(
            polls.get(),
            limit,
            "limit {limit}: the executor did not run on the value it was delivered"
        );
    }
}

#[test]
fn a_reactor_that_will_not_advance_is_an_error_and_not_a_spin() {
    // ⚠️ The activity must register a STRICTLY FUTURE deadline. With a past one the
    // promotion path fires, `wait_until` is never called, and the test would pass for
    // the wrong reason — gotcha #17.
    struct RefusingReactor;
    impl kernel::ports::reactor::Reactor for RefusingReactor {
        fn now(&self) -> Monotonic {
            Monotonic::ORIGIN
        }
        fn wall_time(&self) -> kernel::time::WallTime {
            kernel::time::WallTime::from_millis_since_epoch(0)
        }
        fn wait_until(&mut self, _deadline: Monotonic) -> Option<Monotonic> {
            None
        }
    }

    let sleep = Sleep::new();
    let mut executor = Executor::new(
        SeededRng::new(1),
        RefusingReactor,
        Parameters::new(TURN_LIMIT),
        &sleep,
    );
    // ⚠️ THE ACTIVITY DECLARES ITS OWN DEADLINE, and until 2026-08-18 the bench wrote it
    // into the cell before `run` instead. That preload went through the hole finding K-1
    // names, so this probe was resting on the very defect the fix removes.
    executor.spawn(async {
        sleep.until(Monotonic::from_millis(5_000));
        Yield::once().await;
    });
    assert_eq!(executor.run(), Err(RunError::ReactorWillNotAdvance));
}

#[test]
fn a_wait_already_over_wakes_immediately_and_the_clock_does_not_move() {
    // ⛔ The boundary is the point: `deadline == now`, not a strictly past instant. It is
    // what discriminates `until <= instant` from `until < instant`.
    //
    // ⛔ And the second assertion is the direction that gets forgotten: `Ok(())` alone
    // does NOT prove the executor declined to advance — an implementation that satisfied
    // the sleeper BY MOVING THE CLOCK would pass the first assertion.
    let sleep = Sleep::new();
    let mut executor = Executor::new(
        SeededRng::new(1),
        VirtualReactor::new(),
        Parameters::new(TURN_LIMIT),
        &sleep,
    );
    // ⛔ AND THE THIRD THING, learnt on 2026-08-18: written with the bench preloading the
    // cell, THIS PROBE WAS VACUOUS THE MOMENT K-1 WAS FIXED — not red, GREEN FOR NOTHING.
    // Measured: with the entry drained and `until <= instant` mutated to `until < instant`
    // — the very discrimination the paragraph above claims — the old form stayed green
    // while the same mutation turned five other tests red. With the activity declaring its
    // own deadline it goes red, which is the whole reason the two lines swapped places.
    executor.spawn(async {
        sleep.until(Monotonic::ORIGIN);
        Yield::once().await;
    });
    assert_eq!(executor.run(), Ok(()));
    assert_eq!(
        executor.now(),
        Monotonic::ORIGIN,
        "the clock moved to satisfy a wait that was already over"
    );
}

#[test]
fn a_suspension_request_is_not_inherited_by_the_next_activity() {
    // ⛔ PERMANENT REGRESSION (V31, ADR-0021). The first draft drained the `Sleep` cell
    // only on the `Pending` arm: an activity that requested a suspension and then
    // returned `Ready` left the request behind, and the NEXT activity polled inherited
    // it — asleep on a deadline that was never its own.
    //
    // ⚠️ Across a RANGE of seeds, not one: on a single seed the defect is a coin toss. The
    // leak needs the DECLARING activity to be polled FIRST, which is `below(2) == 1` on the
    // turn's only draw — so the seeds split in two and neither half is negligible.
    //
    // 📌 MEASURED HERE, on this executor, by putting the defect back and reading the clock:
    // seeds 2, 3, 5 and 6 leak (clock 9999), seeds 1 and 4 do not (clock 0). That is FOUR
    // out of six, and errata E10 records three — {1, 3, 5} — from an earlier measurement.
    // The divergence is registered rather than smoothed over (gotcha #15); what both agree
    // on, and what this loop rests on, is that one seed is not enough.
    for seed in 1..=6u64 {
        let sleep = Sleep::new();
        let mut executor = Executor::new(
            SeededRng::new(seed),
            VirtualReactor::new(),
            Parameters::new(TURN_LIMIT),
            &sleep,
        );
        // Declares a suspension, then finishes WITHOUT yielding.
        executor.spawn(async {
            sleep.until(Monotonic::from_millis(9_999));
        });
        // Merely yields, and must not inherit the deadline above.
        executor.spawn(async {
            Yield::once().await;
        });
        assert_eq!(executor.run(), Ok(()), "seed {seed}");
        assert_eq!(
            executor.now(),
            Monotonic::ORIGIN,
            "seed {seed}: a suspension request leaked to another activity"
        );
    }
}

/// A future written BY HAND, because an async block cannot express what this needs: its
/// locals are dropped INSIDE the poll that completes it, before the executor reads the
/// cell. Here the destructor belongs to the boxed future itself, so it runs when the
/// finished task is removed from the vector — after the loop, hence after the last read.
struct WritesFromItsDestructor<'a> {
    sleep: &'a Sleep,
    ran: &'a Cell<bool>,
}

impl core::future::Future for WritesFromItsDestructor<'_> {
    type Output = ();
    fn poll(
        self: core::pin::Pin<&mut Self>,
        _context: &mut core::task::Context<'_>,
    ) -> core::task::Poll<()> {
        core::task::Poll::Ready(())
    }
}

impl Drop for WritesFromItsDestructor<'_> {
    fn drop(&mut self) {
        self.ran.set(true);
        self.sleep.until(Monotonic::from_millis(9_999));
    }
}

// ⛔ FINDING K-1 OF THE 2026-08-11 AUDIT, and the two probes below are TWO because the
// entry paths are two — not because the cause is two. The suite stops at the first red, so
// one probe per path is what keeps the second from going unproven while a test claims
// otherwise: gotcha #65, the shape the first audit decision taught on 2026-08-17.
//
// ⚠️ WHAT MAKES THEM NON-VACUOUS IS NOT HERE, and it is worth saying rather than
// duplicating: an executor that ignored `Sleep` altogether would satisfy both. That
// direction is already held by `c3_virtual_time_does_not_wait`, which needs the clock to
// REACH 20 000, and by the probe above. A third copy would be gotcha #49.

#[test]
fn a_request_written_before_the_run_belongs_to_nobody() {
    // The bench holds `&Sleep` and can write to it at any time. Before the fix the first
    // activity the SEED happened to poll inherited that deadline — a suspension nobody
    // asked for, honoured on an arbitrary victim.
    for seed in 1..=6u64 {
        let sleep = Sleep::new();
        let mut executor = Executor::new(
            SeededRng::new(seed),
            VirtualReactor::new(),
            Parameters::new(TURN_LIMIT),
            &sleep,
        );
        executor.spawn(async {
            Yield::once().await;
        });
        executor.spawn(async {
            Yield::once().await;
        });

        sleep.until(Monotonic::from_millis(9_999));

        assert_eq!(executor.run(), Ok(()), "seed {seed}");
        assert_eq!(
            executor.now(),
            Monotonic::ORIGIN,
            "seed {seed}: a request written before the run was honoured on an activity \
             that never asked for it"
        );
    }
}

#[test]
fn a_request_written_by_a_destructor_belongs_to_nobody() {
    // The second path, and the one draining at the entry of `run` does NOT close: the
    // write happens mid-run, between the last read of one turn and the first poll of the
    // next. Measured on 2026-08-18 — with the entry drained, this still reached 9999.
    for seed in 1..=6u64 {
        let sleep = Sleep::new();
        let ran = Cell::new(false);
        let mut executor = Executor::new(
            SeededRng::new(seed),
            VirtualReactor::new(),
            Parameters::new(TURN_LIMIT),
            &sleep,
        );

        executor.spawn(WritesFromItsDestructor {
            sleep: &sleep,
            ran: &ran,
        });
        // Still `Pending` on the turn after, so it is the one that would inherit.
        executor.spawn(async {
            Yield::once().await;
            Yield::once().await;
        });

        let outcome = executor.run();
        let clock = executor.now();
        drop(executor);

        // ⛔ Gotcha #17: prove the injection HAPPENED before believing the oracle. A first
        // draft of this probe used an async block and was VACUOUS — the guard died inside
        // the poll, and the test passed against the unfixed executor.
        assert!(ran.get(), "seed {seed}: the destructor never ran");
        assert_eq!(outcome, Ok(()), "seed {seed}");
        assert_eq!(
            clock,
            Monotonic::ORIGIN,
            "seed {seed}: a destructor's write was honoured as the next activity's request"
        );
    }
}

#[test]
fn re_registering_a_past_deadline_for_ever_still_terminates() {
    // ⛔ The assertion that makes promoting expired sleepers SAFE rather than merely
    // nicer. Without it, "we removed the abort" has no proof that the pathological case
    // still ends.
    let sleep = Sleep::new();
    let mut executor = Executor::new(
        SeededRng::new(1),
        VirtualReactor::new(),
        Parameters::new(50),
        &sleep,
    );
    executor.spawn(async {
        loop {
            sleep.until(Monotonic::ORIGIN);
            Yield::once().await;
        }
    });
    assert_eq!(executor.run(), Err(RunError::TurnLimitReached));
}

#[test]
fn measure_and_print_the_interleaving() {
    // Not an assertion: a MEASUREMENT, printed so the figure enters the documentation
    // instead of being guessed. Run with `-- --nocapture`.
    let trace = trace_of(20_260_806);
    let (switches, transitions) = switches_and_transitions(&trace);
    println!("INTERLEAVING seed=20260806: {switches} switches over {transitions} transitions");
}
