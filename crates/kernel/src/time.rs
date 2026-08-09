//! The two concepts of time, and they are two DISTINCT TYPES (§2.1).
//!
//! | Concept     | What it is for                                    | Who uses it   |
//! |-------------|---------------------------------------------------|---------------|
//! | `Monotonic` | deadlines, grant validity windows, grace, timeouts | THE DECISIONS |
//! | `WallTime`  | what time it is in the world -- Q14, journal stamps| THE RECORD    |
//!
//! ⛔ No kernel decision depends on wall time. The system clock goes backwards -- NTP,
//! daylight saving, the user changing it -- and a run that died for that reason would be
//! an irreproducible defect, which is the class this sub-project exists to remove.
//!
//! They are two types and not two functions over one type: swapping them does not
//! compile, by the same mechanism that separates `Instruction` from `Untrusted`. The
//! negative test is `tests/compile_fail/monotonic_as_wall.rs`.
//!
//! Unit: the millisecond, everywhere (decision D1 of the milestone 2 plan).

/// A duration. Not an instant: it has no origin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Millis(u64);

impl Millis {
    pub const ZERO: Millis = Millis(0);

    pub const fn new(value: u64) -> Self {
        Millis(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Monotonic time: never goes backwards. The origin is arbitrary and carries no
/// meaning -- only differences do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Monotonic(u64);

impl Monotonic {
    /// The origin of the monotonic scale. `simulator` starts here; `platform` maps the
    /// operating system's own origin onto it.
    pub const ORIGIN: Monotonic = Monotonic(0);

    pub const fn from_millis(value: u64) -> Self {
        Monotonic(value)
    }

    pub const fn as_millis(self) -> u64 {
        self.0
    }

    /// Saturating and NOT wrapping: a deadline that wraps becomes a deadline in the
    /// past and fires immediately -- a defect that hides itself.
    pub const fn saturating_add(self, delta: Millis) -> Self {
        Monotonic(self.0.saturating_add(delta.0))
    }

    /// The distance from an earlier instant. Saturates to zero when `earlier` is in
    /// fact later, for the same reason.
    pub const fn saturating_since(self, earlier: Monotonic) -> Millis {
        Millis(self.0.saturating_sub(earlier.0))
    }
}

/// Wall time: what time it is in the world. ONLY the record reads it.
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
