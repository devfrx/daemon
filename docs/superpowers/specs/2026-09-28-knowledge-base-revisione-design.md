# Knowledge base, la revisione: il disegno

✅ **DISEGNO SCRITTO E RILETTO il 2026-09-29**, lo stesso giorno in cui è stato aperto. ✅ **Riletto dal proprietario il
2026-09-29**, in chat, sotto accettazione condizionata: cinque voci poste una per volta in A/B, col consiglio scritto — la
sezione *«La rilettura del proprietario»*, in fondo —, e il consiglio scelto **cinque volte su cinque**; alla voce 2 il
consiglio ribaltava la decisione 2 del coordinatore della 6.1. ⚠️ Questa riga diceva *«resta la rilettura del
proprietario, per intero — la 6.6»*, ed è riscritta sul posto, sul precedente del disegno del 2026-09-04. Si è scritto
**sezione per sezione**: ciascuna si presenta in chat a parole semplici, il proprietario la
approva, e solo dopo si scrive qui — `CLAUDE.md`, *«Sezione per sezione»*. La scaletta l'ha approvata il proprietario il
2026-09-29, e questa tabella è la **casa unica** dello stato delle sezioni:

| # | Sezione | Stato |
|---|---|---|
| 1 | Il modello nuovo, in una pagina | ✅ approvata il 2026-09-29 |
| 2 | Il disegno del 2026-09-04: che cosa si corregge | ✅ approvata il 2026-09-29 |
| 3 | Gli ADR: i rimandi in testa, e l'ADR nuovo del backup | ✅ approvata il 2026-09-29 |
| 4 | La porta dei file e il codice che crescerà: chi costruisce che cosa | ✅ approvata il 2026-09-29 |
| 5 | Roadmap, tracciabilità, stella polare della GUI e design/09, col perimetro del 13 riletto | ✅ approvata il 2026-09-29 |
| 6 | I controlli per artefatto, verificato-dedotto-assunto, le voci aperte e il prossimo passo | ✅ approvata il 2026-09-29 |

Nessuna riga porta più ⏳: chi riprende comincia dalla 6.6. L'ingresso di ciascuna sezione è stato l'elenco *«Che cosa le
risposte cambiano»* della consegna, in archivio, **riletto** contro i documenti di adesso e non eseguito a scatola chiusa.

Questo file era la **consegna** del brainstorming della revisione, chiuso il 2026-09-29: è riscritto **sul posto**, allo
stesso percorso, perché il puntatore della §6 del [compendio](../../COMPENDIO.md) non cambi casa — come il
[disegno del 2026-09-04](2026-09-04-knowledge-base-design.md). La consegna sta **parola per parola** in
[`archivio/consegna-brainstorming-knowledge-base-revisione-intera.md`](../../archivio/consegna-brainstorming-knowledge-base-revisione-intera.md):
le domande D1–D20 come furono poste, coi cinque criteri e le fonti lette; i buchi K1–K53; i tre controlli; l'elenco di ciò
che le risposte cambiano. È lì il **perché** di ogni riga di questo disegno, e ci si va con una domanda in mano, non per
farsi un'idea.

⚠️ **Non è una spec, e da solo non cambia niente.** Il disegno approvato della knowledge base resta quello del
2026-09-04: questo ne **corregge** le righe che le risposte hanno superato, e dice che cosa va negli ADR — rimandi in
testa, e un ADR nuovo —, nella roadmap, nella tracciabilità, nella stella polare della GUI e in design/09. Le righe le
scrive il **piano dei documenti**, in una sessione sua; il codice lo scrivono i sotto-progetti, ciascuno col suo disegno.
⛔ E **non disegna la capacità**: il compendio, §8, lo vieta prima del sotto-progetto 6.

📌 **Metodo.** Ogni affermazione porta la sua specie — **verificata**, **dedotta**, **assunta** — e ogni cifra il comando
che la rifà. Le decisioni sono del proprietario e stanno nella tabella qui sotto; il disegno le traduce, e dove una
traduzione ne cambierebbe il merito si ferma e chiede.

**Le regole di questo lavoro**

| Regola | Da dove viene |
|---|---|
| ogni scelta si controlla **esplicitamente** sui cinque criteri di `anthropic-skills:decision-principles`, con verificato, dedotto e assunto separati. È un'**accettazione condizionata**: se una scelta li viola ci si ferma e lo si dice | la richiesta del proprietario del 2026-09-28, ripetuta all'apertura del disegno e all'approvazione della sezione 1, il 2026-09-29 |
| una sezione alla volta, prima a parole semplici e poi lo schema; le domande una alla volta, in A/B, col costo di ciascuna e il consiglio | [`CLAUDE.md`](../../../CLAUDE.md) |
| l'elenco della consegna è un **ingresso**, non un ordine: si rilegge contro i documenti di adesso, e ciò che manca si aggiunge | `CLAUDE.md`, *«Un piano è un'ipotesi»*; la skill `anthropic-skills:session-resume`: una lista ereditata si ri-deriva |
| il disegno non tocca gli altri documenti: li corregge il piano dei documenti, dopo | `CLAUDE.md`, *«Una fase per sessione»*; il precedente del 2026-09-04 |
| dove i software di oggi hanno una risposta, la si legge alla fonte e la si adotta; al proprietario resta ciò che urta una decisione del progetto | la consegna del proprietario del 2026-09-29, a D9 e a D11 |
| codice in inglese, documentazione in italiano; nessun numero senza comando; nessuna fonte senza data | `CLAUDE.md` |

## Le risposte del proprietario, D1–D20

⛔ **Sono sue, e non si riaprono senza di lui.** Copiate **parola per parola** dalla consegna, dove ogni domanda sta
com'era posta, col costo delle due risposte. ⚠️ Nella consegna la riga D20 era fusa col titolo che la seguiva, e perdeva
la cella della data: qui è intera; l'archivio la tiene com'era, perché è un verbale.

| # | Risposta | Data |
|---|---|---|
| D1 | ⛔ **respinta**: il proprietario risponde col suo documento, riportato nella sezione *«Il documento del proprietario»*; le sue decisioni aperte 2 e 3 si accolgono come le propone | 2026-09-28 |
| D2 | ✅ **A** — la copia di ADR-0024 resta, **solo** per le azioni dell'agente, e il giornale resta; nessuno storico dei cambi del proprietario, nessun versioning, nessun sync. Nessun ADR cambia. ⚠️ **2026-09-29, controllo finale, CF12:** con D14 la copia vale per ogni scrittura **del programma**, il riconciliatore compreso | 2026-09-28 |
| D3 | ✅ **A** — due specie di zona: la knowledge base, una e mappata, e le zone di lavoro, aperte come in Claude Desktop anche fuori dalla root, ciascuna col permesso per la sessione e la copia prima delle modifiche dell'agente; una zona **fuori** dalla root **non** entra nel grafo né nella ricerca, e nella knowledge base c'è la sua **scheda progetto**, che le fa da router; fuori da ogni zona l'agente non legge e non scrive. Posta il 2026-09-28, riformulata lo stesso giorno ⚠️ **2026-09-29, D18:** fuori da ogni zona l'agente **non scrive**; **legge** col permesso, come Claude Code | 2026-09-29 |
| D4 | ✅ **A** — separati per natura: i **router** in `.<nomeapp>/` alla root, nascosta dal modulo di piattaforma, e l'**indice** nella cartella dati del programma, fra i dati rigenerabili, uno per root; i dati del programma nella cartella dati per utente del sistema, e il programma salva nel proprio backup i suoi dati e i router ⚠️ **2026-09-29, la revisione:** la metà sul backup urta ADR-0022, e torna al proprietario come **D12** — ✅ A, 2026-09-29 | 2026-09-29 |
| D5 | ✅ **A** — due specie: il **rumore**, dove lo scanner non entra ma il file o la cartella restano un nodo del grafo e l'agente li apre se serve; il **privato**, fuori dall'indice e da ciò che l'agente vede, con la porta che rifiuta la lettura e il confinamento dei comandi che nega quei percorsi. Le regole del privato le cambia solo il proprietario: l'agente propone, e ciò che rende leggibile qualcosa chiede conferma a ogni preset ⚠️ **2026-09-29, D16 e D20:** il file è un percorso protetto, e anche una riga di rumore l'agente la propone soltanto | 2026-09-29 |
| D6 | ✅ **A** — nel dubbio, **rotto** e una domanda: il caso certo — la stessa impronta, un solo candidato — si applica da solo; un file chiave che il riconciliatore non ritrova con certezza resta nella mappa segnato rotto, l'agente non lo segue, il pannello lo mostra, e il riconciliatore propone i candidati — lo stesso nome altrove, o un contenuto simile come fa git — fra cui sceglie il proprietario; due file identici sono un caso di dubbio | 2026-09-29 |
| D7 | ✅ **A** — un sì per la sessione: la tripla `(file, .<nomeapp>/, scrittura)` si concede alla prima modifica di un router e vale per la sessione di D11, come il punto 3 di ADR-0016; ogni modifica si vede nel turno e si annulla. Alla prima posa il proprietario aveva risposto con una domanda — la sessione non era definita, K37 —, e D7 si è riposta dopo D11. Con la risposta ha chiesto se la sessione sia già integrata nei permessi: **non ancora**, e dove va lo dice la sezione di D11 | 2026-09-29 |
| D8 | ✅ **A** — una terza specie di linea: la scansione, senza modello, legge i link dei file di testo — `[testo](percorso)` e `[[nota]]` — e ne fa linee del grafo; un link verso un file che non c'è è un segnale **rotto**; un link che esce dalla root punta alla scheda della sua zona, o non si disegna; i filtri — per specie di linea, per area, per gli orfani — al 6 | 2026-09-29 |
| D9 | ✅ **A** — lo stato dell'arte: la fiducia si chiede una volta per zona e sta nel giornale; una zona non fidata va in modalità ristretta, coi file-guida non caricati da soli; in una zona fidata i file-guida si caricano a ogni sessione, anche quando cambiano, con l'impronta della versione caricata nel giornale; un file-guida non concede permessi; un import che esce dalla zona chiede la sua approvazione. La regola 3 e la pretesa 1.1e del disegno del 2026-09-04 ricevono un richiamo datato col disegno di questa revisione: per una guida di zona l'approvazione è la fiducia alla cartella. Posta il 2026-09-29 e riformulata sullo stato dell'arte lo stesso giorno | 2026-09-29 |
| D10 | ✅ **A, come Claude Desktop** — *«la selezione del modello deve funzionare come in cladue desktop (sia parte coding agentico che chat/cowork)»*: il proprietario sceglie il modello col selettore, per la sessione o come default, e i sotto-agenti lo prendono; la catena di riserva la percorre il nostro gateway, un modello per richiesta a OpenRouter, con un avviso, e ogni risposta porta il nome del modello; un ripiego per disponibilità dura il turno, uno per contenuto resta; la proiezione si compone per ogni candidato, un candidato che non la tiene si salta, e la compressione di OpenRouter si spegne. Nessun ADR cambia; K47 e K48 registrati | 2026-09-29 |
| D11 | ✅ **delegata allo stato dell'arte** — *«come le sessioni moderne delle app moderne stato dell'arte, decision-principles devi seguire»*: la sessione è la run coi suoi sotto-agenti, uguale sul lato chat e sul lato coding; finisce quando il proprietario la chiude, dopo un tempo di inattività o dopo un tempo massimo, e la fa rispettare il core; i due tempi sono parametri consegnati, coi valori al 3 e il riferimento di NIST AAL2; alla fine cadono i suoi sì e si chiudono le sue zone. Il sì oltre la sessione delle app di oggi urta ADR-0016: segnalato, K38 | 2026-09-29 |
| D12 | ✅ **A, delegata allo stato dell'arte** — *«stato dell'arte, segui quello»*: la root è del proprietario, e il suo backup pure, come in Obsidian; il programma salva il giornale, la configurazione e i router — mai i segreti, e non l'indice che si rifà —, e quando crea il backup dice che cosa resta fuori. Un ADR nuovo supererà, per i file della root, le righe degli artefatti e delle guide di ADR-0022 e la sua conseguenza sulla base di conoscenza: lo scrive il disegno di questa revisione ⚠️ **2026-09-29, D17:** l'ADR nuovo vale per **ogni** file del proprietario, la root e le zone di lavoro | 2026-09-29 |
| D13 | ✅ **A** — un file che l'agente scrive in una zona fuori dalla root sta nell'**anello**, per data, dal giornale; nella **rete** la zona è la sua scheda, e da lì si apre; un file che non c'è più si mostra mancante quando lo si apre. La decisione 1 della stella polare della GUI riceverà un richiamo col disegno: la rete ha tutto ciò che sta **nella root**. Riposta con un esempio: alla prima forma il proprietario aveva risposto *«non ho capito spiega meglio»* | 2026-09-29 |
| D14 | ✅ **A, sullo stato dell'arte** — come le app di oggi, il riconciliatore segue un'**impostazione** scelta una volta — da solo, chiedi, mai —, e parte da **«da solo»**, come Obsidian e come D6; ogni correzione va nel giornale, con la copia, e si annulla. ADR-0016 riceverà un rimando col disegno: il permesso lo chiede chi agisce per un modello o invoca una funzione, e la manutenzione deterministica del programma sulla sua cartella segue la sua impostazione. Alla prima forma il proprietario aveva chiesto *«come farebbero con lo stato dell'arte attuale?»*: lette alla fonte Obsidian e VS Code. ⚠️ **2026-09-29, controllo finale, CF5:** il perimetro si scrive *«la correzione deterministica di un fatto che non cambia una scelta del proprietario»*, e non *«la cartella del programma»* — i router sono del proprietario, D4 —; e il rimando va anche in ADR-0038, la cui regola 2 vuole lo stesso permesso per ogni invocatore | 2026-09-29 |
| D15 | ✅ **A** — a pezzi: la porta dei file vera la paga chi usa ciascun pezzo per primo — il **13** la lettura, la sorgente degli eventi e, fuori dalla porta, la finestra del candidato nel gateway; il **6** lo scrivere, anche condizionato, l'elenco, i metadati, spostare, cancellare, le esclusioni del privato e la tripla su una cartella scelta a runtime; il **5** la chiusura delle zone. design/09 e la roadmap si riscrivono col disegno; ogni crescita della porta è un richiamo datato alla §4 della spec del sotto-progetto 1 ⚠️ **2026-09-29, secondo controllo, F1 e F8:** anche i metodi che ci sono si ripartiscono — il 13 dichiara l'ambito, il 6 conserva e ripristina —; la finestra è di `gateway::Candidate`, non della porta | 2026-09-29 |
| D16 | ✅ **A** — come Claude Code: il file del privato è un **percorso protetto**, controllato dal kernel **prima** dei sì; nessun sì — sulla root, su una cartella, di sessione — copre una scrittura dell'agente su di lui, e l'agente può solo proporre; lo cambia il proprietario, a mano o dal pannello, e ogni cambio che rende leggibile chiede conferma a ogni volta, perché è irripetibile — RR8. Lo costruisce il 6, senza aspettare i preset del 4. Chiude K26 ⚠️ **2026-09-29, secondo controllo, M1, M2, M5:** più stretta di Claude Code — il suo `dontAsk` —; il controllo lo fa la **porta**, non il registro; e *«a ogni volta»* è un'eccezione al punto 3 di ADR-0016, col rimando e la modifica di `invoke` nell'elenco | 2026-09-29 |
| D17 | ✅ **A** — nessun file del proprietario nel backup del programma: l'ADR nuovo di D12 supera le righe degli artefatti e delle guide di ADR-0022 per **ogni** file del proprietario, la root **e** le zone di lavoro; il file sta al suo posto, il giornale lo riferisce, la copia per annullare di D2 resta; la storia lunga è dei backup del proprietario e di git, come in Claude Code | 2026-09-29 |
| D18 | ✅ **A** — come Claude Code: fuori da ogni zona l'agente **legge col permesso** — la tripla `(file, percorso, lettura)` di ADR-0016, per la sessione —, e la porta apre per quel file un ambito di sola lettura; la **scrittura** fuori resta `OutsideScope`; la lista di base del privato e i percorsi protetti valgono anche lì; l'import che esce da una zona, D9, è la stessa domanda; un'impostazione blocca ogni lettura fuori. Cambia la metà «leggere» di D3 | 2026-09-29 |
| D19 | ✅ **A** — la cartella dati del programma è un **percorso protetto**, come il file del privato di D16 — l'agente non ci scrive mai, qualunque sì abbia —, e sta nella **lista di base del privato**: non si indicizza e non si legge. Chiude K50 | 2026-09-29 |
| D20 | ✅ **A** — un file solo, tutto protetto: il file delle esclusioni del documento del proprietario resta uno, alla root, con le due sezioni di D5; l'agente **propone** anche le righe di rumore, e le conferma il proprietario. | 2026-09-29 |

## 1. Il modello nuovo, in una pagina — ✅ approvata il 2026-09-29

Che cosa è **deciso** dopo D1–D20. ⚠️ **Deciso non vuol dire costruito:** quasi niente di questo esiste nel codice — oggi
il daemon scrive `journal.redb` e `layout.redb` nella cartella da cui parte, e la porta `filesystem` non ha
un'implementazione vera —; chi costruisce che cosa lo dice la sezione 4. Che cosa cambia rispetto al disegno del
2026-09-04 lo dice la sezione 2.

### 1.1 Dove stanno le cose

| Posto | Che cosa c'è | Di chi | Il backup | Da |
|---|---|---|---|---|
| **la root** — una cartella qualsiasi, anche quella in cui il proprietario tiene tutto; il percorso sta nella configurazione | i file del proprietario: note, guide, catture, e i file che le run vi producono | del proprietario | suo, coi suoi strumenti | il documento; D12 |
| **`.<nomeapp>/`**, alla root, marcata nascosta dal modulo di piattaforma — su Windows il punto nel nome non basta | il router master e gli indici d'area | del proprietario: portano le sue scelte, e non si rifanno senza il setup guidato | anche del programma | D4; D12; K30; K32 |
| **la cartella dati del programma** — `%LOCALAPPDATA%\<nomeapp>\` su Windows, le cartelle XDG su Linux, in sottocartelle per natura | il giornale; la configurazione, con la root, la catena dei modelli e la disposizione dei pannelli; l'indice del livello strutturale, uno per root | del programma, ed è un **percorso protetto e privato**: l'agente non ci scrive, non la legge, e non si indicizza | del programma, per ciò che non si rifà: il giornale e la configurazione; l'indice no, si rifà; i segreti **mai** | D4; D19; ADR-0022 |
| **le zone di lavoro** — cartelle che il proprietario apre per lavorarci, come in Claude Desktop, anche fuori dalla root | le repo e i progetti, e i file che l'agente vi scrive | del proprietario | suo e di git | D3; D17 |

✅ **Dove stanno le copie** che il checkpoint tiene prima che l'agente cambi un file: nella cartella dati del programma, in
chiaro, **fuori** dal backup, potate come vuole ADR-0018 — K53, deciso da ADR-0040 nella 3.2, punto 5. ⚠️ **Richiamo del
2026-09-29, dalla sezione 6:** questa riga diceva *«non è deciso»*, ed era scritta prima della sezione 3.

### 1.2 Chi scrive

| Chi | Come | Da |
|---|---|---|
| **il proprietario** | da fuori, con qualunque strumento, anche un altro agente: il programma non assume di essere il solo a scrivere | il documento |
| **l'agente** | da dentro, nella root e nelle zone aperte. Ogni scrittura è un effetto giornalato, con la copia prima, e si annulla. Quando crea, sposta o modifica un file aggiorna i router nello stesso turno, con un sì per la sessione sulla cartella dei router. Prima di scrivere controlla che il file non sia cambiato dall'ultima lettura, e se è cambiato si ferma. Cancella solo in modo **morbido** — nel cestino di sistema — e con conferma | il documento; D2; D7; K10; la decisione aperta 3 del proprietario |
| **il riconciliatore** | senza modello, all'avvio e sugli eventi del sorvegliante. Toglie la voce di un file cancellato, e segue uno spostamento quando l'impronta è la stessa e il candidato uno solo; nel dubbio la voce **resta**, segnata **rotta**, coi candidati, e sceglie il proprietario. Segue un'impostazione — da solo, chiedi, mai — che parte da «da solo»; ogni correzione è giornalata, con la copia | il documento; D6; D14 |
| **il pannello** | il click invoca le funzioni del registro, le stesse che invoca il modello, con lo stesso permesso: *«senza l'agente»* vuol dire senza il **modello** | il documento; ADR-0038 |

Lo stato **derivato** — l'indice e il grafo — si riallinea in due momenti: il **sorvegliante**, mentre il programma è
acceso, e la **scansione** di riconciliazione all'avvio; il sorvegliante sa dire *«ho perso eventi, riscansiona»*, e
un'indicizzazione in corso si dichiara prima — ADR-0019. I router li corregge il riconciliatore, qui sopra. **Nessuno
storico** dei cambi del proprietario, nessun versioning, nessun sync: una knowledge base per installazione — il
documento, D2, K5.

### 1.3 Come si trova un file

**Il setup**, idempotente e coi soli percorsi relativi alla root: prima la **scansione**, che fa il livello strutturale
senza modello; poi il **setup guidato** — l'agente chiede in una volta le aree di lavoro, i file più usati e che cosa è
privato, **propone** le aree, il proprietario le conferma, e l'agente scrive il router master e gli indici d'area. È il
documento, con la sua decisione aperta 2 accolta com'era.

**I due livelli.** Lo **strutturale** è l'indice completo — albero, metadati, testo per la ricerca —, senza modello e
sempre riallineato. Il **semantico** è il router master, che nomina le aree e punta a un indice per area; l'indice elenca
i file chiave, una riga l'uno, sotto una pagina: puntatori, non un inventario. Un'area non è una cartella.

**La strada dell'agente**, e il piano del 2026-09-04 che ciascun passo usa:

| # | Passo | Il piano |
|---|---|---|
| 1 | legge il router master e sceglie l'area | **0**: il kernel lo carica sempre, per chiave; per una zona di lavoro la chiave è la sua scheda progetto |
| 2 | legge l'indice dell'area: un file chiave si trova qui, al primo salto | **1**: un salto giornalato, col suo costo |
| 3 | cerca nel livello strutturale, nelle cartelle dell'area | la ricerca testuale senza modello — **nuova**, con la prima metà del 6 |
| 4 | cerca in tutta la knowledge base, per ultimo | idem |

Il costo in token dipende dalla strada fatta, non da quanto è grande la cartella. La ricerca per **somiglianza** — il
piano 2 del 2026-09-04 — resta alla seconda metà del 6. Un file nuovo entra **subito** nel livello strutturale, e in un
router solo se il proprietario o l'agente lo promuovono a file chiave.

### 1.4 I confini

| | Dentro la root e le zone aperte | Fuori da ogni zona |
|---|---|---|
| **scrivere** | col permesso: col preset di default chiede, e un sì vale la sessione; un effetto **irripetibile** chiede a ogni invocazione | **mai**: la porta risponde `OutsideScope` |
| **leggere** | procede, col preset di default | chiede, **un file alla volta**, e il sì vale la sessione; un'impostazione blocca ogni lettura fuori |

| Il confine | Che cosa dice | Da |
|---|---|---|
| l'ambito | quello della root c'è sempre, dalla configurazione; una zona si apre col permesso, per la sessione. Niente `..`, niente collegamenti simbolici che escono | il documento; D3; D11; K14 |
| il privato | una **lista di base**, comune a tutte le zone e che non si toglie — `.env`, le chiavi, `.ssh`, la cartella dati del programma —; più il **file delle esclusioni** alla root, in due parti: il **rumore** — `.git`, `node_modules`, i binari, i media pesanti —, che lo scanner non apre ma che resta un nodo del grafo, e che l'agente apre se serve; e il **privato**, fuori dall'indice e da ciò che l'agente vede, con la porta che ne rifiuta la lettura. Una zona ha la lista di base, le regole del proprietario per quella zona, e le esclusioni della repo, che possono solo **aggiungere** privato | D3; D5; D19; K41 |
| i percorsi protetti | il file delle esclusioni e la cartella dati del programma: nessun sì copre una scrittura dell'agente su di loro; l'agente **propone** — anche una riga di rumore — e il proprietario cambia; un cambio che rende leggibile qualcosa chiede conferma a ogni volta | D16; D19; D20 |
| i file-guida delle repo | `CLAUDE.md`, `AGENTS.md` e simili: la **fiducia** si dà una volta per zona e sta nel giornale; in una zona non fidata non si caricano da soli; in una fidata si caricano a ogni sessione, anche quando cambiano, con l'impronta della versione caricata nel giornale; non concedono mai permessi; un import che esce dalla zona chiede il suo sì. Le skill della knowledge base restano ad AUD-004 | D9; D18 |

⚠️ **Il limite, dichiarato.** La porta non vede un comando che l'agente esegue, né un server MCP: per loro il privato e i
percorsi protetti li tengono il confinamento di livello 2, che porta il 5, e il 4, che porta MCP — K36, K12.

### 1.5 La sessione

È la **run** che il proprietario apre — una conversazione, o un compito dell'agente — coi suoi **sotto-agenti**, uguale
sul lato della chat e su quello del coding. Finisce alla prima di quattro cose: il proprietario la **chiude**; passa un
tempo di **inattività**; passa un tempo **massimo**; il **core** si riavvia — il core, non la finestra: la GUI è
sacrificabile, ADR-0004. La fa rispettare il core; i due tempi sono parametri consegnati, coi valori al 3 e il riferimento
di NIST AAL2. Quando finisce cadono i suoi sì e si chiudono le sue zone; la run resta nel giornale e si riprende, e chi
riprende chiede di nuovo — D11, RR3.

### 1.6 Il grafo e il pannello

- Ogni file **indicizzato** è un nodo — il rumore compreso, il privato no.
- Tre specie di linea: la **cartella**, l'**area**, e i **link** che il proprietario scrive, `[testo](percorso)` e
  `[[nota]]` — D8. Il segnale **rotto** per un link verso un file che non c'è e per un file chiave che il riconciliatore
  non ritrova — D6, D8. I filtri per specie di linea, per area, per gli orfani.
- Una zona fuori dalla root è **un** nodo, la sua scheda; i file che l'agente vi scrive stanno nell'**anello** della
  Home, per data, dal giornale — D3, D13.
- Il pannello: grafo o griglia, raggruppati per cartella e per area; ricerca mentre si scrive, coi filtri per tipo e per
  cartella; al click l'anteprima e il percorso, con un pulsante per copiarlo; tutte le CRUD, attraverso il registro; e
  si aggiorna da solo quando la cartella cambia, anche da fuori — il documento.
- Un file «solo online» — un segnaposto di OneDrive — lo scanner non lo apre, e resta un nodo; lo apre chi lo chiede —
  K34.

### 1.7 Il modello

Lo sceglie il proprietario, col selettore accanto al pulsante di invio, per la sessione o come default; i sotto-agenti
lo prendono, salvo quello scritto nella loro definizione. La **catena di riserva** la scrive il proprietario nella
configurazione — senza catena, nessun ripiego —, e la percorre il nostro gateway, un modello per richiesta a OpenRouter,
con la sua compressione **spenta**. Un ripiego per disponibilità dura il turno, uno per contenuto resta; i limiti di
frequenza non fanno ripiegare, e su OpenRouter resta da misurare — K48. La proiezione si compone **per ogni candidato**,
e un candidato che non la tiene si salta. Un avviso quando il modello cambia, e ogni risposta porta il nome del modello
che l'ha data — D10.

### 1.8 Che cosa non cambia

| | |
|---|---|
| la **strada B** | la knowledge base resta una capacità L2 nel 6; il kernel non sa che cosa siano un router o un'area — ADR-0001 |
| l'**ordine** | 2, poi 13, poi 3 — la decisione 16 del 2026-09-04; il 2 è chiuso, lo dice la sua riga in [`roadmap.md`](../../roadmap.md) |
| **AUD-004** | sbarra ancora il 13: l'ADR del proprietario sulle skill |
| **nessuna sesta proprietà** della §3 del compendio | ⚠️ **dedotto**: ogni pezzo nuovo si aggiunge senza rifare ciò che c'è — un campo su un indice nuovo del giornale, ADR-0036; una variante nuova della porta, col richiamo datato alla spec. Il **vincolo d'ordine** resta, e il 13 cresce: la sezione 5 |

### 1.9 Regge crescendo? E la verifica chiesta dall'approvazione

| Arriva | Che cosa porta, e dove sta la risposta |
|---|---|
| **3** e **4** — le run, più run insieme, i server MCP | la sessione di D11; gli ambiti sono della porta e non della run, K35; MCP, K12 |
| **5** — repo e git | le zone e i due attori; il privato per i comandi, K36 |
| **6** — la mappa e la ricerca | una root enorme, con la scansione incrementale, K29; il router master che cresce con le aree e il grafo con migliaia di nodi, K22 |
| **7** — i file grandi | restano nodi; il limite di dimensione di ADR-0024, K21 |
| **11** — backup e ripristino | l'ADR nuovo e le copie del checkpoint, K53: la sezione 3 |
| **12** — le catture | atterrano nella root, come ogni file nuovo |

**La verifica.** Il proprietario ha approvato la sezione *«se tutto segue i principi … ed è coerente»*. Riletto il testo
presentato in chat contro le risposte e la consegna, quattro frasi erano imprecise e sette cose mancavano; il merito non
cambia.

| Nel testo presentato | Qui | Perché |
|---|---|---|
| *«vero dopo D1–D20»* | *«deciso»* | quasi niente esiste nel codice — M4 della consegna |
| *«ogni file è un punto»* | ogni file **indicizzato**; il privato no | D5 |
| *«i suoi file stanno nell'anello»*, di una zona esterna | i file che l'**agente vi scrive** | D13 |
| *«quando il programma si riavvia»* | quando si riavvia il **core** | RR3; ADR-0004 |
| mancavano | il dubbio del riconciliatore; la lista di base del privato; il limite dei comandi e dei server MCP; il setup; la cancellazione morbida; il controllo prima di scrivere; niente sync | D6; D3 e D19; K36 e K12; il documento; D2 |

## 2. Il disegno del 2026-09-04: che cosa si corregge — ✅ approvata il 2026-09-29

**A parole.** Il [disegno del 2026-09-04](2026-09-04-knowledge-base-design.md) resta il disegno approvato della knowledge
base, e il suo piano dei documenti è eseguito dal 2026-09-05. Ogni riga che le risposte hanno superato riceve un
**richiamo datato** nella riga stessa — `CLAUDE.md`, *«ogni correzione a una sezione approvata porta il proprio richiamo
con la data»* —, che dice che cosa vale adesso, da quale risposta, e rimanda a questo disegno. La riga di prima **resta**
leggibile: dice che cosa la risposta ha sciolto. I richiami li scrive il **piano dei documenti**, non questo disegno.

**Riletto riga per riga.** L'elenco della consegna portava diciassette righe su quel disegno. Riletto contro il testo —
le premesse, le dodici risposte, le sezioni dalla 1 alla 7, i vicoli ciechi —, ne mancavano: sono le righe segnate 🆕, e
le parti 🆕 dentro righe che c'erano.

| # | Dove, nel disegno del 2026-09-04 | Che cosa dice il richiamo | Da |
|---|---|---|---|
| 0 🆕 | la testa | il disegno è corretto dalla revisione del 2026-09-28 e 2026-09-29: ogni riga superata porta il proprio richiamo, e il perché sta in questo disegno | — |

**A. La cartella, e chi la scrive**

| # | Dove, nel disegno del 2026-09-04 | Che cosa dice il richiamo | Da |
|---|---|---|---|
| 1 | le premesse, la risposta 1, §1.1a | non più *«un archivio unico»*: una cartella qualsiasi, anche quella in cui il proprietario tiene tutto, e la root arriva dalla configurazione — ADR-0034. 🆕 *«Progetti, note, tutto dentro»*: una repo **dentro** la root sta nel livello strutturale, meno le esclusioni; una repo **fuori** è una zona di lavoro, con la sua scheda progetto nella knowledge base | il documento; D3; K3 |
| 2 | la risposta 3, §1.1b, il primo punto della §1.3 | non più *«solo il nostro assistente»*: due attori, e il proprietario scrive da fuori con qualunque strumento; cade l'esclusione degli *«altri strumenti che leggono o scrivono la cartella»*. ⚠️ **L'altra metà della risposta 3 resta:** il modello lo sceglie il proprietario, e il routing lo applica — D10 la conferma, col selettore | il documento; D10 |
| 3 🆕 | il vicolo cieco *«anche altri agenti da fuori»* | riaperto dal proprietario col suo documento: i due rischi di allora — router che marciscono per mano altrui, skill riscritte da altri — li reggono il riconciliatore e AUD-004 | il documento; D6; D14 |
| 4 | §2.3, la regola 2 | *«ogni scrittura nella cartella»* si legge *«ogni scrittura del programma»*: il proprietario scrive da fuori, e il riconciliatore scrive come effetto giornalato | il documento; D14 |
| 5 | §1.4, le righe *«chi aggiorna il router»*, *«e se il router marcisce»*, *«e se modifico un file a mano»* | l'agente aggiorna i router nello stesso turno, con un sì per la sessione; il riconciliatore corregge da solo i due casi certi, e nel dubbio segna rotto e chiede; l'anello di miglioramento resta per ciò che si ripete. Una modifica del proprietario da fuori **non si approva**, perché è sua: il riconciliatore riallinea lo stato derivato, e una skill cambiata resta ad AUD-004 | D6; D7; D14 |
| 6 | la risposta 9, §1.1g, §2.3 regola 4, 🆕 la decisione 10 | lo spazio designato non è più uno: la root e le zone di lavoro aperte. Da una zona alla root l'agente **copia**, e il file entra con la provenienza della regola 4; 🆕 fuori da ogni zona **legge** col permesso, un file alla volta; **cancellare** è spostare nel cestino di sistema, con conferma. 🆕 ⚠️ **Dedotto:** *«fuori dallo spazio»*, per chi sposta, vuol dire verso una zona aperta — fuori da ogni zona la porta non scrive per nessuno, nemmeno per il click, perché il permesso è lo stesso per ogni invocatore; verso una cartella qualsiasi, la si apre come zona, o si sposta il file con l'OS | D3; D18; K20; la decisione aperta 3 del proprietario; ADR-0038 |

**B. Come si naviga e si cerca**

| # | Dove, nel disegno del 2026-09-04 | Che cosa dice il richiamo | Da |
|---|---|---|---|
| 7 | le risposte 2, 4 e 10, §4.1 | l'agente cerca anche nel livello strutturale: un file che nessun router punta si trova lo stesso, con un costo in più, e *«per l'agente non esistono»* non regge più. La ricerca **testuale**, senza modello, arriva con la prima metà del 6; quella per **somiglianza** resta dopo | il documento |
| 8 🆕 | il vicolo cieco *«le cartelle come struttura della mappa»* | riaperto dal documento: cartella e area insieme, per il pannello e per la ricerca dell'agente, che si limita alle cartelle dell'area | il documento |
| 9 | §1.1c, 🆕 §1.2, la decisione 15 | l'«ambito» del 2026-09-04 erano **due cose**, e si separano. **Il confine della porta**, ADR-0024, è la root, dalla configurazione, o una zona aperta. **La chiave del piano 0** è opaca per il kernel: per una zona la sua scheda progetto; per la knowledge base il router master, e l'area la sceglie l'agente leggendolo, salvo che la run nasca da un'area. **La forma della risorsa** di un permesso su un percorso scelto a runtime — un identificativo coniato dal kernel — è **registrata**, non decisa: la chiude il 6, K44, con K49 e K51 | D3; RR5; K44 |
| 10 🆕 | la risposta 7 | *«la foto atterra in un gruppo, il router segue»* → la cattura entra nella knowledge base come ogni file nuovo — nel livello strutturale, nel grafo e nella ricerca —, e in un router solo se il proprietario o l'agente la promuovono; la run la vede, come riferimento. ✅ **Deciso dal proprietario il 2026-09-29**: la domanda sta sotto questa tabella | il documento; la risposta qui sotto |
| 11 | §4.2, l'indice | ogni file **indicizzato** è un nodo, il rumore compreso; le frecce del 2026-09-04 stanno nelle tre specie di linea — l'**area**, cioè router master → indice d'area → file chiave; il **link**, che porta anche il ritorno dalla skill al suo router; e la **cartella** —; il segnale rotto anche per un file chiave che il riconciliatore non ritrova, e per un link verso un file che non c'è | D5; D6; D8 |
| 12 | §4.3, il pannello | la ricerca mentre si scrive cerca nel **testo**, coi filtri per tipo e per cartella, e non più solo sui nomi; la griglia accanto al grafo; 🆕 l'**anteprima** al click, accanto al percorso col suo «copia», che c'era già nella §4.1 | il documento |
| 13 | §4.4 | la prima metà del 6 guadagna il livello strutturale, con la ricerca testuale senza modello; la seconda resta per la somiglianza | il documento |

**C. I pezzi del kernel, e i file su disco**

| # | Dove, nel disegno del 2026-09-04 | Che cosa dice il richiamo | Da |
|---|---|---|---|
| 14 | §2.2, la cartella su disco e 🆕 l'indice | la cartella non è più *«nel backup»* per intero: la root sta nei backup del proprietario, e il programma salva i router, che stanno in `.<nomeapp>/`; 🆕 l'indice sta nella cartella dati del programma, uno per root | D4; D12 |
| 15 | §2.2, i trigger | il sorvegliante più la scansione all'avvio; il meccanismo sa dire *«ho perso eventi, riscansiona»*, e un'indicizzazione in corso si dichiara prima — ADR-0019 | il documento; K8; K9; K29 |
| 16 | §2.2 il registro delle guide, §2.3 regola 3, §1.1e, 🆕 la decisione 9 | per il file-guida di una zona l'approvazione è la **fiducia alla cartella**, e l'impronta si scrive a **ogni caricamento**; per le skill della knowledge base decide AUD-004 | D9 |
| 17 🆕 | §2.2 la proiezione, e §1.4 *«come funziona se uso modelli diversi?»* | in un ripiego la proiezione si compone per il **candidato** — la sua finestra e la sua guida —, e un candidato che non la tiene si salta; senza catena di riserva il ripiego non c'è, e il costo resta zero | D10 |
| 18 🆕 | §1.1d, §2.4, la riga 13 della tabella della §5.1 | il vincolo d'ordine resta, e il 13 **cresce**: la lettura e la sorgente degli eventi della porta dei file, la finestra del candidato nel gateway, il «riscansiona», la fiducia di D9, la proiezione per candidato. L'ordine vive nella roadmap: la sezione 5 | D15; D9; D10 |

**D. Che cosa il disegno escludeva**

| # | Dove, nel disegno del 2026-09-04 | Che cosa dice il richiamo | Da |
|---|---|---|---|
| 19 | §1.3 | non regge più *«un record nuovo del giornale, una porta nuova, … regole di backup nuove: niente»*: record nuovi — la fine della sessione, la fiducia —, la porta che cresce, e le regole di backup dell'ADR nuovo. 🆕 E nemmeno *«riaprire ADR-0022»*: l'ADR nuovo ne supera due righe | CF15; D9; D11; D12; D17; K45 |
| 20 | 🆕 l'approccio scelto, §3.1–§3.3, 🆕 §6.4, 🆕 il vicolo cieco *«un ADR nuovo per la knowledge base»* | *«nessun ADR nuovo»* cade con D12: arriva l'ADR del backup, che **non** è un ADR della knowledge base — supera due righe di ADR-0022 per i file del proprietario; la riga *«0022, 0024, 0014: nessuno»* cade, e la sezione 3 dice che cosa ricevono; la riga di I1 si rilegge: la cartella è del proprietario, e il giornale, lo stato autorevole, sta nella cartella dati, che è protetta | D9; D12; D17; D19; K35 |
| 21 | §1.4 la riga del privato, la decisione 13, §4.5 | **chiusa**: rumore e privato nel file delle esclusioni, che è un percorso protetto come la cartella dati del programma; per le zone, K41 | D5; D16; D19; D20; K41 |
| 22 | §6.3 e §7 | *«nessuna fonte esterna»* non regge più: l'idea dei router viene dalla guida ARMS, datata il giorno del disegno; e le fonti della revisione stanno in [`riferimenti.md`](../../riferimenti.md) | la guida ARMS |
| 23 🆕 | la tabella 3.4, la riga «1–7» | la riga riassume le sette risposte — *«archivio a mappa»*, *«solo il nostro assistente»* — ed è la **casa unica** del loro stato: le risposte 1, 2, 3, 4 e 7 sono corrette dalle righe 1, 2, 7 e 10 di questa tabella, e senza il richiamo la riga le direbbe prese com'erano; la 5 e la 6 reggono — ⚠️ **dedotto**. ⚠️ **Aggiunta il 2026-09-29 dalla sezione 6**, la 6.1 | le righe 1, 2, 7 e 10 |

**La domanda di questa sezione, e la risposta**

| | |
|---|---|
| la domanda | la cattura di un gesto: la risposta 7 del 2026-09-04 dice *«la foto atterra in un gruppo, il router segue»*; il documento del proprietario vuole i router parziali per scelta — i file chiave, sotto una pagina — e un file nuovo in un router solo se promosso |
| **A**, il consiglio | come ogni file nuovo: nel livello strutturale, nel grafo e nella ricerca; in un router solo se promossa; la run la vede comunque. Costo: finché non è promossa, non è nella mappa |
| **B** | ogni cattura entra da sola nell'indice di un'area. Costo: l'indice cresce a ogni foto e supera la pagina, contro il documento; e una regola speciale per le catture |
| i cinque criteri | **correttezza**: la risposta 7 e il documento letti; **coerenza**: A segue il documento; **debito**: nessuno per A, la regola speciale per B; **stato dell'arte**: non decide, è una regola del proprietario; **proporzione**: A non costruisce niente |
| verificato, dedotto, assunto | **verificati**: la risposta 7 e il documento; **dedotto**: che una cattura sia un file nuovo per la regola del documento; **assunto**: niente |
| ✅ **la risposta** | **A**, dal proprietario, il 2026-09-29, in chat |

**La verifica chiesta dall'approvazione.** Il proprietario ha approvato la sezione *«se tutto segue i principi … ed è
coerente»*. Riletta la tabella presentata in chat contro il disegno del 2026-09-04 e la consegna; il merito non cambia, e
la sola riga dedotta nuova — la 6 — è stata detta al proprietario.

| Nel testo presentato | Qui | Perché |
|---|---|---|
| riga 2 | la metà della risposta 3 sul modello **resta** | la risposta 3 ha due metà, e D10 conferma la seconda |
| riga 5 | anche il sì per la sessione di D7 | la riga *«chi aggiorna il router»* |
| riga 6 | *«fuori dallo spazio»* vuol dire verso una zona aperta, anche per il click — **dedotto** | D3 e ADR-0038 insieme |
| riga 9 | *«l'ambito è un numero dato dal kernel»* diceva più del deciso: l'identificativo è la forma **registrata**, K44; e l'«ambito» del 2026-09-04 erano due cose | RR5; K44; D3 |
| riga 11 | le frecce del 2026-09-04 dentro le tre specie di linea | la §4.2 le nomina |
| riga 12 | *«copia percorso»* non era nuovo: c'era nella §4.1 | la §4.1 |
| righe 6, 16 e 20 | la decisione 10, la decisione 9 e l'approccio scelto fra i posti da correggere | la tabella 3.4 e la testa del 2026-09-04 |

## 3. Gli ADR: i rimandi in testa, e l'ADR nuovo del backup — ✅ approvata il 2026-09-29

**A parole.** Un ADR non si riscrive. Quando una decisione nuova lo completa, riceve in testa un **rimando datato**, sotto
`Deciders`, nella forma che il repository usa dal 2026-09-03 — il precedente è ADR-0001 —; e la sua voce nella §5 del
compendio riceve una riga che vi rimanda. Qui c'è che cosa dice ciascun rimando, e l'ADR nuovo che D12 chiede. Li scrive il
**piano dei documenti**; qui si decide che cosa dicono.

**Riletti il 2026-09-29, per intero:** ADR-0009, 0010, 0011, 0012, 0014, 0015, 0016, 0022, 0024, 0025 e 0038. Ciascuno
**contro i fratelli**, gotcha #59: le coppie sono nella colonna a destra. L'elenco della consegna ne portava otto;
ADR-0010, 0012 e 0015 sono 🆕.

### 3.1 I rimandi in testa

| ADR | Che cosa dice il rimando | Da | Riletto contro |
|---|---|---|---|
| **0009** guide, sensori, anelli | il file-guida di una zona si carica per **fiducia alla cartella**, con l'impronta di ogni caricamento nel giornale; i trigger sono il sorvegliante più la scansione all'avvio, e sanno dire *«ho perso eventi, riscansiona»*. 🆕 ⚠️ **Dedotto:** il riconciliatore che corregge da solo i casi certi **non** è l'anello di miglioramento, e non tocca la sua regola *«non si auto-modifica in silenzio»*, che riguarda le guide e i sensori del sistema: i router sono dati del proprietario, osservati come `Untrusted`, e ogni correzione è giornalata, con la copia, e visibile | D9; K8; K9; K29; D6; D14 | 0014, 0038 |
| **0010** 🆕 budget della proiezione | in un ripiego la proiezione si compone **per il candidato** — la sua finestra e la sua guida — e si controlla prima di mandarla; un candidato la cui finestra non tiene ciò che ADR-0008 dice mai sacrificabile si **salta**; la compressione *middle-out* di OpenRouter è spenta | D10 | 0008, 0012 |
| **0011** routing risolto | la **sessione** della contabilità è quella di D11: la run radice che il proprietario apre, coi suoi discendenti | D11; RR1 | 0016 |
| **0012** 🆕 equivalenza del fallback | che cosa fa scattare la catena lo decide D10: sovraccarico, indisponibilità, errore del server; **non** i limiti di frequenza — resta da misurare su OpenRouter, K48 —, e il contesto eccessivo si controlla **prima**, per candidato, col rimando di ADR-0010. La catena la scrive il proprietario e la percorre il gateway, un modello per richiesta; un rifiuto per contenuto ripiega e resta. Il *Context* dell'ADR, che nomina limiti di frequenza e contesto eccessivo, è contesto e non decisione | D10; K48 | 0010, 0011 |
| **0014** confine dei dati non fidati | per il file-guida di una zona fidata, il **passaggio esplicito e giornalato** che l'ADR chiede è la fiducia alla cartella, data una volta dal proprietario e scritta nel giornale; ogni caricamento scrive l'impronta, la provenienza; un file-guida non concede permessi — ADR-0016 | D9 | 0015, 0016 |
| **0015** 🆕 descrizioni degli strumenti | il file-guida di una zona fidata **non** si riapprova quando cambia, come nei software letti per D9. ⚠️ È il *rug pull* che il richiamo AUD-004 di questo ADR descrive per le skill, **accettato** dal proprietario con D9 per la zona che ha dichiarato fidata — un `git pull` arriva all'agente senza che lui lo guardi —; restano il punto 5 di questo ADR, perché un file-guida non concede permessi, l'impronta di ogni caricamento nel giornale, e la modalità ristretta per la zona non fidata. Le descrizioni degli strumenti — anche dei server MCP di una zona fidata — restano sotto questo ADR; per le skill della knowledge base decide AUD-004, che ha qui il caso scritto | D9 | 0009, 0014 |
| **0016** permessi | la **sessione** del punto 3 è quella di D11, e finisce anche al riavvio del core — RR3; il permesso lo chiede chi agisce per un modello o invoca una funzione, e **la correzione deterministica di un fatto, che non cambia una scelta del proprietario**, segue un'impostazione — D14, col perimetro di CF5; i **percorsi protetti** — il file delle esclusioni e la cartella dati del programma —: nessun sì copre una scrittura dell'agente su di loro, e il controllo lo fa la porta prima di ogni sì; un effetto **irripetibile** chiede a **ogni invocazione**, non per la sessione — un'eccezione al punto 3, che la regola 4 di ADR-0038 dice già per ogni invocatore; fuori da ogni zona la **lettura** è una tripla `(file, percorso, lettura)` per la sessione, un file alla volta, e un'impostazione blocca ogni lettura fuori. Il sì oltre la sessione delle app di oggi resta registrato, K38 | D11; D14; D16; D18; D19; RR3; M1; M2 | 0007, 0014, 0038 |
| **0022** layout dei dati e backup | **modificato da ADR-0040** per i file del proprietario: la riga «artefatti», le guide della riga «configurazione, guide, profili», e la conseguenza *«la base di conoscenza sopravvive alla reinstallazione perché i documenti sorgente … sono nel backup»*; il rimando del 2026-09-08, che per le guide diceva *«la politica non cambia»*, si legge con ADR-0040. Il resto regge: la separazione per natura, il giornale cifrato, i segreti mai, gli indici fuori, i requisiti del motore. La forma sta nella 3.3 | D12; D17 | 0018, 0023, 0024 |
| **0024** checkpoint ad ambiti | l'ambito di una zona si **chiude** con la sessione; gli ambiti sono della porta e non della run, e fra due run li separa la sola tripla — K35; un ambito di **sola lettura**, per un file fuori da ogni zona, non è un ambito di lavoro e non tiene copie; 🆕 ⚠️ **dedotto:** il checkpoint copre le scritture del **programma**, non quelle del proprietario da fuori — D2; dove stanno le copie e quanto durano lo dice ADR-0040; il limite di dimensione resta da fissare, K21 | D2; D3; D11; D18; K21; K35; N2 | 0018, 0022 |
| **0025** confinamento a livelli | per i comandi che l'agente esegue, il livello 2 nega i percorsi del **privato**, la **scrittura** sui percorsi protetti e, con l'impostazione accesa, la **lettura** fuori da ogni zona: la porta non vede uno script che apre i file da sé | D5; D16; D18; D19; K36; N3 | 0016 |
| **0038** registro delle funzioni | la regola 2 — lo stesso permesso per ogni invocatore — non copre il riconciliatore, che non è un invocatore: la correzione deterministica di un fatto segue la sua impostazione, e un router toccato dalla GUI resta una funzione del registro; 🆕 nel rimando del 2026-09-05, *«spostare … fuori»* vuol dire verso una zona aperta: fuori da ogni zona non scrive nessuno, nemmeno il click — la riga 6 della sezione 2 | D3; D14; CF5 | 0016, 0024 |
| **0039** 🆕 telecamera | nella riga *«la destinazione di una cattura»* del perimetro negativo, dove sta il rimando del 2026-09-05: la cattura entra nella root come **ogni file nuovo** — nel livello strutturale, nel grafo e nella ricerca —, e in un router solo se il proprietario o l'agente la promuovono; la run la vede, come riferimento. È la riga 10 della sezione 2, con la risposta A del proprietario. La voce del compendio **non** cambia: dice *«nella knowledge base come artefatto, la run la vede come riferimento»*, e regge. ⚠️ **Richiamo del 2026-09-29, dalla rilettura del proprietario, voce 2:** la voce del compendio nomina **anche** questo richiamo, come fece col rimando del 2026-09-05 — la frase regge, ma senza il secondo rimando resterebbe indietro rispetto al suo ADR; la decisione 2 della 6.1 è ribaltata. ⚠️ **Aggiunta il 2026-09-29 dalla sezione 6**, la 6.1 | la risposta alla domanda della sezione 2 | le righe di 0038 e di 0018 che nominano la cattura e gli artefatti |

⚠️ **Nessun rimando serve in ADR-0007, 0018, 0019, 0023, 0034 e 0036**: le risposte li **usano**, non li cambiano. Un
permesso scritto senza sessione, nei giornali di oggi, si legge come di una sessione finita, e davanti al dubbio ci si
ferma — ADR-0007, CF9; la potatura delle copie è la logica di ADR-0018; un'indicizzazione in corso è una voce della lista
aperta del degrado, ADR-0019; il giornale resta cifrato, ADR-0023; le impostazioni di D14 e D18 e la ritenzione delle copie
sono parametri consegnati, ADR-0034; la sessione entra nel record del permesso su un indice nuovo, ADR-0036. Il codice che
ne segue lo dice la sezione 4.

### 3.2 L'ADR nuovo — ADR-0040, *«Dove vivono i dati, e che cosa salva il programma»*

Il titolo e il nome del file li fissa il piano. **Modifica ADR-0022** — la forma della 3.3 —, e prende anche la decisione
di D4 sul posto dei dati, che oggi il sorgente dichiara non presa da nessun ADR: *«where a per-user data directory belongs
is a decision no ADR has taken»*, in `crates/daemon/src/main.rs`.

| # | Decide | Da |
|---|---|---|
| 1 | **I dati del programma** stanno nella cartella dati per utente del sistema — `%LOCALAPPDATA%\<nomeapp>\` su Windows, le cartelle XDG su Linux —, in sottocartelle per natura, come vuole ADR-0022; la configurazione porta il percorso della root | D4; K1 |
| 2 | **I router** stanno in `.<nomeapp>/`, alla root, marcata nascosta dal modulo di piattaforma; **l'indice** nella cartella dati del programma, fra i dati che si rifanno — su Linux, la cache —, uno per root | D4; K30; K32 |
| 3 | **I file del proprietario** — la root e le zone di lavoro, coi file che le run vi producono e con le guide — stanno al loro posto, riferiti dal giornale, e **non** entrano nel backup del programma: li salvano i backup del proprietario e git. ⚠️ **L'eccezione è una:** `.<nomeapp>/`, coi router, che il programma salva — punto 4 | D12; D17 |
| 4 | **Il backup del programma** contiene il giornale, cifrato, la configurazione e i router; mai i segreti, né l'indice, né i pesi; e **quando lo crea dice che cosa resta fuori** — la root, le zone, le copie —, il seguito di ADR-0022 | D12; ADR-0022 |
| 5 | **Le copie del checkpoint** di ADR-0024 stanno nella cartella dati del programma, in una sottocartella loro; **in chiaro**, come i file che copiano; **fuori** dal backup; potate con la logica di ADR-0018, e mai quelle di un passo in dubbio non ancora riconciliato. Dopo un ripristino un passo di prima non si annulla più, e il programma lo dice: una copia assente non è una copia mai fatta — ADR-0018 | K53; lo stato dell'arte, 3.4 |
| 6 | la cartella dati è un **percorso protetto e privato**: l'agente non ci scrive, non la legge, e non si indicizza; la regola vive nel rimando di ADR-0016, e qui si nomina | D19 |

**Negative (accettate)**

| | |
|---|---|
| il backup dei file del proprietario | è suo: il programma non lo fa, e lo dice quando crea il proprio |
| dopo un ripristino | i passi di prima non si annullano; un passo in dubbio al momento del backup si riconcilia **senza** la sua copia, e allora si ferma e chiede — ADR-0007 · ⚠️ **dedotto** |
| una copia in chiaro | resta finché non è potata, anche se nel frattempo il proprietario cancella il file o lo rende privato; la protegge il sistema operativo, come in Claude Code |
| i router | finiscono in due backup, quello del programma e quello del proprietario, perché stanno nella root |
| una root spostata | l'indice si rifà con una scansione |

**Alternative considerate:** router e indice insieme in `.<nomeapp>/` — la B di D4; anche gli artefatti e le guide della
root nel backup del programma, con l'inseguitore di artefatti — la B di D12; gli artefatti delle zone nel backup — la B di
D17; le copie nel backup e cifrate, perché l'annulla sopravviva al ripristino — contro D17, coi file grandi due volte, K21.

**Il seguito:** il limite di dimensione delle copie resta di ADR-0024, K21; il tempo di ritenzione delle copie è un
parametro consegnato, ADR-0034, col valore a chi le costruisce; chi costruisce la cartella dati lo dice la sezione 4.

### 3.3 La forma: un ADR modificato in parte

**Il caso è nuovo:** nessun ADR del repository è mai stato superato solo in parte, e `CLAUDE.md` conosce due casi —
*«superato → `Superseded by`; completato → un rimando»*.

| | |
|---|---|
| la domanda | ADR-0040 cambia tre punti di ADR-0022, e il resto regge: **A**, modificato in parte — ADR-0022 resta `Accepted`, con un rimando in testa che nomina le righe, e ADR-0040 dichiara che lo modifica: la forma *«Amends / Amended by»* di `adr-tools`, 3.4; **B**, superato per intero — `Superseded by ADR-0040`, e ADR-0040 ricopia ciò che di ADR-0022 regge |
| il costo | **A**: una riga nuova in `CLAUDE.md`. **B**: testo stabile ricopiato — il gotcha #68 —, un ADR lungo, e un ADR `Accepted` in meno nei conteggi |
| i cinque criteri | **correttezza**: le tre fonti lette dal sorgente, e il controllo dei conteggi di `check-docs.sh` letto; **coerenza**: A è la forma dei rimandi che il repository usa già; **debito**: A nessuno, B un secondo testo della stessa decisione; **stato dell'arte**: A è `adr-tools`; MADR ha solo lo stato *superseded*; **proporzione**: A cambia una riga di regola, B riscrive un ADR |
| verificato, dedotto, assunto | **verificati**: le tre fonti; il conteggio, che conta le righe `- **Status:** Accepted`. **Dedotto**: che la regola di `CLAUDE.md`, scritta per il superamento intero, non copra il caso. **Assunto**: niente |
| ✅ **la risposta** | **A**, dal proprietario, il 2026-09-29, in chat |

**Che cosa ne segue, e lo scrive il piano:**

| Dove | Che cosa |
|---|---|
| la testa di ADR-0022 | *«⚠️ Rimando del ‹data› — modificato da ADR-0040»*, con le tre righe nominate e *«le altre reggono»*, al posto della frase *«Nessuna riga di questo ADR è superata»*; lo stato resta `Accepted` |
| la testa di ADR-0040 | *«Modifica ADR-0022»*, con le stesse tre righe |
| `CLAUDE.md`, la riga *«ADR append-only»* | un terzo caso: *superato in parte → un rimando in testa che nomina le righe, e l'ADR nuovo lo dichiara; lo stato resta `Accepted`* |
| il compendio | la voce nuova di ADR-0040 nella §5, che `check-docs.sh` pretende; le righe dei rimandi nelle voci della 3.1; i totali degli ADR; e nella tabella della §13 la riga del caso nuovo |
| i totali degli ADR, negli altri documenti di stato | la guardia dei conteggi di `check-docs.sh` li legge nei documenti della sua lista, e oggi il totale sta anche in `HANDOFF.md`, `roadmap.md` e `AVVIO-CHAT.md`: si riallineano, e in `AVVIO-CHAT.md` la cifra si **toglie** — la domanda 1 della 6.1. Il comando sta sotto la tabella. ⚠️ **Richiamo del 2026-09-29, dalla sezione 6:** la riga del compendio nominava tre documenti, e il comando stava in questa cella con la barra verticale — F10 |

Il comando della riga dei totali, fuori dalla tabella per F10: fa ciò che fa la guardia — toglie i code span, poi cerca i
totali — sui documenti della sua lista, che vive in `scripts/check-docs.sh`, nel passo *«ADR counts declared in the
prose»*: se cambia là, vale quella. Aggiunto il file di ADR-0040, `bash scripts/check-docs.sh` nomina da sé ogni totale da
riallineare.

```
for f in docs/HANDOFF.md docs/roadmap.md docs/README.md docs/COMPENDIO.md docs/AVVIO-CHAT.md CLAUDE.md; do sed 's/`[^`]*`//g' "$f" | grep -nE '[0-9]+ (ADR in stato|ADR|decisioni architetturali)' | sed "s|^|$f:|"; done
```

### 3.4 Le fonti lette per questa sezione

Portate in [`riferimenti.md`](../../riferimenti.md), nella sezione *«La revisione della knowledge base — le fonti del
disegno»*, con lo stesso commit.

| Fonte | Letta | Per |
|---|---|---|
| `adr-tools`, lo script `adr-new`, dal sorgente | 2026-09-29 | la 3.3: l'opzione per legare due ADR, con *«Amends»* nel nuovo e *«Amended by»* nel vecchio, accanto a quella che supera e cambia lo stato del vecchio |
| MADR, il modello di ADR, dal sorgente | 2026-09-29 | la 3.3: gli stati, e nessuno per una modifica parziale |
| Joel Parker Henderson, *Architecture decision record*, il README dal sorgente | 2026-09-29 | la 3.3: non si altera un ADR; lo si completa aggiungendo, o lo si supera con un ADR nuovo |
| Anthropic, *Checkpointing* e *Explore the .claude directory* di Claude Code, dal sorgente | 2026-09-29 | la 3.2, punto 5: le copie stanno nella cartella dell'applicazione, non nel progetto; in chiaro, protette dai permessi del sistema operativo; cancellate dopo trenta giorni per default; nessun backup |

### 3.5 La verifica chiesta dall'approvazione

Il proprietario ha approvato la sezione *«se tutto segue i principi … ed è coerente»*. Riletta la tabella presentata in chat
contro le risposte, gli ADR e le fonti; il merito non cambia.

| Nel testo presentato | Qui | Perché |
|---|---|---|
| ADR-0040: *«mai i tuoi file (root e zone)»* e *«router»* insieme | l'**eccezione** detta: `.<nomeapp>/` sta nella root, e il programma la salva | D12 salva i router; D17 parla degli artefatti |
| le copie *«cancellate col tempo»* | mai quelle di un passo in dubbio non riconciliato; e un passo in dubbio ripristinato senza copia si ferma e chiede | ADR-0018; ADR-0007 |
| ADR-0010: *«se non ci sta, quel modello si salta»* | se la sua finestra non tiene ciò che ADR-0008 dice **mai sacrificabile** | D10 |
| ADR-0012 | i limiti di frequenza, da misurare su OpenRouter | K48 |
| ADR-0009, il riconciliatore | **dedotto**, e detto così | i router come `Untrusted`, §2.2 del 2026-09-04 |
| ADR-0016, *«il riconciliatore segue la sua impostazione»* | il perimetro scritto per esteso: la correzione deterministica di un fatto, che non cambia una scelta del proprietario | CF5 |
| ADR-0015: *«è la differenza con questo ADR»* | detta per quello che è: il *rug pull* del richiamo AUD-004 di ADR-0015, accettato con D9 per le zone fidate, con le difese che restano; e le descrizioni dei server MCP di una zona fidata restano sotto ADR-0015 | ADR-0015 riletto per intero; D9 tocca i file-guida, non gli strumenti |

## 4. La porta dei file e il codice che crescerà: chi costruisce che cosa — ✅ approvata il 2026-09-29

**A parole.** La porta `filesystem` è il pezzo del kernel che legge e scrive su disco. Le risposte le chiedono di crescere,
e chiedono al kernel altri pezzi: la sessione, il gateway vero, i percorsi protetti. Qui c'è **chi costruisce ogni pezzo,
e quando**, con la regola di D15: paga chi usa il pezzo per primo. ⛔ **Nessuna riga di codice nasce con questo disegno:**
ogni pezzo lo costruisce il suo sotto-progetto, col suo disegno e col suo piano.

### 4.1 Che cosa c'è oggi nel codice — verificato il 2026-09-29, su `92243d9`

Il codice di prodotto non è cambiato dalla fotografia della consegna, `f830cb9`:
`git diff --stat f830cb9..HEAD -- crates/ gui/ scripts/ Cargo.lock Cargo.toml` non rende nulla.

| Che cosa | Il comando |
|---|---|
| la porta ha cinque metodi — `declare_scope`, `preserve`, `restore`, `read`, `write` — e tre errori, `OutsideScope`, `Unavailable`, `Missing` | `grep -n '^    fn ' crates/kernel/src/ports/filesystem.rs` e `grep -n -A10 'pub enum FilesystemError' crates/kernel/src/ports/filesystem.rs` |
| il kernel non interpreta i percorsi, e non sa dire se due `Path` sono lo stesso file | `grep -n -i 'interpret' crates/kernel/src/ports/filesystem.rs` |
| nessuna implementazione vera: `platform` implementa cinque altre porte e non `Filesystem`, c'è solo il falso di un test, e il simulatore non la nomina | i tre comandi sotto la tabella |
| un permesso non ha sessione, e resta concesso per sempre, anche dopo un riavvio | `grep -n 'SCOPED TO A SESSION' crates/kernel/src/permission.rs` e `grep -n 'triple therefore survives' crates/kernel/src/registry.rs` |
| la risorsa di un permesso è un `&'static str`, e `invoke` chiede soltanto `is_granted` | `grep -n 'pub resource' crates/kernel/src/permission.rs` e `grep -n 'permission::is_granted' crates/kernel/src/registry.rs` |
| `Parameters` ha quattro campi | `grep -n -A6 '^pub struct Parameters' crates/kernel/src/parameters.rs` |
| il candidato del gateway ha il nome `&'static str` e nessuna finestra, e la catena arriva per chiamata | `grep -n -A8 '^pub struct Candidate' crates/kernel/src/gateway/mod.rs` e `grep -n 'THE CHAIN IS DELIVERED PER CALL' crates/kernel/src/gateway/mod.rs` |
| il daemon apre `journal.redb` e `layout.redb` nella cartella da cui parte | `grep -n -e 'JOURNAL_PATH: ' -e 'LAYOUT_PATH: ' -e 'no ADR has taken' crates/daemon/src/main.rs` |

I tre comandi della terza riga — fuori dalla tabella, perché la barra verticale in una cella va scritta con la barra
rovesciata davanti, e copiata così rende zero: F10 della consegna. Rendono cinque righe, una riga, e niente:

```
grep -rEn '^impl (Custody|Journal|Reactor|Rng|Filesystem|Network|Process|Ipc) for ' crates/platform/src/
grep -rEn '^impl Filesystem for ' crates/
grep -rln Filesystem crates/simulator
```

### 4.2 Chi costruisce che cosa

| Chi | Che cosa | Perché lui | Da |
|---|---|---|---|
| **13** | la **lettura** della porta, con `declare_scope` per la root; la **sorgente degli eventi** del sorvegliante, che sa dire *«ho perso eventi, riscansiona»*; l'implementazione vera in `platform`, il doppio del simulatore e la suite di conformità, che nascono con lui e crescono con gli altri; fuori dalla porta, la **finestra** in `gateway::Candidate` e la proiezione per candidato; e il registro delle guide in una forma che ammetta anche l'approvazione **per fiducia alla cartella**, con l'impronta a ogni caricamento | è il primo che legge i file-guida, sorveglia la cartella e compone il contesto | D15; D9; D10; K9; F1; F2 |
| **3** | la **sessione**: la run radice nel giornale, il record di fine con la sua causa — anche il riavvio del core —, la sessione nel record del permesso su un indice nuovo e facoltativo, i due tempi come parametri consegnati; un permesso scritto senza sessione si legge come di una sessione finita. Il **gateway vero**: il nome del candidato scelto a runtime, il cammino sulla catena quando una chiamata fallisce, i tentativi e il cambio di modello nel record di routing. Il **selettore** del modello | porta le run | D11; D10; RR1–RR4; CF9; K47 |
| **6** | lo **scrivere**, anche condizionato — solo se il file è ancora la versione letta —; **elencare** e i **metadati**; **spostare**; **cancellare**, nel cestino di sistema; **conservare e ripristinare**, cioè le copie; le **esclusioni del privato**; i **percorsi protetti**, con la modifica di `invoke`; la tripla su una **cartella scelta a runtime**; l'impostazione del riconciliatore; e la **cartella dati** per utente e la **cartella nascosta** dei router, `.<nomeapp>/`, marcata nascosta dal modulo di piattaforma, se nessuno le porta prima. ⚠️ **Richiamo del 2026-09-29, dalla sezione 6:** la cartella nascosta mancava, e la 5.1 la dava già al 6 | scrive router e note, fa la scansione e il riconciliatore | D15; D14; D16; D19; K10; K30; K44; M4 |
| **5** | **aprire e chiudere** le zone di lavoro; la domanda di **fiducia** alla zona e il suo record; il **livello 2** che nega ai comandi il privato e la scrittura sui percorsi protetti | apre le zone ed esegue comandi | D3; D9; K35; K36 |
| il primo che **legge fuori**, a sessione esistente | l'ambito di sola lettura per un file fuori da ogni zona, con la tripla per la sessione, e l'impostazione che blocca ogni lettura fuori | — | D18; M6 |

### 4.3 Due precisazioni, dedotte dalla regola di D15

| | La precisazione | Perché |
|---|---|---|
| 1 | **la lettura fuori da ogni zona non la costruisce il 13**, come diceva la decisione 2 del coordinatore nel secondo controllo, M6: il sì di D18 vale **per la sessione**, e la sessione la porta il 3, che viene dopo. Prima di allora un import da fuori **non si carica**, e un avviso lo dice — la fallita chiusa di ADR-0012, per analogia | un sì costruito prima della sessione varrebbe per sempre, contro il punto 3 di ADR-0016 e CF9 |
| 2 | **la fiducia alle zone di D9 si divide**: il 13 dà al registro delle guide la forma che la ammette — una pretesa, come le due della §1.1e del 2026-09-04 —; la domanda al proprietario e il record della fiducia li costruisce chi apre per primo una zona coi file-guida, il 5. L'elenco della consegna la dava tutta al 13 | prima del 5 non esiste una zona da fidare: costruirla nel 13 sarebbe un pezzo senza chiamante, una previsione — gotcha #57 |

### 4.4 I due meccanismi, nella forma che regge la radice

| Meccanismo | La forma | Da |
|---|---|---|
| **i percorsi protetti** | il controllo **non** sta nel registro, perché il kernel non interpreta i percorsi e `invoke` non ne vede: il kernel dichiara una variante nuova dell'errore della porta — per esempio `FilesystemError::Protected` — e il modo di dichiarare i percorsi protetti; l'appartenenza la decide l'implementazione della porta, dopo aver risolto i collegamenti; e il rifiuto vale anche dentro un ambito concesso | D16; D19; M1 |
| **«chiede a ogni invocazione»** | con `Approval::Checked`, una funzione il cui effetto è `EffectClass::Unrepeatable` non si accontenta di `is_granted`: chiede ogni volta, a qualunque invocatore — la regola 4 di ADR-0038 | D16; M2; RR8 |

### 4.5 La spec del sotto-progetto 1

Ogni pezzo nuovo della porta, e il permesso con la sessione, cambiano ciò che la spec del sotto-progetto 1, che è del
proprietario, dice della porta `filesystem` e del permesso — la **§6.6**. ⚠️ La spec **non ha una sezione** della porta dei
file: la nomina solo la tabella delle famiglie della **§2.3**, nella riga `filesystem`, che rimanda alla §4 — e la §4 è
*«Giornale, riconciliazione e motore di persistenza»*. ✅ **Deciso dal proprietario il 2026-09-29, con la sezione:** il
**piano dei documenti** mette **adesso** un richiamo datato che annuncia i pezzi nuovi e chi li costruisce, e rimanda qui —
nella riga `filesystem` della §2.3, dove dice anche che la §4 non descrive la porta, e in testa alla §6.6 —; il testo
esatto di ciascun pezzo lo scrive chi lo costruisce, col suo richiamo. ⚠️ **Richiamo del 2026-09-29, dalla sezione 6:**
diceva *«la §4 — la porta `filesystem`»* e *«in testa alle due sezioni»*; il posto nella §2.3 è la risposta A del
proprietario alla domanda 2 della 6.1.
⛔ La **§8** non si tocca: V21 resta ⚠️ parziale col suo innesco — la sezione di D11 nella consegna archiviata —; V36, Q22
e la riga `filesystem` della §8.2.2 pure, registrate nella 6.5.

### 4.6 Registrate, col chiusore

| # | Il caso | Chi |
|---|---|---|
| K10 | la scrittura condizionata, senza corsa col proprietario | il 6 |
| K35 | gli ambiti sono della porta e non della run | il 5, con la chiusura delle zone |
| K44 · K49 · K51 | la risorsa di un permesso su un percorso scelto a runtime; la chiave stabile della fiducia; l'identificativo coniato alla richiesta, col percorso come `Untrusted` | il 6; il 5 per la fiducia |
| K47 | il gateway: il nome a runtime, il cammino, i tentativi, il cambio di modello | il 3; la finestra il 13 |
| K52 | una cartella privata spostata: nel dubbio resta privata, e il riconciliatore chiede | il 6 |
| K36 · K12 | il privato per i comandi; i server MCP | il 5; il 4 |

**Rispetto al testo presentato in chat** — approvato con un sì, senza condizione —: aggiunti dall'elenco della consegna,
senza cambiare il merito, il doppio del simulatore e la suite di conformità col 13, F2; i metadati, conservare e
ripristinare, e la modifica di `invoke` col 6; l'aprire le zone col 5; e i comandi della 4.1.

## 5. Roadmap, tracciabilità, stella polare della GUI e design/09, col perimetro del 13 riletto — ✅ approvata il 2026-09-29

**A parole.** Quattro documenti dicono ancora ciò che le risposte hanno superato. Qui si decide **che cosa** vi si
corregge; le righe le scrive il **piano dei documenti**. Nella stella polare della GUI e in design/09, che sono disegni
approvati, ogni correzione è un **richiamo datato** nella riga; nella roadmap e nella tracciabilità, che sono tabelle di
stato, la cella si riscrive col suo richiamo datato, come fanno già.

**Riletto contro i documenti di adesso, il 2026-09-29.** L'elenco della consegna portava le righe della roadmap, undici
righe della tracciabilità, due punti della stella polare e due di design/09. Le righe segnate 🆕 mancavano. Ogni riga
si ritrova col comando, non col numero di riga: `grep -n '^| <numero> |' docs/roadmap.md`, e il nome della riga per
gli altri tre documenti.

### 5.1 `roadmap.md`

**La forma, decisa con la domanda della 5.5:** ogni riga porta una **frase corta** e un **rimando alla 4.2** di questo
disegno, che resta la **casa unica** di chi costruisce che cosa. La lista non si ricopia.

| Riga | Che cosa dice | Da |
|---|---|---|
| **13** | cresce: la lettura della porta, con l'ambito della root; la sorgente degli eventi, col «riscansiona»; l'implementazione vera in `platform`, il doppio del simulatore e la suite di conformità; la finestra in `gateway::Candidate` e la proiezione per candidato; il registro delle guide nella forma che ammette la fiducia alla cartella. La dipendenza non cambia: 1, e AUD-004 | 4.2; D15; D9; D10 |
| **3** | la sessione, il gateway vero, il selettore del modello | 4.2; D10; D11 |
| **5** | aprire e chiudere le zone; la domanda di fiducia e il suo record; il livello 2 per i comandi | 4.2; D3; D9 |
| **6** | cade *«archivio unico»*: la root è una cartella qualsiasi, dalla configurazione. La prima metà porta il livello strutturale con la ricerca testuale, la scansione e il riconciliatore con la sua impostazione; lo scrivere e il resto della porta della 4.2; i percorsi protetti; la cartella dati per utente e la cartella nascosta dei router, se nessuno le porta prima. La seconda metà resta la somiglianza | 4.2; D4; D14; D15; D16; D19 |
| **10** 🆕 | **nessun cambio.** L'elenco gli dava la cartella dati e la cartella nascosta, K1 e K30; per la regola di D15 le porta chi le usa per primo, il 6 — ⚠️ **dedotto** | D15 |
| **11** | il rimando ad ADR-0040 accanto ad ADR-0022; la dipendenza diventa **6, 9**: il 5 esce — ✅ la seconda domanda della 5.5 | D12; D15; ADR-0040 |
| *«Backup dopo indici e pesi»* | la non-vacuità di 6 e 9 resta; cade *«serve inoltre il filesystem reale, che arriva con 5»*: la porta arriva a pezzi; l'interfaccia che dichiara le esclusioni al momento del backup è il punto 4 di ADR-0040 | D15; ADR-0040 |
| *«Il primo valore utile»* 🆕 | *«1 + 2 + 3»* diventa *«1 + 2 + 13 + 3»*: il 13 sta prima del 3 dal 2026-09-04, e la riga non lo diceva già prima di questa revisione | la decisione 16 del 2026-09-04 |

### 5.2 `tracciabilita.md`

Undici righe: `grep -c -E '^\| (Multi-repo|Mappa del progetto|Git e gestione|Collezioni|File watching|Sessioni multiple|Selettore di modello|Backup della KB|Sandboxing|Permessi e sandbox|Modalità di permessi)' docs/tracciabilita.md`.

| Riga | Che cosa dice | Da |
|---|---|---|
| `Collezioni e knowledge base` | cade *«archivio unico»*: la root, i due livelli, il riconciliatore | il documento; D4 |
| `Multi-repo/multi-progetto` · `Mappa del progetto` | le zone di lavoro, col 5, e la scheda progetto nella knowledge base, col 6 | D3 |
| `Git e gestione branch` | la repo è una zona di lavoro | D3 |
| `File watching e awareness del progetto` | il sorvegliante e la scansione all'avvio, col «riscansiona», al 13; la politica al 6 | K8; K9; D15 |
| `Sessioni multiple` | la sessione di D11, al 3 | D11 |
| `Selettore di modello per compito` | il selettore della sessione, come Claude Desktop, al 3; il modello scritto nella definizione di un sotto-agente | D10 |
| `Backup della KB indipendente dall'app` | cade *«documenti nel backup»*: la root è nei backup del proprietario, i router in quello del programma — ADR-0040 | D12; D17 |
| `Sandboxing ed esecuzione` · `Permessi e sandbox policy` · `Modalità di permessi a più livelli` | i percorsi protetti, col 6; la lettura fuori da ogni zona col permesso, e l'impostazione che la blocca, al primo che legge fuori; il livello 2, col 5 | D16; D18; D19 |

### 5.3 La stella polare della GUI

Sette punti; l'elenco ne portava due.

| Dove | Che cosa dice il richiamo | Da |
|---|---|---|
| la decisione 1 | la rete ha tutto ciò che sta **nella root**; un file che l'agente scrive in una zona fuori sta nell'anello, e nella rete attraverso la scheda della zona | D13 |
| la decisione 11, la riga 13 della barra, e la voce registrata del selettore | **deciso**: il modello si sceglie a mano accanto al pulsante di invio, per la sessione o come default, come Claude Desktop, e ogni risposta porta il nome del modello | D10 |
| 🆕 la decisione 13 e il modulo **Ambito** | l'ambito della run diventa la **zona di lavoro**, aperta per la sessione, e la root c'è sempre; il resto — la copia prima delle modifiche, la domanda che dice se il percorso è dentro o fuori — regge | D3; D11 |
| 🆕 la voce registrata *«le letture ovunque o solo dentro l'ambito?»* | **chiusa da D18**: dentro la root e le zone la lettura procede; fuori chiede, un file alla volta, per la sessione | D18 |
| 🆕 la voce registrata *«un ambito per progetto o uno per run?»* | la zona è della sessione, e gli ambiti sono della porta e non della run; il resto della domanda — la cartella proposta a una nuova run — resta al 3 | D3; D11; K35 |
| 🆕 il modulo **Knowledge base** | la ricerca nel **testo**, coi filtri per tipo e per cartella; la griglia accanto al grafo; l'anteprima al click; tre specie di linea; il segnale rotto anche per il file chiave perso | il documento; D6; D8 |
| 🆕 il modulo **Checkpoint** | le copie stanno nella cartella dati del programma, fuori dal backup, e dopo un ripristino un passo di prima non si annulla | ADR-0040 |

### 5.4 design/09

Non basta una riga: il diagramma e la tabella si riallineano ad ADR-0040. L'elenco ne portava due punti.

| Dove | Che cosa dice il richiamo | Da |
|---|---|---|
| la riga degli artefatti, e 🆕 il nodo del diagramma | i file del proprietario — la root e le zone, le guide comprese — **non** sono nel backup del programma; la porta dei file arriva a pezzi, 13, 6 e 5, e non più *«l'implementazione vera col 5»* | D12; D15; D17 |
| 🆕 una riga nuova, i **router** | in `.<nomeapp>/` alla root, in chiaro, **nel** backup del programma, non ricostruibili; li scrive il 6 | D4; D12; K32 |
| 🆕 una riga nuova, le **copie del checkpoint** | nella cartella dati, in chiaro, fuori dal backup, potate come vuole ADR-0018; col 6 | K53; ADR-0040 |
| 🆕 la riga degli indici | l'indice del livello strutturale sta nella cartella dati del programma, uno per root | D4 |
| 🆕 il richiamo del 2026-09-08 in testa | *«le guide sono … artefatti dell'utente»*: file del proprietario, fuori dal backup del programma; e **dove** sta tutto: la cartella dati per utente, ADR-0040 punto 1 | D4; D12 |
| 🆕 il nodo del giornale | *«guide approvate (col 13)»*: anche la fiducia alle zone, col 5, e l'impronta di ogni caricamento, col 13 | D9; 4.3 |

### 5.5 Le due domande, e le risposte

| | La domanda | La risposta |
|---|---|---|
| 1 | come la roadmap porta chi costruisce che cosa: **A**, una frase corta e un rimando alla 4.2; **B**, la lista copiata in ogni riga — due copie che divergono, gotcha #68 | ✅ **A**, dal proprietario, il 2026-09-29 |
| 2 | trovata dalla verifica dopo la prima risposta: la dipendenza dell'11 dal 5 aveva un solo motivo, il filesystem reale, che D15 toglie; V32, V33 e Q21 non chiedono il 5 — il comando sotto la tabella. **A**, l'11 dipende da 6 e 9; **B**, il 5 resta senza motivo scritto | ✅ **A**, dal proprietario, il 2026-09-29 |

Il comando della seconda — fuori dalla tabella, per la trappola F10 della 4.1 —; rende tre righe:

```
grep -n -E '^\| *(V32|V33|Q21) *\|' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
```

Entrambe sotto la stessa accettazione condizionata delle sezioni prima. **Verificati:** le righe dei quattro documenti,
e le tre righe della spec. **Dedotti:** la riga 10; la riga del primo valore; che nient'altro nell'11 chieda il 5.
**Assunto:** niente.

### 5.6 Il perimetro del 13, riletto

Lo dicono la **4.2** e la **1.8**, e qui non si ripete. Che cosa **non** porta: la lettura fuori da ogni zona e la domanda
di fiducia — la 4.3. Che cosa non cambia: la strada B; l'ordine 2, 13, 3; **AUD-004 lo sbarra ancora**, e ha ora il caso
di D9 scritto nel rimando di ADR-0015 — la 3.1.

### 5.7 La verifica chiesta dall'approvazione

Il proprietario ha approvato *«se tutto segue i principi … ed è coerente»*. Riletto il testo presentato in chat contro i
quattro documenti e la spec; il merito non cambia, salvo la seconda domanda, che è andata al proprietario.

| Nel testo presentato | Qui | Perché |
|---|---|---|
| la riga 11, *«la frase non regge più»* | anche la **dipendenza** dal 5 perde il motivo: la seconda domanda | le righe V32, V33 e Q21 della spec |
| *«anche la cartella nascosta dei router»* al 6 | *«se nessuno le porta prima»*, come la cartella dati | la 4.2 |
| la stella polare, *«le zone valgono per la sessione»* | la root c'è sempre, e la domanda dentro o fuori regge | la decisione 13 riletta per intero |
| design/09 | anche il nodo del giornale | le guide approvate, e la fiducia di D9 |

### 5.8 Il disegno dei gesti — 🆕 aggiunta il 2026-09-29 dalla sezione 6

Il [disegno dei gesti](2026-09-03-riconoscimento-gesti-design.md) porta in due righe la frase che la riga 10 della sezione 2
supera — la foto che atterra in un gruppo, col router che segue. È un disegno approvato: la correzione è un **richiamo
datato** nella riga, e lo scrive il piano dei documenti. Trovata dalla rilettura della 6.1.

| Dove | Che cosa dice il richiamo | Da |
|---|---|---|
| la decisione 7, *«dove finisce la cattura con un gesto»* | la cattura entra nella root come ogni file nuovo — nel livello strutturale, nel grafo e nella ricerca —, e in un router solo se il proprietario o l'agente la promuovono; la run riceve il riferimento | la riga 10 della sezione 2 |
| il buco logico *«La foto va in che contesto?»* | la stessa correzione; e *«la cartella della knowledge base è l'archivio»* si legge: la root, una cartella qualsiasi, dalla configurazione | le righe 1 e 10 della sezione 2 |

La tabella delle decisioni di quel disegno non cambia forma: due righe si riscrivono, nessuna si aggiunge.

## 6. I controlli per artefatto, verificato-dedotto-assunto, le voci aperte e il prossimo passo — ✅ approvata il 2026-09-29

**A parole.** Questa sezione chiude il disegno. Prima dice che cosa ha trovato la **rilettura** delle sezioni 1–5 contro i
documenti di adesso, e le due domande che ne sono nate. Poi, per ogni cosa che il piano dei documenti scriverà, **quale
controllo la esercita** — la Definizione di «fatto» del piano, che il piano copia da qui —, con le trappole misurate oggi.
Poi **dove sta**, per ogni sezione, ciò che è verificato, dedotto e assunto; le **voci che restano aperte**, ciascuna col
suo chiusore; e il **prossimo passo**. Il proprietario l'ha approvata il 2026-09-29 sotto la stessa accettazione
condizionata delle sezioni prima — *«se tutto segue i principi di decision-principles ed è coerente»* —, e la verifica sta
nella 6.7.

### 6.1 La rilettura delle sezioni 1–5, e che cosa ha corretto

Rilette il 2026-09-29, su `2af0990`, contro i documenti che il piano toccherà e contro `scripts/check-docs.sh`. **Il merito
delle risposte non cambia:** ogni correzione porta il suo richiamo datato nella sezione che tocca.

| # | Trovato | Corretto in |
|---|---|---|
| 1 | [ADR-0039](../../adr/0039-telecamera-come-sorgente-di-percezione.md) porta ancora, nella riga della destinazione di una cattura, *«un file in un gruppo con il router che segue»*: la frase che la riga 10 della sezione 2 supera. La sezione 3 non lo nominava, né fra i rimandi né fra gli ADR che non ne hanno bisogno | la 3.1, una riga nuova |
| 2 | il [disegno dei gesti](2026-09-03-riconoscimento-gesti-design.md) porta la stessa frase, in due righe; la sezione 5 non lo nominava | la 5.8, nuova |
| 3 | la tabella 3.4 del [disegno del 2026-09-04](2026-09-04-knowledge-base-design.md), casa unica dello stato delle sue decisioni, riassume nella riga «1–7» le risposte con *«archivio a mappa»* e *«solo il nostro assistente»*; la sezione 2 correggeva le risposte, non quella riga | la sezione 2, la riga 23 |
| 4 | la 4.2, casa unica di chi costruisce che cosa, non portava la **cartella nascosta** dei router, che la 5.1 e la 5.7 danno al 6 | la 4.2, la riga del 6 |
| 5 | la 1.1 diceva che il posto delle copie del checkpoint *«non è deciso»*, e la 3.2, punto 5, l'ha deciso dopo | la 1.1 |
| 6 | la 3.3 nominava tre documenti per i totali degli ADR, e la guardia dei conteggi ne legge di più: il totale sta anche in [`HANDOFF.md`](../../HANDOFF.md) e in [`AVVIO-CHAT.md`](../../AVVIO-CHAT.md), che la decisione 32 della [stella polare della GUI](2026-09-07-direzione-gui-design.md) vuole fermo. E il comando stava in una cella, con la barra verticale — F10 | la 3.3; la domanda 1 |
| 7 | la 4.5 diceva *«la §4 — la porta `filesystem`»*: la §4 della [spec del sotto-progetto 1](2026-08-06-sottoprogetto-1-kernel.md) è *«Giornale, riconciliazione e motore di persistenza»*, e nessuna sezione descrive il contratto della porta; la spec la nomina solo nella riga `filesystem` della tabella delle famiglie della §2.3, che rimanda alla §4 | la 4.5; la domanda 2 |
| 8 | la 4.6 non portava dieci voci registrate della consegna; e una voce aperta della [porta di qualità](../../porta-di-qualita.md), **E94**, perde la premessa col 3 e col 6. ⚠️ **Richiamo del 2026-09-29, dalla rilettura del proprietario, voce 3:** E94 del piano del Traguardo 6 è **chiusa dal 2026-09-01**, per decisione del proprietario — commit `3c90119` —, e la riga che la dava aperta è stantia: è la riga 34 della tabella del Traguardo 5, la contraddizione C-S5-3. Resta vero, e più netto, il seguito: la riga della 6.5, riscritta | la 6.5 |
| 9 | V36 e Q22 della §8 della spec, e la riga `filesystem` della sua §8.2.2, hanno l'innesco **D (5)**; ma conservare e ripristinare sui file veri li costruisce il 6, e la suite di conformità della porta nasce col 13 — la 4.2 | la 6.5, registrata |

I comandi delle righe che ne hanno uno, nel loro ordine — fuori dalla tabella, per F10; le righe 4 e 5 si leggono in questo
file. La riga 1 ne ha due: il secondo dice che la voce di ADR-0039 nel compendio regge.

```
grep -n 'router che segue' docs/adr/0039-telecamera-come-sorgente-di-percezione.md
grep -n 'la run la vede come riferimento' docs/COMPENDIO.md
grep -n 'in un gruppo' docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md
grep -n 'solo il nostro assistente · a piani' docs/superpowers/specs/2026-09-04-knowledge-base-design.md
grep -n 'for f in docs/HANDOFF.md' scripts/check-docs.sh
grep -n '^## 4\. ' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
grep -n 'ambiti di checkpoint, artefatti' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
grep -c declare_scope docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
grep -n 'TERZA BOCCA' docs/porta-di-qualita.md
grep -n -E '^\| (V36|Q22) \|' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
grep -n 'D — si scrive su file reali' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
```

**Le due domande della sezione, e le risposte**

| | La domanda | La risposta |
|---|---|---|
| 1 | `AVVIO-CHAT.md` scrive il totale degli ADR, la guardia dei conteggi lo legge, e la decisione 32 della stella polare della GUI lo vuole fermo dal 2026-09-09: con ADR-0040 il cancello è rosso. **A**, la cifra si **toglie**, una volta: è la regola di `CLAUDE.md` per la cifra che vive in più documenti, e la forma che il file usa già — il richiamo del 2026-09-01 in testa, gotcha #31. **B**, si porta al totale nuovo, come si fece il 2026-09-03: ogni ADR nuovo tornerebbe a toccare un file fermo | ✅ **A**, dal proprietario, il 2026-09-29, in chat |
| 2 | dove va il richiamo della porta dei file nella spec del sotto-progetto 1: **A**, nella riga `filesystem` della §2.3, l'unico posto in cui la spec la nomina, dicendo che la §4 non la descrive; **B**, in testa alla §4, com'era scritto — ma la §4 è il giornale, e chi legge la §2.3 non troverebbe niente | ✅ **A**, dal proprietario, il 2026-09-29, in chat |
| i cinque criteri | **correttezza**: la lista della guardia, la riga di `AVVIO-CHAT.md` e la decisione 32 lette; i titoli della §2.3 e della §4 letti. **Coerenza**: 1A è la regola di `CLAUDE.md` e la forma del file; 2A mette la nota dove la porta è nominata. **Debito**: 1B un tocco a ogni ADR in un file fermo; 2B una nota nel posto sbagliato, e la riga della §2.3 che resta sola. **Stato dell'arte**: non decide, sono regole del repository. **Proporzione**: una riga ciascuna, in tutte e quattro | |
| verificato, dedotto, assunto | **verificati**: la guardia nel sorgente di `scripts/check-docs.sh`, la riga del file, la decisione 32, i titoli della spec. **Dedotto**: che negli altri documenti i totali si riallineino invece di togliersi — la decisione 1 qui sotto. **Assunto**: niente | |

**Le decisioni del coordinatore in questa sezione** — il proprietario può ribaltarle:

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | negli altri documenti di stato i totali degli ADR si **riallineano**, come il 2026-09-03 | la regola di `CLAUDE.md` toglie una cifra perché, riallineata, resterebbe difesa dalla **sola** regola; questi totali li difende la guardia, che fa rosso — ⚠️ dedotto. Costo: un riallineamento a ogni ADR nuovo, che la guardia non lascia dimenticare |
| 2 | la voce di ADR-0039 nella §5 del compendio **non** cambia. ⚠️ **Ribaltata dal proprietario il 2026-09-29, alla rilettura, voce 2:** la voce nomina **anche** il richiamo nuovo, come fece col rimando del 2026-09-05 e come ogni altro ADR della 3.1 | la sua frase regge, e il compendio ha un tetto. Costo: una riga, se il proprietario la vuole |
| 3 | V36, Q22 e la riga della §8.2.2 **registrate**, senza domanda | è la forma del precedente: l'innesco B (3) di Q6 e Q11, registrato nella stella polare della GUI col proprietario come chiusore. Costo zero oggi |
| 4 | E94 registrata come **legame**, senza domanda. ⚠️ **Richiamo del 2026-09-29, dalla rilettura del proprietario, voce 3:** la premessa era falsa — E94 del piano del Traguardo 6 è chiusa dal 2026-09-01, e la decisione di AUD-050 per quel tipo è **presa** —; la riga della 6.5 porta ora il suo nome vero, e la chiude il disegno del primo che ne ha bisogno | la decisione è di AUD-050, già del proprietario nella porta di qualità; qui si scrive solo **quando** smette di essere un'ipotesi. Costo zero oggi |

### 6.2 Che cosa scrive il piano dei documenti, e il controllo che esercita ciascun artefatto

È la **Definizione di «fatto»** del piano dei documenti: il piano la copia da qui. La casa di **che cosa** dice ciascun
artefatto resta la sezione nella terza colonna; qui sta **come si prova** che c'è. Ogni controllo si prova nelle due
direzioni: prima del piano nessun file da toccare porta un link a questo disegno, tranne il compendio col puntatore della
§6 — il comando E —; dopo, ciascuno ne porta.

| # | Artefatto | Che cosa dice | Il controllo |
|---|---|---|---|
| 1 | il disegno del 2026-09-04: il richiamo in testa, e un richiamo per ogni riga della sezione 2 | la sezione 2 | la revisione, riga per riga contro la sezione 2; il link a questo disegno, comando E |
| 2 | i rimandi in testa agli ADR della 3.1, e il richiamo nella riga di ADR-0039 | la 3.1 | ciascun ADR riletto **contro i fratelli** della sua riga — gotcha #59; il comando A prima e dopo: un rimando in più per ADR; la voce di ciascuno nella §5 del compendio con la riga che rimanda, **ADR-0039 compreso**. ⚠️ **Richiamo del 2026-09-29, dalla rilettura del proprietario, voce 2:** diceva *«salvo ADR-0039 — la decisione 2 della 6.1»* |
| 3 | ADR-0040, e la sua riga nell'indice di `README.md` | la 3.2 e la 3.3 | `check-docs.sh`, rosso in tre modi finché manca qualcosa: la voce della §5 del compendio, la riga dell'indice, un totale vecchio. E, cercati: lo stato `Accepted`; *«Modifica ADR-0022»* in testa ad ADR-0040 e *«modificato da ADR-0040»* in testa ad ADR-0022; le *«Negative (accettate)»* |
| 4 | i totali degli ADR nei documenti di stato | la 3.3 | la guardia dei conteggi; e per `AVVIO-CHAT.md` il comando B, che oggi rende una riga e dopo niente: la cifra è tolta, domanda 1 |
| 5 | la riga *«ADR append-only»* di `CLAUDE.md` | la 3.3 | la frase del terzo caso, cercata; nessuna cifra seguita da «ADR», perché `CLAUDE.md` è nella lista della guardia |
| 6 | il compendio: la voce di ADR-0040; le righe dei rimandi; la riga della §13; 🆕 questo disegno nella §12, nella riga della knowledge base o in una sua; 🆕 la data in testa | la 3.3; la §12 e la data qui | `check-docs.sh`, col tetto e l'accoppiamento della §5; il **margine misurato prima** di scrivere, comando C; il comando E sul compendio rende almeno due righe, il puntatore e la §12. La data: la §13 del compendio dice perché è la riga più facile da lasciare indietro |
| 7 | 🆕 `README.md`: questo disegno fra le specifiche, sulla forma della riga del disegno del 2026-09-04 | qui | il comando E su `README.md` |
| 8 | la spec del sotto-progetto 1: il richiamo nella riga `filesystem` della §2.3, e quello in testa alla §6.6 | la 4.5 | il comando E; ⛔ la **§8 non cambia**: il blocco A della §6 del compendio rende lo stesso prima e dopo |
| 9 | `roadmap.md` | la 5.1 | ogni riga della 5.1 col rimando alla 4.2, contata col comando E; nessuna riga rinumerata; la riga 10 intatta |
| 10 | `tracciabilita.md` | la 5.2 | il comando della 5.2 rende ancora undici righe, e ciascuna porta il link |
| 11 | la stella polare della GUI | la 5.3 | i sette punti, ciascuno col richiamo datato e il link |
| 12 | design/09 | la 5.4 | i sei punti; il diagramma e la tabella dicono la stessa cosa |
| 13 | il disegno dei gesti | la 5.8 | le due righe col richiamo; nessuna riga nuova nella sua tabella delle decisioni |
| 14 | 🆕 `HANDOFF.md` | qui | solo i totali degli ADR, alla guardia |
| 15 | ⛔ **nessun codice** | la sezione 4 | il comando D, vuoto |
| 16 | i fine-riga di ogni file toccato | — | la trappola 3 della 6.3 |

I comandi, fuori dalla tabella per F10. **A** — i rimandi, uno in più per ADR a piano eseguito:

```
for n in 0009 0010 0011 0012 0014 0015 0016 0022 0024 0025 0038 0039; do printf '%s ' $n; grep -c 'Rimando del' docs/adr/$n-*.md; done
```

⚠️ **Richiamo del 2026-09-29, dalla scrittura del piano:** il comando conta le **righe**, e in ADR-0039 il rimando nuovo entra nella riga che porta già quello del 2026-09-05 — la cella del perimetro negativo —: renderebbe `1` prima e dopo. Il piano lo usa con `grep -o 'Rimando del' … | wc -l`, che conta le **occorrenze**: P-15 del [piano](../plans/2026-09-29-knowledge-base-revisione-documenti.md).

**B** — il totale in `AVVIO-CHAT.md`, come lo legge la guardia; a piano eseguito non rende niente:

```
sed 's/`[^`]*`//g' docs/AVVIO-CHAT.md | grep -nE '[0-9]+ (ADR in stato|ADR|decisioni architetturali)'
```

**C** — il margine del compendio sotto il suo tetto, nella forma **CRLF**, che è quella che vincola: lo stesso numero su ogni
clone — la trappola 2 della 6.3; **D** — il codice, da `<base>`, il commit su cui si apre il piano; **E** — il link a
questo disegno, su ciascun file. ⚠️ **Richiamo del 2026-09-29, dalla rilettura del proprietario:** il comando C era il
tetto e `wc -c` del compendio, che su un clone LF rende un byte in meno per riga.

```
echo $(( $(sed -n 's/^ceiling=//p' scripts/check-docs.sh) - $(tr -d '\r' < docs/COMPENDIO.md | wc -c) - $(tr -cd '\n' < docs/COMPENDIO.md | wc -c) ))
git diff --stat <base>..HEAD -- crates/ gui/ scripts/ Cargo.lock Cargo.toml
grep -c '2026-09-28-knowledge-base-revisione-design' <file>
```

### 6.3 Le trappole, misurate il 2026-09-29

| # | Trappola | Che cosa fare |
|---|---|---|
| 1 | il totale degli ADR non vive solo nel compendio e nella roadmap: la guardia legge più documenti, e oggi lo trova anche in `HANDOFF.md` e in `AVVIO-CHAT.md` | aggiunto il file di ADR-0040, `bash scripts/check-docs.sh` nomina ogni documento e ogni totale; in `AVVIO-CHAT.md` la cifra si toglie, comando B |
| 2 | il compendio ha un tetto, e il verde non è un margine: misurato il 2026-09-29, il margine è di **8 607 byte** — comando C. ⚠️ **Richiamo del 2026-09-29, dalla rilettura del proprietario:** è il margine in forma **CRLF**, ed è quello che vincola — `check-docs.sh` misura l'albero di lavoro con `wc -c`, e su un clone CRLF il compendio porta un byte in più per riga; su un clone LF, `wc -c` darebbe al piano un margine che non ha. Il comando C ora rende la forma CRLF su ogni clone | il piano aggiunge una voce, una riga di rimando per ADR, una riga in §12 e una in §13: ci sta, ⚠️ dedotto; si rimisura **prima**, e ciò che è verbale va in archivio |
| 3 | i fine-riga sono misti fra i file di questo piano, e cambiano da una macchina all'altra | `git ls-files --eol` su ogni file prima di toccarlo — conta la colonna `i/` —; Python con `newline=""`, su un temporaneo e `os.replace`, gotcha #82; il conto dei CR rifatto dopo |
| 4 | F10: un comando con la barra verticale dentro una cella di tabella | fuori dalla tabella, come nella 4.1, nella 5.5 e qui |
| 5 | il `grep` di Git Bash: con `-i` e più di un `-e` non rende niente — ci è cascato anche questo disegno, la 6.7. ⚠️ **Richiamo del 2026-09-29, dalla rilettura del proprietario:** non è un «niente» — il `grep` 3.0 di Git Bash **va in crash**, e bash stampa `Aborted`; con lo stderr scartato, o in fondo a una pipeline, sembra un niente | un `grep` per parola, e la controprova su un input che deve rendere uno |
| 6 | la §4 della spec del sotto-progetto 1 non è la porta dei file | il richiamo va nella riga `filesystem` della §2.3 — domanda 2 |
| 7 | ADR-0024 si chiama `…-ad-ambiti-dichiarati.md`: un link dedotto dal titolo è rotto | `ls docs/adr` prima del link |
| 8 | `CLAUDE.md` e `HANDOFF.md` sono nella lista della guardia | nelle righe nuove, i numeri piccoli a parole |
| 9 | il pre-controllo ha trovato un difetto in ogni compito dispacciato | ogni compito si rilegge contro i documenti di allora, non contro questo disegno |

### 6.4 Verificato, dedotto, assunto: dove sta ciascuno

Ogni sezione porta le sue specie nelle proprie righe, e qui non si ricopiano — gotcha #68: questa è la **mappa**, più ciò
che la sezione 6 aggiunge.

| Sezione | Verificato | Dedotto | Assunto |
|---|---|---|---|
| 1 | le risposte e la consegna, rilette — la 1.9 | *«nessuna sesta proprietà»*, la 1.8; che il costo in token dipenda dalla strada fatta, la 1.3 — lo misura il 6, K22 | che la regola di Claude Code — i limiti di frequenza non fanno ripiegare — valga anche su OpenRouter, la 1.7: lo misura il 3, K48 |
| 2 | il disegno del 2026-09-04, riga per riga | la riga 6, *«fuori dallo spazio»*; nella riga 23, che la 5 e la 6 reggano | niente |
| 3 | undici ADR per intero, e le fonti della 3.4 | i dedotti segnati nella 3.1 e nella 3.2; che la regola di `CLAUDE.md` non coprisse il superamento parziale, la 3.3 | niente |
| 4 | lo stato del codice, coi comandi della 4.1 | le due precisazioni della 4.3 | niente |
| 5 | le righe dei quattro documenti, e le tre righe della spec | la riga 10; il primo valore; che nient'altro nell'11 chieda il 5 — la 5.5 | niente |
| 6 | i comandi della 6.1 e della 6.2; il codice fermo da `1be712e`; la lista della guardia, nel sorgente dello script; le voci aperte della porta di qualità, un `grep` per parola | la decisione 1 della 6.1; il legame di E94 col 3 e col 6 — ⚠️ **richiamo del 2026-09-29, dalla rilettura:** E94 è chiusa, e il dedotto è ora che il primo fra 13, 3 e 6 che scrive nel giornale un testo scelto a runtime riapra la forma sigillata, la 6.5; le tre righe della §8; che il piano stia sotto il tetto | niente |

### 6.5 Le voci che restano aperte, e chi le chiude

La **4.6** tiene quelle della porta e del codice che cresce; qui stanno **le altre**, rilette contro la consegna e i
documenti di adesso. Le due tabelle non si sovrappongono: ogni voce ha una casa sola.

| # | Il caso | Chi la chiude |
|---|---|---|
| K11 | un segreto incollato in una nota **non** esclusa resta leggibile, e finisce nel backup del proprietario | il 6: un sensore sulle scritture, col canary che esiste già |
| K17 | *«dimentica davvero»*: una nota cancellata resta nelle copie, nei payload del giornale e nei backup finché non sono potati | il traguardo della ritenzione, e l'11 |
| K19 | maiuscole, separatori, nomi riservati e lunghezza dei percorsi, fra Windows e Linux | chi costruisce ciascun pezzo della porta vera — la 4.2: il 13 per la lettura, il 6 per lo scrivere —, e il 10 |
| K21 | il limite di dimensione delle copie, con avviso, che ADR-0024 chiede e nessuno ha fissato | il primo che scrive file grandi, il 5 o il 7 |
| K22 | la knowledge base che cresce: il router master caricato sempre, e il grafo con migliaia di nodi | il 6, misurando |
| K29 | una root enorme: la scansione incrementale, e l'*«indicizzazione in corso»* dichiarata prima — ADR-0019 | il 6; l'evento *«riscansiona»* è del 13, nella 4.2 |
| K34 | i file *«solo online»* di OneDrive: lo scanner legge l'attributo e non li apre, e restano nodi | il 6, con la regola letta alla fonte in [`riferimenti.md`](../../riferimenti.md) |
| K38 | il sì oltre la sessione, che le app di oggi offrono e ADR-0016 non ha mai valutato | il **proprietario**, con un ADR nuovo se vorrà riaprire il punto 3 di ADR-0016 |
| K41 | dove stanno le regole del proprietario per una zona di lavoro | il 5, che costruisce le zone |
| K48 | se su OpenRouter un limite di frequenza possa essere di un solo modello, e allora un ripiego servirebbe | il 3, misurando |
| **il testo scelto a runtime nei record del giornale** — era **E94** | ⚠️ **Richiamo del 2026-09-29, dalla rilettura del proprietario, voce 3:** questa riga si chiamava **E94**, la diceva aperta citando *«la voce 34 del Traguardo 6»* della [porta di qualità](../../porta-di-qualita.md) — che è la riga 34 della tabella del **Traguardo 5**, stantia: la contraddizione **C-S5-3** —, e la dava al proprietario *«prima del 3, che arriva per primo»*. **E94 del piano del Traguardo 6 è chiusa dal 2026-09-01**, per decisione del proprietario — commit `3c90119` —: i dettagli del giornale che portano testo accettano **solo testo del codice** — campi privati, e `RoutingDetail::new` e `PermissionDetail::new` prendono `&'static str`, `grep -n 'pub fn new' crates/kernel/src/record.rs` —, e tre casi `compile_fail` fanno rosso chi li allarga — `ls crates/kernel/tests/compile_fail/*detail*_is_not_runtime_text.rs`. ⚠️ **Dedotto:** il 3 scrive il nome di un modello scelto a runtime, K47, e il 6 il percorso di una cartella scelta a runtime, K44 e K51; forse già il 13, se identifica una guida caricata col suo percorso — dipende dal suo disegno. Il primo di loro **riapre la forma**: con quale tipo entra il testo scelto a runtime, e come lo stampa il `Debug`. ⛔ **Gotcha #96:** le tre prove tengono i campi che esistono, non la proprietà — un record **nuovo** nasce senza prova, e nulla lo dice. 📌 **Gotcha #99:** un `E<n>` si cita col suo piano | il **disegno del primo** che ne ha bisogno, in una sua sezione che approva il proprietario — la risposta A della rilettura, voce 3; il 6 applica la stessa forma al percorso |
| **V36 · Q22**, e la riga `filesystem` della §8.2.2 | nella §8 della spec l'innesco è **D (5)** — *«si esegue codice o un comando, e si scrive su file reali»* —; ma conservare e ripristinare sui file veri li costruisce il 6, e la suite di conformità della porta nasce col 13 — la 4.2. La lettera resta vera come innesco; se le tre righe debbano potersi chiudere prima è del proprietario: è la forma dell'innesco B (3) di Q6 e Q11, registrata nella stella polare della GUI | il **proprietario**, con la §8.2, la §8.3 e la §8.4 |
| **AUD-004** | la decisione registrata sulle skill: sbarra ancora il 13 — la 5.6. ⚠️ **Detto alla rilettura del 2026-09-29, voce 4:** il 2026-09-04 il proprietario ne scelse il **quando** — in parallelo al 2, e prima del brainstorming del 13 —; il 2 si è chiuso senza l'ADR, quindi la prima metà è in ritardo, e la seconda regge: il 13 non è cominciato | il **proprietario**, con un ADR suo, prima del 13 — confermato alla rilettura, voce 4 |

⚠️ Delle altre voci della consegna, nessuna resta aperta senza casa: le chiuse lo dicono nella consegna archiviata, e quelle
che toccano la porta stanno nella 4.6.

### 6.6 Il prossimo passo

⛔ **Il prossimo passo vive nella §6 del [compendio](../../COMPENDIO.md)**, in un posto solo; qui sta l'ordine.

1. ✅ la sezione 6 — presentata, approvata e scritta il 2026-09-29, con le correzioni della 6.1.
2. ✅ la **rilettura del proprietario** — `superpowers:brainstorming` —, fatta il 2026-09-29: in chat, cinque voci in A/B, una
   alla volta, col consiglio; l'esito nella sezione *«La rilettura del proprietario»*. Il puntatore della §6 del compendio
   passa al piano.
3. ✅ il **piano dei documenti** — **scritto il 2026-09-29**, in una sessione sua, al suo [percorso](../plans/2026-09-29-knowledge-base-revisione-documenti.md); il **pre-controllo** in una sua;
   l'**esecuzione** in una sua, un subagente per compito — `superpowers:subagent-driven-development`. La Definizione di
   «fatto» e le trappole le copia dalla 6.2 e dalla 6.3.
4. poi, come dice la §6 del compendio: il **sotto-progetto 13**, che AUD-004 sbarra — l'ADR del proprietario viene prima —,
   col perimetro della 4.2; e il brainstorming dei **modelli decisionali**, dopo questa revisione.

### 6.7 La verifica chiesta dall'approvazione

Il proprietario ha approvato *«se tutto segue i principi di decision-principles ed è coerente»*. Riletto il testo presentato
in chat contro i documenti, e rilanciati i comandi prima di scrivere; **il merito non cambia**.

| Nel testo presentato | Qui | Perché |
|---|---|---|
| *«nessun file da toccare ha ancora un link a questo disegno»* | il compendio ne ha uno, il puntatore della §6 | il comando E |
| le voci aperte che mancavano, da K11 a K48 | e **E94**: il primo `grep` sulle voci aperte della porta di qualità, con `-i` e più di un `-e`, non aveva reso niente — la trappola 5 —; rifatto una parola alla volta, ha trovato la voce | la voce 34 del Traguardo 6 — ⚠️ **richiamo del 2026-09-29, dalla rilettura:** è la riga 34 della tabella del Traguardo 5, stantia, ed E94 è chiusa — la 6.5 |
| V36 e Q22 | e la riga `filesystem` della §8.2.2, trovata rilanciando i comandi prima di scrivere | lo stesso innesco, **D (5)** |
| i totali, *«anche in `HANDOFF.md` e `AVVIO-CHAT.md`»* | e le righe del compendio e di `README.md` per questo disegno, che nessuna sezione nominava: la 6.2, righe 6, 7 e 14 | il precedente del 2026-09-04, che le scrisse col piano |
| la risposta 1, A | negli altri documenti i totali si riallineano | la decisione 1 del coordinatore, nella 6.1 |
| ADR-0039, *«un richiamo nella sua riga»* | la voce del compendio non cambia — ⚠️ **ribaltata alla rilettura del 2026-09-29, voce 2:** la voce lo nomina | la decisione 2 del coordinatore |
| *«al compendio restano 8 607 byte»* | scritto con la data e il comando, nella trappola 2 | la regola di `CLAUDE.md` sui numeri misurati |

**I cinque criteri**, controllati esplicitamente sulla sezione. **Correttezza:** ogni riga porta il suo comando, rilanciato
prima di scrivere; il `grep` che non aveva reso niente è rifatto, e detto. **Coerenza:** le forme sono quelle del disegno del
2026-09-04 — il controllo per artefatto, la Definizione di «fatto», le trappole, le specie — e dei richiami datati; le voci
aperte stanno in due tabelle che non si sovrappongono. **Debito:** nessuna voce sparisce, e ognuna ha un chiusore; le tre
righe della §8 ed E94 sono scritte, non taciute. **Stato dell'arte:** niente di esterno — la consegna lo diceva, e nessuna
riga ne ha avuto bisogno. **Proporzione:** nessun controllo nuovo nel cancello; comandi, e la guardia che c'è.

## La rilettura del proprietario — il 2026-09-29

Il passo 2 della 6.6, fatto in chat il 2026-09-29 con `superpowers:brainstorming`, in una sessione sua. Prima, i comandi
del *«Come si riprende»* rilanciati, e lo stato dichiarato ha retto riga per riga: `main` allineato dopo un fast-forward,
nessun codice mosso da `1be712e`, `GATE GREEN`, `check-docs.sh` → `OK`, il controllo delle tabelle vuoto. Poi il disegno
riletto per intero, e ogni voce **verificata contro il repository di adesso** prima di porla. Le voci sono le decisioni
del coordinatore della 6.1 e della chiusura, e le voci della 6.5 col proprietario come chiusore; il precedente è la
rilettura del 2026-09-04.

| # | La voce | La risposta | Che cosa ne segue |
|---|---|---|---|
| 1 | i totali degli ADR negli altri documenti di stato — la decisione 1 della 6.1 | ✅ **A**, il consiglio: si **riallineano**, come il 2026-09-03 | niente di nuovo: lo dicono già la 3.3 e la riga 4 della 6.2 |
| 2 | la voce di ADR-0039 nel compendio — la decisione 2 della 6.1, *«non cambia»* | ✅ **A**, il consiglio, che **ribalta** la decisione del coordinatore: la voce nomina **anche** il richiamo nuovo, come fece col rimando del 2026-09-05 e come ogni altro ADR della 3.1 | i richiami nella 3.1, nella decisione 2 della 6.1, nella riga 2 della 6.2 e nella 6.7 |
| 3 | E94 — la decisione 4 della 6.1, e la sua riga nella 6.5 | ✅ **A**, il consiglio, sotto accettazione condizionata — *«se la scelta è coerente con quanto esiste e segue i principi di decision-principles»* —, verificata prima di registrarla: la riga porta il suo nome vero, e la chiude il disegno del primo che ne ha bisogno | la riga della 6.5 riscritta; i richiami nella riga 8 e nella decisione 4 della 6.1, nella 6.4 e nella 6.7 |
| 4 | le voci registrate col proprietario come chiusore — V36, Q22 e la riga `filesystem` della §8.2.2; K38; AUD-004 | ✅ **A**, il consiglio, sotto la stessa condizione, verificata: restano registrate come sono | nella riga di AUD-004, il ritardo sul *«quando»* del 2026-09-04, detto invece di taciuto |
| 5 | le correzioni scritte senza domanda — quelle della 6.1, la decisione 3 della chiusura, e le tre di questa rilettura | ✅ **A**, il consiglio, sotto la stessa condizione — *«coerente e corretta»* —, verificata coi comandi della 6.1 rilanciati: accettate come sue | i richiami della 6.2 e della 6.3 |

⚠️ **Non è stata una domanda** la decisione 1 della chiusura, il commit senza `Co-Authored-By`: è la regola di
`CLAUDE.md`, e la direttiva di sistema della sessione dice da sé che `CLAUDE.md` prevale.

**Le tre correzioni di fatto di questa rilettura**, misurate prima di porle; i comandi sotto la tabella, per F10:

| | Che cosa diceva il disegno | Che cosa è vero |
|---|---|---|
| a | E94 aperta, decisione del proprietario prima del 3 | chiusa dal 2026-09-01, e la riga che la dava aperta è la contraddizione C-S5-3 — la voce 3; il primo comando e il secondo |
| b | il margine del compendio, **8 607 byte**, col tetto e `wc -c` | il numero è la forma **CRLF**, ed è quella che vincola; su un clone LF `wc -c` rende un byte in meno per riga — il comando C nuovo, nella 6.2 |
| c | il `grep` con `-i` e più di un `-e` non rende niente | va in crash, `Aborted` — il terzo comando, sul `grep` 3.0 di Git Bash |

```
git log -1 --format=%B 3c90119
grep -n 'C-S5-3' docs/porta-di-qualita.md
grep -c -i -e E94 -e zzzqqq docs/porta-di-qualita.md
```

## Come si riprende — scritto alla chiusura della rilettura del proprietario, il 2026-09-29

⛔ **Da sapere subito: niente è a metà.** Il disegno è **chiuso**: le sei sezioni approvate e scritte, e la rilettura del
proprietario fatta — la sezione qui sopra. Tutto è pushato: si riparte anche da un'altra macchina, dopo il fetch. La
chiusura precedente sta in
[`archivio/consegna-brainstorming-knowledge-base-revisione.md`](../../archivio/consegna-brainstorming-knowledge-base-revisione.md),
parola per parola.

| | Stato, e il comando che lo rifà |
|---|---|
| ramo | `main` allineato a `origin`: `git fetch --all --prune`, poi `git status -sb`; nessuno stash, `git stash list` |
| i commit di questa sessione | `git log --oneline 1a399c2..HEAD`: la rilettura, con questa chiusura |
| codice di prodotto | **non toccato**: `git diff --stat 1be712e..HEAD -- crates/ gui/ scripts/ Cargo.lock Cargo.toml` non rende nulla |
| cancello | `bash scripts/gate.sh` → `GATE GREEN` all'apertura e prima del commit, e `bash scripts/check-docs.sh` → `OK`: si rilanciano, non si citano |
| le tabelle | ogni riga di tabella di questo file ha le colonne della sua intestazione: il controllo sotto questa tabella non rende niente |
| fine-riga | questo file, il compendio e i due archivi toccati: **LF** nell'indice, la colonna `i/` di `git ls-files --eol`; nell'albero dipende dalla macchina |
| il puntatore | la §6 del compendio: il disegno è chiuso, e il prossimo passo è il **piano dei documenti** |

Il controllo delle tabelle — fuori dalla tabella, per F10:

```
awk '/^```/{c=!c; next} c{next} /^\|/{l=$0; gsub(/\\\|/,"",l); n=gsub(/\|/,"|",l); if(!t){t=1; h=n; s=NR} else if(n!=h) print NR": "n" contro "h" (riga "s")"; next} {t=0}' docs/superpowers/specs/2026-09-28-knowledge-base-revisione-design.md
```

**Il compito della sessione che riprende — il piano dei documenti:**

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è il commit di questa chiusura, o uno dopo.
2. La lettura obbligatoria di `CLAUDE.md`; poi **questo file per intero**, a blocchi: il piano lo traduce, e la 6.2 e la 6.3
   ne sono la Definizione di «fatto» e le trappole, da copiare. La sezione *«La rilettura del proprietario»* fa parte
   dell'ingresso — alla voce 2, ADR-0039 riceve **anche** la riga del compendio.
3. `bash scripts/gate.sh` all'apertura, da solo; e il **margine del compendio** col comando C della 6.2, **prima** di
   scrivere il piano.
4. **Il piano** — `superpowers:writing-plans` —: per ogni artefatto della 6.2, il compito che lo scrive e il controllo che
   lo esercita; `<base>`, nel comando D, è il commit su cui si apre il piano.
5. Il **pre-controllo** in una sessione sua, e l'**esecuzione** in una sua — `CLAUDE.md`, *«Una fase per sessione»*.

**Le decisioni prese in questa sessione, col perché** — il proprietario può ribaltarle:

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | il commit **senza** il trailer `Co-Authored-By` | `CLAUDE.md`, *«senza co-autore»*; la direttiva di sistema dice da sé che `CLAUDE.md` prevale. Costo: un `--amend` |
| 2 | l'esito della rilettura in una sezione sua, prima di questa, e la riga in testa riscritta sul posto | il precedente del 2026-09-04, che fece lo stesso. Costo: nessuno |
| 3 | il comando C nuovo al posto dei due di prima, invece di una nota su quale macchina li misurava | un comando che rende lo stesso numero su ogni clone non ha bisogno di dire dove vale: un'etichetta di forma è una misura travestita, e invecchia come una cifra. Costo: nessuno |

**Che cosa questa sessione ha trovato, e che cosa insegna:**

| Trovato | Che cosa insegna |
|---|---|
| **una voce data aperta da una riga di stato, e chiusa da una sezione dello stesso file** — E94, e la riga 34 della porta di qualità | 📌 *Prima di registrare una voce aperta se ne cerca la chiusura: per nome nel file che la tiene, e nel log — `git log --oneline --grep=E94`. E un `E<n>` si cita col suo piano, gotcha #99* |
| **il margine del compendio misurato con `wc -c`** | 📌 *Il conteggio in byte di un file di testo dipende dal clone, come un'etichetta di fine-riga: si misura nella forma che vincola* |

**Da verificare alla fonte prima del piano:** niente di esterno.
