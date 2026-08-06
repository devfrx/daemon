//! C7 — l'**I/O durevole** è iniettabile, e un crash al confine di persistenza è
//! riproducibile dal seed.
//!
//! V29 elenca quattro cose iniettabili: tempo, casualità, **I/O**, scheduling. C1–C6
//! coprono le altre tre. Questa è la quarta, e non è la meno importante: la tecnica di
//! verifica di Q5 in ADR-0021 è la *crash-injection ai confini di persistenza*, e il
//! confine principale è il giornale write-ahead di ADR-0007.
//!
//! La proprietà che si compra con la doppia scrittura: **un passo con intento e senza
//! esito è in dubbio, e il dubbio è sempre rilevabile.**

use crate::sched::Rng;

/// Il processo è caduto al confine di persistenza. Non è un errore applicativo:
/// è la simulazione dello spegnimento.
#[derive(Debug, PartialEq)]
pub struct Caduto;

/// Il confine sostituibile. Il codice sotto test non conosce nessun'altra via
/// verso il durevole: nessuna chiamata al filesystem, mai.
pub trait Giornale {
    fn intento(&mut self, passo: u64, descrizione: &str) -> Result<(), Caduto>;
    fn esito(&mut self, passo: u64, esito: &str) -> Result<(), Caduto>;
}

/// Doppio in memoria che cade a una scrittura scelta dal seed.
pub struct GiornaleCadente {
    pub righe: Vec<String>,
    cade_alla: u32,
    scritture: u32,
}

impl GiornaleCadente {
    /// `cade_alla == None` significa nessun crash: serve al caso di controllo.
    pub fn nuovo(seed: u64, scritture_previste: u32) -> Self {
        let mut rng = Rng::new(seed);
        GiornaleCadente {
            righe: Vec::new(),
            cade_alla: rng.below(scritture_previste as u64) as u32,
            scritture: 0,
        }
    }

    pub fn senza_crash() -> Self {
        GiornaleCadente {
            righe: Vec::new(),
            cade_alla: u32::MAX,
            scritture: 0,
        }
    }

    fn scrivi(&mut self, riga: String) -> Result<(), Caduto> {
        if self.scritture == self.cade_alla {
            return Err(Caduto);
        }
        self.scritture += 1;
        self.righe.push(riga);
        Ok(())
    }
}

impl Giornale for GiornaleCadente {
    fn intento(&mut self, passo: u64, descrizione: &str) -> Result<(), Caduto> {
        self.scrivi(format!("passo={passo} INTENTO {descrizione}"))
    }
    fn esito(&mut self, passo: u64, esito: &str) -> Result<(), Caduto> {
        self.scrivi(format!("passo={passo} ESITO {esito}"))
    }
}

/// Esegue `passi` passi scrivendo write-ahead: **intento prima dell'effetto, esito
/// dopo**. Si ferma alla caduta.
pub fn esegui<G: Giornale>(giornale: &mut G, passi: u64) {
    for p in 0..passi {
        if giornale.intento(p, "chiamata a strumento").is_err() {
            return;
        }
        // qui, nel kernel reale, avviene l'effetto
        if giornale.esito(p, "ok").is_err() {
            return;
        }
    }
}

/// Riconciliazione: il passo con intento e senza esito. `None` se non ce n'è.
///
/// È la proprietà che ADR-0007 compra con la seconda scrittura durevole. Un giornale
/// scritto *dopo* l'esecuzione non potrebbe rispondere a questa domanda.
pub fn passo_in_dubbio(righe: &[String]) -> Option<u64> {
    let mut aperto: Option<u64> = None;
    for riga in righe {
        let passo: u64 = riga
            .split_whitespace()
            .next()
            .and_then(|c| c.strip_prefix("passo="))
            .and_then(|n| n.parse().ok())?;
        if riga.contains(" INTENTO ") {
            aperto = Some(passo);
        } else if riga.contains(" ESITO ") && aperto == Some(passo) {
            aperto = None;
        }
    }
    aperto
}
