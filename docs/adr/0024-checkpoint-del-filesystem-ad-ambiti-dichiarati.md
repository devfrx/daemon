# ADR-0024: Il checkpoint del filesystem copre ambiti dichiarati

- **Status:** Proposed
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

Lacuna L-1: [ADR-0007](0007-giornale-write-ahead-e-riconciliazione.md) giornala le
**run**, non lo stato dei **file**. Riprendere una run dopo un crash funziona; tornare
indietro dopo che un passo ha rovinato dieci file no.

Non è una preoccupazione della sola capacità Coding: anche Generazione asset scrive
mesh e texture, e Conoscenza scrive indici e documenti derivati. Per la parità di
[ADR-0001](0001-architettura-a-kernel-con-capacita-paritarie.md) è quindi **kernel**.

Alternative considerate:

- **Snapshot dell'intero filesystem.** Copertura totale.
  *Contro:* impraticabile per volume e per tempo; e cattura anche ciò che non
  c'entra nulla con la run.
- **Affidarsi al versionamento (git).** Zero codice nuovo.
  *Contro:* git è **intenzionale e a grana di commit**, il checkpoint serve
  **automatico e a grana di passo**. Non copre file non versionati, non copre gli
  stati intermedi di una run, e non tutte le capacità lavorano dentro un repository.
- **Snapshot di ambiti dichiarati.** *Contro:* copertura parziale per costruzione.

## Decision

**1. Un _ambito di lavoro_ è un insieme di percorsi dichiarato esplicitamente.** Il
checkpoint copre quelli e nient'altro.

**2. Prima che un effetto tocchi un file dentro un ambito, la versione precedente è
conservata** e riferita dal passo del giornale che l'ha modificata. Ripristinare
significa riportare l'ambito allo stato precedente al passo N.

È lo stesso principio del write-ahead applicato ai file: si conserva *prima*, perché
dopo è troppo tardi.

**3. È distinto dal versionamento e vi convive.** Il checkpoint è automatico e a grana
di passo; git resta intenzionale e a grana di commit. Non si sostituiscono.

**4. Limite dichiarato: gli effetti fuori dagli ambiti non sono coperti.** Un comando
che scrive altrove non è annullabile dal checkpoint — e per la §4 resta un effetto
`verificabile` o `irripetibile`, quindi soggetto ad approvazione (§6).

## Consequences

- **Positive:**
  - Rollback istantaneo a uno stato noto-buono, senza dipendere da git e senza
    obbligare l'utente a lavorare in un repository.
  - Serve a tutte le capacità che producono file, non solo a Coding.
  - Si aggancia al giornale che esiste già: il checkpoint è **un riferimento in più
    sul passo**, non un secondo sistema.
- **Negative (accettate):**
  - Costo di spazio proporzionale alla quantità di scrittura. Serve una potatura,
    con la stessa logica a livelli di [ADR-0018](0018-ritenzione-a-livelli-del-giornale.md).
  - **Un ambito dichiarato male produce falsa sicurezza**: l'utente crede di poter
    tornare indietro e non può.
  - File molto grandi dentro un ambito rendono il costo sproporzionato.
- **Follow-up richiesti:**
  - L'interfaccia deve mostrare **cosa è coperto** dall'ambito attivo, prima che
    l'agente inizi a scrivere. È la mitigazione diretta della falsa sicurezza.
  - Va deciso in fase di implementazione un limite di dimensione oltre il quale un
    file viene escluso dal checkpoint **con avviso**, invece di far esplodere il disco
    in silenzio.
