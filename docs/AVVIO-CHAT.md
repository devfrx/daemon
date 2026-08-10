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

E QUESTE QUANDO SERVIRANNO, NON PRIMA
  /superpowers:brainstorming                 prima di qualunque lavoro creativo
  /superpowers:writing-plans                 quando si scriverà un piano nuovo
  /superpowers:subagent-driven-development   per ESEGUIRE un piano: un subagente
                                             fresco per compito, con revisione fra
                                             uno e l'altro. È la modalità scelta
  /superpowers:test-driven-development       quando si scrive codice

LEGGI SOLO QUESTI DUE FILE, PER INTERO, POI FERMATI     — 139 KB in tutto
  1. CLAUDE.md
  2. docs/COMPENDIO.md — contiene TUTTE le decisioni del progetto: le 37 ADR
     compresse, le sei invarianti, le proprietà non retrofittabili, lo stack
     (§4), lo stato di oggi e il prossimo passo (§6), il non rilitigabile (§7),
     cosa NON rifare (§8), i quarantanove gotcha (§9), le trappole di
     check-docs.sh (§10), i vincoli sul primo commit di codice (§11).

⛔ CODICE IN INGLESE, DOCUMENTAZIONE IN ITALIANO — §1.0 della spec.
   Crate, moduli, tipi, funzioni, messaggi d'uscita e commenti nel sorgente
   sono in inglese; i documenti restano in italiano; un riferimento al codice
   dentro un documento si scrive in inglese, col nome esatto del sorgente.
   Non è tipografia: la regola non stava scritta, e un traguardo intero è stato
   scritto con gli identificatori italiani e poi rifatto.

⚠️ IL REPOSITORY CONTIENE CODICE RUST — due traguardi eseguiti, e il terzo
   è il piano che stiamo per eseguire.
   Serve rustup: rust-toolchain.toml tira giù da sé la 1.95.0 e il bersaglio
   x86_64-unknown-none, quindi non installare niente a mano. Su Windows serve
   in più il linker MSVC (Visual Studio Build Tools), che rustup NON porta.
   Il primo comando utile è
       bash scripts/gate.sh
   e dice in un colpo se l'ambiente regge: deve stampare GATE GREEN.

⛔ NON aprire docs/HANDOFF.md, la spec del sotto-progetto 1, né la cartella
   docs/adr/ «per farsi un'idea»: insieme pesano oltre mezzo megabyte, e
   l'idea è già nel compendio. Aprirai UN file — uno — quando ti servirà il
   perché di una decisione: le alternative scartate, le misure, i costi
   accettati. La §12 del compendio dice quale.

⚠️ Il compendio è una COMPRESSIONE, non una selezione: ci sono dentro tutte le
   decisioni, non quelle attinenti al compito di oggi. Sparisce il ragionamento
   lungo, non la decisione. Nessuna può sfuggirti perché «non sembrava attinente».

SEI COSE CHE RIBADISCO, ANCHE SE STANNO NEI FILE
  · Non sono operativo in Rust. Quando l'argomento esce dal mio dominio,
    spiegamelo PRIMA a parole semplici e POI schematizza.
  · Un piano scritto qui è un'IPOTESI, non un'istruzione, e nel Traguardo 2
    il difetto è stato nel piano molto più spesso che nel codice. Quattro
    forme, in ordine di quanto sono difficili da vedere:
      1. la SONDA È SBAGLIATA — vacua, o attacca il caso invece del
         meccanismo, o guarda una direzione sola. Si coglie rileggendo.
      2. la SONDA MANCA, e non si vede leggendo perché non c'è niente da
         leggere. L'unica domanda che la trova: per ogni artefatto che il
         compito produce, quale controllo lo esercita?
      3. l'ARTEFATTO è sbagliato, e compila. Al Task 11 il piano dettava
         una porta CHE NON SI PUÒ IMPLEMENTARE: passava la porta di
         qualità, e si vede solo scrivendone un'implementazione DA FUORI
         DALLA CRATE. Una porta dichiarata in anticipo va provata dal lato
         di chi la implementerà, non da quello di chi la dichiara.
      4. il COMPITO È GIÀ ESEGUITO, e il piano non lo sa — gotcha #49. Ai
         Task 13-14 il piano dettava di AGGIUNGERE a un registro righe che
         c'erano già, perché la disciplina impone di aggiornarlo a ogni
         passo: un compito di consolidamento in coda nasce vuoto. Non si
         vede né rileggendo il piano né leggendo il codice, perché tutti e
         due sono corretti. Si vede SOLO chiedendosi, prima di eseguire:
         ciò che questo compito detta di produrre esiste già?
    Prova in negativo PRIMA di crederci, e dove diverge registra la
    divergenza invece di allinearti all'attesa.
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

Parti confermandomi cosa hai letto, qual è la tua lettura dello stato, e qual è
il prossimo passo secondo la §6 — POI ASPETTA le mie richieste prima di
proporre qualunque cosa.
```

---

## Perché è così corto

| | Prima | Adesso |
|---|---|---|
| il messaggio | ~9 KB | ~4 KB |
| lettura che ordinava | l'intero corpus, oltre mezzo megabyte | **139 KB** — `CLAUDE.md` più il compendio |
| decisioni note all'agente | tutte, dopo aver letto tutto | **tutte**, dopo 139 KB |

⚠️ **I due numeri di destra si rimisurano, e sono già stati falsi due volte.** Dicevano
*«24 KB, ~6k token»*: era vero quando il compendio pesava un terzo di oggi, e nessuno
l'aveva più rifatto. Poi hanno detto **88 KB** mentre erano **91**, ed è per questo che
questa riga è stata riscritta. È il gotcha **#31** — una cifra messa a sostegno di una
regola giusta non viene mai riverificata, perché nessuno dubita della regola. Il rapporto
resta quello che conta: **139 KB contro mezzo megabyte**.

Il messaggio lungo elencava undici letture «PER INTERO», fra cui **tutti** gli ADR, nove
diagrammi e la spec del sotto-progetto 1, che oggi pesa **271 KB**. Non è che chiedesse
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

⚠️ **Cosa aggiornare qui, e quando.** Il ramo se cambia; il peso dei due file
obbligatori quando uno dei due cresce; il numero dei gotcha quando §9 ne guadagna uno; i
prerequisiti d'ambiente quando il repository comincia a richiedere uno strumento nuovo —
è successo con `rustup`, e prima di allora non serviva niente. ⛔ **Lo SHA non è in questo
elenco**, ed è la voce che è stata tolta: si compila incollando, non si mantiene.

📌 **Il percorso del repository ha due valori, e non è una svista da «sanare».** Si lavora
su **due macchine**, e il campo le nomina entrambe: un percorso solo sembrava aggiornato e
su una delle due era falso. Chi incolla il messaggio lascia pure tutti e due — l'agente
parte dentro il repository e la propria posizione la conosce già.
