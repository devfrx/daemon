# Istruzioni per l'agente

## ⛔ Prima cosa, e unica lettura obbligatoria

Leggi **questo file** e poi **[`docs/COMPENDIO.md`](docs/COMPENDIO.md)**, per intero.
Poi **fermati**.

Il compendio contiene **tutte** le decisioni del progetto — le 37 ADR, le sei
invarianti, lo stack, i gotcha, lo stato di oggi e il prossimo passo — ciascuna
compressa a poche righe.

⚠️ **Insieme questi due file pesano 85 KB** (`wc -c`, il 2026-08-08), cioè poco più di
**ventimila token** con lo stesso rapporto che fu usato per prezzarli la prima volta.
**Non sono più «circa seimila token»**: quella cifra era vera quando pesavano 24 KB, e
nessuno l'ha più rifatta — gotcha **#31**, terza occorrenza. Restano comunque la lettura
più economica che esista qui: l'alternativa è mezzo megabyte.

⛔ **Non aprire** `docs/HANDOFF.md`, la spec del sotto-progetto 1, o la cartella
`docs/adr/` «per farsi un'idea». Insieme pesano **oltre mezzo megabyte** — 577 KB con `wc -c`
il 2026-08-08, e possono solo crescere; la spec da sola ne fa 259 — e l'idea è
già nel compendio. Quando ti servirà il **perché** di una decisione — le alternative
scartate, le misure, i costi accettati — apri **quel** file, uno solo. La §12 del
compendio dice quale.

⚠️ **Il compendio è una compressione, non una selezione.** Ci sono dentro tutte le
decisioni, non quelle pertinenti al compito di oggi. Sparisce il ragionamento lungo,
non la decisione: nessuna può sfuggirti perché «non sembrava attinente».

## Cos'è questo progetto, in quattro righe

Assistente desktop locale, utente singolo, Windows primario poi Linux, **GPU singola
RTX 5080 da 16 GB**, OpenRouter primario con inferenza locale opzionale.
**Piattaforma a quattro pilastri paritari** — conversazione e conoscenza, agenti e
coding, voce, generazione asset 3D — su un **kernel comune** (ADR-0001).

Il vincolo dominante non è funzionale ma **di risorsa**. Il kernel **non implementa
nessuna funzionalità utente**: fornisce i meccanismi.

⚠️ **Questo non è un repository di sola documentazione.** Il codice del prodotto si
scrive **qui**, e vive in [`crates/`](crates/): cinque crate, con `kernel` e `simulator`
in `no_std`. Gli spike in [`spikes/`](spikes/) restano **prove**, fuori dal workspace.
La porta di qualità si lancia con un comando solo — `bash scripts/gate.sh` — e la mappa
dei controlli è in [`docs/porta-di-qualita.md`](docs/porta-di-qualita.md).
Lo stato corrente e il prossimo passo stanno nella **§6 del compendio** — non qui, o si
disallineano.

## Skill da invocare, in questo repository

Vanno invocate **prima** di qualsiasi risposta o esplorazione, non dopo.

| Skill | Perché qui |
|---|---|
| `superpowers:using-superpowers` | è il preambolo: se una skill può applicarsi, si invoca |
| `anthropic-skills:dev-discipline` | governa il **codice**: esplora prima di scrivere, YAGNI, convenzioni del repo, niente scorciatoie non dichiarate |
| `anthropic-skills:dev-communication` | governa la **conversazione** intorno al codice: cosa si decide da soli e cosa si porta al proprietario |
| `superpowers:brainstorming` | prima di qualunque lavoro creativo, e **prima di entrare in plan mode** |
| `superpowers:writing-plans` | quando si scriverà il piano — **non prima** che le voci aperte siano chiuse |
| `superpowers:subagent-driven-development` | per **eseguire** un piano: un subagente fresco per compito, con revisione fra uno e l'altro. È la modalità scelta dal proprietario |
| `superpowers:test-driven-development` | quando comincerà il codice |

## Come si lavora qui

| Regola | |
|---|---|
| **Spec prima del codice** | nessun sotto-progetto si implementa senza spec approvata |
| ⛔ **Codice in inglese, documentazione in italiano** | **§1.0 della spec.** Crate, moduli, tipi, funzioni, messaggi d'uscita e commenti nel sorgente sono **in inglese**; i documenti restano **in italiano**; un riferimento al codice dentro un documento si scrive **in inglese, col nome esatto del sorgente**. ⚠️ Non è tipografia: la regola non stava né qui né nel compendio, e un traguardo intero è stato scritto con gli identificatori italiani e poi rifatto — gotcha **#40** |
| **Sezione per sezione** | si presenta, si discute, **si approva**, si scrive. Mai tutto insieme |
| **Decidere sul merito** | né scorciatoie né sovra-ingegnerizzazione. «Non pigro» **non** significa «più costoso» |
| **Rendere verificabile** | un principio che non si può controllare è un'intenzione. Gli invarianti diventano test |
| **Un'evidenza scritta prima della misura è un'ipotesi** | si misura, e dove diverge **si registra la divergenza** invece di allinearsi all'attesa |
| **Un controllo si prova in due direzioni** | che scatti dove deve, **e che non scatti dove non deve**. La seconda si dimentica |
| **Schema-first** | tabelle, diagrammi, elenchi numerati. Niente muri di testo |
| **Ma prima a parole** | quando l'argomento esce dal dominio del proprietario (non è operativo in Rust), si spiega **prima** a parole semplici e **poi** si schematizza |
| **Stato dell'arte verificato** | se una nozione non è certa si cerca **prima** di scrivere, e la fonte si traccia in [`docs/riferimenti.md`](docs/riferimenti.md). **Mai inventare** |
| **Dichiarare i costi** | ogni decisione elenca ciò che peggiora. Un ADR senza `Negative (accettate)` è incompleto |
| **Un'idea nuova può essere già stata scartata** | prima di proporre qualcosa che **sostituisce** una decisione presa, si cerca **dove era già stata valutata e perché era caduta**. Si riapre **solo con una prova nuova**; e se la prova nuova gioca contro, si **registra e si chiude**. Vale anche — soprattutto — per le proprie idee |
| **ADR append-only** | superato → `Superseded by`; completato → un **rimando**. Completare una riga di verifica **non** è superare l'ADR |
| **Richiamo datato** | ogni correzione a una sezione approvata porta il proprio richiamo con la data |
| **Le misure nello scratchpad** | non nel repository, e si ripulisce dopo |
| **Audit a ogni chiusura** | `bash scripts/check-docs.sh` prima di ogni commit di documentazione |
| **Commit e push** | alla chiusura di ogni voce si **committa e si pusha**, senza chiedere, e **senza co-autore** |

## Manutenzione della documentazione

Alla chiusura di ogni sotto-progetto si aggiornano **nello stesso passaggio**:
[`docs/COMPENDIO.md`](docs/COMPENDIO.md), [`docs/roadmap.md`](docs/roadmap.md),
[`docs/README.md`](docs/README.md), [`docs/tracciabilita.md`](docs/tracciabilita.md),
lo stato degli spike, [`docs/HANDOFF.md`](docs/HANDOFF.md) se emergono gotcha nuovi, e
questo file se cambia il modo di lavorare.

Alla chiusura di ogni **voce** — non solo di un sotto-progetto — si aggiornano
[`docs/COMPENDIO.md`](docs/COMPENDIO.md) e [`docs/HANDOFF.md`](docs/HANDOFF.md), e
[`docs/riferimenti.md`](docs/riferimenti.md) **se la voce ha portato una misura o una
fonte**. Poi si committa e si pusha.

⛔ **Il compendio non può restare indietro**, e non è lasciato alla buona volontà:
`check-docs.sh` pretende una voce in §5 per **ogni** file in `docs/adr/`. Un ADR nuovo
senza voce è un **rosso**. Vedi §13 del compendio.

Un documento di stato disallineato è peggio di nessun documento: **mente con
autorevolezza**.
