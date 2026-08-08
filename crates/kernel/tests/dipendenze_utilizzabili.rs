//! Che una versione esista non vuol dire che funzioni — gotcha #22.
//! `cargo add bincode` risolve alla 3.0.0, il cui INTERO SORGENTE è un `compile_error!`.
//! Questo test non prova la logica: prova che le voci spedite si compilano e si usano.
//!
//! ⛔ QUESTO FILE NON DICHIARA `#![no_std]`, E NON È UNA DIMENTICANZA.
//! Un test di integrazione è una crate a sé, e il banco di `#[test]` ha bisogno di `std`
//! per girare: con `#![no_std]` qui, il file non collega e fallisce per il motivo
//! sbagliato — gotcha #9. La prova che le dipendenze reggono **senza sistema operativo**
//! non è questo test: è `scripts/gate-no-os.sh` (Task 4), che compila `kernel` per
//! `x86_64-unknown-none`. Quello è il meccanismo; questo è solo il round-trip.

#[test]
fn bincode_2_fa_round_trip_in_no_std() {
    let atteso: u32 = 4096;
    let byte: Vec<u8> =
        bincode::encode_to_vec(atteso, bincode::config::standard()).expect("codifica");
    assert!(!byte.is_empty());
    let (letto, consumati): (u32, usize) =
        bincode::decode_from_slice(&byte, bincode::config::standard()).expect("decodifica");
    assert_eq!(letto, atteso);
    // I byte consumati pareggiano la lunghezza: è la regola che il gotcha #34 impone sul
    // canale a frame, e vale la pena esercitarla da subito.
    assert_eq!(consumati, byte.len());
}

#[test]
fn minicbor_fa_round_trip_in_no_std() {
    let atteso: u32 = 4096;
    let mut byte: Vec<u8> = Vec::new();
    minicbor::encode(atteso, &mut byte).expect("codifica");
    let letto: u32 = minicbor::decode(&byte).expect("decodifica");
    assert_eq!(letto, atteso);
}
