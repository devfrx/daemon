# ADR-0021: Simulazione deterministica, e iniettabilità come requisito di costruzione

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

I requisiti più difficili del kernel non sono verificabili con test a esempi:

| Requisito | Perché sfugge ai test tradizionali |
|---|---|
| Q2 — zero OOM | dipende dall'**intreccio** di richieste concorrenti, non da un caso |
| Q4 — kill di un worker in *qualsiasi* istante | gli istanti sono innumerevoli; sceglierne tre a mano non prova nulla |
| Q5 — riavvio a metà run senza effetti rieseguiti | è **crash-consistency**: la violazione si manifesta solo dopo l'interruzione, quando gira la logica di ripristino |
| I1 — nessuna perdita uccidendo un processo | idem |

Esiste una tecnica consolidata per esattamente questa classe: la **simulazione
deterministica** (DST). Esplora molti cammini di esecuzione, inietta guasti casuali, e
— dato il seed iniziale — **riproduce esattamente lo stesso cammino**, rendendo
riproducibile un difetto di concorrenza. Nata in FoundationDB, è oggi in uso in
TigerBeetle, Resonate e nella riscrittura di SQLite di Turso, con presentazioni
dedicate a QCon London e FOSDEM 2026.

Ad essa si affianca la **crash-injection ai confini di persistenza**: si enumerano i
punti in cui i dati diventano durevoli, si inietta un crash in ciascuno, e si valida
lo stato dopo il ripristino. È letteralmente il test di ADR-0007.

Il punto critico: **la DST non è retrofittabile.** Richiede che tempo, casualità, I/O
e ordinamento delle attività concorrenti siano sostituibili dall'esterno. Se il codice
legge l'orologio di sistema o genera numeri casuali direttamente, non c'è modo di
riprodurre un'esecuzione.

Alternative considerate:

- **Test a esempi + qualche test di concorrenza.** Basso costo iniziale.
  *Contro:* le proprietà che contano restano non verificate, e i difetti che emergono
  sono irriproducibili — il tipo di bug che costa settimane.
- **DST introdotta più avanti, "quando serve".** *Contro:* significa riscrivere il
  kernel. Non è un test aggiunto: è una proprietà del codice.

## Decision

**1. Il kernel si verifica con simulazione deterministica** per le proprietà di
concorrenza, crash e ripristino, e con crash-injection ai confini di persistenza per
la riconciliazione (§4).

**2. Tempo, casualità, I/O e scheduling sono iniettabili.** È un **requisito di
costruzione**, non un'infrastruttura di test: nessun componente del kernel legge
l'orologio, genera casualità o esegue I/O se non attraverso un confine sostituibile.

**3. Ogni difetto trovato in simulazione conserva il proprio seed**, e il seed diventa
un caso di regressione permanente.

Vale l'argomento già usato per I6 e per il confine OS: **è una proprietà che si
ottiene solo costruendola dall'inizio.** Il costo di introdurla adesso è un'astrazione;
il costo di introdurla dopo è una riscrittura.

## Consequences

- **Positive:**
  - I requisiti Q2, Q4, Q5 e le invarianti I1/I2/I5 diventano verificabili invece che
    dichiarati.
  - Un difetto di concorrenza è **riproducibile a comando**: la classe di bug più
    costosa del progetto perde il suo costo principale.
  - I seed accumulati formano una suite di regressione che cresce da sola, e si
    innesta naturalmente sull'anello 4 (§5).
- **Negative (accettate):**
  - **Vincola l'implementazione fin dalla prima riga**: niente orologio di sistema,
    niente casualità diretta, niente I/O non mediato. È il vincolo più invasivo
    dell'intera spec.
  - La scelta del linguaggio del core dovrà tenerne conto: alcuni ecosistemi rendono
    la sostituzione dello scheduler molto più semplice di altri. Diventa un criterio
    dell'ADR sul linguaggio, non un dettaglio.
  - Costruire il simulatore è lavoro reale prima che il kernel faccia qualcosa di
    visibile.
- **Follow-up richiesti:**
  - L'ADR sul linguaggio del core deve valutare esplicitamente la **sostituibilità
    dello scheduling**. È il primo caso in cui una decisione di test vincola una
    decisione di architettura, e va detto invece che scoperto.
