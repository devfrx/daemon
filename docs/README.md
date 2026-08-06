# Documentazione di progetto

Assistente desktop locale — piattaforma a quattro pilastri (conversazione, conoscenza,
agenti/coding, voce, generazione asset) su kernel comune.

**Fase corrente: progettazione del kernel. Nessun codice scritto.**

## Dove va cosa

| Cartella | Contiene | Risponde alla domanda |
|---|---|---|
| `adr/` | Architecture Decision Records | *perché* abbiamo deciso così |
| `design/` | Diagrammi Mermaid della struttura | *com'è fatto* il sistema |
| `superpowers/specs/` | Specifiche dei sotto-progetti | *cosa* costruiamo, prima di costruirlo |

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
| [0005](adr/0005-arbitrato-gpu-su-due-dimensioni.md) | Arbitrato GPU su due dimensioni, quota audio sottratta | Proposed |
| [0006](adr/0006-due-policy-vram-come-oggetti-distinti.md) | Due policy VRAM come oggetti distinti | Proposed |

## Indice dei diagrammi

| Diagramma | Descrive |
|---|---|
| [Topologia dei processi](design/01-topologia-dei-processi.md) | Classi di processo, proprietà dello stato, canali |
| [Arbitrato delle risorse GPU](design/02-arbitrato-gpu.md) | Dimensioni della risorsa, ciclo di vita della concessione, corsie |

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
