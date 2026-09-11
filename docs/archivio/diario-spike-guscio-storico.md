# Il diario dello spike del guscio — le chiusure 1–7, archiviate il 2026-09-11

⛔ **Non è una lettura obbligatoria.** È la cronaca ripresa per ripresa della **parte 1** del
sotto-progetto 2 — SP-8, lo spike del guscio — che è **eseguita per intero** dal 2026-09-10.
Si apre con una domanda storica in mano — *«perché quella sessione decise così?»*, *«quale
trappola aveva trovato?»* — non per farsi un'idea.

📌 **Archiviate il 2026-09-11**, dalla coda del [piano della parte
1](../superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md), **parola per
parola**: sono i byte che stavano in quel file, non un riassunto. Nel piano vivo resta la **sola
ottava chiusura**, che è la consegna alla sessione che scrive la parte 2.

⚠️ **Perché.** `CLAUDE.md`: *«Un verbale di correzione non resta nel documento corretto — va in
`docs/archivio/`, con la data; il documento vivo porta ciò che è vero adesso»*; e la §6 del
[compendio](../COMPENDIO.md): *«La cronaca ripresa per ripresa non sta qui: vive nella stella
polare e, parola per parola, negli archivi»*. Il **diario di un piano** era l'ultimo posto in cui
quella regola non era stata applicata, e si pagava a **ogni** ripresa. Il comando che rifà la
misura, e i suoi limiti, stanno nella sezione *«Sfoltimento del compendio»* di
[`riferimenti.md`](../riferimenti.md); la lezione è il gotcha **#119** di
[`HANDOFF.md`](../HANDOFF.md).

---

### La settima chiusura — 2026-09-10: il compito 7 ESEGUITO col proprietario allo schermo — `dockview` resta, il guscio è Electron, E14 scritta — si riprende dal compito 8

⛔ **DA SAPERE SUBITO.** Niente è a metà nel repository: albero pulito, nessuno stash, nessuna operazione git in corso, nessun
server acceso (il relay di SP-7 e Vite spenti, verificato sulle porte 7878 e 5173), nessun processo dei gusci vivo, nessun codice
di prodotto toccato. ⚠️ **Le due app SONO ANCORA INSTALLATE** (D19) — `%LOCALAPPDATA%\Programs\sp8-electron`,
`%LOCALAPPDATA%\sp8-tauri`, più `%LOCALAPPDATA%\sp8-electron-updater` — e le disinstalla il **compito 8**; in
`$HOME/sp8-measure/` (D21) ci sono ora **dieci** CSV — i quattro del compito 6 più `electron-hidden`, `electron-fullpage`,
`tauri-hidden`, `tauri-fullpage`, `electron-covered`, `electron-front` — e, copiati lì alla chiusura perché lo scratchpad muore con
la sessione, tre file di prova (`console-browser-mosse-2026-09-10.txt`, la console del browser del proprietario; `q3-cdp-2026-09-10.txt`;
`o1-coperta-2026-09-10.txt`) e tre attrezzi (`cdp-popout.mjs`, `cover.ps1`, `front.ps1`): il compito 8 cancella la cartella intera,
**dopo** la propria revisione. Il proprietario ha chiuso con `session-handoff` («appena finisce», cioè dopo la revisione del 7) a
compito 7 chiuso e revisionato. Tre cose in cima: (1) **la decisione sul guscio è presa: A, Electron**, sua («A»), scritta in
`RISULTATI.md` con la tabella che ha avuto davanti; il compito 8 la porta in ADR-0029 con le sue parole; (2) **Q3 è `REFUSED` in
entrambi i gusci**, per cause diverse e lette (O7, E14): per Electron il limite è l'origine `file://` — rimedio **dedotto**, servire
la build da http(s), parte 2 — e per Tauri `window.open` nullo; ⚠️ i modelli del compito 8 che nominano Q3 e Q4 vanno riletti
contro questo esito **prima** di dettarli; (3) **M4 non passa su nessuno dei due nemmeno con la chat nascosta** (146,8 % Electron,
138,1 % Tauri): è la scena `three` senza tetto di fps, un fatto per la SPA (tetto di fps o rendering a richiesta) da dire in
ADR-0029 accanto a P3; O1 è misurato **in parte** (coperta e a riposo ~2 % col titolo congelato; sotto flusso gli stessi picchi della
corsa in primo piano). Il commit `5dc9c6f` porta ancora il trailer `Co-Authored-By` (quarta chiusura). ⚠️ **Questa sessione è stata
aperta da un workspace scratch dell'app desktop, non dal repo**: ha funzionato, coi percorsi assoluti, ma la prossima si apre **nella
cartella del repo** (`C:\Users\zagor\Desktop\harness`), o `CLAUDE.md` e la memoria del progetto non si caricano da soli e lo
scratchpad supera MAX_PATH (le trappole qui sotto).

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main` = `origin/main`: `git fetch --all --prune`, poi `git status -sb` → `## main...origin/main`, niente sotto |
| I commit di questa sessione | `git log --oneline e4d96eb..HEAD` — **due** più questo: `61abe86` (compito 7: la sezione SP-8 completa, la riga SP-8 in roadmap, la posizione 7 ✅, E14, i richiami in P-3 e P-6), `a22fde3` (la correzione della revisione: la riga 4 delle mosse), poi la chiusura |
| Codice di prodotto, cancello, CI, gli spike | **non toccati**: `git diff --stat 5ad4634..HEAD -- crates/ scripts/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml spikes/gesti/ spikes/gui-ipc/` non rende nulla; e in questa sessione nemmeno `spikes/gui-shell/`: `git diff --stat e4d96eb..HEAD -- spikes/gui-shell/` non rende nulla |
| La posizione e l'errata | i compiti **1–7** a `✅ 2026-09-10`; l'errata **E1–E14**: E14 dal compito 7 (sotto `file://` dockview 8.2.0 rifiuta la finestra a parte; P-4 smentito in parte), senza rimedio in questo piano — `grep -c '^\| \*\*E[0-9]*\*\* \|' <piano>` → `14`; P-3 e P-6 portano il richiamo «Misurato il 2026-09-10» — `grep -c '^✅ \*\*Misurato il 2026-09-10, compito 7:\*\*' <piano>` → `2` |
| L'esito | la sezione **SP-8** di `spikes/RISULTATI.md` è **completa**: `awk '/^## SP-8 /{s=1;next} s&&/^## /{s=0} s' spikes/RISULTATI.md \| grep -c '⏳'` → `0`, e `grep -c '<[^ ]'` sulla stessa sezione → `0`; O1 col richiamo misurato, O6 e O7 nuove, cinque righe nuove nelle Evidenze |
| Roadmap | la riga SP-8 chiusa, senza cifre: `grep '^| SP-8 |' docs/roadmap.md \| grep -c 'chiuso il'` → `1`; la riga 6 datata al compito 7 |
| Le app installate, i CSV e le prove | `Get-ChildItem $env:LOCALAPPDATA\Programs -Directory -Filter 'sp8-*'` e `Get-ChildItem $env:LOCALAPPDATA -Directory -Filter 'sp8-*'` → le tre cartelle; `ls "$HOME/sp8-measure"` → dieci CSV, tre file di prova, tre attrezzi |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN` prima di **ogni** commit (log datati nello scratchpad, letti dal revisore contro `git log -1 --format=%ci`) e alla chiusura; `bash scripts/check-docs.sh` → `OK` |
| Fine-riga | il piano **LF** (`tr -cd '\r' < <file> \| wc -c` → `0`); roadmap, compendio e `spikes/RISULTATI.md` CRLF con CR = righe (`RISULTATI` a 448 dopo il compito 7, roadmap a 338) |
| Margine del compendio | il comando del vincolo 11: `9948` prima del richiamo di questa chiusura; le righe `⏭️` restano **tre** |
| Il registro di esecuzione | `.superpowers/sdd/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio/progress.md` su **questa** macchina, ignorato da git, con le righe della sessione 4 e del compito 7, `task-7-review-dispatch.md` e `task-7-review-report.md` (il revisore in sola lettura) |

#### Le decisioni prese eseguendo, nell'ordine (seguono le ventitré della sesta chiusura)

| # | Decisione | Perché | Costo se sbagliata |
|---|---|---|---|
| 24 | la sessione è proseguita dal workspace scratch, su scelta del proprietario («partiamo»), con `cd` in testa a ogni comando e la memoria del progetto letta a mano | riaprire costava a lui; nel repo non cambia nulla | nessuno nel repo; un'ora di trappole (MAX_PATH, cwd) |
| 25 | le mosse 1–7 proposte in una tabella sola; il proprietario le ha fatte nel suo ordine e ha risposto incollando la console e con un «si a tutto» a cinque domande: le parole sono registrate **così**, una risposta per cinque righe, dichiarato nella riga 1 | il protocollo vuole le sue parole, non una parafrasi; fargliele ripetere una per una a fine giornata non avrebbe reso parole più vere | una cella che un lettore prende per un riassunto: è dichiarata |
| 26 | la mossa 7 ufficiale sulla disposizione di default (dopo «azzera»); il `DIFFERENT` del primo caricamento — una disposizione salvata con una finestra a parte, pop-up bloccato dal browser al ricaricamento — è **O6**, un fatto, non il verdetto | il protocollo non fissa la disposizione; il blocco dei pop-up è del browser e nei gusci non si presenta | nessuno: entrambi i risultati sono scritti |
| 27 | Q3 `REFUSED` in entrambi: la **causa** letta dalla console del renderer con la porta di debug e uno script CDP di sola lettura, e scritta (O7, E14); le corse di misura senza porta | la decisione A/B dipende da Q3, e «rifiutata» senza causa avrebbe pesato uguale un limite del guscio (Tauri) e un limite dello spike (Electron, `file://`) | quindici minuti; il rifiuto è lo stesso con e senza porta |
| 28 | E14 senza rimedio in questo piano: servire la build da http(s) è dedotto e si misura nel guscio della SPA (parte 2) | vincolo 2 e vincolo 17 (nessun codice nuovo nella parte 1); il compito 7 misura, non corregge | un fatto in ADR-0029 che dice «oggi no» dove domani sarà «sì con un server locale»: dichiarato |
| 29 | O1 misurato con un Blocco note massimizzato come copertura e una corsa in primo piano di confronto; esito «in parte», scritto così | era il passo 3 della sesta chiusura; una prova parziale dichiarata vale più di una deduzione | nessuno: il verdetto P3 non cambia |
| 30 | le due corse di Electron di M4/M5 fatte con relay e Vite ancora accesi, **non rifatte**; dichiarato nelle Evidenze | il campionatore legge la CPU del solo albero, su 28 processori logici; rifarle costava due gesti al proprietario | qualche punto di asimmetria fra i gusci, dichiarata |
| 31 | la riga M5 «pagina intera» scritta `0 / n` con l'etichetta «dedicata / condivisa» | coerente con la riga M5 a riposo e con E10; un numero solo avrebbe nascosto quale contatore | nessuno |
| 32 | la copia scratchpad di `replace_unique.py` ha una funzione `long()` che aggiunge `\\?\` ai percorsi; il testo dell'aiutante nel piano non cambia | lo scratchpad di questa sessione supera MAX_PATH e Python non apriva i file; è un attrezzo, non un artefatto | nessuno |
| 33 | il consiglio sul guscio: **A**, per la regola del Passo 7 letta contro i numeri (pari dove conta; Q4 col trucco e Q2 su Linux contro Tauri; Q3 rifiutata in entrambi); il proprietario: «A» | D15 e ADR-0029 | nessuno: è sua |
| 34 | i gusci chiusi dal coordinatore con `Stop-Process` quando il proprietario non li ha chiusi dalla finestra | le corse successive volevano processi puliti | nessuno |

#### Che cosa il revisore ha trovato, e dove è finito

| Compito | Rilievo | Dove |
|---|---|---|
| 7 | **Approvato con rilievi** — revisore `sonnet` in sola lettura, `task-7-review-report.md`, ~24 min e ~250k token: le sonde del Passo 10 tutte attese, `GATE GREEN` e check-docs `OK` rifatti con un log datato (`gate-review-task7-1913.log`), i numeri di M4, M5 e O1 **riconciliati cifra per cifra** coi sei CSV nuovi, le sequenze di `cpu_pct` comprese, le quattro caselle del criterio di chiusura soddisfatte | — |
| 7 | **Important**: la riga della barra della mossa 4 citava `move 4 / Q3: popout OPENED for stato`, che non sta in nessun file di prova — sta solo nelle parole del proprietario in chat | corretto in `a22fde3`: la colonna della barra cita solo `for permessi`, verificata nel file della console; la finestra su Stato resta nelle sue parole, con la nota che l'ha riferita in chat; le sue frasi in chat sono ora appese in coda a `console-browser-mosse-2026-09-10.txt` (scratchpad e `$HOME/sp8-measure/`), che è la fonte scritta |
| 7 | **Minor**: quattro aggiunte di prosa oltre le tre eccezioni dichiarate al modello del Passo 8 — l'intestazione delle mosse col browser e l'URL, la nota sulla risposta unica nella riga 1, la spiegazione del pallino e dei bottoni nella riga 8, la lettura dei numeri nella riga della decisione — tutte esatte nel merito | **accettato, non tolto** (decisione del coordinatore): ognuna porta un fatto che il compito 8 userà, nessuna una cifra nuova; il gotcha #76 vale per la prosa che non dice nulla |
| 7 | dichiarato non verificabile dal revisore: le citazioni del proprietario che non stavano in un file (venivano dalla chat) | chiuso con l'appendice delle sue frasi al file della console |

#### Le trappole di questa sessione — istruzioni, non aneddoti

- ⛔ **una sessione aperta da un workspace scratch** dell'app desktop: il cwd del tool Bash **si azzera** dopo ogni chiamata (`cd "C:/Users/zagor/Desktop/harness" &&` in testa a tutto, `git -C` per i comandi singoli); `CLAUDE.md` e la memoria del progetto non si caricano da soli; lo **scratchpad supera MAX_PATH** (~246 caratteri): Git Bash lo regge, **Python e PowerShell no** — `FileNotFoundError` su un file che c'è, e `python <script>` non apre nemmeno lo script (`python - < script`). Il prefisso `\\?\` funziona solo **dentro** Python (`"\\\\?\\" + os.path.abspath(p)`): passato da Bash come argomento arriva storpiato in `\?\`, anche con `MSYS_NO_PATHCONV=1`.
- **`| O1 |` e `| O5 |` esistono anche in SP-7**: un *Trova* preso col `grep` sul file intero non è unico; si estrae dentro la sezione con l'`awk` di SP-8 e si verifica `righe=1`.
- **un blocco lungo con backtick passato a Bash come argomento** («unexpected EOF while looking for matching backtick») non esegue nulla: i testi lunghi si scrivono con `Write` in un file e si appendono con `cat >>`; gli script Python con `Write` e `python - < file`.
- **E12 colpisce anche dentro `powershell -Command`** (`Import-Csv "…\\$f"` → «accesso negato» a una cartella): la barra `/` prima della variabile.
- **la console del renderer di un guscio installato si legge senza DevTools**: Electron con `--remote-debugging-port=<porta>`, Tauri con `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=<porta>` nell'ambiente; `cdp-popout.mjs` (in `$HOME/sp8-measure/`) è il modello — `Runtime.enable`, `Log.enable`, click e tasto via `Input.dispatch*`, la barra via `Runtime.evaluate`; il `process.exit` col WebSocket aperto stampa un'asserzione di libuv, innocua. Le corse di misura si fanno **senza** porta.
- **una finestra si porta davanti da Bash** con `(New-Object -ComObject WScript.Shell).AppActivate(<pid>)` in PowerShell; per coprirla apposta, un secondo PowerShell nascosto che apre `notepad` massimizzato (`cover.ps1`); la finestra dell'app di chat copre il guscio mentre il proprietario scrive «fatto»: si riporta davanti **prima** di ogni corsa.
- **il proprietario a fine giornata risponde «si a tutto» e incolla la console**: si accetta, si salva l'incollato in un file e si conta col `grep`; una tabella lunga in un messaggio gli è sfuggita («che tabella?»): si ripete, corta.
- **i server avviati in background dal tool Bash** (relay, Vite) si fermano dai PID in ascolto (`Get-NetTCPConnection -LocalPort 7878,5173 -State Listen`) uccidendo l'albero; il task in background chiude con exit 127, atteso.
- **il blocco dei pop-up del browser** rompe il ripristino di una disposizione con una finestra a parte al ricaricamento (nessun gesto dell'utente): la mossa 7 si misura su una disposizione senza finestre a parte, o si scrive che è il browser (O6).

#### Che cosa la sessione nuova fa, nell'ordine

1. Aprirla **nella cartella del repo**. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è il commit di questa chiusura o uno dopo.
2. La lettura obbligatoria di `CLAUDE.md`; di questo piano la testa, i vincoli, la posizione, «Come si esegue», l'**errata E1–E14**,
   il pre-controllo, le decisioni, la mappa dei file, questo diario; il **compito 8 per intero** solo al dispaccio, estratto per
   intestazione (`awk '/^## Compito 8:/{f=1} /^## L.intestazione del compendio/{exit} f' <piano>`).
3. **Prima del compito 8**, il suo pre-controllo contro i file di adesso: il `grep` delle tredici case di P-19 rilanciato (il compendio
   e la roadmap sono cambiati oggi); le righe `⏭️` del compendio (`3`); il margine del compendio (vincolo 11) e l'intestazione corta
   (D16); l'`awk` delle tabelle spezzate come baseline (rilievo 12 della terza chiusura); ⚠️ i modelli di ADR-0029 e dei richiami che
   nominano **Q3 e Q4** riletti contro l'esito vero — Q3 `REFUSED` in **entrambi** con le cause (O7, E14), Q4 `auto` per Electron e
   `pointer` per Tauri, M4 sopra soglia su entrambi anche con la chat nascosta, O1 misurato in parte — e la decisione **A, Electron**
   con le parole del proprietario («A»); `grep -c '<[^ ]'` sui modelli prima di dettarli (E13); nessuna variabile dopo `\\` (E12); la
   sonda della Definizione di «fatto» delimitata (rilievo 3 della terza chiusura). Poi `superpowers:subagent-driven-development` come
   per i compiti 1–6: un implementatore `sonnet` fresco col dispaccio in un file, il revisore che rilancia ogni comando e rilegge
   ADR-0029 **contro 0027, 0030 e 0033** (gotcha #59); la disinstallazione delle app e la cancellazione di `$HOME/sp8-measure/`
   **dopo** la revisione del compito 8, che può volerne rileggere i CSV e le prove.
4. Il compito 8 porta a `HANDOFF.md` i gotcha di questo diario e delle chiusure quarta, quinta e sesta; alla §2 della stella polare il
   richiamo su `dockview` (CSS, E2) e sul contratto di `gui-ipc` (decisione 10); alla §4 quello di E1; e chiude il puntatore della §6
   del compendio.
5. Alla chiusura di ogni sessione: questo diario, la memoria dell'agente, `session-handoff`.

### La sesta chiusura — 2026-09-10: il rimedio di E9/E10 ESEGUITO e il compito 6 ESEGUITO e revisionato, l'errata E11–E13 scritta — si riprende dal compito 7, col proprietario allo schermo

⛔ **DA SAPERE SUBITO.** Niente è a metà nel repository: albero pulito, nessuno stash, nessuna operazione git in corso, nessun
server acceso, nessun processo dei gusci vivo, nessun codice di prodotto toccato. ⚠️ **Le due app SONO INSTALLATE** (D19) —
`%LOCALAPPDATA%\Programs\sp8-electron` e `%LOCALAPPDATA%\sp8-tauri`, più `%LOCALAPPDATA%\sp8-electron-updater`, la cache
dell'auto-updater di `electron-builder`, non chiesta e registrata nell'evidenza M2 — e le disinstalla il **compito 8**; i quattro
CSV delle corse stanno in `$HOME/sp8-measure/` (D21). Il proprietario ha chiuso con `session-handoff` («chiudi con
session-handoff ora») a compito 6 chiuso e revisionato, dopo aver scelto di **non** fare il compito 7 in questa sessione. Tre cose
in cima: (1) **P3 non passa** per nessuno dei due gusci in nessuna delle quattro corse — `cpu_pct max` sotto flusso fra 196,8 % e
225,3 % di un core contro la soglia del 25 % — ed è scritto `❌ non passa` nell'esito, da leggere contro **O2** (la scena `three` a
~200 fps domina, ~90–110 % a riposo) e contro la CPU con la chat **nascosta** del compito 7, che esiste per questo — ⚠️ **ma una terza corsa di Electron, del revisore,
ha dato 17,4 %** perché il renderer stava a ~0 % dai 20 s in poi col titolo ancora a ~235 fps (finestra coperta o in secondo piano:
**dedotto**, si misura al compito 7): il richiamo in **O1** dell'esito, e la trappola qui sotto; (2) la
**taglia della tessera 3D di Electron** (`scene=` nel titolo) **balla** fra corse e fasi — 446×318, 215×318, 99×318 — mentre in
Tauri è fissa (100×349), a finestra fissa in entrambi: registrata in **O3**, causa **non misurata**; il revisore, nella sua corsa, ha letto `scene=99x318` stabile fra riposo e flusso, e nello screenshot della corsa Tauri la console dell'emettitore copriva la finestra del guscio, quindi la disposizione dei pannelli non è stata vista — da tenere
davanti al compito 7 (M3 di Electron confronta fps a taglie diverse) e all'8 (ADR-0029 legge M3); (3) la decisione su **M5** è
**presa**: la via A — `tree.ps1` legge anche `Shared Usage` — su delega del proprietario («decidi in base a
`decision-principles`»), la via B (la preferenza grafica di Windows, o la macchina con la RTX 5080) resta sua e registrata nel
richiamo E10 del protocollo; il commit `5dc9c6f` porta ancora il trailer `Co-Authored-By` (quarta chiusura).

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main` = `origin/main`: `git fetch --all --prune`, poi `git status -sb` → `## main...origin/main`, niente sotto |
| I commit di questa sessione | `git log --oneline 939ef5d..HEAD` — **due** più questo: `5b5a883` (il commit d'errata E9/E10: `tree.ps1` con `vram_shared_mb`, due richiami nel protocollo congelato, la posizione), `063d45d` (compito 6: la sezione SP-8 in `spikes/RISULTATI.md`, la riga E11, la posizione), poi la chiusura |
| Codice di prodotto, cancello, CI, gli spike vecchi | **non toccati**: `git diff --stat 5ad4634..HEAD -- crates/ scripts/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml spikes/gesti/ spikes/gui-ipc/` non rende nulla |
| La posizione e l'errata | i compiti **1–6** a `✅ 2026-09-10`; l'errata **E1–E13**: E11 (la cadenza del campionatore) dal compito 6, **E12 ed E13 scritte a questa chiusura** (il comando `\\$f`, i segnaposto nelle celle «Comando») — `grep -c '^\| \*\*E[0-9]*\*\* \|' <piano>` → `13`; nessuna delle tre ha un rimedio al codice |
| L'esito | la sezione **SP-8** di `spikes/RISULTATI.md`, prima di SP-7, con tre sottosezioni: M1–M5, P1–P2, i processi, Q1 e Q2 **piene**; `awk '/^## SP-8 /{s=1;next} s&&/^## /{s=0} s' spikes/RISULTATI.md \| grep -c '⏳'` → `7` (più l'intestazione: otto righe che il compito 7 consuma); `grep -c '<[^ ]'` sulla stessa sezione → `0` |
| Le app installate e i CSV | `Get-ChildItem $env:LOCALAPPDATA\Programs -Directory -Filter 'sp8-*'` e `Get-ChildItem $env:LOCALAPPDATA -Directory -Filter 'sp8-*'` → le cartelle della riga M2; `ls "$HOME/sp8-measure"` → `electron-1.csv`, `electron-2.csv`, `tauri-1.csv`, `tauri-2.csv`, più i due CSV del revisore (`review-*.csv`) nello scratchpad, che muore con la sessione |
| Gli artefatti sul disco, ignorati da git | quelli della quinta chiusura, invariati: i due installatori, `sp8-tauri.exe`, `core.exe`, i tre `node_modules/` |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN` prima di **ogni** commit (log datati nello scratchpad, letti dai revisori contro `git log -1 --format=%ci`) e alla chiusura; `bash scripts/check-docs.sh` → `OK` |
| Fine-riga | i file dell'app, dei gusci, degli script, il protocollo e il piano **LF** (`tr -cd '\r' < <file> \| wc -c` → `0`); `.gitignore`, roadmap, compendio e `spikes/RISULTATI.md` CRLF con CR = righe (`RISULTATI` a 428 dopo il compito 6) |
| Margine del compendio | il comando del vincolo 11: `10137` prima del richiamo di questa chiusura; le righe `⏭️` restano **tre** |
| Il registro di esecuzione | `.superpowers/sdd/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio/progress.md` su **questa** macchina, ignorato da git, coi dispacci e i rapporti di E9/E10 (`task-6pre-dispatch-s3.md`, `task-6pre-report.md`, `task-6pre-review-report.md`) e del compito 6 (`task-6-dispatch.md`, `task-6-report.md`, `task-6-review-report.md`), le righe d'errata dettate (`e11-row.txt`, `e12e13-rows.txt`). Su un'altra macchina non ci sono, e **questo diario più l'errata bastano** |

#### Le decisioni prese eseguendo, nell'ordine (seguono le sedici della quinta chiusura)

| # | Decisione | Perché | Costo se sbagliata |
|---|---|---|---|
| 17 | **M5: la via A**, su delega del proprietario ai cinque criteri; B registrata, non presa | A è misurata (i valori stanno in `Shared Usage`), coerente col rimedio già scritto in E10, dichiara il proxy nel protocollo e nell'ADR, ed è il minimo che fa misurare qualcosa; B è un'impostazione di sistema — vietata all'agente — e il suo effetto è dedotto | una rimisura del compito 6, se il proprietario volesse la dedicata |
| 18 | un dispaccio scritto nella sessione precedente si **riscrive** (`task-6pre-dispatch-s3.md`), non si adatta a mano | HEAD, lo scratchpad (vuoto alla ripresa) e una sezione già eseguita dal commit di chiusura erano stantii; le quattro domande si ripassano sul testo nuovo | nessuno |
| 19 | la sonda `grep -c … → 8` del dispaccio era **sbagliata** (7 righe, 9 occorrenze): corretta nel dispaccio di revisione, nessuna errata | il file riproduce T1–T6 alla lettera; l'implementatore ha registrato la divergenza invece di forzare il file, ed è il comportamento giusto | nessuno |
| 20 | **E11**: nessun rimedio al campionatore di `tree.ps1` (~2 s per campione, non 250 ms) | lo script è lo stesso per i due gusci, il confronto regge; ~15 campioni per fase a 60 s; P3 non cambia verdetto (sette volte la soglia); O5 lo dichiara | un picco più breve di due secondi sfugge, per entrambi allo stesso modo |
| 21 | **E12**: `/` come secondo separatore prima di una variabile nei comandi a PowerShell | `"$VAR\\$altra"` nel tool Bash non espande la seconda variabile; Windows e .NET accettano la barra | nessuno |
| 22 | **E13**: le celle «Comando» delle Evidenze portano il comando **letterale** | il modello contraddiceva la propria sonda «zero slot»; i modelli dei compiti 7 e 8 si passano al `grep` prima di dettarli | nessuno |
| 23 | il compito 7 **non** in questa sessione: scelta del proprietario («chiudi con session-handoff ora») | il 7 vuole lui allo schermo per ~40 minuti, con la telecamera | nessuno |

#### Che cosa i revisori hanno trovato, e dove è finito

| Compito | Rilievo | Dove |
|---|---|---|
| E9/E10 | Approvato con rilievi — Minor: `vram_shared_mb max` balla di **centinaia** di MB fra due corse dello stesso `sp8-tauri.exe` (618/600 riposo/flusso in una, 86/121 nell'altra); il commit riproduce T1–T6, R1–R2 e P1 alla lettera (diff automatico) | punto 1 del dispaccio del 6 (si scrivono entrambe le corse, non si sceglie); O1 dell'esito |
| 6 | Approvato con rilievi — **Important**: una terza corsa di Electron del revisore ha dato `cpu_pct max 17,4 %` sotto flusso con `fps=235`, `msgs=2000`, `p2mean=0.44ms`: P3 passerebbe. Letta nel CSV dal coordinatore: `procs` 4 costante, CPU a ~0 % dagli ultimi 20 s del riposo fino alla fine — la stessa forma che sta dentro la corsa 2 ufficiale (25 s a ~0 % a riposo, la media 22,5 %). Quattro Minor: la sonda `^-[^-]` → 2 del dispaccio di revisione era sbagliata (1: la riga SP-7 ricompare ed è contesto per git), il conto grezzo delle barre dà falsi positivi sulle `\|` scappate, la cella M3 a riposo porta un solo `scene=` per guscio (il modello dettato), O2 diceva «già sopra il 25 %» per otto valori quando uno (22,5 %) è sotto | il richiamo datato in **O1** dell'esito e la frase di O2 corretta, in questo commit di chiusura e ri-rivisti in sola lettura; l'ipotesi — Chromium strozza il renderer di una finestra coperta e il titolo non lo rivela — è **dedotta** e si misura al compito 7; i Minor 1–3 nel registro, il 4 corretto |

#### Le trappole di questa sessione — istruzioni, non aneddoti

- ⛔ **il renderer di Electron va a ~0 % di CPU quando la finestra è coperta o in secondo piano, e il titolo continua a dire ~235 fps**: letto nei CSV (la corsa del revisore dai 20 s alla fine; la corsa 2 ufficiale nei primi 25 s di riposo), `procs` 4 costante; Tauri mai. Dedotto, non misurato. Le corse si fanno con la finestra del guscio in primo piano e nulla sopra — la console dell'emettitore compare sopra: si sposta o si minimizza prima — e una corsa a finestra coperta si fa **apposta** al compito 7 per misurarlo.
- **un dispaccio della sessione precedente è stantio alla ripresa**: HEAD, il percorso dello scratchpad (quello vecchio è **vuoto**: `replace_unique.py` si ricrea dal piano con `sed -n '43,78p'`), una sezione già eseguita dal commit di chiusura. Si riscrive e si ripassano le quattro domande.
- **`grep -c` conta le RIGHE con almeno un'occorrenza, non le occorrenze**: un'attesa numerica di un dispaccio si misura sul testo `new` prima di dettarla.
- ⛔ **nel tool Bash `"$VAR\\$altra"` NON espande la seconda variabile** (E12): la barra `/` prima di una variabile, o il percorso intero in una variabile sola.
- **`Get-Process … -ErrorAction SilentlyContinue` senza corrispondenze rende `exit=1`** con output vuoto: è «nessun processo»; non si incatena con `&&`.
- **un segnaposto descrittivo in una cella «Comando» è uno slot per la sonda** (E13): il modello di una sezione si passa a `grep -c '<[^ ]'` prima di dettarlo.
- **`Shared Usage` balla di centinaia di MB fra corse** dello stesso eseguibile, e **il campionatore va a ~2 s per campione** (E11): due corse per guscio e O1/O5 esistono per questo.
- **`python -c` con un percorso `/c/Users/...` fallisce** (Python su Windows vuole `C:\…`), e `'\'` dentro una stringa Python è un apice escapato: gli script di patch si scrivono in un file, con `os.path.join` e stringhe raw.
- **`electron-builder` crea anche `%LOCALAPPDATA%\sp8-electron-updater`** all'installazione: il compito 8 la toglie con le app.
- **`AskUserQuestion` a due opzioni con «Recommended»**: il proprietario ha risposto «decidi in base a decision-principles» (M5) e «chiudi con session-handoff ora» — la delega chiude la voce, non si ridomanda.
- **i costi**: l'implementatore `sonnet` di E9/E10 ~10 min e ~130k token, il suo revisore ~9 min e ~130k; l'implementatore del compito 6 ~35 min e ~285k (quattro corse da 60 s, due installazioni, tre pagine web); il revisore del 6 ~17 min e ~200k (due corse da 60 s, uno screenshot, tre pagine).

#### Che cosa la sessione nuova fa, nell'ordine

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è il commit di questa chiusura o uno dopo.
2. La lettura obbligatoria di `CLAUDE.md`; di questo piano la testa, i vincoli, la posizione, «Come si esegue», l'**errata E1–E13**,
   il pre-controllo, le decisioni, la mappa dei file, questo diario; il **compito 7 per intero** al momento di eseguirlo — e lo esegue
   il **coordinatore** col proprietario allo schermo, **non** un subagente (regola 7 di «Come si esegue»); ciò che ne esce lo rilegge
   un revisore in sola lettura.
3. **Prima del compito 7**, il suo pre-controllo contro il codice di adesso: i prerequisiti (`ls spikes/gesti/.venv/Scripts/python.exe spikes/gesti/hand_landmarker.task spikes/gesti/relay/target/release/sp7-relay.exe`,
   una telecamera con `Get-PnpDevice -Class Camera -Status OK`, le due app installate, i quattro CSV); ogni comando del compito con
   `\\$variabile` si corregge come dice E12; il modello delle righe che il 7 riempie si passa a `grep -c '<[^ ]'` come dice E13; la
   taglia `scene=` di Electron si guarda nella finestra prima di fidarsi degli fps di M3; le sette righe `⏳` più l'intestazione si
   consumano tutte, con le parole del proprietario così come sono. **E la misura dell'ipotesi di O1**, prima della CPU con la chat
   nascosta: una corsa breve di Electron con la finestra **coperta apposta** da un'altra (`tree.ps1 -Seconds 30 -EmitterAt 8`) contro
   una in primo piano — se la CPU cade a ~0 % e il titolo dice ancora ~235 fps, il richiamo in O1 passa da dedotto a **misurato**, e
   il proprietario legge P3 sapendo che per Electron «finestra dietro» vuol dire «renderer fermo», non «costo basso».
4. Poi il **compito 8** — che porta a `HANDOFF.md` i gotcha di questo diario e delle chiusure quarta e quinta, alla §2 della stella
   polare il richiamo su `dockview` (CSS, E2) e sul contratto di `gui-ipc` (decisione 10), alla §4 quello di E1; disinstalla le due app
   e `sp8-electron-updater`, cancella `$HOME/sp8-measure/`.
5. Alla chiusura di ogni sessione: questo diario, la memoria dell'agente, `session-handoff`.

### La quinta chiusura — 2026-09-10: i compiti 3, 4 e 5 ESEGUITI, l'errata E6–E10 scritta, il rimedio di E9/E10 pronto e NON eseguito — si riprende dal commit d'errata E9/E10, poi dal compito 6

⛔ **DA SAPERE SUBITO.** Niente è a metà nel repository: albero pulito, nessuno stash, nessuna operazione git in corso,
nessun server acceso, nessun processo dei gusci vivo, nessun codice di prodotto toccato, **le due app NON sono installate**
(le installa il compito 6). Il proprietario ha chiuso con `session-handoff` («appena siamo pronti al task 6 lo faremo nella
prossima sessione») a compito 5 chiuso. Tre cose in cima, sue: (1) ⏳ **la decisione su M5** — la macchina delle misure è
un portatile con due GPU e i gusci vanno sull'integrata, dove `Dedicated Usage` è zero (E9, E10): **A**, consigliata e
già scritta come rimedio di E10 — `tree.ps1` legge anche `Shared Usage` e riporta due colonne, due richiami datati nel
protocollo — oppure **B**, sua: la preferenza grafica di Windows «prestazioni elevate» per `sp8-electron.exe` e
`sp8-tauri.exe`, o le misure sulla macchina con la RTX 5080; A e B non si escludono, e l'esito scrive comunque le GPU
lette; (2) il commit `5dc9c6f` porta ancora il trailer `Co-Authored-By` (quarta chiusura); (3) **P3 non passerà per
nessuno dei due gusci** con la scena `three` a ~200 fps nella Home (`cpu_pct max` 184 % Electron, 213 % Tauri sotto
flusso, ~90 % a riposo con la sola scena): non è un difetto, è ciò che O2 e la CPU con la chat nascosta del compito 7
esistono per leggere — da sapere **prima** di leggere l'esito.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main` = `origin/main`: `git fetch --all --prune`, poi `git status -sb` → `## main...origin/main`, niente sotto |
| I commit di questa sessione | `git log --oneline 33911fa..HEAD` — **tre** più questo: `62429bc` (compito 3), `8fc9696` (compito 4, col giro di correzione E8 dentro), `d5eb0b8` (compito 5), poi la chiusura |
| Codice di prodotto, cancello, CI, gli spike vecchi | **non toccati**: `git diff --stat 5ad4634..HEAD -- crates/ scripts/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml spikes/gesti/ spikes/gui-ipc/` non rende nulla |
| La posizione e l'errata | i compiti **1–5** a `✅ 2026-09-10`; l'errata **E1–E10**: E6–E8 dal compito 4, **E9 ed E10 scritte a questa chiusura col rimedio NON eseguito** — `grep -c '^| \*\*E[0-9]*\*\* |' <piano>` → `10` |
| Gli artefatti sul disco, ignorati da git, **su questa macchina** | `spikes/gui-shell/electron/out/sp8-electron Setup 0.0.0.exe` (111 565 262 byte) e `out/win-unpacked/`; `spikes/gui-shell/tauri/src-tauri/target/release/sp8-tauri.exe` e `bundle/nsis/sp8-tauri_0.0.0_x64-setup.exe` (2 162 513 byte); `spikes/gui-ipc/target/release/core.exe`; i `node_modules/` dei tre progetti. Su un'altra macchina si rifanno coi comandi del Passo 1 del compito 6 (più `node -e "require('electron')"` dopo `npm install`, E7) |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN` prima di **ogni** commit (log datati nello scratchpad, letti dai revisori contro `git log -1 --format=%ci`) e alla chiusura; `bash scripts/check-docs.sh` → `OK` |
| Fine-riga | i file dell'app, dei gusci, degli script, il protocollo e il piano **LF** (`tr -cd '\r' < <file> \| wc -c` → `0`); `.gitignore` (55 righe), roadmap, compendio e `spikes/RISULTATI.md` CRLF con CR = righe |
| Margine del compendio | il comando del vincolo 11: `10232` prima del richiamo di questa chiusura; le righe `⏭️` restano **tre** |
| Il registro di esecuzione | `.superpowers/sdd/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio/progress.md` su **questa** macchina, ignorato da git, coi brief, i dispacci, i rapporti e i pacchetti dei compiti 1–5, e i dispacci **pronti**: `task-6pre-dispatch.md` (il rimedio di E9/E10: sei sostituzioni in `tree.ps1`, due richiami nel protocollo, una corsa di prova), `e9e10-rows.txt`, `task-6-brief.md`. Su un'altra macchina non ci sono, e **questo diario più le voci E9/E10 bastano** a riscriverli |

#### Le decisioni prese eseguendo, nell'ordine (seguono le nove della quarta chiusura)

| # | Decisione | Perché | Costo se sbagliata |
|---|---|---|---|
| 10 | compito 3: le chiavi italiane di `Wire` (`canale`, `emesso_micros`, `carico`, `Token`/`Stato`/`Metriche`) **restano**, contro il rilievo Important «plan-mandated» del revisore | sono le chiavi JSON dell'emettitore di `spikes/gui-ipc`, uno spike riusato com'è e fuori mappa; chi deserializza nomina le chiavi del produttore; uno strato di traduzione in uno spike è YAGNI; `Wire` muore con lo spike | sette identificatori italiani in un file che non sale nel prodotto |
| 11 | compito 3: il ⚠️ del revisore sugli fps (`fps=1 min=0`) **non è una lacuna**: il pannello del browser tiene la pagina `hidden` e sospende `requestAnimationFrame`; gli fps si provano nel guscio vero | prova isolata del revisore (0 callback in 3 s); i cubi si muovono fra due screenshot | nessuno: la corsa del compito 4 li ha misurati (197–212) |
| 12 | **E6**: `interprocess` 2.4.3 nell'emettitore (lockfile di agosto su disco) resta; l'esito scrive la versione letta | ignorato ≠ assente; i due capi usano lo stesso nome della pipe | nessuno |
| 13 | **E7**: `node -e "require('electron')"` dopo `npm install` (Electron 44 senza `postinstall`) | misurato: `npm install` in 15 s senza `dist/` | nessuno |
| 14 | **E8**: `"signAndEditExecutable": false` in `electron/package.json` contro l'`EBUSY` di `editWindowsResources` (Defender) | l'opzione salta il passo che fallisce (winPackager.js:240); nessuna misura legge le risorse dell'eseguibile; due corse identiche escludono «riprova»; l'esclusione di Defender è un'impostazione di sicurezza, del proprietario | un eseguibile descritto «Electron» in Task Manager |
| 15 | **E9, E10**: le voci scritte a questa chiusura; il rimedio — A — pronto nel dispaccio e **non eseguito**, perché il proprietario ha chiuso senza scegliere fra A e B e la prossima sessione parte da lì | «una divergenza è una voce d'errata prima di essere un rimedio»; una modifica al metro del protocollo congelato si dichiara (vincolo 7), e il proprietario può ancora aggiungere B | una rimisura del compito 6 |
| 16 | i giri di correzione e i compiti sono implementatori **freschi** `sonnet` col rapporto come memoria (`SendMessage` assente, confermato con `ToolSearch`); un subagente in pausa su un comando in background **riparte da solo** | misurato due volte sul compito 5 (`tauri build`, il cancello) | nessuno |

#### Che cosa i revisori hanno trovato, e dove è finito

| Compito | Rilievo | Dove |
|---|---|---|
| 3 | Important plan-mandated: le chiavi italiane di `Wire` | decisione 10; **da portare al compito 8** nel richiamo alla §2 della stella polare (il contratto dell'emettitore è quello di `gui-ipc`) |
| 3 | ⚠️ fps non misurabili nel pannello del browser | decisione 11; misurati al compito 4 |
| 3 | Minor: `three` 0.185.1 senza `.d.ts` e nessun controllo dei tipi (P-8) | registro SDD, revisione finale |
| 3 | Minor: `p2mean` leggermente **negativo** col generatore locale (`Date.now()` in ms troncati contro `emesso_micros` sub-ms) | **da dire nell'esito del compito 6** accanto a P2: con l'emettitore vero il ritardo è positivo (5–10 ms medi misurati), con un errore di troncamento fino a 1 ms |
| 4 | Important: `vram_mb max = 0` in due corse — WebGPU sull'integrata | E9, E10 |
| 4 | il primo implementatore `BLOCKED` su `npm run dist` (`EBUSY`) | E8, giro di correzione 1, commit `8fc9696` |
| 5 | Approvato senza rilievi: sette file byte-identici al brief, tre corse di prova coerenti (`msgs=2000 lost=0 holes=0 src=tauri`, fps 188–215), `procs=7` costante — WebView2 dentro l'albero di Tauri —, titolo nativo dinamico visto nello screenshot e nel processo nello stesso istante, `gen/schemas` con `core:window:allow-set-title`; le tre note dell'implementatore (icone iOS/Android, avviso autocrlf, `attached to pid`) verificate innocue |

#### Le trappole di questa sessione — istruzioni, non aneddoti

- **un subagente con un comando lungo in background si mette in pausa** e la notifica arriva come «finished» con un testo del tipo «waiting for the notification»: **riparte da solo** alla fine del comando — non ridispacciare; un'attesa in background sul log (`until grep -q '^exit=' <log>; do sleep 10; done`) sveglia il coordinatore.
- **`SendMessage` non esiste** (ToolSearch lo conferma): il giro di correzione è un implementatore fresco che legge il rapporto, appeso allo stesso file.
- **`electron@44.3.0` non scarica il binario con `npm install`** (nessun `postinstall`): `node -e "require('electron')"` prima di lanciare `electron.exe` (E7).
- **`electron-builder` 26.15.3 va in `EBUSY` sull'eseguibile appena scritto** quando riscrive le risorse dopo l'integrità asar (Defender): `signAndEditExecutable: false` (E8); l'esclusione di Defender è del proprietario.
- **per guardare una finestra nativa** da un revisore: PowerShell `Add-Type -AssemblyName System.Windows.Forms,System.Drawing` e `CopyFromScreen` su `PrimaryScreen.Bounds`, PNG nello scratchpad, poi lo strumento Read; il titolo nello screenshot deve coincidere con `last title:` di `tree.ps1`.
- **il pannello del browser tiene la pagina `document.hidden=true`**: `requestAnimationFrame` sospeso, fps 0/1 — gli fps si misurano nei gusci.
- **questa macchina è un portatile con due GPU** (Intel UHD + RTX 4060 Laptop) e WebGPU va sull'integrata: `Dedicated Usage` è 0 per costruzione, i valori stanno in `Shared Usage` (E9, E10). `Win32_VideoController` prima di scrivere «la GPU» in un protocollo.
- **`cpu_pct max` sotto flusso è ~200 % di un core per entrambi i gusci**: la scena a ~200 fps domina; P3 si legge contro O2 e la chat nascosta, non contro il picco nudo.
- **`tauri icon` genera anche iOS e Android** (52 file): dentro `icons/*` della mappa, si committano. **`tauri build` dura ~6 minuti** la prima volta: in background con il log su file, e si aspetta la notifica.
- **in Tauri il titolo porta `ua=Chrome/152.0.0.0,Edg/152.0.0.0`** (WebView2 maschera la versione): Q2 si legge dal valore `pv` del registro, come il protocollo dice.
- **`interprocess` 2.4.3 nell'emettitore**: il `Cargo.lock` ignorato esiste su disco (E6); il guscio Tauri ha la 2.4.4 e `wry` 0.55.1 (non la 0.57.0 del registro: la tira `tauri` 2.11.5).
- **il pacchetto di revisione con un lockfile pesa ~150 KB** e `sonnet` lo regge se il dispaccio dice «scorrilo, non giudicarlo riga per riga».

#### Che cosa la sessione nuova fa, nell'ordine

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è il commit di questa chiusura o uno dopo.
2. La lettura obbligatoria di `CLAUDE.md`; di questo piano la testa, i vincoli, la posizione, «Come si esegue», l'**errata E1–E10**,
   il pre-controllo, le decisioni, la mappa dei file, questo diario; il compito 6 per intero solo al dispaccio, estratto per intestazione.
3. **La decisione su M5 col proprietario, A/B** (in cima a questo diario). Con A, o senza risposta: **il commit d'errata E9/E10** —
   su questa macchina col dispaccio pronto `task-6pre-dispatch.md` (implementatore `sonnet`, poi una revisione ristretta che rilancia la
   corsa di prova e legge `vram_shared_mb max` sopra zero); altrove lo si riscrive dalle voci E9 ed E10: sei sostituzioni in
   `tree.ps1` (il commento, l'inizializzazione `$vramShared = 0`, il blocco del contatore con i due `Get-Counter` e i due filtri
   sul `Path`, la riga del campione `vram_shared_mb`, il `Format-Table`, la sintesi con `{7}`), il richiamo E9 sotto «La macchina» e
   il richiamo E10 sotto la tabella M1–M5 del protocollo, messaggio `guscio(compito 6, errata E9 ed E10): …`. Con B: il proprietario
   imposta la preferenza grafica **prima** delle misure, e A resta consigliata comunque (le due colonne dicono quale GPU ha pagato).
4. **Il compito 6**, col pre-controllo rifatto contro il codice di adesso (`task-6-brief.md` è già estratto per intestazione): i tre
   eseguibili esistono su questa macchina; l'esito scrive le GPU lette e l'adattatore di `api=` (E9), entrambe le colonne di M5 (E10),
   la nota su `p2mean` (decisione della revisione del 3), la versione di `interprocess` letta nei due lockfile (E6); l'`awk` di P-9 sul
   filtro delle istanze **non è più l'indiziato** di un `vram_mb` a zero. Poi il compito 7 col proprietario, poi l'8 — che porta a
   `HANDOFF.md` i gotcha di questo diario e della quarta chiusura, alla §2 della stella polare il richiamo su `dockview` (CSS) e sul
   contratto di `gui-ipc` (decisione 10), alla §4 quello di E1.
5. Alla chiusura di ogni sessione: questo diario, la memoria dell'agente, `session-handoff`.


### La quarta chiusura — 2026-09-10: i compiti 1 e 2 ESEGUITI, il compito 3 pre-controllato — si riprende dal compito 3

⛔ **DA SAPERE SUBITO.** Niente è a metà nel repository: albero pulito, nessuno stash, nessuna operazione git in corso,
nessun server acceso (la porta 5173 è libera), nessun codice di prodotto toccato. Il proprietario ha chiuso con
`session-handoff` («dopo questa task») mentre il compito 3 era **pre-controllato e non dispacciato**: si riprende da lì.
Due cose in cima, sue: (1) il commit `5dc9c6f` porta un trailer `Co-Authored-By` contro `CLAUDE.md` («senza co-autore») —
toglierlo vuole `git commit --amend` e un push forzato di `main`, che nessuna sessione fa da sola; (2) la sua domanda alla
chiusura — *questa UI è la base di quella finale?* — ha la risposta nella testa di questo piano e nella §3 della stella
polare: **no**. È lo spike **SP-8** in `spikes/gui-shell/`, fuori dal workspace: misura il guscio (M1–M5, Q1–Q4) e fa
giudicare `dockview` (le otto mosse). La SPA del sotto-progetto 2 nasce nella **parte 2**, in `gui/`, con lo stack della
§2 del disegno del 2; dello spike sopravvivono le misure, i verdetti, il protocollo e l'errata, non il codice.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main` = `origin/main`: `git fetch --all --prune`, poi `git status -sb` → `## main...origin/main`, niente sotto |
| I commit di questa sessione | `git log --oneline 07b984c..HEAD` — **cinque** più questo: `9d869a7` (compito 1), `5dc9c6f` (E1), `01694e3` (compito 2), `ce84042` (E2 ed E3), `c8a5ac8` (E4), poi la chiusura |
| Codice di prodotto, cancello, CI, gli spike vecchi | **non toccati**: `git diff --stat 5ad4634..HEAD -- crates/ scripts/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml spikes/gesti/ spikes/gui-ipc/` non rende nulla |
| La posizione e l'errata | i compiti **1** e **2** a `✅ 2026-09-10` nella tabella della posizione; l'errata **E1–E5**, tutte del 2026-09-10 |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN` prima di **ogni** commit (i log nello scratchpad portano la data, e il revisore la legge contro `git log -1 --format=%ci`) e alla chiusura; `bash scripts/check-docs.sh` → `OK` |
| Fine-riga | i file dell'app, il protocollo e il piano **LF**: `tr -cd '\r' < <file> \| wc -c` → `0`; `.gitignore`, roadmap e compendio CRLF con CR = righe |
| Margine del compendio | il comando del vincolo 11: `10362` prima del richiamo di questa chiusura; le righe `⏭️` restano **tre** |
| Il registro di esecuzione | `.superpowers/sdd/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio/progress.md` su **questa** macchina, ignorato da git, coi brief, i dispacci, i rapporti e i pacchetti di revisione dei compiti 1–2; su un'altra macchina non c'è, e **questo diario basta** |

#### Le decisioni prese eseguendo, nell'ordine

| # | Decisione | Perché | Costo se sbagliata |
|---|---|---|---|
| 1 | si esegue su `main`, commit e push a ogni compito, senza co-autore, nessun worktree | `CLAUDE.md`, vincolo 13 | nessuno |
| 2 | la scansione preliminare dei compiti 2–8 è la revisione della terza sessione; il pre-controllo si rifà a ogni dispaccio contro il codice di adesso | questo diario vieta di leggere i compiti tutti insieme | un difetto scoperto al dispaccio diventa errata |
| 3 | **E1**: la clausola «EQUAL / DIFFERENT» della mossa 7 resta nel protocollo; la frase che apre la tabella delle mosse lo dice; la §4 della stella polare riceve il richiamo al compito 8 | il protocollo non era ancora congelato; la §4 è la casa del perché | un richiamo da anticipare |
| 4 | il trailer in `5dc9c6f` **resta** finché il proprietario non decide | un push forzato di `main` è irreversibile e verso l'esterno | un trailer in un commit di documenti |
| 5 | **E2**: `dockview` 8.2.0 entra **solo per il CSS** (`import 'dockview/dist/styles/dockview.css'`); l'API resta `dockview-core`, il ponte resta nostro | `dockview-core` non spedisce CSS; `dockview` ha 583 byte di JS, il foglio, e nessun'altra dipendenza | il richiamo alla §2 della stella polare al compito 8 |
| 6 | **E3**: `defaultTabComponent: 'bigtab'` nelle opzioni di `createDockview` | senza, `createTabComponent` non è mai chiamato e la presa grande non esiste | nessuno |
| 7 | **E4**, modifica al metro dichiarata: la mossa 7 si misura sul JSON **canonico** e scrive anche il grezzo; richiamo datato nel protocollo congelato | tre comportamenti di `dockview` misurati: l'ordine delle chiavi di `panels`, +2 px per gruppo galleggiante a ogni ripristino, i minimi sui viewport stretti | il proprietario può rifiutare il canonico: il grezzo resta stampato |
| 8 | **E5**: il `main.ts` del compito 3 tiene la prima riga dell'import del CSS | riga 5 del pre-controllo: il testo dettato è di prima di E2 | nessuno |
| 9 | i modelli: implementatori e revisori `sonnet`, mai `haiku`; il revisore dei compiti con codice apre il browser | i turni contano più del prezzo, e le regole del repository sono fitte | il costo per compito |

#### Che cosa i revisori hanno trovato, e dove è finito

| Compito | Rilievo | Dove |
|---|---|---|
| 1 | il cancello **non** era stato rilanciato dopo le modifiche: il rapporto citava il log di apertura del coordinatore (09:24) per un commit delle 09:37 | giro 1: due corse fresche, il rapporto corretto; da allora ogni dispaccio pretende un log **nuovo** con la data |
| 1 | la mossa 7 del protocollo non era «parola per parola» la §4 | E1 |
| 2 | `createTabComponent` mai invocato; `dockview-core` senza CSS; la mossa 7 `DIFFERENT` | E3, E2, E4 — verificati nel browser dal ri-revisore, due giri |
| 2 | minor rimandato: `canonical()` prima di `firstDivergence()` è ridondante in `home.ts` | alla revisione finale del ramo |

#### Le trappole di questa sessione — istruzioni, non aneddoti

- **un subagente cita un log vecchio come prova**: il dispaccio dà il nome di un log **nuovo**, e il revisore ne legge la data con `ls -la --time-style=full-iso` contro `git log -1 --format=%ci`; un `GATE GREEN` senza data non è una prova.
- **il promemoria di attribuzione dell'harness chiede un `Co-Authored-By`**, e un subagente lo ha preferito a `CLAUDE.md`: ogni dispaccio dice esplicitamente che `CLAUDE.md` prevale.
- **`task-brief` legge `Task N`, non `Compito N`**, e le righe del piano scivolano a ogni riga d'errata: il brief si estrae **per intestazione** — `awk '/^## Compito 3:/{f=1} /^## Compito 4:/{exit} f' <piano>` — mai per numero di riga.
- **`git status --porcelain` accorpa una cartella non tracciata**: le attese sui file nuovi si leggono con `-uall`.
- **il browser dell'automazione**: `left_click_drag` non muove il DnD di `dockview` sotto nessuna strategia, `Ctrl+Alt+frecce` è intercettato, `window.open` naviga la scheda stessa — le mosse 2, 4 e 6 si giudicano con la mano del proprietario al compito 7. Il viewport predefinito è 1024×768: le misure di layout si fanno a **1920×1080** (`resize_window`), o i minimi di `dockview` falsano il ripristino.
- **una scheda già aperta può essere agganciata a un'anteprima locale** e rifiutare `navigate`: `tabs_create`, poi il `tabId` su ogni azione; il server di Vite si avvia in background su percorso assoluto e si **ferma** (`Get-NetTCPConnection -LocalPort 5173 -State Listen`), o il prossimo `npm run dev` va sulla 5174.
- **il subagente non si riprende** (`SendMessage` assente in questo harness): i giri di correzione sono implementatori freschi che leggono il rapporto precedente; il rapporto, appeso, è la memoria.
- **`dockview` 8.2.0, misurato**: niente CSS in `dockview-core` (l'unica `createElement("style")` non è un'iniezione); `createTabComponent` scatta solo con `defaultTabComponent` o `tabComponent`; `toJSON` dopo `fromJSON` riordina `panels`, allarga di 2 px un gruppo galleggiante a ogni giro, impone i minimi (100 px per pannello) su un viewport stretto; `dockview` 8.3.0 e `three` 0.186.0 sono al registro, le appuntate restano (vincolo 8).

#### Che cosa la sessione nuova fa, nell'ordine

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è il commit di questa chiusura o uno dopo.
2. La lettura obbligatoria di `CLAUDE.md`; di questo piano la testa, i vincoli, la posizione, «Come si esegue», l'**errata E1–E5**,
   il pre-controllo, le decisioni, la mappa dei file, questo diario; il **compito 3** per intero solo al dispaccio, estratto per intestazione.
3. Il pre-controllo del compito 3 è **fatto** (E5; le sonde del Passo 1 rilanciate il 2026-09-10 danno l'atteso; `Messaggio` di
   `spikes/gui-ipc` ha i quattro campi di `Wire`; `three` 0.185.1 espone `./webgpu`): si rifanno le sonde, poi si dispaccia con lo
   schema dei compiti 1–2 — brief e dispaccio in file, niente lettura d'apertura, log del cancello nuovo, commit senza trailer; il
   revisore apre `npm run dev` a 1920×1080 e legge il titolo (dodici campi `chiave=`), la chat con la provenienza, la scena
   (`api=WebGPU:…` o `WebGL2:…` è una misura), la tessera Mano («relay not reachable»); la mossa 8 è del compito 7.
4. Poi i compiti 4, 5, 6 (le misure), 7 (col proprietario), 8 (la chiusura, che porta a `HANDOFF.md` i gotcha di questo diario,
   alla §2 della stella polare il richiamo su `dockview` come sorgente del CSS, alla §4 quello di E1).
5. Alla chiusura di ogni sessione: questo diario, la memoria dell'agente, `session-handoff`.

### La terza chiusura — 2026-09-10: la revisione del piano intero FATTA, i tre spostamenti di stato FATTI — si ESEGUE

⛔ **DA SAPERE SUBITO.** Niente è a metà, **nemmeno questo file**: il piano è scritto per intero e **riletto** — la revisione
del punto 4 della seconda chiusura (copertura dei disegni, segnaposto, nomi fra i compiti, ogni *Trova* rilanciato) e i tre
spostamenti di stato del punto 5 sono **fatti** in questa sessione, con la data vera; ciò che la revisione ha corretto è
scritto **nei compiti** (la tabella qui sotto), l'errata resta vuota perché il piano non è eseguito. Albero pulito, nessuno
stash, nessuna operazione git in corso, **nessun codice toccato**. Lo scratchpad è ripulito: tutto ciò che vale sta qui e nel
commit. ⛔ **L'esecuzione — compito 1 con un subagente fresco — è di una sessione NUOVA** (decisione 48 della stella polare).

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main`, allineato a `origin`: `git fetch --all --prune`, poi `git status -sb` → `## main...origin/main`, niente sotto |
| I commit delle tre sessioni del piano | `git log --oneline 5ad4634..HEAD` — **tre**: `b50e8b8`, il piano a metà; `4d626ea`, i compiti 6–8; poi questo, la revisione e gli spostamenti di stato |
| Codice, cancello, CI, spike | **non toccati**: `git diff --stat 5ad4634..HEAD -- crates/ scripts/ .github/ spikes/ Cargo.lock Cargo.toml rust-toolchain.toml` non rende nulla |
| Documenti toccati da questa sessione | `git diff --stat 4d626ea..HEAD -- docs/` → questo piano, `docs/roadmap.md` (la riga del piano nella tabella dei piani e la riga 6), il disegno del 2 (la §10, il richiamo «PIANO DELLA PARTE 1 SCRITTO») e `docs/COMPENDIO.md` (il richiamo nel punto 2 del prossimo passo della §6, da ⏳ a ✅) — i tre spostamenti di stato |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN` all'apertura (baseline, letto dal log nello scratchpad) e alla chiusura, prima del commit; `bash scripts/check-docs.sh` → `OK`. Si rilanciano, non si citano |
| Fine-riga | questo piano e il disegno del 2 **LF** — `tr -cd '\r' < <file> \| wc -c` → `0`; roadmap e compendio CRLF con CR = righe, `git ls-files --eol` → `i/lf w/crlf`, invariato |
| Margine del compendio | il comando del vincolo 11: `10313` all'apertura di questa sessione — **non** i `10395` di P-22: il richiamo della seconda chiusura era più lungo di quello della prima, non «della stessa taglia» — e il valore alla chiusura lo dà il comando; il compito 8 lo rimisura comunque (D18) |
| Le righe `⏭️` del compendio | `grep -c '⏭️' docs/COMPENDIO.md` → `3` prima e dopo lo spostamento (c): il richiamo riscritto è ✅, senza marcatore (P-20, D24) |
| Debito lasciato | **dichiarato**: le righe **dedotte** e non provate — P-3, P-6, P-7, P-21, P-25 — restano candidate all'errata, non voci; niente altro |

#### Che cosa la revisione ha trovato, e dove è corretto — nei compiti, non nell'errata

Le quattro dimensioni del punto 4 della seconda chiusura, più i tre spostamenti di stato letti contro il giorno in cui si
scrivono. Il metodo: la lettura d'apertura per intero, poi la stella polare («Il modello della GUI», le §2, §3, §4, le decisioni,
le registrate, i vicoli ciechi; la tabella dello stato e delle decisioni a 50 righe), il disegno del 2 (la testa, le §1, §2, §6a,
§8, §9, §10, i vicoli ciechi), poi questo piano per intero a 200 righe per chiamata — 17 blocchi in parallelo, il più grande
26,6 KB (righe 401–600); poi **due revisori in sola lettura in parallelo**, uno sui compiti 1–5 e uno sui compiti 6–8 col
diario, coi prompt salvati in file nello scratchpad, che hanno rilanciato ogni *Trova* col `grep -c` e ogni sonda dei «Passo 1»,
e i cui rapporti il coordinatore ha verificato a campione coi propri `grep` lanciati **prima** di dispacciarli.

| # | Dove | Che cosa diceva | Misurato il 2026-09-10 | Rimedio, nel compito |
|---|---|---|---|---|
| 1 | compito 6, Passo 7; compito 7, Passo 1 | «le righe con `⏳` sono **otto** — l'intestazione, …»; `Atteso: 8` | l'`awk` di delimitazione fa `{s=1;next}` sull'intestazione, quindi **non la stampa**: sul modello estratto dal piano il conteggio rende **7**, e 8 solo col `grep` sul modello intero | attesa **sette** in entrambi, con la ragione scritta; il compito 7 consuma comunque tutte e otto le righe (l'intestazione col proprio *Trova*) |
| 2 | compito 7, Passo 8, seconda riga | «le **due righe** del capoverso d'apertura che portano … e …» | nel modello del compito 6 quel capoverso è **una** riga (riga 2487 del piano) | *Trova* riscritto: la riga che comincia con `⏳ **Le otto mosse, …`, intera, presa dal file |
| 3 | compito 8, Passo 8, blocco 2 | `grep -c '^\| M[1-5] \|\|^\| Q[1-4] \|' … # 9` | «Come si chiude» di ADR-0029 porta **già oggi** quattro righe `\| M1 \|`…`\| M4 \|`: dopo la *Decision* il conteggio farebbe 13 | il `grep` **delimitato** a `## Decision` … `### Come si chiude` con l'`awk`, atteso 9 |
| 4 | «Come si riprende», punto 5 della seconda chiusura; compito 8, Passi 3 e 4; compito 1, Passo 3 | i testi fissati portavano **2026-09-09** per la sessione che completa il piano, e la riga della roadmap diceva «in **due** sessioni … la seconda i compiti 6–8 **e la revisione**»; il compito 1 diceva che la riga del piano in roadmap «era arrivata il 2026-09-09» | la sessione che completa è del **2026-09-10** ed è la **terza**; la riga in roadmap la scrive lei | i tre testi scritti con la data vera e «tre sessioni»; i *Trova* del compito 8 (il richiamo del compendio, la cella della roadmap) e il testo del compito 1 allineati **nello stesso commit**. È il gotcha #57: una data scritta prima del giorno è una previsione |
| 5 | P-22 | «il richiamo riscritto è della stessa taglia» | margine `10313` all'apertura contro i `10395` di P-22: 82 byte in più | nessuno: registrato qui; ogni compito rimisura (D18) |
| 6 | compito 4, Passo 5; compito 5, Passo 4; compito 6, Passo 4; compito 7, Passo 6; compito 8, Passo 1 — otto siti | `tree.ps1 … \| tail -6` (e `-4`, `-3`, `-2`) con l'attesa «`emitter started at …`, `rest:`, `stream:`, `last title:`» | `tree.ps1` scrive `emitter started` **durante** il ciclo e poi stampa la tabella di **tutti** i campioni — uno ogni 250 ms — prima delle righe di sintesi: dopo un `tail` la riga dell'emettitore non c'è mai (revisore 1, letto nel codice dettato) | negli otto siti `\| grep -E '^(shell started\|attached to pid\|emitter started\|rest:\|stream:\|last title:)'`: le sole righe che le attese leggono |
| 7 | compito 3, criterio di chiusura | «il titolo porta le **sette** misure» | i campi `chiave=` dell'interfaccia e di `stats.line()` sono **dodici**; D6 ne elenca otto; nulla ne conta sette (revisore 1) | «i dodici campi `chiave=`», elencati |
| 8 | la mappa dei file, la riga del guscio Tauri | non nomina `tauri/icon.png` | il compito 5 lo crea con `icon.py`, `git add spikes/gui-shell/tauri` lo committa e `.gitignore` non lo ignora: il vincolo 9 («solo file della mappa») sarebbe violato dalla **mappa**, non dal compito (revisore 1) | `icon.png` aggiunto alla riga |
| 9 | compito 7, Passo 4 e Passo 8 — la mossa 8 | «la barra scrive `move 8: pointerdown … on div.bigtab`» | `hand.ts` scrive `tag.classe` dell'elemento di `elementFromPoint`, e `.bigtab-title { flex: 1 }` riempie la presa: sul titolo esce `span.bigtab-title`, sui tre comandi `button.`, che fermano la propagazione (revisore 1, dedotto dal CSS e dal DOM) | l'attesa dice entrambe le forme, e che `on button.` non è il no tecnico di P-6 |
| 10 | P-9, un dedotto | «la forma delle istanze del contatore, `pid_<n>_luid_…`» | **misurato** il 2026-09-10 dal revisore 1 con `Get-Counter -ListSet`: `pid_10364_luid_0x00000000_0x000183A5_phys_0`, il filtro `^pid_(\d+)_` regge | il richiamo in P-9: da dedotto a misurato |
| 11 | compito 8, Passo 9 — il `grep` di `Proposed` filtrato | «rende **solo** righe con un richiamo datato, o la riga 29 dell'audit» | due case tengono `Proposed` su una riga **senza** parola del filtro, col richiamo poche righe sotto: la testa della voce 0029 della §5 del compendio (riga 454 oggi) e «Nessuna voce aperta resta nella spec» di `HANDOFF.md` (155) (revisore 2, rilanciato) | il filtro toglie anche quelle due righe, e l'attesa dice perché |
| 12 | compito 8, Passo 9 — l'`awk` delle tabelle spezzate | «non stampa nulla» | stampa **già oggi** sei righe sui file non toccati: quattro sono due tabelle consecutive con una riga vuota in mezzo (compendio 883; riferimenti 108, 1740, 2395), due sono tabelle **spezzate preesistenti** — `HANDOFF.md` 1153–1155, i gotcha 49 e 50, e `riferimenti.md` 103–105 (revisore 2, rilanciato) | il ciclo si lancia anche al Passo 1 e il Passo 9 pretende **lo stesso output**; le due spezzate sono **del proprietario** — registrate qui e nel rapporto, non toccate |
| 13 | compito 8, Passo 8, blocco 2 — il commento | «the heading alone: no file listed» | `check-docs.sh` stampa `  (none)` sotto l'intestazione quando nessun ADR è `Proposed` (P-24, riga 306 dello script) (revisore 2) | il commento corretto |
| 14 | «Come si riprende», punto 4 (b) della seconda chiusura | «`grep -n 'TBD\|TODO\|da scrivere'` non rende nulla» | rende la riga del comando e la 2411, dove «da scrivere» è prosa («una divergenza da scrivere, non da spiegare»): nessun segnaposto (misurato due volte, coordinatore e revisore 2) | nessuno: il punto (b) è verificato così, e detto qui |

#### Che cosa la sessione nuova fa, nell'ordine — l'ESECUZIONE

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è il commit di questa chiusura o uno dopo; se è
   uno dopo, il «prossimo passo» della §6 del compendio si rilegge contro `HEAD` di adesso.
2. La lettura obbligatoria di `CLAUDE.md` — il compendio a blocchi di 200 righe, i due pezzi dell'audit — poi di **questo piano**
   la testa (righe 1–99: strumenti, `replace_unique.py`, la regola sui *Trova* presi dal file), i vincoli globali, la posizione,
   «Come si esegue un compito», l'errata, il pre-controllo P-1…P-25, le decisioni D1…D25, la mappa dei file, le voci aperte;
   **non** i compiti tutti insieme — ogni compito si legge quando si dispaccia (regola 1 di «Come si esegue»), coi disegni nelle
   sezioni che nomina. Le tabelle lunghe: 200 righe per chiamata reggono su tutto il piano (i byte per blocco si misurano prima).
3. `superpowers:subagent-driven-development` dal **compito 1**: un subagente fresco per compito, col prompt salvato in un file
   dello scratchpad **prima** di dispacciarlo (un subagente in background muore col processo); prima di ogni dispaccio il
   pre-controllo del compito si **rilegge** contro il codice di adesso — le quattro domande di `CLAUDE.md` e le tre righe in più —
   e il primo difetto trovato è una voce d'**errata** col proprio numero; il revisore rilancia ogni comando accanto a
   un'affermazione misurabile e li elenca (regola 5), e per i compiti 2–5 apre l'app nel browser o lancia il guscio e guarda.
4. Il compito 7 lo esegue il **coordinatore col proprietario** allo schermo (regola 7); su un no all'insieme delle mosse, il
   compito 7-bis si scrive in quella sessione (D12) prima dell'8; il compito 8 chiude e scrive il puntatore della §6 del compendio.
5. A ogni compito: `check-docs.sh` e `gate.sh` prima del commit, la posizione aggiornata nello stesso commit, i fine-riga
   rimisurati, push, senza co-autore; alla chiusura di ogni sessione «Come si riprende» riscritto come diario, la memoria
   dell'agente, `session-handoff`.

### La seconda chiusura — 2026-09-09: i compiti 6–8 SCRITTI, la revisione e gli spostamenti di stato NO

✅ **RICHIAMO DEL 2026-09-10:** ciò che qui manca è **fatto** — la terza chiusura qui sopra. Il testo resta com'era.

⛔ **DA SAPERE SUBITO.** Niente è a metà **nel repository**: albero pulito, nessuno stash, nessuna operazione git in
corso, nessun codice toccato. A metà è ancora **questo file**, ma meno della prima volta: i compiti **6, 7 e 8** sono
scritti per intero nella forma dei compiti 1–5 — ogni *Trova* rilanciato col `grep` il 2026-09-09 contro `b50e8b8`, ogni
attesa contata — il pre-controllo P-19…P-25 e le decisioni D19–D25 stanno nelle loro tabelle, la mappa dei file è
aggiornata, «Dopo il compito 8» è scritto. ⛔ **Mancano, e li fa la sessione nuova PRIMA di eseguire:** la **revisione del
piano intero** (punto 4 qui sotto) e i **tre spostamenti di stato** (punto 5). Il proprietario ha chiuso con *«fermati, si
continua nella prossima sessione»*. Lo scratchpad è ripulito: tutto ciò che vale sta qui e nel commit.

| | Stato alla chiusura, e il comando che lo rifà |
|---|---|
| Ramo | `main`, allineato a `origin`: `git fetch --all --prune`, poi `git status -sb` → `## main...origin/main`, niente sotto |
| I commit delle due sessioni del piano | `git log --oneline 5ad4634..HEAD` — **due**: `b50e8b8`, il piano a metà; poi questo, coi compiti 6–8 |
| Codice, cancello, CI, spike | **non toccati**: `git diff --stat 5ad4634..HEAD -- crates/ scripts/ .github/ spikes/ Cargo.lock Cargo.toml rust-toolchain.toml` non rende nulla |
| Documenti toccati da questa sessione | `git diff --stat b50e8b8..HEAD -- docs/` → questo piano, e `docs/COMPENDIO.md` per il richiamo nel punto 2 del prossimo passo della §6, riscritto allo stato di questa chiusura |
| Cancello | `bash scripts/gate.sh` → `GATE GREEN` all'apertura (baseline, letto dal log) e alla chiusura, prima del commit; `bash scripts/check-docs.sh` → `OK`. Si rilanciano, non si citano |
| Fine-riga | questo piano **LF** — `tr -cd '\r' < docs/superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md \| wc -c` → `0`; il compendio CRLF con CR = righe, `git ls-files --eol docs/COMPENDIO.md` → `i/lf w/crlf` |
| Margine del compendio | il comando del vincolo 11: `10395` all'apertura di questa sessione — il richiamo della prima chiusura aveva consumato 453 dei `10848` — e il richiamo riscritto è della stessa taglia (P-22) |
| Debito lasciato | **dichiarato**: la revisione finale e i tre spostamenti di stato, qui sotto; le righe **dedotte** e non provate — P-3, P-6, P-7 della prima sessione, P-21 e P-25 di questa — restano candidate all'errata, non voci |

### Che cosa la sessione nuova fa, nell'ordine — ✅ i sei passi della seconda chiusura, ESEGUITI il 2026-09-10

I sei passi com'erano — la lettura, il pre-controllo dei compiti 6–8 riletto, la revisione nelle quattro dimensioni, i tre
spostamenti di stato coi testi fissati, il cancello e il commit — stanno parola per parola in
`git show 4d626ea:docs/superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md`; qui non si ricopiano (gotcha
#68). I tre testi fissati al punto 5 sono **scritti** nei tre file — con la data vera, il 2026-09-10, e «tre sessioni» (rilievo 4
della terza chiusura) — e vivono lì e nei *Trova* del compito 8, non qui. L'ordine per la sessione che **esegue** è nella terza
chiusura, qui sopra.

### La prima chiusura — 2026-09-09, il piano a metà: le sue decisioni e le sue trappole

La prima sessione si è chiusa per contesto saturo dopo il compito 5 e ha committato il piano a metà (`b50e8b8`), col
richiamo nel punto 2 della §6 del compendio. Le sue **tracce** dei compiti 6–8 sono **assorbite** nei compiti e tolte da
qui, perché una seconda casa diverge (gotcha #68): il testo com'era in
`git show b50e8b8:docs/superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md`. Le sue tre decisioni, e i
suoi vicoli ciechi, restano:

| | Decisione | Perché |
|---|---|---|
| a | il piano è **committato a metà**, in un file tracciato, col puntatore della §6 che lo dice | il proprietario ha chiuso all'improvviso; lo scratchpad è effimero e si lavora da due macchine (voce E43 del piano del Traguardo 6); il precedente è il punto fermo del brainstorming del 2. Costo: un commit in più, e la testa che dice «A METÀ» finché la sessione nuova non lo completa |
| b | i tre spostamenti di stato «piano scritto» (roadmap, §10 del 2, §6 del compendio) **non** sono fatti: si fanno a piano completo | uno stato «scritto» su un piano a metà mentirebbe; il richiamo «A METÀ» nella §6 basta a farlo trovare |
| c | nessuna riga nuova in `riferimenti.md` per le verifiche di oggi | sono verifiche di **API** (le firme di `dockview-core`, i sorgenti di `three`, la crate `interprocess`, lo schema di Tauri), non misure di stato dell'arte; stanno nel pre-controllo (P-1…P-9) con la data. Se il proprietario le vuole anche lì, è una riga del compito 8 |

- la testa del piano dei gesti letta col tool Bash a 190 righe **trabocca** (35 KB): si legge 1–83, 84–137, 138–186; questo piano è più grande — misurare i byte per blocco prima.
- `dockview-core` 8.2.0 non spedisce CSS (P-1): un `import` del foglio di stile sarebbe stato il primo rosso; verificato aprendo il tarball del registro in memoria con `tarfile`, che è il metodo che regge anche per le firme `.d.ts`.
- i file dei permessi di Tauri su GitHub (`crates/tauri/permissions/…`) rendono 404 dal ramo `dev`: il permesso `core:default` resta dedotto (P-7); la pagina docs.rs di `GenericNamespaced` è 404, il **sorgente della crate** da `static.crates.io` risponde e dice `\\.\pipe\` (P-5); `cdn.jsdelivr.net` serve i sorgenti di `three` per file (P-6).
- il piano si scrive in **frammenti** col tool `Write` (uno per sezione, sotto i 25 KB) e si unisce con `cat`: un `Write` solo non regge; i frammenti restano nello scratchpad e **muoiono con la sessione** — un punto fermo si committa prima che il contesto sia saturo, non dopo.
- la §10 del 2 chiede al piano di **copiare** la Definizione di «fatto»: il compito 8 la copia per intero, non la rimanda.

### Vicoli ciechi e trappole della seconda sessione

- `bash scripts/gate.sh > "$TMPDIR/gate.log"` dal tool Bash: `$TMPDIR` è **vuoto** in questa Git Bash, la redirezione va in `/gate.log` e fallisce con *Permission denied* — il cancello non gira. Il percorso dello scratchpad si scrive per esteso, assoluto.
- le righe `⏭️` del compendio sono **tre**, non due: la terza è un code span nella tabella delle voci aperte (riga 735 il 2026-09-09). L'attesa «due» delle tracce era **non contata**: ora P-20 e D24.
- la tabella dello stato e delle decisioni della stella polare (righe 44–143) letta col tool Bash a **100** righe trabocca (43,8 KB, salvata su file e persa): 50 righe, o 30 sulla sola tabella delle decisioni, com'era già scritto nei suoi vicoli ciechi.
- questo piano letto a 200 righe per chiamata regge su tutti i blocchi, tredici chiamate in parallelo.
- i frammenti dei compiti scritti col tool `Write` nello scratchpad, sotto i 25 KB l'uno, e uniti con uno script Python che li legge — non con un `Write` solo. Una recinzione a tre accenti che contiene un'altra recinzione a tre accenti si chiude prima del tempo: la esterna va a **quattro** (compito 8, Passo 6).
- un comando con una barra verticale dentro una cella di tabella è una trappola doppia — `\|` per la tabella, che l'esecutore copierebbe dentro un'espressione regolare: i comandi con `|` stanno nei blocchi di codice, non nelle celle (compito 8, Passo 8, riscritto).
- il proprietario ha chiuso la sessione mentre si scriveva la sezione «Dopo il compito 8»: la chiusura è costata più della scrittura, perché il piano riferisce P-19…P-25 e D19–D25 e senza le tabelle i riferimenti sarebbero rimasti appesi. La lezione è quella della prima chiusura: il punto fermo si committa **prima**.

### Vicoli ciechi e trappole della terza sessione

- l'`awk '/^## SP-8 /{s=1;next} s&&/^## /{s=0} s'` **non stampa l'intestazione** che apre la sezione: un'attesa che la conta è
  sbagliata di uno. Un'attesa su una sezione futura si misura sul **modello** estratto dal piano —
  `awk 'NR>=<riga>{ if(/^```/){exit} print }' <piano> > modello.md`, poi lo stesso comando del compito sul modello — e non a mente.
- un testo **fissato** per una sessione futura non porta la data del giorno in cui si scrive: porta `<data>`, o dice che la
  sessione lo ridata. I tre testi del punto 5 della seconda chiusura portavano il 2026-09-09 e la sessione è arrivata il
  2026-09-10 (gotcha #57: una previsione citata come misura); i *Trova* del compito 8 che li ricopiavano andavano allineati.
- un `\| tail -N` su uno script che stampa una **tabella per campione** prima delle righe di sintesi nasconde tutto ciò che lo
  script ha scritto durante il ciclo: si filtra con `grep -E` sulle righe che l'attesa nomina, non con `tail`.
- una sonda `grep -c` sull'ADR intero conta anche le righe che l'ADR ha **già** nella *Context*: la sonda della Definizione di
  «fatto» si scrive **delimitata** alla sezione che il compito produce, come l'`awk` di `RISULTATI.md`.
- una sonda «il `grep -v` delle parole del richiamo non rende nulla» è **per riga**: una casa il cui richiamo sta poche righe
  sotto resta nell'output; e l'`awk` delle tabelle spezzate ha falsi positivi su due tabelle consecutive e coglie spezzature
  **preesistenti**. Ogni sonda «non rende nulla» si lancia **prima**, sui file di adesso, e la sua baseline si scrive nel Passo 1.
- questo piano letto a 200 righe per chiamata regge su tutti i 17 blocchi, in parallelo, coi byte per blocco misurati prima
  (`awk '{b+=length($0)+1} NR%200==0{print NR, b; b=0}'`): il più grande 26,6 KB. La stella polare a 50 righe sulla tabella
  dello stato e delle decisioni (22–23 KB); le altre sezioni prescritte stanno sotto i 20 KB.
- due revisori in parallelo per perimetro — compiti 1–5; compiti 6–8 e il diario — in sola lettura, coi prompt salvati in file
  nello scratchpad e con l'ordine di **non** fare la lettura d'apertura; il coordinatore lancia **prima** una decina di `grep -c`
  sui *Trova* come baseline, e verifica i rapporti a campione contro quella.
