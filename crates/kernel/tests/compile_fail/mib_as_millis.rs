// Rule A, one direction: VRAM is not a duration. §5.1 -- "swapping MiB for milliseconds
// MUST NOT COMPILE", the same mechanism that separates `Instruction` from `Untrusted`.
fn takes_a_duration(_value: kernel::time::Millis) {}

fn main() {
    takes_a_duration(kernel::arbiter::Mib::new(4096));
}
