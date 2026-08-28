#!/usr/bin/env bash
# Consistency check of the documentation.
# To be run BEFORE EVERY DOCUMENTATION COMMIT -- and therefore at every closure of a
# section, ADR or sub-project, which is when one commits. The cadence is the one in
# CLAUDE.md and in §7.5.1 of the sub-project 1 spec: a single one, written in three
# places that must say the same thing. Aligned on 2026-08-08: this line used to say
# "at the closure of every section, ADR or sub-project", that is a cadence coarser
# than the one the quality gate assigns to it.
# Exit 0 = everything consistent. Exit 1 = there is something to fix.
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1

failures=0
report() { echo "  ✗ $*"; failures=$((failures + 1)); }

echo "== internal links =="
# The plans are excluded on purpose: they contain forward references to
# artefacts that the plan itself has yet to create.
#
# ⛔ AND SO IS WHATEVER GIT IGNORES, 2026-08-24. This gate was reading .md files that are not
# in the repository at all, so its verdict depended on the WORKING DIRECTORY instead of on
# what gets delivered -- red today because of three links inside '.superpowers/', and it could
# just as well be green for a file that simply is not on disk that day. Excluding that one
# directory by name would leave '/scratch/' and '/tmp/'
# open, and CLAUDE.md sends every measurement into a scratchpad: the rule has to be the
# general one.
# ⚠️ NOT 'git ls-files': the gate runs BEFORE the commit, so a document that nobody has
# 'git add'-ed yet must still be checked in the run that counts. Untracked and ignored are
# different things, and only the second is dropped here.
# ⚠️ Asked ONCE, and it FAILS OPEN: with no work tree 'git check-ignore' writes nothing, the
# set stays empty and every file is scanned, which is the behaviour of before this change. A
# filter that failed closed would scan nothing and exit green -- gotcha #26.
mds=$(find . -name '*.md' -not -path './.git/*' -not -path './docs/superpowers/plans/*')
ignored=$'\n'$(printf '%s\n' "$mds" | git check-ignore --stdin 2>/dev/null)$'\n'
broken=$(
  while IFS= read -r f; do
    case "$ignored" in *$'\n'"$f"$'\n'*) continue ;; esac
    d=$(dirname "$f")
    grep -o '](\([^)#]*\.md\)[^)]*)' "$f" 2>/dev/null |
      sed 's/](\(.*\))/\1/' | cut -d'#' -f1 | grep -v '^http' |
      while IFS= read -r l; do
        [ -f "$d/$l" ] || echo "$f -> $l"
      done
  done <<<"$mds"
)
[ -z "$broken" ] || while IFS= read -r r; do report "broken link: $r"; done <<<"$broken"

echo "== ADR: files vs index =="
n_file=$(ls docs/adr/*.md 2>/dev/null | wc -l)
n_idx=$(grep -cE '^\| \[00' docs/README.md)
[ "$n_file" -eq "$n_idx" ] || report "ADR: $n_file files, $n_idx index entries"

echo "== diagrams: files vs index =="
d_file=$(ls docs/design/*.md 2>/dev/null | wc -l)
d_idx=$(grep -cE '^\| \[.*\]\(design/' docs/README.md)
[ "$d_file" -eq "$d_idx" ] || report "design: $d_file files, $d_idx index entries"

echo "== section numbering: duplicates =="
# The check is PER FILE. Concatenating the specs would produce a false positive: every
# spec legitimately has its own §0, and a check that fails for the wrong reason is worse
# than no check at all -- it teaches people to ignore the audit.
#
# ⛔ NON-VACUITY GUARD, added 2026-08-11 -- gotcha #26 at the level ABOVE the delimiter.
# 'nullglob' is OFF here (measured), so a renamed or moved directory leaves the pattern
# UNEXPANDED: the loop below runs once on a path that does not exist, grep fails to stderr,
# 'dup' stays empty and NOTHING is reported. The same hole covered the V30 check right
# after. Measured by renaming the directory: both checks produced zero '✗' while all 24 Q
# lost their verification method. A guard on the delimiter does not help when the FILE SET
# is what went missing.
spec_count=$(ls docs/superpowers/specs/*.md 2>/dev/null | wc -l)
[ "$spec_count" -gt 0 ] ||
  report "no spec found under docs/superpowers/specs/: the duplicate-section and V30 checks would both pass on nothing."

# ⛔ THE PATTERN WAS WRONG IN TWO WAYS AT ONCE, both corrected 2026-08-27 -- finding AUD-025.
# It read '^#{2,3} [0-9]+(\.[0-9]+)?' and neither half of that survived measurement.
#
# 1. `#{2,3}` IS BLIND TO `####`, which is where most of the numbering lives. Measured on the
#    spec this check governs: 88 numbered headings at levels two and three, and 96 at level
#    four. The check looked at fewer than HALF of them, and the blind half is exactly where
#    §7.4.1, §7.4.3 and §8.6.1 live -- the anchors the two awk passes of THIS file use as
#    delimiters, and the targets every cross-document reference points at. A duplicated §7.4.1
#    is a duplicated delimiter for the checks below and a reference that resolves to two
#    places. Counter-probe run on 2026-08-27:
#      printf '#### 7.4.1 a\n#### 7.4.1 b\n' | grep -cE '^#{2,3} [0-9]+(\.[0-9]+)?'   -> 0
#    Two identical headings, ZERO rows handed to `uniq -d`. It is gotcha #26 moved off the
#    delimiter and onto the PATTERN: the loop runs, the file is there, `dup` is empty by
#    construction.
#
# 2. `(\.[0-9]+)?` TRUNCATES AT THE SECOND GROUP, which is a FALSE RED waiting for a file to
#    number three levels deep under `###`. Measured the same day:
#      printf '### 7.4.1 uno\n### 7.4.2 due\n' | grep -ohE '^#{2,3} [0-9]+(\.[0-9]+)?' | sort | uniq -d
#      -> '### 7.4'
#    Two DIFFERENT sections reported as one duplicate. ⚠️ It does not fire today -- no `##` or
#    `###` heading in any spec carries three groups (measured: zero in all four files) -- so
#    this half is a landmine defused, not a red repaired. `*` instead of `?` reads the whole
#    number, which is the only thing that can be compared for equality.
#
# ✅ WITH THE NEW PATTERN THE CHECK IS GREEN ON TODAY'S FILES: no real duplicate appears at any
# level, measured before the change rather than discovered by it.
for f in docs/superpowers/specs/*.md; do
  dup=$(grep -ohE '^#{2,6} [0-9]+(\.[0-9]+)*' "$f" | sort | uniq -d | tr '\n' ' ')
  [ -z "${dup// /}" ] || report "duplicate sections in $f: $dup"
done

echo "== every Q requirement has a verification method (V30) =="
# ⛔ 'sort -u' AND NOT 'sort -uV', corrected 2026-08-11. 'comm' compares by COLLATION and
# says so: version sort puts Q9 before Q10, collation puts Q10 before Q9, and 'comm' walks
# two lists it believes sorted. The defect is invisible while the two sides agree -- today
# they do, so 'comm' never meets an unpairable line and does not even print its warning --
# and it lies EXACTLY when the check has something true to say. Measured on Q1..Q24: with
# Q9 alone missing a method, '-uV' reported fifteen names (Q9 through Q24) instead of one.
# A red nobody can read teaches people to ignore the audit, which is the thing the comment
# above declares to be worse than no check at all.
missing=$(comm -23 \
  <(grep -ohE '^\| Q[0-9]+ \|' docs/superpowers/specs/*.md | grep -oE 'Q[0-9]+' | sort -u) \
  <(grep -ohE '^\| Q[0-9]+ \|' docs/design/08-strategia-di-test.md | grep -oE 'Q[0-9]+' | sort -u) |
  tr '\n' ' ')
[ -z "${missing// /}" ] || report "V30 violated — Q with no verification method: $missing"

spec_sp1=docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md

# ⛔ EXISTENCE GUARD, added 2026-08-11, AND IT IS THE ONE THE OTHER GUARDS CANNOT BE.
# The six assertions of §8.6.1 live in the two awk passes below, and every one of their
# non-vacuity guards -- «delimiter not found», «rows==0», «defends==0» -- sits in an END
# block. Measured: when the input file cannot be opened, awk emits a FATAL and END NEVER
# RUNS. So '$catalogue' and '$states' stay empty, '[ -z ... ]' is true, nothing is
# reported, and check-docs.sh exits 0 -- GATE GREEN with all six assertions dead in
# silence. That is §8.6.2's own enemy one level up: the guards protect against a RENUMBERED
# heading, not against a RENAMED file. A guard inside END cannot, by construction, defend
# against the input that is missing.
[ -f "$spec_sp1" ] ||
  report "$spec_sp1 does not exist: the six assertions of §8.6.1 would not run at all."

echo "== catalogue §7.4: every check defends something (rule 1) and has its counter-probe (rule 3) =="
# TWO assertions on the same table, in a single pass: the delimiters are written ONCE, or
# they become two places to keep aligned and the first one that stops lies in silence.
#
# RULE 3 -- every entry carries TWO probes: the one that must fire and the one that must
# stay green. It was the only point of §7 not verifiable in its turn (§7.7.1), that is,
# an intention.
# RULE 1 -- the «Difende» cell names a V, an I or a Q (branch 1a), or else a catalogue
# entry whose validity the row upholds (branch 1b, decided on 2026-08-08 closing §7.1.1).
# Without it, it is a habit and not a check: it is the same rule with which §7.4.3 throws
# clippy out of the gate.
#
# The «Difende» column is NOT always the first one: in block B of the tokens it is the
# third. It is read from the HEADER instead of by position -- positional reading is trap
# 3 of this script, and here there was no reason to inherit it.
# The «§» character does not enter the comparison: it is multibyte, and byte-matching
# depends on the locale. Same reason why the §8 states are recognised by a word instead
# of by an emoji.
# Both are checks on the FORM: they prove that the cell is filled and that it NAMES
# something of the right shape, not that the counter-probe really exists nor that the
# attribution is true (§8.6.4). Whoever writes «§7.4.1» next to a lint passes.
# Non-vacuity guard: a missing delimiter, an empty range or no «Difende» cell read are a
# FAILURE. A script that finds nothing to check would exit green -- gotcha #26.
catalogue=$(
  awk '
    function trim(s) {
      sub(/^[[:space:]]+/, "", s); sub(/[[:space:]]+$/, "", s); return s
    }
    /^#### 7\.4\.1/ { opens=1; inside=1; next }
    /^#### 7\.4\.3/ { closes=1; inside=0; next }
    inside && !/^\|/ { expected=0; idx=0; next }
    inside && /^\|/ {
      row=$0
      gsub(/\\\|/, "", row)                  # protected pipes do not separate cells
      n=gsub(/\|/, "|", row) - 1             # cells = separators - 1
      if (row ~ /^\|[-:|[:space:]]+\|$/) {                     # separator row
        expected=n
        if (idx == 0) printf "row %d: catalogue table with no Difende column\n", NR
        next
      }
      if (expected == 0) {                                     # header row
        idx=0
        m=split(row, h, "|")
        for (i=2; i<m; i++) if (h[i] ~ /Difende/) idx=i-1
        next
      }
      rows++
      first=row
      sub(/^[[:space:]]*\|[[:space:]]*/, "", first); sub(/[[:space:]]*\|.*$/, "", first)
      last=row
      sub(/\|[[:space:]]*$/, "", last); sub(/^.*\|/, "", last)
      last=trim(last)
      if (n < expected)   printf "row %d (%s): counter-probe column missing\n", NR, first
      else if (last == "") printf "row %d (%s): empty counter-probe\n", NR, first
      if (idx > 0) {
        split(row, c, "|")
        def=trim(c[idx+1])
        defends++
        if (def !~ /(^|[^A-Za-z])[IVQ][0-9]/ && def !~ /7\.4\./)
          printf "row %d: «Difende» = «%s» names neither a V, an I or a Q, nor a catalogue entry (rule 1)\n", NR, def
      }
    }
    END {
      if (!opens)     print "delimiter «#### 7.4.1» not found"
      if (!closes)    print "delimiter «#### 7.4.3» not found"
      if (rows==0)    print "no catalogue row in the range: the check would be vacuous"
      if (defends==0) print "no Difende cell read: rule 1 would check nothing"
    }
  ' "$spec_sp1"
)
[ -z "$catalogue" ] || while IFS= read -r r; do report "catalogue §7.4 — $r"; done <<<"$catalogue"

echo "== §8: every V and every Q has a state, and the deferred ones have their trigger =="
# The mitigation promised by §0.6 against "deferred tends to become forgotten". Four
# assertions (§8.6.1): completeness and non-duplication, state within the closed set,
# mandatory trigger for «parziale» and «rimandato». The state is recognised by a WORD,
# not by an emoji: byte-matching on emoji depends on the locale, and a red caused by the
# locale is a red for the wrong reason.
states=$(
  awk '
    function trim(s) {
      sub(/^[[:space:]]+/, "", s); sub(/[[:space:]]+$/, "", s); return s
    }
    /^## 8\. / { opens=1; inside=1 }
    inside && /^\|[[:space:]]*[VQ][0-9]+[[:space:]]*\|/ {
      row=$0; gsub(/\\\|/, "", row)
      n=split(row, c, "|")
      id=trim(c[2])
      if (seen[id]++) { printf "%s appears more than once\n", id; next }
      if (n < 7) { printf "%s: the row does not have the five columns\n", id; next }
      state=trim(c[4]); trigger=trim(c[6])
      k = (state ~ /verificato qui/) + (state ~ /parziale/) \
        + (state ~ /rimandato/)     + (state ~ /non controllato/)
      if (k == 0) { printf "%s: state not allowed — «%s»\n", id, state; next }
      if (k > 1)  { printf "%s: ambiguous state — «%s»\n", id, state; next }
      if (state ~ /parziale/ || state ~ /rimandato/)
        if (trigger == "" || trigger == "—" || trigger == "-")
          printf "%s: «%s» with no trigger\n", id, state
    }
    END {
      if (!opens) { print "delimiter «## 8.» not found"; exit }
      for (i=1; i<=37; i++) if (!seen["V" i]) printf "missing row for V%d\n", i
      for (i=1; i<=24; i++) if (!seen["Q" i]) printf "missing row for Q%d\n", i
    }
  ' "$spec_sp1"
)
[ -z "$states" ] || while IFS= read -r r; do report "§8 — $r"; done <<<"$states"

echo "== compendium §5: one entry per ADR, and none too many =="
# The compendium is the only mandatory reading at the start of a session (CLAUDE.md).
# Blunt consequence: a decision that does not appear there DOES NOT EXIST for the reader.
# A stale compendium is worse than no compendium -- it lies with authority, and whoever
# reads it has no way of noticing because they BELIEVE they know everything.
# Level 2 -- external check: delete the script and the rule disappears.
# Level 1 is not reachable: no compiler reads a .md.
# TWO directions (gotcha #24): an ADR with no entry, and an entry with no ADR. The second
# catches the renamed or removed file, which the first does not see.
# Non-vacuity guard: a missing delimiter or an empty block are a FAILURE. A script that
# finds nothing to check would exit green -- gotcha #26.
compendium=docs/COMPENDIO.md
if [ ! -f "$compendium" ]; then
  report "$compendium is missing, and CLAUDE.md declares it mandatory reading"
else
  grep -qE '^## 5\. ' "$compendium" || report "compendium §5 — delimiter «## 5. » not found"
  grep -qE '^## 6\. ' "$compendium" || report "compendium §5 — delimiter «## 6. » not found"
  entries=$(
    awk '/^## 5\. / { inside=1; next } /^## 6\. / { inside=0 } inside' "$compendium" |
      grep -oE '^\*\*[0-9]{4} —' | grep -oE '[0-9]{4}' | sort -u
  )
  n_entries=$(printf '%s\n' "$entries" | grep -cE '^[0-9]{4}$' || true)
  if [ "$n_entries" -eq 0 ]; then
    report "compendium §5 — no entry in the range: the check would be vacuous"
  else
    absent=$(comm -23 \
      <(ls docs/adr/*.md 2>/dev/null | xargs -n1 basename | cut -c1-4 | sort -u) \
      <(printf '%s\n' "$entries") | tr '\n' ' ')
    [ -z "${absent// /}" ] || report "compendium §5 — ADR with no entry: $absent"
    strays=$(comm -13 \
      <(ls docs/adr/*.md 2>/dev/null | xargs -n1 basename | cut -c1-4 | sort -u) \
      <(printf '%s\n' "$entries") | tr '\n' ' ')
    [ -z "${strays// /}" ] || report "compendium §5 — entry with no matching ADR: $strays"
  fi
fi

echo "== ADR counts declared in the prose =="
# The status documents declare how many ADR exist. The number ages in silence: no check
# intercepted it, and two pieces of prose were stale.
# It covers exactly three forms, and no more:
#   «N ADR in stato ...»         -> must equal the number of Accepted ones
#   «N ADR»                      -> must equal the total
#   «N decisioni architetturali» -> must equal the total
# Declared limit: a number spelled out in words is invisible to this guard.
adr_tot=$(ls docs/adr/*.md 2>/dev/null | wc -l)
adr_acc=$(grep -l '^- \*\*Status:\*\* Accepted' docs/adr/*.md 2>/dev/null | wc -l)
for f in docs/HANDOFF.md docs/roadmap.md docs/README.md docs/COMPENDIO.md docs/AVVIO-CHAT.md CLAUDE.md; do
  [ -f "$f" ] || continue
  # The examples live inside code spans: `2 ADR nuovi` is an example, not a
  # declaration. They are stripped before comparing, or the check accuses the
  # documentation of itself -- a success indeed.
  while IFS= read -r m; do
    [ -n "$m" ] || continue
    n=${m%% *}
    case "$m" in
      *"in stato"*) expected=$adr_acc; label="in Accepted status" ;;
      *) expected=$adr_tot; label="in total" ;;
    esac
    [ "$n" -eq "$expected" ] || report "$f declares $n ADR $label, they are $expected"
  done < <(sed 's/`[^`]*`//g' "$f" | grep -oE '[0-9]+ (ADR in stato|ADR|decisioni architetturali)')
done

echo "== ADR still in Proposed =="
prop=$(grep -l 'Status:\*\* Proposed' docs/adr/*.md 2>/dev/null | tr '\n' ' ')
[ -z "${prop// /}" ] && echo "  (none)" || echo "  awaiting approval: $prop"
echo "== compendium size ceiling =="
# ⛔ THE CEILING EXISTS BECAUSE THE RULE ALONE DID NOT HOLD. On 2026-08-28 the compendium
# was 623516 bytes and 92% of it was the history of corrected numbers: every fix appended a
# paragraph explaining why the old number was wrong, to the one file every session must read
# in full. The rule that forbids it is in CLAUDE.md; this is what makes it checkable.
# «A principle that cannot be checked is an intention» -- CLAUDE.md.
#
# ⚠️ WHY 220 KB AND NOT 80. The design of 2026-08-28 proposed 80 KB, aiming at a ~65 KB
# compendium. That figure assumed the OPEN entries of §6 would be consolidated into a single
# table; the owner chose the conservative route instead -- the blocks naming an open entry
# stay in §6 WORD FOR WORD, because summarising an owner decision can lose one in silence.
# That choice costs ~45000 tokens of §6 and puts the file at 202372 bytes. The ceiling is
# set on what was MEASURED, with about 11% of headroom, and the design carries the dated
# recall. ⛔ WHEN THOSE ENTRIES ARE CONSOLIDATED, THIS NUMBER COMES DOWN: it is a ceiling
# on the file as it is, not a licence to grow into it.
#
# Non-vacuity: a missing file is a FAILURE, not a silent pass -- gotcha #26.
ceiling=225280
if [ ! -f "$compendium" ]; then
  report "$compendium is missing: the size ceiling would be vacuous"
else
  size=$(wc -c < "$compendium")
  if [ "$size" -gt "$ceiling" ]; then
    report "$compendium is $size bytes, over the $ceiling ceiling -- see docs/superpowers/specs/2026-08-28-sfoltimento-compendio-design.md"
  fi
fi


echo
if [ "$failures" -eq 0 ]; then
  echo "OK — no inconsistencies."
else
  echo "$failures inconsistencies to fix."
  exit 1
fi
