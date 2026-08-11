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
  /anthropic-skills:repo-audit          <-- SERVE SUBITO, ed è cambiato rispetto
                                        alla sessione scorsa: quello che viene
                                        dopo NON è un brainstorming ma
                                        l'ESECUZIONE DELL'AUDIT già fatto. La
                                        skill governa la remediation: radice e
                                        mai sintomo, niente campionamento
                                        silenzioso, e i fix in sequenza — il
                                        parallelismo è per LEGGERE, mai per
                                        scrivere

E QUESTE QUANDO SERVIRANNO, NON PRIMA
  /superpowers:test-driven-development       quando ricomincerà il codice: buona
                                             parte delle voci aperte è una SONDA
                                             che manca, non una funzione
  /superpowers:brainstorming                 quando l'audit sarà chiuso e
                                             comincerà il Traguardo 5 — è lavoro
                                             creativo, e si invoca PRIMA di
                                             entrare in plan mode
  /superpowers:writing-plans                 quando le voci aperte di QUEL
                                             brainstorming saranno chiuse
  /superpowers:subagent-driven-development   quando ci sarà un piano da
                                             ESEGUIRE: un subagente fresco per
                                             compito, con revisione fra uno e
                                             l'altro. È la modalità scelta, e ha
                                             appena portato dieci compiti su dieci

LEGGI QUESTI TRE FILE, PER INTERO, POI FERMATI          — 278 KB in tutto
  1. CLAUDE.md
  2. docs/COMPENDIO.md — contiene TUTTE le decisioni del progetto: le 37 ADR
     compresse, le sei invarianti, le proprietà non retrofittabili, lo stack
     (§4), lo stato di oggi e il prossimo passo (§6), il non rilitigabile (§7),
     cosa NON rifare (§8), i sessantaquattro gotcha (§9), le trappole di
     check-docs.sh (§10), i vincoli sul primo commit di codice (§11).
  3. docs/audit-2026-08-11.md — 22 KB. ⛔ È IL TERZO FILE E NON È UN'ECCEZIONE
     ALLA REGOLA DEI DUE: è IL COMPITO. Contiene la copertura dichiarata, le
     quattro radici, i finding con causa radice e dimostrazione, ciò che è
     stato verificato PULITO, e la §8 con le OTTO DECISIONI che aspettano me.
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
   dieci, GATE GREEN a tutti.
   Serve rustup: rust-toolchain.toml tira giù da sé la 1.95.0 e il bersaglio
   x86_64-unknown-none, quindi non installare niente a mano. Su Windows serve
   in più il linker MSVC (Visual Studio Build Tools), che rustup NON porta.
   Il primo comando utile è
       bash scripts/gate.sh
   e dice in un colpo se l'ambiente regge: deve stampare GATE GREEN.

⛔ IL PROSSIMO PASSO È L'ESECUZIONE DELL'AUDIT — NON IL TRAGUARDO 5.
   Il 2026-08-11 il repository ha ricevuto il suo primo audit completo: nove
   revisori paralleli in sola lettura, ogni finding grave riverificato sul
   sorgente, SEDICI corretti e provati nella stessa sessione. Il resto è
   aperto, ed è quello che devi portarmi.
   ⚠️ Questa riga ha detto TRE traguardi, poi QUATTRO, poi «quello che viene
      dopo è un BRAINSTORMING»: si riscrive quando il passo si chiude.

   LE TRE COSE CHE L'AUDIT HA TROVATO, e la prima è la più grave:
     · LA SUITE DI CONFORMITÀ PROVA V6 SOLO SU UN ARCHIVIO VUOTO. Le promesse
       5 e 8a costruiscono un giornale vuoto e chiedono subito il rifiuto:
       una guardia che chieda «l'archivio è vuoto?» invece di «questo passo ha
       un intento?» le soddisfa entrambe. Misurato: sostituite così le guardie
       di FileJournal::outcome e ::note, cargo test --workspace dà 32 target,
       171 passati, ZERO falliti — e la mutazione È OSSERVABILE, perché su un
       archivio non vuoto accetta esiti e note per passi MAI APERTI. Gotcha
       #63. ⛔ Chiuderlo è un'AGGIUNTA AL CONTRATTO DI UNA PORTA CONDIVISA,
       cioè una decisione mia: non farlo di iniziativa.
     · DUE BUCHI DEL CANCELLO lo lasciavano verde col confine caduto — già
       corretti e provati coi codici d'uscita veri. build = 'gen.rs' fra apici
       singoli sfuggiva al controllo dei build script (#61, il #28 riaperto da
       un carattere di quoting), e check-docs.sh non verificava che la spec
       ESISTESSE: le sei asserzioni di §8.6.1 vivono in blocchi END di awk, ed
       END non gira su un fatal (#60).
     · TRE DOCUMENTI DI STATO dicevano che il Traguardo 4 era da fare, mentre
       era eseguito da un commit — già corretti.

   ⛔ E IL GOTCHA PIÙ UTILE CHE NE È USCITO È IL #59: un ADR può essere
   falsificato da un ADR FRATELLO SCRITTO LO STESSO GIORNO, e nessuno dei due
   se ne accorge perché entrambi sono coerenti con sé stessi. ADR-0026 dice
   che il simulatore non va scritto perché c'è madsim; ADR-0031, stessa data,
   misura 55 crate e lo scarta — e il codice gli dà ragione. Nessuna delle
   quattro domande del pre-controllo lo coglie: guardano tutte il compito
   contro il CODICE, mai una decisione contro le decisioni vicine.

   ✅ E CIÒ CHE L'AUDIT HA TROVATO SANO CONTA QUANTO IL RESTO: le due campagne
   DST sono solide — la quarta occorrenza del difetto di vacuità NON c'è —
   nessun segreto in centosettantuno commit, il grafo spedito è esattamente
   quello dichiarato, e i quattordici conteggi di test di porta-di-qualita.md
   sono giusti tutti e quattordici.

⛔ NON aprire docs/HANDOFF.md, la spec del sotto-progetto 1, né la cartella
   docs/adr/ «per farsi un'idea»: insieme pesano oltre mezzo megabyte — 698 KB
   il 2026-08-11, la spec da sola 277, i tre piani più grandi 168, 162 e 114 —
   e l'idea è già nel compendio. Aprirai UN file — uno — quando ti servirà
   il perché di una decisione: le alternative scartate, le misure, i costi accettati. La §12
   del compendio dice quale.

⛔ IL FORMATO DUREVOLE È CONGELATO. I byte in crates/kernel/tests/frozen/ —
   tre record e una mappa che il banco rilegge — NON si rigenerano mai: se
   cambiano non è un aggiornamento, è un CAMBIO DI FORMATO. Un campo nuovo
   dev'essere Option con #[cbor(default)] e prendere un INDICE NUOVO, e il
   significato di un indice non cambia mai.

⛔ VENTUNO DECISIONI SONO STATE PRESE ESEGUENDO, non dai piani — PUOI
   RIBALTARLE TUTTE, e stanno nelle errata in testa ai due piani:
     · Traguardo 3: NOVE, fra cui Record::encode che NON restituisce Result,
       un SECONDO intento sullo stesso passo RIFIUTATO, la porta che guadagna
       note(), e il record con DUE campi — contenuto non fidato all'indice 3,
       ragione del chiamante al 4.
     · Traguardo 4: DODICI. Le tre che governano il resto:
         - la caduta NON SI RIPRENDE, ai due livelli. Un giornale che rifiuta
           una volta e poi riparte modella un disco cattivo, non un crash.
         - il punto di caduta viene da un generatore DIVERSO da quello
           dell'interlacciamento, con seme derivato: due generatori seminati
           con lo stesso numero danno la STESSA sequenza.
         - il numero di semi NON si massimizza: si sceglie sulla CHIUSURA
           DELLO SPAZIO DEGLI ESITI. Un tetto è un vincolo, non un bersaglio.

⛔ LA LEZIONE DEL TRAGUARDO 4, imparata TRE VOLTE ogni volta DOPO aver chiuso
   la precedente, e la stessa che l'audit ha ritrovato sulla conformità:
     «L'INIEZIONE È AVVENUTA» e «C'ERA QUALCOSA DA VERIFICARE» SONO DUE
     AFFERMAZIONI, e una prova che tiene solo la prima è VERDE avendo
     confrontato insiemi vuoti.
   Il racconto per esteso — C7a, C7b, il ciclo di livello 2 — sta nella §6 del
   compendio. Quel che serve qui è la domanda: IN QUALE ALTRO STATO DEL MONDO
   QUESTA ASSERZIONE RESTEREBBE VERDE?

⚠️ LE QUESTIONI APERTE STANNO IN DUE POSTI, e non è un'incoerenza: la §6 del
   compendio tiene quelle del PRODOTTO — nessuna è un difetto oggi, e per
   ciascuna è scritto perché — mentre la §5 dell'audit tiene quelle della
   QUALITÀ, che sono difetti veri e vanno chiuse. Fra le prime: le nove righe
   di guasto scoperte (hanno un indirizzo, quindi sono uno scaglionamento e
   non un arretrato), la metà del gotcha #51 che resta fuori, e semi-dst.md
   che NON HA UN CHIUDENTE. Fra le seconde, oltre a V6: read_back mai
   obbligato a scegliere per passo, il turn limit consegnato che può essere
   ignorato senza che nulla diventi rosso, quattro gruppi su cinque della
   conformità reactor mai visti scattare, e bincode dichiarato NON MANTENUTO
   (RUSTSEC-2025-0141) — che però ha ZERO usi di produzione, quindi la
   finestra per decidere si chiude da sola al Traguardo 6.

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
    gotcha lasciandolo aperto. Prima di eseguire
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
  · E il banco con cui misuri sbaglia VERSO L'ATTESA — gotcha #48, a TREDICI
    occorrenze. Le forme misurate stanno nella §9 del compendio; qui basta il
    contro-verso: prova che la mutazione si sia APPLICATA, compila in un passo
    separato dall'eseguire, per ogni mutazione su un valore provane DUE, e non
    ripristinare con `git checkout --` un file che stai anche scrivendo.
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

Parti confermandomi cosa hai letto, qual è la tua lettura dello stato, e come
proponi di affrontare le OTTO DECISIONI della §8 dell'audit — quali portarmi
per prime e perché — POI ASPETTA le mie richieste prima di scrivere qualunque
cosa. ⛔ Non c'è un piano da riprendere né un brainstorming da aprire: il
Traguardo 4 è CHIUSO e l'audit è FATTO; quello che manca è eseguirlo.
```

---

## Perché è così corto

| | Prima | Adesso |
|---|---|---|
| il messaggio | ~9 KB | ⛔ **14,7 KB**, e il riferimento è superato di parecchio: 7,7 → 9,8 → 12,2 → **14,7**, cioè la **quarta crescita di seguito** e la terza a due cifre percentuali. ⚠️ **La consegna del 2026-08-11 aveva PROVATO a comprimere e non è bastato**: le lezioni del Traguardo 4 e il gotcha #48 sono stati portati nel compendio — 344 byte recuperati — e il blocco dell'audit li ha più che compensati. 📌 Va detto invece che nascosto: **comprimere ciò che è vecchio non basta quando ciò che è nuovo pesa di più.** La prossima consegna deve decidere **cosa TOGLIERE**, non cosa accorciare |
| lettura che ordinava | l'intero corpus, oltre mezzo megabyte | **256 KB** — `CLAUDE.md` più il compendio, e **278** con l'audit |
| decisioni note all'agente | tutte, dopo aver letto tutto | **tutte**, dopo 256 KB |

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
resta quello che conta: **278 KB contro mezzo megabyte**.

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
