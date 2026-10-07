//! The `ipc` conformance suite expanded against the REAL transport, plus the promises only a
//! real peer can exercise -- among them who may open the channel at all (ADR-0041).
//!
//! ⚠️ THE CHANNEL IS PER-TEST AND CARRIES THE LINE NUMBER, the shape
//! `crates/daemon/src/main.rs` already uses for its private directories -- except in `build`,
//! where a counter numbers the calls because all three suite tests share that one call site.
//! Two tests sharing a channel pass alone and fail together, which is the flakiest red there is.
//!
//! ⚠️ THE WAITS ARE BOUNDED `yield_now` LOOPS AND NOT FIXED SLEEPS, except where the assertion is
//! about something that has NOT arrived -- there an interval must elapse, and that one is named.
//!
//! ⛔ AND EVERY WAIT HAS A CEILING AND A `panic!`, those on `receive` included: a regression of
//! the transport must come out as a red that names what it waited for, never as a gate that hangs
//! without a line (E5 and E6 of the sub-project 2 plan). ⚠️ RECALL OF 2026-10-02 -- audit of
//! 2026-09-30, AUD-703. ⚠️ RECALL OF 2026-10-07 -- audit of 2026-09-30, the re-verification of
//! P24: which waits are on `receive` says
//! `grep -n 'the_next_answer(&mut' crates/platform/tests/ipc_contract_real.rs`, never a count here.

use std::io::Write;
use std::path::PathBuf;

use kernel::framing;
use kernel::numbering::Progressive;
use kernel::ports::ipc::{ClientId, Ipc, IpcError};
use platform::ipc::{Account, LocalSocketIpc};

/// The cap every test in this file hands over. Big enough for any body written here, small
/// enough that `a_body_over_the_cap_is_malformed` can exceed it without allocating.
const CAP: usize = 4_096;

/// How long any wait of this file lasts before it gives a verdict. Generous against a loaded
/// machine, and far below a hang.
const CEILING: std::time::Duration = std::time::Duration::from_secs(5);

/// The channel called `name`: a pipe on Windows; on Linux, a socket file in a directory of its
/// own, closed to every other account.
///
/// ⛔ THE DIRECTORY IS NOT DECORATION ON LINUX: `LocalSocketIpc::bound` refuses one that the group
/// or others can enter (ADR-0041, point 5), and a probe must not lean on the `$XDG_RUNTIME_DIR` of
/// whatever machine runs it. ⚠️ THE MODE IS SET AFTER THE CREATION, and not left to it: a umask
/// may clear bits but never set them, and `set_permissions` writes exactly the bits named.
fn channel_named(name: String) -> PathBuf {
    #[cfg(windows)]
    {
        PathBuf::from(format!(r"\\.\pipe\{name}"))
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let directory = std::env::temp_dir().join(name);
        let _ = std::fs::remove_dir_all(&directory);
        std::fs::create_dir_all(&directory).expect("a fresh directory for this channel");
        std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(0o700))
            .expect("the directory is closed to every other account");
        directory.join("core")
    }
}

fn channel_for_line(line: u32) -> PathBuf {
    channel_named(format!("harness-ipc-contract-{}-{}", std::process::id(), line))
}

fn this_account() -> Account {
    Account::of_this_process().expect("the system names the account this probe runs as")
}

/// ⛔ A FRESH NUMBER PER CALL AND NOT `line!()`, WHICH WOULD EXPAND ONCE. This helper is the
/// one place where `line!()` is not at the call site: all three suite tests call it, and
/// libtest runs them on parallel threads, so they would all bind THE SAME channel -- the trap
/// the module doc above names. The shape is `journal_contract_real.rs`'s `AtomicU64`.
///
/// ⚠️ AND THE PREFIX IS NOT `channel_for_line`'s, which is the half a counter alone does not
/// cover: the third call would be number 2, and a test on line 2 would collide with it.
fn build() -> LocalSocketIpc {
    static NEXT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let nth = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    LocalSocketIpc::bound(
        &channel_named(format!("harness-ipc-suite-{}-{}", std::process::id(), nth)),
        &this_account(),
        Progressive::starting_at(0),
        CAP,
    )
    .expect("the listener binds")
}

include!("../../kernel/tests/contract/ipc.rs");

ipc_contract_suite!(build);

/// Connects, writes `bytes` verbatim, and stays open for `hold`.
///
/// ⛔ THE CONNECT HAS A CEILING TOO: a listener that never comes up would otherwise leave this
/// thread retrying for as long as the process lives, behind a test that `accept_one` has already
/// failed.
fn a_peer_that_writes(channel: PathBuf, bytes: Vec<u8>, hold_ms: u64) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        use interprocess::local_socket::{prelude::*, GenericFilePath, Stream};
        let name = channel.as_path().to_fs_name::<GenericFilePath>().expect("name");
        let started = std::time::Instant::now();
        let mut stream = loop {
            match Stream::connect(name.clone()) {
                Ok(stream) => break stream,
                Err(_) if started.elapsed() < CEILING => std::thread::yield_now(),
                Err(error) => panic!("the peer never connected in {CEILING:?}: {error}"),
            }
        };
        stream.write_all(&bytes).expect("write");
        std::thread::sleep(std::time::Duration::from_millis(hold_ms));
    })
}

/// ⛔ THE CEILING IS WHAT MAKES A PEER THAT DIED BEFORE `accept` A RED INSTEAD OF A SILENCE, and
/// it is here because this loop was the UNBOUNDED one while the send loop below already carried a
/// ceiling with the reason written beside it. The plan bounded the loop it thought could spin and
/// left open the one that did: a peer built with `hold_ms = 0` connected and left before the first
/// `accept`, the listener cleared the empty connection, and this waited for a client that was
/// never coming again -- twenty-three minutes of a gate that printed NOTHING (E5 of the plan).
///
/// ⚠️ E5 CLOSES THE RACE; THIS TURNS A RECURRENCE INTO A VERDICT. A hang yields no verdict at
/// all, so it cannot be read, bisected or reported -- it only eats the gate.
fn accept_one(ipc: &mut LocalSocketIpc) -> ClientId {
    let started = std::time::Instant::now();
    loop {
        if let Some(client) = ipc.accept() {
            return client;
        }
        if started.elapsed() > CEILING {
            panic!("nobody was accepted in {CEILING:?}: the peer died before `accept` (E5)");
        }
        std::thread::yield_now();
    }
}

/// What `receive` answers next that is not `Ok(None)`: a whole frame, or the error.
///
/// ⛔ THE ONE WAIT ON `receive` IN THIS FILE, AND IT HAS THE CEILING `accept_one` HAS, for the
/// reason `accept_one` gives. The defects these probes exist to catch are exactly the ones in which
/// `receive` never stops answering `Ok(None)` -- a peer that left and is never seen leaving, a
/// frame that never comes whole -- so an unbounded loop here turns the red into a hang. With the
/// end of the stream never noticed, `a_peer_that_goes_away_is_disconnected_and_leaves_the_table`
/// HUNG before this helper -- measured by the audit's verification of AUD-703 -- and with it goes
/// red within the ceiling, naming the client: MEASURED on 2026-10-02.
fn the_next_answer(ipc: &mut LocalSocketIpc, client: ClientId) -> Result<Vec<u8>, IpcError> {
    let started = std::time::Instant::now();
    loop {
        match ipc.receive(client) {
            Ok(Some(frame)) => return Ok(frame),
            Err(error) => return Err(error),
            Ok(None) if started.elapsed() < CEILING => std::thread::yield_now(),
            Ok(None) => panic!(
                "`receive` answered nothing for {CEILING:?} for {client:?}: no frame came whole, \
                 and the peer was never seen leaving"
            ),
        }
    }
}

#[test]
fn a_whole_frame_comes_back_whole_envelope_included() {
    let channel = channel_for_line(line!());
    let mut ipc = LocalSocketIpc::bound(&channel, &this_account(), Progressive::starting_at(0), CAP)
        .expect("binds");
    let peer = a_peer_that_writes(channel, framing::frame(b"hello core").expect("frame"), 300);

    let client = accept_one(&mut ipc);
    let seen = the_next_answer(&mut ipc, client).expect("a live client");
    assert_eq!(
        seen,
        framing::frame(b"hello core").expect("frame"),
        "THE WHOLE FRAME comes back, envelope included: `IpcMessage::decode` unframes it"
    );
    peer.join().expect("the peer ends");
}

#[test]
fn two_frames_in_one_write_come_back_one_at_a_time() {
    // ⛔ THE CASE THAT BREAKS A TRANSPORT BUILT ON `unframe`: two frames in one read.
    let channel = channel_for_line(line!());
    let mut ipc = LocalSocketIpc::bound(&channel, &this_account(), Progressive::starting_at(0), CAP)
        .expect("binds");
    let mut bytes = framing::frame(b"first").expect("frame");
    bytes.extend_from_slice(&framing::frame(b"second").expect("frame"));
    let peer = a_peer_that_writes(channel, bytes, 300);

    let client = accept_one(&mut ipc);
    let first = the_next_answer(&mut ipc, client).expect("a live client");
    let second = the_next_answer(&mut ipc, client).expect("a live client");
    assert_eq!(first, framing::frame(b"first").expect("frame"));
    assert_eq!(second, framing::frame(b"second").expect("frame"));
    peer.join().expect("the peer ends");
}

#[test]
fn two_accepts_hand_out_different_and_ascending_numbers() {
    // ⛔ THE PROMISE IS THE COUNTER'S, checked HERE because `accept` is the only site that mints
    // a `ClientId`: a transport that started a private `u64` would pass every other test here.
    let channel = channel_for_line(line!());
    let mut ipc =
        LocalSocketIpc::bound(&channel, &this_account(), Progressive::starting_at(41), CAP)
            .expect("binds");
    let first_peer = a_peer_that_writes(channel.clone(), Vec::new(), 300);
    let first = accept_one(&mut ipc);
    let second_peer = a_peer_that_writes(channel, Vec::new(), 300);
    let second = accept_one(&mut ipc);

    assert_eq!(first, ClientId::new(41), "the counter was started at 41");
    assert_eq!(second, ClientId::new(42));
    first_peer.join().expect("peer one ends");
    second_peer.join().expect("peer two ends");
}

#[test]
fn the_transport_draws_its_numbers_from_the_counter_it_shares() {
    // ⛔ THE ONE COUNTER OF THE CORE, SEEN FROM THE TRANSPORT'S SIDE: what the transport mints, the
    // other holder of the same counter never hands out, and the other way round. The probe above
    // cannot see it -- a counter started at 41 ascends whether or not anybody else holds it.
    let channel = channel_for_line(line!());
    let numbers = Progressive::starting_at(0);
    let mut ipc = LocalSocketIpc::bound(&channel, &this_account(), numbers.share(), CAP)
        .expect("binds");

    let first_peer = a_peer_that_writes(channel.clone(), Vec::new(), 300);
    let first = accept_one(&mut ipc);
    let taken_between = numbers.take();
    let second_peer = a_peer_that_writes(channel, Vec::new(), 300);
    let second = accept_one(&mut ipc);

    assert_eq!(
        (first, taken_between, second),
        (ClientId::new(0), 1, ClientId::new(2)),
        "the client took 0, the other holder 1, the next client 2: ONE sequence"
    );
    first_peer.join().expect("peer one ends");
    second_peer.join().expect("peer two ends");
}

#[test]
fn half_a_frame_is_not_a_body_yet_and_not_an_error() {
    let channel = channel_for_line(line!());
    let mut ipc = LocalSocketIpc::bound(&channel, &this_account(), Progressive::starting_at(0), CAP)
        .expect("binds");
    let whole = framing::frame(b"a body long enough to cut").expect("frame");
    let half = whole[..whole.len() - 3].to_vec();
    let peer = a_peer_that_writes(channel, half, 300);

    let client = accept_one(&mut ipc);
    // ⚠️ A REAL INTERVAL, because the assertion is about what has NOT arrived.
    std::thread::sleep(std::time::Duration::from_millis(80));
    assert_eq!(ipc.receive(client), Ok(None));
    peer.join().expect("the peer ends");
}

#[test]
fn a_body_over_the_cap_is_malformed_and_the_client_stays() {
    // ⛔ THIS IS THE ONLY PRODUCER OF `MalformedMessage` IN THIS TRANSPORT, and D9 says why:
    // the envelope cannot tell a broken frame from a valid one, so the cap is what makes the
    // promise of §3 true. ⚠️ NOTHING BIG IS ALLOCATED: only the prefix is written.
    let channel = channel_for_line(line!());
    let mut ipc = LocalSocketIpc::bound(&channel, &this_account(), Progressive::starting_at(0), CAP)
        .expect("binds");
    let over = ((CAP + 1) as u32).to_be_bytes().to_vec();
    let peer = a_peer_that_writes(channel, over, 300);

    let client = accept_one(&mut ipc);
    let seen = the_next_answer(&mut ipc, client);
    assert_eq!(seen, Err(IpcError::MalformedMessage));
    assert_eq!(
        ipc.receive(client),
        Err(IpcError::MalformedMessage),
        "THE CLIENT STAYS: it is the peer's mistake, not its death, and a length-prefixed \
         stream cannot be resynchronised -- so it says the same thing again"
    );
    peer.join().expect("the peer ends");
}

#[test]
fn a_peer_that_goes_away_is_disconnected_and_leaves_the_table() {
    let channel = channel_for_line(line!());
    let mut ipc = LocalSocketIpc::bound(&channel, &this_account(), Progressive::starting_at(0), CAP)
        .expect("binds");
    let peer = a_peer_that_writes(channel, Vec::new(), 200);
    let client = accept_one(&mut ipc);
    // ⛔ THE BASELINE FIRST: alive and silent is `Ok(None)`, NOT gone. Without this line the
    // assertion below would be green on a transport that cannot tell the two apart -- which is
    // exactly what a nonblocking read did on Windows (D78).
    assert_eq!(ipc.receive(client), Ok(None), "a live peer that has written nothing is not gone");
    peer.join().expect("the peer ends");

    let seen = the_next_answer(&mut ipc, client);
    assert_eq!(seen, Err(IpcError::Disconnected));
    assert_eq!(
        ipc.receive(client),
        Err(IpcError::Disconnected),
        "and it SAYS SO AGAIN, which is the whole of what this oracle can see: `receive` \
         answers `Disconnected` both for a client that has left the table and for one still in \
         it whose channel is closed. That the table SHRANK is checked by \
         `ipc::tests::a_peer_reported_gone_by_receive_is_out_of_the_table`, in \
         `crates/platform/src/ipc.rs`, where `clients` is reachable"
    );
}

#[test]
fn a_send_to_a_peer_that_left_is_disconnected_and_drops_the_client() {
    // ⛔ THE SECOND DIRECTION OF THE `send` ROW OF §3: a peer seen leaving THROUGH A WRITE, not
    // through a read. The write is blocking (D78), so a write after the peer's death fails instead
    // of pretending; the loop is bounded because the death can take a moment to reach the pipe.
    let channel = channel_for_line(line!());
    let mut ipc = LocalSocketIpc::bound(&channel, &this_account(), Progressive::starting_at(0), CAP)
        .expect("binds");
    // ⛔ IT HOLDS, AND THE HOLD IS NOT DECORATION (E5): with `0` the peer could connect and die
    // before the first `accept`, the listener cleared the empty connection, and `accept_one`
    // waited for a client that was never coming -- measured 1 hang in 60. The length carries no
    // meaning for the assertion: `peer.join()` below is what makes the peer dead before `send`.
    let peer = a_peer_that_writes(channel, Vec::new(), 200);
    let client = accept_one(&mut ipc);
    peer.join().expect("the peer ends");

    let frame = framing::frame(b"anyone there").expect("frame");
    let started = std::time::Instant::now();
    let seen = loop {
        match ipc.send(client, &frame) {
            Ok(()) if started.elapsed() < std::time::Duration::from_secs(2) => std::thread::yield_now(),
            other => break other,
        }
    };
    assert_eq!(seen, Err(IpcError::Disconnected), "a write to a peer that left is `Disconnected`");
    assert_eq!(
        ipc.receive(client),
        Err(IpcError::Disconnected),
        "and it is `Disconnected` FROM HERE ON, which is as far as this oracle reaches: the \
         same answer comes back whether or not the write removed the row. That it WAS removed \
         is checked by `ipc::tests::a_peer_a_send_finds_gone_is_out_of_the_table`, in \
         `crates/platform/src/ipc.rs`"
    );
}

/// ⛔ ADR-0041, THE DIRECTION IN WHICH THE ACCOUNT GETS IN: a channel bound for the account this
/// process runs as takes this process's connections, THREE IN A ROW -- the measurement the ADR
/// records, and on Windows the proof that the descriptor still lets the server create the
/// instances after the first. The other direction -- another account refused -- needs an account
/// this process is not, and lives in the unit module of `crates/platform/src/ipc.rs`, beside the
/// private constructor of one.
#[test]
fn this_account_gets_in_three_times_in_a_row() {
    let channel = channel_for_line(line!());
    let mut ipc = LocalSocketIpc::bound(&channel, &this_account(), Progressive::starting_at(0), CAP)
        .expect("binds");

    let mut accepted = Vec::new();
    for _ in 0..3 {
        let peer = a_peer_that_writes(channel.clone(), Vec::new(), 100);
        accepted.push(accept_one(&mut ipc));
        peer.join().expect("the peer ends");
    }
    assert_eq!(
        accepted,
        [ClientId::new(0), ClientId::new(1), ClientId::new(2)],
        "three connections of this account, three clients"
    );
}

/// ⛔ ADR-0041, POINT 5: a runtime directory its group or others can enter stops the start, and
/// nothing is bound in it. The group bit and the others bit are tried apart, because a check on
/// one of them alone would pass a probe that tried only the other.
#[cfg(unix)]
#[test]
fn a_directory_open_to_other_accounts_does_not_bind() {
    use std::os::unix::fs::PermissionsExt;
    for mode in [0o750, 0o705] {
        let channel = channel_for_line(line!());
        let directory = channel.parent().expect("the channel has a directory").to_path_buf();
        std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(mode))
            .expect("the directory is opened");

        let refused =
            LocalSocketIpc::bound(&channel, &this_account(), Progressive::starting_at(0), CAP);
        assert_eq!(
            refused.err().map(|error| error.kind()),
            Some(std::io::ErrorKind::PermissionDenied),
            "a directory of mode {mode:o} is open to another account"
        );
        assert!(!channel.exists(), "and nothing was bound in a directory of mode {mode:o}");
    }
}

/// ⛔ ADR-0041, POINT 6, THE FIRST HALF: a core that fell without unwinding leaves its socket file
/// on the disk, and that file must not stop the next start. A `std` listener bound and dropped is
/// that file: unlike the transport, `std` does not remove its socket when it falls.
#[cfg(unix)]
#[test]
fn a_socket_left_by_a_core_that_fell_does_not_stop_the_start() {
    let channel = channel_for_line(line!());
    drop(std::os::unix::net::UnixListener::bind(&channel).expect("the core that fell had bound"));
    assert!(channel.exists(), "the socket outlives the listener: what a fallen core leaves behind");

    let mut ipc = LocalSocketIpc::bound(&channel, &this_account(), Progressive::starting_at(0), CAP)
        .expect("nobody answers on the old socket, so it is cleared and the start binds");
    let peer = a_peer_that_writes(channel, Vec::new(), 300);
    accept_one(&mut ipc);
    peer.join().expect("the peer ends");
}

/// ⛔ ADR-0041, POINT 6, THE SECOND HALF: a socket a LIVE core answers on stops the start -- and
/// leaves that core where it was, which is the half a start that removed the file unasked would
/// break while still failing "correctly" at its own bind.
#[cfg(unix)]
#[test]
fn a_socket_a_live_core_answers_on_stops_the_start_and_leaves_that_core_listening() {
    let channel = channel_for_line(line!());
    let mut first =
        LocalSocketIpc::bound(&channel, &this_account(), Progressive::starting_at(0), CAP)
            .expect("the first core binds");

    let second =
        LocalSocketIpc::bound(&channel, &this_account(), Progressive::starting_at(0), CAP);
    assert_eq!(
        second.err().map(|error| error.kind()),
        Some(std::io::ErrorKind::AddrInUse),
        "a live core answers, so the second start stops"
    );

    // ⚠️ THE FIRST ACCEPT MAY BE THE SECOND START'S OWN QUESTION, which is a connection too;
    // either way, somebody reaching the first core is the proof that its socket is still there.
    let peer = a_peer_that_writes(channel, Vec::new(), 300);
    accept_one(&mut first);
    peer.join().expect("the peer ends");
}
