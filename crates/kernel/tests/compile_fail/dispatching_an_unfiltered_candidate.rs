//! Catalogue §7.4.1 block B, row `Q13` — a candidate that has NOT been through the constraint
//! filter is not expressible as the argument of an execution. It is not that dispatching it is
//! forbidden: it cannot be SAID.

use kernel::gateway::{Candidate, dispatch};
use kernel::ports::journal::StepId;
use simulator::journal::MemoryJournal;

fn main() {
    let mut journal = MemoryJournal::new();

    let unfiltered = Candidate {
        model: "a-model",
        local: true,
        retains: false,
        price: 0,
    };

    // The whole case: `dispatch` wants a `Conforming`, and a `Candidate` is not one.
    let _ = dispatch(unfiltered, StepId::new(1), &mut journal);
}

// ⛔ IT REPORTS BY THE ORACLE AND NOT BY COMPILING, which is the WEAKER shape (gotcha #42), and
// `conforming_has_no_constructor.rs` beside it does the same on the roads that forge a token
// from outside — measured, and declared in that case. Whoever widens this row reads both, and
// reads every regeneration of either oracle by hand.
//
// ⛔ Names `kernel::` and declares no attributes of its own — gotcha #39.
