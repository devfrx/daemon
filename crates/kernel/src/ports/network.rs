//! The `network` port: THE SINGLE EXIT POINT TOWARDS THE NETWORK (V25, Q20).
//!
//! ⚠️ The description of this family was widened on 2026-08-07 (F5, §2.3.1). It used to
//! say "exit towards the providers", while V25 and Q20 promise a single exit point
//! towards THE NETWORK -- and ADR-0017 has already decided a SECOND consumer, the opt-in
//! OTLP export. With the narrower description that consumer would have been born
//! OUTSIDE the single exit point, which is exactly what V25 forbids.
//!
//! ⚠️ Implementation STAGED (§0.4), and the allow-list of authorised crates is EMPTY. An
//! empty allow-list always passes, so the check is provable in one direction only; §7.4.2
//! declares that hole and it stays declared until the sub-project that turns the network
//! on.
//!
//! ⛔ NO TELEMETRY LEAVES THE MACHINE BY DEFAULT (ADR-0017). The OTLP export is opt-in,
//! with a destination chosen by the user.
//!
//! ⛔ DECLARED OPEN QUESTION, AND IT IS NOT RESOLVED HERE. Two sentences in this file do not
//! sit together, and both are written down rather than one of them quietly dropped:
//!
//! - the signature of `request` is SYNCHRONOUS AND BLOCKING -- it returns the whole answer,
//!   so somebody waited for it;
//! - the rule under it says readiness comes from the `reactor` and NOTHING WAITS INSIDE
//!   `network` (§2.4, no thread in the decision path).
//!
//! It matters because this kernel's executor is COOPERATIVE: a blocking call in the decision
//! path stops every other activity, and excluding exactly that is what the sub-project exists
//! for. ⚠️ The question is written and DELIBERATELY NOT ANSWERED. The signature comes from the
//! spec and this file has no authority to redraw it; inventing a shape now would freeze one
//! nobody has measured -- ADR-0009's rule, that a minimal contract can be widened and a rich
//! wrong one cannot, and the same reasoning that removed `Wakeup` from `reactor`. The two have
//! to be reconciled THE DAY THIS PORT GETS AN IMPLEMENTATION, and UNTIL THEN THERE IS NO
//! CALLER AND THEREFORE NOT YET A DEFECT: nothing in the kernel calls `request`, so nothing
//! blocks. A written open question is worth more here than an invented answer.
//!
//! ⚠️ What holds the signature meanwhile is `tests/ports_are_implementable.rs`: a fake, and
//! calls that exercise it. It proves the trait is implementable and callable, not that this is
//! the right shape -- and a fake that answers instantly says nothing about the question above.

use alloc::vec::Vec;

/// Where a request is going. Opaque to the kernel: parsing a URL is not a kernel
/// concern, and a structured type here would invite the kernel to reason about hosts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Endpoint(Vec<u8>);

impl Endpoint {
    pub fn new(raw: Vec<u8>) -> Self {
        Endpoint(raw)
    }

    /// The bytes back out, for `Path::as_bytes`'s reason: the privacy of a tuple-struct field
    /// is MODULE-scoped, so without this an implementation outside `kernel` could not reach the
    /// endpoint it is supposed to dial.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// ⚠️ Three variants and no implementation to produce any of them: the "no caller, no item"
/// rule does not reach an error vocabulary while the port itself is staged. The argument is
/// written out once, on `FilesystemError`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkError {
    /// No route, no connection, no answer.
    Unreachable,
    /// The peer answered with a failure.
    Refused,
    /// The wait reached its deadline. Distinct from `Unreachable`: a timeout says
    /// nothing about whether the effect happened, which is what makes the step IN DOUBT.
    TimedOut,
}

pub trait Network {
    /// Sends a request and waits for the whole answer.
    ///
    /// ⚠️ Readiness comes from the `reactor`, as for every other port: nothing waits
    /// inside `network`. §2.4 stays intact -- no thread in the decision path.
    ///
    /// ⛔ THAT RULE IS TRUE AND THIS SIGNATURE IS BLOCKING, and the two are reconciled by
    /// NOBODY YET: see the declared open question at the top of this module. It is not a
    /// defect today because no caller exists.
    fn request(&mut self, endpoint: &Endpoint, body: &[u8]) -> Result<Vec<u8>, NetworkError>;
}
