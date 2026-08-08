#!/usr/bin/env bash
# L'allow-list sul grafo transitivo di kernel e simulator -- ADR-0031, §7.3.1.
#
# DUE GRAFI, DUE ERRORI, DUE RIMEDI OPPOSTI. Non e' completezza: unificarli insegna il
# riflesso "aggiungi alla lista" ANCHE per una violazione di I3, dove aggiungere E' la
# violazione. E un controllo che guarda solo cio' che spedisce lascia passare in silenzio
# proprio l'evento che ADR-0031 dice di rivedere.
#
#   spedita   -> `cargo tree -e normal,no-proc-macro` -> errore "I3 violato"
#                RIMEDIO: togliere la dipendenza. Aggiungerla alla lista NON e' un rimedio.
#   di build  -> il complemento fra `-e no-dev` e la riga sopra
#                RIMEDIO: valutare e aggiungere alla lista, con giustificazione.
#   di sviluppo -> esclusa, e l'esclusione e' PROVATA (vedi la guardia sotto).
#
# ⚠️ `cargo tree` e non `cargo metadata`: il secondo non risolve le feature. Misurato,
# gotcha #23: undici crate segnalate contro le due reali, cioe' 5x di sovra-segnalazione.
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1

fallimenti=0
segnala() { echo "  ✗ $*"; fallimenti=$((fallimenti + 1)); }

# --- Le due liste. Sede unica: la §7.3.1 della spec; questo file la rispecchia. ---
SPEDITE="bincode
kernel
minicbor
simulator
unty"

DI_BUILD="bincode_derive
minicbor-derive
proc-macro2
quote
syn
unicode-ident
virtue"

# ⛔ LA CLASSE DI CARATTERI DEL grep E' UN VINCOLO MISURATO, NON UNA FORMATTAZIONE.
# Con `[a-z0-9_-]` -- come era scritta -- una crate col nome MAIUSCOLO veniva scartata dal
# filtro, quindi non compariva fra gli intrusi e il cancello usciva VERDE. Misurato con
# `Inflector`, che e' una crate reale: nel grafo spedito del kernel, uscita 0. E' un falso
# negativo su I3, cioe' il modo di fallire peggiore per questo controllo. Sonda N5.
# Chi la "semplifica" riapre il punto cieco, e il cancello torna verde senza dirlo.
nomi() { sed 's/^[^a-zA-Z0-9_-]*//' | awk '{print $1}' | grep -E '^[A-Za-z0-9_-]+$' | sort -u; }

for crate in kernel simulator; do
  echo "== $crate: grafo SPEDITO =="
  spedito=$(cargo tree -p "$crate" -e normal,no-proc-macro --prefix none 2>/dev/null | nomi)
  intrusi=$(comm -23 <(printf '%s\n' "$spedito") <(printf '%s\n' "$SPEDITE" | sort -u))
  if [ -n "$intrusi" ]; then
    for i in $intrusi; do
      segnala "I3 violato -- $crate spedisce '$i', che non e' in lista."
      echo "      ⛔ RIMEDIO: TOGLIERE la dipendenza. Aggiungerla alla lista non e' un rimedio."
      echo "      Da dove arriva:"
      cargo tree -p "$crate" -e normal,no-proc-macro -i "$i" 2>/dev/null | sed 's/^/        /'
    done
  fi

  echo "== $crate: grafo DI BUILD =="
  completo=$(cargo tree -p "$crate" -e no-dev --prefix none 2>/dev/null | nomi)
  build=$(comm -13 <(printf '%s\n' "$spedito") <(printf '%s\n' "$completo"))
  nuove=$(comm -23 <(printf '%s\n' "$build") <(printf '%s\n' "$DI_BUILD" | sort -u))
  if [ -n "$nuove" ]; then
    for n in $nuove; do
      segnala "grafo di build cambiato -- '$n' non e' in lista."
      echo "      ✅ RIMEDIO: valutarla e AGGIUNGERLA alla lista, con la giustificazione."
      echo "      E' l'evento da rivedere che ADR-0031 dichiara fra le proprie Negative."
    done
  fi

  # Guardia di non-vacuita': se i due grafi COINCIDONO il filtro non distingue niente,
  # ed e' la condizione esatta in cui M-3 e' stata ingannata (§7.2.3). Non passa in
  # silenzio: il controllo lo SEGNALA.
  if [ "$spedito" = "$completo" ]; then
    segnala "$crate: grafo spedito e grafo completo COINCIDONO -- il filtro non sta distinguendo niente."
    echo "      Non e' 'la lista e' corta': e' 'l'interrogazione era stretta'."
  fi
done

echo
if [ "$fallimenti" -eq 0 ]; then
  echo "OK -- i due grafi corrispondono alle due liste."
else
  echo "$fallimenti violazioni. Leggi il RIMEDIO: NON e' lo stesso per i due grafi."
  exit 1
fi
