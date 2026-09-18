//! The registry of the program's functions (ADR-0038): ONE registry, MANY invokers, ONE
//! permission.
//!
//! ⛔ THE MECHANISM AND NOT THE CONTENT, which is rule 1 of ADR-0038 and the shape ADR-0009 gives
//! every kernel registry. This module knows how to HOLD a function, how to check its triple and
//! how to JOURNAL an invocation; WHICH functions exist is brought by the gui and by the
//! capabilities. ⛔ AND THE RULE IS HELD AT LEVEL 1 RATHER THAN BY GOOD INTENTIONS: nothing here
//! IMPORTS `Arbiter`, `VramPolicy` or any other effect, and it could not without an `import` a
//! reader would see. The day the gesture arrives (sub-project 12) it registers ITS functions from
//! outside, exactly as the click does — which is what "no gesture-only logic" means when it stops
//! being a sentence.
//!
//! ⛔ SO `invoke` TAKES THE EFFECT AS A CLOSURE. That is the only shape in which the mechanism can
//! run something it does not know, and the closure receives the journal because the effect writes
//! its own step (§5 of the sub-project 2 design: "l'effetto, che è il passo B di `set_policy`
//! com'è").
//!
//! ⛔ AND THE ARRIVING NAME NEVER BECOMES A `&'static str`. It is COMPARED against the registered
//! names and dropped — the discipline `permission::is_granted` already writes out for the triple:
//! a name from outside sitting in a type the kernel decides with is untrusted text inside a
//! decision (ADR-0014). What reaches the journal is the REGISTERED name, which is ours.
//!
//! ⚠️ WHAT THIS MODULE IS NOT, so the next reader does not go looking: it is not a second
//! permission system (ADR-0038, negative perimeter) — it asks `permission::is_granted` and
//! nothing else; and it does not hold the SESSION boundary of ADR-0016, because
//! `permission::is_granted` re-reads the whole journal and a granted triple therefore survives a
//! restart. That limit is declared in §5 of the sub-project 2 design and belongs to whoever brings
//! the runs, sub-project 3.

use alloc::vec::Vec;

use crate::permission::{self, Permission, PermissionError};
use crate::ports::ipc::ClientId;
use crate::ports::journal::{Journal, JournalError, StepId};
use crate::record::{EffectClass, InvocationDetail, Record, RecordV1, Trust};

/// Who asked.
///
/// ⛔ ONE VARIANT TODAY, AND THE OTHERS ARE NAMED IN ADR-0038 RATHER THAN GUESSED AT: gesture
/// (sub-project 12), voice (8), and the agent. Each arrives WITH its invoker, and none of them
/// touches this file's `invoke`.
///
/// ⚠️ IT CARRIES THE `ClientId` AND THE RECORD DOES NOT, which is deliberate and is worth the
/// line. The identifier is what the DISPATCH needs — it is how the answer finds its way back to
/// the client that asked — while the record keeps the CLASS of invoker and not the connection: a
/// `ClientId` is a handle on a socket that reconnecting changes, so journalling it would durably
/// record something that means nothing an hour later. The trigger for recording it is a consumer
/// that needs to tell two simultaneous invokers apart, and ADR-0004 says there is at most one gui.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Invoker {
    /// The gui's client, over the `ipc` port.
    Gui(ClientId),
}

impl Invoker {
    /// How the invoker reaches the durable record.
    ///
    /// ⛔ AN EXHAUSTIVE `match` AND NOT `as u8`, AND THAT IS THE ONLY REASON THIS FUNCTION EXISTS
    /// — `Operation::is_write`'s lesson at four values instead of two. A cast numbers a variant
    /// BY POSITION, so inserting `Gesture` before `Gui` would silently repoint every record
    /// already written, and the archive is the one thing that cannot be migrated cheaply. A
    /// `match` makes that day `error[E0004]`, here, where the decision belongs.
    ///
    /// ⚠️ AND THE CODES ARE WRITTEN OUT rather than derived: a new invoker takes THE NEXT FREE
    /// NUMBER and never one that has been used, which is rule 4 of §4.9.2 applied to a value
    /// instead of an index — the same promise, one level down.
    pub fn code(self) -> u8 {
        match self {
            Invoker::Gui(_) => 0,
        }
    }
}

/// A function of the program, as the registry holds it.
///
/// ⛔ THE NAME IS `&'static str`, LIKE BOTH NAMES OF `Permission` AND FOR THE SAME REASON (I6):
/// this is a type the kernel DECIDES with — `invoke` compares against it — and a name that
/// arrived from outside would be untrusted text sitting inside a decision (ADR-0014).
///
/// ⚠️ `Copy`, so `invoke` can hand the whole thing about without borrowing the registry across
/// the effect. Three small fields; nothing here owns anything.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Function {
    /// How it is named on the wire and in the record.
    pub name: &'static str,
    /// The triple of ADR-0016 that protects it — rule 2 of ADR-0038: the same permission
    /// whatever the invoker.
    pub permission: Permission,
    /// How its effect reconciles after a crash (ADR-0007). It is the class the invocation's own
    /// step carries, and it is MANDATORY here for the reason it is mandatory on the record:
    /// "an effect without a declared class" must not be expressible.
    pub effect: EffectClass,
}

/// Whether the registry must ASK about the permission, or has just been told.
///
/// ⛔ AN ENUM AND NOT A `bool`, which is the lesson `crate::permission::Operation::is_write`
/// already recorded and `crate::wire::ipc::Access` repeats: a `bool` folds every future third
/// case into one of the two, in silence.
///
/// ⛔ AND IT EXISTS BECAUSE THE APPROVAL PATH IS OTHERWISE UNREACHABLE (P-43).
/// `permission::grant` writes a NOTE and wants a step somebody else opened -- its own doc --
/// so decision 21 of the north star puts the grant INSIDE step A, between the invocation note
/// and the effect. The check in `invoke` runs BEFORE step A is opened, so on the second pass
/// it would answer "not granted" again and the round would never close. The order belongs to
/// the mechanism that owns it, not to the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Approval {
    /// The invocation arrived on its own: the registry asks `is_granted` and refuses if not.
    Checked,
    /// The user has just approved the triple. The registry does not ask, and writes the grant
    /// on step A, between the invocation note and the effect.
    JustGiven,
}

/// What can go wrong invoking.
///
/// ⛔ `NotRegistered` AND `PermissionRequired` ARE NOT THE SAME REFUSAL, and folding them would
/// lose exactly the distinction the gui needs: one is a bug in the caller, the other is a
/// question for the user. The gui turns the second into the confirmation window (§6a) and the
/// first into nothing at all.
///
/// ⛔ AND `PermissionRequired` CARRIES THE TRIPLE, because the gui has to SHOW it: ADR-0016 wants
/// the user to approve `(tool, resource, operation)` and not "something". It is the REGISTERED
/// triple, so all three names are ours.
///
/// ⚠️ `Permission(PermissionError)` IS NOT `PermissionRequired`: the first says the archive would
/// not answer, the second that it answered no. The paragraph of `permission::PermissionError`'s
/// own doc that opens "WHY `is_granted` CANNOT SIMPLY RETURN `false`" spells out why "unknown"
/// reported as "not granted" is forbidden, and folding them here would undo that at the call site
/// that matters most.
///
/// ⚠️ AND TWO OF THESE FOUR VARIANTS ARE EXERCISED BY NO PROBE, declared rather than left to be
/// discovered: `Permission(..)` and `Journal(..)` are built at six sites of `invoke`, and
/// `tests/registry.rs` reaches neither — every journal it hands `invoke` answers every call.
/// They stay because they are PROPAGATION arms, which every kernel module that touches a port
/// has, and because folding them into a refusal would be exactly the "unknown reported as no"
/// the paragraph above forbids. ⛔ AND NO TRIGGER IS NAMED HERE: which consumer will first hand
/// this function a journal that refuses is not known today, and a deadline written in prose that
/// nothing makes fire is gotcha #77.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvokeError {
    /// No function of that name is registered. ⛔ AND NOTHING IS WRITTEN — not an intent, not a
    /// note. A name nobody registered is not an event of this program, and journalling it would
    /// let a peer fill the durable archive with words it chose.
    NotRegistered,
    /// The triple is not granted. Nothing is written, and the effect is not run.
    PermissionRequired(Permission),
    /// The archive would not say whether it is granted.
    Permission(PermissionError),
    /// A write was refused.
    Journal(JournalError),
}

/// The registry itself.
///
/// ⛔ IT HOLDS STATE AND `permission::is_granted` DOES NOT, and the asymmetry is not an
/// inconsistency. That one is a PROJECTION of the journal, and a projection with state would
/// answer from when it was built instead of from what the archive says. This is a REGISTRATION
/// TABLE — the shape ADR-0009 gives the registries of guides and sensors — and it lives in the
/// core, where I1 says the authoritative state lives.
pub struct Registry {
    functions: Vec<Function>,
}

// ⛔ NO `impl Default` — the same decision, for the same reason, as `MemoryJournal` and
// `SystemReactor`: nothing calls it. The argument is written out once, in
// `crates/platform/src/reactor.rs`.
impl Registry {
    /// A registry that holds nothing yet.
    ///
    /// ⚠️ `const` IS PROMISED BY NAME AND EXERCISED BY NOTHING, and it is declared here rather
    /// than left to be found: both call sites are `let` bindings, and no `const` or `static` in
    /// this repository depends on it, so removing the qualifier leaves every bench green. It
    /// stays for the reason `E27` kept the derives of `custody`: a name promised to the tasks
    /// that come after is a CONTRACT, and what is not yet exercised is declared instead of being
    /// removed or claimed. ⛔ AND NO TRIGGER IS NAMED: which consumer will first want a registry
    /// in a `const` context is not known today, and a deadline in prose that nothing makes fire
    /// is gotcha #77.
    pub const fn new() -> Self {
        Registry {
            functions: Vec::new(),
        }
    }

    /// Registers a function.
    ///
    /// ⛔ A NAME ALREADY REGISTERED REPLACES, AND IT DOES NOT PILE UP. Two entries under one name
    /// would make `invoke` depend on whether the search reads the first or the last, which is a
    /// silent difference in what a permission protects. ⚠️ AND IT IS NOT AN ERROR, because there
    /// is no caller for one: registration happens once at start-up, from source, and a `Result`
    /// nobody can fail would be ceremony. The day registration is driven by data, it grows one.
    pub fn register(&mut self, function: Function) {
        match self
            .functions
            .iter_mut()
            .find(|held| held.name == function.name)
        {
            Some(held) => *held = function,
            None => self.functions.push(function),
        }
    }

    /// The function held under this name, if the registry holds one.
    ///
    /// ⛔ IT EXISTS FOR THE APPROVAL PATH AND ITS CALLER ARRIVES WITH `crate::serving` IN TASK 7,
    /// and until then `invoke` is the only one: `crate::serving` compares the triple a peer sends
    /// back against the triple THE REGISTRY holds, rather than trusting the strings that came in
    /// (ADR-0014, and the doc of `crate::wire::ipc::Triple` asks that consumer for it by name).
    pub fn held(&self, name: &str) -> Option<Function> {
        self.functions.iter().find(|held| held.name == name).copied()
    }

    /// Invokes the function named `name`, if it is registered and its triple is granted.
    ///
    /// The order is the one §5 of the sub-project 2 design fixes, and each line is load-bearing:
    ///
    /// 1. the name is looked up — not registered, nothing is written;
    /// 2. with `Approval::Checked`, the triple is asked of the journal — not granted, nothing
    ///    is written and the effect is NOT run, which is the probe
    ///    `a_triple_that_is_not_granted_never_reaches_the_effect`;
    /// 3. `intent` on step A, carrying the FUNCTION'S class;
    /// 4. a `note` on step A with the structured detail — who invoked what;
    /// 5. with `Approval::JustGiven`, the GRANT on step A — decision 21 of the north star puts
    ///    the permission on the step it UNLOCKS, and `permission::grant` wants a step already
    ///    open;
    /// 6. the EFFECT, which opens and closes its own step B if it has one;
    /// 7. `outcome` on step A.
    ///
    /// ⛔ THE NOTE COMES AFTER THE INTENT AND BEFORE THE EFFECT, and `Journal::note` enforces half
    /// of that by refusing a note on a step with no intent. The other half — before the effect —
    /// is what makes a crash mid-invocation reconstructible: step A is in doubt WITH ITS CLASS,
    /// and the note says what was being attempted.
    ///
    /// ⛔ `argument` IS `&[u8]` AND GOES INTO THE PAYLOAD, NOT INTO THE DETAIL. It is text the
    /// peer chose, so it is untrusted by inheritance (ADR-0014) and travels under the `trust`
    /// label that says so — the shape `VerdictDetail` established for its detail text. Bytes and
    /// not `&str` because the port exchanges bytes and this function has no reason to require
    /// that an argument be text at all.
    ///
    /// ⚠️ THE EFFECT RECEIVES THE JOURNAL, and it has to: `Arbiter::set_policy` writes step B
    /// through it, and lending it twice is not expressible. The cost is that the effect could
    /// write anything at all — this function cannot police it, and says so rather than pretending.
    pub fn invoke<J, T, E>(
        &self,
        journal: &mut J,
        step: StepId,
        name: &str,
        invoker: Invoker,
        argument: &[u8],
        approval: Approval,
        effect: E,
    ) -> Result<T, InvokeError>
    where
        J: Journal,
        E: FnOnce(&mut J) -> Result<T, JournalError>,
    {
        let Some(function) = self.held(name) else {
            return Err(InvokeError::NotRegistered);
        };

        if approval == Approval::Checked
            && !permission::is_granted(journal, &function.permission)
                .map_err(InvokeError::Permission)?
        {
            return Err(InvokeError::PermissionRequired(function.permission));
        }

        journal
            .intent(step, &opened(function))
            .map_err(InvokeError::Journal)?;
        journal
            .note(step, &noted(function, invoker, argument))
            .map_err(InvokeError::Journal)?;

        // ⛔ THE GRANT GOES ON STEP A, BETWEEN THE INVOCATION NOTE AND THE EFFECT -- decision 21
        // of the north star, held here rather than by the caller. The permission settles on the
        // step it UNLOCKS, exactly as `sensor::run_the_ring` settles a verdict on the step it
        // judges.
        if approval == Approval::JustGiven {
            permission::grant(journal, step, &function.permission)
                .map_err(InvokeError::Journal)?;
        }

        let produced = effect(journal).map_err(InvokeError::Journal)?;

        journal
            .outcome(step, &closed(function))
            .map_err(InvokeError::Journal)?;

        Ok(produced)
    }
}

/// Step A's intent. ⚠️ THE PAYLOAD IS EMPTY AND THE ARGUMENT IS NOT HERE: it belongs to the note,
/// with the detail that says what it is an argument TO. An intent that carried it would put the
/// same untrusted bytes in the archive twice.
fn opened(function: Function) -> Vec<u8> {
    Record::V1(RecordV1::intent(
        function.effect,
        Trust::Instruction,
        Vec::new(),
        "a function of the registry was invoked",
    ))
    .encode()
}

/// The note that says who invoked what, with the argument in the payload.
fn noted(function: Function, invoker: Invoker, argument: &[u8]) -> Vec<u8> {
    Record::V1(RecordV1::invocation(
        function.effect,
        // ⛔ `Untrusted` BECAUSE OF THE PAYLOAD AND NOT BECAUSE OF THE DETAIL. The label describes
        // the payload (ADR-0014), and the payload is the argument the peer chose. The detail
        // beside it is ours, which is exactly why it is a detail and not a payload.
        Trust::Untrusted,
        Vec::from(argument),
        "who invoked which function, and with what",
        InvocationDetail::new(function.name, invoker.code()),
    ))
    .encode()
}

/// Step A's outcome.
fn closed(function: Function) -> Vec<u8> {
    Record::V1(RecordV1::outcome(
        function.effect,
        Trust::Instruction,
        Vec::new(),
        "the invocation finished",
    ))
    .encode()
}
