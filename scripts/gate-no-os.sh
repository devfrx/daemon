#!/usr/bin/env bash
# The no-OS gate -- §7.3.2 of the sub-project 1 spec.
#
# IT ADDS TO the allow-list, it does not replace it. The two fail in complementary ways:
# the list ENUMERATES and names the culprit ("X unty <- kernel -> bincode -> unty"); the
# gate PROVES and catches a crate ALREADY ON THE LIST that reaches the OS by an unforeseen
# route -- feature unification -- but only says "no" without saying who. The list is the
# diagnosis, the gate is the proof.
# ⚠️ AND IT ALSO CATCHES A CONSTRAINED CRATE THAT WRITES `extern crate std;` -- the one line
# `#![no_std]` does not refuse: E0463 on x86_64-unknown-none, while the same file builds for the
# host. Measured on 2026-10-06 on a scratch crate with toolchain 1.95.0, and on the kernel itself
# by condition 2 of §7.2 of the closure design.
# RECALL OF 2026-10-06 -- audit of 2026-09-30, AUD-587.
#
# TARGET: x86_64-unknown-none, and it is not a detail. It has to differ from the real
# target along ONE SINGLE dimension, the absence of the OS. thumbv7em-none-eabihf differs
# along four (arch, pointer, 64-bit atomics) and is a source of reds for the wrong reason
# -- gotcha #9 applied to the target.
#
# ⛔ DO NOT add --workspace. The command names the constrained crates one by one, and that is not
# a convenience: with --workspace the gate fails on `platform` with "can't find crate for
# std", that is, right reason and wrong crate. It is probe B3, which did not exist.
# ⚠️ AND THE SAME LIST IS WRITTEN BY HAND IN TWO MORE SCRIPTS -- `CRATES` in gate-deps.sh and
# `CONSTRAINED` in gate-attributes.sh -- and NOTHING checks that the three agree: a crate that
# joins the constrained set goes into all three.
# RECALL OF 2026-10-06 -- audit of 2026-09-30, AUD-684.
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1

TARGET=x86_64-unknown-none

echo "== no-OS gate -- $TARGET =="

if ! rustup target list --installed | grep -qx "$TARGET"; then
  echo "  ✗ target $TARGET not installed."
  echo "    rustup target add $TARGET   (or rely on rust-toolchain.toml)"
  echo "    Without it the gate would be red for the wrong reason -- constraint 4 of §11."
  exit 1
fi

# ⚠️ `--locked` here too, and for the same reason as in gate.sh: this script is also run on its
# own, and a step that re-resolves the lockfile would hand `gate-deps.sh` a graph nobody approved
# -- finding G-5. It is NOT a second control: it is the same control taking its input from the
# committed lockfile instead of from whatever cargo resolves today.
if cargo build --locked -p kernel -p simulator --target "$TARGET" 2>&1; then
  echo "  ✓ kernel and simulator build without an operating system"
  exit 0
else
  echo "  ✗ kernel or simulator do NOT build for $TARGET."
  echo "    The gate does not say who dragged it in: look at the output of"
  echo "    scripts/gate-deps.sh, which names the bounce."
  exit 1
fi
