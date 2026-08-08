// Rule: `HashMap` is not nameable in the kernel. Mechanism: FREE consequence of
// `no_std` — `HashMap` lives in `std`, not in `alloc`. Force: compiler, E0433.
// Defends: V29 · gotcha #12 — `RandomState` is seeded per process, and the iteration
// order is not reproducible across runs.
#![no_std]

extern crate alloc;

fn count() {
    let _m: std::collections::HashMap<u8, u8> = std::collections::HashMap::new();
}

fn main() {}
