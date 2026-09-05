# ADR-0008: Il contesto è una proiezione dello stato durevole, non lo stato

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

> ⚠️ **Rimando del 2026-09-05 — la proiezione si costruisce nello stesso sotto-progetto di kernel del
> registro delle guide, prima della prima capacità che la usa; e la mappa della knowledge base è una
> categoria della proiezione.** Il
> [disegno della knowledge base](../superpowers/specs/2026-09-04-knowledge-base-design.md) (§1.1d e
> §2.2, sotto accettazione condizionata, riletto dal proprietario il 2026-09-04) ha misurato che al
> 2026-09-04 la proiezione qui decisa non ha una riga di codice (§6.1 del disegno, col comando), e
> la colloca nel sotto-progetto **13** della roadmap con il registro delle guide e i trigger di
> [ADR-0009](0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md). Per la knowledge base il
> router centrale e quello dell'ambito entrano nella proiezione a ogni passo **per chiave** —
> ambito, run, modello — mai per lettura del loro testo
> ([ADR-0020](0020-nessun-modello-nel-percorso-decisionale-del-kernel.md)); le foglie entrano come
> **riferimenti** e si rileggono su richiesta, che è già la regola di questo ADR per gli artefatti.
> **Nessuna riga di questo ADR è superata.**

## Context

Le run lunghe falliscono quasi sempre per la stessa ragione: la finestra di contesto
si riempie prima che il task sia finito. È il requisito Q6.

L'approccio diffuso è riassumere la conversazione quando lo spazio scarseggia. È
lossy **e irreversibile**: ciò che il riassunto taglia è perso per sempre, e nessuno
sa cosa sia stato tagliato. Il sistema non può nemmeno accorgersi del danno.

Alternative considerate:

- **Contesto = stato.** La conversazione *è* la memoria della run.
  *Pro:* semplicissimo, nessuna struttura da mantenere.
  *Contro:* perdita irreversibile; impossibile cambiare modello a metà run;
  un sub-agente inquina il contesto del padre con tutto il proprio lavoro.
- **Contesto = proiezione.** La verità sta in strutture durevoli; la finestra è una
  vista ricalcolabile a ogni passo.
  *Contro:* l'agente deve *scrivere* nello stato, non solo parlare.

## Decision

**Il contesto è una proiezione dello stato durevole.** La finestra si compone dagli
elementi durevoli a ogni passo, e la compattazione **ricalcola la proiezione** invece
di riassumere la conversazione.

| Elemento durevole della run | Sacrificabile? |
|---|---|
| obiettivo | mai |
| vincoli e regole applicabili | mai |
| piano e stato dei passi | mai |
| decisioni prese, con il motivo | mai |
| fatti acquisiti, con la provenienza | mai |
| artefatti prodotti — **riferimenti**, non contenuti | mai; il contenuto si rilegge su richiesta |
| trascrizione grezza | **sì: è l'unica cosa sacrificabile** |

Che la trascrizione grezza sia l'unica perdita ammessa è il cuore della decisione.
Tutto ciò che serve a proseguire è strutturato, quindi non finisce mai nel tritacarne
del riassunto.

## Consequences

- **Positive:**
  - Q6 soddisfatto: il contesto che si riempie smette di essere un evento terminale.
  - **Cambiare modello a metà run** diventa possibile: la proiezione si ricompone.
  - **Sub-agente = proiezione ristretta.** L'isolamento del contesto non è un
    meccanismo in più: è la stessa idea applicata con un filtro diverso.
  - Riprendere una run dopo giorni funziona come riprenderla dopo un minuto.
- **Negative (accettate):**
  - Ogni passo paga la ricomposizione della proiezione.
  - L'agente deve **scrivere** nello stato durevole, non solo produrre testo. È un
    costo reale nella progettazione degli strumenti e dei prompt.
  - **Il modo di fallire si sposta**, non scompare: da "il contesto si riempie" a
    "l'agente non ha registrato una decisione". Una decisione non scritta è, a tutti
    gli effetti, una decisione mai presa.
- **Follow-up richiesti:**
  - Mitigazione del punto precedente: **registrare è un passo con un effetto**, non
    un'aspettativa sul comportamento del modello. Se non è giornalato, non è
    avvenuto — e questo lo rende osservabile invece che sperato.
  - Il kernel fornisce il **meccanismo** (stato durevole, ricomposizione); la
    **politica** di cosa includere nella proiezione appartiene alla capacità.
