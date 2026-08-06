//! C6 — soglia del protocollo: **100 esecuzioni con lo stesso seed, tracce
//! byte-identiche.** Una divergenza su 100 è già un difetto di concorrenza
//! irriproducibile, che è la classe di bug che la DST esiste per eliminare.

use kernel_spike::concorrenza::{esegui_async, esegui_thread};
use std::collections::BTreeSet;

const ESECUZIONI: usize = 100;
const TASK: usize = 3;
const PASSI: usize = 6;
const SEED: u64 = 20260806;

#[test]
fn c6_future_native_sotto_il_nostro_esecutore_sono_ordinabili_dal_seed() {
    let tracce: BTreeSet<Vec<String>> = (0..ESECUZIONI)
        .map(|_| esegui_async(SEED, TASK, PASSI))
        .collect();

    assert_eq!(
        tracce.len(),
        1,
        "C6 violato: {ESECUZIONI} esecuzioni con seed {SEED} hanno prodotto {} tracce distinte",
        tracce.len()
    );

    // La traccia non deve essere banale: se i task non si interlacciassero,
    // il determinismo sarebbe vero ma privo di significato.
    let traccia = tracce.iter().next().expect("una traccia");
    assert_eq!(traccia.len(), TASK * PASSI);
    let interlacciata = traccia.windows(2).any(|w| {
        w[0].split_whitespace().next() != w[1].split_whitespace().next()
    });
    assert!(
        interlacciata,
        "C6 vacuo: i task non si sono interlacciati, il determinismo non prova nulla"
    );
}

#[test]
fn c6_seed_diversi_producono_interlacciamenti_diversi() {
    assert_ne!(
        esegui_async(SEED, TASK, PASSI),
        esegui_async(SEED + 1, TASK, PASSI),
        "se il seed non cambia l'interlacciamento, non si sta esplorando nulla"
    );
}

/// Controprova. Non è un criterio: serve a stabilire il **confine** di C6.
/// Se anche questa fosse deterministica, C6 non misurerebbe nulla.
#[test]
fn controprova_i_thread_dell_os_non_sono_ordinabili() {
    let tracce: BTreeSet<Vec<String>> = (0..ESECUZIONI)
        .map(|_| esegui_thread(TASK, PASSI))
        .collect();

    assert!(
        tracce.len() > 1,
        "atteso non determinismo dai thread dell'OS, ottenute {} tracce distinte su {ESECUZIONI}. \
         Se è 1, la controprova è troppo debole per stabilire il confine di C6 — \
         va irrobustita, non festeggiata",
        tracce.len()
    );
}
