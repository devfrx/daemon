# Istruzioni per l'agente

## ⛔ Prima cosa, e unica lettura obbligatoria

Leggi **questo file** e poi **[`docs/COMPENDIO.md`](docs/COMPENDIO.md)**, per intero.
Poi **fermati**.

Il compendio contiene **tutte** le decisioni del progetto — le 37 ADR, le sei
invarianti, lo stack, i gotcha, lo stato di oggi e il prossimo passo — ciascuna
compressa a poche righe.

⛔ **E la testa di [`docs/audit-2026-08-27.md`](docs/audit-2026-08-27.md) — tutto ciò che
sta PRIMA della sezione «Dettaglio».** ⚠️ **RICHIAMO DEL 2026-08-28: queste righe dicevano
*«Quel rapporto È APERTO ED È IL COMPITO DI OGGI, non una consultazione»*, e i suoi finding
sono chiusi.** Resta lettura d'apertura come **verbale e delega** — la sua sezione *«Come si
concludono quelli aperti»* è **il metodo con cui qui si rimedia**, e la colonna *«Stato»*
della sua tabella è la **casa unica** di che cosa resta, insieme alla tabella delle **voci
senza numero AUD**, che sono le sole ancora aperte e in gran parte del proprietario.
⛔ **Il «Dettaglio» dopo quella tabella NON si legge intero: se ne apre UNA scheda per
volta, quella del finding su cui si lavora.**

⚠️ **Quanto costa questa lettura NON è scritto qui, ed è una decisione.** Ogni volta che
un peso o un conto di token è stato scritto su questa riga è invecchiato: ha detto
«seimila token» a 24 KB, «88 KB» a 91, «165» a 192, «624» a 213. **Sei volte, il gotcha
**#31**.** Ora lo dice il comando, che non marcisce:

```bash
wc -c CLAUDE.md docs/COMPENDIO.md
```

📌 **E in token, se serve** — `pip install tiktoken`; è il tokenizzatore di OpenAI, quindi
su italiano con emoji il conto di Claude è più alto e questo è un **limite inferiore**:

```bash
python -c "import tiktoken,io; e=tiktoken.get_encoding('cl100k_base'); print(sum(len(e.encode(io.open(p,encoding='utf-8').read())) for p in ['CLAUDE.md','docs/COMPENDIO.md']))"
```

⚠️ **E [`docs/audit-2026-08-11.md`](docs/audit-2026-08-11.md) è uscito da questa riga il
2026-08-27:** è chiuso otto decisioni su otto, e ciò che insegnava vive nel compendio —
un file chiuso non è una lettura obbligatoria.

⛔ **Non aprire** [`docs/HANDOFF.md`](docs/HANDOFF.md), la spec del sotto-progetto 1, la
cartella [`docs/adr/`](docs/adr/) o [`docs/archivio/`](docs/archivio/) «per farsi
un'idea». Insieme sono **di gran lunga** la mole maggiore del repository — il conto lo dà
`find docs -name '*.md' | xargs wc -c | sort -n` — e l'idea è già nel compendio.
Quando ti servirà il **perché** di una decisione — le alternative scartate, le misure, i
costi accettati — apri **quel** file, uno solo. La §12 del compendio dice quale.

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
| ⛔ **Un numero misurato non si scrive: si scrive il COMANDO che lo produce** | un numero invecchia al primo commit che tocca ciò che misura; un comando no. Se il numero deve comparire — perché sostiene una decisione — porta accanto il **comando** e la **data**, e vive in **una** casa sola. ⚠️ **Misurato il 2026-08-28:** il **92%** della lettura obbligatoria era storia di numeri corretti, e la testa di questo file aveva sbagliato il proprio peso **sei volte**. Gotcha **#31** |
| ⛔ **Un verbale di correzione non resta nel documento corretto** | va in [`docs/archivio/`](docs/archivio/), con la data; il documento vivo porta ciò che è **vero adesso**. È la metà mancante della riga qui sopra: senza di essa la regola dice *dove* mettere il numero e non *dove* mettere la sua storia, e la storia si accumula esattamente dove la si è corretta. ⛔ **Il freno è nel cancello**, non nella buona volontà: `check-docs.sh` respinge un compendio sopra il proprio tetto |
| **Le misure nello scratchpad** | non nel repository, e si ripulisce dopo |
| ⛔ **I fine-riga sono misti _per file_** | non c'è una convenzione da seguire: c'è **un file da non cambiare**. Uno script che riscrive un sorgente ne normalizza i fine-riga senza dirlo, e `git diff` dichiara **seicento righe cambiate** che nessuno ha toccato — successo **tre volte**, l'ultima il 2026-08-18 con un `sed -i` su `crates/kernel/Cargo.toml`, **43 CR → 0**. Chi scrive uno strumento che tocca file **conserva i fine-riga di quel file**, e li **rimisura dopo** con `tr -cd '\r' \| wc -c` invece di fidarsi |
| ⛔ **Una dipendenza si aggiunge in _due_ passi** | dal 2026-08-18 il cancello passa `--locked` a **tutti** i suoi siti `cargo` — *tutti* è la relazione che regge quando il cancello guadagna un passo, una cifra no; il comando che la verifica sta in [`docs/riferimenti.md`](docs/riferimenti.md), in una casa sola — quindi il `Cargo.lock` è un **ingresso** e non più un effetto. Toccare un manifesto da solo lascia il cancello **rosso**: il lockfile si rinfresca **fuori** dal cancello — un `cargo build` senza il flag — e si committa **insieme** al manifesto. È il punto e non il prezzo: ADR-0031 chiama l'aggiunta di una voce *«un atto deliberato e rivedibile»*, e un lockfile che il cancello aggiornava da sé non era né l'uno né l'altro. Finding **G-5** |
| **Audit a ogni chiusura** | `bash scripts/check-docs.sh` prima di ogni commit di documentazione |
| **Commit e push** | alla chiusura di ogni voce si **committa e si pusha**, senza chiedere, e **senza co-autore** |

## Prima di eseguire un compito di un piano

⛔ **Un piano è un'ipotesi, e il pre-controllo di ogni compito _prima_ di dispacciarlo ha
trovato almeno un difetto reale in TUTTI i compiti dispacciati finora, senza una sola
eccezione.** ⚠️ **Nessun numeratore qui**, per costruzione: un cumulativo invecchia a ogni
compito, *«tutti, senza eccezione»* no.

Si fanno **quattro domande**, e **ciascuna coglie ciò che le altre tre non colgono**.

| | Il difetto | Che cosa lo coglie |
|---|---|---|
| 1 | la **sonda è sbagliata** — vacua, o attacca il caso invece del meccanismo | **rileggere** |
| 2 | la **sonda manca** | *per ogni artefatto che il compito produce, quale controllo lo esercita?* Non si vede leggendo: non c'è niente da leggere |
| 3 | l'**artefatto è sbagliato**, e compila | **solo** scriverne un'implementazione **da fuori dalla crate** |
| 4 | il **compito è già eseguito** | *ciò che detta di produrre esiste già?* |

⛔ **E tre cose che l'elenco NON coglie, una riga l'una.** Sono istruzioni, non aneddoti:
il **caso** che ciascuna ha prodotto vive in [`docs/HANDOFF.md`](docs/HANDOFF.md), nel
gotcha che porta il suo numero, e si apre solo se serve.

| | La regola | Il caso |
|---|---|---|
| 5 | ⛔ **Il contratto cresce sotto il piano: un compito scritto prima si legge contro il codice di ADESSO, non contro il piano** | Traguardo 3 — un compito dettava «le cinque operazioni» quando erano sei, uno congelava quattro campi quando erano cinque, uno attendeva un rosso che era verde |
| 6 | ⛔ **Vale anche per un DISEGNO, e le guardie non sono tutto il codice: ciò che ti smentisce può stare in un BANCO DI PROVA, perfino in un commento** | gotcha **#58** |
| 7 | ⛔ **Un ADR si legge anche contro i propri FRATELLI**, non solo contro il codice: due decisioni della stessa data possono contraddirsi senza che nessuna delle due nomini l'altra | gotcha **#59** |
| 8 | ⛔ **Un RAPPORTO è un piano, e si prezza leggendo il CODICE — in ENTRAMBE le direzioni**: può chiedere più del necessario, e può chiedere meno | gotcha **#65** |

⚠️ **Il testo lungo di queste quattro righe è stato tolto da qui il 2026-08-28**, non
perso: erano quattro paragrafi che ricopiavano i gotcha **#49**, **#58**, **#59** e **#65**
di [`docs/HANDOFF.md`](docs/HANDOFF.md), che ne è la casa. ⛔ **Ciò che è rimasto è
l'ISTRUZIONE**, perché questo file è l'unica lettura obbligatoria e `HANDOFF.md` non lo è:
una regola che sparisse di qui non verrebbe letta da nessuno. **Il caso** invece si apre
solo quando serve, ed è per questo che può vivere altrove.

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
