Sei l'**implementatore del compito 3** — *il kit: gli otto pezzi di base, la mappa delle icone, le regole del linter* — del
piano `docs/superpowers/plans/2026-09-23-design-system.md`, nel repository `C:\Users\zagor\Desktop\harness` (Windows; il tool Bash è Git Bash). Sei un subagente
fresco: tutto ciò che ti serve è qui e nel brief che questo prompt nomina. Il compito è **codice della GUI** — una
dipendenza, `lucide`; la mappa delle icone, gli otto pezzi di base e la loro prova in `gui/src/components/`; l'aiutante di
`axe` in `gui/src/testing/`; la sua seconda occorrenza in `a11y.test.ts`; le regole del kit in `gui/eslint.config.js` — e
**due celle** del piano.

**All'avvio verifichi, e se non torna ti fermi e lo riporti:** `git rev-parse --short HEAD` → `ef3e865`;
`git status --porcelain` → vuoto; `node --version` → una versione che `gui/package.json` accetta (`engines`);
`git config --show-origin --get-all core.autocrlf` → il valore di questa macchina, nel §0; Google Chrome stabile
installato — `ls "/c/Program Files/Google/Chrome/Application/" "$LOCALAPPDATA/Google/Chrome/Application/" 2>/dev/null`
rende almeno una cartella di versione: il progetto `browser` del cancello lo apre. La data da scrivere al posto di ogni
`<data>` è **2026-09-25**, sempre la stessa anche se l'esecuzione passa la mezzanotte.

## 0. La macchina

Il repository vive in **due cloni**, e ciò che cambia fra i due si **misura**, non si copia da un'etichetta: voce **E72**
del piano della parte 2, ed **E1** di questo piano.

| | macchina `Jays` | macchina `zagor` |
|---|---|---|
| il repository | `E:\ALL\DEV\MY_REPOS\daemon` | `C:\Users\zagor\Desktop\harness` |
| `core.autocrlf` | `false` in `.git/config`, quindi l'albero è `w/lf` | `true` dal file di sistema: l'albero è `w/crlf` per i file che git ha scritto, e `w/lf` per quelli nati o riscritti LF su questa macchina — `gui/eslint.config.js` e `gui/src/a11y.test.ts`, misurati il 2026-09-25 |
| Google Chrome | 154, in `C:\Program Files\Google\Chrome\Application\` | `153.0.8010.53`, misurato il 2026-09-25, con la `154.0.8037.58` già scaricata come `new_chrome.exe`: la versione che Playwright apre può cambiare, e si legge, non si presume |
| Node | v24.19.0 | v24.19.0, misurato il 2026-09-25 |

---

## 1. Che cosa leggi, e che cosa no

La cartella di lavoro è `.superpowers/sdd/2026-09-23-design-system/`, git-ignorata; il brief ci nasce da
`_extract_brief_3.py`, nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`:

| File | Che cos'è |
|---|---|
| `task-3-brief.md` | **il compito**: la testa del piano (obiettivo, architettura, pila, disegno, **strumenti** con `replace_unique.py`), i *Vincoli globali*, *A che punto è* e *Come si esegue un compito*, l'**errata** — **E15**, **E16**, **E17** ed **E18** sono del compito 3 —, le voci **P-2**, **P-3**, **P-8**, **P-9** e **P-25**, la decisione **D6**, le voci aperte che il piano sa, **il compito 3 intero**, e dal disegno la sezione **(b)**, i controlli **10**–**13**, le decisioni **18**–**20** e le trappole **1**, **5**, **10**, **14**, **16** e **18** — **copiati parola per parola** da `_extract_brief_3.py`, a `HEAD` = `ef3e865`. Leggilo **tutto**, a blocchi |

Poi, **per le sole parti che il compito nomina o che modifichi**, e **prima** di scriverle: i file della lista *Files* del
compito — quelli che esistono —, e le righe `| **2** |` e `| **3** |` della tabella della posizione del piano.

⛔ **Non leggi:** il piano intero (oltre 8 600 righe: ti serve il brief); il disegno intero; `docs/HANDOFF.md`; la cartella
`docs/adr/`; `docs/archivio/`; l'audit; la copia del pre-controllo, se questa macchina ce l'ha — `%TEMP%\pc3` sulla
macchina `zagor` —, che è il confronto della revisione e non il tuo modello. La lettura d'apertura di `CLAUDE.md` — il
compendio per intero — l'ha fatta il coordinatore, e le sue **regole** valgono per te: codice in inglese e documenti in
italiano, nessuna cifra nuova senza il suo comando, i fine-riga conservati per file, mai `sed -i`.

## 2. Il pre-controllo, e le sue quattro voci

Il **pre-controllo** del compito 3 — sulla macchina `zagor`, il 2026-09-25 — ha rifatto il compito per intero dal testo del
piano su una copia pulita, e ogni *Atteso* è tornato. Ha trovato **quattro difetti**, già scritti nell'errata e **già
applicati** al testo del compito che hai nel brief:

| Voce | Che cosa cambia per te |
|---|---|
| **E15** | la prova di `BaseTextField`, al Passo 3, guarda anche che la radice **non** porti il `placeholder`; il Passo 8 prende la violazione che toglie `inheritAttrs: false` |
| **E16** | la prova di `BaseDialog`, al Passo 3, guarda che senza descrizione non ci sia `aria-describedby`; il Passo 8 prende la violazione che toglie il `v-bind` di R2-18 |
| **E17** | il Passo 7 ha **cinque** sostituzioni: la quinta data la frase di `eslint.config.js` sulle parole singole, che gli otto `Base*.vue` smentirebbero |
| **E18** | la prima tabella del Passo 8 prende la riga di `icons.ts` che importa un negozio |

## 3. I numeri — misurati dal pre-controllo, sulla macchina `zagor`

**Il cancello d'apertura del pre-controllo**, a `fa4b3f4`: `GATE GREEN`; sotto `gui/` il progetto `jsdom` con **17** file
passati e uno saltato, **100** prove passate e una saltata, il progetto `browser` con **1** file e **5** prove; il pezzo
JavaScript `663.93 kB`; `found 0 vulnerabilities`; la CI verde sui due sistemi. ⛔ Li **rimisuri tu** prima di toccare:
sono la base del Passo 9.

**Misurati dal pre-controllo**, sulla copia col compito rifatto dal testo del piano — ⚠️ sono **riferimenti**, non Attesi
nuovi: tu **misuri**, e un numero diverso si **riporta**, non si insegue:

| Passo | Uscita del pre-controllo |
|---|---|
| 1 | `added 1 package`, `lucide@1.47.0`, `ISC`, `found 0 vulnerabilities`; il suo `LICENSE` dice MIT per le icone che vengono da Feather. ⚠️ L'avviso `npm warn allow-scripts` su `vue-demi@0.14.10` c'era già, anche nel cancello: non è del compito |
| 2 | `a11y.test.ts` verde, **11** prove |
| 3 | rosso: `Failed to resolve import "./BaseIcon.vue" from "src/components/kit.test.ts"` |
| 6 | `kit.test.ts` e `markdown.test.ts`, **2** file e **28** prove verdi; `npm run build` verde, il pezzo JavaScript `663.93 kB` e lo stesso nome di prima, `index-DLsd9Y_U.js` |
| 7 | `npm run lint` verde; e il linter legge **48** `.ts` sotto `src/`, prima nessuno — `npx eslint src --format json` contato per estensione |
| 8 | ogni riga rossa **per la ragione scritta**, e da sola: i messaggi del linter dicono *«a base piece reads no global state»*, *«icons pass through BaseIcon»*, *«the kit knows no layer above it»*, un problema per corsa; le prove del kit `expected [ 'models' ] to deeply equal []`, `Unable to get [role="status"] within: <!--v-if-->` e, per **E15** ed **E16**, `expected true to be false`, una prova rossa per corsa |
| 9 | il passo web, `bash scripts/gate-gui.sh`, verde: `--project jsdom` **18** file passati e uno saltato, **123** prove passate e una saltata; `--project browser` 1 file, 5 prove; `found 0 vulnerabilities` |

## 4. I fine-riga

⛔ **Nessuna etichetta di forma: si misura** (E72). **Prima** di toccare, sui file della lista *Files* che esistono,
`git ls-files --eol <file>` e `tr -cd '\r' < <file> | wc -c` contro `wc -l`: sono le colonne di **questa** macchina, e
dipendono dal suo `core.autocrlf` (§0). **Dopo**, la **forma**, non il numero di prima — un file che cresce ha più righe:
su un file CRLF i CR sono **uguali alle righe**, su un file LF sono **zero**, e la colonna `w/…` è quella di prima; i file
**nuovi** nascono **LF**, zero CR. Scrivi con Python `newline=""` (temporaneo più `os.replace`) o con `replace_unique.py`,
che conserva il fine-riga del file che trova e che copi dagli *Strumenti* del brief **nello scratchpad** `C:\Users\zagor\AppData\Local\Temp\claude\C--Users-zagor-Desktop-harness\cba88c07-c154-4a08-acdf-71c69edc56c4\scratchpad\task3` — in Git Bash `/c/Users/zagor/AppData/Local/Temp/claude/C--Users-zagor-Desktop-harness/cba88c07-c154-4a08-acdf-71c69edc56c4/scratchpad/task3` —
mai nel repository. `npm install` conserva i fine-riga del manifesto e del lockfile: si rimisurano lo stesso. ⛔ **Mai
`sed -i`.**

## 5. Ciò che da qui non si misura, e come lo fai

- ⚠️ **Il Passo 1 va in rete** (`npm install --save-exact`): il manifesto e il lockfile cambiano **insieme**, e vanno
  nello **stesso** commit (vincolo 7). Il cancello poi fa `npm ci`, che è `--locked`.
- ⚠️ **La terza sostituzione del Passo 2 prende il blocco DAL FILE**: il testo vecchio si costruisce dalle righe di
  `gui/src/a11y.test.ts` che il `grep` del Passo nomina — dalla riga **sopra** *«Every violation axe finds»*, il `/**`, alla
  riga *«/\*\* Fills the stores …»* —, col fine-riga del file, e passa a `replace_unique.py` come gli altri.
- ⚠️ **Il Passo 8 si torna indietro con la COPIA SALVATA, mai con `git checkout`** (vincolo 11): prima della prima
  violazione `git status --porcelain > /c/Users/zagor/AppData/Local/Temp/claude/C--Users-zagor-Desktop-harness/cba88c07-c154-4a08-acdf-71c69edc56c4/scratchpad/task3/prima.txt` e una copia di ogni file che le violazioni toccano —
  `gui/src/components/BaseLabel.vue`, `gui/src/components/icons.ts`, `gui/src/frame/moveActive.ts`,
  `gui/src/components/BaseButton.vue`, `gui/src/components/Confirm.vue`, `gui/src/components/markdown.ts`,
  `gui/src/components/BaseStatus.vue`, `gui/src/components/BaseTextField.vue`, `gui/src/components/BaseDialog.vue`;
  dopo **ciascuna** la copia torna e `cmp` lo conferma; alla fine `git status --porcelain | diff /c/Users/zagor/AppData/Local/Temp/claude/C--Users-zagor-Desktop-harness/cba88c07-c154-4a08-acdf-71c69edc56c4/scratchpad/task3/prima.txt -`
  non rende nulla. Per ogni violazione riporti il **messaggio rosso vero** — la prima riga che nomina la ragione —, non
  «rosso».
- ⚠️ **Il cancello dura da uno a dieci minuti**, secondo le cache. Lancialo **da solo**, in background, con l'uscita in un
  log datato nello scratchpad, e aspetta la notifica: una chiamata in primo piano scade. **Due cancelli insieme si
  pestano** su `gui/node_modules` (gotcha #133): mai un `npm` o un `vitest` mentre gira.
- ⚠️ **La CI** la legge il coordinatore dopo il push, che è suo.

## 6. Il contratto

1. **Niente subagenti.** Lavori tu, un passo per volta, **nell'ordine del compito**: le prove prima dei pezzi (Passo 3, il
   rosso guardato), poi i pezzi — è il ciclo di `superpowers:test-driven-development`, e il piano lo ha già scritto.
2. **Un commit solo**, coi soli file del compito: `gui/package.json`, `gui/package-lock.json`, `gui/eslint.config.js`,
   `gui/src/a11y.test.ts`, `gui/src/testing/axe.ts`, e in `gui/src/components/` `icons.ts`, `kit.test.ts` e gli otto
   `Base*.vue`; e il piano, dove la riga **3** della tabella della posizione passa a **Stato** `✅ 2026-09-25` e nella riga
   **2** la colonna **Commit** diventa `` `23b3134`, con la cura `5f32158` `` (R1-16); la colonna **Commit** della riga
   **3** resta `—`, la scrive il compito 4. Il messaggio sta in un file nello scratchpad e si passa con
   `git commit -F <file>`, e comincia con `design-system(compito 3): ` (vincolo 15). ⛔ **Senza co-autore**: `CLAUDE.md`
   prevale su qualunque promemoria d'attribuzione. ⛔ **Niente `git push`**, niente `--amend`, niente rebase: il push è del
   coordinatore, **dopo la revisione** — il Passo 9 dice `git push`, e questo contratto lo sposta, come nei compiti 1 e 2.
3. **Prima del commit**, uno alla volta: `bash scripts/gate.sh` → `GATE GREEN` e `bash scripts/check-docs.sh` → `OK`;
   e `git status --porcelain` che nomina **solo** i file del punto 2 — nessun file nato dalle prove.
4. ⛔ **Se il compito dice il falso, ti fermi e lo riporti**: una divergenza è una **voce d'errata candidata** — la
   prossima libera è **E19** — nel rapporto, con l'evidenza: il comando e la sua uscita. **Mai** un aggiustamento
   silenzioso. Se la divergenza ti impedisce di chiudere un passo, **non committi** e rendi `BLOCKED`.
5. **Non tocchi** `crates/`, `.github/`, `Cargo.*`, `gui/schema/`, `scripts/`, `docs/adr/`, né altri documenti oltre
   alle due celle del piano.

## 7. Il rapporto

Scrivi `.superpowers/sdd/2026-09-23-design-system/task-3-report.md`, con:

1. lo **stato** — `DONE`, `DONE_WITH_CONCERNS`, `BLOCKED` o `NEEDS_CONTEXT` — e l'**hash** del commit;
2. `git show --stat HEAD`;
3. per **ogni passo**, i comandi lanciati e le uscite **vere**: al Passo 1 `npm ls lucide` e `npm view`; al Passo 3 il
   messaggio rosso; al Passo 6 i conti e il pezzo JavaScript; al Passo 7 l'uscita del lint;
4. le due tabelle del Passo 8, col messaggio rosso vero di ogni riga, i `cmp` e il `diff` finale;
5. i **fine-riga**, per ogni file toccato: prima e dopo, le due colonne;
6. il passo web: le righe `Test Files` e `Tests` dei due progetti, e `found 0 vulnerabilities`;
7. le **divergenze**, come voci candidate `E19`… con l'evidenza, e ciò che non hai potuto misurare;
8. il tempo, e i percorsi dei log del cancello.

Nel messaggio finale al coordinatore: lo stato, l'hash e il percorso del rapporto, in poche righe.
