# ADR-0034: I parametri di decisione sono consegnati al kernel, non letti

- **Status:** Accepted
- **Date:** 2026-08-07
- **Deciders:** proprietario del progetto

## Context

[ADR-0021](0021-simulazione-deterministica-e-iniettabilita.md) rende iniettabili
**tempo, casualità, I/O e scheduling** (V29). Quei quattro hanno una proprietà in
comune: sono i punti in cui il **non determinismo** entra in una decisione, e
sostituirli compra la riproducibilità.

Una decisione del kernel dipende però da **due** insiemi, non da uno:

| | Esempio | Governato da |
|---|---|---|
| ciò che **il mondo risponde** | «che ora è», «il file c'è», «quale attività è pronta» | ✅ V29 |
| i **parametri con cui il kernel è configurato** | «il budget è 16 GB», «la quota audio è 1 GB», «la policy attiva è REMOTA» | ⛔ nulla |

Il secondo insieme non ha nome, non ha una porta, e non è variabile in simulazione.

### Chi legge un parametro, e da dove

Cinque decisioni già accettate ne leggono uno, e **nessuna delle cinque passa da
un'interfaccia**:

| Parametro | Deciso in | Quando serve |
|---|---|---|
| quale policy VRAM è attiva | [ADR-0006](0006-due-policy-vram-come-oggetti-distinti.md) | all'avvio del core |
| la quota audio sottratta | [ADR-0005](0005-arbitrato-gpu-su-due-dimensioni.md) | all'avvio |
| la quota di presentazione | [ADR-0033](0033-gpu-della-gui-quota-di-presentazione.md) | all'avvio |
| il default dei vincoli sui dati | [ADR-0016](0016-permessi-granulari-e-default-dei-vincoli-sui-dati.md) | alla prima richiesta |
| il profilo «riservato» | [ADR-0023](0023-cifratura-a-riposo-e-gestore-dei-segreti.md) | **prima** dell'avvio |

Più i tetti di autonomia (V8), il budget della proiezione
([ADR-0010](0010-budget-della-proiezione-invece-di-soglia-di-riempimento.md)), le
finestre di ritenzione ([ADR-0018](0018-ritenzione-a-livelli-del-giornale.md)), il
livello di confinamento di default ([ADR-0025](0025-confinamento-a-livelli.md)) — e il
**totale** di VRAM, che la formula del budget allocabile usa in tre documenti senza che
nessuno ne dichiari la provenienza.

### Era già stato registrato, con l'innesco della specie sbagliata

La §8.3 della [spec del sotto-progetto 1](../superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md),
riga V3, lo dice: *«la provenienza dal profilo di configurazione **non ha consumatore**:
nessuna sezione mette la configurazione in perimetro»*, con innesco **A — esiste
un'interfaccia (2)**.

> La §8 ha fatto il proprio lavoro registrandolo invece di nasconderlo. Ma l'innesco è
> della specie sbagliata: *«esiste un'interfaccia»* descrive quando un parametro diventa
> **modificabile**, non quando diventa **necessario**. L'arbitro ha bisogno di un budget
> nel sotto-progetto 1, dove nessuna interfaccia esiste.

È lo stesso difetto di forma che la §8.5.1 aveva trovato sul backup: una riga registrata
contro una condizione che non è il suo vero innesco.

### Il modo di fallire, se non si decide

Un parametro non consegnato diventa una **costante scritta dentro il kernel**. Ed è
invisibile: non compare in nessun elenco, non fa scattare nessuna voce del catalogo §7, e
si manifesta solo il giorno in cui qualcuno prova a scrivere uno scenario che la fa
variare e scopre che non può. È la forma esatta del gotcha #12 — una violazione che nessun
elenco di «chiamate OS» mostrerebbe.

⚠️ **Cosa non poggia su una misura, e va detto.** Questa decisione non ne ha una, e non
ne serve una: non c'è un discriminante fra due opzioni che un numero possa sciogliere.
È una constatazione di coerenza, verificabile rileggendo i documenti citati sopra. La
stima *«la correzione tardiva sarebbe pervasiva ma meccanica»* è invece un giudizio, non
un dato.

### Alternative considerate

- **A — nulla: i parametri restano costanti nel kernel.**
  *Pro:* nessun attrito, nessun tipo nuovo.
  *Contro:* invisibile a ogni controllo della porta di qualità; la DST non può variarli,
  quindi un'intera classe di scenari è irraggiungibile — incluso quello di **RK-1**, che
  [ADR-0033](0033-gpu-della-gui-quota-di-presentazione.md) dichiara come proprio innesco
  osservabile.

- **B — il kernel legge la configurazione attraverso la porta `filesystem`.**
  *Pro:* I3 formalmente rispettato, nessun tipo nuovo.
  *Contro:* reintroduce un kernel che **chiede** invece di **ricevere**. Un registro a
  chiavi stringa è la porta girevole che V29 chiude: qualunque cosa diventa raggiungibile
  con una chiave nuova, e il formato dell'archivio entra nel kernel.

- **C — un valore globale, letto da chi serve.**
  *Contro:* i difetti di A, più uno: due simulazioni nello stesso processo
  condividerebbero i parametri, e l'isolamento fra corse della campagna DST cadrebbe.

- **D — parametri risolti, consegnati alla costruzione.**
  *Contro:* attrito su ogni firma che ne legge uno, e un tipo in più da tenere allineato
  agli ADR che nominano parametri.

## Decision

**Nessuna decisione del kernel legge un parametro che non le è stato consegnato.**

| # | Regola |
|---|---|
| 1 | Il kernel **non legge la configurazione**. Riceve, alla costruzione, un valore che porta i **parametri risolti** di cui ha bisogno |
| 2 | Il kernel **non nomina** un file, una chiave o un valore di default: nessuno dei tre è esprimibile al suo interno |
| 3 | Chi **produce** il valore sta fuori: è `daemon`, che in produzione lo ricava dall'archivio attraverso `platform`, e in simulazione lo riceve dal banco di prova |
| 4 | La **sostituzione** di un parametro è un passo giornalato. Non è nuovo: la §5.4 lo impone già alla transizione di policy, e qui se ne riconosce la **forma generale** |

**Il precedente, che rende questa una generalizzazione e non un'astrazione nuova.**
[ADR-0011](0011-routing-risolto-e-giornalato-per-richiesta.md) dice del record di routing:
*«contiene la decisione **risolta**, non un riferimento alla configurazione: rileggere la
configurazione di oggi non dice cosa accadde ieri»*. È la stessa mossa un livello sopra —
là il giornale non vi rimanda, qui il kernel.

### Perimetro negativo — cosa questa decisione **non** è

Il rischio qui non è la pigrizia, è l'opposto. Va scritto, o cresce da solo:

| Non è | |
|---|---|
| un **sistema di configurazione** | niente formato, niente schema, niente validazione, niente ricarica a caldo |
| un **registro a chiavi stringa** | è l'alternativa B, e la sua conseguenza è che il kernel torna a poter chiedere |
| una **sostituzione a caldo generalizzata** | in questo sotto-progetto nessuno può cambiare un parametro. Entra solo la policy, perché la §5.4 la pretendeva già |
| un ADR sul **formato** dell'archivio | non serve a nessuno adesso, e non si decide qui |

### Cosa entra ora e cosa si scaglia

Con il criterio A/B/C della §0.3 della spec del sotto-progetto 1:

| | Regola | Perché |
|---|---|---|
| il **tipo** dei parametri risolti, e il fatto che il kernel li riceve | **B** | consegnarli dopo cambia la firma di ogni decisione che ne legge uno: pervasivo, e invisibile a ogni controllo finché non si prova a variarli |
| che il kernel non nomini file, chiave o default | **B** | idem |
| che la sostituzione sia un passo giornalato | — | ✅ già preteso dalla §5.4 |
| l'**archivio** su disco e il suo formato | **C** | innesco: esiste un'interfaccia (2) |
| il **pannello** che li modifica | **C** | idem |
| i **valori** reali | — | li tarano SP-1, SP-2 e M5: parametri, non impianto |

**Non è A** — senza, la DST prova ancora Q2, Q4 e Q5 a parametri fissi. **Non è C** —
l'arbitro ha bisogno di un budget in questo sotto-progetto, non quando arriverà una
capacità L2. È **B**, di specie **meccanica**: la correzione tardiva è pervasiva ma non è
una riprogettazione, e dirlo evita di gonfiarla per giustificare il lavoro.

## Consequences

- **Positive:**
  - I3 vale anche per i parametri, e non solo per le chiamate: oggi il confine copriva
    ciò che il kernel *fa*, non ciò con cui *è stato configurato*.
  - **La DST può far variare i parametri col seme.** Lo scenario di RK-1 — quota audio
    più quota di presentazione contro il profilo minimo di TRELLIS2 — diventa
    esplorabile **prima** che M5 ne misuri i valori. La misura resta necessaria per il
    numero; la campagna copre il comportamento a quel numero.
  - **«Due policy attive» smette di essere rappresentabile.** Se il valore consegnato ne
    porta una, l'unicità che la §5.4 verificava con un test a esempi sale al compilatore,
    con il dispositivo del gettone (§6.3).
  - La provenienza del **totale** di VRAM smette di essere ignota: è un parametro
    dichiarato, con lo stesso trattamento che [ADR-0005](0005-arbitrato-gpu-su-due-dimensioni.md)
    dà alla riserva — *dichiarata dal richiedente, picco misurato*.

- **Negative (accettate):**
  - **Attrito su ogni firma che legge un parametro.** Si paga a ogni riga, non una volta
    sola. È lo stesso genere di costo che ADR-0021 dichiara per V29 — *«vincola
    l'implementazione fin dalla prima riga»*.
  - **I valori di default vivono in `daemon` come letterali**, finché non esiste
    l'archivio. È il confine corretto, non una scorciatoia — ma va **detto**, non
    nascosto.
  - **Un tipo in più da tenere allineato.** Ogni ADR che introduce un parametro deve
    comparirvi, o quel parametro rientra come costante da un'altra porta.
  - ⛔ **Il compilatore non può vietare una costante.** Una regola può pretendere che una
    decisione **riceva** i propri parametri; non può impedire che qualcuno ne scriva uno
    dentro il kernel. È il limite del gettone (§6.3.2) applicato qui: **prova la
    provenienza, non l'esclusività**. Il controllo che copre quel buco è la campagna, e
    copre solo i parametri che fa davvero variare — non è una prova di assenza.
  - **Il rischio è la crescita.** Un «sistema di configurazione» qui sarebbe
    sovra-ingegnerizzazione; il perimetro negativo sopra è la difesa, ed è la parte che
    non va tolta.

- **Follow-up richiesti:**
  - La sezione che lo specifica è la **§2.8** della spec del sotto-progetto 1, con i due
    controlli e le rispettive contro-sonde.
  - La §5.1 dichiara la provenienza del **totale** di VRAM.
  - Il catalogo §7.4.1 acquisisce la riga sull'unicità della policy, che nessuna sezione
    enumerava benché la §5.4 l'avesse decisa.
  - La riga **V3** della §8.3 va riscritta: la sua metà rimandata non è più «la
    configurazione non ha consumatore» ma «l'archivio e il pannello non esistono», e
    l'innesco A resta valido **per quella metà**.
  - Se un giorno un parametro dovesse essere modificabile a caldo, il meccanismo è già
    scelto — un passo giornalato — e non va inventato allora.
