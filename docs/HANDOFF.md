# Handoff — ripresa del progetto

Scritto il **2026-08-06**, alla chiusura della sessione di progettazione del kernel.
Serve a riprendere senza rifare, e senza rilitigare ciò che è già deciso.

## In trenta secondi

Assistente desktop locale, utente singolo, GPU singola RTX 5080 16 GB. **Piattaforma
a quattro pilastri paritari** su kernel comune. Spec del kernel **§0–§10 completa, 30
ADR**. Stack deciso **tranne il guscio della GUI**: core in **Rust**, interfaccia web in
**Vue 3**, worker ML in **Python**; Tauri contro Electron è ancora aperto
([ADR-0029](adr/0029-guscio-della-gui.md), `Proposed`) e non blocca nulla.

⚠️ **Una lacuna aperta nel kernel**, trovata durante la revisione e non cercata: **la
GPU usata dalla GUI non è arbitrata da nessuno.** Vedi sotto — va chiusa nel
sotto-progetto 1.

Il vincolo che governa tutto non è funzionale ma di risorsa: quattro aree che si
contendono una sola GPU.

L'unico codice nel repository è in [`../spikes/rust/`](../spikes/rust/): sono **prove**,
non il kernel.

## Prima cosa da fare

**Scrivere la spec del sotto-progetto 1** — kernel + simulatore DST — poi il piano, poi
il codice. Vale «spec prima del codice» come per tutto il resto.

Lo stack non è più una domanda aperta:

| ADR | Decisione | Misurata da |
|---|---|---|
| **0026** | core in **Rust** | SP-5 e SP-6 su tre candidati. Rust è l'unico che passa entrambi |
| **0027** | GUI a **interfaccia web**, non toolkit nativo | G7, con P1–P4 misurati su un prototipo IPC |
| **0028** | worker ML in **Python** | non una scelta: i modelli hanno implementazioni Python |
| **0029** | ⚠️ **guscio: aperto** — Tauri o Electron | **niente**: sono argomenti, non misure. È il motivo per cui è `Proposed` |
| **0030** | interfaccia in **Vue 3** | merito + competenza del proprietario, criterio legittimo qui e non in ADR-0026 |

### Le due cose aperte, e perché non bloccano

| Aperta | Si chiude con | Blocca il sotto-progetto 1? |
|---|---|---|
| **guscio della GUI** (ADR-0029) | quattro misure M1–M4 su un frontend Vue minimo con scena 3D, sui due gusci | **no**: il sotto-progetto 1 è interamente Rust e non tocca la GUI |
| **GPU della GUI non arbitrata** | un ADR quando si progetta l'arbitro | **no, ma è lì che va chiusa** — è L1, non GUI |

#### La lacuna su I2, per esteso

[ADR-0005](adr/0005-arbitrato-gpu-su-due-dimensioni.md) e
[design/02](design/02-arbitrato-gpu.md) **non menzionano mai la GUI**. La verifica di I2
è scritta solo sui worker: «nessun *worker* si avvia senza una concessione valida».

Ma un viewer 3D (G6) usa la GPU, e il compositing della webview la usa anche senza 3D.
Durante un render TRELLIS2 che vuole 13–14 GB su 16, quella VRAM è contesa. Tre uscite:

| Opzione | Conseguenza |
|---|---|
| il viewer chiede una concessione come tutti | I2 resta vero, ma la GUI smette di essere «solo stato di presentazione» (I1) |
| il viewer è esente | **I2 è falso come scritto** e va riformulato dichiarando il rischio |
| quota sottratta, come per l'audio | riusa il meccanismo che ADR-0005 ha già inventato per la voce |

Vale identico per Tauri e per Electron: **è una questione di kernel, non di guscio.**

Toolchain verificata il 2026-08-06: `rustc` 1.95.0 · `cargo` 1.95.0 · `clippy` 0.1.95.

### I quattro vincoli che ADR-0026 impone alla prima riga di codice

Conseguenze **misurate**, non raccomandazioni. Vanno tradotte in controlli automatici.

| # | Vincolo | Perché |
|---|---|---|
| 1 | il kernel è una **crate propria**, la piattaforma un'altra | i confini sono a granularità di crate, non di modulo |
| 2 | `#![forbid(unsafe_code)]`, **non** `deny` | `forbid` non è scavalcabile da un `#[allow]` locale (`E0453`) |
| 3 | la crate del kernel è `#![no_std]` + `alloc` | è ciò che rende `E0433` un errore del **compilatore** e non un lint |
| 4 | **`std::collections::HashMap` vietato** | vedi gotcha #12 |

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
| Il core è **Rust** (ADR-0026) | riaprirlo significa rifare SP-5 e SP-6, i cui esiti sono misurati e registrati con seed e versioni. Il criterio che ha deciso è lo **spareggio #1**, e discende da V29 e ADR-0021: **rimettere in discussione il linguaggio significa rimettere in discussione la DST**, non il linguaggio |

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
| 12 | **`std::collections::HashMap` viola V29** | `RandomState` è seminato casualmente **per processo**: l'ordine di iterazione non è riproducibile fra esecuzioni. Non compare in nessun elenco di «chiamate OS» e si manifesta come traccia divergente e inspiegabile. Usare `BTreeMap`, o un hasher fissato. *(Vale anche altrove: in Go la randomizzazione delle `map` è deliberata — misurate 8 sequenze distinte su 200 iterazioni della stessa map, e lì non c'è alternativa ordinata nella libreria standard.)* |
| 13 | **Un lint non è il compilatore** | `clippy` ferma la violazione ma `cargo build` no, e un `#[allow]` per riga la annulla. Solo `forbid` e `no_std` producono un divieto non scavalcabile. **Misurato**: la regola clippy ha bloccato un uso *legittimo* di `Instant::now()` in un test, e ha richiesto un `allow` — cioè ha dimostrato di essere aggirabile mentre faceva il proprio lavoro |
| 14 | **Un test negativo va provato _in negativo_** | il piano degli spike conteneva **due sonde di non-vacuità sbagliate su tre**: quella di TypeScript modificava il tipo sbagliato e il controllo passava comunque, quindi non provava nulla. Un controllo che non si è visto fallire **non è un controllo**. Vale per ogni test di compilazione fallita, per ogni regola di importazione, per ogni `grep` di conformità |
| 15 | **Un'evidenza scritta prima della misura è un'ipotesi, non un risultato** | il piano dettava il testo delle evidenze da riportare. Tre di quelle affermazioni sono risultate **false** alla misura — inclusa una che nascondeva un buco reale nel confine dei tipi. Si esegue, si misura, si registra ciò che si è visto; dove diverge, si registra la divergenza |

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
| ❌ ri-derivare l'architettura | è in **30 ADR**, ciascuno con alternative scartate e motivo |
| ❌ riscrivere `tracciabilita.md` da zero | 170 funzionalità già mappate: si **aggiorna**, non si rigenera |
| ❌ ri-cercare lo stato dell'arte già tracciato | è in `riferimenti.md` con le fonti. Verificane semmai l'invecchiamento |
| ❌ rifare gli spike SP-5 e SP-6 | esiti, seed, versioni e comandi sono in [`../spikes/RISULTATI.md`](../spikes/RISULTATI.md). I prototipi esclusi sono recuperabili dalla storia git, lo SHA è lì |
| ❌ progettare una capacità L2 | prima il kernel deve esistere (ADR-0001) |
| ⚠️ fidarsi delle fonti senza data | l'ecosistema si muove a cadenza mensile; `riferimenti.md` riporta la data di consultazione |

## Domande legittimamente aperte

Non sono lacune: sono decisioni **rimandate con criterio**, e ciascuna ha già il metodo
per chiudersi.

| Domanda | Si chiude con | Blocca? |
|---|---|---|
| ~~Linguaggio del core~~ | ✅ **ADR-0026: Rust** | — |
| ~~Interfaccia web o toolkit nativo~~ | ✅ **ADR-0027: web** | — |
| ~~Ecosistema dei worker ML~~ | ✅ **ADR-0028: Python** | — |
| ~~Framework dell'interfaccia~~ | ✅ **ADR-0030: Vue 3** | — |
| ⚠️ **Guscio: Tauri o Electron** | ADR-0029 `Proposed`, misure M1–M4 | no |
| ⚠️ **GPU della GUI non arbitrata** | ADR nel sotto-progetto 1 | no, ma va chiusa lì |
| Motore di persistenza | ADR successivo; requisiti in §10.6, candidati Rust già verificati | l'implementazione |
| CPU della GUI con rendering reale (P3) | rimisura nel sotto-progetto 2 | no: il margine misurato è 21,4 % su 25 %, **stretto** |
| Curva qualità/VRAM di TRELLIS2 | SP-1 | no: tara i profili di risorsa |
| Voce < 600 ms sotto carico | SP-2 | no |
| Budget della proiezione per modello | SP-3 | no: vale un default conservativo, dichiarato |
| Provider con annullamento senza addebito | SP-4 | no |

## Mappa dei documenti

| File | Cosa contiene |
|---|---|
| [`../CLAUDE.md`](../CLAUDE.md) | istruzioni operative per l'agente |
| [`roadmap.md`](roadmap.md) | 11 sotto-progetti, ordine, dipendenze, decisioni aperte |
| [`tracciabilita.md`](tracciabilita.md) | 170 funzionalità → dove vive ciascuna |
| [`README.md`](README.md) | indice di ADR e diagrammi |
| [`adr/`](adr/) | **30 decisioni**. Leggi **0001** e **0004** per primi: tutto il resto ne discende. Poi **0026** (linguaggio) se devi scrivere codice |
| [`design/`](design/) | 9 diagrammi Mermaid della struttura corrente |
| [`superpowers/specs/`](superpowers/specs/) | la spec del kernel, §0–§10 |
| [`superpowers/plans/`](superpowers/plans/) | il piano dello stack — **eseguito**, con l'errata in testa che documenta cosa il piano sbagliava |
| [`riferimenti.md`](riferimenti.md) | fonti esterne, con data e con **cosa non abbiamo adottato** |
| [`../spikes/`](../spikes/) | **prove, non kernel.** `PROTOCOLLO.md` criteri e soglie · `CANDIDATI.md` pre-selezione · `RISULTATI.md` esiti, seed, versioni, evidenze · `GUI-REQUISITI.md` G1–G21 e P1–P4 |
| [`../spikes/rust/`](../spikes/rust/) | il prototipo vincente: confine dei tipi, esecutore deterministico, `Future` native, giornale write-ahead. **Punto di partenza del simulatore** |
| [`../spikes/gui-ipc/`](../spikes/gui-ipc/) | prototipo IPC con P1–P4 misurati |
| `../scripts/check-docs.sh` | controllo di coerenza, verificato anche in negativo. **Da eseguire prima di ogni commit di documentazione** |

## Come si aggiorna questo handoff

Alla chiusura di ogni sotto-progetto, **nello stesso passaggio**: `roadmap.md`,
`tracciabilita.md`, lo stato degli spike, `CLAUDE.md` se cambia il prossimo passo, e
questo file se emergono gotcha nuovi.

Un documento di stato disallineato è peggio di nessun documento: mente con autorevolezza.
