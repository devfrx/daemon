//! The `filesystem` port: checkpoint scopes and artefacts (§4, ADR-0024).
//!
//! A WORKING SCOPE is an explicitly declared set of paths; the checkpoint covers those
//! and nothing else. Before an effect touches a file inside a scope, the previous
//! version is PRESERVED and referred to by the journal step -- it is write-ahead applied
//! to files.
//!
//! ⛔ THE DECLARED LIMIT: effects outside the scopes are not covered. Distinct from git
//! and coexisting with it -- the checkpoint is automatic and step-grained, git is
//! intentional and commit-grained.
//!
//! ⚠️ Implementation STAGED (§0.4): the real filesystem belongs to a later sub-project.
//! The port is declared here because §3.1 declares the port list exhaustive and the
//! simulator substitutes all of them -- a port added after the campaign means C1 was
//! verified on a smaller world (gotcha #17).
//!
//! ⚠️ AND WHAT HOLDS THESE SIGNATURES TODAY IS ONE TEST, worth naming for what it is:
//! `tests/ports_are_implementable.rs` writes a fake for this trait and calls it. It buys
//! that the signatures are IMPLEMENTABLE FROM OUTSIDE THE CRATE and callable; it does NOT
//! buy that they are the right signatures -- the spec decides that -- and it is not the
//! conformance suite, which needs two implementations to compare and is born with the real
//! filesystem.

use alloc::vec::Vec;

use crate::ports::journal::StepId;

/// A path, as the kernel sees it: an opaque sequence of bytes.
///
/// ⛔ The kernel does NOT interpret paths: separators, drive letters, case sensitivity
/// and length limits are OS-specific, and I3 keeps them behind the platform module.
///
/// ⛔ AND THE CONSEQUENCE THAT SENTENCE DOES NOT DRAW, which is the one that bites: a kernel
/// that does not interpret paths CANNOT DECIDE WHETHER TWO `Path` DESIGNATE THE SAME FILE.
/// `==` here compares BYTES, not files -- on a case-insensitive volume two different byte
/// strings name one file, and nothing in this crate can tell. Therefore "inside a declared
/// scope" is a decision belonging to WHOEVER IMPLEMENTS THE PORT: the trait says that
/// `OutsideScope` exists and says nothing about how membership is computed. It looks obvious
/// until somebody compares two `Path` with `==` and believes they have compared two files --
/// which is why `two_spellings_of_one_file_are_two_paths` asserts it instead of leaving it
/// written here.
///
/// ⚠️ The derive list is short on purpose, and it is `StepId`'s argument applied again. No
/// ordering: nothing sorts these. No `Hash`: `HashMap` IS NOT NAMEABLE in this crate --
/// `tests/compile_fail/hashmap_in_kernel.rs`, gotcha #12 -- so `Hash` would be a derive whose
/// usual consumer cannot exist here. They come back the day something needs them, with the
/// caller that needs them. ⛔ `Clone` is NOT in that category: it is LOAD-BEARING for
/// `declare_scope`, which hands the implementation a borrowed slice it has to retain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path(Vec<u8>);

impl Path {
    pub fn new(raw: Vec<u8>) -> Self {
        Path(raw)
    }

    /// The bytes back out.
    ///
    /// ⚠️ Without this the trait would be UNIMPLEMENTABLE outside `kernel`: the privacy of a
    /// tuple-struct field is MODULE-scoped, so a `platform` implementation could never hand
    /// the path to the OS.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// The handle of a preserved version, referred to by a journal step.
///
/// ⚠️ OPAQUE, AND THERE IS NO GETTER. The plan dictated a `get() -> u64`, and it goes the way
/// `StepId::get()` went one task earlier: NOTHING READS THE NUMBER. The whole contract today is
/// that `preserve` hands one out and `restore` takes it back, and an opaque handle is what lets
/// an implementation encode in it whatever it needs -- "there was no file here before"
/// included. The getter comes back the day the durable record of §4.9 has to write it down,
/// with that caller.
///
/// ⚠️ Same short derive list as `Path`, for the same two reasons: nothing sorts these, and
/// `Hash` has no possible consumer where `HashMap` is not nameable (gotcha #12).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckpointId(u64);

impl CheckpointId {
    pub const fn new(value: u64) -> Self {
        CheckpointId(value)
    }
}

/// What can go wrong on the way to a file.
///
/// ⚠️ THE "NO CALLER, NO ITEM" RULE DOES NOT REACH THESE VARIANTS, and saying so is cheaper
/// than someone re-deriving it, because it looks like the rule that removed the `Wakeup` enum
/// from `reactor`. There the variant `EventReady` had NO POSSIBLE PRODUCER -- nothing in this
/// milestone generates external events. Here NO variant has a producer, for the plain reason
/// that the port has no implementation at all: applying the rule on that basis would empty the
/// enum instead of pruning it. Each of the three is a failure a real store really has.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemError {
    /// The path is outside every declared working scope. Fail-closed: a checkpoint that
    /// silently did not cover a file is worse than no checkpoint.
    OutsideScope,
    /// The underlying store refused.
    Unavailable,
    /// Nothing was found under that handle.
    Missing,
}

pub trait Filesystem {
    /// Declares a working scope. Only what is declared gets covered.
    fn declare_scope(&mut self, paths: &[Path]) -> Result<(), FilesystemError>;

    /// Preserves the current version of `path` before an effect touches it, and ties it
    /// to the journal step that is about to act.
    fn preserve(&mut self, step: StepId, path: &Path) -> Result<CheckpointId, FilesystemError>;

    /// Puts a preserved version back.
    ///
    /// ⚠️ It returns `()` and NOT the bytes, and that is not an omission: a restore is an
    /// EFFECT ON THE FILE. A caller that wanted the bytes in hand would be reading, and
    /// reading is `read`.
    fn restore(&mut self, checkpoint: CheckpointId) -> Result<(), FilesystemError>;

    /// Reads the content of a path inside a declared scope.
    fn read(&self, path: &Path) -> Result<Vec<u8>, FilesystemError>;

    /// Writes content to a path inside a declared scope.
    fn write(&mut self, path: &Path, content: &[u8]) -> Result<(), FilesystemError>;
}
