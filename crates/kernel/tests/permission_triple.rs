//! The permission is a TRIPLE, and "which permissions are active now" is a PROJECTION of the
//! journal (§6.6, ADR-0016, `V21`).
//!
//! ⛔ EVERY PROBE HERE OPENS THE GRANTED STEP FIRST, AND THAT IS NOT SETUP NOISE — it is the
//! write-ahead discipline of ADR-0007 showing through. `Journal::note` says of itself that "a note
//! for a step with NO INTENT is `OutOfOrder`. A note is an annotation UPON something, and a step
//! nobody opened is not something", and `MemoryJournal` enforces it. `permission::grant` writes a
//! note, so the step it annotates has to exist, and the one who opened it is the CALLER — never
//! `grant`. It is the same asymmetry `tests/sensor_ring.rs` declares for the step the ring judges,
//! and it is written here because the dictated probes started from an empty journal and could not
//! pass: measured, `Err(OutOfOrder)` on all five.
//!
//! ⛔ AND THE THREE COMPONENTS OF THE TRIPLE GET THREE SEPARATE PROBES, one changing ONE component
//! each. A single probe changing two at once cannot say which conjunct of `is_granted` did the
//! work — drop the `tool` comparison and a probe that also moved the resource stays green, which is
//! the shape that lets a widened permission through in silence.

use kernel::permission::{Operation, Permission, PermissionError, grant, is_granted};
use kernel::ports::journal::{Journal, JournalError, StepId};
use kernel::record::{
    Detail, EffectClass, Record, RecordError, RecordKind, RecordV1, RoutingDetail, Trust,
};
use simulator::journal::MemoryJournal;

/// The triple every probe grants, and the one the three "not covered" probes vary ONE component
/// of.
const READ_A: Permission = Permission {
    tool: "file",
    resource: "/a",
    operation: Operation::Read,
};

/// Opens the step the permission is granted upon, and answers nothing.
///
/// ⛔ `grant` DOES NOT DO THIS, and the asymmetry is the point: `StepId` has no allocator — whether
/// one arrives is the owner's decision — so a function that minted the intent of a step it did not
/// open would be inventing an identity the port is meant to assign.
fn open_the_step(journal: &mut MemoryJournal, step: StepId) {
    let intent = Record::V1(RecordV1::intent(
        EffectClass::Idempotent,
        Trust::Instruction,
        Vec::new(),
        "the step a permission is granted upon",
    ))
    .encode();
    journal.intent(step, &intent).expect("intent");
}

/// A journal open enough to receive an intent and REFUSING the note.
///
/// ⛔ IT IS NOT THE ALL-REFUSING FAKE OF `tests/boundary_promotion.rs`, and the narrowing is what
/// makes the probe say something. A journal that refused the intent too would leave the step
/// unopened, and `grant`'s failure could then be `OutOfOrder` from the missing intent rather than
/// the refusal being tested — the probe would be green for the wrong reason. Here the step IS
/// open, so the only thing that can fail is the write `grant` performs.
///
/// ⚠️ `NotDurable` IS A STATE A REAL JOURNAL REACHES when the disk is full, which is what keeps
/// this a fixture rather than a liar; and it would not pass the conformance suite, for the reason
/// `RefusingJournal` states there.
struct NoteRefusingJournal {
    opened: bool,
}

impl Journal for NoteRefusingJournal {
    fn intent(&mut self, _step: StepId, _record: &[u8]) -> Result<(), JournalError> {
        self.opened = true;
        Ok(())
    }
    fn outcome(&mut self, _s: StepId, _r: &[u8]) -> Result<(), JournalError> {
        Err(JournalError::NotDurable)
    }
    fn note(&mut self, _s: StepId, _r: &[u8]) -> Result<(), JournalError> {
        Err(JournalError::NotDurable)
    }
    fn read_back(&self, _s: StepId) -> Result<Vec<u8>, JournalError> {
        Err(JournalError::Missing)
    }
    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError> {
        // Nothing is ever recorded here — the note refuses and the intent is dropped — so an
        // empty journal is not a shortcut, it is the truth about this fake.
        Ok(Vec::new())
    }
    fn prune(&mut self, _s: StepId) -> Result<(), JournalError> {
        Ok(())
    }
}

/// A journal whose `replay` refuses outright, for the OTHER road into `PermissionError::Journal`.
struct ReplayRefusingJournal;

impl Journal for ReplayRefusingJournal {
    fn intent(&mut self, _step: StepId, _record: &[u8]) -> Result<(), JournalError> {
        Ok(())
    }
    fn outcome(&mut self, _s: StepId, _r: &[u8]) -> Result<(), JournalError> {
        Ok(())
    }
    fn note(&mut self, _s: StepId, _r: &[u8]) -> Result<(), JournalError> {
        Ok(())
    }
    fn read_back(&self, _s: StepId) -> Result<Vec<u8>, JournalError> {
        Err(JournalError::Missing)
    }
    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError> {
        Err(JournalError::NotDurable)
    }
    fn prune(&mut self, _s: StepId) -> Result<(), JournalError> {
        Ok(())
    }
}

#[test]
fn a_granted_triple_is_granted() {
    // The direction without which every "not covered" probe below could be satisfied by a
    // projection that answers `false` to everything.
    let mut journal = MemoryJournal::new();
    let step = StepId::new(1);
    open_the_step(&mut journal, step);

    grant(&mut journal, step, &READ_A).expect("grant");

    assert!(is_granted(&journal, &READ_A).expect("is_granted"));
}

#[test]
fn grant_writes_every_field_it_says_it_writes() {
    // ⛔ `grant` WRITES SIX FIELDS AND `is_granted` READS TWO, so four of them were held by
    // NOTHING and a fifth road was walked by nobody. Every probe above reaches the record only
    // through `is_granted`, which looks at `kind()` and `detail()`. Measured on 2026-09-01, one
    // mutation at a time with the whole workspace run after each: `trust`
    // `Instruction → Untrusted`, `effect` `Unrepeatable → Idempotent`, `reason` → `""`, an
    // `alloc::vec![0xAA]` in place of the empty `payload`, and the `write` flag flattened to
    // `false` — five mutants, five times `43 targets, 321 passed, 0 failed, 2 ignored`, identical
    // to the baseline figure for figure.
    //
    // ⛔ IT IS ERRATA `E97` ON THE SISTER FUNCTION, not a species of its own: the commit before
    // this one closed exactly this on `gateway::dispatch`, and the shape is reused from
    // `tests/gateway_decisor.rs` rather than invented. An affirmation nobody holds is what those
    // fields already were.
    //
    // ⛔ AND THE DOCS DO AFFIRM THEM, which is what makes them claims and not decoration: `grant`
    // says of `trust` that "the label is TRUE rather than decorative", of `effect` that it is
    // "filled with what is TRUE of this record", and `tests/reconciliation.rs` says of its own
    // helper that the class "is what `grant` writes" — an affirmation ABOUT `grant` that lives in
    // another file and that nothing tied back to `grant`.
    //
    // ⚠️ THE TRIPLE GRANTED HERE IS A `Write`, AND THAT IS THE ONE THING THAT CANNOT BE BORROWED
    // FROM `READ_A`: every other call to `grant` in the workspace passes a `Read`, so the road
    // `Operation::Write → write: true` was walked by nothing at all and flattening the flag to
    // `false` cost nothing. ⛔ THE OTHER DIRECTION IS HELD BY
    // `a_different_OPERATION_is_not_covered` — measured, flattening the same flag to `true` turns
    // it red — so between the two probes one line is held both ways (§7.1.1, rule 3).
    let mut journal = MemoryJournal::new();
    let step = StepId::new(1);
    open_the_step(&mut journal, step);

    let write_a = Permission {
        operation: Operation::Write,
        ..READ_A
    };
    grant(&mut journal, step, &write_a).expect("grant");

    let entries = journal.replay().expect("replay");
    // ⚠️ TWO AND NOT ONE, and the grant is the second: the first is the intent that opened the
    // step, which the write-ahead discipline demands before any note upon it.
    assert_eq!(entries.len(), 2);
    let (at, bytes) = &entries[1];
    assert_eq!(*at, step);

    let Record::V1(body) = Record::decode(bytes).expect("decode");
    assert_eq!(body.kind(), RecordKind::Permission);
    assert_eq!(body.trust(), Trust::Instruction);
    assert_eq!(body.effect(), EffectClass::Unrepeatable);
    assert_eq!(body.reason(), "a permission was granted for this triple");
    assert!(
        body.payload().is_empty(),
        "the payload carries {} bytes, and `grant` affirms that no external byte enters this \
         record",
        body.payload().len()
    );

    let Some(Detail::Permission(detail)) = body.detail() else {
        panic!("the permission record carries no structured detail");
    };
    // ⚠️ THE TWO NAMES WERE ALREADY HELD and are asserted here anyway, so that WHAT `grant` WRITES
    // is legible in one place instead of being deduced from four probes that ask questions.
    // Measured on 2026-09-01: swapping the two arguments of `PermissionDetail::new` inside
    // `grant` turns THREE probes of this file red — `a_granted_triple_is_granted`,
    // `a_record_of_another_species_is_stepped_over_and_not_read_as_a_permission`, and this one.
    assert_eq!(detail.tool(), "file");
    assert_eq!(detail.resource(), "/a");
    assert!(
        detail.write(),
        "a granted WRITE was written down as a read, and the grant could never be found again"
    );
}

#[test]
#[allow(non_snake_case)]
fn a_different_OPERATION_is_not_covered() {
    // ⛔ THE SAME TOOL AND THE SAME RESOURCE, and only the operation moves. Granting a READ must
    // not grant a WRITE — the whole default preset of ADR-0016 divides exactly on this line,
    // "reads, tests and builds proceed" against "writes, commands and network egress ask".
    let mut journal = MemoryJournal::new();
    let step = StepId::new(1);
    open_the_step(&mut journal, step);

    grant(&mut journal, step, &READ_A).expect("grant");

    let write_a = Permission {
        operation: Operation::Write,
        ..READ_A
    };
    assert!(!is_granted(&journal, &write_a).expect("is_granted"));
}

#[test]
#[allow(non_snake_case)]
fn a_different_RESOURCE_is_not_covered() {
    // ⛔ THE SAME TOOL AND THE SAME OPERATION, and only the resource moves. This is the half that
    // makes a permission a permission over `~/x` and not over "the filesystem".
    let mut journal = MemoryJournal::new();
    let step = StepId::new(1);
    open_the_step(&mut journal, step);

    grant(&mut journal, step, &READ_A).expect("grant");

    let read_b = Permission {
        resource: "/b",
        ..READ_A
    };
    assert!(!is_granted(&journal, &read_b).expect("is_granted"));
}

#[test]
#[allow(non_snake_case)]
fn a_different_TOOL_is_not_covered() {
    // ⛔ THIS IS THE ONE §6.6 NAMES OUT LOUD — "never «the tool»". Granting `(file, /a, read)` must
    // not grant `(net, /a, read)`: the resource and the operation are identical, and a projection
    // that dropped the tool from the comparison would hand a network tool a permission the user
    // gave a file tool.
    let mut journal = MemoryJournal::new();
    let step = StepId::new(1);
    open_the_step(&mut journal, step);

    grant(&mut journal, step, &READ_A).expect("grant");

    let net_a = Permission {
        tool: "net",
        ..READ_A
    };
    assert!(!is_granted(&journal, &net_a).expect("is_granted"));
}

#[test]
fn nothing_is_granted_on_an_empty_journal() {
    // ⛔ THE NON-VACUITY PROBE, and the asymmetry it exists for is worth the line: a projection
    // answering `true` to everything passes exactly ONE of the four probes above that ask
    // `is_granted`, and one answering `false` to everything passes THREE of them. Only this probe and `a_granted_triple_is_granted`
    // together pin both ends, and without this one an empty journal granting everything would go
    // unnoticed.
    let journal = MemoryJournal::new();

    assert!(!is_granted(&journal, &READ_A).expect("is_granted"));
}

#[test]
fn a_journal_that_refuses_the_note_makes_the_grant_fail() {
    // ⛔ §4 WAY ①: the error road of `grant`. Without this probe a `grant` that SWALLOWED the
    // refusal — `let _ = journal.note(..); Ok(())` — stays green on every probe above, because
    // they all use a journal that accepts. The recording is not a courtesy: if it fails, the grant
    // fails, or the whole mechanism rests on the diligence of whoever calls.
    let mut journal = NoteRefusingJournal { opened: false };
    let step = StepId::new(1);
    journal.intent(step, b"the step").expect("intent");
    assert!(journal.opened, "the fixture did not open the step");

    assert_eq!(
        grant(&mut journal, step, &READ_A),
        Err(JournalError::NotDurable)
    );
}

#[test]
fn a_journal_that_will_not_replay_is_not_an_answer_of_false() {
    // ⛔ §4 WAY ① FROM THE OTHER SIDE, and it is `is_granted`'s half rather than `grant`'s. A
    // projection that answered `Ok(false)` when it could not read the archive would report "not
    // granted" for "I do not know" — the silent degradation ADR-0019 forbids — and every probe
    // above would stay green, because they all replay successfully.
    let journal = ReplayRefusingJournal;

    assert_eq!(
        is_granted(&journal, &READ_A),
        Err(PermissionError::Journal(JournalError::NotDurable))
    );
}

#[test]
fn a_record_that_will_not_decode_is_not_an_answer_of_false() {
    // ⛔ §4 WAY ②, AND IT IS THE ONE THE DOC DECLARES LOAD-BEARING (P-15 · D20). `PermissionError`
    // says in so many words that folding an unreadable record into `false` is the silent partial
    // truth D20 exists to prevent — and until this probe NOTHING held that sentence. A `continue`
    // in place of the `?` on `Record::decode` passes every other probe in this file.
    //
    // ⚠️ THE BYTES ARE WRITTEN THROUGH THE PORT, which takes `&[u8]` and validates nothing — road
    // A4 of `kernel::boundary`, which already declares that nothing requires every write to the
    // journal to be a `Record`.
    let mut journal = MemoryJournal::new();
    let step = StepId::new(1);
    open_the_step(&mut journal, step);
    journal
        .note(step, b"not a record of any version")
        .expect("note");

    assert_eq!(
        is_granted(&journal, &READ_A),
        Err(PermissionError::Record(RecordError::Malformed))
    );
}

#[test]
fn a_record_of_another_species_is_stepped_over_and_not_read_as_a_permission() {
    // ⛔ §4 WAY ③: a MIXED journal, which is the only journal this function ever really sees — a
    // journal holding a permission holds at least the intent of the step it was noted upon. The
    // other species here is a ROUTING record, chosen because it CARRIES A `detail` of its own: a
    // projection that looked at the detail without checking the `kind` would have to step over it
    // for a different reason, and one that checked neither would read a `RoutingDetail` where a
    // `PermissionDetail` belongs.
    //
    // ⚠️ AND BOTH DIRECTIONS ARE HERE, because a probe that only asserted `true` could not tell a
    // correct skip from a projection that answers `true` to any journal with a permission in it
    // anywhere.
    let mut journal = MemoryJournal::new();
    let step = StepId::new(1);
    open_the_step(&mut journal, step);

    let routing = Record::V1(RecordV1::routing(
        EffectClass::Idempotent,
        Trust::Instruction,
        Vec::new(),
        "the gateway resolved the routing for this step",
        RoutingDetail::new("local-medium", 3, true),
    ))
    .encode();
    journal.note(step, &routing).expect("routing");

    grant(&mut journal, step, &READ_A).expect("grant");

    assert!(
        is_granted(&journal, &READ_A).expect("is_granted"),
        "the granted triple was lost among records of other species"
    );

    let net_a = Permission {
        tool: "net",
        ..READ_A
    };
    assert!(
        !is_granted(&journal, &net_a).expect("is_granted"),
        "a record of another species was read as a permission for a triple nobody granted"
    );
}

#[test]
fn a_permission_record_without_its_detail_is_not_an_answer_of_false() {
    // ⛔ §4 WAY ④, AND IT IS REACHABLE — measured on 2026-09-01 rather than declared impossible.
    // In SOURCE it is unpronounceable: `RecordV1::permission` takes its `PermissionDetail` by
    // value, so the `kind`/`detail` pair cannot be split. From BYTES it is one byte's work, which
    // is road A4 of `kernel::boundary` again — `Record::decode` is `pub` and privacy does not
    // watch the derived `Decode`.
    //
    // ⚠️ THE RECIPE IS THE ONE `record.rs` USES FOR ITS OWN DEMONSTRATION: encode a legitimate
    // record and overwrite the kind byte at run time. Measured, the result decodes to
    // `RecordV1 { kind: Permission, .. detail: None }` — a record claiming to be a grant while
    // naming no tool, no resource and no operation.
    //
    // ⛔ AND THE ANSWER IS AN ERROR AND NOT `false`, for the reason `PermissionError::Record`
    // carries: such a record grants nothing and denies nothing, so the only honest answer is that
    // this build cannot read it. A `continue` here would let a corrupt archive read as a clean
    // "not granted".
    let mut journal = MemoryJournal::new();
    let step = StepId::new(1);
    open_the_step(&mut journal, step);

    let mut bytes = Record::V1(RecordV1::note(
        EffectClass::Unrepeatable,
        Trust::Instruction,
        Vec::new(),
        "a note, about to be relabelled from outside",
    ))
    .encode();
    // Index 0 of `RecordV1` sits at byte 4 — `tests/frozen/record_v1.map`, checked by
    // `every_field_sits_at_the_offset_the_map_gives_it`. `05` is `RecordKind::Permission`.
    bytes[4] = 5;
    journal.note(step, &bytes).expect("note");

    assert_eq!(
        is_granted(&journal, &READ_A),
        Err(PermissionError::Record(RecordError::Malformed))
    );
}
