//! The `process` port: THE WHOLE LIFE OF A WORKER -- start, dialogue, kill (§2.3.1,
//! ADR-0035). It is ONE port and not two: the object you talk to is the one the start
//! returned, and the start demands a grant (§5.6). Splitting start from dialogue would
//! reopen the closure that took I2 from the test to THE COMPILER.
//!
//! # The tension of design/01, and how it dissolves (§6.10.1)
//!
//! One row says "the worker does not answer on its own initiative"; the next says "the
//! audio stream flows back up to the core". The shape of the port dissolves it:
//!
//! > EVERY BYTE THAT FLOWS BACK IS COVERED BY A RECEIPT, and receipts are issued only by
//! > an instruction. A frame no receipt covers is not data, it is A FAULT -- and whoever
//! > names one gets `UnsolicitedFrame` back.
//!
//! ⛔ AND THAT LAST CLAUSE IS THE HONEST FORM OF THE SENTENCE. It used to read "has no way
//! of being named", which promises THE COMPILER and is false: `SingleReceipt::new` and
//! `StreamReceipt::new` are `pub`, so anyone can write `StreamReceipt::new(7)` and has
//! just named a frame no instruction covers. The rule is real; what enforces it is the
//! IMPLEMENTATION AT RUNTIME, not the type system. The full argument sits beside the two
//! constructors, and it is the difference between this rule and `Grant`'s.
//!
//! The audio worker keeps a stream receipt open for its whole life, opened by a single
//! instruction at start-up.
//!
//! ⚠️ A STREAM RECEIPT IS NOT A JOURNAL STEP. The fragments flowing back from a
//! continuous transcription are a SOURCE OF EVENTS, not steps (ADR-0011, gotcha #1):
//! journalling them would violate Q1. What gets journalled is the grant and the outcome.
//!
//! # What milestone 2 builds, and what it does not
//!
//! The trait and its types. NOT the implementation (milestone 6), NOT the wire format
//! (§6.10.3: `minicbor`, the port exchanges BYTES, every frame declares its own length
//! and decoding checks the bytes consumed), and NOT the negative tests of §6.10.5 rows
//! 1-4: all four need to OBTAIN a `Worker`, a `Worker` comes only from `start(grant,..)`,
//! and no arbiter issues grants until milestone 5. A row proved in one direction only is
//! not admissible (§7.1.1 rule 3), so they are registered as not-yet-covered in
//! `docs/porta-di-qualita.md`.
//!
//! ⚠️ AND WHAT HOLDS THESE SIGNATURES MEANWHILE IS ONE TEST, the same one that holds
//! `filesystem` and `network`: `tests/ports_are_implementable.rs` writes a fake for
//! `Worker` and one for `Process` and calls them. It buys that the signatures are
//! IMPLEMENTABLE FROM OUTSIDE THE CRATE and callable; it does NOT buy that they are the
//! right signatures, and it is not the conformance suite, which needs two implementations
//! to compare and is born with the real worker channel in milestone 6.

use alloc::vec::Vec;

/// A grant from the arbiter. THE ONLY WAY TO START A WORKER.
///
/// ⛔ There is deliberately NO public constructor. A grant can be issued only by the
/// arbiter (§5.6), which arrives in milestone 5; whoever writes "start the worker"
/// without one DOES NOT COMPILE. Today the type has no issuer, and that is why the
/// negative tests of §6.10.5 are staged rather than written vacuously.
///
/// ⛔ AND THIS IS THE ONE PLACE WHERE THE ABSENCE OF A CONSTRUCTOR IS THE POINT, which is
/// why the remedy applied to the two receipts below STOPS HERE. `platform` RECEIVES a
/// grant, it does not create one: naming `Grant` in a signature is all an implementation
/// outside the crate needs, and it is what `tests/ports_are_implementable.rs` exercises.
/// The declared limit that follows is real and is written rather than left to be
/// discovered: `Process::start` is IMPLEMENTABLE today and NOT CALLABLE, by anyone, until
/// milestone 5 gives the arbiter a way to issue one. A test-only constructor was weighed
/// and refused in `docs/porta-di-qualita.md` -- it would create the second way of
/// obtaining a grant that §5.6 exists to take away from the compiler.
///
/// ⚠️ DIVERGENCE FROM THE PLAN, AND IT WAS MEASURED. The plan dictated a named field
/// `reserved_mib: u64`. It buys the unconstructibility above, and it costs a
/// `#[allow(dead_code)]` -- nothing reads the number, and this repository treats an
/// `allow` as a prohibition switched off (gotcha #13). The private UNIT field gives the
/// identical guarantee for free: from an integration test `Grant(())` is
/// `error[E0423]: cannot initialize a tuple struct which contains private fields`, with
/// zero warnings and no `allow`. The named field also spelled out a piece of the arbiter's
/// model -- a reservation in MiB -- that belongs to milestone 5 and would have been
/// invented here.
///
/// ⚠️ NO `Debug` EITHER, for the reason that removed `CheckpointId::get()` and the unused
/// derives on `Path` and `StepId`: nothing formats a grant. It comes back with the caller
/// that needs it. The receipts below KEEP `Debug` -- there it is load-bearing, `unwrap_err`
/// requires it.
pub struct Grant(());

/// What to start. Opaque to the kernel: an executable path and its arguments are
/// OS-specific, and I3 keeps them behind the platform module.
///
/// ⚠️ NO `Clone`, AND THE CONTRAST WITH `Path` IS THE WHOLE ARGUMENT. There `Clone` is
/// LOAD-BEARING and says so: `declare_scope` hands the implementation a BORROWED slice it
/// has to retain, so without `Clone` the port is not implementable TODAY. `Endpoint` is
/// the same. This type crosses the port BY VALUE -- `start` takes it owned -- so whoever
/// implements already owns it, moves it instead of duplicating it, and reaches the bytes
/// with `as_bytes`. Measured in both directions: removed from here and from `Frame`,
/// `cargo test --workspace` is green with zero warnings; removed from `Path` as a
/// counter-probe, it is red. ⛔ "A caller will want it in milestone 6" is NOT the same
/// shape as "not implementable today", and it is answered by the formula this repository
/// already uses twice: it comes back the day something needs it, with that caller.
#[derive(Debug, PartialEq, Eq)]
pub struct WorkerDescriptor(Vec<u8>);

impl WorkerDescriptor {
    pub fn new(raw: Vec<u8>) -> Self {
        WorkerDescriptor(raw)
    }

    /// The bytes back out, for `Path::as_bytes`'s reason: the privacy of a tuple-struct
    /// field is MODULE-scoped, so without this an implementation outside `kernel` could
    /// not hand the descriptor to the OS it is supposed to spawn on.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// One message on the wire, as bytes.
///
/// ⛔ The port exchanges BYTES and not typed messages, for the same reason as `journal`:
/// with a byte port THE SIMULATOR EXCHANGES BYTES, so the DST campaign really exercises
/// encoding and decoding instead of going around them (§6.10.3).
///
/// ⚠️ NO `Clone`, same argument as `WorkerDescriptor` and written once there. Every frame
/// crosses the port BY VALUE. ⛔ The tempting objection is I5 -- retries live in the core,
/// and a retry would resend a frame -- and it does not hold: that caller arrives in
/// milestone 6, and until then the derive would be kept alive by an intention. The fake in
/// `tests/ports_are_implementable.rs` is the instrument this repository gave itself for
/// exactly that question, and it never clones one.
#[derive(Debug, PartialEq, Eq)]
pub struct Frame(Vec<u8>);

impl Frame {
    pub fn new(bytes: Vec<u8>) -> Self {
        Frame(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// The receipt of an instruction expecting ONE answer.
///
/// ⛔ Reading CONSUMES it: reading twice does not compile.
#[derive(Debug)]
pub struct SingleReceipt {
    pub(crate) id: u64,
}

impl SingleReceipt {
    /// ⛔ WITHOUT THIS THE PORT IS NOT IMPLEMENTABLE, and the plan did not have it. It is
    /// gotcha #46 in its worse form: not "I cannot read a field" but "I CANNOT PRODUCE THE
    /// RETURN VALUE". `instruct_one` must HAND BACK a `SingleReceipt`, whoever implements
    /// `Worker` is `platform` (milestone 6), and the privacy of a struct field is
    /// MODULE-scoped -- so from outside `kernel` the value could not be built at all.
    /// Measured, not deduced: `SingleReceipt::new` absent, the fake in
    /// `tests/ports_are_implementable.rs` failed with `E0599`.
    ///
    /// ⚠️ AND THIS IS NOT `Grant`. There the absence of a constructor is the design (§5.6)
    /// and it stays absent: a grant is RECEIVED from the arbiter, never minted by the
    /// implementation. A receipt is the opposite -- it is MINTED BY the implementation,
    /// which is the only party that knows what it just sent.
    ///
    /// ⛔ THE DECLARED LIMIT THIS COSTS, and it is the most important one in the file
    /// because it is the one a reader would otherwise assume he had been given. `new` is
    /// `pub` because `Worker` is implemented OUTSIDE this crate -- `platform`, milestone 6
    /// -- and Rust has no visibility narrower than `pub` that still reaches another crate.
    /// So "a receipt implies an instruction", the sentence §6.10.1 rests the whole port on,
    /// IS NOT A GUARANTEE OF THE COMPILER: anyone can mint `SingleReceipt::new(7)` and hand
    /// it to a reader. ⛔ WHAT ENFORCES IT IS THE IMPLEMENTATION, AT RUNTIME, by answering
    /// `UnsolicitedFrame` to a receipt it never issued -- which is why that variant is not
    /// decoration and why `close` must be able to refuse too, even though its name suggests
    /// only the worker-to-core direction. `tests/ports_are_implementable.rs` exercises
    /// exactly that refusal, with a genuine stream open alongside a forged receipt.
    ///
    /// ⚠️ Contrast with `Grant`, deliberately: there the guarantee IS the compiler's,
    /// because no constructor exists at all. The two rules look alike in the prose of
    /// §5.6 and §6.10.1 and are enforced by entirely different machinery -- the kind of
    /// asymmetry `boundary.rs` writes out rather than leaves to be discovered.
    pub const fn new(id: u64) -> Self {
        SingleReceipt { id }
    }

    /// The number back out, and it is LOAD-BEARING rather than a convenience.
    ///
    /// ⛔ THE ARGUMENT THAT KEEPS IT IS THE ONE THAT REMOVED `CheckpointId::get()`, applied
    /// again and landing the other way -- worth writing down, because the two look alike.
    /// §6.10.1 makes correlation the whole contract of this port: every byte that flows
    /// back is covered by a receipt, so an implementation that cannot tell two receipts
    /// apart cannot honour it. `CheckpointId` needs no getter because it is `Copy` and
    /// `PartialEq`: the implementation retains a copy and compares. ⛔ A RECEIPT CANNOT BE,
    /// and deliberately: `Copy` or `Clone` would let a caller read the same receipt twice,
    /// which is exactly the guarantee the type exists to give. With duplication forbidden,
    /// extracting a plain value is THE ONLY WAY LEFT to correlate -- so the getter is not
    /// a speculative convenience, it is what the consumption guarantee costs.
    ///
    /// ⛔ AND THAT ARGUMENT IS NOW HELD BY A TEST, having spent one revision held by nothing.
    /// Measured: a fake with ZERO calls to `id()` satisfied all 8 tests, because none of them
    /// kept two receipts open at once. `answers_are_correlated_to_the_receipt_that_asked` is
    /// what fails today if this getter stops being used -- it is the only test that dies when
    /// `read_one` answers a constant equal to the first id.
    pub const fn id(&self) -> u64 {
        self.id
    }
}

/// The receipt of an instruction expecting A STREAM of answers.
///
/// ⛔ TWO RECEIPT TYPES AND NOT AN ENUM WITH TWO ARMS. It costs one extra reading
/// function and it buys that "a single answer becomes a stream" IS NOT EXPRESSIBLE --
/// which is exactly the sentence in design/01.
#[derive(Debug)]
pub struct StreamReceipt {
    pub(crate) id: u64,
}

impl StreamReceipt {
    /// Same reason as `SingleReceipt::new`, written once there -- INCLUDING the declared
    /// limit: this constructor is `pub`, a forged `StreamReceipt::new(7)` is expressible,
    /// and what refuses it is `read_next` and `close` at runtime rather than the compiler.
    pub const fn new(id: u64) -> Self {
        StreamReceipt { id }
    }

    /// Same reason as `SingleReceipt::id`, written once there -- and here the need is
    /// sharper still: `read_next` takes `&mut self` and the receipt STAYS OPEN, so an
    /// implementation serving two streams at once has nothing but this number to tell
    /// which of them is being read.
    pub const fn id(&self) -> u64 {
        self.id
    }
}

/// What can go wrong on the way to a worker.
///
/// ⚠️ THE "NO CALLER, NO ITEM" RULE DOES NOT REACH THESE VARIANTS, the same note that sits
/// on `FilesystemError` and `NetworkError` and for the same reason: the port has no
/// implementation at all, so NO variant has a producer, and applying the rule on that
/// basis would empty the enum instead of pruning it. Each of the four is a failure a real
/// worker channel really has. ⛔ `StartFailed` is the one with neither producer NOR test,
/// and it stays: it is the only word this vocabulary has for a spawn that did not happen,
/// and it becomes reachable the day `start` becomes callable -- milestone 5.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessError {
    /// The process could not be started.
    StartFailed,
    /// The worker died. Always possible: a worker can be killed without warning.
    Died,
    /// The frame did not decode, or the bytes consumed did not equal the length the
    /// frame declared. ⛔ A CBOR decoder stops at the first complete element and ignores
    /// the tail: without this check a malformed frame is a WRONG VALUE, not an error
    /// (gotcha #34, measured).
    MalformedFrame,
    /// The worker spoke with no receipt open. It is a FAULT, not data (§6.10.1).
    UnsolicitedFrame,
}

/// The handle of a live worker. Obtained ONLY from `Process::start`.
pub trait Worker {
    /// An instruction expecting one answer.
    fn instruct_one(&mut self, frame: Frame) -> Result<SingleReceipt, ProcessError>;

    /// An instruction expecting a stream of answers. IT IS THE INSTRUCTION that declares
    /// which of the two, because whoever sends it knows what it expects.
    fn instruct_stream(&mut self, frame: Frame) -> Result<StreamReceipt, ProcessError>;

    /// The single answer. CONSUMES the receipt.
    fn read_one(&mut self, receipt: SingleReceipt) -> Result<Frame, ProcessError>;

    /// The next frame of a stream. The receipt stays open until the worker declares the
    /// end or the core closes it.
    fn read_next(&mut self, receipt: &mut StreamReceipt) -> Result<Option<Frame>, ProcessError>;

    /// Closes a stream.
    fn close(&mut self, receipt: StreamReceipt) -> Result<(), ProcessError>;

    /// Kills the worker, and it is ALWAYS lawful (§5.3, point 4).
    ///
    /// ⛔ CONSUMES the `Worker`: instructing it after the kill does not compile.
    fn kill(self) -> Result<(), ProcessError>;
}

pub trait Process {
    /// The handle type this implementation returns.
    type Handle: Worker;

    /// Starts a worker.
    ///
    /// ⛔ Takes the GRANT as an argument: whoever writes "start the worker" without one
    /// does not compile. This is the half of I2 that belongs to the compiler; the other
    /// half -- that `process` is the only port towards processes -- rests on a level 2
    /// check and is therefore deletable. Declared, not hidden (§5.6).
    fn start(
        &mut self,
        grant: Grant,
        descriptor: WorkerDescriptor,
    ) -> Result<Self::Handle, ProcessError>;
}
