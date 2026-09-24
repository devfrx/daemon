Sei l'**implementatore del compito 1** — *i token: la tavola copiata, il tema sulla radice, i nomi nuovi* — del piano
`docs/superpowers/plans/2026-09-23-design-system.md`, nel repository `E:\ALL\DEV\MY_REPOS\daemon`. Sei un subagente
fresco: tutto ciò che ti serve è qui e nel brief che questo prompt nomina. Il compito è **codice della GUI** — CSS,
TypeScript, Vue, due dipendenze — più **una** riga del compendio e **una** riga del piano.

**All'avvio verifichi, e se non torna ti fermi e lo riporti:** `git rev-parse --short HEAD` → `d10d9a5`;
`git status --porcelain` → vuoto; `node --version` → `v24.19.0`; `git config --show-origin core.autocrlf` →
`file:.git/config` e `false`. La data da scrivere al posto di ogni `<data>` è **2026-09-24**, sempre la stessa anche se
l'esecuzione passa la mezzanotte.

---

## 1. Che cosa leggi, e che cosa no

La cartella del dispaccio è `.superpowers/sdd/2026-09-23-design-system/` (git-ignorata):

| File | Che cos'è |
|---|---|
| `task-1-brief.md` | **il compito**: la testa del piano (obiettivo, architettura, pila, disegno, **strumenti** con `replace_unique.py`), i *Vincoli globali*, *A che punto è* e *Come si esegue un compito*, l'**errata** (una voce, **E1**), le voci **P-1, P-4, P-5, P-12, P-13**, le decisioni **D1** e **D2**, le voci aperte che il piano sa, **il compito 1 intero**, e dal disegno la sezione **(a)** e i **controlli 1–7** — **copiati parola per parola** da `_extract_brief_1.py`, a `HEAD` = `d10d9a5`. Leggilo **tutto**, a blocchi: pesa ~80 KB |

Poi, **per le sole parti che il compito nomina o che modifichi**, e **prima** di scriverle: i file della lista *Files*
del compito; la riga di `docs/COMPENDIO.md` che comincia con `Lo stile di oggi è un **segnaposto dichiarato**`; la riga
`| **1** |` della tabella della posizione del piano; la tavola `docs/superpowers/specs/2026-09-22-design-system-tavole/token.html`
**solo** fra i tre segnaposti del vincolo 2, se vuoi vedere che cosa copia lo script.

⛔ **Non leggi:** il piano intero (oltre 8 500 righe: ti serve il brief); il disegno intero (la (a) e i controlli 1–7 sono
nel brief); `docs/HANDOFF.md`; la cartella `docs/adr/`; `docs/archivio/`; l'audit. La lettura d'apertura di `CLAUDE.md` —
il compendio per intero — l'ha fatta il coordinatore, e le sue **regole** valgono per te: codice in inglese e documenti
in italiano, nessuna cifra nuova senza il suo comando, i fine-riga conservati per file, mai `sed -i`.

## 2. Il pre-controllo, e la voce d'errata di questa sessione

Il **pre-controllo** del compito 1 è fatto — `6ae9b8b`, sull'altra macchina (`zagor`) — e **non ha trovato difetti**: il
compito è stato rifatto per intero dal testo del piano su una copia pulita, e ogni *Atteso* è tornato. La consegna lo
scrive nel *«Come si riprende»* del piano; non ti serve leggerlo, i numeri utili sono nella §3.

Preparando questo dispaccio, **su questa macchina**, il coordinatore ha trovato **E1** — già scritta nell'errata e
**già applicata** al testo del compito che hai nel brief (`d10d9a5`): il Passo 10 diceva *«CRLF su questa macchina»* di
`gui/src/stores/layout.ts`, ed era la misura di `zagor`. Qui è `i/lf w/lf`. **Nessun Atteso cambia**: `replace_unique.py`
conserva il fine-riga che trova.

## 3. I numeri — rimisurati oggi qui, e quelli del pre-controllo

**Rimisurati il 2026-09-24 su questa macchina, a `6ae9b8b`/`d10d9a5`** (il secondo commit tocca solo il piano):

| Che cosa | Uscita |
|---|---|
| il censimento del Passo 1 | **50** righe: le **dieci** di `tokens.css` e gli **undici** componenti dell'Atteso; i due veli (`Confirm.vue:49`, `Drawer.vue:38`); i due `z-index` (`Drawer.vue:39`, `:44`) — **uguale all'Atteso** |
| `git ls-files --eol` sui file che esistono della lista *Files*, e sugli undici componenti | **`i/lf w/lf` su tutti**, CR **0** |
| il cancello | `GATE GREEN` a `6ae9b8b` (la prima corsa della sessione, a cache fredde, ~10 minuti) e a `d10d9a5` (`real 1m0.724s`); sotto `gui/` **84** prove passate e **una** saltata |
| il pezzo JavaScript di base — `(cd gui && npm run build 2>&1 \| grep -E 'assets/index-.*\.js ')` | `dist/assets/index-DXs5ZA4m.js   663.26 kB │ gzip: 201.23 kB` — ⛔ lo **rimisuri tu** al Passo 1, prima di toccare, perché il Passo 15 lo vuole nel commit accanto a quello dopo |
| il margine del compendio — `wc -c docs/COMPENDIO.md` e `grep -n '^ceiling=' scripts/check-docs.sh` | **89790** e `ceiling=100352` |
| la CI di `6ae9b8b` | `success` su `gate (ubuntu-latest)` e `gate (windows-latest)` |

**Misurati dal pre-controllo su `zagor`**, su una copia pulita col compito rifatto dal testo del piano — ⚠️ sono
**riferimenti**, non Attesi nuovi: tu **misuri**, e un numero diverso si **riporta**, non si insegue:

| Passo | Uscita del pre-controllo |
|---|---|
| 2 | il diff di `package.json` `4 +++-`; la licenza `OFL-1.1` |
| 8 | **6** prove rosse su 28; la scelta sconosciuta **verde** |
| 9 | **9** verdi |
| 13 | undici righe `ok:`, e il censimento pulito |
| 15 | **100** prove passate e **una** saltata; *build* e linter verdi; il pezzo JavaScript `663.93 kB` contro i `663.26 kB` di prima; `npm audit` a 0 |
| 16 | le **otto** violazioni rosse, ciascuna per la ragione scritta — il contrasto nomina `color-text-muted on color-bg: 3.34 < 4.5`, la guardia dice `color-bg-test` nei due temi, i colori `panels/Strip.vue:30` |
| 17 | il testo peggiore del dock a **5,86** nello scuro e **6,22** nel chiaro, `seen` **39**, Geist caricato |

## 4. I fine-riga

Su questa macchina `core.autocrlf` è **`false` in `.git/config`**: i file che il compito tocca e che esistono sono
**`i/lf w/lf`**, CR **0**; i file **nuovi** nascono **LF**. Scrivi con Python `newline=""` (temporaneo più `os.replace`)
o con `replace_unique.py`, che copi dagli *Strumenti* del brief **nello scratchpad**
`C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\223a9934-3feb-4768-87c2-d14a64ff36ab\scratchpad`
— mai nel repository; lì vanno anche `extract_tokens.py` e `rename_tokens.py`. ⛔ **Mai `sed -i`.** Il Passo 16
rimisura tutto: CR **uguale a prima** per ogni file che esisteva, **0** per i nuovi, e il primo dei suoi due comandi
non rende nulla.

## 5. Ciò che da qui non si misura, e come lo fai

- ⚠️ **Il Passo 2 va in rete** (`npm install --save-exact`): il manifesto e il lockfile cambiano **insieme**, e vanno
  nello **stesso** commit (vincolo 7). Il cancello poi fa `npm ci`, che è `--locked`.
- ⚠️ **Il Passo 16 si torna indietro con la COPIA SALVATA, mai con `git checkout`** (vincolo 11): prima della prima
  violazione `git status --porcelain > <scratchpad>/prima.txt` e una copia di ogni file che le otto violazioni toccano;
  dopo **ciascuna** la copia torna e `cmp` lo conferma; alla fine `git status --porcelain | diff <scratchpad>/prima.txt -`
  non rende nulla. Per ogni violazione riporti il **messaggio rosso vero** — la prima riga che nomina la ragione —,
  non «rosso».
- ⚠️ **Il Passo 17 si fa nel browser dell'app**, coi suoi strumenti (`mcp__Claude_Browser__*`; se sono differiti, li
  carichi con `ToolSearch`). Tre trappole **misurate** sul repository:
  1. `navigate` **rifiuta** `localhost`: la pagina si apre con `preview_start` e l'URL `http://localhost:5173`;
     `harnessFake.deliverAll()`, il cambio di tema e lo snippet del contrasto si lanciano con `javascript_tool`.
  2. il server — `(cd gui && npm run dev)` — si lancia **in background**, con l'uscita in un log nello scratchpad. ⛔
     **Non aspettarlo con `grep 'Local:'` sul log**: Vite colora la riga e `Local` e `:` restano separati da un codice
     ANSI (misurato oggi). Aspetta con un ciclo **limitato** su
     `netstat -ano | grep ':5173' | grep LISTENING`, con `python -c "import time; time.sleep(0.5)"` fra un giro e
     l'altro: il `sleep` in primo piano del tool Bash è bloccato. Oggi il server ascolta su `[::1]:5173`.
  3. il server **si spegne per PID**: `netstat -ano | grep ':5173' | grep LISTENING` dà il PID nell'ultima colonna,
     poi `taskkill //F //PID <pid>`. ⛔ **Mai `taskkill //IM node.exe`**, che ucciderebbe anche l'harness. Alla fine,
     lo stesso `netstat` non rende nulla.
  Per ciascun tema riporti `theme`, `seen` e le **tre** righe di `worst`; e la `font-family` calcolata di un testo
  del dock, letta con `getComputedStyle` (è ciò che *«DevTools, Computed»* vuol dire qui).
- ⚠️ **Il cancello dura da uno a dieci minuti** su questa macchina, secondo le cache. Lancialo **da solo**, in
  background, con l'uscita in un log datato nello scratchpad, e aspetta la notifica: una chiamata in primo piano scade.
  **Due cancelli insieme si pestano** su `gui/node_modules` (gotcha #133): mai un `npm` o un `vitest` mentre gira.
- ⚠️ **Il margine del compendio** si misura prima e dopo il Passo 14 (vincolo 13).

## 6. Il contratto

1. **Niente subagenti.** Lavori tu, un passo per volta, **nell'ordine del compito**: le prove prima del codice (Passi
   3–7), il rosso guardato (Passo 8), poi il codice — è il ciclo di `superpowers:test-driven-development`, e il piano lo
   ha già scritto.
2. **Un commit solo**, coi soli file del compito: quelli che il `git add` del Passo 18 nomina —
   `gui/package.json`, `gui/package-lock.json`, `gui/src`, `docs/COMPENDIO.md` e il piano, dove la riga **1** della
   tabella della posizione passa a **Stato** `✅ 2026-09-24` e la colonna **Commit** resta `—` (la scrive il compito 2,
   R1-16). Il messaggio sta in un file nello scratchpad e si passa con `git commit -F <file>`: la **prima riga** è il
   messaggio del Passo 18, alla lettera; poi una riga vuota e le **due** righe del pezzo JavaScript — quella del Passo 1
   e quella del Passo 15 — com'è il Passo 15. ⛔ **Senza co-autore.** ⛔ **Niente `git push`**, niente `--amend`,
   niente rebase: il push è del coordinatore, **dopo la revisione** — il Passo 18 dice `git push`, e questo contratto
   lo sposta, come nei compiti della parte 2.
3. **Prima del commit**, uno alla volta: `bash scripts/gate.sh` → `GATE GREEN` e `bash scripts/check-docs.sh` → `OK`;
   e `git status --porcelain` che nomina **solo** i file del punto 2 — nessun file nato dalle prove.
4. ⛔ **Se il compito dice il falso, ti fermi e lo riporti**: una divergenza è una **voce d'errata candidata** — la
   prossima libera è **E2** — nel rapporto, con l'evidenza: il comando e la sua uscita. **Mai** un aggiustamento
   silenzioso, e ⛔ **mai un valore dei token ritoccato** (vincolo 2, e il Passo 9 lo dice: se una coppia fallisce si
   porta al proprietario). Se la divergenza ti impedisce di chiudere un passo, **non committi** e rendi `BLOCKED`.
5. **Non tocchi** `crates/`, `scripts/`, `.github/`, `Cargo.*`, `gui/schema/`, `docs/adr/`, né altri documenti oltre
   alla riga del compendio e alla riga della posizione.

## 7. Il rapporto

Scrivi `.superpowers/sdd/2026-09-23-design-system/task-1-report.md`, con:

1. lo **stato** — `DONE`, `DONE_WITH_CONCERNS`, `BLOCKED` o `NEEDS_CONTEXT` — e l'**hash** del commit;
2. `git show --stat HEAD`;
3. per **ogni passo**, i comandi lanciati e le uscite **vere**: il censimento del Passo 1 e del Passo 13 per intero;
   al Passo 8, quali prove sono rosse e con quale messaggio;
4. la tabella delle **otto** violazioni del Passo 16, col messaggio rosso vero di ciascuna, i `cmp` e il `diff` finale;
5. i **fine-riga**: le due uscite del Passo 16, per intero;
6. il Passo 17: per ciascun tema `theme`, `seen` e `worst`, e la `font-family`; e che il server è spento;
7. le righe del pezzo JavaScript, prima e dopo; il margine del compendio, prima e dopo;
8. le **divergenze**, come voci candidate `E2`… con l'evidenza, e ciò che non hai potuto misurare;
9. il tempo, e i percorsi dei log del cancello.

Nel messaggio finale al coordinatore: lo stato, l'hash e il percorso del rapporto, in poche righe.
