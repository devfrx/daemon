//! The `journal` port (§4.1). The kernel declares what it needs; whoever provides it
//! stays outside.
//!
//! | Operation   | What it does                                                     |
//! |-------------|------------------------------------------------------------------|
//! | `intent`    | makes the INTENTION of a step durable, before the effect happens |
//! | `outcome`   | makes the OUTCOME durable, after                                 |
//! | `read_back` | re-reads on resume, for reconciliation                           |
//! | `prune`     | replaces a payload with a fingerprint and a size (ADR-0018)      |
//!
//! ⛔ THE PORT EXCHANGES BYTES, not typed records (ADR-0036). The encoding of the record
//! lives in `kernel` and §4.9 states its rule. Two consequences this table does not
//! show: the SIMULATOR EXCHANGES BYTES, so the DST campaign really exercises encoding
//! and decoding instead of going around them; and the durable form stays the kernel's
//! property.
//!
//! ⚠️ Milestone 2 declares the port. The record, the version enum, the explicit indices
//! and THE FROZEN BYTES are milestone 3 — constraint 14 of §11 makes the frozen bytes
//! enter the repository AT THE FIRST RECORD WRITTEN, and writing one here would freeze a
//! format §4.9 has not yet put to the test.

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

    /// Replaces a payload with its fingerprint and size (ADR-0018).
    ///
    /// ⛔ Pruning is IRREVERSIBLE and must be declared: an absent payload and one that
    /// was never recorded must not be indistinguishable. And a step IN DOUBT is never
    /// prunable until it has been reconciled.
    fn prune(&mut self, step: StepId) -> Result<(), JournalError>;
}
