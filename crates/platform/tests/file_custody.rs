// WHAT ONLY THE REAL CUSTODY PROMISES: that a package survives the process, and that two
// handles on one file are refused. Neither is askable of the in-memory double, which is CORRECT
// not to make them -- asserting them in the conformance suite would turn a correct
// implementation red (gotcha #44).
//
// ⛔ WHAT IS NOT HERE, AND IT WAS EXAMINED RATHER THAN FORGOTTEN: a probe on the file's
// permission bits. `file_journal.rs` has one because ADR-0023 promises the JOURNAL is "protected
// as much as your system account is", and 0644 would be less. This archive is the
// "configuration" one of ADR-0022 -- NOT encrypted, in the backup -- so no ADR promises anything
// about its mode, and a probe without a source is a probe nobody can read. It gets 0600 anyway,
// because `FileBackend::open` sets it for every archive, and THAT is already held by
// `the_journal_file_is_not_world_readable`. Written down so the next census finds this absence
// explained instead of missing.

use std::path::Path;

use kernel::ports::custody::{Custody, CustodyKey};
use platform::custody::FileCustody;
use platform::journal::OpenError;

/// A directory of this test's own, emptied on entry. Same mechanism and same reasoning as
/// `file_journal.rs`; ⛔ the PREFIX differs from every other bench's, because a line number alone
/// does not keep two files apart.
fn private_dir_for_line(line: u32) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("daemon-file-custody-{line}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create the test directory");
    dir
}

fn archive_in(dir: &Path) -> std::path::PathBuf {
    dir.join("custody.redb")
}

#[test]
fn what_was_kept_survives_reopening_the_file() {
    // §2 of the north star, piece 3: "apri, scrivi, riapri, rileggi". It is the promise the
    // whole port exists for -- the layout has to outlive the window that drew it.
    //
    // ⛔ AND THE BYTES ARE NOT TEXT, on purpose: this way the probe holds the OPACITY across a
    // reopening too, which is the one place a format-aware implementation would still be free to
    // "tidy" the package -- on the way in, or on the way back out.
    let dir = private_dir_for_line(line!());
    let path = archive_in(&dir);
    let written: &[u8] = &[0xff, b'{', 0x00, 0x9c, b'}'];

    {
        let mut custody = FileCustody::open(&path).expect("open");
        custody
            .keep(CustodyKey::Layout, written)
            .expect("keep must succeed");
    }

    let reopened = FileCustody::open(&path).expect("reopen");
    assert_eq!(
        reopened.retrieve(CustodyKey::Layout),
        Ok(Some(written.to_vec())),
        "a package kept before the handle was dropped must be there after it is opened again"
    );
}

#[test]
fn a_second_custody_on_an_open_file_is_refused() {
    // ⛔ THE LOCK IS NOT AN EXTRA: two writers on one archive is the corruption the refusal
    // exists to prevent, and `redb`'s own backend takes one, so a replacement that did not would
    // drop a guarantee IN SILENCE.
    let dir = private_dir_for_line(line!());
    let path = archive_in(&dir);

    let _first = FileCustody::open(&path).expect("the first open must succeed");

    match FileCustody::open(&path) {
        Err(OpenError::AlreadyOpen) => {}
        Err(other) => panic!("refused, but for the wrong reason: {other:?}"),
        Ok(_) => panic!("a second custody on an open file must be refused"),
    }
}

#[test]
fn a_second_custody_on_a_closed_file_is_not_refused() {
    // ⛔ THE DIRECTION ONE FORGETS (§7.1.1 rule 3). Without it, an `open` that refused ALWAYS
    // would satisfy the test above -- and the port would be unusable for the reason it exists,
    // which is being reopened at every start-up.
    let dir = private_dir_for_line(line!());
    let path = archive_in(&dir);

    {
        let _first = FileCustody::open(&path).expect("the first open must succeed");
    }

    FileCustody::open(&path).expect("a custody on a closed file must open");
}
