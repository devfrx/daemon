#!/usr/bin/env bash
# The attributes of the constrained crates, REALLY declared -- §7.4.1 block A, constraint 2 of §11.
#
# TWO THINGS, AND THE SECOND IS NOT A BONUS: the attributes are declared (below), and the
# constrained crates HAVE NO BUILD SCRIPT (further down, with its own measurement). They live
# in the same file because they share the same list of crates, and that list has ONE home.
#
# THE MEASUREMENT THAT JUSTIFIES IT, and it is not a hypothesis. Without this check one can
# remove '#![forbid(unsafe_code)]' from crates/kernel/src/lib.rs, write a REAL 'unsafe' in
# the kernel, and the gate stays GREEN on all five preceding checks. Measured.
#
# ⛔ THE FOUR CASES OF tests/compile_fail/ DO NOT COVER THE HOLE, and it looks like they do.
# Each one REDECLARES its own attributes and none of them ever names 'kernel::': they prove
# that the MECHANISM bites where it is declared, not that it is declared HERE. The milestone
# Definition of done asks for the second thing and was verifying the first.
#
# ⛔ 'forbid' AND NOT 'deny', and that is the point of the rule, not a preference: 'deny' is
# overridable by a local '#[allow(unsafe_code)]', 'forbid' is not -- E0453, and that is the
# case allow_overrides_forbid.rs. A check looking for the word 'unsafe_code' alone would come
# out GREEN on '#![deny(unsafe_code)]': that is probe 3, and it tells this check apart from
# the naive one.
#
# ⛔ THE CRATES ARE NAMED, NOT DISCOVERED -- exactly as gate-no-os.sh names
# '-p kernel -p simulator'. 'platform', 'secrets' and 'daemon' are NOT checked: platform
# uses 'std' and 'unsafe' ON PURPOSE, as counter-probes of the kernel's prohibitions. A check
# that fired there too would be red for the wrong reason -- gotcha #24.
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1

failures=0
report() { echo "  ✗ $*"; failures=$((failures + 1)); }

# Single home for the constrained files: a crate is added HERE, not elsewhere.
CONSTRAINED="crates/kernel/src/lib.rs
crates/simulator/src/lib.rs"

# ⚠️ ANCHORED TO THE START OF THE LINE, and that is not formatting. Without '^[[:space:]]*'
# a COMMENTED-OUT attribute -- '// #![forbid(unsafe_code)]' -- would satisfy the check: it
# is the worst false negative, because the line looks present to whoever reads the diff.
# With the anchor the '//' comes first and the line does not match. DECLARED COST: an
# attribute buried in a block comment /* ... */ still escapes. Closing that would mean a
# parser, and the remedy would be more fragile than the hole.
#
# ⚠️ The 'forbid' list is PERMISSIVE about the rest: '#![forbid(unsafe_code, missing_docs)]'
# has to pass. Demanding the exact line would turn a legitimate widening of the prohibition
# red -- gotcha #9 applied to the check.
RE_NO_STD='^[[:space:]]*#!\[[[:space:]]*no_std[[:space:]]*\]'
RE_FORBID='^[[:space:]]*#!\[[[:space:]]*forbid[[:space:]]*\([^)]*\bunsafe_code\b[^)]*\)[[:space:]]*\]'
RE_DENY='^[[:space:]]*#!\[[[:space:]]*deny[[:space:]]*\([^)]*\bunsafe_code\b[^)]*\)[[:space:]]*\]'
RE_ALLOC='^[[:space:]]*extern[[:space:]]+crate[[:space:]]+alloc[[:space:]]*;'

examined=0
for f in $CONSTRAINED; do
  echo "== $f =="

  # Non-vacuity guard: a file that IS NOT THERE is a failure, not "nothing to check".
  # Without it, renaming or moving a crate is enough for the check to stop checking while
  # EXITING GREEN -- gotcha #26, already met twice in this milestone.
  if [ ! -f "$f" ]; then
    report "$f DOES NOT EXIST. It is not 'nothing to check': it is the check not checking."
    continue
  fi
  examined=$((examined + 1))

  grep -qE "$RE_NO_STD" "$f" || report "$f does not declare '#![no_std]'."
  grep -qE "$RE_ALLOC"  "$f" || report "$f does not declare 'extern crate alloc;'."

  if grep -qE "$RE_DENY" "$f"; then
    report "$f declares 'deny(unsafe_code)' where 'forbid' is required."
    echo "      ⛔ REMEDY: 'forbid'. 'deny' is overridable by a local '#[allow(unsafe_code)]'"
    echo "      and the prohibition goes back to being a preference -- constraint 2 of §11."
  fi

  if ! grep -qE "$RE_FORBID" "$f"; then
    report "$f does not declare '#![forbid(unsafe_code)]'."
    echo "      ⛔ Without it, an 'unsafe' in the kernel compiles and NO other check sees it:"
    echo "      the compile_fail cases redeclare their own attributes and do not look here."
  fi

  # ⛔ NO BUILD SCRIPT ON THE CONSTRAINED CRATES, AND THE HOLE WAS MEASURED, not feared.
  # A 'crates/kernel/build.rs' calling std::time::SystemTime::now(), std::fs::metadata() and
  # std::env::var(), injecting the result with 'cargo:rustc-env', left the gate GREEN ON SIX
  # CHECKS OUT OF SIX. Each one misses the target for its own reason:
  #   cargo build / cargo test  a build script is a SEPARATE target, compiled FOR THE HOST:
  #                             using 'std' there is its job, not a violation.
  #   gate-no-os.sh             build scripts compile for the host EVEN WITH --target. It does
  #                             not merely miss it: it RUNS it.
  #   gate-deps.sh              it reads the GRAPH. A build script with no dependencies of its
  #                             own adds no node. ⚠️ A '[build-dependencies]' IS caught: the
  #                             invisible one is the script WITHOUT dependencies.
  #   gate-attributes.sh        it read only 'src/lib.rs'. 'build.rs' has attributes of its own
  #                             and the 'forbid' of 'lib.rs' does not reach it. That is why the
  #                             check lives HERE: this is the script whose blind spot it was.
  #   check-docs.sh             it does not look at the code at all.
  #
  # WHAT IT VIOLATES: I3 -- OS calls inside the kernel crate -- and V29, the third property
  # that cannot be retrofitted: 'cargo:rustc-env' plus 'env!()' BAKES INTO THE KERNEL a value
  # read from the world at build time. It is gotcha #28 to the letter -- a parameter that is
  # not delivered is a constant, and a constant is invisible.
  #
  # ⛔ PERIMETER: 'kernel' and 'simulator' ONLY, the same two the list above names, and the
  # directory is DERIVED from that list so there is no second place to keep aligned.
  # 'platform', 'secrets' and 'daemon' MAY have a build script: that is exactly where the I/O
  # has to live, and a check firing there too would be red for the wrong reason -- gotcha #24.
  crate_dir=$(dirname "$(dirname "$f")")
  manifest="$crate_dir/Cargo.toml"

  if [ -f "$crate_dir/build.rs" ]; then
    report "$crate_dir/build.rs exists: a constrained crate must have NO build script."
    echo "      ⛔ REMEDY: REMOVE IT. It is the remedy of an I3 violation on the shipped graph,"
    echo "      not the one of the build graph: there is nothing to add to any list."
    echo "      WHY IT IS NOT PEDANTRY: that file is compiled FOR THE HOST and runs with the"
    echo "      whole of 'std' at hand -- SystemTime::now(), fs, env -- and with 'cargo:rustc-env'"
    echo "      plus 'env!()' it BAKES what it read into the kernel as a constant. I3 and V29,"
    echo "      gotcha #28. AND NO OTHER CHECK SEES IT: build and test compile it because that"
    echo "      is a build script's job, gate-no-os.sh builds it for the host even with --target"
    echo "      and RUNS it, gate-deps.sh adds no node for a script with no dependencies of its"
    echo "      own, and check-docs.sh does not read code. Measured: six out of six GREEN."
  fi

  # ⚠️ SECOND ROUTE, and it is the one a rename takes: 'build = \"gen.rs\"' in the manifest is
  # the same object under another name, and the existence test above does not see it. What
  # tells a declaration apart from 'build = false' -- which DISABLES the script and must stay
  # green, gotcha #24 again -- is that a PATH is a string or an array while the disabling form
  # is a bare boolean.
  #
  # ⛔ THIS PATTERN ANCHORED ON THE DOUBLE QUOTE UNTIL 2026-08-11, AND IT WAS A FALSE NEGATIVE
  # ON I3 -- the one way this check must not fail. TOML has two string forms, and the literal
  # one is single-quoted: 'build = '"'"'gen.rs'"'"'' is the SAME VALUE and slipped through.
  # Measured rather than reasoned: a crate declaring it that way builds on cargo 1.95.0 (exit
  # 0) and its script RUNS -- the build directory's 'output' file carried the injected
  # 'cargo:rustc-env'. With the other five checks blind by construction (see above), the gate
  # came out GREEN on six of six with a build script reading the clock, the filesystem and the
  # environment inside the kernel. Gotcha #28, reopened by a quoting character.
  #
  # 📌 The lesson is the shape, not the character: ANCHOR ON THE KEY, NEVER ON THE DELIMITER.
  # The array form 'build = [...]' is rejected by stable cargo today and is accepted here
  # anyway, so the day it stabilises this check does not have to be remembered.
  # ⚠️ ONE MANIFEST IS ENOUGH, and that is measured too, not assumed: 'build' is NOT among the
  # keys '[workspace.package]' can hand down. Tried on cargo 1.95.0 -- 'build.workspace = true'
  # is rejected at parse time with "invalid type: map, expected a boolean, string or array".
  # So the crate's own manifest is the only place a build script can be declared from.
  if [ ! -f "$manifest" ]; then
    report "$manifest DOES NOT EXIST: the build-script check would read nothing and pass."
  elif grep -qE "^[[:space:]]*build[[:space:]]*=[[:space:]]*[\"'[]" "$manifest"; then
    report "$manifest declares a build script."
    echo "      ⛔ REMEDY: REMOVE IT. Renaming 'build.rs' does not change what it is -- a host"
    echo "      target with all of 'std', invisible to the other five checks. See above."
  fi
done

# Second non-vacuity guard, on the same gotcha #26 but one level up: if the list above were
# emptied, the loop would not run and the script would exit 0 IN SILENCE.
if [ "$examined" -eq 0 ]; then
  report "no file examined: the list of constrained files is empty, and a check that reads nothing proves nothing."
fi

echo
if [ "$failures" -eq 0 ]; then
  echo "OK -- kernel and simulator declare the three attributes, neither uses 'deny', and"
  echo "neither has a build script."
else
  echo "$failures violations. A missing attribute is not style: it is the invariant vanishing without leaving a red anywhere else."
  exit 1
fi
