Sei il **ri-revisore del compito 1** — *ADR-0040, e ciò che il cancello pretende con lui* — del piano
`docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md`, nel repository `C:\EVERYTHING\DEV\MY_REPOS\daemon`
(Windows; il tool Bash è Git Bash: apri ogni chiamata con `cd /c/EVERYTHING/DEV/MY_REPOS/daemon &&`). Sei un subagente
fresco e **non hai scritto nulla** di ciò che rivedi. La revisione è **delimitata**: rivedi **la cura** del coordinatore,
un commit solo, contro i rilievi della revisione.

| Commit | Chi | Che cosa |
|---|---|---|
| `025dc0f` | l'implementatore | il compito 1, sopra `8bb8440` — già rivisto: **non** lo riesamini |
| `1a8c237` | il coordinatore | la cura della revisione e la chiusura della sessione, sopra `025dc0f` — **questo** rivedi |

**All'avvio verifichi:** `git rev-parse --short HEAD` → `1a8c237`; `git status --porcelain` → vuoto;
`git log --oneline 025dc0f..HEAD` → **una** riga; `git status -sb` → `ahead 2`. Se non torna, ti fermi e lo riporti.

⛔ **Non modifichi nulla nell'albero del repository**: niente scritture, niente commit, niente push. Le prove che chiedono di
scrivere le fai in copie nella tua cartella, `C:/Users/zagor/AppData/Local/Temp/claude/C--EVERYTHING-DEV-MY-REPOS-daemon/5b9699a7-2ab7-410d-849d-c39261487f06/scratchpad/rereview1`
— in Git Bash `/c/Users/zagor/AppData/Local/Temp/claude/C--EVERYTHING-DEV-MY-REPOS-daemon/5b9699a7-2ab7-410d-849d-c39261487f06/scratchpad/rereview1` —,
che crei tu. Niente subagenti. Il cancello **non** lo lanci: l'ha girato il coordinatore prima del commit, e il commit porta
solo documenti; `bash scripts/check-docs.sh` sì. Da Python, Git Bash si chiama col percorso intero: `bash` da solo prende
quella di WSL.

## 1. Che cosa leggi

Nella cartella `.superpowers/sdd/2026-09-29-knowledge-base-revisione-documenti/`:

| File | Che cos'è |
|---|---|
| `task-1-review.md` | **la revisione**: leggi la sezione *«2. I rilievi»* per intero — R1…R11 — e la **5.6**, dove giudica le voci ER-11 ed ER-12 del rapporto. Sono i rilievi contro cui misuri la cura |
| `review-025dc0f..1a8c237.diff` | il **pacchetto** della cura: il commit, lo `--stat` e il diff intero con dieci righe di contesto |
| `task-1-brief.md` | il compito come era dettato: ti serve per il **vincolo 13** — il numeratore dei compiti vive nella sola tabella della posizione —, per gli attrezzi e per la sezione *«Come si esegue un compito»*. Non rileggerlo tutto: cerca ciò che ti serve |

⛔ **Non leggi** il piano intero, il disegno intero, `docs/HANDOFF.md`, l'archivio oltre le code toccate, l'audit, la spec:
`grep -n` e `sed -n` nel punto esatto. Le regole di `CLAUDE.md` valgono per te: documenti in italiano, nessuna cifra misurata
senza il suo comando, un puntatore in una casa sola, i fine-riga per file.

## 2. Che cosa verifichi

1. **Rilievo per rilievo, da R1 a R8, ed ER-11, ER-12: `RISOLTO` o `NON RISOLTO`**, con l'evidenza — il comando e la sua
   uscita. Un rilievo che il coordinatore ha **registrato** invece di curare — R4, la metà di R5 su `HANDOFF.md`, R7, R8 — è
   risolto se la registrazione dice il vero, sta dove il piano tiene le voci aperte e nomina un chiusore. R9, R10, R11 sono
   rimandati per decisione del coordinatore, scritta nel registro: dillo, e di' se la decisione ti pare sbagliata.
2. **Le prove del coordinatore, rilanciate** — elenca i comandi:
   - la consegna archiviata in `docs/archivio/consegna-piano-knowledge-base-revisione-documenti.md` è **parola per parola**
     il *«Come si riprende»* del piano a `025dc0f`, diversa solo sulle righe coi link riscritti per `docs/archivio/`;
   - il puntatore e la riga della data archiviati in coda a `docs/archivio/stato-storico.md` sono **parola per parola** quelli
     del compendio a `025dc0f`, coi link riscritti;
   - il `--check` dei blocchi che restano — E2…E6, POS2…POS6 —, con gli attrezzi estratti dal brief in una copia tua: tutti
     `checked`; e **POS6 applicato a una copia del piano**: il capoverso in testa alla posizione sostituito **intero**, senza
     righe orfane;
   - `tables.awk` del brief sui file toccati, contro `025dc0f`; i CR uguali alle righe sui file `w/crlf`, zero sui due `w/lf`;
     il margine del compendio, comando C del brief; `bash scripts/check-docs.sh` → `OK`;
   - le copie nella cartella del dispaccio, `docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti-esecuzione/`
     — `task-1-dispatch.md`, `task-1-report.md`, `review-1-prompt.md`, `task-1-review.md` — sono identiche ai file omonimi
     della cartella di lavoro: `cmp`.
3. **La cura, che non rompa altro — nel solo diff della cura:**
   - **(a)** ogni frase nuova **dice il vero** contro git, contro la tabella della posizione e contro i rilievi — soprattutto
     il *«Come si riprende»* nuovo del piano: lo stato, i commit della sessione, i costi, le decisioni, le lezioni —; e
     nessuna frase nuova rimette il **numero di un compito** fuori dalla tabella della posizione (vincolo 13, gotcha #68);
   - **(b)** le voci ER-11…ER-18 dicono il vero, ciascuna contro il suo rilievo e contro il file che nomina;
   - **(c)** ADR-0040: tolta **solo** la coda del seguito, e il recinto del compito 1 nel piano è ora **identico** al file —
     il testo del recinto col `<data>` sostituito dalla data del file —; nessun'altra riga di un ADR cambia;
   - **(d)** la §7 del compendio: la frase nuova è coerente con la riga *«ADR superato in parte»* della §13 e con la riga
     *«ADR append-only»* di `CLAUDE.md`;
   - **(e)** la consegna del metodo, `docs/superpowers/specs/2026-09-30-metodo-decision-map-design.md`: le due frasi toccate
     dicono il vero, e nient'altro è cambiato;
   - **(f)** ogni **cifra nuova** nelle righe aggiunte porta accanto il comando, o vive in una riga datata.
4. **Il contratto:** il commit della cura comincia con `knowledge-base-revisione(compito 1): `, non ha co-autore —
   `git log -1 --format=%B 1a8c237 | grep -ci co-authored` → `0` —, e nessun file di `crates/`, `gui/`, `scripts/`, `.github/`,
   `Cargo.*`.

Un difetto nuovo che trovi **fuori** dal diff della cura va in una riga a parte, *«fuori perimetro»*: non allunga questa
revisione.

## 3. Il rapporto

Scrivi `.superpowers/sdd/2026-09-29-knowledge-base-revisione-documenti/task-1-re-review.md`:

1. il **verdetto**, in una riga: tutti i rilievi risolti o no, e se la cura rompe qualcosa;
2. la tabella **rilievo → `RISOLTO` / `NON RISOLTO`**, con l'evidenza;
3. i **rilievi nuovi** nel diff della cura, classificati **Critico**, **Importante**, **Minore**, **Nit** — ciascuno col
   file, la riga ritrovata col `grep` sulla frase, l'evidenza e la cura proposta **in forma di testo**, senza applicarla;
4. l'elenco dei **comandi rilanciati**, con le uscite;
5. ciò che non hai potuto verificare, e perché; e le righe *«fuori perimetro»*.

Nel messaggio finale al coordinatore: il verdetto, il conto dei rilievi nuovi per classe, il percorso del rapporto — in
poche righe.
