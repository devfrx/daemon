Sei il **revisore del compito 1** — *ADR-0040, e ciò che il cancello pretende con lui* — del piano
`docs/superpowers/plans/2026-09-29-knowledge-base-revisione-documenti.md`, nel repository `C:\EVERYTHING\DEV\MY_REPOS\daemon`
(Windows; il tool Bash è Git Bash: apri ogni chiamata con `cd /c/EVERYTHING/DEV/MY_REPOS/daemon &&`). Sei un subagente
fresco e **non hai scritto nulla** di ciò che rivedi. Rivedi **un commit**:

| Commit | Chi | Che cosa |
|---|---|---|
| `025dc0f` | l'implementatore | il compito 1, sopra `8bb8440` |

**All'avvio verifichi:** `git rev-parse --short HEAD` → `025dc0f`; `git status --porcelain` → vuoto;
`git log --oneline 8bb8440..HEAD` → **una** riga. Se non torna, ti fermi e lo riporti.

⛔ **Non modifichi nulla nell'albero del repository**: niente scritture, niente commit, niente push. Le prove che chiedono
di scrivere le fai su un **clone**: `git clone C:/EVERYTHING/DEV/MY_REPOS/daemon C:/Users/zagor/AppData/Local/Temp/rv1`,
poi `git -C C:/Users/zagor/AppData/Local/Temp/rv1 checkout --detach 8bb8440`. Lo lasci dov'è: lo cancella il coordinatore.
La tua cartella per i log e i file è `C:/Users/zagor/AppData/Local/Temp/claude/C--EVERYTHING-DEV-MY-REPOS-daemon/5b9699a7-2ab7-410d-849d-c39261487f06/scratchpad/review1`
— in Git Bash `/c/Users/zagor/AppData/Local/Temp/claude/C--EVERYTHING-DEV-MY-REPOS-daemon/5b9699a7-2ab7-410d-849d-c39261487f06/scratchpad/review1` —;
la crei tu. Niente subagenti.

⛔ **Il cancello, uno alla volta**: gira sull'**albero del repository**, con `run_in_background` del tool Bash e **senza
`&`**, verso un log nuovo nella tua cartella — `bash scripts/gate.sh > "<log>" 2>&1; echo "EXIT=$?" >> "<log>"` —, e aspetti
la notifica. Mentre gira, nessun altro comando che tocchi `gui/` (gotcha #133). Nel clone il cancello **non** serve.

La macchina, misurata dal coordinatore il 2026-09-30: `jays`; `core.autocrlf` `true` da `C:/Program Files/Git/etc/gitconfig`,
quindi i documenti del compito sono `i/lf w/crlf`, coi CR uguali alle righe; il clone eredita la stessa configurazione. Il
cancello del coordinatore a `8bb8440`: `GATE GREEN`.

## 1. Che cosa leggi

Nella cartella `.superpowers/sdd/2026-09-29-knowledge-base-revisione-documenti/`:

| File | Che cos'è |
|---|---|
| `task-1-brief.md` | **il compito come era dettato**, parola per parola dal piano e dal disegno a `8bb8440`: la testa del piano, gli attrezzi, i vincoli globali, *«Come si esegue un compito»*, l'**errata per intero** — **ER-9** ed **ER-10** sono del pre-dispaccio del coordinatore, del 2026-09-30 —, le voci P e D che il compito nomina o usa, le voci aperte che il piano sa, la **Definizione di «fatto»** con le trappole, il **compito 1 intero**, e dal disegno la testa della sezione 3, la riga di ADR-0022 della 3.1, la 3.2, la 3.3, la 3.4 e le due domande della 6.1. Leggilo **tutto**, a blocchi |
| `task-1-dispatch.md` | il prompt che l'implementatore ha ricevuto — il **contratto** contro cui lo giudichi: fa i Passi 1–5, ma **non** fa il push, che è del coordinatore dopo di te |
| `task-1-report.md` | il **rapporto dell'implementatore**: è una dichiarazione, non un fatto. Porta due voci d'errata candidate, **ER-11** ed **ER-12**, sulle parole del piano: giudicale come ogni altra affermazione |
| `review-8bb8440..025dc0f.diff` | il **pacchetto**: la lista dei commit, lo `--stat` e il diff intero con dieci righe di contesto, in un file solo |

Poi `git show --stat 025dc0f` e il diff, file per file. **Leggi per intero** i tre fratelli della riga di ADR-0022 nella 3.1
del disegno — `docs/adr/0018-ritenzione-a-livelli-del-giornale.md`, `docs/adr/0023-*.md`,
`docs/adr/0024-checkpoint-del-filesystem-ad-ambiti-dichiarati.md` —, ADR-0022 e ADR-0040 come sono a `025dc0f`. ⛔ **Non
leggi** il piano intero, il disegno intero, `docs/HANDOFF.md` oltre le righe toccate, l'archivio oltre la coda di
`docs/archivio/stato-storico.md`, l'audit, la spec: dove ti serve, `grep -n` e `sed -n` nel punto esatto. Le regole di
`CLAUDE.md` valgono per te: documenti in italiano, nessuna cifra misurata senza il suo comando, un puntatore in una casa
sola, i fine-riga per file.

## 2. Che cosa verifichi

1. **Ogni affermazione misurabile del rapporto si RILANCIA** — il comando accanto, la tua uscita accanto alla sua: è la
   regola 5 di *«Come si esegue un compito»*. Elenca i comandi rilanciati. Un numero che non torna è un rilievo. Il log del
   cancello che il rapporto cita dev'essere **dell'implementatore**: `ls -la --time-style=full-iso <log>` contro
   `git log -1 --format=%ci 025dc0f`.
2. **La conformità al dettato, nel clone.** Da `8bb8440`, rifai **tu** i Passi 2 e 3 dal brief, con `D=2026-09-30`:
   `extract.py` copiato a mano dal brief nella tua cartella e confrontato col suo recinto del piano, il `sed` di ADR-0040,
   `archive_head.py`, il blocco E1, il blocco POS1. Poi, nel clone, `git diff --stat 025dc0f` deve essere **vuoto**: i dieci
   file dell'implementatore sono quelli che il piano detta. **Provalo nell'altra direzione:** un carattere cambiato in una
   copia di uno dei dieci file nel clone, e lo stesso comando nomina **quel** file e nessun altro; poi il clone torna com'era.
3. **Le prove del Passo 4 e la sonda di ER-10**, rilanciate sull'albero del repository: la sonda di ER-10 con `HEAD` al
   posto di `8bb8440` non vale più — rilanciala con `git show 8bb8440:docs/COMPENDIO.md`, e deve rendere `SAME`. E il
   **criterio di chiusura** del compito 1, riga per riga.
4. **Il merito, riletto — il cuore della revisione** (regola 5 di *«Come si esegue un compito»*, gotcha #59):
   - **(a)** ADR-0040 contro la **3.2**, punto per punto — i sei punti della *Decision*, le *Negative (accettate)*, le
     alternative, il seguito —, e contro la **3.3** — la riga `Modifica:` in testa, lo stato `Accepted`. Ciò che il
     *Context* afferma si ritrova: la frase di `crates/daemon/src/main.rs`, e le fonti dello stato dell'arte nella sezione
     *«La revisione della knowledge base — le fonti del disegno»* di `docs/riferimenti.md`. Ogni link porta a un file vero;
   - **(b)** il rimando nuovo di ADR-0022 contro **ADR-0018, ADR-0023 e ADR-0024**, i fratelli della sua riga nella 3.1, e
     contro i **due rimandi** che ADR-0022 porta già — il 2026-09-08 in testa e il 2026-08-07 in fondo —: niente si
     contraddice? Ciò che il rimando dice delle tre righe modificate è ciò che dice ADR-0040? **Nessuna riga preesistente**
     di ADR-0022 è cambiata;
   - **(c)** il compendio: la voce di ADR-0040 dice ciò che dice l'ADR, né più né meno; la riga nella voce di ADR-0022; i
     totali; la riga della §8; la riga della §13, coerente con la riga nuova di `CLAUDE.md` e con la 3.3; la riga della
     data **dice il vero** dopo questo commit;
   - **(d)** `docs/AVVIO-CHAT.md`: la cifra tolta **dentro** il messaggio recintato, e il richiamo **fuori**, in testa;
   - **(e)** che nessuna frase **dica il falso** dopo questo commit (gotcha #58), nei dieci file: soprattutto un totale o un
     *«l'ultimo»* degli ADR rimasto indietro — a parole, dove la guardia non guarda: la trappola 1 della §10 del
     compendio —, e ciò che dichiara *fatto* contro git e contro la posizione del piano.
5. **Ogni cifra nuova** nelle righe aggiunte — `git diff -U0 8bb8440..025dc0f` e le righe `+` con una cifra —: porta accanto il
   comando che la produce, o vive in una riga datata? Nei documenti della lista della guardia, i numeri piccoli a parole
   (vincolo 5).
6. **I vincoli globali** del brief — soprattutto i **fine-riga**: per ogni file toccato `git ls-files --eol` e
   `tr -cd '\r' < <file> | wc -c` contro `wc -l`, a `8bb8440` e dopo — su un file `w/crlf` i CR sono uguali alle righe, e la
   colonna `w/…` non cambia; ADR-0040 `w/lf` —; le **tabelle** — `tables.awk` del brief, il conto per file uguale a prima,
   più ADR-0040 —; il **perimetro**: `git diff --stat 8bb8440..025dc0f -- crates/ gui/ scripts/ .github/ Cargo.toml Cargo.lock`
   vuoto, nessun ADR oltre ADR-0022 e il nuovo, del piano la sola riga della posizione.
7. **Il cancello intero**, una volta, sull'albero del repository: `GATE GREEN`; e `bash scripts/check-docs.sh` → `OK`, col
   margine del compendio — il comando C del brief.
8. **Il contratto** del prompt dell'implementatore: **un** commit, coi **dieci** file del `git add` del Passo 5; il
   messaggio che comincia con `knowledge-base-revisione(compito 1): `; niente push — `git status -sb` → `ahead 1` —;
   niente co-autore — `git log -1 --format=%B 025dc0f | grep -ci co-authored` → `0`.

Un difetto del **dettato** — del piano, non dell'esecuzione — non si cura: è una **voce d'errata candidata**, la prossima
libera dopo quelle del rapporto — **ER-11** se non ne porta —, col testo corretto proposto. Una decisione già presa dal
proprietario o dal piano si **segnala** lo stesso, se la trovi sbagliata: dirlo è il tuo lavoro, deciderlo no.

## 3. Il rapporto

Scrivi `.superpowers/sdd/2026-09-29-knowledge-base-revisione-documenti/task-1-review.md`:

1. il **verdetto**, in una riga: conforme o no, e la qualità approvata o no;
2. i **rilievi**, classificati **Critico** (rende falso ciò che il compito consegna, o rompe un Atteso o un criterio),
   **Importante** (una regola del repository violata, o un'affermazione del rapporto non provata), **Minore**, **Nit** —
   ciascuno col file, la riga ritrovata col `grep` sulla frase, l'evidenza (comando e uscita) e la cura proposta **in forma
   di testo**, senza applicarla;
3. l'elenco dei **comandi rilanciati**, con le uscite;
4. le due direzioni della conformità nel clone: che cosa hai cambiato, e che cosa ha reso;
5. la rilettura del merito, punto per punto del §2.4;
6. ciò che **non** hai potuto verificare, e perché.

Nel messaggio finale al coordinatore: il verdetto, il conto dei rilievi per classe, il percorso del rapporto — in poche
righe.
