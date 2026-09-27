Sei il **revisore del compito 7** — *le viste col nome, sotto: il pacchetto, il negozio, il dock, la geometria comune, lo
schema* — del piano `docs/superpowers/plans/2026-09-23-design-system.md`, nel repository `E:\ALL\DEV\MY_REPOS\daemon`
(Windows; il tool Bash è Git Bash: apri ogni chiamata con `cd /e/ALL/DEV/MY_REPOS/daemon &&`). Sei un subagente fresco e
**non hai scritto nulla** di ciò che rivedi. Rivedi **un commit**:

| Commit | Chi | Che cosa |
|---|---|---|
| `0417c9c` | l'implementatore | il compito 7, sopra `fa1ae0e` |

**All'avvio verifichi:** `git rev-parse --short HEAD` → `0417c9c`; `git status --porcelain` → vuoto;
`git log --oneline fa1ae0e..HEAD` → **una** riga. Se non torna, ti fermi e lo riporti.

⛔ **Non modifichi nulla nell'albero del repository**: niente scritture, niente commit, niente push. Le prove che
chiedono una mutazione le fai su un **clone**: `git clone E:/ALL/DEV/MY_REPOS/daemon C:/Users/Jays/AppData/Local/Temp/rv7`,
poi `git -C C:/Users/Jays/AppData/Local/Temp/rv7 checkout --detach 0417c9c` e `(cd /c/Users/Jays/AppData/Local/Temp/rv7/gui && npm ci)`.
Il clone sta in `%TEMP%` e **non** nello scratchpad: su questa macchina `LongPathsEnabled` è 0, e i percorsi di
`node_modules` sotto lo scratchpad passerebbero i 260 caratteri. Lo lasci dov'è: lo cancella il coordinatore. Lo
scratchpad per i log e per i tuoi file è
`C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\e4b4f474-4c3b-4180-b8de-ae411307e17a\scratchpad\review7`
(in Git Bash `/c/Users/Jays/AppData/Local/Temp/claude/E--ALL-DEV-MY-REPOS-daemon/e4b4f474-4c3b-4180-b8de-ae411307e17a/scratchpad/review7`).
Niente subagenti.

⛔ **Una cosa alla volta**: il cancello e un `npm`/`vitest` sullo **stesso** albero si pestano su `gui/node_modules`
(gotcha #133), e due suite insieme sulla stessa macchina rendono instabili le prove col tempo (P-20, P-21 del piano). Su
questa macchina la memoria è 31,2 GB e le app del proprietario ne tengono gran parte — 6,4 GB liberi su 31,2, misurati dal coordinatore il 2026-09-27 alle 17:30, prima del dispaccio —: mai due suite, mai due
cancelli, mai un cancello mentre nel clone gira una suite. Il cancello gira sull'**albero del repository**, in background
verso un log nuovo nello scratchpad, e aspetti la notifica; le mutazioni girano **nel clone**. ⛔ Il browser delle prove è il
**Chrome installato**, senza finestra: se Playwright dice che manca, ti fermi e lo riporti — niente `npx playwright
install`, niente download (decisione 22 del disegno). ⚠️ **Chrome si aggiorna da sé** (lezione 5 della consegna
dell'esecuzione del compito 3): un rosso del progetto `browser` che non nomina una prova — `Target page, context or browser
has been closed`, `Failed to connect to the browser session` — si rilancia dopo aver riletto la versione dal **nome della
cartella**, `ls "/c/Program Files/Google/Chrome/Application/"`. ⛔ **Mai `chrome.exe --version`**: su Windows apre il
browser col profilo dell'utente e non stampa nulla (lezione 4).

La macchina, misurata dal coordinatore il 2026-09-27 alla ripresa: `core.autocrlf` `false` in `.git/config` — `--get-all`
rende `true` dal file di sistema, `false` da `C:/Users/Jays/.gitconfig` e `false` da `.git/config`, e vale l'ultima —,
quindi l'albero è `w/lf`, e i file nuovi nascono `w/lf`; un clone fresco prende il `false` del file dell'utente, e il suo
albero è `w/lf` fuori dai file che sono CRLF già nell'indice (`git ls-files --eol | grep i/crlf`). Node v24.19.0; Chrome
`154.0.8037.58`, letto dal nome della cartella. Il cancello d'apertura del coordinatore, a `fa1ae0e`, lanciato da solo:
`GATE GREEN` in 1m19s, sotto `gui/` il progetto `jsdom` con 19 file passati e uno saltato, 144 prove passate e una saltata,
il progetto `browser` con 6 file e 56 prove, il pezzo JavaScript `691.82 kB` compresso `210.77 kB` (`index-DKNLC6Y2.js`),
`found 0 vulnerabilities` — le cifre esatte del pre-controllo, a `1c168a1`. Il suo log è `logs/gate-apertura.log` nello
scratchpad padre di `review7`, e **non** è quello dell'implementatore.

## 1. Che cosa leggi

Nella cartella `.superpowers/sdd/2026-09-23-design-system/`:

| File | Che cos'è |
|---|---|
| `task-7-brief.md` | **il compito come era dettato**, parola per parola dal piano e dal disegno a `fa1ae0e`: la testa con gli *Strumenti*, i vincoli globali, *A che punto è* e *Come si esegue un compito*, l'errata (**E75**…**E81** sono del suo pre-controllo e sono **già applicate** al testo, tranne **E81**, che è del compito 8; **E2** ha allineato una sua *Trova*, **E64** i suoi conti; **E40** è la finestra prima del benvenuto), la voce P-19, le decisioni D3, D4, D5, D12 e D13, le voci aperte che il piano sa, il compito 7 intero, e dal disegno le risposte 13 e 19, la Panoramica e le viste salvate col nome della (d), i controlli 17 e 18 e le decisioni 19 e 20 del coordinatore. Leggilo **tutto**, a blocchi: il suo peso lo dà `wc -c -l` |
| `dispatch-task-7.md` | il prompt che l'implementatore ha ricevuto — il **contratto** contro cui lo giudichi. ⚠️ Il suo commit porta anche la **riga 7** della tabella della posizione del piano (Passo 9), e **non** la riga 6 (**E79**) |
| `task-7-report.md` | il **rapporto dell'implementatore**: è una dichiarazione, non un fatto |

E nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`:

- `compare_task7.py`, lo script del pre-controllo che ricostruisce dal **testo del piano** ogni file dettato — la ricetta sta
  nel *«Come si riprende»* del piano, sotto *«La ricetta del compito 7»* (`grep -n 'La ricetta del compito 7'`), nelle due
  forme `W` e `R` — e lo confronta col commit; il piano cambiato lo mostra come `CHECK BY HAND`. ⚠️ **È una dichiarazione**:
  lo leggi, e lo provi.
- `plan_ops.py`, lo strumento del pre-controllo che applica un compito del piano **dal suo testo**, passo per passo —
  `python plan_ops.py <piano> 7 list`, e `apply <radice> --upto N` applica i *Trova*/*Sostituisci*, i *Crea* e i *Riscrivi*
  dei Passi fino all'N compreso. Il piano glielo dai **com'era alla base**:
  `git show fa1ae0e:docs/superpowers/plans/2026-09-23-design-system.md > <scratchpad review7>/plan-base.md`. ⚠️ **È una
  dichiarazione anche lui**: prima di servirtene, `list` deve rendere le operazioni della ricetta — **quattordici**, con le
  righe del piano della ricetta, misurato dal coordinatore prima di questo dispaccio — e il suo `apply` di tutti i passi, su
  un clone a `fa1ae0e`, deve dare i **dieci** file di codice del commit: `git -C <clone> add -A`, poi
  `git -C <clone> diff --cached --stat 0417c9c -- gui/` vuoto. ⚠️ Senza `add -A` il confronto è falso: `git diff <commit>`
  dà per **cancellato** un file nuovo che sta sul disco ma non nell'indice — misurato dal coordinatore prima di questo
  dispaccio.

Poi `git show 0417c9c`. ⛔ **Non leggi** il piano intero (oltre 11 000 righe), il disegno intero, `docs/HANDOFF.md`,
`docs/adr/`, `docs/archivio/`, l'audit: dove ti serve, `grep -n` e `sed -n` nel punto esatto. Le regole di `CLAUDE.md`
valgono per te: codice in inglese e documenti in italiano, nessuna cifra senza il suo comando, i fine-riga per file.

## 2. Che cosa verifichi

1. **Ogni affermazione misurabile del rapporto si RILANCIA** — il comando accanto, la tua uscita accanto alla sua
   (regola 5 di *«Come si esegue un compito»*). Elenca i comandi rilanciati. Un numero che non torna è un rilievo. I log
   del cancello che il rapporto cita devono essere **dell'implementatore**: `ls -la --time-style=full-iso <log>` contro
   `git log -1 --format=%ci 0417c9c`.
2. **La conformità al dettato.** `python docs/superpowers/plans/2026-09-23-design-system-esecuzione/compare_task7.py fa1ae0e 0417c9c`
   dalla radice del repository, con `PYTHONIOENCODING=utf-8`. Ma prima **provalo nelle due direzioni**: leggilo e di' se
   ricostruisce davvero ciò che il compito detta — le due forme della ricetta, `W` e `R`; poi nel **clone** fai **due**
   commit usa-e-getta, uno alla volta, e dopo ciascuno lancia lo script, dal clone, contro quel commit: uno che cambia **un**
   carattere in un commento di un file scritto per intero (`W`, per esempio `gui/src/frame/nearest.ts`), uno che cambia **un**
   carattere in un commento di un file toccato da una sostituzione, **fuori** dal testo sostituito (`R`, per esempio
   `gui/src/frame/dock.ts`). Ciascuno deve dire `DIFFERS` su quel file **e su nessun altro**, ed uscire 1; poi il clone
   torna a `0417c9c`. Sul commit vero, il piano compare come `CHECK BY HAND`: il suo diff deve toccare la **sola riga 7**
   della tabella della posizione — la riga 6 no (**E79**) —; e il disegno **non** deve comparire.
3. **Il rosso del Passo 2 e i verdi dei Passi 3–6, riprodotti** con `plan_ops.py` nel clone, un passo per volta: per il
   passo N, il clone a `fa1ae0e` pulito (`git -C <clone> checkout --detach --force fa1ae0e` e `git -C <clone> clean -fd`,
   che non tocca `node_modules`, ignorata), poi `apply <clone> --upto N`, e il comando del passo. Il rosso contro l'Atteso e
   contro il rapporto, **per nome di prova**. ⚠️ **Un Atteso si misura sulla sequenza dei passi, non su una corsa pulita** —
   lezione 1 del pre-controllo del compito 4. I comandi e gli Atteso, dal compito, tutti sotto `gui/`:
   - **Passo 2** — `npx vitest run --project jsdom src/frame/nearest.test.ts src/frame/schematic.test.ts src/stores/stores.test.ts src/frame/frame.test.ts`:
     `Tests  7 failed | 28 passed (35)` — `Failed to resolve import "./nearest"` e `"./schematic"`; le **sei** prove nuove
     del negozio coi messaggi dell'Atteso, fra cui `layout.showView is not a function` e, due volte, `layout.saveNamed is not
     a function` — la seconda è la prova di **E75** —; e il dock, `expected [ 'activity', 'costs', …(5) ] to deeply equal
     [ 'status' ]`. Verdi tutte le prove che c'erano.
   - **Passo 3** — `npx vitest run --project jsdom src/frame/nearest.test.ts src/frame/keys.test.ts`: verde, `Tests  8 passed (8)`.
   - **Passo 4** — `npx vitest run --project jsdom src/frame/schematic.test.ts`: verde, due prove.
   - **Passo 5** — `npx vitest run --project jsdom src/stores/stores.test.ts`: verde, `Tests  20 passed (20)`.
   - **Passo 6** — `npx vitest run --project jsdom src/frame/frame.test.ts`: verde, `Tests  15 passed (15)`.
   Alla fine il clone torna a `0417c9c`, e `git -C <clone> status --porcelain` vuoto.
4. **Le quindici violazioni del Passo 8**, nel clone a `0417c9c`, una alla volta, com'è scritto nella tabella del Passo 8,
   ciascuna sulla **suite intera** — `npx vitest run` sotto `gui/`, i due progetti — perché tre righe fanno cadere prove di
   altri file (**E77**): ciascuna rossa **per la ragione scritta** — il messaggio vero, non «rosso» — e con le prove che
   cadono **quelle scritte**, né più né meno. Indietro con la copia salvata (vincolo 11), `cmp` dopo ciascuna,
   `git -C <clone> status --porcelain` vuoto alla fine. E **la riga che nessuna prova tiene** (**E76**): in `Frame.vue`
   `layout.view = view;` al posto di `layout.showView(view);`, la suite intera **resta verde** — la dichiarazione è vera?
   ⚠️ Se il rapporto porta **voci candidate** — `E82` e le seguenti — le giudichi con la tua corsa: il fatto regge, e per la
   ragione che il rapporto dice? Il testo che propone è giusto e basta?
5. **La stabilità** — **P-19**: la prova della tastiera del compito 5 cadeva una volta su dieci corse. Nel clone a `0417c9c`,
   **tre** corse di `npx vitest run`, una alla volta: una caduta anche **una** volta si riporta con la sua uscita (P-19,
   D23), non si rilancia finché passa. ⛔ **Il compito 7 non si guarda nel browser**: la regola 5 di *«Come si esegue un
   compito»* non lo nomina — niente server di sviluppo.
6. **Il cancello intero**, una volta, sull'**albero del repository**: `GATE GREEN`; le righe `Test Files` e `Tests` dei due
   progetti, contro il rapporto e contro la base — il progetto `jsdom` cresce dei **due** file nuovi, `nearest.test.ts` e
   `schematic.test.ts`, e nessuna delle prove di prima deve essere sparita, per **nome di file** e non solo per conto —;
   `found 0 vulnerabilities`; la riga del pezzo JavaScript contro la base: qui **cresce**, com'è previsto, e la cifra va al
   proprietario (**E80**, N-2). E `bash scripts/check-docs.sh` → `OK`. ⚠️ Una prova che cade anche **una** volta in una
   corsa che non la muta si riporta con la sua uscita: non si rilancia finché passa.
7. **Il codice, oltre il dettato.** Il codice è dettato dal piano, quindi un difetto del dettato **non si cura**: è una
   **voce d'errata candidata** — la prossima libera è **E82**, o la prima dopo le candidate del rapporto — col testo
   corretto proposto. Guarda soprattutto:
   - **(a)** ogni riga che porta il suo perché in un commento — tolta, quale prova cade? Una riga che nessuna prova tiene e
     che la tabella del Passo 8 non nomina è un rilievo, se non è dichiarata come **E76**;
   - **(b)** ogni commento che dice *«every»*, *«all»*, *«only»*, *«two»*, *«three»* o una cifra su qualcosa che il compito fa
     crescere o cambia — i modi in cui il negozio cambia sotto il dock (**E78**), i campi del pacchetto, le viste, i
     chiamanti di `nearest` —: `grep -rn` sulle parole **e** sulle cifre, e ogni riga che il censimento restituisce si legge
     **intera**;
   - **(c)** le **interfacce** che il compito produce per il compito 8, la lista *Produces* del compito: in `LayoutPack` i
     campi facoltativi `named?: NamedView[]` e `openNamed?: string`, con `NamedView`; in `useLayout()` lo stato
     `openNamed`, `showView(view)` e `saveNamed(name, layout, shown)` → `"saved" | "empty" | "taken"` (**D12**); `apply(api,
     view, pack, named?)`; `nearest<T extends { rect: Box }>(from, candidates, direction)` con `Box` e `Direction`;
     `schematic(layout)` → `Tile[]` in frazioni del quadrato unitario. ⛔ **La domanda 3 del pre-controllo — un artefatto
     sbagliato che compila si trova solo usandolo da fuori**: nel clone scrivi un file di prova **usa-e-getta**, fuori da ciò
     che il compito ha scritto, che li usa come li userà il compito 8 — la Panoramica che disegna le miniature con
     `schematic` sulle tre viste e su una vista col nome, le frecce che scelgono con `nearest` fra rettangoli dati a mano,
     *«Salva questa vista»* con `saveNamed` e i nomi mostrati, l'apertura e la chiusura con `openNamed` e `showView` —, lo
     lanci, e lo togli; riporta ciò che l'interfaccia rende scomodo, ambiguo o sbagliato;
   - **(d)** che nessun commento **dica il falso** dopo questo commit (gotcha #58): la testa di `createDock` (**E78**), i
     commenti di `moveActive.ts` sulla geometria che ora vive in `nearest.ts`, quelli di `layout.ts` sul pacchetto e sulla
     finestra prima del benvenuto (**E40**, **E81**), quelli di `frame.test.ts` e di `stores.test.ts` che il compito tocca;
   - **(e)** la cura di **E75**, il nome riletto **sempre** dal pacchetto in `receive`, e che la prova di **E75** la tenga
     davvero — la riga *«ciò che arriva illeggibile»* del Passo 8.
8. **I vincoli globali** del brief — soprattutto il **7** (nessuna dipendenza nuova: `gui/package.json` e il lockfile non
   cambiano), l'**8**, il **9** (fine-riga: `git ls-files --eol` e i CR, prima e dopo, per ogni file toccato; i quattro file
   nuovi LF; `gui/src/stores/layout.ts`, riscritto per intero, col terminatore che aveva), l'**11** (le due direzioni), il
   **12** (`git diff --stat fa1ae0e..0417c9c -- crates/ gui/schema/` vuoto: ciò che è nuovo nel pacchetto è un campo
   **facoltativo** dei byte opachi).
9. **Il contratto** del prompt dell'implementatore: un commit, i soli file del suo punto 2 — i dieci di codice e, del piano,
   la sola riga 7 della posizione —, il messaggio che comincia con `design-system(compito 7): ` e porta il pezzo JavaScript
   prima e dopo (**E80**), niente push (`git status -sb` → `ahead 1`), niente co-autore
   (`git log -1 --format=%B 0417c9c | grep -ci co-authored` → 0).

## 3. Il rapporto

Scrivi `.superpowers/sdd/2026-09-23-design-system/task-7-review.md`:

1. il **verdetto**, in una riga: conforme o no;
2. i **rilievi**, classificati **Critico** (rende falso ciò che il compito consegna, o rompe un Atteso o un criterio),
   **Importante** (una regola del repository violata, o un'affermazione del rapporto non provata), **Minore**, **Nit** —
   ciascuno col file, la riga ritrovata col `grep` sulla frase, l'evidenza (comando e uscita) e la cura proposta **in
   forma di testo**, senza applicarla; un difetto del **dettato** come voce d'errata candidata;
3. l'elenco dei **comandi rilanciati**, con le uscite;
4. il **consumatore da fuori** del punto 7(c): che cosa hai scritto, che cosa ha reso, e il suo percorso nello scratchpad;
5. ciò che **non** hai potuto verificare, e perché.

Nel messaggio finale al coordinatore: il verdetto, il conto dei rilievi per classe, il percorso del rapporto — in poche
righe.
