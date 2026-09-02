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
    // the first match. ⚠️ Measured, and half of (b) escapes THIS test — under a hypothesis
    // that used to be an accident of the test and IS NOW THE CONTRACT: with AT MOST ONE INTENT
    // PER STEP, reversing the intents among themselves changes no answer here, because each
    // step still meets its own intent before its own outcome. ⛔ SINCE 2026-08-10 THE
    // HYPOTHESIS CANNOT BE DROPPED — `intent` refuses a second one for the same step — so the
    // three-call witness this line used to name is no longer constructible, and the sentence
    // that named it has been rewritten rather than left pointing at a deleted test. ✅ Measured
    // again after the guard, because a killer that goes away in silence is the whole danger:
    // writing the intents at the HEAD is still caught, by the CONFORMANCE suite this time —
    // promise 4, the write order of `replay` across steps, which did not exist when the old
    // sentence was written. The mutation changed owner; it did not survive.
    //
    // ⚠️ AND IT DOES NOT PROVE THE WHOLE OF THE WRITE ORDER `journal.rs` claims. The global
    // order ACROSS steps is what `replay` owes — SINCE 2026-08-10, when the port gained it, and
    // this line used to say "`replay` does not exist yet" — and it is held from outside, by
    // `crates/kernel/tests/journal_contract.rs`, against BOTH implementations. What is held
    // here is the order WITHIN a step under interleaving, which conformance does not reach:
    // `replay` hands back identities and bytes, and telling an intent from an outcome is the
    // kernel's job. That is what THIS test asks, and NOT "as much as the surface can be asked".
    // An earlier version of this line said the second, and it was false: that is the sentence
    // that stops the next reader from looking, so it does not get written again.
    //
    // ⛔ RECALL OF 2026-09-02: THE TWO CLAIMS ABOVE ARE FALSE AGAINST THE PORT. `replay` owes
    // the WHOLE write order and not only the order ACROSS steps — `ports/journal.rs` says
    // "Re-reads EVERYTHING, in write order" and "WRITE ORDER IS PART OF THE PROMISE" — and
    // conformance DOES reach inside a step: promise 8(c) asserts `[(step, intent), (step, note)]`,
    // two records of one step in write order. What the suite does not exercise is TWO NOTES upon
    // ONE step: measured, of the four `.note` calls in `assert_journal_contract` two are asserted
    // refused and the two that succeed sit on different steps. So the scope claimed above is
    // wider than the truth, and it is the SUITE that stops, never the contract.
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
fn a_second_intent_on_the_same_step_is_refused() {
    // ⛔ THIS TEST CHANGED OBJECT ON 2026-08-10, AND THE OLD OBJECT IS WORTH KNOWING. It was
    // `a_second_intent_on_the_same_step_reads_back_the_first`, and it pinned a behaviour that
    // was UNDECIDED — `intent` had no guard, a second one was accepted in silence — so that
    // whoever decided would change a red instead of getting a surprise. The decision was taken
    // where it binds BOTH implementations, the conformance suite (promise 6): one intent per
    // step, ADR-0007's own wording. The pin has done its job and now holds the answer.
    //
    // The second assertion is the half that gets forgotten: refusing is not enough if the
    // second intent went in anyway. `read_back` still answering with the first proves the
    // refusal did not half-happen.
    let mut journal = MemoryJournal::new();
    let step = StepId::new(1);

    journal.intent(step, b"the first intent").expect("intent");

    assert_eq!(
        journal.intent(step, b"the second intent"),
        Err(JournalError::OutOfOrder)
    );

    assert_eq!(
        journal.read_back(step).expect("read back"),
        b"the first intent".to_vec(),
        "the second intent was refused and written anyway"
    );
}

#[test]
fn an_intent_for_another_step_is_still_accepted() {
    // ⛔ THE COUNTER-PROBE OF THE GUARD ABOVE, and it is the direction one forgets (§7.1.1,
    // rule 3): that the guard looks at WHICH step, not merely at whether anything is there.
    // It is the same question `an_outcome_is_refused_when_the_intent_belongs_to_another_step`
    // asks of `outcome`, one operation over.
    //
    // ⚠️ AND IT IS NOT A MEASURED HOLE — THE FIRST DRAFT OF THIS COMMENT CLAIMED IT WAS, AND
    // THE MEASUREMENT SAID OTHERWISE. Writing the guard as `if !self.entries.is_empty()` — one
    // that refuses every intent after the first, anywhere — is caught by SIX tests, not by
    // this one alone: `each_step_reads_back_its_own_first_record` here, and five in the
    // conformance suite, which die on the SETUP of promise 4 because it writes two intents for
    // two different steps. So this test is not what stands between that mutation and a green
    // bench. What it buys is that the property is NAMED: a rule held only as a side effect of
    // a test about something else is a rule the next reader re-decides, which is the same
    // argument that keeps `read_back_returns_the_intent_and_not_the_outcome` next to its
    // neighbour. ⛔ The claim was written before the measure and the measure contradicted it;
    // it is corrected here rather than quietly dropped.
    let mut journal = MemoryJournal::new();

    journal
        .intent(StepId::new(1), b"the first step")
        .expect("intent");

    assert_eq!(journal.intent(StepId::new(2), b"the second step"), Ok(()));
}

#[test]
fn a_note_upon_a_step_nobody_opened_is_refused() {
    // ⛔ THE GUARD `note` SHARES WITH `outcome`, and it is the same sentence one operation over:
    // a note is an annotation UPON something, and a step nobody opened is not something. What it
    // does NOT share is `intent`'s guard — see the test below.
    let mut journal = MemoryJournal::new();

    assert_eq!(
        journal.note(StepId::new(3), b"a note about nothing"),
        Err(JournalError::OutOfOrder)
    );
}

#[test]
fn a_note_is_refused_when_the_intent_belongs_to_another_step() {
    // ⛔ THE HOLE THAT WAS MEASURED ON `outcome` AND WOULD HAVE BEEN REBUILT HERE. The refusal
    // above is checked on an EMPTY journal, where "no intent for this step" and "no intent at
    // all" are the same sentence — so a `note` whose guard forgot `e.step == step` would pass
    // it. This is the twin of `an_outcome_is_refused_when_the_intent_belongs_to_another_step`,
    // written because the defect it catches is not hypothetical: it was found on `outcome`.
    let mut journal = MemoryJournal::new();
    journal
        .intent(StepId::new(1), b"the step that did happen")
        .expect("intent");

    assert_eq!(
        journal.note(StepId::new(2), b"a note upon a step that did not"),
        Err(JournalError::OutOfOrder)
    );
}

#[test]
fn a_step_may_carry_many_notes_and_they_keep_their_order() {
    // ⛔ THE DIRECTION ONE FORGETS (§7.1.1 rule 3), and here it is a DECISION rather than a
    // permission left lying about. One intent per step is ADR-0007's own wording, so a second is
    // outside the model; nothing says how many times one interaction with the world may consult
    // external content, and a caller that promotes twice within a step is ordinary. A `note`
    // that inherited `intent`'s guard would refuse the second, and this is where that goes red.
    //
    // ⚠️ AND THE ORDER WITHIN THE STEP IS THE HALF CONFORMANCE DOES NOT REACH, for the reason
    // `each_step_reads_back_its_own_first_record` gives: `replay` owes the order ACROSS steps,
    // and what is asked here is the order WITHIN one.
    // ⛔ RECALL OF 2026-09-02: "the half conformance does not reach" IS FALSE, and the correction
    // is NOT restated here — it is in the recall inside
    // `each_step_reads_back_its_own_first_record`, which this paragraph already points at. What
    // the suite does not exercise is TWO NOTES upon ONE step, which is exactly what this test
    // writes, so this is the case that is held here and nowhere else.
    let mut journal = MemoryJournal::new();
    let step = StepId::new(1);
    journal
        .intent(step, b"what it set out to do")
        .expect("intent");

    assert_eq!(journal.note(step, b"the first note"), Ok(()));
    assert_eq!(journal.note(step, b"the second note"), Ok(()));

    let replayed = journal.replay().expect("replay");
    let bytes: Vec<&[u8]> = replayed.iter().map(|(_, b)| b.as_slice()).collect();
    assert_eq!(
        bytes,
        vec![
            b"what it set out to do".as_slice(),
            b"the first note".as_slice(),
            b"the second note".as_slice(),
        ]
    );
}

#[test]
fn a_note_does_not_take_the_intents_place_in_read_back() {
    // ⛔ `read_back` ANSWERS WITH THE FIRST RECORD OF THE STEP, and a note must not become that
    // record. The decision is the one `read_back_returns_the_intent_and_not_the_outcome` names —
    // the intent is what says WHAT THE STEP WAS — and a note is by construction never the first
    // thing a step carries, because `note` refuses a step with no intent. This pins that the
    // two mechanisms agree instead of leaving it to follow from them.
    let mut journal = MemoryJournal::new();
    let step = StepId::new(9);
    journal
        .intent(step, b"what it set out to do")
        .expect("intent");
    journal
        .note(step, b"and what it read on the way")
        .expect("note");

    assert_eq!(
        journal.read_back(step).expect("read back"),
        b"what it set out to do".to_vec()
    );
}

// ⛔ AND ONE TEST WAS WRITTEN HERE AND THEN REMOVED, WHICH IS RECORDED INSTEAD OF THE TEST.
// `a_note_does_not_admit_an_outcome_for_a_step_that_never_had_an_intent` claimed to be the
// counter-probe of the internal `EntryKind::Note` variant — that filing a note under
// `EntryKind::Intent` would make `has_intent` answer `true` for a step whose only record is a
// note, and V6 would fall. THE MUTATION WAS RUN AND NOTHING WENT RED, that test included, and
// the reason is sound rather than a gap: `note` itself REFUSES a step with no intent, so a
// note-only step cannot be built through the port, so `has_intent` can only be made true by a
// note on a step that already had an intent. The variant is genuinely unobservable from out
// here, exactly as `crates/simulator/src/journal.rs` declares beside it.
//
// ⚠️ THE TEST WENT AND THE MEASUREMENT STAYED, because the two failures are not the same size: a
// probe whose comment claims a defect it cannot see teaches the next reader that the defect is
// covered, which is worse than an uncovered defect somebody knows about. Gotcha #15 — a true
// measurement, of another thing.

#[test]
fn prune_refuses_and_leaves_the_record_where_it_was() {
    // ⚠️ THIS TEST HELD "THE DECLARED NON-IMPLEMENTATION" UNTIL 2026-08-10 and expected
    // `Err(Missing)` — `prune` refused EVERYTHING then, and the comment here explained why. Task
    // 11 gave it the one rule of ADR-0018 that needs no fingerprint, so the refusal is now a
    // REASONED one and carries its own word. The paragraph is replaced rather than deleted: it
    // was true, and a reader meeting `StepInDoubt` deserves to see when `Missing` stopped being
    // the answer.
    //
    // ⛔ WHAT IS HELD HERE AND NOT IN THE CONFORMANCE SUITE, which is the whole reason this test
    // survives promise 7: that the refusal DID NOT PRUNE ANYWAY. The suite reads the error and
    // stops there; an irreversible operation that half-happened and then reported failure is
    // worse than one nobody wrote, and no promise looks.
    let mut journal = MemoryJournal::new();
    let step = StepId::new(5);
    journal
        .intent(step, b"a payload worth keeping")
        .expect("intent");

    assert_eq!(journal.prune(step), Err(JournalError::StepInDoubt));

    assert_eq!(
        journal.read_back(step).expect("read back"),
        b"a payload worth keeping".to_vec(),
        "prune refused and pruned anyway"
    );

    // ⛔ AND THE THIRD ANSWER, WHICH THE CONFORMANCE SUITE DOES NOT PIN — declared here rather
    // than left to be discovered. `prune` has three answers: `Missing` for a step nobody wrote,
    // `StepInDoubt` for an open one, `Ok` for a reconciled one. Promises 7 and 7b hold the last
    // two ACROSS BOTH implementations; this one is held for the in-memory double alone, so the
    // two could in principle diverge on it with nothing going red. It is not a hole this task
    // opened — both refused every prune with `Missing` before it — and closing it costs a ninth
    // promise with a liar of its own, which no measurement asks for yet.
    assert_eq!(
        journal.prune(StepId::new(404)),
        Err(JournalError::Missing),
        "a step nobody ever wrote is Missing, not in doubt"
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
