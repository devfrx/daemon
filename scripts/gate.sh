#!/usr/bin/env bash
# La porta di qualita', livello 2 -- §7.5.1 della spec del sotto-progetto 1.
#
# CADENZA: a ogni commit. Il livello 1 NON e' qui, e non e' una dimenticanza: le voci di
# livello 1 non "girano" mai -- SONO il compilatore. Se il codice compila, quelle regole
# valgono, e non esiste un modo di saltarle o di rimandarle a stasera.
#
# ⛔ Un rosso di questa porta significa sempre "invariante violata", mai "stile
# discutibile". `clippy` gira come igiene del codice ma NON ha voce qui: nessun V dipende
# da lui, e la regola 1 del criterio di ammissione (§7.1.1) dice che allora non entra.
# Il livello 3 del catalogo e' VUOTO, ed e' una decisione (§7.4.3).
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1

fallimenti=0
esegui() {
  echo
  echo "######## $1"
  shift
  if "$@"; then :; else fallimenti=$((fallimenti + 1)); fi
}

esegui "compilazione del workspace"        cargo build --workspace
esegui "test a esempi e compile-fail"      cargo test --workspace
esegui "cancello senza OS"                 bash scripts/gate-no-os.sh
esegui "allow-list sui due grafi"          bash scripts/gate-deps.sh
esegui "coerenza della documentazione"     bash scripts/check-docs.sh

echo
if [ "$fallimenti" -eq 0 ]; then
  echo "PORTA VERDE."
else
  echo "PORTA ROSSA -- $fallimenti controlli falliti."
  exit 1
fi
