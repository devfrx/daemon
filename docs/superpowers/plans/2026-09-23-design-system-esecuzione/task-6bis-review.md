# La revisione del compito 6bis — la cura delle tre voci del Passo 8

> Revisore: subagente fresco, macchina `Jays`, 2026-09-27, dalle 15:24 alle 16:00 circa. Commit rivisto: `da7a522`, sopra
> `9579ca0`. Chrome `154.0.8037.58` (nome della cartella, e `browser.version()` di Playwright), Node v24.19.0; memoria libera
> 5,9–6,1 GB su 31,2 durante tutta la revisione. Clone delle mutazioni: `C:\Users\Jays\AppData\Local\Temp\rv6bis`, lasciato
> a `da7a522`, pulito. Log e schermate: `<sp>` = `C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\2f940fbc-999d-4486-9faf-6d913c767183\scratchpad\review6bis`.

## 1. Il verdetto

**Conforme.** Il commit è il testo del piano — `compare_task6bis.py` provato nelle due direzioni —, ogni rosso e ogni verde dei
Passi 1–15 si riproduce cifra per cifra, il cancello è verde, e nessun rilievo è critico o importante: **due minori e due nit**,
tutti del **dettato**, come voci d'errata candidate **E70**–**E73**.

## 2. I rilievi

**Critico:** nessuno. **Importante:** nessuno — ogni affermazione misurabile del rapporto è tornata (§3).

### M-1 — Minore — E70 candidata: la fascia del timbro diverso **sulla pagina** non la tiene nessuna prova

- **Dove.** `gui/src/frame/Band.vue` — `grep -n -e 'class="band"' -e 'on-page' gui/src/frame/Band.vue` → righe 23 e 27 (il ramo
  `v-if="connection.phase === 'stale'"`, riga 22) e 29 (il ramo che aspetta). La prova del ramo `stale` è *«stops the window
  when the core speaks another protocol…»* di `gui/src/frame/frame.test.ts`, riga 251: guarda tono, titolo, timbro e
  l'assenza del pulsante, non il posto. La prova nel browser, `frame.browser.test.ts`, misura **solo** la fascia che aspetta.
- **Evidenza** (nel clone, copia salvata, `cmp` uguale dopo; `<sp>\extra\run.log`, `run-M.log`):
  - tolto `on-page` dal ramo `stale` → `npx vitest run --project jsdom src/frame/frame.test.ts` → `Tests  14 passed (14)`;
    `npx vitest run --project browser src/frame/frame.browser.test.ts` → `Tests  2 passed (2)`;
  - tolto `class="band"` dal ramo `stale` → gli stessi due verdi, 14 e 2.
  Una fascia `stop` col raggio del controllo (8) o senza i 12 px dai lati passerebbe il cancello. La (d) del disegno — *«i due
  casi»* — e la (b), *«Chi lo usa»*, mettono **tutte e due** le fasce sulla pagina; il controllo 23, alla lettera, misura nel
  browser «la fascia» e resta tenuto: per questo è minore, non importante.
- **Cura proposta** (testo, non applicata). In `frame.test.ts`, nella prova *«stops the window…»*, dopo
  `expect(notice.attributes("data-tone")).toBe("stop");`:

  ```ts
      // ON THE PAGE AS THE WAITING BAND, whose place the browser measures (E60).
      expect(notice.classes()).toContain("band");
      expect(notice.attributes("data-on-page")).toBeDefined();
  ```

  **Misurata nel clone:** verde sul codice, `Tests  14 passed (14)`; con `on-page` tolto, rossa la sola prova,
  `AssertionError: expected undefined to be defined`, `Tests  1 failed | 13 passed (14)`; con `class="band"` tolto, rossa la sola
  prova, `AssertionError: expected [ 'base-notice' ] to include 'band'`. E due righe nella tabella del Passo 15, coi due messaggi.
  La strada più larga — la fascia `stop` misurata nel browser, consegnando `StaleBuild` in `frame.browser.test.ts` — la può
  prendere il compito 8, che estende il file (**E63**).

### M-2 — Minore — E71 candidata: **D25** dice *«Costo: nessuno»*, e un costo c'è

- **Dove.** Il piano, `grep -n '^| \*\*D25\*\*' docs/superpowers/plans/2026-09-23-design-system.md` → riga 312; nel codice,
  `gui/src/testing/probes.ts` righe 36 (*«The same half pixel decides "in the corner"…»*) e 86
  (`const inTheCorner = Math.abs(dx - dy) <= SLACK;`).
- **Evidenza.** Nell'angolo il giudizio è a **due lati** — `Math.abs(inner - (outer - dx)) <= SLACK` —, fuori dall'angolo a
  **uno** — solo il troppo tondo. Abbassando a mezzo pixel la soglia di *«nell'angolo»*, un pezzo a **un pixel** dalla diagonale
  passa alla regola a un lato. Misurato nel clone con un caso a mano, una prova usa-e-getta scritta per fallire e poi tolta
  (`<sp>\zz-review-d25.browser.test.ts`, `<sp>\extra\run.log` righe G e H): un pezzo a 13/14 dall'angolo in basso a sinistra di un
  angolo di 20, raggio **4** (il concentrico sarebbe 7):
  - con la sonda del commit → `{"near":1,"bad":[]}` — accettato;
  - con la sola soglia dell'angolo riportata a 1,5 → `{"near":1,"bad":["piece in outer, bottom-left: radius 4.0, outer 20.0,
    distance 13.0/14.0"]}` — rifiutato.

  E la misura di D25 regge: con la soglia dell'angolo a 1,5 e `SLACK` a 0,5, `npx vitest run --project browser` →
  `Test Files  6 passed (6)`, `Tests  56 passed (56)` (riga C) — nessuna prova tiene quella soglia. **Latente:** una prova
  usa-e-getta che elenca ogni coppia incontrata (`<sp>\zz-review-pairs.browser.test.ts`, `<sp>\pairs.log`) — pagina kit, finestra,
  dock, gruppo staccato, fascia, nei due temi — trova **25** coppie per tema, tutte con `dx = dy` e scarto **0,00**: oggi nessuna coppia
  cambia classe.
- **Cura proposta** (testo; la decisione è del coordinatore, D25 è sua). La colonna del costo di D25: *«Costo: a mezzo pixel un
  pezzo a un pixel dalla diagonale dell'angolo — 13/14 da un angolo di 20 — cade nella regola fuori dall'angolo, che rifiuta solo
  il troppo tondo: un raggio troppo piccolo lì passa, dove la soglia di 1,5 lo rifiutava (misurato dalla revisione del 6bis: 4 a
  13/14, verde a 0,5 e rosso a 1,5). Oggi ogni coppia che le prove incontrano ha le due distanze uguali e scarto 0, e nessuna prova
  tiene la soglia dell'angolo: con 1,5 le 56 prove del browser restano verdi.»* Registrata e non consigliata: una riga in
  `probes.browser.test.ts` che fissi il nuovo verdetto — terrebbe una indulgenza, non una regola.

### N-1 — Nit — E72 candidata: una prova di Stato parla ancora di *«event row»*

- **Dove.** `gui/src/panels/modules.test.ts:70` — `grep -n 'shows no event row'` → *«says the core has not spoken, and shows no
  event row, before anything arrives»*. La prima sostituzione del Passo 12 ha portato l'attesa su `.base-notice` e ha lasciato il
  titolo, mentre il commento di `Status.vue` dice ora *«ONE MESSAGE FOR THE LAST Verdict»* (trappola 21: le prove si riscrivono sul
  pezzo). Non è falso alla lettera — il messaggio porta la classe `event` —, ma nomina la forma di prima.
- **Cura proposta.** Il titolo *«says the core has not spoken, and shows no message, before anything arrives»*; nessun esito cambia.

### N-2 — Nit — E73 candidata, per il pre-controllo del compito 8 (**E63**): due cose che `frame.browser.test.ts` porta con sé

- **(a) L'oracolo di un token in tre copie.** `computed(property, token)` di `gui/src/frame/frame.browser.test.ts:55` è identico,
  riga per riga, a quello di `gui/src/frame/dock.browser.test.ts:58`; `colourOf` di `kit.browser.test.ts:52` è lo stesso oracolo per
  `color`. La ragione di **D29** — *«un secondo modo di fare la stessa cosa»* — vale anche per l'aiutante. Proposta: lo decide il
  compito 8, che estende il file e riusa gli aiutanti — portarli in `gui/src/testing/` o dichiarare la copia —; non ora.
- **(b) Il dock non si dispone.** L'`afterEach` di `frame.browser.test.ts` (righe 24–25) smonta l'app, ma `Frame.vue` non dispone mai
  il dock — il suo `onUnmounted` (riga 30) toglie solo l'ascoltatore della tastiera —, mentre `dock.browser.test.ts` (righe 27–29)
  dispone ogni dock e dice perché: `dockview-core` 8.3.1 tiene i gruppi galleggianti della pagina in una lista di modulo, `+ 2` a
  gruppo. Oggi innocuo — nessun gruppo galleggia in `frame.browser.test.ts`, e le due prove sono verdi nei due temi —; conta se il
  compito 8 vi stacca un gruppo. **Dedotto** leggendo il codice, non misurato.

### Fuori dal commit — preesistenti, misurati **uguali** a `9579ca0`, per lo sguardo del proprietario (non contati)

- **P-1. L'anello del fuoco sulla scatola di Stato si vede solo in alto.** Nella Home a 1440 × 900 la scatola `.status` scorre, e
  Chrome la mette nella sequenza del tabulatore (uno *scroller* senza nulla di focalizzabile dentro): col Tab ci si arriva,
  `:focus-visible`, `solid 2px offset 2px`; ma la scheda ne taglia i lati e il fondo, e resta la sola riga in alto, dopo la presa.
  Uguale a `9579ca0` (`<sp>\look\9579ca0-light-stato-focus.png` contro `da7a522-light-stato-focus.png`). La riga «focus» della (a)
  vuole il contorno *«visibile (2.4.7) e non nascosto (2.4.11)»* e cita 2.4.13: il Passo 16 lo incontra guardando la barra *«col
  fuoco dentro»*.
- **P-2. Il contorno di «Riprova» sulla fascia quasi non si vede.** Il pulsante ha fondo trasparente e bordo `1px` sul fondo del
  tono: nel chiaro `rgb(220, 210, 196)` su `rgb(240, 231, 221)`, **1,22**; nello scuro `rgb(47, 40, 41)` su `rgb(49, 40, 32)`,
  **1,00**; la scritta 14,53 e 11,61. Uguale a `9579ca0`. La (a) tiene `--color-border` per decoro; nello scuro «Riprova» si legge
  come una parola in grassetto (`<sp>\look\zoom-dark-spa-retry.png`).
- **P-3.** La console del server di sviluppo dà un 404 su `/favicon.ico` — `curl` → `404` —: non tocca il compito.

## 3. I comandi rilanciati, con le uscite

| # | Comando | La mia uscita | Contro rapporto / Atteso |
|---|---|---|---|
| 0 | `git rev-parse --short HEAD`; `git status --porcelain`; `git log --oneline 9579ca0..HEAD`; `git status -sb` | `da7a522`; vuoto; una riga; `[ahead 1]` | ✓ |
| 1 | `python …/compare_task6bis.py 9579ca0 da7a522` (dalla radice, `PYTHONIOENCODING=utf-8`) | 20 righe `OK`, *«20 paths, 0 mode changes, 0 not matching the plan's text»*, uscita **0**; né il piano né il disegno nell'uscita | ✓ §1 del rapporto |
| 2 | nel clone, commit usa-e-getta **W**: `(N2 of E60)` → `(N3 of E60)` in `BaseNotice.vue`, poi lo script contro `HEAD` | `DIFFERS gui/src/components/BaseNotice.vue` e nessun altro, *«1 not matching»*, uscita **1** | la prima direzione, **W** |
| 3 | nel clone, commit usa-e-getta **S**: `thin:` → `thim:` nel commento di `--size-scrollbar` in `base.css` | `DIFFERS gui/src/tokens/base.css` e nessun altro — né la tavola né `themes.css` —, uscita **1**; il clone tornato a `da7a522`, pulito | la prima direzione, **S** |
| 4 | `python plan_ops.py <piano> 6bis list`, e il confronto a macchina con la ricetta (`<sp>\recipe_check.py`) | 39 operazioni; le righe del *Trova*, del *Sostituisci* e dei file interi coincidono con la ricetta, passo per passo, 0 differenze; `S 6680 6881` sono i due recinti Python | ✓ |
| 5 | `plan_ops.py … apply <clone>` su `9579ca0` pulito, la tavola e `base.css` da `da7a522`, `git add -A`, `git diff --cached --stat da7a522` | *«39 operations, 0 refused»*; diff **vuoto**; contro `9579ca0`: `19 files changed, 644 insertions(+), 66 deletions(-)` | ✓ i 19 file del commit |
| 6 | Passo 1, a `9579ca0` nel clone: i cinque `grep`/`ls` e `npm run build` | `/* 20 */` e `/* 32 */` senza `--border-width`; `0` e `0`; `launchOptions: { channel: "chrome" }`; `2`; *No such file* per i due; `index-CjDBuiZn.js 690.61 kB │ gzip: 210.41 kB`, `2631 modules`, CSS `154.72 kB │ gzip: 15.46 kB` | ✓ |
| 7 | Passo 2: `npx vitest run --project browser src/testing/probes.browser.test.ts` | rossa la sola *«sees the pixel of a border…»*, `expected { near: 2, bad: [] } to deeply equal { near: 2, bad: [ …(2) ] }`, `Tests  1 failed \| 7 passed (8)` | ✓ |
| 8 | Passo 3: i tre file del browser | `Tests  6 failed \| 36 passed (42)`, `Test Files  2 failed \| 1 passed (3)`: nei due temi *«keeps every radius concentric»* (`base-list-row in kit-card, bottom-left`/`bottom-right: radius 8.0, outer 20.0, distance 13.0/13.0`), *«opens its window…»* (`base-button in base-dialog, bottom-right: radius 8.0, outer 20.0, distance 13.0/13.0`), il dock (`dv-floating-titlebar` e `dv-groupview in dv-resize-container`, quattro voci `radius 20.0, outer 20.0, distance 1.0/1.0`) | ✓ |
| 9 | Passo 4: `src/tokens/tokens.browser.test.ts` | `Error: the token --size-scrollbar is not defined here: are the token sheets loaded?`, `Tests  1 failed \| 6 passed (7)` | ✓ |
| 10 | Passo 5: jsdom `src/tokens/`; browser, tre file; `grep -c scrollbar base.css`; `git diff --stat -- themes.css` | `Tests  17 passed (17)`; `Tests  5 failed \| 36 passed (41)` — la barra `{ vertical: +0, horizontal: +0 }`, la cornice a `radius 21.0, outer 34.0, distance 12.0/12.0` coi tre compagni nei due temi, il dock a `21.0/21.0, distance 1.0/1.0` —; `17`; vuoto | ✓ |
| 11 | Passi 6, 7, 9, 11, 13 (i verdi) | `7 passed (7)`; `42 passed (42)`; `31 passed (31)`; `22 passed (22)`; jsdom `31 passed (31)` e browser `2 passed (2)` | ✓ |
| 12 | Passo 8: jsdom `src/components/kit.test.ts` | `Error: Failed to resolve import "./BaseNotice.vue" from "src/components/kit.test.ts". Does the file exist?`, `Tests  no tests` | ✓ |
| 13 | Passo 10: `src/kit/kit.browser.test.ts` | `Tests  6 failed \| 16 passed (22)`: nei due temi *«keeps every radius…»* `expected null not to be null` (`:82:25`), *«dresses each tone…»* `expected [] to deeply equal [ 'info', 'ok', 'stop', 'warn' ]`, *«sets a message's title…»* `expected null not to be null` | ✓ |
| 14 | Passo 12: jsdom due file; browser `frame.browser.test.ts` | `Tests  4 failed \| 27 passed (31)`, tutte `Error: Unable to get .base-notice within: …` — le due della fascia, il verdetto, la richiesta in volo —; `Tests  2 failed (2)`, `expected +0 to be close to 12, received difference is 12, but expected 0.05` (`:75:23`) | ✓ |
| 15 | Passo 14, a `da7a522` nel clone: `npm test`, `npm run build`, `npm run lint` | `Test Files  25 passed \| 1 skipped (26)`, `Tests  200 passed \| 1 skipped (201)`; `2634 modules`, `index-DKNLC6Y2.js 691.82 kB │ gzip: 210.77 kB`, CSS `156.62 kB │ gzip: 15.80 kB`; lint uscita 0 | ✓ |
| 16 | Passo 15, le **sedici** righe una alla volta (`<sp>\violations.py`, `<sp>\violations\run.log`) | ogni riga rossa col messaggio della tabella e sulle prove che la tabella nomina, nei due temi dove lo dice; la riga 2 anche `board.test.ts`, *«base.css is the board's block, byte for byte»*; le righe 8 e 9 nello scuro `[ 'rgb(47, 27, 29)', …(2) ]` e `expected 'rgb(229, 158, 161)' to be 'rgb(236, 230, 218)'`; dopo ciascuna `cmp` uguale; alla fine `git status --porcelain` identico a prima | ✓ la tabella del §4 del rapporto, cifra per cifra |
| 17 | mutazioni oltre la tabella (`<sp>\extra.py`, `<sp>\extra\`) | **A**, **M**: verdi (M-1); **B**, `inherits: true` per l'innesco: rossa, `expected '1' to be '0'` — la non-ereditarietà è tenuta —; **C**: 56 su 56 (M-2); **D**, senza il colore del testo: rossa nei due temi, `expected 'rgb(168, 42, 19)' to be 'rgb(101, 91, 87)'` e `expected 'rgb(239, 121, 101)' to be 'rgb(163, 154, 143)'` (la seconda misura di **E65**); **F**, il solo gruppo staccato a `--radius-card`: rossa nei due temi, le due voci `dv-groupview … radius 21.0, outer 21.0` | — |
| 18 | `bash scripts/gate.sh`, da solo, sull'albero (`<sp>\gate-review-20260927-155052.log`) | **`GATE GREEN.`** in 69 s; jsdom `Test Files  19 passed \| 1 skipped (20)`, `Tests  144 passed \| 1 skipped (145)`; browser `Test Files  6 passed (6)`, `Tests  56 passed (56)`; `found 0 vulnerabilities`; `index-DKNLC6Y2.js 691.82 kB │ gzip: 210.77 kB` | ✓ §6 del rapporto; contro la base `137`/`48` → `144`/`56` |
| 19 | le prove **per nome**: `npx vitest list --project jsdom\|browser` a `9579ca0` e a `da7a522`, nel clone | nessun file sparito (24 → 25 file elencati; `git ls-tree` 25 → 26 file di prova, il nuovo è `frame.browser.test.ts`); i soli nomi che escono sono le tre riscritture dettate — il `describe` *«the eight pieces, under axe»* → *«the pieces of the kit, under axe»*, *«is there while waiting…»*, *«shows one event row only once…»* —; entrano 7 prove jsdom e 8 del browser | ✓ |
| 20 | `bash scripts/check-docs.sh` | uscita 0, *«OK — no inconsistencies.»* | ✓ |
| 21 | i log del cancello del rapporto: `ls -la --time-style=full-iso` | `gate-step1-20260927-150555.log` 15:07:13, `gate-final-20260927-151651.log` 15:18:09, nello scratchpad `task6bis\` — dell'implementatore, non quello del coordinatore (`logs\gate-baseline.log`, 14:44); il commit alle 15:19:03; le loro righe coincidono col rapporto | ✓ |
| 22 | fine-riga: per ogni file del commit, CR e righe del blob a `9579ca0` e a `da7a522`, `git ls-files --eol`, CR e righe nell'albero | tutti `i/lf w/lf`, **0** CR prima e dopo; le righe coincidono con la tabella del §5 del rapporto, file per file (`token.html` 644 → 748 … `Band.vue` 42 → 41); i due nuovi LF, `create mode 100644` | ✓ |
| 23 | `sha1sum` dei tre fogli del Passo 5, prima e dopo; `len` dei due fogli | `445e5410`/`5a51054e`/`476481c0` → `445e5410`/`5a4ce31a`/`126a59ce`; 5795 e 6078 caratteri | ✓ |
| 24 | vincoli: `git diff --stat 9579ca0 da7a522 -- gui/package.json gui/package-lock.json crates/ gui/schema/ .github/ Cargo.toml Cargo.lock scripts/ docs/adr/ docs/superpowers/plans/ docs/superpowers/specs/2026-09-22-design-system-design.md`; i colori a mano; `var(--ref-` fuori dai token; `git diff --check` | tutto vuoto; nessun `#…`, `rgb(`, `hsl(`, `hwb(`, `lab(`, `lch(`, `oklab(`, `oklch(`, `color-mix(` nei `.vue` toccati né in `dock.css`; nessun `var(--ref-` fuori da `gui/src/tokens/`; uscita 0 | ✓ vincoli 5, 6, 7, 12 |
| 25 | la non-vacuità, la riga del controllo 20: `grep -c -E 'NON-VACUITY\|toBeGreaterThan\(0'` per file del browser | `dock` 10, `frame` 2, `kit` 17, `settings` 1, `probes` 1, `tokens` 5 | ✓ vincolo 11 |
| 26 | il contratto: i file, il messaggio, il co-autore | i 19 file sono esattamente quelli del punto 2 del prompt; il messaggio comincia con `design-system(compito 6bis): ` e chiude con le due righe del pezzo JavaScript; `git log -1 --format=%B da7a522 \| grep -ci co-authored` → `0`; nessuna cella del piano, niente disegno | ✓ **E69** |
| 27 | il censimento del gotcha #58: `grep -rn` su `card of 20`, `frame of 32`, `raggi diventano 20`, `cornice 32 =`, `1.5`, `eight`/`otto`/`nine`, `band`/`fascia`, `event row` in `gui/src` e `gui/eslint.config.js`, ogni riga letta intera | nessuna cifra vecchia dei raggi fuori dal piano; `1.5` solo nei commenti che raccontano lo scarto di prima; *«eight»* solo nel commento nuovo di `kit.test.ts` e nelle *«eight moves»* di SP-8; *«event row»* in `modules.test.ts:70` (N-1) | ✓ salvo N-1 |

I verdi e i rossi sono tutti al primo colpo: **nessuna** prova è caduta in una corsa che non la mutava (P-19, D23).

**Lo script di confronto, letto.** Le tre forme ricostruiscono ciò che il compito detta: `W` il recinto più un LF; `R` la
sostituzione unica sul testo di `9579ca0` o su quello già sostituito; `S` i due recinti Python **eseguiti** su una copia della
tavola di `9579ca0`, e la tavola, `base.css` e `themes.css` che scrivono sono il dettato — `themes.css` invariato compreso. Due
limiti, detti dallo script o dedotti leggendolo, e innocui qui: i fine-riga si leggono LF dai due lati (li ho misurati a parte,
riga 22), e il `mode` di un file **nuovo** non lo guarda — `git diff --summary` scrive `create mode …`, non `mode change` —: i due
file nuovi sono `100644`.

## 4. Guardarla — la SPA e la pagina kit, nel Chrome installato, con le barre accese

Server di sviluppo del clone a `da7a522`, `npm run dev -- --port 5391 --strictPort`; Playwright dal `node_modules` del clone,
`channel: "chrome"`, `ignoreDefaultArgs: ["--hide-scrollbars"]`, 1440 × 900, densità 1; il tema con `dataset.theme` **e**
`emulateMedia({ colorScheme })` — così un `watchTheme` che riapplicasse il sistema dà lo stesso tema. Script: `<sp>\look.mjs`,
`look2.mjs`, `look3.mjs`; misure in `<sp>\look\report.json` e `report2.json` (le chiavi `*.spa.4.*` e `*.spa.6.*` di `report.json`
sono **nulle**: quella prima passata ha misurato barre e zona d'arrivo con la finestra di conferma aperta, e il suo velo prendeva il
puntatore; `look2.mjs` le ha rifatte nell'ordine giusto, e le schermate nulle sono state tolte). Schermate in `<sp>\look\`. Poi il
server **fermato per PID** — 42396 e, per la base, 32720, con `taskkill /T /F` —: nessuna porta 5391 in ascolto, **0** processi
`node.exe`.

Ciò che vedo, e che l'aspetto lo giudica il proprietario (controllo 15):

| | Chiaro | Scuro |
|---|---|---|
| **la fascia a core spento**, prima di `harnessFake.deliverAll()` (`*-spa-1-band-waiting*.png`, `zoom-*-1-band-waiting*.png`) | `warn`, `triangle-alert` nel colore del tono, titolo nel colore del testo; sulla pagina, raggio **21**, bordo 1; a **12** px dai lati, subito sotto la barra (50 = 50), **12** px sopra le schede; «Riprova» a **13/13/13** dal lato e dai bordi, raggio **8**: concentrico; icona, titolo e pulsante sulla stessa riga, centri 75/75/75. Il fuoco da tastiera su «Riprova»: contorno `solid 2px` bordeaux, `offset 2px`, intero, dentro la fascia | uguale, nei colori dello scuro; il fuoco rosa antico, intero. Il contorno del pulsante quasi non si vede (**P-2**) |
| **dopo la consegna** (`*-spa-2-band-stale*.png`) | `stop`, `circle-x`; il titolo e sotto *«Timbro atteso: 18364758544493064720»* in `muted`; raggio 21, a 12 dai lati e dalle schede, riga in alto (nessuna azione); nella barra *«Core: timbro diverso»* nel colore di `stop` | uguale |
| **Impostazioni** — scelta «Locale» (`*-inflight-with-window.png`) | *«Richiesta inviata: in attesa del core.»* `info`, raggio 8, sotto il gruppo della policy; col clic si apre anche la finestra di conferma *«Serve un permesso»* — *«Per: vram-policy con local»* — col suo velo; con Esc la finestra si chiude, il fuoco torna su «Locale» e il messaggio sparisce (`*-after-escape.png`) | uguale |
| **Stato**, il verdetto (`*-spa-3a-stato-card.png`, `*-floating-clip.png`) | `stop`, *«Ultima richiesta di VRAM: rifiutata»* e sotto *«chiesti 4096 MiB, tetto 1024»*, raggio 8; nella Home la scatola scorre, e del messaggio in fondo si vede, al bordo della scheda, solo la riga di sopra — contenuto che scorre, come la riga di prima a `9579ca0` | uguale |
| **le barre di Stato e Impostazioni** (`*-bar-*.png`, `zoom-*-bar-*.png`) | spesse **10**; a riposo nessun cursore, `--scrollbar-trigger` `0`; col puntatore sopra la scatola il cursore compare, sottile e tondo, **senza frecce**, staccato dall'alto e dal basso, e l'innesco è `1`; col puntatore sul cursore si scurisce; col fuoco dentro — la scatola stessa per Stato, un radio per Impostazioni — compare, innesco `1`. L'angolo fra le due barre non c'è: le scatole scorrono solo in verticale. Col fuoco sulla scatola di Stato l'anello si vede solo in alto (**P-1**) | uguale; il cursore chiaro sul carbone |
| **i raggi del dock** (`*-drag-zone*.png`, `*-floating-clip.png`, `zoom-*-floating-corners.png`) | la zona d'arrivo, trascinando «STATO» su Attività: raggio **20** ai quattro angoli, a 1 px dentro la scheda da 21; il gruppo staccato con «Stacca la tessera…»: contenitore 21 col bordo 1, barra del titolo `20px 20px 0 0` a 1/1, gruppo `0 0 20px 20px` a 1/1 | uguale |
| **la pagina kit** (`*-kit-1-full.png`, `*-kit-2-*.png`, `*-kit-5-dialog*.png`) | la scheda «Messaggi» coi quattro toni, raggio 8, nessun testo tagliato; «Messaggio sulla pagina» raggio 21 con «Riprova» a 13/13/13, raggio 8, centri allineati; la cornice delle carte **34**, bordo `1px solid rgba(0, 0, 0, 0)`, carte **21** a 13/13; la finestra raggio 21, bordo 1, «Consenti» a 13/13 raggio 8; il fuoco su «Riprova» visibile; la pagina scorre, con la sua barra da 10 | uguale; nello scuro il bordeaux del tono `info` e il rosso di `stop` si somigliano, com'era detto in **E60** |

Nulla che sbordi o si sovrapponga fuori da ciò che il dock fa da sé — il gruppo staccato si posa sopra Permessi —, nessun testo
tagliato (`scrollWidth` contro `clientWidth` sui titoli e sui testi dei messaggi), nessun errore nella console della pagina kit;
nella SPA il solo 404 di **P-3**.

**Il frammento del contrasto** del Passo 8 del compito 6, sulla SPA dopo la consegna e senza finestre aperte (`<sp>\look3-da7a522.log`):

| | `seen` | i primi tre di `worst` |
|---|---|---|
| chiaro, `.dock *` | 34 | **5,39** *«chiesti 4096 MiB, te»* · 5,7 *«niente ancora»* · 5,7 *«arriva col sotto-pro»* |
| scuro, `.dock *` | 34 | **5,4** *«chiesti 4096 MiB, te»* · 6,41 *«Degrado»* · 6,41 *«Policy VRAM»* |
| chiaro, `.frame *` (fascia compresa) | 41 | 5,39 *«Timbro atteso: 18364»* · 5,39 · 5,7 *«Home»* |
| scuro, `.frame *` | 41 | 5,4 *«Timbro atteso: 18364»* · 5,4 · 6,41 |

Il primo di `worst` è **sopra 4,5** nei due temi. A `9579ca0`, con lo stesso comando: 5,7 e 6,41 — il nuovo minimo è il testo
`muted` sul fondo tenue di `stop`, una coppia che `contrast.test.ts` giudica già.

**Ciò che lo scarto nuovo cambia nel giudizio di chi usava la sonda** — punto 7 (f): le prove della pagina kit e del dock sono
verdi, e ogni coppia che la sonda vi incontra — 25 per tema, elencate da `<sp>\pairs.log`, dal puntino dentro i radio alla
cornice delle carte, al gruppo staccato e alla fascia — ha le due distanze **uguali** e scarto **0,00**: il mezzo pixel non ne
accetta nessuna per indulgenza, e nessuna cambia fra *«nell'angolo»* e *«fuori»*.

## 5. Ciò che non ho potuto verificare, e perché

- **Il mouse vero**: il mio puntatore è simulato. Il cursore della barra *acceso* l'ho visto nelle schermate col puntatore e col
  fuoco, e ho letto l'innesco nei tre stati; la conferma col mouse vero resta al Passo 16, del proprietario.
- **La preferenza di Windows che tiene sempre visibili le barre, e i colori forzati**: non misurati, restano assunti come dice E59.
- **La CI** su `ubuntu-latest` e `windows-latest`: la legge il coordinatore dopo il push, che è suo.
- **L'altra macchina**, `zagor`: tutto qui è della macchina `Jays`, densità 1; a 175 % vale la trappola 28.
- **Le tre corse di stabilità dell'implementatore**: ho letto `stability-1.log` (`Tests  200 passed | 1 skipped (201)`), non le ho
  ripetute tre volte; le mie corse — i Passi, le 16 violazioni, le mutazioni, `npm test` e il cancello — sono andate tutte come
  atteso, al primo colpo.
- **Il metodo dichiarato dal rapporto** — il brief letto per intero, i 93 recinti estratti, i 36 *Trova* contati prima di toccare —:
  è processo; è coerente con ciò che si misura — 93 recinti nella sezione del compito (`awk`), 36 sostituzioni su 39 operazioni in
  `plan_ops.py` —, e il suo esito, il commit, è il testo del piano.
