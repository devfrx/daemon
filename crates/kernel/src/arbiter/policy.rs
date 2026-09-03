//! The two VRAM policies of ADR-0006. TWO OBJECTS WITH ONE INTERFACE, one active at a time,
//! chosen by the configuration profile -- NOT two arms of a conditional.
//!
//! ⛔ THE REASON IS IN ADR-0006 AND IT IS ABOUT DRIFT: an `if` on the origin of the
//! inference, planted in the middle of the admission, spreads invisibly as the admission
//! grows. Duplication between two objects is VISIBLE and BOUNDED; the drift of a conditional
//! is invisible and diffuse.
//!
//! ⛔ AND AT MILESTONE 5 THEY ARE NOT EMPTY SHELLS, which was the open question. The
//! difference is ONE DECISION inside the admission path -- "a request does not fit. Can room
//! be made?" -- and no model is needed to answer it: evicting a resident IS revoking a
//! preemptible grant, a mechanism §6 built anyway.
//!
//! ⚠️ AND A TRAIT HERE IS NOT THE SHAPE THE MODULE COMMENT OF `mod.rs` REFUSES. That one
//! refuses a trait `Arbiter` with two implementations, "an abstraction with no second
//! implementor". `MakeRoom` HAS two implementors, and they answer differently: that is the
//! difference between an abstraction invented for testability and one the domain asked for.

/// What the admission asks the active policy.
///
/// ⛔ ONE QUESTION AND NOT A FAMILY OF THEM. A rich interface here would invite the
/// admission to branch on the policy in several places, which is the conditional ADR-0006
/// refuses, arriving by another road.
pub trait MakeRoom {
    /// A request does not fit. May the arbiter take resources back to seat it?
    fn may_make_room(&self) -> bool;

    /// What this policy is called, for the journalled transition of §5.4.
    fn name(&self) -> &'static str;
}

/// The DEFAULT (ADR-0006): OpenRouter, VRAM free. Nothing is revoked to make room -- the
/// request queues, or it is refused.
pub struct RemotePolicy;

impl MakeRoom for RemotePolicy {
    fn may_make_room(&self) -> bool {
        false
    }

    fn name(&self) -> &'static str {
        "remote"
    }
}

/// Local inference: the machine's VRAM is the working set, so seating a new request may mean
/// taking resources back from preemptible grants in lower lanes.
pub struct LocalPolicy;

impl MakeRoom for LocalPolicy {
    fn may_make_room(&self) -> bool {
        true
    }

    fn name(&self) -> &'static str {
        "local"
    }
}

/// The policy the arbiter was BUILT WITH. ⛔ ONE, AND THE TYPE SAYS SO: "two active
/// policies" is NOT EXPRESSIBLE, which is `V3` at level 1 rather than a test at level 2
/// (§7.4.1 C, and §5.4 said this rule would rise to the compiler).
pub enum VramPolicy {
    Remote(RemotePolicy),
    Local(LocalPolicy),
}

impl MakeRoom for VramPolicy {
    fn may_make_room(&self) -> bool {
        match self {
            VramPolicy::Remote(policy) => policy.may_make_room(),
            VramPolicy::Local(policy) => policy.may_make_room(),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            VramPolicy::Remote(policy) => policy.name(),
            VramPolicy::Local(policy) => policy.name(),
        }
    }
}
