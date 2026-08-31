//! The resource model of §5.1: VRAM as a type of its own, and compute as ORDERED LANES
//! rather than a number.

use crate::time::Millis;
use minicbor::{Decode, Encode};

/// VRAM, in whole MiB.
///
/// ⛔ A TYPE OF ITS OWN AND NOT A BARE INTEGER, for the reason §5.1 gives in one line:
/// swapping MiB for milliseconds MUST NOT COMPILE. It is the same mechanism that separates
/// `Instruction` from `Untrusted` and `Monotonic` from `WallTime`, and it is held by four
/// cases in `tests/compile_fail/` -- two for "neither is passable for the other", two for
/// "no `From` path exists".
///
/// ⛔ WHOLE MiB, AND THE QUANTISATION IS THE POINT. The resource is quantised; an integer
/// removes every question about rounding, and a rounding question inside a deterministic
/// decision path is debt (§5.1).
///
/// ⛔ AND IT IS A WIRE TYPE AS WELL AS A DECISION TYPE, from milestone 6 -- the FIRST
/// `minicbor` derive outside `record.rs`. The cost is declared here rather than left to be
/// found: `minicbor` now serves two artefacts with opposite requirements (§6.10.7), and a
/// change made for `crate::wire::worker` reaches a type the arbiter decides on.
///
/// ⛔ THE ROAD THAT AVOIDED IT WAS REFUSED ON THE MERITS AND NOT ON COST: putting a bare
/// `u64` in the message is exactly the case those four `compile_fail` cases exist to remove
/// -- an integer that comes back off the wire and that nobody has to call MiB.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Encode, Decode)]
pub struct Mib(#[n(0)] u64);

impl Mib {
    /// No VRAM at all. It is the identity of `saturating_add` and the floor of
    /// `saturating_sub`, and having it named keeps `Mib::new(0)` out of the arithmetic.
    pub const ZERO: Mib = Mib(0);

    pub const fn new(value: u64) -> Self {
        Mib(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }

    /// Saturating and NOT wrapping, and the DIRECTION is what makes this safe rather than
    /// merely defined.
    ///
    /// ⛔ An overflow saturates to `u64::MAX`, which is GREATER than any ceiling, so the
    /// request is REFUSED. A wrapping add would yield a SMALLER number and produce
    /// over-admission -- Q2 giving way in silence, which is the one failure the whole
    /// arbiter exists to prevent. It is the same argument already written beside
    /// `Monotonic::saturating_add`, landing on the same side.
    pub const fn saturating_add(self, other: Mib) -> Self {
        Mib(self.0.saturating_add(other.0))
    }

    /// Saturating to zero. A budget cannot go negative, and a wrapping subtraction would
    /// yield an enormous free budget -- the same over-admission by the other road.
    pub const fn saturating_sub(self, other: Mib) -> Self {
        Mib(self.0.saturating_sub(other.0))
    }
}

/// The three compute lanes of §5.1 and design/02. NOT a number: contention on compute is
/// governed by ORDER plus a "reduce your footprint" signal, never by an amount.
///
/// ⛔ `Ord` IS WRITTEN BY HAND, FROM AN EXPLICIT KEY, and that is the decision rather than
/// ceremony. A DERIVED `Ord` follows the order in which the variants are DECLARED, so
/// reordering them would change the arbiter's priorities and NOTHING WOULD GO RED.
/// Removing the trap beats watching it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComputeClass {
    /// Wake word, VAD, STT, TTS. Never preempted, and its VRAM is held by a PERMANENT
    /// GRANT rather than subtracted from the budget -- a subtraction without a holder
    /// leaves I2 false for that consumer (ADR-0033, gotcha #4).
    Realtime,
    /// Chat and the foreground agent. Served before `Batch`.
    Interactive,
    /// 3D render, indexing, background runs. May wait indefinitely.
    Batch,
}

impl ComputeClass {
    /// The order, stated ONCE and in one place. Lower is served first.
    pub const fn priority(self) -> u8 {
        match self {
            ComputeClass::Realtime => 0,
            ComputeClass::Interactive => 1,
            ComputeClass::Batch => 2,
        }
    }
}

impl PartialOrd for ComputeClass {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ComputeClass {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.priority().cmp(&other.priority())
    }
}

/// Whether the arbiter may take the resource back, and -- when it may -- how long the
/// holder gets to hand it over.
///
/// ⛔ NOT A BOOLEAN, AND THE GRACE TIME LIVES INSIDE THE VARIANT. §5.3 point 3 wants
/// `Revoking` to be NOT REPRESENTABLE for a non-preemptible profile -- "not constructible",
/// not "checked at runtime". A boolean cannot do that. This enum makes TWO illegal states
/// disappear together: a non-preemptible profile that carries a grace time, and a
/// preemptible one that has none.
///
/// ⚠️ DIVERGENCE FROM THE LETTER OF §5.2, DECLARED. That table lists TWO fields --
/// `preemptible: boolean` and `release_grace: duration` -- and this is ONE. The spirit of
/// §5.3 point 3 is what forces it; the letter of §5.2 is what it costs. Registered in the
/// errata of the milestone 5 plan so the owner can overturn it seeing it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Preemption {
    /// The arbiter never takes it back. ⚠️ NOT "permanent": a job that cannot be
    /// interrupted still FINISHES and releases. Permanence is not a type -- it is "nobody
    /// calls release".
    Never,
    /// The arbiter may take it back, and the holder gets this long to comply.
    After(Millis),
}

impl Preemption {
    /// The grace time, when there is one. `None` is not a missing value: it is the
    /// statement that this profile is never revoked.
    pub const fn grace(self) -> Option<Millis> {
        match self {
            Preemption::Never => None,
            Preemption::After(grace) => Some(grace),
        }
    }
}

/// What the arbiter RECEIVES in order to decide (§5.2). Named and versioned: design/02
/// makes the version part of the NAME -- `trellis2-512-lean`, `trellis2-1024` -- because a
/// kind of work does not produce a number but a CURVE, and the useful points of that curve
/// become distinct named profiles.
///
/// ⛔ `name` IS `&'static str` AND NOT `String`, and the reason is finding P-1, closed on
/// 2026-08-18: a profile name is chosen when the code is written, so it is a literal in the
/// binary, and runtime text -- which is where untrusted content lives -- cannot get here at
/// all. A `String` would buy nothing and reopen a road that was measured shut.
///
/// ⛔ AND `cold_start` IS NOT HERE. §5.2.1: design/02 says it is "used to warn the user, not
/// to decide", which was a written rule and therefore a recommendation. It lives in
/// `WorkDescriptor`, which the admission does not receive, so a decision that wanted to read
/// it HAS NO WAY.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceProfile {
    pub name: &'static str,
    /// The reservation DECLARED by the requester, not measured after the fact. A
    /// systematically wrong reservation is a defect of the PROFILE and not an incident
    /// (ADR-0005).
    pub reserved_vram: Mib,
    pub compute_class: ComputeClass,
    pub preemption: Preemption,
}

/// What goes to the PRESENTATION side, and never to the admission (§5.2.1).
///
/// ⚠️ IT IS TIED TO ITS PROFILE BY A NAME AND BY NOTHING ELSE. §5.2.1 accepted the cost in
/// those words -- "two structures instead of one, and one more place to keep them aligned"
/// -- and a shared type would put `cold_start` back within reach of the decision, which is
/// the whole thing this split exists to prevent.
///
/// ⚠️ AND THE PLACE THAT KEEPS THEM ALIGNED DOES NOT EXIST YET, written here because this is
/// where whoever builds the first descriptor will look: nothing in the product builds one, so
/// there is no pair to keep aligned and nothing that could hold it. Declared 2026-08-27,
/// finding AUD-020 -- the paragraph that measures it sits on
/// `a_descriptor_names_the_profile_it_describes` in `crates/kernel/tests/arbiter_resource.rs`.
///
/// ⚠️ NOT TO BE CONFUSED WITH `crate::ports::process::WorkerDescriptor`, which is one letter
/// away and is a different thing: that one is WHAT TO START, opaque bytes for the OS. No
/// file imports both unqualified.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkDescriptor {
    pub profile_name: &'static str,
    /// How long the work takes to become useful when it starts cold. ⛔ FOR WARNING THE
    /// USER, NEVER FOR DECIDING.
    pub cold_start: Millis,
}
