# Messaggio di avvio di una chat

Da copiare all'inizio di ogni nuova sessione.

⚠️ **Il campo dello SHA è un segnaposto, e si compila incollando.** Lo SHA appartiene al
**messaggio**, dove è vero nel momento in cui si legge — non a questo file, che del
messaggio è il **modello**: un hash scritto qui nasce già vecchio di uno, perché il commit
che lo aggiorna lo contiene. È la stessa ragione per cui [`HANDOFF.md`](HANDOFF.md) non ne
porta, e quella riga delega esplicitamente a qui.

⛔ **Prima questo campo portava un valore vero, e non ha funzionato.** È stato incollato
tale e quale **due volte** — l'ultima vecchio di **quattro** commit — e due commit
(`a2ac9f9`, `c7f3203`) sono serviti solo a rincorrerlo. Un valore d'esempio valido non si
distingue da un dato; un segnaposto sì. Gotcha **#43**.

⚠️ **Questo messaggio non nomina il prossimo passo, ed è deliberato.** Lo stato vive nella
§6 del compendio, in un posto solo. Duplicarlo qui lo farebbe divergere.

⛔ **Dice però di che _specie_ è il lavoro che viene dopo, e non è una deroga alla riga qui
sopra.** La skill da invocare dipende dalla **specie** del lavoro — creativo, oppure
esecuzione di un piano — non dal deliverable; e senza quella riga l'agente entra in plan
mode **saltando il brainstorming**, che è precisamente ciò che `CLAUDE.md` vieta. Il
*quale* deliverable resta scritto solo nella §6.

---

```
Riprendo un progetto già progettato, in corso.

Repo:   due macchine — C:\Users\zagor\Desktop\harness
                       E:\ALL\DEV\MY_REPOS\daemon
Remote: https://github.com/devfrx/daemon.git · branch spec/sottoprogetto-1-kernel
Albero pulito, ramo pushato. Ultimo commit: <<< INCOLLA QUI: git log --oneline -1 >>>

⛔ PRIMA DI PROPORRE QUALSIASI COSA: leggi, poi ASCOLTA. Non entrare in plan
   mode, non proporre passi, non scrivere file finché non te lo dico.

INVOCA QUESTE SKILL PRIMA DI QUALSIASI RISPOSTA O ESPLORAZIONE
  /superpowers:using-superpowers
  /anthropic-skills:dev-discipline      governa il codice
  /anthropic-skills:dev-communication   governa la conversazione intorno al codice
  /superpowers:subagent-driven-development  <-- SERVE SUBITO: quello che viene
                                        dopo NON è lavoro creativo ma l'ESECUZIONE
                                        di un piano già scritto. Un subagente
                                        fresco per compito, con revisione fra uno
                                        e l'altro. È la modalità scelta

E QUESTE QUANDO SERVIRANNO, NON PRIMA
  /superpowers:test-driven-development       quando si scrive codice — cioè al
                                             primo compito
  /superpowers:brainstorming                 NON serve adesso: il brainstorming
                                             del traguardo in corso è chiuso e il
                                             piano è scritto. Serve al prossimo
                                             traguardo, PRIMA del suo piano
  /superpowers:writing-plans                 idem — il piano di oggi c'è già

LEGGI SOLO QUESTI DUE FILE, PER INTERO, POI FERMATI     — 240 KB in tutto
  1. CLAUDE.md
  2. docs/COMPENDIO.md — contiene TUTTE le decisioni del progetto: le 37 ADR
     compresse, le sei invarianti, le proprietà non retrofittabili, lo stack
     (§4), lo stato di oggi e il prossimo passo (§6), il non rilitigabile (§7),
     cosa NON rifare (§8), i cinquantotto gotcha (§9), le trappole di
     check-docs.sh (§10), i vincoli sul primo commit di codice (§11).
  ⚠️ In token costano ALMENO il triplo di quel che i KB suggeriscono: misurato,
     quattrocento righe del solo compendio pesano 25148 token, e il compendio
     ne ha oltre milleseicento. È un LIMITE INFERIORE, non un totale — §12.

⛔ CODICE IN INGLESE, DOCUMENTAZIONE IN ITALIANO — §1.0 della spec.
   Crate, moduli, tipi, funzioni, messaggi d'uscita e commenti nel sorgente
   sono in inglese; i documenti restano in italiano; un riferimento al codice
   dentro un documento si scrive in inglese, col nome esatto del sorgente.
   Non è tipografia: la regola non stava scritta, e un traguardo intero è stato
   scritto con gli identificatori italiani e poi rifatto.

⚠️ IL REPOSITORY CONTIENE CODICE RUST — QUATTRO traguardi su sei eseguiti,
   l'ultimo il Traguardo 4 (il simulatore DST: il guasto), dieci compiti su
   dieci, GATE GREEN a tutti. ⛔ Quello che viene dopo NON è codice ma un
   BRAINSTORMING — il Traguardo 5, l'arbitro GPU — e la specie del lavoro è
   cambiata: prima si esplora, poi si disegna, poi si scrive il piano, poi si
   esegue. Sta scritto solo nella §6 del compendio.
   ⚠️ Questa riga ha detto TRE traguardi, poi QUATTRO, e ha detto «quello che
      viene dopo È CODICE» quando lo era: si riscrive quando il passo si chiude.
   Serve rustup: rust-toolchain.toml tira giù da sé la 1.95.0 e il bersaglio
   x86_64-unknown-none, quindi non installare niente a mano. Su Windows serve
   in più il linker MSVC (Visual Studio Build Tools), che rustup NON porta.
   Il primo comando utile è
       bash scripts/gate.sh
   e dice in un colpo se l'ambiente regge: deve stampare GATE GREEN.

⛔ NESSUN FILE VA APERTO OLTRE AI DUE, e questa riga è cambiata di specie il
   2026-08-11: il Traguardo 4 è ESEGUITO, quindi non c'è un piano in corso da
   leggere a compiti. Quello che viene dopo è il BRAINSTORMING del Traguardo 5
   — l'arbitro GPU — e un brainstorming si apre con la §6 del compendio, non
   con un piano: scriverne uno per codice che non esiste significa inventare.
   ⚠️ La riga qui sopra diceva «DUE FILE VANNO APERTI: il disegno e il piano
      del traguardo in corso». Vale di nuovo il giorno in cui un piano c'è.

⛔ NON aprire docs/HANDOFF.md, la spec del sotto-progetto 1, né la cartella
   docs/adr/ «per farsi un'idea»: insieme pesano oltre mezzo megabyte — 689 KB
   il 2026-08-11, la spec da sola 277, i piani 168 e 162 — e l'idea è già nel
   compendio. Aprirai UN file — uno — quando ti servirà il perché di una
   decisione: le alternative scartate, le misure, i costi accettati. La §12
   del compendio dice quale.

⛔ IL FORMATO DUREVOLE È CONGELATO. I byte in crates/kernel/tests/frozen/ —
   tre record e una mappa che il banco rilegge — NON si rigenerano mai: se
   cambiano non è un aggiornamento, è un CAMBIO DI FORMATO. Un campo nuovo
   dev'essere Option con #[cbor(default)] e prendere un INDICE NUOVO, e il
   significato di un indice non cambia mai.

⛔ CINQUE COSE DECISE ESEGUENDO il Traguardo 3, e non dal suo piano — PUOI
   RIBALTARLE, e stanno nell'errata in testa a quel piano:
     E22 Record::encode NON restituisce Result · E19 un SECONDO intento sullo
     stesso passo è RIFIUTATO · E27 la porta guadagna note() e il record la
     variante RecordKind::Note · E28 il record porta DUE campi, il contenuto
     non fidato all'indice 3 e la ragione del chiamante al 4 · E31 la via A4
     del confine dei dati è chiusa a LIVELLO 2, non «al formato».
   L'errata ne marca NOVE come DECISIONE; E31 non è fra quelle, perché
   corregge cosa significhi «chiusa».

⛔ E SETTE DECISIONI LE HA PRESE IL PIANO IN CORSO — anche queste ribaltabili,
   e stanno in testa a quel piano. Le due che governano il resto:
     · la caduta NON SI RIPRENDE. Un giornale che rifiuta una volta e poi
       riparte modella un disco cattivo, non un crash — ed è la permanenza
       che fa fermare TUTTE le attività interlacciate, non solo quella che ha
       toccato il confine.
     · il punto di caduta si estrae da un generatore DIVERSO da quello
       dell'interlacciamento, con seme derivato: due generatori seminati con
       lo stesso numero danno la STESSA sequenza, e la campagna esplorerebbe
       una diagonale dello spazio invece dello spazio.

⚠️ DIECI QUESTIONI RESTANO APERTE, in un posto solo: §6 del compendio, «Cosa il
   Traguardo 3 lascia aperto». Nessuna è un difetto oggi e per ciascuna è
   scritto perché. Il brainstorming che le aveva come materia è CHIUSO: ora
   sono il contesto dell'esecuzione, e UNA di esse il piano in corso la chiude.

⚠️ Il compendio è una COMPRESSIONE, non una selezione: ci sono dentro tutte le
   decisioni, non quelle attinenti al compito di oggi. Sparisce il ragionamento
   lungo, non la decisione. Nessuna può sfuggirti perché «non sembrava attinente».

SEI COSE CHE RIBADISCO, ANCHE SE STANNO NEI FILE
  · Non sono operativo in Rust. Quando l'argomento esce dal mio dominio,
    spiegamelo PRIMA a parole semplici e POI schematizza.
  · Un piano scritto qui è un'IPOTESI, non un'istruzione: il difetto è stato
    nel piano più spesso che nel codice, e nel Traguardo 3 il pre-controllo
    ne ha trovato almeno uno in DODICI compiti su dodici. Prima di eseguire
    un compito si fanno le QUATTRO DOMANDE — l'elenco è in CLAUDE.md, «Prima
    di eseguire un compito di un piano», col testo lungo nel gotcha #49:
      1. la SONDA È SBAGLIATA — si coglie rileggendo
      2. la SONDA MANCA — per ogni artefatto che il compito produce, quale
         controllo lo esercita? Non c'è niente da leggere
      3. l'ARTEFATTO è sbagliato e compila — si vede SOLO scrivendone
         un'implementazione DA FUORI DALLA CRATE
      4. il COMPITO È GIÀ ESEGUITO — ciò che detta di produrre esiste già?
    E una quinta che non è nell'elenco: IL CONTRATTO CRESCE SOTTO IL PIANO —
    un compito scritto prima si legge contro il codice di ADESSO, non contro
    il piano. Prova in negativo PRIMA di crederci, e dove diverge registra
    la divergenza invece di allinearti all'attesa.
    ⛔ E vale anche per il DISEGNO che il piano traduce — gotcha #58, misurato
    scrivendo quel piano: il disegno era stato letto contro la spec, gli ADR e
    le GUARDIE, e sbagliava due cose che stavano nei BANCHI DI PROVA, di cui
    una scritta in un COMMENTO. Le guardie non sono tutto il codice: un
    documento che le ha lette si sente verificato, ed è lì che smette di
    guardare i test. E un precedente si cita per la RAGIONE che lo ha
    prodotto, non per la forma.
  · E il banco con cui misuri sbaglia VERSO L'ATTESA — gotcha #48. Prova
    che la mutazione si sia applicata, compila in un passo separato
    dall'eseguire, e per ogni mutazione su un valore provane due. Sono già
    NOVE i risultati credibili e falsi in due sessioni, e la peggiore è
    la più innocua a vedersi: una rifinitura di LEGGIBILITÀ disarma la
    campagna di mutazione senza che nulla diventi rosso, perché le ancore
    sono accoppiate ai nomi del codice.
  · Alla chiusura di ogni voce COMMITTA E PUSHA senza chiedere, e SENZA
    co-autore. Prima però: bash scripts/gate.sh — comprende check-docs.sh.
  · Se ti viene un'idea che SOSTITUISCE una decisione presa, cerca PRIMA dove
    era già stata valutata e perché era caduta: la §7 del compendio elenca le
    riaperture più tentate e il costo di ciascuna. Riaprila solo con una prova
    nuova — e se la prova nuova gioca contro, registrala e chiudila. È il
    gotcha #32, e c'è perché è già successo.
  · Un controllo si prova in DUE direzioni: che scatti dove deve, e che NON
    scatti dove non deve. La seconda si dimentica, ed è quella che conta —
    gotcha #24. Vale anche per i tuoi test, non solo per i miei.

Parti confermandomi cosa hai letto, qual è la tua lettura dello stato, qual è
il prossimo passo secondo la §6 e da quale compito del piano si riprende —
POI ASPETTA le mie richieste prima di proporre qualunque cosa.
```

---

## Perché è così corto

| | Prima | Adesso |
|---|---|---|
| il messaggio | ~9 KB | **9,8 KB** ⚠️ era 7,7 fino al 2026-08-11 |
| lettura che ordinava | l'intero corpus, oltre mezzo megabyte | **240 KB** — `CLAUDE.md` più il compendio |
| decisioni note all'agente | tutte, dopo aver letto tutto | **tutte**, dopo 240 KB |

⚠️ **I due numeri di destra si rimisurano, e sono già stati falsi TRE volte.** ⛔ **La terza è
del 2026-08-10, chiudendo il Traguardo 3:** dicevano **165 KB** in **quattro** punti di questo
file mentre erano **192**, e il file che quella cifra descrive era cresciuto di ventitré
kilobyte sotto di essa. ⛔ **E i punti di questo file sono CINQUE, non quattro — ricontati col
`grep` su tutto il repository il 2026-08-11**, quando la cifra è passata a **207**: la
diciassettesima misura del compendio scriveva *«uno in `CLAUDE.md` e quattro qui»*, ed è il
gotcha **#31** applicato **all'elenco delle case** invece che alla cifra. Chi rimisura rifà anche
quel conteggio, che costa un comando. Prima ancora dicevano
*«24 KB, ~6k token»*: era vero quando il compendio pesava un terzo di oggi, e nessuno
l'aveva più rifatto. Poi hanno detto **88 KB** mentre erano **91**, ed è per questo che
questa riga è stata riscritta. È il gotcha **#31** — una cifra messa a sostegno di una
regola giusta non viene mai riverificata, perché nessuno dubita della regola. Il rapporto
resta quello che conta: **240 KB contro mezzo megabyte**.

Il messaggio lungo elencava undici letture «PER INTERO», fra cui **tutti** gli ADR, nove
diagrammi e la spec del sotto-progetto 1, che oggi pesa **277 KB**. Non è che chiedesse
troppo: chiedeva la cosa giusta nel posto sbagliato. Le decisioni servono **tutte** — ma
compresse, non integrali.

⚠️ **La voce sul piano è nata il 2026-08-09**, eseguendo i Task 1–6 del Traguardo 2. Non è una
massima: era il ritratto di sei difetti su sei.

> 🔁 **E il suo destinatario è cambiato due volte in un giorno solo, il 2026-08-10 — vale la
> pena scriverlo perché è la parte che invecchia più in fretta di tutto questo file.** La
> mattina il piano del Traguardo 2 è stato **eseguito per intero**, e la lezione ha smesso di
> servire a chi riprende **dentro** un piano per servire a chi ne **scrive** uno; la sera il
> piano del Traguardo 3 era **scritto**, e il destinatario è tornato a essere **chi esegue**.
> 📌 **Quindi vale per entrambi, ed è la formulazione che regge:** le quattro specie sono ciò
> che chi **scrive** un piano deve evitare di far ripetere, **e** ciò che chi lo **esegue**
> deve cercare prima di credere a un compito. La quarta — *il compito è già eseguito* —
> appartiene interamente al secondo: non c'è modo di scriverla via.

> 🔁 **Riscritta lo stesso giorno, chiudendo il Task 11 — e il ritratto era diventato stretto.**
> Diceva *«le sue sonde sono la parte che sbaglia»*, ed elencava sei difetti che erano **tutti**
> di quella specie. Poi i Task 8–10 hanno prodotto il difetto **«la sonda manca»** — che non si
> coglie rileggendo, perché non c'è niente da leggere — e il Task 11 il difetto **«l'artefatto è
> sbagliato, e compila»**. Tre specie, non una, e la seconda e la terza si trovano con domande
> diverse dalla prima. ⛔ Restringere il ritratto alla specie già vista è il gotcha **#29**
> spostato dalle invarianti a questo file: la formulazione più corta viene letta al posto di
> quella giusta. 📌 **E una sesta voce è nata dallo stesso compito**, sul banco di misura: il
> gotcha **#48** è costato quattro falsi risultati **credibili** in una sessione sola, e uno è
> ricapitato a chi stava verificando la riga che lo descrive.

> 🔁 **Riscritta il 2026-08-10, chiudendo il traguardo — e il ritratto era stretto una seconda
> volta, nello stesso modo.** Diceva *«tre specie»*, e i Task 13–14 ne hanno prodotta una
> **quarta**: il **compito già eseguito**. ⛔ È la più difficile delle quattro, e per una ragione
> che le altre tre non hanno: quelle si trovano guardando **il piano** o **il codice**, e questa
> non sta in nessuno dei due — il piano è coerente con sé stesso, il codice è corretto, e a
> essere sbagliato è il **rapporto fra il piano e il repository**. 📌 La domanda che la coglie
> non assomiglia alle altre tre: non *«questa sonda scatta?»* né *«quale controllo esercita
> questo artefatto?»*, ma **«ciò che questo compito detta di produrre esiste già?»**. ⚠️ E che il
> ritratto si sia ristretto **due volte di seguito** è esso stesso il dato: ogni volta elencava
> esattamente le specie viste fino a quel giorno, il che è il gotcha **#29** — la formulazione
> più corta viene letta al posto di quella giusta, e qui la più corta era anche l'ultima vera.

> 🔁 **Riletto il 2026-08-10, chiudendo il Traguardo 3 — e la cifra dei gotcha era stantia.**
> Diceva *«cinquantaquattro»* mentre la §9 ne conta **cinquantasei**: il **#55** e il **#56** sono
> nati col Task 11 e nessuno ha toccato questa riga. ⛔ **Ed è la voce che l'elenco qui sotto
> nomina per prima fra quelle da aggiornare** — *«il numero dei gotcha quando §9 ne guadagna
> uno»* — cioè la regola c'era, scritta in questo file, dodici righe più in basso. Un promemoria
> non è un controllo, e la guardia dei conteggi di `check-docs.sh` sa contare gli **ADR** e non i
> gotcha. ⚠️ Stessa passata: *«due traguardi eseguiti, e il terzo è il piano che stiamo per
> eseguire»* diceva il falso a **tre** eseguiti.

⚠️ **Cosa aggiornare qui, e quando.** Il ramo se cambia; il peso dei due file
obbligatori quando uno dei due cresce; il numero dei gotcha quando §9 ne guadagna uno; il
**numero dei traguardi eseguiti** quando uno si chiude; il **peso del messaggio** quando il
blocco fra le due recinzioni cresce — è la cella che nessuno rifaceva, ed era falsa di metà; i
prerequisiti d'ambiente quando il repository comincia a richiedere uno strumento nuovo —
è successo con `rustup`, e prima di allora non serviva niente. ⛔ **Lo SHA non è in questo
elenco**, ed è la voce che è stata tolta: si compila incollando, non si mantiene.

> 🔁 **Rimisurato il 2026-08-10, chiudendo il Traguardo 3 — e la cella del messaggio era la
> quarta cifra falsa di questo file.** Diceva **«~4 KB»**; misurato sul blocco fra le due
> recinzioni, a HEAD era **6,2 KB in 104 righe**, cresciuto sotto la propria descrizione senza
> che nessuno la rifacesse. ⚠️ **E questa passata l'ha fatto crescere ancora, a 7,7 KB in 127
> righe**, per le sei cose che la chiusura del traguardo pretendeva: il costo in token come
> limite inferiore, i pesi di ciò che non si apre, il **formato congelato**, le **cinque
> decisioni** ribaltabili, le **dieci** questioni aperte, e `superpowers:brainstorming` spostata
> fra quelle che servono **subito**. ⛔ **Il conto è stato pagato dove costava meno**, non
> ignorato: le quattro specie di difetto di piano passano da ventidue righe a quindici, perché
> il testo lungo vive ora in `CLAUDE.md` e nel gotcha **#49** e qui bastava l'elenco — la regola
> è *rimandare invece di riscrivere*. 📌 **Il rapporto che questo file esiste per difendere non
> si è mosso: 9,8 KB di messaggio che ordinano 240 KB di lettura, contro 689 di corpus.**

📌 **Il percorso del repository ha due valori, e non è una svista da «sanare».** Si lavora
su **due macchine**, e il campo le nomina entrambe: un percorso solo sembrava aggiornato e
su una delle due era falso. Chi incolla il messaggio lascia pure tutti e due — l'agente
parte dentro il repository e la propria posizione la conosce già.
