// §6.10.5 row 2: `kill` CONSUMES the worker, so instructing it afterwards is a use after
// move -- `E0382`. I2 held by the compiler rather than by discipline.
//
// ⛔ Names `kernel::` and declares no attributes of its own -- gotcha #39, WITH ONE MEASURED
// EXCEPTION. `Process` and `Worker` are TRAITS: a fully-qualified `impl kernel::…::Worker for
// FakeWorker` compiles the impl block without help, but `.start(..)`, `.instruct_one(..)` and
// `.kill()` below are METHOD-CALL syntax, and Rust resolves those only against traits already
// IN SCOPE -- measured, not assumed: without the import the compiler answered `E0599` on
// `FakeProcess.start(..)` itself, "method not found", the wrong defect entirely, before this
// case ever reached the one it exists to prove. `as _` brings in the methods and not the
// names, so the fully-qualified `impl` headers below stay exactly as written elsewhere.
//
// ⚠️ AND THE REST OF THE PREAMBLE BELOW IS REPEATED VERBATIM from
// `crates/kernel/tests/worker_tokens.rs`, with `kernel::` in place of its `use` statements:
// `trybuild` compiles every case in this directory as its own crate, and test code does not
// cross a crate boundary -- the same reason `Yield` is duplicated word for word between
// `executor_determinism.rs` and `dst_campaign.rs`.
use kernel::ports::process::{Process as _, Worker as _};

fn a_real_grant() -> kernel::arbiter::Grant {
    let mut arbiter = kernel::arbiter::Arbiter::new(
        kernel::parameters::Parameters::new(10_000, kernel::arbiter::Mib::new(16_384)),
        kernel::arbiter::VramPolicy::Remote(kernel::arbiter::RemotePolicy),
    );
    let kernel::arbiter::Admission::Granted(grant) = arbiter.admit(
        &kernel::arbiter::ResourceProfile {
            name: "asr-realtime",
            reserved_vram: kernel::arbiter::Mib::new(1_024),
            compute_class: kernel::arbiter::ComputeClass::Realtime,
            preemption: kernel::arbiter::Preemption::Never,
        },
        kernel::time::Millis::new(1_000_000),
        kernel::time::Monotonic::ORIGIN,
    ) else {
        panic!("1024 of 16384 fits");
    };
    grant
}

struct FakeWorker {
    next: u64,
}

impl kernel::ports::process::Worker for FakeWorker {
    fn instruct_one(
        &mut self,
        _frame: kernel::ports::process::Frame,
    ) -> Result<kernel::ports::process::SingleReceipt, kernel::ports::process::ProcessError> {
        self.next += 1;
        Ok(kernel::ports::process::SingleReceipt::new(self.next))
    }

    fn instruct_stream(
        &mut self,
        _frame: kernel::ports::process::Frame,
    ) -> Result<kernel::ports::process::StreamReceipt, kernel::ports::process::ProcessError> {
        self.next += 1;
        Ok(kernel::ports::process::StreamReceipt::new(self.next))
    }

    fn read_one(
        &mut self,
        receipt: kernel::ports::process::SingleReceipt,
    ) -> Result<kernel::ports::process::Frame, kernel::ports::process::ProcessError> {
        Ok(kernel::ports::process::Frame::new(
            receipt.id().to_le_bytes().to_vec(),
        ))
    }

    fn read_next(
        &mut self,
        _receipt: &mut kernel::ports::process::StreamReceipt,
    ) -> Result<Option<kernel::ports::process::Frame>, kernel::ports::process::ProcessError> {
        Ok(None)
    }

    fn close(
        &mut self,
        _receipt: kernel::ports::process::StreamReceipt,
    ) -> Result<(), kernel::ports::process::ProcessError> {
        Ok(())
    }

    fn kill(self) -> Result<(), kernel::ports::process::ProcessError> {
        Ok(())
    }
}

struct FakeProcess;

impl kernel::ports::process::Process for FakeProcess {
    type Handle = FakeWorker;

    fn start(
        &mut self,
        _grant: kernel::arbiter::Grant,
        _descriptor: kernel::ports::process::WorkerDescriptor,
    ) -> Result<Self::Handle, kernel::ports::process::ProcessError> {
        Ok(FakeWorker { next: 0 })
    }
}

fn a_started_worker() -> FakeWorker {
    FakeProcess
        .start(
            a_real_grant(),
            kernel::ports::process::WorkerDescriptor::new(b"asr.exe".to_vec()),
        )
        .expect("the fake always starts")
}

fn main() {
    let mut worker = a_started_worker();
    worker.kill().expect("killing is always lawful");
    let _ = worker.instruct_one(kernel::ports::process::Frame::new(b"too late".to_vec()));
}
