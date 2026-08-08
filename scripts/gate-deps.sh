#!/usr/bin/env bash
# The allow-list on the transitive graph of kernel and simulator -- ADR-0031, §7.3.1.
#
# TWO GRAPHS, TWO ERRORS, TWO OPPOSITE REMEDIES. It is not completeness: merging them
# teaches the "add it to the list" reflex ALSO for an I3 violation, where adding IS the
# violation. And a check that looks only at what ships lets through in silence exactly the
# event that ADR-0031 says to review.
#
#   shipped     -> `cargo tree -e normal,no-proc-macro` -> error "I3 violated"
#                  REMEDY: remove the dependency. Adding it to the list is NOT a remedy.
#   build-only  -> the complement between `-e no-dev` and the line above
#                  REMEDY: assess it and add it to the list, with a justification.
#   dev-only    -> excluded, and the exclusion is PROVEN (see the guard below).
#
# ⚠️ `cargo tree` and not `cargo metadata`: the latter does not resolve features. Measured,
# gotcha #23: eleven crates reported against the two real ones, that is 5x over-reporting.
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1

failures=0
report() { echo "  ✗ $*"; failures=$((failures + 1)); }

# --- The two lists. Single home: §7.3.1 of the spec; this file mirrors it. ---
SHIPPED="bincode
kernel
minicbor
simulator
unty"

BUILD_ONLY="bincode_derive
minicbor-derive
proc-macro2
quote
syn
unicode-ident
virtue"

# ⛔ THE CHARACTER CLASS OF THE grep IS A MEASURED CONSTRAINT, NOT FORMATTING.
# With `[a-z0-9_-]` -- as it was written -- a crate with an UPPERCASE name was dropped by
# the filter, so it did not show up among the intruders and the gate came out GREEN.
# Measured with `Inflector`, which is a real crate: in the kernel's shipped graph, exit 0.
# It is a false negative on I3, that is, the worst way to fail for this check. Probe N5.
# Whoever "simplifies" it reopens the blind spot, and the gate goes green without saying so.
names() { sed 's/^[^a-zA-Z0-9_-]*//' | awk '{print $1}' | grep -E '^[A-Za-z0-9_-]+$' | sort -u; }

for crate in kernel simulator; do
  echo "== $crate: SHIPPED graph =="
  shipped_graph=$(cargo tree -p "$crate" -e normal,no-proc-macro --prefix none 2>/dev/null | names)
  intruders=$(comm -23 <(printf '%s\n' "$shipped_graph") <(printf '%s\n' "$SHIPPED" | sort -u))
  if [ -n "$intruders" ]; then
    for i in $intruders; do
      report "I3 violated -- $crate ships '$i', which is not on the list."
      echo "      ⛔ REMEDY: REMOVE the dependency. Adding it to the list is not a remedy."
      echo "      Where it comes from:"
      cargo tree -p "$crate" -e normal,no-proc-macro -i "$i" 2>/dev/null | sed 's/^/        /'
    done
  fi

  echo "== $crate: BUILD graph =="
  full_graph=$(cargo tree -p "$crate" -e no-dev --prefix none 2>/dev/null | names)
  build_graph=$(comm -13 <(printf '%s\n' "$shipped_graph") <(printf '%s\n' "$full_graph"))
  unlisted=$(comm -23 <(printf '%s\n' "$build_graph") <(printf '%s\n' "$BUILD_ONLY" | sort -u))
  if [ -n "$unlisted" ]; then
    for n in $unlisted; do
      report "build graph changed -- '$n' is not on the list."
      echo "      ✅ REMEDY: assess it and ADD IT to the list, with the justification."
      echo "      It is the event to review that ADR-0031 declares among its own Negative."
    done
  fi

  # Non-vacuity guard: if the two graphs COINCIDE the filter is not distinguishing
  # anything, and that is the exact condition in which M-3 was fooled (§7.2.3). It does not
  # pass in silence: the check REPORTS it.
  if [ "$shipped_graph" = "$full_graph" ]; then
    report "$crate: shipped graph and full graph COINCIDE -- the filter is not distinguishing anything."
    echo "      It is not 'the list is short': it is 'the query was narrow'."
  fi
done

echo
if [ "$failures" -eq 0 ]; then
  echo "OK -- the two graphs match the two lists."
else
  echo "$failures violations. Read the REMEDY: it is NOT the same for the two graphs."
  exit 1
fi
