# Istruzioni per l'agente

Leggi questo file per intero, poi [`docs/HANDOFF.md`](docs/HANDOFF.md), prima di
toccare qualsiasi cosa.

## Cos'è questo progetto

Assistente desktop locale, utente singolo, Windows primario poi Linux, **GPU singola
RTX 5080 da 16 GB**, OpenRouter primario con inferenza locale opzionale.

**Piattaforma a quattro pilastri paritari** — conversazione e conoscenza, agenti e
coding, voce, generazione asset 3D — su un **kernel comune**. Nessun pilastro prevale
e nessuno ha accesso privilegiato al kernel: è la decisione fondativa (ADR-0001) e
tutto il resto ne discende.

Il vincolo dominante non è funzionale ma di risorsa: quattro aree che si contendono
una sola GPU da 16 GB.

## Stato: progettazione. Nessun codice scritto.

Non esiste sorgente in questo repository. Esiste una spec del kernel completa e 25
decisioni architetturali. **Non iniziare a implementare**: il linguaggio del core non
è ancora scelto, e due spike possono escluderne alcuni.

## Da dove partire, in quest'ordine

| # | File | Cosa ti dà |
|---|---|---|
| 1 | [`docs/HANDOFF.md`](docs/HANDOFF.md) | **parti da qui**: gotcha, non rilitigabile, metodo, cosa non rifare |
| 2 | [`docs/roadmap.md`](docs/roadmap.md) | stato, ordine dei sotto-progetti, prossimo passo |
| 3 | [`docs/README.md`](docs/README.md) | indice di ADR, diagrammi e spec |
| 4 | [`docs/adr/`](docs/adr/) | il **perché** di ogni decisione — leggi ADR-0001 e ADR-0004 per primi |
| 5 | [`docs/superpowers/specs/2026-08-06-kernel-design.md`](docs/superpowers/specs/2026-08-06-kernel-design.md) | la spec del kernel, §0–§10 |
| 6 | [`docs/tracciabilita.md`](docs/tracciabilita.md) | ogni funzionalità della mappa originale → dove vive |
| 7 | [`docs/riferimenti.md`](docs/riferimenti.md) | provenienza di ciò che non abbiamo dedotto noi |

## Come si lavora qui

| Regola | |
|---|---|
| **Spec prima del codice** | nessun sotto-progetto si implementa senza spec approvata |
| **Sezione per sezione** | si presenta, si discute, si approva, si scrive. Mai tutto insieme |
| **Decidere sul merito** | né scorciatoie né sovra-ingegnerizzazione. «Non pigro» non significa «più costoso» |
| **Rendere verificabile** | un principio che non si può controllare è un'intenzione. Gli invarianti diventano test |
| **Schema-first** | tabelle, diagrammi, elenchi numerati. Niente muri di testo |
| **Stato dell'arte verificato** | se una nozione non è certa, si cerca **prima** di scrivere e si traccia la fonte in `riferimenti.md`. Mai inventare |
| **Audit a ogni chiusura** | `bash scripts/check-docs.sh` — link, indici, numerazioni, V30, ADR pendenti |
| **Dichiarare i costi** | ogni decisione elenca ciò che peggiora, non solo ciò che migliora |
| **ADR append-only** | una decisione superata si marca `Superseded by`, non si cancella |

## Le sei invarianti del kernel (ADR-0004)

Vincolano ogni scelta successiva. Una violazione richiede un ADR, non una deroga.

| # | |
|---|---|
| I1 | Lo stato autorevole vive **solo nel core**. GUI e worker non hanno persistenza propria |
| I2 | La GPU ha **un solo proprietario**: nessun processo la tocca senza concessione dell'arbitro |
| I3 | Il core non contiene **codice OS-specifico**: tutto passa dal modulo di piattaforma |
| I4 | Il protocollo IPC è **privato, singolo, non versionato** |
| I5 | I worker sono **senza stato**: ritentativi, code e priorità stanno nel core |
| I6 | Il contenuto non fidato **non attraversa mai** il confine delle istruzioni |

## Tre proprietà che non si aggiungono dopo

Si ottengono solo costruendole dall'inizio. Se le trascuri, la correzione è una
riscrittura, non una patch.

| | Proprietà | Da |
|---|---|---|
| 1 | Confine dei dati non fidati nel sistema di tipi | I6 · ADR-0014 |
| 2 | Nessuna chiamata OS-specifica nel kernel | I3 · ADR-0002 |
| 3 | **Iniettabilità** di tempo, casualità, I/O e scheduling | V29 · ADR-0021 |

Una quarta, di natura diversa ma altrettanto vincolante: **nessuna esecuzione di codice
o comando sotto il livello 2 di confinamento** (V35 · ADR-0025). I permessi applicativi
da soli non sono un confine contro codice eseguito.

## Prossimo passo

**Eseguire il piano** [`docs/superpowers/plans/2026-08-06-spike-linguaggio-del-core.md`](docs/superpowers/plans/2026-08-06-spike-linguaggio-del-core.md).

11 task, 75 step. Chiude **l'intero stack** con tre ADR:

| ADR | Decide | Come |
|---|---|---|
| **0026** | linguaggio del core | spike SP-5 e SP-6 su Rust, Go, TypeScript |
| **0027** | stack della GUI | valutato **in coppia** col core, non da solo |
| **0028** | ecosistema dei worker ML | ratifica una scelta finora implicita |

Entrambi gli spike possono escludere un candidato, quindi precedono ADR-0026, che
precede ogni riga di codice del kernel.

Prerequisiti verificati il 2026-08-06: Rust 1.95 presente, Node 24.9 presente,
**Go assente** (`winget install --id GoLang.Go -e`) senza il quale i task 5 e 6 non
partono.

La spec del kernel è completa (§0–§10, 25 ADR) e **non ha lacune aperte**: le cinque
trovate dall'esercizio di tracciabilità sono state chiuse dalla §10.

## Manutenzione della documentazione

Alla chiusura di ogni sotto-progetto si aggiornano **nello stesso passaggio**:
`docs/roadmap.md`, `docs/tracciabilita.md`, lo stato degli spike, `docs/HANDOFF.md` se
emergono gotcha nuovi, e questo file se cambia il prossimo passo.

Prima di ogni commit di documentazione: `bash scripts/check-docs.sh`.

Un documento di stato disallineato è peggio di nessun documento: mente con autorevolezza.
