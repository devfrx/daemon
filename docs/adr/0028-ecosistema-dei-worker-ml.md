# ADR-0028: Ecosistema dei worker ML — Python, ratificato invece che implicito

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

Questa scelta era **già stata fatta per inerzia e mai ratificata**. TRELLIS2, gli
embedding, la trascrizione e la sintesi vocale vivono in Python in ogni documento del
progetto, senza che nessun ADR lo dicesse e senza che nessuno ne avesse valutato i
costi.

Una decisione presa per inerzia è indistinguibile da una decisione presa male: nessuno
sa se sia stata valutata, e quindi nessuno sa se possa essere rimessa in discussione.
Questo ADR non sceglie: **ratifica**, e dichiara ciò che si paga.

**L'alternativa non esiste davvero.** I modelli che il perimetro richiede hanno
implementazioni di riferimento in Python, e le loro dipendenze native — CUDA in testa —
sono esposte attraverso quell'ecosistema. Riscrivere l'inferenza in un altro linguaggio
significherebbe reimplementare il modello, non portarlo: è un progetto a sé, e non
questo.

| Carico | Perché Python non è negoziabile |
|---|---|
| TRELLIS2 (asset 3D) | implementazione di riferimento e leve documentate — `max_num_tokens`, `generate_texture_slat`, passi di campionamento — vivono lì ([riferimenti](../riferimenti.md)) |
| embedding per la conoscenza | idem |
| STT e TTS | idem |

Va detto con precisione **cosa** questa decisione riguarda e cosa no:
[ADR-0026](0026-linguaggio-del-core.md) esclude Python dal **core** perché V28 e V19
richiedono verifica statica e ADR-0004 richiede concorrenza reale. Quell'esclusione
riguardava il core, non il progetto. Qui Python rientra dove è insostituibile.

Alternative considerate:

- **Reimplementare l'inferenza nel linguaggio del core.** *Pro:* un solo runtime da
  impacchettare. *Contro:* è la riscrittura dei modelli, non un port. Fuori scala.
- **Worker Python, confinati in processi propri.** *Contro:* i costi elencati sotto.

## Decision

**I worker ML si scrivono in Python**, ciascuno in un processo proprio, secondo la
classe `worker` di [ADR-0004](0004-topologia-di-processo.md).

**Cosa i worker NON contengono**, e non è una raccomandazione:

| Vietato nel worker | Vive nel core | Da |
|---|---|---|
| logica di ritentativo | sì | **I5** |
| code e priorità | sì | **I5** |
| stato che sopravviva al processo | sì | **I1** |
| comunicazione con un altro worker | il core coordina | ADR-0004 |
| accesso alla GPU senza concessione | l'arbitro concede | **I2** |

Un worker esegue un compito e può essere **ucciso senza preavviso** in qualsiasi
istante. È la proprietà che rende Q4 verificabile, e che il linguaggio del worker non
influenza in alcun modo.

## Consequences

- **Positive:**
  - I modelli si usano nella forma in cui sono pubblicati e mantenuti, senza uno strato
    di traduzione da tenere allineato a ogni rilascio.
  - **Il confine di processo diventa obbligatorio invece che una scelta**, e questo
    *conferma* ADR-0004 invece di contraddirlo: un OOM dello strato ML non può portarsi
    via il core, perché non condivide con lui nemmeno il runtime.
  - Il linguaggio del core non deve essere compatibile con Python in-process. Quel
    vincolo, che in altri progetti deciderebbe tutto, qui non esiste — ed è il motivo
    per cui ADR-0026 ha potuto decidere sul merito.

- **Negative (accettate):**
  - **Il packaging non è più un binario singolo.** Un ambiente Python va installato,
    versionato e aggiornato accanto al core. È un costo di L3 (integrazione OS,
    packaging) che ADR-0026 non ha e che va pagato qui.
  - **Avvio a freddo dell'interprete, in aggiunta a quello del modello.** Pesa
    direttamente su **Q8**, che promette un avvio a freddo *dichiarato*: la promessa
    resta mantenibile, ma il numero da dichiarare è più grande.
  - **Dipendenze native fuori dal nostro controllo**: CUDA, driver, ruote binarie
    compilate contro versioni specifiche. Un aggiornamento del driver può rompere
    l'inferenza senza che nulla nel nostro codice sia cambiato. Va registrato come
    rischio nella §9 della spec.
  - Due ecosistemi di dipendenze da tenere aggiornati, con due modelli di sicurezza
    diversi e due superfici di supply chain.

- **Follow-up richiesti:**
  - Il sotto-progetto 9 (gestione modelli locali) e il sotto-progetto 10 (integrazione
    OS) devono trattare l'ambiente Python come **artefatto da versionare**, non come
    prerequisito dell'utente. Un assistente locale che chiede all'utente di installare
    Python ha già fallito.
  - Il costo di avvio a freddo va **misurato** e non stimato, prima di fissare il valore
    dichiarato di Q8.
  - Il profilo di risorsa di ogni worker ([ADR-0005](0005-arbitrato-gpu-su-due-dimensioni.md))
    deve includere il costo dell'interprete, non solo quello del modello: altrimenti la
    riserva è sistematicamente sottostimata proprio dove conta.
