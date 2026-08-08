//! Il simulatore DST: implementazioni finte dei tratti che il kernel dichiara.
//!
//! Vincolata come `kernel`, e per la stessa ragione: una corsa deterministica non può
//! contenere una sorgente di non determinismo, e `HashMap` è la più insidiosa perché non
//! compare in nessun elenco di «chiamate OS» — gotcha #12.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;
