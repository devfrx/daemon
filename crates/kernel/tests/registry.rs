//! The registry of the program's functions (ADR-0038), exercised FROM OUTSIDE THE CRATE.
//!
//! ⛔ THE FUNCTIONS REGISTERED HERE ARE OF NO CONSEQUENCE, and that is the point rather than a
//! shortcut: this bench proves the MECHANISM. The one real function — the VRAM policy change —
//! is registered by the dispatch, and its probe lives with it.

use kernel::permission::{self, Operation, Permission};
use kernel::ports::ipc::ClientId;
use kernel::ports::journal::{Journal, JournalError, StepId};
use kernel::record::{Detail, EffectClass, Record, RecordKind};
use kernel::registry::{Approval, Function, InvokeError, Invoker, Registry};
use simulator::journal::MemoryJournal;

const GUARDED: Permission = Permission {
    tool: "registry",
    resource: "arbiter",
    operation: Operation::Write,
};

fn a_function() -> Function {
    Function {
        name: "set-policy",
        permission: GUARDED,
        effect: EffectClass::Idempotent,
    }
}

fn a_registry() -> Registry {
    let mut registry = Registry::new();
    registry.register(a_function());
    registry
}

#[test]
fn a_name_that_is_not_registered_is_refused_and_writes_nothing() {
    // §5 of the sub-project 2 design, in its own words: "un nome non registrato → rifiutato, nessun
    // record". ⛔ AND THE SECOND HALF IS THE ONE THAT MATTERS: a peer that could make the core
    // write a record by naming anything would own the durable archive.
    let mut journal = MemoryJournal::new();
    let registry = a_registry();

    let outcome = registry.invoke(
        &mut journal,
        StepId::new(1),
        "set-polizy",
        Invoker::Gui(ClientId::new(1)),
        b"local",
        Approval::Checked,
        |_| -> Result<(), JournalError> { panic!("the effect must not run") },
    );

    assert_eq!(outcome, Err(InvokeError::NotRegistered));
    assert!(
        journal.replay().expect("replay").is_empty(),
        "a name nobody registered must leave the archive exactly as it was"
    );
}

#[test]
fn a_triple_that_is_not_granted_never_reaches_the_effect() {
    // The first of the two directions §5 asks for. ⛔ THE EFFECT PANICS, so "never reached" is
    // held by the test failing loudly rather than by an assertion that could be forgotten.
    let mut journal = MemoryJournal::new();
    let registry = a_registry();

    let outcome = registry.invoke(
        &mut journal,
        StepId::new(1),
        "set-policy",
        Invoker::Gui(ClientId::new(1)),
        b"local",
        Approval::Checked,
        |_| -> Result<(), JournalError> { panic!("the effect must not run without the triple") },
    );

    assert_eq!(outcome, Err(InvokeError::PermissionRequired(GUARDED)));
    assert!(
        journal.replay().expect("replay").is_empty(),
        "a refused invocation must leave the archive exactly as it was"
    );
}

#[test]
fn a_granted_triple_reaches_the_effect_and_the_step_closes() {
    // The direction one forgets (§7.1.1 rule 3). Without it, an `invoke` that refused ALWAYS
    // would satisfy the two probes above.
    let mut journal = MemoryJournal::new();
    let registry = a_registry();

    // The grant is a note upon a step of its own, which is what `permission::grant` promises.
    journal
        .intent(StepId::new(1), &a_bare_intent())
        .expect("the granting step");
    permission::grant(&mut journal, StepId::new(1), &GUARDED).expect("grant");
    journal
        .outcome(StepId::new(1), &a_bare_outcome())
        .expect("the granting step closes");

    let produced = registry
        .invoke(
            &mut journal,
            StepId::new(2),
            "set-policy",
            Invoker::Gui(ClientId::new(7)),
            b"local",
            Approval::Checked,
            |journal| {
                // The effect writes its OWN step B, which is what `set_policy` really does.
                journal.intent(StepId::new(3), &a_bare_intent())?;
                journal.outcome(StepId::new(3), &a_bare_outcome())?;
                Ok(42u32)
            },
        )
        .expect("a granted invocation must run");

    assert_eq!(produced, 42, "invoke must hand back what the effect produced");

    // ⛔ AND THE SECOND HALF OF THIS PROBE'S NAME, WHICH NOTHING HELD UNTIL NOW: "the step
    // closes". `produced == 42` says the effect RAN and says nothing at all about point 7 of the
    // sequence the doc of `invoke` lists -- an `invoke` that forgot the `outcome` on step A would
    // hand back 42 just the same and leave A IN DOUBT FOR EVER, which is the one failure ADR-0007
    // exists to prevent, with this probe green over it. Measured in both directions on
    // 2026-09-18: with that `outcome` removed this assertion goes red naming step 2.
    //
    // ⚠️ AND THE ORACLE IS THE PROJECTION RATHER THAN A LIST OF KINDS, because it covers A AND B
    // AT ONCE: the effect opens and closes ITS OWN step, and a step of the effect left open is
    // the same defect one level down. The sibling probe asserts the ORDER of the species on A;
    // this one asserts that nothing anywhere is left owing an outcome.
    let in_doubt = kernel::reconcile::steps_in_doubt(&journal).expect("the projection must answer");
    assert!(
        in_doubt.is_empty(),
        "a finished invocation must leave no step in doubt: neither A nor the effect's own B. \
         These are still owing an outcome: {in_doubt:?}"
    );
}

#[test]
fn the_note_carries_the_registered_name_the_invoker_and_the_argument() {
    // §5: "una sonda legge il dettaglio dopo `replay`". ⛔ AND IT READS ALL THREE, because each
    // is a different road: the name proves the REGISTERED one reached the record and not the
    // arriving text, the code proves `Invoker::code` was consulted, and the payload proves the
    // argument did NOT end up in the structured half.
    let mut journal = MemoryJournal::new();
    let registry = a_registry();

    journal
        .intent(StepId::new(1), &a_bare_intent())
        .expect("the granting step");
    permission::grant(&mut journal, StepId::new(1), &GUARDED).expect("grant");
    journal
        .outcome(StepId::new(1), &a_bare_outcome())
        .expect("the granting step closes");

    registry
        .invoke(
            &mut journal,
            StepId::new(2),
            "set-policy",
            Invoker::Gui(ClientId::new(7)),
            b"local",
            Approval::Checked,
            |_| -> Result<(), JournalError> { Ok(()) },
        )
        .expect("a granted invocation must run");

    let mut seen = None;
    for (_, bytes) in journal.replay().expect("replay") {
        let Record::V1(body) = Record::decode(&bytes).expect("every record must decode");
        if body.kind() != RecordKind::Invocation {
            continue;
        }
        let Some(Detail::Invocation(detail)) = body.detail() else {
            panic!("an invocation record must carry an invocation detail");
        };
        seen = Some((
            detail.function().to_string(),
            detail.invoker(),
            body.payload().to_vec(),
        ));
    }

    let (function, invoker, payload) = seen.expect("the invocation note must be in the archive");
    assert_eq!(function, "set-policy");
    assert_eq!(invoker, 0, "the gui's code, from `Invoker::code`");
    assert_eq!(payload, b"local".to_vec(), "the argument travels in the payload");
}

#[test]
fn an_approval_just_given_skips_the_question_and_writes_the_grant_on_step_a() {
    // ⛔ THE OTHER ROAD OF SEQUENCE 3, AND `a_triple_that_is_not_granted_never_reaches_the_effect`
    // IS ITS MIRROR: the SAME journal, with the SAME ungranted triple, and the only difference
    // is the word `JustGiven`. Without this probe `Approval` would be an enum whose second
    // variant nothing ever takes -- and the round the gui really walks would be held only by
    // another task's bench.
    let mut journal = MemoryJournal::new();
    let registry = a_registry();

    let produced = registry
        .invoke(
            &mut journal,
            StepId::new(2),
            "set-policy",
            Invoker::Gui(ClientId::new(1)),
            b"local",
            Approval::JustGiven,
            |_| -> Result<(), JournalError> { Ok(()) },
        )
        .expect("an approval just given must not be asked about");

    assert_eq!(produced, ());

    let kinds: Vec<RecordKind> = journal
        .replay()
        .expect("replay")
        .iter()
        .map(|(_, bytes)| match Record::decode(bytes).expect("every record must decode") {
            Record::V1(body) => body.kind(),
        })
        .collect();
    // ⛔ THE ORDER IS THE ASSERTION: the grant sits BETWEEN the invocation note and the
    // outcome, which is decision 21 of the north star. A grant written before the intent would
    // be a note on a step nobody opened, which `Journal::note` refuses outright; one written
    // after the effect would leave the effect running on a permission not yet recorded.
    assert_eq!(
        kinds,
        vec![
            RecordKind::Intent,
            RecordKind::Invocation,
            RecordKind::Permission,
            RecordKind::Outcome,
        ],
        "the grant goes on step A, between the note and the outcome"
    );

    // ⛔ AND THE SECOND DIRECTION, without which the probe above would pass over a registry
    // that simply stopped checking: the triple is now REALLY granted, so a later `Checked`
    // invocation goes through where the mirror probe refuses it.
    assert!(
        permission::is_granted(&journal, &GUARDED).expect("is_granted answers"),
        "the grant that was written must be the one `is_granted` reads back"
    );
}

#[test]
fn registering_the_same_name_twice_replaces_rather_than_piling_up() {
    // ⛔ THE PROPERTY THAT KEEPS A PERMISSION MEANING ONE THING. With two entries under one name,
    // which triple protects the function would depend on the order of a search.
    let mut registry = Registry::new();
    registry.register(a_function());
    registry.register(Function {
        effect: EffectClass::Unrepeatable,
        ..a_function()
    });

    let mut journal = MemoryJournal::new();
    journal
        .intent(StepId::new(1), &a_bare_intent())
        .expect("the granting step");
    permission::grant(&mut journal, StepId::new(1), &GUARDED).expect("grant");
    journal
        .outcome(StepId::new(1), &a_bare_outcome())
        .expect("the granting step closes");

    registry
        .invoke(
            &mut journal,
            StepId::new(2),
            "set-policy",
            Invoker::Gui(ClientId::new(1)),
            b"",
            Approval::Checked,
            |_| -> Result<(), JournalError> { Ok(()) },
        )
        .expect("the second registration must be the one that answers");

    // The SECOND registration's class is what reached the archive — which is the observable
    // difference between replacing and piling up.
    let classes: Vec<EffectClass> = journal
        .replay()
        .expect("replay")
        .into_iter()
        .filter_map(|(_, bytes)| {
            let Record::V1(body) = Record::decode(&bytes).ok()?;
            (body.kind() == RecordKind::Invocation).then(|| body.effect())
        })
        .collect();

    assert_eq!(classes, vec![EffectClass::Unrepeatable]);
}

fn a_bare_intent() -> Vec<u8> {
    Record::V1(kernel::record::RecordV1::intent(
        EffectClass::Idempotent,
        kernel::record::Trust::Instruction,
        Vec::new(),
        "a step of no consequence",
    ))
    .encode()
}

fn a_bare_outcome() -> Vec<u8> {
    Record::V1(kernel::record::RecordV1::outcome(
        EffectClass::Idempotent,
        kernel::record::Trust::Instruction,
        Vec::new(),
        "and it closed",
    ))
    .encode()
}
