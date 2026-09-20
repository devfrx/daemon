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
/// ⚠️ THE SAME SHAPE LIVES IN THREE PLACES, AND EACH ONE SAYS SO: here,
/// `crates/daemon/src/main.rs`, and the sub-project 2 DST campaign (task 10). ⛔ AUDIO FIRST, and it is not
/// arbitrary: both profiles sit in one lane, and the daemon's own comment argues it.
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
/// ⛔ THE FIFTH COPY OF THIS SHAPE, AND IT STAYS LOCAL: D34, re-read on 2026-09-14 (P-74). The
/// other four are `crates/simulator/tests/arbiter_campaign.rs`, `crates/kernel/tests/serving.rs`,
/// the sub-project 2 campaign (task 10) and `crates/daemon/src/main.rs`. A common home would have to be
/// GENERIC over the reactor, because three of the five wrap `VirtualReactor` and two -- the daemon
/// and this one -- wrap `SystemReactor`.
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
        // `borrow_mut` are a panic that only appears when the two tasks interleave -- that is, not
        // in the shortest probe.
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
    /// ⚠️ IF TASK 7 ALREADY EXPOSES SUCH A HELPER when this is written, it is REUSED and not
    /// copied: two ways to obtain the same wrong stamp is one too many.
    fn a_stamp_that_is_not_ours() -> kernel::wire::ipc::BuildStamp {
        kernel::wire::ipc::stamp_set()
            .into_iter()
            .find_map(|message| match message {
                IpcMessage::StaleBuild(stamp) => Some(stamp),
                _ => None,
            })
            .expect("the canonical set holds a StaleBuild")
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

    /// What a probe waits for before it stops reading.
    ///
    /// ⛔ A PREDICATE AND NOT A COUNT (R5-4): the faucet streams a token per turn, so a fixed
    /// number of messages fills up with tokens before the one the probe is after has been sent.
    /// The loop still ends when the server closes -- `read` answers `Ok(0)` -- so a predicate that
    /// is never satisfied makes the probe FAIL on what it heard, not hang.
    type Until = fn(&[IpcMessage]) -> bool;

    /// The peer, in the shape task 9 gives it and for its reasons.
    ///
    /// ⛔ IT CAN HANG, AND MEASURING IT IS WHAT E110 COST. Nothing closes this connection:
    /// `Core::forget` drops the client from the CORE's table and never from the transport, which
    /// keeps `Connected` and the detached reader thread `accept` spawned for it -- and
    /// `LocalSocketIpc::drop_client` has no caller, because "the core closes" is not an operation
    /// of the port (decision 22). So `read` sees `Ok(0)` only when the peer's own predicate is
    /// satisfied, never because the run ended. ⛔ A PROBE THAT WAITS FOR A MESSAGE THE CORE WILL
    /// NEVER SEND THEREFORE HANGS FOR EVER, which is the worst red there is (E5, E6): it prints
    /// nothing, it cannot be bisected, and it eats the gate. ⚠️ EVERY CALLER WHOSE PREDICATE THE
    /// TOKEN STREAM SATISFIES may `join` AFTER the run; one that observes SILENCE takes
    /// `as_they_come` instead and collects under a ceiling.
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
        // ⛔ THE STREAM FOR WHOEVER OBSERVES SILENCE (E110). `None` for the five probes whose
        // predicate the token stream satisfies: they `join` and read the vector it returns.
        as_they_come: Option<mpsc::Sender<IpcMessage>>,
    ) -> std::thread::JoinHandle<Vec<IpcMessage>> {
        std::thread::spawn(move || {
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
                    if let Some(sender) = &as_they_come {
                        let _ = sender.send(message.clone());
                    }
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
        })
    }

    #[test]
    fn the_welcome_gives_the_sequence_one() {
        let name = a_name_for("welcome");
        let (_hand, words) = a_keyboard();
        let peer = a_peer_that_says(
            name.clone(),
            vec![IpcMessage::Hello(build_stamp())],
            |heard| heard.len() >= WELCOME,
            None,
            None,
        );
        run_the_graph(&name, WITH_A_PEER, Millis::new(0), words);
        let heard = peer.join().expect("the peer thread");
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
            Some(as_they_come),
        );
        run_the_graph(&name, WITH_A_PEER, Millis::new(0), words);
        // ⛔ A BOUNDED COLLECT AND NOT A `join`: THE WAIT IS THE OBSERVATION, and two seconds of
        // nothing is what "and then silence" means. ⚠️ The peer thread is left dangling on
        // purpose -- it dies with the test binary -- and the cost is declared rather than hidden.
        let mut heard = Vec::new();
        while let Ok(message) = arrivals.recv_timeout(std::time::Duration::from_secs(2)) {
            heard.push(message);
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
        let peer = a_peer_that_says(
            name.clone(),
            vec![IpcMessage::Hello(build_stamp())],
            |heard| {
                heard
                    .iter()
                    .filter(|message| matches!(message, IpcMessage::Degradation(_)))
                    .count()
                    >= 2
            },
            Some((hand, "degrade")),
            None,
        );
        run_the_graph(&name, WITH_A_PEER, Millis::new(0), words);
        let heard = peer.join().expect("the peer thread");
        let degraded: Vec<bool> = heard
            .iter()
            .filter_map(|message| match message {
                IpcMessage::Degradation(report) => Some(report.routing_degraded),
                _ => None,
            })
            .collect();
        assert_eq!(
            degraded,
            [false, true],
            "the welcome's Degradation is clean, and the word buys a SECOND one from the REAL code: {heard:?}"
        );
    }

    #[test]
    fn verdict_reaches_the_peer() {
        let name = a_name_for("verdict");
        let (hand, words) = a_keyboard();
        // ⛔ TYPED AFTER THE WELCOME, for the reason `degrade` states (R5-4): a `Verdict` sent to an
        // empty `attending()` reaches nobody, and the word is spent.
        let peer = a_peer_that_says(
            name.clone(),
            vec![IpcMessage::Hello(build_stamp())],
            |heard| heard.iter().any(|message| matches!(message, IpcMessage::Verdict(_))),
            Some((hand, "verdict")),
            None,
        );
        run_the_graph(&name, WITH_A_PEER, Millis::new(0), words);
        let heard = peer.join().expect("the peer thread");
        assert!(
            heard.iter().any(|message| matches!(message, IpcMessage::Verdict(_))),
            "the word `verdict` asks the REAL arbiter and the answer reaches the gui: {heard:?}"
        );
    }

    #[test]
    fn the_tokens_arrive_untrusted() {
        let name = a_name_for("tokens");
        let (_hand, words) = a_keyboard();
        let peer = a_peer_that_says(
            name.clone(),
            vec![IpcMessage::Hello(build_stamp())],
            |heard| {
                heard
                    .iter()
                    .filter(|message| matches!(message, IpcMessage::Token { .. }))
                    .count()
                    >= 3
            },
            None,
            None,
        );
        run_the_graph(&name, WITH_A_PEER, Millis::new(0), words);
        let heard = peer.join().expect("the peer thread");
        let tokens: Vec<&IpcMessage> = heard
            .iter()
            .filter(|message| matches!(message, IpcMessage::Token { .. }))
            .collect();
        assert!(!tokens.is_empty(), "the faucet streams: {heard:?}");
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
        // ⛔ THE HALF THAT GETS FORGOTTEN, and the one that catches P-69. With NO word queued the
        // turn must pass anyway: if the faucet read `stdin` blockingly — or called `recv` instead
        // of `try_recv` — nothing would arrive at all, and every probe above would hang rather
        // than fail. ⚠️ THE HAND STAYS ON THE KEYBOARD ACROSS THE RUN (R5-6, 2026-09-15), so
        // `try_recv` answers `Empty`: that is the case in which `recv` would wait for ever. A
        // dropped sender is the EASY case -- `recv` returns `Err` at once, std's doc says so -- and
        // with it the mutation G1 would stay green. Measured at the plan review.
        let name = a_name_for("nowords");
        let (hand, words) = a_keyboard();
        let peer = a_peer_that_says(
            name.clone(),
            vec![IpcMessage::Hello(build_stamp())],
            |heard| heard.iter().any(|message| matches!(message, IpcMessage::Token { .. })),
            None,
            None,
        );
        run_the_graph(&name, WITH_A_PEER, Millis::new(0), words);
        drop(hand);
        let heard = peer.join().expect("the peer thread");
        assert!(
            heard.iter().any(|message| matches!(message, IpcMessage::Token { .. })),
            "with nothing typed the faucet still streams: {heard:?}"
        );
    }
}
