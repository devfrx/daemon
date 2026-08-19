// Rule A, the OTHER direction. Neither type is the stricter one -- a memory size and a
// duration each go wrong in their own way -- which is why both directions have a case
// here, exactly as `Monotonic`/`WallTime` do. A guard written in one direction only left
// the gate GREEN on the dangerous side once already (the widened `V29` row of §7.4.1).
fn takes_a_size(_value: kernel::arbiter::Mib) {}

fn main() {
    takes_a_size(kernel::time::Millis::new(4096));
}
