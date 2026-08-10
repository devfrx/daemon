//! The kernel: logic, decisions, declared traits. No call to the OS.
//!
//! Milestone 2 is filling this crate with the INJECTABLE SUBSTRATE: the two concepts of
//! time, the randomness port, the delivered decision parameters, the executor, and the
//! six families of ports as traits. The kernel never takes anything from the world — it
//! is handed a provider, and in simulation that provider is a fake governed by a seed.
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
