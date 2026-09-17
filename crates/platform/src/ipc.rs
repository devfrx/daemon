//! The `ipc` port over a local socket: one named pipe on Windows, one unix socket on Linux.
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
//! ⛔ AND A POISONED CLIENT IS NO LONGER DRAINED, WHICH IS THE SECOND DECLARED LIMIT OF THIS
//! FILE. Once a declared length has passed the cap, `receive` answers `MalformedMessage` BEFORE
//! the drain loop, while the reader thread keeps reading and pushing into an `mpsc::channel()`
//! that has NO bound on its capacity. Both halves of that state are deliberate -- `poisoned` is
//! never cleared, and the client STAYS in the table because a peer's mistake is not its death
//! (D9) -- so the condition is PERMANENT, and a broken or hostile peer that keeps writing after
//! declaring an over-cap length buffers its bytes in the channel instead of in `pending`, which
//! is the very growth the cap exists to refuse. In sub-project 2 the peer is our own gui, local
//! and trusted, and nothing but a broken or hostile one reaches this branch; whether the reader
//! thread should get backpressure -- an `mpsc::sync_channel(N)`, which would block it instead of
//! letting it queue -- is a MEASUREMENT for sub-project 3 and not a guess here, exactly as the
//! per-client outgoing queue of the paragraph above. What is not acceptable is leaving the cost
//! unnamed, and until this paragraph it was: found by the review of 2026-09-17 (I-2).
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
}

// ⚠️ A UNIT TEST MODULE IN `src/`, WHERE THIS PORT OTHERWISE PUTS EVERY PROBE IN `tests/`, and
// the deviation is declared rather than left to be noticed. ⛔ ONLY PRIVACY MOVES A PROBE IN
// HERE, and here the private thing is the FIELD `clients`: from an integration test -- a crate
// of its own -- it is unreachable, so `self.clients.len()` cannot be read from
// `tests/ipc_contract_real.rs` at all. The precedent, and the wording, is
// `crates/platform/src/rng.rs`, which lives in `src/` because a private FIELD is what its probe
// needs, and `crates/kernel/src/arbiter/mod.rs`, which names that file as its own precedent.
//
// ⛔ AND WHAT THEY GUARD IS A PROPERTY THE BENCH NEXT DOOR CANNOT SEE. The review of 2026-09-17
// removed EACH of the two `self.drop_client(at)` calls in turn and all ten probes of
// `tests/ipc_contract_real.rs` stayed GREEN: their oracle is `receive`, and `receive` answers
// `Disconnected` in two indistinguishable ways -- the client is not in the table, OR it is and
// its channel is closed with nothing whole left. The OBSERVABLE is the same either way; the
// table is not, and this is the only place from which anybody can look at it. Without these two
// probes the table would leak a `Connected` -- a `Stream`, a `Receiver` and a `pending` buffer --
// per dead peer, and nothing would go red.
//
// ⛔ THE CHEAP WAY OUT WAS REFUSED ON THE MERITS: making `clients` `pub` so that the probes
// could sit in `tests/` would publish the table to the whole workspace to buy a file move, and
// making room is a CONSEQUENCE of a request, never a thing somebody asks for. It is the same
// argument `rng.rs` and `arbiter/mod.rs` make for their own private state.
#[cfg(test)]
mod tests {
    use super::*;

    /// The cap these probes hand over. Nothing here writes a body, so the value does not matter.
    const CAP: usize = 4_096;

    /// ⚠️ ITS OWN PREFIX, NOT THE ONE `tests/ipc_contract_real.rs` USES, and that is not caution:
    /// a socket name is global to the machine, a line number is unique inside ONE file only, and
    /// the two binaries can run at the same time. The pid separates two runs, the line two probes.
    fn socket_name_for_line(line: u32) -> String {
        format!("harness-ipc-unit-{}-{}", std::process::id(), line)
    }

    /// Connects to `name`, holds the connection for `hold_ms`, then lets it go -- which is what
    /// makes the peer dead. ⚠️ IT HOLDS, AND THE HOLD IS NOT DECORATION (E5 of the plan): a peer
    /// that dies before the first `accept` is a peer that is never accepted at all.
    fn a_peer_that_stays_then_leaves(name: String, hold_ms: u64) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let ns = name.to_ns_name::<GenericNamespaced>().expect("name");
            let started = std::time::Instant::now();
            let _stream = loop {
                match Stream::connect(ns.clone()) {
                    Ok(stream) => break stream,
                    Err(_) if started.elapsed() < std::time::Duration::from_secs(5) => {
                        thread::yield_now()
                    }
                    Err(error) => panic!("the peer never connected in five seconds: {error}"),
                }
            };
            thread::sleep(std::time::Duration::from_millis(hold_ms));
        })
    }

    /// ⛔ WITH A CAP AND A `panic!` AND NOT A BARE `loop` (E6 of the plan): a probe that hangs
    /// eats the gate without printing one line, which is the least readable red there is.
    fn accept_one(ipc: &mut LocalSocketIpc) -> ClientId {
        let started = std::time::Instant::now();
        loop {
            if let Some(client) = ipc.accept() {
                return client;
            }
            if started.elapsed() > std::time::Duration::from_secs(5) {
                panic!("nobody was accepted in five seconds: the peer died before `accept` (E5)");
            }
            thread::yield_now();
        }
    }

    #[test]
    fn a_peer_reported_gone_by_receive_is_out_of_the_table() {
        let name = socket_name_for_line(line!());
        let mut ipc =
            LocalSocketIpc::bound(&name, Progressive::starting_at(0), CAP).expect("binds");
        let peer = a_peer_that_stays_then_leaves(name, 200);
        let client = accept_one(&mut ipc);
        // ⛔ THE BEFORE, MEASURED AND NOT ASSUMED: without it a table that never grew would pass
        // the assertion below for the wrong reason.
        assert_eq!(ipc.clients.len(), 1, "an accepted peer IS in the table");
        peer.join().expect("the peer ends");

        let started = std::time::Instant::now();
        let seen = loop {
            match ipc.receive(client) {
                Ok(None) if started.elapsed() < std::time::Duration::from_secs(5) => {
                    thread::yield_now()
                }
                other => break other,
            }
        };
        assert_eq!(seen, Err(IpcError::Disconnected), "a peer that left is `Disconnected`");
        assert_eq!(
            ipc.clients.len(),
            0,
            "and the TABLE SHRANK. `receive` answering `Disconnected` a second time proves \
             nothing here -- it says that for a client that is not in the table too -- so this \
             length is the only witness that the row was actually removed"
        );
    }

    #[test]
    fn a_peer_a_send_finds_gone_is_out_of_the_table() {
        let name = socket_name_for_line(line!());
        let mut ipc =
            LocalSocketIpc::bound(&name, Progressive::starting_at(0), CAP).expect("binds");
        let peer = a_peer_that_stays_then_leaves(name, 200);
        let client = accept_one(&mut ipc);
        assert_eq!(ipc.clients.len(), 1, "an accepted peer IS in the table");
        peer.join().expect("the peer ends");

        // ⛔ THE SECOND DIRECTION, AND IT IS A DIFFERENT CALL SITE: the peer is seen leaving
        // through a WRITE. The loop is bounded because the death takes a moment to reach the pipe.
        let frame = framing::frame(b"anyone there").expect("frame");
        let started = std::time::Instant::now();
        let seen = loop {
            match ipc.send(client, &frame) {
                Ok(()) if started.elapsed() < std::time::Duration::from_secs(5) => {
                    thread::yield_now()
                }
                other => break other,
            }
        };
        assert_eq!(seen, Err(IpcError::Disconnected), "a write to a peer that left is `Disconnected`");
        assert_eq!(
            ipc.clients.len(),
            0,
            "and the TABLE SHRANK ON THAT WRITE, which is the half `send`'s own oracle cannot \
             reach: the caller sees `Disconnected` whether or not the row is still there"
        );
    }
}
