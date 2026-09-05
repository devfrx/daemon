# ADR-0009: Guide, sensori e anelli di controllo sono meccanismi di kernel

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

> ⚠️ **Rimando del 2026-09-05 — il registro delle guide e i trigger si costruiscono in un sotto-progetto
> di kernel PRIMA della prima capacità che inietta una guida, e il registro porta due pretese.** Il
> [disegno della knowledge base](../superpowers/specs/2026-09-04-knowledge-base-design.md) (§1.1d–e
> e §2, sotto accettazione condizionata, riletto dal proprietario il 2026-09-04) ha misurato che al
> 2026-09-04 nessuno dei meccanismi qui decisi — registro delle guide, trigger — né la proiezione di
> [ADR-0008](0008-contesto-come-proiezione-dello-stato.md) ha una riga di codice (§6.1 del disegno,
> col comando), e ha fissato l'**ordine**: nascono nel sotto-progetto **13** della roadmap, prima del
> 3, o la prima capacità si inietta le skill a modo suo e nascono due strade. Le due pretese: la
> **chiave di contesto** con cui il registro inietta esprime *ambito, run, modello*; ciò che il
> registro conserva porta **provenienza e impronta** all'approvazione, e «approvate ora» è una
> **proiezione del giornale**, non un secondo archivio — la forma che `crates/kernel/src/permission.rs`
> già usa per i permessi. Il sensore d'integrità dei router e l'anello che propone una voce quando
> un puntatore marcisce sono **usi** del contratto del sensore e dell'anello di miglioramento, non
> cambiamenti. **Nessuna riga di questo ADR è superata.** ⚠️ Se le difese di
> [ADR-0015](0015-descrizioni-degli-strumenti-fissate-all-approvazione.md) si estendano alle skill —
> la decisione registrata dal rimedio di AUD-004 — resta del proprietario, con un ADR suo, prima
> del 13.

## Context

Lo stato dell'arte 2026 tratta l'harness — cioè **tutto ciò che sta attorno al
modello** — come oggetto di progettazione a sé (`Agent = Model + Harness`). Il
vocabolario consolidato distingue:

| Categoria | Natura | Esempi |
|---|---|---|
| **Guide** (feedforward) | anticipano il comportamento, agiscono *prima* | convenzioni, documentazione di architettura, regole di progetto, istruzioni d'uso degli strumenti |
| **Sensori** (feedback) | osservano *dopo* l'azione e permettono l'autocorrezione | linter, type checker, test, revisione, monitoraggio a runtime |

E distingue i sensori per costo di esecuzione:

| Tipo | Velocità | Determinismo |
|---|---|---|
| **computazionale** | ms–s | deterministico |
| **inferenziale** | s–min | probabilistico, ma semanticamente più ricco |

Il nostro design possiede già il **substrato di osservazione** (il giornale,
[ADR-0007](0007-giornale-write-ahead-e-riconciliazione.md)) ma non ha né un posto
dove vivono le guide, né un'astrazione per i sensori, né alcun **anello di ritorno**
che consumi il giornale per migliorare il sistema.

L'osservazione decisiva è che tre cose che sembravano appartenere a capacità diverse
sono **lo stesso oggetto**:

| Capacità | "Sensore" |
|---|---|
| coding | linter, type checker, esecuzione dei test |
| generazione asset | validazione della mesh (buchi, artefatti, topologia) prima dell'export |
| conoscenza / RAG | verifica che ogni affermazione abbia una citazione risolvibile |

Tutti e tre osservano un artefatto e producono un verdetto. Implementarli tre volte
dentro tre capacità significherebbe tre vocabolari, tre modi di riportare gli esiti e
nessun anello di miglioramento unificato — oltre a violare la parità di
[ADR-0001](0001-architettura-a-kernel-con-capacita-paritarie.md).

Alternative considerate:

- **Sensori come dettaglio interno di ciascuna capacità.** Nessuna astrazione da
  progettare. *Contro:* triplicazione, e l'anello di miglioramento diventa
  impossibile perché non esiste un formato comune di verdetto.
- **Registro di kernel con contratto comune.** *Contro:* rischio di astrazione
  prematura se i tre casi divergono più di quanto sembri.

## Decision

Il kernel espone quattro meccanismi generici. Le capacità portano il contenuto.

| Meccanismo | Il kernel fornisce | La capacità porta |
|---|---|---|
| **Registro delle guide** | archiviazione, versionamento, iniezione nella proiezione | quali guide, per quale contesto |
| **Registro dei sensori** | esecuzione, classificazione per costo, verdetti nel giornale | quali sensori, quale rubrica |
| **Anello di verifica** | dopo un passo, esegue i sensori applicabili; un verdetto negativo rientra nell'anello come feedback | la soglia di accettazione |
| **Trigger** | l'anello parte da eventi — pianificazione, cambiamento di file, fine di un'altra run — non solo dall'utente | quali eventi le interessano |

**Contratto minimo del sensore:** `(artefatto) → (verdetto, dettaglio, costo)`.
Deliberatamente povero: tutto ciò che è specifico resta dentro l'implementazione. È
la mitigazione contro l'astrazione prematura — un contratto minimo si può allargare,
uno ricco e sbagliato no.

**Anello di miglioramento.** Adottiamo la regola dell'harness engineering: *quando un
problema si ripete, si migliora il controllo, non il prompt*. Il giornale contiene già
i dati necessari; il kernel rileva la ricorrenza e **propone** una guida o un sensore
nuovo. **L'utente approva.** Non si auto-modifica in silenzio.

**Le skill dichiarative di [ADR-0003](0003-estensibilita-solo-mcp-e-skill-dichiarative.md)
trovano qui la loro collocazione: sono guide.** Il meccanismo esisteva già senza nome.

## Consequences

- **Positive:**
  - Un solo vocabolario per linter, test, validazione mesh e verifica delle
    citazioni. Aggiungere un sensore a una capacità non richiede infrastruttura nuova.
  - L'anello di miglioramento poggia su un substrato che esiste già (il giornale):
    non serve un secondo sistema di osservabilità.
  - Le regole di progetto della mappa funzionale diventano guide registrate, non
    testo iniettato a mano.
- **Negative (accettate):**
  - **Rischio di astrazione prematura.** Se i sensori delle tre aree divergono più
    del previsto, il contratto comune diventa un vincolo. Mitigato dal contratto
    minimo, ma non eliminato: va rivisto dopo il secondo sensore reale.
  - I sensori inferenziali costano token e tempo. Devono essere classificati per
    costo ed esclusi dall'anello stretto.
  - L'anello di miglioramento aggiunge una fonte di proposte che l'utente deve
    valutare: mal calibrato, diventa rumore.
- **Follow-up richiesti:**
  - Adottare il principio *"tieni la qualità a sinistra"*: i sensori si ordinano per
    costo e velocità — i più economici dentro l'anello di verifica, i costosi a valle.
  - Rivedere il contratto del sensore dopo che ne esistono due reali in aree diverse.
    Se non si adatta, è il momento di spezzarlo, non di piegarlo.
