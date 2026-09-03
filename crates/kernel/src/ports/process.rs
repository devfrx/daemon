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
//! 1-4: a row proved in one direction only is not admissible (§7.1.1 rule 3).
//!
//! ⛔ RECALL OF 2026-08-21, AUDIT FINDING P-2. The reason this paragraph gave for staging
//! them -- "all four need to OBTAIN a `Worker`, a `Worker` comes only from
//! `start(grant,..)`, and no arbiter issues grants until milestone 5" -- was FALSE, and it
//! is TAKEN OUT rather than reworded. A `Worker` comes from IMPLEMENTING THIS TRAIT, with
//! no grant anywhere: measured from outside the crate, and `tests/ports_are_implementable.rs`
//! has done it since milestone 2. The verbal and both measurements live in
//! `docs/porta-di-qualita.md`.
//!
//! ⛔ AND A SECOND RECALL, 2026-08-21, MILESTONE 5 TASK 11: the STATE that sentence still
//! carried -- "so they are registered as not-yet-covered" -- IS TAKEN OUT TOO, and not
//! rewritten. The pass above took out the false REASON and left the state standing; hours
//! later the four `compile_fail` cases were written and the row went CLOSED. The state
//! belongs to that register's cell alone: a figure kept in two houses rots in the one
//! nobody moves.
//!
//! ⚠️ AND WHAT HOLDS THESE SIGNATURES MEANWHILE IS NOT ONE TEST -- `process` is the one
//! family where that is so, and what each bench buys DIFFERS, which is the half worth knowing.
//! `tests/ports_are_implementable.rs` buys that the signatures are IMPLEMENTABLE FROM OUTSIDE
//! THE CRATE and callable. `tests/worker_tokens.rs` buys more than implementability: it drives
//! `start` with a `Grant` the arbiter really issued, so the port is exercised on the ADMISSION
//! PATH the other bench never touches. And the `tests/compile_fail/` cases hold the TOKEN
//! SHAPES at level 1, where no test can reach. ⛔ None of the three buys that these are the
//! RIGHT signatures, and none is the conformance suite, which needs two implementations to
//! compare and is born with the real worker channel in milestone 6.
//!
//! ⛔ DATED RECALL, 2026-08-28 -- FINDING AUD-054. The paragraph above read "IS ONE TEST, the
//! same one that holds `filesystem` and `network`". True when written, false from `5fceee1`
//! (2026-08-21), which landed `worker_tokens.rs` AND the four `compile_fail` cases in ONE
//! commit. ⚠️ AND THIS FILE WAS TOUCHED TWICE AFTERWARDS WITHOUT THE PARAGRAPH MOVING --
//! `15095be` the very next day, to announce those four cases, and `f275f0c` on 2026-08-27.
//! ⛔ SO THE COUNT IS GONE RATHER THAN REALIGNED: it lived in THREE houses -- here, `ipc.rs`
//! and `ports/mod.rs` -- and `CLAUDE.md` says a figure kept in more than one house is REMOVED,
//! not re-corrected. `grep -rn 'impl Worker for' crates/` names the benches, and a count
//! written here would age again. ⚠️ For `filesystem` and `network` the old sentence is still
//! TRUE, measured: each has exactly ONE implementation from outside the crate.
//!
//! ⛔ DATED RECALL, 2026-08-31 -- MILESTONE 6 TASK 3. "NOT the wire format" was TWO claims
//! inside one clause: a statement about MILESTONE 2, still true, and a description of
//! something nobody had built. The second half died with this commit. ✅ THE FORMAT EXISTS:
//! the envelope is `crate::framing` -- a declared length, four bytes big-endian -- and the
//! schema that rides inside it is `crate::wire::worker`, whose `decode` checks the bytes
//! consumed. ⚠️ AND THIS FILE DID NOT MOVE, which is the half worth saying: `Frame` is still
//! an opaque `Vec<u8>` and no signature here changed. The schema lives OUTSIDE the port,
//! where the journal's has lived since ADR-0036 -- §6.10.3, "the port exchanges BYTES, not
//! typed messages". ⚠️ The other two clauses stand: there is still no implementation of this
//! trait outside the benches, and the rows 1-4 sentence is a statement about milestone 2.

use alloc::vec::Vec;

use crate::arbiter::Grant;

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
/// worker channel really has.
///
/// ⛔ RECALL OF 2026-08-27, finding AUD-051 — THIS SAID `StartFailed` "is the one with neither
/// producer NOR test" and that it "becomes reachable the day `start` becomes callable --
/// milestone 5". That day came on 2026-08-21 and NOTHING TURNED RED: a deadline written in
/// prose has nothing that makes it fire (gotcha #77), and the commit that made `start`
/// callable touched this file the next day for two other recalls without rereading this one.
/// The variant now has what the other three had — a producer and a test — in
/// `tests/worker_tokens.rs`, so what keeps it alive is something that must keep COMPILING
/// instead of a date. ⚠️ ITS STRENGTH IS LEVEL 1, AND SAYING SO IS THE POINT: a double proves
/// the word is constructible and that `start`'s signature carries a failure back, NOT that a
/// real spawn failure produces it. That producer arrives with the milestone that implements
/// this port, and it does not need a deadline in prose to be noticed missing.
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

/// What starting a worker did. ⛔ NOT a `Result`, and the shape is `Admission`'s.
///
/// ⛔ THE REJECTED ARM CARRIES THE GRANT BACK BY NAME. `start` consumes it, so before this
/// type a failed start dropped a reservation nobody could rebuild -- `GrantId` is private and
/// `tests/compile_fail/grant_has_no_constructor.rs` pins it -- and the books held it for the
/// whole declared window. The sweep was the only way back.
///
/// ⚠️ WHY NOT `Result<H, (Grant, ProcessError)>`: no error in this repository carries the
/// value it consumed, measured on 2026-08-30 with
/// `grep -rnE "Result<[^,]+, *\([A-Z]" crates/ --include=*.rs`, which returns ONE line --
/// this one, quoting the shape it rejects. The shape this project uses for "several
/// outcomes, each carrying what belongs to it" is
/// `Admission`. A second idiom would be a second way to say one thing.
///
/// ⚠️ NO `Debug` AND NO `PartialEq`, for the reason `Admission` has neither: `Rejected`
/// carries a `Grant`. Probes destructure with `let … else` instead of `assert_eq!`.
#[must_use]
pub enum Started<H> {
    /// The worker is alive, and the grant is now its.
    Running(H),
    /// It never started. The grant comes back, and so does the reason.
    Rejected { grant: Grant, error: ProcessError },
}

/// What killing a worker did.
///
/// ⛔ A STRUCT AND NOT AN ENUM, because there are not two states: there is ONE state with two
/// facts. The grant comes back whatever happened, and `outcome` says whether the kill itself
/// went cleanly.
///
/// ⛔ THE GRANT SITS OUTSIDE EVERY `Result`, and that is the teaching part: it comes back even
/// on the arm where the worker died badly. `kill` is ALWAYS LAWFUL (§5.3 point 4), and a
/// reservation is a fact of the books, not of the process's health.
///
/// ⚠️ NO `Debug` AND NO `PartialEq`, same reason again -- it carries a `Grant`. The bench
/// asserts on `outcome`, which is a `Result<(), ProcessError>` and derives both.
#[must_use]
pub struct Killed {
    /// The reservation, back to whoever will hand it to the arbiter.
    pub grant: Grant,
    /// Whether the kill itself succeeded.
    pub outcome: Result<(), ProcessError>,
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
    /// ⛔ AND IT RETURNS THE GRANT, outside the `Result` -- see `Killed`.
    fn kill(self) -> Killed;
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
    ///
    /// ⛔ IT RETURNS `Started` AND NOT A `Result`, so the grant of a failed start has a way
    /// home -- see `Started::Rejected`.
    fn start(&mut self, grant: Grant, descriptor: WorkerDescriptor) -> Started<Self::Handle>;
}
