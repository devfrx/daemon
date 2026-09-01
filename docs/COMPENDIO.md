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
> (812 KB in byte LF il 2026-08-27, e possono solo crescere — la spec da sola ne fa 298), e
> l'idea è già qui.

**Aggiornato il 2026-08-27**, coi rimedi del secondo audit; l'ultimo contenuto di **merito** è la
chiusura del Traguardo 5 — il Task 13, che è un audit e non una scrittura. Manutenzione: §13.
⚠️ **Questa riga ha sbagliato due volte su due, e la seconda è il finding AUD-034.** Diceva
**2026-08-11** dopo decine di passate; poi **2026-08-25**, mentre `f2bc784` — un'ondata di
correzione — l'aveva riscritto nel merito il **2026-08-26**. È il gotcha **#31** sull'intestazione,
che nessuno rilegge perché è la cornice e non il contenuto: la stessa forma trovata **quattro**
volte su [`HANDOFF.md`](HANDOFF.md).
⛔ **E la causa è strutturale, REGISTRATA E NON PRESA perché tocca il modo di lavorare:** la §13
aggancia la manutenzione a *«ADR nuovo, ADR superato, voce chiusa, gotcha nuovo, misura nuova,
decisione dello stack, cambio del prossimo passo»*, e **nessuno** di quei ganci copre una
**riscrittura di merito dentro una riga che c'è già** — quindi nulla richiama la data, e infatti
l'ha riportata qui il rimedio di un audit e non la manutenzione. Aggiungere un gancio cambia la
§13, ed è del **proprietario**.

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
| schema IPC | **`bincode` 2.0.1** — appuntato a `2`. ⚠️ **Dichiarato NON MANTENUTO** — RUSTSEC-2025-0141, `INFO`, non una vulnerabilità. ⛔ **RICHIAMO DEL 2026-08-31: qui stava *«registrato il 2026-08-18, si decide al Traguardo 6»*, e il Traguardo 6 ha MISURATO.** L'avviso è ancora attivo e il monte è archiviato; esistono alternative **mantenute**, e per una di esse lo **stesso formato sul filo** non è più una dichiarazione ma una **misura** — **M-12**, del 2026-08-31. ✅ **DECISO il 2026-08-31 dal proprietario: `bincode` 2.0.1 RESTA e §6.1.1 non si riapre** — contro l'evidenza di **M-12** e non attorno, perché la radice di C-1 è il **buco fra due criteri** e non questa crate; la cura alla radice è la voce **X-3**, che resta aperta. Le ragioni in [`porta-di-qualita.md`](porta-di-qualita.md), le fonti in [`riferimenti.md`](riferimenti.md) | M-1 · §6.1.1 · gotcha #22 · C-1 |
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
dall'arbitro, e una riserva sistematicamente sbagliata è un difetto del profilo.
⚠️ **RICHIAMO DEL 2026-08-27, finding AUD-032:** quei cinque attributi sono realizzati da
**due** strutture e non da un oggetto solo — `ResourceProfile` ne porta **quattro**, con
prelazionabilità e tempo di rilascio fusi in `Preemption`, e `cold_start` vive in
`WorkDescriptor`, che l'ammissione **non riceve**. La decisione **non è superata**; il perché
delle due divergenze sta nel **rimando datato in testa ad ADR-0005**, in una casa sola. **La
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
| `ipc` | TypeScript | **M-11**: `bincode-ts` decodifica, valori giusti | **`bincode`** — §6.1.1 **confermata**, non riaperta. ⛔ **RICHIAMO DEL 2026-08-31:** la misura **C-1** del Traguardo 6 ha trovato alternative **mantenute** al **nostro** capo del filo, e **M-12** ha misurato che una di esse mette gli **stessi byte** sul filo — e il proprietario ha **deciso il 2026-08-31 di NON riaprirla**, con quell'evidenza in mano: §6.1.1 resta **confermata**. ⚠️ Il **pari** invece non è cambiato: `bincode-ts` è fermo alla 1.0.0 del 2025-07-17, cioè esattamente ciò che M-11 misurò |
| `process` | Python | **M-10**: nessuna libreria per `bincode` | **`minicbor`** — voce già spedita, lista invariata |

⚠️ **Due canali privati con formati diversi non sono un'incoerenza:** la differenza è
**misurata**, non accidentale, e non va «sanata». ⛔ E un decodificatore scritto e
mantenuto **da noi** nel linguaggio del pari **non è una via**: è una seconda definizione
dello schema, e misurato sbaglia **in silenzio** — un lettore ingenuo del varint ha
restituito `251` al posto di `4096` senza sollevare nulla.

---

## 6. Dove siamo, e cosa viene dopo

✅ **LO SFOLTIMENTO DEL COMPENDIO È CHIUSO IL 2026-08-28.** La lettura obbligatoria di
apertura sessione è passata da **230 995** a **88 796** token — **2,6 volte** — senza
perdere una decisione e senza toccare una riga di prodotto. ⛔ **Il freno è nel cancello e
non nella buona volontà:** `check-docs.sh` respinge un compendio sopra il proprio tetto,
provato in **tre** direzioni. Il disegno sta in
[`superpowers/specs/2026-08-28-sfoltimento-compendio-design.md`](superpowers/specs/2026-08-28-sfoltimento-compendio-design.md),
i verbali in [`archivio/`](archivio/). ⚠️ **I numeri si rifanno col comando**, che sta in
[`riferimenti.md`](riferimenti.md), e non si rileggono qui.

✅ **IL SECONDO AUDIT COMPLETO — 2026-08-27 — HA I SUOI FINDING TUTTI CHIUSI DAL 2026-08-28**,
[`audit-2026-08-27.md`](audit-2026-08-27.md), col rosso o la misura riprodotti **prima** del
rimedio ogni volta che il finding tocca codice o cancello. ⚠️ **Nessun numeratore qui, nemmeno
adesso che sarebbe comodo:** lo dice la colonna «Stato» di quel rapporto, che ne è la **casa
unica**, e il comando sta in fondo a questa sezione — gotcha #68, che è la radice **R3**
dell'audit stesso. Diciotto revisori in parallelo, ognuno smentito da un secondo; **98 finding
proposti, 25 scartati, 73 finali** in **sette radici**.
⛔ **MA IL RAPPORTO NON È FINITO, e la differenza va detta invece che arrotondata:** restano le
**voci senza numero AUD**, la cui tabella in quel file è la casa unica, e la maggior parte sono
**decisioni del proprietario**. ⛔ **RICHIAMO DEL 2026-08-28: questo blocco diceva *«ED È IL
COMPITO DI OGGI»* e *«la specie del lavoro è RIMEDIO — niente brainstorming, niente plan
mode»*.** Entrambe erano vere e non lo sono più: il rapporto resta lettura d'apertura come
**verbale e delega** — il metodo con cui qui si rimedia — e il prossimo passo **riparte proprio
dal brainstorming**. ⛔ **Si legge fino alla tabella dei 73, mai il «Dettaglio» intero:** sono 73
schede, e se ne apre **una** per volta, come un piano si legge a compiti.

⚠️ **I RACCONTI DEI SINGOLI FINDING NON VIVONO PIÙ QUI, dal 2026-08-28.** Questa §6 ne portava
uno per **AUD-013** mentre due paragrafi più sopra dichiarava che *«la colonna Stato di quel
rapporto è la casa unica»*: era una **seconda casa**, e sotto il tetto del compendio era anche un
conto alla rovescia. 📌 **Dove sono adesso:** il *che cosa è stato fatto* nella colonna
omonima di [`audit-2026-08-27.md`](audit-2026-08-27.md), il racconto per esteso in
[`archivio/stato-storico.md`](archivio/stato-storico.md) — che ne porta anche le **misure**, con
la data — e ciò che **resta aperto** nella tabella delle voci senza numero AUD di quel rapporto,
dove `E64` è entrata come **X-4**. ⛔ **È una decisione del proprietario, presa il 2026-08-28:**
le due regole si scontravano davvero — *«alla chiusura di ogni voce si aggiorna COMPENDIO.md»*
contro il tetto che lo sfoltimento ha messo nel cancello.

**Spec del kernel §0–§10 completa.** Spec del **sotto-progetto 1** con §0–§8 approvate,
**riaperta su sette voci** — **tutte chiuse** — **§8 riallineata e chiusa il 2026-08-08**, e
**audit sezione-contro-ADR passato**.

### I sei traguardi del sotto-progetto 1

| # | | |
|---|---|---|
| 1 | scheletro e porta di qualità | ✅ 2026-08-08 |
| 2 | substrato iniettabile | ✅ 2026-08-10 |
| 3 | giornale e formato durevole | ✅ 2026-08-10 |
| 4 | simulatore DST — il guasto | ✅ 2026-08-11 |
| 5 | arbitro GPU | ✅ 2026-08-25 |
| 6 | gli altri meccanismi | ⬜ **davanti** |

⚠️ **Nessun numeratore di compiti in questa tabella**, per costruzione: invecchierebbe a
ogni compito. Il racconto di ciascun traguardo sta nel proprio piano, in
[`superpowers/plans/`](superpowers/plans/); i verbali di chiusura stanno in
[`archivio/stato-storico.md`](archivio/stato-storico.md).

### Il prossimo passo

⛔ **Il TRAGUARDO 6** — gli altri meccanismi: il gateway di inferenza, i sensori, i permessi, lo
stato di degrado e il canale verso i worker. ✅ **IL BRAINSTORMING È CHIUSO E IL DISEGNO È COMPLETO dal 2026-08-30**, tutte le sezioni
approvate una per volta. ⚠️ **Nessun conteggio di sezioni qui, e non è mai stato riallineato:**
questa riga disse *«cinque sezioni … la 6 e la 7 non sono state nemmeno presentate»*, poi fu
**tolta** invece che portata a sei, e oggi non c'è più niente da contare. La casa unica resta
l'intestazione del disegno (gotcha **#68**). Vive in
[`superpowers/specs/2026-08-28-…-traguardo-6-altri-meccanismi-design.md`](superpowers/specs/2026-08-28-sottoprogetto-1-traguardo-6-altri-meccanismi-design.md),
che ne è la casa unica e dichiara di sé di essere **completo**. ⚠️ **RICHIAMO DEL 2026-08-30:**
qui stava *«dichiara di sé di essere in corso»*, ed è diventato falso lo stesso giorno. ✅ **La condizione per passare a `writing-plans` è CHIUSA il 2026-08-30**, e non era quella che
sembrava. Questa riga diceva *«finché le sezioni mancanti non ci sono»*, e non ce ne sono più;
poi che a governare fosse la chiusura delle **voci aperte**, e quella regola era
**insoddisfacibile** — gotcha **#89**, il verbale in [`HANDOFF.md`](HANDOFF.md). ⛔ **A sbarrare
è la colonna *«Chi la chiude»*** di [`porta-di-qualita.md`](porta-di-qualita.md): una voce il cui
chiusore è **questo traguardo** va chiusa o portata dal piano, le altre si **sanno**. ⚠️ **Quali
sbarrino oggi non è scritto qui:** lo dice il comando che vive in quella sezione.
✅ **E L'ULTIMO SBARRAMENTO È CADUTO IL 2026-08-30.** Era la §7.4.6 della **spec**, che dava per
costruito il **trasporto reale** di `ipc` e `process`: un piano scritto contro di essa sarebbe
risultato **mancante di due compiti**. Le due celle passano a **❌ scaglionata** — la parola che
quella colonna già usa — col richiamo datato, deciso dal proprietario (vincolo globale 7).
⛔ **E chiudendolo si è misurato che il DISEGNO sbagliava la RAGIONE, non il perimetro:**
attribuiva la non-costruzione alla metà di **prontezza** del `reactor`, mentre le due porte sono
**a interrogazione** — `Ok(None)` è risposta ordinaria, e il doc di `Ipc::receive` scrive
*«or the core could not poll this port at all»*. A tenerle fuori sono **§0.2** e **§0.4 riga §1**.
Terza occorrenza del gotcha **#58**: chi prezzava aveva letto una **guardia**, non le due porte.
⚠️ **Prima** si legge ciò che il Traguardo 5 lascia aperto: la tabella *«Le voci aperte del
Traguardo 5»* di [`porta-di-qualita.md`](porta-di-qualita.md), casa unica, che dice anche **chi
chiude** ciascuna. ⚠️ **Le voci senza numero AUD** dell'audit del 2026-08-27 restano aperte e in
gran parte del proprietario — la loro tabella in quel rapporto è la casa unica — ⛔ **ma NON
sbarrano:** questa riga diceva *«ciò che lo sbarra sono decisioni del PROPRIETARIO»*, contro la
riga qui sopra e contro il gotcha **#89**, a nove righe di distanza nello stesso blocco.

✅ **E IL PIANO È FINITO IL 2026-08-30**, cinque parti in un giorno. Vive in
[`superpowers/plans/2026-08-30-…-traguardo-6-altri-meccanismi.md`](superpowers/plans/2026-08-30-sottoprogetto-1-traguardo-6-altri-meccanismi.md),
che ne è la casa unica. ⚠️ **RICHIAMO DEL 2026-08-30: qui stava *«APERTO … in scrittura e NON
finito»*.** ⛔ **Quanti compiti abbia e a che punto sia NON è scritto qui**, ed è il rimedio e non
una svista: un numeratore invecchierebbe a ogni passo, e il piano lo dichiara di sé nella propria
intestazione — gotcha **#68**, la stessa cura che la cella del Traguardo 5 ha ricevuto due volte.
📌 **Il suo pre-controllo ha trovato qualcosa in ogni parte scritta, e più d'una ha cambiato il
prezzo del disegno.** ⛔ **RICHIAMO DEL 2026-08-30: qui c'era *«ha già trovato quattro cose, e
due cambiano il prezzo»*, ed era stantio dopo tre parti scritte.** Il numerale è **tolto e non
riallineato**, come la §12 fece col peso e la §11 col numeratore dei vincoli: è un cumulativo, e
la sua casa unica è la sezione del pre-controllo **dentro il piano**.
⚠️ **La più grande tocca l'artefatto irreversibile** — come una **specie nuova** di record
durevole entri nel formato — ed è chiusa **con una misura** e non con un argomento; la
**composizione** è stata misurata sui byte congelati veri prima di scrivere la parte che vi
poggia, e la **forma** che ne discende è una decisione del proprietario **rimandata ai criteri**.
Il racconto, le misure e le decisioni vivono nel piano, non qui; ciò che ne è uscito di generale
è il gotcha **#90** di [`HANDOFF.md`](HANDOFF.md).
✅ **IL COMPITO 1 È ESEGUITO, e con esso si chiudono TRE voci aperte del Traguardo 5** — `E30`,
`R6` ed `E21`, marcate nella tabella unica di [`porta-di-qualita.md`](porta-di-qualita.md) col
proprio commit. L'arbitro ha un'**identità consegnata**, `release` risponde **tre** cose e non due,
e la porta `process` **restituisce la concessione** — anche sull'avvio fallito, la via che nessuno
aveva discusso. ⛔ **A che punto sia il piano NON è scritto qui**, e il racconto compito per compito
neppure: la casa unica è l'intestazione del piano, e le divergenze la sua **errata**.
⚠️ **E il pre-controllo ha tenuto: ha prodotto voci d'errata PRIMA di dispacciare**, fra cui una sonda dettata che
**non poteva compilare** e un censimento che non vedeva tre file su cinque. Il ciclo di revisione è
**tornato pulito**, e ciò che ne è uscito di generale è il gotcha **#91**.
✅ **IL COMPITO 3 È ESEGUITO IL 2026-08-31, e con esso il VINCOLO 15 della §11 è ONORATO** — la
busta dei due canali privati e lo schema del canale worker. La riga del vincolo 15 **esce** dalla
tabella *«cosa resta davanti»* della §11, che è la forma che quella sezione prescrive: una riga che
sparisce non è una riga corretta.
⚠️ **E il pre-controllo ha tenuto una seconda volta, con la voce più cara del traguardo: una sonda
dettata era VACUA.** Non sbagliata — **corretta**, e disarmata dal **dato** che il piano le dava:
sarebbe passata **verde** senza distinguere ciò che esiste per distinguere. ⛔ **E la revisione ne ha
trovata una seconda sul prodotto — stesso verde che non prova niente, ma specie diversa:** una via
d'errore che **nessuna** sonda raggiungeva — mutata, l'intero workspace restava verde **cifra per
cifra**. Ciò che ne è uscito di generale è il gotcha **#92**, dalla prima.
✅ **IL COMPITO 3BIS È ESEGUITO IL 2026-08-31, e la misura C-1 è RIFATTA da fonti primarie invece che
ricordata** — deciderlo a memoria sarebbe stato il gotcha **#48**. **Esito:** RUSTSEC-2025-0141 è
ancora **attivo**, il monte di `bincode` è **archiviato** e l'ultima versione pubblicata è il
segnaposto `compile_error!`; e alternative **mantenute** esistono, una delle quali dichiara **lo
stesso formato sul filo**, cioè lascerebbe il pari TypeScript dov'è.
⚖️ **È il CASO B, e il compito si è FERMATO PRIMA DI DECIDERE:** §6.1.1 è spec, la riapertura è la
**D12** ed è del proprietario. Le fonti stanno in [`riferimenti.md`](riferimenti.md) e la voce aperta,
col chiusore, in [`porta-di-qualita.md`](porta-di-qualita.md).
✅ **E LA MISURA M-12 È FATTA IL 2026-08-31, per decisione del proprietario — *misurare prima di
scegliere*.** La compatibilità sul filo del fork era una **dichiarazione del suo README**, ed è ora
un **fatto misurato**: cinque casi byte per byte identici, andata-e-ritorno incrociata **sui
valori**, e il pari `bincode-ts` che legge i byte del fork **coi valori giusti**. ⛔ **E la misura
ha portato due costi che nessuno aveva:** il grafo spedito di ADR-0031 crescerebbe di **una voce
netta**, e una compatibilità misurata **oggi** non vincola le versioni **future** del fork.
✅ **E C-1 È DECISA LO STESSO GIORNO: `bincode` 2.0.1 RESTA, §6.1.1 non si riapre.** ⛔ **La ragione
che decide non è nessuna delle due che sembravano:** la radice di C-1 è il **buco fra due criteri**
— nessuno chiede come stia la libreria al **nostro** capo — e sostituire **una** libreria cura una
crate lasciando il buco aperto per le altre **sette**. La cura alla radice è la voce **X-3**
dell'audit del 2026-08-27, che resta **aperta e del proprietario**. Le cinque ragioni stanno in
[`porta-di-qualita.md`](porta-di-qualita.md), voce **C-1**, in una casa sola.
✅ **IL COMPITO 4 È ESEGUITO IL 2026-08-31, `GATE GREEN` a ogni commit** — la busta `ipc` porta
i due messaggi, sul ramo `bincode` che la decisione **C-1** ha lasciato dov'era.
⛔ **E il fatto che conta è un RILIEVO DI MERITO della revisione, non un difetto del prodotto:**
il doc del tipo spendeva **quattro** paragrafi sul campo che **rifiuta** — `name`, contenuto non
fidato — e **nessuno** sui due che l'arbitro **obbedisce**. `compute_class` sceglie la corsia e
apre la guardia di `ask_back`; `preemption` decide se una concessione possa essere richiamata:
arrivano dal **medesimo pari** del nome, sono un privilegio **più forte di una stringa**, e non
sono controllati contro **niente** — mentre `reserved_vram` deve superare il tetto in `admit`.
⚖️ **Decisione del proprietario, il 2026-08-31: si DICHIARA il costo col proprio innesco, e i
campi NON si restringono.** Restringerli riaprirebbe la **D16** per proteggere un chiamante **che
non esiste**, cioè il gotcha **#46** dal verso sbagliato. ⛔ **L'innesco è il CONSUMATORE:** il
compito che per primo decodificherà byte in un `ResourceProfile` è dove si scrive la metà
**verificante** di ADR-0005, che oggi non ha casa.
✅ **IL COMPITO 5 È ESEGUITO, `GATE GREEN` a ciascuno dei tre commit** — il
contratto del sensore, il campo `detail` all'indice **5** con la variante `RecordKind::Verdict`, e
l'anello che giornala. `V10` passa a **coperta** nel catalogo, e nel blocco C resta scoperta la
sola `V5`. ⛔ **E il traguardo ha toccato l'ARTEFATTO IRREVERSIBILE:** nasce il **quarto** record
congelato, che porta insieme la variante nuova **e** `detail: Some` — con `None` non pinzerebbe
niente dell'indice 5, perché un `None` in coda non viene scritto. ✅ **L'additività è provata sui
file veri e non su uno specchio:** i tre `.cbor` esistenti, ricodificati dal tipo a sei campi,
tornano **byte-identici**.
⛔ **E ciò che conta di più è che il pre-controllo ha tenuto una TERZA volta, con due voci
bloccanti:** il file dettato **non compilava** — undici nomi fuori dalla lista `use` — e **due
sonde su tre non potevano passare**, perché l'anello scrive una **nota** su un passo che nessuno
aveva aperto e il contratto della porta la rifiuta con `OutOfOrder`. Il costo del rimedio non era
la riga di setup ma gli **oracoli**, riscritti tutti e tre.
⚠️ **E una mutazione è sopravvissuta, chiusa con una sonda in più invece che con un'asserzione più
larga:** l'ordine *«il costo dichiarato si legge PRIMA di eseguire»* — che il doc chiama **V11** —
non era tenuto da nulla, perché un sensore che gira e viene scartato **non scrive niente** come
uno che non gira. Da lì il gotcha **#95**.
⚠️ **IL COMPITO 5 HA LASCIATO UN RESIDUO DI METODO — NESSUN CICLO DI REVISIONE INDIPENDENTE — E
IL 2026-08-31 LA REVISIONE È STATA FATTA, da una sessione fresca e non da un subagente.** ⛔ **E
ha trovato TRE rilievi, di cui DUE mutanti vivi sull'intero workspace**, nessuno dei quali le
cinque mutazioni dettate poteva vedere perché **nessuna delle cinque li toccava**: `Detail` era il
**quarto enum sul filo senza nessuna guardia di crescita** — una variante nuova entrava nel formato
durevole lasciando la baseline identica **cifra per cifra**, mentre la stessa aggiunta a
`RecordKind` è `` error[E0004] `` — e la **classe d'effetto del passo di feedback** è tenuta da
nulla e non argomentata, mentre quella del verdetto, che `reconcile` non legge mai, porta cinque
righe. ✅ **Entrambi sono chiusi**, il secondo per **decisione del proprietario del 2026-08-31: la classe
la consegna il chiamante**, `correction_effect: EffectClass`. ⚖️ **E a decidere non è la lettera di
ADR-0007 ma la coerenza:** è ciò che quella stessa funzione fa una riga sopra per `next`, ed è la
forma di ADR-0034 — l'anello non sa che cosa farà la correzione, e smette di indovinarla. ⛔ **E LA SECONDA PASSATA È STATA FATTA LO STESSO GIORNO, sul perimetro allargato che la prima
lascia: ALTRI DUE rilievi, `E65` ed `E66`.** Quattro **mutanti vivi** su `run_the_ring`, che scrive
**due** record mentre il banco ne teneva per intero uno — e il più grave fa marcare `Instruction`
un payload che viene da un artefatto `Untrusted`, cioè **I6 dentro il formato durevole**. E tre
affermazioni di data della specie che la passata precedente aveva **deciso** di togliere,
sopravvissute perché quel censimento non arrivò mai al sorgente (radice **R1**, passo 3 della
disciplina d'audit). ✅ **Il prodotto della prima passata REGGE**, misurato e non assunto.
⛔ **E IL TERZO GIRO È STATO FATTO LO STESSO GIORNO, IL PRIMO DA UN SOTTO-AGENTE** — il
proprietario li ha autorizzati, ed era la condizione che `E53` registrava come mancante. Altri
**sei** rilievi, `E67`–`E72`: il gotcha **#98** che riproduce sul campo accanto, i conteggi del
compito 5 sopravvissuti nella testa dei byte congelati e nella loro mappa, e un'annotazione che
due documenti dichiaravano portante ed è **inerte**, misurata. ⚠️ **E uno dei sei era prezzato più
grande del difetto** — gotcha **#65** applicato al rapporto di un sotto-agente.
⛔ **Il residuo è RIDOTTO UNA TERZA VOLTA e ancora non chiuso**, per la regola stessa: **tre**
giri, e **tutti e tre** hanno trovato difetti veri — *«finché una passata non torna pulita»*.
La casa unica resta `E53` dell'errata; le misure
stanno in [`porta-di-qualita.md`](porta-di-qualita.md), e ciò che ne è uscito di generale sono i
gotcha **#96**, **#97** e **#98** — l'ultimo è che un **record congelato** è un oracolo di
**formato** e non di comportamento, e il valore che il suo letterale sceglie può lasciare l'altro
inesercitato in tutto il workspace.
⏭️ **IL PROSSIMO PASSO È IL COMPITO 7** — `E94` è **decisa e chiusa** il 2026-09-01, e da essa
nascono `E95`, che detta al compito 7 di nascere sigillato, ed `E96`, che è del proprietario.
✅ **Il pre-controllo È FINITO il 2026-09-01 — casa unica `E102` più `E103`.** Le quattro domande
hanno dato un **terzo bloccante** che `E102` non aveva: `grant` scrive una **nota** su un passo che
nessuno ha aperto, e la porta la rifiuta — **tutte e cinque** le sonde dettate falliscono. Seconda
occorrenza esatta della voce bloccante del compito 5, e ciò che la smentiva stava **nel banco di quel
compito** (gotcha **#58**). ⚖️ **DUE scelte aspettano il proprietario e il compito NON si dispaccia
prima:** la variante di `JournalError`, che tocca il **contratto di una porta** condivisa da due
implementazioni, e la **forma sigillata** che `E95` impone a `PermissionDetail`, dove il precedente si
biforca fra `RoutingDetail` e `VerdictDetail`.
⛔ **RICHIAMO DEL 2026-09-01: il QUARTO e il QUINTO giro sono fatti — `E73`–`E78` e `E79`–`E82`,
tutti riverificati dal coordinatore prima di rimediare e tutti veri.** ✅ **E il quinto ha
introdotto la domanda che ha pagato:** invece di *«trova difetti»*, *«ciascun rimedio dell'ondata
precedente ha chiuso la CLASSE o l'occorrenza?»*, con risposta al `grep`. Tutti e quattro i suoi
rilievi sono venuti da lì, e il più caro è `E79`: il rimedio di `E73` aveva chiuso **un sito su
tre**, e gli altri due erano **mutanti vivi**.
✅ **E IL 2026-09-01 IL PROPRIETARIO HA SCELTO DI CHIUDERE ALLA RADICE — `AUD-050` È CHIUSO A
LIVELLO 1**, che era la terza delle tre vie che `E53` gli metteva davanti. `RecordV1` non ha più
nessun campo pubblico: sei accessori, e **un costruttore per specie** con `reason: &'static str`.
⛔ **Chiude DUE cose insieme:** la fuga di P-1 *«attraverso una seconda bocca»* — un letterale di
struct metteva testo di runtime all'indice 4, riprodotto da fuori la crate prima di toccare
niente — e la classe di `E73`/`E79`, perché la coppia `kind`/`Detail` non è più **rifiutata** ma
**impronunciabile**: `kind` non è parametro di niente. ⚠️ **Costo misurato e non citato dalla
scheda, che è di agosto — ⛔ e la CIFRA È TOLTA DA QUI IL 2026-09-01, non riallineata:** viveva in
sei case e due sue clausole erano false alla misura, sul precedente di **AUD-018** e **AUD-060**.
La casa unica è [`porta-di-qualita.md`](porta-di-qualita.md), dove vive **col comando**.
✅ **I byte congelati NON si sono mossi**, controllato per primo. Il verbale è la voce **E83** dell'errata; il caso nuovo è
il **35°** `compile_fail`, provato nelle due direzioni.
⛔ **E IL SESTO GIRO È FATTO IL 2026-09-01 SU `git show c63c8c8`, E NON È TORNATO PULITO: quattro
rilievi, `E84`–`E87`, tutti riverificati e tutti veri.** ✅ **La voce del proprietario è CHIUSA lo
stesso giorno: il compito 6 si apre comunque.** ⛔ **E il giro ha SMENTITO l'argomento con cui il
ciclo stava per chiudersi:** `E84` è un difetto di **PRODOTTO** e non di prosa — `E83` aveva
declassato un caso `compile_fail` dalla forma forte alla debole **senza nessun rosso**. 📌 *A
decidere quanto valga un giro è il **perimetro**, non il suo numero.* ⚠️ Casa unica: `E53`.
✅ **E IL PROPRIETARIO HA AUTORIZZATO I SOTTO-AGENTI IL 2026-08-31**, quindi la modalità che il
piano prescrive in testa — *un subagente fresco per compito, con revisione fra uno e l'altro* — è
di nuovo praticabile, e il compito 6 **non è più sbarrato dal metodo**.
✅ **IL COMPITO 6 È ESEGUITO IL 2026-09-01, `GATE GREEN`, in UN commit e non due** — il decisore del
gateway, il gettone di conformità e il record risolto. `Q13` è coperta con **entrambe** le metà, il
formato guadagna la specie `Routing` col **quinto** record congelato, e i quattro vecchi sono
**byte-identici**, controllato per primo. ⛔ **I due commit dettati NON erano ordinabili** — il
primo importa un tipo che nasce nel secondo: voce `E89`. ⛔ **E il pre-controllo ha tenuto una
QUINTA volta, `E88`:** il brief era scritto contro un `RecordV1` che `E83` aveva sigillato il giorno
prima, e il peggiore dei tre bloccanti avrebbe **riaperto AUD-050**. 📌 *Un pre-controllo ha una
data come tutto il resto.*
✅ **E LA VOCE `E94` CHE IL COMPITO AVEVA APERTO È CHIUSA IL 2026-09-01, per decisione del
proprietario:** `RoutingDetail` prende la forma di AUD-050 — campi privati, un costruttore che
prende `&'static str` — e i byte congelati **non si sono mossi**, controllato per primo. ⛔ **E la
scheda prezzava un sito in meno**, gotcha **#65** su una voce d'errata. Il racconto, le due
direzioni della prova e la ragione per cui `VerdictDetail` **non** è sigillata stanno in
[`porta-di-qualita.md`](porta-di-qualita.md), in una casa sola. ⛔ **Ne nascono DUE voci:** `E95`
detta al compito 7 di far nascere `PermissionDetail` già sigillato, ed `E96` è **del proprietario**
— un rimando `E<n>` nel sorgente non è un riferimento, perché il numero è unico dentro **un** piano.
⛔ **E IL COMPITO 6 ERA STATO CONSEGNATO SENZA REVISIONE, mentre il piano prescrive *«un subagente fresco per compito, con revisione fra uno e l'altro»*: la revisione è stata fatta il 2026-09-01 ed è lo STESSO perimetro del settimo giro di `E53`** — una passata sola invece di due. **Cinque rilievi, `E97`–`E101`, tutti riverificati dal coordinatore prima di rimediare e tutti veri.** ⛔ **E DUE SONO DI PRODOTTO, entrambi mutanti vivi sull'intero workspace:** `dispatch` scrive **sei** campi del record durevole e la sonda ne rileggeva **quattro**, quindi l'etichetta `Trust` — che è `I6` e ADR-0014 — poteva essere scritta **falsa** senza un solo rosso; e la via d'errore di `dispatch` non era raggiunta da niente, quindi un `dispatch` che **inghiottiva** il rifiuto del giornale restava verde. ✅ **Entrambi chiusi con le due direzioni provate**, e le mutazioni vivono in [`porta-di-qualita.md`](porta-di-qualita.md), in una casa sola. 📌 **E la regola che il sesto giro aveva enunciato ha retto:** *«si rivede finché una passata non torna pulita» è debole dopo una passata documentale e forte dopo un cambiamento di CODICE* — questo perimetro era codice nuovo, e i difetti di prodotto sono tornati. ⚠️ **Quanti giri e che cosa resti aperto NON sta qui: la casa unica è la voce `E53` dell'errata.**
⚠️ La voce `E50` dice che l'elenco dei file di un compito che tocca `RecordKind` deve nominare
**anche** `crates/simulator/tests/dst_campaign.rs`, che porta un oracolo con un `match` esaustivo.
⚠️ **Il compito 2 non
esiste**, ed è dichiarato nel piano: il timbro di build è **uscito dal perimetro**. Subagent-driven,
un subagente fresco per compito con revisione fra uno e l'altro, e **prima** il pre-controllo delle
quattro domande di [`../CLAUDE.md`](../CLAUDE.md), che ha trovato un difetto reale in **tutti** i
compiti dispacciati finora — **compreso questo**.
⛔ **RICHIAMO DEL 2026-08-28: qui e nel riquadro del Traguardo 5 si nominava `E30` come la voce
che «va decisa prima». È DECISA nel merito, e il nome è TOLTO da entrambe invece che
riallineato** — quale voce sbarri il traguardo è uno **stato**, e uno stato ha una casa sola.
✅ **I finding dell'audit sono chiusi**, e a dirlo non è questa riga ma il comando:

```
awk -F'|' 'NF>4{gsub(/^ +| +$/,"",$5); print $5}' docs/audit-2026-08-27.md | grep -c aperto
```

---

### Le voci ancora aperte, e il racconto che le circonda

⛔ **Questi blocchi sono TENUTI PAROLA PER PAROLA, e non riassunti.** Ognuno porta una voce
che **nessuno ha chiuso** — spesso una decisione che aspetta il proprietario — insieme al
racconto di come è nata.

⚠️ **ED È UN DEBITO DICHIARATO, non una svista.** Lo sfoltimento del 2026-08-28 poteva
**estrarre** ogni voce aperta in una tabella sola e archiviare il racconto: avrebbe portato
questa §6 da ~33 000 a ~4 000 token. ⛔ **Non è stato fatto, e la ragione è che quelle voci
sono DECISIONI DEL PROPRIETARIO:** riassumerle di iniziativa significa poterne travisare o
perdere una **in silenzio**, che è il fallimento contro cui esiste l'audit di questo
repository. 📌 **La consolidazione è una voce propria**, da presentare al proprietario una
per una — non un effetto collaterale di uno sfoltimento.

⚠️ **La cernita è MECCANICA e provata in due direzioni:** un blocco resta se nomina una voce
aperta; e ciò che stava per essere archiviato è stato ripassato con marcatori **diversi**,
tenendo tutto ciò che pescava. **Su un dubbio si conserva**, perché un archivio si riapre e
una decisione persa no.

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

✅ **IL TRAGUARDO 5 È ESEGUITO — il 2026-08-25, tredici compiti su tredici, `GATE GREEN` a
ciascuno**, subagent-driven, un compito per volta con revisione fra uno e l'altro, dal
2026-08-19. L'arbitro esiste per intero: il vocabolario della risorsa, l'ammissione, le code per
corsia, la revoca con la propria grazia, le due policy VRAM, il cablaggio di produzione, i
quattro casi negativi della porta `process` e la campagna DST dell'arbitro. Il racconto compito
per compito sta nel riquadro subito sotto.
⛔ **E il Task 13 era un AUDIT e non una scrittura, quindi ciò che ha trovato conta più di ciò
che ha aggiunto:** i riconteggi del Passo 1 e il *«dodici»* del Passo 2 **reggono senza uno
scarto** — rieseguiti, non citati — e a mancare era la **condizione 8**, la sola che il piano
aggiungeva alle sette del disegno: le voci aperte in **una** tabella sola. ✅ Raccolte in
[`porta-di-qualita.md`](porta-di-qualita.md), sezione *«Le voci aperte del Traguardo 5, in una
tabella sola»*, **con la colonna di chi le chiude** — e per alcune di esse il chiusore **non** è
il proprietario, che è la notizia portata da quella colonna. ⚠️ **Quali, lo dice il comando che
vive in quella sezione**, in una casa sola.
⚠️ **RICHIAMO DEL 2026-08-25 — questa riga diceva *«VIVEVANO in sei riquadri sparsi del
registro»***, al passato, e i sei riquadri erano **ancora lì immutati**: `git diff --stat` sul
registro dava `+103/−0`, cioè la tabella si era **affiancata** e non sostituita. ⛔ **Il rimedio
è che i sei riquadri ORA NOMINANO la tabella**, uno per uno — prima il rimando esisteva solo
nella direzione tabella → riquadro, e chi arrivava al riquadro vedeva una voce aperta isolata,
non sapeva che esistesse un indice, e chiudendola lì lasciava viva e falsa la riga
corrispondente. La lettera della condizione **8** era soddisfatta — **una** tabella — e lo scopo
no.
⚠️ **RICHIAMO DEL 2026-08-30 — QUESTA RIGA PORTAVA IL MARCATORE DEL PROSSIMO PASSO, ed è
corretta invece che cancellata**, sul precedente identico del riquadro del Traguardo 4: era il
puntatore **di allora**, dentro il racconto della chiusura del Traguardo 5, e nessuno l'ha
spento quando quel racconto ha smesso di essere in corso. **Il puntatore vivo sta nella *«Il
prossimo passo»* di questa §6, in un posto solo** — e a trovare la seconda casa è stato il
censimento col `grep`, non la memoria. ⚠️ **E il marcatore è nominato QUI A PAROLE per forza:**
scritto com'è, questo capoverso diventerebbe esso stesso una casa del `grep` che lo censisce, ed
è la nota che il registro porta già sul censimento delle voci aperte. ⛔ **E diceva anche *«si riparte dal BRAINSTORMING»***: il brainstorming del Traguardo
6 è **chiuso**, il disegno **completo** e il piano **finito**. Ciò che resta vero della riga è
che **prima** si legge ciò che il Traguardo 5 lascia aperto — la tabella *«Le voci aperte del
Traguardo 5»* di [`porta-di-qualita.md`](porta-di-qualita.md), che ne è la casa unica.
⛔ **Il sotto-progetto 1 NON è chiuso:** restano il Traguardo 6 e la §8 di
[`tracciabilita.md`](tracciabilita.md), che si aggiorna alla chiusura del sotto-progetto e non
di un traguardo.
⚠️ **Il numeratore lo muove chi esegue, e vive QUI e in nessun altro punto di questa
sezione** — ne teneva una seconda copia quattro riquadri più sotto, che è il modo in cui una
copia marcisce senza che nessuna delle due sembri sbagliata (gotcha **#68**).
⛔ **RICHIAMO DEL 2026-08-20 — questa riga elencava i valori che il numeratore aveva già detto**
(*«quattro»*, *«cinque»*, *«sei»*, *«sette»*) **ed è TOLTA, non estesa a otto:** un elenco che si
allunga a ogni compito **è** un numeratore, e invecchia peggio di uno in cifre perché non c'è
nessun numero da riconoscere come stantio. È la stessa correzione che la 45ª misura applicò a
[`README.md`](README.md), qui applicata alla riga che il numeratore lo **tiene**.
⛔ **E IL 2026-08-21 UNA PASSATA DI AUDIT SU CODICE E DOCUMENTI HA TROVATO NOVE DIFETTI, E LA
CLASSE NUOVA È UNA SOLA: UNA SCADENZA SCRITTA IN PROSA NON HA NIENTE CHE LA FACCIA SCATTARE.**
È il gotcha **#77**, e l'ha data **due volte lo stesso file**: `reconcile.rs` dichiarava
*«chi scrive è UNA FUNZIONE sola»* con l'innesco *«l'aiutante nasce col SECONDO scrittore»*, e
il secondo è arrivato col **Task 9** senza che nulla diventasse rosso; `ports/journal.rs` e
`ports/ipc.rs` promettevano l'allocatore di `StepId` *«col Traguardo 3»*, chiuso da undici
giorni senza. ⛔ **Il contrasto è la parte utile:** le scadenze `E10`, `E67` e `E74` hanno
retto **due volte su due** perché erano tenute da un avviso `dead_code` — **il compilatore le
ricorda**. Una scadenza in prosa non ha nessun meccanismo dietro.
⚠️ **I nove si contano così, ricontati e non asseriti — la prima stesura di questo riquadro ne
dava *«otto»* alla radice R1 e ne elencava quattro:** **due** sono il #77 (l'aiutante e
l'allocatore); **quattro** sono la radice **R1**, cioè una correzione che non attraversa gli altri
documenti — la **tabella dei pesi della §12**, ferma alla **42ª misura** per cinque passate mentre
i verbali dalla 43ª alla 47ª portavano i numeri giusti; **tre** conteggi di test nel registro
fermi al 2026-08-18; *«otto promesse e otto bugiardi»* in **tre** sorgenti vivi dove sono **nove e
dodici**; e la mappa dei documenti di [`HANDOFF.md`](HANDOFF.md), che enumerava *«quattro piani»*
dove sono **sei**. ⛔ **Gli scarti NON sono ripetuti qui**: vivono nella **48ª misura** della §12,
in un posto solo — scriverli anche in questa sezione sarebbe il gotcha **#68** commesso dentro il
riquadro che lo riporta. Gli ultimi **tre** sono il `fmt` rosso qui sotto e due coerenze minori:
`MemoryJournal` senza la nota sull'avviso clippy che i suoi due fratelli portano, e un `assert!`
di esecuzione su un operando `const`.
✅ **E un difetto che il documento stesso prevedeva:** `cargo fmt --all --check` era **rosso**,
**ventisette hunk in sei file**, mentre [`porta-di-qualita.md`](porta-di-qualita.md) diceva
*«oggi è verde su tutto il workspace»* — il segnale che *«nessuno fa rispettare»*, non imposto
per decisione della §7.4.3, ha smesso di essere vero dove si lavorava. Corretto nel file, come
quel paragrafo prescrive.
⛔ **E correggerlo ha fatto scattare la trappola dei fine-riga di `CLAUDE.md`, misurata invece
che temuta:** `cargo fmt` ha normalizzato `ports_are_implementable.rs` da **972 `CR` a zero**
— uno dei **quattro** file con `CR` **nell'indice** — e `git diff` dichiarava **1944 righe
cambiate** su due hunk veri. Ripristinato da una **copia byte-esatta presa prima**, mai da
`git checkout --` (gotcha **#48**, dodicesima forma): il censimento `git ls-files --eol` è
**immutato**. 📌 La riga di `CLAUDE.md` nomina `sed -i`; qui il colpevole era `cargo fmt`.
✅ **Baseline invariata a passata chiusa:** `GATE GREEN`, **35 target, 248 passate, 0 fallite,
2 ignorate**, `cargo build --locked --workspace` a **zero avvisi**, `cargo fmt --all --check`
**pulito**, `check-docs.sh` **verde**.
⚠️ **UNA VOCE APERTA REGISTRATA E NON PRESA, ed è del proprietario:** se le **due** funzioni
che scrivono un record debbano condividere un **aiutante** che tenga in passo il `kind` e
l'operazione. Tocca la forma di codice con due siti di chiamata; intanto ciascuna ha la
**propria sonda**, e le sonde sono nominate in `crates/kernel/src/reconcile.rs`.
⚠️ **E una SECONDA, la stessa da sei passate:** questa passata **non ha toccato**
[`riferimenti.md`](riferimenti.md), deliberatamente. Le sue misure vivono nel registro,
accanto al controllo che difendono.

✅ **E IL 2026-08-21 È CHIUSO IL FINDING P-2 DELL'AUDIT, prima di dispacciare il Task 11 e per
decisione del proprietario.** Era l'**unica** voce della §5.3 rimasta senza marca — P-1, K-1 e
PL-1 furono chiuse il 2026-08-18 — e non era stata rimandata con un argomento: nessuno l'aveva
guardata. A ritrovarla è stato il **pre-controllo del Task 11**, perché P-2 **è la premessa di
quel compito**.
⛔ **Cosa diceva, e perché conta più di una frase sbagliata.** Le quattro righe di §6.10.5 erano
scaglionate dal Traguardo 2 con la ragione *«tutte e quattro pretendono di ottenere un `Worker`;
un `Worker` lo restituisce solo `start(grant, ..)`; e nessuno emette concessioni prima del
Traguardo 5»*. ✅ **Misurato su una sonda usa-e-getta scritta da FUORI la crate, compilata,
passata e cancellata nella stessa corsa** — non dedotto, e non ripreso dal rapporto (gotcha
**#65**): `impl Worker for W` **senza nominare `Grant`** compila, quindi un `Worker` viene
dall'**implementare il tratto**, e `crates/kernel/tests/ports_are_implementable.rs` lo fa dal
Traguardo 2 con `ScriptedWorker`.
⛔ **E la metà che il rapporto chiamava «giusta» è SCADUTA, il che è il gotcha #77 su un
rapporto d'audit.** *«`Grant` non ha costruttore pubblico, quindi `Process::start` non è davvero
chiamabile»* era vero l'11 agosto; dal **Task 5** `Admission::Granted(Grant)` è **pubblica**, e
la stessa sonda ha consegnato una concessione vera a un `start` scritto da fuori: **compila e
passa**. Nessuno se n'era accorto perché una condizione in prosa non fa diventare rosso niente.
⛔ **Ciò che si chiude è la RAGIONE, non la copertura**, ed è la forma già usata per il #51: le
quattro righe restavano scoperte per la **sola** ragione vera — mancava la direzione *«deve
scattare»* — e le ha chiuse il **Task 11**. ⚠️ **La misura sui chiamanti di `Process::start` non
è più ripetuta qui:** era *«zero in tutto il workspace»*, vera **prima** del Task 11 e falsa
adesso, perché quel compito ne ha scritti. Vive in [`porta-di-qualita.md`](porta-di-qualita.md),
sezione «P-2», col proprio qualificatore — e un rimando non può marcire.
✅ **Sei case vive corrette, tre verbali non toccati**, censite col `grep` e guardate in faccia
una per una (#70): il doc di modulo di `crates/kernel/src/ports/process.rs`, tre punti di
`crates/kernel/tests/ports_are_implementable.rs` — fra cui il **nome** della sonda, che
affermava il falso ed è ora `the_process_port_is_implementable` (precedente **E40**) — e due
righe del registro. I due piani e il disegno del Traguardo 5 sono **verbali** e restano come
sono; la ragione falsa che il piano **detta nel codice del Task 11** è la voce d'errata `E130`,
scritta prima di dispacciarlo.
⛔ **E una forma NUOVA del #70, che il censimento sul nome intero non trovava:** la riga `M10`
della tabella delle mutazioni nominava la sonda **in forma abbreviata**,
`..._start_is_not_callable`. 📌 *Un rinomino si censisce sul frammento più corto che resti
unico, non sul nome intero.*
⚠️ **E una divergenza dal pre-controllo, registrata perché il proprietario la veda:** il ledger
dava *«la chiusura di P-2 tocca §7.4, che è SPEC»*, quindi fuori dal vincolo globale 7.
**Misurato: non la tocca** — nessuna riga di catalogo è aggiunta, tolta o riformulata. 📌 Il
**#65** vale anche per un **pre-controllo**: si prezza leggendo il codice, non il verbale.
Il racconto per esteso, con le tre misure, sta in
[`porta-di-qualita.md`](porta-di-qualita.md), sezione «P-2».

| | Cosa ha portato | Commit |
|---|---|---|
| **Task 1** | il modulo `arbiter` nasce e `Mib` **non è un intero nudo** — quattro casi `compile_fail` (le due direzioni più le due vie `From`) e tre sonde sull'aritmetica, dove la **direzione** della saturazione è l'asserzione | `dc6ac4c` |
| **Task 2** | `ComputeClass` a tre corsie con `Ord` **scritto a mano da una chiave esplicita** — un `Ord` derivato segue l'ordine di dichiarazione, quindi riordinare le varianti rovescerebbe le priorità **senza un rosso** — e `Preemption::{Never, After(Millis)}`, che fa sparire **due** stati illegali insieme | `2fab856` |
| **Task 3** | `ResourceProfile` e `WorkDescriptor`, con **`cold_start` fuori dall'ammissione**: una decisione che volesse leggerlo **non ha una strada** | `89e6632` |
| **Task 4** | `Grant` **si sposta** da chi lo consuma a chi lo emette, `Admission` a **tre vie** e `Activity` **annidata** — la revoca di una concessione non prelazionabile non è *vietata*, è **impronunciabile** | `3c5df88` · `b91186d` |
| **Task 5** | **l'arbitro che ammette e rilascia**: `Parameters` guadagna `total_vram` — **consegnato e non chiesto**, ed è l'unico dei tre addendi della §5.1 a esserlo — e `Arbiter` nasce con `admit`, `release` che **consuma** la concessione, e `allocated`. ⛔ **La scadenza E10 è SCATTATA E RISPETTATA:** il warning che il Task 4 aveva lasciato viaggiare di proposito è sparito perché `release` legge `Grant::id`, cioè il campo serviva davvero — `cargo build --locked --workspace` a **zero warning**, verificato | `d662644` · `681798e` · `70e0a22` |
| **Task 6** | **le code, e sono per corsia**: `Admission::Queued` guadagna un **produttore**, `promote` serve **prima la corsia migliore** e dentro la corsia **per arrivo**, e la costruzione di una concessione si estrae in `issue` — l'**unico** posto della crate che ne costruisce una, che è ciò che la §5.6 esiste per tenere. ⛔ **E la revisione ha trovato un mutante vivo nell'INTERO workspace:** `promote` che non serviva mai la corsia `Realtime` lasciava **34 target su 34 verdi**, perché nessuna sonda promuoveva un'attesa in quella corsia — *«prima la corsia migliore»* era provato solo sulla **seconda** migliore. Il codice era giusto; a mancare era la **prova**. Gotcha nuovo **#74** | `47941dd` · `b297911` · `c919cee` |
| **Task 7** | **la revoca, e la grazia che scade**: `Arbiter::revoking()`, `ask_back` che **marca e non prende** — la riserva resta nei libri per tutta la grazia — e `collect_expired` che diventa una riscossione a **due scadenze**, la finestra dichiarata dal richiedente e la grazia che una revoca ha dato. `Held` guadagna **tre** campi (`lane`, `activity`, `grace`) e ne ha **cinque**. ⛔ **Tre decisioni del proprietario prese eseguendo:** `ask_back` resta **`pub(crate)`** e le sue sonde vivono in un `#[cfg(test)] mod tests` **dentro `crates/kernel/src/arbiter/mod.rs`** — il primo di `kernel`, sul precedente di `crates/platform/src/rng.rs` — perché il doc del tipo dice che *«fare spazio è la conseguenza di una richiesta, mai una cosa che qualcuno chiede»*; i **due avvisi `dead_code`** sono **ratificati** con scadenza al Task 8, **nessun `#[allow]`**, avviso **visibile**, sul precedente di `E10`; e `ask_back` **non marca vittime quando il recuperabile non copre il bisogno**, riclassificato da politica aperta a **difetto** — sfrattare un lavoro per una richiesta che non si siede comunque è il degrado silenzioso vietato da ADR-0005 e ADR-0019 | `1b5af8d` · `7b3882e` · `83c7242` · `701987c` |
| **Task 8** | **le due policy VRAM, e la decisione sta dentro l'ammissione**: il tratto `MakeRoom` con **una domanda sola** — *«una richiesta non entra: si può fare spazio?»* — `RemotePolicy` e `LocalPolicy` come **due oggetti**, e `VramPolicy` che ne tiene **uno alla volta**, che è `V3` al **livello 1**: *«due policy attive»* non è pronunciabile. Il ramo d'ammissione chiede la domanda in **un posto solo**; l'accodamento è **estratto** in `enqueue`, a comportamento provato invariato; e il modulo `arbiter` è ora la **cartella con tre file** che la struttura del piano prevedeva. ⛔ **E ciò che rende il compito possibile solo a questo punto del traguardo:** *«sfrattare un residente»* **è** *«revocare una concessione prelazionabile»*, cioè il meccanismo che il Task 7 ha costruito — quindi le due policy non sono gusci vuoti e non serve **nessun modello** per distinguerle (ADR-0020). Era la domanda che il disegno aveva lasciato aperta | `4b89fea` · `ea0cc09` · `a53907e` · `b49c835` |
| **Task 9** | **la transizione di policy è un passo giornalato, e l'intento viene prima dell'effetto**: `Arbiter::set_policy` scrive l'**intento**, poi assegna, poi scrive l'**esito**, con la funzione libera privata `transition_record(kind, policy)` accanto all'`impl`. `EffectClass::Idempotent` è **argomentata e non scelta**; `Trust::Instruction` porta il **payload vuoto**; e `reason` porta il **nome della policy**, che è la ragione per cui `MakeRoom::name` esiste. **Cinque** sonde nuove in `crates/kernel/tests/arbiter_policy.rs`, che passa da **sette a dodici** test. ⛔ **E la scelta che le rende non vacue:** le asserzioni stanno sull'**archivio**, non sulla policy — *«dopo la transizione la policy è l'altra»* è **verde con zero record scritti**, quindi da sola non prova niente. È la **proprietà DST numero 4**, e rende scrivibile la campagna del **Task 12**. ⛔ **E un fatto di livello 1 che nessuno aveva rivendicato:** l'ordine *intento prima dell'effetto* è in parte tenuto dal **COMPILATORE** — `VramPolicy` non deriva `Copy` né `Clone`, quindi assegnare `self.policy` prima di scrivere l'intento **muove** il valore e `policy.name()` sotto è `` error[E0382] ``: la mutazione dettata dal piano **non compila**. Voce `E115` ② | `4c07d26` · `036a93a` |
| **Task 10** | **il grafo di produzione monta l'arbitro, il giornale e le due concessioni permanenti**, ed è il primo chiamante di produzione che l'arbitro abbia mai avuto. `run_the_production_graph` prende un percorso, `StartupError` ha **tre** varianti e `daemon` passa da **una** sonda a **otto**. ⛔ **`E41` è chiusa DAVVERO, e la differenza è il punto:** il piano la chiudeva con un'asserzione dentro un test, e una quota permanente che non è `Granted` è ora un **errore d'avvio** che **nomina la quota caduta** — con **due** sonde permanenti e non una mutazione, perché le vie sono due e falliscono diversamente (`Queued` a `1_500` MiB, `Refused` a `500`). Una direzione tenuta da una mutazione è tenuta da niente (gotcha **#72**). ⛔ **E i due campi delle riserve erano QUATTRO mutanti vivi** — `Preemption::Never` è l'unico sito di produzione che **sceglie** la parola di ADR-0033, `ComputeClass::Realtime` è la premessa della frase accanto a `build_the_arbiter`, e nessuna sonda li teneva: pinzati, non dichiarati, perché la decisione l'ha presa un ADR e il doc la **afferma** (confine **#73**/**#14**). ⛔ **E `FOR_EVER` non è letteralmente «mai»:** `saturating_add` satura **a** `u64::MAX` e `collect_expired` confronta `<=`, quindi all'ultimo millisecondo rappresentabile **entrambe** le quote vengono riscosse — `allocated()` torna `Mib(0)` invece di `Mib(1792)`, misurato. Il confine è pinzato nelle **due** direzioni | `800ffeb` · `9c91e18` · `280e491` · `7fa8a37` · `a20158d` · `57e7b32` |
| **Task 11** | **le quattro righe di §6.10.5 escono dallo scaglionamento**: i quattro casi `compile_fail` della porta `process` — parlare senza la maniglia (`E0599`), istruire dopo l'uccisione (`E0382`), leggere senza ricevuta (`E0061`), leggere due volte la stessa ricevuta (`E0382`) — con le contro-sonde in `crates/kernel/tests/worker_tokens.rs`, che un `Grant` vero se lo fanno dare da `Arbiter::admit` e **mai** da un costruttore di test. Blocco B a **quattro su cinque**, blocco C a **diciassette su diciannove**. ⛔ **E la ragione per cui erano ferme era FALSA dal Traguardo 2**, non dal Traguardo 5: il finding **P-2** dell'audit, chiuso il giorno prima di dispacciare il compito — un `Worker` si ottiene **implementando il tratto**, non da `start(grant, ..)`, e `ScriptedWorker` lo faceva da tre traguardi. Il racconto vive in [`porta-di-qualita.md`](porta-di-qualita.md), sezione «P-2», in una casa sola; da lì il gotcha **#79**. ⛔ **E il compito è costato più ondate di correzioni di ogni altro di questo traguardo, e quante lo dice `git log`**: la nona ha trovato che `E139` — scritta dall'ottava per chiudere un rilievo — quantificava a **sei** righe una deriva che per i tre puntatori che nomina è di **diciannove**, o non misurabile affatto. ⛔ **E ha trovato un difetto che non era del compito: il cancello leggeva `.md` che git IGNORA**, quindi il suo verdetto dipendeva dalla cartella di lavoro invece che da ciò che si consegna — rosso per i file di lavoro di chi lo eseguiva, e verde quando quei file semplicemente non ci sono. ✅ Chiuso in `scripts/check-docs.sh` con la regola **generale** — non legge ciò che git ignora — e **non** con l'esclusione di una cartella per nome, che avrebbe lasciato aperti `/scratch/` e `/tmp/`, dove `CLAUDE.md` manda ogni misura; e **non** con `git ls-files`, che avrebbe fatto sfuggire un documento nuovo non ancora aggiunto **proprio nella corsa che conta**, perché il cancello gira prima del commit | `5fceee1`, il prodotto; le ondate che seguono le conta `git log` |
| **Task 12** | **la campagna DST dell'arbitro, e l'arbitro sotto prova è quello VERO, mosso da attività dentro l'esecutore** — non c'è una finta, perché l'arbitro è **logica e non una porta**, e ciò che si inietta viene da fuori: l'interlacciamento dal seme, il passare del tempo dall'orologio virtuale, la caduta dal giornale. `crates/simulator/tests/arbiter_campaign.rs`, **cinque** sonde — l'oracolo che l'ammissione decida davvero, l'oracolo di non-vacuità sui mondi, e tre delle cinque proprietà di §5.7, con quella della scadenza tenuta da **due testimoni distinti** e non da una soglia aritmetica. ⛔ **E IL COMPITO COSÌ COM'ERA SCRITTO NON POTEVA PASSARE:** lo scenario dettato faceva fallire l'oracolo che il compito **stesso** detta — **un** esito distinto su ventimila semi, cioè *«una corsa ripetuta»*. Le due cause, lette nel codice: attività **identiche in tutto ciò che i libri vedono**, e `now` preso dall'**indice del ciclo** invece che dall'**orologio virtuale**. ✅ **Riscritto dentro il compito, decisione del proprietario**, col rosso di partenza **riprodotto prima** del rimedio e il verde misurato dopo — voce `E144`, e da qui il gotcha nuovo **#81**. ⛔ **E la campagna di mutazione ha lasciato UN MUTANTE VIVO su codice di produzione**, `M9`: tolta ad `Arbiter::release` la riscossione delle scadute, **niente in tutto il workspace diventa rosso**, perché ogni turno di ogni parte passa da `admit`, che riscuote per primo. **Dichiarato e non pinzato** (gotcha **#73**) — pinzarlo congelerebbe la scelta che `E30` mette davanti al proprietario — e le quattro revisioni hanno verificato che la rinuncia regge. Voce `E151` | `c94784b`, il prodotto; le ondate e la riformattazione che seguono le conta `git log` |
| **Task 13** | **la chiusura, che è un AUDIT e non una scrittura, e gran parte era GIÀ ESEGUITA — gotcha #49 alla terza occasione, riconosciuto invece che rieseguito.** I riconteggi del Passo 1 sono stati **rifatti col comando** e non citati, e non hanno prodotto **nessuno** scarto con ciò che i documenti già scrivevano; il *«dodici»* del Passo 2 è stato ricontato **sulla §7.4 della spec** delimitando per intestazione — `3 + 8 + 1` — e **regge**. ⛔ **A mancare era la sola condizione che il piano aggiungeva alle sette del disegno, la 8:** le voci aperte in **una** tabella sola, mentre vivevano in **sei riquadri** del registro. ✅ Raccolte in [`porta-di-qualita.md`](porta-di-qualita.md) **con la colonna di chi le chiude**, ed è la colonna che ha portato la notizia: per alcune di esse il chiusore **non** è il proprietario — **quali, lo dice il comando** che vive in quella sezione, e la cifra con l'elenco è uscita di qui il 2026-08-28. Sparse fra i riquadri si leggevano tutte come *«aspetta il proprietario»*. ⛔ **E il censimento ha dato il #70 in ENTRAMBE le forme, misurato mentre lo si scriveva:** il filtro sull'errata restituisce candidate che sono **già chiuse**, e **manca** voci aperte vere, trovate leggendo la §6; e l'intestazione della sezione nuova è essa stessa una **casa** del `grep` che censisce. ⛔ **RICHIAMO DEL 2026-08-25 — qui c'erano DUE CIFRE, *«venticinque»* e *«dodici»*, che il comando NON restituisce**, ed erano la seconda e la terza casa di un valore il cui unico posto sensato è il blocco di comandi che lo produce. **Tolte e non riallineate:** quel blocco è stato **rilanciato**, corretto e datato in [`porta-di-qualita.md`](porta-di-qualita.md), e qui resta il rimando | questo commit |

⛔ **L'errata del piano cresce, e il pre-controllo ha trovato un difetto in dodici compiti su
dodici.** ⚠️ **Quante voci abbia lo dice il piano, non questa riga:** il totale è **tolto** e
non riallineato per la seconda volta — diceva *«centotrenta»*
quando erano **141**, e un cumulativo che due sorgenti diverse fanno crescere invecchia più in
fretta di quanto qualcuno lo riconti (gotcha **#31**). Il **numeratore dei compiti** invece
resta, perché §6 è la sua **unica** casa e `CLAUDE.md` vi delega. ⚠️ **E non tutte le voci
vengono da un compito:** `E130` nasce chiudendo il finding **P-2** prima di dispacciare il
Task 11, ed `E141` è un difetto del **coordinatore** trovato dall'ondata a cui era stato dato
l'oracolo sbagliato. ⛔ **E la
notizia del Task 10 è QUALI:** **due** delle sue voci d'errata sono difetti del **coordinatore** e non del
piano — `E127`, il divieto di toccare `crates/kernel/src/arbiter/` che copriva anche tre frasi
rese false lo stesso giorno, ed `E128`, un'**esclusività falsa dettata in un brief** e copiata nel
sorgente. ⚠️ **In entrambi i casi chi eseguiva si è FERMATO e le ha riportate** invece di
aggirarle, ed è la condotta che le ha rese visibili. 📌 *Un brief è un'affermazione come le
altre* — gotcha **#65** applicato a chi coordina.
⚠️ **SEI al Task 9**, di cui **due bloccanti per costruzione**: **tutti** gli import mancavano — `String`,
`Record`, `RecordKind`, `Journal`, `StepId`, `JournalError` — e **nessuna sonda leggeva
`reason`**, cioè il campo che il doc del compito dichiara essere la ragione per cui
`MakeRoom::name` esiste. ⛔ **E una delle sei è un difetto del COORDINATORE e non del piano**,
`E116`: l'oracolo sui fine-riga che il brief pretendeva **non poteva essere verde**. Il racconto
sta nella §9, dentro il gotcha **#31**.
OTTO al Task 8, di cui **due bloccanti** allo stesso modo: il codice dettato chiamava un
`enqueue` che **non esiste**, e una sonda dettata non compilava perché `name()` vive solo sul
tratto e `MakeRoom` non era nella lista `use`. ⚠️ **E una cifra del pre-controllo stesso era sbagliata** —
diceva *«cinque siti in quattro file»*, i file sono **cinque**: `E101`, e la misura ha ora una
casa sola. Il Task 7 ne aveva prodotti NOVE,
fra cui uno che lo rendeva **incompilabile per costruzione**: il Passo 1 metteva in un banco
d'integrazione — che è una crate a sé — sonde che chiamano un `pub(crate)`, cioè
`` error[E0624] ``, misurato prima di dispacciare. Nessuna riapre una decisione: cadono
comandi di verifica che **non potevano fallire**, un import, una citazione sbagliata, due
mutazioni dettate che non rovesciavano il proprio caso, **un conteggio di chiamanti sbagliato in
entrambi i termini** (il piano dice *«venti siti in sei file»*, sono **diciannove in quattro**,
più un **oracolo** che nessuno aveva contato perché non è un chiamante), e — col Task 6 — un
**corpo dettato** che è stato disatteso sul merito (`E45`: `promote` cammina le corsie con
l'iterazione del `BTreeMap` invece che con un array esplicito, perché l'array direbbe l'ordine
una **seconda** volta, che è la trappola per cui `ComputeClass` non deriva `Ord`).
⛔ **E il Task 5 ha prodotto TRE decisioni di disegno REGISTRATE E NON PRESE**, che aspettano il
proprietario e non bloccano l'esecuzione in corso: **E30** — `release` risponde `UnknownGrant` anche a una
concessione **propria ma scaduta**, perché riscuote prima di cercare, e il nome della variante
afferma allora il falso; ⚠️ **va decisa prima del Traguardo 6**, dove `Worker::kill` restituisce
la concessione a lavoro **finito**, che può benissimo essere dopo la finestra. ⛔ **E il Task 7
l'ha ALLARGATA, riscrivendola nel proprio posto invece di affiancarla:** con `collect_expired` a
**due** scadenze le cause di `UnknownGrant` sono **tre** e non due — la terza è una concessione
**chiesta indietro** la cui **grazia** è scaduta — quindi `ReleaseError::Expired` da sola non le
separa più, e anche la **forma** del rimedio è parte di ciò che il proprietario deve scegliere.
✅ **Misurata e non dedotta:** chiesta indietro a `0` con grazia `500` e rilasciata a `500` esatti
→ `Err(UnknownGrant)`, la stessa a `499` → `Ok(Mib(4096))`. **E31** —
`saturating_add` può sovra-ammettere al limite superiore, misurato: con `ceiling = u64::MAX` un
secondo `admit` da 1 MiB torna `Granted`. **E32** — `parameters` e `arbiter` sono ora mutuamente
dipendenti.

⛔ **E DUE DECISIONI NUOVE ASPETTANO IL PROPRIETARIO — `E50` ed `E51`.** ⚠️ **RICHIAMO DEL 2026-08-21:** diceva *«chiusore il Task 10»*, e il Task 10 si è chiuso **senza toccarle**: la radice di composizione **non orchestra**. Il chiusore è chi costruirà il **primo ciclo di orchestrazione** — quello che decide quando chiamare `promote` rispetto ad `admit` — e oggi non ne esiste nessuno.
Nessuna è un difetto oggi, nessuna blocca l'esecuzione in corso, ed **entrambe sono dichiarate non tenute nel
sorgente** invece che pinzate da una sonda. **E50** — fra corsie `promote` **scavalca**, cioè fa
esattamente ciò che il suo stesso commento rifiuta *dentro* una corsia, con un'inversione di
priorità sopra: misurato, un'attesa `Realtime` da 4096 accodata **prima** di una `Batch` da 1024
vede promuovere la `Batch`. **E51** — `admit` non consulta **mai** la coda, quindi un ritardatario
la scavalca; e la formula *«l'ordine per corsia è ciò che tiene validi i numeri di M-7»* è vera di
`promote` e **non** dell'arbitro, perché l'ordine d'**ammissione** la sconfessa.
📌 **Perché dichiarate e non pinzate, ed è il gotcha #73 alla seconda applicazione:** una sonda
che congelasse la caduta fra corsie congelerebbe **la scelta stessa** che le due voci mettono
davanti al proprietario, e *«una sonda che va cancellata per prendere una decisione è un voto
contro il prenderla»*. ⛔ **Il costo è scritto accanto ai due paragrafi:** sono **mutanti vivi**
— rimisurati due volte **al Task 6**, `34 target · 222 passati` sotto ciascuno, e la cifra resta
quella di allora perché è l'istantanea di allora — e i paragrafi diventano **falsi in silenzio**
il giorno in cui il Task 10 cambiasse l'ordine. ⚠️ **Il Task 7 non lo ha cambiato:** `ask_back`
percorre le corsie dalla **peggiore**, che è un'altra operazione e un'altra regola.
⛔ **RICHIAMO DEL 2026-08-20 — IL TASK 8 NON HA CAMBIATO L'ORDINE MA HA CAMBIATO LA SPECIE DI
`E51`: da TEORICA a RAGGIUNGIBILE IN PRODUZIONE**, e la frase si riscrive invece di ricevere una
riga sotto. Fino al Task 7 `ask_back` non aveva chiamanti di produzione, quindi **nessuna revoca
avveniva mai** fuori da una sonda e la stanza che una revoca libera **non esisteva**: non c'era
niente da scavalcare. Sotto `VramPolicy::Local` c'è — si chiede indietro un residente **per** un
biglietto in coda, la spazzata libera quella riserva alla scadenza della grazia, e il primo
`admit` diretto si siede sulla stanza fatta per un altro. ✅ **Misurato nelle due direzioni** su
una sonda usa-e-getta cancellata subito dopo: **LOCAL** → il ritardatario è `Granted`, `promote`
torna vuoto e il biglietto resta in coda; **REMOTE**, che è il mondo prima del Task 8 → lo stesso
ritardatario è `Queued`. ⚖️ **Il chiusore è chi costruirà il primo ciclo di orchestrazione**, ed
è ancora una decisione di orchestrazione: ciò che cambia è che il costo di lasciarla aperta si
paga ora **in produzione** e non sulla carta. ⚠️ **RICHIAMO DEL 2026-08-21:** diceva *«resta il
Task 10»*. Voce `E100`.

⛔ **E CIÒ CHE IL TASK 6 LASCIAVA AL TASK 7 È STATO MISURATO, e il riquadro si riscrive invece
di appenderci sotto una riga vera** (finding **A-2**). ① Diceva che `Held` ha **due** campi e che
il Task 7 sarebbe il primo lettore di **entrambi** quelli che mancano: ora `Held` ne ha **cinque**
— il Task 7 ne ha portati **tre** — e i lettori sono quelli previsti più uno che il disegno non
elencava. `ask_back` legge `lane` per scegliere la vittima e `activity` per rifiutare chi è già in
uscita; `revoking()` legge `activity`; e il **terzo campo è `grace`**, che serve perché
`Preemption` vive nel **profilo** e il profilo non si conserva — senza di esso la scadenza di una
revoca non ha un addendo. ② `E30` **va decisa prima del Traguardo 6**, e il Task 7 l'ha resa
**più larga** invece di chiuderla. ③ La baseline che il piano cita nei propri passi — *«32
target, 194 passati»* — è quella di **prima del Task 1**, e non si sposta: ogni compito misura
la propria.

⛔ **E DUE VOCI NUOVE SONO REGISTRATE E NON PRESE, nessuna delle quali è un difetto oggi.**
**`E70`** — dentro una corsia la vittima è la **più vecchia**, e nessun ADR lo decide: è un
**mutante vivo** rimisurato due volte, dichiarato **accanto alla frase** che compra, come `E50` ed
`E51` e per la stessa ragione (gotcha **#73**). ⛔ **E il `filter` sull'ammissibilità nella catena
che costruisce `lanes` è un caso DIVERSO, e la distinzione è il punto:** è a **comportamento
nullo** — toglierlo lascia `34 target, 236 passati`, identico alla baseline dello stesso giorno —
quindi è un **mutante vivo garantito** e non una politica aperta: nessuna decisione del
proprietario lo chiuderà, perché non c'è niente da decidere. Resta perché è ciò che rende vera
**per costruzione** la frase che i due insiemi sono lo stesso insieme, e la sua non-difendibilità
è scritta accanto alla frase con la misura. ⚠️ **E le sonde permanenti del compito non hanno una
riga di catalogo:** la §7.4 è **spec** (vincolo globale 7), quindi si **registra** e non si
prende — stesso trattamento di `PL-1` e di `K-1`/`B-1`, stessa ragione (gotcha **#36**).

✅ **E LA SCADENZA `E67`/`E74` È SCATTATA ED È STATA ONORATA AL TASK 8 — è la parte falsificabile
del Task 7, e queste righe sono MISURE e non più previsioni.** I **due avvisi `dead_code`** che il
Task 7 aveva lasciato viaggiare di proposito — `` fields `lane` and `grace` are never read `` e
`` method `ask_back` is never used ``, **nessun `#[allow]`** — sono spariti **da soli**: `admit`
sotto `LocalPolicy` è diventato il chiamante di produzione di `ask_back`, e `ask_back` legge
`lane` e `grace`. ✅ `cargo build --locked --workspace` → **zero avvisi**, erano due.
⛔ **Il precedente di `E10` — campo tenuto, avviso visibile, nessun `#[allow]`, scadenza scritta
accanto al codice — ha ora retto DUE volte su DUE**: è la sola forma di debito che questo
traguardo si concede, e si ripaga da sé perché il compilatore la ricorda.
⛔ **MA LA SCADENZA ERA RIMASTA SCRITTA AL PRESENTE ACCANTO AL CODICE, e il commit del compito non
la toccava:** chi arrivava dopo vi leggeva l'ordine di **togliere il metodo appena diventato la
spina dorsale di `LocalPolicy`**, e il rapporto la dava per tolta mentre nel diff **non c'era
nessun hunk**. Corretta nella prima ondata (`E99`), e la falsità del rapporto registrata nella
seconda (`E107`) — perché l'ondata che la chiudeva aveva a sua volta dichiarato di aver riscritto
**due** frasi avendone riscritta **una**.

⛔ **E IL DATO DEL TASK 8 NON È CIÒ CHE HA PORTATO: È ANCORA QUANTO È COSTATO PROVARLO — ma
stavolta c'è anche il RIMEDIO, misurato invece che sperato.** Tre revisioni piene più una stretta
sulla prosa, come al Task 7, e ciascuna delle prime tre ha trovato difetti reali:
① una **scadenza scaduta** rimasta al presente nel doc di `ask_back`, quella qui sopra;
② i due argomenti che `admit` **calcola** per `ask_back` — lo **scarto** e la **corsia** — tenuti
da **nessuna** sonda, perché lo scenario del banco era **degenere**: `ceiling`, `allocated`,
`asked` e `needed` valevano tutti lo stesso numero, quattro grandezze che coincidono e nessuna
asserzione che possa distinguerle. **Due mutanti vivi sull'intero workspace** con conseguenze
vere, di cui uno faceva **degradare `LocalPolicy` a `Remote` in silenzio** su una macchina
parzialmente piena;
③ un **aiutante di banco che non teneva niente**, e il cui doc dichiarava il contrario.
📌 **E la costante: NOVE volte in due compiti un'ondata di correzioni ha chiuso dei rilievi
APRENDONE DI NUOVI NELLA PROSA scritta per chiuderli**, e **due** volte un **rapporto** ha
dichiarato fatta una correzione **assente dal diff**. ⛔ Il codice regge alla prima o alla seconda
passata; le frasi no.
✅ **E IL RIMEDIO CHE HA FUNZIONATO, ED È IL GOTCHA NUOVO #76:** all'ultima ondata è stata data
l'istruzione di **TOGLIERE invece di riscrivere meglio** — **quattordici** righe messe contro
**diciassette** tolte, **una** sola voce d'errata per l'intera passata — e la revisione successiva
è tornata **pulita**. Il ciclo si è chiuso in **una** passata invece di spiralare.
📌 *Meno parole, meno superficie per una falsità nuova.*
⛔ **E la scelta fra gotcha nuovo e forma del #45 è scritta invece di essere lasciata implicita:
è NUOVO.** Il #45 dice che il rimedio **nasce non provato**, e prescrive di rivedere l'ondata
**come** il compito: agisce sul **numero di giri**. Il #76 dice quale **forma** di rimedio
converge, e agisce sull'**istruzione** che si dà all'ondata. Due leve diverse, e la seconda porta
una misura che la prima non ha.
⚠️ **E gli altri due candidati NON sono stati promossi, il che è a sua volta una decisione:** una
**correzione dichiarata e assente dal diff** (`E107`, quinta occorrenza) è coperta fra il **#65**
— *un rapporto è un'affermazione come le altre* — e il **#45**; e una **revoca di mutazione
fallita a metà**, che ha lasciato un file mutato per sette righe ed è stata colta dal `cmp` del
protocollo e **non da un test** (`E111`), è una forma del **#48**. ⛔ Un gotcha che non insegna
niente **diluisce quelli che insegnano**, e la §9 è l'elenco che si rilegge per primo.

⛔ **E TRE VOCI SONO REGISTRATE E NON PRESE, nessuna delle quali è un difetto oggi.** **`E94`** —
la policy è consegnata come **secondo argomento** di `Arbiter::new`, mentre §2.8.2 e
[ADR-0034](adr/0034-parametri-di-decisione-consegnati-non-letti.md) fondano `V3` sul **valore
consegnato**, cioè `Parameters`. ✅ La forza di `V3` è **identica**, misurata; ma spostarla dentro
`Parameters` tocca un tipo che la §2.8 pinza con **due** righe di catalogo, e quanti chiamanti
abbia lo dice `E94` — qui non si ripete. Non è una rifinitura. **`E100`** — l'inversione di priorità di `E51`, sopra; chiusore il
**Task 10**. **`E104`** — una **dominanza** nuova fra sonde, dichiarata e **non cancellata**: una
campagna è un campione, non una dimostrazione (forma di `E37`/`E79`/`E93`). E restano aperte le
voci del Task 7.
✅ **RICHIAMO DEL 2026-08-20 — `V3` È COPERTA, e la frase si riscrive perché a essere cambiato è
il FATTO e non una qualificazione** (gotcha **#76**, e il suo limite dichiarato). Diceva
*«`V3` è contata PARZIALE e non coperta, ed è l'innesco di un compito futuro»*: quel compito è il
**Task 9**, l'innesco è **scattato** ed è stato **raccolto**. La seconda metà della contro-sonda
di catalogo — *«e la transizione resta un passo giornalato (§5.4)»* — esiste, e la riga di
§7.4.1 blocco C **chiude per intero**. ⛔ **Il numeratore delle righe coperte sta in
[`porta-di-qualita.md`](porta-di-qualita.md) e non qui**, in una casa sola: scriverlo anche in
questa sezione lo metterebbe in due documenti, che è il gotcha **#68** — e la cella di quel file
lo ha ricontato per la **sesta** volta invece di dedurlo.

⛔ **E CIÒ CHE IL TASK 10 LASCIA AL TASK 11, che NON è un pre-controllo: quello si fa prima di
dispacciarlo, contro il codice di allora.** Qui c'è solo ciò che è **misurato oggi**.
① **`set_policy` non ha ANCORA un chiamante di produzione, e la previsione del Task 9 era
falsa.** Misurato eseguendo: il grafo **costruisce** l'arbitro con `VramPolicy::Remote(RemotePolicy)`
e non **transita** mai. Una transizione all'avvio sarebbe uno scambio che nessuno chiede, quindi
**non è stata inventata** per far tornare la frase: il registro è **riscritto** e il chiamante si
sposta al primo orchestratore.
② **`E41` È CHIUSA — il chiusore era il Task 10, e ha chiuso.** `E50` e `E51`/`E100` **no**, e il
loro chiusore **non è più un numero di compito**: è chi costruirà il primo ciclo di orchestrazione.
Decisione del proprietario del **2026-08-21**, presa **prima** di dispacciare il compito, sul fatto
misurato che la radice di composizione **non orchestra**.
③ La baseline da cui il Task 11 parte è **35 target, 255 passate, 0 fallite, 2 ignorate**, col
bersaglio `daemon` a **otto** sonde — e si **rimisura**, non si cita. ④ **`V3` è coperta**, e le
righe del blocco C che restano scoperte sono `V5`, `V10` e le due di §6.10 — il numeratore lo dice
[`porta-di-qualita.md`](porta-di-qualita.md), non questa riga. ⑤ **Voci ereditate e non chiuse
qui**, tutte del proprietario: `E30` — ⚠️ **va decisa prima del Traguardo 6** — `E31`, `E32`,
`E94`, `E104`.
⚠️ **E la regola lasciata dal Task 9 è SCATTATA senza essere onorata, e va detto invece che
taciuto:** *«il primo compito che tocca `crates/kernel/tests/arbiter_admission.rs` rimisura tutte
le celle della campagna»*. Il Task 10 lo ha toccato — **quattro righe di doc, nessuna sonda** — e
nessuna cella è stata rimisurata. ✅ Misurato oggi: quel banco porta **venti** test, mentre i
verbali del 2026-08-19 ne dichiarano **diciannove**, che è la misura di allora e resta tale.
**Registrata, non presa:** se una regola debba scattare su un tocco che non muove nessuna cifra è
del proprietario.

⛔ **E CIÒ CHE IL TASK 11 LASCIA AL TASK 12, che NON è un pre-controllo: quello si fa prima di
dispacciarlo, contro il codice di allora.** Qui c'è solo ciò che è **misurato oggi**.
① La baseline da cui il Task 12 parte è quella scritta **una volta sola** più in alto in questa
sezione, nel paragrafo dei numeri — e comunque si **rimisura**, non si cita. ⚠️ Questo punto la
ricopiava, cioè teneva la stessa cifra in **due punti della stessa sezione**: è il difetto che
il paragrafo dei numeri porta scritto nel proprio richiamo.
② **Né il blocco B né il blocco C sono chiusi**, e le righe che restano **non hanno tutte la
stessa causa**: una aspetta un meccanismo che non esiste — il filtro dei vincoli, §6.3, Traguardo
6 — e un'altra aspetta soltanto un **caso di livello 1**, perché il tipo che nomina esiste già.
⚠️ Quale sia quale lo dice [`porta-di-qualita.md`](porta-di-qualita.md), che le enumera: dedurlo
da una causa sola qui sarebbe falso per almeno una riga.
⛔ **I due numeratori NON stanno qui:** li tiene [`porta-di-qualita.md`](porta-di-qualita.md), e
la riga che questo riquadro sostituisce lo diceva già con queste parole — *«non questa riga»*.
Scriverli anche qui era il gotcha **#68** commesso dentro il riquadro che lo cita.
③ **`scripts/check-docs.sh` ha una regola nuova, e chi scrive un controllo deve saperla:** il
cancello **non legge i file che git ignora**. Provata nelle due direzioni — un `.md` **non
tracciato ma non ignorato** con un link rotto lo fa uscire **1**, e i `.md` dentro una cartella
ignorata lo lasciano `GATE GREEN`. ⚠️ La distinzione fra *non tracciato* e *ignorato* è portante
e sta scritta accanto al codice: il cancello gira **prima** del commit.
④ **Voci ereditate e non chiuse qui.** Del **proprietario**: `E30` — ⚠️ **va decisa prima del
Traguardo 6** — `E31`, `E32`, `E70`, `E94`, `E104`, e ora `E140`. ⛔ **`E50` ed `E51`/`E100` NON
sono sue:** il loro chiusore è **chi costruirà il primo ciclo di orchestrazione**, e lo dice questa
stessa sezione. Erano nell'elenco perché una quantificazione scritta su una lista si legge come
verificata su tutti i nomi — gotcha **#67**, colto dalla decima revisione.
⛔ ⑤ **IL CICLO DI REVISIONE DEL TASK 11 È CHIUSO, e come si è chiuso va detto invece che
riassunto.** Ogni revisione del Task 11 ha trovato rilievi veri. ⚠️ **E la specie va detta
giusta:** i **diciannove** Important della nona, della decima e dell'undicesima sono **diciotto**
nella **prosa** e nelle **cifre derivate** e **uno** nel **codice** — il caso `compile_fail` che
sotto la propria regressione degradava a `mismatch`, nona revisione.
✅ **RICHIAMO DEL 2026-08-25 — LA REVISIONE MANCANTE È STATA FATTA, E CON ESSA ALTRE QUATTRO.**
Questo punto diceva *«E L'ULTIMA ONDATA NON È STATA RIVISTA … la regola "si rivede finché una
passata non torna pulita" è onorata a metà, e il prodotto è verde ma il verbale no»*, e lasciava
a chi riprende la scelta fra aprirne un'altra e accettare il residuo. ⛔ **Il proprietario ha
scelto di aprirla**, e ne è uscito un ciclo di **cinque revisioni** — la dodicesima fino alla
sedicesima — e **quattro ondate**, che sono i **quattro** commit `docs(ondata)` fra quella
consegna e questa. La serie degli Important, contata uno per uno sui rapporti invece che
ricordata: **7 · 4 · 2 · 2 · 1**. ✅ **E il fatto che conta più della serie: il verdetto sul
codice è lo stesso in tutte e cinque le revisioni — nessun difetto — e il cancello è uscito
`GATE GREEN` a ogni giro.** Misurato sul diff dell'intero ciclo: i file mossi sono **tre**, tutti
di documentazione, e `--name-only` limitato a `crates/`, `scripts/` e `Cargo.lock` ne nomina
**zero**.
⛔ **E il ciclo si è fermato per una ragione che NON è «una passata pulita»: è il PERIMETRO, e va
scritto perché è il residuo dichiarato.** L'unico Important della sedicesima revisione vive in un
**rapporto di lavoro che git ignora** — un conteggio di commit misurato prima del commit che lo
sposta — cioè **fuori da ciò che si consegna**: nei file tracciati quell'affermazione non c'è,
verificato rileggendo le case che quel conteggio riguarda, dove al posto del numero sta un
**rimando a `git log`**. Continuare avrebbe significato **rivedere artefatti che nessuno
riceve**, con un verdetto che dipende dalla **cartella di lavoro**: è la forma del gotcha **#80**
— quello che questo stesso compito ha chiuso dentro il cancello — applicata al **perimetro di una
revisione** invece che a quello di un controllo. ⚠️ **Il residuo, detto senza gonfiarlo e senza
tacerlo:** l'ultimo giro **non** è tornato pulito, e ciò che vi resta aperto sta **fuori dal
perimetro di consegna**. Chi riprende non eredita una correzione da fare in un file tracciato;
eredita **la ragione per cui non ce n'è una**, e la libertà di rifiutarla.
⛔ **E una causa del non-convergere è misurata, non supposta:** le cifre **rimisurate sui file**
sono giuste **ventidue su ventidue**; quelle **derivate da un testo** — il totale di un'errata,
il numeratore di un pre-controllo, il peso di un file citato altrove, un verbale che riporta i
propri numeri — sbagliate **otto su otto**. 📌 **La decisione che ne segue è del proprietario e
non è stata presa:** `CLAUDE.md` prescrive già che *«una cifra che vive in PIÙ documenti si
TOGLIE»*, e i **pesi** vivono a mano in più case, censite nella §12. Toglierli e lasciarli alla
sola §12 chiuderebbe la classe che ha prodotto **tre** dei diciannove rilievi, non tutti — ma
tocca `CLAUDE.md` e [`AVVIO-CHAT.md`](AVVIO-CHAT.md), che sono i documenti d'ingresso.
⚠️ **RICHIAMO DEL 2026-08-28:** la §12 **non li censisce più** — la colonna dei kilobyte è
uscita, come il disegno dello sfoltimento prescriveva, e al suo posto c'è il comando. Quindi
*«lasciarli alla sola §12»* non è più una via, e la decisione resta **del proprietario**: le case a
mano sopravvivono nel **messaggio** di [`AVVIO-CHAT.md`](AVVIO-CHAT.md), non qui. Il verbale sta in
[`archivio/misure-dimensioni.md`](archivio/misure-dimensioni.md).

⛔ **E DUE VOCI NUOVE ASPETTANO IL PROPRIETARIO, nessuna delle quali è un difetto oggi.**
**①** — **un quinto caso `compile_fail` è misurato e non preso:** un secondo `start` con lo
stesso `Grant` è `E0382`, *perché `Grant` non è `Copy`, e non lo può diventare perché non deriva
nemmeno `Clone`*. Se pretenda una **riga di catalogo propria** lo decide la §7.4, che è **spec**
(vincolo globale 7). ⚠️ Sta qui e non solo nel registro **per la regola che il registro stesso
scrive**: uno scarto fra codice e catalogo si dichiara come **voce aperta** portata dalla §6, mai
come nota — una nota si legge e si dimentica (gotcha **#36**). Nella cella di
[`porta-di-qualita.md`](porta-di-qualita.md) era una nota in fondo a una cella lunga cinquemila
caratteri.
**②** — **una domanda di convenzione, e nasce da un confronto fra due celle e non da un difetto.**
La riga «leggere da un worker ← una **ricevuta**» entra fra le **coperte** del blocco **B**, che è
il blocco dei *gettoni non falsificabili* di §6.3; ma `SingleReceipt::new` è **`pub`** e
raggiungibile da fuori la crate, quindi il caso prova l'**arità** e non l'**autenticità** — il
limite è già dichiarato accanto al costruttore in `crates/kernel/src/ports/process.rs` e due
volte nel registro. ⛔ **Sui termini letterali della riga di catalogo la copertura è piena**, e
non è un rilievo di correttezza. La domanda è di **coerenza**: questo stesso registro ha tenuto
`Q8 · §5.2.1` e `V3` a **PARZIALE** per lacune più strette, con la formula *«una riga parziale
non è una riga chiusa»*. Se quella convenzione regga anche qui è del **proprietario**, e va
decisa **vedendola** invece che dedotta dalla differenza fra due celle.

⛔ **E CIÒ CHE IL TASK 12 LASCIA AL TASK 13, che NON è un pre-controllo: quello si fa prima di
dispacciarlo, contro il codice di allora.** Qui c'è solo ciò che è **misurato oggi**.
① La baseline da cui il Task 13 parte è quella scritta **una volta sola** più in alto in questa
sezione, nel paragrafo dei numeri — e comunque si **rimisura**, non si cita.
② **Il Task 13 è un AUDIT e non una scrittura, e lo dice il piano nel proprio titolo:** si parte
dai numeri e non dalle frasi, ed è il gotcha **#49** alla terza occasione. ⚠️ **E un conteggio che
il piano detta è già dichiarato da rifare dal piano stesso:** le **dodici** righe di catalogo del
disegno si ricontano **sulla §7.4 della spec**, e *«se il conteggio vero diverge, vince il
conteggio»*.
③ **`E151` — il mutante vivo `M9` — è APERTA, ed è del proprietario.** Tolta ad
`Arbiter::release` la riscossione delle scadute, **niente in tutto il workspace diventa rosso** —
la campagna di mutazione riporta sotto `M9` la **stessa quaterna del verde**, `37 · 264 · 0 · 2`,
che questa passata ha rimisurato **senza** la mutazione. La ragione, letta nel codice: ogni turno
di ogni parte passa da `admit`, che riscuote per primo, quindi quando un ritardatario torna la
spazzata l'ha già fatta qualcun altro. ⛔ **Dichiarato e non pinzato, e le quattro
revisioni hanno verificato che la scelta REGGE:** la sonda che lo ucciderebbe congelerebbe la
scelta che `E30` mette davanti al proprietario — gotcha **#73** — e il doc di `Arbiter::release`
**non promette** quella riscossione, quindi il mutante non contraddice il contratto della propria
funzione. ⚠️ **Altrove però delle frasi DIVENTANO false, e non è un dettaglio:** è il rilievo che
le tre ondate hanno inseguito, e si è chiuso **sostituendo un elenco di case con una regola di
lettura** — *ogni riga che il comando restituisce si legge intera* — perché un elenco invecchia
e una regola no.
④ **Voci ereditate e non chiuse qui, tutte del proprietario:** `E30` — ⚠️ **va decisa prima del
Traguardo 6** — `E31`, `E32`, `E70`, `E94`, `E104`, `E140`, e **nuove di questo compito** `E151`
ed `E152`. ⛔ **`E50` ed `E51`/`E100` NON sono sue:** il loro chiusore è **chi costruirà il primo
ciclo di orchestrazione**, e lo dice questa stessa sezione.
⑤ **La costante di non-vacuità della campagna è un RILEVATORE DI CAMBIAMENTO, e il giorno in cui
diventa rossa si rilegge come una decisione e non come un difetto.** `EXPECTED_OUTCOMES` fissa a
**sette** gli esiti distinti dello scenario, sul precedente di `EXPECTED_DOUBT_SETS` di
`dst_campaign.rs`: il rimedio a un rosso è **rimisurare lo spazio e riscegliere i due numeri**,
non editare il sette finché la barra torna verde — che sarebbe il gotcha **#25** su una costante
scritta a mano. La frase è accanto alla costante.
⑥ **IL CICLO DI REVISIONE DEL TASK 12 È TORNATO PULITO.** Quattro revisioni, serie degli
Important **5 · 2 · 1 · 0**, contata uno per uno sui rapporti e non ricordata — ⚠️ **e diverge
da come la consegna la prezzava**, quindi la divergenza è **registrata** nella §12 invece che
appianata (gotcha **#15**). ✅ **Il verdetto sul codice è lo stesso in tutte e quattro — nessun
difetto** — e delle tre ondate **nessuna** ha toccato `scripts/` né `Cargo.lock`: misurato,
`git diff --name-only c94784b..3f89cad` limitato a `crates/`, `scripts/` e `Cargo.lock` nomina
**un solo** file, ed è il banco della campagna — mosso da **due** commit, i **commenti** della
prima ondata e la **riformattazione** `rustfmt` di `bd103c4`, che non cambia semantica.
⑦ **`cargo fmt --all --check` era ROSSO su un file di questo compito, e la deriva è arrivata col
commit del prodotto e non con un'ondata** — misurato dalla seconda revisione rieseguendo
`rustfmt --check` sui due commit. ✅ Chiuso in `bd103c4`, **un** file e **un** hunk. 📌 È il
segnale che [`porta-di-qualita.md`](porta-di-qualita.md) chiama *«nessuno lo fa rispettare»*: il
cancello non impone `fmt` per decisione della §7.4.3, quindi ogni compito che scrive Rust se lo
deve chiedere da sé.
⑧ **La trappola dei fine-riga di `CLAUDE.md` ha una TERZA forma, e l'ha trovata l'esecuzione:**
`git stash` / `git stash pop` **rimaterializzano** il file dall'indice attraverso `smudge`, e con
`core.autocrlf=true` lo restituiscono **CRLF** — ⚠️ **senza che `git diff` lo mostri**, perché
`clean` rinormalizza in scrittura: a muoversi è il solo `git ls-files --eol`. Il racconto sta in
[`porta-di-qualita.md`](porta-di-qualita.md), accanto alle altre due forme; la decisione di **non**
promuoverlo a riga di §9 sta nella §9, con la ragione.

⛔ **E il Task 13 ha prodotto TRE voci d'errata nuove.** `E153` — `riferimenti.md` resta
intoccato anche qui, ed è la risoluzione di `E146` alla seconda occorrenza. `E154` — **due
lavori veri che il brief non elencava fra i `Modify:`**, la riga *«piano da scrivere»* del
disegno e la decisione su `semi-dst.md`: 📌 *l'elenco dei file di un compito è un'affermazione
come le altre, e si legge contro il codice* — gotcha **#65** applicato all'intestazione invece
che al corpo. `E155` — il censimento `git ls-files --eol` viveva in **due** documenti, ed è
**tolto** da quello dove stava dentro una voce aperta.

⛔ **E il Task 5 ha insegnato DUE cose, entrambe uscite dalla revisione e non dall'esecuzione.**
La prima è il gotcha nuovo **#73** — *fissare con una sonda un comportamento che una decisione
aperta può cambiare trasforma la sonda in un voto contro il prenderla*: i tre valori misurati di
`release` **non** sono stati messi in un test, e la rinuncia è scritta accanto al tipo col proprio
costo (`E39`). La seconda è la **quarta forma del #31**, e datare **non** la salva: a invecchiare
non è solo il conteggio di una misura ma il **qualificatore** — *«rossa, e sola»* si legge come
una garanzia mentre un numero stantio si vede, e l'undicesima sonda ha tolto l'esclusività a due
righe del registro senza che nulla lo dicesse (`E38`).
⚠️ **E una voce aperta REGISTRATA E NON PRESA, sul modo di lavorare e non sul prodotto:**
`CLAUDE.md` prescrive che alla chiusura di ogni voce si aggiorni [`riferimenti.md`](riferimenti.md)
*«se la voce ha portato una misura o una fonte»*, e i compiti del Traguardo 5 ne hanno
portate a decine **senza toccarlo**: vivono tutte in [`porta-di-qualita.md`](porta-di-qualita.md),
accanto alla sonda che difendono. ⛔ Non è stato appianato qui perché scegliere fra *«spostarle»* e
*«cambiare la regola»* è una decisione del proprietario, e cominciare a farlo a metà traguardo
avrebbe prodotto **due** convenzioni invece di una. ⚠️ **E il Task 6 l'ha allargata invece di
chiuderla:** da solo ha portato **quindici** mutazioni misurate, tutte nel registro e nessuna in
`riferimenti.md`. ⛔ **E il Task 7 l'ha allargata ANCORA, ed è la prova più forte finora:** la sua
campagna è a **ventisei** righe — quasi il doppio di quella del Task 6 — rimisurata **da capo tre
volte** perché la suite cresceva sotto di lei, e il file è cresciuto di **quarantaquattro
kilobyte** in un compito solo, mentre `riferimenti.md` è rimasto **immobile a 198**.
⛔ **E il Task 8 l'ha allargata una QUARTA volta, con la prova di specie diversa che mancava:** la
sua campagna è stata **rieseguita per intero due volte in un giorno** — perché il banco cresceva
sotto di lei, e poi perché un aiutante ne è stato tolto — e in una di quelle riesecuzioni **lo
strumento di misura ha sbagliato**, colto dal `cmp` del protocollo. ⚠️ Sono esattamente *«le
fonti e i comandi»* di cui parla la §13, e stanno **tutte** nel registro. La voce resta
**registrata e non presa**, con una prova in più e non con una risposta: ⛔ **questa passata NON
ha toccato `riferimenti.md`, deliberatamente**, perché spostare le misure comincerebbe la
convenzione nuova senza che nessuno l'abbia scelta. ⛔ **E il Task 9 l'ha allargata una QUINTA
volta di seguito**, con l'invarianza del file che è essa stessa il dato: vedi la **46ª misura**
della §12.

⚠️ **E UNA SECONDA VOCE APERTA REGISTRATA E NON PRESA, nata il 2026-08-20 e sul modo di lavorare:
una MISURA NUOVA su una regola già presa, quella dei FINE-RIGA.** `git ls-files --eol` dice che
**nell'indice tutti i file tracciati sono LF** (`i/lf`), tranne i **quattro** dichiarati
`i/crlf`, e molti di quelli che nell'albero di lavoro sono CRLF: `core.autocrlf` vale
**`true`**. ⛔ **IL CENSIMENTO IN CIFRE È TOLTO DA QUI il 2026-08-25, e non riallineato.**
Diceva *«140 · **75** · 4 · 3»*, misurato il 2026-08-20; rieseguito oggi con
`git ls-files --eol` la seconda cifra è **85** — dieci file LF/LF in più, che il traguardo ha
aggiunto — e le altre tre reggono. ⚠️ **A renderlo stantio non è stata una svista ma la sua
collocazione:** quelle quattro cifre vivevano **in due documenti**, qui dentro una **voce
aperta** e in [`porta-di-qualita.md`](porta-di-qualita.md) dentro un **verbale datato**, dove
sono giuste e restano — un verbale dice cosa fu misurato quel giorno, una voce aperta si legge
come *«questo è lo stato»*. La regola di `CLAUDE.md` è che una cifra in più documenti **si
toglie**, e la rimisura di oggi vive nella **§12**.
⚠️ **La regola di `CLAUDE.md` resta giusta e non è riscritta:** uno script che riscrive un
sorgente ne cambia comunque i fine-riga **sotto chi ci lavora**, e i **quattro** file con `CR`
**nell'indice** sono davvero l'unico posto in cui la normalizzazione arriva fino al `git diff`.
Ma il **diff** è protetto da git più di quanto i documenti dichiarino, ed è un fatto che nessuno
aveva misurato. ⛔ **Registrata come VOCE APERTA e non come nota** — una nota si legge e si
dimentica, gotcha **#36** — e riaprire una decisione presa è del proprietario.

⚠️ **Dove sta il dettaglio, e perché non è qui:** le sonde, le mutazioni con il proprio esito
misurato e le righe di catalogo coperte stanno in [`porta-di-qualita.md`](porta-di-qualita.md),
aggiornato **a ogni compito**; le divergenze stanno nell'**errata in testa al piano**, che si
legge **prima** del compito. Questa sezione dice **dove siamo**, non ciò che quei due dicono già.
⛔ **E ciò che l'audit lascia al Traguardo 5 va saputo PRIMA di aprirlo:** le **nove righe di
guasto scoperte** della §3.3 hanno ciascuna il proprio indirizzo — e l'**arbitro è questo
traguardo** — la metà non chiudibile del gotcha **#51**, `semi-dst.md` che **non ha un chiudente**,
e la voce aperta consolidata qui sopra. ⛔ **E il disegno ha ricontato: le righe che il Traguardo 5
eredita davvero sono UNA, più una condivisa** — non cinque. Vedi il riquadro qui sotto.

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

⛔ **I TREDICI PASSI CHIUSI DEI TRAGUARDI 1–5 SONO USCITI DA QUI il 2026-09-01, e non
riassunti:** stanno in [`archivio/stato-storico.md`](archivio/stato-storico.md), parola per
parola. Erano un elenco **barrato**, ogni voce ✅ chiusa fra il 2026-08-08 e il 2026-08-25 —
cioè un **verbale**, e un verbale invecchia onestamente in archivio. È il criterio di taglio
del disegno del [2026-08-28](superpowers/specs/2026-08-28-sfoltimento-compendio-design.md):
*«resta in §6 ciò che è vero adesso»*.

⚠️ **La cernita è stata PROVATA, non dedotta.** Nessuna delle tredici voci era aperta; e i
codici d'errata ancora vivi che il blocco nominava — `E25`, `E31`, `E51` — hanno **altre case
in questa stessa §6**, quindi nessuna decisione del proprietario ha perso la propria riga viva.
Gli altri codici citati appartengono a traguardi **chiusi**, e vi erano nominati al passato.

📌 **Perché adesso e non alla prossima passata:** il tetto del compendio lasciava **ventun
byte** — misurato il 2026-09-01 con `wc -c docs/COMPENDIO.md` contro il tetto di
`scripts/check-docs.sh` — e il **prossimo** commit di documentazione avrebbe fatto rosso il
cancello. ⛔ **Il tetto è SCESO nello stesso passaggio**, perché un tetto che resta alto dopo
uno sfoltimento è il permesso di ricrescere: il numero e il suo perché vivono accanto al
controllo che difendono, in `scripts/check-docs.sh`, e in nessun altro posto.

⚠️ **E la consolidazione delle voci APERTE resta da fare, dichiarata e non fatta:** è il debito
che il riquadro in cima a questa sottosezione registra, ed è una voce del proprietario, da
presentare una per una — non un effetto collaterale di questo sfoltimento.

⛔ **E quattro questioni restano aperte nel sorgente, dichiarate e non risolte.** Nessuna delle
quattro è un difetto oggi, ed è scritto **perché**; tutte si pagano più avanti, e chi riprende
deve saperle **prima** di scrivere:

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

⛔ **Raccolte qui il 2026-08-10 perché chi riprende deve saperle PRIMA di scrivere, non
trovandole.** Nessuna è un difetto oggi, e per ciascuna è scritto **perché**; erano sparse fra il
sorgente, il registro e l'errata, che è il modo in cui una voce aperta smette di esserlo senza che
nessuno l'abbia chiusa.

| | Dove è dichiarata | Chi la chiude |
|---|---|---|
| ⛔ **ADR-0018 è violata da entrambe le implementazioni:** un payload potato e uno mai registrato sono **indistinguibili in tre modi**. La via che non costa un'impronta è stata cercata e la misura la **uccide** — svuotare il payload fa rispondere `SuspendAndAsk` su **ogni** passo potato, a ogni ripresa | voce aperta 1 di [`porta-di-qualita.md`](porta-di-qualita.md), accanto a `prune` in tutte e due, e nel blocco **7b** della conformità | il traguardo della **ritenzione**, **insieme** alla decisione sulla funzione d'impronta — che è una voce nuova nella lista di ADR-0031 |
| ⚠️ **la terza risposta di `prune` non è tenuta da nessuna promessa:** `Missing` per un passo mai scritto lo tiene **solo** il doppio in memoria, e la mutazione `M10` su `redb` **sopravvive all'intero workspace** | voce aperta 2 dello stesso file | il **primo consumatore** di `prune`, cioè la spazzata di ritenzione |
| ⛔ **le due nozioni di «in dubbio» DIVERGONO, e la divergenza cade dal lato che AUTORIZZA la distruzione:** la porta chiede *quale operazione è stata chiamata*, `steps_in_doubt` chiede *cosa dicono i record* — e un record d'esito che la build non decodifica è in dubbio per il kernel e **potabile** per la porta. ✅ **Misurato il 2026-08-27 da fuori la crate, su entrambe:** `steps_in_doubt` risponde `[InDoubt { step: StepId(1), resolution: SuspendAndAsk }]` e `prune` risponde `Ok(())`. ⛔ **Non è chiudibile sulla porta**, che non decodifica (ADR-0036): l'obbligo è di **chi chiama**, e quel chiamante **non esiste ancora** — quanti ne abbia oggi lo dice la voce aperta, in una casa sola | voce aperta **3** di [`porta-di-qualita.md`](porta-di-qualita.md), l'obbligo accanto a `Journal::prune` in `crates/kernel/src/ports/journal.rs`, e la dichiarazione nel blocco **7b** della conformità | il traguardo della **ritenzione**, come la prima riga di questa tabella e per un motivo imparentato: entrambe aspettano che qualcuno **chiami** `prune`. Finding **AUD-006** |
| ⛔ **`replay()` carica TUTTO in memoria**, e la copia dei byte è stata misurata a **tre** allocazioni per record, non una | doc di `replay` in `crates/kernel/src/ports/journal.rs`, ed **E25** dell'errata | il primo consumatore che misuri un giornale grande. Il rimedio noto è un **checkpoint**, lo stesso che pagherebbe anche le scansioni di `FileJournal` |
| ✅ **CHIUSA NELLA METÀ CHIUDIBILE il 2026-08-11** — che la durabilità sia **chiesta** — e ⛔ **aperta nella lettera dell'enunciato: la MORTE del processo.** Ciò che si osserva è una chiamata a `sync_data` su un backend **nostro**, dentro un processo **vivo**; non sono osservati che la chiamata raggiunga il supporto, l'**ordine** fra `write` e `sync_data`, il commit di `prune`, né un modello di guasto in cui una scrittura non sincronizzata possa davvero **sparire** — misurato, a `falls_at = 45` il record si rilegge benché la caduta abbia rifiutato proprio il `sync_data` del suo commit. Il perimetro per esteso in [`riferimenti.md`](riferimenti.md). ⚠️ **La riga originale:** la durabilità attraverso la morte del processo non è osservabile da dentro il processo, e `Durability::None` lascia **sei test su sei verdi** | accanto al codice in `crates/platform/src/journal.rs`, gotcha **#51** | l'**iniezione di livello 2** del **Traguardo 4**, attraverso il `StorageBackend` che il Task 8 ha reso sostituibile. ⛔ **QUESTA CELLA HA DETTO IL FALSO DAL BRAINSTORMING ALL'ESECUZIONE, e la correzione è del 2026-08-11, misurata due volte.** Diceva: *«con `Durability::None` `redb` non chiama `sync_data`, quindi un backend che conta le chiamate lo dice — una campagna che pretende «`sync_data` è scattato almeno una volta» diventa rossa appena la garanzia sparisce»*. **È falsa in entrambe le metà:** sotto quella mutazione `redb` chiama `sync_data` **sette volte all'apertura** e arriva a undici, perché **sei sync su sette nascono prima che esista un record** — `create_with_backend` nudo ne fa sei; e la forma *«almeno una volta»* è quindi **l'oracolo cieco per eccellenza**, verde proprio sotto la mutazione che esiste per cogliere. ✅ **La forma giusta è un DELTA attraverso la scrittura** — il conteggio dopo la scrittura maggiore di quello dopo l'apertura — e da lì il #51 è **chiuso nella metà chiudibile**: vedi la §6 e il perimetro scritto in [`riferimenti.md`](riferimenti.md). 📌 **La forma generale, che vale oltre il caso:** un contatore che parte da un valore che **il soggetto sotto esame non ha prodotto** non è un oracolo su quel soggetto. ⚠️ E il difetto non era il numero ma la **previsione**: la cella fu scritta quando il backend cadente non esisteva — gotcha **#57**, *«una decisione presa prima che esistesse ciò di cui parla è una previsione, e si cita come se fosse una misura»* |
| ⚠️ **le guardie di `FileJournal` sono SCANSIONI**, ~56 ns per record, e `has_intent` si paga a ogni scrittura: supera il pavimento dell'`fsync` solo oltre ~26 000 record | doc di `FileJournal`, e le misure in [`riferimenti.md`](riferimenti.md) | nessuno **finché nessuna misura lo chiede**: il rimedio è lo stesso checkpoint, e due meccanismi per una misura sola si comprano quando la misura c'è |
| ⛔ **le vie A1, A2, A5, A7 del confine dei dati non fidati** restano aperte | `crates/kernel/src/boundary.rs`, voce per voce | ⛔ **nessuno**, e ciascuna lo **dichiara**: non è un arretrato, è il **pavimento** |
| ⚠️ **l'amplificazione dello spazio di `redb`**, misurata in M-8 su carico **sintetico** | §4.8 della spec | *«da rimisurare sul carico reale prima di congelare i parametri di ADR-0018»* |
| ⚠️ **il `kind` del record e l'operazione della porta restano due verità indipendenti**, e nulla di livello 1 impedisce a uno scrittore futuro di farle divergere | `crates/kernel/src/reconcile.rs` | ✅ **chiusa come DECISIONE dal proprietario**, non come garanzia: **ciascuno** degli scrittori ha la propria sonda. ⚠️ **RICHIAMO DEL 2026-08-21:** questa cella diceva *«la sonda copre l'unico scrittore che esiste, e l'aiutante nasce col secondo»*, e il secondo è arrivato col **Task 9** — `Arbiter::set_policy` — senza che nulla diventasse rosso. Gotcha **#77**. Se l'aiutante vada costruito è **registrato e non preso**: è del proprietario |
| ⚠️ **il registro non è sorvegliato** dalla guardia dei conteggi | il capoverso qui sopra | il **proprietario**: allargare la lista non basta, servirebbe un controllo diverso — **registrata, non presa** |
| ⚠️ **il puntatore al prossimo passo non ha una guardia**: dal 2026-08-18 vive in un posto solo, ma **nulla impedisce** a un documento di ricominciare a riscriverlo domani — ed è già successo tre volte. ⛔ La forma meccanizzabile esiste e costa un comando: *fuori da `COMPENDIO.md`, ogni riga che porta `⏭️` deve nominare la §6* | il riquadro di chiusura qui sopra, e la 25ª misura della §12 | il **proprietario**: è una **riga di catalogo** nuova in `check-docs.sh`, cioè una sua decisione (vincolo globale 7) — **registrata, non presa**, come la guardia sui pesi e l'elenco dei semi |
| ⚠️ **l'elenco dei semi non avrà un chiudente**, e sarà l'unico artefatto del Traguardo 4 senza: nessun controllo pretende che una sua voce **nomini un test esistente**, e un elenco di semi senza proprietà è l'artefatto che marcisce meglio di tutti | §10 del [disegno del Traguardo 4](superpowers/specs/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst-design.md) | il **proprietario**: sarebbe una riga di catalogo nuova in `check-docs.sh` — **registrata, non presa**, come la guardia sui pesi della §12 |
| ⛔ **i 25,8 µs di M-2 sono citati in tre punti della SPEC, e la cifra è falsificata** — non è confrontabile con nulla che esista oggi: il prototipo non è nel repository, l'esecutore era un altro, il protocollo era un colpo singolo, e lo scenario **aveva** il giornale, contro la formula *«scenario minimo»* che lo fa leggere altrimenti. ✅ La conclusione che sostiene — *«migliaia di semi stanno dentro un secondo»* — **regge ed era per difetto** | richiami datati già scritti in [`HANDOFF.md`](HANDOFF.md), [`riferimenti.md`](riferimenti.md) e [`design/08`](design/08-strategia-di-test.md), col numero vivo. Restano le tre citazioni nella **spec del sotto-progetto 1** | il **proprietario**: la spec si approva sezione per sezione, e un richiamo datato lì è una sua decisione — **registrata, non presa** |
| ⚠️ **il portachiavi non ha un chiudente scritto**: nessuno script verifica che solo `secrets` lo raggiunga, e nessuna riga dice chi lo farà. ⛔ **RICHIAMO DEL 2026-08-27, finding AUD-026: la seconda metà è falsa da oggi** — la §8 della spec lo dice; la prima resta vera | riga di *«Cosa la porta NON controlla»* in [`porta-di-qualita.md`](porta-di-qualita.md), e le righe **V34**, **Q24** e **Q17** della §8 della spec | ✅ **ASSEGNATO il 2026-08-27** — le tre righe passano a ⏳ **rimandato** con innesco, sul precedente di **V16** (§8.5.3.1). ⚠️ **Quale innesco non si scrive qui**: la casa unica è la colonna *Innesco* di §8.3 e §8.4. ⛔ **E nel merito la voce resta aperta**: assegnare un innesco non è scrivere il controllo |

⛔ **Nessuna rinumerazione di sezioni**: lo script legge §7.4 e §8 **per posizione**.

| # | Traguardo | Stato |
|---|---|---|
| **1** | **scheletro e porta di qualità** — le cinque crate e i controlli, **zero logica** | ✅ **eseguito il 2026-08-08**, `GATE GREEN` |
| **2** | **il substrato iniettabile** — tempo, casualità, I/O, scheduling, l'esecutore, le sei porte | ✅ **eseguito il 2026-08-10**, `GATE GREEN`. [Piano](superpowers/plans/2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md) scritto ed eseguito **per intero, quattordici compiti su quattordici**: i due tempi · la porta `Rng` · i parametri consegnati · la porta `Reactor` · **l'esecutore** · l'orologio virtuale · **il reattore reale e la prima suite di conformità** · il **cablaggio di produzione** in `daemon`, coi default letterali · il **confine dei tipi** `Untrusted`/`Instruction`, con la promozione che pretende la porta `journal` · le porte **`filesystem` e `network`** · la porta **`process`**, coi gettoni e le **due ricevute distinte** · la porta **`ipc`**, che chiude le **sei famiglie** · il **registro dei controlli** e questa chiusura. ⛔ **Zero record del giornale scritti**, ed è deliberato: i byte congelati appartengono al Traguardo 3 |
| **3** | giornale e formato durevole — la porta a byte, l'enum di versione, **i byte congelati** | ✅ **eseguito il 2026-08-10, dodici compiti su dodici**, `GATE GREEN` a tutti. ⚠️ **Ricontati il 2026-08-10 chiudendo il traguardo:** diceva *«otto compiti»*, ed era la terza delle tre cifre discordi dello stesso file. ⚠️ **Ricontati il 2026-08-10:** diceva *«due compiti»* ed era già indietro di uno al commit precedente, di **tre** a questo — e chiamava il compito *«la conformità coi **tre** bugiardi»* quando i bugiardi consegnati sono **sette**. Il numeratore lo muove chi esegue, e chi esegue guarda la §6. [Piano](superpowers/plans/2026-08-10-sottoprogetto-1-traguardo-3-giornale-e-formato-durevole.md) **scritto il 2026-08-10**, dodici compiti in due parti: ✅ il record versionato · ✅ la riga di catalogo dell'etichetta · ✅ il **doppio in memoria** · ✅ la **conformità coi sette bugiardi** e ✅ `replay()`, eseguiti come un compito solo · ✅ la **riconciliazione su un insieme**, che ha riportato indietro la firma di `replay()` invece di deciderla · ✅ **`promote` che diventa una nota**, con l'operazione `note()` e la variante `RecordKind::Note` che il compito ha dovuto inventare · ✅ **`redb` in `platform`** col **backend nostro**, la chiave progressiva e la prova che il confine è **sostituibile da fuori** · ✅ la conformità contro **entrambe** a ogni commit · ✅ **i byte congelati**, tre record e una mappa riletta dal banco · ✅ `prune` che rifiuta un passo in dubbio · ✅ la **chiusura**, che è stata un **audit** e non una scrittura. ⛔ **Congelamento per ultimo**, che è la decisione D1 del piano |
| 4 | il simulatore DST — **il guasto**, non il tempo virtuale: quello è del Traguardo 2 | ✅ **eseguito il 2026-08-11, dieci compiti su dieci**, `GATE GREEN` a ciascuno. ⛔ **L'errata è a settanta voci in nove passate, di cui dodici DECISIONI** — il pre-controllo ha trovato un difetto in **dieci compiti su dieci**. La più importante è **E52**: due righe dei **documenti di stato** dicevano il falso su come chiudere il gotcha **#51**, e lo dicevano **dal brainstorming**. ⛔ **E la lezione imparata TRE volte:** *«l'iniezione è avvenuta»* e *«c'era qualcosa da verificare»* sono **due** affermazioni, e una campagna che tiene solo la prima è **verde avendo confrontato insiemi vuoti** — successo a `C7a`, poi a `C7b`, poi al ciclo di livello 2, **ogni volta dopo che la precedente era stata chiusa**. ✅ **Brainstorming, disegno e piano tutti il 2026-08-11** — [il disegno](superpowers/specs/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst-design.md), che fissa il perimetro (il **motore**, non tutte le finte), i **due livelli come due campagne**, l'oracolo di non-vacuità e i **sette** artefatti col controllo che esercita ciascuno; e il [piano](superpowers/plans/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst.md), **dieci compiti in tre parti** — il giornale cadente · lo scenario giornalato e `C7a` · `C7b` con l'oracolo preso dalla **traccia** e non dall'archivio · la campagna breve col numero di semi **misurato** · il backend cadente scritto **da fuori la crate** · la coerenza dopo la riapertura e il **#51 chiuso dal conteggio dei `sync_data`** · la campagna di livello 2 · l'elenco dei semi · il tempo di parete nel cancello · la chiusura. ⚠️ Il titolo diceva *«tempo virtuale, guasti, campagna, semi»* e il tempo virtuale era eseguito da due traguardi |
| 5 | arbitro GPU — ammissione, corsie, concessione, le due policy | ✅ **ESEGUITO il 2026-08-25**, `GATE GREEN` a ciascun compito — aperto il 2026-08-18 con brainstorming chiuso, [disegno](superpowers/specs/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu-design.md) e [piano](superpowers/plans/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu.md) scritti lo stesso giorno, esecuzione dal 2026-08-19. ⛔ **Il traguardo non CREA righe di catalogo: ne CHIUDE dodici** già scritte — **undici chiuse e una dichiarata PARZIALE**, ricontate sulla §7.4 dal Task 13. ⛔ **Le voci che lascia aperte stanno in una tabella sola** di [`porta-di-qualita.md`](porta-di-qualita.md), con la colonna di chi le chiude. ⛔ **A che punto sia NON è scritto qui**, ed è il rimedio e non una svista: questa cella ha detto *«esecuzione da fare»* e poi *«QUATTRO compiti su tredici»* mentre la §6 diceva **cinque**, cioè la stessa cifra in due posti dello **stesso file** con due valori — la forma peggiore del gotcha **#31**. Il numeratore lo muove chi **esegue**, e chi esegue guarda **la §6**, dove vive in un posto solo. ⛔ **RICHIAMO DEL 2026-08-25: la chiusura del traguardo ce l'ha RIMESSO** — *«tredici compiti su tredici»*, nella stessa cella che qui sopra dichiara di non portarlo, e insieme alla voce 13 dell'ordine faceva **tre** case dentro la §6 contro l'*«in nessun altro punto di questa sezione»* del riquadro in cima. **Tolto di nuovo**, ed è il **#68** ricreato dentro il riquadro che lo vieta: una regola scritta in un documento non vincola quel documento, nemmeno quando il documento è quello che la ospita. ⚠️ **Questa cella diceva *«eredita CINQUE delle nove righe di guasto»*, e la cifra era sbagliata**: contate sulla §7 del disegno del Traguardo 4 sono **una**, più una condivisa. Non è ricorretta qui — **è tolta**, e la cella rimanda alla fonte: una cifra che vive in più documenti si toglie (`CLAUDE.md`). ⛔ **E non porta il puntatore al suo posto:** quello vive **in cima a questa sezione**, in un posto solo |
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

📌 **Il ritratto pieno, per il confronto della prossima volta:** **diciotto ✅ · tredici ⚠️ ·
sei ⏳** per i V · **nove · otto · sette** per i Q. ⚠️ «Tredici» era anche il numero di
partenza, ma per una tabella diversa: la storia sta in §8.8, e si riconta **ogni volta**.

---

📚 **Gli stati passati** — i verbali dei traguardi chiusi, dei due audit e delle voci già
chiuse — stanno in [`archivio/stato-storico.md`](archivio/stato-storico.md).
⛔ **Non è una lettura obbligatoria.**

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

## 9. I gotcha

⛔ **La loro casa è [`HANDOFF.md`](HANDOFF.md), sezione «I gotcha», ed è UNA SOLA.**
Ogni voce porta la trappola, il perché fa male, e — dove è stato corretto — il verbale
della correzione con la data.

📌 **Quanti siano lo dice il comando, non questa riga:**

```
awk '/^## I gotcha/{s=1; next} s&&/^## /{s=0} s&&/^\| [0-9]+ \|/{c++} END{print c}' docs/HANDOFF.md
```

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

## 10. Le trappole di `check-docs.sh`

Da sapere **prima** di scrivere, non dopo il rosso.

⚠️ **RICHIAMO DEL 2026-08-28: il titolo diceva *«Le cinque trappole»*.** Il numerale è **tolto e
non riallineato a sei** — è una popolazione che cresce a ogni trappola misurata, e la tabella
qui sotto è la sua casa unica. Gotcha **#68**.

| # | Trappola |
|---|---|
| **1** | **I conteggi.** Ogni occorrenza di `<cifra> ADR`, `<cifra> ADR in stato ...` e `<cifra> decisioni architetturali` nei documenti di stato è confrontata con la realtà. Scrivere `2 ADR nuovi` la fa scattare, perché legge il `2` come **totale**. ⚠️ **Per i numeri piccoli si usano le parole**; gli esempi vanno nei code span — e **il code span non deve andare a capo**, perché lo spogliamento è riga per riga. Punti ciechi dichiarati: un numero **a parole** è invisibile, e così `<cifra> decisioni` **senza** «architetturali» |
| **2** | **La numerazione.** Il controllo sui duplicati è **per file** e cattura `^#{2,3} <numero>`, quindi `### 7.4.1` sarebbe letto come duplicato di `### 7.4`. **Le sotto-sotto-sezioni si scrivono con `####`** |
| **3** | **Due tabelle sono lette _per posizione_.** Nel **catalogo §7.4** la contro-sonda è l'**ultima** colonna e non può essere vuota. In **§8.3 e §8.4** le colonne sono **cinque**, con lo stato in **terza** e l'innesco in **quinta**. ⛔ E i **delimitatori sono intestazioni** (`#### 7.4.1`, `#### 7.4.3`, `## 8.`): rinumerarle è un **rosso**, non un ritocco. ⚠️ **La sesta asserzione fa eccezione, e deliberatamente:** la colonna «Difende» del catalogo **non è sempre la prima** — nei blocchi A e C e in §7.4.2 lo è, nel **blocco B dei gettoni è la terza** — quindi si cerca per **intestazione**. Non «uniformarla» alle altre: un controllo posizionale giudicherebbe la colonna sbagliata su cinque righe |
| **4** | **Un falso positivo in attesa.** La guardia dei conteggi gira su una lista fissa di documenti di stato. In `tracciabilita.md` esistono righe come `§4 ADR-0008`, dove il regex leggerebbe `4 ADR`. **Oggi non scatta**, perché quel file non è nella lista. Se servisse aggiungerlo, il rimedio è il **regex**, non il documento |
| **6** | ⛔ **Il controllo dei link NON verifica i FRAMMENTI, e un'ancora pura è INVISIBILE — misurato il 2026-08-28.** Il passo estrae con `grep -o '](\([^)#]*\.md\)[^)]*)'` e poi taglia con `cut -d'#' -f1`, quindi un rimando *«parentesi-quadra-chiusa, tonda, `file.md`, cancelletto, ancora»* è controllato **solo** per la parte `file.md`, e la forma **senza file** — solo `#ancora`, un rimando dentro lo stesso documento — **non viene nemmeno estratta**. ✅ **Provato sulla pipeline vera**, tre casi in un file temporaneo: quello con un file esistente e quello con file **più** ancora inventata escono **entrambi senza il frammento**; quello con la sola ancora **non esce affatto**. ⚠️ **E gli esempi qui sopra sono scritti a parole per FORZA:** scritti nella loro sintassi vera facevano **rosso il cancello** — `broken link: docs/COMPENDIO.md -> vero.md` — perché il controllo **non distingue un esempio da un rimando**, che è il cugino della trappola **5**. 📌 **Quindi un'ancora è un rimando che nessun controllo difende**, e marcisce in silenzio quando un titolo cambia: una sezione si **nomina** invece di collegarla, oppure si accetta il rischio **sapendolo**. Trovata scrivendone una chiudendo AUD-013, e tolta prima del commit |
| **5** | ⛔ **Il controllo dei link NON legge i file che git IGNORA — dal 2026-08-24.** Un `.md` dentro `.superpowers/`, `/scratch/` o `/tmp/` non è controllato, ed è **voluto**: prima lo era, e il verdetto del cancello dipendeva allora dalla **cartella di lavoro** invece che da ciò che si consegna. ⚠️ **La distinzione che conta, e non è la stessa cosa:** un file **non tracciato ma non ignorato** — un documento nuovo che nessuno ha ancora `git add`-ato — **è letto**, perché il cancello gira **prima** del commit ed è lì che il controllo serve. ⚠️ E il filtro **fallisce aperto**: se l'interrogazione a git non risponde si scandisce tutto. Gotcha **#80** |

---

## 11. I quindici vincoli sul primo commit di codice

Non sono decisioni da prendere: sono decisioni **prese**, che ogni piano deve tradurre in
passi.

✅ **I primi cinque sono onorati dal Traguardo 1** — cinque crate · `no_std` + `alloc` +
`forbid` su `kernel` e `simulator` · `bincode` appuntato a `2` con la ragione accanto ·
il bersaglio del cancello dichiarato in `rust-toolchain.toml` · `spikes/` fra gli
`exclude`. ⚠️ Il quarto ha una sottigliezza misurata: gotcha **#38**.

⛔ **RICHIAMO DEL 2026-08-27, finding AUD-007 — questa riga diceva *«gli altri dieci restano
davanti, e chi li copre è scritto in `porta-di-qualita.md`»*, e le affermazioni false erano
DUE.** La prima era ferma alla chiusura del **Traguardo 1** e non è mai stata riletta:
`git log -L 2534,2538:docs/COMPENDIO.md` dà **una sola scrittura**, `cf2983f`, in un file la
cui intestazione si data al Traguardo 5 — e lo stesso file la smentisce in §5, dove i **byte
congelati** (vincolo 14) esistono dal 2026-08-10. La seconda mandava al registro per una
copertura che il registro **non tiene**: [`porta-di-qualita.md`](porta-di-qualita.md) mappa le
righe di catalogo della **§7.4**, non i vincoli di questa sezione — e quanto poco vi si affacci
questa §11 lo dice `grep -c '§11' docs/porta-di-qualita.md` contro `grep -c '§7.4'` sullo stesso
file, che è un rapporto e non una cifra da tenere aggiornata.
⛔ **E il rimedio non è riallineare la cifra a un numero nuovo, che è la parte da ricordare:**
un numeratore che cresce a ogni traguardo è esattamente ciò che è marcito qui. Al suo posto c'è
una **regola di lettura**, che resta vera quando una riga se ne va — la stessa cura che la §6
ha usato per `M9`: *un elenco invecchia, una regola no*.

📌 **La regola: resta davanti solo ciò che la tabella qui sotto nomina, e ogni vincolo che non
vi compare è onorato.** Misurati uno per uno contro il codice il 2026-08-27, **coi comandi**.

| Vincolo | Cosa resta davanti, e il comando che lo dice | Chi lo chiude |
|---|---|---|
| **8**, la sola terza gamba | la **DST profonda su ciclo lungo**: le due campagne sono `#[ignore]` con la propria ragione scritta accanto, e **nessuno le lancia mai** — `grep -nE 'schedule\|cron\|--ignored' .github/workflows/quality-gate.yml scripts/gate.sh` non torna **nulla**. ⚠️ Le altre due gambe reggono e non sono in dubbio: il livello 1 **è** il compilatore, il livello 2 gira dentro il `cargo test` del cancello | il **proprietario**: un passo di CI nuovo è una sua decisione (vincolo globale 7), come `cargo audit` della voce **X-3** dell'audit |

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

| Se ti serve… | Apri |
|---|---|
| ⛔ **il verbale del SECONDO audit completo, e la sua DELEGA** — i 73 finding con causa radice, riproduzione e stato, le sette radici, e la sezione *«Come si concludono quelli aperti»*, che è la **ricetta**: lo stato alla consegna, ciò che NON è verificato, la disciplina in cinque passi e l'ordine consigliato. ⛔ **La colonna «Stato» di quel rapporto è la CASA UNICA di quali finding siano chiusi** — non si ricopia altrove. ⚠️ **Si legge a FINDING, mai intero.** ⚠️ **Riga aggiunta il 2026-08-27:** mancava dalla tabella dal giorno in cui il file è nato, ed è la stessa specie di difetto che la 7ª e la 15ª misura registrarono — *per accorgersi di una riga ASSENTE bisogna partire dall'elenco dei file citati, non dalle righe presenti* | [`audit-2026-08-27.md`](audit-2026-08-27.md) — ⚠️ **a finding, mai intero** |
| il **verbale del primo audit completo** — le quattro radici, i finding con causa radice e dimostrazione, ciò che è stato verificato **pulito**, e la §8 con le otto decisioni, **tutte eseguite** fra il 2026-08-17 e il 2026-08-18. ⛔ **Si apre per il METODO, non per il compito:** è il posto in cui si legge come un rimedio si prezza leggendo il codice invece del rapporto — più piccolo, più grande, o di specie diversa. ⚠️ **Questa cella diceva *«COSA DEVI FARE ADESSO … ne restano tre … è il prossimo passo»***, corretta il 2026-08-18 | [`audit-2026-08-11.md`](audit-2026-08-11.md) — oggi una **consultazione** |
| il **perché** di una decisione, le alternative scartate, i costi accettati | `docs/adr/<numero>-*.md` — **uno solo** |
| il **come** del sotto-progetto 1: §0–§8 con le evidenze delle misure | [`specs/2026-08-06-sottoprogetto-1-kernel.md`](superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md) — ⚠️ **a sezioni, mai intera** |
| ⛔ **il perimetro del Traguardo 5** — l'arbitro: quanto ne costruisce, le forme che la §5 descrive a parole, e per ogni artefatto **il controllo che lo esercita**. ⛔ **Si legge PRIMA di scriverne il piano**, ed è il file da cui si riprende | [`specs/2026-08-18-…-traguardo-5-arbitro-gpu-design.md`](superpowers/specs/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu-design.md) — ⚠️ **non è una spec**: è lo scaglionamento e le forme che la §5 non fissa |
| ⛔ **come si ESEGUE il Traguardo 5** — tredici compiti in cinque parti, col codice per ogni passo, le mutazioni da provare e i comandi. ⚠️ **L'errata in testa si legge PRIMA del compito**, e il pre-controllo del piano — le sette voci — sta subito sotto | [`plans/2026-08-18-…-traguardo-5-arbitro-gpu.md`](superpowers/plans/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu.md) — ⚠️ **a compiti, mai intero** |
| ⛔ **il perimetro del Traguardo 4** — quanto ne costruisce, dove vive ciascun pezzo, e per ogni artefatto **il controllo che lo esercita**. Si legge **prima** di scriverne il piano | [`specs/2026-08-11-…-traguardo-4-simulatore-dst-design.md`](superpowers/specs/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst-design.md) — ⚠️ **non è una spec**: è lo scaglionamento che la §3 non fissa |
| il **cosa** del kernel: §0–§10 | [`specs/2026-08-06-kernel-design.md`](superpowers/specs/2026-08-06-kernel-design.md) |
| il testo integrale dei **gotcha** e delle **misure**, con i numeri | [`HANDOFF.md`](HANDOFF.md) — ⚠️ **a sezioni** |
| ⛔ **cosa una sezione deve incassare, prima di proporle una modifica** | [`HANDOFF.md`](HANDOFF.md) — il **consuntivo voce per voce**: cosa era stato deciso, dove è finito, e cosa resta da scrivere. È **autorevole**, e si legge **prima** di proporre, non dopo. ⚠️ **La sezione, non il file** |
| l'ordine dei dodici sotto-progetti e le dipendenze | [`roadmap.md`](roadmap.md) |
| dove vive una funzionalità della mappa originale | [`tracciabilita.md`](tracciabilita.md) — ⚠️ **leggi il riquadro in testa**: risponde a «dove vive», **non** a «di quale meccanismo ha bisogno». È la crepa da cui sono uscite le sette voci |
| **dove vive ogni controllo** della porta, riga per riga sul catalogo §7.4, e cosa **non** è coperto | [`porta-di-qualita.md`](porta-di-qualita.md) |
| ⛔ **perché un seme NON è un oracolo**, e cosa identifica un caso in ciascuna delle due campagne DST — al livello 2 *«un seme»* **non esiste** | [`semi-dst.md`](semi-dst.md) — ⚠️ **nasce vuoto**, e la riga vuota è deliberata; ⛔ **è CRLF integrale**, misurato il 2026-08-25 |
| la **strategia di test** — è la fonte di verità sulla porta di qualità, e mappa Q1–Q24 → metodo | [`design/08-strategia-di-test.md`](design/08-strategia-di-test.md) |
| la **topologia dei processi** — contiene la tensione che F1b deve conciliare | [`design/01-topologia-dei-processi.md`](design/01-topologia-dei-processi.md) |
| gli altri diagrammi della struttura | [`design/`](design/) — nove file |
| gli **esiti degli spike**, con seed, versioni e comandi | [`../spikes/RISULTATI.md`](../spikes/RISULTATI.md) |
| i requisiti della GUI, G1–G21 e P1–P4 | [`../spikes/GUI-REQUISITI.md`](../spikes/GUI-REQUISITI.md) |
| la **provenienza** di ciò che non abbiamo dedotto noi, con le date | [`riferimenti.md`](riferimenti.md) |
| il **modello** di come si scrive un piano qui, con l'errata in testa | [`plans/2026-08-06-spike-linguaggio-del-core.md`](superpowers/plans/2026-08-06-spike-linguaggio-del-core.md) |
| ⛔ **cosa il piano del Traguardo 1 detta e il repository smentisce** — quattro voci, prima fra tutte gli identificatori italiani | [`plans/2026-08-08-sottoprogetto-1-traguardo-1-scheletro-e-porta.md`](superpowers/plans/2026-08-08-sottoprogetto-1-traguardo-1-scheletro-e-porta.md) — ⚠️ **solo l'errata in testa**, il resto è eseguito |
| ⛔ **come si esegue un piano qui, e le quattro specie di difetto** — è il piano del Traguardo 2, **eseguito per intero**, con quarantanove voci di errata in sei passate | [`plans/2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md`](superpowers/plans/2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md) — ⚠️ **a compiti, mai intero**. ⛔ **RICHIAMO DEL 2026-08-28, finding AUD-035:** la cella diceva *«è il secondo file più grande del repository, dopo la spec»*, ed è **tolta e non riallineata** — era **ottavo** quando il finding lo misurò il 2026-08-27 e **decimo** un giorno dopo, e un ordinamento marcisce come una cifra. Lo rifà il comando sotto questa tabella |
| ⛔ **come si esegue un piano, e come si CHIUDE un traguardo** — è il piano del Traguardo 3, **eseguito per intero**, dodici compiti su dodici. ⚠️ **L'errata in testa si legge prima del compito**, ed è a **settantasette voci in nove passate**, di cui **nove decisioni**; le ultime tre sono la **Definizione di «fatto» che invecchia** | [`plans/2026-08-10-sottoprogetto-1-traguardo-3-giornale-e-formato-durevole.md`](superpowers/plans/2026-08-10-sottoprogetto-1-traguardo-3-giornale-e-formato-durevole.md) — ⚠️ **a compiti, mai intero** |
| ⛔ **come si esegue un piano quando il pre-controllo trova un difetto in DIECI compiti su dieci** — è il piano del Traguardo 4, **eseguito per intero**. ⚠️ **L'errata in testa è a settanta voci in nove passate, di cui dodici DECISIONI**, e si legge **prima** di riaprire qualunque cosa che quel traguardo abbia toccato | [`plans/2026-08-11-…-traguardo-4-simulatore-dst.md`](superpowers/plans/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst.md) — ⚠️ **a compiti, mai intero** |
| l'indice di ADR e diagrammi | [`README.md`](README.md) |
| ⛔ **il messaggio da incollare all'inizio di una chat**, e il perché di ogni sua riga | [`AVVIO-CHAT.md`](AVVIO-CHAT.md) — ⚠️ **il peso del messaggio lo dà il comando sotto questa tabella**, non questa cella: ⛔ **RICHIAMO DEL 2026-08-28** — diceva *«**20606 byte LF** su **303** righe»*, ed è invecchiato lo stesso giorno, quando la riga 3 del messaggio ha smesso di dire che l'audit era il compito di oggi. Ciò che **resta** qui è il **metodo** — le righe **fra le due recinzioni, escluse** — senza il quale due lettori onesti ottengono due numeri (59ª misura) |

⚠️ **I pesi non stanno più in questa tabella, e non è una svista.** Un peso scritto
invecchia al primo commit che tocca il file; il comando che lo produce no:

```
find docs -name '*.md' | xargs wc -c | sort -n
```

📌 **E il messaggio di [`AVVIO-CHAT.md`](AVVIO-CHAT.md), che ha un perimetro proprio** — le
righe fra le due recinzioni, escluse:

```
awk '/^```$/{n++} n==1 && !/^```$/{c++; b+=length($0)+1} END{print c" righe, "b" byte"}' docs/AVVIO-CHAT.md
```

📚 **Le misure storiche** — il verbale di come i pesi sono cambiati dal
2026-08-08 al 2026-08-28 — stanno in
[`archivio/misure-dimensioni.md`](archivio/misure-dimensioni.md). ⛔ **Non è una lettura
obbligatoria:** si apre con una domanda storica in mano, non per farsi un'idea.

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
| ⛔ **e un SECONDO controllo, sulla TAGLIA** | `check-docs.sh` respinge un compendio sopra un **tetto in byte**. Il numero e il suo perché vivono **accanto al controllo**, in `scripts/check-docs.sh`, e in nessun altro posto. ⚠️ **È un tetto, non un obiettivo, e il verde NON è un segnale di margine:** il 2026-09-01 era verde a **ventun byte** dal rosso. Chi aggiunge testo qui si chiede **prima dove va**: ciò che è vero adesso resta, un **verbale** va in [`archivio/`](archivio/) — gotcha **#100** |

📋 **Il messaggio da incollare all'inizio di una chat** vive in
[`AVVIO-CHAT.md`](AVVIO-CHAT.md). Non nomina il prossimo passo, deliberatamente: lo
stato sta nella §6, in un posto solo.

**Cosa aggiornare, e quando:**

| Evento | Cosa tocchi qui |
|---|---|
| ADR nuovo | una voce in **§5** — obbligatoria, la pretende lo script |
| ADR superato | la voce resta e si marca; gli ADR sono **append-only** |
| voce della riapertura chiusa | la tabella e l'ordine in **§6** |
| gotcha nuovo | ⛔ **niente qui:** la casa è **una sola**, la sezione *«I gotcha»* di [`HANDOFF.md`](HANDOFF.md), e la §9 vi **rimanda** invece di copiare. ⚠️ **RICHIAMO DEL 2026-08-28:** questa riga diceva *«una riga in §9, e il testo integrale in `HANDOFF.md`»*, cioè ordinava di **ricreare il duplicato** che lo sfoltimento aveva tolto lo stesso giorno — gotcha **#68**, dentro la tabella che governa la manutenzione |
| **misura nuova** | le **fonti** e i **comandi** in `riferimenti.md`, la riga d'esito in `HANDOFF.md`, e le evidenze nell'ADR o nella sezione che la misura decide. ⛔ I prototipi restano nello scratchpad e si ripuliscono |
| decisione dello stack | **§4** |
| cambio del prossimo passo | **§6** |

Il resto della manutenzione — `roadmap.md`, `tracciabilita.md`, stato degli spike,
`HANDOFF.md`, `CLAUDE.md` — resta come prima: **nello stesso passaggio**, alla chiusura
di ogni sotto-progetto.
