// Rule: the kernel does not NAME `std`. Mechanism: `#![no_std]`. Force: compiler, E0433.
// Defends: I3 · V28 · V29 — §7.4.1 block A.
#![no_std]

fn reads_a_file() {
    let _ = std::fs::read_to_string("/etc/passwd");
}

fn main() {}
