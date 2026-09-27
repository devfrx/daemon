# Il rapporto dell'implementatore — compito 6bis, la cura delle tre voci del Passo 8

## 1. Lo stato

**`DONE`** — commit **`da7a522`**, non pushato (il push è del coordinatore, dopo la revisione).

Il codice è quello dettato: `python docs/superpowers/plans/2026-09-23-design-system-esecuzione/compare_task6bis.py 9579ca0 HEAD`
→ 20 righe `OK` (i 19 file del commit più `gui/src/tokens/themes.css`, dettato **invariato** dal Passo 5), *«20 paths, 0 mode
changes, 0 not matching the plan's text»*, uscita **0**. Nessuna divergenza dal testo del compito: ogni rosso e ogni verde dei
Passi 1–15 ha reso l'uscita che il compito e il §3 del dispaccio dicono (§7 qui sotto: nessuna voce candidata).

Verifiche d'avvio, macchina `Jays`, tutte come il dispaccio le vuole:

| Comando | Uscita |
|---|---|
| `git rev-parse --short HEAD` | `9579ca0` |
| `git status --porcelain` | vuoto |
| `git log --oneline dc77fc8..HEAD -- gui/` | nulla |
| `node --version` | `v24.19.0`, dentro `^22.22.2 \|\| ^24.15.0 \|\| >=26.0.0` di `gui/package.json` |
| `git config --show-origin --get-all core.autocrlf` | `true` dal file di sistema, `false` da `C:/Users/Jays/.gitconfig`, `false` da `.git/config` — vale l'ultima, `false`, come il §0 |
| `ls "/c/Program Files/Google/Chrome/Application/" "$LOCALAPPDATA/Google/Chrome/Application/"` | la cartella **`154.0.8037.58`**; uscita 2, perché la seconda cartella non esiste — conta la cartella |

Il brief `task-6bis-brief.md` (a `HEAD` = `9579ca0`) letto **per intero**, a blocchi; poi i soli file che il compito nomina o
modifica, `BaseIcon.vue`, `BaseButton.vue`, `BaseStatus.vue`, `stores/connection.ts`, `stores/core.ts`, e ciò che
`frame.browser.test.ts` consuma (`App.vue`, `main.ts`, `tokens/index.ts`, `tokens/readToken.ts`, `Frame.vue`, la classe
`.bar` di `ViewBar.vue`). Le righe `| **6** |` e `| **6bis** |` del piano (righe 132 e 133) **non toccate**.

**Il metodo di scrittura.** I recinti del compito copiati **dal brief a macchina**: uno script dello scratchpad,
`extract_blocks.py`, scrive ogni recinto della sezione del compito in un file suo — 93 recinti —; un secondo, `dry_run.py`,
conta ogni *Trova* nel file che vuole, sull'albero prima di toccare: **36 *Trova* su 36 presenti una volta sola**, e nessun
*Sostituisci* già presente. Le sostituzioni con `replace_unique.py`, copiato dagli *Strumenti* del brief nello scratchpad; i
due file nuovi e `Band.vue` scritti con Python `newline=""` (temporaneo più `os.replace`) dal loro recinto; `cure_board.py` ed
`extract_tokens.py` sono i recinti 17 e 19, copiati a macchina, e prima di lanciarli `ast.parse` li ha letti puliti, e le 128
righe di `cure_board.py` con un `\n` letterale erano intatte. Mai `sed -i`.

## 2. `git show --stat HEAD`

```
da7a522 design-system(compito 6bis): la cura delle tre voci del Passo 8 -- il bordo nei raggi, la barra di scorrimento, …

 .../2026-09-22-design-system-tavole/token.html     | 118 +++++++++++++++++++--
 gui/src/components/BaseDialog.vue                  |   2 +-
 gui/src/components/BaseNotice.vue                  |  89 ++++++++++++++++
 gui/src/components/icons.ts                        |  14 ++-
 gui/src/components/kit.test.ts                     |  48 ++++++++-
 gui/src/frame/Band.vue                             |  35 +++---
 gui/src/frame/frame.browser.test.ts                |  92 ++++++++++++++++
 gui/src/frame/frame.test.ts                        |  25 ++++-
 gui/src/kit/Kit.vue                                |  23 +++-
 gui/src/kit/kit.browser.test.ts                    |  54 +++++++++-
 gui/src/panels/Settings.vue                        |   6 +-
 gui/src/panels/Status.vue                          |  32 ++++--
 gui/src/panels/modules.test.ts                     |  28 +++--
 gui/src/testing/probes.browser.test.ts             |  25 ++++-
 gui/src/testing/probes.ts                          |   9 +-
 gui/src/tokens/base.css                            |  69 +++++++++++-
 gui/src/tokens/dock.css                            |   6 +-
 gui/src/tokens/tokens.browser.test.ts              |  30 ++++++
 gui/vite.config.ts                                 |   5 +-
 19 files changed, 644 insertions(+), 66 deletions(-)
```

Autore `devfrx`, **nessun co-autore** (`git log -1 --format=%B | grep -ci co-authored` → `0`). Il messaggio sta in
`<scratchpad>/commit-msg.txt`, passato con `git commit -F`; comincia con `design-system(compito 6bis): ` e chiude con le due
righe del pezzo JavaScript, prima e dopo (N-2). I 19 file sono esattamente quelli del punto 2 del contratto: 2 nuovi, 1
riscritto, 16 toccati. ⛔ Nessuna cella del piano, e il disegno no (E69): `git diff --stat 9579ca0..HEAD -- crates/
gui/schema/ .github/ Cargo.toml Cargo.lock gui/package.json gui/package-lock.json docs/adr/ scripts/ docs/superpowers/plans/
docs/superpowers/specs/2026-09-22-design-system-design.md` → vuoto (vincolo 12). `git status --porcelain` dopo il commit →
vuoto.

## 3. I passi, coi comandi e le uscite vere

I log di ogni passo stanno nello scratchpad, `<scratchpad>/task6bis/stepN*.log` (sotto, `<scratchpad>` =
`C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\2f940fbc-999d-4486-9faf-6d913c767183\scratchpad`).

**Passo 1 — il punto di partenza.** `git status --porcelain > <scratchpad>/task6bis/prima.txt` → 0 byte.

| Comando | Uscita |
|---|---|
| `grep -n -e 'radius-card:' -e 'radius-frame:' gui/src/tokens/base.css` | `39:  --radius-card: calc(var(--radius-control) + var(--space-3));   /* 20 */` e `40:  --radius-frame: calc(var(--radius-card) + var(--space-3));     /* 32 */` — senza `--border-width` |
| `grep -c -i 'scrollbar' gui/src/tokens/base.css …/token.html` | `0` e `0` |
| `grep -n 'launchOptions' gui/vite.config.ts` | `112:            provider: playwright({ launchOptions: { channel: "chrome" } }),` |
| `grep -c '1.5' gui/src/testing/probes.ts` | `2` |
| `ls gui/src/components/BaseNotice.vue gui/src/frame/frame.browser.test.ts` | `No such file or directory` per i due |
| `(cd gui && npm run build 2>&1 \| grep -E 'assets/index-.*\.js ')` | `dist/assets/index-CjDBuiZn.js 690.61 kB │ gzip: 210.41 kB` — la baseline |

Poi, da solo e in background, `bash scripts/gate.sh` → **`GATE GREEN`**, 78 s: jsdom `Test Files  19 passed | 1 skipped
(20)`, `Tests  137 passed | 1 skipped (138)`; browser `Test Files  5 passed (5)`, `Tests  48 passed (48)`; `found 0
vulnerabilities`. Uguale al pre-controllo.

**Passo 2 — le prove della sonda.** Tre sostituzioni in `probes.browser.test.ts`, `ok … (LF)` ciascuna. `(cd gui && npx vitest
run --project browser src/testing/probes.browser.test.ts)` → uscita 1, **rossa la sola prova nuova**, *«sees the pixel of a
border: the same bar at the outer radius is one pixel too round (E61)»*: `AssertionError: expected { near: 2, bad: [] } to
deeply equal { near: 2, bad: [ …(2) ] }`; `Tests  1 failed | 7 passed (8)`.

**Passo 3 — la sonda a mezzo pixel.** Due sostituzioni in `probes.ts`. I tre file del browser → uscita 1, `Test Files  2 failed
| 1 passed (3)`, **`Tests  6 failed | 36 passed (42)`**; le sonde verdi; rosse nei due temi:
- *«keeps every radius concentric (answer 4)»* della pagina kit: `base-list-row in kit-card, bottom-left: radius 8.0, outer
  20.0, distance 13.0/13.0` e il suo `bottom-right`;
- *«opens its window with the radii concentric, and no axe violation»*: `base-button in base-dialog, bottom-right: radius
  8.0, outer 20.0, distance 13.0/13.0`;
- *«keeps every radius of its own concentric, a floating group's too (answer 4)»* del dock, `expected [ …(4) ] to deeply
  equal []`: `dv-floating-titlebar in dv-resize-container, top-left: radius 20.0, outer 20.0, distance 1.0/1.0`, il suo
  `top-right`, e `dv-groupview in dv-resize-container, bottom-left` e `bottom-right`, le stesse cifre.

**Passo 4 — la prova della barra.** Una sostituzione in `tokens.browser.test.ts`. → uscita 1, `Tests  1 failed | 6 passed
(7)`, *«draw every scroll bar `--size-scrollbar` thick, …»*: **`Error: the token --size-scrollbar is not defined here: are
the token sheets loaded?`**, da `readToken src/tokens/readToken.ts:12:26`.

**Passo 5 — la tavola, poi la copia.** `sha1sum` prima: `themes.css` `445e5410…`, `base.css` `5a51054e…`, `token.html`
`476481c0…`.

```
python <scratchpad>/task6bis/cure_board.py "$(git rev-parse --show-toplevel)"
ok: E:/ALL/DEV/MY_REPOS/daemon\docs\superpowers\specs\2026-09-22-design-system-tavole\token.html (LF)
python <scratchpad>/task6bis/extract_tokens.py "$(git rev-parse --show-toplevel)"
ok: E:/ALL/DEV/MY_REPOS/daemon\gui\src\tokens\base.css (5795 characters)
ok: E:/ALL/DEV/MY_REPOS/daemon\gui\src\tokens\themes.css (6078 characters)
```

`sha1sum` dopo: `themes.css` **`445e5410…`, invariato**; `base.css` `5a4ce31a…`; `token.html` `126a59ce…`. Poi:
- `(cd gui && npx vitest run --project jsdom src/tokens/)` → uscita 0, `Test Files  5 passed (5)`, **`Tests  17 passed (17)`**;
- i tre file del browser → uscita 1, **`Tests  5 failed | 36 passed (41)`**: la barra, `expected { vertical: +0, horizontal: +0 }
  to deeply equal { vertical: 10, horizontal: 10 }`; la cornice della pagina kit nei due temi, `base-button in kit-frame,
  top-left: radius 21.0, outer 34.0, distance 12.0/12.0` coi suoi `bottom-left`, `top-right`, `bottom-right`; il dock nei due
  temi, le quattro voci di prima a `radius 21.0, outer 21.0, distance 1.0/1.0`. **Verdi** le righe di `BaseList` e la finestra;
- `grep -c 'scrollbar' gui/src/tokens/base.css` → **`17`**; `git diff --stat -- gui/src/tokens/themes.css` → vuoto.

In `token.html`, trappola 27: `grep -n -e 'raggi diventano' -e 'cornice 3'` → riga 455, *«… i raggi diventano 21 e 34 (erano
18 e 28) …»*, e riga 721, *«cornice 34 = scheda 21 + 12 + 1 · scheda 21 = controllo 8 + 12 + 1 · …»*.

**Passo 6 — le barre mostrate.** Una sostituzione in `vite.config.ts`. `tokens.browser.test.ts` → uscita 0, **`Tests  7 passed
(7)`**.

**Passo 7 — ciò che sta subito dentro un bordo.** Una sostituzione ciascuno in `dock.css`, `Kit.vue` e `BaseDialog.vue` (E66).
I tre file del browser → uscita 0, `Test Files  3 passed (3)`, **`Tests  42 passed (42)`**.

**Passo 8 — le prove di `BaseNotice`.** Quattro sostituzioni in `kit.test.ts`. `(cd gui && npx vitest run --project jsdom
src/components/kit.test.ts)` → uscita 1, **`Error: Failed to resolve import "./BaseNotice.vue" from
"src/components/kit.test.ts". Does the file exist?`**, `Tests  no tests`.

**Passo 9 — le icone dei toni, e il pezzo.** Quattro sostituzioni in `icons.ts`; `BaseNotice.vue` creato dal recinto, LF, 89
righe. → uscita 0, **`Tests  31 passed (31)`**; nessun avviso nel log.

**Passo 10 — la pagina kit, le prove.** Tre sostituzioni in `kit.browser.test.ts`. → uscita 1, **`Tests  6 failed | 16 passed
(22)`**, nei due temi: *«keeps every radius concentric»* `expected null not to be null` (`kit.browser.test.ts:82`); *«dresses
each tone of a message in its roles (control 23, E60)»* `expected [] to deeply equal [ 'info', 'ok', 'stop', 'warn' ]`;
*«sets a message's title and icon on the line of its action …»* `expected null not to be null`.

**Passo 11 — la pagina kit, i messaggi.** Tre sostituzioni in `Kit.vue`. → uscita 0, **`Tests  22 passed (22)`**.

**Passo 12 — i tre usi, le prove.** Tre sostituzioni in `frame.test.ts`, tre in `modules.test.ts`; `frame.browser.test.ts`
creato dal recinto, LF, 92 righe.
- jsdom sui due file → uscita 1, **`Tests  4 failed | 27 passed (31)`**, tutte `Error: Unable to get .base-notice within: …` —
  le due della fascia (*«is a warning with «Riprova» …»*, *«stops the window when the core speaks another protocol …»*),
  quella del verdetto (*«shows the last Verdict as a message in its tone …»*) e quella della richiesta in volo (*«sends Invoke
  with the registry's literals …»*);
- browser `frame.browser.test.ts` → uscita 1, **`Tests  2 failed (2)`**, nei due temi `AssertionError: expected +0 to be close
  to 12, received difference is 12, but expected 0.05` (`frame.browser.test.ts:75`, il lato sinistro).

**Passo 13 — i tre usi, il pezzo.** `Band.vue` riscritto per intero col terminatore che aveva, **LF** (0 CR prima), 41 righe;
tre sostituzioni in `Settings.vue`, tre in `Status.vue`. → jsdom uscita 0, **`Tests  31 passed (31)`**; browser uscita 0,
**`Tests  2 passed (2)`**. Le prove della regione — *«keeps its status region …»* nei tre usi — restano com'erano (nessuna
sostituzione le tocca) e verdi.

**Passo 14 — tutto.** `(cd gui && npm test && npm run build && npm run lint)`:
- `npm test` → uscita 0, **`Test Files  25 passed | 1 skipped (26)`**, **`Tests  200 passed | 1 skipped (201)`**;
- `npm run build` → uscita 0, `vue-tsc --noEmit` pulito, `✓ 2634 modules transformed` (erano 2631), **`dist/assets/index-DKNLC6Y2.js
  691.82 kB │ gzip: 210.77 kB`**; l'avviso dei pezzi sopra 500 kB, lo stesso di prima (N-2 di E187, del proprietario); il CSS da
  `154.72 kB │ gzip: 15.46 kB` a `156.62 kB │ gzip: 15.80 kB`;
- `npm run lint` → uscita 0, nessun messaggio.

**Passo 15 — le due direzioni.** Prima della prima violazione `git status --porcelain > <scratchpad>/task6bis/prima-violazioni.txt`
(i 19 file del compito) e una copia di ciascuno dei nove file che le violazioni toccano in `<scratchpad>/task6bis/copies/`,
`cmp` uguale. Ogni violazione con `violate.py` — una sostituzione esatta, rifiutata se il testo vecchio non c'è una volta sola —,
il file che la deve cogliere lanciato **da solo**, poi la copia indietro e `cmp`. La tabella al §4.

**Dopo il commit**, la stabilità: `npm test` altre **3** volte → 3 su 3 `Tests  200 passed | 1 skipped (201)`; con il Passo 14 e
il cancello, ogni prova è girata **5 volte su 5 verde**, nessuna caduta (log `stability-1…3.log`).

## 4. La tabella del Passo 15

Tutte le sedici righe rosse col messaggio del compito; la «prima riga che nomina la ragione» è quella di `AssertionError`.

| # | La prova (il file lanciato da solo) | La violazione | Il rosso vero | Quali prove cadono | Indietro |
|---|---|---|---|---|---|
| 1 | `probes.browser.test.ts`, il pixel del bordo | `const SLACK = 1.5;` in `probes.ts` | `expected { near: 2, bad: [] } to deeply equal { near: 2, bad: [ …(2) ] }` | *«sees the pixel of a border …»*; `Tests  1 failed \| 7 passed (8)` | `cmp` uguale |
| 2 | `tokens.browser.test.ts`, lo spessore | tolta `::-webkit-scrollbar { … }` da `base.css` | `expected { vertical: 15, horizontal: 15 } to deeply equal { vertical: 10, horizontal: 10 }` | *«draw every scroll bar …»*; `Tests  1 failed \| 6 passed (7)`. E, lanciato a parte sotto jsdom, `board.test.ts`: *«base.css is the board's block, byte for byte»*, `expected '/* EVERYTHING THAT IS NOT A COLOUR --…' to be '/* EVERYTHING THAT IS NOT A COLOUR --…'`, `Tests  1 failed \| 1 passed (2)` — come il compito dice | `cmp` uguale |
| 3 | `tokens.browser.test.ts`, l'innesco | tolta `:hover, :focus-within { --scrollbar-trigger: 1; }` | `expected '0' to be '1' // Object.is equality` | *«draw every scroll bar …»*; `Tests  1 failed \| 6 passed (7)` | `cmp` uguale |
| 4 | `tokens.browser.test.ts`, le barre mostrate | tolto `, ignoreDefaultArgs: ["--hide-scrollbars"]` da `vite.config.ts` | `expected { vertical: +0, horizontal: +0 } to deeply equal { vertical: 10, horizontal: 10 }` | *«draw every scroll bar …»*; `Tests  1 failed \| 6 passed (7)` | `cmp` uguale |
| 5 | `dock.browser.test.ts`, la barra del titolo staccata | in `dock.css` il suo raggio di nuovo `var(--radius-card) var(--radius-card) 0 0` | `expected [ …(2) ] to deeply equal []`: `dv-floating-titlebar in dv-resize-container, top-left: radius 21.0, outer 21.0, distance 1.0/1.0` e il suo `top-right` | *«keeps every radius of its own concentric, a floating group's too»*, nei due temi; `Tests  2 failed \| 14 passed (16)` | `cmp` uguale |
| 6 | `kit.browser.test.ts`, la cornice | tolto il bordo trasparente di `.kit-frame` in `Kit.vue` | `expected [ …(4) ] to deeply equal []`: `base-button in kit-frame, top-left: radius 21.0, outer 34.0, distance 12.0/12.0`, `bottom-left`, `top-right`, `bottom-right` | *«keeps every radius concentric»*, nei due temi; `Tests  2 failed \| 20 passed (22)` | `cmp` uguale |
| 7 | `kit.browser.test.ts`, la riga al centro | tolta `.base-notice[data-action]` da `BaseNotice.vue` | `expected 2 to be less than or equal to 0.5` | *«sets a message's title and icon on the line of its action …»*, nei due temi; `Tests  2 failed \| 20 passed (22)` | `cmp` uguale |
| 8 | `kit.browser.test.ts`, i ruoli del tono | `[data-tone="info"]` → `[data-tone="info-not"]` | chiaro `expected [ 'rgba(0, 0, 0, 0)', …(2) ] to deeply equal [ 'rgb(239, 225, 221)', …(2) ]`; scuro `… [ 'rgb(47, 27, 29)', …(2) ]` | *«dresses each tone of a message in its roles»*, nei due temi; `Tests  2 failed \| 20 passed (22)` | `cmp` uguale |
| 9 | `kit.browser.test.ts`, le parole nei loro ruoli | tolto `color: var(--color-text);` da `.title` | chiaro `expected 'rgb(122, 31, 46)' to be 'rgb(27, 23, 24)'` (E65); scuro `expected 'rgb(229, 158, 161)' to be 'rgb(236, 230, 218)'` | *«dresses each tone …»*, nei due temi; `Tests  2 failed \| 20 passed (22)` | `cmp` uguale |
| 10 | `kit.test.ts` (jsdom), l'icona col nome del tono | `name="info"` al posto di `:name="tone"` | `expected 'info' to be 'ok' // Object.is equality` | *«draws the icon of its tone, by the tone's name, in each of the four»*; `Tests  1 failed \| 30 passed (31)` | `cmp` uguale |
| 11 | `frame.browser.test.ts`, il raggio sulla pagina | tolta `.base-notice[data-on-page]` | `expected '8px' to be '21px' // Object.is equality` | *«lays the band on the page …»*, nei due temi; `Tests  2 failed (2)` | `cmp` uguale |
| 12 | `frame.browser.test.ts`, lo spazio della fascia | in `Band.vue` la regola `.band` vuota | `expected +0 to be close to 12, received difference is 12, but expected 0.05` | *«lays the band on the page …»*, nei due temi; `Tests  2 failed (2)` | `cmp` uguale |
| 13 | `frame.test.ts` (jsdom), «Riprova» che riprova | tolto ` @click="connection.retry()"` | `expected "wrappedAction" to be called once, but got 0 times` | *«is a warning with «Riprova» while waiting, «Riprova» retries, …»*; `Tests  1 failed \| 13 passed (14)` | `cmp` uguale |
| 14 | `frame.test.ts` (jsdom), il timbro diverso | in `Band.vue` `tone="stop"` → `tone="warn"` | `expected 'warn' to be 'stop' // Object.is equality` | *«stops the window when the core speaks another protocol …»*; `Tests  1 failed \| 13 passed (14)` | `cmp` uguale |
| 15 | `modules.test.ts` (jsdom), il rifiuto | in `Status.vue` `Refused: "warn"` | `expected 'warn' to be 'stop' // Object.is equality` | *«shows the last Verdict as a message in its tone …»*; `Tests  1 failed \| 16 passed (17)` | `cmp` uguale |
| 16 | `modules.test.ts` (jsdom), la richiesta in volo | in `Settings.vue` `tone="warn"` | `expected 'warn' to be 'info' // Object.is equality` | *«sends Invoke with the registry's literals on a change, …»*; `Tests  1 failed \| 16 passed (17)` | `cmp` uguale |

Alla fine, i nove file contro le copie: `cmp` uguale per ciascuno. Dalla radice, `git status --porcelain | diff
<scratchpad>/task6bis/prima.txt -` → le **19** righe dei file del compito e nient'altro (17 ` M`, 2 `??`: `BaseNotice.vue` e
`frame.browser.test.ts`); `git status --porcelain | diff <scratchpad>/task6bis/prima-violazioni.txt -` → **nulla**; `git status
--porcelain --ignored | grep -E '\.tmp|__screenshots__'` → nulla: nessun file nato dai rossi (R2-3).

## 5. I fine-riga

Misurati prima di toccare (`<scratchpad>/task6bis/eol-before.txt`) e dopo (`eol-after.txt`): `tr -cd '\r' < f | wc -c` contro
`wc -l`, e `git ls-files --eol`. Su questa macchina ogni file del compito è `i/lf w/lf` con **0 CR**, prima e dopo: la forma non
cambia, cresce solo il numero delle righe.

| File | Prima: CR / righe, colonne | Dopo: CR / righe, colonne |
|---|---|---|
| `docs/superpowers/specs/2026-09-22-design-system-tavole/token.html` | 0 / 644, `i/lf w/lf` | 0 / 748, `i/lf w/lf` |
| `gui/src/tokens/base.css` | 0 / 96, `i/lf w/lf` | 0 / 159, `i/lf w/lf` |
| `gui/src/tokens/themes.css` (dettato invariato) | 0 / 107, `i/lf w/lf` | 0 / 107, `i/lf w/lf` — lo stesso `sha1` |
| `gui/src/testing/probes.ts` | 0 / 158, `i/lf w/lf` | 0 / 163, `i/lf w/lf` |
| `gui/src/testing/probes.browser.test.ts` | 0 / 92, `i/lf w/lf` | 0 / 107, `i/lf w/lf` |
| `gui/vite.config.ts` | 0 / 122, `i/lf w/lf` | 0 / 125, `i/lf w/lf` |
| `gui/src/tokens/tokens.browser.test.ts` | 0 / 132, `i/lf w/lf` | 0 / 162, `i/lf w/lf` |
| `gui/src/tokens/dock.css` | 0 / 164, `i/lf w/lf` | 0 / 166, `i/lf w/lf` |
| `gui/src/components/BaseDialog.vue` | 0 / 120, `i/lf w/lf` | 0 / 120, `i/lf w/lf` |
| `gui/src/components/icons.ts` | 0 / 73, `i/lf w/lf` | 0 / 83, `i/lf w/lf` |
| `gui/src/components/kit.test.ts` | 0 / 272, `i/lf w/lf` | 0 / 318, `i/lf w/lf` |
| `gui/src/kit/Kit.vue` | 0 / 266, `i/lf w/lf` | 0 / 287, `i/lf w/lf` |
| `gui/src/kit/kit.browser.test.ts` | 0 / 156, `i/lf w/lf` | 0 / 206, `i/lf w/lf` |
| `gui/src/panels/Settings.vue` | 0 / 98, `i/lf w/lf` | 0 / 100, `i/lf w/lf` |
| `gui/src/panels/Status.vue` | 0 / 65, `i/lf w/lf` | 0 / 81, `i/lf w/lf` |
| `gui/src/panels/modules.test.ts` | 0 / 322, `i/lf w/lf` | 0 / 334, `i/lf w/lf` |
| `gui/src/frame/frame.test.ts` | 0 / 252, `i/lf w/lf` | 0 / 271, `i/lf w/lf` |
| `gui/src/frame/Band.vue` (riscritto) | 0 / 42, `i/lf w/lf` | 0 / 41, `i/lf w/lf` — il terminatore di prima, LF |
| `gui/src/components/BaseNotice.vue` (nuovo) | — | 0 / 89, `i/lf w/lf` dopo `git add` |
| `gui/src/frame/frame.browser.test.ts` (nuovo) | — | 0 / 92, `i/lf w/lf` dopo `git add` |

`git diff --check` pulito prima del commit; i due file nuovi finiscono con un `\n` e non hanno spazi in coda.

## 6. Il cancello

Due corse, **da sole**, in background, e nient'altro mentre giravano.

| Corsa | Esito | `gui:` jsdom | `gui:` browser | Pezzo JavaScript | Audit |
|---|---|---|---|---|---|
| Passo 1, sull'albero a `9579ca0` | `GATE GREEN`, 78 s | `Test Files  19 passed \| 1 skipped (20)`, `Tests  137 passed \| 1 skipped (138)` | `Test Files  5 passed (5)`, `Tests  48 passed (48)` | `690.61 kB │ gzip: 210.41 kB` | `found 0 vulnerabilities` |
| prima del commit, sull'albero del compito | `GATE GREEN`, 78 s | `Test Files  19 passed \| 1 skipped (20)`, `Tests  144 passed \| 1 skipped (145)` | `Test Files  6 passed (6)`, `Tests  56 passed (56)` | `691.82 kB │ gzip: 210.77 kB` | `found 0 vulnerabilities` |

144 + 56 = 200 prove, 19 + 6 = 25 file: i conti del Passo 14. Poi, da solo, `bash scripts/check-docs.sh` → uscita 0, *«OK — no
inconsistencies.»*

## 7. Le divergenze, e ciò che non ho misurato

**Nessuna voce candidata**: la prossima libera resta **E70**. Ogni uscita del compito e del §3 del dispaccio si è riprodotta, cifra
per cifra — le sei cadute del Passo 3, le cinque del Passo 5 con `scrollbar` 17 volte, i conti dei Passi 6–14, il pezzo
JavaScript `691.82 kB`/`210.77 kB`, le sedici righe del Passo 15. E il confronto del coordinatore, `compare_task6bis.py`, rende
0 file diversi dal testo del piano. Il Passo 6 ha mostrato le barre al progetto browser (D26): nessun'altra prova ha cambiato
esito — verdi le 42 del Passo 7 e le 200 del Passo 14, cinque volte su cinque.

Tre osservazioni, **non** divergenze, per chi rilegge:
1. La tabella del Passo 15 dà il solo messaggio del chiaro per le righe 8 e 9; quelli dello scuro misurati sono nel §4 — riga 8
   `[ 'rgb(47, 27, 29)', …(2) ]`, riga 9 `expected 'rgb(229, 158, 161)' to be 'rgb(236, 230, 218)'`.
2. Chi rilancia i comandi del Passo 1 **dopo** il compito, come verifica, trova `grep -c '1.5' gui/src/testing/probes.ts` → `1`,
   non `0`: è il commento di `SLACK`, *«1.5 kept green a card's radius …»*, dettato dal Passo 3. Le soglie sono tutte `SLACK`.
3. Cercando i commenti che il compito avrebbe potuto rendere falsi (gotcha #58), sull'albero del compito: `grep -rn -E 'card of
   20|frame of 32|cards of 20|20 and 32'` in `gui/` → nulla; `grep -rn -i -E '\beight\b|\botto\b' gui/src gui/eslint.config.js`
   → solo il commento nuovo di `kit.test.ts` e le *«eight moves»* di SP-8 in `frame/dock.ts` e `frame/VueContent.ts`, che non
   contano pezzi; il commento di `concentricRadii` su *«a 24 px bar's `20px 20px 0 0` stays 20 … the dock's floating title bar»*
   resta vero, perché la barra è ora `calc(21 - 1)`, cioè 20.

**Non misurato, e perché:**
- ⛔ **il Passo 16** (E69): lo sguardo è del proprietario col coordinatore, dopo la revisione — la barra col mouse vero, i raggi,
  i messaggi, la fascia prima e dopo `harnessFake.deliverAll()`, il frammento del contrasto. Non ho aperto la SPA né la pagina
  kit, e nessun server di sviluppo è stato acceso;
- il **cursore acceso** della barra: la prova legge l'innesco `--scrollbar-trigger`, non il disegno (controllo 22); la preferenza
  di Windows che tiene sempre visibili le barre e i colori forzati restano assunti;
- **la CI**, che legge il coordinatore dopo il suo push;
- l'altra macchina, `zagor`: tutto qui è della macchina `Jays`, Chrome `154.0.8037.58`, densità 1 del progetto browser.

## 8. Il tempo, e i log

Dalla verifica d'avvio, alle 15:03 circa del 2026-09-27, al commit `da7a522` alle 15:19:03; le tre corse di stabilità finite
alle 15:19:54; questo rapporto subito dopo. Circa **20 minuti**, di cui 2 × 78 s di cancello.

I log del cancello:
- `C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\2f940fbc-999d-4486-9faf-6d913c767183\scratchpad\task6bis\gate-step1-20260927-150555.log`
- `C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\2f940fbc-999d-4486-9faf-6d913c767183\scratchpad\task6bis\gate-final-20260927-151651.log`

Nella stessa cartella: `check-docs.log`, i log dei passi (`step1-build.log` … `step14-lint.log`), delle sedici violazioni
(`v1.log` … `v16.log`, `v2-board.log`) e della stabilità (`stability-1…3.log`); gli script (`replace_unique.py`,
`cure_board.py`, `extract_tokens.py`, `extract_blocks.py`, `dry_run.py`, `create.py`, `rewrite.py`, `violate.py`, `apply.sh`,
`v.sh`, `restore.sh`), i recinti estratti in `blocks/`, le copie salvate in `copies/`, `prima.txt`, `prima-violazioni.txt`,
`eol-before.txt`, `eol-after.txt` e `commit-msg.txt`. Niente di tutto questo sta nel repository.
