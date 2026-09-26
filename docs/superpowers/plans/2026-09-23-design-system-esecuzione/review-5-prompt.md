Sei il **revisore del compito 5** — *il kit al lavoro: i pezzi di base nei pannelli e nella cornice* — del piano
`docs/superpowers/plans/2026-09-23-design-system.md`, nel repository `E:\ALL\DEV\MY_REPOS\daemon` (Windows; il tool
Bash è Git Bash: apri ogni chiamata con `cd /e/ALL/DEV/MY_REPOS/daemon &&`). Sei un subagente fresco e **non hai scritto
nulla** di ciò che rivedi. Rivedi **un commit**:

| Commit | Chi | Che cosa |
|---|---|---|
| `545f500` | l'implementatore | il compito 5, sopra `39827e8` |

**All'avvio verifichi:** `git rev-parse --short HEAD` → `545f500`; `git status --porcelain` → vuoto;
`git log --oneline 39827e8..HEAD` → **una** riga. Se non torna, ti fermi e lo riporti.

⛔ **Non modifichi nulla nell'albero del repository**: niente scritture, niente commit, niente push. Le prove che
chiedono una mutazione le fai su un **clone**: `git clone E:/ALL/DEV/MY_REPOS/daemon C:/Users/Jays/AppData/Local/Temp/rv5`,
poi `git -C C:/Users/Jays/AppData/Local/Temp/rv5 checkout --detach 545f500` e `(cd /c/Users/Jays/AppData/Local/Temp/rv5/gui && npm ci)`.
Il clone sta in `%TEMP%` e **non** nello scratchpad: su questa macchina `LongPathsEnabled` è 0, e i percorsi di
`node_modules` sotto lo scratchpad passerebbero i 260 caratteri. Lo lasci dov'è: lo cancella il coordinatore. Lo
scratchpad per i log e per i tuoi file è
`C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\b62a978b-28b5-4b69-963c-b58323e34338\scratchpad\review5`
(in Git Bash `/c/Users/Jays/AppData/Local/Temp/claude/E--ALL-DEV-MY-REPOS-daemon/b62a978b-28b5-4b69-963c-b58323e34338/scratchpad/review5`).
Niente subagenti.

⛔ **Una cosa alla volta**: il cancello e un `npm`/`vitest` sullo **stesso** albero si pestano su `gui/node_modules`
(gotcha #133), e due suite insieme sulla stessa macchina rendono instabili le prove col tempo (P-20, P-21 del piano). Su
questa macchina la memoria è 31,2 GB e le app del proprietario ne tengono gran parte — 4,9 GB liberi, misurati dal
coordinatore mentre girava l'implementatore —: mai due suite, mai due cancelli, mai un cancello mentre nel clone gira una
suite o un server di sviluppo. Il cancello gira sull'**albero del repository**, in background verso un log nuovo nello
scratchpad, e aspetti la notifica; le mutazioni girano **nel clone**. ⛔ Il browser delle prove è il **Chrome installato**,
senza finestra: se Playwright dice che manca, ti fermi e lo riporti — niente `npx playwright install`, niente download
(decisione 22 del disegno). ⚠️ **Chrome si aggiorna da sé** (lezione 5 della consegna dell'esecuzione del compito 3): un
rosso del progetto `browser` che non nomina una prova — `Target page, context or browser has been closed`, `Failed to
connect to the browser session` — si rilancia dopo aver riletto la versione dal **nome della cartella**,
`ls "/c/Program Files/Google/Chrome/Application/"`. ⛔ **Mai `chrome.exe --version`**: su Windows apre il browser col
profilo dell'utente e non stampa nulla (lezione 4).

La macchina, misurata dal coordinatore il 2026-09-26: `core.autocrlf` `false` in `.git/config` — `--get-all` rende `true`
dal file di sistema, `false` da `C:/Users/Jays/.gitconfig` e `false` da `.git/config`, e vale l'ultima —, quindi l'albero
è `w/lf`, e i file nuovi nascono `w/lf`; un clone fresco prende il `false` del file dell'utente, e il suo albero è `w/lf`
fuori dai quattro file che sono CRLF già nell'indice, tutti sotto `crates/` (`git ls-files --eol | grep w/crlf`). Node
v24.19.0; Chrome `154.0.8037.58`, letto dal nome della cartella. Il cancello d'apertura del coordinatore, a `39827e8`:
`GATE GREEN`, sotto `gui/` il progetto `jsdom` con 18 file passati e uno saltato, 125 prove passate e una saltata, il
progetto `browser` con 3 file e 27 prove, il pezzo JavaScript `663.93 kB` (`index-CiZv4zPX.js`), `found 0
vulnerabilities`; il suo log è `gate-baseline.log` nello scratchpad padre di `review5`, e **non** è quello
dell'implementatore.

## 1. Che cosa leggi

Nella cartella `.superpowers/sdd/2026-09-23-design-system/`:

| File | Che cos'è |
|---|---|
| `task-5-brief.md` | **il compito come era dettato**, parola per parola dal piano e dal disegno a `39827e8`: la testa con gli *Strumenti*, i vincoli globali, l'errata (**E34**–**E37** sono del compito 5, già applicate al testo; **E2** ed **E4** sono le cure del compito 1 su cui si appoggia), le voci P-3, P-8, P-19 e P-20, le voci aperte che il piano sa, il compito 5 intero, e dal disegno le risposte 5, 16, 17 e 21, *«I due temi»* della (a), i pezzi di base, `BaseStatus` e le regole del linter della (b), la (e), i controlli 11, 12 e 13, le trappole 5 e 10 e la decisione 21 del coordinatore. Leggilo **tutto**, a blocchi: il suo peso lo dà `wc -c -l` |
| `dispatch-task-5.md` | il prompt che l'implementatore ha ricevuto — il **contratto** contro cui lo giudichi |
| `task-5-report.md` | il **rapporto dell'implementatore**: è una dichiarazione, non un fatto |

E nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`: `compare_task5.py`, lo script del
pre-controllo che ricostruisce dal **testo del piano** ogni file dettato — la ricetta sta nel *«Come si riprende»* del
piano, sotto *«La ricetta del compito 5»* — e lo confronta col commit. ⚠️ **È anch'esso una dichiarazione**: lo leggi, e
lo provi.

Poi `git show 545f500`. ⛔ **Non leggi** il piano intero (oltre 8 900 righe), il disegno intero, `docs/HANDOFF.md`,
`docs/adr/`, `docs/archivio/`, l'audit: dove ti serve, `grep -n` e `sed -n` nel punto esatto. Le regole di `CLAUDE.md`
valgono per te: codice in inglese e documenti in italiano, nessuna cifra senza il suo comando, i fine-riga per file.

📌 **Un confronto in più, se ti serve:** la copia del pre-controllo `%TEMP%\pc5b` è il compito rifatto dal testo corretto
del piano, e i suoi diciassette file sono uguali, byte per byte, a quelli che `compare_task5.py` ricostruisce — misurato
dal pre-controllo sulla copia `%TEMP%\pc5c`. Non le modifichi.

## 2. Che cosa verifichi

1. **Ogni affermazione misurabile del rapporto si RILANCIA** — il comando accanto, la tua uscita accanto alla sua
   (regola 5 di *«Come si esegue un compito»*). Elenca i comandi rilanciati. Un numero che non torna è un rilievo. I log
   del cancello che il rapporto cita devono essere **dell'implementatore**: `ls -la --time-style=full-iso <log>` contro
   `git log -1 --format=%ci 545f500`.
2. **La conformità al dettato.** `python docs/superpowers/plans/2026-09-23-design-system-esecuzione/compare_task5.py 39827e8 545f500`
   dalla radice del repository, con `PYTHONIOENCODING=utf-8`. Ma prima **provalo nelle due direzioni**: leggilo e di' se
   ricostruisce davvero ciò che il compito detta — le cinque forme della ricetta, `W`, `R`, `RL`, `X` e `S`, e la cella del
   piano —; poi nel **clone** fai un commit che cambia **un** carattere di **un** file dettato — in un commento o in una
   stringa di un componente riscritto — e lancia lo script, dal clone, contro quel commit: deve dire `DIFFERS` su quel file
   **e su nessun altro**, ed uscire 1. Ciò che lo script lascia `CHECK BY HAND` lo guardi tu: nel piano cambia **solo** la
   colonna Commit della riga **4** della tabella della posizione — `` `841dc54`, con la cura `8d098d0` `` —, e la riga
   **5** resta `⬜`: la porta a ✅ il coordinatore, col verbale del Passo 8 (**E35**).
3. **I rossi del Passo 1, riprodotti.** Nel clone a `545f500`, lo stato del Passo 1: le quattro prove che il Passo 1 tocca
   — `gui/src/panels/modules.test.ts`, `gui/src/a11y.test.ts`, `gui/src/frame/frame.test.ts` e
   `gui/src/panels/settings.browser.test.ts` — come sono nel commit, e ciascuno degli altri tredici file del compito com'era
   a `39827e8` (`git -C <clone> show 39827e8:<file>`), salvato prima in una copia. Lancia le prove come le lancia il Passo
   1: i rossi contro l'Atteso e contro il rapporto — sotto jsdom le prove di Impostazioni e le tre di M-3, queste con
   `Unable to get [role="status"]`; verdi le prove della finestra di conferma; nel browser rossa la prova della tastiera —,
   per **nome** di prova. Poi i file tornano dalle copie, e `cmp` lo conferma. ⚠️ **Un Atteso si misura sulla sequenza dei
   passi, non su una corsa pulita** — lezione 1 del pre-controllo del compito 4.
4. **Il Passo 2**, nel clone: `gui/src/locales/it.json` com'era a `39827e8` e la prova di `copy.test.ts` rossa, `system:
   expected [] to include 'system'`; tornato il file, verde; e la direzione rossa, la scelta `sepia` in
   `gui/src/tokens/theme.ts` com'è scritta nel Passo 2. Copie e `cmp`, come sopra.
5. **La direzione rossa di M-3 del Passo 6**, nel clone, **un pannello per volta** — `gui/src/frame/Band.vue`,
   `gui/src/panels/Settings.vue`, `gui/src/panels/Status.vue` —: `BaseStatus` rimesso dentro il `v-if`, e deve cadere la
   **sola** prova di M-3 di quel pannello, `Unable to get [role="status"]`. Copie e `cmp` dopo ciascuna.
6. **Le violazioni del Passo 7**, nel clone, una alla volta, com'è scritto nel Passo 7: ciascuna rossa **per la ragione
   scritta** — il messaggio vero, non «rosso» — e con **un errore solo** (**E34**: una violazione con parole dentro ne
   rompe due); la riga 4 verde; indietro con la copia salvata (vincolo 11), `git -C <clone> status --porcelain` vuoto alla
   fine. E il **confine** del blocco nuovo di `gui/eslint.config.js`: la testa delle regole d'import vuole che ogni blocco
   dica la lista intera, quindi un import di `lucide` — il nome del pacchetto lo dice `gui/package.json` — in un file di
   `panels/` deve restare rosso col messaggio di prima; e nulla deve scattare in `components/`.
7. **Guardarla** — punto 5 di *«Come si esegue un compito»*: un verde non prova che una cosa si veda. Nel clone,
   `npm run dev` in background, poi la SPA nel **Chrome installato**, a grandezza vera. Il modo: uno script Playwright
   nello scratchpad — `playwright` preso dal `node_modules` del clone, per esempio con `createRequire`;
   `chromium.launch({ channel: "chrome" })`; nessun download —, oppure gli strumenti del browser integrato
   (`mcp__Claude_Browser__*`), se li hai. Il core finto si guida dalla pagina con `harnessFake.deliver(…)`, come nel Passo
   8. Nei **due temi** — la scelta del tema è nuova, in Impostazioni —, uno screenshot dopo ciascun passo: la fascia al
   caricamento, e dopo `harnessFake.deliver("Accepted")` che se ne va; la barra coi pezzi del kit; il cassetto `sheet` che
   si apre e si chiude, col fuoco che torna al pulsante che l'ha aperto; la finestra di conferma col fuoco dentro, ed Esc
   che rifiuta senza mandare niente; Impostazioni coi due gruppi, e la tastiera sui radio, con le frecce; Permessi, Passi e
   il segnaposto. ⚠️ **Nessuna prova apre il cassetto né guarda il pulsante di vista della barra**: è registrato dal
   pre-controllo e non preso, perché il compito 8 li riscrive entrambi con le loro prove — qui li **guardi**, e non proponi
   una prova. Gli screenshot li **guardi**, uno per uno, e dici che cosa vedi: il contrasto, il focus visibile, testo
   tagliato, cose che sbordano, la radice che cambia tema. Poi **fermi** il server di sviluppo per PID — chiudere la shell
   non basta, il suo `node` resta in ascolto — prima di qualunque suite o cancello, e controlli che nessun processo
   `node.exe` rimanga acceso per mano tua. ⛔ **Il Passo 8 non è tuo** (**E35**): l'Assistente vocale lo sente il
   proprietario, col coordinatore, dopo la revisione.
8. **Il cancello intero**, una volta, sull'**albero del repository**: `GATE GREEN`; le righe `Test Files` e `Tests` dei due
   progetti, contro il rapporto e contro la base — che nessuna delle prove di prima sia sparita, per **nome di file** e
   non solo per conto —; `found 0 vulnerabilities`; la riga del pezzo JavaScript, contro la base: qui **cresce**, com'è
   previsto, e la cifra va al proprietario (N-2). E `bash scripts/check-docs.sh` → `OK`.
9. **Il codice, oltre il dettato.** Il codice è dettato dal piano, quindi un difetto del dettato **non si cura**: è una
   **voce d'errata candidata** — la prossima libera è **E38** — col testo corretto proposto. Guarda soprattutto:
   **(a)** ogni riga di un componente riscritto che porta il suo perché in un commento — tolta, quale prova cade?;
   **(b)** ogni commento che dice *«ogni»* o *«tutti»* su una cartella che il compito fa crescere, e ogni blocco del
   linter provato sul suo **confine** (**E26**); **(c)** le **interfacce** che il compito produce, la lista *Produces* del
   compito: nessun `<button>`, `<ul>` od `<ol>` scritti a mano nei template di `panels/` e `frame/`, e nessun import di
   `reka-ui` o di `lucide` in quelle due cartelle — **tranne** i due pulsanti che `frame/BigTab.ts` crea con
   `document.createElement` (R3-16, trappola 5), che toglie il compito 6 —; in `it.json` le chiavi `settings.themeTitle` e
   `settings.theme.system`, `.light`, `.dark`; **(d)** che nessun commento **dica il falso** dopo questo commit (gotcha
   #58) — il ponte di `gui/src/tokens/dock.css` (**E36**) e il blocco `harness/ts` di `gui/eslint.config.js`, che promette
   `reka-ui` *«from task 5»*; il commento del tema in `gui/src/frame/dock.ts` lo toglie il compito 6, ed è atteso —;
   **(e)** i vincoli **4** (le parole solo da `it.json`: `no-raw-text` a `error` fuori dalla pagina kit), **5** (nessun
   colore a mano in `gui/src/**/*.vue` e in `tokens/dock.css`) e **6** (nessun `var(--ref-` fuori da `gui/src/tokens/`).
10. **I vincoli globali** del brief — soprattutto il **7** (nessuna dipendenza nuova: `gui/package.json` e il lockfile non
   cambiano), l'**8**, il **9** (fine-riga: `git ls-files --eol` e i CR, prima e dopo, per ogni file toccato; il file
   nuovo LF), l'**11** (le due direzioni, e la guardia di non-vacuità di `settings.browser.test.ts` col segno
   `⛔ NON-VACUITY`, **E37**), il **12** (`git diff --stat 39827e8..545f500 -- crates/ gui/schema/` vuoto).
11. **Il contratto** del prompt dell'implementatore: un commit, i soli file del suo punto 2, la cella della riga 4, la
   riga 5 a `⬜`, il messaggio che comincia con `design-system(compito 5): ` e porta il pezzo JavaScript prima e dopo,
   niente push (`git status -sb` → `ahead 1`), niente co-autore (`git log -1 --format=%B 545f500 | grep -ci co-authored`
   → 0).

## 3. Il rapporto

Scrivi `.superpowers/sdd/2026-09-23-design-system/task-5-review.md`:

1. il **verdetto**, in una riga: conforme o no;
2. i **rilievi**, classificati **Critico** (rende falso ciò che il compito consegna, o rompe un Atteso o un criterio),
   **Importante** (una regola del repository violata, o un'affermazione del rapporto non provata), **Minore**, **Nit** —
   ciascuno col file, la riga ritrovata col `grep` sulla frase, l'evidenza (comando e uscita) e la cura proposta **in
   forma di testo**, senza applicarla; un difetto del **dettato** come voce d'errata candidata;
3. l'elenco dei **comandi rilanciati**, con le uscite;
4. che cosa hai **visto** guardando la SPA, tema per tema, coi percorsi degli screenshot;
5. ciò che **non** hai potuto verificare, e perché.

Nel messaggio finale al coordinatore: il verdetto, il conto dei rilievi per classe, il percorso del rapporto — in poche
righe.
