Sei il **curatore del compito 8** del piano `docs/superpowers/plans/2026-09-23-design-system.md` — *la cornice: la barra
col nome della vista, la Panoramica, la striscia a pillola* —, nel repository `E:\ALL\DEV\MY_REPOS\daemon` (Windows; il tool
Bash è Git Bash: apri ogni chiamata con `cd /e/ALL/DEV/MY_REPOS/daemon &&`). Sei un subagente fresco. Il compito è
**eseguito** — `ac56b11`, il testo del piano byte per byte — e **rivisto**: conforme al dettato, con **nove voci
candidate**, **E102**…**E110**. Il coordinatore le ha **decise** una per una coi cinque criteri di
`anthropic-skills:decision-principles`, fuori dal merito approvato (le decisioni sono qui sotto, §2): tu le **curi**, le
**provi** nelle due direzioni, le **scrivi** nell'errata e **allinei** il testo del compito 8. Un commit solo, niente push.

**All'avvio verifichi, e se non torna ti fermi e lo riporti:** `git rev-parse --short HEAD` → `ac56b11`;
`git status --porcelain` → vuoto; `git log --oneline bc6e94d..HEAD` → una riga; `node --version` → `v24.19.0`;
`git config --show-origin --get-all core.autocrlf` → l'ultima riga `false` da `.git/config` (l'albero è `w/lf`); Chrome
`154.0.8037.58` dal **nome della cartella**, `ls "/c/Program Files/Google/Chrome/Application/"`. ⛔ **Mai
`chrome.exe --version`**: su Windows apre il browser col profilo dell'utente.

## 0. Le regole di questa macchina e di questo repository

- **Codice in inglese, documenti in italiano**; un riferimento al codice in un documento porta il **nome esatto** del
  sorgente. Nessuna cifra nuova in un documento senza il comando che la produce accanto, o la data e la macchina della misura.
- **I fine-riga si conservano per file**: prima e dopo, `git ls-files --eol <file>` e `tr -cd '\r' < <file> | wc -c`. Su
  questa macchina i file di `gui/` e il piano sono LF, zero CR. Si scrive con Python `newline=""` (temporaneo più
  `os.replace`), **mai `sed -i`**. I file nuovi nascono LF.
- **Una cosa alla volta**: il cancello e un `npm`/`vitest` sullo **stesso** albero si pestano su `gui/node_modules`
  (gotcha #133); due suite insieme rendono instabili le prove col tempo (P-20, P-21). La memoria libera è poca — 5,1 GB su
  31,2, misurati dal coordinatore alle 20:45 —: mai due suite, mai due cancelli. Il cancello, `bash scripts/gate.sh`, dura
  da uno a dieci minuti: **da solo**, in background, con l'uscita in un log nello scratchpad, e aspetti la notifica.
- ⛔ Il browser delle prove è il **Chrome installato**: niente `npx playwright install`, niente download. Un rosso del
  progetto `browser` che non nomina una prova (`Target page, context or browser has been closed`) si rilancia dopo aver
  riletto la versione dal nome della cartella: Chrome si aggiorna da sé.
- **Ogni direzione rossa si torna indietro con la COPIA SALVATA, mai con `git checkout`** (vincolo 11 del piano): prima,
  `git status --porcelain > <scratchpad>/prima.txt` e una copia di ogni file che le mutazioni toccano; dopo **ciascuna** la
  copia torna e `cmp` lo conferma; alla fine `git status --porcelain` nomina soltanto i file delle cure.
- Lo **scratchpad** per log, copie e attrezzi:
  `C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\d19536a1-7be4-437b-8e11-39fed58af059\scratchpad\cure8`
  (in Git Bash `/c/Users/Jays/AppData/Local/Temp/claude/E--ALL-DEV-MY-REPOS-daemon/d19536a1-7be4-437b-8e11-39fed58af059/scratchpad/cure8`).
- Un **clone** con `node_modules` già installato c'è: `C:/Users/Jays/AppData/Local/Temp/rv8`, lasciato dal revisore a
  `ac56b11` e pulito (misurato dal coordinatore: `git -C … status --porcelain` vuoto). Ti serve al §4, e puoi usarlo come
  vuoi; non lo cancelli: è del coordinatore. `node_modules` è ignorata, e `git clean -fd` non la tocca.
- **Niente subagenti.** ⛔ Non tocchi `crates/`, `.github/`, `Cargo.*`, `gui/schema/`, `gui/package.json`,
  `gui/package-lock.json`, `scripts/`; dei documenti, **solo il piano**, e nel piano **solo** ciò che il §3 elenca — ⛔ né la
  tabella della posizione (le celle le scrive la chiusura, **E101**) né la sezione *«Come si riprende»*.

## 1. Che cosa leggi

Nella cartella di lavoro `.superpowers/sdd/2026-09-23-design-system/`, git-ignorata:

| File | Che cos'è |
|---|---|
| `task-8-review.md` | **la revisione**: il §2 per intero — i rilievi **I-1**, **I-2**, **M-1**…**M-4**, la candidata **E102** confermata al §2.3, **N-1**, **N-2** —, e dove ti serve il §4 (il consumatore da fuori) e il §5 (che cosa ha visto). Le prove e gli script del revisore stanno in `scratchpad/review8/`, accanto al tuo: `consumer/`, `extra_rows.py`, `violations8.py`, `probe8*.mjs`, i log |
| `task-8-report.md` | il rapporto dell'implementatore, e al suo §7 la candidata **E102** |
| `task-8-brief.md` | il compito come era dettato: l'errata — le righe **E82**…**E90** sono il **modello** delle righe che scrivi (le cure della revisione del compito 7) —, e il compito 8 intero, con la tabella del **Passo 9** |

Poi i file che le cure toccano, **prima** di scriverli. ⛔ Non leggi il piano intero (oltre 11 000 righe): `grep -n` e
`sed -n` nel punto esatto. Non leggi `docs/HANDOFF.md`, `docs/adr/`, `docs/archivio/`, l'audit.

## 2. Le cure — decise dal coordinatore

Per **ogni** cura che cambia il comportamento: **la prova prima**, vista **rossa** sul codice di oggi col suo messaggio vero;
poi la cura, e la prova **verde**; poi la **seconda direzione**: la cura tolta a mano (copia salvata), la prova di nuovo
rossa per la ragione giusta, e la copia che torna. ⚠️ **Preferisci attese dentro le prove che ci sono**, come le cure di
**E82**: i nomi e i conti delle prove non cambiano, né i rossi del Passo 3. Dove una prova **nuova** serve, dillo, e i conti
degli Atteso si rimisurano (§4).

| Voce | Classe | La cura decisa | Dove |
|---|---|---|---|
| **E102** | Nit | nel piano, il comando `jsdom` del **Passo 7** del compito 8 prende `src/stores/stores.test.ts`, coi quattro file nell'ordine del Passo 3 — **E93** lo dichiarava già fatto. La riga cambia **senza spostare righe**. Nessun codice | il piano |
| **E103** | Importante | la **strada A** della revisione: il **generatore** scrive ogni vista **dopo** che `dockview` ha applicato i vincoli — `api.layout(1600, 1000, true)` prima di scriverla —, per **tutte e tre** le viste, non per la sola Compatta: la radice è il generatore, non il file. Poi `REGENERATE_VIEWS=1 npx vitest run src/panels/views/generate-views.test.ts` sotto `gui/`, e il diff dei tre JSON: Compatta a `[944, 56]` (misurato dal revisore sotto jsdom); **Home e Lavoro dovrebbero restare uguali — è DEDOTTO**: se cambiano, ti fermi su questa voce e lo riporti col diff, senza committarla. La prova: nella prova delle miniature di `frame.test.ts` (*«draws each view in miniature from its layout…»*), un'attesa per **ogni** vista spedita — le tessere senza la striscia arrivano in basso almeno fin dove comincia la riga della striscia —, rossa sul `compact.json` di oggi. E nel piano un **richiamo datato** nella riga **D18** (`grep -n '^| \*\*D18\*\* |'`), che oggi dichiara un costo — *«a tutta pagina»* — che il file smentiva | `gui/src/panels/views/generate-views.test.ts`, i tre JSON in `gui/src/panels/views/`, `gui/src/frame/frame.test.ts`, il piano |
| **E104** | Importante | in `createDock` (`gui/src/frame/dock.ts`) il corpo del `settle` diventa una funzione, che ascoltano **sia** `api.onDidLayoutChange` **sia** `api.onDidMaximizedGroupChange`. La prova, sotto jsdom accanto alle prove del dock in `frame.test.ts`: reso attivo un gruppo, `maximize()` spedisce un `SaveLayout` col `maximizedNode`, `exitMaximized()` uno senza — rossa oggi (il revisore l'ha misurato: `<review8>/consumer/zz-review-maximize.test.ts`, `<review8>/consumer-maximize.log`). ⛔ **E la regola che c'è non si rompe** — il commento *«SHOWING RESETS THE BASELINE»* di `dock.ts`: **misura prima e dopo la cura** che cosa spedisce il **mostrare** una vista (`fromJSON`) la cui disposizione porta `maximizedNode` — una vista col nome ingrandita, per esempio —; se la cura aggiunge un salvataggio lì, lo tratti con lo stesso meccanismo del `fromJSON` di oggi, e lo dici; una prova tiene la risposta | `gui/src/frame/dock.ts`, `gui/src/frame/frame.test.ts` |
| **E105** | Minore | nel **kit**, non nella Panoramica, perché vale per ogni `BaseLabel` e `BaseIcon` dentro un `BaseButton` spento: in `gui/src/components/BaseButton.vue`, accanto a `.base-button:disabled`, le parole e l'icona prendono il colore dello spento del pulsante (`color: inherit`), col perché nel commento — la (b): un pulsante spento prende `--color-text-disabled`, la specie di **E19**. Il peso del selettore contro le regole di `BaseLabel` è **dedotto** dal revisore: **misuralo** con la prova. La prova, nel browser: le parole della carta `[data-card="save"]` **prima** della risposta del core, contro l'oracolo `computed("color", "--color-text-disabled")` — rossa oggi (misurato dal revisore: il colore è quello di `--color-text-muted`). ⚠️ Controlla che le prove del kit e i contrasti non cambino: il testo di un controllo spento è esente dal contrasto (WCAG 1.4.3); se una prova esistente cade, ti fermi su questa voce e lo riporti | `gui/src/components/BaseButton.vue`, `gui/src/frame/frame.browser.test.ts` |
| **E106** | Minore | in `gui/src/frame/Overview.vue` il `BaseTextField` del nome prende `:placeholder="$t('overview.name')"` — la chiave c'è, e `placeholder` arriva all'`input` (`BaseTextField.vue`, il commento *«The caller's attributes»*); nella prova *«says under the field a name that is empty or taken…»* un'attesa sul `placeholder` dell'`input`, rossa oggi | `Overview.vue`, `frame.test.ts` |
| **E107** | Minore | le attese che la revisione propone al suo M-3 (`task-8-review.md`, *«Testo proposto»* di M-3), nelle prove che ci sono: **X2**, **X3**, **X5**, **X8**, **X10** nella prova delle frecce di `frame.browser.test.ts`; **X6**, **X7**, **X9** sotto jsdom in `frame.test.ts`. Ciascuna vista rossa con la **sua** riga tolta — la mutazione della revisione, `<review8>/extra_rows.py` —, poi verde. **X4**, **X12** e **X14 dichiarati**, come **E89**: aspetto o ripristino che nessun caso d'uso oggi produce — il perché nella riga d'errata | `frame.browser.test.ts`, `frame.test.ts` |
| **E108** | Minore | in `gui/src/testing/probes.ts` l'oracolo `computed` chiama **prima** `readToken(token)`, che lancia su un token che la pagina non definisce — la regola di `readToken` (**E73**) —, col perché nel commento; una prova in `gui/src/testing/probes.browser.test.ts` che un token inesistente lancia. ⛔ **Il nome resta `computed`** — decisione del coordinatore: lo scontro col `computed` di Vue fallisce **forte**, al caricamento (`Identifier 'computed' has already been declared`), e si risolve con un alias dove capita; rinominare toccherebbe tre file di prova per un caso che nessun file ha oggi. Dichiarato nella riga | `probes.ts`, `probes.browser.test.ts` |
| **E109** | Nit | le quattro frasi di N-1 riscritte **senza conto né elenco chiuso**, come la revisione propone: la testa di `probes.ts`, `axe.ts` (*«turn it ON»*), `frame.browser.test.ts` (*«task 8 adds the strip and the overview»*), `Overview.vue` (*«it the store holds no package»*). Nessuna prova cambia | i quattro file |
| **E110** | Nit, d'aspetto | **nessuna cura**: la carta *«Salva questa vista»* contro la tavola la giudica il proprietario al **Passo 10** (controllo 15). La riga d'errata lo dice, col fatto visto dal revisore e la misura che proporrebbe | il piano |

## 3. Il piano

1. **Le righe d'errata E102…E110**, subito dopo la riga **E101**, nella forma di **E82**…**E90**: la classe, *«Compito 8,
   Passo N — …»* (o *«Di prima del compito 8»* per **E104**), il fatto con chi l'ha trovato — la revisione del compito 8,
   col suo codice (I-1…N-2), o il rapporto dell'implementatore per **E102** —, la decisione del coordinatore con
   l'alternativa scartata e il suo costo, e *«✅ Curata nel commit che scrive questa riga»* coi messaggi rossi e verdi che
   **tu** hai misurato, la data e la macchina (`Jays`). Una riga è **una** riga di tabella: niente a-capo dentro.
2. **Il testo del compito 8 allineato** alle cure, **anche per i file che il compito non scriveva**: le prove di **E103**,
   **E104** ed **E105** stanno in file del compito, e le loro cure fuori — col testo del compito ripetuto senza quelle cure,
   il Passo 7 sarebbe rosso. Quindi **ogni cura diventa un'operazione del compito**, nel passo dove serve: `dock.ts` nel
   Passo 6, con la cornice; `BaseButton.vue`, `generate-views.test.ts` e `compact.json` nel Passo 5, con la Panoramica — il
   JSON come *Riscrivi* per intero, la copia di ciò che il generatore ha scritto, detto accanto; la guardia di `computed`
   dentro la sostituzione di `probes.ts` del Passo 2, e la prova nuova di `probes.browser.test.ts` nel Passo 2 —; la riga
   *Files* del compito le nomina; e le operazioni che il compito aveva — `Overview.vue` (*Crea*), `frame.test.ts`,
   `frame.browser.test.ts`, `probes.ts`, `axe.ts` (*Trova*/*Sostituisci*) — col testo curato. La prova: `python
   docs/superpowers/plans/2026-09-23-design-system-esecuzione/plan_ops.py <piano curato> 8 apply <clone>` su un clone a
   `bc6e94d` pulito (`git -C <clone> checkout --detach --force bc6e94d`, `git -C <clone> clean -fd`), nessun rifiuto, poi
   `git -C <clone> add -A` e `git -C <clone> diff --cached --stat <il tuo commit> -- gui/` **vuoto**: il testo curato rende
   **tutto** ciò che i due commit cambiano sotto `gui/`, byte per byte. Un modello dell'attrezzo che rigenera il testo del
   compito da una copia è `fix8.py` del pre-controllo, in
   `/c/Users/Jays/AppData/Local/Temp/claude/E--ALL-DEV-MY-REPOS-daemon/00051bdc-20ec-4100-bfb2-05d61ad20541/scratchpad/tools/`
   — un modello, non un attrezzo pronto: le cure sono altre.
3. **Le righe nuove della tabella del Passo 9**, una per ogni prova che una cura aggiunge o rafforza: la violazione messa a
   mano e il rosso **misurato sulla suite intera**, come le righe che ci sono.
4. Il **richiamo datato** in **D18** (**E103**).

## 4. Le misure prima del commit

- Sull'**albero** con le cure: `(cd gui && npm test && npm run build && npm run lint)` verdi; la suite intera **tre** volte
  — `npx vitest run --reporter=json --outputFile=<scratchpad>/corsa-N.json` —, una caduta anche **una** volta si riporta
  (P-19); il pezzo JavaScript, `npm run build 2>&1 | grep -E 'assets/index-.*\.js '`, contro `698.14 kB`.
- Sul **clone**, **sulla sequenza dei passi** e col **testo curato**: `plan_ops.py … apply --upto 2` e il Passo 3 — i suoi
  due comandi —, poi `--upto 7` sopra e il Passo 7, poi il Passo 8: se un conto o un rosso cambia rispetto all'Atteso scritto,
  l'Atteso si **corregge** nel testo col numero misurato, e la riga d'errata lo dice.
- Le **righe nuove del Passo 9**, ciascuna sulla suite intera, e la copia che torna.
- Il **cancello** sull'albero, da solo: `GATE GREEN`; `bash scripts/check-docs.sh` → `OK`; e `git status --porcelain` coi
  soli file delle cure e il piano.

## 5. Il commit

**Uno**, sopra `ac56b11`: il messaggio in un file nello scratchpad, `git commit -F <file>`, che comincia con
`design-system(compito 8): le cure della revisione -- ` e dice ogni voce, le misure, il pezzo JavaScript, i fine-riga, il
cancello — la forma del commit `a606f33` (`git log -1 --format=%B a606f33`). ⛔ **Senza co-autore**: `CLAUDE.md` prevale su
qualunque promemoria d'attribuzione. ⛔ **Niente `git push`**, niente `--amend`, niente rebase.

⛔ **Se una cura decisa si rivela sbagliata** — misurata, non creduta —, **non la cambi in silenzio**: ti fermi su quella
voce, la lasci fuori dal commit, e la riporti con l'evidenza. Le altre vanno avanti. Se nessun commit è possibile, rendi
`BLOCKED`.

## 6. Il rapporto

Scrivi `.superpowers/sdd/2026-09-23-design-system/cure-8-report.md`: lo **stato** (`DONE`, `DONE_WITH_CONCERNS`,
`BLOCKED`) e l'**hash**; `git show --stat HEAD`; per ogni voce la cura, il rosso **prima**, il verde, la **seconda
direzione**, coi messaggi veri; le righe nuove del Passo 9; le misure del §4; i fine-riga di ogni file toccato, prima e
dopo; ciò che hai deciso tu e perché; ciò che non hai potuto misurare. Nel messaggio finale: lo stato, l'hash e il percorso
del rapporto, in poche righe.
