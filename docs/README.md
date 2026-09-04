# Documentazione di progetto

Assistente desktop locale — piattaforma a **quattro pilastri paritari** (conversazione e
conoscenza, agenti e coding, voce, generazione asset) su kernel comune. ⚠️ **Questa riga ne
elencava cinque dicendo «quattro»**: il raggruppamento è quello della §1 del compendio.

**Fase corrente: il sotto-progetto 1 è ✅ chiuso il 2026-09-03 contro la §0.7 della sua spec; il
prossimo passo lo dice la §6 del [compendio](COMPENDIO.md).** ⚠️ **La coda di date che la «Fase
corrente» portava è TOLTA e non riallineata:** diceva *«il 2026-08-08 e il 2026-08-10»* per
quattro traguardi, ed era già sbagliata a sé stante —
**questo stesso file** data il Traguardo 4 al **2026-08-11**, nel capoverso che ne annuncia il
brainstorming. ⚠️ **RICHIAMO DEL 2026-08-25: qui c'era un NUMERO DI RIGA — *«la riga 16»* — e lo
stesso commit che lo scriveva aveva inserito sei righe sopra di sé, spostando il bersaglio.**
**Tolto e sostituito con un'àncora testuale:** un numero di riga dentro un documento vivo è un
puntatore senza guardia, e lo stesso giorno la stessa cura ha tolto i **sette** rimandi per numero
di riga della colonna *«Dove è dichiarata»* di [`porta-di-qualita.md`](porta-di-qualita.md), più la
riga che li introduceva. Un elenco di date
si allunga a ogni chiusura e non c'è nessun numero da riconoscere come stantio: le date, una
per traguardo, stanno nella tabella dei traguardi di [`roadmap.md`](roadmap.md) e nella §6 del
[`COMPENDIO.md`](COMPENDIO.md), e un rimando non può marcire.
Il codice del prodotto vive in [`../crates/`](../crates/) — cinque crate,
`kernel` e `simulator` in `no_std` — e la porta di qualità gira con un comando solo,
`bash scripts/gate.sh`. Il **Traguardo 3** — giornale e formato durevole — è chiuso con
**dodici compiti su dodici**: il record come enum di versione, **due** implementazioni della
porta `journal` tenute alle stesse nove promesse a ogni commit, la riconciliazione. ⛔ **Col
Task 10 il formato durevole è CONGELATO:** i byte del record sono nel repository e non si
rigenerano mai. ✅ **Il brainstorming del Traguardo 4 è chiuso il 2026-08-11** e il disegno è
scritto: [Traguardo 4 — il disegno](superpowers/specs/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst-design.md).
✅ **E il Traguardo 4 è ESEGUITO lo stesso giorno**, dieci compiti su dieci, `GATE GREEN` a
ciascuno: il simulatore porta ora **il guasto** — il giornale che cade, il backend cadente di
`redb` scritto **da fuori la crate**, e due campagne con **soggetti diversi**, la riconciliazione
del kernel e la coerenza del motore. ⛔ Il disegno aveva ristretto il traguardo su una misura —
delle dieci righe di guasto della §3.3 **una sola** ha oggi il proprio soggetto — quindi si è
costruito **il motore** della DST e non tutte le finte. ⛔ **E lo stesso giorno il repository ha
ricevuto il suo primo AUDIT COMPLETO** — [`audit-2026-08-11.md`](audit-2026-08-11.md): sedici
finding corretti e provati, il resto aperto. ✅ **La prima delle otto decisioni è eseguita il
2026-08-17** — la conformità del giornale provava **tre** promesse solo su un archivio vuoto o a
**un passo solo**; chiuse con un passante, senza promesse nuove. ✅ **E l'ottava il 2026-08-18** —
G-5: `--locked` su tutti i siti `cargo` del cancello, così il `Cargo.lock` **tracciato** smette di
essere un **effetto** del cancello e ne diventa un **ingresso**. ✅ **E la sesta lo stesso
giorno** — A-1, A-2, A-4, A-7: quattro richiami datati su ADR e diagrammi, **nessuna decisione
riaperta**, perché a cadere sono quattro **evidenze** e non quattro scelte. ✅ **E la quinta** —
C-1: `bincode` è dichiarato **non mantenuto** (RUSTSEC-2025-0141, `INFO`), ha **zero** usi di
produzione, e la registrazione vive accanto alla voce nel manifesto; si decide al Traguardo 6.
✅ **E la quarta** — PL-1: il file del giornale nasceva **0644** su Linux, cioè **meno**
dell'account che ADR-0023 promette; ora è **0600**, misurato su Linux vero.
✅ **E LE OTTO SONO CHIUSE il 2026-08-18, otto su otto.**
✅ **E lo stesso giorno il brainstorming del Traguardo 5 è chiuso, e il disegno è scritto:**
[Traguardo 5 — il disegno](superpowers/specs/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu-design.md).
Il perimetro è **l'arbitro intero**, e si prova ciò che si può provare senza un secondo
meccanismo; `Grant` **si sposta** nel modulo dell'arbitro perché un modulo fratello non può
costruirlo — `E0423`, misurato — e il traguardo **non crea righe di catalogo: ne tocca dodici**
già scritte. ⚠️ **Questa riga diceva *«ne chiude dodici»***, e a chiusura il conteggio ricontato
sulla §7.4 dice **undici chiuse e una dichiarata PARZIALE** — la riga della campagna DST, che
resta parziale perché la §5.7 elenca cinque proprietà e la campagna dell'arbitro ne tiene tre.
Le altre due si iniettano su `process` e `ipc`, cioè al **Traguardo 6**.
⛔ **RICHIAMO DEL 2026-09-02: quella riga NON è più parziale, ed è il compito 9 del Traguardo 6 a
chiuderla** — `E152`. Le due proprietà mancanti hanno la propria sonda: la **2** in
`crates/simulator/tests/worker_kill_campaign.rs` e la **3** in
`crates/simulator/tests/gui_death_campaign.rs`, che riconcilia con `kernel::client::ClientGrants`
alla disconnessione. ⚠️ **Il periodo qui sopra resta perché è un verbale del 2026-08-18 e in
quella data era vero**; ciò che è **falso adesso** è soltanto il presente *«resta parziale»*, e
lo stato di oggi **non vive qui** — si legge nella §6 del compendio, e la riga di catalogo che
questo periodo cita si legge in [`porta-di-qualita.md`](porta-di-qualita.md).
✅ **E lo stesso giorno il [piano del Traguardo 5](superpowers/plans/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu.md)
è scritto: tredici compiti in cinque parti.** Il pre-controllo del piano ha trovato **sette**
cose leggendo il disegno contro il codice — gotcha **#58** — e tre sono per il proprietario:
`WorkDescriptor` dista **una lettera** da `WorkerDescriptor`, che esiste già ed è un'altra cosa;
la riga di catalogo `Q2 · §5.1` è **una** dove le regole sono **due**; e due celle del catalogo
nominano **identificatori italiani** che il traguardo fa esistere in inglese.
✅ **E IL TRAGUARDO 5 È ESEGUITO IL 2026-08-25, tredici compiti su tredici**, subagent-driven,
`GATE GREEN` a ogni compito, dal 2026-08-19. ⛔ **Il tredicesimo era un AUDIT e non una
scrittura**, e ciò che ha trovato è che gran parte era già eseguita: i riconteggi non hanno
prodotto uno scarto con i documenti, e a mancare erano le **voci aperte in una tabella sola** —
ora in [`porta-di-qualita.md`](porta-di-qualita.md), **con la colonna di chi le chiude**.
⛔ **A CHE PUNTO FOSSE NON ERA SCRITTO QUI, ed era il rimedio e non una svista:** questa riga portava il
numeratore — *«cinque compiti su tredici»* — **due righe sopra** la frase che dice *«qui non si
ripetono, ed è la ragione per cui questa riga non è mai marcita»*. Era il gotcha **#68**, una
regola che non vincola il documento che la ospita, e la cifra è **tolta invece che ricorretta**.
⛔ **RICHIAMO DEL 2026-08-20 — E CON LA CIFRA ERA RIMASTO L'ELENCO, che è la stessa cosa scritta a
parole.** Questa riga enumerava ciò che l'esecuzione aveva portato — *«il modulo `arbiter` con
`Mib` … e le code per corsia»* — e la lista si **fermava al Task 6** con otto compiti chiusi:
un numeratore in prosa invecchia come uno in cifre, e per giunta non si vede. **Tolto, non
esteso**: la tabella dei compiti sta nella §6, in un posto solo.
⚠️ **Quante e quali**, le cose da sapere prima di riprendere e le decisioni di disegno che
aspettano il proprietario stanno nella **§6 del [`COMPENDIO.md`](COMPENDIO.md)**.
⏭️ **Il prossimo passo sta nella §6 del [`COMPENDIO.md`](COMPENDIO.md), in un posto solo** — qui
non si ripete. ⚠️ **Questa riga lo riscriveva**, e ha detto *«scrivere il piano»*, poi *«il
brainstorming»*, poi *«eseguirlo»*, poi *«l'esecuzione dell'audit … ne restano tre»*: quattro
formulazioni, l'ultima falsa dal 2026-08-18. **Tolta invece che corretta** — 25ª misura.
⚠️ **Questa riga diceva «dieci compiti su dodici, si riprende dal Task 11»** a traguardo chiuso:
è la lezione della tredicesima misura — la §6 del compendio giusta **nasconde** gli altri
documenti di stato.

⛔ **Non c'è ancora nessuna funzionalità utente, ed è deliberato.** Il Traguardo 1 non
portava nemmeno logica: solo lo scheletro e i controlli, perché un cancello costruito **dopo**
la logica è un cancello che nessuno ha mai visto fallire. Il Traguardo 2 ha portato il
**substrato** — tempo, casualità, scheduling, l'esecutore, le **sei famiglie di porte** — che
sono **meccanismi**, non funzionalità: il kernel non implementa niente per l'utente, fornisce
ciò su cui le capacità poggeranno. Vale «spec prima del codice», e ⛔ **il codice è in
inglese, la documentazione in italiano** (§1.0 della spec del sotto-progetto 1).

> Se stai riprendendo il progetto, le letture obbligatorie sono **due**, e sono
> [`../CLAUDE.md`](../CLAUDE.md) e [`COMPENDIO.md`](COMPENDIO.md). ⛔ **Non** `HANDOFF.md`,
> che si apre a sezioni e quando serve il testo integrale di un gotcha o di una misura.

## Dove va cosa

| Percorso | Contiene | Risponde alla domanda |
|---|---|---|
| [`COMPENDIO.md`](COMPENDIO.md) | ⛔ **l'unica lettura obbligatoria oltre a `CLAUDE.md`**: tutte le decisioni compresse, le invarianti, lo stack, i gotcha, lo stato di oggi e il prossimo passo | *cosa è già deciso*, tutto, in un colpo solo |
| [`audit-2026-08-11.md`](audit-2026-08-11.md) | ⛔ **il primo audit completo del repository** — codice, script, documenti, ADR, diagrammi. Copertura dichiarata, le quattro radici, i finding con causa radice e dimostrazione, cosa è **pulito**, e le otto decisioni del proprietario. ✅ **CHIUSO il 2026-08-18, otto su otto.** ⚠️ **Questa cella lo dava APERTO e «il prossimo passo»**, con *«la 1 … la 8 e la 6»* e *«le tre righe barrate»* quando barrate erano otto: era ferma al 2026-08-18 e la riga qui sopra la smentiva **nello stesso file** — finding **AUD-001** del 2026-08-27. ⛔ **Il prossimo passo non si scrive più qui**, in nessuna forma: sta nella **§6 del [`COMPENDIO.md`](COMPENDIO.md)**, in un posto solo | *cosa fu trovato l'11 agosto*, e come si conduce un audit qui — **consultazione**, non arretrato |
| [`audit-2026-08-27.md`](audit-2026-08-27.md) | ⛔ **il secondo audit completo** — 18 revisori in parallelo, ognuno smentito da un secondo, 98 finding proposti e **25 scartati**. Le **sette radici**, i **73** finding con causa radice e comando di riproduzione, e lo **stato di ciascuno**. ✅ **CHIUSO nei suoi 73 finding il 2026-08-28**, e restano aperte le sole **voci senza numero AUD**, in gran parte decisioni del proprietario. ⚠️ **Si legge a finding, mai intero** | *come si rimedia* qui, e *cosa resta aperto* |
| [`AVVIO-CHAT.md`](AVVIO-CHAT.md) | il messaggio da incollare all'inizio di una nuova sessione | *come si apre* una chat su questo repository |
| [`HANDOFF.md`](HANDOFF.md) | Gotcha, non rilitigabile, metodo, cosa non rifare — ⚠️ **a sezioni**, non per farsi un'idea | *come riprendere* senza rifare |
| [`roadmap.md`](roadmap.md) | Sotto-progetti, ordine, stato, spike aperti | *a che punto siamo* e *cosa viene dopo* |
| [`tracciabilita.md`](tracciabilita.md) | Mappa funzionale → sede di ogni funzionalità | *dove vive* ciò che è stato chiesto |
| [`porta-di-qualita.md`](porta-di-qualita.md) | Dove vive ogni controllo della porta, mappato riga per riga sul catalogo §7.4. Un comando solo: `bash scripts/gate.sh` | *cosa è sorvegliato*, da quale file, e con quali sonde |
| `adr/` | Architecture Decision Records | *perché* abbiamo deciso così |
| `design/` | Diagrammi Mermaid della struttura | *com'è fatto* il sistema |
| `superpowers/specs/` | Specifiche dei sotto-progetti | *cosa* costruiamo, prima di costruirlo |
| ⛔ [`superpowers/plans/`](superpowers/plans/) | i piani, uno per traguardo, ciascuno con l'**errata in testa** che dice dove il piano sbagliava. ⚠️ **Mancava da questa tabella**, aggiunto il 2026-08-10: è la cartella **da cui si riprende il lavoro** | *da dove si riparte*, e cosa il piano ha già sbagliato |
| [`riferimenti.md`](riferimenti.md) | Fonti esterne consultate | *da dove viene* ciò che non abbiamo dedotto noi |

`roadmap.md` e `tracciabilita.md` si aggiornano **alla chiusura di ogni
sotto-progetto**, nello stesso passaggio.

## Regole della documentazione

1. Gli ADR sono **append-only**. Una decisione superata non si cancella: si marca
   `Superseded by ADR-XXXX` e se ne scrive una nuova.
2. I diagrammi in `design/` descrivono lo stato **corrente**, mai la storia. Si
   aggiornano nello stesso task che cambia il sistema, mai "dopo".
3. Nessun sotto-progetto si implementa senza spec approvata.

## Indice delle decisioni

| ADR | Decisione | Status |
|---|---|---|
| [0001](adr/0001-architettura-a-kernel-con-capacita-paritarie.md) | Architettura a kernel con capacità paritarie | Accepted |
| [0002](adr/0002-windows-primario-con-confine-os-esplicito.md) | Windows primario, confine OS esplicito | Accepted |
| [0003](adr/0003-estensibilita-solo-mcp-e-skill-dichiarative.md) | Estensibilità solo via MCP e skill dichiarative | Accepted |
| [0004](adr/0004-topologia-di-processo.md) | Topologia di processo: core, gui, worker | Accepted |
| [0005](adr/0005-arbitrato-gpu-su-due-dimensioni.md) | Arbitrato GPU su due dimensioni, quota audio sottratta | Accepted |
| [0006](adr/0006-due-policy-vram-come-oggetti-distinti.md) | Due policy VRAM come oggetti distinti | Accepted |
| [0007](adr/0007-giornale-write-ahead-e-riconciliazione.md) | Giornale write-ahead delle run e riconciliazione alla ripresa | Accepted |
| [0008](adr/0008-contesto-come-proiezione-dello-stato.md) | Il contesto è una proiezione, non lo stato | Accepted |
| [0009](adr/0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md) | Guide, sensori e anelli di controllo sono meccanismi di kernel | Accepted |
| [0010](adr/0010-budget-della-proiezione-invece-di-soglia-di-riempimento.md) | Budget della proiezione invece di soglia di riempimento | Accepted |
| [0011](adr/0011-routing-risolto-e-giornalato-per-richiesta.md) | Routing risolto e giornalato per ogni richiesta | Accepted |
| [0012](adr/0012-equivalenza-del-fallback-e-fallimento-chiuso.md) | Equivalenza del fallback dai vincoli; sui dati si fallisce chiuso | Accepted |
| [0013](adr/0013-conformita-allo-schema-e-un-verdetto-di-sensore.md) | La conformità allo schema è un verdetto di sensore | Accepted |
| [0014](adr/0014-confine-dei-dati-non-fidati-nel-sistema-di-tipi.md) | Il confine dei dati non fidati vive nel sistema di tipi | Accepted |
| [0015](adr/0015-descrizioni-degli-strumenti-fissate-all-approvazione.md) | Descrizioni degli strumenti fissate all'approvazione | Accepted |
| [0016](adr/0016-permessi-granulari-e-default-dei-vincoli-sui-dati.md) | Permessi come tripla, default dei vincoli sui dati per profilo | Accepted |
| [0017](adr/0017-giornale-sorgente-trace-proiezione.md) | Il giornale è la sorgente, il trace è una proiezione | Accepted |
| [0018](adr/0018-ritenzione-a-livelli-del-giornale.md) | Ritenzione a livelli: la struttura sopravvive, i payload si potano | Accepted |
| [0019](adr/0019-lo-stato-di-degrado-e-un-oggetto-osservabile.md) | Lo stato di degrado è un oggetto osservabile | Accepted |
| [0020](adr/0020-nessun-modello-nel-percorso-decisionale-del-kernel.md) | Nessun modello nel percorso decisionale del kernel | Accepted |
| [0021](adr/0021-simulazione-deterministica-e-iniettabilita.md) | Simulazione deterministica, iniettabilità di costruzione | Accepted |
| [0022](adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md) | Layout dei dati per natura, backup del solo irriproducibile | Accepted |
| [0023](adr/0023-cifratura-a-riposo-e-gestore-dei-segreti.md) | Cifratura con chiavi dell'OS, gestore dei segreti unico | Accepted |
| [0024](adr/0024-checkpoint-del-filesystem-ad-ambiti-dichiarati.md) | Checkpoint del filesystem ad ambiti dichiarati | Accepted |
| [0025](adr/0025-confinamento-a-livelli.md) | Confinamento a livelli: il kernel richiede, la piattaforma implementa | Accepted |
| [0026](adr/0026-linguaggio-del-core.md) | Linguaggio del core: Rust | Accepted |
| [0027](adr/0027-stack-della-gui.md) | La GUI è un'interfaccia web, non un toolkit nativo | Accepted |
| [0028](adr/0028-ecosistema-dei-worker-ml.md) | Ecosistema dei worker ML: Python, ratificato | Accepted |
| [0029](adr/0029-guscio-della-gui.md) | Guscio della GUI: Tauri o Electron | ⚠️ **Proposed** |
| [0030](adr/0030-framework-dell-interfaccia.md) | Framework dell'interfaccia: Vue 3 | Accepted |
| [0031](adr/0031-dipendenze-del-kernel-parte-del-confine.md) | Le dipendenze del kernel sono parte del confine I3 | Accepted |
| [0032](adr/0032-motore-di-persistenza.md) | Motore di persistenza: `redb`, con backend nostro | Accepted |
| [0033](adr/0033-gpu-della-gui-quota-di-presentazione.md) | GPU della GUI: quota di presentazione sottratta, concessione tenuta dal core | Accepted |
| [0034](adr/0034-parametri-di-decisione-consegnati-non-letti.md) | I parametri di decisione sono consegnati al kernel, non letti | Accepted |
| [0035](adr/0035-porta-verso-i-worker-e-lettura-di-i4.md) | La porta verso i worker, e cosa significa «singolo» in I4 | Accepted |
| [0036](adr/0036-evoluzione-del-formato-durevole-del-giornale.md) | L'evoluzione del formato durevole del giornale | Accepted |
| [0037](adr/0037-criterio-del-pari-per-il-formato-dei-canali.md) | Il criterio del pari: il formato di un canale privato si sceglie anche sull'ecosistema di chi lo legge | Accepted |
| [0038](adr/0038-registro-delle-funzioni-del-programma.md) | Il registro delle funzioni del programma: un registro, molti invocatori, lo stesso permesso | Accepted |
| [0039](adr/0039-telecamera-come-sorgente-di-percezione.md) | La telecamera come sorgente di percezione always-on sotto il core | Accepted |

## Indice dei diagrammi

| Diagramma | Descrive |
|---|---|
| [Topologia dei processi](design/01-topologia-dei-processi.md) | Classi di processo, proprietà dello stato, canali |
| [Arbitrato delle risorse GPU](design/02-arbitrato-gpu.md) | Dimensioni della risorsa, ciclo di vita della concessione, corsie |
| [Run durevoli e proiezione](design/03-run-durevoli.md) | Livelli dello stato, ciclo di vita del passo, riconciliazione |
| [Anelli, guide e sensori](design/04-anelli-e-sensori.md) | I quattro anelli, feedforward vs feedback, budget della proiezione |
| [Gateway di inferenza](design/05-gateway-inferenza.md) | Risoluzione di una richiesta, catena di riserva, contabilità |
| [Permessi e confine dei dati](design/06-permessi-e-confine-dei-dati.md) | I due canali, ereditarietà dell'etichetta, permessi, canary |
| [Osservabilità e degrado](design/07-osservabilita-e-degrado.md) | Tassonomia degli errori, stato di degrado, proiezioni del giornale |
| [Strategia di test](design/08-strategia-di-test.md) | I due strati, le quattro tecniche, mappa Q1–Q24 → metodo |
| [L0 fisico](design/09-l0-fisico.md) | Archivi, chiavi e segreti, checkpoint, livelli di confinamento |

## Specifiche

| Spec | Sotto-progetto | Stato |
|---|---|---|
| [Kernel](superpowers/specs/2026-08-06-kernel-design.md) | L0 fondamenta + L1 arbitri trasversali | ✅ **completa e approvata** |
| [Sotto-progetto 1](superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md) | Implementazione del kernel + simulatore DST | §0–§8 approvate, riapertura su sette voci ✅ **tutta chiusa** (F3, F6, F5, F1a, **F2 con F7**, **F1b**, **F4**), e **§8 riallineata e chiusa** il 2026-08-08, poi **audit sezione-contro-ADR** passato. ✅ **Spec completa**, e il [piano del Traguardo 1](superpowers/plans/2026-08-08-sottoprogetto-1-traguardo-1-scheletro-e-porta.md) è **eseguito**. Anche il [piano del Traguardo 2](superpowers/plans/2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md) è **eseguito** il 2026-08-10, **per intero**: quattordici compiti su quattordici, fra il 2026-08-09 e il 2026-08-10, con le **sei famiglie di porte complete**. ✅ Il [piano del **Traguardo 3**](superpowers/plans/2026-08-10-sottoprogetto-1-traguardo-3-giornale-e-formato-durevole.md) è **scritto** il 2026-08-10 ed **eseguito** lo stesso giorno: **dodici compiti su dodici**, `GATE GREEN` a tutti. ⚠️ **Questa cella diceva «otto compiti su dodici, si riprende dal Task 9»** a traguardo chiuso. ✅ Col **Task 7** il kernel scrive il **primo record vero**: la porta guadagna `note()`, il record `RecordKind::Note` e il campo `reason`, e la via **A4** del confine dei dati non fidati si chiude a **livello 2**. ✅ Col **Task 8** nasce la **seconda implementazione** della porta `journal` — `redb` col **backend scritto da noi** in `platform` — con la chiave **progressiva della scrittura** e la prova, scritta da fuori la crate, che il confine dello `StorageBackend` è **davvero sostituibile**. ✅ Col **Task 10** i **byte congelati** — tre record e una mappa che il banco rilegge — e col **Task 11** `prune`, che rifiuta un passo **in dubbio** e accetta uno riconciliato. ✅ Del **Traguardo 4** brainstorming, disegno e piano sono tutti chiusi il 2026-08-11, **ed è ESEGUITO lo stesso giorno: dieci compiti su dieci**, `GATE GREEN` a ciascuno. ⚠️ **Questa cella ha detto «si deve ancora fare il brainstorming», poi «manca il piano», poi «resta da eseguire» — tre volte sbagliata, tutte e tre a traguardo più avanti di quanto dicesse**, e la terza contraddiceva la riga 16 dello stesso file. La terza l'ha trovata l'audit del 2026-08-11; l'avviso che questa cella già portava non è bastato a farla rileggere |
| [Traguardo 4 — il disegno](superpowers/specs/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst-design.md) | il simulatore DST | ⛔ **Non è una spec:** è lo **scaglionamento** che la §3 della spec del sotto-progetto 1 deliberatamente non fissa — perimetro, dove vive ciascun pezzo, e per ogni artefatto **il controllo che lo esercita**. ✅ **Scritto il 2026-08-11**, e il piano lo traduce in compiti |
| [Traguardo 5 — il disegno](superpowers/specs/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu-design.md) | l'arbitro GPU | ⛔ **Non è una spec:** è lo scaglionamento e le **forme** che la §5 descrive a parole — dove vive l'arbitro, dove vive `Grant` e perché si sposta, i parametri consegnati, il ciclo della concessione, le due policy, e per ogni artefatto **il controllo che lo esercita**. ✅ **Scritto il 2026-08-18**, ⛔ e **si legge PRIMA di scriverne il piano** — cosa che il piano dello stesso giorno ha fatto, trovandovi **sette** cose |
| [Traguardo 6 — il disegno](superpowers/specs/2026-08-28-sottoprogetto-1-traguardo-6-altri-meccanismi-design.md) | gli altri meccanismi | ⛔ **Non è una spec:** perimetro, forme e il controllo che esercita ciascun artefatto; la **§8** è il verbale della chiusura del traguardo |
| [La chiusura — il disegno](superpowers/specs/2026-09-02-sottoprogetto-1-chiusura-design.md) | la chiusura del sotto-progetto 1 | ⛔ **Non è una spec:** come si rilegge la §0.7 contro il codice; la **§7** è il verbale |
| [Riconoscimento gesti — il disegno](superpowers/specs/2026-09-03-riconoscimento-gesti-design.md) | il riconoscimento gesti dalla telecamera | ⛔ **Non è una spec:** perimetro, forme e il controllo che esercita ciascun artefatto; le decisioni col loro chiusore; la §6.4 porta l'esito di SP-7 |

## Decomposizione del sistema

| Livello | Blocco | Dipende da |
|---|---|---|
| **L0** | Fondamenta — processi, persistenza, configurazione, segreti, tracing, bus eventi, packaging | — |
| **L1** | Arbitro risorse GPU · Gateway di inferenza | L0 |
| **L2** | Conversazione · Conoscenza/RAG · Agenti · Coding · Voce · Generazione asset | L0, L1 |
| **L3** | Integrazione OS — hotkey, tray, notifiche, daemon, offline, i18n, a11y | L0 |
| **XX** | Sicurezza — **non è un livello**: è un vincolo che entra nel design di L0, L1 e L2 dal primo giorno | — |

Le dipendenze sono rigide verso il basso: nessuna capacità di L2 si progetta prima
che L0 e L1 siano stabili, perché tutte e sei negoziano con l'arbitro GPU e con il
gateway di inferenza.
