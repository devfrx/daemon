//! The `ipc` port: the server towards the gui (§6.1).
//!
//! ⛔ THE GUI IS SACRIFICIAL. It owns no authoritative state (I1), so killing it loses
//! nothing, and the port must survive a client that dies at any instant -- there is no
//! liveness protocol against a process designed to die. When the gui dies holding an
//! ordinary grant, the core notices FROM THE IPC DISCONNECTION and reconciles (ADR-0033).
//!
//! # The core decides WHEN to emit, and the gui does not pull (§6.1.4)
//!
//! ⛔ It is already so BY CONSTRUCTION, and §6.1.4 asks in those words for it to be WRITTEN
//! ANYWAY, SO THAT IT DOES NOT GET ERODED. The shape of the trait is the argument: `send` is
//! called BY THE CORE, when the core decides; `receive` hands over what the client has already
//! SAID, which is a request arriving and not a read the client performed; and THERE IS NO
//! THIRD OPERATION. Nothing here lets a client name a piece of state and ask for its current
//! value, so "the gui refreshes itself" is not expressible -- a gui that wants something asks,
//! and then waits to be told.
//!
//! ⛔ WHY IT IS LOAD-BEARING RATHER THAN TIDY (ADR-0027). If rendering ever costs too much,
//! the lever is THE UPDATE FREQUENCY DECIDED BY THE CORE: aggregating, sampling or coalescing
//! updates is a KERNEL choice, and it exists only because the core is the party that calls
//! `send`. Give the gui a pull and that lever leaves the kernel -- quietly, with nothing going
//! red, because a pull is an addition and additions do not break anything.
//!
//! ⚠️ Milestone 2 declares the port. Milestone 6 brings the SCHEMA -- `bincode` in
//! `kernel`, chosen because the peer is TypeScript and M-11 measured that it can read it
//! (ADR-0037) -- and the BUILD STAMP of §6.1.2, which is how a stale gui is refused
//! without versioning. I4 renounces versioning; the stamp is the mechanism that replaces
//! it: ONE accepted value, and a gui that carries a different one does not start and says so.
//! It is not a contract, it is an identity.
//!
//! ⛔ DATED RECALL, 2026-08-31 -- THE FORMAT NAMED ABOVE WAS REOPENED AND IS SETTLED AGAIN.
//! ⚠️ THIS HEADING READ "IS NO LONGER SETTLED", and it is corrected rather than deleted: what
//! follows is a MINUTE and stands as written, but a heading is what gets read. Gotcha #31, the
//! same correction this crate's manifest received the same day.
//!
//! Milestone 6 redid the C-1 measurement from primary sources: RUSTSEC-2025-0141 is still
//! active, `bincode` upstream is archived, and MAINTAINED alternatives exist -- one of them
//! claiming the same wire format, which would leave the TypeScript peer reader exactly where
//! M-11 left it. ⚖️ The MEASUREMENT was done and the CHOICE was not, because §6.1.1 is spec
//! and reopening it belongs to the owner.
//!
//! ✅ AND THE OWNER RULED THE SAME DAY: `bincode` 2.0.1 STAYS, §6.1.1 IS NOT REOPENED -- with
//! measure M-12 in hand rather than around it. The reasons live in ONE house, the `bincode`
//! note in this crate's Cargo.toml; the entry is closed in docs/porta-di-qualita.md.
//! ⚠️ THIS FILE WAS THE HOUSE THAT RULING MISSED: the deciding commit touched five others and
//! not this one, so the paragraph above said "the choice is not [made]" for a day after it was.
//!
//! ⛔ DATED RECALL, 2026-08-31 -- THE OTHER HALF OF THAT SENTENCE, AND IT IS THE HALF NOTHING
//! HAS CORRECTED: THE BUILD STAMP DOES NOT ARRIVE WITH MILESTONE 6. §3.4 takes it out of the
//! perimeter and makes it a DECLARED NON-CONSTRUCTION, so "milestone 6 brings ... the BUILD
//! STAMP of §6.1.2" above is A DEADLINE WRITTEN IN PROSE (gotcha #77), and this is the run in
//! which it falls due. Nothing ever went red for it, and nothing could have: a sentence about
//! a future milestone has no oracle.
//! ⚠️ WHAT IS CORRECTED IS THE *WHEN* AND NOT THE *WHAT*, and the paragraph above stays for
//! the *WHAT*.
//! ✅ THE SCHEMA HALF DID ARRIVE, and in the format named: `crate::wire::ipc` carries
//! `IpcMessage` over the envelope of `crate::framing`. ✅ AND THE STAMP ARRIVED WITH
//! SUB-PROJECT 2, beside the schema: `crate::wire::ipc::BuildStamp` says what it IS,
//! `crate::wire::ipc::build_stamp` computes it, and the handshake of `crate::serving` refuses a
//! gui that carries another one with `IpcMessage::StaleBuild`. ⚠️ RECALL OF 2026-10-03 -- audit
//! of 2026-09-30, AUD-061.
//!
//! ⛔ The port exchanges BYTES, like `journal` and `process`. The schema lives in
//! `kernel` and the simulator therefore exchanges bytes too, so the DST campaign really
//! exercises encoding and decoding instead of going around them.
//!
//! ⛔ DECLARED OPEN QUESTION, AND IT IS NOT RESOLVED HERE -- named at the top of the file, as
//! `network` names its own. `accept` HAS NO ERROR CHANNEL while `receive` has one, so a listener
//! that has itself broken -- as opposed to a client that has -- surfaces as `None`, a wrong value
//! rather than an error; and since the transport exists, `platform::ipc::LocalSocketIpc`, that is
//! what every error of its listener does. ⚠️ The consequence that matters is the PRICE: closing
//! it costs the SIGNATURE, not a third variant of `IpcError`. The full argument, and why the
//! signature still stands today, sits on `Ipc::accept`. ⚠️ RECALL OF 2026-10-02 -- audit of
//! 2026-09-30, AUD-078.
//!
//! ⚠️ AND WHAT HOLDS THESE THREE SIGNATURES IS EVERY IMPLEMENTATION FROM OUTSIDE THE CRATE, and
//! WHICH THEY ARE IS WHAT THE COMMAND PRINTS rather than this line:
//! `grep -rnE "^ *impl Ipc for" crates/`. ⛔ DATED RECALL, 2026-09-20: here stood "IS TWO
//! IMPLEMENTATIONS", naming `tests/ports_are_implementable.rs` and `simulator::ipc::DyingGui`.
//! It was true when milestone 6 wrote it and false from task 2 of sub-project 2, which brought
//! the TRANSPORT -- `platform::ipc::LocalSocketIpc` -- and from tasks 7 and 10, which added two
//! more fakes. The figure is REMOVED rather than realigned, because the dated-recall chain on
//! `ClientId` in this file predicted the recurrence in writing: "naming a third would only
//! schedule the same correction again" (E89 of the sub-project 2 plan).
//! They buy that the signatures are IMPLEMENTABLE
//! FROM OUTSIDE THE CRATE and callable; they do NOT buy that they are the right signatures, and
//! neither does the conformance suite: `crates/kernel/tests/contract/ipc.rs`, expanded by
//! `crates/platform/tests/ipc_contract_real.rs` against the real transport ONLY (D82), holds that
//! transport to ONE contract and compares no fake with it. ⚠️ RECALL OF 2026-10-03 -- audit of
//! 2026-09-30, AUD-408, AUD-081.
//!
//! ⛔ DATED RECALL, 2026-09-02, MILESTONE 6 TASK 9 -- THE PARAGRAPH ABOVE SAID "ONE TEST, the
//! same one that holds `filesystem` and `network`", AND IT NAMED "the client that DIES WHEN THE
//! SEED DECIDES that §3.1 asks for". BOTH HALVES ARE REWRITTEN AND NOT ANNOTATED BELOW, which is
//! finding A-2's rule, and they went for different reasons.
//! ⚠️ The second half was FALSE BEFORE THIS COMMIT and not because of it: the milestone 6 design
//! measured it at §6.7 -- `FakeGui::dies` is called EXPLICITLY by two tests and no seed moves it,
//! so the fake next door has never been the one §3.1 asks for. ✅ That fake exists now, and it
//! exists ELSEWHERE: `simulator::ipc::DyingGui` dies at an operation drawn from the seed, with
//! its own bench in `crates/simulator/tests/dying_gui.rs` and the campaign of §5.7's third
//! property in `crates/simulator/tests/gui_death_campaign.rs`.
//! ⚠️ The first half went with it -- MEASURED, not reasoned: `grep -rnE "^ *impl Ipc for"
//! crates/` returns TWO lines on 2026-09-02, and a count that is quietly wrong is worse than one
//! that is loudly missing (gotcha #31).
//!
//! ⚠️ DATED RECALL, 2026-08-28 -- FINDING AUD-054. That list read "`filesystem`, `network` and
//! `process`", and `process` is OUT: from `5fceee1` (2026-08-21) its signatures are held by two
//! benches plus four `compile_fail` cases, so naming it here understated it. ⛔ What is removed
//! is ONE WORD, not the sentence. ⛔ DATED RECALL, 2026-09-02: this went on to say "measured,
//! `Ipc` has exactly one implementation from outside the crate, and so do `filesystem` and
//! `network`", and the `ipc` half of that measurement EXPIRED with milestone 6 task 9 -- there
//! are two now, counted above. What the AUD-054 correction was ABOUT is untouched, because it was
//! about `process` being named in a list it had outgrown; the reckoning for `process` lives on
//! `ports/process.rs`, in one house, and the figure for `filesystem` and `network` lives on
//! theirs rather than being restated here (`E77`: a number that lives in two places gets taken
//! out of one).
//!
//! ⛔ AND THE SIGNATURES ARE NOT COPIED FROM A SPEC TABLE, unlike `process`'s: §6.1 fixes the
//! schema, the stamp, the identifiers and the direction of initiative, and it does NOT carry a
//! table of normative signatures the way §6.10.2 does. These three come from the milestone 2
//! plan. Written down because it changes what a later reader may do with them: they are open
//! to a measured argument in a way `process`'s are not.

use alloc::vec::Vec;

/// Which connected client. Progressive, assigned by the core: §2.2 chose progressive
/// identifiers over random ones, and §6.1.3 is where that choice becomes binding. The list of
/// consumers of randomness inside the kernel is declared EMPTY, and minting these at random
/// would reopen it AT THE FIRST LINE OF SCHEMA -- gotcha #12 in its most insidious shape,
/// because nothing about `ClientId` looks like a source of non-determinism.
///
/// ⛔ AND §6.1.3 DOES NOT SAY "PROGRESSIVE" IN THE ABSTRACT: IT SAYS THE JOURNAL'S
/// PROGRESSIVES. The counter is the one `journal` will allocate for `StepId` -- deterministic
/// by construction and readable in a trace -- and it DOES NOT EXIST YET: see `ports::journal`
/// and the dated recall there, which is where the WHEN is registered. ⚠️ This line named
/// milestone 3 until 2026-08-21, and that milestone closed without it. Whoever implements this
/// port in milestone 6 draws from THAT counter rather than starting a private one of its own:
/// two independent counters that look identical are a divergence nothing would report.
///
/// ⛔ DATED RECALL, 2026-08-31 -- THE SUBJECT AGED AND THE CLAIM DID NOT, so the sentence is
/// RE-POINTED AND NOT REMOVED (§6.5, gotcha #87). MILESTONE 6 DOES NOT IMPLEMENT THIS PORT: it
/// brings the SCHEMA -- `crate::wire::ipc` -- and the transport is staged out (open item 5).
/// Measured rather than assumed: `grep -rnE "^ *impl Ipc for" crates/` finds TWO FAKES and no
/// transport. ⛔ ANCHORED, AND SPELLING OUT `for`: the bare `impl Ipc` matches
/// `impl IpcMessage` too, and an unanchored one matches the paragraphs that quote it.
/// ⚠️ DATED RECALL, 2026-09-02: that line read "still finds ONE bench fake and nothing else",
/// and task 9 of this milestone added the second -- `simulator::ipc::DyingGui`. ⛔ THE CLAIM IT
/// SUPPORTS IS UNCHANGED AND THAT IS WHY THE FIGURE IS CORRECTED RATHER THAN DROPPED: a fake is
/// not a transport, so "this milestone does not implement this port" is as true at two as it was
/// at one, and the counter it guards -- two independent progressive counters that look identical
/// -- is guarded by neither number but by the sentence above.
/// ⚠️ Read "whoever implements this port" WITH NO MILESTONE ATTACHED. This line has now
/// named two, and naming a third would only schedule the same correction again.
/// ⛔ DATED RECALL, 2026-09-20 -- IT SCHEDULED IT, AND THE APPOINTMENT CAME: task 2 of
/// sub-project 2 brought `platform::ipc::LocalSocketIpc`, so "finds TWO FAKES and no transport"
/// above is false in BOTH halves, and tasks 7 and 10 added two more fakes. ✅ THE FIGURE IS
/// REPLACED BY THE COMMAND AND NOT BY A THIRD NUMBER, which is what this very paragraph asked
/// for: `grep -rnE "^ *impl Ipc for" crates/`. ⛔ WHAT THE SENTENCE ABOVE CLAIMS IS UNTOUCHED:
/// MILESTONE 6 did not implement this port, and that stays true however many implementations
/// exist now -- the dated recall is re-pointed, never dropped (§6.5, gotcha #87). E89 of the
/// sub-project 2 plan holds the record.
/// ⛔ AND REMOVING IT WOULD LEAVE UNGUARDED THE VERY DEFECT IT EXISTS TO PREVENT, which is the
/// whole reason the fix is a re-point: two independent counters that look identical, diverging
/// with nothing to report it.
/// ⚠️ DATED RECALL, 2026-09-17 -- THE COUNTER IT GUARDS NOW EXISTS, AND THIS PORT IS STILL NOT
/// IMPLEMENTED HERE. It is `crate::numbering::Progressive`, seeded by
/// `crate::numbering::seeded_from` above every step in the journal. "Whoever implements this
/// port in milestone 6 draws from THAT counter rather than starting a private one of its own" is no longer a
/// promise about a type that does not exist: the type is one line away, and a private `u64`
/// inside the transport is now a visible choice rather than the only road.
/// ⚠️ RECALL OF 2026-10-02 -- audit of 2026-09-30, AUD-045, AUD-052: the transport and
/// `crate::serving::Core` now draw from the SAME counter -- the composition root builds one and
/// hands each a `crate::numbering::Progressive::share` of it -- and the type no longer copies.
///
/// ⚠️ "ASSIGNED BY THE CORE" MEANS "NOT CHOSEN BY THE CLIENT", and the line is worth spending
/// because the other reading contradicts the signature below it. `accept` RETURNS one, so the
/// party that mints it is whoever implements this trait -- `platform`, which lives inside the
/// core process. What the sentence rules out is a gui that names itself.
///
/// ⚠️ THE PLAN DICTATED `PartialOrd, Ord, Hash` AND A `get()`, AND ALL FOUR ARE GONE. Measured
/// one at a time, removing and rebuilding, the way `CheckpointId::get()` went at task 10 --
/// each removal left `cargo build --workspace` and `cargo test --workspace` green with zero
/// warnings.
///
/// - `get()` -- the argument FOR it is real and it is the one that kept `SingleReceipt::id`
///   alive: whoever implements must map a `ClientId` onto a real pipe. It does not land here,
///   and the difference is the whole point. A receipt CANNOT be `Copy` -- duplicating one
///   would destroy the consumption guarantee it exists to give -- so extracting the number is
///   the only correlation left. `ClientId` IS `Copy`, so an implementation RETAINS one and
///   compares it, exactly as `InMemoryFilesystem` does with `CheckpointId` and never asks for
///   its value. The fake is what settled it: written first, it needed no getter.
/// - `Ord`/`PartialOrd` -- the argument FOR is that gotcha #12 bans `HashMap` and pushes
///   toward `BTreeMap`, which demands `Ord`. It does not survive the #46 test, which is the
///   one that matters for whoever implements this port from outside: an outside implementation
///   is not BLOCKED without it -- a table plus `==` works, as every other fake here does and as
///   `platform::ipc::LocalSocketIpc` does -- and unlike a missing accessor or constructor, `Ord`
///   can be added later by anyone, in one line, breaking nothing. That is a convenience, not the
///   entry door of whoever comes. ⚠️ RECALL OF 2026-10-03 -- audit of 2026-09-30, AUD-550.
/// - `Hash` -- worse than unused, and refused for the reason `Path` and `StepId` refuse it:
///   its consumer is `HashMap`, which `tests/compile_fail/hashmap_in_kernel.rs` forbids
///   outright. ⛔ A DERIVE THAT ENABLES THE FORBIDDEN THING IS WORSE THAN ONE NOBODY CALLS: it
///   makes the violation one keystroke cheaper in `platform`, where `std` is reachable.
///
/// ⛔ AND EVERY DERIVE THAT REMAINS IS ACCOUNTED FOR, three by a red and one by the compiler --
/// the list is closed on purpose, so that nobody arriving with task 11's pruning lesson finds
/// one without a reason beside it. `PartialEq`/`Eq` is the mechanism that REPLACES the getter
/// above, so that argument would have rested on nothing had it not been checked: removed,
/// `E0369`, `==` cannot be applied. `Copy` -- `E0382`, twenty-three of them over EIGHT
/// declaration sites. `Debug` -- `E0277`, demanded by the `assert_eq!`s, which is why it stays
/// here where `Grant` lost it. ⚠️ `Clone` IS NOT A CHOICE AND NEVER WAS: `Copy` requires it
/// (`trait Copy: Clone`), so it cannot be weighed on its own. Measured rather than asserted --
/// dropped while `Copy` stays, `kernel` itself fails to build with
/// `E0277: the trait bound ClientId: Clone is not satisfied`, which is a stronger refusal than
/// any of the three above: not a test going red, the crate not compiling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClientId(u64);

impl ClientId {
    pub const fn new(value: u64) -> Self {
        ClientId(value)
    }
}

/// What can go wrong on the way to a client.
///
/// ⛔ BOTH VARIANTS HAVE A PRODUCER, the transport `platform::ipc::LocalSocketIpc`: `Disconnected`
/// when a peer has gone or was never in its table, `MalformedMessage` when a declared length
/// passes the cap it is delivered. The note on `FilesystemError`, `NetworkError` and
/// `ProcessError` -- no implementation, so no variant has a producer -- holds for those ports and
/// no longer for this one. ⚠️ RECALL OF 2026-10-02 -- audit of 2026-09-30, AUD-078.
///
/// ⚠️ TWO VARIANTS AND THREE METHODS, so not every word is reachable on every path -- and that
/// is deliberate rather than sloppy. `MalformedMessage` belongs to `receive`, where bytes
/// ARRIVE; the bytes handed to `send` were produced by the core with the kernel's own schema,
/// so a malformed one there is a defect of the kernel and not a failure of this port.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpcError {
    /// The client is gone. NOT an error condition of the core: the gui is sacrificial,
    /// and its disconnection is the signal that triggers reconciliation.
    ///
    /// ⚠️ ONE WORD FOR TWO FACTS, and it is a decision: "this client died" and "this
    /// identifier was never issued" are the same thing seen from the core -- THERE IS NOBODY
    /// THERE -- and neither one gives the core a different move to make.
    Disconnected,
    /// The peer sent a frame the transport will not take. In `platform::ipc::LocalSocketIpc`
    /// that is a declared length past the cap it is delivered, and from then on every `receive`
    /// from that client answers this: the state is PERMANENT.
    ///
    /// ⛔ IT IS NOT THE DECODING, which this port never does: it hands over whole frames. A body
    /// that does not decode, or whose bytes consumed do not equal the declared length -- the rule
    /// of gotcha #34, the one `ProcessError::MalformedFrame` keeps on its own port -- is a
    /// `crate::framing::WireError`, answered by `crate::wire::ipc::IpcMessage::decode` inside the
    /// core, and `crate::serving` absorbs it without going through this type. ⚠️ RECALL OF
    /// 2026-10-03 -- audit of 2026-09-30, AUD-059, AUD-060.
    ///
    /// ⚠️ DISTINCT FROM `Disconnected` ON PURPOSE: a peer that talks nonsense is still there.
    /// Collapsing the two would have the core tear down a live gui over one bad frame.
    MalformedMessage,
}

pub trait Ipc {
    /// Accepts a client that is waiting, if there is one. Never blocks: readiness comes
    /// from the `reactor`, as for every other port.
    ///
    /// ⛔ IT RETURNS AN `Option` WHERE THE OTHER TWO RETURN A `Result`, and that asymmetry is
    /// intended. `IpcError::Disconnected` is a statement ABOUT A `ClientId` -- "the one you
    /// are naming is not there any more" -- and this is the only method that TAKES no
    /// `ClientId`. It carries no assumption that could have gone stale between being formed
    /// and being used, so there is nothing for that word to be said about; and it decodes
    /// nothing, so the other word is out of reach too. The failure modes this vocabulary knows
    /// simply do not exist here.
    ///
    /// ⛔ AND "NOBODY IS WAITING" IS NOT THE FAILURE IT RESEMBLES. The gui is 0..1 and
    /// sacrificial and the core holds all the authoritative state (I1, ADR-0004), so the core
    /// runs perfectly well with no gui at all: `None` is the ORDINARY state of this port, and
    /// for long stretches the only one. A `Result` would dress the normal case as an error
    /// path, which is how a caller learns to ignore the error path.
    ///
    /// ⚠️ DECLARED RATHER THAN LEFT TO BE DISCOVERED: a LISTENER that has itself broken -- as
    /// opposed to a client that has -- gets no word from this vocabulary today, and surfaces here
    /// as `None`, which is a wrong value rather than an error (gotcha #30). ⛔ AND IT IS NO LONGER
    /// A HYPOTHESIS: the transport, `platform::ipc::LocalSocketIpc`, answers `None` for EVERY error
    /// of its listener, and only `WouldBlock` among them means "nobody is knocking".
    ///
    /// ⛔ AND THE RESIDUE IS ALSO AN ASYMMETRY BETWEEN THESE SIGNATURES, which is worth saying
    /// straight because the cheap reading gets the PRICE OF CLOSING IT wrong. `receive` two
    /// methods below returns `Result<Option<Vec<u8>>, IpcError>`: there "nothing is ready" and
    /// "it is broken" are ALREADY distinct, and the ordinary case is not dressed as an error.
    /// `accept` could have had that exact shape. The argument above refutes
    /// `Result<ClientId, IpcError>`, where `None` would have to BECOME an error -- it does not
    /// touch `Result<Option<ClientId>, IpcError>`, the form the neighbouring method already
    /// uses. So the residue is not only "are two variants enough": it is that `accept` HAS NO
    /// ERROR CHANNEL AT ALL while `receive` has one.
    ///
    /// ⛔ THE COST THAT FOLLOWS, and it is the part a later reader would otherwise get wrong:
    /// adding a third variant tomorrow WOULD NOT CLOSE THIS. There is nowhere to return it.
    /// Closing it means CHANGING THE SIGNATURE, not widening the enum -- and whoever reopens
    /// this should know that before deciding it is cheap.
    ///
    /// ⚠️ AND THE SIGNATURE STILL STANDS AS IT IS, BUT THE REASON IT WAS KEPT FOR HAS AGED. It was
    /// kept because a `Result` here would have been one that could never be `Err` -- dead surface,
    /// of exactly the kind this port pruned three derives and a getter for -- and that held while
    /// the port had no implementation. A real listener CAN fail, so the `Err` would now have a
    /// producer, and only the word for it is missing. Whether to change the signature and add that
    /// word, or keep the `Option` and declare for good that the transport swallows a broken
    /// listener, is the owner's: registered by the audit of 2026-09-30, AUD-533. Until then the
    /// posture is `network`'s declared open question, for the same reason: a minimal vocabulary
    /// can be widened, a rich wrong one cannot (ADR-0009). ⚠️ RECALL OF 2026-10-02 -- same audit,
    /// AUD-533, AUD-078, AUD-551.
    fn accept(&mut self) -> Option<ClientId>;

    /// Sends bytes to a client.
    ///
    /// ⛔ CALLED BY THE CORE, WHEN THE CORE DECIDES (§6.1.4). This is the only operation that
    /// emits, and no call in this trait lets a client provoke it.
    fn send(&mut self, client: ClientId, message: &[u8]) -> Result<(), IpcError>;

    /// Takes the next message from a client, if one is ready.
    ///
    /// ⚠️ `Ok(None)` IS NOT AN ERROR: an idle client and a broken one must not give the same
    /// answer, or the core could not poll this port at all.
    ///
    /// ⛔ AND THIS IS NOT A PULL, which is the sentence §6.1.4 exists to protect. What comes
    /// back is what the CLIENT chose to say; the core still decides whether and when to
    /// answer, and it answers with `send`.
    fn receive(&mut self, client: ClientId) -> Result<Option<Vec<u8>>, IpcError>;
}
