//! The two concepts of time, and they are two DISTINCT TYPES (§2.1).
//!
//! | Concept     | What it is for                                    | Who uses it   |
//! |-------------|---------------------------------------------------|---------------|
//! | `Monotonic` | deadlines, grant validity windows, grace, timeouts | THE DECISIONS |
//! | `WallTime`  | what time it is in the world — Q14, journal stamps | THE RECORD    |
//!
//! ⛔ No kernel decision depends on wall time. The system clock goes backwards — NTP,
//! daylight saving, the user changing it — and a run that died for that reason would be
//! an irreproducible defect, which is the class this sub-project exists to remove.
//!
//! They are two types and not two functions over one type, by the same mechanism that
//! separates `Instruction` from `Untrusted`. The ban has TWO rules, each proved in BOTH
//! directions in `tests/compile_fail/`: rule A — neither can be passed where the other
//! is expected — by `monotonic_as_wall.rs` and `wall_as_monotonic.rs`; rule B — no
//! `From`/`Into` path exists between them — by `no_conversion_from_monotonic_to_wall.rs`
//! and `no_conversion_from_wall_to_monotonic.rs`.
//!
//! ⛔ So do NOT add `impl From<Monotonic> for WallTime`, nor the reverse. Rule B's two
//! cases catch it by COMPILING, which trybuild reports as a failure outright — they do
//! not depend on their oracle to notice.
//!
//! Unit: the millisecond, everywhere (decision D1 of the milestone 2 plan).

/// A duration. Not an instant: it has no origin.
///
/// ⛔ IT CARRIES THE `bincode` DERIVES BECAUSE THIS TYPE CROSSES A PRIVATE CHANNEL, and it is
/// the only one of the four that no message names: it arrives through
/// `crate::arbiter::Preemption::After(Millis)`, inside `crate::wire::ipc::GrantRequest`. ⚠️ SO
/// NO DECLARATION UNDER `wire/` NAMES THIS TYPE -- `grep -rn "Millis" crates/kernel/src/wire/`
/// finds prose and no code. That is why this says where to look, instead of leaving two
/// derives with no findable caller.
///
/// ⛔ AND IT IS `Millis` AND NOT `Monotonic` OR `WallTime` THAT GOES, which is the whole of the
/// module doc above holding at the boundary: what the gui declares is a DURATION -- how long
/// it would take to hand the resource back -- and neither clock crosses. A decodable
/// `Monotonic` would be an instant on OUR scale minted by a peer, and a decodable `WallTime`
/// would put the clock this kernel refuses to decide with into a decision type.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, bincode::Encode, bincode::Decode,
)]
pub struct Millis(u64);

impl Millis {
    pub const fn new(value: u64) -> Self {
        Millis(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Monotonic time: never goes backwards. The origin is arbitrary and carries no
/// meaning — only differences do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Monotonic(u64);

impl Monotonic {
    /// The origin of the monotonic scale. `simulator` starts here; `platform` maps the
    /// operating system's own origin onto it.
    pub const ORIGIN: Monotonic = Monotonic(0);

    pub const fn from_millis(value: u64) -> Self {
        Monotonic(value)
    }

    /// Saturating and NOT wrapping: a deadline that wrapped would land in the PAST and
    /// fire immediately — a defect that hides itself.
    pub const fn saturating_add(self, delta: Millis) -> Self {
        Monotonic(self.0.saturating_add(delta.0))
    }

    /// The distance from an earlier instant, saturating to zero when `earlier` is in
    /// fact later. Also never wrapping, but the failure is the OPPOSITE one: a wrapped
    /// subtraction yields some 584 million years — a timeout that never fires.
    pub const fn saturating_since(self, earlier: Monotonic) -> Millis {
        Millis(self.0.saturating_sub(earlier.0))
    }
}

/// Wall time: what time it is in the world. ONLY the record reads it.
///
/// ⚠️ `Ord` orders the VALUES, not the events. Two stamps can compare in the opposite
/// order to the one in which they happened, because the clock between them may have
/// stepped. Sorting journal entries by `WallTime` gives a chronology, never a CAUSAL
/// order — for that, `Monotonic`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WallTime(u64);

impl WallTime {
    pub const fn from_millis_since_epoch(value: u64) -> Self {
        WallTime(value)
    }

    pub const fn as_millis_since_epoch(self) -> u64 {
        self.0
    }
}
