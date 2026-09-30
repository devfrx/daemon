# Ri-revisione del compito 1 — la cura `1a8c237`, contro i rilievi della revisione

Scritta dal ri-revisore il 2026-09-30, sulla macchina `jays`, su `1a8c237` sopra `025dc0f`. Il ri-revisore non ha scritto
nulla di ciò che rivede e **non ha toccato l'albero del repository** — `git status --porcelain` vuoto e `HEAD` = `1a8c237`
all'inizio e alla fine —: le prove che scrivono stanno in `…/scratchpad/rereview1`, qui sotto `R`, e le copie pesanti — un
clone, tre simulazioni, un repository di prova — sono state tolte dopo l'uso. Nessun subagente; il cancello non l'ho girato,
come vuole il mandato; `check-docs.sh` sì.

**All'avvio:** `git rev-parse --short HEAD` → `1a8c237`; `git status --porcelain` → vuoto; `git log --oneline 025dc0f..HEAD`
→ una riga; `git status -sb` → `## main...origin/main [ahead 2]`. Tutto come dice il mandato.

## 1. Il verdetto

**Tutti i rilievi risolti — R1…R8, ER-11, ER-12 —, e la cura non rompe i compiti che restano: i compiti 2–6 simulati in
sequenza su un clone di `1a8c237`, `check-docs.sh` → `OK` dopo ciascuno e il margine sempre positivo. Rilievi nuovi: due
Minori e tre Nit, nessun Critico né Importante.**

| Classe | Quanti | Quali |
|---|---|---|
| Critico | nessuno | — |
| Importante | nessuno | — |
| Minore | due | M1 — la riga della data archiviata come *«uscita dal compendio»*, e rimasta; M2 — la colonna *«Commit»* a *«due»* senza una voce d'errata |
| Nit | tre | N1 — due citazioni fra caporali non letterali; N2 — la riga di R8 senza la clausola della 4.2; N3 — il *«Come si riprende»* tace i tre Nit e la decisione di rimandarli |

## 2. Rilievo per rilievo

| Rilievo | Esito | L'evidenza |
|---|---|---|
| **R1** — la testa della posizione | ✅ **RISOLTO** | il capoverso è la sola riga fisica **444** del piano, fra due righe vuote (`sed -n '442,447p'`); è il testo che la revisione proponeva, parola per parola: *«… ▶️ **L'esecuzione è in corso**, un compito per sessione: fin dove, lo dice la tabella qui sotto; …»*. `grep -n "L'esecuzione non è cominciata"` sul piano rende solo le righe di ER-13 ed ER-14, che la citano |
| **R2** — POS6 taglia il capoverso | ✅ **RISOLTO** | POS6 applicato a una copia del piano di `HEAD` con `D=2026-10-09`: `checked: 2 edits in 1 files`, `applied: 2 edits in 1 files`; il `diff` contro il piano tocca le sole righe 444 e 453; sotto l'intestazione della posizione restano la riga *«✅ **IL PIANO È ESEGUITO, il 2026-10-09.** …»*, una riga vuota e la tabella — nessuna riga orfana. CR = righe prima e dopo: `2085` e `2085`. Lo stesso nella simulazione in sequenza (§5.7) |
| **R3** — il puntatore e il *«Come si riprende»* | ✅ **RISOLTO** | `grep -c 'riparte dal compito' docs/COMPENDIO.md` → `0`; `grep -c 'esecuzione è in corso' docs/COMPENDIO.md` → `1`; `grep -c 'Nessun compito è eseguito'` sul piano → `0`; le controprove in archivio rendono `1` e `1`. Il puntatore com'era sta in coda a `stato-storico.md`, rifatto byte per byte (§5.2). Nessun numero di compito nelle righe aggiunte del compendio e della consegna del metodo (§4) |
| **R4** → ER-15 | ✅ **RISOLTO**, per registrazione | ER-15 è nell'errata, riga 505, e nel punto 5 del *«Come si riprende»*; il chiusore è nominato: il compito 2, blocco E2. Dice il vero: ADR-0040 riga 57 e compendio riga 585 rimandano al rimando di ADR-0016; `grep -n -i 'rimando\|cartella dati\|percors[oi] protett'` su ADR-0016 → niente, exit `1`; nel blocco E2, la parte di ADR-0016 porta *«cartella dati»* → `1`. Sta nell'errata e non fra le voci aperte, giusto: la chiude un compito di questo piano. D10, che ER-15 cita, dichiara davvero il ritardo della riga della data (riga 588) |
| **R5** → ER-16 | ✅ **RISOLTO**; la metà su `HANDOFF.md` per registrazione, con N1 | la §7, riga 720: *«(`Superseded by`) — o che lo superi **in parte**, la riga «ADR superato in parte» della §13 —, non una conversazione»*; coerente con la riga 917 della §13, *«ADR **superato in parte**»*, e con la riga 79 di `CLAUDE.md`, *«**superato in parte** → un rimando in testa che nomina le righe, e l'ADR nuovo lo dichiara — lo stato resta `Accepted`»*: in tutte e tre serve un ADR nuovo. `HANDOFF.md` non è nel diff; la sua frase, righe 1056–1057, è registrata fra le voci aperte, riga 654, col proprietario come chiusore — ma citata male: N1. Nessun'altra gemella fuori da archivio e piani: `grep -rn -i 'rimett[a-z]* in discussione'` trova solo le due |
| **R6** → ER-17 | ✅ **RISOLTO** | §5.3: il recinto del compito 1 col `<data>` sostituito da `2026-09-30`, la `Date` del file, è **identico** ad ADR-0040 (`cmp`); fra il recinto di `025dc0f` e quello di `HEAD` cambia la sola riga 82, e la coda tolta è esattamente *«: il sotto-progetto 6, se nessuno le porta prima»*; la frase ora finisce come la 3.2 del disegno, riga 372, *«chi costruisce la cartella dati lo dice la sezione 4»*. ADR-0040 non è pubblicato: `git cat-file -e origin/main:<ADR-0040>` fallisce, `origin/main` = `8bb8440` |
| **R7** → ER-18 | ✅ **RISOLTO**, per registrazione | il commento è alla riga 191 di `crates/daemon/src/main.rs`, *«a window layout is neither»*, del 2026-09-19 (`git blame`), prima del piano (`ff6f0e5`, 2026-09-29); la disposizione è nella configurazione — ADR-0022 righe 8, 13, 15 — e la configurazione nel backup — ADR-0040 riga 55. ER-18 all'errata, riga 508; la riga fra le voci aperte, 652, subito sotto quella di ER-3, con lo stesso chiusore |
| **R8** | ✅ **RISOLTO**, per registrazione — con N2 | la riga 653 fra le voci aperte: dice il vero — ADR-0023, rimando del 2026-08-18, righe 102–108, il file nasceva `0644`, *«leggibile da qualunque utente della macchina»*; riga 113, `0600` sul file; ADR-0040 riga 74, la copia in chiaro *«la protegge il sistema operativo»* —; sta dove il piano tiene le voci aperte; nomina un chiusore, il 6 — ma senza la clausola della 4.2 sulla cartella dati: N2. La domanda A/B al proprietario è diventata la decisione 7 del *«Come si riprende»*, ribaltabile, col costo detto |
| **ER-11** | ✅ **RISOLTO** | riga 501: è il testo raffinato della 5.6 della revisione, a meno di due congiunzioni. Dice il vero: `git diff --numstat 8bb8440 025dc0f` — i file `w/crlf` che guadagnano righe sono proprio ADR-0022 (+14), `README.md` (+1), il compendio (+11), `AVVIO-CHAT.md` (+2) e l'archivio (+6); il rapporto tracciato porta i CR per file prima (righe 159–167) e dopo (328–337); `conv()` di `apply_edits.py` converte in CRLF il testo nuovo |
| **ER-12** | ✅ **RISOLTO** | riga 502: al Passo 4 il comando è `git ls-files --eol $F docs/adr/0040-*.md`, con otto file in `F` (brief, riga 963); `grep -n '^- Create:'` sul piano → una riga sola, la 749. La misura, rifatta in un repository di prova con lo stesso `core.autocrlf` (`true`): non tracciato con `-o --exclude-standard` → `i/ w/lf`; senza `-o` → niente; dopo `git add` → `i/lf w/lf`. Oggi ADR-0040 rende `i/lf w/lf` |
| R9, R10, R11 | ⏸️ rimandati — decisione del coordinatore, scritta nel registro (`progress.md`, riga 35) | **R10 e R11: giusto.** R10 è d'ordine, e la revisione stessa non cura adesso; R11 chiederebbe di ritoccare la copia tracciata di un rapporto che dev'essere parola per parola, e la revisione, tracciata accanto, dice già il numero giusto. **R9: non la direi sbagliata** — segue il consiglio della revisione, *«nessuna in ADR-0040»*, e la 3.4 del disegno porta la stessa frase —, **ma la finestra che ha giustificato ER-17 vale anche qui**: dopo il push il *Context* di ADR-0040, riga 34, attribuisce per sempre a una fonte *«letta alla fonte»* un dedotto, *«e in nessun backup»*, e si corregge solo con un rimando datato. Se il commit si riapre per M1 o M2, un *«⚠️ dedotto»* accanto a quelle parole, nel file e nel recinto, con una voce d'errata, costa quanto ER-17. La decisione, però, vive nel **solo registro**, che non è tracciato: N3 |

## 3. I rilievi nuovi, nel solo diff della cura

### M1 — Minore: la riga della data è archiviata come *«uscita dal compendio»*, ed è rimasta; e non dice la §7 nuova

- **I file e le righe:** `docs/archivio/stato-storico.md` righe 2694–2698 — `grep -n 'archiviati il 2026-09-30, alla chiusura del compito 1' docs/archivio/stato-storico.md` → `2694`; l'avvertenza alla 2696, *«Veri il giorno in cui furono scritti.** Usciti dal compendio parola per parola»*; la riga della data archiviata alla 2698. `docs/COMPENDIO.md` riga 20 — `grep -n 'Il contenuto di merito nuovo è la voce di ADR-0040' docs/COMPENDIO.md` → `20`.
- **L'evidenza:** la cura ha lanciato `archive_head.py --pointer`, che archivia **sempre** anche la riga della data (brief, righe 329–332), e poi ha riscritto il solo puntatore. La riga della data del compendio è la stessa a `025dc0f` e a `1a8c237` — `diff` delle due righe `**Aggiornato il ` → vuoto —, e l'ultima riga archiviata, col link rimesso, è **identica** a quella viva. L'archivio ne porta sedici e tutte diverse (`sort -u` → `16` su `16`): è la prima volta che una riga della data entra in archivio senza essere riscritta, e l'intestazione — *«com'erano»*, *«Usciti dal compendio»* — lo dice falsamente. D10 del piano accoppia i due gesti: *«si riscrive … e ogni volta la riga di prima va in archivio»*. Intanto la cura ha scritto contenuto di merito nella §7, riga 720, e la riga della data dice ancora *«Il contenuto di merito nuovo è la voce di ADR-0040, coi rimandi»*: è l'evento che la §13 registra come scoperto — *«una riscrittura di merito dentro una riga che c'è già»*, AUD-034.
- **Perché Minore:** nessun passo del piano dipende da questa riga, e l'archivio non è lettura obbligatoria; ma la riga della data è quella che la §13 chiama la più facile da lasciare indietro, e il compendio è una lettura d'apertura.
- **La cura, in testo:** **A, il consiglio** — riscrivere adesso la riga della data, che è ancora il compito 1 e lo stesso giorno, come D10 la vuole riscritta *«al compito 1»*, col comando C prima — oggi `6353` —, perché nomini anche la §7; per esempio: *«… la riga del caso nuovo in §13 e la regola della §7 che lo nomina — e, compito per compito, … Il contenuto di merito nuovo è la voce di ADR-0040, coi rimandi, e il superamento in parte nella §7. …»*. La copia già in archivio diventa vera — è la riga *«com'era»* — e non serve un secondo `archive_head.py`. Il blocco E6 ancora la riga col prefisso `>= **Aggiornato il`: non cambia. **B** — lasciare la riga, e togliere a mano dalla coda dell'archivio la copia, con l'intestazione nella forma del solo puntatore: `archive_head.py` quella forma non l'ha, il testo diventerebbe scritto a mano, e la §7 resterebbe taciuta. Sconsigliata.

### M2 — Minore: la colonna *«Commit»* dice *«due»* per il compito 1, senza una voce d'errata; la Forma dice *«un commit ciascuno»*, e POS2…POS6 dettano *«uno»*

- **I file e le righe:** il piano, riga 448 — `grep -n '^| \*\*1\*\* | ADR-0040' <piano>` → `| **1** | … | due | ✅ 2026-09-30 |`; la Forma, riga 20 — `grep -n 'un commit ciascuno' <piano>` → `20`; i recinti di POS2…POS6, righe 1210, 1464, 1651, 1812, 1953, tutti con `| uno |`.
- **L'evidenza:** POS1 dettava *«uno»*, ed è ciò che il commit del compito 1 aveva scritto; la cura l'ha portato a *«due»*. Il numero è vero — `git log --oneline --grep='knowledge-base-revisione(compito 1)' | wc -l` → `2`, controprova sul compito 2 → `0` —, ma **nessuna voce** lo dice: né l'errata, né le decisioni del *«Come si riprende»*. E il piano ora si contraddice: la Forma dice *«Sei compiti in sequenza, un commit ciascuno»*. Col modo di lavorare adottato — le decisioni 2 e 4: l'implementatore committa, il coordinatore cura e chiude in un commit suo, e quello del compito 1 ha preso il prefisso del compito — i compiti 2–6 avranno con ogni probabilità due commit ciascuno, e POS2…POS6 scriveranno *«uno»*: la simulazione in sequenza (§5.7) chiude il piano con la colonna *«due, uno, uno, uno, uno, uno»*. La regola 3 di *«Come si esegue un compito»*: *«una divergenza è una voce d'errata prima di essere un rimedio»*.
- **Perché Minore:** nessun controllo legge la colonna, e i commit veri li dà il comando della riga 457; ma è la casa unica della posizione, e il difetto torna cinque volte.
- **La cura, in testo — una voce d'errata, per esempio ER-19:** *«⚠️ Compiti 1–6, la Forma e i blocchi POS1…POS6 — la colonna «Commit» detta «uno», e dal compito 1 un compito ha due commit col suo prefisso: quello dell'implementatore e la chiusura del coordinatore, con le cure della revisione — le decisioni 2 e 4 del «Come si riprende», E120. Trovata dalla ri-revisione del compito 1, il 2026-09-30. ✅ Alla chiusura di ogni compito il coordinatore porta nella colonna, a parole, il conto che rende `git log --oneline --grep='knowledge-base-revisione(compito N)'`; la Forma, «un commit ciascuno», si legge così.»* Riallineare i cinque recinti costerebbe cinque blocchi da riprovare: non lo consiglio.

### N1 — Nit: due citazioni fra caporali non sono parola per parola

- **I file e le righe:** il piano, riga 654 — `grep -n 'Rimettere in discussione uno' <piano>` → `654` —, e riga 2065 — `grep -n 'si annota al Passo 1, ed è la stessa al Passo 4' <piano>` → `2065`.
- **L'evidenza:** la riga 654 localizza la frase di `HANDOFF.md` come *«Rimettere in discussione uno richiede un ADR nuovo che lo superi»*; `HANDOFF.md` dice *«Rimetterne in discussione uno **richiede un ADR** / **nuovo che lo superi**»*, a capo dopo *«ADR»*: `grep -c 'Rimettere in discussione uno' docs/HANDOFF.md` → `0`, `grep -c 'Rimetterne in discussione uno' docs/HANDOFF.md` → `1`. La colonna *«Dove vive»* è un localizzatore, e con questa citazione il `grep` del chiusore non trova niente. La decisione 1 mette fra caporali una parafrasi di ER-9, che dice *«al Passo 1 il valore sull'archivio si **annota**, e al Passo 4 è **lo stesso**»*.
- **La cura, in testo:** alla riga 654, *«Rimetterne in discussione uno»* — un pezzo corto, perché la frase va a capo, come insegna una lezione del pre-controllo —; alla 2065, ER-9 con le sue parole, o la parafrasi senza caporali.

### N2 — Nit: la riga di R8 dà al 6 la cartella dati senza la clausola della 4.2

- **Il file e la riga:** il piano, riga 653 — `grep -n 'il \*\*6\*\*, che costruisce le copie e la cartella dati' <piano>` → `653`.
- **L'evidenza:** la 4.2 del disegno, riga 471, dà al 6 le copie — *«conservare e ripristinare, cioè le copie»* — e la cartella dati *«se nessuno le porta prima»*; le due righe sorelle, di ER-3 alla 651 e di ER-18 alla 652, portano la clausola, questa no. Se la cartella dati la costruisce prima un altro sotto-progetto, la riga manda la proprietà di sicurezza di PL-1 al chiusore sbagliato proprio dove serve: al primo che crea la cartella. È la stessa commissione che ER-17 ha tolto da ADR-0040; qui, nella colonna che deve nominare un chiusore, basta renderla esatta.
- **La cura, in testo:** *«il **6** per le copie; per la cartella dati chi la costruisce per primo — il 6, se nessuno la porta prima — la 4.2 del disegno»*.

### N3 — Nit: il *«Come si riprende»* tace i tre Nit della revisione e la decisione di rimandarli

- **Il file e la riga:** il piano, riga 2019 — `grep -n 'e sei Minori' <piano>` → `2019` —, e la tabella delle decisioni, dalla riga 2061.
- **L'evidenza:** il capoverso d'apertura conta *«due Importanti … e sei Minori»*, e la revisione ne aveva anche tre Nit; la tabella *«Le decisioni prese in questa sessione … il proprietario può ribaltarle»* ha otto righe, e nessuna dice che R9, R10 e R11 sono rimandati. Quella decisione vive nel solo registro, `progress.md`, che `.superpowers/sdd/.gitignore` esclude (`git check-ignore -v` → `*`): non arriva sull'altra macchina, e il proprietario non la vede per ribaltarla. La cura proposta da R11 — l'etichetta nella copia tracciata del rapporto — resta non applicata senza che un documento tracciato dica perché.
- **La cura, in testo:** nel capoverso, *«…, sei Minori e tre Nit»*; e una nona riga: *«R9, R10 e R11 della revisione **non curati** | R9: nessuna cura in ADR-0040, come consiglia la revisione — la 3.4 del disegno porta la stessa frase —; R10: d'ordine, alla prossima volta che la testa di `AVVIO-CHAT.md` si tocca; R11: la copia tracciata del rapporto resta parola per parola, e la revisione dice il numero giusto accanto. Costo: …»*.

## 4. Il vincolo 13 nelle frasi nuove — §3(a) del mandato

Nel compendio e nella consegna del metodo le righe aggiunte non portano **nessun** numero di compito:
`git diff -U0 025dc0f 1a8c237 -- <file> | grep '^+[^+]' | grep -o -i '.\{0,50\}compit[oi] [0-9]…'` → niente su entrambi.
Nel piano i numeri che compaiono sono di due specie, e **nessuna** è il numeratore:

| Specie | Dove | Perché non marcisce |
|---|---|---|
| l'**identità** di un compito, nella forma di sempre delle voci | ER-11…ER-18 (*«Compito 1, Passo 4 — …»*); le tre righe nuove fra le voci aperte; il punto 5, *«Il compito 2 chiude la finestra di ER-15»* | dice che cosa fa un compito, non a che punto si è: resta vera quale che sia il prossimo passo |
| la **storia** della sessione, sotto un'intestazione datata | *«scritto alla chiusura del compito 1, il 2026-09-30»*; *«Il compito 1 è eseguito, rivisto e curato»*; *«il pre-dispaccio, il compito 1, la chiusura»*; la tabella dei costi | è vera per sempre — un compito eseguito resta eseguito —, e la consegna si riscrive a ogni chiusura, con la precedente in archivio |

Il *«prossimo»* è sempre detto senza numero: *«il prossimo compito, o un ticket della mappa»*, *«la prima riga ⏳ della
tabella della posizione»*. La forma che R3 aveva colto — *«riparte dal compito 1»*, una promessa che il compito 1 rendeva
falsa — non torna in nessuna riga aggiunta.

**Le altre voci di §3(a), dette in breve:** i commit della sessione — `git log --oneline dd0e265..HEAD` → `8bb8440`, `025dc0f`,
`1a8c237`, il pre-dispaccio, il compito 1, la chiusura ✅ —; i costi — ~272k, 58 chiamate, ~20 min; ~452k, 124, ~38 min; la
banda 0,5–0,8 milioni; la somma ~0,72 — tornano col registro, righe 23, 25 e 27 ✅; le decisioni 1–8 tornano col registro, con
ER-9 (riga 499), con E120 nell'archivio del design system (riga 2556: *«il push è suo»*) e col punto 5 del prompt spedito,
che vietava all'implementatore il piano oltre la riga della posizione ✅; le lezioni — l'`<data>` sostituito da `conv()`
ovunque, il `git add -N` del clone, la `bash` di WSL, il controllo dei link che esclude apposta `docs/superpowers/plans/`
(`scripts/check-docs.sh`, il commento del passo *«internal links»*) e la sequenza fra apici inversi alla riga 256 del rapporto
tracciato — tornano ✅; la cartella del dispaccio porta davvero `dispatch-task-1.md`, il modello col campo `<HEAD>`, e
`_extract_brief_1.py` ✅. Nessun link nelle righe aggiunte del piano, che il controllo dei link non legge.

**Le cifre nuove — §3(f):** i costi stanno sotto un'intestazione datata, *«Il costo misurato, il 2026-09-30»*; il *«due»*
della posizione ha il suo comando alla riga 457; *«otto righe, e non nove»* di ER-12 è datata e nomina il comando. ✅

## 5. I comandi rilanciati, con le uscite

**5.1 Gli attrezzi.** `extract.py` scritto a mano dal brief, righe 74–126, col tool di scrittura: 53 righe, `0` CR, `0` barre
rovesciate (`tr -cd '\134'`); identico al recinto del brief e a quello del piano di `HEAD` (`cmp`); controprova, una copia con
`chr(96) * 3` → diversa. Lanciato sul piano di `HEAD` e su quello di `025dc0f`: diciotto righe ciascuno, `EXIT=0`; fra i due
cambia il solo `adr0040.md`. I quattro attrezzi estratti sono identici ai recinti del brief (`cmp`, per ciascuno).

**5.2 Le prove del coordinatore.**

```text
# la consegna archiviata contro il «Come si riprende» del piano a 025dc0f
old section: plan@025dc0f lines 2006-2061 (56); archived copy: archive lines 204-259 (56)
diff: 7c7  < [archivio](../../archivio/consegna-piano-…)   > [archivio](consegna-piano-…)      -> la sola riga col link

# il puntatore e la riga della data in coda a stato-storico.md: archive_head.py --pointer rilanciato, con la stessa
# intestazione, sul compendio e sull'archivio di 025dc0f (CRLF come nell'albero: CR 930 = righe, 2692 = righe)
archived: 2 piece(s) under «…»
SIM == REPO, byte for byte
controprova, lo stesso sul compendio di 1a8c237: DIFFERENT — la sola riga 2704, quella del puntatore

# i blocchi che restano, sul repository (la data di oggi)
e2: checked: 20 edits in 11 files     pos2: checked: 1 edits in 1 files
e3: checked: 59 edits in 4 files      pos3: checked: 1 edits in 1 files
e4: checked: 24 edits in 3 files      pos4: checked: 1 edits in 1 files
e5: checked: 32 edits in 3 files      pos5: checked: 1 edits in 1 files
e6: checked: 6 edits in 4 files       pos6: checked: 2 edits in 1 files
controprova, e1 già applicato: refused: 18 problem(s), no file written

# POS6 su una copia del piano: vedi R2

# tables.awk sui dieci file toccati, a 025dc0f e a HEAD
0 righe prima e 0 dopo su ciascuno (i quattro nuovi: 0); controprova su una riga rotta: «ctl.md:3: 2 contro 3 (riga 1)»

# fine-riga: git ls-files --eol, CR, righe
w/crlf  COMPENDIO 930=930 · consegna in archivio 259=259 · stato-storico 2709=2709 · il piano 2085=2085
w/lf    ADR-0040 0 · consegna del metodo 0 · le quattro copie 0

# comando C
6353   (a 025dc0f: 6483)

# bash scripts/check-docs.sh
exit=0, «OK — no inconsistencies.»

# le copie nella cartella del dispaccio, contro la cartella di lavoro
IDENTICAL: task-1-dispatch.md (8335 bytes), task-1-report.md (38240), review-1-prompt.md (9962), task-1-review.md (50103)
controprova, due file diversi: DIFFERENT; e per ciascuna, blob di 1a8c237 == albero
```

**5.3 ADR-0040 — §3(c).** `grep -c '<data>'` sul recinto → `1`; `sed "s/<data>/2026-09-30/"` del recinto di `HEAD` → `cmp`
col file: identico; controprova, il recinto di `025dc0f` contro il file di oggi: diverso; il recinto di `025dc0f` contro il
file di `025dc0f`: identico. `git diff --numstat 025dc0f 1a8c237 -- docs/adr/` → `1 1` sul solo ADR-0040.

**5.4 La consegna del metodo — §3(e).** `git diff -U0` → due hunk, righe 37 e 104; `--word-diff`: tolti *«nessun compito
eseguito»*, *«dal compito 1»*, *«1 del»*, aggiunti *«fin dove, lo dice la tabella della posizione del piano»*, *«che viene
nel»*, *«la prima riga ⏳ della sua tabella della posizione, e»*. Nient'altro.

**5.5 Il contratto — §4 del mandato.** `git log -1 --format=%s 1a8c237 | grep -c '^knowledge-base-revisione(compito 1): '` →
`1`; `git log -1 --format=%B 1a8c237 | grep -ci co-authored` → `0`, controprova su una riga col trailer → `1`;
`git diff --name-only 025dc0f 1a8c237 -- crates/ gui/ scripts/ .github/ 'Cargo.*' | wc -l` → `0`, controprova su `docs/` →
`10`; il comando D da `dd0e265` e da `025dc0f` → vuoto.

**5.6 Il comando E.** Sull'archivio: `8` a `025dc0f`, `9` a `HEAD` — il puntatore archiviato porta il link al disegno, come
R3 prevedeva e come ER-9 vuole che si annoti; sul compendio `1` e `1`; sulla consegna in archivio `1` e `1`.

**5.7 In più: i compiti 2–6 in sequenza, su un clone di `1a8c237`** — `git clone --no-local` in `R`, poi `R/seq.sh`, un
compito al giorno dal 2026-10-01, coi comandi dei compiti — i blocchi, `archive_head.py --pointer`, `replace_pointer.py` —:

```text
task 2: checked/applied 20 edits in 11 files; pos 1   check-docs exit=0 OK   margin=4448   D vuoto
task 3: checked/applied 59 edits in 4 files;  pos 1   check-docs exit=0 OK   margin=4260   D vuoto
task 4: checked/applied 24 edits in 3 files;  pos 1   check-docs exit=0 OK   margin=4260   D vuoto
task 5: checked/applied 32 edits in 3 files;  pos 1   check-docs exit=0 OK   margin=4260   D vuoto
task 6: archived 2 pieces; 6 edits in 4 files; replaced lines 628-637; pos6 2 edits   check-docs exit=0 OK   margin=4691
CR = righe su tutti i ventiquattro file toccati dalla sequenza — i ventitré dei blocchi, più l'archivio
```

Il minimo del margine scende di 130 byte rispetto alla simulazione del pre-dispaccio — `4390` in ER-9, `4260` qui —: è
ciò che la cura ha speso, `6483 − 6353`. Dopo il compito 6 la testa della posizione è la sola riga di POS6, senza orfane.
ADR-0040 non è toccato dai compiti 2–6.

## 6. Ciò che non ho potuto verificare, e le righe fuori perimetro

- **Il push.** *«Tutto è pushato»* e *«`main` allineato a `origin`»*, nel *«Come si riprende»*, sono veri solo dopo il push,
  che per contratto segue questa ri-revisione: oggi `origin/main` = `8bb8440`, `ahead 2`. Se da qui nasce un altro commit,
  la riga dei commit della sessione e la tabella dei costi lo devono portare, con questa ri-revisione.
- **Il cancello.** Non l'ho girato. Il log del coordinatore, `gate-2026-09-30-chiusura.log`, finisce con `GATE GREEN.` ed
  `EXIT=0`, scritto alle 12:02:59; il commit è delle 12:03:08.
- **I costi e le risposte del proprietario.** Li ho confrontati col solo registro, che li copia dai rapporti del tool e dalla
  chat: non ho visto né gli uni né l'altra. Così la *«risposta A»* della decisione 3 e il fatto che R8 sia stato detto al
  proprietario.
- **La lezione dei `+L` senza barra.** È un fatto di prima del commit: il file di modifiche finale del coordinatore porta la
  barra su tutte le undici righe `+L`, il che è coerente, non una prova.
- **Il contenuto delle quattro copie tracciate.** Le ho confrontate byte per byte, non rilette: erano l'oggetto della
  revisione di prima.
- **Fuori perimetro — il punto 4 della consegna del metodo**, che la cura non tocca: prova contro i blocchi che restano solo
  un ticket che cambia `CLAUDE.md` o il compendio; ma i blocchi E2…E6 toccano ventidue file — `grep -h '^@@ '` sui blocchi,
  `sort -u`: undici ADR, `README.md`, il compendio, `roadmap.md`, `tracciabilita.md`, design/09, design/10, quattro disegni
  e la spec del sotto-progetto 1 —, e `CLAUDE.md` non è fra loro. Un ticket che tocca uno degli altri ventuno non è tenuto
  al `--check`; la rete resta il punto 2 del *«Come si riprende»* del piano, alla sessione dopo.
- **Fuori perimetro, già noto:** `pointer6.txt` così com'è toglie il link alla consegna del metodo — nel clone, dopo il
  compito 6, `grep -c '2026-09-30-metodo-decision-map-design' docs/COMPENDIO.md` → `0` —: è ciò che ER-8 dice, e chi esegue il
  compito 6 lo deve applicare. Non è un difetto della cura.
