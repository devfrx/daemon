//! The in-memory `custody` (§2 of the GUI north star). One of the two implementations the
//! conformance suite runs against; the other is `redb` under `platform`.
//!
//! ⛔ THERE IS NO FALLING DOUBLE HERE, unlike `journal.rs`. Failing at an operation chosen by
//! the seed is fault injection, and nothing asks for it on this port yet: the campaign of
//! sub-project 2 substitutes the WORKING one. The day a campaign wants a
//! falling custody it wraps this type, exactly as `CrashingJournal` wraps `MemoryJournal`, and
//! for the same reason -- one archive, not two truths to hold in step.

use alloc::vec::Vec;

use kernel::ports::custody::{Custody, CustodyError, CustodyKey};

/// Keeps the packages in memory, one per key.
///
/// ⛔ A `Vec` OF PAIRS AND NOT AN `Option`, AND WITH ONE KEY THAT LOOKS LIKE CEREMONY. It is not.
/// An `Option<Vec<u8>>` would be BLIND TO THE KEY BY CONSTRUCTION -- `retrieve` would have
/// nothing to compare -- and the conformance suite CANNOT CATCH THAT with one variant, which is
/// measured and written in that file's head. So the shape is the guard: this type looks the key
/// up, and keeps on looking it up when a second one arrives.
///
/// ⚠️ AND NOT A `HashMap`, which is the rule of this crate rather than a preference:
/// `RandomState` is seeded per process and the iteration order is not reproducible in a
/// deterministic world -- gotcha #12. `MemoryJournal` keeps a `Vec` for the same reason.
pub struct MemoryCustody {
    packages: Vec<(CustodyKey, Vec<u8>)>,
}

// ⛔ NO `impl Default`, AND ITS ABSENCE IS THE DECISION -- the same one, for the same reason, as
// `MemoryJournal`, `SystemReactor` and `VirtualReactor`: nothing calls it, and this repository
// removes such items rather than keeping them for symmetry. `cargo clippy` asks for one
// (`new_without_default`); the warning is ACCEPTED and NOT silenced, because §7.4.3 gives clippy
// no voice in the gate and an `#[allow]` would hide the next occurrence too. The argument is
// written out once, in `crates/platform/src/reactor.rs`.
impl MemoryCustody {
    pub const fn new() -> Self {
        MemoryCustody {
            packages: Vec::new(),
        }
    }
}

impl Custody for MemoryCustody {
    fn keep(&mut self, key: CustodyKey, bytes: &[u8]) -> Result<(), CustodyError> {
        // ⛔ REPLACE IN PLACE AND NOT PUSH. Pushing would make `retrieve` depend on whether it
        // reads the first match or the last, which is the defect `AppendingCustody` wears in the
        // suite -- and the port's doc says `keep` replaces "whatever was there".
        match self.packages.iter_mut().find(|(kept, _)| *kept == key) {
            Some((_, package)) => {
                package.clear();
                package.extend_from_slice(bytes);
            }
            None => self.packages.push((key, bytes.to_vec())),
        }
        Ok(())
    }

    fn retrieve(&self, key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError> {
        Ok(self
            .packages
            .iter()
            .find(|(kept, _)| *kept == key)
            .map(|(_, package)| package.clone()))
    }
}
