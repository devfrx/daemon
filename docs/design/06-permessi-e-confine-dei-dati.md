# Permessi e confine dei dati non fidati

Come il contenuto esterno attraversa il sistema, e cosa serve per agire.
Fonte di verità su chi può fare cosa e su cosa non può mai diventare un'autorizzazione.

Decisioni: [ADR-0014](../adr/0014-confine-dei-dati-non-fidati-nel-sistema-di-tipi.md) ·
[ADR-0015](../adr/0015-descrizioni-degli-strumenti-fissate-all-approvazione.md) ·
[ADR-0016](../adr/0016-permessi-granulari-e-default-dei-vincoli-sui-dati.md).

## I due canali

```mermaid
flowchart TB
    subgraph FID["CANALE ISTRUZIONI — fidato"]
        U["utente"]
        G["guide registrate<br/>skill dichiarative, regole"]
        K["prompt di sistema<br/>del kernel"]
    end

    subgraph NF["CANALE DATI — non fidato"]
        W["pagine web · ricerche"]
        T["output degli strumenti"]
        D["documenti · PDF · OCR"]
        F["file letti dall agente"]
        M["descrizioni MCP"]
        P["risposte dei provider"]
        V["trascrizioni vocali"]
    end

    FID --> PR["PROIEZIONE"]
    NF -->|"etichettato, ereditario"| PR
    PR --> A{"azione con effetto?"}
    A -->|"decisione dipende<br/>SOLO dal canale fidato"| OK["procede secondo i permessi"]
    A -->|"decisione dipende<br/>da contenuto NON fidato"| ASK["richiede la stessa autorizzazione<br/>che servirebbe senza richiesta"]

    classDef fid fill:#0f766e,stroke:#134e4a,color:#fff
    classDef nfid fill:#b45309,stroke:#78350f,color:#fff
    class U,G,K fid
    class W,T,D,F,M,P,V nfid
```

**Entrambi i canali raggiungono il modello.** Non è una difesa contro l'inganno: è
una difesa contro l'**escalation di privilegio**. Il modello può essere convinto di
qualsiasi cosa; ciò che non può è convertire quella convinzione in autorizzazione.

## Ereditarietà dell'etichetta

| Operazione su contenuto non fidato | Risultato |
|---|---|
| estrazione, ritaglio | non fidato |
| riassunto | non fidato |
| traduzione | non fidato |
| concatenazione con contenuto fidato | **non fidato** (il peggiore vince) |
| conversione esplicita a istruzione | **evento giornalato**, mai implicito |

Senza ereditarietà basterebbe un riassunto per ripulire un attacco.

## Ciclo di approvazione di uno strumento di terze parti

```mermaid
stateDiagram-v2
    [*] --> Proposto : server MCP installato

    Proposto --> Approvato : utente legge la descrizione integrale e accetta
    Proposto --> Rifiutato : utente rifiuta

    Approvato --> Sospeso : impronta della descrizione cambiata
    Sospeso --> Approvato : ri-approvazione, con diff mostrato
    Sospeso --> Rifiutato : utente rifiuta la nuova versione

    Approvato --> Revocato : utente revoca
    Rifiutato --> [*]
    Revocato --> [*]

    note right of Sospeso
        Sospeso, non degradato e
        non avvisato-e-usato: e la
        difesa contro il rug pull.
    end note
```

| Regola | Motivo |
|---|---|
| Si mostra la **descrizione integrale**, non il nome | l'utente deve valutare il testo che influenzerà il modello |
| L'impronta è fissata all'approvazione | un cambiamento successivo è rilevabile |
| Una descrizione **non concede permessi** | i permessi vengono solo dalla tripla, mai da un metadato |

## Permessi

| Componente | Esempi |
|---|---|
| **strumento** | file, shell, rete, strumento MCP `x` |
| **risorsa** | un percorso, una allow-list di comandi, un host |
| **operazione** | lettura, scrittura, esecuzione, uscita |

| Preset | Procede senza chiedere | Chiede |
|---|---|---|
| `chiede sempre` | nulla | ogni azione con effetto |
| **`auto-approva sicuri`** *(default)* | letture, test, build | scritture, comandi, uscite di rete |
| `autonomo` | quasi tutto | effetti `irripetibili` (§4) · azioni fermate da un sensore |

**Un'approvazione non si estende**: vale per la tripla concessa e per la sessione
corrente. `~/progetti/x` non implica `~/progetti/y`, e oggi non implica domani.

## Vincoli sui dati: default ed escalation

```mermaid
flowchart TD
    R["richiesta"] --> P["default del profilo<br/>di configurazione attivo"]
    P --> S{"il contenuto ha attraversato<br/>il gestore dei segreti?"}
    S -->|no| N["vale il default del profilo"]
    S -->|si| E["ESCALATION AUTOMATICA<br/>classe piu stretta"]
    E --> C{"esiste endpoint conforme?"}
    C -->|si| OK["procede"]
    C -->|no| FC["FALLISCE CHIUSO"]

    classDef bad fill:#b45309,stroke:#78350f,color:#fff
    class FC,E bad
```

Il default resta usabile, ma la regola scatta sempre dove conta. **Falla nota:** un
segreto incollato a mano in chat non attraversa il gestore e aggira l'escalation.

## Canary di esfiltrazione

| Aspetto | |
|---|---|
| Cos'è | valori sentinella nel gestore dei segreti |
| Come funziona | la loro comparsa in contenuto in uscita è un **verdetto di sensore** (§5): blocca e segnala |
| Cosa copre | esfiltrazione dei segreti **noti** |
| Cosa **non** copre | dati sensibili generici. È una rete, non un muro |

## Regole che i diagrammi non esprimono

- **Non esiste sanitizzazione.** Non si tenta di rimuovere istruzioni dal testo: si
  impedisce che diventino autorizzazioni.
- Il record di routing (§3) non contiene **mai** credenziali (V16).
- La provenienza deve essere **visibile nell'interfaccia**: se l'utente non vede da
  dove viene un contenuto, approva alla cieca e la difesa collassa sull'anello umano.
- Il modo di fallire più probabile di questa sezione non è tecnico: è **l'utente che
  approva per stanchezza**. I preset lo riducono, non lo eliminano.
