//! Il kernel: logica, decisioni, tratti dichiarati. Nessuna chiamata all'OS.
//!
//! Questa crate non contiene ancora nulla: il Traguardo 1 costruisce lo scheletro e la
//! porta di qualità, e la logica arriva dai traguardi successivi. Gli attributi qui
//! sotto NON sono decorazione — sono tre delle regole di livello 1 della §7.4.1, e i
//! loro test negativi vivono in `tests/compile_fail/`.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;
