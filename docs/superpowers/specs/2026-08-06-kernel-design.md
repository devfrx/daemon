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
| 3 | Gateway di inferenza | Da presentare |
| 4 | Persistenza, run durevoli e idempotenza | **Proposta — in attesa di approvazione** |
| 5 | Permessi e confine dei dati non fidati | Da presentare |
| 6 | Errori, degrado e osservabilità | Da presentare |
| 7 | Test e criteri di accettazione | Da presentare |
| 8 | Rischi e spike di validazione | Da presentare |

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
dagli spike della §8 — non sono segnaposto, sono valori il cui metodo di
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

Q5–Q7 sono i requisiti delle **long-horizon tasks** e discendono dalla §4.

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
| V4 | `Rifiutata` e `InCoda` sono esiti distinti e vanno distinti anche in interfaccia | §6, GUI |

### 2.3 Domande aperte, con il metodo per chiuderle

| ID | Domanda | Si chiude con |
|---|---|---|
| SP-1 | Quanta VRAM serve realmente a TRELLIS2 su 16 GB? | spike §8 — determina se la policy LOCALE può tenere un LLM caldo |
| SP-2 | Ridurre l'occupazione dei job `batch` basta a tenere Q1 sotto i 600 ms? | spike §8 — se no, l'unica leva è sospendere il `batch` |

---

## 4. Persistenza, run durevoli e idempotenza

> **Stato: proposta, in attesa di approvazione.** ADR-0007 e 0008 sono in `Proposed`.
>
> Presentata prima della §3 perché non ha dipendenze da essa, e perché è la sezione
> che decide se le **long-horizon tasks** arrivano in fondo.

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
| V5 | Nessun effetto senza classe dichiarata; l'assenza vale `irripetibile` | ogni strumento, §5 |
| V6 | Write-ahead obbligatorio: nulla si esegue prima che l'intento sia durevole | §3, ogni capacità |
| V7 | Il contesto non è mai sorgente di verità: ciò che deve sopravvivere si scrive | §3, ogni capacità |
| V8 | Ogni run ha un tetto; in assenza di configurazione vale un default conservativo | §6, GUI |
| V9 | Ogni ingresso in `AttesaUmano` emette una notifica | §6, L3 |

### 4.4 Il modo di fallire che questa sezione introduce

Onestà sul rovescio della medaglia: il fallimento non scompare, **si sposta**.

| Prima | Dopo |
|---|---|
| "il contesto si riempie e l'informazione è persa" | "l'agente non ha registrato una decisione" |

Mitigazione strutturale: **registrare è un passo con un effetto giornalato**, non
un'aspettativa sul comportamento del modello. Se non è giornalato non è avvenuto — il
che rende il problema *osservabile* invece che silenzioso.

---

*Sezioni 3 e 5–8: in lavorazione. Ogni sezione approvata viene aggiunta qui e la
tabella di avanzamento aggiornata nello stesso passaggio.*
