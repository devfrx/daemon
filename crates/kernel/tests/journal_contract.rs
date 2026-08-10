// THE CONFORMANCE SUITE OF THE `journal` PORT (§7.4.6). What it is worth is exactly what the
// DST campaign is worth: the campaign runs against the in-memory double, and every run is
// worth the evidence that the double and `redb` answer the same contract.
//
// ⛔ REGULAR COMMENTS AND NOT `//!`, BECAUSE THIS FILE IS `include!`d.
// `crates/platform/tests/journal_contract_real.rs` expands it IN ITEM POSITION, and an inner
// attribute — which is what `//!` desugars to — is not permitted there.
//
// ⚠️ THIS PARAGRAPH STOOD IN THE FUTURE TENSE UNTIL 2026-08-10, and it is dated rather than
// quietly reworded. It said "this file WILL BE `include!`d ... task 9 of this milestone", and
// the tense was right then: writing in the present would have described a file that did not
// exist. Task 9 landed and the file does exist, so the same sentence is now wrong in the other
// direction. A tense is a status claim like any other (gotcha #31).
//
// ⛔ THE ASSERTIONS LIVE HERE AND NOWHERE ELSE. Two copies would diverge, and the first one
// that diverged would lie in silence — a conformance suite that no longer compares anything
// still prints `ok`. An integration test is a crate of its own and cannot import another
// test's items, so textual inclusion is the mechanism, not a shortcut.
//
// ⛔ WHAT IS DELIBERATELY ABSENT: durability across a process restart. It is a promise of the
// REAL implementation only — the in-memory double cannot make it and is CORRECT not to.
// Asserting it here would turn a correct implementation red, which is gotcha #44. For the
// double it lives in `crates/simulator/tests/memory_journal.rs`; for the real one it lives in
// `crates/platform/tests/file_journal.rs` — this clause read "it WILL live in
// `crates/platform/tests/`, where today there is no journal test at all" until task 8 wrote
// that file.
//
// ⚠️ DECLARED COST, AND IT HAS BEEN PAID SINCE 2026-08-10 — this sentence read "AND IT IS NOT
// YET BEING PAID" until task 9. `include!` carries the `#[test]` functions of this file along
// with it, so the ELEVEN tests below RUN A SECOND TIME inside `platform`'s binary: that binary
// reports TWELVE tests, these eleven plus the one that builds the real journal. It buys the
// single copy of the assertions and costs a few milliseconds — nothing here touches the disk or
// sleeps, and that stayed true when the real journal started pruning: `prune` runs against the
// in-memory double here and against the file only in `platform`'s own copy.
// ⚠️ The figure said "eight" and then "ten" during 2026-08-10, as promise 8 brought a liar, the
// substring constraint became a test of its own, and promise 7b brought the ninth liar; counted
// rather than remembered.

use kernel::ports::journal::{Journal, JournalError, StepId};

/// ⛔ ONE MESSAGE PER PROMISE, AND NOT ONE SHARED. With a shared message a liar caught by
/// promise 1 would be indistinguishable from one caught by promise 4 — in exactly the place
/// built to distinguish them — and a test claiming to pin the second would be satisfied by
/// the first. `reactor_contract.rs` learned this at task 7 of milestone 2; it is not
/// relearned here.
///
/// ⛔ AND NO MESSAGE IS A SUBSTRING OF ANOTHER, which is a constraint and not a coincidence:
/// the negative tests match with `contains`, so two messages sharing a prefix long enough to
/// be the whole of one of them would make that one satisfiable by the other. They share
/// `journal contract violated: ` and diverge immediately after.
pub const READ_BACK_MESSAGE: &str =
    "journal contract violated: what `intent` wrote must come back from `read_back` unchanged";

/// The promise the second implementation does NOT meet by itself, which is why it is written
/// down. See promise 2 for the argument.
pub const READ_BACK_IS_THE_INTENT_MESSAGE: &str =
    "journal contract violated: after its outcome, a step must still read back its INTENT";

pub const MISSING_MESSAGE: &str =
    "journal contract violated: a step never written must answer Missing, not empty bytes";

pub const REPLAY_ORDER_MESSAGE: &str =
    "journal contract violated: `replay` must return records in WRITE ORDER";

pub const OUT_OF_ORDER_MESSAGE: &str =
    "journal contract violated: an `outcome` with no `intent` must be refused (V6)";

/// The other half of the ordering discipline, decided on 2026-08-10. See promise 6.
pub const SECOND_INTENT_MESSAGE: &str =
    "journal contract violated: a step already carrying an `intent` must refuse a second one";

pub const PRUNE_IN_DOUBT_MESSAGE: &str =
    "journal contract violated: a step IN DOUBT must never be prunable (ADR-0018)";

/// The OTHER DIRECTION of promise 7, and it carries a message of its own for the same reason
/// every other promise does: a journal caught REFUSING a reconciled step and one caught PRUNING
/// a step in doubt are opposite defects, and a shared message would name neither.
pub const PRUNE_RECONCILED_MESSAGE: &str =
    "journal contract violated: a step that HAS an outcome must be prunable";

/// The operation that arrived on 2026-08-10 with `Untrusted::promote`. See promise 8.
pub const NOTE_MESSAGE: &str = "journal contract violated: a `note` upon an open step must be \
     kept, and must never displace that step's intent";

/// Every promise the `journal` port makes, checked against ONE implementation.
///
/// It takes a FACTORY and not a journal because several assertions need one that has never
/// been written to, and once a record is in there is no going back.
///
/// ⛔ THE ORDER OF THE BLOCKS IS PART OF THE SUITE, because the suite stops at the FIRST
/// promise a journal breaks. Every liar below therefore has to survive every promise ahead of
/// its own and die on that one — which is the property each negative test measures by reading
/// the panic payload instead of settling for `is_err()`.
pub fn assert_journal_contract<J: Journal, F: Fn() -> J>(build: F) {
    // ── 1. What `intent` writes, `read_back` returns unchanged ────────────────────────────
    // ⛔ THIS IS ROAD A6 OF `crate::boundary`, and it is the reason this suite is scheduled
    // in this milestone at all. Without it a journal that answers `Ok(())` and writes nothing
    // satisfies the type boundary, and the promotion of untrusted text succeeds having
    // recorded NOTHING.
    {
        let mut journal = build();
        let step = StepId::new(7);
        let written: &[u8] = b"the bytes of a record";

        // ⛔ THE PROMISE'S OWN MESSAGE ON THE `expect`, AND IT WAS MEASURED RATHER THAN
        // FORESEEN. Promise 1 is broken in TWO ways — the wrong bytes come back, or NOTHING
        // does — and A6 is the second one: a journal that writes nothing answers `Missing`
        // here and never reaches the `assert_eq!` below. With a bare `"read_back must find it"`
        // the suite still went red, but with a payload that names no promise, and
        // `a_journal_that_writes_nothing_is_caught` reported «fired, but NOT on promise 1» —
        // the suite catching A6 and being unable to say so. Both failures carry promise 1's
        // words because both ARE promise 1.
        journal.intent(step, written).expect("intent must succeed");
        let read = journal.read_back(step).expect(READ_BACK_MESSAGE);

        assert_eq!(read.as_slice(), written, "{}", READ_BACK_MESSAGE);
    }

    // ── 2. After the outcome, `read_back` STILL answers with the intent ───────────────────
    // ⛔ THE PROMISE THE SECOND IMPLEMENTATION DOES NOT MEET BY ITSELF, and the only reason it
    // is written here rather than left to each: a table keyed on the identity of the step —
    // which is the natural shape of a `redb` table, and the natural shape of a key-value store
    // in general — answers with the LAST write, or worse OVERWRITES the intent with it. Both
    // forms are caught here.
    //
    // ⛔ WHICH OF THE TWO RECORDS COMES BACK IS A DECISION, not an accident of whichever lookup
    // the implementation happens to use. The intent is the record that says WHAT THE STEP WAS
    // FOR; hiding it behind its own outcome leaves a resumed run able to read what happened and
    // no longer able to read what it had set out to do. `crates/simulator/src/journal.rs`
    // carries the same argument next to the code that implements it.
    //
    // ⚠️ AND WITHOUT THIS BLOCK PROMISE 1 DOES NOT SEE IT: promise 1 writes an intent and no
    // outcome, and on such a step the first record and the last are THE SAME RECORD. The choice
    // only becomes observable on a COMPLETE step, which is what this block builds.
    {
        let mut journal = build();
        let step = StepId::new(9);
        let intent: &[u8] = b"what it set out to do";

        journal.intent(step, intent).expect("intent must succeed");
        journal
            .outcome(step, b"what came of it")
            .expect("an outcome after its own intent must succeed");

        let read = journal
            .read_back(step)
            .expect(READ_BACK_IS_THE_INTENT_MESSAGE);

        assert_eq!(
            read.as_slice(),
            intent,
            "{}",
            READ_BACK_IS_THE_INTENT_MESSAGE
        );
    }

    // ── 3. A step never written is Missing, not empty ─────────────────────────────────────
    // Telling "not there" from "there and empty" is the same family as gotcha #30: a bench
    // that only looks at Ok/Err does not see the WRONG ANSWER.
    {
        let journal = build();
        assert_eq!(
            journal.read_back(StepId::new(999)),
            Err(JournalError::Missing),
            "{}",
            MISSING_MESSAGE
        );
    }

    // ── 4. `replay` returns records in write order ────────────────────────────────────────
    // Reconciliation computes the doubt by walking this sequence. An arbitrary order gives it
    // an arbitrary answer — and gives it SILENTLY, which is worse.
    //
    // ⚠️ THREE WRITES AND NOT TWO, and the third is what makes the block non-vacuous: with
    // `intent(1)` then `intent(2)` alone, an implementation that sorted by step identity
    // instead of keeping write order would be indistinguishable from a conforming one. The
    // expected sequence of identities is `1, 2, 1` — not sorted, and not producible by sorting.
    //
    // ⛔ AND THE COMPARISON CARRIES THE BYTES, WHICH IS NOT THOROUGHNESS BUT A MEASURED
    // NECESSITY. Comparing the IDENTITIES alone leaves this block VACUOUS against the very
    // liar built to break it: `1, 2, 1` IS A PALINDROME, so a `replay` that hands the journal
    // back REVERSED produces the same three identities in the same three places, and
    // `ShuffledJournal` passed the whole suite. Measured, not argued — the test reported «the
    // suite is vacuous on promise 4». The records tell the two apart because the bytes do:
    // `first, second, third and last` reversed is `third and last, second, first`.
    //
    // ⛔ AND THE PAYLOAD LENGTHS ARE ASYMMETRIC ON PURPOSE — 5, 6, 14. With `first, second,
    // third` they were `5, 6, 5`, PALINDROMIC TOO, and this block was safe only because the
    // comparison happens to look at the bytes rather than at sizes. A rewrite that compared
    // lengths would rebuild the vacuity that has already been built here once, by exactly the
    // mechanism that built it. No symmetry is left for it to lean on.
    {
        let mut journal = build();
        journal.intent(StepId::new(1), b"first").expect("intent 1");
        journal.intent(StepId::new(2), b"second").expect("intent 2");
        journal
            .outcome(StepId::new(1), b"third and last")
            .expect("outcome 1");

        let replayed = journal.replay().expect("replay must succeed");
        let records: Vec<(StepId, &[u8])> = replayed
            .iter()
            .map(|(step, bytes)| (*step, bytes.as_slice()))
            .collect();

        assert_eq!(
            records,
            vec![
                (StepId::new(1), b"first".as_slice()),
                (StepId::new(2), b"second".as_slice()),
                (StepId::new(1), b"third and last".as_slice()),
            ],
            "{}",
            REPLAY_ORDER_MESSAGE
        );
    }

    // ── 5. An `outcome` with no `intent` is refused ───────────────────────────────────────
    // V6, held by the port. See the doc of `JournalError::OutOfOrder` for why this is the
    // nature of a write-ahead journal and not a policy of the kernel.
    {
        let mut journal = build();
        assert_eq!(
            journal.outcome(StepId::new(3), b"too early"),
            Err(JournalError::OutOfOrder),
            "{}",
            OUT_OF_ORDER_MESSAGE
        );
    }

    // ── 6. A SECOND `intent` on the same step is refused ─────────────────────────────────
    // ⛔ DECIDED ON 2026-08-10, EXECUTING, AND IT IS AN ADDITION TO THE CONTRACT OF A SHARED
    // PORT. Until this line the behaviour was UNDECIDED: `intent` had no guard, a second one
    // was accepted in silence, and `read_back` answered with the first. The three reasons,
    // shortest first: ADR-0007 says "the intent of EVERY step", one per step, so a second one
    // is outside the model rather than a case to discipline; it is the SYMMETRIC HALF of
    // promise 5 — V6 held by the PORT instead of by the caller's diligence, and the port
    // already held the other direction; and it costs one line now against two implementations
    // and an archive later.
    //
    // ⛔ AND IT BELONGS HERE RATHER THAN IN EITHER IMPLEMENTATION, which is the whole point.
    // Promise 2 already forces a key finer than the identity of the step, and with such a key
    // "the first intent wins" falls out for free — but that is an ACCORD BY ACCIDENT OF THE
    // KEY DESIGN, not by contract. Key on the step, which is the natural choice, and the two
    // implementations diverge WITH NOTHING GOING RED. That is the case this suite exists for.
    //
    // ⚠️ NO NEW ERROR VARIANT: `OutOfOrder` is widened rather than joined. The port declares
    // its error type "deliberately poor — a rich error type invites the kernel to branch on
    // the reason", and "an operation arrived out of order for this step" covers both halves.
    {
        let mut journal = build();
        let step = StepId::new(5);
        journal
            .intent(step, b"the first intent")
            .expect("the first intent must succeed");

        assert_eq!(
            journal.intent(step, b"the second intent"),
            Err(JournalError::OutOfOrder),
            "{}",
            SECOND_INTENT_MESSAGE
        );
    }

    // ── 7. A step IN DOUBT is never prunable ──────────────────────────────────────────────
    // ⛔ NOT NEGOTIABLE (ADR-0018): pruning a step that has an intent and no outcome destroys
    // the only trace of something that MAY have happened. ⚠️ This is the ONLY promise about
    // `prune` in this milestone — decision D7 leaves retention out, because the fingerprint
    // of a pruned payload needs a hash function and that would be a NEW ENTRY in the ADR-0031
    // list, which is a deliberate act nobody has measured.
    //
    // ✅ AND THE HALF THIS BLOCK COULD NOT TELL APART IS CLOSED, ON 2026-08-10, BY BLOCK 7b
    // BELOW. The paragraph that stood here is replaced rather than deleted because it was true
    // and it named its own remedy: the assertion asks that `prune` REFUSE, both implementations
    // refused EVERYTHING (decision D7), and so both satisfied this line WITHOUT ever consulting
    // whether the step was in doubt — the family of gotcha #30. Closing it needed the OTHER half
    // of the pair, a step NOT in doubt whose prune must be ACCEPTED, and that could not be
    // written while `prune` was unimplemented on both sides. Task 11 implemented it and 7b is the
    // half. `AlwaysInDoubtJournal` is the liar that proves 7b bites.
    //
    // ⚠️ AND THE ASSERTION IS AN EXACT ANSWER AND NOT `is_err()` ANY MORE, which is a second
    // thing this block did not hold: with `is_err()` one implementation could answer `Missing`
    // and the other `StepInDoubt` and NOTHING would go red — a divergence in silence, which is
    // the family of defect this whole suite exists for.
    //
    // ⛔ TWO ASSERTIONS AND ONE MESSAGE, because both ARE this promise, and the second was
    // MEASURED to be missing rather than added for symmetry: with the bare intent alone, an
    // implementation that read "closed" as "carries a second record of any sort" passed — and
    // both of them can be written that way, because neither `replay` nor `read_back` says which
    // operation wrote a record. Mutation `M12` filed a NOTE as an outcome inside `FileJournal`
    // and the ENTIRE workspace stayed green: a step in doubt became prunable, which is the one
    // thing this promise exists to forbid.
    {
        let mut journal = build();
        let step = StepId::new(4);
        journal
            .intent(step, b"in doubt from birth")
            .expect("intent");

        assert_eq!(
            journal.prune(step),
            Err(JournalError::StepInDoubt),
            "{}",
            PRUNE_IN_DOUBT_MESSAGE
        );

        // ⛔ AND A NOTE IS NOT AN OUTCOME. A note says what the step READ on the way; only an
        // outcome says what came of it, so a step whose intent has only a note for company has
        // still not been reconciled and is still not prunable.
        journal
            .note(step, b"and what it read on the way")
            .expect("a note upon an open step must succeed");

        assert_eq!(
            journal.prune(step),
            Err(JournalError::StepInDoubt),
            "{}",
            PRUNE_IN_DOUBT_MESSAGE
        );
    }

    // ── 7b. A step that is NOT in doubt CAN be pruned ─────────────────────────────────────
    // ⛔ THE DIRECTION ONE FORGETS (§7.1.1 rule 3), and here it is LOAD-BEARING and not tidy.
    // Promise 7 asks `prune` to REFUSE, and a `prune` that refuses EVERYTHING satisfies it
    // without ever consulting whether the step is in doubt — which is what BOTH implementations
    // did until this block existed. Gotcha #30.
    //
    // ⚠️ NUMBERED 7b AND NOT 9 ON PURPOSE: it is the second direction of ONE rule, not a rule of
    // its own. The suite therefore holds NINE promises across TEN blocks. ⚠️ The plan called it
    // "5b" and the number was stale — `prune` has been the SEVENTH promise since the guard on
    // `intent` took the sixth. Recounted on the source, gotcha #31.
    //
    // ⛔ AND WHAT THIS BLOCK DELIBERATELY DOES NOT ASSERT: what the archive looks like AFTER a
    // prune succeeds. ADR-0018 requires that "a payload that is absent and one that was never
    // recorded not be indistinguishable", and BOTH implementations fail that — MEASURED on
    // 2026-08-10 and not argued: a pruned step and one nobody ever wrote both answer
    // `Err(Missing)` to `read_back`, are both absent from `replay`, and answer `Err(Missing)`
    // alike to a second `prune`. Pinning any post-state here would freeze the wrong one. The
    // limit is declared where the defect is — beside `prune` in both implementations — and
    // carried as an OPEN ENTRY in `docs/porta-di-qualita.md`, because a note is read and
    // forgotten (gotcha #36). It belongs to the milestone that brings retention.
    //
    // ⚠️ AND A SECOND THING THIS SUITE DOES NOT PIN, declared rather than left to be found:
    // `prune`'s THIRD answer, `Missing` for a step nobody ever wrote. Promises 7 and 7b hold the
    // other two across both implementations; this one is held for the in-memory double alone, in
    // `crates/simulator/tests/memory_journal.rs`, so the two could diverge on it with nothing
    // going red — MEASURED, mutation `M10`, which survived the whole workspace. It is not a hole
    // this task opened, since both refused every prune with `Missing` before it, and closing it
    // costs a promise with a liar of its own that no measurement asks for yet.
    // ⛔ THREE ASSERTIONS AND ONE MESSAGE, because all three ARE this promise, and the second and
    // third were MEASURED to be missing rather than reasoned into place. With `is_ok()` alone,
    // TWO mutations of `prune` left the whole workspace green: one that answered `Ok(())` and
    // pruned NOTHING (`M5`, `M9` — both implementations), and one that answered `Ok(())` and
    // pruned THE WHOLE JOURNAL (`M6`). The first is the shape of gotcha #30 — an operation judged
    // by its return value; the second is the more expensive by far, because "prune step 5" was
    // free to destroy every other step in the archive. ⚠️ Neither is about WHAT a pruned step
    // leaves behind, which is why both can be held here without deciding it.
    {
        let mut journal = build();
        let step = StepId::new(5);
        let bystander = StepId::new(50);
        let kept: &[u8] = b"the record of another step entirely";

        journal.intent(bystander, kept).expect("intent");
        journal.intent(step, b"opened").expect("intent");
        journal.outcome(step, b"closed").expect("outcome");

        // (a) THE LIAR DIES HERE, and it is first because nothing else can be asked of a prune
        // that never happened.
        assert!(journal.prune(step).is_ok(), "{}", PRUNE_RECONCILED_MESSAGE);

        // (b) SOMETHING HAPPENED. ⛔ `assert_ne!` AGAINST THE ORIGINAL BYTES AND NOT `Missing`,
        // ON PURPOSE: what a pruned step reads back is exactly the question this milestone does
        // not answer, and pinning `Missing` here would freeze one answer to it. Every retention
        // form that ADR-0018 could choose — absent, empty, a fingerprint and a size — differs
        // from the payload, so this assertion survives the decision instead of pre-empting it.
        assert_ne!(
            journal.read_back(step).ok().as_deref(),
            Some(b"opened".as_slice()),
            "{}",
            PRUNE_RECONCILED_MESSAGE
        );

        // (c) AND ONLY THAT STEP. An irreversible operation with an unbounded blast radius is
        // worse than one nobody wrote.
        assert_eq!(
            journal.read_back(bystander).ok().as_deref(),
            Some(kept),
            "{}",
            PRUNE_RECONCILED_MESSAGE
        );
    }

    // ── 8. A note upon an open step is kept, and never displaces its intent ───────────────
    // ⛔ THE OPERATION ARRIVED ON 2026-08-10 BECAUSE A CALLER NEEDED IT — `Untrusted::promote`,
    // the first kernel code that writes a record at all — and the two operations already here
    // were both MEASURED to be wrong for it: as a second `intent` the port refuses, and with the
    // guard removed reconciliation replaces the caller's resolution with the note's; as an
    // `outcome` the caller's step leaves the doubt without having executed. The argument is
    // written out on `Journal::note`.
    //
    // ⛔ THIS IS ROAD A4's HALF OF ROAD A6. A journal that accepts a note and keeps nothing
    // makes the promotion succeed having recorded NOTHING — and what goes missing is precisely
    // the untrusted content and the label that says it was untrusted, which is the whole of what
    // task 7 added to the record. Promise 1 does not see it: it never writes a note.
    //
    // ⚠️ THREE ASSERTIONS AND ONE MESSAGE, because all three ARE this promise: the note attaches
    // only to something, it survives, and it does not take the intent's place. The ORDER is
    // chosen so the liar dies on the LAST of them — a suite that stopped at the first would
    // never exercise the other two against anything.
    {
        let mut journal = build();
        // (a) A note upon a step nobody opened has nothing to attach to.
        assert_eq!(
            journal.note(StepId::new(8), b"a note about nothing"),
            Err(JournalError::OutOfOrder),
            "{}",
            NOTE_MESSAGE
        );
    }
    {
        let mut journal = build();
        let step = StepId::new(6);
        let intent: &[u8] = b"what it set out to do";
        let note: &[u8] = b"and what it read on the way";

        journal.intent(step, intent).expect("intent must succeed");
        journal.note(step, note).expect(NOTE_MESSAGE);

        // (b) The intent still answers `read_back`. A store keyed on the step alone answers with
        // the note instead — the shape promise 2 catches for outcomes, one operation over.
        let read = journal.read_back(step).expect(NOTE_MESSAGE);
        assert_eq!(read.as_slice(), intent, "{}", NOTE_MESSAGE);

        // (c) And the note reached the archive. ⛔ THIS IS THE ASSERTION THE LIAR DIES ON, and it
        // is last on purpose: a journal that validated the note and then dropped it passes (a)
        // and (b) without a mark on it.
        let replayed = journal.replay().expect("replay must succeed");
        let records: Vec<(StepId, &[u8])> = replayed
            .iter()
            .map(|(step, bytes)| (*step, bytes.as_slice()))
            .collect();
        assert_eq!(
            records,
            vec![(step, intent), (step, note)],
            "{}",
            NOTE_MESSAGE
        );
    }
}

#[test]
fn the_in_memory_journal_honours_the_contract() {
    assert_journal_contract(simulator::journal::MemoryJournal::new);
}

#[test]
fn no_promise_message_is_a_substring_of_another() {
    // ⛔ THE CONSTRAINT THAT MAKES `contains` SAFE, AND UNTIL 2026-08-10 IT WAS ONLY DECLARED.
    // `assert_caught_on` matches the panic payload with `contains`, so if one message were a
    // substring of another, a liar caught on the WRONG promise would still satisfy the test that
    // names the right one — the suite would keep printing `ok` while pointing at the wrong
    // place. The rule was written in the doc of `READ_BACK_MESSAGE` and held by nothing; a
    // declared reason that no test holds is a comment.
    //
    // ⚠️ IT WAS RE-RUN RATHER THAN TRUSTED when `NOTE_MESSAGE` arrived, and again when
    // `PRUNE_RECONCILED_MESSAGE` did, which is the whole reason it is a test now: the constraint
    // is over the SET, so every message added has to be checked against ALL the others — NINE
    // messages are 72 ordered pairs, and re-reading nine strings by eye is exactly the check
    // nobody repeats. ⚠️ The two prune messages are the closest pair the set has ever held, both
    // opening "a step ", which is why the count is written down instead of the reassurance.
    let messages = [
        ("READ_BACK", READ_BACK_MESSAGE),
        ("READ_BACK_IS_THE_INTENT", READ_BACK_IS_THE_INTENT_MESSAGE),
        ("MISSING", MISSING_MESSAGE),
        ("REPLAY_ORDER", REPLAY_ORDER_MESSAGE),
        ("OUT_OF_ORDER", OUT_OF_ORDER_MESSAGE),
        ("SECOND_INTENT", SECOND_INTENT_MESSAGE),
        ("PRUNE_IN_DOUBT", PRUNE_IN_DOUBT_MESSAGE),
        ("PRUNE_RECONCILED", PRUNE_RECONCILED_MESSAGE),
        ("NOTE", NOTE_MESSAGE),
    ];

    for (name, message) in messages {
        for (other_name, other) in messages {
            if name == other_name {
                continue;
            }
            assert!(
                !other.contains(message),
                "{name} is a substring of {other_name}: a liar caught on {other_name} would \
                 satisfy the test that names {name}"
            );
        }
    }

    // The other direction, the one that gets forgotten (§7.1.1 rule 3): a bench where every
    // message were distinct BY BEING EMPTY would pass the loop above without saying anything.
    for (name, message) in messages {
        assert!(!message.is_empty(), "{name} is empty");
    }
}

// ⛔ THE DIRECTION ONE FORGETS (§7.1.1 rule 3): a suite never seen to fail is not a suite. The
// nine tests below break the port's promises ONE EACH, and demand that the suite notices each —
// and notices it ON THE RIGHT PROMISE, which is what reading the payload buys over `is_err()`.
//
// ⛔ NINE AND NOT THREE, and the six that were added are the lesson of gotcha #14. The suite
// dies at the FIRST promise a journal breaks, so a liar that violates promise 1 never reaches
// promise 5: with the three liars this task was dictated with, promises 2, 3 and 5 WERE NEVER
// SEEN TO FIRE. `SilentJournal` violates promise 5 as well — its `outcome` answers `Ok(())` —
// and dies on promise 1 long before getting there, which is exactly the shape of a control that
// looks covered and is not.
//
// ⚠️ THE COUNT WAS "SIX" AND THEN "EIGHT" DURING 2026-08-10, and is dated rather than quietly
// bumped: promise 6 and its liar arrived with the guard on `intent`, promise 8 and its liar with
// `note`, promise 7b and its liar with `prune`. A number inside a sentence that stays true is the
// exact shape of gotcha #31.
//
// ⚠️ AND EACH IS BROKEN IN A DIFFERENT WAY (gotcha #45): writes dropped, the wrong record
// returned, absence reported as emptiness, order reversed, the write-ahead guard removed on
// `outcome`, the guard removed on `intent`, retention granted, a write VALIDATED AND THEN
// DISCARDED, and — the ninth — retention REFUSED to everything, in the right words. Two liars
// broken the same way prove one thing twice and leave the other promise unguarded.

#[test]
fn a_journal_that_writes_nothing_is_caught() {
    // Road A6 of `crate::boundary`, as an executable case.
    assert_caught_on(SilentJournal::new, READ_BACK_MESSAGE, "promise 1");
}

#[test]
fn a_journal_that_answers_with_the_outcome_is_caught() {
    assert_caught_on(
        LastWriteWinsJournal::new,
        READ_BACK_IS_THE_INTENT_MESSAGE,
        "promise 2",
    );
}

#[test]
fn a_journal_that_answers_empty_instead_of_missing_is_caught() {
    assert_caught_on(
        EmptyInsteadOfMissingJournal::new,
        MISSING_MESSAGE,
        "promise 3",
    );
}

#[test]
fn a_journal_that_loses_the_write_order_is_caught() {
    assert_caught_on(ShuffledJournal::new, REPLAY_ORDER_MESSAGE, "promise 4");
}

#[test]
fn a_journal_that_accepts_an_outcome_with_no_intent_is_caught() {
    assert_caught_on(PermissiveJournal::new, OUT_OF_ORDER_MESSAGE, "promise 5");
}

#[test]
fn a_journal_that_accepts_a_second_intent_is_caught() {
    assert_caught_on(
        UnguardedIntentJournal::new,
        SECOND_INTENT_MESSAGE,
        "promise 6",
    );
}

#[test]
fn a_journal_that_prunes_a_step_in_doubt_is_caught() {
    assert_caught_on(EagerPruner::new, PRUNE_IN_DOUBT_MESSAGE, "promise 7");
}

#[test]
fn a_journal_that_calls_every_step_in_doubt_is_caught() {
    assert_caught_on(
        AlwaysInDoubtJournal::new,
        PRUNE_RECONCILED_MESSAGE,
        "promise 7b",
    );
}

#[test]
fn a_journal_that_validates_a_note_and_keeps_it_nowhere_is_caught() {
    assert_caught_on(DiscardedNoteJournal::new, NOTE_MESSAGE, "promise 8");
}

/// Runs the suite against a deliberately broken journal and demands that it be caught, and
/// caught ON THE NAMED PROMISE.
///
/// ⛔ IT READS THE PANIC PAYLOAD AND DOES NOT SETTLE FOR `is_err()`. A negative test that only
/// checked "something panicked" would claim to have caught the null write even when a DIFFERENT
/// assertion fired, and would keep saying `ok` the day the promise it names stops firing. That
/// is gotcha #15 — a true measurement, of another thing.
///
/// ⚠️ `contains` AND NOT EQUALITY, and it is not a weakening. `assert_eq!` with a custom message
/// panics with `assertion `left == right` failed: <message>` followed by the two values, so the
/// payload is never equal to the constant; only the one `assert!` in the suite would match
/// exactly. Equality would make five of these six tests fail for a reason that has nothing to do
/// with the journals. Substring matching is safe here because no message is a substring of
/// another — see the doc of `READ_BACK_MESSAGE`.
fn assert_caught_on<J, F>(build: F, expected: &str, promise: &str)
where
    J: Journal,
    F: Fn() -> J + std::panic::RefUnwindSafe,
{
    let message = message_the_suite_fails_with(build).unwrap_or_else(|| {
        panic!("THE SUITE IS VACUOUS ON {promise}: a journal that breaks it passed the suite")
    });
    assert!(
        message.contains(expected),
        "the suite did fire, but NOT on {promise} — so {promise} is still unproven.\n\
         expected to contain: {expected}\n\
         actual payload: {message}"
    );
}

/// Runs the suite and returns the message it failed with, or `None` if it passed.
///
/// ⚠️ The panic hook is silenced for the duration of the call: the panic is EXPECTED, and its
/// backtrace in the test output would train the reader to ignore backtraces. It is restored
/// immediately, so any LATER panic prints normally.
///
/// ⛔ DECLARED LIMIT, because the hook is PROCESS-WIDE and libtest runs tests on parallel
/// threads: a panic raised in ANOTHER test that happens to land inside this window is reported
/// as `FAILED` with no stdout section — the failure is never hidden, only its message. The
/// window is microseconds wide and the risk is negligible, which is why it is declared rather
/// than engineered away.
fn message_the_suite_fails_with<J, F>(build: F) -> Option<String>
where
    J: Journal,
    F: Fn() -> J + std::panic::RefUnwindSafe,
{
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let outcome = std::panic::catch_unwind(|| assert_journal_contract(&build));
    std::panic::set_hook(previous);

    match outcome {
        Ok(()) => None,
        Err(payload) => Some(panic_message(payload.as_ref())),
    }
}

/// The text of a panic, dug out of the payload. `assert!`/`assert_eq!` with a format argument
/// panic with a `String`; a `panic!("literal")` with no arguments carries a `&str` instead, and
/// both are handled so that this helper cannot report nothing for a message that is right there.
fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(text) = payload.downcast_ref::<String>() {
        text.clone()
    } else if let Some(text) = payload.downcast_ref::<&str>() {
        (*text).to_string()
    } else {
        String::from("<panic payload that is neither String nor &str>")
    }
}

/// Answers `Ok(())` and writes nothing. ⛔ This is road A6 of `crate::boundary` made
/// executable: the generic bound is satisfied, the promotion succeeds, and NOTHING WAS
/// RECORDED. Caught by promise 1.
struct SilentJournal;

impl SilentJournal {
    fn new() -> Self {
        SilentJournal
    }
}

impl Journal for SilentJournal {
    fn intent(&mut self, _step: StepId, _record: &[u8]) -> Result<(), JournalError> {
        Ok(())
    }
    fn outcome(&mut self, _step: StepId, _record: &[u8]) -> Result<(), JournalError> {
        Ok(())
    }
    fn note(&mut self, _step: StepId, _record: &[u8]) -> Result<(), JournalError> {
        Ok(())
    }
    fn read_back(&self, _step: StepId) -> Result<Vec<u8>, JournalError> {
        Err(JournalError::Missing)
    }
    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError> {
        Ok(Vec::new())
    }
    fn prune(&mut self, _step: StepId) -> Result<(), JournalError> {
        Err(JournalError::Missing)
    }
}

/// Records everything correctly and hands back THE LAST record of the step instead of the
/// first. ⛔ THIS IS THE SHAPE A KEY-VALUE STORE HAS BY NATURE — a `redb` table keyed on the
/// identity of the step answers with the last write, and the variant that keeps only one
/// record per key OVERWRITES the intent with the outcome. Both are the same defect and promise
/// 2 catches both.
///
/// It walks further into the suite than `SilentJournal` does, and that is its purpose: promise
/// 1 writes an intent and no outcome, and on such a step the first record and the last ARE THE
/// SAME RECORD, so this journal sails through it. It is the only journal here that reaches
/// promise 2's assertion.
struct LastWriteWinsJournal {
    inner: simulator::journal::MemoryJournal,
    /// Every record in write order, consulted from the back — which is the whole defect.
    written: Vec<(StepId, Vec<u8>)>,
}

impl LastWriteWinsJournal {
    fn new() -> Self {
        LastWriteWinsJournal {
            inner: simulator::journal::MemoryJournal::new(),
            written: Vec::new(),
        }
    }
}

impl Journal for LastWriteWinsJournal {
    fn intent(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.intent(step, record)?;
        self.written.push((step, record.to_vec()));
        Ok(())
    }
    fn outcome(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.outcome(step, record)?;
        self.written.push((step, record.to_vec()));
        Ok(())
    }
    fn note(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.note(step, record)?;
        self.written.push((step, record.to_vec()));
        Ok(())
    }
    fn read_back(&self, step: StepId) -> Result<Vec<u8>, JournalError> {
        // ⛔ THE DEFECT, and it is one `.rev()` wide: the LAST record of the step rather than
        // the first. A step that never got its outcome still reads back correctly.
        self.written
            .iter()
            .rev()
            .find(|(written_step, _)| *written_step == step)
            .map(|(_, bytes)| bytes.clone())
            .ok_or(JournalError::Missing)
    }
    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError> {
        self.inner.replay()
    }
    fn prune(&mut self, step: StepId) -> Result<(), JournalError> {
        self.inner.prune(step)
    }
}

/// Records everything correctly and reports the ABSENCE of a step as an EMPTY record.
/// ⚠️ Broken in a third way: it does not lose a write and it does not pick the wrong record —
/// it destroys the distinction between "never written" and "written empty", which is the
/// distinction reconciliation needs to tell a step it has never heard of from one whose
/// payload was pruned away. Caught by promise 3.
struct EmptyInsteadOfMissingJournal {
    inner: simulator::journal::MemoryJournal,
}

impl EmptyInsteadOfMissingJournal {
    fn new() -> Self {
        EmptyInsteadOfMissingJournal {
            inner: simulator::journal::MemoryJournal::new(),
        }
    }
}

impl Journal for EmptyInsteadOfMissingJournal {
    fn intent(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.intent(step, record)
    }
    fn outcome(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.outcome(step, record)
    }
    fn note(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.note(step, record)
    }
    fn read_back(&self, step: StepId) -> Result<Vec<u8>, JournalError> {
        // ⛔ THE DEFECT: `Missing` laundered into a successful read of nothing.
        match self.inner.read_back(step) {
            Err(JournalError::Missing) => Ok(Vec::new()),
            other => other,
        }
    }
    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError> {
        self.inner.replay()
    }
    fn prune(&mut self, step: StepId) -> Result<(), JournalError> {
        self.inner.prune(step)
    }
}

/// Writes everything and hands `replay` back in reverse. ⚠️ BROKEN IN A DIFFERENT WAY FROM THE
/// ones above ON PURPOSE — gotcha #45: two liars broken the same way prove one thing twice, and
/// the second promise stays unguarded. Caught by promise 4.
struct ShuffledJournal {
    inner: simulator::journal::MemoryJournal,
}

impl ShuffledJournal {
    fn new() -> Self {
        ShuffledJournal {
            inner: simulator::journal::MemoryJournal::new(),
        }
    }
}

impl Journal for ShuffledJournal {
    fn intent(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.intent(step, record)
    }
    fn outcome(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.outcome(step, record)
    }
    fn note(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.note(step, record)
    }
    fn read_back(&self, step: StepId) -> Result<Vec<u8>, JournalError> {
        self.inner.read_back(step)
    }
    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError> {
        let mut all = self.inner.replay()?;
        all.reverse();
        Ok(all)
    }
    fn prune(&mut self, step: StepId) -> Result<(), JournalError> {
        self.inner.prune(step)
    }
}

/// Accepts an `outcome` for a step it never saw an intent for, and reports success.
/// ⛔ THIS IS THE WRITE-AHEAD PROTOCOL WITH ITS GUARD REMOVED — the shape any journal has
/// before somebody decides that V6 is the PORT's to hold rather than the caller's. It is the
/// only journal here that reaches promise 5: every other liar either dies earlier or refuses
/// correctly. Caught by promise 5.
struct PermissiveJournal {
    inner: simulator::journal::MemoryJournal,
}

impl PermissiveJournal {
    fn new() -> Self {
        PermissiveJournal {
            inner: simulator::journal::MemoryJournal::new(),
        }
    }
}

impl Journal for PermissiveJournal {
    fn intent(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.intent(step, record)
    }
    fn outcome(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        // ⛔ THE DEFECT: the refusal is swallowed and the caller is told all is well. The
        // record is dropped, which is the honest consequence of a journal that has no place to
        // put an outcome whose intent it never saw — and it is precisely the state the
        // protocol exists to make impossible.
        match self.inner.outcome(step, record) {
            Err(JournalError::OutOfOrder) => Ok(()),
            other => other,
        }
    }
    fn note(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.note(step, record)
    }
    fn read_back(&self, step: StepId) -> Result<Vec<u8>, JournalError> {
        self.inner.read_back(step)
    }
    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError> {
        self.inner.replay()
    }
    fn prune(&mut self, step: StepId) -> Result<(), JournalError> {
        self.inner.prune(step)
    }
}

/// Keeps every record faithfully, in order, with the write-ahead guard on `outcome` — and NO
/// GUARD ON `intent`: a second intent for a step that already carries one is appended in
/// silence. ⛔ THIS IS NOT A LIE, WHICH IS WHY IT IS THE SHAPE THAT MATTERS. It is what a
/// journal looks like before somebody decides that one intent per step is the port's to hold,
/// and it is what `MemoryJournal` itself was until 2026-08-10.
///
/// ⚠️ AND IT DOES NOT LAUNDER A REFUSAL, unlike `PermissiveJournal`, which turns its inner
/// `Err(OutOfOrder)` into `Ok(())`. Two liars that both swallow a refusal would be one defect
/// written twice (gotcha #45); this one has its own store precisely so that the missing guard
/// is a MISSING GUARD and not a suppressed one. It is the only journal here that reaches
/// promise 6, and it passes all five before it on its own merits.
struct UnguardedIntentJournal {
    entries: Vec<UnguardedEntry>,
}

struct UnguardedEntry {
    step: StepId,
    is_intent: bool,
    bytes: Vec<u8>,
}

impl UnguardedIntentJournal {
    fn new() -> Self {
        UnguardedIntentJournal {
            entries: Vec::new(),
        }
    }

    fn has_intent(&self, step: StepId) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.step == step && entry.is_intent)
    }
}

impl Journal for UnguardedIntentJournal {
    fn intent(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        // ⛔ THE DEFECT, and it is an absence: no `if self.has_intent(step)` above this line.
        self.entries.push(UnguardedEntry {
            step,
            is_intent: true,
            bytes: record.to_vec(),
        });
        Ok(())
    }
    fn outcome(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        if !self.has_intent(step) {
            return Err(JournalError::OutOfOrder);
        }
        self.entries.push(UnguardedEntry {
            step,
            is_intent: false,
            bytes: record.to_vec(),
        });
        Ok(())
    }
    fn note(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        // ⚠️ Correct, and it never runs: this journal dies on promise 6, which is ahead of the
        // one that writes a note. It is written properly all the same — a liar broken in two
        // places proves nothing about either.
        if !self.has_intent(step) {
            return Err(JournalError::OutOfOrder);
        }
        self.entries.push(UnguardedEntry {
            step,
            is_intent: false,
            bytes: record.to_vec(),
        });
        Ok(())
    }
    fn read_back(&self, step: StepId) -> Result<Vec<u8>, JournalError> {
        self.entries
            .iter()
            .find(|entry| entry.step == step)
            .map(|entry| entry.bytes.clone())
            .ok_or(JournalError::Missing)
    }
    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError> {
        Ok(self
            .entries
            .iter()
            .map(|entry| (entry.step, entry.bytes.clone()))
            .collect())
    }
    fn prune(&mut self, _step: StepId) -> Result<(), JournalError> {
        Err(JournalError::Missing)
    }
}

/// Prunes anything it is asked to prune, including a step in doubt. Caught by promise 7.
struct EagerPruner {
    inner: simulator::journal::MemoryJournal,
}

impl EagerPruner {
    fn new() -> Self {
        EagerPruner {
            inner: simulator::journal::MemoryJournal::new(),
        }
    }
}

impl Journal for EagerPruner {
    fn intent(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.intent(step, record)
    }
    fn outcome(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.outcome(step, record)
    }
    fn note(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.note(step, record)
    }
    fn read_back(&self, step: StepId) -> Result<Vec<u8>, JournalError> {
        self.inner.read_back(step)
    }
    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError> {
        self.inner.replay()
    }
    fn prune(&mut self, _step: StepId) -> Result<(), JournalError> {
        Ok(())
    }
}

/// Refuses EVERY prune, and refuses it with the RIGHT WORD: `StepInDoubt` for a step that has
/// been reconciled exactly as for one that has not.
///
/// ⛔ BROKEN IN A NINTH WAY, and gotcha #45 is why the shape was chosen rather than reached for.
/// The eight above drop every write, hand back the wrong record, report absence as emptiness,
/// reverse the order, remove the guard on `outcome`, remove the guard on `intent`, grant
/// retention, and validate a write then discard it. This one REFUSES SOMETHING LEGITIMATE — the
/// only liar here whose defect is saying NO — and it says it with an answer that is CORRECT on
/// the case promise 7 tests. `EagerPruner` is its opposite and not its twin: that one destroys
/// evidence, this one never destroys anything and never asks the question.
///
/// ⛔ IT ANSWERS `StepInDoubt` AND NOT `Missing`, WHICH IS THE WHOLE POINT AND NOT A DETAIL. With
/// `Missing` it would die on promise 7, six lines earlier, and promise 7b would go on being
/// unproven while a test claimed otherwise — the shape of gotcha #14 rebuilt inside its own
/// remedy. To die on 7b it has to pass 7 ON ITS MERITS, and passing 7 on its merits means
/// giving the right refusal for the right case by accident.
///
/// ⚠️ AND IT IS NOT HYPOTHETICAL: it is what BOTH implementations were until 2026-08-10, save for
/// the word — they answered `Missing`. Promise 7 was satisfied by both without either consulting
/// anything, which is the open entry in `docs/porta-di-qualita.md` that promise 7b closes.
struct AlwaysInDoubtJournal {
    inner: simulator::journal::MemoryJournal,
}

impl AlwaysInDoubtJournal {
    fn new() -> Self {
        AlwaysInDoubtJournal {
            inner: simulator::journal::MemoryJournal::new(),
        }
    }
}

impl Journal for AlwaysInDoubtJournal {
    fn intent(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.intent(step, record)
    }
    fn outcome(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.outcome(step, record)
    }
    fn note(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.note(step, record)
    }
    fn read_back(&self, step: StepId) -> Result<Vec<u8>, JournalError> {
        self.inner.read_back(step)
    }
    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError> {
        self.inner.replay()
    }
    fn prune(&mut self, _step: StepId) -> Result<(), JournalError> {
        // ⛔ THE DEFECT, and it is a question never asked: the step is not consulted at all.
        Err(JournalError::StepInDoubt)
    }
}

/// Checks that a note has a step to attach to, answers `Ok(())`, and KEEPS NOTHING.
///
/// ⛔ BROKEN IN AN EIGHTH WAY, and gotcha #45 is why the shape was chosen rather than reached
/// for. The seven above drop every write, return the wrong record, report absence as emptiness,
/// reverse the order, remove the guard on `outcome`, remove the guard on `intent`, and grant
/// retention. This one VALIDATES AND THEN DISCARDS — it does the check, says yes, and stores
/// nothing. `SilentJournal` is the nearest neighbour and is not the same: it never checks
/// anything and dies on promise 1, six promises earlier.
///
/// ⚠️ AND IT IS THE SHAPE A REAL IMPLEMENTATION REALLY TAKES: `note` is the newest operation on
/// the port, so it is the one most likely to be stubbed `Ok(())` while the rest is written
/// properly. The consequence is exactly road A6 aimed at road A4 — the promotion succeeds, and
/// what silently fails to reach the archive is the untrusted content together with the label
/// that says it was untrusted.
struct DiscardedNoteJournal {
    inner: simulator::journal::MemoryJournal,
}

impl DiscardedNoteJournal {
    fn new() -> Self {
        DiscardedNoteJournal {
            inner: simulator::journal::MemoryJournal::new(),
        }
    }
}

impl Journal for DiscardedNoteJournal {
    fn intent(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.intent(step, record)
    }
    fn outcome(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.outcome(step, record)
    }
    fn note(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        // The guard is real — which is what lets this journal walk past assertion (a) of
        // promise 8 and die on (c). `read_back` answers `Missing` for a step with no records at
        // all, and the first record of a step is always its intent, so this asks the right
        // question through the surface the port offers.
        if self.inner.read_back(step).is_err() {
            return Err(JournalError::OutOfOrder);
        }
        // ⛔ THE DEFECT, and it is one line long: `record` goes nowhere.
        let _ = record;
        Ok(())
    }
    fn read_back(&self, step: StepId) -> Result<Vec<u8>, JournalError> {
        self.inner.read_back(step)
    }
    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError> {
        self.inner.replay()
    }
    fn prune(&mut self, step: StepId) -> Result<(), JournalError> {
        self.inner.prune(step)
    }
}
