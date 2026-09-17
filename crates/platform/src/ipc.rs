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
