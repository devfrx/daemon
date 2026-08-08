#!/usr/bin/env bash
# The attributes of the constrained crates, REALLY declared -- §7.4.1 block A, constraint 2 of §11.
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
done

# Second non-vacuity guard, on the same gotcha #26 but one level up: if the list above were
# emptied, the loop would not run and the script would exit 0 IN SILENCE.
if [ "$examined" -eq 0 ]; then
  report "no file examined: the list of constrained files is empty, and a check that reads nothing proves nothing."
fi

echo
if [ "$failures" -eq 0 ]; then
  echo "OK -- kernel and simulator declare the three attributes, and neither uses 'deny'."
else
  echo "$failures violations. A missing attribute is not style: it is the invariant vanishing without leaving a red anywhere else."
  exit 1
fi
