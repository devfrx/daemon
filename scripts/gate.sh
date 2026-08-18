#!/usr/bin/env bash
# The quality gate, level 2 -- §7.5.1 of the sub-project 1 spec.
#
# CADENCE: on every commit. Level 1 is NOT here, and that is not an oversight: the level 1
# entries never "run" -- they ARE the compiler. If the code compiles, those rules hold, and
# there is no way to skip them or to put them off until tonight.
#
# ⛔ A red from this gate always means "invariant violated", never "questionable style".
# `clippy` runs as code hygiene but has NO voice here: no V depends on it, and rule 1 of the
# admission criterion (§7.1.1) says that in that case it does not get in.
# Level 3 of the catalogue is EMPTY, and that is a decision (§7.4.3).
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1

failures=0
run() {
  echo
  echo "######## $1"
  shift
  if "$@"; then :; else failures=$((failures + 1)); fi
}

# ⛔ `--locked` ON EVERY CARGO CALL OF THE GATE, and it is not tidiness: it makes Cargo.lock an
# INPUT of the gate instead of a SIDE EFFECT of it. Without it the first cargo step re-resolves
# and REWRITES the tracked lockfile, and `gate-deps.sh` -- which measures the transitive graph
# against the ADR-0031 list -- then measures the graph cargo has just invented instead of the one
# that was approved. .gitignore says the lockfile is versioned for exactly the opposite reason.
#
# MEASURED, not reasoned (finding G-5 of the 2026-08-11 audit): with `minicbor` removed from
# crates/kernel/Cargo.toml, `gate-deps.sh` as it was came out `OK -- the two graphs match the two
# lists`, exit 0, having silently rewritten Cargo.lock by 1 insertion and 33 deletions. The
# non-vacuity guard did NOT catch it: the two graphs were non-empty and still different.
#
# ⚠️ THE COST, DECLARED: adding or bumping a dependency is now a TWO-STEP act. Touching a
# manifest alone leaves the gate RED; the lockfile has to be refreshed OUTSIDE the gate -- a
# plain `cargo build` without the flag -- and COMMITTED together with the manifest. That is the
# point rather than the price: ADR-0031 calls adding an entry "a deliberate and reviewable act",
# and a lockfile that the gate updates by itself is neither deliberate nor reviewable.
run "workspace build"                     cargo build --locked --workspace
run "example and compile-fail tests"      cargo test --locked --workspace
run "no-OS gate"                          bash scripts/gate-no-os.sh
run "allow-list on the two graphs"        bash scripts/gate-deps.sh
run "attributes of the constrained crates" bash scripts/gate-attributes.sh
run "documentation consistency"           bash scripts/check-docs.sh

# ⛔ A SEVENTH STEP THAT IS NOT A SEVENTH CONTROL, and the catalogue count stays at six. The
# assertions of both DST campaigns already run inside `cargo test --workspace` above -- that IS
# the cadence constraint 8 of §11 asks for, and nothing here can go red for a reason that check
# has not already caught. This runs them a SECOND time for one reason only: constraint 7 wants
# the WALL TIME PRINTED ON EVERY RUN -- "so that the slowdown becomes visible before it becomes
# a temptation" -- and `cargo test` swallows the output of tests that pass.
#
# ⚠️ TWO COSTS, both declared. The short campaigns run twice, which is ~0.2s. And a failing
# campaign turns the gate red TWICE, from this step and from the second check: that redundancy
# is not a defect but the only proof the step really executes what it claims -- a printing step
# that could not go red would be indistinguishable from one that prints nothing.
run "DST campaigns -- wall time" bash -c '
  cargo test --locked -p simulator --test dst_campaign -- --nocapture &&
  cargo test --locked -p platform --test engine_crash_consistency -- --nocapture'

echo
if [ "$failures" -eq 0 ]; then
  echo "GATE GREEN."
else
  echo "GATE RED -- $failures checks failed."
  exit 1
fi
