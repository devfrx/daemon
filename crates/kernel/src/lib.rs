//! The kernel: logic, decisions, declared traits. No call to the OS.
//!
//! This crate contains nothing yet: Milestone 1 builds the skeleton and the quality
//! gate, and the logic arrives with the later milestones. The attributes below are NOT
//! decoration — they are three of the level 1 rules of §7.4.1, and their negative
//! tests live in `tests/compile_fail/`.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod time;
