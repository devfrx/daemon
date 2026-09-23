# Rapporto R2 — compito 2 (il browser dei test) e compito 3 (il kit)

Revisore R2, 2026-09-23, HEAD `d589d15`, in sola lettura sul repository. Cartella di prova:
`C:\Users\zagor\AppData\Local\Temp\probe-ds2` (copia di `gui/` e `scripts/gate-gui.sh` col `tar` dei vincoli, poi `npm ci`).
Del compito 1 la cartella ha solo ciò che il 2 consuma, preso **dal testo del piano**: i due caratteri installati
(`@fontsource-variable/geist@5.3.0`, `@fontsource/barlow@5.3.0`), `base.css` e `themes.css` tagliati dalla tavola con
`extract_tokens.py` copiato dal passo 9, `theme.ts`, `index.ts` e `dock.css` copiati dai passi 10–11. `main.ts` non è
toccato: l'app della cartella di prova resta quella di oggi, con `tokens.css`.

## Rilievi

### R2-1 — Critico — la sonda del contorno del focus è ROSSA come è dettata: `--color-focus` vive sotto `[data-theme]`
- Dove: piano righe 1281–1297, compito 2, passo 2 (e l'atteso del passo 4, righe 1450–1452)
- Che cosa dice il piano: «Atteso: il progetto `browser` **verde**, cinque prove»
- Che cosa è vero: la regola `:focus-visible { outline: var(--focus-width) solid var(--color-focus) }` di `base.css` legge
  un ruolo che `themes.css` definisce **solo** sotto `[data-theme="dark"]` e `[data-theme="light"]`; la prova non posa
  `data-theme` sulla radice (lo fa `main.ts`, compito 1, non la prova), quindi `var(--color-focus)` è vuoto, la
  dichiarazione è invalida al valore calcolato e `outline-style` torna `none`. Non c'entra l'alto contrasto: il contorno
  manca anche senza.
- Prova: nella cartella di prova, configurazione del passo 3 applicata con `replace_unique.py` (tre sostituzioni uniche,
  CRLF conservati: CR 53 → 103 = righe) → `npx vitest run --project browser` → `Tests 1 failed | 4 passed (5)`,
  `AssertionError: expected false to be true` alla riga `expect(ring(button)).toBe(true)`. Sonda di diagnosi, stesso
  bottone dopo `userEvent.tab()`: senza `data-theme` →
  `{"outlineStyle":"none","outlineWidth":"3px","focusVisible":true,"active":true,"forced":true,"colorFocus":""}`; con
  `document.documentElement.dataset.theme = "dark"` → `{"outlineStyle":"solid","outlineWidth":"2px","colorFocus":"#BF5567"}`.
- Correzione proposta: nella prova, prima dell'emulazione, `document.documentElement.dataset.theme = "dark";` (o
  `watchTheme(() => "dark")` col suo `stop`), e nell'`afterEach` `delete document.documentElement.dataset.theme;`.
  Rilanciato con questa correzione: `Tests 5 passed (5)`; poi `npm test` → 16 file (15 passed, 1 skipped), 90 prove
  (89 passed, 1 skipped) contro la baseline della cartella di prova, 15 file e 85 prove: **+1 file, +5 prove**, com'è
  l'atteso; `npm run build` verde, con `vue-tsc` che legge `vite.config.ts` e `browser.d.ts`.

### R2-2 — Importante — la stessa sonda non prova l'alto contrasto: passa identica senza `forcedColors`
- Dove: piano righe 1281–1297, compito 2, passo 2; tabella del passo 5, righe 1456–1461 (nessuna riga per questa prova)
- Che cosa dice il piano: «keep the focus ring under Windows' high contrast: an outline, which forced colours keep», e
  come seconda direzione `button.style.outline = "none"` → `ring(button)` falso
- Che cosa è vero: con `data-theme` posato (R2-1) il contorno c'è **con e senza** l'emulazione: la prova non guarda nulla
  che l'alto contrasto cambi. La «seconda direzione» toglie il contorno a mano e scatterebbe anche senza alto contrasto:
  prova la funzione `ring`, non la trappola 11. Se `emulateMedia({ forcedColors })` non facesse nulla, la prova resterebbe
  verde.
- Prova: sonda di diagnosi nel Chrome installato, `data-theme="dark"`, **senza** `forcedColors` →
  `{"outlineStyle":"solid","outlineWidth":"2px","forced":false}`, lo stesso verdetto di `forced: true`. E ciò che l'alto
  contrasto **fa** davvero: un anello fatto con `box-shadow` e `outline:none`, sotto `forcedColors: "active"` →
  `{"outlineStyle":"none","boxShadow":"none","forced":true}`.
- Correzione proposta: dopo l'emulazione, la guardia `expect(matchMedia("(forced-colors: active)").matches).toBe(true);`
  (misurata `true` dentro l'iframe delle prove); come seconda direzione quella che l'alto contrasto produce: lo stesso
  bottone con `outline: none; box-shadow: 0 0 0 2px currentColor` → `getComputedStyle(button).boxShadow` vale `"none"` —
  è la ragione per cui la (a) sceglie l'`outline`. E una riga per questa prova nella tabella del passo 5.

### R2-3 — Importante — ogni rosso del progetto `browser` scrive PNG dentro `gui/src/`, e git non li ignora
- Dove: piano righe 1426–1437 (la configurazione), compito 2, passo 3; passo 5, righe 1454–1463
- Che cosa dice il piano: «Poi `git diff --stat` a zero sui file toccati per le prove.»
- Che cosa è vero: `browser.screenshotFailures` vale di base `!browser.ui`, quindi vero senza finestra, e
  `screenshotDirectory` vale `__screenshots__` **accanto al file di prova**: ogni prova che fallisce **nel browser** — le
  righe 3 e 4 del passo 5, fatte rosse apposta, e la sonda di R2-1 com'è dettata (le righe 1 e 2 no: lì il browser non
  c'è) — lascia `gui/src/tokens/__screenshots__/tokens.browser.test.ts/<nome della prova>-1.png`. `.gitignore` non li copre, e
  `git diff --stat` non vede i file non tracciati: il passo dichiara pulito un albero sporco, e un `git add` largo li
  committerebbe.
- Prova: `grep -n 'screenshotFailures' node_modules/vitest/dist/chunks/reporters.d.DtoKVV2s.d.ts` → riga 1709,
  `@default !browser.ui`, e poco sopra `If not set, all screenshots are saved to __screenshots__ directory in the same
  folder as the test file`; dopo il rosso di R2-1 e quelli delle sonde di diagnosi `find src -name '__screenshots__'` →
  `src/tokens/__screenshots__`, un PNG per prova fallita; la riga 4 del passo 5 rifatta →
  `src/tokens/__screenshots__/tokens.browser.test.ts/the-tokens--in-a-real-browser--…--put-the-motion-to-zero-…-1.png`;
  `git -C <repo> check-ignore -v gui/src/tokens/__screenshots__/x.png` → exit 1.
- Correzione proposta: `screenshotFailures: false` nel progetto `browser`, detto accanto (nel cancello nessuno le
  guarda); oppure `/gui/src/**/__screenshots__/` in `.gitignore`. E nel passo 5 `git status --porcelain` vuoto, non
  `git diff --stat`.

### R2-4 — Importante — la sonda dei caratteri non distingue Barlow dal suo primo ripiego, e il commento dice il contrario
- Dove: piano righe 1247–1269, compito 2, passo 2
- Che cosa dice il piano: «without `tabular-nums` Barlow's digits are proportional -- measured on the token board on
  2026-09-23, 67.34 against 108.10 px -- so a probe that measured the fallback, or the wrong variant, would see the
  difference.»
- Che cosa è vero: il primo ripiego di `--font-family-tool` su Windows è `"Bahnschrift"`, che ha **anche lui** cifre
  proporzionali di base e uguali con `tabular-nums`: se `--font-display` disegnasse col ripiego — un nome di famiglia che
  non combacia col `@font-face` — le due direzioni delle larghezze resterebbero verdi. E `loaded("Barlow") >= 4` è vero
  perché è **la prova stessa** a chiedere `document.fonts.load('… "Barlow"')`: prova che le facce si caricano, non che il
  token disegni con loro. Il legame del controllo 9 — il carattere del token è Barlow, caricato — non è provato.
- Prova: nel Chrome installato, `font: var(--font-display)` con la famiglia forzata, larghezze in px di `111111` e
  `000000`: `"Barlow"` normale 67.33 / 108.11, tabulare 100.42 / 100.42; `"Bahnschrift"` normale **63.84 / 102.81**,
  tabulare **102.19 / 102.19**. `--font-display` calcolato: `300 2rem/2.5rem "Barlow", "Bahnschrift", system-ui,
  sans-serif`.
- Correzione proposta: una terza asserzione — lo stesso testo col token e con la catena del token **senza** `"Barlow"`
  (`getPropertyValue("--font-family-tool")` col primo nome tolto) misura diverso: 67.33 contro 63.84 px sulle `1`
  normali, 100.42 contro 102.19 sugli `0` tabulari, soglia 0,5 come la prima; e il commento riscritto: la seconda
  direzione prova che `tabular-nums` agisce, non quale carattere disegna.

### R2-5 — Minore — la riga 2 del passo 5 non diventa rossa per nessuna delle due ragioni che il piano attende
- Dove: piano riga 1459, compito 2, passo 5
- Che cosa dice il piano: «rosso: sotto Node non c'è `document`, o il rettangolo è zero — si **legge** quale»
- Che cosa è vero: con `enabled: false` il file non si carica affatto: `Error: vitest/browser can be imported only inside
  the Browser Mode. Your test is running in forks pool.` — `Test Files 1 failed | 14 passed | 1 skipped (16)`. La guardia
  «is a real one» non arriva a girare: questa violazione non la esercita, e chi «legge quale» trova una terza ragione (il
  punto 3 di *«Come si esegue»* lo ferma).
- Prova: `enabled: true` → `enabled: false` nella cartella di prova, `npm test` → exit 1 col messaggio sopra.
- Correzione proposta: l'atteso col messaggio vero, e detto accanto che fuori dal browser il file non si carica: è
  l'import a fare da guardia per questa riga, non il rettangolo.

### R2-6 — Minore — «and it is Chrome»: la regex prende anche il Chromium di Playwright
- Dove: piano righe 1235–1243, compito 2, passo 2
- Che cosa dice il piano: «and the channel is the installed Chrome (decision 22)», con
  `expect(navigator.userAgent).toMatch(/Chrome\//)`
- Che cosa è vero: nell'iframe delle prove `navigator.userAgent` è `… HeadlessChrome/153.0.0.0 Safari/537.36`: `/Chrome\//`
  combacia per la coda di `HeadlessChrome/`, e combacerebbe col Chromium scaricato. Ciò che distingue il canale è
  `navigator.userAgentData.brands`, misurato `[{"brand":"Google Chrome","version":"153"},{"brand":"Not_A
  Brand","version":"8"},{"brand":"Chromium","version":"153"}]`. Su questa macchina `%LOCALAPPDATA%\ms-playwright` ha
  `chromium-1234` del 2026-08-26, non la revisione 1243 che `playwright-core` 1.63.0 vuole (`browsers.json`): qui un
  ritorno al browser scaricato fallirebbe all'avvio; ma dove il Chromium 1243 c'è — il ripiego che il disegno suggerisce,
  R2-8 — passerebbe inosservato.
- Prova: sonda di diagnosi nel progetto `browser` della cartella di prova (`inner` misurato 1440 × 900, `top: false`:
  la finestra delle prove e l'iframe sono quelli del piano); `ls -la --time-style=full-iso "$LOCALAPPDATA/ms-playwright"`;
  `head -20 node_modules/playwright-core/browsers.json`.
- Correzione proposta: la marca `"Google Chrome"` fra `userAgentData.brands`, letta attraverso un tipo scritto accanto
  (`userAgentData` non è nella `lib` DOM del `tsconfig`); o il commento ridotto a «a Chromium».

### R2-7 — Minore — «`npm test` coi **due** progetti nell'uscita»: nel cancello, col reporter di base, non ci sono
- Dove: piano righe 1450–1451, compito 2, passo 4
- Che cosa dice il piano: «`npm test` coi **due** progetti nell'uscita, `jsdom` e `browser`»
- Che cosa è vero: senza terminale — com'è il cancello, e com'è un subagente — il reporter di base stampa solo il
  riepilogo quando tutto è verde: nessun nome di progetto.
- Prova: `npm test > log; grep -c 'jsdom' log; grep -c 'browser (chromium)' log` → `0` e `0`;
  `npx vitest run --reporter=verbose | grep -oE '\|(jsdom|browser \(chromium\))\|' | sort | uniq -c` → `85 |jsdom|`,
  `5 |browser (chromium)|`.
- Correzione proposta: l'atteso con quel comando e i due conteggi, dal file (trappola 6: i backslash).

### R2-8 — Minore, da segnalare al proprietario — il ripiego della decisione 22 non funziona col canale `chrome`
- Dove: disegno, decisione 22 e riga «dove c'è» della (f); nel piano, riga 9 del *«Come si riprende»*
  (`porta-di-qualita.md`, «Chrome come prerequisito»)
- Che cosa dice il disegno: «sull'altra macchina serve Chrome, o `npx playwright install chromium`»
- Che cosa è vero: con `launchOptions: { channel: "chrome" }` Playwright cerca Google Chrome stabile, e il Chromium
  scaricato **non** lo sostituisce; il suo messaggio per un Chrome che manca è `Chromium distribution 'chrome' is not
  found at … Run "npx playwright install chrome"`. Il piano, nel commento del cancello (passo 6), dice giusto — «a
  machine without Google Chrome goes red here» —; il disegno no.
- Prova: `lib/coreBundle.js` di `playwright-core` 1.63.0: il `throw new Error` con `Chromium distribution '${name}' is
  not found${location2}${installation}` e l'`installation` fatto con `buildPlaywrightCLICommand(sdkLanguage, "install " +
  name)`.
- Correzione proposta: il compito 9 scrive in `porta-di-qualita.md` «Google Chrome, o `npx playwright install chrome`»,
  non `chromium`; la frase della decisione 22 si corregge col richiamo datato, se il proprietario vuole.

### R2-9 — Critico — `kit.test.ts` non compila: `BaseList as Component` è un TS2352, e il *build* del passo 6 è rosso
- Dove: piano riga 1626, compito 3, passo 3 (e l'atteso del passo 6, righe 2533–2535; il passo 9)
- Che cosa dice il piano: «`const List = BaseList as Component;`» e poi «Atteso: **verde** — … il *build* verde, con
  `vue-tsc` che ha letto la riga `@ts-expect-error`»
- Che cosa è vero: un componente generico (`<script setup lang="ts" generic="T">`) per `vue-tsc` 3.3.11 è una funzione, e
  la conversione diretta a `Component` non «si sovrappone abbastanza»: `vue-tsc --noEmit` si ferma, quindi `npm run build`
  esce 2, e il cancello con lui. `vitest` non se ne accorge (non controlla i tipi): le 22 prove passano, ed è per questo che
  un esecutore vedrebbe il verde al primo comando e il rosso al secondo.
- Prova: nella cartella di prova, coi nove file del compito dettati parola per parola → `npm run build` → exit 2, un solo
  errore: `src/components/kit.test.ts(21,14): error TS2352: Conversion of type '<T>(__VLS_props: …) => import("vue").VNode
  & { __ctx?: … }' to type 'Component' may be a mistake because neither type sufficiently overlaps with the other. If
  this was intentional, convert the expression to 'unknown' first.`
- Correzione proposta: `const List = BaseList as unknown as Component;` — rilanciato: `npm run build` exit 0. E la
  seconda direzione della riga `@ts-expect-error` regge: con `ICONS: Record<string, IconNode>` al posto di `satisfies`
  (quindi `IconName` = `string`) `vue-tsc` dà `kit.test.ts(47,5): error TS2578: Unused '@ts-expect-error' directive.`

### R2-10 — Minore — il commento del blocco `harness/ts-in-vue` resta a dire il contrario del blocco nuovo
- Dove: piano righe 2562–2585, compito 3, passo 7
- Che cosa dice il piano: aggiunge `harness/ts` con `files: ["**/*.ts"]`, e non tocca nient'altro del file
- Che cosa è vero: due blocchi sopra, il commento di `harness/ts-in-vue` dice ancora *«The `.ts` files are still nobody's
  business here (P-101): `vue-tsc` inside `npm run build` is the level 1 of the web world.»* Dopo il passo 7 è falso, e a
  dirlo è il file che lo smentisce tre righe sotto (gotcha #58: ciò che smentisce può stare in un commento).
- Prova: `grep -n "nobody's business" gui/eslint.config.js` → riga 25 oggi, riga 36 dopo le tre sostituzioni nella
  cartella di prova; `grep -n "nobody's business" docs/superpowers/plans/2026-09-23-design-system.md` → nessuna riga.
- Correzione proposta: una quarta sostituzione che riscrive quella frase — «the `.ts` files are read from the design
  system on (block `harness/ts`, P-2 of its plan); their TYPES stay with `vue-tsc`».

### R2-11 — Minore, da segnalare al proprietario — P-3 restringe una regola approvata della (b) senza la sua A/B
- Dove: piano riga 174 (P-3) e righe 2604–2624, compito 3, passo 7
- Che cosa dice il disegno: la tabella *«Le regole»* della (b) — «in `components/` **nessun import** di `pinia`,
  `stores/`, `panels/`, `frame/`, `transport/` | `no-restricted-imports` di ESLint, limitato a `components/`»
- Che cosa è vero: il fatto di P-3 è vero — `Confirm.vue` importa `../stores/core` e `../stores/invoke` (righe 5–6), e la
  (b) lo tiene in `components/` come pezzo composto: il disegno approvato si contraddice da sé. Il piano la risolve da
  solo, restringendo la regola ai `Base*.vue` e a `icons.ts`; ma non è una delle decisioni «lasciate al piano» (D1–D8),
  e il vincolo globale 1 dice «ci si **ferma e lo si riporta**». Il costo della strada scelta non è scritto: in
  `components/` restano senza regola `Confirm.vue`, `markdown.ts`, `kit.test.ts` — che importa `../panels/registry`, e
  con la regola della (b) alla lettera sarebbe rosso — e ogni pezzo composto futuro.
- Prova: `grep -n 'stores' gui/src/components/Confirm.vue` → righe 5 e 6; nella cartella di prova il linter è verde con
  `kit.test.ts` che importa `../panels/registry`.
- Correzione proposta: portarla al proprietario in forma A/B — **A** la regola ai soli pezzi di base, come P-3 (costo:
  i pezzi composti di `components/` senza guardia); **B** la regola su `components/**` con l'eccezione nominata di
  `Confirm.vue` e dei test (costo: un'eccezione per ogni pezzo composto).

### R2-12 — Minore — il comando che il passo 2 dà per trovare il blocco dell'aiutante ne lascia fuori metà
- Dove: piano righe 1589–1593, compito 3, passo 2
- Che cosa dice il piano: il blocco «che comincia con il `/**` sopra `async function violations…`», «intero, preso dal
  file (`grep -n 'async function violations\|Fills the stores' gui/src/a11y.test.ts`, poi `sed -n` fra le due righe)»
- Che cosa è vero: quel `grep` rende le righe **33** e **38**; il `/**` sta alla **25**. Chi segue il comando taglia da
  33 a 38 e lascia orfano il commento di otto righe sopra `welcome()`: le prove restano verdi e nessuno se ne accorge.
- Prova: `grep -n 'Every violation axe finds\|async function violations\|Fills the stores' gui/src/a11y.test.ts` → `26`,
  `33`, `38`; `sed -n '24,26p'` → riga vuota, `/**`, ` * Every violation axe finds …`. Col blocco 25–38 la
  sostituzione dà 11 prove, gli stessi nomi di prima (`vitest --reporter=verbose`, `diff` vuoto), 5 `it(` prima e dopo.
- Correzione proposta: `grep -n 'Every violation axe finds\|Fills the stores'` e il blocco dalla riga **sopra** la prima
  (il `/**`) alla seconda.

### R2-13 — Minore — al compito 3 il pezzo JavaScript non cambia: le «ventiquattro icone» non ci sono ancora
- Dove: piano righe 2533–2535, compito 3, passo 6; e la voce *N-2 di E187* a riga 212
- Che cosa dice il piano: «Il pezzo JavaScript dopo il *build* … si scrive nel commit: le icone sono ventiquattro, circa
  mezzo kB ciascuna»
- Che cosa è vero: nessun file dell'app importa il kit prima del compito 5, quindi `vite` scarta `icons.ts` e `lucide`:
  il pezzo è identico a quello di prima.
- Prova: `npm run build` nella cartella di prova dopo il compito 2 (prima del 3), dopo il compito 3, e dopo il `npm ci`
  della sequenza del cancello → `dist/assets/index-j6b66g-W.js 663.26 kB │ gzip: 201.23 kB` tutte e tre le volte (lo
  stesso nome di file: stesso contenuto) — la stessa cifra che la misura 2 del disegno dà per la baseline di oggi.
- Correzione proposta: al compito 3 «invariato: il kit non è ancora importato», e la nota delle icone al compito 5, dove
  i pannelli le importano e la misura dice qualcosa sulla (e).

### R2-14 — Minore — `BaseButton` calcola «solo icona» una volta sola: gli slot non sono reattivi
- Dove: piano righe 1992–1993, compito 3, passo 5
- Che cosa dice il piano: `const iconOnly = computed(() => props.icon !== undefined && slots.default === undefined);`
- Che cosa è vero: `useSlots()` non è reattivo, quindi il `computed` dipende solo da `props.icon`: se chi lo usa passa le
  parole e poi le toglie (o il contrario), `aria-label`, `title` e `data-icon-only` restano quelli del primo disegno. Nel
  caso peggiore resta un pulsante con la sola icona e **senza nome**. Oggi nessun uso dei compiti 4–8 cambia lo slot, ma
  è un pezzo di base.
- Prova: sonda nella cartella di prova — `h(BaseButton, { icon: "float", label: "Stacca" }, words.value ? { default: () =>
  "Stacca" } : {})`, poi `words.value = false` e `nextTick` → `<button … data-variant="secondary" data-size="md"><svg …
  data-icon="float" …></svg></button>`: né parole, né `aria-label`, né `data-icon-only`.
- Correzione proposta: calcolarlo nel disegno — una funzione chiamata dal template, o `$slots.default` direttamente nel
  template — non un `computed`.

### R2-15 — Minore — le tavole non danno l'icona della Chat: `message-square` vi segna i «Messaggi di stato»
- Dove: piano riga 200 (D6) e riga 1874, compito 3, passo 4 (commento di `icons.ts`)
- Che cosa dice il piano: D6, «le tavole danno Stato, Permessi, Passi, Attività, Chat, i moduli, la ricerca, le viste»; nel
  codice «The boards gave Stato, Permessi, Passi, Attività and Chat»
- Che cosa è vero: nelle quattro tavole non c'è né una Chat né un'icona della Chat; `message-square` compare solo nella
  tavola dello stile, come titolo della sezione «Messaggi di stato». Le altre combaciano: `gauge` Stato, `shield-check`
  Permessi, `list-checks` Passi, `activity` Attività, `layout-grid` moduli, `search`, `layers` le viste, `bookmark-plus`
  «Salva questa vista», `mic` «In ascolto».
- Prova: `grep -n -i 'chat' docs/superpowers/specs/2026-09-22-design-system-tavole/*.html` → nessuna riga; uno script che
  stampa il testo dopo ogni `<svg … data-icon=…>` → `message-square | Messaggi di stato`.
- Correzione proposta: la Chat fra le scelte del piano (D6), e il commento «The boards gave Stato, Permessi, Passi and
  Attività»; e detto che `message-square` sulle tavole vuol dire già un'altra cosa, se un giorno i messaggi entrano nel
  kit.

### R2-16 — Minore — le prove del kit non si provano nella direzione rossa, e una non ha la guardia di non-vacuità
- Dove: piano righe 1645–1647 e 1818–1822, compito 3, passi 3 e 8
- Che cosa dice il piano: l'unico rosso delle prove del kit è quello del passo 3, «i pezzi non esistono ancora»; il passo 8
  prova nelle due direzioni solo il linter
- Che cosa è vero: il rosso del passo 3 è `Failed to resolve import "./BaseIcon.vue"`: dice che il file manca, non che
  ciascuna asserzione sa fallire (vincolo globale 11). E *«has an icon for every module type»* è verde anche con
  `PANEL_TYPES` vuoto: `[].filter(…)` è `[]`.
- Prova: `npx vitest run src/components/kit.test.ts` prima dei pezzi → `Error: Failed to resolve import "./BaseIcon.vue"`,
  `Tests no tests`.
- Correzione proposta: `expect(PANEL_TYPES.length).toBeGreaterThan(0)` in quella prova, e al passo 8 due righe per il kit
  — poi indietro — rilanciate nella cartella di prova (`tools/kit_directions*.py`, file ripristinati, `diff -r` vuoto):
  tolta la riga `models: Cpu,` da `ICONS` → `AssertionError: expected [ 'models' ] to deeply equal []`; `BaseStatus` che
  nasce con le parole (`v-if` su una funzione che guarda se lo slot disegna qualcosa che non sia un commento: il difetto
  di M-3) → `Error: Unable to get [role="status"] within: <!--v-if-->`. ⚠️ Un `v-if="$slots.default"` qualunque **resta
  verde**, e a ragione: chi usa `BaseStatus` passa sempre lo slot, anche quando disegna `null`.

### R2-17 — Nit — un `import()` dinamico passa la regola dei pezzi di base
- Dove: piano righe 2555–2559 e 2647–2649, compito 3, passi 7 e 8
- Che cosa dice il piano: «What a base piece may not reach: the global state, and every layer above the kit»
- Che cosa è vero: `no-restricted-imports` guarda gli `import`/`export … from` statici; `const core = await
  import("../stores/core");` dentro `BaseStatus.vue` → `npx eslint src` exit 0. Tutti gli altri modi provati scattano:
  `pinia` in `BaseList.vue`, `../stores/core` in `icons.ts`, `@/stores/layout`, `../stores/layout.ts`, `export { useCore }
  from "../stores/core"`, `lucide` in `kit.test.ts`.
- Prova: `tools/lint_directions.py extra` nella cartella di prova, un caso per volta, il file ripristinato byte per byte
  (`diff -r` vuoto).
- Correzione proposta: dirlo accanto alla nota *«Il linter non ha una guardia di non-vacuità»*; oppure
  `no-restricted-syntax` su `ImportExpression` per gli stessi percorsi.

### R2-18 — Nit — `BaseDialog` scrive due misure a mano, e senza descrizione lascia un `aria-describedby` a vuoto
- Dove: piano righe 2437–2503, compito 3, passo 6
- Che cosa dice il piano: `width: min(30rem, calc(100vw - var(--space-12)));`, `max-height: 60vh;` e
  `<DialogDescription v-if="description !== undefined" …>` senza altro per il caso senza descrizione
- Che cosa è vero: (1) la (b) vuole lo «stile `scoped` coi **soli token**»; i 53 token che gli otto pezzi leggono esistono
  tutti (in `base.css` o in **tutti e due** i temi, nessuna scala, nessun colore a mano), ma `BaseDialog` scrive
  `min(30rem, …)` e `max-height: 60vh`, che nella tavola non ci sono (la `.dlg` della tavola è larga `74%` della sua
  cornice). (2) Senza `description` `reka-ui` 2.10.4 avvisa `Missing \`Description\` or \`aria-describedby="undefined"\`
  for DialogContent` e lascia `aria-describedby="reka-dialog-description-v-1"` che non punta a nulla; `axe` sotto jsdom
  non lo conta. Il cassetto del compito 5 usa `BaseDialog` senza descrizione.
- Prova: `vars_defined.py` nella cartella di prova → `distinct vars used: 53`, `every var() of the pieces is defined`,
  `scales read by pieces: []`; `grep -nE '[0-9]+(px|rem|vh|vw|em|ms)'` sui `Base*.vue` → le sole righe 62 e 67 di
  `BaseDialog.vue`; sonda jsdom → `{"describedby":"reka-dialog-description-v-1","describedExists":false,"violations":[]}`.
- Correzione proposta: dire accanto che la larghezza è una scelta del piano (o un token nuovo, che è della tavola); e su
  `DialogContent` `v-bind="description === undefined ? { 'aria-describedby': undefined } : {}"`: in
  `Dialog/DialogContentImpl.js` i `$attrs` si fondono **dopo** l'`aria-describedby` di `reka-ui`, e `useWarning`
  (`Dialog/utils.js`) avvisa solo se l'attributo c'è. Provato sulla cartella di prova: senza descrizione
  `{"id":null}` e nessun avviso; con la descrizione `{"id":"reka-dialog-description-v-1","target":"D"}`; `vue-tsc` verde.

### Conteggio

| Gravità | Rilievi |
|---|---|
| Critico | 2 — R2-1, R2-9 |
| Importante | 3 — R2-2, R2-3, R2-4 |
| Minore | 11 — R2-5, R2-6, R2-7, R2-8, R2-10, R2-11, R2-12, R2-13, R2-14, R2-15, R2-16 (R2-8 e R2-11 da segnalare al proprietario) |
| Nit | 2 — R2-17, R2-18 |

Visto **fuori fetta**, una riga per R1: la tabella della posizione, riga 127, dice «i nomi nuovi nei **dodici**
componenti», il compito 1, riga 230, «gli **undici** componenti che il censimento del passo 1 nomina».

## Verificato e giusto

Tutto nella cartella di prova, sul codice di `d589d15` copiato, coi blocchi del piano estratti **parola per parola** da
uno script (`tools/block.py`, che taglia il primo blocco recintato dopo una riga data) e applicati con il
`replace_unique.py` della testa del piano, anch'esso estratto dal piano.

**Il compito 2**
- Passo 1: `npm install --save-dev --save-exact @vitest/browser-playwright@4.1.11 playwright@1.63.0` → `npm ls` dà
  `@vitest/browser-playwright@4.1.11` con `@vitest/browser@4.1.11`, `vitest@4.1.11` invariato, `playwright@1.63.0`;
  `npm view playwright@1.63.0 scripts.install scripts.postinstall` non stampa nulla (e `npm view … scripts` neppure: il
  pacchetto non ha script); licenze `Apache-2.0` e `MIT`; `peerDependencies = { vitest: '4.1.11', playwright: '*' }`, com'è
  la (f).
- Nessun browser scaricato: `%LOCALAPPDATA%\ms-playwright` è del 2026-08-26, revisione 1234, mentre `playwright-core`
  1.63.0 vuole la 1243 (`browsers.json`); e nell'iframe `userAgentData.brands` contiene `"Google Chrome"`: gira il Chrome
  installato, canale `chrome`.
- Passo 2: il rosso è quello giusto — `Error: vitest/browser can be imported only inside the Browser Mode. Your test is
  running in forks pool.`
- Passo 3: le tre sostituzioni di `vite.config.ts` sono uniche e il file resta CRLF (CR 53 → 103, righe 53 → 103).
- Le API, nei file spediti: `playwright(options?: PlaywrightProviderOptions)` con `launchOptions?: Omit<LaunchOptions,
  "tracesDir">` e l'aumento `declare module "vitest/node" { interface BrowserCommandContext { page: Page; frame(); iframe;
  context } }` in `@vitest/browser-playwright/dist/index.d.ts`; `BrowserCommand<Payload extends unknown[] = [],
  ReturnValue = any>`, `headless?: boolean` (`@default process.env.CI`), `viewport?: { width; height }` (414 × 896),
  `commands?: Record<string, BrowserCommand<any>>`, `instances?`, `extends?: string | true` in
  `vitest/dist/chunks/reporters.d.DtoKVV2s.d.ts`.
- Il tipo del comando nelle due direzioni: senza `src/browser.d.ts` `vue-tsc` dà sei `TS2339: Property 'emulateMedia'
  does not exist on type 'BrowserCommands'`; con il file, exit 0.
- `page.emulateMedia` sulla pagina che contiene l'iframe vale anche dentro l'iframe: `matchMedia("(forced-colors:
  active)")` vero, durate a `0ms`, `data-theme` che segue lo schema in tutte e due le direzioni; finestra 1440 × 900.
- Passo 4, con R2-1 corretto: `browser` 5 prove verdi; `npm test` +1 file e +5 prove sulla baseline; `npm run build` verde.
- Passo 5: riga 1 — `channel: "chrome-that-does-not-exist"` → exit 1, `Error: browserType.launch: Unsupported chromium
  channel "chrome-that-does-not-exist"`; riga 3 — senza Barlow 300 → `expected 3 to be greater than or equal to 4`; riga
  4 — senza il blocco `prefers-reduced-motion` → `--duration-fast: expected '110ms' to be '0ms'`. (La riga 2: R2-5.)
- Passo 6: il testo `echo "-------- gui: probes"` + `npm test` è in `scripts/gate-gui.sh`, righe 36–37, una volta; il file
  è LF; il cancello fa `npm ci`, `npm run build`, `npm test`, `npm run lint` in quest'ordine, e rifatti a mano nella
  cartella di prova dopo un `npm ci` fresco sono **verdi** coi compiti 2 e 3 (correzioni R2-1 e R2-9 applicate):
  `added 341 packages`, `Test Files 16 passed | 1 skipped (17)`, `Tests 111 passed | 1 skipped (112)`, lint verde. Chrome
  come prerequisito è detto nel commento del cancello, com'è `cargo audit` in `gate.sh` riga 48; il messaggio di
  Playwright per un Chrome che manca esiste (R2-8).
- Le fonti della riga 2 del *«Come si riprende»* e del passo, alla v4.1.11 (`raw.githubusercontent.com/vitest-dev/vitest/
  v4.1.11/…`, tutte `200`): `commands.md` riga 133 *«`page` references the full page that contains the test iframe»*;
  `context.md` `viewport(width, height)`; `projects.md` `extends: true` *«to inherit the options from the root config»*;
  `css.md` *«This option is not applied to browser tests»*; `headless.md` `Default: process.env.CI`; `viewport.md`
  `Default: 414x896`; `playwright.md` `launchOptions` con `channel`. Dicono ciò che il piano fa dire loro.
- P-12: `grep -rn 'matchMedia' node_modules/jsdom/lib` non rende nulla.

**Il compito 3**
- Passo 1: `lucide@1.47.0` in `dependencies`, `npm view … license` → `ISC`; il `LICENSE` spedito porta l'ISC di Lucide e
  il MIT di Feather; nessuno script d'installazione.
- `lucide` 1.47.0: `type IconNode = [tag: string, attrs: SVGProps][]` e `export type { CreateIconsOptions, IconNode, Icons,
  SVGProps }` in `dist/lucide.d.ts`; le **24** icone di `icons.ts` sono ciascuna `declare const <Nome>: IconNode;`;
  `dist/esm/icons/search.mjs` è `[["path", { d: … }], ["circle", { cx: "11", cy: "11", r: "8" }]]`, senza `key`.
- I **diciotto** tipi di `PANEL_TYPES` (`grep -c 'module: "' gui/src/panels/registry.ts` → 18) hanno un'icona ciascuno,
  per nome; la prova lo conferma verde.
- Le icone che le tavole mostrano combaciano con la mappa (tranne la Chat, R2-15).
- `reka-ui` 2.10.4: `RadioGroupRoot` con `useVModel(…, { passive: props.modelValue === void 0 })` — un `null` dato è
  controllato, com'è P-8; `RadioGroupItem` chiama `rootContext.changeModelValue` su `onUpdate:checked`; il resto — i
  componenti di `Dialog`, `as-child` di `DialogTrigger`, gli slot — lo prova `vue-tsc` verde e le prove.
- Passo 2: `testing/axe.ts` e le tre sostituzioni in `a11y.test.ts` (file LF nell'albero, non CRLF): 5 `it(` prima e
  dopo, **11** prove prima e dopo con gli **stessi nomi** (`--reporter=verbose`, `diff` vuoto); l'aiutante ha la stessa
  semantica (`color-contrast` spento se non chiesto).
- Passi 3–6: rosso prima dei pezzi (`Failed to resolve import "./BaseIcon.vue"`); poi `npx vitest run src/components` →
  **27** verdi, 22 di `kit.test.ts` e 5 di `markdown.test.ts`, con un solo avviso (R2-18); `npm run build` verde dopo
  R2-9.
- Le attribuzioni `scoped` arrivano dove servono: l'`svg` di `BaseIcon` porta `data-v-…` (radice di un componente
  funzionale), e il dialogo, il velo, titolo, descrizione e azioni teletrasportati sul `body` portano quello di
  `BaseDialog`. `BaseDialog` senza `v-model` si apre dallo slot `trigger` e si chiude con Esc; gli attributi di
  `DialogTrigger` (`aria-haspopup`, `aria-expanded`, `data-state`) arrivano sul `<button>` di `BaseButton`.
- I 53 token che gli otto pezzi leggono esistono tutti, in `base.css` o in tutti e due i temi; nessuna scala, nessun
  colore a mano (`vars_defined.py`).
- Passo 7: le tre sostituzioni di `eslint.config.js` sono uniche (file LF); P-2 vero — prima `eslint src` legge 21 `.vue`
  e 4 `.json`, nessun `.ts`; dopo 21 `.vue`, **45** `.ts`, 4 `.json` (`--format json`); `vue/essential/rules` non ha
  `files` (83 regole); il linter resta **verde** sul codice di oggi più i compiti 2 e 3. `regex` nei `patterns` di
  `no-restricted-imports` c'è in ESLint 10.10.0 (`lib/rules/no-restricted-imports.js`, `oneOf: [{ required: ["group"] },
  { required: ["regex"] }]`). `no-raw-text` di `@intlify/eslint-plugin-vue-i18n` 4.5.1 guarda nei `.ts` solo le opzioni
  `template:` e il JSX, quindi le parole esemplari negli `h()` delle prove non scattano.
- Passo 8, le quattro righe: `BaseLabel.vue` + `../stores/core` → `'../stores/core' import is restricted from being used
  by a pattern. a base piece reads no global state…`; `moveActive.ts` + `lucide` → `'lucide' import is restricted from
  being used. icons pass through BaseIcon…`; `BaseButton.vue` + `lucide` → lo stesso; nessuna violazione → verde. Il file
  ripristinato byte per byte dopo ogni caso (`diff -r` vuoto). Che un blocco successivo **sostituisca** le opzioni e non
  le fonda è provato dal verde di `icons.ts`, che importa `lucide`.
- `@ts-expect-error` nelle due direzioni (R2-9).
- Le frasi che il compito cita dal disegno ci sono: «un test con `axe`** per ciascuno» (riga 281), «mai lo stato globale»
  (254), «no empty box» (296), «dentro una pillola va una pillola» (211); `.m-strip .sp` è nella tavola dello stile (3
  righe); le regole `.btn`, `.field`, `.row`, `.dlg` col `grep` della testa del compito (righe 303–351), e `.lab` (272) e
  `.card` (280), che il `grep` non nomina ma i pezzi usano.
- Le forme dei pezzi usate dai compiti 4–8 (`<Base…` in righe 2661–4012) sono quelle dichiarate in *Interfaces*; la riga
  3 del *«Come si riprende»* coincide col compito, salvo che non nomina `type` di `BaseTextField`, che il compito 5 usa.
- Il disegno (b): otto pezzi, gli stessi; forma, slot, `reka-ui` per dialogo e radio, nessuna scritta dentro; due regole
  del linter su quattro (le altre due al compito 5, come dice il *«Come si riprende»*); i controlli 10–13 coperti per la
  parte jsdom e linter.

## Non verificato, e perché

- Il compito 1 non è stato rivisto: la cartella di prova ne ha preso solo `extract_tokens.py`, `theme.ts`, `index.ts`,
  `dock.css` e i caratteri; `main.ts`, `App.vue`, `layout.ts` e i rinomini non ci sono. Le prove del compito 2 non ne
  dipendono (il rilievo R2-1 nasce proprio dall'assenza di `watchTheme` sulla radice).
- La corsa sulla CI (`ubuntu-latest`, `windows-latest`): non si può fare da qui; Chrome sulle due immagini è la lettura
  dei README del disegno, non una misura mia.
- Le frecce del radio di `reka-ui` sotto jsdom (il clic arriva col `setTimeout` di `RadioGroupItem`): sono del compito 5.
- Il primo avvio del progetto `browser` con molte dipendenze (l'ottimizzazione di `vite` che ricarica la pagina) si
  vedrà al compito 4, che è di un altro revisore.
- Il `tsc` di `vite.config.ts` in un `vue-tsc` senza cache: misurato solo con la cartella di prova, non sull'altra macchina.

## Comandi rilanciati

Nella cartella di prova `C:\Users\zagor\AppData\Local\Temp\probe-ds2` (gli script in `tools/`, i log accanto a `gui/`):

- `tar … | tar -xf - -C <PROVA>`, `npm ci` (327 pacchetti), `npx vitest run` baseline (15 file, 85 prove)
- `npm install --save-exact @fontsource-variable/geist@5.3.0 @fontsource/barlow@5.3.0`; `python tools/extract_tokens.py <PROVA>`
- `npm install --save-dev --save-exact @vitest/browser-playwright@4.1.11 playwright@1.63.0`; `npm ls …`; `npm view playwright@1.63.0 scripts.install scripts.postinstall`; `npm view playwright@1.63.0 scripts`; `npm view … license`; `npm view @vitest/browser-playwright@4.1.11 license peerDependencies dependencies`
- `npx vitest run src/tokens/tokens.browser.test.ts` (rosso, sotto jsdom)
- `python tools/replace_unique.py gui/vite.config.ts …` × 3; `tr -cd '\r' < vite.config.ts | wc -c`
- `npx vitest run --project browser` (dettato: 1 rosso; corretto: 5 verdi); tre sonde di diagnosi nel browser
- `npm test`; `npx vitest run --reporter=verbose | grep -oE …`; `npm run build`; `npx vue-tsc --noEmit` con e senza `browser.d.ts`
- le quattro righe del passo 5 (canale, `enabled: false`, Barlow 300, movimento)
- `curl https://raw.githubusercontent.com/vitest-dev/vitest/v4.1.11/docs/…` × 8
- `grep -n 'screenshotFailures' …reporters.d.DtoKVV2s.d.ts`; `git check-ignore -v gui/src/tokens/__screenshots__/x.png`
- `npm install --save-exact lucide@1.47.0`; `npm view lucide@1.47.0 license scripts dependencies`; `grep` su `dist/lucide.d.ts` per le 24 icone
- `grep -c 'module: "' gui/src/panels/registry.ts`; lo script dei nomi delle icone nelle tavole; `grep -n -i chat` sulle tavole
- `python tools/replace_unique.py gui/src/a11y.test.ts …` × 3; `npx vitest run --project jsdom src/a11y.test.ts --reporter=verbose` prima e dopo
- `npx vitest run src/components/kit.test.ts` (rosso prima dei pezzi); `npx vitest run src/components --reporter=verbose`; quattro sonde di diagnosi jsdom
- `npm run build` (dettato: exit 2, TS2352; corretto: exit 0); `npx vue-tsc --noEmit` con `ICONS: Record<string, IconNode>`
- `npx eslint src` prima e dopo le tre sostituzioni, e `--format json` per i file letti; `python tools/lint_directions.py` e `… extra`; `diff -r` dopo ciascuno
- `python tools/vars_defined.py`; `grep -nE '[0-9]+(px|rem|vh|vw|em|ms)'` sui pezzi
- `python tools/kit_directions.py` e `tools/kit_directions2.py` (le violazioni a mano delle prove del kit), `diff -r` dopo
- `node -e` sui blocchi di `flat/essential`; `grep` su `no-restricted-imports.js`, `no-raw-text.js`, `no-missing-keys.js`, `RadioGroupRoot.js`, `RadioGroupItem.js`, `DialogContentImpl.js`, `Dialog/utils.js`, `playwright-core/lib/coreBundle.js`
- la sequenza web del cancello: `npm ci --no-audit --no-fund && npm run build && npm test && npm run lint` → verde
- nel repository, **in sola lettura**: `git status --porcelain`, `git ls-files --eol`, `git rev-parse --short HEAD`, `git check-ignore`, `git show -s --format='%h %ci' eab020d 386fc5c`, `grep`/`sed -n`/`awk` su piano, disegno, tavole, `gui/` e `scripts/`

La cartella di prova resta, per chi corregge e vuole rifare le prove; si toglie con `rm -rf /c/Users/zagor/AppData/Local/Temp/probe-ds2`.

## Stato del repository alla fine

`git -C C:/Users/zagor/Desktop/harness status --porcelain` → uscita **vuota** (exit 0); `HEAD` = `d589d15`.

