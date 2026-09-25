# Il rapporto del compito 3 — il kit: gli otto pezzi di base, la mappa delle icone, le regole del linter

Implementatore del compito 3, macchina `zagor`, il 2026-09-25. Il mandato: `dispatch-task-3.md`; il compito: `task-3-brief.md`,
letto per intero.

## 1. Lo stato

**DONE** — il commit è **`c7b7bcd`** (`c7b7bcd6a286b97a898be9699b9e0d666a4c33f0`), sopra `ef3e865`, **senza co-autore**
(`git log -1 --format=%B | grep -ci co-authored` → `0`), **non pushato**: il push è del coordinatore, dopo la revisione.
Nessuna voce candidata d'errata: una sola differenza da un riferimento del pre-controllo, al §8, che il testo del piano non
smentisce.

## 2. `git show --stat HEAD`

```
commit c7b7bcd6a286b97a898be9699b9e0d666a4c33f0
Author: devfrx <zagor2012@icloud.com>
Date:   Fri Sep 25 13:18:10 2026 +0200

    design-system(compito 3): il kit -- lucide 1.47.0, il pacchetto dei soli disegni, appuntato fra le dipendenze, [...] check-docs OK, GATE GREEN

 docs/superpowers/plans/2026-09-23-design-system.md |   4 +-
 gui/eslint.config.js                               |  66 +++++-
 gui/package-lock.json                              |   7 +
 gui/package.json                                   |   1 +
 gui/src/a11y.test.ts                               |  15 +-
 gui/src/components/BaseButton.vue                  | 147 +++++++++++++
 gui/src/components/BaseDialog.vue                  | 109 ++++++++++
 gui/src/components/BaseIcon.vue                    |  55 +++++
 gui/src/components/BaseLabel.vue                   |  33 +++
 gui/src/components/BaseList.vue                    |  39 ++++
 gui/src/components/BaseRadioGroup.vue              | 105 ++++++++++
 gui/src/components/BaseStatus.vue                  |  12 ++
 gui/src/components/BaseTextField.vue               |  97 +++++++++
 gui/src/components/icons.ts                        |  73 +++++++
 gui/src/components/kit.test.ts                     | 232 +++++++++++++++++++++
 gui/src/testing/axe.ts                             |  15 ++
 16 files changed, 991 insertions(+), 19 deletions(-)
```

Il messaggio intero è nel commit (`git log -1 --format=%B c7b7bcd`); comincia con `design-system(compito 3): il kit`. I sedici
file sono **solo** quelli del punto 2 del contratto. Prima del commit `git status --porcelain` li nominava tutti e nient'altro;
dopo, è vuoto.

## 3. L'avvio, e il cancello d'apertura

| Verifica | Uscita |
|---|---|
| `git rev-parse --short HEAD` | `ef3e865` |
| `git status --porcelain` | vuoto |
| `node --version` | `v24.19.0` — `engines` di `gui/package.json`: `^22.22.2 \|\| ^24.15.0 \|\| >=26.0.0` |
| `git config --show-origin --get-all core.autocrlf` | `file:C:/Program Files/Git/etc/gitconfig	true` |
| Chrome | `ls "/c/Program Files/Google/Chrome/Application/"` → `153.0.8010.53` e `154.0.8037.58`, con `new_chrome.exe`; la cartella sotto `$LOCALAPPDATA` non c'è |

**Il cancello d'apertura**, a `ef3e865`, da solo, in background — `bash scripts/gate.sh`, 12:49:58 → 12:55:23:

```
GATE GREEN.
added 340 packages in 58s                                  (npm ci)
dist/assets/index-DLsd9Y_U.js    663.93 kB │ gzip: 201.53 kB
> vitest run --project jsdom
 Test Files  17 passed | 1 skipped (18)
      Tests  100 passed | 1 skipped (101)
> vitest run --project browser
 Test Files  1 passed (1)
      Tests  5 passed (5)
found 0 vulnerabilities
```

Tutto come il §3 del mandato. L'avviso `npm warn allow-scripts` su `vue-demi@0.14.10` c'è già qui.

**Il linter prima del compito**, sotto `gui/`: `npx eslint src --format json`, contato per estensione con Python → `files: 17
{'.json': 4, '.vue': 13} errors: 0 warnings: 0`; `find src -name '*.ts' | wc -l` → `45`.

## 4. Passo per passo

Ogni file nuovo è stato **estratto dal brief byte per byte** (uno script nello scratchpad, `blocks.py`, che scrive il corpo di un
recinto; prima di usarli ho controllato che ogni recinto compaia **tale e quale** nel piano a `HEAD`, fine-riga a parte), mai
ricopiato a mano. Dopo il commit: `git show HEAD:<file> | cmp - <recinto>` → uguali tutti e undici i file nuovi.
`replace_unique.py` è la copia degli *Strumenti* del brief, nello scratchpad: `diff` col recinto → identico.

### Passo 1 — `lucide`, in due passi

```
$ (cd gui && npm install --save-exact lucide@1.47.0 && npm ls lucide && npm view lucide@1.47.0 license)
added 1 package, and audited 342 packages in 28s
104 packages are looking for funding
found 0 vulnerabilities
npm warn allow-scripts 1 package has install scripts not yet covered by allowScripts:
npm warn allow-scripts   vue-demi@0.14.10 (postinstall: ...)
harness-gui@0.0.0 C:\Users\zagor\Desktop\harness\gui
`-- lucide@1.47.0
ISC
```

- `git diff gui/package.json` → `+    "lucide": "1.47.0",` in `dependencies`, fra `dockview-core` e `markdown-it`, versione esatta.
- `git diff gui/package-lock.json` → la stessa riga fra le dipendenze della radice, e la voce `node_modules/lucide` con
  `"version": "1.47.0"`, `resolved` dal registro npm, `integrity` e `"license": "ISC"`; nessun `hasInstallScript`.
- `gui/node_modules/lucide/LICENSE`: `ISC License` in testa, poi *«The following Lucide icons are derived from the Feather
  project»* (riga 19) e *«The MIT License (MIT) (for the icons listed above)»* (riga 23).
- `gui/node_modules/lucide/package.json`: gli `scripts` sono solo di *build* e di prova — nessuno d'installazione —, e ha
  solo `devDependencies`: è il pacchetto dei **soli disegni**, non quello per Vue.

### Passo 2 — l'aiutante di `axe`, seconda occorrenza

- `gui/src/testing/axe.ts` creato (LF, 15 righe).
- Le tre sostituzioni in `gui/src/a11y.test.ts` con `replace_unique.py` → `ok: gui/src/a11y.test.ts (LF)` tre volte. La terza
  presa **dal file**, com'è il §5 del mandato: `grep -n 'Every violation axe finds\|Fills the stores' gui/src/a11y.test.ts` →
  `26:` e `38:`; il testo vecchio le righe **25–38** (dal `/**` alla riga di `welcome`), il nuovo la sola riga 38. Il file ne
  esce con `import { violations } from "./testing/axe";` sopra `createFakeBridge`, una riga vuota e il commento di `welcome`.

```
$ (cd gui && npx vitest run src/a11y.test.ts)
 Test Files  1 passed (1)
      Tests  11 passed (11)
```

### Passo 3 — le prove, prima dei pezzi

`gui/src/components/kit.test.ts` creato (LF, 232 righe).

```
$ (cd gui && npx vitest run src/components/kit.test.ts)
Error: Failed to resolve import "./BaseDialog.vue" from "src/components/kit.test.ts". Does the file exist?
  Plugin: vite:import-analysis
 Test Files  1 failed (1)
      Tests  no tests
exit=1
```

**Rosso**, perché i pezzi non esistono. ⚠️ Il file nominato **cambia da una corsa all'altra**: rilanciato tre volte,
`./BaseButton.vue`, `./BaseList.vue`, `./BaseIcon.vue` — il §8.

### Passi 4, 5 e 6 — la mappa, i pezzi

Creati, LF, nell'ordine del compito: `icons.ts` (73 righe) e `BaseIcon.vue` (55); `BaseButton.vue` (147), `BaseLabel.vue` (33),
`BaseList.vue` (39), `BaseStatus.vue` (12); `BaseTextField.vue` (97), `BaseRadioGroup.vue` (105), `BaseDialog.vue` (109). Ogni
token che gli otto pezzi usano è definito nei fogli del compito 1: sotto `gui/src`, `grep -oh 'var(--[a-z0-9-]*'
components/Base*.vue`, e per ciascun nome un `grep -c -- "--<nome>:"` su `tokens/base.css` e `tokens/themes.css` → `names: 53
missing: 0`. I diciotto `module` di `panels/registry.ts` hanno ciascuno la sua voce nella mappa, letti prima e provati dalla
prova di D6.

```
$ (cd gui && npx vitest run src/components && npm run build)
 Test Files  2 passed (2)
      Tests  28 passed (28)
> vue-tsc --noEmit && vite build
✓ 746 modules transformed.
dist/assets/index-D7FEpuBn.css     145.10 kB │ gzip:  14.04 kB
dist/assets/index-DLsd9Y_U.js      663.93 kB │ gzip: 201.53 kB
✓ built in 1.61s
(!) Some chunks are larger than 500 kB after minification.   (N-2 di E187, c'era già)
exit=0
```

I due file sono `kit.test.ts` (23 prove) e `markdown.test.ts` (5). Nessuna riga `stderr` nella corsa delle prove. Il pezzo
JavaScript **non cambia**: stesso nome e stessa misura del cancello d'apertura. Non l'ho scritto nel messaggio del commit: la
voce N-2 di E187 del piano dice che il compito 3 non lo scrive (R2-13).

**`vue-tsc` legge la riga `@ts-expect-error`** — misurato, oltre il testo del compito, **dopo** il cancello finale e prima del
commit: `gui/tsconfig.json` ha `"include": ["src/**/*.ts", "src/**/*.vue", "vite.config.ts"]` senza `exclude`; con `"search"`
al posto di `"not-an-icon"` in `kit.test.ts`, `npx vue-tsc --noEmit` → `src/components/kit.test.ts(50,5): error TS2578: Unused
'@ts-expect-error' directive.`, uscita 2. Il file è tornato dalla copia salvata, e `cmp` lo conferma.

### Passo 7 — il linter: i `.ts`, e le regole del kit

Le cinque sostituzioni in `gui/eslint.config.js` con `replace_unique.py` → `ok: gui/eslint.config.js (LF)` cinque volte; il
file ne esce di 143 righe, e l'ho riletto per intero dopo.

```
$ (cd gui && npm run lint)
> harness-gui@0.0.0 lint
> eslint src
exit=0
```

**Verde**, e niente altro stampato. Dopo: `npx eslint src --format json` → `files: 73 {'.json': 4, '.ts': 48, '.vue': 21} errors:
0 warnings: 0`; `find src -name '*.ts' | wc -l` → `48`. Prima: 17 file, **nessun** `.ts`. Nessuna regola già attiva scatta su
un `.ts` ora letto.

### Passo 8 — le due direzioni

Prima della prima violazione: `git status --porcelain > <scratchpad>/prima.txt`, e una copia (`cp -p`) dei nove file del §5 del
mandato, ciascuna confermata con `cmp`. Ogni violazione messa con uno script nello scratchpad (`mutate.py`) che conserva il
fine-riga del file — `Confirm.vue` è CRLF qui, lo script l'ha scritta CRLF —, poi il comando, poi la copia indietro e `cmp`.
Le due tabelle sono al §5.

### Passo 9 — tutte le prove, il cancello, il commit

```
$ (cd gui && npm test && npm run build && npm run lint)
 Test Files  19 passed | 1 skipped (20)
      Tests  128 passed | 1 skipped (129)
dist/assets/index-DLsd9Y_U.js      663.93 kB │ gzip: 201.53 kB
> eslint src
exit=0
```

(`npm test` è una corsa sola coi due progetti: 18 + 1 file, 123 + 5 prove.)

**Le due celle del piano**, con `replace_unique.py` sulla riga intera — il piano qui è CRLF — → `ok: … (CRLF)` due volte;
`git diff --stat` → `4 ++--`:

```
-| **2** | … il tema che segue il sistema | — | ✅ 2026-09-25 |
+| **2** | … il tema che segue il sistema | `23b3134`, con la cura `5f32158` | ✅ 2026-09-25 |
-| **3** | … e le due regole sui pezzi di base | — | ⬜ |
+| **3** | … e le due regole sui pezzi di base | — | ✅ 2026-09-25 |
```

Poi, uno alla volta: `bash scripts/gate.sh` → `GATE GREEN.` (il §7); `bash scripts/check-docs.sh` → `OK — no
inconsistencies.`; `git diff --stat HEAD -- crates/ gui/schema/ .github/ scripts/ docs/adr/ Cargo.toml Cargo.lock` → vuoto;
nessun `*.tmp` sotto `gui/src`; il commit con `git commit -F <scratchpad>/commit-msg.txt`.

## 5. Le due tabelle del Passo 8

**Il linter** — `npm run lint` dopo ciascuna:

| La violazione messa a mano | Il rosso vero | `cmp` dopo |
|---|---|---|
| in `BaseLabel.vue`, sotto l'import di `IconName`, `import { useCore } from "../stores/core";` | `4:1  error  '../stores/core' import is restricted from being used by a pattern. a base piece reads no global state and knows no layer above it (design system, section (b))  no-restricted-imports` — `✖ 1 problem (1 error, 0 warnings)` | uguale |
| in `icons.ts`, sotto `} from "lucide";`, `import { useCore } from "../stores/core";` (**E18**) | `28:1  error  '../stores/core' import is restricted from being used by a pattern. a base piece reads no global state and knows no layer above it (design system, section (b))  no-restricted-imports` — `✖ 1 problem`: l'import di `lucide` dello stesso file resta muto | uguale |
| in `frame/moveActive.ts`, riga 2, `import { Search } from "lucide";` | `2:1  error  'lucide' import is restricted from being used. icons pass through BaseIcon and the one map, src/components/icons.ts (answer 11 of the design system)  no-restricted-imports` — `✖ 1 problem`: i `.ts` ora si leggono | uguale |
| in `BaseButton.vue`, sotto l'import di `IconName`, `import { Search } from "lucide";` | `6:1  error  'lucide' import is restricted from being used. icons pass through BaseIcon and the one map, src/components/icons.ts (answer 11 of the design system)  no-restricted-imports` — `✖ 1 problem` | uguale |
| in `Confirm.vue`, sotto l'import di `useInvoke`, `import { PANEL_TYPES } from "../panels/registry";` | `7:1  error  '../panels/registry' import is restricted from being used by a pattern. the kit knows no layer above it: panels, frame and transport use the kit, not the other way (design system, section (b))  no-restricted-imports` — `✖ 1 problem`: i due negozi di `Confirm.vue` restano muti | uguale |
| in `markdown.ts`, riga 2, `import type { Bridge } from "../transport/bridge";` | `2:1  error  '../transport/bridge' import is restricted from being used by a pattern. the kit knows no layer above it: panels, frame and transport use the kit, not the other way (design system, section (b))  no-restricted-imports` — `✖ 1 problem`: la regola arriva a un `.ts` di `components/` e a un `import type` | uguale |
| niente — `icons.ts` importa `lucide` (riga 27), `Confirm.vue` legge due negozi (righe 5 e 6), `kit.test.ts` legge `PANEL_TYPES` (riga 5) | **verde**: `npm run lint` esce 0 | — |

**Le prove del kit** — `npx vitest run --project jsdom src/components/kit.test.ts` dopo ciascuna; prima della prima, sul codice
del compito: `Test Files  1 passed (1)`, `Tests  23 passed (23)`:

| La violazione messa a mano | Il rosso vero | `cmp` dopo |
|---|---|---|
| in `icons.ts` tolta la riga `models: Cpu,` | `AssertionError: expected [ 'models' ] to deeply equal []`, alla prova *«has an icon for every module type, by the same name (D6 of the plan)»* — `Tests  1 failed \| 22 passed (23)` | uguale |
| in `BaseStatus.vue` la regione dentro `v-if="(slots.default?.() ?? []).some((node) => node.type !== Comment)"`, con `Comment` e `useSlots` importati da `vue` e `const slots = useSlots();` | `Error: Unable to get [role="status"] within: <!--v-if-->`, alla prova *«is in the DOM while empty, and the words enter the same region»* — `Tests  1 failed \| 22 passed (23)` | uguale |
| in `BaseTextField.vue` tolta la riga `defineOptions({ inheritAttrs: false });` (**E15**) | `AssertionError: expected true to be false // Object.is equality`, a `kit.test.ts:142:57`, la riga `expect(wrapper.element.hasAttribute("placeholder")).toBe(false);` della prova *«binds its text, puts the caller's attributes on the input, and says an error under it»* — `Tests  1 failed \| 22 passed (23)` | uguale |
| in `BaseDialog.vue` tolta la riga `v-bind="description === undefined ? { 'aria-describedby': undefined } : {}"` (**E16**) | `AssertionError: expected true to be false // Object.is equality`, a `kit.test.ts:196:54`, la riga `expect(dialog?.hasAttribute("aria-describedby")).toBe(false);` della prova *«opens bound, names itself with its title, and asks to close on Esc»*; e l'avviso di `reka-ui` *«Warning: Missing `Description` or `aria-describedby="undefined"` for DialogContent.»* — `Tests  1 failed \| 22 passed (23)` | uguale |

E l'avvertenza della riga di `BaseStatus`, misurata anche lei: con `v-if="$slots.default"` → `Tests  23 passed (23)`, **verde**,
com'è scritto; `cmp` uguale dopo.

**Alla fine**, dopo l'ultima copia tornata: `cmp` uguale su tutti e nove i file, e `git status --porcelain | diff
<scratchpad>/prima.txt -` → **nulla**. Nessun file nato dalle prove. (Dopo il Passo 9 lo stesso `diff` nomina la sola riga
` M docs/superpowers/plans/2026-09-23-design-system.md`: sono le due celle, scritte dopo il Passo 8.)

## 6. I fine-riga

Misurati con `tr -cd '\r' < <file> | wc -c` contro `wc -l < <file>`, e `git ls-files --eol`. I file nuovi non c'erano prima.

| File | Prima: CR / righe, `git ls-files --eol` | Dopo: CR / righe, `git ls-files --eol` |
|---|---|---|
| `gui/package.json` | 43 / 43, `i/lf w/crlf` | 44 / 44, `i/lf w/crlf` |
| `gui/package-lock.json` | 5076 / 5076, `i/lf w/crlf` | 5083 / 5083, `i/lf w/crlf` |
| `gui/eslint.config.js` | 0 / 83, `i/lf w/lf` | 0 / 143, `i/lf w/lf` |
| `gui/src/a11y.test.ts` | 0 / 127, `i/lf w/lf` | 0 / 114, `i/lf w/lf` |
| `docs/superpowers/plans/2026-09-23-design-system.md` | 8656 / 8656, `i/lf w/crlf` | 8656 / 8656, `i/lf w/crlf` |
| `gui/src/testing/axe.ts` | — | 0 / 15, `i/lf w/lf` |
| `gui/src/components/icons.ts` | — | 0 / 73, `i/lf w/lf` |
| `gui/src/components/kit.test.ts` | — | 0 / 232, `i/lf w/lf` |
| `gui/src/components/BaseButton.vue` | — | 0 / 147, `i/lf w/lf` |
| `gui/src/components/BaseDialog.vue` | — | 0 / 109, `i/lf w/lf` |
| `gui/src/components/BaseIcon.vue` | — | 0 / 55, `i/lf w/lf` |
| `gui/src/components/BaseLabel.vue` | — | 0 / 33, `i/lf w/lf` |
| `gui/src/components/BaseList.vue` | — | 0 / 39, `i/lf w/lf` |
| `gui/src/components/BaseRadioGroup.vue` | — | 0 / 105, `i/lf w/lf` |
| `gui/src/components/BaseStatus.vue` | — | 0 / 12, `i/lf w/lf` |
| `gui/src/components/BaseTextField.vue` | — | 0 / 97, `i/lf w/lf` |

Su ogni file CRLF i CR sono uguali alle righe, sui LF sono zero, la colonna `w/…` è quella di prima, i nuovi nascono LF; nel
commit sono tutti `i/lf`. I tre file che le violazioni del Passo 8 hanno toccato e che il compito non cambia — `Confirm.vue`
(`i/lf w/crlf`, 74 / 74), `markdown.ts` e `frame/moveActive.ts` (`i/lf w/lf`) — sono tornati byte per byte (`cmp`), e non sono
nel commit.

## 7. Il passo web — il cancello finale

`bash scripts/gate.sh`, da solo, in background, sull'albero col compito e le due celle, 13:08:42 → 13:15:06:

```
added 341 packages in 1m                                   (npm ci: uno in più, lucide)
dist/assets/index-DLsd9Y_U.js    663.93 kB │ gzip: 201.53 kB
> vitest run --project jsdom
 Test Files  18 passed | 1 skipped (19)
      Tests  123 passed | 1 skipped (124)
> vitest run --project browser
 Test Files  1 passed (1)
      Tests  5 passed (5)
-------- gui: lint
> eslint src
-------- gui: advisories
found 0 vulnerabilities
GATE GREEN.
```

`bash scripts/check-docs.sh` → `OK — no inconsistencies.`

## 8. Le divergenze, e ciò che non ho misurato

**Voci candidate d'errata: nessuna.** Ogni *Atteso* del compito è tornato, e ogni riga delle due tabelle del Passo 8 è rossa per
la ragione scritta, un problema — o una prova — per corsa.

**Una differenza da un riferimento del pre-controllo** — non dal piano:

| Passo | Il riferimento del mandato | Misurato | Perché non è una voce |
|---|---|---|---|
| 3 | `Failed to resolve import "./BaseIcon.vue" from "src/components/kit.test.ts"` | la prima corsa `./BaseDialog.vue`; tre corse ancora, `./BaseButton.vue`, `./BaseList.vue`, `./BaseIcon.vue` | il piano dice solo *«rosso — i pezzi non esistono ancora»*, ed è vero: quale dei pezzi mancanti `vite:import-analysis` nomini per primo **cambia da una corsa all'altra** (dedotto: gli import si risolvono in parallelo, e vince il primo rifiuto). Un riferimento futuro direbbe *«un `Failed to resolve import` di un `./Base*.vue`»* |

Gli altri riferimenti del §3 del mandato tornano uguali: `added 1 package`, `lucide@1.47.0`, `ISC`, `found 0 vulnerabilities`;
11 prove al Passo 2; 2 file e 28 prove, e `index-DLsd9Y_U.js` a `663.93 kB`, al Passo 6; 48 `.ts` letti al Passo 7; i messaggi
del Passo 8; 18 file e 123 prove col `jsdom`, 1 file e 5 prove col `browser`, al Passo 9.

**Oltre il testo del compito**, due misure in più, entrambe tornate indietro con la copia salvata e `cmp`: `vue-tsc` legge la
riga `@ts-expect-error` (il §4, Passo 6); un `v-if="$slots.default"` qualunque in `BaseStatus.vue` resta verde (il §5).

**Non misurato da qui:** la CI, che legge il coordinatore dopo il push; l'aspetto dei pezzi nel browser, nei due temi — la
pagina kit è del compito 4 —; l'Assistente vocale su `BaseStatus` — del compito 5 (controllo 13). La macchina `Jays` non l'ho
vista: i fine-riga del §6 sono di `zagor`.

## 9. Il tempo, e i log

Dalle 12:48 circa (l'avvio) alle 13:18:10 (il commit): una mezz'ora, di cui 5 min 25 s il cancello d'apertura e 6 min 24 s
quello finale.

Nello scratchpad `C:\Users\zagor\AppData\Local\Temp\claude\C--Users-zagor-Desktop-harness\cba88c07-c154-4a08-acdf-71c69edc56c4\scratchpad\task3\`,
lasciato com'è perché questo rapporto lo nomina — la pulizia è del coordinatore:

| File | Che cosa |
|---|---|
| `gate-opening-2026-09-25.log` | il cancello d'apertura, a `ef3e865` |
| `gate-final-2026-09-25.log` | il cancello finale, prima del commit |
| `check-docs-2026-09-25.log` | `check-docs.sh` prima del commit |
| `step1-…`, `step2-…`, `step3-red-…`, `step6-…`, `step7-lint-…`, `step9-local-2026-09-25.log` | le uscite dei passi |
| `v1-lint.log` … `v6-lint.log`, `v7-lint-green.log`; `k1.log` … `k4.log`, `k2b.log`; `tsc-idle.log` | le due direzioni, e le due misure in più |
| `eslint-before.json`, `eslint-after.json` | i file letti dal linter, prima e dopo |
| `prima.txt`, `copie/` | lo stato di git e le copie salvate del Passo 8 |
| `replace_unique.py`, `blocks.py`, `mutate.py`, `b/`, `commit-msg.txt` | gli attrezzi, i recinti estratti dal brief, il messaggio del commit |
