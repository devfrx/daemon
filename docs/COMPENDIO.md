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
> accettati — apri **quel** file. Uno, non trentasei. La §12 dice quale.
>
> ⛔ **Cosa NON fare.** Non aprire `HANDOFF.md`, la spec del sotto-progetto 1, o la
> cartella `adr/` «per farsi un'idea». Insieme pesano **oltre mezzo megabyte**
> (593 KB con `wc -c` il 2026-08-09, e possono solo crescere — la spec da sola ne fa 267), e
> l'idea è già qui.

**Aggiornato il 2026-08-09.** Manutenzione: §13.

---

## 1. Il progetto, in dieci righe

Assistente desktop locale, utente singolo, Windows primario poi Linux, **GPU singola
RTX 5080 da 16 GB**, OpenRouter primario con inferenza locale opzionale.

**Piattaforma a quattro pilastri paritari** — conversazione e conoscenza, agenti e
coding, voce, generazione asset 3D — su un **kernel comune**. Nessun pilastro prevale,
nessuno ha accesso privilegiato al kernel.

Il vincolo dominante **non è funzionale ma di risorsa**: quattro aree che si contendono
una sola GPU da 16 GB.

**Il kernel non implementa nessuna funzionalità utente: fornisce i meccanismi.**

⚠️ **Questo non è un repository di sola documentazione.** Il codice del prodotto si
scrive qui, e vive in [`../crates/`](../crates/): cinque crate, con `kernel` e `simulator`
in `no_std`. Gli spike in `spikes/` restano **prove**, fuori dal workspace.

---

## 2. Le sei invarianti del kernel

Vincolano ogni scelta successiva. Una violazione richiede un ADR, non una deroga.

| # | Enunciato | ⚠️ |
|---|---|---|
| **I1** | Lo stato autorevole vive **solo nel core**. GUI e worker non hanno persistenza propria | |
| **I2** | La GPU ha **un solo proprietario**: nessun processo la tocca senza concessione dell'arbitro | riga di verifica **completata** da ADR-0033: copre tutte e tre le classi di processo, non i soli worker |
| **I3** | Il core non contiene **codice OS-specifico**: tutto passa dal modulo di piattaforma | il divieto di *nominare* `std` non basta — la lista delle dipendenze è l'altra metà (ADR-0031) |
| **I4** | Il protocollo IPC è **privato, singolo, non versionato** | «singolo» si legge **per canale privato** — ADR-0035 |
| **I5** | I worker sono **senza stato**: ritentativi, code e priorità stanno nel core | |
| **I6** | Il contenuto non fidato **non attraversa mai** il confine delle istruzioni | |

⚠️ **Gotcha strutturale (#29):** ogni invariante ha due colonne, l'**enunciato** e
**come si verifica**. La seconda è più corta e viene letta al posto della prima, ma è
scritta guardando i casi che esistevano quel giorno. È già successo **due volte** — I2
e I4. Completare una riga di verifica **non è superare l'invariante**: ADR-0004 non è
mai stato superato, ha ricevuto due rimandi. Chi trova il terzo caso lo aggiunga,
invece di aprire un dibattito sull'invariante.

---

## 3. Le cinque proprietà che non si aggiungono dopo

Si ottengono **solo** costruendole dall'inizio. Trascurarle costa una riscrittura — o,
per la quarta, una migrazione.

| # | Proprietà | Da |
|---|---|---|
| 1 | Confine dei dati non fidati **nel sistema di tipi** | I6 · ADR-0014 |
| 2 | Nessuna chiamata OS-specifica nel kernel | I3 · ADR-0002 |
| 3 | **Iniettabilità** di tempo, casualità, I/O e scheduling — **e dei parametri di decisione**, che sono l'altro asse | V29 · ADR-0021 · ADR-0034 |
| 4 | Il **record durevole dichiara la propria versione**, e i campi si identificano per **indice esplicito** | §4.9 · ADR-0036 |
| 5 | **Nessuna esecuzione di codice o comando sotto il livello 2** di confinamento | V35 · ADR-0025 |

⚠️ **La quarta ha una finestra che si chiude da sola:** alla prima riga di codice che
scrive un record. Dopo, la correzione non è una riscrittura ma la **migrazione
dell'unico archivio irriproducibile**.

---

## 4. Lo stack

| Strato | Scelta | Da |
|---|---|---|
| core | **Rust** | ADR-0026, sostenuto da SP-5 e SP-6 misurati |
| gui — forma | **interfaccia web**, non toolkit nativo | ADR-0027, deciso da G7 |
| gui — framework | **Vue 3**, come SPA | ADR-0030 |
| gui — **guscio** | ⚠️ **APERTO**: Tauri o Electron | ADR-0029, `Proposed` — **non blocca nulla** |
| worker ML | **Python** | ADR-0028 |
| persistenza | **`redb` 4.1.0**, con `StorageBackend` scritto da noi | ADR-0032 |
| dipendenze del kernel | **allow-list sul grafo transitivo**, due grafi con rimedi opposti | ADR-0031 · §7.3.1 |
| schema IPC | **`bincode` 2.0.1** — appuntato a `2` | M-1 · §6.1.1 · gotcha #22 |
| formato del **giornale** | **versione + indici espliciti** — `minicbor` 2.3.0, codifica in `kernel` | ADR-0036 · §4.9 |
| formato del **canale worker** | **`minicbor` 2.3.0**, codifica in `kernel`, porta a **byte** | ADR-0037 · §6.10 |
| **edition** | **2024**, su tutte e cinque le crate | scelta dal piano del Traguardo 1 |
| **toolchain appuntata** | `rust-toolchain.toml`: canale **`1.95.0`**, componenti `rustfmt` e `clippy`, bersaglio **`x86_64-unknown-none`** | vincolo 4 di §11 — il bersaglio dichiarato lì si installa da solo su una macchina pulita |
| **nomenclatura** | **codice in inglese, documentazione in italiano** | §1.0 della spec — vedi il riquadro qui sotto |

⛔ **La convenzione di nomenclatura — §1.0, e non è un ADR.**

| | |
|---|---|
| **Codice** | interamente in **inglese**: nomi di crate, moduli, tipi, funzioni, messaggi d'uscita, commenti nel sorgente |
| **Documentazione** | in **italiano** |
| **Riferimenti al codice dentro la documentazione** | in **inglese**, col **nome esatto del sorgente** |

Il costo accettato: fra la parola di un ADR («l'arbitro») e il nome nel codice (`arbiter`)
c'è una traduzione da tenere a mente leggendo. Il beneficio: il codice non stona con un
ecosistema interamente inglese, e non nasce un **dialetto misto**, che è la condizione
peggiore delle due.

⚠️ **Perché sta qui, e perché prima non c'era.** La §1.0 è una **sezione di spec**, non un
ADR: `check-docs.sh` pretende una voce di §5 per ogni file in `docs/adr/`, quindi nessun
controllo ne pretendeva la presenza. Un agente ha letto per intero **entrambi** i file
obbligatori e ha scritto un traguardo intero con gli identificatori in italiano. Gotcha
**#40**.

⛔ **Eccezione, e non è un'incoerenza:** le parole che `check-docs.sh` **cerca dentro i
documenti** restano italiane — `verificato qui`, `parziale`, `rimandato`,
`non controllato`, e l'intestazione «Difende» del catalogo. Sono dati che lo script
confronta, non identificatori.

⛔ **Il kernel porta due serializzatori, e non è duplicazione.** Gli artefatti però sono
**tre**, e a deciderli non è il solo requisito di evoluzione:

| Artefatto | Rinuncia a evolvere? | Il pari sa leggere `bincode`? | Formato |
|---|---|---|---|
| canale `ipc` | sì (I4) | **sì** — M-11 | `bincode` |
| canale `process` | sì (I4) | **no** — M-10 | `minicbor` |
| giornale | **no, deve evolvere** | — (il pari siamo noi) | `minicbor` |

⛔ **Non riaprire §6.1.1** «tanto ora c'è `minicbor` nel kernel». È già stato tentato il
**2026-08-08**, e la misura ha dato torto a chi lo tentava: i due canali privati non sono
lo stesso problema, perché i loro **pari** sono diversi (ADR-0037).

Le cinque crate: `kernel` · `platform` · `secrets` · `simulator` · `daemon`.
Solo **due delle cinque** — `kernel` e `simulator` — sono vincolate da ADR-0031:
`platform`, `secrets` e `daemon` sono **il posto dove l'I/O deve vivere**.

---

## 5. Le trentasette decisioni

Sono **37 ADR**, di cui **36 ADR in stato Accepted** e uno `Proposed` (0029).
Ordine numerico. Il *perché*, le alternative scartate e i costi accettati stanno nel
file di ciascuno: `docs/adr/`.

**0001 — Kernel con capacità paritarie.** Il sistema è un *kernel* con *capacità*.
Conversazione, conoscenza, agenti, coding, voce e generazione asset sono consumatori
**paritari** degli stessi servizi centrali: arbitro GPU, gateway di inferenza,
persistenza, permessi, bus eventi. **Nessuna capacità ha accesso privilegiato né
scorciatoie verso il kernel** — è questa regola a rendere la scelta reale invece che
dichiarativa. È la decisione fondativa: tutto il resto ne discende.

**0002 — Windows primario, con confine OS esplicito.** Si sviluppa e si testa su
Windows, ma ogni punto OS-specifico sta fin da subito dietro un **modulo di
piattaforma** con interfaccia definita e implementazione fittizia per i test. Linux si
aggiunge implementando quel modulo, senza rimettere mano al kernel. Il confine è un
vincolo **verificabile**, controllato staticamente sui grafi di importazione (I3).

**0003 — Estensibilità solo tramite MCP e skill dichiarative.** **Nessun codice di
terze parti gira nel processo dell'applicazione.** Esistono esattamente due
meccanismi: **server MCP** (processo esterno, permessi propri, revocabile) e **skill
dichiarativa** (istruzioni e dati, **non** codice eseguibile, quindi nessun isolamento
necessario). Coprono la sostanza di un marketplace di plugin senza il contratto
pubblico da congelare e senza la superficie d'attacco.

**0004 — Topologia di processo.** Tre classi, non una di più.

| Classe | Quante | Vita | Possiede |
|---|---|---|---|
| **core** | una, istanza singola | lunga, indipendente dalla GUI | **tutto** lo stato autorevole |
| **gui** | 0..1 | effimera, **sacrificabile** | solo stato di presentazione |
| **worker** | 0..N | breve, **uccidibile in qualsiasi istante** | nulla |

I worker sono **pochi, stupidi e a vita breve**: non hanno stato proprio, non si
parlano tra loro, non decidono, non ritentano. Non sono micro-servizi. Qui vivono le
sei invarianti (§2). La topologia a micro-servizi è stata scartata **perché più
costosa e peggiore**: distribuisce l'unica cosa che deve restare centralizzata.

**0005 — Arbitrato GPU su due dimensioni.** VRAM = capacità **esclusiva**, meccanismo
di **ammissione** (o entra, o non parte). Calcolo = contesa **condivisibile**,
meccanismo a **corsie**. Ogni tipo di lavoro dichiara un **profilo di risorsa**
nominato e versionato (riserva VRAM, classe di calcolo, prelazionabilità, tempo di
rilascio, avvio a freddo); la riserva è dichiarata dal richiedente e **verificata**
dall'arbitro, e una riserva sistematicamente sbagliata è un difetto del profilo. **La
quota VRAM dell'audio è sottratta dal budget all'avvio e non vi rientra**: un budget
sottratto non può essere allocato per errore, mentre una priorità può essere rispettata
tardi. Nessun degrado silenzioso.

**0006 — Le due policy VRAM sono oggetti distinti.** Non due rami di un condizionale:
**due oggetti che implementano la stessa interfaccia**, uno solo attivo, scelto dal
profilo di configurazione. Il passaggio è una **transizione esplicita con effetti
osservabili** — eviction, ricarica, notifica all'utente — **offerta** all'utente, non
imposta. La duplicazione fra due policy è visibile e circoscritta; la deriva di un
condizionale è invisibile e diffusa. Default: **OpenRouter, VRAM libera**.

**0007 — Giornale write-ahead e ripresa come riconciliazione.** Giornale
**append-only**: l'intento di ogni passo è reso durevole **prima** dell'esecuzione,
l'esito dopo. Un passo con intento e senza esito è **in dubbio**, e il dubbio è sempre
*rilevabile*. **La ripresa è riconciliazione, non replay cieco.** Ogni effetto dichiara
la propria **classe**, e la classe determina la riconciliazione:

| Classe | Riconciliazione del dubbio |
|---|---|
| `verificabile` | interroga il mondo, poi completa o ripianifica |
| `idempotente` | riesegui |
| `irripetibile` | **sospendi e chiedi all'utente** |

**Un effetto senza classe dichiarata è trattato come `irripetibile`**: davanti a un
dubbio non risolvibile il sistema si ferma, non indovina.

**0008 — Il contesto è una proiezione dello stato durevole, non lo stato.** La finestra
si compone dagli elementi durevoli a ogni passo, e la compattazione **ricalcola la
proiezione** invece di riassumere la conversazione. **Mai sacrificabili:** obiettivo,
vincoli e regole applicabili, piano e stato dei passi, decisioni con il motivo, fatti
con la provenienza, artefatti come **riferimenti** (il contenuto si rilegge su
richiesta). **L'unica cosa sacrificabile è la trascrizione grezza** — ed è il cuore
della decisione: tutto ciò che serve a proseguire è strutturato, quindi non finisce mai
nel tritacarne del riassunto.

**0009 — Guide, sensori e anelli sono meccanismi di kernel.** Il kernel espone quattro
meccanismi **generici**; il contenuto lo portano le capacità: **registro delle guide**
(archiviazione, versionamento, iniezione nella proiezione) · **registro dei sensori**
(esecuzione, classificazione per costo, verdetti nel giornale) · **anello di verifica**
(dopo un passo esegue i sensori applicabili; un verdetto negativo rientra come
feedback) · **trigger** (l'anello parte da eventi — pianificazione, cambiamento di
file, fine di un'altra run — non solo dall'utente). Contratto del sensore,
deliberatamente povero: `(artefatto) → (verdetto, dettaglio, costo)` — un contratto
minimo si può allargare, uno ricco e sbagliato no. **Anello di miglioramento:** quando
un problema si ripete **si migliora il controllo, non il prompt**; il kernel rileva la
ricorrenza e **propone**, l'utente **approva**. Non si auto-modifica in silenzio. Le
skill dichiarative di 0003 sono **guide**.

**0010 — La proiezione ha un budget di qualità, non una soglia di riempimento.** Budget
target espresso come frazione della finestra, configurabile per modello. La
ricomposizione è **continua e proattiva**: serve a mantenere il budget, non a evitare
l'overflow. Il limite della finestra resta come **guardia**, non come politica. La
proiezione è **misurata per categoria** e la misura entra nel giornale — senza, «il
contesto è troppo pieno» è un'impressione e non un dato.

**0011 — La politica di routing è risolta e giornalata per ogni richiesta.** Ogni
richiesta produce un **record di routing risolto**: modello, destinazione, provider,
parametri, vincoli richiesti, catena di riserva valutata, tentativi, esito. Contiene la
decisione **risolta**, non un riferimento alla configurazione — rileggere la
configurazione di oggi non dice cosa accadde ieri. La contabilità **cavalca l'identità
del giornale** (passo → run → run padre): le quattro granularità sono aggregazioni
della stessa gerarchia, non quattro contatori.

⛔ **Corollario accettato, ed è centrale:** ogni richiesta di inferenza **generativa**
è un passo di una run — **anche un singolo messaggio di chat**. Una conversazione è una
run interattiva di lunga durata. **Non esiste un percorso «chat» accanto a un percorso
«agente».** ⚠️ **Confine netto:** l'inferenza **percettiva** always-on — wake word,
VAD, trascrizione continua — **non** è un passo: è una **sorgente di eventi** (anello
3), non passa dal gateway, e giornalarla violerebbe Q1. La trascrizione che diventa un
messaggio **apre** un passo; i frammenti audio che l'hanno prodotta no. Il costo si
registra **anche per gli stream interrotti**.

**0012 — L'equivalenza del fallback è definita dai vincoli, e sui dati si fallisce
chiuso.** Un candidato che viola un vincolo della richiesta **non è un fallback: è una
richiesta diversa**, scartata prima della valutazione. Due classi con esiti opposti a
catena esaurita:

| Classe | A catena esaurita |
|---|---|
| vincoli **su dati e riservatezza** (ritenzione, provider esclusi, solo locale) | **fallisce chiuso**: errore, nessun ripiego |
| vincoli **su qualità e costo** (tetto di prezzo, modello preferito, latenza) | **degrado dichiarato**: procede avvisando |

L'**indisponibilità di risorsa** è causa di fallback di prima classe: se l'arbitro GPU
rifiuta o accoda, non è un errore. **Un ritentativo non è un passo nuovo.**

**0013 — La conformità allo schema è un verdetto di sensore, non un'eccezione.** La
validazione dello schema è un **sensore computazionale**: un output non conforme
produce un verdetto negativo con dettaglio, che rientra nell'anello come qualsiasi
altro. La correzione è un **passo nuovo**, giornalato. La violazione di schema diventa
osservabile e misurabile insieme a tutte le altre.

**0014 — Il confine dei dati non fidati vive nel sistema di tipi, e la sanitizzazione
non esiste.** Il contenuto esterno viaggia in un **tipo distinto** da quello che
trasporta le istruzioni: non è assegnabile a un campo istruzione, la conversione
richiede un passaggio esplicito, e il passaggio è **giornalato**. **L'etichetta è
ereditaria:** estrarre, riassumere, tradurre o concatenare produce ancora contenuto non
fidato — altrimenti basterebbe un riassunto per ripulire un attacco. **Non esiste
sanitizzazione:** non si tenta di rimuovere istruzioni dal testo.

> **Un'istruzione trovata nei dati non è mai un'autorizzazione.** Il contenuto non
> fidato può *informare*, mai *autorizzare*.

Conseguenza: ogni azione la cui **decisione** dipende da contenuto non fidato richiede
la stessa autorizzazione che richiederebbe se l'utente non l'avesse chiesta.

**0015 — Le descrizioni degli strumenti sono fissate all'approvazione.** La descrizione
si mostra **integralmente** all'utente all'approvazione — non solo il nome, ma il testo
che influenzerà il modello — e se ne registra l'**impronta**. Se cambia, lo strumento è
**sospeso** finché non è ri-approvato, con il diff mostrato: non degradato, non
avvisato-e-usato. Nella proiezione le descrizioni sono marcate come **dichiarate da
terzi**. **Una descrizione non concede permessi**: è testo, non autorità.

**0016 — Permessi come tripla, e default dei vincoli sui dati per profilo.** Un
permesso è una **tripla** `(strumento × risorsa × operazione)` — `(file, ~/x, lettura)`
e non «il filesystem», `(rete, host specifico, uscita)` e non «internet». Tre preset di
supervisione: `chiede sempre` · **`auto-approva sicuri`** *(default: letture, test e
build procedono; scritture, comandi e uscite di rete chiedono)* · `autonomo` (conferma
solo per effetti `irripetibili` e azioni fermate da un sensore). **Un'approvazione non
si estende:** vale per quella tripla e per quella sessione. Il default dei vincoli sui
dati lo dichiara il profilo, **ma qualunque richiesta il cui contenuto abbia
attraversato il gestore dei segreti sale automaticamente alla classe più stretta**, e
se non trova endpoint conforme fallisce chiuso. **Canary di esfiltrazione:** valori
sentinella nel gestore dei segreti; la loro comparsa in uscita è un verdetto di sensore
che blocca.

**0017 — Il giornale è la sorgente, il trace è una proiezione.** Trace, contabilità,
metriche e dataset di regressione sono **proiezioni** del giornale. Si adotta il
vocabolario **GenAI di OpenTelemetry** per la proiezione, **non** per l'archiviazione:
se la convenzione cambia, cambia la proiezione, non i dati. **Nessuna telemetria lascia
la macchina per default**; l'esportazione OTLP è **opt-in** con destinazione scelta
dall'utente.

**0018 — Ritenzione a livelli: la struttura sopravvive, i payload si potano.**
Struttura (identità di run e passi, transizioni, esiti, record di routing, costi,
verdetti, decisioni) → ritenzione lunga, ed è la parte piccola. Payload (prompt,
risposte, output degli strumenti, trascrizioni) → finestra breve, poi **potati e
sostituiti con impronta e dimensione**. Artefatti → **riferimenti**, il contenuto vive
sul filesystem. **La potatura è irreversibile e va dichiarata:** un payload assente e
uno mai registrato non devono essere indistinguibili. **Un passo in dubbio non è mai
potabile** finché non è riconciliato.

**0019 — Lo stato di degrado è un oggetto osservabile, non una collezione di errori.**
Il core mantiene uno **stato di degrado corrente**, aggiornato dagli eventi
(connettività, arbitro GPU, salute dei provider, permessi, strumenti sospesi), e lo
espone come oggetto osservabile. **Il principio: si dichiara prima, non si fallisce
dopo** — l'utente deve sapere cosa è disponibile *prima* di tentare. Generalizza a
tutto il sistema il «nessun degrado silenzioso» di ADR-0005: era una regola locale,
diventa una proprietà del kernel.

**0020 — Nessun modello nel percorso decisionale del kernel.** I modelli sono invocati
*attraverso* il kernel e i loro esiti sono **dati opachi**, mai giudizi su cui il
kernel basa il proprio comportamento. Corollario: **il kernel è testabile interamente
senza chiamare un modello.** La valutazione probabilistica — giudice, dataset curati,
trace-based eval — appartiene alle capacità L2, dove il non-determinismo vive davvero.

**0021 — Simulazione deterministica, e iniettabilità come requisito di costruzione.**
Il kernel si verifica con **simulazione deterministica** per concorrenza, crash e
ripristino, e con **crash-injection** ai confini di persistenza. **Tempo, casualità,
I/O e scheduling sono iniettabili** — requisito di **costruzione**, non infrastruttura
di test: nessun componente legge l'orologio, genera casualità o esegue I/O se non
attraverso un confine sostituibile. **Ogni difetto trovato conserva il proprio seme**,
e il seme diventa una regressione permanente. Rimando: ADR-0034 aggiunge il **secondo
asse**, i parametri di configurazione.

**0022 — Layout dei dati per natura, e backup del solo irriproducibile.** Separazione
**per natura, non per componente**; ogni archivio ha la propria politica.

| Archivio | Cifrato | Nel backup |
|---|---|---|
| giornale | **sì** | sì |
| artefatti | no — sono già file dell'utente | sì |
| configurazione, guide, profili | no | sì |
| indici ed embedding | no | **no** — rigenerabili |
| pesi dei modelli locali | no | **no** — gestione dedicata |
| segreti | **sì, con chiave propria** | ⛔ **mai** |

I segreti sono esclusi perché **un backup che trasporta chiavi API è un vettore di
fuga**, non una comodità. Quattro requisiti del motore di persistenza; il **quarto** —
ogni operazione di I/O **iniettabile** — è quello che ha poi deciso ADR-0032.

**0023 — Cifratura a riposo con chiavi dell'OS, e gestore dei segreti unico.** Le
chiavi le gestiscono le facility dell'OS, raggiunte dal modulo di piattaforma (I3); il
daemon parte senza interazione. **Onestà sulla forza reale:** qui «cifrato a riposo»
significa **protetto quanto il tuo account di sistema**, e va scritto **in interfaccia**
— una falsa sicurezza è peggio di nessuna sicurezza. **Il gestore dei segreti è
l'unico punto di lettura delle credenziali**, e da questo punto unico discendono tre
meccanismi già decisi: mascheratura nel record di routing, escalation automatica dei
vincoli sui dati, canary di esfiltrazione. **Profilo «riservato»** opzionale con
passphrase, che **disattiva avvio automatico e voce always-on** — mutuamente esclusivi,
e fingere il contrario sarebbe disonesto.

**0024 — Il checkpoint del filesystem copre ambiti dichiarati.** Un **ambito di
lavoro** è un insieme di percorsi dichiarato esplicitamente; il checkpoint copre quelli
e nient'altro. Prima che un effetto tocchi un file dentro un ambito, la versione
precedente è **conservata** e riferita dal passo del giornale. È il write-ahead
applicato ai file. **Distinto da git e vi convive:** il checkpoint è automatico e a
grana di passo, git è intenzionale e a grana di commit. **Limite dichiarato:** gli
effetti fuori dagli ambiti non sono coperti.

**0025 — Confinamento a livelli.** Quattro livelli. Il kernel **richiede** un livello
per ogni azione; la piattaforma lo **implementa** (I3), e il kernel non sa come.

| Livello | Confine | Regge contro codice eseguito? |
|---|---|---|
| **0** | nessuno | no |
| **1** | permessi applicativi | ⛔ **no** |
| **2** | processo ristretto dell'OS | sì |
| **3** | macchina virtuale leggera | sì, anche a fuga dal kernel guest |

**Default: livello 2 minimo per qualsiasi esecuzione di codice generato o di comando.**
Il livello 1 resta ammesso solo per strumenti interni che non eseguono codice; il
livello 3 è opzionale e fuori dal primo traguardo. **Se il livello richiesto non è
disponibile, l'azione non parte** — fail-closed: un confinamento più debole non è un
ripiego, è un'altra cosa.

**0026 — Il core si scrive in Rust.** Ha deciso lo **spareggio #1**: *il controllo
deterministico è **posseduto** o soltanto **fornito** dai test?* — l'unico su cui i tre
candidati divergono in modo non recuperabile, e discende da V29 e ADR-0021, che
dichiarano la simulazione deterministica **non retrofittabile**. In Rust l'ordine delle
unità concorrenti è deciso dall'esecutore del kernel e vale **anche fuori dai test**;
in Go lo scheduler appartiene al runtime e il determinismo è *fornito* solo dentro i
test, e misurato solo parzialmente; in TypeScript il controllo esiste solo rinunciando
ad `async`/`await`, e senza parallelismo reale. ⚠️ **L'esito non era scontato:** i
criteri erano fissati prima che i candidati esistessero, e la verifica su Go è stata
eseguita per **falsificare** l'attesa.

**0027 — La GUI è un'interfaccia web, non un toolkit nativo.** Ha deciso **G7 —
artifacts o canvas con anteprima viva**: rendere contenuto arbitrario prodotto da un
modello con anteprima viva **richiede** un motore web, e un toolkit nativo dovrebbe
incorporarne uno comunque, cioè pagare due stack invece di uno. Rinforzano G6 (viewer
3D) e G20 (accessibilità: quella del web è la più matura). **Scelta a basso rischio per
costruzione:** se fosse sbagliata, la GUI si riscrive **senza toccare il kernel**.

**0028 — Worker ML in Python.** Non è una scelta: i modelli hanno implementazioni
Python. L'ADR ne dichiara i costi. Ciò che un worker **non** contiene, e non è una
raccomandazione: logica di ritentativo (I5), code e priorità (I5), stato che sopravviva
al processo (I1), comunicazione con un altro worker (il core coordina), accesso alla
GPU senza concessione (I2). Un worker può essere **ucciso senza preavviso**.

**0029 — ⚠️ Guscio della GUI: DECISIONE APERTA.** `Proposed`. Raccomandazione
**Electron**, ma sono **argomenti, non misure**, ed è per questo che resta aperta. Si
chiude con **M1–M5** all'inizio del sotto-progetto 2: RAM a riposo e sotto streaming ·
dimensione del pacchetto · fps del viewer 3D e API grafica reale **su Windows e
Linux** · P3 con rendering vero · **M5**, VRAM a riposo e sotto carico 3D (aggiunta da
ADR-0033). Se M3 mostra la stessa API grafica su entrambe le piattaforme con Tauri, la
decisione si **ribalta**. ✅ **Non blocca il sotto-progetto 1**, che è interamente Rust
e non tocca la GUI.

**0030 — L'interfaccia si scrive in Vue 3, come SPA.** Ha deciso la **competenza del
proprietario**, criterio **legittimo qui** perché nessuna invariante vincola la scelta
e la GUI è l'artefatto più sacrificabile del sistema (in ADR-0026 non lo era). Per le
componenti pesanti si preferiscono le librerie **agnostiche** rispetto al framework
(`three`, `codemirror`) alle incapsulazioni Vue: sopravvivrebbero a un cambio.

**0031 — Le dipendenze del kernel sono parte del confine I3.** Le crate che devono
essere deterministiche e prive di OS — `kernel` e `simulator` — hanno una **lista
nominata** delle dipendenze ammesse, verificata sul grafo **transitivo**. Cinque
regole: ogni voce porta la propria **giustificazione scritta** · il controllo è sul
grafo **transitivo**, perché il pericolo arriva di rimbalzo · il controllo è **provato
in negativo** · aggiungere una voce è un **atto deliberato e rivedibile** · **la lista
nasce vuota**. ⚠️ `simulator` non aggiunge voci proprie **ma il suo grafo non è vuoto**:
dipende da `kernel`. **Perimetro:** `platform`, `secrets` e `daemon` **non** sono
vincolati — è lì che l'I/O deve vivere. Oggi la lista contiene `bincode` 2.0.1 con
`unty`, e `minicbor` 2.3.0. ⚠️ Il grafo **di build** è passato a sette voci, e per la
prima volta il kernel porta `syn` a tempo di compilazione.

**0032 — Motore di persistenza: `redb` 4.1.0, con il backend sotto il nostro
controllo.** Usato con uno `StorageBackend` **scritto da noi** invece di quello su file
predefinito. Il backend nostro **non è un dettaglio**: è il punto in cui il requisito 4
(I/O iniettabile) diventa reale. Due implementazioni: backend su file in `platform`
(l'I/O vero) e backend **cadente in memoria** in `simulator` (cade a un'operazione
scelta dal seme — è **l'iniezione di livello 2**). `redb` vive in `platform`, quindi
ADR-0031 non lo vincola: il kernel conosce solo la porta `journal`.

**0033 — La GPU della GUI: quota di presentazione sottratta, concessione tenuta dal
core.** Il consumo GPU della GUI si modella come **tre consumatori distinti**:
compositing della webview e viewer 3D **entro** la quota (dentro la quota di
presentazione, **nessun rifiuto esecutivo**) · viewer 3D **oltre** la quota
(**concessione ordinaria** richiesta via IPC, rifiuto esecutivo sì).

```
budget allocabile = totale − quota audio − quota presentazione
```

Il **core** richiede all'avvio una concessione di presentazione **permanente e non
prelazionabile**; la GUI la consuma **senza mai chiederla**. Regge perché: la
concessione è stato del core (I1) · la sottrazione **non è esenzione**, la concessione
ha un titolare · **sopravvive alla GUI uccisa in qualsiasi istante**, quindi nessun
protocollo di liveness contro un processo progettato per morire · la quota non si
libera a GUI chiusa, o la GUI riaperta andrebbe in OOM. Se la GUI muore tenendo una
concessione ordinaria, il core se ne accorge dalla **disconnessione IPC** e riconcilia.
**I2 si completa, non si riformula.**

**0034 — I parametri di decisione sono consegnati al kernel, non letti.** **Nessuna
decisione del kernel legge un parametro che non le è stato consegnato.** Il kernel
**non legge la configurazione**: riceve alla costruzione un valore con i **parametri
risolti** · **non nomina** un file, una chiave o un default, e nessuno dei tre è
esprimibile al suo interno · chi **produce** il valore è `daemon`, che in produzione lo
ricava dall'archivio via `platform` e in simulazione lo riceve dal banco · la
**sostituzione** di un parametro è un passo giornalato. ⛔ **Perimetro negativo:** non
è un sistema di configurazione (niente formato, schema, validazione, ricarica a caldo),
non è un registro a chiavi stringa, non è sostituzione a caldo generalizzata, e non
decide il formato dell'archivio. In sotto-progetto 1 i default sono **letterali in
`daemon`**.

**0035 — La porta verso i worker, e cosa significa «singolo» in I4.** Il dialogo con un
worker vive dentro la porta **`process`**, che copre **avvio, dialogo e uccisione** —
non nasce una porta nuova, le famiglie restano sei. Gli schemi dei due canali privati
sono **distinti**, ed entrambi vivono in `kernel`. **«Singolo» significa: un meccanismo
di trasporto e uno schema _per canale privato_** — nessun broker, nessun service
discovery, nessuna negoziazione, nessun versionamento. Ciò che I4 compra è che non
esista un **contratto pubblico** da congelare, e nessuno dei due canali ha consumatori
esterni. Il rifiuto di un pari stantio resta il **timbro di build**, identico sui due
canali. **I4 si completa, non si riformula.**

**0036 — L'evoluzione del formato durevole del giornale.** **Ogni record durevole
dichiara la propria versione, e i suoi campi si identificano per indice esplicito.**

| # | Regola |
|---|---|
| 1 | il tipo del record è un **enum di versione**: «un record senza versione» **non è esprimibile** (livello 1, compilatore) |
| 2 | ogni campo porta un **indice esplicito**, scritto nel tipo e leggibile nel diff — costa **un byte** |
| 3 | un campo nuovo è **facoltativo** e prende un **indice nuovo** |
| 4 | un indice **si ritira e non si riusa mai**: il buco resta |
| 5 | un cambiamento **non additivo** apre una **versione nuova**; il lettore dispaccia e converte |
| 6 | la **codifica vive in `kernel`**, e la porta `journal` scambia **byte** |

La codifica sta in `kernel` per coerenza di proprietà, perché così il simulatore scambia
byte e la campagna DST esercita davvero codifica e decodifica **iniettando i crash
dentro la scrittura**, e perché il costo misurato è piccolo. **Il controllo è uno solo**
— i **byte congelati** nel repository con la mappa `indice → nome → valore atteso` — e
non due, perché un registro separato sarebbe un secondo posto da tenere allineato e il
primo che smette mente in silenzio. ⛔ **I byte congelati non si rigenerano:** se
cambiano non è un aggiornamento, è un **cambio di formato**.

**0037 — Il criterio del pari.** **Il formato di un canale privato si sceglie _anche_
sull'ecosistema di chi lo legge, e la risposta si _misura per pari_.** M-1 chiedeva se il
**grafo transitivo** fosse accettabile: domanda giusta per I3, ma interamente sul
**nostro** capo del filo. Un canale privato ne ha due, e il secondo non è Rust. La seconda
metà — ***«il pari ha un lettore conforme e mantenuto?»*** — non era scritta da nessuna
parte, e P1 sembrava rispondervi pur avendo **due binari Rust** ai due capi.

| Canale | Il pari | Misura | Formato |
|---|---|---|---|
| `ipc` | TypeScript | **M-11**: `bincode-ts` decodifica, valori giusti | **`bincode`** — §6.1.1 **confermata**, non riaperta |
| `process` | Python | **M-10**: nessuna libreria per `bincode` | **`minicbor`** — voce già spedita, lista invariata |

⚠️ **Due canali privati con formati diversi non sono un'incoerenza:** la differenza è
**misurata**, non accidentale, e non va «sanata». ⛔ E un decodificatore scritto e
mantenuto **da noi** nel linguaggio del pari **non è una via**: è una seconda definizione
dello schema, e misurato sbaglia **in silenzio** — un lettore ingenuo del varint ha
restituito `251` al posto di `4096` senza sollevare nulla.

---

## 6. Dove siamo, e cosa viene dopo

**Spec del kernel §0–§10 completa.** Spec del **sotto-progetto 1** con §0–§8 approvate,
**riaperta su sette voci** — **tutte chiuse** — **§8 riallineata e chiusa il 2026-08-08**, e
**audit sezione-contro-ADR passato**.

✅ **Il Traguardo 1 è eseguito, il 2026-08-08.** Il codice del prodotto non è più zero righe:
esiste il workspace, esiste la porta di qualità, e la porta è **verde**.

| | |
|---|---|
| **il workspace** | alla radice, `resolver = "3"`, edition **2024**, `spikes/` fra gli `exclude`. Le cinque crate di §1.2 esistono tutte |
| **`kernel` e `simulator`** | `#![no_std]` + `alloc` + `#![forbid(unsafe_code)]`, e **nessuna logica di prodotto**: è deliberato, non un lavoro lasciato a metà |
| **la porta** | un comando solo — `bash scripts/gate.sh` — con **sei** controlli: build del workspace · test · cancello senza OS · allow-list sui due grafi · **attributi delle crate vincolate** · coerenza della documentazione. La CI lancia lo stesso comando, `.github/workflows/quality-gate.yml` |
| **la mappa dei controlli** | [`porta-di-qualita.md`](porta-di-qualita.md): ogni riga del catalogo §7.4 → il file che la implementa, con le sonde per nome e ciò che **non** è ancora coperto |
| ⛔ **quattro trappole nuove** | gotcha **#38**, **#39**, **#40**, **#41**, più una **seconda occorrenza** di #26 e una di #25. Non erano deducibili: sono uscite eseguendo |

⛔ **Una revisione ha aggiunto una riga al catalogo il 2026-08-09, ed è stata misurata.** Il
quinto controllo della porta — `gate-attributes.sh` — ne copre ora **due**: che le crate
vincolate **dichiarino** i propri attributi, e che **non abbiano un build script**. Un
`crates/kernel/build.rs` che chiama l'orologio, il filesystem e l'ambiente e inietta il
risultato con `cargo:rustc-env` lasciava la porta **verde su sei controlli su sei** —
`build` e `test` lo compilano perché è il mestiere di un build script, il cancello senza OS
lo compila **per l'host** e lo **esegue**, l'allow-list non vede nodi nuovi, gli attributi
leggevano il solo `src/lib.rs`. Difende **I3 e V29 direttamente**, non è di ramo 1b: è il
gotcha **#28**, un valore del mondo cotto dentro il kernel. Il catalogo **§7.4.2 passa da
dodici a tredici** voci di livello 2; il registro è [`porta-di-qualita.md`](porta-di-qualita.md).
📌 Lo stesso commit ha rinominato il workflow in `quality-gate.yml`: era l'**ultimo residuo
italiano nel codice**, e costava zero solo finché non fosse mai stato eseguito.

⛔ **Il piano ha ricevuto un'errata in testa, e non si riscrive.** La prima voce sono gli
identificatori: il piano li detta **italiani**, il codice eseguito è in **inglese** perché
lo impone la §1.0 della spec — «spec prima del codice», e il piano non aveva l'autorità per
derogarvi. Le altre tre sono, in ordine, la seconda occorrenza di **#26**, quella di
**#25**, e il gotcha **#39**.

Le sette voci sono emerse rileggendo `tracciabilita.md` con una domanda che nessuno le
aveva posto: ***«di quale meccanismo di kernel ha bisogno questa funzionalità, e la
spec lo nomina?»***. La crepa è la **legenda**: `📋` significa «sotto-progetto
assegnato», **non** «non richiede un meccanismo di kernel».

| Voce | Cosa | Stato |
|---|---|---|
| **F3** | i parametri di decisione non erano consegnati al kernel | ✅ chiusa — ADR-0034, §2.8 |
| **F6** | la VRAM totale non aveva provenienza | ✅ chiusa con F3, §5.1 |
| **F5** | `network` era «verso i provider», V25 promette «verso la **rete**» | ✅ chiusa — §2.3.1 |
| **F1a** | nessuna porta per **parlare** con un worker | ✅ chiusa — ADR-0035, §2.3.1 |
| **F2** | l'evoluzione del formato durevole del giornale | ✅ chiusa — ADR-0036, §4.9 |
| **F7** | fork e branching | ✅ chiusa con F2 — §4.9.5 |
| **F1b** | il **progetto** della porta `process` in §5–§6 | ✅ chiusa — ADR-0037, §6.10 |
| **F4** | l'anello 3 non è collocato in §0.4 | ✅ chiusa — §0.4.3 |

### L'ordine, già deciso

1. ~~**§8**~~ — ✅ **chiusa il 2026-08-08**, toccata una volta sola come previsto.
2. ~~**Il piano**~~ — ✅ **scritto**: [Traguardo 1](superpowers/plans/2026-08-08-sottoprogetto-1-traguardo-1-scheletro-e-porta.md).
3. ~~**Il codice del Traguardo 1**~~ — ✅ **eseguito** subagent-driven, otto compiti più quattro di riallineamento alla §1.0. `GATE GREEN`.
4. ~~**Il piano del Traguardo 2**~~ — ✅ **scritto il 2026-08-09**: [Traguardo 2](superpowers/plans/2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md), quattordici compiti in due parti.
5. 🔵 **Il codice del Traguardo 2 — in corso.** ✅ **Task 1–6 eseguiti** subagent-driven il 2026-08-09, `GATE GREEN` a ogni compito. ⏭️ **Il prossimo passo è il Task 7** — il reattore reale in `platform`, e la prima suite di conformità fra porta finta e porta vera.

⛔ **Nessuna rinumerazione di sezioni**: lo script legge §7.4 e §8 **per posizione**.

### Il Traguardo 2, compito per compito — dove riprendere

| # | Compito | Stato |
|---|---|---|
| 1 | i due tempi, `Monotonic` e `WallTime`, e il terzo che li lega | ✅ |
| 2 | la porta `Rng`, e l'implementazione seminata in `simulator` | ✅ |
| 3 | i parametri di decisione, consegnati e non letti | ✅ |
| 4 | la porta `Reactor` | ✅ |
| 5 | **l'esecutore** | ✅ |
| 6 | il reattore finto, e la misura dell'interlacciamento | ✅ |
| **7** | **il reattore reale in `platform`, e la prima suite di conformità** | ⏭️ **riprende qui** |
| 8 | il cablaggio di produzione in `daemon`, coi default letterali | ⬜ |
| 9–12 | il confine dei tipi · `journal` e `filesystem` · `process` coi gettoni · `ipc` | ⬜ |
| 13–14 | il registro dei controlli · la chiusura del traguardo | ⬜ |

⛔ **Cosa il Task 7 deve coprire, e lo dice chi ha scritto il Task 6:** il ramo
`deadline <= now → None` di `VirtualReactor::wait_until` **non è esercitato da nessun test**
— l'esecutore non lo raggiunge più da quando promuove i dormienti scaduti — e
`VirtualReactor::wall_time()` **non è letto da nessuno**. Sono contratto della porta: o li
copre la suite di conformità, o partono non provati.

📌 **Sei difetti del piano trovati eseguendo, non leggendo**, e il più grave è invisibile
per costruzione: la cella `Sleep` veniva svuotata **solo sul ramo `Pending`**, quindi
un'attività che chiedeva di dormire e poi finiva lasciava la richiesta alla successiva.
**C1 resta verde** — la fuga è deterministica, quindi riproducibile e perciò invisibile a un
controllo di riproducibilità. Regressione permanente su un intervallo di semi, non su uno.

### Il sotto-progetto 1 si esegue a traguardi, e ciascuno ha il proprio piano

Scrivere ora un piano per codice che non esiste significa inventare. **Il Traguardo 1 è
eseguito e il piano del 2 è scritto; quelli dal terzo in poi si scrivono quando si arriva.**

| # | Traguardo | Stato |
|---|---|---|
| **1** | **scheletro e porta di qualità** — le cinque crate e i controlli, **zero logica** | ✅ **eseguito il 2026-08-08**, `GATE GREEN` |
| **2** | **il substrato iniettabile** — tempo, casualità, I/O, scheduling, l'esecutore, le sei porte | 🔵 **in corso**. [Piano](superpowers/plans/2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md) scritto ed eseguito fino al **Task 6 su 14**: i due tempi · la porta `Rng` · i parametri consegnati · la porta `Reactor` · **l'esecutore** · l'orologio virtuale. ⏭️ **Riprende dal Task 7** |
| 3 | giornale e formato durevole — la porta a byte, l'enum di versione, **i byte congelati** | ⬜ |
| 4 | il simulatore DST — tempo virtuale, guasti, campagna, semi | ⬜ |
| 5 | arbitro GPU — ammissione, corsie, concessione, le due policy | ⬜ |
| 6 | gli altri meccanismi — gateway, sensori, permessi, degrado, canale worker | ⬜ |

⛔ **Perché la porta di qualità viene prima della logica, e non è pedanteria:** un cancello
costruito **dopo** è un cancello che nessuno ha mai visto fallire, e la §7.1.1 dice che
allora non è un cancello. Su uno scheletro vuoto ogni controllo si prova in **due**
direzioni al costo di poche righe; dopo, la seconda direzione diventa cara e si smette.

📌 **Due scelte prese dal piano**, perché la spec non le fissa e allora costavano zero:
**edition `2024`** su tutte e cinque le crate, e un **`rust-toolchain.toml`** che dichiara
versione e **bersaglio del cancello**, così che il vincolo 4 della §11 si soddisfi da solo
su una macchina pulita. ✅ **Entrambe sono nel repository**, e stanno in §4.

✅ **L'ultima decisione aperta è chiusa — §7.1.1, il 2026-08-08.** Le otto righe del catalogo
§7.4 la cui colonna «Difende» non nominava un `V`, un'`I` o un `Q` non erano un problema solo:
**cinque** sono state **ri-attribuite** (`Q8` · `V29` · `Q2` · `I2` · `V29`), e **tre** —
`forbid(unsafe_code)`, il grafo **di build**, i **test di contratto** — non difendono una
proprietà del sistema ma **il verdetto di altri controlli**. Per queste la regola 1 ha ora un
**ramo 1b**, e `check-docs.sh` una **sesta asserzione** che la verifica. Dettagli in §7.1.1.

### §8 — ✅ chiusa. Cosa ha trovato, in quattro righe

| | |
|---|---|
| **le sette voci non hanno cambiato nessun voto** | hanno cambiato **cosa una cella nomina**. Riallineate `Q4`, `Q5`, `Q14` e la riga `process` di §8.2.2; `V25`, `Q20` e `V29` **rilette e lasciate come stavano** |
| ⛔ **il catalogo aveva saltato cinque controlli** | i cinque della §6.10.5 (F1b) non erano in §7.4, quindi `Q4` non poteva nominarli senza violare §8.1.2. Sono entrati — due gettoni in §7.4.1 B, due voci in C, uno di livello 2 in §7.4.2. **Non sono controlli nuovi**: §8.5.4, gotcha **#36** |
| **e non era gotcha #32** | cercato prima di scrivere: ADR-0037 non nomina mai il catalogo, la chiusura di F1b lo colloca in altre sei sezioni, §7.4.4 riduce tre voci che sono altre. **Mai valutato, mai scartato** |
| **due ritratti di conteggi erano stantii** | §8.8 e §7.4.7. Ricontati sulla tabella, non dedotti |

### L'audit sezione-contro-ADR — ✅ 2026-08-08. Quaranta rilievi, e uno ha ribaltato un voto

Undici revisori in sola lettura, uno per sezione più due sulle formulazioni di V e Q,
ciascuno contro gli ADR e le fonti. **Nessuno stato ⛔ e nessuna decisione riaperta**; tutte
le correzioni portano il proprio richiamo datato.

| | |
|---|---|
| ⛔ **`V16` torna a ⚠️ `parziale`** | la colonna «Vincolo» di §8.3 lo riportava **troncato** — mancava *«nomi di provider e parametri **sì**»*, la metà **positiva**, che è verificata qui dallo stesso test a esempi di `V15` e `Q14`. Il declassamento di §8.5.3.1 era corretto sulla metà che aveva davanti. **§8.5.5** |
| **otto formulazioni di vincolo erano troncate** | `V5` `V16` `V25` `V28` `V30` `V31` `V34` `V36`. Sette innocue, una no. È il gotcha **#29** spostato dalle invarianti alla tabella che le giudica: una riga di verifica stretta lascia scoperto un caso, una **riformulazione** stretta cambia l'oggetto del giudizio |
| ⛔ **due firme erano impossibili** | `Worker::istruisci → Ricevuta` restituiva un tipo che non esiste altrove, mentre le letture prendono `RicevutaSingola` e `RicevutaFlusso`: o era l'enum che la decisione vieta, o **non c'era modo di ottenere un flusso** — cioè l'audio. Ora le istruzioni sono **due** (§6.10.2) |
| **`daemon` non monta il simulatore** | la tabella §1.2 diceva *«sceglie `platform` o `simulator`»*, il grafo accanto non aveva quell'arco, e ADR-0034 dà ragione al grafo: in simulazione il cablaggio lo fa **il banco**. Era l'unica ambiguità strutturale sul verso delle dipendenze, ed è quella che il piano deve tradurre in `Cargo.toml` |
| **`network` era rimasto stretto in §3.1** | F5 aveva allargato la cella in §2.3 e non la gemella nella tabella che si dichiara *«esattamente le porte della §2.3»*: l'esportazione OTLP sarebbe nata fuori dall'unico punto di uscita |
| **sei conteggi stantii** | «tre ADR» in §0.1 (sono sette) · «due regole restano test» in §1.6 (tre) · «cinque ADR che nominano un parametro» in §2.8 (otto) · «cinque regole» in §4.9.2 (sei) · «due voci spedite» in §7.4.4 (tre) · «nove voci di livello 2» in §7.7 (undici) |
| **tre ADR hanno ricevuto un rimando** | **0021** — «il seed è un caso di regressione *permanente*» non regge, protegge la **proprietà** · **0032** — «la lista del kernel resta vuota» è falsa, e lo era già alla sua data · **0035** — «nessun gettone nuovo da inventare», F1b ne ha portati due |
| ✅ **la voce che restava aperta è chiusa** | **otto righe del catalogo** avevano una colonna «Difende» che non nominava un V, un'I o un Q. Chiusa il 2026-08-08: cinque ri-attribuite, tre nel **ramo 1b** della regola 1 — sostengono il **verdetto** di altre righe, non una proprietà. La regola non è più un'intenzione: sesta asserzione di `check-docs.sh`, provata su 8 rosse e 25 verdi alla prima corsa. §7.1.1 |

📌 **Il ritratto pieno, per il confronto della prossima volta:** **diciotto ✅ · tredici ⚠️ ·
sei ⏳** per i V · **nove · otto · sette** per i Q. ⚠️ «Tredici» era anche il numero di
partenza, ma per una tabella diversa: la storia sta in §8.8, e si riconta **ogni volta**.

### F1b — ✅ chiusa. Cosa ha deciso, in sei righe

| | |
|---|---|
| **la tensione di `design/01` si scioglie con un gettone** | *«il worker non risponde di iniziativa propria»* contro *«il flusso audio risale al core»*: ogni byte che risale è coperto da una **ricevuta**, e le ricevute le emette solo un'istruzione. Un frame che nessuna ricevuta copre è un **guasto**, non un dato. Quarto uso del dispositivo di §6.3.1 |
| **la vita del worker sta in un oggetto solo** | l'avvio restituisce il `Worker`, ed è l'unico modo di parlargli; `uccidi` lo **consuma**. Istruire dopo l'uccisione non compila. I2 resta al compilatore |
| **due tipi di ricevuta, non un enum** | singola e di flusso: «una risposta singola diventa un flusso» non è **esprimibile**. Costa una funzione di lettura in più |
| **il formato è `minicbor`, e la porta scambia byte** | perché il pari **Python non sa leggere `bincode`** — ADR-0037, M-10. Voce già spedita: la lista di ADR-0031 **non cresce**. Col byte sulla porta il simulatore esercita davvero la codifica |
| **due regole uscite dalla misura** | il frame **dichiara la propria lunghezza** e la decodifica verifica i byte consumati (gotcha #34) · ogni `Vec<u8>` porta l'**annotazione di stringa di byte**, o il flusso audio raddoppia (gotcha #35) |
| ⛔ **e una divergenza registrata** | l'istruzione diceva di allargare le giustificazioni di `bincode`. Con la misura **non si allargano**: `bincode` serve il solo canale gui, e ad allargarsi sono le righe di `minicbor`. Gotcha #15 |

### F4 — ✅ chiusa. E la classe attesa era sbagliata a metà

La classe attesa era **C**. Scritta invece che assunta, si è spaccata in **due**:

| Pezzo | Regola |
|---|---|
| il **registro dei trigger**, e l'apertura di una run da un evento | **C** — nessun consumatore finché non esiste una capacità che parta da un evento, e la DST prova Q2, Q4 e Q5 aprendo le run direttamente |
| che ogni **sorgente di eventi** entri da una **porta dichiarata**, e che si dica quale | **B** — §3.1 dichiara le porte esaustive e il simulatore le sostituisce tutte: una sorgente scoperta dopo è **una porta aggiunta dopo la campagna**, e nulla diventerebbe rosso |

**Le due righe della tracciabilità che vi pendevano hanno una porta:** *Scheduling* e
*File watching* entrano entrambe da **`reactor`** — la prima è già coperta dal tempo
virtuale (§3.2), la seconda è **dichiarata** con implementazione scaglionata, la stessa
postura di `network`. Sta su `reactor` e non su `filesystem` perché ciò che deve essere
deterministico è **quando arriva la notifica**, non quale percorso.

✅ **Le famiglie di porte restano sei**, ed è la ragione per cui F4 è costata una
sotto-sezione invece di una riscrittura. 📌 **Per la §8 non produce nessuna riga nuova:**
**V29** copre già le sorgenti di eventi, e lo dice la sua stessa riga di verifica — *«C1
fallisce a ogni sorgente nascosta»*. Lo stato non cambia.

### ✅ Dove nasce il workspace — deciso dal piano, ed **eseguito**

Era l'unica domanda strutturale che la spec aveva deliberatamente lasciato al piano.

| | |
|---|---|
| **il workspace nasce alla radice**, da zero | `Cargo.toml` con `crates/{kernel,platform,secrets,simulator,daemon}`. Niente si eredita |
| ⛔ **`spikes/` è fra gli `exclude`** | `spikes/rust/` è a sua volta un **workspace annidato** e porta un `clippy.toml` che a livello di workspace scatterebbe addosso a `platform`, che *deve* chiamare l'orologio — vincolo 5 |
| **«punto di partenza» significa che si copia** | la §2.5 dice riga per riga cosa entra in `crates/kernel/` e cosa **resta** negli spike. Nel Traguardo 1 **non è ancora salito niente**: sale col substrato, Traguardo 2 |

---

## 7. Non rilitigabile

Rimettere in discussione un ADR `Accepted` **richiede un ADR nuovo che lo superi**
(`Superseded by`), non una conversazione. Le più tentate, e il costo di riaprirle:

| Decisione | Se la riapri |
|---|---|
| I quattro pilastri sono **paritari** (0001) | il kernel diventa il servo di un pilastro, gli altri tre cittadini di seconda classe per sempre |
| **Tre** classi di processo (0004) | la quarta si giustifica contro la tabella, o non si fa |
| **Nessun codice di terzi in-process** (0003) | rientrano contratto pubblico da congelare e superficie d'attacco |
| Default **OpenRouter, VRAM libera** (0006) | lo swap coordinato passa da eccezione a caso normale, e cambia tutta la UX di attesa |
| **Fail-closed** sui vincoli dei dati (0012) | la protezione torna a essere una preferenza |
| Il **contesto è una proiezione** (0008) | le run lunghe tornano a perdere informazione in modo irreversibile |
| **Nessun modello** nel percorso decisionale (0020) | un fallimento del kernel smette di essere sempre un difetto, e la DST diventa impossibile |
| Il motore è **`redb` con backend nostro** (0032) | si rinuncia a metà della verifica: è il punto in cui il **livello 2** di crash diventa iniettabile |
| L'**esecutore vive nel kernel** (§2.4) | un runtime di ecosistema gli restituisce l'ordine delle attività — il controllo che lo spareggio #1 aveva comprato |
| La **concessione di presentazione la tiene il core** (0033) | la scorciatoia «esentiamo la GUI» rende **I2 falso**; darla alla GUI crea una concessione che si perde ogni volta che la GUI muore |
| Il controllo delle dipendenze misura **due grafi con rimedi opposti** (§7.3.1) | unificarli insegna il riflesso «aggiungi alla lista» anche per una violazione di I3, dove aggiungere **è** la violazione |
| Il **cancello senza OS si aggiunge**, non sostituisce la lista (§7.3.2) | la lista coglie una crate **nuova**, il cancello una **già ammessa** che raggiunge l'OS per una via non prevista. E solo la lista dice il nome del colpevole |
| Il **livello 3 del catalogo è vuoto** (§7.4.3) | un rosso della porta deve significare sempre «invariante violata», mai «stile discutibile», o si impara a ignorarlo |
| Il **ramo 1b nomina le voci del catalogo** di cui sostiene la validità (§7.1.1) | allargarlo a «una proprietà decisa in una sezione nominata» — la scorciatoia che sembra la stessa cosa — rende la regola 1 **incapace di rifiutare**, e da lì `clippy` rientra dalla porta. La differenza fra le due formulazioni è tutta qui |
| L'**innesco è obbligatorio** per `parziale` e `rimandato` (§8.1) | `parziale` diventa la casella comoda in cui parcheggiare tutto |
| La **guardia di non-vacuità** dei controlli (§8.6.2) | senza, basta rinumerare una sotto-sezione perché due controlli smettano di controllare **uscendo verdi** |
| I **parametri sono consegnati**, non letti (0034) | una costante nel kernel non fa scattare nessun controllo, e toglie alla DST l'unico modo di esplorare RK-1 |
| Il **dialogo con un worker vive in `process`** (0035) | metterlo su `ipc` spezza la vita del worker fra due porte e perde il meccanismo che ha portato I2 dal test al **compilatore** — senza che nulla diventi rosso |
| Il **formato del giornale è deciso a sé** (0036) | usare `bincode` anche lì importa in un artefatto che **deve** evolvere una decisione presa dove l'evoluzione era stata **rinunciata** |
| Il formato di un canale privato si sceglie **anche sul pari** (0037) | si torna a scegliere guardando **un capo solo** del filo, e che il pari sappia leggere si scopre quando il codice esiste già. E l'asimmetria fra i due canali torna a sembrare una svista da «sanare» |

---

## 8. Cosa NON rifare

| | |
|---|---|
| ❌ **ri-derivare l'architettura** | è nei 37 ADR, ciascuno con alternative scartate e motivo |
| ❌ **riscrivere `tracciabilita.md` da zero** | centosettanta funzionalità già mappate: si **aggiorna**, e **solo alla chiusura del sotto-progetto 1** — quindi non ora |
| ❌ **ri-cercare lo stato dell'arte già tracciato** | è in `riferimenti.md` con le fonti. Verificane semmai l'invecchiamento |
| ❌ **rifare gli spike SP-5 e SP-6** | esiti, seed, versioni e comandi in `spikes/RISULTATI.md` |
| ❌ **rifare le misure da M-1 a M-11** | tutte chiuse, con comandi, versioni e sonde. M-9 sta per intero in ADR-0036, **M-10 e M-11 in ADR-0037**. L'unica aperta è **M5** (senza trattino), e richiede una GUI |
| ❌ **riaprire le due decisioni della §7.3** | prese dopo aver misurato. Riaprirle richiede una misura nuova, non un'opinione |
| ❌ **riaprire la copertura della §8** | |
| ❌ **riaprire F3, F6, F5, F1a, F2, F7** | chiuse, con i limiti dichiarati |
| ⛔ **riaprire §6.1.1** «tanto ora c'è `minicbor` nel kernel» | **tentato il 2026-08-08, e la misura ha dato torto**: i due canali privati non sono lo stesso problema, perché i loro **pari** sono diversi (ADR-0037, M-11). Riaprirla richiede una misura nuova sul pari, non un argomento di simmetria |
| ⛔ **rigenerare i byte congelati del giornale** | se cambiano non è un aggiornamento, è un cambio di formato: si apre una versione nuova |
| ❌ **progettare una capacità L2** | prima il kernel deve esistere |
| ❌ **promuovere l'aiutante `passo_in_dubbio` dello spike** | assume esecuzione sequenziale: con l'interlacciamento dà un **falso negativo** (gotcha #20) |
| ❌ **far salire `spikes/rust/clippy.toml`** nel workspace reale | a livello di workspace scatterebbe addosso a `platform` (§7.4.4) |
| ⚠️ **fidarsi delle fonti senza data** | l'ecosistema si muove a cadenza mensile |

---

## 9. I quarantatré gotcha

Trappole **reali**, molte trovate correggendo errori già commessi in questo progetto.
Il testo completo, con le misure, è in `HANDOFF.md`.

| # | Trappola |
|---|---|
| 1 | **«Tutto è una run» vale solo per l'inferenza _generativa_.** Applicarlo a wake word, VAD e trascrizione continua viola Q1 e riempie il giornale di rumore |
| 2 | **Ritentativo o passo nuovo?** Discriminante: *il modello ha prodotto output?* No → stesso passo. Sì ma respinto da un sensore → passo nuovo: quell'output esiste ed è stato pagato |
| 3 | **Policy VRAM ≠ destinazione della richiesta.** In policy locale una singola richiesta può finire su un provider remoto senza che la policy cambi |
| 4 | **La quota audio sottratta non esenta da I2.** Il worker audio ha una concessione permanente, non l'assenza di concessione |
| 5 | **I permessi applicativi non sono un confine contro codice eseguito.** Un processo figlio non passa dal mediatore. Serve il livello 2 |
| 6 | **«Cifrato a riposo» qui vale quanto l'account OS.** Va detto *in interfaccia*, non solo nell'ADR |
| 7 | **Il giornale è la sorgente; trace, contesto, costi e metriche sono proiezioni.** Non costruire un secondo sistema di osservabilità |
| 8 | **Ogni requisito Q deve avere un metodo di verifica** (V30). La §10 ha violato la regola appena scritta; `check-docs.sh` ora lo rileva |
| 9 | **Go non ha test di compilazione fallita di serie.** Un driver che compila fuori dal modulo fallisce per il motivo sbagliato |
| 10 | **`xorshift` resta bloccato su zero.** Senza guardia sullo stato iniziale certi seed producono traccia vuota, e lo spike *sembra* passare |
| 11 | **Il contesto degrada _prima_ che la finestra si riempia** (context rot). Si tiene un budget target, non una soglia |
| 12 | ⛔ **`std::collections::HashMap` viola V29.** `RandomState` è seminato **per processo**: l'ordine di iterazione non è riproducibile. Non compare in nessun elenco di «chiamate OS» e si manifesta come traccia divergente inspiegabile. Usare `BTreeMap`, o un hasher fissato |
| 13 | **Un lint non è il compilatore.** `clippy` ferma la violazione ma `cargo build` no, e un `#[allow]` per riga la annulla. Solo `forbid` e `no_std` producono un divieto non scavalcabile |
| 14 | ⛔ **Un test negativo va provato _in negativo_.** Il piano degli spike aveva **due sonde di non-vacuità sbagliate su tre**. Un controllo che non si è visto fallire **non è un controllo** |
| 15 | ⛔ **Un'evidenza scritta prima della misura è un'ipotesi, non un risultato.** Tre affermazioni dettate dal piano sono risultate **false** alla misura. Si misura, e dove diverge **si registra la divergenza** |
| 16 | **`no_std` impedisce di _nominare_ `std`, non di _raggiungere_ l'OS.** Misurato: una crate `no_std` + `forbid(unsafe_code)` legge un file e l'orologio attraverso una dipendenza, e **compila ed esegue**. La lista delle dipendenze è l'altra metà del confine |
| 17 | **Iniettare un guasto dove il codice non arriva è una prova _vacua_ che sembra un successo.** Si conta **prima** quante operazioni compie davvero il codice, poi si verifica che il guasto sia *scattato* |
| 18 | **Misurare il transitorio invece del regime dà la risposta opposta.** Per una proprietà di stato stazionario, un solo giro non è una misura |
| 19 | **Un avanzamento nullo dichiarato riuscito è un ciclo infinito.** Filtrare alle scadenze **strettamente future**, e mettere una guardia sui giri dell'esecutore |
| 20 | **Un crash lascia _più_ passi in dubbio, non uno.** Misurato: seme 99 → passi `[3, 7]`. L'aiutante dello spike assume sequenzialità e dà un falso negativo |
| 21 | **Il rifiuto dell'arbitro è esecutivo solo verso ciò che avviamo noi.** Il compositor della webview compone lo stesso. Verso di lui una quota è una **promessa di budget**, non un'imposizione. Corollario: una quota sottratta **senza titolare della concessione** non salva I2 affatto |
| 22 | **Che una versione esista non vuol dire che funzioni.** `cargo add bincode` risolve alla `3.0.0`, il cui intero sorgente è un `compile_error!`. La versione utile è la `2.0.1`, **appuntata a `2`** con la ragione scritta accanto |
| 23 | **`cargo metadata` non risolve le feature; `cargo tree` sì.** Misurato: undici crate segnalate contro le due reali. Un allow-list costruito sull'interfaccia macchina «giusta» sovra-segnala di cinque volte |
| 24 | ⛔ **Un controllo si prova in _due_ direzioni.** #14 copre metà: mai visto fallire = non è un controllo. L'altra metà è che **uno che scatta dove non deve è peggio di uno assente**, perché insegna a ignorare l'audit |
| 25 | ⛔ **Rigenerare in blocco l'oracolo di un test negativo lo rende una tautologia.** Vale per gli `.stderr` di `trybuild` **e per i byte congelati del giornale**. Ogni volta che l'oracolo è un file generato dal test stesso, aggiornarlo automaticamente **cancella l'oracolo**. 📌 **Seconda occorrenza:** l'oracolo di `trybuild` è accoppiato al **grafo linkato**, non al solo sorgente del caso — se `kernel` linkasse `std` sparirebbe una riga dall'output e **due oracoli diventerebbero rossi insieme**, per un motivo estraneo alla regola sotto test |
| 26 | ⛔ **Un controllo delimitato per intestazione si spegne quando qualcuno rinumera — e si spegne _verde_.** Rimedio: se un delimitatore non si trova, o l'intervallo è vuoto, **è un fallimento**. ⚠️ Il rimedio **sbagliato** è mettere a guardia un numero atteso di righe. 📌 **Seconda occorrenza:** in `trybuild` un **glob** che non pesca nulla **non è un errore** e il banco usciva verde, mentre un percorso **letterale** inesistente diventa rosso. L'asimmetria è la parte da ricordare |
| 27 | **La legenda di una tabella risponde a una domanda sola, e chi legge ne assume un'altra.** È così che la spec è stata riaperta su sette voci. Il rimedio non è riscrivere la legenda: è **rileggere con un'altra domanda** |
| 28 | **Un parametro non consegnato è una costante, e una costante è invisibile.** Non compare in nessun elenco, non fa scattare nessun controllo, e si manifesta solo come uno scenario che la campagna **non può esplorare** |
| 29 | **La riga di _verifica_ di un'invariante è il punto in cui l'invariante si restringe in silenzio.** Già successo con I2 e con I4. Completare una riga di verifica **non è superare l'ADR** |
| 30 | ⛔ **Un banco che guarda solo `Ok`/`Err` non vede la _risposta sbagliata_.** Misurato in M-9: cinque celle su trentasei restituiscono `Ok` con **valori sbagliati**. Confrontare i **valori**, non l'esito. In un archivio durevole il modo di fallire peggiore non è l'errore — è il record che ti restituisce il numero sbagliato |
| 31 | **Una stima di costo prezzata sulla variante sbagliata sopravvive, perché viene citata invece che rifatta.** Misurato: un byte su ventisei, contro il «permanente su ogni campo» che stava per far scartare la forma giusta. Una stima che sta per **decidere** va rimisurata |
| 32 | ⛔ **Un'idea che sembra nuova può essere già stata scartata, e il compendio non lo dice.** Comprime le **decisioni**, non le alternative respinte: una proposta ragionevole può essere già caduta — con la misura — dentro un ADR o una sezione. Prima di proporre una **sostituzione** si cerca dove era già stata valutata e perché. Successo il 2026-08-08 con `minicbor` su `ipc`: a smontare la proposta è stata **la stessa misura che l'aveva motivata** |
| 33 | **Il nome del formato è occupato da un'altra cosa, e in due ecosistemi.** Su PyPI `bincode` installa un modulo `b64tools`, funzioni base64; su npm `bincode` è una CLI di sviluppo con l'IA. È il gotcha #22 nella forma più larga: **che un nome esista non dice cosa contiene**, e cercare per nome trova pacchetti che non c'entrano |
| 34 | ⛔ **Un decodificatore CBOR si ferma al primo elemento completo e ignora la coda.** Misurato: dando a `cbor2` i byte di `bincode` restituisce `1` — nessuna eccezione, un valore plausibile. Su un canale a frame «ha decodificato» non prova nulla: serve che i **byte consumati** siano pari alla lunghezza dichiarata |
| 35 | **Un `Vec<u8>` non annotato raddoppia il traffico, in silenzio.** In `minicbor`, senza l'annotazione di stringa di byte, si codifica come **array di numeri**. Misurato su 4096 B: **7813** contro **4101**, cioè **1,91×**. Compila, fa round-trip, ed è corretto: costa solo il doppio |
| 36 | ⛔ **La tabella «come si verifica» di una sezione non è il catalogo, e il passaggio si salta.** Una sezione che decide un meccanismo scrive le proprie sonde lì — ed è giusto — ma il **catalogo §7.4** è l'unico posto che §8.1.2 ammette, ed è la lista che il piano tradurrà in lavoro. Successo **due volte**: V2/V4/V10 (§8.5.3) e i cinque controlli della §6.10.5 (§8.5.4). ⚠️ La diagnosi è l'**asimmetria**: nella stessa riapertura §2.8.4 e §4.9.4 le righe le hanno aggiunte, §6.10.5 no, e **nessun documento dice perché** — quindi non era una decisione. ⛔ Il rimedio **non** è irrigidire lo script (§8.6.4): è cercare questa classe di difetto a ogni sezione che decide un controllo. È il #29 spostato dalle invarianti ai controlli |
| 37 | ⛔ **Un controllo può difendere _un altro controllo_ invece di una proprietà, e la regola d'ammissione lo scambia per un'abitudine — cioè lo fa togliere.** Misurato: **otto righe su trentatré** del catalogo, fra cui `forbid(unsafe_code)`. Cancellalo e **nessuna** riga diventa rossa: diventano tutte **meno vere**, perché un `unsafe` falsifica un gettone o transmuta un newtype. ⛔ Il rimedio **non** è allargare la regola a «una proprietà decisa in una sezione nominata» — così non rifiuterebbe più niente, e una regola che non rifiuta mai è decorazione. È darle un **secondo ramo** con un criterio proprio: *nomina le voci del catalogo di cui sostieni la validità*. Quello rifiuta ancora `clippy` |
| 38 | **Un controllo che interroga uno stato può _modificarlo_ mentre lo interroga, e allora la sua condizione non può essere vera.** `gate-no-os.sh` chiede a `rustup` quali bersagli siano installati. Misurato: quel comando **riconcilia `rust-toolchain.toml` prima di rispondere**, quindi se il bersaglio manca **l'atto di chiederlo lo installa**. Con la rete la guardia **non può scattare**; scatta **senza rete**, verificato con uscita 1 e messaggio corretto. È **la via offline, non una rete di sicurezza**, e la differenza va scritta accanto alla guardia |
| 39 | ⛔ **Un test negativo che _ridichiara le proprie precondizioni_ prova il meccanismo, non che il sistema sia configurato così.** I quattro casi di `compile_fail/` ridichiarano ciascuno `#![no_std]` e `#![forbid(unsafe_code)]` e non nominano mai `kernel::`: provano che il divieto **morde dove è dichiarato**, non che il kernel lo dichiari. Misurato: tolto `#![forbid(unsafe_code)]` da `crates/kernel/src/lib.rs` **e scritto un `unsafe` vero**, la porta restava **verde su cinque controlli su cinque**. Rimedio: `scripts/gate-attributes.sh`, livello 2. ⚠️ Peggiore di come sembra: quella riga è di **ramo 1b** e sostiene i blocchi A, B e C — toglierla non spegneva una regola, invalidava **il fondamento del livello 1** |
| 40 | ⛔ **Questo file dichiara di contenere tutte le decisioni, ma il controllo ne pretende una voce solo per gli _ADR_: una decisione che vive in una _sezione di spec_ può mancare, e allora per chi legge non esiste.** Misurato il 2026-08-08: la **§1.0** — codice in inglese, documentazione in italiano — non era né qui né in `CLAUDE.md`. Un agente ha letto per intero **entrambi** i file obbligatori e ha scritto un traguardo intero con gli identificatori in italiano, poi corretto con sei rinomini di file, undici di funzione e la rigenerazione dei quattro oracoli. ⚠️ Il controllo di §13 accoppia le voci ai **file** in `docs/adr/`: esatto per ciò che misura, cieco a tutto il resto. ⛔ Il rimedio non è irrigidire lo script — nessun elenco di «sezioni che contano» resterebbe vero — è che **una decisione scritta fuori da un ADR va portata a mano nel compendio**, e chi la scrive è l'unico che può saperlo |
| 41 | **Un filtro che _normalizza l'ingresso_ di un controllo decide anche che cosa il controllo può vedere.** `gate-deps.sh` estraeva i nomi di crate con una classe di caratteri **minuscola**. Misurato: una crate col nome maiuscolo veniva scartata dal filtro, quindi non compariva fra gli intrusi e il cancello usciva **verde** — un falso negativo su I3, il modo di fallire peggiore per quel controllo. Provato con `Inflector`, crate reale: uscita 0 prima, uscita 1 e nome del colpevole dopo. ⚠️ Col corteo di dipendenze minuscole che si porta dietro, il controllo segnalava **il corteo e non il capofila**, pur stampandolo dentro ogni catena |
| 42 | ⛔ **Un test di compilazione fallita che scatta come `mismatch` è disarmato da una rigenerazione in blocco; uno che scatta come `error` no — e `trybuild` stampa la parola.** Misurato: col confine fra `Monotonic` e `WallTime` guardato dal solo caso «passa l'uno dove va l'altro», aggiungere `impl From<WallTime> for Monotonic` **non** lo fa smettere di fallire — resta `E0308`, perché Rust non applica `From` al sito di chiamata. Lo rende rosso il fatto che rustc **aggiunga quattro righe di `help: call Into::into`** che l'oracolo non porta: un **`mismatch`**. Quindi quel guardiano **poggia interamente sul fatto che l'oracolo non venga rigenerato** — è il #25 con la conseguenza che nessuno aveva scritto: **l'oracolo registra anche l'assenza di una via di conversione**. Rimedio: un **secondo caso di forma diversa** — `let _x: Monotonic = wall.into();` — che con l'`impl From` presente **compila**, e scatta come `error`. ⚠️ **Il test generale, che costa zero:** una regola guardata solo da casi `mismatch` è una regola che una rigenerazione spegne in silenzio. 📌 E le regole erano **due** — «non si passa l'uno per l'altro» e «non esiste una via di conversione» — con la seconda scritta **in un commento**, cioè un'intenzione |
| 43 | ⛔ **In un modello, un valore d'esempio _valido_ viene incollato così com'è: non si distingue da un dato. E un avviso accanto non è un rimedio.** Il campo «Ultimo commit» di [`AVVIO-CHAT.md`](AVVIO-CHAT.md) portava un hash vero, con accanto una riga che **dichiarava il difetto** e ne scaricava il rimedio su chi incolla. Misurato: il file era vecchio di **due** commit, il messaggio incollato di **quattro**, e due commit erano serviti solo a rincorrerlo — non possono raggiungerlo, perché lo contengono. ⚠️ **Togliere il campo era la proposta sbagliata:** `HANDOFF.md` non porta lo SHA **perché delega a lì**, e la prova ha giocato contro — il **#32 applicato a sé**, con un rimedio migliore in uscita. Il difetto è più stretto: lo SHA appartiene all'**istanza**, il **modello** ci metteva un valore concreto. Rimedio: un **segnaposto** che nomina il comando che lo riempie — si fallisce **rosso** invece che verde, come nel **#26** |

---

## 10. Le quattro trappole di `check-docs.sh`

Da sapere **prima** di scrivere, non dopo il rosso.

| # | Trappola |
|---|---|
| **1** | **I conteggi.** Ogni occorrenza di `<cifra> ADR`, `<cifra> ADR in stato ...` e `<cifra> decisioni architetturali` nei documenti di stato è confrontata con la realtà. Scrivere `2 ADR nuovi` la fa scattare, perché legge il `2` come **totale**. ⚠️ **Per i numeri piccoli si usano le parole**; gli esempi vanno nei code span — e **il code span non deve andare a capo**, perché lo spogliamento è riga per riga. Punti ciechi dichiarati: un numero **a parole** è invisibile, e così `<cifra> decisioni` **senza** «architetturali» |
| **2** | **La numerazione.** Il controllo sui duplicati è **per file** e cattura `^#{2,3} <numero>`, quindi `### 7.4.1` sarebbe letto come duplicato di `### 7.4`. **Le sotto-sotto-sezioni si scrivono con `####`** |
| **3** | **Due tabelle sono lette _per posizione_.** Nel **catalogo §7.4** la contro-sonda è l'**ultima** colonna e non può essere vuota. In **§8.3 e §8.4** le colonne sono **cinque**, con lo stato in **terza** e l'innesco in **quinta**. ⛔ E i **delimitatori sono intestazioni** (`#### 7.4.1`, `#### 7.4.3`, `## 8.`): rinumerarle è un **rosso**, non un ritocco. ⚠️ **La sesta asserzione fa eccezione, e deliberatamente:** la colonna «Difende» del catalogo **non è sempre la prima** — nei blocchi A e C e in §7.4.2 lo è, nel **blocco B dei gettoni è la terza** — quindi si cerca per **intestazione**. Non «uniformarla» alle altre: un controllo posizionale giudicherebbe la colonna sbagliata su cinque righe |
| **4** | **Un falso positivo in attesa.** La guardia dei conteggi gira su una lista fissa di documenti di stato. In `tracciabilita.md` esistono righe come `§4 ADR-0008`, dove il regex leggerebbe `4 ADR`. **Oggi non scatta**, perché quel file non è nella lista. Se servisse aggiungerlo, il rimedio è il **regex**, non il documento |

---

## 11. I quindici vincoli sul primo commit di codice

Non sono decisioni da prendere: sono decisioni **prese**, che ogni piano deve tradurre in
passi.

✅ **I primi cinque sono onorati dal Traguardo 1** — cinque crate · `no_std` + `alloc` +
`forbid` su `kernel` e `simulator` · `bincode` appuntato a `2` con la ragione accanto ·
il bersaglio del cancello dichiarato in `rust-toolchain.toml` · `spikes/` fra gli
`exclude`. ⚠️ Il quarto ha una sottigliezza misurata: gotcha **#38**. Gli altri dieci
restano davanti, e chi li copre è scritto in [`porta-di-qualita.md`](porta-di-qualita.md).

| # | Vincolo | Da |
|---|---|---|
| 1 | **cinque crate**: `kernel` · `platform` · `secrets` · `simulator` · `daemon`. `kernel` non dipende da nessuna crate del progetto | §1.2 |
| 2 | `kernel` e `simulator`: `#![no_std]` + `alloc` + `#![forbid(unsafe_code)]`. ⚠️ **`forbid`, non `deny`** — `deny` è scavalcabile da un `#[allow]` locale | §1.4 · ADR-0026 |
| 3 | il manifesto **appunta `bincode` a `2`**, con la ragione scritta accanto | §6.1.1 · gotcha #22 |
| 4 | `rustup target add x86_64-unknown-none` è un **prerequisito dell'ambiente**, o la porta è rossa per il motivo sbagliato | §7.3.2 |
| 5 | il `clippy.toml` di `spikes/rust/` **non sale** | §7.4.4 |
| 6 | l'aiutante `passo_in_dubbio` dello spike **non sale così com'è** | §4.3 · gotcha #20 |
| 7 | il numero di semi della campagna breve è **fissato e versionato**, e il tempo di parete si stampa a ogni corsa | §7.5.3 |
| 8 | cadenza: livello 1 **a ogni compilazione** (non «gira»: *è* il compilatore), livello 2 a ogni commit, DST profonda su ciclo lungo | §7.5.1 |
| 9 | riga per riga, **cosa sale da `spikes/rust/` e cosa resta** | §2.5 |
| 10 | ogni regola nuova porta **due** sonde e un caso in `tests/compile_fail/` con il suo `.stderr` — da **leggere**, non da rigenerare in blocco | §7.1.4 · gotcha #25 |
| 11 | **nessuna decisione legge un parametro che non le è stato consegnato**. In sotto-progetto 1 i default sono **letterali in `daemon`** | §2.8 · ADR-0034 |
| 12 | il record durevole è un **enum di versione**, ogni campo ha un **indice esplicito**, un campo nuovo è facoltativo con indice nuovo, un indice **si ritira e non si riusa mai** | §4.9 · ADR-0036 |
| 13 | la porta `journal` scambia **byte**, non record tipizzati: la codifica vive in `kernel` | §4.1 · §4.9.3 · §7.3.1 |
| 14 | ⛔ al **primo record scritto**, i suoi byte entrano nel repository come **oracolo**, con la mappa `indice → nome → valore atteso`. **Non si rigenerano** | §4.9.4 · gotcha #25 |
| 15 | il **canale worker** usa `minicbor`, la porta `process` scambia **byte**, ogni frame **dichiara la propria lunghezza** e la decodifica verifica i byte consumati, e ogni `Vec<u8>` porta l'**annotazione di stringa di byte** | §6.10 · ADR-0037 · gotcha #34, #35 |

---

## 12. Dove guardare, quando il compendio non basta

Apri **un** file, quello che serve. Non la cartella.

| Se ti serve… | Apri | Peso |
|---|---|---|
| il **perché** di una decisione, le alternative scartate, i costi accettati | `docs/adr/<numero>-*.md` — **uno solo** | 2–19 KB l'uno |
| il **come** del sotto-progetto 1: §0–§8 con le evidenze delle misure | [`specs/2026-08-06-sottoprogetto-1-kernel.md`](superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md) — ⚠️ **a sezioni, mai intera** | 267 KB |
| il **cosa** del kernel: §0–§10 | [`specs/2026-08-06-kernel-design.md`](superpowers/specs/2026-08-06-kernel-design.md) | 44 KB |
| il testo integrale dei **gotcha** e delle **misure**, con i numeri | [`HANDOFF.md`](HANDOFF.md) — ⚠️ **a sezioni** | 113 KB |
| ⛔ **cosa una sezione deve incassare, prima di proporle una modifica** | [`HANDOFF.md`](HANDOFF.md) — il **consuntivo voce per voce**: cosa era stato deciso, dove è finito, e cosa resta da scrivere. È **autorevole**, e si legge **prima** di proporre, non dopo | ⚠️ **la sezione, non il file** |
| l'ordine dei dodici sotto-progetti e le dipendenze | [`roadmap.md`](roadmap.md) | 15 KB |
| dove vive una funzionalità della mappa originale | [`tracciabilita.md`](tracciabilita.md) — ⚠️ **leggi il riquadro in testa**: risponde a «dove vive», **non** a «di quale meccanismo ha bisogno». È la crepa da cui sono uscite le sette voci | 15 KB |
| **dove vive ogni controllo** della porta, riga per riga sul catalogo §7.4, e cosa **non** è coperto | [`porta-di-qualita.md`](porta-di-qualita.md) | 15 KB |
| la **strategia di test** — è la fonte di verità sulla porta di qualità, e mappa Q1–Q24 → metodo | [`design/08-strategia-di-test.md`](design/08-strategia-di-test.md) | 8 KB |
| la **topologia dei processi** — contiene la tensione che F1b deve conciliare | [`design/01-topologia-dei-processi.md`](design/01-topologia-dei-processi.md) | 4 KB |
| gli altri diagrammi della struttura | [`design/`](design/) — nove file | 4–9 KB l'uno |
| gli **esiti degli spike**, con seed, versioni e comandi | [`../spikes/RISULTATI.md`](../spikes/RISULTATI.md) | |
| i requisiti della GUI, G1–G21 e P1–P4 | [`../spikes/GUI-REQUISITI.md`](../spikes/GUI-REQUISITI.md) | |
| la **provenienza** di ciò che non abbiamo dedotto noi, con le date | [`riferimenti.md`](riferimenti.md) | 32 KB |
| il **modello** di come si scrive un piano qui, con l'errata in testa | [`plans/2026-08-06-spike-linguaggio-del-core.md`](superpowers/plans/2026-08-06-spike-linguaggio-del-core.md) | 68 KB |
| ⛔ **cosa il piano del Traguardo 1 detta e il repository smentisce** — quattro voci, prima fra tutte gli identificatori italiani | [`plans/2026-08-08-sottoprogetto-1-traguardo-1-scheletro-e-porta.md`](superpowers/plans/2026-08-08-sottoprogetto-1-traguardo-1-scheletro-e-porta.md) — ⚠️ **solo l'errata in testa**, il resto è eseguito | 50 KB |
| ⛔ **il compito da cui si riprende** — è il piano **in corso**, e il Task 7 sta lì | [`plans/2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md`](superpowers/plans/2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md) — ⚠️ **a compiti, mai intero**: è il **secondo file più grande** del repository, dopo la spec | 131 KB |
| l'indice di ADR e diagrammi | [`README.md`](README.md) | 10 KB |

📏 **I pesi servono a decidere se aprire, e si rimisurano quando si toccano i file che
contano.** Prima misura il 2026-08-08: tre erano stantii, e il quarto — *«insieme pesano
oltre settecento kilobyte»*, in testa a questo file, in `CLAUDE.md` e in `AVVIO-CHAT.md` —
era **falso di un quarto**. È il gotcha **#31**: un numero messo a sostegno di una regola
giusta non viene mai rimisurato, perché nessuno dubita della regola.

> 🔁 **Rimisurati lo stesso giorno, chiudendo la §7.1.1 — e il gotcha #31 si è ripetuto sul
> paragrafo scritto per chiuderlo.** Cinque pesi su dodici erano di nuovo fuori, e non tutti
> per la crescita:
>
> | | |
> |---|---|
> | **cresciuti** | spec del sotto-progetto 1 `245 → 253` · HANDOFF `84 → 92` · roadmap `11 → 13`. HANDOFF era **già** stantio prima di questa sessione: due commit l'avevano toccato dopo la misura |
> | ⛔ **sovrastimati, su file mai toccati** | il piano degli spike `72 → 68` · `design/08` `9 → 8` · il minimo degli ADR dichiarato `4 KB` per un file da **2441 byte**. La crescita non spiega un numero che **scende**: la prima misura contava lo spazio **allocato** — che su NTFS arrotonda ogni file al cluster — invece dei byte |
>
> 📌 **Il rimedio, e stavolta è il metodo invece del numero:** i pesi si misurano con
> **`wc -c`, arrotondati a KiB**. Scritto qui, il prossimo che riconta ottiene la stessa
> cifra o scopre una crescita vera — e non un artefatto dello strumento.

> 🔁 **Terza misura, alla chiusura del Traguardo 1 — e il metodo ha retto.** Tutti gli
> scarti sono **crescite vere**, nessuno è un artefatto dello strumento: è la differenza
> fra le prime due misure e questa.
>
> | | |
> |---|---|
> | **cresciuti** | HANDOFF `92 → 104`, per i quattro gotcha nuovi · spec del sotto-progetto 1 `253 → 259` · roadmap `13 → 14` · README `9 → 10` |
> | **voci nuove in tabella** | [`porta-di-qualita.md`](porta-di-qualita.md) **9 KB** e il piano del Traguardo 1 **50 KB**: esistono da oggi |
> | **invariati** | kernel-design 44 · tracciabilità 15 · riferimenti 25 · `design/08` 8 · il piano degli spike 68 · gli ADR 2–19 |
>
> Il totale dell'insieme *«HANDOFF + spec del sotto-progetto 1 + `adr/`»* era **577 KB**.
> ⚠️ Cresce a ogni chiusura: la frase in testa dice «oltre mezzo megabyte» apposta.

> 🔁 **Quarta misura, il 2026-08-09, chiudendo la passata di coerenza.** Quattro scarti,
> tutti **crescite vere**: spec del sotto-progetto 1 `259 → 263` e
> [`porta-di-qualita.md`](porta-di-qualita.md) `9 → 11` per la riga del build script ·
> HANDOFF `104 → 105` e [`riferimenti.md`](riferimenti.md) `25 → 30` per le misure del
> Traguardo 1, che il §13 pretendeva lì e non c'erano. L'insieme passa da **577** a
> **581 KB**, ed è la cifra in testa a questo file e in `CLAUDE.md`. ⚠️ **E i due file
> obbligatori sono passati da 85 a 87 KB**: la cifra vive in `CLAUDE.md` e in
> [`AVVIO-CHAT.md`](AVVIO-CHAT.md), e va rifatta ogni volta che uno dei due cresce.
> 📌 Due misure di seguito senza artefatti dello strumento: il metodo `wc -c` regge.

> 🔁 **Quinta misura, il 2026-08-09, chiudendo la sessione dei Task 1–6 del Traguardo 2.**
> Tutti scarti sono **crescite vere**; il metodo `wc -c` regge da tre misure di seguito.
>
> | | |
> |---|---|
> | **cresciuti** | HANDOFF `105 → 109` · spec del sotto-progetto 1 `263 → 266` (le tre righe nuove del catalogo) · [`porta-di-qualita.md`](porta-di-qualita.md) `11 → 13` · roadmap `14 → 15` |
> | **voce nuova** | il **piano del Traguardo 2**, che è il più grande scritto finora |
> | **invariati** | kernel-design 44 · tracciabilità 15 · riferimenti 30 · `design/08` 8 · README 10 · il piano degli spike 68 |
>
> L'insieme *«HANDOFF + spec del sotto-progetto 1 + `adr/`»* passa da **581** a **589 KB**.
> ⚠️ **E i due file obbligatori sono passati da 87 a 88 KB**: la cifra vive in `CLAUDE.md` e
> in [`AVVIO-CHAT.md`](AVVIO-CHAT.md), e va rifatta ogni volta che uno dei due cresce.

> 🔁 **Sesta misura, il 2026-08-09 — e il gotcha #31 aveva cambiato forma.** Nata di
> rimbalzo: toccando [`porta-di-qualita.md`](porta-di-qualita.md) per registrarvi il rifiuto
> di `rustfmt`, la regola *«i pesi si rimisurano quando si toccano i file che contano»*
> obbligava a rifare quella riga — che diceva **11 KB** mentre la **quinta misura, due
> riquadri più su, aveva già registrato `11 → 13`**.
>
> ⛔ **Non era una riga: erano tutte e quattro.** La quinta misura dichiarava quattro
> crescite, e **nessuna** delle quattro era stata riportata nella tabella qui sopra. Il
> riquadro e la tabella sono **due posti**, si aggiorna il primo, e il secondo continua a
> rispondere a chi deve decidere se aprire un file.
>
> | Riga | Diceva | Misurata ora | |
> |---|---|---|---|
> | spec del sotto-progetto 1 | 263 | **267** | la quinta diceva 266 |
> | [`HANDOFF.md`](HANDOFF.md) | 105 | **111** | la quinta diceva 109 |
> | [`riferimenti.md`](riferimenti.md) | 30 | **32** | mai corretta |
> | [`roadmap.md`](roadmap.md) | 14 | **15** | la quinta diceva 15 |
> | [`porta-di-qualita.md`](porta-di-qualita.md) | 11 | **15** | la quinta diceva 13; cresce oggi per il riquadro su `rustfmt` |
>
> **Le altre sette righe reggono** — kernel-design 44 · tracciabilità 15 · `design/08` 8 ·
> `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · README 10 — e così
> il minimo e il massimo degli ADR, `2–19 KB`, ricontati sui file (2441 B e 19291 B).
>
> ⚠️ **Anche i due aggregati erano fuori, e uno è colpa di questo commit.** L'insieme
> *«HANDOFF + spec + `adr/`»* è **591 KB** (604910 B) contro i 589 dichiarati — cresciuto per
> conto suo. I **due file obbligatori** passano da 88 a **94 KB**: **91 erano già a HEAD**, e
> il resto lo aggiunge questo riquadro. La cifra vive in `CLAUDE.md` e in
> [`AVVIO-CHAT.md`](AVVIO-CHAT.md), ed è aggiornata in entrambi.
>
> ⛔ **Questa cifra descrive il file che la contiene, e si scrive per ultima.** Misurata
> mentre scrivevo ha dato 92, poi 93, poi **95946 B**: ogni riga aggiunta la spostava, e due
> volte l'ho scritta già falsa. Converge solo perché l'ultima correzione è **di sole cifre**,
> che non cambiano la lunghezza della riga. 📌 Chi tocca questo paragrafo **rimisura dopo
> averlo chiuso**, e cambia solo il numero.
>
> 📌 **La forma nuova del #31, ed è quella da ricordare:** il numero non era *mai stato*
> rimisurato — lo era stato, e scritto **in uno solo dei due posti in cui vive**. Rimisurare
> non basta: si scrive **dove qualcuno legge per decidere**, che è la tabella, non il verbale
> della misura.

> 🔁 **Settima misura, il 2026-08-09, chiudendo la voce dello SHA — e la sesta aveva corretto
> i numeri lasciando fuori una riga intera.** Il rimedio scritto nella sesta — *«si scrive dove
> qualcuno legge per decidere, che è la tabella»* — è stato applicato ai **pesi esistenti** e
> non alla **voce mancante**: la quinta misura dichiarava *«voce nuova: il piano del Traguardo
> 2»*, e in tabella quella riga **non è mai entrata**.
>
> | | |
> |---|---|
> | ⛔ **riga aggiunta** | il **piano del Traguardo 2**, **131 KB** — il secondo file più grande del repository, e proprio quello da cui si riprende. Chi doveva decidere se aprirlo non aveva né la voce né il peso |
> | **cresciuto** | [`HANDOFF.md`](HANDOFF.md) `111 → 113`, per il gotcha #43 di questa voce |
> | **invariati, ricontati** | spec del sotto-progetto 1 267 · kernel-design 44 · roadmap 15 · tracciabilità 15 · [`porta-di-qualita.md`](porta-di-qualita.md) 15 · riferimenti 32 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · README 10 · ADR `2–19` |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **591** a **593 KB** (607118 B). I **due file
> obbligatori** passano da 94 a **97 KB**: la cifra vive in `CLAUDE.md` e in
> [`AVVIO-CHAT.md`](AVVIO-CHAT.md), ed è aggiornata in entrambi.
>
> 📌 **La forma successiva del #31, e completa quella della sesta misura:** una tabella si
> aggiorna in **due modi** — correggendo una cella, e **aggiungendo una riga**. Il secondo si
> dimentica, e per una ragione che vale la pena scrivere: rileggendo, **una riga assente non si
> vede**, mentre una cella sbagliata sì. Chi rimisura conta anche le **righe**, non solo i
> numeri dentro di esse.

⚠️ Ed è la ragione per cui la frase in testa dice «oltre mezzo megabyte» invece di una cifra:
**un limite inferiore misurato resta vero mentre i documenti crescono, una cifra esatta no.**

⚠️ **Prima di ogni commit di documentazione:** `bash scripts/check-docs.sh`

---

## 13. Come si aggiorna questo file

⛔ **Questo file non è opzionale e non può restare indietro.** Un compendio stantio è
peggio di nessun compendio: mente con autorevolezza, e l'agente non ha modo di
accorgersene perché **crede** di sapere tutto.

Per questo la sua completezza **non è lasciata alla buona volontà**:

| | |
|---|---|
| **il controllo** | `scripts/check-docs.sh` pretende **una voce in §5 per ogni file in `docs/adr/`**, accoppiata per numero. Un ADR nuovo senza voce → **rosso** |
| **il livello di forza** | **2 — controllo esterno.** Se cancelli lo script, la regola sparisce: sotto non c'è nient'altro. Il livello 1 non è raggiungibile — nessun compilatore legge un `.md` |
| **la guardia di non-vacuità** | se il blocco §5 non si trova, o è vuoto, **è un fallimento** — gotcha #26 |
| ⛔ **ciò che il controllo NON copre** | il controllo accoppia le voci ai **file in `docs/adr/`**: una decisione che vive in una **sezione di spec** non è pretesa da nessuno, e se manca qui **per chi legge non esiste**. Successo con la §1.0, ed è costato un traguardo intero da rifare — gotcha **#40**. ⛔ Il rimedio non è irrigidire lo script: è che **chi scrive una decisione fuori da un ADR la porta a mano nel compendio**, perché è l'unico che può saperlo |

📋 **Il messaggio da incollare all'inizio di una chat** vive in
[`AVVIO-CHAT.md`](AVVIO-CHAT.md). Non nomina il prossimo passo, deliberatamente: lo
stato sta nella §6, in un posto solo.

**Cosa aggiornare, e quando:**

| Evento | Cosa tocchi qui |
|---|---|
| ADR nuovo | una voce in **§5** — obbligatoria, la pretende lo script |
| ADR superato | la voce resta e si marca; gli ADR sono **append-only** |
| voce della riapertura chiusa | la tabella e l'ordine in **§6** |
| gotcha nuovo | una riga in **§9**, e il testo integrale in `HANDOFF.md` |
| **misura nuova** | le **fonti** e i **comandi** in `riferimenti.md`, la riga d'esito in `HANDOFF.md`, e le evidenze nell'ADR o nella sezione che la misura decide. ⛔ I prototipi restano nello scratchpad e si ripuliscono |
| decisione dello stack | **§4** |
| cambio del prossimo passo | **§6** |

Il resto della manutenzione — `roadmap.md`, `tracciabilita.md`, stato degli spike,
`HANDOFF.md`, `CLAUDE.md` — resta come prima: **nello stesso passaggio**, alla chiusura
di ogni sotto-progetto.
