# ADR-0004: Topologia di processo — core di servizio, GUI sottile, worker effimeri

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

Cinque requisiti del perimetro discriminano tra le topologie possibili, e nessuno di
essi è negoziabile:

| # | Requisito | Natura |
|---|---|---|
| R1 | Run agentiche che sopravvivono a chiusura e riavvio dell'interfaccia | strutturale |
| R2 | Voce always-on con daemon in background e avvio automatico | strutturale |
| R3 | Un solo arbitro autorevole della GPU, o l'OOM è inevitabile | strutturale |
| R4 | Lo strato ML può morire per OOM senza portarsi via nient'altro | strutturale |
| R5 | Task pianificati che scattano anche a finestra chiusa | strutturale |

Sono **proprietà del processo**, non funzionalità: non si aggiungono a un'architettura
che non le prevede, si ottengono cambiandola.

Alternative considerate:

- **A — Kernel di servizio + client sottile.** Un daemon possiede lo stato; la GUI si
  collega e si scollega; worker effimeri fanno il lavoro pesante.
  *Pro:* soddisfa R1–R5 per costruzione. *Contro:* protocollo IPC e ciclo di vita
  dei processi da progettare; debug attraverso un confine di processo.
- **B — Applicazione monolitica + worker ML.** Un solo processo applicativo.
  *Pro:* nettamente più semplice; un solo posto per lo stato; debug diretto.
  *Contro:* non soddisfa R1, R2, R5. Mitigarlo con "minimizza nella tray invece di
  chiudere" è una finzione: un difetto nel viewer 3D porta via una run agentica in corso.
- **C — Servizi locali separati per dominio.** Un servizio per voce, agenti, asset…
  *Pro:* isolamento dei guasti massimo. *Contro:* rende l'arbitrato GPU un problema
  di consenso distribuito e aggiunge un salto di processo su un percorso con 600 ms
  di budget — peggiora cioè proprio i due pezzi più critici.

## Decision

Adottiamo la **topologia A**, nella sua forma minima: tre classi di processo, non una
di più.

| Classe | Quante | Vita | Possiede |
|---|---|---|---|
| **core** | 1, istanza singola | lunga, indipendente dalla GUI | **tutto** lo stato autorevole |
| **gui** | 0..1 | effimera, sacrificabile | solo stato di presentazione |
| **worker** | 0..N | breve, uccidibile in qualsiasi istante | nulla |

I worker sono **pochi, stupidi e a vita breve**: eseguono un compito e possono essere
uccisi senza preavviso. Non sono micro-servizi — non hanno stato proprio, non si
parlano tra loro, non decidono, non ritentano. È questo che distingue A da C.

Il perché delle esclusioni, in una riga ciascuna:

- **B è scartata** perché non può acquisire R1/R2/R5 se non *diventando A per
  accrescimento*, un pezzo alla volta e senza che nessuno abbia progettato il confine.
  Quella è la definizione esatta del debito architetturale.
- **C è scartata benché più costosa** perché frammentare in servizi *rimanda* la
  domanda su chi possiede cosa invece di risolverla, e distribuisce l'unica cosa che
  deve restare centralizzata.

## Invarianti

Un principio che non si può controllare è un'intenzione. Queste sono controllabili e
diventano test:

| # | Invariante | Come si verifica |
|---|---|---|
| **I1** | Lo stato autorevole vive solo nel core. GUI e worker non hanno persistenza propria. | Uccidere gui o worker in qualsiasi istante non perde né corrompe nulla |
| **I2** | La GPU ha un solo proprietario: nessun processo la tocca senza concessione dell'arbitro. | Nessun worker si avvia senza una concessione valida — ⚠️ **verifica completata da [ADR-0033](0033-gpu-della-gui-quota-di-presentazione.md)**: questa riga copriva una classe di processo su tre, e anche la `gui` tocca la GPU |
| **I3** | Il core non contiene codice OS-specifico: ogni chiamata all'OS passa dal modulo di piattaforma. | Analisi statica dei grafi di importazione |
| **I4** | Il protocollo IPC è privato, singolo e non versionato: un trasporto, uno schema, nessun broker, nessun service discovery. | Nessun consumatore esterno, per [ADR-0003](0003-estensibilita-solo-mcp-e-skill-dichiarative.md) |
| **I5** | I worker sono senza stato e senza voce in capitolo: ritentativi, code e priorità stanno nel core. | Un worker non contiene logica di retry né di scheduling |
| **I6** | Il contenuto non fidato non attraversa mai il confine delle istruzioni: entra etichettato come dato e l'etichetta viaggia con lui. | Il tipo che trasporta contenuto esterno è distinto da quello delle istruzioni |

**I6 è l'unica voce di sicurezza che deve nascere nel kernel.** Approvazioni,
privilegio minimo e sandbox possono arrivare con le capacità senza costi retroattivi;
l'etichettatura dei dati non fidati no — aggiungerla dopo significa riscrivere ogni
percorso. È la difesa strutturale contro prompt injection e tool poisoning.

## Consequences

- **Positive:**
  - R1–R5 soddisfatti per costruzione, non per accorgimento.
  - L'arbitro GPU vive in un unico processo con un unico lock: la forma più semplice
    possibile del pezzo che deve essere infallibile.
  - La GUI diventa sacrificabile: si può ricaricare, ricompilare o far crashare mentre
    una run agentica prosegue indisturbata.
  - Un OOM nello strato ML non tocca nient'altro.
- **Negative (accettate):**
  - C'è un protocollo IPC da progettare e un ciclo di vita da gestire: istanza
    singola, chi avvia chi, recupero dopo crash, daemon orfani.
  - Il debug attraversa un confine di processo; serve tracing correlato fin da subito.
  - Tre eseguibili da impacchettare e aggiornare invece di uno.
- **Follow-up richiesti:**
  - Il conteggio "tre classi di processo" è esso stesso una regola: ogni proposta di
    aggiungerne una quarta si giustifica contro la tabella sopra, o non si fa.
  - La scelta del linguaggio del core è un **ADR separato e successivo**: decidere il
    linguaggio prima della topologia è l'errore che poi obbliga a piegare
    l'architettura allo strumento.
