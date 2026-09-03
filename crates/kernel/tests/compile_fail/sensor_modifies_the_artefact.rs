//! Catalogue §7.4.1 block C, row `V10` — a sensor that MODIFIES the artefact does not compile.
//! §6.4.2 hands it over by immutable reference, so the trait method cannot reach through it.

use kernel::boundary::Untrusted;
use kernel::sensor::{CostClass, Sensor, Verdict, VerdictOutcome};
use kernel::time::Millis;

struct MeddlingSensor;

impl Sensor for MeddlingSensor {
    fn declared_cost(&self) -> CostClass {
        CostClass::Computational
    }

    fn observe(&self, artefact: &Untrusted) -> Verdict {
        // A sensor that rewrites what it was handed. This is the whole case.
        *artefact = Untrusted::new("something else".into());

        Verdict {
            outcome: VerdictOutcome::Pass,
            detail: Untrusted::new(String::new()),
            spent: Millis::new(0),
        }
    }
}

fn main() {}

// ⛔ THE COUNTER-PROBE IS NOT HERE, AND IT ALREADY HAS A HOME: the catalogue row's "observing it
// and returning a verdict compiles" is `a_passing_sensor_writes_a_verdict_and_opens_nothing` in
// `tests/sensor_ring.rs`, which implements the trait and runs it. A copy here would be gotcha
// #49.
//
// ⛔ THE SECOND ROAD THE SAME MECHANISM SHUTS, measured rather than assumed: an `impl` that
// DECLARES `observe(&self, artefact: &mut Untrusted)` does not match the trait and fails with
// `error[E0053]`. It is not the case written here because the catalogue row is about the
// HANDING OVER, not about what an implementor may declare — but whoever widens this row should
// know the road exists and is already shut.
//
// ⛔ Names `kernel::` and declares no attributes of its own — gotcha #39.
//
// ⛔ THE NOTE IS DOWN HERE ON PURPOSE: the oracle quotes the line of the assignment, so a
// paragraph added at the top would move the code and break it. Whoever writes here appends.
