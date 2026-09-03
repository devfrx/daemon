// §6.10.5 row 1: you talk to a worker ONLY with the object `start` returned. What you have
// BEFORE starting -- the grant and the descriptor -- is not it.
//
// ⛔ Names `kernel::` and declares no attributes of its own -- gotcha #39.
//
// ⚠️ AND IT DOES NOT REPEAT THE PREAMBLE that its three siblings in this directory share
// (`a_real_grant`, `FakeWorker`, `FakeProcess`, `a_started_worker`): this `main` needs neither
// a `Grant` nor a `Worker`, only `WorkerDescriptor` and `Frame`. Pulling the rest in would
// leave four items DEAD -- `dead_code` warnings inside a case whose oracle is a text file, and
// this workspace has no `#[allow]` to silence them with.
//
// ⛔ THE IMPORT BELOW IS DELIBERATE THOUGH NOTHING IN THIS `main` RESOLVES THROUGH IT, and it
// is what keeps the red non-rewritable -- 2026-08-24. Without it, the regression this case
// defends (`impl Worker for WorkerDescriptor`) would leave the trait OUT OF SCOPE: the file
// would go on not compiling, and `trybuild` would answer `mismatch`, the one verdict a
// blanket `TRYBUILD=overwrite` can silence (gotcha #25 and #42). With it, that same
// regression makes this file COMPILE, and the answer becomes `error`, which no regeneration
// can absorb. ⚠️ AND IT COSTS NO WARNING, measured with a `rustc` probe outside the
// repository rather than reasoned: trait in scope and NOT implemented gives this oracle
// unchanged but for the line number -- rustc aborts on `E0599` before `unused_imports` is
// ever reached -- and trait in scope AND implemented exits 0, where the import is used.
use kernel::ports::process::Worker as _;

fn main() {
    let descriptor = kernel::ports::process::WorkerDescriptor::new(b"asr.exe".to_vec());
    let _ = descriptor.instruct_one(kernel::ports::process::Frame::new(b"hello".to_vec()));
}
