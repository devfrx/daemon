//! The counter-probes of the four rows of §6.10.5 -- the "must stay green" half.
//!
//! ⛔ RECALL OF 2026-08-21, AUDIT FINDING P-2. The paragraph that used to open this file said
//! "until milestone 5 they could not be written at all: all four need to OBTAIN a `Worker`, a
//! `Worker` comes only from `start(grant, ..)`, and nobody issued grants" -- it was FALSE and
//! is TAKEN OUT rather than reworded. A `Worker` is obtained by IMPLEMENTING THE TRAIT, with no
//! grant anywhere: `tests/ports_are_implementable.rs` has done exactly that since milestone 2,
//! with `ScriptedWorker`. ⛔ THE TRUE REASON the four rows stayed uncovered is that the "must
//! fire" direction was missing (§7.1.1 rule 3): a rule proved in one direction only is not
//! admissible, and this file with its four `compile_fail` siblings is what writes the other
//! direction. The verbal record is in `docs/porta-di-qualita.md`, section "P-2".
//!
//! ⛔ AND THE GRANT HERE IS A REAL ONE, from a real admission. A test-only constructor was
//! weighed and refused in `docs/porta-di-qualita.md`: it would create the SECOND way of
//! obtaining a grant that §5.6 exists to take away from the compiler. The bench goes through
//! the admission like everybody else.

use kernel::arbiter::{
    Admission, Arbiter, ComputeClass, Grant, Mib, Preemption, RemotePolicy, ResourceProfile,
    VramPolicy,
};
use kernel::parameters::Parameters;
use kernel::ports::process::{
    Frame, Process, ProcessError, SingleReceipt, StreamReceipt, Worker, WorkerDescriptor,
};
use kernel::time::{Millis, Monotonic};

/// A grant obtained the only way there is.
fn a_real_grant() -> Grant {
    let mut arbiter = Arbiter::new(
        Parameters::new(10_000, Mib::new(16_384)),
        VramPolicy::Remote(RemotePolicy),
    );
    let Admission::Granted(grant) = arbiter.admit(
        &ResourceProfile {
            name: "asr-realtime",
            reserved_vram: Mib::new(1_024),
            compute_class: ComputeClass::Realtime,
            preemption: Preemption::Never,
        },
        Millis::new(1_000_000),
        Monotonic::ORIGIN,
    ) else {
        panic!("1024 of 16384 fits");
    };
    grant
}

/// The smallest worker that answers. ⚠️ IT IS A FAKE AND IT IS ALLOWED TO BE POOR: what this
/// file tests is the SHAPE OF THE TOKENS, not a worker channel -- that is the conformance
/// suite, and it needs two implementations and a real worker (milestone 6).
struct FakeWorker {
    next: u64,
}

impl Worker for FakeWorker {
    fn instruct_one(&mut self, _frame: Frame) -> Result<SingleReceipt, ProcessError> {
        self.next += 1;
        Ok(SingleReceipt::new(self.next))
    }

    fn instruct_stream(&mut self, _frame: Frame) -> Result<StreamReceipt, ProcessError> {
        self.next += 1;
        Ok(StreamReceipt::new(self.next))
    }

    fn read_one(&mut self, receipt: SingleReceipt) -> Result<Frame, ProcessError> {
        Ok(Frame::new(receipt.id().to_le_bytes().to_vec()))
    }

    fn read_next(&mut self, _receipt: &mut StreamReceipt) -> Result<Option<Frame>, ProcessError> {
        Ok(None)
    }

    fn close(&mut self, _receipt: StreamReceipt) -> Result<(), ProcessError> {
        Ok(())
    }

    fn kill(self) -> Result<(), ProcessError> {
        Ok(())
    }
}

struct FakeProcess;

impl Process for FakeProcess {
    type Handle = FakeWorker;

    fn start(
        &mut self,
        _grant: Grant,
        _descriptor: WorkerDescriptor,
    ) -> Result<Self::Handle, ProcessError> {
        Ok(FakeWorker { next: 0 })
    }
}

fn a_started_worker() -> FakeWorker {
    FakeProcess
        .start(a_real_grant(), WorkerDescriptor::new(b"asr.exe".to_vec()))
        .expect("the fake always starts")
}

/// §6.10.5 row 1, counter-probe: WITH the handle, talking compiles and works.
#[test]
fn with_the_handle_the_worker_can_be_instructed() {
    let mut worker = a_started_worker();
    let receipt = worker
        .instruct_one(Frame::new(b"hello".to_vec()))
        .expect("the fake answers");
    assert_eq!(receipt.id(), 1);
}

/// §6.10.5 row 2, counter-probe: instructing BEFORE the kill compiles.
#[test]
fn instructing_before_the_kill_compiles() {
    let mut worker = a_started_worker();
    let _ = worker
        .instruct_one(Frame::new(b"hello".to_vec()))
        .expect("answered");
    worker.kill().expect("killing is always lawful");
}

/// §6.10.5 rows 3 and 4, counter-probe: reading ONCE, with the receipt, compiles.
#[test]
fn reading_once_with_the_receipt_compiles() {
    let mut worker = a_started_worker();
    let receipt = worker
        .instruct_one(Frame::new(b"hello".to_vec()))
        .expect("answered");
    let expected = receipt.id();
    let answer = worker.read_one(receipt).expect("answered");
    assert_eq!(answer.as_bytes(), &expected.to_le_bytes());
}

/// ⛔ AND THE ONE THAT SAYS THE GRANT IS SPENT: `start` CONSUMES it, so one grant starts one
/// worker.
///
/// ⚠️ RECALL OF 2026-08-21, TASK 11 BRIEF PART A4. The comment that used to close this
/// function claimed this property was held by a `compile_fail` case -- "a second `start`
/// would need a second grant... held by `compile_fail/`, not here". FALSE, measured against
/// the 29 cases that existed before this task: none names a `Grant` moved by `start`, and the
/// two that name `Grant` at all hold different rules (no public constructor; matched only
/// inside `Admission`). ⛔ WHAT HOLDS "one grant starts one worker" IS THE COMPILER, not a case
/// in this repository: `Process::start` takes `grant: Grant` BY VALUE and `Grant` derives no
/// `Clone` (`crates/kernel/src/arbiter/mod.rs`, no `#[derive]` above `pub struct Grant`), so a
/// second `.start(grant, ..)` below would not compile -- `grant` was moved into the call that
/// is already there. That is why this function calls `start` exactly once: a second call is
/// not refused at runtime, it does not exist to be written.
#[test]
fn one_grant_starts_one_worker() {
    let grant = a_real_grant();
    // ⚠️ NOT `assert!(first.is_ok())`: the fake always answers `Ok`, so that assertion could
    // never fail and would prove nothing -- a vacuous probe. What this function proves is the
    // SHAPE above it, that it compiles at all with `grant` used exactly once; `expect` below
    // is for the panic message, not for coverage.
    FakeProcess
        .start(grant, WorkerDescriptor::new(b"asr.exe".to_vec()))
        .expect("the fake always starts");
}
