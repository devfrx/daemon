//! The production wiring: it mounts `platform`, opens the journal, builds the arbiter with the
//! two permanent grants of ADR-0033, builds the executor and runs it.
//!
//! # It PRODUCES the parameters, the kernel only RECEIVES them
//!
//! The kernel reads no configuration — §2.8, ADR-0034 — so somebody has to resolve the
//! values and DELIVER them at construction. In production that somebody is this binary.
//!
//! In this sub-project the resolved values are LITERALS RIGHT HERE: constraint 11 of §11.
//! That is the correct boundary and not a shortcut — the parameter store arrives later with
//! an interface of its own, and until it does the value has to be chosen somewhere. What
//! makes it acceptable is that it is WRITTEN DOWN rather than hidden: a literal in `daemon`
//! is visible and can vary from one call site to the next, whereas the same number written
//! inside the kernel would appear in no list and could not be made to vary at all
//! (gotcha #28).
//!
//! # ⛔ It does NOT mount `simulator`
//!
//! `Cargo.toml` does not depend on it, with the reason written where the line is missing.
//! The daemon is the PRODUCTION wiring: it mounts `platform`. In simulation the wiring is
//! the test bench's job, and the bench receives the resolved parameters exactly as this file
//! produces them.
//!
//! # What a run with NO activities proves
//!
//! Nothing is spawned, and that is not a placeholder: there is no work to do yet. What the
//! run claims is THE WHOLE GRAPH ASSEMBLES — the real `Rng`, the real `Reactor`, the real
//! `Journal` on the disk, the arbiter holding the two permanent grants of ADR-0033, the
//! delivered `Parameters` and the executor's `Sleep` cell fit together, and the executor runs
//! to completion.
//!
//! ⚠️ RECALL OF 2026-08-21, MILESTONE 5 TASK 10. This heading said "it is the ONE claim this
//! binary can make today", and this task is what made that false: the start-up gained failures
//! it can say out loud and did not have — the journal will not open, and either permanent quota
//! of ADR-0033 does not get in — and each of them is a claim of its own. The sentence is
//! REWRITTEN and not answered beside itself, which is finding A-2 of this project's audit.
//!
//! ⛔ RECALL OF 2026-09-19, SUB-PROJECT 2, TASK 9 — THE HEADING ABOVE AND WHAT IT CLAIMS ARE FALSE
//! FROM THIS TASK, and they are dated rather than answered underneath themselves, which is the
//! same finding A-2 the paragraph above names. An activity IS spawned now —
//! `kernel::serving::serve`, from `run_the_graph` — so there is work to do, there is no run "with
//! NO activities", and the executor CANNOT run to completion, because `serve` is a `loop` with no
//! exit and `Executor::run` is `while !self.tasks.is_empty()`: a run that ENDED would mean the
//! core stopped serving.
//!
//! What the run claims INSTEAD: the whole graph assembles — the real `SequentialRng`, the one
//! real `SystemReactor` seen from two places, the real `FileJournal`, the real `LocalSocketIpc`,
//! the layout archive open or not, the arbiter holding the two permanent grants of ADR-0033 on
//! the policy the journal names — and the serving activity TAKES THE TURNS it is given, which is
//! what a peer hearing its welcome is the only witness of.

use std::cell::RefCell;
use std::path::Path;

use kernel::arbiter::{
    self, Admission, Arbiter, ArbiterId, ComputeClass, Grant, Mib, PolicyError, Preemption,
    RemotePolicy, ResourceProfile, VramPolicy,
};
use kernel::executor::{Executor, RunError, Sleep};
use kernel::numbering::{self, Progressive};
use kernel::parameters::Parameters;
use kernel::ports::custody::{Custody, CustodyError, CustodyKey};
use kernel::ports::journal::JournalError;
use kernel::ports::reactor::Reactor;
use kernel::serving::{self, Core};
use kernel::time::{Millis, Monotonic, WallTime};
use platform::custody::FileCustody;
use platform::ipc::LocalSocketIpc;
use platform::journal::{FileJournal, OpenError};
use platform::reactor::SystemReactor;
use platform::rng::SequentialRng;

/// How many turns the executor may take before it declares a block (§3.2.1).
///
/// # ⛔ It is a COUNT OF TURNS, not a ceiling on wall-clock time
///
/// The distinction is not pedantry, and an earlier version of this comment got it wrong by
/// asserting that a turn "performs no I/O". IT CAN. A turn is one iteration of
/// `Executor::run`, and that iteration may contain `reactor.wait_until` — which on
/// `SystemReactor`, the reactor THIS FILE wires, is a real `std::thread::sleep`. So the wall
/// time of a turn is whatever that turn waits for, and no number chosen here bounds it.
///
/// Measured on this graph, which is what settles it:
///
/// | Case                                          | Cost                   |
/// |-----------------------------------------------|------------------------|
/// | the whole ceiling spent polling, no waits     | 100 000 turns ≈ 15 ms  |
/// | ONE run whose turns contain a 2000 ms wait    | 2.0004 s               |
///
/// # What the value therefore buys, stated exactly
///
/// - ABOVE anything legitimate. The reference scenario — three activities of four steps
///   each — takes NINE turns, so the limit clears it by FOUR orders of magnitude.
/// - It catches a block that DOES NOT WAIT in far less than a second: the top row is the
///   whole ceiling in about fifteen milliseconds. Those are the two failures
///   `RunError::TurnLimitReached` documents — an activity that yields for ever, and one that
///   re-registers an elapsed deadline. Both spin, so both land there.
/// - ⚠️ AND IT DOES NOT BOUND THE CLOCK for an activity that keeps going back to sleep on
///   deadlines still in the FUTURE. That run is not spinning, it is waiting; it still ends,
///   because the turns still run out, but at whatever wall time its waits add up to. The
///   guarantee is TERMINATION, not promptness.
///
/// # Where the nine comes from
///
/// 📌 MEASURED, not carried over. The plan said "fewer than forty", and that figure was
/// never checked — an expectation written before the measurement is a hypothesis, which is
/// gotcha #15, named at the top of `crates/kernel/tests/executor_determinism.rs` for this
/// exact reason. The instrument is the limit itself: `run` fails as soon as `turns > limit`,
/// so the SMALLEST limit that still returns `Ok(())` IS the count. It is nine, and the same
/// nine on all 200 seeds of that file — the seed changes the ORDER within a turn, not the
/// NUMBER of turns. Eight fails, which is what makes nine a boundary rather than a guess.
/// ⛔ RECALL OF 2026-09-19, SUB-PROJECT 2, TASK 9 — THE VALUE IS `u64::MAX`, AND THE TABLE ABOVE NOW
/// DESCRIBES A RUN THIS BINARY NO LONGER MAKES. Until this task the graph had NO activity, so the
/// ceiling bounded nothing that existed; from here it carries `kernel::serving::serve`, which is a
/// `loop` with no exit. A finite ceiling would therefore be a CLOCK ON THE DAEMON'S LIFE — the core
/// would stop serving after so many turns, for no reason a user could name — and that is the one
/// thing this number must not be. Decision A of §5 of the sub-project 2 design, delegated and taken.
///
/// ⚠️ WHAT IS GIVEN UP, SAID PLAINLY: with no ceiling, an activity that spins can no longer be
/// caught HERE. `RunError::TurnLimitReached` remains reachable in the benches, which hand their own
/// finite limits through `Parameters` (ADR-0034), and in the sub-project 2 campaign (task 10). In production the
/// guard against a run that goes nowhere is an OS watchdog, which §5 assigns to sub-project 10 along
/// with the clean shutdown. Declared, not pinned (gotcha #73).
///
/// ⚠️ AND THE SATURATION IS THE PRECEDENT `FOR_EVER` SET, in this same file: `u64::MAX` is not
/// "never", it is "further than this run can reach", and the arithmetic is the same one the comment
/// beside `FOR_EVER` measured — about 584 million years at one turn per millisecond, and turns are
/// far faster than that.
const EXECUTOR_TURN_LIMIT: u64 = u64::MAX;

/// How much VRAM the machine has, in whole MiB (§5.1).
///
/// # ⛔ It is DECLARED here, because the kernel has no way to ask
///
/// Querying the GPU is an OS call, which I3 forbids the kernel, and none of the port families
/// supplies hardware capacity. So the total is a DELIVERED parameter like every other, and
/// this binary is the somebody who resolves it — constraint 11 of §11, the same boundary
/// `EXECUTOR_TURN_LIMIT` sits on.
///
/// # Where the number comes from
///
/// 16384 MiB is the single RTX 5080 this project is built around — the resource constraint
/// that ADR-0005 calls dominant. ⚠️ IT IS NOT MEASURED FROM THE DEVICE and cannot be from
/// here: a wrong total produces OVER-ADMISSION, which §5.1 declares as the cost of
/// delivering it rather than asking for it. That makes a systematic discrepancy a defect of
/// this line, visible and variable, instead of an incident nobody can locate.
///
/// # What reads it, since this task
///
/// ⚠️ RECALL OF 2026-08-21, MILESTONE 5 TASK 10 — THIS PARAGRAPH SAID THE OPPOSITE, and the
/// sentence it replaces was a DEADLINE IN PROSE that this task is what makes come due
/// (gotcha #77). It read "NOTHING IN THIS BINARY READS IT YET beyond handing it over. No
/// arbiter is wired here: the production wiring of the arbiter, with the two permanent grants
/// of §4.3, is a later task of milestone 5". That task is THIS one. The whole paragraph is
/// rewritten rather than contradicted underneath itself — finding A-2 — because the half that
/// stayed true ("the value is CHOSEN IN `daemon` and travels through `Parameters`") reads as
/// an excuse when it is left standing beside a denial.
///
/// It travels through `Parameters` and reaches TWO consumers now: `Executor::new`, which
/// carries it without reading it, and `Arbiter::new`, which uses it as the ceiling every
/// admission is measured against. There is no second road for it to arrive by.
const TOTAL_VRAM: Mib = Mib::new(16_384);

/// Which arbiter this binary runs.
///
/// ⛔ A LITERAL, ON THE SAME BOUNDARY AS `TOTAL_VRAM` and for the same reason: §6.1.3 forbids
/// the kernel to MINT an identifier, so the value has to be chosen HERE and delivered through
/// `Parameters`. There is nothing to derive it from — this process runs exactly one arbiter,
/// and the identity exists so `Arbiter::release` can tell a grant of its own from a grant of
/// somebody else's arbiter, not so anyone can enumerate them.
///
/// ⚠️ THE VALUE IS ARBITRARY AND THAT IS NOT A GAP: what `release` compares is EQUALITY, so
/// any value works as long as two arbiters that are meant to differ are given different ones.
/// The day a second arbiter is wired here, it gets a second literal — and that is precisely
/// the friction §2.8.5 wants a parameter to have.
const ARBITER_ID: ArbiterId = ArbiterId::new(0);

/// Where the journal file lives, in production.
///
/// ⛔ A LITERAL, ON THE SAME BOUNDARY AS `TOTAL_VRAM` and for the same reason: the value has
/// to be chosen somewhere until the parameter store arrives, and a literal in `daemon` is
/// visible and can be varied. ⚠️ AND IT IS RELATIVE TO THE WORKING DIRECTORY, which is
/// declared rather than defended: where a per-user data directory should be is a decision no
/// ADR has taken, and inventing one here would be that decision taken by whoever typed the
/// path. Every test passes its OWN path, so nothing in the gate depends on this constant.
const JOURNAL_PATH: &str = "journal.redb";

/// Where the LAYOUT ARCHIVE lives, in production — the seventh port's store.
///
/// ⛔ A SECOND FILE AND NOT A SECOND TABLE IN THE JOURNAL, and the difference is ADR-0022: the
/// journal is authoritative state and is BACKED UP AND ENCRYPTED; a window layout is neither. It is
/// also what lets the layout archive fail to open WITHOUT stopping the start-up (decision 35),
/// which sharing a file with the journal would make impossible.
///
/// ⚠️ RELATIVE TO THE WORKING DIRECTORY, exactly as `JOURNAL_PATH` is and declared for the same
/// reason: where a per-user data directory belongs is a decision no ADR has taken, and inventing one
/// here would be that decision taken by whoever typed the path. Every probe passes its OWN path.
const LAYOUT_PATH: &str = "layout.redb";

/// The name the GUI knocks on.
///
/// ⛔ IT IS PROTOCOL AND NOT A TUNING KNOB, and that is why it is named in this task's closing
/// criterion: THE SHELL must open the SAME name, and nothing in the gate couples the two ends.
/// ⛔ AND THE SHELL IS NOT A TASK OF THIS PLAN — the fake core binds THE SAME NAME, as a copy
/// (`gui/fake-core/src/main.rs`, task 12, whose closing criterion compares the two literals — D45),
/// the SPA never touches a socket, and §8 puts the end-to-end run in the shell "outside today's gate". The day the
/// shell exists, the coupling is a probe; until then this literal is the whole of the agreement, and
/// a fact of protocol living in one house with no index naming it is how a fact of protocol rots in
/// silence.
///
/// ⛔ A NAMESPACED NAME AND NOT A PATH: `LocalSocketIpc::bound` resolves it through
/// `to_ns_name::<GenericNamespaced>()`, which is what makes ONE string work as a named pipe on
/// Windows and as a local socket on Linux — the two systems of ADR-0002 behind one line.
///
/// ⚠️ THE `harness-` PREFIX IS THE ONE THE BENCHES ALREADY USE, so that a stray socket left behind
/// by a crash is recognisable as ours by name alone.
const SOCKET_NAME: &str = "harness-core";

/// The longest body the core will buffer from a peer (D9).
///
/// ⛔ DELIVERED RATHER THAN INVENTED, like `TOTAL_VRAM` and for the same reason (ADR-0034): the
/// transport must not name a default, so the number is chosen HERE, where it is visible and can be
/// varied. What it buys is written beside `LocalSocketIpc::max_body`: without a cap a peer declaring
/// four gibibytes would be buffered for ever, because any four bytes are a valid length.
///
/// ⚠️ THE SIZE IS NOT MEASURED AND IS DECLARED AS SUCH. The largest message that climbs this wire is
/// the layout package — `toJSON()` of `dockview` plus the active view — and no such package exists
/// yet to measure. What the value has to be is COMFORTABLY ABOVE that and FAR BELOW a memory
/// problem, and a mebibyte is both. ⛔ ITS TRIGGER IS THE FIRST PACKAGE REFUSED: a `SaveLayout` that
/// comes back `MalformedMessage` is this line being too small, not a broken peer, and the remedy is
/// this literal rather than a loosening of the transport.
const MAX_BODY: usize = 1024 * 1024;

/// How long the serving activity sleeps between turns (§5, ADR-0034).
///
/// ⛔ IT EXISTS BECAUSE THE REACTOR HAS NO I/O READINESS, which is entry 5 of §9 of the sub-project 2
/// design, confirmed as-is by the owner on 2026-09-09: nothing can wake the core when a byte
/// arrives, so the core LOOKS, on a rhythm. The tick is that rhythm, and it is the WORST-CASE
/// LATENCY between the GUI speaking and the core hearing.
///
/// ⚠️ NOT MEASURED, AND DECLARED AS SUCH. What picks it is a trade with no measurement behind it
/// yet: larger wastes nothing and makes the GUI feel slow, smaller costs a syscall per turn for
/// latency nobody can perceive. Sixteen milliseconds is one frame at sixty hertz — the interval the
/// GUI itself is already paced by, so the core cannot be the slower half of a round trip.
/// ⛔ ITS TRIGGER IS THE FIRST PERCEIVED-LATENCY MEASUREMENT on the assembled shell, which this
/// plan does not build (P-53): until somebody watches a round trip, any number here is an argument.
///
/// ⚠️ AND THE BENCHES DO NOT INHERIT IT: the tick is delivered through `Parameters`, so a probe
/// hands its own — zero, where a turn must not wait (`Sleep::until`'s rule). That is what keeps the
/// production value out of the gate's wall clock.
const GUI_TICK: Millis = Millis::new(16);

/// The audio quota, and the presentation quota of ADR-0033.
///
/// ⛔ THEY ARE NOT SUBTRACTIONS, THEY ARE TWO PERMANENT GRANTS, and the difference is I2. A
/// quota subtracted from the budget WITHOUT A HOLDER leaves I2 false for that consumer --
/// "the subtraction is not an exemption" (ADR-0005, gotcha #4) -- whereas a grant HAS a
/// holder by construction. ADR-0033 says it in those words: "the core REQUESTS a permanent,
/// non-preemptible presentation grant at start-up".
const AUDIO_QUOTA: Mib = Mib::new(1_024);
const PRESENTATION_QUOTA: Mib = Mib::new(768);

/// ⛔ "PERMANENT" IS NOT A TYPE -- it is "nobody calls release". The window is saturated on
/// purpose: `Monotonic::saturating_add` does not wrap, so there is no special case inside the
/// arbiter for a grant that never expires, and none is wanted.
///
/// ⚠️ AND IT IS NOT LITERALLY "NEVER", WHICH WAS MEASURED RATHER THAN REASONED. This comment
/// was dictated saying "a deadline this far out NEVER ARRIVES", and it does arrive, at exactly
/// one instant: `Monotonic::ORIGIN.saturating_add(FOR_EVER)` saturates AT `u64::MAX`, and
/// `Arbiter::collect_expired` compares `expires_at <= now`, so a sweep at the last
/// representable millisecond of the axis collects both quotas. ✅ MEASURED, not deduced --
/// `allocated()` comes back `Mib(0)` instead of `Mib(1792)` there. 📌 What the saturation
/// buys is unchanged: about 584 million years of monotonic time, on a clock that starts at
/// process start-up.
const FOR_EVER: Millis = Millis::new(u64::MAX);

/// The two profiles the composition root reserves at start-up.
///
/// ⛔ THE ARBITER DOES NOT KNOW THESE ARE CALLED "audio" AND "presentation". It sees two
/// permanent grants like any other -- which is ADR-0001: no capability has privileged access.
/// Wiring the two names inside the arbiter would be two special cases in a mechanism that has
/// to be even-handed.
///
/// ⚠️ THEY ARE CONSTANTS AND NOT TWO LITERALS INSIDE THE WIRING, and the reason is that the
/// probes name them: a profile built twice is a profile that can drift, and the probe would
/// then be checking its own copy.
const AUDIO_RESERVATION: ResourceProfile = ResourceProfile {
    name: "audio-reserved",
    reserved_vram: AUDIO_QUOTA,
    compute_class: ComputeClass::Realtime,
    preemption: Preemption::Never,
};

const PRESENTATION_RESERVATION: ResourceProfile = ResourceProfile {
    name: "presentation-reserved",
    reserved_vram: PRESENTATION_QUOTA,
    compute_class: ComputeClass::Realtime,
    preemption: Preemption::Never,
};

/// The one `SystemReactor`, seen from two places.
///
/// ⛔ IT EXISTS BECAUSE THE TWO SIGNATURES DISAGREE, not for tidiness: `Executor::new` takes the
/// reactor BY VALUE and `kernel::serving::serve` takes a clock BY REFERENCE, so one of the two has
/// to be a light copy over a single owner.
///
/// ⛔ AND BUILDING TWO `SystemReactor`s WOULD COMPILE AND BE WRONG. That type carries an `origin`
/// anchored to `Instant::now()` INSIDE its constructor — its own doc calls it "THE ONLY ORIGIN IT
/// HAS" — so two instances answer two different `Monotonic`s for the same real instant. The
/// deadlines `serve` computes would then not be comparable with the ones the executor waits on:
/// two independent truths about one fact, which is the shape of `E25`, at the layer that decides
/// when an activity wakes.
///
/// ⚠️ THE SHAPE IS REPEATED ON PURPOSE AND THE DUPLICATION IS DECLARED RATHER THAN HIDDEN, and
/// WHERE THE COPIES ARE IS WHAT THE COMMAND SAYS AND NOT THIS LINE —
/// `grep -rn 'struct SharedClock' crates/ gui/ --include='*.rs' | grep -v '///'`. ⛔ IT WALKS
/// BOTH TREES ON PURPOSE: `gui/` is a sibling of `crates/`, it exists already, and task 12 of
/// this plan puts a copy of its own in `gui/fake-core`, which a command looking only under
/// `crates/` could never see. ⛔ AND THE SECOND HALF IS WHAT MAKES IT BLIND TO THE LINE ABOVE:
/// without it the command COUNTS ITS OWN CITATION and answers one more than there are —
/// measured on 2026-09-19, four against three. ⚠️ ITS LIMIT, DECLARED RATHER THAN HIDDEN AND
/// MEASURED RATHER THAN GUESSED: the filter drops any line CONTAINING `///`, so it hides this
/// citation and every other that quotes the CURED motif, whatever comment marker that one sits
/// behind -- the `'///'` travels inside the quoted text. What it would still count is a line
/// quoting the BLIND form, without the filter, from outside a `///` comment. None of the copies can be
/// imported in any case — a `tests/` file is a crate of its own, a binary exports nothing, and
/// `gui/fake-core` is outside this workspace altogether.
///
/// ⛔ WHETHER IT SHOULD RISE INTO `simulator` IS DECIDED, AND THE ANSWER IS NO: D34 keeps it
/// local (P-59, P-74). This crate refuses to depend on `simulator` — its manifest says so — so a
/// common home there could not serve the callers that live outside it, and the reactors the
/// copies wrap are not even the same type. One home for all of them would want a wrapper generic
/// over `R: Reactor`, which is more machinery than the lines it would save.
struct SharedClock<'a> {
    inner: &'a RefCell<SystemReactor>,
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

/// The layout archive, open or not — and the core starts either way (decision 35 of §8).
///
/// ⛔ IT LIVES HERE AND NOT IN THE PORT, and that is the whole decision. `FileCustody::open` hands
/// back a `Result` and `Core::new` wants a `Custody` BY VALUE, so between the two there is a value
/// missing, and the only place that knows the core must start anyway is the composition root. The
/// two roads not taken: making `open` always hand back a custody would change a task already
/// written and throw away the name of the failure that `OpenError` carries; a SECOND variant of
/// `CustodyError` would give the port a word for a state that belongs to the root, and it is the
/// variant with no caller that `CustodyError`'s own doc refuses.
///
/// ⛔ AND NOTHING NEW IS NEEDED IN THE ACTIVITY: `kernel::serving` already turns
/// `Err(CustodyError::Unavailable)` into `LayoutState::Unavailable`, so decision 35 falls out of
/// this type without a line anywhere else — which is what §8 predicted in those words, "no new
/// operation in the port".
///
/// ⚠️ DECLARED RESIDUAL — WHY THE ARCHIVE WOULD NOT OPEN DOES NOT REACH THE OPERATOR. `OpenError`
/// is dropped here rather than carried, because a field only `Debug` reads is flagged dead (the
/// paragraph beside `main` measured exactly that), and printing from `run_the_graph` would take on
/// the job that same function's doc gives to `main` alone. ⛔ ITS TRIGGER IS THE FIRST DIAGNOSTIC
/// CHANNEL the daemon grows — sub-project 10, with the clean shutdown — and until then the operator
/// sees the effect, "layout unavailable", and not the cause.
enum MaybeCustody {
    Open(FileCustody),
    Unavailable,
}

impl MaybeCustody {
    /// ⛔ IT SWALLOWS THE ERROR ON PURPOSE, which is the sentence above turned into code: a layout
    /// archive that will not open is NOT authoritative state (I1), so it does not stop a start-up
    /// the way the journal does.
    fn open(path: &Path) -> Self {
        match FileCustody::open(path) {
            Ok(custody) => MaybeCustody::Open(custody),
            Err(_) => MaybeCustody::Unavailable,
        }
    }
}

impl Custody for MaybeCustody {
    fn keep(&mut self, key: CustodyKey, bytes: &[u8]) -> Result<(), CustodyError> {
        match self {
            MaybeCustody::Open(custody) => custody.keep(key, bytes),
            MaybeCustody::Unavailable => Err(CustodyError::Unavailable),
        }
    }

    fn retrieve(&self, key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError> {
        match self {
            MaybeCustody::Open(custody) => custody.retrieve(key),
            MaybeCustody::Unavailable => Err(CustodyError::Unavailable),
        }
    }
}

/// Why the start-up did not complete.
///
/// ⛔ THREE VARIANTS, AND THE THIRD IS THE ONE THAT CLOSES `E41`. An impossible VRAM
/// configuration stopped announcing itself the day the arbiter grew queues: the second
/// permanent quota comes back `Queued` instead of `Refused`, and nobody will ever serve it,
/// because releasing a permanent grant is exactly what nobody does. The arbiter cannot repair
/// that -- "permanence is not a type, it is nobody calls release", so it cannot tell a ticket
/// that WILL be served from one that never will -- and a ticket that waits for ever is the
/// silent degradation ADR-0005 and ADR-0019 forbid. Here it is not silent: the start-up stops
/// and NAMES the quota.
///
/// ⚠️ NO `PartialEq`, AND IT IS FORCED RATHER THAN CHOSEN: `OpenError` derives `Debug` alone,
/// so an `assert_eq!` on this type does not compile and the probes match instead. `Debug` is
/// what the probes and `main` both need, and it is the only thing `OpenError` gives.
///
/// ⛔ RECALL OF 2026-09-19, SUB-PROJECT 2, TASK 9 — THE VARIANTS ARE NOW SIX, AND THE PARAGRAPH
/// ABOVE IS ABOUT THE THIRD, WHICH IS UNCHANGED. The wiring grew three failures it can name and
/// did not have: the journal will not replay at all, so the step number to carry on from cannot
/// be found (`numbering::seeded_from`); it replays, and a record in it is not one this build can
/// read, so the policy in force cannot be answered (ADR-0006, `policy_now`); and the local socket
/// will not bind, which on both systems means somebody is already listening on that name — a
/// SECOND core, which is exactly what a single-instance process must refuse to be.
///
/// ⛔ THE FIRST TWO ARE TWO DIFFERENT FAULTS AND NOT ONE SAID TWICE, AND THE ORDER IS WHAT MAKES
/// THEM SO. Both re-reads go through the same `Journal::replay`, so whichever runs FIRST takes
/// every replay failure and the second can only fail on what replay handed back. `seeded_from`
/// runs first and replaying is its ONLY way to fail, so `Numbering` means "the archive would not
/// re-read"; `policy_now` runs second, on a replay that worked, so `Policy` means "it re-read,
/// and a record in it is not one this build understands". Written the other way round —
/// `policy_now` first — `Numbering` would have had no possible producer at all, and its sentence
/// in `main` would be one no operator could ever see.
///
/// ⚠️ Each is spelt out in `main` for the reason written there: `#[derive(Debug)]` does not count
/// as a read, so folding them would flag their payloads dead.
#[derive(Debug)]
enum StartupError {
    /// The journal file would not open. Two things a human has to tell apart live inside
    /// `OpenError`: a wrong path, and a file another journal already holds.
    Journal(OpenError),
    /// A permanent quota of ADR-0033 did not get in. The name is the profile's own.
    ReservedQuota { name: &'static str },
    /// The run stopped without finishing.
    Run(RunError),
    /// The journal would not say which VRAM policy is in force (ADR-0006, `policy_now`).
    Policy(PolicyError),
    /// The journal would not say which step number to carry on from (`numbering::seeded_from`).
    /// ⛔ DECLARED, NOT PINNED (gotcha #73): the re-read runs FIRST, so reaching this variant
    /// wants a `redb` archive that will not replay at all — a corrupt DATABASE, not a corrupt
    /// record — and nothing in this workspace knows how to make one. ITS TRIGGER IS THE FIRST
    /// BENCH THAT DOES. It stays because it is reachable in production; what is missing is the
    /// provocation, not the road.
    Numbering(JournalError),
    /// The local socket would not bind. ⛔ ON BOTH SYSTEMS THE ORDINARY CAUSE IS A SECOND CORE
    /// ALREADY LISTENING, and refusing is the point: `daemon` is the single instance of ADR-0004,
    /// and two cores on one journal is the one thing the exclusive lock cannot catch, because the
    /// second one never gets that far.
    Ipc(std::io::Error),
}

/// Builds the production graph and runs the executor, handing back what the start-up said.
///
/// ⚠️ IT IS A FUNCTION RATHER THAN THE BODY OF `main` SO THAT A TEST CAN CALL IT. The
/// quality gate runs `cargo build` and `cargo test`, never `cargo run`, so a wiring that
/// only `main` touches would be the one part of this milestone that no check exercises —
/// and a principle nobody can check is an intention. `main` keeps the process-level job,
/// what to print and what to exit with, and nothing else.
///
/// ⛔ RECALL OF 2026-09-19, SUB-PROJECT 2, TASK 9 — NO TEST CALLS IT ANY MORE, AND THE PARAGRAPH
/// ABOVE IS DATED. It hands `u64::MAX`, so a probe calling it would HANG rather than fail (D28);
/// every probe goes through `run_the_graph` with a limit of its own. What this function chooses —
/// the four production literals and the socket name — is walked by nothing: declared, not
/// covered. The sentence below about "the test that already existed" describes that day, not this.
///
/// ⛔ THE PATH IS AN ARGUMENT, AND THAT IS NOT CAUTION. Handed a `FileJournal`, the test that
/// already existed starts writing a REAL FILE; a fixed path in a shared directory is gotcha
/// #52, and on Windows the clean-up of an open file fails silently, so the red would come out
/// on Linux — the project's second system.
fn run_the_production_graph(journal_path: &Path, layout_path: &Path) -> Result<(), StartupError> {
    run_the_graph(
        Parameters::new(EXECUTOR_TURN_LIMIT, TOTAL_VRAM, ARBITER_ID, GUI_TICK),
        journal_path,
        layout_path,
        SOCKET_NAME,
    )
}

/// The graph itself, on parameters it is HANDED rather than reads.
///
/// ⛔ IT EXISTS SO THE TWO PERMANENT QUOTAS CAN BE PROVEN TO STOP THE START-UP, and that is
/// worth the extra function: with the total taken from `TOTAL_VRAM` inside the body, a probe
/// could not build a machine too small to hold them, and the error branch that closes `E41`
/// would be reachable by no check at all. It is also the shape ADR-0034 already imposes
/// everywhere else — the value is DELIVERED at construction, and here it is delivered one
/// level further down.
/// ⛔ RECALL OF 2026-09-19, SUB-PROJECT 2, TASK 9 — THE SOCKET NAME IS AN ARGUMENT TOO, FOR THE REASON
/// THE PATH ALREADY IS. The doc of `run_the_production_graph` says a fixed path in a shared directory is gotcha #52;
/// a fixed socket NAME is worse, because it is shared across the whole machine rather than a
/// directory: two probes binding `SOCKET_NAME` at once make the second fail with "already in use",
/// and `cargo test` runs them at once BY DEFAULT. Every probe hands its own name, built from
/// `line!()` and the process id, exactly as `private_dir_for_line` does (P-55).
///
/// ⚠️ NO PROBE HERE WATCHES A CLIENT DIE: this binary exposes no `Core`, so the wiring of
/// `ClientGrants::on_disconnect` is held by the bench of task 7 and by the campaign of task 10, on
/// `Core::attending` (R4-9). In sub-project 2 no ordinary grant exists to give back (D5).
fn run_the_graph(
    parameters: Parameters,
    journal_path: &Path,
    layout_path: &Path,
    socket_name: &str,
) -> Result<(), StartupError> {
    // ⛔ RECALL OF 2026-09-19, SUB-PROJECT 2, TASK 9 — THE JOURNAL HAS A CONSUMER NOW, and the comment
    // that stood here said it did not: "THE JOURNAL HAS NO CONSUMER IN THIS BINARY YET … The day
    // something journals, it journals into this one." That day is this one. It is still opened
    // first, so a bad path stops the start-up here rather than at the first write.
    let journal = FileJournal::open(journal_path).map_err(StartupError::Journal)?;

    // ⛔ THE TWO PROJECTIONS ARE READ BEFORE THE JOURNAL IS HANDED OVER, and the order is forced:
    // `Core::new` takes it by value. Both re-read the whole archive, which is the cost
    // `Journal::replay` declares of itself.
    //
    // ⛔ AND WHICH OF THE TWO GOES FIRST DECIDES WHAT TWO ERRORS MEAN. They share one
    // `Journal::replay`, so whichever runs first takes every replay failure and the second can
    // only fail on what replay handed back. Seeding first is what gives `StartupError::Numbering`
    // and `StartupError::Policy` two different meanings instead of one; the doc of that enum
    // argues it, and the other order left `Numbering` with no producer at all.
    //
    // ⛔ AND THAT ORDER IS DECLARED, NOT PINNED. ⚠️ IT IS NOT THE "LOAD-BEARING" OF THE
    // DECLARATION ORDER FURTHER DOWN, which the COMPILER holds and which says so of itself:
    // NOTHING holds this one. ✅ MEASURED on 2026-09-19 by applying the swap rather than deduced:
    // with `policy_now` put back in front and nothing else touched, `cargo test --locked -p
    // daemon` comes back 15 passed, 0 failed, and the whole workspace stays green. ⛔ AND NO
    // PROBE CAN HOLD IT, which is a fact about the port and not a gap in the bench: telling the
    // two orders apart wants an archive that OPENS and then will not REPLAY, and
    // `FileJournal::replay` maps every one of its faults onto `JournalError::NotDurable` from a
    // `redb` error -- none of which a caller can arrange from outside. The probe below cannot see
    // the difference either: a record this build cannot read answers `PolicyError::Record` under
    // BOTH orders. ⛔ ITS TRIGGER IS THE ONE WRITTEN BESIDE `StartupError::Numbering`: the first
    // bench that can make a `FileJournal` refuse to replay. Until then this paragraph and review
    // are what hold it, and saying so is the point.
    let steps = numbering::seeded_from(&journal).map_err(StartupError::Numbering)?;

    // ⛔ AND THE `unwrap_or` IS WHERE THE DEFAULT OF ADR-0006 LIVES — D27. `policy_now` answers an
    // `Option` because the kernel may not name a default (ADR-0034), and `None` means NOBODY EVER
    // CHANGED IT rather than "remote". Folding the two inside the kernel would leave this root
    // unable to tell those apart.
    let policy = arbiter::policy_now(&journal)
        .map_err(StartupError::Policy)?
        .unwrap_or(VramPolicy::Remote(RemotePolicy));

    let arbiter = build_the_arbiter(parameters, policy)?;

    let ipc = LocalSocketIpc::bound(socket_name, Progressive::starting_at(0), MAX_BODY)
        .map_err(StartupError::Ipc)?;

    // ⚠️ THE DECLARATION ORDER IS LOAD-BEARING and swapping two lines does not compile: the
    // executor borrows `sleep`, `core` and `clock` for its whole life, `clock` borrows `reactor`,
    // and locals drop in reverse order of declaration.
    let reactor = RefCell::new(SystemReactor::new());
    let core = RefCell::new(Core::new(
        ipc,
        journal,
        MaybeCustody::open(layout_path),
        arbiter,
        steps,
        parameters,
    ));
    let clock = SharedClock { inner: &reactor };
    let sleep = Sleep::new();

    let mut executor = Executor::new(
        SequentialRng::new(),
        SharedClock { inner: &reactor },
        parameters,
        &sleep,
    );
    executor.spawn(serving::serve(&core, &clock, &sleep));

    executor.run().map_err(StartupError::Run)
}

/// Builds the arbiter and takes the two permanent quotas of ADR-0033 out of its budget.
///
/// ⛔ IT HANDS THE ARBITER BACK INSTEAD OF KEEPING IT, and that is what makes the BOOKS
/// checkable: `run_the_graph` answers `Result<(), StartupError>` and nothing else, so a probe
/// that wants to ask `allocated()` or `policy()` has to be given the object. A probe that
/// assembled its own would be a second copy of the wiring, green on the day the two drift.
///
/// ⛔ THE ORDER IS AUDIO FIRST, AND IT IS NOT ARBITRARY: both profiles sit in
/// `ComputeClass::Realtime`, so nothing inside the arbiter breaks the tie between two requests
/// of one lane except arrival. On a machine too small for both, whichever is asked for SECOND
/// is the one that does not get in -- which is what the two probes name.
///
/// ⚠️ THE TWO GRANTS ARE DROPPED HERE AND THE RESERVATIONS ARE NOT, and that is the point
/// rather than an oversight: "permanent" is not a type, it is "nobody calls release". The
/// arbiter keeps both in its books until somebody hands a grant back, and nobody ever will.
fn build_the_arbiter(parameters: Parameters, policy: VramPolicy) -> Result<Arbiter, StartupError> {
    // ⛔ RECALL OF 2026-09-19, SUB-PROJECT 2, TASK 9 — THE POLICY IS HANDED IN, AND THE COMMENT THAT WAS
    // HERE SAID THE OPPOSITE. It read: "REMOTE is the default of ADR-0006, and reopening that turns
    // a coordinated swap from an exception into the normal case." The DEFAULT is unchanged and is
    // still remote; what changed is WHO SAYS SO. The recall at the head of ADR-0006 splits the two
    // facts: "the profile gives the DEFAULT, and the CURRENT policy is the projection of the
    // journal". The default now lives at the ONE call site that re-reads the journal, as an
    // `unwrap_or`, and this function names neither.
    let mut arbiter = Arbiter::new(parameters, policy);

    let _audio = reserve(&mut arbiter, &AUDIO_RESERVATION)?;
    let _presentation = reserve(&mut arbiter, &PRESENTATION_RESERVATION)?;

    Ok(arbiter)
}

/// Turns an `Admission` into a start-up decision.
///
/// ⛔ THE TWO FAILING ANSWERS ARE THE SAME FAILURE HERE, AND THEY ARE NOT THE SAME EVERYWHERE.
/// `Refused` means "bigger than the whole machine" and `Queued` means "bigger than what is
/// free right now"; for an ordinary request those call for different behaviour, which is why
/// `Admission` has no `is_granted()`. For a PERMANENT quota they collapse: nobody releases a
/// permanent grant, so a queued one waits for ever, and waiting for ever at start-up is a
/// misconfiguration exactly like asking for more than the machine has.
///
/// ⚠️ AND THE COLLAPSE IS MADE HERE AND NOT INSIDE THE ARBITER, which is `E41` in one line:
/// the arbiter cannot tell a ticket that will be served from one that never will, and a rule
/// it cannot evaluate is not a rule it can enforce. The composition root can, because it is
/// the one that knows nobody is ever going to release these two.
///
/// ⛔ DECLARED LIVE MUTANT -- `Monotonic::ORIGIN` IS HELD BY NOTHING, said here rather than
/// left to be found beside the residuals that ARE declared. `FOR_EVER` saturates, so
/// `now.saturating_add(FOR_EVER)` is `u64::MAX` whatever `now` is, and every starting instant
/// gives the same `expires_at`. ✅ MEASURED on 2026-08-21, not deduced: with this argument
/// changed to `Monotonic::from_millis(1)` NOTHING in the whole workspace goes red. The count
/// is row 15 of the mutation campaign in `docs/porta-di-qualita.md`.
///
/// ⚖️ AND IT IS DECLARED AND NOT PINNED, because there is no claim behind it to defend: the
/// indifference is an ARITHMETIC CONSEQUENCE of the saturation and not a decision somebody
/// took. The day the window stops saturating, the starting instant starts mattering.
fn reserve(arbiter: &mut Arbiter, profile: &ResourceProfile) -> Result<Grant, StartupError> {
    match arbiter.admit(profile, FOR_EVER, Monotonic::ORIGIN) {
        Admission::Granted(grant) => Ok(grant),
        Admission::Queued(_) | Admission::Refused { .. } => {
            Err(StartupError::ReservedQuota { name: profile.name })
        }
    }
}

/// ⛔ DECLARED RESIDUAL — THE ERROR BRANCHES BELOW ARE COVERED BY NOTHING, and saying so is
/// the point. The wiring was pulled out into a function precisely because the gate runs
/// `build` and `test` and never `run`; this is the half that stayed behind. No check observes
/// that a failed start-up writes to stderr, leaves stdout empty, and exits 1.
///
/// ⚠️ AND WHICH BRANCHES WERE WALKED BY HAND IS NAMED, because "verified by hand" over four
/// arms is a claim about three of them nobody made. Walked on 2026-08-21, in a scratch
/// directory outside the repository: the `Ok` arm — exit 0, the sentence on stdout, stderr
/// EMPTY, and a `journal.redb` of 1 056 768 bytes left behind — and the `Journal` arm, provoked
/// by putting a DIRECTORY where the file should be: exit 1, stdout EMPTY, and
/// `File(Os { code: 5, kind: PermissionDenied, … })` on stderr. ⛔ `ReservedQuota` AND `Run`
/// WERE NOT WALKED: neither can be provoked from outside without editing this file, which is
/// what the mutation campaign does and a hand-run cannot. And a verification by hand is a
/// moment in time in any case, not a control.
///
/// ⚠️ AND IT IS NOT WORTH THE PRICE, which has to be said rather than implied. Covering it
/// means spawning the built binary as a CHILD PROCESS and reading back its two streams and
/// its exit status, in order to hold lines that make no decision. `platform`'s `wait_until`
/// declares a residual for the same shape of reason: a control that is absent and DECLARED
/// beats one that is contorted. The trade stops being fair the day these branches grow a
/// decision of their own.
///
/// ⛔ THE THREE FAILURES ARE SPELT OUT ONE BY ONE AND NOT FOLDED INTO ONE `{error:?}`, and it
/// is FORCED rather than a preference: `#[derive(Debug)]` does not count as a read for the
/// dead-code analysis, so a single arm left `Journal`'s and `Run`'s payloads flagged as
/// "field `0` is never read" — MEASURED, two warnings on `cargo test -p daemon`. Silencing
/// that with an `#[allow]` is a prohibition switched off (gotcha #13), and emptying the two
/// payloads would throw away the only thing that says WHICH file and WHICH failure. Reading
/// them is what the fix had to be, and the operator gets three different sentences out of it.
///
/// ⛔ RECALL OF 2026-09-19, SUB-PROJECT 2, TASK 9 — THEY ARE SIX. The reason is unchanged and is
/// the reason the three new ones are also spelt out: a single arm would leave every payload
/// flagged "never read". ⚠️ AND THE RESIDUAL ABOVE GREW WITH THEM: none of the six error branches
/// is walked by a check, and three of them now name values — the socket, the policy, the counter
/// — that only `main` knows how to print. ⚠️ AND THE `Ok` ARM IS WORSE OFF THAN THEY ARE: it has
/// no possible producer at all since this task, which is said at the arm itself rather than
/// counted a second time here.
fn main() {
    match run_the_production_graph(Path::new(JOURNAL_PATH), Path::new(LAYOUT_PATH)) {
        // ⛔ RECALL OF 2026-09-19, SUB-PROJECT 2, TASK 9 — THIS ARM HAS NO POSSIBLE PRODUCER, AND
        // THE SENTENCE IT PRINTED WAS FALSE TWICE OVER. It read "the executor ran with no
        // activities": this task spawns one, and while that one is `kernel::serving::serve` --
        // a `loop` with no exit -- `run()` cannot answer `Ok(())` at all, because `Executor::run`
        // reaches its `Ok(())` only when the task list EMPTIES, and this list never does.
        // ⛔ IT IS THE STANDARD E61 USED AGAINST `StartupError::Numbering`, applied in the other
        // direction and said out loud instead of left to be found: a branch no run can reach is a
        // claim nobody checks, and none of this file's probes covers it. ⚠️ THE ARM STAYS, because
        // the `match` must be exhaustive and `Ok` is a variant of `Result` rather than a choice
        // made here. ⛔ ITS TRIGGER IS THE CLEAN SHUTDOWN of sub-project 10, which §5 assigns
        // there along with the watchdog: the day the serving activity can FINISH, this is the
        // sentence that prints, so it is written to be true on that day rather than on the last.
        Ok(()) => println!(
            "daemon: the graph is wired, the two reserved quotas are held, and the serving \
             activity finished."
        ),
        // ⛔ stderr and exit 1 on every failing branch: a start-up that did not complete must
        // be distinguishable by a caller that reads neither stream.
        Err(StartupError::Journal(error)) => {
            stop(&format!(
                "the journal at {JOURNAL_PATH} would not open: {error:?}"
            ));
        }
        Err(StartupError::ReservedQuota { name }) => {
            stop(&format!(
                "the reserved quota {name} did not get in: this machine is too small for the \
                 quotas of ADR-0033"
            ));
        }
        Err(StartupError::Run(error)) => {
            stop(&format!(
                "the executor stopped without finishing: {error:?}"
            ));
        }
        Err(StartupError::Policy(error)) => {
            stop(&format!(
                "the journal at {JOURNAL_PATH} would not say which VRAM policy is in force: {error:?}"
            ));
        }
        Err(StartupError::Numbering(error)) => {
            stop(&format!(
                "the journal at {JOURNAL_PATH} would not say which step to carry on from: {error:?}"
            ));
        }
        Err(StartupError::Ipc(error)) => {
            stop(&format!(
                "the channel {SOCKET_NAME} would not bind, and the usual cause is a core already \
                 running: {error:?}"
            ));
        }
    }
}

/// Says why the start-up stopped, on stderr, and leaves exit code 1 behind.
///
/// ⚠️ `!` AND NOT `()`, so that the call sites above do not each need a statement saying nothing
/// follows. It is the return type `std::process::exit` already has.
///
/// ⛔ RECALL OF 2026-09-19, SUB-PROJECT 2, TASK 9 — THIS SAID "the THREE call sites above" AND THE
/// FIGURE IS REMOVED RATHER THAN REALIGNED, which is the same cure this task gave the "six port
/// families" line: a count in prose goes false again at the seventh error this binary learns to
/// name. ⚠️ AND NO COMMAND IS OFFERED IN ITS PLACE, which is deliberate: the number must not
/// appear here at all, so handing the reader a motif to count with would only be the same
/// numeral one step removed -- and a motif written on this line would count ITS OWN citation,
/// measured at 7 against 6 on 2026-09-19.
fn stop(reason: &str) -> ! {
    eprintln!("daemon: {reason}.");
    std::process::exit(1)
}

#[cfg(test)]
mod tests {
    //! ⚠️ A UNIT TEST MODULE IN `src/`, where this repository otherwise puts tests in
    //! `tests/` — and here the deviation is NOT a preference, it is FORCED. The functions
    //! under test are private in a `bin` target, and an integration test is a crate of its own
    //! that can link only a LIBRARY. No file under `tests/` can reach
    //! `run_the_production_graph`, so moving this module out would not relocate the tests, it
    //! would delete them.
    //!
    //! ⚠️ HOW MANY SUCH MODULES THERE ARE, AND WHERE, COMES FROM THE COMMAND AND NOT FROM THIS
    //! LINE — `grep -rn --include='*.rs' 'mod tests {' crates/*/src/`. What differs between them
    //! is the REASON, and each states its own: here it is forced, elsewhere it IS a choice.
    //!
    //! ⛔ RECALL OF 2026-08-28, FINDING AUD-060 — THIS SAID "one of THREE" AND NAMED THE OTHER
    //! TWO. The figure was RIGHT and is removed anyway: the same count sat in
    //! `crates/platform/src/rng.rs` saying TWO, and correcting only the false house would keep
    //! the count in two places, which is what let them diverge (gotcha #68).

    use super::*;

    // ⚠️ IMPORTED HERE AND NOT AT THE TOP OF THE FILE, and it is MEASURED rather than tidy:
    // `MakeRoom` is the trait `VramPolicy::name` lives on, nothing outside these tests calls it,
    // and at the top it made `cargo build --locked --workspace` say `unused import` — a warning
    // this repository does not switch off with an `#[allow]` (gotcha #13).
    use kernel::arbiter::MakeRoom;

    // ⚠️ THE WIRE IS A BENCH-ONLY IMPORT, for the same reason `MakeRoom` is: the binary itself
    // never encodes or decodes a message -- `kernel::serving` does that on the other side of the
    // port -- so at the top of the file every one of these would be an `unused import`. The peer
    // of `a_peer_that_says` is the only thing here that speaks the wire.
    use kernel::framing;
    use kernel::wire::ipc::{build_stamp, IpcMessage, LayoutState, PolicyName};

    // ⚠️ AND THE TWO BELOW ARE BENCH-ONLY FOR A DIFFERENT REASON: the binary never NAMES a
    // policy other than the default and never writes a record of its own, so they reach this file
    // only through the transition `the_policy_in_the_journal_is_the_one_the_arbiter_starts_on`
    // arranges, and through the garbage record `a_record_this_build_cannot_read_...` writes.
    use kernel::arbiter::LocalPolicy;
    // ⚠️ `Journal` IS THE TRAIT AND NOT THE TYPE: `intent` is a method of the port, and the
    // binary itself never calls it -- it hands the journal over to `Core` and `Core` writes.
    use kernel::ports::journal::{Journal, StepId};

    /// ⛔ A DIRECTORY OF ITS OWN PER CALL SITE, from `line!()`, and it is not caution: a
    /// fixed path in a shared directory is gotcha #52, measured at milestone 3. Windows
    /// refuses to delete a file that is open, so the removal FAILS SILENTLY there and the
    /// red comes out on Linux -- the project's second system.
    ///
    /// ⚠️ AND THE PREFIX IS DIFFERENT from the two benches of `platform`, because a line
    /// number is unique inside ONE file only and the binaries run together.
    fn private_dir_for_line(line: u32) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("daemon-production-graph-{line}"));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("a fresh directory for this call site");
        dir
    }

    /// ⛔ A NAME OF ITS OWN PER CALL SITE, and it is the socket twin of `private_dir_for_line`
    /// (P-55). A socket name is machine-wide rather than directory-wide, so two probes sharing one
    /// pass alone and fail together — the flakiest red there is — and `cargo test` runs them at
    /// once by default. The process id is in it because two `cargo test` invocations can overlap.
    fn socket_name_for_line(line: u32) -> String {
        format!("harness-daemon-{}-{}", std::process::id(), line)
    }

    /// How many messages the core sends after a valid `Hello` -- the welcome of sequence 1.
    ///
    /// ⛔ MEASURED ON THE DISPATCH OF TASK 7, `greet` in `crates/kernel/src/serving.rs`:
    /// `Accepted`, `Degradation`, `Policy`, `Layout`, `Steps`. The bench of that task pins the same
    /// number in `the_welcome_is_the_five_messages_of_sequence_one`, so the day the welcome grows,
    /// that probe goes red before this constant does.
    const WELCOME: usize = 5;

    /// The turn budget of every probe that has a PEER. ⛔ IT IS WALL CLOCK IN DISGUISE (R4-6): the
    /// listener lives only inside `run_the_graph`, and the peer connects from a thread the OS
    /// schedules when it likes, so a short run can end before the peer ever knocks. A hundred
    /// thousand turns at a zero tick is a fraction of a second of polling, and it is the same
    /// number the turn probe delivers. Probes WITHOUT a peer keep their own small budgets.
    const WITH_A_PEER: u64 = 100_001;

    /// A peer on the other end of the wire: it connects, says its piece, and hands back what it
    /// heard.
    ///
    /// ⛔ IT IS WHAT MAKES THE TURN PROBE NON-VACUOUS (P-52). `serve` is a `loop` with no exit, so
    /// `Executor::run` answers `Err(TurnLimitReached)` at ANY limit — one turn or a hundred
    /// thousand — and reading that value alone would be green over an activity that never ran. What
    /// only a peer can say is THAT THE CORE IS SERVING.
    ///
    /// ⛔ IT CANNOT HANG, AND THE REASON IS THE DROP ORDER RATHER THAN A TIMEOUT: when
    /// `run_the_graph` returns, its locals fall, the `LocalSocketIpc` falls with them, and the
    /// server end of this connection closes — so `read` here comes back `Ok(0)` and the loop ends.
    /// Every caller therefore `join`s AFTER the run, never before.
    ///
    /// ⚠️ THE CONNECT IS A `yield_now` LOOP AND NOT A SLEEP, the shape `ipc_contract_real.rs` uses:
    /// the listener exists from `bound()`, which happens before `run()`, but this thread may be
    /// scheduled first. ⛔ AND THE LOOP HAS A WALL-CLOCK DEADLINE (R4-6): the listener lives only
    /// inside `run_the_graph`, so if the run ends before this thread connects, `connect` fails FOR
    /// EVER and a bare loop would hang the gate -- the worst red there is. Five seconds is not a
    /// tuning: it is an order of magnitude above any scheduling delay, and the panic names this line.
    fn a_peer_that_says(
        name: String,
        said: Vec<IpcMessage>,
        wants: usize,
    ) -> std::thread::JoinHandle<Vec<IpcMessage>> {
        std::thread::spawn(move || {
            use interprocess::local_socket::{prelude::*, GenericNamespaced, Stream};
            use std::io::{Read, Write};

            let ns = name.to_ns_name::<GenericNamespaced>().expect("a namespaced name");
            let started = std::time::Instant::now();
            let mut stream = loop {
                match Stream::connect(ns.clone()) {
                    Ok(stream) => break stream,
                    Err(error) => {
                        assert!(
                            started.elapsed() < std::time::Duration::from_secs(5),
                            "the peer could not connect within five seconds ({error:?}): the run \
                             ended before this thread got to the listener (R4-6)"
                        );
                        std::thread::yield_now();
                    }
                }
            };
            for message in &said {
                let bytes = message.encode().expect("the peer frames what it sends");
                stream.write_all(&bytes).expect("the peer writes");
            }

            let mut buffer = Vec::new();
            let mut heard = Vec::new();
            let mut chunk = [0_u8; 4_096];
            while heard.len() < wants {
                match stream.read(&mut chunk) {
                    Ok(0) => break,
                    Ok(read) => buffer.extend_from_slice(&chunk[..read]),
                    Err(_) => break,
                }
                // ⚠️ `take_frame` AND NOT `unframe`: the buffer ordinarily holds a frame and a half,
                // which `unframe` refuses by design (P-13). It hands back where the next frame starts,
                // and `IpcMessage::decode` is given the WHOLE frame, envelope included, because
                // `decode` unframes what it is given (task 2's bench says so in those words). Measured
                // at the plan review (R4-2): `decode(body)` answered `Err` on every message.
                while let Some((_, next)) = framing::take_frame(&buffer) {
                    heard.push(
                        IpcMessage::decode(&buffer[..next]).expect("the core sends what it says"),
                    );
                    buffer.drain(..next);
                }
            }
            heard
        })
    }

    /// The arbiter of the PRODUCTION parameters, or a red that NAMES the quota that fell.
    ///
    /// ⛔ IT HOLDS SOMETHING INSTEAD OF ONLY SHORTENING TWO CALL SITES, which is the shape a
    /// bench helper has to have here -- task 8 already paid once for one that held nothing
    /// while its doc said otherwise. What it holds is that on `TOTAL_VRAM` BOTH quotas of
    /// ADR-0033 get in: ✅ MEASURED, with `TOTAL_VRAM` cut to `Mib::new(1_000)` every caller
    /// goes red through this `panic!`, and row 2 of the campaign is that measurement.
    ///
    /// ⚠️ `match` AND NOT `.expect(…)`, and it is forced rather than chosen: `Arbiter` has no
    /// `Debug`, so the `Result` cannot be formatted as a whole. Taking the error out first is
    /// what lets the failure say which quota fell.
    fn the_production_arbiter() -> Arbiter {
        match build_the_arbiter(
            Parameters::new(EXECUTOR_TURN_LIMIT, TOTAL_VRAM, ARBITER_ID, GUI_TICK),
            VramPolicy::Remote(RemotePolicy),
        ) {
            Ok(arbiter) => arbiter,
            Err(error) => panic!("a permanent quota of ADR-0033 must be granted: {error:?}"),
        }
    }

    /// ⛔ RECALL OF 2026-09-19, SUB-PROJECT 2, TASK 9 — THE NAME CHANGED BECAUSE THE CLAIM DID. There is no
    /// "completion" any more: the graph now carries `kernel::serving::serve`, a `loop` with no exit,
    /// so a run that ENDED would mean the core stopped serving. What the run terminating proves is
    /// that the turns ran out, which is the delivered limit reaching the executor.
    ///
    /// ⛔ AND IT CANNOT GO THROUGH `run_the_production_graph` ANY MORE, which is the cost D28
    /// declares: that function hands `u64::MAX`, so calling it here would HANG rather than fail —
    /// the worst way for a gate to break, because a gate that does not come back says nothing to
    /// anybody. The limit is delivered instead, which is the shape ADR-0034 imposes everywhere.
    ///
    /// ⛔ THE TICK IS ZERO, AND IT IS NOT A SHORTCUT (P-51). `serve` naps every turn, and this
    /// binary mounts the REAL `SystemReactor`, whose `wait_until` is a real sleep — so a production
    /// tick would cost tick × turns of wall clock inside `bash scripts/gate.sh`. A deadline already
    /// reached makes the activity READY instead (`Sleep::until`'s own rule), so the turn is polling
    /// and the ceiling costs milliseconds. ⚠️ What the zero tick does NOT buy is that
    /// `Reactor::wait_until` is reached on this graph; that half is the sub-project 2 campaign's
    /// (task 10), where the clock is virtual.
    ///
    /// ⚠️ THE RESIDUAL OF THIS PROBE GREW, and it is the same residual said wider: it did not cover
    /// the VALUE of `EXECUTOR_TURN_LIMIT`, and now it does not cover the wiring of
    /// `run_the_production_graph` either — the four production literals it chooses are walked by
    /// nothing. The doc of that function carries the recall.
    #[test]
    fn the_production_graph_assembles_and_the_serving_activity_takes_the_turns() {
        let dir = private_dir_for_line(line!());

        let outcome = run_the_graph(
            Parameters::new(8, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),
            &dir.join("journal.redb"),
            &dir.join("layout.redb"),
            &socket_name_for_line(line!()),
        );

        match outcome {
            Err(StartupError::Run(RunError::TurnLimitReached)) => {}
            other => panic!("the graph must assemble and the serving loop must run: {other:?}"),
        }
    }

    /// ⛔ WHAT THIS BUYS THAT THE ASSEMBLY TEST DOES NOT: that the journal is really OPENED.
    /// A wiring that had simply dropped the `FileJournal::open` line would assemble and run to
    /// completion exactly as before -- nothing in this binary reads the journal yet -- so the
    /// test above would stay green over a graph with no durable store in it at all. The file
    /// on the disk is the only thing that tells the two apart, and it is there because
    /// `FileJournal::open` COMMITS on every open, which is written down beside that function.
    ///
    /// ⛔ RECALL OF 2026-09-19, SUB-PROJECT 2, TASK 9 — TWO CLAUSES ABOVE ARE DATED, AND WHAT THIS
    /// PROBE BUYS IS NOT ONE OF THEM. The expected outcome is now `Err(TurnLimitReached)`, for the
    /// reason the probe above carries: the graph runs `kernel::serving::serve`, a `loop` with no
    /// exit, so an `Ok(())` would mean the core stopped serving — and the call goes through
    /// `run_the_graph` with a limit of its own, because `run_the_production_graph` hands
    /// `u64::MAX` and would hang (D28). And "nothing in this binary reads the journal yet" stopped
    /// being true today: `numbering::seeded_from` and `arbiter::policy_now` both re-read it and
    /// `Core` is handed it, so dropping the `open` line no longer compiles at all. What tells the
    /// two wirings apart is unchanged — only an `open` that really happened leaves a file behind.
    #[test]
    fn the_production_graph_leaves_its_journal_on_the_disk() {
        let dir = private_dir_for_line(line!());
        let path = dir.join("journal.redb");

        let outcome = run_the_graph(
            Parameters::new(8, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),
            &path,
            &dir.join("layout.redb"),
            &socket_name_for_line(line!()),
        );

        match outcome {
            Err(StartupError::Run(RunError::TurnLimitReached)) => {}
            other => panic!("the graph must assemble and the serving loop must run: {other:?}"),
        }
        assert!(
            path.is_file(),
            "the journal must be a real file, and this is what says `open` was reached"
        );
    }

    /// The other direction of the same rule (§7.1.1 rule 3): a journal that CANNOT be opened
    /// must stop the start-up instead of being carried on without.
    ///
    /// ⚠️ THE FAILURE IS PROVOKED BY A DIRECTORY THAT IS NOT THERE, which is the one way to
    /// make `open` fail that needs no privileges and behaves the same on both of the project's
    /// systems -- a locked file would need a second process on Linux, and a read-only path
    /// would need a mode change Windows spells differently.
    #[test]
    fn a_journal_that_cannot_be_opened_stops_the_start_up() {
        let dir = private_dir_for_line(line!());

        let outcome = run_the_graph(
            Parameters::new(8, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),
            &dir.join("no-such-directory").join("journal.redb"),
            &dir.join("layout.redb"),
            &socket_name_for_line(line!()),
        );

        match outcome {
            Err(StartupError::Journal(_)) => {}
            other => panic!("a journal that will not open must stop the start-up: {other:?}"),
        }
    }

    /// ⛔ WHAT THIS BUYS THAT THE ASSEMBLY TEST DOES NOT: that the two quotas are HELD, not
    /// subtracted. An arbiter that had merely lowered its ceiling would pass the test above
    /// and leave I2 false for the two consumers -- gotcha #4, and it is the whole reason the
    /// design diverges from the letter of §5.1.
    ///
    /// ⚠️ IT CALLS THE PRODUCTION BUILDER AND DOES NOT REBUILD THE ARBITER, which is the whole
    /// reason that builder is a function of its own: `allocated()` is not reachable through
    /// `run_the_graph`, which hands back a `Result<(), StartupError>` and nothing else, so a
    /// probe of the BOOKS has to be handed the arbiter itself. An arbiter assembled here would
    /// be a second copy, and the day the two drifted apart this would go on passing about a
    /// graph nobody ships.
    ///
    /// ⛔ AND IT PINS THE POLICY, WHICH IS NOT DECORATION: `VramPolicy::Remote` is the DEFAULT
    /// of ADR-0006 and the comment beside that line says so, and an assertion with no guard is
    /// gotcha #14. ✅ MEASURED, not feared: with the wiring swapped to
    /// `VramPolicy::Local(LocalPolicy)` and this line absent, the WHOLE WORKSPACE stayed green
    /// -- 253 passed, 0 failed. The mutant was alive.
    ///
    /// ⛔ RECALL OF 2026-09-19, SUB-PROJECT 2, TASK 9 — THE POLICY ASSERTION NOW HOLDS WHAT THE
    /// HELPER CHOOSES AND NOT WHAT THE ROOT CHOOSES, AND THE PARAGRAPH ABOVE DESCRIBES THE OLD
    /// WIRING. `build_the_arbiter` is handed a policy today, so `the_production_arbiter` is the
    /// one that names `VramPolicy::Remote(RemotePolicy)` and this line reads it straight back.
    /// The line STAYS, because the two together are still what would go red if the helper started
    /// naming something else. ⛔ WHAT MOVED IS THE CLAIM ABOUT THE ROOT: that the composition root
    /// starts on the DEFAULT of ADR-0006 — and on what the journal says instead, when it says
    /// anything — is held by `the_policy_in_the_journal_is_the_one_the_arbiter_starts_on`, which
    /// reads it where it now lives, at the `unwrap_or` of `run_the_graph`.
    ///
    /// ⚠️ `match` AND NOT `assert!(… .is_ok())`, and it is forced rather than chosen:
    /// `build_the_arbiter` hands back an `Arbiter`, which has no `Debug`, so the `Result`
    /// cannot be formatted. Taking the error out first is what lets a failure say which quota
    /// fell.
    #[test]
    fn the_two_reserved_quotas_are_held_by_grants_and_not_subtracted() {
        let arbiter = the_production_arbiter();

        assert_eq!(
            arbiter.allocated(),
            AUDIO_QUOTA.saturating_add(PRESENTATION_QUOTA),
            "the quotas are SPOKEN FOR, which is what a subtraction would not show"
        );
        assert_eq!(
            arbiter.policy().name(),
            "remote",
            "the arbiter runs the policy `the_production_arbiter` hands it, which is the DEFAULT \
             of ADR-0006"
        );
    }

    /// ⛔ WHAT HOLDS `FOR_EVER`, and without it the constant was an ASSERTION WITH NO GUARD --
    /// gotcha #14. ✅ MEASURED: with `FOR_EVER` cut to `Millis::new(1)` and this probe absent,
    /// the WHOLE WORKSPACE stayed green, 253 passed and 0 failed. Nothing else in this binary
    /// ever advances the clock, so nothing else can see a validity window at all.
    ///
    /// ⚠️ `promote` AND NOT `allocated()`, and the choice is the arbiter's own doc rather than
    /// taste: `allocated` DELIBERATELY COLLECTS NOTHING, so asking it alone cannot tell "the
    /// sweep happened" from "the number looks right anyway". `promote` sweeps first.
    ///
    /// ⛔ AND IT WALKS THE BOUNDARY IN BOTH DIRECTIONS (§7.1.1 rule 3), which is the half that
    /// gets forgotten. `Monotonic::ORIGIN.saturating_add(FOR_EVER)` saturates AT `u64::MAX`, and
    /// `collect_expired` compares `expires_at <= now`, so the last representable instant on the
    /// axis is exactly the one at which a "permanent" grant IS swept. INSIDE the window --
    /// `u64::MAX - 1` -- both quotas survive; AT `u64::MAX` both are collected and `allocated()`
    /// comes back `Mib::ZERO`. The window is half-open at both ends, which is one rule and not
    /// two.
    ///
    /// ⚠️ AND THE OUTER SIDE IS WHAT THE COMMENT BESIDE `FOR_EVER` ASSERTS. It was written to
    /// record a measurement and then held by nothing, which is gotcha #14 inside the paragraph
    /// that exists to answer it; the second sweep below is what makes it a control.
    #[test]
    fn a_permanent_grant_survives_to_the_last_instant_of_the_axis_and_is_swept_at_it() {
        let mut arbiter = the_production_arbiter();

        let promoted = arbiter.promote(Monotonic::from_millis(u64::MAX - 1));

        assert!(
            promoted.is_empty(),
            "nothing was ever queued, so nothing can come out of a queue"
        );
        assert_eq!(
            arbiter.allocated(),
            AUDIO_QUOTA.saturating_add(PRESENTATION_QUOTA),
            "a permanent grant is still held after a sweep 584 million years out"
        );

        let swept = arbiter.promote(Monotonic::from_millis(u64::MAX));

        assert!(
            swept.is_empty(),
            "still nothing was ever queued, so still nothing comes out of a queue"
        );
        assert_eq!(
            arbiter.allocated(),
            Mib::ZERO,
            "the OTHER side of the boundary: at the last instant a permanent grant IS swept"
        );
    }

    /// ⛔ WHAT HOLDS THE TWO FIELDS OF THE TWO RESERVATIONS, and without it each of the four
    /// was an assertion with no guard -- gotcha #14. These two constants are the only place in
    /// production code that CHOOSES `Preemption::Never`, and `ComputeClass::Realtime` is the
    /// PREMISE of the sentence beside `build_the_arbiter`: both profiles sit in one lane, so
    /// nothing inside the arbiter breaks the tie except arrival.
    /// ✅ MEASURED on 2026-08-21: with any ONE of the four changed and this probe absent, the
    /// WHOLE WORKSPACE stayed green -- 35 targets, 254 passed, 0 failed, 2 ignored, the
    /// baseline exactly. Four live mutants.
    ///
    /// ⛔ IT PINS THE VALUE AND NOT THE CONSEQUENCE, AND THE PRICE IS WRITTEN DOWN INSTEAD OF
    /// IMPLIED. This repository prefers the probe that attacks the MECHANISM, so that one was
    /// BUILT AND MEASURED first: an arbiter under `VramPolicy::Local`, where `may_make_room`
    /// answers yes, asked for more than is free and then swept past every grace. ⚠️ IT KILLS
    /// NEITHER FIELD, because inside `Arbiter::ask_back` the two stand behind ONE ANOTHER'S
    /// guard. `held.lane <= below` drops every `Realtime` grant -- `Realtime` is the top lane,
    /// so a `Realtime` grant is never BELOW the asking lane, whoever asks -- and that guard runs
    /// BEFORE the one that reads the grace, so `preemption` is never reached; with the lane
    /// changed alone, `Preemption::Never` gives no grace and the grant falls at the second
    /// guard instead. A probe no single mutation can kill is the vacuous probe, so it was not
    /// kept. What that road measured lives in the register, beside the mutation campaign.
    ///
    /// ⚖️ WHAT THIS ONE THEREFORE DOES NOT PROVE, said out loud: that either field changes
    /// anything the arbiter DOES. It says ADR-0033's word and the tie-break premise are still
    /// the ones written down.
    #[test]
    fn the_two_reservations_declare_no_preemption_and_one_lane() {
        for reservation in [&AUDIO_RESERVATION, &PRESENTATION_RESERVATION] {
            assert_eq!(
                reservation.preemption,
                Preemption::Never,
                "ADR-0033 asks for a NON-PREEMPTIBLE grant, and {} is where that is CHOSEN",
                reservation.name
            );
            assert_eq!(
                reservation.compute_class,
                ComputeClass::Realtime,
                "one lane for both is what leaves ARRIVAL the only tie-break, and {} left it",
                reservation.name
            );
        }
    }

    /// ⛔ THE SCENARIO OF `E41` EXACTLY, AND IT IS A PERMANENT PROBE AND NOT A MUTATION. A
    /// direction of proof held by a mutation is held by NOTHING: the mutation is reverted and
    /// the record is left saying the line is closed -- gotcha #72.
    ///
    /// `E41` says an impossible configuration stopped ANNOUNCING ITSELF when the queues
    /// arrived: the second permanent quota comes back `Queued`, and nobody will ever serve it
    /// because releasing a permanent grant is exactly what nobody does. The arbiter cannot
    /// repair that -- it cannot tell a ticket that WILL be served from one that never will --
    /// so the visibility belongs HERE, in the composition root, which asks for the two grants
    /// itself and can say so out loud.
    ///
    /// ⚠️ IT GOES THROUGH THE WHOLE GRAPH and not through a hand-built arbiter, which is what
    /// makes it hold the WIRING and not just `reserve`: with the two reservations taken out of
    /// `run_the_graph` this probe goes red.
    #[test]
    fn a_permanent_quota_that_only_queues_stops_the_start_up() {
        let dir = private_dir_for_line(line!());

        // 1024 fits in 1500; 1024 + 768 does not, and under `RemotePolicy` -- which may not
        // make room -- a request that fits the machine but not the moment is QUEUED.
        //
        // ⛔ THE LIMIT IS FINITE AND NOT `EXECUTOR_TURN_LIMIT`, which is `u64::MAX` since task 9
        // -- D28, and it is not decoration. This probe is meant to stop at `build_the_arbiter`,
        // long before the executor exists, so today the delivered limit is never read at all.
        // The day a change let the two quotas through, `u64::MAX` would make this probe HANG
        // instead of going red, and a gate that does not come back says nothing to anybody
        // (E5, E6). A finite limit turns that silence into a verdict.
        let outcome = run_the_graph(
            Parameters::new(8, Mib::new(1_500), ARBITER_ID, Millis::new(0)),
            &dir.join("journal.redb"),
            &dir.join("layout.redb"),
            &socket_name_for_line(line!()),
        );

        match outcome {
            Err(StartupError::ReservedQuota { name }) => assert_eq!(
                name, "presentation-reserved",
                "the failure must NAME the quota that did not get in"
            ),
            other => panic!("a permanent quota that only queues must stop the start-up: {other:?}"),
        }
    }

    /// The SECOND way the same failure arrives, and it is a second probe rather than a second
    /// assertion because the two travel by different roads inside `admit` (gotcha #65):
    /// "bigger than what is free right now" is `Queued`, "bigger than the whole machine" is
    /// `Refused`. One probe would leave whichever road it does not take uncovered.
    #[test]
    fn a_permanent_quota_bigger_than_the_machine_stops_the_start_up() {
        let dir = private_dir_for_line(line!());

        // 1024 is more than the whole machine, so no release will ever make room for it.
        //
        // ⛔ THE LIMIT IS FINITE FOR THE REASON THE PROBE ABOVE GIVES.
        let outcome = run_the_graph(
            Parameters::new(8, Mib::new(500), ARBITER_ID, Millis::new(0)),
            &dir.join("journal.redb"),
            &dir.join("layout.redb"),
            &socket_name_for_line(line!()),
        );

        match outcome {
            Err(StartupError::ReservedQuota { name }) => assert_eq!(
                name, "audio-reserved",
                "the failure must NAME the quota that did not get in"
            ),
            other => {
                panic!(
                    "a permanent quota bigger than the machine must stop the start-up: {other:?}"
                )
            }
        }
    }

    /// ⛔ WHAT THIS BUYS THAT THE ASSEMBLY PROBE DOES NOT: that the core is STILL SERVING when the
    /// turns run out. The assembly probe reads `Err(TurnLimitReached)`, and that value comes back at
    /// ANY limit because `serve` never finishes — so on its own it cannot tell a hundred thousand
    /// turns from none. ✅ MEASURED, not feared: with the limit cut to `0` and no peer at all, `run`
    /// answers exactly the same `Err(TurnLimitReached)` without one turn of serving — the executor
    /// refuses before the first poll (R4-7) — 2026-09-19. What separates the two runs is what the
    /// PEER heard, never the value of `run`, and that measurement is what makes the assertion below
    /// an oracle rather than a restatement of the loop.
    ///
    /// ⛔ AND IT IS THE PROBE §5 ASKS FOR — "the graph with the GUI stays alive past a hundred
    /// thousand turns" — which is the number the old `EXECUTOR_TURN_LIMIT` stopped at. A daemon that
    /// died there would have died after minutes of ordinary use, silently, with `TurnLimitReached`
    /// nobody reads.
    ///
    /// ⚠️ THE PEER IS JOINED AFTER THE RUN, always: the server end closes when the run's locals
    /// fall, and that close is what ends the peer's read loop. Joining first would deadlock.
    #[test]
    fn the_graph_with_the_gui_stays_alive_past_a_hundred_thousand_turns() {
        let dir = private_dir_for_line(line!());
        let name = socket_name_for_line(line!());

        // ⚠️ THE PEER SPEAKS BEFORE THE RUN STARTS, and that is allowed: the listener exists from
        // `bound()` inside `run_the_graph`, and the helper retries the connect until it does.
        let peer = a_peer_that_says(name.clone(), vec![IpcMessage::Hello(build_stamp())], 1);

        let outcome = run_the_graph(
            Parameters::new(WITH_A_PEER, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),
            &dir.join("journal.redb"),
            &dir.join("layout.redb"),
            &name,
        );

        match outcome {
            Err(StartupError::Run(RunError::TurnLimitReached)) => {}
            other => panic!("the serving loop must still be looping when the turns end: {other:?}"),
        }

        let heard = peer.join().expect("the peer thread does not panic");
        assert!(
            matches!(heard.first(), Some(IpcMessage::Accepted { .. })),
            "the core must have SERVED the peer, which is what the turn count alone cannot say: \
             {heard:?}"
        );
    }

    /// ⛔ THE PROBE OF SEQUENCE 2 OF THE NORTH STAR, end to end on the REAL archive: the GUI saves,
    /// the core stops, the core starts again, and the same package comes back. It is the one probe
    /// that holds the seventh port's whole reason for existing.
    ///
    /// ⛔ TWO RUNS, ONE ARCHIVE PATH, TWO SOCKET NAMES. The path is shared because that is the
    /// claim; the names are not, because a named pipe may linger a moment after its listener falls
    /// on Windows, and a probe that fails only on one of the project's two systems is the red this
    /// repository pays for most (gotcha #52).
    ///
    /// ⚠️ THE FIRST RUN'S PEER WANTS THE WELCOME AND THEN THE ANSWER TO `SaveLayout`, so it asks for
    /// enough messages to reach it; how many the welcome is comes from §5 and is RE-READ rather
    /// than assumed — a wrong count here makes the peer wait for a message that never comes, and the
    /// close ends it with a short vector instead of a hang.
    #[test]
    fn a_layout_saved_is_found_again_after_a_restart() {
        let dir = private_dir_for_line(line!());
        let journal = dir.join("journal.redb");
        let layout = dir.join("layout.redb");
        let package = b"{\"grid\":1}".to_vec();

        let first = socket_name_for_line(line!());
        let saver = a_peer_that_says(
            first.clone(),
            vec![
                IpcMessage::Hello(build_stamp()),
                IpcMessage::SaveLayout(package.clone()),
            ],
            WELCOME + 1,
        );
        let _ = run_the_graph(
            Parameters::new(WITH_A_PEER, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),
            &journal,
            &layout,
            &first,
        );
        let saved = saver.join().expect("the peer thread does not panic");
        assert!(
            saved
                .iter()
                .any(|said| said == &IpcMessage::Layout(LayoutState::Package(package.clone()))),
            "the core answers a save with what it now HOLDS (decision 13): {saved:?}"
        );

        let second = socket_name_for_line(line!());
        let reader = a_peer_that_says(second.clone(), vec![IpcMessage::Hello(build_stamp())], WELCOME);
        let _ = run_the_graph(
            Parameters::new(WITH_A_PEER, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),
            &journal,
            &layout,
            &second,
        );
        let found = reader.join().expect("the peer thread does not panic");
        assert!(
            found
                .iter()
                .any(|said| said == &IpcMessage::Layout(LayoutState::Package(package.clone()))),
            "a RESTARTED core finds the package the previous one kept: {found:?}"
        );
    }

    /// ⛔ DECISION 35, THE WHOLE OF IT: an archive that will not open must NOT stop the start-up, and
    /// every `SaveLayout` must come back "unavailable". A core that refused to start here would be
    /// treating a window layout as authoritative state, which I1 says it is not.
    ///
    /// ⚠️ THE FAILURE IS PROVOKED BY A DIRECTORY THAT IS NOT THERE, the same way
    /// `a_journal_that_cannot_be_opened_stops_the_start_up` provokes its own — one road, two
    /// opposite outcomes, which is what makes the pair say something.
    #[test]
    fn a_layout_archive_that_will_not_open_lets_the_core_start() {
        let dir = private_dir_for_line(line!());
        let name = socket_name_for_line(line!());
        // A directory that is not there: `FileCustody::open` fails, and `MaybeCustody` swallows it.
        let broken = dir.join("not-there").join("layout.redb");

        let peer = a_peer_that_says(name.clone(), vec![IpcMessage::Hello(build_stamp())], WELCOME);
        let outcome = run_the_graph(
            Parameters::new(WITH_A_PEER, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),
            &dir.join("journal.redb"),
            &broken,
            &name,
        );

        match outcome {
            Err(StartupError::Run(RunError::TurnLimitReached)) => {}
            other => panic!("a layout archive that will not open must not stop the start-up: {other:?}"),
        }
        let heard = peer.join().expect("the peer thread does not panic");
        assert!(
            heard.contains(&IpcMessage::Layout(LayoutState::Unavailable)),
            "the welcome must say the layout is UNAVAILABLE, not `Nothing`: {heard:?}"
        );
    }

    /// ⛔ THE OTHER DIRECTION OF `MaybeCustody`, and without it a wrapper that ALWAYS refused would
    /// pass the probe above: the archive that opens must really delegate. ✅ Held by the restart
    /// probe of step 11, which is why this one asserts the REFUSING half only -- said here rather
    /// than left for a reviewer to wonder about.
    #[test]
    fn a_core_started_on_a_broken_archive_answers_unavailable_to_every_save() {
        let dir = private_dir_for_line(line!());
        let name = socket_name_for_line(line!());
        let broken = dir.join("not-there").join("layout.redb");

        let peer = a_peer_that_says(
            name.clone(),
            vec![
                IpcMessage::Hello(build_stamp()),
                IpcMessage::SaveLayout(b"{\"grid\":1}".to_vec()),
            ],
            WELCOME + 1,
        );
        let _ = run_the_graph(
            Parameters::new(WITH_A_PEER, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),
            &dir.join("journal.redb"),
            &broken,
            &name,
        );

        let heard = peer.join().expect("the peer thread does not panic");
        assert_eq!(
            heard.get(WELCOME),
            Some(&IpcMessage::Layout(LayoutState::Unavailable)),
            "the answer to a save on a broken archive is UNAVAILABLE, never a package: {heard:?}"
        );
        assert!(
            !heard
                .iter()
                .any(|said| matches!(said, IpcMessage::Layout(LayoutState::Package(_)))),
            "and no package ever comes back: {heard:?}"
        );
    }

    /// ⛔ THE `unwrap_or` OF D27, MEASURED IN BOTH DIRECTIONS, and the two are different claims.
    /// Empty journal → the DEFAULT of ADR-0006, which is remote and lives HERE as a literal; a
    /// journal carrying a transition → what the transition says, not the default. A `policy_now`
    /// that answered `Remote` on an empty archive would make the two indistinguishable, which is
    /// exactly what D27 refused inside the kernel.
    ///
    /// ⚠️ THE TRANSITION IS WRITTEN THROUGH `Arbiter::set_policy` ON A REAL `FileJournal`, not by
    /// hand: a hand-built record would be this probe agreeing with itself about a format, and the
    /// pair `set_policy`/`policy_now` is one artefact.
    #[test]
    fn the_policy_in_the_journal_is_the_one_the_arbiter_starts_on() {
        let dir = private_dir_for_line(line!());
        let journal = dir.join("journal.redb");
        let layout = dir.join("layout.redb");

        // ⛔ THE FIRST DIRECTION: an EMPTY journal, and the welcome names the default of ADR-0006.
        let first = socket_name_for_line(line!());
        let peer = a_peer_that_says(first.clone(), vec![IpcMessage::Hello(build_stamp())], WELCOME);
        let _ = run_the_graph(
            Parameters::new(WITH_A_PEER, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),
            &journal,
            &layout,
            &first,
        );
        let heard = peer.join().expect("the peer thread does not panic");
        assert!(
            heard.iter().any(|said| matches!(
                said,
                IpcMessage::Policy(report) if report.policy == PolicyName::Remote
            )),
            "an empty journal starts on the default, which is remote: {heard:?}"
        );

        // ⛔ THE SECOND DIRECTION: a transition WRITTEN THROUGH `set_policy` on the real archive,
        // between the two runs, and the next start names it. The arbiter here is a bare one: what
        // is under test is the pair `set_policy`/`policy_now` on the archive, not the two quotas.
        {
            let mut archive = FileJournal::open(&journal).expect("the journal opens between runs");
            let mut arbiter = Arbiter::new(
                Parameters::new(WITH_A_PEER, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),
                VramPolicy::Remote(RemotePolicy),
            );
            arbiter
                .set_policy(VramPolicy::Local(LocalPolicy), StepId::new(1_000), &mut archive)
                .expect("the transition is written");
        }

        let second = socket_name_for_line(line!());
        let peer = a_peer_that_says(second.clone(), vec![IpcMessage::Hello(build_stamp())], WELCOME);
        let _ = run_the_graph(
            Parameters::new(WITH_A_PEER, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),
            &journal,
            &layout,
            &second,
        );
        let heard = peer.join().expect("the peer thread does not panic");
        assert!(
            heard.iter().any(|said| matches!(
                said,
                IpcMessage::Policy(report) if report.policy == PolicyName::Local
            )),
            "a restarted core starts on the policy the journal holds, not on the default: {heard:?}"
        );
    }

    /// ⛔ THE SECOND CORE. `StartupError::Ipc` has exactly one ordinary cause, and a variant with no
    /// probe is a claim nobody checks: two graphs on one socket name, and the second must stop
    /// instead of starting beside the first.
    ///
    /// ⚠️ NO THREAD AND NO RACE (R4-8): the first core is reduced to the one thing that matters, a
    /// listener bound to the name and held for the whole probe. On Windows `interprocess` creates
    /// the first instance with `FILE_FLAG_FIRST_PIPE_INSTANCE`, so a second `bound` on the name is
    /// refused; on Linux the second bind is `EADDRINUSE`. Read in the crate's source, not assumed.
    #[test]
    fn a_second_core_on_the_same_channel_stops_the_start_up() {
        let dir = private_dir_for_line(line!());
        let name = socket_name_for_line(line!());
        let _first = LocalSocketIpc::bound(&name, Progressive::starting_at(0), MAX_BODY)
            .expect("the first listener binds");

        let outcome = run_the_graph(
            Parameters::new(8, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),
            &dir.join("journal.redb"),
            &dir.join("layout.redb"),
            &name,
        );

        match outcome {
            Err(StartupError::Ipc(_)) => {}
            other => panic!("a second core on the same channel must stop at the bind: {other:?}"),
        }
    }

    /// ⛔ THE ROAD INTO `StartupError::Policy`, AND IT IS THE ONE THAT CAN BE PROVOKED. The two
    /// re-reads of the journal name two DIFFERENT failures: `Numbering` means the archive would
    /// not replay at all, and `Policy` means it replayed and a record in it is not one this build
    /// understands. Only the second can be arranged from outside, and it is arranged the way
    /// `crates/kernel/tests/arbiter_policy.rs` already arranges it -- bytes written THROUGH the port,
    /// which takes `&[u8]` and validates nothing (road A4 of `kernel::boundary`).
    ///
    /// ⚠️ THE BYTES GO IN AS AN `intent` AND NOT AS A `note`, and that is measured rather than
    /// stylistic: `note` and `outcome` refuse a step that has no intent yet, `intent` refuses only a
    /// step that already has one. A `note` here would come back `OutOfOrder` and the probe would be
    /// red for the wrong reason.
    ///
    /// ⛔ AND THE OTHER ROAD, `StartupError::Numbering`, IS DECLARED AND NOT PINNED: its trigger is
    /// written beside the variant itself, where a reader of the type finds it, rather than copied
    /// to a second house here (gotcha #68).
    #[test]
    fn a_record_this_build_cannot_read_stops_the_start_up() {
        let dir = private_dir_for_line(line!());
        let journal = dir.join("journal.redb");

        {
            let mut archive = FileJournal::open(&journal).expect("the archive is created");
            archive
                .intent(StepId::new(1), b"not a record of any version")
                .expect("the port takes bytes and validates nothing");
        }

        let outcome = run_the_graph(
            Parameters::new(8, TOTAL_VRAM, ARBITER_ID, Millis::new(0)),
            &journal,
            &dir.join("layout.redb"),
            &socket_name_for_line(line!()),
        );

        match outcome {
            Err(StartupError::Policy(_)) => {}
            other => panic!("a journal this build cannot read must stop the start-up: {other:?}"),
        }
    }
}
