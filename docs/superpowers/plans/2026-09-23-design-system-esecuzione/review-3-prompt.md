Sei il **revisore del compito 3** — *il kit: gli otto pezzi di base, la mappa delle icone, le regole del linter* — del
piano `docs/superpowers/plans/2026-09-23-design-system.md`, nel repository `C:\Users\zagor\Desktop\harness` (Windows; il
tool Bash è Git Bash: apri ogni chiamata con `cd /c/Users/zagor/Desktop/harness &&`). Sei un subagente fresco e **non hai
scritto nulla** di ciò che rivedi. Rivedi **un commit**:

| Commit | Chi | Che cosa |
|---|---|---|
| `c7b7bcd` | l'implementatore | il compito 3, sopra `ef3e865` |

**All'avvio verifichi:** `git rev-parse --short HEAD` → `c7b7bcd`; `git status --porcelain` → vuoto;
`git log --oneline ef3e865..HEAD` → **una** riga. Se non torna, ti fermi e lo riporti.

⛔ **Non modifichi nulla nell'albero del repository**: niente scritture, niente commit, niente push. Le prove che
chiedono una mutazione le fai su un **clone**: `git clone C:/Users/zagor/Desktop/harness C:/Users/zagor/AppData/Local/Temp/rv3`,
poi `git -C C:/Users/zagor/AppData/Local/Temp/rv3 checkout --detach c7b7bcd` e `(cd .../rv3/gui && npm ci)`. Il clone sta
in `%TEMP%` e **non** nello scratchpad: su questa macchina `LongPathsEnabled` è 0, e i percorsi di `node_modules` sotto lo
scratchpad passerebbero i 260 caratteri. Lo lasci dov'è: lo cancella il coordinatore. Lo scratchpad per i log è
`C:\Users\zagor\AppData\Local\Temp\claude\C--Users-zagor-Desktop-harness\cba88c07-c154-4a08-acdf-71c69edc56c4\scratchpad\review3`
(in Git Bash `/c/Users/zagor/AppData/Local/Temp/claude/C--Users-zagor-Desktop-harness/cba88c07-c154-4a08-acdf-71c69edc56c4/scratchpad/review3`).
Niente subagenti.

⛔ **Una cosa alla volta**: il cancello e un `npm`/`vitest` sullo **stesso** albero si pestano su `gui/node_modules`
(gotcha #133), e due suite insieme sulla stessa macchina rendono instabili le prove col tempo (P-20, P-21 del piano). Su
questa macchina la memoria è stretta — 15,7 GB, e le app del proprietario ne tengono gran parte —: mai due suite, mai due
cancelli, mai un cancello mentre gira una suite nel clone. Il cancello gira sull'**albero del repository**, in background
verso un log nuovo nello scratchpad, e aspetti la notifica; le mutazioni girano **nel clone**. ⛔ Il browser delle prove è
il **Chrome installato**, senza finestra: se Playwright dice che manca, ti fermi e lo riporti — niente
`npx playwright install`, niente download (decisione 22 del disegno).

La macchina, misurata dal coordinatore il 2026-09-25: `core.autocrlf` `true` dal file di sistema (l'albero è `w/crlf` per i
file che git ha scritto, e `w/lf` per `gui/eslint.config.js` e `gui/src/a11y.test.ts`; i file nuovi nascono `w/lf`), Node
v24.19.0, Chrome `153.0.8010.53` con la `154.0.8037.58` già scaricata come `new_chrome.exe` — la versione che Playwright
apre si legge, non si presume. Il cancello d'apertura del coordinatore, a `ef3e865`: `GATE GREEN`, sotto `gui/` il progetto
`jsdom` con 17 file passati e uno saltato, 100 prove passate e una saltata, il progetto `browser` con un file e 5 prove, il
pezzo JavaScript `663.93 kB` (`index-DLsd9Y_U.js`), `found 0 vulnerabilities`; il suo log è `gate-baseline.log` nello
scratchpad padre di `review3`, e **non** è quello dell'implementatore.

## 1. Che cosa leggi

Nella cartella `.superpowers/sdd/2026-09-23-design-system/`:

| File | Che cos'è |
|---|---|
| `task-3-brief.md` | **il compito come era dettato**, parola per parola dal piano e dal disegno a `ef3e865`: la testa con gli *Strumenti*, i vincoli globali, l'errata (**E15**–**E18** sono del compito 3, già applicate al testo), le voci P-2, P-3, P-8, P-9 e P-25, la decisione D6, il compito 3 intero, la (b) del disegno, i controlli 10–13, le decisioni 18–20 e le trappole 1, 5, 10, 14, 16 e 18. Leggilo **tutto**, a blocchi: pesa ~99 KB, 1640 righe |
| `dispatch-task-3.md` | il prompt che l'implementatore ha ricevuto — il **contratto** contro cui lo giudichi |
| `task-3-report.md` | il **rapporto dell'implementatore**: è una dichiarazione, non un fatto |

E nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`: `compare_task3.py`, lo script del
pre-controllo che ricostruisce dal **testo del piano** ogni file dettato e lo confronta col commit. ⚠️ **È anch'esso una
dichiarazione**: lo leggi, e lo provi.

Poi `git show c7b7bcd` — il lockfile non si legge riga per riga: se ne guardano le voci nuove, con le versioni, le licenze e
gli script d'installazione. ⛔ **Non leggi** il piano intero (oltre 8 600 righe), il disegno intero, `docs/HANDOFF.md`,
`docs/adr/`, `docs/archivio/`, l'audit: dove ti serve, `grep -n` e `sed -n` nel punto esatto. Le regole di `CLAUDE.md`
valgono per te: codice in inglese e documenti in italiano, nessuna cifra senza il suo comando, i fine-riga per file.

📌 **Un confronto in più, se ti serve:** la copia del pre-controllo, `%TEMP%\pc3`, ramo `t3`, è il compito rifatto dal testo
del piano di `ee084d7` — per il compito 3 lo stesso di `ef3e865` —, e i log delle sue misure stanno in
`%TEMP%\pc3-tools\logs\`. Non la modifichi.

## 2. Che cosa verifichi

1. **Ogni affermazione misurabile del rapporto si RILANCIA** — il comando accanto, la tua uscita accanto alla sua
   (regola 5 di *«Come si esegue un compito»*). Elenca i comandi rilanciati. Un numero che non torna è un rilievo. Il log
   del cancello che il rapporto cita dev'essere **dell'implementatore**: `ls -la --time-style=full-iso <log>` contro
   `git log -1 --format=%ci c7b7bcd`.
2. **La conformità al dettato.** `python docs/superpowers/plans/2026-09-23-design-system-esecuzione/compare_task3.py ef3e865 c7b7bcd`
   dalla radice del repository, con `PYTHONIOENCODING=utf-8`. Ma prima **provalo nelle due direzioni**: leggilo e di' se
   ricostruisce davvero ciò che il compito detta — la ricetta del *«Come si riprende»* del piano (i file interi, le
   sostituzioni, il blocco dell'aiutante preso dal file), `lucide` esatto in `package.json`, il suo appuntamento nel
   lockfile, le due celle del piano —; poi nel **clone** fai un commit che cambia **un** carattere di **un** file dettato
   (per esempio un commento di `gui/src/components/BaseStatus.vue`) e lancia lo script, dal clone, contro quel commit: deve
   dire `DIFFERS` su quel file **e su nessun altro**, ed uscire 1. Ciò che lo script lascia `CHECK BY HAND` lo guardi tu:
   nel piano cambiano **solo** le righe **2** e **3** della tabella della posizione — la riga 3 a `✅ 2026-09-25` con la
   colonna Commit `—`, la riga 2 con la colonna Commit `` `23b3134`, con la cura `5f32158` ``; il lockfile porta la voce
   nuova che il Passo 1 misura — la licenza, e nessuno script d'installazione — nello **stesso** commit del manifesto
   (vincolo 7); e il `LICENSE` del pacchetto dice ciò che l'Atteso del Passo 1 afferma.
3. **Il rosso del Passo 3, riprodotto.** Nel clone a `c7b7bcd`, togli i pezzi che il Passo 3 non ha ancora — `icons.ts` e
   gli otto `Base*.vue` di `gui/src/components/`, ciascuno salvato prima in una copia —, lascia `kit.test.ts`, e lancia la
   prova come la lancia il Passo 3: il messaggio rosso, contro l'Atteso del Passo 3 e il rapporto. Poi i file tornano dalle
   copie, e `cmp` lo conferma.
4. **Le violazioni del Passo 8**, nel clone a `c7b7bcd`, una alla volta, com'è scritto nel Passo 8 — la prima tabella con
   `npm run lint`, la seconda con `npx vitest run --project jsdom src/components/kit.test.ts` —: ciascuna rossa **per la
   ragione scritta** — il messaggio vero, non «rosso» —, **da sola** (un problema per corsa, una prova rossa per corsa), e
   tornata indietro con la copia salvata (vincolo 11), con `git -C <clone> status --porcelain` vuoto alla fine. La riga
   verde della prima tabella, e il `v-if="$slots.default"` che la seconda dice verde: verdi.
5. **Il cancello intero**, una volta, sull'**albero del repository**: `GATE GREEN`; le righe `Test Files` e `Tests` dei due
   progetti, contro il rapporto e contro la base — che nessuna delle cento prove di prima sia sparita dal `jsdom`, per
   **nome di file** e non solo per conto —; `found 0 vulnerabilities`; la riga del pezzo JavaScript, contro la base. E
   `bash scripts/check-docs.sh` → `OK`. E che il linter legga davvero i `.ts` di `src/`: `npx eslint src --format json`,
   contato per estensione, nel clone.
6. **Il codice, oltre il dettato.** Il codice è dettato dal piano, quindi un difetto del dettato **non si cura**: è una
   **voce d'errata candidata** (la prossima libera la dice il rapporto dell'implementatore: se ne ha usate, la tua viene
   dopo) col testo corretto proposto. Guarda soprattutto le tre lezioni del pre-controllo, scritte nel *«Come si
   riprende»* del piano: **(a)** ogni riga di un pezzo che porta il suo perché in un commento — tolta, quale prova cade?
   (così sono nate E15 ed E16); **(b)** ogni commento che dice *«ogni»* o *«tutti»* su una cartella che il compito fa
   crescere (E17) — in `gui/eslint.config.js`, in `gui/src/a11y.test.ts` dopo che l'aiutante se n'è andato, nei pezzi
   stessi; **(c)** ogni blocco del linter provato sul suo **confine**, non solo sul caso che lo motiva (E18). E poi: che le
   **interfacce** che il compito produce per i compiti 4–8 — la lista *Interfaces* del compito: props, slot, eventi,
   `violations` — siano quelle del codice; i vincoli **4** (nessuna scritta nei pezzi di base), **5** (nessun colore a
   mano) e **6** (nessun `var(--ref-` fuori da `gui/src/tokens/`); che nessun commento **dica il falso** dopo questo commit
   (gotcha #58).
7. **I vincoli globali** del brief — soprattutto il **7** (le dipendenze in due passi, `npm ci` nel cancello), l'**8**
   (versioni appuntate ed esatte: `lucide` 1.47.0; `reka-ui` resta 2.10.4), il **9** (fine-riga: `git ls-files --eol` e i
   CR, prima e dopo, per ogni file toccato; i file nuovi LF), l'**11** (le due direzioni, la guardia), il **12**
   (`git diff --stat ef3e865..c7b7bcd -- crates/ gui/schema/` vuoto).
8. **Il contratto** del prompt dell'implementatore: un commit, i soli file del suo punto 2, le due celle della posizione,
   il messaggio che comincia con `design-system(compito 3): `, niente push (`git status -sb` → `ahead 1`), niente
   co-autore (`git log -1 --format=%B c7b7bcd | grep -ci co-authored` → 0).

## 3. Il rapporto

Scrivi `.superpowers/sdd/2026-09-23-design-system/task-3-review.md`:

1. il **verdetto**, in una riga: conforme o no;
2. i **rilievi**, classificati **Critico** (rende falso ciò che il compito consegna, o rompe un Atteso o un criterio),
   **Importante** (una regola del repository violata, o un'affermazione del rapporto non provata), **Minore**, **Nit** —
   ciascuno col file, la riga ritrovata col `grep` sulla frase, l'evidenza (comando e uscita) e la cura proposta **in
   forma di testo**, senza applicarla; un difetto del **dettato** come voce d'errata candidata;
3. l'elenco dei **comandi rilanciati**, con le uscite;
4. ciò che **non** hai potuto verificare, e perché.

Nel messaggio finale al coordinatore: il verdetto, il conto dei rilievi per classe, il percorso del rapporto — in poche
righe.

---

> **La ripresa.** Il revisore si è fermato a metà verso le 14:05 del 2026-09-25, quando il processo di Claude Code si è
> chiuso; il coordinatore l'ha ripreso alle 14:12 con `SendMessage`, col messaggio qui sotto, parola per parola. ⚠️ Una
> sua frase era falsa: *«nessun log del cancello intero»* — `gate-review-c7b7bcd.log`, delle 13:32, c'era, e un
> `ls | tail -20` del coordinatore l'aveva tagliato; il revisore l'ha trovato e non ha rilanciato il cancello.

Riprendi la revisione del compito 3: il processo di Claude Code si è chiuso verso le 14:05 e ti ha interrotto a metà.

Lo stato adesso, misurato dal coordinatore alle 14:12: l'albero del repository è pulito, a `c7b7bcd`, `ahead 1`; il clone `C:/Users/zagor/AppData/Local/Temp/rv3` è a `c7b7bcd` e `git status --porcelain` è vuoto; nessun processo `node.exe` o `chrome.exe` rimasto acceso; nello scratchpad `review3` i tuoi ultimi file sono `step9-npm-test.log` (14:03), `tsc-nodirective.log` e `step6-build.log`; nessun log del cancello intero, e `task-3-review.md` non esiste ancora.

Continua da dove eri, secondo il mandato in `.superpowers/sdd/2026-09-23-design-system/review-3-prompt.md`: prima ricontrolla che il clone sia com'era prima delle tue mutazioni (se un file del clone è cambiato, torna dalla copia salvata), poi fai ciò che manca — fra l'altro, se non l'hai fatto, la prova di `compare_task3.py` nelle due direzioni nel clone, il cancello intero sull'albero del repository (da solo, in background, verso un log nuovo), `check-docs.sh` — e scrivi il rapporto `task-3-review.md`. Nessun altro processo usa il cancello o `gui/node_modules`.

Alla fine rispondi in poche righe: il verdetto, il conto dei rilievi per classe e il percorso del rapporto.
