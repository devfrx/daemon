//! The activity that serves the gui, driven FROM OUTSIDE THE CRATE against an in-memory wire.
//!
//! ⛔ WHAT THIS BENCH IS FOR, AND IT IS NOT ONLY THE DISPATCH: it is the first caller to build
//! `serving::serve` from outside, which is what `gui/fake-core` must do at task 12 (§7 of the
//! sub-project 2 design). A signature that could only be assembled inside `kernel` would not be
//! found out any other way.
//!
//! ⛔ `Err(RunError::TurnLimitReached)` IS THE EXPECTED ANSWER OF EVERY ROUND. `serve` is a loop
//! with no exit -- a core that stopped serving would be the defect -- so the run ends when the
//! turns run out. What the probes read is THE WIRE and THE BOOKS, never the value of `run`.
//!
//! ⚠️ AND THE CLOCK IS SHARED THROUGH `SharedClock` rather than handed out twice, which is the
//! shape `crates/simulator/tests/arbiter_campaign.rs` already uses: `VirtualReactor` HOLDS the
//! instant, so two of them would drift and the activity would read an origin the executor had
//! long left. ⛔ THIS IS THE SECOND COPY OF THAT WRAPPER IN THE REPOSITORY -- declared rather than
//! discovered (gotcha #49): the third and fourth arrive with tasks 9 and 10 (D29, D34), the fifth
//! with task 12 (P-74). Where it should live is DECIDED, by D34: it stays local, because the
//! daemon refuses to depend on `simulator` and a common home would serve three callers out of
//! four (P-59).

use core::cell::{Cell, RefCell};

use kernel::arbiter::{
    Admission, Arbiter, ArbiterId, ComputeClass, MakeRoom, Mib, Preemption, RemotePolicy,
    ResourceProfile, VramPolicy,
};
use kernel::executor::{nap, Executor, RunError, Sleep};
use kernel::numbering::Progressive;
use kernel::parameters::Parameters;
use kernel::ports::custody::{Custody, CustodyError, CustodyKey};
use kernel::ports::ipc::{ClientId, Ipc, IpcError};
use kernel::ports::journal::{Journal, StepId};
use kernel::ports::reactor::Reactor;
use kernel::record::{EffectClass, Record, RecordKind, RecordV1, RoutingDetail, Trust};
use kernel::serving::{serve, Core, POLICY_FUNCTION};
use kernel::time::{Millis, Monotonic, WallTime};
use kernel::wire::ipc::{
    build_stamp, stamp_set, Access, BuildStamp, Call, GrantRequest, IpcMessage, LayoutState,
    PolicyName, Protection, Triple,
};
use simulator::custody::MemoryCustody;
use simulator::journal::MemoryJournal;
use simulator::reactor::VirtualReactor;
use simulator::rng::SeededRng;

/// ⚠️ A LITERAL OF THIS BENCH. Nothing here admits anything through the dispatch -- `Request` is
/// not served (D5) -- but `Parameters` carries every delivered value positionally, and §2.8.2
/// rule 2 forbids the kernel to name a default.
const TOTAL: Mib = Mib::new(8_192);

/// The tick this bench delivers. ⛔ IT IS NOT ZERO, AND THAT IS THE POINT: a zero tick makes `nap`
/// behave as a yield (`Sleep::until`'s own rule), and no round would ever move the virtual clock
/// or reach `Reactor::wait_until`.
const TICK: Millis = Millis::new(50);

/// Enough turns for the longest round here, with room to spare. ⛔ FIXED AND VERSIONED WITH THIS
/// FILE (constraint 7 of §11), never drawn from the clock or from the environment.
const TURNS: u64 = 64;

const GUI: ClientId = ClientId::new(1);
const OTHER: ClientId = ClientId::new(2);

/// The gui side of the wire, and the only thing the probes read.
struct Wire {
    waiting: Vec<ClientId>,
    up: Vec<(ClientId, Vec<u8>)>,
    down: Vec<(ClientId, Vec<u8>)>,
    gone: Vec<ClientId>,
}

impl Wire {
    const fn new() -> Self {
        Wire {
            waiting: Vec::new(),
            up: Vec::new(),
            down: Vec::new(),
            gone: Vec::new(),
        }
    }

    /// The gui connects, and then says these things in this order.
    fn arrives(&mut self, client: ClientId, said: &[IpcMessage]) {
        self.waiting.push(client);
        for message in said {
            self.up
                .push((client, message.encode().expect("the bench frames what it sends")));
        }
    }

    /// This client is dead: every `send` and `receive` on it answers `Disconnected`.
    fn dead(&mut self, client: ClientId) {
        self.gone.push(client);
    }

    /// Everything the core has said to this client, in order, decoded and taken off the wire.
    fn heard(&mut self, client: ClientId) -> Vec<IpcMessage> {
        let mut out = Vec::new();
        let mut kept = Vec::new();
        for (id, bytes) in self.down.drain(..) {
            if id == client {
                out.push(IpcMessage::decode(&bytes).expect("the core frames what it sends"));
            } else {
                kept.push((id, bytes));
            }
        }
        self.down = kept;
        out
    }
}

/// The `ipc` port over that wire.
struct FakeIpc<'a> {
    wire: &'a RefCell<Wire>,
}

impl Ipc for FakeIpc<'_> {
    /// ⚠️ IT DOES NOT CONSULT `gone`, deliberately: a listener hands over whoever connected, and
    /// whether that peer is still alive is what the FIRST `receive` finds out. A fake that refused
    /// to accept a dead client would hide the very path `on_disconnect` exists for.
    fn accept(&mut self) -> Option<ClientId> {
        let mut wire = self.wire.borrow_mut();
        if wire.waiting.is_empty() {
            None
        } else {
            Some(wire.waiting.remove(0))
        }
    }

    fn send(&mut self, client: ClientId, message: &[u8]) -> Result<(), IpcError> {
        let mut wire = self.wire.borrow_mut();
        if wire.gone.contains(&client) {
            return Err(IpcError::Disconnected);
        }
        wire.down.push((client, Vec::from(message)));
        Ok(())
    }

    fn receive(&mut self, client: ClientId) -> Result<Option<Vec<u8>>, IpcError> {
        let mut wire = self.wire.borrow_mut();
        if wire.gone.contains(&client) {
            return Err(IpcError::Disconnected);
        }
        let at = wire.up.iter().position(|(id, _)| *id == client);
        Ok(at.map(|index| wire.up.remove(index).1))
    }
}

/// One clock for the executor AND for the activity. ⚠️ See the note at the top of this file.
struct SharedClock<'a> {
    inner: &'a RefCell<VirtualReactor>,
}

impl Reactor for SharedClock<'_> {
    fn now(&self) -> Monotonic {
        self.inner.borrow().now()
    }

    fn wall_time(&self) -> WallTime {
        self.inner.borrow().wall_time()
    }

    fn wait_until(&mut self, deadline: Monotonic) -> Option<Monotonic> {
        self.inner.borrow_mut().wait_until(deadline)
    }
}

/// The seventh port of this bench: `MemoryCustody` underneath, plus a tap that makes `keep` refuse.
///
/// ⛔ IT EXISTS FOR ONE PROBE, `save_layout_that_the_port_refuses_comes_back_with_the_old_package`:
/// decision 13 says a failed write comes back as the OLD package, and `MemoryCustody` does not know
/// how to fail. The tap is a `Cell` so that the `before` closure can arm it through `&C`, which is
/// all `Core::custody` hands out. Measured at the plan review (R9a-9, 2026-09-15): without it the
/// second direction of decision 13 was a sentence in a comment and not a probe.
struct BenchCustody {
    inner: MemoryCustody,
    keeps_left: Cell<u32>,
}

impl BenchCustody {
    fn new() -> Self {
        BenchCustody {
            inner: MemoryCustody::new(),
            keeps_left: Cell::new(u32::MAX),
        }
    }
}

impl Custody for BenchCustody {
    fn keep(&mut self, key: CustodyKey, bytes: &[u8]) -> Result<(), CustodyError> {
        if self.keeps_left.get() == 0 {
            return Err(CustodyError::Unavailable);
        }
        self.keeps_left.set(self.keeps_left.get() - 1);
        self.inner.keep(key, bytes)
    }

    fn retrieve(&self, key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError> {
        self.inner.retrieve(key)
    }
}

type BenchCore<'b> = Core<FakeIpc<'b>, MemoryJournal, BenchCustody>;

struct Bench {
    wire: RefCell<Wire>,
    clock: RefCell<VirtualReactor>,
}

impl Bench {
    fn new() -> Self {
        Bench {
            wire: RefCell::new(Wire::new()),
            clock: RefCell::new(VirtualReactor::new()),
        }
    }

    /// One round with the serving activity alone.
    fn round<'b>(
        &'b self,
        before: impl FnOnce(&mut BenchCore<'b>),
        after: impl FnOnce(&mut BenchCore<'b>),
    ) {
        self.round_with_tap(before, false, after);
    }

    /// One round, optionally with a SECOND activity on the same cell.
    ///
    /// ⚠️ THE DECLARATION ORDER IS LOAD-BEARING and swapping two lines does not compile: the
    /// executor borrows `sleep`, `core` and `clock` for its whole life, and locals drop in reverse
    /// order of declaration.
    fn round_with_tap<'b>(
        &'b self,
        before: impl FnOnce(&mut BenchCore<'b>),
        tapped: bool,
        after: impl FnOnce(&mut BenchCore<'b>),
    ) {
        let parameters = Parameters::new(TURNS, TOTAL, ArbiterId::new(1), TICK);
        let mut built = Core::new(
            FakeIpc { wire: &self.wire },
            MemoryJournal::new(),
            BenchCustody::new(),
            Arbiter::new(parameters, VramPolicy::Remote(RemotePolicy)),
            Progressive::starting_at(1),
            parameters,
        );
        before(&mut built);

        let core = RefCell::new(built);
        let clock = SharedClock { inner: &self.clock };
        let sleep = Sleep::new();
        let tapped_once = Cell::new(false);
        let mut executor = Executor::new(
            SeededRng::new(1),
            SharedClock { inner: &self.clock },
            parameters,
            &sleep,
        );
        executor.spawn(serve(&core, &clock, &sleep));
        if tapped {
            executor.spawn(degrade_once(&core, &clock, &sleep, &tapped_once));
        }

        assert_eq!(
            executor.run(),
            Err(RunError::TurnLimitReached),
            "`serve` is a loop with no exit: the run ends when the turns run out"
        );
        drop(executor);
        if tapped {
            // ⛔ THE NON-VACUITY OF THE TAPPED ROUND. Without it, a tap that never woke up would
            // make `a_degradation_written_by_a_second_activity_reaches_the_gui` prove nothing at
            // all -- it would just be the welcome, arriving as usual.
            assert!(tapped_once.get(), "the second activity must have run");
        }
        after(&mut core.into_inner());
    }

    fn heard(&self, client: ClientId) -> Vec<IpcMessage> {
        self.wire.borrow_mut().heard(client)
    }
}

/// A SECOND activity on the same cell: it sleeps one tick, then writes a degraded routing into the
/// core's journal.
///
/// ⛔ IT IS THE TAP OF `gui/fake-core` IN MINIATURE (§7 of the sub-project 2 design, the `degrade`
/// word): the write reaches the journal WITHOUT passing through the dispatch, which is the whole
/// reason D23 re-reads the degradation every turn instead of after its own writes.
async fn degrade_once<'b>(
    core: &RefCell<BenchCore<'b>>,
    clock: &SharedClock<'_>,
    sleep: &Sleep,
    ran: &Cell<bool>,
) {
    nap(sleep, clock.now().saturating_add(Millis::new(120))).await;
    let record = Record::V1(RecordV1::routing(
        EffectClass::Idempotent,
        Trust::Instruction,
        Vec::new(),
        "a degraded routing, written by somebody who is not the dispatch",
        RoutingDetail::new("a-model", 2, true),
    ))
    .encode();
    // ⚠️ ONE STATEMENT, SO THE BORROW DIES WITH IT. Holding it across a suspension is what the
    // whole shape of `serve` forbids, and this activity plays by the same rule.
    core.borrow_mut()
        .journal()
        .intent(StepId::new(900), &record)
        .expect("the memory journal writes");
    ran.set(true);
}

/// A stamp that is NOT this build's.
///
/// ⛔ IT CANNOT BE MINTED, and that is the whole value of `BuildStamp`: its doc refuses a public
/// constructor, because "a stamp anyone can mint from any number is a stamp that proves nothing".
/// So it is TAKEN from the canonical set, whose `Hello` carries a fixed literal. ⚠️ AND THE
/// ASSERTION IS NOT DECORATION: the day that literal ever coincided with a real stamp, this probe
/// would be testing the ACCEPTING path while claiming to test the refusing one.
fn a_stamp_that_is_not_ours() -> BuildStamp {
    let Some(IpcMessage::Hello(stamp)) = stamp_set().into_iter().next() else {
        panic!("the canonical set opens with `Hello`")
    };
    assert_ne!(
        stamp,
        build_stamp(),
        "the canonical `Hello` must not carry this build's own stamp"
    );
    stamp
}

/// The call the gui sends to change the policy, with the argument the one function reads.
fn switch_to_local() -> Call {
    Call {
        function: String::from(POLICY_FUNCTION.name),
        argument: String::from("local"),
    }
}

/// The triple of the one function, as the core itself would put it on the wire.
fn the_triple() -> Triple {
    Triple {
        tool: String::from(POLICY_FUNCTION.permission.tool),
        resource: String::from(POLICY_FUNCTION.permission.resource),
        operation: Access::Write,
    }
}

#[test]
fn the_welcome_is_the_five_messages_of_sequence_one() {
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(GUI, &[IpcMessage::Hello(build_stamp())]);

    bench.round(|_| {}, |_| {});

    let heard = bench.heard(GUI);
    // ⛔ THE ORDER IS THE ASSERTION AND NOT A BONUS: sequence 1 of the north star fixes it, and a
    // gui told the policy before it was admitted would be drawing a core it is not attached to.
    assert!(
        matches!(heard.first(), Some(IpcMessage::Accepted(Protection::AsSystemAccount))),
        "the welcome opens with `Accepted`: {heard:?}"
    );
    assert!(
        matches!(heard.get(1), Some(IpcMessage::Degradation(_))),
        "then the degradation: {heard:?}"
    );
    assert!(
        matches!(
            heard.get(2),
            Some(IpcMessage::Policy(report))
                if report.policy == PolicyName::Remote
                    && report.total == TOTAL
                    && report.allocated == Mib::ZERO
        ),
        "then the policy: the default of ADR-0006, the delivered total, empty books: {heard:?}"
    );
    assert!(
        matches!(heard.get(3), Some(IpcMessage::Layout(LayoutState::Nothing))),
        "then the layout, and an archive that opened and holds nothing is `Nothing`: {heard:?}"
    );
    assert!(
        matches!(heard.get(4), Some(IpcMessage::Steps(steps)) if steps.is_empty()),
        "then the step list, empty on a fresh journal: {heard:?}"
    );
    // ⛔ THE SECOND DIRECTION, and without it the five above pass over a core that never stops
    // talking: §6.1.4 says the core sends only what CHANGES, and nothing changed after the welcome.
    assert_eq!(
        heard.len(),
        5,
        "after the welcome the core sends only what changed: {heard:?}"
    );
}

#[test]
fn a_stale_stamp_gets_the_expected_one_and_then_the_core_stops_listening() {
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(
        GUI,
        // ⛔ THE SECOND MESSAGE IS THE PROBE. "The core closes" is not an operation of the port
        // (decision 22): what it does is STOP LISTENING, and only a message sent AFTER the refusal
        // tells that apart from "it answered and then had nothing more to say".
        &[
            IpcMessage::Hello(a_stamp_that_is_not_ours()),
            IpcMessage::Invoke(switch_to_local()),
        ],
    );

    bench.round(
        |_| {},
        |core| {
            assert!(
                core.journal().replay().expect("the memory journal replays").is_empty(),
                "a refused gui writes nothing, and its later words are not read"
            );
        },
    );

    let heard = bench.heard(GUI);
    assert_eq!(
        heard,
        vec![IpcMessage::StaleBuild(build_stamp())],
        "a stale gui is told the EXPECTED stamp, and then nothing at all: {heard:?}"
    );
}

#[test]
fn a_client_that_has_not_shaken_hands_is_served_nothing_and_keeps_its_place() {
    // ⛔ THE HANDSHAKE IS A GATE AND NOT A GREETING -- §5 of the sub-project 2 design, "the first
    // message must be `Hello`". Removing the gate reddens THIS PROBE ALONE, in the whole
    // workspace -- measured on 2026-09-18, and that is why it exists.
    // MEASURED before the gate existed (E46 of the plan): such a peer moved the policy to `local`,
    // its package reached the seventh port, and the journal held the six records of the round --
    // `Permission` among them -- while `attending()` stayed EMPTY.
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(
        OTHER,
        &[
            IpcMessage::Approve { triple: the_triple(), call: switch_to_local() },
            IpcMessage::SaveLayout(vec![7, 7, 7]),
            // ⛔ THE THIRD MESSAGE IS THE SECOND DIRECTION: refusing is not disconnecting. Had the
            // two above cost this client its place, the `Hello` would reach nobody and no welcome
            // would come back. It is the only way to tell "kept" from "dropped" from out here,
            // because `attending()` cannot see a client that is still in `Greeting` -- E43's
            // lesson, used on purpose this time instead of walked into.
            IpcMessage::Hello(build_stamp()),
        ],
    );

    bench.round(
        |_| {},
        |core| {
            assert_eq!(
                core.arbiter().policy().name(),
                "remote",
                "a peer that has not shaken hands must not reach the effect"
            );
            assert!(
                core.journal().replay().expect("the memory journal replays").is_empty(),
                "and it must not write a record either"
            );
            assert_eq!(
                core.custody().retrieve(CustodyKey::Layout),
                Ok(None),
                "and nothing of its must reach the seventh port"
            );
            assert!(
                core.attending().contains(&OTHER),
                "and it is at the table: the `Hello` it sent LAST was still read"
            );
        },
    );

    let heard = bench.heard(OTHER);
    // The welcome, and not one word about the two that were refused -- the refusal is silent, like
    // the dispatch's other three roads in
    // `a_word_the_dispatch_does_not_know_is_refused_without_a_word`.
    assert!(
        matches!(heard.first(), Some(IpcMessage::Accepted(_))),
        "what comes back is the welcome, opening where it always opens: {heard:?}"
    );
    assert_eq!(heard.len(), 5, "the welcome, and nothing else at all: {heard:?}");
}

#[test]
fn a_request_reaches_neither_the_arbiter_nor_the_journal() {
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(
        GUI,
        &[
            IpcMessage::Hello(build_stamp()),
            IpcMessage::Request(GrantRequest {
                reserved_vram: Mib::new(1_024),
                compute_class: ComputeClass::Batch,
                preemption: Preemption::Never,
            }),
        ],
    );

    bench.round(
        |_| {},
        |core| {
            // ⛔ THE DIRECTION THAT DECIDES, AND IT IS NOT "no `Verdict` came back". D5 says the
            // branch builds NO `ResourceProfile` at all, so what has to be shown is that no value
            // of the peer reached the arbiter: the books are untouched, and untouched means zero
            // because this bench reserves nothing of its own.
            assert_eq!(
                core.arbiter().allocated(),
                Mib::ZERO,
                "an unserved request must not reach the arbiter's books"
            );
            assert!(
                core.journal().replay().expect("the memory journal replays").is_empty(),
                "and it must not open a step either"
            );
        },
    );

    let heard = bench.heard(GUI);
    assert_eq!(heard.len(), 5, "the welcome, and not one word more: {heard:?}");
}

#[test]
fn an_invoke_without_the_permission_asks_for_the_triple_and_writes_nothing() {
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(
        GUI,
        &[
            IpcMessage::Hello(build_stamp()),
            IpcMessage::Invoke(switch_to_local()),
        ],
    );

    bench.round(
        |_| {},
        |core| {
            assert_eq!(
                core.arbiter().policy().name(),
                "remote",
                "a refused invocation must not run the effect"
            );
            assert!(
                core.journal().replay().expect("the memory journal replays").is_empty(),
                "the registry refuses BEFORE it opens step A: nothing is written"
            );
        },
    );

    let heard = bench.heard(GUI);
    assert_eq!(
        heard.get(5),
        Some(&IpcMessage::PermissionRequired(the_triple())),
        "the gui is asked for the triple the REGISTRY holds: {heard:?}"
    );
}

#[test]
fn an_approve_whose_triple_does_not_match_the_registry_is_refused() {
    let bench = Bench::new();
    let mut lie = the_triple();
    // ⛔ THE STRINGS CAME BACK FROM THE PEER AND ARE UNTRUSTED (ADR-0014), which is what the doc of
    // `wire::ipc::Triple` asks this consumer for by name. A core that believed them would write a
    // permission for a triple nobody was ever asked about.
    lie.resource = String::from("everything");

    bench.wire.borrow_mut().arrives(
        GUI,
        &[
            IpcMessage::Hello(build_stamp()),
            IpcMessage::Approve { triple: lie, call: switch_to_local() },
        ],
    );

    bench.round(
        |_| {},
        |core| {
            assert_eq!(
                core.arbiter().policy().name(),
                "remote",
                "a triple that does not match must not unlock the effect"
            );
            assert!(
                core.journal().replay().expect("the memory journal replays").is_empty(),
                "and it must not write a permission either"
            );
        },
    );

    let heard = bench.heard(GUI);
    assert_eq!(heard.len(), 5, "the welcome, and nothing else: {heard:?}");
}

#[test]
fn an_approve_changes_the_policy_and_the_gui_is_told() {
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(
        GUI,
        &[
            IpcMessage::Hello(build_stamp()),
            IpcMessage::Approve { triple: the_triple(), call: switch_to_local() },
        ],
    );

    bench.round(
        |_| {},
        |core| {
            assert_eq!(
                core.arbiter().policy().name(),
                "local",
                "the whole point of the round: the effect really ran"
            );
            let kinds: Vec<RecordKind> = core
                .journal()
                .replay()
                .expect("the memory journal replays")
                .iter()
                .map(|(_, bytes)| match Record::decode(bytes).expect("this build wrote these") {
                    Record::V1(body) => body.kind(),
                })
                .collect();
            // ⛔ THE ORDER OF DECISION 21, READ BACK OUT OF THE ARCHIVE: step A's intent, the
            // invocation note, the permission note, then step B's own intent and outcome, then A's
            // outcome. A round that wrote the permission FIRST would be putting a note on a step
            // nobody had opened, which is what `permission::grant`'s doc forbids.
            assert_eq!(
                kinds,
                vec![
                    RecordKind::Intent,
                    RecordKind::Invocation,
                    RecordKind::Permission,
                    RecordKind::Intent,
                    RecordKind::Outcome,
                    RecordKind::Outcome,
                ],
                "the journal must read as decision 21 dictates"
            );
        },
    );

    let heard = bench.heard(GUI);
    assert!(
        matches!(heard.get(5), Some(IpcMessage::Policy(report)) if report.policy == PolicyName::Local),
        "the gui is told the NEW policy: {heard:?}"
    );
    assert!(
        matches!(heard.get(6), Some(IpcMessage::Steps(steps)) if steps.len() == 1 && steps[0].done),
        "and the step list, with the invocation closed: {heard:?}"
    );
}

#[test]
fn a_client_that_dies_gives_its_grant_back() {
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(GUI, &[IpcMessage::Hello(build_stamp())]);
    // ⚠️ IT CONNECTS AND SAYS NOTHING, AND IT IS ALREADY DEAD. That is not a contrived case: it is
    // `ClientGrants::on_disconnect`'s own words -- "A gui may die before it ever asked — it may die
    // before it was ever accepted —".
    bench.wire.borrow_mut().arrives(OTHER, &[]);
    bench.wire.borrow_mut().dead(OTHER);

    bench.round(
        |core| {
            // ⛔ THE GRANT IS PUT IN BY HAND, AND THE REASON IS D5 ITSELF: nothing in the dispatch
            // of sub-project 2 issues one, because `Request` is not served. The wiring still has to
            // be right -- ADR-0033 says the core notices FROM THE IPC DISCONNECTION and
            // reconciles -- and this is the only way to hold it until the 3D pillar brings the
            // writer.
            //
            // ⛔ AND THERE ARE TWO OF THEM, OF DIFFERENT SIZES, WHICH IS WHAT MAKES THE `after`
            // BLOCK DECIDE. With one grant "gave back only the dead client's" and "gave back
            // everything it holds" leave the SAME number in the books, so a sweeping
            // reconciliation would pass -- measured, and it is rilievo I-1. With 1024 held by the
            // client that dies and 512 by the one that lives, the books answer three different
            // numbers: 512 when the right pair goes, 1024 when the wrong one does, 0 when both do.
            let grant_of = |core: &mut BenchCore<'_>, name, vram| {
                let profile = ResourceProfile {
                    name,
                    reserved_vram: Mib::new(vram),
                    compute_class: ComputeClass::Batch,
                    preemption: Preemption::Never,
                };
                let Admission::Granted(grant) =
                    core.arbiter()
                        .admit(&profile, Millis::new(1_000_000), Monotonic::ORIGIN)
                else {
                    panic!("this bench's machine is big enough for these two grants")
                };
                grant
            };
            let dying = grant_of(core, "a-client-of-the-bench", 1_024);
            let living = grant_of(core, "the-gui-of-the-bench", 512);
            core.grants().register(OTHER, dying);
            core.grants().register(GUI, living);
            assert_eq!(
                core.arbiter().allocated(),
                Mib::new(1_536),
                "the books hold both before the round, or the round proves nothing"
            );
        },
        |core| {
            // ⛔ 512 AND NOT ZERO, AND THAT IS THE WHOLE ASSERTION: the grant of the client that
            // DIED comes back and the one of the client that LIVES does not. A reconciliation that
            // swept every pair it holds would answer 0, and one that released the wrong pair would
            // answer 1024 -- neither is this number.
            assert_eq!(
                core.arbiter().allocated(),
                Mib::new(512),
                "only the grant of the client that died comes back to the books"
            );
            // ⛔ AND THE TABLE IS HELD BY THE ASSERTION ABOVE, NOT BY A SECOND ONE. An assertion on
            // `attending()` would be VACUOUS here, which is why there is none: that accessor
            // filters on `Stage::Attending`, and `OTHER` never reaches it -- it says nothing,
            // `dead(OTHER)` kills it before the round, and `FakeIpc::receive` looks at `gone`
            // first -- so `!contains(&OTHER)` would be green even if `forget` never ran at all.
            // What holds the table is the line above: a grant returns to the books ONLY by way of
            // `forget`, which is the same line that takes the client off the table. A better
            // assertion would want `clients` exposed, and "an API item with no caller in this
            // repository is deleted" (`crates/kernel/src/boundary.rs`).
        },
    );

    // ⚠️ AND THE CLIENT THAT DID NOT DIE IS STILL SERVED, which is what this last line says and
    // ALL it says. ⛔ IT IS NOT THE WITNESS AGAINST A SWEEPING RECONCILIATION -- that is the
    // assertion on the books above, which is where the two pairs of different sizes earn their
    // keep. This line was claimed to be that witness and was not: with one grant registered a
    // sweep left the same number behind, every probe of this file stayed green, and the red came
    // out in `crates/kernel/tests/client_grants.rs` and `crates/simulator/tests/gui_death_campaign.rs`
    // instead (rilievo I-1, measured 2026-09-18).
    let heard = bench.heard(GUI);
    assert_eq!(heard.len(), 5, "the living gui got its whole welcome: {heard:?}");
}

#[test]
fn save_layout_comes_back_with_what_the_port_holds() {
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(
        GUI,
        &[
            IpcMessage::Hello(build_stamp()),
            IpcMessage::SaveLayout(vec![7, 7, 7]),
        ],
    );

    bench.round(
        |_| {},
        |core| {
            assert_eq!(
                core.custody().retrieve(CustodyKey::Layout),
                Ok(Some(vec![7, 7, 7])),
                "the package really reached the seventh port"
            );
        },
    );

    let heard = bench.heard(GUI);
    // ⛔ DECISION 13: the core sends back WHAT IT HOLDS after every write, never an error variant,
    // so a failed write comes back as the OLD package and the gui sees it by comparing. The
    // failed write is `save_layout_that_the_port_refuses_comes_back_with_the_old_package`.
    assert_eq!(
        heard.get(5),
        Some(&IpcMessage::Layout(LayoutState::Package(vec![7, 7, 7]))),
        "and what comes back is what the port holds: {heard:?}"
    );
}

#[test]
fn save_layout_that_the_port_refuses_comes_back_with_the_old_package() {
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(
        GUI,
        &[
            IpcMessage::Hello(build_stamp()),
            IpcMessage::SaveLayout(vec![7, 7, 7]),
            IpcMessage::SaveLayout(vec![9, 9, 9]),
        ],
    );

    bench.round(
        // ⛔ THE SECOND DIRECTION OF DECISION 13: one `keep` is allowed, the second is refused.
        |core| core.custody().keeps_left.set(1),
        |core| {
            assert_eq!(
                core.custody().retrieve(CustodyKey::Layout),
                Ok(Some(vec![7, 7, 7])),
                "the refused write must not have reached the port"
            );
        },
    );

    let heard = bench.heard(GUI);
    // The core answers with what it HOLDS, both times: no error variant, and the second answer is
    // the OLD package, which is how the gui learns that the save did not stick.
    assert_eq!(
        heard.get(5),
        Some(&IpcMessage::Layout(LayoutState::Package(vec![7, 7, 7]))),
        "the first save comes back as itself: {heard:?}"
    );
    assert_eq!(
        heard.get(6),
        Some(&IpcMessage::Layout(LayoutState::Package(vec![7, 7, 7]))),
        "the refused save comes back as the OLD package: {heard:?}"
    );
    assert_eq!(heard.len(), 7, "and nothing else is said about it: {heard:?}");
}

#[test]
fn a_word_the_dispatch_does_not_know_is_refused_without_a_word() {
    // ⛔ THE THREE SILENT ROADS OF THE DISPATCH, WHICH NO OTHER PROBE HERE WALKS (R3-13 of the plan
    // review, 2026-09-15): an argument that names no policy, a function nobody registered, and
    // bytes that are no frame at all. Each is refused WITHOUT A WORD AND WITHOUT A RECORD --
    // untrusted content informs, it never authorises (ADR-0014) -- and the client is KEPT: a bad
    // frame is not a dead peer. Without this probe a `policy_named` that fell back to
    // `Some(remote)` for any unknown text would pass every other probe in this file.
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(
        GUI,
        &[
            IpcMessage::Hello(build_stamp()),
            IpcMessage::Invoke(Call {
                function: String::from(POLICY_FUNCTION.name),
                argument: String::from("gpu"),
            }),
            IpcMessage::Invoke(Call {
                function: String::from("frobnicate"),
                argument: String::from("local"),
            }),
        ],
    );
    // Three bytes that are no frame at all, pushed past `arrives` because `arrives` frames.
    bench.wire.borrow_mut().up.push((GUI, vec![0xff, 0xff, 0xff]));

    bench.round(
        |_| {},
        |core| {
            assert_eq!(
                core.arbiter().policy().name(),
                "remote",
                "a word the dispatch does not know must not move the policy"
            );
            assert!(
                core.journal().replay().expect("the memory journal replays").is_empty(),
                "and none of the three writes a record"
            );
            assert!(
                core.attending().contains(&GUI),
                "and the client that said them is still at the table"
            );
        },
    );

    let heard = bench.heard(GUI);
    assert_eq!(
        heard.len(),
        5,
        "the welcome, and not one word about any of the three: {heard:?}"
    );
}

#[test]
fn an_unchanged_degradation_is_not_resent() {
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(GUI, &[IpcMessage::Hello(build_stamp())]);

    bench.round(|_| {}, |_| {});

    let heard = bench.heard(GUI);
    // ⛔ THE FIRST DIRECTION OF D23: one `Degradation`, in the welcome, and the sweep of every one
    // of the turns that follow sends nothing, because nothing changed. Without this the probe
    // below would pass over a core that shouts the same state on every tick.
    assert_eq!(
        heard
            .iter()
            .filter(|message| matches!(message, IpcMessage::Degradation(_)))
            .count(),
        1,
        "an unchanged degradation is not resent: {heard:?}"
    );
}

#[test]
fn a_degradation_written_by_a_second_activity_reaches_the_gui() {
    let bench = Bench::new();
    bench.wire.borrow_mut().arrives(GUI, &[IpcMessage::Hello(build_stamp())]);

    bench.round_with_tap(|_| {}, true, |_| {});

    let heard = bench.heard(GUI);
    // ⛔ THE SECOND DIRECTION OF D23, AND IT IS THE ONE THAT DECIDES THE RULE. The degraded routing
    // is written by ANOTHER ACTIVITY -- which is exactly the `degrade` word of `gui/fake-core`'s
    // tap (§7) -- so a core that only re-read the degradation after ITS OWN writes would never see
    // it, and this count would stay at one.
    let reports: Vec<bool> = heard
        .iter()
        .filter_map(|message| match message {
            IpcMessage::Degradation(report) => Some(report.routing_degraded),
            _ => None,
        })
        .collect();
    assert_eq!(
        reports,
        vec![false, true],
        "the welcome says clean, and the sweep then says degraded: {heard:?}"
    );
}
