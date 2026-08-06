# L0 fisico — archivi, chiavi, checkpoint, confinamento

Dove finiscono i byte, chi possiede le chiavi, cosa si può annullare e come si
confina un processo. Fonte di verità sul supporto fisico della persistenza.

Decisioni: [ADR-0022](../adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md) ·
[ADR-0023](../adr/0023-cifratura-a-riposo-e-gestore-dei-segreti.md) ·
[ADR-0024](../adr/0024-checkpoint-del-filesystem-ad-ambiti-dichiarati.md) ·
[ADR-0025](../adr/0025-confinamento-a-livelli.md).

## Gli archivi

```mermaid
flowchart TB
    subgraph CIF["cifrati"]
        G[("giornale<br/>run, passi, routing<br/>verdetti, costi")]
        S[("segreti<br/>chiave propria")]
    end
    subgraph CHI["in chiaro"]
        C[("configurazione<br/>profili, guide, policy")]
        A[("artefatti<br/>file prodotti")]
        I[("indici<br/>embedding, RAG")]
        M[("pesi dei<br/>modelli locali")]
    end

    G --> B{{"BACKUP"}}
    C --> B
    A --> B
    I -.->|"escluso:<br/>si ricostruisce"| B
    M -.->|"escluso:<br/>si riscarica"| B
    S -.->|"MAI:<br/>vettore di fuga"| B

    classDef enc fill:#1d4ed8,stroke:#1e3a8a,color:#fff
    classDef plain fill:#0f766e,stroke:#134e4a,color:#fff
    class G,S enc
    class C,A,I,M plain
```

| Archivio | Cifrato | Nel backup | Ricostruibile |
|---|---|---|---|
| giornale | **sì** | sì | no |
| segreti | **sì**, chiave propria | **mai** | no, ma re-inseribili |
| configurazione, guide, profili | no | sì | no |
| artefatti prodotti | no — sono già file dell'utente | sì | no |
| indici ed embedding | no | **no** | sì, dai documenti |
| pesi dei modelli locali | no | **no** | sì, riscaricabili |

**Il backup contiene solo l'irriproducibile.** Un backup che trascina decine di GB di
pesi riscaricabili non viene fatto; uno che trasporta chiavi API è un vettore di fuga.

## Chiavi e segreti

```mermaid
flowchart LR
    OS[("facility dell OS<br/>chiave mai esposta<br/>all applicazione")] -->|"via modulo<br/>di piattaforma (I3)"| K["chiave di cifratura"]
    K --> G[("giornale")]
    K --> S[("segreti")]

    S --> GS["GESTORE DEI SEGRETI<br/>unico punto di lettura"]
    GS --> M1["mascheratura nel<br/>record di routing (V16)"]
    GS --> M2["escalation dei vincoli<br/>sui dati (ADR-0016)"]
    GS --> M3["canary di<br/>esfiltrazione"]

    classDef uniq fill:#1d4ed8,stroke:#1e3a8a,color:#fff
    class GS uniq
```

I tre meccanismi a destra funzionano **perché** esiste un punto unico di lettura. Con
credenziali leggibili da più punti nessuno dei tre sarebbe verificabile.

### Cosa significa davvero «cifrato a riposo», qui

| Protegge da | **Non** protegge da |
|---|---|
| disco letto da un altro account | chi ha già il tuo account di sistema |
| copia dei file fatta da un altro utente | malware che gira come te |

La chiave è protetta dalle credenziali di accesso dell'utente. **La sicurezza dei dati
equivale a quella dell'account OS**, e questa frase va in interfaccia — «cifrato»
suona più forte di quanto sia, e una falsa sicurezza è peggio di nessuna sicurezza.

### La composizione mutuamente esclusiva

| Profilo | Chiave | Avvio automatico | Voce always-on |
|---|---|---|---|
| **normale** *(default)* | facility dell'OS | ✅ | ✅ |
| **riservato** | passphrase all'avvio | ❌ | ❌ |

Non si possono avere entrambe. Nel profilo riservato il sistema **rifiuta** di
abilitare l'avvio automatico, non si limita a sconsigliarlo.

## Checkpoint del filesystem

```mermaid
stateDiagram-v2
    [*] --> Dichiarato : ambito di lavoro definito

    Dichiarato --> Conservato : un passo sta per toccare un file dentro l ambito
    Conservato --> Riferito : la versione precedente e legata al passo N
    Riferito --> Dichiarato : passo successivo

    Riferito --> Ripristinato : richiesta di rollback al passo N
    Ripristinato --> Dichiarato : l ambito torna allo stato precedente

    Riferito --> Potato : oltre la finestra di ritenzione

    note right of Conservato
        Si conserva PRIMA, come il
        write-ahead del giornale:
        dopo e troppo tardi.
    end note
```

| | Checkpoint | Versionamento (git) |
|---|---|---|
| Quando | **automatico**, a ogni passo | intenzionale, a ogni commit |
| Grana | il passo | il commit |
| Copre file non versionati | sì | no |
| Richiede un repository | no | sì |

Convivono: non si sostituiscono.

**Limite dichiarato:** gli effetti **fuori** dagli ambiti non sono coperti. Per la §4
restano effetti `verificabili` o `irripetibili`, quindi soggetti ad approvazione (§6).
L'interfaccia deve mostrare cosa è coperto **prima** che l'agente inizi a scrivere.

## Confinamento

| Livello | Confine | Garantito da | Regge contro codice eseguito? |
|---|---|---|---|
| **0** | nessuno | — | no |
| **1** | permessi applicativi (tripla §6) | il kernel media ogni accesso | **no** |
| **2** | processo ristretto dell'OS | primitive di sistema | sì |
| **3** | macchina virtuale leggera | hypervisor | sì, anche a fuga dal kernel guest |

```mermaid
flowchart TD
    A["azione da eseguire"] --> L{"livello richiesto"}
    L -->|"1 - non esegue codice"| OK1["procede sotto mediazione del kernel"]
    L -->|"2 o 3"| D{"il modulo di piattaforma<br/>lo fornisce qui?"}
    D -->|si| OK2["procede confinata<br/>livello registrato nel giornale"]
    D -->|no| FC["NON PARTE<br/>fail-closed"]

    classDef bad fill:#b45309,stroke:#78350f,color:#fff
    class FC bad
```

**Default: livello 2 minimo per qualsiasi esecuzione di codice generato o di comando.**

Il livello 1 non è un confine contro codice eseguito, ed è l'illusione più pericolosa
del capitolo sicurezza: un processo figlio non passa dal mediatore e può aprire ciò
che l'utente può aprire.

**Un confinamento più debole di quello richiesto non è un ripiego: è un'altra cosa.**
Perciò l'azione non parte — su una piattaforma non ancora supportata l'app non
«funziona a metà», rifiuta di eseguire.

## Regole che i diagrammi non esprimono

- Solo il **giornale** è autorevole (I1). Gli altri archivi possono essere ricostruiti
  o rifiutati; nessuno di essi è la verità.
- Il **livello di confinamento usato** entra nel giornale insieme al passo: senza, non
  si può stabilire a posteriori in quali condizioni un comando è stato eseguito.
- Il checkpoint pota con la stessa logica a livelli del giornale (ADR-0018), e un file
  troppo grande viene **escluso con avviso**, non silenziosamente.
- Ogni operazione di I/O di questo strato resta **iniettabile** (V29): è lo strato che
  la simulazione deterministica deve poter sostituire per intero.
