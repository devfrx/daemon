//! SP-6: il confine dei dati non fidati vive nel sistema di tipi (V19, V20).
//!
//! T4 — `forbid` (non `deny`) perché non è scavalcabile da un `#[allow]` locale:
//! il tentativo produce `E0453: allow(unsafe_code) incompatible with previous forbid`.
//! È ciò che rende l'unica via di aggiramento — la transmutazione — vietata dal
//! compilatore invece che da una convenzione.
//!
//! ⚠️ RICHIAMO DEL 2026-10-03 — la transmutazione non è l'unica via. `forbid(unsafe_code)` la
//! vieta, ma `Instruction::new(String)` e `Untrusted::as_str()` sono pubblici, quindi
//! `Instruction::new(u.as_str().to_string())` porta il testo non fidato nel canale delle
//! istruzioni senza `unsafe`; e i moduli figli vedono i campi privati (`Instruction(u.0)`). Il
//! kernel le dichiara vie A1/A2 e A7 di `crate::boundary`, non chiudibili. Audit del
//! 2026-09-30, AUD-681.

#![forbid(unsafe_code)]

pub mod concorrenza;
pub mod giornale;
pub mod sched;

/// Contenuto che può occupare il canale delle istruzioni.
#[derive(Debug, Clone, PartialEq)]
pub struct Instruction(String);

/// Contenuto proveniente da una fonte esterna. Non è mai un'autorizzazione.
#[derive(Debug, Clone, PartialEq)]
pub struct Untrusted(String);

impl Instruction {
    pub fn new(text: String) -> Self {
        Instruction(text)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Untrusted {
    pub fn new(raw: String) -> Self {
        Untrusted(raw)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// T2 — unico percorso di conversione. Nel kernel reale la chiamata è giornalata.
    ///
    /// ⚠️ RICHIAMO DEL 2026-10-03 — unico percorso NOMINATO, non l'unico: `Instruction::new`,
    /// pubblico, ne apre un altro (la testa di questo file). Audit del 2026-09-30, AUD-681.
    pub fn promote_to_instruction(self, _motivo: &str) -> Instruction {
        Instruction(self.0)
    }
}

/// T3 — l'etichetta è ereditaria: riassumere non ripulisce nulla (V20).
pub fn summarize(input: &Untrusted) -> Untrusted {
    Untrusted(input.0.chars().take(50).collect())
}

/// Il canale delle istruzioni accetta solo `Instruction`.
pub fn build_prompt(system: &Instruction, user: &Instruction) -> String {
    format!("{}\n{}", system.as_str(), user.as_str())
}
