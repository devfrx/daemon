//! ADR-0033: "if the GUI dies holding an ordinary grant, the core notices from the IPC
//! DISCONNECTION and reconciles". This is that reconciliation, seen from outside the crate.
//!
//! ⛔ THE BASELINE IS NEVER ZERO IN THIS FILE, and it is the assertion and not the setup. "The
//! sum comes back to baseline" is green for a reconciliation that releases EVERYTHING when the
//! baseline is zero, so every probe here builds a grant SOMEBODY ELSE holds first — the core's
//! own presentation quota of ADR-0033 is the model — and asserts the books still carry it
//! afterwards.
//!
//! ⚠️ WHAT THIS FILE DOES NOT HOLD, said rather than assumed: it never touches the `ipc` port.
//! `ClientGrants` answers a disconnection; SEEING one is the business of whoever polls `send`
//! and `receive`, and that half is held by the campaign in
//! `crates/simulator/tests/gui_death_campaign.rs`, where the death is read out of
//! `Err(IpcError::Disconnected)` and out of nothing else.

use kernel::arbiter::{
    Admission, Arbiter, ArbiterId, ComputeClass, Grant, Mib, Preemption, ReleaseError, Released,
    RemotePolicy, ResourceProfile, VramPolicy,
};
use kernel::client::ClientGrants;
use kernel::parameters::Parameters;
use kernel::ports::ipc::ClientId;
use kernel::time::{Millis, Monotonic};

/// ⚠️ BOTH TAKEN FROM `crates/kernel/tests/arbiter_admission.rs`, and declared rather than left
/// to be noticed: TEST CODE DOES NOT CROSS CRATE BOUNDARIES, let alone bench boundaries, so a
/// figure shared with another bench is COPIED and the copy is named. It is the deviation `Yield`
/// carries in `crates/simulator/tests/arbiter_campaign.rs`, for the same reason. Nothing here
/// depends on their staying equal: they are a machine big enough that every profile below fits.
const TURN_LIMIT: u64 = 10_000;
const TOTAL: Mib = Mib::new(16_384);

/// What the core keeps for itself, and it is the BASELINE. ⛔ It is held by nobody this file
/// ever disconnects, so every assertion below can say "this much was still there afterwards".
const CORE_QUOTA: Mib = Mib::new(2_048);

/// What a gui asks for.
const GUI_QUOTA: Mib = Mib::new(1_024);

/// The window a client declares. ⛔ IT IS SHORT ON PURPOSE and the core's is not: the fourth
/// probe disconnects at `WINDOW + 1`, and with one window for both the core's own grant would
/// have expired too and the baseline would have gone with it.
const WINDOW: Millis = Millis::new(5_000);

/// The window the core's own quota declares — long enough that nothing here ever collects it.
/// ⚠️ It is `LONG` of `arbiter_admission.rs` under another name, copied for the reason given on
/// `TURN_LIMIT` above.
const FOREVER: Millis = Millis::new(1_000_000);

fn new_arbiter() -> Arbiter {
    Arbiter::new(
        Parameters::new(TURN_LIMIT, TOTAL, ArbiterId::new(1)),
        VramPolicy::Remote(RemotePolicy),
    )
}

fn profile(name: &'static str, reserved: Mib) -> ResourceProfile {
    ResourceProfile {
        name,
        reserved_vram: reserved,
        compute_class: ComputeClass::Interactive,
        preemption: Preemption::Never,
    }
}

/// A grant, obtained the only way there is. ⚠️ `Admission` has no `Debug`, so the `let … else`
/// is not a style — `expect` does not exist on it.
fn granted(
    arbiter: &mut Arbiter,
    name: &'static str,
    reserved: Mib,
    valid_for: Millis,
    now: Monotonic,
) -> Grant {
    let Admission::Granted(grant) = arbiter.admit(&profile(name, reserved), valid_for, now) else {
        panic!("{name} asked for {reserved:?} of {TOTAL:?}, which fits");
    };
    grant
}

/// The core's own quota, in the books before any gui says anything.
fn an_arbiter_with_a_baseline() -> (Arbiter, Grant) {
    let mut arbiter = new_arbiter();
    let core = granted(
        &mut arbiter,
        "core-presentation",
        CORE_QUOTA,
        FOREVER,
        Monotonic::ORIGIN,
    );
    assert_eq!(
        arbiter.allocated(),
        CORE_QUOTA,
        "the baseline is in the books"
    );
    (arbiter, core)
}

#[test]
fn a_disconnected_client_gives_its_grant_back() {
    // ⛔ THE ASSERTION IS ON `allocated()`, not on a bookkeeping flag: the property of §5.7 is
    // about THE SUM, and a flag could be right while the sum was wrong.
    let (mut arbiter, _core) = an_arbiter_with_a_baseline();
    let gui = ClientId::new(1);

    let grant = granted(&mut arbiter, "gui", GUI_QUOTA, WINDOW, Monotonic::ORIGIN);
    let mut clients = ClientGrants::new();
    clients.register(gui, grant);
    assert_eq!(
        arbiter.allocated(),
        CORE_QUOTA.saturating_add(GUI_QUOTA),
        "the gui's reservation is in the books while it holds it"
    );

    let released = clients.on_disconnect(gui, &mut arbiter, Monotonic::ORIGIN);

    // ⛔ THE RESERVATION CAME BACK NOW, and `Now` is what says so: `AlreadyCollected` would mean
    // the sweep got there first, which at `ORIGIN` inside a window of `WINDOW` it did not.
    assert_eq!(released, Ok(vec![Released::Now(GUI_QUOTA)]));
    assert_eq!(
        arbiter.allocated(),
        CORE_QUOTA,
        "the sum did not return to the baseline"
    );
}

#[test]
fn a_disconnect_gives_back_only_that_client_s_grants() {
    // ⛔ THE HALF THAT GETS FORGOTTEN (§7.1.1 rule 3): a reconciliation that released EVERYTHING
    // would pass the probe above. Two clients, one dies, the other keeps its reservation.
    let (mut arbiter, _core) = an_arbiter_with_a_baseline();
    let doomed = ClientId::new(1);
    let survivor = ClientId::new(2);

    let mut clients = ClientGrants::new();
    // ⛔ THE DOOMED CLIENT HOLDS TWO, and that is not padding: with one apiece, "released the
    // right one" and "released one of them" are the same green. It also makes the loop's
    // second turn load-bearing.
    clients.register(
        doomed,
        granted(&mut arbiter, "gui-a", GUI_QUOTA, WINDOW, Monotonic::ORIGIN),
    );
    clients.register(
        survivor,
        granted(&mut arbiter, "gui-b", GUI_QUOTA, WINDOW, Monotonic::ORIGIN),
    );
    clients.register(
        doomed,
        granted(&mut arbiter, "gui-c", GUI_QUOTA, WINDOW, Monotonic::ORIGIN),
    );

    let released = clients.on_disconnect(doomed, &mut arbiter, Monotonic::ORIGIN);

    assert_eq!(
        released,
        Ok(vec![Released::Now(GUI_QUOTA), Released::Now(GUI_QUOTA)]),
        "the disconnection owed back exactly the two grants that client held"
    );
    assert_eq!(
        arbiter.allocated(),
        CORE_QUOTA.saturating_add(GUI_QUOTA),
        "the survivor's reservation left the books with the dead client's"
    );

    // ⛔ AND IT IS STILL REGISTERED, which the sum alone does not say -- MEASURED on 2026-09-02
    // rather than argued. The shape this file's own doc warns against, `drain(..)` with the
    // non-matching pairs never put back, releases exactly the right grants and leaves the books
    // exactly right: it passes every assertion above and dies HERE, because the survivor's pair
    // was thrown away with the dead client's and its reservation can never come home. It is
    // therefore a different axis from "released everything", which dies on the first assertion
    // of this probe (gotcha #55).
    assert_eq!(
        clients.on_disconnect(survivor, &mut arbiter, Monotonic::ORIGIN),
        Ok(vec![Released::Now(GUI_QUOTA)])
    );
    assert_eq!(arbiter.allocated(), CORE_QUOTA);
}

#[test]
fn a_disconnect_of_a_client_that_holds_nothing_changes_nothing() {
    // ⛔ AND IT MUST NOT BE AN ERROR: a client may die before it ever asked, and treating that as
    // a fault would make an ordinary event look like a defect -- the shape of `ReleaseError`,
    // whose `UnknownGrant` §5.6 spent a whole open voice on.
    let (mut arbiter, _core) = an_arbiter_with_a_baseline();
    let mut clients = ClientGrants::new();

    let released = clients.on_disconnect(ClientId::new(7), &mut arbiter, Monotonic::ORIGIN);

    // ⚠️ ONE ASSERTION AND NOT TWO. An `assert_ne!` against `Err(UnknownGrant)` stood here and
    // was REMOVED rather than reworded: the equality above already fails on any `Err`, so it
    // could never have been the one to go red, and a probe that cannot fail is a line a reader
    // has to check. What it was there to SAY is in this message instead.
    assert_eq!(
        released,
        Ok(Vec::new()),
        "a client that never asked for anything is not a caller defect: an ordinary death was \
         reported as a fault"
    );
    assert_eq!(
        arbiter.allocated(),
        CORE_QUOTA,
        "a disconnection that owed nothing moved the books"
    );
}

#[test]
fn a_disconnect_after_the_window_reports_already_collected() {
    // ⛔ THE DIRECTION THE THREE PROBES ABOVE DO NOT REACH, and it is `E30` arriving on the gui:
    // a client that dies AFTER its own window has one already-swept reservation to hand back,
    // and the campaign produces exactly that on a share of its seeds. Without this probe an
    // `on_disconnect` that discarded or refused `AlreadyCollected` would compile green.
    let (mut arbiter, _core) = an_arbiter_with_a_baseline();
    let gui = ClientId::new(1);

    let mut clients = ClientGrants::new();
    clients.register(
        gui,
        granted(&mut arbiter, "gui", GUI_QUOTA, WINDOW, Monotonic::ORIGIN),
    );

    let too_late = Monotonic::ORIGIN.saturating_add(Millis::new(WINDOW.get() + 1));
    let released = clients.on_disconnect(gui, &mut arbiter, too_late);

    // ⛔ NOT AN `Err`. The window closing is the arbiter's own sweep, not a grant it never
    // issued, and since 2026-08-30 the two are told apart inside `Ok`.
    assert_eq!(released, Ok(vec![Released::AlreadyCollected]));
    assert_eq!(
        arbiter.allocated(),
        CORE_QUOTA,
        "the baseline went with the expired grant"
    );
}

#[test]
fn a_foreign_grant_is_the_caller_s_defect_and_leaves_the_rest_registered() {
    // ⛔ THE ONLY PROBE THAT REACHES THE `Err` ARM, and until it existed the whole argument for
    // the shape of `on_disconnect` rested on a paragraph. The four probes above all take the
    // happy path, so `drain(..)` WITH repartition -- the form the plan prescribed -- was green on
    // every one of them and diverged from the shipped form on exactly the road none of them
    // walked. Measured: it dies here and nowhere else.
    //
    // ⛔ AND THE `Err` IS THE CALLER'S DEFECT AND NOT THE CLOCK'S. `Arbiter::release` answers
    // `Err(UnknownGrant)` for one cause only since 2026-08-30 -- a grant ANOTHER arbiter issued.
    // The window closing is `Ok(AlreadyCollected)`, which the probe above holds.
    let (mut arbiter, _core) = an_arbiter_with_a_baseline();

    // ⛔ THE SECOND ARBITER'S IDENTITY IS LOAD-BEARING: `release` compares `Grant::issuer`, so two
    // arbiters handed the SAME `ArbiterId` ARE one as far as this guard can tell, and the probe
    // would go green for the wrong reason. It is the argument `arbiter_admission.rs` makes about
    // its own two helpers.
    let mut elsewhere = Arbiter::new(
        Parameters::new(TURN_LIMIT, TOTAL, ArbiterId::new(2)),
        VramPolicy::Remote(RemotePolicy),
    );
    let foreign = granted(
        &mut elsewhere,
        "gui-elsewhere",
        GUI_QUOTA,
        WINDOW,
        Monotonic::ORIGIN,
    );

    let gui = ClientId::new(1);
    let mut clients = ClientGrants::new();
    // ⛔ THE ORDER IS THE PROBE: native, foreign, native. The `?` returns on the SECOND pair, so
    // the THIRD is the one that has to survive the failure -- and with a `drain` that took every
    // pair out before releasing any, it would not.
    clients.register(
        gui,
        granted(&mut arbiter, "gui-a", GUI_QUOTA, WINDOW, Monotonic::ORIGIN),
    );
    clients.register(gui, foreign);
    clients.register(
        gui,
        granted(&mut arbiter, "gui-b", GUI_QUOTA, WINDOW, Monotonic::ORIGIN),
    );

    let both = CORE_QUOTA
        .saturating_add(GUI_QUOTA)
        .saturating_add(GUI_QUOTA);
    assert_eq!(
        arbiter.allocated(),
        both,
        "the two native grants are in the books; the foreign one is in another arbiter's"
    );

    assert_eq!(
        clients.on_disconnect(gui, &mut arbiter, Monotonic::ORIGIN),
        Err(ReleaseError::UnknownGrant),
        "a grant this arbiter never issued is the caller's defect, and the one thing that is"
    );

    // ⛔ WHAT THE `Err` COST, STATED EXACTLY: the first native grant went back BEFORE the failure
    // -- `release` had already answered for it -- and the foreign one is gone, consumed by the
    // call that refused it. Neither is recoverable, and the doc of `on_disconnect` says so.
    assert_eq!(
        arbiter.allocated(),
        CORE_QUOTA.saturating_add(GUI_QUOTA),
        "the grant released before the failure did not come back, or the failing call took more \
         than the grant it was given"
    );

    // ⛔ AND THIS IS THE ASSERTION THE WHOLE PROBE EXISTS FOR: the third pair is STILL REGISTERED.
    // A caller that comes back with the right arbiter loses none of what it still held. Under
    // `drain` this answers `Ok([])` and the reservation is unreachable for ever.
    assert_eq!(
        clients.on_disconnect(gui, &mut arbiter, Monotonic::ORIGIN),
        Ok(vec![Released::Now(GUI_QUOTA)]),
        "the failure threw away a grant it had not been asked to release"
    );
    assert_eq!(arbiter.allocated(), CORE_QUOTA);
}
