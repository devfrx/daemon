//! The gateway decisor (§6.2): the chain, the two classes of constraint, and what each does when
//! the chain runs out. ⛔ NO MODEL IS CALLED HERE, and that is ADR-0020 in practice: the decisor
//! is verifiable with no provider in existence.

use kernel::gateway::{Candidate, Constraint, GatewayError, dispatch, resolve};
use kernel::ports::journal::{Journal, JournalError, StepId};
use kernel::record::{Detail, EffectClass, Record, RecordKind, RecordV1, Trust};
use simulator::journal::MemoryJournal;

/// Runs here, keeps nothing, and it is the cheapest of the three.
const LOCAL_CHEAP: Candidate = Candidate {
    model: "local-small",
    local: true,
    retains: false,
    price: 1,
};

/// ⛔ THE ONE EVERY DATA CONSTRAINT DISCARDS: it leaves this machine AND the provider keeps the
/// data, so it is the counter-example of `LocalOnly` and of `NoRetention` at once.
const REMOTE_DEAR: Candidate = Candidate {
    model: "remote-large",
    local: false,
    retains: true,
    price: 100,
};

/// ⚠️ THE THIRD EXISTS FOR ONE REASON: a chain of three lets `evaluated` be told apart from
/// "how many were walked". With a chain of one the two numbers are both 1 and no assertion can
/// separate them — errata `E59`.
const LOCAL_PRICEY: Candidate = Candidate {
    model: "local-medium",
    local: true,
    retains: false,
    price: 50,
};

/// Opens the step the routing is about to be written upon, and it is NOT setup noise.
///
/// ⛔ `Journal::note` says of itself that "a note for a step with NO INTENT is `OutOfOrder`. A
/// note is an annotation UPON something, and a step nobody opened is not something", and
/// `MemoryJournal` enforces it. A routing record is an annotation upon a step that EXISTS, and
/// the one who opened it is the caller — never the gateway, which is handed the `StepId` and
/// has no allocator for one. ⚠️ The dictated probe started from an empty journal and could not
/// pass: it is `E45` at its second occurrence, errata `E57`.
fn open_the_step(journal: &mut MemoryJournal, step: StepId) {
    let intent = Record::V1(RecordV1::intent(
        EffectClass::Idempotent,
        Trust::Instruction,
        Vec::new(),
        "the step whose routing is resolved",
    ))
    .encode();
    journal.intent(step, &intent).expect("intent");
}

#[test]
fn a_conforming_candidate_is_chosen_and_nothing_is_degraded() {
    // ⛔ THE COUNTER-PROBE OF CATALOGUE ROW `Q13` — "filtered -> it compiles". It lives here and
    // not beside `dispatching_an_unfiltered_candidate.rs`: gotcha #49.
    let chain = [LOCAL_CHEAP, REMOTE_DEAR];
    let resolved = resolve(&chain, &[Constraint::LocalOnly]).expect("resolve");
    assert!(!resolved.was_degraded());
}

#[test]
#[allow(non_snake_case)]
fn a_data_constraint_with_no_candidate_FAILS_CLOSED() {
    // ⛔ ADR-0012: constraints on DATA AND CONFIDENTIALITY fail closed at chain exhaustion.
    // There is no degraded road here, and the absence of one is the assertion.
    //
    // ⚠️ `matches!` AND NOT `assert_eq!`, and it is not style: `Conforming` derives `Debug` and
    // nothing else, so comparing a `Result<Conforming, _>` does not compile. Deriving
    // `PartialEq` for the bench alone is the road the compiler itself suggests and the one this
    // repository has already refused for `Admission` — a derive that exists for no caller is the
    // same item the "no caller, no item" rule keeps off `Conforming` itself. Errata `E58`.
    let chain = [REMOTE_DEAR];
    let refused = resolve(&chain, &[Constraint::LocalOnly]);
    assert!(
        matches!(refused, Err(GatewayError::NoConformingCandidate)),
        "a data constraint with no candidate did not fail closed"
    );
}

#[test]
#[allow(non_snake_case)]
fn a_quality_constraint_with_no_candidate_DEGRADES_AND_SAYS_SO() {
    // ⛔ The other class: quality and cost proceed, DECLARING it. The two directions of the same
    // row, and they must not be the same test -- a single one could not tell them apart.
    let chain = [REMOTE_DEAR];
    let resolved = resolve(&chain, &[Constraint::PriceCeiling(10)]).expect("resolve");
    assert!(resolved.was_degraded());
}

#[test]
fn an_empty_chain_fails_closed_too() {
    let refused = resolve(&[], &[]);
    assert!(
        matches!(refused, Err(GatewayError::NoConformingCandidate)),
        "an empty chain found something to conform"
    );
}

#[test]
fn a_retaining_candidate_is_discarded_by_no_retention() {
    // ⛔ THE EXPENSIVE DIRECTION OF `NoRetention`, and it was missing: no probe named the
    // constraint at all, so the arm `!candidate.retains` was a live mutant on the whole
    // workspace — and removing the negation is the mutation that admits exactly the providers
    // that KEEP the data. Its class is `Data`, so the answer here is the closed failure of
    // ADR-0012 and never a degraded road. Errata `E61`.
    let chain = [REMOTE_DEAR];
    let refused = resolve(&chain, &[Constraint::NoRetention]);
    assert!(
        matches!(refused, Err(GatewayError::NoConformingCandidate)),
        "a provider that keeps the data satisfied `NoRetention`"
    );
}

#[test]
fn a_candidate_that_keeps_nothing_satisfies_no_retention() {
    // ⛔ THE OTHER DIRECTION, AND IT IS A SECOND `#[test]` AND NOT A SECOND ASSERTION: a single
    // probe stops at the first failing assertion, so the second half would never be exercised on
    // the very mutation both halves exist to kill (gotcha #14). Two separate probes make that
    // mutation kill TWO, which is what says the pair is not one check written twice.
    let chain = [LOCAL_CHEAP];
    let resolved = resolve(&chain, &[Constraint::NoRetention]).expect("resolve");
    assert!(!resolved.was_degraded());
}

#[test]
#[allow(non_snake_case)]
fn the_dispatch_journals_the_RESOLVED_decision_and_not_a_reference_to_it() {
    // ⛔ ADR-0011: the record holds the RESOLVED decision, "not a reference to the configuration
    // -- re-reading today's configuration does not say what happened yesterday". So the model
    // NAME is in the record, and never an index into the chain.
    //
    // ⛔ THE CHAIN IS THREE LONG, deliberately: `evaluated` is how many the chain OFFERED, and
    // with a chain of one that is indistinguishable from how many the filter WALKED. Here the
    // road TAKEN walks one and the chain offers three, so the assertion below rules out "how
    // many the SECOND pass walked". Errata `E59`.
    //
    // ⚠ AND IT DOES NOT RULE OUT THE THIRD READING, which is said rather than left to be
    // discovered: the FIRST pass finds nothing here and therefore EXHAUSTS the chain, so "how
    // many the FIRST pass walked" is also three. Measured -- rewriting `evaluated` to that count
    // left the whole workspace green. The reading that closes it is the probe below, where the
    // first pass stops at index 0. Errata `E100`.
    let mut journal = MemoryJournal::new();
    let step = StepId::new(1);
    open_the_step(&mut journal, step);

    // ⚠️ A CEILING OF ZERO IS "ONLY WHAT COSTS NOTHING", and none of the three is free: the
    // quality class therefore has no candidate at all, which is what makes the second road the
    // one taken and `degraded` true.
    let chain = [LOCAL_PRICEY, REMOTE_DEAR, LOCAL_CHEAP];
    let resolved = resolve(
        &chain,
        &[Constraint::LocalOnly, Constraint::PriceCeiling(0)],
    )
    .expect("resolve");

    dispatch(resolved, step, &mut journal).expect("dispatch");

    let entries = journal.replay().expect("replay");
    // ⚠️ TWO AND NOT ONE, and the second is the routing: the first is the intent that opened the
    // step, which the write-ahead discipline demands before any note upon it.
    assert_eq!(entries.len(), 2);
    let (at, bytes) = &entries[1];
    assert_eq!(*at, step);
    let Record::V1(body) = Record::decode(bytes).expect("decode");
    assert_eq!(body.kind(), RecordKind::Routing);
    // ⛔ THE FIELDS NOTHING HELD, AND `trust` IS THE ONE THAT MATTERS: it is the `I6`
    // label of ADR-0014 on a DURABLE record. Until this line a `dispatch` writing `Untrusted`
    // left the WHOLE workspace green -- measured before writing it, `42 targets, 307 passed`,
    // identical to the baseline. `reason` and `effect` were the same shape. Errata `E97`.
    //
    // ⚠ RICHIAMO DEL 2026-09-01: this said "THE THREE FIELDS" and "`dispatch` writes six fields
    // and only four were read back", and the arithmetic did not close with ITSELF -- six minus
    // four is two, not three. The counts mixed units: `four` counted ASSERTIONS and `six` counted
    // FIELDS. The field that fell outside both was `payload`, and it was still a live mutant a
    // day later. The numerals are GONE rather than realigned; what holds is the rule below --
    // every field this record carries is read back. Errata `E116`.
    //
    // ⚠ `effect` IS PINNED THOUGH `reconcile` NEVER READS IT, and that is deliberate: the
    // doc beside the call AFFIRMS the value is true of this record, and an affirmation nobody
    // holds is what the four fields above already were. It is the Task 10 precedent -- pinned,
    // not declared, because a doc asserts it.
    assert_eq!(body.trust(), Trust::Instruction);
    assert_eq!(body.effect(), EffectClass::Idempotent);
    assert_eq!(
        body.reason(),
        "the gateway resolved the routing for this step"
    );
    // ⛔ AND THE `payload`, WHICH IS THE PREMISE OF THE `trust` LINE ABOVE RATHER THAN ONE MORE
    // FIELD. `Trust::Instruction` is true of this record ONLY BECAUSE no external byte is in it,
    // so an unread payload leaves the `I6` label resting on nothing. Measured on 2026-09-01:
    // three bytes put into the payload left the whole workspace green, `43 targets, 324 passed`,
    // identical to the baseline. The wording is the sister bench's, `permission_triple.rs`, where
    // the same class was closed first. Errata `E116`.
    assert!(
        body.payload().is_empty(),
        "the payload carries {} bytes, and `dispatch` affirms that no external byte enters this record",
        body.payload().len()
    );
    let Some(Detail::Routing(routing)) = body.detail() else {
        panic!("the routing record carries no structured detail");
    };
    assert_eq!(routing.model(), "local-medium");
    assert_eq!(routing.evaluated(), 3);
    assert!(routing.degraded());
}

#[test]
fn dispatch_does_not_swallow_the_journal_saying_no() {
    // ⛔ THE ERROR ROAD, REACHABLE BY NOTHING UNTIL THIS PROBE. `dispatch` returns
    // `Result<(), JournalError>` and no probe ever looked at the `Err`. Measured before writing
    // it: turning the last line into `let _ = journal.note(..); Ok(())` left the whole workspace
    // green -- `42 targets, 307 passed`, identical to the baseline. A `dispatch` that reports
    // success while the routing record never reached the journal is the silent loss ADR-0007
    // exists to forbid. Errata `E98`.
    //
    // ⚠ THE STEP IS NOT OPENED ON PURPOSE, which is the one thing `open_the_step` exists to
    // avoid everywhere else in this bench: `Journal::note` refuses a note upon a step with no
    // intent, and that refusal is the promise the doc of `dispatch` makes out loud.
    let mut journal = MemoryJournal::new();
    let step = StepId::new(1);

    let chain = [LOCAL_CHEAP];
    let resolved = resolve(&chain, &[Constraint::LocalOnly]).expect("resolve");

    assert_eq!(
        dispatch(resolved, step, &mut journal),
        Err(JournalError::OutOfOrder)
    );
}

#[test]
fn evaluated_is_what_the_chain_OFFERED_and_not_what_the_first_pass_walked() {
    // ⛔ THE THIRD READING OF "evaluated", AND THIS IS THE ONLY PROBE THAT CLOSES IT. Its
    // sister above cannot: there the first pass finds nothing and walks all three, so "offered"
    // and "first pass walked" are the same number and no assertion can separate them.
    //
    // ⛔ HERE THE FIRST PASS STOPS AT INDEX 0 -- no quality constraint exists, so `preferred`
    // is vacuously true and `LOCAL_CHEAP` is admissible -- so it walks ONE while the chain offers
    // THREE. `evaluated == 3` is therefore false of "walked" under both readings at once, which
    // is what makes the number mean the chain and not the traversal. Errata `E100`.
    let mut journal = MemoryJournal::new();
    let step = StepId::new(1);
    open_the_step(&mut journal, step);

    let chain = [LOCAL_CHEAP, REMOTE_DEAR, LOCAL_PRICEY];
    let resolved = resolve(&chain, &[Constraint::LocalOnly]).expect("resolve");
    assert!(!resolved.was_degraded());

    dispatch(resolved, step, &mut journal).expect("dispatch");

    let entries = journal.replay().expect("replay");
    let (_, bytes) = &entries[1];
    let Record::V1(body) = Record::decode(bytes).expect("decode");
    let Some(Detail::Routing(routing)) = body.detail() else {
        panic!("the routing record carries no structured detail");
    };
    // ⚠ THE MODEL IS PINNED TOO, and not as decoration: without it a `resolve` that walked
    // the chain backwards would still offer three and the probe would say nothing about WHICH
    // one stopped the first pass.
    assert_eq!(routing.model(), "local-small");
    assert_eq!(routing.evaluated(), 3);
    assert!(!routing.degraded());
}
