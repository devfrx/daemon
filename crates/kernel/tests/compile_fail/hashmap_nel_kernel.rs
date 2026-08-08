// Regola: `HashMap` non è nominabile nel kernel. Meccanismo: conseguenza GRATUITA di
// `no_std` — `HashMap` vive in `std`, non in `alloc`. Forza: compilatore, E0433.
// Difende: V29 · gotcha #12 — `RandomState` è seminato per processo, e l'ordine di
// iterazione non è riproducibile fra esecuzioni.
#![no_std]

extern crate alloc;

fn conta() {
    let _m: std::collections::HashMap<u8, u8> = std::collections::HashMap::new();
}

fn main() {}
