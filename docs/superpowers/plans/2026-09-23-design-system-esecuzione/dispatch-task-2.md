> ⚠️ **Per il coordinatore, prima di dispacciare** — questo file è il **modello**, e viaggia con git. Il prompt che parte
> si scrive nella cartella di lavoro `.superpowers/sdd/2026-09-23-design-system/`, ignorata, **senza** questo riquadro e
> coi campi fra `<…>` riempiti: `<repo>`, `<HEAD>` — l'ultimo commit di `main` —, `<data>`, `<scratchpad>`, e i valori
> della macchina del §0 misurati, non copiati. Il brief si genera **prima**, dalla radice del repository, con
> `python docs/superpowers/plans/2026-09-23-design-system-esecuzione/_extract_brief_2.py`, e deve dire *«piano e
> disegno coincidono con `HEAD`»*. Alla chiusura del compito il prompt spedito, il rapporto, il prompt del revisore e la
> revisione si copiano nella cartella tracciata e si committano: il punto 8 di *«Come si esegue un compito»*.

Sei l'**implementatore del compito 2** — *il browser dei test: due progetti, Chrome installato, le prime prove vere* — del
piano `docs/superpowers/plans/2026-09-23-design-system.md`, nel repository `<repo>`. Sei un subagente
fresco: tutto ciò che ti serve è qui e nel brief che questo prompt nomina. Il compito è **codice della GUI** — la
configurazione di `vitest`, due file TypeScript, due dipendenze di sviluppo —, **il passo delle prove** di
`scripts/gate-gui.sh`, **un richiamo datato** nel disegno del sotto-progetto 2 e **due celle** del piano.

**All'avvio verifichi, e se non torna ti fermi e lo riporti:** `git rev-parse --short HEAD` → `<HEAD>`;
`git status --porcelain` → vuoto; `node --version` → una versione che `gui/package.json` accetta (`engines`);
`git config --show-origin --get-all core.autocrlf` → il valore di questa macchina, nel §0; Google Chrome stabile
installato — `ls "/c/Program Files/Google/Chrome/Application/" "$LOCALAPPDATA/Google/Chrome/Application/" 2>/dev/null`
rende almeno una cartella di versione. La data da scrivere al posto di ogni `<data>` è **<data>**, sempre la stessa anche
se l'esecuzione passa la mezzanotte.

## 0. La macchina

Il repository vive in **due cloni**, e ciò che cambia fra i due si **misura**, non si copia da un'etichetta: voce **E72**
del piano della parte 2, ed **E1** di questo piano.

| | macchina `Jays` | macchina `zagor` |
|---|---|---|
| il repository | `E:\ALL\DEV\MY_REPOS\daemon` | `C:\Users\zagor\Desktop\harness` |
| `core.autocrlf` | `false` in `.git/config`, quindi l'albero è `w/lf` | `true` dal file di sistema, quindi l'albero è `w/crlf` |
| Google Chrome | 154, in `C:\Program Files\Google\Chrome\Application\` | 153, alla consegna del 2026-09-24 |
| Node | v24.19.0 | da misurare |

---

## 1. Che cosa leggi, e che cosa no

La cartella di lavoro è `.superpowers/sdd/2026-09-23-design-system/`, git-ignorata; il brief ci nasce da
`_extract_brief_2.py`, nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`:

| File | Che cos'è |
|---|---|
| `task-2-brief.md` | **il compito**: la testa del piano (obiettivo, architettura, pila, disegno, **strumenti** con `replace_unique.py`), i *Vincoli globali*, *A che punto è* e *Come si esegue un compito*, l'**errata** — **E10**, **E11**, **E12** ed **E13** sono del compito 2 —, le voci **P-12** e **P-21**, le voci aperte che il piano sa, **il compito 2 intero**, e dal disegno la sezione **(f)**, i controlli **8**, **9** e **20** e le trappole **1**, **2**, **6**, **11** e **15** — **copiati parola per parola** da `_extract_brief_2.py`, a `HEAD` = `<HEAD>`. Leggilo **tutto**, a blocchi: pesa ~62 KB |

Poi, **per le sole parti che il compito nomina o che modifichi**, e **prima** di scriverle: i file della lista *Files* del
compito; in `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` **la sola riga** che comincia con
``| `scripts/gate-gui.sh` |``; le righe `| **1** |` e `| **2** |` della tabella della posizione del piano.

⛔ **Non leggi:** il piano intero (oltre 8 500 righe: ti serve il brief); il disegno intero; `docs/HANDOFF.md`; la cartella
`docs/adr/`; `docs/archivio/`; l'audit; la copia del pre-controllo, se questa macchina ce l'ha — `%TEMP%\pc2` sulla
macchina `Jays` —, che è il confronto della revisione e non il tuo modello. La lettura d'apertura di `CLAUDE.md` — il compendio per intero — l'ha fatta il coordinatore, e le
sue **regole** valgono per te: codice in inglese e documenti in italiano, nessuna cifra nuova senza il suo comando, i
fine-riga conservati per file, mai `sed -i`.

## 2. Il pre-controllo, e le sue quattro voci

Il **pre-controllo** del compito 2 — `ebfc255`, sulla macchina `Jays` — ha rifatto il compito per intero dal testo del piano
su una copia pulita, e ogni *Atteso* è tornato. Ha trovato **quattro difetti**, già scritti nell'errata e **già applicati**
al testo del compito che hai nel brief:

| Voce | Che cosa cambia per te |
|---|---|
| **E10**, del proprietario | il cancello lancia i due progetti **uno alla volta** — `npm test -- --project jsdom`, poi `npm test -- --project browser` —, perché in una corsa sola un progetto senza file è **verde**. Cambiano il commento del Passo 3, il Passo 6 — che scrive anche il **richiamo datato** nella §8 del disegno del 2 — e il Passo 5, che prende la violazione del progetto vuoto |
| **E11** | il Passo 5 prende la violazione che toglie `:focus-visible` da `base.css` |
| **E12** | l'Atteso della riga del tema, al Passo 5, nomina anche la prova rossa sotto jsdom |
| **E13** | il file usa-e-getta del Passo 5 si nomina — la riga *«l'attesa di 5 s»* —, non si conta |

## 3. I numeri — misurati dal pre-controllo, sulla macchina `Jays`

**Il cancello d'apertura del pre-controllo**, a `b66aee8`: `GATE GREEN`, sotto `gui/` **100** prove passate e **una** saltata; il pezzo
JavaScript `663.93 kB`; la CI verde sui due sistemi. ⛔ Li **rimisuri tu** prima di toccare: sono la base del Passo 4.

**Misurati dal pre-controllo**, sulla copia col compito rifatto dal testo del piano — ⚠️ sono **riferimenti**, non Attesi
nuovi: tu **misuri**, e un numero diverso si **riporta**, non si insegue:

| Passo | Uscita del pre-controllo |
|---|---|
| 1 | undici pacchetti nuovi — `@blazediff/core`, `@polka/url`, `@vitest/browser`, `@vitest/browser-playwright`, `mrmime`, `playwright`, `playwright-core`, `pngjs`, `sirv`, `totalist`, `ws` —, MIT e Apache-2.0, nessuno con script d'installazione; `npm view … scripts.install scripts.postinstall` non stampa nulla; `found 0 vulnerabilities`. ⚠️ L'avviso `npm warn allow-scripts` su `vue-demi@0.14.10` c'era già, anche nel cancello: non è del compito |
| 2 | rosso: `vitest/browser can be imported only inside the Browser Mode. Your test is running in forks pool.` |
| 4 | il progetto `browser`, **5** prove verdi; `npm test`, **18** file passati e uno saltato, **105** prove passate e una saltata; `npm run build` verde, il pezzo JavaScript `663.93 kB`, com'era; i due progetti, `5 \|browser (chromium)\|` e `101 \|jsdom\|` |
| 5 | le **dieci** violazioni rosse per la ragione scritta: `Unsupported chromium channel "chrome-that-does-not-exist"`; il messaggio del Browser Mode; `No test files found, exiting with code 1` per ciascun progetto vuoto, mentre `npm test` esce 0; `expected 3 to be greater than or equal to 4`; `--duration-fast: expected '110ms' to be '0ms'` e `board.test.ts`; la guardia dell'alto contrasto, `expected false to be true`; il contorno senza `:focus-visible`, `expected false to be true` alla prova `ring(button)` e `board.test.ts`; `expected 0 to be greater than 0.5` al primo confronto col ripiego, e `board.test.ts`; il tema a **5057 ms**, `expected 'light' to be 'dark'`, e `theme.test.ts` sotto jsdom; l'attesa verde con la riga `expect:`, rossa senza a **1057 ms** |
| 6 | il passo web, `bash scripts/gate-gui.sh`, verde: `--project jsdom` 17 file passati e uno saltato, 100 prove e una saltata; `--project browser` 1 file, 5 prove; `found 0 vulnerabilities` |

## 4. I fine-riga

⛔ **Nessuna etichetta di forma: si misura** (E72). **Prima** di toccare, sui file della lista *Files* che esistono,
`git ls-files --eol <file>` e `tr -cd '\r' < <file> | wc -c`: sono le colonne di **questa** macchina, e dipendono dal
suo `core.autocrlf` (§0). **Dopo**: CR **uguale a prima** e colonna `w/…` uguale per ogni file che esisteva; **0** per i
file **nuovi**, che nascono **LF**. Scrivi con Python `newline=""` (temporaneo più `os.replace`) o con
`replace_unique.py`, che conserva il fine-riga del file che trova e che copi dagli *Strumenti* del brief **nello
scratchpad** `<scratchpad>` — mai nel repository. ⛔ **Mai `sed -i`.**

## 5. Ciò che da qui non si misura, e come lo fai

- ⚠️ **Il Passo 1 va in rete** (`npm install --save-exact`): il manifesto e il lockfile cambiano **insieme**, e vanno
  nello **stesso** commit (vincolo 7). Il cancello poi fa `npm ci`, che è `--locked`.
- ⚠️ **Il Passo 5 si torna indietro con la COPIA SALVATA, mai con `git checkout`** (vincolo 11): prima della prima
  violazione `git status --porcelain > <scratchpad>/prima.txt` e una copia di ogni file che le violazioni toccano —
  `gui/vite.config.ts`, `gui/src/tokens/index.ts`, `gui/src/tokens/base.css`, `gui/src/tokens/theme.ts`,
  `gui/src/tokens/tokens.browser.test.ts`; dopo **ciascuna** la copia torna e `cmp` lo conferma; il file usa-e-getta
  `gui/src/late.browser.test.ts` si **cancella**; alla fine `git status --porcelain | diff <scratchpad>/prima.txt -` non
  rende nulla. Per ogni violazione riporti il **messaggio rosso vero** — la prima riga che nomina la ragione —, non
  «rosso».
- ⚠️ **Il browser è senza finestra** (`headless: true`). ⛔ Se Playwright dice che Chrome manca, **ti fermi e lo riporti**:
  niente `npx playwright install`, niente download (decisione 22 del disegno).
- ⚠️ **Il cancello dura da uno a dieci minuti**, secondo le cache. Lancialo **da solo**, in background, con l'uscita in un
  log datato nello scratchpad, e aspetta la notifica: una chiamata in primo piano scade. **Due cancelli insieme si
  pestano** su `gui/node_modules` (gotcha #133): mai un `npm` o un `vitest` mentre gira.
- ⚠️ **La CI** — la prima corsa col browser su `ubuntu-latest` e `windows-latest` — la legge il coordinatore dopo il push,
  che è suo.

## 6. Il contratto

1. **Niente subagenti.** Lavori tu, un passo per volta, **nell'ordine del compito**: la prova prima della configurazione
   (Passo 2, il rosso guardato), poi la configurazione — è il ciclo di `superpowers:test-driven-development`, e il piano
   lo ha già scritto.
2. **Un commit solo**, coi soli file del compito: `gui/package.json`, `gui/package-lock.json`, `gui/vite.config.ts`,
   `gui/src/browser.d.ts`, `gui/src/tokens/tokens.browser.test.ts`, `scripts/gate-gui.sh`,
   `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` e il piano, dove la riga **2** della tabella
   della posizione passa a **Stato** `✅ <data>` e nella riga **1** la colonna **Commit** diventa
   `` `95068bb`, con le cure `69d10fa` ed `e2cd7df` `` (R1-16); la colonna **Commit** della riga **2** resta `—`, la
   scrive il compito 3. Il messaggio sta in un file nello scratchpad e si passa con `git commit -F <file>`, e comincia
   con `design-system(compito 2): ` (vincolo 15). ⛔ **Senza co-autore.** ⛔ **Niente `git push`**, niente `--amend`,
   niente rebase: il push è del coordinatore, **dopo la revisione** — il Passo 7 dice `git push`, e questo contratto lo
   sposta, come nel compito 1.
3. **Prima del commit**, uno alla volta: `bash scripts/gate.sh` → `GATE GREEN` e `bash scripts/check-docs.sh` → `OK`;
   e `git status --porcelain` che nomina **solo** i file del punto 2 — nessun file nato dalle prove.
4. ⛔ **Se il compito dice il falso, ti fermi e lo riporti**: una divergenza è una **voce d'errata candidata** — la
   prossima libera è **E14** — nel rapporto, con l'evidenza: il comando e la sua uscita. **Mai** un aggiustamento
   silenzioso. Se la divergenza ti impedisce di chiudere un passo, **non committi** e rendi `BLOCKED`.
5. **Non tocchi** `crates/`, `.github/`, `Cargo.*`, `gui/schema/`, `docs/adr/`, né altri documenti oltre alle due celle
   del piano e al richiamo del Passo 6.

## 7. Il rapporto

Scrivi `.superpowers/sdd/2026-09-23-design-system/task-2-report.md`, con:

1. lo **stato** — `DONE`, `DONE_WITH_CONCERNS`, `BLOCKED` o `NEEDS_CONTEXT` — e l'**hash** del commit;
2. `git show --stat HEAD`;
3. per **ogni passo**, i comandi lanciati e le uscite **vere**: al Passo 1 `npm ls` e il comando `npm view`; al Passo 2
   il messaggio rosso; al Passo 4 i conti dei due progetti;
4. la tabella delle **dieci** violazioni del Passo 5, col messaggio rosso vero di ciascuna, i `cmp` e il `diff` finale;
5. i **fine-riga**, per ogni file toccato: prima e dopo, le due colonne;
6. il passo web: le righe `Test Files` e `Tests` dei due progetti, e `found 0 vulnerabilities`;
7. le **divergenze**, come voci candidate `E14`… con l'evidenza, e ciò che non hai potuto misurare;
8. il tempo, e i percorsi dei log del cancello.

Nel messaggio finale al coordinatore: lo stato, l'hash e il percorso del rapporto, in poche righe.
