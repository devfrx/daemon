# Il rapporto del curatore del compito 8 — le cure della revisione, E102…E110

> Curatore: subagente fresco. Macchina `Jays`, il 2026-09-27; Node `v24.19.0`; Chrome `154.0.8037.58` dal nome della
> cartella; `core.autocrlf` `false` da `.git/config`, l'albero `w/lf`. Scratchpad:
> `C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\d19536a1-7be4-437b-8e11-39fed58af059\scratchpad\cure8\`
> (sotto: `<cure8>`). Clone `C:/Users/Jays/AppData/Local/Temp/rv8`: usato e **rimesso com'era**, staccato a `ac56b11`,
> `git status --porcelain` vuoto, nessun file `zz-` rimasto.

## 1. Lo stato

**`DONE_WITH_CONCERNS`** — commit **`77c4062`**, sopra `ac56b11`, uno solo, niente push (`## main...origin/main [ahead 2]`),
nessun co-autore (`git log -1 --format=%B | grep -ci co-authored` → `0`).

Le nove voci sono curate, provate nelle due direzioni e scritte, **salvo una parte di E107**: la dichiarazione decisa per
**X4** si è rivelata sbagliata, misurata (§3, E107). Mi sono fermato su X4 — né dichiarata né sondata — e la riga d'errata
lo dice come voce aperta per il coordinatore (⏳). Il resto è andato avanti.

## 2. `git show --stat HEAD`

```
 docs/superpowers/plans/2026-09-23-design-system.md | 508 +++++++++++++++++++--
 gui/src/components/BaseButton.vue                  |   6 +
 gui/src/frame/Overview.vue                         |   5 +-
 gui/src/frame/dock.ts                              |  32 +-
 gui/src/frame/frame.browser.test.ts                |  38 +-
 gui/src/frame/frame.test.ts                        |  76 ++-
 gui/src/panels/views/compact.json                  |   4 +-
 gui/src/panels/views/generate-views.test.ts        |   4 +
 gui/src/testing/axe.ts                             |   4 +-
 gui/src/testing/probes.browser.test.ts             |  14 +-
 gui/src/testing/probes.ts                          |  16 +-
 11 files changed, 645 insertions(+), 62 deletions(-)
```

## 3. Le voci, una per una

Ogni «rosso» è misurato sul codice di `ac56b11` con la prova già scritta; ogni «seconda direzione» sulla **suite intera**
(`<cure8>/mut8.py`, i log in `<cure8>/mut/m-*.log`, i JSON in `<cure8>/mut/m-*.json`), la cura tolta a mano, poi la copia
salvata che torna e `cmp` uguale.

| Voce | La cura | Rosso prima | Verde | Seconda direzione |
|---|---|---|---|---|
| **E102** | nel piano il comando jsdom del Passo 7 coi quattro file, nell'ordine del Passo 3; la riga cambia senza spostare righe | — (solo testo) | sul clone, Passo 7: `Test Files  4 passed (4)`, `Tests  62 passed (62)` | — |
| **E103** | in `generate-views.test.ts`, `write` fa `api.layout(api.width, api.height, true)` prima di scrivere; `compact.json` rigenerato | `compact: expected 50 to be greater than or equal to 94.399999` | la suite intera verde | `compact.json` a 500/500 → lo stesso rosso (riga `E103`); e il generatore senza la riga, rigenerato → `compact.json` torna a 500/500, Home e Lavoro uguali, la prova di nuovo rossa, lo stesso messaggio (`<cure8>/logs/gen2-*.log`); ripristinati, `cmp` uguale |
| **E104** | `settleIfMoved` in `createDock`, ascoltato da `onDidLayoutChange`, dalla chiusura della finestra e da `onDidMaximizedGroupChange` **differito a un microtask** | `expected 1 to be 2` | verde | senza l'ascoltatore `expected 1 to be 2` (riga `E104a`); sentito subito `expected 4 to be 2` (riga `E104b`) |
| **E105** | in `BaseButton.vue` `.base-button:disabled :deep(.base-label), .base-button:disabled :deep(.base-icon) { color: inherit; }` | chiaro `expected 'rgb(101, 91, 87)' to be 'rgb(163, 154, 143)'`; scuro `expected 'rgb(163, 154, 143)' to be 'rgb(111, 102, 96)'` | verde; kit e contrasti invariati | regola tolta → lo stesso rosso (`E105a`); senza il selettore dell'icona → chiaro `expected 'rgb(122, 31, 46)' to be 'rgb(163, 154, 143)'`, scuro `expected 'rgb(191, 85, 103)' to be 'rgb(111, 102, 96)'` (`E105b`); le parole senza `:disabled` → chiaro `expected 'rgb(163, 154, 143)' to be 'rgb(101, 91, 87)'`, scuro `expected 'rgb(111, 102, 96)' to be 'rgb(163, 154, 143)'` (`E105d`, tre corse, lo stesso messaggio); la regola intera senza `:disabled` → l'icona resta bordeaux, lo stesso messaggio di `E105b` (`E105c`: il peso misurato) |
| **E106** | `:placeholder="$t('overview.name')"` sul campo del nome | `expected null to be 'Nome della vista'` | verde | riga tolta → lo stesso rosso |
| **E107** | attese nelle prove che c'erano: X2, X3, X5, X8, X10 nella prova delle frecce; X6, X7, X9 sotto jsdom | — (tengono righe che c'erano: verdi sul codice) | verdi | X2 `expected [ -1, +0, -1, -1 ] to deeply equal [ -1, -1, -1, +0 ]`; X3 `expected <button data-v-2d3a2488 …(7)>…(1)</button> to be <button data-v-2d3a2488 …(8)>…(2)</button>`; X5, X6, X10 `expected false to be true`; X7 `expected [ null, 'page', null, null, …(2) ] to deeply equal [ Array(6) ]`; X8 `expected <form data-v-b7d5761d …(1)>…(2)</form> to be null`; X9 `expected true to be false` |
| **E108** | `computed` chiama prima `readToken(token)`; l'import in testa a `probes.ts`; prova nuova in `probes.browser.test.ts` | `expected [Function] to throw an error` | verde | guardia tolta → lo stesso rosso |
| **E109** | le quattro frasi riscritte | — | — | nessuna prova cambia |
| **E110** | nessuna cura; la riga d'errata | — | — | — |

Ogni riga di `mut8.py`: `tests_as_row=True` — cadono **solo** le prove che la riga nomina, nei due temi dove la prova è
nei due temi —, `cmp=True`, e alla fine `git status` come prima (`<cure8>/logs/mut8.log`).

**E104, la regola che c'è** (`<cure8>/diag/zz-cure-show.test.ts`, log `<cure8>/logs/diag-show-*.log`), sotto jsdom nel
clone, `dockview-core` 8.3.1:

| | Mostrare una vista col nome ingrandita, poi Home (S1) | Home ingrandita a mano, poi Lavoro, poi Home (S2) |
|---|---|---|
| prima della cura | 0 e 0 `SaveLayout` | l'ingrandimento **non** salvato; Lavoro e Home: nessun salvataggio |
| ascoltatore sentito subito | **1 e 2**: la vista col nome riscritta a metà `fromJSON`; poi `layouts.home` = la vista col nome a metà smontata, la sola `costs` | l'ingrandimento salvato; poi `layouts.work` = le schede di Home, e Home riscritta senza `activeGroup` |
| cura differita | **0 e 0** | l'ingrandimento salvato col `maximizedNode`; Lavoro e Home: nessun salvataggio, Home si riapre ingrandita, `activeGroup` `"3"` |

**E107, X4** (`<cure8>/diag/zz-cure-x4.test.ts`, log `<cure8>/logs/diag-x4-*.log`): un nome scritto a metà, «Revis»; F3
chiude la Panoramica, F3 la riapre. Col codice: nessun campo aperto (verde). Senza le due righe del `watch`
(`naming.value = false; name.value = "";`): `expected { naming: true, value: 'Revis' } to deeply equal { naming: false,
value: null }`. Quindi *«ripristino che nessun caso d'uso oggi produce»* è falso per X4: chiudere a metà nome e riaprire è
un uso di oggi. **Non** l'ho dichiarata né sondata: tocca al coordinatore. Un'attesa costerebbe poche righe nella prova
*«says under the field…»* sotto jsdom — questo è un fatto, non una proposta decisa.

**E107, le due attese della revisione che non avrebbero preso la loro riga** (`<cure8>/variants.py`, log
`<cure8>/variants/*.log`): la prova delle frecce con `ArrowLeft` nel campo e X5 mutata → `Tests  1 passed`; la prova con la
sola riapertura dopo Invio su Lavoro e X3 mutata → `Tests  1 passed`. File tornati dalla copia, `cmp` uguale.

## 4. Le righe nuove del Passo 9 (sedici)

Quelle della tabella del piano, misurate sulla suite intera come sopra: `E103` (miniatura), `E104a`/`E104b`
(l'ingrandimento che si salva; mostrare che non salva), `E105a`/`E105b`/`E105d` (spenta; l'icona; le parole dopo la
risposta), `E106`, X2, X3, X5, X6, X7, X8, X9, X10, `E108`. Nessuna riga nuova per E109 (commenti) né per E110.

## 5. Le misure del §4

| Che cosa | Uscita |
|---|---|
| albero: `(cd gui && npm test && npm run build && npm run lint)` | `Test Files  27 passed \| 1 skipped (28)`, `Tests  236 passed \| 1 skipped (237)`; build e linter puliti |
| albero: tre corse `npx vitest run --reporter=json --outputFile=<cure8>/corsa-N.json` | 28 file, **236** passate, **0** cadute, 1 saltata, ciascuna |
| il pezzo JavaScript | `dist/assets/index-Bg0Z_4Ot.js  698.20 kB │ gzip: 213.00 kB`, contro `698.14 kB` e `212.98 kB` |
| clone, testo curato intero su `bc6e94d` pulito | `50 operations, 0 refused`; dopo il commit, `git add -A` e `git diff --cached --stat 77c4062 -- gui/` **vuoto**, 0 byte, `--summary` vuoto |
| clone, sequenza: `--upto 2`, Passo 3 | `29 operations, 0 refused`; jsdom `Test Files  3 failed \| 1 passed (4)`, `Tests  1 failed \| 24 passed (25)`, i due import e `layout.showNamed is not a function`; browser `Tests  10 failed \| 39 passed (49)`, i dieci messaggi dell'Atteso; `probes.browser.test.ts` `Tests  9 passed (9)` |
| clone, `--only-step 4`, `5`, `6`, `7` sopra, Passo 7 | `7`, `8`, `4`, `2 operations, 0 refused`; jsdom `Tests  62 passed (62)`; browser `Tests  49 passed (49)` |
| clone, Passo 8 | `Tests  236 passed \| 1 skipped (237)`; build e linter puliti; `698.20 kB`, `213.00 kB`; il risultato della sequenza identico all'albero |
| il cancello, da solo, sull'albero | `GATE GREEN.`; jsdom `Test Files  21 passed \| 1 skipped (22)`, `Tests  170 passed \| 1 skipped (171)`; browser `Test Files  6 passed (6)`, `Tests  66 passed (66)`; `found 0 vulnerabilities` (`<cure8>/logs/gate.log`) |
| `bash scripts/check-docs.sh` | `OK — no inconsistencies.` |
| `git status --porcelain` prima del commit | i soli dieci file delle cure e il piano; dopo, vuoto |

Gli Atteso del testo curato sono scritti coi numeri qui sopra; nessuno è cambiato rispetto a ciò che avevo scritto prima di
misurare.

## 6. I fine-riga

Prima (`ac56b11`) e dopo (`77c4062`), `git ls-files --eol` e `tr -cd '\r' | wc -c`: tutti **`i/lf w/lf`**, **0 CR**.

| File | Righe prima | Righe dopo |
|---|---|---|
| `docs/superpowers/plans/2026-09-23-design-system.md` | 11760 | 12208 |
| `gui/src/components/BaseButton.vue` | 151 | 157 |
| `gui/src/frame/Overview.vue` | 275 | 276 |
| `gui/src/frame/dock.ts` | 157 | 163 |
| `gui/src/frame/frame.browser.test.ts` | 287 | 319 |
| `gui/src/frame/frame.test.ts` | 488 | 554 |
| `gui/src/panels/views/compact.json` | 64 | 64 |
| `gui/src/panels/views/generate-views.test.ts` | 99 | 103 |
| `gui/src/testing/axe.ts` | 27 | 27 |
| `gui/src/testing/probes.browser.test.ts` | 107 | 119 |
| `gui/src/testing/probes.ts` | 177 | 185 |

`home.json` e `work.json`, rigenerati, sono usciti uguali byte per byte e non sono nel commit.

## 7. Ciò che ho deciso io, e perché

1. **E103** — `api.layout(api.width, api.height, true)` invece dei numeri `1600, 1000` scritti una seconda volta: la griglia
   alla sua misura, nessuna costante ripetuta; l'uscita misurata è la stessa, `[944, 56]`. E l'attesa legge la riga della
   striscia dal JSON spedito, `maximumHeight / grid.height`, non dalla tessera della striscia nello schema: col file di prima
   la tessera della striscia comincia al 50 %, dove finisce la knowledge base, e un'attesa letta dallo schema sarebbe stata
   verde — dedotto dai numeri del file, 500 e 500.
2. **E104** — l'ascoltatore **differito** (`queueMicrotask`): misurato che sentito subito rompe la regola che c'è (§3). La
   chiusura della finestra usa la stessa funzione, che ne aveva una copia; la testa di `createDock` lo dice. La prova usa il
   giro Lavoro → Home con Home ingrandita invece di una vista col nome: la stessa strada, `show` → `apply` → `fromJSON`; la
   vista col nome è misurata nella diagnostica.
3. **E105** — due attese in più della sola parola spenta: l'icona (il secondo selettore della regola, che altrimenti nessuna
   prova terrebbe) e le parole di nuovo nel `--color-text-muted` dopo la risposta (la seconda direzione di `:disabled`). Il
   rosso di `E105d` è il colore dello spento all'inizio della transizione del pulsante: stabile in tre corse, e la riga lo dice.
4. **E107** — X5 con la freccia **a destra** e X3 con **giù, Esc e F3** prima di riaprire, perché le attese del testo della
   revisione non prendevano la loro riga (misurato, §3); l'aiutante `press` di `frame.test.ts` crea un evento `cancelable` e lo
   rende, per `defaultPrevented`.
5. **E108** — la metà verde della prova nuova con una proprietà personalizzata messa a mano sulla radice (`--probe-colour`),
   perché quel file di prova non carica i token.
6. **E109** — nel commento di `Overview.vue`, oltre alla frase della revisione, *«would send one without the core's
   layouts»* al posto di *«would send `layouts: {}`»*: con un `settle` del dock prima del benvenuto il pacchetto non è
   `layouts: {}`.
7. **Il testo del compito** — le undici operazioni nuove dove il prompt le mette, con una frase d'introduzione ciascuna; nel
   Passo 3 una frase che dice perché le prove delle cure non cambiano i conti; gli Atteso dei Passi 7 e 8; i Files e il
   Produces. Il generatore del testo è `<cure8>/cure8plan.py` (modello: `fix8.py`), con la prosa in `<cure8>/prose.py` e le
   righe d'errata in `<cure8>/rows.md`: rigenera la sezione dalla copia curata e verifica in memoria che le cinquanta
   operazioni, applicate ai file di `bc6e94d`, rendano l'albero byte per byte.
8. **La sequenza sul clone** — il prompt dice *«`--upto 7` sopra»*: riapplicherebbe i Passi 1–2 già applicati, e i rifiuti
   sarebbero certi; ho applicato `--only-step 4`, `5`, `6`, `7`, uno alla volta, com'era nella revisione (§3, riga 14).

## 8. Ciò che non ho potuto misurare, e ciò che il coordinatore deve sapere

- **X4** (E107): la decisione è del coordinatore — la misura è al §3.
- **E104 nel browser vero**: la cura è provata sotto jsdom (`dockview-core` 8.3.1, la stessa libreria); nella SPA dal vivo
  non l'ho guardata — la presa grande, l'ingrandimento e il ripristino li guarda il Passo 10.
- **E105 a vista**: provata sui colori calcolati, non guardata; lo sguardo del Passo 10 la vede.
- **Lo sguardo** (Passo 10, controllo 15), la **CI** dopo il push, la macchina **`zagor`**: non da qui.
- ⚠️ **La ricetta di `compare_task8.py`** (nel *«Come si riprende»* del piano, che non ho toccato) porta i numeri di riga
  del piano a `bc6e94d`, e lì resta valida per il confronto `bc6e94d` → `ac56b11`; il testo curato ha spostato le righe del
  compito 8 — la sua intestazione da 9061 a 9070, quella del compito 9 da 11021 a 11469 (`git show <commit>:<piano> | grep -n
  '^## Compito [89]:'`) —, quindi una ricetta per confrontare un'esecuzione futura col testo curato va rifatta. E102 da sola non
  spostava righe, come la revisione diceva; le altre cure sì.
