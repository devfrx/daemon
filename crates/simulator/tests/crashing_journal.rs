//! `CrashingJournal`: what only IT promises. What EVERY journal promises is the conformance
//! suite's business — `crates/kernel/tests/journal_contract.rs` — and this type is not held to
//! it, deliberately: a journal that stops working is a LIAR by construction, and gotcha #50
//! says a fake may break a contract when the test speaks about the breaking.

use kernel::ports::journal::{Journal, JournalError, StepId};
use simulator::journal::CrashingJournal;

const WRITES: u64 = 8;

#[test]
fn it_falls_at_the_write_it_was_told_to_fall_at() {
    // ⛔ NOT "it falls somewhere": at THE write. The number is handed in rather than drawn, so
    // this probe does not depend on the generator — that is `the_same_seed_chooses_the_same_
    // write`'s job.
    let mut journal = CrashingJournal::falling_at(2);

    assert_eq!(journal.intent(StepId::new(1), b"one"), Ok(()));
    assert_eq!(journal.outcome(StepId::new(1), b"one done"), Ok(()));
    assert_eq!(
        journal.intent(StepId::new(2), b"two"),
        Err(JournalError::NotDurable)
    );
}

#[test]
fn after_the_fall_every_later_write_is_refused_too() {
    // ⛔ THE DIFFERENCE BETWEEN A CRASH AND A BAD DISK, and it is decision D2. A journal that
    // refused once and then worked again would let the other interleaved activities carry on
    // writing after the process was supposed to be gone.
    let mut journal = CrashingJournal::falling_at(0);

    assert_eq!(
        journal.intent(StepId::new(1), b"one"),
        Err(JournalError::NotDurable)
    );
    assert_eq!(
        journal.intent(StepId::new(2), b"two"),
        Err(JournalError::NotDurable)
    );
    assert_eq!(
        journal.outcome(StepId::new(2), b"two done"),
        Err(JournalError::NotDurable)
    );
    assert_eq!(
        journal.note(StepId::new(2), b"a note"),
        Err(JournalError::NotDurable)
    );
}

#[test]
fn what_was_written_before_the_fall_survives() {
    // The archive the reconciliation will read after the restart.
    let mut journal = CrashingJournal::falling_at(1);
    assert_eq!(journal.intent(StepId::new(1), b"one"), Ok(()));
    assert_eq!(
        journal.outcome(StepId::new(1), b"one done"),
        Err(JournalError::NotDurable)
    );

    let survivor = journal.into_survivor();
    assert_eq!(
        survivor.replay().expect("replay"),
        vec![(StepId::new(1), b"one".to_vec())]
    );
}

#[test]
fn a_write_the_protocol_refuses_does_not_consume_a_crash_position() {
    // ⛔ GOTCHA #17 FROM THE OTHER SIDE, and the reason this probe exists at all. The crash
    // point is drawn against how many writes the scenario REALLY performs. A write the
    // write-ahead protocol refuses never reached storage, so if it consumed a position the
    // fall would drift EARLIER than the drawn point — and with a point near the last write it
    // would not fire at all, which is the vacuous green this whole type exists to make
    // impossible.
    //
    // ⚠️ It is also the only probe that makes an INNER write fail. Without it the declared
    // behaviour of `may_write` — "the counter moves only on an Ok" — is a sentence in a
    // comment that nothing holds, which is gotcha #45.
    let mut journal = CrashingJournal::falling_at(2);

    assert_eq!(journal.intent(StepId::new(1), b"one"), Ok(()));

    // Refused by the port: step 2 has no intent. It must move nothing.
    assert_eq!(
        journal.outcome(StepId::new(2), b"orphan"),
        Err(JournalError::OutOfOrder)
    );
    assert!(!journal.has_fallen(), "a refused write is not the fall");
    assert_eq!(journal.writes_done(), 1, "a refused write reached no storage");

    // So the write that falls is still the third one that really reaches the archive.
    assert_eq!(journal.outcome(StepId::new(1), b"one done"), Ok(()));
    assert_eq!(journal.writes_done(), 2);
    assert_eq!(
        journal.note(StepId::new(1), b"a note"),
        Err(JournalError::NotDurable)
    );
}

#[test]
fn a_journal_told_not_to_crash_never_falls() {
    // ⛔ THE OTHER DIRECTION (rule 3 of §7.1.1): a control that fires where it must not is
    // worse than one that is absent. C7a rests entirely on this one.
    let mut journal = CrashingJournal::without_crash();
    for step in 0..64u64 {
        assert_eq!(journal.intent(StepId::new(step), b"i"), Ok(()), "step {step}");
        assert_eq!(journal.outcome(StepId::new(step), b"o"), Ok(()), "step {step}");
    }
    assert!(!journal.has_fallen());
    assert_eq!(journal.writes_done(), 128);
}

#[test]
fn the_same_seed_chooses_the_same_write() {
    let first = CrashingJournal::from_seed(99, WRITES);
    let second = CrashingJournal::from_seed(99, WRITES);
    assert_eq!(first.falls_at(), second.falls_at());
}

#[test]
fn the_drawn_point_lies_inside_the_writes_the_scenario_performs() {
    // ⛔ GOTCHA #17: injecting a fault where the code never arrives is a VACUOUS proof that
    // looks like a success. If the point could land past the last write, some seeds would
    // simply never crash and the campaign would report green for having done nothing.
    for seed in 0..500u64 {
        let point = CrashingJournal::from_seed(seed, WRITES).falls_at();
        assert!(point < WRITES, "seed {seed} drew {point}, outside 0..{WRITES}");
    }
}

#[test]
fn different_seeds_choose_different_points() {
    // ⛔ AND THE OTHER HALF OF #17: a point that never moves would make five hundred seeds one
    // single experiment repeated.
    let mut seen = std::collections::BTreeSet::new();
    for seed in 0..500u64 {
        seen.insert(CrashingJournal::from_seed(seed, WRITES).falls_at());
    }
    assert!(seen.len() > 1, "the point never moves: {} distinct", seen.len());
}

#[test]
fn has_fallen_says_no_until_it_falls() {
    // The campaign's non-vacuity oracle: without it, "the run produced no doubt" and "the
    // crash never fired" are the same green.
    let mut journal = CrashingJournal::falling_at(1);
    assert!(!journal.has_fallen());
    assert_eq!(journal.intent(StepId::new(1), b"one"), Ok(()));
    assert!(!journal.has_fallen());
    assert_eq!(
        journal.outcome(StepId::new(1), b"one done"),
        Err(JournalError::NotDurable)
    );
    assert!(journal.has_fallen());
}
