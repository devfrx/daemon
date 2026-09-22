/**
 * The registry's ONE function in sub-project 2, as the SPA names it on the wire.
 *
 * ⛔ THREE LITERALS IN TWO LANGUAGES, AND NOTHING COUPLES THEM (P-88): the name lives in
 * `crates/kernel/src/serving.rs` (`POLICY_FUNCTION`), and the two arguments are what
 * `MakeRoom::name` answers in `crates/kernel/src/arbiter/policy.rs`. A rename over there and
 * this `Invoke` is refused WITHOUT A MESSAGE -- a function the registry does not hold is refused
 * and nothing is written (§5 of the sub-project 2 design) -- so the fault would be mute. The
 * closing criterion of task 14 compares the literals with a command, as D45 does for the socket
 * name.
 *
 * ⚠️ THE FIXTURES SAY `arbiter.set_policy`, AND THAT IS NOT THIS: the canonical set carries an
 * ARBITRARY value, chosen so that no two encodings are equal. What the core actually holds is
 * what is written here.
 */
export const VRAM_POLICY = {
  name: "vram-policy",
  argument: { remote: "remote", local: "local" },
} as const;

export type PolicyArgument = (typeof VRAM_POLICY.argument)[keyof typeof VRAM_POLICY.argument];
