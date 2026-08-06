use kernel_spike::sched::World;
use std::time::Instant;

fn traccia(seed: u64) -> Vec<String> {
    let mut w = World::new(seed);
    w.spawn("alfa", 5);
    w.spawn("beta", 5);
    w.run()
}

#[test]
fn c1_stesso_seed_stessa_traccia() {
    assert_eq!(traccia(42), traccia(42));
}

#[test]
fn c2_seed_diversi_tracce_diverse() {
    assert_ne!(traccia(42), traccia(43));
}

/// L'unico uso legittimo dell'orologio vero in tutto lo spike: **misurare** che il
/// tempo virtuale non abbia atteso davvero. Il codice sotto test non lo tocca (C5).
///
/// L'`allow` è necessario perché la regola di T6 è a granularità di **crate** e non
/// distingue il kernel dal test che lo misura. È la prova, su un caso reale, che il
/// meccanismo (a) — il lint — è disattivabile per sito: `parziale`, non `passa`.
/// Con `#![forbid(...)]` questo `allow` sarebbe stato rifiutato (E0453).
#[allow(clippy::disallowed_methods)]
#[test]
fn c3_il_tempo_e_virtuale() {
    let inizio = Instant::now();
    let mut w = World::new(7);
    w.spawn("lento", 5);
    w.sleep_virtuale(5_000); // 5 secondi virtuali
    let _ = w.run();
    assert!(w.now() >= 5_000, "l'orologio virtuale deve essere avanzato");
    assert!(
        inizio.elapsed().as_millis() < 1_000,
        "C3 violato: il test ha atteso davvero"
    );
}

#[test]
fn c4_il_guasto_e_riproducibile() {
    let a = traccia(99);
    let b = traccia(99);
    assert!(
        a.iter().any(|e| e.contains("GUASTO")),
        "il seed 99 deve iniettare almeno un guasto"
    );
    assert_eq!(a, b, "C4 violato: il guasto non è riproducibile");
}
