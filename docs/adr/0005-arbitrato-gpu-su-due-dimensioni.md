# ADR-0005: Arbitrato GPU su due dimensioni, con quota audio sottratta dal budget

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

Quattro pilastri paritari si contendono una sola GPU da 16 GB. L'arbitro è
l'autorità unica (I2), ma resta da decidere **cosa** arbitra e **come** protegge i
lavori sensibili alla latenza.

L'approccio consueto — contare solo la VRAM — è insufficiente e produce un difetto
osservabile: la VRAM basta perché un render 3D parta, ma è la contesa di *calcolo* a
far balbettare wake word e STT. Un sistema che modella una sola dimensione non può
nemmeno rilevare il problema, tanto meno risolverlo.

Sulla protezione dei lavori audio, due approcci:

- **Priorità alta alla corsia audio:** semplice.
  *Contro:* una priorità è un ordinamento, e un ordinamento si applica solo al
  momento di scegliere *chi servire dopo*. Sotto pressione la memoria può essere già
  stata allocata a qualcun altro: la priorità arriva tardi.
- **Quota sottratta dal budget allocabile:** la VRAM dell'audio non entra mai nel
  pool assegnabile.
  *Contro:* quella memoria resta inutilizzata quando l'audio è spento.

## Decision

**1. L'arbitro modella due dimensioni**, con meccanismi distinti:

| Dimensione | Natura | Meccanismo |
|---|---|---|
| VRAM | capacità esclusiva | ammissione — o entra, o non parte |
| Calcolo | contesa condivisibile | corsie — chi ha priorità fa ridurre l'occupazione agli altri |

**2. Ogni tipo di lavoro dichiara un profilo di risorsa** nominato e versionato
(riserva VRAM, classe di calcolo, prelazionabilità, tempo di rilascio, avvio a
freddo). La riserva è **dichiarata dal richiedente e verificata dall'arbitro**: il
picco reale viene misurato e registrato, e una riserva sistematicamente sbagliata è
un difetto del profilo, non un incidente.

> ⚠️ **Rimando del 2026-08-27 — finding AUD-032. I cinque attributi sono realizzati da DUE
> strutture e non da un oggetto solo, e la decisione resta valida e non è superata.** Nel codice
> del Traguardo 5 `ResourceProfile` ha **quattro** campi — `name`, `reserved_vram`,
> `compute_class`, `preemption` — e le divergenze sono due, entrambe deliberate:
>
> | | |
> |---|---|
> | *prelazionabilità* e *tempo di rilascio* | sono **un** campo, `Preemption::{Never, After(Millis)}`: due campi separati rendono pronunciabile *«non prelazionabile con una grazia di 500 ms»*, che è uno stato illegale. La §5.3 della spec pretende che `InRevoca` sia **non rappresentabile**, e un booleano non può renderlo tale |
> | *avvio a freddo* | **non è nel profilo affatto**. `cold_start` vive in `WorkDescriptor`, che l'ammissione **non riceve**: serve ad **avvisare l'utente**, mai a decidere, e una decisione che volesse leggerlo **non ha una strada**. Non è una dimenticanza ma una **regola di livello 1**, tenuta dal caso `crates/kernel/tests/compile_fail/admission_reads_cold_start.rs`, che attende `E0609` |
>
> ⛔ **Perché il rimando arriva solo oggi, ed è la parte da ricordare:** le due divergenze erano
> state discusse e registrate contro la **spec** (§5.1, §5.2, §5.2.1) e contro l'errata del piano
> del Traguardo 5, e la catena si è fermata lì. Il documento di **origine** della lista dei cinque
> attributi non è stato toccato, come non lo era stato il compendio che la ricopia — gotcha
> **#68** applicato a un **elenco di campi** invece che a un numero. ⚠️ E la pratica opposta era
> visibile a due passi: [ADR-0004](0004-topologia-di-processo.md) porta due richiami e
> [ADR-0006](0006-due-policy-vram-come-oggetti-distinti.md) uno, per la stessa ragione.

**3. La quota VRAM dell'audio è sottratta dal budget allocabile all'avvio** e non vi
rientra. Nessun altro lavoro può richiederla, nemmeno a GPU scarica.

Vince sulla priorità perché una priorità può essere rispettata tardi, mentre un
budget sottratto **non può essere allocato per errore**. È la differenza tra una
garanzia e una buona intenzione.

## Consequences

- **Positive:**
  - Q1 (voce < 600 ms sotto carico) diventa difendibile per costruzione sul fronte
    memoria, invece di dipendere dall'ordine di arrivo delle richieste.
  - I profili rendono le stime **falsificabili**: uno sbagliato si scopre da solo.
  - Distinguere `Rifiutata` (non entra mai) da `InCoda` (non ora) elimina le attese
    infinite per lavori impossibili.
- **Negative (accettate):**
  - La quota audio è **memoria sprecata quando la voce è spenta**. Costo accettato in
    cambio della garanzia; mitigabile rendendo la quota nulla se la voce è disattivata
    nel profilo di configurazione.
  - I profili di risorsa sono manutenzione: cambiare modello o parametri di qualità
    obbliga a rivedere il profilo.
  - La contesa di calcolo si governa solo **indirettamente** (far ridurre
    l'occupazione ai lavori `batch`). Non è una garanzia forte.
- **Follow-up richiesti:**
  - **SP-2 (spike):** verificare che la riduzione di occupazione dei job `batch`
    basti a tenere Q1 sotto i 600 ms durante un render. Se non basta, l'unica leva
    restante è sospendere il job `batch`, e va deciso se è accettabile.
  - Il valore della quota audio si fissa dopo aver misurato i modelli vocali reali.
