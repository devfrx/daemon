// THE CONFORMANCE SUITE OF THE `ipc` PORT (§3 of the sub-project 2 design). ⛔ IT IS EXPANDED AGAINST
// THE REAL TRANSPORT ONLY (D82): every promise worth holding here needs a peer that writes bytes,
// and the two fakes of this port pass bytes verbatim and frame nothing -- expanded on them the
// suite would be comparing itself. No liar either: one expansion has nothing to disagree with.
//
// ⛔ REGULAR COMMENTS AND NOT `//!`, BECAUSE THIS FILE IS `include!`d.
// `crates/platform/tests/ipc_contract_real.rs` expands it IN ITEM POSITION, and an inner
// attribute -- which is what `//!` desugars to -- is not permitted there. Same mechanism as
// `journal_contract.rs`, and for the same reason: two copies of the assertions would diverge,
// and the first one that diverged would print `ok` while comparing nothing.
//
// ⛔ WHAT IS DELIBERATELY ABSENT: every promise that needs a real peer writing bytes. A whole
// frame, half a frame, a body over the cap, an ended stream -- none of them can be staged
// against a fake that has no stream, and asserting them here would be a suite comparing itself.
// They live in `crates/platform/tests/ipc_contract_real.rs` beside the `include!`, which is
// where `journal_contract_real.rs` puts its own.
//
// ⛔ THIS FILE IS NOT A TEST TARGET. It lives under `tests/contract/`, which cargo does not
// auto-discover, so `kernel` neither compiles it alone nor warns about an unused macro (D82) --
// `journal_contract.rs` next door IS a target only because it also holds a test of its own.

/// What every implementation must answer. The macro takes a constructor so each crate hands
/// over its own.
macro_rules! ipc_contract_suite {
    ($build:expr) => {
        #[test]
        fn with_nobody_connected_accept_answers_none() {
            let mut ipc = $build();
            assert_eq!(
                ipc.accept(),
                None,
                "an empty accept is the NORMAL state of a core nobody has opened a gui against"
            );
        }

        #[test]
        fn sending_to_an_unknown_client_is_disconnected() {
            let mut ipc = $build();
            assert_eq!(
                ipc.send(ClientId::new(9_999), b"anything"),
                Err(IpcError::Disconnected),
                "a client that is not in the table is gone, which is the same thing"
            );
        }

        #[test]
        fn receiving_from_an_unknown_client_is_disconnected() {
            let mut ipc = $build();
            assert_eq!(ipc.receive(ClientId::new(9_999)), Err(IpcError::Disconnected));
        }
    };
}
