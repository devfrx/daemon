# ADR-0017: Il giornale è la sorgente, il trace è una proiezione

- **Status:** Proposed
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

L'osservabilità degli agenti si è standardizzata attorno alle **convenzioni semantiche
GenAI di OpenTelemetry** (`gen_ai.*`): un vocabolario comune per chiamate al modello,
passi dell'agente, invocazioni di strumenti, uso di token, costo e metriche di
qualità. È adottato dai principali strumenti del settore, e gli agenti di riferimento
emettono già trace leggibili da qualsiasi backend OTLP.

C'è però un fatto che ne condiziona l'adozione: **è ancora pre-stabile**. A giugno
2026 gli attributi e gli span `gen_ai.*` sono stati spostati in un repository
dedicato, ma senza rilascio 1.0 e senza promessa di stabilità — i nomi possono ancora
cambiare.

Il nostro sistema ha già una registrazione durevole: il giornale
([ADR-0007](0007-giornale-write-ahead-e-riconciliazione.md)), che esiste per un motivo
diverso — la ripresa dopo crash — ed è la sorgente di verità della §4.

Alternative considerate:

- **Adottare OTel come formato di archiviazione.** Un solo modello di dati.
  *Contro:* lega il substrato durevole della ripresa a una specifica sperimentale.
  Un cambio di nomi negli attributi diventerebbe una migrazione dei dati di
  ripristino: rischio sproporzionato al beneficio.
- **Formato interamente proprietario, nessuna compatibilità.** Nessun vincolo esterno.
  *Contro:* si rinuncia a ogni strumento esistente e si reinventa un vocabolario che
  altri hanno già discusso per due anni.
- **Giornale come sorgente, trace come proiezione esportabile.**
  *Contro:* una trasformazione da mantenere.

## Decision

**1. Il giornale resta la sorgente di verità.** Trace, contabilità, metriche e dataset
di regressione sono **proiezioni** ricavate da esso — la stessa forma già adottata per
il contesto ([ADR-0008](0008-contesto-come-proiezione-dello-stato.md)) e per la GUI
([ADR-0004](0004-topologia-di-processo.md)).

**2. Adottiamo il vocabolario GenAI di OpenTelemetry per la proiezione trace**, non
per l'archiviazione. Il giornale registra i fatti; la proiezione li nomina secondo la
convenzione. Se la convenzione cambia, cambia la proiezione — **non i dati**.

**3. Nessuna telemetria lascia la macchina per default.** L'esportazione verso un
backend OTLP esiste, è **opt-in**, e la destinazione la sceglie l'utente. Il
local-first non è un vincolo tecnico: è una promessa che il sistema deve poter
mantenere in modo verificabile.

## Consequences

- **Positive:**
  - Un solo substrato serve cinque scopi: ripresa, trace, costi, metriche e dataset
    di regressione. Nessun secondo sistema di osservabilità da tenere allineato.
  - La pre-stabilità dello standard diventa irrilevante: assorbita dalla proiezione.
  - I trace sono leggibili da strumenti esistenti senza che il progetto vi si leghi.
  - «Nessuna telemetria esterna» è dimostrabile: c'è un solo punto di uscita.
- **Negative (accettate):**
  - Una trasformazione da scrivere e mantenere allineata alla convenzione.
  - Il giornale non è direttamente ispezionabile con strumenti OTLP: serve esportare.
- **Follow-up richiesti:**
  - La proiezione deve dichiarare **quale versione** della convenzione emette. Un
    trace senza versione, in uno standard che cambia, è ambiguo.
  - I trace esportati contengono contenuto: prima dell'esportazione va applicata la
    stessa mascheratura dei segreti richiesta da V16 (§6).
