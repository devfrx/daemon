//! The boundary of untrusted data, IN THE TYPE SYSTEM (§6.5, ADR-0014, I6).
//!
//! > An instruction found in data is NEVER an authorisation. Untrusted content may
//! > INFORM, never AUTHORISE.
//!
//! ⛔ THERE IS NO SANITISATION. No attempt is made to strip instructions out of text:
//! external content travels in a type distinct from the one that carries instructions,
//! it is not assignable to an instruction field, and the conversion needs an explicit
//! step — which ADR-0014 requires to be JOURNALLED. What the type system delivers of that
//! requirement, and what it does not, is written out on `Untrusted::promote`.
//!
//! ⛔ THE LABEL IS HEREDITARY. Extracting, summarising, translating or concatenating
//! still produces untrusted content: otherwise a summary would be enough to launder an
//! attack.
//!
//! ⚠️ That is the RULE, from ADR-0014, and it is written in the plural while TODAY THERE IS
//! ONE DERIVATION — `summarize`. The other three are not written here: an API item with no
//! caller is deleted in this repository (the rule that already removed `Millis::ZERO` and a
//! `Wakeup` enum), and a derivation nobody calls would be exactly that. The rule is stated in
//! full anyway because it binds the ones that WILL come: each of them returns `Untrusted`,
//! and a derivation that returned `Instruction` would be the laundering ADR-0014 forbids.
//!
//! ⛔ The ban has TWO rules, each with its case in `tests/compile_fail/`: rule A — untrusted
//! content cannot be passed where an instruction is expected — by `untrusted_as_instruction.rs`;
//! rule B — no `From`/`Into` path leads from `Untrusted` to `Instruction` — by
//! `no_conversion_from_untrusted_to_instruction.rs`. So do NOT add
//! `impl From<Untrusted> for Instruction`: the SECOND case catches it by COMPILING, which
//! trybuild reports as a failure outright instead of through its oracle.
//!
//! ⛔ And it is the second and not the first, which was MEASURED: with that impl present the
//! rule A case stays `ok` — its output still matches its oracle exactly — so on I6 the direct
//! case is what stands between a conversion and a green gate. Measured a second time from the
//! other end: with that impl present AND the direct case removed, ALL SIX checks of the gate
//! come out green. The measurement, and why this pair behaves differently from the two times
//! of §2.1, is written in the two cases.
//!
//! ⛔ AND EACH RULE HAS ONE CASE HERE, WHERE THE TWO TIMES OF §2.1 HAVE TWO. It is an
//! asymmetry, not an omission, and the reason is written in the case itself: only one of the
//! two directions is dangerous.
//!
//! §2.5 brought these two types up from the spike with their substance unchanged. What
//! changed is the conversion: it now RECEIVES THE `journal` PORT (§6.5), the token device of
//! §6.3 applied to the boundary.
//!
//! ⛔ AND WHAT THAT BUYS IS EXACTLY ONE THING: the conversion cannot be written without
//! NAMING the port. It does NOT buy that anything was recorded — `promote` is generic over
//! any `Journal`, and one that answers `Ok(())` without writing a byte satisfies the bound.
//! The honesty of an implementation is not a level 1 property and never becomes one; it is
//! what a CONFORMANCE SUITE holds, the way `tests/reactor_contract.rs` holds the `Reactor`
//! with two deliberate liars. That suite for `journal` belongs to milestone 3, with the
//! durable record it would have to check against. Until it exists, this boundary rests on
//! the type system for the naming and on review for the rest — said here rather than implied,
//! because "the conversion is journalled" read as a guarantee is precisely the sentence this
//! paragraph replaced.

use core::fmt;

use alloc::string::String;
use alloc::vec::Vec;

use crate::ports::journal::{Journal, JournalError, StepId};

/// Content allowed to occupy the instruction channel.
///
/// ⚠️ `Debug` is DERIVED here and hand-written on `Untrusted`, and the asymmetry is the point:
/// an instruction is content we chose, so printing it leaks nothing that was not already ours.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction(String);

/// Content coming from an external source.
///
/// ⛔ `Debug` is NOT derived — see the hand-written impl below, and read it before "tidying"
/// this into a derive.
#[derive(Clone, PartialEq, Eq)]
pub struct Untrusted(String);

/// ⛔ THE CONTENT IS NOT PRINTED, and that is the whole implementation.
///
/// A derived `Debug` puts external text into every `{:?}` — a log line, a panic message, an
/// `assert_eq!` failure — and external text reaching the LOGS is the same class of problem as
/// external text reaching the instruction channel: content nobody chose, arriving somewhere it
/// is read as if somebody had. It is the reason this type exists at all.
///
/// It also closed a road out of the boundary, and that is why the impl is here rather than in
/// a "hardening" milestone: with the derive, `Instruction::new(format!("{:?}", untrusted))`
/// carried the text across intact, and nothing went red. Road A3 of the residual on
/// `Untrusted::promote`.
///
/// ⚠️ The length is kept deliberately. Diagnostics need to tell an empty payload from a large
/// one, and a byte count discloses nothing about the content. Pinned by
/// `the_debug_of_untrusted_does_not_print_the_content` — a closed road that no test holds is a
/// road that reopens the day somebody adds `Debug` back to the derive list, with the gate
/// staying green.
impl fmt::Debug for Untrusted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Untrusted(<{} bytes>)", self.0.len())
    }
}

impl Instruction {
    pub fn new(text: String) -> Self {
        Instruction(text)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Untrusted {
    pub fn new(raw: String) -> Self {
        Untrusted(raw)
    }

    /// ⚠️ NO PRODUCTION CALLER TODAY — only assertions use it, and that is recorded so a YAGNI
    /// pass does not remove it without knowing what it is doing. It stays because reading
    /// external content is what the system is FOR: the day a capability puts a fetched page
    /// into a prompt AS DATA, it reads it through here. ⛔ And removing it would close road
    /// A1/A2 of the residual below by accident rather than by decision — a road that shuts
    /// itself when an unrelated cleanup runs is not a closed road, it is a coincidence.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The ONE conversion path, and it takes the journal port.
    ///
    /// `reason` is meant to be recorded with the promotion: a promotion whose reason nobody
    /// wrote down is indistinguishable from one nobody thought about. ⚠️ MEANT TO BE — nothing
    /// here checks that `reason` is non-empty, and nothing checks that the implementation
    /// wrote it. Both belong to the conformance suite of milestone 3.
    ///
    /// ⚠️ Milestone 2 records the reason as raw bytes. The versioned record with
    /// explicit indices is §4.9, milestone 3, and this call site is one of the first it
    /// will change. Two things §4.9 inherits, declared here so they are not discovered there:
    ///
    /// - ⛔ THE RECORD DOES NOT CONTAIN THE PROMOTED TEXT, only the reason. So `read_back` of
    ///   this step cannot answer "what crossed the boundary at step 7?" — it answers "why did
    ///   somebody say it should". The provisional thing is the FORMAT; the missing thing is
    ///   the CONTENT, and those are two different gaps.
    /// - ⛔ THIS WRITES `intent` AND NEVER `outcome`, so by ADR-0007 the step it names is IN
    ///   DOUBT the moment it is written — and a step in doubt is never prunable. OPEN
    ///   QUESTION, left open rather than settled by whoever wrote the call: is a promotion a
    ///   STEP OF ITS OWN, which then owes its own outcome, or a note ON THE CALLER'S step,
    ///   which already has one? §4.9 decides it at milestone 3, and the answer changes this
    ///   signature either way.
    ///
    /// ⛔ DECLARED RESIDUAL, and it is long because it was MEASURED instead of reasoned: a
    /// review went looking for ways around this call and SEVEN OF THEM COMPILE TODAY. Neither
    /// this signature nor `promote_without_journal.rs` is to be read as more than it is.
    ///
    /// ⭐ First what HOLDS, because it holds better than it looks. The privacy of a
    /// tuple-struct field is MODULE-scoped and not `pub(crate)`: a SIBLING module inside
    /// `kernel` can neither build `Instruction(…)` nor read `Untrusted.0` — `E0423` and
    /// `E0616`, measured. The roads below are therefore the roads; there is no eighth hiding
    /// elsewhere in the crate.
    ///
    /// What IS covered, at level 1: this call cannot be made without naming the port —
    /// `promote_without_journal.rs` — and no `From`/`Into` road goes around it —
    /// `no_conversion_from_untrusted_to_instruction.rs`.
    ///
    /// ⛔ The roads that COMPILE, with the price of closing each, which is the part worth
    /// knowing. ⚠️ The heading used to read "what is NOT covered", and it stopped being the
    /// truth as soon as entries started closing: **two of the seven are closed** — A3 at level
    /// 1 and A6 at level 2 — and a heading that called them uncovered would mislead in the one
    /// direction nobody checks. Each entry says its own state.
    ///
    /// - **A1/A2 — `Instruction::new(untrusted.as_str().into())`.** Reaches the instruction
    ///   channel with the journal never hearing of it. NOT closable here: making
    ///   `Instruction::new` private stops the trusted sources too — the system prompt, what the
    ///   user typed — and nothing in this milestone tells them apart from the untrusted ones.
    /// - **A3 — `Instruction::new(format!("{:?}", untrusted))`.** ✅ CLOSED, by the
    ///   hand-written `Debug` above. It cost a derive and bought back the diagnostics with a
    ///   byte count.
    /// - **A4 — a round trip through the journal**: `outcome(id, untrusted.as_str().as_bytes())`,
    ///   then `read_back`, then `String::from_utf8`, then `Instruction::new`. NOT closable at
    ///   level 1, and the reason is a decision rather than an oversight: ADR-0036 has the port
    ///   exchange BYTES, and bytes carry no labels. ⚠️ This is the right place to notice it —
    ///   §4.9 designs the versioned record at milestone 3, and the label could become A FIELD
    ///   WITH AN EXPLICIT INDEX there. Cheap then, retrofitted later only by migrating the one
    ///   irreproducible archive.
    /// - **A5 — `transmute`, from a crate that allows `unsafe`.** `platform`, `secrets` and
    ///   `daemon` allow it ON PURPOSE. Closing it would mean giving the two types different
    ///   layouts — some 8 bytes per value — to stop somebody who writes `unsafe` WHILE NAMING
    ///   the kernel's two types. That is not an accident anybody has by mistake; it is
    ///   sabotage, and armouring against it is the wrong trade. Declared, not closed.
    /// - **A6 — a `Journal` that answers `Ok(())` and writes nothing.** ✅ CLOSED on 2026-08-10
    ///   by the conformance suite, `tests/journal_contract.rs`, where this exact journal is
    ///   `SilentJournal` and promise 1 catches it on the first assertion. ⚠️ AND THE LIMIT IS
    ///   PART OF THE CLOSURE: a conformance suite is worth the evidence that TWO
    ///   implementations answer alike, and the second one — `redb` in `platform` — arrives at
    ///   task 8. Until then what is closed is the road, not the agreement: no journal that
    ///   silently discards a write can pass the suite, and every journal this kernel is given
    ///   is meant to pass it. It is a level 2 rule and not level 1 — nothing stops somebody
    ///   from writing a `Journal` that never meets the suite — which is exactly why it is
    ///   written here rather than counted as a compiler guarantee.
    /// - **A7 — a CHILD module of `boundary`.** Field privacy reaches descendants, so a
    ///   submodule added tomorrow can build `Instruction(…)` directly. Not closable: it is the
    ///   same mechanism that lets this module build them at all.
    ///
    /// Why none of the ones still open becomes a level 1 rule: each would have to quantify over
    /// code THAT DOES NOT EXIST YET, and Rust states no rule about a call site not yet written.
    /// ⚠️ A6 is the counter-example that proves the shape of the answer rather than breaking it:
    /// it was closed at **level 2**, by a test that runs, not by the compiler — and the price
    /// was a whole conformance suite. The remaining five are declared, not fixed, and what
    /// holds them meanwhile is review. The guard covers the roads that exist; it is not total.
    pub fn promote<J: Journal>(
        self,
        journal: &mut J,
        step: StepId,
        reason: &str,
    ) -> Result<Instruction, JournalError> {
        let mut record: Vec<u8> = Vec::new();
        record.extend_from_slice(reason.as_bytes());
        journal.intent(step, &record)?;
        Ok(Instruction(self.0))
    }

    /// Shortening does not clean anything: the result is still untrusted (V20).
    ///
    /// ⚠️ `keep` counts CHARACTERS, not bytes. Slicing the bytes would be shorter to write
    /// and would panic on any text whose cut falls inside a multi-byte character — and
    /// external text is precisely the text nobody chose.
    ///
    /// ⛔ That reason is PINNED by `summarize_counts_characters_and_not_bytes`, and it was not
    /// before: with ASCII-only fixtures, replacing this body with a clamped byte slice left all
    /// six tests green while the mutant panicked on the first accented character. A declared
    /// reason that no test holds is a comment, and this one now has a test.
    pub fn summarize(&self, keep: usize) -> Untrusted {
        Untrusted(self.0.chars().take(keep).collect())
    }
}

/// The instruction channel accepts only `Instruction`.
///
/// ⚠️ NO PRODUCTION CALLER TODAY, and it is not waiting for one. It exists because rule A's
/// negative case has to name `kernel::` rather than declare its own types — gotcha #39 — and a
/// rule about "where an instruction is expected" needs somewhere that expects one. Written down
/// so a YAGNI pass does not remove it as dead weight; ⚠️ that removal would at least be NOISY
/// rather than silent — `untrusted_as_instruction.rs` would stop compiling for a different
/// reason and its oracle would go to `mismatch` — but noisy in a way that reads as an unrelated
/// breakage, which is how the deletion survives review.
pub fn build_prompt(system: &Instruction, user: &Instruction) -> String {
    let mut prompt = String::with_capacity(system.0.len() + user.0.len() + 1);
    prompt.push_str(system.as_str());
    prompt.push('\n');
    prompt.push_str(user.as_str());
    prompt
}
