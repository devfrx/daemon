//! The fake `reactor`: the virtual clock (§3.2).
//!
//! > THE CLOCK ADVANCES ONLY WHEN NOBODY CAN WORK. While a ready activity exists, time
//! > stands still; when none is, the reactor takes the clock TO THE FIRST FUTURE
//! > DEADLINE.
//!
//! ⚠️ Milestone 2 builds the clock and nothing else. Fault injection, the campaign and
//! the seed list are §3.3 to §3.5, milestone 4.

use kernel::ports::reactor::Reactor;
use kernel::time::{Monotonic, WallTime};

pub struct VirtualReactor {
    now: Monotonic,
    wall: WallTime,
}

impl VirtualReactor {
    pub const fn new() -> Self {
        VirtualReactor {
            now: Monotonic::ORIGIN,
            // A fixed origin, and it is deliberate: a virtual wall clock read from the
            // machine would be a source of divergence in a run that must be reproducible.
            wall: WallTime::from_millis_since_epoch(0),
        }
    }
}

impl Reactor for VirtualReactor {
    fn now(&self) -> Monotonic {
        self.now
    }

    fn wall_time(&self) -> WallTime {
        self.wall
    }

    fn wait_until(&mut self, deadline: Monotonic) -> Option<Monotonic> {
        // ⛔ STRICTLY IN THE FUTURE, and `None` otherwise. A null advance declared
        // successful is an infinite loop: the first draft took the minimum of ALL
        // registered deadlines, the minimum fell on an already-finished task, the clock
        // did not move, and the function said it had advanced anyway. §3.2.1, gotcha #19.
        if deadline <= self.now {
            return None;
        }
        let elapsed = deadline.saturating_since(self.now);
        self.now = deadline;
        // ⚠️ The two clocks move TOGETHER, by the same amount. A virtual wall clock frozen
        // at the origin while the monotonic one jumps twenty seconds would hand the
        // journal a stamp that contradicts its own ordering — and §2.1 forbids a decision
        // from reading this one, not the record from being consistent with it.
        self.wall = WallTime::from_millis_since_epoch(
            self.wall
                .as_millis_since_epoch()
                .saturating_add(elapsed.get()),
        );
        Some(self.now)
    }
}
