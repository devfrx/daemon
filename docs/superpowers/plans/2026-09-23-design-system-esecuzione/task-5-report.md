# Il rapporto dell'implementatore del compito 5 — il kit al lavoro

> Macchina `Jays`, 2026-09-26. Base `39827e8`. Lo scratchpad dei log e degli attrezzi:
> `C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\b62a978b-28b5-4b69-963c-b58323e34338\scratchpad\task5`
> (qui sotto `<SP>`).

## 1. Lo stato

**`DONE`** — commit **`545f500`** (`545f500703b2ccb92bfeac0cfc7add635803a1b6`), padre `39827e8`, senza co-autore, **non
pushato** (il push è del coordinatore, dopo la revisione). Nessuna divergenza dal testo del compito: nessuna voce
candidata `E38`. Il Passo 8 non è stato fatto (**E35**).

Le verifiche d'avvio, tutte tornate: `git rev-parse --short HEAD` → `39827e8`; `git status --porcelain` → vuoto;
`node --version` → `v24.19.0` (`engines`: `^22.22.2 || ^24.15.0 || >=26.0.0`); `git config --show-origin --get-all
core.autocrlf` → tre righe, `true` dal file di sistema, `false` da `C:/Users/Jays/.gitconfig`, `false` da `.git/config`;
Chrome dal nome della cartella → `154.0.8037.58` (uscita 2, la seconda cartella non c'è, com'è detto).

## 2. `git show --stat HEAD`

```
commit 545f500703b2ccb92bfeac0cfc7add635803a1b6
Author: devfrx <zagor2012@icloud.com>
Date:   Sat Sep 26 14:15:47 2026 +0200

 docs/superpowers/plans/2026-09-23-design-system.md |  2 +-
 gui/eslint.config.js                               | 21 +++++
 gui/src/a11y.test.ts                               |  2 +-
 gui/src/components/Confirm.vue                     | 78 +++++++------------
 gui/src/frame/Band.vue                             | 38 +++++----
 gui/src/frame/Drawer.vue                           | 66 ++++++----------
 gui/src/frame/ViewBar.vue                          | 16 ++--
 gui/src/frame/frame.test.ts                        | 14 ++++
 gui/src/locales/copy.test.ts                       | 22 +++++-
 gui/src/locales/it.json                            |  8 +-
 gui/src/panels/Permissions.vue                     | 28 +++----
 gui/src/panels/Placeholder.vue                     |  3 +-
 gui/src/panels/Settings.vue                        | 87 ++++++++++-----------
 gui/src/panels/Status.vue                          | 14 ++--
 gui/src/panels/Steps.vue                           | 31 ++++----
 gui/src/panels/modules.test.ts                     | 90 +++++++++++++++-------
 gui/src/panels/settings.browser.test.ts            | 61 +++++++++++++++
 gui/src/tokens/dock.css                            |  5 +-
 18 files changed, 357 insertions(+), 229 deletions(-)
```

I diciassette file del punto 2 del contratto più il piano, e nient'altro: `git status --porcelain` prima del commit
nominava solo quelli, e dopo è vuoto. `git diff --stat 39827e8..HEAD -- crates/ gui/schema/` → vuoto (vincolo 12).

⚠️ **Come ho scritto il testo dettato**: mai ribattuto a mano. Ogni recinto è stato preso **dal brief** con `sed -n` sulle
sue righe (il brief è la copia parola per parola del piano a `39827e8`), e scritto con gli attrezzi dello scratchpad —
`replace_unique.py` del piano per le *Trova/Sostituisci*, `splice_block.py` per il blocco di Impostazioni preso coi due
ancoraggi, `rewrite_whole.py` (Python `newline=""`, temporaneo più `os.replace`, il terminatore del file com'era) per i
sette file riscritti. Dopo ogni riscrittura, `cmp` fra il recinto e il file: **uguali tutti e sette**. Un solo inciampo,
mio e senza effetto: al primo tentativo su `it.json` ho preso le righe del brief sfasate di una, e `replace_unique.py` ha
rifiutato, `refused: 0 occurrences`, senza scrivere; ripreso con le righe giuste.

## 3. I passi, coi comandi e le uscite vere

### Passo 1 — le prove che cambiano, prima del codice

- `python <SP>/retarget_confirm.py <root>` → `ok` su `gui/src/panels/modules.test.ts` e `gui/src/a11y.test.ts` (i conteggi
  2, 4 e 1 tornati; sei `'[role="dialog"]'` in `modules.test.ts`, uno in `a11y.test.ts`).
- Il blocco `describe("Impostazioni", …)` preso dal file coi due ancoraggi: `lines 137-199 replaced by 87`.
- `useLayout` negli import; la prova di M-3 di Stato in coda al suo `describe`; quella della fascia in `frame.test.ts`;
  `gui/src/panels/settings.browser.test.ts` creato **LF**, 61 righe, col `⛔ NON-VACUITY` di **E37** alla riga 44.

`(cd gui && npx vitest run --project jsdom src/panels/modules.test.ts src/a11y.test.ts src/frame/frame.test.ts)` → uscita 1:

```
 Test Files  2 failed | 1 passed (3)
      Tests  8 failed | 29 passed (37)
```

| Prova rossa | Il messaggio |
|---|---|
| `frame.test.ts > the band > keeps its status region while connected, …(M-3 of E187)` | `Error: Unable to get [role="status"] within: <!--v-if-->` |
| `modules.test.ts > Stato > keeps the event's status region before a Verdict, …(M-3 of E187)` | `Error: Unable to get [role="status"] within: <section … class="status">` |
| `Impostazioni > is off until the core has said which policy is active` | `AssertionError: expected [] to have a length of 2 but got +0` |
| `Impostazioni > sends Invoke with the registry's literals on a change, …` | `AssertionError: expected undefined to be 'true'` |
| `Impostazioni > keeps its status region before an Invoke, …(M-3 of E187)` | `Error: Unable to get [role="status"] within: <section … class="settings">` |
| `Impostazioni > leaves the control on the core's policy until the core answers, …` | `AssertionError: expected [] to deeply equal [ 'true', 'false' ]` |
| `Impostazioni > keeps saying a call is in flight after the yes, …` | `AssertionError: expected false to be true` (il `approve()`: nessun radio da cliccare, nessuna chiamata in volo) |
| `Impostazioni > chooses the theme, which the layout package keeps at once …` | `AssertionError: expected [] to deeply equal [ 'true', 'false', 'false' ]` |

Verdi, dalla stessa corsa col reporter `verbose`: `the confirmation window > opens only when the core asked AND a call is in
flight, and Approve carries the call` e `… > sends nothing on a no, and closes`, e le undici di `a11y.test.ts`, fra cui
*«the confirmation window, open, has no violation»*.

`(cd gui && npx vitest run --project browser src/panels)` → uscita 1, `Tests  1 failed (1)`:
`settings.browser.test.ts > asks the core on an arrow key as on a click, …` → `AssertionError: expected [] to have a length
of 2 but got +0`.

Tutto come l'Atteso, e come i riferimenti del pre-controllo.

### Passo 2 — le parole del tema

Le quattro sostituzioni in `copy.test.ts`, `ok` ciascuna. `(cd gui && npx vitest run --project jsdom src/locales)`:

| | Uscita |
|---|---|
| rosso, prima delle parole | `AssertionError: system: expected [] to include 'system'` — `Tests  1 failed \| 2 passed (3)` |
| verde, con la sostituzione in `it.json` (e `JSON.parse` del file riuscito) | `Tests  3 passed (3)` |
| direzione rossa: in `tokens/theme.ts` `["system", "light", "dark", "sepia" as ThemeChoice]` | `AssertionError: sepia: expected [ 'system', 'light', 'dark' ] to include 'sepia'` — `Tests  1 failed \| 2 passed (3)` |

Prima della violazione `git status --porcelain > <SP>/prima-p2.txt` e la copia di `theme.ts`; dopo, la copia tornata,
`cmp` uguale, `git status --porcelain | diff <SP>/prima-p2.txt -` vuoto.

### Passi 3, 4 e 5 — la conferma e il cassetto, le regioni di stato, Impostazioni

`Confirm.vue`, `Drawer.vue`, `Band.vue` e `Settings.vue` riscritti per intero, `cmp` col recinto uguale; in `Status.vue`
la riga dell'evento dentro `BaseStatus` e il suo import; in `gui/src/tokens/dock.css` la sostituzione di **E36**,
applicata una volta. Nessuna corsa dettata fra questi passi: la prova è la corsa del Passo 6.

### Passo 6 — Permessi, Passi, il segnaposto, la barra

`Permissions.vue`, `Steps.vue` e `ViewBar.vue` riscritti, `cmp` uguale; in `Placeholder.vue` le due sostituzioni.

`(cd gui && npm test && npm run build)` → uscita 0:

```
 Test Files  22 passed | 1 skipped (23)
      Tests  158 passed | 1 skipped (159)
dist/assets/index-BIBdFfwk.css                             151.78 kB │ gzip:  15.09 kB
dist/assets/index-BS90oxvQ.js                              689.58 kB │ gzip: 209.99 kB
```

Il pezzo JavaScript: **`663.93 kB` → `689.58 kB`**, lo stesso nome del pre-controllo, `index-BS90oxvQ.js`. Nel log nessun
avviso oltre a quello noto dei pezzi sopra i 500 kB (N-2 di E187).

**La direzione rossa di M-3**, un pannello per volta, con `npm test` intero ogni volta; prima `<SP>/prima-p6.txt` e le copie
dei tre file:

| Pannello | La violazione | Uscita |
|---|---|---|
| `gui/src/frame/Band.vue` | `<BaseStatus v-if="connection.phase !== 'connected'">` e `<div class="band">` | uscita 1, `Tests  1 failed \| 157 passed \| 1 skipped (159)`: la sola `frame.test.ts > the band > keeps its status region while connected, …`, `Error: Unable to get [role="status"] within: <!--v-if-->` |
| `gui/src/panels/Settings.vue` | `<BaseStatus v-if="invoke.inFlight !== null">` e `<p>` | uscita 1, `Tests  1 failed \| 157 passed \| 1 skipped (159)`: la sola `Impostazioni > keeps its status region before an Invoke, …`, `Error: Unable to get [role="status"] within: <section … class="settings">` |
| `gui/src/panels/Status.vue` | `<BaseStatus v-if="core.lastVerdict !== null">` e `<p class="event">` | uscita 1, `Tests  1 failed \| 157 passed \| 1 skipped (159)`: la sola `Stato > keeps the event's status region before a Verdict, …`, `Error: Unable to get [role="status"] within: <section … class="status">` |

Dopo ciascuna la copia tornata, `cmp equal`; alla fine `git status --porcelain | diff <SP>/prima-p6.txt -` vuoto.

**La prova della tastiera, stabile**: `npx vitest run --project browser src/panels` **10 su 10** verdi, `Tests  1 passed (1)`;
`npm test` intero **5 su 5** verdi, `Tests  158 passed | 1 skipped (159)`. Nessuna caduta. Log in `<SP>/stability/`.

## 4. Il Passo 7 — la tabella

Il blocco `harness/panels-and-frame` in coda a `gui/eslint.config.js`; `(cd gui && npm run lint)` → uscita 0. Poi,
prima della prima violazione, `<SP>/prima-p7.txt` e le copie di `Strip.vue`, `ViewBar.vue` e `moveActive.ts`:

| Riga | La violazione | `npm run lint` | Errori | Ritorno |
|---|---|---|---|---|
| 1 | in `gui/src/panels/Strip.vue`, sotto `<div class="strip">`, `<button type="button"></button>` | uscita 1 — `12:5  error  a button is BaseButton (design system, section (b))  vue/no-restricted-html-elements` | `✖ 1 problem (1 error, 0 warnings)` | copia, `cmp` uguale |
| 2 | in `gui/src/frame/ViewBar.vue`, sotto `<header class="bar">`, `<ul><li></li></ul>` | uscita 1 — `17:5  error  a list is BaseList (design system, section (b))  vue/no-restricted-html-elements` | `✖ 1 problem (1 error, 0 warnings)` | copia, `cmp` uguale |
| 3 | in `gui/src/frame/moveActive.ts`, sotto la riga 1, `import { DialogRoot } from "reka-ui";` | uscita 1 — `2:1  error  'reka-ui' import is restricted from being used. panels and frame use the base pieces, which sit on reka-ui (design system, section (b))  no-restricted-imports` | `✖ 1 problem (1 error, 0 warnings)` | copia, `cmp` uguale |
| 4 | niente: `Confirm.vue` su `BaseDialog`, `BaseDialog.vue` che importa `reka-ui` | uscita 0 | 0 | — |

Alla fine `git status --porcelain | diff <SP>/prima-p7.txt -` → vuoto (`git status as before`). Le righe 1 e 2 con **un
errore solo**, com'è **E34**. Lo script: `<SP>/lint-red.sh`; i log `<SP>/p7-lint-row{1,2,3,4}.log`.

## 5. I fine-riga

Misurati con `tr -cd '\r' < f | wc -c`, `wc -l` e `git ls-files --eol` (`<SP>/eol-before.txt`, `<SP>/eol-after.txt`). Su
questa macchina tutti `i/lf w/lf` prima e dopo: CR **zero** in ogni file, prima e dopo, e la colonna `w/lf` invariata.

| File | Prima: CR / righe | Dopo: CR / righe |
|---|---|---|
| `gui/src/components/Confirm.vue` | 0 / 74 | 0 / 50 |
| `gui/src/frame/Drawer.vue` | 0 / 58 | 0 / 36 |
| `gui/src/frame/Band.vue` | 0 / 36 | 0 / 42 |
| `gui/src/frame/ViewBar.vue` | 0 / 65 | 0 / 69 |
| `gui/src/panels/Settings.vue` | 0 / 83 | 0 / 84 |
| `gui/src/panels/Permissions.vue` | 0 / 45 | 0 / 47 |
| `gui/src/panels/Steps.vue` | 0 / 43 | 0 / 46 |
| `gui/src/panels/Placeholder.vue` | 0 / 39 | 0 / 40 |
| `gui/src/panels/Status.vue` | 0 / 61 | 0 / 65 |
| `gui/src/locales/it.json` | 0 / 129 | 0 / 135 |
| `gui/src/locales/copy.test.ts` | 0 / 51 | 0 / 65 |
| `gui/src/panels/modules.test.ts` | 0 / 249 | 0 / 285 |
| `gui/src/a11y.test.ts` | 0 / 114 | 0 / 114 |
| `gui/src/frame/frame.test.ts` | 0 / 203 | 0 / 217 |
| `gui/src/panels/settings.browser.test.ts` | — (nuovo) | 0 / 61; dopo il commit `i/lf w/lf` |
| `gui/eslint.config.js` | 0 / 150 | 0 / 171 |
| `gui/src/tokens/dock.css` | 0 / 77 | 0 / 78 |
| `docs/superpowers/plans/2026-09-23-design-system.md` | 0 / 8974 | 0 / 8974 |

⚠️ Per leggere la colonna del file nuovo prima del commit ho dato `git add -N` (intent-to-add): tocca solo l'indice, e il
file è entrato nel commit comunque.

## 6. Il cancello

| | All'apertura, a `39827e8` | Prima del commit |
|---|---|---|
| log | `<SP>/gate-open-20260926-140058.log`, 14:00:58–14:02:10 | `<SP>/gate-commit-20260926-141358.log`, 14:13:58–14:15:16 |
| `jsdom` | `Test Files  18 passed \| 1 skipped (19)`, `Tests  125 passed \| 1 skipped (126)` | `Test Files  18 passed \| 1 skipped (19)`, `Tests  130 passed \| 1 skipped (131)` |
| `browser` | `Test Files  3 passed (3)`, `Tests  27 passed (27)` | `Test Files  4 passed (4)`, `Tests  28 passed (28)` |
| pezzo JavaScript | `index-CiZv4zPX.js 663.93 kB │ gzip: 201.53 kB` | `index-BS90oxvQ.js 689.58 kB │ gzip: 209.99 kB` |
| `npm audit` | `found 0 vulnerabilities` | `found 0 vulnerabilities` |
| esito | `GATE GREEN.` | `GATE GREEN.` |

I conti tornano: cinque prove nuove sotto jsdom — due di Impostazioni (M-3 e il tema), M-3 di Stato, M-3 della fascia, le
parole del tema — e una nel browser. Poi `bash scripts/check-docs.sh` → `OK — no inconsistencies.`, da solo, dopo il
cancello.

## 7. La SPA

L'ho guardata, e non è il Passo 8: `(cd gui && npm run dev -- --port 5173 --strictPort)`, la pagina nel pannello Browser
dell'app, vista emulata 1440 × 900, Chromium del pannello — non il Chrome delle prove. Ciò che ho guardato:

1. **Al caricamento, tema scuro:** la barra coi tre pulsanti `quiet` delle viste, la ricerca spenta con l'icona e le sue
   parole, il chip, «+ moduli»; la fascia *«Il core non ha risposto.»* con «Riprova»; Permessi coi titoli di `BaseLabel` e
   le icone; Impostazioni con «POLICY VRAM» spenta e «TEMA» su «Sistema». Le tre `.base-status` nel DOM, vuote; in
   pagina c'è anche una quarta `role="status"`, la `dv-live-region` di `dockview`, che non è nostra.
2. **La sequenza del Passo 8, senza l'Assistente vocale:** `Accepted` → la regione della fascia vuota; `StaleBuild` → **lo
   stesso elemento** con *«Il core parla una versione diversa del protocollo. La finestra non procede. Timbro atteso: …»*;
   `Policy` e un clic vero sull'etichetta «Locale» → `Invoke { function: "vram-policy", argument: "local" }`, la spunta
   ferma su «OpenRouter», nella regione di Impostazioni *«Richiesta inviata: in attesa del core.»*; `PermissionRequired` →
   la finestra di conferma aperta, il fuoco dentro, «Rifiuta» `quiet` e «Consenti» `primary`, guardata nel tema scuro; clic
   su «Consenti» → `Approve`, la finestra chiusa, la riga ancora in volo; `Policy` → la riga via; `Verdict` → **lo stesso
   elemento** di Stato con *«Ultima richiesta di VRAM: rifiutata chiesti 4096 MiB, tetto 1024»*. Che si **sentano** resta
   al proprietario.
3. **Il tema:** un clic su «Chiaro» → `data-theme="light"` e un `SaveLayout` col pacchetto `{ view: "home", layouts: { home
   }, theme: "light" }`; guardato nel chiaro: i pannelli, e il cassetto a foglio dal basso con «I MODULI» e le righe di
   `BaseList`; Esc lo chiude e il fuoco torna su «+ moduli».
4. **La freccia sul gruppo del tema:** col tasto del pannello, premuto e rilasciato insieme, il fuoco va su «Scuro» e non
   si sceglie niente — il comportamento di P-19 di `reka-ui` 2.10.4, non un difetto; con `keydown` e `keyup` sintetici a
   80 ms l'uno dall'altro, la freccia da «Chiaro» sceglie «Scuro», `data-theme="dark"`, la spunta su «Scuro».
5. **La console:** le sole due righe di debug di `[vite]`, nessun avviso.

Il server fermato **per PID** prima del cancello: `Stop-Process` su `35348`, poi nulla in ascolto su 5173 e il processo
sparito; il pannello chiuso.

## 8. Le divergenze, e ciò che non ho potuto misurare

**Nessuna voce candidata.** Ogni Atteso è tornato, coi messaggi dei riferimenti del pre-controllo.

Tre **osservazioni**, che non smentiscono nessuna riga del compito — per chi rivede:

1. In `Settings.vue` la `BaseStatus` vuota sta in una colonna flex con `gap: var(--space-4)`: è alta zero, ma i suoi due
   `gap` restano, e fra il gruppo della policy e quello del tema lo spazio è doppio finché nessuna chiamata è in volo —
   visto negli screenshot, e dedotto dal CSS. Nella fascia no: `.frame` di `Frame.vue` è una colonna flex **senza** `gap`.
2. Cresce anche il foglio CSS: `index-Bx6RaF8P.css 145.10 kB` → `index-BIBdFfwk.css 151.78 kB`, dai due log del cancello.
3. Un clic dentro un pannello del dock manda un `SaveLayout` (il gruppo attivo cambia): visto nella SPA, ed è della parte 2,
   non di questo compito.

**Non misurato da qui:** l'Assistente vocale (Passo 8, del proprietario col coordinatore, **E35**); la CI (del
coordinatore, dopo il push); la macchina `zagor`.

## 9. Il tempo, e i log

Dalle verifiche d'avvio, poco prima delle 14:00, al commit delle 14:15:47: circa venti minuti, più il rapporto. Due corse
del cancello, 72 s e 78 s.

- Il cancello: `<SP>/gate-open-20260926-140058.log` e `<SP>/gate-commit-20260926-141358.log`.
- `check-docs.sh`: `<SP>/check-docs.log`.
- Le corse dei passi: `<SP>/p1-jsdom.log`, `<SP>/p1-jsdom-verbose.log`, `<SP>/p1-browser.log`, `<SP>/p2-red.log`,
  `<SP>/p2-green.log`, `<SP>/p2-viol.log`, `<SP>/p6-test-build.log`, `<SP>/p6-m3-{band,settings,status}.log`,
  `<SP>/p7-lint-green.log`, `<SP>/p7-lint-row{1,2,3,4}.log`, `<SP>/stability/`.
- Il messaggio del commit: `<SP>/commit-msg.txt`.
