//! The schemas of the two private channels, one file each.
//!
//! ⛔ SHARING A FOLDER IS NOT SHARING A SCHEMA -- ADR-0035, rule 2. The two schemas are
//! distinct and so are the two formats, and ADR-0037 measured why: the peers differ. What
//! they do share is the envelope, and it lives in `crate::framing`.
//!
//! ⚠️ RECALL OF 2026-08-31, AUD-046 IN THIS FILE'S SHAPE: a line here said "`ipc` IS NOT HERE
//! YET: it arrives with task 4, in the format that task 3bis decides", and the commit that
//! added the module below made it false. REMOVED, NOT REALIGNED -- what this module contains
//! is said by its `pub mod` list, which cannot go stale, and a prose inventory beside a list
//! is a second house for the same fact.

pub mod ipc;
pub mod worker;
