# Documentazione di progetto

Questo file è l'**indice** della documentazione. Il ritratto del progetto sta in [`../CLAUDE.md`](../CLAUDE.md),
*«Cos'è questo progetto, in quattro righe»*; **lo stato per sotto-progetto** nella tabella dei sotto-progetti di
[`roadmap.md`](roadmap.md), e quello per traguardo nella tabella dei traguardi dello stesso file; **il prossimo passo**
nella §6 del [compendio](COMPENDIO.md). Qui non si ripetono: due copie divergono (gotcha #68).
⚠️ **RICHIAMO DEL 2026-10-06** — audit del 2026-09-30, AUD-121, AUD-579.

> Se stai riprendendo il progetto, le letture obbligatorie sono **due**, e sono
> [`../CLAUDE.md`](../CLAUDE.md) e [`COMPENDIO.md`](COMPENDIO.md). ⛔ **Non** `HANDOFF.md`,
> che si apre a sezioni e quando serve il testo integrale di un gotcha o di una misura.

## Dove va cosa

| Percorso | Contiene | Risponde alla domanda |
|---|---|---|
| [`COMPENDIO.md`](COMPENDIO.md) | ⛔ **l'unica lettura obbligatoria oltre a `CLAUDE.md`**: tutte le decisioni compresse, le invarianti, lo stack, i gotcha, lo stato di oggi e il prossimo passo | *cosa è già deciso*, tutto, in un colpo solo |
| [`audit-2026-08-11.md`](audit-2026-08-11.md) | ⛔ **il primo audit completo del repository** — codice, script, documenti, ADR, diagrammi. Copertura dichiarata, le quattro radici, i finding con causa radice e dimostrazione, cosa è **pulito**, e le otto decisioni del proprietario. ✅ **Le otto decisioni sono eseguite il 2026-08-18**; le voci della §5 rimaste aperte stanno nella sua **§9**, con chi le chiude — ⚠️ **RICHIAMO DEL 2026-10-03**, audit del 2026-09-30, AUD-084. ⚠️ **Questa cella lo dava APERTO e «il prossimo passo»**, con *«la 1 … la 8 e la 6»* e *«le tre righe barrate»* quando barrate erano otto: era ferma al 2026-08-18 e la riga qui sopra la smentiva **nello stesso file** — finding **AUD-001** del 2026-08-27. ⛔ **Il prossimo passo non si scrive più qui**, in nessuna forma: sta nella **§6 del [`COMPENDIO.md`](COMPENDIO.md)**, in un posto solo | *cosa fu trovato l'11 agosto*, e come si conduce un audit qui — **consultazione**, non arretrato |
| [`audit-2026-08-27.md`](audit-2026-08-27.md) | ⛔ **il secondo audit completo** — 18 revisori in parallelo, ognuno smentito da un secondo, 98 finding proposti e **25 scartati**. Le **sette radici**, i **73** finding con causa radice e comando di riproduzione, e lo **stato di ciascuno**, nella colonna «Stato», che ne è la casa unica; aperte restano anche le **voci senza numero AUD**, in gran parte decisioni del proprietario — ⚠️ **RICHIAMO DEL 2026-10-03**, audit del 2026-09-30, AUD-602. ⚠️ **Si legge a finding, mai intero** | *come si rimedia* qui, e *cosa resta aperto* |
| [`audit-2026-09-30.md`](audit-2026-09-30.md) | ⛔ **il terzo audit completo** — codice e documentazione, da testa a piedi, con la skill `repo-audit`: le nove radici, i trenta pacchetti di correzione con la loro riverifica, e la tabella che risolve gli `AUD-NNN` citati dai documenti, che da AUD-001 ad AUD-073 hanno la stessa forma di quelli del secondo audit. Aperte restano *«Le scelte aperte»*, al proprietario una alla volta; dove si è arrivati e il prossimo passo, la §6 del [compendio](COMPENDIO.md). ⚠️ **Si legge a sezioni, mai intero** | *che cosa è stato corretto, da quale pacchetto e con quale riverifica*, e *che cosa aspetta il proprietario* |
| [`AVVIO-CHAT.md`](AVVIO-CHAT.md) | il messaggio d'avvio di una sessione, e il perché di ogni sua riga: dal 2026-09-09 il proprietario non lo incolla più, e non è lettura d'apertura — la §12 del [compendio](COMPENDIO.md), decisione 32 della [stella polare](superpowers/specs/2026-09-07-direzione-gui-design.md); come si apre una sessione lo dice [`../CLAUDE.md`](../CLAUDE.md). ⚠️ **RICHIAMO DEL 2026-10-07** — audit del 2026-09-30, AUD-107 | *perché* il messaggio d'avvio dice ciò che dice |
| [`HANDOFF.md`](HANDOFF.md) | Gotcha, non rilitigabile, metodo, cosa non rifare — ⚠️ **a sezioni**, non per farsi un'idea | *come riprendere* senza rifare |
| [`roadmap.md`](roadmap.md) | Sotto-progetti, ordine, stato, spike aperti | *a che punto siamo* e *cosa viene dopo* |
| [`tracciabilita.md`](tracciabilita.md) | Mappa funzionale → sede di ogni funzionalità | *dove vive* ciò che è stato chiesto |
| [`porta-di-qualita.md`](porta-di-qualita.md) | Dove vive ogni controllo della porta, mappato riga per riga sul catalogo §7.4. Un comando solo: `bash scripts/gate.sh` | *cosa è sorvegliato*, da quale file, e con quali sonde |
| [`semi-dst.md`](semi-dst.md) | I casi delle due campagne DST, ciascuno col test permanente della propria proprietà. ⚠️ **Non ha un chiudente**: nessuno script verifica che una voce nomini un test che esiste — ⚠️ **RICHIAMO DEL 2026-10-03**: mancava da questa tabella, audit del 2026-09-30, AUD-602 | *perché un seme non è un oracolo*, e come si identifica un caso in ciascuna campagna |
| `adr/` | Architecture Decision Records | *perché* abbiamo deciso così |
| `design/` | Diagrammi Mermaid della struttura | *com'è fatto* il sistema |
| `superpowers/specs/` | Specifiche dei sotto-progetti | *cosa* costruiamo, prima di costruirlo |
| ⛔ [`superpowers/plans/`](superpowers/plans/) | i piani, uno per traguardo, ciascuno con l'**errata in testa** che dice dove il piano sbagliava. ⚠️ **Mancava da questa tabella**, aggiunto il 2026-08-10: è la cartella **da cui si riprende il lavoro** | *da dove si riparte*, e cosa il piano ha già sbagliato |
| [`riferimenti.md`](riferimenti.md) | Fonti esterne consultate | *da dove viene* ciò che non abbiamo dedotto noi |

Che cosa si aggiorna, e quando, lo dice la tabella *«Manutenzione della documentazione»* di
[`../CLAUDE.md`](../CLAUDE.md), in una casa sola. ⚠️ **RICHIAMO DEL 2026-10-07** — audit del
2026-09-30, AUD-108.

## Regole della documentazione

1. Gli ADR sono **append-only**. Una decisione superata non si cancella: si marca
   `Superseded by ADR-XXXX` e se ne scrive una nuova.
2. I diagrammi in `design/` descrivono lo stato **corrente**, mai la storia. Si
   aggiornano nello stesso task che cambia il sistema, mai "dopo". Ciò che è **deciso e non
   costruito** vi sta col segno **«(col N)»**: N è il sotto-progetto della [roadmap](roadmap.md)
   che lo costruisce, o il primo che ne avrà bisogno quando la roadmap non lo fissa —
   «(col primo …)» —; se chi lo costruisce è una scelta aperta, il segno porta i candidati e
   il rimando alla scelta, mai un numero solo. Un pezzo senza segno esiste nel codice, e «oggi»
   lo dice dove sta accanto a uno deciso. Il sotto-progetto N, quando costruisce, **toglie il
   proprio «(col N)»** nello stesso task; in [design/10](design/10-modello-dei-dati-durevoli.md)
   l'entità passa dal secondo diagramma al primo. ⚠️ **RICHIAMO DEL 2026-10-02** — audit del
   2026-09-30, AUD-119 e AUD-122: il segno è la regola di tutti i file di `design/`, ed è nato
   coi diagrammi del 2026-09-08, decisioni 16–18 della
   [stella polare](superpowers/specs/2026-09-07-direzione-gui-design.md).
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
| [0029](adr/0029-guscio-della-gui.md) | Guscio della GUI: Electron, deciso con SP-8 | Accepted |
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
| [0040](adr/0040-dove-vivono-i-dati-e-che-cosa-salva-il-programma.md) | Dove vivono i dati, e che cosa salva il programma — modifica in parte ADR-0022 | Accepted |
| [0041](adr/0041-chi-puo-parlare-col-core.md) | Chi può parlare col core — il solo account che lo esegue | Accepted |

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
| [Modello dei dati durevoli](design/10-modello-dei-dati-durevoli.md) | Il giornale a entità, ciò che è deciso per sotto-progetto, le due verità sul record |

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
| [Knowledge base — il disegno](superpowers/specs/2026-09-04-knowledge-base-design.md) | la knowledge base: che cosa chiede al kernel, e dove va | ⛔ **Non è una spec**, e **non disegna la capacità**: perimetro, la forma nel kernel, i rimandi in append, la GUI e il sotto-progetto 6 in due metà, le voci aperte col loro chiusore, e per ogni artefatto il controllo che lo esercita; il verdetto — nessuna sesta proprietà «che non si aggiunge dopo», ma un vincolo d'ordine: il sotto-progetto 13 prima del 3 |
| [Stella polare della GUI](superpowers/specs/2026-09-07-direzione-gui-design.md) | la direzione della GUI | ⛔ **Non è una spec:** viste, moduli, disposizione, il protocollo core ↔ GUI, le decisioni col loro chiusore |
| [Sotto-progetto 2 — il disegno](superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md) | GUI minima | ⛔ **Non è una spec:** perimetro, filo, schema, registro, core finto, prove e cancello. ✅ **RICHIAMO DEL 2026-09-22:** la parte 2 del piano è **eseguita** — la Definizione di «fatto» del [piano della parte 2](superpowers/plans/2026-09-11-sottoprogetto-2-parte-2-gui-minima.md), coi comandi. ⚠️ **RICHIAMO DEL 2026-10-06** — audit del 2026-09-30, AUD-658: la §10 è il verbale della chiusura, e il prossimo passo sta nella §6 del [compendio](COMPENDIO.md) |
| [Design system — il disegno](superpowers/specs/2026-09-22-design-system-design.md) | il design system della GUI, il sotto-progetto 14 | ⛔ **Non è una spec:** il linguaggio visivo; i token e i due temi, il kit, il dock, la cornice, le voci registrate e le sonde che diventano test — le sezioni (a)–(f); per ogni artefatto il controllo che lo esercita, e le voci del proprietario ✅ **RICHIAMO DEL 2026-09-28:** il piano è **eseguito** — la Definizione di «fatto» del [piano](superpowers/plans/2026-09-23-design-system.md), coi comandi |

## Decomposizione del sistema

| Livello | Blocco | Dipende da |
|---|---|---|
| **L0** | Fondamenta — processi, persistenza, configurazione, segreti, tracing, bus eventi | — |
| **L1** | Arbitro risorse GPU · Gateway di inferenza | L0 |
| **L2** | Conversazione · Conoscenza/RAG · Agenti · Coding · Voce · Generazione asset | L0, L1 |
| **L3** | Integrazione OS — hotkey, tray, notifiche, daemon, offline, i18n, a11y, packaging | L0 |
| **XX** | Sicurezza — **non è un livello**: è un vincolo che entra nel design di L0, L1 e L2 dal primo giorno | — |

Le dipendenze sono rigide verso il basso: tutte e sei le capacità di L2 negoziano con
l'arbitro GPU e con il gateway di inferenza. Quando una capacità si progetta lo dice la §8
del [compendio](COMPENDIO.md), riga *«progettare una capacità L2»*, e l'ordine dei
sotto-progetti la [roadmap](roadmap.md). Il packaging sta in L3, come in ADR-0028, ADR-0032 e
nella roadmap. ⚠️ **RICHIAMO DEL 2026-10-06** — audit del 2026-09-30, AUD-120, AUD-565.
