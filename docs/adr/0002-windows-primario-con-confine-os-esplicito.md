# ADR-0002: Windows primario, con confine OS esplicito

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

Il progetto ha come obiettivo il supporto di Windows e Linux. Il costo del supporto
Linux non è però distribuito uniformemente: si concentra quasi interamente nelle aree
già difficili — cattura audio always-on, sandbox di esecuzione, avvio automatico,
hotkey globali, tray, packaging. Sviluppare e testare su entrambi dal giorno 1
raddoppia la superficie di integrazione proprio dove è più insidiosa.

Alternative considerate:

- **Entrambi dal giorno 1:** portabilità garantita, il confine non può marcire.
  *Contro:* raddoppia il lavoro di integrazione e test.
- **Windows prima, Linux dopo, senza confine previsto:** più veloce all'inizio.
  *Contro:* il codice OS-specifico si diffonde nel kernel e la portabilità diventa
  una riscrittura invece che un'aggiunta.
- **Solo Windows:** semplificazione tangibile.
  *Contro:* decisione difficilmente reversibile.

## Decision

Sviluppiamo e testiamo su **Windows**, ma il design isola fin da subito ogni punto
OS-specifico dietro un **modulo di piattaforma** con interfaccia definita e
implementazione fittizia per i test. Linux si aggiunge in seguito implementando
quel modulo, senza rimettere mano al kernel.

Il confine è un **vincolo verificabile, non una buona intenzione**: il kernel non
contiene nessuna chiamata OS-specifica, e questo si controlla staticamente sui grafi
di importazione. Vedi invariante 3 in [ADR-0004](0004-topologia-di-processo.md).

## Consequences

- **Positive:**
  - Metà della superficie di debug durante lo sviluppo, con costo di design ~nullo.
  - Il confine, essendo controllato automaticamente, non può degradare in silenzio.
  - L'implementazione fittizia rende il kernel testabile senza toccare l'OS.
- **Negative (accettate):**
  - Il confine è verificato, ma non *validato*: finché non esiste
    un'implementazione Linux reale, non sappiamo se l'astrazione ha la forma giusta.
    Rischio noto e accettato, mitigato dal punto seguente.
  - Ogni chiamata OS costa una indirezione in più rispetto alla chiamata diretta.
- **Follow-up richiesti:**
  - Al primo punto OS-specifico non banale (cattura audio, sandbox), schizzare
    *su carta* come sarebbe l'implementazione Linux prima di congelare l'interfaccia.
    Serve a scoprire subito le astrazioni sbagliate, senza scrivere il codice.
