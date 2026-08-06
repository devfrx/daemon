#!/usr/bin/env bash
# Controllo di coerenza della documentazione.
# Da eseguire alla chiusura di ogni sezione, ADR o sotto-progetto.
# Uscita 0 = tutto coerente. Uscita 1 = c'è qualcosa da sistemare.
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1

fallimenti=0
segnala() { echo "  ✗ $*"; fallimenti=$((fallimenti + 1)); }

echo "== link interni =="
# I piani sono esclusi di proposito: contengono riferimenti in avanti ad
# artefatti che il piano stesso deve ancora creare.
rotti=$(
  while IFS= read -r f; do
    d=$(dirname "$f")
    grep -o '](\([^)#]*\.md\)[^)]*)' "$f" 2>/dev/null |
      sed 's/](\(.*\))/\1/' | cut -d'#' -f1 | grep -v '^http' |
      while IFS= read -r l; do
        [ -f "$d/$l" ] || echo "$f -> $l"
      done
  done < <(find . -name '*.md' -not -path './.git/*' -not -path './docs/superpowers/plans/*')
)
[ -z "$rotti" ] || while IFS= read -r r; do segnala "link rotto: $r"; done <<<"$rotti"

echo "== ADR: file vs indice =="
n_file=$(ls docs/adr/*.md 2>/dev/null | wc -l)
n_idx=$(grep -cE '^\| \[00' docs/README.md)
[ "$n_file" -eq "$n_idx" ] || segnala "ADR: $n_file file, $n_idx voci d'indice"

echo "== diagrammi: file vs indice =="
d_file=$(ls docs/design/*.md 2>/dev/null | wc -l)
d_idx=$(grep -cE '^\| \[.*\]\(design/' docs/README.md)
[ "$d_file" -eq "$d_idx" ] || segnala "design: $d_file file, $d_idx voci d'indice"

echo "== numerazione delle sezioni: duplicati =="
dup=$(grep -ohE '^#{2,3} [0-9]+(\.[0-9]+)?' docs/superpowers/specs/*.md | sort | uniq -d | tr '\n' ' ')
[ -z "${dup// /}" ] || segnala "sezioni duplicate: $dup"

echo "== ogni requisito Q ha un metodo di verifica (V30) =="
mancanti=$(comm -23 \
  <(grep -ohE '^\| Q[0-9]+ \|' docs/superpowers/specs/*.md | grep -oE 'Q[0-9]+' | sort -uV) \
  <(grep -ohE '^\| Q[0-9]+ \|' docs/design/08-strategia-di-test.md | grep -oE 'Q[0-9]+' | sort -uV) |
  tr '\n' ' ')
[ -z "${mancanti// /}" ] || segnala "V30 violato — Q senza metodo di verifica: $mancanti"

echo "== ADR ancora in Proposed =="
prop=$(grep -l 'Status:\*\* Proposed' docs/adr/*.md 2>/dev/null | tr '\n' ' ')
[ -z "${prop// /}" ] && echo "  (nessuno)" || echo "  in attesa di approvazione: $prop"

echo
if [ "$fallimenti" -eq 0 ]; then
  echo "OK — nessuna incoerenza."
else
  echo "$fallimenti incoerenze da sistemare."
  exit 1
fi
