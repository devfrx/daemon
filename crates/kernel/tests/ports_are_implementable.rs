//! One fake per port declared WITHOUT an implementation, and calls that exercise them
//! (§2.3, gotcha #17).
//!
//! ⛔ WHAT THIS BUYS: that the signatures of `Filesystem` and `Network` are IMPLEMENTABLE and
//! CALLABLE. A trait nobody implements has not been proved implementable — a signature can be
//! unusable for borrows, for ownership or for object safety, and one finds that out when the
//! code that has to use it already exists. It has already happened in this milestone: a
//! `Wakeup` enum declared in advance turned out to be unusable and was removed.
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
