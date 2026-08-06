# Spec — Kernel (L0 fondamenta + L1 arbitri trasversali)

- **Data:** 2026-08-06
- **Sotto-progetto:** kernel. È il primo, perché tutte e sei le capacità di L2
  dipendono da esso.
- **Stato:** in costruzione — vedi tabella di avanzamento.

## Avanzamento delle sezioni

| § | Sezione | Stato |
|---|---|---|
| 0 | Perimetro, vincoli e requisiti di qualità | Approvata |
| 1 | Architettura di processo | Approvata |
| 2 | Arbitro risorse GPU e policy VRAM | Approvata |
| 3 | Gateway di inferenza | **Proposta — in attesa di approvazione** |
| 4 | Persistenza, run durevoli e idempotenza | Approvata |
| 5 | Harness: guide, sensori e anelli di controllo | Approvata |
| 6 | Permessi e confine dei dati non fidati | Da presentare |
| 7 | Errori, degrado e osservabilità | Da presentare |
| 8 | Test e criteri di accettazione | Da presentare |
| 9 | Rischi e spike di validazione | Da presentare |

---

## 0. Perimetro, vincoli e requisiti di qualità

### 0.1 Cosa è

Applicazione desktop GUI, local-first, utente singolo. Piattaforma a quattro pilastri
paritari su kernel comune ([ADR-0001](../../adr/0001-architettura-a-kernel-con-capacita-paritarie.md)).

### 0.2 Cosa il kernel NON fa

Il perimetro negativo è l'artefatto più prezioso di questa sezione.

| Il kernel non… | Perché |
|---|---|
| …conosce le capacità | Deve restare testabile senza alcuna capacità caricata (ADR-0001) |
| …contiene interfaccia grafica | La GUI è un processo separato e sacrificabile (ADR-0004) |
| …contiene chiamate OS-specifiche | Passano tutte dal modulo di piattaforma (I3) |
| …carica codice di terze parti | Estensioni solo via MCP e skill dichiarative ([ADR-0003](../../adr/0003-estensibilita-solo-mcp-e-skill-dichiarative.md)) |
| …espone un'API pubblica | Il protocollo IPC è privato e non versionato (I4) |

### 0.3 Vincoli dati

| Vincolo | Valore |
|---|---|
| GPU | singola, RTX 5080, 16 GB VRAM |
| Sistema operativo | Windows primario; Linux successivo dietro confine esplicito ([ADR-0002](../../adr/0002-windows-primario-con-confine-os-esplicito.md)) |
| Inferenza | OpenRouter primaria; inferenza locale opzionale |
| Utenza | singolo utente, nessuna multi-tenancy, nessuna autenticazione |
| Rete | richiesta per OpenRouter; degrado esplicito quando assente |

### 0.4 Requisiti di qualità

Espressi come **scenari misurabili**: è qui che vive la difficoltà reale del sistema,
non nell'elenco delle funzionalità. Le soglie marcate *(da tarare)* saranno fissate
dagli spike della §9 — non sono segnaposto, sono valori il cui metodo di
determinazione è già deciso.

| ID | Scenario | Soglia |
|---|---|---|
| Q1 | Da fine enunciato a primo fonema di risposta, con job GPU pesante in corso | < 600 ms al 95° percentile |
| Q2 | Job GPU concorrenti che causano OOM | zero, per costruzione (I2) |
| Q3 | Chiusura o crash della GUI durante una run agentica | la run prosegue, nessuna perdita di stato |
| Q4 | Crash o kill di un worker in qualsiasi istante | nessuna corruzione, nessuna perdita (I1) |
| Q5 | Riavvio del core a metà di una run lunga | ripresa dall'ultimo passo giornalato, **nessun effetto rieseguito** |
| Q6 | Contesto esaurito prima che il task sia finito | la run prosegue: il contesto si ricalcola dallo stato durevole, nessuna informazione persa |
| Q7 | Run che supera il tetto di passi, tempo o costo | si ferma e chiede; non prosegue in silenzio |
| Q8 | Ricarica di un modello locale dopo scarico (avvio a freddo) | segnalata all'utente prima che percepisca un blocco |
| Q9 | Contenuto non fidato che tenta di iniettare istruzioni | non raggiunge mai il canale delle istruzioni (I6) |
| Q10 | Esito di un passo che viola un sensore | rientra nell'anello con il feedback del sensore, senza intervento umano |
| Q11 | Occupazione della proiezione | resta al **budget target**, non al limite della finestra |
| Q12 | Difetto che si ripresenta per la seconda volta | il sistema propone una guida o un sensore, non una raccomandazione generica |
| Q13 | Richiesta con vincolo sui dati e nessun endpoint conforme | **fallisce chiuso**: errore che nomina il vincolo, nessun ripiego |
| Q14 | Ricostruire con cosa è stato eseguito un passo di sei mesi fa | il record di routing lo dice, indipendentemente dalla configurazione attuale |

Q5–Q7 sono i requisiti delle **long-horizon tasks** (§4).
Q10–Q12 sono i requisiti dell'**harness** (§5).
Q13–Q14 sono i requisiti del **gateway** (§3).

### 0.6 Le tre discipline e dove vivono

Il progetto adotta esplicitamente le tre discipline consolidate nello stato dell'arte
2025–2026. Sono impilate: ognuna avvolge la precedente.

| Disciplina | Oggetto | Formula |
|---|---|---|
| **Context engineering** | l'insieme dei token che il modello vede a ogni turno — memoria, strumenti, recupero, stato | *l'informazione giusta, nel formato giusto, al momento giusto* |
| **Loop engineering** | il ciclo in cui l'agente gira: trigger, topologia, verificatore, regole di arresto | quattro anelli annidati |
| **Harness engineering** | tutto ciò che circonda il modello | `Agent = Model + Harness` |

Non sono una sezione della spec: sono **trasversali**, e ogni loro leva ha una casa
esplicita.

| Leva | Disciplina | Sezione che la possiede |
|---|---|---|
| Contesto come artefatto curato | context | §4 — [ADR-0008](../../adr/0008-contesto-come-proiezione-dello-stato.md) |
| Budget della proiezione, misura per categoria | context | §5 — [ADR-0010](../../adr/0010-budget-della-proiezione-invece-di-soglia-di-riempimento.md) |
| Politiche di memoria e recupero | context | §4 (meccanismo) · capacità Conoscenza (politica) |
| Anello dell'agente | loop | capacità Agenti, su meccanismo §4 |
| Anello di verifica | loop | §5 — [ADR-0009](../../adr/0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md) |
| Trigger ed eventi | loop | §5 |
| Regole di arresto | loop | §4 — confini di autonomia (V8) |
| Anello di miglioramento | loop | §5 |
| Guide (feedforward) | harness | §5 · le skill dichiarative di [ADR-0003](../../adr/0003-estensibilita-solo-mcp-e-skill-dichiarative.md) *sono* guide |
| Sensori (feedback) | harness | §5 |
| Schemi degli strumenti | harness | §3 — output vincolato |
| Sandbox ed esecuzione | harness | §6 |
| Livelli di permesso e gate umani | harness | §6 |
| Routing | harness | §3 |
| Osservabilità | harness | §4 (giornale) · §7 |

### 0.5 Requisiti strutturali che vincolano la topologia

Vedi la tabella R1–R5 in [ADR-0004](../../adr/0004-topologia-di-processo.md#context).
Sono proprietà del processo, non funzionalità: non si aggiungono a posteriori.

---

## 1. Architettura di processo

**Decisione:** [ADR-0004](../../adr/0004-topologia-di-processo.md) — core di servizio,
GUI sottile, worker effimeri.

**Struttura corrente:** [Topologia dei processi](../../design/01-topologia-dei-processi.md).

Le sei invarianti I1–I6 definite nell'ADR sono vincolanti per tutte le sezioni
successive. Ogni scelta di design che segue deve poter essere verificata contro di
esse; una violazione richiede un ADR, non una deroga.

---

## 2. Arbitro risorse GPU e policy VRAM

**Decisioni:** [ADR-0005](../../adr/0005-arbitrato-gpu-su-due-dimensioni.md) ·
[ADR-0006](../../adr/0006-due-policy-vram-come-oggetti-distinti.md).

**Struttura:** [Arbitrato delle risorse GPU](../../design/02-arbitrato-gpu.md).

### 2.1 In sintesi

| Scelta | Sostanza |
|---|---|
| Due dimensioni | VRAM = capacità (ammissione) · calcolo = contesa (corsie) |
| Profilo di risorsa | descrittore nominato per ogni tipo di lavoro; riserva dichiarata, picco misurato |
| Quota audio | **sottratta** dal budget allocabile, non prioritaria |
| Tre corsie | `realtime` mai prelazionabile · `interattivo` · `batch` |
| Due policy | REMOTA (default) e LOCALE come oggetti distinti, con transizione esplicita |
| Nessun degrado silenzioso | ogni decisione presa al posto dell'utente viene comunicata |

### 2.2 Vincoli che la §2 impone alle sezioni successive

| # | Vincolo | Colpisce |
|---|---|---|
| V1 | Nessun lavoro tocca la GPU senza concessione valida | §3 gateway, ogni capacità L2 |
| V2 | Ogni tipo di lavoro GPU deve avere un profilo di risorsa dichiarato | ogni capacità L2 |
| V3 | La policy attiva è una sola e proviene dal profilo di configurazione | §3, §4 |
| V4 | `Rifiutata` e `InCoda` sono esiti distinti e vanno distinti anche in interfaccia | §7, GUI |

### 2.3 Domande aperte, con il metodo per chiuderle

| ID | Domanda | Si chiude con |
|---|---|---|
| SP-1 | Quanta VRAM serve realmente a TRELLIS2 su 16 GB? | spike §9 — determina se la policy LOCALE può tenere un LLM caldo |
| SP-2 | Ridurre l'occupazione dei job `batch` basta a tenere Q1 sotto i 600 ms? | spike §9 — se no, l'unica leva è sospendere il `batch` |

---

## 3. Gateway di inferenza

> **Stato: proposta, in attesa di approvazione.** ADR-0011, 0012 e 0013 sono in
> `Proposed`.

**Decisioni:** [ADR-0011](../../adr/0011-routing-risolto-e-giornalato-per-richiesta.md) ·
[ADR-0012](../../adr/0012-equivalenza-del-fallback-e-fallimento-chiuso.md) ·
[ADR-0013](../../adr/0013-conformita-allo-schema-e-un-verdetto-di-sensore.md).

**Struttura:** [Gateway di inferenza](../../design/05-gateway-inferenza.md).

### 3.1 In sintesi

| Scelta | Sostanza |
|---|---|
| Record di routing risolto | ogni richiesta giornala la decisione **risolta**, non un rimando alla configurazione |
| Tutto è una run | ogni interazione con un modello è un passo; una chat è una run interattiva |
| Contabilità gerarchica | messaggio/sessione/run/sub-agente sono aggregazioni della stessa gerarchia |
| Equivalenza del fallback | definita dai **vincoli** della richiesta, non dalla capacità del modello |
| Due classi di vincolo | dati → **fallisce chiuso** · qualità e costo → degrado dichiarato |
| Rifiuto dell'arbitro GPU | causa di fallback di prima classe, non errore |
| Ritentativo ≠ passo | resta dentro lo stesso passo: cambia il record, non la run |
| Schema non conforme | **verdetto di sensore** (§5), non eccezione del gateway |
| Stream interrotti | il costo si registra comunque |

### 3.2 Perché "tutto è una run" non è sovra-ingegnerizzazione

È la condizione perché i meccanismi della §4 siano **universali invece che specifici
dell'agente**. Senza di essa servirebbe un secondo percorso per la chat: un'altra
contabilità, un altro annullamento, un altro tracciamento. Il costo è due scritture
durevoli su una latenza già dominata dalla chiamata al modello.

### 3.3 Vincoli che la §3 impone alle sezioni successive

| # | Vincolo | Colpisce |
|---|---|---|
| V15 | Ogni richiesta dichiara i propri vincoli, anche quando coincidono con i default | ogni capacità |
| V16 | Il record di routing non contiene mai credenziali; nomi di provider e parametri sì | §6 |
| V17 | Ritentativo e cambio di candidato restano dentro lo stesso passo | §4, capacità Agenti |
| V18 | Un errore di vincolo non soddisfatto deve nominare **quale** vincolo | §7, GUI |

### 3.4 Il costo che questa sezione introduce

**Più richieste falliranno del tutto**, per scelta. Un vincolo sui dati che non trova
endpoint conformi produce un errore invece di una risposta.

È controintuitivo e sotto pressione la tentazione di "provare comunque" sarà forte:
per questo la decisione è presa adesso e scritta, non lasciata al momento in cui
qualcosa non funziona. Il contrappeso è V18 — se il sistema si rifiuta di rispondere,
deve dire esattamente perché.

### 3.5 Domanda aperta

| ID | Domanda | Si chiude con |
|---|---|---|
| SP-4 | Quali provider supportano l'annullamento senza addebito? | verifica sul campo alla prima integrazione: determina l'ordine di preferenza per le richieste che si annullano spesso |

---

## 4. Persistenza, run durevoli e idempotenza

Presentata prima della §3 perché non ha dipendenze da essa, e perché è la sezione che
decide se le **long-horizon tasks** arrivano in fondo.

**Decisioni:** [ADR-0007](../../adr/0007-giornale-write-ahead-e-riconciliazione.md) ·
[ADR-0008](../../adr/0008-contesto-come-proiezione-dello-stato.md).

**Struttura:** [Run durevoli, giornale e proiezione](../../design/03-run-durevoli.md).

### 4.1 In sintesi

| Scelta | Sostanza |
|---|---|
| Tre livelli di stato | durevole (verità) · proiezione (contesto) · presentazione (GUI) |
| Giornale write-ahead | intento prima di eseguire, esito dopo → il **dubbio è rilevabile** |
| Ripresa = riconciliazione | non replay cieco: per ogni passo in dubbio si stabilisce cosa è accaduto |
| Tre classi di effetto | `verificabile` · `idempotente` · `irripetibile` |
| Default sicuro | effetto non classificato = `irripetibile` → sospendi e chiedi |
| Contesto = proiezione | compattare **ricompone**, non riassume; solo la trascrizione grezza è sacrificabile |
| Confini di autonomia | passi, tempo, costo: il superamento **sospende**, non termina |

### 4.2 Perché è nel kernel e non nella capacità agenti

Il giornale delle run serve anche alla coda dei render 3D, all'indicizzazione RAG e
alla deep research: sono tutte attività lunghe con effetti da riconciliare. Metterlo
nella capacità agenti le darebbe un accesso privilegiato, che
[ADR-0001](../../adr/0001-architettura-a-kernel-con-capacita-paritarie.md) vieta.

Il kernel fornisce il **meccanismo**; ogni capacità porta la propria **politica**
(quali passi, quali effetti, cosa entra nella proiezione).

### 4.3 Vincoli che la §4 impone alle sezioni successive

| # | Vincolo | Colpisce |
|---|---|---|
| V5 | Nessun effetto senza classe dichiarata; l'assenza vale `irripetibile` | ogni strumento, §6 |
| V6 | Write-ahead obbligatorio: nulla si esegue prima che l'intento sia durevole | §3, ogni capacità |
| V7 | Il contesto non è mai sorgente di verità: ciò che deve sopravvivere si scrive | §3, ogni capacità |
| V8 | Ogni run ha un tetto; in assenza di configurazione vale un default conservativo | §7, GUI |
| V9 | Ogni ingresso in `AttesaUmano` emette una notifica | §7, L3 |

### 4.4 Il modo di fallire che questa sezione introduce

Onestà sul rovescio della medaglia: il fallimento non scompare, **si sposta**.

| Prima | Dopo |
|---|---|
| "il contesto si riempie e l'informazione è persa" | "l'agente non ha registrato una decisione" |

Mitigazione strutturale: **registrare è un passo con un effetto giornalato**, non
un'aspettativa sul comportamento del modello. Se non è giornalato non è avvenuto — il
che rende il problema *osservabile* invece che silenzioso.

---

## 5. Harness: guide, sensori e anelli di controllo

**Decisioni:** [ADR-0009](../../adr/0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md) ·
[ADR-0010](../../adr/0010-budget-della-proiezione-invece-di-soglia-di-riempimento.md).

**Struttura:** [Anelli di controllo, guide e sensori](../../design/04-anelli-e-sensori.md).

### 5.1 In sintesi

| Scelta | Sostanza |
|---|---|
| Due tipi di controllo | **guide** (feedforward, probabilistiche) · **sensori** (feedback, verificabili) |
| Quattro anelli | agente · verifica · eventi · miglioramento |
| Contratto del sensore | minimo per scelta: `(artefatto) → (verdetto, dettaglio, costo)` |
| Sensori per costo | computazionali dentro l'anello stretto · inferenziali a valle |
| Anello di miglioramento | il sistema **propone**, l'utente approva; non si auto-modifica |
| Budget della proiezione | occupazione obiettivo, non soglia di overflow (**context rot**) |
| Misura per categoria | l'occupazione del contesto per categoria entra nel giornale |

### 5.2 L'unificazione che giustifica la collocazione nel kernel

| Capacità | Il suo "sensore" |
|---|---|
| Coding | linter, type checker, esecuzione dei test |
| Generazione asset | validazione della mesh prima dell'export |
| Conoscenza / RAG | verifica che ogni affermazione abbia una citazione risolvibile |

Sono lo **stesso oggetto**: osservano un artefatto e producono un verdetto.
Implementarli dentro tre capacità darebbe tre vocabolari, tre formati di verdetto e
nessun anello di miglioramento possibile — oltre a violare la parità di ADR-0001.

### 5.3 Vincoli che la §5 impone alle sezioni successive

| # | Vincolo | Colpisce |
|---|---|---|
| V10 | Un sensore osserva e produce un verdetto: **non modifica nulla** | ogni capacità |
| V11 | Ogni sensore dichiara il proprio costo; gli inferenziali restano fuori dall'anello stretto | §3, ogni capacità |
| V12 | L'anello 4 propone, non applica: nessuna auto-modifica delle guide senza approvazione | §7, GUI |
| V13 | La ricomposizione della proiezione mantiene il **budget**, non evita l'overflow | §3, capacità Agenti |
| V14 | Un verdetto negativo che rientra nell'anello è **un passo nuovo**, giornalato | §4 |

### 5.4 Domanda aperta

| ID | Domanda | Si chiude con |
|---|---|---|
| SP-3 | Oltre quale frazione della finestra la qualità cala, per i modelli usati? | spike §9 — fino ad allora vale un default conservativo, **dichiarato come tale** |

### 5.5 Il rischio che questa sezione introduce

**Astrazione prematura.** Il contratto comune del sensore è ipotizzato su tre casi di
cui nessuno esiste ancora. Se divergono più del previsto, il contratto diventa un
vincolo invece che uno strumento.

Mitigazione: il contratto è deliberatamente povero, e va **rivisto dopo il secondo
sensore reale in aree diverse**. Se non si adatta, si spezza — non si piega.

---

*Sezioni 3 e 6–9: in lavorazione. Ogni sezione approvata viene aggiunta qui e la
tabella di avanzamento aggiornata nello stesso passaggio.*
