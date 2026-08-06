//! C7 — I/O durevole iniettabile, crash riproducibile per seed, passo in dubbio
//! rilevabile. V29 · ADR-0007 · ADR-0021.

use kernel_spike::giornale::{esegui, passo_in_dubbio, Giornale, GiornaleCadente};

const PASSI: u64 = 8;
const SCRITTURE_PREVISTE: u32 = (PASSI * 2) as u32;

fn esecuzione(seed: u64) -> Vec<String> {
    let mut g = GiornaleCadente::nuovo(seed, SCRITTURE_PREVISTE);
    esegui(&mut g, PASSI);
    g.righe
}

#[test]
fn c7_stesso_seed_stessa_traccia_crash_incluso() {
    for seed in [1_u64, 7, 42, 99, 20260806] {
        assert_eq!(
            esecuzione(seed),
            esecuzione(seed),
            "C7 violato: il crash non è riproducibile con seed {seed}"
        );
    }
}

#[test]
fn c7_il_crash_avviene_davvero() {
    // Un test di riproducibilità su un'esecuzione che non cade mai è vacuo.
    let cadute = (0..50_u64)
        .filter(|s| esecuzione(*s).len() < SCRITTURE_PREVISTE as usize)
        .count();
    assert!(
        cadute > 0,
        "C7 vacuo: nessuno dei 50 seed ha prodotto una caduta"
    );
}

#[test]
fn c7_il_passo_in_dubbio_e_rilevabile() {
    // Si cerca un seed che cada su un numero DISPARI di scritture, cioè fra
    // l'intento e l'esito: è il caso che ADR-0007 chiama "passo in dubbio".
    let seed_con_dubbio = (0..200_u64)
        .find(|s| passo_in_dubbio(&esecuzione(*s)).is_some())
        .expect("almeno un seed su 200 deve cadere fra intento ed esito");

    let righe = esecuzione(seed_con_dubbio);
    let passo = passo_in_dubbio(&righe).expect("il dubbio deve essere rilevabile");

    assert!(
        righe.iter().any(|r| r.contains(&format!("passo={passo} INTENTO"))),
        "il passo in dubbio deve avere un intento registrato"
    );
    assert!(
        !righe.iter().any(|r| r.contains(&format!("passo={passo} ESITO"))),
        "il passo in dubbio non deve avere un esito"
    );

    // Il seed va registrato: un risultato senza seed non è valido.
    println!("C7 — seed con passo in dubbio: {seed_con_dubbio}, passo {passo}");
}

#[test]
fn c7_senza_crash_nessun_passo_resta_in_dubbio() {
    let mut g = GiornaleCadente::senza_crash();
    esegui(&mut g, PASSI);
    assert_eq!(g.righe.len(), SCRITTURE_PREVISTE as usize);
    assert_eq!(
        passo_in_dubbio(&g.righe),
        None,
        "senza crash non deve esserci alcun dubbio: il rilevatore darebbe falsi positivi"
    );
}

#[test]
fn c7_l_ordine_e_write_ahead_intento_prima_dell_esito() {
    let mut g = GiornaleCadente::senza_crash();
    esegui(&mut g, 3);
    let ordine: Vec<&str> = g
        .righe
        .iter()
        .map(|r| if r.contains("INTENTO") { "I" } else { "E" })
        .collect();
    assert_eq!(
        ordine,
        vec!["I", "E", "I", "E", "I", "E"],
        "ADR-0007: l'intento è durevole PRIMA dell'effetto, l'esito dopo"
    );
}

#[test]
fn c7_il_giornale_e_sostituibile_senza_toccare_il_codice_sotto_test() {
    // Un secondo doppio, che conta soltanto. Se `esegui` conoscesse il filesystem
    // questo non compilerebbe: il confine è nel tipo, non nella disciplina.
    struct Contatore(u32);
    impl Giornale for Contatore {
        fn intento(&mut self, _p: u64, _d: &str) -> Result<(), kernel_spike::giornale::Caduto> {
            self.0 += 1;
            Ok(())
        }
        fn esito(&mut self, _p: u64, _e: &str) -> Result<(), kernel_spike::giornale::Caduto> {
            self.0 += 1;
            Ok(())
        }
    }
    let mut c = Contatore(0);
    esegui(&mut c, PASSI);
    assert_eq!(c.0, SCRITTURE_PREVISTE);
}
