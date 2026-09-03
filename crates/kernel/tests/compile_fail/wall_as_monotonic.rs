// Catalogue §7.4.1 block C, row `V29 · §2.1`, the OTHER direction: wall time reaching a
// decision must NOT compile.
//
// ⛔ This is the direction that matters. A journal stamped with monotonic time is a
// wrong label; a DEADLINE derived from wall time is a run that dies when NTP steps the
// clock — the irreproducible defect this sub-project exists to remove. The sibling case
// `monotonic_as_wall.rs` guards the benign direction; without this one the ban is half a
// ban, and that was MEASURED: with only the sibling, adding
// `impl From<WallTime> for Monotonic` left the gate green on six checks out of six.
//
// ⛔ Names `kernel::` and declares no attributes of its own — gotcha #39.

use kernel::time::{Monotonic, WallTime};

/// Stands for any decision site: it takes monotonic time, because a deadline must never
/// depend on a clock that can go backwards.
fn expires_at(_deadline: Monotonic) {}

fn main() {
    let now_in_the_world = WallTime::from_millis_since_epoch(1_775_000_000_000);
    expires_at(now_in_the_world);
}
