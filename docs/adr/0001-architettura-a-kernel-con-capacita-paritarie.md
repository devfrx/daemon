# ADR-0001: Architettura a kernel con capacità paritarie

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

> ⚠️ **Rimando del 2026-09-03 — il pilastro «voce» si legge «voce e gesti», e i pilastri restano
> quattro.** Il [disegno del riconoscimento gesti](../superpowers/specs/2026-09-03-riconoscimento-gesti-design.md)
> (decisione 1, sotto accettazione condizionata) mette il riconoscimento gesti dalla telecamera
> **dentro** il pilastro della voce, come sorgente di percezione sorella della wake word
> ([ADR-0039](0039-telecamera-come-sorgente-di-percezione.md)), e costruisce quel pilastro in
> due sotto-progetti — l'8 «Voce» e il 12 «Gesti» — come il kernel lo è in quattro. **Nessuna
> riga di questo ADR è superata:** un quinto pilastro avrebbe richiesto un ADR che supera la
> parola «quattro» per un contendente della GPU che oggi non la usa, ed è stato scartato. I
> consumatori paritari guadagnano un invocatore in più — il gesto — con lo stesso permesso di
> tutti gli altri ([ADR-0038](0038-registro-delle-funzioni-del-programma.md)).

## Context

La mappa funzionale descrive quattro aree — conversazione, agenti/coding, voce,
generazione asset 3D — senza gerarchia tra loro. Un'architettura non può però
ottimizzare per quattro centri di gravità: deve sceglierne uno e servire gli altri,
oppure dichiarare esplicitamente che non ne esiste uno e pagarne il prezzo.

Alternative considerate:

- **Baricentro su un pilastro** (es. harness agentico, o assistente vocale): il
  pilastro scelto detta la struttura e gli altri tre si adattano.
  *Pro:* primo pezzo utilizzabile molto prima; ottimizzazioni mirate legittime.
  *Contro:* gli altri tre pilastri diventano cittadini di seconda classe in modo
  irreversibile, perché l'accesso privilegiato del pilastro dominante si sedimenta.
- **Piattaforma paritaria:** nessuna area prevale; tutte consumano gli stessi
  servizi centrali alle stesse condizioni.
  *Pro:* struttura simmetrica, testabile, senza percorsi privilegiati.
  *Contro:* il primo pezzo utilizzabile arriva più tardi, perché prima del primo
  pilastro va costruito il kernel.

## Decision

Adottiamo la **piattaforma paritaria**: il sistema è un *kernel* con *capacità*.
Conversazione, conoscenza, agenti, coding, voce e generazione asset sono consumatori
paritari degli stessi servizi centrali — arbitro GPU, gateway di inferenza,
persistenza, permessi, bus eventi.

**Nessuna capacità ha accesso privilegiato né scorciatoie verso il kernel.** Questa è
la regola che rende la scelta reale invece che dichiarativa.

## Consequences

- **Positive:**
  - Aggiungere una settima capacità non richiede di toccare le altre sei.
  - Il kernel è testabile senza alcuna capacità caricata.
  - I conflitti di risorsa si risolvono in un punto solo, con una politica sola.
- **Negative (accettate):**
  - Il primo pezzo utilizzabile arriva più tardi rispetto a un'architettura con
    baricentro. È il costo diretto della scelta, non un effetto collaterale.
  - Il kernel deve essere progettato generico senza conoscere le capacità, il che
    richiede più disciplina e più rigore nei contratti.
- **Follow-up richiesti:**
  - Il primo sotto-progetto è il kernel (L0 + L1), non una capacità.
  - Ogni capacità futura va verificata contro la regola "nessun accesso
    privilegiato": una violazione è un ADR, non una scorciatoia.
