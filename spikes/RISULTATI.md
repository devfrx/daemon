# Risultati degli spike

Data di esecuzione: _(da compilare)_

## SP-6 — Confine dei dati non fidati

| Criterio | Rust | Go | TypeScript |
|---|---|---|---|
| T1 non compila | | | |
| T2 percorso unico | | | |
| T3 ereditarietà | | | |
| T4 aggiramento | | | |
| T5 rilevabile globalmente | | | |

## SP-5 — Iniettabilità e riproducibilità

| Criterio | Rust | Go | TypeScript |
|---|---|---|---|
| C1 stesso seed → stessa traccia | | | |
| C2 seed diversi → tracce diverse | | | |
| C3 tempo virtuale | | | |
| C4 guasto riproducibile | | | |
| C5 nessun orologio/RNG globale | | | |
| C6 concorrenza nativa ordinabile | | | |

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

Una riga per criterio e candidato: comando eseguito, output osservato, divergenze
rispetto a ciò che ci si aspettava.
