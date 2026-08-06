# Risultati degli spike

Data di esecuzione: _(da compilare)_

Criteri e soglie: [PROTOCOLLO.md](PROTOCOLLO.md) — congelato al primo commit di
codice di spike.

## SP-6 — Confine dei dati non fidati, e confini statici del kernel

| Criterio | Rust | Go | TypeScript |
|---|---|---|---|
| T1 non compila | | | |
| T2 percorso unico | | | |
| T3 ereditarietà | | | |
| T4 aggiramento | | | |
| T5 rilevabile globalmente | | | |
| T6 importazione vietata, provata in negativo | | | |

## SP-5 — Iniettabilità e riproducibilità

| Criterio | Rust | Go | TypeScript |
|---|---|---|---|
| C1 stesso seed → stessa traccia | | | |
| C2 seed diversi → tracce diverse | | | |
| C3 tempo virtuale | | | |
| C4 guasto riproducibile | | | |
| C5 nessun orologio/RNG globale | | | |
| C6 concorrenza nativa ordinabile | | | |
| C7 I/O iniettabile, crash riproducibile | | | |

## Osservazioni registrate — non criteri

| # | Rust | Go | TypeScript |
|---|---|---|---|
| O1 motore di persistenza conforme a §10.6 | | | |
| O2 daemon a vita lunga, istanza singola | | | |

## Versioni degli strumenti

| Candidato | Comando | Output |
|---|---|---|
| Rust | `rustc --version` | |
| Go | `go version` | |
| TypeScript | `npx tsc --version` | |

## Seed usati

Un risultato senza seed non è valido.

| Criterio | Candidato | Seed | Note |
|---|---|---|---|
| | | | |

## Evidenze

Una riga per criterio e candidato: comando eseguito, output osservato, e **le
divergenze** rispetto a ciò che ci si aspettava. Una divergenza non registrata è un
risultato perso.

| Criterio | Candidato | Comando | Output osservato | Divergenza dall'attesa |
|---|---|---|---|---|
| | | | | |
