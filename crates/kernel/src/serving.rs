//! The activity that serves the gui over the `ipc` port (§5 of the sub-project 2 design).
//!
//! ⛔ IT LIVES IN `kernel` AND NOT IN `daemon`, and that is what makes it checkable at all. A
//! dispatch inside a binary is reachable by no integration test -- `daemon`'s own unit module
//! writes out why -- the DST of task 10 drives THIS one over a wire of its own, held outside the
//! core behind a `RefCell` (D32), and `gui/fake-core` runs THIS one on in-memory ports instead of
//! writing a second dispatch of its own (§7). A second
//! copy is green on the day the two drift apart.
//!
//! ⛔ NAMED `serving` AND NOT `dispatch`: `crate::gateway::dispatch` already exists and means
//! another thing -- handing a conforming token to a provider. Two `dispatch`es in one kernel is
//! the ambiguity `crate::numbering` refused for `counter`. And not `session` either:
//! `crate::permission` opens by declaring that there is no session in the kernel, and the
//! permission session boundary belongs to a later sub-project.
//!
//! # One borrow per turn, and never across a suspension
//!
//! ⛔ THE WHOLE CORE SITS BEHIND ONE `RefCell` THE CALLER OWNS, which the tap of `gui/fake-core`
//! shares (§7). That is sound here and it is NOT sound by luck: the executor polls ONE activity at
//! a time and nothing inside a poll can reach the executor (§2.4.1), so two activities cannot be
//! inside the cell at once -- PROVIDED no borrow is held across an `.await`. Every borrow in this
//! file is taken and dropped inside one block, and the tapped round of `tests/serving.rs` is what
//! holds it: a borrow that survived the `.await` panics there.
//!
//! # What it does NOT do, and each one has its closer
//!
//! - ⛔ IT DOES NOT SERVE `IpcMessage::Request` -- D5 of the sub-project 2 part 2 plan, argued on the
//!   branch itself. Nothing here builds a `ResourceProfile`, so no value the peer chose reaches the
//!   arbiter at all.
//! - ⚠️ NOTHING HERE EVER CALLS `ClientGrants::register`, which follows from the line above: with
//!   no request served, no grant is ever issued to a client. The register and `on_disconnect` are
//!   wired anyway, because ADR-0033 says the core notices a dead gui FROM THE IPC DISCONNECTION and
//!   reconciles; the writer arrives with the 3D pillar, and the benches hold the wiring meanwhile
//!   by putting a grant in by hand -- `tests/serving.rs` from task 7, and the DST campaign of
//!   task 10 since 2026-09-19. Which files those are is what the command on `grants` prints.
//! - ⚠️ IT NEVER STOPS. In production the turn limit is `u64::MAX` (task 9); under a finite limit
//!   the run ends as `RunError::TurnLimitReached`, which is the expected answer and not a failure.

use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::arbiter::{Arbiter, LocalPolicy, MakeRoom, RemotePolicy, VramPolicy};
use crate::client::ClientGrants;
use crate::degradation::degradation_now;
use crate::executor::{nap, Sleep};
use crate::numbering::Progressive;
use crate::parameters::Parameters;
use crate::permission::{Operation, Permission};
use crate::ports::custody::{Custody, CustodyError, CustodyKey};
use crate::ports::ipc::{ClientId, Ipc, IpcError};
use crate::ports::journal::{Journal, StepId};
use crate::ports::reactor::Reactor;
use crate::record::{Detail, EffectClass, Record, RecordKind};
use crate::registry::{Approval, Function, InvokeError, Invoker, Registry};
use crate::time::Monotonic;
use crate::wire::ipc::{
    build_stamp, Access, BuildStamp, Call, DegradationReport, IpcMessage, LayoutState, PolicyName,
    PolicyReport, Protection, StepSummary, Triple,
};

/// The ONE function the registry holds in sub-project 2: the VRAM policy change.
///
/// ⛔ IT IS REGISTERED HERE AND NOT IN `crate::registry` -- D16, which is rule 1 of ADR-0038: the
/// kernel gives the MECHANISM, and WHICH functions exist is brought by whoever uses them. A
/// registry that named `Arbiter::set_policy` would have to import the arbiter, and the second
/// invoker -- the gesture, with sub-project 12 -- would have to add its effect in there too, which
/// is the "logic for gestures only" that ADR refuses, inside out.
///
/// ⚠️ `Idempotent` AND NOT `Unrepeatable`: setting the policy twice to the same value leaves the
/// same world, which is the argument written beside `Arbiter::set_policy` itself.
pub const POLICY_FUNCTION: Function = Function {
    name: "vram-policy",
    permission: Permission {
        tool: "registry",
        resource: "arbiter",
        operation: Operation::Write,
    },
    effect: EffectClass::Idempotent,
};

/// Where a known client has got to.
enum Stage {
    /// Connected, and it has not introduced itself. Only `Hello` is answered.
    Greeting,
    /// The stamp matched. From here the core sends the piece that CHANGES, and the gui does not
    /// pull (§6.1.4).
    Attending {
        /// The last degradation this client was told about -- D23: the sweep sends only on change.
        told: DegradationReport,
    },
}

struct Client {
    id: ClientId,
    stage: Stage,
}

/// Whether the client the turn is looking at stays on the table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Keep,
    Forget,
}

/// Everything the serving activity owns, in one place so that one cell holds it all.
pub struct Core<I: Ipc, J: Journal, C: Custody> {
    ipc: I,
    journal: J,
    custody: C,
    arbiter: Arbiter,
    registry: Registry,
    grants: ClientGrants,
    steps: Progressive,
    clients: Vec<Client>,
    parameters: Parameters,
}

impl<I: Ipc, J: Journal, C: Custody> Core<I, J, C> {
    /// ⛔ THE REGISTRY IS BUILT AND FILLED HERE, with the one function of sub-project 2: D16.
    /// A caller cannot forget to register it, and a caller cannot register a second one either --
    /// which is deliberate: the invokers grow (the gesture, the voice), the FUNCTIONS of sub-project
    /// 2 do not.
    pub fn new(
        ipc: I,
        journal: J,
        custody: C,
        arbiter: Arbiter,
        steps: Progressive,
        parameters: Parameters,
    ) -> Self {
        let mut registry = Registry::new();
        registry.register(POLICY_FUNCTION);
        Core {
            ipc,
            journal,
            custody,
            arbiter,
            registry,
            grants: ClientGrants::new(),
            steps,
            clients: Vec::new(),
            parameters,
        }
    }

    /// The journal, for whoever shares this cell.
    pub fn journal(&mut self) -> &mut J {
        &mut self.journal
    }

    /// The arbiter, for whoever shares this cell.
    pub fn arbiter(&mut self) -> &mut Arbiter {
        &mut self.arbiter
    }

    /// The seventh port, to read what it holds.
    ///
    /// ⚠️ `&C` AND NOT `&mut C`: `keep` belongs to the dispatch, and a caller that could write the
    /// layout from outside would be a second writer of a value the gui believes the core owns.
    pub fn custody(&self) -> &C {
        &self.custody
    }

    /// The register of who holds what.
    ///
    /// ⚠️ IT HAS NO PRODUCTION WRITER, and that is stated rather than hidden: sub-project 2 issues
    /// no grant to a client (D5), so every caller is a bench or a campaign. Removing it would
    /// leave `on_disconnect`'s wiring held by nothing at all. ⛔ WHO CALLS IT IS WHAT THE COMMAND
    /// PRINTS, not a tally that ages the day another bench arrives:
    /// `grep -rn '\.grants()' crates/ --include='*.rs' | grep -vE '^[^:]+:[0-9]+: *//[!/]'`
    /// -- and the filter is there because a line that NAMES the pattern is counted by it (E63,
    /// E88, E96 of the sub-project 2 plan). Here the tally said ONE bench and the campaign of
    /// task 10 made it two, on 2026-09-19.
    pub fn grants(&mut self) -> &mut ClientGrants {
        &mut self.grants
    }

    /// Who has finished the handshake. ⚠️ A client still in `Greeting`, and a client that has been
    /// refused or has died, is not here.
    pub fn attending(&self) -> Vec<ClientId> {
        self.clients
            .iter()
            .filter(|client| matches!(client.stage, Stage::Attending { .. }))
            .map(|client| client.id)
            .collect()
    }
}

/// The activity: one turn, then a nap of the delivered tick, for ever.
///
/// ⛔ IT POLLS, AND THAT IS NOT A CHOICE: the `reactor` port has no I/O readiness, which §5 of the
/// sub-project 2 design states as the reason for the tick. What IS a choice is the tick's value, and
/// it is delivered (ADR-0034) rather than picked here.
pub async fn serve<'a, I, J, C, R>(core: &'a RefCell<Core<I, J, C>>, clock: &'a R, sleep: &'a Sleep)
where
    I: Ipc,
    J: Journal,
    C: Custody,
    R: Reactor,
{
    loop {
        // ⛔ ONE BLOCK, AND THE BORROW DIES WITH IT. See the head of this file: the `.await` below
        // must never be reached with the cell borrowed.
        let tick = {
            let core = &mut *core.borrow_mut();
            let now = clock.now();
            core.take_connections();
            core.answer_everyone(now);
            core.sweep_degradation(now);
            core.parameters.gui_tick()
        };
        nap(sleep, clock.now().saturating_add(tick)).await;
    }
}

impl<I: Ipc, J: Journal, C: Custody> Core<I, J, C> {
    /// Everybody the listener has ready, added in the greeting stage.
    fn take_connections(&mut self) {
        while let Some(id) = self.ipc.accept() {
            self.clients.push(Client { id, stage: Stage::Greeting });
        }
    }

    /// One `receive` per known client, and the dispatch of whatever came back.
    fn answer_everyone(&mut self, now: Monotonic) {
        let mut index = 0;
        while index < self.clients.len() {
            let id = self.clients[index].id;
            let outcome = match self.ipc.receive(id) {
                // ⛔ ONE MESSAGE PER CLIENT PER TURN, AND IT IS A CHOICE: draining a client dry
                // would let one peer hold the turn for as long as it keeps talking, and every
                // other client -- and the degradation sweep -- would wait behind it.
                Ok(None) => Outcome::Keep,
                Ok(Some(bytes)) => self.answer(index, &bytes),
                // ⛔ THE ONE SIGNAL THERE IS. `Ipc` has no disconnection event, so an `Err` on
                // `send` or `receive` is what ADR-0033 calls "the core notices from the ipc
                // disconnection" -- `gui_death_campaign.rs` reads it in exactly this place.
                Err(IpcError::Disconnected) => Outcome::Forget,
                // ⚠️ A PEER THAT TALKS NONSENSE IS STILL THERE, which is `IpcError`'s own words:
                // folding this into `Disconnected` would tear down a live gui over one bad frame.
                Err(IpcError::MalformedMessage) => Outcome::Keep,
            };
            match outcome {
                Outcome::Keep => index += 1,
                Outcome::Forget => self.forget(index, now),
            }
        }
    }

    /// One message, dispatched.
    fn answer(&mut self, index: usize, bytes: &[u8]) -> Outcome {
        let Ok(message) = IpcMessage::decode(bytes) else {
            // ⚠️ SAME READING AS `MalformedMessage` ABOVE: bytes that will not decode are a bad
            // frame, not a dead peer.
            return Outcome::Keep;
        };
        let id = self.clients[index].id;
        // ⛔ ONLY `Hello` IS ANSWERED BEFORE THE HANDSHAKE, which is §5 of the sub-project 2 design
        // -- "the first message must be `Hello`" -- and the promise `Stage::Greeting`'s own doc
        // makes. Without this gate `Invoke`, `Approve` and `SaveLayout` are
        // dispatched for a peer that never introduced itself: MEASURED on 2026-09-18, such a peer
        // moved the policy to `local`, its package reached the seventh port, and the six records of
        // the round were in the journal -- `Permission` among them -- while `attending()` was EMPTY
        // (E46 of the plan).
        //
        // ⚠️ REFUSED WITHOUT A WORD AND WITHOUT A RECORD, like the three silent roads this dispatch
        // already has: what arrived is content the peer chose, and untrusted content informs, it
        // never authorises (ADR-0014). ⚠️ AND THE CLIENT IS KEPT: speaking out of turn is not a dead
        // peer, any more than a bad frame is, and its `Hello` is still read on a later turn.
        //
        // ⛔ IT ASKS WHAT THE MESSAGE IS AND NOT WHETHER IT IS ONE OF THE SERVED ONES, so a variant
        // added tomorrow is refused before the handshake rather than served by an omission. The
        // other direction -- that a variant cannot be forgotten ALTOGETHER -- is the exhaustive
        // `match` below, which is why this is a gate in front of it and not three guards inside it.
        if !matches!(message, IpcMessage::Hello(_))
            && !matches!(self.clients[index].stage, Stage::Attending { .. })
        {
            return Outcome::Keep;
        }
        match message {
            IpcMessage::Hello(stamp) => self.greet(index, stamp),
            IpcMessage::Invoke(call) => self.run(id, call, Approval::Checked),
            IpcMessage::Approve { triple, call } => self.approve(id, triple, call),
            IpcMessage::SaveLayout(package) => self.keep_layout(id, &package),
            // ⛔ NOT SERVED, AND IT IS D5. Serving it would force the core to name a compute lane
            // and a preemption for a consumer that does not exist: `ComputeClass::Batch` is
            // documented "3D render, indexing, background runs" and ADR-0033 describes a VIEWER,
            // and no line anywhere says which is right -- choosing one would be a deduction
            // presented as a design. Refusing it with `Verdict::Refused` would LIE about that
            // type, which means "bigger than the whole machine". Not serving it asserts nothing
            // false, is the shape this repository already uses (`promote` has no caller either --
            // "declared, not pinned", gotcha #73), and takes the privilege away at the root: NO
            // VALUE OF THE PEER REACHES THE ARBITER, because nothing here builds a
            // `ResourceProfile` at all. ⚠️ Entry 27 of milestone 6's open items keeps its trigger
            // intact, and its closer is still the 3D pillar.
            IpcMessage::Request(_) => Outcome::Keep,
            // ⚠️ CORE -> GUI VARIANTS ARRIVING UPWARD. The schema is ONE enum for both directions
            // (I4 renounces versioning, so there is no per-direction type), which makes these
            // expressible and perfectly decodable -- so they are not `MalformedMessage`, and they
            // are IGNORED rather than answered. Tearing the client down for one is the move
            // `IpcError`'s own doc argues against: the gui is sacrificial, and a live one must not
            // be lost over a stray frame.
            IpcMessage::Accepted(_)
            | IpcMessage::StaleBuild(_)
            | IpcMessage::Degradation(_)
            | IpcMessage::Policy(_)
            | IpcMessage::PermissionRequired(_)
            | IpcMessage::Token { .. }
            | IpcMessage::Layout(_)
            | IpcMessage::Steps(_)
            | IpcMessage::Verdict(_) => Outcome::Keep,
        }
    }

    /// The handshake of §6.1.2, and the welcome of sequence 1.
    fn greet(&mut self, index: usize, stamp: BuildStamp) -> Outcome {
        let id = self.clients[index].id;
        if stamp != build_stamp() {
            // ⛔ "THE CORE CLOSES" IS NOT AN OPERATION OF THE PORT (decision 22). What the core
            // does is STOP LISTENING, so this client leaves the table: the gui does not start, says
            // so (§6.1.2), exits by itself, and there is nobody left here to see it go.
            let _ = self.tell(id, &IpcMessage::StaleBuild(build_stamp()));
            return Outcome::Forget;
        }

        let told = match self.degradation() {
            Some(report) => report,
            // ⛔ AN UNKNOWN DEGRADATION IS NOT SENT AS "NOTHING IS DEGRADED", which is the silent
            // degradation ADR-0019 forbids and the argument `DegradationError` spells out. The wire
            // has no third state for it: the gui is simply not told, and the sweep will tell it as
            // soon as the journal reads again. ⚠️ REGISTERED AND NOT TAKEN: whether `Degradation`
            // should gain an "unknown" the way `LayoutState` gained `Unavailable` (decision 35) is
            // the owner's, and it is a variant on a wire that never retires one.
            None => return Outcome::Keep,
        };

        // The welcome, in the order sequence 1 of the north star fixes.
        for message in [
            IpcMessage::Accepted(Protection::AsSystemAccount),
            IpcMessage::Degradation(told),
            IpcMessage::Policy(self.policy_report()),
            IpcMessage::Layout(self.layout()),
        ] {
            if self.tell(id, &message) == Outcome::Forget {
                return Outcome::Forget;
            }
        }
        if let Some(steps) = self.step_list() {
            if self.tell(id, &IpcMessage::Steps(steps)) == Outcome::Forget {
                return Outcome::Forget;
            }
        }

        self.clients[index].stage = Stage::Attending { told };
        Outcome::Keep
    }
}

impl<I: Ipc, J: Journal, C: Custody> Core<I, J, C> {
    /// An invocation, on either of the two roads of sequence 3.
    fn run(&mut self, id: ClientId, call: Call, approval: Approval) -> Outcome {
        let Some(policy) = policy_named(&call.argument) else {
            // ⚠️ AN ARGUMENT THAT NAMES NO POLICY IS REFUSED WITHOUT WRITING ANYTHING, exactly as
            // an unregistered name is: it is text the peer chose, and untrusted content informs,
            // it never authorises (ADR-0014).
            return Outcome::Keep;
        };

        // ⛔ DESTRUCTURED AND NOT REACHED THROUGH THE ACCESSORS, and it is forced rather than
        // tidy: the effect closure needs `&mut arbiter` while `invoke` is holding `&registry` and
        // `&mut journal`. Three methods on `&mut self` cannot be alive at once; three field
        // bindings can.
        let Core { registry, journal, arbiter, steps, .. } = self;
        // ⚠️ BOTH NUMBERS ARE TAKEN UP FRONT, and a refused invocation therefore burns two. That is
        // not a defect: ADR-0036 retires INDICES and never reuses them, and says nothing of the
        // sort about step numbers, which are a progressive and not a schema.
        let step_a = StepId::new(steps.take());
        let step_b = StepId::new(steps.take());
        let outcome = registry.invoke(
            journal,
            step_a,
            &call.function,
            Invoker::Gui(id),
            call.argument.as_bytes(),
            approval,
            |journal| arbiter.set_policy(policy, step_b, journal),
        );

        match outcome {
            Ok(()) => {
                // ⚠️ TWO STATEMENTS AND NOT ONE, deliberately: `self.tell(…, self.policy_report())`
                // asks the compiler for a two-phase borrow, which works and is the kind of line a
                // later edit turns into `E0502` for no reason anybody can see.
                let report = self.policy_report();
                if self.tell(id, &IpcMessage::Policy(report)) == Outcome::Forget {
                    return Outcome::Forget;
                }
                match self.step_list() {
                    Some(steps) => self.tell(id, &IpcMessage::Steps(steps)),
                    None => Outcome::Keep,
                }
            }
            Err(InvokeError::PermissionRequired(permission)) => {
                self.tell(id, &IpcMessage::PermissionRequired(triple_of(permission)))
            }
            // ⚠️ THE OTHER THREE SAY NOTHING, and each for its own reason. `NotRegistered` is a
            // name the peer chose and is refused in silence, like the argument above. `Permission`
            // and `Journal` are the archive failing to answer or to write: there is no variant on
            // this wire that says "the core could not read its own journal", and inventing one
            // here would be a schema decision taken in a dispatch. ⛔ REGISTERED AND NOT TAKEN,
            // with its closer: the first module that has to show a core in trouble -- the Status
            // tile of sub-project 6.
            Err(InvokeError::NotRegistered)
            | Err(InvokeError::Permission(_))
            | Err(InvokeError::Journal(_)) => Outcome::Keep,
        }
    }

    /// The approval road: the triple comes back from the peer, and it is COMPARED, never believed.
    fn approve(&mut self, id: ClientId, triple: Triple, call: Call) -> Outcome {
        // ⛔ WHAT DECIDES IS THE TRIPLE THE REGISTRY HOLDS, and this is the consumer the doc of
        // `crate::wire::ipc::Triple` names: the strings that came back are untrusted (ADR-0014), so
        // a core that converted them into a `Permission` would be granting a permission for a
        // triple nobody was ever asked about. It cannot even be done by accident -- `Permission`
        // wants `&'static str` and a `String` off the wire is not one.
        let Some(function) = self.registry.held(&call.function) else {
            return Outcome::Keep;
        };
        if triple != triple_of(function.permission) {
            return Outcome::Keep;
        }
        self.run(id, call, Approval::JustGiven)
    }

    /// `SaveLayout`, and what comes back is what the port HOLDS (decision 13).
    fn keep_layout(&mut self, id: ClientId, package: &[u8]) -> Outcome {
        // ⛔ THE WRITE'S ERROR IS NOT SENT ON. Decision 13 says the core answers with what it holds
        // after every write, so a failed write comes back as the OLD package and the gui sees it by
        // comparing -- no error variant, and no gui that believes a save stuck when it did not.
        let _ = self.custody.keep(CustodyKey::Layout, package);
        let state = self.layout();
        self.tell(id, &IpcMessage::Layout(state))
    }

    /// D23: every turn, only while somebody is attending, and only when it CHANGED.
    fn sweep_degradation(&mut self, now: Monotonic) {
        if self.attending().is_empty() {
            // ⛔ A BOUND AND NOT AN OPTIMISATION: `degradation_now` re-reads the whole journal, and
            // with no gui attending there is nobody to tell. The cost is the one that function
            // declares of itself, and its remedy is the checkpoint `Journal::replay` names.
            return;
        }
        let Some(state) = self.degradation() else {
            return;
        };

        let mut index = 0;
        while index < self.clients.len() {
            let stale = matches!(self.clients[index].stage, Stage::Attending { told } if told != state);
            if !stale {
                index += 1;
                continue;
            }
            let id = self.clients[index].id;
            match self.tell(id, &IpcMessage::Degradation(state)) {
                Outcome::Keep => {
                    self.clients[index].stage = Stage::Attending { told: state };
                    index += 1;
                }
                Outcome::Forget => self.forget(index, now),
            }
        }
    }

    /// A client leaves the table, and every grant it held goes back to the arbiter.
    ///
    /// ⚠️ THE `Result` OF `on_disconnect` IS DROPPED, and it is said rather than hidden: the only
    /// `Err` it can give is `ReleaseError::UnknownGrant`, which means a grant of ANOTHER arbiter --
    /// this core builds one, so it is unreachable here -- and `ClientGrants` keeps the pairs it has
    /// not released, so a later caller with the right arbiter loses none of them.
    fn forget(&mut self, index: usize, now: Monotonic) {
        let client = self.clients.remove(index);
        let _ = self.grants.on_disconnect(client.id, &mut self.arbiter, now);
    }

    /// One message out, and what a failure on the way means.
    fn tell(&mut self, id: ClientId, message: &IpcMessage) -> Outcome {
        // ⚠️ A MESSAGE THAT WILL NOT ENCODE BECOMES AN EMPTY BODY rather than an error, which is
        // `IpcMessage::encode`'s own containment argument; what can still arrive here is a
        // `WireError` from the ENVELOPE. Nothing is sent, and nothing is said about it: there is no
        // variant that means "the core could not speak", and the gui asks for nothing (§6.1.4).
        let Ok(bytes) = message.encode() else {
            return Outcome::Keep;
        };
        match self.ipc.send(id, &bytes) {
            Ok(()) => Outcome::Keep,
            Err(IpcError::Disconnected) => Outcome::Forget,
            // ⚠️ UNREACHABLE ON THIS OPERATION and written rather than guessed at: the port's own
            // doc says `MalformedMessage` belongs to `receive`, because the bytes handed to `send`
            // were produced by the kernel's schema and a malformed one there is a defect of the
            // kernel, not a failure of the port.
            Err(IpcError::MalformedMessage) => Outcome::Keep,
        }
    }

    /// The degradation, or `None` when the archive cannot say.
    fn degradation(&self) -> Option<DegradationReport> {
        degradation_now(&self.arbiter, &self.journal)
            .ok()
            .map(|state| DegradationReport {
                vram_exhausted: state.vram_exhausted,
                routing_degraded: state.routing_degraded,
            })
    }

    /// The policy with the two numbers the kernel actually holds -- D20.
    fn policy_report(&self) -> PolicyReport {
        PolicyReport {
            // ⛔ AN EXHAUSTIVE `match` AND NOT A STRING COMPARISON ON `name()`: a third policy must
            // stop the compiler here, not arrive on the wire as one of these two.
            policy: match self.arbiter.policy() {
                VramPolicy::Remote(_) => PolicyName::Remote,
                VramPolicy::Local(_) => PolicyName::Local,
            },
            allocated: self.arbiter.allocated(),
            total: self.parameters.total_vram(),
        }
    }

    /// What the seventh port holds under the layout key, in its three states (decision 35).
    fn layout(&self) -> LayoutState {
        match self.custody.retrieve(CustodyKey::Layout) {
            Ok(Some(package)) => LayoutState::Package(package),
            Ok(None) => LayoutState::Nothing,
            Err(CustodyError::Unavailable) => LayoutState::Unavailable,
        }
    }

    /// The step list: in sub-project 2 these are the registry's invocations.
    ///
    /// ⛔ `None` WHEN THE ARCHIVE CANNOT BE READ, and never a short list. An incomplete list read
    /// as complete is the same silent partial truth `is_granted` and `degradation_now` both refuse
    /// in their own words, arrived at here by the one road nobody guards: a record this build
    /// cannot decode, skipped, would take a step out of the list the gui shows.
    fn step_list(&self) -> Option<Vec<StepSummary>> {
        let entries = self.journal.replay().ok()?;
        let mut list: Vec<StepSummary> = Vec::new();
        for (step, bytes) in entries {
            let Ok(Record::V1(body)) = Record::decode(&bytes) else {
                return None;
            };
            match body.kind() {
                RecordKind::Invocation => {
                    if let Some(Detail::Invocation(detail)) = body.detail() {
                        list.push(StepSummary {
                            step: step.get(),
                            function: String::from(detail.function()),
                            done: false,
                        });
                    }
                }
                // ⚠️ THE OUTCOME OF STEP A, WHICH IS THE STEP THE INVOCATION NOTE SITS ON. Step B's
                // outcome names a step no line of this list carries, so it finds nothing and
                // changes nothing -- which is right, and is why the search is by step and not a
                // count.
                RecordKind::Outcome => {
                    if let Some(line) = list.iter_mut().find(|line| line.step == step.get()) {
                        line.done = true;
                    }
                }
                _ => {}
            }
        }
        Some(list)
    }
}

/// The triple of a permission, on the wire.
///
/// ⛔ AN EXHAUSTIVE `match` AND NOT `Operation::is_write()`, and that is the lesson this repository
/// has already paid for: `is_write` folds EVERY other variant into `false`, so a third operation
/// would cross the wire as a read. The doc of `crate::wire::ipc::Access` records the measurement.
fn triple_of(permission: Permission) -> Triple {
    Triple {
        tool: String::from(permission.tool),
        resource: String::from(permission.resource),
        operation: match permission.operation {
            Operation::Read => Access::Read,
            Operation::Write => Access::Write,
        },
    }
}

/// The policy an untrusted argument names, if it names one.
///
/// ⛔ THE NAMES ARE THE POLICIES' OWN, read through `MakeRoom::name`, and not two literals here:
/// two houses for one pair of strings is what lets them drift, and the gui shows the same words
/// (G15). ⚠️ AND THERE IS NO EXHAUSTIVE `match` ON THE WAY IN -- the input is a `String` the peer
/// chose -- so the closed set lives in this one function and everything else is `None`.
fn policy_named(argument: &str) -> Option<VramPolicy> {
    let remote = VramPolicy::Remote(RemotePolicy);
    let local = VramPolicy::Local(LocalPolicy);
    if argument == remote.name() {
        Some(remote)
    } else if argument == local.name() {
        Some(local)
    } else {
        None
    }
}
