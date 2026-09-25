# Il rapporto del compito 4 — la pagina kit: fuori dal pacchetto, e le sonde diventano prove nel browser

Implementatore del compito 4, macchina `Jays`, dalle 23:53 del 2026-09-25 alle 00:18 del 2026-09-26 (la data scritta nel
piano è **2026-09-25**, come dettato). Il mandato: `dispatch-task-4.md`; il compito: `task-4-brief.md`, letto per intero.
Nessun subagente.

## 1. Lo stato

**DONE_WITH_CONCERNS** — il commit è **`841dc54`** (`841dc541cc393d42e85526d5793c000dc7199d60`), sopra `1b531e9`, **senza
co-autore** (`git log -1 --format=%B | grep -ci co-authored` → `0`), **non pushato**: il push è del coordinatore, dopo la
revisione. Ogni Atteso del compito è tornato, e i nuovi file sono **byte per byte** il dettato del brief (`cmp` con una
seconda estrazione, §3).

La riserva è **una voce candidata Nit, E28** (§8): il Passo 4 inserisce il blocco `harness/kit-page-specimens` **sotto** il
commento di testa *«THE IMPORT RULES OF THE KIT»*, che ora introduce un blocco che non è una regola d'import. Seguito il
dettato, non corretto in silenzio. E due note per chi guarda la pagina col pannello del browser (§7): le frecce sintetiche
non scelgono un radio di `reka-ui`, e il pannello nascosto rende a volte fotogrammi vecchi.

## 2. `git show --stat HEAD`

```
commit 841dc541cc393d42e85526d5793c000dc7199d60
Author: devfrx <zagor2012@icloud.com>
Date:   Sat Sep 26 00:14:32 2026 +0200

    design-system(compito 4): la pagina kit -- fuori dal pacchetto, e le sonde delle tavole diventano prove nel browser. [...] check-docs OK, GATE GREEN

 docs/superpowers/plans/2026-09-23-design-system.md |   4 +-
 gui/eslint.config.js                               |   7 +
 gui/kit.html                                       |  12 +
 gui/src/kit/Kit.vue                                | 266 +++++++++++++++++++++
 gui/src/kit/kit.browser.test.ts                    | 151 ++++++++++++
 gui/src/kit/main.ts                                |   8 +
 gui/src/testing/probes.ts                          | 126 ++++++++++
 gui/vite.config.ts                                 |   8 +
 scripts/gate-gui.sh                                |   5 +
 9 files changed, 585 insertions(+), 2 deletions(-)
```

Il messaggio intero è nel commit (`git log -1 --format=%B 841dc54`). I nove file sono **solo** quelli del punto 2 del
contratto; prima del commit `git status --porcelain` nominava quelli e nient'altro, dopo è vuoto.
`git diff --stat 1b531e9..HEAD -- crates/ gui/schema/` → vuoto (vincolo 12).

Nel piano, le **due celle** e basta: riga **3**, colonna Commit `` `c7b7bcd`, con la cura `9d2ffb0` `` (R1-16); riga **4**,
Stato `✅ 2026-09-25`, Commit resta `—`.

## 3. Passo per passo, coi comandi e le uscite vere

### L'avvio, e il cancello d'apertura

| Verifica | Uscita |
|---|---|
| `git rev-parse --short HEAD` | `1b531e9` |
| `git status --porcelain` | vuoto |
| `node --version` | `v24.19.0` — `engines`: `^22.22.2 \|\| ^24.15.0 \|\| >=26.0.0` |
| `git config --show-origin --get-all core.autocrlf` | tre righe: `true` da `C:/Program Files/Git/etc/gitconfig`, `false` da `C:/Users/Jays/.gitconfig`, `false` da `.git/config` — vale l'ultima, come nel §0 |
| Chrome, dal nome della cartella | `154.0.8037.58` in `/c/Program Files/Google/Chrome/Application/`; la cartella sotto `$LOCALAPPDATA` non c'è, e il comando esce **2** |

**Il cancello d'apertura**, a `1b531e9`, da solo, in background (73 s): `GATE GREEN.`; `jsdom` `Test Files  18 passed | 1
skipped (19)`, `Tests  125 passed | 1 skipped (126)`; `browser` `Test Files  1 passed (1)`, `Tests  5 passed (5)`;
`dist/assets/index-CiZv4zPX.js  663.93 kB │ gzip: 201.53 kB`; `found 0 vulnerabilities`. Gli stessi numeri del coordinatore.

### Come ho scritto i file

Nessun testo ricopiato a mano: uno script nello scratchpad, `extract.py`, copia i recinti del brief — per marcatore e
ordinale — nei file nuovi (LF, `newline=""`, temporaneo più `os.replace`) e nei testi *Trova*/*Sostituisci* dei tre file
modificati, che `replace_unique.py` (copiato dagli *Strumenti* del brief) applica. Dopo il commit, una seconda estrazione in
una cartella a parte: `cmp` uguale per i cinque file nuovi; e il testo del compito 4 nel brief si ritrova **alla lettera** nel
piano (`task in plan` → `True`, 35 226 caratteri).

### Passo 1 — `gui/src/testing/probes.ts`

Creato dal recinto del brief, 126 righe, LF.

### Passo 2 — la riga di E24, poi la prova, rossa

`gui/vite.config.ts` con `optimizeDeps: { force: true }` e il suo commento, **prima** della prova (`ok: gui/vite.config.ts
(LF)`, +8 righe); poi `gui/src/kit/kit.browser.test.ts`, 151 righe.

`(cd gui && npx vitest run --project browser src/kit)` → **exit 1**:

```
Forced re-optimization of dependencies
(!) Failed to run dependency scan. Skipping dependency pre-bundling. [...] Could not resolve './Kit.vue' in src/kit/kit.browser.test.ts
Internal server error: Failed to resolve import "./Kit.vue" from "src/kit/kit.browser.test.ts". Does the file exist?
 FAIL  |browser (chromium)| src/kit/kit.browser.test.ts [ src/kit/kit.browser.test.ts ]
 Test Files  1 failed (1)
      Tests  no tests
```

### Passo 3 — la pagina

`gui/kit.html` (12 righe), `gui/src/kit/main.ts` (8), `gui/src/kit/Kit.vue` (266), dai recinti del brief, LF.

### Passo 4 — il blocco del linter

`ok: gui/eslint.config.js (LF)`, +7 righe. Vedi E28 al §8 per **dove** lo mette il *Trova*.

### Passo 5 — verde alla prima corsa dopo il rosso

`(cd gui && npx vitest run --project browser && npm run lint && npm run build)` → **exit 0**:

```
Forced re-optimization of dependencies
 Test Files  2 passed (2)
      Tests  23 passed (23)
> eslint src                       (nessuna riga: verde)
✓ 746 modules transformed.
dist/assets/index-Bx6RaF8P.css                             145.10 kB │ gzip:  14.04 kB
dist/assets/index-CiZv4zPX.js                              663.93 kB │ gzip: 201.53 kB
```

**E24 tenuta**: nessun `'set' on proxy`, nessun *«unexpectedly reloaded»*. Il pezzo JavaScript ha **lo stesso nome** e la
stessa misura del `main` su questa macchina: la SPA non è toccata. Con `--reporter=verbose`, le nove prove per tema, tutte ✓
(le cinque del compito 2 invariate).

**I conteggi delle guardie** — trappola 1: *si guardano i conteggi prima del verdetto*. Misurati con una prova usa-e-getta
scritta per fallire (il `console.log` di una prova verde non arriva al terminale), copiata in `src/kit/`, lanciata da sola e
tolta, `git status --porcelain` com'era prima. Uguali nei due temi, viewport 1440 × 900:
`roots=9 near=12 bad=0 seen=87 boxed=87 problems=0 icons=47 centred=47 iconProblems=0 off=primary/secondary/quiet
dialogNear=1 dialogBad=0`. ⚠️ La finestra confronta **una** coppia sola, il `Consenti` nel suo angolo in basso a destra:
è la coppia che la riga 1 del Passo 7 fa cadere.

### Passo 6 — la pagina fuori dal pacchetto, nelle due direzioni

`scripts/gate-gui.sh` → `ok: scripts/gate-gui.sh (LF)`, +5 righe (il modo nell'indice resta `100644`, e `git diff` non
dichiara un cambio di modo).

**Rossa** — in `gui/vite.config.ts`, dopo il blocco `define`, `build: { rolldownOptions: { input: { index: "index.html",
kit: "kit.html" } } },` (copia salvata prima); `bash scripts/gate-gui.sh`, da solo, in background → **exit 1** in 21 s:

```
dist/index.html                                              0.53 kB │ gzip:   0.30 kB
dist/kit.html                                                0.54 kB │ gzip:   0.31 kB
dist/assets/kit-CP1QMCqd.css                                 9.67 kB │ gzip:   1.88 kB
dist/assets/kit-Bw1CQ6GG.js                                 31.78 kB │ gzip:  10.42 kB
dist/assets/index-Dwx_PUol.js                              521.80 kB │ gzip: 150.40 kB
the kit page is in the package
```

**Verde** — `gui/vite.config.ts` tornato dalla copia salvata, `cmp` uguale, `git status --porcelain` uguale a `prima.txt`;
`bash scripts/gate-gui.sh` → **exit 0** in 51 s: `dist/assets/index-CiZv4zPX.js  663.93 kB`; `jsdom` `Test Files  18 passed
| 1 skipped (19)`, `Tests  125 passed | 1 skipped (126)`; `browser` `Test Files  2 passed (2)`, `Tests  23 passed (23)`; lint
verde; `found 0 vulnerabilities`.

### Passo 7 — al §4

### Passo 8 — al §7; poi il cancello e il commit

`bash scripts/gate.sh`, da solo, in background (00:12–00:14, 78 s) → `GATE GREEN.`, `documentation consistency` →
`OK — no inconsistencies.`; poi `bash scripts/check-docs.sh`, da solo → exit 0, `OK — no inconsistencies.`; poi il commit.

## 4. Le due tabelle del Passo 7

`prima.txt` preso **prima della prima violazione** (quella rossa del Passo 6) e copiato ogni file che le violazioni toccano:
`BaseButton.vue`, `BaseList.vue`, `BaseLabel.vue`, `BaseTextField.vue`, `BaseStatus.vue`, `themes.css`, `Kit.vue`,
`kit.browser.test.ts`, `eslint.config.js`, `vite.config.ts`. Ogni violazione con `replace_unique.py`, una per volta, da uno
script (`v7.sh`) che rifiuta se il file non è uguale alla sua copia **prima**, mostra il `diff` della violazione, lancia, rimette
la copia e fa `cmp`: **uguale dopo ciascuna**. Alla fine `git status --porcelain | diff prima.txt -` → **nulla**, e nessun
`__screenshots__` né `.png` sotto `gui/src`.

### Le sonde — `(cd gui && npx vitest run --project browser)`

| # | La violazione | Esito | Le prove che cadono | Il messaggio rosso vero |
|---|---|---|---|---|
| 1 | `BaseButton.vue`, `border-radius: var(--radius-card)` | `2 failed \| 21 passed (23)` | *«opens its window with the radii concentric, and no axe violation»*, chiaro e scuro | `"base-button in base-dialog, bottom-right: radius 16.0, outer 20.0, distance 13.0/13.0"` |
| 2 | `BaseList.vue`, righe a `var(--radius-card)` | `2 failed \| 21 passed (23)` | *«keeps every radius concentric (answer 4)»*, chiaro e scuro | `"base-list-row in kit-card, bottom-left: radius 16.0, outer 20.0, distance 13.0/13.0"` e lo stesso `bottom-right` |
| 3 | `Kit.vue`, `.kit-note` con `white-space: nowrap; overflow: hidden;` | `2 failed \| 21 passed (23)` | *«cuts no text, and lets nothing stick out of its box»*, chiaro e scuro | `"cut: kit-note \"Principale, normale, dis\" 490>422"` — la sola nota lunga, a 1440 px |
| 4 | `Kit.vue`, `.kit-strip > .base-button { margin-right: -40px; }` | `4 failed \| 19 passed (23)` | *«cuts no text…»* e *«has no axe violation -- contrast included, on the drawn page»*, chiaro e scuro | `"sticks out: base-button of kit-strip"`; e `AssertionError: expected 1 to be +0` su `expect(judged.incomplete).toBe(0)` — la guardia di R3-7 |
| 5 | `BaseLabel.vue`, `.base-label :deep(.base-icon) { position: relative; top: 3px; }` | `2 failed \| 21 passed (23)` | *«draws every icon in currentColor, and centres it»*, chiaro e scuro | quindici righe, da `"off centre by 3.00 px: modules in base-label"` |
| 6 | `Kit.vue`, `.kit` con `font: 400 0.875rem/1.25rem serif` | `2 failed \| 21 passed (23)` | *«dresses labels and numbers in Barlow, and the text in Geist»*, chiaro e scuro | `AssertionError: expected 'serif' to be 'Geist Variable'` |
| 7 | `themes.css`, `--color-text-muted` dello scuro a `var(--ref-neutral-39)` | `2 failed \| 21 passed (23)` | le **due** di `axe` nello **scuro** soltanto: la pagina e la finestra | `"color-contrast: #v-0-legend, .kit-card:nth-child(1) > h2, …"` e `"color-contrast: #reka-dialog-description-v-15, .actions > .base-button[data-variant=\"quiet\"]…"`. E `(cd gui && npx vitest run --project jsdom src/tokens)` → `Tests  2 failed \| 13 passed (15)`: *«themes.css is the board's block, byte for byte»* (`board.test.ts`) e *«dark: every pair reads at its threshold (WCAG 2.2, 1.4.3 and 1.4.11)»* (`contrast.test.ts`), come dice R3-8 |
| 8 | `BaseList.vue`, `var(--radius-card)` **e** `margin-inline: var(--space-1);` | `2 failed \| 21 passed (23)` | *«keeps every radius concentric (answer 4)»*, chiaro e scuro | `"base-list-row in kit-card, bottom-left: radius 16.0, outer 20.0, distance 17.0/13.0"` e lo stesso `bottom-right` |
| 8b | `BaseList.vue`, la sola `margin-inline: var(--space-1);`, raggio giusto | **`Tests  23 passed (23)`** | nessuna | — fuori dall'angolo un raggio più piccolo è ammesso (E27) |
| 9 | `BaseButton.vue`, tolta la regola del `quiet` spento (E19) | `2 failed \| 21 passed (23)` | *«draws every button that is off in the disabled colour, whatever its variant (E19)»*, chiaro e scuro | scuro: atteso `"quiet: rgb(111, 102, 96)"`, ricevuto `"quiet: rgb(163, 154, 143)"`; chiaro: atteso `"quiet: rgb(163, 154, 143)"`, ricevuto `"quiet: rgb(101, 91, 87)"` |
| 10 | `BaseTextField.vue`, `.frame:hover:not([data-disabled])` | `2 failed \| 21 passed (23)` | *«keeps the error's border under the pointer (E20)»*, chiaro e scuro | scuro `expected 'rgb(163, 154, 143)' to be 'rgb(129, 27, 7)'`; chiaro `expected 'rgb(101, 91, 87)' to be 'rgb(251, 182, 168)'` |
| 11 | `kit.browser.test.ts`, tolta `await userEvent.hover(frame as HTMLElement);` | `2 failed \| 21 passed (23)` | la stessa prova, chiaro e scuro, alla guardia | `AssertionError: expected false to be true`, su `matches(":hover")` |

Ogni riga cade **solo** sulle prove che la tabella del piano nomina — la riga 4 anche su `axe`, come la tabella dice — e
mai il progetto intero: E25 tiene, nessun `TimeoutError` a catena.

### Il linter — `(cd gui && npm run lint)` (E26)

| # | La violazione | Il messaggio rosso vero |
|---|---|---|
| L1 | tolto il blocco `harness/kit-page-specimens` | `✖ 33 problems (33 errors, 0 warnings)`, tutti in `src/kit/Kit.vue`, tutti `@intlify/vue-i18n/no-raw-text`: il primo `56:11  error  raw text 'Il kit' is used`, l'ultimo `151:41  error  raw text 'Icone' is used` |
| L2 | col blocco, in `BaseStatus.vue` `role="status">ciao<slot />` | `11:42  error  raw text 'ciao' is used  @intlify/vue-i18n/no-raw-text`, `✖ 1 problem` |
| L3 | col blocco, in testa allo `<script>` di `Kit.vue` `import { Search } from "lucide";` | `2:1  error  'lucide' import is restricted from being used. icons pass through BaseIcon and the one map, src/components/icons.ts (answer 11 of the design system)  no-restricted-imports`, `✖ 1 problem` |

## 5. I fine-riga

Misurati prima di toccare e dopo il commit, con `git ls-files --eol` e `tr -cd '\r' < f | wc -c` contro `wc -l`. Su questa
macchina `core.autocrlf` è `false`: tutto è `w/lf`.

| File | Prima | Dopo |
|---|---|---|
| `gui/eslint.config.js` | `i/lf w/lf`, CR 0, 143 righe | `i/lf w/lf`, CR 0, 150 righe |
| `gui/vite.config.ts` | `i/lf w/lf`, CR 0, 114 righe | `i/lf w/lf`, CR 0, 122 righe |
| `scripts/gate-gui.sh` | `i/lf w/lf`, CR 0, 78 righe | `i/lf w/lf`, CR 0, 83 righe |
| `docs/superpowers/plans/2026-09-23-design-system.md` | `i/lf w/lf`, CR 0, 8807 righe | `i/lf w/lf`, CR 0, 8807 righe |
| `gui/kit.html` | nuovo | `i/lf w/lf`, CR 0, 12 righe |
| `gui/src/kit/main.ts` | nuovo | `i/lf w/lf`, CR 0, 8 righe |
| `gui/src/kit/Kit.vue` | nuovo | `i/lf w/lf`, CR 0, 266 righe |
| `gui/src/kit/kit.browser.test.ts` | nuovo | `i/lf w/lf`, CR 0, 151 righe |
| `gui/src/testing/probes.ts` | nuovo | `i/lf w/lf`, CR 0, 126 righe |

I sei file che le violazioni toccano e rimettono — `BaseButton.vue`, `BaseList.vue`, `BaseLabel.vue`, `BaseTextField.vue`,
`BaseStatus.vue`, `themes.css` — erano `i/lf w/lf`, CR 0, e sono tornati identici (`cmp`); non sono nel commit.

## 6. Il passo web, nel cancello finale

`bash scripts/gate.sh` sull'albero che il commit `841dc54` ha poi fissato — `git status --porcelain` nominava i soli nove
file —, 00:12–00:14:

```
dist/assets/index-CiZv4zPX.js                              663.93 kB │ gzip: 201.53 kB
 Test Files  18 passed | 1 skipped (19)          (--project jsdom)
      Tests  125 passed | 1 skipped (126)
 Test Files  2 passed (2)                        (--project browser)
      Tests  23 passed (23)
found 0 vulnerabilities
GATE GREEN.
```

## 7. Il Passo 8 — che cosa ho guardato

`(cd gui && npm run dev -- --strictPort)` in background (Vite 8.3.0, `http://localhost:5173/`), la pagina aperta nel
pannello del browser di questa sessione. Guardata a grandezza vera nella larghezza del pannello, 800 px, due colonne,
un'immagine per pixel; e a 1440 × 900, la misura delle prove, tre colonne, in immagine ridotta. Il server fermato **prima**
del cancello: il `TaskStop` ha chiuso la shell ma non il `node` figlio, rimasto in ascolto sulla 5173 — fermato per PID dopo
averne letto la riga di comando (`vite\bin\vite.js --strictPort`); dopo, nessun `node` di `gui/`.

- **I tre temi dalla scelta in cima**: *Sistema* segue il sistema — scuro su questa macchina —, *Chiaro* e *Scuro*. Nei due
  temi: i tre pulsanti spenti nel colore dello spento, il `quiet` compreso (E19); il campo con l'errore col bordo e la riga
  rossi; le carte nella cornice coi raggi concentrici e il segno tutto intorno a *Home*; la pillola nella pillola; le
  ventiquattro icone col loro nome.
- **La finestra, con la tastiera vera del pannello**, nei due temi: Invio sul pulsante la apre, `role="dialog"`,
  `aria-describedby` sulla descrizione, il fuoco su *Rifiuta*; Tab → *Consenti*, col contorno `solid 2px` → di nuovo
  *Rifiuta*: il fuoco resta dentro; Esc la chiude e il fuoco torna su *Apri la finestra*; il `body` prende
  `pointer-events: none` da aperta e `auto` da chiusa.
- ⚠️ **I radio, e una trappola per chi guarda col pannello.** Con il tasto del pannello, `ArrowDown` sul radio *Chiaro* sposta
  il fuoco su *Scuro* **senza sceglierlo**. La causa è nel pacchetto installato, `node_modules/reka-ui/dist/RadioGroup/
  RadioGroupItem.js` (2.10.4): la scelta avviene in un `setTimeout(0)` dopo il fuoco, **solo se la freccia è ancora giù** —
  `isArrowKeyPressed`, rimesso a `false` dal `keyup` —, e il pannello manda `keydown` e `keyup` di fila. Con `keydown` e
  `keyup` a 60 ms l'uno dall'altro, da uno script nella pagina, la scelta segue: *Chiaro* → *Scuro* (tema scuro) → *Sistema*
  → *Scuro*; in *Scelta*, *OpenRouter* → *Locale* → *OpenRouter*; il gruppo spento ha i radio `disabled` e `tabIndex -1`.
  Non è un difetto del kit — un tasto vero resta giù più di un giro di timer —, ma il revisore, col pannello, vedrebbe lo
  stesso. **Non misurato**: se `userEvent.keyboard('{ArrowDown}')` di Vitest tenga il tasto abbastanza; conta per una prova
  futura che usi le frecce su un radio di `reka-ui` (Impostazioni, compito 5).
- ⚠️ **Il pannello nascosto** dà a volte un fotogramma vecchio: subito dopo un cambio di tema, due immagini mostravano colori a
  metà transizione (i pulsanti `secondary` pallidi nel chiaro, le carte chiare nello scuro); `getComputedStyle` rendeva i
  colori giusti, `document.getAnimations()` era vuoto, e l'immagine dopo era giusta. Nessun difetto della pagina.
- Una nota di sola vista, **non** una voce: nella scheda *Stato* il `<p>` dentro `BaseStatus` tiene i margini del browser,
  16 px, e lo spazio sopra la frase è più largo dei 12 delle altre schede. È un esemplare della pagina, e nessuna sonda lo
  giudica.

## 8. Le divergenze, e ciò che non ho misurato

### Voce candidata **E28** — Nit — Compito 4, Passo 4: il blocco finisce sotto la testa delle regole d'import

Il *Trova* del Passo 4 è `  {\n    name: "harness/imports",`, cioè **dopo** il commento di testa che introduce i blocchi
d'import. A `841dc54`, `sed -n '112,124p' gui/eslint.config.js`:

```js
  /**
   * ⛔ THE IMPORT RULES OF THE KIT (design system, section (b)). One rule, a scope per block, and the ORDER MATTERS: in a flat
   * config a later block REPLACES an earlier one's options for the same rule, it does not merge them -- so every block
   * says the whole list for its files.
   */
  {
    // ⛔ THE KIT PAGE'S WORDS ARE SPECIMENS (D8 of the design-system plan): a development page outside the package, whose
    // words in `it.json` would ship for nothing. The one exception to the raw-text rule, in one place, like Chat's above.
    name: "harness/kit-page-specimens",
    files: ["src/kit/**"],
    rules: { "@intlify/vue-i18n/no-raw-text": "off" },
  },
  {
    name: "harness/imports",
```

Il commento *«THE IMPORT RULES … One rule, a scope per block»* ora introduce un blocco che spegne `no-raw-text`, e il
*«like Chat's above»* del blocco nuovo rimanda a un blocco che sta sopra la testa. La specie del gotcha **#58**, come E17 ed
E22 nello stesso file. **Nessun effetto sul comportamento**: le tre righe della seconda tabella del Passo 7 sono rosse come
dettato, `npm run lint` verde. **La cura proposta**: un *Trova* che ancori il blocco **prima** della testa — la fine del
blocco di `Chat.vue`, `    rules: { "vue/no-v-html": "off" },\n  },` —, così il blocco sta fra le due eccezioni e la testa
torna sopra i soli blocchi d'import. Ho seguito il dettato; la scelta è del coordinatore.

### Nessun'altra divergenza

Ogni numero del §3 del mandato è tornato uguale: il rosso del Passo 2, i 2 file e 23 prove del Passo 5, `index-CiZv4zPX.js`
`663.93 kB`, le due direzioni del Passo 6, i messaggi del Passo 7 — `base-button in base-dialog, bottom-right: radius 16.0,
outer 20.0, distance 13.0/13.0` e `base-list-row in kit-card, bottom-left: radius 16.0, outer 20.0, distance 17.0/13.0`
compresi —, i trentatré `raw text … is used`, `raw text 'ciao' is used`, *«'lucide' import is restricted from being used»*.

### Ciò che non ho misurato

- La ragione `elmPartiallyObscured` della riga 4: ho visto il rosso `expected 1 to be +0` della guardia, non ho letto la
  voce `incomplete` di `axe`.
- Le prove in più del pre-controllo per E27 — il ramo reso cieco con `: true`, e il `throw` —: non rifatte. Che sia il ramo
  *fuori dall'angolo* a dare il rosso della riga 8 lo dice il codice della sonda: con `17.0/13.0`, `|dx − dy| = 4 > 1,5`.
- `userEvent.keyboard` di Vitest sulle frecce di un radio (§7).
- La CI: è del coordinatore, dopo il push.

## 9. Il tempo, e i log

Dalle 23:53 del 2026-09-25 alle 00:14 del 2026-09-26 il commit, circa ventuno minuti; poi il rapporto, fino alle 00:18. Tutti nello scratchpad
`C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\85fa78d3-ac8f-4098-9b47-eaa6f1c6ea46\scratchpad\task4\`:

| Log | Che cosa |
|---|---|
| `gate-open-20260925-2354.log` | il cancello d'apertura, `GATE GREEN`, 73 s |
| `step2-red.log` | il rosso del Passo 2 |
| `step5.log`, `step5-verbose.log` | il Passo 5, e l'elenco delle 23 prove |
| `diag-counts.log` | i conteggi delle guardie, dalla prova usa-e-getta |
| `gate-gui-red-20260926-0000.log` | il passo web rosso del Passo 6, 21 s |
| `gate-gui-green-20260926-0002.log` | il passo web verde del Passo 6, 51 s |
| `v7-01.log` … `v7-11.log`, `v7-08b.log`, `v7-07-jsdom.log`, `v7-L1.log` … `v7-L3.log` | le violazioni del Passo 7 |
| `dev-server.log` | il server del Passo 8 |
| `gate-final-20260926-0020.log` | il cancello prima del commit, `GATE GREEN`, 78 s — il nome porta un'ora stimata: è girato dalle 00:12 alle 00:14 |
| `check-docs-20260926.log` | `check-docs.sh`, `OK — no inconsistencies.` |

Nello scratchpad anche gli attrezzi: `extract.py`, `replace_unique.py`, `plan_cells.py`, `v7.sh`, i testi delle violazioni,
le copie salvate in `saved/` e `prima.txt`. Nulla di tutto questo nel repository.
