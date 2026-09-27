> ⚠️ **Per il coordinatore, prima di dispacciare** — questo file è il **modello**, e viaggia con git. Il prompt che parte
> si scrive nella cartella di lavoro `.superpowers/sdd/2026-09-23-design-system/`, ignorata, **senza** questo riquadro e
> coi campi fra `<…>` riempiti: `<repo>`, `<HEAD>` — l'ultimo commit di `main` —, `<scratchpad>`, e i valori della macchina
> del §0 misurati, non copiati. Il brief si genera **prima**, dalla radice del repository, con
> `python docs/superpowers/plans/2026-09-23-design-system-esecuzione/_extract_brief_6bis.py`, e deve dire *«piano e disegno
> coincidono con `HEAD`»*. ⛔ **Il codice del compito è provato su `dc77fc8`**: `git log --oneline dc77fc8..HEAD -- gui/`
> deve rendere nulla, o il compito si rilegge contro il codice di allora (`CLAUDE.md`, domanda 5). Alla chiusura del compito
> il prompt spedito, il rapporto, il prompt del revisore e la revisione si copiano nella cartella tracciata e si committano:
> il punto 8 di *«Come si esegue un compito»*. ⛔ **Il Passo 16 non è dell'implementatore** (**E69**): lo fanno il
> proprietario e il coordinatore dopo la revisione e le sue cure, e la chiusura scrive le righe 6 e 6bis della posizione.

Sei l'**implementatore del compito 6bis** — *la cura delle tre voci del Passo 8: il bordo nei raggi, la barra di
scorrimento, i messaggi* — del piano `docs/superpowers/plans/2026-09-23-design-system.md`, nel repository `<repo>`. Sei un
subagente fresco: tutto ciò che ti serve è qui e nel brief che questo prompt nomina. Il compito è **codice della GUI** e la
**tavola dei token** — due file nuovi, uno riscritto per intero, sedici toccati, due dei quali da uno script —, e **nessuna
cella** del piano.

**All'avvio verifichi, e se non torna ti fermi e lo riporti:** `git rev-parse --short HEAD` → `<HEAD>`;
`git status --porcelain` → vuoto; `git log --oneline dc77fc8..HEAD -- gui/` → nulla; `node --version` → una versione che
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
| Google Chrome | `154.0.8037.58`, letto dal nome della cartella il 2026-09-27 dal pre-controllo: si rilegge | si rilegge: si aggiorna **da sé** |
| Node | v24.19.0, il 2026-09-27 | v24.19.0 |

---

## 1. Che cosa leggi, e che cosa no

La cartella di lavoro è `.superpowers/sdd/2026-09-23-design-system/`, git-ignorata; il brief ci nasce da
`_extract_brief_6bis.py`, nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`:

| File | Che cos'è |
|---|---|
| `task-6bis-brief.md` | **il compito**: la testa del piano (obiettivo, architettura, pila, disegno, **strumenti** con `replace_unique.py`), i *Vincoli globali*, *A che punto è* e *Come si esegue un compito*, l'**errata** — **E59**, **E60** ed **E61** sono le tre voci che il compito cura, **E62** ed **E63** lo hanno scritto, **E64**…**E69** sono del suo pre-controllo e sono **già applicate** al suo testo —, le voci **P-16** e **P-23**, le decisioni **D9** e **D25**…**D29**, le voci aperte che il piano sa, **il compito 6bis intero**, e dal disegno le risposte **4** e **20**, il linguaggio visivo, le sezioni **(a)**, **(b)** e **(f)**, la barra e la fascia della **(d)**, i controlli **22** e **23**, le trappole **21**…**28** e le decisioni **28** e **29** del coordinatore — **copiati parola per parola** da `_extract_brief_6bis.py`, a `HEAD` = `<HEAD>`. Leggilo **tutto**, a blocchi |

Poi, **per le sole parti che il compito nomina o che modifichi**, e **prima** di scriverle: i file della lista *Files* del
compito che esistono; `gui/src/components/BaseIcon.vue`, `BaseButton.vue` e `BaseStatus.vue`, di cui `BaseNotice` usa
props, slot e forma; `gui/src/stores/connection.ts` e `gui/src/stores/core.ts`, che la fascia e Stato leggono; e le righe
`| **6** |` e `| **6bis** |` della tabella della posizione del piano, **da non toccare**.

⛔ **Non leggi:** il piano intero (oltre 11 000 righe: ti serve il brief); il disegno intero; `docs/HANDOFF.md`; la cartella
`docs/adr/`; `docs/archivio/`; l'audit; le copie del pre-controllo, se questa macchina le ha — lo scratchpad della sessione
del pre-controllo, sulla macchina `Jays` —, che sono il confronto della revisione e non il tuo modello. La lettura
d'apertura di `CLAUDE.md` — il compendio per intero — l'ha fatta il coordinatore, e le sue **regole** valgono per te: codice
in inglese e documenti in italiano, nessuna cifra nuova senza il suo comando, i fine-riga conservati per file, mai `sed -i`.

## 2. Il pre-controllo, e le sue sei voci

Il **pre-controllo** del compito 6bis — sulla macchina `Jays`, il 2026-09-27 — ha rifatto il compito per intero dal testo
del piano su un `git worktree` di `dc77fc8`, poi dal testo corretto su una copia pulita, e sopra il 6bis ha applicato i
compiti 7 e 8. Ha trovato **sei difetti**, già scritti nell'errata e **già applicati** al testo che hai nel brief:

| Voce | Che cosa cambia per te |
|---|---|
| **E64** | niente nel tuo compito: corregge i testi dei compiti 7, 8 e 9 che il 6bis rende falsi. ⛔ Non li tocchi |
| **E65** | la prova *«dresses each tone of a message in its roles»* del Passo 10 guarda anche il titolo contro `--color-text` e il testo contro `--color-text-muted`, con la sua guardia; e una riga in più nella tabella del Passo 15 |
| **E66** | il Passo 7 porta una terza sostituzione, il commento dei raggi di `gui/src/components/BaseDialog.vue`, e il file è nella riga *Files* |
| **E67** | niente nel tuo compito: il richiamo nel disegno sul colore calcolato, **D28** |
| **E68** | niente nel tuo compito: lo sguardo del revisore nomina il 6bis |
| **E69** | ⛔ **i Passi 16 e 17 cambiano per te**: il Passo 16 **non lo fai** — è del proprietario col coordinatore, dopo la revisione —, e al Passo 17 fai **solo il commit del codice**, senza le celle della posizione e senza il disegno |

## 3. I numeri — misurati dal pre-controllo, sulla macchina `Jays`

**Il cancello d'apertura del pre-controllo**, a `dc77fc8`: `GATE GREEN`; sotto `gui/` il progetto `jsdom` con **19** file
passati e uno saltato, **137** prove passate e una saltata, il progetto `browser` con **5** file e **48** prove; il pezzo
JavaScript `690.61 kB`, compresso `210.41 kB`; `found 0 vulnerabilities`. ⛔ Li **rimisuri tu** prima di toccare.

**Misurati dal pre-controllo**, sulla copia — ⚠️ sono **riferimenti**, non Attesi nuovi: tu **misuri**, e un numero diverso
si **riporta**, non si insegue:

| Passo | Uscita del pre-controllo |
|---|---|
| 1 | i raggi `/* 20 */` e `/* 32 */`; `0` e `0` per `scrollbar`; `launchOptions: { channel: "chrome" }`; `2` righe con `1.5`; i due file nuovi assenti; `690.61 kB` |
| 2 | `Tests  1 failed \| 7 passed (8)`, la sola prova nuova del pixel del bordo |
| 3 | `Tests  6 failed \| 36 passed (42)`: le righe di `BaseList`, il pulsante nell'angolo della finestra, il gruppo staccato — sette voci `radius … distance 13.0/13.0` e `1.0/1.0` |
| 4 | `Error: the token --size-scrollbar is not defined here: are the token sheets loaded?` |
| 5 | jsdom `Tests  17 passed (17)`; browser `Tests  5 failed \| 36 passed (41)` — la barra `{ vertical: +0, horizontal: +0 }`, la cornice della pagina kit a 21/34, il dock a 21 —; `scrollbar` **17** volte in `base.css`; `themes.css` invariato |
| 6 | `Tests  7 passed (7)` |
| 7 | `Tests  42 passed (42)` |
| 8 | `Failed to resolve import "./BaseNotice.vue"` |
| 9 | `Tests  31 passed (31)` |
| 10 | `Tests  6 failed \| 16 passed (22)` |
| 11 | `Tests  22 passed (22)` |
| 12 | jsdom `Tests  4 failed \| 27 passed (31)`, `Unable to get .base-notice within`; browser `Tests  2 failed (2)`, `expected +0 to be close to 12` |
| 13 | jsdom `Tests  31 passed (31)`; browser `Tests  2 passed (2)` |
| 14 | `Test Files  25 passed \| 1 skipped (26)`, `Tests  200 passed \| 1 skipped (201)`; il pezzo **`691.82 kB`**, compresso `210.77 kB`; *build* e linter puliti |
| 15 | le **sedici** righe coi messaggi della tabella, ciascuna rossa sulla sua prova, e alla fine `git status` coi soli diciannove file del compito |

La suite intera è stata stabile: **5** corse su 5, 200 prove passate ciascuna. ⛔ Se una prova cade anche **una** volta, lo
**riporti** con la sua uscita: una caduta è una voce d'errata, non una corsa da ripetere finché passa (P-19, D23).

## 4. I fine-riga

⛔ **Nessuna etichetta di forma: si misura** (E72). **Prima** di toccare, sui file della lista *Files* che esistono,
`git ls-files --eol <file>` e `tr -cd '\r' < <file> | wc -c` contro `wc -l`: sono le colonne di **questa** macchina, e
dipendono dal suo `core.autocrlf` (§0). **Dopo**, la **forma**, non il numero di prima — un file che cresce ha più righe:
su un file CRLF i CR sono **uguali alle righe**, su un file LF sono **zero**, e la colonna `w/…` è quella di prima; i file
**nuovi** nascono **LF**, zero CR; un file riscritto per intero tiene il terminatore che ha oggi. Scrivi con Python
`newline=""` (temporaneo più `os.replace`) o con `replace_unique.py`, che conserva il fine-riga del file che trova e che
copi dagli *Strumenti* del brief **nello scratchpad** `<scratchpad>`, mai nel repository; lì vanno anche i log, e i due
script del Passo 5, `cure_board.py` ed `extract_tokens.py`. ⛔ **Mai `sed -i`.**

## 5. Ciò che da qui non si misura, e come lo fai

- ⚠️ **Ogni direzione rossa si torna indietro con la COPIA SALVATA, mai con `git checkout`** (vincolo 11): prima della prima
  violazione `git status --porcelain > <scratchpad>/prima.txt` e una copia di ogni file che le violazioni toccano —
  `gui/src/testing/probes.ts`, `gui/src/tokens/base.css`, `gui/vite.config.ts`, `gui/src/tokens/dock.css`,
  `gui/src/kit/Kit.vue`, `gui/src/components/BaseNotice.vue`, `gui/src/frame/Band.vue`, `gui/src/panels/Status.vue` e
  `gui/src/panels/Settings.vue`, che questo compito ha già scritto o cambiato —; dopo **ciascuna** la copia torna e `cmp` lo
  conferma; alla fine `git status --porcelain | diff <scratchpad>/prima.txt -` rende soltanto i file del compito. Per ogni
  violazione riporti il **messaggio rosso vero** — la prima riga che nomina la ragione — e **quali** prove cadono, non
  «rosso».
- ⚠️ **Il cancello dura da uno a dieci minuti**, secondo le cache. Lancialo **da solo**, in background, con l'uscita in un
  log datato nello scratchpad, e aspetta la notifica: una chiamata in primo piano scade. **Due cancelli insieme si
  pestano** su `gui/node_modules` (gotcha #133): mai un `npm`, un `vitest` o un `npm run dev` mentre gira.
- ⚠️ **Chrome si aggiorna da sé fra una corsa e l'altra** (lezione 5 della consegna dell'esecuzione del compito 3): un rosso
  del progetto `browser` che non nomina una prova — `Target page, context or browser has been closed`, `Failed to connect to
  the browser session` — si rilancia **dopo** aver riletto la versione dal nome della cartella, prima di cercarne la causa
  nel codice.
- ⚠️ **Dal Passo 6 il progetto `browser` mostra le barre di scorrimento** (**D26**): è voluto, e il pre-controllo ha misurato
  che nessun'altra prova cambia. Se una prova del browser si sposta di 10 px, lo **riporti**.
- ⛔ **Il Passo 16 non lo fai** (**E69**): l'aspetto lo giudica il proprietario, col coordinatore, dopo la revisione. Se per
  una misura apri la SPA con `(cd gui && npm run dev)`, lo fermi **per PID** prima del cancello — chiudere la shell non
  basta, il suo `node` resta in ascolto (lezione 3 della consegna dell'esecuzione del compito 4).
- ⚠️ **La CI** la legge il coordinatore dopo il push, che è suo.

## 6. Il contratto

1. **Niente subagenti.** Lavori tu, un passo per volta, **nell'ordine del compito**: le prove prima del codice — Passo 2 e 3,
   la sonda; Passo 4 e 6, la barra; Passo 8 e 9, il pezzo; Passo 10 e 11, la pagina kit; Passo 12 e 13, i tre usi —, i rossi
   guardati: è il ciclo di `superpowers:test-driven-development`, e il piano lo ha già scritto.
2. **Un commit solo**, coi soli file del compito: i nuovi `gui/src/components/BaseNotice.vue` e
   `gui/src/frame/frame.browser.test.ts`; il riscritto `gui/src/frame/Band.vue`; i toccati
   `docs/superpowers/specs/2026-09-22-design-system-tavole/token.html` e `gui/src/tokens/base.css` — dai due script del
   Passo 5 —, `gui/src/testing/probes.ts`, `gui/src/testing/probes.browser.test.ts`, `gui/vite.config.ts`,
   `gui/src/tokens/tokens.browser.test.ts`, `gui/src/tokens/dock.css`, `gui/src/components/BaseDialog.vue`,
   `gui/src/components/icons.ts`, `gui/src/components/kit.test.ts`, `gui/src/kit/Kit.vue`,
   `gui/src/kit/kit.browser.test.ts`, `gui/src/panels/Settings.vue`, `gui/src/panels/Status.vue`,
   `gui/src/panels/modules.test.ts` e `gui/src/frame/frame.test.ts`. ⛔ **Nessuna cella del piano, e il disegno no**
   (**E69**). Il messaggio sta in un file nello scratchpad e si passa con `git commit -F <file>`, comincia con
   `design-system(compito 6bis): ` (vincolo 15), e porta il pezzo JavaScript **prima e dopo** (N-2). ⛔ **Senza
   co-autore**: `CLAUDE.md` prevale su qualunque promemoria d'attribuzione. ⛔ **Niente `git push`**, niente `--amend`,
   niente rebase: il push è del coordinatore, **dopo la revisione**.
3. **Prima del commit**, uno alla volta: `bash scripts/gate.sh` → `GATE GREEN` e `bash scripts/check-docs.sh` → `OK`;
   e `git status --porcelain` che nomina **solo** i file del punto 2 — nessun file nato dalle prove.
4. ⛔ **Se il compito dice il falso, ti fermi e lo riporti**: una divergenza è una **voce d'errata candidata** — la
   prossima libera è **E70** — nel rapporto, con l'evidenza: il comando e la sua uscita. **Mai** un aggiustamento
   silenzioso. Se la divergenza ti impedisce di chiudere un passo, **non committi** e rendi `BLOCKED`.
5. **Non tocchi** `crates/`, `.github/`, `Cargo.*`, `gui/schema/`, `gui/package.json`, `gui/package-lock.json`,
   `docs/adr/`, `scripts/`, il piano, il disegno, né altri documenti oltre alla tavola dei token del Passo 5.

## 7. Il rapporto

Scrivi `.superpowers/sdd/2026-09-23-design-system/task-6bis-report.md`, con:

1. lo **stato** — `DONE`, `DONE_WITH_CONCERNS`, `BLOCKED` o `NEEDS_CONTEXT` — e l'**hash** del commit;
2. `git show --stat HEAD`;
3. per **ogni passo** dall'1 al 15, i comandi lanciati e le uscite **vere**: i rossi coi nomi delle prove, nei due progetti;
   al Passo 5 le uscite dei due script; al Passo 14 i conti e il *build* col pezzo JavaScript;
4. la tabella del Passo 15, col messaggio rosso vero di ogni riga, i `cmp` e il `diff` finale;
5. i **fine-riga**, per ogni file toccato: prima e dopo, le due colonne;
6. il cancello: le righe `Test Files` e `Tests` dei due progetti, e `found 0 vulnerabilities`;
7. le **divergenze**, come voci candidate `E70`… con l'evidenza, e ciò che non hai potuto misurare;
8. il tempo, e i percorsi dei log del cancello.

Nel messaggio finale al coordinatore: lo stato, l'hash e il percorso del rapporto, in poche righe.
