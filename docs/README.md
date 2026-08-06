# Documentazione di progetto

Assistente desktop locale — piattaforma a quattro pilastri (conversazione, conoscenza,
agenti/coding, voce, generazione asset) su kernel comune.

**Fase corrente: progettazione del kernel. Nessun codice scritto.**

## Dove va cosa

| Percorso | Contiene | Risponde alla domanda |
|---|---|---|
| `adr/` | Architecture Decision Records | *perché* abbiamo deciso così |
| `design/` | Diagrammi Mermaid della struttura | *com'è fatto* il sistema |
| `superpowers/specs/` | Specifiche dei sotto-progetti | *cosa* costruiamo, prima di costruirlo |
| [`riferimenti.md`](riferimenti.md) | Fonti esterne consultate | *da dove viene* ciò che non abbiamo dedotto noi |

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
| [Strategia di test](design/08-strategia-di-test.md) | I due strati, le quattro tecniche, mappa Q1–Q20 → metodo |

## Specifiche

| Spec | Sotto-progetto | Stato |
|---|---|---|
| [Kernel](superpowers/specs/2026-08-06-kernel-design.md) | L0 fondamenta + L1 arbitri trasversali | In costruzione |

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
