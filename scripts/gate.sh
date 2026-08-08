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

run "workspace build"                     cargo build --workspace
run "example and compile-fail tests"      cargo test --workspace
run "no-OS gate"                          bash scripts/gate-no-os.sh
run "allow-list on the two graphs"        bash scripts/gate-deps.sh
run "attributes of the constrained crates" bash scripts/gate-attributes.sh
run "documentation consistency"           bash scripts/check-docs.sh

echo
if [ "$failures" -eq 0 ]; then
  echo "GATE GREEN."
else
  echo "GATE RED -- $failures checks failed."
  exit 1
fi
