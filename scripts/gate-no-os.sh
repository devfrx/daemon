#!/usr/bin/env bash
# Il cancello senza OS -- §7.3.2 della spec del sotto-progetto 1.
#
# SI AGGIUNGE alla allow-list, non la sostituisce. I due falliscono in modo
# complementare: la lista ENUMERA e nomina il colpevole ("X unty <- kernel -> bincode
# -> unty"); il cancello PROVA e coglie una crate GIA' IN LISTA che raggiunge l'OS per
# una via non prevista -- l'unificazione delle feature -- ma dice solo "no" senza dire
# chi. La lista e' la diagnosi, il cancello e' la prova.
#
# BERSAGLIO: x86_64-unknown-none, e non e' un dettaglio. Deve differire dal bersaglio
# reale in UNA SOLA dimensione, l'assenza dell'OS. thumbv7em-none-eabihf ne differisce
# per quattro (arch, puntatore, atomici a 64 bit) ed e' una sorgente di rossi per il
# motivo sbagliato -- gotcha #9 applicato al bersaglio.
#
# ⛔ NON aggiungere --workspace. Il comando nomina le DUE crate vincolate, e non e' una
# comodita': con --workspace il cancello fallisce su `platform` con "can't find crate
# for std", cioe' motivo giusto e crate sbagliata. E' la sonda B3, che non esisteva.
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1

BERSAGLIO=x86_64-unknown-none

echo "== cancello senza OS -- $BERSAGLIO =="

if ! rustup target list --installed | grep -qx "$BERSAGLIO"; then
  echo "  ✗ bersaglio $BERSAGLIO non installato."
  echo "    rustup target add $BERSAGLIO   (o affidati a rust-toolchain.toml)"
  echo "    Senza, la porta sarebbe rossa per il motivo sbagliato -- vincolo 4 della §11."
  exit 1
fi

if cargo build -p kernel -p simulator --target "$BERSAGLIO" 2>&1; then
  echo "  ✓ kernel e simulator compilano senza sistema operativo"
  exit 0
else
  echo "  ✗ kernel o simulator NON compilano per $BERSAGLIO."
  echo "    Il cancello non dice chi l'ha tirata dentro: guarda l'uscita di"
  echo "    scripts/gate-deps.sh, che nomina il rimbalzo."
  exit 1
fi
