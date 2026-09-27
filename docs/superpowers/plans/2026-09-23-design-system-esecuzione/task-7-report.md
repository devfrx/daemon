# Il rapporto del compito 7 — le viste col nome, sotto

## 1. Lo stato

**`DONE`** — commit **`0417c9c`** su `main`, a partire da `fa1ae0e`. ⛔ Nessun `git push` (è del coordinatore, dopo la
revisione), nessun `--amend`, nessun co-autore. Macchina `Jays`: `core.autocrlf` → `false` (l'ultima di `--get-all`, da
`.git/config`), Google Chrome `154.0.8037.58` letto dal nome della cartella in `/c/Program Files/Google/Chrome/Application/`
(la cartella di `$LOCALAPPDATA` non c'è, uscita 2 come previsto), Node `v24.19.0` contro `engines`
`^22.22.2 || ^24.15.0 || >=26.0.0`.

Le verifiche d'avvio, tutte tornate: `git rev-parse --short HEAD` → `fa1ae0e`; `git status --porcelain` → vuoto;
`git log --oneline 1c168a1..HEAD -- gui/` → nulla. Il brief `task-7-brief.md` porta in testa `HEAD` = `fa1ae0e` e
*«piano e disegno coincidono con `HEAD`»*; letto tutto, a blocchi.

⚠️ **Come ho applicato il testo del piano:** con `plan_ops.py` della cartella tracciata del dispaccio,
`python plan_ops.py docs/superpowers/plans/2026-09-23-design-system.md 7 apply . --only-step N`, **un passo per volta
nell'ordine del compito** (2, poi 3, 4, 5, 6), coi rossi guardati prima del codice. Lo strumento legge i recinti dal piano a
`fa1ae0e` e scrive come `replace_unique.py` — un'occorrenza unica, `newline=""`, temporaneo e `os.replace`, i file nuovi LF,
il riscritto col terminatore che aveva —; prima, `list` ha reso le **14** operazioni del compito e un `--dry` sull'albero
`14 operations, 0 refused`. Dopo il commit, `compare_task7.py fa1ae0e 0417c9c` → i dieci file `OK`, il piano *«CHECK BY
HAND»* con la sola riga 7 cambiata, `11 paths, 0 mode changes, 0 not matching the plan's text`, uscita 0. `replace_unique.py`
l'ho copiato nello scratchpad dal brief, e non è servito.

## 2. `git show --stat HEAD`

```
commit 0417c9c  (design-system(compito 7): le viste col nome, sotto -- il pacchetto, il negozio, il dock, la geometria comune, lo schema. …)

 docs/superpowers/plans/2026-09-23-design-system.md |   2 +-
 gui/src/frame/Frame.vue                            |   5 +-
 gui/src/frame/dock.ts                              |  29 +++---
 gui/src/frame/frame.test.ts                        |  16 +++
 gui/src/frame/moveActive.ts                        |  22 ++---
 gui/src/frame/nearest.test.ts                      |  42 ++++++++
 gui/src/frame/nearest.ts                           |  40 ++++++++
 gui/src/frame/schematic.test.ts                    |  52 ++++++++++
 gui/src/frame/schematic.ts                         |  56 +++++++++++
 gui/src/stores/layout.ts                           | 101 ++++++++++++++++---
 gui/src/stores/stores.test.ts                      | 110 ++++++++++++++++++++-
 11 files changed, 432 insertions(+), 43 deletions(-)
```

Il messaggio intero sta nel commit (`git log -1 --format=%B 0417c9c`): comincia con `design-system(compito 7): `, porta le
due righe del pezzo JavaScript (**E80**) e finisce con `check-docs OK, GATE GREEN`; `grep -ci co-authored` → `0`.
`git diff --stat fa1ae0e..HEAD -- crates/ gui/schema/` → vuoto (vincolo 12).

## 3. I passi 1–7, coi comandi e le uscite vere

I log stanno nello scratchpad, `C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\e4b4f474-4c3b-4180-b8de-ae411307e17a\scratchpad\task7\`.

**Passo 1** — alle 17:18:04.

| Comando | Uscita |
|---|---|
| `git status --porcelain > <scratchpad>/prima.txt` | 0 byte: l'albero era pulito |
| `grep -rnw 'Direction' gui/src --include=*.ts --include=*.vue` | tre righe, tutte `gui/src/frame/moveActive.ts` — `:3` il tipo, `:18` `moveActive`, `:47` `directionOf` |
| `grep -rln 'i18n' gui/src/stores` | nulla (uscita 1) |
| `grep -rln 'Frame.vue' gui/src --include=*.test.ts` | nulla (uscita 1) |
| `(cd gui && npm run build 2>&1 \| grep -E 'assets/index-.*\.js ')` | `dist/assets/index-DKNLC6Y2.js  691.82 kB │ gzip: 210.77 kB` — la baseline (**E80**) |
| `bash scripts/gate.sh`, da solo, in background | `GATE GREEN.`, uscita 0, 74 s; sotto `gui/` jsdom `Test Files  19 passed \| 1 skipped (20)`, `Tests  144 passed \| 1 skipped (145)`; browser `Test Files  6 passed (6)`, `Tests  56 passed (56)`; `691.82 kB │ gzip: 210.77 kB`; `found 0 vulnerabilities` — i numeri del pre-controllo, uguali |

**Passo 2** — `plan_ops.py … 7 apply . --only-step 2` → `5 operations, 0 refused` (i due *Crea*, le due sostituzioni di
`stores.test.ts`, quella di `frame.test.ts`). Poi
`(cd gui && npx vitest run --project jsdom src/frame/nearest.test.ts src/frame/schematic.test.ts src/stores/stores.test.ts src/frame/frame.test.ts)`,
uscita 1, **rosso per le ragioni dell'Atteso**:

- `Error: Failed to resolve import "./nearest" from "src/frame/nearest.test.ts". Does the file exist?`
- `Error: Failed to resolve import "./schematic" from "src/frame/schematic.test.ts". Does the file exist?`
- *«keeps the named views and the open one…»* → `AssertionError: expected undefined to deeply equal [ { name: 'Revisione', …(1) } ]`
- *«drops what it cannot read…»* → `AssertionError: expected { view: 'home', layouts: { home: {} } } to deeply equal { view: 'home', …(2) }`
- *«writes a move in an open named view…»* → `AssertionError: expected { view: 'home', …(1) } to deeply equal { view: 'home', …(3) }`
- *«shows one of the three by closing the named view…»* → `TypeError: layout.showView is not a function`
- *«saves the layout on screen under a new name…»* → `TypeError: layout.saveNamed is not a function`
- *«closes the named view when the core holds no package it can read…»* (**E75**) → `TypeError: layout.saveNamed is not a function`
- il dock, *«shows the named view that is open…»* → `AssertionError: expected [ 'activity', 'costs', …(5) ] to deeply equal [ 'status' ]`
- `Test Files  4 failed (4)`, **`Tests  7 failed | 28 passed (35)`** — verdi tutte le prove che c'erano.

**Passo 3** — `--only-step 3` → `3 operations, 0 refused` (`nearest.ts`, due sostituzioni in `moveActive.ts`).
`(cd gui && npx vitest run --project jsdom src/frame/nearest.test.ts src/frame/keys.test.ts)` → `Test Files  2 passed (2)`,
**`Tests  8 passed (8)`**.

**Passo 4** — `--only-step 4` → `1 operations, 0 refused`. `(cd gui && npx vitest run --project jsdom src/frame/schematic.test.ts)`
→ `Test Files  1 passed (1)`, **`Tests  2 passed (2)`**.

**Passo 5** — `--only-step 5` → `1 operations, 0 refused`, `rewrite gui/src/stores/layout.ts (LF)`; dopo, `CR=0` su 223
righe. `(cd gui && npx vitest run --project jsdom src/stores/stores.test.ts)` → **`Tests  20 passed (20)`**.

**Passo 6** — `--only-step 6` → `4 operations, 0 refused` (le tre di `dock.ts`, con la testa di **E78**, e quella di
`Frame.vue`). `(cd gui && npx vitest run --project jsdom src/frame/frame.test.ts)` → **`Tests  15 passed (15)`**.

**Passo 7** — `(cd gui && npm test && npm run build && npm run lint)`, poi altre quattro `npm test`, in fila, 40 s in tutto:

| | Uscita |
|---|---|
| `npm test`, corse 1–5 | ciascuna **`Test Files  27 passed \| 1 skipped (28)`** e **`Tests  212 passed \| 1 skipped (213)`**, uscita 0; durate 5,36–5,74 s. Nessuna caduta, P-19 compresa |
| `npm run build` | uscita 0, `dist/assets/index-v8xlIAVo.js  693.02 kB │ gzip: 211.20 kB` — il pezzo JavaScript dopo (**E80**); l'unico avviso è quello dei pezzi sopra i 500 kB (N-2 di E187) |
| `npm run lint` | uscita 0, nessuna riga oltre l'intestazione `eslint src` |

⚠️ `npm test` fuori da un terminale stampa il solo riepilogo complessivo, senza le righe per progetto: che le 27 contino i sei
file del browser lo dice la riga `Forced re-optimization of dependencies` (il progetto `browser`, **E24**) e lo conferma il
cancello, che lancia i due progetti uno alla volta (§6).

## 4. Il Passo 8 — le quindici violazioni

Prima della prima: `git status --porcelain > <scratchpad>/prima-8.txt` (i dieci file del compito) e le copie di
`nearest.ts`, `schematic.ts`, `layout.ts` e `dock.ts` in `<scratchpad>/copie/`, `cmp` uguale. Ogni violazione con
`violate.py` (una sostituzione unica, fine-riga conservati), poi `(cd gui && npx vitest run)` — **la suite intera, i due
progetti** —, poi la copia indietro con `cp` e `cmp`: **15 su 15** `restored: cmp equal`, e dopo l'ultima i quattro file
uguali alle copie, nessun `.tmp` rimasto. Il messaggio è la prima riga che nomina la ragione, copiata dal log.

| # | La violazione | Le prove che cadono, col messaggio vero | Riepilogo |
|---|---|---|---|
| 1 | `nearest.ts`: tolto `\|\| across(a.rect) - across(b.rect)` dal `sort` | *«goes to the card beyond…»* `AssertionError: r1c2 up: expected 'r0c0' to be 'r0c2'`; *«breaks a tie on the other axis…»* `expected 'r1c0' to be 'r1c1'` | `1 failed \| 26 passed \| 1 skipped (28)`, `Tests  2 failed \| 210 passed \| 1 skipped (213)` |
| 2 | `nearest.ts`: tolta `.filter(({ rect }) => beyond(rect))` | `keys.test.ts` *«splits its own group on that side when nothing lies there»* `expected 'moved' to be 'split'` (**E77**); *«goes to the card beyond…»* `r0c1 right: expected 'r0c0' to be 'r0c2'`; *«breaks a tie…»* `expected 'r0c1' to be 'r1c1'`; *«finds nothing beyond an edge…»* `expected { name: 'r1c0', …(1) } to be undefined` | `2 failed \| 25 passed`, `Tests  4 failed \| 208 passed` |
| 3 | `schematic.ts`: `const next = orientation;` | *«cuts the unit square…»* `expected [ …(3) ] to deeply equal [ …(3) ]`; *«tiles each view that ships…»* `home: the strip: expected false to be true` | `1 failed`, `Tests  2 failed \| 210 passed` |
| 4 | `schematic.ts`: dopo `place(…)`, `for (const floating of layout.floatingGroups ?? []) tiles.push({ x: 0, y: 0, width: 0, height: 0, views: floating.data.views });` | *«cuts the unit square…»* `expected [ …(4) ] to deeply equal [ …(3) ]` | `1 failed`, `Tests  1 failed \| 211 passed` |
| 5 | `layout.ts`: tolta `if (read.some((kept) => sameName(kept.name, name))) continue;` | *«drops what it cannot read…»* `expected { view: 'home', …(2) } to deeply equal { view: 'home', …(2) }` | `Tests  1 failed \| 211 passed` |
| 6 | `layout.ts`, `unpack`: `if (typeof open === "string") pack.openNamed = open;` | *«drops what it cannot read…»* `expected { view: 'home', …(3) } to deeply equal { view: 'home', …(2) }` | `Tests  1 failed \| 211 passed` |
| 7 | `layout.ts`, `settle`: `if (true) {` | *«writes a move in an open named view…»* `expected { view: 'home', …(3) } to deeply equal { view: 'home', …(3) }` | `Tests  1 failed \| 211 passed` |
| 8 | `layout.ts`, `receive`: tolta la riga di `openNamed` | il dock di `frame.test.ts` `expected [ 'activity', 'costs', …(5) ] to deeply equal [ 'status' ]` (**E77**); *«keeps the named views…»* `expected null to be 'Revisione'`; *«writes a move…»* `expected { view: 'home', …(2) } to deeply equal { view: 'home', …(3) }`; la prova di **E75** `expected 'Revisione' to be null` | `2 failed \| 25 passed`, `Tests  4 failed \| 208 passed` |
| 9 | `layout.ts`, `receive`: `if (saved.value !== null) openNamed.value = saved.value.openNamed ?? null;` | la prova di **E75** `expected 'Revisione' to be null` | `Tests  1 failed \| 211 passed` |
| 10 | `layout.ts`, `showView`: tolta `openNamed.value = null;` | il dock `expected [ 'status' ] to deeply equal [ 'activity', 'costs', …(5) ]` (**E77**); *«shows one of the three…»* `expected 'Revisione' to be null` | `2 failed \| 25 passed`, `Tests  2 failed \| 210 passed` |
| 11 | `layout.ts`, `saveNamed`: tolto `...shown,` da `taken` | *«saves the layout on screen…»* `expected 'saved' to be 'taken'` | `Tests  1 failed \| 211 passed` |
| 12 | `layout.ts`, `sameName`: `return a === b;` | *«drops what it cannot read…»* `expected { view: 'home', …(2) } to deeply equal { view: 'home', …(2) }`; *«saves the layout…»* `expected 'saved' to be 'taken'` | `1 failed`, `Tests  2 failed \| 210 passed` |
| 13 | `layout.ts`, `onScreen`: le due righe dell'`if` → `if (openNamed.value !== null) pack.openNamed = openNamed.value;` | *«shows one of the three…»* `expected { view: 'compact', layouts: {}, …(2) } to deeply equal { view: 'compact', …(2) }` | `Tests  1 failed \| 211 passed` |
| 14 | `dock.ts`: tolto `() => layout.openNamed,` dal `watch` | il dock `expected [ 'status' ] to deeply equal [ 'activity', 'costs', …(5) ]` | `Tests  1 failed \| 211 passed` |
| 15 | `dock.ts`, `apply`: `const chosen = undefined;` | il dock `expected [ 'activity', 'costs', …(5) ] to deeply equal [ 'status' ]` | `Tests  1 failed \| 211 passed` |

Ogni riga cade come la tabella detta, con le prove in più che **E77** nomina — nessuna di più, nessuna di meno. Ogni
`npx vitest run` è uscito 1.

**Alla fine**, due `diff`, perché il nome `prima.txt` vive in due momenti (vedi §7):

- `git status --porcelain | diff <scratchpad>/prima-8.txt -` → **vuoto**: nessun file nato dalle prove (vincolo 11);
- `git status --porcelain | diff <scratchpad>/prima.txt -`, col file del Passo 1 → soltanto i dieci file del compito,
  `M` `Frame.vue`, `dock.ts`, `frame.test.ts`, `moveActive.ts`, `layout.ts`, `stores.test.ts` e `??` `nearest.test.ts`,
  `nearest.ts`, `schematic.test.ts`, `schematic.ts` (Passo 8).

## 5. I fine-riga

Misurati con `tr -cd '\r' < <file> | wc -c`, `wc -l` e `git ls-files --eol`, prima di toccare (`eol-prima.txt`) e dopo
(`eol-dopo.txt`, e dopo `git add` per i nuovi):

| File | Prima | Dopo |
|---|---|---|
| `gui/src/stores/layout.ts` (riscritto) | CR 0 su 144 righe, `i/lf w/lf` | CR 0 su 223, `i/lf w/lf` |
| `gui/src/stores/stores.test.ts` | CR 0 su 195, `i/lf w/lf` | CR 0 su 303, `i/lf w/lf` |
| `gui/src/frame/moveActive.ts` | CR 0 su 63, `i/lf w/lf` | CR 0 su 53, `i/lf w/lf` |
| `gui/src/frame/dock.ts` | CR 0 su 152, `i/lf w/lf` | CR 0 su 157, `i/lf w/lf` |
| `gui/src/frame/Frame.vue` | CR 0 su 70, `i/lf w/lf` | CR 0 su 71, `i/lf w/lf` |
| `gui/src/frame/frame.test.ts` | CR 0 su 275, `i/lf w/lf` | CR 0 su 291, `i/lf w/lf` |
| `docs/superpowers/plans/2026-09-23-design-system.md` | CR 0 su 11210, `i/lf w/lf` | CR 0 su 11210, `i/lf w/lf` |
| `gui/src/frame/nearest.ts` (nuovo) | — | CR 0 su 40, `i/lf w/lf` |
| `gui/src/frame/nearest.test.ts` (nuovo) | — | CR 0 su 42, `i/lf w/lf` |
| `gui/src/frame/schematic.ts` (nuovo) | — | CR 0 su 56, `i/lf w/lf` |
| `gui/src/frame/schematic.test.ts` (nuovo) | — | CR 0 su 52, `i/lf w/lf` |

Su questa macchina ogni file toccato era ed è LF; i nuovi nascono LF; la forma non cambia. Mai `sed -i`.

## 6. Il cancello, e la posizione

**Passo 9** — la riga 7 della tabella della posizione: lo **Stato** da `⬜ — pre-controllato il 2026-09-27: sette voci, …:
viene l'esecuzione` a **`✅ 2026-09-27`**, con `row7.py` (nello scratchpad: tocca la sola ultima cella dell'unica riga che
comincia con `| **7** |`, e rifiuta se la forma non è quella attesa); la colonna **Commit** resta `—` (la scrive il compito 8,
**E79**); le righe 6 e 6bis non toccate.

Poi, uno alla volta:

- `bash scripts/gate.sh`, da solo, in background → **`GATE GREEN.`**, uscita 0, 77 s. Sotto `gui/`:
  - jsdom: **`Test Files  21 passed | 1 skipped (22)`**, **`Tests  156 passed | 1 skipped (157)`**;
  - browser: **`Test Files  6 passed (6)`**, **`Tests  56 passed (56)`**;
  - `dist/assets/index-v8xlIAVo.js  693.02 kB │ gzip: 211.20 kB`;
  - **`found 0 vulnerabilities`**.
- `bash scripts/check-docs.sh` → `OK — no inconsistencies.`, uscita 0.
- `git status --porcelain` → i soli undici file del compito (i dieci di `gui/` e il piano), nessun file nato dalle prove.

## 7. Le divergenze, e ciò che non ho misurato

**Nessuna voce d'errata candidata**: ogni Atteso dei Passi 1–8 è tornato com'è scritto, coi numeri del pre-controllo — la
baseline `691.82 kB`/`210.77 kB`, `Tests  7 failed | 28 passed (35)`, 8, 2, 20, 15, `212 passed | 1 skipped (213)` cinque
volte, `693.02 kB`/`211.20 kB`, e le quindici righe coi loro messaggi. `E82` resta libera.

Tre cose da sapere, che **non** sono difetti del compito:

1. **Il Passo 9 dice *«e `git push`»*** — e il vincolo 15 *«si committa e si pusha a ogni compito»* —; il prompt del dispaccio
   lo vieta all'implementatore: il push è del coordinatore, dopo la revisione. Ho seguito il dispaccio.
2. **`prima.txt` nomina due momenti:** il Passo 1 lo scrive all'inizio, il vincolo 11 e il §5 del dispaccio *«prima della
   prima violazione»*; e le due frasi *«alla fine»* sono vere ciascuna col suo momento — col file del Passo 1 il `diff`
   *«rende soltanto i file del compito»* (Passo 8), con quello di prima delle violazioni *«non rende nulla»* (vincolo 11). Ho
   tenuto i due file, `prima.txt` e `prima-8.txt`, e riportato i due `diff` (§4).
3. **La cella della riga 7:** l'ho portata a `✅ 2026-09-27` alla lettera del Passo 9, com'erano le righe 3, 4 e 6 nei commit
   dei loro implementatori; la frase del pre-controllo che c'era — *«pre-controllato il 2026-09-27: sette voci, E75…E81 …:
   viene l'esecuzione»* — è uscita con lei, e la sua traccia resta nell'errata (E75–E81) e in `fa1ae0e`. Se la chiusura la
   vuole nella cella, è del coordinatore.

**Non misurato da me:** la riga di `Frame.vue` che chiama `showView` senza una prova che la tenga — **E76**, dichiarata, e il
dispaccio dice di non aggiungere una prova: non ho rifatto la violazione; lo sguardo nel browser, che la regola 5 non chiede
per il compito 7; la CI, che è del coordinatore dopo il push; la macchina `zagor`.

## 8. Il tempo, e i log

- Inizio della lettura poco dopo le 17:10; Passo 1 alle 17:18:04; commit `0417c9c` alle 17:27:52 (+0200); questo rapporto
  alle 17:29. Il cancello del Passo 1 74 s, quello del Passo 9 77 s; le cinque corse del Passo 7 col *build* e il linter 40 s.
- I log del cancello, nello scratchpad `C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\e4b4f474-4c3b-4180-b8de-ae411307e17a\scratchpad\task7\`:
  - `gate-passo1-20260927-171820.log` — il cancello d'apertura, a `fa1ae0e`;
  - `gate-passo9-20260927-172537.log` — il cancello sull'albero del compito, prima del commit.
- Accanto: `passo1-build.log`, `passo2.log`…`passo6.log`, `passo7-test-1.log`…`passo7-test-5.log`, `passo7-build.log`,
  `passo7-lint.log`, `passo8-run.log` e `passo8-v1.log`…`passo8-v15.log`, `check-docs.log`, `eol-prima.txt`,
  `eol-dopo.txt`, `prima.txt`, `prima-8.txt`, `copie/`; gli strumenti `violate.py`, `passo8.sh`, `row7.py`,
  `replace_unique.py`; e `commit-msg.txt`, il messaggio passato a `git commit -F`.
