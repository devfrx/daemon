//! The only place that touches the OS keyring.
//!
//! It is a crate separate from `platform` for one reason only: V34 requires that "a
//! single place reads the credentials" be verifiable **statically**, and in Rust the
//! verifiable granularity is the crate. Inside `platform` it would be a rule between
//! modules, that is, a convention. It is why the crates are five and not four (§1.2).
