# Il rapporto del compito 8 — la cornice: la barra col nome della vista, la Panoramica, la striscia a pillola

## 1. Lo stato

**`DONE_WITH_CONCERNS`** — commit **`ac56b11`** su `main`, a partire da `bc6e94d`. ⛔ Nessun `git push` (è del coordinatore,
dopo la revisione), nessun `--amend`, nessun co-autore (`git log -1 --format=%B HEAD | grep -ci co-authored` → `0`). Il
codice è il testo del piano, byte per byte, e ogni Atteso misurabile è tornato; la **preoccupazione** è una sola voce
candidata, **E102**, un comando del Passo 7 che non nomina un file che l'Atteso conta (§7) — non ferma nessun passo.

Le verifiche d'avvio, tutte tornate: `git rev-parse --short HEAD` → `bc6e94d`; `git status --porcelain` → vuoto;
`git log --oneline f3206c7..HEAD -- gui/` → nulla; `node --version` → `v24.19.0`, contro `engines`
`^22.22.2 || ^24.15.0 || >=26.0.0`; `git config --show-origin --get-all core.autocrlf` → tre righe, `true` dal file di
sistema, `false` da `C:/Users/Jays/.gitconfig`, **`false` da `.git/config`**, l'ultima, che vale: l'albero è `w/lf`; Google
Chrome **`154.0.8037.58`**, letto dal nome della cartella in `/c/Program Files/Google/Chrome/Application/` — la cartella di
`$LOCALAPPDATA` non c'è, e il comando esce `2` stampando la versione, com'è previsto. Macchina `Jays`.

Il brief `task-8-brief.md` porta in testa *«a `HEAD` = `bc6e94d` — piano e disegno coincidono con `HEAD`»*; letto tutto, a
blocchi. In più, con un mio script nello scratchpad (`ops_equal.py`), `plan_ops.parse` sul brief e sul piano rende le
**stesse 39 operazioni** del compito 8: il testo che ho letto è quello che ho applicato.

⚠️ **Come ho applicato il testo del piano:** con `plan_ops.py` della cartella tracciata del dispaccio, come l'implementatore
del compito 7 — `python plan_ops.py docs/superpowers/plans/2026-09-23-design-system.md 8 apply . --only-step N`, **un passo per
volta nell'ordine del compito** (2, poi 4, 5, 6, 7), coi rossi del Passo 3 guardati prima del codice. Lo strumento scrive come
`replace_unique.py` — un'occorrenza unica, `newline=""`, temporaneo e `os.replace`, i file nuovi LF, i riscritti col
terminatore che avevano —; prima, `list` ha reso le **39** operazioni del compito e un `--dry` del Passo 2 `23 operations,
0 refused`. Dopo il commit, `compare_task8.py bc6e94d HEAD` → i **venti** file `OK`, `--- 20 paths, 0 mode changes, 0 not
matching the plan's text`, uscita 0: il piano non è fra i file cambiati (**E101**). `replace_unique.py` l'ho copiato nello
scratchpad dal brief, e non è servito.

## 2. `git show --stat HEAD`

```
commit ac56b11  (design-system(compito 8): la cornice -- la barra col nome della vista, la Panoramica, la striscia a pillola. …)

 gui/src/a11y.test.ts                |  34 ++++-
 gui/src/components/Confirm.vue      |  10 +-
 gui/src/frame/Drawer.vue            |  13 +-
 gui/src/frame/Frame.vue             |  35 +++--
 gui/src/frame/Overview.vue          | 275 ++++++++++++++++++++++++++++++++++++
 gui/src/frame/ViewBar.vue           |  30 ++--
 gui/src/frame/dock.browser.test.ts  |  20 +--
 gui/src/frame/frame.browser.test.ts | 253 +++++++++++++++++++++++++++++----
 gui/src/frame/frame.test.ts         | 195 ++++++++++++++++++++++++-
 gui/src/kit/kit.browser.test.ts     |  23 +--
 gui/src/locales/copy.test.ts        |  13 +-
 gui/src/locales/it.json             |  14 +-
 gui/src/panels/Strip.vue            |  16 ++-
 gui/src/stores/drawer.ts            |  13 ++
 gui/src/stores/invoke.ts            |  13 +-
 gui/src/stores/layout.ts            |  13 +-
 gui/src/stores/stores.test.ts       |  12 ++
 gui/src/testing/axe.ts              |  12 ++
 gui/src/testing/probes.ts           |  14 ++
 gui/src/tokens/dock.css             |  17 +++
 20 files changed, 902 insertions(+), 123 deletions(-)
 create mode 100644 gui/src/frame/Overview.vue
 create mode 100644 gui/src/stores/drawer.ts
```

Il messaggio intero sta nel commit (`git log -1 --format=%B ac56b11`): comincia con `design-system(compito 8): `, porta il
pezzo JavaScript prima e dopo (**E80**, N-2), dice la voce candidata del Passo 7 e finisce con `check-docs OK, GATE GREEN`.
`git diff --stat bc6e94d..HEAD -- crates/ gui/schema/` → vuoto (vincolo 12). Nessun file fuori dalla lista del punto 2 del
contratto: né il piano, né `docs/`, `scripts/`, `.github/`, `Cargo.*`, `gui/package*.json`.

## 3. I passi 1–8, coi comandi e le uscite vere

I log stanno nello scratchpad, `C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\d19536a1-7be4-437b-8e11-39fed58af059\scratchpad\task8\`
(in Git Bash `/c/Users/Jays/AppData/Local/Temp/claude/E--ALL-DEV-MY-REPOS-daemon/d19536a1-7be4-437b-8e11-39fed58af059/scratchpad/task8/`).

**Passo 1** — alle 20:27:13.

| Comando | Uscita |
|---|---|
| `git status --porcelain > <scratchpad>/prima.txt` | 0 byte: l'albero era pulito |
| `grep -rnw 'F3' gui/src` | nulla, uscita 1 (**E91**: `-w` non prende `#F3EEE6`) |
| `ls gui/src/stores/drawer.ts` | `No such file or directory`, uscita 2 |
| `grep -rn 'bar\.views' gui/src` | una riga sola, `gui/src/frame/ViewBar.vue:17:    <nav :aria-label="$t('bar.views')">` |
| `grep -n '"reka-ui"' gui/package.json` | `23:    "reka-ui": "2.10.4",` |
| `(cd gui && npm run build 2>&1 \| grep -E 'assets/index-.*\.js ')` | `dist/assets/index-CvaZoYyv.js  693.02 kB │ gzip: 211.20 kB` — la baseline (**E80**, **E91**) |
| `bash scripts/gate.sh`, da solo, in background | `GATE GREEN.`, uscita 0, 78 s; sotto `gui/` jsdom `Test Files  21 passed \| 1 skipped (22)`, `Tests  157 passed \| 1 skipped (158)`; browser `Test Files  6 passed (6)`, `Tests  56 passed (56)`; `693.02 kB │ gzip: 211.20 kB`; `found 0 vulnerabilities` — i numeri del pre-controllo, uguali. Log: `gate-passo1-20260927-202731.log` |

**Passo 2** — alle 20:29:05, `plan_ops.py … 8 apply . --only-step 2` → `23 operations, 0 refused`: `testing/axe.ts` (1),
`testing/probes.ts` (1), `kit/kit.browser.test.ts` (4), `frame/dock.browser.test.ts` (3), `frame/frame.test.ts` (2),
`stores/stores.test.ts` (1), `a11y.test.ts` (4), `locales/copy.test.ts` (3), `frame/frame.browser.test.ts` (4). Nove file
modificati, nessuno nuovo.

**Passo 3** — alle 20:29:24, rosso **per le ragioni dell'Atteso**.

`(cd gui && npx vitest run --project jsdom src/frame/frame.test.ts src/stores/stores.test.ts src/a11y.test.ts src/locales/copy.test.ts)`, uscita 1:

- `FAIL |jsdom| src/a11y.test.ts` → `Error: Failed to resolve import "./stores/drawer" from "src/a11y.test.ts". Does the file exist?`
- `FAIL |jsdom| src/frame/frame.test.ts` → `Error: Failed to resolve import "../stores/drawer" from "src/frame/frame.test.ts". Does the file exist?`
- `stores.test.ts` (21 prove, 1 rossa): *«shows a named view only by a name the list holds, and refuses any other (E90)»* → `TypeError: layout.showNamed is not a function`
- `copy.test.ts` **verde**
- `Test Files  3 failed | 1 passed (4)`, **`Tests  1 failed | 24 passed (25)`**

`(cd gui && npx vitest run --project browser src/frame/frame.browser.test.ts src/frame/dock.browser.test.ts src/kit/kit.browser.test.ts)`, uscita 1:

- `dock.browser.test.ts` (16 prove, 2 rosse), nei due temi, *«draws every group as a card, and the strip as a pill, `--space-3` from its neighbours»* → `AssertionError: expected '21px' to be '9999px'`
- `frame.browser.test.ts` (11 prove, 8 rosse):
  - nei due temi *«floats the strip as a pill 12 px from the sides and 24 from the bottom, and keeps every card off the page's corners (control 19)»* → `AssertionError: expected 21 to be greater than or equal to 25`
  - nei due temi *«opens the drawer from the strip's pill, gives the focus back to it when the drawer closes, and from the keyboard opens it with the focus in sight»* → `AssertionError: expected null not to be null`
  - nei due temi *«draws the overview with its radii concentric, nothing cut or sticking out, the icons centred, and no axe violation»* → `AssertionError: expected null not to be null` (`Matcher did not succeed in time`: F3 non apre nulla)
  - *«moves by geometry with the arrows, enters a view with Enter, and gives the focus back to the view's name»* → `AssertionError: expected null not to be null`
  - *«draws the focus ring of a panel that scrolls inside it, clear of its card, and its corners below follow the card's (E74)»* → `AssertionError: expected 9 to be greater than or equal to 13`
- verdi: le 22 della pagina kit, le 14 altre del dock, le due fasce e la barra
- `Test Files  2 failed | 1 passed (3)`, **`Tests  10 failed | 39 passed (49)`**

**Passo 4** — alle 20:30:14, `--only-step 4` → `7 operations, 0 refused`: `stores/drawer.ts` creato (LF), tre sostituzioni in
`stores/invoke.ts`, due in `components/Confirm.vue`, `frame/Drawer.vue` riscritto (LF).

**Passo 5** — alle 20:30:20, `--only-step 5` → `5 operations, 0 refused`: due sostituzioni in `locales/it.json`, due in
`stores/layout.ts`, `frame/Overview.vue` creato (LF).

**Passo 6** — alle 20:30:26, `--only-step 6` → `2 operations, 0 refused`: `frame/ViewBar.vue` e `frame/Frame.vue` riscritti (LF).

**Passo 7** — `--only-step 7` → `2 operations, 0 refused`: `panels/Strip.vue` riscritto (LF), una sostituzione in
`tokens/dock.css`. Poi, alle 20:30:36:

| Comando | Uscita |
|---|---|
| `(cd gui && npx vitest run --project jsdom src/frame/frame.test.ts src/a11y.test.ts src/locales/copy.test.ts)` — **com'è scritto** | uscita 0, `Test Files  3 passed (3)`, **`Tests  40 passed (40)`** — l'Atteso dice 61 (**E102**, §7) |
| lo stesso con `src/stores/stores.test.ts`, i **quattro** file dell'Atteso e del Passo 3 | uscita 0, `Test Files  4 passed (4)`, **`Tests  61 passed (61)`** — l'Atteso |
| `(cd gui && npx vitest run --project browser src/frame/frame.browser.test.ts src/frame/dock.browser.test.ts src/kit/kit.browser.test.ts)` | uscita 0, `Test Files  3 passed (3)`, **`Tests  49 passed (49)`** |

**Passo 8** — alle 20:31:05.

| Comando | Uscita |
|---|---|
| `(cd gui && npm test && npm run build && npm run lint)` | `npm test` uscita 0, **`Test Files  27 passed \| 1 skipped (28)`**, **`Tests  234 passed \| 1 skipped (235)`**; `npm run build` uscita 0 — `vue-tsc --noEmit` muto, `✓ 2640 modules transformed`, l'avviso di sempre sui pezzi sopra i 500 kB (N-2); `npm run lint` uscita 0, nessun messaggio |
| `(cd gui && npm run build 2>&1 \| grep -E 'assets/index-.*\.js ')` | **`dist/assets/index-D1K6_br5.js  698.14 kB │ gzip: 212.98 kB`**, contro la baseline `693.02 kB`, `211.20 kB` |
| `npx vitest run --reporter=json --outputFile=<scratchpad>/corsa-N.json`, N da 1 a 5 | cinque uscite 0; in ciascun rapporto 28 file, `numPassedTests` **234**, `numFailedTests` 0, `numPendingTests` 1, nessun file rosso: **cinque corse su cinque**, nessuna caduta |

I file di prova sono **gli stessi** del compito 7 (28 = 22 jsdom + 6 browser) e le prove **ventuno** in più: dodici sotto jsdom
— le otto della cornice in `frame.test.ts`, le due della Panoramica e del cassetto aperti in `a11y.test.ts`, quella delle parole
delle viste in `copy.test.ts`, quella di `showNamed` in `stores.test.ts` — e nove nel browser, da 40 a 49 nei tre file.

## 4. Il Passo 9 — le due direzioni

Il metodo: prima della prima violazione `git status --porcelain > <scratchpad>/prima-passo9.txt` (i venti file del compito) e
una copia di ognuno dei **nove** file che le violazioni toccano — `Overview.vue`, `Frame.vue`, `ViewBar.vue`, `Strip.vue`,
`it.json`, `dock.css`, `layout.ts`, e i due **fuori** dal compito, `components/BaseDialog.vue` e `frame/Band.vue` — in
`<scratchpad>/copie/`. Con lo script `violations.py` (nello scratchpad): ogni violazione è una sostituzione a occorrenza unica
— controllate tutte e 32 prima, *«0 anchors not unique»* —, poi `npx vitest run` sotto `gui/`, **i due progetti, la suite
intera**, col rapporto JSON; poi la copia torna, e `filecmp` a byte la dice uguale. Dalle 20:34:14 alle 20:38:46 (la riga 25
l'avevo prima provata da sola, alle 20:34:03, per provare lo strumento: stesso esito). I log e i JSON, uno per riga, in
`<scratchpad>/violazioni/`, il riepilogo in `violazioni.log`.

Le violazioni, com'è la tabella: 1 `current: false,` per le tre; 2 e 3 `if (!drawer.open) …` e `if (!invoke.asking) …`; 4 tolta
`open.value = false;` da `choose`; 5 tolta `refusal.value = result;`; 6 `props.snapshot(), []);`; 7 tolta la riga del `.filter`
sulla striscia; 8 tolto ` @click="drawer.open = true"`; 9 tolta `<BaseLabel>{{ card.name }}</BaseLabel>`; 10 tolta la riga di
`"compact"` **con la virgola** di `"work"` (**E95**); 11 tolta la regola `:has(.strip)` (selettore e blocco, il commento resta);
12 il margine di `.dock` a `0 var(--space-3) var(--space-3)`; 13 il padding di `.bar` a `var(--space-2) 0`; 14 il `blur()`
prima di aprire; 15 il raggio di `.mini` a `var(--radius-card)`; 16 `.caption` con `width: 40px;` e `overflow: hidden;`; 17 e
18 le due regole prima di `.caption {`; 19 tolto ` @keydown="onArrow"`; 20 `:tabindex="0"`; 21 `show: () => { layout.view =
view; },`; 22 `layout.showNamed((layout.saved?.named ?? [])[0]?.name ?? entry.name);`; 23 tolta `:disabled="layout.arrivals ===
0"`; 24 il `try`/`catch` tolto, `tiles = schematic(saved);`; 25 tolta la riga del `return false` di `showNamed`; 26 tolta la
riga `outline-offset`; 27 tolte le due `border-bottom-*-radius`; 28 tolta `position: sticky;` in `BaseDialog.vue`; 29
`max-width: 320px;`; 30 `background: var(--color-bg-raised);` dopo il `padding` di `.bar`; 31 tolta `overflow: clip;`; 32
tolto `on-page` dal ramo `stale` di `Band.vue`.

**Ogni riga rossa col messaggio della tabella del piano, e sulle prove che la tabella nomina — né più né meno**; le tre righe di
**E95** (4, 8, 9) anche sulle prove in più che nominano. La colonna dei messaggi è la prima riga del fallimento, com'è nel
rapporto JSON (senza il suffisso `// Object.is equality`); i totali sono della suite intera, 235 prove.

| # | La prova (riga della tabella) | Il rosso vero | Le prove che cadono | La suite | `cmp` |
|---|---|---|---|---|---|
| 1 | frame.test.ts, la carta corrente | `AssertionError: expected [ null, null, null, null ] to deeply equal [ 'page', null, null, null ]` | 1: `frame/frame.test.ts`: *«the frame (the (d) of the design system) names the view on screen in the bar, and opens the overview from the name»* | 1 failed, 233 passed, 1 skipped | uguale |
| 2 | frame.test.ts, F3 sotto la conferma (D15) | `AssertionError: expected true to be false` | 1: `frame/frame.test.ts`: *«the frame (the (d) of the design system) opens and closes the overview with F3, and keeps quiet while the confirmation or the drawer is open»* | 1 failed, 233 passed, 1 skipped | uguale |
| 3 | frame.test.ts, F3 sotto il cassetto (D16) | `AssertionError: expected true to be false` | 1: `frame/frame.test.ts`: *«the frame (the (d) of the design system) opens and closes the overview with F3, and keeps quiet while the confirmation or the drawer is open»* | 1 failed, 233 passed, 1 skipped | uguale |
| 4 | frame.test.ts, la carta scelta | `AssertionError: expected true to be false`<br>`Error: expected <div data-v-51513c71 …(10)>…(3)</div> to be null` | 2: `frame/frame.test.ts`: *«the frame (the (d) of the design system) shows the view of the card chosen and closes, opens a named view by its name, and closes it with one of the three (E76, E82)»*<br>`frame/frame.browser.test.ts`: *«the overview's grid, under the keys (R3-23 of the review) moves by geometry with the arrows, enters a view with Enter, and gives the focus back to the view's name»* | 2 failed, 232 passed, 1 skipped | uguale |
| 5 | frame.test.ts, il rifiuto detto (D4) | `AssertionError: expected 'AnnullaSalva' to contain 'Scrivi un nome.'` | 1: `frame/frame.test.ts`: *«the frame (the (d) of the design system) says under the field a name that is empty or taken, and saves a new one and opens it (D4, D12)»* | 1 failed, 233 passed, 1 skipped | uguale |
| 6 | frame.test.ts, i nomi delle tre (D12) | `AssertionError: a name that is taken keeps the overview open: expected false to be true` | 1: `frame/frame.test.ts`: *«the frame (the (d) of the design system) says under the field a name that is empty or taken, and saves a new one and opens it (D4, D12)»* | 1 failed, 233 passed, 1 skipped | uguale |
| 7 | frame.test.ts, la miniatura (D14) | `AssertionError: expected [ 'activity', 'costs', …(5) ] to deeply equal [ 'activity', 'costs', …(4) ]` | 1: `frame/frame.test.ts`: *«the frame (the (d) of the design system) draws each view in miniature from its layout: a tile per group, with its module's icon, and not the strip (D14)»* | 1 failed, 233 passed, 1 skipped | uguale |
| 8 | frame.test.ts, il pulsante della striscia | `AssertionError: expected false to be true`<br>`Error: expected null not to be null` | 3: `frame/frame.test.ts`: *«the frame (the (d) of the design system) opens the drawer from the strip's button»*<br>`frame/frame.browser.test.ts` (chiaro): *«the frame, light theme opens the drawer from the strip's pill, gives the focus back to it when the drawer closes, and from the keyboard opens it with the focus in sight»*<br>`frame/frame.browser.test.ts` (scuro): *«the frame, dark theme opens the drawer from the strip's pill, gives the focus back to it when the drawer closes, and from the keyboard opens it with the focus in sight»* | 3 failed, 231 passed, 1 skipped | uguale |
| 9 | a11y.test.ts, la Panoramica aperta | `AssertionError: expected [ Array(1) ] to deeply equal []` | 3: `a11y.test.ts`: *«the SPA, with the welcome delivered the overview, open, has no violation»*<br>`frame/frame.browser.test.ts` (chiaro): *«the frame, light theme draws the overview with its radii concentric, nothing cut or sticking out, the icons centred, and no axe violation»*<br>`frame/frame.browser.test.ts` (scuro): *«the frame, dark theme draws the overview with its radii concentric, nothing cut or sticking out, the icons centred, and no axe violation»* | 3 failed, 231 passed, 1 skipped | uguale |
| 10 | copy.test.ts, le parole delle viste | `AssertionError: compact: expected [ 'home', 'work' ] to include 'compact'` | 1: `locales/copy.test.ts`: *«the strings has a name for every view that ships (the overview and the bar build `views.${view}`)»* | 1 failed, 233 passed, 1 skipped | uguale |
| 11 | frame.browser.test.ts e dock.browser.test.ts, la pillola | `AssertionError: expected '21px' to be '9999px'`<br>`AssertionError: expected [ …(2) ] to deeply equal []`<br>`AssertionError: expected 21 to be greater than or equal to 25` | 6: `frame/dock.browser.test.ts` (chiaro): *«the dressed dock, light theme draws every group as a card, and the strip as a pill, `--space-3` from its neighbours»*<br>`frame/dock.browser.test.ts` (chiaro): *«the dressed dock, light theme keeps every radius of its own concentric, a floating group's too (answer 4)»*<br>`frame/dock.browser.test.ts` (scuro): *«the dressed dock, dark theme draws every group as a card, and the strip as a pill, `--space-3` from its neighbours»*<br>`frame/dock.browser.test.ts` (scuro): *«the dressed dock, dark theme keeps every radius of its own concentric, a floating group's too (answer 4)»*<br>`frame/frame.browser.test.ts` (chiaro): *«the frame, light theme floats the strip as a pill 12 px from the sides and 24 from the bottom, and keeps every card off the page's corners (control 19)»*<br>`frame/frame.browser.test.ts` (scuro): *«the frame, dark theme floats the strip as a pill 12 px from the sides and 24 from the bottom, and keeps every card off the page's corners (control 19)»* | 6 failed, 228 passed, 1 skipped | uguale |
| 12 | frame.browser.test.ts, i 24 px dal fondo | `AssertionError: expected 12 to be close to 24, received difference is 12, but expected 0.05` | 2: `frame/frame.browser.test.ts` (chiaro): *«the frame, light theme floats the strip as a pill 12 px from the sides and 24 from the bottom, and keeps every card off the page's corners (control 19)»*<br>`frame/frame.browser.test.ts` (scuro): *«the frame, dark theme floats the strip as a pill 12 px from the sides and 24 from the bottom, and keeps every card off the page's corners (control 19)»* | 2 failed, 232 passed, 1 skipped | uguale |
| 13 | frame.browser.test.ts, gli angoli della pagina | `AssertionError: expected [ 'base-button view-name' ] to deeply equal []` | 2: `frame/frame.browser.test.ts` (chiaro): *«the frame, light theme floats the strip as a pill 12 px from the sides and 24 from the bottom, and keeps every card off the page's corners (control 19)»*<br>`frame/frame.browser.test.ts` (scuro): *«the frame, dark theme floats the strip as a pill 12 px from the sides and 24 from the bottom, and keeps every card off the page's corners (control 19)»* | 2 failed, 232 passed, 1 skipped | uguale |
| 14 | frame.browser.test.ts, il fuoco che torna (P-22) | `Error: expected <body style>…(1)</body> to be <button data-v-2d3a2488 …(6)>…(1)</button>` | 2: `frame/frame.browser.test.ts` (chiaro): *«the frame, light theme opens the drawer from the strip's pill, gives the focus back to it when the drawer closes, and from the keyboard opens it with the focus in sight»*<br>`frame/frame.browser.test.ts` (scuro): *«the frame, dark theme opens the drawer from the strip's pill, gives the focus back to it when the drawer closes, and from the keyboard opens it with the focus in sight»* | 2 failed, 232 passed, 1 skipped | uguale |
| 15 | frame.browser.test.ts, i raggi della Panoramica | `AssertionError: expected [ …(12) ] to deeply equal []` | 2: `frame/frame.browser.test.ts` (chiaro): *«the frame, light theme draws the overview with its radii concentric, nothing cut or sticking out, the icons centred, and no axe violation»*<br>`frame/frame.browser.test.ts` (scuro): *«the frame, dark theme draws the overview with its radii concentric, nothing cut or sticking out, the icons centred, and no axe violation»* | 2 failed, 232 passed, 1 skipped | uguale |
| 16 | frame.browser.test.ts, il testo tagliato | `AssertionError: expected [ 'cut: caption "Lavoro" 49>40', …(1) ] to deeply equal []` | 2: `frame/frame.browser.test.ts` (chiaro): *«the frame, light theme draws the overview with its radii concentric, nothing cut or sticking out, the icons centred, and no axe violation»*<br>`frame/frame.browser.test.ts` (scuro): *«the frame, dark theme draws the overview with its radii concentric, nothing cut or sticking out, the icons centred, and no axe violation»* | 2 failed, 232 passed, 1 skipped | uguale |
| 17 | frame.browser.test.ts, le icone centrate | `AssertionError: expected [ …(15) ] to deeply equal []` | 2: `frame/frame.browser.test.ts` (chiaro): *«the frame, light theme draws the overview with its radii concentric, nothing cut or sticking out, the icons centred, and no axe violation»*<br>`frame/frame.browser.test.ts` (scuro): *«the frame, dark theme draws the overview with its radii concentric, nothing cut or sticking out, the icons centred, and no axe violation»* | 2 failed, 232 passed, 1 skipped | uguale |
| 18 | frame.browser.test.ts, il contrasto | `AssertionError: expected [ Array(1) ] to deeply equal []` | 2: `frame/frame.browser.test.ts` (chiaro): *«the frame, light theme draws the overview with its radii concentric, nothing cut or sticking out, the icons centred, and no axe violation»*<br>`frame/frame.browser.test.ts` (scuro): *«the frame, dark theme draws the overview with its radii concentric, nothing cut or sticking out, the icons centred, and no axe violation»* | 2 failed, 232 passed, 1 skipped | uguale |
| 19 | frame.browser.test.ts, le frecce (decisione 19) | `AssertionError: expected <button data-v-2d3a2488 …(8)>…(2)</button> to be <button data-v-2d3a2488 …(7)>…(2)</button>` | 1: `frame/frame.browser.test.ts`: *«the overview's grid, under the keys (R3-23 of the review) moves by geometry with the arrows, enters a view with Enter, and gives the focus back to the view's name»* | 1 failed, 233 passed, 1 skipped | uguale |
| 20 | frame.browser.test.ts, il giro del Tab (D17) | `AssertionError: expected [ +0, +0, +0, -1 ] to deeply equal [ +0, -1, -1, -1 ]` | 1: `frame/frame.browser.test.ts`: *«the overview's grid, under the keys (R3-23 of the review) moves by geometry with the arrows, enters a view with Enter, and gives the focus back to the view's name»* | 1 failed, 233 passed, 1 skipped | uguale |
| 21 | frame.test.ts, una delle tre chiude la vista col nome (E97) | `AssertionError: expected 'Lettura' to be null` | 1: `frame/frame.test.ts`: *«the frame (the (d) of the design system) shows the view of the card chosen and closes, opens a named view by its name, and closes it with one of the three (E76, E82)»* | 1 failed, 233 passed, 1 skipped | uguale |
| 22 | frame.test.ts, la vista col nome scelta (E97) | `AssertionError: expected 'Revisione' to be 'Lettura'` | 1: `frame/frame.test.ts`: *«the frame (the (d) of the design system) shows the view of the card chosen and closes, opens a named view by its name, and closes it with one of the three (E76, E82)»* | 1 failed, 233 passed, 1 skipped | uguale |
| 23 | frame.test.ts, Salva questa vista spenta (E98) | `AssertionError: expected false to be true` | 1: `frame/frame.test.ts`: *«the frame (the (d) of the design system) keeps «Salva questa vista» off until the core has answered (E81)»* | 1 failed, 233 passed, 1 skipped | uguale |
| 24 | frame.test.ts, la miniatura che non si legge (E99) | `TypeError: Cannot destructure property 'maximizedNode' of 'layout.grid' as it is undefined.` | 1: `frame/frame.test.ts`: *«the frame (the (d) of the design system) draws a layout it cannot read as an empty miniature, and every other as before (E90)»* | 1 failed, 233 passed, 1 skipped | uguale |
| 25 | stores.test.ts, un nome che la lista non tiene (E99) | `AssertionError: expected true to be false` | 1: `stores/stores.test.ts`: *«the named views in the package (design system, section (d)) shows a named view only by a name the list holds, and refuses any other (E90)»* | 1 failed, 233 passed, 1 skipped | uguale |
| 26 | frame.browser.test.ts, l'anello dentro (E100) | `AssertionError: expected 9 to be greater than or equal to 13` | 1: `frame/frame.browser.test.ts`: *«the bar, and the focus in a panel that scrolls draws the focus ring of a panel that scrolls inside it, clear of its card, and its corners below follow the card's (E74)»* | 1 failed, 233 passed, 1 skipped | uguale |
| 27 | frame.browser.test.ts, gli angoli dell'anello (E100) | `AssertionError: expected +0 to be close to 20, received difference is 20, but expected 0.05` | 1: `frame/frame.browser.test.ts`: *«the bar, and the focus in a panel that scrolls draws the focus ring of a panel that scrolls inside it, clear of its card, and its corners below follow the card's (E74)»* | 1 failed, 233 passed, 1 skipped | uguale |
| 28 | frame.browser.test.ts, il cassetto dalla tastiera (E96, E38) | `AssertionError: expected 1025 to be less than or equal to 900` | 2: `frame/frame.browser.test.ts` (chiaro): *«the frame, light theme opens the drawer from the strip's pill, gives the focus back to it when the drawer closes, and from the keyboard opens it with the focus in sight»*<br>`frame/frame.browser.test.ts` (scuro): *«the frame, dark theme opens the drawer from the strip's pill, gives the focus back to it when the drawer closes, and from the keyboard opens it with the focus in sight»* | 2 failed, 232 passed, 1 skipped | uguale |
| 29 | frame.browser.test.ts, il segnaposto intero (E96, E39) | `AssertionError: expected 314.8876953125 to be less than or equal to 270` | 1: `frame/frame.browser.test.ts`: *«the bar, and the focus in a panel that scrolls draws the bar as part of the page, and the search's whole placeholder in its field (E57, E39)»* | 1 failed, 233 passed, 1 skipped | uguale |
| 30 | frame.browser.test.ts, la barra senza fondo (E96, E57) | `AssertionError: expected 'rgb(251, 248, 243)' to be 'rgba(0, 0, 0, 0)'` | 1: `frame/frame.browser.test.ts`: *«the bar, and the focus in a panel that scrolls draws the bar as part of the page, and the search's whole placeholder in its field (E57, E39)»* | 1 failed, 233 passed, 1 skipped | uguale |
| 31 | frame.browser.test.ts, la pagina che non sborda (E96, E43) | `AssertionError: expected [ 50, +0, +0, +0, +0, +0, +0, +0 ] to deeply equal [ +0, +0, +0, +0, +0, +0, +0, +0 ]` | 2: `frame/frame.browser.test.ts` (chiaro): *«the frame, light theme lays both bands on the page -- under the bar, 12 px from the sides and from the cards, rounded like a card -- and the page never spills while one comes in (control 23, E60, E70, E43)»*<br>`frame/frame.browser.test.ts` (scuro): *«the frame, dark theme lays both bands on the page -- under the bar, 12 px from the sides and from the cards, rounded like a card -- and the page never spills while one comes in (control 23, E60, E70, E43)»* | 2 failed, 232 passed, 1 skipped | uguale |
| 32 | frame.browser.test.ts, la fascia stop sulla pagina (E96, E70) | `AssertionError: expected undefined to be defined`<br>`AssertionError: expected '8px' to be '21px'` | 3: `frame/frame.test.ts`: *«the band stops the window when the core speaks another protocol: no action, and the stamp the core expects (E60)»*<br>`frame/frame.browser.test.ts` (chiaro): *«the frame, light theme lays both bands on the page -- under the bar, 12 px from the sides and from the cards, rounded like a card -- and the page never spills while one comes in (control 23, E60, E70, E43)»*<br>`frame/frame.browser.test.ts` (scuro): *«the frame, dark theme lays both bands on the page -- under the bar, 12 px from the sides and from the cards, rounded like a card -- and the page never spills while one comes in (control 23, E60, E70, E43)»* | 3 failed, 231 passed, 1 skipped | uguale |

Alla fine:

- ogni copia contro il suo file, `cmp`: **nove su nove uguali** — e `git diff --quiet -- gui/src/components/BaseDialog.vue
  gui/src/frame/Band.vue` esce 0: i due file fuori dal compito sono quelli di `HEAD`;
- `git status --porcelain | diff <scratchpad>/prima-passo9.txt -` → **nulla**: nessun file nato dalle prove (vincolo 11);
- `git status --porcelain | diff <scratchpad>/prima.txt -` — la fotografia del Passo 1, albero pulito — → **soltanto i venti
  file del compito**: diciotto ` M` e i due `??`, `gui/src/frame/Overview.vue` e `gui/src/stores/drawer.ts`.

Il Passo 10 **saltato** (**E101**): è del proprietario col coordinatore, dopo la revisione. Nessuna cella del piano toccata.

## 5. I fine-riga

Misurati **prima** di toccare, con `tr -cd '\r' < f | wc -c`, `wc -l` e `git ls-files --eol`, e **dopo** il Passo 9 e di nuovo
dopo il commit per i due file nuovi. La forma non cambia: su questa macchina ogni file è **LF**, zero CR, `i/lf w/lf`, prima e
dopo; i due file nuovi nascono LF, con l'a-capo finale, e dopo `git add` sono `i/lf w/lf`.

| File | Prima: CR / righe / colonne | Dopo: CR / righe / colonne |
|---|---|---|
| `gui/src/stores/drawer.ts` (nuovo) | — | 0 / 13 / `i/lf w/lf` |
| `gui/src/frame/Overview.vue` (nuovo) | — | 0 / 275 / `i/lf w/lf` |
| `gui/src/frame/ViewBar.vue` (riscritto) | 0 / 69 / `i/lf w/lf` | 0 / 61 / `i/lf w/lf` |
| `gui/src/frame/Frame.vue` (riscritto) | 0 / 71 / `i/lf w/lf` | 0 / 84 / `i/lf w/lf` |
| `gui/src/frame/Drawer.vue` (riscritto) | 0 / 36 / `i/lf w/lf` | 0 / 33 / `i/lf w/lf` |
| `gui/src/panels/Strip.vue` (riscritto) | 0 / 41 / `i/lf w/lf` | 0 / 55 / `i/lf w/lf` |
| `gui/src/stores/invoke.ts` | 0 / 73 / `i/lf w/lf` | 0 / 82 / `i/lf w/lf` |
| `gui/src/stores/layout.ts` | 0 / 223 / `i/lf w/lf` | 0 / 234 / `i/lf w/lf` |
| `gui/src/components/Confirm.vue` | 0 / 50 / `i/lf w/lf` | 0 / 44 / `i/lf w/lf` |
| `gui/src/locales/it.json` | 0 / 135 / `i/lf w/lf` | 0 / 145 / `i/lf w/lf` |
| `gui/src/tokens/dock.css` | 0 / 166 / `i/lf w/lf` | 0 / 183 / `i/lf w/lf` |
| `gui/src/frame/frame.test.ts` | 0 / 295 / `i/lf w/lf` | 0 / 488 / `i/lf w/lf` |
| `gui/src/frame/frame.browser.test.ts` | 0 / 92 / `i/lf w/lf` | 0 / 287 / `i/lf w/lf` |
| `gui/src/stores/stores.test.ts` | 0 / 316 / `i/lf w/lf` | 0 / 328 / `i/lf w/lf` |
| `gui/src/a11y.test.ts` | 0 / 114 / `i/lf w/lf` | 0 / 136 / `i/lf w/lf` |
| `gui/src/locales/copy.test.ts` | 0 / 65 / `i/lf w/lf` | 0 / 74 / `i/lf w/lf` |
| `gui/src/frame/dock.browser.test.ts` | 0 / 219 / `i/lf w/lf` | 0 / 213 / `i/lf w/lf` |
| `gui/src/kit/kit.browser.test.ts` | 0 / 206 / `i/lf w/lf` | 0 / 189 / `i/lf w/lf` |
| `gui/src/testing/axe.ts` | 0 / 15 / `i/lf w/lf` | 0 / 27 / `i/lf w/lf` |
| `gui/src/testing/probes.ts` | 0 / 163 / `i/lf w/lf` | 0 / 177 / `i/lf w/lf` |

`BaseDialog.vue` e `Band.vue`, toccati dalle sole violazioni, sono tornati uguali byte per byte (`cmp`), quindi coi loro
fine-riga. Mai `sed -i`.

## 6. Il cancello

`bash scripts/gate.sh`, da solo, in background, sull'albero finito prima del commit: **`GATE GREEN.`**, uscita 0, 80 s.

- sotto `gui/`, progetto jsdom: **`Test Files  21 passed | 1 skipped (22)`**, **`Tests  169 passed | 1 skipped (170)`**
- progetto browser: **`Test Files  6 passed (6)`**, **`Tests  65 passed (65)`**
- il *build*: `dist/assets/index-D1K6_br5.js  698.14 kB │ gzip: 212.98 kB`
- **`found 0 vulnerabilities`**

Poi, da solo, `bash scripts/check-docs.sh` → `OK — no inconsistencies.`, uscita 0. E `git status --porcelain` coi soli venti
file del compito, nessun file nato dalle prove.

## 7. Le divergenze, e ciò che non ho misurato

**E102** (candidata), Nit — **Compito 8, Passo 7 — il comando sotto jsdom non nomina `src/stores/stores.test.ts`, e l'Atteso lo
conta.** **E93** dice *«e `stores.test.ts` nei comandi dei Passi 3 e 7»*, e l'Atteso del Passo 7 vuole `Tests  61 passed (61)`
*«nei quattro file: … quella di `showNamed`»*, che sta in `stores.test.ts`; ma il recinto del Passo 7 — riga 10928 del piano a
`bc6e94d`, riga 2215 del brief — è ancora `(cd gui && npx vitest run --project jsdom src/frame/frame.test.ts src/a11y.test.ts
src/locales/copy.test.ts)`, tre file, mentre quello del Passo 3 — riga 10124 — ha i quattro. Misurato il 2026-09-27, macchina
`Jays`, sull'albero del compito: com'è scritto, `Test Files  3 passed (3)`, `Tests  40 passed (40)`; con
`src/stores/stores.test.ts`, `Test Files  4 passed (4)`, `Tests  61 passed (61)` — la differenza, 21, sono le prove di
`stores.test.ts`. Evidenza: `grep -n 'npx vitest run --project jsdom src/frame/frame.test.ts' docs/superpowers/plans/2026-09-23-design-system.md`
rende le righe 10124 e 10928, e solo la prima nomina `src/stores/stores.test.ts`. Nessun codice cambia; la cura proposta: il
comando del Passo 7 coi quattro file, come quello del Passo 3. Non mi ha fermato: il Passo 8 lancia la suite intera, e lì
la prova di `showNamed` è verde.

Nient'altro diverge: ogni Atteso dei Passi 1, 3, 7 (coi quattro file), 8 e 9 è tornato col suo numero e col suo messaggio, e
`compare_task8.py` dice il commit uguale al testo del piano.

**Ciò che non ho misurato, o non da qui:**

- il **Passo 10**, lo sguardo nei due temi — del proprietario col coordinatore (**E101**); non ho aperto la SPA;
- la **CI** dopo il push, che è del coordinatore;
- la macchina **`zagor`**: le colonne dei fine-riga qui sopra sono di questa macchina, `core.autocrlf` `false`.

## 8. Il tempo, e i log

La lettura del brief e dei file prima delle 20:24:03, quando ho creato la cartella dello scratchpad; il Passo 1 alle 20:27:13, il commit alle 20:41:46 (`git log -1 --format=%ci`
→ `2026-09-27 20:41:46 +0200`); questo rapporto subito dopo. Il cancello: 78 s all'apertura, 80 s prima del commit; la suite
intera circa 6 s; le trentadue violazioni in 4 minuti e mezzo.

I log, nello scratchpad `C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\d19536a1-7be4-437b-8e11-39fed58af059\scratchpad\task8\`:

| Che cosa | File |
|---|---|
| il cancello d'apertura (Passo 1) | `gate-passo1-20260927-202731.log` |
| il cancello prima del commit | `gate-commit-20260927-203929.log` |
| `check-docs.sh` | `check-docs.log` |
| il *build* del Passo 1 | `passo1-build.log` |
| i rossi del Passo 3 | `passo3-jsdom.log`, `passo3-browser.log` |
| i verdi del Passo 7 | `passo7-jsdom-3file.log` (com'è scritto), `passo7-jsdom-4file.log`, `passo7-browser.log` |
| il Passo 8 | `passo8-test.log`, `passo8-build.log`, `passo8-lint.log`, `corsa-1.json` … `corsa-5.json` |
| il Passo 9 | `violazioni.log`, `violazioni/v01.log` … `v32.log` e `.json`, `violazioni/summary.json`, `copie/`, `prima.txt`, `prima-passo9.txt` |
| i fine-riga | `eol-prima.txt`, `eol-dopo.txt` |
| il messaggio del commit | `commit-msg.txt` |
| gli strumenti | `replace_unique.py` (dal brief), `ops_equal.py`, `runs.py`, `violations.py`, `table9.py` |

`fill_prompt.py` e `task-8-brief.old.md`, nella stessa cartella, erano già lì: sono del coordinatore, e non li ho toccati.
