//! I test di compilazione fallita: le regole di livello 1, viste scattare.
//!
//! ⛔ GOTCHA #25 — GLI `.stderr` NON SI RIGENERANO IN BLOCCO.
//! `trybuild` offre `TRYBUILD=overwrite` per riscriverli tutti sull'output corrente.
//! Serve quando i messaggi del compilatore cambiano legittimamente. Usato senza leggerli,
//! ogni caso diventa «l'errore atteso è quello che è uscito» e la suite passa per sempre.
//! La rigenerazione è un atto deliberato e **si legge nel diff**.
//!
//! ⚠️ Un test di compilazione fallita ha forza di livello 1 e visibilità di livello 2
//! (§7.1.3): cancellarlo NON riapre la violazione, la rende invisibile.

#[test]
fn le_regole_di_livello_1_non_compilano() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
}
