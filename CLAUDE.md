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

## Stato: spec del sotto-progetto 1 in corso, approvata fino alla §7.3.

Spec del kernel completa e **33 decisioni architetturali**.

> ⚠️ **Questo non è un repository di sola documentazione.** Il codice del prodotto si
> scrive **qui**, in questo repository, quando la spec sarà completa: la
> documentazione è la fase corrente, non lo scopo. Oggi l'unico codice presente è in
> [`spikes/rust/`](spikes/rust/) — sono **prove**, non il kernel, ma
> [§2.5 della spec](docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md) dice già
> riga per riga quali pezzi salgono a `kernel/` e quali restano dove sono.

⚠️ **Una cosa da decidere nel piano, non prima:** `spikes/rust/` ha un **proprio
`Cargo.toml`** e alla radice non ce n'è nessuno. Va deciso se il workspace delle cinque
crate nasce alla radice — escludendo gli spike — o accanto ad essi.

| Strato | Scelta | Da |
|---|---|---|
| core | **Rust** | ADR-0026, sostenuto da SP-5 e SP-6 misurati |
| gui — forma | **interfaccia web**, non toolkit nativo | ADR-0027, deciso da G7 |
| gui — framework | **Vue 3** | ADR-0030 |
| gui — **guscio** | ⚠️ **aperto**: Tauri o Electron | ADR-0029, `Proposed` |
| worker ML | **Python** | ADR-0028 |
| persistenza | **`redb` 4.1.0**, con `StorageBackend` scritto da noi | ADR-0032, requisito 4 misurato |
| dipendenze del kernel | **allow-list sul grafo transitivo** | ADR-0031, emerso da una misura |
| schema IPC | **`bincode` 2.0.1** — appuntato a `2`, vedi gotcha #22 | M-1, spec §6.1.1. Prime voci della lista di ADR-0031 |
| GPU della **GUI** | **quota di presentazione sottratta**, concessione tenuta dal core | ADR-0033, chiude la lacuna su I2 |

**Il sotto-progetto 1 è iniziato**, ma come spec, non come codice: le §0–§6 e le §7.0–§7.3
sono approvate; restano §7.4–§7.7 e §8. Il guscio aperto non lo blocca — il sotto-progetto 1
è interamente Rust e non tocca la GUI.

✅ **La lacuna su I2 è chiusa** da ADR-0033: il consumo GPU della GUI si modella come
**tre consumatori distinti**, e I2 è ora verificato su tutte e tre le classi di processo.
Per i worker è passato da test a **compilatore** — la porta `process` richiede una
concessione come argomento.

## Da dove partire, in quest'ordine

| # | File | Cosa ti dà |
|---|---|---|
| 1 | [`docs/HANDOFF.md`](docs/HANDOFF.md) | **parti da qui**: gotcha, non rilitigabile, metodo, cosa non rifare |
| 2 | [`docs/roadmap.md`](docs/roadmap.md) | stato, ordine dei sotto-progetti, prossimo passo |
| 3 | [`docs/README.md`](docs/README.md) | indice di ADR, diagrammi e spec |
| 4 | [`docs/adr/`](docs/adr/) | il **perché** di ogni decisione — leggi ADR-0001 e ADR-0004 per primi |
| 5 | [`docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md`](docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md) | **il lavoro in corso**: approvata fino alla §7.3, con tutte le evidenze delle misure |
| 6 | [`docs/superpowers/specs/2026-08-06-kernel-design.md`](docs/superpowers/specs/2026-08-06-kernel-design.md) | la spec del kernel, §0–§10 — il *cosa*, di cui la precedente è il *come* |
| 7 | [`docs/tracciabilita.md`](docs/tracciabilita.md) | ogni funzionalità della mappa originale → dove vive |
| 8 | [`docs/riferimenti.md`](docs/riferimenti.md) | provenienza di ciò che non abbiamo dedotto noi |

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

**Riprendere la §7 dalla §7.4** — il catalogo dei controlli, ciascuno con la sonda *e* la
contro-sonda. Poi §7.5 la cadenza, §7.6 il perimetro negativo, §7.7 i costi. Poi §8. Poi
il piano. Poi il codice.

✅ **§7.0–§7.3 approvate e scritte.** Le evidenze di M-3 sono trasferite nella §7.2, e le
due domande che nessuna misura decideva sono chiuse:

| Domanda | Decisione |
|---|---|
| runtime o totale? | **entrambi**, due comandi e **due rimedi opposti** — una violazione fra le crate *spedite* si ripara togliendo la dipendenza, non aggiungendola alla lista. Dipendenze di sviluppo escluse, esclusione **provata** |
| cancello bare-metal? | **si aggiunge**, non sostituisce. Bersaglio spostato a **`x86_64-unknown-none`**: differisce dal reale in **una** dimensione invece di quattro |

⚠️ **Una riga di `HANDOFF.md` era sbagliata ed è corretta**: `cargo tree -e no-proc-macro`
**non** separa il grafo di runtime da quello totale — lascia dentro le dipendenze di
sviluppo, e con esse `windows-sys`. Il comando corretto è `-e normal,no-proc-macro`.
M-3 non poteva accorgersene: il suo workspace non aveva dipendenze di sviluppo.

Si presenta la sezione, si discute, si approva, si scrive. Mai tutto insieme, e mai
un'affermazione che si può misurare senza averla misurata prima.

Lo stack è deciso **tranne il guscio della GUI**, che non blocca nulla. Le domande che
bloccavano l'implementazione sono chiuse:

| ADR | Decisione | Cosa l'ha decisa |
|---|---|---|
| **0026** | core in **Rust** | è l'unico dei tre candidati che passa **entrambi** gli spike. Go fallisce C6 con una misura; TypeScript è parziale su T4, T6 e C6 |
| **0027** | GUI a **interfaccia web** | **G7**, artifacts con anteprima viva: non ammette alternativa. P1–P4 misurati |
| **0028** | worker ML in **Python** | non è una scelta: i modelli hanno implementazioni Python. L'ADR ne dichiara i costi |
| **0029** | ⚠️ guscio: **aperto** | né Tauri né Electron hanno una misura a sostegno. Raccomandazione Electron, ma è un argomento. Si chiude con **M1–M5** nel sotto-progetto 2 — la quinta è arrivata da ADR-0033 |
| **0030** | interfaccia in **Vue 3** | competenza del proprietario — criterio **legittimo qui** perché nessuna invariante vincola la scelta e la GUI è sacrificabile. In ADR-0026 non lo era |

Evidenze, seed e versioni: [`spikes/RISULTATI.md`](spikes/RISULTATI.md) ·
[`spikes/GUI-REQUISITI.md`](spikes/GUI-REQUISITI.md).

### Quattro vincoli che ADR-0026 impone alla prima riga di codice

Non sono raccomandazioni: sono conseguenze misurate, e vanno tradotte in controlli
automatici, non lasciate alla memoria.

| # | Vincolo | Perché |
|---|---|---|
| 1 | il **kernel è una crate propria**, la piattaforma un'altra | i confini di T6 sono a granularità di crate, non di modulo |
| 2 | `#![forbid(unsafe_code)]` sul kernel, non `deny` | `forbid` non è scavalcabile da un `#[allow]` locale (`E0453`) |
| 3 | la crate del kernel è `#![no_std]` + `alloc` | è ciò che rende `E0433` un errore del **compilatore** e non un lint |
| 4 | **`std::collections::HashMap` è vietato** | `RandomState` è seminato per processo: l'ordine di iterazione non è riproducibile, e viola V29 senza comparire in nessun elenco di «chiamate OS» |

Un **quinto**, misurato dopo: `no_std` impedisce di *nominare* `std`, **non** di
raggiungere l'OS attraverso una dipendenza. La lista delle dipendenze del kernel è
l'altra metà del confine — ADR-0031, gotcha #16.

La spec del kernel è completa (§0–§10) e **non ha lacune aperte**: le cinque trovate
dall'esercizio di tracciabilità sono state chiuse dalla §10.

## Manutenzione della documentazione

Alla chiusura di ogni sotto-progetto si aggiornano **nello stesso passaggio**:
`docs/roadmap.md`, `docs/tracciabilita.md`, lo stato degli spike, `docs/HANDOFF.md` se
emergono gotcha nuovi, e questo file se cambia il prossimo passo.

Prima di ogni commit di documentazione: `bash scripts/check-docs.sh`.

Un documento di stato disallineato è peggio di nessun documento: mente con autorevolezza.
