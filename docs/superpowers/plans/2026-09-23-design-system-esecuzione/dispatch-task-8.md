Sei l'**implementatore del compito 8** — *la cornice: la barra col nome della vista, la Panoramica, la striscia a pillola* —
del piano `docs/superpowers/plans/2026-09-23-design-system.md`, nel repository `E:\ALL\DEV\MY_REPOS\daemon` (Windows; il tool Bash è Git Bash, dove è `/e/ALL/DEV/MY_REPOS/daemon`). Sei un subagente fresco: tutto ciò che
ti serve è qui e nel brief che questo prompt nomina. Il compito è **codice della GUI** — due file nuovi, quattro riscritti per
intero, quattordici toccati — e **nessuna cella** del piano (**E101**).

**All'avvio verifichi, e se non torna ti fermi e lo riporti:** `git rev-parse --short HEAD` → `bc6e94d`;
`git status --porcelain` → vuoto; `git log --oneline f3206c7..HEAD -- gui/` → nulla; `node --version` → una versione che
`gui/package.json` accetta (`engines`); `git config --show-origin --get-all core.autocrlf` → il valore di questa macchina,
nel §0; Google Chrome stabile installato, e la sua versione **letta dal nome della cartella** —
`ls "/c/Program Files/Google/Chrome/Application/" "$LOCALAPPDATA/Google/Chrome/Application/" 2>/dev/null` rende almeno una
cartella di versione, come `154.0.8037.58`; se una delle due cartelle non esiste il comando esce **2** pur stampando la
versione: conta la cartella, non l'uscita. ⛔ **Mai `chrome.exe --version`**: su Windows apre il browser col profilo
dell'utente e non stampa nulla (lezione 4 della consegna dell'esecuzione del compito 3, in archivio).

## 0. La macchina

Il repository vive in **due cloni**, e ciò che cambia fra i due si **misura**, non si copia da un'etichetta: voce **E72**
del piano della parte 2, ed **E1** di questo piano.

| | macchina `Jays` | macchina `zagor` |
|---|---|---|
| il repository | `E:\ALL\DEV\MY_REPOS\daemon` | `C:\Users\zagor\Desktop\harness` |
| `core.autocrlf` | `false` in `.git/config`, quindi l'albero è `w/lf` — `--get-all` rende più righe, e vale l'ultima | `true` dal file di sistema: l'albero è `w/crlf` per i file che git ha scritto, e `w/lf` per quelli nati o riscritti LF su quella macchina |
| Google Chrome | `154.0.8037.58`, riletta dal nome della cartella il 2026-09-27 dal coordinatore, alla ripresa: si rilegge | si rilegge: si aggiorna **da sé** |
| Node | `v24.19.0`, il 2026-09-27 | non misurata da questa macchina |

---

## 1. Che cosa leggi, e che cosa no

La cartella di lavoro è `.superpowers/sdd/2026-09-23-design-system/`, git-ignorata; il brief ci nasce da
`_extract_brief_8.py`, nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`:

| File | Che cos'è |
|---|---|
| `task-8-brief.md` | **il compito**: la testa del piano (obiettivo, architettura, pila, disegno, **strumenti** con `replace_unique.py`), i *Vincoli globali*, *A che punto è* e *Come si esegue un compito*, l'**errata** — **E91**…**E101** sono del suo pre-controllo e sono **già applicate** al suo testo —, le voci **P-11** e **P-22**…**P-25**, le decisioni **D4**, **D5**, **D12**…**D18** e **D29**, le voci aperte che il piano sa, **il compito 8 intero**, e dal disegno le risposte **13**, **19** e **20**, la riga «focus» della (a), la barra, la fascia, la Panoramica, le viste salvate col nome e la finestra della **(d)**, i controlli **17** e **19** e le decisioni **18**–**20** del coordinatore — **copiati parola per parola** da `_extract_brief_8.py`, a `HEAD` = `bc6e94d`. Leggilo **tutto**, a blocchi |

Poi, **per le sole parti che il compito nomina o che modifichi**, e **prima** di scriverle: i file della lista *Files* del
compito che esistono; `gui/src/components/BaseDialog.vue`, `BaseButton.vue`, `BaseTextField.vue` e `icons.ts`, che la
Panoramica consuma; `gui/src/frame/Band.vue`, `nearest.ts` e `schematic.ts`; `gui/src/frame/VueContent.ts`, la classe
`panel` che la regola di **E100** guarda; `gui/src/transport/fakeBridge.ts` e `gui/src/schema/messages.ts`, che le prove
usano.

⛔ **Non leggi:** il piano intero (oltre 11 000 righe: ti serve il brief); il disegno intero; `docs/HANDOFF.md`; la cartella
`docs/adr/`; `docs/archivio/`; l'audit; le copie del pre-controllo, se questa macchina le ha — lo scratchpad della sessione
del pre-controllo, sulla macchina `Jays` —, che sono il confronto della revisione e non il tuo modello. La lettura
d'apertura di `CLAUDE.md` — il compendio per intero — l'ha fatta il coordinatore, e le sue **regole** valgono per te: codice
in inglese e documenti in italiano, nessuna cifra nuova senza il suo comando, i fine-riga conservati per file, mai `sed -i`.

## 2. Il pre-controllo, e le sue undici voci

Il **pre-controllo** del compito 8 — sulla macchina `Jays`, il 2026-09-27 — ha rifatto il compito per intero dal testo del
piano su un `git worktree` di `f3206c7`, poi dal testo corretto su una copia pulita. Ha trovato **undici voci**, già scritte
nell'errata e **già applicate** al testo che hai nel brief:

| Voce | Che cosa cambia per te |
|---|---|
| **E91** | il Passo 1 cerca `F3` con `grep -rnw`, e misura la baseline del pezzo JavaScript |
| **E92** | `frame.browser.test.ts` **c'è dal 6bis**: il Passo 2 lo **estende** con quattro sostituzioni, non lo crea; l'oracolo `computed(property, token)` va in `testing/probes.ts`, e le prove del dock, della cornice e della pagina kit lo prendono da lì |
| **E93** | i conti del Passo 3, del 7 e dell'8 sono **misurati**: tu misuri, e un numero diverso lo riporti |
| **E94** | nel `Frame.vue` riscritto il commento di **E86** — la geometria vive in `nearest.ts` |
| **E95** | nel Passo 9 tre righe nominano prove in più, e la riga delle parole delle viste toglie anche la virgola di `"work"` |
| **E96** | la prova delle fasce prende la fascia `stop` e la pagina che non sborda; quella del cassetto l'apertura dalla tastiera; una prova nuova la barra e il segnaposto |
| **E97** | la prova della vista col nome ne porta **due**, sceglie la seconda, e chiude con una delle tre |
| **E98** | ⚠️ il **codice dettato**: *«Salva questa vista»* spenta finché `layout.arrivals` è zero; e tre prove consegnano prima una risposta del core |
| **E99** | ⚠️ il **codice dettato**: `drawn` disegna vuota una disposizione che non si legge; `showNamed` nel negozio, e la Panoramica apre le viste col nome solo da lì |
| **E100** | ⚠️ il **codice dettato**, deciso dal proprietario: l'anello del fuoco di un pannello che scorre, **dentro**, con una regola in `tokens/dock.css` e la sua prova |
| **E101** | ⛔ **salti il Passo 10**, lo sguardo: è del proprietario col coordinatore, dopo la revisione; e ⛔ **non tocchi il piano** — le celle le scrive la chiusura |

## 3. I numeri — misurati dal pre-controllo, sulla macchina `Jays`

**Il cancello d'apertura del pre-controllo**, a `f3206c7`: `GATE GREEN`; sotto `gui/` il progetto `jsdom` con **21** file
passati e uno saltato, **157** prove passate e una saltata, il progetto `browser` con **6** file e **56** prove; il pezzo
JavaScript `693.02 kB`, compresso `211.20 kB`; `found 0 vulnerabilities`. ⛔ Li **rimisuri tu** prima di toccare.

**Misurati dal pre-controllo**, sulla copia col testo corretto — ⚠️ sono **riferimenti**, non Attesi nuovi: tu **misuri**, e
un numero diverso si **riporta**, non si insegue:

| Passo | Uscita del pre-controllo |
|---|---|
| 1 | `grep -rnw 'F3'` vuoto; `drawer.ts` assente; `bar.views` nel solo `ViewBar.vue`; `reka-ui` `2.10.4`; `693.02 kB`, compresso `211.20 kB` |
| 3 | sotto jsdom `Failed to resolve import "../stores/drawer"` e `"./stores/drawer"`, e `layout.showNamed is not a function`, `Tests  1 failed \| 24 passed (25)`; nel browser `Tests  10 failed \| 39 passed (49)`, coi messaggi dell'Atteso |
| 7 | sotto jsdom `Tests  61 passed (61)`; nel browser `Tests  49 passed (49)` |
| 8 | `Test Files  27 passed \| 1 skipped (28)`, `Tests  234 passed \| 1 skipped (235)`, cinque corse su cinque; *build* e linter puliti; il pezzo **`698.14 kB`**, compresso `212.98 kB` |
| 9 | le **trentadue** righe coi messaggi della tabella, ciascuna rossa sulle sue prove, e alla fine `git status` coi soli venti file del compito |

⛔ Se una prova cade anche **una** volta nelle cinque corse, lo **riporti** con la sua uscita: una caduta è una voce d'errata,
non una corsa da ripetere finché passa (P-19).

## 4. I fine-riga

⛔ **Nessuna etichetta di forma: si misura** (E72). **Prima** di toccare, sui file della lista *Files* che esistono,
`git ls-files --eol <file>` e `tr -cd '\r' < <file> | wc -c` contro `wc -l`: sono le colonne di **questa** macchina, e
dipendono dal suo `core.autocrlf` (§0). **Dopo**, la **forma**, non il numero di prima — un file che cresce ha più righe:
su un file CRLF i CR sono **uguali alle righe**, su un file LF sono **zero**, e la colonna `w/…` è quella di prima; i file
**nuovi** nascono **LF**, zero CR; i quattro riscritti per intero tengono il terminatore che hanno oggi. Scrivi con Python
`newline=""` (temporaneo più `os.replace`) o con `replace_unique.py`, che conserva il fine-riga del file che trova e che copi
dagli *Strumenti* del brief **nello scratchpad** `C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\d19536a1-7be4-437b-8e11-39fed58af059\scratchpad\task8` — in Git Bash `/c/Users/Jays/AppData/Local/Temp/claude/E--ALL-DEV-MY-REPOS-daemon/d19536a1-7be4-437b-8e11-39fed58af059/scratchpad/task8` —, mai nel repository; lì vanno anche i log. ⛔ **Mai
`sed -i`.**

## 5. Ciò che da qui non si misura, e come lo fai

- ⚠️ **Ogni direzione rossa si torna indietro con la COPIA SALVATA, mai con `git checkout`** (vincolo 11): prima della prima
  violazione `git status --porcelain > /c/Users/Jays/AppData/Local/Temp/claude/E--ALL-DEV-MY-REPOS-daemon/d19536a1-7be4-437b-8e11-39fed58af059/scratchpad/task8/prima.txt` e una copia di ogni file che le violazioni toccano; dopo
  **ciascuna** la copia torna e `cmp` lo conferma; alla fine `git status --porcelain | diff /c/Users/Jays/AppData/Local/Temp/claude/E--ALL-DEV-MY-REPOS-daemon/d19536a1-7be4-437b-8e11-39fed58af059/scratchpad/task8/prima.txt -` rende
  soltanto i file del compito. Lancia ogni violazione sulla **suite intera**, `npx vitest run` sotto `gui/`, i due progetti:
  diverse righe fanno cadere prove di altri file, e la tabella le nomina (**E95**). Per ogni violazione riporti il **messaggio
  rosso vero** — la prima riga che nomina la ragione — e **quali** prove cadono, non «rosso».
- ⚠️ **Il cancello dura da uno a dieci minuti**, secondo le cache. Lancialo **da solo**, in background, con l'uscita in un
  log datato nello scratchpad, e aspetta la notifica: una chiamata in primo piano scade. **Due cancelli insieme si
  pestano** su `gui/node_modules` (gotcha #133): mai un `npm`, un `vitest` o un `npm run dev` mentre gira.
- ⚠️ **Chrome si aggiorna da sé fra una corsa e l'altra** (lezione 5 della consegna dell'esecuzione del compito 3): un rosso
  del progetto `browser` che non nomina una prova — `Target page, context or browser has been closed`, `Failed to connect to
  the browser session` — si rilancia **dopo** aver riletto la versione dal nome della cartella, prima di cercarne la causa
  nel codice.
- ⚠️ **La CI** la legge il coordinatore dopo il push, che è suo.

## 6. Il contratto

1. **Niente subagenti.** Lavori tu, un passo per volta, **nell'ordine del compito**: le prove prima del codice — il Passo 2
   intero, rosso al Passo 3, poi i Passi 4–7 —, i rossi guardati: è il ciclo di `superpowers:test-driven-development`, e il
   piano lo ha già scritto. ⛔ **Il Passo 10 lo salti** (**E101**).
2. **Un commit solo**, coi soli file del compito: i nuovi `gui/src/stores/drawer.ts` e `gui/src/frame/Overview.vue`; i
   riscritti `gui/src/frame/ViewBar.vue`, `Frame.vue`, `Drawer.vue` e `gui/src/panels/Strip.vue`; i toccati
   `gui/src/stores/invoke.ts`, `gui/src/stores/layout.ts`, `gui/src/components/Confirm.vue`, `gui/src/locales/it.json`,
   `gui/src/tokens/dock.css`, `gui/src/frame/frame.test.ts`, `gui/src/frame/frame.browser.test.ts`,
   `gui/src/stores/stores.test.ts`, `gui/src/a11y.test.ts`, `gui/src/locales/copy.test.ts`,
   `gui/src/frame/dock.browser.test.ts`, `gui/src/kit/kit.browser.test.ts`, `gui/src/testing/axe.ts` e
   `gui/src/testing/probes.ts`. ⛔ **Il piano no**, né alcun altro documento (**E101**). Il messaggio sta in un file nello
   scratchpad e si passa con `git commit -F <file>`, comincia con `design-system(compito 8): ` (vincolo 15), e porta il
   pezzo JavaScript **prima e dopo** (**E80**, N-2). ⛔ **Senza co-autore**: `CLAUDE.md` prevale su qualunque promemoria
   d'attribuzione. ⛔ **Niente `git push`**, niente `--amend`, niente rebase: il push è del coordinatore, **dopo la
   revisione**.
3. **Prima del commit**, uno alla volta: `bash scripts/gate.sh` → `GATE GREEN` e `bash scripts/check-docs.sh` → `OK`;
   e `git status --porcelain` che nomina **solo** i file del punto 2 — nessun file nato dalle prove.
4. ⛔ **Se il compito dice il falso, ti fermi e lo riporti**: una divergenza è una **voce d'errata candidata** — la
   prossima libera è **E102** — nel rapporto, con l'evidenza: il comando e la sua uscita. **Mai** un aggiustamento
   silenzioso. Se la divergenza ti impedisce di chiudere un passo, **non committi** e rendi `BLOCKED`.
5. **Non tocchi** `crates/`, `.github/`, `Cargo.*`, `gui/schema/`, `gui/package.json`, `gui/package-lock.json`,
   `docs/`, `scripts/`.

## 7. Il rapporto

Scrivi `.superpowers/sdd/2026-09-23-design-system/task-8-report.md`, con:

1. lo **stato** — `DONE`, `DONE_WITH_CONCERNS`, `BLOCKED` o `NEEDS_CONTEXT` — e l'**hash** del commit;
2. `git show --stat HEAD`;
3. per **ogni passo** dall'1 al 8, i comandi lanciati e le uscite **vere**: i rossi coi nomi delle prove; al Passo 8 le
   cinque corse, il *build* col pezzo JavaScript e il linter;
4. la tabella del Passo 9, col messaggio rosso vero di ogni riga e le prove che cadono, i `cmp` e il `diff` finale;
5. i **fine-riga**, per ogni file toccato: prima e dopo, le due colonne;
6. il cancello: le righe `Test Files` e `Tests` dei due progetti, e `found 0 vulnerabilities`;
7. le **divergenze**, come voci candidate `E102`… con l'evidenza, e ciò che non hai potuto misurare;
8. il tempo, e i percorsi dei log del cancello.

Nel messaggio finale al coordinatore: lo stato, l'hash e il percorso del rapporto, in poche righe.
