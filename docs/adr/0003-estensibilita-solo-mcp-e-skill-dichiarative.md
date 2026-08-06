# ADR-0003: Estensibilità solo tramite MCP e skill dichiarative

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

La mappa funzionale elencava quattro meccanismi di estensione distinti — MCP, skill,
sistema di plugin, marketplace di estensioni — con costi radicalmente diversi. La
domanda che li separa è una sola, ed è la più pesante del progetto sul piano della
sicurezza: **codice di terze parti può girare dentro il processo dell'applicazione?**

Il contesto aggrava la domanda: il sistema esegue già codice generato da un LLM e
custodisce chiavi API. Un plugin difettoso o malevolo caricato nel processo
principale avrebbe accesso diretto a entrambi.

Alternative considerate:

- **Plugin caricati nel processo principale:** massima potenza, minima latenza.
  *Contro:* un plugin può leggere le chiavi API, corrompere lo stato o far cadere
  l'intera applicazione.
- **Plugin nativi isolati in processi separati:** potenza alta, rischio contenuto.
  *Contro:* impone un contratto pubblico versionato da mantenere per sempre, più
  l'intero ciclo di vita dei processi figli.

## Decision

**Nessun codice di terze parti gira nel processo dell'applicazione.** Esistono
esattamente due meccanismi di estensione:

| Meccanismo | Cos'è | Isolamento |
|---|---|---|
| **Server MCP** | processo esterno che espone strumenti e dati | processo proprio, permessi propri, revocabile |
| **Skill dichiarativa** | istruzioni e dati, **non** codice eseguibile | nessuna esecuzione, quindi nessun isolamento necessario |

Le due coprono la sostanza di ciò che un marketplace di plugin prometterebbe, senza
il costo del contratto pubblico e senza la superficie d'attacco.

## Consequences

- **Positive:**
  - Il protocollo interno del kernel resta **privato**: nessun consumatore esterno,
    quindi nessun contratto da congelare né da versionare pubblicamente. È questa la
    conseguenza che rende economica la topologia a processi di [ADR-0004](0004-topologia-di-processo.md).
  - Superficie d'attacco minima; nessun caricamento dinamico di codice.
  - Il kernel resta libero di evolvere senza rotture verso l'esterno.
- **Negative (accettate):**
  - Le estensioni **non possono aggiungere interfaccia grafica** né nuovi tipi di job
    GPU. Se un giorno servisse, sarà un ADR nuovo, non uno scivolamento.
  - Ogni strumento esterno paga il costo di un salto di processo.
- **Follow-up richiesti:**
  - I server MCP sono di terze parti: le loro **descrizioni degli strumenti sono
    contenuto non fidato** e vanno trattate come tali (tool poisoning). Vedi
    invariante 6 in [ADR-0004](0004-topologia-di-processo.md).
