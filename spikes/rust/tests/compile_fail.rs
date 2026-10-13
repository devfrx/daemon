#[test]
fn t1_il_non_fidato_non_entra_nel_canale_istruzioni() {
    // Nessuna guardia sul numero dei casi: per `trybuild` 1.0.120 un glob che non pesca niente
    // non è un errore — stampa «There are no trybuild tests enabled yet.» ed esce verde (gotcha
    // #26) —, quindi con `tests/compile_fail/` senza casi questo test passerebbe senza provare
    // T1. Il banco del prodotto ha la guardia: `crates/kernel/tests/compile_fail.rs`.
    // ⚠️ RICHIAMO DEL 2026-10-13 — audit del 2026-09-30, AUD-2227.
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
}
