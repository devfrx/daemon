# ADR-0014: Il confine dei dati non fidati vive nel sistema di tipi, e la sanitizzazione non esiste

- **Status:** Accepted
- **Date:** 2026-08-06
- **Deciders:** proprietario del progetto

## Context

L'invariante I6 stabilisce che il contenuto non fidato non attraversa mai il confine
delle istruzioni. Restava da decidere **come** si applica.

Le fonti non fidate sono molte e crescono con il sistema:

| Fonte | Perché non è fidata |
|---|---|
| output degli strumenti | arbitrario, spesso remoto |
| pagine web e risultati di ricerca | scritte da chiunque |
| documenti importati, PDF, OCR | l'utente li ha scelti, non scritti |
| contenuto del filesystem letto dall'agente | può contenere qualsiasi cosa |
| descrizioni degli strumenti MCP | scritte da terzi ([ADR-0015](0015-descrizioni-degli-strumenti-fissate-all-approvazione.md)) |
| risposte dei provider di inferenza | testo generato, non istruzioni dell'utente |
| trascrizioni vocali | catturano anche ciò che l'utente non ha detto |

L'approccio istintivo è la **sanitizzazione**: filtrare o neutralizzare le istruzioni
presenti nel testo. Non funziona, e vale la pena dire perché: non esiste un criterio
affidabile per distinguere «testo che contiene un'istruzione» da «testo che parla di
istruzioni», e l'attaccante controlla la codifica. Un filtro produce l'apparenza di
una difesa e fallisce in silenzio — la peggiore combinazione possibile.

Alternative considerate:

- **Sanitizzazione** (filtri, escaping, delimitatori). *Contro:* illusione di
  sicurezza; incoraggia a smettere di cercare difese vere.
- **Solo approvazione umana**, nessuna difesa strutturale. *Contro:* sposta tutto il
  carico sull'utente, che dopo la ventesima richiesta approva senza leggere.
- **Confinamento tramite tipo.** *Contro:* attrito su ogni percorso che porta
  contenuto esterno verso il modello.

## Decision

**1. Il confine è nel sistema di tipi.** Il contenuto proveniente da fonti esterne è
trasportato da un tipo distinto da quello che trasporta le istruzioni. Non è
assegnabile a un campo istruzione: la conversione richiede un passaggio esplicito, e
il passaggio è giornalato.

**2. L'etichetta è ereditaria.** Estrarre, riassumere, tradurre o concatenare
contenuto non fidato produce contenuto non fidato. La contaminazione si propaga
attraverso ogni trasformazione — altrimenti basterebbe un riassunto per ripulire un
attacco.

**3. Non esiste sanitizzazione.** Non tentiamo di rimuovere istruzioni dal testo. La
regola è secca:

> **Un'istruzione trovata nei dati non è mai un'autorizzazione.**
> Il contenuto non fidato può *informare*, mai *autorizzare*.

**4. Conseguenza operativa.** Ogni azione la cui *decisione* dipende da contenuto non
fidato richiede la stessa autorizzazione che richiederebbe se l'utente non l'avesse
chiesta. Leggere una lista di cose da fare autorizza a leggerla, non a eseguirla.

## Consequences

- **Positive:**
  - **La difesa non dipende dal riconoscere l'attacco.** Un attacco non riconosciuto
    resta comunque inefficace, perché non può convertirsi in autorizzazione. È
    l'unica proprietà che regge contro un avversario che innova.
  - Q9 diventa verificabile staticamente — sui tipi — invece che per ispezione.
  - Il costo si paga una volta all'inizio. Aggiungerlo dopo significherebbe
    riscrivere ogni percorso ([ADR-0004](0004-topologia-di-processo.md), I6).
- **Negative (accettate):**
  - Attrito su ogni percorso che porta contenuto esterno verso il modello: la
    conversione va dichiarata, sempre, senza scorciatoie.
  - **Il modello vede comunque il contenuto non fidato.** Questa non è una difesa
    contro l'inganno del modello: è una difesa contro l'**escalation di privilegio**.
    Il modello può essere convinto di qualsiasi cosa; ciò che non può è agire senza
    autorizzazione. Dichiararlo evita di riporre nella difesa fiducia che non merita.
- **Follow-up richiesti:**
  - La GUI deve rendere **visibile la provenienza**: se l'utente non vede da dove
    viene un contenuto, approva alla cieca e la difesa collassa sull'anello umano.
  - Va deciso in §7 come si presenta una conversione esplicita nell'audit: è
    l'evento più interessante del giornale dal punto di vista della sicurezza.
