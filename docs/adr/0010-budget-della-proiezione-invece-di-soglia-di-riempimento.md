# ADR-0010: La proiezione ha un budget di qualità, non una soglia di riempimento

- **Status:** Proposed
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

[ADR-0008](0008-contesto-come-proiezione-dello-stato.md) stabilisce che il contesto è
una proiezione ricomponibile. Restava implicito **quando** ricomporla, e la
formulazione iniziale di Q6 lasciava intendere: quando la finestra si esaurisce.

Questa assunzione è sbagliata, ed è documentata come tale. Il fenomeno ha un nome —
**context rot** — ed è il degrado misurabile della qualità delle risposte al crescere
del contesto, *ben prima* che il limite tecnico sia raggiunto: l'attenzione si
distribuisce su una finestra lunga e il modello sottopesa l'informazione centrale.
La conseguenza pratica riportata dalla ricerca 2025–2026 è che la finestra realmente
utile è sensibilmente inferiore a quella dichiarata, e che il fattore limitante è la
**qualità** del contesto, non il suo volume.

Progettare la compattazione come reazione all'esaurimento significa quindi far girare
l'agente in zona degradata per la maggior parte di una run lunga — cioè proprio dove
le long-horizon tasks passano il loro tempo.

Alternative considerate:

- **Soglia di riempimento** (approccio diffuso): si ricompone quando serve.
  *Pro:* nessuna configurazione, nessuna finestra "sprecata".
  *Contro:* si opera degradati per gran parte della run, e il degrado è invisibile —
  non produce errori, produce risposte peggiori.
- **Budget target:** la proiezione ha un'occupazione obiettivo molto al di sotto del
  limite, e la ricomposizione la mantiene lì.
  *Contro:* si rinuncia a una parte della finestra disponibile; il valore giusto non
  è noto a priori.

## Decision

**La proiezione ha un budget target, espresso come frazione della finestra del modello
e configurabile per modello.** La ricomposizione è **continua e proattiva**: serve a
mantenere il budget, non a evitare l'errore di overflow. Il limite della finestra
resta come guardia di sicurezza, non come politica.

**La proiezione è misurata per categoria** (obiettivo, vincoli, piano, decisioni,
fatti, riferimenti agli artefatti, trascrizione) e la misura entra nel giornale.
Senza questo, "il contesto è troppo pieno" non è un dato ma un'impressione: con
questo, si sa *quale categoria* lo sta divorando.

## Consequences

- **Positive:**
  - L'agente lavora nella zona in cui il modello rende, non in quella in cui entra.
  - La misura per categoria rende l'ottimizzazione mirata invece che congetturale.
  - Il budget è per modello: modelli diversi hanno finestre utili diverse, e la
    configurazione lo riflette invece di ignorarlo.
- **Negative (accettate):**
  - Si rinuncia deliberatamente a una parte della finestra pagata.
  - Ricomposizioni più frequenti: più lavoro per passo.
  - Il valore corretto del budget **non è noto a priori** e dipende dal modello.
- **Follow-up richiesti:**
  - **SP-3 (spike):** determinare empiricamente, per i modelli effettivamente usati,
    la frazione di finestra oltre la quale la qualità cala. Fino ad allora vale un
    default conservativo, dichiarato come tale e non spacciato per misurato.
