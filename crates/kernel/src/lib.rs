//! The kernel: logic, decisions, declared traits. No call to the OS.
//!
//! WHAT THIS CRATE HOLDS IS THE LIST OF `pub mod` BELOW, and that list is the only answer
//! that cannot go stale: a prose summary of it ages the day a milestone adds a module by
//! touching the tail of the file. ⚠️ RECALL OF 2026-08-28, AUD-046: this said "Milestone 2
//! IS FILLING this crate with the INJECTABLE SUBSTRATE" and named FIVE things while `pub
//! mod` named nine — four milestones had added modules without ever entering the paragraph
//! that summarises them. REMOVED, not realigned to nine. The kernel never takes anything
//! from the world — it is handed a provider, and in simulation that provider is a fake
//! governed by a seed.
//! The attributes below are NOT decoration — they are three of the level 1 rules of
//! §7.4.1, and their negative tests live in `tests/compile_fail/`.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod time;

pub mod rng;

pub mod parameters;

pub mod ports;

pub mod executor;

pub mod boundary;

pub mod record;

pub mod reconcile;

pub mod arbiter;

pub mod framing;

pub mod wire;

pub mod sensor;
