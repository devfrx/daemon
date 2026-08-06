//! SP-6: il confine dei dati non fidati vive nel sistema di tipi (V19, V20).
//!
//! T4 — `forbid` (non `deny`) perché non è scavalcabile da un `#[allow]` locale:
//! il tentativo produce `E0453: allow(unsafe_code) incompatible with previous forbid`.
//! È ciò che rende l'unica via di aggiramento — la transmutazione — vietata dal
//! compilatore invece che da una convenzione.

#![forbid(unsafe_code)]

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
