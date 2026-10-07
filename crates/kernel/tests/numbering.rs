//! The one progressive counter of the core: it hands out ascending numbers, it starts ABOVE
//! every step the journal already holds, and it stays ONE however many consumers hold it.
//!
//! ⛔ THE SECOND DIRECTION IS THE ONE THAT DECIDES HERE, and it is the reason this file is not
//! two asserts. A counter that always started at zero would pass "two numbers differ and
//! ascend" and would still hand a RESTARTED core the numbers it has already written -- which is
//! exactly the defect `ClientId`'s doc names: "two independent counters that look identical are
//! a divergence nothing would report". The seeding probe is that direction.
//!
//! ⛔ AND THE SHARING PAIR HOLDS THE PROMISE THE MODULE IS FOR: one counter, however many hold it.
//! Its first probe is not vacuous, MEASURED on 2026-10-02: written with a COPY of the counter
//! where it calls `share` -- what a `Copy` counter allows -- it draws `[0, 0, 0, 1, 1]`. The pair
//! holds both halves -- every holder of ONE counter draws from one sequence, and two counters
//! started apart are two sequences -- so the first cannot pass on a counter that secretly shares
//! with everybody.

use kernel::numbering::{seeded_from, Progressive};
use kernel::ports::journal::{Journal, StepId};
use simulator::journal::MemoryJournal;

#[test]
fn it_hands_out_ascending_numbers_from_where_it_was_started() {
    let numbers = Progressive::starting_at(7);
    assert_eq!(numbers.take(), 7, "the FIRST number is the one it was started at");
    assert_eq!(numbers.take(), 8);
    assert_eq!(numbers.take(), 9);
}

#[test]
fn every_holder_of_one_counter_draws_from_the_same_sequence() {
    // ⛔ THE THREE HOLDERS OF PRODUCTION, NAMED FOR THEM: the core numbers steps, the transport
    // numbers clients, and the faucet of `gui/fake-core` numbers the steps it opens -- and none of
    // them may hand out a number another one already has.
    let core = Progressive::starting_at(0);
    let transport = core.share();
    let faucet = transport.share();

    let drawn = [
        core.take(),
        transport.take(),
        faucet.take(),
        transport.take(),
        core.take(),
    ];
    assert_eq!(
        drawn,
        [0, 1, 2, 3, 4],
        "ONE sequence, whoever draws from it: no number twice, and none skipped"
    );
}

#[test]
fn two_counters_started_apart_are_two_sequences() {
    // ⛔ THE OTHER DIRECTION, AND WITHOUT IT THE PROBE ABOVE PROVES LESS THAN IT SEEMS: a counter
    // that shared ONE hidden sequence with every other counter would pass it too. Sharing happens
    // through `share` and nowhere else, which is why a composition root builds the counter ONCE.
    let first = Progressive::starting_at(0);
    let second = Progressive::starting_at(0);
    assert_eq!(
        first.take(),
        second.take(),
        "two `starting_at` are two counters, and two counters hand out the same numbers"
    );
}

#[test]
fn an_empty_journal_seeds_it_at_zero() {
    let journal = MemoryJournal::new();
    let numbers = seeded_from(&journal).expect("an empty journal replays");
    assert_eq!(numbers.take(), 0);
}

#[test]
fn a_written_journal_seeds_it_above_the_highest_step_it_holds() {
    let mut journal = MemoryJournal::new();
    journal.intent(StepId::new(4), b"intent").expect("intent");
    journal.intent(StepId::new(41), b"intent").expect("intent");
    journal.intent(StepId::new(9), b"intent").expect("intent");

    let numbers = seeded_from(&journal).expect("replay");
    assert_eq!(
        numbers.take(),
        42,
        "ABOVE THE HIGHEST, not above the last written: the journal is not sorted by arrival"
    );
}

#[test]
fn a_reopened_journal_never_hands_back_a_number_it_already_holds() {
    // ⛔ THE SHAPE OF THE DEFECT, not a restatement of the probe above: a core that restarts
    // takes numbers, and NONE of them may collide with a step already on the disk.
    let mut journal = MemoryJournal::new();
    let numbers = Progressive::starting_at(0);
    for _ in 0..5 {
        journal
            .intent(StepId::new(numbers.take()), b"intent")
            .expect("intent");
    }

    let after_restart = seeded_from(&journal).expect("replay");
    let written: Vec<u64> = journal
        .replay()
        .expect("replay")
        .iter()
        .map(|(step, _)| step.get())
        .collect();
    for _ in 0..3 {
        let fresh = after_restart.take();
        assert!(
            !written.contains(&fresh),
            "the restarted counter handed back {fresh}, which the journal already holds"
        );
    }
}
