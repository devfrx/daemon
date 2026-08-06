# ADR-0013: La conformità allo schema è un verdetto di sensore, non un'eccezione

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

Quando il gateway chiede un output vincolato a uno schema — per una chiamata a
strumento, un dato strutturato, un modulo — l'output può non conformarsi. È un caso
frequente, non eccezionale.

Il trattamento istintivo è l'eccezione: la chiamata fallisce, qualcuno la ritenta.
Ma il gateway avrebbe così un proprio percorso di correzione, parallelo e diverso da
quello che la §5 ha già definito per linter, test e validazione della mesh — cioè per
esattamente lo stesso tipo di evento: *un artefatto osservato non rispetta un criterio
verificabile*.

## Decision

**La validazione dello schema è un sensore computazionale** ai sensi di
[ADR-0009](0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md). Un output non
conforme produce un **verdetto negativo con dettaglio**, che rientra nell'anello di
verifica come qualsiasi altro — non un'eccezione, non un percorso dedicato.

Conseguenza pratica: la correzione è un passo nuovo, giornalato (V14), con il
dettaglio del sensore come feedback. La violazione di schema diventa **osservabile e
misurabile** insieme a tutte le altre, invece di nascondersi in un contatore di errori
del gateway.

## Consequences

- **Positive:**
  - Un solo percorso di correzione per tutti i verdetti verificabili.
  - Le violazioni di schema entrano nelle statistiche dell'anello di miglioramento:
    se un certo strumento le produce sistematicamente, l'anello 4 lo rileva e propone
    una guida migliore o uno schema più semplice — che è la causa reale.
  - Nessuna infrastruttura di ritentativo specifica del gateway.
- **Negative (accettate):**
  - Un output malformato costa un giro d'anello anziché un ritentativo immediato,
    quindi è leggermente più lento del percorso dedicato.
  - Il gateway dipende dal registro dei sensori: è un accoppiamento fra §3 e §5 che
    non esisterebbe con un percorso separato.
- **Follow-up richiesti:**
  - Va distinto il *decoding vincolato* (l'output non **può** essere malformato,
    quando il provider lo supporta) dalla *validazione a posteriori*. Il primo rende
    il sensore superfluo; il secondo lo rende necessario. Il record di routing deve
    dire quale dei due è stato usato, perché cambia il significato dell'assenza di
    verdetti.
