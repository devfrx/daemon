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

# ⛔ THE CRATES ARE A VARIABLE AND THE EXPECTED COUNT IS DERIVED FROM IT, never written twice.
# The reverse check at the bottom is valid only if EVERY crate was measured, and a literal `2`
# beside a list of two is the shape this repository has already paid for five times (gotcha
# #31): the list grows, the number stays, and the guard silently starts checking a union it
# believes complete.
CRATES="kernel simulator"
expected_crates=$(printf '%s\n' $CRATES | wc -l)
measured=0
shipped_union=""
build_union=""


# ⛔ `--locked` ON EVERY `cargo tree` BELOW, and it is what makes this check trustworthy at all:
# without it cargo RE-RESOLVES, rewrites the tracked Cargo.lock, and this script then measures
# the graph cargo has just invented instead of the one ADR-0031 approved. Measured rather than
# reasoned (finding G-5 of the 2026-08-11 audit): with `minicbor` removed from
# crates/kernel/Cargo.toml, this script came out `OK -- the two graphs match the two lists`,
# exit 0, and Cargo.lock had lost 33 lines. The remedy for a stale lockfile is to refresh it
# OUTSIDE the gate and COMMIT it with the manifest -- see the block in gate.sh.
#
# ⚠️ WHAT THE EXPLICIT FAILURE BRANCH BUYS IS THE DIAGNOSIS, NOT THE RED. Without it a failing
# `cargo tree` leaves BOTH graphs empty, they compare equal, and the non-vacuity guard at the
# bottom of this file already turns the gate red -- but saying "the query was narrow" when the
# truth is "the lockfile is stale". A red whose reason is wrong is what §7.1.1 calls worse than
# no check at all, because it teaches people to stop reading the output.
#
# ⚠️ AND THE ERROR IS SHOWN BY RE-RUNNING, never by folding stderr into the capture: a `cargo
# tree` that prints "Blocking waiting for file lock on package cache" would hand `names` the
# word "Blocking", which passes its character class and would be reported as an INTRUDER on I3
# -- a red for the wrong reason. The second run costs nothing: it happens only on the way out.
for crate in $CRATES; do
  echo "== $crate: SHIPPED graph =="
  if ! shipped_raw=$(cargo tree --locked -p "$crate" -e normal,no-proc-macro --prefix none 2>/dev/null); then
    report "$crate: the SHIPPED graph could NOT be measured -- 'cargo tree --locked' failed."
    echo "      ⛔ Most often a STALE Cargo.lock: a manifest changed and the lockfile did not."
    echo "      Refresh it OUTSIDE the gate (a plain 'cargo build') and COMMIT it with the"
    echo "      manifest. The lockfile is an INPUT of this check, not an effect of it."
    echo "      The error, verbatim:"
    cargo tree --locked -p "$crate" -e normal,no-proc-macro --prefix none 2>&1 >/dev/null | sed 's/^/        /'
    continue
  fi
  shipped_graph=$(printf '%s\n' "$shipped_raw" | names)
  intruders=$(comm -23 <(printf '%s\n' "$shipped_graph") <(printf '%s\n' "$SHIPPED" | sort -u))
  if [ -n "$intruders" ]; then
    for i in $intruders; do
      report "I3 violated -- $crate ships '$i', which is not on the list."
      echo "      ⛔ REMEDY: REMOVE the dependency. Adding it to the list is not a remedy."
      echo "      Where it comes from:"
      cargo tree --locked -p "$crate" -e normal,no-proc-macro -i "$i" 2>/dev/null | sed 's/^/        /'
    done
  fi

  echo "== $crate: BUILD graph =="
  if ! full_raw=$(cargo tree --locked -p "$crate" -e no-dev --prefix none 2>/dev/null); then
    report "$crate: the BUILD graph could NOT be measured -- 'cargo tree --locked' failed."
    echo "      Same cause and same remedy as above: the lockfile is an INPUT of this check."
    echo "      The error, verbatim:"
    cargo tree --locked -p "$crate" -e no-dev --prefix none 2>&1 >/dev/null | sed 's/^/        /'
    continue
  fi
  full_graph=$(printf '%s\n' "$full_raw" | names)
  build_graph=$(comm -13 <(printf '%s\n' "$shipped_graph") <(printf '%s\n' "$full_graph"))
  unlisted=$(comm -23 <(printf '%s\n' "$build_graph") <(printf '%s\n' "$BUILD_ONLY" | sort -u))
  if [ -n "$unlisted" ]; then
    for n in $unlisted; do
      report "build graph changed -- '$n' is not on the list."
      echo "      ✅ REMEDY: assess it and ADD IT to the list, with the justification."
      echo "      It is the event to review that ADR-0031 declares among its own Negative."
    done
  fi

  # ⚠️ ACCUMULATED HERE AND NOT EARLIER, and the position is the guard: both `continue`
  # branches above jump over this line, so a crate whose `cargo tree` failed contributes
  # NOTHING to the union and `measured` does not count it. The reverse check below refuses to
  # run on an incomplete union rather than reporting phantoms that are only missing.
  shipped_union="$shipped_union
$shipped_graph"
  build_union="$build_union
$build_graph"
  measured=$((measured + 1))

  # Non-vacuity guard: if the two graphs COINCIDE the filter is not distinguishing
  # anything, and that is the exact condition in which M-3 was fooled (§7.2.3). It does not
  # pass in silence: the check REPORTS it.
  if [ "$shipped_graph" = "$full_graph" ]; then
    report "$crate: shipped graph and full graph COINCIDE -- the filter is not distinguishing anything."
    echo "      It is not 'the list is short': it is 'the query was narrow'."
  fi
done

# ⛔ THE SECOND DIRECTION, ADDED 2026-08-27 -- finding AUD-027. The two loops above report what
# is in a GRAPH and not on a LIST. NOTHING reported what is on a LIST and in no graph.
#
# A PHANTOM entry -- added by mistake, or left behind when the dependency was dropped from the
# manifest -- is a permission granted in advance. The day that crate really enters the shipped
# graph, by feature unification or on the bounce, the loop above finds it ON THE LIST and says
# nothing: a false negative on I3, which the header of this file calls the worst way to fail for
# this check. It also defeats what ADR-0031 asks of the gate -- that adding an entry be "a
# deliberate and reviewable act". An entry nobody can watch arrive is neither.
#
# MEASURED, not reasoned: with `openssl` added as the first line of SHIPPED, this script came
# out `OK -- the two graphs match the two lists`, exit 0.
#
# ⚠️ ON THE UNION AND NOT PER CRATE, and that is the point rather than a shortcut: `simulator`
# is in the shipped graph of `simulator` and NOT in that of `kernel`, so a per-crate reverse
# check would report it as a phantom while auditing `kernel` -- a red for the wrong reason.
# Measured the same day: the union of the two shipped graphs is EXACTLY the five names of
# SHIPPED, and the union of the two complements is EXACTLY the seven of BUILD_ONLY.
#
# ⚠️ SKIPPED WHEN A MEASUREMENT FAILED, AND THE SKIP IS ANNOUNCED. A crate whose `cargo tree`
# failed is absent from the union, and every list entry only that crate reaches would then look
# like a phantom. The stale lockfile is already reported above with its own remedy; reporting it
# a second time under the wrong name is the failure §7.1.1 calls worse than no check at all.
echo "== the two lists: entries no graph reaches =="
if [ "$measured" -ne "$expected_crates" ]; then
  echo "  SKIPPED -- $measured of $expected_crates graphs measured. The union is incomplete,"
  echo "  and a phantom reported from an incomplete union would name the wrong cause."
else
  unione() { printf '%s\n' "$1" | grep -v '^$' | sort -u; }
  for pair in "SHIPPED:$shipped_union" "BUILD_ONLY:$build_union"; do
    lista_nome=${pair%%:*}
    unione_val=${pair#*:}
    eval "lista_val=\$$lista_nome"
    fantasmi=$(comm -13 <(unione "$unione_val") <(unione "$lista_val"))
    if [ -n "$fantasmi" ]; then
      for f in $fantasmi; do
        report "phantom entry -- '$f' is on $lista_nome and in NO graph."
        echo "      ⛔ REMEDY: REMOVE the line. An entry that matches nothing is a permission"
        echo "      granted in advance: the day that crate arrives, the check above stays silent."
      done
    fi
  done
  [ "$failures" -eq 0 ] && echo "  ✓ no phantom entry: every line of both lists is reached by a graph"
fi

echo
if [ "$failures" -eq 0 ]; then
  echo "OK -- the two graphs match the two lists, and both directions were checked."
else
  echo "$failures violations. Read the REMEDY: it is NOT the same for the two graphs."
  exit 1
fi
