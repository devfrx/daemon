# ADR-0019: Lo stato di degrado è un oggetto osservabile, non una collezione di errori

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

Il sistema ha molte condizioni in cui funziona **parzialmente**: rete assente, GPU
satura, provider indisponibile, modello locale scaricato, permesso non concesso,
strumento MCP sospeso.

Il trattamento consueto è per azione: ogni tentativo fallisce con il proprio errore.
Dal punto di vista dell'utente questo produce il modo di fallire più frustrante che
esista — **si scopre cosa non funziona un tentativo alla volta**, e il quadro
complessivo non è mai visibile. Con quattro pilastri e una GPU condivisa, le
condizioni parziali non sono l'eccezione: sono lo stato normale in metà delle
giornate.

Alternative considerate:

- **Errori per azione.** Nessuna infrastruttura. *Contro:* nessun quadro d'insieme;
  l'utente prova, fallisce, prova un'altra cosa, fallisce ancora.
- **Stato di degrado esplicito**, mantenuto dal core e osservabile prima di agire.
  *Contro:* un oggetto in più da mantenere aggiornato e coerente.

## Decision

**Il core mantiene uno stato di degrado corrente**, aggiornato dagli eventi
(connettività, arbitro GPU, salute dei provider, permessi, strumenti sospesi), e lo
espone come oggetto osservabile.

**Il principio: si dichiara prima, non si fallisce dopo.** L'utente deve poter sapere
cosa è disponibile *prima* di tentare, non scoprirlo per tentativi.

| Condizione | Resta disponibile | Cade |
|---|---|---|
| **offline** | inferenza locale, RAG locale, generazione asset, voce | OpenRouter, ricerca web |
| **GPU satura** | tutto ciò che è remoto; voce (quota riservata, §2) | inferenza locale, generazione asset |
| **provider indisponibile** | fallback della catena; locale se configurato | il provider stesso |
| **modello locale scaricato** | tutto, con avvio a freddo dichiarato (Q8) | latenza del primo token |
| **strumento MCP sospeso** | tutto il resto | quello strumento, finché non ri-approvato (§6) |

Questo generalizza a tutto il sistema il «nessun degrado silenzioso» già deciso per
l'arbitro GPU ([ADR-0005](0005-arbitrato-gpu-su-due-dimensioni.md)): era una regola
locale, diventa una proprietà del kernel.

## Consequences

- **Positive:**
  - L'utente vede il quadro invece di scoprirlo per tentativi.
  - Le capacità possono consultare lo stato e adattarsi — per esempio offrire il
    dirottamento su remoto durante un render, invece di limitarsi ad attendere.
  - Il degrado diventa **misurabile**: quanto tempo il sistema passa in stato
    parziale è un dato, non un'impressione.
- **Negative (accettate):**
  - Un oggetto di stato in più, che può divergere dalla realtà se un evento va perso.
    Mitigazione: lo stato è **derivato**, ricalcolabile, mai autorevole di per sé.
  - Rischio di allarmismo: mostrare troppi stati parziali rende l'interfaccia
    ansiogena. Va mostrato ciò che **cambia cosa l'utente può fare**, non ogni
    variazione interna.
- **Follow-up richiesti:**
  - La rappresentazione in interfaccia è materia della GUI, ma il vincolo è di
    kernel: nessuna azione deve fallire per una condizione che era **già nota** e non
    era stata dichiarata.
