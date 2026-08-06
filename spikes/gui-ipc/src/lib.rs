//! Protocollo del prototipo. Non è il protocollo del kernel: è il minimo che serve a
//! misurare P1–P4.

use serde::{Deserialize, Serialize};

/// I tre canali logici che devono convivere sullo stesso trasporto.
/// Sono logici, non fisici: I4 chiede **un** trasporto e **uno** schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Canale {
    /// streaming token per token (G4)
    Token,
    /// stato di degrado (G9, ADR-0019)
    Stato,
    /// occupazione del contesto e costo (G11, G12)
    Metriche,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Messaggio {
    pub canale: Canale,
    /// progressivo globale: serve a P1, i messaggi persi si contano dai buchi
    pub seq: u64,
    /// micros da EPOCH all'emissione: serve a P2
    pub emesso_micros: u128,
    pub carico: String,
}

pub const TOTALE: u64 = 2000;
pub const DURATA_MS: u64 = 10_000;

/// Nome del canale locale. `interprocess` lo traduce in named pipe su Windows e in
/// socket unix su Linux, senza che il codice cambi (G19).
pub const NOME: &str = "gui-ipc-spike";

pub fn ora_micros() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("orologio prima di EPOCH")
        .as_micros()
}
