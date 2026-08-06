# ADR-0020: Nessun modello nel percorso decisionale del kernel

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

I sistemi ad agenti sono probabilistici: una funzione che restituisce output diversi a
parità di input è comportamento normale, e richiede una metodologia di valutazione
diversa dal test tradizionale. Applicare il testing classico a un sistema
non-deterministico è l'errore più diffuso del settore.

Ma questa proprietà **non è uniforme dentro il sistema**. La ricerca 2026 sulla
valutazione a livelli isolati distingue esplicitamente lo *scaffold deterministico* di
un agente in produzione — che si può proteggere con un harness di test **senza LLM**,
bloccato sulle regressioni — dallo strato probabilistico, che richiede valutazione con
giudice e dataset curati.

Il nostro kernel appartiene interamente al primo strato, e vale la pena verificarlo
componente per componente:

| Componente del kernel | Decide con un modello? |
|---|---|
| arbitro GPU: ammissione, code, corsie, revoca | no |
| gateway: scelta del candidato, catena, vincoli, contabilità | no — la *risposta* è probabilistica, la *decisione di routing* no |
| giornale: transizioni, riconciliazione, classi di effetto | no |
| permessi, confine dei tipi, canary | no |
| registro dei sensori: esecuzione, raccolta dei verdetti | no — un sensore *inferenziale* è probabilistico, ma il kernel tratta il verdetto come **dato opaco** |
| anello 4: rilevamento della ricorrenza | no — la *proposta* può essere inferenziale, il *rilevamento* no |

Nessun componente usa un modello per decidere. Non è un caso: è una proprietà emersa
dal design, e conviene renderla vincolante prima che qualcuno la eroda per comodità.

## Decision

**Il kernel non contiene mai un modello nel proprio percorso decisionale.** I modelli
sono invocati *attraverso* il kernel e i loro esiti sono trattati come dati opachi:
mai come giudizi su cui il kernel basa il proprio comportamento.

Corollario operativo: **il kernel è testabile interamente senza chiamare un modello.**
La valutazione probabilistica — giudice, dataset curati, trace-based eval — appartiene
alle capacità L2, dove il non-determinismo effettivamente vive.

## Consequences

- **Positive:**
  - I test del kernel sono veloci, deterministici e eseguibili a ogni commit, senza
    chiavi API, senza costo per esecuzione e senza esiti intermittenti.
  - Un fallimento del kernel è **sempre** un difetto, mai variabilità del modello: la
    distinzione più costosa da fare a posteriori è già fatta a monte.
  - Rende possibile la simulazione deterministica di
    [ADR-0021](0021-simulazione-deterministica-e-iniettabilita.md), che con un modello
    nel percorso non sarebbe riproducibile.
- **Negative (accettate):**
  - Alcune decisioni che un modello renderebbe più "intelligenti" restano basate su
    regole: la scelta del candidato di routing, il rilevamento delle ricorrenze,
    l'ordinamento delle code. È una rinuncia deliberata, e si paga in raffinatezza.
  - La tentazione di violarla arriverà travestita da miglioramento — «e se il modello
    scegliesse il provider migliore?». Va trattata come un ADR, non come un'aggiunta.
- **Follow-up richiesti:**
  - La regola è verificabile: il kernel non deve avere alcun percorso di chiamata
    verso il gateway *per proprio conto*. Va controllato staticamente, come I3.
