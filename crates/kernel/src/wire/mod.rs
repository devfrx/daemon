//! The schemas of the two private channels, one file each.
//!
//! ⛔ SHARING A FOLDER IS NOT SHARING A SCHEMA -- ADR-0035, rule 2. The two schemas are
//! distinct and so are the two formats, and ADR-0037 measured why: the peers differ. What
//! they do share is the envelope, and it lives in `crate::framing`.
//!
//! ⚠️ `ipc` IS NOT HERE YET: it arrives with task 4, in the format that task 3bis decides.

pub mod worker;
