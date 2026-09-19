//! The sub-project 2 campaign: `kernel::serving::serve` under two faults -- a gui that dies on the
//! port, and an archive that falls in the middle of an invocation.
//!
//! ⛔ WHY NOT `simulator::ipc::DyingGui`, WHICH §5 OF THE DESIGN NAMES. Three reasons, each
//! measured on 2026-09-14 while this task was written. It says exactly ONE thing --
//! `IpcMessage::Request` -- and the dispatch drops it on the floor (D5), so no grant is ever
//! issued and the property would compare empty sets. It can pronounce neither `Hello` nor
//! `Approve`, so the second fault is out of reach with it. And it would be MOVED INTO `Core`,
//! which does not expose its transport, so nothing could read it afterwards.
//!
//! ⛔ SO THE WIRE LIVES OUTSIDE THE CORE, behind a `RefCell` the campaign owns, and the port holds
//! a borrow of it. That is the shape of `crates/kernel/tests/serving.rs`, and this is the THIRD
//! place it is written: a `tests/` is a crate of its own and a binary exports nothing, so the
//! first two cannot be imported.
//!
//! ⛔ AND THE FIRST HALF WOULD BE VACUOUS WITHOUT A GRANT PUT IN BY HAND. Sub-project 2 issues NO
//! grant to a client -- `serving.rs` says so of itself -- so a gui that dies here holds nothing,
//! and "the sum came back to the baseline" is green because it never moved. The grant goes in
//! through `Core::grants`, which is the caller that accessor's own doc names.
//!
//! ⛔ AND THE BASELINE IS NOT ZERO, WHICH BUYS BOTH DIRECTIONS AT ONCE. Two holders are in the
//! books before the gui dies: the core's own presentation quota (ADR-0033), which no register ever
//! holds, and a SECOND client that is registered and does not die. A reconciliation that did
//! nothing leaves the sum ABOVE the baseline; one that released every pair it holds instead of the
//! dead client's takes the second client's and leaves it BELOW. One assertion, two mutations.
//!
//! ⚠️ AND THAT IS WHY THE SUM IS THE ORACLE RATHER THAN THE REGISTER: `ClientGrants` exposes
//! `new`, `register` and `on_disconnect` and NOTHING THAT COUNTS WITHOUT RELEASING -- measured --
//! and `grants()` and `arbiter()` are two mutable borrows of the same `&mut Core`, so a campaign
//! outside the crate cannot call `on_disconnect` at all.
//!
//! ⚠️ WHAT THIS CAMPAIGN DOES NOT HOLD, said rather than left to be assumed:
//!
//! - it does NOT re-prove `ClientGrants`. `gui_death_campaign.rs` holds property 3 of §5.7 against
//!   the register directly; what is under test HERE is the ACTIVITY -- that `serve` reads the
//!   death from the port and reconciles, at a turn the seed chooses.
//! - it does NOT reach `Reactor::wait_until` on a real clock: the clock is virtual, so a wait is
//!   an assignment. The wall-clock half is the daemon's, and its own probe declares it.
//! - it does NOT sweep a gui that dies AFTER its window. That road is
//!   `crates/kernel/tests/client_grants.rs::a_disconnect_after_the_window_reports_already_collected`.
//!
//! ⛔ THE CLOCK IS SHARED THROUGH A LOCAL `SharedClock`, AND THAT IS A DECISION -- D34, not a copy
//! nobody noticed (gotcha #49). ⛔ HOW MANY COPIES THERE ARE IS WHAT THE COMMAND SAYS AND NOT THIS
//! LINE: `grep -rn 'struct SharedClock' crates/ gui/ --include='*.rs' | grep -v '///'`. It walks
//! BOTH trees because `gui/` is a sibling of `crates/` that exists already and takes a copy of its
//! own at task 12, and the filter is not decoration: without it the command counts its own
//! citation, which is the defect E63 measured on 2026-09-19 beside the same motif in
//! `crates/daemon/src/main.rs`. ⚠️ THE LIMIT OF THAT FILTER IS WRITTEN OUT ONCE, THERE, and not
//! copied here: a pointer that lives in two documents is one that diverges (gotcha #68).
//!
//! ⛔ AND IT STAYS LOCAL, WHICH IS D34 AND NOT AN OVERSIGHT: `crates/daemon/Cargo.toml` refuses IN
//! WRITING to depend on `simulator`, the reactors these copies wrap are not even the same type,
//! and one home for all of them would want a wrapper generic over `R: Reactor` -- more machinery
//! than the lines it would save (P-59, P-74). ⚠️ ITS TRIGGER, DECLARED RATHER THAN LEFT TO BE
//! DISCOVERED: a copy born inside `simulator` ITSELF, or in a crate that can import it, wrapping
//! the SAME reactor -- on that day the measure is taken again. This file is not that day: a
//! `tests/` is a crate of its own and nothing can import it.

use core::cell::RefCell;
use std::collections::BTreeSet;

use kernel::arbiter::{
    Admission, Arbiter, ArbiterId, ComputeClass, Grant, Mib, Preemption, RemotePolicy,
    ResourceProfile, VramPolicy,
};
use kernel::executor::{Executor, RunError, Sleep};
use kernel::numbering::Progressive;
use kernel::parameters::Parameters;
use kernel::ports::ipc::{ClientId, Ipc, IpcError};
use kernel::ports::reactor::Reactor;
// ⚠️ `EffectClass` IS HERE FOR ONE `match`, AND IT IS NOT IN THE DICTATED LIST: the helper
// `the_doubt_the_one_function_resolves_to` reads `POLICY_FUNCTION.effect` and names the three
// classes, which cannot be spelled without the type. Without this line the file does not compile
// at all (E77 of the plan).
use kernel::record::EffectClass;
use kernel::reconcile::{steps_in_doubt, Resolution};
// ⚠️ `below` LIVES ON THE EXTENSION TRAIT, not on `SeededRng`: `kernel::rng::RngExt`
// carries it and the trait has to be in scope. Its own doc explains why an inherent
// `fn below` on a concrete type would shadow it at the call site.
use kernel::rng::RngExt;
use kernel::serving::{serve, Core, POLICY_FUNCTION};
use kernel::time::{Millis, Monotonic, WallTime};
use kernel::wire::ipc::{build_stamp, Access, Call, IpcMessage, Triple};
use simulator::custody::MemoryCustody;
use simulator::journal::{CrashingJournal, MemoryJournal};
use simulator::reactor::VirtualReactor;
use simulator::rng::SeededRng;

/// The whole machine.
const TOTAL: Mib = Mib::new(8_192);

/// The core's own presentation quota (ADR-0033). ⛔ IT IS NEVER REGISTERED in `ClientGrants`, so no
/// disconnection can reach it whatever the reconciliation does.
const CORE_QUOTA: Mib = Mib::new(1_024);

/// What a second, still-connected client holds. ⛔ IT IS REGISTERED, and it is the witness a
/// mutation can reach: an over-eager reconciliation takes this one and the sum falls BELOW the
/// baseline.
const STANDING_QUOTA: Mib = Mib::new(1_024);

/// What the dying gui holds when it goes. ⛔ PUT IN BY HAND -- see the head of this file.
const GUI_QUOTA: Mib = Mib::new(2_048);

/// What the books hold before the gui dies, and what they must hold again after it is gone.
const BASELINE: Mib = Mib::new(CORE_QUOTA.get() + STANDING_QUOTA.get());

/// The window the standing grants declare -- long enough that nothing here ever collects them.
const FOREVER: Millis = Millis::new(1_000_000);

/// The window the gui's own grant declares.
const GUI_WINDOW: Millis = Millis::new(5_000);

const GUI: ClientId = ClientId::new(1);
const STANDING: ClientId = ClientId::new(2);

/// The tick this campaign delivers. ⛔ IT IS NOT ZERO, AND THAT IS THE POINT: a zero tick makes
/// `nap` behave as a yield (`Sleep::until`'s own rule), and no turn would move the virtual clock.
/// It is `TICK` of `crates/kernel/tests/serving.rs`, and the same value for the same reason.
const TICK: Millis = Millis::new(50);

/// Enough turns for the longest round here, with room to spare. ⛔ FIXED AND VERSIONED WITH THIS
/// FILE (constraint 7 of §11), never drawn from the clock or from an environment variable.
const TURNS: u64 = 64;

/// How many seeds the SHORT campaign sweeps. ⛔ FIXED AND VERSIONED, for `TURNS`' reason. It is the
/// figure the two campaigns beside this one already use.
const SHORT_CAMPAIGN_SEEDS: u64 = 2_000;

/// Everything the gui side holds, and the only thing the probes read. ⛔ IT LIVES OUTSIDE `Core`,
/// which is what makes it readable after the run at all.
struct Wire {
    waiting: Vec<ClientId>,
    /// What each client will say, in order.
    up: Vec<(ClientId, Vec<u8>)>,
    /// What the core said to each client, in order.
    down: Vec<(ClientId, Vec<u8>)>,
    /// The client that will die, and where.
    dying: Option<ClientId>,
    dies_at: u64,
    /// ⛔ `send` AND `receive` ONLY, AND `accept` IS OUT -- `DyingGui`'s own rule and its reason:
    /// the count the death point is drawn against has to count things that CAN report the death,
    /// and `accept` answers `Option<ClientId>` with no error channel.
    operations: u64,
    /// Whether the port ever answered `Disconnected`. ⛔ THE NON-VACUITY ORACLE of the first half.
    death_reported: bool,
}

impl Wire {
    fn new() -> Self {
        Wire {
            waiting: Vec::new(),
            up: Vec::new(),
            down: Vec::new(),
            dying: None,
            dies_at: u64::MAX,
            operations: 0,
            death_reported: false,
        }
    }

    /// The gui connects and then says these things in this order.
    fn arrives(&mut self, client: ClientId, said: &[IpcMessage]) {
        self.waiting.push(client);
        for message in said {
            self.up.push((
                client,
                message.encode().expect("the campaign frames what it sends"),
            ));
        }
    }

    /// ⛔ THE DEATH POINT, AND THE THREE RULES ARE `CrashingJournal::from_seed`'s, cited rather
    /// than reinvented: the seed must be DERIVED from the campaign's (or the campaign explores a
    /// diagonal of the space instead of the space), `operations` must be COUNTED in a run where
    /// nothing dies, and it must not be ZERO.
    fn dies(&mut self, client: ClientId, at: u64) {
        self.dying = Some(client);
        self.dies_at = at;
    }

    /// The twin of `CrashingJournal::without_crash()`: the client is WATCHED and never dies, so
    /// that `operations` counts. ⛔ IT EXISTS FOR THE PRIZE PROBE: `may_operate` counts only the
    /// client it watches, and a run with no watched client counts nothing at all -- measured at the
    /// plan review (R4-20), where the prize probe as first written asserted `0 >= OPERATIONS`.
    fn watched_for_ever(&mut self, client: ClientId) {
        self.dies(client, u64::MAX);
    }

    /// Whether this operation may proceed, MARKING the death when it may not.
    fn may_operate(&mut self, client: ClientId) -> bool {
        if self.dying != Some(client) {
            return true;
        }
        if self.operations >= self.dies_at {
            self.death_reported = true;
            return false;
        }
        self.operations += 1;
        true
    }

    /// Everything the core has said to this client, decoded and taken off the wire.
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
    /// ⚠️ IT DOES NOT CONSULT THE DEATH, deliberately: a listener hands over whoever connected, and
    /// whether that peer is still alive is what the FIRST `receive` finds out. A fake that refused
    /// to accept a dying client would hide the very path the reconciliation exists for.
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
        if !wire.may_operate(client) {
            return Err(IpcError::Disconnected);
        }
        wire.down.push((client, Vec::from(message)));
        Ok(())
    }

    fn receive(&mut self, client: ClientId) -> Result<Option<Vec<u8>>, IpcError> {
        let mut wire = self.wire.borrow_mut();
        if !wire.may_operate(client) {
            return Err(IpcError::Disconnected);
        }
        let at = wire.up.iter().position(|(id, _)| *id == client);
        // ⚠️ `Ok(None)` IS NOT THE DEATH: an idle client and a dead one must not give the same
        // answer, or the core could not poll this port.
        Ok(at.map(|index| wire.up.remove(index).1))
    }
}

/// One clock for the executor AND for the activity -- see the head of this file for why it is
/// local. `VirtualReactor` HOLDS the instant, so two of them would drift.
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

/// The seed the death point is drawn from. ⛔ DERIVED, and a DIFFERENT mixing from the one the
/// executor is seeded with: two draws from the same number move together, and the campaign would
/// explore a DIAGONAL of the space instead of the space (decision D2 of the milestone 4 plan).
fn death_seed(seed: u64) -> u64 {
    seed.wrapping_mul(0xBF58_476D_1CE4_E5B9)
}

/// The seed the crash point is drawn from -- a third mixing, for the same reason.
fn crash_seed(seed: u64) -> u64 {
    seed.wrapping_mul(0x94D0_49BB_1331_11EB)
}

/// A standing grant: one of the holders the books carry before the gui dies.
///
/// ⚠️ `Admission` has no `Debug`, so the `let ... else` is not a style: `expect` does not exist on
/// it. It is `gui_death_campaign.rs`'s helper, and the same reason.
fn standing_grant(
    arbiter: &mut Arbiter,
    name: &'static str,
    reserved: Mib,
    window: Millis,
) -> Grant {
    let profile = ResourceProfile {
        name,
        reserved_vram: reserved,
        compute_class: ComputeClass::Realtime,
        preemption: Preemption::Never,
    };
    let Admission::Granted(grant) = arbiter.admit(&profile, window, Monotonic::ORIGIN) else {
        panic!("the quota {name} of {reserved:?} fits an empty machine of {TOTAL:?}");
    };
    grant
}

/// The approval the gui sends to change the policy.
///
/// ⛔ THE TOOL AND THE RESOURCE ARE BUILT FROM `POLICY_FUNCTION` AND NOT WRITTEN OUT: the approval
/// road COMPARES what comes back against the triple the registry holds, so a literal there would
/// be testing the literal.
///
/// ⚠️ THE OPERATION CANNOT BE, AND SAYING SO IS THE POINT. `Triple::operation` is
/// `wire::ipc::Access` and `POLICY_FUNCTION.permission.operation` is `permission::Operation` --
/// twins by D11 -- and NO conversion exists between them: measured on 2026-09-19 with
/// `grep -rn 'impl From<Operation>' crates/kernel/src/`, which returns nothing. So ONE field of
/// the three is a literal, and a doc claiming all three were built would be the shape `E56`
/// exists for.
///
/// ⛔ AND IT IS `Approve` AND NOT `Invoke`: `Registry::invoke` asks `is_granted` BEFORE opening the
/// step, so an `Invoke` on a triple nobody granted answers `PermissionRequired` having written
/// NOTHING -- zero writes, zero doubt, and a crash point drawn on an empty range, which
/// `CrashingJournal::from_seed` refuses with a `debug_assert!` written for it.
fn the_approval() -> IpcMessage {
    IpcMessage::Approve {
        triple: Triple {
            tool: String::from(POLICY_FUNCTION.permission.tool),
            resource: String::from(POLICY_FUNCTION.permission.resource),
            operation: Access::Write,
        },
        call: Call {
            function: String::from(POLICY_FUNCTION.name),
            argument: String::from("local"),
        },
    }
}

/// What one run of the first half observed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Died {
    /// Where the wire was TOLD to die. ⚠️ NOT AN ORACLE: it feeds the world count and the
    /// diagnostic of the assertion below.
    dies_at: u64,
    /// How many messages the gui had heard before it went -- this is what makes the worlds
    /// distinct, and it is a fact about the ACTIVITY rather than about the fake.
    heard: usize,
    /// What the books held once the activity had stopped.
    allocated: u64,
}

fn one_death(seed: u64) -> Died {
    let parameters = Parameters::new(TURNS, TOTAL, ArbiterId::new(1), TICK);
    let mut arbiter = Arbiter::new(parameters, VramPolicy::Remote(RemotePolicy));

    // ⛔ THE THREE HOLDERS, AND THE CORE'S OWN IS NEVER REGISTERED -- ADR-0033.
    let _core_quota = standing_grant(&mut arbiter, "core-presentation", CORE_QUOTA, FOREVER);
    let standing = standing_grant(&mut arbiter, "gui-standing", STANDING_QUOTA, FOREVER);
    let for_the_gui = standing_grant(&mut arbiter, "gui-request", GUI_QUOTA, GUI_WINDOW);

    let wire = RefCell::new(Wire::new());
    {
        let mut open = wire.borrow_mut();
        open.arrives(GUI, &[IpcMessage::Hello(build_stamp())]);
        open.arrives(STANDING, &[IpcMessage::Hello(build_stamp())]);
        open.dies(GUI, SeededRng::new(death_seed(seed)).below(OPERATIONS));
    }
    let dies_at = wire.borrow().dies_at;

    let mut built = Core::new(
        FakeIpc { wire: &wire },
        MemoryJournal::new(),
        MemoryCustody::new(),
        arbiter,
        Progressive::starting_at(1),
        parameters,
    );
    // ⛔ BY HAND, AND THE HEAD OF THIS FILE SAYS WHY. `Core::grants`' own doc names this caller.
    built.grants().register(STANDING, standing);
    built.grants().register(GUI, for_the_gui);

    let clock = RefCell::new(VirtualReactor::new());
    let core = RefCell::new(built);
    let shared = SharedClock { inner: &clock };
    let sleep = Sleep::new();
    let mut executor = Executor::new(
        SeededRng::new(seed),
        SharedClock { inner: &clock },
        parameters,
        &sleep,
    );
    executor.spawn(serve(&core, &shared, &sleep));
    assert_eq!(
        executor.run(),
        Err(RunError::TurnLimitReached),
        "seed {seed}: `serve` is a loop with no exit"
    );
    drop(executor);
    let mut core = core.into_inner();

    // ⛔ ORACLE ONE -- THE INJECTION FIRED -- ASSERTED PER SEED, which is stronger than any total
    // taken afterwards and is where the red of a "the wire never dies" mutation comes from.
    assert!(
        wire.borrow().death_reported,
        "seed {seed}: the wire was told to die at operation {dies_at} of {OPERATIONS} and the port \
         never answered `Disconnected`, so this run injected nothing"
    );

    // ⛔ AND THE CLIENT IS OFF THE TABLE. `attending` is the only thing `Core` shows of its own
    // bookkeeping, and it says the activity really let go rather than merely stopped talking.
    let attending = core.attending();
    assert!(
        !attending.contains(&GUI),
        "seed {seed}: the gui is gone from the port and still on the core's table: {attending:?}"
    );
    assert!(
        attending.contains(&STANDING),
        "seed {seed}: the client that did not die was dropped with the other one: {attending:?}"
    );

    // ⛔ THE PROPERTY, AND IT CATCHES BOTH MUTATIONS AT ONCE: a reconciliation that did nothing
    // leaves the sum ABOVE the baseline, one that released every pair it holds leaves it BELOW.
    let allocated = core.arbiter().allocated();
    assert_eq!(
        allocated, BASELINE,
        "seed {seed}: after the gui died the sum is {allocated:?} and the baseline was {BASELINE:?}"
    );

    Died {
        dies_at,
        heard: wire.borrow_mut().heard(GUI).len(),
        allocated: allocated.get(),
    }
}

/// What one run of the second half observed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Crashed {
    falls_at: u64,
    /// Whether the fall really happened -- read from the journal the core still holds.
    fell: bool,
    /// The steps the archive leaves in doubt, with their resolution, in order.
    in_doubt: Vec<(u64, Doubt)>,
}

/// `Resolution`, flattened so that a world can be counted.
///
/// ⚠️ A LOCAL ENUM AND NOT `Resolution` ITSELF, and it is not duplication for its own sake:
/// `Resolution` derives neither `Ord` nor `PartialOrd` -- measured on 2026-09-14 -- a `BTreeSet`
/// wants both, and adding a derive to a shipped type for the convenience of a bench is the trade
/// `ports::process` refused when `Grant` was asked for a `Debug`. It is the shape
/// `gui_death_campaign.rs` uses for `Verdict`, and the same reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Doubt {
    AskTheWorld,
    RunAgain,
    SuspendAndAsk,
}

impl From<Resolution> for Doubt {
    fn from(resolution: Resolution) -> Self {
        match resolution {
            Resolution::AskTheWorld => Doubt::AskTheWorld,
            Resolution::RunAgain => Doubt::RunAgain,
            Resolution::SuspendAndAsk => Doubt::SuspendAndAsk,
        }
    }
}

fn one_crash(seed: u64) -> Crashed {
    let parameters = Parameters::new(TURNS, TOTAL, ArbiterId::new(1), TICK);
    let arbiter = Arbiter::new(parameters, VramPolicy::Remote(RemotePolicy));
    let journal = CrashingJournal::from_seed(crash_seed(seed), WRITES_PER_APPROVAL);
    let falls_at = journal.falls_at();

    let wire = RefCell::new(Wire::new());
    wire.borrow_mut()
        .arrives(GUI, &[IpcMessage::Hello(build_stamp()), the_approval()]);

    let built = Core::new(
        FakeIpc { wire: &wire },
        journal,
        MemoryCustody::new(),
        arbiter,
        Progressive::starting_at(1),
        parameters,
    );

    let clock = RefCell::new(VirtualReactor::new());
    let core = RefCell::new(built);
    let shared = SharedClock { inner: &clock };
    let sleep = Sleep::new();
    let mut executor = Executor::new(
        SeededRng::new(seed),
        SharedClock { inner: &clock },
        parameters,
        &sleep,
    );
    executor.spawn(serve(&core, &shared, &sleep));
    assert_eq!(
        executor.run(),
        Err(RunError::TurnLimitReached),
        "seed {seed}: `serve` is a loop with no exit"
    );
    drop(executor);
    let mut core = core.into_inner();

    // ⛔ THE ARCHIVE IS READ THROUGH THE JOURNAL THE CORE STILL HOLDS, and `Core` hands it back by
    // `&mut`. ⚠️ THAT `CrashingJournal::replay` IS NOT BLOCKED BY THE FALL is VERIFIED in step 1,
    // not assumed: `may_write` governs writes, and reading is another thing.
    let fell = core.journal().has_fallen();
    let in_doubt = steps_in_doubt(core.journal())
        .expect("the crashed archive still replays")
        .into_iter()
        .map(|step| (step.step.get(), Doubt::from(step.resolution)))
        .collect();

    Crashed { falls_at, fell, in_doubt }
}

/// How many operations the activity performs on a client it has welcomed.
///
/// ⛔ IT IS THE PREMISE THE DEATH POINT IS DRAWN AGAINST, and it is HELD rather than trusted by
/// `the_activity_operates_on_a_welcomed_client_at_least_this_many_times`. A point drawn past the
/// last operation never fires, and a campaign whose fault never arrives is green for having done
/// nothing (gotcha #17).
///
/// ⛔ HOW ONE REMEDIES A RED HERE: RE-MEASURE AND RE-CHOOSE. Editing this until the bar goes green
/// is gotcha #25. What invalidates it: a change to `TURNS`, to the welcome of sequence 1, or to
/// what the activity sends per turn.
///
/// ✅ MEASURED ON 2026-09-19 AND NOT PREDICTED, which is why it is not the figure the plan carried:
/// the plan said 8, the probe read 37 off its own red. Six of them are the first turn -- one
/// `receive` and the five messages of the welcome -- and the other thirty-one are one `receive` per
/// later round, because a round of `serve` costs TWO executor turns: the poll that arms the nap,
/// and the poll after the timer fires.
const OPERATIONS: u64 = 37;

/// How many writes an approved invocation performs when nothing falls.
///
/// ⛔ COUNTED IN A RUN WITHOUT A CRASH, which is `CrashingJournal::from_seed`'s own instruction: a
/// count taken from a run that already crashed stops at the crash and would draw every later point
/// out of reach. Held by `an_approval_without_a_crash_writes_this_many_records`.
///
/// ⛔ AND IT IS THE APPROVAL ROAD, not the plain invocation: `Approve` writes the permission note
/// on step A as well (decision 21 of the north star).
///
/// ✅ MEASURED ON 2026-09-19 with `assert_eq!`, and the plan's prediction happened to match: step
/// A's intent, the invocation note, the permission note, the three of `Arbiter::set_policy` on step
/// B, and step A's outcome.
const WRITES_PER_APPROVAL: u64 = 7;

/// ⛔ THE FIRST PRIZE, COUNTED IN A RUN WHERE NOBODY DIES -- the twin of `without_crash()`.
#[test]
fn the_activity_operates_on_a_welcomed_client_at_least_this_many_times() {
    let parameters = Parameters::new(TURNS, TOTAL, ArbiterId::new(1), TICK);
    let mut arbiter = Arbiter::new(parameters, VramPolicy::Remote(RemotePolicy));
    let _core_quota = standing_grant(&mut arbiter, "core-presentation", CORE_QUOTA, FOREVER);
    let standing = standing_grant(&mut arbiter, "gui-standing", STANDING_QUOTA, FOREVER);
    let for_the_gui = standing_grant(&mut arbiter, "gui-request", GUI_QUOTA, GUI_WINDOW);

    let wire = RefCell::new(Wire::new());
    {
        let mut open = wire.borrow_mut();
        open.arrives(GUI, &[IpcMessage::Hello(build_stamp())]);
        open.arrives(STANDING, &[IpcMessage::Hello(build_stamp())]);
        // ⛔ WATCHED AND NEVER KILLED: `may_operate` counts only the client it watches, so a run
        // with no watched client counts nothing at all (R4-20).
        open.watched_for_ever(GUI);
    }

    let mut built = Core::new(
        FakeIpc { wire: &wire },
        MemoryJournal::new(),
        MemoryCustody::new(),
        arbiter,
        Progressive::starting_at(1),
        parameters,
    );
    built.grants().register(STANDING, standing);
    built.grants().register(GUI, for_the_gui);

    let clock = RefCell::new(VirtualReactor::new());
    let core = RefCell::new(built);
    let shared = SharedClock { inner: &clock };
    let sleep = Sleep::new();
    let mut executor = Executor::new(
        SeededRng::new(0),
        SharedClock { inner: &clock },
        parameters,
        &sleep,
    );
    executor.spawn(serve(&core, &shared, &sleep));
    assert_eq!(
        executor.run(),
        Err(RunError::TurnLimitReached),
        "`serve` is a loop with no exit"
    );
    drop(executor);

    let done = wire.borrow().operations;
    assert!(
        done >= OPERATIONS,
        "the activity performed {done} operations on a welcomed client while `OPERATIONS` says \
         {OPERATIONS}: every death point drawn from the tail of that range can never fire, and \
         those seeds would sweep nothing at all"
    );
}

/// ⛔ THE SECOND PRIZE, COUNTED IN A RUN WITHOUT A CRASH -- `C7a` for this campaign.
#[test]
fn an_approval_without_a_crash_writes_this_many_records() {
    let parameters = Parameters::new(TURNS, TOTAL, ArbiterId::new(1), TICK);
    let arbiter = Arbiter::new(parameters, VramPolicy::Remote(RemotePolicy));

    let wire = RefCell::new(Wire::new());
    wire.borrow_mut()
        .arrives(GUI, &[IpcMessage::Hello(build_stamp()), the_approval()]);

    let built = Core::new(
        FakeIpc { wire: &wire },
        CrashingJournal::without_crash(),
        MemoryCustody::new(),
        arbiter,
        Progressive::starting_at(1),
        parameters,
    );

    let clock = RefCell::new(VirtualReactor::new());
    let core = RefCell::new(built);
    let shared = SharedClock { inner: &clock };
    let sleep = Sleep::new();
    let mut executor = Executor::new(
        SeededRng::new(0),
        SharedClock { inner: &clock },
        parameters,
        &sleep,
    );
    executor.spawn(serve(&core, &shared, &sleep));
    assert_eq!(
        executor.run(),
        Err(RunError::TurnLimitReached),
        "`serve` is a loop with no exit"
    );
    drop(executor);
    let mut core = core.into_inner();

    let written = core.journal().writes_done();
    assert_eq!(
        written, WRITES_PER_APPROVAL,
        "the approval road wrote {written} records and `WRITES_PER_APPROVAL` says \
         {WRITES_PER_APPROVAL}: the crash point is drawn below that number, so a scenario that \
         writes fewer leaves the tail of the range firing on nothing"
    );

    // ⛔ THE SECOND DIRECTION, and it is a different claim: a doubt with NO crash would mean the
    // write-ahead discipline itself is broken, and every seed below would be measuring that
    // instead of the fault this campaign injects.
    let in_doubt = steps_in_doubt(core.journal()).expect("the journal replays");
    assert!(
        in_doubt.is_empty(),
        "nothing fell and a step is still in doubt, so what the crash campaign measures is not \
         the crash: {in_doubt:?}"
    );
}

/// How many DISTINCT worlds the gui-death scenario can produce at all.
///
/// ⛔ IT IS THE NON-VACUITY CONSTANT AND A CHANGE DETECTOR on the shape of the scenario, the
/// posture of `EXPECTED_WORLDS` in `gui_death_campaign.rs`. Without it, "this many seeds see the
/// whole space" would be a sentence in a comment that no run can contradict.
///
/// ⛔ HOW ONE REMEDIES A RED HERE: RE-MEASURE THE SPACE AND RE-CHOOSE BOTH NUMBERS. Editing this
/// until the bar goes green is gotcha #25. What invalidates it, named so that a red reads as a
/// decision rather than as a defect: a change to `OPERATIONS`, to `TURNS`, to the welcome of
/// sequence 1, or to `SHORT_CAMPAIGN_SEEDS`.
///
/// ✅ MEASURED ON 2026-09-19 by running the sweep and reading the number off the red, not
/// predicted: the plan carried no figure here at all. It comes out EQUAL to `OPERATIONS`, and that
/// is the coverage claim itself -- every one of the death points is drawn by some seed, and the
/// other two fields add no dimension of their own: `allocated` is the baseline on every seed
/// because that is what the property asserts, and `heard` is a function of where the death landed.
const EXPECTED_DEATH_WORLDS: usize = 37;

/// How many DISTINCT worlds the journal-crash scenario can produce at all. Same posture, same
/// remedy as `EXPECTED_DEATH_WORLDS`.
///
/// ⛔ WHAT INVALIDATES IT: a change to `WRITES_PER_APPROVAL`, to what `Registry::invoke` or
/// `Arbiter::set_policy` write, or to `SHORT_CAMPAIGN_SEEDS`.
///
/// ✅ MEASURED ON 2026-09-19, read off the red of the sweep. It comes out EQUAL to
/// `WRITES_PER_APPROVAL` for the same reason its twin equals `OPERATIONS`: every crash point is
/// drawn by some seed, and the doubt set each one leaves is a function of where the archive fell.
const EXPECTED_CRASH_WORLDS: usize = 7;

/// The resolution the ONE function of this sub-project declares.
///
/// ⛔ READ FROM `POLICY_FUNCTION.effect` AND NOT WRITTEN OUT, or the probe would pin the literal
/// instead of the class. `kernel::reconcile::resolution_of` is private -- measured on 2026-09-19 --
/// so the table it holds is restated here in one `match` rather than imported.
///
/// ⚠️ THIS `match` DOES NOT DECIDE ANYTHING, and neither does `From<Resolution> for Doubt`: both
/// CONVERT so that a world can be counted. Row 24 of milestone 6 -- "`Resolution` is settled by no
/// `match`" -- stays open, and its closer is the first consumer that branches on it.
fn the_doubt_the_one_function_resolves_to() -> Doubt {
    match POLICY_FUNCTION.effect {
        EffectClass::Verifiable => Doubt::AskTheWorld,
        EffectClass::Idempotent => Doubt::RunAgain,
        EffectClass::Unrepeatable => Doubt::SuspendAndAsk,
    }
}

/// ⛔ PROPERTY ONE: the activity reads the death from the port and reconciles. Asserted per seed
/// inside `one_death`; what is left here is the NON-VACUITY of the sweep.
#[test]
fn a_gui_that_dies_under_the_activity_gives_its_grant_back() {
    let started = std::time::Instant::now();
    let mut distinct = BTreeSet::new();
    for seed in 0..SHORT_CAMPAIGN_SEEDS {
        distinct.insert(one_death(seed));
    }
    let elapsed = started.elapsed();

    assert!(
        distinct.len() > 1,
        "every seed produced the SAME world: the two mixings are moving together, so this \
         campaign is one run repeated {SHORT_CAMPAIGN_SEEDS} times"
    );
    assert_eq!(
        distinct.len(),
        EXPECTED_DEATH_WORLDS,
        "the campaign saw {} of the {EXPECTED_DEATH_WORLDS} worlds this scenario can produce -- \
         either the scenario changed shape or {SHORT_CAMPAIGN_SEEDS} seeds no longer reach the end \
         of the space, and BOTH numbers must be re-measured rather than this one edited",
        distinct.len()
    );
    println!(
        "DST serving, gui death: {} distinct worlds over {SHORT_CAMPAIGN_SEEDS} seeds, {elapsed:?}",
        distinct.len()
    );
}

/// ⛔ PROPERTY TWO: an approval whose archive falls leaves the step in doubt WITH ITS CLASS.
///
/// ⛔ AND THE NON-VACUITY IS TWO CLAIMS AND NOT ONE: that every fall actually fired, and that at
/// least one seed really left a step in doubt -- a crash on the FIRST write writes nothing at all,
/// and a sweep that only ever fell there would compare EMPTY SETS, which is the lesson milestone 4
/// learned three times, each time after closing the previous one.
#[test]
fn an_approval_that_crashes_leaves_the_step_in_doubt_with_its_class() {
    let started = std::time::Instant::now();
    let mut distinct = BTreeSet::new();
    let mut left_something_in_doubt = 0u64;

    for seed in 0..SHORT_CAMPAIGN_SEEDS {
        let observed = one_crash(seed);
        assert!(
            observed.fell,
            "seed {seed}: the archive was told to fall at write {} of {WRITES_PER_APPROVAL} and \
             never did, so this run injected nothing",
            observed.falls_at
        );
        // ⛔ THE CLASS, AND THIS IS ROW 24 OF MILESTONE 6 BEING ASSERTED RATHER THAN DECIDED: the
        // open item says `Resolution` is settled by no `match`, and this campaign adds none -- it
        // asserts the value the declared class produces. ⚠️ AND THE EXPECTED VALUE IS READ FROM
        // `POLICY_FUNCTION.effect` rather than written out, or the probe would pin the literal.
        for (step, resolution) in &observed.in_doubt {
            assert_eq!(
                *resolution,
                the_doubt_the_one_function_resolves_to(),
                "seed {seed}: step {step} is in doubt as {resolution:?}, which is not what the \
                 class this road declares resolves to"
            );
        }
        if !observed.in_doubt.is_empty() {
            left_something_in_doubt += 1;
        }
        distinct.insert(observed);
    }
    let elapsed = started.elapsed();

    assert!(
        left_something_in_doubt > 0,
        "on no seed did the archive leave a step in doubt: every one of the \
         {SHORT_CAMPAIGN_SEEDS} falls landed on the very first write, so nothing was ever opened \
         and every doubt set compared was empty"
    );
    assert_eq!(
        distinct.len(),
        EXPECTED_CRASH_WORLDS,
        "the campaign saw {} of the {EXPECTED_CRASH_WORLDS} worlds this scenario can produce -- \
         re-measure BOTH numbers rather than editing this one",
        distinct.len()
    );
    println!(
        "DST serving, journal crash: {left_something_in_doubt} of {SHORT_CAMPAIGN_SEEDS} seeds \
         left a step in doubt, {} distinct worlds, {elapsed:?}",
        distinct.len()
    );
}
