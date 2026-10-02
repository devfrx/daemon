//! The `ipc` port over a local socket: one named pipe on Windows, one unix socket on Linux.
//!
//! ⛔ THE CHANNEL BELONGS TO ONE ACCOUNT OF THE OPERATING SYSTEM, AND THE SYSTEM HOLDS IT --
//! ADR-0041. `LocalSocketIpc::bound` is DELIVERED the `Account` the core runs as, which the
//! composition root asks of the system through `Account::of_this_process` and never picks here
//! (point 8), and before the first message the operating system keeps every other account out:
//! ONCE, at the root, with no check per function or per message, and with the kernel, the schema,
//! the `Hello` and the build stamp untouched (point 2).
//!
//! - WINDOWS: the pipe is created with an explicit security descriptor of ONE rule -- full access
//!   to the account's SID, and to nobody else -- and full access includes the right the server
//!   needs to create the instances after the first. `PIPE_REJECT_REMOTE_CLIENTS` and
//!   `FILE_FLAG_FIRST_PIPE_INSTANCE` stay, and `interprocess` 2.4.4 sets both by itself (point 4).
//! - LINUX: the socket is a FILE in the account's runtime directory, and no longer a name in the
//!   abstract namespace, which no permission reaches (`unix(7)`). Before binding, the directory
//!   must be the account's and closed to group and others, or the core does not start and says
//!   why (point 5); a socket left by a core that fell is removed only when no live core answers on
//!   it, and under a lock beside it, so that two starts cannot take the name from each other
//!   (point 6). Without `$XDG_RUNTIME_DIR` there is no channel at all (point 7).
//!
//! ⛔ WHOEVER IS REFUSED NEVER BECOMES A CLIENT: the system refuses it before the listener, so no
//! `ClientId` is minted, nothing reaches the journal and no message is read (point 9). The probes
//! of the refusals need an account this process is NOT, and live in the module at the foot of
//! this file for that reason; the rest are in `tests/ipc_contract_real.rs`. ⚠️ THE GUI'S OWN END
//! IS NOT HERE: it belongs to the shell, which does not exist yet, and the fake core and every
//! probe are Rust clients without that check -- ADR-0041, Negative consequences.
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
//! is the very growth the cap exists to refuse. Whether the reader thread should get
//! backpressure -- an `mpsc::sync_channel(N)`, which would block it instead of letting it queue --
//! is a MEASUREMENT for sub-project 3 and not a guess here, exactly as the per-client outgoing
//! queue of the paragraph above. What is not acceptable is leaving the cost unnamed, and until
//! this paragraph it was: found by the review of 2026-09-17 (I-2).
//!
//! ⚠️ AND IT IS NOT ONLY A BROKEN OR HOSTILE PEER THAT REACHES THAT BRANCH. ADR-0041 narrows who
//! can send a body to the processes of the core's own account and does not remove a bad one, which
//! a faulty gui can send too; and a SOUND gui reaches it with a layout package over the cap the
//! daemon delivers -- and is then unheard for the rest of its session, with no word to anyone,
//! because `kernel::serving` keeps a `MalformedMessage` client and answers nothing. Whether the
//! overflow should be told, or the package capped before the transport, is the owner's: audit of
//! 2026-09-30, AUD-051. ⚠️ RECALL OF 2026-10-02 -- same audit, AUD-051 and ADR-0041.
//!
//! ⛔ A `Vec` AND NOT A `HashMap` for the client table, the reckoning gotcha #12 records for the
//! kernel: the table holds ONE client in sub-project 2 (the gui is 0..1, ADR-0004), and a map
//! would buy nothing while introducing an iteration order.

use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;

use interprocess::local_socket::{
    prelude::*, GenericFilePath, Listener, ListenerNonblockingMode, ListenerOptions, Stream,
};
use interprocess::TryClone;
use kernel::framing;
use kernel::numbering::Progressive;
use kernel::ports::ipc::{ClientId, Ipc, IpcError};

/// How much a reader thread asks the stream for at a time. NOT a cap: bodies are bounded by the
/// delivered `max_body`, and a frame longer than this simply arrives in several chunks.
const CHUNK: usize = 4096;

/// The account of the operating system a channel belongs to: the ONLY one that may open it
/// (ADR-0041, point 1).
///
/// ⛔ ASKED OF THE SYSTEM AND DELIVERED, NEVER PICKED HERE (point 8): the composition root calls
/// `of_this_process` and hands the answer to `LocalSocketIpc::bound`, as it hands the name and the
/// cap. That is what makes the refusal provable with ONE account: a probe delivers another one.
///
/// ⚠️ AND THAT IS WHY THE ONLY PUBLIC CONSTRUCTOR ASKS THE SYSTEM. A constructor from a raw SID or
/// user id would have no caller but the probes, and it would let any caller in the workspace bind
/// a channel for an account the core does NOT run as -- an escape from point 1 bought to move a
/// probe. The probes that need a foreign account sit at the foot of this file instead, where the
/// private constructors are reachable: the argument `clients` already makes for its own privacy.
pub struct Account {
    /// The SID of the token's user, in its string form.
    #[cfg(windows)]
    sid: String,
    /// The EFFECTIVE user id: the one the system checks a file's owner against, and the owner of
    /// every file this process creates.
    #[cfg(unix)]
    uid: libc::uid_t,
}

impl Account {
    /// The account this process runs as, asked of the operating system.
    pub fn of_this_process() -> io::Result<Account> {
        #[cfg(windows)]
        {
            on_windows::sid_of_this_process().map(|sid| Account { sid })
        }
        #[cfg(unix)]
        {
            // SAFETY: `geteuid` takes nothing, touches no memory of ours, and cannot fail.
            Ok(Account {
                uid: unsafe { libc::geteuid() },
            })
        }
    }
}

/// Where the channel called `name` lives for THIS account (ADR-0041, points 4, 5 and 7).
///
/// - Windows: the pipe `\\.\pipe\<name>`. ⚠️ The pipe namespace is ONE PER MACHINE, so another
///   account that takes the name first keeps the core from starting -- a cost ADR-0041 accepts.
/// - Linux: the socket file `<$XDG_RUNTIME_DIR>/<name>`. ⛔ WITHOUT THAT VARIABLE THERE IS NO
///   CHANNEL, AND THE CORE DOES NOT START (point 7). XDG 0.8 asks an application to fall back to a
///   replacement directory and warn; the fallback is NOT built today, and the divergence is
///   declared in the ADR rather than taken here.
pub fn channel_of_this_account(name: &str) -> io::Result<PathBuf> {
    #[cfg(windows)]
    {
        Ok(PathBuf::from(format!(r"\\.\pipe\{name}")))
    }
    #[cfg(unix)]
    {
        on_unix::channel_in(std::env::var_os("XDG_RUNTIME_DIR"), name)
    }
}

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
    /// nothing to report it. ⛔ AND WHAT ARRIVES IS A `share` OF THE CORE'S ONE COUNTER, the one
    /// `kernel::serving::Core` numbers its steps from -- `kernel::numbering`.
    numbers: Progressive,
    /// The longest body this transport will buffer. ⛔ DELIVERED TOO, and it is what gives
    /// `IpcError::MalformedMessage` a producer: the envelope cannot tell a broken frame from a
    /// valid one -- any four bytes are a length -- so without a cap the promise of §3 would have
    /// nothing to hold, and a peer declaring four gibibytes would be buffered for ever.
    max_body: usize,
}

impl LocalSocketIpc {
    /// Binds the listener on `channel` for `account`, and takes the counter it mints `ClientId`s
    /// from and the cap.
    ///
    /// ⛔ ALL FOUR ARE DELIVERED: the channel and the account by `channel_of_this_account` and
    /// `Account::of_this_process` in production, and by the probes otherwise -- which is how a
    /// probe binds a channel of its own, and how one binds a channel for ANOTHER account.
    pub fn bound(
        channel: &Path,
        account: &Account,
        numbers: Progressive,
        max_body: usize,
    ) -> io::Result<Self> {
        let listener = listen(channel, account)?;
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

/// The listener, on Windows: the pipe, and the descriptor that keeps every other account out.
///
/// ⛔ `D:P(A;;GA;;;<SID>)`, THE DESCRIPTOR ADR-0041 MEASURED: a protected DACL of ONE rule, which
/// grants full access to the account's SID and therefore nothing to anybody else. Measured on
/// 2026-10-02 -- the ADR's protocol, in `docs/riferimenti.md` -- it takes three clients in a row,
/// and the SAME descriptor for another SID refuses this very process with `PermissionDenied`; the
/// two probes are `this_account_gets_in_three_times_in_a_row` and the refusal at the foot of this
/// file. ⚠️ FULL ACCESS IS THE ADR'S WORD, AND THE PROBES DO NOT HOLD IT AGAINST A NARROWER MASK:
/// the server needs the right to create the instances after the first, and MEASURED on 2026-10-02
/// a rule of `GRGW` takes three clients in a row as well -- generic write carries that right. What
/// they hold is that the rule admits the account, again and again, and refuses another one.
#[cfg(windows)]
fn listen(channel: &Path, account: &Account) -> io::Result<Listener> {
    use interprocess::os::windows::local_socket::ListenerOptionsExt;
    use interprocess::os::windows::security_descriptor::SecurityDescriptor;
    use widestring::U16CString;

    let rule = U16CString::from_str(format!("D:P(A;;GA;;;{})", account.sid)).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "the security descriptor of the channel holds a NUL",
        )
    })?;
    ListenerOptions::new()
        .name(channel.to_fs_name::<GenericFilePath>()?)
        // ⛔ `Accept` AND NOT `Both`: the streams stay BLOCKING (D78, module doc).
        .nonblocking(ListenerNonblockingMode::Accept)
        .security_descriptor(SecurityDescriptor::deserialize(&rule)?)
        .create_sync()
}

/// The listener, on Linux: the directory checked, a fallen core's socket cleared, then the bind.
///
/// ⛔ THE ORDER IS THE POINT. The directory first, because a socket in a directory others can
/// enter is a socket others can open (`unix(7)`: "pathname sockets honor the permissions of the
/// directory they are in"). Then the lock, and under it the question a socket file left on the
/// disk raises: a live core that answers stops this start, as a second core is stopped on
/// Windows; a file nobody answers on is a core that fell without unwinding, and it is removed --
/// the shape of git's `unix-stream-server.c`. ⚠️ WITHOUT THE LOCK two starts at once can each find
/// the other's socket unanswered and remove it, and both "succeed": ADR-0004's single instance
/// would be two.
#[cfg(unix)]
fn listen(channel: &Path, account: &Account) -> io::Result<Listener> {
    let directory = channel.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("the channel {} names no directory", channel.display()),
        )
    })?;
    on_unix::ensure_private(directory, account.uid)?;
    let lock = on_unix::lock_beside(channel)?;
    on_unix::clear_a_fallen_core(channel)?;
    let listener = ListenerOptions::new()
        .name(channel.to_fs_name::<GenericFilePath>()?)
        // ⛔ `Accept` AND NOT `Both`: the streams stay BLOCKING (D78, module doc).
        .nonblocking(ListenerNonblockingMode::Accept)
        .create_sync();
    // ⚠️ RELEASED ONLY ONCE THE NAME IS TAKEN, so the next start's question meets a live core.
    drop(lock);
    listener
}

#[cfg(windows)]
mod on_windows {
    use std::ffi::OsString;
    use std::io;
    use std::os::windows::ffi::OsStringExt;
    use std::ptr;

    use windows_sys::Win32::Foundation::{CloseHandle, LocalFree, HANDLE};
    use windows_sys::Win32::Security::Authorization::ConvertSidToStringSidW;
    use windows_sys::Win32::Security::{GetTokenInformation, TokenUser, TOKEN_QUERY, TOKEN_USER};
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    /// The token of this process, closed when it falls.
    struct Token(HANDLE);

    impl Drop for Token {
        fn drop(&mut self) {
            // SAFETY: the handle came from `OpenProcessToken`, and it is closed here and nowhere else.
            unsafe { CloseHandle(self.0) };
        }
    }

    /// The SID of the user this process runs as -- the token's user, not its owner -- in its
    /// string form.
    pub(super) fn sid_of_this_process() -> io::Result<String> {
        let mut handle: HANDLE = ptr::null_mut();
        // SAFETY: `GetCurrentProcess` is a pseudo-handle that needs no closing, and `handle` is a
        // valid place for the token to be written.
        if unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle) } == 0 {
            return Err(io::Error::last_os_error());
        }
        let token = Token(handle);

        // The first call only asks how large the answer is, and is expected to "fail" saying so.
        let mut needed = 0_u32;
        // SAFETY: a null buffer of length zero is written nothing; only `needed` is.
        unsafe { GetTokenInformation(token.0, TokenUser, ptr::null_mut(), 0, &mut needed) };
        if needed == 0 {
            return Err(io::Error::last_os_error());
        }
        // ⚠️ A BUFFER OF `u64` AND NOT OF `u8`: `TOKEN_USER` holds a pointer, and a buffer of bytes
        // promises no alignment for one.
        let mut buffer = vec![0_u64; (needed as usize).div_ceil(8)];
        // SAFETY: the buffer holds at least `needed` bytes and is aligned for `TOKEN_USER`.
        if unsafe {
            GetTokenInformation(token.0, TokenUser, buffer.as_mut_ptr().cast(), needed, &mut needed)
        } == 0
        {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: the call succeeded, so the buffer begins with a `TOKEN_USER`, and the SID it
        // points to lives inside the same buffer, which outlives every use below.
        let user = unsafe { &*buffer.as_ptr().cast::<TOKEN_USER>() };

        let mut wide: *mut u16 = ptr::null_mut();
        // SAFETY: `user.User.Sid` is a valid SID while `buffer` lives, and `wide` receives a string
        // the system allocates.
        if unsafe { ConvertSidToStringSidW(user.User.Sid, &mut wide) } == 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: on success `wide` is a NUL-terminated string, read up to its terminator, and
        // freed once with `LocalFree`, as the documentation of `ConvertSidToStringSidW` asks.
        let sid = unsafe {
            let length = (0..).take_while(|&at| *wide.add(at) != 0).count();
            let sid = OsString::from_wide(std::slice::from_raw_parts(wide, length));
            LocalFree(wide.cast());
            sid
        };
        sid.into_string().map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "the SID of this process is not Unicode")
        })
    }
}

#[cfg(unix)]
mod on_unix {
    use std::ffi::OsString;
    use std::fs::{self, File, OpenOptions};
    use std::io;
    use std::os::unix::fs::MetadataExt;
    use std::os::unix::net::UnixStream;
    use std::path::{Path, PathBuf};

    /// The channel called `name` in the runtime directory `runtime` names, if it names one.
    ///
    /// ⚠️ AN EMPTY VALUE NAMES NO DIRECTORY and is read as no value, which is stated rather than
    /// left to `PathBuf`: joined, it would make a path relative to wherever the core was started.
    pub(super) fn channel_in(runtime: Option<OsString>, name: &str) -> io::Result<PathBuf> {
        match runtime {
            Some(directory) if !directory.is_empty() => Ok(PathBuf::from(directory).join(name)),
            _ => Err(io::Error::new(
                io::ErrorKind::NotFound,
                "XDG_RUNTIME_DIR is not set, so this account has no runtime directory to put the \
                 channel in, and without one the core does not start (ADR-0041, point 7)",
            )),
        }
    }

    /// Refuses a directory that is not the account's, or that its group or others can enter.
    ///
    /// ⛔ ANY BIT FOR GROUP OR OTHERS REFUSES, the `0o077` mask, and not the "others" bits alone:
    /// XDG 0.8 wants the directory "0700", and a group can hold another account.
    pub(super) fn ensure_private(directory: &Path, uid: libc::uid_t) -> io::Result<()> {
        let held = fs::metadata(directory).map_err(|error| {
            io::Error::new(
                error.kind(),
                format!(
                    "the runtime directory {} cannot be read: {error}",
                    directory.display()
                ),
            )
        })?;
        if !held.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotADirectory,
                format!("the runtime directory {} is not a directory", directory.display()),
            ));
        }
        if held.uid() != uid {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!(
                    "the runtime directory {} belongs to the account {}, and the channel is for \
                     the account {uid} alone (ADR-0041, point 5)",
                    directory.display(),
                    held.uid()
                ),
            ));
        }
        if held.mode() & 0o077 != 0 {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!(
                    "the runtime directory {} is open to other accounts -- mode {:o} -- and the \
                     channel is for its own account alone (ADR-0041, point 5)",
                    directory.display(),
                    held.mode() & 0o777
                ),
            ));
        }
        Ok(())
    }

    /// The lock every start takes before it asks whether a core is alive, held until it binds.
    ///
    /// ⛔ A FILE BESIDE THE SOCKET, LOCKED WITH `File::lock` -- `flock` on Linux -- AND NEVER
    /// REMOVED. The lock dies with the process that holds it, so a core that falls while holding it
    /// leaves nothing to clear by hand; and removing the file would let a later start lock a NEW
    /// file of the same name while an earlier one still holds the old, which is two locks for one
    /// channel. It costs one empty file in a directory that is the account's alone.
    pub(super) fn lock_beside(channel: &Path) -> io::Result<File> {
        let mut path = channel.as_os_str().to_owned();
        path.push(".lock");
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)?;
        lock.lock()?;
        Ok(lock)
    }

    /// Stops the start if a core answers on `channel`, and removes the socket of one that fell.
    ///
    /// ⚠️ A LIVE CORE ACCEPTS THE QUESTION AS A CLIENT, and that is harmless: the connection is
    /// dropped at once, and that core forgets it the first time it reads from it.
    pub(super) fn clear_a_fallen_core(channel: &Path) -> io::Result<()> {
        match UnixStream::connect(channel) {
            Ok(_) => Err(io::Error::new(
                io::ErrorKind::AddrInUse,
                format!(
                    "a core is already listening on {}, and this one stops (ADR-0004)",
                    channel.display()
                ),
            )),
            Err(error) if error.kind() == io::ErrorKind::ConnectionRefused => {
                fs::remove_file(channel)
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error),
        }
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
            // ⛔ EVERY ERROR OF THE LISTENER COMES OUT AS `None`, AND ONLY ONE OF THEM IS ORDINARY:
            // `WouldBlock`, nobody is knocking. Any other is a listener that has itself broken, and
            // the port has no word for it -- the residue declared on `Ipc::accept`, real since this
            // transport exists. ⚠️ Whether the signature changes, or the `Option` stays and this
            // arm is declared for good, is the owner's: audit of 2026-09-30, AUD-533.
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
// HERE, and here the private things are TWO. The FIELD `clients`: from an integration test -- a
// crate of its own -- it is unreachable, so `self.clients.len()` cannot be read from
// `tests/ipc_contract_real.rs` at all. The precedent, and the wording, is
// `crates/platform/src/rng.rs`, which lives in `src/` because a private FIELD is what its probe
// needs, and `crates/kernel/src/arbiter/mod.rs`, which names that file as its own precedent. And,
// since ADR-0041, an `Account` this process is NOT, whose constructors stay private for the
// reason written on `Account`; with them the refusals of the system, which no public road may
// reach.
//
// ⛔ AND WHAT THE FIRST TWO GUARD IS A PROPERTY THE BENCH NEXT DOOR CANNOT SEE. The review of
// 2026-09-17 removed EACH of the two `self.drop_client(at)` calls in turn and all ten probes of
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

    /// An account this process is NOT -- for the probes of the refusals, and nowhere else.
    impl Account {
        /// ⛔ AN INVENTED SID, NOT A REAL GROUP: no token holds it, whatever groups the account
        /// running the probe belongs to. It is the very one ADR-0041 measured, in its protocol in
        /// `docs/riferimenti.md`.
        #[cfg(windows)]
        fn another_than_this_one() -> Account {
            Account {
                sid: String::from("S-1-5-21-1-2-3-1001"),
            }
        }

        /// The user id after this process's own: not this account, whoever runs the probe.
        #[cfg(unix)]
        fn another_than_this_one() -> Account {
            // SAFETY: `geteuid` takes nothing and cannot fail.
            Account {
                uid: unsafe { libc::geteuid() }.wrapping_add(1),
            }
        }
    }

    /// ⚠️ ITS OWN PREFIX, NOT THE ONE `tests/ipc_contract_real.rs` USES, and that is not caution:
    /// a channel is global to the machine -- a pipe name, or a directory in the shared temporary
    /// one -- a line number is unique inside ONE file only, and the two binaries can run at the
    /// same time. The pid separates two runs, the line two probes.
    ///
    /// ⛔ ON LINUX THE CHANNEL GETS A DIRECTORY OF ITS OWN, CLOSED TO EVERY OTHER ACCOUNT, because
    /// a directory any other account can enter is one `bound` refuses (ADR-0041, point 5) -- and
    /// the probes must not lean on `$XDG_RUNTIME_DIR`, which the machine running them may not have.
    fn channel_for_line(line: u32) -> PathBuf {
        let name = format!("harness-ipc-unit-{}-{}", std::process::id(), line);
        #[cfg(windows)]
        {
            PathBuf::from(format!(r"\\.\pipe\{name}"))
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let directory = std::env::temp_dir().join(name);
            let _ = std::fs::remove_dir_all(&directory);
            std::fs::create_dir_all(&directory).expect("a fresh directory for this probe");
            std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(0o700))
                .expect("the directory is closed to every other account");
            directory.join("core")
        }
    }

    fn this_account() -> Account {
        Account::of_this_process().expect("the system names the account this probe runs as")
    }

    /// Connects to `channel`, holds the connection for `hold_ms`, then lets it go -- which is what
    /// makes the peer dead. ⚠️ IT HOLDS, AND THE HOLD IS NOT DECORATION (E5 of the plan): a peer
    /// that dies before the first `accept` is a peer that is never accepted at all.
    fn a_peer_that_stays_then_leaves(channel: PathBuf, hold_ms: u64) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let name = channel.as_path().to_fs_name::<GenericFilePath>().expect("name");
            let started = std::time::Instant::now();
            let _stream = loop {
                match Stream::connect(name.clone()) {
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
        let channel = channel_for_line(line!());
        let mut ipc =
            LocalSocketIpc::bound(&channel, &this_account(), Progressive::starting_at(0), CAP)
                .expect("binds");
        let peer = a_peer_that_stays_then_leaves(channel, 200);
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
        let channel = channel_for_line(line!());
        let mut ipc =
            LocalSocketIpc::bound(&channel, &this_account(), Progressive::starting_at(0), CAP)
                .expect("binds");
        let peer = a_peer_that_stays_then_leaves(channel, 200);
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

    /// ⛔ THE REFUSAL OF ADR-0041 ON WINDOWS, POINTS 4 AND 9 TOGETHER: a channel bound for another
    /// account refuses THIS process when it connects, and the refused connection never becomes a
    /// client. Its other direction -- this account gets in, three times in a row -- is
    /// `this_account_gets_in_three_times_in_a_row` in `tests/ipc_contract_real.rs`.
    ///
    /// ⛔ "NEVER BECOMES A CLIENT" IS READ ON THE COUNTER, which the probe keeps a `share` of: a
    /// refused peer that had reached the listener would have been handed a `ClientId`, and the
    /// counter would have moved.
    #[cfg(windows)]
    #[test]
    fn a_peer_outside_the_account_is_refused_by_the_system_and_never_becomes_a_client() {
        let channel = channel_for_line(line!());
        let numbers = Progressive::starting_at(0);
        let mut ipc = LocalSocketIpc::bound(
            &channel,
            &Account::another_than_this_one(),
            numbers.share(),
            CAP,
        )
        .expect("a channel for another account binds: the refusal is the system's, at connect");

        let refused = Stream::connect(
            channel.as_path().to_fs_name::<GenericFilePath>().expect("name"),
        );
        assert_eq!(
            refused.err().map(|error| error.kind()),
            Some(io::ErrorKind::PermissionDenied),
            "the system refuses a process of another account before the listener sees it"
        );
        assert_eq!(ipc.accept(), None, "nobody reached the listener, so nobody is accepted");
        assert_eq!(
            numbers.take(),
            0,
            "and no `ClientId` was minted: the counter has not moved"
        );
    }

    /// ⛔ THE REFUSAL OF ADR-0041 ON LINUX, POINT 5: a directory that is not the account's stops the
    /// start, and nothing is bound in it. The other direction -- the account's own directory binds
    /// and its account gets in -- is `this_account_gets_in_three_times_in_a_row`.
    #[cfg(unix)]
    #[test]
    fn a_channel_for_another_account_does_not_bind() {
        let channel = channel_for_line(line!());
        let refused = LocalSocketIpc::bound(
            &channel,
            &Account::another_than_this_one(),
            Progressive::starting_at(0),
            CAP,
        );
        assert_eq!(
            refused.err().map(|error| error.kind()),
            Some(io::ErrorKind::PermissionDenied),
            "the directory is this account's, and the channel was asked for another one"
        );
        assert!(!channel.exists(), "and nothing was bound in it");
    }

    /// ⛔ POINT 7, IN BOTH DIRECTIONS: no `$XDG_RUNTIME_DIR` -- or an empty one -- is no channel, and
    /// one that is set puts the channel inside it. Read through the function that takes the value,
    /// because the variable itself belongs to the process and the probes run in parallel.
    #[cfg(unix)]
    #[test]
    fn without_a_runtime_directory_there_is_no_channel() {
        assert_eq!(
            on_unix::channel_in(None, "core").err().map(|error| error.kind()),
            Some(io::ErrorKind::NotFound),
            "no runtime directory, no channel"
        );
        assert_eq!(
            on_unix::channel_in(Some(std::ffi::OsString::new()), "core")
                .err()
                .map(|error| error.kind()),
            Some(io::ErrorKind::NotFound),
            "an empty value names no directory"
        );
        assert_eq!(
            on_unix::channel_in(Some("/run/user/1000".into()), "core").ok(),
            Some(PathBuf::from("/run/user/1000/core")),
            "a runtime directory holds the channel"
        );
    }

    /// ⛔ THE LOCK OF POINT 6: a start that finds the channel's lock held WAITS, and the question
    /// it then asks meets the core that held it. Without the lock it would not wait: it would find
    /// nobody, bind, and "succeed" beside the other start.
    ///
    /// ⚠️ THE INTERVAL IS REAL, because the first assertion is about something that has NOT
    /// happened -- the start has not returned -- and that is the one case this repository lets a
    /// probe sleep. It decides only the red direction: with the lock, the start cannot return
    /// while the lock is held, however long the wait.
    #[cfg(unix)]
    #[test]
    fn a_start_that_finds_the_lock_held_waits_and_then_meets_the_core() {
        let channel = channel_for_line(line!());
        // "Another start is in progress": its lock is held, and its core is not bound yet.
        let held = on_unix::lock_beside(&channel).expect("the lock is taken");

        let waiting = {
            let channel = channel.clone();
            thread::spawn(move || {
                LocalSocketIpc::bound(&channel, &this_account(), Progressive::starting_at(0), CAP)
                    .map(|_| ())
                    .map_err(|error| error.kind())
            })
        };
        thread::sleep(std::time::Duration::from_millis(200));
        assert!(!waiting.is_finished(), "a start must wait while another start holds the lock");

        // The other start binds its core, and lets the lock go.
        let _other_core = std::os::unix::net::UnixListener::bind(&channel)
            .expect("the other start binds while it holds the lock");
        drop(held);

        assert_eq!(
            waiting.join().expect("the waiting start does not panic"),
            Err(io::ErrorKind::AddrInUse),
            "and once the lock is free, the waiting start meets a live core and stops"
        );
    }
}
