//! The `journal` port (§4.1). The kernel declares what it needs; whoever provides it
//! stays outside.
//!
//! | Operation   | What it does                                                     |
//! |-------------|------------------------------------------------------------------|
//! | `intent`    | makes the INTENTION of a step durable, before the effect happens |
//! | `outcome`   | makes the OUTCOME durable, after                                 |
//! | `note`      | appends a NOTE upon a step already open — neither of the two     |
//! | `read_back` | re-reads ONE step BY NAME, for reconciliation                     |
//! | `replay`    | re-reads EVERYTHING, in write order, to discover the names       |
//! | `prune`     | replaces a payload with a fingerprint and a size (ADR-0018)      |
//!
//! ⛔ THE PORT EXCHANGES BYTES, not typed records (ADR-0036). The encoding of the record
//! lives in `kernel` and §4.9 states its rule. Two consequences this table does not
//! show: the SIMULATOR EXCHANGES BYTES, so the DST campaign really exercises encoding
//! and decoding instead of going around them; and the durable form stays the kernel's
//! property.
//!
//! ⚠️ Milestone 2 declared the port, and milestone 3 has since built ALL of what this note
//! announced. The record, the version enum and the explicit indices EXIST — `crate::record` —
//! and so do THE FROZEN BYTES, in `crates/kernel/tests/frozen_bytes.rs`, since 2026-08-10.
//! ⚠️ THE PARAGRAPH SAID "what is still ahead is THE FROZEN BYTES" UNTIL THEN, and it is dated
//! rather than deleted: constraint 14 of §11 made them enter the repository at the first record
//! written, and they were deliberately the LAST thing of the milestone, so that a real consumer
//! had exercised the format before it was frozen. It has, and it is — and from here a new field
//! of `RecordV1` must be OPTIONAL with a NEW index, or it is a new version of the record.
//!
//! ⚠️ `replay` ARRIVED ON 2026-08-10, and the sentence this paragraph used to carry — "the
//! port is not finished, `replay` is not here" — is why it is dated instead of deleted. A port
//! grows when something needs it, and what needed this one is the RECONCILIATION: §4.3 collects
//! ALL the steps with an intent and no outcome, and `read_back` asks for a step BY NAME. After a
//! crash the kernel does not know the names — its memory is exactly what it lost — so with
//! `read_back` alone the set is not discoverable.
//!
//! ⚠️ AND THE SIGNATURE IS STILL A HYPOTHESIS while this line is being read: the reconciliation
//! is written NEXT, and it is the first caller that will put it under strain. If it turns out
//! cramped or insufficient there, it changes HERE — bending the caller to a signature decided
//! too early is the mistake this rule exists to prevent.
//!
//! ⚠️ `note` ARRIVED ON 2026-08-10 TOO, AND IT IS THE SAME RULE PAYING OUT A SECOND TIME. The
//! port did not grow because somebody foresaw a use: it grew because `Untrusted::promote` — the
//! first kernel code that writes a record at all — could not be written with the two operations
//! that were here. Both were tried and both were MEASURED to be wrong; the measurements are on
//! the operation itself. ⛔ AND THE PORT GROWING COST TEN IMPLEMENTATIONS A LINE EACH, counted
//! from the compiler and not estimated: `cargo build --workspace --all-targets` answered with
//! exactly ten `E0046` — one real, seven liars in `crates/kernel/tests/journal_contract.rs`, two
//! fakes in `crates/kernel/tests/boundary_promotion.rs`. That is the recurring price of decision
//! D6, and it is paid knowingly.
//!
//! ⚠️ THE TENSE IS PAST AND THE NUMBER IS A MEASUREMENT OF THAT MOMENT, not a description of
//! today: the operation brought a liar of its own and this file had ELEVEN implementations,
//! twelve once `redb` landed at task 8. A cost figure written in the present tense is a figure
//! that goes quietly wrong the first time the set grows — gotcha #31.
//!
//! ✅ TASK 8 LANDED ON 2026-08-10 AND THE TWELFTH IS `platform::journal::FileJournal`, counted
//! with `grep -rn "impl Journal for"` rather than by adding one to the sentence above. It is the
//! FIRST implementation outside a test that is not the in-memory double, and what it cost the
//! port is one item: `StepId::get`, without which no implementation outside `kernel` can write a
//! step's identity down.

use alloc::vec::Vec;

/// The identity of a step. It WILL BE progressive and assigned by the journal, NOT random:
/// §2.2 chose that over random identifiers because it is deterministic by construction and
/// readable in a trace.
///
/// ⚠️ THE FUTURE TENSE IS EXACT, AND TODAY NOTHING ASSIGNS ANYTHING. `new` is public, `intent`
/// RECEIVES the identity from its caller, and this port declares no operation that allocates
/// one; so "assigned by the journal" describes the design and not this file, and saying so is
/// cheaper than a reader deducing a guarantee that is not here.
///
/// ⚠️ RECALL OF 2026-08-21 — THIS SAID "the allocator arrives with milestone 3". Milestone 3
/// closed on 2026-08-10 WITH the durable record and WITHOUT the allocator, and milestone 4
/// closed after it: nothing under `crates/*/src/` constructs a `StepId` except
/// `FileJournal::replay`, which rebuilds one it has just read. The milestone is REMOVED rather
/// than moved to a later one — a date written in prose has nothing to fire it when it passes,
/// which is the shape `crate::reconcile` records for its own expired trigger. WHEN the
/// allocator arrives is the owner's: registered, not taken.
///
/// ⚠️ The derive list is short on purpose. No ordering and no `Hash`: nothing sorts or indexes
/// these yet, and `Hash` in particular would be an invitation in a crate where a negative case
/// forbids `HashMap` outright (gotcha #12). They come back the day something needs them, with
/// the caller that needs them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StepId(u64);

impl StepId {
    pub const fn new(value: u64) -> Self {
        StepId(value)
    }

    /// The number back out.
    ///
    /// ⚠️ IT CAME BACK ON 2026-08-10, WITH THE CALLER THAT NEEDED IT, and the day was named in
    /// advance: `CheckpointId`'s doc in `ports/filesystem.rs` says of its own getter that it
    /// "comes back the day the durable record of §4.9 has to write it down, with that caller".
    /// That day is task 8 of milestone 3 and that caller is `platform::journal::FileJournal`.
    ///
    /// ⛔ WITHOUT IT THE PORT IS UNIMPLEMENTABLE OUTSIDE `kernel`, which is what makes this an
    /// obligation rather than a convenience: the privacy of a tuple-struct field is
    /// MODULE-scoped, so a durable implementation could compare two `StepId` and never write
    /// one down. `Path::as_bytes` exists two files over for exactly this reason, and says so.
    /// ⚠️ `replay` needs no counterpart because `new` is already public — the way back in was
    /// always open, and only the way out was missing.
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// What can go wrong on the way to durability. Deliberately poor: a rich error type
/// invites the kernel to branch on the reason, and the reason belongs to whoever
/// implements the port.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JournalError {
    /// The write did not reach durable storage.
    NotDurable,
    /// The read found nothing under that identity.
    Missing,
    /// ⛔ An operation arrived OUT OF ORDER for this step, and there are THREE ways to do that.
    ///
    /// - an `outcome` for a step that has no `intent`;
    /// - a SECOND `intent` for a step that already carries one;
    /// - a `note` upon a step that has no `intent`.
    ///
    /// All three are V6 held by the port rather than by the caller: "nothing executes before the
    /// intent is durable" is the NATURE of a write-ahead journal, not a policy the kernel
    /// layers on top. A port that accepts any of them leaves the protocol resting on the
    /// diligence of whoever calls — the same reason `boundary_promotion.rs` requires that a
    /// refusing journal refuses the promotion too.
    ///
    /// ⚠️ THE SECOND AND THIRD HALVES BOTH ARRIVED ON 2026-08-10, and each WIDENED this variant
    /// instead of adding a neighbour. That is deliberate and it is this enum's own rule, three
    /// lines above: a rich error type invites the kernel to branch on the reason, and the reason
    /// belongs to whoever implements the port. "Out of order for this step" is one sentence that
    /// covers all three, and the kernel has nothing to decide differently between them.
    ///
    /// ⚠️ THIS DOC SAID "TWO WAYS" FOR ONE COMMIT, and is dated rather than silently renumbered:
    /// gotcha #31 is a count that ages inside a sentence nobody rereads because the sentence
    /// around it stayed true.
    ///
    /// ⛔ ONE INTENT PER STEP IS ADR-0007's OWN WORDING — "the intent of every step" — so a
    /// second one is outside the model rather than a case to discipline. It is held for BOTH
    /// implementations by `crates/kernel/tests/journal_contract.rs`, promise 6, because an
    /// implementation keyed on the identity of the step would otherwise diverge from the
    /// in-memory one with nothing going red.
    OutOfOrder,
    /// ⛔ A PRUNE WAS ASKED FOR A STEP THAT HAS AN INTENT AND NO OUTCOME. ADR-0018, not
    /// negotiable: a step in doubt is never prunable until it has been reconciled, because
    /// pruning it destroys the only trace of something that MAY have happened.
    ///
    /// ⛔ WHY A FOURTH VARIANT AND NOT A FOURTH WAY OF BEING `OutOfOrder`, since this type is
    /// deliberately poor and the second and third ways WIDENED that variant instead of joining
    /// it. Because `OutOfOrder` is defined by its INVARIANT and not by its words: all three of
    /// its ways are V6 — "nothing executes before the intent is durable" — and its own doc says
    /// exactly that. Pruning too early breaks no V6; it breaks ADR-0018's retention rule, which
    /// is a different invariant in a different ADR. Folding it in would make that doc's own
    /// sentence false, and a variant whose meaning is "one of two unrelated rules" is the rich
    /// error type this enum refuses by another route.
    ///
    /// ⛔ AND THE CALLER REALLY HAS TO TELL THEM APART, which is the test this enum sets before
    /// it grows. `OutOfOrder` says the caller BROKE the write-ahead protocol: nothing should ever
    /// do it, and a caller that meets it has a defect to surface. This one is ORDINARY — a
    /// retention sweep walking the archive meets steps still in doubt as a matter of course,
    /// skips them, and comes back after the next reconciliation. One is a bug, the other is
    /// control flow, and a single variant would force a normal sweep to look like a bug.
    ///
    /// ⚠️ NO VARIANT FOR "PRUNED ALREADY", and it is an absence with a reason: what a pruned
    /// step looks like afterwards is not settled — see `Journal::prune`.
    StepInDoubt,
}

pub trait Journal {
    /// Makes the intention of a step durable. NOTHING EXECUTES BEFORE THE INTENT IS
    /// DURABLE (V6): the cost is two writes per step, accepted in ADR-0007.
    ///
    /// ⛔ ONE INTENT PER STEP. A second one for a step that already carries an intent is
    /// `OutOfOrder` — see that variant for why it widened rather than gained a neighbour.
    fn intent(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError>;

    /// Makes the outcome durable, after the effect happened.
    fn outcome(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError>;

    /// Appends a NOTE upon a step that is already open. ⛔ IT IS NEITHER OF THE OTHER TWO, and
    /// that is the whole reason it exists rather than being folded into one of them.
    ///
    /// ⛔ WHY IT ARRIVED ON 2026-08-10, and it is a MEASUREMENT and not a preference. A note has
    /// one caller — `Untrusted::promote`, which records a crossing of the untrusted boundary
    /// onto THE CALLER'S STEP, because a promotion touches nothing outside and by ADR-0007 is
    /// therefore not a step of its own. Both existing operations were tried and both fail:
    ///
    /// - `intent` is REFUSED, because the caller's step already carries one — and even with
    ///   that guard removed, reconciliation reads a second `Intent` record for the step and
    ///   REPLACES the caller's resolution with the note's. Measured: a step the caller declared
    ///   `Idempotent` came back `SuspendAndAsk`.
    /// - `outcome` is accepted and takes the step OUT OF THE DOUBT although nothing has
    ///   executed. Measured: `steps_in_doubt` answered `[]` — a true doubt vanishing in
    ///   silence, the one failure ADR-0007 exists to prevent.
    ///
    /// ⛔ THE WRITE-AHEAD DISCIPLINE APPLIES: a note for a step with NO INTENT is `OutOfOrder`.
    /// A note is an annotation UPON something, and a step nobody opened is not something.
    ///
    /// ⛔ AND THERE IS DELIBERATELY NO LIMIT ON HOW MANY, which is the opposite answer from
    /// `intent`'s and has its own reason rather than being an omission. One intent per step is
    /// ADR-0007's own wording, so a second is outside the model; nothing says how many times one
    /// interaction with the world may consult external content, and a caller that promotes twice
    /// within one step is ordinary rather than suspect. ⚠️ Gotcha #46 does not apply here as it
    /// applied to `intent`: this operation is NOT declared in advance of its callers — it is
    /// declared BY one, and the one says many.
    ///
    /// ⚠️ AND WHAT THIS OPERATION DOES NOT BUY, declared rather than assumed from its existence:
    /// nothing here observable through the port distinguishes an implementation that stores a
    /// note in its own right from one that files it wherever it files outcomes. The port cannot
    /// see an implementation's bookkeeping. What the separate operation buys is that a CALLER
    /// cannot write a note through `intent` and trip its guard, and that the port's vocabulary
    /// matches the record's `RecordKind`. The semantics live in the record — see
    /// `crate::reconcile`, which neither opens nor closes a doubt on a `Note`.
    fn note(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError>;

    /// Re-reads on resume. Returns the bytes as they were written: decoding is the
    /// kernel's job, which is what keeps the durable form its property.
    fn read_back(&self, step: StepId) -> Result<Vec<u8>, JournalError>;

    /// Re-reads EVERYTHING, in write order, for reconciliation.
    ///
    /// ⛔ THE PORT DOES NOT KNOW WHAT "IN DOUBT" MEANS, and that is deliberate. It hands back
    /// what it has; the kernel decodes and computes the set (§4.3). An operation like
    /// `steps_in_doubt()` would move a decision of the kernel inside whoever implements the
    /// port, which is the opposite of how every other port here is built.
    ///
    /// ⚠️ WHY THIS EXISTS AT ALL, since `read_back` already reads: `read_back` asks for a step
    /// BY NAME, and after a crash the kernel does not know the names — its memory is exactly
    /// what it lost. With `read_back` alone the set of steps in doubt is not discoverable.
    ///
    /// ⛔ WRITE ORDER IS PART OF THE PROMISE, not a property of whichever container the
    /// implementation happens to use. Reconciliation computes the doubt by walking this
    /// sequence, and an arbitrary order gives it an arbitrary answer — SILENTLY, which is
    /// worse. It is held by `crates/kernel/tests/journal_contract.rs`, which is also what
    /// stops a key-value implementation from answering in key order.
    ///
    /// ⛔ DECLARED COST, and it is real: this loads the whole journal into memory. On a
    /// production archive it does not hold. The known remedy is a CHECKPOINT — a point past
    /// which everything is reconciled — and designing one now would freeze a mechanism no
    /// measurement has touched. It is closed by the first consumer that measures a large
    /// journal, not by this milestone.
    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError>;

    /// Replaces a payload with its fingerprint and size (ADR-0018).
    ///
    /// ⛔ Pruning is IRREVERSIBLE and must be declared: an absent payload and one that
    /// was never recorded must not be indistinguishable. And a step IN DOUBT is never
    /// prunable until it has been reconciled.
    fn prune(&mut self, step: StepId) -> Result<(), JournalError>;
}
