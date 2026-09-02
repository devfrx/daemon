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
# assertions of every DST campaign named below already run inside `cargo test --workspace`
# above -- that IS the cadence constraint 8 of §11 asks for, and nothing here can go red for a
# reason that check has not already caught. This runs them a SECOND time for one reason only:
# constraint 7 wants the WALL TIME PRINTED ON EVERY RUN -- "so that the slowdown becomes
# visible before it becomes a temptation" -- and `cargo test` swallows the output of tests
# that pass.
#
# ⚠️ EVERY CAMPAIGN HAS TO BE ADDED HERE BY NAME, and that is the lesson rather than the
# history: this step names its targets ONE BY ONE, so a campaign absent from the list is
# SILENT. It has been learned TWICE, both times the same way -- the bench in place, the line
# not yet added, and the gate GREEN over it:
#   - the arbiter's, task 12 of milestone 5. Measured on 2026-08-25: the gate came out GREEN
#     and its output contained the four `DST arbiter` lines ZERO times.
#   - the gui's and the worker's, task 9 of milestone 6. Measured on 2026-09-02:
#     `bash scripts/gate.sh | grep -c "DST gui death\|DST worker kills"` came out 0 with the
#     gate GREEN, and greater than zero with the two targets in the list -- the same command
#     in both directions.
# ⛔ AND THE NUMBER OF TARGETS IS WRITTEN NOWHERE BUT IN THE `run` BELOW, which IS the list.
# This comment said "all three DST campaigns" until 2026-09-02 and the campaigns were five: a
# tally in a comment ages the day somebody adds one, and the list cannot (gotcha #31).
#
# ⚠️ TWO COSTS, both declared. The short campaigns run twice: RE-MEASURED on 2026-09-02,
# the second pass costs 0.81s of test time -- dst_campaign 0.21s, arbiter_campaign 0.16s,
# gui_death_campaign 0.01s, worker_kill_campaign 0.02s, engine_crash_consistency 0.41s.
# ⚠️ THE FIGURE IS RE-MEASURED WHENEVER THIS LIST CHANGES AND NEVER REALIGNED FROM MEMORY,
# which is why it carries its date: it said "~0.2s" at milestone 4 with two campaigns and
# "1.45s" on 2026-08-25 with three. ⛔ AND THE RE-MEASUREMENT IS NOT THE ARITHMETIC ANYBODY
# WOULD HAVE PREDICTED: the three targets of that 1.45s cost 0.78s of today's 0.81s -- the
# same binaries, the same machine, at roughly half the price -- so the two new campaigns add
# 0.03s and the figure still FELL. Which is the point: it is an ORDER OF MAGNITUDE and not a
# constant -- the arbiter binary alone, same command, came out 0.63s and 1.53s within one
# session on this machine -- so nothing asserts on it, and what the gate collects is the
# printed line, for a reader to compare against the run before. And a failing
# campaign turns the gate red TWICE, from this step and
# from the second check: that redundancy is not a defect but the only proof the step really
# executes what it claims -- a printing step that could not go red would be indistinguishable
# from one that prints nothing.
run "DST campaigns -- wall time" bash -c '
  cargo test --locked -p simulator --test dst_campaign -- --nocapture &&
  cargo test --locked -p simulator --test arbiter_campaign -- --nocapture &&
  cargo test --locked -p simulator --test gui_death_campaign -- --nocapture &&
  cargo test --locked -p simulator --test worker_kill_campaign -- --nocapture &&
  cargo test --locked -p platform --test engine_crash_consistency -- --nocapture'

echo
if [ "$failures" -eq 0 ]; then
  echo "GATE GREEN."
else
  echo "GATE RED -- $failures checks failed."
  exit 1
fi
