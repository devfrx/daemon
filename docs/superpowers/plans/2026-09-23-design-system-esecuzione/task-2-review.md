# La revisione del compito 2 del design system — il browser dei test

> Revisore: subagente fresco, macchina `zagor` (`C:\Users\zagor\Desktop\harness`), il 2026-09-25, dalle 08:42 alle 09:40
> circa. Commit rivisto: `23b3134`, sopra `f00d5b7`. Mandato: `review-2-prompt.md`; contratto dell'implementatore:
> `dispatch-task-2.md`; testo del compito: `task-2-brief.md`; rapporto: `task-2-report.md`.
> Nell'albero del repository **nessuna scrittura** oltre a questo file, che sta nella cartella ignorata: `git status
> --porcelain` vuoto prima e dopo, `HEAD` sempre `23b3134`, `## main...origin/main [ahead 1]`. Le mutazioni nel clone
> `C:\Users\zagor\AppData\Local\Temp\rv2`, lasciato a `23b3134` e pulito. I log e gli attrezzi nello scratchpad
> `C:\Users\zagor\AppData\Local\Temp\claude\C--Users-zagor-Desktop-harness\adc89962-1e86-4371-976e-69fc30a546ee\scratchpad\review2\`.

## 1. Il verdetto

**CONFORME.** Il commit è il testo del piano parola per parola — `compare_task2.py` esce 0, ed è stato provato nelle due
direzioni —, ogni *Atteso* del compito torna rimisurato su questa macchina, i vincoli e il contratto sono rispettati, e
ogni affermazione misurabile del rapporto torna col comando rilanciato.

| Classe | Quanti | Quali |
|---|---|---|
| Critico | 0 | |
| Importante | 0 | |
| Minore | 2 | **M-1**, lo script di confronto del pre-controllo; **M-2**, voce d'errata candidata **E14** sul testo dettato |
| Nit | 0 | |

Nessuno dei due tocca la conformità del commit: M-1 sta nell'attrezzo del coordinatore, M-2 è un difetto del **dettato**,
che l'implementatore ha copiato com'era scritto. La finestra delle prove senza sonda (§3.8) **non** è un rilievo: il
pre-controllo l'ha già registrata e non presa, e la mia misura non porta una prova nuova che basti a riaprirla.

## 2. I rilievi

### M-1 — Minore — `compare_task2.py`: il piano non cambia mai il codice d'uscita, e la data la prende dal commit stesso

**Dove.** `docs/superpowers/plans/2026-09-23-design-system-esecuzione/compare_task2.py`, ritrovate col `grep` sulle frasi:

| Riga | Il testo |
|---|---|
| 16 | la docstring: `Exit 0 when everything matches, 1 otherwise.` |
| 154 | il ramo del piano: `print(f"CHECK BY HAND  {path}  -- more than the two cells of step 7:")`, **senza** `bad += 1` |
| 66 | `DATE = row2.group(2)`: il giorno si legge nella riga 2 del commit rivisto, e nessuno lo confronta con altro |

**Il difetto.** Le due celle della riga 1 e della riga 2 sono dettate (Passo 7 e punto 2 del contratto), e lo script le
ricostruisce; ma se il commit le sbaglia stampa `CHECK BY HAND … more than the two cells of step 7` — che per giunta dice
«più di», quando è «diverso» — e **esce 0**. La docstring dice il falso per il piano. E la data: se la riga 2 e il
richiamo del disegno portano lo **stesso** giorno sbagliato, tutto è `OK`. Il pre-controllo l'ha provato con quattro
mutanti — un valore di `vite.config.ts`, il file usa-e-getta lasciato, un file in più, `playwright` non esatto (riga 8561
del piano) —, nessuno sul piano.

**L'evidenza**, nel clone, dalla sua radice, con `PYTHONIOENCODING=utf-8`:

| Commit nel clone | Che cosa cambia | `compare_task2.py f00d5b7 HEAD` |
|---|---|---|
| `32f0bd2` | una lettera di un commento di `gui/src/tokens/tokens.browser.test.ts`, `CHROME` → `CHROMe` | `DIFFERS        gui/src/tokens/tokens.browser.test.ts` col diff della riga 7, ogni altro percorso `OK`, `--- 9 paths, 1 not matching the plan's text; date 2026-09-25`, **exit=1** — la direzione che il mandato chiede, e regge |
| `30945f5` | nella riga 1 della posizione `69d10fa` → `69d10fb` | `CHECK BY HAND  docs/superpowers/plans/2026-09-23-design-system.md  -- more than the two cells of step 7:` col diff, poi `--- 9 paths, 0 not matching the plan's text; date 2026-09-25`, **exit=0** |
| `9d75f53` (del giorno `2026-09-25`) | la riga 2 a `✅ 2026-09-21` e il richiamo a `RICHIAMO DEL 2026-09-21` | tutto `OK`, `--- 9 paths, 0 not matching the plan's text; date 2026-09-21`, **exit=0** |

Su `23b3134` il verdetto non cambia: le due celle sono giuste — lo script dice `OK … (the two cells of step 7)`, e letto a
mano il diff del piano è di due righe, la 1 con `` `95068bb`, con le cure `69d10fa` ed `e2cd7df` `` e la 2 con
`✅ 2026-09-25`, la colonna Commit della 2 a `—`.

**La cura proposta, a parole.** Nel ramo del piano, prima le due celle: `ROW1_NEW` e `ROW2_NEW` una volta ciascuna nel
piano del commit, altrimenti `DIFFERS` e `bad += 1`; `CHECK BY HAND`, senza contare, solo quando le due celle sono giuste
e cambia **altro** — una voce d'errata, com'è scritto nella docstring. Accanto a `date <DATE>`, il giorno del commit
(`git log -1 --format=%cs <target>`), e se differiscono una riga `CHECK BY HAND` che lo dice — non un `DIFFERS`: il
dispaccio fissa il giorno anche se l'esecuzione passa la mezzanotte. Se invece il piano deve restare fuori dal codice
d'uscita per scelta, la docstring lo dice. Vale per il modello di `compare_task3.py`; `compare_task1.py` lasciava il piano
sempre al lettore, e la sua docstring ha la stessa frase.

### M-2 — Minore — voce d'errata candidata **E14** — la seconda direzione della sonda del movimento è verde su un valore vuoto

**Dove.** `gui/src/tokens/tokens.browser.test.ts`, righe 76–78 (`grep -n 'not.toBe("0ms")'` → 78; il commento alla 76);
nel piano, il recinto del Passo 2, righe 1403–1405. È testo **dettato**, copiato com'era: non si cura in silenzio.

```ts
    // ⛔ THE SECOND DIRECTION: without the request, the durations are the board's and not zero.
    await commands.emulateMedia({ reducedMotion: "no-preference" });
    expect(rootStyle().getPropertyValue("--duration-fast").trim()).not.toBe("0ms");
```

**Il difetto.** Il commento dice *«the durations are the board's»*; l'asserzione guarda **un** nome solo e chiede che non
sia la stringa `0ms`: una stringa vuota passa. È la forma della trappola 1 — una negazione è verde quando non trova niente.
La prima direzione della prova non è vacua, e una durata che manchi del tutto la fa rossa; ma una durata che manchi
**solo** nel blocco di base, e resti nel blocco di `prefers-reduced-motion`, la sonda del browser non la vede.

**L'evidenza**, nel clone, con la copia salvata (vincolo 11): tolta da `base.css` la riga `  --duration-fast: 110ms;` del
`:root` di base, `npm test -- --reporter=verbose` → exit 1, ma per la ragione sbagliata:
`✓ |browser (chromium)| … put the motion to zero when the system asks for less, and only then`, e rossa soltanto
`× |jsdom| src/tokens/board.test.ts > the token files > base.css is the board's block, byte for byte`,
`Tests  1 failed | 104 passed | 1 skipped (106)`. Oggi il cancello la prende da `board.test.ts`, non dalla sonda.

**Il testo corretto proposto**, misurato nel clone: verde sul `base.css` del commit (`npm test -- --project browser` →
`Tests  5 passed (5)`), rosso con la stessa mutazione, `AssertionError: --duration-fast: expected '' to match
/^[1-9]\d*ms$/`. Nessun valore della tavola vi è ricopiato (vincolo 2):

```ts
    // ⛔ THE SECOND DIRECTION: without the request, every duration is a duration and not zero -- a VALUE, so a
    // name the base block lost is red here too, and not only in `board.test.ts` (E14 of the design-system plan).
    await commands.emulateMedia({ reducedMotion: "no-preference" });
    for (const name of ["--duration-fast", "--duration-moderate", "--duration-slow"]) {
      expect(rootStyle().getPropertyValue(name).trim(), name).toMatch(/^[1-9]\d*ms$/);
    }
```

La voce nell'errata e il recinto del Passo 2 allineato; la cura nel commit che la scrive, come le cure del compito 1
(`69d10fa`). Trovata dalla revisione del compito 2.

## 3. I comandi rilanciati, con le uscite — punto per punto del mandato

### 3.0 L'avvio

| Comando | Uscita |
|---|---|
| `git rev-parse --short HEAD` | `23b3134` |
| `git status --porcelain` | vuoto |
| `git log --oneline f00d5b7..HEAD` | una riga, `23b3134 design-system(compito 2): il browser dei test -- …` |
| `git log -1 --format=%P 23b3134` | `f00d5b7907298be0acf3f4463bd4b7a61c3533d6` |

### 3.1 Le affermazioni misurabili del rapporto

| Affermazione del rapporto | Comando rilanciato | La mia uscita | |
|---|---|---|---|
| Node `v24.19.0`, npm `11.17.0`, dentro `engines` | `node --version`; `npm --version`; `engines` letto da `gui/package.json` | `v24.19.0`; `11.17.0`; `^22.22.2 \|\| ^24.15.0 \|\| >=26.0.0` | torna |
| `core.autocrlf` | `git config --show-origin --get-all core.autocrlf` | `file:C:/Program Files/Git/etc/gitconfig	true` | torna |
| le cartelle di Chrome | `ls "/c/Program Files/Google/Chrome/Application/"` | `153.0.8010.53`, `154.0.8037.58`, `chrome.exe`, `new_chrome.exe`, … | torna |
| Playwright apre la 153 | `(Get-Item …\chrome.exe).VersionInfo.ProductVersion`; `chromium.launch({ channel: "chrome", headless: true })` e `browser.version()` col `playwright` del clone | `153.0.8010.53`; `153.0.8010.53`, e `navigator.userAgent` `… HeadlessChrome/153.0.0.0 Safari/537.36` | torna |
| il cancello d'apertura, `GATE GREEN`, `real 5m27.503s` | `grep` su `task2/gate-2026-09-25-baseline.log` | `GATE GREEN.`, `real	5m27.503s` | torna |
| la base, 17 file e 100 prove | nel clone a `f00d5b7`, `npm ci`, poi `npx vitest run --reporter=json` | `Test Files  17 passed \| 1 skipped (18)`, `Tests  100 passed \| 1 skipped (101)`; la saltata è `generate-views.test.ts > regenerates the three committed views` | torna |
| i log del cancello sono dell'implementatore | `ls -la --time-style=full-iso` sui log; `git log -1 --format=%ci 23b3134` | finale `2026-09-25 08:35:48` (partito alle `08:30:31`, file `.start`), d'apertura `08:19:08` (partito alle `08:13:40`), il commit `2026-09-25 08:37:28 +0200`; quello del coordinatore, `gate-open.log`, è delle `08:10:53` ed è un altro file | torna |
| il passo web del cancello finale | `grep` su `task2/gate-2026-09-25-final.log` | `> vitest run --project jsdom`, `17 passed \| 1 skipped (18)`, `100 passed \| 1 skipped (101)`; `> vitest run --project browser`, `1 passed (1)`, `5 passed (5)`; `found 0 vulnerabilities`; `index-DLsd9Y_U.js  663.93 kB`; `GATE GREEN.` | torna |
| `git show --stat HEAD` | `git show --numstat --format= 23b3134` | piano 2/2, disegno 1/1, lockfile 159/0, `package.json` 2/0, `browser.d.ts` 13/0, la prova 118/0, `vite.config.ts` 69/8, `gate-gui.sh` 10/1 | torna, compresi i 69 e 8 del Passo 3 |
| Passo 1, `npm ls` | nel clone a `23b3134`, dopo `npm ci` (`added 340 packages`) | lo stesso albero: `@vitest/browser-playwright@4.1.11` con `@vitest/browser@4.1.11`, `playwright@1.63.0` con `playwright-core@1.63.0`, `vitest@4.1.11` | torna |
| Passo 1, nessuno script d'installazione | `npm view playwright@1.63.0 scripts.install scripts.postinstall`; `scripts` dei `package.json` installati | non stampa nulla, exit 0; `playwright` e `playwright-core` `{}`, i due `@vitest/…` solo `build`/`dev` | torna |
| undici pacchetti, MIT e Apache-2.0 | `lockdiff.py f00d5b7 23b3134`, nello scratchpad | `added 11, removed 0, changed 1`: gli undici del rapporto, `dev=True`, `hasInstallScript=False`, `registry.npmjs.org`, `sha512`; licenze MIT, e Apache-2.0 per `playwright` e `playwright-core`; la voce cambiata è la radice `""`, le sole due `devDependencies` nuove (lo script dell'implementatore la esclude con `if k`, e la dice a parte) | torna |
| Passo 2, il rosso | §3.3 | | torna |
| Passo 4 | nel clone, `(npx vitest run --project browser && npm test && npm run build)` | exit 0: `1 passed (1)`/`5 passed (5)`; `18 passed \| 1 skipped (19)`/`105 passed \| 1 skipped (106)`; `vue-tsc --noEmit && vite build` verde, `dist/assets/index-DLsd9Y_U.js  663.93 kB │ gzip: 201.53 kB`, stesso nome della base | torna |
| i due progetti, dallo script del piano | il recinto 1603 estratto con `blocks.py`, le sue due righe in `projects-2.sh` nello scratchpad, lanciato dalla radice del clone | `      5 \|browser (chromium)\|` e `    101 \|jsdom\|`, exit 0 | torna |
| Passo 5, le dieci violazioni | §3.4 | | torna |
| Passo 6 | `grep -c 'dal compito 2 del \[piano del design system\]' docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` | `1` | torna |
| Passo 7, i tre hash esistono | `git log --oneline -1` di `95068bb`, `69d10fa`, `e2cd7df` | i tre commit del compito 1 | torna |
| i fine-riga | §3.9 | | torna |
| senza finestra | §3.7 | | torna |
| niente co-autore | `git log -1 --format=%B 23b3134 \| grep -ci co-authored` | `0` | torna |

Nessun numero del rapporto è tornato diverso. I tempi della riga del tema e dell'attesa usa-e-getta cambiano da corsa a
corsa, come il rapporto dice (N3): miei `5087 ms`, `2059 ms` e `1051 ms`, contro `5388`, `2050` e `1071`.

### 3.2 La conformità al dettato, `compare_task2.py`

- **Letto.** Ricostruisce dal testo del piano a `f00d5b7`: i due file interi (`W`), le cinque sostituzioni in catena —
  tre su `vite.config.ts`, una su `gate-gui.sh`, una sul disegno, col `<data>` —, il file usa-e-getta che non deve restare
  (`S`), `package.json` come JSON con le due voci esatte e le chiavi in ordine, le versioni appuntate nel lockfile, il
  piano con le due celle; ogni altro file cambiato è `UNEXPECTED`. Non guarda i fine-riga (per costruzione) né le voci del
  lockfile oltre le appuntate (lo dice: `CHECK BY HAND`). Il suo difetto è M-1.
- **La ricetta** — `recipe_check.py`, nello scratchpad, sul piano a `f00d5b7`: le otto righe alle righe 8571–8578, e ogni
  numero apre il recinto giusto — 1327 la prova (118 righe, sotto *«Crea `gui/src/tokens/tokens.browser.test.ts`»*), 1450
  `browser.d.ts` (13), 1479/1486, 1495/1501 e 1523/1537 le tre coppie *Trova*/*Sostituisci* del Passo 3, 1630 il file
  usa-e-getta, 1651/1658 `gate-gui.sh`, 1676/1682 il disegno —; ciascun recinto sta una volta nel brief (`export default
  defineConfig({` due, perché ricompare nella propria sostituzione). I recinti del compito fuori dalla ricetta sono comandi:
  1312, 1466, 1594, 1603.
- **Sul commit**, dalla radice del repository: `PYTHONIOENCODING=utf-8 python …/compare_task2.py f00d5b7 23b3134` → nove
  `OK` — piano (le due celle), disegno, lockfile, `package.json`, `browser.d.ts`, `late.browser.test.ts` non lasciato, la
  prova, `vite.config.ts`, `gate-gui.sh` —, `--- 9 paths, 0 not matching the plan's text; date 2026-09-25`, exit 0.
- **Nelle due direzioni:** la tabella di M-1.
- **A mano, ciò che lascia `CHECK BY HAND`:** il piano — il diff sono le sole righe 1 e 2 della posizione, com'è detto
  sopra —; il lockfile — undici voci nuove, nessuna tolta, nessuna delle voci di prima cambiata, la radice con le due
  `devDependencies`, nello **stesso** commit di `package.json` (vincolo 7).

### 3.3 Il rosso del Passo 2, riprodotto

Nel clone a `23b3134`, copia salvata di `gui/vite.config.ts`, poi il file com'era a `f00d5b7` (`git show
f00d5b7:gui/vite.config.ts`, coi CRLF del clone; `git diff --stat` → `8 insertions(+), 69 deletions(-)`), dipendenze e
prova lasciate: `npx vitest run src/tokens/tokens.browser.test.ts` → exit 1,

```
 FAIL  src/tokens/tokens.browser.test.ts [ src/tokens/tokens.browser.test.ts ]
Error: vitest/browser can be imported only inside the Browser Mode. Your test is running in forks pool. Make sure your regular tests are excluded from the "test.include" glob pattern.
 ❯ src/tokens/tokens.browser.test.ts:1:1
 Test Files  1 failed (1)
      Tests  no tests
```

— l'*Atteso* del Passo 2 e il rapporto. Il file tornato dalla copia: `cmp` uguale, `git status --porcelain` vuoto.

### 3.4 Le dieci violazioni del Passo 5, nel clone

`viol.py save` prima della prima (`prima.txt` vuoto, cinque copie: `vite.config.ts`, `tokens/index.ts`, `base.css`,
`theme.ts`, `tokens.browser.test.ts`); ogni violazione messa con una sostituzione unica che conserva i CRLF del clone, poi
`viol.py restore` e `viol.py check` — `cmp` uguale sui cinque file e `git status now: b'' | same as prima.txt: True` dopo
**ciascuna**; e dopo ciascuna `find gui/src -name '__screenshots__' -o -name '*.png'` non trova nulla. Il comando è
`npm test` dentro `gui/`, dove il compito non ne detta un altro.

| # | La violazione | Comando → exit | Il messaggio vero | Contro l'*Atteso* e il rapporto |
|---|---|---|---|---|
| 1 | `channel: "chrome-that-does-not-exist"` | `npm test` → 1; e nella forma del cancello, `npm test -- --project browser` → 1 | `Error: browserType.launch: Unsupported chromium channel "chrome-that-does-not-exist"`; `Test Files  17 passed \| 1 skipped (19)`, `Errors  1 error`; nella forma del cancello `Tests  no tests`, `Errors  1 error` | tornano |
| 2 | il progetto `browser` con `enabled: false` | `npm test` → 1 | `FAIL  \|browser\| src/tokens/tokens.browser.test.ts`, `Error: vitest/browser can be imported only inside the Browser Mode. Your test is running in forks pool. …`; `Test Files  1 failed \| 17 passed \| 1 skipped (19)` | tornano |
| 3a | l'`include` del `browser` a `["src/**/*.nothing.test.ts"]` | `--project browser` → **1**; `--project jsdom` → 0; `npm test` → **0** | `No test files found, exiting with code 1`, `include: src/**/*.nothing.test.ts`; il jsdom `17 passed \| 1 skipped (18)`, `100 passed \| 1 skipped (101)`; `npm test` lo stesso: Chrome non parte, e la corsa è verde | tornano: è **E10** |
| 3b | l'`include` del `jsdom` allo stesso modo | `--project jsdom` → **1**; `--project browser` → 0; `npm test` → **0** | `No test files found, exiting with code 1`; il browser `1 passed (1)`, `5 passed (5)`; `npm test` lo stesso: le cento prove sotto jsdom spariscono, e la corsa è verde | tornano: è **E10** |
| 4 | tolta la riga di Barlow 300 da `tokens/index.ts` | `npm test` → 1 | `AssertionError: expected 3 to be greater than or equal to 4`; `Tests  1 failed \| 104 passed \| 1 skipped (106)` | tornano |
| 5 | tolto il blocco `prefers-reduced-motion` | `npm test` → 1 | `AssertionError: --duration-fast: expected '110ms' to be '0ms'` e `FAIL \|jsdom\| src/tokens/board.test.ts > … base.css is the board's block, byte for byte`; tornato il file, `npx vitest run src/tokens/board.test.ts` → `Tests  2 passed (2)` | tornano |
| 6 | tolta la riga `await commands.emulateMedia({ forcedColors: "active" });` | `npm test` → 1 | `AssertionError: expected false to be true` a `src/tokens/tokens.browser.test.ts:84:58`: la guardia, una riga più su nel file violato | tornano |
| 7 | tolto il blocco `:focus-visible` (**E11**) | `npm test` → 1 | `expected false to be true` a `96:25`, `96\|     expect(ring(button)).toBe(true);`, e `board.test.ts` | tornano |
| 8 | `--font-display` col solo ripiego | `npm test` → 1 | `AssertionError: expected 0 to be greater than 0.5` a `67:86`, il primo confronto col ripiego — le righe 56, 60 e 66 passano —, e `board.test.ts` | tornano |
| 9 | tolta `system.addEventListener("change", apply);` | `npm test` → 1 | nel browser `× … through the real query while the choice is \`system\` 5087ms`, `expected 'light' to be 'dark'` a `113:48`, `Caused by: Error: Matcher did not succeed in time.`; sotto jsdom (**E12**) `expected 'dark' to be 'light'` a `theme.test.ts:39:32` | tornano |
| 10 | `gui/src/late.browser.test.ts` dal recinto 1630 del piano a `f00d5b7`; poi, col file lì, tolta la riga `expect:` | `npm test -- --project browser --reporter=verbose` → 0 con la riga, 1 senza | con: `✓ … waits for a value that arrives two seconds late 2059ms`, `Tests  6 passed (6)`; senza: `× … 1051ms`, `AssertionError: expected 'before' to be 'after'`, `Matcher did not succeed in time.`; poi `viol.py unlate` → `exists now = False` | tornano |

Alla fine `git -C C:/Users/zagor/AppData/Local/Temp/rv2 status --porcelain` → vuoto.

### 3.5 Il cancello coi due progetti uno alla volta (E10), nel clone

`bash scripts/gate-gui.sh` dalla radice del clone, in background verso un log, una corsa per volta; il file torna dalla
copia dopo ciascuna, `cmp` uguale, stato vuoto.

| Corsa | exit, tempo | La riga che dice perché |
|---|---|---|
| sul commit | **0**, `real 3m56.904s` | `> vitest run --project jsdom` → `17 passed \| 1 skipped (18)`, `100 passed \| 1 skipped (101)`; `> vitest run --project browser` → `1 passed (1)`, `5 passed (5)`; `found 0 vulnerabilities`; il pezzo `663.93 kB` |
| `include` del `browser` vuoto | **1**, `real 2m12.446s` | il jsdom verde, poi `> vitest run --project browser` → `No test files found, exiting with code 1` e `include: src/**/*.nothing.test.ts`; lint e audit non girano (`set -e`) |
| `include` del `jsdom` vuoto | **1**, `real 1m17.005s` | `> vitest run --project jsdom` → `No test files found, exiting with code 1`; il browser non si raggiunge |

Nota d'ambiente: nel clone `scripts/gate-gui.sh` è CRLF (`core.autocrlf` `true`, checkout fresco); la bash di Git per
Windows lo esegue — misurato prima con uno script CRLF di due righe, `set -euo pipefail` compreso, exit 0.

### 3.6 Il cancello intero, sull'albero del repository

`bash scripts/gate.sh`, da solo, in background verso `review2/gate-review.log`, dalle 08:47:49 alle 08:53:25:
`GATE GREEN.`, `real	5m35.900s`, `exit=0`.

| Riga | Il mio cancello | Il rapporto | La base (`gate-open.log`, `f00d5b7`) |
|---|---|---|---|
| jsdom | `Test Files  17 passed \| 1 skipped (18)`, `Tests  100 passed \| 1 skipped (101)` | uguale | `17 passed \| 1 skipped (18)`, `100 passed \| 1 skipped (101)`, in una corsa sola |
| browser | `Test Files  1 passed (1)`, `Tests  5 passed (5)` | uguale | — |
| audit | `found 0 vulnerabilities` | uguale | uguale |
| il pezzo JavaScript | `dist/assets/index-DLsd9Y_U.js  663.93 kB │ gzip: 201.53 kB` | uguale | uguale |

**Per nome di file, e non solo per conto:** la base nel clone a `f00d5b7` e il progetto `jsdom` a `23b3134`, ciascuno col
reporter JSON (`base-tests.json`, `jsdom-tests.json`), confrontati da `cmp_json.py`: 18 file e 101 prove da tutte e due le
parti, `files only in the first: []`, `files only in the second: []`, `tests only in the first: []`, `tests only in the
second: []`, `status changed: []` — le stesse prove per nome completo, file per file.

`bash scripts/check-docs.sh` da solo → `OK — no inconsistencies.`, exit 0 (e lo stesso dentro il cancello).
`git status --porcelain` dopo il cancello uguale a prima (vuoto).

### 3.7 Il browser è quello vero, e resta senza finestra

- **Quale:** `browser.version()` → `153.0.8010.53`; la 154, in `new_chrome.exe`, non è quella aperta.
- **Durante** un `npm test -- --project browser` verde, `chrome-poll.ps1` (nello scratchpad; `Get-CimInstance
  Win32_Process` ogni 200 ms, sui soli processi col profilo di Playwright): otto processi fra le 09:13:52 e le 09:13:55,
  eseguibile `C:\Program Files\Google\Chrome\Application\chrome.exe` (versione del file `153.0.8010.53`); **un** processo
  browser, con `--headless --user-data-dir=C:\Users\zagor\AppData\Local\Temp\playwright_chromiumdev_profile-i2BZTf
  --remote-debugging-pipe`, e sette figli suoi — tre `renderer`, `gpu-process`, `utility` rete e storage,
  `crashpad-handler`; `processes owning a visible main window: 0`; `probe chrome processes left at the end of the poll: 0`.
  Prima della corsa: `0`.
- **Il canale fa da guardia al prerequisito**, oltre il dettato: con `channel: "chrome-beta"` — un canale valido, non
  installato qui — `npm test -- --project browser` → exit 1, `Error: browserType.launch: Chromium distribution
  'chrome-beta' is not found at C:\Users\zagor\AppData\Local\Google\Chrome Beta\Application\chrome.exe` e `Run "npx
  playwright install chrome-beta"`: il messaggio che i commenti di `vite.config.ts` e di `gate-gui.sh` promettono.
- **La marca:** con `toContain("Microsoft Edge")` al posto di `"Google Chrome"`, rosso `expected [ Array(3) ] to include
  'Microsoft Edge'`: la lista delle marche è piena, e il verde di sempre dice che contiene `"Google Chrome"`.

### 3.8 Il codice, oltre il dettato

| Che cosa | Come l'ho guardato | Esito |
|---|---|---|
| **la guardia di non-vacuità** in ogni prova del browser (vincolo 11) | letto, e le violazioni della §3.4 | c'è in tutte: la scatola da 120 px e la marca; `loaded(…)` sopra zero e sopra quattro, *«the chain starts with Barlow»*; le tre durate a `0ms`; `matches` dell'emulazione e `activeElement`; `data-theme` a `light` prima dei due cambi. **Salvo** la seconda direzione del movimento: M-2 |
| **il meccanismo del progetto, non il browser** (lezione 2 del *«Come si riprende»*) | per ogni prova, una violazione su un file del progetto | caratteri: `tokens/index.ts` e `base.css`; movimento: `base.css`; contorno: `base.css` (**E11**); tema: `theme.ts`; il browser: `vite.config.ts`. Guardano il browser, com'è dichiarato, la guardia dell'emulazione e la direzione del `box-shadow` |
| **`vue-tsc` vede il tipo di `emulateMedia`** | nel clone, `browser.d.ts` tolto e poi un valore sbagliato; `npx vue-tsc --noEmit` | senza il file exit 2, sette `error TS2339: Property 'emulateMedia' does not exist on type 'BrowserCommands'` (righe 12, 72, 77, 83, 108, 112, 114); con `reducedMotion: "fast"` exit 2, `error TS2322: Type '"fast"' is not assignable to type '"reduce" \| "no-preference" \| null \| undefined'`; col file com'è, verde (`npm run build`). L'aumento regge benché `BrowserCommands` sia dichiarata in `vitest/internal/browser` e solo riesportata da `vitest/browser`; `tsconfig.json` include `src/**/*.ts` e `vite.config.ts` |
| **il richiamo nella §8 del disegno del 2** | `grep` e la riga 400, sotto `### §8 — Le prove e il cancello` | porta la data del commit, `2026-09-25`; dice `npm test` due volte, `--project jsdom` e poi `--project browser` — vero, righe 45–46 di `gate-gui.sh` —, e il perché di E10, misurato alla §3.4; il link `../plans/2026-09-23-design-system.md` risolve (`check-docs.sh` `OK`) |
| **nessun commento dice il falso** (gotcha #58) | `grep` sui commenti di `gui/src`, `scripts`, `.github` che nominano la configurazione delle prove | `gate-gui.sh`: *«§8 fixed "npm ci, npm run build, npm test"; this step appends rather than reordering»* resta vero, e il richiamo lo accompagna; `theme.test.ts`: *«the real query is proven in the browser, task 2»* ora è vero; `jsdom-setup.ts`: *«Mounted by `test.setupFiles`»* vero nel progetto `jsdom`; la riga 937 della stella polare, *«`npm test` (`vitest run`) gira in `gate-gui.sh`»*, vera. I tre commenti con una promessa misurabile, misurati: `screenshotFailures` (sotto), il Chrome che manca (`chrome-beta`, §3.7), `headless` di base `isCI` (letto: `resolved.browser.headless ??= isCI` in `vitest/dist/chunks/coverage.DM_a_rWm.js:490`) |
| **`screenshotFailures: false`**, la seconda direzione | nel clone, la riga tolta e la violazione 4 messa; `npm test -- --project browser` | exit 1 e `Failure screenshot: - src/tokens/__screenshots__/tokens.browser.test.ts/the-tokens--in-a-real-browser--…-1.png`; `git status` → `?? gui/src/tokens/__screenshots__/` **e** `?? gui/.vitest-attachments/`. Con la riga, nessuno dei due in nessuna corsa rossa: il commento dice il vero, e la seconda cartella non la nomina ma la evita lo stesso. Le due cartelle, mie, tolte |
| **la finestra delle prove, 1440 × 900** | nel clone, la riga `viewport` tolta (`vite.config.ts:107`) | `npm test -- --project browser` → exit 0, `Tests  5 passed (5)`: nessuna prova la tiene. Con una riga nella prima prova, `expect([innerWidth, innerHeight], "the window of the probes").toEqual([1440, 900]);`, verde sul commit e rossa senza `viewport`, `expected [ 414, 896 ] to deeply equal [ 1440, 900 ]`. **Non è un rilievo:** il pre-controllo l'ha misurata e scritta, *«nessuna prova la tiene, e nessun controllo del disegno lo chiede — registrato, non preso»* (riga 8547 del piano). L'unico elemento nuovo — le prove della cornice del compito 8 sono scritte *«at the probes' 1440 x 900»* (riga 6952) — non basta a riaprirla: senza la finestra quelle prove andrebbero rosse, non verdi. Lo porto al coordinatore come fatto, con la riga misurata |

### 3.9 I vincoli globali

| Vincolo | Comando | Esito |
|---|---|---|
| **7**, le dipendenze in due passi | il lockfile e il manifesto nel diff di `23b3134`; `npm ci --no-audit --no-fund` nel cancello | nello stesso commit; `npm ci` verde nel mio cancello e nel clone, quindi manifesto e lockfile d'accordo |
| **8**, versioni appuntate ed esatte | `lockdiff.py`; `npm ls` nel clone | `"@vitest/browser-playwright": "4.1.11"`, `"playwright": "1.63.0"`, tutte le `devDependencies` esatte; `vitest` resta `4.1.11`; nel lockfile `@vitest/browser` `4.1.11` e `playwright-core` `1.63.0` |
| **9**, i fine-riga | `tr -cd '\r' < f \| wc -c`, `wc -l`, `git ls-files --eol` sull'albero; i CR dei blob a `f00d5b7` e a `23b3134` | nell'indice tutti `i/lf`, 0 CR prima e dopo; nell'albero: `package.json` 43/43, lockfile 5076/5076, `vite.config.ts` 114/114, disegno 726/726, piano 8603/8603, tutti `w/crlf`; `gate-gui.sh` 0/78 `w/lf`; `browser.d.ts` 0/13 e la prova 0/118, `w/lf`: nati LF. `index.ts`, `base.css`, `theme.ts` 15/15, 96/96, 52/52 `w/crlf`, e nessun diff. La forma di ciascun file è quella di prima: su un file che cresce i CR non restano uguali, restano uguali alle righe — la nota N1 del rapporto è giusta |
| **11**, le due direzioni e la guardia | §3.4, §3.8 | le dieci righe nelle due direzioni; la guardia in ogni prova, con M-2 |
| **12**, il kernel non cambia | `git diff --stat f00d5b7..23b3134 -- crates/ gui/schema/`, e con `.github/ Cargo.toml Cargo.lock docs/adr/` | vuoto, e vuoto |

### 3.10 Il contratto del prompt dell'implementatore

| Punto | Esito |
|---|---|
| un commit solo | `git log --oneline f00d5b7..HEAD` → una riga |
| i soli file del punto 2 | gli otto: `gui/package.json`, `gui/package-lock.json`, `gui/vite.config.ts`, `gui/src/browser.d.ts`, `gui/src/tokens/tokens.browser.test.ts`, `scripts/gate-gui.sh`, il disegno del 2, il piano; e `compare_task2.py` non trova nulla di `UNEXPECTED` |
| le righe 1 e 2 della posizione | la 1 con `` `95068bb`, con le cure `69d10fa` ed `e2cd7df` ``, la 2 a `✅ 2026-09-25` con la colonna Commit `—` |
| il messaggio | comincia con `design-system(compito 2): ` |
| niente push | `git status -sb` → `## main...origin/main [ahead 1]` |
| niente co-autore | `grep -ci co-authored` → `0` |

## 4. Ciò che non ho potuto verificare, e perché

| Che cosa | Perché | Che cosa ho misurato al suo posto |
|---|---|---|
| la **CI** di `23b3134` su `ubuntu-latest` e `windows-latest`, la prima corsa col browser | il commit non è pushato (`ahead 1`): la legge il coordinatore dopo il push | niente; `.github/workflows/quality-gate.yml` gira `bash scripts/gate.sh` e non installa Chrome: vale ciò che la (f) dice delle immagini |
| la **seconda direzione della marca**: un Chromium direbbe `Chromium`, non `Google Chrome` | l'unico Chromium sul disco, `%LOCALAPPDATA%\ms-playwright\chromium-1234` (del 2026-08-26; `playwright-core` 1.63.0 vuole la `1243`, `browsers.json`), qui non parte: con `executablePath` su di lui, `Error: browserType.launch: spawn UNKNOWN`; scaricarne uno è vietato (decisione 22) | la lista delle marche piena (§3.7) |
| la **seconda direzione di `headless: true`** | senza la riga si aprirebbe una finestra sullo schermo del proprietario | letto nel sorgente: `resolved.browser.headless ??= isCI`; e misurato che la finestra oggi non c'è (§3.7) |
| i fine-riga dell'albero dell'implementatore **prima** del compito | l'albero ora è a `23b3134` | i blob a `f00d5b7` (tutti `i/lf`) e la forma dopo, uguale a quella che il rapporto dà per prima |
| il comando `npm install` dell'implementatore (vincolo 7, *«fuori dal cancello con `npm install --save-exact`»*) | un comando passato non si osserva | il suo risultato: versioni esatte nel manifesto, lockfile coerente (`npm ci` verde) |

## 5. Note per il coordinatore

- **Il clone** `C:\Users\zagor\AppData\Local\Temp\rv2` resta a `23b3134`, pulito, con `node_modules`; porta tre commit
  sonda irraggiungibili (`32f0bd2`, `30945f5`, `9d75f53`), che se ne vanno col clone.
- **Playwright lascia un profilo vuoto in `%TEMP%` quando il LANCIO fallisce** — non quando la suite passa, né quando una
  prova è rossa: `playwright_chromiumdev_profile-*` da 0 byte dopo il canale inesistente, `chrome-beta` e `spawn UNKNOWN`.
  I cinque lasciati dai miei lanci li ho tolti, compreso uno da 7,9 MB di un mio script `node` interrotto; ne restano due
  non miei: `-hgweE4`, delle 08:23:38 di oggi — la violazione 1 dell'implementatore — e `-JLJsFF`, del 2026-09-23.
  Innocui; nessun processo resta (§3.7).
- **La nota N1 del rapporto** — la regola del dispaccio *«CR uguale a prima»* è falsa per un file che cresce — è
  confermata dalla misura: `vite.config.ts` da 53 a 114 CR, uguali alle righe. È già nel registro del coordinatore.
- **Gli attrezzi della revisione**, nello scratchpad `review2\`: `viol.py` e `run-viol.sh` (le violazioni con le copie),
  `recipe_check.py`, `lockdiff.py`, `cmp_json.py`, `chrome-poll.ps1`, `e14_proposal.py`, `viewport_probe.py`, e i log di
  ogni corsa, uno per riga delle tabelle qui sopra.
