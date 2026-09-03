// Rule B, the other direction. Same argument, written once above.
fn main() {
    let duration = kernel::time::Millis::new(4096);
    let _size: kernel::arbiter::Mib = duration.into();
}
