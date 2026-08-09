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
//! ⛔ THE TABLE IS THE DESIGN, NOT AN INVENTORY OF FILES. Today this module declares FOUR
//! submodules — `reactor`, which the executor needs; `journal`, which the promotion of
//! `crate::boundary` demands as an argument; and `filesystem` and `network`, which have NO
//! CALLER AT ALL and are here for the reason above. `process` and `ipc` arrive by Task 12 of
//! the milestone 2 plan, and this table is completed there. A `pub mod` naming a file that
//! does not exist does not compile, so for those two the table is still the only place they
//! can be named at all — and naming them is exactly what gotcha #17 above is asking for.
//!
//! ⚠️ A TRAIT NOBODY IMPLEMENTS IS NOT A TRAIT PROVED IMPLEMENTABLE. The two declared without
//! a caller are held by `tests/ports_are_implementable.rs` — a fake for each, and calls that
//! exercise it. It buys that the signatures compile FROM OUTSIDE THE CRATE and can be called;
//! it does not buy that they are the right signatures, and it is not the conformance suite,
//! which needs two implementations to compare.
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

pub mod journal;

pub mod network;

pub mod reactor;
