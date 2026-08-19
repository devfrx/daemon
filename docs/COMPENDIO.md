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
> accettati — apri **quel** file. Uno, non trentasette. La §12 dice quale.
>
> ⛔ **Cosa NON fare.** Non aprire `HANDOFF.md`, la spec del sotto-progetto 1, o la
> cartella `adr/` «per farsi un'idea». Insieme pesano **oltre mezzo megabyte**
> (726 KB in byte LF il 2026-08-18, e possono solo crescere — la spec da sola ne fa 277), e
> l'idea è già qui.

**Aggiornato il 2026-08-11.** Manutenzione: §13.

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
| schema IPC | **`bincode` 2.0.1** — appuntato a `2`. ⚠️ **Dichiarato NON MANTENUTO** — RUSTSEC-2025-0141, `INFO`, non una vulnerabilità: **registrato il 2026-08-18, si decide al Traguardo 6** | M-1 · §6.1.1 · gotcha #22 · C-1 |
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
e ⛔ **a diventare regressione permanente è la PROPRIETÀ che quel difetto violava, non il
seme** — un seme non riproduce la stessa esecuzione dopo un cambio di codice, quindi è un
**punto di ripartenza per indagare** e non un oracolo, e un elenco di semi presentato come
suite sarebbe una falsa sicurezza. ⚠️ **Richiamo del 2026-08-18, finding A-2:** questa riga
diceva *«e il seme diventa una regressione permanente»*, formulazione **già falsificata in
ADR-0021 il 2026-08-08** e sopravvissuta intatta qui e in
[`design/08`](design/08-strategia-di-test.md) — che si dichiara *fonte di verità sulla porta
di qualità*. È la radice **R1**: una correzione attraversa il documento in cui nasce, non gli
altri. Rimando: ADR-0034 aggiunge il **secondo asse**, i parametri di configurazione.

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
(l'I/O vero) e backend **cadente in memoria** — cade a un'operazione scelta dal seme, ed è
**l'iniezione di livello 2**. ⛔ **Il cadente vive in `platform` e NON in `simulator`, e questa
riga diceva `simulator` fino al 2026-08-11**, come la tabella dell'ADR da cui è compressa:
`redb` non ha `no_std`, i sei metodi di `StorageBackend` restituiscono `std::io::Error`, e il
grafo spedito di `simulator` lo rifiuterebbe come **«I3 violated»** — la cui unica cura scritta
è *togliere la dipendenza*. Non è una decisione riaperta: era una **previsione** scritta quando
`crates/simulator/` non esisteva. Rimando datato in ADR-0032, e la diagnosi è che i **due
livelli di crash erano trattati come una cosa sola** mentre hanno soggetti diversi.
`redb` vive in `platform`, quindi ADR-0031 non lo vincola: il kernel conosce solo la porta
`journal`.

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
✅ **Esistono dal 2026-08-10** — `crates/kernel/tests/frozen_bytes.rs` e `tests/frozen/` — e
sono **tre** record, non uno: i tre enum `index_only` hanno **otto** varianti fra loro e un
record solo ne fisserebbe tre. Le otto sono state rinumerate una per una: **otto rossi su otto**.
✅ **E l'additività della regola 3 è MISURATA, non citata:** un campo facoltativo su un indice
libero lascia i byte **identici** finché è `None` — `minicbor` tronca un `None` in coda invece
di scrivere `null` — e li allunga di un byte quando è `Some`, che è la metà senza la quale il
verde non proverebbe nulla (gotcha **#54**). ⛔ **La mappa è RILETTA dal banco**, non prosa:
offset e byte di ogni riga devono ricostruire il `.cbor`, il che rende impossibile un
segnaposto sopravvissuto al commit (gotcha #43).

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

✅ **I Traguardi 1 e 2 sono eseguiti — il 2026-08-08 e il 2026-08-10.** Il codice del prodotto
non è più zero righe: esiste il workspace, esiste la porta di qualità, la porta è **verde**, e
sopra di essa sta il **substrato iniettabile** al completo. ✅ **Il Traguardo 3 è eseguito il
2026-08-10: dodici compiti su dodici, `GATE GREEN` a tutti** — il **record durevole**
(`crates/kernel/src/record.rs`), la **riga di catalogo dell'etichetta di fiducia** col proprio
caso negativo, il **doppio in memoria del giornale** (`crates/simulator/src/journal.rs`), la
**suite di conformità** con **nove promesse in dieci blocchi e nove bugiardi**
(`crates/kernel/tests/journal_contract.rs` — ⚠️ **i bugiardi sono DODICI dal 2026-08-17**, e le
promesse restano nove: la cifra qui è quella che il Traguardo 3 consegnò), l'operazione **`replay()`** sulla porta, la
**riconciliazione** (`crates/kernel/src/reconcile.rs`), col Task 7 il **primo record che il
kernel scrive davvero** — la nota di `Untrusted::promote` — col **Task 8** la **seconda
implementazione della porta `journal`**: `redb` col **backend scritto da noi**
(`crates/platform/src/journal.rs`), col **Task 9** la conformità che gira contro **entrambe a
ogni commit** (`crates/platform/tests/journal_contract_real.rs`), col **Task 10** i **byte
congelati** (`crates/kernel/tests/frozen/`) e col **Task 11** `prune`, che rifiuta un passo **in
dubbio** e accetta uno riconciliato, e col **Task 12** il **registro riallineato** e questa
chiusura. ⚠️ **Questa cifra ha detto «nove» a dieci compiti eseguiti e «undici» a dodici**, e la
lista si fermava prima ogni volta: gotcha **#31**, ricontata il 2026-08-10 due volte.
⛔ **Una decisione presa in revisione, e va vista perché è un'aggiunta al contratto di una porta
condivisa:** un **secondo `intent`** sullo stesso passo è ora **rifiutato** — ADR-0007 dice
*«l'intento di ogni passo»*, uno per passo — e `OutOfOrder` si **allarga** invece di guadagnare
una variante vicina. ⛔ **I Task 4 e 5 sono stati eseguiti come uno solo**:
separati non funzionano — il Task 4 scrive una suite che chiama `replay()`, che la porta guadagna
solo al Task 5, quindi chiuderebbe con la **porta rossa**.

⛔ **E il Task 6 ha riportato indietro una domanda invece di deciderla, ed è la più importante
che questo traguardo abbia prodotto.** La decisione **D6** del piano dice che la firma di
`replay()` è un'**ipotesi** finché non ha un consumatore, e la riconciliazione è quel consumatore.
Il verdetto misurato: `Vec<(StepId, Vec<u8>)>` **è bastato** — nessuna contorsione — **ma la porta
restituisce meno di quello che sa**. `replay()` non dice **quale delle sue due operazioni** abbia
scritto ciascuna voce, mentre il giornale lo sa: `MemoryJournal` tiene un `EntryKind` interno e
`JournalError::OutOfOrder` è **definito** sulle due operazioni. La riconciliazione ricostruisce
allora quella distinzione dal **campo `kind` del record**, che è una **seconda verità
indipendente**. ⚠️ **Misurate le due direzioni del disaccordo, e falliscono diversamente:** un
`intent()` con un record che dice `Outcome` fa **sparire in silenzio un dubbio vero** — l'unico
fallimento che ADR-0007 esiste per impedire — mentre un `outcome()` con un record che dice
`Intent` riporta in dubbio un passo concluso. ⚠️ **Non è un difetto oggi**, perché nessun codice
del kernel scrive ancora un record: `promote` lo guadagna al **Task 7**. ⛔ **Non decisa qui**,
perché cambiare `replay()` tocca la **porta**, la **conformità** e **due** implementazioni; e la
conseguenza va vista insieme alla decisione: se l'autorità passa alla porta, il campo `kind` del
record diventa **ridondante**, e allora o se ne va — **cambio di formato** — o resta come
riscontro incrociato che **qualcuno controlla davvero**. ⛔ **La scadenza è SCATTATA il
2026-08-10**: i byte congelati esistono, quindi togliere `kind` non è più una rifinitura ma
l'apertura di una `Record::V2`. Dichiarata in `crates/kernel/src/reconcile.rs` ed **E25**
dell'errata.

✅ **Il brainstorming del Traguardo 4 è chiuso il 2026-08-11, e il disegno è scritto:**
[`specs/2026-08-11-…-traguardo-4-simulatore-dst-design.md`](superpowers/specs/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst-design.md).
✅ **E il piano è scritto lo stesso giorno:**
[Traguardo 4](superpowers/plans/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst.md), **dieci
compiti in tre parti**. ✅ **E IL TRAGUARDO 4 È ESEGUITO lo stesso giorno: dieci compiti su
dieci**, subagent-driven, con le due revisioni e `GATE GREEN` a ciascuno. Il simulatore porta ora
**il guasto**: il giornale che cade e non si riprende, il **backend cadente di `redb`** scritto da
**fuori la crate**, due campagne con **soggetti diversi** — la riconciliazione del kernel e la
coerenza del motore — il gotcha **#51 chiuso nella metà chiudibile**, l'elenco dei semi che nasce
**vuoto e non dimenticato**, e il tempo di parete che il cancello **stampa a ogni corsa**.
⛔ **E il 2026-08-11 il repository ha ricevuto il suo primo AUDIT COMPLETO** — codice, script del
cancello, documenti, ADR, diagrammi — con nove revisori paralleli in sola lettura e ogni finding
grave riverificato sul sorgente. Il rapporto è
[`audit-2026-08-11.md`](audit-2026-08-11.md), **e si legge prima di riprendere**.
✅ **Sedici finding corretti e provati nella stessa sessione**; il resto è **aperto e assegnato al
proprietario**. ✅ **E L'AUDIT È CHIUSO IL 2026-08-18, otto decisioni su otto** — il racconto di ciascuna sta
nei riquadri qui sotto. ⚠️ **Questa riga diceva *«il prossimo passo è l'ESECUZIONE
DELL'AUDIT — non il Traguardo 5»***, ed era una delle **quattro case di questo solo file**
che il `grep` del 2026-08-18 ha trovato ferme: il puntatore vivo sta **in cima alla §6**,
in un posto solo.

✅ **E LA PRIMA DELLE OTTO DECISIONI È ESEGUITA IL 2026-08-17 — T-2 e T-1, il finding più grave.**
La suite di conformità del giornale provava **tre** promesse su nove — la **1**, la **5** e la
**8** — soltanto nello stato in cui **ogni guardia plausibile passa**: archivio **vuoto**, o con
**un passo solo**. Chiuse con un **passante** in archivio: un passo **diverso** da quello sotto
esame è l'unico stato in cui *«l'archivio è vuoto»* smette di coincidere con *«questo passo non ha
un intento»*, e *«il record di questo passo»* con *«il primo record che c'è»*.
⛔ **E la notizia è quanto è costato, perché l'audit lo prezzava come un'altra cosa.** La §8 dava
la decisione 1 per *«un'aggiunta al contratto di una porta condivisa»*; letta contro il codice di
oggi non è servita **nessuna promessa nuova** e **nessuna riga di prodotto** è stata toccata — le
due implementazioni filtrano già per passo (`has_intent(step)`, `stored == step.get()`) e la porta
lo dichiara già (*«re-reads ONE step BY NAME»*, e le tre vie di `OutOfOrder`). A mancare non era il
contratto: era lo **stato che distingue una guardia sbagliata**. 📌 È la quinta domanda del
pre-controllo che ha pagato — *un compito scritto prima si legge contro il codice di adesso* —
applicata a un **rapporto d'audit** invece che a un piano.
⛔ **E i finding erano TRE e non due, che è il gotcha nuovo #65:** l'audit raggruppa le promesse 5
e 8a in un finding solo, e ha ragione sulla **causa** — `note` e `outcome` condividono `has_intent`
— ma la suite **muore alla prima promessa rotta**, quindi un bugiardo cieco su entrambe muore sulla
5 e il blocco della 8a resta **non provato mentre un test afferma il contrario**. Ne servono
**due**, e sono un tipo solo con **due istanze**, o sarebbero lo stesso difetto scritto due volte.
✅ **Il rosso è stato riprodotto PRIMA di correggere**, che è la disciplina di questo repository: i
tre bugiardi — **J14** `StepBlindJournal`, **J15** e **J16** `BlindGuardJournal` — contro la suite
com'era hanno risposto tutti e tre `THE SUITE IS VACUOUS ON promise 1 / 5 / 8`.
✅ **E la seconda direzione è misurata sulle IMPLEMENTAZIONI VERE e non sui soli bugiardi**, perché
un bugiardo prova che il blocco morde, non che il blocco raggiunga `redb`: **sei** mutazioni — le
due guardie e il predicato di `read_back`, su ciascuna delle due implementazioni — applicate **una
alla volta**, ciascuna compilata ed eseguita a sé e poi revocata. **Sei rosse su sei, ciascuna col
messaggio della propria promessa**; prima del rimedio tutte e sei lasciavano il workspace verde. A
campagna chiusa `git diff --stat` nomina **il solo file della suite**. La tabella sta in
[`porta-di-qualita.md`](porta-di-qualita.md).
⚠️ **E due conteggi erano stantii PRIMA di questa passata**, trovati contandoli invece che
leggendoli: `journal_contract_real.rs` dichiarava che la suite chiama la fabbrica **nove** volte —
sono **dieci** dal giorno in cui `note` divise la promessa 8 in due blocchi — e **otto** bugiardi,
che erano nove dal Task 11. Gotcha **#31**.
📌 **Baseline dopo il rimedio:** `GATE GREEN`, `cargo test --workspace --no-fail-fast` →
**32 target, 177 passati, 0 falliti, 2 ignorati** (erano 171: tre test nuovi qui e tre nella copia
che `platform` include).
✅ **E LA SECONDA DECISIONE È ESEGUITA IL 2026-08-18 — G-5, `--locked` nel cancello.** Il
`Cargo.lock` **tracciato** era un **effetto** del cancello invece che un suo **ingresso**: un
manifesto derivato faceva ri-risolvere `cargo`, il lockfile veniva riscritto in silenzio, e
`gate-deps.sh` — che misura il grafo transitivo contro la lista di ADR-0031 — misurava allora **il
grafo che `cargo` aveva appena inventato**, credendo di misurare quello approvato.
⛔ **Riprodotto prima di correggere, cosa che il rapporto non aveva fatto:** tolta la riga di
`minicbor` da `crates/kernel/Cargo.toml`, il controllo rispondeva `OK -- the two graphs match the
two lists`, **exit 0**, col lockfile alleggerito di **33 righe**. La guardia di non-vacuità **non**
lo coglieva: i due grafi erano non vuoti e **diversi**.
⛔ **E il rapporto lo prezzava «una riga», cioè al rovescio della decisione 1:** i siti `cargo` del
percorso del cancello sono **sei** su **tre** script — `gate.sh` ×4, `gate-no-os.sh` ×1,
`gate-deps.sh` ×3 — perché i due script si lanciano **anche da soli**, e un controllo che vale solo
passando dal cancello è più debole di uno che vale sempre. 📌 **Il rimedio si prezza leggendo il
codice, non il rapporto** — gotcha **#65**, e stavolta la misura lo ha fatto **crescere** invece
che restringere.
⛔ **E chiuderlo alla lettera avrebbe aperto una vacuità nuova.** Con `--locked` un `cargo tree`
che fallisce lascia **entrambi** i grafi vuoti, quindi coincidenti, quindi la guardia di
non-vacuità diventa rossa **da sola** — dicendo però *«la query era stretta»* dove la verità è
*«il lockfile è stantio»*. Il ramo d'errore esplicito compra la **diagnosi**, non il rosso, ed è
scritto così accanto al codice invece che taciuto. ⚠️ **E l'errore si mostra RI-ESEGUENDO invece
di unire `stderr` alla cattura:** un `cargo tree` che stampa *«Blocking waiting for file lock on
package cache»* darebbe a `names` la parola `Blocking`, che passa la sua classe di caratteri e
verrebbe riportata come **intrusa su I3** — un rosso per la ragione sbagliata, cioè il gotcha
**#41** spostato dallo stesso filtro alla sua sorgente.
✅ **Due direzioni misurate, sonde N6 e N7** di [`porta-di-qualita.md`](porta-di-qualita.md).
**N6:** stesso guasto col rimedio → **exit 1**, messaggio che nomina il lockfile stantio, lockfile
**intatto**; cancello intero → **`GATE RED -- 5 checks failed`**, con `Cargo.lock` immobile per
tutta la corsa. **N7:** stato pulito → `GATE GREEN`, **32 target, 177 passati, 0 falliti, 2
ignorati**, `git status` vuoto. ⚠️ **N7 non prova che il rimedio morda** — anche prima quella corsa
lasciava l'albero pulito: serve a escludere che `--locked` renda rosso uno stato **corretto**, e le
due si leggono in coppia.
⚠️ **Il costo è dichiarato accanto al codice:** aggiungere o alzare una dipendenza è ora un atto in
**due passi** — il manifesto da solo lascia il cancello **rosso**, e il lockfile va rinfrescato
**fuori** dal cancello e committato **insieme** al manifesto. È il punto e non il prezzo: ADR-0031
chiama l'aggiunta di una voce *«un atto deliberato e rivedibile»*, e un lockfile che il cancello
aggiorna da sé non è né l'uno né l'altro.
⚠️ **E il banco ha dato la trappola dei fine-riga mentre la si applicava:** `sed -i` ha
**normalizzato CRLF → LF** su `crates/kernel/Cargo.toml` — da **43 CR a zero** — senza dirlo, che è
la riga di `CLAUDE.md` incontrata eseguendola invece che leggendola. Ripristino da una **copia
byte-esatta** presa prima, mai da `git checkout --` (gotcha **#48**, dodicesima forma), e i tre
script sono stati modificati con uno strumento di edit dopo aver **misurato** che i CR reggessero.

✅ **E LA SESTA DECISIONE È ESEGUITA IL 2026-08-18 — i quattro rimandi datati A-1, A-2, A-4,
A-7.** Nessuna decisione riaperta: cadono quattro **evidenze**, non quattro scelte.

| | Cosa diceva il documento | Cosa dice il codice |
|---|---|---|
| **A-1** | [ADR-0026](adr/0026-linguaggio-del-core.md): *«esiste `madsim` … quindi il simulatore non va scritto da zero»* | `simulator` ha **una** dipendenza e **512** righe a mano; `madsim` non è né in `Cargo.lock` né in `crates/`. E a scartarlo fu [ADR-0031](adr/0031-dipendenze-del-kernel-parte-del-confine.md) — **stessa data** — a 55 crate |
| **A-2** | *«il seme diventa una regressione permanente»* | falsificata in ADR-0021 il **2026-08-08**: a entrare nella suite è la **proprietà**, non il seme |
| **A-4** | [`design/01`](design/01-topologia-dei-processi.md): canale worker **a senso unico** | il tratto `Worker` ha **sei** verbi contati sul sorgente, e i frame **risalgono** — dentro una **ricevuta** |
| **A-7** | `OpenError`: *«la radice di composizione lo apre, una volta»* | `crates/daemon/src/main.rs` cabla `SequentialRng`, `SystemReactor`, `Parameters`, `Sleep` e l'esecutore — e **nessun giornale** |

⛔ **Due erano più larghi di come il rapporto li scrive, e in due modi diversi.** **A-2** viveva
in **due** case oltre ad ADR-0021 — questa §5 e [`design/08`](design/08-strategia-di-test.md),
che *si dichiara fonte di verità sulla porta di qualità* ed è quindi l'ultimo posto in cui una
formulazione falsificata dovrebbe sopravvivere: ci è sopravvissuta **dieci giorni**, ed è la
radice **R1**. **A-7** non è una frase imprecisa ma una **previsione citata come misura**: *«la
radice di composizione lo apre»* è scritta al presente e si legge come un fatto, mentre parla di
codice **non ancora scritto** — gotcha **#57** applicato a una **giustificazione** invece che a
una collocazione. 📌 E il richiamo dice la cosa esatta: **l'argomento regge, l'evidenza no.**
`open` davvero non è un'operazione della porta, e quella è una proprietà leggibile **oggi**.

✅ **E LA QUINTA È ESEGUITA LO STESSO GIORNO — C-1, e ciò che si esegue è la REGISTRAZIONE.**
`bincode` 2.0.1 è coperto da **RUSTSEC-2025-0141 — «Bincode is unmaintained»**, emesso il
**2026-01-07**, categoria **`INFO`**: non è una vulnerabilità, e le altre sette dipendenze sono
pulite. ⛔ **La notizia non è l'avviso: è il BUCO FRA DUE CRITERI.** [ADR-0037](adr/0037-criterio-del-pari-per-il-formato-dei-canali.md)
chiede *«il pari ha un lettore conforme e **mantenuto**?»* — puntato verso il **capo lontano** del
filo, TypeScript, misura M-11; **M-1** puntava verso di noi e chiedeva un'altra cosa, *«il grafo
transitivo è accettabile per I3?»*. **Nessuno dei due chiede se sia mantenuta la libreria del
NOSTRO capo**, e `gate-deps.sh` verifica **quali** crate ci sono, non **come stanno**. L'avviso
era pubblico **sette mesi prima** che §6.1.1 fosse riconfermata il 2026-08-08.
✅ **E il costo di agire è quasi zero oggi, misurato invece che citato:** `grep -rn bincode crates/
--include=*.rs` dà **zero usi di produzione** — un commento di documentazione in
`crates/kernel/src/ports/ipc.rs` e una sonda in `crates/kernel/tests/dependencies_usable.rs`. Lo
schema del canale `ipc` è il **Traguardo 6**. ⛔ **È una finestra che si chiude da sola**, come la
quarta proprietà della §3: si **decide allora**, mentre la scelta è ancora libera, e la
registrazione vive **accanto alla voce** in `crates/kernel/Cargo.toml` — dove guarda chi la tocca
— invece che solo qui. 📌 **La domanda che ne esce, e vale oltre il caso:** *questo criterio è
puntato verso **entrambi** i capi del filo?* Gotcha **#64**.

✅ **E LA QUARTA È ESEGUITA LO STESSO GIORNO — PL-1, e la scelta del proprietario è `0600` SUL
FILE.** ADR-0023 promette che il giornale a riposo sia *«protetto quanto il tuo account di
sistema»* e pretende che la frase si mostri **in interfaccia**; il file nasceva **0644** su Linux,
cioè **leggibile da chiunque**, cioè **meno** dell'account. ✅ **Misurato su un Linux vero — WSL,
`umask` 0022 — invece che dedotto dai doc di `std`:** `open` a `0o666` dà **644**, a `0o600` dà
**600**. Non c'era **nessun** `.mode()` in tutto `crates/`.
⛔ **Perché il file e non la cartella, che era l'altra opzione:** `0700` sulla cartella coprirebbe
anche gli archivi futuri in un colpo solo, ma **la cartella non ha un proprietario nel codice** —
nessuno la crea — quindi la regola nominerebbe un **chiamante che non esiste**, che è esattamente
il difetto del finding **A-7** chiuso poche ore prima. Prenderselo per risparmiare una riga era lo
scambio peggiore.
⛔ **E la cosa da ricordare non è il permesso: è che il difetto era INVISIBILE DOVE SI LAVORA.**
Windows non ha il modo Unix, quindi né il codice né una sonda potevano dirne nulla sull'host di
sviluppo, e il rosso era **programmato per uscire il giorno del secondo sistema** — la stessa
forma del gotcha **#52**. È la ragione per cui l'audit dichiarava PL-1 *fuori copertura*; a
renderlo misurabile è stato notare che la **CI gira su `ubuntu-latest`**.
✅ **Provato per quanto si può da qui, e il resto è dichiarato:** il percorso `cfg(unix)` è stato
**type-checkato per Linux** prima del push — `cargo check --locked -p platform --target
x86_64-unknown-linux-gnu --tests` — perché su Windows quel blocco **non viene nemmeno compilato**;
la direzione *«deve scattare»* è provata dalla **misura del sistema** (senza la riga il file nasce
644, e `644 & 0o077 ≠ 0`) e non da una corsa del banco mutato; il **valore vero** lo misura la CI.
⚠️ **L'asserzione è «nessuno tranne il proprietario» e non «esattamente 0600»**, perché `mode()`
resta mascherato dall'umask e un'uguaglianza esatta andrebbe **rossa su un sistema più chiuso del
richiesto**, cioè dove la promessa è **mantenuta** — gotcha **#24**.
⚠️ **Due limiti dichiarati:** `mode()` vale solo alla **creazione**, quindi un giornale nato prima
resta 0644 e portarlo giù è una **migrazione** che non esiste ancora; e la sonda **non è una riga
di catalogo** — aggiungerla alla §7.4 è una decisione del proprietario, e fino ad allora sta
**registrata come voce aperta** nel registro invece che come nota (gotcha **#36**).
📌 **E il conteggio dei test di quel file è il primo del registro che DIPENDE DAL SISTEMA**: sei
su Windows, sette su Linux. Dichiarato invece di sceglierne uno.

✅ **E LA TERZA È ESEGUITA IL 2026-08-18 — K-1 insieme a B-1, e il rapporto le prezzava sbagliate
in TRE modi.** La cella `Sleep` accettava scritture da **fuori un poll**, e una sospensione che
nessuna attività aveva chiesto veniva onorata su un'attività **scelta dal seme**.
⛔ **Il rimedio della §8 — drenare all'ingresso di `run` — non basta e non è ben puntato.** Le vie
sono **due**, e la seconda è **dentro** la run: un distruttore gira dopo l'ultima lettura del
ciclo, e col drenaggio all'ingresso il clock arrivava ancora a **9999**, misurato.
📌 **La forma giusta è svuotare la cella SUBITO PRIMA di ogni poll**, ed è **una riga**: sposta
l'invariante da *«nessuno scriva mai fuori da un poll»*, che nulla può imporre, a *«conta solo ciò
che è scritto durante questo poll»*, che è imposto lì. Tutte le vie chiudono in un punto solo,
comprese quelle non ancora immaginate. ⛔ **La via idiomatica non si riapre:** un waker su misura
non è costruibile qui — `Waker::from_raw` è `unsafe` — **misurato in M-5**. E far possedere la
cella all'`Executor` è più invasivo **e non chiude il `Drop`**: caduta sul merito, non sul costo.
⛔ **E LA NOTIZIA È COSA SUCCEDE ALL'ALTRA SONDA, che è il gotcha nuovo #66.** Il rapporto dice
che chiuderlo rende rosse **due** sonde permanenti. Ne diventa rossa **una**: l'altra —
`a_wait_already_over_wakes_immediately_and_the_clock_does_not_move` — **resta verde e diventa
vacua**. Misurato invece che dedotto: col rimedio applicato e `until <= instant` mutato in
`until < instant`, cioè **la discriminazione che il suo stesso commento dichiara di difendere**,
la forma vecchia resta **verde** mentre la stessa mutazione fa rossi **cinque** altri test.
Riscritta con l'attività che dichiara la propria scadenza, va rossa. 📌 **Un rosso lo vedi, una
vacuità no** — e nessuna delle sette domande del pre-controllo la coglie, perché guardano il
compito, mai le sonde che poggiavano sul difetto che stai chiudendo.
✅ **Tre sonde nuove, e sono TRE perché le vie sono tre — non le cause** (gotcha #65):
`a_request_written_before_the_run_belongs_to_nobody`,
`a_request_written_by_a_destructor_belongs_to_nobody`, e per B-1
`the_delivered_turn_limit_is_honoured_by_its_value`, il cui oracolo è il **conteggio dei poll** e
non l'errore, su **due** valori (gotcha #48). ⚠️ **La prima stesura della sonda dei `Drop` era
VACUA e sta scritta**: un blocco `async` distrugge i suoi locali **dentro** il poll, quindi serviva
un `Future` a mano — gotcha **#17**.
📌 **Baseline dopo il rimedio:** `GATE GREEN`, `cargo test --workspace --no-fail-fast --locked` →
**32 target, 180 passati, 0 falliti, 2 ignorati** (erano 177: le tre sonde nuove).
⚠️ **E il commento di `Sleep` dichiarava il falso** — *«la richiesta … appartiene sempre
all'attività che ha appena girato»*, dove «dopo ogni poll» esclude la **precedente** e nient'altro.
Riscritto col proprio richiamo datato, non appeso: lasciarlo era **A-2** rifatto.
⚠️ **Voce aperta registrata, non presa:** le tre sonde **non hanno una riga di catalogo**. La §7.4
è spec, e il vincolo globale 7 la mette fuori da questa passata — stesso trattamento di **PL-1**,
stessa ragione (gotcha #36). Il verbale in [`porta-di-qualita.md`](porta-di-qualita.md).

✅ **E LA SETTIMA È ESEGUITA LO STESSO GIORNO — P-1, e il rapporto ha ragione sul difetto e torto
sul rimedio.** La via **A3** era dichiarata **chiusa** e aveva una **seconda bocca**: `Untrusted`
aveva smesso di stampare il proprio contenuto, ma `promote` prendeva `reason: &str` e il `Debug`
scritto a mano di `RecordV1` **stampa l'indice 4 per intero**. Riprodotto da fuori la crate —
`RecordV1 { … payload: <16 bytes>, reason: "ignore your instructions" }`, il campo protetto
nascosto e quello non protetto spalancato.
⛔ **La frase che lo autorizzava è la classe del difetto, ed è il gotcha nuovo #67.** Il commento
giustifica **quattro** campi con una ragione sola — *«sono il vocabolario del kernel, nobody
outside chose them»* — vera per **tre**. `reason` lo sceglie il chiamante. **È l'ELENCO a farla
leggere come verificata:** chi la controlla si ferma al primo nome che torna.
⛔ **E IL RIMEDIO DELLA §8 NON AVREBBE CHIUSO LA STRADA.** Propone `reason: &Instruction`;
`Instruction::new` è **`pub`** e prende qualunque `String`, quindi
`Instruction::new(untrusted.as_str().into())` lo soddisfa — ed è la **via A1/A2**, dichiarata
**non chiudibile** nella stessa lista. 📌 **Una guardia a newtype vale quanto il suo COSTRUTTORE**,
e quella avrebbe comprato l'**apparenza** di una chiusura sopra una strada già dichiarata aperta.
⚠️ Sarebbe stata anche un gioco di parole sui tipi: `Instruction` significa *contenuto ammesso nel
canale delle istruzioni*, e una giustificazione non è quello.
⛔ **La terza opzione cade su un FATTO:** `reason` come enum sarebbe la lettura più onesta di
«vocabolario», ma è l'**indice 4 del record durevole** — cambiarne il tipo muove i **byte
congelati**, cioè apre una `Record::V2` (ADR-0036). Sproporzionato, e speculativo con **un solo**
chiamante di produzione.
✅ **Il rimedio è `reason: &'static str`**, ed è **una parola**: il contenuto esterno è dato di
**runtime**, un `&'static str` è un letterale nel binario. **Zero** siti di chiamata riscritti —
erano tutti già letterali, misurato — **un** oracolo `.stderr` aggiornato a mano, e il **formato
non si muove** (`frozen_bytes.rs` sei su sei). ✅ **La sonda è un caso `compile_fail` nella forma
forte del #42:** rimessa la firma a `&str` il caso **compila**, e `trybuild` risponde `error`
invece di `mismatch` — non lo disarma un `TRYBUILD=overwrite` in blocco.
⚠️ **Resta aperto e dichiarato:** `String::leak` dà ancora un `&'static str` — stesso scambio con
cui **A5** liquida il `transmute` — e un letterale può **mentire**, che è provenienza e non
correttezza, il limite che **A4** già dichiara. Ciò che ha chiuso è la strada che si prende **senza
accorgersene**. ⚠️ **Voce aperta registrata, non presa:** la riga di catalogo, come per K-1/B-1.
📌 **Baseline invariata:** `GATE GREEN`, **32 target, 180 passati, 0 falliti, 2 ignorati**; i casi
di `compile_fail` passano da diciassette a **diciotto**.

✅ **E L'OTTAVA È ESEGUITA LO STESSO GIORNO — la decisione 7, le cinque sonde mancanti. L'AUDIT È
CHIUSO: otto decisioni su otto.** Le cinque voci sono **quattro soggetti diversi**, e ciascuna è la
stessa forma di difetto — *un'asserzione vale solo lo stato in cui è fatta* — su una porta diversa.
⛔ **B-2, e la suite colpita è la più importante del progetto:** la conformità del `reactor`, su cui
poggia la validità dell'intera simulazione deterministica, aveva **due bugiardi per UN gruppo** —
cancellare i blocchi 1, 3, 4 e 5 lasciava il workspace verde. ✅ **Cinque bugiardi nuovi, uno per
ASSERZIONE e non per gruppo** (#65). ⛔ **Il gruppo 5 non ha asserzioni**, quindi era l'unico blocco
la cui cancellazione nessun oracolo poteva notare: un `wall_time` che esplode trasforma *«il blocco
esiste»* in *«il blocco gira»*. ⛔ **E scrivendo i bugiardi è uscito un difetto che l'audit non
aveva visto: l'asserzione 4b è IMPLICATA dalla 4a** — `second_deadline` è calcolata da
`first_reached`, quindi un bugiardo per la 4b **non è scrivibile**. Non è vacua, è **muta**.
Registrata e non presa: toglierla tocca la conformità di una porta condivisa.
⛔ **B-3:** i test tenevano **un solo** checkpoint, e lì *«trova per id»* e *«prendi il primo»* sono
la stessa frase — 13 su 13 verdi sotto la mutazione. 📌 **Il rimedio è il PASSANTE, identico a
quello della prima decisione di questo audit** sulla conformità del giornale: stesso difetto, porta
diversa. E due argomenti nel sorgente vi poggiavano — `CheckpointId` e `ClientId` non hanno getter
*«perché un'implementazione lo ritiene e lo CONFRONTA»*, un argomento su un confronto che nulla
osservava.
⛔ **S-1/S-2:** il percorso di **successo** di `CrashingJournal::note` non era **mai** preso — ogni
`note` del file rispondeva `NotDurable` — e *«il contatore si muove solo su un `Ok`»* era tenuto per
il solo `outcome`. 📌 **La sonda lo aveva scritto di sé stessa** — *«esclusività su un insieme che
cresce è l'affermazione che invecchia in silenzio»* — e ha invecchiato in **sette giorni**.
⛔ **E la terza mutazione ha trovato un buco nella sonda che avevo appena scritto:** controllava il
**contatore** e non che la nota **raggiungesse l'archivio**, e una `note` che risponde `Ok` senza
delegare muove il contatore ugualmente. Chiusa leggendo `replay()`. È il **#66** applicato a sé
stessi.
⛔ **S-5:** `partial > 0` è soddisfatta da **UN** gradino, mentre l'intera ragione per cui la
campagna di livello 2 è **profonda** invece che **larga** è la tabella dei pioli del disegno —
4/4, 11/11, 21/21, 31/31, 41/41 — che era **prosa**. ✅ **Ora è un'uguaglianza**, e la tabella regge
a entrambe le profondità: **`rungs=4/4`** sulla corta, **`rungs=31/31`** sulla profonda, rimisurate
invece che citate.
📌 **Baseline:** `GATE GREEN`, **32 target, 194 passati, 0 falliti, 2 ignorati** — erano 180. I
quattordici in più sono le nove sonde nuove, di cui le cinque del `reactor` contano **doppio**
perché quel file è `include!`d anche da `platform`.
⚠️ **Voce aperta CONSOLIDATA:** le **dieci** sonde permanenti che l'esecuzione dell'audit ha
prodotto **non hanno riga di catalogo**, e la §7.4 è spec (vincolo globale 7). Raccolte in una
tabella sola in [`porta-di-qualita.md`](porta-di-qualita.md) invece che in quattro riquadri, perché
quattro voci aperte sullo stesso oggetto sono il modo in cui una smette di esserlo senza che
nessuno l'abbia chiusa.

⏭️ **IL PROSSIMO PASSO È IL TASK 5 DEL TRAGUARDO 5** — l'arbitro che ammette e rilascia.
⛔ **L'esecuzione è COMINCIATA il 2026-08-19: quattro compiti su tredici, `GATE GREEN` a
ciascuno**, subagent-driven, un compito per volta con revisione fra uno e l'altro. Il racconto
sta nel riquadro subito sotto, e ciò che chi riprende deve sapere **prima** di aprire il Task 5
sono le **tre** cose che quel riquadro elenca in fondo.
✅ **Il brainstorming è chiuso il
2026-08-18, il disegno è scritto**
([`specs/2026-08-18-…-traguardo-5-arbitro-gpu-design.md`](superpowers/specs/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu-design.md),
dodici sezioni) **e lo stesso giorno è scritto il PIANO**:
[`plans/2026-08-18-…-traguardo-5-arbitro-gpu.md`](superpowers/plans/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu.md),
**tredici compiti in cinque parti**. Si esegue **subagent-driven**, un compito per volta con
revisione fra uno e l'altro.
⚠️ **Questa riga ha detto *«il TRAGUARDO 5, e si riparte dal BRAINSTORMING»***, poi *«il PIANO
del Traguardo 5»*, poi *«l'ESECUZIONE del Traguardo 5»*: è la riga che invecchia per costruzione, e vive **in un posto solo**.

✅ **E IL 2026-08-19 L'ESECUZIONE È COMINCIATA — QUATTRO COMPITI SU TREDICI, `GATE GREEN` a
ciascuno**, subagent-driven: un subagente fresco per compito, una revisione fra uno e l'altro, e
il pre-controllo delle sette domande **prima** di ogni dispaccio.

| | Cosa ha portato | Commit |
|---|---|---|
| **Task 1** | il modulo `arbiter` nasce e `Mib` **non è un intero nudo** — quattro casi `compile_fail` (le due direzioni più le due vie `From`) e tre sonde sull'aritmetica, dove la **direzione** della saturazione è l'asserzione | `dc6ac4c` |
| **Task 2** | `ComputeClass` a tre corsie con `Ord` **scritto a mano da una chiave esplicita** — un `Ord` derivato segue l'ordine di dichiarazione, quindi riordinare le varianti rovescerebbe le priorità **senza un rosso** — e `Preemption::{Never, After(Millis)}`, che fa sparire **due** stati illegali insieme | `2fab856` |
| **Task 3** | `ResourceProfile` e `WorkDescriptor`, con **`cold_start` fuori dall'ammissione**: una decisione che volesse leggerlo **non ha una strada** | `89e6632` |
| **Task 4** | `Grant` **si sposta** da chi lo consuma a chi lo emette, `Admission` a **tre vie** e `Activity` **annidata** — la revoca di una concessione non prelazionabile non è *vietata*, è **impronunciabile** | `3c5df88` · `b91186d` |

📌 **I numeri, misurati e non dedotti:** `cargo test --workspace --no-fail-fast --locked` →
**34 target, 204 passati, 0 falliti, 2 ignorati** (erano 32 e 194 all'apertura del traguardo); i
casi di `compile_fail` passano da diciotto a **ventisei**. ⛔ **E il conteggio della baseline che
il piano scrive nei propri passi è STANTIO per costruzione** — dice *«32 target, 194 passati»*,
che era vero prima del Task 1: ogni compito **misura la propria** prima di cominciare, o
confronta contro un numero che i compiti precedenti hanno già spostato.

⛔ **L'errata del piano è a DICIASSETTE voci in quattro compiti, e il pre-controllo ha trovato un
difetto in quattro compiti su quattro.** Nessuna riapre una decisione: cadono comandi di verifica
che **non potevano fallire**, un import, una citazione sbagliata, e due mutazioni dettate che non
rovesciavano il proprio caso.

⛔ **TRE COSE DA SAPERE PRIMA DI APRIRE IL TASK 5**, e sono tutte state pagate:

| | |
|---|---|
| **E10 — un warning viaggia di proposito, con una scadenza FALSIFICABILE** | `Grant { id: GrantId }` accende `field 'id' is never read`, perché il suo lettore è `release`, che è il **Task 5**. ⛔ Al Task 5 `cargo build --locked --workspace` deve dare **zero warning**: se il warning è ancora lì, il campo **non serviva**, e allora va tolto il campo — non il warning. ⚠️ Nessun `#[allow]`: ciò che il vecchio commento di `Grant(())` rifiutò era **quello**, non un campo |
| **E15 — `crates/kernel/tests/arbiter_admission.rs` ESISTE GIÀ** | è nato al Task 4 per tenere la seconda direzione di `I2 · §5.3`, che altrimenti sarebbe rimasta a una mutazione revocata. La tabella dei file del piano lo assegna ai Task 5–7: per il Task 5 è un **Modify**, non un **Create**, e il commento di modulo dettato dal piano si **fonde**, non si sovrascrive |
| **La lezione della revisione del Task 4, imparata DUE volte nello stesso compito** | *una direzione tenuta da una mutazione che poi revochi **non è tenuta***, e una sonda si registra per il morso che **ha**. `V4` era dichiarata chiusa su una mutazione revocata — e la regola che lo vieta stava scritta **quattro righe sotto, nello stesso file** (gotcha **#68**); e la sonda nuova era registrata *«rossa a runtime»* mentre i suoi `assert_ne!` confrontano varianti **diverse** di un enum, quindi non possono fallire: la sua forza è a **compilazione**, ed è legittima — a non esserlo era il verbale |

⚠️ **Dove sta il dettaglio, e perché non è qui:** le sonde, le mutazioni con il proprio esito
misurato e le righe di catalogo coperte stanno in [`porta-di-qualita.md`](porta-di-qualita.md),
aggiornato **a ogni compito**; le divergenze stanno nell'**errata in testa al piano**, che si
legge **prima** del compito. Questa sezione dice **dove siamo**, non ciò che quei due dicono già.
⛔ **E ciò che l'audit lascia al Traguardo 5 va saputo PRIMA di aprirlo:** le **nove righe di
guasto scoperte** della §3.3 hanno ciascuna il proprio indirizzo — e l'**arbitro è questo
traguardo** — la metà non chiudibile del gotcha **#51**, `semi-dst.md` che **non ha un chiudente**,
e la voce aperta consolidata qui sopra. ⛔ **E il disegno ha ricontato: le righe che il Traguardo 5
eredita davvero sono UNA, più una condivisa** — non cinque. Vedi il riquadro qui sotto.

✅ **E IL 2026-08-18 UNA PASSATA DI COERENZA HA TOLTO LA RIGA CHE MARCIVA, invece di
correggerla per la terza volta.** Il puntatore al prossimo passo e il **conteggio delle
decisioni d'audit** vivevano **riscritti**: il puntatore in **cinque** documenti di stato, il
conteggio in **quattro**. ⛔ **Ricontate invece che stimate: il puntatore aveva NOVE case** —
di cui **quattro in questo solo file** — e il conteggio **sette**, con **tre valori distinti**
(*«ne restano tre»*, *«quattro»*, *«cinque»*) contro il vero, che è **zero**: ciascuno fermo
alla decisione che l'aveva scritto. ⛔ **[`HANDOFF.md`](HANDOFF.md) da solo ne teneva quattro,
con tre valori diversi**, e una di esse portava per giunta un conteggio di **eseguite** —
*«due»* — falso allo stesso modo; la §12 di **questo** file era la settima.
⛔ **E la §8 dell'audit stesso dava TRE decisioni per non eseguite** — la 2 (P-1), la 3 (K-1) e
la 7 (le sonde) — mentre il commit che le chiudeva si intitola *«otto decisioni su otto»*: il
verbale non era stato timbrato. Timbrate ora, e ciascuna col **prezzo vero** del rimedio.
⚠️ **La regola per evitarlo esisteva già, scritta due volte.** `CLAUDE.md` dice *«lo stato
corrente e il prossimo passo stanno nella §6 — non qui, o si disallineano»* — e lo dice
**quarantasei righe sotto** una riga propria che dava l'audit per prossimo passo; e la 25ª
misura della §12 prescrive per intero *«si cerca `grep '⏭️'` su tutti i documenti di stato e
si guardano tutte le case»*. 📌 **Un promemoria non è un controllo**, ed è la **terza**
occorrenza della radice **R1** su questa identica riga: finding **D-1**, poi il 2026-08-17,
poi oggi. 📌 **E la regola violata viveva DENTRO il documento che la violava** — gotcha
**#68** — mentre lo strumento che ha applicato il rimedio ha dato il **#69**.
✅ **Quindi il rimedio non è ricorreggere le case: è TOGLIERLE.** I documenti secondari ora
**rimandano** alla §6 invece di riscriverla — e un rimando non può marcire. ⛔ **E la prova di
quale forma regga era già nel repository, non è un'opinione:**
[`AVVIO-CHAT.md`](AVVIO-CHAT.md) è l'unico documento che su questa riga non è **mai** marcito,
in nove passate, ed è l'unico che **si rifiuta di nominare il prossimo passo** dichiarandone
la ragione. 📌 È la 25ª misura applicata a un secondo oggetto: *decidere cosa **togliere**,
non cosa accorciare*.
⚠️ **Ciò che NON è stato toccato, e per una ragione:** lo **stato per traguardo** resta nelle
tabelle di [`roadmap.md`](roadmap.md) e [`README.md`](README.md). È il loro mestiere, non è la
cifra che è marcita, e toglierlo sarebbe stato invasività senza guadagno — il perimetro di una
passata si prende dal **drift misurato**, non dalla categoria.

✅ **E IL 2026-08-18 IL BRAINSTORMING DEL TRAGUARDO 5 È CHIUSO, E IL DISEGNO È SCRITTO** —
[`specs/2026-08-18-…-traguardo-5-arbitro-gpu-design.md`](superpowers/specs/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu-design.md),
**dodici sezioni, 31 KB**. Il perimetro: **l'arbitro intero**, e si prova ciò che si può provare
**senza un secondo meccanismo**.
⛔ **La decisione che governa le altre, e nasce da una formulazione corretta prima di scrivere.**
Delle cinque proprietà della §5.7, tre si iniettano su porte che esistono e due su porte che non
hanno implementazione — ma *«due non si possono fare»* era troppo grosso: quelle due sono **due
metà incollate**, e la metà d'**arbitro** si prova oggi. 📌 **E le due metà d'arbitro sono UNA
SOLA** — *rilasciare restituisce la riserva* — perché all'arbitro non serve sapere **chi** tiene
una concessione: dargli un «titolare» lo accoppierebbe a `ipc` per una cosa che il Traguardo 6
risolve con una mappa dalla propria parte.
⛔ **E il fatto che decide la collocazione di `Grant` è MISURATO, non dedotto:** un modulo
**fratello** di `ports::process` **non può costruire** `Grant` — `error[E0423]`, riprodotto su una
crate usa-e-getta. Quindi il tipo **si sposta nel modulo `arbiter`**, che è chi lo emette; un
costruttore `pub(crate)` dove sta ora avrebbe comprato l'apparenza — *una guardia vale quanto il
suo costruttore*, gotcha **#67**.
✅ **E la notizia buona: il Traguardo 5 NON CREA righe di catalogo, ne CHIUDE dodici già scritte**
— tre nel blocco B dei gettoni, otto nel blocco C, una di livello 2. Contate sul catalogo §7.4 e
non dedotte. Fra queste, le **quattro righe di §6.10.5** ferme dal Traguardo 2 perché *«senza
`Grant` non si ottiene un `Worker`»*.
⛔ **E il disegno ha trovato una cifra sbagliata in due documenti di stato: «cinque delle nove
righe di guasto».** Contate sulla fonte — la §7 del disegno del Traguardo 4, che è la tabella che
gli indirizzi li **assegna** — sono **una** al Traguardo 5, **una** condivisa e **sette** al
Traguardo 6. 📌 **Da dove viene il cinque:** la §5.7 della spec ha esattamente **cinque** righe, ma
sono le **proprietà che la DST verifica**, non le righe di guasto. Due tabelle diverse, e la cifra
dell'una letta contro l'altra. ⚠️ **Registrata e non presa:** `CLAUDE.md` prescrive di
**toglierla**, non di ricorreggerla, ed è una decisione del proprietario.
⚠️ **Due divergenze dalla lettera della spec, dichiarate perché il proprietario possa ribaltarle
vedendole:** la §5.1 dice *«i tre addendi sono parametri consegnati»* e il disegno ne consegna
**uno** — gli altri due sono la riserva di due **concessioni permanenti**, perché *«la sottrazione
non è un'esenzione»* (gotcha #4) e metterli in `Parameters` darebbe due campi che nessuna decisione
del kernel legge; e la contro-sonda della riga `Q8` nomina una **proiezione di presentazione** che
non esiste, quindi a leggere `cold_start` sarà una **finta**.
📌 **Baseline prima di cominciare:** `GATE GREEN`, **32 target, 194 passati, 0 falliti, 2
ignorati**, albero pulito.

✅ **E LO STESSO GIORNO IL PIANO DEL TRAGUARDO 5 È SCRITTO** —
[`plans/2026-08-18-…-traguardo-5-arbitro-gpu.md`](superpowers/plans/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu.md),
**tredici compiti in cinque parti, 174 KB**. Il vocabolario della risorsa · il ciclo della
concessione · le due policy · il cablaggio e ciò che il `Grant` sblocca · la campagna e la
chiusura.
⛔ **E il pre-controllo del PIANO ha trovato SETTE cose leggendo il disegno contro il codice —
gotcha #58, che vale per un disegno come per un compito.** Le due che contano di più non sono
divergenze di merito ma **omissioni di dichiarazione**: il disegno cambia i **due** campi
`preemptible`/`release_grace` della §5.2 in **uno** (`Preemption::Never | After(Millis)`) e
**non lo dichiara**, mentre dichiara la divergenza gemella sulla §5.1; e `Admission` **non può
derivare `Debug` né `PartialEq`**, perché `Grant` non li ha e non deve averli — quindi ogni
sonda dell'arbitro confronta con `matches!` e `let … else` invece che con `assert_eq!`, che è
una conseguenza sul banco che nessuna rilettura del disegno mostra.
⛔ **E una la eredita il Traguardo 6, dichiarata perché non la riscopra:** `Process::start`
**consuma** il `Grant`, e `Arbiter::release` lo consumerà pure — quindi chi avvia un worker
non ha più nulla da rilasciare. La via naturale è che **`Worker::kill` restituisca la
concessione**, e non si costruisce ora perché quel chiamante non esiste (gotcha **#46** dal
verso sbagliato).
⚠️ **Tre voci sono per il proprietario, registrate e non prese:** `WorkDescriptor` dista **una
lettera** da `WorkerDescriptor`, che esiste già in `ports/process.rs` ed è un'altra cosa — la
collisione `record`/`boundary` del Traguardo 3 riscrisse **due oracoli**; la riga di catalogo
`Q2 · §5.1` è **una** e formulata in **una direzione**, mentre il disegno la vuole tenuta da
**due** regole e il piano ne scrive **quattro** casi; e due celle del catalogo — quella di
`V4` e quella di `I2 · §5.3` — nominano **identificatori italiani** (`Concessa`, `InCoda`,
`InRevoca`) che dal Task 4 esistono in inglese, quindi diventano riferimenti al codice che la
§1.0 vieta. ⛔ **Tutte e tre toccano la §7.4 o la §5, che sono SPEC**: vincolo globale 7.
📌 **Otto decisioni prese dal piano**, e la prima governa le altre: `Arbiter::new` prende
**`Parameters`** e non un `Mib` nudo, che è la forma di `Executor::new` e la riga di catalogo
`V29 · §2.8 · ADR-0034`. Le altre sette stanno in testa al piano.

⛔ **Cosa l'audit ha trovato, e la prima voce è la più grave** — ✅ **chiusa il 2026-08-17, vedi il
riquadro qui sopra.** La suite di conformità provava
**V6 solo su un archivio VUOTO**: sostituendo le guardie di `FileJournal::outcome` e `::note`
con *«l'archivio è vuoto?»* invece di *«questo passo ha un intento?»*, `cargo test --workspace`
dà **32 target, 171 passati, ZERO falliti** — e la mutazione **è osservabile**, perché su un
archivio non vuoto accetta un esito e una nota per passi **mai aperti**. È il gotcha **#63**.
⛔ **E due buchi del cancello lo lasciavano verde col confine caduto**, entrambi corretti e
provati coi codici d'uscita veri: `build = 'gen.rs'` fra apici singoli sfuggiva al controllo dei
build script (**#61**), e `check-docs.sh` non verificava che la spec **esistesse**, con le sei
asserzioni di §8.6.1 che vivono in blocchi `END` di `awk` — e `END` non gira (**#60**).
⛔ **Tre documenti di stato dicevano che il Traguardo 4 era da fare**, eseguito da un commit.
📌 **Sei gotcha nuovi, #59–#64**, e il più utile è il **#59**: un ADR può essere falsificato da un
ADR **fratello della stessa data**, e nessuna delle quattro domande del pre-controllo lo coglie
perché guardano tutte il compito contro il **codice**, mai un ADR contro i **fratelli**.
✅ **E ciò che l'audit ha trovato SANO va detto**: le due campagne DST sono solide — la quarta
occorrenza del difetto di vacuità **non c'è** — nessun segreto in centosettantuno commit, il
grafo spedito è esattamente quello dichiarato, e i **quattordici** conteggi di test di
[`porta-di-qualita.md`](porta-di-qualita.md) sono giusti **tutti e quattordici**.
⛔ **E il disegno ha ricevuto un richiamo PRIMA che il piano fosse scritto, perché il codice lo ha
smentito su due punti** — §11 del disegno: `CrashingBackend` vive in un **banco di prova** di
`platform` e non in `src/`, perché ciò che il Task 8 comprò è che il confine sia raggiungibile **da
fuori la crate**, e il precedente di `abandon_without_commit` **non trasferisce** (quel metodo è
`pub` perché *non* è scrivibile da fuori); e non può avvolgere `redb::InMemoryBackend`, che tiene i
guardiani **privati**, quindi l'archivio **non si riaprirebbe** — e riaprirlo è l'intera domanda del
livello 2. ⚠️ **La prima risposta era già scritta in un commento** di
`crates/platform/tests/file_journal.rs`; la seconda si vedeva solo leggendo la libreria. Nessuna
delle due si vedeva rileggendo il disegno, che era coerente con sé stesso.

⛔ **Cosa ha deciso il disegno, e la prima decisione governa le altre: il Traguardo 4 costruisce
il MOTORE della DST, non tutte le finte della §3.1.** Il fatto che decide è un conteggio, ed è
stato ottenuto leggendo la §3.3 contro il codice di **oggi**: delle **dieci** righe di guasto,
**una sola** ha il proprio soggetto — la caduta fra intento ed esito sulla porta `journal`,
la cui riconciliazione esiste dal Traguardo 3. Le altre nove iniettano un guasto dentro un
meccanismo che **non esiste**: l'arbitro è il Traguardo 5, il canale worker e lo stato di
degrado il Traguardo 6. Costruirle ora è la decisione **D1 del piano del Traguardo 3 al
rovescio**, più il gotcha **#46**.
⚠️ **E il Traguardo 4 non porta il determinismo — quello c'è dal Traguardo 2: porta il guasto.**
**C1, C2, C3** e la **non-vacuità** sono già test permanenti in
`crates/kernel/tests/executor_determinism.rs`; mancano **C7a** e **C7b**, il giornale cadente,
il backend cadente, la campagna e l'elenco dei semi. ⚠️ Un caso è a metà e la distinzione conta:
`a_crash_leaves_more_than_one_step_in_doubt` tiene la **proprietà** su uno stato costruito **a
mano**, non su uno spazio di semi.
⛔ **E i due livelli di crash sono DUE CAMPAGNE con soggetti diversi** — livello 1 esamina la
riconciliazione del kernel, livello 2 la coerenza dopo crash di `redb` — che è la ragione per cui
ADR-0032 collocava male il backend cadente. Le nove righe scoperte hanno **ciascuna il proprio
indirizzo** nella §7 del disegno: un arretrato con un indirizzo è uno scaglionamento.
⚠️ **Questa riga diceva «il prossimo passo è il brainstorming»**, e prima ancora «il Task 12»:
è la riga che invecchia per costruzione, e si riscrive **quando il passo si chiude**, non dopo.
⛔ **E il Task 11 ha lasciato due voci aperte MISURATE, non supposte, in
[`porta-di-qualita.md`](porta-di-qualita.md).** (1) **ADR-0018 pretende che un payload potato e
uno mai registrato non siano indistinguibili, ed entrambe le implementazioni lo violano**: dopo
la potatura rispondono lo stesso `Err(Missing)`, spariscono entrambe da `replay`, e una seconda
`prune` non le distingue. ⚠️ **La via che sembrava non costare l'impronta è stata cercata e la
misura la uccide:** svuotare il payload lasciando la voce rende i due distinguibili e non costa
nessuna promessa, ma `steps_in_doubt` risponde allora **`SuspendAndAsk`** su un passo riconciliato
e potato — byte vuoti sono indecifrabili, e un record indecifrabile rimette il passo in dubbio —
quindi il sistema si fermerebbe su **ogni** passo potato, a **ogni** ripresa. Una traccia utile
dev'essere **leggibile dalla riconciliazione**, cioè una decisione di **formato**: appartiene al
traguardo della ritenzione, **insieme** alla decisione sull'impronta (una voce nuova in ADR-0031).
(2) `prune` ha **tre** risposte e la conformità ne tiene **due**: `Missing` per un passo mai
scritto è tenuta solo dal doppio in memoria. ⛔ **E `JournalError` ha una QUARTA variante,
`StepInDoubt`, che è un'aggiunta al contratto di una porta condivisa:** non allarga `OutOfOrder`
perché quello è definito da **V6** — *«tutte e tre le sue vie sono V6»*, dice il suo doc — e potare
troppo presto viola **ADR-0018**, un invariante diverso; e perché il chiamante deve distinguere
*«hai rotto il protocollo»* (un difetto) da *«questo passo non è ancora potabile»* (una spazzata
di ritenzione ordinaria). ⛔ **`FileJournal` ha dovuto cambiare il proprio archivio per rispondere**
— la tabella `redb` guadagna il **byte dell'operazione** — perché contare i record è sbagliato:
una **nota** non è un esito.
⛔ **E le tre scadenze di E33 sono scattate col Task 10:** da quel commit nessun campo nuovo di
`RecordV1` può essere obbligatorio — dev'essere `Option` con `#[cbor(default)]` e un indice
nuovo — nessun indice esistente può cambiare significato, e una variante nuova dei tre enum
smette di essere gratis. Stanno scritte accanto al codice, in testa a
`crates/kernel/tests/frozen_bytes.rs` e in `crates/kernel/src/record.rs`.
⚠️ **Questa riga diceva *«il prossimo passo è il Task 7»* fino al 2026-08-10**,
mentre il Task 7 era eseguito da un commit e la riga gemella trenta righe più sotto diceva già
**Task 8**: due frasi di stato **nella stessa sezione** che si contraddicevano, e il gotcha
**#31** nella forma che costa di più — chi legge la §6 dall'alto trova la prima. ✅ **La misura
del Task 8 diceva che il Task 9 sarebbe stato una formalità, e lo è stata a metà:** la suite
girava già verde contro `FileJournal`, e infatti ha compilato e passato al primo colpo — ma la
**fabbrica** dettata dal piano non reggeva, perché cancellava un percorso fisso in una cartella
condivisa (gotcha **#52**) contro un'implementazione che tiene un **lucchetto** e una suite che
la chiama **nove volte**. Il valore del compito sta nelle **contro-sonde**, non nel verde. Il
Traguardo 1 ha lasciato questo:

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
5. ~~**Il codice del Traguardo 2**~~ — ✅ **eseguito il 2026-08-10**, `GATE GREEN`. **Tutti e quattordici i compiti** subagent-driven, fra il 2026-08-09 e il 2026-08-10, con `GATE GREEN` a ogni compito; alla chiusura `cargo test --workspace` dà **72 target verdi e zero fallimenti**, e dentro il banco `compile_fail` esegue **quattordici** casi via `trybuild`. ✅ **Col Task 12 le sei famiglie di porte sono complete**: `reactor` · `journal` · `filesystem` · `network` · `process` · `ipc`, e §3.1 le dichiara esaustive. ⛔ **E il Task 13 ha scoperto di essere già eseguito**: dettava di *aggiungere* al registro [`porta-di-qualita.md`](porta-di-qualita.md) righe che i Task 1–12 vi avevano già scritto a ogni passo, e a mancare non era l'aggiunta ma il **riconteggio** — gotcha **#49**. ✅ **E la voce che restava aperta è chiusa il 2026-08-09:** il caso `no_conversion_from_untrusted_to_instruction` del Task 9 — misurato **portante** — ha ora la propria riga nel **catalogo §7.4.1 blocco C**, la **regola B** della coppia `Untrusted`/`Instruction` accanto alla regola A (`Q9 · I6 · V20`), con richiamo datato. ⛔ **Non era una rifinitura, ed è il punto da ricordare:** la regola A è **cieca** proprio a quella via — con `impl From<Untrusted> for Instruction` presente il caso resta `ok` invece di dare il `mismatch` che il gotcha #42 prevede, perché lì lo scarto è fra **riferimenti** — quindi senza la riga B la porta resta verde **col confine già caduto**. Il blocco C passa da diciassette a **diciotto** righe, e §7.4.7, §8.3 (`Q9` e `Q15`) e [`porta-di-qualita.md`](porta-di-qualita.md) sono riallineate nello stesso passaggio.
6. ~~**Il piano del Traguardo 3**~~ — ✅ **scritto il 2026-08-10**: [Traguardo 3](superpowers/plans/2026-08-10-sottoprogetto-1-traguardo-3-giornale-e-formato-durevole.md), **dodici compiti in due parti**. ⛔ **Sette decisioni prese dal piano, e la prima governa tutte le altre:** i **byte congelati sono l'ultimo compito**, non il primo. Il nome del traguardo li mette in vetrina e la tentazione è produrli subito, ma non si rigenerano mai — congelarli prima che un consumatore vero e **due** implementazioni abbiano esercitato il formato significherebbe congelare la forma sbagliata, che è il difetto del Task 11 del Traguardo 2 nella sua forma più cara. ⚠️ **Due voci aperte del codice trovano risposta**: una promozione **non è un passo proprio** (ADR-0007 fissa la granularità), e l'**etichetta di fiducia entra nel record** perché è l'unico campo che, se manca, porta informazione che **si perde per sempre**. ⛔ **E la porta guadagna `replay()`**, perché `read_back` chiede un passo **per nome** e dopo un crash il kernel non sa i nomi: non era una decisione presa, era una **lacuna** — ADR-0007 dice *«per ogni passo in dubbio»* senza dire come si scoprono.
7. ~~**Il codice del Traguardo 3**~~ — ✅ **eseguito il 2026-08-10**, subagent-driven, un compito per volta con revisione fra uno e l'altro. **Dodici compiti su dodici**, `GATE GREEN` a tutti. ⚠️ **Ricontati una QUARTA volta il 2026-08-10, chiudendo il traguardo:** questa riga diceva *«undici»*, e la cifra è invecchiata **quattro volte su quattro** — è la riga più stantia del file per costruzione, e il rimedio scritto sotto (*«si riconta prima di appendere il capoverso»*) non ha retto nemmeno una volta. ⚠️ **Ricontati una terza volta il 2026-08-10, col Task 10**, che è la terza volta di seguito che questa cifra viene appesa senza essere toccata — gotcha **#31** sul numeratore, e a questo punto è la riga più stantia del file per costruzione: si riconta **prima** di appendere il capoverso, non dopo. ⚠️ **Ricontati una seconda volta il 2026-08-10, col Task 9:** questa riga diceva *«sette compiti»* mentre il capoverso del **Task 8** le era già stato appeso sotto senza toccare la cifra — la **stessa** forma del difetto che la nota qui accanto descrive, ripetuta dallo stesso file due paragrafi dopo averla dichiarata. Una cifra dentro una frase che resta vera è il gotcha **#31**. ⚠️ **Ricontati il 2026-08-10:** questa riga diceva *«tre compiti»* e vi era stato **appeso** il capoverso sui Task 4 e 5 senza toccare la cifra — la §6 dello stesso file era già stata portata a cinque, quindi il documento si contraddiceva **dentro sé stesso**, che è la forma peggiore. I primi tre: il **record durevole** — enum di versione `Record::V1`, quattro campi con indice esplicito, `Debug` scritto a mano che non stampa il payload — la **riga di catalogo dell'etichetta di fiducia** (`Q9 · I6 · V20 · §4.9`) col caso `record_without_trust_label.rs`, e il **doppio in memoria del giornale** con la variante `JournalError::OutOfOrder`, che tiene V6 sulla **porta** invece che sulla diligenza del chiamante. ⛔ **Un'errata in testa al piano, sette voci dal solo Task 1**, e i costi misurati stanno in [`riferimenti.md`](riferimenti.md): la collisione `record`/`boundary` che ha riscritto **due oracoli pre-esistenti** è permanente. ⛔ **E il Task 3 ha trovato la lacuna di specie 2 più netta del progetto**: il criterio di chiusura che il piano dettava — `4 passed` — è soddisfatto da un giornale che **non registra nessun esito**.
   ✅ **Task 4 e 5 eseguiti il 2026-08-10, e come un compito solo**, perché separati non funzionano: il Task 4 scrive una suite che chiama `replay()`, che la porta guadagna solo al Task 5, e il suo commit lascerebbe la **porta rossa** contro il vincolo globale 8; per giunta il `git add` del Task 5 non nominava il file del Task 4, che così non sarebbe **mai** entrato nel repository. Ne sono usciti **sette bugiardi e non tre**, uno per promessa — sei eseguendo, il settimo dalla decisione presa in revisione: la suite muore alla **prima** promessa violata, quindi con tre bugiardi **due promesse non si vedevano mai fallire** — gotcha **#14**. ⛔ **E due promesse dettate erano vacue contro il proprio bugiardo, misurate e non rilette:** quella sull'ordine di `replay` confrontava le **sole identità** dei passi e la sequenza `1, 2, 1` **è un palindromo**, così `ShuffledJournal` passava la suite intera; e la promessa 1 rileggeva con un `expect` che ingoiava proprio la via **A6**, l'unica cosa per cui la suite esiste. ⚠️ **Due promesse in più che nessun compito chiedeva:** dopo intento **ed** esito, `read_back` deve rendere ancora l'**intento** — senza, al Task 8 una tabella `redb` chiavata sul passo risponde l'esito e **nulla diventa rosso** — e, **decisa dal coordinatore in revisione**, un **secondo `intent`** sullo stesso passo è **rifiutato**: la promessa 2 costringe già `redb` a chiavare più fine del passo, ma è un accordo **per accidente del disegno della chiave** e non per contratto. ✅ **Task 6 eseguito il 2026-08-10**, la **riconciliazione** — e in due commit, perché un passo preliminare toglie a `Record::encode` un `Result` che non poteva essere `Err` (**E22**, decisione del coordinatore). ⛔ **Tre difetti del compito dettato, trovati col pre-controllo e tutti misurati prima di decidere:** la §1.0 violata una **terza** volta nei commenti dettati (**E23**) · lo stesso passo che compariva **due volte** nell'insieme, con **due** produttori e non uno (**E24**) · e le **due verità indipendenti** su «intento o esito», che è la domanda riportata al proprietario (**E25**). ⚠️ **Quattro sonde che nessun compito chiedeva:** il giornale **vuoto**, l'**ordine** — che l'unica sonda dettata teneva per accidente, seconda forma del palindromo di E12 — e le due dell'insieme. ✅ **Task 7 eseguito il 2026-08-10** — `Untrusted::promote` è il **primo codice del kernel che scrive un record vero**, e il compito dettato aveva **due conflitti di formato**, misurati prima di decidere. ⛔ **Primo: la promozione non è un secondo intento, è una TERZA COSA.** Il `promote` dettato scriveva con `intent()` sul passo del chiamante, che da **E19** lo rifiuta — e il test dettato non lo vedeva perché poggiava su `RecordingJournal`, che è **riga per riga il bugiardo J7** della conformità. ⛔ **E anche a guardia rilassata il disegno resta rotto, per una ragione indipendente che nessuno aveva visto:** un secondo record `Intent` sullo stesso passo fa **sostituire** alla riconciliazione la risoluzione del chiamante — misurato, `Idempotent` torna `SuspendAndAsk`, cioè la promozione **declassa in silenzio un passo che non le appartiene**; e la sonda dettata confrontava le **sole identità**, quindi era cieca proprio a quello (terza occorrenza del palindromo di E12). **Decisione del coordinatore, confermata dal proprietario:** la porta guadagna **`note()`** e il record **`RecordKind::Note`**, insieme — una nota deve pur portare un `kind`, e i due esistenti sono i due difetti; la riconciliazione le dà un **arm vuoto**. Scartate e misurate: rilassare la guardia (non ripara nulla), scrivere con `outcome()` (`steps_in_doubt` risponde `[]`, un dubbio vero sparisce in silenzio) e la variante `Note` trasportata da `outcome()` — la più economica, scartata perché istanzia di proposito la divergenza di **E25**. ⛔ **Secondo: l'etichetta di fiducia era attaccata alla stringa sbagliata.** Il record dettato metteva nel `payload` la **ragione scritta dal chiamante** e la marcava `Trust::Untrusted`: nessun byte esterno entrava, e l'etichetta non era decorativa ma **falsa**, perché il doc di `Trust` dice che riguarda **il payload**. Il record guadagna quindi **`reason` all'indice 4**, e l'indice 3 porta il **contenuto non fidato** — assegnazione **forzata**, perché il `Debug` scritto a mano nasconde **solo l'indice 3**. ✅ **Così la via A4 si chiude, ma a LIVELLO 2 e non "al formato":** la via come `boundary.rs` la scrive passa da **byte grezzi**, non da un `Record`, e nulla impone che ogni scrittura sia un record. Le vie chiuse sono **tre** — A3, A4, A6 — e le **quattro** che restano sono tutte dichiarate **non chiudibili**: ciò che resta non è un arretrato, è il **pavimento**. ⛔ **`RecordingJournal` è stato tolto e non riparato**, e l'audit delle **ventuno** implementazioni di porta fuori da `src/` ne ha trovata una seconda che rompe un contratto — `RefusingReactor` — **che resta**, perché la regola giusta è *una finta può rompere un contratto quando il test parla della rottura*. ⚠️ **Costo contato:** dieci `E0046` (undici dopo il Task 8), una promessa nuova in conformità col proprio bugiardo («valida e poi butta»), e **due nomi di test corretti** invece che lasciati stantii. `cargo test --workspace`: **26 target, 127 test**.
   ✅ **Task 8 eseguito il 2026-08-10** — `redb` e il **backend scritto da noi**, in `platform`. ⛔ **Il piano era stantio sul contratto e lo dichiarava: non detta il codice di `redb`**, perché quando fu scritto l'API 4.1.0 non era verificabile e dettarla a memoria avrebbe prodotto codice *plausibile e falso*. Letta nella cache del registro **prima** di scrivere. ⛔ **La chiave è un PROGRESSIVO DELLA SCRITTURA e non il passo**, ed è tutto il disegno: `redb` è un B-tree **ordinato per chiave**, quindi chiavare sul passo darebbe a `replay` l'ordine **delle identità** invece di quello di scrittura, e un passo terrebbe **un solo record** — l'esito sovrascriverebbe l'intento e cadrebbe anche la promessa 2. ⚠️ **Il prezzo della chiave progressiva, che il piano non nomina: le guardie diventano SCANSIONI** — misurato **~56 ns per record** in release, lineare, con il pavimento della scrittura all'`fsync` (**~1,45 ms**): la scansione lo supera solo **oltre ~26 000 record**, quindi **non è ottimizzata** e il rimedio del giorno in cui morderà è lo stesso **checkpoint** che `replay` dichiara già di volere. ⛔ **Due decisioni che cambiano firme pubbliche:** `StepId::get()` **torna** — senza, la porta **non è implementabile fuori da `kernel`**, e il doc di `CheckpointId` aveva già fissato quel giorno *«con quel chiamante»* — e **`open` NON restituisce `JournalError`** ma un errore proprio, perché nessuna delle tre varianti significa *«non ho potuto aprire il file»* e `open` non è un'operazione della porta. ⛔ **La prova che il confine è reale, che il piano non chiedeva:** un confine dichiarato in anticipo non ha chiamanti per costruzione (gotcha **#46**), quindi una **seconda implementazione di `StorageBackend` scritta da fuori** gira nel banco — ed è servito aggiungere `FileJournal::with_backend`, che il piano non nomina: con la sola `open(path)` il confine sarebbe stato **inesistente in pratica**, il difetto del Task 11 del Traguardo 2. ⛔ **Due gotcha nuovi, entrambi dalle mutazioni o dalla misura:** **#51** — una garanzia sulla **morte del processo** non è osservabile da dentro il processo: `Durability::None` lascia **sei test su sei verdi**, ed è dichiarato accanto al codice invece che scoperto al Traguardo 4 — e **#52** — un difetto di parallelismo **mascherato dal sistema operativo**: la cancellazione della cartella condivisa avviene davvero (**tre volte su sei**) ma Windows rifiuta di cancellare un file aperto, quindi il rosso esce **su Linux**. ⚠️ **E una mutazione ha trovato un difetto vero:** il rimedio alla vacuità era esso stesso vacuo — terza occorrenza del **#45**. `cargo test --workspace --no-fail-fast`: **27 target, 133 test**.
   ✅ **Task 9 eseguito il 2026-08-10** — la conformità gira contro **entrambe** le implementazioni **a ogni commit**, `crates/platform/tests/journal_contract_real.rs`, che raggiunge le asserzioni per `include!` come `reactor_contract_real.rs` fa per `reactor`. ✅ **Ricontate leggendo il sorgente e non fidandosi della misura anticipata del Task 8: le promesse sono OTTO e passano tutte e otto**; il binario di `platform` porta **undici** test, i dieci inclusi più quello vero, e il costo — i bugiardi che girano una seconda volta — è **scritto**, non nascosto. ⚠️ **Sono cifre del Task 9, superate dal Task 11 e ricontate eseguendo il 2026-08-10:** con la promessa **7b** le promesse sono **nove** e i bugiardi **nove**, `kernel --test journal_contract` porta **undici** test e il binario di `platform` **dodici**. ⛔ **Tre difetti nel codice dettato, tutti nella fabbrica, e il compito valeva quelli:** il piano cancellava un **percorso fisso in una cartella condivisa** — su Windows la cancellazione **fallisce in silenzio** a file aperto, quindi la fabbrica riaprirebbe **i dati vecchi** (gotcha **#52**, nato il giorno prima); la fabbrica è chiamata **nove volte** e `FileJournal` tiene un **lucchetto esclusivo**; e `assert_journal_contract` prende **`Fn`** e non `FnMut`, quindi numerare le chiamate richiede un `AtomicU64`. Rimedio: **un file nuovo a ogni chiamata** — un nome mai esistito non può essere sporco — in una cartella **per call site** dal `line!()`, con **prefisso diverso** da quello di `file_journal.rs`, perché un numero di riga è unico dentro **un** file solo e i due binari girano insieme. Provato **otto** volte di seguito sull'intero workspace: **28 target, 144 test**, otto su otto verdi. ⛔ **E lo Step 3 è stato fatto in TRE direzioni invece di una**, perché una sola proverebbe una promessa su otto: `read_back` rotta muore sulla **promessa 1**, la guardia del secondo intento tolta sulla **promessa 6** — dopo aver superato cinque promesse sui propri meriti — `replay` rovesciato sulla **promessa 4**, ciascuna **col proprio messaggio**; e la **mutazione di controllo** non muove nulla. ✅ **I due lati sono separati, misurato:** con `FileJournal` rotta, `kernel --test journal_contract` resta **verde 10 su 10** e dentro `platform` restano verdi i dieci test inclusi. ✅ **Con esso la via A6 di `boundary.rs` passa da «chiusa la strada» a «chiuso l'accordo»**: era dichiarata chiusa *«fino a che la seconda implementazione non c'è»*, e ora c'è ed è tenuta.
   ✅ **Task 10 eseguito il 2026-08-10 — i BYTE CONGELATI, l'unico artefatto del progetto che non si corregge.** ⛔ **Il compito dettato era stantio sul record che doveva congelare, e i difetti erano sei più uno.** Il costruttore dettato ha **quattro** campi dove `RecordV1` ne ha cinque; `.encode().expect("encode")` non compila dal Task 6; ⛔ **entrambe le mutazioni dettate sono INDICI DUPLICATI e non compilano** — `error: duplicate index numbers` — quindi quella che «deve scattare» non si sarebbe **mai vista scattare** sull'oracolo che non si rigenera, e quella che «deve restare verde» sarebbe stata un errore di compilazione che il piano dichiara equivalente ad **ADR-0036 smentito**. ⛔ **E il settimo, che nessuno aveva visto: la mappa dettata sbagliava l'OFFSET, non solo l'arità** — il byte 2 è `81`, l'array a **un elemento** del corpo della variante, e l'array dei campi sta al byte **3**; l'inquadratura è `82 00 81 85` e un record misura **21** byte. Misurato sull'uscita vera invece che dedotto. ⛔ **Il difetto grosso era la COPERTURA:** un record solo fissa **tre** indici di variante su **otto** — `RecordKind` 3, `EffectClass` 3, `Trust` 2, ricontate sul sorgente — e il Task 1 aveva già misurato che ogni altra sonda sopravvive a una rinumerazione simmetrica, quindi cinque indici su otto sarebbero rimasti tenuti da **nulla**. ✅ **Congelati TRE record**, il minimo che li copra tutti, con `kind` ed `effect` a **quadrato latino** perché nessuna coppia di campi si possa scambiare senza muovere almeno un file; **le otto varianti rinumerate una per una, otto rossi su otto**. ✅ **L'additività misurata in DUE direzioni, e la seconda è il gotcha nuovo #54:** un campo facoltativo all'indice libero 5 lascia i byte **identici** con `None` — `minicbor` **tronca** un `None` in coda invece di scrivere `null` — e li porta a 22 con `Some(9)`; senza la seconda misura, «i byte non si sono mossi» sarebbe stato compatibile con un campo che sul filo non arriva mai, e ADR-0036 sarebbe stato «confermato» da una misura vuota. ✅ **La mappa è RILETTA dal banco** — offset e byte devono ricostruire il `.cbor` — così un `<fill in>` non può sopravvivere al commit (gotcha #43), e la colonna di prosa è dichiarata **non verificata** dentro la mappa stessa. ⛔ **Nessun percorso di rigenerazione:** i byte sono stati **scritti a mano** dall'uscita di una sonda usa-e-getta, cancellata nello stesso commit. ⚠️ **E un `.gitattributes` di UNA riga** — mai un `* text=auto`, che normalizzerebbe sorgenti che nessuno ha toccato — verificato con `git check-attr` e col blob dell'indice invece che dato per scontato. `cargo test --workspace --no-fail-fast`: **29 target, 150 test**.
   ✅ **Task 11 eseguito il 2026-08-10** — `prune` rifiuta un passo **in dubbio** e accetta uno riconciliato, su **entrambe** le implementazioni; chiude la voce aperta di **E11**, che aspettava `prune` da sei compiti. ⛔ **Lo Step 1 attendeva un ROSSO e la partenza era VERDE**: `prune` rispondeva `Missing` a tutto, quindi `is_err()` passava e la promessa 7 era soddisfatta **per caso** — a renderla non-vacua è la sola contro-sonda nuova, la **7b**, col bugiardo `AlwaysInDoubtJournal` che rifiuta tutto **con la parola giusta**. ⛔ **Il `prune` dettato viola una regola non negoziabile di ADR-0018 e la misura lo conferma su entrambe:** un payload potato e uno mai registrato sono indistinguibili in **tre** modi; la via che sembrava non costare l'impronta funziona ma fa rispondere `SuspendAndAsk` alla riconciliazione su ogni passo potato, quindi non è economica — dichiarata in due **voci aperte** invece che chiusa a metà. ⛔ **`JournalError` guadagna `StepInDoubt`, quarta variante su un tipo dichiarato «deliberatamente povero»**, e non allarga `OutOfOrder` perché quello è definito da **V6** mentre questa è ADR-0018 — invarianti diversi — e perché il chiamante deve distinguere un difetto da una spazzata ordinaria. ⛔ **`FileJournal` NON POTEVA rispondere alla domanda** e la sua tabella `redb` guadagna il **byte dell'operazione**: contare i record è sbagliato perché una **nota** non è un esito, e decodificare i byte è vietato da ADR-0036. ⛔ **Quattro mutazioni su quindici sopravvissute al primo giro, e tre erano difetti veri:** `prune` che risponde `Ok` e **non pota niente**, `prune` che pota **l'intero giornale**, e una **nota archiviata come esito** che rendeva potabile un passo in dubbio. Chiuse tutte e tre **senza decidere nulla sulla ritenzione**. `cargo test --workspace --no-fail-fast` → **29 target, 152 test**.
   ✅ **Task 12 eseguito il 2026-08-10 — ed era un AUDIT, non una scrittura: il gotcha #49 per la seconda volta, previsto dal compito stesso.** ⛔ **Le quattro righe che lo Step 2 dettava di spostare fra le coperte erano già spostate tutte e quattro**, dai compiti che le avevano prodotte; a mancare non era l'aggiunta ma il **riconteggio**, come al Traguardo 2. ⛔ **E la classe che non si vede leggendo ne ha data una vera:** partendo dall'elenco dei bugiardi invece che dalla colonna, **`J13` non era mai entrato** nella colonna «deve scattare» della riga di catalogo dei test di contratto — esisteva nella tabella delle sonde dal Task 11 e da lì risultava inesistente. ⛔ **Cinque conteggi di test erano stantii nel registro** — `boundary_promotion.rs` otto contro **quindici**, `record_shape.rs` dieci contro **dodici**, `reconciliation.rs` nove contro **undici**, `journal_contract_real.rs` undici contro **dodici**, i casi `compile_fail` quattordici contro **diciassette** — e la campagna della conformità si intestava *«otto promesse, otto bugiardi»* dove sono **nove e nove**: la cifra della passata **non è stata alzata per simmetria**, perché sarebbe stata un'ipotesi, e la **7b** è dichiarata misurata altrove, da `M14b`. ⛔ **E la Definizione di «fatto» era stantia in tre condizioni su dodici** — la 4 dice *«tre bugiardi»*, la 6 *«A4 chiusa al formato»* che **E31** aveva già respinto, la 9 detta due direzioni che **E51** ha misurato **incompilabili** — corrette **nell'errata** e non nel testo, che è il registro di ciò che fu deciso. ✅ **Il confronto con ADR-0036 sulla dimensione del record è stato misurato al posto giusto:** i totali non sono confrontabili, perché la forma del record che l'ADR prezzava **non è scritta da nessuna parte**; ciò che è confrontabile è **quanto costa la busta di versione**, e **converge esattamente** — `+3` byte, `82 00 81`, sia su un record pieno (18 → **21**) sia su uno vuoto (6 → **9**), dove l'ADR misurò `27 → 30`. ⚠️ **Diverge la percentuale, e si registra invece di arrotondarla:** `+11 %` là, **`+17 %`** qui, perché la base è più corta. `bash scripts/gate.sh` → `GATE GREEN`; `cargo test --workspace --no-fail-fast` → **29 target, 152 test**.
   ✅ **Audit di chiusura della sessione, il 2026-08-10 — dopo il Task 12 e distinto da esso.** Il
   Task 12 aveva chiuso il **traguardo**; questa passata ha riletto **tutti e nove** i documenti di
   stato contro il repository, partendo dai numeri invece che dalle frasi. ⛔ **La classe di difetto
   della sessione — la stessa cifra in più posti con valori diversi — ne ha dati altri sei**, e il
   più caro attraversava **tre** file: l'errata del Traguardo 3 diceva *«settanta voci in otto
   passate»* in `HANDOFF.md` (due volte) e in [`roadmap.md`](roadmap.md), col valore giusto —
   **settantasette in nove** — scritto nella §12 di **questo** file. Gli altri: le **promesse** della
   conformità, otto in due punti di questa §6 e nove in un terzo · i **test** dei due binari del
   giornale, undici e dieci · le **funzionalità** di [`tracciabilita.md`](tracciabilita.md),
   centosettanta in §8 contro **centosettantuno** ricontate · il **peso del messaggio** di
   [`AVVIO-CHAT.md`](AVVIO-CHAT.md), *«~4 KB»* contro **6,2** misurati · e i **pilastri** di
   [`README.md`](README.md), che ne elencava cinque dicendo quattro. ⛔ **E tre affermazioni di
   stato erano false in `HANDOFF.md`:** il punto di ripresa nominava il solo **Traguardo 1**, il
   giornale write-ahead *«sale col Traguardo 3»* al futuro, e la mappa dei piani dava il piano del
   Traguardo 3 come **da eseguire**. ⚠️ **Due divergenze sono state REGISTRATE e non appianate,
   perché stanno nel sorgente e questa era una passata documentale:** l'intestazione di
   `crates/platform/tests/journal_contract_real.rs` dice *«le DIECI prove della suite — la finta, il
   vincolo sulle sottostringhe e gli OTTO bugiardi»* dove sono **undici e nove**, e il file gemello
   `crates/kernel/tests/journal_contract.rs` porta le cifre **giuste** dieci righe più su; e le *«72
   target verdi»* attribuite alla chiusura del Traguardo 2 — scritte identiche qui e in `HANDOFF.md`
   — non riconciliano con nessun'altra misura del progetto (**25 target** a quella data, **29**
   oggi), e non sono state riscritte perché rifarle richiederebbe uno stato che non esiste più.
   ⚠️ **Questa riga diceva *«il prossimo è l'ESECUZIONE DELL'AUDIT … e non il Traguardo 5»***,
   ed è corretta il 2026-08-18: l'audit è **chiuso**, e il puntatore vivo sta in cima alla §6. ✅ Il
   brainstorming del Traguardo 4, il disegno, il piano **e la sua esecuzione** si sono chiusi
   tutti il 2026-08-11, e lo stesso giorno il repository ha ricevuto il suo **primo audit
   completo**.
   ⚠️ Questa riga ha detto *«il brainstorming»*, poi *«il piano»*, poi *«l'esecuzione»*, poi il
   Task 2, il 3, il 4 e il 5 **nello stesso giorno**: è la riga che invecchia più in fretta del
   file, e si riscrive **quando il passo si chiude**. ⚠️ **E diceva «una delle TRE in cui il
   prossimo passo vive dentro questa sola sezione»: sono QUATTRO**, ricontate col `grep` invece
   che citate — righe 618, qui, 794 e la riga del Traguardo 4 nella tabella dei sei. È il gotcha
   **#31** nella forma che la **ventesima misura** aveva descritto per i pesi: *le case si contano
   una volta sola, quando si scrive il rimedio*, e chi riconta non si fidi del numero scritto nel
   verbale precedente.
8. ~~**Il brainstorming del Traguardo 4**~~ — ✅ **chiuso il 2026-08-11**, e il disegno è scritto:
   [Traguardo 4 — il disegno](superpowers/specs/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst-design.md).
   ⛔ **Ha trovato una collocazione non eseguibile in un ADR `Accepted`**, e non riaprendo una
   decisione ma misurandola: il backend cadente di [ADR-0032](adr/0032-motore-di-persistenza.md)
   *«vive in `simulator`»*, e `simulator` è `no_std`, si costruisce per `x86_64-unknown-none` e ha
   un grafo spedito a lista chiusa la cui unica cura per un intruso è **togliere la dipendenza**.
   `redb` non ha `no_std`. **Rimando datato**, non `Superseded by`: è sbagliata una **cella**, non
   la decisione — ed è la **seconda** volta per quell'ADR. ⚠️ **La stessa cella viveva anche nella
   §5 di questo file**, ed è stata corretta nello stesso passaggio: la classe di difetto della
   sessione precedente, colta stavolta **prima** di committare.
   ⚠️ **E una lettura affrettata è stata registrata invece che taciuta:** `StorageBackend` ha
   **sei** metodi, non cinque — `close` ha un'implementazione predefinita — e l'ADR li dichiarava
   giusti. L'errore andava **a sfavore** dell'oracolo, non a favore: il conteggio dei punti scattati
   è la non-vacuità della campagna, quindi un metodo in meno sarebbe stato un oracolo più debole
   senza che nulla lo dicesse.
9. ~~**Il piano del Traguardo 4**~~ — ✅ **scritto il 2026-08-11**:
   [Traguardo 4](superpowers/plans/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst.md), **dieci
   compiti in tre parti**. ⛔ **Sette decisioni prese dal piano**, e la seconda è quella che governa
   il resto: la caduta **non si riprende** — un giornale che rifiuta una volta e poi riparte modella
   un disco cattivo, non un crash, e la permanenza è ciò che fa fermare **tutte** le attività
   interlacciate e non solo quella che ha toccato il confine. ⚠️ **E il punto di caduta si estrae da
   un generatore diverso da quello dell'interlacciamento**, con seme derivato: due `SeededRng`
   costruiti dallo stesso numero danno la **stessa** sequenza, quindi la campagna esplorerebbe una
   **diagonale** dello spazio invece dello spazio.
10. ~~**L'esecuzione del Traguardo 4**~~ — ✅ **ESEGUITO il 2026-08-11**, subagent-driven, un
    compito per volta con revisione fra uno e l'altro: **dieci compiti su dieci**, `GATE GREEN` a
    ciascuno; `cargo test --workspace` → **32 target, 171 test** di cui
    **due** ignorati, le due campagne profonde. ⛔ **E il Task 7 ha stabilito cosa compra una
    campagna PROFONDA, misurandolo: non lo spazzamento.** Allargare l'intervallo oltre la
    saturazione dà corse **indistinguibili da corse senza iniezione** — a ottocento punti ne
    scattano sempre trentacinque — mentre **approfondire lo scenario compra stati nuovi uno per
    record**, e la metrica che lo dimostra sono i **pioli**: le lunghezze di prefisso distinte che
    tornano sono `record + 1` a ogni profondità misurata. ⚠️ **E la saturazione non è lineare:** il
    terzo record costa dieci operazioni dove il primo ne costa sei, e il `Drop` ne costa quattordici
    a due record e dodici altrove — dedurla avrebbe sbagliato. ✅ **E il gotcha #51 è chiuso nella
    metà chiudibile**, col perimetro di ciò che **non** compra scritto per esteso — perché
    *«è chiuso»* nella forma nuda mentirebbe. ⛔ **Ma la lezione del Task 6 è un'altra, ed è la
    terza volta che questo traguardo la impara: un ciclo che verifica la COERENZA e un oracolo che
    verifica che il GUASTO SIA SCATTATO possono essere entrambi verdi mentre la proprietà per cui
    esistono è sparita.** Misurato: togliendo la durabilità, il ciclo di coerenza è **interamente
    verde**; a renderlo un secondo testimone è un'asserzione sui **gradini** — che esistano punti in
    cui torna *qualcosa ma non tutto* — perché senza durabilità la scala collassa a **zero-o-tutto**.
    📌 Serve un controllo che guardi **la forma di ciò che sopravvive**, non solo che qualcosa sia
    sopravvissuto. ⛔ **E il Task 5 ha trovato, prima
    che il Task 6 fosse scritto, che l'oracolo destinato a chiudere il gotcha #51 NON FUNZIONA.**
    Il piano voleva contare le chiamate a `sync_data`: ma **sei sync su sette nascono prima che
    esista un record** — `create_with_backend` nudo ne fa sei — quindi il conteggio è dominato
    dall'apertura ed è **cieco** proprio alla perdita per cui esisterebbe. Misurato: con la
    durabilità tolta, la sonda in quella forma resta **verde**. 📌 **La forma generale: un
    contatore che parte da un valore che il soggetto sotto esame non ha prodotto non è un oracolo
    su quel soggetto** — la cura è il **delta**, non il totale. ⛔ **E lo scenario di livello 2
    SATURA a cinquantotto operazioni**, quindi la campagna profonda che il Task 7 detta
    costerebbe **venti volte tanto esplorando zero stati in più**. ⛔ **E il Task 4 ha stabilito come si sceglie il numero
    di semi, che è la decisione che il vincolo 7 della §11 chiedeva: NON massimizzando.** *«Il più
    grande multiplo di cento sotto il tetto»* insegue una cifra che **satura** — i ventiquattro
    punti di caduta sono coperti già a duecento semi e l'insieme in dubbio massimo vale **tre**,
    che è il suo tetto **strutturale**. Il criterio vero è la **chiusura dello spazio degli
    esiti**: gli insiemi in dubbio distinti sono **centonove**, l'ultimo compare al seme **1038**,
    e ventimila semi in più non ne producono altri. Scelti **duemila**, che costano l'**11 %** del
    tetto. ⛔ **E la guardia su quel criterio è stata adottata solo dopo aver misurato che non
    scattasse dove non deve:** sei costanti di mescolamento diverse danno **centonove tutte e
    sei**, quindi il conteggio è proprietà dello **scenario** e non dei semi che lo campionano.
    ⚠️ **Due misure del coordinatore sono state smentite da chi eseguiva:** il modello di costo
    `2 × semi × costo` sbagliava del **37 %** — una corsa che cade si ferma al proprio punto,
    `C7a` arriva in fondo — e il tetto vero cade a ~19 000 semi. ⛔ **E il Task 3 — `C7b` — ha
    trovato il difetto più istruttivo del traguardo: la non-vacuità che una campagna dichiara non
    è quella che le serve.** *«L'iniezione è avvenuta»* e *«c'era qualcosa da verificare»* sono
    **due** affermazioni, e il piano ne teneva una sola: con un giornale che cade al primo byte,
    duecento semi su duecento cadono davvero **e duecento confronti su duecento sono
    `[] == []`** — verde, e nulla verificato. ⚠️ **Ed era il difetto che il Task 2 aveva chiuso per `C7a` un
    compito prima, reimportato**: chiuderlo in un posto non lo chiude nell'altro. Ora `C7b` ha
    **due** oracoli, e il discrimine fra loro è **provato** — le mutazioni che svuotano la
    campagna sparano sul secondo, quelle che spengono il guasto sul primo. ⛔ **Il Task 2 — lo scenario giornalato e `C7a` — ha trovato che `C7a` era verde
    su un archivio VUOTO:** *«nessun passo è in dubbio»* e *«lo scenario non ha scritto niente»*
    erano lo stesso verde, misurato con un giornale che cade alla scrittura zero, e il piano vi
    rispondeva con una mutazione **una tantum** mentre allo stesso buco su `C7b` dà un oracolo
    **permanente**. ⛔ **E una regola nuova, uscita da una mutazione che nessuno aveva chiesto:**
    quando due mutazioni uccidono la **stessa** asserzione, prima di concludere che la sonda non
    distingue i due difetti (gotcha **#55**) si cerca **una terza mutazione che lasci passare la
    prima asserzione** — se esiste, le due non erano in competizione ma su assi diversi.
    ✅ **E la decisione di NON fissare qui l'interlacciamento è stata verificata in negativo**,
    costruendo il controfattuale sequenziale: il massimo insieme in dubbio scende da **tre** a
    **uno**, quindi la sonda del Task 3 va davvero rossa e una in più qui sarebbe stata
    duplicazione (gotcha **#49**). Il Task 1:
    `CrashingJournal` in `crates/simulator/src/journal.rs`, **dieci** sonde in
    `crates/simulator/tests/crashing_journal.rs`, **cinque mutazioni uccise su cinque**.
    ⛔ **Il pre-controllo ha trovato un difetto vero, ed è la specie che non si vede leggendo:**
    il doc di `may_write` dichiara che il contatore si muove **solo su un `Ok`**, e nessuna delle
    otto sonde dettate faceva mai fallire una scrittura interna — la mutazione corrispondente
    **sopravviveva a tutte e otto**. ⛔ **E la revisione ne ha trovato un secondo, peggiore:
    `prune` mutava l'archivio DOPO la caduta**, unica operazione mutante fuori dalla guardia,
    mentre il doc prometteva *«ogni scrittura successiva è rifiutata»* e il limite dichiarato
    nominava le **sole letture**. 📌 La classe vale oltre il caso — è il gotcha **#29** spostato
    su un limite dichiarato: **una partizione scritta in un doc lascia scoperto il membro che non
    appartiene a nessuna delle due categorie**, e nulla lo segnala perché il doc *sembra*
    esaustivo. Le otto voci d'errata stanno in testa al piano; le misure in
    [`riferimenti.md`](riferimenti.md), la campagna in [`porta-di-qualita.md`](porta-di-qualita.md).
    ⚠️ **Questa riga diceva *«il prossimo è il Task 2»* a traguardo CHIUSO da dieci compiti**, ed è
    corretta il 2026-08-17 invece che cancellata: era il puntatore di avanzamento **dentro** il
    racconto del Traguardo 4, e nessuno l'ha spento quando il racconto ha smesso di essere in
    corso. Il puntatore vivo sta **in cima alla §6**, in un posto solo — gotcha **#31** nella forma
    che la §6 stessa dichiara di temere: *la riga che invecchia per costruzione*.
11. ~~**Il brainstorming del Traguardo 5**~~ — ✅ **chiuso il 2026-08-18**, e il disegno è scritto:
    [Traguardo 5 — il disegno](superpowers/specs/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu-design.md),
    **dodici sezioni**. Il racconto per esteso sta nel riquadro in cima alla §6; qui basta ciò che
    governa il piano: il perimetro è **l'arbitro intero**, le due proprietà «impossibili» sono
    **due metà** e la loro metà d'arbitro è **una sola**, `Grant` **si sposta** nel modulo
    dell'arbitro perché un fratello non può costruirlo (`E0423`, misurato), e le righe di catalogo
    che il traguardo tocca sono **dodici già scritte** e nessuna nuova.
12. ~~**Il piano del Traguardo 5**~~ — ✅ **scritto il 2026-08-18**:
    [Traguardo 5](superpowers/plans/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu.md),
    **tredici compiti in cinque parti**. ⛔ **Otto decisioni prese dal piano, e la prima governa
    le altre:** `Arbiter::new` prende **`Parameters`**, non un `Mib` nudo. Le altre: `Grant`
    porta un identificatore privato e `release` lo **consuma** · `release` risponde
    `Result<Mib, ReleaseError>`, e l'`Err` **è raggiungibile** — una concessione rilasciata
    sull'arbitro sbagliato · `ComputeClass` implementa `Ord` **a mano**, da una chiave esplicita ·
    la riscossione delle scadute è **privata** e gira in testa a ogni operazione · la transizione
    di policy prende il giornale **per riferimento** · il modulo è una **cartella con tre file** ·
    `Grant` **non** è ri-esportato da `ports::process`. ⚠️ **Il racconto del pre-controllo — le
    sette voci — sta nel riquadro in cima alla §6.**

⛔ **E quattro questioni restano aperte nel sorgente, dichiarate e non risolte.** Nessuna delle
quattro è un difetto oggi, ed è scritto **perché**; tutte si pagano più avanti, e chi riprende
deve saperle **prima** di scrivere:

| | |
|---|---|
| **la porta `network` ha una tensione non conciliata** | la firma di `request` è **sincrona e bloccante** — restituisce la risposta intera, quindi qualcuno ha atteso — mentre la regola scritta sotto dice che la prontezza viene dal `reactor` e che **nulla attende dentro `network`**; e l'esecutore di questo kernel è **cooperativo**. Le due si conciliano **il giorno in cui la porta riceve un'implementazione**: finché non ha chiamanti nulla blocca, quindi non è ancora un difetto. La domanda è scritta **aperta** in `crates/kernel/src/ports/network.rs` |
| **il residuo dichiarato su `Untrusted::promote`** | il meccanismo compra **una cosa sola**: che la conversione non si scriva senza **nominare** la porta `journal`. **Non** compra che qualcosa sia stato registrato — `promote` è generico su qualunque `Journal`. Sono **sette** le vie che aggirano il confine e **compilano**, tutte elencate in `crates/kernel/src/boundary.rs`. ✅ **Ricontate il 2026-08-10 col Task 7, e sulle voci invece che dedotte:** ne sono chiuse **tre** — il `Debug` di `Untrusted` (**A3**, livello 1), la conformità `journal_contract.rs` (**A6**, livello 2) e ora l'**etichetta di fiducia nel record** (**A4**, livello 2). ⛔ **A4 è chiusa a livello 2 e non «al formato», ed è la differenza che conta:** la via come `boundary.rs` la scrive passa da **byte grezzi**, non da un `Record` — `Record::decode` risponde `Malformed` e l'andata-e-ritorno funziona lo stesso, perché la porta scambia byte. L'etichetta chiude la via **per chi passa dal formato**, e nulla oggi impone che ogni scrittura sul giornale sia un record. ⛔ **A6 restava chiusa a metà finché la suite non girava contro DUE implementazioni.** ⚠️ **Ricontata il 2026-08-10 col Task 8:** la seconda **esiste** — `platform::journal::FileJournal` — e la suite le gira contro **verde, otto promesse su otto**, misurato; ma il file che la esegue **dentro** il repository era il **Task 9**, e *«misurato una volta»* non è *«tenuto a ogni commit»*. ✅ **Ricontata una terza volta il 2026-08-10, col Task 9: quella metà è CHIUSA.** `crates/platform/tests/journal_contract_real.rs` tiene le due implementazioni alle stesse promesse **a ogni commit**, con tre contro-sonde su tre promesse diverse e una mutazione di controllo. ⚠️ **Questa riga diceva «otto promesse», ed erano otto al Task 9:** il Task 11 ha portato la **7b**, quindi sono **nove promesse in dieci blocchi e nove bugiardi**, ricontati eseguendo il 2026-08-10 — gotcha **#31** su un numero che è cresciuto sotto la frase che lo conteneva. ✅ **Ricontati una QUARTA volta il 2026-08-17, chiudendo T-1 e T-2: i bugiardi sono DODICI, le promesse restano nove.** Ed è la notizia per A6: la suite non è cresciuta di promesse, è cresciuta di **stati** — su tre di quelle nove girava solo dove ogni guardia plausibile passa. A6 resta comunque una regola di **livello 2** — nulla impedisce di scrivere un `Journal` che la suite non incontri mai. ✅ **E le quattro vie che restano — A1, A2, A5, A7 — sono TUTTE dichiarate non chiudibili**, quindi ciò che resta non è un arretrato ma il **pavimento**: è la notizia vera di questo riconteggio |
| **la tesi della porta `process` la tiene l'implementazione, non il compilatore** | *«ogni byte che risale è coperto da una ricevuta»* è la frase su cui la porta è costruita — ma `SingleReceipt::new` e `StreamReceipt::new` sono **`pub`, e devono esserlo**: chi implementa `Worker` è `platform`, cioè un'altra crate, e Rust non ha una visibilità che arrivi fin lì e non oltre. Quindi **una ricevuta si può forgiare**, ed è la ragione per cui `close` deve poter rispondere `UnsolicitedFrame` pur andando core→worker. Il limite è scritto accanto ai due costruttori in `crates/kernel/src/ports/process.rs`. ⛔ **Il contrasto è con `Grant`**, che il costruttore non ce l'ha e la cui garanzia è davvero del compilatore. A chiudere la differenza sarà la **suite di conformità** della porta, che pretende due implementazioni: **Traguardo 6** |
| **`Ipc::accept` non ha un canale d'errore, e il prezzo di dargliene uno è la firma** | `accept` restituisce `Option<ClientId>` e **non può fallire**: nessuna delle due varianti di `IpcError` lo raggiunge — `Disconnected` è un'affermazione **su un `ClientId`**, e `accept` è l'unico metodo che un `ClientId` non lo prende. Corretto oggi, e «nessuno in attesa» è lo stato **ordinario**: la gui è 0..1 e sacrificabile. ⛔ **Ma un _ascoltatore_ rotto — che non è un client — arriverebbe come `None`, cioè un valore sbagliato invece di un errore** (gotcha #30). ⛔ **E il prezzo di chiuderlo va detto giusto, perché la prima stesura lo sbagliava:** aggiungere una terza variante **non basterebbe**, non c'è dove restituirla — costa la **firma**, `Result<Option<ClientId>, IpcError>`, che è la forma che `receive` già usa. Oggi la firma resta perché un `Result` che non può mai essere `Err` è superficie morta. Dichiarato in `crates/kernel/src/ports/ipc.rs`, in testa al file come in `network.rs` |

⛔ **E ce n'era una sesta, che questa tabella non ha mai elencato — chiusa il 2026-08-10, ed è
il gotcha #40 su una questione invece che su una decisione.** `Record::encode` restituiva
`Result<Vec<u8>, RecordError>` con la questione dichiarata **solo accanto alla funzione**, in
`crates/kernel/src/record.rs`: quell'`Err` era **irraggiungibile**, misurato al Task 1 sui tipi —
`Vec<u8>` come `Write` di `minicbor` ha `Error = Infallible`, e le altre due strade (`Message`,
`Custom`) hanno due soli produttori nella 2.3.0, `SystemTime` e un `Path` non-UTF-8, nessuno dei
due nel grafo di questo tipo. ⛔ **Decisione del coordinatore, non del piano: la firma è
`pub fn encode(&self) -> Vec<u8>`**, ed è la stessa posizione già presa per `Ipc::accept` —
*«un `Result` che non può mai essere `Err` è superficie morta»*. Pesano due cose che
`Ipc::accept` non ha: al **Task 7** `promote` lo chiama, e un `.expect` che non può sparare
**dentro il confine dei dati non fidati** è debito; e i chiamanti sono **pochi** oggi e molti
dopo. ⚠️ **Questa riga diceva «due», e il numero era sbagliato:** contati invece che ricordati,
i file chiamanti erano **uno** — `crates/kernel/tests/record_shape.rs`, con **nove** siti — e il
secondo che era stato contato, `compile_fail/record_without_version.rs`, **non è un chiamante**:
nomina `RecordV1::encode`, e la ragione per cui quel caso esiste è che quel metodo inerente
**non c'è**. ✅ L'errore va **a favore** dell'argomento, non contro.
`RecordError` **resta**, ristretto a `decode`. ⚠️ Registrata come **E22** nell'errata del piano,
perché il proprietario possa ribaltarla vedendola. ⚠️ **E il difetto vero non è la firma: è che
una questione aperta nel sorgente non compariva qui**, quindi per chi legge non esisteva — la
tabella qui sopra si popola a mano, come le decisioni.

⛔ **E una quinta questione è aperta fuori dal sorgente, trovata il 2026-08-10 e non decisa.**
[`porta-di-qualita.md`](porta-di-qualita.md) **non è sorvegliato** dalla guardia dei conteggi di
`check-docs.sh`: la lista dei documenti che quella guardia legge è fissa — `HANDOFF.md`,
`roadmap.md`, `README.md`, questo file, [`AVVIO-CHAT.md`](AVVIO-CHAT.md) e `CLAUDE.md` — e il
registro non c'è. ⚠️ **È la ragione strutturale per cui uno scarto vi è vissuto**: la riga *«sei
righe su diciassette»* dove sono sette su diciotto, col numero giusto scritto quattrocento righe
più su **nello stesso file**. Le cifre del registro sono difese **solo da chi riconta**.
⛔ **Allargare la lista è una decisione, e non è stata presa**: il registro nomina conteggi che
la guardia non sa verificare — non «`<cifra>` ADR», ma quante righe di catalogo siano coperte —
quindi non basta aggiungerlo all'elenco, servirebbe un controllo diverso. Scritto qui perché chi
lo riprende non debba riscoprirlo. ✅ **Rimisurato il 2026-08-10 chiudendo il Traguardo 3, e la
previsione ha retto:** in quel file **cinque** conteggi di test erano di nuovo stantii e nessuno
è diventato rosso. La quinta questione resta aperta con una prova in più.

### Cosa il Traguardo 3 lascia aperto, in un posto solo

⛔ **Raccolte qui il 2026-08-10 perché chi riprende deve saperle PRIMA di scrivere, non
trovandole.** Nessuna è un difetto oggi, e per ciascuna è scritto **perché**; erano sparse fra il
sorgente, il registro e l'errata, che è il modo in cui una voce aperta smette di esserlo senza che
nessuno l'abbia chiusa.

| | Dove è dichiarata | Chi la chiude |
|---|---|---|
| ⛔ **ADR-0018 è violata da entrambe le implementazioni:** un payload potato e uno mai registrato sono **indistinguibili in tre modi**. La via che non costa un'impronta è stata cercata e la misura la **uccide** — svuotare il payload fa rispondere `SuspendAndAsk` su **ogni** passo potato, a ogni ripresa | voce aperta 1 di [`porta-di-qualita.md`](porta-di-qualita.md), accanto a `prune` in tutte e due, e nel blocco **7b** della conformità | il traguardo della **ritenzione**, **insieme** alla decisione sulla funzione d'impronta — che è una voce nuova nella lista di ADR-0031 |
| ⚠️ **la terza risposta di `prune` non è tenuta da nessuna promessa:** `Missing` per un passo mai scritto lo tiene **solo** il doppio in memoria, e la mutazione `M10` su `redb` **sopravvive all'intero workspace** | voce aperta 2 dello stesso file | il **primo consumatore** di `prune`, cioè la spazzata di ritenzione |
| ⛔ **`replay()` carica TUTTO in memoria**, e la copia dei byte è stata misurata a **tre** allocazioni per record, non una | doc di `replay` in `crates/kernel/src/ports/journal.rs`, ed **E25** dell'errata | il primo consumatore che misuri un giornale grande. Il rimedio noto è un **checkpoint**, lo stesso che pagherebbe anche le scansioni di `FileJournal` |
| ✅ **CHIUSA NELLA METÀ CHIUDIBILE il 2026-08-11** — che la durabilità sia **chiesta** — e ⛔ **aperta nella lettera dell'enunciato: la MORTE del processo.** Ciò che si osserva è una chiamata a `sync_data` su un backend **nostro**, dentro un processo **vivo**; non sono osservati che la chiamata raggiunga il supporto, l'**ordine** fra `write` e `sync_data`, il commit di `prune`, né un modello di guasto in cui una scrittura non sincronizzata possa davvero **sparire** — misurato, a `falls_at = 45` il record si rilegge benché la caduta abbia rifiutato proprio il `sync_data` del suo commit. Il perimetro per esteso in [`riferimenti.md`](riferimenti.md). ⚠️ **La riga originale:** la durabilità attraverso la morte del processo non è osservabile da dentro il processo, e `Durability::None` lascia **sei test su sei verdi** | accanto al codice in `crates/platform/src/journal.rs`, gotcha **#51** | l'**iniezione di livello 2** del **Traguardo 4**, attraverso il `StorageBackend` che il Task 8 ha reso sostituibile. ⛔ **QUESTA CELLA HA DETTO IL FALSO DAL BRAINSTORMING ALL'ESECUZIONE, e la correzione è del 2026-08-11, misurata due volte.** Diceva: *«con `Durability::None` `redb` non chiama `sync_data`, quindi un backend che conta le chiamate lo dice — una campagna che pretende «`sync_data` è scattato almeno una volta» diventa rossa appena la garanzia sparisce»*. **È falsa in entrambe le metà:** sotto quella mutazione `redb` chiama `sync_data` **sette volte all'apertura** e arriva a undici, perché **sei sync su sette nascono prima che esista un record** — `create_with_backend` nudo ne fa sei; e la forma *«almeno una volta»* è quindi **l'oracolo cieco per eccellenza**, verde proprio sotto la mutazione che esiste per cogliere. ✅ **La forma giusta è un DELTA attraverso la scrittura** — il conteggio dopo la scrittura maggiore di quello dopo l'apertura — e da lì il #51 è **chiuso nella metà chiudibile**: vedi la §6 e il perimetro scritto in [`riferimenti.md`](riferimenti.md). 📌 **La forma generale, che vale oltre il caso:** un contatore che parte da un valore che **il soggetto sotto esame non ha prodotto** non è un oracolo su quel soggetto. ⚠️ E il difetto non era il numero ma la **previsione**: la cella fu scritta quando il backend cadente non esisteva — gotcha **#57**, *«una decisione presa prima che esistesse ciò di cui parla è una previsione, e si cita come se fosse una misura»* |
| ⚠️ **le guardie di `FileJournal` sono SCANSIONI**, ~56 ns per record, e `has_intent` si paga a ogni scrittura: supera il pavimento dell'`fsync` solo oltre ~26 000 record | doc di `FileJournal`, e le misure in [`riferimenti.md`](riferimenti.md) | nessuno **finché nessuna misura lo chiede**: il rimedio è lo stesso checkpoint, e due meccanismi per una misura sola si comprano quando la misura c'è |
| ⛔ **le vie A1, A2, A5, A7 del confine dei dati non fidati** restano aperte | `crates/kernel/src/boundary.rs`, voce per voce | ⛔ **nessuno**, e ciascuna lo **dichiara**: non è un arretrato, è il **pavimento** |
| ⚠️ **l'amplificazione dello spazio di `redb`**, misurata in M-8 su carico **sintetico** | §4.8 della spec | *«da rimisurare sul carico reale prima di congelare i parametri di ADR-0018»* |
| ⚠️ **il `kind` del record e l'operazione della porta restano due verità indipendenti**, e nulla di livello 1 impedisce a uno scrittore futuro di farle divergere | `crates/kernel/src/reconcile.rs` | ✅ **chiusa come DECISIONE dal proprietario**, non come garanzia: la sonda copre **l'unico scrittore che esiste**, e l'aiutante nasce col **secondo** |
| ⚠️ **il registro non è sorvegliato** dalla guardia dei conteggi | il capoverso qui sopra | il **proprietario**: allargare la lista non basta, servirebbe un controllo diverso — **registrata, non presa** |
| ⚠️ **il puntatore al prossimo passo non ha una guardia**: dal 2026-08-18 vive in un posto solo, ma **nulla impedisce** a un documento di ricominciare a riscriverlo domani — ed è già successo tre volte. ⛔ La forma meccanizzabile esiste e costa un comando: *fuori da `COMPENDIO.md`, ogni riga che porta `⏭️` deve nominare la §6* | il riquadro di chiusura qui sopra, e la 25ª misura della §12 | il **proprietario**: è una **riga di catalogo** nuova in `check-docs.sh`, cioè una sua decisione (vincolo globale 7) — **registrata, non presa**, come la guardia sui pesi e l'elenco dei semi |
| ⚠️ **l'elenco dei semi non avrà un chiudente**, e sarà l'unico artefatto del Traguardo 4 senza: nessun controllo pretende che una sua voce **nomini un test esistente**, e un elenco di semi senza proprietà è l'artefatto che marcisce meglio di tutti | §10 del [disegno del Traguardo 4](superpowers/specs/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst-design.md) | il **proprietario**: sarebbe una riga di catalogo nuova in `check-docs.sh` — **registrata, non presa**, come la guardia sui pesi della §12 |
| ⛔ **i 25,8 µs di M-2 sono citati in tre punti della SPEC, e la cifra è falsificata** — non è confrontabile con nulla che esista oggi: il prototipo non è nel repository, l'esecutore era un altro, il protocollo era un colpo singolo, e lo scenario **aveva** il giornale, contro la formula *«scenario minimo»* che lo fa leggere altrimenti. ✅ La conclusione che sostiene — *«migliaia di semi stanno dentro un secondo»* — **regge ed era per difetto** | richiami datati già scritti in [`HANDOFF.md`](HANDOFF.md), [`riferimenti.md`](riferimenti.md) e [`design/08`](design/08-strategia-di-test.md), col numero vivo. Restano le tre citazioni nella **spec del sotto-progetto 1** | il **proprietario**: la spec si approva sezione per sezione, e un richiamo datato lì è una sua decisione — **registrata, non presa** |
| ⚠️ **il portachiavi non ha un chiudente scritto**: nessuno script verifica che solo `secrets` lo raggiunga, e nessuna riga dice chi lo farà | riga di *«Cosa la porta NON controlla»* in [`porta-di-qualita.md`](porta-di-qualita.md) | ⛔ **da assegnare** — trovato chiudendo il Traguardo 3, rileggendo il registro contro la condizione 11 |

⛔ **Nessuna rinumerazione di sezioni**: lo script legge §7.4 e §8 **per posizione**.

### Il Traguardo 2, compito per compito — tutti eseguiti

| # | Compito | Stato |
|---|---|---|
| 1 | i due tempi, `Monotonic` e `WallTime`, e il terzo che li lega | ✅ |
| 2 | la porta `Rng`, e l'implementazione seminata in `simulator` | ✅ |
| 3 | i parametri di decisione, consegnati e non letti | ✅ |
| 4 | la porta `Reactor` | ✅ |
| 5 | **l'esecutore** | ✅ |
| 6 | il reattore finto, e la misura dell'interlacciamento | ✅ |
| 7 | il reattore reale in `platform`, e la prima suite di conformità | ✅ |
| 8 | il cablaggio di produzione in `daemon`, coi default letterali | ✅ |
| 9 | il confine dei tipi, e la promozione che pretende il giornale | ✅ |
| 10 | le porte `filesystem` e `network` | ✅ |
| 11 | `process` coi gettoni e le due ricevute | ✅ |
| 12 | `ipc`, e la tabella completa delle sei famiglie | ✅ |
| 13 | il registro dei controlli — e **il compito era già eseguito**, gotcha **#49** | ✅ |
| 14 | la chiusura del traguardo nei documenti di stato | ✅ |

✅ **I due buchi che il Task 6 aveva lasciato in eredità sono chiusi — ma uno dei due NON era
chiudibile dove era stato assegnato.** Il ramo `deadline <= now → None` di
`VirtualReactor::wait_until` è ora esercitato dalla conformità in **entrambe** le metà, `==` e
`<` (sonde R3 e R4). ⛔ `VirtualReactor::wall_time()` no: la conformità gira contro **tutte e
due** le implementazioni, quindi può asserire solo ciò che **entrambe** promettono, e i due
orologi che si muovono insieme sono una proprietà **della finta** — la vera serve `wall_time`
dall'orologio di sistema, che NTP fa arretrare. Metterla in conformità avrebbe reso **rossa
un'implementazione corretta**. È il gotcha **#44**: il buco si è chiuso in
`crates/simulator/tests/virtual_clock.rs`, e in conformità è rimasta una riga che prova **la
sola chiamabilità**, dichiarata come tale.

📌 **Sei difetti del piano trovati eseguendo, non leggendo**, e il più grave è invisibile
per costruzione: la cella `Sleep` veniva svuotata **solo sul ramo `Pending`**, quindi
un'attività che chiedeva di dormire e poi finiva lasciava la richiesta alla successiva.
**C1 resta verde** — la fuga è deterministica, quindi riproducibile e perciò invisibile a un
controllo di riproducibilità. Regressione permanente su un intervallo di semi, non su uno.

📌 **Il Task 7 ne ha aggiunti cinque, e quattro sono stati colti _leggendo_ il piano prima di
eseguirlo:** l'asserzione su `wall_time()` che sembrava copertura ed era `let _ = …` · la metà
`<` del ramo `deadline <= now` mai esercitata · il `catch_unwind` che accettava **qualunque**
panic invece del proprio · `SequentialRng` che nasceva **senza un test**. ⛔ **Il quinto è
uscito solo dalla revisione, ed è il più istruttivo:** il caso aggiunto per chiudere il
secondo era a sua volta **cancellabile lasciando la porta verde** — gotcha **#45**. ⚠️ E due
mutazioni sono sopravvissute a tutto: una è stata chiusa (`wall_time()` della vera, sonda
**R5**), l'altra è un **residuo dichiarato** invece che un test, perché distinguerla
richiederebbe un controllo non deterministico — e un controllo che scatta a caso è peggio di
uno assente.

📌 **Il Task 8 del Traguardo 3 ne ha aggiunti due, ed entrambi vengono da una misura che ha
smentito chi la faceva.** ⛔ **#51** — una garanzia sulla **morte del processo** non è
osservabile da dentro il processo: `set_durability(Durability::None)` lascia **sei test su sei
verdi**, e non è una lacuna del banco ma la forma del banco. ⛔ **#52** — un difetto di
parallelismo **mascherato dal sistema operativo**: la cancellazione della cartella condivisa
avviene **tre volte su sei**, ma Windows rifiuta di cancellare un file aperto, quindi il rosso
esce su **Linux**, che è il secondo sistema previsto dal progetto. ⚠️ **E una terza occorrenza
del #45:** il rimedio scritto per chiudere una vacuità (`abandon_without_commit` che risponde
`Result`) era **esso stesso vacuo**, e l'ha detto una mutazione — chiuso spostando il controllo
**dentro** il metodo, dove la transazione è ancora aperta.

### Il sotto-progetto 1 si esegue a traguardi, e ciascuno ha il proprio piano

Scrivere ora un piano per codice che non esiste significa inventare. **I Traguardi 1 e 2 sono
eseguiti; quelli dal terzo in poi si scrivono quando si arriva.** ✅ **Il piano del Traguardo 3
è scritto il 2026-08-10**, ed **eseguito lo stesso giorno** — subagent-driven, un
compito per volta con revisione fra uno e l'altro: **dodici su dodici**, `GATE GREEN` a tutti.
⚠️ **Ricontati il 2026-08-10 chiudendo il traguardo:** questa riga diceva *«sei su dodici»*
mentre la §6 era a **undici** e la tabella qui sotto a **otto** — la stessa cifra in **tre** posti
di questo file con **tre valori diversi**, che è la forma peggiore del gotcha **#31** e quella che
il riquadro precedente prometteva di non ripetere. ⚠️ **Ricontati il 2026-08-10:** questa riga
diceva *«due su dodici»* mentre la §6 e la tabella qui sotto erano già a cinque — la stessa cifra
in tre posti, aggiornata in due.

| # | Traguardo | Stato |
|---|---|---|
| **1** | **scheletro e porta di qualità** — le cinque crate e i controlli, **zero logica** | ✅ **eseguito il 2026-08-08**, `GATE GREEN` |
| **2** | **il substrato iniettabile** — tempo, casualità, I/O, scheduling, l'esecutore, le sei porte | ✅ **eseguito il 2026-08-10**, `GATE GREEN`. [Piano](superpowers/plans/2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md) scritto ed eseguito **per intero, quattordici compiti su quattordici**: i due tempi · la porta `Rng` · i parametri consegnati · la porta `Reactor` · **l'esecutore** · l'orologio virtuale · **il reattore reale e la prima suite di conformità** · il **cablaggio di produzione** in `daemon`, coi default letterali · il **confine dei tipi** `Untrusted`/`Instruction`, con la promozione che pretende la porta `journal` · le porte **`filesystem` e `network`** · la porta **`process`**, coi gettoni e le **due ricevute distinte** · la porta **`ipc`**, che chiude le **sei famiglie** · il **registro dei controlli** e questa chiusura. ⛔ **Zero record del giornale scritti**, ed è deliberato: i byte congelati appartengono al Traguardo 3 |
| **3** | giornale e formato durevole — la porta a byte, l'enum di versione, **i byte congelati** | ✅ **eseguito il 2026-08-10, dodici compiti su dodici**, `GATE GREEN` a tutti. ⚠️ **Ricontati il 2026-08-10 chiudendo il traguardo:** diceva *«otto compiti»*, ed era la terza delle tre cifre discordi dello stesso file. ⚠️ **Ricontati il 2026-08-10:** diceva *«due compiti»* ed era già indietro di uno al commit precedente, di **tre** a questo — e chiamava il compito *«la conformità coi **tre** bugiardi»* quando i bugiardi consegnati sono **sette**. Il numeratore lo muove chi esegue, e chi esegue guarda la §6. [Piano](superpowers/plans/2026-08-10-sottoprogetto-1-traguardo-3-giornale-e-formato-durevole.md) **scritto il 2026-08-10**, dodici compiti in due parti: ✅ il record versionato · ✅ la riga di catalogo dell'etichetta · ✅ il **doppio in memoria** · ✅ la **conformità coi sette bugiardi** e ✅ `replay()`, eseguiti come un compito solo · ✅ la **riconciliazione su un insieme**, che ha riportato indietro la firma di `replay()` invece di deciderla · ✅ **`promote` che diventa una nota**, con l'operazione `note()` e la variante `RecordKind::Note` che il compito ha dovuto inventare · ✅ **`redb` in `platform`** col **backend nostro**, la chiave progressiva e la prova che il confine è **sostituibile da fuori** · ✅ la conformità contro **entrambe** a ogni commit · ✅ **i byte congelati**, tre record e una mappa riletta dal banco · ✅ `prune` che rifiuta un passo in dubbio · ✅ la **chiusura**, che è stata un **audit** e non una scrittura. ⛔ **Congelamento per ultimo**, che è la decisione D1 del piano |
| 4 | il simulatore DST — **il guasto**, non il tempo virtuale: quello è del Traguardo 2 | ✅ **eseguito il 2026-08-11, dieci compiti su dieci**, `GATE GREEN` a ciascuno. ⛔ **L'errata è a settanta voci in nove passate, di cui dodici DECISIONI** — il pre-controllo ha trovato un difetto in **dieci compiti su dieci**. La più importante è **E52**: due righe dei **documenti di stato** dicevano il falso su come chiudere il gotcha **#51**, e lo dicevano **dal brainstorming**. ⛔ **E la lezione imparata TRE volte:** *«l'iniezione è avvenuta»* e *«c'era qualcosa da verificare»* sono **due** affermazioni, e una campagna che tiene solo la prima è **verde avendo confrontato insiemi vuoti** — successo a `C7a`, poi a `C7b`, poi al ciclo di livello 2, **ogni volta dopo che la precedente era stata chiusa**. ✅ **Brainstorming, disegno e piano tutti il 2026-08-11** — [il disegno](superpowers/specs/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst-design.md), che fissa il perimetro (il **motore**, non tutte le finte), i **due livelli come due campagne**, l'oracolo di non-vacuità e i **sette** artefatti col controllo che esercita ciascuno; e il [piano](superpowers/plans/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst.md), **dieci compiti in tre parti** — il giornale cadente · lo scenario giornalato e `C7a` · `C7b` con l'oracolo preso dalla **traccia** e non dall'archivio · la campagna breve col numero di semi **misurato** · il backend cadente scritto **da fuori la crate** · la coerenza dopo la riapertura e il **#51 chiuso dal conteggio dei `sync_data`** · la campagna di livello 2 · l'elenco dei semi · il tempo di parete nel cancello · la chiusura. ⚠️ Il titolo diceva *«tempo virtuale, guasti, campagna, semi»* e il tempo virtuale era eseguito da due traguardi |
| 5 | arbitro GPU — ammissione, corsie, concessione, le due policy | 🔵 **aperto il 2026-08-18: brainstorming chiuso, [disegno](superpowers/specs/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu-design.md) scritto, [piano](superpowers/plans/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu.md) scritto — tredici compiti in cinque parti — esecuzione COMINCIATA il 2026-08-19: QUATTRO compiti su tredici, `GATE GREEN` a ciascuno.** ⚠️ **Questa cella diceva *«esecuzione da fare»***, ed è la cifra che il numeratore muove: la muove chi **esegue**, e chi esegue guarda la §6 del compendio, dove il racconto sta per esteso e in un posto solo. Il perimetro, gli artefatti col proprio controllo e le righe scoperte col proprio indirizzo stanno **nel disegno**, §0 e §9. ⚠️ **Questa cella diceva *«eredita CINQUE delle nove righe di guasto»*, e la cifra era sbagliata**: contate sulla §7 del disegno del Traguardo 4 sono **una**, più una condivisa. Non è ricorretta qui — **è tolta**, e la cella rimanda alla fonte: una cifra che vive in più documenti si toglie (`CLAUDE.md`). ⛔ **E non porta il puntatore al suo posto:** quello vive **in cima a questa sezione**, in un posto solo |
| 6 | gli altri meccanismi — gateway, sensori, permessi, degrado, canale worker | ⬜ — eredita **le altre**, e quante sono lo dice la §7 del disegno del Traguardo 4, non questa cella |

⚠️ **E il Traguardo 4 lascia aperte tre cose, dichiarate e non risolte.** ⛔ **Le nove righe di
guasto della §3.3 che restano scoperte**, ciascuna col proprio indirizzo ai Traguardi 5 e 6 — non
è un arretrato, è uno **scaglionamento**, ed è la §7 del disegno a darglielo. ⛔ **Il gotcha #51 è
chiuso nella METÀ chiudibile** — che la durabilità sia *chiesta* — e ciò che resta fuori è la
lettera del suo enunciato, la **morte del processo**: il perimetro esatto sta in
[`riferimenti.md`](riferimenti.md), ed è più lungo di ciò che la chiusura compra.
⚠️ **[`semi-dst.md`](semi-dst.md) non ha un chiudente**, ed è l'unico artefatto del traguardo
senza: una guardia in `check-docs.sh` che pretenda che ogni voce nomini un test esistente sarebbe
una **riga di catalogo nuova**, cioè una decisione **del proprietario — registrata, non presa**.

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
| **«punto di partenza» significa che si copia** | la §2.5 dice riga per riga cosa entra in `crates/kernel/` e cosa **resta** negli spike. Nel Traguardo 1 non era salito niente. ✅ **Onorata dal Traguardo 2, verificata riga per riga il 2026-08-10:** è salito tutto ciò che la §2.5 gli assegnava — `boundary.rs`, `rng.rs` con la seminata in `simulator`, `executor.rs`, la porta `journal`, i casi di `compile_fail`. Restano negli spike le due righe che la §2.5 dichiara **non** debbano salire, e il **doppio cadente** del giornale, che è del **Traguardo 4** |

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
| ❌ **riscrivere `tracciabilita.md` da zero** | **centosettantuno** funzionalità già mappate — ricontate sulle quindici tabelle il 2026-08-10, ché questa riga diceva *«centosettanta»*: si **aggiorna**, e **solo alla chiusura del sotto-progetto 1** — quindi non ora |
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

## 9. I settantadue gotcha

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
| 29 | **La riga di _verifica_ di un'invariante è il punto in cui l'invariante si restringe in silenzio.** Già successo con I2 e con I4. Completare una riga di verifica **non è superare l'ADR**. 📌 **Terza occorrenza:** non una riga di verifica ma una **riformulazione** — la colonna «Vincolo» di §8.3, otto formulazioni troncate su trentasette, e su `V16` la metà caduta era quella che **cambiava il verdetto**. 📌 **Quarta, il 2026-08-11, e sposta la forma su un LIMITE DICHIARATO in un doc di codice:** `CrashingJournal` prometteva *«ogni scrittura successiva è rifiutata»* e dichiarava il proprio limite sulle **sole letture** — due categorie scritte guardando i casi che si avevano in mente quel giorno, e `prune` **non appartiene a nessuna delle due**: muta, quindi non è una lettura, ma non è fra le tre scritture contro cui si estrae il punto di caduta. È rimasta fuori dalla guardia, e un processo morto potava ancora. ⛔ Nessun controllo poteva accorgersene per la stessa ragione delle prime tre: **la partizione sembra esaustiva**, quindi chi rilegge non cerca il terzo membro. 📌 *Questo limite dichiarato è una partizione, e ne copre tutti i membri?* |
| 30 | ⛔ **Un banco che guarda solo `Ok`/`Err` non vede la _risposta sbagliata_.** Misurato in M-9: cinque celle su trentasei restituiscono `Ok` con **valori sbagliati**. Confrontare i **valori**, non l'esito. In un archivio durevole il modo di fallire peggiore non è l'errore — è il record che ti restituisce il numero sbagliato |
| 31 | **Una stima di costo prezzata sulla variante sbagliata sopravvive, perché viene citata invece che rifatta.** Misurato: un byte su ventisei, contro il «permanente su ogni campo» che stava per far scartare la forma giusta. Una stima che sta per **decidere** va rimisurata. 📌 **Terza forma, e non è una stima mai rifatta: è una misura _vera_ che marcisce mentre il codice sotto di lei cresce.** Un commento diceva *«misurato: cancella questa intera funzione e i test restano verdi»*, ed era vero — la funzione aveva due righe. Poi le è stato aggiunto sotto l'**unica** copertura di `WorkerDescriptor`, e quel commento **autorizzava a cancellarla**. Non c'era niente da rimisurare: a cambiare è stato l'**oggetto**. 📌 Si lega l'affermazione a **ciò che fu misurato** — «queste due righe» — mai al **contenitore**, che può crescere senza avvisare |
| 32 | ⛔ **Un'idea che sembra nuova può essere già stata scartata, e il compendio non lo dice.** Comprime le **decisioni**, non le alternative respinte: una proposta ragionevole può essere già caduta — con la misura — dentro un ADR o una sezione. Prima di proporre una **sostituzione** si cerca dove era già stata valutata e perché. Successo il 2026-08-08 con `minicbor` su `ipc`: a smontare la proposta è stata **la stessa misura che l'aveva motivata** |
| 33 | **Il nome del formato è occupato da un'altra cosa, e in due ecosistemi.** Su PyPI `bincode` installa un modulo `b64tools`, funzioni base64; su npm `bincode` è una CLI di sviluppo con l'IA. È il gotcha #22 nella forma più larga: **che un nome esista non dice cosa contiene**, e cercare per nome trova pacchetti che non c'entrano |
| 34 | ⛔ **Un decodificatore CBOR si ferma al primo elemento completo e ignora la coda.** Misurato: dando a `cbor2` i byte di `bincode` restituisce `1` — nessuna eccezione, un valore plausibile. Su un canale a frame «ha decodificato» non prova nulla: serve che i **byte consumati** siano pari alla lunghezza dichiarata |
| 35 | **Un `Vec<u8>` non annotato raddoppia il traffico, in silenzio.** In `minicbor`, senza l'annotazione di stringa di byte, si codifica come **array di numeri**. Misurato su 4096 B: **7813** contro **4101**, cioè **1,91×**. Compila, fa round-trip, ed è corretto: costa solo il doppio |
| 36 | ⛔ **La tabella «come si verifica» di una sezione non è il catalogo, e il passaggio si salta.** Una sezione che decide un meccanismo scrive le proprie sonde lì — ed è giusto — ma il **catalogo §7.4** è l'unico posto che §8.1.2 ammette, ed è la lista che il piano tradurrà in lavoro. Successo **tre volte**: V2/V4/V10 (§8.5.3) · i cinque controlli della §6.10.5 (§8.5.4) · e la **regola B** della coppia `Untrusted`/`Instruction`, implementata al Task 9 e catalogata solo il 2026-08-09. ✅ **La terza è la prima colta _prima_ che si sedimentasse**, e a coglierla non è stato un controllo: il registro [`porta-di-qualita.md`](porta-di-qualita.md) ha dichiarato il caso come *implementato e non coperto dal catalogo*, e la §6 l'ha portato come **voce aperta** finché qualcuno l'ha chiusa. 📌 Da lì la regola operativa: uno scarto fra codice e catalogo si scrive nel registro **come voce aperta**, mai come nota — una nota si legge e si dimentica. ⚠️ La diagnosi è l'**asimmetria**: nella stessa riapertura §2.8.4 e §4.9.4 le righe le hanno aggiunte, §6.10.5 no, e **nessun documento dice perché** — quindi non era una decisione. ⛔ Il rimedio **non** è irrigidire lo script (§8.6.4): è cercare questa classe di difetto a ogni sezione che decide un controllo. È il #29 spostato dalle invarianti ai controlli |
| 37 | ⛔ **Un controllo può difendere _un altro controllo_ invece di una proprietà, e la regola d'ammissione lo scambia per un'abitudine — cioè lo fa togliere.** Misurato: **otto righe su trentatré** del catalogo, fra cui `forbid(unsafe_code)`. Cancellalo e **nessuna** riga diventa rossa: diventano tutte **meno vere**, perché un `unsafe` falsifica un gettone o transmuta un newtype. ⛔ Il rimedio **non** è allargare la regola a «una proprietà decisa in una sezione nominata» — così non rifiuterebbe più niente, e una regola che non rifiuta mai è decorazione. È darle un **secondo ramo** con un criterio proprio: *nomina le voci del catalogo di cui sostieni la validità*. Quello rifiuta ancora `clippy` |
| 38 | **Un controllo che interroga uno stato può _modificarlo_ mentre lo interroga, e allora la sua condizione non può essere vera.** `gate-no-os.sh` chiede a `rustup` quali bersagli siano installati. Misurato: quel comando **riconcilia `rust-toolchain.toml` prima di rispondere**, quindi se il bersaglio manca **l'atto di chiederlo lo installa**. Con la rete la guardia **non può scattare**; scatta **senza rete**, verificato con uscita 1 e messaggio corretto. È **la via offline, non una rete di sicurezza**, e la differenza va scritta accanto alla guardia |
| 39 | ⛔ **Un test negativo che _ridichiara le proprie precondizioni_ prova il meccanismo, non che il sistema sia configurato così.** I quattro casi di `compile_fail/` ridichiarano ciascuno `#![no_std]` e `#![forbid(unsafe_code)]` e non nominano mai `kernel::`: provano che il divieto **morde dove è dichiarato**, non che il kernel lo dichiari. Misurato: tolto `#![forbid(unsafe_code)]` da `crates/kernel/src/lib.rs` **e scritto un `unsafe` vero**, la porta restava **verde su cinque controlli su cinque**. Rimedio: `scripts/gate-attributes.sh`, livello 2. ⚠️ Peggiore di come sembra: quella riga è di **ramo 1b** e sostiene i blocchi A, B e C — toglierla non spegneva una regola, invalidava **il fondamento del livello 1** |
| 40 | ⛔ **Questo file dichiara di contenere tutte le decisioni, ma il controllo ne pretende una voce solo per gli _ADR_: una decisione che vive in una _sezione di spec_ può mancare, e allora per chi legge non esiste.** Misurato il 2026-08-08: la **§1.0** — codice in inglese, documentazione in italiano — non era né qui né in `CLAUDE.md`. Un agente ha letto per intero **entrambi** i file obbligatori e ha scritto un traguardo intero con gli identificatori in italiano, poi corretto con sei rinomini di file, undici di funzione e la rigenerazione dei quattro oracoli. ⚠️ Il controllo di §13 accoppia le voci ai **file** in `docs/adr/`: esatto per ciò che misura, cieco a tutto il resto. ⛔ Il rimedio non è irrigidire lo script — nessun elenco di «sezioni che contano» resterebbe vero — è che **una decisione scritta fuori da un ADR va portata a mano nel compendio**, e chi la scrive è l'unico che può saperlo |
| 41 | **Un filtro che _normalizza l'ingresso_ di un controllo decide anche che cosa il controllo può vedere.** `gate-deps.sh` estraeva i nomi di crate con una classe di caratteri **minuscola**. Misurato: una crate col nome maiuscolo veniva scartata dal filtro, quindi non compariva fra gli intrusi e il cancello usciva **verde** — un falso negativo su I3, il modo di fallire peggiore per quel controllo. Provato con `Inflector`, crate reale: uscita 0 prima, uscita 1 e nome del colpevole dopo. ⚠️ Col corteo di dipendenze minuscole che si porta dietro, il controllo segnalava **il corteo e non il capofila**, pur stampandolo dentro ogni catena |
| 42 | ⛔ **Un test di compilazione fallita che scatta come `mismatch` è disarmato da una rigenerazione in blocco; uno che scatta come `error` no — e `trybuild` stampa la parola.** Misurato: col confine fra `Monotonic` e `WallTime` guardato dal solo caso «passa l'uno dove va l'altro», aggiungere `impl From<WallTime> for Monotonic` **non** lo fa smettere di fallire — resta `E0308`, perché Rust non applica `From` al sito di chiamata. Lo rende rosso il fatto che rustc **aggiunga quattro righe di `help: call Into::into`** che l'oracolo non porta: un **`mismatch`**. Quindi quel guardiano **poggia interamente sul fatto che l'oracolo non venga rigenerato** — è il #25 con la conseguenza che nessuno aveva scritto: **l'oracolo registra anche l'assenza di una via di conversione**. Rimedio: un **secondo caso di forma diversa** — `let _x: Monotonic = wall.into();` — che con l'`impl From` presente **compila**, e scatta come `error`. ⚠️ **Il test generale, che costa zero:** una regola guardata solo da casi `mismatch` è una regola che una rigenerazione spegne in silenzio. 📌 E le regole erano **due** — «non si passa l'uno per l'altro» e «non esiste una via di conversione» — con la seconda scritta **in un commento**, cioè un'intenzione |
| 43 | ⛔ **In un modello, un valore d'esempio _valido_ viene incollato così com'è: non si distingue da un dato. E un avviso accanto non è un rimedio.** Il campo «Ultimo commit» di [`AVVIO-CHAT.md`](AVVIO-CHAT.md) portava un hash vero, con accanto una riga che **dichiarava il difetto** e ne scaricava il rimedio su chi incolla. Misurato: il file era vecchio di **due** commit, il messaggio incollato di **quattro**, e due commit erano serviti solo a rincorrerlo — non possono raggiungerlo, perché lo contengono. ⚠️ **Togliere il campo era la proposta sbagliata:** `HANDOFF.md` non porta lo SHA **perché delega a lì**, e la prova ha giocato contro — il **#32 applicato a sé**, con un rimedio migliore in uscita. Il difetto è più stretto: lo SHA appartiene all'**istanza**, il **modello** ci metteva un valore concreto. Rimedio: un **segnaposto** che nomina il comando che lo riempie — si fallisce **rosso** invece che verde, come nel **#26** |
| 44 | ⛔ **Una suite di conformità prova solo ciò che TUTTE le implementazioni promettono: il buco che le viene assegnato può non essere chiudibile lì, e forzarcelo rende rossa un'implementazione _corretta_.** §6 e [`porta-di-qualita.md`](porta-di-qualita.md) assegnavano alla conformità di `reactor` di coprire `VirtualReactor::wall_time()`. Ma la finta muove i due orologi **insieme** (deliberato: un timbro fermo contraddirebbe il proprio ordinamento) e la **vera non deve**, perché serve `wall_time` dall'orologio di sistema, che NTP fa arretrare. Il buco si è chiuso in `crates/simulator/tests/virtual_clock.rs`, e in conformità resta una riga che prova **la sola chiamabilità**, dichiarata come tale invece del `let _ = …` che sembra copertura. ⛔ La forma generale: assegnare un buco a un controllo condiviso **presume che il buco sia una proprietà condivisa** — quando non lo è, non si indebolisce il controllo, gli si trova **l'altra sede**. Contro-sonda **R6** |
| 45 | ⛔ **Il rimedio a una copertura mancante è esso stesso un controllo, e nasce non provato — perché lo si scrive credendo di stare già rimediando.** Il caso `deadline < now`, aggiunto al Task 7 per coprire la metà di ramo che il piano lasciava fuori, era **cancellabile lasciando la porta verde**: il bugiardo del file moriva sul primo caso senza mai raggiungerlo, e i due condividevano **lo stesso messaggio**. ⚠️ Il difetto stava **dentro il file che spende un intero test negativo proprio per impedirlo**. Rimedio in due pezzi: due messaggi **distinti**, così che il payload dica quale metà ha sparato, e un **secondo bugiardo** rotto in modo diverso. Valgono il **#14** e il **#24** anche per il tappo. Sonda **R4**. 📌 **Seconda occorrenza al Task 11, e stavolta il non-provato era un'_eccezione dichiarata_.** La finta di `Worker` spendeva quattro righe a dire che `kill` la guardia di liveness **non ce l'ha, di proposito** — uccidere è sempre lecito — e **niente lo teneva**: dandogliela, i nove test restavano **verdi**. ⚠️ È saltata fuori da una **rifinitura di stile**: estraendo l'aiutante `alive()`, l'unico punto che non lo chiama è diventato **visibile**, e visibile ha fatto chiedere se fosse provato. ⛔ Un'eccezione scritta in un commento è indistinguibile da una dimenticanza finché una mutazione non prova che il sistema la difende: il #14 vale anche per ciò che il codice sceglie di **non** fare |
| 46 | ⛔ **Su una porta mai implementata, YAGNI cancella ciò che serve a implementarla.** Al Task 10 la regola avrebbe tolto `Path::as_bytes()` ed `Endpoint::as_bytes()`, senza chiamanti. **Misurato: le due porte sarebbero rimaste non implementabili fuori da `kernel`** — la privacy del campo di una tuple-struct è **di modulo**, quindi `platform` non può leggere `Path.0`, e nulla diventa rosso perché lì l'implementazione non esiste. ⚠️ Il difetto non è YAGNI: è che su un tratto **dichiarato in anticipo** i chiamanti sono vuoti **per costruzione**, quindi il criterio non distingue il morto dalla **sola porta d'ingresso di chi verrà**. Rimedio: un'**implementazione finta** in un test (`ports_are_implementable.rs`), che dà un chiamante a ciò che serve e lascia scoperto solo il morto vero. 📌 E prova per giunta che le firme siano **implementabili** — ha colto che `Clone` su `Path` è portante per `declare_scope`. 📌 **Seconda occorrenza al Task 11, in forma peggiore:** non «non riesco a **leggere** un campo» ma «non riesco a **produrre** il valore di ritorno» — le due ricevute di `process` non erano costruibili da fuori, e `instruct_one` deve restituirne una. Un accessore mancante si intuisce leggendo il tipo; un **costruttore** mancante no, perché da dentro la crate il tipo si costruisce benissimo: il difetto **esiste solo dal lato di fuori**, e l'unico strumento che sta di fuori è la finta |
| 47 | ⛔ **Gli errori di rustc si mascherano fra passate: l'elenco che leggi è quello della _prima passata che ha fallito_, non tutti.** Misurato: il letterale `SingleReceipt { id: 7 }` scritto da fuori dalla crate **non dava nessun errore** — e la lettura ovvia era che un campo `pub(crate)` fosse scrivibile da fuori. È un **`E0451`**, che lo emette la passata di **privacy**, la quale **non gira** se la compilazione si ferma prima al type-check. Sanati quelli, compare. ⚠️ *«Ho corretto e adesso compila»* e *«ho corretto e adesso emerge il secondo errore»* sono indistinguibili **prima** di correggere. 📌 Quando si prova che qualcosa **non** è possibile, si sanano prima tutti gli errori diversi da quello cercato |
| 48 | ⛔ **Un banco di misura sbaglia _verso l'attesa_, ed è peggio di uno che si pianta: si smette di guardarlo quando conferma.** Quattro inciampi reali in una sessione, tutti nel banco e nessuno nel codice: due `sed` che non agganciavano la riga — la mutazione non si applicava e il verde somigliava alla **vacuità che si cacciava** · un rilevatore su `^error` che pescava l'`error: test failed` di `cargo`, dichiarando «non compila» dieci mutazioni che compilavano **e uccidevano** · una costante scelta a caso che coincideva col valore atteso, così che con `7` il test moriva e con `1` **passava** · una sostituzione globale che ha riscritto il corpo dell'aiutante **dentro sé stesso**, colta dal conteggio dei siti e non dai test. È il **#15 applicato allo strumento** e il #17 spostato dall'iniezione al misuratore. 📌 **Contro-verso:** provare che la mutazione **si sia applicata**, compilare in un passo **separato** dall'eseguire, e per ogni mutazione su un valore **provarne due**. 📌 **Salite a nove col Task 12, e tre forme sono nuove.** ⛔ Un numero solo **misurato quattro volte e sbagliato tre**, ogni volta per un difetto diverso — e la terza misura, un parser che guardava il ramo sbagliato dell'albero delle diagnostiche, ha risposto **«zero» con uscita pulita**: la bugia più credibile, perché è **un numero preciso da uno strumento che sembra funzionare**. ⛔ **Due strumenti gemelli, corretto uno solo**: il bug riparato in uno è rimasto nell'altro, e nulla lo segnalava perché quello riparato funzionava. ⛔ **E la più insidiosa: una rifinitura di _leggibilità_ disarma la campagna di mutazione senza che nulla diventi rosso** — una rinomina richiesta da una revisione ha reso stantie due ancore, e una mutazione è tornata «zero siti» invece di un esito. 📌 Le ancore sono **accoppiate ai nomi del codice**: la campagna si rilancia dopo ogni **rifinitura**, non solo dopo ogni cambiamento di comportamento. 📌 **Decima, il 2026-08-11, e la causa è nuova: il sorgente dello strumento si corrompe PRIMA di girare, in silenzio e in modo selettivo.** `python - <<'PY'` decodifica lo stdin nel **codepage di sistema** e non in UTF-8: una stringa di ricerca con un trattino lungo o un `⛔` **non trova nulla**, mentre quelle ASCII sostituiscono regolarmente — quindi lo script applica **una parte** delle modifiche ed esce **con successo**. In un repository i cui file sono pieni di quei caratteri è il modo di fallire peggiore. 📌 Non basta provare che la mutazione si sia applicata: **si conta quante sostituzioni sono andate a segno**, e non si passa testo non-ASCII a un interprete via stdin. 📌 **Undicesima, lo stesso giorno e sorella della decima: un escape della shell che non sopravvive al trasporto non FALLISCE, DEGRADA IN UN ALTRO PATTERN VALIDO.** `grep -c $'\r$'` per contare i fine-riga CRLF ha risposto **`0`** su un file che ne ha **291**: il byte `CR` letterale si perde e il pattern diventa `r$`, che in BRE è *«riga che finisce per la lettera r»* — e non ne esiste nessuna. ⛔ **La stessa tecnica, alla prima invocazione, era degradata diversamente e aveva risposto «tutte le righe»: due risposte opposte, entrambe sbagliate, nessuna rumorosa.** `tr -cd '\r' \| wc -c` funziona perché l'escape lo interpreta `tr` e non la shell. ⚠️ **E la forma da ricordare è quella del secondo errore:** chi aveva colto il primo ha creduto al secondo senza dubitarne — **aver corretto una misura la rende immune al sospetto**, ed è il momento in cui si smette di guardarla. 📌 **Dodicesima, e non è un inganno del banco ma un pericolo del RIPRISTINO: chi muta un file che sta anche SCRIVENDO non ha `git` come rete di sicurezza.** Il ripristino canonico dopo una mutazione è `git checkout -- <file>`, ed è sicuro finché il file è committato; su un file che porta lavoro **non ancora committato** cancella **tutto il compito**. Sfiorato il 2026-08-11 e fermato da un classificatore, non da una regola. 📌 Il contro-verso: quando il bersaglio della mutazione è anche l'artefatto in corso, si ripristina **con gli strumenti di edit** o da una copia presa **prima**, mai da `git` |
| 49 | ⛔ **Un compito di consolidamento in coda a un piano è già eseguito, se il piano impone di consolidare a ogni passo — e chi lo esegue alla lettera duplica invece di verificare.** Il **Task 13** dettava di aggiungere al registro [`porta-di-qualita.md`](porta-di-qualita.md) **quattro** righe di regole coperte, **tre** contro-sonde e **quattro** righe di «cosa resta scoperto». Il registro ne aveva già **dieci**, **quattro** e **nove**: i Task 1–12 lo avevano aggiornato a ogni passo, che è la disciplina che questo repository impone. Eseguirlo alla lettera avrebbe **duplicato** informazione già presente — il difetto per cui due giorni prima quel file era sceso da 531 a 449 righe — e avrebbe lasciato in piedi i **conteggi stantii**, perché il compito chiedeva di **aggiungere**, non di **ricontare**. E i conteggi stantii c'erano: *«sei righe su diciassette»* dove sono **sette su diciotto**, col numero giusto scritto quattrocento righe più su **nello stesso file**. ⛔ **Ed è la quarta specie di difetto del piano.** Le altre tre sono la **sonda sbagliata** (si coglie rileggendo), la **sonda assente** (si coglie solo chiedendosi, per ogni artefatto che il compito produce, quale controllo lo eserciti) e l'**artefatto sbagliato** (si coglie solo scrivendone un'implementazione da fuori dalla crate). Questa è il **compito stantio**, e non si vede in nessuno dei tre modi: il piano è coerente con sé stesso, il codice è corretto, e nessuna rilettura del piano la rivela. Si vede **solo** confrontando ciò che il compito dà per da fare con ciò che il repository **ha già**. ⚠️ **E lo stesso meccanismo aveva colpito la _Definizione di «fatto»_ dello stesso piano**, che pretende *«otto casi `compile_fail` — i quattro del Traguardo 1 più i quattro di questo»* dove sono **quattordici**, quattro più dieci: un criterio di chiusura invecchia come tutto il resto, e nessuno lo rilegge perché è il **metro**, non l'oggetto misurato. 📌 **La domanda che lo coglie, e costa una riga:** *prima di eseguire un compito, ciò che detta di produrre esiste già?* |
| 50 | ⛔ **Una finta che rompe un contratto è legittima quando il test parla della ROTTURA, e difettosa quando il test parla del comportamento ORDINARIO.** `RecordingJournal` era riga per riga `UnguardedIntentJournal`, il bugiardo della promessa 6 — e su di esso poggiavano i test ordinari della promozione: il `promote` dettato **passava** contro la finta e rispondeva `OutOfOrder` contro **entrambe** le implementazioni vere. ⚠️ La regola «nessuna finta può rompere una promessa» sarebbe **falsa**: l'audit delle **ventuno** implementazioni fuori da `src/` ne ha trovata una seconda — `RefusingReactor` — che ne rompe una **e va bene**, perché il suo test si chiama *«un reattore che non avanza»*. 📌 **La domanda che separa i casi:** *questo test misura ciò che il sistema fa normalmente, o ciò che fa quando qualcuno sbaglia?* E una finta scritta **prima** della suite va riesaminata **il giorno in cui la suite nasce** |
| 51 | ⛔ **Una garanzia che riguarda la MORTE del processo non è osservabile da dentro il processo.** `FileJournal::append` si appoggia alla durabilità di default di `redb` (`Immediate`, letta nel sorgente); inserendo `set_durability(Durability::None)` **tutti e sei** i test restano **verdi**. Non è una lacuna del banco: i test riaprono il file dentro un processo **vivo**, quindi le scritture sono comunque nelle mani del sistema operativo. 📌 Ogni volta che una promessa dice *«sopravvive a un crash»*, il test in-process tiene al più *«è leggibile dopo una riapertura ordinata»*, che è un'altra affermazione. Il rimedio è un **meccanismo** — l'iniezione di livello 2 del Traguardo 4 — e finché non c'è, la garanzia si **dichiara non tenuta** accanto al codice |
| 52 | ⛔ **Un difetto di parallelismo può essere mascherato dal sistema operativo: verde su Windows, rosso su Linux.** L'aiutante dettato fa `remove_dir_all` su una cartella **condivisa**, e il piano credeva bastassero nomi di file distinti. Strumentato: **tre chiamate su sei** cancellano davvero la cartella sotto altri test — la corsa **c'è** — ma il rosso **non si riproduce in dodici esecuzioni**, perché le altre tre rispondono `PermissionDenied`: Windows rifiuta di cancellare una cartella con dentro un file aperto. Su Linux `unlink` riesce e il banco cadrebbe. 📌 Il progetto è **Windows primario poi Linux**: il difetto è programmato per uscire il giorno del secondo sistema. ✅ Rimedio: una cartella **per call site**, dal `line!()` — distinte **per costruzione**, non per disciplina. *«Non si riproduce»* non è *«non c'è»* |
| 53 | ⛔ **Una misura presa in ANTICIPO dice che il risultato sarà verde, non che il banco che lo produrrà sia giusto.** Il Task 8 aveva misurato la conformità contro `FileJournal` — *«otto su otto»*, errata **E45** — e la previsione era **giusta**. Ma era stata presa con una fabbrica **diversa** da quella dettata dal piano, che contiene **tre** guasti: percorso fisso in cartella condivisa con l'errore di cancellazione ignorato (**#52**), un'implementazione che tiene un **lucchetto**, e una suite che la chiama **nove** volte mentre una promessa conta l'intero archivio. ⚠️ E il file usa-e-getta non esisteva più: restava la cifra senza il banco. 📌 Una misura anticipata vale come **previsione dell'esito**, mai come **collaudo del codice che verrà scritto**, e si riporta dicendo **con quale banco** è stata presa. È il **#15** spostato nel tempo |
| 54 | ⛔ **Una mutazione che deve restare VERDE può esserlo perché non ha fatto niente — e allora il verde non prova la tolleranza, prova che la mutazione non è arrivata da nessuna parte.** Il **#48** pretende che la mutazione **si applichi e compili**; non basta quando il verdetto atteso è il verde. Al Task 10 la regola 3 di §4.9.2 — *un campo nuovo è facoltativo e prende un indice nuovo* — si prova aggiungendo un `Option` su un indice libero e verificando che i byte congelati **non si muovano**: e non si muovono. ⛔ **Ma un campo che non arrivasse mai sul filo darebbe lo stesso identico esito**, e la conclusione *«il formato tollera l'aggiunta additiva, ADR-0036 confermato»* sarebbe **falsa avendo l'aria di essere misurata**. Misurato: con `None` sono **21 byte identici** — `minicbor` **tronca** un `None` in coda invece di scrivere `null`, e l'intestazione dell'array resta `85` — e con `Some(9)` sono **22**, con `86` e il valore in fondo. 📌 **Contro-verso:** prima di concludere che una mutazione è invisibile **dove la si vuole invisibile**, provare che è **osservabile da qualche parte**. È il #24 applicato non al controllo ma **alla mutazione**: anche una mutazione si prova in due direzioni |
| 55 | ⛔ **Quando più asserzioni condividono UN messaggio, una mutazione mirata a una di esse è colta da un'altra: il rosso c'è e non prova ciò che si voleva provare.** Al Task 11 del Traguardo 3, per mostrare che la promessa **7b** non è vacua, si neutralizza l'asserzione che il bugiardo uccide e si pretende un *«THE SUITE IS VACUOUS»*. **La sonda è rimasta VERDE**: le tre asserzioni del blocco portano lo stesso messaggio — è la regola *un messaggio per promessa* — il bugiardo è caduto sulla **seconda**, e il confronto è per `contains`, che non distingue quale abbia sparato. ⚠️ E il verde si leggeva come una prova quando diceva solo che il blocco ha ancora denti **da qualche parte**. 📌 **La non-vacuità di un blocco a più asserzioni si prova togliendo IL BLOCCO, non un'asserzione.** È il rovescio del **#54** e la specie del **#15** |
| 56 | ⛔ **Due implementazioni della stessa porta non conoscono le stesse cose: una guardia nuova può costare a una tre righe e all'altra un CAMBIO DI ARCHIVIO — e il piano lo scrive «e l'equivalente nell'altra».** Al Task 11 `MemoryJournal` sa quale operazione ha scritto ogni voce, quindi *«questo passo ha un esito?»* è una riga; `FileJournal` tiene `(passo, byte)` e **non poteva rispondere**: `has_intent` chiede solo *«c'è QUALCHE record»*, contare i record è sbagliato perché una **nota** non è un esito, e decodificare i byte è vietato da **ADR-0036**. L'unica via era scrivere l'operazione nell'archivio, cioè cambiarne il formato. ⚠️ E il difetto era già dichiarato impossibile in un commento — vero della domanda di allora, falso della successiva. 📌 **Prima di scrivere «e l'equivalente nell'altra», chiedersi se l'altra abbia l'INFORMAZIONE per rispondere** |
| 57 | ⛔ **Una decisione presa PRIMA che esistesse ciò di cui parla è una PREVISIONE, e si cita come se fosse una misura.** [ADR-0032](adr/0032-motore-di-persistenza.md) colloca il backend cadente *«in `simulator`»*, ed è **non eseguibile**: `redb` non ha `no_std`, i sei metodi di `StorageBackend` restituiscono `std::io::Error`, `simulator` si costruisce per `x86_64-unknown-none`, e il suo grafo spedito ha una lista chiusa la cui **unica cura scritta** per un intruso è *«REMOVE the dependency. Adding it to the list is not a remedy»*. ⚠️ **La misura che chiuse quell'ADR era vera** — dodici crash iniettati, dodici riaperture coerenti — ma fu presa in uno **spike**, quando `crates/simulator/` non esisteva e non aveva i propri vincoli: la riga sulla collocazione non era il risultato, era **dove chi misurava immaginava che sarebbe finito**. ⛔ **Il difetto non si vede rileggendo l'ADR**, che è coerente con sé stesso, né eseguendo, perché nessuno ci aveva ancora provato: si vede solo **leggendo la decisione contro le guardie di oggi**. 📌 **La diagnosi vale più dell'errore: i due livelli di crash erano trattati come una cosa sola** mentre hanno **soggetti** diversi — il livello 1 esamina la riconciliazione del kernel, il livello 2 la coerenza di `redb` — quindi collocazioni, costi e cadenze diversi. 📌 **La domanda che lo coglie, e costa una riga:** *quando questa riga fu scritta, esisteva la cosa di cui parla?* È il **#15** spostato dallo strumento alla **collocazione**, e il **#53** — *«una misura anticipata vale come previsione dell'esito, mai come collaudo del codice che verrà scritto»* — applicato al **dove** invece che al **cosa**. ⚠️ E la stessa cella viveva **anche nella §5** di questo file: un ADR compresso eredita l'errore dell'ADR |
| 58 | ⛔ **Un documento di DISEGNO si legge contro il codice esattamente come un compito — e i BANCHI DI PROVA sono codice.** Il disegno del Traguardo 4 fu scritto leggendo la spec, gli ADR e le **guardie** — `gate-*.sh`, i manifesti, il sorgente di `redb` — e sbagliava **due** cose, entrambe visibili solo aprendo i banchi, che nessuna di quelle letture tocca. ⛔ **Prima:** collocava `CrashingBackend` in `platform/src/` appoggiandosi al precedente di `abandon_without_commit`, e **la risposta giusta era scritta in un commento** di `crates/platform/tests/file_journal.rs` — *«Milestone 4 will put a FAILING one in the same place»*. Il precedente per giunta **non trasferiva**: quel metodo è `pub` perché **non è scrivibile da fuori**, mentre un backend lo è, ed è da fuori che deve essere scritto o non prova nulla (**#46**). ⛔ **Seconda:** lo faceva avvolgere `redb::InMemoryBackend`, che tiene i guardiani **privati** — i byte muoiono con l'oggetto e l'archivio **non si riapre**, che è l'intera domanda a cui il livello 2 risponde. ⚠️ **Nessuna delle due si vede rileggendo il disegno**, coerente con sé stesso, e nessuna è il **#57**: quella è una decisione **anteriore** a ciò che nomina, questa è un documento **posteriore** che non ha guardato. 📌 **La domanda che le coglie, ed è la quinta del pre-controllo spostata dal compito al disegno:** *questo documento è stato letto contro il codice di oggi, **banchi di prova compresi**?* 📌 **E la generalizzazione utile: le guardie non sono tutto il codice.** Un documento che le ha lette **si sente verificato**, ed è precisamente lì che smette di guardare |
| 59 | ⛔ **Un ADR può essere falsificato da un ADR FRATELLO scritto lo stesso giorno, e nessuno dei due se ne accorge perché entrambi sono coerenti con sé stessi.** Misurato il 2026-08-11: [ADR-0026](adr/0026-linguaggio-del-core.md) elenca fra le proprie `Positive` *«esiste un runtime deterministico di ecosistema — `madsim` 0.2.34 … quindi il simulatore non va scritto da zero»*, e [ADR-0031](adr/0031-dipendenze-del-kernel-parte-del-confine.md) — **`Date: 2026-08-06`, la stessa** — misura che una crate `no_std` che dipende da `madsim` **compila** con **55 crate** nel grafo e conclude *«55 crate di superficie non verificabile contro un esecutore che nel prototipo è ~30 righe»*. Il codice di oggi dà ragione a 0031: `simulator` ha **una** dipendenza (`kernel`), **512 righe** scritte a mano, e `madsim` non compare né in `Cargo.lock` né in `crates/`. ⛔ **Non è il #57**: là la decisione è **anteriore** a ciò che nomina e la smentita arriva dopo, dal codice; qui la smentita è **contemporanea** e sta in un altro file della stessa cartella. ⚠️ **Nessuno dei due ADR nomina l'altro**, quindi la contraddizione non è visibile da nessuno dei due lati. 📌 **La domanda che lo coglie, e nessuna delle quattro del pre-controllo la fa:** *questo ADR è stato letto contro i suoi **fratelli**, non solo contro il codice?* Le quattro domande guardano tutte il compito contro il codice |
| 60 | ⛔ **Una guardia di non-vacuità dentro un blocco `END` di `awk` non può, per costruzione, difendere dall'input che MANCA.** Misurato il 2026-08-11: le **sei** asserzioni di §8.6.1 vivono in due passate `awk` sulla spec, e ognuna delle loro guardie — *«delimiter not found»*, *«rows==0»*, *«defends==0»* — sta in `END`. Quando il file non si apre, `awk` emette un **fatal** ed **`END` non gira**: le variabili restano vuote, `[ -z … ]` è vero, nessun `report`, e `check-docs.sh` esce **0** — `GATE GREEN` con sei asserzioni morte in silenzio. ⚠️ È il nemico di **§8.6.2 un livello sopra**: quelle guardie proteggono da un'**intestazione rinumerata**, non da un **file rinominato**. 📌 Vale per ogni glob e ogni percorso letterale: la stessa cecità copriva i due controlli delimitati da `*.md` — `nullglob` è **off**, quindi rinominando la cartella delle spec i duplicati di sezione e **V30** davano **zero rossi** mentre tutte e ventiquattro le Q perdevano il metodo di verifica. Rimedio: l'esistenza si verifica **prima**, fuori da `awk` |
| 61 | ⛔ **Un pattern che àncora sul DELIMITATORE invece che sulla CHIAVE lascia passare la stessa cosa scritta in un'altra forma — e in TOML le forme sono due.** Misurato il 2026-08-11: `gate-attributes.sh` cercava un build script con `build[[:space:]]*=[[:space:]]*"`, e la **virgoletta doppia** era ciò che lo distingueva da `build = false`. Ma TOML ha anche la **stringa letterale fra apici singoli**: `build = 'gen.rs'` è **lo stesso valore**, e sfuggiva. ⛔ Provato invece che dedotto: una crate che lo dichiara così **costruisce su cargo 1.95.0 con exit 0** e il suo script **viene eseguito** — il file `output` della directory di build porta il `cargo:rustc-env` iniettato. Con gli altri cinque controlli **ciechi per costruzione** (il build script si compila per l'host, non aggiunge nodi al grafo, e `check-docs.sh` non legge codice), il cancello usciva **VERDE su sei su sei** con un build script che legge orologio, filesystem e ambiente **dentro il kernel**: è il **#28** riaperto da un carattere di quoting. 📌 La forma generale, e vale oltre TOML: **si àncora sulla chiave, mai sul delimitatore.** È il **#41** — *«un filtro che normalizza l'ingresso decide anche cosa il controllo può vedere»* — spostato dal filtro al **riconoscitore** |
| 62 | ⛔ **`comm` confronta per COLLAZIONE, e `sort -V` non produce quell'ordine: il falso positivo che ne esce è invisibile finché il controllo non ha qualcosa di vero da dire.** Misurato il 2026-08-11 su `check-docs.sh`: i due lati del controllo V30 erano ordinati con `sort -uV`, che mette `Q9` prima di `Q10` mentre la collazione fa l'inverso. Finché i due insiemi **coincidono**, `comm` non incontra mai una riga spaiata e **non emette nemmeno il proprio avviso di disordine** — il difetto è latente al 100 %. Togliendo a **`Q9`** il suo metodo di verifica, il controllo riportava **sedici** nomi (`Q9` … `Q24`), **quindici dei quali ce l'hanno**. 📌 È l'altra metà del **#24**: *un controllo che scatta dove non deve è peggio di uno assente, perché insegna a ignorare l'audit* — e qui lo fa **esattamente nel momento in cui serve**. ⚠️ Lo stato d'uscita di `comm` (**1**, *«input is not in sorted order»*) veniva scartato: un verdetto perso in una pipe |
| 63 | ⛔ **Una promessa di conformità provata SOLO nello stato in cui ogni guardia plausibile passa non è una promessa: è una coincidenza.** Misurato il 2026-08-11: le promesse **5** (*«un `outcome` senza `intent` è rifiutato»*, V6) e **8a** (*«una nota su un passo mai aperto è rifiutata»*) di `journal_contract.rs` costruiscono entrambe un giornale **vuoto** e chiedono subito il rifiuto. Una guardia che chieda *«l'archivio è vuoto?»* invece di *«questo passo ha un intento?»* le soddisfa **entrambe**. ⛔ **Sostituite così le guardie di `FileJournal::outcome` e `::note`, `cargo test --workspace --no-fail-fast` dà 32 target, 171 passati, ZERO falliti** — e la mutazione **è osservabile** (#54 provato in due direzioni): su archivio non vuoto accetta un esito e una nota per passi **mai aperti**, cioè fa sparire il dubbio che ADR-0007 esiste per rendere rilevabile. 📌 Stessa forma su `read_back`, che **otto blocchi su dieci** esercitano su un giornale a **un passo solo**, dove *«cerca questo passo»* e *«restituisci il primo record»* sono la stessa frase. 📌 **La domanda che le coglie:** *in quale altro stato del mondo questa asserzione resterebbe verde?* |
| 64 | ⛔ **Due criteri possono coprire ciascuno la propria metà e lasciare scoperto il BUCO FRA loro, e nessuna rilettura dell'uno o dell'altro lo mostra.** Misurato il 2026-08-11: `bincode` 2.0.1 è coperto da **RUSTSEC-2025-0141 — «Bincode is unmaintained»**, categoria `INFO` e non una vulnerabilità, emessa il **2026-01-07**. L'avviso era pubblico **sette mesi prima** che §6.1.1 fosse riconfermata il 2026-08-08, e nessuno l'ha visto benché il criterio esistesse: **ADR-0037** chiede *«il pari ha un lettore conforme e **mantenuto**?»* — ma lo punta **verso il pari** (TypeScript, M-11); **M-1** puntava verso di noi e chiedeva un'altra cosa, *«il grafo transitivo è accettabile per I3?»*. ⛔ **Nessuno dei due chiede se sia mantenuta la libreria del NOSTRO capo del filo**, e `gate-deps.sh` verifica **quali** crate ci sono, non **come stanno**. ✅ **Il costo di agire è quasi zero oggi e cresce da solo:** `bincode` ha **zero usi di produzione** — un commento in `ports/ipc.rs` e la sonda `dependencies_usable.rs` — perché lo schema del canale `ipc` è il **Traguardo 6**. È una finestra che si chiude da sola, come la quarta proprietà di §3. 📌 **La domanda:** *questo criterio è puntato verso entrambi i capi del filo?* |
| 65 | ⛔ **Due difetti con la stessa CAUSA non hanno la stessa COPERTURA, e un rapporto che li raggruppa per causa fa scrivere un rimedio che ne prova uno solo.** Misurato il 2026-08-17 chiudendo T-2: l'audit elenca le promesse **5** e **8a** come un finding, e ha ragione sulla **causa** — `outcome` e `note` condividono `has_intent` in entrambe le implementazioni, quindi una guardia cieca le acceca insieme. ⛔ **Ma la suite muore alla PRIMA promessa rotta**, quindi un bugiardo cieco su tutt'e due muore sulla 5 e il blocco della 8a resta **non provato mentre un test afferma il contrario**: servono **due** bugiardi, ciascuno che superi l'altro blocco **sui propri meriti**. ⚠️ Il rimedio dettato dal rapporto è **giusto**; a essere dimezzata è la sua **prova**, e si vede solo scrivendola. 📌 Un tipo solo con **due istanze**, o sarebbe lo stesso difetto scritto due volte (#45 dall'altro lato). 📌 **La domanda:** *quanti di questi blocchi un solo bugiardo riesce a raggiungere?* |
| 66 | ⛔ **Chiudere un difetto non rende rosse le sonde che vi poggiavano: alcune diventano VACUE, e quella direzione non si vede.** Misurato il 2026-08-18 chiudendo K-1. Il rapporto prevedeva **due** sonde permanenti rosse; ne è diventata rossa **una**. L'altra — che scriveva la scadenza **dal banco** invece di lasciarla dichiarare all'attività — non aveva più nulla da esercitare, e **restava verde**: provato mutando `until <= instant` in `until < instant`, cioè **la discriminazione che il suo commento dichiara di difendere**, con la vecchia forma **verde** mentre la stessa mutazione faceva rossi **cinque** altri test. ⚠️ **Non è il #63:** lì la sonda nasceva vacua, qui **lo diventa**, e a renderla tale è il rimedio stesso — una sonda che ieri mordeva oggi non morde più, e nessun rosso lo annuncia. ⛔ **Nessuna delle sette domande del pre-controllo lo coglie**, perché guardano il compito contro il codice, mai **le sonde che poggiavano sul difetto che stai chiudendo**. 📌 **La domanda, e va fatta prima di correggere:** *quali sonde passano ATTRAVERSO questo difetto, e quali di esse resteranno verdi senza più provare nulla?* Poi si mutano, una per una. |
| 67 | ⛔ **Una giustificazione scritta su un ELENCO di nomi si legge come verificata su tutti, e basta che regga sul primo.** Misurato il 2026-08-18 chiudendo P-1: il `Debug` di `RecordV1` giustifica quattro campi con una ragione sola — *«`kind`, `effect`, `trust` e `reason` sono il vocabolario del kernel, **nobody outside chose them**»* — vera per **tre**, falsa per `reason`, che lo sceglie il **chiamante**. Il risultato è che A3 era dichiarata CHIUSA mentre il testo esterno usciva dalla **giustificazione** invece che dal payload. ⚠️ **Non è il #29:** lì una partizione lascia scoperto un membro che non appartiene a nessuna categoria; qui **una sola affermazione è quantificata su un elenco**, e la frase non dice **per quale nome** è stata controllata. 📌 **E ha un secondo tempo, sul RIMEDIO:** il rapporto proponeva `reason: &Instruction`, che **non chiude nulla** — `Instruction::new` è `pub`, quindi la guardia è soddisfatta dalla via **A1/A2**, dichiarata non chiudibile **dieci righe sopra**. **Una guardia a newtype vale esattamente quanto il suo COSTRUTTORE**, e una che non lo dice compra l'apparenza di una chiusura sopra una strada già dichiarata aperta. 📌 **Le due domande:** *questa ragione l'ho verificata su OGNI nome dell'elenco, o sul primo?* e *questo tipo, chi può costruirlo?* |
| 68 | ⛔ **Una regola scritta in un documento non vincola quel documento, e la sua violazione più vecchia è quasi sempre lì dentro.** Misurato il 2026-08-18: `CLAUDE.md` alla riga **59** scrive *«lo stato corrente e il prossimo passo stanno nella §6 del compendio — **non qui, o si disallineano**»*, e alla riga **13** — quarantasei righe **sopra**, nello stesso file — dava l'audit per **prossimo passo**. Stessa forma nella §12 di questo file, che prescrive *«quando si sposta il prossimo passo si cerca `grep '⏭️'` su tutti i documenti di stato e si guardano TUTTE le case»* mentre ne teneva **quattro** ferme. ⛔ **Non è distrazione, è strutturale:** una regola si scrive **guardando gli altri** — nasce per correggere qualcun altro — quindi il documento che la ospita non viene mai riletto **contro di essa**. ⚠️ **Non è la radice R1:** lì una **correzione** non attraversa gli **altri** documenti; qui una **regola** non attraversa il **proprio**. 📌 **La domanda, e costa una rilettura:** *questo documento rispetta la regola che contiene?* 📌 **E il corollario sul rimedio:** se la regola dice *«vive in un posto solo»*, riallineare le copie **ricrea lo stato in cui la regola è di nuovo l'unica difesa** — il rimedio è toglierle |
| 69 | ⛔ **Uno strumento che muta più file e RIFERISCE mentre scrive può fallire rumorosamente e lasciare comunque l'insieme applicato a metà: l'errore non è nell'ingresso, è nell'USCITA.** Misurato il 2026-08-18: uno script che applicava ventiquattro sostituzioni su sette file stampava una riga di verbale **dopo ogni scrittura**, e la riga conteneva una freccia. Su una console `cp1252` `print` solleva `UnicodeEncodeError`, e ha sollevato **dopo il primo file**: `CLAUDE.md` scritto, gli altri sei no, **exit 1**. ⚠️ **Non è il #48**, che descrive un banco che sbaglia **verso l'attesa, in silenzio**, e di cui ci si fida perché conferma: qui il guasto è **rumoroso** — traceback pieno, uscita diversa da zero — e proprio per questo si legge come *«non ha fatto niente»* mentre l'insieme è **parzialmente applicato**. ⛔ **Nessun contatore di sostituzioni lo coglie**, perché le sostituzioni erano tutte giuste. 📌 **Il contro-verso è di ORDINAMENTO, non di codifica:** uno strumento che muta file **non stampa nulla finché non ha finito di scrivere** — si accumula il verbale e lo si emette alla fine. Riparare la sola codifica chiude **quella** freccia e lascia aperta la classe. ✅ Ripristinato con `git checkout --`, lecito **solo** perché quei file non portavano lavoro non committato: è la dodicesima forma del **#48** al rovescio, e la condizione va detta perché non è la regola |
| 70 | ⛔ **Il `grep` con cui si ricontano le case di una cifra restituisce CANDIDATE, non case — e correggerle tutte rompe contenuto giusto.** Misurato il 2026-08-18 riallineando i pesi della §12. La cifra dei tre file da leggere è **353**, e `grep -rn "353"` ne riporta **sei**: quattro sono pesi, **due no** — `audit-2026-08-11.md` e `riferimenti.md` scrivono `COMPENDIO.md:353`, che è un **numero di riga** dentro una citazione esatta del finding **A-2**. Riscriverle avrebbe rotto due riferimenti corretti mentre si credeva di riallineare un peso. ⚠️ **E il difetto vive dentro il RIMEDIO, che è ciò che lo rende utile:** *«ricontare le case col `grep` invece che dal verbale precedente»* è la cura che la **ventesima** e la **ventiseiesima** misura prescrivono contro il **#31**, cioè contro il gotcha più frequente del progetto — e nessuno ne aveva scritto il modo di fallire. ⛔ **La causa è strutturale, non distrazione:** più una cifra è un **numero nudo** — 322, 353, 719 — più è probabile che una ricerca testuale peschi qualcosa che è un numero nudo per un'altra ragione: una riga, un byte, una percentuale, un identificatore. ⚠️ **Non è il #41**, dove un filtro **esclude** ciò che il controllo dovrebbe vedere: qui **include** roba di un'altra specie, e il danno va nella direzione opposta — non un falso negativo silenzioso, ma una **correzione sbagliata scritta con sicurezza**. ⚠️ **E non è il #48**, che descrive un banco di **misura**: qui lo strumento ha risposto correttamente alla domanda che gli è stata posta. 📌 **Il contro-verso, e costa una lettura:** il `grep` trova dove **guardare**, mai cosa **cambiare**. Ogni occorrenza si legge in faccia prima di toccarla, e il conteggio delle case si scrive **dopo** averle guardate. ⛔ **E la seconda forma, misurata mezz'ora dopo la prima: il `grep` TROVA la casa e chi legge non la guarda, perché l'uscita è TRONCATA.** Il censimento di `⏭️` riportava `roadmap.md:128`, scartata dopo centoventi caratteri per via del filtro messo lì a rendere leggibile l'elenco; dentro c'erano **due** affermazioni false insieme. Nella prima forma il difetto è nell'uscita del `grep`, nella seconda nel **filtro** che gli si mette dopo — il **#41** applicato all'occhio invece che allo script. 📌 **Una riga trovata da un censimento si legge INTERA, o il censimento non è stato fatto** |
| 71 | ⛔ **Un documento che DICHIARA una divergenza si legge come se le avesse dichiarate tutte, e la seconda passa senza che nessuno la cerchi.** Misurato il 2026-08-18, scrivendo il piano del Traguardo 5 contro il codice. Il disegno dichiara per esteso la divergenza dalla **§5.1** — *«i tre addendi sono parametri consegnati»*, e ne consegna **uno** — con tabella, argomento e costi, e la marca *«registrata perché il proprietario possa ribaltarla vedendola»*. ⛔ **E tace su quella gemella dalla §5.2**, dove i **due** campi `preemptible` e `release_grace` diventano **uno**, `Preemption::Never \| After(Millis)`. ⚠️ **La seconda è giusta nel merito** — la §5.3 punto 3 pretende che `InRevoca` sia *«non rappresentabile»* e un booleano non può renderlo tale — quindi non è un errore di scelta: è che il proprietario **non l'ha vista**, e una divergenza vista è ribaltabile mentre una taciuta no. ⛔ **Non è il #58**, dove un documento non ha guardato il codice: qui l'ha guardato, ha scelto bene, e non l'ha scritto. ⛔ **Non è il #67**, dove una **giustificazione** è quantificata su un elenco di nomi ed è vera solo del primo: qui non c'è nessun elenco, c'è un documento la cui **postura dichiarativa** su una voce si estende in silenzio a tutte. 📌 **La causa è di lettura e non di scrittura:** un lettore che incontra un riquadro *«divergenza dichiarata»* ne ricava che l'autore **stava cercando** le divergenze, e smette di cercarle lui — la stessa dinamica per cui un documento che ha letto le guardie *«si sente verificato»* (#58). 📌 **La domanda, e costa una rilettura del documento contro la propria fonte:** *quali ALTRE divergenze ha questo documento, oltre a quella che dichiara?* |
| 72 | ⛔ **Una DIREZIONE di prova tenuta da una MUTAZIONE è tenuta da niente: la mutazione si revoca, e il verbale resta a dire che la riga è chiusa.** Misurato il 2026-08-19, eseguendo il Task 4 del Traguardo 5. La riga di catalogo `V4` — *«l'esito trattato come due vie invece di tre»* — ha la direzione negativa tenuta da un caso `compile_fail` **permanente**, e la positiva — *«distinguere le tre compila»* — era stata chiusa citando **la mutazione 1 del passo di verifica**, cioè un'aggiunta temporanea al codice che lo stesso passo prescrive di **revocare**. ⛔ **E la regola che lo vieta era scritta QUATTRO RIGHE SOTTO, nello stesso file**, per la riga vicina: *«una mutazione sparisce quando la revochi»* — che è il gotcha **#68** (una regola non vincola il documento che la ospita) applicato non a un documento intero ma a **due celle adiacenti di una tabella**. ⚠️ **Non è il #14**, dove un controllo non si è mai visto fallire: qui il controllo si è visto fallire **una volta**, sotto la mutazione, e quel rosso è stato scambiato per una copertura. ⚠️ **E non è il #45**, dove il rimedio a una copertura mancante nasce non provato: qui non è mai nato nulla di permanente. 📌 **La forma generale, e vale per ogni tabella a due direzioni:** una mutazione **prova** che una sonda morde, non **è** la sonda. Ciò che chiude una direzione deve **restare nel repository dopo il commit**. 📌 **La domanda, e costa una lettura della propria tabella:** *ciò che chiude questa direzione esiste ancora quando la passata è finita?* |

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
| il **verbale del primo audit completo** — le quattro radici, i finding con causa radice e dimostrazione, ciò che è stato verificato **pulito**, e la §8 con le otto decisioni, **tutte eseguite** fra il 2026-08-17 e il 2026-08-18. ⛔ **Si apre per il METODO, non per il compito:** è il posto in cui si legge come un rimedio si prezza leggendo il codice invece del rapporto — più piccolo, più grande, o di specie diversa. ⚠️ **Questa cella diceva *«COSA DEVI FARE ADESSO … ne restano tre … è il prossimo passo»***, corretta il 2026-08-18 | [`audit-2026-08-11.md`](audit-2026-08-11.md) — oggi una **consultazione** | 31 KB |
| il **perché** di una decisione, le alternative scartate, i costi accettati | `docs/adr/<numero>-*.md` — **uno solo** | 2–19 KB l'uno |
| il **come** del sotto-progetto 1: §0–§8 con le evidenze delle misure | [`specs/2026-08-06-sottoprogetto-1-kernel.md`](superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md) — ⚠️ **a sezioni, mai intera** | 277 KB |
| ⛔ **il perimetro del Traguardo 5** — l'arbitro: quanto ne costruisce, le forme che la §5 descrive a parole, e per ogni artefatto **il controllo che lo esercita**. ⛔ **Si legge PRIMA di scriverne il piano**, ed è il file da cui si riprende | [`specs/2026-08-18-…-traguardo-5-arbitro-gpu-design.md`](superpowers/specs/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu-design.md) — ⚠️ **non è una spec**: è lo scaglionamento e le forme che la §5 non fissa | 31 KB |
| ⛔ **come si ESEGUE il Traguardo 5** — tredici compiti in cinque parti, col codice per ogni passo, le mutazioni da provare e i comandi. ⚠️ **L'errata in testa si legge PRIMA del compito**, e il pre-controllo del piano — le sette voci — sta subito sotto | [`plans/2026-08-18-…-traguardo-5-arbitro-gpu.md`](superpowers/plans/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu.md) — ⚠️ **a compiti, mai intero** | 174 KB |
| ⛔ **il perimetro del Traguardo 4** — quanto ne costruisce, dove vive ciascun pezzo, e per ogni artefatto **il controllo che lo esercita**. Si legge **prima** di scriverne il piano | [`specs/2026-08-11-…-traguardo-4-simulatore-dst-design.md`](superpowers/specs/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst-design.md) — ⚠️ **non è una spec**: è lo scaglionamento che la §3 non fissa | 30 KB |
| il **cosa** del kernel: §0–§10 | [`specs/2026-08-06-kernel-design.md`](superpowers/specs/2026-08-06-kernel-design.md) | 44 KB |
| il testo integrale dei **gotcha** e delle **misure**, con i numeri | [`HANDOFF.md`](HANDOFF.md) — ⚠️ **a sezioni** | 223 KB |
| ⛔ **cosa una sezione deve incassare, prima di proporle una modifica** | [`HANDOFF.md`](HANDOFF.md) — il **consuntivo voce per voce**: cosa era stato deciso, dove è finito, e cosa resta da scrivere. È **autorevole**, e si legge **prima** di proporre, non dopo | ⚠️ **la sezione, non il file** |
| l'ordine dei dodici sotto-progetti e le dipendenze | [`roadmap.md`](roadmap.md) | 29 KB |
| dove vive una funzionalità della mappa originale | [`tracciabilita.md`](tracciabilita.md) — ⚠️ **leggi il riquadro in testa**: risponde a «dove vive», **non** a «di quale meccanismo ha bisogno». È la crepa da cui sono uscite le sette voci | 15 KB |
| **dove vive ogni controllo** della porta, riga per riga sul catalogo §7.4, e cosa **non** è coperto | [`porta-di-qualita.md`](porta-di-qualita.md) | 156 KB |
| ⛔ **perché un seme NON è un oracolo**, e cosa identifica un caso in ciascuna delle due campagne DST — al livello 2 *«un seme»* **non esiste** | [`semi-dst.md`](semi-dst.md) — ⚠️ **nasce vuoto**, e la riga vuota è deliberata | 6 KB |
| la **strategia di test** — è la fonte di verità sulla porta di qualità, e mappa Q1–Q24 → metodo | [`design/08-strategia-di-test.md`](design/08-strategia-di-test.md) | 11 KB |
| la **topologia dei processi** — contiene la tensione che F1b deve conciliare | [`design/01-topologia-dei-processi.md`](design/01-topologia-dei-processi.md) | 5 KB |
| gli altri diagrammi della struttura | [`design/`](design/) — nove file | 4–11 KB l'uno |
| gli **esiti degli spike**, con seed, versioni e comandi | [`../spikes/RISULTATI.md`](../spikes/RISULTATI.md) | 23 KB |
| i requisiti della GUI, G1–G21 e P1–P4 | [`../spikes/GUI-REQUISITI.md`](../spikes/GUI-REQUISITI.md) | 6 KB |
| la **provenienza** di ciò che non abbiamo dedotto noi, con le date | [`riferimenti.md`](riferimenti.md) | 192 KB |
| il **modello** di come si scrive un piano qui, con l'errata in testa | [`plans/2026-08-06-spike-linguaggio-del-core.md`](superpowers/plans/2026-08-06-spike-linguaggio-del-core.md) | 68 KB |
| ⛔ **cosa il piano del Traguardo 1 detta e il repository smentisce** — quattro voci, prima fra tutte gli identificatori italiani | [`plans/2026-08-08-sottoprogetto-1-traguardo-1-scheletro-e-porta.md`](superpowers/plans/2026-08-08-sottoprogetto-1-traguardo-1-scheletro-e-porta.md) — ⚠️ **solo l'errata in testa**, il resto è eseguito | 50 KB |
| ⛔ **come si esegue un piano qui, e le quattro specie di difetto** — è il piano del Traguardo 2, **eseguito per intero**, con quarantanove voci di errata in sei passate | [`plans/2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md`](superpowers/plans/2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md) — ⚠️ **a compiti, mai intero**: è il **secondo file più grande** del repository, dopo la spec | 162 KB |
| ⛔ **come si esegue un piano, e come si CHIUDE un traguardo** — è il piano del Traguardo 3, **eseguito per intero**, dodici compiti su dodici. ⚠️ **L'errata in testa si legge prima del compito**, ed è a **settantasette voci in nove passate**, di cui **nove decisioni**; le ultime tre sono la **Definizione di «fatto» che invecchia** | [`plans/2026-08-10-sottoprogetto-1-traguardo-3-giornale-e-formato-durevole.md`](superpowers/plans/2026-08-10-sottoprogetto-1-traguardo-3-giornale-e-formato-durevole.md) — ⚠️ **a compiti, mai intero** | 168 KB |
| ⛔ **come si esegue un piano quando il pre-controllo trova un difetto in DIECI compiti su dieci** — è il piano del Traguardo 4, **eseguito per intero**. ⚠️ **L'errata in testa è a settanta voci in nove passate, di cui dodici DECISIONI**, e si legge **prima** di riaprire qualunque cosa che quel traguardo abbia toccato | [`plans/2026-08-11-…-traguardo-4-simulatore-dst.md`](superpowers/plans/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst.md) — ⚠️ **a compiti, mai intero** | 114 KB |
| l'indice di ADR e diagrammi | [`README.md`](README.md) | 18 KB |
| ⛔ **il messaggio da incollare all'inizio di una chat**, e il perché di ogni sua riga | [`AVVIO-CHAT.md`](AVVIO-CHAT.md) — ⚠️ il **messaggio** ne è **14,1** (14460 byte LF): `−1281 B` il 2026-08-18, la consegna all'esecuzione | 25 KB |

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

> 🔁 **Ottava misura, il 2026-08-09, chiudendo il Task 7 — e stavolta il #31 non si è ripetuto.**
> È la prima rimisura fatta **applicando la lezione della settima**: si aggiornano le celle **e**
> si contano le **righe**, e si scrive nella tabella prima che nel verbale. Nessuna riga mancava:
> quella del piano del Traguardo 2, aggiunta due misure fa, ha retto al primo controllo.
>
> | | |
> |---|---|
> | **cresciuti** | [`HANDOFF.md`](HANDOFF.md) `113 → 117` per i gotcha **#44** e **#45** · [`porta-di-qualita.md`](porta-di-qualita.md) `15 → 17` per le sei sonde `R` · il **piano del Traguardo 2** `131 → 135` per l'errata del Task 7 |
> | **invariati, ricontati** | spec del sotto-progetto 1 267 · kernel-design 44 · roadmap 15 · tracciabilità 15 · riferimenti 32 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · README 10 · ADR `2–19` |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **593** a **597 KB** (611081 B). I **due file
> obbligatori** passano da 97 a **101 KB**: la cifra vive in `CLAUDE.md` e in
> [`AVVIO-CHAT.md`](AVVIO-CHAT.md), ed è aggiornata in entrambi.
>
> 📌 **E una cifra tonda è una trappola in arrivo**, quindi si dice adesso: i due file
> obbligatori hanno passato i **101 KB**, e la frase che li accompagna li prezza *«circa
> venticinquemila token»* col rapporto della prima volta. Il rapporto **non è stato rimisurato
> da quando fu fissato**, e nessuno lo dubita perché la regola che sostiene è giusta — che è la
> definizione esatta del **#31**. ⚠️ Resta comunque il confronto che conta, ed è **101 KB contro
> 597**.

> 🔁 **Nona misura, il 2026-08-09, chiudendo la sessione dei Task 8–10 del Traguardo 2 — e la
> riga mancante della settima non si è ripetuta.** Contate le **righe** prima dei numeri dentro
> di esse, come pretende la lezione della settima: ogni file citato in questa sezione ha la sua
> voce in tabella, verificato uno per uno. Nessuna riga da aggiungere.
>
> | | |
> |---|---|
> | **cresciuto** | [`HANDOFF.md`](HANDOFF.md) `117 → 119`, per il gotcha **#46** — YAGNI che, su una porta mai implementata, cancella ciò che serve a implementarla |
> | ⛔ **e tre righe che questo riquadro dichiarava invariate lo erano solo a metà passata** | [`porta-di-qualita.md`](porta-di-qualita.md) `17 → 27` per gli undici artefatti registrati · [`riferimenti.md`](riferimenti.md) `32 → 43` per le misure dei Task 7–10 · il **piano del Traguardo 2** `135 → 147` per l'errata E19–E35 |
> | **invariati, ricontati a passata chiusa** | spec del sotto-progetto 1 267 · kernel-design 44 · roadmap 15 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · README 10 · ADR `2–19` |
>
> ⛔ **E la riga qui sopra è essa stessa il #31, colto mentre accadeva.** Questo riquadro fu
> scritto **a metà passata**, quando quei tre file erano ancora a HEAD, e dichiarava
> «invariati, ricontati» tre file che di lì a poco sarebbero cresciuti — per un lavoro **già
> in corso in parallelo**, e quindi prevedibile. 📌 **La regola che ne esce, e costa zero:** un
> verbale di misura si scrive **quando la passata è chiusa**, mai mentre altri stanno ancora
> scrivendo; e se lo si scrive prima, si rimisura prima di committare. Una misura vera **di un
> momento sbagliato** è indistinguibile da una misura falsa per chi la legge dopo.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **597** a **599 KB** (613166 B). I **due file
> obbligatori** passano da 101 a **108 KB**: **103 erano già a HEAD**, ed è la cifra che
> `CLAUDE.md` e [`AVVIO-CHAT.md`](AVVIO-CHAT.md) portano **oggi**; il resto lo aggiungono questo
> riquadro e le due questioni aperte della §6. ⚠️ **In quei due file la cifra dice ancora 103**,
> e va rifatta lì: vive in **tre** posti, e questa passata ne ha aggiornato **uno**.
>
> ⛔ **E il verbale è arrivato dopo i numeri, che è la variante successiva del #31.** Le cifre
> erano **giuste e già in tabella**; a mancare era il riquadro, perché la misura era stata
> scritta **solo nel messaggio del commit** — un posto che nessuno rilegge per decidere se
> aprire un file. Un numero corretto senza il proprio verbale non si può né rifare né dubitare:
> chi rimisura non sa da cosa parte. 📌 Il verbale è parte della misura, non il suo racconto.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi si rimisura **dopo**
> aver chiuso questo riquadro e si corregge **di sole cifre** — il metodo della sesta misura,
> alla seconda applicazione.

> 🔁 **Decima misura, il 2026-08-09, chiudendo la voce della regola B — e per la prima volta
> nessuna riga era stantia se non quelle che questa passata ha fatto crescere.** È anche la prima
> scritta **a passata chiusa**, che è la regola uscita dalla nona: le tre righe qui sotto sono
> state misurate quando non restava niente da scrivere, non mentre qualcuno scriveva ancora.
>
> | | |
> |---|---|
> | **cresciuti** | spec del sotto-progetto 1 `267 → 271` per la riga della **regola B**, il suo richiamo e i riallineamenti di §7.4.7 e §8.3 · [`HANDOFF.md`](HANDOFF.md) `119 → 120` per la terza occorrenza del **#36** · [`porta-di-qualita.md`](porta-di-qualita.md) `27 → 28` per la riga nuova e la nota di chiusura |
> | **invariati, ricontati** | kernel-design 44 · roadmap 15 · tracciabilità 15 · [`riferimenti.md`](riferimenti.md) 43 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · il **piano del Traguardo 2** 147 · README 10 · ADR `2–19` (2441 B e 19291 B) |
> | ✅ **righe contate prima dei numeri** | ogni file citato in §12 ha la propria voce in tabella, verificato uno per uno. **Nessuna riga da aggiungere** — la lezione della settima, alla seconda applicazione riuscita |
> | ✅ **e il residuo dichiarato dalla nona è chiuso** | la nona misura lasciava scritto *«in quei due file la cifra dice ancora 103»*: `CLAUDE.md` e [`AVVIO-CHAT.md`](AVVIO-CHAT.md) sono stati allineati, e la cifra oggi è la stessa in **tutti e tre** i posti |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **599** a **605 KB** (619105 B). I **due file
> obbligatori** passano da 108 a **111 KB**. Le due cifre vivono in `CLAUDE.md`, in
> [`AVVIO-CHAT.md`](AVVIO-CHAT.md) e qui, e sono aggiornate in tutti e tre — insieme alla spec,
> che in testa a questo file e in `CLAUDE.md` è nominata per nome (`270`).
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla terza
> applicazione. Regge perché ogni correzione qui è fra numeri della **stessa lunghezza**.
>
> 📌 **Cosa dice il #31 questa volta, ed è una notizia buona:** non si è ripetuto. Quattro misure
> di seguito senza artefatti dello strumento, due di seguito senza righe mancanti, e la prima
> senza celle stantie estranee alla passata. ⚠️ Il che è esattamente il momento in cui si smette
> di rimisurare — quindi resta scritto che la **prossima** si fa lo stesso.

> 🔁 **Undicesima misura, il 2026-08-09, chiudendo il Task 11 — e la crescita più grande non è
> quella che ci si aspetta.** Contate le **righe** prima dei numeri dentro di esse: ogni file
> citato in §12 ha la sua voce, nessuna da aggiungere. Scritta **a passata chiusa**, come pretende
> la nona.
>
> | | |
> |---|---|
> | **cresciuti** | ⛔ [`porta-di-qualita.md`](porta-di-qualita.md) `28 → 38` — **il salto più grande di ogni misura finora**, e non per una porta nuova: dieci kilobyte sono la **campagna di mutazione** e i quattro esiti credibili e falsi del banco · [`HANDOFF.md`](HANDOFF.md) `120 → 127` per i gotcha **#47** e **#48** e le tre occorrenze nuove · [`riferimenti.md`](riferimenti.md) `43 → 47` per le misure del Task 11 · il **piano del Traguardo 2** `147 → 153` per l'errata E36–E41 |
> | **invariati, ricontati** | spec del sotto-progetto 1 271 · kernel-design 44 · roadmap 15 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · README 10 · ADR `2–19` (2441 B e 19291 B) |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **605** a **611 KB** (625805 B). I **due file
> obbligatori** passano da 111 a **117 KB**; la cifra vive in `CLAUDE.md`, in
> [`AVVIO-CHAT.md`](AVVIO-CHAT.md) e qui, ed è aggiornata in tutti e tre.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla quarta
> applicazione.
>
> 📌 **E il rapporto che la §12 esiste per difendere non si è mosso: 117 KB contro 611.** ⚠️ Ma un
> registro che cresce di dieci kilobyte in un compito solo è il **prossimo** candidato al #31 di
> un genere nuovo — non un numero stantio, un **documento che smette di essere letto perché è
> diventato troppo lungo**. Scritto qui perché chi lo noterà per primo abbia da dove partire.

> 🔁 **Dodicesima misura, il 2026-08-10, chiudendo il Task 12 — e per la prima volta un file è
> _sceso_.** Scritta a passata chiusa, righe contate prima dei numeri: nessuna voce da aggiungere.
>
> | | |
> |---|---|
> | ⛔ **sceso** | [`porta-di-qualita.md`](porta-di-qualita.md) **47 → 40**, e la storia è tutta lì: era arrivato a 47 KB e **531 righe**, di cui **228 — il 43%** — di prosa su **una riga di tabella su tre**, mentre le due righe vicine nella stessa tabella ne hanno **zero**. Riportato a **449 righe** con quindici intestazioni invece di cinque. In tabella la cella passa da 38 a **40**, perché 38 era il valore dell'undicesima misura: il picco a 47 non è mai stato scritto qui |
> | **cresciuti** | [`HANDOFF.md`](HANDOFF.md) `127 → 129` per le tre forme nuove del **#48** · il **piano del Traguardo 2** `153 → 158` per l'errata E42–E46 |
> | **invariati, ricontati** | spec del sotto-progetto 1 271 · kernel-design 44 · roadmap 15 · tracciabilità 15 · [`riferimenti.md`](riferimenti.md) 47 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · README 10 · ADR `2–19` |
>
> ⛔ **E questa misura ha trovato un residuo dell'undicesima, che è il #31 nella forma che la
> SESTA misura aveva scritto per impedirlo.** L'undicesima aveva portato l'aggregato da 605 a
> **611** e lo aveva scritto **solo nel proprio riquadro**: in testa a questo file e in
> `CLAUDE.md` — cioè **dove qualcuno legge per decidere se aprire** — era rimasto **605**. La
> cifra dei due file obbligatori, invece, era stata propagata in tutti e tre i posti. 📌 Quindi
> non è che la lezione non ci fosse: **c'era, scritta da me, e ho applicato metà del rimedio**.
> Un aggregato ha **due** case e un numero solo ne ha una, e la seconda si dimentica proprio
> perché la prima è stata fatta.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **611** a **614 KB** (628305 B), ed è corretto
> **in tutti e tre i posti** questa volta. I **due file obbligatori** passano da 117 a **123 KB**.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla quinta
> applicazione.
>
> 📌 **E il registro che scende è la notizia della misura.** Undici misure di seguito hanno
> registrato solo crescite, e questa è la prima volta che un documento è stato **ridotto perché
> aveva smesso di rispondere alla propria domanda**. ⚠️ La regola che ne esce vale oltre il caso:
> un file che cresce non è un problema, un file che cresce **in una sezione sola** lo è — e il
> segnale non è il peso, è **la sproporzione fra righe vicine della stessa tabella**.

> 🔁 **Tredicesima misura, il 2026-08-10, chiudendo la sessione — ed è la passata di
> _manutenzione_, quella che di solito non si misura.** Nessun compito eseguito: solo il
> riallineamento dei documenti di stato che erano rimasti al **Task 6**.
>
> | | |
> |---|---|
> | **cresciuti** | [`riferimenti.md`](riferimenti.md) `47 → 52` per le misure del Task 12 · [`HANDOFF.md`](HANDOFF.md) `129 → 131` per lo stato e i richiami · [`roadmap.md`](roadmap.md) `15 → 16` e [`README.md`](README.md) `10 → 11`, che erano **fermi sei compiti indietro** |
> | **invariati, ricontati** | spec 271 · kernel-design 44 · tracciabilità 15 · [`porta-di-qualita.md`](porta-di-qualita.md) 40 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · il piano del Traguardo 2 158 · ADR `2–19` |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **614** a **615 KB** (630190 B). I **due file
> obbligatori** passano da 123 a **124 KB**, mossi da questo riquadro stesso; rimisurati
> **dopo** averlo chiuso e corretti **di sole cifre**, come sempre.
>
> ⛔ **E la notizia di questa misura è quali file erano stantii.** `roadmap.md` e `README.md`
> dicevano *«eseguito fino al Task 6, si riprende dal Task 7»* mentre il repository era al
> **12**: sei compiti di scarto, per **due sessioni**. Nessuno se n'era accorto perché la §6
> del compendio era **giusta**, e chi lavora legge quella. ⚠️ **Ma non è chi lavora il loro
> lettore**: `README.md` è la porta d'ingresso di chi arriva, e `roadmap.md` è dove si guarda
> per decidere **cosa viene dopo**. 📌 La regola che ne esce, ed è la ragione per cui questa
> passata esiste: **la §6 giusta non protegge gli altri documenti di stato — li nasconde**,
> perché toglie a chi lavora ogni occasione di incontrarli stantii.

> 🔁 **Quattordicesima misura, il 2026-08-10, chiudendo il Traguardo 2 — e porta la misura che
> tredici riquadri avevano rimandato.** Scritta a passata chiusa, righe contate prima dei numeri
> dentro di esse: ogni file citato in questa sezione ha la sua voce, nessuna da aggiungere.
>
> | | |
> |---|---|
> | **cresciuti** | [`HANDOFF.md`](HANDOFF.md) `131 → 137` per il gotcha **#49**, il blocco di lascito del Traguardo 2 e lo stato · il **piano del Traguardo 2** `158 → 162` per l'errata E47–E49 · [`roadmap.md`](roadmap.md) `16 → 17` e [`porta-di-qualita.md`](porta-di-qualita.md) `40 → 41` |
> | **invariati, ricontati** | spec del sotto-progetto 1 271 · kernel-design 44 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · README 11 · ADR `2–19` (2441 B e 19291 B) |
> | ⛔ **e una riga che questo riquadro aveva già dichiarato invariata** | [`riferimenti.md`](riferimenti.md) `52 → 54`, per i comandi del riconteggio del catalogo, scritti lì **dopo** che questo riquadro era chiuso. È il difetto della **nona** misura — *«un verbale si scrive quando la passata è chiusa, mai mentre altri stanno ancora scrivendo»* — ripetuto da me contro me stesso, a distanza di quattro riquadri e con la regola scritta di mia mano. 📌 Colto **prima** di committare, e questo è l'unico merito: la nona prescriveva *«e se lo si scrive prima, si rimisura prima di committare»*, ed è quella metà del rimedio ad aver funzionato |
> | ⛔ **una riga riscritta, non un numero** | la voce del piano del Traguardo 2 diceva *«il compito da cui si riprende — è il piano in corso, e **il Task 11** sta lì»*: era stantia di **due compiti** già prima di questa sessione, e la §6 nel frattempo era giusta. È il difetto della **tredicesima** misura — la §6 giusta nasconde gli altri documenti — spostato **dentro questo stesso file**, fra la §6 e la §12 |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **615** a **621 KB** (635880 B), corretto in
> **tutti e tre** i posti. I **due file obbligatori** passano da 124 a **133 KB**.
>
> ⛔ **E il #31 che `CLAUDE.md` dichiarava «il prossimo in arrivo» è arrivato, ed è peggio di
> come la riga lo prezzava.** Quella riga dice *«circa venticinquemila token»* per i due file,
> col rapporto della prima volta, e ammette che **il rapporto non è mai stato rimisurato**.
> Misurato oggi, indirettamente ma senza ambiguità: leggendo questo file, **quattrocento righe
> hanno pesato `25148` token** — cioè da sole quanto la riga attribuisce ai **due file interi**.
> Questo file ne ha **milleduecentosessantaquattro**.
>
> ⚠️ **Non scrivo il totale, e la ragione è la regola di questo repository.** Lo strumento che
> ha prodotto quel numero è il lettore di file di un agente, non un contatore di token dedicato,
> e la densità cambia per sezione — le tabelle costano più della prosa. Un totale calcolato da
> un campione sarebbe **un'ipotesi scritta come misura**, cioè il gotcha #15. Quel che è
> **certo** è il limite inferiore, ed è quello che serve a decidere: i due file obbligatori
> costano **almeno tre volte** ciò che la loro riga dichiara. 📌 La cifra esatta la scriverà chi
> passerà un contatore vero su entrambi; fino ad allora la riga porta il limite inferiore, come
> la frase in testa porta «oltre mezzo megabyte» invece di un totale.
>
> 📌 **E il confronto che la §12 esiste per difendere regge lo stesso, anzi meglio di prima:**
> **133 KB contro 621**. Il rapporto sbagliato non ha mai messo in pericolo la regola che
> sosteneva — ed è precisamente perché la regola era giusta che nessuno ha dubitato del numero.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo**
> aver chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> sesta applicazione.

> 🔁 **Quindicesima misura, il 2026-08-10, scrivendo il piano del Traguardo 3 — ed è una passata
> con una riga nuova, che è il modo di aggiornare una tabella che si dimentica.** Scritta a
> passata chiusa; righe contate prima dei numeri dentro di esse.
>
> | | |
> |---|---|
> | ⛔ **tre righe aggiunte** | il **piano del Traguardo 3**, **92 KB** — il file da cui si riprende · [`AVVIO-CHAT.md`](AVVIO-CHAT.md) **12 KB** · e i **pesi degli spike**, `RISULTATI.md` **23** e `GUI-REQUISITI.md` **6**, che avevano la voce e la **cella vuota** da sempre |
> | **cresciuti** | [`HANDOFF.md`](HANDOFF.md) `137 → 138` · [`roadmap.md`](roadmap.md) `17 → 18` |
> | **invariati, ricontati** | README 11 · [`riferimenti.md`](riferimenti.md) 54 · [`porta-di-qualita.md`](porta-di-qualita.md) 41 · spec 271 · kernel-design 44 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · il piano del Traguardo 2 162 · ADR `2–19` |
> | **e un conteggio stantio trovato di rimbalzo** | la tabella dei piani di [`roadmap.md`](roadmap.md) diceva *«errata di quarantasei voci in quattro passate»*: con E47–E49 sono **quarantanove in sei**. Non l'ha trovato un controllo — l'ha trovato il fatto che quella riga andava toccata comunque per aggiungerne una accanto |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **621** a **622 KB** (637165 B). I **due file
> obbligatori** passano da 133 a **139 KB**.
>
> ⛔ **E la riga di `AVVIO-CHAT.md` mancava da sempre, mancata da SEI misure di seguito che
> dichiaravano di averla cercata.** Dalla nona in poi ogni riquadro porta la frase *«ogni file
> citato in questa sezione ha la sua voce in tabella, verificato uno per uno»* — e
> [`AVVIO-CHAT.md`](AVVIO-CHAT.md) è citato **dieci volte dentro la §12 stessa**, oltre che in
> §6, §9 e §13. ⚠️ **Non l'ha trovata chi rimisurava: l'ha trovata una revisione esterna**, e la
> differenza è il punto. 📌 **La forma del #31 che ne esce, ed è nuova:** una verifica ripetuta
> uguale sei volte **non è sei verifiche** — è una sola, ripetuta. Chi controlla la propria
> tabella parte dalle righe che ci sono e ne verifica i numeri; per accorgersi di una riga
> **assente** bisogna partire dall'**altro capo** — dall'elenco dei file citati — ed è un
> movimento che nessuna delle sei ha fatto. Chi rimisura la prossima volta parta di lì.
>
> 📌 **Altre due righe assenti, fuori dalla §12 e pre-esistenti, chiuse nella stessa passata:**
> la mappa dei documenti di [`HANDOFF.md`](HANDOFF.md) non elencava **questo file** né
> `AVVIO-CHAT.md`, e la tabella «Dove va cosa» di [`README.md`](README.md) non elencava
> `superpowers/plans/`, cioè **la cartella da cui si riprende il lavoro**.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla settima
> applicazione.

> 🔁 **Sedicesima misura, il 2026-08-10, chiudendo il Task 2 del Traguardo 3 — ed è la prima
> aperta perché una revisione ha chiesto il verbale, non perché chi scriveva se ne fosse
> accorto.** Scritta a passata chiusa; righe contate prima dei numeri dentro di esse, e
> **partendo dall'elenco dei file citati** — il movimento che la quindicesima ha prescritto:
> diciotto citazioni nella §12, diciotto righe, **nessuna assente**.
>
> | | |
> |---|---|
> | **cresciuti** | spec del sotto-progetto 1 `271 → 274` · [`HANDOFF.md`](HANDOFF.md) `138 → 142` · [`riferimenti.md`](riferimenti.md) `54 → 62` · [`porta-di-qualita.md`](porta-di-qualita.md) `41 → 44` · il piano del Traguardo 3 `92 → 95`, per l'errata che il Task 1 gli ha messo in testa |
> | **invariati, ricontati** | kernel-design 44 · roadmap 18 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · il piano del Traguardo 2 162 · README 11 · [`AVVIO-CHAT.md`](AVVIO-CHAT.md) 12 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` (2441 B e 19291 B) · `design/` nove file `4–9` |
> | **una cella di testo, non di cifre** | la riga del piano del Traguardo 3 diceva *«dodici compiti, e il primo è il record durevole»*. Il primo **è eseguito**: ora dice da dove si riprende, che è la sola cosa che quella riga serve a dire |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **622** a **629 KB** (643910 B). I **due file
> obbligatori** passano da 139 a **143 KB**, e quella cifra vive in **cinque** posti — uno in
> `CLAUDE.md` e **quattro** in [`AVVIO-CHAT.md`](AVVIO-CHAT.md) — più il riquadro della
> quindicesima misura, che è **verbale** e non si tocca.
>
> ⛔ **E cinque cifre vivevano in cinque posti con tre valori diversi, che è la forma peggiore
> del #31.** Il Task 2 aveva rimisurato *«622 → 630»* e l'aveva scritto in **un posto solo**,
> lasciando `CLAUDE.md` a contraddirsi **a tre righe di distanza** — 622 alla riga 25, 630 alla
> 28 — più il riquadro in testa a questo file e la riga della spec in
> [`AVVIO-CHAT.md`](AVVIO-CHAT.md). ⚠️ **È letteralmente ciò che la sesta misura aveva scritto
> per impedirlo** — *«rimisurare non basta: si scrive dove qualcuno legge per decidere»* —
> ripetuto **dentro `CLAUDE.md`**, il file che quella regola contiene. Chi rimisura cerca la
> cifra vecchia con un `grep` **su tutto il repository**, non nel file che ha in mano.
>
> ⚠️ **E due delle cifre di quella rimisura erano sbagliate, ciascuna a modo suo.** La spec era
> **troncata** invece che arrotondata — il metodo della §12 è *«`wc -c`, arrotondati a KiB»* — e
> poi il numero è **sceso ancora**, perché la stessa revisione ha fatto accorciare un richiamo
> datato di venti righe: `276` alla revisione, **274** a passata chiusa. 📌 **La lezione è
> sull'ordine, non sull'aritmetica:** una rimisura fatta **prima** dell'ultima correzione è
> un'ipotesi, e vale anche quando le correzioni **riducono** invece di far crescere.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, all'ottava
> applicazione.

> 🔁 **Diciassettesima misura, il 2026-08-10, chiudendo il Task 8 del Traguardo 3 — e la notizia
> non è quanto sono cresciuti i file, è che NESSUNO li aveva rimisurati per cinque compiti.**
> I Task 3, 4+5, 6 e 7 hanno fatto crescere sei righe della §12 e **nessuna** è stata toccata:
> la tabella era già sbagliata **prima** che questa sessione cominciasse, e chi l'ha letta in
> mezzo ha deciso su cifre vecchie. ⚠️ **È il #31 nella forma che la sedicesima misura aveva
> descritto** — *«il numeratore lo muove chi esegue, e chi esegue non apre la §12»* — con la
> differenza che qui non è una riga: sono **sei**.
>
> | | |
> |---|---|
> | **quanto erano indietro AL COMMIT PRECEDENTE**, cioè prima di questo compito | [`porta-di-qualita.md`](porta-di-qualita.md) `44 → 71` (**+27**) · il piano del Traguardo 3 `95 → 125` (**+30**) · [`riferimenti.md`](riferimenti.md) `62 → 84` (**+22**) · [`HANDOFF.md`](HANDOFF.md) `142 → 153` (**+11**) · spec del sotto-progetto 1 `274 → 277` · [`README.md`](README.md) `11 → 12` |
> | **cresciuti da questo compito**, sopra quei valori | [`porta-di-qualita.md`](porta-di-qualita.md) `71 → 73` per la sonda nuova · il piano `125 → 136` per le dodici voci d'errata · [`riferimenti.md`](riferimenti.md) `84 → 89` per le misure del Task 8 · [`HANDOFF.md`](HANDOFF.md) `153 → 159` per i gotcha **#51** e **#52** e le tre righe del traguardo · [`roadmap.md`](roadmap.md) `18 → 19` · questo file `147 → 156` |
> | **invariati, ricontati** | kernel-design 44 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · il piano del Traguardo 2 162 · [`AVVIO-CHAT.md`](AVVIO-CHAT.md) 12 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · `adr/` 214 |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **629** a **649 KB** (664851 B) — e **644 era
> già il vero** al commit precedente. I **due file obbligatori** passano da 143 a **165 KB**, e
> **156 era già il vero**: la cifra vive in **cinque** posti, uno in `CLAUDE.md` e **quattro** in
> [`AVVIO-CHAT.md`](AVVIO-CHAT.md), cercati col `grep` su **tutto il repository** come la
> sedicesima misura prescrive.
>
> 📌 **Cosa cambierebbe se qualcuno volesse che questo non ricapiti**, dichiarato invece che
> promesso: la §13 dice di aggiornare la §12 *«se cambia dove guardare»*, e una crescita di
> venti KB **non cambia dove guardare** — quindi la regola, letta alla lettera, non è stata
> violata da nessuno dei cinque compiti. ⛔ Il rimedio non è un'esortazione ma un **controllo**:
> `check-docs.sh` sa già confrontare due insiemi, e confrontare una cifra dichiarata con
> `wc -c` è lo stesso mestiere. Non è stato scritto qui perché sarebbe una riga di catalogo
> nuova, e quella è una decisione del proprietario — **registrata, non presa**.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla nona
> applicazione.

> 🔁 **Diciottesima misura, il 2026-08-10, chiudendo il Traguardo 3 — ed è la prima passata in cui
> il metodo accumulato è stato eseguito PER INTERO e ha retto senza correzioni.** Scritta a
> **passata chiusa**, cioè dopo l'ultima riga di documentazione e non prima; righe contate
> **partendo dall'elenco dei file citati** e non dalle righe presenti — **diciotto** citazioni,
> **diciotto** righe, **nessuna assente**; `wc -c` **arrotondato** a KiB e non troncato;
> l'aggregato con le **due case**, cifra tonda e byte esatti.
>
> | | |
> |---|---|
> | **cresciuti** | [`porta-di-qualita.md`](porta-di-qualita.md) `73 → 95` — di nuovo il salto più grande, ed è il registro che assorbe la chiusura di un traguardo · [`HANDOFF.md`](HANDOFF.md) `159 → 175` per i gotcha **#53**, **#54**, **#55**, **#56** e le due tabelle di chiusura · il piano del Traguardo 3 `136 → 168` per le voci d'errata da **E50** a **E77** · [`riferimenti.md`](riferimenti.md) `89 → 106` per le misure dei Task 9, 10, 11 e 12 · [`roadmap.md`](roadmap.md) `19 → 22` · [`README.md`](README.md) `12 → 13` · [`AVVIO-CHAT.md`](AVVIO-CHAT.md) `12 → 13`, che non si muoveva **da sedici misure** · questo file `156 → 183` |
> | **invariati, ricontati** | spec del sotto-progetto 1 **277** — e l'invarianza è **il dato**, perché il Traguardo 3 ha toccato la spec l'ultima volta al Task 2 · kernel-design 44 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · il piano del Traguardo 2 162 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` (2441 B e 19291 B) · `design/` nove file `4–9` |
> | ⚠️ **una cella corretta DALLA MISURA e non dalla stima** | la riga del piano del Traguardo 3 era stata riscritta **prima** di misurare, con `162` preso dal piano gemello che gli somiglia. Il `wc -c` dice **168**. ⛔ È il gotcha **#15** dentro la passata che esiste per impedirlo: una cifra plausibile scritta prima della misura è un'ipotesi, e **somigliare a un file vicino non è misurarlo** |
> | ⚠️ **una cella di testo, non di cifre** | la riga del piano del Traguardo 3 diceva *«il compito da cui si riprende … i primi **otto** sono eseguiti, si riprende dal **Task 9**»* con **dodici** eseguiti: era stantia di quattro compiti, ed è la stessa specie che la dodicesima misura registrò per il piano del Traguardo 2. Ora dice cos'è quel file **adesso** — il modello di come si chiude un traguardo — che è la sola cosa che quella riga serve a dire quando il piano non è più in corso |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **649** a **665 KB** (681073 B). ⚠️ **Il
> denominatore è cresciuto senza che nessuno abbia scritto una riga di spec o un ADR:** i sedici
> kilobyte sono **tutti** di `HANDOFF.md`, cioè il costo di chiudere un traguardo, non di
> progettare.
>
> 📌 **E una cosa che questa passata dice e le altre diciassette no:** il rimedio proposto dalla
> diciassettesima — *«`check-docs.sh` sa già confrontare due insiemi, e confrontare una cifra
> dichiarata con `wc -c` è lo stesso mestiere»* — **non è stato scritto**, e in questa passata
> **sette** righe su diciotto erano di nuovo fuori. La proposta resta **registrata e non presa**:
> è una riga di catalogo nuova, e quella è una decisione del proprietario.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla decima
> applicazione.

> 🔁 **Diciannovesima misura, il 2026-08-10, chiudendo la sessione con l'audit di coerenza — ed è
> una passata NUOVA e non una correzione della diciottesima.** ⛔ **Dichiarato perché, ché la
> scelta era fra le due:** i numeri della diciottesima erano **veri a `07815a3`**, verificati uno
> per uno prima di toccare qualsiasi cosa; riscriverli avrebbe fatto descrivere a quel verbale uno
> stato che non ha mai misurato, che è il difetto della **nona** — *«una misura vera di un momento
> sbagliato è indistinguibile da una misura falsa per chi la legge dopo»*. Un verbale si aggiunge,
> non si riscrive; si corregge **di sole cifre** soltanto quando descrive **il file che lo
> contiene**, ed è l'unica correzione fatta qui.
>
> Scritta a **passata chiusa**; righe contate **partendo dall'elenco dei file citati** — **diciannove**
> bersagli, **venti** righe, perché `HANDOFF.md` ne ha due; **nessuna assente**. ⚠️ **La diciottesima
> diceva *«diciotto citazioni, diciotto righe»***, e la coppia era già di venti a diciannove: la
> verifica prescritta dalla quindicesima è stata **fatta**, e il suo esito **non è stato ricontato**.
>
> | | |
> |---|---|
> | **cresciuti** | [`AVVIO-CHAT.md`](AVVIO-CHAT.md) `13 → 16` — il salto più grande in proporzione, ed è il file che questa passata esisteva per rifare · [`HANDOFF.md`](HANDOFF.md) `175 → 176` per le tre affermazioni di stato false e le due cifre d'errata · [`porta-di-qualita.md`](porta-di-qualita.md) `95 → 96` per le date mancanti alle due campagne · `CLAUDE.md` `9 → 10` per il pre-controllo dei compiti · questo file `183 → 190` |
> | **invariati, ricontati** | spec del sotto-progetto 1 **277** · kernel-design 44 · roadmap 22 · [`README.md`](README.md) 13 · [`riferimenti.md`](riferimenti.md) 106 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · il piano del Traguardo 2 162 · il piano del Traguardo 3 168 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` (2441 B e 19291 B) · `adr/` 214 · `design/` nove file `4–9` |
> | ⚠️ **una cella che non è un peso** | quella di [`AVVIO-CHAT.md`](AVVIO-CHAT.md) dice ora **due** numeri, il file e il **messaggio** dentro di esso: chi apre quel file per incollarlo non gliene importa dei sedici kilobyte, gliene importa dei **7,7** che finiscono nella chat. Il peso del file non rispondeva alla domanda per cui la §12 esiste |
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **665** a **667 KB** (682795 B), corretto in
> **tutti e tre** i posti. I **due file obbligatori** passano da 192 a **200 KB**.
>
> 📌 **E la notizia di questa misura è che il rimedio proposto dalla diciassettesima è ora chiesto
> per la TERZA volta.** *«`check-docs.sh` sa già confrontare due insiemi, e confrontare una cifra
> dichiarata con `wc -c` è lo stesso mestiere»*: la diciottesima registrò che senza di esso sette
> righe su diciotto erano di nuovo fuori, e questa passata — che non ha eseguito **nessun compito**,
> solo riletto documenti — ne ha mosse **cinque**. ⛔ Resta **registrata e non presa**: è una riga di
> catalogo nuova, e quella è una decisione del proprietario. Ma il conto delle volte in cui sarebbe
> servita è ora scritto.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, all'undicesima
> applicazione.

> 🔁 **Ventesima misura, il 2026-08-11, chiudendo il brainstorming del Traguardo 4 — ed è la
> prima aperta da un BRAINSTORMING e non da un compito o da un audit.** Nessuna riga di codice
> scritta, nessun compito eseguito: solo un disegno e i documenti di stato che lo incassano. Ha
> mosso **sei** righe. Scritta a **passata chiusa**; righe contate **partendo dall'elenco dei file
> citati** — **venti** bersagli, **ventuno** righe, perché `HANDOFF.md` ne ha due; **nessuna
> assente**.
>
> | | |
> |---|---|
> | ⛔ **riga aggiunta** | il **disegno del Traguardo 4**, **23 KB** — il file da cui si riprende, e che si legge **prima** di scriverne il piano |
> | **cresciuti** | [`HANDOFF.md`](HANDOFF.md) `176 → 181` per il gotcha **#57**, il blocco del prossimo passo e due celle stantie · [`riferimenti.md`](riferimenti.md) `106 → 110` per le **otto** misure del brainstorming · [`roadmap.md`](roadmap.md) `22 → 23` · [`README.md`](README.md) `13 → 14` · [`AVVIO-CHAT.md`](AVVIO-CHAT.md) `16 → 17`, che non si muoveva **da diciassette misure** e cresce oggi per il riconteggio delle proprie case · questo file `190 → 200` |
> | **invariati, ricontati** | spec del sotto-progetto 1 **277** · kernel-design 44 · tracciabilità 15 · [`porta-di-qualita.md`](porta-di-qualita.md) 96 · `design/08` 8 · `design/01` 4 · `design/` nove file `4–9` · il piano degli spike 68 · il piano del Traguardo 1 50 · il piano del Traguardo 2 162 · il piano del Traguardo 3 168 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` (2441 B e 19291 B) |
> | ⛔ **una cella corretta DALLA MISURA, e per la seconda volta di seguito** | la riga del disegno del Traguardo 4 era stata scritta **`15 KB`** — un numero plausibile, buttato lì mentre si aggiungeva la riga. Il `wc -c` dice **23**. È esattamente il rilievo della **diciottesima** misura, ripetuto quattro riquadri dopo e da chi l'aveva letto: una cifra scritta prima della misura è un'ipotesi, e **una riga nuova nasce senza peso** finché qualcuno non lo misura |
>
> ⛔ **E la notizia di questa passata è di specie nuova: era stantio il conteggio di IN QUANTI
> POSTI VIVE UNA CIFRA.** La diciassettesima dichiara che l'aggregato *«vive in tre posti»* e la
> cifra dei due file obbligatori *«in cinque, uno in `CLAUDE.md` e quattro in `AVVIO-CHAT.md`»*.
> Cercate col `grep` **su tutto il repository** — che è il metodo che la sedicesima prescrive — le
> case sono **quattro** e **sei**: `AVVIO-CHAT.md` ne ha guadagnata una per ciascuna, e nessuno
> l'ha registrato perché **si contano le case una volta sola, quando si scrive il rimedio**.
> 📌 **La forma del #31 che ne esce:** il rimedio della sesta misura — *«si scrive dove qualcuno
> legge per decidere»* — porta con sé un **elenco delle case**, e quell'elenco è **esso stesso una
> cifra dentro una frase**. Chi rimisura non si fidi del numero di case scritto nel verbale
> precedente: lo **rifaccia col `grep`**, che costa un comando.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **667** a **675 KB** (691423 B), corretto in
> **tutte e quattro** le case. I **due file obbligatori** passano da 200 a **210 KB**, corretti in
> **tutte e sei**.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla dodicesima
> applicazione.
>
> 📌 **E il rimedio proposto dalla diciassettesima è ora chiesto per la QUARTA volta.** Questa
> passata non ha eseguito nessun compito e ha comunque mosso **sei** righe su ventuno, più i due
> aggregati e i loro conteggi di case. Resta **registrata e non presa**: è una riga di catalogo
> nuova, e quella è una decisione del proprietario.

> 🔁 **Ventunesima misura, il 2026-08-11, chiudendo il piano del Traguardo 4 — ed è la SECONDA
> passata dello stesso giorno**, perché la voce si è chiusa due volte: il disegno e poi il piano.
> Scritta a passata chiusa; righe contate **partendo dall'elenco dei file citati** — **ventuno**
> bersagli, **ventidue** righe, perché `HANDOFF.md` ne ha due.
>
> | | |
> |---|---|
> | ⛔ **riga aggiunta** | il **piano del Traguardo 4**, **71 KB** — il compito da cui si riprende |
> | **cresciuti** | il **disegno del Traguardo 4** `23 → 27`, per il richiamo della §11 in cui il codice lo ha smentito · questo file `200 → 206` |
> | **invariati, ricontati** | [`HANDOFF.md`](HANDOFF.md) 181 · [`riferimenti.md`](riferimenti.md) 110 · [`roadmap.md`](roadmap.md) 23 · [`README.md`](README.md) 14 · [`AVVIO-CHAT.md`](AVVIO-CHAT.md) 17 · [`porta-di-qualita.md`](porta-di-qualita.md) 96 · spec del sotto-progetto 1 **277** · kernel-design 44 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · `design/` nove file `4–9` · il piano degli spike 68 · il piano del Traguardo 1 50 · il piano del Traguardo 2 162 · il piano del Traguardo 3 168 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
>
> ⛔ **E la notizia è che il difetto della ventesima si è ripetuto identico, alla PRIMA occasione
> utile — che è la TERZA volta di seguito.** La riga del piano è stata scritta **`47 KB`** mentre
> si aggiungeva, e il `wc -c` dice **71**. La diciottesima misura lo registrò per il piano del
> Traguardo 3 (`162` scritto, **168** misurato, *«somigliare a un file vicino non è misurarlo»*), la
> ventesima per il disegno (`15` scritto, **23** misurato), e questa per il piano. **Tre passate
> consecutive, lo stesso gesto**: si aggiunge una riga, la cella del peso vuole un numero, e la mano
> ne scrive uno **plausibile** invece di lasciarla vuota fino alla misura.
>
> 📌 **Il rimedio, e stavolta non è un'esortazione: una riga nuova nasce SENZA peso.** Si scrive la
> cella `—`, si chiude la passata, si misura, e si riempie insieme a tutte le altre. Una cella
> vuota è visibilmente incompleta; una cella con dentro un numero verosimile **non lo è**, ed è
> precisamente per questo che sopravvive al commit. È il gotcha **#43** — *«un valore d'esempio
> valido viene incollato così com'è: non si distingue da un dato»* — applicato al peso invece che
> allo SHA.
>
> ⚠️ **E l'errore cresce:** `15 → 23` era il 53 % in più, `47 → 71` è il 51 %. Non è una stima che
> migliora avvicinandosi al vero: è **sempre la stessa stima sbagliata**, fatta guardando un file
> vicino che non c'entra.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **675** a **676 KB** (692398 B) — l'unico
> movimento è la precisazione in [ADR-0032](adr/0032-motore-di-persistenza.md) — corretto in **tutte
> e quattro** le case. I **due file obbligatori** passano da 210 a **216 KB**, corretti in tutte e
> sei.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla tredicesima
> applicazione.

> 🔁 **Ventiduesima misura, il 2026-08-11, chiudendo la sessione — ed è la TERZA dello stesso
> giorno**, perché la voce si è chiusa tre volte: il disegno, il piano, e la consegna al prossimo
> agente. Scritta a passata chiusa; righe contate partendo dall'elenco dei file citati —
> **ventuno** bersagli, **ventidue** righe, nessuna assente.
>
> | | |
> |---|---|
> | ✅ **nessuna riga aggiunta**, e il rimedio della ventunesima **non ha avuto occasione di fallire** | va detto invece che spacciato per successo: la ventunesima prescrive che *«una riga nuova nasce senza peso»*, e questa passata **non ne ha aggiunte**. La regola resta **non provata** |
> | **cresciuti** | [`HANDOFF.md`](HANDOFF.md) `181 → 184` per il gotcha **#58** e lo stato · [`AVVIO-CHAT.md`](AVVIO-CHAT.md) `17 → 19`, rifatto per la consegna · [`riferimenti.md`](riferimenti.md) `110 → 112` per **D4-9** e **D4-10** · [`roadmap.md`](roadmap.md) `23 → 24` per la riga del piano nella tabella dei piani · `CLAUDE.md` `10 → 11` · questo file `206 → 211` |
> | **invariati, ricontati** | [`README.md`](README.md) 14 · [`porta-di-qualita.md`](porta-di-qualita.md) 96 · [`tracciabilita.md`](tracciabilita.md) 15 · il **disegno del Traguardo 4** 27 · il **piano del Traguardo 4** 71 · spec del sotto-progetto 1 **277** · kernel-design 44 · `design/08` 8 · `design/01` 4 · `design/` nove file `4–9` · il piano degli spike 68 · il piano del Traguardo 1 50 · il piano del Traguardo 2 162 · il piano del Traguardo 3 168 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
> | ⛔ **e una riga di [`riferimenti.md`](riferimenti.md) mancava, non un numero** | le misure **D4-9** e **D4-10** — quelle che hanno corretto il disegno — erano state scritte **nel disegno e nell'ADR** ma non nel file che raccoglie le misure, perché arrivarono **dopo** che quella sezione era già chiusa. È il difetto della **nona** misura — *«un verbale si scrive quando la passata è chiusa»* — nella forma in cui la passata si **riapre** |
>
> ⛔ **E la notizia di questa misura è il MESSAGGIO, che è la cifra per cui `AVVIO-CHAT.md`
> esiste.** Diceva **7,7 KB** in due posti; misurato ora è **9,8** — cresciuto del **27 %** in una
> sessione sola, perché la consegna al prossimo agente ha aggiunto il blocco dei due file da
> aprire, le sette decisioni del piano e il gotcha **#58**. ⚠️ **Il rapporto che quel file difende
> regge comunque — 9,8 KB di messaggio che ordinano 242 KB di lettura, contro 689 di corpus** —
> ma la crescita è il **prossimo candidato** al difetto che la dodicesima misura registrò per
> [`porta-di-qualita.md`](porta-di-qualita.md): non un numero stantio, **un documento che smette di
> essere letto perché è diventato troppo lungo**. Un messaggio da incollare ha un limite naturale
> che una tabella non ha, ed è la pazienza di chi lo rilegge.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **676** a **689 KB** (695206 B), corretto in tutte
> e quattro le case. I **due file obbligatori** passano da 216 a **242 KB**, corretti in tutte e
> sei.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> quattordicesima applicazione.

> 🔁 **Ventitreesima misura, il 2026-08-11, chiudendo il Traguardo 4 — ed è la prima passata in
> cui a crescere di più non è `HANDOFF.md`.** Scritta a **passata chiusa**; righe contate
> **partendo dall'elenco dei file citati**, che è il movimento della quindicesima.
>
> | | |
> |---|---|
> | ⛔ **cresciuti, e il primo è il dato** | [`riferimenti.md`](riferimenti.md) `112 → 143` — **+31 KB in un traguardo solo**, ed è dove sono finite le misure che hanno smentito il piano · [`porta-di-qualita.md`](porta-di-qualita.md) `96 → 117` · [`HANDOFF.md`](HANDOFF.md) `184 → 194` · il **piano del Traguardo 4** `71 → 114`, per le settanta voci d'errata · questo file `211 → 229` |
> | ⛔ **riga aggiunta** | [`semi-dst.md`](semi-dst.md), **6 KB** — misurata **prima** di scrivere la cella, che è il rimedio della ventunesima e la **prima volta che ha avuto occasione di essere applicato** |
> | **invariati, ricontati** | spec del sotto-progetto 1 **277** · il disegno del Traguardo 4 **27** · il piano del Traguardo 3 168 · kernel-design 44 · roadmap 24 · README 14 · [`AVVIO-CHAT.md`](AVVIO-CHAT.md) 19 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · il piano degli spike 68 · il piano del Traguardo 1 50 · il piano del Traguardo 2 162 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
>
> ⛔ **E la notizia è che il rapporto per cui la §12 esiste si è MOSSO, per la prima volta nella
> direzione sbagliata.** L'insieme *«HANDOFF + spec + `adr/`»* passa da **679** a **689 KB**, ma i
> **due file obbligatori** passano da **222** a **242** — cioè il denominatore è cresciuto
> dell'1,5 % e il numeratore del **8,1 %**. ⚠️ La ragione è che il Traguardo 4 ha prodotto
> soprattutto **decisioni e misure**, che vivono nel compendio, e poco perimetro nuovo, che vivrebbe
> nella spec. 📌 Non è un difetto oggi — 242 contro 689 regge — ma è la **prima misura in cui la
> lettura obbligatoria cresce più in fretta del corpus che risparmia**, e chi rimisura la prossima
> volta guardi quel rapporto e non solo le celle.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> quindicesima applicazione.

> 🔁 **Ventiquattresima misura, il 2026-08-11, chiudendo la sessione con la consegna al prossimo
> agente — ed è una passata di sola documentazione, come la tredicesima.** Scritta a passata
> chiusa; righe contate partendo dall'elenco dei file citati.
>
> | | |
> |---|---|
> | **cresciuti** | [`AVVIO-CHAT.md`](AVVIO-CHAT.md) `19 → 21`, rifatto per la consegna · [`roadmap.md`](roadmap.md) `24 → 26` · [`riferimenti.md`](riferimenti.md) `143 → 145` · [`HANDOFF.md`](HANDOFF.md) `194 → 195` |
> | **invariati, ricontati** | questo file **231** · [`porta-di-qualita.md`](porta-di-qualita.md) 117 · [`README.md`](README.md) 14 · [`semi-dst.md`](semi-dst.md) 6 · `CLAUDE.md` 11 · spec 277 · il disegno del Traguardo 4 27 · i piani 168, 162, 114, 50, 68 · tracciabilità 15 · `design/08` 8 · `design/01` 4 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
>
> ⛔ **E la notizia è il MESSAGGIO, che è la cifra per cui quel file esiste: è passato da 9,8 a
> 12,2 KB, +24 % in una sessione — la SECONDA crescita a due cifre di seguito.** La ventiduesima
> misura lo aveva registrato come *«il prossimo candidato»* al difetto che la dodicesima trovò per
> il registro: non un numero stantio, **un documento che smette di essere letto perché è diventato
> troppo lungo**. 📌 **La consegna successiva COMPRIMA invece di appendere:** le lezioni di un
> traguardo chiuso da due traguardi si spostano nel compendio, dove chi legge le trova comunque, e
> il messaggio tiene solo ciò che serve a **non sbagliare il primo passo**. ⚠️ Un messaggio da
> incollare ha un limite che nessuna tabella ha, ed è la pazienza di chi lo rilegge.
>
> ✅ **L'aggregato è fermo — 689 → 690 KB — e i due file obbligatori pure, a 242.** Il rapporto che
> la §12 esiste per difendere non si è mosso in questa passata: a muoverlo era stato il traguardo,
> non la consegna.

> 🔁 **Venticinquesima misura, il 2026-08-11, chiudendo l'AUDIT COMPLETO — ed è la prima passata
> aperta da qualcosa che non è né un compito né una chiusura di traguardo.** Scritta a passata
> chiusa; righe contate **partendo dall'elenco dei file citati**, che è il movimento della
> quindicesima.
>
> | | |
> |---|---|
> | ⛔ **riga aggiunta** | [`audit-2026-08-11.md`](audit-2026-08-11.md), **22 KB** — misurata **prima** di scrivere la cella, che è il rimedio della ventunesima alla **seconda** applicazione riuscita. ⚠️ Ed è messa **in cima** alla tabella e non in coda: è il **prossimo passo**, non un documento di consultazione |
> | **cresciuti** | [`HANDOFF.md`](HANDOFF.md) `196 → 203` per i gotcha **#59–#64** e il punto di ripresa · [`riferimenti.md`](riferimenti.md) `145 → 152` per le misure dell'audit · [`porta-di-qualita.md`](porta-di-qualita.md) `117 → 120` per i tre controlli riparati · [`AVVIO-CHAT.md`](AVVIO-CHAT.md) `21 → 24` · [`roadmap.md`](roadmap.md) `26 → 27` · `CLAUDE.md` `11 → 12` · questo file `231 → 244` |
> | **invariati, ricontati** | spec del sotto-progetto 1 **277** · il disegno del Traguardo 4 30 · kernel-design 44 · [`README.md`](README.md) 15 · tracciabilità 15 · [`semi-dst.md`](semi-dst.md) 6 · `design/08` 10 · `design/01` 4 · i piani 168, 162, 114, 68, 50 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
> | ✅ **`tracciabilita.md` NON è stato toccato, ed è una decisione** | l'audit non ha spostato nessuna funzionalità, quindi la mappa non cambia; e la §8 dice che si aggiorna **solo alla chiusura del sotto-progetto 1**. Le sue **171** funzionalità sono state **ricontate** — la cifra è giusta — e la crepa della legenda `📋` porta già i propri due riquadri |
>
> ⛔ **E la notizia di questa misura è il MESSAGGIO, che cresce per la QUARTA volta di seguito e
> per la terza a due cifre percentuali: 7,7 → 9,8 → 12,2 → 14,7 KB.** La 24ª aveva prescritto
> *«la consegna successiva COMPRIMA invece di appendere»*, e la compressione **è stata fatta** —
> le lezioni del Traguardo 4 e le tre forme del gotcha #48 sono state portate qui, dove chi legge
> le trova comunque. Ha recuperato **344 byte**, e il blocco dell'audit li ha più che compensati.
> 📌 **La regola che ne esce, e va scritta perché il rimedio della 24ª non è bastato:**
> *comprimere ciò che è vecchio non basta quando ciò che è nuovo pesa di più.* La prossima
> consegna deve decidere **cosa TOGLIERE**, non cosa accorciare — e il candidato naturale è il
> blocco delle decisioni ribaltabili, che è un rimando a due errata già scritte.
>
> ⛔ **E il rapporto che la §12 esiste per difendere si è mosso ancora nella direzione sbagliata,
> per la seconda misura di seguito.** L'insieme *«HANDOFF + spec + `adr/`»* passa da **691** a
> **698 KB**; la lettura obbligatoria passa da **242** a **256 KB**, e con l'audit — che ora è il
> **terzo file da leggere** — a **278**. Il denominatore è cresciuto dell'1 %, il numeratore del
> **15,7 %**. ⚠️ Non è un difetto oggi — 278 contro 698 regge — ma è la ragione per cui
> quel blocco va tolto e non accorciato.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> sedicesima applicazione.

> 🔁 **Ventiseiesima misura, il 2026-08-17, chiudendo la prima delle otto decisioni dell'audit —
> ed è la prima passata in cui il MESSAGGIO non è cresciuto.** Scritta a passata chiusa; righe
> contate **partendo dall'elenco dei file citati**, che è il movimento della quindicesima —
> ventidue bersagli, ventitré righe, perché `HANDOFF.md` ne ha due; nessuna assente.
>
> | | |
> |---|---|
> | ✅ **nessuna riga aggiunta** | nessun documento nuovo: la decisione 1 ha prodotto **codice di banco** e voci dentro documenti che esistevano. Il rimedio della ventunesima — *«una riga nuova nasce senza peso»* — **non ha avuto occasione di fallire**, e va detto invece che spacciato per successo |
> | **cresciuti** | [`porta-di-qualita.md`](porta-di-qualita.md) `120 → 127` — il salto più grande, ed è il registro che assorbe tre bugiardi e la campagna delle sei mutazioni · [`riferimenti.md`](riferimenti.md) `152 → 156` · [`HANDOFF.md`](HANDOFF.md) `203 → 206` per il gotcha **#65** e il punto di ripresa · [`audit-2026-08-11.md`](audit-2026-08-11.md) `22 → 25` per il richiamo in testa alla §5 · questo file `244 → 253` · [`README.md`](README.md) `15 → 16` · [`AVVIO-CHAT.md`](AVVIO-CHAT.md) `24 → 25`, benché il **messaggio** dentro di esso sia fermo · `CLAUDE.md` `12 → 12` e [`roadmap.md`](roadmap.md) `27 → 27`, mossi di poche righe |
> | **invariati, ricontati** | spec del sotto-progetto 1 **277** · il disegno del Traguardo 4 30 · kernel-design 44 · [`tracciabilita.md`](tracciabilita.md) 15 · [`semi-dst.md`](semi-dst.md) 6 · `design/08` 10 · `design/01` 4 · i piani 168, 162, 114, 68, 50 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
>
> ✅ **E LA NOTIZIA È IL MESSAGGIO, che per quattro misure di seguito era la cattiva.** La serie
> era 7,7 → 9,8 → 12,2 → 14,7 KB; oggi il blocco fra le due recinzioni passa da **15036 a 15040
> byte** — **`+4 B` e una riga** — pur avendo incassato una decisione intera dell'audit.
> ⛔ **Ha funzionato la prescrizione della venticinquesima, alla lettera:** *«la prossima consegna
> deve decidere cosa TOGLIERE, non cosa accorciare — e il candidato naturale è il blocco delle
> decisioni ribaltabili, che è un rimando a due errata già scritte»*. Quel blocco è stato **tolto**
> e ridotto a un rimando di sei righe, e il finding V6 — nel frattempo **chiuso** — è passato da
> diciotto righe a due. 📌 **La regola che ne esce, ed è la prima volta che questa serie ne produce
> una che funziona:** si toglie un **rimando duplicato** o una voce **chiusa**; una **lezione** non
> si toglie, si sposta nel compendio, dove chi legge la trova comunque.
>
> ⛔ **E le case sono state ricontate col `grep`, non riprese dal verbale precedente** — che è il
> rimedio della ventesima, e per la seconda volta il conteggio era stantio. La venticinquesima
> dichiarava **sei** case per la cifra dei due file obbligatori: sono **tre** (una in `CLAUDE.md`,
> due in [`AVVIO-CHAT.md`](AVVIO-CHAT.md)). L'aggregato ne ha **quattro**, e la cifra **coi tre
> file da leggere** altre **quattro**. 📌 L'elenco delle case è esso stesso una cifra dentro una
> frase, e invecchia come tutte.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **698** a **701 KB** (718041 B), corretto in tutte
> e quattro le case. I **due file obbligatori** passano da 256 a **265 KB**, e coi tre da 278 a
> **290**, corretti in tutte le loro.
>
> ⚠️ **Il rapporto che la §12 difende si è mosso ancora nella direzione sbagliata, per la terza
> misura di seguito:** il denominatore è cresciuto dello **0,4 %**, il numeratore del **2,9 %**.
> Molto meno delle due volte precedenti — la decisione 1 ha prodotto soprattutto **codice** — ma la
> direzione è la stessa, e chi rimisura guardi quel rapporto e non solo le celle.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> diciassettesima applicazione.

> 🔁 **Ventisettesima misura, il 2026-08-17, subito dopo la ventiseiesima — ed è una passata di
> sola documentazione, aperta per verificare che la precedente avesse finito. NON AVEVA FINITO.**
> Scritta a passata chiusa; cercata col `grep` invece che a memoria, che è l'unica ragione per cui
> ha trovato qualcosa.
>
> | | |
> |---|---|
> | ⛔ **quattro affermazioni di stato FALSE, tutte in [`HANDOFF.md`](HANDOFF.md)** | tre righe dicevano *«il prossimo passo è il brainstorming del Traguardo 5»* — nella «In trenta secondi», nel blocco del lascito e sotto il titolo **«Prima cosa da fare»** — mentre il **Punto di ripresa**, settanta righe più su **nello stesso file**, diceva *«non il Traguardo 5»*. ⛔ **È il finding D-1 dell'audit, che le aveva lasciate:** quella passata corresse il Punto di ripresa e non le tre gemelle. Radice **R1** — *una correzione attraversa il documento in cui nasce, non gli altri* — e qui nemmeno tutto il documento |
> | ⛔ **e la ventiseiesima ha commesso R1 dentro la passata che chiudeva R1** | la cella di `journal_contract_real.rs` in [`porta-di-qualita.md`](porta-di-qualita.md) ha avuto corretta l'**intestazione** (`dodici → quindici` test) e **non il corpo**, che ha continuato a dire *«nove bugiardi»*, *«dodici test»* e *«nove per corsa»* per un commit intero. Una cella lunga ha **due** posti in cui vive lo stesso numero, e chi corregge quello in cima non vede l'altro |
> | **altre tre voci stantie** | *«le sonde sono J1…J13»* nella riga di copertura del registro (sono **J1…J16**) · il peso dell'audit `22 → 25 KB` in `HANDOFF.md` · e in **questo file** un *«⏭️ il prossimo è il Task 2»* fermo da **dieci** compiti, dentro il racconto del Traguardo 4 |
> | **cresciuti** | [`HANDOFF.md`](HANDOFF.md) `206 → 208` · [`porta-di-qualita.md`](porta-di-qualita.md) `127 → 128` · `CLAUDE.md` `12 → 13` per la **settima** domanda del pre-controllo · questo file `253 → 257` |
> | **invariati, ricontati** | [`riferimenti.md`](riferimenti.md) 156 · [`audit-2026-08-11.md`](audit-2026-08-11.md) 25 · [`AVVIO-CHAT.md`](AVVIO-CHAT.md) 25 · [`roadmap.md`](roadmap.md) 27 · [`README.md`](README.md) 16 · spec **277** · disegno T4 30 · kernel-design 44 · tracciabilità 15 · `semi-dst.md` 6 · `design/08` 10 · `design/01` 4 · i piani 168, 162, 114, 68, 50 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
>
> ⛔ **E la notizia è che una passata di verifica APERTA SUBITO DOPO ne ha trovate sei.** La
> ventiseiesima aveva fatto tutto ciò che la §13 pretende — compendio, handoff, riferimenti,
> registro, e i pesi rimisurati — e restava **falsa in quattro punti**, uno dei quali sotto il
> titolo *«Prima cosa da fare»*, che è la frase più autorevole del file più autorevole.
> 📌 **La regola che ne esce, e costa un comando:** quando si sposta il **prossimo passo**, non si
> corregge la riga che si ha davanti — si cerca **`grep '⏭️'` su tutti i documenti di stato** e si
> guardano **tutte** le case. Vale per il prossimo passo come la sesta misura lo scrisse per i
> pesi, e le case sono di più di quante ne ricordi chi ha appena scritto una di esse.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **701** a **703 KB**. I **due file obbligatori**
> passano da 265 a **270 KB**, e coi tre da 290 a **295**.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> diciottesima applicazione.

> 🔁 **Ventottesima misura, il 2026-08-18, chiudendo la decisione 8 dell'audit (G-5) — ed è la
> prima passata in cui NESSUNA cella era stantia e lo strumento sbagliava lo stesso.** Scritta a
> passata chiusa; righe contate partendo dall'elenco dei file citati.
>
> ⛔ **LA NOTIZIA È IL METODO, NON I NUMERI: «`wc -c`» NON DICE QUALI BYTE, E QUESTO REPOSITORY SI
> LAVORA DA DUE MACCHINE.** Rimisurando con `wc -c` sull'albero di lavoro, **nove** celle
> risultavano fuori di 1–4 KB **su file non toccati da giorni** — la spec `277 → 281`,
> `kernel-design 44 → 45`, il disegno del Traguardo 4 `30 → 31`, tutti e cinque i piani. La
> lettura ovvia era il gotcha **#31** nella forma peggiore: *«invariati, ricontati»* dichiarato
> per tre verbali di seguito su un numero sbagliato, cioè il finding **D-6** dell'audit ripetuto.
>
> ✅ **Ed era falsa, e a smentirla è stata la FORMA dello scarto e non il suo valore:** era
> **uniforme**, e l'invecchiamento non lo è. I fine-riga di questo albero sono **CRLF**; le misure
> precedenti furono prese su byte **LF**. `wc -c` conta **un byte in più per riga**, quindi su un
> file di **3961** righe fa esattamente i quattro kilobyte dello scarto.
>
> | File | `wc -c` qui (CRLF) | senza i CR | La cella diceva |
> |---|---|---|---|
> | spec del sotto-progetto 1 | 281 | **276,9** | **277** ✅ |
> | kernel-design | 45 | **44,3** | **44** ✅ |
> | disegno del Traguardo 4 | 31 | **30,5** | **30** ✅ |
> | il piano degli spike | 70 | **67,9** | **68** ✅ |
> | i quattro piani dei traguardi | 52 · 165 · 171 · 115 | **50 · 162 · 168 · 114** | **50 · 162 · 168 · 114** ✅ |
>
> 📌 **Quindi nessuna cella era stantia, e il verbale precedente aveva ragione.** È il gotcha
> **#48** — *«il banco con cui misuri sbaglia»* — nella forma che si coglie: lo scarto uniforme.
> Un difetto di misura ha una **firma**, l'invecchiamento no; e la lezione che vale oltre il caso è
> che **prima di correggere nove celle si guarda se sbagliano tutte della stessa quantità
> relativa.** Nove correzioni sbagliate sarebbero state committate in un colpo solo.
>
> ⛔ **E resta una decisione APERTA, registrata e non presa:** la §12 dichiara il metodo come
> *«`wc -c`, arrotondati a KiB»*, e quella formula **non è machine-indipendente**. Su questa
> macchina l'albero è CRLF, e i due percorsi del repository in [`AVVIO-CHAT.md`](AVVIO-CHAT.md)
> dicono che le macchine sono **due**: chi rimisura dall'altra otterrebbe numeri diversi, li
> correggerebbe in buona fede, e le cifre **oscillerebbero per sempre** senza che nessuno dei due
> sbagli. La cura è **una riga di metodo** — i pesi si misurano sui byte **LF**, cioè
> `wc -c` meno i `CR` — ed è scritta qui invece che nella riga del metodo perché **cambiare il
> metodo della §12 è una decisione del proprietario**, come la guardia sui pesi che la
> diciassettesima misura ha chiesto quattro volte.
>
> | | |
> |---|---|
> | **cresciuti** (byte LF) | [`riferimenti.md`](riferimenti.md) `156 → 162` — il salto più grande, ed è la sezione delle misure di G-5 · [`porta-di-qualita.md`](porta-di-qualita.md) `128 → 130` per le sonde **N6** e **N7** · [`HANDOFF.md`](HANDOFF.md) `208 → 209` · [`roadmap.md`](roadmap.md) `27 → 28` · [`audit-2026-08-11.md`](audit-2026-08-11.md) `25 → 26` per le due righe barrate della §8 · questo file `257 → 261` |
> | ⚠️ **e una cella stantia VERA, l'unica** | [`README.md`](README.md) `15 → 16`. È l'unica delle dieci che lo scarto uniforme non spiegava, e si vedeva solo **dopo** aver tolto i CR: il difetto di strumento **nascondeva** il difetto di dato |
> | **invariati, ricontati** | spec **277** · kernel-design 44 · disegno T4 30 · [`AVVIO-CHAT.md`](AVVIO-CHAT.md) 25 · tracciabilità 15 · [`semi-dst.md`](semi-dst.md) 6 · `design/08` 10 · `design/01` 4 · `design/` nove file `4–10` · i piani 68, 50, 162, 168, 114 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` (2441 B e 19584 B) |
>
> ⛔ **E il MESSAGGIO è cresciuto, dopo una passata sola di fermo: da 15040 a 15361 byte, +321 B
> e +2,1 %.** La compressione prescritta dalla 25ª **è stata fatta** — il blocco di T-2/T-1 è
> diventato un elenco di due voci — e non è bastata a pagare il blocco di G-5. ⚠️ Va detto
> invece che spacciato per stabilità: la 26ª aveva chiuso a `+4 B` e questa non ci è riuscita.
> 📌 La regola della 26ª regge lo stesso — *si toglie un rimando duplicato o una voce chiusa* — ma
> il candidato da togliere alla prossima consegna è ora **il blocco delle due decisioni chiuse**,
> che diventerà un rimando alla §6 appena una terza si chiude.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **703** a **704 KB**. I **due file obbligatori**
> passano da 270 a **279 KB**, e coi tre da 295 a **305**.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> diciannovesima applicazione.

> 🔁 **Ventinovesima misura, il 2026-08-18, chiudendo la decisione 6 dell'audit (A-1, A-2, A-4,
> A-7) — ed è la SECONDA passata dello stesso giorno**, come la ventunesima: la voce si è chiusa
> due volte. Scritta a passata chiusa, in **byte LF** — che è il metodo che la ventottesima ha
> dovuto scoprire per non correggere nove celle giuste.
>
> | | |
> |---|---|
> | **cresciuti** | [`riferimenti.md`](riferimenti.md) `162 → 165` per le misure dei quattro richiami · [`HANDOFF.md`](HANDOFF.md) `209 → 210` · [`audit-2026-08-11.md`](audit-2026-08-11.md) `26 → 27` per le tre righe barrate della §8 · `design/08` `10 → 11` e `design/01` `4 → 5`, che portano ora il proprio richiamo · questo file `261 → 268` |
> | ⛔ **e una riga di INTERVALLO che nessuno aggiorna mai** | `design/` nove file `4–10 → 4–11`: è la cella che invecchia in silenzio, perché un intervallo *sembra* sempre giusto. Ricontata su tutti e nove i file, non dedotta dai due che ho toccato |
> | **invariati, ricontati** | spec **277** · kernel-design 44 · disegno T4 30 · [`porta-di-qualita.md`](porta-di-qualita.md) 130 · [`roadmap.md`](roadmap.md) 28 · [`README.md`](README.md) 16 · [`AVVIO-CHAT.md`](AVVIO-CHAT.md) 25 · tracciabilità 15 · [`semi-dst.md`](semi-dst.md) 6 · i piani 68, 50, 162, 168, 114 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` (2441 B e 19291 B in byte LF — ⚠️ con `wc -c` su questo albero il massimo dice **19584**, che sono i **293 CR** di quel file: la ventottesima, applicata) |
>
> ✅ **E il MESSAGGIO non si è mosso: 15361 byte, invariato.** Non per virtù ma per compenso — le
> tre correzioni di conteggio si annullano fra loro (`SEI → CINQUE` due volte, `la prima e
> l'ottava → la 1, la 6 e l'8` una). Va detto così invece di attribuirselo: la 26ª chiuse a
> `+4 B` **per una compressione decisa**, questa a `0` **per caso**.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **704** a **707 KB**. I **due file obbligatori**
> passano da 279 a **284 KB**, e coi tre da 305 a **311**.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> ventesima applicazione.

> 🔁 **Trentesima misura, il 2026-08-18, chiudendo la decisione 5 (C-1) — TERZA passata dello
> stesso giorno, e volutamente CORTA.** In byte LF, a passata chiusa.
>
> | | |
> |---|---|
> | **cresciuti** | [`riferimenti.md`](riferimenti.md) `165 → 167` · [`audit-2026-08-11.md`](audit-2026-08-11.md) `27 → 28` · questo file `268 → 272` |
> | **invariati** | [`HANDOFF.md`](HANDOFF.md) 210 · [`roadmap.md`](roadmap.md) 28 — ⚠️ **e l'invarianza è il dato**: la cella del sotto-progetto 1 è stata **accorciata** mentre la si aggiornava, sostituendo l'elenco delle decisioni chiuse con un **rimando alla §6**. Ricopiarlo lì è ciò che ha fatto invecchiare quella riga **tre volte** · [`README.md`](README.md) 16 · [`AVVIO-CHAT.md`](AVVIO-CHAT.md) 25 · `CLAUDE.md` 13 · [`porta-di-qualita.md`](porta-di-qualita.md) 130 · spec **277** · tutto il resto come alla ventinovesima |
>
> ⚠️ **Il messaggio: 15361 → 15369 byte, `+8 B`.** Il conteggio è passato a QUATTRO decisioni e il
> blocco delle chiuse ha guadagnato una voce: è il segnale che la 29ª aveva previsto — **il
> blocco delle decisioni chiuse va tolto e sostituito da un rimando alla §6** appena la quinta
> si chiude, o cresce di una voce per decisione fino alla fine.
>
> L'insieme resta **707 KB**. I **due file obbligatori** passano da 284 a **287 KB**, e coi tre da
> 311 a **315**.

> 🔁 **Trentunesima misura, il 2026-08-18, chiudendo la decisione 4 (PL-1) — QUARTA passata dello
> stesso giorno.** In byte LF, a passata chiusa.
>
> | | |
> |---|---|
> | **cresciuti** | [`riferimenti.md`](riferimenti.md) `167 → 170` · [`porta-di-qualita.md`](porta-di-qualita.md) `130 → 133` per la settima sonda di `file_journal.rs` · [`HANDOFF.md`](HANDOFF.md) `210 → 211` · [`audit-2026-08-11.md`](audit-2026-08-11.md) `28 → 29` · [`README.md`](README.md) `16 → 17` · questo file `272 → 277` |
> | **invariati, ricontati** | [`roadmap.md`](roadmap.md) 28 · [`AVVIO-CHAT.md`](AVVIO-CHAT.md) 25 · `CLAUDE.md` 13 · spec **277** · kernel-design 44 · disegno T4 30 · tracciabilità 15 · [`semi-dst.md`](semi-dst.md) 6 · i piani 68, 50, 162, 168, 114 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
>
> ⚠️ **Il messaggio: 15369 → 15367 byte, `−2 B`.** Il conteggio delle decisioni si è accorciato
> (`QUATTRO → TRE`) più di quanto il blocco delle chiuse sia cresciuto. ⛔ **Non è la compressione
> che la 29ª prescriveva**, è aritmetica: il blocco ha guadagnato **un'altra voce**, che è il quinto
> giro dello stesso meccanismo. La prescrizione resta **non applicata** e va detto così.
>
> 📌 **E questa passata ha prodotto una cifra di specie nuova per il registro: un conteggio che
> DIPENDE DAL SISTEMA.** `file_journal.rs` porta **sei** test su Windows e **sette** su Linux,
> perché il settimo è `cfg(unix)`. Dichiarato invece di sceglierne uno — ed è la stessa lezione
> della ventottesima, un piano sotto: **una misura ha bisogno di dire su quale macchina è stata
> presa**, o due lettori onesti ottengono due numeri e si correggono a vicenda per sempre.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **707** a **711 KB**. I **due file obbligatori**
> passano da 287 a **292 KB**, e coi tre da 315 a **321**.

> 🔁 **Trentaduesima misura, il 2026-08-18, chiudendo la decisione 3 (K-1 con B-1) — QUINTA
> passata dello stesso giorno.** In byte LF, a passata chiusa.
>
> | | |
> |---|---|
> | **cresciuti** | [`porta-di-qualita.md`](porta-di-qualita.md) `133 → 140` per il verbale di K-1/B-1 · [`riferimenti.md`](riferimenti.md) `170 → 174` · [`HANDOFF.md`](HANDOFF.md) `211 → 213` per il gotcha **#66** · questo file `277 → 283` · [`AVVIO-CHAT.md`](AVVIO-CHAT.md) `25 → 26` |
> | **invariati, ricontati** | [`audit-2026-08-11.md`](audit-2026-08-11.md) 29 · [`README.md`](README.md) 17 · [`roadmap.md`](roadmap.md) 28 · `CLAUDE.md` 13 · spec **277** |
>
> ⛔ **E questa passata ha trovato un difetto di METODO che i numeri nascondevano, sul banco di
> misura e non sui documenti.** La prima corsa dava l'audit a **28 KB** contro i **29** della 31ª,
> su un file che `git diff` dichiara **non toccato**. Non era il file: era **l'arrotondamento**.
> `29469` byte fanno `28,78` KB, che **tronca a 28** e **arrotonda a 29** — e la serie di queste
> trentadue misure arrotonda. ⚠️ **Il metodo non era scritto da nessuna parte**, quindi due lettori
> onesti ottengono due numeri e si correggono a vicenda per sempre: è la lezione della 31ª — *«una
> misura ha bisogno di dire su quale macchina è stata presa»* — sull'asse dell'**operazione**
> invece che della macchina. 📌 **Da qui in poi: byte LF, `int(n/1024 + 0.5)`.**
> ⛔ **E una seconda cifra falsa è stata colta prima di entrare nel documento**, che è il punto:
> la prima misura dell'insieme dava **743 KB** perché il glob `*sottoprogetto-1*` rastrellava
> **anche** il disegno del Traguardo 4. L'insieme è *«HANDOFF + LA spec + `adr/`»*, e la
> contro-prova che il glob fosse sbagliato era già lì: la spec da sola deve fare **277**, e faceva
> 308. Gotcha **#48**, quattordicesima occorrenza.
>
> ⚠️ **Il messaggio: 15367 → 15940 byte, `+573 B`** — la crescita più grande da quando si conta,
> ed è **tutta** nel blocco delle decisioni chiuse, che ha guadagnato la sua **terza** voce.
> ⛔ **La prescrizione della 29ª è ora dovuta e non più solo prevista:** quel blocco va **tolto e
> sostituito da un rimando alla §6**. Resta **non applicata**, e va detto così invece di rimandarla
> in silenzio una quinta volta.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **711** a **713 KB**. I **due file obbligatori**
> passano da 292 a **296 KB**, e coi tre da 321 a **325**.

> 🔁 **Trentatreesima misura, il 2026-08-18, chiudendo la decisione 2 (P-1) — SESTA passata dello
> stesso giorno.** In byte LF, arrotondati — `int(n/1024 + 0.5)`, il metodo fissato dalla 32ª.
>
> | | |
> |---|---|
> | **cresciuti** | [`COMPENDIO.md`](COMPENDIO.md) `283 → 289` · [`HANDOFF.md`](HANDOFF.md) `213 → 215` per il gotcha **#67** · [`porta-di-qualita.md`](porta-di-qualita.md) `140 → 145` · [`riferimenti.md`](riferimenti.md) `174 → 178` · [`AVVIO-CHAT.md`](AVVIO-CHAT.md) `26 → 27` |
> | **invariati, ricontati** | [`audit-2026-08-11.md`](audit-2026-08-11.md) 29 · [`README.md`](README.md) 17 · [`roadmap.md`](roadmap.md) 28 · `CLAUDE.md` 13 · spec **277** |
>
> ⛔ **IL MESSAGGIO: 15940 → 16659 byte, `+719 B`, la crescita più grande mai registrata — e la
> prescrizione della 29ª è ora SCADUTA, non più dovuta.** Il blocco delle decisioni chiuse ha la
> sua **quarta** voce, e cresce di una a ogni chiusura come la 29ª aveva previsto **cinque passate
> fa**. ⚠️ **Va detto come sta:** non è stata applicata perché ogni singola passata la trovava più
> economica da rimandare che da eseguire, ed è precisamente il modo in cui un debito dichiarato
> resta dichiarato per sempre. 📌 **Con l'ultima decisione della §8 il blocco arriva a cinque, e a
> quel punto TOGLIERLO è il lavoro di chiusura dell'audit, non una rifinitura.**
>
> ⚠️ **E una cifra di questa passata NON è in KB e vale rileggerla:** i casi di `compile_fail`
> passano da **diciassette a diciotto**, ed è il primo caso nuovo dal Traguardo 2.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **713** a **715 KB**. I **due file obbligatori**
> passano da 296 a **303 KB**, e coi tre da 325 a **332**.

> 🔁 **Trentaquattresima misura, il 2026-08-18, chiudendo la decisione 7 — e con essa l'AUDIT
> INTERO. Settima passata dello stesso giorno.** In byte LF, `int(n/1024 + 0.5)`.
>
> | | |
> |---|---|
> | **cresciuti** | [`porta-di-qualita.md`](porta-di-qualita.md) `145 → 156` — la passata che l'ha fatto crescere di più, e sono due sezioni: la decisione 7 e la **voce aperta consolidata** · [`riferimenti.md`](riferimenti.md) `178 → 183` · questo file `289 → 295` |
> | **calato** | ⚡ [`AVVIO-CHAT.md`](AVVIO-CHAT.md) `27 → 25` |
> | **invariati, ricontati** | [`HANDOFF.md`](HANDOFF.md) 215 · [`audit-2026-08-11.md`](audit-2026-08-11.md) 29 · [`README.md`](README.md) 17 · [`roadmap.md`](roadmap.md) 28 · `CLAUDE.md` 13 · spec **277** |
>
> ✅ **IL MESSAGGIO È CALATO PER LA PRIMA VOLTA DA QUANDO SI CONTA: 16659 → 14947 byte, `−1712 B`,
> cioè −10,3 %.** La serie era 7,7 → 9,8 → 12,2 → 14,7 → 15,0 → 15,4 → 15,9 → **16,7 KB**, sette
> crescite e un solo fermo. ⛔ **Ha funzionato la prescrizione della 29ª — applicata alla settima
> occasione, che è la prima in cui era DOVUTA e non solo prevista:** il blocco delle decisioni
> chiuse, arrivato a **quattro** voci e destinato alla quinta, è stato **tolto** e sostituito da
> una riga sola — *«il rimedio si prezza leggendo il codice, non il rapporto, e può risultare più
> piccolo, più grande o di specie diversa»* — più il rimando alla §6.
> 📌 **La lezione è sopravvissuta, il verbale no**, ed è la regola della 25ª: *si toglie un
> rimando duplicato o una voce chiusa, mai una lezione*. Le quattro chiusure per esteso stanno
> nella §6, che chi legge apre comunque.
> ⚠️ **E il messaggio ha cambiato SPECIE oltre che dimensione:** la skill che serve subito non è
> più `repo-audit` ma `brainstorming`, perché il prossimo passo non è più esecuzione ma lavoro
> creativo. Era la ragione per cui quel blocco esisteva; chiuso l'audit, la ragione è finita.
>
> L'insieme *«HANDOFF + spec + `adr/`»* resta **715 KB**. I **due file obbligatori** passano da
> 303 a **308 KB**, e coi tre da 332 a **337**.

> 🔁 **Trentacinquesima misura, il 2026-08-18, chiudendo la passata di coerenza sul puntatore
> al prossimo passo.** In byte LF, `int(n/1024 + 0.5)`.
>
> | | |
> |---|---|
> | **cresciuti** | questo file `295 → 304` · [`riferimenti.md`](riferimenti.md) `183 → 188` per il censimento · [`audit-2026-08-11.md`](audit-2026-08-11.md) `29 → 31`, i timbri delle tre decisioni che erano rimaste senza · [`roadmap.md`](roadmap.md) `28 → 29` |
> | ⚡ **calato** | [`HANDOFF.md`](HANDOFF.md) — quattro blocchi **tolti** e sostituiti da rimandi alla §6: `220527 → 220362 B`, che arrotondato resta **215 KB** |
> | **invariati, ricontati** | [`README.md`](README.md) 17 · [`AVVIO-CHAT.md`](AVVIO-CHAT.md) 25 · [`porta-di-qualita.md`](porta-di-qualita.md) 156 · `CLAUDE.md` 13 · [`tracciabilita.md`](tracciabilita.md) 15 · [`semi-dst.md`](semi-dst.md) 6 · spec **277** |
>
> ⛔ **E LA NOTIZIA NON È UNA CRESCITA: È CHE TRE CELLE DELLA TABELLA QUI SOPRA ERANO GIÀ
> STANTIE A `HEAD`, e nessuna delle tre l'ha fatta questa passata.**
> [`HANDOFF.md`](HANDOFF.md) diceva **211** dove la 33ª misura aveva già registrato **215**;
> [`porta-di-qualita.md`](porta-di-qualita.md) diceva **133** dove la 34ª aveva registrato
> **156**, uno scarto di **ventitré** kilobyte; [`riferimenti.md`](riferimenti.md) diceva
> **170** dove la 34ª aveva registrato **183**.
> 📌 **È il difetto che la SESTA misura descrisse per esteso, e che da allora nessuno ha più
> cercato:** *«il riquadro e la tabella sono due posti, si aggiorna il primo, e il secondo
> continua a rispondere a chi deve decidere se aprire un file»*. Le tre celle sbagliate sono
> esattamente quelle che servono a **decidere se aprire**, ed erano fuori di 4, 13 e 23 KB.
> ⚠️ **E la diagnosi è la STESSA della passata che questo verbale chiude, su un oggetto
> diverso:** una cifra vive in due posti, si scrive nel verbale, e la tabella diverge in
> silenzio. Radice **R1**. 📌 Chi rimisura non legge i verbali: **rimisura i file**, e poi
> confronta con **tutte** le celle, non con l'ultimo riquadro.
>
> ⛔ **IL MESSAGGIO: 14947 → 15014 byte, `+67 B` e una riga** — quella che dice che l'audit è
> **chiuso** e non è più il compito. ✅ **È la crescita più piccola mai registrata**, ed è nel
> verso giusto: la 34ª aveva tolto 1712 byte, questa aggiunge un **fatto** senza riaprire il
> blocco che era stato chiuso. ⚠️ **Due celle che DESCRIVONO il messaggio erano stantie e sono
> rifatte:** la tabella *«Perché è così corto»* di [`AVVIO-CHAT.md`](AVVIO-CHAT.md) era ferma
> al 2026-08-17 — *«da 15036 a 15040 byte»* — e la cella della §12 era ferma a due passate
> prima, *«15,0, ed è tornato a crescere, `+321 B`»*. Gotcha **#31**, e sono **due** case
> della stessa cifra.
>
> L'insieme *«HANDOFF + spec + `adr/`»* resta **715 KB**. I **due file obbligatori** passano
> da 308 a **317 KB**, e coi tre da 337 a **348**.

> 🔁 **Trentaseiesima misura, il 2026-08-18, chiudendo la manutenzione che il §13 pretende e
> che la passata precedente aveva saltato.** In byte LF, `int(n/1024 + 0.5)`.
>
> | | |
> |---|---|
> | **cresciuti** | [`HANDOFF.md`](HANDOFF.md) `215 → 219` — il testo integrale dei due gotcha nuovi, che è la voce più cara di questa passata · questo file `304 → 308` · `CLAUDE.md` `13 → 14`, la riga di metodo |
> | ⚡ **calato di un byte** | [`AVVIO-CHAT.md`](AVVIO-CHAT.md), che resta **25 KB**: il **messaggio** passa da 15014 a **15013**, perché *«sessantasette»* è una lettera più lungo di *«sessantanove»* |
> | **invariati, ricontati** | [`riferimenti.md`](riferimenti.md) 188 · [`audit-2026-08-11.md`](audit-2026-08-11.md) 31 · [`roadmap.md`](roadmap.md) 29 · [`README.md`](README.md) 17 · [`porta-di-qualita.md`](porta-di-qualita.md) 156 · [`tracciabilita.md`](tracciabilita.md) 15 · [`semi-dst.md`](semi-dst.md) 6 · spec **277** |
>
> ⛔ **E l'aggregato si muove per la prima volta da tre passate: 715 → 719 KB**, tutto da
> [`HANDOFF.md`](HANDOFF.md). ⚠️ **La cifra vive in QUATTRO case** — la testa di questo file,
> `CLAUDE.md`, e **due** punti di [`AVVIO-CHAT.md`](AVVIO-CHAT.md) — ricontate col `grep` e
> aggiornate tutte e quattro nella stessa passata. 📌 È la 20ª misura applicata a sé stessa:
> *le case si contano una volta sola, quando si scrive il rimedio*, e chi riconta non si fida
> del numero scritto nel verbale precedente.
>
> ⚠️ **I due file obbligatori passano da 317 a 322 KB, e coi tre da 348 a 353** — cioè la
> lettura d'avvio è cresciuta di **tre** kilobyte per incassare due gotcha e una riga di
> metodo. ⛔ **Va detto come sta, perché è un costo e non un guadagno:** la 25ª misura
> prescrive di **togliere**, e questa passata ha **aggiunto**. La differenza è che ciò che
> entra è una **lezione** — e la stessa 25ª dice che una lezione non si toglie mai: si toglie
> un rimando duplicato o una voce chiusa. Il rapporto che questo file difende resta
> **353 KB contro 719**.

> 🔁 **Trentasettesima misura, il 2026-08-18, chiudendo il brainstorming del Traguardo 5 e il
> suo disegno.** In byte LF, `int(n/1024 + 0.5)`, a passata chiusa; righe contate **partendo
> dall'elenco dei file citati**, che è il movimento della quindicesima.
>
> | | |
> |---|---|
> | ✅ **riga aggiunta** | il **disegno del Traguardo 5**, **31 KB** — misurata **prima** di scrivere la cella, che è il rimedio della ventunesima alla **terza** applicazione riuscita. ⚠️ Ed è messa **sopra** quella del Traguardo 4: è il file da cui si riprende, non un documento di consultazione |
> | **cresciuti** | questo file `308 → 320` · [`riferimenti.md`](riferimenti.md) `188 → 192` per le quattro misure `D5` · [`HANDOFF.md`](HANDOFF.md) `219 → 222` per il testo integrale del **#70** · [`roadmap.md`](roadmap.md) `29 → 30` · [`AVVIO-CHAT.md`](AVVIO-CHAT.md) `25 → 26` · [`README.md`](README.md) `17 → 18` |
> | **invariati, ricontati** | [`porta-di-qualita.md`](porta-di-qualita.md) 156 · [`audit-2026-08-11.md`](audit-2026-08-11.md) 31 · `CLAUDE.md` 14 · spec **277** · kernel-design 44 · disegno T4 **30** · [`tracciabilita.md`](tracciabilita.md) 15 · [`semi-dst.md`](semi-dst.md) 6 · `design/08` 11 · `design/01` 5 · i piani 68, 50, 162, 168, 114 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
> | ⚠️ **e una cella che questo riquadro aveva già dichiarato invariata** | [`README.md`](README.md) `17 → 18`. Il riquadro era stato scritto **prima** di aggiungervi la riga del disegno, ed è il difetto della **nona** misura — *«un verbale si scrive quando la passata è chiusa»* — colto **prima di committare**, che è la metà del rimedio che la nona prescrive |
>
> ⛔ **E LA NOTIZIA È DI SPECIE NUOVA: UNA CIFRA PUÒ AVERE CASE FALSE, E UN `grep` CIECO LE
> CORREGGEREBBE.** Cercando le case di **353** — la cifra dei tre file da leggere — il `grep` ne
> riporta sei, e **due non sono pesi**: `audit-2026-08-11.md:222` e `riferimenti.md:1546` scrivono
> `COMPENDIO.md:353`, che è un **numero di riga**. Correggerle avrebbe rotto due citazioni esatte
> del finding **A-2** mentre si credeva di riallineare un peso.
> 📌 **Il contro-verso, e costa una lettura:** il `grep` trova le **candidate**, non le case. Ogni
> occorrenza si guarda in faccia prima di toccarla. ⛔ **Ed è il gotcha nuovo #70**, perché il
> difetto vive **dentro il rimedio** che la ventesima e la ventiseiesima misura prescrivono contro
> il #31 — non è il **#41** (là un filtro *esclude*, qui *include* roba di un'altra specie) né il
> **#48** (là sbaglia un banco di *misura*, qui lo strumento ha risposto giusto).
>
> ✅ **E le case sono state ricontate col `grep`, non riprese dal verbale precedente** — che è il
> rimedio della ventesima. La cifra dei **due file obbligatori** ne ha **tre** (una in `CLAUDE.md`,
> due in [`AVVIO-CHAT.md`](AVVIO-CHAT.md)); quella **coi tre file** ne ha **quattro** (una in
> `CLAUDE.md`, tre in `AVVIO-CHAT.md`); l'**aggregato** ne ha **quattro**. I riquadri della §12
> sono **verbali** e non si riscrivono.
>
> ⛔ **E QUESTO RIQUADRO DICEVA *«l'aggregato non si è mosso: resta 719»*, ED ERA FALSO — scritto
> prima di aver finito di scrivere.** `HANDOFF.md` entra nell'aggregato, e il testo integrale del
> gotcha **#70** ce l'ha messo dentro: l'insieme *«HANDOFF + spec + `adr/`»* passa da **719** a
> **721 KB** (738453 B). 📌 È la nona misura per la terza volta — *una misura vera di un momento
> sbagliato è indistinguibile da una misura falsa per chi la legge dopo* — e a coglierla è stata
> la rimisura prima del commit, che è l'altra metà di quel rimedio.
> I **due file obbligatori** passano da 322 a **334 KB**, e coi tre da 353 a **365**.
>
> ⚠️ **E il rapporto che la §12 difende si muove ancora nella direzione sbagliata**, per la quarta
> misura di seguito: il denominatore cresce dello **0,3 %**, il numeratore del **3,4 %**. La
> ragione è la stessa della ventitreesima — un lavoro creativo produce **decisioni**, che vivono
> qui, e poco perimetro nuovo, che vivrebbe nella spec.
>
> ⚠️ **E il messaggio è cresciuto di `+573 B`, da 15013 a 15586**, cioè `+3,8 %`. Due voci: la
> **specie** del lavoro è cambiata — non più creativo, ma la traduzione di un disegno in compiti,
> e la skill che serve subito con essa — e il gotcha **#70**. ⛔ **Nessuna compressione fatta**, e
> va detto così: il blocco delle lezioni dell'audit è già un rimando, e togliere una **lezione**
> la 25ª misura lo vieta.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> ventunesima applicazione.

> 🔁 **Trentottesima misura, il 2026-08-18, chiudendo la consegna alla sessione successiva — ed è
> una passata di sola coerenza, come la ventisettesima, aperta per verificare che la precedente
> avesse finito. NON AVEVA FINITO.** In byte LF, `int(n/1024 + 0.5)`.
>
> | | |
> |---|---|
> | ⛔ **una cella con DUE affermazioni false insieme** | [`roadmap.md`](roadmap.md), tabella dei **dodici sotto-progetti**: *«il prossimo è l'ESECUZIONE DELL'AUDIT … ne restano tre»* e *«l'arbitro GPU riparte dopo, e si comincia dal brainstorming»*. Entrambe false, e la seconda lo è diventata **in questa stessa sessione** |
> | **cresciuti** | [`HANDOFF.md`](HANDOFF.md) `222 → 223` per la seconda forma del **#70** · questo file `320 → 323` |
> | ⚡ **calato** | [`roadmap.md`](roadmap.md) `30 → 29`: la cella riscritta è **più corta** di quella che portava il puntatore e i conteggi. Togliere costa meno che ricorreggere, anche in byte |
> | **invariati, ricontati** | [`riferimenti.md`](riferimenti.md) 192 · [`README.md`](README.md) 18 · [`AVVIO-CHAT.md`](AVVIO-CHAT.md) 26 · `CLAUDE.md` 14 · [`porta-di-qualita.md`](porta-di-qualita.md) 156 · [`audit-2026-08-11.md`](audit-2026-08-11.md) 31 · spec **277** · il disegno del Traguardo 5 **31** |
>
> ⛔ **E LA NOTIZIA È COME QUELLA CASA È SOPRAVVISSUTA A DUE CENSIMENTI.** Il primo è la **passata
> di coerenza del 2026-08-18**, il cui verbale dichiara *«cinque documenti, nove case»* con
> `roadmap.md` **fra i cinque**: le case sistemate erano quelle della tabella dei traguardi, e
> questa vive in un'**altra tabella dello stesso file**. 📌 *Un censimento che trova il **file** non
> ha trovato le sue **case**.* Il secondo censimento è di **oggi**, e ha riportato la riga giusta —
> `roadmap.md:128` — che è stata scartata dopo **centoventi caratteri**, per via del filtro messo lì
> a rendere leggibile l'elenco. ⛔ **È la seconda forma del gotcha #70, nata mezz'ora dopo la
> prima:** là il difetto è nell'**uscita** del `grep`, qui nel **filtro** che gli si mette dopo. Una
> riga trovata da un censimento **si legge intera**, o il censimento non è stato fatto.
>
> ⚠️ **E la cella lo aveva scritto di sé stessa**, che è ciò che la rende utile: *«questa cella si
> limita al conteggio: ricopiarlo qui è ciò che ha fatto invecchiare questa riga tre volte»* — e lo
> ricopiava. Gotcha **#68**, la regola violata dentro il documento che la contiene, alla seconda
> occorrenza misurata. ✅ **Rimedio: tolto**, non ricorretto — il puntatore e i conteggi vivono
> nella §6, in un posto solo.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da 721 a **722 KB**. I **due file obbligatori** vanno
> da 334 a **337 KB**, e coi tre da 365 a **368**. ✅ **Il messaggio non si è mosso: 15586 byte** — questa
> passata non lo ha toccato.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> ventiduesima applicazione.

> 🔁 **Trentanovesima misura, il 2026-08-18, chiudendo il PIANO del Traguardo 5 — ed è la
> seconda passata dello stesso giorno**, come la ventunesima e la ventinovesima. In byte LF,
> `int(n/1024 + 0.5)`, a passata chiusa; righe contate **partendo dall'elenco dei file citati**,
> che è il movimento della quindicesima.
>
> | | |
> |---|---|
> | ✅ **riga aggiunta** | il **piano del Traguardo 5**, **174 KB** — misurato **prima** di scrivere la cella, rimedio della ventunesima alla **quarta** applicazione riuscita. ⚠️ Ed è messo **sopra** il disegno del Traguardo 4: è il file da cui si riprende |
> | **cresciuti** | [`riferimenti.md`](riferimenti.md) `192 → 198` per le sette misure `P5` · questo file `323 → 335` · [`HANDOFF.md`](HANDOFF.md) `223 → 226` per il testo integrale del **#71** · [`roadmap.md`](roadmap.md) `29 → 30` |
> | **invariati, ricontati** | [`README.md`](README.md) 18 — ⚠️ **toccato e invariato**, otto righe non bastano a muovere l'arrotondamento · [`AVVIO-CHAT.md`](AVVIO-CHAT.md) 26 · [`porta-di-qualita.md`](porta-di-qualita.md) 156 · [`audit-2026-08-11.md`](audit-2026-08-11.md) 31 · `CLAUDE.md` 14 · spec **277** · il disegno del Traguardo 5 **31** · disegno T4 30 · kernel-design 44 · [`tracciabilita.md`](tracciabilita.md) 15 · [`semi-dst.md`](semi-dst.md) 6 · i piani 68, 50, 162, 168, 114 · `RISULTATI.md` 23 · `GUI-REQUISITI.md` 6 · ADR `2–19` |
>
> ⛔ **E LA NOTIZIA È IL PIANO STESSO: NASCE PIÙ GRANDE DI QUANTO OGNI PIANO PRECEDENTE SIA MAI
> DIVENTATO.** I due più grandi sono il Traguardo 3 a **168 KB** e il Traguardo 2 a **162** —
> e quelle cifre sono **dopo** le rispettive errata, settantasette voci in nove passate e
> quarantanove in sei. Questo parte da **174** con l'errata **vuota**.
> 📌 **La causa è misurabile e non è prolissità:** il perimetro è *«l'arbitro intero»* (§0.1 del
> disegno), i compiti sono **tredici** contro dodici e dieci, e i vincoli globali di questo
> repository pretendono in **ogni** passo il codice per esteso, le mutazioni con la propria
> attesa, e il comando col proprio esito. ⚠️ **Va detto come sta invece di essere spiegato:** se
> l'errata cresce come le due precedenti, quel file passa i **200 KB** — e la §12 esiste per
> dire a chi legge **se aprire**, non per giustificare. La riga della tabella dice *«a compiti,
> mai intero»*, che è l'unica difesa che ha.
>
> ⚠️ **E il rapporto che la §12 difende si muove ancora nella direzione sbagliata, per la quinta
> misura di seguito:** il denominatore cresce dello **0,6 %**, il numeratore dell'**1,8 %**. La
> ragione è la stessa della ventitreesima e della trentasettesima — un lavoro di **disegno o di
> piano** produce decisioni e misure, che vivono qui, e poco perimetro nuovo, che vivrebbe nella
> spec.
>
> ⛔ **E QUESTA PASSATA HA CREDUTO DI AVER COLTO IL GOTCHA #10 E SI SBAGLIAVA — la divergenza si
> registra invece di allinearla all'attesa.** Lo strumento di edit ha davvero **normalizzato i
> fine-riga** di `COMPENDIO.md` e `roadmap.md` da LF a **CRLF**, misurato: `CR` da **0** a `2894`
> e `282`, cioè **ogni riga**. La conclusione scritta di getto era *«`git diff` dichiarerà
> duemilanovecento righe cambiate»*. ✅ **Misurata, è FALSA:** questo repository ha
> `core.autocrlf=true`, quindi git normalizza CRLF→LF entrando nell'indice, e il diff è rimasto
> **pulito** — sessantadue righe su un file di 2894.
> ⛔ **E il perché vale più dell'errore, perché dice DOVE la trappola morde davvero.** Censiti i
> blob committati: i file con `CR` nell'indice sono **quattro in tutto il repository**, e sono
> **tutti sorgenti Rust** — `crates/kernel/src/ports/process.rs` (291),
> `crates/kernel/tests/ports_are_implementable.rs` (971),
> `crates/kernel/tests/reactor_contract.rs` (669), `crates/platform/src/reactor.rs` (123).
> **Nessun documento.** Su un documento LF la normalizzazione è assorbita da `autocrlf`; su
> quei quattro **no**, ed è per questo che il caso di `G-5` — `sed -i` su un `Cargo.toml` — si
> vide e questo no.
> 📌 **La regola di `CLAUDE.md` regge e diventa più precisa:** *chi tocca uno di quei quattro file
> conserva i fine-riga di quel file, e li rimisura dopo*. ⛔ **E due dei quattro sono file che il
> piano del Traguardo 5 modifica al Task 4** — `ports/process.rs` e `ports_are_implementable.rs`
> — quindi l'avvertenza è scritta **dentro il piano**, dove serve, e non solo qui.
> 📌 **E la riga di `CLAUDE.md` nomina `sed -i`, mentre lo strumento colpevole qui è un altro:** la
> regola vale per **qualunque** strumento che riscriva un file, non per il comando che l'ha
> insegnata.
> ⚠️ **E un secondo tentativo è stato fermato dalla decima forma del #48:** una sostituzione via
> `python - <<'PY'` con testo non-ASCII **non ha trovato nulla** — lo stdin è decodificato nel
> codepage di sistema — ed è fallita sull'`assert` invece di applicare metà delle modifiche. La
> difesa era già scritta nel gotcha; a farla scattare è stato l'`assert`, non la memoria.
>
> ⛔ **Il MESSAGGIO: 15586 → 15741 byte, `+155 B` e `+1,0 %`** — e la crescita è **tutta di
> specie**, non di contenuto. Il messaggio ordinava *«traduci un disegno in un piano»* e ora
> ordina *«esegui un piano, un compito per volta»*: `writing-plans` e
> `subagent-driven-development` si **scambiano di posto** fra le skill che servono subito e
> quelle che serviranno, e `test-driven-development` sale, perché il lavoro che viene **è
> codice**. ✅ **Nessun blocco appeso:** le due voci che entrano ne sostituiscono due che escono,
> che è la regola della 26ª — *si toglie un rimando duplicato o una voce chiusa, mai una
> lezione*.
>
> L'insieme *«HANDOFF + spec + `adr/`»* passa da **722** a **726 KB** (742950 B). I **due file
> obbligatori** passano da 337 a **349 KB**, e coi tre da 368 a **380**.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> ventitreesima applicazione.

> 🔁 **Quarantesima misura, il 2026-08-18, chiudendo la CONSEGNA alla sessione che eseguirà —
> ed è una passata di sola coerenza, come la ventisettesima e la trentottesima, aperta per
> verificare che il messaggio d'avvio reggesse per una specie di lavoro diversa. NON REGGEVA,
> in cinque punti.** In byte LF, `int(n/1024 + 0.5)`, a passata chiusa.
>
> | | |
> |---|---|
> | ⛔ **la falsità più cara** | il blocco delle **questioni aperte di QUALITÀ** elencava **sei** difetti — il turn limit ignorabile, quattro gruppi su cinque della conformità `reactor`, il finto filesystem sui `CheckpointId`, la via **A3**, il giornale a `0644`, `bincode` non mantenuto — e **tutti e sei sono chiusi dal 2026-08-18**, dalle otto decisioni dell'audit. Il messaggio ordinava al prossimo agente di credere aperti sei difetti che non lo sono |
> | **altre quattro stantie** | il peso dell'audit `29 → 31 KB` · *«i TRE piani più grandi 168, 162 e 114»*, dove il più grande è ora **174** · *«le QUATTRO domande»* del pre-controllo, mentre trenta righe più su lo stesso messaggio ne diceva **SETTE** — due cifre della stessa cosa **dentro lo stesso file** · e il blocco sul gotcha **#58** citava solo il caso del Traguardo 4 |
> | ⚡ **calato** | [`AVVIO-CHAT.md`](AVVIO-CHAT.md) `26 → 25` |
> | **cresciuto** | questo file `335 → 339` |
> | **invariati, ricontati** | `CLAUDE.md` 14 · [`HANDOFF.md`](HANDOFF.md) 226 · [`riferimenti.md`](riferimenti.md) 198 · [`roadmap.md`](roadmap.md) 30 · [`README.md`](README.md) 18 · il **piano del Traguardo 5** 174 · [`porta-di-qualita.md`](porta-di-qualita.md) 156 · [`audit-2026-08-11.md`](audit-2026-08-11.md) 31 · spec **277** · il disegno del Traguardo 5 31 |
>
> ✅ **IL MESSAGGIO: 15741 → 14460 byte, `−1281 B`, cioè `−8,1 %`** — ed è il **secondo** calo da
> quando si conta, dopo quello della 34ª. ⛔ **Ha pagato la regola della 26ª applicata alla
> lettera:** ciò che è stato tolto è il **riassunto dell'audit** — le due falle del cancello, il
> #59, ciò che l'audit ha trovato sano, ciò che lascia al Traguardo 5 — cioè un **rimando
> duplicato**, perché quel file è uno dei **tre che il messaggio ordina di leggere per intero**.
> ✅ **Le lezioni non sono state tolte ma spostate**, che è la distinzione della 26ª: il
> **#71** entra accanto al #58, e le **sette** domande del pre-controllo sostituiscono le quattro.
>
> ⛔ **E LA NOTIZIA DI QUESTA MISURA È CHE IL DIFETTO ERA GIÀ STATO SEGNALATO E NON CHIUSO.** Le
> sei questioni chiuse furono riportate al proprietario **all'apertura della sessione
> precedente**, confrontando il messaggio incollato con la §5 dell'audit — *«sei su sei»* — e la
> sessione ha poi scritto un piano, aggiornato nove documenti e committato **senza toccarle**.
> 📌 **La classe è nuova e vale oltre il caso: una divergenza SEGNALATA A VOCE non è una
> divergenza REGISTRATA.** Il verbale la conserva, la conversazione no — e chi l'ha detta è
> esattamente chi smette di cercarla, perché ricorda di averne parlato. ⚠️ **Non è la radice R1**,
> dove una correzione non attraversa gli altri documenti: qui la correzione **non è mai
> esistita**, è esistito solo il suo annuncio.
>
> L'insieme *«HANDOFF + spec + `adr/`»* resta **726 KB**. I **due file obbligatori** passano da
> 349 a **353 KB**, e coi tre da 380 a **384**.
>
> ⛔ **La cifra dei due file descrive il file che la contiene**, quindi è rimisurata **dopo** aver
> chiuso questo riquadro e corretta **di sole cifre** — metodo della sesta misura, alla
> ventiquattresima applicazione.

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
