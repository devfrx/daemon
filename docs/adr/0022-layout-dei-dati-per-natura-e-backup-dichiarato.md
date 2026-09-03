# ADR-0022: Layout dei dati per natura, e backup del solo irriproducibile

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

[ADR-0007](0007-giornale-write-ahead-e-riconciliazione.md) e
[ADR-0018](0018-ritenzione-a-livelli-del-giornale.md) decidono la **semantica** della
persistenza — write-ahead, riconciliazione, potatura — ma mai il **supporto fisico**.
È la lacuna L-1/L-2/L-3 della tracciabilità.

Il sistema produce dati di natura profondamente diversa, e le differenze non sono
cosmetiche: cambiano ritenzione, cifratura e riproducibilità.

| Dato | Ricostruibile? | Contiene contenuto sensibile? | Cresce come |
|---|---|---|---|
| giornale | **no** | sì (prompt, risposte) | numero di passi |
| artefatti prodotti | no | sono file dell'utente | uso |
| configurazione, guide, profili | no | no | lentamente |
| indici e embedding | **sì**, dai documenti | derivato | dimensione della base |
| pesi dei modelli locali | **sì**, riscaricabili | no | decine di GB |
| segreti | **no**, ma re-inseribili | **sì, massimamente** | trascurabile |

Alternative considerate:

- **Archivio unico indifferenziato.** Semplice.
  *Contro:* ritenzione, cifratura e backup diventano uniformi. O si cifra tutto —
  pagando su indici che possono essere enormi e su pesi che non ne hanno bisogno — o
  non si cifra nulla. E il backup finisce per trascinarsi decine di GB riscaricabili.
- **Separazione per componente** (uno per capacità). *Contro:* violerebbe la parità di
  ADR-0001 e frammenterebbe dati della stessa natura.
- **Separazione per natura.** *Contro:* più archivi da gestire.

## Decision

**1. I dati sono separati per natura, non per componente.** Ogni archivio ha la
propria politica di ritenzione, cifratura e backup.

| Archivio | Cifrato | Nel backup | Ritenzione |
|---|---|---|---|
| giornale | **sì** | sì | struttura lunga, payload potati (ADR-0018) |
| artefatti | no — sono già file dell'utente | sì | riferiti dal giornale |
| configurazione, guide, profili | no | sì | permanente |
| indici ed embedding | no | **no** | rigenerabile |
| pesi dei modelli locali | no | **no** | gestione dedicata |
| segreti | **sì, con chiave propria** | **mai** | permanente |

**2. Il backup contiene solo l'irriproducibile.** Indici, pesi e cache sono esclusi
perché ricostruibili; i segreti sono esclusi perché **un backup che trasporta chiavi
API è un vettore di fuga**, non una comodità.

**3. Requisiti del motore di persistenza** — requisiti, non scelta:

| # | Requisito | Da |
|---|---|---|
| 1 | scrittura durevole e ordinata, con conferma prima dell'esecuzione | ADR-0007, V6 |
| 2 | lettura concorrente mentre si scrive | GUI + core + proiezioni |
| 3 | potatura selettiva senza riscrivere l'archivio | ADR-0018 |
| 4 | ogni operazione di I/O **iniettabile** | V29 · ADR-0021 |

La scelta concreta del motore è un **ADR successivo**, dopo quello sul linguaggio del
core: dipende da cosa quell'ecosistema offre.

> ✅ **Rimando — il motore è scelto:** [ADR-0032](0032-motore-di-persistenza.md), `redb`
> 4.1.0 con `StorageBackend` scritto da noi. Il requisito che ha deciso è il **4**, l'I/O
> iniettabile: è l'unico che i candidati non esponevano tutti, ed è il punto in cui il
> secondo livello di crash diventa iniettabile.

## Consequences

- **Positive:**
  - Il backup resta piccolo e onesto: contiene ciò che davvero non si può rifare.
  - Cifratura e ritenzione si applicano dove servono, senza costo dove non servono.
  - La base di conoscenza sopravvive alla reinstallazione perché i documenti sorgente
    e la configurazione sono nel backup, e l'indice si ricostruisce.
- **Negative (accettate):**
  - **Ripristinare non è istantaneo**: gli indici vanno ricostruiti, i pesi
    riscaricati, i segreti re-inseriti a mano.
  - Più archivi significano più codice di gestione e più modi di essere incoerenti fra
    loro dopo un crash — mitigato dal fatto che solo il giornale è autorevole (I1).
- **Follow-up richiesti:**
  - L'interfaccia deve dichiarare **cosa il backup non contiene al momento in cui lo
    si crea**, non al momento del ripristino. Scoprirlo mentre si ripristina è il modo
    peggiore di scoprirlo.

> 📌 **Rimando — dove vive questa decisione, aggiunto il 2026-08-07.** Le due metà di
> questo ADR si realizzano in **due momenti diversi**, e confonderle è ciò che aveva
> lasciato il backup senza proprietario:
>
> | Metà | Quando |
> |---|---|
> | il **layout per natura** — archivi separati, ciascuno con la propria politica | **già rispettato** nel sotto-progetto 1: giornale e segreti nascono come archivi distinti, per V34 e per la struttura delle crate |
> | **backup e ripristino**, con V32, V33 e Q21 | **sotto-progetto 11** della [roadmap](../roadmap.md), dopo il 5, il 6 e il 9 |
>
> L'ordine non è comodità. V32 esclude *indici e pesi perché ricostruibili*: finché il
> sotto-progetto 6 non produce indici e il 9 non porta pesi, l'elenco delle esclusioni è
> **vuoto**, e verificarlo su un elenco vuoto è una prova che non può fallire. Il
> follow-up qui sopra — dichiarare le esclusioni *al momento del backup* — è la ragione
> per cui serve anche un'interfaccia.
>
> La lacuna era rimasta invisibile fino alla §8 della
> [spec del sotto-progetto 1](../superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md),
> che ha dovuto assegnare un innesco a V32, V33 e Q21 e non ha trovato nessun destinatario.
