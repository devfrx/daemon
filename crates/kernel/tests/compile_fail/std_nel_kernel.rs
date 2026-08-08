// Regola: il kernel non NOMINA `std`. Meccanismo: `#![no_std]`. Forza: compilatore, E0433.
// Difende: I3 · V28 · V29 — §7.4.1 blocco A.
#![no_std]

fn legge_un_file() {
    let _ = std::fs::read_to_string("/etc/passwd");
}

fn main() {}
