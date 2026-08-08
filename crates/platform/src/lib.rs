//! Le implementazioni reali dei tratti dichiarati dal kernel: filesystem, orologio, rete,
//! processi, confinamento.
//!
//! ⛔ Questa crate USA `std` e USERÀ `unsafe` per la FFI, ed è deliberato: è il posto dove
//! l'I/O deve vivere (ADR-0031, perimetro). Le funzioni qui sotto esistono come
//! CONTRO-SONDE — provano che i divieti del kernel non scattano dove non devono, che è la
//! direzione che si dimentica (§7.1.1 regola 3, gotcha #24). Non cancellarle finché non
//! esiste codice reale che dimostri le stesse due cose.

/// Contro-sonda di `no_std`: `platform` nomina `std::fs` e **compila**.
pub fn contro_sonda_std_compila() -> bool {
    core::mem::size_of::<std::fs::File>() > 0
}

/// Contro-sonda di `forbid(unsafe_code)`: `platform` usa `unsafe` e **compila**.
///
/// Se qualcuno dichiarasse i divieti a livello di workspace, questa funzione smetterebbe
/// di compilare — ed è esattamente ciò che la contro-sonda deve intercettare.
pub fn contro_sonda_unsafe_compila() -> usize {
    let x: u8 = 42;
    let p = &raw const x;
    // SAFETY: `p` deriva da un riferimento a `x`, vivo per tutta la funzione.
    unsafe { *p as usize }
}
