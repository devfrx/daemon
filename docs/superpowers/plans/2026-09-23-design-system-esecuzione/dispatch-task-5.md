Sei l'**implementatore del compito 5** — *il kit al lavoro: i pezzi di base nei pannelli e nella cornice* — del piano
`docs/superpowers/plans/2026-09-23-design-system.md`, nel repository `E:\ALL\DEV\MY_REPOS\daemon` (Windows; il tool Bash è Git Bash). Sei un subagente fresco: tutto ciò che ti
serve è qui e nel brief che questo prompt nomina. Il compito è **codice della GUI** — sette componenti riscritti per intero
sui pezzi di base, due toccati, le prove che cambiano e una nuova nel browser, le parole del tema, le due regole del linter
su `panels/` e `frame/`, un commento di `gui/src/tokens/dock.css` — e **una cella** del piano.

**All'avvio verifichi, e se non torna ti fermi e lo riporti:** `git rev-parse --short HEAD` → `39827e8`;
`git status --porcelain` → vuoto; `node --version` → una versione che `gui/package.json` accetta (`engines`);
`git config --show-origin --get-all core.autocrlf` → il valore di questa macchina, nel §0; Google Chrome stabile
installato, e la sua versione **letta dal nome della cartella** —
`ls "/c/Program Files/Google/Chrome/Application/" "$LOCALAPPDATA/Google/Chrome/Application/" 2>/dev/null` rende almeno una
cartella di versione, come `154.0.8037.58` — su questa macchina la seconda cartella non esiste, e il comando esce **2**
pur stampando la versione: conta la cartella, non l'uscita. ⛔ **Mai `chrome.exe --version`**: su Windows apre il browser col profilo
dell'utente e non stampa nulla (lezione 4 della consegna dell'esecuzione del compito 3, in archivio).

## 0. La macchina

Il repository vive in **due cloni**, e ciò che cambia fra i due si **misura**, non si copia da un'etichetta: voce **E72**
del piano della parte 2, ed **E1** di questo piano.

| | macchina `Jays` | macchina `zagor` |
|---|---|---|
| il repository | `E:\ALL\DEV\MY_REPOS\daemon` | `C:\Users\zagor\Desktop\harness` |
| `core.autocrlf` | `false` in `.git/config`, quindi l'albero è `w/lf` — `--get-all` rende tre righe, `true` dal file di sistema, `false` da `C:/Users/Jays/.gitconfig` e `false` da `.git/config`, e vale l'ultima | `true` dal file di sistema: l'albero è `w/crlf` per i file che git ha scritto, e `w/lf` per quelli nati o riscritti LF su quella macchina |
| Google Chrome | `154.0.8037.58`, letto dal nome della cartella il 2026-09-26, e riletto dal coordinatore prima del dispaccio | `154.0.8037.58` dalla consegna dell'esecuzione del compito 3: si aggiorna **da sé** — si rilegge |
| Node | v24.19.0 | v24.19.0 |

---

## 1. Che cosa leggi, e che cosa no

La cartella di lavoro è `.superpowers/sdd/2026-09-23-design-system/`, git-ignorata; il brief ci nasce da
`_extract_brief_5.py`, nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`:

| File | Che cos'è |
|---|---|
| `task-5-brief.md` | **il compito**: la testa del piano (obiettivo, architettura, pila, disegno, **strumenti** con `replace_unique.py`), i *Vincoli globali*, *A che punto è* e *Come si esegue un compito*, l'**errata** — **E34**, **E35**, **E36** ed **E37** sono del compito 5, ed **E2** ed **E4** sono le cure del compito 1 su cui si appoggia —, le voci **P-3**, **P-8**, **P-19** e **P-20**, le voci aperte che il piano sa, **il compito 5 intero**, e dal disegno le risposte **5**, **16**, **17** e **21**, *«I due temi»* della sezione **(a)**, i pezzi di base, `BaseStatus` e le regole del linter della sezione **(b)**, la sezione **(e)**, i controlli **11**, **12** e **13**, le trappole **5** e **10** e la decisione **21** del coordinatore — **copiati parola per parola** da `_extract_brief_5.py`, a `HEAD` = `39827e8`. Leggilo **tutto**, a blocchi |

Poi, **per le sole parti che il compito nomina o che modifichi**, e **prima** di scriverle: i file della lista *Files* del
compito che esistono; gli otto pezzi di base in `gui/src/components/`, di cui i componenti riscritti usano props, slot ed
eventi; `gui/src/stores/layout.ts` e `gui/src/tokens/theme.ts`, per il tema; e le righe `| **4** |` e `| **5** |` della
tabella della posizione del piano.

⛔ **Non leggi:** il piano intero (oltre 8 900 righe: ti serve il brief); il disegno intero; `docs/HANDOFF.md`; la cartella
`docs/adr/`; `docs/archivio/`; l'audit; le copie del pre-controllo, se questa macchina le ha — `%TEMP%\pc5`, `%TEMP%\pc5b` e
`%TEMP%\pc5c` sulla macchina `Jays` —, che sono il confronto della revisione e non il tuo modello. La lettura d'apertura di
`CLAUDE.md` — il compendio per intero — l'ha fatta il coordinatore, e le sue **regole** valgono per te: codice in inglese e
documenti in italiano, nessuna cifra nuova senza il suo comando, i fine-riga conservati per file, mai `sed -i`.

## 2. Il pre-controllo, e le sue quattro voci

Il **pre-controllo** del compito 5 — sulla macchina `Jays`, il 2026-09-26 — ha rifatto il compito per intero dal testo del
piano su una copia pulita, e poi dal testo corretto su una seconda, e ha aperto la SPA nel browser nei due temi. Ha trovato
**quattro difetti**, già scritti nell'errata e **già applicati** al testo del compito che hai nel brief:

| Voce | Che cosa cambia per te |
|---|---|
| **E34** | nel Passo 7 le violazioni delle righe 1 e 2 sono **senza parole** — `<button type="button"></button>` e `<ul><li></li></ul>` —, e ciascuna riga vuole **un errore solo**, quello della regola: con una parola dentro scattava anche `no-raw-text`, e il rosso non provava la regola |
| **E35** | il **Passo 8 non è tuo**: lo fa il proprietario col coordinatore, dopo la revisione. Nel Passo 9 la riga **5** della posizione **resta `⬜`** — la porta a ✅ il coordinatore, col verbale — e tu scrivi solo la colonna **Commit** della riga 4 |
| **E36** | il Passo 5 ha una sostituzione in più, in `gui/src/tokens/dock.css`: il commento del ponte nominava i radio nativi di Impostazioni, che il compito toglie |
| **E37** | nel Passo 1 `settings.browser.test.ts` porta il commento `⛔ NON-VACUITY` sopra `expect(radios()).toHaveLength(2);`: è il segno che la Definizione di «fatto» conta |

## 3. I numeri — misurati dal pre-controllo, sulla macchina `Jays`

**Il cancello d'apertura del pre-controllo**, a `257fc5a`: `GATE GREEN`; sotto `gui/` il progetto `jsdom` con **18** file
passati e uno saltato, **125** prove passate e una saltata, il progetto `browser` con **3** file e **27** prove; il pezzo
JavaScript `663.93 kB`; `found 0 vulnerabilities`; la CI verde sui due sistemi. Il coordinatore l'ha rilanciato
all'apertura di questa sessione, a `39827e8`: gli stessi numeri, e la CI di `39827e8` verde sui due sistemi. ⛔ Li **rimisuri tu** prima di toccare.

**Misurati dal pre-controllo**, sulla copia col compito rifatto dal testo corretto — ⚠️ sono **riferimenti**, non Attesi
nuovi: tu **misuri**, e un numero diverso si **riporta**, non si insegue:

| Passo | Uscita del pre-controllo |
|---|---|
| 1 | sotto jsdom `Tests  8 failed \| 29 passed (37)`: le sei prove di Impostazioni e le tre di M-3 — una delle quali è di Impostazioni —, queste con `Unable to get [role="status"]`; verdi le due della finestra di conferma e `a11y.test.ts`. Nel browser rossa la prova della tastiera, `expected [] to have a length of 2 but got +0` |
| 2 | rosso `system: expected [] to include 'system'`; dopo le parole `Tests  3 passed (3)`; la direzione rossa, `sepia: expected [ 'system', 'light', 'dark' ] to include 'sepia'` |
| 6 | `npm test` verde: `Test Files  22 passed \| 1 skipped (23)`, `Tests  158 passed \| 1 skipped (159)`; `npm run build` verde, il pezzo JavaScript **`689.58 kB`**, su `Jays` `index-BS90oxvQ.js`; la direzione rossa di M-3, `BaseStatus` dentro il `v-if` **un pannello per volta**, fa cadere ciascuna volta la **sola** prova di M-3 di quel pannello, `Unable to get [role="status"]` |
| 7 | `npm run lint` verde; le righe 1, 2 e 3 con **un errore solo** ciascuna — *«a button is BaseButton…»*, *«a list is BaseList…»*, *«'reka-ui' import is restricted from being used. panels and frame use the base pieces…»* —; la riga 4 è il verde del linter |

La prova della tastiera nel browser è stata stabile: **10** corse su 10 da sola, **5** su 5 nella suite intera. ⛔ Se cade
anche **una** volta, lo **riporti** con la sua uscita: una caduta è una voce d'errata, non una corsa da ripetere finché passa
(P-19, D23).

## 4. I fine-riga

⛔ **Nessuna etichetta di forma: si misura** (E72). **Prima** di toccare, sui file della lista *Files* che esistono,
`git ls-files --eol <file>` e `tr -cd '\r' < <file> | wc -c` contro `wc -l`: sono le colonne di **questa** macchina, e
dipendono dal suo `core.autocrlf` (§0). **Dopo**, la **forma**, non il numero di prima — un file che cresce ha più righe:
su un file CRLF i CR sono **uguali alle righe**, su un file LF sono **zero**, e la colonna `w/…` è quella di prima; i file
**nuovi** nascono **LF**, zero CR; un file riscritto per intero tiene il terminatore che ha oggi. Scrivi con Python
`newline=""` (temporaneo più `os.replace`) o con `replace_unique.py`, che conserva il fine-riga del file che trova e che
copi dagli *Strumenti* del brief **nello scratchpad** `C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\b62a978b-28b5-4b69-963c-b58323e34338\scratchpad\task5` — in Git Bash `/c/Users/Jays/AppData/Local/Temp/claude/E--ALL-DEV-MY-REPOS-daemon/b62a978b-28b5-4b69-963c-b58323e34338/scratchpad/task5` —, mai nel repository; lì vanno anche
`retarget_confirm.py` del Passo 1 e i log. ⛔ **Mai `sed -i`.**

## 5. Ciò che da qui non si misura, e come lo fai

- ⚠️ **Ogni direzione rossa si torna indietro con la COPIA SALVATA, mai con `git checkout`** (vincolo 11): prima della prima
  violazione `git status --porcelain > /c/Users/Jays/AppData/Local/Temp/claude/E--ALL-DEV-MY-REPOS-daemon/b62a978b-28b5-4b69-963c-b58323e34338/scratchpad/task5/prima.txt` e una copia di ogni file che le violazioni toccano —
  `gui/src/tokens/theme.ts` per il Passo 2; `gui/src/frame/Band.vue`, `gui/src/panels/Settings.vue` e
  `gui/src/panels/Status.vue` per il Passo 6, che questo compito ha già riscritto; `gui/src/panels/Strip.vue`,
  `gui/src/frame/ViewBar.vue` e `gui/src/frame/moveActive.ts` per il Passo 7 —; dopo **ciascuna** la copia torna e `cmp` lo
  conferma; alla fine `git status --porcelain | diff /c/Users/Jays/AppData/Local/Temp/claude/E--ALL-DEV-MY-REPOS-daemon/b62a978b-28b5-4b69-963c-b58323e34338/scratchpad/task5/prima.txt -` non rende nulla. Per ogni violazione riporti
  il **messaggio rosso vero** — la prima riga che nomina la ragione —, **quali** prove cadono e, nel Passo 7, **quanti**
  errori dà il linter, non «rosso».
- ⚠️ **Il cancello dura da uno a dieci minuti**, secondo le cache. Lancialo **da solo**, in background, con l'uscita in un
  log datato nello scratchpad, e aspetta la notifica: una chiamata in primo piano scade. **Due cancelli insieme si
  pestano** su `gui/node_modules` (gotcha #133): mai un `npm`, un `vitest` o un `npm run dev` mentre gira.
- ⚠️ **Chrome si aggiorna da sé fra una corsa e l'altra** (lezione 5 della consegna dell'esecuzione del compito 3): un rosso
  del progetto `browser` che non nomina una prova — `Target page, context or browser has been closed`, `Failed to connect to
  the browser session` — si rilancia **dopo** aver riletto la versione dal nome della cartella, prima di cercarne la causa
  nel codice.
- ⛔ **Il Passo 8 non lo fai** (**E35**): è del proprietario, con l'Assistente vocale, dopo la revisione. *Guardare* la SPA
  nei due temi è del revisore (punto 5 di *«Come si esegue un compito»*): tu lanci `(cd gui && npm run dev)` **solo** se
  hai un modo di aprire la pagina e di guardarla, e in quel caso lo **fermi per PID** prima del cancello — chiudere la shell
  non basta, il suo `node` resta in ascolto (lezione 3 della consegna dell'esecuzione del compito 4); se non puoi, lo dici
  nel rapporto, e non è un difetto.
- ⚠️ **La CI** la legge il coordinatore dopo il push, che è suo.

## 6. Il contratto

1. **Niente subagenti.** Lavori tu, un passo per volta, **nell'ordine del compito**: le prove prima del codice (Passo 1, i
   rossi guardati; Passo 2, la prova prima delle parole), poi i componenti — è il ciclo di
   `superpowers:test-driven-development`, e il piano lo ha già scritto.
2. **Un commit solo**, coi soli file del compito: `gui/src/components/Confirm.vue`, `gui/src/frame/Drawer.vue`,
   `gui/src/frame/Band.vue`, `gui/src/frame/ViewBar.vue`, `gui/src/panels/Settings.vue`, `gui/src/panels/Permissions.vue`,
   `gui/src/panels/Steps.vue`, `gui/src/panels/Placeholder.vue`, `gui/src/panels/Status.vue`, `gui/src/locales/it.json`,
   `gui/src/locales/copy.test.ts`, `gui/src/panels/modules.test.ts`, `gui/src/a11y.test.ts`, `gui/src/frame/frame.test.ts`,
   `gui/src/panels/settings.browser.test.ts`, `gui/eslint.config.js` e `gui/src/tokens/dock.css`; e il piano, dove nella
   riga **4** della tabella della posizione la colonna **Commit** diventa `` `841dc54`, con la cura `8d098d0` `` (R1-16),
   e la riga **5** resta `⬜` (**E35**). Il messaggio sta in un file nello scratchpad e si passa con `git commit -F <file>`,
   comincia con `design-system(compito 5): ` (vincolo 15), e porta il pezzo JavaScript **prima e dopo**, com'è nel messaggio
   del compito 1 — `git show -s --format=%B 95068bb | grep -n 'kB'`. ⛔ **Senza co-autore**: `CLAUDE.md` prevale su
   qualunque promemoria d'attribuzione. ⛔ **Niente `git push`**, niente `--amend`, niente rebase: il push è del
   coordinatore, **dopo la revisione** — il Passo 9 dice `git push`, e questo contratto lo sposta, come nei compiti 1–4.
3. **Prima del commit**, uno alla volta: `bash scripts/gate.sh` → `GATE GREEN` e `bash scripts/check-docs.sh` → `OK`;
   e `git status --porcelain` che nomina **solo** i file del punto 2 — nessun file nato dalle prove.
4. ⛔ **Se il compito dice il falso, ti fermi e lo riporti**: una divergenza è una **voce d'errata candidata** — la
   prossima libera è **E38** — nel rapporto, con l'evidenza: il comando e la sua uscita. **Mai** un aggiustamento
   silenzioso. Se la divergenza ti impedisce di chiudere un passo, **non committi** e rendi `BLOCKED`.
5. **Non tocchi** `crates/`, `.github/`, `Cargo.*`, `gui/schema/`, `gui/package.json`, `gui/package-lock.json`,
   `docs/adr/`, `scripts/`, né altri documenti oltre alla cella del piano.

## 7. Il rapporto

Scrivi `.superpowers/sdd/2026-09-23-design-system/task-5-report.md`, con:

1. lo **stato** — `DONE`, `DONE_WITH_CONCERNS`, `BLOCKED` o `NEEDS_CONTEXT` — e l'**hash** del commit;
2. `git show --stat HEAD`;
3. per **ogni passo**, i comandi lanciati e le uscite **vere**: al Passo 1 i rossi, coi nomi delle prove; al Passo 2 il rosso,
   il verde e la direzione rossa; al Passo 6 i conti, il *build* col pezzo JavaScript, e la direzione rossa di M-3 pannello
   per pannello;
4. la tabella del Passo 7, col messaggio rosso vero di ogni riga, il **numero** degli errori, i `cmp` e il `diff` finale;
5. i **fine-riga**, per ogni file toccato: prima e dopo, le due colonne;
6. il cancello: le righe `Test Files` e `Tests` dei due progetti, e `found 0 vulnerabilities`;
7. la SPA: che cosa hai guardato, o perché non hai potuto — il Passo 8 non è tuo;
8. le **divergenze**, come voci candidate `E38`… con l'evidenza, e ciò che non hai potuto misurare;
9. il tempo, e i percorsi dei log del cancello.

Nel messaggio finale al coordinatore: lo stato, l'hash e il percorso del rapporto, in poche righe.
