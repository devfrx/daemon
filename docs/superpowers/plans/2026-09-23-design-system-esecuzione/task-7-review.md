# La revisione del compito 7 — le viste col nome, sotto

Revisore: subagente fresco, macchina `Jays`, il 2026-09-27 dalle 17:30. Commit rivisto: `0417c9c`, sopra `fa1ae0e`.
Clone delle mutazioni: `C:\Users\Jays\AppData\Local\Temp\rv7`, lasciato a `0417c9c` staccato, `git status --porcelain`
vuoto, con `gui/node_modules` e tre commit usa-e-getta non referenziati (le prove di `compare_task7.py`). Log e file miei:
`C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\e4b4f474-4c3b-4180-b8de-ae411307e17a\scratchpad\review7\`
(sotto, `<review7>`).

## 1. Il verdetto

**Conforme.** Il commit è il testo del piano byte per byte — dieci file su dieci, e del piano la sola riga 7 —, ogni Atteso
dei Passi 1–8 torna, le quindici violazioni cadono come la tabella detta, il cancello è verde, e ogni affermazione
misurabile del rapporto si rifà uguale. **0 critici, 0 importanti, 4 minori, 5 nit**: tutti difetti del **dettato** — sonde
che mancano, un commento reso falso, un caso che lo schema non vede — e nessuno dell'esecuzione. Le voci candidate partono
da **E82**: il rapporto non ne porta.

## 2. I rilievi

### Minori

**M-1 → E82 candidata — le prove delle viste col nome ne usano UNA sola, e tre righe che scelgono QUALE non le tiene
nessuno.** Tutte e tre sono righe da perdita silenziosa, proprio quella che **D4** vuole evitare (*«sovrascrivere in silenzio
perderebbe una vista»*).

| Riga, ritrovata col `grep -n` | La mutazione, sulla suite intera | Esito |
|---|---|---|
| `gui/src/stores/layout.ts:162`, `named: [...(saved.value?.named ?? []), { name: wanted, layout }]` | `named: [{ name: wanted, layout }]` — *«Salva questa vista»* cancella ogni vista col nome salvata prima | **verde**, `Tests  212 passed \| 1 skipped (213)` (`<review7>/rx-L.log`) |
| `gui/src/stores/layout.ts:140`, `.map((entry) => (entry.name === open ? { name: open, layout } : entry))` | `.map((entry) => ({ name: entry.name, layout }))` — una mossa riscrive **ogni** vista col nome | **verde**, 212 (`rx-J.log`) |
| `gui/src/frame/dock.ts:146`, `pack?.named?.find((entry) => entry.name === named)?.layout` | `pack?.named?.[0]?.layout` — il dock mostra la prima della lista, qualunque sia il nome | **verde**, 212 (`rx-K.log`) |

La causa: in `stores.test.ts` le prove di riga 240 (*«writes a move in an open named view…»*) e 271 (*«saves the layout on
screen under a new name…»*) e in `frame.test.ts` quella di riga 225 (*«shows the named view that is open…»*) hanno **una**
vista col nome, e con una sola «quale» non si distingue da «tutte» o da «la prima». La specie di **E29** — un caso che
giudica sparito senza che la guardia lo veda.
**Cura proposta, in testo** (un'aggiunta al Passo 2, tre prove o tre attese; provate nel clone, `<review7>/proposed7-stores.test.ts`
e `<review7>/proposed7-frame.test.ts`): *(P1)* un pacchetto con `named: [Revisione, Altra]` e `openNamed: "Revisione"`, un
`settle`, e `named` spedito uguale a `[{ Revisione, moved }, { Altra, other }]`; *(P2)* un pacchetto con `named: [Prima]`,
poi `saveNamed("Revisione", …)` → `"saved"`, e `named` spedito `[{ Prima }, { Revisione }]`; *(P5)* nel dock due viste col
nome, aperta la **seconda**, e il dock mostra la seconda. Misurate: verdi sul codice, `Tests  5 passed (5)`; rossa P1 sotto
la mutazione di riga 140, `expected [ { name: 'Revisione', …(1) }, …(1) ] to deeply equal [ { name: 'Revisione', …(1) }, …(1) ]`;
rossa P2 sotto quella di riga 162, `expected [ { name: 'Revisione', …(1) } ] to deeply equal [ { name: 'Prima', …(1) }, …(1) ]`;
rossa P5 sotto quella di riga 146, `expected [ 'costs' ] to deeply equal [ 'status' ]` (`pp-J.log`, `pp-L.log`, `pp-K.log`).
Tre righe nuove nel Passo 8.

**M-2 → E83 candidata — la cura di E75 è tenuta sul ramo `null`, non sul ramo leggibile.** `gui/src/stores/layout.ts:113`,
`openNamed.value = saved.value?.openNamed ?? null;`, e il commento sopra promette *«or none»*. Il caso che il commento
nomina per primo — decisione 13, *«the OLD package after a write that did not stick»* — è un pacchetto **leggibile** senza
il nome; la prova di **E75** consegna `Nothing`, e la seconda direzione di *«keeps the named views…»* parte da un negozio
nuovo, dove il nome è già `null`. Misurato: con `openNamed.value = saved.value === null ? null : (saved.value.openNamed ?? openNamed.value);`
— «tengo il nome se il pacchetto tace», una regressione plausibile — la suite intera resta **verde**, 212 (`rx-A.log`); e
in quello stato la mossa dopo finisce nel nulla, com'era E75. **Cura proposta** *(P3)*: il benvenuto `{ view: "home",
layouts: {} }`, `saveNamed("Revisione", …)`, poi di nuovo il pacchetto vecchio — la scrittura non ha attecchito —:
`openNamed` a `null`, e il `settle` dopo spedisce `{ view: "home", layouts: { home: moved } }`. Verde sul codice, rossa
con la mutazione, `expected 'Revisione' to be null` (`pp-A.log`). Una riga nel Passo 8.

**M-3 → E84 candidata — *«tiles each view that ships without a gap or an overlap»* non guarda le sovrapposizioni.**
`gui/src/frame/schematic.test.ts:38`: la prova somma le aree e guarda i bordi, e una sovrapposizione più un buco della
stessa area sommano a uno lo stesso. Misurato: in `gui/src/frame/schematic.ts:47`, `x: box.x + offset * box.width` →
`x: offset * box.width` (dimenticato l'origine del ramo orizzontale) lascia la suite intera **verde**, 212 (`rx-O.log`),
mentre in Lavoro `steps` e `sensors` finiscono sopra `scope`: la prova `HAND` non lo vede perché il suo ramo orizzontale
parte da `x = 0`, e la striscia resta in fondo. **Cura proposta** *(P6)*, nella stessa prova: per ogni coppia di tessere
l'area dell'intersezione minore di `1e-9`, col perché nel commento — somma uguale al quadrato, tessere dentro e nessuna
coppia sovrapposta vogliono dire che le tessere lo coprono. Verde sul codice; rossa con la mutazione, `work: scope over steps:
expected 0.07846999999999998 to be less than 1e-9` (`pp-code-s.log`, `pp-O.log`; il file `<review7>/proposed7s.test.ts`).
Una riga nel Passo 8.

**M-4 → E85 candidata — lo schema non vede un gruppo INGRANDITO, e la miniatura dice il falso.** `grep -c maximizedNode
gui/src/frame/schematic.ts` → `0`. La presa grande ha il comando *ingrandisci* (`gui/src/frame/BigTab.ts:47`,
`parameters.api.maximize()`); `dockview-core` 8.3.1 serializza la griglia **non** ingrandita più `grid.maximizedNode`
(`serialize()` in `node_modules/dockview-core/dist/package/main.esm.mjs`, riga 2044, letto) e la rimette ingrandita in
`deserialize`. Misurato dal consumatore (§4), sotto jsdom col `createDock` vero: `group.api.maximize()` → **un**
`SaveLayout`, con `layouts.home.grid.maximizedNode` = `{ "location": [0, 0, 0, 0] }`; il dock mostra **1** gruppo; dopo
`fromJSON` `hasMaximizedGroup()` → `true`; e `schematic` di quel pacchetto rende **7** tessere, la griglia intera. La
Panoramica del compito 8 disegnerebbe una vista che si apre con un gruppo solo come la griglia intera, e la (d) le chiede
che le miniature *«dicono sempre il vero»* (risposta 19). **D5** pensava ai galleggianti, non a questo. **Due strade**:
**A** — `schematic` segue `grid.maximizedNode.location` fino alla foglia e rende quel gruppo solo, sul quadrato intero, ciò
che si apre; una prova sotto jsdom da un `toJSON()` vero dopo `maximize()`; poche righe, e il merito approvato realizzato.
**B** — D5 lo dichiara come i galleggianti, e il commento di `schematic` con lei: costa una miniatura falsa in uno stato che
il proprietario raggiunge con un clic, contro la (d). Il consiglio è **A**; come si disegni la vista ingrandita è aspetto, e
lo sguardo del compito 8 lo vede.

### Nit

**N-1 → E86 candidata — un commento di `gui/src/frame/Frame.vue` che il compito rende falso** (gotcha **#58**, la specie di
**E78**): riga 17, *«The mapping and the geometry live in `moveActive.ts`; this is only the wire.»* Dal Passo 3 la
geometria vive in `nearest.ts` — lo dice `moveActive.ts:18` stesso, *«The geometry itself is `nearest`»*. Il compito tocca
`Frame.vue` e non questa frase: `grep -n 'geometry live' gui/src/frame/Frame.vue` → `17:`. **Cura proposta**: una seconda
sostituzione in `Frame.vue` nel Passo 6, *«// mapping lives in `moveActive.ts` and the geometry in `nearest.ts`; this is
only the wire.»*; nessuna prova cambia.

**N-2 → E87 candidata — due commenti danno per presente il chiamante del compito 8.** `grep -rn "nearest(" gui/src
--include=*.ts --include=*.vue` fuori dalle prove → il solo `moveActive.ts:26`; ma `moveActive.ts:18` dice *«shared with the
overview's grid since the design system»* e `nearest.ts:13` *«and of the arrows in the overview's grid … its second
occurrence»*. Veri dal compito 8, che usa `nearest` in `onArrow` (piano a `fa1ae0e`, compito 8). La specie di **E22 (b)**,
curata allora con *«from task 5»*. **Cura proposta**: *«… shared with the overview's grid from task 8 …»* e *«… and, from
task 8, of the arrows …»*; oppure dichiararla, come **E76**, perché vive un compito solo.

**N-3 → E88 candidata — le guardie di `readNamed` su una voce `null` e su una disposizione `null` non le tiene nessuna
prova.** `gui/src/stores/layout.ts:59`, `if (typeof entry !== "object" || entry === null) continue;`, e `:61`, `… || layout ===
null`. Tolta la prima, la suite resta **verde** (`rx-B.log`) — e una voce `null` fa lanciare la destrutturazione dentro il
`try` di `unpack`, che rende `null`: **tutto** il pacchetto perso, disposizioni e tema; tolto `|| layout === null`, **verde**
(`rx-C.log`), e una vista col nome con la disposizione `null` resta. **Cura proposta** *(P4)*: nella lista di *«drops what it
cannot read…»* anche `null` e `{ name: "Nulla", layout: null }`. Verde sul codice; rossa con la prima mutazione, `expected null
to deeply equal { view: 'home', layouts: {}, …(1) }`, e con la seconda (`pp-B.log`, `pp-C.log`).

**N-4 → E89 candidata — due rami difensivi senza sonda, da dichiarare.** `gui/src/frame/schematic.ts:43`, `total > 0 ? … :
1 / children.length`: tolto il ramo, **verde** (`rx-D.log`) — nessuna disposizione vera ha le `size` a zero. E la frase di
`gui/src/frame/dock.ts:142-143`, *«a name the package no longer holds falls back to the view of always»*: con `apply` che
non ricade, **verde** (`rx-H.log`); dopo **E75** la strada c'è solo scrivendo in `openNamed` un nome che la lista non ha,
misurato dal consumatore (§4), e il compito 8 scrive solo nomi della lista. Bastano due righe nel testo del compito, come
**E76**.

**N-5 → per il pre-controllo del compito 8 — una disposizione malformata rompe TUTTA la Panoramica.** `unpack` tiene come
disposizione ogni oggetto — la prova di riga 224 tiene `{ name: "Revisione", layout: {} }` — e `schematic({})` lancia
`TypeError: Cannot read properties of undefined (reading 'root')`, misurato dal consumatore. Il dock lancia già sulla
stessa, `Error: dockview: root must be of type branch` — la classe è di prima, dalla parte 2 —, ma solo quando quella vista
si mostra; il `computed` delle schede del compito 8 chiama `schematic` su **ogni** vista insieme. La cura, se c'è, è del
compito 8: una miniatura che non si legge si disegna vuota, e la sua prova.

### Ciò che il rapporto dice di sé, giudicato

- **Il `git push` del Passo 9 e del vincolo 15** non fatto: giusto, il dispaccio lo vieta e il push è del coordinatore dopo
  la revisione. Nessun rilievo.
- **La cella della riga 7** a `✅ 2026-09-27`, senza la nota del pre-controllo: alla lettera del Passo 9, e come le righe 3, 4
  e 6 nei commit dei loro implementatori — misurato, `git show c7b7bcd`, `841dc54` e `c1102fc` sul piano: ciascuno porta la
  sua riga da `⬜` a `✅ <data>` con `—` nella colonna Commit. La traccia del pre-controllo resta in E75–E81. Nessun rilievo.
- **I due `prima.txt`**: corretto, e i due `diff` tornano con i loro momenti.

## 3. I comandi rilanciati, con le uscite

Il repository e il clone, uno alla volta; nessuna suite insieme a un'altra, nessuna suite col cancello acceso. La memoria
libera prima di cominciare, `Get-CimInstance Win32_OperatingSystem` → `6,4 GB free of 31,2 GB`. Chrome letto dal nome
della cartella, `ls "/c/Program Files/Google/Chrome/Application/"` → `154.0.8037.58`; Node `v24.19.0`.

| # | Comando | La mia uscita | Il rapporto |
|---|---|---|---|
| avvio | `git rev-parse --short HEAD`; `git status --porcelain`; `git log --oneline fa1ae0e..HEAD` | `0417c9c`; vuoto; **una** riga | — |
| contratto | `git show --stat 0417c9c` | 11 file, `432 insertions(+), 43 deletions(-)`: i dieci di `gui/` e il piano | uguale |
| contratto | `git log -1 --format=%B 0417c9c \| grep -ci co-authored` | `0` | `0` |
| contratto | `git status -sb` | `## main...origin/main [ahead 1]` | niente push |
| contratto | il messaggio | comincia con `design-system(compito 7): `, porta *«da 691.82 kB, compresso 210.77 kB, a 693.02 kB, compresso 211.20 kB»* | uguale |
| v. 12 | `git diff --stat fa1ae0e..0417c9c -- crates/ gui/schema/ \| wc -l` | `0` | vuoto |
| v. 7 | `git diff --stat fa1ae0e..0417c9c -- gui/package.json gui/package-lock.json \| wc -l` | `0` | — |
| v. 9 | per ogni file, CR del blob a `fa1ae0e` e a `0417c9c`, CR dell'albero, `git ls-files --eol` | tutti `CR=0` prima e dopo, `i/lf w/lf`, l'ultimo byte `\n`; righe `layout.ts` 144→223, `stores.test.ts` 195→303, `moveActive.ts` 63→53, `dock.ts` 152→157, `Frame.vue` 70→71, `frame.test.ts` 275→291, il piano 11210→11210, i nuovi 40, 42, 56, 52 | uguale, cifra per cifra |
| log | `ls -la --time-style=full-iso` sui log del rapporto contro `git log -1 --format=%ci 0417c9c` (`17:27:52`) | `gate-passo1-…171820.log` 17:19:34, `gate-passo9-…172537.log` 17:26:54: prima del commit, nella cartella `task7/`, e diversi da `logs/gate-apertura.log` del coordinatore (17:09:34, `cmp` diverso) | i due log citati |
| log | `grep` nei log dell'implementatore | Passo 1 `index-DKNLC6Y2.js 691.82 kB │ gzip: 210.77 kB`, jsdom 19+1/144+1, browser 6/56; Passo 9 `index-v8xlIAVo.js 693.02 kB │ gzip: 211.20 kB`, jsdom 21+1/156+1, browser 6/56; le cinque corse del Passo 7 tutte `27 passed \| 1 skipped (28)` e `212 passed \| 1 skipped (213)`, 5,36–5,74 s | uguale |
| 1 | `git grep -nw Direction fa1ae0e -- 'gui/src/*.ts' 'gui/src/*.vue'`; `git grep -ln i18n fa1ae0e -- gui/src/stores`; `git grep -ln Frame.vue fa1ae0e -- 'gui/src/*.test.ts'` | tre righe, tutte `moveActive.ts` (`:3`, `:18`, `:47`); uscita 1; uscita 1 | uguale |
| 2 | `compare_task7.py fa1ae0e 0417c9c`, dalla radice, `PYTHONIOENCODING=utf-8` | dieci `OK`; il piano `CHECK BY HAND` con **una** riga cambiata, la 7 — la 6 no (E79); il disegno assente; `--- 11 paths, 0 mode changes, 0 not matching the plan's text`, uscita 0 | uguale |
| 2 | lo script provato: nel clone un commit che cambia `rectangle` in `rectanglE` nel commento di testa di `nearest.ts` (forma `W`) | `DIFFERS gui/src/frame/nearest.ts` e nessun altro, `1 not matching`, uscita 1 | — |
| 2 | un commit che cambia `in px` in `iN px` in `dock.ts:14`, fuori da ogni sostituzione (forma `R`) | `DIFFERS gui/src/frame/dock.ts` e nessun altro, uscita 1 | — |
| 2 | in più: un commit che tocca il disegno | `UNEXPECTED docs/superpowers/specs/2026-09-22-design-system-design.md`, uscita 1; il clone tornato a `0417c9c` | — |
| 2 | le righe della ricetta, `sed -n` sul piano a `fa1ae0e` | le 23 righe aprono ciascuna il recinto che la ricetta dice, dietro il *Crea*, *Trova*, *Sostituisci con* o *Riscrivi* giusto | — |
| 3 | `plan_ops.py <review7>/plan-base.md 7 list` | `14 operations`, alle righe 8101, 8148, 8205, 8217, 8345, 8393, 8438, 8457, 8507, 8576, 8813, 8834, 8863, 8884 — quelle della ricetta | `14` |
| 3 | nel clone a `fa1ae0e` pulito, `plan_ops.py … apply` di tutto, poi `git add -A` e `git diff --cached --stat 0417c9c -- gui/` | `14 operations, 0 refused`; il `diff` di `gui/` **vuoto**; sull'albero intero il solo piano, la riga 7 che `plan_ops.py` non scrive | `14 operations, 0 refused` |
| 3 | il brief contro il piano: la sezione `## Compito 7` di `plan-base.md` e quella del brief | uguali, a meno di una riga vuota in coda | — |
| 3 | Passo 2, `<review7>/step.sh 2`: clone a `fa1ae0e`, `apply --upto 2`, il comando del passo | `Tests  7 failed \| 28 passed (35)`; `Failed to resolve import "./nearest"` e `"./schematic"`; `stores.test.ts (20 tests \| 6 failed)` coi messaggi dell'Atteso, `layout.showView is not a function` e due volte `layout.saveNamed is not a function`, la seconda la prova di E75; `frame.test.ts (15 tests \| 1 failed)`, `expected [ 'activity', 'costs', …(5) ] to deeply equal [ 'status' ]`; verdi le 14+14 di prima | uguale, per nome di prova |
| 3 | Passi 3, 4, 5, 6, in fila sugli stessi `node_modules` | `Tests  8 passed (8)`; `Tests  2 passed (2)`; `Tests  20 passed (20)`; `Tests  15 passed (15)` | uguale |
| 4 | le quindici violazioni della tabella, scritte da me dal testo del brief (`<review7>/rv_violate.py`), una alla volta, `npx vitest run` intero, indietro con la copia e `cmp` | ciascuna rossa col messaggio della tabella e con le prove che nomina, né più né meno — la 2 anche `keys.test.ts`, `expected 'moved' to be 'split'`, 4 prove; la 8 anche il dock, 4; la 10 anche il dock, 2 —; `restored: cmp equal` 16 volte; `git status --porcelain` uguale a prima (`<review7>/rv-step8-run.log`, `rv-v1.log`…`rv-v15.log`) | uguale, riga per riga |
| 4 | **E76**: in `Frame.vue` `layout.view = view;` al posto di `layout.showView(view);`, suite intera | **verde**, `Tests  212 passed \| 1 skipped (213)` (`rv-v16.log`): la dichiarazione è vera | non rifatta, com'era detto |
| 5 | tre corse di `npx vitest run` nel clone a `0417c9c`, una alla volta | tre volte `Test Files  27 passed \| 1 skipped (28)` e `Tests  212 passed \| 1 skipped (213)`, 7,3–7,6 s (`stab-1.log`…`stab-3.log`). Nessuna caduta; e in nessuna delle 33 corse intere di questa revisione a `0417c9c` è caduta una prova che la mutazione non toccava, la tastiera di P-19 compresa | cinque su cinque |
| 6 | `bash scripts/gate.sh` sull'albero del repository, da solo, in background (`<review7>/gate-review7.log`) | `GATE GREEN.`, 1m14s; jsdom `Test Files  21 passed \| 1 skipped (22)` e `Tests  156 passed \| 1 skipped (157)`; browser `Test Files  6 passed (6)` e `Tests  56 passed (56)`; `dist/assets/index-v8xlIAVo.js  693.02 kB │ gzip: 211.20 kB`; `found 0 vulnerabilities` | uguale |
| 6 | contro la base, per nome: `npx vitest run --reporter=json` nel clone a `0417c9c` e a `fa1ae0e`, confrontati da `<review7>/names.py` | base 26 file, 200 passate e 1 saltata; `0417c9c` 28 file, 212 e 1; file solo alla base: nessuno; solo dopo: `nearest.test.ts`, `schematic.test.ts`; prove della base sparite: **0**; esiti cambiati: nessuno; nuove: 12 — 3, 2, 6 e 1 | — |
| 6 | il pezzo JavaScript contro la base | da `691.82 kB`, compresso `210.77 kB` (il cancello d'apertura del coordinatore e il Passo 1) a `693.02 kB`, compresso `211.20 kB`: **+1,20 kB**, compresso **+0,43 kB**, per il proprietario (**E80**, N-2) | uguale |
| 6 | `bash scripts/check-docs.sh` | `OK — no inconsistencies.`, uscita 0 | uguale |
| 7 | le mutazioni oltre la tabella (`<review7>/rv_extra.py`, `rx-*.log`), suite intera | **verdi**: A, B, C, D, H, J, K, L, O — in M-1…M-3 e N-3…N-4; **rosse**: E (i nomi delle viste col nome non presi, `expected 'saved' to be 'taken'`), F (il nome vuoto, `expected 'saved' to be 'empty'`), G (la lista vuota tenuta, 6 prove), N (`saveNamed` che non apre, `expected null to be 'Revisione'`) | — |
| 7 | le sonde proposte, nel clone, sul codice e sotto ciascuna mutazione | verdi sul codice, `Tests  5 passed (5)` e `1 passed (1)`; rossa ciascuna sotto la sua (`pp-*.log`) | — |
| 7 | `npx vue-tsc --noEmit` nel clone col consumatore dentro | uscita 0: gli usi del compito 8 compilano | — |

## 4. Il consumatore da fuori — punto 7(c)

**Che cosa ho scritto:** `gui/src/frame/overview-consumer.test.ts`, nel solo clone, poi tolto — la copia è
`<review7>/overview-consumer.test.ts` e ciò che ha misurato `<review7>/consumer7-out.json`. Usa le interfacce come le usa il
testo del compito 8: `createDock` vero sotto jsdom; il benvenuto; un pannello spostato; *«Salva questa vista»* con
`saveNamed` e i nomi mostrati letti da `i18n` (`views.home`, `views.work`, `views.compact`); le miniature con `schematic` sulle
tre viste — quella del proprietario se c'è, altrimenti la spedita — e sulle viste col nome; la scheda corrente con
`openNamed ?? view`; l'apertura scrivendo `openNamed`, la chiusura con `showView`; le frecce con `nearest` su sei schede
scritte a mano, tre colonne a 24 px come la tavola; e i bordi taglienti. Lanciato con `npx vitest run --project jsdom`,
`Tests  4 passed (4)`; e `npx vue-tsc --noEmit` uscita 0.

**Che cosa ha reso:**

| Uso | Esito |
|---|---|
| `saveNamed` | `"  "` → `"empty"`; `" lavoro "` → `"taken"` per il nome mostrato; `"Revisione"` → `"saved"` e `openNamed` `"Revisione"`; `"REVISIONE"` → `"taken"`; una seconda, `"Seconda"`, dopo `showView("home")` → `"saved"`, e la lista `["Revisione", "Seconda"]` |
| il dock | dopo il salvataggio mostra la vista salvata; aperta `"Revisione"` scrivendo `openNamed`, la mostra, e nessun `SaveLayout` in più per averla aperta (`sent` fermo a 3); dopo `showView("work")` mostra Lavoro e `openNamed` è `null` |
| `schematic` | Home — quella del proprietario, col pannello spostato, che il `settle` ha scritto in `layouts.home` — 6 tessere, con `costs+status` in una; Lavoro 9 e Compatta 2, le spedite; ciascuna vista col nome la sua; `active` c'è su ogni tessera, e le frazioni sommano |
| `nearest` | la griglia si percorre come una griglia — da *Salva* su va a Compatta, da Seconda su a Lavoro, da Compatta giù a *Salva* —; con una scheda che manca sotto Compatta, giù va alla più vicina sull'altro asse, Seconda; un `DOMRect` passa per `Box` e compila |

**Che cosa l'interfaccia rende scomodo, ambiguo o sbagliato:**

1. **Il gruppo ingrandito** — M-4: la miniatura disegna 7 tessere dove la vista si apre con un gruppo solo.
2. **Una disposizione malformata** — N-5: `schematic({})` lancia, e `apply` lancia già; la Panoramica le chiede tutte insieme.
3. **`openNamed` si scrive senza guardia.** Scritto un nome che la lista non ha, `"Fantasma"`, il dock ricade sulla vista di
   sempre, e il `settle` dopo spedisce `layouts: {}` con `named` com'era e `openNamed: "Fantasma"` nei byte: la mossa è persa
   in silenzio, e il nome penzola finché `unpack` non lo butta al ritorno. È la malattia di **E75** per un'altra porta.
   ⚠️ **Dedotto dal testo del piano:** il compito 8 scrive solo `entry.name` di `saved.named`, quindi oggi la porta non la
   apre nessuno; lo tiene il suo pre-controllo, o un `showNamed(name)` che rifiuta un nome ignoto, simmetrico a `showView`.
4. **Due campi per la scheda corrente**, `openNamed ?? view`: il costo che **D3** dichiara; nessuna sorpresa.
5. **`nearest` esclude la scheda di partenza per geometria, non per identità.** Con rettangoli nulli — jsdom, o una scheda
   non disegnata — rende la scheda stessa (`zeroRectsDownFromHome` → `"home"`), e il titolo *«never the card it starts
   from»* vale per rettangoli più larghi di un pixel. ⚠️ **Dedotto dal testo del piano:** `onArrow` del compito 8 toglie
   `from` dai candidati, come `moveActive`; resta da sapere per le sue prove sotto jsdom.

## 5. Ciò che non ho potuto verificare, e perché

- **La CI**: è del coordinatore, dopo il push.
- **La macchina `zagor`** e il suo `w/crlf`: qui tutto è `i/lf w/lf`; i file nuovi nascono LF, misurato solo qui.
- **Lo sguardo nel browser**: la regola 5 non lo chiede per il compito 7, e non ho aperto la SPA.
- **Il `--dry` dell'implementatore sul suo albero**: al suo posto l'`apply` sul clone a `fa1ae0e`, 14 su 14, e il confronto a
  vuoto col commit.
- **E81**: è del pre-controllo del compito 8.
- **Il consumatore del compito 8 scritto per davvero**: ho usato le interfacce come le usa il testo del compito 8 —
  `drawn`, `cards`, `onArrow`, `saveNamed` letti da `plan-base.md` —, non il suo codice, che non esiste ancora.
