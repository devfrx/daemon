// ⛔ È IL CASO CHE GIUSTIFICA `forbid` INVECE DI `deny`: senza, il vincolo 2 della §11
// resta una preferenza. Con `deny` l'`#[allow]` vince e l'`unsafe` non è più segnalato
// (non compila lo stesso, ma per il rumore di codegen); `forbid` lo rifiuta: E0453.
#![no_std]
#![forbid(unsafe_code)]

#[allow(unsafe_code)]
fn deferenzia() -> u8 {
    let x: u8 = 1;
    let p = &raw const x;
    unsafe { *p }
}

fn main() {}
