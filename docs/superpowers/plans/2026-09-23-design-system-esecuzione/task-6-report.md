# Il rapporto dell'implementatore — compito 6, il dock vestito

## 1. Lo stato

**`DONE_WITH_CONCERNS`** — commit **`c1102fc`**, non pushato (il push è del coordinatore, dopo la revisione).

Il codice è quello dettato: `python docs/superpowers/plans/2026-09-23-design-system-esecuzione/compare_task6.py 1a83208 HEAD`
→ 16 righe `OK`, *«16 paths, 0 mode changes, 0 not matching the plan's text; date 2026-09-26»*, uscita **0**. La
preoccupazione è di **sola errata**: una voce candidata, **E49** (nit), due righe del Passo 7 che fanno cadere più prove di
quelle che l'Atteso nomina (§8); e un'osservazione sul controllo 15, non una divergenza (§8).

Verifiche d'avvio, macchina `Jays`: `git rev-parse --short HEAD` → `1a83208`; `git status --porcelain` → vuoto;
`node --version` → `v24.19.0`, dentro `^22.22.2 || ^24.15.0 || >=26.0.0`; `git config --show-origin --get-all
core.autocrlf` → `true` dal file di sistema, `false` dal file dell'utente, `false` da `.git/config` — vale l'ultima, `false`;
Google Chrome `154.0.8037.58`, letto dal nome della cartella in `/c/Program Files/Google/Chrome/Application/` (il comando esce
2 perché `$LOCALAPPDATA/Google/Chrome/Application/` non esiste). Il brief a `HEAD` = `1a83208`, *«piano e disegno coincidono
con `HEAD`»*, letto per intero.

## 2. `git show --stat HEAD`

```
c1102fc design-system(compito 6): il dock vestito -- il tema nostro, le schede, la presa grande coi …

 docs/superpowers/plans/2026-09-23-design-system.md |   4 +-
 gui/eslint.config.js                               |   2 +-
 gui/src/frame/BigTab.ts                            |  68 +++++----
 gui/src/frame/BigTabFace.vue                       |  40 ++++++
 gui/src/frame/Frame.vue                            |   4 +
 gui/src/frame/bigtab.test.ts                       |  31 ++--
 gui/src/frame/dock.browser.test.ts                 | 133 ++++++++++++++++++
 gui/src/frame/dock.ts                              |  41 +++++-
 gui/src/frame/frame.test.ts                        |  37 ++++-
 gui/src/jsdom-setup.ts                             |  13 ++
 gui/src/testing/probes.browser.test.ts             |  40 +++++-
 gui/src/testing/probes.ts                          |  24 +++-
 gui/src/tokens/dock.css                            | 156 ++++++++++++++-------
 gui/src/tokens/dock.test.ts                        |  45 ++++++
 gui/src/tokens/readToken.ts                        |  14 ++
 gui/src/tokens/tokens.browser.test.ts              |  11 ++
 16 files changed, 555 insertions(+), 108 deletions(-)
```

Autore `devfrx`, **nessun co-autore** (`git log -1 --format=%B | grep -ci co-authored` → 0). Il messaggio comincia con
`design-system(compito 6): ` e chiude con le due righe del pezzo JavaScript, prima e dopo (N-2). `git diff --stat
1a83208..HEAD -- crates/ gui/schema/` → vuoto (vincolo 12); nessun tocco a `.github/`, `scripts/`, `docs/adr/`,
`gui/package.json`, `gui/package-lock.json`.

Il metodo di scrittura: i recinti del compito copiati **dal brief a macchina** (uno script dello scratchpad che scrive ogni
recinto del compito in un file, 45 recinti), i file nuovi e i riscritti copiati da lì, le sostituzioni con
`replace_unique.py` — ogni *Trova* contato prima, una occorrenza ciascuno.

## 3. I passi, coi comandi e le uscite vere

**Passo 1.** `git status --porcelain > <scratchpad>/prima.txt` → 0 byte. `grep -n 'themeAbyss' gui/src/frame/dock.ts` →
righe 1 (l'import) e 56 (`theme: themeAbyss,`). In `dockview.css` 8.3.1 le due ridefinizioni su sé stesse, righe 1037
(`.dv-resize-container`) e 1203 (`.dv-render-overlay`), `--dv-overlay-z-index: var(--dv-overlay-z-index, 999);`. In
`dockview-core` 8.3.1 la riga 12023, dentro `var AriaLevelTracker = class` (riga 12008):
`` this._orderedList[i].style.zIndex = `calc(var(--dv-overlay-z-index, 999) + ${i * 2})`; ``. Il *build*:
`dist/assets/index-COUloyjC.js 689.65 kB │ gzip: 210.02 kB` — la baseline. Poi, da solo, `bash scripts/gate.sh` →
`GATE GREEN`, 64 s: jsdom `Test Files  18 passed | 1 skipped (19)`, `Tests  132 passed | 1 skipped (133)`; browser `Test
Files  4 passed (4)`, `Tests  28 passed (28)`; `found 0 vulnerabilities` — i numeri del §3 del dispaccio.

**Passo 2.** Le due sostituzioni in `tokens.browser.test.ts`; `npx vitest run --project browser
src/tokens/tokens.browser.test.ts` → **rosso**, `Error: Failed to import test file
E:/ALL/DEV/MY_REPOS/daemon/gui/src/tokens/tokens.browser.test.ts` (Vite: *«Failed to resolve import "./readToken"»*),
`Test Files  1 failed (1)`, `Tests  no tests`. Creato `readToken.ts`; la stessa corsa → **verde**, `Tests  6 passed (6)`.

**Passo 3.** Creati `dock.test.ts` e `dock.browser.test.ts`, le due sostituzioni in `frame.test.ts` e le due in
`probes.browser.test.ts`.

- `npx vitest run --project jsdom src/tokens/dock.test.ts src/frame/frame.test.ts` → `Test Files  2 failed (2)`,
  `Tests  3 failed | 12 passed (15)`:
  - `dock.test.ts > our dockview theme > sets every variable the reference theme sets, but the tab groups' and the
    palette's` — `AssertionError: expected [ …(43) ] to deeply equal []` (la guardia verde);
  - `frame.test.ts > the dock > wears our theme, and hands it to dockview again when the theme on screen changes …` —
    `AssertionError: expected false to be true`;
  - `frame.test.ts > the dock > refuses a gap that is not a length in px, and builds the theme from the token that is` —
    `AssertionError: expected [Function] to throw error matching /not a length in px/ but got '(0 ,
    __vite_ssr_import_13__.harnessTh…'`.
- `npx vitest run --project browser src/frame/dock.browser.test.ts src/testing/probes.browser.test.ts` → `Test Files  2
  failed (2)`, `Tests  10 failed | 5 passed (15)`:
  - nei due temi *«draws every group as a card, `--space-3` from its neighbours»* — `expected '0px' to be '20px'`;
  - nei due temi *«keeps every radius of its own concentric, a floating group's too (answer 4)»* — `expected '0px' to be
    '20px'`;
  - nei due temi *«keeps the grab as tall as its strip, `--size-control-lg` (move 5 of SP-8, E45)»* — `expected [ 27, 27,
    27, 27, 27 ] to deeply equal []`;
  - nei due temi *«floats a group BELOW the dialogs …»* — `expected 999 to be 50`;
  - *«reads the radius a corner is DRAWN with …(E46)»* — `expected { near: 2, bad: [ …(2) ] } to deeply equal { near: 2,
    bad: [] }`;
  - *«does not judge what reaches the corner through a box that scrolls …(E44)»* — `expected { near: 1, …(1) } to deeply
    equal { near: +0, bad: [] }`;
  - verde la pillola, *«still shrinks the radii where CSS does …»*, e le quattro prove di prima della sonda.

**Passo 4.** `dock.css` e `jsdom-setup.ts` riscritti, le tre sostituzioni in `dock.ts`, quella di `Frame.vue`, le tre di
`probes.ts`. `npx vitest run --project jsdom src/tokens src/frame` → `Test Files  8 passed (8)`, `Tests  37 passed (37)`;
`npx vitest run --project browser src/frame src/tokens src/testing` → `Test Files  3 passed (3)`, `Tests  21 passed (21)`
— le prove di `bigtab.test.ts` di prima ancora verdi. Nessun avviso nei due log: nel browser solo la riga *«Forced
re-optimization of dependencies»*, l'`optimizeDeps: { force: true }` di E24.

**Passo 5.** `bigtab.test.ts` riscritto; `npx vitest run --project jsdom src/frame/bigtab.test.ts` → `Tests  3 failed (3)`:
`expected undefined to be 'Stato'`, `expected [ undefined, undefined ] to deeply equal [ 'float', 'fullPage' ]`,
`TypeError: tab.dispose is not a function`. Creato `BigTabFace.vue`, riscritto `BigTab.ts`, la sostituzione in
`eslint.config.js`; `npx vitest run --project jsdom src/frame/bigtab.test.ts && npm run lint` → `Tests  3 passed (3)`, e
`eslint src` esce 0 senza una riga.

**Passo 6.** `npm test && npm run build && npm run lint` → uscita 0, 10 s: `Test Files  24 passed | 1 skipped (25)`,
`Tests  177 passed | 1 skipped (178)` — due file più del compito 5 (19 + 4 = 23 file, più lo saltato), `dock.test.ts` e
`dock.browser.test.ts`. Il *build*: `dist/assets/index-DX-fK1-d.js 690.57 kB │ gzip: 210.40 kB` (il foglio CSS
`index-BLRkCvga.css 154.43 kB`). Il pezzo JavaScript da **689.65** a **690.57 kB**, compresso da 210.02 a 210.40: +0,92 kB,
lo stesso scarto che il piano misurava il 2026-09-23 da 689,58 a 690,50. Il linter esce 0. Nel log solo l'avviso noto dei
pezzi sopra i 500 kB (N-2 di E187).

**Stabilità** (P-19, D23): `npx vitest run --project browser src/frame/dock.browser.test.ts
src/testing/probes.browser.test.ts` **10 corse su 10** verdi, `Tests  15 passed (15)` ciascuna; `npx vitest run` intero
**5 su 5**, `Tests  177 passed | 1 skipped (178)`; e nelle venti corse intere del Passo 7 nessuna prova è caduta fuori da
quelle che la violazione tocca. Nessuna caduta da riportare.

## 4. Il Passo 7 — le due direzioni

Prima della prima violazione `git status --porcelain > <scratchpad>/prima-violazioni.txt` (i 15 file del compito) e la
copia degli otto file che le violazioni toccano. Ogni violazione con `replace_unique.py` (il testo vecchio unico, contato
prima: 20 su 20), poi `npx vitest run --reporter=verbose` — i **due progetti interi**, per sapere quali prove cadono —,
poi la copia torna e `cmp` la conferma. Il messaggio è la prima riga che nomina la ragione.

| # | La prova | La violazione | Il rosso vero | Le prove che cadono | Il conto | `cmp` |
|---|---|---|---|---|---|---|
| 1 | `dock.test.ts`, P-6 | tolta `--dv-overlay-z-index: var(--z-floating);`, l'ultima del blocco del tema | `expected [ '--dv-overlay-z-index' ] to deeply equal []` | jsdom, *«sets every variable the reference theme sets …»* | `1 failed \| 176 passed \| 1 skipped (178)` | uguale |
| 2 | `dock.test.ts`, la guardia | `".dockview-theme-abyss-that-is-not"` | `expected 0 to be greater than 40` | la sola guardia, *«sees the reference theme it replaces»* — la prova di P-6, senza niente da confrontare, **verde** | `1 failed \| 176 passed` | uguale |
| 3 | `bigtab.test.ts`, l'icona | `icon: undefined,` | `expected null not to be null` | *«shows a module's Italian name with the module's icon …»* | `1 failed \| 176 passed` | uguale |
| 4 | `bigtab.test.ts`, il trascinamento | tolto `@pointerdown.stop` dal primo pulsante | `expected 2 to be 1 // Object.is equality` | *«carries two named commands of the kit …»* | `1 failed \| 176 passed` | uguale |
| 5 | `bigtab.test.ts`, lo smontaggio | tolta `this.app?.unmount();` | `expected 1 to be +0 // Object.is equality` | *«takes its face away when dockview disposes of the tab»* | `1 failed \| 176 passed` | uguale |
| 6 | `frame.test.ts`, il tema che segue | tolta la riga del `watch(shownTheme, …)` | `expected [] to deeply equal [ 'light', 'dark' ]` | *«wears our theme, and hands it to dockview again …»* | `1 failed \| 176 passed` | uguale |
| 7 | `frame.test.ts`, il `gap` in px | `return Number.parseFloat(value);` al posto delle due righe | `expected [Function] to throw an error` | *«refuses a gap that is not a length in px …»* | `1 failed \| 176 passed` | uguale |
| 8 | i token sotto jsdom | tolta `document.head.append(sheet);` | `Error: the token --space-3 is not defined here: are the token sheets loaded?`, cinque volte | le **cinque** di `frame.test.ts` che montano il dock o leggono il `gap`: *«shows a saved view …»*, *«saves nothing when the window closes …»*, *«leaves the strip's params alone …»*, *«wears our theme …»*, *«refuses a gap …»* | `5 failed \| 172 passed` | uguale |
| 9 | `tokens.browser.test.ts`, `readToken` | tolta la riga dell'`if` | `expected [Function] to throw an error` | *«reach TypeScript through `readToken` …»* | `1 failed \| 176 passed` | uguale |
| 10 | `dock.browser.test.ts`, le schede | tolta `border-radius: var(--radius-card);` da `.dv-groupview` | `expected '0px' to be '20px'` | *«draws every group as a card …»*, chiaro e scuro | `2 failed \| 175 passed` | uguale |
| 11 | `dock.browser.test.ts`, la distanza | `gap: 0 };` | `expected [ Array(10) ] to deeply equal []`, le distanze a 0; ⚠️ e sotto jsdom `expected [Function] to throw an error` | *«draws every group as a card …»*, chiaro e scuro — ⚠️ **e** jsdom `frame.test.ts`, *«refuses a gap that is not a length in px …»*: **E49 (a)** | `3 failed \| 174 passed` | uguale |
| 12 | `dock.browser.test.ts`, i raggi | `.dockview-theme-harness .dv-tabs-and-actions-container { border-radius: var(--radius-control); }` prima di *«The mark of the visible tab»* | `expected [ …(8) ] to deeply equal []`, all'ultima attesa (riga 103): `dv-tabs-and-actions-container in dv-groupview, top-left: radius 8.0, outer 20.0, distance 1.0/1.0` e il suo `top-right`, per quattro gruppi | *«keeps every radius of its own concentric …»*, chiaro e scuro | `2 failed \| 175 passed` | uguale |
| 13 | `dock.browser.test.ts`, il livello | tolta la riga del contenitore galleggiante | `expected 999 to be 50 // Object.is equality` | *«floats a group BELOW the dialogs …»*, chiaro e scuro | `2 failed \| 175 passed` | uguale |
| 14 | `dock.browser.test.ts`, la presa | tolte le due righe del `padding` | `expected [ 32, 32, 32, 32, 32 ] to deeply equal []` | *«keeps the grab as tall as its strip …»*, chiaro e scuro | `2 failed \| 175 passed` | uguale |
| 15 | `dock.browser.test.ts`, il raggio del contenitore | tolta `border-radius: var(--radius-card);` dal contenitore | `expected '0px' to be '20px' // Object.is equality` | *«keeps every radius of its own concentric …»*, chiaro e scuro | `2 failed \| 175 passed` | uguale |
| 16 | `dock.browser.test.ts`, la barra del titolo | tolta la regola `… > .dv-floating-titlebar` | `expected 2 to be greater than or equal to 4` | *«keeps every radius of its own concentric …»*, chiaro e scuro | `2 failed \| 175 passed` | uguale |
| 17 | `dock.browser.test.ts`, il fondo delle linguette | tolte le quattro righe `--dv-…-tab-background-color` dal contenitore | `expected 'rgb(27, 23, 24)' to be 'rgb(36, 31, 32)' // Object.is equality` | *«floats a group BELOW the dialogs …»*, **solo scuro** — la metà chiara non può mordere | `1 failed \| 176 passed` | uguale |
| 18 | `probes.browser.test.ts`, il raggio disegnato | `return Math.min(radius(element, corner), box.height / 2, box.width / 2);` | la barra: `expected { near: 2, bad: [ …(2) ] } to deeply equal { near: 2, bad: [] }`; il dock: `expected [ …(2) ] to deeply equal []` | *«reads the radius a corner is DRAWN with …»*, e *«keeps every radius of its own concentric …»* chiaro e scuro | `3 failed \| 174 passed` | uguale |
| 19 | `probes.browser.test.ts`, la riduzione | `return radius(element, corner);` | la pillola: `expected { near: 2, bad: [] } to deeply equal { near: 2, bad: [ …(2) ] }`; ciò che scorre: `expected { near: 1, …(1) } to deeply equal { near: 1, …(1) }`; il dock: `expected [ …(24) ] to deeply equal []`; ⚠️ la pagina kit: `expected [ …(24) ] to deeply equal []` | la pillola, ciò che scorre, il dock chiaro e scuro — ⚠️ **e** `kit.browser.test.ts`, *«keeps every radius concentric (answer 4)»*, chiaro e scuro, coi radio e i pulsanti a pillola letti 9999: **E49 (b)** | `6 failed \| 171 passed` | uguale |
| 20 | `probes.browser.test.ts`, ciò che scorre | tolta `if (scrolled(element, ancestor)) continue;` | `expected { near: 1, …(1) } to deeply equal { near: +0, bad: [] }` | la sola prova di ciò che scorre; quella del dock **verde** | `1 failed \| 176 passed` | uguale |

Alla fine, dalla radice: `git status --porcelain | diff <scratchpad>/prima-violazioni.txt -` → **nulla**; `git status
--porcelain | diff <scratchpad>/prima.txt -` → soltanto i **quindici** file del compito (11 ` M`, 4 `??`); `cmp` di nuovo
sugli otto file contro le copie → uguali tutti; nessun file ignorato nato sotto `gui/src`. Nessun file nato dai rossi del
browser (R2-3).

## 5. I fine-riga

Misurati prima di toccare e dopo, con `tr -cd '\r' < f | wc -c`, `wc -l` e `git ls-files --eol`. Questa macchina ha
`core.autocrlf` `false`: ogni file `i/lf w/lf`.

| File | Prima: CR / righe, `--eol` | Dopo: CR / righe, `--eol` |
|---|---|---|
| `gui/src/tokens/readToken.ts` | — (nuovo) | 0 / 14, `i/lf w/lf` |
| `gui/src/tokens/dock.test.ts` | — (nuovo) | 0 / 45, `i/lf w/lf` |
| `gui/src/frame/BigTabFace.vue` | — (nuovo) | 0 / 40, `i/lf w/lf` |
| `gui/src/frame/dock.browser.test.ts` | — (nuovo) | 0 / 133, `i/lf w/lf` |
| `gui/src/tokens/dock.css` | 0 / 78, `i/lf w/lf` | 0 / 130, `i/lf w/lf` |
| `gui/src/frame/BigTab.ts` | 0 / 62, `i/lf w/lf` | 0 / 58, `i/lf w/lf` |
| `gui/src/frame/bigtab.test.ts` | 0 / 52, `i/lf w/lf` | 0 / 67, `i/lf w/lf` |
| `gui/src/jsdom-setup.ts` | 0 / 13, `i/lf w/lf` | 0 / 26, `i/lf w/lf` |
| `gui/src/frame/dock.ts` | 0 / 121, `i/lf w/lf` | 0 / 152, `i/lf w/lf` |
| `gui/src/frame/Frame.vue` | 0 / 66, `i/lf w/lf` | 0 / 70, `i/lf w/lf` |
| `gui/src/frame/frame.test.ts` | 0 / 217, `i/lf w/lf` | 0 / 252, `i/lf w/lf` |
| `gui/src/tokens/tokens.browser.test.ts` | 0 / 121, `i/lf w/lf` | 0 / 132, `i/lf w/lf` |
| `gui/src/testing/probes.ts` | 0 / 134, `i/lf w/lf` | 0 / 156, `i/lf w/lf` |
| `gui/src/testing/probes.browser.test.ts` | 0 / 56, `i/lf w/lf` | 0 / 92, `i/lf w/lf` |
| `gui/eslint.config.js` | 0 / 171, `i/lf w/lf` | 0 / 171, `i/lf w/lf` |
| `docs/superpowers/plans/2026-09-23-design-system.md` | 0 / 9298, `i/lf w/lf` | 0 / 9298, `i/lf w/lf` |

I file nuovi nascono LF; i riscritti tengono il terminatore che avevano, LF; nessun `sed -i`.

## 6. Il cancello

`bash scripts/gate.sh`, da solo, in background, dopo il Passo 8 e con il server di sviluppo fermato: `GATE GREEN`, 64 s.

- jsdom: ` Test Files  19 passed | 1 skipped (20)`, `      Tests  137 passed | 1 skipped (138)`
- browser: ` Test Files  5 passed (5)`, `      Tests  40 passed (40)`
- `dist/assets/index-DX-fK1-d.js 690.57 kB │ gzip: 210.40 kB`
- `found 0 vulnerabilities`

Poi `bash scripts/check-docs.sh` → *«OK — no inconsistencies.»*, uscita 0. `git status --porcelain` prima del commit: i
quindici file del compito e il piano, nient'altro.

## 7. Il Passo 8

Fatto a metà, la metà che si misura: `npm run dev -- --port 5173 --strictPort`, e uno script Playwright dello scratchpad
col **Chrome installato**, `channel: "chrome"`, senza finestra e **con le barre accese** —
`ignoreDefaultArgs: ["--hide-scrollbars"]` (E48) —; Chrome `154.0.8037.58`. Il server fermato **per PID** (2672, il `node`
in ascolto, con `taskkill //T //F`; anche il padre 34240 sparito), e nessuno in ascolto su 5173 prima del cancello.

| Misura | Chiaro | Scuro |
|---|---|---|
| il frammento del passo 17 del compito 1 (il recinto del Passo 8, copiato dal brief), a 1440 × 900 dopo `harnessFake.deliverAll()` | `seen` **33**, il peggiore **6,22** (*«Degrado»*, *«Policy VRAM»*, *«Giornale»*) | `seen` **33**, il peggiore **6,41** (gli stessi) |
| le linguette visibili della striscia | cinque — Stato, Permessi, Impostazioni, Attività, Costi —, linguetta e presa alte **40**, `padding` `0px 8px` | le stesse |
| il gruppo staccato con «Stacca la tessera» su Stato | un `.dv-resize-container`, `z-index` **50**, raggio **20px**, intestazione e linguetta `rgb(251, 248, 243)` | `z-index` 50, raggio 20px, intestazione e linguetta **`rgb(36, 31, 32)`** (E47) |
| poi «+ moduli», e il secondo frammento | — | `base-dialog-veil` |

**E43, la fascia che entra:** in una pagina nuova, dopo `Accepted`, un contatore su `requestAnimationFrame` per 40
fotogrammi e `harnessFake.deliver("StaleBuild")`: nessun fotogramma con `scrollHeight > clientHeight` o `scrollWidth >
clientWidth` sull'elemento che scorre la pagina, a **1440 × 900**, **1920 × 950**, **1400 × 960** e **1000 × 700**, con la
fascia a video (*«Il core parla una versione diversa del p…»*).

Nella console della pagina un solo errore, `Failed to load resource: … 404`: è `favicon.ico` — `curl` → 404, `index.html`
non dichiara un'icona, e fra le risposte della pagina nessuna sopra il 399.

**Gli scatti**, con le barre accese, per chi porta l'aspetto al proprietario, nello scratchpad del dispaccio:
`step8-home-light.png`, `step8-home-dark.png`, `step8-floating-light.png`, `step8-floating-dark.png`,
`step8-drawer-dark.png`; i numeri in `step8-result.json`. ⛔ **L'aspetto non l'ho giudicato**: le schede, le linguette col
segno, i divisori al passaggio, la **zona d'arrivo trascinando una linguetta** (D10: il bordo non lo vede nessuna prova, e
io non ho trascinato), il gruppo staccato, le barre di scorrimento di Stato e Impostazioni all'angolo tondo della scheda
(E48) — sono del proprietario (controllo 15, D24).

## 8. Le divergenze, e ciò che non ho misurato

**E49 — candidata, Nit — Compito 6, Passo 7: due righe fanno cadere più prove di quelle che l'Atteso nomina**, la specie
di **E12**: chi esegue si fermerebbe su una divergenza che non c'è. Nessuna delle due toglie alla riga ciò che prova — il
rosso della prova nominata c'è, col suo messaggio —; nessun codice cambia.

- **(a)** la riga *«`dock.browser.test.ts`, la distanza»*, `gap: 0 };`: oltre alle due del browser, rossa anche sotto
  jsdom `frame.test.ts > the dock > refuses a gap that is not a length in px, and builds the theme from the token that is`,
  `AssertionError: expected [Function] to throw an error` (riga 216) — il `gap` scritto a mano non passa più da
  `pixels`, e con `--space-3` a `0.75rem` `harnessTheme()` non lancia. Misurato: `npx vitest run --reporter=verbose` →
  `Tests  3 failed | 174 passed | 1 skipped (178)`. Il testo proposto per l'Atteso: *«rosso, nei due temi: `expected [
  Array(10) ] to deeply equal []`, le distanze a 0; e sotto jsdom la prova del `gap` in px di `frame.test.ts`, `expected
  [Function] to throw an error`: il `gap` scritto a mano non legge il token»*.
- **(b)** la riga *«`probes.browser.test.ts`, la riduzione»*, `return radius(element, corner);`: oltre alla pillola, a ciò
  che scorre e al dock nei due temi, rossa anche la prova dei raggi della **pagina kit** nei due temi,
  `kit.browser.test.ts > the kit page, light theme > keeps every radius concentric (answer 4)` e la sua scura, `expected [
  …(24) ] to deeply equal []` (riga 72), con voci come `radio in kit-card, top-left: radius 9999.0, outer 20.0, distance
  13.0/65.0` e `base-button in kit-strip, bottom-right: radius 9999.0, outer 9999.0, distance 9.0/9.0` — senza nessuna
  riduzione i radio e le pillole del kit si leggono 9999. Misurato: `Tests  6 failed | 171 passed | 1 skipped (178)`. La
  frase di E46 *«sulla pagina kit … nessun esito cambia»* parla della cura contro la riduzione di prima, non della riduzione
  tolta. Il testo proposto: aggiungere all'Atteso *«e nei due temi quella dei raggi della pagina kit, `expected [ …(24) ] to
  deeply equal []`, i radio e le pillole letti 9999»*.

**Un'osservazione, non una divergenza — il controllo 15 alla lettera.** Il disegno scrive *«`themeAbyss` non compare più
nel sorgente»*; dopo il compito `grep -rn themeAbyss gui/src` rende **quattro** righe, tutte **commenti dettati dal piano**
— `frame/dock.ts:80`, `frame/frame.test.ts:195`, `tokens/dock.css:3`, `tokens/dock.test.ts:23` — e nessun import né uso. La
prova che il piano dà al controllo 15 è quella del DOM, *«`themeAbyss`'s is gone (control 15)»* in `frame.test.ts`, verde.
Lo segnalo perché il compito 9, che non ho letto, potrebbe verificare il controllo 15 con un `grep`.

**Ciò che non ho misurato:** l'aspetto del Passo 8 (§7), del proprietario; la CI, del coordinatore dopo il push. Nessun
rosso del progetto `browser` fuori dalle violazioni, quindi nessuna rilettura della versione di Chrome durante il lavoro.

## 9. Il tempo, e i log

Dalle 18:01 alle 18:20 del 2026-09-26, macchina `Jays` — il commit alle 18:17 —: circa 20 minuti. Due cancelli da 64 s
ciascuno.

Lo scratchpad del dispaccio è
`C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\d5484876-d48f-457d-8568-4ed6a1b75b37\scratchpad\task6\`:

- i cancelli: `gate-apertura-20260926-180236.log` (a `1a83208`) e `gate-chiusura-20260926-181446.log` (sull'albero del
  commit); `check-docs.log`;
- i passi: `build-passo1.log`, `passo2-rosso.log`, `passo2-verde.log`, `passo3-jsdom.log`, `passo3-browser.log`,
  `passo4-jsdom.log`, `passo4-browser.log`, `passo5-rosso.log`, `passo5-verde.log`, `passo6.log`;
- il Passo 7: `violazioni-2-20.txt` e `viol-01.log` … `viol-20.log`; le violazioni in `viol/`, le copie in `copie/`,
  `prima.txt` e `prima-violazioni.txt`;
- la stabilità: `stab-1.log` … `stab-10.log`, `suite-1.log` … `suite-5.log`;
- il Passo 8: `step8.cjs`, `step8.log`, `step8-result.json`, i cinque `.png`, `dev-server.log`;
- i fine-riga: `eol-prima.txt`, `eol-dopo.txt`; il confronto col piano: `compare_task6.log`; il messaggio: `commit-msg.txt`.
