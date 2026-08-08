//! L'unico punto che tocca il portachiavi dell'OS.
//!
//! È una crate separata da `platform` per una ragione sola: V34 chiede che «un solo punto
//! legge le credenziali» sia verificabile **staticamente**, e in Rust la granularità
//! verificabile è la crate. Dentro `platform` sarebbe una regola fra moduli, cioè una
//! convenzione. È il motivo per cui le crate sono cinque e non quattro (§1.2).
