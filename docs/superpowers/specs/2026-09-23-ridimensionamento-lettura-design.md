# Il ridimensionamento della lettura — il verbale del 2026-09-23

⚠️ **È il verbale del mandato del proprietario del 2026-09-23, eseguito lo stesso giorno.** Questo file è nato come
consegna dell'avvio, scritta alla chiusura della sessione del [disegno del design system](2026-09-22-design-system-design.md);
quel testo sta, **parola per parola**, in
[`archivio/consegna-avvio-ridimensionamento-lettura.md`](../../archivio/consegna-avvio-ridimensionamento-lettura.md), col
punto fermo scritto a metà sessione. ✅ **Richiamo del 2026-09-24:** qui anche la seconda passata, la **compressione senza perdite del compendio** — la
sezione in coda, prima di *«Come si riprende»*. ✅ **E, la sera dello stesso giorno, la terza, a metà:** la compressione di `porta-di-qualita.md` — la sezione che segue la seconda. ⚠️ **Non è una spec e non è un disegno.** Il prossimo passo sta nella §6 del
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

## La passata del 2026-09-24 — la compressione del compendio

Il giorno dopo, il proprietario ha lanciato la skill `lean-docs`. Le misure della lettura fissa, prima e dopo, e i comandi
che le rifanno stanno in [`riferimenti.md`](../../riferimenti.md), *«Sfoltimento del compendio»*, sottosezione del
2026-09-24 — lì e non qui, perché un numero vive in una casa sola.

| | La domanda | La risposta | Dove sta |
|---|---|---|---|
| 6 | che cosa si snellisce: il **compendio**, fuori dalla §5 e dalla tabella delle voci aperte, o solo le **cose piccole** — la nota di memoria diventata diario e l'indice della memoria? | **A**, il compendio | — |
| 7 | sezione per sezione: la §6, la §12, la testa con la §13, la §1 | *«decidi secondo decision-principles»*, a ciascuna: **A** su tutte, coi cinque criteri controllati uno per uno; la trappola 6 della §10, lo stesso taglio, decisa allo stesso modo | il commit della passata; i testi com'erano in [`archivio/lettura-di-apertura-storico.md`](../../archivio/lettura-di-apertura-storico.md) e, quelli della §6, in [`archivio/stato-storico.md`](../../archivio/stato-storico.md); il tetto di `check-docs.sh` sceso con la sua regola |

⚠️ **Una frase corretta e non compressa, e lo si dice:** la riga della §12 sul disegno del Traguardo 5 diceva *«ed è il
file da cui si riprende»*, falsa dal 2026-08-25, quando il traguardo si è chiuso. È uscita, e il testo com'era sta in
archivio.

✅ **La prova che non manca niente**, in due metà. Meccanica: lo script della skill non trova àncore perse né link rotti, e
ogni blocco archiviato è uguale all'originale a meno dei link riscritti per la cartella. Funzionale: un sotto-agente che
leggeva **solo** il compendio nuovo ha risposto a sette domande su sette, scelte fra le eccezioni e i casi limite delle
sezioni toccate.

### Registrate, non prese — 2026-09-24

| | Perché no, oggi |
|---|---|
| la §4 e la §11 | un centinaio di token ciascuna, e nella §4 la tabella della nomenclatura sarebbe diventata prosa, contro la forma a tabelle che il proprietario vuole |
| la §5 | è già la compressione degli ADR |
| la tabella delle **voci aperte** della §6 | resta del proprietario, voce per voce; è il grasso più grosso rimasto, e la misura sta in [`riferimenti.md`](../../riferimenti.md) |
| la nota di memoria `lettura-obbligatoria-a-blocchi-di-400-righe.md` e l'indice della memoria | la prima è ancora un diario di aggiunte datate, il secondo ha righe lunghe: erano la via **B**, non scelta |
| l'ordine della §10 | la trappola 6 sta prima della 5, e lo ha notato il sotto-agente; era così anche prima, e riordinare non è comprimere |

## La passata del 2026-09-24, sera — `porta-di-qualita.md`, a metà

Il proprietario ha rilanciato `lean-docs` chiedendo quali fossero i file più grandi. La misura — `git ls-files '*.md' |
xargs wc -c | sort -rn | head`, e i token col comando di `tiktoken` di `CLAUDE.md` puntato su quei file — ha detto che i
più grandi sono **verbali**: piani eseguiti e archivio, che `CLAUDE.md` vuole come sono e che nessuna sessione apre
all'inizio. Il documento **vivo** più grande è [`porta-di-qualita.md`](../../porta-di-qualita.md), che si legge a pezzi a
ogni piano e a ogni lettura della CI.

| | La domanda | La risposta | Dove sta |
|---|---|---|---|
| 8 | da quale file si parte: `porta-di-qualita.md`, il vivo più grande, o i più grandi verbali compresi? | **A**, `porta-di-qualita.md`; dopo, se regge, `HANDOFF.md` e `riferimenti.md` | — |
| 9 | l'inventario adesso, con quattro sotto-agenti su fette disgiunte, o in una sessione nuova? | **A**, adesso — costo detto prima, 0,6–1M token; speso circa 1,04M, quattro Opus | fuori da git: *«Come si riprende»* |
| 10 | dove va la cronaca? | **A** — il file di oggi **intero**, parola per parola, in `archivio/porta-di-qualita-storico.md`; il file vivo resta al suo percorso, riscritto con le sole unità vive, e porta un indice *«sezione di prima → archivio»*; i commenti nel codice **non** si toccano: li risolve l'indice | la riscrittura |
| 11 | le contraddizioni: si comprime prima, o si corregge prima? | **A**, si comprime prima. Quelle dentro la cronaca vanno in archivio col resto e restano vere come verbale del loro giorno; quelle nelle unità vive restano com'erano, **segnate** e in una lista, e si correggono dopo, in una sessione loro | la riscrittura |
| 12 | quando la riscrittura? | **A**, la sessione successiva, prima del design system | la §6 del [compendio](../../COMPENDIO.md) |

### Ciò che l'inventario ha trovato, e che la riscrittura deve tenere

⚠️ **Quasi nessuna sezione è verbale pura:** la cronaca sta **dentro** le sezioni — tabelle di mutazione, richiami datati,
riconteggi — accanto alle unità vive. Per questo la decisione 10 archivia il file **intero** invece di ritagliarne i pezzi.

| Che cosa | Chi la legge |
|---|---|
| le intestazioni delle due tabelle delle voci aperte e di *«Cosa la porta NON controlla»*, il loro **ordine** — il Traguardo 6 resta in fondo —, le colonne e le parole che discriminano, e la riga 29 del Traguardo 6 | gli `awk` della §6 del compendio e quelli dentro il file |
| le righe della tabella del livello 1 che aprono con `**blocco C**` | il `grep -cE` di [`riferimenti.md`](../../riferimenti.md), *«I comandi con cui si riconta il catalogo»* |
| il **nome di ogni file di test** e di ogni caso `compile_fail` | il ciclo degli orfani, nello stesso posto di `riferimenti.md`. ⚠️ Già oggi ne rende alcuni: il ciclo li nomina, e la riscrittura non ne aggiunge |
| il paragrafo *«Leggere la CI da terra»*, nella testa | le consegne di fine sessione |
| i titoli che il codice nomina — `task 8 section`, `section "P-2"`, `row 15 of the mutation campaign`, le voci aperte numerate | una ventina di commenti in `crates/`: li risolve l'indice verso l'archivio |

Le contraddizioni, contate negli inventari, sono **43**: fatti scritti al presente che il codice di oggi o un'altra parte
dello stesso file smentisce. Sono **registrate, non risolte**, in ciascun inventario con la prova.

## Come si riprende — scritto alla chiusura della sessione del 2026-09-24, sera

⛔ **DA SAPERE SUBITO: la passata è A METÀ, e la sua materia prima NON è in git.** Albero pulito, tutto pushato, nessun
subagente in corso, nessun codice di prodotto toccato. La chiusura precedente sta, parola per parola, in
[`archivio/consegna-avvio-ridimensionamento-lettura.md`](../../archivio/consegna-avvio-ridimensionamento-lettura.md).

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| **ramo** | `main`, allineato a `origin`: `git fetch --all --prune`, poi `git status -sb` |
| **i commit della sessione** | `git log --oneline c4ec042..HEAD` |
| **la CI** | coi due comandi di [`porta-di-qualita.md`](../../porta-di-qualita.md), il paragrafo *«Leggere la CI da terra»*: si legge **per prima** quella del commit di questa sessione |
| **cancello** | `bash scripts/gate.sh`, **da solo**, → `GATE GREEN` prima del commit; `bash scripts/check-docs.sh` → `OK` |
| **la materia prima** | sulla macchina `zagor`, in `.lean-docs/` alla radice del repository, esclusa da git con `.git/info/exclude`: `prima/docs/porta-di-qualita.md`, la fotografia del file com'era a `c4ec042`; `mandato-comune.md`, il mandato dei quattro lettori; `inventario-S0-S5.md`, la testa e la coda, e `inventario-S1.md`…`inventario-S4.md`, le righe 83–1033, 1034–2072, 2073–3091 e 3092–4763. ⚠️ **Su un'altra macchina non esiste**, e l'inventario si rifà col costo della decisione 9 |

**Il compito della sessione successiva** — le fasi 3 e 4 di `lean-docs`, in una sessione nuova:

1. `git fetch --all --prune`, `git status -sb`; la CI della chiusura, per prima.
2. Gli inventari, a pezzi, e l'**unione fra le fette**, che non è ancora fatta: i duplicati fra una fetta e l'altra si
   vedono solo lì.
3. `archivio/porta-di-qualita-storico.md`: il file di oggi intero, con una testa d'archivio e i soli link riscritti per
   la cartella; la prova è il `diff` contro `git show c4ec042:docs/porta-di-qualita.md`, che deve mostrare **solo** i link.
4. `porta-di-qualita.md` riscritto con le unità vive, una sezione alla volta, con l'indice verso l'archivio; ogni unità
   viva contraddetta porta il suo segno, e la lista delle contraddizioni sta in fondo.
5. La prova che non manca niente: lo script `misura.py` della skill, `--prima .lean-docs/prima/ --dopo docs/porta-di-qualita.md`;
   gli `awk` e il `grep -cE` qui sopra, che rendono **lo stesso** di prima; il ciclo degli orfani, che non ne rende di più;
   `check-docs.sh` e `gate.sh`; e un sotto-agente che legge **solo** il file nuovo e risponde alle domande di controllo
   degli inventari.
6. Le misure prima e dopo, coi comandi, in [`riferimenti.md`](../../riferimenti.md), *«Sfoltimento del compendio»*; il
   verbale qui; `.lean-docs/` cancellata; commit e push.
