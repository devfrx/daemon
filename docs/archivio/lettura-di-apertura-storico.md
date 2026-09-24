# Archivio — i testi usciti dalla lettura d'apertura, dal 2026-09-09

⛔ **Non è una lettura obbligatoria.** Verbali datati: i testi che il **mandato del proprietario** — decisione 26 della
[stella polare della GUI](../superpowers/specs/2026-09-07-direzione-gui-design.md), 2026-09-09: sfoltire la lettura
d'apertura, ogni taglio in A/B — ha fatto uscire dai documenti d'ingresso, `CLAUDE.md` e il messaggio di
[`AVVIO-CHAT.md`](../AVVIO-CHAT.md), **parola per parola**, con la data e il taglio che li ha prodotti. Ciò che esce dalla
§6 del compendio sta in [`stato-storico.md`](stato-storico.md), come dal 2026-08-28; la cronaca della stella polare in
[`consegna-brainstorming-direzione-gui.md`](consegna-brainstorming-direzione-gui.md). ✅ **Richiamo del 2026-09-23:**
qui anche la cronaca delle correzioni uscita dalle **altre** sezioni del compendio, col mandato del ridimensionamento
della lettura — la sezione in coda.

⚠️ **Ciò che è scritto qui era vero il giorno in cui fu scritto.** La lettura d'apertura viva la dice
[`CLAUDE.md`](../../CLAUDE.md); il prossimo passo sta nella §6 di [`COMPENDIO.md`](../COMPENDIO.md), in un posto solo.

## Taglio 2 — la testa dell'audit del 2026-08-27 · approvato A il 2026-09-09

📌 **La misura del giorno, coi comandi che la rifanno.** La testa fino a «Dettaglio» —
`awk '/^## Dettaglio/{exit} {b+=length($0)+1} END{print b}' docs/audit-2026-08-27.md` — pesava **82 907 byte**
(27 475 token `cl100k`, limite inferiore), di cui **36 121** «Stato dei rimedi» e **19 591** «I 73 finding» (il comando per
blocco sta nel punto 1 del prossimo passo della stella polare); ciò che resta da leggere all'apertura — le due sezioni
nominate in `CLAUDE.md` — lo dà
`awk '/^### Le voci aperte che NON/||/^### La disciplina/{p=1} /^### Le decisioni prese/||/^### Le trappole/{p=0} p{b+=length($0)+1} END{print b}' docs/audit-2026-08-27.md`,
che il giorno del taglio rendeva **4 806**.

### `CLAUDE.md`, «Prima cosa, e unica lettura obbligatoria» — il capoverso sull'audit com'era

⛔ **E la testa di [`docs/audit-2026-08-27.md`](../audit-2026-08-27.md) — tutto ciò che
sta PRIMA della sezione «Dettaglio».** ⚠️ **RICHIAMO DEL 2026-08-28: queste righe dicevano
*«Quel rapporto È APERTO ED È IL COMPITO DI OGGI, non una consultazione»*, e i suoi finding
sono chiusi.** Resta lettura d'apertura come **verbale e delega** — la sua sezione *«Come si
concludono quelli aperti»* è **il metodo con cui qui si rimedia**, e la colonna *«Stato»*
della sua tabella è la **casa unica** di che cosa resta, insieme alla tabella delle **voci
senza numero AUD**, che sono le sole ancora aperte e in gran parte del proprietario.
⛔ **Il «Dettaglio» dopo quella tabella NON si legge intero: se ne apre UNA scheda per
volta, quella del finding su cui si lavora.**

### `docs/AVVIO-CHAT.md`, il messaggio da incollare, voce 3 — com'era

```
  3. docs/audit-2026-08-27.md — ⛔ SOLO FINO ALLA TABELLA DEI 73 FINDING,
     cioè tutto cio' che sta PRIMA della sezione «Dettaglio».
     ⚠️ RICHIAMO DEL 2026-09-01: qui stava «54 KB», e sono 81 — la testa è
     cresciuta a ogni rimedio, perché è lì che vive la tabella dei rimedi.
     TOLTO e non riallineato: un peso misurato lo dà il comando, che non
     marcisce — awk '/^## Dettaglio/{exit} {b+=length($0)+1} END{print b}'
     docs/audit-2026-08-27.md
     ⚠️ I suoi finding sono CHIUSI dal 2026-08-28 — questa riga diceva «È IL
     COMPITO DI OGGI, non un verbale». Si legge come VERBALE e DELEGA: il
     metodo con cui qui si rimedia, e le voci senza numero AUD, che sono le
     sole ancora aperte.
     Contiene: come è stato condotto, la copertura, la baseline, le SETTE
     radici, lo stato dei rimedi, la sezione «Come si concludono quelli
     aperti» — che è LA DELEGA, con la disciplina in cinque passi, le due
     trappole del repository e l'ordine consigliato — e la tabella dei 73
     con la colonna «Stato», che è la loro CASA UNICA.
     ⛔ IL «Dettaglio» DOPO QUELLA TABELLA NON SI LEGGE INTERO: sono 73
     schede da ~20 righe, 204 KB, e se ne apre UNA per volta, quella del
     finding su cui stai lavorando. Si legge a FINDING, come un piano si
     legge a compiti.
```

## Taglio 5 — i verbali dentro `CLAUDE.md` · approvato A il 2026-09-09

⛔ **Il taglio 5 del mandato (decisione 26), approvato A il 2026-09-09, tredicesima ripresa (decisione 31 della
[stella polare della GUI](../superpowers/specs/2026-09-07-direzione-gui-design.md)):** i verbali dentro la testa e
dentro le celle delle tabelle di `CLAUDE.md` escono; ogni regola resta, col suo perché in una riga e il rimando «il
verbale in archivio». Misurato prima di chiedere: 2 173 byte e 753 token `cl100k` su 15 728 byte e 5 019 token (LF),
meno dell'1% della lettura d'apertura — contro la stima *«a ~10 KB, ~2 000 token»* della dodicesima ripresa, e la
divergenza è stata detta al proprietario prima della domanda. I pezzi com'erano al commit `b0901e4`, **parola per
parola**, coi soli link riscritti per questa cartella, e per ciascuno dove stava:

### In testa — le sei volte del gotcha #31, dentro la riga sul costo della lettura

Stava fra *«ed è una decisione.»* e *«Ora lo dice il comando, che non marcisce:»*.

Ogni volta che
un peso o un conto di token è stato scritto su questa riga è invecchiato: ha detto
«seimila token» a 24 KB, «88 KB» a 91, «165» a 192, «624» a 213. **Sei volte, il gotcha
**#31**.**

### In testa — la riga su `docs/audit-2026-08-11.md`

Stava fra il comando `tiktoken` e il capoverso *«Non aprire»*.

⚠️ **E [`docs/audit-2026-08-11.md`](../audit-2026-08-11.md) è uscito da questa riga il
2026-08-27:** è chiuso otto decisioni su otto, e ciò che insegnava vive nel compendio —
un file chiuso non è una lettura obbligatoria.

### La riga `superpowers:writing-plans` della tabella delle skill — il richiamo del 2026-08-30

Stava fra *«quando si scriverà il piano.»* e *«**si SANNO prima di scrivere**»*.

⛔ **RICHIAMO DEL 2026-08-30: qui stava *«non prima che le voci aperte siano chiuse»*, ed era INSODDISFACIBILE** — fra le voci aperte ce ne sono con chiusore *«il traguardo della ritenzione»* o *«nessuno finché nessuna misura lo chiede»*, quindi nessun piano avrebbe più potuto essere scritto. **E la pratica la violava da due piani su due:** le voci raccolte il 2026-08-10 *«perché chi riprende deve saperle PRIMA di scrivere»* erano aperte quando furono scritti i piani del **Traguardo 4** (2026-08-11) e del **Traguardo 5** (2026-08-18), e lo sono ancora. La regola vera è quella che la raccolta stessa enuncia:

### La riga dei fine-riga di «Come si lavora qui» — le tre volte e il `sed -i`

Stava dopo *«che nessuno ha toccato»*.

— successo **tre volte**, l'ultima il 2026-08-18 con un `sed -i` su `crates/kernel/Cargo.toml`, **43 CR → 0**.

### La riga della dipendenza in due passi — la relazione «tutti»

Stava dopo *«a **tutti** i suoi siti `cargo`»*.

— *tutti* è la relazione che regge quando il cancello guadagna un passo, una cifra no; il comando che la verifica sta in [`docs/riferimenti.md`](../riferimenti.md), in una casa sola —

### La riga della dipendenza in due passi — «il punto e non il prezzo»

Stava prima di *«Finding **G-5**»*.

È il punto e non il prezzo: ADR-0031 chiama l'aggiunta di una voce *«un atto deliberato e rivedibile»*, e un lockfile che il cancello aggiornava da sé non era né l'uno né l'altro.

### Sotto le quattro domande del pre-controllo — il capoverso del 2026-08-28

Stava dopo la tabella delle righe 5–8, prima di «Manutenzione della documentazione».

⚠️ **Il testo lungo di queste quattro righe è stato tolto da qui il 2026-08-28**, non
perso: erano quattro paragrafi che ricopiavano i gotcha **#49**, **#58**, **#59** e **#65**
di [`docs/HANDOFF.md`](../HANDOFF.md), che ne è la casa. ⛔ **Ciò che è rimasto è
l'ISTRUZIONE**, perché questo file è l'unica lettura obbligatoria e `HANDOFF.md` non lo è:
una regola che sparisse di qui non verrebbe letta da nessuno. **Il caso** invece si apre
solo quando serve, ed è per questo che può vivere altrove.

### La riga «Uno schema è una verifica» — la prima applicazione

Stava in coda alla cella, dopo *«mai in silenzio»*.

La prima applicazione, la sezione 1 della passata sui diagrammi, ha trovato due difetti in nove righe

## Il taglio del ridimensionamento — la cronaca delle correzioni del compendio · approvato A il 2026-09-23

⛔ **Il taglio 2 del mandato del proprietario del 2026-09-23** — ridimensionare la lettura, con la [consegna](../superpowers/specs/2026-09-23-ridimensionamento-lettura-design.md): i richiami che raccontano una correzione escono dal [compendio](../COMPENDIO.md), fuori dalla §6 e dalla sua tabella delle voci aperte, che sono decisioni del proprietario e restano com'erano. Dove il testo portava anche un fatto vero oggi, il compendio lo tiene senza la cronaca. Misurato col comando della consegna: da 33695 a 31792 token `cl100k`. I pezzi com'erano, **parola per parola**, coi soli link riscritti per questa cartella, e per ciascuno dove stava:

### §4, la riga di `bincode` — il primo richiamo

Stava fra *«non una vulnerabilità.»* e *«L'avviso è ancora attivo»*.

⛔ **RICHIAMO DEL 2026-08-31: qui stava *«registrato il 2026-08-18, si decide al Traguardo 6»*, e il Traguardo 6 ha MISURATO.** 

### §4, la riga di `bincode` — il secondo richiamo

Stava fra *«nella sua tabella dell'audit»* e *«. Le ragioni in»*.

 (richiamo del 2026-09-22, E231 del [piano della parte 2](../superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md): qui stava *«che resta aperta»*, falso dal suo compito 16)

### §4, sotto la convenzione di nomenclatura — perché sta qui

Stava fra la tabella della convenzione e *«Eccezione, e non è un'incoerenza»*.

⚠️ **Perché sta qui, e perché prima non c'era.** La §1.0 è una **sezione di spec**, non un
ADR: `check-docs.sh` pretende una voce di §5 per ogni file in `docs/adr/`, quindi nessun
controllo ne pretendeva la presenza. Un agente ha letto per intero **entrambi** i file
obbligatori e ha scritto un traguardo intero con gli identificatori in italiano. Gotcha
**#40**.

### §5, ADR-0021 — il richiamo del finding A-2

Stava fra *«sarebbe una falsa sicurezza.»* e *«Rimando: ADR-0034»*.

 ⚠️ **Richiamo del 2026-08-18, finding A-2:** questa riga
diceva *«e il seme diventa una regressione permanente»*, formulazione **già falsificata in
ADR-0021 il 2026-08-08** e sopravvissuta intatta qui e in
[`design/08`](../design/08-strategia-di-test.md) — che si dichiara *fonte di verità sulla porta
di qualità*. È la radice **R1**: una correzione attraversa il documento in cui nasce, non gli
altri.

### §5, ADR-0029 — la voce com'era, quando la decisione era aperta

Era la voce intera di ADR-0029; al suo posto, la decisione com'è oggi.

**0029 — ⚠️ Guscio della GUI: DECISIONE APERTA.** `Proposed`. Raccomandazione
**Electron**, ma sono **argomenti, non misure**, ed è per questo che resta aperta. Si
chiude con **M1–M5** all'inizio del sotto-progetto 2: RAM a riposo e sotto streaming ·
dimensione del pacchetto · fps del viewer 3D e API grafica reale **su Windows e
Linux** · P3 con rendering vero · **M5**, VRAM a riposo e sotto carico 3D (aggiunta da
ADR-0033). Se M3 mostra la stessa API grafica su entrambe le piattaforme con Tauri, la
decisione si **ribalta**. ✅ **Non blocca il sotto-progetto 1**, che è interamente Rust
e non tocca la GUI. ✅ **RICHIAMO DEL 2026-09-10: CHIUSA — Electron**, deciso dal proprietario con M1–M5 e Q1–Q4 misurate da SP-8 su Windows, coi criteri congelati prima; l'innesco Linux nell'ADR; `dockview` resta dopo le otto mosse. Il testo sopra resta com'era.

### §5, ADR-0032 — la riga che diceva `simulator`

Stava dopo *«**l'iniezione di livello 2**.»*; al suo posto, il fatto senza la cronaca.

⛔ **Il cadente vive in `platform` e NON in `simulator`, e questa
riga diceva `simulator` fino al 2026-08-11**, come la tabella dell'ADR da cui è compressa:
`redb` non ha `no_std`, i sei metodi di `StorageBackend` restituiscono `std::io::Error`, e il
grafo spedito di `simulator` lo rifiuterebbe come **«I3 violated»** — la cui unica cura scritta
è *togliere la dipendenza*. Non è una decisione riaperta: era una **previsione** scritta quando
`crates/simulator/` non esisteva. Rimando datato in ADR-0032, e la diagnosi è che i **due
livelli di crash erano trattati come una cosa sola** mentre hanno soggetti diversi.

### §5, ADR-0035 — le famiglie di porte

Stava dopo *«che copre **avvio, dialogo e uccisione** —»*; al suo posto, il conto di oggi.

non nasce una porta nuova, le famiglie restano sei. ⚠️ **RICHIAMO DEL 2026-09-17:** sono
**sette** dal compito 4 del sotto-progetto 2 — `custody` — e il merito di questa decisione resta
intatto: il dialogo col worker **non** ha aperto una porta nuova (rimando datato in testa
all'ADR).

### §8, la riga delle misure — il richiamo E236

Stava in coda alla cella, dopo *«entrambe scritte in ADR-0029»*.

 (richiamo del 2026-09-22, E236 del [piano della parte 2](../superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md))

### §9 — la seconda copia dei gotcha

Erano i due capoversi in coda alla §9; al loro posto, la regola in una riga.

⚠️ **Questa sezione ne portava una SECONDA copia, tolta il 2026-08-28.** Non era una
sintesi: erano gli stessi 82 numeri, per **31 578** token — mentre la sua stessa seconda
riga dichiarava che il testo completo stava in `HANDOFF.md`. È il gotcha **#68** — *un
puntatore che vive in più documenti si toglie, non si ricorregge* — commesso dentro il
documento che quella regola la contiene, ed è la radice **R3** dell'audit del 2026-08-27.

⛔ **E le due copie erano DIVERSE, il che è il costo vero di una seconda casa.** Confrontate
riga per riga prima di togliere: gli 82 numeri coincidevano, ma **quattro** righe della §9
erano più lunghe, e `HANDOFF.md` portava verbali di correzione che la §9 non aveva. La §9
era una **biforcazione ferma**. Le **due** clausole che diceva in più — sulle righe **59** e
**61** — sono state **spostate in `HANDOFF.md`** prima della cancellazione, col richiamo
datato. 📌 **Un duplicato non resta identico: diverge, e nessuno dei due lati lo sa.**

### §10 — il titolo che diceva «Le cinque trappole»

Stava fra *«Da sapere **prima** di scrivere, non dopo il rosso.»* e la tabella.

⚠️ **RICHIAMO DEL 2026-08-28: il titolo diceva *«Le cinque trappole»*.** Il numerale è **tolto e
non riallineato a sei** — è una popolazione che cresce a ogni trappola misurata, e la tabella
qui sotto è la sua casa unica. Gotcha **#68**.

### §11 — il richiamo del finding AUD-007

Stava fra *«gotcha **#38**.»* e *«📌 **La regola: resta davanti solo ciò che la tabella qui sotto nomina»*.

⛔ **RICHIAMO DEL 2026-08-27, finding AUD-007 — questa riga diceva *«gli altri dieci restano
davanti, e chi li copre è scritto in `porta-di-qualita.md`»*, e le affermazioni false erano
DUE.** La prima era ferma alla chiusura del **Traguardo 1** e non è mai stata riletta:
`git log -L 2534,2538:docs/COMPENDIO.md` dà **una sola scrittura**, `cf2983f`, in un file la
cui intestazione si data al Traguardo 5 — e lo stesso file la smentisce in §5, dove i **byte
congelati** (vincolo 14) esistono dal 2026-08-10. La seconda mandava al registro per una
copertura che il registro **non tiene**: [`porta-di-qualita.md`](../porta-di-qualita.md) mappa le
righe di catalogo della **§7.4**, non i vincoli di questa sezione — e quanto poco vi si affacci
questa §11 lo dice `grep -c '§11' docs/porta-di-qualita.md` contro `grep -c '§7.4'` sullo stesso
file, che è un rapporto e non una cifra da tenere aggiornata.
⛔ **E il rimedio non è riallineare la cifra a un numero nuovo, che è la parte da ricordare:**
un numeratore che cresce a ogni traguardo è esattamente ciò che è marcito qui. Al suo posto c'è
una **regola di lettura**, che resta vera quando una riga se ne va — la stessa cura che la §6
ha usato per `M9`: *un elenco invecchia, una regola no*.

### §12, la riga dell'audit del 2026-08-27 — la riga aggiunta

Stava in coda alla cella, dopo *«⚠️ **Si legge a FINDING, mai intero.**»*.

 ⚠️ **Riga aggiunta il 2026-08-27:** mancava dalla tabella dal giorno in cui il file è nato, ed è la stessa specie di difetto che la 7ª e la 15ª misura registrarono — *per accorgersi di una riga ASSENTE bisogna partire dall'elenco dei file citati, non dalle righe presenti*

### §12, la riga dell'audit del 2026-08-11 — la cella corretta

Stava in coda alla cella, dopo *«o di specie diversa.»*.

 ⚠️ **Questa cella diceva *«COSA DEVI FARE ADESSO … ne restano tre … è il prossimo passo»***, corretta il 2026-08-18

### §12, la riga del piano del Traguardo 2 — il finding AUD-035

Stava in coda alla cella, dopo *«⚠️ **a compiti, mai intero**.»*.

 ⛔ **RICHIAMO DEL 2026-08-28, finding AUD-035:** la cella diceva *«è il secondo file più grande del repository, dopo la spec»*, ed è **tolta e non riallineata** — era **ottavo** quando il finding lo misurò il 2026-08-27 e **decimo** un giorno dopo, e un ordinamento marcisce come una cifra. Lo rifà il comando sotto questa tabella

### §12, la riga di `AVVIO-CHAT.md` — il peso che invecchiò

Stava dopo *«non questa cella:»*; al suo posto, il solo metodo.

⛔ **RICHIAMO DEL 2026-08-28** — diceva *«**20606 byte LF** su **303** righe»*, ed è invecchiato lo stesso giorno, quando la riga 3 del messaggio ha smesso di dire che l'audit era il compito di oggi. Ciò che **resta** qui è il **metodo** — le righe **fra le due recinzioni, escluse** — senza il quale due lettori onesti ottengono due numeri (59ª misura).

### §13, la riga «gotcha nuovo» — il duplicato ordinato

Stava in coda alla cella, dopo *«invece di copiare.»*.

 ⚠️ **RICHIAMO DEL 2026-08-28:** questa riga diceva *«una riga in §9, e il testo integrale in `HANDOFF.md`»*, cioè ordinava di **ricreare il duplicato** che lo sfoltimento aveva tolto lo stesso giorno — gotcha **#68**, dentro la tabella che governa la manutenzione

## La revisione di `CLAUDE.md` · approvata A il 2026-09-23

⛔ **Il proprietario ha chiesto, il 2026-09-23, di rivedere `CLAUDE.md` per intero** — dentro il mandato del [ridimensionamento della lettura](../superpowers/specs/2026-09-23-ridimensionamento-lettura-design.md), rispondendo al taglio 4 sui diari dei piani — e ha approvato **A** la revisione presentata sezione per sezione: via la cronaca, un numerale sbagliato corretto, e le regole che vivevano solo nella memoria dell'agente di una macchina portate nel file che leggono tutte. Il file com'era al commit `f020cd9`, **parola per parola**, in citazione e coi soli link riscritti per questa cartella:

> # Istruzioni per l'agente
>
> ## ⛔ Prima cosa, e unica lettura obbligatoria
>
> Leggi **questo file** e poi **[`docs/COMPENDIO.md`](../COMPENDIO.md)**, per intero.
> Poi **fermati**.
>
> Il compendio contiene **tutte** le decisioni del progetto — le 39 ADR, le sei
> invarianti, lo stack, i gotcha, lo stato di oggi e il prossimo passo — ciascuna
> compressa a poche righe.
>
> ⛔ **E di [`docs/audit-2026-08-27.md`](../audit-2026-08-27.md) SOLO DUE PEZZI: la tabella
> *«Le voci aperte che NON hanno un numero AUD»* — le sole ancora aperte, in gran parte del
> proprietario — e *«La disciplina, in cinque passi»*, il metodo con cui qui si rimedia.** I 73
> finding sono **tutti chiusi** — quanti, lo dice il comando in fondo alla §6 del compendio — e il
> «Dettaglio» **NON si legge intero**: se ne apre **UNA** scheda per volta, quella del finding su cui
> si lavora. ✅ **RICHIAMO DEL 2026-09-09, decisione 26:** qui stava *«tutta la testa, fino a
> «Dettaglio»»*, in gran parte le due tabelle dei finding chiusi; il testo com'era è in
> [`docs/archivio/lettura-di-apertura-storico.md`](lettura-di-apertura-storico.md).
>
> ⚠️ **Quanto costa questa lettura NON è scritto qui, ed è una decisione.** Ogni peso scritto su
> questa riga è invecchiato — sei volte, gotcha **#31**; il verbale in
> [`docs/archivio/lettura-di-apertura-storico.md`](lettura-di-apertura-storico.md). Ora lo dice il comando, che non marcisce:
>
> ```bash
> wc -c CLAUDE.md docs/COMPENDIO.md
> ```
>
> 📌 **E in token, se serve** — `pip install tiktoken`; è il tokenizzatore di OpenAI, quindi
> su italiano con emoji il conto di Claude è più alto e questo è un **limite inferiore**:
>
> ```bash
> python -c "import tiktoken,io; e=tiktoken.get_encoding('cl100k_base'); print(sum(len(e.encode(io.open(p,encoding='utf-8').read())) for p in ['CLAUDE.md','docs/COMPENDIO.md']))"
> ```
>
> ⛔ **Non aprire** [`docs/HANDOFF.md`](../HANDOFF.md), la spec del sotto-progetto 1, la
> cartella [`docs/adr/`](../adr/) o [`docs/archivio/`](./) «per farsi
> un'idea». Insieme sono **di gran lunga** la mole maggiore del repository — il conto lo dà
> `find docs -name '*.md' | xargs wc -c | sort -n` — e l'idea è già nel compendio.
> Quando ti servirà il **perché** di una decisione — le alternative scartate, le misure, i
> costi accettati — apri **quel** file, uno solo. La §12 del compendio dice quale.
>
> ⚠️ **Il compendio è una compressione, non una selezione.** Ci sono dentro tutte le
> decisioni, non quelle pertinenti al compito di oggi. Sparisce il ragionamento lungo,
> non la decisione: nessuna può sfuggirti perché «non sembrava attinente».
>
> ## Cos'è questo progetto, in quattro righe
>
> Assistente desktop locale, utente singolo, Windows primario poi Linux, **GPU singola
> RTX 5080 da 16 GB**, OpenRouter primario con inferenza locale opzionale.
> **Piattaforma a quattro pilastri paritari** — conversazione e conoscenza, agenti e
> coding, voce e gesti, generazione asset 3D — su un **kernel comune** (ADR-0001, col rimando datato in testa).
>
> Il vincolo dominante non è funzionale ma **di risorsa**. Il kernel **non implementa
> nessuna funzionalità utente**: fornisce i meccanismi.
>
> ⚠️ **Questo non è un repository di sola documentazione.** Il codice del prodotto si
> scrive **qui**, e vive in [`crates/`](../../crates/): cinque crate, con `kernel` e `simulator`
> in `no_std` — e, dal sotto-progetto 2, in [`gui/`](../../gui/): la SPA e il core finto, fuori dal workspace e
> provati da `scripts/gate-gui.sh` (richiamo del 2026-09-22). Gli spike in [`spikes/`](../../spikes/) restano
> **prove**, fuori dal workspace.
> La porta di qualità si lancia con un comando solo — `bash scripts/gate.sh` — e la mappa
> dei controlli è in [`docs/porta-di-qualita.md`](../porta-di-qualita.md).
> Lo stato corrente e il prossimo passo stanno nella **§6 del compendio** — non qui, o si
> disallineano.
>
> ## Skill da invocare, in questo repository
>
> Vanno invocate **prima** di qualsiasi risposta o esplorazione, non dopo.
>
> | Skill | Perché qui |
> |---|---|
> | `superpowers:using-superpowers` | è il preambolo: se una skill può applicarsi, si invoca |
> | `anthropic-skills:dev-discipline` | governa il **codice**: esplora prima di scrivere, YAGNI, convenzioni del repo, niente scorciatoie non dichiarate |
> | `anthropic-skills:dev-communication` | governa la **conversazione** intorno al codice: cosa si decide da soli e cosa si porta al proprietario |
> | `superpowers:brainstorming` | prima di qualunque lavoro creativo, e **prima di entrare in plan mode** |
> | `superpowers:writing-plans` | quando si scriverà il piano. Le voci aperte non si aspettano chiuse (richiamo del 2026-08-30: la regola vecchia, *«non prima che le voci aperte siano chiuse»*, era insoddisfacibile, perché alcune voci hanno chiusore *«il traguardo della ritenzione»* o *«nessuno»* — il verbale in archivio): **si SANNO prima di scrivere**. A **sbarrare** è la colonna *«Chi la chiude»* di [`docs/porta-di-qualita.md`](../porta-di-qualita.md): una voce il cui chiusore è **questo traguardo** o **il proprietario, prima** va chiusa o portata dal piano; le altre si conoscono e si dichiarano |
> | `superpowers:subagent-driven-development` | per **eseguire** un piano: un subagente fresco per compito, con revisione fra uno e l'altro. È la modalità scelta dal proprietario |
> | `superpowers:test-driven-development` | quando comincerà il codice |
>
> ## Come si lavora qui
>
> | Regola | |
> |---|---|
> | **Spec prima del codice** | nessun sotto-progetto si implementa senza spec approvata |
> | ⛔ **Codice in inglese, documentazione in italiano** | **§1.0 della spec.** Crate, moduli, tipi, funzioni, messaggi d'uscita e commenti nel sorgente sono **in inglese**; i documenti restano **in italiano**; un riferimento al codice dentro un documento si scrive **in inglese, col nome esatto del sorgente**. ⚠️ Non è tipografia: la regola non stava né qui né nel compendio, e un traguardo intero è stato scritto con gli identificatori italiani e poi rifatto — gotcha **#40** |
> | **Sezione per sezione** | si presenta, si discute, **si approva**, si scrive. Mai tutto insieme |
> | **Decidere sul merito** | né scorciatoie né sovra-ingegnerizzazione. «Non pigro» **non** significa «più costoso» |
> | **Rendere verificabile** | un principio che non si può controllare è un'intenzione. Gli invarianti diventano test |
> | **Un'evidenza scritta prima della misura è un'ipotesi** | si misura, e dove diverge **si registra la divergenza** invece di allinearsi all'attesa |
> | **Un controllo si prova in due direzioni** | che scatti dove deve, **e che non scatti dove non deve**. La seconda si dimentica |
> | **Schema-first** | tabelle, diagrammi, elenchi numerati. Niente muri di testo |
> | **Ma prima a parole** | quando l'argomento esce dal dominio del proprietario (non è operativo in Rust), si spiega **prima** a parole semplici e **poi** si schematizza |
> | **Stato dell'arte verificato** | se una nozione non è certa si cerca **prima** di scrivere, e la fonte si traccia in [`docs/riferimenti.md`](../riferimenti.md). **Mai inventare** |
> | ⛔ **Uno schema è una verifica, e corregge ciò che esiste** | **decisione del proprietario del 2026-09-08, e vale per ogni studio, brainstorming o diagramma futuro:** prima dell'A/B si controllano esplicitamente tre cose — che cosa **esiste già** (codice, ADR, disegni), che cosa **arriva** (la roadmap) e se **regge crescendo** — e si dicono a parole. Se lo schema è più corretto di una logica, di un ADR o del codice, si correggono **quelli**: l'ADR col richiamo datato, il codice come compito del piano, sempre in forma A/B, mai in silenzio |
> | **Dichiarare i costi** | ogni decisione elenca ciò che peggiora. Un ADR senza `Negative (accettate)` è incompleto |
> | **Un'idea nuova può essere già stata scartata** | prima di proporre qualcosa che **sostituisce** una decisione presa, si cerca **dove era già stata valutata e perché era caduta**. Si riapre **solo con una prova nuova**; e se la prova nuova gioca contro, si **registra e si chiude**. Vale anche — soprattutto — per le proprie idee |
> | **ADR append-only** | superato → `Superseded by`; completato → un **rimando**. Completare una riga di verifica **non** è superare l'ADR |
> | **Richiamo datato** | ogni correzione a una sezione approvata porta il proprio richiamo con la data |
> | ⛔ **Un puntatore o una cifra che vive in PIÙ documenti si TOGLIE, non si ricorregge** | riallinearlo lo rimette nello stato in cui la **regola** è di nuovo l'unica difesa, e quella regola non ha retto **tre volte**. I documenti secondari **rimandano** alla §6 del compendio invece di riscriverla: un rimando non può marcire. ⚠️ Lo stato **per traguardo** resta nelle tabelle di [`docs/roadmap.md`](../roadmap.md) e [`docs/README.md`](../README.md) — il perimetro di una passata si prende dal drift **misurato**, non dalla categoria. Gotcha **#68** |
> | ⛔ **Un numero misurato non si scrive: si scrive il COMANDO che lo produce** | un numero invecchia al primo commit che tocca ciò che misura; un comando no. Se il numero deve comparire — perché sostiene una decisione — porta accanto il **comando** e la **data**, e vive in **una** casa sola. ⚠️ **Misurato il 2026-08-28:** il **92%** della lettura obbligatoria era storia di numeri corretti, e la testa di questo file aveva sbagliato il proprio peso **sei volte**. Gotcha **#31** |
> | ⛔ **Un verbale di correzione non resta nel documento corretto** | va in [`docs/archivio/`](./), con la data; il documento vivo porta ciò che è **vero adesso**. È la metà mancante della riga qui sopra: senza di essa la regola dice *dove* mettere il numero e non *dove* mettere la sua storia, e la storia si accumula esattamente dove la si è corretta. ⛔ **Il freno è nel cancello**, non nella buona volontà: `check-docs.sh` respinge un compendio sopra il proprio tetto |
> | **Le misure nello scratchpad** | non nel repository, e si ripulisce dopo |
> | ⛔ **I fine-riga sono misti _per file_** | non c'è una convenzione da seguire: c'è **un file da non cambiare**. Uno script che riscrive un sorgente ne normalizza i fine-riga senza dirlo, e `git diff` dichiara **seicento righe cambiate** che nessuno ha toccato — successo **tre volte** (il verbale in archivio). Chi scrive uno strumento che tocca file **conserva i fine-riga di quel file**, e li **rimisura dopo** con `tr -cd '\r' \| wc -c` invece di fidarsi |
> | ⛔ **Una dipendenza si aggiunge in _due_ passi** | dal 2026-08-18 il cancello passa `--locked` a **tutti** i suoi siti `cargo` (il comando che lo verifica sta in [`docs/riferimenti.md`](../riferimenti.md)) — quindi il `Cargo.lock` è un **ingresso** e non più un effetto. Toccare un manifesto da solo lascia il cancello **rosso**: il lockfile si rinfresca **fuori** dal cancello — un `cargo build` senza il flag — e si committa **insieme** al manifesto. Il perché: ADR-0031 vuole che aggiungere una voce sia *«un atto deliberato e rivedibile»*; il verbale in archivio. Finding **G-5** |
> | **Audit a ogni chiusura** | `bash scripts/check-docs.sh` prima di ogni commit di documentazione |
> | **Commit e push** | alla chiusura di ogni voce si **committa e si pusha**, senza chiedere, e **senza co-autore** |
>
> ## Prima di eseguire un compito di un piano
>
> ⛔ **Un piano è un'ipotesi, e il pre-controllo di ogni compito _prima_ di dispacciarlo ha
> trovato almeno un difetto reale in TUTTI i compiti dispacciati finora, senza una sola
> eccezione.** ⚠️ **Nessun numeratore qui**, per costruzione: un cumulativo invecchia a ogni
> compito, *«tutti, senza eccezione»* no.
>
> Si fanno **quattro domande**, e **ciascuna coglie ciò che le altre tre non colgono**.
>
> | | Il difetto | Che cosa lo coglie |
> |---|---|---|
> | 1 | la **sonda è sbagliata** — vacua, o attacca il caso invece del meccanismo | **rileggere** |
> | 2 | la **sonda manca** | *per ogni artefatto che il compito produce, quale controllo lo esercita?* Non si vede leggendo: non c'è niente da leggere |
> | 3 | l'**artefatto è sbagliato**, e compila | **solo** scriverne un'implementazione **da fuori dalla crate** |
> | 4 | il **compito è già eseguito** | *ciò che detta di produrre esiste già?* |
>
> ⛔ **E tre cose che l'elenco NON coglie, una riga l'una.** Sono istruzioni, non aneddoti:
> il **caso** che ciascuna ha prodotto vive in [`docs/HANDOFF.md`](../HANDOFF.md), nel
> gotcha che porta il suo numero, e si apre solo se serve.
>
> | | La regola | Il caso |
> |---|---|---|
> | 5 | ⛔ **Il contratto cresce sotto il piano: un compito scritto prima si legge contro il codice di ADESSO, non contro il piano** | Traguardo 3 — un compito dettava «le cinque operazioni» quando erano sei, uno congelava quattro campi quando erano cinque, uno attendeva un rosso che era verde |
> | 6 | ⛔ **Vale anche per un DISEGNO, e le guardie non sono tutto il codice: ciò che ti smentisce può stare in un BANCO DI PROVA, perfino in un commento** | gotcha **#58** |
> | 7 | ⛔ **Un ADR si legge anche contro i propri FRATELLI**, non solo contro il codice: due decisioni della stessa data possono contraddirsi senza che nessuna delle due nomini l'altra | gotcha **#59** |
> | 8 | ⛔ **Un RAPPORTO è un piano, e si prezza leggendo il CODICE — in ENTRAMBE le direzioni**: può chiedere più del necessario, e può chiedere meno | gotcha **#65** |
>
> ## Manutenzione della documentazione
>
> Alla chiusura di ogni sotto-progetto si aggiornano **nello stesso passaggio**:
> [`docs/COMPENDIO.md`](../COMPENDIO.md), [`docs/roadmap.md`](../roadmap.md),
> [`docs/README.md`](../README.md), [`docs/tracciabilita.md`](../tracciabilita.md),
> lo stato degli spike, [`docs/HANDOFF.md`](../HANDOFF.md) se emergono gotcha nuovi, e
> questo file se cambia il modo di lavorare.
>
> Alla chiusura di ogni **voce** — non solo di un sotto-progetto — si aggiornano
> [`docs/COMPENDIO.md`](../COMPENDIO.md) e [`docs/HANDOFF.md`](../HANDOFF.md), e
> [`docs/riferimenti.md`](../riferimenti.md) **se la voce ha portato una misura o una
> fonte**. Poi si committa e si pusha.
>
> ⛔ **Il compendio non può restare indietro**, e non è lasciato alla buona volontà:
> `check-docs.sh` pretende una voce in §5 per **ogni** file in `docs/adr/`. Un ADR nuovo
> senza voce è un **rosso**. Vedi §13 del compendio.
>
> Un documento di stato disallineato è peggio di nessun documento: **mente con
> autorevolezza**.

## La compressione del compendio · 2026-09-24

⛔ **Il mandato del proprietario del 2026-09-24** — comprimere senza perdite il [compendio](../COMPENDIO.md) con la skill `lean-docs`, fuori dalla §5 e dalla tabella delle voci aperte della §6, ogni sezione portata in A/B e decisa, su delega del proprietario, coi cinque criteri di `decision-principles` — ha riscritto i blocchi qui sotto. Stanno qui **parola per parola**, coi soli link riscritti per questa cartella; i numeri di riga sono quelli del compendio **prima** della passata, e quelli della §6 stanno in [`stato-storico.md`](stato-storico.md).

### la testa — righe 1–32

Righe 1–32: le quattro note in testa e la riga della data, coi suoi due capoversi.

# Compendio — l'architettura intera, in un colpo solo

> 🎯 **A cosa serve questo file.** È l'unica lettura obbligatoria all'inizio di una
> sessione. Contiene **tutte** le decisioni del progetto — non quelle che sembrano
> pertinenti al compito di oggi: **tutte** — ciascuna compressa a poche righe.
>
> ⚠️ **La differenza che conta.** Questo file non è una *selezione*, è una
> *compressione*. Leggere solo gli ADR «pertinenti» fa perdere le decisioni che danno
> forma al lavoro in modo indiretto, e il modo di fallire è che non sai nemmeno che
> esistono. Qui ci sono tutte: sparisce il **perché** lungo, non la **decisione**.
>
> 📖 **Come si usa.** Leggi questo file per intero. Poi, se e solo se ti serve il
> ragionamento dietro una decisione — le alternative scartate, le misure, i costi
> accettati — apri **quel** file. Uno, non tutti. La §12 dice quale.
>
> ⛔ **Cosa NON fare.** Non aprire `HANDOFF.md`, la spec del sotto-progetto 1, o la
> cartella `adr/` «per farsi un'idea». Insieme pesano **oltre mezzo megabyte**
> (812 KB in byte LF il 2026-08-27, e possono solo crescere — la spec da sola ne fa 298), e
> l'idea è già qui.

**Aggiornato il 2026-09-23**, col **ridimensionamento della lettura** eseguito — la cronaca delle correzioni fuori dalle sezioni che non sono la §6, in [`archivio/lettura-di-apertura-storico.md`](lettura-di-apertura-storico.md) — e il puntatore della §6 al design system — disegno riletto, piano in scrittura — e al 13; l'ultimo contenuto di merito è la voce di ADR-0029, riscritta allo stato di oggi. Il testo com'era, con le sedici riprese, è in [`archivio/stato-storico.md`](stato-storico.md). Manutenzione: §13.
⚠️ **Questa riga ha sbagliato due volte su due, e la seconda è il finding AUD-034.** Diceva
**2026-08-11** dopo decine di passate; poi **2026-08-25**, mentre `f2bc784` — un'ondata di
correzione — l'aveva riscritto nel merito il **2026-08-26**. È il gotcha **#31** sull'intestazione,
che nessuno rilegge perché è la cornice e non il contenuto: la stessa forma trovata **quattro**
volte su [`HANDOFF.md`](../HANDOFF.md).
⛔ **E la causa è strutturale, REGISTRATA E NON PRESA perché tocca il modo di lavorare:** la §13
aggancia la manutenzione a *«ADR nuovo, ADR superato, voce chiusa, gotcha nuovo, misura nuova,
decisione dello stack, cambio del prossimo passo»*, e **nessuno** di quei ganci copre una
**riscrittura di merito dentro una riga che c'è già** — quindi nulla richiama la data, e infatti
l'ha riportata qui il rimedio di un audit e non la manutenzione. Aggiungere un gancio cambia la
§13, ed è del **proprietario**.

### §1, il ritratto — righe 38–54

Il corpo della §1, sotto il titolo.

Assistente desktop locale, utente singolo, Windows primario poi Linux, **GPU singola
RTX 5080 da 16 GB**, OpenRouter primario con inferenza locale opzionale.

**Piattaforma a quattro pilastri paritari** — conversazione e conoscenza, agenti e
coding, voce e gesti, generazione asset 3D — su un **kernel comune**. Nessun pilastro prevale,
nessuno ha accesso privilegiato al kernel.

Il vincolo dominante **non è funzionale ma di risorsa**: quattro aree che si contendono
una sola GPU da 16 GB.

**Il kernel non implementa nessuna funzionalità utente: fornisce i meccanismi.**

⚠️ **Questo non è un repository di sola documentazione.** Il codice del prodotto si
scrive qui, e vive in [`../crates/`](../../crates/): cinque crate, con `kernel` e `simulator`
in `no_std` — e, dal sotto-progetto 2, in [`../gui/`](../../gui/): la SPA e il core finto `gui/fake-core/`, fuori dal
workspace e provati da `scripts/gate-gui.sh` (richiamo del 2026-09-22, E232 del [piano della parte 2](../superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md)). Gli spike in `spikes/` restano
**prove**, fuori dal workspace.

### §10, la trappola 6 — righe 819

La riga intera.

| **6** | ⛔ **Il controllo dei link NON verifica i FRAMMENTI, e un'ancora pura è INVISIBILE — misurato il 2026-08-28.** Il passo estrae con `grep -o '](../\([^)#]*\.md\)[^)]*)'` e poi taglia con `cut -d'#' -f1`, quindi un rimando *«parentesi-quadra-chiusa, tonda, `file.md`, cancelletto, ancora»* è controllato **solo** per la parte `file.md`, e la forma **senza file** — solo `#ancora`, un rimando dentro lo stesso documento — **non viene nemmeno estratta**. ✅ **Provato sulla pipeline vera**, tre casi in un file temporaneo: quello con un file esistente e quello con file **più** ancora inventata escono **entrambi senza il frammento**; quello con la sola ancora **non esce affatto**. ⚠️ **E gli esempi qui sopra sono scritti a parole per FORZA:** scritti nella loro sintassi vera facevano **rosso il cancello** — `broken link: docs/COMPENDIO.md -> vero.md` — perché il controllo **non distingue un esempio da un rimando**, che è il cugino della trappola **5**. 📌 **Quindi un'ancora è un rimando che nessun controllo difende**, e marcisce in silenzio quando un titolo cambia: una sezione si **nomina** invece di collegarla, oppure si accetta il rischio **sapendolo**. Trovata scrivendone una chiudendo AUD-013, e tolta prima del commit |

### §12, la tabella e la sua coda — righe 863–929

Dal primo capoverso della §12 al suo ultimo, prima del filetto.

Apri **un** file, quello che serve. Non la cartella.

| Se ti serve… | Apri |
|---|---|
| ⛔ **il verbale del SECONDO audit completo, e la sua DELEGA** — i 73 finding con causa radice, riproduzione e stato, le sette radici, e la sezione *«Come si concludono quelli aperti»*, che è la **ricetta**: lo stato alla consegna, ciò che NON è verificato, la disciplina in cinque passi e l'ordine consigliato. ⛔ **La colonna «Stato» di quel rapporto è la CASA UNICA di quali finding siano chiusi** — non si ricopia altrove. ⚠️ **Si legge a FINDING, mai intero.** | [`audit-2026-08-27.md`](../audit-2026-08-27.md) — ⚠️ **a finding, mai intero** |
| il **verbale del primo audit completo** — le quattro radici, i finding con causa radice e dimostrazione, ciò che è stato verificato **pulito**, e la §8 con le otto decisioni, **tutte eseguite** fra il 2026-08-17 e il 2026-08-18. ⛔ **Si apre per il METODO, non per il compito:** è il posto in cui si legge come un rimedio si prezza leggendo il codice invece del rapporto — più piccolo, più grande, o di specie diversa. | [`audit-2026-08-11.md`](../audit-2026-08-11.md) — oggi una **consultazione** |
| il **perché** di una decisione, le alternative scartate, i costi accettati | `docs/adr/<numero>-*.md` — **uno solo** |
| il **come** del sotto-progetto 1: §0–§8 con le evidenze delle misure | [`specs/2026-08-06-sottoprogetto-1-kernel.md`](../superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md) — ⚠️ **a sezioni, mai intera** |
| ⛔ **il perimetro del Traguardo 5** — l'arbitro: quanto ne costruisce, le forme che la §5 descrive a parole, e per ogni artefatto **il controllo che lo esercita**. ⛔ **Si legge PRIMA di scriverne il piano**, ed è il file da cui si riprende | [`specs/2026-08-18-…-traguardo-5-arbitro-gpu-design.md`](../superpowers/specs/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu-design.md) — ⚠️ **non è una spec**: è lo scaglionamento e le forme che la §5 non fissa |
| ⛔ **come si ESEGUE il Traguardo 5** — tredici compiti in cinque parti, col codice per ogni passo, le mutazioni da provare e i comandi. ⚠️ **L'errata in testa si legge PRIMA del compito**, e il pre-controllo del piano — le sette voci — sta subito sotto | [`plans/2026-08-18-…-traguardo-5-arbitro-gpu.md`](../superpowers/plans/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu.md) — ⚠️ **a compiti, mai intero** |
| ⛔ **il perimetro del Traguardo 6** — gli altri meccanismi, le forme, il controllo per artefatto, e la **§8** col verbale della sua chiusura | [`specs/2026-08-28-…-traguardo-6-altri-meccanismi-design.md`](../superpowers/specs/2026-08-28-sottoprogetto-1-traguardo-6-altri-meccanismi-design.md) |
| ⛔ **come si è ESEGUITO il Traguardo 6** — dieci compiti in cinque parti, con l'errata in testa | [`plans/2026-08-30-…-traguardo-6-altri-meccanismi.md`](../superpowers/plans/2026-08-30-sottoprogetto-1-traguardo-6-altri-meccanismi.md) — ⚠️ **a compiti, mai intero** |
| ⛔ **come si è CHIUSO il sotto-progetto 1** — le condizioni della §0.7 rilette contro il codice, e la **§7** col verbale | [`specs/2026-09-02-…-chiusura-design.md`](../superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md) |
| il piano della chiusura, con l'errata in testa e la tabella della posizione | [`plans/2026-09-02-…-chiusura.md`](../superpowers/plans/2026-09-02-sottoprogetto-1-chiusura.md) |
| ⛔ **il perimetro del RICONOSCIMENTO GESTI** — la forma della telecamera nel kernel, il registro delle funzioni, le decisioni col loro chiusore, e per ogni artefatto il controllo che lo esercita | [`specs/2026-09-03-riconoscimento-gesti-design.md`](../superpowers/specs/2026-09-03-riconoscimento-gesti-design.md) — ⚠️ **non è una spec** |
| come si è **eseguito** il riconoscimento gesti — i due ADR, i rimandi, la roadmap, SP-7 e la sonda S3, con l'errata in testa e la tabella della posizione | [`plans/2026-09-03-riconoscimento-gesti.md`](../superpowers/plans/2026-09-03-riconoscimento-gesti.md) — ⚠️ **a compiti, mai intero** |
| ⛔ **il perimetro della KNOWLEDGE BASE** — che cosa la mappa chiede al kernel e dove va: la strada B, i tre meccanismi del sotto-progetto 13 con le due pretese, le CRUD nel registro delle funzioni, il pannello col 6, le decisioni col loro chiusore, e per ogni artefatto il controllo che lo esercita | [`specs/2026-09-04-knowledge-base-design.md`](../superpowers/specs/2026-09-04-knowledge-base-design.md) — ⚠️ **non è una spec**, e **non disegna la capacità** |
| come si è **eseguito** il piano dei documenti della knowledge base — i rimandi in testa a quattro ADR e nella riga di ADR-0039, la riga 13 in roadmap, le righe di tracciabilità, la decisione 7 dei gesti chiusa, con l'errata in testa e la tabella della posizione | [`plans/2026-09-04-knowledge-base-documenti.md`](../superpowers/plans/2026-09-04-knowledge-base-documenti.md) — ⚠️ **a compiti, mai intero** |
| ⛔ **la direzione della GUI** — le viste, i moduli, la disposizione, il protocollo core ↔ GUI, e la tabella delle decisioni col loro chiusore | [`specs/2026-09-07-direzione-gui-design.md`](../superpowers/specs/2026-09-07-direzione-gui-design.md) — ⚠️ **non è una spec**: è la stella polare, e si legge **per intero** da chi riprende il fronte GUI. ⚠️ **Richiamo del 2026-09-23:** nel brainstorming del design system si legge **a pezzi**, e per intero prima di scriverne il disegno — decisione del proprietario, risposta 7 della [consegna](../superpowers/specs/2026-09-22-design-system-design.md) |
| ⛔ **il perimetro della GUI minima** — che cosa il 2 costruisce e che cosa no, il filo, lo schema, il registro, il core finto, le prove e il cancello, le decisioni aperte col chiusore | [`specs/2026-09-06-sottoprogetto-2-gui-minima-design.md`](../superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md) — ⚠️ **non è una spec**; la **§10** dice come si riprende |
| come si è **eseguita la parte 1** — SP-8, i due gusci misurati, ADR-0029 chiuso | [`plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md`](../superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md) — ⚠️ **a compiti, mai intero** |
| ⛔ **come si è ESEGUITA la parte 2** — il filo, la settima porta, il registro, il daemon, la SPA, il cancello web, X-1 e X-3; con l'errata in testa, la tabella della posizione e la **Definizione di «fatto»** | [`plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md`](../superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md) — ⚠️ **a compiti, mai intero** |
| ⛔ **il perimetro del Traguardo 4** — quanto ne costruisce, dove vive ciascun pezzo, e per ogni artefatto **il controllo che lo esercita**. Si legge **prima** di scriverne il piano | [`specs/2026-08-11-…-traguardo-4-simulatore-dst-design.md`](../superpowers/specs/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst-design.md) — ⚠️ **non è una spec**: è lo scaglionamento che la §3 non fissa |
| il **cosa** del kernel: §0–§10 | [`specs/2026-08-06-kernel-design.md`](../superpowers/specs/2026-08-06-kernel-design.md) |
| il testo integrale dei **gotcha** e delle **misure**, con i numeri | [`HANDOFF.md`](../HANDOFF.md) — ⚠️ **a sezioni** |
| ⛔ **cosa una sezione deve incassare, prima di proporle una modifica** | [`HANDOFF.md`](../HANDOFF.md) — il **consuntivo voce per voce**: cosa era stato deciso, dove è finito, e cosa resta da scrivere. È **autorevole**, e si legge **prima** di proporre, non dopo. ⚠️ **La sezione, non il file** |
| l'ordine dei sotto-progetti e le dipendenze — quanti siano lo dice la tabella di quel file | [`roadmap.md`](../roadmap.md) |
| dove vive una funzionalità della mappa originale | [`tracciabilita.md`](../tracciabilita.md) — ⚠️ **leggi il riquadro in testa**: risponde a «dove vive», **non** a «di quale meccanismo ha bisogno». È la crepa da cui sono uscite le sette voci |
| **dove vive ogni controllo** della porta, riga per riga sul catalogo §7.4, e cosa **non** è coperto | [`porta-di-qualita.md`](../porta-di-qualita.md) |
| ⛔ **perché un seme NON è un oracolo**, e cosa identifica un caso in ciascuna delle due campagne DST — al livello 2 *«un seme»* **non esiste** | [`semi-dst.md`](../semi-dst.md) — ⚠️ **nasce vuoto**, e la riga vuota è deliberata; ⛔ **è CRLF integrale**, misurato il 2026-08-25 |
| la **strategia di test** — è la fonte di verità sulla porta di qualità, e mappa Q1–Q24 → metodo | [`design/08-strategia-di-test.md`](../design/08-strategia-di-test.md) |
| la **topologia dei processi** — contiene la tensione che F1b deve conciliare | [`design/01-topologia-dei-processi.md`](../design/01-topologia-dei-processi.md) |
| gli altri diagrammi della struttura | [`design/`](../design/) — quanti lo dice `ls docs/design/` |
| gli **esiti degli spike**, con seed, versioni e comandi | [`../spikes/RISULTATI.md`](../../spikes/RISULTATI.md) |
| i requisiti della GUI, G1–G21 e P1–P4 | [`../spikes/GUI-REQUISITI.md`](../../spikes/GUI-REQUISITI.md) |
| la **provenienza** di ciò che non abbiamo dedotto noi, con le date | [`riferimenti.md`](../riferimenti.md) |
| il **modello** di come si scrive un piano qui, con l'errata in testa | [`plans/2026-08-06-spike-linguaggio-del-core.md`](../superpowers/plans/2026-08-06-spike-linguaggio-del-core.md) |
| ⛔ **cosa il piano del Traguardo 1 detta e il repository smentisce** — quattro voci, prima fra tutte gli identificatori italiani | [`plans/2026-08-08-sottoprogetto-1-traguardo-1-scheletro-e-porta.md`](../superpowers/plans/2026-08-08-sottoprogetto-1-traguardo-1-scheletro-e-porta.md) — ⚠️ **solo l'errata in testa**, il resto è eseguito |
| ⛔ **come si esegue un piano qui, e le quattro specie di difetto** — è il piano del Traguardo 2, **eseguito per intero**, con quarantanove voci di errata in sei passate | [`plans/2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md`](../superpowers/plans/2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md) — ⚠️ **a compiti, mai intero**. |
| ⛔ **come si esegue un piano, e come si CHIUDE un traguardo** — è il piano del Traguardo 3, **eseguito per intero**, dodici compiti su dodici. ⚠️ **L'errata in testa si legge prima del compito**, ed è a **settantasette voci in nove passate**, di cui **nove decisioni**; le ultime tre sono la **Definizione di «fatto» che invecchia** | [`plans/2026-08-10-sottoprogetto-1-traguardo-3-giornale-e-formato-durevole.md`](../superpowers/plans/2026-08-10-sottoprogetto-1-traguardo-3-giornale-e-formato-durevole.md) — ⚠️ **a compiti, mai intero** |
| ⛔ **come si esegue un piano quando il pre-controllo trova un difetto in DIECI compiti su dieci** — è il piano del Traguardo 4, **eseguito per intero**. ⚠️ **L'errata in testa è a settanta voci in nove passate, di cui dodici DECISIONI**, e si legge **prima** di riaprire qualunque cosa che quel traguardo abbia toccato | [`plans/2026-08-11-…-traguardo-4-simulatore-dst.md`](../superpowers/plans/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst.md) — ⚠️ **a compiti, mai intero** |
| l'indice di ADR e diagrammi | [`README.md`](../README.md) |
| ⛔ **il messaggio da incollare all'inizio di una chat**, e il perché di ogni sua riga | [`AVVIO-CHAT.md`](../AVVIO-CHAT.md) — ⚠️ **il peso del messaggio lo dà il comando sotto questa tabella**, non questa cella: il **metodo** sono le righe **fra le due recinzioni, escluse**, o due lettori onesti ottengono due numeri (59ª misura). ⚠️ **Dal 2026-09-09 il proprietario non lo incolla più** (decisione 32 della stella polare della GUI): il file resta com'è, e non è lettura d'apertura |

⚠️ **I pesi non stanno più in questa tabella, e non è una svista.** Un peso scritto
invecchia al primo commit che tocca il file; il comando che lo produce no:

```
find docs -name '*.md' | xargs wc -c | sort -n
```

📌 **E il messaggio di [`AVVIO-CHAT.md`](../AVVIO-CHAT.md), che ha un perimetro proprio** — le
righe fra le due recinzioni, escluse:

```
awk '/^```$/{n++} n==1 && !/^```$/{c++; b+=length($0)+1} END{print c" righe, "b" byte"}' docs/AVVIO-CHAT.md
```

📚 **Le misure storiche** — il verbale di come i pesi sono cambiati dal
2026-08-08 al 2026-08-28 — stanno in
[`archivio/misure-dimensioni.md`](misure-dimensioni.md). ⛔ **Non è una lettura
obbligatoria:** si apre con una domanda storica in mano, non per farsi un'idea.

⚠️ Ed è la ragione per cui la frase in testa dice «oltre mezzo megabyte» invece di una cifra:
**un limite inferiore misurato resta vero mentre i documenti crescono, una cifra esatta no.**

⚠️ **Prima di ogni commit di documentazione:** `bash scripts/check-docs.sh`

### §13, il messaggio d'avvio — righe 949–951

Stava fra la tabella dei controlli e *«Cosa aggiornare, e quando»*.

📋 **Il messaggio da incollare all'inizio di una chat** vive in
[`AVVIO-CHAT.md`](../AVVIO-CHAT.md). Non nomina il prossimo passo, deliberatamente: lo
stato sta nella §6, in un posto solo.
