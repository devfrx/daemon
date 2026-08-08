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
    guardia_di_non_vacuita();
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
}

/// ⛔ Il banco SENZA casi dev'essere rosso. Senza questa guardia è VERDE, ed è misurato su
/// `trybuild` 1.0.120 — non dedotto:
///
/// - `expand.rs:20` — con un pattern che contiene `*`, `glob()` restituisce `Err`
///   **solo** se è il pattern a essere malformato, non se non corrisponde a nulla: zero
///   corrispondenze danno zero casi espansi;
/// - `run.rs:74` — con zero casi stampa un avviso giallo, lascia `failures` a zero e non
///   solleva niente. Cioè esce verde.
///
/// ⚠️ L'asimmetria che rende necessaria la guardia: un percorso **letterale** inesistente
/// diventa invece rosso, perché passa da `check_exists`. È il solo glob a ingoiare il
/// vuoto, e nessuno lo ricostruisce leggendo `t.compile_fail(...)`.
///
/// ⛔ Nessun numero atteso, per la ragione di §8.6.2: un conteggio fissato diventerebbe
/// rosso il giorno in cui il banco cresce per un motivo legittimo — gotcha #9 applicato
/// alla guardia. Si controlla che i casi siano **più di zero**.
fn guardia_di_non_vacuita() {
    let cartella = "tests/compile_fail";
    let casi = match std::fs::read_dir(cartella) {
        Ok(voci) => voci
            .flatten()
            .filter(|voce| voce.path().extension().is_some_and(|est| est == "rs"))
            .count(),
        // La cartella che non si apre vale zero casi: i due modi di essere vuoto sono lo
        // stesso guasto, e il messaggio sotto li nomina entrambi.
        Err(_) => 0,
    };
    assert!(
        casi > 0,
        "banco vuoto: `{cartella}/` non esiste o non contiene nessun `.rs`. Non è \
         «niente da fare»: è la porta di livello 1 che non sta provando NULLA, e senza \
         questa guardia uscirebbe VERDE."
    );
}
