# ADR-0006: Le due policy VRAM sono oggetti distinti, non due rami di un condizionale

- **Status:** Proposed
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

Il sistema ha due regimi di occupazione della GPU, e sono profondamente diversi:

| | Policy REMOTA *(default)* | Policy LOCALE |
|---|---|---|
| Chi occupa VRAM | audio riservato soltanto | audio + LLM + embedding locali |
| Prima di un job 3D | nulla da fare | eviction coordinata, obbligatoria |
| Dopo un job 3D | nulla da fare | ricarica, con avvio a freddo visibile |
| Chat durante un render | inalterata | bloccata, oppure dirottata su remoto |
| Modo di fallire | rete assente | avvio a freddo lungo, attese |

Non condividono né le invarianti né i modi di fallire. L'unica cosa che condividono è
il punto in cui vengono interrogate.

Alternative considerate:

- **Un condizionale sull'origine dell'inferenza,** sparso nei punti di decisione
  dell'arbitro. *Pro:* meno codice all'inizio. *Contro:* i due comportamenti derivano
  in silenzio; ogni nuova decisione aggiunge un ramo, e nessuno sa più quale regola
  valga in quale combinazione.
- **Due oggetti policy con la stessa interfaccia,** uno attivo per volta.
  *Contro:* un'astrazione in più e un po' di duplicazione fra le due.

## Decision

Le due policy sono **due oggetti distinti che implementano la stessa interfaccia**.
Uno solo è attivo, determinato dal profilo di configurazione corrente.

Il passaggio da una all'altra è una **transizione esplicita con effetti osservabili**
— eviction, ricarica, notifica all'utente — non un cambio di flag. Il "passaggio
suggerito a OpenRouter durante i render" della mappa funzionale *è* questa
transizione, offerta all'utente invece che imposta.

Vince perché la duplicazione fra due policy è visibile e circoscritta, mentre la
deriva di un condizionale è invisibile e diffusa. Fra i due costi, il primo si può
pagare quando serve; il secondo si scopre solo quando ha già fatto danno.

## Consequences

- **Positive:**
  - Ogni policy si testa in isolamento, con le proprie invarianti.
  - Aggiungere un terzo regime in futuro non tocca i due esistenti.
  - Il default resta chiaro e dichiarato: **REMOTA**, VRAM libera per TRELLIS2.
- **Negative (accettate):**
  - Un'interfaccia in più da mantenere, e qualche regola duplicata fra le due policy.
  - Il costo si paga anche se il regime locale non venisse mai usato davvero.
- **Follow-up richiesti:**
  - Se l'inferenza locale diventasse il default al posto di OpenRouter, lo swap
    coordinato passerebbe da eccezione a caso normale e cambierebbe l'intera UX di
    attesa e notifica. Sarebbe un ADR nuovo, non una riconfigurazione.
  - La fattibilità della policy LOCALE dipende da **SP-1**: quanta VRAM serve
    realmente a TRELLIS2 su 16 GB (§8 della spec). Se il picco supera ~13–14 GB,
    tenere un LLM caldo durante un render è impossibile e le due cose vanno dichiarate
    mutuamente esclusive.
