# I modelli decisionali «System One» — la consegna dell'avvio

## Stato in una riga

⏳ **Non è un disegno: è la consegna dell'avvio**, scritta il 2026-09-28 dalla sessione che ha chiuso **E130**, dopo le
ricerche che il proprietario ha chiesto. Il brainstorming si apre in una **sessione nuova**, per sua decisione. Questo file
diventa il diario del brainstorming e poi il disegno, **allo stesso percorso**, com'è stato per il
[design system](2026-09-22-design-system-design.md).

## ⛔ Da sapere subito

1. **[ADR-0020](../../adr/0020-nessun-modello-nel-percorso-decisionale-del-kernel.md) decide il posto prima di ogni altra
   cosa:** un modello non entra mai nel percorso decisionale del kernel, e il suo esito è un **dato opaco**. Il proprietario
   chiede la funzione *«nel core e kernel del programma, come se fosse nata insieme ad esso»*: può nascere insieme **nei
   posti che il kernel ha già** — un worker, un sensore inferenziale, uno strumento delle capacità —, non nelle decisioni
   del kernel. Cambiarlo è un ADR che supera lo 0020, e la §7 del [compendio](../../COMPENDIO.md) lo mette fra le decisioni
   non rilitigabili: senza, la simulazione deterministica non si fa.
2. **I tre modelli hanno pochi giorni:** Laya è del 2026-09-18, rizzo-flow del 2026-09-21, Jev in accesso anticipato dal
   2026-09-27. *Novità non è maturità* — il criterio 4 di `anthropic-skills:decision-principles` —: si decidono prima il
   **posto** e il **contratto**, e il **modello** tardi, misurato sulla macchina.
3. **Le cifre qui sotto sono lette attraverso lo strumento di lettura della sessione**, che riassume la pagina: una cifra si
   rilegge alla fonte prima di entrare in un disegno. Le fonti in [`riferimenti.md`](../../riferimenti.md), *«I modelli
   decisionali «System One» — le fonti dell'avvio»*.
4. **Il prossimo passo del progetto resta il 13**, e questo fronte non lo sbarra: il perché nella proposta 3, più sotto.

## Che cosa chiede il proprietario

Parola per parola, il 2026-09-28:

> *«no ora non è necessario, inoltre ti faccio una domanda, conosci jev/laya/rizzo-flow e tutte queste ia decisionali non
> regressive? se non le conosci cercale e documentati su come funzionano ed a cosa servono, se volessi implementarne una
> locale (che sia attivabile a scelta) nel core e kernel del programma come feature come se fosse nata insieme ad esso,
> inserirla negli schemi disegnati e darle una posizione logica e strutturale nell'architettura, in modo professionale e
> secondo i principi di /anthropic-skills:decision-principles in che modo potremmo farlo? e quando? e soprattutto che cosa
> potremmo fargli fare? potrebbe enhanchare il lato agentico/jarvis e da agentic OS e se sì in che modo? (sicuramente in una
> sessione nuova)»*

## Che cosa sono — lette alle fonti il 2026-09-28

Modelli che **non scrivono testo**. Ricevono uno **stato** — testo, un messaggio, un JSON — e delle **domande tipizzate** con
le risposte ammesse già decise da chi chiede: sì o no, una fra N scelte, un punteggio, un numero. In **una sola passata**
rendono la risposta con una **probabilità** per ogni opzione. Non c'è niente da analizzare dopo, e niente fuori dalla lista.
L'addestramento che i due originali nominano, **RLCD**, premia con regole di punteggio *strettamente proprie*: l'unico modo
di vincere è dichiarare probabilità oneste.

| | **Jev** | **Laya** | **rizzo-flow** |
|---|---|---|---|
| chi, quando | TypeSafe AI, accesso anticipato dal 2026-09-27 | Convai Innovations, 2026-09-18 | Rizzo AI Academy, 2026-09-21 |
| licenza | non detta | Apache-2.0 | Apache-2.0 |
| dove gira | **solo** come servizio in rete | in locale, `pip install laya`, su CPU o GPU | in locale, un server HTTP su `localhost` sopra `llama.cpp`, con un'interfaccia compatibile con quella di Jev |
| com'è fatto | un'architettura nuova, con un campionatore parallelo | un **encoder**: ModernBERT-large da 421M (512 token), mmBERT-base multilingue da 322M (1024), e un punto addestrato sulle decisioni tipizzate | un **LLM piccolo** — Spark-X2.5 da 4B o 1.7B, messo a punto — di cui legge in una passata le probabilità delle **lettere** di risposta, A–Z |
| domande | fino a 255 opzioni | scelta, punteggio, sì o no | sì o no, scelta, punteggio, numero, con astensione; al più 26 opzioni |
| tempo dichiarato | 70–500 ms dall'inizio alla fine | ~33–40 ms su una T4 | ~50 ms a decisione col 4B |
| memoria dichiarata | — | piccola; va anche su CPU | ~5,6 GiB col 4B in Q8_0, ~2,3 GiB col 1.7B, misurati su una RTX 5060 Ti da 16 GB |
| limiti dichiarati | non genera testo | i punti di base rispondono **quasi a caso** senza addestramento sul compito (0,362); escono troppo sicuri e vogliono la taratura della temperatura sui propri dati; il punteggio ordinale è il più debole | probabilità **non tarate** per default; sei risposte sicure e sbagliate su trentasei quando manca l'evidenza; un modello residente e le richieste in fila; provato su una macchina |

## Che cosa esiste già nel progetto — verificato il 2026-09-28

| Decisione | Che cosa dice a questo fronte |
|---|---|
| [ADR-0020](../../adr/0020-nessun-modello-nel-percorso-decisionale-del-kernel.md) | nessun componente del kernel decide con un modello; il verdetto di un sensore **inferenziale** è un dato opaco; nell'anello 4 la *proposta* può essere inferenziale, il *rilevamento* no; e *«la tentazione di violarla arriverà travestita da miglioramento»*: si tratta come un ADR, non come un'aggiunta |
| [ADR-0009](../../adr/0009-guide-sensori-e-anelli-sono-meccanismi-di-kernel.md) | il **registro dei sensori**, col contratto `(artefatto) → (verdetto, dettaglio, costo)` e la classificazione per costo: un modello decisionale è un sensore inferenziale **economico** |
| [`tracciabilita.md`](../../tracciabilita.md) | il *«Classificatore di sicurezza delle azioni»* è già mappato: 🔶, si realizza come **sensore**, la politica sta negli Agenti |
| [ADR-0004](../../adr/0004-topologia-di-processo.md) e [ADR-0028](../../adr/0028-ecosistema-dei-worker-ml.md) | un modello vive in un **worker**: Python, senza stato, uccidibile, niente GPU senza concessione |
| [ADR-0039](../../adr/0039-telecamera-come-sorgente-di-percezione.md) | la forma della telecamera: **spenta per default**, accesa da una funzione del registro, la concessione chiesta all'accensione, e *«riservato»* la spegne; il **primo worker vero** lo paga il sotto-progetto 12 |
| [ADR-0038](../../adr/0038-registro-delle-funzioni-del-programma.md) | il registro: la stessa porta e la stessa tripla per ogni invocatore; un evento di percezione **informa, mai autorizza** |
| [ADR-0014](../../adr/0014-confine-dei-dati-non-fidati-nel-sistema-di-tipi.md) | un'istruzione trovata nei dati non è mai un'autorizzazione: una decisione presa su contenuto non fidato chiede la stessa autorizzazione |
| [ADR-0005](../../adr/0005-arbitrato-gpu-su-due-dimensioni.md) | ogni tipo di lavoro dichiara un **profilo di risorsa**; un modello su CPU non chiede concessioni GPU |
| [ADR-0011](../../adr/0011-routing-risolto-e-giornalato-per-richiesta.md) | l'inferenza **generativa** è un passo di una run, quella **percettiva** sempre accesa è una sorgente di eventi: un modello decisionale non è né l'una né l'altra, e dove cada è una domanda aperta |
| [ADR-0003](../../adr/0003-estensibilita-solo-mcp-e-skill-dichiarative.md) | nessun codice di terzi nel processo dell'applicazione: un worker, o un processo esterno |

**Che cosa arriva**, dalla [roadmap](../../roadmap.md): il **13** — registro delle guide, **trigger** e proiezione — è il
prossimo; poi il **3**, la conversazione; il **4**, gli agenti, introduce **sensori e anelli**; il **6** la knowledge base;
il **12** paga il primo worker vero; l'**8** è la voce.

## Le proposte del coordinatore — dedotte, non decisioni

1. **Il posto.** Un **worker decisionale** sotto il core, con un **contratto** che non dipende dal modello — uno stato e delle
   domande tipizzate in ingresso, risposte tipizzate con le probabilità in uscita —, così il modello si cambia senza toccare
   nient'altro. Lo usano le **capacità**, per le proprie scelte, e il **registro dei sensori**, come sensore inferenziale. È
   **spento per default** e si accende con una funzione del registro, come la telecamera; porta il suo profilo di risorsa, o
   nessuna concessione se gira su CPU. Negli schemi: `design/01` per i processi, `design/04` per anelli e sensori.
2. **Gli usi, uno per meccanismo** — il lato «Jarvis» e da Agentic OS:
   - **smistare** ciò che dice la voce o fa un gesto verso la funzione giusta del registro, in decine di millisecondi e anche
     senza rete;
   - **giudicare un evento** che un trigger consegna — *«vale la pena svegliare un agente?»* —: il sistema che si muove da
     solo senza pagare un LLM a ogni evento; giudica la **capacità** svegliata, non il kernel;
   - **sensori veloci** dopo un passo: *«il compito è finito?»*, *«questa azione esce dall'ambito?»* — la riga del
     classificatore di sicurezza di `tracciabilita.md`;
   - **mettere in ordine** la knowledge base: un file nuovo, in quale gruppo;
   - **scegliere quale modello chiedere** al gateway, economico o grande: lo sceglie la capacità, e il gateway resta a regole.
3. **Quando.** **Non prima del 13**: i trigger del kernel non hanno bisogno di sapere dei modelli, perché a giudicare è la
   capacità che il trigger sveglia. Il brainstorming quando il proprietario lo colloca; poi uno **spike** che misura i
   candidati sulla nostra macchina — VRAM, tempo, taratura sui nostri casi —, come SP-5…SP-8; il sotto-progetto col primo
   numero libero, **dopo il 12**, che paga il primo worker vero, e **vicino al 4**, dove nascono i sensori.
4. **Che cosa non si fa.** Il modello nell'arbitro, nella scelta del candidato del gateway, nei permessi o nel rilevamento
   delle ricorrenze: sono le rinunce che l'ADR-0020 accetta per nome.

## Le domande del brainstorming, una alla volta — proposte, non decisioni

1. Quale uso **per primo**: lo smistamento di voce e gesti, il giudizio sui trigger, i sensori, la knowledge base?
2. **CPU o GPU**: un encoder piccolo come Laya, senza concessione, o un LLM piccolo come rizzo-flow, che su una 5080 da 16 GB
   condivisa da quattro pilastri prende un terzo della memoria?
3. Una chiamata al worker decisionale è un **passo** giornalato col suo costo (ADR-0011), un **verdetto** di sensore
   (ADR-0009), o l'uno o l'altro secondo chi chiama?
4. Il **sotto-progetto** nuovo: dove nella roadmap, e se lo spike viene prima del disegno.
5. La **GUI**: dove si accende e si spegne — le Impostazioni? — e dove si vedono le probabilità — Passi, Attività?

## Prossimo passo, eseguibile

In una sessione nuova, quando il proprietario lo colloca: la lettura d'apertura di `CLAUDE.md`; poi **questo file intero**;
per il perché, l'ADR-0020 e l'ADR-0009, un file per volta; poi `superpowers:brainstorming`, con le domande **una alla
volta**, in A/B col consiglio; il diario qui, committato a ogni risposta. Prima di scrivere una cifra, la fonte riletta.

## Come tornare operativi

`git fetch --all --prune`, poi `git status -sb`: il ramo `main`, allineato. Il commit che scrive questo file lo dice
`git log --oneline -1 -- docs/superpowers/specs/2026-09-28-modelli-decisionali-design.md`.
