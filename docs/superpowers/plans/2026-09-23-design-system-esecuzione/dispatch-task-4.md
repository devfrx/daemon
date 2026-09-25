Sei l'**implementatore del compito 4** — *la pagina kit: fuori dal pacchetto, e le sonde diventano prove nel browser* — del
piano `docs/superpowers/plans/2026-09-23-design-system.md`, nel repository `E:\ALL\DEV\MY_REPOS\daemon` (Windows; il tool Bash è Git Bash). Sei un subagente fresco: tutto ciò che
ti serve è qui e nel brief che questo prompt nomina. Il compito è **codice della GUI** — le tre sonde delle tavole come
funzioni in `gui/src/testing/probes.ts`; la pagina kit, `gui/kit.html` con `gui/src/kit/`, e la sua prova nel browser; una
riga del progetto `browser` in `gui/vite.config.ts`; il blocco delle parole esemplari in `gui/eslint.config.js`; la prova
che la pagina resti fuori dal pacchetto in `scripts/gate-gui.sh` — e **due celle** del piano.

**All'avvio verifichi, e se non torna ti fermi e lo riporti:** `git rev-parse --short HEAD` → `1b531e9`;
`git status --porcelain` → vuoto; `node --version` → una versione che `gui/package.json` accetta (`engines`);
`git config --show-origin --get-all core.autocrlf` → il valore di questa macchina, nel §0; Google Chrome stabile
installato, e la sua versione **letta dal nome della cartella** —
`ls "/c/Program Files/Google/Chrome/Application/" "$LOCALAPPDATA/Google/Chrome/Application/" 2>/dev/null` rende almeno una
cartella di versione, come `154.0.8037.58` — su questa macchina la seconda cartella non esiste, e il comando
esce **2** pur stampando la versione: conta la cartella, non l'uscita. ⛔ **Mai `chrome.exe --version`**: su Windows apre il browser col profilo
dell'utente e non stampa nulla (lezione 4 della consegna dell'esecuzione del compito 3, in archivio). La data da scrivere al
posto di ogni `<data>` è **2026-09-25**, sempre la stessa anche se l'esecuzione passa la mezzanotte.

## 0. La macchina

Il repository vive in **due cloni**, e ciò che cambia fra i due si **misura**, non si copia da un'etichetta: voce **E72**
del piano della parte 2, ed **E1** di questo piano.

| | macchina `Jays` | macchina `zagor` |
|---|---|---|
| il repository | `E:\ALL\DEV\MY_REPOS\daemon` | `C:\Users\zagor\Desktop\harness` |
| `core.autocrlf` | `false` in `.git/config`, quindi l'albero è `w/lf` — `--get-all` rende tre righe, `true` dal file di sistema, `false` da `C:/Users/Jays/.gitconfig` e `false` da `.git/config`, e vale l'ultima | `true` dal file di sistema: l'albero è `w/crlf` per i file che git ha scritto, e `w/lf` per quelli nati o riscritti LF su quella macchina |
| Google Chrome | `154.0.8037.58`, letto dal nome della cartella il 2026-09-25, e riletto dal coordinatore prima del dispaccio | `154.0.8037.58` dalla consegna dell'esecuzione del compito 3: si aggiorna **da sé**, e il 2026-09-25 l'ha fatto durante una revisione — si rilegge |
| Node | v24.19.0 | v24.19.0 |

---

## 1. Che cosa leggi, e che cosa no

La cartella di lavoro è `.superpowers/sdd/2026-09-23-design-system/`, git-ignorata; il brief ci nasce da
`_extract_brief_4.py`, nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`:

| File | Che cos'è |
|---|---|
| `task-4-brief.md` | **il compito**: la testa del piano (obiettivo, architettura, pila, disegno, **strumenti** con `replace_unique.py`), i *Vincoli globali*, *A che punto è* e *Come si esegue un compito*, l'**errata** — **E24**, **E25**, **E26** ed **E27** sono del compito 4, ed **E19** ed **E20** sono le cure del compito 3 che le sue prove nuove tengono —, la voce **P-4**, le decisioni **D7** e **D8**, le voci aperte che il piano sa, **il compito 4 intero**, e dal disegno le risposte **4**, **12** e **20**, la pagina kit della sezione **(b)**, la sezione **(f)**, i controlli **9**, **11**, **14** e **20**, le trappole **1**, **3**, **4** e **12**, e **le tre sonde delle tavole**, intere — **copiati parola per parola** da `_extract_brief_4.py`, a `HEAD` = `1b531e9`. Leggilo **tutto**, a blocchi |

Poi, **per le sole parti che il compito nomina o che modifichi**, e **prima** di scriverle: i file della lista *Files* del
compito che esistono — `gui/eslint.config.js`, `gui/vite.config.ts`, `scripts/gate-gui.sh` —, i pezzi del kit che le
violazioni del Passo 7 toccano, e le righe `| **3** |` e `| **4** |` della tabella della posizione del piano.

⛔ **Non leggi:** il piano intero (oltre 8 700 righe: ti serve il brief); il disegno intero; `docs/HANDOFF.md`; la cartella
`docs/adr/`; `docs/archivio/`; l'audit; le copie del pre-controllo, se questa macchina le ha — `%TEMP%\pc4`, `%TEMP%\pc4b` e
`%TEMP%\pc4c` sulla macchina `Jays` —, che sono il confronto della revisione e non il tuo modello. La lettura d'apertura di
`CLAUDE.md` — il compendio per intero — l'ha fatta il coordinatore, e le sue **regole** valgono per te: codice in inglese e
documenti in italiano, nessuna cifra nuova senza il suo comando, i fine-riga conservati per file, mai `sed -i`.

## 2. Il pre-controllo, e le sue quattro voci

Il **pre-controllo** del compito 4 — sulla macchina `Jays`, il 2026-09-25 — ha rifatto il compito per intero dal testo del
piano su una copia pulita, e poi dal testo corretto su una seconda. Ha trovato **quattro difetti**, già scritti nell'errata e
**già applicati** al testo del compito che hai nel brief:

| Voce | Che cosa cambia per te |
|---|---|
| **E24** | il Passo 2 **comincia** da una riga in `gui/vite.config.ts` — `optimizeDeps: { force: true }` sul progetto `browser` —, **prima** della prova: senza, la prima corsa del Passo 5 dopo il rosso del Passo 2 cade **tutta** con `TypeError: 'set' on proxy: trap returned falsish for property 'style'`, e non per la pagina |
| **E25** | la prova della pagina ha **due prove in più per tema** — i pulsanti spenti di ogni variante (**E19**) e il bordo dell'errore sotto il puntatore (**E20**) —, l'aiutante `colourOf`, `userEvent`, e le pagine montate si smontano in `afterEach`: **nessuna** prova si smonta da sé. Il Passo 7 ha tre righe in più |
| **E26** | il Passo 7 ha una **seconda tabella**: il blocco del Passo 4, tolto, e il suo confine, con `npm run lint` |
| **E27** | il Passo 7 ha una riga per il ramo **fuori dall'angolo** di `concentricRadii`: le righe della lista più tonde **e** spostate dall'angolo |

## 3. I numeri — misurati dal pre-controllo, sulla macchina `Jays`

**Il cancello d'apertura del pre-controllo**, a `71a9f84`: `GATE GREEN`; sotto `gui/` il progetto `jsdom` con **18** file
passati e uno saltato, **125** prove passate e una saltata, il progetto `browser` con **1** file e **5** prove; il pezzo
JavaScript `663.93 kB`; `found 0 vulnerabilities`; la CI verde sui due sistemi. **Rimisurati dal coordinatore** a `1b531e9`, sulla stessa macchina, prima di questo dispaccio: gli stessi numeri, e `GATE GREEN`; la CI di `1b531e9` verde sui due sistemi. ⛔ Li **rimisuri tu** prima di toccare.

**Misurati dal pre-controllo**, sulla copia col compito rifatto dal testo corretto — ⚠️ sono **riferimenti**, non Attesi
nuovi: tu **misuri**, e un numero diverso si **riporta**, non si insegue:

| Passo | Uscita del pre-controllo |
|---|---|
| 2 | rosso: `Failed to resolve import "./Kit.vue" from "src/kit/kit.browser.test.ts"` |
| 5 | il progetto `browser` **verde alla prima corsa** dopo il rosso del Passo 2: **2** file, **23** prove; `npm run lint` verde; `npm run build` verde, il pezzo JavaScript `663.93 kB` col nome che ha il `main` sulla stessa macchina — su `Jays` `index-CiZv4zPX.js` |
| 6 | rosso *«the kit page is in the package»* con la pagina fra gli ingressi; verde senza: il passo web con `--project jsdom` **18** file passati e uno saltato, **125** prove passate e una saltata, `--project browser` **2** file e **23** prove, `found 0 vulnerabilities` |
| 7 | ogni riga rossa **per la ragione scritta**, e sulle **sole** prove che nomina: i messaggi della tabella del Passo 7, fra cui `base-button in base-dialog, bottom-right: radius 16.0, outer 20.0, distance 13.0/13.0` e `base-list-row in kit-card, bottom-left: radius 16.0, outer 20.0, distance 17.0/13.0`; nella seconda tabella, trentatré `raw text … is used`, poi `raw text 'ciao' is used`, poi *«'lucide' import is restricted from being used»* |

## 4. I fine-riga

⛔ **Nessuna etichetta di forma: si misura** (E72). **Prima** di toccare, sui file della lista *Files* che esistono,
`git ls-files --eol <file>` e `tr -cd '\r' < <file> | wc -c` contro `wc -l`: sono le colonne di **questa** macchina, e
dipendono dal suo `core.autocrlf` (§0). **Dopo**, la **forma**, non il numero di prima — un file che cresce ha più righe:
su un file CRLF i CR sono **uguali alle righe**, su un file LF sono **zero**, e la colonna `w/…` è quella di prima; i file
**nuovi** nascono **LF**, zero CR. Scrivi con Python `newline=""` (temporaneo più `os.replace`) o con `replace_unique.py`,
che conserva il fine-riga del file che trova e che copi dagli *Strumenti* del brief **nello scratchpad** `C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\85fa78d3-ac8f-4098-9b47-eaa6f1c6ea46\scratchpad\task4` — in Git Bash `/c/Users/Jays/AppData/Local/Temp/claude/E--ALL-DEV-MY-REPOS-daemon/85fa78d3-ac8f-4098-9b47-eaa6f1c6ea46/scratchpad/task4` —, mai nel repository. ⛔ **Mai `sed -i`.**

## 5. Ciò che da qui non si misura, e come lo fai

- ⚠️ **Il Passo 7 si torna indietro con la COPIA SALVATA, mai con `git checkout`** (vincolo 11): prima della prima
  violazione `git status --porcelain > /c/Users/Jays/AppData/Local/Temp/claude/E--ALL-DEV-MY-REPOS-daemon/85fa78d3-ac8f-4098-9b47-eaa6f1c6ea46/scratchpad/task4/prima.txt` e una copia di ogni file che le violazioni toccano —
  `gui/src/components/BaseButton.vue`, `gui/src/components/BaseList.vue`, `gui/src/components/BaseLabel.vue`,
  `gui/src/components/BaseTextField.vue`, `gui/src/components/BaseStatus.vue`, `gui/src/tokens/themes.css`,
  `gui/src/kit/Kit.vue`, `gui/src/kit/kit.browser.test.ts`, `gui/eslint.config.js` —, e per la direzione rossa del Passo 6
  `gui/vite.config.ts`; dopo **ciascuna** la copia torna e `cmp` lo conferma; alla fine
  `git status --porcelain | diff /c/Users/Jays/AppData/Local/Temp/claude/E--ALL-DEV-MY-REPOS-daemon/85fa78d3-ac8f-4098-9b47-eaa6f1c6ea46/scratchpad/task4/prima.txt -` non rende nulla. Per ogni violazione riporti il **messaggio rosso
  vero** — la prima riga che nomina la ragione —, e **quali** prove cadono, non «rosso».
- ⚠️ **Il cancello dura da uno a dieci minuti**, secondo le cache, e il Passo 6 lancia `bash scripts/gate-gui.sh` due volte.
  Lancia ciascuno **da solo**, in background, con l'uscita in un log datato nello scratchpad, e aspetta la notifica: una
  chiamata in primo piano scade. **Due cancelli insieme si pestano** su `gui/node_modules` (gotcha #133): mai un `npm`, un
  `vitest` o un `npm run dev` mentre gira.
- ⚠️ **Chrome si aggiorna da sé fra una corsa e l'altra** (lezione 5 della consegna dell'esecuzione del compito 3): un rosso
  del progetto `browser` che non nomina una prova — `Target page, context or browser has been closed`, `Failed to connect to
  the browser session` — si rilancia **dopo** aver riletto la versione dal nome della cartella, prima di cercarne la causa
  nel codice.
- ⚠️ **Il Passo 8, *guardarla*,** lo fa il revisore nel browser, nei due temi (punto 5 di *«Come si esegue un compito»*):
  tu lanci `(cd gui && npm run dev)` **solo** se hai un modo di aprire `/kit.html` e di guardarla, e in quel caso lo
  **fermi** prima del cancello — tiene aperti file sotto `gui/node_modules`; se non puoi, lo dici nel rapporto, e non è un
  difetto.
- ⚠️ **La CI** la legge il coordinatore dopo il push, che è suo.

## 6. Il contratto

1. **Niente subagenti.** Lavori tu, un passo per volta, **nell'ordine del compito**: la prova prima della pagina (Passo 2, il
   rosso guardato), poi la pagina — è il ciclo di `superpowers:test-driven-development`, e il piano lo ha già scritto.
2. **Un commit solo**, coi soli file del compito: `gui/kit.html`, `gui/src/kit/main.ts`, `gui/src/kit/Kit.vue`,
   `gui/src/kit/kit.browser.test.ts`, `gui/src/testing/probes.ts`, `gui/eslint.config.js`, `gui/vite.config.ts` e
   `scripts/gate-gui.sh`; e il piano, dove la riga **4** della tabella della posizione passa a **Stato** `✅ 2026-09-25` e nella
   riga **3** la colonna **Commit** diventa `` `c7b7bcd`, con la cura `9d2ffb0` `` (R1-16); la colonna **Commit** della riga
   **4** resta `—`, la scrive il compito 5. Il messaggio sta in un file nello scratchpad e si passa con
   `git commit -F <file>`, e comincia con `design-system(compito 4): ` (vincolo 15). ⛔ **Senza co-autore**: `CLAUDE.md`
   prevale su qualunque promemoria d'attribuzione. ⛔ **Niente `git push`**, niente `--amend`, niente rebase: il push è del
   coordinatore, **dopo la revisione** — il Passo 8 dice `git push`, e questo contratto lo sposta, come nei compiti 1, 2 e 3.
3. **Prima del commit**, uno alla volta: `bash scripts/gate.sh` → `GATE GREEN` e `bash scripts/check-docs.sh` → `OK`;
   e `git status --porcelain` che nomina **solo** i file del punto 2 — nessun file nato dalle prove.
4. ⛔ **Se il compito dice il falso, ti fermi e lo riporti**: una divergenza è una **voce d'errata candidata** — la
   prossima libera è **E28** — nel rapporto, con l'evidenza: il comando e la sua uscita. **Mai** un aggiustamento
   silenzioso. Se la divergenza ti impedisce di chiudere un passo, **non committi** e rendi `BLOCKED`.
5. **Non tocchi** `crates/`, `.github/`, `Cargo.*`, `gui/schema/`, `gui/package.json`, `gui/package-lock.json`,
   `docs/adr/`, altri script oltre a `scripts/gate-gui.sh`, né altri documenti oltre alle due celle del piano.

## 7. Il rapporto

Scrivi `.superpowers/sdd/2026-09-23-design-system/task-4-report.md`, con:

1. lo **stato** — `DONE`, `DONE_WITH_CONCERNS`, `BLOCKED` o `NEEDS_CONTEXT` — e l'**hash** del commit;
2. `git show --stat HEAD`;
3. per **ogni passo**, i comandi lanciati e le uscite **vere**: al Passo 2 il messaggio rosso; al Passo 5 i conti, il lint e
   il pezzo JavaScript; al Passo 6 le due direzioni;
4. le due tabelle del Passo 7, col messaggio rosso vero di ogni riga e le prove che cadono, i `cmp` e il `diff` finale;
5. i **fine-riga**, per ogni file toccato: prima e dopo, le due colonne;
6. il passo web: le righe `Test Files` e `Tests` dei due progetti, e `found 0 vulnerabilities`;
7. il Passo 8: che cosa hai guardato, o perché non hai potuto;
8. le **divergenze**, come voci candidate `E28`… con l'evidenza, e ciò che non hai potuto misurare;
9. il tempo, e i percorsi dei log del cancello.

Nel messaggio finale al coordinatore: lo stato, l'hash e il percorso del rapporto, in poche righe.
