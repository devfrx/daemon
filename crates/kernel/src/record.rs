//! The durable record (§4.9). ⛔ EVERY DURABLE RECORD DECLARES ITS OWN VERSION, AND ITS
//! FIELDS ARE IDENTIFIED BY EXPLICIT INDEX — ADR-0036, and the six rules are in §4.9.2.
//!
//! ⛔ THE ENCODING LIVES HERE, IN `kernel`, and the `journal` port exchanges BYTES. Three
//! reasons, from §4.9.3: the data model is the kernel's property (§4.4); with bytes on the
//! port the SIMULATOR EXCHANGES BYTES TOO, so the DST campaign really exercises encoding and
//! decoding instead of going around them; and the measured cost is small.
//!
//! ⛔ ARRAY ENCODING, NOT MAP, AND IT IS WRITTEN OUT EVEN THOUGH IT IS THE DEFAULT.
//! Measured in ADR-0036: array 27 bytes (+4 %), map 33 (+27 %), positional 26. The ADR notes
//! that the earlier estimate "priced the map instead of the array" — so the number that
//! decided this is the array one. A default nobody wrote down is a default somebody changes.
//! ⚠️ AND THAT IT COSTS NOTHING TO WRITE IT WAS MEASURED, not assumed: with and without
//! `#[cbor(array)]` on the two types below, a record encodes to the same bytes down to the
//! length. ⚠️ THE BYTES OF THAT MEASUREMENT WERE `82 00 81 84 00 01 00 40` UNTIL 2026-08-10,
//! when index 4 arrived and the inner array went from four elements to five: an empty record is
//! now `82 00 81 85 00 01 00 40 60`, nine bytes. Re-measured rather than left standing —
//! gotcha #31 is a number nobody rechecks because the rule it supports is right, and the rule
//! here is still right.
//!
//! ⚠️ AND THE ARRAY HAS A PRICE THE MAP DOES NOT, which belongs beside those numbers: a
//! RETIRED INDEX COSTS A NULL BYTE FOR EVER. The array is positional, so a gap has to be
//! written to keep the ones after it in place, whereas a map simply omits the key. The
//! comparison above is between the shapes as they are TODAY; every index the format retires
//! moves it by one byte per record, in the archive's favourite direction, which is bigger.
//!
//! ⛔ AND `#[cbor(index_only)]` ON THE THREE ENUMS CARRIES ITS OWN CONSTRAINT, declared here
//! because it binds a FUTURE change and nothing in the file would otherwise say so: it encodes
//! a variant as its bare index, with no room for a body, so a variant under it can NEVER GAIN A
//! FIELD. The day one of them needs to carry a value — a `Verifiable` that names what to ask
//! the world, say — the annotation comes off and EVERY RECORD EVER WRITTEN changes shape. That
//! is a new version of the record, not an edit to this line. The byte-string annotation below
//! declares its own stake; this one is the stake of the three above it.

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use minicbor::{Decode, Encode};

/// Is this the INTENTION of a step, its OUTCOME, or a NOTE upon it? The whole write-ahead
/// protocol rests on telling the first two apart: a step with an intent and no outcome is IN
/// DOUBT (§4.2), and the doubt is what makes recovery possible.
///
/// ⛔ `Note` ARRIVED ON 2026-08-10 AND IS A THIRD THING, not a variety of either. It was
/// decided by the coordinator while executing task 7 and is recorded in the plan's errata so
/// the owner can overturn it by SEEING it. What forced it, measured rather than argued:
/// `Untrusted::promote` writes a record onto THE CALLER'S STEP, which already carries an
/// intent, and the two roads the plan left open both fail —
///
/// - written as a second `intent`, the port REFUSES it (`OutOfOrder`, one intent per step);
///   and even with the guard relaxed, reconciliation reads a second `Intent` record for the
///   same step and REPLACES the caller's resolution with the note's. Measured: a caller that
///   declared `Idempotent` came back `SuspendAndAsk` — the promotion silently downgraded a
///   step it does not own.
/// - written as an `outcome`, reconciliation takes the step OUT OF THE DOUBT although it has
///   not executed. Measured: `steps_in_doubt` answered `[]`. That is a true doubt vanishing in
///   silence, which is the one failure ADR-0007 exists to prevent.
///
/// So a note is neither, and the record says so. `crate::reconcile` neither opens nor closes a
/// doubt on it.
///
/// ⚠️ THE COST IS DECLARED AND IT IS A FORMAT COST: a build that does not know this variant
/// decodes such a record to `RecordError::Malformed`. ⛔ THE DIRECTION IS SAFE, and that is
/// what makes the cost acceptable — reconciliation reads a record it cannot decode as
/// `SuspendAndAsk`, so an older build STOPS rather than guesses.
///
/// ⚠️ "IT IS FREE TODAY BECAUSE NO ARCHIVE EXISTS" WAS TRUE UNTIL 2026-08-10, and it is dated
/// rather than quietly rewritten: `tests/frozen_bytes.rs` landed that day and THE FORMAT IS
/// FROZEN. A fourth variant would not move the frozen bytes — the three that exist keep their
/// indices and the files carry only those — but it makes every record carrying it undecodable
/// to a build that predates it, and that is a cost somebody pays instead of a hypothesis.
/// ⚠️ MEASURED, AND THE COMPILER GETS THERE FIRST: adding `#[n(3)] Amend` never reaches any
/// bench, because `crate::reconcile` matches this enum exhaustively and the LIBRARY stops with
/// `E0004`. So the level 1 guard is the reconciliation's match; the frozen bytes speak after it.
///
/// ⚠️ AND THE TWO EXISTING VARIANTS DID NOT MOVE, which was measured and not assumed:
/// `#[cbor(index_only)]` encodes a variant as its bare index, so `Intent` stays `00` and
/// `Outcome` stays `01`, and all of `tests/record_shape.rs` stayed green with the variant added.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cbor(index_only)]
pub enum RecordKind {
    #[n(0)]
    Intent,
    #[n(1)]
    Outcome,
    /// ⛔ A NOTE UPON A STEP, which neither opens a doubt nor closes one. It is what
    /// `Untrusted::promote` writes: a promotion touches nothing outside, so by ADR-0007 —
    /// "a step is AN INTERACTION WITH THE OUTSIDE WORLD" — it is not a step of its own, and
    /// the step it names is the caller's, which already owes an outcome.
    #[n(2)]
    Note,
    /// ⛔ A SENSOR VERDICT UPON THE STEP'S ARTEFACT (§6.4). Like `Note` it neither opens a doubt
    /// nor closes one — the doubt of ADR-0007 is about EFFECTS reaching the world, and a verdict
    /// is a fact recorded ABOUT a step, not an effect of it. ⚠️ AND THE EMPTY ARM IN
    /// `reconcile` WAS RE-MEASURED FOR THIS VARIANT rather than inherited from `Note`'s: see
    /// the arm itself.
    #[n(3)]
    Verdict,
}

/// How an effect may be reconciled after a crash (ADR-0007).
///
/// ⛔ THE CLASS IS A MANDATORY FIELD OF THE RECORD, and that is the point: §7.4.4 raised V5
/// to the compiler precisely so that "an effect without a declared class" IS NOT
/// EXPRESSIBLE. A defaulted class would put the decision back where the risk is — the
/// forgetfulness of whoever writes.
///
/// ⚠️ THE `Unrepeatable` DEFAULT OF ADR-0007 IS NOT GONE, BUT IT IS NOT REACHABLE HERE EITHER,
/// and the difference was measured on the types rather than assumed. `RecordV1::effect` is not
/// an `Option`, carries no `#[cbor(default)]`, and `EffectClass` implements no `Default`: a
/// record whose array is short does NOT decode to `Unrepeatable`, it decodes to
/// `RecordError::Malformed`. So there is no defaulting in this file, and reading one into it
/// would be reading a guarantee that is not here.
///
/// ⚠️ AND THE CASE THE DEFAULT IS FOR RUNS THE OTHER WAY ROUND. "Records written before the
/// class existed" is EMPTY BY CONSTRUCTION — V1 is the first version and the field has been
/// mandatory in it from the first byte ever written. The real case is a LATER version that
/// drops the field, and under ADR-0036 that is the ordinary shape of a field absent in another
/// version. ⚠️ THE FUTURE TENSE IS EXACT, AND TODAY NOTHING DEFAULTS ANYTHING: the version
/// that removes it will declare `Option<EffectClass>` with `#[cbor(default)]` and resolve
/// `None` to `Unrepeatable` — the safe reading, which suspends and asks — and until such a
/// version exists that mechanism is named here and implemented nowhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cbor(index_only)]
pub enum EffectClass {
    /// Ask the world what happened, then finish or re-plan.
    #[n(0)]
    Verifiable,
    /// Just run it again.
    #[n(1)]
    Idempotent,
    /// ⛔ Suspend and ask the user. Also what an undeclared class means.
    #[n(2)]
    Unrepeatable,
}

/// Whether the payload of this record crossed the untrusted boundary (I6, ADR-0014).
///
/// ⛔ THIS FIELD IS WHY IT IS HERE ON DAY ONE, and the reason is written where it was found:
/// road A4 of `crate::boundary`. Write external text into the journal, read it back as raw
/// bytes, and it comes out indistinguishable from an instruction — BYTES CARRY NO LABELS.
/// The record is where a label can live, and `boundary.rs` prices the alternative exactly:
/// "retrofitted later only by migrating the one irreproducible archive".
///
/// ⚠️ AND THE LIMIT IS THE TOKEN'S LIMIT, declared rather than discovered later: this proves
/// PROVENANCE, NOT CORRECTNESS (§6.3.2). Whoever writes a record can label it wrongly. What
/// it buys is that a reader can no longer LOSE the distinction, which is a different thing
/// from making it impossible to lie about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cbor(index_only)]
pub enum Trust {
    /// The payload may be used as an instruction.
    #[n(0)]
    Instruction,
    /// ⛔ The payload came from outside and stays outside (V20). Reading it back yields
    /// `Untrusted`, never a `String` that somebody may hand to the instruction channel.
    #[n(1)]
    Untrusted,
}

/// The structured detail a record of OUR OWN SPECIES carries (D20). ⛔ IT IS A TYPE AND NOT
/// OPAQUE BYTES, and the reason is ADR-0036 rule 6: the encoding lives in `kernel`. Opaque
/// bytes here would need a second decode nobody could perform without knowing the `kind` out of
/// band, which is the `payload` problem moved into a new box.
///
/// ⛔ AN UNKNOWN VARIANT DOES NOT DECODE, and that is what makes the field safe. Measured on
/// 2026-08-30 (P-15): `minicbor` answers `unknown enum variant N`, `Record::decode` maps every
/// error to `RecordError::Malformed`, and `reconcile` resolves that to `SuspendAndAsk`. A build
/// that does not know a species STOPS instead of guessing.
///
/// ⚠️ NO `#[cbor(..)]` ATTRIBUTE, AND THAT IS MEASURED RATHER THAN OVERLOOKED. `Record` above
/// carries an explicit `#[cbor(array)]` and this one does not, which reads as an asymmetry
/// between the only two data-carrying enums of the crate. Measured on 2026-09-01 against
/// `minicbor` 2.3.0 outside the repository: the two forms put the SAME bytes on the wire —
/// `82 00 81 82 f4 07` either way — because array is already the derive's default. The
/// asymmetry is therefore cosmetic, and it is written down so that nobody "harmonises" it
/// believing they are fixing a format. Errata `E47`.
///
/// ⚠️ IT PAIRS WITH `RecordKind` AND NOTHING AT LEVEL 1 HOLDS THE PAIR — declared, not defended.
/// A `RecordV1` with `kind: Verdict` and some other `Detail` is constructible, because
/// `RecordV1` is `pub` with public fields (finding AUD-050, registered and not taken). What
/// holds the pair is that ONE function per species builds the record, the way `Arbiter::issue`
/// is the only place that mints a `Grant` (§5.6). It is the shape of E25 in a new place, and it
/// is written down rather than discovered.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum Detail {
    /// A sensor verdict upon the step's artefact (§6.4).
    #[n(0)]
    Verdict(#[n(0)] VerdictDetail),
}

/// The structured half of a verdict (§6.4.1). ⛔ THE DETAIL TEXT IS NOT HERE: it is untrusted by
/// inheritance (ADR-0014) and travels in the record's `payload`, under the `trust` label that
/// exists to say so. What lives here is what is OURS and structured — the outcome, and the cost
/// the sensor reports having spent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cbor(array)]
pub struct VerdictDetail {
    /// `false` is `VerdictOutcome::Fail`. ⚠️ A `bool` AND NOT THE ENUM, and the asymmetry is
    /// deliberate: `sensor::VerdictOutcome` is a kernel type free to grow a third answer, while
    /// this one is on the WIRE and an index here never retires (rule 4 of §4.9.2). The day the
    /// enum grows, this field becomes a new optional index and the `bool` retires — which is
    /// exactly the discipline, and it is cheaper than reserving indices for answers nobody has.
    #[n(0)]
    pub passed: bool,
    /// ⚠️ `Millis` DOES NOT COME HERE, and the cost is declared: carrying it would give a time
    /// type the format's derives, and the conversion would still have to happen somewhere. A
    /// `u64` whose name says the unit is the smallest thing that holds, and the conversion lives
    /// in the one function that builds this record.
    #[n(1)]
    pub spent_millis: u64,
}

/// Version 1 of the durable record.
///
/// ⛔ EVERY FIELD CARRIES AN EXPLICIT INDEX, and the indices follow three rules that no
/// compiler enforces (§4.9.2): a new field is OPTIONAL and takes a NEW index; an index is
/// RETIRED AND NEVER REUSED — the gap stays; a non-additive change opens a NEW VERSION.
///
/// ⚠️ THE FUTURE TENSE WAS EXACT UNTIL 2026-08-10, AND IT IS DATED RATHER THAN REWRITTEN. This
/// paragraph said "TODAY NOTHING HOLDS THEM", and it was true: measured at that commit, moving a
/// variant onto a FREE index left the whole bench green — the derive renumbers encoding and
/// decoding together, so no round trip can see it. ⛔ `tests/frozen_bytes.rs` NOW EXISTS AND THE
/// THREE RULES ARE A CHECK. ⚠️ RECALL OF 2026-08-31: this passage said, AT THE PRESENT, that it
/// "freezes THREE records, which between them pin all EIGHT variant indices of the three enums
/// above", and both counts grew the day `Verdict`, `Detail` and index 5 arrived. DATED rather
/// than realigned, because the measurement belongs to 2026-08-10 — each of the eight renumbered
/// ONE AT A TIME, red eight times out of eight. How many are frozen NOW is `the_frozen_records()`
/// in that file, in one house. ⛔ AND THE `.cbor` FILES ARE NEVER REGENERATED: if they move it is
/// a NEW VERSION of the record, not an updated test.
///
/// ⚠️ `Clone` HAS NO CALLER IN THE CRATE AT THIS COMMIT, and is kept deliberately rather than
/// by inattention — the derive lists of `StepId` and `ClientId` are justified line by line and
/// this one owes the same. It is NOT removable by the #46 test, which asks what an outside
/// implementer is BLOCKED from doing: a record is the unit `journal` hands out and callers of
/// `read_back` will hold one while writing the next, and unlike `Ord` a missing `Clone` on a
/// struct with private-by-default construction elsewhere is not a one-line fix for them. It
/// costs nothing on the wire and nothing at run time unless called. ⚠️ NOT MEASURED BY
/// REMOVAL, and that is the honest state of it: `kernel` compiles without it today, so what is
/// written here is an argument and not a red.
#[derive(Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(array)]
pub struct RecordV1 {
    #[n(0)]
    pub kind: RecordKind,
    #[n(1)]
    pub effect: EffectClass,
    #[n(2)]
    pub trust: Trust,
    /// ⛔ THE BYTE-STRING ANNOTATION IS LOAD-BEARING, not decoration. Without it `minicbor`
    /// encodes a `Vec<u8>` as an ARRAY OF NUMBERS: it compiles, it round-trips, and it costs
    /// 1.91x — measured on 4096 B, 7813 against 4101. Gotcha #35.
    ///
    /// ⛔ THIS INDEX HOLDS THE CONTENT THE `trust` FIELD SPEAKS ABOUT, and after 2026-08-10 that
    /// is a rule and not a description. It is index 3 that the hand-written `Debug` below hides,
    /// so anything that may have come from outside belongs HERE and nowhere else in this struct.
    /// Putting untrusted content at any other index would print it in the first `{:?}` that
    /// reaches a log.
    #[n(3)]
    #[cbor(with = "minicbor::bytes")]
    pub payload: Vec<u8>,
    /// Why the record was written, in OUR words. ⛔ IT IS TEXT AND THE PAYLOAD IS BYTES, and the
    /// asymmetry is the point rather than an accident of typing: the payload is somebody else's
    /// and may be anything, this is ours and is always UTF-8. The two travel as different CBOR
    /// major types, so the distinction is in the archive and not only in the source.
    ///
    /// ⛔ IT ARRIVED WITH INDEX 4 ON 2026-08-10, AND IT IS WHY THE `trust` LABEL IS TRUE. Until
    /// then `Untrusted::promote` was to put THE REASON in `payload` and label it
    /// `Trust::Untrusted` — a label on the caller's own justification, which never crossed any
    /// boundary. `Trust`'s own doc says the label is about THE PAYLOAD, so that record would have
    /// carried a false statement in the one field whose whole job is to be true, in the one call
    /// site the boundary exists for. Splitting them lets the payload hold the external content
    /// and the label describe it.
    ///
    /// ⚠️ MANDATORY AND NOT `Option`, AND THAT WAS A DECISION WITH A DEADLINE ON IT — THE
    /// DEADLINE FELL ON 2026-08-10. Rule 3 of §4.9.2 — "a new field is OPTIONAL and takes a new
    /// index" — governs a field added to a version SOMEBODY HAS ALREADY WRITTEN, and when this
    /// one arrived V1 had never been written: there was no short array anywhere to decode, and an
    /// `Option` no reader would ever find `None` in would have been dead surface, the same
    /// argument that took the `Result` off `encode`. ⛔ THE EXEMPTION IS SPENT, AND THIS IS THE
    /// LAST MANDATORY FIELD V1 WILL EVER HAVE: `tests/frozen_bytes.rs` exists, so a field added
    /// to V1 MUST be `Option` at a NEW index, and the meaning of an
    /// index that already exists must never change (rule 4 — the reuse was measured, and it
    /// decodes to the WRONG SILENCE rather than to an error).
    ///
    /// ✅ AND THE ADDITIVE HALF WAS MEASURED, IN BOTH DIRECTIONS, because a rule nobody has run
    /// is a hope. ⛔ THE MEASURE IS NOT REPEATED HERE: it lives on `detail` below, on the field
    /// that really carries it (P-15, 2026-08-30), and a pointer cannot rot. The sentence that
    /// stood here named "the free index 5" — free on 2026-08-10, and `detail`'s since
    /// `4e4b725`, so the recipe it gave no longer compiles: `duplicate index numbers`.
    ///
    /// ⚠️ AND IT STAYS PRINTABLE, deliberately: the hand-written `Debug` hides index 3 and shows
    /// this one, because a failed assertion on a record has to say what the record was FOR.
    #[n(4)]
    pub reason: String,
    /// ⛔ OUR OWN STRUCTURED DATA, AND THE THIRD CONTENT BOX (D20). `payload` is somebody
    /// else's and `reason` is our prose; this is our STRUCTURE, and putting it in either of the
    /// other two was measured to be wrong — putting CBOR in `payload` reopens the defect that
    /// splitting `reason` shut on 2026-08-10, and `reason` is text.
    ///
    /// ⛔ OPTIONAL, AT A NEW INDEX — rule 3 of §4.9.2, and the exemption `reason` used is SPENT:
    /// `tests/frozen_bytes.rs` exists, so this is how every field added to V1 arrives from now
    /// on. ⚠️ THE `#[cbor(default)]` BELOW IS CONVENTION AND NOT THE MECHANISM, measured on
    /// 2026-08-31: removed, the whole workspace stays green and the older 21-byte records still
    /// decode to `None`, because `minicbor` already reads a missing `Option` field as `None`.
    /// It is kept — belt and braces cost nothing on an artefact that cannot be regenerated —
    /// but it is `Option` that carries the rule, and this doc used to say otherwise. Errata
    /// `E72`.
    ///
    /// ✅ ADDITIVE, MEASURED IN BOTH DIRECTIONS on 2026-08-30 (P-15): with `None` the three
    /// frozen records encode to the SAME 21 BYTES — `minicbor` truncates a trailing `None`
    /// instead of writing `null` — and with `Some` the array header moves `85` -> `86`.
    ///
    /// ⛔ AND THE FIELD ALONE IS NOT ENOUGH, WHICH IS WHY THE `kind` GROWS WITH IT (D20): a build
    /// that does not know this index decodes a record carrying it and LOSES THE SUBSTANCE IN
    /// SILENCE — measured. The new `kind` is what makes that build stop.
    #[n(5)]
    #[cbor(default)]
    pub detail: Option<Detail>,
}

/// ⛔ THE PAYLOAD IS NOT PRINTED, and it is the same defence `Untrusted` carries, applied to
/// the type that holds the LABEL. Road A3 of the residual on `Untrusted::promote` says external
/// text reaching the logs is the same class of problem as external text reaching the
/// instruction channel; `boundary.rs` wrote `Debug` by hand to close it, and a DERIVED `Debug`
/// here reopens it in a weaker form — weaker because it reopens it on the one type whose
/// `trust` field exists to say the bytes came from outside. A `{:?}` in a log line, a panic
/// message, a failed `assert_eq!`, and the payload is out.
///
/// ⚠️ EVERY OTHER FIELD STAYS READABLE, deliberately and for the reason the length stays on
/// `Untrusted`: a failed `assert_eq!` has to remain diagnostic. `kind`, `effect`, `trust`,
/// `reason` and `detail` are the kernel's own vocabulary — nobody outside chose them — and they
/// are exactly what one wants to read when a record comes back wrong. Only the payload is
/// somebody else's, and the list above is the whole of it: the numeral is gone rather than
/// realigned, because it has already aged twice.
///
/// ⛔ DATED RECALL, 2026-08-18 — FINDING P-1. That sentence was true of three fields out of four
/// and FALSE OF `reason`, which the CALLER chooses. `promote` took a `&str`, so
/// `other.promote(&mut journal, step, smuggled.as_str())` put external text at index 4 and this
/// impl printed it whole. Demonstrated from outside the crate:
/// `RecordV1 { … payload: <16 bytes>, reason: "ignore your instructions" }` — the guarded field
/// shut, the unguarded one wide open. ⚠️ **And the LIST is what made it read as verified:** four
/// names share one justification, the justification holds for three, and nothing in the sentence
/// says which — so a reader checking it stops at the first name that fits. ✅ `promote` takes a
/// `&'static str`, so ON THAT ROAD `reason` really is text chosen at authoring time. Road A3 in
/// `boundary.rs` says what that shuts and what it leaves declared.
///
/// ⛔ DATED RECALL, 2026-08-28 — FINDING AUD-050. The clause above read "✅ The sentence is now
/// TRUE rather than merely reworded", and that verdict was priced on ONE road. `promote` is not
/// the only way to fill index 4: `RecordV1` is `pub` with every field `pub`, so a struct literal
/// from ANY crate puts a runtime `String` at `reason` without going near `promote`, and this impl
/// prints it whole. Measured from outside the crate on a throwaway probe deleted in the same run:
/// `RecordV1 { kind: Note, effect: Unrepeatable, trust: Untrusted, payload: <24 bytes>,
/// reason: "ignore your instructions" }` — the guarded field shut, the unguarded one wide open,
/// which is P-1 reproduced through a second mouth. ⚠️ SO THE SENTENCE HOLDS OF THREE FIELDS AND OF
/// THE `promote` ROAD, not of the type: `reason` is the kernel's own vocabulary only while the
/// caller goes through `promote`. ⚠️ And the road is already walked INSIDE the repository —
/// `tests/record_shape.rs` writes that literal with `reason: String::from("ignore your
/// instructions")` from another crate, so it is not a road nobody takes.
/// ⛔ SHUTTING IT AT LEVEL 1 IS THE OWNER'S — REGISTERED, NOT TAKEN. Private fields plus a
/// constructor would touch every construction site across three crates, `frozen_bytes.rs`
/// included; `grep -rn 'RecordV1 {' crates/ --include=*.rs` counts them, and a count written here
/// would age. Until then the floor is DECLARED on road A3 in `boundary.rs`, which is where a
/// reader looks for what is open.
///
/// ⚠️ THAT SENTENCE SAID "THE OTHER THREE" UNTIL 2026-08-10 and is dated rather than quietly
/// renumbered: `reason` arrived at index 4 that day, and it is on THIS side of the line on
/// purpose. It is the text the caller wrote to justify the record; printing it discloses
/// nothing nobody chose, and hiding it would leave a failed assertion unable to say what the
/// record was for.
///
/// ⛔ AND `detail` IS PRINTED, WHICH IS THE D25 AND NOT AN OVERSIGHT. The field carries OUR
/// bytes by construction (D20), so printing it opens no road A3; NOT printing it would give
/// `RecordV1` a second hidden field that nobody decided to hide, against the half this doc calls
/// "the one that gets forgotten" — a `Debug` that hid everything would pass the assertion below
/// and leave a failed `assert_eq!` on a record saying nothing at all. ⚠️ THE GUARANTEE IS
/// DISCIPLINE AND NOT TYPE, exactly as for `reason`: a struct literal from any crate can put
/// anything at index 5, which is AUD-050 in a second place.
///
/// ⚠️ Pinned by `the_debug_of_a_record_does_not_print_the_payload`, because a closed road that
/// no test holds is a road that reopens the day somebody puts `Debug` back in the derive list
/// above, with the gate staying green.
impl fmt::Debug for RecordV1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RecordV1 {{ kind: {:?}, effect: {:?}, trust: {:?}, payload: <{} bytes>, \
             reason: {:?}, detail: {:?} }}",
            self.kind,
            self.effect,
            self.trust,
            self.payload.len(),
            self.reason,
            self.detail
        )
    }
}

/// The durable record. ⛔ A RECORD WITHOUT A VERSION IS NOT EXPRESSIBLE — rule 1 of §4.9.2,
/// held at level 1 by the type itself.
///
/// ⚠️ ONE VARIANT TODAY, AND IT IS NOT CEREMONY — written down because a YAGNI pass would
/// remove it and would be wrong. `minicbor` encodes an enum as a two-element array: variant
/// index, then value. So the version TRAVELS IN THE BYTES. Removing the enum would not
/// remove a level of indirection, it would remove a byte from the format — and that byte is
/// the whole of rule 1. Contrast with `Wakeup`, deleted at milestone 2 (errata E9): that one
/// wrapped a value and bought no error anywhere; this one is written to the archive.
///
/// ⚠️ `Debug` IS DERIVED HERE AND HAND-WRITTEN ON `RecordV1`, which is not an inconsistency:
/// the derive delegates to the inner impl, so `{:?}` on a whole record prints
/// `V1(RecordV1 { .. payload: <N bytes> })` and the payload stays shut. Nothing outside chose
/// the word `V1`. ⚠️ `Clone` carries the same note as `RecordV1`'s, and for the same reason.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(array)]
pub enum Record {
    #[n(0)]
    V1(#[n(0)] RecordV1),
}

/// What can go wrong DECODING a record.
///
/// ⚠️ Deliberately poor, and for the reason `JournalError` is: a rich error invites the
/// kernel to branch on the reason, and there is exactly one thing to do with a record that
/// will not decode.
///
/// ⚠️ THIS SENTENCE SAID "encoding or decoding" UNTIL 2026-08-10, and it is dated rather than
/// silently rewritten: `encode` returned a `Result` that could never be `Err`, and when that
/// signature went so did half of this type's job. Nothing is lost by the narrowing — decoding
/// is where the failure really lives, because the bytes come from an archive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordError {
    /// The bytes are not a record of any version this build knows.
    Malformed,
}

impl Record {
    /// Encodes to the bytes the `journal` port exchanges. ⛔ IT CANNOT FAIL, AND THE SIGNATURE
    /// SAYS SO — this returned `Result<Vec<u8>, RecordError>` until 2026-08-10, with the open
    /// question below beside it, and the question is now CLOSED.
    ///
    /// ⛔ WHY THE `Err` WAS UNREACHABLE, kept because it is the EVIDENCE that removing it is
    /// safe and not an opinion about it. Measured on the types rather than deduced:
    /// `minicbor::encode` returns `Result<(), minicbor::encode::Error<W::Error>>`, and `Vec<u8>`
    /// implements `minicbor::encode::Write` with `type Error = core::convert::Infallible`, so
    /// the WRITE road of that error is uninhabited here. Its other two roads — a message and a
    /// custom error — have exactly two producers in `minicbor` 2.3.0, `SystemTime` and a
    /// non-UTF-8 `Path`, and NEITHER IS IN THIS TYPE'S GRAPH: three `index_only` enums and a
    /// byte string. So the compiler could not see it, but nothing could produce it.
    ///
    /// ⛔ AND THE THREE REASONS FOR CLOSING IT NOW rather than at the version that first needs
    /// an error. The repository already holds this position and wrote it down for `Ipc::accept`:
    /// A `Result` THAT CAN NEVER BE `Err` IS DEAD SURFACE, of the kind that port pruned three
    /// derives and a getter for. `Untrusted::promote` will call this at task 7, and an `.expect`
    /// that cannot fire, sitting INSIDE THE CODE OF THE UNTRUSTED-DATA BOUNDARY, is debt and not
    /// prudence — a reader of that file has to establish that it cannot fire before trusting the
    /// line it is on. And the call sites are FEW today and will be many afterwards: the edit
    /// costs least now and most later.
    ///
    /// ⚠️ "FEW" WAS "TWO" FOR ONE COMMIT, AND THE COUNT WAS WRONG — dated rather than
    /// quietly fixed, because a wrong number attached to a right rule is exactly how gotcha #31
    /// works. Counted instead of remembered: at the commit that changed this signature there was
    /// ONE calling file, `tests/record_shape.rs`, with NINE call sites.
    /// `tests/compile_fail/record_without_version.rs` was counted as the second and is not a
    /// caller at all — it names `RecordV1::encode`, and its entire purpose is that no such
    /// inherent method exists. ✅ The error runs in the argument's FAVOUR: fewer call sites means
    /// the edit was cheaper than claimed, not dearer.
    ///
    /// ⚠️ THE PRICE IS DECLARED, and it is the one the open question named: the day a later
    /// version encodes something that CAN fail, this signature changes and every call site with
    /// it. That is a compiler error at each of them, which is the direction this repository
    /// accepts everywhere else — see `a_record_is_matched_exhaustively_and_that_is_the_point`.
    /// ⚠️ AND IT HIDES THE ASYMMETRY WITH `decode`, which really can fail: `RecordError` STAYS
    /// for that reason and only that one, so the type is now `decode`'s alone.
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        // ⚠️ THE RESULT IS DROPPED AND NOT `expect`ed, AND THAT IS THE POINT OF THE CHANGE: an
        // `.expect` here would only move the dead branch one level down, from many call sites to
        // one. ⛔ AND THE IMPOSSIBLE CASE IS CONTAINED RATHER THAN IGNORED — measured on the
        // shape of the failure, not hoped: an encoder that stopped early would leave `bytes`
        // TRUNCATED OR EMPTY, and `Record::decode` answers `Malformed` to both
        // (`bytes_that_are_not_a_record_decode_to_malformed` holds exactly those two inputs).
        // Reconciliation reads a record it cannot decode as `SuspendAndAsk`, so the archive
        // would stop the system rather than hand it a wrong answer — ADR-0007's own rule.
        let _ = minicbor::encode(self, &mut bytes);
        bytes
    }

    /// Decodes from the bytes the `journal` port hands back.
    ///
    /// # ⛔ IT REFUSES A RECORD WITH ANYTHING AFTER IT, and that is the contract rather than
    /// strictness
    ///
    /// `RecordError::Malformed` says "the bytes are not a record of any version this build
    /// knows", and a record followed by four bytes nobody wrote is not a record. Until
    /// 2026-08-27 this function answered `Ok` to exactly that and the tail VANISHED without a
    /// word — finding AUD-047, measured rather than reasoned:
    ///
    /// ```text
    /// Record::decode(&valid)           -> Ok(..)
    /// Record::decode(&valid ++ [0xFF; 4]) -> Ok(..)   <- four bytes nobody wrote, accepted
    /// ```
    ///
    /// ⚠️ THE CAUSE IS IN `minicbor` AND NOT HERE, which is why the remedy is a line of ours
    /// rather than a bug report: `minicbor::decode` is `Decoder::new(b).decode()` (source of
    /// 2.3.0, `src/lib.rs:173-178`) and a CBOR decoder is under no obligation to consume its
    /// whole input — it reads one item and stops. Asking whether it stopped AT THE END is the
    /// caller's job, and this is the caller.
    ///
    /// ⚠️ WHY IT MATTERS WHILE NO ARCHIVE CAN PRODUCE IT TODAY. Both implementations of the
    /// port delimit every entry by construction, so neither hands back a tail. But that is a
    /// property of the two IMPLEMENTATIONS, not of this TYPE, and it is this type whose doc a
    /// reader believes. `encode` above drops its `Result` on the grounds that a stopped encoder
    /// leaves bytes "TRUNCATED OR EMPTY" and that `decode` answers `Malformed` to both: the
    /// opposite direction — bytes in EXCESS read as a good record — stopped nothing and was
    /// declared nowhere. Reconciliation turns every `Err` into `SuspendAndAsk`, so an archive
    /// that cannot be read halts the system instead of guessing (ADR-0007); an archive that is
    /// read WRONG does neither.
    pub fn decode(bytes: &[u8]) -> Result<Self, RecordError> {
        let mut decoder = minicbor::Decoder::new(bytes);
        let record = decoder.decode().map_err(|_| RecordError::Malformed)?;

        // ⛔ THE WHOLE OF THE FIX, AND IT IS AN EQUALITY AND NOT A `>=`: a decoder that stopped
        // BEFORE the end left something unread, which is the same refusal for the same reason.
        if decoder.position() != bytes.len() {
            return Err(RecordError::Malformed);
        }

        Ok(record)
    }
}
