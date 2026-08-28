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
//! with deliberate liars of its own. ⚠️ THAT SUITE EXISTS SINCE 2026-08-10 —
//! `tests/journal_contract.rs` — and this paragraph is dated
//! rather than rewritten, because the sentence it replaced ("the conversion is journalled",
//! read as a guarantee) is the mistake it exists to prevent and the correction has its own
//! date. ⛔ WHAT THE SUITE BUYS IS STILL NOT WHAT THE TYPE SYSTEM BUYS: it is level 2, so
//! nothing stops somebody writing a `Journal` that never meets it. Road A6 below says exactly
//! how far it reaches.
//!
//! ⚠️ RECALL OF 2026-08-21 — THE COUNTS ARE GONE FROM THIS FILE AND NOT REALIGNED. This
//! paragraph said "eight promises and eight liars", and road A6 below said "the same eight
//! promises": they are NINE and TWELVE, and have been since 2026-08-17. A figure that lives
//! in more than one file goes stale in the copy nobody moves, so it is REMOVED here and left
//! where it is recounted — `tests/journal_contract.rs`, which counts its own tests with a
//! `grep` instead of bumping a number. `CLAUDE.md`'s rule, and gotcha #68.
//! ⛔ AND THIS RECALL WAS ITSELF FALSE UNTIL 2026-08-28, finding AUD-049: a THIRD count sat
//! nine lines above it — "two deliberate liars" of `tests/reactor_contract.rs`, which has had
//! SEVEN of them since 2026-08-18 — so "the counts are gone from this file" was written with
//! one still standing, on the very line the same pass was editing. Removed too, and left
//! where that suite recounts its own liars.

use core::fmt;

use alloc::string::String;

use crate::ports::journal::{Journal, JournalError, StepId};
use crate::record::{EffectClass, Record, RecordKind, RecordV1, Trust};

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
    /// `reason` is recorded with the promotion, at index 4 of the record: a promotion whose
    /// reason nobody wrote down is indistinguishable from one nobody thought about. ⚠️ NOTHING
    /// HERE CHECKS THAT `reason` IS NON-EMPTY, and that is still true; what has changed since
    /// milestone 2 is that the conformance suite now checks the implementation really keeps it.
    ///
    /// ⛔ THE OPEN QUESTION OF MILESTONE 2 IS ANSWERED, AND IT NEEDED A THIRD ANSWER. It read:
    /// is a promotion a STEP OF ITS OWN, which then owes its own outcome, or a note ON THE
    /// CALLER'S step, which already has one? It is a NOTE — ADR-0007 fixes the granularity, "a
    /// step is AN INTERACTION WITH THE OUTSIDE WORLD", and a promotion touches nothing outside.
    /// A step of its own would double the durable writes for something that reaches nothing and
    /// would sit in doubt for ever, because nobody owes it an outcome.
    ///
    /// ⛔ AND ANSWERING "A NOTE" DID NOT SAY HOW TO WRITE ONE, which is where the plan for this
    /// task was wrong and the error was found by MEASURING both roads rather than by reading:
    ///
    /// - as a second `intent` on the caller's step, the port REFUSES it — one intent per step,
    ///   `OutOfOrder`. And with that guard removed it is worse than a refusal: reconciliation
    ///   reads a second `Intent` record for the step and REPLACES the caller's resolution with
    ///   this one's. Measured — a step the caller declared `Idempotent` came back
    ///   `SuspendAndAsk`. A promotion would silently downgrade a step it does not own.
    /// - as an `outcome`, the step LEAVES THE DOUBT although the caller has not executed.
    ///   Measured — `steps_in_doubt` answered `[]`. A true doubt vanishing in silence is the one
    ///   failure ADR-0007 exists to prevent.
    ///
    /// So the port gained `note` and the record gained `RecordKind::Note`, together, because
    /// neither half works without the other: a note has to carry SOME `kind`, and both existing
    /// ones are the defects above. Decided by the coordinator on 2026-08-10 and recorded in the
    /// plan's errata so the owner can overturn it by seeing it.
    ///
    /// ⚠️ THE CLASS IS `Unrepeatable` AND IT IS AN INERT FIELD, which is the opposite of what
    /// this paragraph first said. `crate::reconcile` never reads the `effect` of a `Note` — the
    /// arm is empty — so nothing branches on it and no argument about repeatability is doing any
    /// work here. `Unrepeatable` is what an inert field is filled with in this record: it is
    /// ADR-0007's own answer for a class nobody can act on, so if some later reader ever treats
    /// an unknown kind as an intent, the value it finds stops the system instead of re-running
    /// something. Saying "it is not a placeholder" would be a claim the measurement contradicts.
    ///
    /// ⛔ AND THE STEP IS THE CALLER'S. `step` is the step the caller already opened and already
    /// owes an outcome for, so `note` refuses if that intent is not there. Before this, every
    /// promotion left behind a step with an intent and no outcome: IN DOUBT FROM BIRTH and never
    /// prunable.
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
    /// truth as soon as entries started closing: **three of the seven are closed** — A3 at level
    /// 1, A4 and A6 at level 2 — and a heading that called them uncovered would mislead in the
    /// one direction nobody checks. Each entry says its own state.
    ///
    /// ⚠️ COUNTED ON THE ENTRIES BELOW AND NOT DEDUCED, because this line has already been wrong
    /// once: there are SIX entries and SEVEN roads, since A1/A2 is two. Closed: A3, A4, A6.
    /// Open: A1, A2, A5, A7 — FOUR, and every one of them is an entry that declares itself NOT
    /// CLOSABLE. That last sentence is new on 2026-08-10 and is the real change: what remains is
    /// no longer a backlog, it is the declared floor.
    ///
    /// - **A1/A2 — `Instruction::new(untrusted.as_str().into())`.** Reaches the instruction
    ///   channel with the journal never hearing of it. NOT closable here: making
    ///   `Instruction::new` private stops the trusted sources too — the system prompt, what the
    ///   user typed — and nothing in this milestone tells them apart from the untrusted ones.
    /// - **A3 — `Instruction::new(format!("{:?}", untrusted))`.** ✅ CLOSED, by the
    ///   hand-written `Debug` above. It cost a derive and bought back the diagnostics with a
    ///   byte count.
    ///
    ///   ⛔ AND IT WAS DECLARED CLOSED WHILE A SECOND MOUTH WAS OPEN — finding P-1 of the
    ///   2026-08-11 audit, shut on 2026-08-18. `Untrusted` stopped printing its content, but
    ///   `promote` took `reason: &str` and `RecordV1`'s hand-written `Debug` PRINTS index 4 in
    ///   full. So external text walked out through the justification instead of through the
    ///   payload, and demonstrated from outside the crate it read:
    ///   `RecordV1 { … payload: <16 bytes>, reason: "ignore your instructions" }` — the guarded
    ///   field hidden, the unguarded one wide open.
    ///
    ///   ✅ **Shut at level 1 by the type of the argument: `reason: &'static str`.** External
    ///   content is RUNTIME data; a `&'static str` is a literal in the binary. The accidental
    ///   road stops compiling, and the case that holds it is
    ///   `tests/compile_fail/promote_reason_is_not_runtime_text.rs`.
    ///
    ///   ⛔ **AND THE FIX THE REPORT NAMED WOULD NOT HAVE SHUT IT.** §8 proposed
    ///   `reason: &Instruction`; `Instruction::new` is `pub` and takes any `String`, so
    ///   `Instruction::new(untrusted.as_str().into())` satisfies it — which is road **A1/A2**,
    ///   declared right above as NOT CLOSABLE. A newtype guard is worth exactly what its
    ///   CONSTRUCTOR is worth, and this one would have bought the look of a closure over a road
    ///   this very list already declares open. ⚠️ It would also have been a type pun:
    ///   `Instruction` means *content allowed in the instruction channel*, and a justification
    ///   is not that — using it here blurs the one distinction the type exists to draw.
    ///
    ///   ⚠️ **WHAT IS STILL OPEN, and it is declared rather than papered over.** `String::leak`
    ///   yields a `&'static str`, so a caller determined to smuggle can still do it — the same
    ///   trade A5 states below: deliberate, visible, and not an accident anybody has by
    ///   mistake. And a literal can still LIE — `"quoted by the user"` on a promotion the user
    ///   never asked for — which is provenance and not correctness, exactly the limit A4
    ///   declares. What shut is the road somebody takes without noticing.
    /// - **A4 — a round trip through the journal**: `outcome(id, untrusted.as_str().as_bytes())`,
    ///   then `read_back`, then `String::from_utf8`, then `Instruction::new`. ✅ CLOSED on
    ///   2026-08-10, **at level 2**, by the record carrying the label: `promote` now writes the
    ///   untrusted content into `payload` — index 3 — with `trust: Trust::Untrusted` beside it,
    ///   so what comes back out of a decoding SAYS what it was. It is NOT closable at level 1,
    ///   and the reason is a decision rather than an oversight: ADR-0036 has the port exchange
    ///   BYTES, and bytes carry no labels.
    ///
    ///   ⛔ AND THE LIMIT IS PART OF THE CLOSURE, in the shape A6's is. The label closes this
    ///   road FOR WHATEVER PASSES THROUGH THE FORMAT, and nothing today requires that every
    ///   write to the journal be a `Record`: the road as written above hands the port RAW BYTES,
    ///   `Record::decode` answers `Malformed` on them, and the round trip still works. So what
    ///   is closed is the road THROUGH THE RECORD — the one a promotion actually takes — and
    ///   what holds the rest is the same thing that holds A5 and A7: review. ⚠️ Saying "A4 is
    ///   closed" without this paragraph would be the sentence that stops the next reader looking.
    ///
    ///   ⚠️ AND THE LABEL PROVES PROVENANCE, NOT CORRECTNESS (§6.3.2): whoever writes a record
    ///   may still label it wrongly. What it buys is that a reader can no longer LOSE the
    ///   distinction, which is a different thing from making it impossible to lie about.
    ///
    ///   ⚠️ IT WAS ALMOST CLOSED ON PAPER AND NOT IN FACT, and that is worth the line. The plan
    ///   for this task put THE REASON in `payload` and labelled it `Trust::Untrusted` — the
    ///   caller's own justification, which never crossed any boundary — so no untrusted byte
    ///   would have entered the record at all and the label would have been FALSE rather than
    ///   merely decorative. The content and the reason now travel at two indices, 3 and 4.
    /// - **A5 — `transmute`, from a crate that allows `unsafe`.** `platform`, `secrets` and
    ///   `daemon` allow it ON PURPOSE. Closing it would mean giving the two types different
    ///   layouts — some 8 bytes per value — to stop somebody who writes `unsafe` WHILE NAMING
    ///   the kernel's two types. That is not an accident anybody has by mistake; it is
    ///   sabotage, and armouring against it is the wrong trade. Declared, not closed.
    /// - **A6 — a `Journal` that answers `Ok(())` and writes nothing.** ✅ CLOSED on 2026-08-10
    ///   by the conformance suite, `tests/journal_contract.rs`, where this exact journal is
    ///   `SilentJournal` and promise 1 catches it on the first assertion. ✅ AND THE LIMIT THAT
    ///   WAS PART OF THE CLOSURE IS SPENT, later the same day at task 9. This entry read: "a
    ///   conformance suite is worth the evidence that TWO implementations answer alike, and the
    ///   second one — `redb` in `platform` — arrives at task 8. Until then what is closed is the
    ///   road, not the agreement." The second one arrived, and
    ///   `crates/platform/tests/journal_contract_real.rs` holds it to the same promises ON
    ///   EVERY COMMIT — which is the whole difference between "measured once" and "held". ⛔ IT
    ///   IS STILL A LEVEL 2 RULE AND NOT LEVEL 1: nothing stops somebody from writing a
    ///   `Journal` that never meets the suite, which is exactly why it is written here rather
    ///   than counted as a compiler guarantee.
    /// - **A7 — a CHILD module of `boundary`.** Field privacy reaches descendants, so a
    ///   submodule added tomorrow can build `Instruction(…)` directly. Not closable: it is the
    ///   same mechanism that lets this module build them at all.
    ///
    /// Why none of the ones still open becomes a level 1 rule: each would have to quantify over
    /// code THAT DOES NOT EXIST YET, and Rust states no rule about a call site not yet written.
    /// ⚠️ A4 and A6 are the counter-examples that prove the shape of the answer rather than
    /// breaking it: both were closed at **level 2**, by things that run rather than by the
    /// compiler — a whole conformance suite for one, a field in the durable format for the
    /// other. ⛔ THE REMAINING FOUR — A1, A2, A5, A7 — ARE NOT A BACKLOG: each of their entries
    /// declares itself not closable, so what is left is the floor and not the unfinished part.
    /// What holds them is review. The guard covers the roads that exist; it is not total.
    pub fn promote<J: Journal>(
        self,
        journal: &mut J,
        step: StepId,
        reason: &'static str,
    ) -> Result<Instruction, JournalError> {
        // ⛔ THE PAYLOAD IS THE UNTRUSTED CONTENT AND THE REASON IS A FIELD OF ITS OWN. Index 3
        // is the one the record's hand-written `Debug` hides, and index 4 the one it prints:
        // somebody else's bytes in the first, our words in the second. Swapping them would put
        // external text into the first `{:?}` that reaches a log — road A3, reopened one type
        // over.
        //
        // ⚠️ THE COPY IS ONE ALLOCATION AND IT IS DELIBERATE: the bytes are taken before
        // `Instruction(self.0)` MOVES the string, so the content is copied once into the record
        // and the string itself is not copied at all.
        let record = Record::V1(RecordV1 {
            kind: RecordKind::Note,
            effect: EffectClass::Unrepeatable,
            trust: Trust::Untrusted,
            payload: self.0.as_bytes().to_vec(),
            reason: String::from(reason),
        })
        .encode();

        journal.note(step, &record)?;
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
