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
fn main() {
    let descriptor = kernel::ports::process::WorkerDescriptor::new(b"asr.exe".to_vec());
    let _ = descriptor.instruct_one(kernel::ports::process::Frame::new(b"hello".to_vec()));
}
