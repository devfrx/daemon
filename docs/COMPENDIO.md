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
> accettati — apri **quel** file. Uno, non tutti. La §12 dice quale.
>
> ⛔ **Cosa NON fare.** Non aprire `HANDOFF.md`, la spec del sotto-progetto 1, o la
> cartella `adr/` «per farsi un'idea». Insieme pesano **oltre mezzo megabyte**
> (812 KB in byte LF il 2026-08-27, e possono solo crescere — la spec da sola ne fa 298), e
> l'idea è già qui.

**Aggiornato il 2026-09-09**, col puntatore della §6 riscritto per la **sezione 4** della stella polare della GUI — lo spike di accettazione di `dockview`, otto mosse, la mossa 8 decisa dal proprietario — **scritta**, e **sfoltito** della cronaca delle riprese, che vive nella stella polare e, parola per parola, in [`archivio/stato-storico.md`](archivio/stato-storico.md), e alla chiusura per il **mandato del proprietario** sulla lettura d'apertura (decisione 26 della stella polare); poi, lo stesso giorno, il paragrafo dell'audit della §6 riscritto per il **taglio 2** del mandato, e il riquadro delle voci aperte ridotto agli **indici** per il **taglio 3**, e — alla tredicesima ripresa dello stesso giorno — il puntatore «Il prossimo passo» riscritto allo **stato di oggi** per il **taglio 4**, e il suo stato del mandato aggiornato alla **chiusura del mandato** (cinque tagli, `AVVIO-CHAT.md` com'è), e alla quattordicesima ripresa il punto 1 del prossimo passo passato alla **§9** del 2 con le §7 e §8 scritte, e alla chiusura le §9, §10 e la sezione 6 decise su delega e da scrivere, e alla quindicesima ripresa **scritte**, col prossimo passo passato al punto 2 — i due disegni sul posto, e alla sedicesima ripresa per i **due disegni scritti sul posto** — la stella polare e il disegno del 2 — con le consegne in archivio e le fonti in `riferimenti.md`; e alla rilettura del proprietario dello stesso giorno il punto 1 del prossimo passo è chiuso, sei voci sue tutte A (decisioni 41–47 della stella polare); l'ultimo contenuto di **merito** resta la sezione 4. Manutenzione: §13.
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
coding, voce e gesti, generazione asset 3D — su un **kernel comune**. Nessun pilastro prevale,
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

## 5. Le decisioni, una per ADR

Sono **39 ADR**, di cui **38 ADR in stato Accepted** e uno `Proposed` (0029).
Ordine numerico. Il *perché*, le alternative scartate e i costi accettati stanno nel
file di ciascuno: `docs/adr/`.

**0001 — Kernel con capacità paritarie.** Il sistema è un *kernel* con *capacità*.
Conversazione, conoscenza, agenti, coding, voce e generazione asset sono consumatori
**paritari** degli stessi servizi centrali: arbitro GPU, gateway di inferenza,
persistenza, permessi, bus eventi. **Nessuna capacità ha accesso privilegiato né
scorciatoie verso il kernel** — è questa regola a rendere la scelta reale invece che
dichiarativa. È la decisione fondativa: tutto il resto ne discende. ⚠️ **Rimando del 2026-09-03, in testa all'ADR:** «voce» si legge **«voce e gesti»**, i pilastri restano quattro, nessuna riga superata — ADR-0039.

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
condizionale è invisibile e diffusa. Default: **OpenRouter, VRAM libera**. ⚠️ **Rimando del 2026-09-08, in testa all'ADR:** il profilo dà il **default**, la policy **corrente** è la proiezione del giornale — l'ultima transizione scritta da `Arbiter::set_policy`; il daemon la rilegge all'avvio, compito del piano del 2 — decisione 17 della stella polare della GUI.

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
nel tritacarne del riassunto. ⚠️ **Rimando del 2026-09-05, in testa all'ADR:** la proiezione si costruisce nel sotto-progetto **13**, prima della prima capacità che la usa; la mappa della knowledge base entra **per chiave**, le foglie come riferimenti — disegno della knowledge base.

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
skill dichiarative di 0003 sono **guide**. ⚠️ **Rimando del 2026-09-05, in testa all'ADR:** registro delle guide e trigger si costruiscono nel sotto-progetto **13**, prima della prima capacità che inietta una guida; le due pretese della mappa — chiave di contesto (ambito, run, modello); provenienza e impronta, con «approvate ora» come proiezione del giornale — disegno della knowledge base.

**0010 — La proiezione ha un budget di qualità, non una soglia di riempimento.** Budget
target espresso come frazione della finestra, configurabile per modello. La
ricomposizione è **continua e proattiva**: serve a mantenere il budget, non a evitare
l'overflow. Il limite della finestra resta come **guardia**, non come politica. La
proiezione è **misurata per categoria** e la misura entra nel giornale — senza, «il
contesto è troppo pieno» è un'impressione e non un dato. ⚠️ **Rimando del 2026-09-05, in testa all'ADR:** la proiezione nasce nel sotto-progetto **13**; la mappa della knowledge base è una **categoria** del budget, per modello — disegno della knowledge base.

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
registra **anche per gli stream interrotti**. ⚠️ **Rimando del 2026-09-03, in testa all'ADR:** nell'inferenza percettiva entra il **tracciamento delle mani**; un gesto di comando apre un passo, i fotogrammi no — ADR-0039.

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
diventa una proprietà del kernel. ⚠️ **Rimando del 2026-09-08, in testa all'ADR:** la lista degli eventi è **aperta** — il codice deriva già il fallback dichiarato di ADR-0012, la telecamera arriva con ADR-0039 — e a GPU satura resta viva anche la **GUI** (ADR-0033); il diagramma vivo è design/07.

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
ogni operazione di I/O **iniettabile** — è quello che ha poi deciso ADR-0032. ⚠️ **Rimando del 2026-09-08, in testa all'ADR:** le **guide** sono file della cartella della knowledge base (disegno del 2026-09-04); la configurazione contiene i profili e, col 2, la **disposizione dei pannelli**, raggiunta dal kernel da una **settima porta** — stella polare della GUI, §2.

**0023 — Cifratura a riposo con chiavi dell'OS, e gestore dei segreti unico.** Le
chiavi le gestiscono le facility dell'OS, raggiunte dal modulo di piattaforma (I3); il
daemon parte senza interazione. **Onestà sulla forza reale:** qui «cifrato a riposo»
significa **protetto quanto il tuo account di sistema**, e va scritto **in interfaccia**
— una falsa sicurezza è peggio di nessuna sicurezza. **Il gestore dei segreti è
l'unico punto di lettura delle credenziali**, e da questo punto unico discendono tre
meccanismi già decisi: mascheratura nel record di routing, escalation automatica dei
vincoli sui dati, canary di esfiltrazione. **Profilo «riservato»** opzionale con
passphrase, che **disattiva avvio automatico e voce always-on** — mutuamente esclusivi,
e fingere il contrario sarebbe disonesto. ⚠️ **Rimando del 2026-09-03, in testa all'ADR:** il profilo «riservato» disattiva **anche la telecamera** — ADR-0039.

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

**0038 — Il registro delle funzioni del programma.** **Un registro unico, molti invocatori,
lo stesso permesso.** Il kernel dà registrazione, invocazione, il permesso come tripla di ADR-0016
e il giornale, nella forma dei registri di ADR-0009; le capacità e la GUI portano le funzioni.
Agente, gesto, voce e click passano dalla **stessa** porta con la **stessa** tripla, e nessuna
logica «solo per gesti» esiste. Un evento di percezione **informa, mai autorizza** (ADR-0014 per
analogia); un effetto irripetibile chiede conferma a qualunque invocatore, e **per default la
conferma non è gestuale**. La manipolazione della GUI — pannelli, menu — è presentazione e
**non passa dal registro**. ⛔ **Nessun codice nasce con l'ADR:** il registro lo costruisce il
primo invocatore, il click del sotto-progetto 2; quali funzioni siano gestuali lo decide il 12. ⚠️ **Rimando del 2026-09-05, in testa all'ADR:** la knowledge base registra le **CRUD** dei propri file e gruppi come funzioni del registro, spostamenti compresi; «aggiungi al contesto» ha **due invocatori**, il click e il modello — disegno della knowledge base.

**0039 — La telecamera come sorgente di percezione always-on sotto il core.** Un worker Python
**possiede** la telecamera e i fotogrammi **non escono mai**; al core arrivano **eventi** — lo
stato continuo della mano (21 punti, coordinate **intere**) e il gesto discreto (enum chiuso,
confidenza intera) — sul porto `process`, una `instruct_stream` poi `read_next` per tutta la
vita, `minicbor` (§6.10). **Eventi, non passi** (ADR-0011): il core smista la manipolazione alla
GUI con `Ipc::send`, **campionata** a una frequenza consegnata (ADR-0034); un gesto di **comando**
apre un passo nella run aperta, dal registro di ADR-0038. **Solo la wake word apre una run.**
Concessione da **zero MiB**, `Preemption::Never`, chiesta all'**accensione**: la telecamera è
**spenta per default**, e accenderla è una funzione del registro; **«riservato» la spegne**
(rimando ad ADR-0023); il campo di `Degradation` nasce col worker. La GUI **disegna la mano dai
21 punti**, niente video, e porta un **indicatore** acceso dal core. ⛔ **Il primo worker vero paga**
— trasporto di `process`, messaggio in giù, timbro di build, prontezza del reattore, ciclo di
lettura — e lo paga il sotto-progetto **12**; la Voce riusa. Tre ipotesi le misurano **SP-7** e
la sonda S3; il confinamento del worker (decisione 13) e la terza quota (decisione 9) restano
**registrati**. Le fonti F1–F9 in [`riferimenti.md`](riferimenti.md). ✅ **Rimando del 2026-09-05, nella riga del perimetro negativo:** la destinazione di una cattura è **decisa** — nella knowledge base come artefatto, la run la vede come riferimento (decisione 7 dei gesti, chiusa dal disegno della knowledge base).

---

## 6. Dove siamo, e cosa viene dopo

✅ **IL SECONDO AUDIT COMPLETO — 2026-08-27 — HA I SUOI FINDING TUTTI CHIUSI DAL 2026-08-28**,
[`audit-2026-08-27.md`](audit-2026-08-27.md): quanti, lo dice la colonna «Stato» di quel rapporto,
che ne è la **casa unica**, col comando in fondo a questa sezione (gotcha #68). ⛔ **Restano le voci
senza numero AUD**, la cui tabella in quel file è la casa unica, in gran parte **decisioni del
proprietario**. ✅ **RICHIAMO DEL 2026-09-09, decisione 26 della stella polare:** all'apertura si
leggono **solo** quella tabella e *«La disciplina, in cinque passi»*; i 73 e il «Dettaglio» si aprono
**una** scheda per volta. Il paragrafo com'era, col richiamo del 2026-08-28, è in
[`archivio/stato-storico.md`](archivio/stato-storico.md).

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
| 6 | gli altri meccanismi | ✅ 2026-09-02 |

⚠️ **Nessun numeratore di compiti in questa tabella**, per costruzione: invecchierebbe a
ogni compito. Il racconto di ciascun traguardo sta nel proprio piano, in
[`superpowers/plans/`](superpowers/plans/); i verbali di chiusura stanno in
[`archivio/stato-storico.md`](archivio/stato-storico.md).

### Il prossimo passo

⛔ **RICHIAMO DEL 2026-09-09 — il taglio 4 del mandato del proprietario (decisione 26 della stella polare della GUI),
approvato A alla tredicesima ripresa (decisione 30).** Questo puntatore era una catena di ✅ su cose chiuse, col racconto di
ciascuna; il testo com'era sta in [`archivio/stato-storico.md`](archivio/stato-storico.md), parola per parola. Qui resta lo
stato di oggi: è l'unico posto dove vive il prossimo passo, e più è corto meno invecchia.

**Chiuso, con la data e il posto del verbale:**

| Che cosa | Quando | Dove |
|---|---|---|
| il **sotto-progetto 1**, contro la §0.7 della spec | 2026-09-03 | la §7 del [disegno della chiusura](superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md) |
| il **riconoscimento gesti** — ADR-0038 e ADR-0039, SP-7 misurato, la sonda S3 nel cancello | 2026-09-04 | [disegno](superpowers/specs/2026-09-03-riconoscimento-gesti-design.md) e [piano](superpowers/plans/2026-09-03-riconoscimento-gesti.md); l'esito di SP-7 in `spikes/RISULTATI.md` |
| la **knowledge base** — brainstorming, disegno riletto dal proprietario, piano dei documenti eseguito: i rimandi in testa a ADR-0008, 0009, 0010 e 0038 e la riga 13 della roadmap. Il verdetto: nessuna sesta proprietà, ma un **vincolo d'ordine** — il **13** prima del 3 | 2026-09-05 | [disegno](superpowers/specs/2026-09-04-knowledge-base-design.md), che chi riprende quel fronte legge **per intero**, e [piano](superpowers/plans/2026-09-04-knowledge-base-documenti.md); la consegna in [`archivio/consegna-brainstorming-knowledge-base.md`](archivio/consegna-brainstorming-knowledge-base.md) |
| le **sezioni 1–4 della stella polare della GUI** — catalogo dei moduli, viste e disposizione con la **settima porta** del kernel, la fetta del 2, lo spike di accettazione di `dockview` — e la **passata sui diagrammi** (decisione 16), con la decisione 18 scritta in `CLAUDE.md` | 2026-09-09 | [stella polare](superpowers/specs/2026-09-07-direzione-gui-design.md), «Le sezioni approvate del disegno» e la tabella delle decisioni; la consegna dell'avvio del 2 in [`archivio/consegna-avvio-brainstorming-sottoprogetto-2.md`](archivio/consegna-avvio-brainstorming-sottoprogetto-2.md) |
| i **due disegni della GUI, scritti sul posto** — la [stella polare](superpowers/specs/2026-09-07-direzione-gui-design.md), sezioni 1–4 e 6, e il [disegno del 2](superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md), §1–§10, con le §1, §2 e §6a riscritte; le due consegne in archivio parola per parola, le fonti in `riferimenti.md` | 2026-09-09 | la **§10 del disegno del 2**, «Come si riprende»; gli archivi [`consegna-brainstorming-direzione-gui.md`](archivio/consegna-brainstorming-direzione-gui.md) e [`consegna-brainstorming-sottoprogetto-2.md`](archivio/consegna-brainstorming-sottoprogetto-2.md) |
| la **rilettura del proprietario** dei due disegni — B alla domanda minima, poi le sei voci sue una per volta, tutte A: decisioni 41–47 | 2026-09-09 | la tabella delle decisioni della [stella polare](superpowers/specs/2026-09-07-direzione-gui-design.md); la §10 del [disegno del 2](superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md); X-1 e X-3 nell'[audit](audit-2026-08-27.md) |

✅ **Il mandato del proprietario del 2026-09-09 — sfoltire la lettura d'apertura (decisione 26 della stella polare) — è
ESEGUITO alla tredicesima ripresa:** cinque tagli, tutti A — la stella polare, la testa dell'audit, il riquadro delle voci
aperte di questa §6, questo puntatore, `CLAUDE.md` — e `AVVIO-CHAT.md` resta com'è, perché il proprietario non lo incolla
più (decisione 32). La misura la rifanno i comandi nel prossimo passo della stella polare; il metodo — misurare prima, ogni
taglio in A/B, niente si cancella, i puntatori in una casa sola — vale per ogni sfoltimento futuro.

⏭️ **IL PROSSIMO PASSO: IL PIANO DEL SOTTO-PROGETTO 2, IN DUE PARTI — LA RILETTURA DEL PROPRIETARIO È FATTA IL 2026-09-09.** I due disegni sono
**scritti sul posto il 2026-09-09** (decisione 39 della stella polare): la
[stella polare della GUI](superpowers/specs/2026-09-07-direzione-gui-design.md) — viste, moduli, disposizione, il protocollo
core ↔ GUI — e il [disegno del 2](superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md), le cui §1–§10 sono approvate.
Si riprende dalla **§10 del disegno del 2**, «Come si riprende», che dà l'ordine coi comandi; i due disegni si leggono **per intero**:

1. ✅ la **rilettura del proprietario** dei due disegni in questa forma, in chat — il sì è condizionato e non si deduce; la
   domanda minima con cui si apre sta nella §10 — **fatta il 2026-09-09**, voce per voce: le sei voci sue tutte A, decisioni 41–47 della stella polare;
2. ⏭️ **la prossima sessione — decisione 48 del 2026-09-09:** `superpowers:writing-plans` col piano del 2 in **due parti**, la prima fino allo spike compreso — guscio più accettazione di
   `dockview` — la seconda scritta dopo la misura; il pre-controllo delle quattro domande di `CLAUDE.md` nella sessione che
   scrive il piano; l'esecuzione, subagent-driven, in sessioni nuove;
   ⏳ **RICHIAMO DEL 2026-09-09, alla chiusura della seconda sessione del piano:** il piano della parte 1 ha **tutti gli otto compiti scritti** — pre-controllo, decisioni, compiti 1–8, «Dopo il compito 8» — in [`superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md`](superpowers/plans/2026-09-09-sottoprogetto-2-parte-1-spike-del-guscio.md); **manca la revisione del piano intero**, coi tre spostamenti di stato: li fa la sessione **nuova** dalla sua sezione «Come si riprende», **prima** di eseguirlo;
3. ADR-0029 si chiude con M1–M5 all'inizio del 2 (voce 0029 della §5) — **e in parallelo AUD-004**, l'ADR del proprietario
   che sbarra il **13**, non il 2 (voce 3 della rilettura del disegno della knowledge base); poi il 13, poi il 3 — decisione 16.

⚠️ La cronaca ripresa per ripresa non sta qui: vive nella stella polare e, parola per parola, negli archivi. Il margine del
tetto di questo file lo dà il comando nella tabella dello stato della stella polare (gotcha #100).

✅ **I finding dell'audit sono chiusi**, e a dirlo non è questa riga ma il comando:

```
awk -F'|' 'NF>4{gsub(/^ +| +$/,"",$5); print $5}' docs/audit-2026-08-27.md | grep -c aperto
```

---

### Le voci ancora aperte, e dove vivono

⛔ **RICHIAMO DEL 2026-09-09 — il taglio 3 del mandato del proprietario (decisione 26 della stella polare
della GUI), approvato A.** Questa sottosezione si intitolava *«Le voci ancora aperte, e il racconto che le
circonda»* e portava, **parola per parola**, il racconto di ogni voce aperta dei Traguardi 3–5 — il debito
dichiarato dello sfoltimento del 2026-08-28. Ogni voce è stata censita **una per una** contro gli indici che
esistono già, e il racconto sta in [`archivio/stato-storico.md`](archivio/stato-storico.md), parola per parola;
qui restano gli **indici**. ⚠️ **Due voci vivevano SOLO nel racconto**, e sono le ultime due righe della tabella
qui sotto.

📌 **Dove vivono le voci aperte.** Tre indici, e per i due del registro il comando che elenca le righe **non**
chiuse — un elenco di nomi invecchierebbe alla prima che si chiude:

| Indice | Che cosa tiene |
|---|---|
| [`porta-di-qualita.md`](porta-di-qualita.md), *«Le voci aperte del Traguardo 5, in una tabella sola»* | le voci del Traguardo 5, con la colonna «chi la chiude» — ⚠️ alcune sono già **chiuse** dal Traguardo 6 e lo dicono nella **terza** colonna, non qui: il comando le salta |
| lo stesso file, *«Le voci aperte del Traguardo 6, in una tabella sola»* | le voci del Traguardo 6, e chi le chiude — **non sempre il proprietario**, e lo dice la quinta colonna |
| [`audit-2026-08-27.md`](audit-2026-08-27.md), *«Le voci aperte che NON hanno un numero AUD»* | X-1…X-4, in gran parte del proprietario: si legge all'apertura, lo prescrive `CLAUDE.md` |
| la tabella qui sotto | le voci dei Traguardi 3 e 4, dei documenti e del modo di lavorare |

```
awk -F'|' '/^## .*LE VOCI APERTE DEL TRAGUARDO 5/{s=1} s&&/^## Cosa la porta NON controlla/{s=0} s&&/^\| [0-9]+ \|/&&$4!~/CHIUSA/{print $2": "$3}' docs/porta-di-qualita.md
awk -F'|' '/^## .*LE VOCI APERTE DEL TRAGUARDO 6/{s=1} s&&/^\| +[0-9]+ \| +le voci \*\*ereditate/{s=0} s&&/^\| [0-9]+ \|/&&$4!~/CHIUSA/{print $2": "$3}' docs/porta-di-qualita.md
```

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
| ⚠️ **una regola di rimisura scatta anche su un tocco che non muove nessuna cifra?** Il Task 9 del Traguardo 5 lasciò la regola *«il primo compito che tocca `crates/kernel/tests/arbiter_admission.rs` rimisura tutte le celle della campagna»*; il Task 10 lo toccò — righe di doc, nessuna sonda — e nessuna cella fu rimisurata. **Registrata nel racconto del Task 10 e non presa**; fino al 2026-09-09 viveva solo in quel racconto | il racconto del Task 10, in [`archivio/stato-storico.md`](archivio/stato-storico.md) dal 2026-09-09 | il **proprietario**: è il modo di lavorare, non il prodotto |
| ⚠️ **i pesi scritti a mano sopravvivono nel messaggio di [`AVVIO-CHAT.md`](AVVIO-CHAT.md)**, mentre la §12 dal 2026-08-28 li dà col comando: toglierli anche di là, lasciando il comando, chiuderebbe una classe di rilievi del ciclo di revisione del Task 11. **Registrata nel racconto del Task 11 (richiamo del 2026-08-28) e non presa**; fino al 2026-09-09 viveva solo in quel racconto | il racconto del Task 11, in [`archivio/stato-storico.md`](archivio/stato-storico.md) dal 2026-09-09, e il verbale delle misure in [`archivio/misure-dimensioni.md`](archivio/misure-dimensioni.md) | il **proprietario**: tocca il documento d'ingresso. ✅ **Chiusa il 2026-09-09, decisione 32 della stella polare della GUI:** il proprietario non incolla più il messaggio, che resta com'è e non è più lettura d'apertura |

📌 **Il ritratto pieno si riconta OGNI VOLTA, e le cifre non stanno qui** — il 2026-09-02 erano
già stantie in entrambe le tabelle. Il comando, che è il blocco **A** della §1.3 del
[disegno della chiusura](superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md):

```bash
awk '/^## 8\. /{i=1} i && /^\|[[:space:]]*[VQ][0-9]+[[:space:]]*\|/ {r=$0; gsub(/\\\|/,"",r); split(r,c,"|"); id=c[2]; gsub(/ /,"",id); k=(c[4]~/verificato qui/)?"ok":(c[4]~/parziale/)?"parziale":(c[4]~/rimandato/)?"rimandato":"altro"; n[substr(id,1,1)" "k]++} END{for (x in n) print x, n[x]}' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
```

---

📚 **Gli stati passati** — i verbali dei traguardi chiusi, dei due audit, delle voci già chiuse e, dal
2026-09-09, il **racconto** delle voci ancora aperte — stanno in [`archivio/stato-storico.md`](archivio/stato-storico.md).
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
| ❌ **ri-derivare l'architettura** | è nei 39 ADR, ciascuno con alternative scartate e motivo |
| ❌ **riscrivere `tracciabilita.md` da zero** | le funzionalità sono già mappate, e **quante** lo dice il comando nel riquadro in testa a [`tracciabilita.md`](tracciabilita.md): si **aggiorna** — riletta alla chiusura del sotto-progetto 1 il 2026-09-03, e si riaggiorna a ogni sotto-progetto chiuso |
| ❌ **ri-cercare lo stato dell'arte già tracciato** | è in `riferimenti.md` con le fonti. Verificane semmai l'invecchiamento |
| ❌ **rifare gli spike SP-5, SP-6 e SP-7** | esiti, versioni e comandi in `spikes/RISULTATI.md` — coi **seed** per SP-5 e SP-6, che SP-7 non ha; per SP-7 il protocollo congelato in `spikes/gesti/PROTOCOLLO.md` |
| ❌ **rifare le misure da M-1 a M-11** | tutte chiuse, con comandi, versioni e sonde. M-9 sta per intero in ADR-0036, **M-10 e M-11 in ADR-0037**. L'unica aperta è **M5** (senza trattino), e richiede una GUI |
| ❌ **riaprire le due decisioni della §7.3** | prese dopo aver misurato. Riaprirle richiede una misura nuova, non un'opinione |
| ❌ **riaprire la copertura della §8** | la §8 è **spec**, e il vincolo globale 1 del piano della chiusura vieta di toccarla: le righe si leggono, non si ri-giudicano. ⚠️ **RICHIAMO DEL 2026-09-03: il sotto-progetto 1 l'ha riaperta una volta**, e per decisione del **proprietario** — la via **A**, voce `E10` dell'errata del [piano della chiusura](superpowers/plans/2026-09-02-sottoprogetto-1-chiusura.md) — col vincolo **sospeso** per il solo compito 3bis e per le sole §8.3 e §8.4. Riaprirla di nuovo richiede la stessa decisione |
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
| ⛔ **il perimetro del Traguardo 6** — gli altri meccanismi, le forme, il controllo per artefatto, e la **§8** col verbale della sua chiusura | [`specs/2026-08-28-…-traguardo-6-altri-meccanismi-design.md`](superpowers/specs/2026-08-28-sottoprogetto-1-traguardo-6-altri-meccanismi-design.md) |
| ⛔ **come si è ESEGUITO il Traguardo 6** — dieci compiti in cinque parti, con l'errata in testa | [`plans/2026-08-30-…-traguardo-6-altri-meccanismi.md`](superpowers/plans/2026-08-30-sottoprogetto-1-traguardo-6-altri-meccanismi.md) — ⚠️ **a compiti, mai intero** |
| ⛔ **come si è CHIUSO il sotto-progetto 1** — le condizioni della §0.7 rilette contro il codice, e la **§7** col verbale | [`specs/2026-09-02-…-chiusura-design.md`](superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md) |
| il piano della chiusura, con l'errata in testa e la tabella della posizione | [`plans/2026-09-02-…-chiusura.md`](superpowers/plans/2026-09-02-sottoprogetto-1-chiusura.md) |
| ⛔ **il perimetro del RICONOSCIMENTO GESTI** — la forma della telecamera nel kernel, il registro delle funzioni, le decisioni col loro chiusore, e per ogni artefatto il controllo che lo esercita | [`specs/2026-09-03-riconoscimento-gesti-design.md`](superpowers/specs/2026-09-03-riconoscimento-gesti-design.md) — ⚠️ **non è una spec** |
| come si è **eseguito** il riconoscimento gesti — i due ADR, i rimandi, la roadmap, SP-7 e la sonda S3, con l'errata in testa e la tabella della posizione | [`plans/2026-09-03-riconoscimento-gesti.md`](superpowers/plans/2026-09-03-riconoscimento-gesti.md) — ⚠️ **a compiti, mai intero** |
| ⛔ **il perimetro della KNOWLEDGE BASE** — che cosa la mappa chiede al kernel e dove va: la strada B, i tre meccanismi del sotto-progetto 13 con le due pretese, le CRUD nel registro delle funzioni, il pannello col 6, le decisioni col loro chiusore, e per ogni artefatto il controllo che lo esercita | [`specs/2026-09-04-knowledge-base-design.md`](superpowers/specs/2026-09-04-knowledge-base-design.md) — ⚠️ **non è una spec**, e **non disegna la capacità** |
| come si è **eseguito** il piano dei documenti della knowledge base — i rimandi in testa a quattro ADR e nella riga di ADR-0039, la riga 13 in roadmap, le righe di tracciabilità, la decisione 7 dei gesti chiusa, con l'errata in testa e la tabella della posizione | [`plans/2026-09-04-knowledge-base-documenti.md`](superpowers/plans/2026-09-04-knowledge-base-documenti.md) — ⚠️ **a compiti, mai intero** |
| ⛔ **il perimetro del Traguardo 4** — quanto ne costruisce, dove vive ciascun pezzo, e per ogni artefatto **il controllo che lo esercita**. Si legge **prima** di scriverne il piano | [`specs/2026-08-11-…-traguardo-4-simulatore-dst-design.md`](superpowers/specs/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst-design.md) — ⚠️ **non è una spec**: è lo scaglionamento che la §3 non fissa |
| il **cosa** del kernel: §0–§10 | [`specs/2026-08-06-kernel-design.md`](superpowers/specs/2026-08-06-kernel-design.md) |
| il testo integrale dei **gotcha** e delle **misure**, con i numeri | [`HANDOFF.md`](HANDOFF.md) — ⚠️ **a sezioni** |
| ⛔ **cosa una sezione deve incassare, prima di proporle una modifica** | [`HANDOFF.md`](HANDOFF.md) — il **consuntivo voce per voce**: cosa era stato deciso, dove è finito, e cosa resta da scrivere. È **autorevole**, e si legge **prima** di proporre, non dopo. ⚠️ **La sezione, non il file** |
| l'ordine dei sotto-progetti e le dipendenze — quanti siano lo dice la tabella di quel file | [`roadmap.md`](roadmap.md) |
| dove vive una funzionalità della mappa originale | [`tracciabilita.md`](tracciabilita.md) — ⚠️ **leggi il riquadro in testa**: risponde a «dove vive», **non** a «di quale meccanismo ha bisogno». È la crepa da cui sono uscite le sette voci |
| **dove vive ogni controllo** della porta, riga per riga sul catalogo §7.4, e cosa **non** è coperto | [`porta-di-qualita.md`](porta-di-qualita.md) |
| ⛔ **perché un seme NON è un oracolo**, e cosa identifica un caso in ciascuna delle due campagne DST — al livello 2 *«un seme»* **non esiste** | [`semi-dst.md`](semi-dst.md) — ⚠️ **nasce vuoto**, e la riga vuota è deliberata; ⛔ **è CRLF integrale**, misurato il 2026-08-25 |
| la **strategia di test** — è la fonte di verità sulla porta di qualità, e mappa Q1–Q24 → metodo | [`design/08-strategia-di-test.md`](design/08-strategia-di-test.md) |
| la **topologia dei processi** — contiene la tensione che F1b deve conciliare | [`design/01-topologia-dei-processi.md`](design/01-topologia-dei-processi.md) |
| gli altri diagrammi della struttura | [`design/`](design/) — quanti lo dice `ls docs/design/` |
| gli **esiti degli spike**, con seed, versioni e comandi | [`../spikes/RISULTATI.md`](../spikes/RISULTATI.md) |
| i requisiti della GUI, G1–G21 e P1–P4 | [`../spikes/GUI-REQUISITI.md`](../spikes/GUI-REQUISITI.md) |
| la **provenienza** di ciò che non abbiamo dedotto noi, con le date | [`riferimenti.md`](riferimenti.md) |
| il **modello** di come si scrive un piano qui, con l'errata in testa | [`plans/2026-08-06-spike-linguaggio-del-core.md`](superpowers/plans/2026-08-06-spike-linguaggio-del-core.md) |
| ⛔ **cosa il piano del Traguardo 1 detta e il repository smentisce** — quattro voci, prima fra tutte gli identificatori italiani | [`plans/2026-08-08-sottoprogetto-1-traguardo-1-scheletro-e-porta.md`](superpowers/plans/2026-08-08-sottoprogetto-1-traguardo-1-scheletro-e-porta.md) — ⚠️ **solo l'errata in testa**, il resto è eseguito |
| ⛔ **come si esegue un piano qui, e le quattro specie di difetto** — è il piano del Traguardo 2, **eseguito per intero**, con quarantanove voci di errata in sei passate | [`plans/2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md`](superpowers/plans/2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md) — ⚠️ **a compiti, mai intero**. ⛔ **RICHIAMO DEL 2026-08-28, finding AUD-035:** la cella diceva *«è il secondo file più grande del repository, dopo la spec»*, ed è **tolta e non riallineata** — era **ottavo** quando il finding lo misurò il 2026-08-27 e **decimo** un giorno dopo, e un ordinamento marcisce come una cifra. Lo rifà il comando sotto questa tabella |
| ⛔ **come si esegue un piano, e come si CHIUDE un traguardo** — è il piano del Traguardo 3, **eseguito per intero**, dodici compiti su dodici. ⚠️ **L'errata in testa si legge prima del compito**, ed è a **settantasette voci in nove passate**, di cui **nove decisioni**; le ultime tre sono la **Definizione di «fatto» che invecchia** | [`plans/2026-08-10-sottoprogetto-1-traguardo-3-giornale-e-formato-durevole.md`](superpowers/plans/2026-08-10-sottoprogetto-1-traguardo-3-giornale-e-formato-durevole.md) — ⚠️ **a compiti, mai intero** |
| ⛔ **come si esegue un piano quando il pre-controllo trova un difetto in DIECI compiti su dieci** — è il piano del Traguardo 4, **eseguito per intero**. ⚠️ **L'errata in testa è a settanta voci in nove passate, di cui dodici DECISIONI**, e si legge **prima** di riaprire qualunque cosa che quel traguardo abbia toccato | [`plans/2026-08-11-…-traguardo-4-simulatore-dst.md`](superpowers/plans/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst.md) — ⚠️ **a compiti, mai intero** |
| l'indice di ADR e diagrammi | [`README.md`](README.md) |
| ⛔ **il messaggio da incollare all'inizio di una chat**, e il perché di ogni sua riga | [`AVVIO-CHAT.md`](AVVIO-CHAT.md) — ⚠️ **il peso del messaggio lo dà il comando sotto questa tabella**, non questa cella: ⛔ **RICHIAMO DEL 2026-08-28** — diceva *«**20606 byte LF** su **303** righe»*, ed è invecchiato lo stesso giorno, quando la riga 3 del messaggio ha smesso di dire che l'audit era il compito di oggi. Ciò che **resta** qui è il **metodo** — le righe **fra le due recinzioni, escluse** — senza il quale due lettori onesti ottengono due numeri (59ª misura). ⚠️ **Dal 2026-09-09 il proprietario non lo incolla più** (decisione 32 della stella polare della GUI): il file resta com'è, e non è lettura d'apertura |

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
