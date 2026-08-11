//! `CrashingJournal`: what only IT promises. What EVERY journal promises is the conformance
//! suite's business — `crates/kernel/tests/journal_contract.rs` — and this type is not held to
//! it, deliberately: a journal that stops working is a LIAR by construction, and gotcha #50
//! says a fake may break a contract when the test speaks about the breaking.

use kernel::ports::journal::{Journal, JournalError, StepId};
use simulator::journal::CrashingJournal;

const WRITES: u64 = 8;

/// How many seeds the probes over the generator sweep. ⚠️ It is a constant rather than a
/// literal because the number also appears in what the assertions SAY, and a count duplicated
/// between code and prose is gotcha #31 waiting for the day one of the two is edited.
const SEEDS: u64 = 500;

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
fn after_the_fall_pruning_is_refused_too_but_it_never_causes_the_fall() {
    // ⛔ `prune` IS THE ONLY MUTATING OPERATION OUTSIDE THE THREE THE CRASH POINT COUNTS, so it
    // has to answer two questions at once, and they pull in opposite directions: a dead process
    // must not prune, AND a prune must not arm or consume the fall — or the crash would land
    // somewhere other than the drawn point.
    let mut journal = CrashingJournal::falling_at(2);

    assert_eq!(journal.intent(StepId::new(1), b"one"), Ok(()));
    assert_eq!(journal.outcome(StepId::new(1), b"one done"), Ok(()));

    // Two writes done, the next one falls. A prune here must change neither fact.
    assert_eq!(journal.prune(StepId::new(1)), Ok(()));
    assert!(!journal.has_fallen(), "a prune is not the fall");
    assert_eq!(
        journal.writes_done(),
        2,
        "a prune reached none of the counted writes"
    );

    // The fall still lands on the third counted write, not earlier and not later.
    assert_eq!(
        journal.intent(StepId::new(2), b"two"),
        Err(JournalError::NotDurable)
    );
    // And once it has fallen, pruning is refused like everything else.
    assert_eq!(journal.prune(StepId::new(1)), Err(JournalError::NotDurable));
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
    // ⚠️ MEASURED ON 2026-08-11 AND WRITTEN AS A MEASUREMENT, not as a property of this file:
    // of the ten probes here on that day, this was the ONLY one that made an INNER write fail.
    // Exclusivity over a set that grows is the claim that ages silently (gotcha #31), so it
    // carries its date instead of the word "always". What the exclusivity means is the point:
    // without this probe the declared behaviour of `may_write` — "the counter moves only on an
    // Ok" — is a sentence in a comment that nothing holds, which is gotcha #45.
    let mut journal = CrashingJournal::falling_at(2);

    assert_eq!(journal.intent(StepId::new(1), b"one"), Ok(()));

    // Refused by the port: step 2 has no intent. It must move nothing.
    assert_eq!(
        journal.outcome(StepId::new(2), b"orphan"),
        Err(JournalError::OutOfOrder)
    );
    assert!(!journal.has_fallen(), "a refused write is not the fall");
    assert_eq!(
        journal.writes_done(),
        1,
        "a refused write reached no storage"
    );

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
        assert_eq!(
            journal.intent(StepId::new(step), b"i"),
            Ok(()),
            "step {step}"
        );
        assert_eq!(
            journal.outcome(StepId::new(step), b"o"),
            Ok(()),
            "step {step}"
        );
    }
    assert!(!journal.has_fallen());
    assert_eq!(journal.writes_done(), 128);
}

#[test]
fn the_same_seed_chooses_the_same_write() {
    // ⚠️ UNFALSIFIABLE BY CONSTRUCTION, and declared so rather than removed (gotcha #44).
    // `from_seed` is a pure function of its two arguments, so no implementation of it could
    // make this red; the real determinism — that the SEQUENCE repeats, which is the property
    // the campaign rests on — is held where it can actually fail, by
    // `crates/simulator/tests/seeded_rng.rs::the_same_seed_gives_the_same_sequence`. This is
    // here as a NAMED STATEMENT of what this type promises its callers, not as coverage, and
    // it must not be counted as either.
    let first = CrashingJournal::from_seed(99, WRITES);
    let second = CrashingJournal::from_seed(99, WRITES);
    assert_eq!(first.falls_at(), second.falls_at());
}

#[test]
fn the_drawn_point_lies_inside_the_writes_the_scenario_performs() {
    // ⛔ GOTCHA #17: injecting a fault where the code never arrives is a VACUOUS proof that
    // looks like a success. If the point could land past the last write, some seeds would
    // simply never crash and the campaign would report green for having done nothing.
    for seed in 0..SEEDS {
        let point = CrashingJournal::from_seed(seed, WRITES).falls_at();
        assert!(
            point < WRITES,
            "seed {seed} drew {point}, outside 0..{WRITES}"
        );
    }
}

#[test]
fn every_write_of_the_scenario_can_be_the_one_that_falls() {
    // ⛔ THE OTHER HALF OF #17, AND `> 1` WAS NOT ENOUGH FOR IT. A point that never moves would
    // make five hundred seeds one experiment repeated — but so would a generator that only ever
    // draws two of the eight, and the position that matters most is THE LAST one: that is where
    // the surviving archive is fullest and the set of steps in doubt is largest, which is
    // exactly what C7b measures.
    //
    // ⚠️ DECLARED COUPLING: this holds because `SEEDS` is far above `WRITES`, not by construction
    // of `below`. Measured on 2026-08-11 over these seeds: every one of the eight positions comes
    // out, 57 to 69 times each.
    let mut seen = std::collections::BTreeSet::new();
    for seed in 0..SEEDS {
        seen.insert(CrashingJournal::from_seed(seed, WRITES).falls_at());
    }
    assert_eq!(
        seen.len(),
        WRITES as usize,
        "{SEEDS} seeds reached only {:?} of the {WRITES} writes",
        seen
    );
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
