//! The real `custody` (§2 of the GUI north star): a `redb` archive OF ITS OWN, on the
//! `FileBackend` the journal already uses.
//!
//! ⛔ ITS OWN FILE AND ITS OWN TABLE, NOT A SECOND TABLE IN THE JOURNAL'S. ADR-0022 separates
//! archives BY NATURE and gives each its own policy: the journal is encrypted, pruned and in the
//! backup; this is the "configuration" archive -- NOT encrypted, in the backup, permanent. Two
//! natures in one file would be one policy for both, and the retention milestone would owe this
//! package an exception written just for it. That is decision 15 of the north star in full.
//!
//! ⚠️ AND IT IS THE SAME `FileBackend`, WHICH IS NOT A CONTRADICTION: the backend is the boundary
//! at which level-2 faults are injected (ADR-0032 requirement 4), and sharing the TYPE is what
//! would make this archive injectable the day a campaign asks. What is not shared is the FILE.
//!
//! ⛔ NO DEPENDENCY IS ADDED, and it is measured rather than assumed: `redb` is already in this
//! crate's manifest for the journal, and `scripts/gate-deps.sh` measures the graphs of `kernel`
//! and `simulator`, neither of which sees it.
//!
//! ⛔ WHAT BOTH IMPLEMENTATIONS PROMISE IS NOT WRITTEN HERE: it is in
//! `crates/kernel/tests/custody_contract.rs`, reached by `include!` from
//! `crates/platform/tests/custody_contract_real.rs`. What only THIS one promises -- surviving a
//! reopening, and refusing a second custody on an open file -- is in
//! `crates/platform/tests/file_custody.rs`.

use std::path::Path;

use kernel::ports::custody::{Custody, CustodyError, CustodyKey};
// `ReadableTable` is NOT imported: in redb 4.1.0 `ReadOnlyTable::get` is inherent, so the import
// would be an unused-import warning at every build. The mutation of step 7 imports it where it
// needs `iter`, which IS the trait's (measured at the plan review, 2026-09-15).
use redb::{Database, ReadableDatabase, TableDefinition};

use crate::journal::{FileBackend, OpenError, engine};

/// The one table: one package per key.
///
/// ⚠️ `u8` AND NOT `&str`, and the reason is the port's rather than this file's: `CustodyKey` is
/// a CLOSED ENUM precisely so that the archive has no namespace, and a string key here would put
/// the namespace back where nobody would see it. `u8` is a `redb` `Key` -- read in
/// `redb-4.1.0/src/types.rs`, `le_impl!(u8)`, which gives both `Value` and `Key` with a NUMERIC
/// comparison -- so nothing about the ordering can surprise us.
const PACKAGES: TableDefinition<u8, &[u8]> = TableDefinition::new("custody-packages");

/// The byte `CustodyKey::Layout` is stored under.
const KEY_LAYOUT: u8 = 0;

/// ⛔ A `match` AND NOT `key as u8`, AND THE DIFFERENCE IS THE COMPILER. A cast would give a
/// SECOND variant its byte BY POSITION, so reordering the enum would silently repoint every
/// package already on the disk -- a migration nobody asked for, arriving in silence. A `match`
/// makes a new variant an `E0004` right here, which is where that decision belongs.
fn byte_of(key: CustodyKey) -> u8 {
    match key {
        CustodyKey::Layout => KEY_LAYOUT,
    }
}

/// The `custody` port against a real file.
///
/// ⛔ NO `with_backend`, UNLIKE `FileJournal`, AND THE ABSENCE IS THE DECISION. The journal has
/// one because milestone 4 injects level-2 faults through it and
/// `crates/platform/tests/engine_crash_consistency.rs` really calls it. Here nobody would: the
/// `FileBackend` boundary is ALREADY proven substitutable from outside the crate by the
/// journal's bench, and what the simulation substitutes on this port is the WHOLE PORT -- piece
/// 4 of §2 of the north star. A second entry point with no caller is the speculative layer this
/// repository refuses. It arrives WITH its caller, or not at all.
pub struct FileCustody {
    database: Database,
}

impl FileCustody {
    /// Opens the archive at `path`, creating it if it is not there.
    ///
    /// ⛔ IT ANSWERS `platform::journal::OpenError`, AND THE ODD PATH IS DELIBERATE -- written
    /// here because whoever reads the signature is exactly who would ask. That type's three
    /// variants -- a file that would not open, a file another handle already holds, an engine
    /// that refused it -- are exactly what OPENING A `redb` ARCHIVE ON A `FileBackend` can
    /// produce, and this opens the same thing. The two alternatives were examined on 2026-09-11:
    /// MOVING it up to `platform::OpenError` would falsify two doc paragraphs IN THE KERNEL --
    /// `crates/kernel/src/framing.rs` and `crates/kernel/src/permission.rs` both name
    /// `platform::journal::OpenError` as the shape they follow -- and DUPLICATING it would leave
    /// two types to hold in step, the first to drift lying in silence. If a THIRD archive ever
    /// arrives, the type moves up and those two kernel paragraphs are corrected in the same
    /// commit; until then it stays where its callers can find it.
    ///
    /// ⚠️ THE LOCK COMES WITH `FileBackend` AND IS NOT RE-ARGUED HERE: a second custody on an
    /// open file is refused, which `crates/platform/tests/file_custody.rs` holds in BOTH
    /// directions.
    pub fn open(path: &Path) -> Result<Self, OpenError> {
        let backend = FileBackend::open(path)?;
        let database = Database::builder()
            .create_with_backend(backend)
            .map_err(engine)?;

        // ⛔ THE TABLE IS CREATED HERE, ON EVERY OPEN, so that every later READ finds it. A
        // `redb` table springs into existence when a WRITE transaction opens it; a read
        // transaction on a fresh file would answer `TableDoesNotExist`, and `retrieve` would have
        // to launder that into `Unavailable` -- reporting a broken archive where the truth is a
        // FIRST RUN, which is exactly the confusion promise 3 exists to forbid. The journal does
        // the same, for the same reason. ⚠️ DECLARED COST: opening always commits, so `open`
        // writes to the disk even when nothing is ever kept.
        let transaction = database.begin_write().map_err(engine)?;
        transaction.open_table(PACKAGES).map_err(engine)?;
        transaction.commit().map_err(engine)?;

        Ok(FileCustody { database })
    }
}

impl Custody for FileCustody {
    fn keep(&mut self, key: CustodyKey, bytes: &[u8]) -> Result<(), CustodyError> {
        // ⛔ ONE VARIANT FOR EVERY FAILURE, AND IT IS THE PORT'S DECISION AND NOT A SHORTCUT:
        // `CustodyError` has one variant because the caller reads the reason FROM WHICH
        // OPERATION FAILED, and the argument is written out on the type. Nothing is lost here
        // that the caller could use.
        let transaction = self
            .database
            .begin_write()
            .map_err(|_| CustodyError::Unavailable)?;
        {
            let mut table = transaction
                .open_table(PACKAGES)
                .map_err(|_| CustodyError::Unavailable)?;
            table
                .insert(byte_of(key), bytes)
                .map_err(|_| CustodyError::Unavailable)?;
        }
        // ⛔ THE HANDLE IS DROPPED BEFORE THE COMMIT, which the block above is for: `redb`
        // refuses to commit with a table handle still live, and the journal's
        // `abandon_without_commit` carries the twin of this note about `TableAlreadyOpen`.
        transaction
            .commit()
            .map_err(|_| CustodyError::Unavailable)?;
        Ok(())
    }

    fn retrieve(&self, key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError> {
        let transaction = self
            .database
            .begin_read()
            .map_err(|_| CustodyError::Unavailable)?;
        let table = transaction
            .open_table(PACKAGES)
            .map_err(|_| CustodyError::Unavailable)?;

        // ⛔ `Ok(None)` FOR A KEY WITH NOTHING UNDER IT, never an error -- promise 3. And the
        // bytes are copied out: the `AccessGuard` borrows the transaction, which dies here.
        Ok(table
            .get(byte_of(key))
            .map_err(|_| CustodyError::Unavailable)?
            .map(|found| found.value().to_vec()))
    }
}
