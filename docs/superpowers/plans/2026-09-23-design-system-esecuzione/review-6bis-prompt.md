Sei il **revisore del compito 6bis** — *la cura delle tre voci del Passo 8: il bordo nei raggi, la barra di scorrimento, i
messaggi* — del piano `docs/superpowers/plans/2026-09-23-design-system.md`, nel repository `E:\ALL\DEV\MY_REPOS\daemon`
(Windows; il tool Bash è Git Bash: apri ogni chiamata con `cd /e/ALL/DEV/MY_REPOS/daemon &&`). Sei un subagente fresco e
**non hai scritto nulla** di ciò che rivedi. Rivedi **un commit**:

| Commit | Chi | Che cosa |
|---|---|---|
| `da7a522` | l'implementatore | il compito 6bis, sopra `9579ca0` |

**All'avvio verifichi:** `git rev-parse --short HEAD` → `da7a522`; `git status --porcelain` → vuoto;
`git log --oneline 9579ca0..HEAD` → **una** riga. Se non torna, ti fermi e lo riporti.

⛔ **Non modifichi nulla nell'albero del repository**: niente scritture, niente commit, niente push. Le prove che
chiedono una mutazione le fai su un **clone**: `git clone E:/ALL/DEV/MY_REPOS/daemon C:/Users/Jays/AppData/Local/Temp/rv6bis`,
poi `git -C C:/Users/Jays/AppData/Local/Temp/rv6bis checkout --detach da7a522` e `(cd /c/Users/Jays/AppData/Local/Temp/rv6bis/gui && npm ci)`.
Il clone sta in `%TEMP%` e **non** nello scratchpad: su questa macchina `LongPathsEnabled` è 0, e i percorsi di
`node_modules` sotto lo scratchpad passerebbero i 260 caratteri. Lo lasci dov'è: lo cancella il coordinatore. Lo
scratchpad per i log e per i tuoi file è
`C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\2f940fbc-999d-4486-9faf-6d913c767183\scratchpad\review6bis`
(in Git Bash `/c/Users/Jays/AppData/Local/Temp/claude/E--ALL-DEV-MY-REPOS-daemon/2f940fbc-999d-4486-9faf-6d913c767183/scratchpad/review6bis`).
Niente subagenti.

⛔ **Una cosa alla volta**: il cancello e un `npm`/`vitest` sullo **stesso** albero si pestano su `gui/node_modules`
(gotcha #133), e due suite insieme sulla stessa macchina rendono instabili le prove col tempo (P-20, P-21 del piano). Su
questa macchina la memoria è 31,2 GB e le app del proprietario ne tengono gran parte — 6,0 GB liberi su 31,2, misurati dal coordinatore il 2026-09-27 alle 15:22, prima del dispaccio: meno della metà dei 14,4 del compito 6 —: mai due suite, mai due
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
`154.0.8037.58`, letto dal nome della cartella. Il cancello d'apertura del coordinatore, a `9579ca0`, lanciato da solo:
`GATE GREEN` in 1m28s, sotto `gui/` il progetto `jsdom` con 19 file passati e uno saltato, 137 prove passate e una saltata,
il progetto `browser` con 5 file e 48 prove, il pezzo JavaScript `690.61 kB` compresso `210.41 kB` (`index-CjDBuiZn.js`),
`found 0 vulnerabilities` — le cifre esatte del pre-controllo, a `dc77fc8`. Il suo log è `logs/gate-baseline.log` nello
scratchpad padre di `review6bis`, e **non** è quello dell'implementatore.

## 1. Che cosa leggi

Nella cartella `.superpowers/sdd/2026-09-23-design-system/`:

| File | Che cos'è |
|---|---|
| `task-6bis-brief.md` | **il compito come era dettato**, parola per parola dal piano e dal disegno a `9579ca0`: la testa con gli *Strumenti*, i vincoli globali, *A che punto è* e *Come si esegue un compito*, l'errata (**E59**, **E60** ed **E61** sono le tre voci che il compito cura, **E62** ed **E63** lo hanno scritto, **E64**…**E69** sono del suo pre-controllo e sono **già applicate** al testo; **E25**, **E29**, **E30**, **E33**, **E44** ed **E46** sono della sonda dei raggi che il compito stringe), le voci P-16 e P-23, le decisioni D9 e D25…D29, le voci aperte che il piano sa, il compito 6bis intero, e dal disegno le risposte 4 e 20, il linguaggio visivo, le sezioni (a), (b) e (f), la barra e la fascia della (d), i controlli 22 e 23, le trappole 21–28 e le decisioni 28 e 29 del coordinatore. Leggilo **tutto**, a blocchi: il suo peso lo dà `wc -c -l` |
| `dispatch-task-6bis.md` | il prompt che l'implementatore ha ricevuto — il **contratto** contro cui lo giudichi. ⚠️ Per **E69** il suo Passo 16 non era suo, e il commit non porta né le celle della posizione né il disegno |
| `task-6bis-report.md` | il **rapporto dell'implementatore**: è una dichiarazione, non un fatto |

E nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`: `compare_task6bis.py`, lo script del
pre-controllo che ricostruisce dal **testo del piano** ogni file dettato — la ricetta sta nel *«Come si riprende»* del piano,
sotto *«La ricetta del compito 6bis»* (`grep -n 'La ricetta del compito 6bis'`) — e lo confronta col commit; i due script
del Passo 5 li **esegue davvero** (la forma `S`). ⚠️ **È anch'esso una dichiarazione**: lo leggi, e lo provi.

E nello scratchpad padre di `review6bis`, in `tools/`: `plan_ops.py`, lo strumento del pre-controllo che applica un
compito del piano **dal suo testo**, passo per passo — `python plan_ops.py <piano> 6bis list`, e `apply <radice> --upto N`
applica i *Trova*/*Sostituisci*, i *Crea* e i *Riscrivi* dei Passi fino all'N compreso. ⚠️ **Non esegue i due script del
Passo 5**, che sono codice Python e non sostituzioni: da quel passo in poi `gui/src/tokens/base.css` e la tavola li prendi dal
commit, dopo che `compare_task6bis.py` li ha detti uguali al dettato. ⚠️ **È una dichiarazione anche lui**: prima di
servirtene, `list` deve rendere le operazioni della ricetta — trentanove, con le righe del piano della ricetta — e il suo
`apply` di tutti i passi, su un clone a `9579ca0` coi due file del Passo 5 presi dal commit, deve dare i diciannove file del
commit (`git -C <clone> diff --stat da7a522` vuoto sui file del compito).

Poi `git show da7a522`. ⛔ **Non leggi** il piano intero (oltre 11 000 righe), il disegno intero, `docs/HANDOFF.md`,
`docs/adr/`, `docs/archivio/`, l'audit: dove ti serve, `grep -n` e `sed -n` nel punto esatto. Le regole di `CLAUDE.md`
valgono per te: codice in inglese e documenti in italiano, nessuna cifra senza il suo comando, i fine-riga per file.

## 2. Che cosa verifichi

1. **Ogni affermazione misurabile del rapporto si RILANCIA** — il comando accanto, la tua uscita accanto alla sua
   (regola 5 di *«Come si esegue un compito»*). Elenca i comandi rilanciati. Un numero che non torna è un rilievo. I log
   del cancello che il rapporto cita devono essere **dell'implementatore**: `ls -la --time-style=full-iso <log>` contro
   `git log -1 --format=%ci da7a522`.
2. **La conformità al dettato.** `python docs/superpowers/plans/2026-09-23-design-system-esecuzione/compare_task6bis.py 9579ca0 da7a522`
   dalla radice del repository, con `PYTHONIOENCODING=utf-8`. Ma prima **provalo nelle due direzioni**: leggilo e di' se
   ricostruisce davvero ciò che il compito detta — le tre forme della ricetta, `W`, `R` e `S` —; poi nel **clone** fai
   **due** commit usa-e-getta, uno alla volta, e dopo ciascuno lancia lo script, dal clone, contro quel commit: uno che
   cambia **un** carattere in un commento di un file scritto per intero (`W`), uno che cambia **un** carattere in un
   commento di `gui/src/tokens/base.css` (`S`). Ciascuno deve dire `DIFFERS` su quel file **e su nessun altro**, ed uscire
   1; poi il clone torna a `da7a522`. ⛔ **Il piano non deve comparire** nell'uscita — il commit del codice non lo tocca
   (**E69**) —, e nemmeno il disegno: se compaiono, è un rilievo.
3. **I rossi dei Passi 2, 3, 4, 5, 8, 10 e 12, riprodotti** con `plan_ops.py` nel clone, un passo per volta: per il passo
   N, il clone a `9579ca0` pulito (`git -C <clone> checkout --detach --force 9579ca0` e `git -C <clone> clean -fd`, che non
   tocca `node_modules`, ignorata), poi `apply <clone> --upto N`, e per N ≥ 5 `gui/src/tokens/base.css` e la tavola dal
   commit (`git -C <clone> show da7a522:<file>`). Il comando del passo, e il rosso contro l'Atteso e contro il rapporto, **per
   nome di prova** e nei due temi. ⚠️ **Un Atteso si misura sulla sequenza dei passi, non su una corsa pulita** — lezione 1
   del pre-controllo del compito 4. I comandi e gli Atteso, dal compito:
   - **Passo 2** — `(cd gui && npx vitest run --project browser src/testing/probes.browser.test.ts)`: rossa la sola prova
     nuova del pixel del bordo, `expected { near: 2, bad: [] } to deeply equal { near: 2, bad: [ …(2) ] }`.
   - **Passo 3** — lo stesso file, `src/kit/kit.browser.test.ts` e `src/frame/dock.browser.test.ts`: le prove della sonda
     verdi, e rosse nei due temi le tre coppie che i token di oggi storcono di un pixel — le righe di `BaseList`, il pulsante
     nell'angolo della finestra, il gruppo staccato del dock —, `Tests  6 failed | 36 passed (42)`.
   - **Passo 4** — `src/tokens/tokens.browser.test.ts`: `Error: the token --size-scrollbar is not defined here: are the
     token sheets loaded?`.
   - **Passo 5** — `npx vitest run --project jsdom src/tokens/` verde, `Tests  17 passed (17)`; nel browser
     `src/tokens/tokens.browser.test.ts src/kit/kit.browser.test.ts src/frame/dock.browser.test.ts`, rosse la barra
     (`{ vertical: +0, horizontal: +0 }`), la cornice della pagina kit a 21/34 e il dock a 21; verdi le righe di `BaseList`
     e la finestra.
   - **Passo 8** — `npx vitest run --project jsdom src/components/kit.test.ts`: `Failed to resolve import "./BaseNotice.vue"`.
   - **Passo 10** — `src/kit/kit.browser.test.ts`: rosse nei due temi tre prove, `Tests  6 failed | 16 passed (22)`.
   - **Passo 12** — sotto jsdom `src/frame/frame.test.ts src/panels/modules.test.ts`, rosse quattro prove, `Unable to get
     .base-notice within`; nel browser `src/frame/frame.browser.test.ts`, rosse le due della fascia, `expected +0 to be
     close to 12`.
   Alla fine il clone torna a `da7a522`, e `git -C <clone> status --porcelain` vuoto.
4. **Le sedici violazioni del Passo 15**, nel clone a `da7a522`, una alla volta, com'è scritto nella tabella del Passo 15:
   ciascuna rossa **per la ragione scritta** — il messaggio vero, non «rosso» — e con le prove che cadono **quelle
   scritte**: nei due temi dove la riga lo dice, e per la regola della barra anche `board.test.ts`, come la riga dice.
   Indietro con la copia salvata (vincolo 11), `cmp` dopo ciascuna, `git -C <clone> status --porcelain` vuoto alla fine.
   ⚠️ Se il rapporto porta **voci candidate** — `E70` e le seguenti — le giudichi con la tua corsa: il fatto regge, e per
   la ragione che il rapporto dice? Il testo che propone è giusto e basta?
5. **Guardarla** — punto 5 di *«Come si esegue un compito»*, che nomina il 6bis (**E68**): un verde non prova che una cosa
   si veda. Nel clone a `da7a522`, `npm run dev` in background, poi la SPA e la pagina kit, `/kit.html`, nel **Chrome
   installato** a 1440 × 900, ⛔ **con le barre di scorrimento accese**: uno script Playwright nello scratchpad —
   `playwright` preso dal `node_modules` del clone, per esempio con `createRequire`; `chromium.launch({ channel: "chrome",
   ignoreDefaultArgs: ["--hide-scrollbars"] })`; nessun download. ⛔ **Non** il pannello del browser dell'app: è del
   coordinatore e del proprietario, e con la finestra dell'app coperta non disegna (lezione 4 del *«Come si riprende»*
   del piano). Nei **due temi** — `document.documentElement.dataset.theme` a `"light"` e a `"dark"` —, e uno
   screenshot dopo ciascuna di queste cose:
   - **la fascia a core spento, PRIMA di `harnessFake.deliverAll()`** — `warn` con «Riprova», sulla pagina, col raggio della
     scheda e a 12 px dai lati e dalle schede, la riga al centro —; **dopo** la consegna, il timbro diverso, `stop` (le
     fixture consegnano anche `StaleBuild`: **E69**, lezione 5 del *«Come si riprende»* del piano);
   - *«Richiesta inviata: in attesa del core.»* in Impostazioni, scegliendo l'altra policy, e il verdetto in Stato;
   - **le barre** di Stato e Impostazioni nella Home: sottili, senza frecce, staccate dalle estremità, l'angolo fra le due
     trasparente; il cursore **assente** a riposo, **presente** col puntatore sopra la scatola e col fuoco dentro. ⚠️ Il tuo
     puntatore è simulato: la conferma col mouse vero resta al Passo 16, del proprietario — tu riporti ciò che il simulato
     mostra, e `getComputedStyle` di `--scrollbar-trigger` nei tre stati;
   - **i raggi** — le schede, il gruppo staccato con «Stacca la tessera», la zona d'arrivo trascinando una linguetta; nella
     pagina kit la cornice, la finestra, e i messaggi in scheda e sulla pagina con «Riprova» nell'angolo;
   - il **frammento del contrasto** del Passo 8 del compito 6 (`grep -n 'Passo 8: guardarlo, nei due temi'` nel piano, e le
     righe che seguono), sulla SPA: il primo di `worst` **sopra 4,5**.
   Gli screenshot li **guardi**, uno per uno, e dici che cosa vedi: il contrasto, il focus visibile, testo tagliato, cose che
   sbordano o si sovrappongono. ⛔ **L'aspetto non lo giudichi tu**: lo giudica il proprietario (controllo 15, Passo 16) — tu
   descrivi, e riporti ciò che è rotto. Poi **fermi** il server di sviluppo per PID — chiudere la shell non basta, il suo
   `node` resta in ascolto — prima di qualunque suite o cancello, e controlli che nessun processo `node.exe` rimanga acceso
   per mano tua.
6. **Il cancello intero**, una volta, sull'**albero del repository**: `GATE GREEN`; le righe `Test Files` e `Tests` dei due
   progetti, contro il rapporto e contro la base — che nessuna delle prove di prima sia sparita, per **nome di file** e non
   solo per conto —; `found 0 vulnerabilities`; la riga del pezzo JavaScript, contro la base: qui **cresce**, com'è previsto,
   e la cifra va al proprietario (N-2). E `bash scripts/check-docs.sh` → `OK`. ⚠️ Una prova che cade anche **una** volta in
   una corsa che non la muta si riporta con la sua uscita (P-19, D23): non si rilancia finché passa.
7. **Il codice, oltre il dettato.** Il codice è dettato dal piano, quindi un difetto del dettato **non si cura**: è una
   **voce d'errata candidata** — la prossima libera è **E70**, o la prima dopo le candidate del rapporto — col testo
   corretto proposto. Guarda soprattutto: **(a)** ogni riga che porta il suo perché in un commento — tolta, quale prova
   cade?; **(b)** ogni commento che dice *«every»*, *«all»*, *«only»* o una cifra su qualcosa che il compito fa crescere o
   cambia — i pezzi di base, le voci di `ICONS`, i raggi; **(c)** le **interfacce** che il compito produce, la lista
   *Produces* del compito — `BaseNotice.vue` con le sue props, lo slot `action`, `data-tone`, `data-on-page` e
   `data-action`, le classi `.title` e `.description`; le voci `info`, `ok`, `warn` e `stop` di `ICONS`; i token
   `--size-scrollbar` e `--scrollbar-trigger` — una proprietà registrata che **non si eredita** —, `--radius-card` e
   `--radius-frame`; lo scarto di mezzo pixel di `concentricRadii` (**D25**, una costante sola); le barre mostrate al
   progetto `browser` (**D26**); gli aiutanti `frame(theme)`, `px(token)` e `computed(property, token)` di
   `frame.browser.test.ts`, che il compito 8 riusa (**E63**); **(d)** che nessun commento **dica il falso** dopo questo
   commit (gotcha #58) — le cifre dei raggi di prima nei commenti di `gui/src`, lo scarto di 1,5, gli *«otto pezzi»* in
   parole **e** in cifre (**E64**: una cifra non la trova un `grep` sulle parole), il commento di `icons.ts` sopra le voci che
   il compito aggiunge — `grep -rn` sulle parole e sulle cifre, e ogni riga che il censimento restituisce si legge
   **intera**; **(e)** i vincoli **4** (le parole solo da `it.json`), **5** (nessun colore a mano in `gui/src/**/*.vue` e
   in `tokens/dock.css`) e **6** (nessun `var(--ref-` fuori da `gui/src/tokens/`); **(f)** la sonda a mezzo pixel resta
   giusta per chi la usava prima: le prove della pagina kit (compito 4) e del dock (compito 6), verdi, e che cosa lo scarto
   nuovo cambia nel loro giudizio, se qualcosa.
8. **I vincoli globali** del brief — soprattutto il **7** (nessuna dipendenza nuova: `gui/package.json` e il lockfile non
   cambiano), l'**8**, il **9** (fine-riga: `git ls-files --eol` e i CR, prima e dopo, per ogni file toccato; i file nuovi
   LF), l'**11** (le due direzioni, e la guardia di non-vacuità di ogni prova del browser, `frame.browser.test.ts`
   compreso), il **12** (`git diff --stat 9579ca0..da7a522 -- crates/ gui/schema/` vuoto).
9. **Il contratto** del prompt dell'implementatore: un commit, i soli diciannove file del suo punto 2, **nessuna** cella
   del piano e **niente** disegno (**E69**), il messaggio che comincia con `design-system(compito 6bis): ` e porta il pezzo
   JavaScript prima e dopo, niente push (`git status -sb` → `ahead 1`), niente co-autore
   (`git log -1 --format=%B da7a522 | grep -ci co-authored` → 0).

## 3. Il rapporto

Scrivi `.superpowers/sdd/2026-09-23-design-system/task-6bis-review.md`:

1. il **verdetto**, in una riga: conforme o no;
2. i **rilievi**, classificati **Critico** (rende falso ciò che il compito consegna, o rompe un Atteso o un criterio),
   **Importante** (una regola del repository violata, o un'affermazione del rapporto non provata), **Minore**, **Nit** —
   ciascuno col file, la riga ritrovata col `grep` sulla frase, l'evidenza (comando e uscita) e la cura proposta **in
   forma di testo**, senza applicarla; un difetto del **dettato** come voce d'errata candidata;
3. l'elenco dei **comandi rilanciati**, con le uscite;
4. che cosa hai **visto** guardando la SPA e la pagina kit, tema per tema, coi percorsi degli screenshot, e il frammento del
   contrasto;
5. ciò che **non** hai potuto verificare, e perché.

Nel messaggio finale al coordinatore: il verdetto, il conto dei rilievi per classe, il percorso del rapporto — in poche
righe.
