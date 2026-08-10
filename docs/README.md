# Documentazione di progetto

Assistente desktop locale — piattaforma a quattro pilastri (conversazione, conoscenza,
agenti/coding, voce, generazione asset) su kernel comune.

**Fase corrente: implementazione del kernel — Traguardi 1 e 2 ✅ eseguiti, il 2026-08-08 e il
2026-08-10.** Il codice del prodotto vive in [`../crates/`](../crates/) — cinque crate,
`kernel` e `simulator` in `no_std` — e la porta di qualità gira con un comando solo,
`bash scripts/gate.sh`. ⏭️ **Il prossimo passo è eseguire il piano del Traguardo 3**, scritto
il 2026-08-10: giornale e formato durevole.

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
| [Sotto-progetto 1](superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md) | Implementazione del kernel + simulatore DST | §0–§8 approvate, riapertura su sette voci ✅ **tutta chiusa** (F3, F6, F5, F1a, **F2 con F7**, **F1b**, **F4**), e **§8 riallineata e chiusa** il 2026-08-08, poi **audit sezione-contro-ADR** passato. ✅ **Spec completa**, e il [piano del Traguardo 1](superpowers/plans/2026-08-08-sottoprogetto-1-traguardo-1-scheletro-e-porta.md) è **eseguito**. Anche il [piano del Traguardo 2](superpowers/plans/2026-08-09-sottoprogetto-1-traguardo-2-substrato-iniettabile.md) è **eseguito** il 2026-08-10, **per intero**: quattordici compiti su quattordici, fra il 2026-08-09 e il 2026-08-10, con le **sei famiglie di porte complete**. ⏭️ Il [piano del **Traguardo 3**](superpowers/plans/2026-08-10-sottoprogetto-1-traguardo-3-giornale-e-formato-durevole.md) è **scritto** il 2026-08-10 — dodici compiti — e **da eseguire** |

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
