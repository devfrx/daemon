# ADR-0015: Le descrizioni degli strumenti sono fissate all'approvazione

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

[ADR-0003](0003-estensibilita-solo-mcp-e-skill-dichiarative.md) ammette server MCP di
terze parti. Le descrizioni dei loro strumenti **devono** raggiungere il modello: è
il loro scopo: senza, l'agente non sa cosa lo strumento faccia né quando usarlo.

Questo le rende l'**unica eccezione strutturale** a
[ADR-0014](0014-confine-dei-dati-non-fidati-nel-sistema-di-tipi.md): contenuto scritto
da terzi che per funzionare deve entrare nel canale che influenza il comportamento.

I due attacchi rilevanti sono documentati:

| Attacco | Meccanica |
|---|---|
| **tool poisoning** | istruzioni malevole nascoste nei metadati dello strumento |
| **rug pull** | descrizione innocua all'installazione, cambiata dopo l'approvazione |

Il secondo è il più insidioso: sconfigge qualsiasi difesa basata sulla revisione
iniziale, perché la revisione avviene su un testo che poi non è più quello in uso.

Alternative considerate:

- **Trattare le descrizioni come dati non fidati puri.** Coerente con ADR-0014.
  *Contro:* rende gli strumenti inutilizzabili — il modello non saprebbe cosa fanno.
- **Fidarsi dopo l'approvazione iniziale.** *Contro:* ignora completamente il rug pull.

## Decision

**1. La descrizione è mostrata integralmente all'utente all'approvazione** — non solo
il nome dello strumento, ma il testo che influenzerà il modello.

**2. La descrizione approvata è fissata:** se ne registra l'impronta.

**3. Se cambia, lo strumento è sospeso** finché non viene ri-approvato, con il *diff*
mostrato. Non degradato, non avvisato-e-usato: **sospeso**.

**4. Nella proiezione le descrizioni sono marcate come dichiarate da terzi.**
Informano su *cosa lo strumento fa*, non su *cosa l'agente deve fare*.

**5. Una descrizione non concede permessi.** I permessi vengono esclusivamente da
[ADR-0016](0016-permessi-granulari-e-default-dei-vincoli-sui-dati.md). Un metadato che
si auto-attribuisce privilegi è ignorato: è testo, non autorità.

## Consequences

- **Positive:**
  - Il rug pull diventa **rilevabile per costruzione**, non per vigilanza.
  - L'utente valuta il testo reale che entrerà nel contesto, non un'etichetta.
  - Il punto 5 chiude la strada più diretta al tool poisoning: anche se il testo è
    malevolo, non può ampliare ciò che lo strumento può toccare.
- **Negative (accettate):**
  - **Aggiornare un server MCP legittimo richiede ri-approvazione.** È fastidioso, ed
    è esattamente la stessa proprietà che blocca l'attacco: non si può avere l'una
    senza l'altra.
  - **L'approvazione iniziale resta il punto debole.** Se l'utente approva senza
    leggere, questa difesa non scatta. Nessun meccanismo risolve del tutto il
    problema; va detto invece che sperato.
- **Follow-up richiesti:**
  - Mitigazione parziale del punto precedente: un **sensore** (§5) che evidenzia nel
    testo della descrizione i costrutti tipici dell'abuso — istruzioni imperative
    rivolte all'assistente, riferimenti ad altri strumenti, richieste di credenziali o
    di lettura di file non pertinenti. È un aiuto alla lettura, **non una garanzia**,
    e va presentato come tale.
