// ⛔ È IL CASO CHE GIUSTIFICA `forbid` INVECE DI `deny`, e senza di lui il vincolo 2 della
// §11 è una preferenza stilistica. Con `deny`, questo file COMPILA. Con `forbid`, il
// compilatore rifiuta l'`#[allow]` stesso: E0453.
#![no_std]
#![forbid(unsafe_code)]

#[allow(unsafe_code)]
fn deferenzia() -> u8 {
    let x: u8 = 1;
    let p = &raw const x;
    unsafe { *p }
}

fn main() {}
