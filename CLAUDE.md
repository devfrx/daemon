# Istruzioni per l'agente

## ⛔ Prima cosa, e unica lettura obbligatoria

1. **`git fetch --all --prune`**, poi `git status -sb`: si lavora da più macchine. Se il ramo è indietro e l'albero è
   pulito, `git merge --ff-only` **subito**, prima di leggere qualunque documento di stato.
2. Leggi **questo file** e **[`docs/COMPENDIO.md`](docs/COMPENDIO.md)**, per intero. Poi **fermati**.
3. Di [`docs/audit-2026-08-27.md`](docs/audit-2026-08-27.md) **solo due pezzi**: la tabella *«Le voci aperte che NON hanno
   un numero AUD»* — le sole ancora aperte, in gran parte del proprietario — e *«La disciplina, in cinque passi»*, il metodo
   con cui qui si rimedia. I 73 finding sono chiusi — lo dice il comando in fondo alla §6 del compendio — e del
   «Dettaglio» si apre **una** scheda per volta, quella del finding su cui si lavora.

Il compendio contiene **tutte** le decisioni del progetto — le ADR, le sei invarianti, lo stack, le trappole, lo stato di
oggi e il prossimo passo — ciascuna compressa a poche righe. ⚠️ **È una compressione, non una selezione:** sparisce il
ragionamento lungo, non la decisione, quindi nessuna può sfuggirti perché «non sembrava attinente».

⛔ **Non aprire** [`docs/HANDOFF.md`](docs/HANDOFF.md), la spec del sotto-progetto 1, [`docs/adr/`](docs/adr/) o
[`docs/archivio/`](docs/archivio/) «per farsi un'idea»: sono la mole maggiore del repository — il conto lo dà
`find docs -name '*.md' | xargs wc -c | sort -n` — e l'idea è già nel compendio. Per il **perché** di una decisione — le
alternative scartate, le misure, i costi accettati — si apre **un** file, quello che la §12 del compendio indica.

📌 **Il peso di questa lettura lo dà il comando, mai una cifra scritta** — gotcha **#31**. In byte, e in token con
`tiktoken` (`pip install tiktoken`), che è il tokenizzatore di OpenAI: su italiano con emoji il conto è un **limite
inferiore**.

```bash
wc -c CLAUDE.md docs/COMPENDIO.md
python -c "import tiktoken,io; e=tiktoken.get_encoding('cl100k_base'); print(sum(len(e.encode(io.open(p,encoding='utf-8').read())) for p in ['CLAUDE.md','docs/COMPENDIO.md']))"
```

## Cos'è questo progetto, in quattro righe

Assistente desktop locale, utente singolo, Windows primario poi Linux, **GPU singola RTX 5080 da 16 GB**, OpenRouter
primario con inferenza locale opzionale. **Piattaforma a quattro pilastri paritari** — conversazione e conoscenza, agenti e
coding, voce e gesti, generazione asset 3D — su un **kernel comune** (ADR-0001, col rimando datato in testa). Il vincolo
dominante non è funzionale ma **di risorsa**. Il kernel **non implementa nessuna funzionalità utente**: fornisce i
meccanismi.

⚠️ **Non è un repository di sola documentazione.** Il codice del prodotto vive in [`crates/`](crates/) — cinque crate, con
`kernel` e `simulator` in `no_std` — e in [`gui/`](gui/): la SPA e il core finto, fuori dal workspace e provati da
`scripts/gate-gui.sh`. Gli spike in [`spikes/`](spikes/) restano **prove**, fuori dal workspace. La porta di qualità si
lancia con un comando solo — `bash scripts/gate.sh` — e la mappa dei controlli è in
[`docs/porta-di-qualita.md`](docs/porta-di-qualita.md). Lo stato corrente e il prossimo passo stanno **solo** nella §6 del
compendio.

## Skill da invocare, in questo repository

Si invocano **prima** di qualsiasi risposta o esplorazione, quando si applicano — non dopo.

| Skill | Perché qui |
|---|---|
| `superpowers:using-superpowers` | il preambolo: se una skill può applicarsi, si invoca |
| `anthropic-skills:decision-principles` | governa **ogni** decisione non banale. Prima di chiedere il sì al proprietario: i cinque criteri controllati **esplicitamente**, e verificato, dedotto e assunto **separati** |
| `anthropic-skills:dev-discipline` | governa il **codice**: esplora prima di scrivere, YAGNI, convenzioni del repo, niente scorciatoie non dichiarate |
| `anthropic-skills:dev-communication` | governa la **conversazione** intorno al codice: cosa si decide da soli e cosa si porta al proprietario |
| `anthropic-skills:session-resume` e `anthropic-skills:session-handoff` | per riprendere e per chiudere una sessione, con la regola di questo repository sulla consegna: la dice *«Manutenzione della documentazione»*, in fondo |
| `superpowers:brainstorming` | prima di qualunque lavoro creativo, e **prima di entrare in plan mode** |
| `superpowers:writing-plans` | per scrivere un piano. Le voci aperte **si sanno prima di scrivere**: a sbarrare è la colonna *«Chi la chiude»* di [`docs/porta-di-qualita.md`](docs/porta-di-qualita.md) — una voce il cui chiusore è **questo traguardo** o **il proprietario, prima** va chiusa o portata dal piano; le altre si conoscono e si dichiarano |
| `superpowers:subagent-driven-development` | per **eseguire** un piano: un subagente fresco per compito, con revisione fra uno e l'altro — la modalità scelta dal proprietario. I subagenti con `model: "opus"` (`"sonnet"` per il lavoro meccanico), mai Fable; più di un subagente solo dopo aver detto il costo e avuto il sì |
| `superpowers:test-driven-development` | per ogni riga di codice di prodotto |

## Come si lavora qui

| Regola | |
|---|---|
| **Spec prima del codice** | nessun sotto-progetto si implementa senza spec approvata |
| ⛔ **Una fase per sessione** | brainstorming, disegno, piano, pre-controllo, ogni compito: ciascuno nella **sua** sessione, anche quando è breve — decisione del proprietario. Come si passa da una all'altra lo dice *«Manutenzione della documentazione»* |
| ⛔ **Codice in inglese, documentazione in italiano** | §1.0 della spec: crate, moduli, tipi, funzioni, messaggi d'uscita e commenti nel sorgente **in inglese**; i documenti **in italiano**; un riferimento al codice dentro un documento si scrive col **nome esatto del sorgente**. Un traguardo intero è stato rifatto per questo — gotcha **#40** |
| **Sezione per sezione** | si presenta, si discute, **si approva**, si scrive. Mai tutto insieme |
| **Decidere sul merito** | né scorciatoie né sovra-ingegnerizzazione. «Non pigro» **non** significa «più costoso» |
| **Rendere verificabile** | un principio che non si può controllare è un'intenzione. Gli invarianti diventano test |
| **Un'evidenza scritta prima della misura è un'ipotesi** | si misura, e dove diverge **si registra la divergenza** invece di allinearsi all'attesa |
| **Un controllo si prova in due direzioni** | che scatti dove deve, **e che non scatti dove non deve**. La seconda si dimentica |
| **Schema-first, ma prima a parole** | tabelle, diagrammi, elenchi numerati, niente muri di testo. Quando l'argomento esce dal dominio del proprietario — non è operativo in Rust — si spiega **prima** a parole semplici. Le domande al proprietario: **una per volta**, in forma **A/B**, col costo di ciascuna opzione e il consiglio |
| **Stato dell'arte verificato** | una nozione incerta si cerca **prima** di scrivere, alla fonte primaria, e la fonte va in [`docs/riferimenti.md`](docs/riferimenti.md). **Mai inventare** |
| ⛔ **Uno schema è una verifica, e corregge ciò che esiste** | decisione del proprietario del 2026-09-08, per ogni studio, brainstorming o diagramma: prima dell'A/B si dicono a parole che cosa **esiste già** (codice, ADR, disegni), che cosa **arriva** (la roadmap) e se **regge crescendo**. Se lo schema è più corretto di una logica, di un ADR o del codice, si correggono **quelli** — l'ADR col richiamo datato, il codice come compito del piano — in A/B, mai in silenzio |
| **Dichiarare i costi** | ogni decisione elenca ciò che peggiora. Un ADR senza `Negative (accettate)` è incompleto |
| **Un'idea nuova può essere già stata scartata** | prima di proporre qualcosa che **sostituisce** una decisione presa, si cerca dove era già stata valutata e perché era caduta. Si riapre **solo con una prova nuova**; se la prova gioca contro, si **registra e si chiude**. Vale soprattutto per le proprie idee |
| **ADR append-only** | superato → `Superseded by`; completato → un **rimando**. Completare una riga di verifica **non** è superare l'ADR |
| **Richiamo datato** | ogni correzione a una sezione approvata porta il proprio richiamo con la data |
| ⛔ **Un puntatore o una cifra che vive in PIÙ documenti si TOGLIE, non si ricorregge** | riallinearlo lascia di nuovo la regola come unica difesa. I documenti secondari **rimandano** alla §6 del compendio invece di riscriverla: un rimando non marcisce. Lo stato **per traguardo** resta nelle tabelle di [`docs/roadmap.md`](docs/roadmap.md) e [`docs/README.md`](docs/README.md), e il perimetro di una passata si prende dal drift **misurato**, non dalla categoria. Gotcha **#68** |
| ⛔ **Un numero misurato non si scrive: si scrive il COMANDO che lo produce** | un numero invecchia al primo commit che tocca ciò che misura, un comando no. Se il numero sostiene una decisione, porta accanto il **comando** e la **data**, e vive in **una** casa sola. Gotcha **#31** |
| ⛔ **Un verbale di correzione non resta nel documento corretto** | va in [`docs/archivio/`](docs/archivio/) parola per parola, con la data; il documento vivo porta ciò che è **vero adesso**. ⛔ **Lo stesso per le chiusure di sessione:** un documento vivo — un piano, un disegno — ne tiene **una**, l'ultima, e a ogni chiusura la precedente va in archivio; i piani già eseguiti restano come sono, perché sono verbali. E per le **note di memoria** dell'agente: regole corte, la storia fuori. Il freno è nel cancello: `check-docs.sh` respinge un compendio sopra il proprio tetto |
| **Le misure nello scratchpad** | non nel repository, e si ripulisce dopo |
| ⛔ **I fine-riga sono misti _per file_** | non c'è una convenzione da seguire: c'è **un file da non cambiare**. Uno strumento che tocca file **conserva i fine-riga di ciascuno** — in Python con `newline=""`, mai `sed -i` — e li **rimisura dopo** con `tr -cd '\r' \| wc -c`; altrimenti `git diff` dichiara centinaia di righe che nessuno ha toccato |
| ⛔ **Una dipendenza si aggiunge in _due_ passi** | il cancello passa `--locked` a **tutti** i suoi siti `cargo` — il comando che lo verifica sta in [`docs/riferimenti.md`](docs/riferimenti.md) — quindi il `Cargo.lock` è un **ingresso**. Il lockfile si rinfresca **fuori** dal cancello, con un `cargo build` senza il flag, e si committa **insieme** al manifesto: ADR-0031 vuole che aggiungere una voce sia *«un atto deliberato e rivedibile»*. Finding **G-5** |
| **Il cancello, uno alla volta** | `bash scripts/check-docs.sh` prima di ogni commit di documentazione; `bash scripts/gate.sh` **da solo** — due cancelli insieme si pestano su `gui/node_modules` (`EBUSY`, successo il 2026-09-23) — e il commit parte **solo sul verde** |
| **Commit e push** | alla chiusura di ogni voce si **committa e si pusha**, senza chiedere, e **senza co-autore** |

## Prima di eseguire un compito di un piano

⛔ **Un piano è un'ipotesi.** Il pre-controllo di ogni compito, **prima** di dispacciarlo, ha trovato almeno un difetto
reale in **tutti** i compiti dispacciati finora, senza eccezione. Si fanno **quattro domande**, e ciascuna coglie ciò che
le altre non colgono.

| | Il difetto | Che cosa lo coglie |
|---|---|---|
| 1 | la **sonda è sbagliata** — vacua, o attacca il caso invece del meccanismo | **rileggere** |
| 2 | la **sonda manca** | *per ogni artefatto che il compito produce, quale controllo lo esercita?* Non si vede leggendo: non c'è niente da leggere |
| 3 | l'**artefatto è sbagliato**, e compila | **solo** scriverne un'implementazione **da fuori dalla crate** |
| 4 | il **compito è già eseguito** | *ciò che detta di produrre esiste già?* |

⛔ **E ciò che le quattro domande NON colgono**, una riga l'una. Sono istruzioni, non aneddoti: il caso vive in
[`docs/HANDOFF.md`](docs/HANDOFF.md), nel gotcha col suo numero, e si apre solo se serve.

| | La regola | Il caso |
|---|---|---|
| 5 | ⛔ **Il contratto cresce sotto il piano: un compito scritto prima si legge contro il codice di ADESSO, non contro il piano** | Traguardo 3 — un compito dettava «le cinque operazioni» quando erano sei, uno congelava quattro campi quando erano cinque, uno attendeva un rosso che era verde |
| 6 | ⛔ **Vale anche per un DISEGNO, e le guardie non sono tutto il codice: ciò che ti smentisce può stare in un BANCO DI PROVA, perfino in un commento** | gotcha **#58** |
| 7 | ⛔ **Un ADR si legge anche contro i propri FRATELLI**, non solo contro il codice: due decisioni della stessa data possono contraddirsi senza che nessuna delle due nomini l'altra | gotcha **#59** |
| 8 | ⛔ **Un RAPPORTO è un piano, e si prezza leggendo il CODICE — in ENTRAMBE le direzioni**: può chiedere più del necessario, e può chiedere meno | gotcha **#65** |

## Manutenzione della documentazione

| Quando | Che cosa si aggiorna, nello stesso passaggio |
|---|---|
| alla chiusura di ogni **sessione** | la consegna nel documento in corso, sezione *«Come si riprende»* — o, se il lavoro non ha ancora un documento, in un file nuovo al percorso del suo futuro disegno; mai in `.handoff/`, la cartella che la skill vuole fuori da git: qui la consegna deve arrivare anche sulle altre macchine — con la chiusura precedente in archivio; il puntatore della §6 del compendio, se il prossimo passo cambia; commit e push |
| alla chiusura di ogni **voce** | [`docs/COMPENDIO.md`](docs/COMPENDIO.md) e [`docs/HANDOFF.md`](docs/HANDOFF.md), e [`docs/riferimenti.md`](docs/riferimenti.md) se la voce ha portato una misura o una fonte; commit e push |
| alla chiusura di ogni **sotto-progetto** | anche [`docs/roadmap.md`](docs/roadmap.md), [`docs/README.md`](docs/README.md), [`docs/tracciabilita.md`](docs/tracciabilita.md), lo stato degli spike, `HANDOFF.md` se emergono gotcha nuovi, e questo file se cambia il modo di lavorare |

⛔ **Il compendio non può restare indietro**, e non è lasciato alla buona volontà: `check-docs.sh` pretende una voce in §5
per **ogni** file in `docs/adr/`, e un ADR nuovo senza voce è un **rosso** (§13 del compendio). Un documento di stato
disallineato è peggio di nessun documento: **mente con autorevolezza**.
