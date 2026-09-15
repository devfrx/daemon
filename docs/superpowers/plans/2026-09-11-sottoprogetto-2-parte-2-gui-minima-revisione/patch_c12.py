"""patch_c12.py -- task 12 (R5-2/R3-16, R5-3, R5-4, R5-5, R5-6, R5-7, R5-8/R9a-1, R5-10/D83, R5-11, R5-12, R5-14, R5-16 and
R3-18 through D88, R5-17, R5-19, R5-20, R5-22, D75, D76), the P-63 command (R5-21), and the ledger rows of task 12.
Scoped, every anchor asserted before any write, atomic.

⚠️ The probe module of Passo 8 is REWRITTEN as a whole rather than patched line by line: R5-3, R5-4, R5-6, R5-7 and R5-12
touch its helpers and five of its six probes, and the peer is declared «in the shape task 9 gives it» while task 9 was
corrected AFTER task 12 was written (R4-6: a wall-clock deadline on the connect; R4-7: the WITH_A_PEER budget). The
realignment is a coherence correction the ledger did not list (decision 72), and the ledger receives the row.
"""
import io
import os
import sys

sys.stdout.reconfigure(encoding="utf-8")
ROOT = r"C:\Users\zagor\Desktop\harness"
PLAN = ROOT + r"\docs\superpowers\plans\2026-09-11-sottoprogetto-2-parte-2-gui-minima.md"
LEDGER = ROOT + r"\docs\superpowers\plans\2026-09-11-sottoprogetto-2-parte-2-gui-minima-revisione\ledger.md"

raw = io.open(PLAN, encoding="utf-8", newline="").read()
assert "\r\n" not in raw
t = raw


def scoped(text, start_marker, end_marker, subs):
    lo = text.index(start_marker)
    hi = text.index(end_marker, lo)
    seg = text[lo:hi]
    for old, new, n in subs:
        c = seg.count(old)
        assert c == n, f"expected {n}, found {c}: {old[:90]!r}"
    for old, new, n in subs:
        seg = seg.replace(old, new)
    return text[:lo] + seg + text[hi:]


# ----------------------------------------------------------------------------------------------- blocks (raw: no escapes)
FILES_DESIGN = r"""- Modify: `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` (**`i/lf w/lf`**) — ⛔ **tre richiami datati nella §7, D88** (R5-8, R9a-1, R5-16, R3-18): il capoverso 🔶 dedotto (**D41** e **P-68** sull'arbitro riscritto, **D22** sul valore di `Accepted`) e le righe *«la disposizione»* e *«la lista dei passi»*, provate nel kernel dal compito 7
"""

CONSUMES_TODAY = r"""- Consumes, da oggi: `kernel::arbiter::{Admission, Arbiter, ArbiterId, ComputeClass, Mib, Preemption, RemotePolicy, ResourceProfile, VramPolicy}`; `kernel::executor::{Executor, Sleep}`; `kernel::parameters::Parameters`; `kernel::ports::journal::{Journal, StepId}`; `kernel::ports::reactor::Reactor`; `kernel::record::{EffectClass, Record, RecordV1, RoutingDetail, Trust}`; `kernel::time::{Millis, Monotonic, WallTime}`; `kernel::wire::ipc::Verdict`; `platform::reactor::SystemReactor`; `platform::rng::SequentialRng`; `simulator::journal::MemoryJournal::new()` — ⚠️ riletti dagli `use` del Passo 5 (R5-14): `Grant` e `Detail` **non** si importano, e qui stavano
"""

EXCLUDE_PY = r"""
```bash
python - <<'EOF'
from pathlib import Path
p = Path("Cargo.toml")
text = p.read_text(encoding="utf-8", newline="")
anchor = 'exclude = ["spikes"]\r\n'
assert text.count(anchor) == 1, "anchor not unique -- the task is executed, or the root changed"
new = (
    "# `gui` is NOT a member either, and NOT for the reason above: `gui/fake-core` is a TOOL with a\r\n"
    "# lockfile of its own, under the root's `rust-toolchain.toml` (rustup walks up: measured, R5-22),\r\n"
    "# built by `scripts/gate-gui.sh` through `--manifest-path`,\r\n"
    "# and `gui/` also holds a node project that cargo has no business walking into.\r\n"
    'exclude = ["spikes", "gui"]\r\n'
)
p.write_text(text.replace(anchor, new, 1), encoding="utf-8", newline="")
EOF
```
"""

GITIGNORE_PY = r"""
```bash
python - <<'EOF'
from pathlib import Path
p = Path(".gitignore")
text = p.read_text(encoding="utf-8", newline="")
anchor = "/gui/dist/\r\n"
assert text.count(anchor) == 1, "the two lines of task 11 are not there, or the task is executed"
assert "/gui/fake-core/target/" not in text, "already there -- the task is executed"
p.write_text(text.replace(anchor, anchor + "/gui/fake-core/target/\r\n", 1), encoding="utf-8", newline="")
EOF
git ls-files --eol .gitignore
git diff --stat .gitignore
```

Atteso: `i/lf w/crlf` **invariato**, e il diff dice **una riga aggiunta** — non seicento (R5-19: i due file CRLF si toccano
con lo script e non a mano, la forma del Passo 3 del compito 11; senza il comando il `sed -i` è a un passo, vincolo globale 4).
"""

LOCKFILE_WHY = r"""
⛔ **E il lockfile si SEMINA dalla radice prima del primo `cargo build` — D83, misurato alla revisione del piano intero
(R5-10):** una risoluzione da zero prende `redb` **4.3.0** mentre la radice appunta la 4.1.0 con un caret
(`crates/platform/Cargo.toml`, *«written by READING the source of 4.1.0»*), e il finto monterebbe una versione della porta
che il prodotto non ha. Con la copia, `cargo build` pota le voci inutili e **conserva** le versioni che soddisfano; il
Passo 10 confronta le crate comuni dei due lockfile, e `cargo audit --file gui/fake-core/Cargo.lock` entra in
`gate-gui.sh` al compito **16**.
"""

CLOCK_ORDER = r"""    let reactor = RefCell::new(SystemReactor::new());
    // ⚠️ THE DECLARATION ORDER IS LOAD-BEARING, as the daemon says of its own (task 9): `serve` and
    // the faucet BORROW `clock` for the executor's whole life and locals drop in reverse order, so
    // `clock` is declared BEFORE the executor. Measured at the plan review (R5-2, R3-16): a temporary
    // `&SharedClock(&reactor)` inside `spawn` dies at the end of its statement (E0716), and a `let`
    // AFTER the executor does not live long enough (E0597).
    let clock = SharedClock(&reactor);
    let sleep = Sleep::new();
"""

NEW_TESTS = r'''#[cfg(test)]
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
    /// ⛔ IT CANNOT HANG, AND THE REASON IS THE DROP ORDER RATHER THAN A TIMEOUT: when
    /// `run_the_graph` returns, its locals fall, the `LocalSocketIpc` with them, and the server end
    /// closes — so `read` comes back `Ok(0)` and the loop ends. ⛔ EVERY CALLER THEREFORE `join`s
    /// AFTER THE RUN, NEVER BEFORE.
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
                    heard.push(
                        IpcMessage::decode(&buffer[..next]).expect("the core sends what it says"),
                    );
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
        // "it stopped listening" apart from "it had nothing more to say". The peer asks for two
        // and gets one: the loop ends on `Ok(0)`, when the run is over.
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
        );
        run_the_graph(&name, WITH_A_PEER, Millis::new(0), words);
        let heard = peer.join().expect("the peer thread");
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
        );
        run_the_graph(&name, WITH_A_PEER, Millis::new(0), words);
        drop(hand);
        let heard = peer.join().expect("the peer thread");
        assert!(
            heard.iter().any(|message| matches!(message, IpcMessage::Token { .. })),
            "with nothing typed the faucet still streams: {heard:?}"
        );
    }
}'''

TESTS_NOTE = r"""
✅ **Riscritte alla revisione del piano intero, 2026-09-15:** `build_stamp` importato dentro `mod tests` (R5-12); a `decode`
va la cornice **intera** (R5-3, come il 9); il pari legge fino a un **predicato** e non a un conteggio, e **digita la parola
dopo l'accoglienza** tenendo lui la mano sulla tastiera (R5-4) — `degrade` asserisce la **seconda** `Degradation`,
`[false, true]`; la sonda dell'accoglienza legge anche i **numeri** del `Policy` (R5-7); la sonda negativa tiene vivo il
mittente, così `try_recv` risponde `Empty` e non `Disconnected` (R5-6); e il pari è **riallineato al 9 corretto** —
scadenza di parete sul `connect` (R4-6) e budget `WITH_A_PEER` (R4-7) — perché la prima stesura lo aveva copiato prima
di quelle correzioni.
"""

STAGE_9 = r"""- [ ] **Passo 9: le due direzioni, misurate**

⛔ **Prima, in scena ciò che è nuovo** — R5-11, la misura al Passo 4 del compito 11: un file non tracciato non ha diff, e
`git add -N` non basta (il diff resta pieno e `git checkout --` svuota il file):

```bash
git add Cargo.toml .gitignore crates/kernel/src/serving.rs gui/fake-core
git status --porcelain gui/fake-core | grep -c 'target/'
```

Atteso: **0** — `/gui/fake-core/target/` morde (**D38**). Poi le mutazioni, **una per volta**, ciascuna compilata,
eseguita e revocata con `git checkout --` sul file toccato, `git diff` a zero alla fine:

| | La mutazione | Atteso |
"""

PASSO_9BIS = r"""- [ ] **Passo 9-bis: i tre richiami nel disegno del 2 — D88**

`docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` è **LF**: Python con `newline=""`, ancora unica
asserita, temporaneo e `os.replace` (la forma del Passo 3 del compito 11). ⛔ **Ogni ancora è la riga intera presa dal file
col `grep`**, mai ricopiata da qui, e il richiamo si **appende in coda**, su **una** riga — nelle celle come fa il Passo 8
del compito 10 (`riga.rstrip().removesuffix("|").rstrip() + " " + richiamo + " |"`). Arrivato qui dalla revisione del piano
intero (R5-8, R9a-1: il blocco *Files* prometteva il richiamo di **D41** e nessun Passo lo scriveva, e il criterio cercava
una data fissa; R5-16 e R3-18: due righe e un dedotto della §7 che il piano ha spostato o deciso senza dirlo nel disegno).

| Dove (`grep -n -F` sulla frase → una riga) | Che cosa si appende, in coda |
|---|---|
| §7, il capoverso 🔶 dedotto — l'ancora è la sua **ultima** riga, `il piano, e l'attività non ha il problema perché vive nel kernel (pezzo 6 della §3 della stella polare).` (`grep -c -F` → **1**); il richiamo si appende in coda a quella riga | `✅ **RICHIAMO DEL <data>, compito 12 del piano della parte 2 (D41, P-68):** la costruzione dell'arbitro **non si sposta** — nessuna casa condivisa regge: il kernel non nomina un default (ADR-0034), le crate restano cinque, una quota VRAM non è dell'OS, e un \`lib.rs\` nel daemon porterebbe il cablaggio di produzione in un attrezzo — quindi il finto la **riscrive** coi propri letterali, e le tre case (\`crates/daemon/src/main.rs\`, la campagna del compito 10, \`gui/fake-core/src/main.rs\`) si nominano a vicenda; decisione del proprietario del 2026-09-14. ⚠️ **E il valore di \`Accepted\` NON è consegnato** a nessuno dei due (**D22**): \`Protection\` ha una variante sola` |
| §7, la riga `\| la disposizione \|` — in coda alla **terza** cella, prima dell'ultimo `\|` | `✅ **RICHIAMO DEL <data>, compito 12 del piano della parte 2 (D88, R5-16):** il giro salva → ritrova è provato **nel kernel** dal compito 7 (\`crates/kernel/tests/serving.rs\`), non nel finto, le cui sonde non mandano \`SaveLayout\`` |
| §7, la riga `\| la lista dei passi \|` — in coda alla **terza** cella, prima dell'ultimo `\|` | `✅ **RICHIAMO DEL <data>, compito 12 del piano della parte 2 (D88, R5-16):** la sonda dell'invocazione vive **nel kernel**, compito 7 (\`crates/kernel/tests/serving.rs\`): l'unico \`Invoke\` del finto è quello del pari stantio, atteso ignorato` |

```bash
grep -c 'RICHIAMO DEL <data>, compito 12' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md
awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md
tr -cd '\r' < docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md | wc -c
```

Atteso (con la data scritta): **3**; niente; **0**.
"""

TIMING = r"""cargo clean --manifest-path gui/fake-core/Cargo.toml
time (cd gui/fake-core && cargo test --locked > /dev/null 2>&1)
time (cd gui/fake-core && cargo test --locked > /dev/null 2>&1)
python - <<'EOF'
import io, re
def versions(path):
    text = io.open(path, encoding="utf-8", newline="").read().replace("\r\n", "\n")
    return dict(re.findall(r'name = "([^"]+)"\nversion = "([^"]+)"', text))
root, fake = versions("Cargo.lock"), versions("gui/fake-core/Cargo.lock")
print(sorted((name, root[name], fake[name]) for name in root.keys() & fake.keys() if root[name] != fake[name]))
EOF
"""

TIMING_ATTESO = r""" ⚠️ **E due tempi, a freddo e a caldo** (R5-17), che il compito **15** scrive con la data nel commento di `gate-gui.sh` — la
riga *«il core finto nel cancello»* della §8 li vuole *«dichiarati e misurati al piano»*, e la cifra non si scrive qui
(vincolo globale 3); e lo script stampa **`[]`**: nessuna crate comune ai due lockfile cambia versione (**D83**)."""

CRIT_DISPATCH = r"""- [ ] ⛔ **nessun ramo di dispaccio nel finto:** `awk '/#\[cfg\(test\)\]/{exit} {print}' gui/fake-core/src/main.rs | grep -cE 'IpcMessage::(Hello|Invoke|Approve|SaveLayout) *(\(|=>)'` → **0** — la metà **senza** le sonde, perché le sonde ne mandano (R5-5: sul file intero il conteggio è **più di zero**, ed è la direzione opposta che prova che il comando morde dove deve); il finto **manda** messaggi, non li interpreta
"""
CRIT_CLOCK = r"""- [ ] `grep -c 'struct SharedClock' gui/fake-core/src/main.rs` → **1**, e `grep -rl 'struct SharedClock' crates/ --include='*.rs' | wc -l` → **4**: cinque in tutto (**P-74**; R5-20: qui stava un `grep -rc`, che stampa un conteggio per file e nessuna somma)
"""
CRIT_RECALLS = r"""- [ ] la §7 del disegno porta i **tre** richiami datati (**D88**): `grep -c 'RICHIAMO DEL <data>, compito 12' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` → **3**, e il disegno resta `i/lf w/lf` a CR zero — ⚠️ qui stava `grep -c 'RICHIAMO DEL 2026-09-14'`, una data di scrittura che a compito eseguito sarebbe rimasta rossa (D75; R5-8, R9a-1)
"""
CRIT_MUTATIONS = r"""- [ ] le cinque mutazioni del Passo 9 eseguite **una per volta** e revocate, `git diff` vuoto — **dopo** il `git add` del Passo 9, senza il quale un file nuovo non ha diff (R5-11)
- [ ] le crate comuni dei due lockfile hanno la **stessa versione**: lo script del Passo 10 stampa `[]` (**D83**)
"""

# ----------------------------------------------------------------------------------------------- task 12
t = scoped(t, "\n## Compito 12:", "\n## Compito 13:", [
    # Files
    ("- Modify: `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` (**`i/lf w/lf`**) — **un** richiamo datato nella §7 (**D41**)\n",
     FILES_DESIGN, 1),
    ("`crates/daemon/src/main.rs` **per intero**, che è la forma da riscrivere\n",
     "`crates/daemon/src/main.rs` **per intero**, che è la forma da riscrivere — e **D83** (il lockfile seminato dalla radice), **D88**\n", 1),
    # Interfaces (R5-14)
    ("- Consumes, dal **compito 2**: `platform::ipc::LocalSocketIpc::bound(name, numbers, max_body)`\n",
     "- Consumes, dal **compito 2**: `platform::ipc::LocalSocketIpc::bound(name, numbers, max_body)`, e `kernel::framing::take_frame` nel pari delle sonde\n", 1),
    ("- Consumes, dal **compito 3**: `kernel::wire::ipc::{IpcMessage, Provenance}`\n",
     "- Consumes, dal **compito 3**: `kernel::wire::ipc::{IpcMessage, Provenance}`; nelle sonde anche `build_stamp`, `stamp_set`, `BuildStamp` e `Call`\n", 1),
    ("- Consumes, da oggi: `kernel::arbiter::{Arbiter, ArbiterId, ComputeClass, Mib, Preemption, ResourceProfile, RemotePolicy, VramPolicy, Admission, Grant}`; `kernel::executor::{Executor, Sleep}`; `kernel::ports::journal::{Journal, StepId}`; `kernel::record::{Detail, EffectClass, Record, RecordV1, RoutingDetail, Trust}`; `kernel::time::{Millis, Monotonic}`; `platform::reactor::SystemReactor`; `platform::rng::SequentialRng`; `simulator::journal::MemoryJournal::new()`\n",
     CONSUMES_TODAY, 1),
    # Passo 2 (R5-19, R5-22)
    ("Poi, in `Cargo.toml` di radice (**CRLF**, quindi Python `newline=\"\"`), `exclude` diventa:\n",
     "Poi, in `Cargo.toml` di radice (**CRLF**, quindi Python `newline=\"\"` — e il comando sta qui sotto, R5-19), `exclude` diventa:\n", 1),
    ("# `gui` is NOT a member either, and NOT for the reason above: `gui/fake-core` is a TOOL with a\n# toolchain and a lockfile of its own, built by `scripts/gate-gui.sh` through `--manifest-path`,\n",
     "# `gui` is NOT a member either, and NOT for the reason above: `gui/fake-core` is a TOOL with a\n# lockfile of its own, under the root's `rust-toolchain.toml` (rustup walks up: measured, R5-22),\n# built by `scripts/gate-gui.sh` through `--manifest-path`,\n", 1),
    ("exclude = [\"spikes\", \"gui\"]\n```\n\n```bash\ncargo metadata --no-deps --format-version 1 --manifest-path gui/fake-core/Cargo.toml > /dev/null; echo \"EXIT=$?\"\ngit ls-files --eol Cargo.toml\n",
     "exclude = [\"spikes\", \"gui\"]\n```\n" + EXCLUDE_PY + "\n```bash\ncargo metadata --no-deps --format-version 1 --manifest-path gui/fake-core/Cargo.toml > /dev/null; echo \"EXIT=$?\"\ngit ls-files --eol Cargo.toml\n", 1),
    ("E in `.gitignore` (**CRLF**), sotto le due righe del compito 11:\n\n```\n/gui/fake-core/target/\n```\n",
     "E in `.gitignore` (**CRLF**), sotto le due righe del compito 11:\n\n```\n/gui/fake-core/target/\n```\n" + GITIGNORE_PY, 1),
    # Passo 4 (R5-10, D83)
    ("```bash\ncd gui/fake-core && cargo build 2>&1 | tail -5; cd ../..\nls gui/fake-core/Cargo.lock\n",
     "```bash\ncp Cargo.lock gui/fake-core/Cargo.lock\ncd gui/fake-core && cargo build 2>&1 | tail -5; cd ../..\nls gui/fake-core/Cargo.lock\n", 1),
    ("⚠️ **`cargo build` SENZA `--locked`** qui, perché il lockfile nasce adesso — è il vincolo globale 6 nella sua\nmetà web.\n",
     "⚠️ **`cargo build` SENZA `--locked`** qui, perché il lockfile nasce adesso — è il vincolo globale 6 nella sua\nmetà web.\n" + LOCKFILE_WHY, 1),
    # Passo 5 (R5-12, D76)
    ("use kernel::wire::ipc::{IpcMessage, Provenance, Verdict, build_stamp};\n", "use kernel::wire::ipc::{IpcMessage, Provenance, Verdict};\n", 1),
    ("milestone-2 design", "sub-project 2 design", 2),
    ("the milestone-2 DST campaign", "the sub-project 2 DST campaign (task 10)", 1),
    ("the milestone-2 campaign", "the sub-project 2 campaign (task 10)", 1),
    ("in this milestone (ADR-0007)", "in this sub-project (ADR-0007)", 1),
    # Passo 7 (R5-2, R3-16)
    ("    let reactor = RefCell::new(SystemReactor::new());\n    let sleep = Sleep::new();\n", CLOCK_ORDER, 1),
    ("    executor.spawn(serve(&core, &SharedClock(&reactor), &sleep));\n    executor.spawn(the_faucet(&core, &SharedClock(&reactor), &sleep, cadence, words));\n",
     "    executor.spawn(serve(&core, &clock, &sleep));\n    executor.spawn(the_faucet(&core, &clock, &sleep, cadence, words));\n", 1),
    # Passo 9 (R5-11, R5-6, R5-7)
    ("- [ ] **Passo 9: le due direzioni, misurate**\n\n| | La mutazione | Atteso |\n", STAGE_9, 1),
    ("| **G1** | `try_recv` → `recv` (bloccante) nel rubinetto | `the_faucet_keeps_streaming_with_no_word_waiting` **rosso**, e si pianta invece di fallire — ⚠️ **il rosso è un timeout, e va detto**: `cargo test` non torna |",
     "| **G1** | `try_recv` → `recv` (bloccante) nel rubinetto | `the_faucet_keeps_streaming_with_no_word_waiting` **rosso**, e si pianta invece di fallire — ⚠️ **il rosso è un timeout, e va detto**: `cargo test` non torna. ⛔ Morde perché la sonda **tiene vivo il mittente** (R5-6): con un mittente caduto `recv` renderebbe `Err` subito e la mutazione resterebbe verde |", 1),
    ("| **G2** | togli `PRESENTATION_RESERVATION` da `build_the_arbiter` | la sonda dell'accoglienza **rossa** sul `Policy`, perché `allocated` scende |\n",
     "| **G2** | togli `PRESENTATION_RESERVATION` da `build_the_arbiter` | la sonda dell'accoglienza **rossa** sul `Policy`, perché `allocated` scende — ⚠️ a coglierlo sono le due righe dei **numeri** (R5-7): l'ordine delle cinque specie resterebbe verde |\n", 1),
    # Passo 9-bis (D88), before Passo 10; Passo 10 (R5-17, D83)
    ("- [ ] **Passo 10: il cancello, e il commit**\n\n```bash\ncd gui/fake-core && cargo test --locked 2>&1 | tail -5; cd ../..\n",
     PASSO_9BIS + "\n- [ ] **Passo 10: il cancello, e il commit**\n\n```bash\ncd gui/fake-core && cargo test --locked 2>&1 | tail -5; cd ../..\n" + TIMING, 1),
    ("Atteso: le sonde del finto verdi; `GATE GREEN`; `OK`; il `diff` **`EXIT=0`**; i due file di radice ancora\n`i/lf w/crlf`; **zero** CR nel sorgente nuovo; in `git status` niente `target/`.\n",
     "Atteso: le sonde del finto verdi; `GATE GREEN`; `OK`; il `diff` **`EXIT=0`**; i due file di radice ancora\n`i/lf w/crlf`; **zero** CR nel sorgente nuovo; in `git status` niente `target/`." + TIMING_ATTESO + "\n", 1),
    # Criteria
    ("- [ ] ⛔ **nessun ramo di dispaccio nel finto:** `grep -cE 'IpcMessage::(Hello|Invoke|Approve|SaveLayout) *(\\(|=>)' gui/fake-core/src/main.rs` → **0** — il finto **manda** messaggi, non li interpreta\n",
     CRIT_DISPATCH, 1),
    ("- [ ] `grep -c 'struct SharedClock' gui/fake-core/src/main.rs` → **1**, e `grep -rc 'struct SharedClock' crates/ --include='*.rs'` conta le altre: **cinque in tutto** (**P-74**)\n",
     CRIT_CLOCK, 1),
    ("- [ ] la §7 del disegno porta il richiamo datato di **D41**: `grep -c 'RICHIAMO DEL 2026-09-14' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` → **più di zero**\n",
     CRIT_RECALLS, 1),
    ("- [ ] le cinque mutazioni del Passo 9 eseguite **una per volta** e revocate, `git diff` vuoto\n", CRIT_MUTATIONS, 1),
])

# ----------------------------------------------------------------------------------------------- Passo 8: the probe module, whole
lo = t.index("\n## Compito 12:")
hi = t.index("\n## Compito 13:", lo)
seg = t[lo:hi]
START = "#[cfg(test)]\nmod tests {\n"
END = "\n```\n\n⛔ **`interprocess` va in `[dev-dependencies]` di `gui/fake-core`, in DUE passi**"
assert seg.count(START) == 1 and seg.count(END) == 1, "the probe module of task 12 is not where the anchors say"
a = seg.index(START)
b = seg.index(END, a)
old_tests = seg[a:b]
assert old_tests.count("fn a_peer_that_says") == 1 and old_tests.count("fn typed") == 1 and old_tests.count("#[test]") == 6
# The note goes AFTER the closing fence and BEFORE the «interprocess in due passi» paragraph, which stays whole.
FENCE, PARAGRAPH = "\n```\n", END[len("\n```\n\n"):]
seg = seg[:a] + NEW_TESTS + FENCE + TESTS_NOTE + "\n" + PARAGRAPH + seg[b + len(END):]
t = t[:lo] + seg + t[hi:]

# ----------------------------------------------------------------------------------------------- P-63 (R5-21)
t = scoped(t, "\n### P-63 ", "\n### P-64 ", [
    ("grep -n 'non si carica da .nessuno. dei due punti' docs/riferimenti.md\n```\n",
     "grep -n 'non si carica da' docs/riferimenti.md\n```\n\n(⚠️ qui stava `.nessuno.`, che non copre il **nessuno** in grassetto della riga — R5-21, 2026-09-15: il comando rendeva zero righe.)\n", 1),
])

# ----------------------------------------------------------------------------------------------- ledger
lraw = io.open(LEDGER, encoding="utf-8", newline="").read()
assert "\r\n" not in lraw
l = lraw
s = l.index("### Compito 12\n")
e = l.index("\n### Compito 13", s)
old_c12 = l[s:e]
assert old_c12.count(" ⬜") == 19, f"task 12 rows: expected 19 open, found {old_c12.count(' ⬜')}"
NEW_C12 = """### Compito 12
- R5-11 `git add Cargo.toml .gitignore crates/kernel/src/serving.rs gui/fake-core` in testa al Passo 9, **in scena e non `-N`** (la misura nella riga R5-11 del compito 11), col `grep -c target/` → 0 come seconda direzione di D38; il criterio delle mutazioni lo nomina ✅
- R5-2/R3-16 `let clock = SharedClock(&reactor);` prima dell'esecutore, `serve(&core, &clock, …)` e `the_faucet(&core, &clock, …)`, il commento «LOAD-BEARING» con E0716/E0597 (misurato dal revisore col modello compilato) ✅
- R5-3 `IpcMessage::decode(&buffer[..next])` nel pari, col commento del 9 (rimisurato: `decode` comincia con `framing::unframe`) ✅
- R5-4 il pari legge fino a un **predicato** (`type Until = fn(&[IpcMessage]) -> bool`) e digita la parola **dopo** l'accoglienza tenendo lui il `Sender` (`then_types: Option<(Sender<String>, &'static str)>`, a `heard.len() >= WELCOME`); `degrade` asserisce `[false, true]` sulle due `Degradation`; `verdict` idem ✅
- R5-5 criterio sulla metà non-test con l'`awk` fino a `#[cfg(test)]` → 0, e la direzione opposta sul file intero → più di zero ✅
- R5-6 `a_keyboard()` rende anche il `Sender`; la sonda negativa lo tiene vivo (`drop(hand)` dopo la corsa) e il commento dice che `Disconnected` è il caso **facile**; G1 resta «timeout» e dice perché morde ✅
- R5-7 accoglienza: `report.allocated == Mib::new(1_024 + 768)` e `report.total == TOTAL_VRAM` (`PolicyReport` è `Copy`); G2 dice che sono queste righe a coglierlo ✅
- R5-8/R9a-1 **Passo 9-bis** con la tabella delle ancore: il richiamo D41+P-68 (e D22, R3-18) in coda all'ultima riga del capoverso 🔶 dedotto (`grep -c -F` → 1 misurato oggi sulla riga 375); criterio `grep -c 'RICHIAMO DEL <data>, compito 12'` → 3; Files riscritto; via la data fissa `2026-09-14` (D75) ✅
- R5-10 (D83) `cp Cargo.lock gui/fake-core/Cargo.lock` prima del primo `cargo build`, il perché (rimisurato: la radice appunta `redb = "4.1.0"` con un caret, il lockfile 4.1.0), lo script del Passo 10 che confronta le crate comuni → `[]`, e il criterio ✅
- R5-12 `use kernel::wire::ipc::build_stamp;` dentro `mod tests`, tolto dall'`use` di testa (usato solo dalle sonde: verificato sul modello) ✅
- R5-14 Consumes riscritto dagli `use` del Passo 5: `take_frame` dal 2; `build_stamp`, `stamp_set`, `BuildStamp`, `Call` dal 3; da oggi anche `Parameters`, `Reactor`, `WallTime`, `Verdict`; via `Grant` e `Detail` ✅
- R5-16 (D88) i due richiami sulle righe «la disposizione» e «la lista dei passi» della §7 nel Passo 9-bis → provate nel kernel dal compito 7 ✅
- R5-17 `cargo clean` + due `time (… cargo test --locked)` nel Passo 10, e l'Atteso che manda la cifra al 15 con la data ✅
- R5-19 due `python - <<'EOF'` con `newline=""` e `assert` per `Cargo.toml` e `.gitignore`, `git ls-files --eol` e `git diff --stat` dopo ✅
- R5-20 `grep -rl … | wc -l` → 4, e il finto → 1 ✅
- R5-21 P-63: `grep -n 'non si carica da' docs/riferimenti.md` (rimisurato: la forma vecchia → 0 righe, la nuova → 1) ✅
- R5-22 «a TOOL with a lockfile of its own, under the root's `rust-toolchain.toml`» nel commento dettato e nello script (rimisurato: `rustup show active-toolchain` da una sottocartella → «overridden by … rust-toolchain.toml») ✅
- R3-18 (D88) D22 dentro il richiamo del capoverso 🔶 (la forma «o nel richiamo D41 del 12» che il rilievo ammetteva) ✅
- D76 «sub-project 2 design» ×2, «the sub-project 2 DST campaign (task 10)», «the sub-project 2 campaign (task 10)», «in this sub-project (ADR-0007)» ✅
- **Coerenza col 9 corretto** (decisione 72, non era nel registro): il pari del 12 è dichiarato «nella forma del 9» ma copiato **prima** di R4-6 e R4-7 — entrano la scadenza di parete di 5 s sul `connect` e il budget `WITH_A_PEER` (100_001) al posto di 4 000/8 000, con `WELCOME` = 5 riletto sul 7; la sonda dell'accoglienza mappa i **primi cinque** (`take(WELCOME)`), perché una `read` può portare anche il primo `Token` ✅
- ⚠️ **Non compilato:** il modulo delle sonde è riscritto sul modello letto, non eseguito (il compito 2 e il 7 non esistono ancora nel repo): chi esegue il 12 lo compila per primo e ogni rosso è una voce d'errata, come dice «Come si esegue» — il revisore R5 aveva compilato la forma vecchia
- Attrezzo: `patch_c12.py` accanto a questo file (tocca anche P-63 e questo registro)
"""
l = l[:s] + NEW_C12 + l[e:]
OLD_STATE = "**Sessione 14 (2026-09-15):** compito 11 applicato (✅), con una riga R5-11 nuova sul 12."
NEW_STATE = "**Sessione 14 (2026-09-15):** compiti 11 e 12 applicati (✅)."
assert l.count(OLD_STATE) == 1
l = l.replace(OLD_STATE, NEW_STATE)

# ----------------------------------------------------------------------------------------------- write, atomically
for path, content in ((PLAN, t), (LEDGER, l)):
    tmp = path + ".tmp"
    io.open(tmp, "w", encoding="utf-8", newline="").write(content)
    os.replace(tmp, path)
print("ok: task 12 patched (plus P-63 and the ledger);", t.count("\n") - raw.count("\n"), "plan lines added;",
      l.count("\n") - lraw.count("\n"), "ledger lines added")
