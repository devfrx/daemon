//! C1, C2, C3 and NON-VACUITY, on the executor that ships — not on the spike's.
//!
//! ⚠️ The interleaving figure is MEASURED HERE and not carried over from SP-5: this
//! executor polls every runnable activity once per turn in a seeded order (decision D4),
//! while the spike picked one at random. Citing the spike's 13-out-of-17 would be an
//! expectation written before the measurement, which is gotcha #15.

use core::cell::RefCell;

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
    executor.spawn(async {
        Yield::once().await;
    });
    sleep.until(Monotonic::from_millis(5_000));
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
    executor.spawn(async {
        Yield::once().await;
    });
    sleep.until(Monotonic::ORIGIN);
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
