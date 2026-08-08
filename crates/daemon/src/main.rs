//! The production wiring: mounts `platform`, starts the executor, hosts the IPC server,
//! and **produces the resolved parameters** it hands to the kernel (§2.8, ADR-0034).
//!
//! In this sub-project the defaults are **literals right here**, not read from a store:
//! constraint 11 of §11.

fn main() {
    println!("daemon: skeleton. No logic in this milestone.");
}
