Sei il **revisore del compito 8** — *la cornice: la barra col nome della vista, la Panoramica, la striscia a pillola* — del
piano `docs/superpowers/plans/2026-09-23-design-system.md`, nel repository `E:\ALL\DEV\MY_REPOS\daemon` (Windows; il tool
Bash è Git Bash: apri ogni chiamata con `cd /e/ALL/DEV/MY_REPOS/daemon &&`). Sei un subagente fresco e **non hai scritto
nulla** di ciò che rivedi. Rivedi **un commit**:

| Commit | Chi | Che cosa |
|---|---|---|
| `ac56b11` | l'implementatore | il compito 8, sopra `bc6e94d` |

**All'avvio verifichi:** `git rev-parse --short HEAD` → `ac56b11`; `git status --porcelain` → vuoto;
`git log --oneline bc6e94d..HEAD` → **una** riga. Se non torna, ti fermi e lo riporti.

⛔ **Non modifichi nulla nell'albero del repository**: niente scritture, niente commit, niente push. Le prove che
chiedono una mutazione le fai su un **clone**: `git clone E:/ALL/DEV/MY_REPOS/daemon C:/Users/Jays/AppData/Local/Temp/rv8`,
poi `git -C C:/Users/Jays/AppData/Local/Temp/rv8 checkout --detach ac56b11` e `(cd /c/Users/Jays/AppData/Local/Temp/rv8/gui && npm ci)`.
Il clone sta in `%TEMP%` e **non** nello scratchpad: su questa macchina `LongPathsEnabled` è 0, e i percorsi di
`node_modules` sotto lo scratchpad passerebbero i 260 caratteri. Lo lasci dov'è: lo cancella il coordinatore. Lo
scratchpad per i log e per i tuoi file è
`C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\d19536a1-7be4-437b-8e11-39fed58af059\scratchpad\review8`
(in Git Bash `/c/Users/Jays/AppData/Local/Temp/claude/E--ALL-DEV-MY-REPOS-daemon/d19536a1-7be4-437b-8e11-39fed58af059/scratchpad/review8`).
Niente subagenti.

⛔ **Una cosa alla volta**: il cancello e un `npm`/`vitest` sullo **stesso** albero si pestano su `gui/node_modules`
(gotcha #133), e due suite insieme sulla stessa macchina rendono instabili le prove col tempo (P-20, P-21 del piano). Su
questa macchina la memoria è 31,2 GB e le app del proprietario ne tengono gran parte — 5,1 GB liberi su 31,2, misurati dal coordinatore il 2026-09-27 alle 20:45, prima del dispaccio —: mai due suite, mai due
cancelli, mai un cancello mentre nel clone gira una suite o un server di sviluppo. Il cancello gira sull'**albero del
repository**, in background verso un log nuovo nello scratchpad, e aspetti la notifica; le mutazioni girano **nel clone**.
⛔ Il browser delle prove è il **Chrome installato**, senza finestra: se Playwright dice che manca, ti fermi e lo riporti —
niente `npx playwright install`, niente download (decisione 22 del disegno). ⚠️ **Chrome si aggiorna da sé** (lezione 5
della consegna dell'esecuzione del compito 3): un rosso del progetto `browser` che non nomina una prova — `Target page,
context or browser has been closed`, `Failed to connect to the browser session` — si rilancia dopo aver riletto la versione
dal **nome della cartella**, `ls "/c/Program Files/Google/Chrome/Application/"`. ⛔ **Mai `chrome.exe --version`**: su
Windows apre il browser col profilo dell'utente e non stampa nulla (lezione 4).

La macchina, misurata dal coordinatore il 2026-09-27 alla ripresa: `core.autocrlf` `false` in `.git/config` — `--get-all`
rende `true` dal file di sistema, `false` da `C:/Users/Jays/.gitconfig` e `false` da `.git/config`, e vale l'ultima —,
quindi l'albero è `w/lf`, e i file nuovi nascono `w/lf`; un clone fresco prende il `false` del file dell'utente, e il suo
albero è `w/lf` fuori dai file che sono CRLF già nell'indice (`git ls-files --eol | grep i/crlf`). Node v24.19.0; Chrome
`154.0.8037.58`, letto dal nome della cartella. Il cancello d'apertura del coordinatore, a `bc6e94d`, lanciato da solo:
`GATE GREEN`, sotto `gui/` il progetto `jsdom` con 21 file passati e uno saltato, 157 prove passate e una saltata, il
progetto `browser` con 6 file e 56 prove, il pezzo JavaScript `693.02 kB` compresso `211.20 kB` (`index-CvaZoYyv.js`),
`found 0 vulnerabilities` — le cifre esatte del pre-controllo, a `f3206c7`. Il suo log è `logs/gate-apertura.log` nello
scratchpad padre di `review8`, e **non** è quello dell'implementatore.

## 1. Che cosa leggi

Nella cartella `.superpowers/sdd/2026-09-23-design-system/`:

| File | Che cos'è |
|---|---|
| `task-8-brief.md` | **il compito come era dettato**, parola per parola dal piano e dal disegno a `bc6e94d`: la testa con gli *Strumenti*, i vincoli globali, *A che punto è* e *Come si esegue un compito*, l'errata (**E91**…**E101** sono del suo pre-controllo e sono **già applicate** al testo; **E63**, **E73**, **E74**, **E76**, **E81**, **E85** ed **E90** sono le voci che lo aspettavano; **E35** ed **E69** la forma dello sguardo; **E40** la finestra prima del benvenuto), le voci P-11 e P-22…P-25, le decisioni D4, D5, D12…D18 e D29, le voci aperte che il piano sa, il compito 8 intero, e dal disegno le risposte 13, 19 e 20, la riga «focus» della (a), la barra, la fascia, la Panoramica, le viste salvate col nome e la finestra della (d), i controlli 17 e 19 e le decisioni 18–20 del coordinatore. Leggilo **tutto**, a blocchi: il suo peso lo dà `wc -c -l` |
| `dispatch-task-8.md` | il prompt che l'implementatore ha ricevuto — il **contratto** contro cui lo giudichi. ⚠️ Per **E101** il suo commit **non** porta nessuna cella del piano né alcun documento, e il Passo 10 non era suo |
| `task-8-report.md` | il **rapporto dell'implementatore**: è una dichiarazione, non un fatto |

E nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`:

- `compare_task8.py`, lo script del pre-controllo che ricostruisce dal **testo del piano** ogni file dettato — la ricetta sta
  nel *«Come si riprende»* del piano, sotto *«La ricetta del compito 8»* (`grep -n 'La ricetta del compito 8'`), nelle due
  forme `W` e `R` — e lo confronta col commit; il piano cambiato è `UNEXPECTED`, come il disegno (**E101**). ⚠️ **È una
  dichiarazione**: lo leggi, e lo provi.
- `plan_ops.py`, lo strumento del pre-controllo che applica un compito del piano **dal suo testo**, passo per passo —
  `python plan_ops.py <piano> 8 list`, e `apply <radice> --upto N` applica i *Trova*/*Sostituisci*, i *Crea* e i *Riscrivi*
  dei Passi fino all'N compreso. Il piano glielo dai **com'era alla base**:
  `git show bc6e94d:docs/superpowers/plans/2026-09-23-design-system.md > <scratchpad review8>/plan-base.md`. ⚠️ **È una
  dichiarazione anche lui**: prima di servirtene, `list` deve rendere le operazioni della ricetta — **trentanove**, con le
  righe del piano della ricetta, misurato dal coordinatore prima di questo dispaccio — e il suo `apply` di tutti i passi, su
  un clone a `bc6e94d`, deve dare i **venti** file di codice del commit: `git -C <clone> add -A`, poi
  `git -C <clone> diff --cached --stat ac56b11 -- gui/` vuoto. ✅ Il coordinatore l'ha fatto prima di questo dispaccio, su un clone
  suo a `bc6e94d`: `39 operations, 0 refused`, venti file, e il confronto con `ac56b11` vuoto — una sua dichiarazione,
  che rifai. ⚠️ Senza `add -A` il confronto è falso:
  `git diff <commit>` dà per **cancellato** un file nuovo che sta sul disco ma non nell'indice.

Poi `git show ac56b11`. ⛔ **Non leggi** il piano intero (oltre 11 000 righe), il disegno intero, `docs/HANDOFF.md`,
`docs/adr/`, `docs/archivio/`, l'audit: dove ti serve, `grep -n` e `sed -n` nel punto esatto. Le regole di `CLAUDE.md`
valgono per te: codice in inglese e documenti in italiano, nessuna cifra senza il suo comando, i fine-riga per file.

## 2. Che cosa verifichi

1. **Ogni affermazione misurabile del rapporto si RILANCIA** — il comando accanto, la tua uscita accanto alla sua
   (regola 5 di *«Come si esegue un compito»*). Elenca i comandi rilanciati. Un numero che non torna è un rilievo. I log
   del cancello che il rapporto cita devono essere **dell'implementatore**: `ls -la --time-style=full-iso <log>` contro
   `git log -1 --format=%ci ac56b11`.
2. **La conformità al dettato.** `python docs/superpowers/plans/2026-09-23-design-system-esecuzione/compare_task8.py bc6e94d ac56b11`
   dalla radice del repository, con `PYTHONIOENCODING=utf-8`. Ma prima **provalo nelle due direzioni**: leggilo e di' se
   ricostruisce davvero ciò che il compito detta — le due forme della ricetta, `W` e `R`; poi nel **clone** fai **due**
   commit usa-e-getta, uno alla volta, e dopo ciascuno lancia lo script, dal clone, contro quel commit: uno che cambia **un**
   carattere in un commento di un file scritto per intero (`W`, per esempio `gui/src/frame/Overview.vue`), uno che cambia
   **un** carattere in un commento di un file toccato da una sostituzione, **fuori** dal testo sostituito (`R`, per esempio
   `gui/src/stores/invoke.ts`). Ciascuno deve dire `DIFFERS` su quel file **e su nessun altro**, ed uscire 1; poi il clone
   torna a `ac56b11`. Sul commit vero, nulla `DIFFERS` e nulla è `UNEXPECTED`, uscita 0: il piano e il disegno **non**
   devono comparire (**E101**).
3. **Il rosso del Passo 3 e il verde del Passo 7, riprodotti** con `plan_ops.py` nel clone: per il passo N, il clone a
   `bc6e94d` pulito (`git -C <clone> checkout --detach --force bc6e94d` e `git -C <clone> clean -fd`, che non tocca
   `node_modules`, ignorata), poi `apply <clone> --upto M` — **M = 2** per il Passo 3, **M = 7** per il Passo 7 —, e il
   comando del passo. Il rosso contro l'Atteso e contro il rapporto, **per nome di prova**. ⚠️ **Un Atteso si misura sulla
   sequenza dei passi, non su una corsa pulita** — lezione 1 del pre-controllo del compito 4. I comandi e gli Atteso sono nel
   compito: il Passo 3 (`grep -n 'Passo 3: lancia le prove, e guardale fallire'` nel brief) e il Passo 7 (il blocco `bash`
   in fondo al Passo 7, `grep -n 'Passo 7: la striscia a pillola'`). ⚠️ **La candidata E102 del rapporto** — il
   coordinatore l'aveva notata leggendo, prima del dispaccio, senza misurarla: il comando `jsdom` del Passo 7 nomina **tre**
   file — `frame.test.ts`, `a11y.test.ts`, `copy.test.ts` — e il suo Atteso dice `Tests  61 passed (61)` *«nei quattro
   file»*, contando anche *«quella di `showNamed`»*, che vive in `stores.test.ts`; il rapporto dice `Tests  40 passed (40)`
   com'è scritto e `Tests  61 passed (61)` con `src/stores/stores.test.ts` aggiunto. Misura le due forme, e giudica la cura
   che propone — il comando coi quattro file, come al Passo 3 — contro l'alternativa, l'Atteso a tre file.
   Alla fine il clone torna a `ac56b11`, e `git -C <clone> status --porcelain` vuoto.
4. **Le trentadue violazioni del Passo 9**, nel clone a `ac56b11`, una alla volta, com'è scritto nella tabella del Passo 9,
   ciascuna sulla **suite intera** — `npx vitest run` sotto `gui/`, i due progetti — perché diverse righe fanno cadere prove
   di altri file (**E95**): ciascuna rossa **per la ragione scritta** — il messaggio vero, non «rosso» — e con le prove che
   cadono **quelle scritte**, né più né meno. Indietro con la copia salvata (vincolo 11), `cmp` dopo ciascuna,
   `git -C <clone> status --porcelain` vuoto alla fine. ⚠️ Se il rapporto porta **voci candidate** — `E102` e le seguenti — le
   giudichi con la tua corsa: il fatto regge, e per la ragione che il rapporto dice? Il testo che propone è giusto e basta?
5. **La stabilità** — **P-19**: la prova della tastiera del compito 5 cadeva una volta su dieci corse. Nel clone a `ac56b11`,
   **tre** corse di `npx vitest run`, una alla volta: una caduta anche **una** volta si riporta con la sua uscita (P-19,
   D23), non si rilancia finché passa.
6. **Guardarla** — punto 5 di *«Come si esegue un compito»*, che nomina il compito 8: un verde non prova che una cosa si
   veda. ⚠️ Il Passo 10 è del proprietario col coordinatore, **dopo** la tua revisione (**E101**): tu guardi per conto tuo.
   Nel clone a `ac56b11`, `npm run dev` in background, poi la SPA nel **Chrome installato** a 1440 × 900, ⛔ **con le barre
   di scorrimento accese**: uno script Playwright nello scratchpad — `playwright` preso dal `node_modules` del clone, per
   esempio con `createRequire`; `chromium.launch({ channel: "chrome", ignoreDefaultArgs: ["--hide-scrollbars"] })`; nessun
   download. ⛔ **Non** il pannello del browser dell'app: è del coordinatore e del proprietario. Nei **due temi** —
   `document.documentElement.dataset.theme` a `"light"` e a `"dark"` —, e uno screenshot dopo ciascuna delle cose che il
   Passo 10 elenca (`grep -n 'Passo 10: guardarlo, nei due temi'` nel brief, e le righe che seguono): la barra col nome della
   vista e l'icona; **F3** e il clic sul nome che aprono la Panoramica, con le miniature delle tre viste e, ingrandito un
   gruppo con la sua presa, la miniatura che lo disegna solo (**E85**); la carta corrente e *«Salva questa vista»* spenta
   **prima** di `harnessFake.deliverAll()` e accesa dopo (**E98**); le frecce, Invio, Esc col fuoco che torna al nome; il nome
   di una vista nuova rifiutato vuoto, rifiutato *«home»*, poi salvato, e la vista salvata riaperta dalla Panoramica; la
   striscia a pillola col pulsante *«moduli»* che apre il cassetto, col clic e dalla tastiera; e col Tab, nella Home, il fuoco
   su Stato, che scorre, con l'anello **dentro** la scheda (**E100**). E il **frammento del contrasto** del Passo 8 del
   compito 6 (`grep -n 'Passo 8: guardarlo, nei due temi'` nel piano, e le righe che seguono), dopo la consegna, due volte:
   sul dock com'è scritto, e con la Panoramica aperta, col selettore `.dock *` sostituito da quello della radice della
   Panoramica, che leggi in `Overview.vue`: il primo di `worst` **sopra 4,5**, e `seen` sopra zero. Gli screenshot li
   **guardi**, uno per uno, e dici che cosa vedi: il contrasto, il focus visibile, testo tagliato, cose che sbordano o si
   sovrappongono. ⛔ **L'aspetto non lo giudichi tu**: lo giudica il proprietario (controllo 15, Passo 10) — tu descrivi, e
   riporti ciò che è rotto. Poi **fermi** il server di sviluppo per PID — chiudere la shell non basta, il suo `node` resta in
   ascolto — prima di qualunque suite o cancello, e controlli che nessun processo `node.exe` rimanga acceso per mano tua.
7. **Il cancello intero**, una volta, sull'**albero del repository**: `GATE GREEN`; le righe `Test Files` e `Tests` dei due
   progetti, contro il rapporto e contro la base — il compito non aggiunge file di prova, e nessuna delle prove di prima
   deve essere sparita, per **nome di file** e non solo per conto —; `found 0 vulnerabilities`; la riga del pezzo
   JavaScript contro la base: qui **cresce**, com'è previsto, e la cifra va al proprietario (**E80**, N-2). E
   `bash scripts/check-docs.sh` → `OK`. ⚠️ Una prova che cade anche **una** volta in una corsa che non la muta si riporta con
   la sua uscita: non si rilancia finché passa.
8. **Il codice, oltre il dettato.** Il codice è dettato dal piano, quindi un difetto del dettato **non si cura**: è una
   **voce d'errata candidata** — la prossima libera è **E103**, dopo la candidata **E102** del rapporto — col testo
   corretto proposto. Guarda soprattutto:
   - **(a)** ogni riga che porta il suo perché in un commento — tolta, quale prova cade? Una riga che nessuna prova tiene e
     che la tabella del Passo 9 non nomina è un rilievo, se non è dichiarata;
   - **(b)** ogni commento che dice *«every»*, *«all»*, *«only»*, *«two»*, *«three»* o una cifra su qualcosa che il compito fa
     crescere o cambia — le viste, le carte, i modi di aprire la Panoramica, gli utenti di `contrastJudged` e di `computed`
     (**P-25**, **E92**) —: `grep -rn` sulle parole **e** sulle cifre, e ogni riga che il censimento restituisce si legge
     **intera**;
   - **(c)** le **interfacce** che il compito produce, la lista *Produces* del compito: `useDrawer()` con `open`; in
     `useInvoke()` il getter `asking`; in `useLayout()` `showNamed(name): boolean` (**E99**); `computed(property, token)` da
     `testing/probes.ts` (**E92**); `Overview.vue` con la prop `snapshot` e `v-model:open`; in `ViewBar.vue` la prop
     `snapshot` e `v-model:overview`; `contrastJudged(node)` da `testing/axe.ts`; in `it.json` le chiavi `overview.*` e
     `drawer.open`; la classe `view-name` e l'attributo `data-card`. ⛔ **La domanda 3 del pre-controllo — un artefatto
     sbagliato che compila si trova solo usandolo da fuori**: nel clone scrivi un file di prova **usa-e-getta**, fuori da ciò
     che il compito ha scritto, che li usa come li userebbe un pannello o una cornice futura — il cassetto aperto da un altro
     componente con `useDrawer` e F3 che allora non apre la Panoramica, `asking` letto da fuori mentre la conferma chiede,
     `showNamed` con un nome che la lista ha e con uno che non ha, `computed` e `contrastJudged` su un elemento che il compito
     non guarda —, lo lanci, e lo togli; riporta ciò che l'interfaccia rende scomodo, ambiguo o sbagliato;
   - **(d)** che nessun commento **dica il falso** dopo questo commit (gotcha #58): quello di **E86** nel `Frame.vue`
     riscritto (**E94**), quelli di `testing/axe.ts` e `testing/probes.ts` (**P-25**, **E92**), quelli di `layout.ts` su
     `showNamed` e sulla finestra prima del benvenuto (**E40**, **E81**, **E98**), la regola di **E100** in `dock.css`, e quelli
     delle prove che il compito tocca;
   - **(e)** le tre cure del **codice dettato** dal pre-controllo — **E98**, **E99**, **E100** —: ciascuna tenuta dalle sue
     righe del Passo 9? E le tre cose di **E90** per questo compito: come è finita ciascuna — curata, dichiarata, o aperta?
9. **I vincoli globali** del brief — soprattutto il **7** (nessuna dipendenza nuova: `gui/package.json` e il lockfile non
   cambiano), l'**8**, il **9** (fine-riga: `git ls-files --eol` e i CR, prima e dopo, per ogni file toccato; i due file
   nuovi LF; i quattro riscritti per intero col terminatore che avevano), l'**11** (le due direzioni, e la guardia di
   non-vacuità di ogni prova del browser), il **12** (`git diff --stat bc6e94d..ac56b11 -- crates/ gui/schema/` vuoto).
10. **Il contratto** del prompt dell'implementatore: un commit, i soli **venti** file di codice del suo punto 2 — nessuna
    cella del piano, nessun documento (**E101**) —, il messaggio che comincia con `design-system(compito 8): ` e porta il
    pezzo JavaScript prima e dopo (**E80**), niente push (`git status -sb` → `ahead 1`), niente co-autore
    (`git log -1 --format=%B ac56b11 | grep -ci co-authored` → 0).

## 3. Il rapporto

Scrivi `.superpowers/sdd/2026-09-23-design-system/task-8-review.md`:

1. il **verdetto**, in una riga: conforme o no;
2. i **rilievi**, classificati **Critico** (rende falso ciò che il compito consegna, o rompe un Atteso o un criterio),
   **Importante** (una regola del repository violata, o un'affermazione del rapporto non provata), **Minore**, **Nit** —
   ciascuno col file, la riga ritrovata col `grep` sulla frase, l'evidenza (comando e uscita) e la cura proposta **in
   forma di testo**, senza applicarla; un difetto del **dettato** come voce d'errata candidata;
3. l'elenco dei **comandi rilanciati**, con le uscite;
4. il **consumatore da fuori** del punto 8(c): che cosa hai scritto, che cosa ha reso, e il suo percorso nello scratchpad;
5. che cosa hai **visto** guardando la SPA, tema per tema, coi percorsi degli screenshot, e i due frammenti del contrasto;
6. ciò che **non** hai potuto verificare, e perché.

Nel messaggio finale al coordinatore: il verdetto, il conto dei rilievi per classe, il percorso del rapporto — in poche
righe.
