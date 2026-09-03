// `I2 · §5.3`: a non-preemptible grant has NOWHERE to put a revocation. §5.3 point 3 wants
// this "not constructible", not "checked at runtime" -- so the state is not forbidden, it
// cannot be spelled.
fn main() {
    let _impossible = kernel::arbiter::Activity::NonPreemptible(
        kernel::arbiter::PreemptibleState::Revoking {
            deadline: kernel::time::Monotonic::from_millis(1_000),
        },
    );
}
