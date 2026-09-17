//! The SEVEN families of ports (§2.3), and the number is not decoration: §3.1 declares this
//! list EXHAUSTIVE — "there are no other points at which the world touches the kernel" —
//! and the simulator substitutes ALL of them.
//!
//! ⚠️ DATED RECALL, 2026-09-17, sub-project 2 task 4: THE NUMBER ABOVE WAS SIX UNTIL TODAY. The
//! seventh family is `custody` -- it keeps the bytes the gui entrusts to the core and hands them
//! back -- designed in §2 of the GUI north star (decision 15) and declared in §2.3 on the same
//! day. ⛔ THE COUNT IS REALIGNED HERE RATHER THAN LEFT DATED IN PLACE, and this one sentence is
//! the exception because it COUNTS WHAT THIS MODULE DECLARES: a figure that disagreed with the
//! `pub mod` list below would make §3.1's "exhaustive" describe a world SMALLER than the real
//! one, which is gotcha #17 — the very thing the next paragraph exists to prevent.
//!
//! ⛔ A port discovered later means criterion C1 was verified on a world SMALLER than the
//! real one, and NOTHING WOULD HAVE GONE RED. That is gotcha #17, and it is the whole
//! reason all six are named in this milestone even though four of them have no caller yet:
//! the campaign is milestone 4, and a trait that does not exist by then is a hole the
//! campaign cannot see. It is the same argument with which F1a refused to let `process`
//! wait for §5 (§2.3.1).
//!
//! | Family       | Designed in | Real implementation arrives in           |
//! |--------------|-------------|------------------------------------------|
//! | `reactor`    | §2.4        | milestone 2 — the executor needs it now  |
//! | `journal`    | §4          | milestone 3                              |
//! | `filesystem` | §4          | staged (§0.4)                            |
//! | `process`    | §5.6, §6.10 | milestone 6                              |
//! | `ipc`        | §6.1        | milestone 6 (the port) · sub-project 2, task 2 (`platform::ipc::LocalSocketIpc`, the real transport) |
//! | `network`    | §2.3.1      | staged — the single exit point           |
//! | `custody`    | GUI north star §2 | sub-project 2, task 5                    |
//!
//! ⛔ THE TABLE IS THE DESIGN, NOT AN INVENTORY OF FILES — and with task 12 the two finally
//! COINCIDE: this module declares SEVEN submodules, one per row. Two of them have a caller —
//! `reactor`, which the executor needs, and `journal`, which the promotion of
//! `crate::boundary` demands as an argument. The other FIVE — `filesystem`, `network`,
//! `process`, `ipc` and `custody` — have NO CALLER AT ALL and are here for the reason above.
//! ⚠️ DATED RECALL, 2026-09-17, sub-project 2 task 2: `ipc` HAS ITS REAL IMPLEMENTATION NOW --
//! `platform::ipc::LocalSocketIpc` -- and its first caller arrives with task 7 (`kernel::serving`),
//! so the FOUR above are THREE from there on. The figure in the sentence is dated here and NOT
//! realigned (gotcha #31): the command that counts is `grep -rnE "^ *impl Ipc for" crates/`.
//! ⚠️ DATED RECALL, 2026-09-17, sub-project 2 task 4: THE BASE WENT FROM FOUR TO FIVE, and the
//! subtraction above is UNTOUCHED because it is still right. `custody` joined the submodules with
//! no caller, so the partition here reads TWO PLUS FIVE and the list had to NAME it: the command
//! that counts the whole is `grep -c "^pub mod " crates/kernel/src/ports/mod.rs`, SEVEN on
//! 2026-09-17. ⛔ A LIST THAT OMITS A MEMBER IS GOTCHA #17 ARRIVING AS AN OMISSION RATHER THAN AS
//! A DIGIT, which is why the count and the names moved together rather than the count alone.
//! ⛔ AND THE TWO SENTENCES COUNT DIFFERENT SETS, SO BOTH HOLD: `ipc` has a real implementation
//! and no caller yet, so the ports with NEITHER are `filesystem`, `network`, `process` and
//! `custody`. ⚠️ `custody`'s FIRST CALLER ARRIVES WITH TASK 7 -- `Core::new`, which takes it BY
//! VALUE -- so this line carries the date on which it stops being true, written before it does.
//!
//! ⚠️ AND THAT COINCIDENCE IS PRECISELY WHEN THE TABLE LOOKS DELETABLE, so the reason it stays
//! is written here rather than left to be re-derived. Until `ipc` landed, the table was the
//! only place the sixth family could be named at all — a `pub mod` naming a file that does not
//! exist does not compile — so it was visibly doing work. Now every row has a file and the
//! table reads like a duplicate of the `pub mod` list below. It is not: the list says what
//! EXISTS, the table says how many there are SUPPOSED to be. Remove it and a seventh family
//! added later stops being a discrepancy anyone can see, which is gotcha #17 arriving by the
//! back door.
//!
//! ⚠️ A TRAIT NOBODY IMPLEMENTS IS NOT A TRAIT PROVED IMPLEMENTABLE. The five declared
//! without a caller are held by `tests/ports_are_implementable.rs` — SIX fakes, because
//! `process` needs two of them (`Worker` and `Process`), and calls that exercise each in both
//! directions. It buys that the signatures compile FROM OUTSIDE THE CRATE and can be called;
//! it does not buy that they are the right signatures, and it is not the conformance suite,
//! which needs two implementations to compare.
//!
//! ⚠️ DATED RECALL, 2026-08-28 — FINDING AUD-054. The FIVE is still TRUE OF THAT FILE and is
//! kept for it; what was false is reading the sentence as the whole account for `process`,
//! which since `5fceee1` (2026-08-21) is ALSO held by `tests/worker_tokens.rs` — where `start`
//! is driven with a `Grant` the arbiter really issued, a path this bench never walks — and by
//! four `tests/compile_fail/` cases at level 1. ⛔ THE COUNT IS NOT EXTENDED HERE, and that is
//! the point: extending it would give the figure a second house and it would rot in the one
//! nobody moves. `ports/process.rs` carries the reckoning for that family; this paragraph
//! keeps only what it measured, which is this one file.
//! ⚠️ DATED RECALL, 2026-09-17, sub-project 2 task 4: SIX from here on -- `custody` has a fake of its own; the FIVE above is dated, not realigned (gotcha #31).
//! ⛔ AND THE OTHER NUMBER IN THAT SENTENCE MOVED WITH IT: the FAMILIES this bench holds a fake
//! for go FOUR -> FIVE, and THAT one is REALIGNED rather than dated, because the sentence is an
//! ARGUMENT and not an inventory -- five families, `process` twice, six fakes -- and a sum that
//! no longer closes teaches nothing. ⚠️ AND THE ARITHMETIC IS COUNTED RATHER THAN ASSERTED,
//! which is the whole reason a figure may live in prose at all:
//! `grep -cE "^impl [A-Za-z]+ for " crates/kernel/tests/ports_are_implementable.rs` answers SIX
//! on 2026-09-17, and the fifth family is `custody`, whose fake is the one this recall dates.
//! Moving one half and leaving the other IS finding AUD-049, the third line of the audit
//! discipline, and it is what this clause exists to close.
//! ⚠️ THE OTHER `four`, IN THE TABLE PARAGRAPH ABOVE, IS A DIFFERENT COUNT AND IS LEFT ALONE ON
//! PURPOSE: it counts the PORTS WITHOUT A CALLER, not the traits this bench holds a fake for,
//! and it is task 2's -- dated in its own recall there. Written here so that the next census
//! does not "fix" the two as if they were one figure in two houses. ⛔ AND IT IS NOT QUOTED
//! WORD FOR WORD ON PURPOSE EITHER: that sentence is what the closing check of task 4 counts,
//! and a quotation of it here would move a count that is supposed to prove nothing moved.
//!
//! ⛔ AND ON `process` THAT TEST EARNED ITS KEEP RATHER THAN CONFIRMING ANYTHING. The port as
//! designed was NOT IMPLEMENTABLE: `instruct_one` has to HAND BACK a `SingleReceipt` whose
//! only field is `pub(crate)`, so from outside the crate the return value could not be built
//! — gotcha #46, in the worse form where what is missing is not a read but a value. Measured
//! (`E0599`, then `E0451` once the first errors stopped masking the privacy pass), and the
//! remedy is written beside the constructors in `process.rs`. ⚠️ `Grant` is the deliberate
//! opposite and keeps no constructor: §5.6 wants that one unbuildable.
//!
//! ⚠️ ON `ipc` THE SAME TEST CONFIRMED INSTEAD, AND THAT IS WORTH RECORDING TOO. Written before
//! the port existed, the fake compiled at the first attempt: no missing constructor, no masked
//! privacy error. What it earned there was the opposite service — it is what proved that
//! `ClientId` needs NO getter, because a fake that retains a `Copy` identifier and compares it
//! never asks for the number. Three derives and an accessor the plan dictated came off on that
//! evidence; the reasoning is beside the type in `ipc.rs`.
//!
//! ⚠️ `journal` is declared here in milestone 2 and IMPLEMENTED in milestone 3: the trait
//! exists because a caller already demanded it, not because the durable format was settled. The
//! record, the version enum and the frozen bytes are §4.9, and constraint 14 of §11 freezes them
//! at the first record written. ⚠️ THE LINE ENDED WITH "so nothing writes one yet" UNTIL
//! 2026-08-10 and is dated rather than rewritten: `Untrusted::promote` writes one, and the frozen
//! bytes are in the repository. The format IS settled now, and `crate::record` states on what
//! terms it may still grow — optional fields at new indices, and nothing else.
//!
//! ⚠️ `rng` IS DECLARED IN §2.2 AND LIVES IN `crate::rng`, NOT HERE. It is a source of
//! non-determinism, not a family of I/O, and the asymmetry is deliberate rather than a
//! filing mistake: the simulator substitutes SEVEN things while §2.3 enumerates SIX, and
//! §3.1 says so in those words. Repeated here so that nobody "fixes" the discrepancy by
//! moving `rng` under this module, or by writing "seven families" in the line above.
//! ✅ DATED RECALL, 2026-09-17 -- THE NUMBERS MOVED AND THE WARNING STANDS, WHICH IS THE WHOLE
//! POINT OF DATING IT RATHER THAN REWRITING IT. A SEVENTH FAMILY ARRIVED -- `custody`, the
//! layout the gui entrusts to the core (decision 15 of the GUI north star) -- so the simulator
//! now substitutes EIGHT things while §2.3 enumerates SEVEN. ⛔ THE DISCREPANCY DID NOT CLOSE,
//! IT MOVED: `rng` is still declared in §2.2 and still lives in `crate::rng`, and moving it
//! under this module is still the wrong fix.
//! ⛔ AND "seven families" IS NOW WRITTEN IN THE LINE ABOVE -- BY THE OPENING SENTENCE OF THIS
//! MODULE, WHICH THE SAME TASK REALIGNED WITH A RECALL OF ITS OWN, AND NOT BY THIS ONE -- AND IT
//! IS NOT THE THING THE SENTENCE FORBIDS. The forbidden seven was the one that COUNTS `rng` as a
//! family of I/O; today's seven counts a real family and leaves `rng` exactly where it was. A
//! reader who sees the two sentences side by side should read this one: the warning is about
//! WHAT IS COUNTED, not about the digit.

pub mod custody;

pub mod filesystem;

pub mod ipc;

pub mod journal;

pub mod network;

pub mod process;

pub mod reactor;
