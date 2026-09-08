# Osservabilità, errori e degrado

Tassonomia degli errori, stato di degrado, proiezioni del giornale.
Fonte di verità su cosa il sistema sa di sé e su cosa ne mostra.

Decisioni: [ADR-0017](../adr/0017-giornale-sorgente-trace-proiezione.md) ·
[ADR-0018](../adr/0018-ritenzione-a-livelli-del-giornale.md) ·
[ADR-0019](../adr/0019-lo-stato-di-degrado-e-un-oggetto-osservabile.md).

⚠️ **RICHIAMO DEL 2026-09-08 — la sezione 2 della passata sui diagrammi della stella polare della GUI.**
Il diagramma dello stato di degrado, la tabella delle condizioni, il diagramma delle proiezioni e la
tabella di ciò che è sempre visibile sono riscritti contro il codice e le decisioni di oggi: le fonti del
degrado sono **sette** e non cinque — il fallback dichiarato di ADR-0012 c'è già nel codice
(`routing_degraded`), la telecamera arriva con ADR-0039 — e ADR-0019 riceve il rimando; la GUI resta viva
anche a GPU satura (ADR-0033); le proiezioni del giornale sono classi e non un elenco di sei; la striscia
del 2 mostra solo ciò che è vivo (decisione 16 del coordinatore). Una voce segnata «(col N)» è decisa e
la costruisce il sotto-progetto N; «oggi» dice che esiste nel codice. Il perché sta nella
[stella polare](../superpowers/specs/2026-09-07-direzione-gui-design.md), sezione 2 della passata,
decisione 18.

## Tassonomia degli errori

Ogni classe ha già un meccanismo, deciso in una sezione precedente. Nessun errore
richiede un percorso nuovo: è la verifica più forte che il design regga.

| Classe | Esempi | Meccanismo | Deciso in |
|---|---|---|---|
| **transitorio** | 5xx, limite di frequenza, timeout senza output | ritentativo nello stesso passo | §3 · V17 |
| **di risorsa** | VRAM insufficiente, GPU occupata | coda, oppure fallback al candidato successivo | §2 · §3 |
| **di vincolo** | nessun endpoint conforme ai vincoli sui dati | **fallisce chiuso** | §3 · ADR-0012 |
| **di autorizzazione** | permesso mancante, strumento sospeso | sospende e chiede | §6 |
| **di verifica** | verdetto negativo di un sensore | rientra nell'anello, passo nuovo | §5 · V14 |
| **di dubbio** | passo `InDubbio` dopo un crash | riconciliazione per classe di effetto | §4 · ADR-0007 |
| **di autonomia** | tetto di passi, tempo o costo superato | `AttesaUmano` + notifica | §4 · V8, V9 |
| **definitivo** | invariante violata, dato corrotto, difetto | fallisce e si dichiara; nessun ripiego | — |

Solo l'ultima riga non ha un meccanismo di recupero, ed è corretto: un'invariante
violata è un difetto del sistema, non una condizione da gestire.

## Stato di degrado

```mermaid
flowchart LR
    E1["connettivita (col 3)"] --> S
    E2["arbitro GPU<br/>oggi: vram_exhausted"] --> S
    E3["salute dei provider (col 3)"] --> S
    E4["permessi (col 4)"] --> S
    E5["strumenti sospesi (col 4)"] --> S
    E6["fallback dichiarato, ADR-0012<br/>oggi: routing_degraded"] --> S
    E7["telecamera (col 12)"] --> S
    S["STATO DI DEGRADO<br/>derivato, ricalcolabile<br/>mai autorevole di per se<br/>oggi: degradation_now, dal giornale"]
    S --> U["interfaccia:<br/>cosa e disponibile ORA<br/>la striscia e il modulo Stato (col 2)"]
    S --> C["capacita:<br/>si adattano invece di fallire"]
    S --> M["metrica:<br/>quanto tempo in stato parziale"]

    classDef der fill:#0f766e,stroke:#134e4a,color:#fff
    class S der
```

Oggi il codice ne deriva **due** campi, `vram_exhausted` e `routing_degraded`, e dichiara nel loro doc
che connettività e salute dei provider non hanno ancora una fonte — nessun campo aspetta fingendo
«va tutto bene» (`crates/kernel/src/degradation.rs`). Il fallback dichiarato di ADR-0012 è una fonte
che ADR-0019 non elencava: il rimando in testa a quell'ADR lo dice.

| Condizione | Resta disponibile | Cade |
|---|---|---|
| **offline** | inferenza locale, RAG locale, generazione asset, voce | OpenRouter, ricerca web |
| **GPU satura** | tutto ciò che è remoto; **voce** (quota riservata, §2); **la GUI** (quota di presentazione, ADR-0033) | inferenza locale, generazione asset, il viewer 3D oltre la quota |
| **provider indisponibile** | fallback della catena; locale se configurato | quel provider |
| **modello locale scaricato** | tutto, con avvio a freddo dichiarato (Q8) | latenza del primo token |
| **strumento MCP sospeso** | tutto il resto | quello strumento, fino a ri-approvazione (§6) |
| **telecamera indisponibile** (col 12) | tutto il resto | il tracciamento delle mani e i gesti; l'indicatore lo dice (ADR-0039) |

**Si dichiara prima, non si fallisce dopo.** Nessuna azione deve fallire per una
condizione che era già nota e non era stata mostrata.

## Il giornale e le sue proiezioni

```mermaid
flowchart LR
    J[("GIORNALE<br/>sorgente unica di verita")]

    J --> R["ripresa<br/>riconciliazione (§4)"]
    J --> P["proiezione di contesto<br/>(§4, col 13)"]
    J --> K["cio che il core sa di se<br/>degrado e permessi (oggi)<br/>policy VRAM corrente (col 2)<br/>guide approvate (col 13)"]
    J --> G["cio che la GUI mostra<br/>Passi (col 2) · Attivita (col 3)<br/>mai uno stato suo (I1)"]
    J --> T["trace<br/>vocabolario OTel GenAI"]
    J --> C["contabilita<br/>token, costi, tetti"]
    J --> M["metriche<br/>latenza, esiti, qualita"]
    J --> D["dataset di regressione<br/>dai fallimenti (anello 4)"]

    T -.->|"esportazione OPT-IN<br/>disattivata per default"| X["backend OTLP esterno"]

    classDef src fill:#1d4ed8,stroke:#1e3a8a,color:#fff
    classDef out fill:#b45309,stroke:#78350f,color:#fff
    class J src
    class X out
```

**Un substrato, molte viste.** Il giornale nasce per la ripresa dopo crash (§4); tutto
il resto sono proiezioni, e la regola non ha un numero: ciò che il core sa di sé — degrado,
permessi, policy VRAM corrente, guide approvate — e ciò che la GUI mostra si rilegge dal
giornale, mai da un secondo archivio (I1, ADR-0009); oggi lo fanno `degradation_now` e
`is_granted`. Il vocabolario OpenTelemetry GenAI si applica alla **proiezione
trace**, non all'archiviazione: se la convenzione cambia — ed è ancora pre-stabile —
cambia la proiezione, non i dati.

## Ritenzione

| Livello | Contenuto | Ritenzione |
|---|---|---|
| **struttura** | identità, transizioni, esiti, routing, costi, verdetti, decisioni | lunga; è la parte piccola |
| **payload** | prompt, risposte, output degli strumenti, trascrizioni | finestra breve → potati, sostituiti da impronta e dimensione |
| **artefatti** | file prodotti | **riferimenti**: il contenuto vive sul filesystem |

| Regola | Motivo |
|---|---|
| Un record potato **dichiara** di esserlo | payload assente e payload mai registrato non devono confondersi. ⚠️ **Rimando del 2026-08-27:** non tenuta dal codice — vedi il rimando in [ADR-0018](../adr/0018-ritenzione-a-livelli-del-giornale.md) |
| Un passo `InDubbio` **non è potabile** | la riconciliazione può dipendere dal payload. ⚠️ **Rimando del 2026-08-27:** la porta la tiene con un'altra nozione di dubbio, e le due divergono — stesso rimando in [ADR-0018](../adr/0018-ritenzione-a-livelli-del-giornale.md) |
| I fallimenti candidati a regressione si **promuovono** prima della potatura | potare un fallimento non sfruttato butta via il dato più prezioso |

È la stessa gerarchia della compattazione del contesto (§4): ciò che è strutturato
sopravvive, il grezzo si sacrifica.

## Cosa deve essere sempre visibile

| Elemento | Perché | Vincolo | Dove | Chi |
|---|---|---|---|---|
| stato di degrado corrente | si dichiara prima, non si fallisce dopo | V27 · G9 | la striscia, e il modulo Stato per intero | 2 |
| permessi attivi nella sessione | un permesso concesso e dimenticato è indistinguibile da uno mai concesso | V21 · §6 · G10 | la striscia, e il modulo Permessi | 2; il confine di sessione col 3 |
| occupazione del contesto **per categoria** | senza misura è un'impressione | §5 · ADR-0010 · G11 | la striscia; per run, nella barra della chat | 3, col dato dal 13 |
| costo corrente e distanza dal tetto | i tetti sospendono: l'utente deve vederli arrivare | §3 · V8 · G12 | la striscia, e il modulo Costi | 3 |
| provenienza del contenuto | senza, si approva alla cieca | V23 · §6 · G13 | nel flusso, su ogni pezzo — non nella striscia | 2 |
| run in `AttesaUmano` | una run bloccata in silenzio è indistinguibile da una morta | V9 · G14 | la striscia (attese), e Attività | 3 |
| telecamera accesa dal core | i fotogrammi non escono mai: l'indicatore è l'unica prova che è accesa | ADR-0039 | la striscia | 12 |
| microfono acceso dal core | la voce always-on si vede, e «riservato» la spegne | ADR-0023 · riga «Controlli di privacy del microfono» di tracciabilità | la striscia | 8 |

⚠️ **Richiamo del 2026-09-08:** «sempre visibile» è del prodotto, non del 2: nella cornice del 2 la
striscia mostra solo ciò che è vivo — degrado e permessi — e ogni altra voce la porta il modulo che la
riempie, col suo numero (stella polare, decisione 16 del coordinatore). Le colonne «Dove» e «Chi» vengono
dal modello della GUI approvato il 2026-09-07 e dal catalogo dei moduli.

## Regole che i diagrammi non esprimono

- **Nessuna telemetria lascia la macchina per default.** L'esportazione è opt-in e la
  destinazione la sceglie l'utente. C'è **un solo punto di uscita**, il che rende la
  promessa verificabile invece che dichiarata.
- Prima dell'esportazione si applica la mascheratura dei segreti (V16).
- La proiezione trace dichiara **quale versione** della convenzione emette: un trace
  senza versione, in uno standard che cambia, è ambiguo.
- Lo stato di degrado è **derivato**: se diverge, si ricalcola. Non è mai la verità.
- Si mostra ciò che **cambia cosa l'utente può fare**, non ogni variazione interna:
  un'interfaccia che segnala tutto è indistinguibile da una che non segnala nulla.
