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

⚠️ **RICHIAMO DEL 2026-10-06** — audit del 2026-09-30, AUD-023, AUD-057, AUD-128, AUD-208, AUD-248, AUD-249,
AUD-250, AUD-252, AUD-253, AUD-254, AUD-255, AUD-256, AUD-257, AUD-262, AUD-267 e AUD-288: la striscia, il
modulo Stato, la policy VRAM corrente e il modulo Passi esistono, dal sotto-progetto 2; ogni altro pezzo deciso e
non costruito porta il segno «(col N)» della regola 2 del [README](../README.md), e chi costruisce l'esportazione
OTLP è una scelta aperta, registrata fra quelle del proprietario nella
[stella polare](../superpowers/specs/2026-09-07-direzione-gui-design.md).

## Tassonomia degli errori

Ogni classe ha già un meccanismo, deciso in una sezione precedente. Nessun errore
richiede un percorso nuovo: è la verifica più forte che il design regga.

| Classe | Esempi | Meccanismo | Deciso in |
|---|---|---|---|
| **transitorio** | 5xx, limite di frequenza, timeout senza output | ritentativo nello stesso passo (col 3) | §3 · V17 |
| **di risorsa** | VRAM insufficiente, GPU occupata | coda, oppure fallback al candidato successivo (col 3) | §2 · §3 |
| **di vincolo** | nessun endpoint conforme ai vincoli sui dati | **fallisce chiuso** | §3 · ADR-0012 |
| **di autorizzazione** | permesso mancante, strumento sospeso (col 4) | sospende (col 3) e chiede | §6 |
| **di verifica** | verdetto negativo di un sensore | rientra nell'anello, passo nuovo | §5 · V14 |
| **di dubbio** | passo `InDubbio` dopo un crash | riconciliazione per classe di effetto: oggi la risoluzione, `steps_in_doubt`; chi la esegue col 3 | §4 · ADR-0007 |
| **di autonomia** (col 3) | tetto di passi, tempo o costo superato | `AttesaUmano` + notifica | §4 · V8, V9 |
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
    S --> U["interfaccia:<br/>cosa e disponibile ORA<br/>la striscia e il modulo Stato"]
    S --> C["capacita (col 3 e seguenti):<br/>si adattano invece di fallire"]
    S --> M["metrica (col il primo che la usa):<br/>quanto tempo in stato parziale"]

    classDef der fill:#0f766e,stroke:#134e4a,color:#fff
    class S der
```

Oggi il codice ne deriva i campi `vram_exhausted` e `routing_degraded`, e dichiara nel suo doc — quello
di `Degradation` — che connettività e salute dei provider (col 3), permessi e strumenti sospesi (col 4)
non hanno ancora una fonte: nessun campo aspetta fingendo «va tutto bene»
(`crates/kernel/src/degradation.rs`). Il fallback dichiarato di ADR-0012 è una fonte che ADR-0019 non
elencava: il rimando in testa a quell'ADR lo dice.

| Condizione | Resta disponibile | Cade |
|---|---|---|
| **offline** (col 3) | inferenza locale (col 9), RAG locale (col 6), generazione asset (col 7), voce (col 8) | OpenRouter, ricerca web (col 6) |
| **GPU satura** | tutto ciò che è remoto (col 3); **voce** (col 8, quota riservata, §2); **la GUI** (quota di presentazione, ADR-0033) | inferenza locale (col 9), generazione asset e il viewer 3D oltre la quota (col 7) |
| **provider indisponibile** (col 3) | fallback della catena; locale se configurato (col 9) | quel provider |
| **modello locale scaricato** (col 9) | tutto, con avvio a freddo dichiarato (Q8) | latenza del primo token |
| **strumento MCP sospeso** (col 4) | tutto il resto | quello strumento, fino a ri-approvazione (§6) |
| **telecamera indisponibile** (col 12) | tutto il resto | il tracciamento delle mani e i gesti; l'indicatore lo dice (ADR-0039) |

**Si dichiara prima, non si fallisce dopo.** Nessuna azione deve fallire per una
condizione che era già nota e non era stata mostrata.

## Il giornale e le sue proiezioni

```mermaid
flowchart LR
    J[("GIORNALE<br/>sorgente unica di verita")]

    J --> R["ripresa: riconciliazione (§4)<br/>oggi la risoluzione, steps_in_doubt<br/>chi la esegue col 3"]
    J --> P["proiezione di contesto<br/>(§4, col 13)"]
    J --> K["cio che il core sa di se<br/>oggi: degrado, permessi,<br/>policy VRAM corrente<br/>col 13 le guide approvate"]
    J --> G["cio che la GUI mostra<br/>oggi i Passi · col 3 Attivita<br/>mai uno stato suo (I1)"]
    J --> T["trace (col 3)<br/>vocabolario OTel GenAI"]
    J --> C["contabilita (col 3)<br/>token, costi, tetti"]
    J --> M["metriche (col il primo che le usa)<br/>latenza, esiti, qualita"]
    J --> D["dataset di regressione dai fallimenti<br/>la promozione col 15<br/>l anello 4 che li legge col 4"]

    T -.->|"esportazione OPT-IN<br/>disattivata per default<br/>chi la costruisce: scelta aperta"| X["backend OTLP esterno"]

    classDef src fill:#1d4ed8,stroke:#1e3a8a,color:#fff
    classDef out fill:#b45309,stroke:#78350f,color:#fff
    class J src
    class X out
```

**Un substrato, molte viste.** Il giornale nasce per la ripresa dopo crash (§4); tutto
il resto sono proiezioni, e la regola non ha un numero: ciò che il core sa di sé — degrado,
permessi, policy VRAM corrente, guide approvate — e ciò che la GUI mostra si rilegge dal
giornale, mai da un secondo archivio (I1, ADR-0009). Oggi lo fanno `degradation_now`,
`is_granted` e `arbiter::policy_now`, e per i Passi `Core::step_list` di `kernel::serving`; chi
rilegge il giornale, col comando che li trova, lo censisce
[design/10](10-modello-dei-dati-durevoli.md). Il vocabolario OpenTelemetry GenAI si applica alla
**proiezione trace**, non all'archiviazione: se la convenzione cambia — ed è ancora pre-stabile —
cambia la proiezione, non i dati.

## Ritenzione

⚠️ **RICHIAMO DEL 2026-10-06** — audit del 2026-09-30, AUD-251, AUD-258, AUD-517 e AUD-646: la ritenzione a
livelli è decisa e la costruisce il 15 — oggi il giornale non pota niente, e `Journal::prune` non ha chiamanti di
produzione (il suo doc, in `crates/kernel/src/ports/journal.rs`) —; l'anello 4, che legge i fallimenti promossi,
arriva col 4.

| Livello | Contenuto | Ritenzione |
|---|---|---|
| **struttura** | identità, transizioni, esiti, routing, costi (col 3), verdetti, decisioni | lunga; è la parte piccola |
| **payload** | prompt e risposte (col 3), output degli strumenti (col 4), trascrizioni (col 8) | finestra breve → potati, sostituiti da impronta e dimensione (col 15) |
| **artefatti** | file prodotti (col il primo che produce un file) | **riferimenti**: il contenuto vive sul filesystem |

| Regola | Motivo |
|---|---|
| Un record potato **dichiara** di esserlo (col 15) | payload assente e payload mai registrato non devono confondersi. ⚠️ **Rimando del 2026-08-27:** non tenuta dal codice — vedi il rimando in [ADR-0018](../adr/0018-ritenzione-a-livelli-del-giornale.md) |
| Un passo `InDubbio` **non è potabile** (col 15) | la riconciliazione può dipendere dal payload. ⚠️ **Rimando del 2026-08-27:** la porta la tiene con un'altra nozione di dubbio, e le due divergono — stesso rimando in [ADR-0018](../adr/0018-ritenzione-a-livelli-del-giornale.md) |
| I fallimenti candidati a regressione si **promuovono** prima della potatura (col 15) | potare un fallimento non sfruttato butta via il dato più prezioso |

È la stessa gerarchia della compattazione del contesto (§4): ciò che è strutturato
sopravvive, il grezzo si sacrifica.

## Cosa deve essere sempre visibile

| Elemento | Perché | Vincolo | Dove | Chi |
|---|---|---|---|---|
| stato di degrado corrente | si dichiara prima, non si fallisce dopo | V27 · G9 | la striscia, e il modulo Stato per intero | 2 |
| permessi attivi nella sessione | un permesso concesso e dimenticato è indistinguibile da uno mai concesso | V21 · §6 · G10 | la striscia, se c'è una richiesta in attesa (`gui/src/panels/Strip.vue`); il modulo Permessi, con le triple concesse da questa finestra (`invoke.approved` in `gui/src/stores/invoke.ts`), vuote dopo un riavvio, e a parole che un permesso concesso prima resta concesso anche dopo un riavvio e lì non compare (`permissions.duration`) | 2 la richiesta in attesa e i sì di questa finestra; la lista del core: scelta aperta (AUD-140); il confine di sessione col 3 |
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

⚠️ **RICHIAMO DEL 2026-10-06** — audit del 2026-09-30, AUD-140: la riga dei permessi attivi dice ciò che il 2
mostra oggi; la lista dei permessi attivi tenuta dal core non è costruita, e chi la costruisce è la scelta aperta
su AUD-140.

## Regole che i diagrammi non esprimono

- **Nessuna telemetria lascia la macchina per default.** L'esportazione è opt-in e la
  destinazione la sceglie l'utente; chi la costruisce è la scelta aperta del diagramma delle
  proiezioni. C'è **un solo punto di uscita** (col 3), il che rende la promessa verificabile
  invece che dichiarata.
- Prima dell'esportazione si applica la mascheratura dei segreti (V16).
- La proiezione trace dichiara **quale versione** della convenzione emette: un trace
  senza versione, in uno standard che cambia, è ambiguo.
- Lo stato di degrado è **derivato**: se diverge, si ricalcola. Non è mai la verità.
- Si mostra ciò che **cambia cosa l'utente può fare**, non ogni variazione interna:
  un'interfaccia che segnala tutto è indistinguibile da una che non segnala nulla.
