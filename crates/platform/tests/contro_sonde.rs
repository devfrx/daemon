//! Le contro-sonde di §7.1.1 regola 3: i divieti del kernel NON scattano dove non devono.
//! Senza queste, una regola troppo larga passerebbe per una regola che funziona — è la
//! direzione che si dimentica, e in M-3 la sonda decisiva è stata proprio questa.

#[test]
fn platform_nomina_std_e_compila() {
    assert!(platform::contro_sonda_std_compila());
}

#[test]
fn platform_usa_unsafe_e_compila() {
    assert_eq!(platform::contro_sonda_unsafe_compila(), 42);
}
