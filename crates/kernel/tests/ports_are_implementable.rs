//! One fake per port declared WITHOUT an implementation, and calls that exercise them
//! (§2.3, gotcha #17).
//!
//! ⛔ WHAT THIS BUYS: that the signatures of `Filesystem`, `Network`, `Worker` and `Process`
//! are IMPLEMENTABLE and CALLABLE. A trait nobody implements has not been proved implementable
//! — a signature can be unusable for borrows, for ownership or for object safety, and one
//! finds that out when the code that has to use it already exists. It has already happened in
//! this milestone: a `Wakeup` enum declared in advance turned out to be unusable and was
//! removed.
//!
//! ⛔ AND ON `process` IT CAUGHT ONE, which is worth recording because it is the first time
//! this file found a defect instead of confirming a design. The port as the plan dictated it
//! could NOT be implemented from here: `instruct_one` must return a `SingleReceipt` whose only
//! field is `pub(crate)`, and field privacy is MODULE-scoped — so the return value was
//! unbuildable outside `kernel`. That is gotcha #46 in its worse form: not an unreadable
//! field, an unproducible VALUE. Two constructors and two getters in `ports/process.rs` are
//! the remedy, each carrying its reason.
//!
//! ⚠️ AND THE MEASUREMENT HAD A TRAP WORTH LEAVING WRITTEN: rustc reported `E0599` for the
//! missing constructors and NOTHING AT ALL for a `SingleReceipt { id }` literal. The literal
//! is an `E0451`, emitted by the PRIVACY pass — which never ran, because type-checking had
//! already failed. Errors from different passes MASK each other, so "I fixed the error and it
//! built" can hide a second one that was never reached.
//!
//! ⛔ WHAT IT DOES NOT BUY: that these are the RIGHT signatures. That is the spec's decision,
//! not this file's. And it is NOT a conformance suite — the one `tests/reactor_contract.rs` is
//! for `reactor` — which needs TWO implementations to compare and is born the day these ports
//! get a real one. The fakes below do nothing useful ON PURPOSE: they have to compile and run.
//!
//! ⚠️ AND THE FAKE DECIDES SOMETHING THE KERNEL CANNOT, which is the whole point of `Path`'s
//! comment: `InMemoryFilesystem` calls a path "inside a scope" when its bytes START WITH the
//! bytes of a declared path. That is A CHOICE OF THIS FAKE, not a rule of the port — the kernel
//! does not interpret paths, so it cannot say whether two `Path` name one file.

use kernel::ports::filesystem::{CheckpointId, Filesystem, FilesystemError, Path};
use kernel::ports::journal::StepId;
use kernel::ports::network::{Endpoint, Network, NetworkError};
use kernel::ports::process::{
    Frame, Grant, Process, ProcessError, SingleReceipt, StreamReceipt, Worker, WorkerDescriptor,
};

#[derive(Default)]
struct InMemoryFilesystem {
    scopes: Vec<Path>,
    files: Vec<(Path, Vec<u8>)>,
    preserved: Vec<(CheckpointId, Path, Vec<u8>)>,
}

impl InMemoryFilesystem {
    fn in_scope(&self, path: &Path) -> bool {
        self.scopes
            .iter()
            .any(|scope| path.as_bytes().starts_with(scope.as_bytes()))
    }

    fn position(&self, path: &Path) -> Option<usize> {
        self.files.iter().position(|(known, _)| known == path)
    }
}

impl Filesystem for InMemoryFilesystem {
    fn declare_scope(&mut self, paths: &[Path]) -> Result<(), FilesystemError> {
        // ⚠️ THIS LINE IS WHY `Path` KEEPS `Clone`: the trait hands over a borrowed slice and
        // the implementation has to retain the scopes. `extend_from_slice` demands it.
        self.scopes.extend_from_slice(paths);
        Ok(())
    }

    fn preserve(&mut self, _step: StepId, path: &Path) -> Result<CheckpointId, FilesystemError> {
        if !self.in_scope(path) {
            return Err(FilesystemError::OutsideScope);
        }
        let content = match self.position(path) {
            Some(i) => self.files[i].1.clone(),
            None => return Err(FilesystemError::Missing),
        };
        // ⚠️ And this is why `CheckpointId` keeps `Copy`: the handle goes into the store AND
        // comes back out of the call.
        let checkpoint = CheckpointId::new(self.preserved.len() as u64);
        self.preserved.push((checkpoint, path.clone(), content));
        Ok(checkpoint)
    }

    fn restore(&mut self, checkpoint: CheckpointId) -> Result<(), FilesystemError> {
        let found = self
            .preserved
            .iter()
            .find(|(known, _, _)| *known == checkpoint)
            .cloned();
        let Some((_, path, content)) = found else {
            return Err(FilesystemError::Missing);
        };
        match self.position(&path) {
            Some(i) => self.files[i].1 = content,
            None => self.files.push((path, content)),
        }
        Ok(())
    }

    fn read(&self, path: &Path) -> Result<Vec<u8>, FilesystemError> {
        if !self.in_scope(path) {
            return Err(FilesystemError::OutsideScope);
        }
        match self.position(path) {
            Some(i) => Ok(self.files[i].1.clone()),
            None => Err(FilesystemError::Missing),
        }
    }

    fn write(&mut self, path: &Path, content: &[u8]) -> Result<(), FilesystemError> {
        if !self.in_scope(path) {
            return Err(FilesystemError::OutsideScope);
        }
        match self.position(path) {
            Some(i) => self.files[i].1 = content.to_vec(),
            None => self.files.push((path.clone(), content.to_vec())),
        }
        Ok(())
    }
}

/// A recorder, modelled on `RecordingJournal` in `boundary_promotion.rs`. What this port is
/// FOR is that everything leaving the machine goes through one place, so the fake worth having
/// is the one that writes down what left.
#[derive(Default)]
struct RecordingNetwork {
    sent: Vec<(Endpoint, Vec<u8>)>,
}

impl Network for RecordingNetwork {
    fn request(&mut self, endpoint: &Endpoint, body: &[u8]) -> Result<Vec<u8>, NetworkError> {
        if endpoint.as_bytes().is_empty() {
            return Err(NetworkError::Unreachable);
        }
        self.sent.push((endpoint.clone(), body.to_vec()));
        let mut answer = Vec::from(endpoint.as_bytes());
        answer.extend_from_slice(body);
        Ok(answer)
    }
}

/// A scripted worker, and it CORRELATES on purpose. §6.10.1 gives the port its shape --
/// every byte that flows back is covered by a receipt -- so a fake that ignored which
/// receipt asked would exercise the method names and not the contract. An audio worker
/// keeps one stream open for its whole life while single instructions come and go, so a
/// fake with a single implicit stream would never touch the correlation at all.
struct ScriptedWorker {
    next_id: u64,
    /// Open streams, and how many frames each still owes before it runs dry.
    streams: Vec<(u64, usize)>,
    /// A worker can die without warning (§5.3). This drives the REFUSING direction.
    dead: bool,
}

impl ScriptedWorker {
    fn new() -> Self {
        ScriptedWorker {
            next_id: 1,
            streams: Vec::new(),
            dead: false,
        }
    }

    fn issue(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// The worker dies on its own, WITHOUT the core asking. `ProcessError::Died` says this
    /// is always possible, and without a way to provoke it the refusing direction of every
    /// method below would be unreachable.
    fn die(&mut self) {
        self.dead = true;
    }

    /// The liveness guard every dialogue method opens with.
    ///
    /// ⛔ IT IS HERE SO THAT ITS ABSENCE IS VISIBLE. Five methods start with `self.alive()?`
    /// and `kill` DOES NOT -- deliberately, because killing is always lawful (§5.3 point 4).
    /// Copied out five times, that exception was a thing you had to read a comment to
    /// notice; as one line repeated five times, the one place it is missing stands out.
    fn alive(&self) -> Result<(), ProcessError> {
        if self.dead {
            return Err(ProcessError::Died);
        }
        Ok(())
    }
}

impl Worker for ScriptedWorker {
    fn instruct_one(&mut self, frame: Frame) -> Result<SingleReceipt, ProcessError> {
        self.alive()?;
        // ⚠️ THE FAKE DECIDES THIS, not the port: an empty frame is this implementation's
        // notion of "did not decode". The port says `MalformedFrame` exists and says
        // nothing about how a frame is judged -- the kernel exchanges bytes and does not
        // read them, so it could not judge one if it wanted to.
        if frame.as_bytes().is_empty() {
            return Err(ProcessError::MalformedFrame);
        }
        Ok(SingleReceipt::new(self.issue()))
    }

    fn instruct_stream(&mut self, frame: Frame) -> Result<StreamReceipt, ProcessError> {
        self.alive()?;
        if frame.as_bytes().is_empty() {
            return Err(ProcessError::MalformedFrame);
        }
        let id = self.issue();
        // The instruction declares how many frames it expects back; this fake reads that
        // off the frame's own length so that two streams can differ.
        self.streams.push((id, frame.as_bytes().len()));
        Ok(StreamReceipt::new(id))
    }

    fn read_one(&mut self, receipt: SingleReceipt) -> Result<Frame, ProcessError> {
        self.alive()?;
        // The answer names the receipt it answers. Nothing else in this file can tell one
        // answer from another, which is the point of correlating.
        Ok(Frame::new(id_bytes(receipt.id())))
    }

    fn read_next(&mut self, receipt: &mut StreamReceipt) -> Result<Option<Frame>, ProcessError> {
        self.alive()?;
        let Some(position) = self.streams.iter().position(|(id, _)| *id == receipt.id()) else {
            // A frame asked for on a receipt this worker never opened is a FAULT and not
            // an empty answer (§6.10.1). `Ok(None)` here would say "the stream ended",
            // which is a different and much quieter lie.
            return Err(ProcessError::UnsolicitedFrame);
        };
        if self.streams[position].1 == 0 {
            return Ok(None);
        }
        self.streams[position].1 -= 1;
        Ok(Some(Frame::new(id_bytes(receipt.id()))))
    }

    fn close(&mut self, receipt: StreamReceipt) -> Result<(), ProcessError> {
        self.alive()?;
        match self.streams.iter().position(|(id, _)| *id == receipt.id()) {
            Some(position) => {
                self.streams.remove(position);
                Ok(())
            }
            None => Err(ProcessError::UnsolicitedFrame),
        }
    }

    fn kill(self) -> Result<(), ProcessError> {
        // ⛔ AND IT DOES NOT REFUSE, DELIBERATELY -- not even a worker already dead. The
        // port says killing is ALWAYS LAWFUL (§5.3 point 4), so a fake that answered
        // `Err(Died)` here would be contradicting the contract it exists to exercise.
        // The refusing direction lives on the instruct and read paths above.
        Ok(())
    }
}

fn id_bytes(id: u64) -> Vec<u8> {
    Vec::from(id.to_le_bytes())
}

/// The other half of the port, and the half that cannot be CALLED from here.
struct SpawningProcess {
    started: usize,
}

impl Process for SpawningProcess {
    type Handle = ScriptedWorker;

    fn start(
        &mut self,
        _grant: Grant,
        _descriptor: WorkerDescriptor,
    ) -> Result<Self::Handle, ProcessError> {
        self.started += 1;
        Ok(ScriptedWorker::new())
    }
}

#[test]
fn the_filesystem_port_can_be_implemented_and_called() {
    let mut filesystem = InMemoryFilesystem::default();
    let inside = Path::new(b"scope/a.txt".to_vec());

    filesystem
        .declare_scope(&[Path::new(b"scope/".to_vec())])
        .expect("the scope was accepted");

    assert_eq!(filesystem.write(&inside, b"first"), Ok(()));
    assert_eq!(filesystem.read(&inside), Ok(b"first".to_vec()));

    let checkpoint = filesystem
        .preserve(StepId::new(1), &inside)
        .expect("the version was preserved");
    assert_eq!(filesystem.write(&inside, b"second"), Ok(()));
    assert_eq!(filesystem.read(&inside), Ok(b"second".to_vec()));
    assert_eq!(filesystem.restore(checkpoint), Ok(()));
    assert_eq!(filesystem.read(&inside), Ok(b"first".to_vec()));
}

#[test]
fn the_filesystem_fake_refuses_outside_its_own_notion_of_scope() {
    // The other direction of the same exercise, the one that gets forgotten (§7.1.1, rule 3):
    // the calls have to be callable AND refusable. ⚠️ WHAT REFUSES IS THE FAKE, not the port:
    // the kernel does not interpret paths, so membership in a scope is the implementation's
    // decision. This test pins that the fake is consistent with itself, nothing more.
    let mut filesystem = InMemoryFilesystem::default();
    let outside = Path::new(b"elsewhere/b.txt".to_vec());

    filesystem
        .declare_scope(&[Path::new(b"scope/".to_vec())])
        .expect("the scope was accepted");

    assert_eq!(
        filesystem.write(&outside, b"x"),
        Err(FilesystemError::OutsideScope)
    );
    assert_eq!(
        filesystem.read(&outside),
        Err(FilesystemError::OutsideScope)
    );
    assert_eq!(
        filesystem.preserve(StepId::new(2), &outside),
        Err(FilesystemError::OutsideScope)
    );
    assert_eq!(
        filesystem.restore(CheckpointId::new(99)),
        Err(FilesystemError::Missing)
    );
}

#[test]
fn two_spellings_of_one_file_are_two_paths() {
    // ⛔ `Path`'s comment, made executable instead of left as prose. On a case-insensitive
    // volume these two name ONE file; here they are two different byte strings and nothing in
    // `kernel` can tell. Whoever implements the port decides what "the same file" means — the
    // kernel cannot, and comparing two `Path` with `==` is comparing bytes, not files.
    assert_ne!(
        Path::new(b"scope/A.txt".to_vec()),
        Path::new(b"scope/a.txt".to_vec())
    );
}

#[test]
fn the_network_port_can_be_implemented_and_called() {
    let mut network = RecordingNetwork::default();
    let endpoint = Endpoint::new(b"the-only-way-out".to_vec());

    let answer = network
        .request(&endpoint, b"body")
        .expect("the fake answered");
    assert_eq!(answer, b"the-only-way-outbody".to_vec());
    assert_eq!(network.sent, vec![(endpoint, b"body".to_vec())]);

    // And refusable, same rule 3 as above.
    assert_eq!(
        network.request(&Endpoint::new(Vec::new()), b"body"),
        Err(NetworkError::Unreachable)
    );
}

#[test]
fn the_process_port_can_be_implemented_and_called() {
    let mut worker = ScriptedWorker::new();

    // One instruction, one answer, and the answer NAMES the receipt that asked for it.
    let receipt = worker
        .instruct_one(Frame::new(b"describe".to_vec()))
        .expect("the instruction was accepted");
    let answer = worker.read_one(receipt).expect("the worker answered");
    assert_eq!(answer, Frame::new(id_bytes(1)));

    // A stream: the receipt STAYS OPEN across reads, which is the shape the audio worker
    // of §6.10.1 needs. Three bytes in the instruction, three frames back.
    let mut stream = worker
        .instruct_stream(Frame::new(b"abc".to_vec()))
        .expect("the stream instruction was accepted");
    assert_eq!(
        worker.read_next(&mut stream),
        Ok(Some(Frame::new(id_bytes(2))))
    );
    assert_eq!(
        worker.read_next(&mut stream),
        Ok(Some(Frame::new(id_bytes(2))))
    );
    assert_eq!(
        worker.read_next(&mut stream),
        Ok(Some(Frame::new(id_bytes(2))))
    );
    // ⛔ AND THIS IS THE REFUSING HALF OF THE SAME METHOD (§7.1.1 rule 3): a stream that has
    // run dry answers `Ok(None)`, which is NOT an error -- ending is lawful. A fake that
    // errored here would make "the stream ended" indistinguishable from "the worker broke".
    assert_eq!(worker.read_next(&mut stream), Ok(None));

    worker.close(stream).expect("the stream closed");
}

#[test]
fn the_process_fake_refuses_where_it_must() {
    // The direction that gets forgotten, and the one that counts (§7.1.1 rule 3, gotcha #24).
    // ⚠️ WHAT REFUSES IS THE FAKE, not the port -- same caveat as the filesystem above: the
    // kernel exchanges bytes and does not read them, so "this frame is malformed" is the
    // implementation's judgement. What the port fixes is that the VOCABULARY exists.
    let mut worker = ScriptedWorker::new();

    // ⚠️ `unwrap_err` AND NOT `assert_eq!` ON THE WHOLE `Result`, and the reason is a
    // decision rather than a style: comparing the `Result` would demand `PartialEq` on the
    // receipts, and a derive added so that a test can use a nicer macro is a derive without
    // a caller -- the very thing the short derive lists on `Path`, `StepId` and
    // `CheckpointId` refuse. The error vocabulary is what these lines are about anyway.
    assert_eq!(
        worker.instruct_one(Frame::new(Vec::new())).unwrap_err(),
        ProcessError::MalformedFrame
    );
    assert_eq!(
        worker.instruct_stream(Frame::new(Vec::new())).unwrap_err(),
        ProcessError::MalformedFrame
    );

    // A receipt this worker never issued is a FAULT and not an empty answer (§6.10.1): the
    // frame has no way of being named, so it is not data.
    //
    // ⛔ AND A REAL STREAM IS OPENED FIRST, which is the whole point of the line below.
    // Measured: without it this probe ran against an EMPTY stream table, where "this id is
    // unknown" and "there are no streams at all" are indistinguishable -- so a fake that had
    // stopped looking at identities altogether still answered `UnsolicitedFrame` here, for
    // the wrong reason (gotcha #24). With a genuine stream open, only a lookup BY IDENTITY
    // can still refuse the forged one.
    let mut genuine = worker
        .instruct_stream(Frame::new(b"real".to_vec()))
        .expect("a genuine stream is open alongside the forged receipt");

    let mut forged = StreamReceipt::new(4242);
    assert_eq!(
        worker.read_next(&mut forged),
        Err(ProcessError::UnsolicitedFrame)
    );
    assert_eq!(worker.close(forged), Err(ProcessError::UnsolicitedFrame));
    // ...and the genuine one is untouched by the two refusals above.
    assert_eq!(
        worker.read_next(&mut genuine),
        Ok(Some(Frame::new(id_bytes(1))))
    );

    // And once the worker has died on its own, EVERY path refuses -- including a receipt
    // that was perfectly valid a moment ago. That is what "a worker can be killed without
    // warning" costs the caller.
    let live = worker
        .instruct_one(Frame::new(b"describe".to_vec()))
        .expect("the instruction was accepted while alive");
    worker.die();
    assert_eq!(worker.read_one(live), Err(ProcessError::Died));
    assert_eq!(
        worker
            .instruct_one(Frame::new(b"describe".to_vec()))
            .unwrap_err(),
        ProcessError::Died
    );
    assert_eq!(
        worker.read_next(&mut StreamReceipt::new(1)),
        Err(ProcessError::Died)
    );
}

#[test]
fn answers_are_correlated_to_the_receipt_that_asked() {
    // ⛔ THE TEST THE SUITE WAS MISSING, AND ITS ABSENCE WAS MEASURED, not suspected. Every
    // other test here keeps ONE receipt open at a time, and against a single open receipt
    // "answer the right one" and "answer the only one" are the same sentence. Measured: with
    // the id lookup replaced by "take stream 0" and both readers answering a constant, the
    // fake contained ZERO calls to `receipt.id()` and all 8 tests stayed GREEN. A fake that
    // does not correlate AT ALL satisfied the whole suite -- so the argument that keeps
    // `SingleReceipt::id` alive rested on nothing. This test is what makes it rest on
    // something.
    let mut worker = ScriptedWorker::new();

    // TWO single receipts open at once, read in the REVERSE order. Reading them in issue
    // order would pass against a fake that simply answers a queue.
    let first = worker
        .instruct_one(Frame::new(b"one".to_vec()))
        .expect("the first instruction was accepted");
    let second = worker
        .instruct_one(Frame::new(b"two".to_vec()))
        .expect("the second instruction was accepted");
    assert_eq!(worker.read_one(second), Ok(Frame::new(id_bytes(2))));
    assert_eq!(worker.read_one(first), Ok(Frame::new(id_bytes(1))));

    // TWO streams open at once, of DIFFERENT lengths, read INTERLEAVED. Different lengths
    // are load-bearing: with equal budgets a reader that served the wrong stream would run
    // dry at the same moment and nothing would notice.
    //
    // ⛔ AND THE ORDER HERE IS CHOSEN, NOT INCIDENTAL. `long` is opened FIRST so it sits at
    // position 0 of the fake's table, and the stream this test closes is `short`, at
    // position 1. Measured: with `short` opened first, closing it removed position 0 either
    // way and a `close` keyed on position instead of identity SURVIVED. The adversarial
    // ordering is what makes the closure below discriminate.
    let mut long = worker
        .instruct_stream(Frame::new(b"xxx".to_vec()))
        .expect("the long stream was accepted");
    let mut short = worker
        .instruct_stream(Frame::new(b"x".to_vec()))
        .expect("the short stream was accepted");

    assert_eq!(
        worker.read_next(&mut short),
        Ok(Some(Frame::new(id_bytes(4))))
    );
    assert_eq!(
        worker.read_next(&mut long),
        Ok(Some(Frame::new(id_bytes(3))))
    );
    // The short one is dry while the long one is not: a reader keyed on position instead of
    // identity ends the wrong stream here.
    assert_eq!(worker.read_next(&mut short), Ok(None));

    // ⛔ AND CLOSING THE DRY ONE MUST NOT TOUCH THE OTHER. A `close` that removed by position
    // would take `long` out from under the reads below -- which would then fail as
    // `UnsolicitedFrame`, a different symptom from a wrong value and one nothing else here
    // would catch.
    worker.close(short).expect("the short stream closed");

    assert_eq!(
        worker.read_next(&mut long),
        Ok(Some(Frame::new(id_bytes(3))))
    );
    assert_eq!(
        worker.read_next(&mut long),
        Ok(Some(Frame::new(id_bytes(3))))
    );
    assert_eq!(worker.read_next(&mut long), Ok(None));
    worker.close(long).expect("the long stream closed");
}

#[test]
fn killing_a_worker_consumes_it() {
    let mut worker = ScriptedWorker::new();
    let receipt = worker
        .instruct_one(Frame::new(b"describe".to_vec()))
        .expect("the instruction was accepted");
    let _ = worker.read_one(receipt).expect("the worker answered");

    // ⛔ WHAT THIS BUYS: that `kill(self)` is CALLABLE at all. A trait method taking `self`
    // by value is not free -- it makes the trait non-object-safe for that method, and a
    // signature one cannot call is exactly the class of defect this file exists to catch.
    // Killing is ALWAYS LAWFUL (§5.3 point 4), so this direction has no refusing twin.
    assert_eq!(worker.kill(), Ok(()));

    // ⛔ AND KILLING A WORKER ALREADY DEAD IS LAWFUL TOO, which is the half that was DECLARED
    // and not tested. Measured: before this line, adding `self.alive()?` to `kill` -- turning
    // the one lawful-always operation into one that refuses -- left all 9 tests GREEN. Four
    // lines of comment asserting the exception, and nothing holding it: an invariant nobody
    // can check is an intention (§5.3 point 4).
    let mut already_dead = ScriptedWorker::new();
    already_dead.die();
    assert_eq!(already_dead.kill(), Ok(()));

    // ⚠️ WHAT IT DOES NOT BUY: that instructing after the kill FAILS TO COMPILE. `worker` is
    // moved by the line above and naming it again would not build -- but a test that
    // compiles cannot assert that something else does not. Only a `compile_fail` case
    // proves the negative, and those are staged with the rest of §6.10.5: all four need a
    // `Worker`, a `Worker` comes only from `start(grant, ..)`, and nothing issues grants
    // before milestone 5. Registered as not-yet-covered in `docs/porta-di-qualita.md`.
}

#[test]
fn the_process_port_is_implementable_but_start_is_not_callable() {
    // ⛔ THE DECLARED LIMIT, written here rather than left to be discovered. `Process` is
    // IMPLEMENTABLE from outside the crate -- naming `Grant` in the signature is all it
    // takes, and the impl above compiles -- but `start` cannot be CALLED by anyone, here or
    // anywhere else, because `Grant` has no public constructor and no issuer until the
    // arbiter arrives in milestone 5 (§5.6). That is the half of I2 that belongs to the
    // compiler, working exactly as intended.
    //
    // ⛔ AND THESE TWO LINES ARE NOT WHAT BUYS THAT -- said in full rather than half, because
    // a test that looks like coverage and is not is worse than no test (gotcha #45). WHAT
    // BUYS IT IS THE `impl Process for SpawningProcess` BLOCK ABOVE: that is what the
    // compiler checks. Measured: delete THESE TWO LINES and the remaining tests still pass,
    // with nothing left but a "struct never constructed" warning -- and `gate.sh` does not
    // pass `-D warnings`. They are a CONSTRUCTION SITE for that warning and a place to hang
    // this comment; they assert nothing the impl has not already proved. Kept for the same
    // reason the conformance suite kept a line proving mere callability: declared as such,
    // instead of a `let _ = ...` that would read as coverage.
    //
    // ⚠️ THE SCOPE OF THAT MEASUREMENT IS "THESE TWO LINES", NOT "THIS FUNCTION", and the
    // difference is not pedantry -- the sentence used to say "this whole function" and had
    // gone false underneath itself. It was measured when the function held only the two
    // lines below; the round-trip block that follows was added afterwards, and M10 in
    // `docs/porta-di-qualita.md` dies with IT. Deleting the function today would take the
    // only coverage `WorkerDescriptor` has. ⛔ A true measurement can rot while the code it
    // describes grows under it: tie the claim to WHAT WAS MEASURED, never to the container
    // that happens to hold it.
    let process = SpawningProcess { started: 0 };
    assert_eq!(process.started, 0);

    // ⛔ AND THE ROUND-TRIP THAT NOTHING ELSE PERFORMS. `WorkerDescriptor::new` and
    // `as_bytes` were the only public items in `process.rs` with NO caller anywhere: `Path`,
    // `Endpoint` and `Frame` all get theirs exercised, these two could not, because the only
    // call that would pass a descriptor is `start`. They live under the gotcha #46 exception
    // -- an item kept alive for the implementation that cannot exist yet -- and the exception
    // was being INVOKED for them rather than exercised. Two lines close that: without
    // `as_bytes` a `platform` implementation could never hand the descriptor to the OS,
    // exactly as for `Path`.
    let raw = b"C:/workers/asr.exe".to_vec();
    let descriptor = WorkerDescriptor::new(raw.clone());
    assert_eq!(descriptor.as_bytes(), &raw[..]);
    // And the kernel does not interpret it: two spellings are two descriptors, `Path`'s rule
    // applying to the same kind of opaque byte string.
    assert_ne!(
        WorkerDescriptor::new(b"asr.exe".to_vec()),
        WorkerDescriptor::new(b"ASR.EXE".to_vec())
    );
}
