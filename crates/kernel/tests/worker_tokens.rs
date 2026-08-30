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
    Admission, Arbiter, ArbiterId, ComputeClass, Grant, Mib, Preemption, RemotePolicy,
    ResourceProfile, VramPolicy,
};
use kernel::parameters::Parameters;
use kernel::ports::process::{
    Frame, Process, ProcessError, SingleReceipt, StreamReceipt, Worker, WorkerDescriptor,
};
use kernel::time::{Millis, Monotonic};

/// A grant obtained the only way there is.
fn a_real_grant() -> Grant {
    let mut arbiter = Arbiter::new(
        Parameters::new(10_000, Mib::new(16_384), ArbiterId::new(1)),
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
/// in this repository: `Process::start` takes `grant: Grant` BY VALUE and `Grant` is not
/// `Copy` -- and cannot become it, because it derives no `Clone` either
/// (`crates/kernel/src/arbiter/mod.rs`, no `#[derive]` above `pub struct Grant`), so a
/// second `.start(grant, ..)` below would not compile -- `grant` was moved into the call that
/// is already there. ⚠️ `Copy` IS THE HINGE AND `Clone` IS NOT, which is what the oracles say
/// too (`reading_twice_from_one_receipt.stderr`: "does not implement the `Copy` trait"):
/// adding `Clone` here would not disarm anything, adding `Copy` would. That is why this
/// function calls `start` exactly once: a second call is not refused at runtime, it does not
/// exist to be written.
#[test]
fn one_grant_starts_one_worker() {
    let grant = a_real_grant();
    // ⚠️ NOT `assert!(first.is_ok())`: the fake always answers `Ok`, so that assertion could
    // never fail and would prove nothing -- a vacuous probe. And this function holds no shape
    // that `a_started_worker` above does not already compile: it is a declared place to hang
    // the reasoning, not a probe. `expect` below is for the panic message, not for coverage.
    FakeProcess
        .start(grant, WorkerDescriptor::new(b"asr.exe".to_vec()))
        .expect("the fake always starts");
}

/// A `Process` that never manages to spawn.
///
/// ⛔ IT EXISTS FOR ONE REASON, and the reason is written here so a YAGNI pass knows what it is
/// looking at: `ProcessError::StartFailed` had neither producer nor test, and the paragraph on
/// `ProcessError` that kept it alive leant on a prose deadline that expired in silence on
/// 2026-08-21 (finding AUD-051). ⚠️ A DOUBLE CANNOT PROVE A REAL SPAWN FAILS. What it proves is
/// that the word is constructible and that `start` carries a failure back to its caller — level
/// 1, the same strength the other three variants get from the fakes that already produce them.
struct FailingProcess;

impl Process for FailingProcess {
    type Handle = FakeWorker;

    fn start(
        &mut self,
        _grant: Grant,
        _descriptor: WorkerDescriptor,
    ) -> Result<Self::Handle, ProcessError> {
        Err(ProcessError::StartFailed)
    }
}

/// The only producer of `StartFailed` in the workspace, and what keeps it from being a word
/// nobody has ever written.
///
/// ⚠️ NOT `assert!(outcome.is_err())`: this fake always fails, so that assertion could not tell
/// `StartFailed` from any other variant and would stay green if the fake returned `Died`. The
/// equality is what pins the variant.
#[test]
fn a_spawn_that_does_not_happen_is_start_failed() {
    let outcome = FailingProcess.start(a_real_grant(), WorkerDescriptor::new(b"asr.exe".to_vec()));

    assert_eq!(outcome.err(), Some(ProcessError::StartFailed));
}
