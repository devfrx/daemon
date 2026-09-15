#!/usr/bin/env bash
# The checks the eleventh closure prescribes after EVERY write on the plan, plus check-docs.
# Usage: bash check_after_write.sh   (from anywhere; cds into the repo)
set -u
cd /c/Users/zagor/Desktop/harness || exit 1
F=docs/superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md
echo "== broken tables (must print nothing) =="
awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' "$F"
echo "== CR count (must be 0) =="
tr -cd '\r' < "$F" | wc -c
echo "== eol =="
git ls-files --eol "$F"
echo "== last ## headers =="
grep -n '^## ' "$F" | tail -4
echo "== counts: working tree vs HEAD =="
for pat in '^### P-' '^| \*\*D[0-9]' '^## Compito'; do
  w=$(grep -c "$pat" "$F"); h=$(git show "HEAD:$F" | grep -c "$pat")
  printf '%-16s tree=%s head=%s\n' "$pat" "$w" "$h"
done
echo "== position rows (must equal Compito count) =="
awk '/^## ▶️ A che punto/{s=1} s&&/^### /{s=0} s&&/^\| \*\*[0-9]+\*\* \|/{c++} END{print c+0}' "$F"
echo "== errata rows (0 until execution) =="
awk '/^## ⚠️ L.errata di questo piano/{s=1; next} s&&/^## /{s=0} s&&/^\| \*\*E[0-9]/{c++} END{print c+0}' "$F"
echo "== declared placeholder (one line, task 12) =="
grep -n '<the version' "$F"
echo "== node -e ellipsis inside tasks (0) / empty its (0) =="
awk '/^## Come si riprende/{exit} /node -e "…"/{c++} END{print c+0}' "$F"
grep -cE '^\s*(it|describe)\([^)]*\(\) => \{\}\)' "$F"
echo "== check-docs =="
bash scripts/check-docs.sh 2>&1 | tail -2
