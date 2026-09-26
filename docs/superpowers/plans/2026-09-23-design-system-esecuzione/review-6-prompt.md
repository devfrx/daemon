Sei il **revisore del compito 6** — *il dock vestito: il tema nostro, le schede, la presa grande coi pezzi del kit* — del
piano `docs/superpowers/plans/2026-09-23-design-system.md`, nel repository `E:\ALL\DEV\MY_REPOS\daemon` (Windows; il tool
Bash è Git Bash: apri ogni chiamata con `cd /e/ALL/DEV/MY_REPOS/daemon &&`). Sei un subagente fresco e **non hai scritto
nulla** di ciò che rivedi. Rivedi **un commit**:

| Commit | Chi | Che cosa |
|---|---|---|
| `c1102fc` | l'implementatore | il compito 6, sopra `1a83208` |

**All'avvio verifichi:** `git rev-parse --short HEAD` → `c1102fc`; `git status --porcelain` → vuoto;
`git log --oneline 1a83208..HEAD` → **una** riga. Se non torna, ti fermi e lo riporti.

⛔ **Non modifichi nulla nell'albero del repository**: niente scritture, niente commit, niente push. Le prove che
chiedono una mutazione le fai su un **clone**: `git clone E:/ALL/DEV/MY_REPOS/daemon C:/Users/Jays/AppData/Local/Temp/rv6`,
poi `git -C C:/Users/Jays/AppData/Local/Temp/rv6 checkout --detach c1102fc` e `(cd /c/Users/Jays/AppData/Local/Temp/rv6/gui && npm ci)`.
Il clone sta in `%TEMP%` e **non** nello scratchpad: su questa macchina `LongPathsEnabled` è 0, e i percorsi di
`node_modules` sotto lo scratchpad passerebbero i 260 caratteri. Lo lasci dov'è: lo cancella il coordinatore. Lo
scratchpad per i log e per i tuoi file è
`C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\d5484876-d48f-457d-8568-4ed6a1b75b37\scratchpad\review6`
(in Git Bash `/c/Users/Jays/AppData/Local/Temp/claude/E--ALL-DEV-MY-REPOS-daemon/d5484876-d48f-457d-8568-4ed6a1b75b37/scratchpad/review6`).
Niente subagenti.

⛔ **Una cosa alla volta**: il cancello e un `npm`/`vitest` sullo **stesso** albero si pestano su `gui/node_modules`
(gotcha #133), e due suite insieme sulla stessa macchina rendono instabili le prove col tempo (P-20, P-21 del piano). Su
questa macchina la memoria è 31,2 GB e le app del proprietario ne tengono gran parte — 14,4 GB liberi, misurati dal coordinatore il 2026-09-26 alle 18:22, prima del dispaccio —: mai due suite, mai due cancelli, mai un cancello mentre nel clone gira una suite o un
server di sviluppo. Il cancello gira sull'**albero del repository**, in background verso un log nuovo nello scratchpad, e
aspetti la notifica; le mutazioni girano **nel clone**. ⛔ Il browser delle prove è il **Chrome installato**, senza
finestra: se Playwright dice che manca, ti fermi e lo riporti — niente `npx playwright install`, niente download (decisione
22 del disegno). ⚠️ **Chrome si aggiorna da sé** (lezione 5 della consegna dell'esecuzione del compito 3): un rosso del
progetto `browser` che non nomina una prova — `Target page, context or browser has been closed`, `Failed to connect to the
browser session` — si rilancia dopo aver riletto la versione dal **nome della cartella**,
`ls "/c/Program Files/Google/Chrome/Application/"`. ⛔ **Mai `chrome.exe --version`**: su Windows apre il browser col
profilo dell'utente e non stampa nulla (lezione 4).

La macchina, misurata dal coordinatore il 2026-09-26: `core.autocrlf` `false` in `.git/config` — `--get-all` rende `true`
dal file di sistema, `false` da `C:/Users/Jays/.gitconfig` e `false` da `.git/config`, e vale l'ultima —, quindi l'albero
è `w/lf`, e i file nuovi nascono `w/lf`; un clone fresco prende il `false` del file dell'utente, e il suo albero è `w/lf`
fuori dai quattro file che sono CRLF già nell'indice, tutti sotto `crates/` (`git ls-files --eol | grep i/crlf`). Node
v24.19.0; Chrome `154.0.8037.58`, letto dal nome della cartella. Il cancello d'apertura del coordinatore, a `1a83208`:
`GATE GREEN`, sotto `gui/` il progetto `jsdom` con 18 file passati e uno saltato, 132 prove passate e una saltata, il
progetto `browser` con 4 file e 28 prove, il pezzo JavaScript `689.65 kB` (`index-COUloyjC.js`), `found 0
vulnerabilities`; il suo log è `gate-baseline.log` nello scratchpad padre di `review6`, e **non** è quello
dell'implementatore.

## 1. Che cosa leggi

Nella cartella `.superpowers/sdd/2026-09-23-design-system/`:

| File | Che cos'è |
|---|---|
| `task-6-brief.md` | **il compito come era dettato**, parola per parola dal piano e dal disegno a `1a83208`: la testa con gli *Strumenti*, i vincoli globali, l'errata (**E44**–**E48** sono del compito 6, già applicate al testo; **E43** è la riga di `Frame.vue` che il suo Passo 4 tiene; **E29**, **E30** ed **E33** sono della sonda dei raggi che il compito corregge), le voci P-6, P-7 e P-15…P-18, le decisioni D9…D11, le voci aperte che il piano sa, il compito 6 intero, e dal disegno le risposte 4, 5, 18 e 20, le sezioni (c) e (f), *«Cosa questo disegno ha misurato»*, i controlli 15 e 16, le trappole 5 e 9 e la decisione 23 del coordinatore. Leggilo **tutto**, a blocchi: il suo peso lo dà `wc -c -l` |
| `dispatch-task-6.md` | il prompt che l'implementatore ha ricevuto — il **contratto** contro cui lo giudichi |
| `task-6-report.md` | il **rapporto dell'implementatore**: è una dichiarazione, non un fatto |

E nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`: `compare_task6.py`, lo script del
pre-controllo che ricostruisce dal **testo del piano** ogni file dettato — la ricetta sta nel *«Come si riprende»* del
piano, sotto *«La ricetta del compito 6»* — e lo confronta col commit. ⚠️ **È anch'esso una dichiarazione**: lo leggi, e
lo provi.

Poi `git show c1102fc`. ⛔ **Non leggi** il piano intero (oltre 9 200 righe), il disegno intero, `docs/HANDOFF.md`,
`docs/adr/`, `docs/archivio/`, l'audit: dove ti serve, `grep -n` e `sed -n` nel punto esatto. Le regole di `CLAUDE.md`
valgono per te: codice in inglese e documenti in italiano, nessuna cifra senza il suo comando, i fine-riga per file.

📌 **Un confronto in più, se ti serve:** la copia del pre-controllo `%TEMP%\pc6b` è il compito rifatto dal testo corretto
del piano, non committato. Non la modifichi.

## 2. Che cosa verifichi

1. **Ogni affermazione misurabile del rapporto si RILANCIA** — il comando accanto, la tua uscita accanto alla sua
   (regola 5 di *«Come si esegue un compito»*). Elenca i comandi rilanciati. Un numero che non torna è un rilievo. I log
   del cancello che il rapporto cita devono essere **dell'implementatore**: `ls -la --time-style=full-iso <log>` contro
   `git log -1 --format=%ci c1102fc`.
2. **La conformità al dettato.** `python docs/superpowers/plans/2026-09-23-design-system-esecuzione/compare_task6.py 1a83208 c1102fc`
   dalla radice del repository, con `PYTHONIOENCODING=utf-8`. Ma prima **provalo nelle due direzioni**: leggilo e di' se
   ricostruisce davvero ciò che il compito detta — le due forme della ricetta, `W` e `R`, e le due celle del piano —; poi
   nel **clone** fai un commit che cambia **un** carattere di **un** file dettato — in un commento o in una stringa di un
   file riscritto — e lancia lo script, dal clone, contro quel commit: deve dire `DIFFERS` su quel file **e su nessun
   altro**, ed uscire 1. Ciò che lo script lascia `CHECK BY HAND` lo guardi tu: nel piano cambiano **solo** la colonna
   Commit della riga **5** della tabella della posizione — `` `545f500`, con le cure `7e25d03` e `9e6657b` `` — e lo Stato
   della riga **6**, `✅ 2026-09-26`.
3. **I rossi dei Passi 2, 3 e 5, riprodotti** nel clone a `c1102fc`, un passo per volta; prima di ciascuno una copia salvata
   di ogni file che tocchi, dopo ciascuno i file tornano dalle copie e `cmp` lo conferma. ⚠️ **Un Atteso si misura sulla
   sequenza dei passi, non su una corsa pulita** — lezione 1 del pre-controllo del compito 4.
   - **Passo 2**: `gui/src/tokens/readToken.ts` tolto, e il comando del Passo 2 rosso con
     `Failed to import test file …/tokens.browser.test.ts`; il file tornato, verde.
   - **Passo 3**: lo stato del Passo 3 — le quattro prove che il passo scrive (`gui/src/tokens/dock.test.ts`,
     `gui/src/frame/dock.browser.test.ts`, `gui/src/frame/frame.test.ts`, `gui/src/testing/probes.browser.test.ts`) e i due
     file del Passo 2 come sono nel commit; i file dei Passi 4 e 5 com'erano a `1a83208` (`git -C <clone> show
     1a83208:<file>`) — `gui/src/tokens/dock.css`, `gui/src/frame/dock.ts`, `gui/src/frame/Frame.vue`,
     `gui/src/jsdom-setup.ts`, `gui/src/testing/probes.ts`, `gui/src/frame/BigTab.ts`, `gui/src/frame/bigtab.test.ts`,
     `gui/eslint.config.js` —; e `gui/src/frame/BigTabFace.vue` tolto. I due comandi del Passo 3: i rossi contro l'Atteso e
     contro il rapporto, **per nome** di prova e nei due temi; e verde la pillola della sonda.
   - **Passo 5**: `gui/src/frame/bigtab.test.ts` com'è nel commit, `gui/src/frame/BigTab.ts` com'era a `1a83208`,
     `gui/src/frame/BigTabFace.vue` tolto: il comando del Passo 5 rosso con le **tre** prove dell'Atteso.
4. **Le venti violazioni del Passo 7**, nel clone, una alla volta, com'è scritto nel Passo 7: ciascuna rossa **per la
   ragione scritta** — il messaggio vero, non «rosso» — e con le prove che cadono **quelle scritte**: nei due temi dove la
   riga lo dice, nel solo scuro per il fondo delle linguette (**E47**), e per ciò che scorre la **sola** prova della sonda.
   Indietro con la copia salvata (vincolo 11), `git -C <clone> status --porcelain` vuoto alla fine. ⚠️ **Il rapporto
   porta una voce candidata, E49**: due righe — *«la distanza»* e *«la riduzione»* — fanno cadere più prove di quelle
   che l'Atteso nomina. La giudichi con la tua corsa: le prove in più cadono davvero, e per la ragione che il rapporto
   dice? Il testo che propone è giusto e basta?
5. **Guardarla** — punto 5 di *«Come si esegue un compito»*: un verde non prova che una cosa si veda. Nel clone,
   `npm run dev` in background, poi la SPA nel **Chrome installato**, a 1440 × 900, ⛔ **con le barre di scorrimento
   accese**: uno script Playwright nello scratchpad — `playwright` preso dal `node_modules` del clone, per esempio con
   `createRequire`; `chromium.launch({ channel: "chrome", ignoreDefaultArgs: ["--hide-scrollbars"] })`; nessun download —,
   oppure gli strumenti del browser integrato (`mcp__Claude_Browser__*`), se li hai. Senza quell'argomento Playwright
   spegne le barre, e ciò che il Passo 8 chiede di guardare non si vede (**E48**). Il core finto si guida dalla pagina con
   `harnessFake.deliverAll()`, come nel Passo 8. Nei **due temi** — `document.documentElement.dataset.theme` a `"light"` e
   a `"dark"` —: le **due misure** del Passo 8, il frammento del contrasto e `elementFromPoint` col cassetto aperto sopra il
   gruppo staccato, contro l'Atteso; e uno screenshot dopo ciascuna delle cose che il Passo 8 nomina — le schede, le
   linguette col segno, i divisori al passaggio, la zona d'arrivo trascinando una linguetta; il gruppo staccato, raggio e
   fondo della sua linguetta compresi; la presa grande coi suoi due pulsanti; le barre di scorrimento dei pannelli che
   scorrono, Stato e Impostazioni nella Home, all'angolo tondo della scheda. Gli screenshot li **guardi**, uno per uno, e
   dici che cosa vedi: il contrasto, il focus visibile, testo tagliato, cose che sbordano o si sovrappongono. ⛔
   **L'aspetto non lo giudichi tu**: lo giudica il proprietario (controllo 15, **D24**) — tu descrivi, e riporti ciò che è
   rotto. Poi **fermi** il server di sviluppo per PID — chiudere la shell non basta, il suo `node` resta in ascolto — prima
   di qualunque suite o cancello, e controlli che nessun processo `node.exe` rimanga acceso per mano tua.
6. **Il cancello intero**, una volta, sull'**albero del repository**: `GATE GREEN`; le righe `Test Files` e `Tests` dei due
   progetti, contro il rapporto e contro la base — che nessuna delle prove di prima sia sparita, per **nome di file** e
   non solo per conto —; `found 0 vulnerabilities`; la riga del pezzo JavaScript, contro la base: qui **cresce**, com'è
   previsto, e la cifra va al proprietario (N-2). E `bash scripts/check-docs.sh` → `OK`. ⚠️ Una prova che cade anche
   **una** volta in una corsa che non la muta si riporta con la sua uscita (P-19, D23): non si rilancia finché passa.
7. **Il codice, oltre il dettato.** Il codice è dettato dal piano, quindi un difetto del dettato **non si cura**: è una
   **voce d'errata candidata** — la prossima libera è **E50**, perché **E49** è la candidata del rapporto dell'implementatore — col testo corretto proposto. Guarda soprattutto:
   **(a)** ogni riga che porta il suo perché in un commento — tolta, quale prova cade?; **(b)** ogni commento che dice
   *«every»*, *«all»* o *«only»* su qualcosa che il compito fa crescere, e il commento che il compito tocca in
   `gui/eslint.config.js`; **(c)** le **interfacce** che il compito produce, la lista *Produces* del compito — `readToken`
   con l'errore per un token che la pagina non definisce, `harnessTheme`, la classe `dockview-theme-harness`, le props e
   gli eventi di `BigTabFace.vue`, `base.css` sotto jsdom da `src/jsdom-setup.ts` (D9), la sonda `concentricRadii` —; e
   che dopo questo commit in `panels/` e `frame/` nessun pulsante nasca più con `document.createElement` (R3-16, trappola
   5); **(d)** che nessun commento **dica il falso** dopo questo commit (gotcha #58): il ponte di `gui/src/tokens/dock.css`
   (**E36**) esce con la riscrittura; ogni riga di `gui/` che nomina ancora `themeAbyss`, il ponte o il compito 6 come
   futuro — `grep -rn` sulle parole, e ogni riga che il censimento restituisce si legge **intera**. ⛔ **Un difetto
   che il coordinatore ha già misurato, e che giudichi:** il controllo 15 del disegno dice *«`themeAbyss` non compare
   più nel sorgente»* (la riga `| 15 |` della tabella dei controlli del prodotto, in
   `docs/superpowers/specs/2026-09-22-design-system-design.md`), e la *Definizione di «fatto»* del piano lo traduce in
   `grep -rn 'themeAbyss' gui/src | wc -l` con l'attesa `0`; dopo questo commit il comando rende **quattro** righe, tutte
   commenti che il compito 6 detta. Il rapporto lo osserva, alla sua §8. Proponi la cura come voce d'errata candidata,
   con le strade che vedi e il costo di ciascuna: la scelta è del coordinatore, e se tocca il disegno del proprietario; **(e)** i vincoli
   **4** (le parole solo da `it.json`), **5** (nessun colore a mano in `gui/src/**/*.vue` e in `tokens/dock.css`) e **6**
   (nessun `var(--ref-` fuori da `gui/src/tokens/`); **(f)** la sonda corretta (**E44**, **E46**) resta giusta per chi la
   usava prima: le prove della pagina kit del compito 4 nel browser, verdi, e ciò che la regola nuova dello scorrimento
   toglie al loro giudizio, se qualcosa.
8. **I vincoli globali** del brief — soprattutto il **7** (nessuna dipendenza nuova: `gui/package.json` e il lockfile non
   cambiano), l'**8**, il **9** (fine-riga: `git ls-files --eol` e i CR, prima e dopo, per ogni file toccato; i file nuovi
   LF), l'**11** (le due direzioni, e la guardia di non-vacuità di ogni prova del browser), il **12**
   (`git diff --stat 1a83208..c1102fc -- crates/ gui/schema/` vuoto).
9. **Il contratto** del prompt dell'implementatore: un commit, i soli file del suo punto 2, le due celle del piano, il
   messaggio che comincia con `design-system(compito 6): ` e porta il pezzo JavaScript prima e dopo, niente push
   (`git status -sb` → `ahead 1`), niente co-autore (`git log -1 --format=%B c1102fc | grep -ci co-authored` → 0).

## 3. Il rapporto

Scrivi `.superpowers/sdd/2026-09-23-design-system/task-6-review.md`:

1. il **verdetto**, in una riga: conforme o no;
2. i **rilievi**, classificati **Critico** (rende falso ciò che il compito consegna, o rompe un Atteso o un criterio),
   **Importante** (una regola del repository violata, o un'affermazione del rapporto non provata), **Minore**, **Nit** —
   ciascuno col file, la riga ritrovata col `grep` sulla frase, l'evidenza (comando e uscita) e la cura proposta **in
   forma di testo**, senza applicarla; un difetto del **dettato** come voce d'errata candidata;
3. l'elenco dei **comandi rilanciati**, con le uscite;
4. che cosa hai **visto** guardando la SPA, tema per tema, coi percorsi degli screenshot, e le due misure del Passo 8;
5. ciò che **non** hai potuto verificare, e perché.

Nel messaggio finale al coordinatore: il verdetto, il conto dei rilievi per classe, il percorso del rapporto — in poche
righe.
