//! The in-memory journal, checked on its own. What BOTH implementations promise lives in
//! the conformance suite (`kernel/tests/journal_contract.rs`); this file holds only what is
//! true of THIS one.

use kernel::ports::journal::{Journal, JournalError, StepId};
use simulator::journal::MemoryJournal;

#[test]
fn what_intent_writes_read_back_returns_unchanged() {
    let mut journal = MemoryJournal::new();
    let step = StepId::new(7);

    journal
        .intent(step, b"the bytes of a record")
        .expect("intent");

    assert_eq!(
        journal.read_back(step).expect("read back"),
        b"the bytes of a record".to_vec()
    );
}

#[test]
fn a_step_never_written_is_missing_and_not_empty() {
    let journal = MemoryJournal::new();

    assert_eq!(
        journal.read_back(StepId::new(1)),
        Err(JournalError::Missing)
    );
}

#[test]
fn an_outcome_without_an_intent_is_refused() {
    // V6: nothing executes before the intent is durable. A journal that accepts an outcome
    // for a step it never saw an intent for leaves the write-ahead protocol resting on the
    // caller's diligence — the same argument with which `boundary_promotion.rs` requires
    // that a journal which refuses ALSO refuses the promotion.
    let mut journal = MemoryJournal::new();

    assert_eq!(
        journal.outcome(StepId::new(3), b"too early"),
        Err(JournalError::OutOfOrder)
    );
}

#[test]
fn an_outcome_after_its_intent_is_accepted() {
    // ⛔ THE HAPPY PATH OF THE WRITE-AHEAD PROTOCOL, and the refusal above is vacuous without
    // it. MEASURED, AND THE CLAIM IS BOUND TO WHAT WAS MEASURED AND TO WHEN (gotcha #31, which
    // is what happens to a claim tied to a container that grows): an `outcome` answering
    // `Err(OutOfOrder)` ALWAYS leaves green ALL FOUR TESTS THE PLAN DICTATED — which is the
    // whole of the plan's closing criterion for this task, satisfied by a journal that records
    // no outcome at all. Against the file as it ships it takes three down: this one and the
    // two that write an outcome before reading back. It is the direction of a control that
    // gets forgotten (§7.1.1, rule 3), on the only sequence the protocol actually walks —
    // intent, then effect, then outcome.
    let mut journal = MemoryJournal::new();
    let step = StepId::new(4);

    journal
        .intent(step, b"what it set out to do")
        .expect("intent");

    assert_eq!(journal.outcome(step, b"what came of it"), Ok(()));
}

#[test]
fn an_outcome_is_refused_when_the_intent_belongs_to_another_step() {
    // ⛔ MEASURED HOLE, not a hypothesis: with this test absent, dropping `e.step == step` from
    // `has_intent` — so that ANY intent anywhere admits ANY outcome — left the whole file
    // green. The two tests above cannot see it between them, and the reason is worth writing
    // down: the refusal is checked on an EMPTY journal, where "no intent for this step" and
    // "no intent at all" are the same sentence. Gotcha #45 — the bench that agrees with the
    // expectation because it never puts it under strain.
    let mut journal = MemoryJournal::new();
    journal
        .intent(StepId::new(1), b"the step that did happen")
        .expect("intent");

    assert_eq!(
        journal.outcome(StepId::new(2), b"the step that did not"),
        Err(JournalError::OutOfOrder)
    );
}

#[test]
fn read_back_returns_the_intent_and_not_the_outcome() {
    // ⛔ WHICH OF THE TWO RECORDS COMES BACK IS A DECISION AND NOT AN ACCIDENT OF `find`, and
    // milestone 3's reconciliation is built on top of the answer. The answer is THE INTENT,
    // and `journal.rs` carries the reason next to the code that implements it. This test is
    // where the decision is NAMED: `each_step_reads_back_its_own_first_record` also goes red
    // when the last is returned instead of the first — measured — but it goes red for its own
    // reason, and a decision nobody named is one the next reader re-takes.
    let mut journal = MemoryJournal::new();
    let step = StepId::new(9);

    journal
        .intent(step, b"what it set out to do")
        .expect("intent");
    journal
        .outcome(step, b"what came of it")
        .expect("outcome after its intent");

    assert_eq!(
        journal.read_back(step).expect("read back"),
        b"what it set out to do".to_vec(),
        "read_back returned the outcome: reconciliation reads a step to learn WHAT IT WAS"
    );
}

#[test]
fn each_step_reads_back_its_own_first_record() {
    // ⛔ TWO STEPS, INTERLEAVED, and it takes two to see two distinct things that one hides.
    // (a) a `read_back` that ignored its `step` argument and returned the first entry it owns
    // would pass every single-step test above; (b) a store that put new entries at the FRONT
    // instead of the back would answer each step with its OUTCOME, because `read_back` takes
    // the first match. ⚠️ Measured, and half of (b) escapes THIS test — but only under an
    // hypothesis this test happens to satisfy: WITH AT MOST ONE INTENT PER STEP, reversing the
    // intents among themselves changes no answer, because each step still meets its own intent
    // before its own outcome. Drop that hypothesis and the reversal becomes visible in three
    // calls with no outcome anywhere, which is what
    // `a_second_intent_on_the_same_step_reads_back_the_first` holds.
    //
    // ⚠️ AND IT DOES NOT PROVE THE WHOLE OF THE WRITE ORDER `journal.rs` claims. The global
    // order ACROSS steps is what `replay` will owe, and `replay` does not exist yet: the port
    // gains it with its first consumer, and until then nothing outside this crate can observe
    // it. What is held here is the order WITHIN a step under interleaving — which is what THIS
    // test asks, and NOT "as much as the surface can be asked". An earlier version of this line
    // said the second, and it was false: that is the sentence that stops the next reader from
    // looking, so it does not get written again.
    let mut journal = MemoryJournal::new();
    let first = StepId::new(1);
    let second = StepId::new(2);

    journal
        .intent(first, b"intent of the first")
        .expect("intent");
    journal
        .intent(second, b"intent of the second")
        .expect("intent");
    journal
        .outcome(first, b"outcome of the first")
        .expect("outcome");
    journal
        .outcome(second, b"outcome of the second")
        .expect("outcome");

    assert_eq!(
        journal.read_back(first).expect("read back"),
        b"intent of the first".to_vec()
    );
    assert_eq!(
        journal.read_back(second).expect("read back"),
        b"intent of the second".to_vec()
    );
}

#[test]
fn a_second_intent_on_the_same_step_reads_back_the_first() {
    // ⛔ THIS PINS WHAT HAPPENS TODAY, AND WHAT HAPPENS TODAY IS UNDECIDED. `intent` has NO
    // GUARD: a second intent for a step that already carries one is accepted in silence, and
    // `read_back` answers with the first. Whether it OUGHT to be accepted binds both
    // implementations and is therefore the conformance suite's to settle — it is written down
    // as an open entry in `docs/porta-di-qualita.md` rather than as a note, because a note is
    // read and forgotten (gotcha #36). Until it is settled the behaviour is nailed down, so
    // that changing it is a red and not a surprise.
    //
    // ⚠️ AND IT IS THE WITNESS that reversing the INTENTS among themselves is observable from
    // outside after all: three calls, no outcome anywhere. Measured — without this test that
    // mutation leaves the whole file green, which is why the sentence in
    // `each_step_reads_back_its_own_first_record` now carries the hypothesis that made it true.
    let mut journal = MemoryJournal::new();
    let step = StepId::new(1);

    journal.intent(step, b"the first intent").expect("intent");
    journal
        .intent(step, b"the second intent")
        .expect("a second intent is accepted today: there is no guard to refuse it");

    assert_eq!(
        journal.read_back(step).expect("read back"),
        b"the first intent".to_vec()
    );
}

#[test]
fn prune_refuses_and_leaves_the_record_where_it_was() {
    // ⛔ THE DECLARED NON-IMPLEMENTATION, held by a test so that it stays declared. Retention
    // is out of this milestone (decision D7) because the fingerprint a pruned record carries
    // demands a hash function, and in the kernel that is a NEW ENTRY IN THE LIST OF ADR-0031 —
    // a deliberate act that wants a measurement nobody has made. So `prune` refuses.
    //
    // The second assertion is the half that matters: refusing is not enough if the payload
    // went anyway. An irreversible operation that half-happened and then reported failure is
    // worse than one that is not implemented at all.
    let mut journal = MemoryJournal::new();
    let step = StepId::new(5);
    journal
        .intent(step, b"a payload worth keeping")
        .expect("intent");

    assert_eq!(journal.prune(step), Err(JournalError::Missing));

    assert_eq!(
        journal.read_back(step).expect("read back"),
        b"a payload worth keeping".to_vec(),
        "prune refused and pruned anyway"
    );
}

#[test]
fn the_memory_journal_does_not_survive_being_dropped() {
    // ⛔ THIS IS WHY THIS TEST IS HERE AND NOT IN THE CONFORMANCE SUITE. Durability across a
    // process restart is a promise of the REAL implementation only, and asserting it in the
    // shared suite would turn a CORRECT implementation red — gotcha #44, declared before it
    // was discovered this time.
    //
    // ⚠️ AND IT IS NOT THE SAME SENTENCE AS `a_step_never_written_is_missing_and_not_empty`,
    // WHICH WAS MEASURED AND NOT ARGUED. The suspicion was fair — the plan's own mutation
    // table lists three mutations for four tests and this is the one it leaves uncovered — but
    // the measurement contradicts it. Give this crate PROCESS-GLOBAL state, a
    // `static AtomicBool` set by `intent` and consulted by `read_back`, which `no_std` and
    // `#![forbid(unsafe_code)]` BOTH permit, and this test goes red while the other stays
    // green when each is run ON ITS OWN. What is held here is that a journal keeps nothing
    // outside itself — the same family as gotcha #12, where the state seeded per PROCESS
    // rather than per instance is the one a deterministic run cannot contain.
    //
    // ⚠️ AND "ON ITS OWN" CARRIES THE WHOLE PROOF, so it says why instead of sitting there.
    // The mutant's flag is per PROCESS, so in a shared run whichever test writes an intent
    // first poisons the twin too, and WHICH ONE THAT IS DEPENDS ON THE POPULATION OF THE FILE.
    // Measured, and the two figures are the argument: with nine tests the twin survived 5 runs
    // out of 5; with the tenth added — it writes intents and its name sorts ahead of the
    // twin's — the twin went down 20 out of 20. Nothing about the journal changed between the
    // two. Run one test per process and the mutant is unambiguous every time: twin green, this
    // one red. ⛔ The instability belongs to THE MUTANT, not to what ships: the journal here
    // holds no global state and comes out green in every arrangement.
    let mut journal = MemoryJournal::new();
    journal.intent(StepId::new(1), b"gone").expect("intent");
    drop(journal);

    let fresh = MemoryJournal::new();
    assert_eq!(fresh.read_back(StepId::new(1)), Err(JournalError::Missing));
}
