#!/usr/bin/env bash
# The web step of the quality gate -- §8 of the sub-project 2 design.
#
# ⛔ ONE GATE, NOT TWO, and that is decision 34 in person. There is no separate CI job that runs
# only when `gui/` changes: `CLAUDE.md` says the gate is launched with ONE command, and a kernel
# change that breaks the fixtures MUST run the SPA probes that read them. The declared cost: this
# step runs for a documentation-only commit too, exactly as the Rust steps already do.
#
# ⛔ `set -e` HERE AND NOT THE FAILURE COUNTER OF `gate.sh`: §8 says this step "stops at the first
# red". The steps below are a chain -- `npm run build` on a tree `npm ci` failed to install would
# fail for the wrong reason -- so the first red is the only one worth reading.
set -euo pipefail
cd "$(dirname "$0")/.." || exit 1

# ⛔ `--locked` FOR THE SAME REASON AS EVERY OTHER CARGO CALL OF THE GATE: the fake core's
# `Cargo.lock` is committed (§8, global constraint 7), so it is an INPUT here and not a side effect.
# ⚠️ AND THE FAKE CORE COMPILES IN ITS OWN `target/`, measured on 2026-09-15 with `cargo metadata`
# (P-104): it is outside the workspace, so it rebuilds `kernel`, `platform` and `simulator` rather
# than reusing `<root>/target`. That is the declared cost of §8, not a misconfiguration.
#   2026-09-22: `cargo test --locked` on the fake core, cold 20.9s, warm 2.5s -- measured by task 12
#   (step 10) and re-run here (R5-17). An order of magnitude, dated; nothing asserts on it.
echo "-------- gui: fake core"
cargo test --locked --manifest-path gui/fake-core/Cargo.toml
# ⛔ THE FAKE CORE'S OWN LOCKFILE IS AUDITED TOO (D83). It is seeded from the root's and pins the same
# crates, but it is a SECOND lockfile, and the `cargo audit` of `gate.sh` reads the root's alone. Same
# verdict expected on the same crates; a divergence between the two is task 12's comparison script.
echo "-------- gui: fake core advisories"
cargo audit --file gui/fake-core/Cargo.lock

cd gui
# `npm ci` is the twin of `--locked`: a manifest and a lockfile that disagree are a red, and
# `engine-strict=true` in `.npmrc` makes a wrong Node a red HERE, with the reason printed.
# ⚠️ AND `npm ci` DELETES `gui/node_modules/` FIRST, EVERY RUN -- so on every commit, documentation
# included: a linked or hand-patched package does not survive the gate, and nothing else says so
# (M-3 of the review, E199).
echo "-------- gui: install"
npm ci --no-audit --no-fund
# `vue-tsc` inside `build` is the level 1 of the web world, the way `rustc` is for the kernel.
echo "-------- gui: build"
npm run build
echo "-------- gui: probes"
npm test
# ⛔ LINT LAST, AND THE ORDER IS §8's, NOT OURS. §8 fixed "npm ci, npm run build, npm test"; this
# step appends rather than reordering an approved section. And the probes carry more meaning than
# the lint, so they must not sit behind it.
echo "-------- gui: lint"
npm run lint
# ⛔ THE SECOND WORLD OF X-3. `npm ci` above installs what the lockfile pins; this asks the registry
# how those pins ARE. Same shape as `cargo audit` in `gate.sh`, same network cost, and the same
# property: it can go red without a commit.
#
# ⛔ NO `--audit-level`, AND THAT IS A DECISION. Measured on 2026-09-15 on the whole dependency set
# of sub-project 2 -- 369 packages -- `npm audit` found 0 vulnerabilities in about six seconds, so
# the noise to tune is ZERO and a threshold picked today would be a knob nobody calibrated, sitting
# ready as a shortcut for the first inconvenient advisory. When one arrives the way out is to
# upgrade, or to declare the exception WITH ITS DATE -- not to lower the bar quietly.
echo "-------- gui: advisories"
npm audit
