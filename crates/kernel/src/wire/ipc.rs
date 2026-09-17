//! The schema of the `ipc` channel: the envelope of `crate::framing` carrying ONE enum.
//!
//! ⛔ ONE ENUM FOR BOTH DIRECTIONS, AND THE DIRECTION IS DOCUMENTED RATHER THAN TYPED. Two
//! enums of one variant each would leave BOTH discriminants unexercised, which is the very
//! thing §6.7 asks two messages for. And typing the direction would buy nothing at the port:
//! `send` takes `&[u8]` and `receive` returns `Vec<u8>`, so the boundary sees no type at all.
//! ⚠️ THE COST, stated: nothing stops a caller from encoding a `Verdict` and sending it UP.
//! Today there is no such caller -- the transport is staged out (open item 5) -- and the day
//! there is one, the guard that pays for itself is on the composition side, not here.
//!
//! ⛔ THE SCHEMA MINTS NO IDENTIFIERS BECAUSE IT CARRIES NONE, and saying it that way is the
//! point (§6.5). Writing "§6.1.3 is satisfied" would be green having compared empty sets. A
//! grant request is not a step of a run: it writes no record and carries neither `StepId` nor
//! `RunId`. The first message that carries an identifier is where the rule becomes real, and
//! where its probe is born.
//!
//! ⛔ NO VERSION ENUM, NO RETIRED-INDEX REGISTER, NO FROZEN BYTES -- I4 renounces versioning
//! (§6.4). What stands in its place is the BUILD STAMP of §6.1.2, WHICH THIS MILESTONE DOES
//! NOT BUILD (§3.4). Until it exists, NOTHING REFUSES A STALE GUI, and today that costs
//! nothing because there is no gui to refuse -- `grep -rnE "^ *impl Ipc for" crates/` returns
//! a bench fake. The trigger is milestone 2 of the subproject, the one that brings the shell.
//!
//! ⛔ AND THE REVOCATION core -> gui IS A DECLARED NON-CONSTRUCTION. ADR-0033 names it -- "the
//! gui stops rendering the 3D and says so" -- and it is the first message this vocabulary will
//! gain. It is not here because no written line demands it today and because §5.7 row 3 speaks
//! of a gui that DIES, not one that is asked. ⚠️ THE COST IS REAL: until then a discretionary
//! grant is preemptible IN THE BOOKS and the gui never hears about it. Open item 7. ⛔ ITS
//! TRIGGER IS THE SAME SHELL, and it is written out rather than inherited from the paragraph
//! above, because the two non-constructions could have had different ones: a revocation needs
//! an ADDRESSEE, and until milestone 2 of the subproject there is nobody to tell. Inventing
//! the message before there is one would freeze a vocabulary against an imaginary consumer --
//! gotcha #46 from the wrong side, which is the same reason §3.4 gives for the stamp.
//!
//! ✅ DATED RECALL, 2026-09-17 -- THE TRIGGER WRITTEN ABOVE FELL DUE TODAY AND IS NOT HONOURED,
//! WHICH IS THE OPPOSITE OF WHAT "the same shell" READS LIKE. The shell arrived (ADR-0029,
//! Electron, 2026-09-10) and the revocation is STILL not built, because the sentence names the
//! shell while the mechanism needs an ADDRESSEE: sub-project 2 does not serve `Request` at all
//! (decision D5 of its plan, argued against this file's own doc of `GrantRequest`), so NO
//! ORDINARY GRANT EXISTS to revoke and there is nobody to tell. ⛔ THE TRIGGER IS THEREFORE
//! THE 3D CONSUMER, sub-project 7 -- the same closer row 27 of milestone 6 carries in
//! docs/porta-di-qualita.md. ⚠️ CORRECTED RATHER THAN DELETED: the paragraph above is the only
//! place that says what the revocation IS, and it is gotcha #77 again -- a deadline written in
//! prose, which nothing can go red for. ⛔ AND A THIRD HALF, FROM TASK 2 OF THE SAME PLAN:
//! the sentence above that `grep -rnE "^ *impl Ipc for" crates/` "returns a bench fake" is
//! false too -- `platform::ipc::LocalSocketIpc` is the real transport now, and the command that
//! counts the implementations is the one written there, not this prose.
//!
//! ✅ AND THE OTHER HALF DID ARRIVE: the BUILD STAMP of §6.1.2 exists as of today,
//! `crate::wire::ipc::build_stamp` over `stamp_set`. "Until it exists, NOTHING REFUSES A STALE
//! GUI" above is now false, and the handshake that uses it is task 7 of the same plan.

use alloc::string::String;
use alloc::vec::Vec;
use bincode::{Decode, Encode};

use crate::arbiter::{ComputeClass, Mib, Preemption};
use crate::framing::{self, WireError};
use crate::time::Millis;

/// What the gui asks for: an ordinary grant beyond the presentation quota (ADR-0033).
///
/// ⛔ IT IS NOT A `ResourceProfile`, AND THE MISSING FIELD IS THE REASON. `ResourceProfile`
/// carries `name: &'static str`, which cannot be produced from arriving bytes without leaking
/// -- and what would be leaked is text CHOSEN BY THE GUI, i.e. untrusted content (ADR-0014)
/// inside a type the arbiter DECIDES with. The split here is the one ADR-0005 already
/// describes: THE REQUESTER DECLARES THE RESERVATION, and the core names the profile.
///
/// ⛔ AND CARRYING THE NAME AND RESOLVING IT IS NOT THE OTHER ROAD: NOTHING MAPS A NAME ONTO
/// A PROFILE. `grep -rn "^[^/]*ResourceProfile {" crates/ --include=*.rs` gives the
/// declaration, the bench helpers that BUILD one, and the sites that spell the fields out; no
/// line maps a name onto a profile, and building one would be a mechanism no written line asks
/// for. ⛔ ANCHORED PAST THE DOC COMMENT, for the reason `crate::ports::ipc` writes down for
/// `impl Ipc for`: an unanchored one matches the paragraph that quotes it.
///
/// ⚠️ THE COST, stated: WHAT THE CORE PICKS IS THE NAME, NOT THE PROFILE. The three fields below
/// are the gui's, and `name` is the one field NO ARBITER DECISION READS: `crate::arbiter::Held`
/// copies out of the profile what it decides with and leaves the name behind, calling one kept
/// inside the arbiter "state no decision reads". ADR-0033 describes a single consumer, the 3D
/// viewer beyond the quota, and that stops being enough the day a second one exists.
///
/// ⛔ SO THE UNTRUSTED HALF IS NOT ONLY THE NAME: `compute_class` AND `preemption` REACH THE
/// ARBITER FROM THE SAME PEER AS THE NAME THIS TYPE REFUSES, AND THE ARBITER OBEYS THEM. Read
/// in `crate::arbiter` rather than inherited. `admit` hands `profile.compute_class` to
/// `ask_back` as `below`, where the `askable` closure drops every holder with
/// `held.lane <= below`; the order is `ComputeClass::priority`, so a request declaring
/// `Realtime` clears that guard for every `Interactive` and `Batch` holder, and one declaring
/// `Batch` clears it for none. The same field is the lane `enqueue` files a waiting request
/// under. `preemption` is read by `issue`, which maps `Preemption::Never` onto
/// `Activity::NonPreemptible` and takes `grace` from that same field, so a request declaring
/// `Never` books a grant `ask_back` can never pick. Both are a stronger privilege than a
/// string, and NEITHER IS CHECKED AGAINST ANYTHING -- `reserved_vram`, by contrast, has to
/// clear the ceiling in `admit`.
///
/// ⛔ WHICH IS TO SAY THE OTHER HALF OF ADR-0005 HAS NO HOUSE YET: the reservation is
/// "declared by the requester and VERIFIED BY THE ARBITER", and only the declaring half is
/// built here. It is not exploitable today because THE CONSUMER DOES NOT EXIST -- nothing
/// turns a message into a `ResourceProfile`, and
/// `grep -rnE "^[^/]*(GrantRequest|IpcMessage)" crates/` names only this file and
/// `crates/kernel/tests/ipc_wire.rs`. ⛔ THE TRIGGER IS THAT CONSUMER: the task that first
/// decodes arriving bytes into a profile is where the verifying half is written and where its
/// probe is born. ⚠️ NARROWING THE FIELDS HERE IS NOT THE SMALLER FIX: the three are what
/// decision D16 of the milestone-6 plan and §6.2 of the design ask for, and cutting them would
/// reopen a design decision to protect a caller that does not exist -- gotcha #46 from the
/// wrong side.
///
/// ⚠️ THE DERIVES, MEASURED ONE AT A TIME ON 2026-08-31 RATHER THAN ARGUED, because task 11 of
/// milestone 5 spent a review pruning derives nobody could name a consumer for, and a list
/// asserted to be accounted for is exactly what that review found. Removed and rebuilt, each
/// on its own: `Debug` -- `E0277`, `IpcMessage` doesn't implement `Debug`, demanded by the
/// `assert_eq!`s of `crates/kernel/tests/ipc_wire.rs`. `PartialEq` -- `E0369`, `==` cannot be
/// applied. `Encode`/`Decode` are the schema itself.
/// ⛔ `Eq` AND `Clone` HAVE NO CONSUMER, AND THAT IS MEASURED, NOT SUSPECTED: dropped one at a
/// time, `cargo build --locked --workspace --tests` compiles with ZERO errors and ZERO
/// warnings. They are here because they are the shape of `crate::wire::worker::FromWorker`,
/// which E33 of this task's errata pointed at, and pruning them is REGISTERED AND NOT TAKEN --
/// the precedent that argues for pruning lives on `crate::ports::ipc::ClientId`, which refused
/// `Ord` because a derive addable later in one line "is a convenience, not the entry door of
/// whoever comes"; the precedent that argues for keeping is the one on the ports themselves,
/// where callers are empty by construction and the criterion cannot tell dead from not-yet.
/// This channel has no transport yet (open item 5), so both readings are live and the choice
/// is the owner's.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct GrantRequest {
    pub reserved_vram: Mib,
    pub compute_class: ComputeClass,
    pub preemption: Preemption,
}

/// The three-way outcome, WITHOUT the grant.
///
/// ⛔ `Granted` IS A UNIT VARIANT AND CARRIES NO `Grant`, which is the whole of this type. A
/// decodable `Grant` would be a capability MINTED FROM BYTES: §5.6 holds that the only site
/// that mints one is `Arbiter::issue`, and `tests/compile_fail/grant_has_no_constructor.rs`
/// exists to make it unspeakable from outside. It would be AUD-050 done again on the
/// strongest token in the project -- a guard is worth exactly what its constructor is worth.
/// ⚠️ AND THE GUI DOES NOT NEED ONE: ADR-0033 says the grant is STATE OF THE CORE (I1). What
/// crosses is the verdict.
///
/// ⛔ THAT PROPERTY IS HELD BY CONSTRUCTION AND NOT BY A NEGATIVE CASE, and the distinction is
/// written rather than left to be inferred: a `compile_fail` case saying "a verdict may not
/// carry a `Grant`" would be a NEW CATALOGUE ROW, i.e. §7.4, i.e. spec -- the owner's, global
/// constraint 7. What holds it today is that the variant is unit and that this paragraph says
/// why. Registered, not taken.
///
/// ⚠️ `Refused` CARRIES TWO NUMBERS AND `Queued` CARRIES NOTHING, and the asymmetry is
/// argued: design/02 wants "why it does not fit and the workable alternative", ADR-0020
/// forbids the kernel to suggest one, so THE INTERFACE BUILDS IT AND THE KERNEL HANDS OVER
/// THE MATERIAL -- the gui is the written consumer of those two. A ticket, by contrast, is
/// load-bearing only for a caller with TWO requests outstanding, and the gui has one.
///
/// ⚠️ Same derive accounting as `GrantRequest`, one house above.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum Verdict {
    Granted,
    Queued,
    Refused { asked: Mib, ceiling: Mib },
}

/// The identity of a build, §6.1.2. ⛔ IT IS NOT A CONTRACT, IT IS AN IDENTITY: one accepted
/// value, and a gui carrying a different one does not start and says so. I4 renounces
/// versioning, and this is the mechanism that stands in its place.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub struct BuildStamp(u64);

impl BuildStamp {
    /// ⛔ THE CONSTRUCTOR IS `crate::wire::ipc::build_stamp` AND NOT A `new` HERE, and that is
    /// the point: a stamp anyone can mint from any number is a stamp that proves nothing. The
    /// only value that exists is the one the canonical set produces.
    pub const fn get(&self) -> u64 {
        self.0
    }
}

/// What the core knows about the journal's protection, as a VALUE and not as fixed text in
/// the gui (G16, ADR-0023).
///
/// ⛔ ONE VARIANT AND NOT A `bool`, AND THE REASON IS ADR-0023 ITSELF: "encrypted at rest"
/// here means PROTECTED AS MUCH AS YOUR SYSTEM ACCOUNT, and that sentence has to reach the
/// interface -- a false sense of security is worse than none. A `bool` would let the gui
/// write its own sentence beside it; an enum makes the sentence the core's, and a second
/// level of protection a VARIANT rather than a silent change of meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub enum Protection {
    /// The keys are the OS's, reached through the platform module. ADR-0023.
    AsSystemAccount,
}

/// The degradation, on the wire. Twin of `crate::degradation::Degradation` -- D11.
///
/// ⚠️ THE TWO FIELDS ARE TODAY'S. ADR-0019 declares the event list OPEN (dated recall of
/// 2026-09-08), so a field added there must STOP HERE and be translated by hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub struct DegradationReport {
    pub vram_exhausted: bool,
    pub routing_degraded: bool,
}

/// Which VRAM policy is active, with the budget. Twin of `crate::arbiter::policy::VramPolicy`
/// -- and here the twin is FORCED: that enum carries the policies themselves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub enum PolicyName {
    /// ADR-0006's default: OpenRouter, VRAM free.
    Remote,
    Local,
}

/// The policy with what the gui shows beside it (G15/G16).
///
/// ⛔ `allocated` IS WHAT THE BOOKS SPEAK FOR, THE TWO PERMANENT QUOTAS INCLUDED, and that
/// is a decision rather than an oversight (D20). ADR-0033 holds those two as GRANTS WITH A
/// HOLDER and not as subtractions -- "the subtraction is not an exemption", gotcha #4 -- so
/// hiding them from this number would commit at the layer the user looks at the very mistake
/// the arbiter was built to avoid. `total` is the machine, delivered through
/// `Parameters::total_vram`; the kernel does not hold the audio and presentation quotas, and
/// deliberately does not -- the doc of that accessor argues it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub struct PolicyReport {
    pub policy: PolicyName,
    pub allocated: Mib,
    pub total: Mib,
}

/// The permission triple of ADR-0016, on the wire. Twin of `crate::permission::Permission`
/// -- and FORCED, for `GrantRequest`'s own reason: that type carries `&'static str`.
///
/// ⛔ `String` AND NOT `&'static str`, WHICH IS THE WHOLE OF THIS TYPE. A `&'static str`
/// cannot be produced from arriving bytes without leaking, and what would leak is text CHOSEN
/// BY THE PEER -- untrusted content (ADR-0014) inside a type a permission decision reads.
/// ⚠️ AND THE DIRECTION MATTERS: the core sends this DOWN (`PermissionRequired`) and receives
/// it back UP (`Approve`). The returning one is untrusted, so whoever consumes it matches it
/// against the triple IT asked for rather than trusting the strings -- written here so the
/// consumer does not rediscover it. The probe is born with that consumer, task 7.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct Triple {
    pub tool: String,
    pub resource: String,
    pub operation: Access,
}

/// The operation of the triple. Twin of `crate::permission::Operation` -- D11.
///
/// ⛔ AN ENUM AND NOT A `bool`, AND THE KERNEL ALREADY PAID FOR THIS LESSON: the doc of
/// `Operation::is_write` records that `matches!` folds EVERY other variant into `false`, so a
/// third operation would reach the DURABLE record as a read -- measured on 2026-09-01 with
/// `Execute` added. On the wire the same fold would make a third operation arrive as a read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub enum Access {
    Read,
    Write,
}

/// Which registry function, and with what argument. ADR-0038.
///
/// ⛔ NAMED `Call` AND NOT `Invocation`, and that is deliberate: task 6 brings
/// `crate::record::Invocation`, the DURABLE detail, and two types one letter apart in the two
/// worlds this plan keeps separate is the ambiguity D8 refused for `counter`.
///
/// ⚠️ THE ARGUMENT IS A `String` AND IT IS UNTRUSTED. Nothing here validates that `function`
/// names a registered function: the registry does, by REFUSING (task 6). A schema that could
/// only express registered names would be a second registry, kept aligned by hand.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct Call {
    pub function: String,
    pub argument: String,
}

/// Where a piece of text came from. Twin of `crate::record::Trust` -- and the twin the design
/// asked for by name, "so as not to hang `bincode` derives on a journal type".
///
/// ⛔ A MODEL'S TEXT IS UNTRUSTED (ADR-0014) and the gui marks it (G13). The label is
/// HEREDITARY: summarising or concatenating untrusted text leaves it untrusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub enum Provenance {
    Trusted,
    Untrusted,
}

/// What the seventh port holds under the layout key -- and its three states, decision 35.
///
/// ⛔ `Unavailable` IS NOT `Nothing`, AND CONFLATING THEM WOULD BE THE SILENT DEGRADATION
/// ADR-0019 FORBIDS. "Nothing" is an archive that opened and is empty -- a first run.
/// "Unavailable" is an archive that would not open at all, and the core starts anyway and
/// SAYS SO. A gui told "nothing" would offer to save; one told "unavailable" knows the save
/// will not stick.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum LayoutState {
    /// The opaque package: `toJSON()` of `dockview` plus the active view.
    Package(Vec<u8>),
    /// The archive opened and holds nothing under the key.
    Nothing,
    /// The archive would not open. The core started anyway (decision 35).
    Unavailable,
}

/// One line of the step list: in sub-project 2 these are registry invocations.
///
/// ⛔ A SUMMARY AND NOT THE RECORD. The journal's records must EVOLVE (ADR-0036) and this wire
/// renounces versioning (I4): sending the record itself would tie the two, and a field added
/// to a durable record would change these bytes with nothing going red.
///
/// ⛔ `done` IS A `bool` AND NOT AN `Option<bool>`, AND IT IS THE JOURNAL'S OWN VOCABULARY
/// (P-39). `RecordV1::outcome` carries no success flag: an outcome written says the step
/// CLOSED, and there is no "closed badly". A step whose effect failed returns before the
/// outcome is written and stays IN DOUBT (ADR-0007), which is `false` here. A third state
/// would be a variant on the wire with no producer, and an index on the wire never retires.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct StepSummary {
    pub step: u64,
    pub function: String,
    /// `false` while the step is still in doubt -- ADR-0007's intent written and no outcome yet.
    pub done: bool,
}

/// One message on the `ipc` wire.
///
/// ⚠️ Same derive accounting as `GrantRequest`.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum IpcMessage {
    /// gui -> core. The handshake, §6.1.2.
    Hello(BuildStamp),
    /// core -> gui. The stamp matched.
    Accepted(Protection),
    /// core -> gui. The stamp did not match; this carries the EXPECTED one, and the core
    /// stops listening to that client -- the port has no close (decision 22).
    StaleBuild(BuildStamp),
    /// core -> gui.
    Degradation(DegradationReport),
    /// core -> gui.
    Policy(PolicyReport),
    /// gui -> core.
    Invoke(Call),
    /// core -> gui. The triple the user must grant, before the invocation can run.
    PermissionRequired(Triple),
    /// gui -> core. ⛔ IT CARRIES THE CALL TOO, decision 21: `permission::grant` writes a NOTE
    /// and wants a step already open, and the core opens that step only when the invocation
    /// arrives. So `Approve` is enough, and the gui does not resend `Invoke`.
    Approve { triple: Triple, call: Call },
    /// core -> gui. In sub-project 2 only the fake core produces these.
    Token { text: String, provenance: Provenance },
    /// core -> gui. At the welcome after `Policy`, and again after every `SaveLayout`.
    Layout(LayoutState),
    /// gui -> core. The opaque package, when the layout settles and when the window closes.
    SaveLayout(Vec<u8>),
    /// core -> gui. At the welcome, and after every invocation.
    Steps(Vec<StepSummary>),
    /// gui -> core. ⚠️ NO GUI OF SUB-PROJECT 2 SENDS THIS -- D5, and the 3D is not in the 2.
    Request(GrantRequest),
    /// core -> gui.
    Verdict(Verdict),
}

impl IpcMessage {
    /// Encodes the message and wraps it in its envelope.
    ///
    /// ⛔ A FAILED ENCODING BECOMES AN EMPTY BODY RATHER THAN AN ERROR, on the containment
    /// argument `Record::encode` makes and `crate::wire::worker::FromWorker::encode` borrows
    /// -- READ AGAINST THIS TYPE'S GRAPH RATHER THAN COPIED. That graph is `Mib(u64)`,
    /// `ComputeClass`, `Preemption`, `Millis(u64)` and unit variants; the writer is a `Vec`
    /// that grows. No variant of `EncodeError` reachable without `std` is producible by it:
    /// they want a writer out of room, a borrowed `RefCell`, or a string the caller supplies.
    /// ⛔ AND THEY ARE NOT ENUMERATED, WHICH IS THE CHOICE: `EncodeError` is
    /// `#[non_exhaustive]` and the manifest asks for `version = "2"` rather than an exact pin,
    /// so a list of its variants is an assertion a future release can falsify IN SILENCE, with
    /// nothing here to go red. The relation survives a variant being added; a tally does not.
    ///
    /// ⚠️ THE SHAPE DIFFERS FROM THE TWIN'S AND THE ARGUMENT DOES NOT. `minicbor::encode`
    /// writes into a `Vec` the caller already owns, so its `Result` is dropped with `let _`;
    /// `encode_to_vec` RETURNS the `Vec`, so dropping the error needs a value in its place --
    /// and the value is the EMPTY body the containment argument already covers.
    ///
    /// ⛔ AND THE CONTAINMENT IS HELD RATHER THAN ASSERTED:
    /// `an_empty_body_in_an_honest_envelope_does_not_decode` is what makes it true. If an empty
    /// body decoded, a stopped encoder would put a message NOBODY WROTE on the wire, and this
    /// paragraph would be a promise kept by nothing.
    ///
    /// ⚠️ DATED RECALL, 2026-09-17 -- THE GRAPH READ ABOVE IS NO LONGER THIS TYPE'S GRAPH, AND
    /// THE ARGUMENT IS RE-READ RATHER THAN INHERITED. The variants added today put
    /// `String` and `Vec<u8>` into it. Re-read in bincode 2.0.1's `src/error.rs` the same day:
    /// the variants reachable without `std` are `UnexpectedEnd` (a writer out of room),
    /// `RefCellAlreadyBorrowed`, `Other(&'static str)` and `OtherString(String)` behind
    /// `alloc` -- and the last two are produced only by a HAND-WRITTEN `Encode`, which nothing
    /// here has. The relation holds; the enumeration did not, which is why it is dated instead
    /// of left standing. ⛔ AND IT IS NOW MEASURED RATHER THAN ARGUED:
    /// `a_string_in_the_schema_still_cannot_stop_the_encoder` in
    /// `crates/kernel/tests/ipc_wire.rs` is the probe, and it feeds the awkward string rather
    /// than a convenient one.
    pub fn encode(&self) -> Result<Vec<u8>, WireError> {
        let body = bincode::encode_to_vec(self, bincode::config::standard()).unwrap_or_default();
        framing::frame(&body)
    }

    /// Reads a message out of an envelope.
    ///
    /// ⛔ TWO CHECKS AND NOT ONE, and they catch different faults: `framing::unframe` catches a
    /// frame whose declared length does not match the bytes -- a TRUNCATION, which no decoder
    /// can see because the missing tail is simply not there -- while `used != body.len()`
    /// catches a body that carries a COMPLETE MESSAGE AND SOMETHING AFTER IT, inside a length
    /// that tells the truth. The decoder stops at the end of the value it understood and
    /// reports how far it got; without this comparison the leftover would pass unread. It is
    /// the line `Record::decode` already carries, finding AUD-047.
    ///
    /// ⚠️ `decode_from_slice` FIXES `Decode`'s CONTEXT PARAMETER TO `()`, which is why nothing
    /// in this signature names it: the trait is `Decode<Context>`, and choosing the context is
    /// the caller's only when the caller wants one.
    pub fn decode(bytes: &[u8]) -> Result<Self, WireError> {
        let body = framing::unframe(bytes)?;
        let (message, used) = bincode::decode_from_slice(body, bincode::config::standard())
            .map_err(|_| WireError::Malformed)?;
        if used != body.len() {
            return Err(WireError::Malformed);
        }
        Ok(message)
    }
}

/// The canonical set: ONE message per variant, in a fixed order.
///
/// ⛔ ONE FUNCTION AND NOT TWO LISTS. The fixtures are generated from this and the stamp is
/// computed from this, so THEY CANNOT DRIFT APART: a variant added here changes both, and a
/// variant added to `IpcMessage` and forgotten here is caught by
/// `every_variant_is_in_the_canonical_set` in `crates/kernel/tests/ipc_wire.rs`, an integration
/// bench OUTSIDE this crate. Two lists would be two places to keep aligned for one property,
/// and the first one to stop being updated lies in silence -- the argument
/// `crates/kernel/tests/frozen/record_v1.map` makes about itself.
///
/// ⚠️ THE VALUES ARE ARBITRARY BUT NOT RANDOM: each one is chosen so that no two encodings
/// are equal and no field is left at its type's default, because a fixture full of zeroes
/// cannot tell a field that is written from one that is skipped. ⚠️ ONE DECLARED EXCEPTION:
/// `DegradationReport::routing_degraded` is `false`, its default -- with two `bool`s this rule
/// and P-34's (two equal values at two offsets pin one offset and its mirror) cannot both
/// hold, and P-34's wins. Do not "fix" it.
pub fn stamp_set() -> Vec<IpcMessage> {
    alloc::vec![
        IpcMessage::Hello(BuildStamp(0x0123_4567_89AB_CDEF)),
        IpcMessage::Accepted(Protection::AsSystemAccount),
        IpcMessage::StaleBuild(BuildStamp(0xFEDC_BA98_7654_3210)),
        IpcMessage::Degradation(DegradationReport {
            vram_exhausted: true,
            routing_degraded: false,
        }),
        IpcMessage::Policy(PolicyReport {
            policy: PolicyName::Remote,
            allocated: Mib::new(12288),
            total: Mib::new(16384),
        }),
        IpcMessage::Invoke(Call {
            function: String::from("arbiter.set_policy"),
            argument: String::from("local"),
        }),
        IpcMessage::PermissionRequired(Triple {
            tool: String::from("arbiter"),
            resource: String::from("policy"),
            operation: Access::Write,
        }),
        IpcMessage::Approve {
            triple: Triple {
                tool: String::from("arbiter"),
                resource: String::from("policy"),
                operation: Access::Write,
            },
            call: Call {
                function: String::from("arbiter.set_policy"),
                argument: String::from("local"),
            },
        },
        IpcMessage::Token {
            text: String::from("ciao"),
            provenance: Provenance::Untrusted,
        },
        IpcMessage::Layout(LayoutState::Package(alloc::vec![0x7B, 0x7D])),
        IpcMessage::SaveLayout(alloc::vec![0x5B, 0x5D]),
        IpcMessage::Steps(alloc::vec![StepSummary {
            step: 42,
            function: String::from("arbiter.set_policy"),
            done: true,
        }]),
        IpcMessage::Request(GrantRequest {
            reserved_vram: Mib::new(2048),
            compute_class: ComputeClass::Interactive,
            preemption: Preemption::After(Millis::new(500)),
        }),
        IpcMessage::Verdict(Verdict::Refused {
            asked: Mib::new(4096),
            ceiling: Mib::new(1024),
        }),
    ]
}

/// The build stamp: FNV-1a over the encoding of the canonical set.
///
/// ⛔ WRITTEN BY HAND AND NOT A DEPENDENCY, and the two reasons are different. ADR-0031 makes
/// adding a crate to the kernel's list a deliberate act, and this is SIX LINES. And it is an
/// IDENTITY, not a defence: nothing here resists a peer that wants to forge a stamp, because a
/// peer that can forge one is already inside the process boundary. ⚠️ THE DAY THIS IS ASKED TO
/// BE A DEFENCE IT IS THE WRONG FUNCTION, and the note is here rather than in the design
/// because this is where someone would reach for it. ⛔ AND IT IS NOT ADR-0018's FINGERPRINT
/// for pruned payloads, which remains a registered decision of the owner
/// (`crate::ports::journal`, the doc of `prune`).
///
/// ⚠️ THE LENGTH GOES IN TOO, not just the bytes: without it two adjacent messages could be
/// re-split differently and hash the same.
pub fn build_stamp() -> BuildStamp {
    let mut hash: u64 = 0xCBF2_9CE4_8422_2325;
    for message in stamp_set() {
        let bytes = message.encode().unwrap_or_default();
        for byte in (bytes.len() as u64).to_be_bytes().iter().chain(bytes.iter()) {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
        }
    }
    BuildStamp(hash)
}
