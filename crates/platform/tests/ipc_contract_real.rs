//! The `ipc` conformance suite expanded against the REAL transport, plus the promises only a
//! real peer can exercise.
//!
//! ⚠️ THE SOCKET NAME IS PER-TEST AND CARRIES THE LINE NUMBER, except in `build`, where a
//! counter numbers the calls because all three suite tests share that one call site, the shape
//! `crates/daemon/src/main.rs` already uses for its private directories: two tests sharing a
//! name pass alone and fail together, which is the flakiest red there is.
//!
//! ⚠️ THE WAITS ARE `yield_now` LOOPS AND NOT FIXED SLEEPS, except where the assertion is about
//! something that has NOT arrived -- there an interval must elapse, and those two are named.

use std::io::Write;

use kernel::framing;
use kernel::numbering::Progressive;
use kernel::ports::ipc::{ClientId, Ipc, IpcError};
use platform::ipc::LocalSocketIpc;

/// The cap every test in this file hands over. Big enough for any body written here, small
/// enough that `a_body_over_the_cap_is_malformed` can exceed it without allocating.
const CAP: usize = 4_096;

fn socket_name_for_line(line: u32) -> String {
    format!("harness-ipc-contract-{}-{}", std::process::id(), line)
}

/// ⛔ A FRESH NUMBER PER CALL AND NOT `line!()`, WHICH WOULD EXPAND ONCE. This helper is the
/// one place where `line!()` is not at the call site: all three suite tests call it, and
/// libtest runs them on parallel threads, so they would all bind THE SAME name -- the trap
/// the module doc above names. The shape is `journal_contract_real.rs`'s `AtomicU64`.
///
/// ⚠️ AND THE PREFIX IS NOT `socket_name_for_line`'s, which is the half a counter alone does
/// not cover: the third call would be number 2, and a test on line 2 would collide with it.
fn build() -> LocalSocketIpc {
    static NEXT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let nth = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    LocalSocketIpc::bound(
        &format!("harness-ipc-suite-{}-{}", std::process::id(), nth),
        Progressive::starting_at(0),
        CAP,
    )
    .expect("the listener binds")
}

include!("../../kernel/tests/contract/ipc.rs");

ipc_contract_suite!(build);

/// Connects, writes `bytes` verbatim, and stays open for `hold`.
fn a_peer_that_writes(name: String, bytes: Vec<u8>, hold_ms: u64) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        use interprocess::local_socket::{prelude::*, GenericNamespaced, Stream};
        let ns = name.to_ns_name::<GenericNamespaced>().expect("name");
        let mut stream = loop {
            match Stream::connect(ns.clone()) {
                Ok(stream) => break stream,
                Err(_) => std::thread::yield_now(),
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
/// all, so it cannot be read, bisected or reported -- it only eats the gate. Five seconds is
/// generous against a loaded machine and far below a hang.
fn accept_one(ipc: &mut LocalSocketIpc) -> ClientId {
    let started = std::time::Instant::now();
    loop {
        if let Some(client) = ipc.accept() {
            return client;
        }
        if started.elapsed() > std::time::Duration::from_secs(5) {
            panic!("nobody was accepted in five seconds: the peer died before `accept` (E5)");
        }
        std::thread::yield_now();
    }
}

#[test]
fn a_whole_frame_comes_back_whole_envelope_included() {
    let name = socket_name_for_line(line!());
    let mut ipc = LocalSocketIpc::bound(&name, Progressive::starting_at(0), CAP).expect("binds");
    let peer = a_peer_that_writes(name, framing::frame(b"hello core").expect("frame"), 300);

    let client = accept_one(&mut ipc);
    let seen = loop {
        match ipc.receive(client).expect("a live client") {
            Some(bytes) => break bytes,
            None => std::thread::yield_now(),
        }
    };
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
    let name = socket_name_for_line(line!());
    let mut ipc = LocalSocketIpc::bound(&name, Progressive::starting_at(0), CAP).expect("binds");
    let mut bytes = framing::frame(b"first").expect("frame");
    bytes.extend_from_slice(&framing::frame(b"second").expect("frame"));
    let peer = a_peer_that_writes(name, bytes, 300);

    let client = accept_one(&mut ipc);
    let mut seen: Vec<Vec<u8>> = Vec::new();
    while seen.len() < 2 {
        match ipc.receive(client).expect("a live client") {
            Some(bytes) => seen.push(bytes),
            None => std::thread::yield_now(),
        }
    }
    assert_eq!(seen[0], framing::frame(b"first").expect("frame"));
    assert_eq!(seen[1], framing::frame(b"second").expect("frame"));
    peer.join().expect("the peer ends");
}

#[test]
fn two_accepts_hand_out_different_and_ascending_numbers() {
    // ⛔ THE PROMISE IS THE COUNTER'S, checked HERE because `accept` is the only site that mints
    // a `ClientId`: a transport that started a private `u64` would pass every other test here.
    let name = socket_name_for_line(line!());
    let mut ipc = LocalSocketIpc::bound(&name, Progressive::starting_at(41), CAP).expect("binds");
    let first_peer = a_peer_that_writes(name.clone(), Vec::new(), 300);
    let first = accept_one(&mut ipc);
    let second_peer = a_peer_that_writes(name, Vec::new(), 300);
    let second = accept_one(&mut ipc);

    assert_eq!(first, ClientId::new(41), "the counter was started at 41");
    assert_eq!(second, ClientId::new(42));
    first_peer.join().expect("peer one ends");
    second_peer.join().expect("peer two ends");
}

#[test]
fn half_a_frame_is_not_a_body_yet_and_not_an_error() {
    let name = socket_name_for_line(line!());
    let mut ipc = LocalSocketIpc::bound(&name, Progressive::starting_at(0), CAP).expect("binds");
    let whole = framing::frame(b"a body long enough to cut").expect("frame");
    let half = whole[..whole.len() - 3].to_vec();
    let peer = a_peer_that_writes(name, half, 300);

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
    let name = socket_name_for_line(line!());
    let mut ipc = LocalSocketIpc::bound(&name, Progressive::starting_at(0), CAP).expect("binds");
    let over = ((CAP + 1) as u32).to_be_bytes().to_vec();
    let peer = a_peer_that_writes(name, over, 300);

    let client = accept_one(&mut ipc);
    let seen = loop {
        match ipc.receive(client) {
            Ok(None) => std::thread::yield_now(),
            other => break other,
        }
    };
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
    let name = socket_name_for_line(line!());
    let mut ipc = LocalSocketIpc::bound(&name, Progressive::starting_at(0), CAP).expect("binds");
    let peer = a_peer_that_writes(name, Vec::new(), 200);
    let client = accept_one(&mut ipc);
    // ⛔ THE BASELINE FIRST: alive and silent is `Ok(None)`, NOT gone. Without this line the
    // assertion below would be green on a transport that cannot tell the two apart -- which is
    // exactly what a nonblocking read did on Windows (D78).
    assert_eq!(ipc.receive(client), Ok(None), "a live peer that has written nothing is not gone");
    peer.join().expect("the peer ends");

    let seen = loop {
        match ipc.receive(client) {
            Ok(None) => std::thread::yield_now(),
            other => break other,
        }
    };
    assert_eq!(seen, Err(IpcError::Disconnected));
    assert_eq!(
        ipc.receive(client),
        Err(IpcError::Disconnected),
        "and it STAYS gone: the client left the table, it was not merely reported once"
    );
}

#[test]
fn a_send_to_a_peer_that_left_is_disconnected_and_drops_the_client() {
    // ⛔ THE SECOND DIRECTION OF THE `send` ROW OF §3: a peer seen leaving THROUGH A WRITE, not
    // through a read. The write is blocking (D78), so a write after the peer's death fails instead
    // of pretending; the loop is bounded because the death can take a moment to reach the pipe.
    let name = socket_name_for_line(line!());
    let mut ipc = LocalSocketIpc::bound(&name, Progressive::starting_at(0), CAP).expect("binds");
    // ⛔ IT HOLDS, AND THE HOLD IS NOT DECORATION (E5): with `0` the peer could connect and die
    // before the first `accept`, the listener cleared the empty connection, and `accept_one`
    // waited for a client that was never coming -- measured 1 hang in 60. The length carries no
    // meaning for the assertion: `peer.join()` below is what makes the peer dead before `send`.
    let peer = a_peer_that_writes(name, Vec::new(), 200);
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
        "and the client LEFT THE TABLE on that write"
    );
}
