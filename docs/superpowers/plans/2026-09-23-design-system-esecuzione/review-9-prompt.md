Sei il **revisore del compito 9** — *la chiusura: i documenti in ogni casa, e la Definizione di «fatto» coi comandi* — del
piano `docs/superpowers/plans/2026-09-23-design-system.md`, nel repository `C:\Users\zagor\Desktop\harness` (Windows; il
tool Bash è Git Bash: apri ogni chiamata con `cd /c/Users/zagor/Desktop/harness &&`). Sei un subagente fresco e **non hai
scritto nulla** di ciò che rivedi. Rivedi **un commit**:

| Commit | Chi | Che cosa |
|---|---|---|
| `8c47715` | l'implementatore | il compito 9, sopra `13fea0d` |

**All'avvio verifichi:** `git rev-parse --short HEAD` → `8c47715`; `git status --porcelain` → vuoto;
`git log --oneline 13fea0d..HEAD` → **una** riga. Se non torna, ti fermi e lo riporti.

⛔ **Non modifichi nulla nell'albero del repository**: niente scritture, niente commit, niente push. Le prove che
chiedono una mutazione le fai su un **clone**: `git clone C:/Users/zagor/Desktop/harness C:/Users/zagor/AppData/Local/Temp/rv9`,
poi `git -C C:/Users/zagor/AppData/Local/Temp/rv9 checkout --detach 8c47715`. Il compito è di **soli documenti**: nel
clone non serve `npm ci`. Lo lasci dov'è: lo cancella il coordinatore. Lo scratchpad per i log e per i tuoi file è
`C:\Users\zagor\AppData\Local\Temp\claude\C--Users-zagor-Desktop-harness\f919d713-f8d4-41b3-a0d4-c94463b505da\scratchpad\review9`
(in Git Bash `/c/Users/zagor/AppData/Local/Temp/claude/C--Users-zagor-Desktop-harness/f919d713-f8d4-41b3-a0d4-c94463b505da/scratchpad/review9`).
Niente subagenti.

⛔ **Una cosa alla volta**: il cancello e un `npm`/`vitest` sullo **stesso** albero si pestano su `gui/node_modules`
(gotcha #133), e due suite insieme sulla stessa macchina rendono instabili le prove col tempo (P-20, P-21 del piano): mai
due suite, mai due cancelli, mai una suite mentre gira il cancello. Il cancello gira sull'**albero del repository**, in
background verso un log nuovo nello scratchpad, e aspetti la notifica. ⛔ Il browser delle prove è il **Chrome
installato**, senza finestra: se Playwright dice che manca, ti fermi e lo riporti — niente `npx playwright install`, niente
download. ⚠️ **Chrome si aggiorna da sé**: un rosso del progetto `browser` che non nomina una prova — `Target page, context
or browser has been closed`, `Failed to connect to the browser session` — si rilancia dopo aver riletto la versione dal
**nome della cartella**, `ls "/c/Program Files/Google/Chrome/Application/"`. ⛔ **Mai `chrome.exe --version`**: su Windows
apre il browser col profilo dell'utente.

La macchina, misurata dal coordinatore il 2026-09-28 alla ripresa: `core.autocrlf` `true` dal file di sistema
(`git config --show-origin --get-all core.autocrlf` rende la sola riga di `C:/Program Files/Git/etc/gitconfig`), quindi i
documenti del compito sono `i/lf` `w/crlf`; Node v24.19.0; Chrome `154.0.8037.58`, letto dal nome della cartella. Il
cancello d'apertura del coordinatore, a `13fea0d`, lanciato da solo: `GATE GREEN`, sotto `gui/` il progetto `jsdom` con 21
file passati e uno saltato, 170 prove passate e una saltata, il progetto `browser` con 6 file e 66 prove, il pezzo
JavaScript `698.20 kB` compresso `213.00 kB`, `found 0 vulnerabilities` — le cifre del pre-controllo. Il suo log è
`gate-open-13fea0d.log` nello scratchpad padre di `review9`, e **non** è quello dell'implementatore. La CI di `13fea0d` è
verde su `ubuntu-latest` e `windows-latest`.

## 1. Che cosa leggi

Nella cartella `.superpowers/sdd/2026-09-23-design-system/`:

| File | Che cos'è |
|---|---|
| `task-9-brief.md` | **il compito come era dettato**, parola per parola dal piano e dal disegno a `13fea0d`: la testa con gli *Strumenti*, i vincoli globali, *A che punto è* e *Come si esegue un compito*, l'errata (**E111**…**E120** sono del suo pre-controllo e sono **già applicate** al testo, anche se l'etichetta della sezione dice «E111…E118»: è un'etichetta vecchia dello script, nota al coordinatore), le voci P-26…P-30, le decisioni che il compito nomina, le voci aperte che il piano sa, il compito 9 intero con la sua **Definizione di «fatto»**, e dal disegno la (e), la (f), la tabella dei controlli, le assunzioni e il suo *«Come si riprende»*. Leggilo **tutto**, a blocchi: il suo peso lo dà `wc -c -l` |
| `task-9-dispatch.md` | il prompt che l'implementatore ha ricevuto — il **contratto** contro cui lo giudichi. ⚠️ Per **E120** il suo commit **non** porta il piano né l'archivio delle consegne, la Definizione di «fatto» la esegue e ne scrive le uscite nel **rapporto**, e i Passi 11 e 12 del piano sono del coordinatore |
| `task-9-report.md` | il **rapporto dell'implementatore**: è una dichiarazione, non un fatto |

E nella cartella tracciata `docs/superpowers/plans/2026-09-23-design-system-esecuzione/`: `compare_task9.py`, lo script
del pre-controllo che ricostruisce dal **testo del piano** a `13fea0d` i nove documenti dettati e li confronta col commit;
senza `--closure` il piano e `docs/archivio/consegna-piano-design-system.md` toccati sono `UNEXPECTED` (**E120**). ⚠️ **È
una dichiarazione**: lo leggi, e lo provi.

Poi `git show --stat 8c47715` e il diff, file per file. ⛔ **Non leggi** il piano intero (oltre 12 000 righe), il disegno
intero, `docs/adr/`, il resto di `docs/archivio/`, l'audit: dove ti serve, `grep -n` e `sed -n` nel punto esatto. Di
`docs/HANDOFF.md` leggi la sola sezione *«I gotcha»*, e solo per il punto 4. Le regole di `CLAUDE.md` valgono per te:
documenti in italiano, nessuna cifra misurata senza il suo comando, un puntatore in una casa sola, i fine-riga per file.

## 2. Che cosa verifichi

1. **Ogni affermazione misurabile del rapporto si RILANCIA** — il comando accanto, la tua uscita accanto alla sua
   (regola 5 di *«Come si esegue un compito»*). Elenca i comandi rilanciati. Un numero che non torna è un rilievo. I log
   del cancello e delle corse che il rapporto cita devono essere **dell'implementatore**:
   `ls -la --time-style=full-iso <log>` contro `git log -1 --format=%ci 8c47715`.
2. **La conformità al dettato.** `python docs/superpowers/plans/2026-09-23-design-system-esecuzione/compare_task9.py 13fea0d 8c47715`
   dalla radice del repository, con `PYTHONIOENCODING=utf-8`. Ma prima **provalo nelle due direzioni**: leggilo e di' se
   ricostruisce davvero ciò che i Passi 2–8 e 10 dettano; poi nel **clone** fai **due** commit usa-e-getta, uno alla volta,
   e dopo ciascuno lancia lo script, dal clone, con `13fea0d` e quel commit: uno che cambia **un** carattere dentro un testo
   **dettato** di un documento — per esempio nella riga 14 di `docs/roadmap.md` —, che deve dire `DIFFERS` su quel file **e
   su nessun altro** ed uscire 1; uno che cambia **un** carattere nel piano, che deve dire `UNEXPECTED` (**E120**) ed uscire 1.
   Poi il clone torna a `8c47715`. Sul commit vero, nulla `DIFFERS`, nulla è `UNEXPECTED` né `MISSING`, uscita 0. I valori
   liberi che lo script **mostra** — le cifre del pezzo JavaScript, `<Enn>` se c'è, `docs/HANDOFF.md` — li giudichi tu: le
   cifre contro la tua misura del punto 6, il testo del gotcha al punto 4.
3. **La Definizione di «fatto», blocco per blocco**, contro le uscite che il rapporto riporta: i blocchi **3** e **4** si
   rilanciano per intero, comando per comando — con **E111** (le dichiarazioni di `LayoutPack`, `3`) ed **E112** (il
   perimetro ha tre case) —; il blocco **1** è il cancello del punto 6; del blocco **2** fai **una** corsa, con l'aiutante
   dettato che copi dal brief nello scratchpad, non cinque: il codice non cambia dal pre-controllo — lo prova
   `git diff --stat 13fea0d..8c47715 -- gui/ crates/ scripts/`, che deve essere vuoto —, e le cinque corse le hanno
   fatte il pre-controllo e l'implementatore. ⚠️ Una prova che cade anche **una** volta si riporta con la sua uscita: non si
   rilancia finché passa (P-19). Il blocco **5** — ciò che un comando non dice —: il rapporto dice dove ha guardato, e
   regge?
4. **Il Passo 9** (**E116**): le voci che il rapporto dice di aver letto — l'errata e le tabelle *«Ciò che questa sessione
   ha imparato»* delle consegne, che il comando del passo elenca: rilancialo e confronta l'elenco. Per ogni lezione, la
   classe che le dà il rapporto — una **trappola** diventa un gotcha, una **regola di lavoro** una proposta per
   `CLAUDE.md`, altrimenti il perché —: la classe regge? Per ogni gotcha scritto in `docs/HANDOFF.md`: è una trappola vera,
   **nuova** — cercala fra quelli che ci sono, per parola —, col numero che continua la serie, nella forma delle righe
   vicine, e il comando che conta i gotcha della §9 del compendio rende ancora il numero giusto? Una lezione che il rapporto
   non classifica è un rilievo.
5. **Il testo archiviato, parola per parola** (**E117**): il puntatore e l'intestazione del compendio che il Passo 6 porta
   in `docs/archivio/stato-storico.md` — `word_for_word.py`, dettato nel Passo 6, che copi dal brief nello scratchpad —
   contro il testo di `13fea0d`. ⚠️ `check-docs.sh` **non** vede una parentesi persa in un rimando (lezione 1 del
   pre-controllo): lo script sì. Provalo anche lui in una direzione che deve fallire: una parola cambiata in una copia.
6. **Il cancello intero**, una volta, sull'**albero del repository**, in background verso un log nuovo: `GATE GREEN`; le
   righe `Test Files` e `Tests` dei due progetti, contro il rapporto e contro la base — il compito non tocca le prove, e
   nessuna deve essere sparita, per **nome di file** e non solo per conto —; `found 0 vulnerabilities`; la riga del pezzo
   JavaScript contro la base. E `bash scripts/check-docs.sh` → `OK`, e il compendio sotto il suo tetto, col margine: il tetto
   lo dice `scripts/check-docs.sh`, i byte `wc -c docs/COMPENDIO.md`.
7. **I documenti, oltre il dettato.** Un difetto del **dettato** non si cura: è una **voce d'errata candidata** — la prossima
   libera è quella dopo le candidate del rapporto, **E121** se non ne porta — col testo corretto proposto. Guarda
   soprattutto:
   - **(a)** ogni **cifra** nuova nelle righe aggiunte — `git diff -U0 13fea0d..8c47715` e le righe `+` con una cifra —:
     porta accanto il **comando** che la produce, o vive in un verbale datato? Un numero misurato scritto senza comando è
     un rilievo (gotcha #31);
   - **(b)** i **puntatori** in una casa sola (gotcha #68): il prossimo passo vive nella sola §6 del compendio — fuori da
     `docs/COMPENDIO.md`, ogni riga nuova o cambiata che porta `⏭️` deve nominare la §6 —; nessun documento secondario
     riscrive lo stato invece di rimandarvi; e il puntatore nuovo **dice il vero**, contro la roadmap e contro ciò che la §6
     di `13fea0d` diceva del sotto-progetto 13 e di AUD-004;
   - **(c)** le **tabelle**: nessuna riga spezzata e nessun a-capo dentro una cella —
     `awk 'prev ~ /^\|/ && $0 == "" {getline nxt; if (nxt ~ /^\|/) print NR} {prev=$0}' <file>` e
     `awk 'prev ~ /^\|/ && $0 != "" && $0 !~ /^\|/ {print NR} {prev=$0}' <file>` su ogni file toccato, contro lo stesso
     comando su `git show 13fea0d:<file>` —, e il testo appeso a una cella **prima** di ` |` (**E119**);
   - **(d)** i **rimandi** nuovi: il file esiste, e un'ancora — che `check-docs.sh` non controlla — porta a un titolo vero;
   - **(e)** che nessuna frase **dica il falso** dopo questo commit (gotcha #58), nei nove documenti e in `HANDOFF.md`:
     soprattutto ciò che dichiara *fatto*, *chiuso* o *eseguito*, contro git e contro la posizione del piano — che al
     commit dell'implementatore ha ancora la riga 9 a `⬜`, perché le celle del piano le scrive la chiusura (**E120**);
   - **(f)** le **promesse** fatte al compito 9: le consegne archiviate e l'errata dicono *«le raccoglie la chiusura»* o
     *«lo scrive il compito 9»* — `grep -n 'compito 9\|la chiusura' <file>` nei file che il brief nomina —: ciascuna è mantenuta,
     o dichiarata?
8. **I vincoli globali** del brief — soprattutto i **fine-riga**: per ogni file toccato `git ls-files --eol` e
   `tr -cd '\r' < <file> | wc -c` contro `wc -l`, a `13fea0d` e dopo — su un file `w/crlf` i CR sono **uguali alle righe**,
   e la colonna `w/…` non cambia —; e il **perimetro**: `git diff --stat 13fea0d..8c47715 -- crates/ gui/ scripts/ .github/ Cargo.toml Cargo.lock`
   vuoto; nessun ADR, nessuna riga della §5 del compendio, nessuna riga della spec del sotto-progetto 1, nessun piano
   eseguito, `CLAUDE.md` intatto.
9. **Il contratto** del prompt dell'implementatore: **un** commit, coi soli documenti del suo punto 2 — i nove, e
   `docs/HANDOFF.md` solo se il Passo 9 scrive un gotcha —; **né il piano né** `docs/archivio/consegna-piano-design-system.md`
   (**E120**); il messaggio che comincia con `design-system(compito 9): `; niente push (`git status -sb` → `ahead 1`);
   niente co-autore (`git log -1 --format=%B 8c47715 | grep -ci co-authored` → 0).

## 3. Il rapporto

Scrivi `.superpowers/sdd/2026-09-23-design-system/task-9-review.md`:

1. il **verdetto**, in una riga: conforme o no;
2. i **rilievi**, classificati **Critico** (rende falso ciò che il compito consegna, o rompe un Atteso o un criterio),
   **Importante** (una regola del repository violata, o un'affermazione del rapporto non provata), **Minore**, **Nit** —
   ciascuno col file, la riga ritrovata col `grep` sulla frase, l'evidenza (comando e uscita) e la cura proposta **in
   forma di testo**, senza applicarla; un difetto del **dettato** come voce d'errata candidata;
3. l'elenco dei **comandi rilanciati**, con le uscite;
4. le due direzioni di `compare_task9.py` e di `word_for_word.py`: che cosa hai cambiato nel clone o nella copia, e che
   cosa hanno reso;
5. il Passo 9: la tua tabella delle lezioni — la consegna, la lezione, la classe del rapporto, la tua;
6. ciò che **non** hai potuto verificare, e perché.

Nel messaggio finale al coordinatore: il verdetto, il conto dei rilievi per classe, il percorso del rapporto — in poche
righe.
