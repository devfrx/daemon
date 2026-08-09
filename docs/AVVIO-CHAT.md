# Messaggio di avvio di una chat

Da copiare all'inizio di ogni nuova sessione.

⚠️ **Il campo dello SHA è un segnaposto, e si compila incollando.** Lo SHA appartiene al
**messaggio**, dove è vero nel momento in cui si legge — non a questo file, che del
messaggio è il **modello**: un hash scritto qui nasce già vecchio di uno, perché il commit
che lo aggiorna lo contiene. È la stessa ragione per cui [`HANDOFF.md`](HANDOFF.md) non ne
porta, e quella riga delega esplicitamente a qui.

⛔ **Prima questo campo portava un valore vero, e non ha funzionato.** È stato incollato
tale e quale **due volte** — l'ultima vecchio di **quattro** commit — e due commit
(`a2ac9f9`, `c7f3203`) sono serviti solo a rincorrerlo. Un valore d'esempio valido non si
distingue da un dato; un segnaposto sì. Gotcha **#43**.

⚠️ **Questo messaggio non nomina il prossimo passo, ed è deliberato.** Lo stato vive nella
§6 del compendio, in un posto solo. Duplicarlo qui lo farebbe divergere.

---

```
Riprendo un progetto già progettato, in corso.

Repo:   due macchine — C:\Users\zagor\Desktop\harness
                       E:\ALL\DEV\MY_REPOS\daemon
Remote: https://github.com/devfrx/daemon.git · branch spec/sottoprogetto-1-kernel
Albero pulito, ramo pushato. Ultimo commit: <<< INCOLLA QUI: git log --oneline -1 >>>

⛔ PRIMA DI PROPORRE QUALSIASI COSA: leggi, poi ASCOLTA. Non entrare in plan
   mode, non proporre passi, non scrivere file finché non te lo dico.

INVOCA QUESTE SKILL PRIMA DI QUALSIASI RISPOSTA O ESPLORAZIONE
  /superpowers:using-superpowers
  /anthropic-skills:dev-discipline      governa il codice
  /anthropic-skills:dev-communication   governa la conversazione intorno al codice

E QUESTE QUANDO SERVIRANNO, NON PRIMA
  /superpowers:brainstorming                 prima di qualunque lavoro creativo
  /superpowers:writing-plans                 quando si scriverà un piano nuovo
  /superpowers:subagent-driven-development   per ESEGUIRE un piano: un subagente
                                             fresco per compito, con revisione fra
                                             uno e l'altro. È la modalità scelta
  /superpowers:test-driven-development       quando si scrive codice

LEGGI SOLO QUESTI DUE FILE, PER INTERO, POI FERMATI     — 101 KB in tutto
  1. CLAUDE.md
  2. docs/COMPENDIO.md — contiene TUTTE le decisioni del progetto: le 37 ADR
     compresse, le sei invarianti, le proprietà non retrofittabili, lo stack
     (§4), lo stato di oggi e il prossimo passo (§6), il non rilitigabile (§7),
     cosa NON rifare (§8), i quarantacinque gotcha (§9), le trappole di
     check-docs.sh (§10), i vincoli sul primo commit di codice (§11).

⛔ CODICE IN INGLESE, DOCUMENTAZIONE IN ITALIANO — §1.0 della spec.
   Crate, moduli, tipi, funzioni, messaggi d'uscita e commenti nel sorgente
   sono in inglese; i documenti restano in italiano; un riferimento al codice
   dentro un documento si scrive in inglese, col nome esatto del sorgente.
   Non è tipografia: la regola non stava scritta, e un traguardo intero è stato
   scritto con gli identificatori italiani e poi rifatto.

⚠️ IL REPOSITORY CONTIENE CODICE RUST, e fino a poco fa non era così.
   Serve rustup: rust-toolchain.toml tira giù da sé la 1.95.0 e il bersaglio
   x86_64-unknown-none, quindi non installare niente a mano. Su Windows serve
   in più il linker MSVC (Visual Studio Build Tools), che rustup NON porta.
   Il primo comando utile è
       bash scripts/gate.sh
   e dice in un colpo se l'ambiente regge: deve stampare GATE GREEN.

⛔ NON aprire docs/HANDOFF.md, la spec del sotto-progetto 1, né la cartella
   docs/adr/ «per farsi un'idea»: insieme pesano oltre mezzo megabyte, e
   l'idea è già nel compendio. Aprirai UN file — uno — quando ti servirà il
   perché di una decisione: le alternative scartate, le misure, i costi
   accettati. La §12 del compendio dice quale.

⚠️ Il compendio è una COMPRESSIONE, non una selezione: ci sono dentro tutte le
   decisioni, non quelle attinenti al compito di oggi. Sparisce il ragionamento
   lungo, non la decisione. Nessuna può sfuggirti perché «non sembrava attinente».

CINQUE COSE CHE RIBADISCO, ANCHE SE STANNO NEI FILE
  · Non sono operativo in Rust. Quando l'argomento esce dal mio dominio,
    spiegamelo PRIMA a parole semplici e POI schematizza.
  · Un piano scritto qui è un'IPOTESI, non un'istruzione — e le sue sonde
    sono la parte che sbaglia. Nel Traguardo 2 sei difetti su sei stavano
    nel piano e non nel codice: un test vacuo, una sonda che attaccava il
    caso invece del meccanismo, una regola guardata in una direzione sola,
    un conteggio inesistente, una variante inusabile, una misura vera ma
    di un'altra cosa. Prova in negativo PRIMA di crederci, e dove diverge
    registra la divergenza invece di allinearti all'attesa.
  · Alla chiusura di ogni voce COMMITTA E PUSHA senza chiedere, e SENZA
    co-autore. Prima però: bash scripts/gate.sh — comprende check-docs.sh.
  · Se ti viene un'idea che SOSTITUISCE una decisione presa, cerca PRIMA dove
    era già stata valutata e perché era caduta: la §7 del compendio elenca le
    riaperture più tentate e il costo di ciascuna. Riaprila solo con una prova
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
| il messaggio | ~9 KB | ~4 KB |
| lettura che ordinava | l'intero corpus, oltre mezzo megabyte | **101 KB** — `CLAUDE.md` più il compendio |
| decisioni note all'agente | tutte, dopo aver letto tutto | **tutte**, dopo 101 KB |

⚠️ **I due numeri di destra si rimisurano, e sono già stati falsi due volte.** Dicevano
*«24 KB, ~6k token»*: era vero quando il compendio pesava un terzo di oggi, e nessuno
l'aveva più rifatto. Poi hanno detto **88 KB** mentre erano **91**, ed è per questo che
questa riga è stata riscritta. È il gotcha **#31** — una cifra messa a sostegno di una
regola giusta non viene mai riverificata, perché nessuno dubita della regola. Il rapporto
resta quello che conta: **101 KB contro mezzo megabyte**.

Il messaggio lungo elencava undici letture «PER INTERO», fra cui **tutti** gli ADR, nove
diagrammi e la spec del sotto-progetto 1, che oggi pesa **267 KB**. Non è che chiedesse
troppo: chiedeva la cosa giusta nel posto sbagliato. Le decisioni servono **tutte** — ma
compresse, non integrali.

⚠️ **La quinta voce è nata il 2026-08-09**, eseguendo i Task 1–6 del Traguardo 2. Non è una
massima: è il ritratto di sei difetti su sei, e vive qui perché il prossimo agente riprende
**dentro** quel piano, dove la lezione serve al primo compito e non al decimo.

⚠️ **Cosa aggiornare qui, e quando.** Il ramo se cambia; il peso dei due file
obbligatori quando uno dei due cresce; il numero dei gotcha quando §9 ne guadagna uno; i
prerequisiti d'ambiente quando il repository comincia a richiedere uno strumento nuovo —
è successo con `rustup`, e prima di allora non serviva niente. ⛔ **Lo SHA non è in questo
elenco**, ed è la voce che è stata tolta: si compila incollando, non si mantiene.

📌 **Il percorso del repository ha due valori, e non è una svista da «sanare».** Si lavora
su **due macchine**, e il campo le nomina entrambe: un percorso solo sembrava aggiornato e
su una delle due era falso. Chi incolla il messaggio lascia pure tutti e due — l'agente
parte dentro il repository e la propria posizione la conosce già.
