# ADR-0018: Ritenzione a livelli — la struttura sopravvive, i payload si potano

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

[ADR-0011](0011-routing-risolto-e-giornalato-per-richiesta.md) ha lasciato aperta la
ritenzione: il giornale cresce a ogni passo, e ora registra anche i record di routing.
Cresce indefinitamente, e nessuno vuole un audit trail che riempie il disco.

Il volume non è distribuito uniformemente. Pochi byte per passo sono struttura —
identità, tempi, esiti, costi, verdetti — e molti byte sono **payload**: prompt
inviati, risposte del modello, output degli strumenti, trascrizioni.

Alternative considerate:

- **Conservare tutto per sempre.** *Contro:* il disco.
- **Ritenzione a tempo uniforme** (es. 90 giorni). *Contro:* si perde tutto di una run
  vecchia, inclusa la sua struttura — che è la parte piccola e la più utile: costi
  storici, tassi di fallimento, casi di regressione.
- **Ritenzione a livelli**, per tipo di dato invece che per età sola.

## Decision

**La ritenzione segue la stessa gerarchia della compattazione del contesto**
([ADR-0008](0008-contesto-come-proiezione-dello-stato.md)): ciò che è strutturato
sopravvive, i payload grezzi si potano.

| Livello | Contenuto | Ritenzione |
|---|---|---|
| **struttura** | identità di run e passi, transizioni, esiti, record di routing, costi, verdetti dei sensori, decisioni | lunga, configurabile; è la parte piccola |
| **payload** | prompt, risposte del modello, output degli strumenti, trascrizioni | finestra breve, poi potati sostituendoli con impronta e dimensione |
| **artefatti** | file prodotti | non nel giornale: **riferimenti**, il contenuto vive sul filesystem |

**La potatura è irreversibile e va dichiarata:** un record potato dice di esserlo. Un
payload assente e un payload mai registrato non devono essere indistinguibili.

**Un passo `InDubbio` non è mai potabile** finché non è riconciliato: la sua
riconciliazione (§4) può dipendere dal payload.

## Consequences

- **Positive:**
  - Le statistiche storiche — costi, tassi di fallimento, ricorrenze per l'anello 4 —
    sopravvivono indefinitamente a costo trascurabile.
  - Il replay dettagliato resta possibile per il passato recente, dove serve davvero.
  - Coerenza: la stessa regola governa cosa si perde nel contesto e nel giornale.
- **Negative (accettate):**
  - **Il replay integrale di una run vecchia non è più possibile.** Si conserva cosa è
    accaduto, non ogni parola scambiata.
  - L'impronta di un payload potato permette di verificare un sospetto, non di
    ricostruire il contenuto.
- **Follow-up richiesti:**
  - Prima della potatura, i fallimenti candidati a diventare casi di regressione
    (anello 4) vanno **promossi** a materiale conservato. Potare un fallimento non
    ancora sfruttato butta via esattamente il dato più prezioso.
