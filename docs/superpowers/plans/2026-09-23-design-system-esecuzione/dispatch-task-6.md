> ⚠️ **Per il coordinatore, prima di dispacciare** — questo file è il **modello**, e viaggia con git. Il prompt che parte
> si scrive nella cartella di lavoro `.superpowers/sdd/2026-09-23-design-system/`, ignorata, **senza** questo riquadro e
> coi campi fra `<…>` riempiti: `<repo>`, `<HEAD>` — l'ultimo commit di `main` —, `<scratchpad>`, `<data>` — il giorno del
> dispaccio, che il Passo 9 scrive nella riga 6 della posizione —, e i valori della macchina del §0 misurati, non copiati.
> Il brief si genera **prima**, dalla radice del repository, con
> `python docs/superpowers/plans/2026-09-23-design-system-esecuzione/_extract_brief_6.py`, e deve dire *«piano e disegno
> coincidono con `HEAD`»*. Alla chiusura del compito il prompt spedito, il rapporto, il prompt del revisore e la revisione
> si copiano nella cartella tracciata e si committano: il punto 8 di *«Come si esegue un compito»*.

Sei l'**implementatore del compito 6** — *il dock vestito: il tema nostro, le schede, la presa grande coi pezzi del kit* —
del piano `docs/superpowers/plans/2026-09-23-design-system.md`, nel repository `<repo>`. Sei un subagente fresco: tutto ciò
che ti serve è qui e nel brief che questo prompt nomina. Il compito è **codice della GUI** — quattro file nuovi, quattro
riscritti per intero, sette toccati, fra cui la sonda dei raggi del compito 4 — e **due celle** del piano.

**All'avvio verifichi, e se non torna ti fermi e lo riporti:** `git rev-parse --short HEAD` → `<HEAD>`;
`git status --porcelain` → vuoto; `node --version` → una versione che `gui/package.json` accetta (`engines`);
`git config --show-origin --get-all core.autocrlf` → il valore di questa macchina, nel §0; Google Chrome stabile
installato, e la sua versione **letta dal nome della cartella** —
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
| Google Chrome | `154.0.8037.58`, letto dal nome della cartella il 2026-09-26 | `154.0.8037.58` dalla consegna dell'esecuzione del compito 3: si aggiorna **da sé** — si rilegge |
| Node | v24.19.0 | v24.19.0 |

---

## 1. Che cosa leggi, e che cosa no

La cartella di lavoro è `.superpowers/sdd/2026-09-23-design-system/`, git-ignorata; il brief ci nasce da
`_extract_brief_6.py`, nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`:

| File | Che cos'è |
|---|---|
| `task-6-brief.md` | **il compito**: la testa del piano (obiettivo, architettura, pila, disegno, **strumenti** con `replace_unique.py`), i *Vincoli globali*, *A che punto è* e *Come si esegue un compito*, l'**errata** — **E44**…**E48** sono del compito 6; **E43** è la riga di `Frame.vue` che il suo Passo 4 tiene; **E29**, **E30** ed **E33** sono della sonda dei raggi che il compito corregge —, le voci **P-6**, **P-7** e **P-15**…**P-18**, le decisioni **D9**…**D11**, le voci aperte che il piano sa, **il compito 6 intero**, e dal disegno le risposte **4**, **5**, **18** e **20**, le sezioni **(c)** e **(f)**, *«Cosa questo disegno ha misurato»*, i controlli **15** e **16**, le trappole **5** e **9** e la decisione **23** del coordinatore — **copiati parola per parola** da `_extract_brief_6.py`, a `HEAD` = `<HEAD>`. Leggilo **tutto**, a blocchi |

Poi, **per le sole parti che il compito nomina o che modifichi**, e **prima** di scriverle: i file della lista *Files* del
compito che esistono; `gui/src/tokens/base.css` per i token che il tema nomina; `gui/src/components/BaseButton.vue`,
`BaseLabel.vue` e `icons.ts`, di cui la faccia della presa usa props ed eventi; e le righe `| **5** |` e `| **6** |` della
tabella della posizione del piano.

⛔ **Non leggi:** il piano intero (oltre 9 100 righe: ti serve il brief); il disegno intero; `docs/HANDOFF.md`; la cartella
`docs/adr/`; `docs/archivio/`; l'audit; le copie del pre-controllo, se questa macchina le ha — `%TEMP%\pc6`, `%TEMP%\pc6b` e
`%TEMP%\pc6c` sulla macchina `Jays` —, che sono il confronto della revisione e non il tuo modello. La lettura d'apertura di
`CLAUDE.md` — il compendio per intero — l'ha fatta il coordinatore, e le sue **regole** valgono per te: codice in inglese e
documenti in italiano, nessuna cifra nuova senza il suo comando, i fine-riga conservati per file, mai `sed -i`.

## 2. Il pre-controllo, e le sue cinque voci

Il **pre-controllo** del compito 6 — sulla macchina `Jays`, il 2026-09-26 — ha rifatto il compito per intero dal testo del
piano su una copia pulita, poi dal testo corretto su una seconda, e ha aperto la SPA in Chrome nei due temi, con le barre
di scorrimento accese. Ha trovato **cinque difetti**, già scritti nell'errata e **già applicati** al testo del compito che
hai nel brief:

| Voce | Che cosa cambia per te |
|---|---|
| **E44** | la prova dei raggi di `dock.browser.test.ts` **stacca un gruppo** con l'aiutante `floatStatus`, e giudica il contenitore galleggiante; la sonda `concentricRadii` non giudica più ciò che arriva a un angolo **attraverso una scatola che scorre** — una sostituzione in `probes.ts` al Passo 4, e la sua prova a mano in `probes.browser.test.ts` al Passo 3 |
| **E45** | `dock.css` porta la regola della presa — niente `padding` verticale sulle linguette della striscia —, e `dock.browser.test.ts` una prova sulla sua altezza |
| **E46** | la sonda legge il raggio che il CSS **disegna**: la seconda sostituzione in `probes.ts`, e due prove a mano |
| **E47** | nel blocco del contenitore galleggiante di `dock.css` i quattro fondi delle linguette a `--color-bg-raised`, e un'attesa in più nella prova del livello |
| **E48** | il Passo 8 nomina il gruppo staccato e le barre di scorrimento all'angolo della scheda, e dice di guardare con le barre accese |

## 3. I numeri — misurati dal pre-controllo, sulla macchina `Jays`

**Il cancello d'apertura del pre-controllo**, a `3a8cfdd`: `GATE GREEN`; sotto `gui/` il progetto `jsdom` con **18** file
passati e uno saltato, **132** prove passate e una saltata, il progetto `browser` con **4** file e **28** prove; il pezzo
JavaScript `689.65 kB`; `found 0 vulnerabilities`. ⛔ Li **rimisuri tu** prima di toccare.

**Misurati dal pre-controllo**, sulla copia col compito rifatto dal testo corretto — ⚠️ sono **riferimenti**, non Attesi
nuovi: tu **misuri**, e un numero diverso si **riporta**, non si insegue:

| Passo | Uscita del pre-controllo |
|---|---|
| 2 | rosso `Failed to import test file …/tokens.browser.test.ts`; poi `Tests  6 passed (6)` |
| 3 | sotto jsdom `Tests  3 failed \| 12 passed (15)`, coi tre messaggi dell'Atteso; nel browser `Tests  10 failed \| 5 passed (15)` — per tema le schede e il contenitore `'0px' to be '20px'`, la presa `[ 27, 27, 27, 27, 27 ]`, il livello `999 to be 50`; nella sonda la barra e ciò che scorre |
| 4 | jsdom `Test Files  8 passed (8)`, `Tests  37 passed (37)`; browser `Test Files  3 passed (3)`, `Tests  21 passed (21)` |
| 5 | i tre rossi dell'Atteso, poi `Tests  3 passed (3)`; `npm run lint` esce 0 |
| 6 | `npm test` verde: `Test Files  24 passed \| 1 skipped (25)`, `Tests  177 passed \| 1 skipped (178)`; il pezzo JavaScript **`690.57 kB`**, compresso `210.40 kB` |
| 7 | le **venti** righe coi messaggi della tabella del Passo 7, e alla fine `git status` coi soli quindici file del compito |
| 8 | il contrasto peggiore **6,22** nel chiaro e **6,41** nello scuro, su 33 scritte; le cinque prese a 40 px, `padding` `0px 8px`; nessun fotogramma con la pagina che sborda quando la fascia entra; `base-dialog-veil` sopra il gruppo staccato |

Le prove del dock e della sonda sono state stabili: **10** corse su 10 da sole, **5** su 5 nella suite intera. ⛔ Se una
cade anche **una** volta, lo **riporti** con la sua uscita: una caduta è una voce d'errata, non una corsa da ripetere
finché passa (P-19, D23).

## 4. I fine-riga

⛔ **Nessuna etichetta di forma: si misura** (E72). **Prima** di toccare, sui file della lista *Files* che esistono,
`git ls-files --eol <file>` e `tr -cd '\r' < <file> | wc -c` contro `wc -l`: sono le colonne di **questa** macchina, e
dipendono dal suo `core.autocrlf` (§0). **Dopo**, la **forma**, non il numero di prima — un file che cresce ha più righe:
su un file CRLF i CR sono **uguali alle righe**, su un file LF sono **zero**, e la colonna `w/…` è quella di prima; i file
**nuovi** nascono **LF**, zero CR; un file riscritto per intero tiene il terminatore che ha oggi. Scrivi con Python
`newline=""` (temporaneo più `os.replace`) o con `replace_unique.py`, che conserva il fine-riga del file che trova e che
copi dagli *Strumenti* del brief **nello scratchpad** `<scratchpad>`, mai nel repository; lì vanno anche i log.
⛔ **Mai `sed -i`.**

## 5. Ciò che da qui non si misura, e come lo fai

- ⚠️ **Ogni direzione rossa si torna indietro con la COPIA SALVATA, mai con `git checkout`** (vincolo 11): prima della prima
  violazione `git status --porcelain > <scratchpad>/prima.txt` e una copia di ogni file che le violazioni toccano —
  `gui/src/tokens/dock.css`, `gui/src/tokens/dock.test.ts`, `gui/src/frame/dock.ts`, `gui/src/frame/BigTab.ts`,
  `gui/src/frame/BigTabFace.vue`, `gui/src/jsdom-setup.ts`, `gui/src/tokens/readToken.ts` e `gui/src/testing/probes.ts`,
  che questo compito ha già scritto o cambiato —; dopo **ciascuna** la copia torna e `cmp` lo conferma; alla fine
  `git status --porcelain | diff <scratchpad>/prima.txt -` rende soltanto i file del compito. Per ogni violazione riporti il
  **messaggio rosso vero** — la prima riga che nomina la ragione — e **quali** prove cadono, non «rosso».
- ⚠️ **Il cancello dura da uno a dieci minuti**, secondo le cache. Lancialo **da solo**, in background, con l'uscita in un
  log datato nello scratchpad, e aspetta la notifica: una chiamata in primo piano scade. **Due cancelli insieme si
  pestano** su `gui/node_modules` (gotcha #133): mai un `npm`, un `vitest` o un `npm run dev` mentre gira.
- ⚠️ **Chrome si aggiorna da sé fra una corsa e l'altra** (lezione 5 della consegna dell'esecuzione del compito 3): un rosso
  del progetto `browser` che non nomina una prova — `Target page, context or browser has been closed`, `Failed to connect to
  the browser session` — si rilancia **dopo** aver riletto la versione dal nome della cartella, prima di cercarne la causa
  nel codice.
- ⚠️ **Il Passo 8**: le sue due misure — il frammento del contrasto nei due temi, e `elementFromPoint` col cassetto aperto
  sopra il gruppo staccato — le fai se hai un modo di aprire la pagina, per esempio Playwright col Chrome installato, **con
  le barre di scorrimento accese**: `ignoreDefaultArgs: ["--hide-scrollbars"]` (**E48**). ⛔ **L'aspetto non lo giudichi
  tu**: lo giudica il proprietario (controllo 15, **D24**), e glielo porta il coordinatore. `(cd gui && npm run dev)` lo
  fermi **per PID** prima del cancello — chiudere la shell non basta, il suo `node` resta in ascolto (lezione 3 della
  consegna dell'esecuzione del compito 4). Se non puoi aprire la pagina, lo dici nel rapporto, e non è un difetto.
- ⚠️ **La CI** la legge il coordinatore dopo il push, che è suo.

## 6. Il contratto

1. **Niente subagenti.** Lavori tu, un passo per volta, **nell'ordine del compito**: le prove prima del codice (Passo 2, la
   prova di `readToken`; Passo 3, le prove del dock e della sonda, i rossi guardati; Passo 5, la prova della presa) — è il
   ciclo di `superpowers:test-driven-development`, e il piano lo ha già scritto.
2. **Un commit solo**, coi soli file del compito: i nuovi `gui/src/tokens/readToken.ts`, `gui/src/tokens/dock.test.ts`,
   `gui/src/frame/BigTabFace.vue` e `gui/src/frame/dock.browser.test.ts`; i riscritti `gui/src/tokens/dock.css`,
   `gui/src/frame/BigTab.ts`, `gui/src/frame/bigtab.test.ts` e `gui/src/jsdom-setup.ts`; i toccati `gui/src/frame/dock.ts`,
   `gui/src/frame/Frame.vue`, `gui/src/frame/frame.test.ts`, `gui/src/tokens/tokens.browser.test.ts`,
   `gui/src/testing/probes.ts`, `gui/src/testing/probes.browser.test.ts` e `gui/eslint.config.js`; e il piano, dove nella
   riga **5** della tabella della posizione la colonna **Commit** diventa `` `545f500`, con le cure `7e25d03` e `9e6657b` ``
   (R1-16), e la riga **6** diventa **Stato** `✅ <data>`. Il messaggio sta in un file nello scratchpad e si passa con
   `git commit -F <file>`, comincia con `design-system(compito 6): ` (vincolo 15), e porta il pezzo JavaScript **prima e
   dopo** (N-2). ⛔ **Senza co-autore**: `CLAUDE.md` prevale su qualunque promemoria d'attribuzione. ⛔ **Niente
   `git push`**, niente `--amend`, niente rebase: il push è del coordinatore, **dopo la revisione** — il Passo 9 dice
   `git push`, e questo contratto lo sposta, come nei compiti 1–5.
3. **Prima del commit**, uno alla volta: `bash scripts/gate.sh` → `GATE GREEN` e `bash scripts/check-docs.sh` → `OK`;
   e `git status --porcelain` che nomina **solo** i file del punto 2 — nessun file nato dalle prove.
4. ⛔ **Se il compito dice il falso, ti fermi e lo riporti**: una divergenza è una **voce d'errata candidata** — la
   prossima libera è **E49** — nel rapporto, con l'evidenza: il comando e la sua uscita. **Mai** un aggiustamento
   silenzioso. Se la divergenza ti impedisce di chiudere un passo, **non committi** e rendi `BLOCKED`.
5. **Non tocchi** `crates/`, `.github/`, `Cargo.*`, `gui/schema/`, `gui/package.json`, `gui/package-lock.json`,
   `docs/adr/`, `scripts/`, né altri documenti oltre alle due celle del piano.

## 7. Il rapporto

Scrivi `.superpowers/sdd/2026-09-23-design-system/task-6-report.md`, con:

1. lo **stato** — `DONE`, `DONE_WITH_CONCERNS`, `BLOCKED` o `NEEDS_CONTEXT` — e l'**hash** del commit;
2. `git show --stat HEAD`;
3. per **ogni passo**, i comandi lanciati e le uscite **vere**: al Passo 3 i rossi coi nomi delle prove, nei due progetti;
   al Passo 4 i conti; al Passo 6 i conti e il *build* col pezzo JavaScript;
4. la tabella del Passo 7, col messaggio rosso vero di ogni riga, i `cmp` e il `diff` finale;
5. i **fine-riga**, per ogni file toccato: prima e dopo, le due colonne;
6. il cancello: le righe `Test Files` e `Tests` dei due progetti, e `found 0 vulnerabilities`;
7. il Passo 8: le misure fatte, o perché non hai potuto — l'aspetto è del proprietario;
8. le **divergenze**, come voci candidate `E49`… con l'evidenza, e ciò che non hai potuto misurare;
9. il tempo, e i percorsi dei log del cancello.

Nel messaggio finale al coordinatore: lo stato, l'hash e il percorso del rapporto, in poche righe.
