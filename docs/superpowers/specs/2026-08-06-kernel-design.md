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
| 2 | Arbitro risorse GPU e policy VRAM | Da presentare |
| 3 | Gateway di inferenza | Da presentare |
| 4 | Persistenza e modello dello stato | Da presentare |
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
| Q5 | Riavvio del core con run in corso | run riprese dall'ultimo checkpoint |
| Q6 | Ricarica di un modello locale dopo scarico (cold start) | segnalata all'utente prima che percepisca un blocco |
| Q7 | Contenuto non fidato che tenta di iniettare istruzioni | non raggiunge mai il canale delle istruzioni (I6) |

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

*Sezioni 2–8: in lavorazione. Ogni sezione approvata viene aggiunta qui e la tabella
di avanzamento aggiornata nello stesso passaggio.*
