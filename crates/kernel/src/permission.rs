//! The permission mechanism (§6.6, ADR-0016). ⛔ A PERMISSION IS A TRIPLE — a TOOL, a RESOURCE
//! and an OPERATION — and never one of the three alone. `(file, ~/x, read)` is the unit; "the
//! filesystem" is not a permission, it is every permission a filesystem tool could ever want.
//!
//! ⛔ AND "WHICH PERMISSIONS ARE ACTIVE NOW" IS A PROJECTION OF THE JOURNAL, NOT A SECOND
//! ARCHIVE. The question is answered by re-reading what was written, exactly as `crate::reconcile`
//! answers "which steps are in doubt". A cache beside the journal would be a second place holding
//! one truth (§7.4.4), and the first of the two to stop being updated lies with authority.
//!
//! ⚠️ WHAT THIS MODULE DOES NOT BUY, declared here rather than left to be discovered:
//!
//! - THERE IS NO REVOCATION YET, so a permission recorded here is held FOR EVER by `is_granted`.
//!   Revocation belongs to the mediator and the approval cycle, staged by rule C; the trigger is
//!   THE FIRST REVOCATION, which will be a species of its own and one more arm in the loop below.
//! - NOTHING HERE IS SCOPED TO A SESSION. `V21` reads "a permission holds for the granted triple
//!   AND for the current session", and this module answers the first half only: there is no
//!   session in the kernel, no interface, no mediator and no approval cycle. The row stays
//!   `⚠️ parziale` deliberately.
//! - `grant` DOES NOT ASK ANYBODY. It records a decision somebody else took — the kernel provides
//!   the mechanism and implements no user-facing behaviour.

use crate::ports::journal::{Journal, JournalError, StepId};
use crate::record::{
    Detail, EffectClass, PermissionDetail, Record, RecordError, RecordKind, RecordV1, Trust,
};
use alloc::vec::Vec;

/// What may be done to the resource.
///
/// ⛔ TWO VARIANTS AND NOT THREE, AND THE LIMIT IS DECLARED WITH ITS OWN TRIGGER. The default
/// preset of ADR-0016 splits exactly here — "reads, tests and builds proceed" on one side,
/// "writes, commands and network egress ask" on the other — so two is what the policy that
/// exists can express. A third operation arrives WITH THE FIRST TOOL THAT NEEDS ONE, and there
/// are no tools yet.
///
/// ⚠️ IT IS A KERNEL TYPE AND NOT A WIRE TYPE, which is what leaves it free to grow: on the wire
/// the operation travels as the `bool` of `record::PermissionDetail`, whose doc carries the whole
/// argument. An enum here costs ONE `error[E0004]`, at `is_write`, the day it grows a third
/// variant, where an enum THERE would cost an index that never retires.
///
/// ⚠️ RICHIAMO DEL 2026-09-01: this said "costs NOTHING the day it grows a third variant", and
/// the measure said otherwise — the third variant reached the durable record as a READ and widened
/// a permission, with the workspace green digit for digit. It costs nothing only BECAUSE
/// `is_write` now makes the day a compile error; the sentence was true of the wish and false of
/// the code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    /// Reading the resource. Encoded as `write: false`.
    Read,
    /// Changing the resource. Encoded as `write: true`.
    Write,
}

impl Operation {
    /// How the operation reaches the wire: `false` for `Read`, `true` for `Write`.
    ///
    /// ⛔ AN EXHAUSTIVE `match` AND NOT `matches!`, AND THAT IS THE ONLY REASON THIS FUNCTION
    /// EXISTS. `matches!(op, Operation::Write)` folds EVERY other variant into `false`, so a third
    /// operation would reach the DURABLE record as a read. Measured on 2026-09-01, with `Execute`
    /// added to the enum: `cargo check --locked --workspace --all-targets` stayed at ZERO errors
    /// and the whole suite stayed green digit for digit (43 targets, 324 passed), while a
    /// throwaway probe written from OUTSIDE the crate showed `grant` writing `write: false` for an
    /// `Execute` and `is_granted` then answering `true` for a read nobody ever granted. The
    /// aliasing runs both ways. A `match` makes that day `error[E0004]` instead.
    ///
    /// ⚠️ AND IT IS ONE FUNCTION RATHER THAN TWO `match`es, because the two call sites read two
    /// DIFFERENT values and could drift apart, while one exhaustive `match` is already enough to
    /// make the growth a compile error — the lesson `E109` wrote for `verdict.outcome`.
    ///
    /// ⚖️ THIS IS `E109`'s CURE ON THE SITES ITS CENSUS DID NOT REACH. That census was scoped to
    /// the two types it was fixing (`CostClass`, `VerdictOutcome`), so it closed the OCCURRENCE and
    /// not the CLASS — an enum the kernel decides on, held by `==`/`matches!` and by no `match`.
    /// Re-censused on 2026-09-01 over the public enums of `kernel/src/`: the census named this one
    /// and `gateway::ConstraintClass`, and measured `Admission` held by ten arms and `Trust` by
    /// `minicbor`, which refuses a variant without an index.
    ///
    /// ⛔ RECALL OF 2026-09-01, NINTH REVIEW ROUND: this said the class had "exactly two members
    /// left", and the census was wrong INSIDE its own perimeter — `arbiter::Activity` and
    /// `arbiter::PreemptibleState` are `pub` in `kernel/src/`, were decided on with `matches!` and
    /// a `_` arm, and grew to `exit 0`; so did `executor::TaskState` and the simulator's
    /// `EntryKind`. A count is not a closure: a grep only names candidates, and the class is closed
    /// by GROWING each enum and reading `error[E0004]`. Errata `E124`.
    pub fn is_write(self) -> bool {
        match self {
            Operation::Read => false,
            Operation::Write => true,
        }
    }
}

/// The triple itself.
///
/// ⛔ BOTH NAMES ARE `&'static str`, AND THAT IS `I6` RATHER THAN TYPING CONVENIENCE. This is a
/// type the kernel DECIDES with: `is_granted` compares against it, and a name that arrived from
/// outside would be untrusted text sitting inside a decision (ADR-0014). It is the same reasoning
/// `gateway::Candidate` carries for the model name it is filtered by.
///
/// ⚠️ AND THE FIELDS STAY `pub`, WHICH IS NOT THE CHOICE `record::PermissionDetail` MAKES — the
/// asymmetry is deliberate and is the whole distinction `E95` draws. That type is ON THE WIRE, so
/// it carries `String` fields a runtime value could reach and needs a constructor to shut the
/// road. This one cannot be handed runtime text WITHOUT LEAKING, and a constructor around two
/// `'static` names would be ceremony that shuts nothing — it is aimed at exactly the road the
/// leak already goes around.
///
/// ⚠️ RICHIAMO DEL 2026-09-01: this said "cannot be handed runtime text AT ALL", and that is
/// false to the measure — `String::leak` yields a `&'static str` from arriving bytes, and
/// `boundary.rs` declares that road OPEN rather than papering over it. The qualified form is the
/// one this repository already owns, three times over in `record.rs`: "not producible from
/// arriving bytes WITHOUT LEAKING". ⛔ The CONCLUSION was never in doubt and is unchanged; it was
/// the PREMISE that overclaimed, and an overclaimed premise under a true conclusion is the shape
/// a reader trusts and should not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Permission {
    pub tool: &'static str,
    pub resource: &'static str,
    pub operation: Operation,
}

/// What can go wrong ANSWERING the question, as opposed to answering it "no".
///
/// ⛔ WHY `is_granted` CANNOT SIMPLY RETURN `false` WHEN IT CANNOT READ THE ARCHIVE. A journal
/// that will not replay, or a record this build cannot decode, means THE ANSWER IS UNKNOWN — and
/// "unknown" reported as "not granted" is the silent partial truth `D20` exists to prevent and the
/// silent degradation `ADR-0019` forbids outright. `false` is a real answer to a real question;
/// folding a failure into it would make the two indistinguishable at exactly the call site where a
/// caller is about to decide whether an effect may reach the world.
///
/// ⛔ IT COMPOSES THE TWO CAUSES THAT REALLY EXIST, and there are exactly two because the
/// projection does exactly two things: it REPLAYS (the port's failure) and it DECODES (the
/// record's). Neither is folded into the other — `JournalError`'s own doc keeps the reason for a
/// write failure with whoever implements the port, and `RecordError` is the kernel's own word for
/// bytes it cannot read.
///
/// ⚠️ THE COST IS DECLARED TWICE OVER. A new error type is new surface, and the branch that
/// produces `Record` MUST have its own probe or it is a live mutant the day it is written —
/// `tests/permission_triple.rs` carries two, one per road into it. And this is THE FIRST ERROR IN
/// `kernel` WHOSE VARIANTS CARRY A PAYLOAD: `framing.rs` said flatly that none does, and that
/// sentence now carries a dated recall. The payloads are CAUSES and not consumed values, they cost
/// nothing (both are `Copy`), and discarding them would make this type say less than the function
/// knows. The shape is `platform::journal::OpenError`'s, which composes `io::Error` and
/// `redb::Error` the same way and for the same reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionError {
    /// The journal would not hand its records back.
    Journal(JournalError),
    /// A record in the journal could not be read as a record this build understands. ⛔ IT ALSO
    /// COVERS A `Permission` RECORD WHOSE `detail` IS NOT A PERMISSION, which is unpronounceable
    /// in source — `RecordV1::permission` takes its detail by value — and reachable from BYTES,
    /// measured. Such a record names no triple, so it grants nothing and denies nothing: the only
    /// honest answer is that this build cannot read it.
    Record(RecordError),
}

/// Records that a permission was granted, upon a step somebody else opened.
///
/// ⛔ IT WRITES A NOTE AND NOT AN INTENT OR AN OUTCOME, for the reason `Journal::note` gives: a
/// grant is an annotation UPON a step, it touches nothing outside, and by ADR-0007 it is therefore
/// not a step of its own. ⛔ THE WRITE-AHEAD DISCIPLINE THEREFORE APPLIES TO THE CALLER: a note on
/// a step with no intent is `OutOfOrder`, and opening the step belongs to whoever owns it and
/// never to this function — the same asymmetry `sensor::run_the_ring` declares for the step it
/// judges.
///
/// ⚠️ `Trust::Instruction` WITH AN EMPTY PAYLOAD, and the label is TRUE rather than decorative: no
/// external byte enters this record. Both names of the triple are `&'static str`, so everything
/// written here is the kernel's own vocabulary. The precedent is `gateway::dispatch` and
/// `Arbiter::set_policy`.
///
/// ⚠️ `EffectClass::Unrepeatable` IS INERT HERE, and it is said rather than left to be deduced:
/// `crate::reconcile` gives `RecordKind::Permission` an empty arm and provably never reads the
/// field. It is filled with what is TRUE of this record rather than with a class this crate
/// consults — a grant is a decision a person took once, and replaying it is not the same act as
/// taking it again. That is the opposite reading from `dispatch`'s `Idempotent`, and the
/// difference is real: re-resolving a routing recomputes a function of the configuration, whereas
/// re-granting a permission would re-answer a question only a human can answer.
pub fn grant<J: Journal>(
    journal: &mut J,
    step: StepId,
    permission: &Permission,
) -> Result<(), JournalError> {
    // ⛔ THROUGH THE SPECIES CONSTRUCTOR AND NOT A STRUCT LITERAL: `RecordV1` has no public field
    // since AUD-050, and `PermissionDetail` none since the day it arrived (`E95`). So the two
    // names reach the wire only by way of a `&'static str` parameter, at both levels.
    let record = Record::V1(RecordV1::permission(
        EffectClass::Unrepeatable,
        Trust::Instruction,
        Vec::new(),
        "a permission was granted for this triple",
        PermissionDetail::new(
            permission.tool,
            permission.resource,
            permission.operation.is_write(),
        ),
    ))
    .encode();

    journal.note(step, &record)
}

/// Answers whether THIS EXACT TRIPLE has been granted, by re-reading the journal.
///
/// ⛔ A `bool` AND NOT A LIST, AND THAT IS WHAT KEEPS `I6` INTACT. A list would have to hand back
/// DECODED names — `String`s built from bytes an archive supplied — which a caller would then
/// compare or display as though they were the kernel's own vocabulary. Answering a QUESTION means
/// the decoded values never leave this function: they are compared against `&'static str` the
/// caller already held, and dropped.
///
/// ⛔ A FREE FUNCTION TAKING THE PORT, exactly like `reconcile::steps_in_doubt`. A struct holding
/// the journal would give the projection STATE (`I1`, `I5`), and state is the one thing a
/// projection must not have: the answer would then depend on when the struct was built rather than
/// on what the archive says now.
///
/// ⚠️ THE COST OF ANSWERING BY RE-READING IS THE COST `Journal::replay` DECLARES: the whole journal
/// is loaded to answer one question, and on a production archive that does not hold. The remedy is
/// the same checkpoint that operation names, and it is closed by the first consumer that measures a
/// large journal — not invented here.
pub fn is_granted<J: Journal>(journal: &J, wanted: &Permission) -> Result<bool, PermissionError> {
    let wanted_write = wanted.operation.is_write();

    for (_, bytes) in journal.replay().map_err(PermissionError::Journal)? {
        // ⛔ A RECORD THIS BUILD CANNOT READ STOPS THE ANSWER, and it does NOT skip to the next
        // one. Skipping would mean reporting "not granted" while a grant may be sitting in the
        // very bytes that would not decode — see `PermissionError` for why that is forbidden.
        let Record::V1(body) = Record::decode(&bytes).map_err(PermissionError::Record)?;

        // ⚠️ EVERY OTHER SPECIES IS SKIPPED AND THAT IS ORDINARY, not a hole: a journal holding a
        // permission holds at least the INTENT of the step the permission was noted upon, so the
        // mixed journal is the only journal this function ever sees.
        if body.kind() != RecordKind::Permission {
            continue;
        }

        let Some(Detail::Permission(detail)) = body.detail() else {
            return Err(PermissionError::Record(RecordError::Malformed));
        };

        // ⛔ ALL THREE OF THE TRIPLE, AND THE `&&` IS THE WHOLE MECHANISM. Dropping any one of the
        // three conjuncts widens every granted permission over everything the dropped component
        // ranges over — which is precisely the "never «the filesystem», never «the tool»" of §6.6.
        // Each of the three has its own probe in `tests/permission_triple.rs`, because a single
        // probe changing two components at once cannot say which conjunct did the work.
        if detail.tool() == wanted.tool
            && detail.resource() == wanted.resource
            && detail.write() == wanted_write
        {
            return Ok(true);
        }
    }

    Ok(false)
}
