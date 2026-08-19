// Rule B: no `From`/`Into` PATH exists between the two. The day somebody writes the impl
// this case starts COMPILING, and trybuild reports that outright as `error` instead of
// through its oracle -- so a bulk regeneration of the `.stderr` files cannot disarm it.
// Gotcha #42, strong form.
fn main() {
    let size = kernel::arbiter::Mib::new(4096);
    let _duration: kernel::time::Millis = size.into();
}
