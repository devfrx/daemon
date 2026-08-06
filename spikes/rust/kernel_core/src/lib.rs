//! T6 — regola di importazione vietata, meccanismo (b): il **compilatore**.
//!
//! `no_std` rimuove dalla portata dell'intera unità di compilazione tutto ciò che
//! parla con il sistema operativo: `std::fs`, `std::net`, `std::env`,
//! `std::process`, `std::time::SystemTime`, `std::thread`. Non è un lint che si può
//! disattivare per riga: è ciò che il compilatore ha caricato.
//!
//! `alloc` restituisce `Vec`, `String`, `Box`, `BTreeMap` — cioè tutto il necessario
//! a un kernel che fa logica pura e passa ogni effetto attraverso un tratto iniettato.
//!
//! I3 · ADR-0002 · V29 · ADR-0021.

#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

/// Il kernel decide con regole e su dati che gli vengono passati.
/// Nessun orologio, nessuna casualità, nessun I/O, nessuna chiamata all'OS.
/// `sort_by_key` è un ordinamento **stabile**: a parità di priorità l'ordine di
/// inserimento è conservato. Per V29 non è un dettaglio di stile — un ordinamento
/// instabile introdurrebbe non determinismo proprio in una coda del kernel.
pub fn ordina_per_priorita(mut lavori: Vec<(u32, String)>) -> Vec<(u32, String)> {
    lavori.sort_by_key(|lavoro| lavoro.0);
    lavori
}
