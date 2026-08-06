# ADR-0012: L'equivalenza del fallback è definita dai vincoli, e sui dati si fallisce chiuso

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

Il fallback a catena — provare il modello o il provider successivo quando il primo
fallisce — è una funzionalità attesa e sensata: protegge da limiti di frequenza,
indisponibilità, contesto eccessivo, moderazione.

Contiene però un'assunzione pericolosa quando la richiesta ha **vincoli sui dati**.
Se una richiesta esige che il provider non conservi i dati e la catena ripiega su un
endpoint che invece li conserva, la richiesta è formalmente riuscita e
sostanzialmente ha fatto **esattamente ciò che si voleva evitare**. Il fallimento è
invisibile: nessun errore, nessun avviso, solo dati finiti dove non dovevano.

Il problema si generalizza oltre la ritenzione dei dati: ogni vincolo che l'utente
pone alla richiesta — provider esclusi, tetto di prezzo, esecuzione solo locale —
può essere violato da un ripiego "utile".

Alternative considerate:

- **Fallback come lista di alternative capaci.** L'equivalenza è definita dalla
  capacità del modello: se sa fare il compito, va bene. *Pro:* massima disponibilità.
  *Contro:* i vincoli della richiesta vengono trattati come preferenze.
- **Fallback come lista di alternative _conformi_.** L'equivalenza è definita dai
  vincoli della richiesta. *Contro:* più richieste falliscono.

## Decision

**1. Un candidato che viola un vincolo della richiesta non è un fallback: è una
richiesta diversa.** Viene scartato prima della valutazione, non provato e poi
rimpianto. L'equivalenza della catena è definita dai **vincoli**, non dalla capacità.

**2. I vincoli si dividono in due classi, con esiti opposti a catena esaurita:**

| Classe | Esempi | A catena esaurita |
|---|---|---|
| **su dati e riservatezza** | ritenzione dati, provider esclusi, solo locale | **fallisce chiuso**: errore, nessun ripiego |
| **su qualità e costo** | tetto di prezzo, modello preferito, latenza | **degrado dichiarato**: si procede avvisando |

Fallire chiuso è controintuitivo — un sistema che si rifiuta di rispondere sembra
rotto — ed è proprio per questo che va deciso adesso e scritto: sotto pressione, la
tentazione di "provare comunque" è forte, e il danno è irreversibile.

**3. L'indisponibilità di risorsa è una causa di fallback di prima classe.** Se il
routing sceglie la destinazione locale e l'arbitro GPU rifiuta o accoda (V1), non è
un errore: è un motivo legittimo per passare al candidato successivo. Il record di
routing ne conserva la traccia.

**4. Un ritentativo non è un passo nuovo.** Riprovare lo stesso candidato dopo un
errore transitorio, o passare al successivo, resta **dentro lo stesso passo**: cambia
il record di routing, non la struttura della run.

## Consequences

- **Positive:**
  - Un vincolo posto dall'utente resta un vincolo, non una preferenza.
  - Il collegamento con §2 diventa naturale: il rifiuto dell'arbitro è un ingresso
    del routing, non un'eccezione da gestire a parte.
  - La distinzione ritentativo/passo tiene il giornale leggibile: una run di 20 passi
    non diventa una run di 60 perché la rete era instabile.
- **Negative (accettate):**
  - **Più richieste falliranno del tutto.** È il prezzo, e va reso comprensibile
    nell'interfaccia: il messaggio deve dire *quale* vincolo non è stato soddisfatto,
    non un generico errore di rete.
  - Ogni richiesta deve dichiarare i propri vincoli, anche quando sono i default:
    lavoro in più per chi definisce le capacità.
- **Follow-up richiesti:**
  - Il default dei vincoli sui dati va deciso esplicitamente in §6. Un default
    permissivo rende la regola inutile; uno troppo stretto rende il sistema
    inutilizzabile con i provider comuni.
  - Le richieste che si annullano spesso vanno preferibilmente instradate verso
    provider che supportano l'annullamento, per non pagare stream interrotti
    ([ADR-0011](0011-routing-risolto-e-giornalato-per-richiesta.md)).
