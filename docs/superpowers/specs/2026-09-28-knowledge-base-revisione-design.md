# Knowledge base, la revisione: il disegno

⏳ **DISEGNO IN CORSO, aperto il 2026-09-29.** Si scrive **sezione per sezione**: ciascuna si presenta in chat a parole
semplici, il proprietario la approva, e solo dopo si scrive qui — `CLAUDE.md`, *«Sezione per sezione»*. La scaletta l'ha
approvata il proprietario il 2026-09-29, e questa tabella è la **casa unica** dello stato delle sezioni:

| # | Sezione | Stato |
|---|---|---|
| 1 | Il modello nuovo, in una pagina | ✅ approvata il 2026-09-29 |
| 2 | Il disegno del 2026-09-04: che cosa si corregge | ✅ approvata il 2026-09-29 |
| 3 | Gli ADR: i rimandi in testa, e l'ADR nuovo del backup | ⏳ da presentare |
| 4 | La porta dei file e il codice che crescerà: chi costruisce che cosa | ⏳ da presentare |
| 5 | Roadmap, tracciabilità, stella polare della GUI e design/09, col perimetro del 13 riletto | ⏳ da presentare |
| 6 | I controlli per artefatto, verificato-dedotto-assunto, le voci aperte e il prossimo passo | ⏳ da presentare |

Chi riprende a metà presenta la prima riga con ⏳. L'ingresso di ciascuna sezione è l'elenco *«Che cosa le risposte
cambiano»* della consegna, in archivio, **riletto** contro i documenti di adesso e non eseguito a scatola chiusa.

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

⏳ **Dove stanno le copie** che il checkpoint tiene prima che l'agente cambi un file — nel backup o no, cifrate o no —
**non è deciso**: K53, con l'ADR nuovo della sezione 3.

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
