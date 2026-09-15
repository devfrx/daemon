"""patch_c2.py -- task 2 corrections (D78, D82, D88, R1-7..R1-15, R9a-3, R10-16, D75, D76).
Anchors asserted unique BEFORE writing; the file is written once, atomically."""
import io
import os
import sys

sys.stdout.reconfigure(encoding="utf-8")
PLAN = r"C:\Users\zagor\Desktop\harness\docs\superpowers\plans\2026-09-11-sottoprogetto-2-parte-2-gui-minima.md"
raw = io.open(PLAN, encoding="utf-8", newline="").read()
assert "\r\n" not in raw
t = raw

NEW_TRANSPORT = r'''//! The `ipc` port over a local socket: one named pipe on Windows, one unix socket on Linux.
//!
//! ⛔ THE LISTENER IS NONBLOCKING, THE STREAMS ARE NOT, AND EVERY CLIENT GETS A READER THREAD (D78).
//! `accept` answers `ErrorKind::WouldBlock` when nobody is connecting -- read in `interprocess`
//! 2.4.4 and measured -- so the core's activity can ask every turn (§3 and §5 of the sub-project 2
//! design). A NONBLOCKING READ was the first design of this file, and it is unusable on Windows:
//! MEASURED on 2026-09-15 with the bench next door, four probes red. With `PIPE_NOWAIT` a read that
//! finds nothing fails with `ERROR_NO_DATA`, which `std` classifies as `BrokenPipe`, and
//! `interprocess::os::windows::misc::downgrade_eof` turns every `BrokenPipe` into `Ok(0)`: a live
//! peer that is silent and a peer that is gone answer the SAME thing. So the stream stays
//! blocking; on `accept` a `try_clone` of it goes to a thread that reads until end of stream and
//! pushes every chunk into an `std::sync::mpsc` channel, and `receive` drains that channel without
//! blocking. End of stream is REAL there: the blocking read answers `Ok(0)` only when the peer is
//! gone, the thread ends, the channel closes, and a closed channel with nothing whole left is
//! `Disconnected`.
//!
//! ⛔ WINDOWS TRAP, READ IN THE DOC OF `Listener::accept` AND NOT DEDUCED: "neglecting to call
//! this periodically may result in new clients being unable to connect" -- a peer that connects
//! and disconnects with no `accept` in between leaves a dead instance that blocks new
//! connections. The activity calls `accept` EVERY TURN, and that is what disarms it. Whoever
//! moves that call out of the loop reopens this.
//!
//! ⛔ IT READS THE STREAM WITH `framing::take_frame` AND NOT `framing::unframe`. `unframe` takes
//! ONE frame and refuses every tail -- its own doc says a real transport wants a second entry
//! point -- and two frames arriving in one read is the ordinary case, not the edge one.
//!
//! ⛔ AND IT ADDS NO ENVELOPE OF ITS OWN: THE MESSAGES ARRIVE ALREADY FRAMED. `IpcMessage::encode`
//! ends in `framing::frame` and `IpcMessage::decode` begins in `framing::unframe`, so a message
//! handed to `send` is ALREADY self-delimiting; framing it again would put a second envelope on
//! the wire, give the same four bytes two meanings, and leave the TypeScript peer stripping two.
//! `send` therefore writes VERBATIM, and `receive` gives back THE WHOLE FRAME -- envelope
//! included -- which is what `decode` expects. The envelope is read here only to find where one
//! message ends and the next begins, which is the one thing a byte stream does not carry and the
//! two `Vec<Vec<u8>>` fakes of this port get for free.
//!
//! ⛔ `send` IS BLOCKING, AND THAT IS A DECLARED LIMIT RATHER THAN AN OVERSIGHT: a gui that stops
//! reading fills the pipe, and the next `send` stalls the core's activity until it reads again.
//! In sub-project 2 the gui is 0..1 and reads every message it is sent (§6a); whether a per-client
//! outgoing queue is worth its state is a MEASUREMENT for sub-project 3, not a guess here. What a
//! stall does NOT do is corrupt the stream: `write_all` writes the whole frame or fails, and a
//! failure drops the client -- a length-prefixed stream cannot be resynchronised (D9).
//!
//! ⛔ A `Vec` AND NOT A `HashMap` for the client table, the reckoning gotcha #12 records for the
//! kernel: the table holds ONE client in sub-project 2 (the gui is 0..1, ADR-0004), and a map
//! would buy nothing while introducing an iteration order.

use std::io::{Read, Write};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;

use interprocess::local_socket::{
    prelude::*, GenericNamespaced, Listener, ListenerNonblockingMode, ListenerOptions, Stream,
};
use interprocess::TryClone;
use kernel::framing;
use kernel::numbering::Progressive;
use kernel::ports::ipc::{ClientId, Ipc, IpcError};

/// How much a reader thread asks the stream for at a time. NOT a cap: bodies are bounded by the
/// delivered `max_body`, and a frame longer than this simply arrives in several chunks.
const CHUNK: usize = 4096;

/// One connected peer, and what has arrived from it so far.
struct Connected {
    id: ClientId,
    /// The stream this end WRITES to. Its reading clone lives on the reader thread.
    stream: Stream,
    /// What the reader thread has read so far, chunk by chunk. When the thread ends the channel
    /// closes, and that closing is how the peer's death reaches `receive`.
    from_peer: Receiver<Vec<u8>>,
    /// Bytes received and not yet formed into a whole frame.
    pending: Vec<u8>,
    /// Set once a declared length exceeded the cap. ⛔ IT IS NOT CLEARED: a length-prefixed
    /// stream cannot be resynchronised, so every later `receive` answers the same thing.
    poisoned: bool,
}

/// Reads `stream` until end of stream or error and hands every chunk to `into`. It ends -- and
/// closes the channel by dropping `into` -- when the peer is gone, or when nobody listens any more.
fn read_until_the_end(mut stream: Stream, into: Sender<Vec<u8>>) {
    let mut chunk = [0_u8; CHUNK];
    loop {
        match stream.read(&mut chunk) {
            Ok(0) | Err(_) => break,
            Ok(read) => {
                if into.send(chunk[..read].to_vec()).is_err() {
                    break;
                }
            }
        }
    }
}

/// The real `ipc` transport.
pub struct LocalSocketIpc {
    listener: Listener,
    clients: Vec<Connected>,
    /// ⛔ DELIVERED, NEVER STARTED HERE (ADR-0034, and the doc of `ClientId`): a private counter
    /// inside this type would be the second one, and two that look identical diverge with
    /// nothing to report it.
    numbers: Progressive,
    /// The longest body this transport will buffer. ⛔ DELIVERED TOO, and it is what gives
    /// `IpcError::MalformedMessage` a producer: the envelope cannot tell a broken frame from a
    /// valid one -- any four bytes are a length -- so without a cap the promise of §3 would have
    /// nothing to hold, and a peer declaring four gibibytes would be buffered for ever.
    max_body: usize,
}

impl LocalSocketIpc {
    /// Binds the listener on `name`, takes the counter it mints `ClientId`s from, and the cap.
    pub fn bound(name: &str, numbers: Progressive, max_body: usize) -> std::io::Result<Self> {
        let ns = name.to_ns_name::<GenericNamespaced>()?;
        let listener = ListenerOptions::new()
            .name(ns)
            // ⛔ `Accept` AND NOT `Both`: the streams stay BLOCKING (D78, module doc).
            .nonblocking(ListenerNonblockingMode::Accept)
            .create_sync()?;
        Ok(LocalSocketIpc {
            listener,
            clients: Vec::new(),
            numbers,
            max_body,
        })
    }

    fn position_of(&self, client: ClientId) -> Option<usize> {
        self.clients.iter().position(|held| held.id == client)
    }

    /// Removes the client from the table. The reader thread is not told: it ends by itself when
    /// the peer's end closes -- which, for a client dropped here, is the case already.
    fn drop_client(&mut self, at: usize) {
        self.clients.remove(at);
    }
}

impl Ipc for LocalSocketIpc {
    fn accept(&mut self) -> Option<ClientId> {
        match self.listener.accept() {
            Ok(stream) => {
                // ⚠️ THE CLONE IS TAKEN BEFORE THE ID IS MINTED: a stream that cannot be cloned is
                // a client that never was, and the counter must not move for it.
                let Ok(reading) = stream.try_clone() else {
                    return None;
                };
                let id = ClientId::new(self.numbers.take());
                let (into, from_peer) = mpsc::channel();
                thread::spawn(move || read_until_the_end(reading, into));
                self.clients.push(Connected {
                    id,
                    stream,
                    from_peer,
                    pending: Vec::new(),
                    poisoned: false,
                });
                Some(id)
            }
            // ⛔ `WouldBlock` IS THE NORMAL STATE and not an error: nobody is knocking.
            Err(_) => None,
        }
    }

    fn send(&mut self, client: ClientId, message: &[u8]) -> Result<(), IpcError> {
        let Some(at) = self.position_of(client) else {
            return Err(IpcError::Disconnected);
        };
        // ⛔ VERBATIM, and BLOCKING (module doc): the message already carries its envelope.
        match self.clients[at].stream.write_all(message) {
            Ok(()) => Ok(()),
            Err(_) => {
                self.drop_client(at);
                Err(IpcError::Disconnected)
            }
        }
    }

    fn receive(&mut self, client: ClientId) -> Result<Option<Vec<u8>>, IpcError> {
        let Some(at) = self.position_of(client) else {
            return Err(IpcError::Disconnected);
        };
        if self.clients[at].poisoned {
            return Err(IpcError::MalformedMessage);
        }

        // Drain what the reader thread pushed since the last turn -- WITHOUT blocking.
        let mut ended = false;
        loop {
            match self.clients[at].from_peer.try_recv() {
                Ok(chunk) => self.clients[at].pending.extend_from_slice(&chunk),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    ended = true;
                    break;
                }
            }
        }

        if let Some(declared) = framing::declared_len(&self.clients[at].pending) {
            if declared > self.max_body {
                // ⛔ THE CLIENT STAYS. It is the peer's mistake, not its death (§3), and the
                // two outcomes must stay distinguishable to the caller.
                self.clients[at].poisoned = true;
                return Err(IpcError::MalformedMessage);
            }
        }

        if let Some((_, consumed)) = framing::take_frame(&self.clients[at].pending) {
            // ⛔ THE WHOLE FRAME, ENVELOPE INCLUDED, and not the body: `IpcMessage::decode`
            // unframes what it is given. Handing back the body would make every caller re-frame it.
            let whole = self.clients[at].pending[..consumed].to_vec();
            self.clients[at].pending.drain(..consumed);
            return Ok(Some(whole));
        }
        if ended {
            // ⛔ ORDER MATTERS: a peer that wrote a whole frame and left is heard FIRST, above,
            // and reported gone only when nothing whole is left. Then it LEAVES THE TABLE.
            self.drop_client(at);
            return Err(IpcError::Disconnected);
        }
        Ok(None)
    }
}'''

subs = [
    # --- Files (R10-16, D82, R1-12, D88) ---
    ("- Modify: `crates/kernel/tests/framing.rs` — le sonde del lettore di flusso, nelle due direzioni",
     "- Modify: `crates/kernel/tests/framing.rs` (**`i/lf w/crlf`**) — le sonde del lettore di flusso, nelle due direzioni"),
    ("- Create: `crates/kernel/tests/ipc_contract.rs` (**LF**) — la suite, `include!`-abile",
     "- Create: `crates/kernel/tests/contract/ipc.rs` (**LF**) — la suite, `include!`-abile; ⛔ **sotto `tests/contract/`, che cargo non scopre da sé: NON è un bersaglio di prova di `kernel` — D82, 2026-09-15**"),
    ("- Modify: `Cargo.lock` (**CRLF**) — **nello stesso commit** del manifesto (vincolo 6)",
     "- Modify: `Cargo.lock` (**`i/lf w/crlf` oggi — e `i/lf w/lf` dopo `cargo build`, che riscrive il lockfile in LF: misurato alla revisione del piano intero, R1-12; l'indice non cambia**) — **nello stesso commit** del manifesto (vincolo 6)"),
    ("falsa per `ipc` da questo compito\n- Read: la §3 del disegno del 2 per intero;",
     "falsa per `ipc` da questo compito\n- Modify: `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` (**LF**) — quattro richiami datati nella §3 (`send`: D10, D78; `receive`: D9, D10, D78; la riga «la suite di conformità»: D82; il capoverso 🔶 dedotto: D78) e uno nella §8 (la riga «il trasporto `ipc` in `platform`»: D82) — **D88**, Passo 9-ter\n- Read: la §3 del disegno del 2 per intero;"),
    ("⛔ **`ipc_contract.rs` è la casa UNICA delle asserzioni**, come `journal_contract.rs`: `platform` lo\nespande con `include!`",
     "⛔ **`tests/contract/ipc.rs` è la casa UNICA delle asserzioni**, come `journal_contract.rs`: `platform` lo\nespande con `include!`"),
    # --- Interfaces (R1-11) ---
    ("- Produces, e i compiti 7, 9 e 12 li usano con questi nomi esatti:\n  - `kernel::framing::declared_len(bytes: &[u8]) -> Option<usize>`",
     "- Produces, e i compiti 9 e 12 li usano con questi nomi esatti (⚠️ il 7 non consuma nulla di qui — il suo banco monta una finta, R1-11 — e `declared_len` ha un chiamante solo, il trasporto stesso):\n  - `kernel::framing::declared_len(bytes: &[u8]) -> Option<usize>`"),
    # --- Passo 1(a): the table row and the recall (R1-7, D78) ---
    ("quattro varianti: `Neither`, `Accept`, `Stream`, `Both` — **`Both`** è quella che serve |",
     "quattro varianti: `Neither`, `Accept`, `Stream`, `Both` — **`Accept`** è quella che serve: listener non bloccante, flussi **bloccanti** (⚠️ qui stava `Both`, corretto il 2026-09-15 per **D78**) |"),
    ("| `Stream::set_nonblocking(bool)` | `stream/trait.rs:44` | il flusso da solo |\n\n⛔ **E una trappola di Windows",
     "| `Stream::set_nonblocking(bool)` | `stream/trait.rs:44` | il flusso da solo — ⛔ **non si chiama** (D78) |\n\n"
     "✅ **RICHIAMO DEL 2026-09-15, dalla revisione del piano intero (R1-7, D78): la tabella resta vera riga per riga, ma LEGGERLA NON BASTAVA.**\n"
     "Il modello di questo compito, eseguito da fuori su Windows 11, ha reso **quattro sonde rosse su nove**: `receive`\n"
     "rispondeva `Disconnected` a un pari **vivo**. Con `PIPE_NOWAIT` una `read` che non trova dati fallisce con\n"
     "`ERROR_NO_DATA` (232), che `std` classifica `BrokenPipe`, e `interprocess::os::windows::misc::downgrade_eof`\n"
     "traduce ogni `BrokenPipe` in `Ok(0)`: pari muto e pari sparito sono **indistinguibili**. `accept` non bloccante\n"
     "invece risponde `WouldBlock`, misurato. La forma è **D78** — `ListenerNonblockingMode::Accept`, flussi bloccanti,\n"
     "un thread lettore per client su un `try_clone` del flusso, un canale `mpsc` che `receive` svuota senza bloccare —\n"
     "ed è il *«primo test rosso del piano»* che il 🔶 dedotto della §3 aspettava: il Passo 9-ter lo scrive nel disegno.\n\n"
     "⛔ **E una trappola di Windows"),
    # --- Passo 5: the suite (D82, D76) ---
    ("`crates/kernel/tests/ipc_contract.rs`, **LF**, nuovo. ⛔ **Commenti normali, non `//!`.**",
     "`crates/kernel/tests/contract/ipc.rs`, **LF**, nuovo — ⛔ **nella cartella `contract/` sotto `tests/`, che cargo non scopre: non è un bersaglio di `kernel` (D82)**. ⛔ **Commenti normali, non `//!`.**"),
    ("// THE CONFORMANCE SUITE OF THE `ipc` PORT (§3 of the milestone-2 design). It runs against every\n// implementation of the port, and what it is worth is that the fakes and the real transport\n// answer the SAME contract.\n",
     "// THE CONFORMANCE SUITE OF THE `ipc` PORT (§3 of the sub-project 2 design). ⛔ IT IS EXPANDED AGAINST\n// THE REAL TRANSPORT ONLY (D82): every promise worth holding here needs a peer that writes bytes,\n// and the two fakes of this port pass bytes verbatim and frame nothing -- expanded on them the\n// suite would be comparing itself. No liar either: one expansion has nothing to disagree with.\n"),
    ("// where `journal_contract_real.rs` puts its own.\n\n/// What every implementation must answer.",
     "// where `journal_contract_real.rs` puts its own.\n//\n// ⛔ THIS FILE IS NOT A TEST TARGET. It lives under `tests/contract/`, which cargo does not\n// auto-discover, so `kernel` neither compiles it alone nor warns about an unused macro (D82) --\n// `journal_contract.rs` next door IS a target only because it also holds a test of its own.\n\n/// What every implementation must answer."),
    # --- Passo 6: include path, the disconnect probe with its baseline (R1-15), the send probe (R1-10) ---
    ('include!("../../kernel/tests/ipc_contract.rs");',
     'include!("../../kernel/tests/contract/ipc.rs");'),
    ("fn a_peer_that_goes_away_is_disconnected_and_leaves_the_table() {\n    let name = socket_name_for_line(line!());\n    let mut ipc = LocalSocketIpc::bound(&name, Progressive::starting_at(0), CAP).expect(\"binds\");\n    let peer = a_peer_that_writes(name, Vec::new(), 0);\n    let client = accept_one(&mut ipc);\n    peer.join().expect(\"the peer ends\");\n",
     "fn a_peer_that_goes_away_is_disconnected_and_leaves_the_table() {\n    let name = socket_name_for_line(line!());\n    let mut ipc = LocalSocketIpc::bound(&name, Progressive::starting_at(0), CAP).expect(\"binds\");\n    let peer = a_peer_that_writes(name, Vec::new(), 200);\n    let client = accept_one(&mut ipc);\n    // ⛔ THE BASELINE FIRST: alive and silent is `Ok(None)`, NOT gone. Without this line the\n    // assertion below would be green on a transport that cannot tell the two apart -- which is\n    // exactly what a nonblocking read did on Windows (D78).\n    assert_eq!(ipc.receive(client), Ok(None), \"a live peer that has written nothing is not gone\");\n    peer.join().expect(\"the peer ends\");\n"),
    ("        \"and it STAYS gone: the client left the table, it was not merely reported once\"\n    );\n}\n```",
     "        \"and it STAYS gone: the client left the table, it was not merely reported once\"\n    );\n}\n\n"
     "#[test]\nfn a_send_to_a_peer_that_left_is_disconnected_and_drops_the_client() {\n"
     "    // ⛔ THE SECOND DIRECTION OF THE `send` ROW OF §3: a peer seen leaving THROUGH A WRITE, not\n"
     "    // through a read. The write is blocking (D78), so a write after the peer's death fails instead\n"
     "    // of pretending; the loop is bounded because the death can take a moment to reach the pipe.\n"
     "    let name = socket_name_for_line(line!());\n"
     "    let mut ipc = LocalSocketIpc::bound(&name, Progressive::starting_at(0), CAP).expect(\"binds\");\n"
     "    let peer = a_peer_that_writes(name, Vec::new(), 0);\n"
     "    let client = accept_one(&mut ipc);\n"
     "    peer.join().expect(\"the peer ends\");\n\n"
     "    let frame = framing::frame(b\"anyone there\").expect(\"frame\");\n"
     "    let started = std::time::Instant::now();\n"
     "    let seen = loop {\n"
     "        match ipc.send(client, &frame) {\n"
     "            Ok(()) if started.elapsed() < std::time::Duration::from_secs(2) => std::thread::yield_now(),\n"
     "            other => break other,\n"
     "        }\n"
     "    };\n"
     "    assert_eq!(seen, Err(IpcError::Disconnected), \"a write to a peer that left is `Disconnected`\");\n"
     "    assert_eq!(\n"
     "        ipc.receive(client),\n"
     "        Err(IpcError::Disconnected),\n"
     "        \"and the client LEFT THE TABLE on that write\"\n"
     "    );\n"
     "}\n```"),
    # --- Passo 9: module line, the recall on lib.rs (R1-14, D75), the expected counts (D82, R1-13) ---
    ("In `crates/platform/src/lib.rs` (**CRLF**), accanto agli altri moduli — l'ordine è di arrivo:",
     "In `crates/platform/src/lib.rs` (**CRLF**), **dopo `pub mod rng;`** — l'ordine è di arrivo (l'ancora, unica, è quella riga):"),
    ("⛔ **E il doc di modulo sopra, che questo compito rende falso — P-25.** Il richiamo si aggiunge in coda al doc, **senza riscrivere la frase**:",
     "⛔ **E il doc di modulo sopra, che questo compito rende falso — P-25.** Il richiamo si aggiunge **sotto il capoverso «⛔ RECALL OF 2026-08-28, FINDING AUD-022»** — quello che data la frase — con una riga `//!` vuota prima, e **senza riscrivere la frase** (R1-14: il *Trova* è l'ultima riga di quel capoverso, presa dal file col `grep -n -A6 'FINDING AUD-022'`):"),
    ("//! ⚠️ DATED RECALL, 2026-09-11 -- \"Today they are `Journal`, `Reactor` and `Rng`\" IS FALSE FROM THIS TASK:\n//! `ipc::LocalSocketIpc` is the fourth.",
     "//!\n//! ⚠️ DATED RECALL, <data> -- THE OPENING SENTENCE, the one that lists `Journal`, `Reactor` and `Rng`, IS\n//! FALSE FROM THIS TASK: `ipc::LocalSocketIpc` is the fourth."),
    ("Atteso: **nove** test verdi nel banco di `platform` (le tre della suite più le sei proprie); il banco\n`framing` del kernel verde con le quattro sonde nuove. ⚠️ `crates/kernel/tests/ipc_contract.rs` **non è\nun banco a sé**: definisce una macro e nessun `#[test]`, quindi `cargo test -p kernel` lo compila e\nriporta zero test — è voluto, e vale la pena scriverlo nel commit perché sembra un errore.",
     "Atteso: **dieci** test verdi nel banco di `platform` (le tre della suite più le sette proprie); il banco\n`framing` del kernel verde con le quattro sonde nuove. ⚠️ `crates/kernel/tests/contract/ipc.rs` **non è un\nbersaglio** di `kernel`: sta sotto `tests/contract/`, quindi `cargo test -p kernel` non lo compila e non\navvisa di una macro inutilizzata (D82 — l'avviso `unused macro definition` c'era, misurato alla revisione\ndel piano intero, R1-13, e il vincolo globale 15 vieta l'`#[allow]` che l'avrebbe zittito)."),
    # --- Passo 10: the loops, the add, the message, the expectation (R1-8, R1-12, D88) ---
    ("for f in crates/kernel/src/framing.rs crates/kernel/tests/framing.rs crates/platform/src/lib.rs crates/platform/Cargo.toml Cargo.lock; do printf '%s CR=' \"$f\"; tr -cd '\\r' < \"$f\" | wc -c; printf '   righe='; wc -l < \"$f\"; done\ngit ls-files --eol crates/kernel/src/framing.rs crates/kernel/tests/framing.rs crates/platform/src/lib.rs crates/platform/Cargo.toml Cargo.lock\n",
     "for f in crates/kernel/src/framing.rs crates/kernel/tests/framing.rs crates/platform/src/lib.rs crates/platform/Cargo.toml Cargo.lock crates/kernel/src/ports/mod.rs crates/kernel/tests/ports_are_implementable.rs; do printf '%s CR=' \"$f\"; tr -cd '\\r' < \"$f\" | wc -c; printf '   righe='; wc -l < \"$f\"; done\ngit ls-files --eol crates/kernel/src/framing.rs crates/kernel/tests/framing.rs crates/platform/src/lib.rs crates/platform/Cargo.toml Cargo.lock crates/kernel/src/ports/mod.rs crates/kernel/tests/ports_are_implementable.rs docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md\n"),
    ("Atteso: CR uguale alle righe sui CRLF e `git ls-files --eol` **invariato**; `GATE GREEN` — ⛔ e",
     "Atteso: CR uguale alle righe sui CRLF (`ports_are_implementable.rs` resta `i/crlf w/crlf`), il disegno `i/lf w/lf` a CR zero, e `git ls-files --eol` **invariato tranne `Cargo.lock`**, che `cargo build` riscrive in **LF** — `i/lf w/lf`, CR = 0, misurato alla revisione del piano intero (R1-12); l'indice era già LF e il diff porta solo le righe di `interprocess`; `GATE GREEN` — ⛔ e"),
    ("git add crates/kernel/src/framing.rs crates/kernel/tests/framing.rs crates/kernel/tests/ipc_contract.rs crates/platform/src/ipc.rs crates/platform/src/lib.rs crates/platform/tests/ipc_contract_real.rs crates/platform/Cargo.toml Cargo.lock docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md\ngit commit -m \"gui(compito 2): il lettore di flusso take_frame accanto a unframe, e il trasporto ipc in platform su interprocess 2.4.4 non bloccante; la suite di conformita ipc_contract.rs inclusa da platform, col pari su un thread\"",
     "git add crates/kernel/src/framing.rs crates/kernel/tests/framing.rs crates/kernel/tests/contract/ipc.rs crates/platform/src/ipc.rs crates/platform/src/lib.rs crates/platform/tests/ipc_contract_real.rs crates/platform/Cargo.toml Cargo.lock crates/kernel/src/ports/mod.rs crates/kernel/tests/ports_are_implementable.rs docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md\ngit commit -m \"gui(compito 2): il lettore di flusso take_frame accanto a unframe, e il trasporto ipc in platform su interprocess 2.4.4 -- listener non bloccante, flussi bloccanti e un thread lettore per client (D78); la suite di conformita in tests/contract/ipc.rs inclusa da platform sul solo trasporto vero (D82); i tre richiami di P-23 e i cinque nel disegno\""),
    # --- criterion ---
    ("- [ ] `cargo test --locked -p platform --test ipc_contract_real` → **nove** passati, e `receive` rende **la cornice intera** (D10), non il corpo",
     "- [ ] `cargo test --locked -p platform --test ipc_contract_real` → **dieci** passati, e `receive` rende **la cornice intera** (D10), non il corpo\n- [ ] `grep -c 'DATED RECALL, <data>' crates/kernel/src/ports/mod.rs` → **2**, `… crates/kernel/tests/ports_are_implementable.rs` → **1**, `grep -c 'RICHIAMO DEL <data>, compito 2' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` → **5** — tutti con la data del giorno scritta (D75) — e `grep -c '<data>'` sui quattro file toccati → **0**\n- [ ] `ls crates/kernel/tests/contract/ipc.rs` esiste e `cargo test --locked -p kernel 2>&1 | grep -c 'unused macro'` → **0** (D82)"),
    ("- [ ] i fine-riga rimisurati e `git ls-files --eol` invariato\n- [ ] la riga **2** della tabella della posizione a ✅ con la data",
     "- [ ] i fine-riga rimisurati e `git ls-files --eol` invariato — tranne `Cargo.lock`, `i/lf w/lf` (R1-12)\n- [ ] la riga **2** della tabella della posizione a ✅ con la data"),
    # --- position row 2 (D82) ---
    ("e la **suite di conformità** `ipc_contract.rs` inclusa da `platform` coi bugiardi; ⛔ **più i tre richiami che `ipc` rende falsi** — richiamo del 2026-09-11, **P-23** | uno | ⬜ |",
     "e la **suite di conformità** `tests/contract/ipc.rs` inclusa da `platform` (⚠️ **sul solo trasporto vero e senza bugiardi — D82, 2026-09-15**; qui stava «`ipc_contract.rs` … coi bugiardi»); ⛔ **più i tre richiami che `ipc` rende falsi** — richiamo del 2026-09-11, **P-23** — e i cinque nel disegno (**D88**). ⛔ **Il trasporto legge su un thread per client — D78, 2026-09-15** | uno | ⬜ |"),
]
for old, new in subs:
    n = t.count(old)
    assert n == 1, f"anchor count {n}: {old[:90]!r}"
for old, new in subs:
    t = t.replace(old, new, 1)

# --- Passo 8: the transport block, replaced wholesale ---
start_marker = "//! The `ipc` port over a local socket: one named pipe on Windows, one unix socket on Linux.\n"
assert t.count(start_marker) == 1
start = t.index(start_marker)
end_marker = "\n```\n\n- [ ] **Passo 9: il modulo nella crate, e il verde**"
end = t.index(end_marker, start)
old_block = t[start:end]
assert "ListenerNonblockingMode::Both" in old_block and old_block.count("\n```") == 0, "unexpected block bounds"
t = t[:start] + NEW_TRANSPORT + t[end:]

# --- Passi 9-bis and 9-ter, inserted before Passo 10 ---
p10 = "- [ ] **Passo 10: i fine-riga, il cancello, il commit**"
lo = t.index("\n## Compito 2:")
hi = t.index("\n## Compito 3:")
assert t.count(p10, lo, hi) == 1, t.count(p10, lo, hi)
p10_at = t.index(p10, lo, hi)
NEW_STEPS = r'''- [ ] **Passo 9-bis: i tre richiami di P-23, nei due file di `ports`** — ⛔ **arrivati qui dalla revisione del piano intero (R1-8): il blocco *Files* li prometteva e nessun Passo li scriveva**

Tutti e tre con `replace_unique.py`. I *Trova* sono righe **prese dal file** (`grep -n` prima), non ricopiate da qui.

**(1)** `crates/kernel/src/ports/mod.rs` (`i/lf w/crlf`): la riga della tabella del disegno che comincia con `//! | \`ipc\`` — `grep -n '^//! | \`ipc\`' crates/kernel/src/ports/mod.rs` → **una** riga, con la cella `milestone 6`. *Sostituisci con* la stessa riga in cui la cella diventa:

```
milestone 6 (the port) · sub-project 2, task 2 (`platform::ipc::LocalSocketIpc`, the real transport)
```

**(2)** stesso file: la riga che finisce con *«have NO CALLER AT ALL and are here for the reason above.»* (`grep -c -F` → 1). *Sostituisci con* la riga stessa, seguita da:

```
//! ⚠️ DATED RECALL, <data>, sub-project 2 task 2: `ipc` HAS ITS REAL IMPLEMENTATION NOW --
//! `platform::ipc::LocalSocketIpc` -- and its first caller arrives with task 7 (`kernel::serving`),
//! so the FOUR above are THREE from there on. The figure in the sentence is dated here and NOT
//! realigned (gotcha #31): the command that counts is `grep -rnE "^ *impl Ipc for" crates/`.
```

**(3)** `crates/kernel/tests/ports_are_implementable.rs` (⛔ **`i/crlf w/crlf`, CRLF anche nell'indice — P-3**): la riga **1**, *«//! One fake per port declared WITHOUT an implementation, and calls that exercise them»* (`head -1`). *Sostituisci con* la riga stessa, seguita da:

```
//! ⚠️ DATED RECALL, <data>, sub-project 2 task 2: `ipc` HAS an implementation now --
//! `platform::ipc::LocalSocketIpc` -- so "WITHOUT an implementation" is no longer true of every
//! fake here. The fakes stay, because a bench of `kernel` cannot open a socket, and the line
//! above is dated rather than rewritten.
```

```bash
grep -c 'DATED RECALL, <data>' crates/kernel/src/ports/mod.rs crates/kernel/tests/ports_are_implementable.rs
git ls-files --eol crates/kernel/src/ports/mod.rs crates/kernel/tests/ports_are_implementable.rs
```

Atteso (con la data scritta): **2** e **1**; `i/lf w/crlf` e `i/crlf w/crlf`, invariati.

- [ ] **Passo 9-ter: i cinque richiami nel disegno del 2 — D88**

`docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` è **LF**: Python con `newline=""`, ancora unica asserita, temporaneo e `os.replace` (la forma del Passo 3 del compito 11). ⛔ **Ogni ancora è la riga intera presa dal file col `grep`**, mai ricopiata da qui, e ogni richiamo si **appende in coda alla cella** che lo riceve, su **una** riga.

| Dove (`grep -n -F` sulla frase → una riga) | Che cosa si appende, in coda alla cella |
|---|---|
| §3, riga `\| \`send\` \|` — la cella *«incornicia con `kernel::framing::frame` e scrive; …»* | `✅ **RICHIAMO DEL <data>, compito 2 del piano della parte 2 (D10, D78):** il trasporto **non** incornicia — \`IpcMessage::encode\` incornicia già e \`send\` scrive verbatim, **bloccante**, sul flusso` |
| §3, riga `\| \`receive\` \|` — la cella *«lettura non bloccante nel buffer; …»* | `✅ **RICHIAMO DEL <data>, compito 2 (D9, D10, D78):** rende la **cornice intera**, busta compresa (\`decode\` la sbuccia); \`MalformedMessage\` nasce dal **tetto consegnato** e avvelena il flusso; e la lettura non è non bloccante — un thread lettore per client spinge i byte in un canale, che \`receive\` svuota senza bloccare` — e la **terza** cella della stessa riga, *«un bugiardo per promessa, sulle tre implementazioni»*, riceve ` ⚠️ **D82:** sul solo trasporto vero, senza bugiardi` |
| §3, riga `\| la suite di conformità \|` | `✅ **RICHIAMO DEL <data>, compito 2 (D82):** vive in \`crates/kernel/tests/contract/ipc.rs\` (non un bersaglio di \`kernel\`), gira sul **solo trasporto vero** e senza bugiardi — ogni sua promessa vuole un pari che scrive byte` |
| §3, il capoverso che comincia con `🔶 Dedotto: che anche i **flussi** di \`interprocess\` leggano senza bloccare` — l'ancora è la **prima** riga del capoverso, e il richiamo si appende in coda all'**ultima** (*«un passo (il doc vieta solo la nota senza intento).»*) | `✅ **RICHIAMO DEL <data>, compito 2 (D78):** la prima deduzione è **falsificata al primo test rosso**, come previsto — su Windows un flusso non bloccante rende \`Ok(0)\` a pari vivo — e i flussi restano bloccanti, letti da un thread per client; la seconda (la nota dopo l'esito) resta dedotta, e la prova il compito 6` |
| §8, riga `\| il trasporto \`ipc\` in \`platform\` (§3) \|` | `✅ **RICHIAMO DEL <data>, compito 2 (D82):** la suite è \`crates/kernel/tests/contract/ipc.rs\`, sul solo trasporto vero — dieci sonde, senza bugiardi` |

```bash
grep -c 'RICHIAMO DEL <data>, compito 2' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md
awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md
tr -cd '\r' < docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md | wc -c
```

Atteso (con la data scritta): **5**; niente; **0**.

'''
t = t[:p10_at] + NEW_STEPS + t[p10_at:]

tmp = PLAN + ".tmp"
io.open(tmp, "w", encoding="utf-8", newline="").write(t)
os.replace(tmp, PLAN)
print("ok: task 2 patched;", len(subs), "substitutions + transport + two steps;", t.count("\n") - raw.count("\n"), "lines added")
