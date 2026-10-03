//! The ONE progressive counter of the core, and the seeding that puts it above the journal.
//!
//! ⛔ ONE COUNTER, NOT ONE PER CONSUMER, and that is the whole of this module. The doc of
//! `crate::ports::ipc::ClientId` states the defect it exists to prevent -- "two independent
//! counters that look identical are a divergence nothing would report" -- and names the counter
//! as "the one `journal` will allocate for `StepId`". It numbers clients -- the `ipc` transport
//! mints a `ClientId` from it -- and steps -- `crate::serving::Core` mints two `StepId` per
//! invocation -- FROM THE SAME INSTANCE: whoever builds the core builds the counter ONCE and hands
//! every consumer a `Progressive::share` of it. ⚠️ RECALL OF 2026-10-02 -- audit of 2026-09-30,
//! AUD-045, AUD-049, AUD-052, AUD-682: the daemon and the fake core now build one and share it.
//!
//! ⚠️ IT IS NOT A PORT AND NOT A PARAMETER. It holds no OS, it decides nothing, and it is
//! DELIVERED to whoever mints identities (ADR-0034): the transport in `platform` RECEIVES one at
//! construction, it does not build one. What ADR-0034 rules out is a decision reading a value
//! nobody handed it; a counter handed over at construction is the shape `Parameters` already has.
//!
//! ⛔ WHY THE SEEDING LIVES HERE AND NOT IN `daemon`. §7 of the milestone-2 design builds the
//! core's listening activity FROM OUTSIDE, in `gui/fake-core`, which cannot import a binary --
//! the same thing that section's own dedotto says about `build_the_arbiter`. A seeding written
//! inside `daemon/src/main.rs` would be COPIED by the fake core, and a copy of the wiring is
//! green on the day the two drift.
//!
//! ⛔ IT ADDS NO OPERATION TO THE `journal` PORT. Whether the allocator belongs inside that port
//! is open item 6 of §9 of the milestone-2 design, confirmed A by the owner on 2026-09-09
//! (decision 42 of the north star): it stays OUT until a second consumer asks for it. `replay`
//! is an operation the port already has.

use alloc::rc::Rc;
use core::cell::Cell;

use crate::ports::journal::{Journal, JournalError};

/// The core's progressive numbers: ONE sequence, however many hold it.
///
/// ⛔ THE NUMBER LIVES IN ONE CELL, AND EVERY HOLDER HOLDS A HANDLE ON IT. The arbiter keeps
/// `next_grant` and `next_ticket` as bare `u64` fields, and that shape is right THERE because each
/// of those counters has ONE consumer, the arbiter itself. This one has several -- the transport,
/// the core, and in `gui/fake-core` the faucet -- and a bare value handed to two of them is two
/// counters that look identical, with nothing to report it.
///
/// ⛔ SO THERE IS NO `Clone` AND NO `Copy`, and the absence is the point: the only road to a
/// second holder is `share`, which hands over THE SAME counter, and a second counter takes a
/// second `starting_at` -- written out, where a reader of the composition root sees it. The
/// derive list is empty for the reason `ClientId`'s is pruned: `PartialEq` would compare two
/// handles by the number they are at, which says nothing about whether they are one counter.
///
/// ⚠️ `Rc` AND `Cell`, NOT AN ATOMIC: the core runs ONE activity at a time on one thread
/// (§2.4.2, one decision at a time), so the cell is never touched by two at once. ⚠️ RECALL OF
/// 2026-10-03 -- audit of 2026-09-30, AUD-045. And the compiler holds it -- `Rc` is
/// not `Send`, so a `Progressive` cannot be carried to a second thread, where a `Cell` would be
/// the wrong tool.
///
/// ⚠️ The overflow reckoning is the arbiter's: a `u64` that counts clients and steps does not
/// reach its end, and a guard here would be a branch no caller can exercise.
pub struct Progressive {
    next: Rc<Cell<u64>>,
}

impl Progressive {
    /// A counter whose FIRST number is `first`.
    ///
    /// ⛔ IT TAKES THE FIRST NUMBER AND NOT THE LAST ONE USED, and the difference is not
    /// cosmetic: "the last one used" has no value on an empty journal, so every caller would
    /// have to invent one, and two callers inventing separately is the divergence this module
    /// exists to prevent.
    pub fn starting_at(first: u64) -> Self {
        Progressive {
            next: Rc::new(Cell::new(first)),
        }
    }

    /// The next number, and the counter moves on -- for EVERY holder of it.
    ///
    /// ⚠️ `&self` AND NOT `&mut self`: the number is shared by every holder, so an exclusive
    /// borrow of ONE handle would promise an exclusivity the counter does not have.
    pub fn take(&self) -> u64 {
        let value = self.next.get();
        self.next.set(value + 1);
        value
    }

    /// Another holder of THIS counter: what either one takes, the other never hands out.
    pub fn share(&self) -> Progressive {
        Progressive {
            next: Rc::clone(&self.next),
        }
    }
}

/// A counter seeded ABOVE every step the journal already holds.
///
/// ⚠️ ABOVE THE HIGHEST, NOT AFTER THE LAST. `replay` answers in the order the records were
/// written, and nothing promises that order is ascending: a journal whose last record is
/// `StepId(9)` may still hold `StepId(41)`.
///
/// ⚠️ AN EMPTY JOURNAL SEEDS IT AT ZERO, and that is stated rather than left to arithmetic: it
/// is the only case in which there is no highest number to stand above.
pub fn seeded_from<J: Journal>(journal: &J) -> Result<Progressive, JournalError> {
    let highest = journal.replay()?.iter().map(|(step, _)| step.get()).max();
    Ok(Progressive::starting_at(match highest {
        Some(highest) => highest + 1,
        None => 0,
    }))
}
