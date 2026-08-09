// Catalogue §7.4.1 block C, row `V29 · §2.1`: monotonic time assigned to wall time must
// NOT compile.
//
// ⛔ This case NAMES `kernel::` and declares NO attributes of its own: gotcha #39. A case
// that redeclared `#![no_std]` would prove that the ban bites where it is declared, not
// that the kernel declares it.

use kernel::time::{Monotonic, WallTime};

/// Stands for any recording site: it takes wall time, because Q14 stamps the journal
/// with the time in the world.
fn stamp_the_record(_when: WallTime) {}

fn main() {
    let deadline = Monotonic::from_millis(5_000);
    // A decision's instant has nothing to do with the time in the world.
    stamp_the_record(deadline);
}
