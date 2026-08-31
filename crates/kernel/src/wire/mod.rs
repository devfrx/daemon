//! The schemas of the two private channels, one file each.
//!
//! ⛔ SHARING A FOLDER IS NOT SHARING A SCHEMA -- ADR-0035, rule 2. The two schemas are
//! distinct and so are the two formats, and ADR-0037 measured why: the peers differ. What
//! they do share is the envelope, and it lives in `crate::framing`.

pub mod ipc;
pub mod worker;
