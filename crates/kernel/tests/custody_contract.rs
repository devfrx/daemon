// THE CONFORMANCE SUITE OF THE `custody` PORT (§2 of the GUI north star). It is worth what the
// two implementations answering the SAME questions is worth: the in-memory double that the DST
// campaign runs against, and `redb` under `platform`.
//
// ⛔ REGULAR COMMENTS AND NOT `//!`, BECAUSE THIS FILE IS `include!`d.
// `crates/platform/tests/custody_contract_real.rs` expands it IN ITEM POSITION, and an inner
// attribute -- which is what `//!` desugars to -- is not permitted there. Same mechanism as
// `journal_contract.rs`, and for the same reason: two copies of the assertions would drift, and
// THE FIRST ONE TO DRIFT WOULD PRINT `ok` WHILE COMPARING NOTHING.
//
// ⛔ A FUNCTION AND NOT A `macro_rules!`, WHICH IS THE OTHER SHAPE IN THIS VERY MILESTONE, so the
// difference is written down rather than left to look like an inconsistency somebody should
// tidy. `ipc_contract.rs` generates its `#[test]`s from a macro because EACH CRATE HANDS ITS OWN
// FACTORY: its real promises need a peer writing bytes, which a fake has not got. `custody` has
// no peer -- one factory is enough, and `kernel` builds `simulator::custody::MemoryCustody`
// itself, because `simulator` is already one of its dev-dependencies. Shape borrowed from
// `journal_contract.rs`, which the north star names by name.
//
// ⛔ WHAT THIS SUITE CANNOT HOLD, AND IT IS MEASURED RATHER THAN SUSPECTED: AN IMPLEMENTATION
// THAT IGNORES THE KEY. `CustodyKey` has ONE variant, so on an archive holding one key "the
// package under THIS key" and "the only package there is" ARE THE SAME PACKAGE -- by
// construction, not by how this bench is written. A `retrieve` that never looked at its argument
// would pass every promise below. The journal closed the twin defect (finding AUD-019) by
// putting a BYSTANDER in the archive; here a bystander IS NOT BUILDABLE.
// ⚠️ MEASURED, not reasoned: task 5 wrote the key-blind mutation, ran this suite against it and
// WATCHED IT PASS, then revoked it -- the record is in that task's commit.
// ⛔ THE TRIGGER IS THE SECOND KEY: whoever adds a variant to `CustodyKey` adds a bystander here
// in the same commit, and this paragraph goes. A second variant added without it would leave the
// port's central promise unheld and nothing would go red. Written here because nobody would
// rediscover it.
//
// ⚠️ WHAT IS DELIBERATELY ABSENT, the other half: SURVIVING A REOPENING. It is a promise of the
// REAL implementation alone -- the in-memory double cannot make it and is CORRECT not to, and
// asserting it here would turn a correct implementation red (gotcha #44). It lives in
// `crates/platform/tests/file_custody.rs`.

use kernel::ports::custody::{Custody, CustodyError, CustodyKey};

/// ⛔ ONE MESSAGE PER PROMISE, AND NOT ONE SHARED -- the rule `journal_contract.rs` states and
/// `reactor_contract.rs` learned the hard way: with a shared message a liar caught by promise 1
/// would be indistinguishable from one caught by promise 4, in exactly the place built to tell
/// them apart. They share `custody contract violated: ` and diverge immediately after, because
/// the negative tests match with `contains`.
pub const KEPT_COMES_BACK_MESSAGE: &str =
    "custody contract violated: what `keep` wrote must come back from `retrieve` byte for byte";

/// Promise 1 of §2 of the north star, in one line: the package is OPAQUE.
pub const OPAQUE_MESSAGE: &str =
    "custody contract violated: bytes that are not text must be kept and handed back untouched";

pub const NOTHING_KEPT_MESSAGE: &str =
    "custody contract violated: a key with nothing under it answers Ok(None), never an error";

pub const REPLACES_MESSAGE: &str =
    "custody contract violated: a second `keep` under one key must REPLACE what was there";

/// The gotcha #30 family: a bench that only looks at `Ok`/`Err` does not see the WRONG ANSWER.
pub const EMPTY_IS_NOT_ABSENT_MESSAGE: &str =
    "custody contract violated: an empty package is KEPT, and must not come back as nothing";

/// Every promise the `custody` port makes, checked against ONE implementation.
///
/// It takes a FACTORY and not a custody because several blocks need one that has never been
/// written to, and `keep` has no undo.
///
/// ⛔ THE ORDER OF THE BLOCKS IS PART OF THE SUITE, because it stops at the FIRST promise an
/// implementation breaks. Every liar below therefore has to survive every promise ahead of its
/// own and die on that one -- which is the property each negative test measures by reading the
/// panic payload instead of settling for `is_err()`.
pub fn assert_custody_contract<C: Custody, F: Fn() -> C>(build: F) {
    // ── 1. What `keep` writes, `retrieve` hands back byte for byte ────────────────────────
    {
        let mut custody = build();
        let written: &[u8] = br#"{"grid":{"root":{"type":"branch"}},"activeView":"home"}"#;

        custody
            .keep(CustodyKey::Layout, written)
            .expect(KEPT_COMES_BACK_MESSAGE);

        let read = custody
            .retrieve(CustodyKey::Layout)
            .expect(KEPT_COMES_BACK_MESSAGE);

        assert_eq!(read.as_deref(), Some(written), "{}", KEPT_COMES_BACK_MESSAGE);
    }

    // ── 2. Bytes that are not text survive unchanged ──────────────────────────────────────
    // ⛔ THIS IS PIECE 1 OF §2 OF THE NORTH STAR MADE EXECUTABLE: "the day `dockview` changes
    // format, the core does not change". An implementation that parsed, validated or
    // canonicalised the package would satisfy promise 1 -- the payload there IS valid JSON and
    // valid UTF-8 -- and die here. That is why the two blocks are not one.
    //
    // ⚠️ THE PAYLOAD IS CHOSEN, NOT RANDOM: `0xff` and `0x80` are not valid UTF-8 in any
    // position, `0x00` is what a C string stops at, and the `{` in the middle is there so that
    // something looking for JSON finds a plausible start and then fails.
    {
        let mut custody = build();
        let written: &[u8] = &[0xff, 0x00, 0x1b, b'{', 0xfe, b'\n', 0x80, 0x7f];

        custody
            .keep(CustodyKey::Layout, written)
            .expect(OPAQUE_MESSAGE);

        let read = custody.retrieve(CustodyKey::Layout).expect(OPAQUE_MESSAGE);

        assert_eq!(read.as_deref(), Some(written), "{}", OPAQUE_MESSAGE);
    }

    // ── 3. A key with nothing under it answers Ok(None) ───────────────────────────────────
    // ⛔ `Ok(None)` AND NOT AN ERROR, and the port's doc argues why: this is the FIRST RUN, the
    // ordinary case, and folding it into an error would make the commonest path look like a
    // failure. The assertion is on the WHOLE `Result`, so an implementation answering
    // `Err(Unavailable)` dies here instead of being quietly unwrapped away.
    {
        let custody = build();
        assert_eq!(
            custody.retrieve(CustodyKey::Layout),
            Ok(None),
            "{}",
            NOTHING_KEPT_MESSAGE
        );
    }

    // ── 4. A second `keep` REPLACES ───────────────────────────────────────────────────────
    // ⛔ AND THE TWO PAYLOADS DIFFER IN LENGTH AS WELL AS IN CONTENT, deliberately: an
    // implementation that APPENDED and handed back the concatenation would be caught by the
    // content alone, but one that kept the longer of the two would not -- so the replacement is
    // the SHORTER one. The same care promise 1 of `journal_contract.rs` takes with its bystander.
    {
        let mut custody = build();
        let first: &[u8] = b"the layout as it was when the window opened";
        let second: &[u8] = b"and as it is now";

        custody.keep(CustodyKey::Layout, first).expect(REPLACES_MESSAGE);
        custody
            .keep(CustodyKey::Layout, second)
            .expect(REPLACES_MESSAGE);

        let read = custody.retrieve(CustodyKey::Layout).expect(REPLACES_MESSAGE);

        assert_eq!(read.as_deref(), Some(second), "{}", REPLACES_MESSAGE);
    }

    // ── 5. An empty package is kept, and is NOT "nothing" ─────────────────────────────────
    // ⛔ THE DISTINCTION THAT AN `Option` INVITES YOU TO LOSE. An implementation holding
    // `Option<Vec<u8>>` and treating an empty slice as "no package" satisfies promises 1 to 4 --
    // none of them ever keeps an empty one -- and makes "kept, and empty" indistinguishable from
    // "never kept". That is the same family as gotcha #30, and the same sentence ADR-0018 spends
    // on pruned payloads: an absence and an emptiness must not look alike.
    //
    // ⚠️ AND IT IS NOT A HYPOTHETICAL SHAPE: it is the FIRST shape an in-memory double takes if
    // nobody says otherwise, which is why the liar for it is written below.
    {
        let mut custody = build();

        custody
            .keep(CustodyKey::Layout, b"")
            .expect(EMPTY_IS_NOT_ABSENT_MESSAGE);

        let read = custody
            .retrieve(CustodyKey::Layout)
            .expect(EMPTY_IS_NOT_ABSENT_MESSAGE);

        assert_eq!(
            read.as_deref(),
            Some(&b""[..]),
            "{}",
            EMPTY_IS_NOT_ABSENT_MESSAGE
        );
    }
}

#[test]
fn the_in_memory_custody_honours_the_contract() {
    assert_custody_contract(simulator::custody::MemoryCustody::new);
}

// ⛔ THE DIRECTION ONE FORGETS (§7.1.1 rule 3): a suite never seen to fail is not a suite. The
// five below break the port's promises ONE EACH, and demand that the suite notices each -- and
// notices it ON THE RIGHT PROMISE, which is what reading the payload buys over `is_err()`.
//
// ⚠️ AND EACH IS BROKEN IN A DIFFERENT WAY (gotcha #45): the write dropped, the bytes
// canonicalised, an absence reported as a failure, a replacement turned into an append, and an
// empty package filed as no package. Two liars broken the same way prove one thing twice and
// leave the other promise unguarded.
//
// ⛔ AND THE ONE THAT IS NOT HERE: a custody BLIND TO THE KEY. It would pass, and the head of
// this file says why and records the measurement. Writing it and asserting it is caught would be
// a green that proves nothing.

#[test]
fn a_custody_that_writes_nothing_is_caught() {
    assert_caught_on(SilentCustody::new, KEPT_COMES_BACK_MESSAGE, "promise 1");
}

#[test]
fn a_custody_that_canonicalises_the_bytes_is_caught() {
    assert_caught_on(TextCustody::new, OPAQUE_MESSAGE, "promise 2");
}

#[test]
fn a_custody_that_errors_instead_of_answering_nothing_is_caught() {
    assert_caught_on(
        ErrorInsteadOfNothingCustody::new,
        NOTHING_KEPT_MESSAGE,
        "promise 3",
    );
}

#[test]
fn a_custody_that_appends_instead_of_replacing_is_caught() {
    assert_caught_on(AppendingCustody::new, REPLACES_MESSAGE, "promise 4");
}

#[test]
fn a_custody_that_files_an_empty_package_as_no_package_is_caught() {
    assert_caught_on(
        EmptyIsAbsentCustody::new,
        EMPTY_IS_NOT_ABSENT_MESSAGE,
        "promise 5",
    );
}

#[test]
fn no_promise_message_is_a_substring_of_another() {
    // ⛔ THE CONSTRAINT THAT MAKES `contains` SAFE. If one message were a substring of another, a
    // liar caught on the WRONG promise would still satisfy the test that names the right one --
    // the suite would keep printing `ok` while pointing at the wrong place. It is a property of
    // the SET, so every message added has to be checked against ALL the others, which is exactly
    // the check nobody repeats by eye.
    let messages = [
        ("KEPT_COMES_BACK", KEPT_COMES_BACK_MESSAGE),
        ("OPAQUE", OPAQUE_MESSAGE),
        ("NOTHING_KEPT", NOTHING_KEPT_MESSAGE),
        ("REPLACES", REPLACES_MESSAGE),
        ("EMPTY_IS_NOT_ABSENT", EMPTY_IS_NOT_ABSENT_MESSAGE),
    ];

    for (name, message) in messages {
        for (other_name, other) in messages {
            if name == other_name {
                continue;
            }
            assert!(
                !other.contains(message),
                "{name} is a substring of {other_name}: a liar caught on {other_name} would \
                 satisfy the test that names {name}"
            );
        }
    }

    // The other direction, the one that gets forgotten (§7.1.1 rule 3): a bench where every
    // message were distinct BY BEING EMPTY would pass the loop above without saying anything.
    for (name, message) in messages {
        assert!(!message.is_empty(), "{name} is empty");
    }
}

fn assert_caught_on<C, F>(build: F, expected: &str, promise: &str)
where
    C: Custody,
    F: Fn() -> C + std::panic::RefUnwindSafe,
{
    let message = message_the_suite_fails_with(build).unwrap_or_else(|| {
        panic!("THE SUITE IS VACUOUS ON {promise}: a custody that breaks it passed the suite")
    });
    assert!(
        message.contains(expected),
        "the suite did fire, but NOT on {promise} — so {promise} is still unproven.\n\
         expected to contain: {expected}\n\
         actual payload: {message}"
    );
}

/// Runs the suite and returns the message it failed with, or `None` if it passed.
///
/// ⚠️ The panic hook is silenced for the duration of the call: the panic is EXPECTED, and its
/// backtrace in the test output would train the reader to ignore backtraces. Restored
/// immediately. ⛔ DECLARED LIMIT, the same one `journal_contract.rs` declares: the hook is
/// PROCESS-WIDE and libtest runs tests on parallel threads, so a panic raised in another test
/// landing inside this window is reported with no stdout section. The failure is never hidden,
/// only its message, and the window is microseconds wide.
fn message_the_suite_fails_with<C, F>(build: F) -> Option<String>
where
    C: Custody,
    F: Fn() -> C + std::panic::RefUnwindSafe,
{
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let outcome = std::panic::catch_unwind(|| assert_custody_contract(&build));
    std::panic::set_hook(previous);

    match outcome {
        Ok(()) => None,
        Err(payload) => Some(panic_message(payload.as_ref())),
    }
}

/// The text of a panic, dug out of the payload. `assert!`/`assert_eq!` with a format argument
/// panic with a `String`; a `panic!("literal")` carries a `&str` instead, and both are handled so
/// that this helper cannot report nothing for a message that is right there.
fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(text) = payload.downcast_ref::<String>() {
        text.clone()
    } else if let Some(text) = payload.downcast_ref::<&str>() {
        (*text).to_string()
    } else {
        String::from("<panic payload that is neither String nor &str>")
    }
}

/// Answers `Ok(())` and keeps nothing. Caught by promise 1.
struct SilentCustody;

impl SilentCustody {
    fn new() -> Self {
        SilentCustody
    }
}

impl Custody for SilentCustody {
    fn keep(&mut self, _key: CustodyKey, _bytes: &[u8]) -> Result<(), CustodyError> {
        Ok(())
    }
    fn retrieve(&self, _key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError> {
        Ok(None)
    }
}

/// Keeps only what is valid text and drops the rest. ⛔ THIS IS THE SHAPE AN IMPLEMENTATION
/// TAKES THE DAY SOMEBODY DECIDES THE PACKAGE "IS JSON ANYWAY": it survives promise 1, whose
/// payload is valid UTF-8, and dies on promise 2. Caught by promise 2.
struct TextCustody {
    kept: Option<Vec<u8>>,
}

impl TextCustody {
    fn new() -> Self {
        TextCustody { kept: None }
    }
}

impl Custody for TextCustody {
    fn keep(&mut self, _key: CustodyKey, bytes: &[u8]) -> Result<(), CustodyError> {
        self.kept = Some(String::from_utf8_lossy(bytes).into_owned().into_bytes());
        Ok(())
    }
    fn retrieve(&self, _key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError> {
        Ok(self.kept.clone())
    }
}

/// Keeps correctly, and reports an absence as a failure. Caught by promise 3.
struct ErrorInsteadOfNothingCustody {
    kept: Option<Vec<u8>>,
}

impl ErrorInsteadOfNothingCustody {
    fn new() -> Self {
        ErrorInsteadOfNothingCustody { kept: None }
    }
}

impl Custody for ErrorInsteadOfNothingCustody {
    fn keep(&mut self, _key: CustodyKey, bytes: &[u8]) -> Result<(), CustodyError> {
        self.kept = Some(bytes.to_vec());
        Ok(())
    }
    fn retrieve(&self, _key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError> {
        match &self.kept {
            Some(package) => Ok(Some(package.clone())),
            None => Err(CustodyError::Unavailable),
        }
    }
}

/// Piles the packages up and hands back the FIRST. ⛔ It sails through promises 1, 2 and 3,
/// which never keep twice, and the concatenating variant of the same defect would too. Caught by
/// promise 4.
struct AppendingCustody {
    kept: Vec<Vec<u8>>,
}

impl AppendingCustody {
    fn new() -> Self {
        AppendingCustody { kept: Vec::new() }
    }
}

impl Custody for AppendingCustody {
    fn keep(&mut self, _key: CustodyKey, bytes: &[u8]) -> Result<(), CustodyError> {
        self.kept.push(bytes.to_vec());
        Ok(())
    }
    fn retrieve(&self, _key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError> {
        Ok(self.kept.first().cloned())
    }
}

/// Files an empty package as no package. ⛔ THE SHAPE AN `Option` INVITES, and the reason
/// promise 5 exists: it answers every other promise correctly. Caught by promise 5.
struct EmptyIsAbsentCustody {
    kept: Option<Vec<u8>>,
}

impl EmptyIsAbsentCustody {
    fn new() -> Self {
        EmptyIsAbsentCustody { kept: None }
    }
}

impl Custody for EmptyIsAbsentCustody {
    fn keep(&mut self, _key: CustodyKey, bytes: &[u8]) -> Result<(), CustodyError> {
        self.kept = if bytes.is_empty() {
            None
        } else {
            Some(bytes.to_vec())
        };
        Ok(())
    }
    fn retrieve(&self, _key: CustodyKey) -> Result<Option<Vec<u8>>, CustodyError> {
        Ok(self.kept.clone())
    }
}
