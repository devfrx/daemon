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
sopra.** La skill da invocare dipende dalla **specie** del lavoro — non dal deliverable — e
senza quella riga l'agente entra in plan mode **saltando il brainstorming**, che è
precisamente ciò che `CLAUDE.md` vieta. Il *quale* deliverable resta scritto solo nella §6.

⛔ **E le specie sono TRE, non due — la terza è nata il 2026-08-27 e va detta perché cambia la
skill.** Prima erano *creativo* ed *esecuzione di un piano*; la terza è il **RIMEDIO DI UN
AUDIT**, e non è nessuna delle due: non c'è niente da progettare, quindi **niente**
brainstorming; e non c'è un piano da eseguire, perché i finding sono **indipendenti** e la
ricetta è già scritta nel rapporto. ⚠️ **Trattarlo come creativo costa un brainstorming e un
piano per un lavoro che non ne ha bisogno; trattarlo come esecuzione fa cercare un piano che
non esiste.** Il messaggio qui sotto lo dice esplicitamente, ed è la riga che si riscrive per
prima quando l'audit si chiude.

---

```
Riprendo un progetto già progettato, in corso.

Repo:   due macchine — C:\Users\zagor\Desktop\harness
                       E:\ALL\DEV\MY_REPOS\daemon
Remote: https://github.com/devfrx/daemon.git · branch spec/sottoprogetto-1-kernel
Albero pulito, ramo pushato. Ultimo commit: <<< INCOLLA QUI: git log --oneline -1 >>>

⛔ PRIMA DI PROPORRE QUALSIASI COSA: leggi, poi ASCOLTA. Non entrare in plan
   mode, non proporre passi, non scrivere file finché non te lo dico.
   ⛔ IL BRAINSTORMING È L'ECCEZIONE, E VIENE SUBITO DOPO LA LETTURA: fa
   DOMANDE, non proposte, e non è plan mode. Se sia da APRIRE o da RIPRENDERE
   lo dice la §6, non questa riga.

INVOCA QUESTE SKILL PRIMA DI QUALSIASI RISPOSTA O ESPLORAZIONE
  /superpowers:using-superpowers
  /anthropic-skills:dev-discipline      governa il codice
  /anthropic-skills:dev-communication   governa la conversazione intorno al codice
  ⛔ IL LAVORO CHE VIENE È CREATIVO, non rimedio: si apre un TRAGUARDO NUOVO,
     e la sequenza è brainstorming → disegno → piano → esecuzione
     subagent-driven. ⛔ A CHE PUNTO SIA quella sequenza NON è scritto qui: è
     uno STATO, e la §6 del compendio ne è la casa unica. Le voci aperte NON
     lo sbarrano — quali, lo dice la tabella unica di
     docs/porta-di-qualita.md, non nominata qui apposta perché un rimando
     non può marcire.
     ⚠️ RICHIAMO DEL 2026-08-28, LA TERZA NELLO STESSO GIORNO: qui stava
     «E SI COMINCIA DAL BRAINSTORMING, subito». Vero la mattina, falso la
     sera: cinque sezioni di disegno erano gia' approvate. TOLTO, non
     riallineato — era la seconda casa di uno stato.
     ⚠️ RICHIAMO DEL 2026-08-28, LA SECONDA NELLO STESSO GIORNO: qui stava
     «MA NON PRIMA CHE TE LO DICA IO, e non prima che le decisioni che lo
     sbarrano siano prese». E prima ancora diceva «NIENTE brainstorming,
     e NIENTE plan mode: la specie del lavoro che viene è RIMEDIO di un audit
     APERTO», e prescriveva di prendere un finding per volta. Era vero fino
     al 2026-08-28 e adesso manderebbe nella direzione opposta a quella
     giusta.

E QUESTE QUANDO SERVIRANNO, NON PRIMA
  /superpowers:brainstorming                 è la PRIMA del traguardo nuovo, e
                                             va PRIMA di entrare in plan mode,
                                             sempre. ⛔ A che punto sia lo
                                             dice la §6, non questa cella
  /superpowers:writing-plans                 quando ci sarà un piano NUOVO da
                                             scrivere, e MAI prima che le voci
                                             aperte siano chiuse — che è la
                                             condizione ancora NON soddisfatta
  /superpowers:subagent-driven-development   quando quel piano ci sarà: un
                                             subagente fresco per compito, con
                                             revisione fra uno e l'altro. È la
                                             modalità scelta, e ha portato TUTTI
                                             i compiti eseguiti finora, senza
                                             un'eccezione: il numeratore vive
                                             nella §6 del compendio e non qui,
                                             perché un cumulativo invecchia a
                                             ogni chiusura. ⛔ E LA
                                             REVISIONE NON È UNA FORMALITÀ: al
                                             Task 6 ha trovato un mutante vivo
                                             nell'INTERO workspace che
                                             l'esecuzione non aveva visto. Anche
                                             un'ONDATA DI CORREZIONI si rivede
                                             come il compito: i suoi difetti
                                             stanno nella PROSA, e «non tocca
                                             righe eseguibili» è un argomento
                                             PER rivederla, non contro. ⛔ SI
                                             RIVEDE FINCHÉ UNA PASSATA NON TORNA
                                             PULITA: il numero di giri non si
                                             decide dalla specie del diff. ✅ E
                                             IL RIMEDIO È MISURATO: all'ondata si
                                             dice TOGLIERE, non riscrivere
                                             meglio, e il giro dopo torna
                                             pulito. ⛔ E LO STATO DI UN COMPITO
                                             SI RIVEDE A HEAD, MAI COMMIT PER
                                             COMMIT: due commit corretti presi
                                             da soli fanno uno stato falso, e il
                                             difetto non sta in nessuno dei due
                                             diff. ⛔ E I RILIEVI SI SCRIVONO
                                             PER SPECIE, non per elenco di
                                             righe: «togli» manda a CERCARE le
                                             altre occorrenze della stessa
                                             forma, «riscrivi» fa guardare solo
                                             il sito indicato
  /superpowers:test-driven-development       quando comincerà il CODICE, che
                                             viene dopo il piano e non prima
  /anthropic-skills:repo-audit               se e quando si aprirà un audit
                                             NUOVO. NON serve ora: quello
                                             dell'11 agosto è chiuso otto su
                                             otto, e quello del 27 è chiuso nei
                                             propri finding dal 2026-08-28.
                                             ⚠️ Questa cella diceva «quel che
                                             resta è eseguirne i rimedi»

LEGGI QUESTI TRE FILE, POI FERMATI — e il TERZO non per intero
  1. CLAUDE.md
  2. docs/COMPENDIO.md — contiene TUTTE le decisioni del progetto: le 37 ADR
     compresse, le sei invarianti, le proprietà non retrofittabili, lo stack
     (§4), lo stato di oggi e il prossimo passo (§6), il non rilitigabile (§7),
     cosa NON rifare (§8), i gotcha (§9 — quanti, lo dice il suo titolo),
     le trappole di check-docs.sh (§10), i vincoli sul primo commit (§11).
  3. docs/audit-2026-08-27.md — ⛔ SOLO FINO ALLA TABELLA DEI 73 FINDING,
     cioè tutto cio' che sta PRIMA della sezione «Dettaglio», 54 KB.
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
     ⚠️ docs/audit-2026-08-11.md — 32 KB — NON si apre più: è chiuso otto
     decisioni su otto, e ciò che insegnava è dentro il compendio.
  ⚠️ In token costano ALMENO il triplo di quel che i KB suggeriscono: misurato,
     quattrocento righe del solo compendio pesano 25148 token, e il compendio
     ne ha oltre quattromila. È un LIMITE INFERIORE, non un totale — §12.

⛔ CODICE IN INGLESE, DOCUMENTAZIONE IN ITALIANO — §1.0 della spec.
   Crate, moduli, tipi, funzioni, messaggi d'uscita e commenti nel sorgente
   sono in inglese; i documenti restano in italiano; un riferimento al codice
   dentro un documento si scrive in inglese, col nome esatto del sorgente.
   Non è tipografia: la regola non stava scritta, e un traguardo intero è stato
   scritto con gli identificatori italiani e poi rifatto.

⚠️ IL REPOSITORY CONTIENE CODICE RUST, e il sotto-progetto 1 è in corso.
   ⛔ Quanti traguardi siano eseguiti e a che punto sia il prossimo lo dice la
   §6 del compendio, in un posto solo: qui non è scritto apposta, perché una
   cifra ricopiata marcisce. ⚠️ E questa riga la ricopiava lo stesso, due
   righe SOPRA la frase che lo vieta — gotcha #68. Tolta, non riallineata.
   Serve rustup: rust-toolchain.toml tira giù da sé la 1.95.0 e il bersaglio
   x86_64-unknown-none, quindi non installare niente a mano. Su Windows serve
   in più il linker MSVC (Visual Studio Build Tools), che rustup NON porta.
   Il primo comando utile è
       bash scripts/gate.sh
   e dice in un colpo se l'ambiente regge: deve stampare GATE GREEN.

✅ I DUE AUDIT COMPLETI SONO CHIUSI NEI PROPRI FINDING — l'11 agosto otto
   decisioni su otto, il 27 agosto tutti i suoi. ⚠️ MA IL SECONDO NON È
   FINITO: restano le voci SENZA numero AUD, e la maggior parte sono
   decisioni MIE. Quali e quante non è scritto qui — la loro tabella nel
   rapporto è la casa unica, e un numeratore ricopiato qui invecchierebbe al
   primo che chiudo: è la radice R3 dell'audit stesso.
   ⚠️ RICHIAMO DEL 2026-08-28, e questo blocco lo prescriveva a sé stesso
   («si riscrive quando l'audit si chiude»): diceva «ED È APERTO. LA SPECIE
   DEL LAVORO CHE VIENE È: RIMEDIO». Adesso la specie è CREATIVA, e il
   rapporto si legge come VERBALE e DELEGA — il metodo, non il compito.

   ⛔ IL METODO RESTA, ED È LA PARTE CHE NON INVECCHIA. Vale per un rimedio,
   per un compito di un piano e per un rapporto letto contro il codice:
   · IL LAVORO SI PREZZA LEGGENDO IL CODICE, NON IL DOCUMENTO CHE LO DESCRIVE
     — può risultare più PICCOLO, più GRANDE, o DI SPECIE DIVERSA. Successo
     su ogni radice chiusa dei due audit, e su ogni compito dispacciato.
   · UNA CORREZIONE PUÒ CRESCERE DI UNA CASA: la frase che stai correggendo
     può vivere anche altrove, e correggerne due su tre è la radice R1
     commessa DENTRO il rimedio. Si cerca col grep prima di chiudere.
   · SI PUÒ FERMARSI PRIMA DI DECIDERE: si chiude il fatto falso e si
     REGISTRA la decisione vera, che è mia. Registrata, non presa.
   · QUANDO UNA FRASE INVECCHIA, GUARDA SE A INVECCHIARE È L'AFFERMAZIONE O
     IL SOGGETTO: sul secondo caso il riflesso «togli la cifra» produce una
     frase PEGGIORE, perché una frase vera di pochi diventa un universale
     falso su molti. Gotcha #87.
   ⚠️ Una quarta riga diceva le stesse parole della prima ed è TOLTA, non
      riscritta meglio — gotcha #76.

⛔ NON aprire docs/HANDOFF.md, la spec del sotto-progetto 1, né la cartella
   docs/adr/ «per farsi un'idea»: insieme pesano oltre mezzo megabyte — 812 KB
   il 2026-08-27, la spec da sola 298, i quattro piani più grandi 373, 168,
   162 e 114 — e l'idea è già nel compendio. ⛔ IL PIÙ GRANDE DEI QUATTRO È
   L'ULTIMO ESEGUITO, e si apre A COMPITI: è cresciuto a ogni compito, perché
   l'errata cresceva.
   Aprirai UN file — uno — quando ti servirà il perché di una decisione: le
   alternative scartate, le misure, i costi accettati. La §12 del compendio
   dice quale.

⛔ IL FORMATO DUREVOLE È CONGELATO. I byte in crates/kernel/tests/frozen/ —
   tre record e una mappa che il banco rilegge — NON si rigenerano mai: se
   cambiano non è un aggiornamento, è un CAMBIO DI FORMATO. Un campo nuovo
   dev'essere Option con #[cbor(default)] e prendere un INDICE NUOVO, e il
   significato di un indice non cambia mai.

⛔ LE DECISIONI PRESE ESEGUENDO, non dai piani, PUOI RIBALTARLE TUTTE:
   NOVE al Traguardo 3, DODICI al Traguardo 4, e l'ultimo piano eseguito ha
   la propria errata, che NON È VUOTA. Stanno nelle errata in testa ai piani,
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
   §8 sono ESEGUITE. ⚠️ MA LA §5 NON E' CHIUSA, e questa riga diceva di sì:
   contate sul rapporto il 2026-08-21, CINQUE voci restano SENZA MARCA —
   T-3, che è dichiarato ALTO, più S-3, A-3, K-4, PL-5 — oltre ai ~20 rilievi
   minori. P-2 era la sesta, ed è chiusa lo stesso giorno; se le cinque siano
   davvero minori NON L'HA MISURATO NESSUNO, ed è la stessa forma con cui P-2
   è sopravvissuto. ⚠️ Più DUE residui registrati: l'asserzione 4b della
   conformità reactor è IMPLICATA dalla 4a, quindi MUTA e non vacua, e le
   DIECI sonde permanenti dell'esecuzione non hanno riga di catalogo, perché
   la §7.4 è SPEC (vincolo globale 7). La tabella sta in porta-di-qualita.md.
   ⛔ E il terzo posto è IL PIANO DEL TRAGUARDO 5, che ne porta due specie.
   Dal proprio PRE-CONTROLLO, SETTE voci di cui TRE sono decisioni tue — un
   nome di tipo a una lettera da uno che esiste già, una riga di catalogo che
   vale per due regole, e due celle che nominano identificatori ITALIANI che
   il traguardo fa esistere in inglese: toccano la SPEC, registrate non prese.
   Dall'ESECUZIONE, altre SEI decisioni di disegno nell'errata — TRE sono
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
    nel piano più spesso che nel codice — in TUTTI i compiti dispacciati
    finora, senza un'eccezione, su piani scritti il giorno prima. Il conto
    per traguardo sta nella §6 del compendio, non qui: un cumulativo
    invecchia a ogni compito, «tutti, senza eccezione» no.
    E SEI voci d'errata erano difetti di un compito SUCCESSIVO, trovati
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

Parti confermandomi cosa hai letto e qual è la tua lettura dello stato, e POI
RIPRENDI DA DOVE LA §6 DICE: non c'è un piano da eseguire né uno da scrivere,
e il brainstorming è il primo passo del traguardo nuovo — ma se sia da aprire
o da riprendere lo dice la §6, non questa riga.
⛔ RICHIAMO DEL 2026-08-28: qui stava «POI APRI IL BRAINSTORMING», e il file
   dichiara di sé, dodici righe più su, di NON nominare il prossimo passo.
   Era una casa di troppo, ed è diventata falsa la sera stessa.
⛔ RICHIAMO DEL 2026-08-28: qui stava «non parte finché non lo dico io, perché
   ciò che lo sbarra sono decisioni MIE». Quale voce sbarri il traguardo è uno
   STATO, e lo stato ha una casa sola: la tabella di docs/porta-di-qualita.md.
I piani già eseguiti restano
verbali, che si leggono a COMPITI e mai interi — e lo stesso vale per il
«Dettaglio» del rapporto, che si legge a FINDING.
⚠️ Da dove ripartire lo dice la §6 del compendio, non questo messaggio.
   ⛔ Questa riga diceva «lo dice il rapporto», ed era vera finché il compito
   era rimediarlo.
```

---

## Perché è così corto

| | Prima | Adesso |
|---|---|---|
| il messaggio | ~9 KB | ✅ **LA SERIE HA GIRATO, e quanto pesa OGGI non è scritto qui:** lo rimisura la **§12 del compendio** a ogni passata, e questa cella ne teneva una seconda copia **ferma al 2026-08-18** — *«14,7 KB»* — che nessuna delle tre passate successive aveva mosso. **Tolta, non riallineata** (gotcha **#68**). La storia resta perché è storia: era 7,7 → 9,8 → 12,2 → 14,7 → 15,0 → 15,4 → 15,9 → **16,7 KB**, sette crescite e un solo fermo; poi il 2026-08-18, chiudendo l'audit, il blocco è **calato per la prima volta** — `16659 → 14947 byte`, **−10,3 %** — e la passata di coerenza dello stesso giorno lo ha riportato a **15014**, `+67 B` e una riga: la crescita più piccola mai registrata. ⛔ **Ha funzionato la prescrizione della 25ª misura, ed è la parte da ricordare:** *«decidere cosa TOGLIERE, non cosa accorciare»*. Tolto il blocco delle ventuno decisioni ribaltabili — che era un **rimando a due errata già scritte** ricopiato per esteso — e compresso a due righe il finding V6, che nel frattempo era **chiuso**. ⚠️ Ciò che si toglie è un **rimando duplicato** o una voce **chiusa**, mai una lezione: quelle si spostano nel compendio, dove chi legge le trova comunque |
| lettura che ordinava | l'intero corpus, oltre mezzo megabyte | ✅ **una FRAZIONE del corpus, e l'aggregato non vive più qui:** lo porta `CLAUDE.md`, in una casa sola. ⛔ **Tolto il 2026-08-27, non riallineato** — la cella diceva *«590 KB … e 622 con l'audit»*, e il terzo file è cambiato: l'audit dell'11 agosto è uscito dalla lettura obbligatoria, la **testa** di quello del 27 è entrata. Una cifra che vive in **cinque** case marcisce in quella che nessuno muove — gotcha **#31**, su questa riga per l'ennesima volta |
| decisioni note all'agente | tutte, dopo aver letto tutto | **tutte**, e senza aprire il resto del corpus |

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
resta quello che conta, e le due cifre vive stanno in `CLAUDE.md` invece che qui — **tolte il
2026-08-27**, perché il terzo file è cambiato e questa era la quinta casa. ⛔ **E il 2026-08-18 una passata ha trovato che il
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
obbligatori quando uno dei due cresce; i
prerequisiti d'ambiente quando il repository comincia a richiedere uno strumento nuovo —
è successo con `rustup`, e prima di allora non serviva niente. ⛔ **Lo SHA non è in questo
elenco**, ed è la voce che è stata tolta: si compila incollando, non si mantiene.
⛔ **E il 2026-08-21 ne è stata tolta una SECONDA: *«il numero dei gotcha quando §9 ne guadagna
uno»*.** Quel numero **non vive più qui** — la 45ª misura lo aveva tolto dal messaggio, che ora
rimanda al titolo della §9 — quindi l'elenco prescriveva di mantenere una cifra **inesistente**.
📌 *Una voce di manutenzione sopravvive alla cosa che manuteneva*, e nessun controllo la coglie:
non è un numero stantio, è un **compito** stantio. Trovata dal pre-controllo della delega, che
legge questo file contro il repository come ogni altro documento (gotcha **#58**).
⛔ **E il 2026-08-25 ne sono state tolte una TERZA e una QUARTA, dallo stesso elenco.**
① *«il **numero dei traguardi eseguiti** quando uno si chiude»*: quel numero **non vive più
qui** — la chiusura del Traguardo 5 lo ha tolto dal messaggio, perché stava due righe sopra la
frase che lo vieta, e ci ha messo un rimando alla §6. ② *«il **peso del messaggio** quando il
blocco fra le due recinzioni cresce — è la cella che nessuno rifaceva, ed era falsa di metà»*:
quella cella è la riga *«il messaggio»* della tabella *«Perché è così corto»*, **svuotata il
2026-08-18** e oggi un rimando alla §12 del compendio, quindi **in questo file** non c'è più
nessun peso del messaggio da rifare: lo rimisura la §12, insieme alla cella della §11 che lo
porta. 📌 **TRE voci di questo solo elenco sono state tolte per la stessa ragione** — il numero
dei gotcha, il numero dei traguardi, il peso del messaggio — **e tutte e tre nella stessa
direzione:** la cosa manutenuta sparisce, la voce che la manuteneva resta, perché chi **toglie**
un valore da un documento non rilegge l'elenco che prescriveva di mantenerlo — e togliere è la
cura che questo repository applica più spesso. ⚠️ **La quarta voce tolta da qui, lo SHA, è di
specie diversa:** quel campo esiste ancora, e a mancare era il modo di tenerlo aggiornato. ⛔ **La domanda che le coglie, e va fatta a ogni chiusura:**
*ciò che questa voce ordina di aggiornare esiste ancora?*

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
