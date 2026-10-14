// §6.10.5 row 4: a single receipt is CONSUMED by the read. Reading twice is a use after
// move -- `E0382` -- so the consumption is a shape; that every byte that comes back is covered
// by a receipt is held at runtime, the receipt constructors being public (audit of 2026-09-30, AUD-405).
//
// ⛔ Names `kernel::` and declares no attributes of its own -- gotcha #39, WITH ONE MEASURED
// EXCEPTION. `Process` and `Worker` are TRAITS: a fully-qualified `impl kernel::…::Worker for
// FakeWorker` compiles the impl block without help, but `.start(..)`, `.instruct_one(..)` and
// `.read_one(..)` below are METHOD-CALL syntax, and Rust resolves those only against traits
// already IN SCOPE -- measured, not assumed: without the import the compiler answered `E0599`
// on `FakeProcess.start(..)` itself, "method not found", the wrong defect entirely, before
// this case ever reached the one it exists to prove. `as _` brings in the methods and not the
// names, so the fully-qualified `impl` headers below stay exactly as written elsewhere.
//
// ⚠️ AND THE REST OF THE PREAMBLE BELOW IS REPEATED VERBATIM from `worker_tokens.rs` in
// `crates/kernel/tests/`, with `kernel::` in place of its `use` statements and its `GUI_TICK`
// written out as the `kernel::time::Millis::new(0)` it stands for: `trybuild` compiles every case
// here as its own crate, and test code does not cross a crate boundary: what that bench
// defines, this case cannot import (⚠️ RECALL OF 2026-10-14 -- audit of 2026-09-30, AUD-794).
use kernel::ports::process::{Process as _, Worker as _};

fn a_real_grant() -> kernel::arbiter::Grant {
    let mut arbiter = kernel::arbiter::Arbiter::new(
        kernel::parameters::Parameters::new(10_000, kernel::arbiter::Mib::new(16_384), kernel::arbiter::ArbiterId::new(2), kernel::time::Millis::new(0)),
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
    grant: kernel::arbiter::Grant,
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

    fn kill(self) -> kernel::ports::process::Killed {
        kernel::ports::process::Killed {
            grant: self.grant,
            outcome: Ok(()),
        }
    }
}

struct FakeProcess;

impl kernel::ports::process::Process for FakeProcess {
    type Handle = FakeWorker;

    fn start(
        &mut self,
        grant: kernel::arbiter::Grant,
        _descriptor: kernel::ports::process::WorkerDescriptor,
    ) -> kernel::ports::process::Started<Self::Handle> {
        kernel::ports::process::Started::Running(FakeWorker { next: 0, grant })
    }
}

fn a_started_worker() -> FakeWorker {
    let kernel::ports::process::Started::Running(worker) = FakeProcess.start(
        a_real_grant(),
        kernel::ports::process::WorkerDescriptor::new(b"asr.exe".to_vec()),
    ) else {
        panic!("the fake always starts");
    };
    worker
}

fn main() {
    let mut worker = a_started_worker();
    let receipt = worker
        .instruct_one(kernel::ports::process::Frame::new(b"hello".to_vec()))
        .expect("answered");
    let _first = worker.read_one(receipt);
    let _second = worker.read_one(receipt);
}

// ⚠️ RECALL OF 2026-10-13 -- audit of 2026-09-30, AUD-1291: the arbiter of `a_real_grant` is
// `ArbiterId::new(2)`, as in the bench the preamble is copied from. The note sits down here
// because the oracle next door quotes the lines of `main`.
