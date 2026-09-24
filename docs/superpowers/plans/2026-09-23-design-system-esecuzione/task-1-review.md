# La revisione del compito 1 — `95068bb`, i token

> Revisore fresco, 2026-09-24 (18:52 → 19:25), macchina dell'account `Jays`, dal prompt `review-1-prompt.md`. Rivisto
> **un** commit, `95068bb` sopra `d10d9a5`. Nulla è stato scritto nell'albero del repository: il cancello e
> `check-docs.sh` ci hanno solo girato, uno alla volta. Le mutazioni e il browser sono stati fatti nel clone
> `C:\Users\Jays\AppData\Local\Temp\rv1`, lasciato a `95068bb`, pulito, col ramo `pass8` (nessun commit), `gui/node_modules`
> e un `gui/dist` del *build* di `d10d9a5`: lo cancella il coordinatore. I log e gli script stanno in
> `C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\223a9934-3feb-4768-87c2-d14a64ff36ab\scratchpad\review1\`
> (d'ora in poi `<rv>`).

## 1. Il verdetto

**Conforme.** Il commit è il testo del piano, byte per byte: `compare_task1.py` è stato provato nelle due direzioni e dice
`0 not matching`, e i tre file da guardare a mano tornano. Ogni *Atteso* rilanciato torna (Passo 8, le otto violazioni del
Passo 16, il cancello, il Passo 17 nei due temi), come tornano i vincoli e il contratto. **Nessun rilievo Critico.** I
rilievi colpiscono il **dettato** (sette voci d'errata candidate, da E2 a E9, due delle quali vanno al proprietario) e **una
frase del rapporto**, non l'esecuzione.

## 2. I rilievi

**Critici 0 · Importanti 2 · Minori 4 · Nit 3.**

| # | Classe | Dove | In breve | Chi cura |
|---|---|---|---|---|
| I-1 | Importante | `gui/src/frame/Drawer.vue:34`; `task-1-report.md:481` | il rapporto dice vero, senza averlo misurato, un commento che è falso: i gruppi galleggianti stanno a **999**, non a 99, e coprono il cassetto | il coordinatore (il commento non è testo del piano) |
| I-2 | Importante | `gui/src/stores/stores.test.ts:178`; `layout.ts:100` | manca la sonda: un `chooseTheme` che perdesse le disposizioni salvate resterebbe verde | **E2**, testo provato |
| M-1 | Minore | `gui/src/tokens/theme.test.ts:63`; `theme.ts:48` | lo *stop* di `watchTheme` è provato a metà: il `watch` di Vue lasciato vivo resta verde | **E3**, testo provato |
| M-2 | Minore | `gui/src/tokens/dock.css:20` | nel tema chiaro i controlli nativi **dentro** il dock si disegnano scuri (`color-scheme: dark` di `themeAbyss`): il radio non scelto sembra scelto | **E4** |
| M-3 | Minore | `gui/src/tokens/themes.css:6` = `token.html:164` | il commento copiato dalla tavola dice «every non-text role 3:1»; i bordi di decoro stanno fra 1,13 e 1,88 | **E5**, sulla tavola: al proprietario |
| M-4 | Minore | `gui/src/tokens/usage.test.ts:39` | «writes no colour by hand» non vede un colore per nome (`color: white`) | **E6**, sul controllo 5: al proprietario |
| N-1 | Nit | `gui/src/tokens/contrast.test.ts:34`, `:38` | cita WCAG 2.2 con la soglia di prima del 2021 (0.03928); nessun effetto sui colori a 8 bit | **E7** |
| N-2 | Nit | `gui/src/tokens/theme.ts:23` | «the dock's `colorScheme` today (task 6)»: oggi `shownTheme` non ha lettori | **E8** |
| N-3 | Nit | `gui/src/panels/Chat.vue:92` | un ruolo di **testo** per un bordo, e il perché (P-13) sta solo nel piano | **E9** |

### I-1 — Importante: il rapporto certifica un commento falso di `Drawer.vue`

- **Dove.** `grep -n 'floating groups are at 99' gui/src/frame/Drawer.vue` → `34`: *«Both sit above `dockview`, whose
  floating groups are at 99.»* E il rapporto, `grep -n 'at 99' task-1-report.md` → `481`: *«resta vero con `--z-overlay` a
  200»*.
- **Evidenza.**
  - In `dockview-core` 8.3.1, `gui/node_modules/dockview-core/dist/package/main.cjs.js:12025`:
    `` this._orderedList[i].style.zIndex = `calc(var(--dv-overlay-z-index, 999) + ${i * 2})` ``; e `dockview.css:1037` ridichiara
    `--dv-overlay-z-index: var(--dv-overlay-z-index, 999)` sullo stesso contenitore. Il riferimento a sé stessa è un ciclo,
    quindi vale il ripiego: 999.
  - Misurato nel browser (clone, tema chiaro, 800×450). Dopo «⧉» su «Costi», lo `z-index` calcolato di
    `.dv-resize-container` è `"999"`, in linea `calc(var(--dv-overlay-z-index, 999) + 0)`, e nessun antenato apre un contesto
    di impilamento. Col cassetto aperto da «+ moduli», `document.elementFromPoint(291, 250)` e `(291, 420)` — il secondo
    punto è nell'area del cassetto — rendono `div.dv-view-container` **dentro** il gruppo galleggiante; `(700, 100)` rende
    `div.drawer-overlay`. Velo e cassetto hanno `z-index` `200`. Nella schermata la lista «I moduli» è coperta dal
    gruppo «Costi», che resta anche fuori dal velo.
- **Che cosa non è.** Non è una regressione: 100 e 101 stavano già sotto 999. Il piano conosce il difetto di fondo, R3-17,
  e lo cura al compito 6 (righe 4990–4997 e 5423 del piano: `--dv-overlay-z-index: var(--z-floating)` sul contenitore, con
  la prova `expected 999 to be 50`). Il commento sparisce col compito 5, che riscrive `Drawer.vue` per intero (riga 4036).
  Però dopo questo commit il commento sta sopra le due righe che il compito ha riscritto e dice il falso (gotcha #58), e il
  rapporto lo dà per vero senza averlo misurato.
- **Cura proposta.** Non è un difetto del dettato, perché il commento non è testo del piano: decide il coordinatore.
  (1) Registrare che la frase del rapporto è falsa. (2) Nel codice, **o** nulla fino al compito 5, dichiarandolo nella
  consegna, **o** la frase corretta col richiamo datato, per esempio:
  `` Both sit above the docked groups. Corrected on 2026-09-24: `dockview`'s FLOATING groups sit at ``
  `calc(var(--dv-overlay-z-index, 999) + 2i), over these, until task 6 moves them to --z-floating (R3-17 of the`
  `design-system review).`

### I-2 (E2) — Importante: manca la sonda della scelta che conserva il pacchetto

- **Dove.** `grep -n 'keep({ layouts: {}, ...(saved.value ?? {})' gui/src/stores/layout.ts` → `100`. La terza prova del
  negozio, `grep -n 'layout.chooseTheme("dark");' gui/src/stores/stores.test.ts` → `178`; nel piano è il Passo 7, riga 672.
- **Evidenza.** La mutazione G2 nel clone, `keep({ layouts: {}, view: view.value, theme: choice });` (tolto lo *spread*),
  seguita da `npx vitest run src/tokens src/stores`, dà `Test Files  7 passed (7)` e `Tests  38 passed (38)` (log
  `<rv>/gaps.out`). La prova chiama `chooseTheme` su un negozio vuoto, dove `saved` è `null`, quindi lo *spread* non si vede.
- **Perché conta.** Dal compito 5 la scelta si fa in Impostazioni, **dopo** l'arrivo del pacchetto del core. Un
  `chooseTheme` che ricostruisse il pacchetto attorno al solo tema manderebbe `layouts: {}`, e il core lo custodirebbe:
  ogni disposizione salvata andrebbe persa, in silenzio, e dal compito 7 anche le viste col nome. È la regola *«Gli
  invarianti diventano test»* di `CLAUDE.md`, domanda 2 delle quattro: *«la sonda manca»*.
- **E2, testo proposto.** Provato nel clone nelle due direzioni con `<rv>/proposals.py`, log `<rv>/proposals.out`. Nel
  Passo 7, nella terza prova, dopo `layout.attach(bridge);`:

  ```ts
      const work = { marker: "work, as the owner left it" } as never;
      layout.receive({ kind: "Layout", value: { state: "Package", bytes: [...pack_({ view: "home", layouts: { work } })] } });
  ```

  e le due attese diventano:

  ```ts
      // ⛔ NOT THE CHOICE ALONE: a choice that rebuilt the package around the theme would drop every layout the owner
      // saved, and the core would keep the loss.
      expect(chosen).toEqual({ view: "home", layouts: { work }, theme: "dark" });
      …
      expect(settled).toEqual({ view: "home", layouts: { work, home }, theme: "dark" });
  ```

  Il titolo resta quello di prima, così la tabella del Passo 16 resta vera; le si aggiunge una nona riga:
  `` `stores.test.ts`, la scelta che conserva | in `chooseTheme` tolto `...(saved.value ?? {}),` | rosso: `sends the choice at once, and a settle after it keeps it` ``.
  Le uscite: sul codice di `95068bb`, `Tests  14 passed (14)`. Con G2, `FAIL … sends the choice at once, and a settle after
  it keeps it` e `AssertionError: expected { view: 'home', layouts: {}, …(1) } to deeply equal { view: 'home', …(2) }`.
  Con la violazione 8 del Passo 16 è ancora rosso: `expected { view: 'home', layouts: { …(2) } } to deeply equal …`.

### M-1 (E3) — Minore: lo *stop* di `watchTheme` è provato a metà

- **Dove.** `grep -n 'return () => {' gui/src/tokens/theme.ts` → `48`. La seconda prova, `grep -n 'stop();'
  gui/src/tokens/theme.test.ts` → `41 63`, quella alla riga 63; nel piano è il Passo 6, riga 632.
- **Evidenza.** G1 (tolto `stop();` dalla funzione resa, così il `watch` sulla scelta resta vivo) → `Tests  38 passed (38)`.
  G4 (tolto `removeEventListener`) → rosso, `expected 1 to be +0`. La prima prova conta gli ascoltatori del sistema, e
  nessuna guarda il `watch`. Il codice è giusto, e `main.ts` lo scarta. Ma il commento della prova, *«A stopped watch that
  still listened would fight the next one»*, promette l'intero.
- **E3, testo proposto.** Provato come E2. Nel Passo 6, seconda prova, dopo `stop();`:

  ```ts
      // ⛔ THE OTHER HALF OF THE STOP: the watch on the choice ends too, not only the system's listener.
      choice.value = "light";
      await nextTick();
      expect(root.dataset.theme).toBe("dark");
  ```

  Su `95068bb` dà `Tests  3 passed (3)`. Con G1 dà `` FAIL … lets `light` and `dark` win over the system, and moves when
  the choice does `` e `expected 'light' to be 'dark'`. Nel Passo 16 si aggiunge una riga: `theme.test.ts`, lo *stop* | tolto
  `stop();` dalla funzione resa | rosso.

### M-2 (E4) — Minore: nel tema chiaro il dock disegna scuri i controlli nativi

- **Dove.** Il blocco del ponte, `grep -n '.dockview-theme-abyss {' gui/src/tokens/dock.css` → `12 20`, quello alla riga
  20; nel piano è il Passo 11, riga 957. La causa sta in `gui/node_modules/dockview/dist/styles/dockview.css:1787–1788`:
  `.dockview-theme-abyss { … color-scheme: dark; }`, che ha la stessa specificità del ponte.
- **Evidenza.** Nel browser del clone, con `data-theme="light"`, `getComputedStyle(…).colorScheme` vale `light` sulla
  radice e su `.dock`, `dark` su `.dockview-theme-abyss`, sul radio di Impostazioni e sulla `section.status` di Stato. A
  grandezza vera (Impostazioni ingrandita, 800×500), il radio **scelto**, «OpenRouter, VRAM libera», è un anello azzurro
  chiaro, e quello **non scelto**, «Locale», è un disco pieno grigio scuro: si legge come se fosse scelto il secondo. Le
  barre di scorrimento di Stato, Permessi e Impostazioni sono scure sul fondo avorio. Per sola ispezione ho iniettato
  `.dockview-theme-abyss { color-scheme: inherit; }`, poi l'ho tolto: i tre valori passano a `light`, e i radio tornano
  giusti, lo scelto pieno e l'altro vuoto. Nel tema scuro non cambia nulla.
- **Perché è sfuggito.** Lo *snippet* del Passo 17 misura il testo e non vede questo, e il «guardato» del rapporto non lo
  nomina. Il piano non conosce questo `color-scheme` **CSS**: parla solo dell'opzione `colorScheme` del tema, che non
  disegna nulla (R3-21). Il difetto è transitorio: il radio esce col compito 5 (`BaseRadioGroup`) e `themeAbyss` col 6.
- **E4, testo proposto.** Nel blocco del ponte di `dock.css` (Passo 11) una riga in più, `color-scheme: inherit;`, con la sua
  ragione nel commento del blocco. Per esempio: *«… and `themeAbyss` sets `color-scheme: dark` as well: without the line
  below, the native controls inside the dock -- scrollbars, the radios of Impostazioni -- are drawn dark under the light
  theme.»* Nell'*Atteso* del Passo 17, nei due temi, si aggiunge
  `getComputedStyle(document.querySelector(".dock input[type=radio]")).colorScheme` uguale al tema. In alternativa lo si
  accetta fino ai compiti 5 e 6, dichiarato: decide il coordinatore.

### M-3 (E5, sulla tavola) — Minore: il commento di `themes.css` dice il falso sui ruoli non-testo

- **Dove.** `grep -n 'every non-text role 3:1' gui/src/tokens/themes.css
  docs/superpowers/specs/2026-09-22-design-system-tavole/token.html` → `themes.css:6` e `token.html:164`. È la riscrittura
  per famiglie che P-1 ha fatto nella tavola (riga 173 del piano): *«… and every non-text role 3:1 …»*.
- **Evidenza.** Il calcolo WCAG sui valori di `themes.css`, sui quattro fondi di base: nello scuro `color-border` sta fra
  1,13 e 1,30 e `color-border-ok`/`-warn`/`-stop` fra 1,62 e 1,88; nel chiaro `color-border` fra 1,20 e 1,41,
  `ok`/`warn`/`stop` fra 1,37 e 1,61, e `color-border-card` è `transparent`. La prova li esenta per nome (`EXEMPT` in
  `contrast.test.ts`, «decoration…»), ma il commento dice «every».
- **Cura.** La copia non si ritocca (vincolo 2, D1): la frase si corregge **nella tavola**, poi si ricopia con
  `extract_tokens.py`, e `board.test.ts` le tiene uguali. Testo proposto: *«Every text role reads 4.5:1 on the backgrounds
  of its FAMILY, and the non-text roles that tell a control apart -- strong border, focus, mark, accent border -- 3:1 on the
  base backgrounds; the decoration borders and the veil are exempt, each with its reason in the contrast test.»* La
  tavola è approvata: decide il proprietario, come per la riscrittura di P-1.

### M-4 (E6, sul controllo 5) — Minore: la prova dei colori a mano non vede un colore per nome

- **Dove.** `grep -n 'writes no colour by hand\|const HAND' gui/src/tokens/usage.test.ts` → `39` e `40`.
- **Evidenza.** G3, `color: white;` in `panels/Strip.vue` → `Tests  38 passed (38)`. Per `oklch(`, `lab(` e `color-mix(`
  vale lo stesso: la regex conosce solo `#…`, `rgb(`, `hsl(`. Questo l'ho **dedotto** dalla regex, senza provarlo caso per
  caso. Oggi in `gui/src` non c'è nessun colore per nome: il `grep` sulle proprietà di colore non rende nulla.
- **Cura.** La prova fa alla lettera il controllo 5 del disegno (*«nessun `#…`, `rgb(`, `hsl(`»*), e allargarla tocca il
  merito approvato (vincolo 1). La scelta è del proprietario: tenere il controllo com'è, dichiarando il limite nel titolo
  o nel commento della prova, oppure allargarlo ai nomi CSS e alle funzioni di CSS Color 4.

### N-1 (E7) — Nit: la soglia di WCAG 2.2

`grep -n 'WCAG 2.2 relative luminance\|0.03928' gui/src/tokens/contrast.test.ts` → `34` e `38`; nel piano è il Passo 4,
riga 405. La fonte è WCAG 2.2, tecnica G18 (<https://www.w3.org/WAI/WCAG22/Techniques/general/G18>), letta oggi: *«if RsRGB
<= 0.04045 …»*, con la nota *«Before May 2021 the value of 0.04045 in the definition was different (0.03928)»*. Effetto
nessuno: con `python -c` nessun canale a 8 bit cade fra le due soglie (10/255 = 0,039216; 11/255 = 0,043137). Cura:
`0.04045`, come nello *snippet* del Passo 17, e la fonte in `riferimenti.md`.

### N-2 (E8) — Nit: «today (task 6)»

`grep -n "colorScheme\` today" gui/src/tokens/theme.ts` → `23`; nel piano è il Passo 10, riga 778. `grep -rn 'shownTheme'
gui/src` trova solo `theme.ts` e `theme.test.ts`, quindi oggi nessuno lo legge. Cura: *«the dock's `colorScheme` from task
6»*.

### N-3 (E9) — Nit: il perché del bordo della provenienza non sta nel sorgente

`grep -n 'border-left: 3px solid var(--color-text-warn)' gui/src/panels/Chat.vue` → `92`. La ragione — P-13: un bordo che
deve leggersi a 3:1, mentre `--color-border-warn` è decoro, fra 1,62 e 1,88 — vive nella tabella del Passo 13 (riga 1103
del piano), non nel file. Nessuna prova giudica gli usi, solo i ruoli, e chi rilegge `Chat.vue` può «correggerlo» in
`--color-border-warn`. Cura: un commento accanto, che lo script dei nomi non può scrivere e che quindi chiede un passo suo.
Per esempio: `/* A TEXT ROLE ON PURPOSE (P-13 of the design-system plan): this border must read 3:1, and`
`--color-border-warn is decoration. The words carry the provenance. */`.

### Verificato, e senza rilievo

- **Il tema sulla radice prima del *mount*.** In `main.ts` c'è `watchTheme(() => layout.theme)` prima di
  `app.mount("#app")`, col `watch` `immediate`, che è sincrono: lo prova la prima prova di `theme.test.ts`, che legge
  `dataset.theme` senza attendere. Nel browser il cablaggio vero, col `matchMedia` della pagina, segue il sistema nelle due
  direzioni: lo schema emulato passa a scuro e poi a chiaro, e `data-theme` e il fondo lo seguono dopo una resa.
- **`settle` conserva il resto del pacchetto** (violazione 8), e **`unpack` legge una scelta sconosciuta come assente**
  (violazione 7).
- **Il commento di `a11y.test.ts:31`** (*«on every text colour over every surface»*) è falso da questo commit, ed è
  dichiarato: il compito 3 toglie l'intero blocco, dal `/**` a *«Fills the stores»* (righe 1747–1752 del piano). Il testo
  nuovo di `testing/axe.ts` dice *«holds the families of the pairs»* (riga 1711).
- **Le variabili CSS.** Ogni `var(--…)` usato in `gui/src` ha una definizione (`<rv>/undefined_vars.py`: 83 nomi usati;
  l'unico «mancante» è la stringa `var(--ref-` dentro `usage.test.ts`). Nessun nome vecchio sotto `gui/`, nessun colore a
  mano nei `.ts` fuori da `tokens/`, e nessun documento vivo, fuori da `plans/`, `specs/` e `archivio/`, nomina ancora
  `tokens.css`.
- **Il ponte vince su `dockview.css`.** Nello scuro il gruppo è `rgb(27, 23, 24)` = `--color-bg-surface` e il contenitore
  delle linguette `rgb(21, 17, 18)` = `--color-bg`: il rettangolo scuro accanto alle linguette è quello che il ponte
  vuole.
- **`compare_task1.py` ricostruisce davvero il dettato.** I 23 numeri della ricetta aprono ciascuno il recinto giusto
  (riletti uno per uno, riga prima e prima riga del recinto). I tagli sono la regola del Passo 9, le rinomine il letterale
  `RENAMES` del piano, e la riga del compendio quella del Passo 14. I suoi limiti, misurati: non confronta i tre file «a
  mano» (M12: `package.json` a `5.3.1` → exit 0), è cieco ai fine-riga (M13) e al separatore dell'aggiunta (M14). Usa
  `git diff --name-only` col rilevamento dei rinomini: qui è senza effetto, perché `--name-status` dà `D` per `tokens.css`
  e `--no-renames` gli stessi 29 percorsi, ma un `--no-renames` lo renderebbe robusto.
- **`seen` dipende dal viewport.** Fa 39 a 800×450 in tutti e due i temi, da un caricamento pulito; è salito a 42 dopo le
  mie interazioni (ingrandimento, cambi di viewport). L'*Atteso* chiede solo che sia sopra zero; per confrontarlo col
  pre-controllo va detto il viewport.

## 3. I comandi rilanciati

Nell'**albero del repository**, uno alla volta:

| Comando | La mia uscita | Il rapporto |
|---|---|---|
| `git rev-parse --short HEAD`; `git status --porcelain`; `git log --oneline d10d9a5..HEAD` | `95068bb`; vuoto; una riga | — |
| `git status -sb` | `## main...origin/main [ahead 1]` | uguale |
| `bash scripts/gate.sh`, da solo, in background → `<rv>/gate-review1-2026-09-24.log` | `GATE GREEN.`, `real 1m0.617s` (18:54:24 → 18:55:24); sotto `gui/`: `added 329 packages in 9s`, `Test Files  17 passed \| 1 skipped (18)`, `Tests  100 passed \| 1 skipped (101)`, `dist/assets/index-Cpli-8MB.js                              663.93 kB │ gzip: 201.53 kB`, `index-FUXF9M4Y.css 145.07 kB │ gzip: 14.03 kB`, **29** righe `.woff2`/`.woff`, la sezione `gui: lint`, `found 0 vulnerabilities` | uguale |
| `bash scripts/check-docs.sh` → `<rv>/check-docs.log` | `OK — no inconsistencies.` | uguale |
| `wc -c docs/COMPENDIO.md` (e a `d10d9a5`); `grep -n '^ceiling=' scripts/check-docs.sh` | `89824` (era `89790`); `356:ceiling=100352` | uguale |
| `PYTHONIOENCODING=utf-8 python …/compare_task1.py d10d9a5 95068bb` → `<rv>/compare-real.out` | 26 `OK` (`stores.test.ts` con separatore `'\n\n'`), 3 `CHECK BY HAND`, `--- 29 paths, 0 not matching the plan's text`, exit 0 | — |
| `git diff --numstat d10d9a5 95068bb --` i tre file «a mano» | piano `1 1`, `package.json` `3 1`, `package-lock.json` `20 0` | `4 +++-` |
| `git diff --word-diff … -- <piano>`; `git diff -U0` | solo la riga 127: `\| — \|[- ⬜ -]{+ ✅ 2026-09-24 +}\|`; Commit resta `—` | uguale |
| le righe aggiunte del lockfile | solo le due voci, `5.3.0`, `"license": "OFL-1.1"`, `resolved` da `registry.npmjs.org` | uguale |
| `npm view @fontsource-variable/geist@5.3.0 license dist.integrity` (e `@fontsource/barlow@5.3.0`) | `OFL-1.1` e l'`integrity` identica a quella del lockfile, per tutti e due | `OFL-1.1` |
| `git ls-files --eol` sui 28 percorsi A/M, `grep -v 'i/lf    w/lf'` | nulla | nulla |
| `tr -cd '\r' \| wc -c` sui blob di `d10d9a5`, di `95068bb` e sull'albero | `0` ovunque; i nuovi `0` | `0` |
| `wc -l` sui 28 file del §5 del rapporto, piano compreso | nessuna differenza (piano `8555`) | — |
| `git diff --stat d10d9a5..95068bb -- crates/ gui/schema/` (vincolo 12); lo stesso con `scripts/ .github/ Cargo.toml Cargo.lock docs/adr/ docs/HANDOFF.md docs/archivio/` | vuoto; vuoto | vuoto |
| `git diff --name-only` fuori dalla lista del `git add` del Passo 18 | nulla | — |
| `git rev-list --count d10d9a5..95068bb`; `cmp` fra `git log -1 --format=%s` e il `-m` del Passo 18 | `1`; identici, 369 byte | — |
| `git log -1 --format=%B 95068bb \| grep -ci co-authored` | `0` | `0` |
| le due righe del pezzo JavaScript nel messaggio | la seconda uguale al *build* del mio cancello; la prima uguale al *build* di `d10d9a5` (sotto) | — |
| `git log -4 --format='%h %an <%ae>'` | `devfrx <zagor2012@icloud.com>` su `95068bb`, `d10d9a5`, `6ae9b8b`, `923ee52` | uguale |
| il compito del brief contro quello del piano a `d10d9a5` | `plan lines 1006 brief lines 1006 identical True` | uguale |

Nel **clone**, a `95068bb` salvo dove detto:

| Comando | La mia uscita | Il rapporto |
|---|---|---|
| `git clone …`, `checkout --detach 95068bb`, `npm ci` → `<rv>/clone-npm-ci.log` | `added 329 packages, and audited 330 packages in 6s`, `found 0 vulnerabilities`; `npm ls` → i due a `5.3.0` | — |
| `<rv>/mutate_compare.py`: 14 commit mutanti, lo script lanciato dal clone → `<rv>/compare-mutants.out` | da M1 a M11 (`theme.ts` W, `layout.ts` R, `stores.test.ts` A, `themes.css` e `base.css` tagli, `Strip.vue` rinominato, `Frame.vue` non dettato, compendio, `main.ts` RL, `tokens.css` ripristinato, un file nuovo): ciascuno `DIFFERS`/`UNEXPECTED`/`NOT DELETED` **sul solo file mutato**, exit 1. M12–M14, i limiti: exit 0 | — |
| Passo 8: ramo `pass8` da `d10d9a5` con le cinque prove di `95068bb`, `npx vitest run src/tokens src/stores` → `<rv>/pass8.log` | `Test Files  5 failed \| 2 passed (7)`, `Tests  6 failed \| 22 passed (28)`. Le rosse: `board.test.ts` ×2 `ENOENT` (`base.css`, `themes.css`); `contrast.test.ts` non carica, `ENOENT … themes.css`; `usage.test.ts` `to include 'tokens/dock.css'` e i due veli; `theme.test.ts` `Failed to resolve import "./theme"`; il negozio `expected undefined to be 'light'` e `layout.chooseTheme is not a function`. Verde: la scelta sconosciuta | uguale |
| Passo 16: `<rv>/violations.py`, otto violazioni una alla volta, copia salvata e `cmp` → `<rv>/violations.out` | 1 `themes.css is the board's block`; 2 `"color-text-muted on color-bg: 3.34 < 4.5"` e altre dieci; 3 `expected [ 'color-bg-test' ] to deeply equal []`, scuro e chiaro; 4 `[ 'panels/Strip.vue' ]`; 5 `'panels/Strip.vue:30: color: #fff;'`; 6 `expected 'dark' to be 'light'` su due prove; 7 `"theme": "purple"`; 8 `-   "theme": "dark"`. `cmp` vero, `status` uguale dopo ciascuna; vuoto alla fine | uguale |
| `<rv>/gaps.py`, G1–G4 → `<rv>/gaps.out` | G1, G2, G3 verdi, 38 su 38; G4 rosso, `expected 1 to be +0` | — |
| `<rv>/proposals.py`, le prove di E2 ed E3 → `<rv>/proposals.out` | vedi I-2 e M-1: verdi sul codice, rosse sulla mutazione | — |
| `npx vitest run src/tokens/board.test.ts src/tokens/contrast.test.ts` | `Tests  9 passed (9)` | uguale |
| `npx vitest run src/tokens/theme.test.ts src/stores` | `Tests  26 passed (26)` | uguale |
| la lunghezza di `base.css` e di `themes.css` | `3554` e `5838` caratteri | uguale |
| il censimento, da `<rv>/census-1.sh` (uguale alle righe 288–291 del piano), a `d10d9a5` | **50** righe, `diff` con quelle del rapporto: nessuna differenza | uguale |
| lo stesso a `95068bb` con `tokens.css` di `d10d9a5` rimesso, cioè lo stato del Passo 13 | **12** righe: le dieci di `tokens.css` e i due `z-index: var(--z-overlay);` | uguale |
| `npm run build` a `d10d9a5` → `<rv>/build-d10.log` | `dist/assets/index-DXs5ZA4m.js   663.26 kB │ gzip: 201.23 kB`; `index-WKtFNNMF.css  131.31 kB │ gzip:  11.51 kB` | uguale |

Nel **browser dell'app**, col server lanciato dal clone (`npm run dev` in background → `<rv>/dev.log`, ciclo limitato su
`netstat`: `[::1]:5173 LISTENING 27832` al primo giro; il processo `node … rv1\gui\node_modules\.bin\..\vite\bin\vite.js`),
`preview_start` su `http://localhost:5173`:

| Che cosa | La mia uscita | Il rapporto |
|---|---|---|
| al caricamento | `data-theme` `dark` (sistema scuro), `body` `rgb(21, 17, 18)` su `rgb(236, 230, 218)` | uguale |
| `harnessFake.deliverAll()` | 7 linguette, 7 prese grandi, 7 pannelli, nessun gruppo galleggiante | uguale |
| lo *snippet* del Passo 17, **scuro**, caricamento pulito, 800×450 (cambio di tema e *snippet* in due chiamate) | `seen: 39`, `worst` `[5.86, "⧉"]`, `[5.86, "⤢"]`, `[5.86, "⧉"]` | uguale |
| lo stesso, **chiaro** | `seen: 39`, `worst` `[6.22, "Stato"]`, `[6.22, "⧉"]`, `[6.22, "⤢"]` | uguale |
| `font-family` calcolata | `"Geist Variable", system-ui, sans-serif` su `.bigtab-title` «Stato», su un `dt` «Degrado» e sui pulsanti della presa grande (26×26); i pulsanti nativi della barra `Arial` (R1-11); `document.fonts.check('14px "Geist Variable"')` → `true`, caricato `Geist Variable 100 900`, sottoinsieme latino; la fascia delle linguette alta `40px` | uguale |
| guardato, **scuro** | fondo carbone, testo avorio; la fascia in ambra con la sua riga; «Core: timbro diverso» in rosso chiaro; Stato, Permessi, Impostazioni a sinistra, Knowledge base al centro senza intestazione, Attività e Costi a destra, la striscia in basso; linguette e pulsanti «⧉» «⤢» contornati e leggibili; testi lunghi di Stato tagliati dal pannello piccolo, che scorre. Niente di illeggibile | — |
| guardato, **chiaro** (800×450 e 1280×800) | fondo avorio, testo scuro, la fascia in ambra, tutto leggibile. **Fuori posto**: le barre di scorrimento scure e il radio di Impostazioni resi scuri, M-2 | — |
| il sistema che cambia | lo schema emulato a scuro, poi a chiaro: `data-theme` e il fondo lo seguono, dopo una resa | — |
| il gruppo galleggiante e il cassetto | `z-index` `999`; il cassetto coperto, I-1 | — |
| lo spegnimento | `taskkill //F //PID 27832` → `OPERAZIONE RIUSCITA`; `netstat … :5173 … LISTENING` → nulla; nessun `node.exe` rimasto; scheda chiusa, viewport e schema riportati prima | uguale |

## 4. Ciò che non ho potuto verificare, e perché

- **Il Passo 2 non è stato rilanciato**: `npm install --save-exact` va in rete e riscrive manifesto e lockfile. Al suo posto
  ho verificato il prodotto: il diff, `npm ls`, e licenza e integrità contro il registro.
- **La storia dell'esecuzione non si rimisura**: gli orari del rapporto, il `git status` prima del `git add`, i log
  nello scratchpad dell'implementatore. Il contenuto del commit l'ho invece verificato per intero.
- **La CI di `95068bb`**: non esiste ancora, perché il commit non è pushato (lo fa il coordinatore, dopo la revisione).
- **I rapporti 1,11:1 e 1,06:1 senza il ponte**, scritti come «(measured)» in `dock.css` (R1-10): sono del piano, e non li
  ho rimisurati.
- **L'altra macchina** (`zagor`, albero CRLF) non era disponibile. Le prove normalizzano il CRLF per costruzione
  (`normal` in `board.test.ts`, `split(/\r?\n/)` in `usage.test.ts`) e lo script di confronto lo legge come LF, ma lì non
  ho girato nulla.
- **La prima pittura prima di ogni script** (R1-13) è del guscio, sotto-progetto 10: fuori da questo compito.
- **Le altre forme di colore a mano** di M-4 (`oklch(`, `lab(`, `color-mix(`) le ho dedotte dalla regex, non provate una
  per una.
- **Le schermate.** Il riquadro del browser era nascosto: gli scatti sono riusciti al secondo tentativo, e `zoom` rende
  il viewport intero in scala. Ogni osservazione qui sopra dice il viewport su cui è stata fatta.
