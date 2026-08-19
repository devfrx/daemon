//! The resource model of §5.1: VRAM as a type of its own, and compute as ORDERED LANES
//! rather than a number.

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mib(u64);

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
