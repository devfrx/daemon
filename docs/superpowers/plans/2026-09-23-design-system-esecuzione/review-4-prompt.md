Sei il **revisore del compito 4** — *la pagina kit: fuori dal pacchetto, e le sonde diventano prove nel browser* — del
piano `docs/superpowers/plans/2026-09-23-design-system.md`, nel repository `E:\ALL\DEV\MY_REPOS\daemon` (Windows; il tool
Bash è Git Bash: apri ogni chiamata con `cd /e/ALL/DEV/MY_REPOS/daemon &&`). Sei un subagente fresco e **non hai scritto
nulla** di ciò che rivedi. Rivedi **un commit**:

| Commit | Chi | Che cosa |
|---|---|---|
| `841dc54` | l'implementatore | il compito 4, sopra `1b531e9` |

**All'avvio verifichi:** `git rev-parse --short HEAD` → `841dc54`; `git status --porcelain` → vuoto;
`git log --oneline 1b531e9..HEAD` → **una** riga. Se non torna, ti fermi e lo riporti.

⛔ **Non modifichi nulla nell'albero del repository**: niente scritture, niente commit, niente push. Le prove che
chiedono una mutazione le fai su un **clone**: `git clone E:/ALL/DEV/MY_REPOS/daemon C:/Users/Jays/AppData/Local/Temp/rv4`,
poi `git -C C:/Users/Jays/AppData/Local/Temp/rv4 checkout --detach 841dc54` e `(cd /c/Users/Jays/AppData/Local/Temp/rv4/gui && npm ci)`.
Il clone sta in `%TEMP%` e **non** nello scratchpad: su questa macchina `LongPathsEnabled` è 0, e i percorsi di
`node_modules` sotto lo scratchpad passerebbero i 260 caratteri. Lo lasci dov'è: lo cancella il coordinatore. Lo
scratchpad per i log e per i tuoi file è
`C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\85fa78d3-ac8f-4098-9b47-eaa6f1c6ea46\scratchpad\review4`
(in Git Bash `/c/Users/Jays/AppData/Local/Temp/claude/E--ALL-DEV-MY-REPOS-daemon/85fa78d3-ac8f-4098-9b47-eaa6f1c6ea46/scratchpad/review4`).
Niente subagenti.

⛔ **Una cosa alla volta**: il cancello e un `npm`/`vitest` sullo **stesso** albero si pestano su `gui/node_modules`
(gotcha #133), e due suite insieme sulla stessa macchina rendono instabili le prove col tempo (P-20, P-21 del piano). Su
questa macchina la memoria è 31,2 GB e le app del proprietario ne tengono gran parte — 4,6 GB liberi, misurati dal
coordinatore mentre girava l'implementatore —: mai due suite, mai due cancelli, mai un cancello mentre nel clone gira una
suite o un server di sviluppo. Il cancello gira sull'**albero del repository**, in background verso un log nuovo nello
scratchpad, e aspetti la notifica; le mutazioni girano **nel clone**. ⛔ Il browser delle prove è il **Chrome installato**,
senza finestra: se Playwright dice che manca, ti fermi e lo riporti — niente `npx playwright install`, niente download
(decisione 22 del disegno). ⚠️ **Chrome si aggiorna da sé** (lezione 5 della consegna dell'esecuzione del compito 3): un
rosso del progetto `browser` che non nomina una prova — `Target page, context or browser has been closed`, `Failed to
connect to the browser session` — si rilancia dopo aver riletto la versione dal **nome della cartella**,
`ls "/c/Program Files/Google/Chrome/Application/"`. ⛔ **Mai `chrome.exe --version`**: su Windows apre il browser col
profilo dell'utente e non stampa nulla (lezione 4).

La macchina, misurata dal coordinatore il 2026-09-25: `core.autocrlf` `false` in `.git/config` — `--get-all` rende `true`
dal file di sistema, `false` da `C:/Users/Jays/.gitconfig` e `false` da `.git/config`, e vale l'ultima —, quindi l'albero
è `w/lf`, e i file nuovi nascono `w/lf`; un clone fresco prende il `false` del file dell'utente — misurato dal
coordinatore con un clone di prova —, e il suo albero è `w/lf` fuori dai quattro file che sono CRLF già nell'indice, tutti
sotto `crates/` (`git ls-files --eol | grep w/crlf`). Node v24.19.0; Chrome `154.0.8037.58`, letto dal nome della cartella. Il cancello
d'apertura del coordinatore, a `1b531e9`: `GATE GREEN`, sotto `gui/` il progetto `jsdom` con 18 file passati e uno
saltato, 125 prove passate e una saltata, il progetto `browser` con un file e 5 prove, il pezzo JavaScript `663.93 kB`
(`index-CiZv4zPX.js`), `found 0 vulnerabilities`; il suo log è `gate-baseline.log` nello scratchpad padre di `review4`, e
**non** è quello dell'implementatore.

## 1. Che cosa leggi

Nella cartella `.superpowers/sdd/2026-09-23-design-system/`:

| File | Che cos'è |
|---|---|
| `task-4-brief.md` | **il compito come era dettato**, parola per parola dal piano e dal disegno a `1b531e9`: la testa con gli *Strumenti*, i vincoli globali, l'errata (**E24**–**E27** sono del compito 4, già applicate al testo; **E19** ed **E20** sono le cure del compito 3 che le sue prove nuove tengono), la voce P-4, le decisioni D7 e D8, il compito 4 intero, le risposte 4, 12 e 20, la pagina kit della (b), la (f), i controlli 9, 11, 14 e 20, le trappole 1, 3, 4 e 12, e le tre sonde delle tavole. Leggilo **tutto**, a blocchi: il suo peso lo dà `wc -c -l` |
| `dispatch-task-4.md` | il prompt che l'implementatore ha ricevuto — il **contratto** contro cui lo giudichi |
| `task-4-report.md` | il **rapporto dell'implementatore**: è una dichiarazione, non un fatto |

E nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`: `compare_task4.py`, lo script del
pre-controllo che ricostruisce dal **testo del piano** ogni file dettato e lo confronta col commit. ⚠️ **È anch'esso una
dichiarazione**: lo leggi, e lo provi.

Poi `git show 841dc54`. ⛔ **Non leggi** il piano intero (oltre 8 700 righe), il disegno intero, `docs/HANDOFF.md`,
`docs/adr/`, `docs/archivio/`, l'audit: dove ti serve, `grep -n` e `sed -n` nel punto esatto. Le regole di `CLAUDE.md`
valgono per te: codice in inglese e documenti in italiano, nessuna cifra senza il suo comando, i fine-riga per file.

📌 **Un confronto in più, se ti serve:** la copia del pre-controllo `%TEMP%\pc4b` è il compito rifatto dal testo corretto
del piano, e i suoi otto file sono uguali, byte per byte, a quelli che `compare_task4.py` ricostruisce — misurato dal
pre-controllo sulla copia `%TEMP%\pc4c`. Non le modifichi.

## 2. Che cosa verifichi

1. **Ogni affermazione misurabile del rapporto si RILANCIA** — il comando accanto, la tua uscita accanto alla sua
   (regola 5 di *«Come si esegue un compito»*). Elenca i comandi rilanciati. Un numero che non torna è un rilievo. I log
   del cancello che il rapporto cita devono essere **dell'implementatore**: `ls -la --time-style=full-iso <log>` contro
   `git log -1 --format=%ci 841dc54`.
2. **La conformità al dettato.** `python docs/superpowers/plans/2026-09-23-design-system-esecuzione/compare_task4.py 1b531e9 841dc54`
   dalla radice del repository, con `PYTHONIOENCODING=utf-8`. Ma prima **provalo nelle due direzioni**: leggilo e di' se
   ricostruisce davvero ciò che il compito detta — la ricetta del *«Come si riprende»* del piano, i file interi e le
   sostituzioni, le due celle del piano —; poi nel **clone** fai un commit che cambia **un** carattere di **un** file
   dettato (per esempio un commento di `gui/src/kit/Kit.vue`) e lancia lo script, dal clone, contro quel commit: deve dire
   `DIFFERS` su quel file **e su nessun altro**, ed uscire 1. Ciò che lo script lascia `CHECK BY HAND` lo guardi tu: nel
   piano cambiano **solo** le righe **3** e **4** della tabella della posizione — la riga 4 a `✅ 2026-09-25` con la colonna
   Commit `—`, la riga 3 con la colonna Commit `` `c7b7bcd`, con la cura `9d2ffb0` ``. ⚠️ La data **2026-09-25** l'ha fissata
   il dispaccio, e resta anche se il commit è del giorno dopo: lo script lo mostra `CHECK BY HAND`, ed è atteso.
3. **Il rosso del Passo 2, riprodotto, e la sequenza di E24.** Nel clone a `841dc54`, togli ciò che al Passo 2 non esiste
   ancora — `gui/kit.html`, `gui/src/kit/main.ts` e `gui/src/kit/Kit.vue`, ciascuno salvato prima in una copia —, lascia
   `gui/src/kit/kit.browser.test.ts`, `gui/src/testing/probes.ts` e la riga di **E24** in `gui/vite.config.ts`, e lancia la
   prova come la lancia il Passo 2: il messaggio rosso, contro l'Atteso e il rapporto. Poi i file tornano dalle copie,
   `cmp` lo conferma, e la corsa del Passo 5 **subito dopo**, sulla cache che il rosso ha lasciato: verde alla prima
   corsa. **E la direzione rossa di E24**: la stessa sequenza **senza** la riga `optimizeDeps: { force: true }` — la
   voce E24 del brief dice come il pre-controllo l'ha misurata —, e la prima corsa dopo il rosso deve cadere com'è scritto.
   Riga e file tornano dalle copie. ⚠️ **Un Atteso si misura sulla sequenza dei passi, non su una corsa pulita** — lezione 1
   del pre-controllo.
4. **Le due direzioni del Passo 6**, nel clone: la riga di prova `build: { rolldownOptions: … }` in `gui/vite.config.ts`
   e `bash scripts/gate-gui.sh` **rosso** con *«the kit page is in the package»*; tolta, **verde**. E la guardia di
   non-vacuità della prima riga: che cosa la fa scattare, provato.
5. **Le violazioni del Passo 7**, nel clone a `841dc54`, una alla volta, com'è scritto nel Passo 7 — la prima tabella col
   progetto `browser`, la seconda con `npm run lint` —: ciascuna rossa **per la ragione scritta** — il messaggio vero, non
   «rosso» —, e sulle **sole** prove che nomina, **da sola**, e tornata indietro con la copia salvata (vincolo 11), con
   `git -C <clone> status --porcelain` vuoto alla fine. E la metà verde della riga di **E27**: con la sola
   `margin-inline` e il raggio giusto, **verde**.
6. **Guardarla** — punto 5 di *«Come si esegue un compito»* e Passo 8: un verde non prova che una cosa si veda. Nel clone,
   `npm run dev` in background, poi `/kit.html` nel **Chrome installato**, a grandezza vera. Il modo: uno script
   Playwright nello scratchpad — `playwright` preso dal `node_modules` del clone, per esempio con `createRequire`;
   `chromium.launch({ channel: "chrome" })`; nessun download —, che apre la pagina e, per **ciascuna** delle scelte del tema
   in cima, salva uno screenshot a pagina intera; poi la tastiera: Tab fino ai radio, le frecce, la finestra aperta con
   Invio e chiusa con Esc, uno screenshot dopo ciascun passo. Se hai gli strumenti del browser integrato
   (`mcp__Claude_Browser__*`), puoi usare quelli al posto dello script. Gli screenshot li **guardi**, uno per uno, e dici
   che cosa vedi: raggi concentrici, testo tagliato, cose che sbordano, icone fuori centro, il contrasto, il focus
   visibile, la radice che cambia tema. Poi **fermi** il server di sviluppo — tiene aperti file sotto `node_modules` —
   prima di qualunque suite o cancello, e controlli che nessun processo `node.exe` rimanga acceso per mano tua.
7. **Il cancello intero**, una volta, sull'**albero del repository**: `GATE GREEN`; le righe `Test Files` e `Tests` dei due
   progetti, contro il rapporto e contro la base — che nessuna delle prove di prima sia sparita, per **nome di file** e
   non solo per conto —; `found 0 vulnerabilities`; la riga del pezzo JavaScript, contro la base — la pagina kit non deve
   muoverla. E `bash scripts/check-docs.sh` → `OK`.
8. **Il codice, oltre il dettato.** Il codice è dettato dal piano, quindi un difetto del dettato **non si cura**: è una
   **voce d'errata candidata** (la prossima libera la dice il rapporto dell'implementatore: se ne ha usate, la tua viene
   dopo) col testo corretto proposto. Guarda soprattutto le lezioni del pre-controllo, scritte nel *«Come si riprende»*
   del piano: **(a)** ogni **ramo** di ciascuna sonda di `gui/src/testing/probes.ts` — `concentricRadii`, `fits`,
   `iconsCentred` — è percorso dalla pagina? Un `throw` sul ramo, nel clone, lo dice; poi un caso verde e uno rosso (così
   è nata **E27**, e la guardia di non-vacuità non lo vede, perché conta i casi di tutti i rami insieme); **(b)** ogni
   app montata da una prova si smonta in `afterEach`, mai sull'ultima riga della prova (**E25**); **(c)** ogni riga della
   pagina o di un pezzo che porta il suo perché in un commento — tolta, quale prova cade?; **(d)** ogni commento che dice
   *«ogni»* o *«tutti»* su una cartella che il compito fa crescere, e ogni blocco del linter provato sul suo **confine**
   (**E26**). E poi: che le **interfacce** che il compito produce per i compiti dopo — la lista *Interfaces* del compito,
   e le firme di `concentricRadii`, `fits` e `iconsCentred` che i compiti 6 e 8 importano — siano quelle del codice, e
   che le due sostituzioni del compito 8 su `kit.browser.test.ts` si applichino **una volta ciascuna** al file committato
   (`grep -n` nel piano per trovarle); i vincoli **4** (nessuna scritta fuori dalla pagina kit), **5** (nessun colore a
   mano in `gui/src/**/*.vue`, `Kit.vue` compresa) e **6** (nessun `var(--ref-` fuori da `gui/src/tokens/`); che nessun
   commento **dica il falso** dopo questo commit (gotcha #58).
9. **I vincoli globali** del brief — soprattutto il **7** (nessuna dipendenza nuova: `gui/package.json` e il lockfile non
   cambiano), l'**8**, il **9** (fine-riga: `git ls-files --eol` e i CR, prima e dopo, per ogni file toccato; i file nuovi
   LF), l'**11** (le due direzioni, la guardia di non-vacuità di ogni prova del browser), il **12**
   (`git diff --stat 1b531e9..841dc54 -- crates/ gui/schema/` vuoto).
10. **Il contratto** del prompt dell'implementatore: un commit, i soli file del suo punto 2, le due celle della posizione,
   il messaggio che comincia con `design-system(compito 4): `, niente push (`git status -sb` → `ahead 1`), niente
   co-autore (`git log -1 --format=%B 841dc54 | grep -ci co-authored` → 0).

## 3. Il rapporto

Scrivi `.superpowers/sdd/2026-09-23-design-system/task-4-review.md`:

1. il **verdetto**, in una riga: conforme o no;
2. i **rilievi**, classificati **Critico** (rende falso ciò che il compito consegna, o rompe un Atteso o un criterio),
   **Importante** (una regola del repository violata, o un'affermazione del rapporto non provata), **Minore**, **Nit** —
   ciascuno col file, la riga ritrovata col `grep` sulla frase, l'evidenza (comando e uscita) e la cura proposta **in
   forma di testo**, senza applicarla; un difetto del **dettato** come voce d'errata candidata;
3. l'elenco dei **comandi rilanciati**, con le uscite;
4. che cosa hai **visto** guardando la pagina, tema per tema, coi percorsi degli screenshot;
5. ciò che **non** hai potuto verificare, e perché.

Nel messaggio finale al coordinatore: il verdetto, il conto dei rilievi per classe, il percorso del rapporto — in poche
righe.
