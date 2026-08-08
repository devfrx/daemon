#!/usr/bin/env bash
# Gli attributi delle crate vincolate, dichiarati DAVVERO -- §7.4.1 blocco A, vincolo 2 §11.
#
# LA MISURA CHE LO GIUSTIFICA, e non e' un'ipotesi. Senza questo controllo si toglie
# '#![forbid(unsafe_code)]' da crates/kernel/src/lib.rs, si scrive un 'unsafe' VERO nel
# kernel, e la porta resta VERDE su tutti e cinque i controlli precedenti. Misurato.
#
# ⛔ I QUATTRO CASI DI tests/compile_fail/ NON COPRONO IL BUCO, e sembra di si'. Ognuno
# RIDICHIARA i propri attributi e nessuno nomina mai 'kernel::': provano che il MECCANISMO
# morde dove e' dichiarato, non che sia dichiarato QUI. La Definizione di "fatto" del
# traguardo chiede la seconda cosa e verificava la prima.
#
# ⛔ 'forbid' E NON 'deny', ed e' il punto della regola, non una preferenza: 'deny' e'
# scavalcabile da un '#[allow(unsafe_code)]' locale, 'forbid' no -- E0453, ed e' il caso
# allow_unsafe_scavalca.rs. Un controllo che cercasse la sola parola 'unsafe_code'
# uscirebbe VERDE su '#![deny(unsafe_code)]': e' la sonda 3, e distingue questo controllo
# da quello ingenuo.
#
# ⛔ LE CRATE SONO NOMINATE, NON SCOPERTE -- esattamente come gate-no-os.sh nomina
# '-p kernel -p simulator'. 'platform', 'secrets' e 'daemon' NON si controllano: platform
# usa 'std' e 'unsafe' DI PROPOSITO, come contro-sonde dei divieti del kernel. Un controllo
# che scattasse anche li' sarebbe rosso per il motivo sbagliato -- gotcha #24.
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1

fallimenti=0
segnala() { echo "  ✗ $*"; fallimenti=$((fallimenti + 1)); }

# Sede unica dei file vincolati: una crate si aggiunge QUI, non altrove.
VINCOLATI="crates/kernel/src/lib.rs
crates/simulator/src/lib.rs"

# ⚠️ ANCORATE A INIZIO RIGA, e non e' formattazione. Senza '^[[:space:]]*' un attributo
# COMMENTATO -- '// #![forbid(unsafe_code)]' -- soddisferebbe il controllo: e' il falso
# negativo peggiore, perche' la riga sembra esserci a chi legge il diff. Con l'ancora il
# '//' precede e la riga non combacia. COSTO DICHIARATO: un attributo sepolto in un
# commento di blocco /* ... */ sfugge ancora. Chiuderlo vorrebbe dire un parser, e il
# rimedio sarebbe piu' fragile del buco.
#
# ⚠️ La lista di 'forbid' e' PERMISSIVA sul resto: '#![forbid(unsafe_code, missing_docs)]'
# deve passare. Pretendere la riga esatta renderebbe rosso un ampliamento legittimo del
# divieto -- gotcha #9 applicato al controllo.
RE_NO_STD='^[[:space:]]*#!\[[[:space:]]*no_std[[:space:]]*\]'
RE_FORBID='^[[:space:]]*#!\[[[:space:]]*forbid[[:space:]]*\([^)]*\bunsafe_code\b[^)]*\)[[:space:]]*\]'
RE_DENY='^[[:space:]]*#!\[[[:space:]]*deny[[:space:]]*\([^)]*\bunsafe_code\b[^)]*\)[[:space:]]*\]'
RE_ALLOC='^[[:space:]]*extern[[:space:]]+crate[[:space:]]+alloc[[:space:]]*;'

esaminati=0
for f in $VINCOLATI; do
  echo "== $f =="

  # Guardia di non-vacuita': il file che NON C'E' e' un fallimento, non "niente da
  # controllare". Senza, basta rinominare o spostare una crate perche' il controllo smetta
  # di controllare USCENDO VERDE -- gotcha #26, gia' incontrato due volte in questo traguardo.
  if [ ! -f "$f" ]; then
    segnala "$f NON ESISTE. Non e' 'niente da controllare': e' il controllo che non controlla."
    continue
  fi
  esaminati=$((esaminati + 1))

  grep -qE "$RE_NO_STD" "$f" || segnala "$f non dichiara '#![no_std]'."
  grep -qE "$RE_ALLOC"  "$f" || segnala "$f non dichiara 'extern crate alloc;'."

  if grep -qE "$RE_DENY" "$f"; then
    segnala "$f dichiara 'deny(unsafe_code)' dove serve 'forbid'."
    echo "      ⛔ RIMEDIO: 'forbid'. 'deny' e' scavalcabile da un '#[allow(unsafe_code)]'"
    echo "      locale e il divieto torna a essere una preferenza -- vincolo 2 della §11."
  fi

  if ! grep -qE "$RE_FORBID" "$f"; then
    segnala "$f non dichiara '#![forbid(unsafe_code)]'."
    echo "      ⛔ Senza, un 'unsafe' nel kernel compila e NESSUN altro controllo lo vede:"
    echo "      i casi compile_fail ridichiarano i propri attributi e non guardano qui."
  fi
done

# Seconda guardia di non-vacuita', sullo stesso gotcha #26 ma un piano sopra: se la lista
# qui sopra venisse svuotata, il ciclo non girerebbe e lo script uscirebbe 0 IN SILENZIO.
if [ "$esaminati" -eq 0 ]; then
  segnala "nessun file esaminato: la lista dei vincolati e' vuota, e un controllo che non legge niente non prova niente."
fi

echo
if [ "$fallimenti" -eq 0 ]; then
  echo "OK -- kernel e simulator dichiarano i tre attributi, e nessuno usa 'deny'."
else
  echo "$fallimenti violazioni. Un attributo mancante non e' stile: e' l'invariante che sparisce senza lasciare un rosso altrove."
  exit 1
fi
