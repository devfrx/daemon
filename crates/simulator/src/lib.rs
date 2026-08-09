//! The DST simulator: fake implementations of the traits the kernel declares.
//!
//! Constrained like `kernel`, and for the same reason: a deterministic run cannot
//! contain a source of non-determinism, and `HashMap` is the most insidious one because
//! it appears in no list of "OS calls" — gotcha #12.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod reactor;
pub mod rng;
