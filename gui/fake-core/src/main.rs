//! The fake core: the REAL serving activity on in-memory ports, plus a faucet.
//!
//! ⛔ WHAT IS FAKE HERE IS ONLY THE FAUCET. The dispatch is `kernel::serving::serve`, the journal
//! and the custody are the in-memory fakes the conformance suites already hold, the ARBITER IS
//! REAL, and the transport is `platform`'s. Nothing in this file branches on an incoming message:
//! the day the daemon changes, this programme must follow it FOR FREE, and a second dispatch here
//! is what §7 of the sub-project 2 design forbids in so many words.
//!
//! ⚠️ IT DOES NOT LIVE ALONGSIDE THE DAEMON: one listener at a time on `SOCKET_NAME`, and whoever
//! starts it knows. What the operating system does with two is not this tool's business.

use std::cell::RefCell;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;

use kernel::arbiter::{
    Admission, Arbiter, ArbiterId, ComputeClass, Mib, Preemption, RemotePolicy, ResourceProfile,
    VramPolicy,
};
use kernel::executor::{Executor, Sleep, nap};
use kernel::numbering::Progressive;
use kernel::parameters::Parameters;
use kernel::ports::journal::{Journal, StepId};
use kernel::ports::reactor::Reactor;
use kernel::record::{EffectClass, Record, RecordV1, RoutingDetail, Trust};
use kernel::serving::{Core, serve};
use kernel::time::{Millis, Monotonic, WallTime};
use kernel::wire::ipc::{IpcMessage, Provenance, Verdict};
use platform::ipc::LocalSocketIpc;
use platform::reactor::SystemReactor;
use platform::rng::SequentialRng;
use simulator::custody::MemoryCustody;
use simulator::journal::MemoryJournal;

/// ⛔ THE SAME NAME THE DAEMON BINDS, AND IT IS A COPY. `crates/daemon/src/main.rs` holds the
/// other one; a binary exports nothing, so it cannot be imported (P-68). ⚠️ NOTHING COUPLES THE
/// TWO: they are compared by a command in this task's closing criteria (D45). If they drift, the
/// gui connects to the WRONG PROGRAMME WITHOUT AN ERROR, because the build stamp is the same.
const SOCKET_NAME: &str = "harness-core";

/// This tool's own cap (D9). ⚠️ It need NOT equal the daemon's: the cap is local to a core, and
/// D31 says so. Only the NAME has to match.
const MAX_BODY: usize = 1024 * 1024;

const TOTAL_VRAM: Mib = Mib::new(16_384);
const AUDIO_QUOTA: Mib = Mib::new(1_024);
const PRESENTATION_QUOTA: Mib = Mib::new(768);
const ARBITER_ID: ArbiterId = ArbiterId::new(0);
const FOR_EVER: Millis = Millis::new(u64::MAX);

/// The turn cadence, and the token cadence, which are ONE number and not two.
///
/// ⛔ ONE CONSTANT DELIBERATELY: `serve` naps a tick per turn and the faucet naps between tokens,
/// and a second constant with the same value would be two numbers nobody compares — P-73 in
/// miniature. §7 asks for "2000 in ten seconds", which is this. ⚠️ The probes hand ZERO instead,
/// through `run_the_graph`, because the reactor here is real.
const GUI_TICK: Millis = Millis::new(5);

/// ⛔ REWRITTEN AND NOT SHARED -- D41, decided by the owner on 2026-09-14. `build_the_arbiter` and
/// `reserve` are private functions of a BINARY and cannot be imported, and no shared home
/// survives (P-68): the kernel may not name a default (ADR-0034, constraint 11 of §11), the crates
/// are fixed at five (constraint 1), a VRAM quota is not an OS concern, and a `lib.rs` in `daemon`
/// would drag the PRODUCTION wiring into a tool.
///
/// ⚠️ THE SAME SHAPE LIVES IN MORE THAN ONE PLACE, AND THE COMMAND SAYS WHERE -- not this line,
/// which named three and was wrong twice (E118): the sub-project 2 campaign of task 10 builds its
/// grants through a generic `standing_grant` helper and does not carry the shape at all, and "each
/// one says so" was true of this house alone. The list is gone and the command stays, which is the
/// rule of `../../../CLAUDE.md` -- a number is not written, the command that produces it is:
/// `grep -rn 'const AUDIO_RESERVATION' crates/ gui/ --include='*.rs' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*//'`
/// -- and the filter is there because the line you are reading NAMES the pattern (E63, E88, E96).
///
/// ⛔ AUDIO FIRST, and it is not arbitrary: both profiles sit in one lane, and the daemon's own
/// comment argues it.
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

fn build_the_arbiter(parameters: Parameters) -> Arbiter {
    // ⛔ `Remote` IS THE DEFAULT OF ADR-0006, and the journal starts EMPTY here, so there is no
    // written transition for the projection of task 8 to re-read. The daemon's re-reading is
    // proven by the daemon's own probe; repeating it here would prove nothing.
    let mut arbiter = Arbiter::new(parameters, VramPolicy::Remote(RemotePolicy));
    for profile in [&AUDIO_RESERVATION, &PRESENTATION_RESERVATION] {
        match arbiter.admit(profile, FOR_EVER, Monotonic::ORIGIN) {
            Admission::Granted(_) => {}
            // ⚠️ A PANIC AND NOT A `StartupError`: this is a TOOL, and an impossible quota here is
            // a defect in this file rather than an operator's misconfiguration.
            Admission::Queued(_) | Admission::Refused { .. } => {
                panic!("the fake core could not reserve {}", profile.name)
            }
        }
    }
    arbiter
}

/// The reactor, shared between the executor -- which takes it BY VALUE -- and `serve`, which wants
/// a BORROW.
///
/// ⛔ A COPY OF THIS SHAPE, AND IT STAYS LOCAL: D34, re-read on 2026-09-14 (P-74). ⚠️ NO ORDINAL
/// AND NO CLOSED LIST HERE -- this census declared itself without carrying its command, which is
/// the one thing E62 had stripped from its siblings and left in the text this task then wrote
/// (E118). What says where they are is the command, over BOTH trees, because `gui/` is a sibling
/// that `crates/`-only greps cannot see:
/// `grep -rn 'struct SharedClock' crates/ gui/ --include='*.rs' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*//'`
/// -- and the filter keeps the line you are reading out of its own count (E63, E88, E96).
///
/// ⛔ A COMMON HOME WOULD HAVE TO BE GENERIC over the reactor, because some of these wrap
/// `VirtualReactor` and others -- the daemon and this one -- wrap `SystemReactor`; D34 keeps the
/// repetition instead, and P-74 re-read that on the merits.
struct SharedClock<'a, R: Reactor>(&'a RefCell<R>);

impl<R: Reactor> Reactor for SharedClock<'_, R> {
    fn now(&self) -> Monotonic {
        self.0.borrow().now()
    }

    fn wall_time(&self) -> WallTime {
        self.0.borrow().wall_time()
    }

    fn wait_until(&mut self, deadline: Monotonic) -> Option<Monotonic> {
        self.0.borrow_mut().wait_until(deadline)
    }
}

/// The text the faucet streams. ⚠️ MARKDOWN WITH A CODE BLOCK, because that is what G4 asks the
/// chat to render, and a stream of plain words would let the renderer be built against something
/// the real one never sends.
const SCRIPT: &[&str] = &[
    "Ecco un esempio.\n\n```rust\nfn main() {\n",
    "    println!(\"ciao\");\n}\n```\n\n",
    "E una riga normale dopo il blocco. ",
];

/// ⛔ EVERY TOKEN IS `Untrusted`, AND IT IS NOT A PRECAUTION: ADR-0014 makes a model's output
/// untrusted, the gui marks it (G13), and a faucet that sent `Trusted` text would let the whole
/// provenance path be built and never exercised.
async fn the_faucet<I, J, C, R>(
    core: &RefCell<Core<I, J, C>>,
    clock: &R,
    sleep: &Sleep,
    // ⛔ DELIVERED AND NOT READ FROM THE CONSTANT (ADR-0034), and here it is not ceremony: the
    // probes mount the REAL `SystemReactor`, so a production cadence would cost cadence × turns of
    // wall clock inside the gate. The probes hand zero, which makes the nap's deadline already
    // reached and the turn polling — the same cure D28 gave task 9.
    cadence: Millis,
    words: Receiver<String>,
) where
    I: kernel::ports::ipc::Ipc,
    J: Journal,
    C: kernel::ports::custody::Custody,
    R: Reactor,
{
    let mut piece = 0usize;
    let mut step = 1_000_000u64;
    loop {
        // ⛔ `try_recv` AND NEVER A BLOCKING READ -- D42. A blocking read inside a task stops the
        // WHOLE executor, because it runs one decision at a time: `serve` would not run, and the
        // fake core would look dead to the gui.
        match words.try_recv() {
            Ok(word) => {
                let mut held = core.borrow_mut();
                match word.trim() {
                    "degrade" => {
                        step += 1;
                        note_a_degraded_routing(held.journal(), StepId::new(step));
                        // ⛔ AND NOTHING ELSE. Telling the gui is the REAL code's job: `serve`
                        // re-reads `degradation_now` and sends `Degradation` only when it changed
                        // (task 7, and its probe `an_unchanged_degradation_is_not_resent`). §7 left
                        // "how it notices" to this plan; task 7 had already fixed it -- P-70.
                    }
                    "verdict" => {
                        let asked = ResourceProfile {
                            name: "faucet-probe",
                            reserved_vram: Mib::new(4_096),
                            compute_class: ComputeClass::Interactive,
                            preemption: Preemption::After(Millis::new(500)),
                        };
                        let now = clock.now();
                        let answer = held.arbiter().admit(&asked, FOR_EVER, now);
                        let verdict = match answer {
                            Admission::Granted(_) => Verdict::Granted,
                            Admission::Queued(_) => Verdict::Queued,
                            Admission::Refused { asked, ceiling } => {
                                Verdict::Refused { asked, ceiling }
                            }
                        };
                        tell_everyone(&mut held, IpcMessage::Verdict(verdict));
                    }
                    other => eprintln!("the faucet knows `degrade` and `verdict`, not `{other}`"),
                }
            }
            Err(TryRecvError::Empty) => {}
            // ⚠️ THE CONSOLE THREAD IS GONE -- stdin closed. The faucet keeps streaming: a fake
            // core started without a terminal is a perfectly good fake core.
            Err(TryRecvError::Disconnected) => {}
        }

        {
            let mut held = core.borrow_mut();
            let text = SCRIPT[piece % SCRIPT.len()];
            piece += 1;
            tell_everyone(
                &mut held,
                IpcMessage::Token {
                    text: text.to_string(),
                    provenance: Provenance::Untrusted,
                },
            );
        }
        // ⛔ THE BORROW IS DROPPED BEFORE THIS LINE, and that is the rule D43 states: two live
        // `borrow_mut` are a panic. ⚠️ THE CLAUSE THAT USED TO JUSTIFY IT IS GONE (E121): it said
        // the panic "only appears when the two tasks interleave -- that is, not in the shortest
        // probe", and the bench underneath falsifies it -- held across the `await`, ALL SIX turn
        // red at once, in 0.00s. The rule stands; the excuse for it did not, which is E33 again.
        let deadline = clock.now().saturating_add(cadence);
        nap(sleep, deadline).await;
    }
}

fn tell_everyone<I, J, C>(core: &mut Core<I, J, C>, message: IpcMessage)
where
    I: kernel::ports::ipc::Ipc,
    J: Journal,
    C: kernel::ports::custody::Custody,
{
    for client in core.attending() {
        // ⚠️ A SEND THAT FAILS IS NOT AN ERROR HERE: the gui is allowed to die at any instant
        // (ADR-0004), and `serve` is the one that reconciles. The faucet just keeps talking.
        let _ = core.ipc().send(client, &message.encode().expect("encode"));
    }
}

fn note_a_degraded_routing<J: Journal>(journal: &mut J, step: StepId) {
    // ⚠️ AN INTENT FIRST, because a note without one is what the port's doc forbids (ADR-0007).
    let intent = Record::V1(RecordV1::intent(
        EffectClass::Idempotent,
        Trust::Instruction,
        Vec::new(),
        "the faucet opened a step so that a routing note has one",
    ))
    .encode();
    journal.intent(step, &intent).expect("the intent");
    let note = Record::V1(RecordV1::routing(
        EffectClass::Idempotent,
        Trust::Instruction,
        Vec::new(),
        "the faucet declared a degraded routing; the REAL code is what reports it",
        RoutingDetail::new("fake-core", 1, true),
    ))
    .encode();
    journal.note(step, &note).expect("the note");
}

/// ⛔ A THREAD OF ITS OWN -- D42, P-69. The executor runs ONE DECISION AT A TIME, so a blocking
/// read inside a task freezes it. The executor's own module doc names this way out for systemic
/// operations: "threads of their own". ⚠️ THE THREAD LIVES HERE AND NOT IN `platform`: an
/// interactive console is not a mechanism of the product.
fn spawn_the_console() -> Receiver<String> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let mut line = String::new();
        while std::io::stdin().read_line(&mut line).unwrap_or(0) > 0 {
            // ⚠️ A SEND THAT FAILS MEANS THE CORE IS GONE: stop, rather than spin on a dead channel.
            if sender.send(line.trim().to_string()).is_err() {
                return;
            }
            line.clear();
        }
    });
    receiver
}

/// The whole graph, with everything a probe has to vary handed in.
///
/// ⛔ SPLIT FROM `main` FOR THE REASON THE DAEMON SPLITS ITS OWN: the gate runs `build` and `test`
/// and never `run`, so a wiring that lived inside `main` would be covered by nothing. ⚠️ AND THE
/// THREE ARGUMENTS ARE THE THREE THINGS A PROBE MUST CHANGE: the NAME, because a socket name is
/// machine-wide and `cargo test` runs in parallel; the TURN LIMIT, because `serve` never ends; and
/// the CADENCE, because the reactor here is real.
fn run_the_graph(name: &str, turns: u64, cadence: Millis, words: Receiver<String>) {
    let parameters = Parameters::new(turns, TOTAL_VRAM, ARBITER_ID, cadence);
    let ipc = LocalSocketIpc::bound(name, Progressive::starting_at(0), MAX_BODY)
        .expect("bind the channel -- is a core already listening on it?");
    let core = RefCell::new(Core::new(
        ipc,
        MemoryJournal::new(),
        MemoryCustody::new(),
        build_the_arbiter(parameters),
        Progressive::starting_at(1),
        parameters,
    ));
    let reactor = RefCell::new(SystemReactor::new());
    // ⚠️ THE DECLARATION ORDER IS LOAD-BEARING, as the daemon says of its own (task 9): `serve` and
    // the faucet BORROW `clock` for the executor's whole life and locals drop in reverse order, so
    // `clock` is declared BEFORE the executor. Measured at the plan review (R5-2, R3-16): a temporary
    // `&SharedClock(&reactor)` inside `spawn` dies at the end of its statement (E0716), and a `let`
    // AFTER the executor does not live long enough (E0597).
    let clock = SharedClock(&reactor);
    let sleep = Sleep::new();

    let mut executor = Executor::new(
        SequentialRng::new(),
        SharedClock(&reactor),
        parameters,
        &sleep,
    );
    executor.spawn(serve(&core, &clock, &sleep));
    executor.spawn(the_faucet(&core, &clock, &sleep, cadence, words));
    // ⛔ `serve` NEVER ENDS, so this only returns when the turns run out. In `main` the limit is
    // `u64::MAX`, that is never in practice: the programme is stopped with Ctrl-C, as §7 says —
    // no clean shutdown in this sub-project (ADR-0007). ⚠️ THE VALUE OF `run` IS NOT AN ORACLE, and
    // no probe reads it: what a probe reads is the PEER.
    let _ = executor.run();
}

fn main() {
    println!("fake core listening on {SOCKET_NAME}; words: degrade, verdict");
    run_the_graph(SOCKET_NAME, u64::MAX, GUI_TICK, spawn_the_console());
}

#[cfg(test)]
mod tests {
    use super::*;

    // ⛔ ONLY THE PROBES MINT THE STAMP (R5-12): imported here and not at the top of the file,
    // where `cargo build` would report it unused -- `cargo test` compiles the test target alone
    // and would never show the warning.
    use kernel::wire::ipc::build_stamp;

    // ⛔ WHAT THE SEALED MODULE LETS OUT, AND IT IS THREE AND NOT TWO -- this comment said two and
    // "there is no third", and `settling::Peer::new(...)` compiles, measured. The third is
    // `Peer::new`, and it MUST be reachable because `a_peer_that_says` lives out here and has to
    // wrap its own thread. What the seal buys is narrower and true: `Peer` carries no way to
    // REACH the thread inside it (`peer.0` is `E0616`), so `collect_until` is the only place a
    // peer can be settled. See the doc on `settling`.
    use settling::{Peer, Stopped, collect_until};

    // ⛔ EACH PROBE BINDS A NAME OF ITS OWN. A socket name is valid for the whole machine and
    // `cargo test` runs in parallel by default: two probes on `SOCKET_NAME` make the second fail
    // with "already in use", which is a red in the wrong probe's box. The lesson is task 9's.
    fn a_name_for(what: &str) -> String {
        format!("fake-core-probe-{what}")
    }

    /// How many messages the core sends after a valid `Hello` -- the welcome of sequence 1.
    ///
    /// ⛔ MEASURED ON THE DISPATCH OF TASK 7, `greet` in `crates/kernel/src/serving.rs`, and pinned
    /// by task 9's `WELCOME` with the same reading: the day the welcome grows, task 7's own probe
    /// goes red before this constant does.
    const WELCOME: usize = 5;

    /// The turn budget of every probe here -- and every one of them has a peer.
    ///
    /// ⛔ IT IS WALL CLOCK IN DISGUISE, task 9's lesson (R4-6, R4-7): the listener lives only
    /// inside `run_the_graph`, and the peer connects from a thread the OS schedules when it likes,
    /// so a short run can end before the peer ever knocks. A hundred thousand turns at a zero tick
    /// is a fraction of a second of polling. Realigned at the plan review, 2026-09-15: the first
    /// draft copied task 9's peer BEFORE that task was corrected, with budgets of four and eight
    /// thousand turns.
    const WITH_A_PEER: u64 = 100_001;

    /// A stamp that is NOT ours. ⛔ `BuildStamp` HAS NO PUBLIC CONSTRUCTOR, and that is deliberate
    /// (task 3: "a stamp anyone can mint from any number is a stamp that proves nothing"), so the
    /// wrong one is taken from where one already exists — the `StaleBuild` of the canonical set,
    /// which holds a value chosen precisely for not being the real one.
    ///
    /// ⛔ AND THE ASSERTION IS NOT DECORATION, which is the half this helper was born without
    /// (E119): the day that literal ever coincided with a real stamp, the probe below would be
    /// exercising the ACCEPTING path while claiming to test the refusing one. Today the two
    /// differ -- measured -- and that is the direction nothing else was proving.
    ///
    /// ⚠️ THE TWIN IN `crates/kernel/tests/serving.rs` COULD NOT BE REUSED, and the doc that
    /// stood here said it could: an integration test is a crate of its own and exports nothing to
    /// another binary. The copy is forced; what was avoidable was losing the guard with it.
    fn a_stamp_that_is_not_ours() -> kernel::wire::ipc::BuildStamp {
        let stamp = kernel::wire::ipc::stamp_set()
            .into_iter()
            .find_map(|message| match message {
                IpcMessage::StaleBuild(stamp) => Some(stamp),
                _ => None,
            })
            .expect("the canonical set holds a StaleBuild");
        assert_ne!(stamp, build_stamp(), "the canonical StaleBuild must not carry this build's own stamp");
        stamp
    }

    /// The keyboard of a probe: the words the faucet reads, and the hand that types them.
    ///
    /// ⛔ THE SENDING END IS HANDED BACK, NEVER DROPPED HERE (R5-4, R5-6, 2026-09-15): a word is
    /// typed by the PEER once the welcome has arrived -- typed before the run it would be spent on
    /// the faucet's first turn, when `attending()` is still empty, and reach nobody -- and the last
    /// probe keeps the hand alive across the run, so that `try_recv` answers `Empty`: the one case
    /// in which a blocking `recv` would wait for ever.
    fn a_keyboard() -> (mpsc::Sender<String>, Receiver<String>) {
        mpsc::channel()
    }

    /// How long a streaming probe waits for its predicate before it gives a verdict on what came.
    ///
    /// ⛔ IT IS THE PROBE'S CLOCK AND NOT THE PEER'S, and that is forced: the peer sits in a
    /// blocking `Read::read`, so a deadline at the head of ITS loop never fires -- E112 falsified
    /// `set_nonblocking` as the way round, and it drops all six.
    const A_CEILING: std::time::Duration = std::time::Duration::from_secs(5);

    /// How long a probe lets the GRAPH run before it gives a verdict instead of waiting.
    ///
    /// ⛔ A SECOND CEILING, AND IT IS NOT THE SAME ONE (`C-2` of the third review). `A_CEILING`
    /// bounds the wait on the PEER, which happens AFTER `run_the_graph` returns -- so it bounds
    /// nothing at all when the graph itself is what never ends. And the graph not ending is not a
    /// hypothesis: it is precisely the mutation `the_faucet_keeps_streaming_with_no_word_waiting`
    /// exists to catch (P-69, D42). Measured 2026-09-21 with `try_recv` turned into a blocking
    /// `recv`: the executor stops, `executor.run()` never returns, and the WHOLE bench was killed
    /// at 120 s without a single probe reaching a verdict. ⚠️ The sentence that said so was
    /// already written here -- *«and every probe above would hang rather than fail»* -- and the
    /// first round DELETED it instead of reading it as the defect report it was (gotcha 129).
    ///
    /// ⚠️ IT IS LONGER THAN `A_CEILING` ON PURPOSE: the graph is the thing under test and runs
    /// `WITH_A_PEER` turns, which costs well under a second per probe (the whole suite is ~2,3 s),
    /// so ten seconds is a margin of an order of magnitude and still bounds a pathological suite
    /// to a minute. ⛔ AND IT IS A PROBE'S CEILING, NOT THE PROGRAMME'S: `main` runs the same
    /// graph with `u64::MAX` turns and must NOT have one -- there the run ends with Ctrl-C (§7).
    const A_GRAPH_CEILING: std::time::Duration = std::time::Duration::from_secs(10);

    /// Run the graph to its last turn, or give a verdict saying it did not get there.
    ///
    /// ⛔ THE GRAPH RUNS ON A THREAD OF ITS OWN ONLY SO THAT THE WAIT CAN BE BOUNDED: there is no
    /// other way to put a deadline on a call that does not take one. What the probe observes is
    /// unchanged -- it still waits for the graph to finish before it collects.
    ///
    /// ⚠️ THE NEW OUTCOMES ARE TWO AND NOT ONE, and the first draft of this doc said one: the red
    /// that used to be a hang, AND the graph thread that dies, which has a verdict of its own and
    /// must not be dressed up as the first. ⛔ The thread is left behind when the ceiling expires,
    /// and the sentence that used to excuse it -- *«the process is ending anyway with a failed
    /// probe»* -- was FALSE: measured, after the first red the run went on to five more probes
    /// with their abandoned graphs still alive. What makes it harmless is narrower and measured:
    /// the six probes bind six DIFFERENT socket names, so an abandoned graph steals no name.
    // ⛔ `#[track_caller]` FOR THE SAME REASON AS `collect_until`, and it was missing here while
    // the wave that added it there wrote the argument out: this helper panics too, and a red that
    // names the helper instead of the probe is a red nobody can act on.
    #[track_caller]
    fn run_the_graph_within(name: &str, turns: u64, cadence: Millis, words: Receiver<String>) {
        let name = name.to_owned();
        let (ended, done) = mpsc::channel();
        std::thread::spawn(move || {
            run_the_graph(&name, turns, cadence, words);
            let _ = ended.send(());
        });
        // ⛔ THE TWO FAILURES ARE NOT THE SAME ONE, and the first draft of this helper told them
        // apart nowhere: `is_ok()` is false for a `Disconnected` too, so a graph thread that
        // PANICKED was reported as an executor that is STUCK -- in 0,00 s, with both halves of
        // the sentence false, and it buried the `expect("bind the channel -- is a core already
        // listening on it?")` that task 9 put there on purpose. ⛔ IT IS THE SAME SPECIES AS
        // `E119`, AND IT REOPENED FOUR TIMES: the wait now goes through `wait_with_a_bottom`,
        // which answers with `Stopped` and forces all three arms to be written.
        match settling::wait_with_a_bottom(&done, A_GRAPH_CEILING, |()| true) {
            Stopped::Satisfied => {}
            Stopped::Ceiling => panic!(
                "the graph did not reach its last turn within {A_GRAPH_CEILING:?}: the executor \
                 is stuck, so no probe below could ever give a verdict (C-2)"
            ),
            Stopped::OtherEndGone => panic!(
                "the graph thread DIED before its last turn: its own panic is printed above this \
                 line and IS the verdict -- read that, not this (E119's species)"
            ),
        }
    }

    /// Collect what the peer streams until `enough` is satisfied, or the ceiling passes -- and
    /// then settle with the peer.
    ///
    /// ⛔ THE HALF OF E110'S CURE THAT WAS MISSED (E116): `a_peer_that_says` says why nothing
    /// closes the connection, so a probe whose predicate can become unsatisfiable must NOT `join`
    /// -- the peer loops for ever and `join` never returns. Here the wait is BOUNDED, and the
    /// assertion after it gives a verdict on what arrived.
    ///
    /// ⛔ AND THE PEER IS TAKEN BY VALUE ON PURPOSE, which is the cure of the SECOND round: the
    /// first draft returned the vector and left the caller to `drop(peer)`, so every caller had to
    /// remember a rule -- and `collect_until` itself then swallowed a peer that had PANICKED,
    /// which is `E119 (2)` reintroduced by the cure of `E116`. Measured: with the peer mutated to
    /// die, three probes passed while their peer was panicking. Now the wrong thing is not
    /// expressible: whoever collects hands the peer over, and the settling happens in one place.
    ///
    /// ⛔ THE SETTLING IS CONDITIONAL, AND THE CONDITION IS FREE. If `enough` holds, the peer has
    /// already left its own loop -- it streams a message BEFORE testing its predicate, so it exits
    /// at the next test -- and `join` returns AT ONCE, propagating a panic if there was one. Only
    /// when the ceiling expired is the peer still running, and only then is it dropped. ✅ PROVED
    /// IN BOTH DIRECTIONS, 2026-09-21: as the code stands, six of six; with the peer mutated to
    /// die as soon as its predicate holds, the probes that used to pass IN SILENCE go red.
    ///
    /// ⚠️ IT READS A CHANNEL AND NOT A SOCKET, which is the only reason a ceiling is enforceable
    /// at all: `recv_timeout` takes one, `Read::read` does not.
    ///
    /// ⚠️ AND IT IS NOT THE LOOP IN `a_stale_stamp_is_refused_and_then_silence`: there the WAIT IS
    /// THE OBSERVATION, so a timeout is the SUCCESS and a `Disconnected` is a failure. Here a
    /// timeout is the failure. ⛔ A `Disconnected` BEFORE `enough` HOLDS IS NOT A NORMAL END --
    /// the clause that used to say so was false (E33): it means the peer's THREAD is over, panic
    /// included. A peer that died AFTER satisfying is caught by the `join`.
    ///
    /// ⛔ AND THE OTHER HALF, WHICH THE ROUND BEFORE GOT WRONG (`C-1` of the third review): THE
    /// ASSERTION THAT FOLLOWS MUST BE AT LEAST AS STRONG AS `enough`. A peer that dies BEFORE
    /// satisfying leaves a short vector and is dropped in silence, so the only thing left to tell
    /// anyone is the assertion -- and if it asks for LESS than the predicate did, the short vector
    /// passes it and the probe goes green over a corpse. Measured 2026-09-21: with every peer
    /// mutated to die after its first token, `the_tokens_arrive_untrusted` -- predicate three
    /// tokens, assertion one -- was GREEN. ⚠️ NO WALL TIME HERE: the figure lives in `E124`, in
    /// one house, and it was already written two ways in the commit that measured it. The
    /// ceiling does not remove the hang on its own: without this half it converts the hang into
    /// a silent green -- which is why the verdict is now given in the `else` below, for every
    /// caller, instead of being asked of each one.
    mod settling {
        use super::*;

        /// Why a wait with a bottom ended. ⛔ THREE OUTCOMES, AND NEVER TWO.
        ///
        /// ⛔ THIS TYPE EXISTS BECAUSE ONE SPECIES REOPENED THREE TIMES IN THIS FILE, always in a
        /// place nobody was looking at: `E119` on the stale probe, `E123` in `collect_until`,
        /// `E135` in the graph helper -- and then a FOURTH time in `collect_until` again, where a
        /// cure written for a different defect put back an `Err(_) => break` that merged the two
        /// failures and then ASSERTED the wrong one, saying *«the ceiling expired»* over a run of
        /// 0,27 s in which the peer had died. Each round the shape was "remember to split the
        /// error", and each round somebody did not. ⛔ NOW SPLITTING IS THE ONLY WAY TO COMPILE:
        /// the wait is done in one place, it answers with this enum, and a caller must match all
        /// three arms. ⚠️ A ceiling that expires and an other end that is GONE mean opposite
        /// things -- one is "nothing came in time", the other is "there is nobody left to come",
        /// panic included -- and a red that names the wrong one sends the reader hunting a defect
        /// that is not there.
        pub(super) enum Stopped {
            /// The predicate held.
            Satisfied,
            /// The ceiling passed with the predicate unmet, and the other end is still running.
            Ceiling,
            /// The other end's thread is OVER -- a panic counts, and the channel says so at once.
            OtherEndGone,
        }

        /// The ONE bounded wait of this bench, and the only place `recv_timeout` is called.
        ///
        /// ⛔ GENERIC OVER WHAT ARRIVES because the two waits carry different payloads -- messages
        /// from the peer, and a single `()` from the graph -- and a second copy is what let the
        /// two drift apart in the first place.
        pub(super) fn wait_with_a_bottom<T>(
            arrivals: &Receiver<T>,
            ceiling: std::time::Duration,
            mut enough: impl FnMut(T) -> bool,
        ) -> Stopped {
            let started = std::time::Instant::now();
            loop {
                let Some(left) = ceiling.checked_sub(started.elapsed()) else {
                    return Stopped::Ceiling;
                };
                match arrivals.recv_timeout(left) {
                    Ok(item) => {
                        if enough(item) {
                            return Stopped::Satisfied;
                        }
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => return Stopped::Ceiling,
                    Err(mpsc::RecvTimeoutError::Disconnected) => return Stopped::OtherEndGone,
                }
            }
        }

        /// The peer's thread, sealed: a probe can hand it over or let it fall, and nothing else.
        ///
        /// ⛔ THE FIELD IS PRIVATE TO THIS MODULE, and that is the whole point of the module
        /// existing. The round before wrote *«now the wrong thing is not expressible»* while
        /// `a_peer_that_says` still handed back a bare `JoinHandle`: a seventh probe that joined
        /// its own peer COMPILED, measured 2026-09-21 (`I-1` of the third review). A rule that
        /// every caller must remember is what failed twice here; the cure is that the compiler
        /// refuses, not that the doc asks.
        pub(super) struct Peer(std::thread::JoinHandle<Vec<IpcMessage>>);

        impl Peer {
            pub(super) fn new(thread: std::thread::JoinHandle<Vec<IpcMessage>>) -> Self {
                Self(thread)
            }
        }

        // ⛔ `#[track_caller]` SO THAT THE RED NAMES THE PROBE and not this line: the verdict below
        // is given here for every caller, and a red that points at the helper is a red nobody can
        // act on.
        #[track_caller]
        pub(super) fn collect_until(
            arrivals: &Receiver<IpcMessage>,
            peer: Peer,
            enough: Until,
        ) -> Vec<IpcMessage> {
            let mut heard = Vec::new();
            // ⛔ A PREDICATE THAT ALREADY HOLDS ON NOTHING IS NOT AN ORACLE, and since the wait
            // below only tests it after something has arrived, this is the one assumption the
            // shape makes. It is asserted rather than assumed, and no probe has ever had one.
            assert!(
                !enough(&heard),
                "this predicate holds on an empty vector, so it says nothing about the run"
            );
            let stopped = wait_with_a_bottom(arrivals, A_CEILING, |message| {
                heard.push(message);
                enough(&heard)
            });
            // ⛔ AND THE VERDICT IS GIVEN HERE, FOR EVERY CALLER, instead of being a rule each one
            // must remember. The rule -- *the assertion after the ceiling must be at least as
            // strong as the predicate* -- was once written in this doc and left to the callers to
            // honour, which is the shape `E127` itself calls "a defect waiting to happen":
            // measured, a probe pairing `len >= 10` with `assert!(!heard.is_empty())` stayed
            // GREEN. Now an unmet predicate is red wherever it happens, and a caller cannot
            // weaken it because it never gets the short vector back.
            match stopped {
                Stopped::Satisfied => {
                    peer.0.join().expect("the peer thread");
                }
                Stopped::Ceiling => {
                    drop(peer);
                    panic!(
                        "the ceiling of {A_CEILING:?} expired with the predicate still unmet \
                         after {} messages, and the peer was STILL RUNNING: nothing arrived in \
                         time, which is not the same as nobody being left to send",
                        heard.len()
                    );
                }
                // ⛔ THE ARM THE FIRST THREE ROUNDS DID NOT HAVE. Joining here cannot hang: the
                // channel closed because the peer's closure ended, so the thread is already over.
                Stopped::OtherEndGone => {
                    let how = peer.0.join();
                    panic!(
                        "the peer's thread ENDED after {} messages with the predicate still \
                         unmet{}",
                        heard.len(),
                        if how.is_err() {
                            " -- it PANICKED, and its own message is printed above this line and \
                             IS the verdict: read that, not this"
                        } else {
                            " without panicking, which means it stopped reading on its own"
                        }
                    );
                }
            }
            heard
        }
    }

    /// What a probe waits for before it stops reading.
    ///
    /// ⛔ A PREDICATE AND NOT A COUNT (R5-4): the faucet streams a token per turn, so a fixed
    /// number of messages fills up with tokens before the one the probe is after has been sent.
    ///
    /// ⚠️ WHETHER A PROBE MAY `join` ITS PEER IS DECIDED IN `a_peer_that_says`, NOT HERE. The
    /// sentence that used to stand at this spot answered it, and answered it wrongly (E117).
    type Until = fn(&[IpcMessage]) -> bool;

    /// The peer, in the shape task 9 gives it and for its reasons.
    ///
    /// ⛔ IT CAN HANG, AND MEASURING IT IS WHAT E110 AND THEN E116 COST. Nothing closes this
    /// connection: `Core::forget` drops the client from the CORE's table and never from the
    /// transport, which keeps `Connected` and the detached reader thread `accept` spawned for it.
    /// `LocalSocketIpc::drop_client` is PRIVATE TO THE TRANSPORT, which calls it on its own
    /// failures -- a `write_all` that errs, a peer whose channel ended -- and its own doc says the
    /// reader thread is not told; nothing in the CORE can call it at all, because "the core
    /// closes" is not an operation of the port (decision 22). ⛔ A PROBE THAT WAITS FOR A MESSAGE
    /// THE CORE WILL NEVER SEND THEREFORE HANGS FOR EVER, which is the worst red there is (E5,
    /// E6): it prints nothing, it cannot be bisected, and it eats the gate.
    ///
    /// ⛔ AND "THE TOKEN STREAM SATISFIES IT" IS THE WRONG TEST -- it is the one this paragraph
    /// used to apply, and it blessed probes that hang. What decides is whether the core can STOP
    /// satisfying the predicate under a mutation: a predicate that only the REAL code satisfies
    /// becomes unsatisfiable the moment that code breaks, which is precisely the run in which the
    /// probe must give a VERDICT.
    ///
    /// ⛔ THE STREAM OF TOKENS IS NOT AN EXEMPTION, AND BELIEVING IT WAS COST A SECOND ROUND.
    /// The faucet is CODE UNDER TEST like any other, so "three tokens arrive" is unsatisfiable in
    /// exactly the run that matters -- and if a stale `Hello` is wrongly refused the client never
    /// reaches `attending()`, so the tokens stop for the WELCOME probe too. Measured: under one
    /// mutation of the faucet, and under one of the kernel's stamp check, three probes that still
    /// joined ran past sixty seconds and were killed. ⛔ THEREFORE EVERY CALLER THAT WAITS ON A
    /// PREDICATE HANDS `as_they_come` AND READS IT THROUGH `collect_until`; there is no exception,
    /// and the rule that used to carve one out is gone. The only probe that does not is the one
    /// waiting for SILENCE, whose ceiling IS its observation.
    ///
    /// ⚠️ THE CONNECT IS A `yield_now` LOOP AND NOT A SLEEP: the listener exists from `bound()`,
    /// which happens inside the run, so this thread may be scheduled first. ⛔ AND THE LOOP HAS A
    /// WALL-CLOCK DEADLINE (task 9, R4-6): if the run ends before this thread connects, `connect`
    /// fails FOR EVER and a bare loop would hang the gate -- the worst red there is. Five seconds
    /// is an order of magnitude above any scheduling delay, and the panic names this line.
    ///
    /// ⚠️ `then_types` IS THE HAND ON THE KEYBOARD (R5-4): once the welcome is in, the peer types
    /// that one word and lets the hand go. Before the welcome nobody is attending, and a word
    /// typed then is spent on nobody.
    fn a_peer_that_says(
        name: String,
        said: Vec<IpcMessage>,
        until: Until,
        mut then_types: Option<(mpsc::Sender<String>, &'static str)>,
        // ⛔ THE STREAM, AND EVERY PROBE HANDS ONE (E110, E116, and the two rounds after). ⚠️ IT
        // WAS AN `Option` UNTIL THE THIRD REVIEW, kept "only because the signature would
        // otherwise lie about a future caller that wants the returned vector" -- but no such
        // caller existed. ⚠️ AND THE REASON FIRST WRITTEN HERE WAS WRONG: it said the option was
        // "the last way left to write the probe that joins its own peer", and it was not -- what
        // closes that probe is the SEAL on `Peer`, measured to hold with the option in place --
        // `E0599` on `join`, `E0616` on the field. Dropping the option is a plain simplification,
        // and all six callers passed `Some`. ⚠️ NO TALLY HERE, deliberately: the
        // one that stood in this comment aged inside a single task.
        as_they_come: mpsc::Sender<IpcMessage>,
    ) -> Peer {
        Peer::new(std::thread::spawn(move || {
            use interprocess::local_socket::{GenericNamespaced, Stream, prelude::*};
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
                             ended before this thread got to the listener (task 9, R4-6)"
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
            while !until(&heard) {
                match stream.read(&mut chunk) {
                    Ok(0) => break,
                    Ok(read) => buffer.extend_from_slice(&chunk[..read]),
                    Err(_) => break,
                }
                // ⚠️ `take_frame` AND NOT `unframe`: the buffer ordinarily holds a frame and a half,
                // which `unframe` refuses by design (P-13). It hands back where the next frame starts,
                // and `IpcMessage::decode` is given the WHOLE frame, envelope included, because
                // `decode` unframes what it is given. Measured at the plan review (R5-3, R4-2):
                // `decode(body)` answered `Err` on every message, in all six probes.
                while let Some((_, next)) = kernel::framing::take_frame(&buffer) {
                    let message =
                        IpcMessage::decode(&buffer[..next]).expect("the core sends what it says");
                    let _ = as_they_come.send(message.clone());
                    heard.push(message);
                    buffer.drain(..next);
                }
                if heard.len() >= WELCOME {
                    if let Some((hand, word)) = then_types.take() {
                        hand.send(word.to_string()).expect("the peer types");
                    }
                }
            }
            heard
        }))
    }

    #[test]
    fn the_welcome_gives_the_sequence_one() {
        let name = a_name_for("welcome");
        let (_hand, words) = a_keyboard();
        // ⛔ IT STREAMS LIKE EVERY OTHER, and here the reason is NOT obvious -- which is why the
        // first round missed it. The welcome is what the KERNEL's `greet` sends, so a valid
        // `Hello` wrongly refused leaves the client out of `attending()`: no welcome, and no
        // tokens either, so nothing ever reaches five. Measured 2026-09-21 with the kernel's stamp
        // check inverted: this probe ran past sixty seconds and was killed.
        let a_full_welcome: Until = |heard| heard.len() >= WELCOME;
        let (as_they_come, arrivals) = mpsc::channel();
        let peer = a_peer_that_says(
            name.clone(),
            vec![IpcMessage::Hello(build_stamp())],
            a_full_welcome,
            None,
            as_they_come,
        );
        run_the_graph_within(&name, WITH_A_PEER, Millis::new(0), words);
        let heard = collect_until(&arrivals, peer, a_full_welcome);
        // ⚠️ THE FIRST FIVE: one `read` may carry the welcome AND the first token, and the frames
        // are drained together.
        let kinds: Vec<&str> = heard
            .iter()
            .take(WELCOME)
            .map(|message| match message {
                IpcMessage::Accepted(_) => "Accepted",
                IpcMessage::Degradation(_) => "Degradation",
                IpcMessage::Policy(_) => "Policy",
                IpcMessage::Layout(_) => "Layout",
                IpcMessage::Steps(_) => "Steps",
                other => panic!("the welcome sent something else: {other:?}"),
            })
            .collect();
        assert_eq!(
            kinds,
            ["Accepted", "Degradation", "Policy", "Layout", "Steps"],
            "sequence 1, in order: {heard:?}"
        );
        // ⛔ AND THE NUMBERS OF THE `Policy`, not only its place (R5-7, 2026-09-15): without these
        // two lines the mutation G2 -- the presentation quota gone from `build_the_arbiter` -- stays
        // green, because the welcome still has five messages in the right order. The literals are
        // this file's own, which is what D41 buys.
        let report = heard
            .iter()
            .find_map(|message| match message {
                IpcMessage::Policy(report) => Some(*report),
                _ => None,
            })
            .expect("the welcome carries a Policy");
        assert_eq!(report.allocated, Mib::new(1_024 + 768), "the two permanent quotas: {report:?}");
        assert_eq!(report.total, TOTAL_VRAM, "the machine, delivered: {report:?}");
    }

    #[test]
    fn a_stale_stamp_is_refused_and_then_silence() {
        let name = a_name_for("stale");
        let (_hand, words) = a_keyboard();
        // ⛔ THE SECOND MESSAGE IS THE PROBE, as task 7 argues: "the core closes" is not an
        // operation of the port (decision 22), so only a message sent AFTER the refusal tells
        // "it stopped listening" apart from "it had nothing more to say".
        //
        // ⛔ AND THAT IS WHY THIS ONE PROBE STREAMS INSTEAD OF BEING JOINED (E110): the second
        // message never comes -- which is the point -- and nothing closes the connection, so a
        // `join` here waits FOR EVER. Measured 2026-09-20: `run_the_graph` returns, `peer.join()`
        // does not.
        let (as_they_come, arrivals) = mpsc::channel();
        let peer = a_peer_that_says(
            name.clone(),
            vec![
                IpcMessage::Hello(a_stamp_that_is_not_ours()),
                IpcMessage::Invoke(kernel::wire::ipc::Call {
                    function: "set-policy".to_string(),
                    argument: "local".to_string(),
                }),
            ],
            |heard| heard.len() >= 2,
            None,
            as_they_come,
        );
        run_the_graph_within(&name, WITH_A_PEER, Millis::new(0), words);
        // ⛔ A BOUNDED COLLECT AND NOT A `join`: THE WAIT IS THE OBSERVATION, and two seconds of
        // nothing is what "and then silence" means. ⚠️ The peer thread is left dangling on
        // purpose -- it dies with the test binary -- and the cost is declared rather than hidden.
        let mut heard = Vec::new();
        loop {
            match arrivals.recv_timeout(std::time::Duration::from_secs(2)) {
                Ok(message) => heard.push(message),
                // ⛔ THE TIMEOUT IS THE SUCCESS: it is the silence itself.
                Err(mpsc::RecvTimeoutError::Timeout) => break,
                // ⛔ A DEAD PEER IS NOT SILENCE, and treating it as one made this probe GREEN on
                // the very case its name promises (E119). With the sender gone `recv_timeout`
                // answers AT ONCE, the collect ends with what it had, and the `drop(peer)` below
                // throws the `JoinHandle` away -- so the peer's PANIC never surfaces. Measured by
                // the review with a `panic!` in the peer after the first message: green in 0,08 s
                // instead of the two seconds this comment claims. The reachable way in is the
                // `IpcMessage::decode(...).expect(...)` of `a_peer_that_says`, i.e. a core that
                // sends a SECOND, malformed frame.
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    panic!("the peer ENDED after {} messages instead of falling silent", heard.len())
                }
            }
        }
        drop(peer);
        assert_eq!(
            heard,
            vec![IpcMessage::StaleBuild(build_stamp())],
            "a stale gui is told the EXPECTED stamp, and then nothing at all: {heard:?}"
        );
    }

    #[test]
    fn degrade_makes_the_real_code_report_it() {
        let name = a_name_for("degrade");
        let (hand, words) = a_keyboard();
        // ⛔ THE WORD IS TYPED BY THE PEER AFTER THE WELCOME (R5-4, 2026-09-15): typed before the
        // run it would be consumed on the faucet's first turn, when `attending()` is still empty,
        // and the welcome's own `Degradation` would already carry it -- green for the wrong reason.
        // ⛔ THE ORACLE IS THE SECOND `Degradation`, not the first: the welcome always sends one.
        // What the word buys is a SECOND one, which task 7's code sends only because the value
        // CHANGED — so this probe exercises the real rule rather than a branch of the faucet.
        //
        // ⛔ AND FOR THAT REASON IT STREAMS INSTEAD OF BEING JOINED (E116): the predicate asks the
        // core for the very message the mechanism under test produces, so the run in which that
        // mechanism breaks is the run in which the predicate can never be met -- and a `join`
        // there waits FOR EVER. Measured 2026-09-20: `timeout 90 cargo test degrade_makes` came
        // back `143` after "has been running for over 60 seconds".
        //
        // ⚠️ ONE PREDICATE HANDED TO BOTH, not two copies: the peer stops on it and the collector
        // waits for it, and two spellings of the same rule are two things nobody compares.
        let two_degradations: Until = |heard| {
            heard
                .iter()
                .filter(|message| matches!(message, IpcMessage::Degradation(_)))
                .count()
                >= 2
        };
        let (as_they_come, arrivals) = mpsc::channel();
        let peer = a_peer_that_says(
            name.clone(),
            vec![IpcMessage::Hello(build_stamp())],
            two_degradations,
            Some((hand, "degrade")),
            as_they_come,
        );
        run_the_graph_within(&name, WITH_A_PEER, Millis::new(0), words);
        let heard = collect_until(&arrivals, peer, two_degradations);
        let degraded: Vec<bool> = heard
            .iter()
            .filter_map(|message| match message {
                IpcMessage::Degradation(report) => Some(report.routing_degraded),
                _ => None,
            })
            .collect();
        // ⛔ THE DIAGNOSTIC IS THE COUNT AND THE DECIDING VALUES, NEVER `{heard:?}`: five seconds
        // of tokens printed whole made a 7,5 MB red, which is unreadable and therefore useless.
        assert_eq!(
            degraded,
            [false, true],
            "the welcome's Degradation is clean, and the word buys a SECOND one from the REAL code (heard {} messages in all)",
            heard.len()
        );
    }

    #[test]
    fn verdict_reaches_the_peer() {
        let name = a_name_for("verdict");
        let (hand, words) = a_keyboard();
        // ⛔ TYPED AFTER THE WELCOME, for the reason `degrade` states (R5-4): a `Verdict` sent to an
        // empty `attending()` reaches nobody, and the word is spent.
        //
        // ⛔ AND IT STREAMS FOR THE REASON `degrade` STREAMS (E116): the predicate asks for the one
        // message the mechanism under test produces, so it is unsatisfiable exactly when that
        // mechanism breaks. Measured 2026-09-20: `timeout 90 cargo test verdict_reaches` came back
        // `143`, not a red.
        let a_verdict: Until =
            |heard| heard.iter().any(|message| matches!(message, IpcMessage::Verdict(_)));
        let (as_they_come, arrivals) = mpsc::channel();
        let peer = a_peer_that_says(
            name.clone(),
            vec![IpcMessage::Hello(build_stamp())],
            a_verdict,
            Some((hand, "verdict")),
            as_they_come,
        );
        run_the_graph_within(&name, WITH_A_PEER, Millis::new(0), words);
        let heard = collect_until(&arrivals, peer, a_verdict);
        // ⛔ THE COUNT AND THE DECIDING VALUE, NEVER `{heard:?}` -- see `degrade` for what that
        // cost. ⚠️ AND THE ASSERTION BELOW IS NOW A RESTATEMENT, NOT THE ORACLE: since
        // `collect_until` gives the verdict itself when the predicate does not hold, reaching
        // this line already means a `Verdict` arrived. It is kept because it names, in the probe,
        // what the probe is about -- but what fails a broken run is the predicate, upstream.
        let verdicts =
            heard.iter().filter(|message| matches!(message, IpcMessage::Verdict(_))).count();
        assert!(
            verdicts >= 1,
            "the word `verdict` asks the REAL arbiter and the answer reaches the gui (heard {} messages, {verdicts} of them verdicts)",
            heard.len()
        );
    }

    #[test]
    fn the_tokens_arrive_untrusted() {
        let name = a_name_for("tokens");
        let (_hand, words) = a_keyboard();
        // ⛔ ONE NUMBER, TWO USES, AND IT IS THE CURE OF `C-1`: the predicate asked for three and
        // the verdict below asked for one, so a peer that died after the first token left a short
        // vector that the verdict still accepted -- green over a corpse. ⚠️ THE WALL TIME LIVES
        // IN `E124` AND NOWHERE ELSE: it stood in two places at once and the two disagreed. The
        // rule is on `settling`; here the two cannot drift apart because there is one number.
        const ENOUGH_TOKENS: usize = 3;
        let three_tokens: Until = |heard| {
            heard
                .iter()
                .filter(|message| matches!(message, IpcMessage::Token { .. }))
                .count()
                >= ENOUGH_TOKENS
        };
        // ⛔ IT STREAMS: the predicate IS the faucet's output, and the faucet is code under test.
        // Measured 2026-09-21 with the token send removed: joined, this probe ran past sixty
        // seconds and was killed. It was the plainest case of all and the first round blessed it.
        let (as_they_come, arrivals) = mpsc::channel();
        let peer = a_peer_that_says(
            name.clone(),
            vec![IpcMessage::Hello(build_stamp())],
            three_tokens,
            None,
            as_they_come,
        );
        run_the_graph_within(&name, WITH_A_PEER, Millis::new(0), words);
        let heard = collect_until(&arrivals, peer, three_tokens);
        let tokens: Vec<&IpcMessage> = heard
            .iter()
            .filter(|message| matches!(message, IpcMessage::Token { .. }))
            .collect();
        // ⛔ THE COUNT, NOT `{heard:?}`, and a red nobody can read is a red nobody acts on (E116).
        // ⚠️ THE REASON FIRST GIVEN HERE HAS MOVED, AND SO HAS THE CASE: it said "when the ceiling
        // expires the vector can be long", and since the verdict for an unmet predicate is given
        // inside `collect_until` this assertion is only ever reached when the predicate HELD --
        // so the vector is minimal, the loop having stopped at the first message that satisfied
        // it. The long-vector red now lives in one place, with its own count. ⚠️ SO THIS FIRST
        // ASSERTION IS A RESTATEMENT AND NOT AN ORACLE -- it repeats the predicate and cannot
        // fail on its own. The ORACLE OF THIS PROBE IS THE SECOND ONE, on provenance, which asks
        // something the predicate never asked.
        assert!(
            tokens.len() >= ENOUGH_TOKENS,
            "the faucet streams {ENOUGH_TOKENS} tokens (heard {} of them in {} messages in all)",
            tokens.len(),
            heard.len()
        );
        // ⛔ EVERY ONE, not "at least one": ADR-0014 makes the label hereditary, and a faucet that
        // marked only the first would be exactly the silent hole G13 exists to close.
        assert!(
            tokens.iter().all(|message| {
                matches!(message, IpcMessage::Token { provenance, .. } if *provenance == Provenance::Untrusted)
            }),
            "every token is untrusted: {tokens:?}"
        );
    }

    #[test]
    fn the_faucet_keeps_streaming_with_no_word_waiting() {
        // ⛔ THE HALF THAT GETS FORGOTTEN, and the one that names P-69. With NO word queued the
        // turn must pass anyway: if the faucet read `stdin` blockingly — or called `recv` instead
        // of `try_recv` — nothing would arrive at all. ⚠️ THE HAND STAYS ON THE KEYBOARD ACROSS
        // THE RUN (R5-6, 2026-09-15), so `try_recv` answers `Empty`: that is the case in which
        // `recv` would wait for ever. A dropped sender is the EASY case -- `recv` returns `Err` at
        // once, std's doc says so.
        //
        // ⛔ AND THE LIMIT IS DECLARED RATHER THAN CLAIMED AWAY (E121, gotcha #77): this probe
        // DISTINGUISHES NOTHING that `the_tokens_arrive_untrusted` does not already distinguish.
        // That one binds its hand with `let (_hand, words)`, and its assertion is strictly
        // stronger. ⚠️ THE TWO HANDS DO NOT LIVE EQUALLY LONG, and this line used to say they did
        // (m-2 of the third review): the sibling's lives to the end of its scope, while THIS one
        // is dropped by name right after the run. What the redundancy rests on is narrower and
        // still true -- during the RUN, which is the only stretch `try_recv` is called in, both
        // hands are alive and both answer `Empty`. Measured: under the mutation G1 BOTH go red,
        // TOGETHER and for the same reason. What this one still buys is the NAME -- a reader
        // looking for "and with nothing typed?" finds a probe that answers it -- and that is all
        // it buys. ⚠️ THIS LINE USED TO SAY "BOTH HANG", and it was true and filed as
        // REDUNDANCY: the evidence that the pair had the defect of E116 was written down
        // here by the very wave that cured only the other two probes. It is red now because both
        // stream (2026-09-21); the redundancy it declares is untouched.
        let name = a_name_for("nowords");
        let (hand, words) = a_keyboard();
        // ⛔ IT STREAMS, for the reason `the_tokens_arrive_untrusted` states: the predicate IS the
        // faucet's output. Measured 2026-09-21 with the token send removed: joined, it ran past
        // sixty seconds and was killed. ⛔ AND THE EVIDENCE WAS ALREADY IN THE COMMENT ABOVE,
        // written by the wave that did not cure it -- the lesson is that a sentence saying "both
        // hang" is a DEFECT REPORT wherever it appears, whatever it was filed under.
        let a_token: Until =
            |heard| heard.iter().any(|message| matches!(message, IpcMessage::Token { .. }));
        let (as_they_come, arrivals) = mpsc::channel();
        let peer = a_peer_that_says(
            name.clone(),
            vec![IpcMessage::Hello(build_stamp())],
            a_token,
            None,
            as_they_come,
        );
        run_the_graph_within(&name, WITH_A_PEER, Millis::new(0), words);
        drop(hand);
        let heard = collect_until(&arrivals, peer, a_token);
        // ⛔ THE COUNT, NOT `{heard:?}` -- see `the_tokens_arrive_untrusted`. ⚠️ AND HERE THE
        // ASSERTION IS THE PREDICATE WORD FOR WORD, so it cannot fail on its own: what fails a
        // broken run is `collect_until`, which gives the verdict when the predicate does not hold
        // within the ceiling. The line stays because it says in the probe what the probe claims;
        // it is not an oracle, and nobody should read it as one.
        assert!(
            heard.iter().any(|message| matches!(message, IpcMessage::Token { .. })),
            "with nothing typed the faucet still streams (heard {} messages in all)",
            heard.len()
        );
    }
}
