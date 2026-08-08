# Messaggio di avvio di una chat

Da copiare all'inizio di ogni nuova sessione. Sostituisci `<SHA>` con l'ultimo commit —
lo SHA sta **qui e non nei file**, perché un hash scritto dentro il file che quel commit
contiene nasce già vecchio di uno.

⚠️ **Questo messaggio non nomina il prossimo passo, ed è deliberato.** Lo stato vive
nella §6 del compendio, in un posto solo. Duplicarlo qui lo farebbe divergere.

---

```
Riprendo un progetto già progettato, in corso.

Repo:   C:\Users\zagor\Desktop\harness
Remote: https://github.com/devfrx/daemon.git · branch spec/sottoprogetto-1-kernel
Albero pulito, ramo pushato. Ultimo commit: <SHA>

⛔ PRIMA DI PROPORRE QUALSIASI COSA: leggi, poi ASCOLTA. Non entrare in plan
   mode, non proporre passi, non scrivere file finché non te lo dico.

INVOCA QUESTE SKILL PRIMA DI QUALSIASI RISPOSTA O ESPLORAZIONE
  /superpowers:using-superpowers
  /anthropic-skills:dev-discipline      governa il codice
  /anthropic-skills:dev-communication   governa la conversazione intorno al codice

E QUESTE QUANDO SERVIRANNO, NON PRIMA
  /superpowers:subagent-driven-development   per ESEGUIRE un piano: un subagente
                                             fresco per compito, con revisione fra
                                             uno e l'altro. È la modalità scelta
  /superpowers:test-driven-development       quando si scrive codice
  /superpowers:brainstorming                 prima di qualunque lavoro creativo
  /superpowers:writing-plans                 quando si scriverà un piano nuovo

LEGGI SOLO QUESTI DUE FILE, PER INTERO, POI FERMATI
  1. CLAUDE.md
  2. docs/COMPENDIO.md — contiene TUTTE le decisioni del progetto: le 37 ADR
     compresse, le sei invarianti, le proprietà non retrofittabili, lo stack,
     il non rilitigabile, i trentasei gotcha, le trappole di check-docs.sh,
     i vincoli sul primo commit di codice, lo stato di oggi e il prossimo passo.

⛔ NON aprire docs/HANDOFF.md, la spec del sotto-progetto 1, né la cartella
   docs/adr/ «per farsi un'idea»: insieme pesano oltre mezzo megabyte, e
   l'idea è già nel compendio. Aprirai UN file — uno — quando ti servirà il
   perché di una decisione: le alternative scartate, le misure, i costi
   accettati. La §12 del compendio dice quale.

⚠️ Il compendio è una COMPRESSIONE, non una selezione: ci sono dentro tutte le
   decisioni, non quelle attinenti al compito di oggi. Sparisce il ragionamento
   lungo, non la decisione. Nessuna può sfuggirti perché «non sembrava attinente».

QUATTRO COSE CHE RIBADISCO, ANCHE SE STANNO NEI FILE
  · Non sono operativo in Rust. Quando l'argomento esce dal mio dominio,
    spiegamelo PRIMA a parole semplici e POI schematizza.
  · Alla chiusura di ogni voce COMMITTA E PUSHA senza chiedere, e SENZA
    co-autore. Prima però: bash scripts/check-docs.sh
  · Se ti viene un'idea che SOSTITUISCE una decisione presa, cerca PRIMA dove
    era già stata valutata e perché era caduta. Riaprila solo con una prova
    nuova — e se la prova nuova gioca contro, registrala e chiudila. È il
    gotcha #32, e c'è perché è già successo.
  · Un controllo si prova in DUE direzioni: che scatti dove deve, e che NON
    scatti dove non deve. La seconda si dimentica, ed è quella che conta —
    gotcha #24. Vale anche per i tuoi test, non solo per i miei.

Parti confermandomi cosa hai letto, qual è la tua lettura dello stato, e qual è
il prossimo passo secondo la §6 — POI ASPETTA le mie richieste prima di
proporre qualunque cosa.
```

---

## Perché è così corto

| | Prima | Adesso |
|---|---|---|
| il messaggio | ~9 KB | ~2 KB |
| lettura che ordinava | oltre 700 KB, e in pratica ~400k token | 24 KB, ~6k token |
| decisioni note all'agente | tutte, **dopo** ~400k token | **tutte**, dopo ~6k token |

Il messaggio lungo elencava undici letture «PER INTERO», fra cui **tutti** gli ADR, nove
diagrammi e una spec da 201 KB. Non è che chiedesse troppo: chiedeva la cosa giusta nel
posto sbagliato. Le decisioni servono **tutte** — ma compresse, non integrali.
