> ⚠️ **Per il coordinatore, prima di dispacciare** — questo file è il **modello**, e viaggia con git. Il prompt che parte
> si scrive nella cartella di lavoro `.superpowers/sdd/2026-09-23-design-system/`, ignorata, **senza** questo riquadro e
> coi campi fra `<…>` riempiti: `<repo>`, `<HEAD>` — l'ultimo commit di `main` —, `<scratchpad>`, e i valori della macchina
> del §0 misurati, non copiati. Il brief si genera **prima**, dalla radice del repository, con
> `python docs/superpowers/plans/2026-09-23-design-system-esecuzione/_extract_brief_7.py`, e deve dire *«piano e disegno
> coincidono con `HEAD`»*. ⛔ **Il codice del compito è provato su `1c168a1`**: `git log --oneline 1c168a1..HEAD -- gui/`
> deve rendere nulla, o il compito si rilegge contro il codice di allora (`CLAUDE.md`, domanda 5). Alla chiusura del compito
> il prompt spedito, il rapporto, il prompt del revisore e la revisione si copiano nella cartella tracciata e si committano:
> il punto 8 di *«Come si esegue un compito»*. Il compito 7 **non ha uno sguardo del proprietario**: la regola 5 di
> *«Come si esegue un compito»* non lo nomina, e il revisore confronta il commit col testo con `compare_task7.py`.

Sei l'**implementatore del compito 7** — *le viste col nome, sotto: il pacchetto, il negozio, il dock, la geometria comune, lo
schema* — del piano `docs/superpowers/plans/2026-09-23-design-system.md`, nel repository `<repo>`. Sei un subagente fresco:
tutto ciò che ti serve è qui e nel brief che questo prompt nomina. Il compito è **codice della GUI** — quattro file nuovi, uno
riscritto per intero, cinque toccati — e la **riga 7** della tabella della posizione del piano.

**All'avvio verifichi, e se non torna ti fermi e lo riporti:** `git rev-parse --short HEAD` → `<HEAD>`;
`git status --porcelain` → vuoto; `git log --oneline 1c168a1..HEAD -- gui/` → nulla; `node --version` → una versione che
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
`_extract_brief_7.py`, nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`:

| File | Che cos'è |
|---|---|
| `task-7-brief.md` | **il compito**: la testa del piano (obiettivo, architettura, pila, disegno, **strumenti** con `replace_unique.py`), i *Vincoli globali*, *A che punto è* e *Come si esegue un compito*, l'**errata** — **E75**…**E81** sono del suo pre-controllo e sono **già applicate** al suo testo, tranne **E81**, che è del compito 8 —, la voce **P-19**, le decisioni **D3**, **D4**, **D5**, **D12** e **D13**, le voci aperte che il piano sa, **il compito 7 intero**, e dal disegno le risposte **13** e **19**, la Panoramica e le viste salvate col nome della **(d)**, i controlli **17** e **18** e le decisioni **19** e **20** del coordinatore — **copiati parola per parola** da `_extract_brief_7.py`, a `HEAD` = `<HEAD>`. Leggilo **tutto**, a blocchi |

Poi, **per le sole parti che il compito nomina o che modifichi**, e **prima** di scriverle: i file della lista *Files* del
compito che esistono; `gui/src/transport/fakeBridge.ts` e `gui/src/schema/messages.ts`, che le prove del negozio usano;
`gui/src/panels/views/index.ts`, le tre viste che `schematic.test.ts` taglia; `gui/src/frame/keys.test.ts`, che tiene
`moveActive`; e le righe `| **6** |`, `| **6bis** |` e `| **7** |` della tabella della posizione del piano — **tocchi solo la
7**.

⛔ **Non leggi:** il piano intero (oltre 11 000 righe: ti serve il brief); il disegno intero; `docs/HANDOFF.md`; la cartella
`docs/adr/`; `docs/archivio/`; l'audit; le copie del pre-controllo, se questa macchina le ha — lo scratchpad della sessione
del pre-controllo, sulla macchina `Jays` —, che sono il confronto della revisione e non il tuo modello. La lettura
d'apertura di `CLAUDE.md` — il compendio per intero — l'ha fatta il coordinatore, e le sue **regole** valgono per te: codice
in inglese e documenti in italiano, nessuna cifra nuova senza il suo comando, i fine-riga conservati per file, mai `sed -i`.

## 2. Il pre-controllo, e le sue sette voci

Il **pre-controllo** del compito 7 — sulla macchina `Jays`, il 2026-09-27 — ha rifatto il compito per intero dal testo del
piano su un `git worktree` di `1c168a1`, poi dal testo corretto su una copia pulita, e sopra il 7 ha applicato il compito 8.
Ha trovato **sette voci**, già scritte nell'errata; sei sono **già applicate** al testo che hai nel brief:

| Voce | Che cosa cambia per te |
|---|---|
| **E75** | ⚠️ il **codice dettato**: nel `receive` del Passo 5 il nome della vista col nome si rilegge **sempre** dal pacchetto, anche quando non ne arriva uno leggibile; il Passo 2 porta una **sesta** prova del negozio, *«closes the named view when the core holds no package it can read…»*; e il Passo 8 una riga in più |
| **E76** | niente da fare: la riga di `Frame.vue` che chiama `showView` non la tiene nessuna prova, ed è **dichiarato**. ⛔ Non aggiungi una prova |
| **E77** | tre righe del Passo 8 nominano le prove in più che fanno cadere: **tu misuri** e riporti |
| **E78** | il Passo 6 porta una **terza** sostituzione in `gui/src/frame/dock.ts`, la testa di `createDock` |
| **E79** | al Passo 9 la riga **6** porta già la colonna **Commit**: ⛔ non la tocchi |
| **E80** | il **pezzo JavaScript**: la baseline al Passo 1, e le due righe nel messaggio del commit |
| **E81** | niente nel tuo compito: è del pre-controllo del compito 8 |

## 3. I numeri — misurati dal pre-controllo, sulla macchina `Jays`

**Il cancello d'apertura del pre-controllo**, a `1c168a1`: `GATE GREEN`; sotto `gui/` il progetto `jsdom` con **19** file
passati e uno saltato, **144** prove passate e una saltata, il progetto `browser` con **6** file e **56** prove; il pezzo
JavaScript `691.82 kB`, compresso `210.77 kB`; `found 0 vulnerabilities`. ⛔ Li **rimisuri tu** prima di toccare.

**Misurati dal pre-controllo**, sulla copia col testo corretto — ⚠️ sono **riferimenti**, non Attesi nuovi: tu **misuri**, e
un numero diverso si **riporta**, non si insegue:

| Passo | Uscita del pre-controllo |
|---|---|
| 1 | `Direction` nel solo `frame/moveActive.ts`, tre righe; i due `grep -rln` vuoti; `691.82 kB`, compresso `210.77 kB` |
| 2 | `Tests  7 failed \| 28 passed (35)`: `Failed to resolve import "./nearest"` e `"./schematic"`, le sei prove nuove del negozio — `saveNamed is not a function` due volte —, e il dock con la Home spedita |
| 3 | `Tests  8 passed (8)` |
| 4 | `Tests  2 passed (2)` |
| 5 | `Tests  20 passed (20)` |
| 6 | `Tests  15 passed (15)` |
| 7 | `Test Files  27 passed \| 1 skipped (28)`, `Tests  212 passed \| 1 skipped (213)`, cinque corse su cinque; *build* e linter puliti; il pezzo **`693.02 kB`**, compresso `211.20 kB` |
| 8 | le **quindici** righe coi messaggi della tabella, ciascuna rossa sulle sue prove, e alla fine `git status` coi soli dieci file del compito |

⛔ Se una prova cade anche **una** volta nelle cinque corse, lo **riporti** con la sua uscita: una caduta è una voce d'errata,
non una corsa da ripetere finché passa (P-19).

## 4. I fine-riga

⛔ **Nessuna etichetta di forma: si misura** (E72). **Prima** di toccare, sui file della lista *Files* che esistono,
`git ls-files --eol <file>` e `tr -cd '\r' < <file> | wc -c` contro `wc -l`: sono le colonne di **questa** macchina, e
dipendono dal suo `core.autocrlf` (§0). **Dopo**, la **forma**, non il numero di prima — un file che cresce ha più righe:
su un file CRLF i CR sono **uguali alle righe**, su un file LF sono **zero**, e la colonna `w/…` è quella di prima; i file
**nuovi** nascono **LF**, zero CR; `gui/src/stores/layout.ts`, riscritto per intero, tiene il terminatore che ha oggi. Scrivi
con Python `newline=""` (temporaneo più `os.replace`) o con `replace_unique.py`, che conserva il fine-riga del file che trova e
che copi dagli *Strumenti* del brief **nello scratchpad** `<scratchpad>`, mai nel repository; lì vanno anche i log. ⛔ **Mai
`sed -i`.**

## 5. Ciò che da qui non si misura, e come lo fai

- ⚠️ **Ogni direzione rossa si torna indietro con la COPIA SALVATA, mai con `git checkout`** (vincolo 11): prima della prima
  violazione `git status --porcelain > <scratchpad>/prima.txt` e una copia di ogni file che le violazioni toccano —
  `gui/src/frame/nearest.ts`, `gui/src/frame/schematic.ts`, `gui/src/stores/layout.ts` e `gui/src/frame/dock.ts`, che
  questo compito ha già scritto o cambiato —; dopo **ciascuna** la copia torna e `cmp` lo conferma; alla fine
  `git status --porcelain | diff <scratchpad>/prima.txt -` rende soltanto i file del compito. Lancia ogni violazione sulla
  **suite intera**, `npx vitest run` sotto `gui/`, i due progetti: tre righe fanno cadere prove di altri file (**E77**). Per
  ogni violazione riporti il **messaggio rosso vero** — la prima riga che nomina la ragione — e **quali** prove cadono, non
  «rosso».
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
   intero, rosso, poi i Passi 3–6 —, i rossi guardati: è il ciclo di `superpowers:test-driven-development`, e il piano lo ha
   già scritto.
2. **Un commit solo**, coi soli file del compito: i nuovi `gui/src/frame/nearest.ts`, `gui/src/frame/nearest.test.ts`,
   `gui/src/frame/schematic.ts` e `gui/src/frame/schematic.test.ts`; il riscritto `gui/src/stores/layout.ts`; i toccati
   `gui/src/stores/stores.test.ts`, `gui/src/frame/moveActive.ts`, `gui/src/frame/dock.ts`, `gui/src/frame/Frame.vue` e
   `gui/src/frame/frame.test.ts`; e il piano, **nella sola riga 7** della tabella della posizione — lo **Stato**
   `✅ <data>`, il Passo 9; ⛔ la riga 6 no (**E79**). Il messaggio sta in un file nello scratchpad e si passa con
   `git commit -F <file>`, comincia con `design-system(compito 7): ` (vincolo 15), e porta il pezzo JavaScript **prima e
   dopo** (**E80**, N-2). ⛔ **Senza co-autore**: `CLAUDE.md` prevale su qualunque promemoria d'attribuzione. ⛔ **Niente
   `git push`**, niente `--amend`, niente rebase: il push è del coordinatore, **dopo la revisione**.
3. **Prima del commit**, uno alla volta: `bash scripts/gate.sh` → `GATE GREEN` e `bash scripts/check-docs.sh` → `OK`;
   e `git status --porcelain` che nomina **solo** i file del punto 2 — nessun file nato dalle prove.
4. ⛔ **Se il compito dice il falso, ti fermi e lo riporti**: una divergenza è una **voce d'errata candidata** — la
   prossima libera è **E82** — nel rapporto, con l'evidenza: il comando e la sua uscita. **Mai** un aggiustamento
   silenzioso. Se la divergenza ti impedisce di chiudere un passo, **non committi** e rendi `BLOCKED`.
5. **Non tocchi** `crates/`, `.github/`, `Cargo.*`, `gui/schema/`, `gui/package.json`, `gui/package-lock.json`,
   `docs/adr/`, `scripts/`, il disegno, né altri documenti; del piano, la sola riga 7 della posizione.

## 7. Il rapporto

Scrivi `.superpowers/sdd/2026-09-23-design-system/task-7-report.md`, con:

1. lo **stato** — `DONE`, `DONE_WITH_CONCERNS`, `BLOCKED` o `NEEDS_CONTEXT` — e l'**hash** del commit;
2. `git show --stat HEAD`;
3. per **ogni passo** dall'1 al 7, i comandi lanciati e le uscite **vere**: i rossi coi nomi delle prove; al Passo 7 le
   cinque corse, il *build* col pezzo JavaScript e il linter;
4. la tabella del Passo 8, col messaggio rosso vero di ogni riga e le prove che cadono, i `cmp` e il `diff` finale;
5. i **fine-riga**, per ogni file toccato: prima e dopo, le due colonne;
6. il cancello: le righe `Test Files` e `Tests` dei due progetti, e `found 0 vulnerabilities`;
7. le **divergenze**, come voci candidate `E82`… con l'evidenza, e ciò che non hai potuto misurare;
8. il tempo, e i percorsi dei log del cancello.

Nel messaggio finale al coordinatore: lo stato, l'hash e il percorso del rapporto, in poche righe.
