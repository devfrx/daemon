Sei il **revisore del compito 1** — *i token: la tavola copiata, il tema sulla radice, i nomi nuovi* — del piano
`docs/superpowers/plans/2026-09-23-design-system.md`, nel repository `E:\ALL\DEV\MY_REPOS\daemon` (Windows; il tool
Bash è Git Bash). Sei un subagente fresco e **non hai scritto nulla** di ciò che rivedi. Rivedi **un commit**:

| Commit | Chi | Che cosa |
|---|---|---|
| `95068bb` | l'implementatore | il compito 1: 29 file, sopra `d10d9a5` |

**All'avvio verifichi:** `git rev-parse --short HEAD` → `95068bb`; `git status --porcelain` → vuoto;
`git log --oneline d10d9a5..HEAD` → **una** riga. Se non torna, ti fermi e lo riporti.

⛔ **Non modifichi nulla nell'albero del repository**: niente scritture, niente commit, niente push. Le prove che
chiedono una mutazione le fai su un **clone**: `git clone E:/ALL/DEV/MY_REPOS/daemon C:/Users/Jays/AppData/Local/Temp/rv1`,
poi `git -C C:/Users/Jays/AppData/Local/Temp/rv1 checkout --detach 95068bb` e `(cd .../rv1/gui && npm ci)`. Il clone sta
in `%TEMP%` e **non** nello scratchpad: su questa macchina `LongPathsEnabled` è 0, e i percorsi di `node_modules` sotto lo
scratchpad passerebbero i 260 caratteri. Lo lasci dov'è: lo cancella il coordinatore. Lo scratchpad per i log è
`C:\Users\Jays\AppData\Local\Temp\claude\E--ALL-DEV-MY-REPOS-daemon\223a9934-3feb-4768-87c2-d14a64ff36ab\scratchpad\review1`.
Niente subagenti.

⛔ **Una cosa alla volta**: il cancello e un `npm`/`vitest` sullo **stesso** albero si pestano su `gui/node_modules`
(gotcha #133), e due suite insieme sulla stessa macchina rendono instabili le prove col tempo (P-20, P-21 del piano).
Il cancello gira sull'**albero del repository** (a cache calde dura ~1 minuto: lancialo in background verso un log e
aspetta la notifica); le mutazioni girano **nel clone**; mai due cose insieme.

## 1. Che cosa leggi

Nella cartella `.superpowers/sdd/2026-09-23-design-system/`:

| File | Che cos'è |
|---|---|
| `task-1-brief.md` | **il compito come era dettato**, parola per parola dal piano e dal disegno a `d10d9a5`: la testa, i vincoli globali, l'errata (**E1**), le voci P e D che nomina, il compito 1, la (a) del disegno e i controlli 1–7 |
| `dispatch-task-1.md` | il prompt che l'implementatore ha ricevuto — il **contratto** contro cui lo giudichi |
| `task-1-report.md` | il **rapporto dell'implementatore**: è una dichiarazione, non un fatto |
| `compare_task1.py` | lo script del coordinatore che ricostruisce dal **testo del piano** ogni file dettato e lo confronta byte per byte col commit — sostituisce la copia `t1` del pre-controllo, che esiste solo sull'altra macchina. ⚠️ **È anch'esso una dichiarazione**: lo leggi, e lo provi |

Poi `git show 95068bb` per intero. ⛔ **Non leggi** il piano intero (oltre 8 500 righe), il disegno intero, `docs/HANDOFF.md`,
`docs/adr/`, `docs/archivio/`, l'audit: dove ti serve, `grep -n` e `sed -n` nel punto esatto. Le regole di `CLAUDE.md`
valgono per te: codice in inglese e documenti in italiano, nessuna cifra senza il suo comando, i fine-riga per file.

## 2. Che cosa verifichi

1. **Ogni affermazione misurabile del rapporto si RILANCIA** — il comando accanto, la tua uscita accanto alla sua
   (regola 5 di *«Come si esegue un compito»*). Elenca i comandi rilanciati. Un numero che non torna è un rilievo.
2. **La conformità al dettato.** `python compare_task1.py d10d9a5 95068bb` (dalla radice del repository, con
   `PYTHONIOENCODING=utf-8`). Ma prima **provalo nelle due direzioni**: leggilo e di' se ricostruisce davvero ciò che il
   compito detta — la ricetta del *«Come si riprende»* del piano, i due tagli della tavola, le rinomine, la riga del
   compendio —; poi nel **clone** fai un commit che cambia **un** carattere di **un** file dettato (per esempio un
   commento di `gui/src/tokens/theme.ts`) e lancia lo script contro quel commit, dal clone: deve dire `DIFFERS` su quel
   file **e su nessun altro**, ed uscire 1. I tre file che lo script lascia `CHECK BY HAND` li guardi tu: nel piano
   cambia **solo** la riga **1** della posizione (`✅ 2026-09-24`, colonna Commit `—` per R1-16); `gui/package.json` e
   `gui/package-lock.json` portano **solo** i due caratteri a `5.3.0`, esatti, con licenza `OFL-1.1`, nello **stesso**
   commit (vincolo 7).
3. **Il rosso del Passo 8, riprodotto.** Nel clone, su un ramo da `d10d9a5`, metti **solo** le prove del compito —
   `board.test.ts`, `contrast.test.ts`, `usage.test.ts`, `theme.test.ts` e il `describe` in coda a `stores.test.ts`, presi
   da `95068bb` — e lancia `(cd gui && npx vitest run src/tokens src/stores)`: quante rosse, e per quale ragione ciascuna,
   contro l'Atteso del Passo 8 (e il rapporto: 6 su 28, la scelta sconosciuta verde).
4. **Le otto violazioni del Passo 16**, nel clone a `95068bb`, una alla volta: ciascuna rossa **per la ragione scritta** —
   il messaggio vero, non «rosso» — e tornata indietro, con `git -C <clone> status --porcelain` vuoto alla fine.
5. **Il cancello**, una volta, sull'albero del repository: `GATE GREEN`, le prove di `gui/` (100 passate e una saltata,
   dice il rapporto), la riga del pezzo JavaScript (`663.93 kB`). E `bash scripts/check-docs.sh` → `OK`.
6. **Il Passo 17, nel browser dell'app**, nei due temi: `preview_start` su `http://localhost:5173` (`navigate` rifiuta
   `localhost`), il server `(cd gui && npm run dev)` lanciato in background **dal clone** — mai due server —, atteso con un
   ciclo limitato su `netstat -ano | grep ':5173' | grep LISTENING` e spento **per PID** con `taskkill //F //PID <pid>`,
   mai `//IM node.exe`. `harnessFake.deliverAll()`, poi per ciascun tema lo snippet del Passo 17 con `javascript_tool` —
   il cambio di tema e lo snippet in **due** chiamate, perché col pannello nascosto `requestAnimationFrame` non scatta
   (lo ha misurato l'implementatore) —, e la `font-family` calcolata. ⛔ **E GUARDI**: una schermata per tema. Un verde
   non prova che una cosa si veda: di' che cosa vedi — il fondo, il testo, la fascia, il dock, le linguette e i pulsanti
   della presa grande — e se qualcosa è illeggibile, tagliato o fuori posto.
7. **Il codice, oltre il dettato.** Il codice è dettato dal piano, quindi un difetto del dettato **non si cura**: è una
   **voce d'errata candidata** (la prossima libera è **E2**) col testo corretto proposto. Guarda soprattutto: che il tema
   sia sulla radice **prima** del *mount* (`main.ts`) e che la prima pittura di Vue abbia i colori; che `watchTheme`
   smetta davvero di ascoltare; che `settle` e `chooseTheme` conservino il resto del pacchetto; che `unpack` legga una
   scelta sconosciuta come assente; che nessun commento **dica il falso** dopo questo commit (gotcha #58) — fra gli altri,
   il rapporto nomina il commento di `gui/src/a11y.test.ts` (lo toglie il compito 3, dice il Passo 14) e quello di
   `Drawer.vue` sui gruppi galleggianti *«at 99»* con `--z-overlay` a 200: verificali tu.
8. **I vincoli globali** del brief — soprattutto il **2** (i valori copiati, mai riscritti), il **5** e il **6** (nessun
   colore a mano, nessuna scala fuori dai token), il **9** (fine-riga: `git ls-files --eol` e i CR dei blob), il **12**
   (`git diff --stat d10d9a5..95068bb -- crates/ gui/schema/` vuoto), il **13** (il margine del compendio).
9. **Il contratto** del prompt dell'implementatore: un commit, i file del Passo 18, la riga 1 della posizione, la prima
   riga del messaggio uguale a quella del Passo 18 e poi le **due** righe del pezzo JavaScript, niente push
   (`git status -sb` → `ahead 1`), niente co-autore (`git log -1 --format=%B 95068bb | grep -ci co-authored` → 0).

## 3. Il rapporto

Scrivi `.superpowers/sdd/2026-09-23-design-system/task-1-review.md`:

1. il **verdetto**, in una riga: conforme o no;
2. i **rilievi**, classificati **Critico** (rende falso ciò che il compito consegna, o rompe un Atteso o un criterio),
   **Importante** (una regola del repository violata, o un'affermazione del rapporto non provata), **Minore**, **Nit** —
   ciascuno col file, la riga ritrovata col `grep` sulla frase, l'evidenza (comando e uscita) e la cura proposta **in
   forma di testo**, senza applicarla; un difetto del **dettato** come voce d'errata candidata `E2`…;
3. l'elenco dei **comandi rilanciati**, con le uscite;
4. ciò che **non** hai potuto verificare, e perché.

Nel messaggio finale al coordinatore: il verdetto, il conto dei rilievi per classe, il percorso del rapporto — in poche
righe.
