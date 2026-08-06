# Handoff — ripresa del progetto

Scritto il **2026-08-06**, alla chiusura della sessione di progettazione del kernel.
Serve a riprendere senza rifare, e senza rilitigare ciò che è già deciso.

## In trenta secondi

Assistente desktop locale, utente singolo, GPU singola RTX 5080 16 GB. **Piattaforma
a quattro pilastri paritari** su kernel comune. Progettazione conclusa per il kernel:
**spec §0–§10 completa, 25 ADR, zero lacune aperte, zero righe di codice**.

Il vincolo che governa tutto non è funzionale ma di risorsa: quattro aree che si
contendono una sola GPU.

## Prima cosa da fare

Eseguire il piano [`superpowers/plans/2026-08-06-spike-linguaggio-del-core.md`](superpowers/plans/2026-08-06-spike-linguaggio-del-core.md)
— 11 task, 75 step. Produce tre ADR che chiudono **l'intero stack**:

| ADR | Cosa decide | Come |
|---|---|---|
| **0026** | linguaggio del core | spike SP-5 e SP-6 su Rust, Go, TypeScript |
| **0027** | stack della GUI | valutato **in coppia** col core, non da solo |
| **0028** | ecosistema dei worker ML | ratifica una scelta finora implicita |

**Non scrivere codice del kernel prima di ADR-0026.** Due spike possono escludere un
linguaggio, e scoprirlo dopo significa riscrivere.

Prerequisiti sulla macchina, verificati il 2026-08-06: Rust 1.95 ✅ · Node 24.9 ✅ ·
**Go ❌ da installare** (`winget install --id GoLang.Go -e`), altrimenti i task 5 e 6
non partono.

## Non rilitigabile

Venticinque ADR in stato `Accepted`. Rimetterne in discussione uno **richiede un ADR
nuovo che lo superi** (`Superseded by`), non una conversazione. Le otto decisioni che
è più probabile qualcuno voglia riaprire per comodità, e la ragione per cui non si fa:

| Decisione | Se la riapri |
|---|---|
| I quattro pilastri sono **paritari**; nessuno ha accesso privilegiato al kernel (ADR-0001) | il kernel diventa il servo di un pilastro e gli altri tre restano cittadini di seconda classe per sempre |
| **Tre** classi di processo, non quattro (ADR-0004) | la quarta si giustifica contro la tabella, o non si fa |
| **Nessun codice di terzi in-process** (ADR-0003) | rientrano contratto pubblico da congelare e superficie d'attacco |
| Default **OpenRouter, VRAM libera** (ADR-0006) | lo swap coordinato passa da eccezione a caso normale e cambia tutta la UX di attesa |
| **Fail-closed** sui vincoli dei dati (ADR-0012) | la protezione torna a essere una preferenza |
| Il **contesto è una proiezione**, non lo stato (ADR-0008) | le run lunghe tornano a perdere informazione in modo irreversibile |
| **Nessun modello** nel percorso decisionale del kernel (ADR-0020) | un fallimento del kernel smette di essere sempre un difetto, e la DST diventa impossibile |
| L'anello 4 **propone**, l'utente approva (ADR-0009) | il harness si auto-modifica in silenzio e diventa indebuggabile |

## Le tre proprietà che non si aggiungono dopo

Se le trascuri, la correzione non è una patch: è una riscrittura.

| # | Proprietà | Da |
|---|---|---|
| 1 | Confine dei dati non fidati **nel sistema di tipi** | I6 · ADR-0014 |
| 2 | Nessuna chiamata OS-specifica nel kernel | I3 · ADR-0002 |
| 3 | **Iniettabilità** di tempo, casualità, I/O e scheduling | V29 · ADR-0021 |

Più una quarta, di natura diversa ma altrettanto vincolante: **nessuna esecuzione di
codice o comando sotto il livello 2 di confinamento** (V35 · ADR-0025).

## I gotcha

Trappole reali, alcune trovate correggendo errori già commessi in questo progetto.

| # | Trappola | Perché fa male |
|---|---|---|
| 1 | **«Tutto è una run» vale solo per l'inferenza _generativa_** | applicarlo a wake word, VAD e trascrizione continua giornalerebbe migliaia di frammenti: viola Q1 e riempie il giornale di rumore. Quelle sono **sorgenti di eventi**, mai passi |
| 2 | **Ritentativo o passo nuovo?** Il discriminante è: *il modello ha prodotto output?* | no (trasporto, 5xx, rifiuto dell'arbitro) → stesso passo. Sì ma respinto da un sensore → passo nuovo, perché quell'output esiste, è stato pagato e deve restare visibile all'anello 4 |
| 3 | **Policy VRAM ≠ destinazione della richiesta** | V3 riguarda *cosa risiede in memoria*. In policy LOCALE una singola richiesta può finire su un provider remoto senza che la policy cambi |
| 4 | **La quota audio sottratta non esenta da I2** | il worker audio ha una concessione *permanente e non prelazionabile*, non l'assenza di concessione |
| 5 | **I permessi applicativi non sono un confine contro codice eseguito** | un processo figlio non passa dal mediatore: apre ciò che l'utente può aprire. Serve il livello 2 |
| 6 | **«Cifrato a riposo» qui vale quanto l'account OS** | va detto *in interfaccia*, non solo nell'ADR. Una falsa sicurezza è peggio di nessuna sicurezza |
| 7 | **Il giornale è la sorgente; trace, contesto, costi e metriche sono proiezioni** | non costruire un secondo sistema di osservabilità: esiste già, ed è il giornale |
| 8 | **Ogni requisito Q deve avere un metodo di verifica** (V30) | la §10 ha violato questa regola appena scritta, aggiungendo Q21–Q24 senza metodo. `scripts/check-docs.sh` ora lo rileva |
| 9 | **Go non ha test di compilazione fallita di serie** | un driver che compila un file *fuori* dal modulo fallisce per il motivo sbagliato: falso positivo. Va tenuto dentro il modulo, dietro un build tag |
| 10 | **xorshift resta bloccato su zero** | senza guardia sullo stato iniziale, certi seed producono una traccia vuota e lo spike sembra passare |
| 11 | **Il contesto degrada _prima_ che la finestra si riempia** (context rot) | compattare all'overflow significa lavorare degradati per gran parte di una run lunga. Si tiene un **budget target**, non una soglia |

## Il metodo di lavoro

Non è preferenza estetica: ha prodotto quattro incoerenze reali intercettate prima che
diventassero codice.

| Regola | |
|---|---|
| **Spec prima del codice** | nessun sotto-progetto si implementa senza spec approvata |
| **Sezione per sezione** | si propone, si discute, si approva, si scrive. Mai tutto insieme |
| **Decidere sul merito** | né scorciatoie né sovra-ingegnerizzazione. «Non pigro» **non** significa «più costoso»: la topologia a micro-servizi è stata scartata *perché* più costosa e peggiore |
| **Rendere verificabile** | un principio che non si può controllare è un'intenzione. Le invarianti diventano test |
| **Dichiarare i costi** | ogni ADR elenca cosa peggiora, non solo cosa migliora. Un ADR senza `Negative (accettate)` è incompleto |
| **Stato dell'arte verificato** | se una nozione non è certa si cerca **prima** di scrivere, e la fonte si traccia in [`riferimenti.md`](riferimenti.md). Mai inventare |
| **Schema-first** | tabelle, diagrammi, elenchi numerati. Niente muri di testo |
| **Audit a ogni chiusura** | `bash scripts/check-docs.sh` — link, indici, numerazioni, V30, ADR pendenti |

## Cosa NON rifare

| | |
|---|---|
| ❌ ri-derivare l'architettura | è in 25 ADR, ciascuno con alternative scartate e motivo |
| ❌ riscrivere `tracciabilita.md` da zero | 170 funzionalità già mappate: si **aggiorna**, non si rigenera |
| ❌ ri-cercare lo stato dell'arte già tracciato | è in `riferimenti.md` con le fonti. Verificane semmai l'invecchiamento |
| ❌ progettare una capacità L2 | prima il kernel deve esistere (ADR-0001) |
| ⚠️ fidarsi delle fonti senza data | l'ecosistema si muove a cadenza mensile; `riferimenti.md` riporta la data di consultazione |

## Domande legittimamente aperte

Non sono lacune: sono decisioni **rimandate con criterio**, e ciascuna ha già il metodo
per chiudersi.

| Domanda | Si chiude con | Blocca? |
|---|---|---|
| Linguaggio del core | SP-5, SP-6 → ADR-0026 | ⛔ **sì, tutto** |
| Stack della GUI | ADR-0027, in coppia col core | il sotto-progetto 2 |
| Ecosistema dei worker ML | ADR-0028 | no, ma è deciso per inerzia finché non si scrive |
| Motore di persistenza | ADR successivo; requisiti già fissati in §10.6 | l'implementazione |
| Curva qualità/VRAM di TRELLIS2 | SP-1 | no: tara i profili di risorsa |
| Voce < 600 ms sotto carico | SP-2 | no |
| Budget della proiezione per modello | SP-3 | no: vale un default conservativo, dichiarato |
| Provider con annullamento senza addebito | SP-4 | no |

## Mappa dei documenti

| File | Cosa contiene |
|---|---|
| [`../CLAUDE.md`](../CLAUDE.md) | istruzioni operative per l'agente |
| [`roadmap.md`](roadmap.md) | 11 sotto-progetti, ordine, dipendenze, spike aperti |
| [`tracciabilita.md`](tracciabilita.md) | 170 funzionalità → dove vive ciascuna |
| [`README.md`](README.md) | indice di ADR e diagrammi |
| [`adr/`](adr/) | 25 decisioni. **Leggi 0001 e 0004 per primi**: tutto il resto ne discende |
| [`design/`](design/) | 9 diagrammi Mermaid della struttura corrente |
| [`superpowers/specs/`](superpowers/specs/) | la spec del kernel, §0–§10 |
| [`superpowers/plans/`](superpowers/plans/) | il piano da eseguire |
| [`riferimenti.md`](riferimenti.md) | fonti esterne, con data e con **cosa non abbiamo adottato** |
| `../scripts/check-docs.sh` | controllo di coerenza, verificato anche in negativo |

## Come si aggiorna questo handoff

Alla chiusura di ogni sotto-progetto, **nello stesso passaggio**: `roadmap.md`,
`tracciabilita.md`, lo stato degli spike, `CLAUDE.md` se cambia il prossimo passo, e
questo file se emergono gotcha nuovi.

Un documento di stato disallineato è peggio di nessun documento: mente con autorevolezza.
