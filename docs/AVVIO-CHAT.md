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
  /superpowers:subagent-driven-development  <-- SERVE SUBITO: il piano è
                                        SCRITTO, quindi non si ri-pianifica —
                                        si ESEGUE, un subagente fresco per
                                        compito con revisione fra uno e l'altro.
                                        È la modalità scelta, e ha portato dieci
                                        compiti su dieci al Traguardo 4 e dodici
                                        su dodici al 3. ⛔ E LA REVISIONE NON È
                                        UNA FORMALITÀ: al Task 6 ha trovato un
                                        mutante vivo nell'INTERO workspace che
                                        l'esecuzione non aveva visto. Anche
                                        un'ONDATA DI CORREZIONI si rivede come
                                        il compito: i suoi difetti stanno nella
                                        PROSA, e «non tocca righe eseguibili»
                                        è un argomento PER rivederla, non
                                        contro. ⛔ AI TASK 7 E 8 SONO SERVITE
                                        TRE REVISIONI PIENE CIASCUNO, e NOVE
                                        volte un'ondata ha aperto rilievi nuovi
                                        nella prosa scritta per chiuderne di
                                        vecchi: il numero di giri non si decide
                                        dalla specie del diff. ✅ E IL RIMEDIO
                                        È MISURATO DUE VOLTE: all'ondata si
                                        dice TOGLIERE, non riscrivere meglio,
                                        e il giro dopo torna pulito. ⛔ E I
                                        RILIEVI SI SCRIVONO PER SPECIE, non
                                        per elenco di righe: «togli» manda a
                                        CERCARE le altre occorrenze della
                                        stessa forma, «riscrivi» fa guardare
                                        solo il sito indicato. ⛔ Quale piano lo dice la
                                        §6 del compendio, che leggi comunque —
                                        non è nominato qui apposta, un rimando
                                        non può marcire
  /superpowers:test-driven-development  perché il lavoro che viene È CODICE

E QUESTE QUANDO SERVIRANNO, NON PRIMA
  /superpowers:brainstorming                 quando ricomincerà un lavoro
                                             creativo. NON serve ora: quello del
                                             Traguardo 5 è chiuso. Si invoca
                                             comunque PRIMA di entrare in plan
                                             mode il giorno che riapre
  /superpowers:writing-plans                 quando ci sarà un piano NUOVO da
                                             scrivere. NON serve ora: quello del
                                             Traguardo 5 è scritto, tredici
                                             compiti in cinque parti
  /anthropic-skills:repo-audit               se e quando si riaprirà un audit.
                                             NON serve ora: quello dell'11
                                             agosto è eseguito per intero

LEGGI QUESTI TRE FILE, PER INTERO, POI FERMATI          — 500 KB in tutto
  1. CLAUDE.md
  2. docs/COMPENDIO.md — contiene TUTTE le decisioni del progetto: le 37 ADR
     compresse, le sei invarianti, le proprietà non retrofittabili, lo stack
     (§4), lo stato di oggi e il prossimo passo (§6), il non rilitigabile (§7),
     cosa NON rifare (§8), i gotcha (§9 — quanti, lo dice il suo titolo),
     le trappole di check-docs.sh (§10), i vincoli sul primo commit (§11).
  3. docs/audit-2026-08-11.md — 31 KB. ⛔ È IL TERZO FILE E NON È UN'ECCEZIONE
     ALLA REGOLA DEI DUE, ed è CHIUSO: non è più IL COMPITO, è il VERBALE
     di COME si lavora qui. Contiene la copertura dichiarata, le
     quattro radici, i finding con causa radice e dimostrazione, ciò che è
     stato verificato PULITO, e la §8 con le otto decisioni — TUTTE E OTTO
     ESEGUITE, e la §5 porta in testa il richiamo datato.
  ⚠️ In token costano ALMENO il triplo di quel che i KB suggeriscono: misurato,
     quattrocento righe del solo compendio pesano 25148 token, e il compendio
     ne ha oltre tremila. È un LIMITE INFERIORE, non un totale — §12.

⛔ CODICE IN INGLESE, DOCUMENTAZIONE IN ITALIANO — §1.0 della spec.
   Crate, moduli, tipi, funzioni, messaggi d'uscita e commenti nel sorgente
   sono in inglese; i documenti restano in italiano; un riferimento al codice
   dentro un documento si scrive in inglese, col nome esatto del sorgente.
   Non è tipografia: la regola non stava scritta, e un traguardo intero è stato
   scritto con gli identificatori italiani e poi rifatto.

⚠️ IL REPOSITORY CONTIENE CODICE RUST — QUATTRO traguardi su sei eseguiti,
   l'ultimo il Traguardo 4 (il simulatore DST: il guasto), dieci compiti su
   dieci, GATE GREEN a tutti — e il QUINTO è IN ESECUZIONE, un compito per
   volta. ⛔ A che punto sia lo dice la §6 del compendio, in un posto solo:
   qui non è scritto apposta, perché una cifra ricopiata marcisce.
   Serve rustup: rust-toolchain.toml tira giù da sé la 1.95.0 e il bersaglio
   x86_64-unknown-none, quindi non installare niente a mano. Su Windows serve
   in più il linker MSVC (Visual Studio Build Tools), che rustup NON porta.
   Il primo comando utile è
       bash scripts/gate.sh
   e dice in un colpo se l'ambiente regge: deve stampare GATE GREEN.

⛔ L'AUDIT È CHIUSO, IL BRAINSTORMING PURE, E IL PIANO È SCRITTO. LA SPECIE
   DEL LAVORO CHE VIENE È: ESEGUIRE UN PIANO, UN COMPITO PER VOLTA. Non più
   lavoro creativo, non più pianificazione — CODICE. ⛔ Il piano si legge a
   COMPITI e mai intero, e prima di ogni compito si fanno le SETTE domande
   del pre-controllo. Quale piano lo dice la §6 del compendio, in un posto
   solo; quanto pesa, il blocco dei pesi qui sotto.
   ⚠️ È la riga che invecchia per costruzione: si riscrive quando il passo si
      chiude. L'elenco di ciò che ha già detto è TOLTO, non esteso.

   ⛔ COSA HANNO INSEGNATO LE OTTO DECISIONI DELL'AUDIT, IN UNA RIGA: IL
   RIMEDIO SI PREZZA LEGGENDO IL CODICE, NON IL RAPPORTO — e può risultare
   più PICCOLO, più GRANDE, o DI SPECIE DIVERSA.

⛔ NON aprire docs/HANDOFF.md, la spec del sotto-progetto 1, né la cartella
   docs/adr/ «per farsi un'idea»: insieme pesano oltre mezzo megabyte — 759 KB
   il 2026-08-21, la spec da sola 277, i quattro piani più grandi 344, 168,
   162 e 114 — e l'idea è già nel compendio. ⛔ IL PIANO DA ESEGUIRE È IL PIÙ
   GRANDE DEI QUATTRO, e si apre A COMPITI: cresce a ogni compito, perché
   l'errata cresce.
   Aprirai UN file — uno — quando ti servirà il perché di una decisione: le
   alternative scartate, le misure, i costi accettati. La §12 del compendio
   dice quale.

⛔ IL FORMATO DUREVOLE È CONGELATO. I byte in crates/kernel/tests/frozen/ —
   tre record e una mappa che il banco rilegge — NON si rigenerano mai: se
   cambiano non è un aggiornamento, è un CAMBIO DI FORMATO. Un campo nuovo
   dev'essere Option con #[cbor(default)] e prendere un INDICE NUOVO, e il
   significato di un indice non cambia mai.

⛔ LE DECISIONI PRESE ESEGUENDO, non dai piani, PUOI RIBALTARLE TUTTE:
   NOVE al Traguardo 3, DODICI al Traguardo 4, e il piano in corso ha la
   propria errata che NON È PIÙ VUOTA. Stanno nelle errata in testa ai piani,
   e la §6 del compendio — che leggi comunque — porta quelle che governano
   le altre. ⛔ L'ERRATA IN TESTA AL PIANO SI LEGGE PRIMA DI OGNI COMPITO,
   non una volta sola: ci sono dentro i comandi dettati che non potevano
   fallire, le mutazioni dettate che non rovesciavano il proprio caso, e
   conteggi di chiamanti sbagliati in ENTRAMBI i termini, e un CORPO DETTATO
   disatteso sul merito. ⚠️ Non sono ricopiate qui: un rimando
   a documenti già scritti è ciò che questo messaggio deve TOGLIERE invece di
   accorciare, ed è la prescrizione della 25ª misura della §12.

⛔ LA LEZIONE DEL TRAGUARDO 4, imparata TRE VOLTE ogni volta DOPO aver chiuso
   la precedente, e la stessa che l'audit ha ritrovato sulla conformità:
     «L'INIEZIONE È AVVENUTA» e «C'ERA QUALCOSA DA VERIFICARE» SONO DUE
     AFFERMAZIONI, e una prova che tiene solo la prima è VERDE avendo
     confrontato insiemi vuoti.
   Il racconto per esteso — C7a, C7b, il ciclo di livello 2 — sta nella §6 del
   compendio. Quel che serve qui è la domanda: IN QUALE ALTRO STATO DEL MONDO
   QUESTA ASSERZIONE RESTEREBBE VERDE?

⚠️ LE QUESTIONI APERTE STANNO IN TRE POSTI, e non è un'incoerenza. La §6 del
   compendio tiene quelle del PRODOTTO — nessuna è un difetto oggi, e per
   ciascuna è scritto perché: le nove righe di guasto scoperte (hanno un
   indirizzo, quindi sono uno scaglionamento e non un arretrato), la metà del
   gotcha #51 che resta fuori, e semi-dst.md che NON HA UN CHIUDENTE.
   ⛔ La §5 dell'AUDIT teneva quelle della QUALITÀ, e le OTTO decisioni della
   §8 sono ESEGUITE: quel che vi resta sono i ~20 rilievi minori dichiarati
   non corretti, più DUE residui registrati — l'asserzione 4b della conformità
   reactor è IMPLICATA dalla 4a, quindi MUTA e non vacua, e le DIECI sonde
   permanenti dell'esecuzione non hanno riga di catalogo, perché la §7.4 è
   SPEC (vincolo globale 7). La tabella sta in porta-di-qualita.md.
   ⛔ E il terzo posto è IL PIANO DEL TRAGUARDO 5, che ne porta due specie.
   Dal proprio PRE-CONTROLLO, SETTE voci di cui TRE sono decisioni tue — un
   nome di tipo a una lettera da uno che esiste già, una riga di catalogo che
   vale per due regole, e due celle che nominano identificatori ITALIANI che
   il traguardo fa esistere in inglese: toccano la SPEC, registrate non prese.
   Dall'ESECUZIONE, altre SEI decisioni di disegno nell'errata — una di esse
   VA DECISA PRIMA DEL TRAGUARDO 6 ed è diventata PIÙ LARGA, e TRE sono
   dichiarate NON TENUTE DA NESSUNA SONDA nel sorgente stesso, di proposito:
   pinzarle congelerebbe la scelta che mettono davanti a te. ⛔ MA IL CONFINE,
   o «dichiaro» diventa una scusa per non provare: si dichiara solo ciò che è
   una SCELTA aperta. Se il doc accanto al codice AFFERMA già la cosa, quella
   è una regola senza prova e si PINZA — al Task 9 due mutanti vivi sono
   stati chiusi così, con due asserzioni in una sonda che esisteva già.
   ⛔ E una QUARTA
   frase è tenuta da un MUTANTE VIVO GARANTITO che NON è una politica aperta:
   non c'è niente da decidere, e sta dichiarata accanto alla frase con la
   misura. La §6 del compendio dice quali e perché.

⚠️ Il compendio è una COMPRESSIONE, non una selezione: ci sono dentro tutte le
   decisioni, non quelle attinenti al compito di oggi. Sparisce il ragionamento
   lungo, non la decisione. Nessuna può sfuggirti perché «non sembrava attinente».

SEI COSE CHE RIBADISCO, ANCHE SE STANNO NEI FILE
  · Non sono operativo in Rust. Quando l'argomento esce dal mio dominio,
    spiegamelo PRIMA a parole semplici e POI schematizza.
  · Un piano scritto qui è un'IPOTESI, non un'istruzione: il difetto è stato
    nel piano più spesso che nel codice — DODICI compiti su dodici al
    Traguardo 3, DIECI su dieci al Traguardo 4, su un piano scritto il giorno
    prima. E SEI voci d'errata erano difetti di un compito SUCCESSIVO, trovati
    misurando in anticipo: una di esse avrebbe fatto dichiarare chiuso un
    gotcha lasciandolo aperto. Prima di eseguire un compito si fanno le SETTE
    domande — le QUATTRO qui sotto più le tre che l'elenco non contiene, e
    l'elenco intero è in CLAUDE.md, «Prima di eseguire un compito di un
    piano», col testo lungo nel gotcha #49:
      1. la SONDA È SBAGLIATA — si coglie rileggendo
      2. la SONDA MANCA — per ogni artefatto che il compito produce, quale
         controllo lo esercita? Non c'è niente da leggere
      3. l'ARTEFATTO è sbagliato e compila — si vede SOLO scrivendone
         un'implementazione DA FUORI DALLA CRATE
      4. il COMPITO È GIÀ ESEGUITO — ciò che detta di produrre esiste già?
    E la QUINTA, che non è nell'elenco: IL CONTRATTO CRESCE SOTTO IL PIANO —
    un compito scritto prima si legge contro il codice di ADESSO, non contro
    il piano. Prova in negativo PRIMA di crederci, e dove diverge registra
    la divergenza invece di allinearti all'attesa.
    La SESTA: un ADR si legge anche contro i propri FRATELLI (#59). La
    SETTIMA: un RAPPORTO è un piano, e si prezza leggendo il codice (#65).
    ⛔ E tutte valgono anche per il DISEGNO che il piano traduce — gotcha #58 —
    e per il PIANO STESSO. Misurato due volte: scrivendo il piano del
    Traguardo 4 il disegno sbagliava due cose che stavano nei BANCHI DI PROVA,
    una in un COMMENTO; scrivendo quello del Traguardo 5 il disegno DICHIARAVA
    una divergenza dalla spec e ne TACEVA una gemella — gotcha #71, e la
    taciuta era giusta nel merito, quindi il difetto non è la scelta ma che il
    proprietario non l'ha vista. Un documento che ha letto le guardie si sente
    verificato, ed è lì che smette di guardare; e uno che dichiara UNA
    divergenza si legge come se le avesse dichiarate tutte.
  · E il banco con cui misuri sbaglia VERSO L'ATTESA — gotcha #48, il più
    frequente del progetto. ⛔ Quante volte NON è scritto qui, ed è tolto e non
    ricorretto: diceva TREDICI mentre la §12 ne registrava già QUATTORDICI, e un
    conteggio che vive in due posti marcisce sempre in quello che non lo muove.
    Le forme misurate stanno nella §9 del compendio; qui basta il
    contro-verso: prova che la mutazione si sia APPLICATA, compila in un passo
    separato dall'eseguire, per ogni mutazione su un valore provane DUE, e non
    ripristinare con `git checkout --` un file che stai anche scrivendo.
    ⛔ E REVOCA RIPRISTINANDO DA UNA COPIA PRESA PRIMA, mai risostituendo
    all'indietro: una revoca che deve CERCARE può fallire e lasciare il file
    mutato — successo al Task 8, sette misure buttate.
    ⛔ E VALE ANCHE PER GLI ORACOLI CHE SCRIVI TU NEI BRIEF, che è dove il
    difetto è caduto al Task 9 — nel brief, non nel piano: una cifra ASSOLUTA
    non è un oracolo su un artefatto che CRESCE, e non può essere verde. Si
    scrive una RELAZIONE fra due misure dello stesso artefatto — «CR == LF e
    zero LF nudi», non «CR == 2820».
    ⛔ L'audit l'ha ripetuto su sé stesso e va detto: la prima misura della
    baseline fu TRONCATA da un `| tail -80` e diede «8 target, 30 test» invece
    di 32 e 171; e la prima copia per provare check-docs.sh OMETTEVA spikes/,
    producendo dodici link rotti che erano del banco e non del codice.
    Entrambe rifatte invece che riportate.
    ⚠️ La forma peggiore resta quella psicologica: chi aveva colto un errore di
    misura ha creduto al RIMPIAZZO senza dubitarne.
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

⚠️ DUE MESSAGGI DI COMMIT SONO MALFORMATI, e la decisione è mia non tua:
   8d875f4 porta un «@» in testa e in coda, 0fd3ec8 ha perso gli accenti.
   Ripulirli richiede un rebase e --force-with-lease su un ramo già pushato,
   e con due macchine sullo stesso repository quella è una scelta che faccio
   io. NON toccarli di iniziativa; se te lo chiedo, allora sì.

Parti confermandomi cosa hai letto e qual è la tua lettura dello stato, POI
ASPETTA le mie richieste prima di scrivere qualunque cosa. ⛔ Non c'è un
audit da eseguire, né un brainstorming da aprire, né un piano da scrivere:
sono chiusi tutti e tre. Quello che viene è l'ESECUZIONE di un piano già
scritto — un compito per volta, con revisione fra uno e l'altro — e il
piano si legge a COMPITI, mai intero.
⚠️ Quale piano, e di che traguardo, sta nella §6 del compendio, non qui.
```

---

## Perché è così corto

| | Prima | Adesso |
|---|---|---|
| il messaggio | ~9 KB | ✅ **LA SERIE HA GIRATO, e quanto pesa OGGI non è scritto qui:** lo rimisura la **§12 del compendio** a ogni passata, e questa cella ne teneva una seconda copia **ferma al 2026-08-18** — *«14,7 KB»* — che nessuna delle tre passate successive aveva mosso. **Tolta, non riallineata** (gotcha **#68**). La storia resta perché è storia: era 7,7 → 9,8 → 12,2 → 14,7 → 15,0 → 15,4 → 15,9 → **16,7 KB**, sette crescite e un solo fermo; poi il 2026-08-18, chiudendo l'audit, il blocco è **calato per la prima volta** — `16659 → 14947 byte`, **−10,3 %** — e la passata di coerenza dello stesso giorno lo ha riportato a **15014**, `+67 B` e una riga: la crescita più piccola mai registrata. ⛔ **Ha funzionato la prescrizione della 25ª misura, ed è la parte da ricordare:** *«decidere cosa TOGLIERE, non cosa accorciare»*. Tolto il blocco delle ventuno decisioni ribaltabili — che era un **rimando a due errata già scritte** ricopiato per esteso — e compresso a due righe il finding V6, che nel frattempo era **chiuso**. ⚠️ Ciò che si toglie è un **rimando duplicato** o una voce **chiusa**, mai una lezione: quelle si spostano nel compendio, dove chi legge le trova comunque |
| lettura che ordinava | l'intero corpus, oltre mezzo megabyte | **469 KB** — `CLAUDE.md` più il compendio, e **500** con l'audit |
| decisioni note all'agente | tutte, dopo aver letto tutto | **tutte**, dopo 469 KB |

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
resta quello che conta: **500 KB contro 759**. ⛔ **E il 2026-08-18 una passata ha trovato che il
`grep` con cui si censiscono le case restituisce CANDIDATE e non case:** delle sei occorrenze di
`353`, **due erano numeri di riga** — `COMPENDIO.md:353`, dentro la citazione del finding A-2 —
e correggerle avrebbe rotto due riferimenti giusti. Gotcha **#70**, ed è il modo di fallire del
rimedio che la ventesima misura prescrive contro il #31. ⛔ **E il 2026-08-18 il METODO ha rivelato un
difetto che i numeri non mostravano: «`wc -c`» non dice QUALI byte, e questo repository si lavora
da DUE macchine.** Su un albero CRLF `wc -c` conta un byte in più per riga, quindi nove celle
della §12 sembravano stantie di 1–4 KB **su file non toccati da giorni**; tolti i CR, tutte e
nove coincidevano. I pesi si misurano in **byte LF**, ed è una riga di metodo **registrata e non
presa** — la ventottesima misura. ⚠️ **E le case sono state ricontate col
`grep` il 2026-08-17, non riprese dal verbale precedente:** l'aggregato ne ha **quattro**,
la cifra dei due file obbligatori **tre**, quella coi tre file **quattro** — la 25ª ne
dichiarava sei per la seconda, ed è il gotcha **#31** applicato all'**elenco delle case**.

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
> si è mosso: 9,8 KB di messaggio che ordinano 242 KB di lettura, contro 689 di corpus.**

📌 **Il percorso del repository ha due valori, e non è una svista da «sanare».** Si lavora
su **due macchine**, e il campo le nomina entrambe: un percorso solo sembrava aggiornato e
su una delle due era falso. Chi incolla il messaggio lascia pure tutti e due — l'agente
parte dentro il repository e la propria posizione la conosce già.
