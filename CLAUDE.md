# Istruzioni per l'agente

## ⛔ Prima cosa, e unica lettura obbligatoria

Leggi **questo file** e poi **[`docs/COMPENDIO.md`](docs/COMPENDIO.md)**, per intero.
Poi **fermati**.

Il compendio contiene **tutte** le decisioni del progetto — le 37 ADR, le sei
invarianti, lo stack, i gotcha, lo stato di oggi e il prossimo passo — ciascuna
compressa a poche righe.

⚠️ **Insieme questi due file pesano 445 KB** (byte LF, il 2026-08-20), e con
[`docs/audit-2026-08-11.md`](docs/audit-2026-08-11.md) — **chiuso il 2026-08-18, otto decisioni
su otto**, quindi oggi una **consultazione** — **476**. ⛔ **E il prezzo in
token che questa riga portava era sbagliato:** diceva *«circa venticinquemila»* col rapporto
usato per prezzarli la prima volta, e quel rapporto — mai rimisurato, perché nessuno dubita
del numero che sostiene una regola giusta — è stato **misurato il 2026-08-10** ed è sbagliato
**per difetto**. Il dato certo: leggendo il compendio, **quattrocento righe sole hanno pesato
`25148` token** — da sole quanto questa riga attribuiva ai due file **interi** — e il
compendio ne ha **oltre tremila**. ⚠️ **Un limite inferiore e non un totale, come la
frase sul mezzo megabyte:** il file cresce, e un limite inferiore misurato **resta vero
mentre cresce** — una cifra esatta no. ⚠️ **La cifra dei token non è scritta apposta:**
servirebbe un contatore dedicato, e un totale ricavato da un campione sarebbe un'ipotesi
travestita da misura. Il metodo e il limite stanno nella §12 del compendio, quattordicesima
misura. 📌 È la **sesta** occorrenza del gotcha **#31** su questa riga, che prima diceva
«circa seimila token» quando i due file pesavano 24 KB, poi «88 KB» quando erano
già 91, e fino alla chiusura del Traguardo 3 **«165»** quando erano 192. Restano comunque la lettura più economica che esista qui: l'alternativa è **750 KB**.

⛔ **Non aprire** `docs/HANDOFF.md`, la spec del sotto-progetto 1, o la cartella
`docs/adr/` «per farsi un'idea». Insieme pesano **oltre mezzo megabyte** — **750 KB** in
byte LF il 2026-08-20, e possono solo crescere; la spec da sola ne fa **277** — e l'idea è
già nel compendio. ⚠️ **Rimisurati lo stesso giorno, dal Task 2 del Traguardo 3**, che li
aveva appena fatti crescere: dicevano «622» e «271», scritti poche ore prima. È la ragione
per cui la riga qui sopra prezza i token come **limite inferiore** e non come totale — un
compito che scrive nella spec invalida una cifra esatta **nel commit stesso in cui la
legge**. Il verbale è la **sedicesima misura** della §12; l'ultima rimisura è **quella in fondo alla §12** — formulata così apposta, perché l'ordinale invecchia: diceva «la ventiduesima» quando i riquadri erano **ventiquattro**, e faceva saltare le due passate che avevano mosso di più. Quando ti servirà il **perché** di
una decisione — le alternative scartate, le misure, i costi accettati — apri **quel** file,
uno solo. La §12 del compendio dice quale.

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
| ⛔ **Un puntatore o una cifra che vive in PIÙ documenti si TOGLIE, non si ricorregge** | riallinearlo lo rimette nello stato in cui la **regola** è di nuovo l'unica difesa, e quella regola non ha retto **tre volte**. I documenti secondari **rimandano** alla §6 del compendio invece di riscriverla: un rimando non può marcire. ⚠️ Lo stato **per traguardo** resta nelle tabelle di [`docs/roadmap.md`](docs/roadmap.md) e [`docs/README.md`](docs/README.md) — il perimetro di una passata si prende dal drift **misurato**, non dalla categoria. Gotcha **#68** |
| **Le misure nello scratchpad** | non nel repository, e si ripulisce dopo |
| ⛔ **I fine-riga sono misti _per file_** | non c'è una convenzione da seguire: c'è **un file da non cambiare**. Uno script che riscrive un sorgente ne normalizza i fine-riga senza dirlo, e `git diff` dichiara **seicento righe cambiate** che nessuno ha toccato — successo **tre volte**, l'ultima il 2026-08-18 con un `sed -i` su `crates/kernel/Cargo.toml`, **43 CR → 0**. Chi scrive uno strumento che tocca file **conserva i fine-riga di quel file**, e li **rimisura dopo** con `tr -cd '\r' \| wc -c` invece di fidarsi |
| ⛔ **Una dipendenza si aggiunge in _due_ passi** | dal 2026-08-18 il cancello passa `--locked` a tutti e **sei** i suoi siti `cargo`, quindi il `Cargo.lock` è un **ingresso** e non più un effetto. Toccare un manifesto da solo lascia il cancello **rosso**: il lockfile si rinfresca **fuori** dal cancello — un `cargo build` senza il flag — e si committa **insieme** al manifesto. È il punto e non il prezzo: ADR-0031 chiama l'aggiunta di una voce *«un atto deliberato e rivedibile»*, e un lockfile che il cancello aggiornava da sé non era né l'uno né l'altro. Finding **G-5** |
| **Audit a ogni chiusura** | `bash scripts/check-docs.sh` prima di ogni commit di documentazione |
| **Commit e push** | alla chiusura di ogni voce si **committa e si pusha**, senza chiedere, e **senza co-autore** |

## Prima di eseguire un compito di un piano

⛔ **Un piano è un'ipotesi, e il pre-controllo di ogni compito _prima_ di dispacciarlo ha
trovato almeno un difetto reale in TUTTI i compiti dispacciati finora, senza una sola
eccezione** — dodici su dodici al Traguardo 3, dieci su dieci al Traguardo 4, e **tutti** quelli
eseguiti del Traguardo 5, il cui numeratore vive nella **§6 del compendio** e non qui.
⚠️ **Questa riga portava il totale — *«ventidue su ventidue»* — e contava due traguardi su tre**:
un cumulativo invecchia a ogni compito, mentre *«tutti, senza eccezione»* è un'affermazione che
resta vera mentre il conto cresce, come il *«oltre mezzo megabyte»* qui sopra. Tolto, non
riallineato. Si fanno **quattro domande**,
e **ciascuna coglie ciò che le altre tre non colgono**. Il testo lungo, coi casi, sta nel
gotcha **#49** della §9 del compendio e in [`docs/AVVIO-CHAT.md`](docs/AVVIO-CHAT.md): qui c'è
il solo elenco, perché è quello che si rilegge prima di dispacciare.

| | Il difetto | Che cosa lo coglie |
|---|---|---|
| 1 | la **sonda è sbagliata** — vacua, o attacca il caso invece del meccanismo | **rileggere** |
| 2 | la **sonda manca** | *per ogni artefatto che il compito produce, quale controllo lo esercita?* Non si vede leggendo: non c'è niente da leggere |
| 3 | l'**artefatto è sbagliato**, e compila | **solo** scriverne un'implementazione **da fuori dalla crate** |
| 4 | il **compito è già eseguito** | *ciò che detta di produrre esiste già?* |

⛔ **E una quinta cosa, misurata eseguendo il Traguardo 3 e che non sta nell'elenco: il
contratto cresce sotto il piano.** Il Task 8 dettava «le cinque operazioni» quando erano sei;
il Task 10 congelava quattro campi quando erano cinque; il Task 11 attendeva un rosso che era
verde. **Un compito scritto prima si legge contro il codice di adesso, non contro il piano.**

⛔ **E vale anche per un DISEGNO — misurato il 2026-08-11, gotcha #58.** Il disegno del
Traguardo 4 fu scritto leggendo la spec, gli ADR e le **guardie**, e sbagliava due cose che
stavano nei **banchi di prova** — di cui una scritta in un **commento**. ⚠️ **Le guardie non sono
tutto il codice:** un documento che ha letto gli script del cancello, i manifesti e i sorgenti
delle dipendenze **si sente verificato**, ed è lì che smette di guardare i test. E un precedente
si cita per la **ragione** che lo ha prodotto, non per la forma.

⛔ **E una SESTA, che l'audit del 2026-08-11 ha prodotto e che nessuna delle cinque coglie: un ADR
si legge anche contro i propri FRATELLI.** [ADR-0026](docs/adr/0026-linguaggio-del-core.md)
dichiara fra le proprie conseguenze positive che *«il simulatore non va scritto da zero»* perché
esiste `madsim`; [ADR-0031](docs/adr/0031-dipendenze-del-kernel-parte-del-confine.md) — **con la stessa data** —
misura che `madsim` porta **55 crate** e lo scarta, e il codice gli dà ragione: `simulator` ha
**una** dipendenza e **512 righe** scritte a mano. ⚠️ **Nessuno dei due nomina l'altro**, quindi
la contraddizione non si vede da nessuno dei due lati, e le cinque domande qui sopra guardano
tutte il compito contro il **codice** — mai una decisione contro le decisioni vicine. Gotcha
**#59**.

⛔ **E una SETTIMA, misurata il 2026-08-17 eseguendo la prima decisione dell'audit: un RAPPORTO
è un piano, e si legge contro il codice come tutti gli altri.** L'audit prezzava quel rimedio
come *«un'aggiunta al contratto di una porta condivisa»*, cioè caro e strutturale; letto contro
il codice di oggi non serviva **nessuna** promessa nuova e **nessuna riga di prodotto** — mancava
uno **stato di prova**, non un contratto. ⚠️ E nella direzione opposta il rapporto era **corto**:
raggruppava due finding per **causa**, ma una suite che si ferma al primo rosso ha bisogno di un
bugiardo per **blocco**, non per causa — quindi erano tre. 📌 **Il rimedio si prezza leggendo il
codice, non il rapporto**, in entrambe le direzioni. Gotcha **#65**.

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
