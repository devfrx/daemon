//! The SEVENTH family of ports: keeping bytes the kernel never opens.
//!
//! ⛔ WHY A PORT AND NOT THE JOURNAL, IN ONE PARAGRAPH, because whoever reads this file is
//! exactly who would ask. The journal keeps "the small part" for ever (ADR-0018): permissions,
//! policy, approved guides -- SMALL DECISIONS. A panel layout is written dozens of times a day
//! and would sit there for ever, encrypted and in the backup, and the retention milestone would
//! owe it an exception written just for it. ADR-0022 had already decided that "configuration"
//! is an archive of its own; THIS PORT IS THAT ARCHIVE, in its smallest form. The full argument
//! is decision 15 of the GUI north star.
//!
//! ⛔ AND IT IS NOT CONFIGURATION OF THE KERNEL, WHICH IS THE OTHER THING IT LOOKS LIKE. A value
//! the kernel DECIDES with is handed to it (ADR-0034, and its negative perimeter says the kernel
//! is not a configuration system). Here the kernel CUSTODIES what the gui entrusts to it and
//! never reads it to decide anything -- it cannot, because the package is opaque. If a future
//! caller wants to READ what is kept here in order to decide, that is ADR-0034's question and
//! not this port's.
//!
//! ⛔ NO PATH AND NO FILE NAME IS NAMEABLE HERE (I3). The key is a closed enum; where the bytes
//! land is `platform`'s business, and the simulator substitutes the whole thing.
//!
//! ⚠️ ONE KEY TODAY, AND THAT IS NOT A SYSTEM WAITING TO HAPPEN. Checked before deciding
//! whether others would need this: approved guides, permissions and the VRAM policy are all
//! PROJECTIONS OF THE JOURNAL (ADR-0009), so the layout is the only PACKAGE in sight. Two
//! operations and one key are not a configuration system -- no format, no schema, no
//! validation, no hot reload.

use alloc::vec::Vec;

/// What is being kept. ⛔ A CLOSED ENUM AND NOT A STRING: a string key is a namespace, and a
/// namespace is the configuration system this port is deliberately not. A second thing to keep
/// is a VARIANT, added deliberately, which is the same shape ADR-0031 asks of a dependency.
///
/// ⚠️ `Eq` IS PROMISED AND NOT YET EXERCISED, AND IT IS DECLARED HERE RATHER THAN LEFT IMPLICIT.
/// Every other derive on this enum has a user in `crates/kernel/tests/ports_are_implementable.rs`
/// -- `Debug` and `PartialEq` in its assertions, `Clone` and `Copy` in the key it holds in a
/// variable and passes BY VALUE to both operations. `Eq` is a MARKER WITH NO METHODS: what
/// exercises it is a key of a map or of a set, and nothing has one. ⛔ IT STAYS BECAUSE IT IS A
/// CONTRACT, promised by name to tasks 5, 7, 9, 10 and 12, and that is the difference from `ipc`,
/// where three derives came OFF on the evidence that nobody had promised them. ✅ THE TRIGGER IS
/// TASK 5's CONFORMANCE SUITE, which compares two implementations: the day it uses it, this
/// paragraph goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustodyKey {
    /// The gui's layout: `toJSON()` of `dockview` plus the active view, as ONE opaque package.
    Layout,
}

/// What can go wrong. ⛔ ONE VARIANT, AND THE REASON IS THE RULE THIS REPOSITORY USES
/// EVERYWHERE: no caller, no variant. Distinguishing "the archive would not open" from "the
/// write was refused" would be two variants with ONE producer between them today, and the
/// caller does not need the distinction to be in the type -- IT READS IT FROM WHICH OPERATION
/// FAILED. `keep` fails and `retrieve` answers: the write was refused, and the activity sends
/// back the old package. Both fail: the archive is unavailable, and the activity says so
/// (decision 35 of the sub-project 2 design). Written here so the consumer does not rediscover it.
///
/// ⚠️ `Eq` IS PROMISED AND NOT YET EXERCISED HERE EITHER, for the reason written beside
/// `CustodyKey` and with the same trigger -- task 5's conformance suite. Said twice because a
/// reader who lands on this enum alone would otherwise have to go and find out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustodyError {
    /// The archive could not be reached -- it would not open, or the write did not land.
    Unavailable,
}

/// Keeping bytes under a key, and handing them back.
///
/// ⛔ TWO OPERATIONS AND NO DELETE. Nothing in the design asks to forget a layout: replacing it
/// is `keep` with other bytes, and "restore the default view" is presentation -- the gui drops
/// the saved entry (§2 of the north star, "examined and without a source today"). A third
/// operation with no caller would be the speculative layer criterion 5 refuses.
pub trait Custody {
    /// Keeps these bytes under this key, replacing whatever was there.
    ///
    /// ⛔ THE BYTES ARE OPAQUE AND STAY THAT WAY. No implementation may parse, validate or
    /// canonicalise them: the day `dockview` changes format, the core does not change. The
    /// probe that holds it is "bytes that are not JSON come back identical", and it lives with
    /// the implementations (task 5).
    fn keep(&mut self, key: CustodyKey, bytes: &[u8]) -> Result<(), CustodyError>;

    /// Hands back what is kept under this key, or `None` if nothing is.
    ///
    /// ⛔ `Ok(None)` AND NOT AN ERROR, AND IT IS A DELIBERATE DIVERGENCE FROM `Journal`, WHOSE
    /// `read_back` ANSWERS `Err(JournalError::Missing)`. The two are not the same question. A
    /// step the journal has no record of is a FAULT -- someone asked about a step that should be
    /// there. A key with nothing under it is the FIRST RUN, the ordinary case, and folding it
    /// into an error would make the commonest path look like a failure and push every caller to
    /// match an error variant to find out that everything is fine. ⚠️ AND IT IS WHAT THE DESIGN
    /// ALREADY ASKS FOR: `Layout` carries "the package, NOTHING, or unavailable", three states,
    /// and `Ok(None)` is the middle one arriving as a value rather than as a failure.
    fn retrieve(&self, key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError>;
}
