//! Catalogue §7.4.1 block C, row `V10`, and the strong partner of `sensor_modifies_the_artefact.rs`:
//! an implementor that DECLARES the artefact `&mut` does not match the trait.

use kernel::boundary::Untrusted;
use kernel::sensor::{CostClass, Sensor, Verdict, VerdictOutcome};
use kernel::time::Millis;

struct GreedySensor;

impl Sensor for GreedySensor {
    fn declared_cost(&self) -> CostClass {
        CostClass::Computational
    }

    fn observe(&self, artefact: &mut Untrusted) -> Verdict {
        // A sensor that asks for the right to rewrite what it is handed. This is the whole case.
        *artefact = Untrusted::new("something else".into());

        Verdict {
            outcome: VerdictOutcome::Pass,
            detail: Untrusted::new(String::new()),
            spent: Millis::new(0),
        }
    }
}

fn main() {}

// ⛔ IT FIRES AS `error`, THE STRONG SHAPE (gotcha #42), and that is why it exists beside
// `sensor_modifies_the_artefact.rs`, which can fire only as `mismatch`. Today the impl above does
// not match the trait -- `E0053` -- and the day V10 breaks the natural way, `Sensor::observe`
// handing the artefact over as `&mut Untrusted`, it matches, the file COMPILES, and trybuild
// answers "Expected test case to fail to compile, but it succeeded." -- a verdict a bulk
// `TRYBUILD=overwrite` cannot bless. Measured on 2026-10-14 in both directions, with the trait and
// `run_the_ring` widened to `&mut Untrusted`: this case `error`, its partner `mismatch`.
// ⚠️ RECALL OF 2026-10-14 -- audit of 2026-09-30, AUD-2141.
//
// ⛔ Names `kernel::` and declares no attributes of its own — gotcha #39.
//
// ⛔ THE NOTE IS DOWN HERE ON PURPOSE: the oracle quotes the line of the signature, so a paragraph
// added at the top would move the code and break it. Whoever writes here appends.
