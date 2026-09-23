# Il ridimensionamento della lettura — il verbale del 2026-09-23

⚠️ **È il verbale del mandato del proprietario del 2026-09-23, eseguito lo stesso giorno.** Questo file è nato come
consegna dell'avvio, scritta alla chiusura della sessione del [disegno del design system](2026-09-22-design-system-design.md);
quel testo sta, **parola per parola**, in
[`archivio/consegna-avvio-ridimensionamento-lettura.md`](../../archivio/consegna-avvio-ridimensionamento-lettura.md), col
punto fermo scritto a metà sessione. ⚠️ **Non è una spec e non è un disegno.** Il prossimo passo sta nella §6 del
[compendio](../../COMPENDIO.md), in un posto solo.

## Il mandato

> *«nella prossima sessione prima del piano, vorrei ridimensionare i file necessari agli handoff, documentazione, e di lettura
> obbligatoria/opzionale perchè attualmente sono molto pesanti e occupano 300/400k token molto velocemente»* — 2026-09-23

## Le decisioni del proprietario

| | La domanda | La risposta | Dove sta |
|---|---|---|---|
| 1 | il metodo resta quello delle tre volte prima? | **A** — niente si cancella, la cronaca in archivio parola per parola, il documento vivo porta lo stato, ogni taglio in A/B | — |
| 2 | taglio 1 — le tre note più grandi della memoria dell'agente diventano regole corte, e la storia va parola per parola in una cartella d'archivio **fuori** dalla memoria, che non si carica | **A** | fuori dal repository, su questa macchina: `memory-archivio/` accanto a `memory/` |
| 3 | taglio 2 — la cronaca delle correzioni esce dal compendio: sedici pezzi, fuori dalla §6 e dalla sua tabella delle voci aperte, che sono decisioni del proprietario | **A** | `4a7111d`; i pezzi in [`archivio/lettura-di-apertura-storico.md`](../../archivio/lettura-di-apertura-storico.md); il tetto di `check-docs.sh` sceso con la sua regola |
| 4 | taglio 3 — i due comandi che leggono la CI da terra in una casa sola, `docs/porta-di-qualita.md`, e il disegno del design system rimanda lì | **A** | `f020cd9` |
| 5 | taglio 4 — nei piani futuri resta solo l'ultima chiusura? | *«Rivedi, ottimizza e migliora per intero claude.md (compresa questa cosa) (anche secondo i principi di decision-principles)»*: la revisione, presentata sezione per sezione, approvata **A** | `eab020d`; il file com'era in [`archivio/lettura-di-apertura-storico.md`](../../archivio/lettura-di-apertura-storico.md) |

## Prima e dopo

`cl100k_base`, limite inferiore; i comandi e la tabella stanno in [`riferimenti.md`](../../riferimenti.md), sezione
*«Sfoltimento del compendio»*, sottosezione del 2026-09-23 — lì e non qui, perché un numero vive in una casa sola.

⚠️ **Ciò che la misura ha detto, in una riga:** la lettura d'apertura era già piccola, e il peso di una sessione è il
**lavoro**. Si è tagliato dove qualcosa **cresceva senza freno** — le note di memoria, la cronaca, i puntatori dentro file
enormi — e il freno è nelle regole di `CLAUDE.md`: un documento vivo tiene **una** chiusura, la memoria tiene **regole**, e
un cancello gira **da solo**.

## Registrate, non prese

| | Perché no, oggi |
|---|---|
| `HANDOFF.md` | si legge un gotcha alla volta, e la mediana di un gotcha è sotto i 500 token; le due sezioni del sotto-progetto 1, *«Prima cosa da fare»* e *«Stato del sotto-progetto 1»*, non le legge nessuna sessione |
| il **piano della parte 2** del sotto-progetto 2 | è chiuso ed è un verbale: il suo diario non lo legge nessuna sessione, e dal taglio 3 nemmeno per la CI. La regola dei diari vale per i piani **futuri** |
| i **disegni** della GUI e del design system | sono materia prima dei piani: leggerli è lavoro, non spreco — gotcha **#119** |
| la tabella delle **voci aperte** della §6 del compendio | sono decisioni del proprietario, e si consolidano voce per voce con lui (decisione 26 della stella polare) |
| le **skill** e i **plugin** | stanno fuori dal repository e sono del proprietario. Una sessione ne carica parecchie migliaia di token — `session-resume` sola circa 5 600, `decision-principles` circa 3 900 — e i plugin accesi aggiungono le loro descrizioni a ogni sessione, in una misura che da qui **non si vede** |

## Come si riprende — scritto alla chiusura della sessione del 2026-09-23

⛔ **DA SAPERE SUBITO: niente è a metà.** Albero pulito, nessuno stash, nessuna operazione git a metà, tutto pushato, nessun server acceso, nessun subagente in corso, nessun codice di prodotto toccato.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| **ramo** | `main`, allineato a `origin`: `git fetch --all --prune`, poi `git status -sb` |
| **i commit della sessione** | `git log --oneline 61c8450..HEAD` |
| **la CI** | coi due comandi di [`porta-di-qualita.md`](../../porta-di-qualita.md), il paragrafo *«Leggere la CI da terra»*: si leggono **per primi** i commit di questa sessione |
| **cancello** | `bash scripts/gate.sh`, **da solo**, → `GATE GREEN` prima di ogni commit della sessione; `bash scripts/check-docs.sh` → `OK` |
| **la memoria dell'agente** | fuori dal repository, su questa macchina: le tre note riscritte come regole corte, e con loro la nota sul consumo della lettura; gli originali byte per byte in `~/.claude/projects/C--Users-zagor-Desktop-harness/memory-archivio/`, che non si carica. Un'altra macchina ha la sua memoria: le regole che contano per tutte ora stanno in `CLAUDE.md` |

**Il compito della sessione successiva** lo dice la §6 del compendio: il **design system** — la rilettura del proprietario
del disegno, poi il piano in una sessione nuova, come dice la sezione *«Come si riprende»* del
[disegno](2026-09-22-design-system-design.md).
