#!/usr/bin/env bash
# Controllo di coerenza della documentazione.
# Da eseguire PRIMA DI OGNI COMMIT di documentazione -- e quindi a ogni chiusura di
# sezione, ADR o sotto-progetto, che è quando si committa. La cadenza è quella di
# CLAUDE.md e della §7.5.1 della spec del sotto-progetto 1: una sola, scritta in tre
# posti che devono dire la stessa cosa. Allineata il 2026-08-08: questa riga diceva
# «alla chiusura di ogni sezione, ADR o sotto-progetto», cioè una cadenza più
# grossolana di quella che la porta di qualità le assegna.
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
# Il controllo è PER FILE. Concatenare le spec produrrebbe un falso positivo: ogni
# spec ha legittimamente la propria §0, e un controllo che fallisce per il motivo
# sbagliato è peggio di un controllo assente — insegna a ignorare l'audit.
for f in docs/superpowers/specs/*.md; do
  dup=$(grep -ohE '^#{2,3} [0-9]+(\.[0-9]+)?' "$f" | sort | uniq -d | tr '\n' ' ')
  [ -z "${dup// /}" ] || segnala "sezioni duplicate in $f: $dup"
done

echo "== ogni requisito Q ha un metodo di verifica (V30) =="
mancanti=$(comm -23 \
  <(grep -ohE '^\| Q[0-9]+ \|' docs/superpowers/specs/*.md | grep -oE 'Q[0-9]+' | sort -uV) \
  <(grep -ohE '^\| Q[0-9]+ \|' docs/design/08-strategia-di-test.md | grep -oE 'Q[0-9]+' | sort -uV) |
  tr '\n' ' ')
[ -z "${mancanti// /}" ] || segnala "V30 violato — Q senza metodo di verifica: $mancanti"

spec_sp1=docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md

echo "== catalogo §7.4: ogni controllo ha la contro-sonda (§7.1.1 regola 3) =="
# La §7.1.1 impone che ogni voce del catalogo porti DUE sonde: quella che deve
# scattare e quella che deve restare verde. Era l'unico punto della §7 non
# verificabile a sua volta (§7.7.1), cioe un'intenzione.
# Il controllo e sulla FORMA: verifica che la casella sia piena, non che la
# contro-sonda esista davvero (§8.6.4). Chi scrive «n/a» passa.
# Guardia di non-vacuita: se un delimitatore non c'e, o l'intervallo e vuoto,
# e un FALLIMENTO. Uno script che non trova niente da controllare uscirebbe
# verde -- gotcha #14 applicato allo script stesso.
vuote=$(
  awk '
    /^#### 7\.4\.1/ { apre=1; dentro=1; next }
    /^#### 7\.4\.3/ { chiude=1; dentro=0; next }
    dentro && !/^\|/ { attese=0; next }
    dentro && /^\|/ {
      riga=$0
      gsub(/\\\|/, "", riga)                 # le pipe protette non separano celle
      n=gsub(/\|/, "|", riga) - 1            # celle = separatori - 1
      if (riga ~ /^\|[-:|[:space:]]+\|$/) { attese=n; next }   # riga separatrice
      if (attese == 0) next                                    # intestazione
      righe++
      prima=riga
      sub(/^[[:space:]]*\|[[:space:]]*/, "", prima); sub(/[[:space:]]*\|.*$/, "", prima)
      ultima=riga
      sub(/\|[[:space:]]*$/, "", ultima); sub(/^.*\|/, "", ultima)
      sub(/^[[:space:]]+/, "", ultima); sub(/[[:space:]]+$/, "", ultima)
      if (n < attese)       printf "riga %d (%s): manca la colonna contro-sonda\n", NR, prima
      else if (ultima == "") printf "riga %d (%s): contro-sonda vuota\n", NR, prima
    }
    END {
      if (!apre)    print "delimitatore «#### 7.4.1» non trovato"
      if (!chiude)  print "delimitatore «#### 7.4.3» non trovato"
      if (righe==0) print "nessuna riga di catalogo nell'\''intervallo: il controllo sarebbe vacuo"
    }
  ' "$spec_sp1"
)
[ -z "$vuote" ] || while IFS= read -r r; do segnala "catalogo §7.4 — $r"; done <<<"$vuote"

echo "== §8: ogni V e ogni Q ha uno stato, e i rimandati hanno l'innesco =="
# La mitigazione promessa dalla §0.6 contro «rimandato tende a diventare
# dimenticato». Quattro asserzioni (§8.6.1): completezza e non-duplicazione,
# stato dentro l'insieme chiuso, innesco obbligatorio per «parziale» e
# «rimandato». Lo stato si riconosce da una PAROLA, non da un'emoji: il
# byte-matching su emoji dipende dal locale, e un rosso per il locale e un
# rosso per il motivo sbagliato.
stati=$(
  awk '
    function pulisci(s) {
      sub(/^[[:space:]]+/, "", s); sub(/[[:space:]]+$/, "", s); return s
    }
    /^## 8\. / { apre=1; dentro=1 }
    dentro && /^\|[[:space:]]*[VQ][0-9]+[[:space:]]*\|/ {
      riga=$0; gsub(/\\\|/, "", riga)
      n=split(riga, c, "|")
      id=pulisci(c[2])
      if (visto[id]++) { printf "%s compare piu di una volta\n", id; next }
      if (n < 7) { printf "%s: la riga non ha le cinque colonne\n", id; next }
      stato=pulisci(c[4]); innesco=pulisci(c[6])
      k = (stato ~ /verificato qui/) + (stato ~ /parziale/) \
        + (stato ~ /rimandato/)     + (stato ~ /non controllato/)
      if (k == 0) { printf "%s: stato non ammesso — «%s»\n", id, stato; next }
      if (k > 1)  { printf "%s: stato ambiguo — «%s»\n", id, stato; next }
      if (stato ~ /parziale/ || stato ~ /rimandato/)
        if (innesco == "" || innesco == "—" || innesco == "-")
          printf "%s: «%s» senza innesco\n", id, stato
    }
    END {
      if (!apre) { print "delimitatore «## 8.» non trovato"; exit }
      for (i=1; i<=37; i++) if (!visto["V" i]) printf "manca la riga per V%d\n", i
      for (i=1; i<=24; i++) if (!visto["Q" i]) printf "manca la riga per Q%d\n", i
    }
  ' "$spec_sp1"
)
[ -z "$stati" ] || while IFS= read -r r; do segnala "§8 — $r"; done <<<"$stati"

echo "== compendio §5: una voce per ogni ADR, e nessuna di troppo =="
# Il compendio e l'unica lettura obbligatoria a inizio sessione (CLAUDE.md).
# Conseguenza secca: una decisione che non compare li, per chi legge NON ESISTE.
# Un compendio stantio e peggio di nessun compendio -- mente con autorevolezza,
# e chi lo legge non ha modo di accorgersene perche CREDE di sapere tutto.
# Livello 2 -- controllo esterno: cancellato lo script, la regola sparisce.
# Il livello 1 non e raggiungibile: nessun compilatore legge un .md.
# DUE direzioni (gotcha #24): un ADR senza voce, e una voce senza ADR. La
# seconda coglie il file rinominato o rimosso, che la prima non vede.
# Guardia di non-vacuita: delimitatore mancante o blocco vuoto sono un
# FALLIMENTO. Uno script che non trova niente da controllare uscirebbe verde
# -- gotcha #26.
compendio=docs/COMPENDIO.md
if [ ! -f "$compendio" ]; then
  segnala "manca $compendio, che CLAUDE.md dichiara lettura obbligatoria"
else
  grep -qE '^## 5\. ' "$compendio" || segnala "compendio §5 — delimitatore «## 5. » non trovato"
  grep -qE '^## 6\. ' "$compendio" || segnala "compendio §5 — delimitatore «## 6. » non trovato"
  voci=$(
    awk '/^## 5\. / { dentro=1; next } /^## 6\. / { dentro=0 } dentro' "$compendio" |
      grep -oE '^\*\*[0-9]{4} —' | grep -oE '[0-9]{4}' | sort -u
  )
  n_voci=$(printf '%s\n' "$voci" | grep -cE '^[0-9]{4}$' || true)
  if [ "$n_voci" -eq 0 ]; then
    segnala "compendio §5 — nessuna voce nell'intervallo: il controllo sarebbe vacuo"
  else
    assenti=$(comm -23 \
      <(ls docs/adr/*.md 2>/dev/null | xargs -n1 basename | cut -c1-4 | sort -u) \
      <(printf '%s\n' "$voci") | tr '\n' ' ')
    [ -z "${assenti// /}" ] || segnala "compendio §5 — ADR senza voce: $assenti"
    intrusi=$(comm -13 \
      <(ls docs/adr/*.md 2>/dev/null | xargs -n1 basename | cut -c1-4 | sort -u) \
      <(printf '%s\n' "$voci") | tr '\n' ' ')
    [ -z "${intrusi// /}" ] || segnala "compendio §5 — voce senza ADR corrispondente: $intrusi"
  fi
fi

echo "== conteggi ADR dichiarati nelle prose =="
# I documenti di stato dichiarano quanti ADR esistono. Il numero invecchia in silenzio:
# nessun controllo lo intercettava, e due prose erano stantie.
# Copre esattamente tre forme, e non di più:
#   «N ADR in stato ...»        -> deve valere il numero degli Accepted
#   «N ADR»                     -> deve valere il totale
#   «N decisioni architetturali» -> deve valere il totale
# Limite dichiarato: un numero scritto a parole è invisibile a questa guardia.
adr_tot=$(ls docs/adr/*.md 2>/dev/null | wc -l)
adr_acc=$(grep -l '^- \*\*Status:\*\* Accepted' docs/adr/*.md 2>/dev/null | wc -l)
for f in docs/HANDOFF.md docs/roadmap.md docs/README.md docs/COMPENDIO.md docs/AVVIO-CHAT.md CLAUDE.md; do
  [ -f "$f" ] || continue
  # Gli esempi stanno nei code span: `2 ADR nuovi` è un esempio, non una
  # dichiarazione. Si spogliano prima di confrontare, o il controllo accusa
  # la documentazione di sé stesso — successo davvero.
  while IFS= read -r m; do
    [ -n "$m" ] || continue
    n=${m%% *}
    case "$m" in
      *"in stato"*) atteso=$adr_acc; eti="in stato Accepted" ;;
      *) atteso=$adr_tot; eti="in totale" ;;
    esac
    [ "$n" -eq "$atteso" ] || segnala "$f dichiara $n ADR $eti, sono $atteso"
  done < <(sed 's/`[^`]*`//g' "$f" | grep -oE '[0-9]+ (ADR in stato|ADR|decisioni architetturali)')
done

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
