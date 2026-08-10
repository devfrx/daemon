//! The `journal` port (§4.1). The kernel declares what it needs; whoever provides it
//! stays outside.
//!
//! | Operation   | What it does                                                     |
//! |-------------|------------------------------------------------------------------|
//! | `intent`    | makes the INTENTION of a step durable, before the effect happens |
//! | `outcome`   | makes the OUTCOME durable, after                                 |
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
//! ⚠️ Milestone 2 declared the port, and milestone 3 has since built half of what this note
//! announced. The record, the version enum and the explicit indices EXIST — `crate::record`.
//! What is still ahead is THE FROZEN BYTES: constraint 14 of §11 makes them enter the
//! repository at the first record written, and they are deliberately the LAST thing of the
//! milestone, so that a real consumer has exercised the format before it is frozen.
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

use alloc::vec::Vec;

/// The identity of a step. It WILL BE progressive and assigned by the journal, NOT random:
/// §2.2 chose that over random identifiers because it is deterministic by construction and
/// readable in a trace.
///
/// ⚠️ THE FUTURE TENSE IS EXACT, AND TODAY NOTHING ASSIGNS ANYTHING. `new` is public, `intent`
/// RECEIVES the identity from its caller, and this port declares no operation that allocates
/// one. The allocator arrives with milestone 3, where the durable record gives it something to
/// be progressive in; until then "assigned by the journal" describes the design and not this
/// file, and saying so is cheaper than a reader deducing a guarantee that is not here.
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
    /// ⛔ An `outcome` arrived for a step that has no `intent`. This is V6 held by the port
    /// rather than by the caller: "nothing executes before the intent is durable" is the
    /// NATURE of a write-ahead journal, not a policy the kernel layers on top. A port that
    /// accepts it leaves the protocol resting on the diligence of whoever calls — the same
    /// reason `boundary_promotion.rs` requires that a refusing journal refuses the
    /// promotion too.
    OutOfOrder,
}

pub trait Journal {
    /// Makes the intention of a step durable. NOTHING EXECUTES BEFORE THE INTENT IS
    /// DURABLE (V6): the cost is two writes per step, accepted in ADR-0007.
    fn intent(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError>;

    /// Makes the outcome durable, after the effect happened.
    fn outcome(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError>;

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
