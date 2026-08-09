//! The SIX families of ports (§2.3), and the number is not decoration: §3.1 declares this
//! list EXHAUSTIVE — "there are no other points at which the world touches the kernel" —
//! and the simulator substitutes ALL of them.
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
//! | `ipc`        | §6.1        | milestone 6                              |
//! | `network`    | §2.3.1      | staged — the single exit point           |
//!
//! ⛔ THE TABLE IS THE DESIGN, NOT AN INVENTORY OF FILES — and with task 12 the two finally
//! COINCIDE: this module declares SIX submodules, one per row. Two of them have a caller —
//! `reactor`, which the executor needs, and `journal`, which the promotion of
//! `crate::boundary` demands as an argument. The other FOUR — `filesystem`, `network`,
//! `process` and `ipc` — have NO CALLER AT ALL and are here for the reason above.
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
//! ⚠️ A TRAIT NOBODY IMPLEMENTS IS NOT A TRAIT PROVED IMPLEMENTABLE. The four declared
//! without a caller are held by `tests/ports_are_implementable.rs` — FIVE fakes, because
//! `process` needs two of them (`Worker` and `Process`), and calls that exercise each in both
//! directions. It buys that the signatures compile FROM OUTSIDE THE CRATE and can be called;
//! it does not buy that they are the right signatures, and it is not the conformance suite,
//! which needs two implementations to compare.
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
//! exists because a caller already demands it, not because the durable format is settled. The
//! record, the version enum and the frozen bytes are §4.9, and constraint 14 of §11 freezes
//! them at the first record written — so nothing writes one yet.
//!
//! ⚠️ `rng` IS DECLARED IN §2.2 AND LIVES IN `crate::rng`, NOT HERE. It is a source of
//! non-determinism, not a family of I/O, and the asymmetry is deliberate rather than a
//! filing mistake: the simulator substitutes SEVEN things while §2.3 enumerates SIX, and
//! §3.1 says so in those words. Repeated here so that nobody "fixes" the discrepancy by
//! moving `rng` under this module, or by writing "seven families" in the line above.

pub mod filesystem;

pub mod ipc;

pub mod journal;

pub mod network;

pub mod process;

pub mod reactor;
