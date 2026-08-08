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

> ✅ **Rimando — [ADR-0034](0034-parametri-di-decisione-consegnati-non-letti.md) aggiunge
> un secondo asse** (2026-08-07). I quattro elencati qui sono i punti in cui il **non
> determinismo** entra in una decisione. Ma una decisione dipende anche dai **parametri
> con cui il kernel è stato configurato** — budget della GPU, quote sottratte, policy
> attiva, tetti di autonomia — che sono deterministici e che nessuna sezione consegnava.
> Questo ADR **non è superato**: la sua enumerazione era corretta per ciò che affermava,
> e ciò che non affermava è ora coperto. Il modo di fallire chiuso da ADR-0034 è che un
> parametro non consegnato diventa una **costante**, invisibile a ogni controllo, che
> impedisce alla campagna di esplorare la propria configurazione.
>
> ✅ **E il secondo asse è entrato nel testo di `V29` il 2026-08-08**, chiudendo la §7.1.1
> della spec del sotto-progetto 1. Fino a quel giorno il vincolo ne nominava quattro, quindi
> il controllo di livello 1 che impedisce di costruire una decisione senza i propri parametri
> **non aveva un vincolo da nominare** — ed era una delle otto righe del catalogo che la
> regola d'ammissione avrebbe fatto togliere.

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

---

## ✅ Rimando — «permanente» e «suite di regressione» sono stati precisati (2026-08-08)

Il punto 3 della Decision dice che il seed diventa *«un caso di regressione permanente»*, e
le Consequences che *«i seed accumulati formano una suite di regressione che cresce da
sola»*. La **§3.4** della spec del sotto-progetto 1 restringe entrambe le formulazioni, e la
restrizione va registrata qui invece di vivere solo là:

| Cosa diceva | Cosa vale |
|---|---|
| il seed è un caso di regressione **permanente** | ⚠️ **no**: un seed non riproduce la stessa esecuzione dopo un cambio di codice. È un **punto di ripartenza per indagare**, non un oracolo |
| i seed formano una **suite di regressione** | ⚠️ **no**: a entrare nella suite è la **proprietà** che quel difetto violava. Un elenco di semi presentato come suite sarebbe una falsa sicurezza |

⛔ **L'ADR non è superato, e la sostanza del punto 3 regge**: ogni difetto trovato in
simulazione **conserva** il proprio seed, e il seed **si versiona**. Ciò che cade è
l'aspettativa che quel seed lo **riprodurrà** per sempre. La conseguenza pratica è scritta
in §3.4 e nella riga V31 di §8.3: *«l'automatismo protegge la proprietà, non il seme»*, e la
debolezza è dichiarata invece che nascosta.

📌 Trovato in un audit sezione-contro-ADR il 2026-08-08. Vale il gotcha **#29**: la
formulazione più corta — qui «permanente» — è quella che viene citata, e nessuno la
confronta con ciò che il meccanismo fa davvero.
