# La direzione della GUI — la stella polare: la consegna del brainstorming del 2026-09-07

⚠️ **QUESTO FILE È LA CONSEGNA della sessione del 2026-09-07**, che ha ripreso il brainstorming del
sotto-progetto 2 dalla [consegna del 2026-09-06](2026-09-06-sottoprogetto-2-gui-minima-design.md) e,
per scelta del proprietario, lo ha **allargato alla forma di tutta la GUI**: non le due schermate del
2, ma la stella polare — Home, Lavoro, Compatta, il nucleo a pagina intera — di cui il 2 costruisce la
cornice più la propria fetta. Il proprietario ha fermato la sessione dopo il quarto wireframe, prima
delle sezioni scritte. La sessione che riprende legge questo file **per intero**, poi la consegna del 2,
conferma le approvazioni ancora sospese, chiude le sezioni che mancano una per volta, e scrive **sul
posto** i due disegni: questo file come *disegno della direzione della GUI*, l'altro come *disegno del
2*. Il testo di questa consegna va allora **parola per parola** in
`docs/archivio/consegna-brainstorming-direzione-gui.md`, coi soli link riscritti per la cartella — è il
viaggio della knowledge base (`07ab6dc` → `6a7967a`) e del 2 (`ae40fa0` → `664265a`).

⚠️ **Non è una spec e non è ancora il disegno.** Il prossimo passo sta nella §6 del
[compendio](../../COMPENDIO.md), in un posto solo.

## Stato in una riga

Stella polare a metà: sette decisioni del proprietario più la modularità, quattro wireframe disegnati e
salvati — Home approvata; Lavoro e Compatta col grafo **approvati come mappa alla ripresa del 2026-09-07**,
con la decisione 10 — ciò che un wireframe non porta va nel catalogo dei moduli — e la 11, il modello come indicatore; la sezione 1 **chiusa** — cinque tabelle piene e una corta, decisione 12 — e le decisioni 11, 12 e 13; le sezioni 2 e 3 **chiuse il 2026-09-08** — viste e disposizione con una **settima porta** (decisioni 14 e 15 delegate), e la fetta del 2 ritagliata (A) — e il **mandato** del proprietario per la sessione nuova, la passata sui diagrammi (decisione 16); nessun codice toccato; mancano le sezioni 4–6 della stella polare e le §7–§10 del 2,
poi i due disegni e il piano; **la quarta ripresa, il 2026-09-08, ha APERTO la passata sui diagrammi** (decisione 16): i nove file di
`docs/design/` letti diagramma per diagramma, la **decisione 17 delegata e presa** — la policy VRAM corrente è la proiezione del
giornale — e la **sezione 1 della passata, design/09, presentata e NON ancora approvata**; nessun diagramma toccato. ✅ **La quinta ripresa, lo stesso giorno, ha SCRITTO la
sezione 1** — approvata A con due correzioni trovate rispondendo alla domanda del proprietario su scalabilità ed esistente — e ha
preso la **decisione 18**: il metodo vale per ogni studio, brainstorming e diagramma futuro. ✅ **E la sezione 2 è cominciata:** design/07
e il rimando ad ADR-0019 scritti (A). ✅ **La sesta ripresa, lo stesso giorno, ha SCRITTO design/03** (A) e il richiamo alla
tabella Passi (A). ✅ **La settima ripresa, lo stesso giorno, ha SCRITTO design/08** (A): la sezione 2 della passata è
**chiusa**; la domanda sulla lettera **E** della §8.2 della spec è **posta e senza risposta** — si riprende da lì, poi dalla
**sezione 3**, i disegni nuovi, col materiale già letto. ✅ **L'ottava ripresa, lo stesso giorno, ha DECISO la lettera E su
delega — A, il numero è 12 — e l'ha scritta nella spec; poi ha SCRITTO design/10, il primo disegno della sezione 3 (A su
delega, decisione 20)**: si riprende dal secondo disegno, «la GUI dentro».

## ⛔ Da sapere subito

**Niente è a metà nel repository.** Albero pulito, nessuno stash, nessuna operazione git a metà, tutto
pushato, **nessun codice toccato**: questa sessione ha prodotto solo documenti — questo file, i tre
wireframe SVG nella cartella accanto, i richiami datati nella consegna del 2, il puntatore della §6.

✅ **Le due riprese del 2026-09-07 hanno toccato solo questo file** e, alla chiusura della seconda, il puntatore
della §6 del compendio; `GATE GREEN` rilanciato alla chiusura della seconda ripresa, dopo l'ultimo commit di merito.

✅ **La terza ripresa, il 2026-09-08, ha toccato solo questo file** e, alla chiusura, il puntatore della §6 del compendio:
stato riletto coi comandi, nessuna divergenza, `check-docs.sh` OK all'apertura; `GATE GREEN` rilanciato alla chiusura.

⛔ **Il prossimo passo NON è la sezione 4.** È il **mandato** del proprietario (decisione 16), per una sessione nuova con un
agente nuovo: la **passata sui diagrammi** — aggiornare quelli esistenti, crearli dove mancano (ER compresi), e **correggere la
progettazione** dove un diagramma mostra un errore. Può riaprire sezioni approvate: una correzione torna al proprietario in
forma A/B, con richiamo datato, mai in silenzio. Il come sta in «Prossimo passo, eseguibile».

✅ **La quarta ripresa, il 2026-09-08, ha toccato solo questo file** e il puntatore della §6 del compendio: stato riletto coi comandi,
nessuna divergenza; `GATE GREEN` rilanciato alla chiusura. ⛔ **La passata sui diagrammi è APERTA e nessun diagramma è scritto:** la
**sezione 1** — design/09 con le sue conseguenze — è **presentata e attende A/B**; la proposta sta parola per parola, col sorgente
mermaid, in «Prossimo passo, eseguibile», e si scrive **solo dopo il sì**. La **decisione 17** — la policy VRAM, delegata — è presa e
sta nella tabella delle decisioni del proprietario; ciò che costa si scrive con la sezione 1.

✅ **La quinta ripresa, il 2026-09-08, ha SCRITTO la sezione 1** — design/09, i rimandi in testa ad ADR-0022 e ADR-0006, una riga in
design/02 e una in design/05 — e ha toccato questo file, `CLAUDE.md` (una riga in «Come si lavora qui», decisione 18) e il puntatore
della §6 del compendio; `check-docs.sh` OK e `GATE GREEN` all'apertura, `check-docs.sh` OK alla chiusura. ⛔ **La passata continua
dalla sezione 2**, un file per volta e A/B, con la decisione 18 addosso a ogni proposta; nessun codice toccato.

✅ **E prima di chiudere ha scritto il primo file della sezione 2, design/07** — sette fonti del degrado, la GUI viva a GPU satura,
le proiezioni a classi, «Dove» e «Chi» in «sempre visibile» — col rimando in testa ad ADR-0019 (A); `check-docs.sh` OK e
`GATE GREEN` rilanciati alla chiusura. ⛔ **Si riprende da design/03**, poi design/08, poi la sezione 3: il come sta nel prossimo
passo. La consegna è questo file, chiesta dal proprietario con `session-handoff`; nessun codice toccato.

✅ **La sesta ripresa, il 2026-09-08, ha scritto design/03** — i tre diagrammi ritoccati e il richiamo in testa — e, in questo
file, il richiamo alla tabella Passi (sei tipi di record, non quattro), la registrata sui due passi per invocazione, la
decisione 23, il punto 24 del fatto; e il puntatore della §6 del compendio. `check-docs.sh` OK e `GATE GREEN` all'apertura e
alla chiusura; fine-riga rimisurati (LF di design/03 e di questo file, CRLF del compendio). ⛔ **Si riprende da design/08**,
poi la sezione 3, un file per volta e A/B, coi tre controlli della decisione 18 detti a parole prima della domanda. La
consegna è questo file, chiesta dal proprietario con `session-handoff`; nessun codice toccato.

✅ **La settima ripresa, il 2026-09-08, ha scritto design/08** — il diagramma dei due strati con la GUI come seconda scatola
dello strato deterministico (col 2), le suite di conformità nominate con la regola che le fa crescere, le sei campagne DST le
stesse nella tabella e nella mappa, la riga Q3, le due note sotto la mappa, il richiamo sul «ciclo lungo» della DST profonda e
su ADR-0026 — approvato A coi tre controlli della decisione 18; `check-docs.sh` OK e `GATE GREEN` all'apertura. ⛔ **La sezione
2 della passata è chiusa: si riprende dalla sezione 3**, i disegni nuovi, ciascuno A/B prima di scriverlo. Due divergenze
**registrate**, del proprietario: la lettera E della §8.2 della spec (il primo worker vero: 7 lì, 12 per ADR-0039) e
l'innesco B (3) di Q6/Q11 (la proiezione nasce col 13).

⛔ **Alla chiusura della settima ripresa, chiesta dal proprietario con `session-handoff` («si fa nella prossima sessione»):
la domanda A/B sulla lettera E è stata POSTA e NON ha risposta.** Sta parola per parola nel prossimo passo ed è la prima cosa
della sessione nuova; nessun file della spec è toccato. Il materiale per la sezione 3 è già letto e sta nel prossimo passo,
coi percorsi e i comandi. Stato riletto coi comandi, `check-docs.sh` OK e `GATE GREEN` alla chiusura; la consegna è questo
file; nessun codice toccato.

✅ **L'ottava ripresa, il 2026-09-08, ha chiuso la lettera E** — delegata dal proprietario («decidi secondo la skill»), decisa
**A** coi tre controlli della decisione 18: il numero è **12** nelle **tre** righe della spec che lo portavano — la tabella delle
lettere di §8.2, la riga `process` di §8.2.2, la riga Q4 di §8.4 — col richiamo datato nel paragrafo «Sulla E»; fine-riga CRLF
della spec conservati e rimisurati; `check-docs.sh` OK. È la prima volta che questa passata tocca la **spec**, e lo fa su delega.
⛔ **Si riprende dalla sezione 3**, dal suo secondo disegno; la seconda registrata (l'innesco B di Q6/Q11) resta ferma.

✅ **E ha scritto design/10** — il modello dei dati durevoli, due `erDiagram`, A su delega (decisione 20) — con la riga di
`README.md` e «nove file» tolto dalla §12 del compendio; un commento falso di `crates/platform/src/journal.rs` è **registrato**
per il piano del 2, non corretto qui. Chiusura chiesta dal proprietario con `session-handoff`: stato riletto coi comandi,
`check-docs.sh` OK e `GATE GREEN` alla chiusura; la consegna è questo file; nessun codice toccato. ⛔ **Si riprende dal secondo
disegno della sezione 3, «la GUI dentro»**: il come, coi percorsi, sta nel prossimo passo.

⚠️ **Le approvazioni sono A CONDIZIONE**, con la stessa formula del 2: il proprietario ha risposto «A che
rispetti la skill» alla strada, e poi A o B a ogni domanda. Se scrivendo il disegno o il piano una
decisione viola un criterio di `anthropic-skills:decision-principles`, l'accettazione decade: ci si
ferma e lo si dice.

⚠️ **Due approvazioni sono sospese, e non vanno date per prese.** Il wireframe di **Lavoro**: il
proprietario ha scelto A sul modulo Passi e ha fatto una domanda sull'ambito, senza chiedere modifiche e
senza dire «approvato». Il wireframe di **Compatta e del grafo**: presentato con la domanda «va bene il
disegno?», e la sessione si è chiusa prima della risposta. La ripresa apre con queste due conferme, in
forma A/B.

✅ **RICHIAMO DEL 2026-09-07, ALLA RIPRESA: le due conferme sono DATE, entrambe A, «come mappa».** Prima di
rispondere il proprietario ha notato che nella barra della chat di Lavoro mancano il contesto della run, la
modalità di esecuzione e il tasto «+ allegati», e che «se dovessimo mettere tutte le info mancanti ce ne
sarebbero un botto»: da lì la **decisione 10** — un wireframe è una **mappa**, dove sta un modulo e chi lo
costruisce, e tutto ciò che un modulo mostra va nel **catalogo dei moduli**, sezione 1 delle sezioni che
mancano, con la fonte di ogni riga. Il paragrafo qui sopra resta com'era: è un verbale.

⚠️ **La modularità è decisa a condizione di una prova.** Il proprietario ha accettato i pannelli
agganciabili di `dockview` dicendo «deve davvero battere la 3», la tela libera. La prova è nello spike
del guscio: se provandola non dà il «Jarvis», si passa alla tela libera **prima** di scrivere la SPA.

⚠️ **Le §1 e §6a del 2 restano approvate ma portano un richiamo datato**: si riscrivono col disegno,
sulla base di questa stella polare. Il richiamo sta in testa alla consegna del 2 e dentro le due sezioni.

## Stato del repo alla chiusura, coi comandi che lo rifanno

| | Comando | Atteso |
|---|---|---|
| ramo | `git fetch --all --prune`, poi `git status -sb` | `## main...origin/main`, niente sotto |
| i commit di questa sessione | `git log --oneline 664265a..HEAD` | i commit del 2026-09-07: la consegna, il punto fermo della prima ripresa, le tabelle e le decisioni della seconda, la sua chiusura, e la terza ripresa del 2026-09-08: la §2, la §3 con la sua chiusura; la quarta ripresa dello stesso giorno: l'apertura della passata sui diagrammi, la sua chiusura; la quinta ripresa: la sezione 1 scritta, design/07 con ADR-0019, la sua chiusura; la sesta ripresa: design/03 e la sua chiusura; la settima ripresa: design/08 e la sua chiusura; l'ottava ripresa: la lettera E nella spec, design/10 e questa chiusura |
| codice e spec non toccati | `git diff --stat 664265a..HEAD -- crates/ scripts/ spikes/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml docs/superpowers/specs/2026-08-06-kernel-design.md docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` | nulla, tranne la spec del sotto-progetto 1: tre righe e un richiamo nella §8.2, l'ottava ripresa su delega (decisione 19) |
| cancello | `bash scripts/gate.sh` | `GATE GREEN` — rilanciato su `664265a` prima di scrivere i documenti della consegna, e di nuovo alla chiusura della seconda ripresa del 2026-09-07 e della quarta ripresa del 2026-09-08, e all'apertura e alla chiusura della quinta, e all'apertura e alla chiusura dell'ottava; `check-docs.sh` a ogni commit. Si rilancia, non si cita |
| documenti | `bash scripts/check-docs.sh` | `OK` |
| fine-riga | `git ls-files --eol docs/COMPENDIO.md docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md docs/superpowers/specs/2026-09-07-direzione-gui-design.md docs/superpowers/specs/2026-09-07-direzione-gui-wireframes/*.svg docs/design/10-modello-dei-dati-durevoli.md` | il compendio `i/lf w/crlf`, gli altri `i/lf w/lf` — su questa macchina: un clone nuovo con `core.autocrlf=true` li mostra `w/crlf`, e non è una divergenza |
| i wireframe esistono | `ls docs/superpowers/specs/2026-09-07-direzione-gui-wireframes/` | `compatta-e-grafo.svg  home.svg  lavoro.svg` |
| margine del compendio | `wc -c docs/COMPENDIO.md` contro `grep -n '^ceiling=' scripts/check-docs.sh` | positivo |

La baseline dei test la dà `cargo test --workspace --no-fail-fast --locked`, non questa riga.

## Fatto in questa sessione

1. Ripresa sulla consegna del 2 con `anthropic-skills:session-resume`: stato riletto coi comandi,
   nessuna divergenza; il cancello non rilanciato all'apertura perché non si toccava codice, rilanciato
   alla chiusura.
2. Prima della progettazione, la domanda del proprietario su tre funzionalità — azioni a un'ora fissa,
   sub-agenti e workflow, il PC «come Cowork» — risposta leggendo i documenti e il codice. L'esito è
   nella tabella *«Ciò che il repo diceva già»* qui sotto.
3. Il percorso classificato **architetturale**: la forma di tutta la GUI, non le due schermate del 2.
4. Le **cinque domande** poste una alla volta in forma A/B col consiglio, tutte risposte; poi le tre
   strade per la modularità, con lo stato dell'arte verificato alle fonti prima di porle.
5. Quattro wireframe disegnati in chat con lo strumento inline (decisione 8 della consegna del 2) e
   **salvati** come SVG autonomi nella cartella accanto a questo file.
6. Due divergenze fra documenti approvati trovate leggendo, e portate al proprietario: i pannelli mobili
   assegnati al 2 dal disegno dei gesti e non nominati dalle sezioni del 2; il replay dei trace
   assegnato alla GUI minima da tracciabilità e non costruito dal 2. Entrambe chiuse a favore del 2.
7. Questo file scritto; i richiami datati nella consegna del 2; il puntatore della §6 mosso.
8. **Alla ripresa, in una sessione nuova lo stesso giorno:** stato riletto coi comandi, nessuna divergenza,
   `GATE GREEN` e `check-docs.sh` OK all'apertura; le due conferme sospese date, entrambe A «come mappa»;
   la decisione 10 sul catalogo dei moduli; questo punto fermo, committato prima di scrivere la sezione 1.
9. **Alla seconda ripresa, lo stesso giorno:** stato riletto coi comandi, nessuna divergenza, `check-docs.sh` OK
   all'apertura, cancello non rilanciato perché non si tocca codice; la domanda del proprietario sul metodo —
   quando e dove si scrivono le informazioni che i wireframe non portano — risposta con la decisione 10; la
   decisione 11 sul modello, indicatore sì, selettore registrato.
10. La **tabella Chat**, prima del catalogo dei moduli: le fonti lette per intero — la §5 del compendio, le
    ventuno righe G di `spikes/GUI-REQUISITI.md`, le righe di tracciabilità sulla conversazione, la §4 e la
    §6a della consegna del 2 — presentata al proprietario con verificato e dedotto separati, e **approvata, A**;
    scritta nella §1 qui sotto, con la riga 24 aggiunta dopo l'approvazione e dichiarata; la riga su allegati
    e «aggiungi al contesto» nelle registrate.
11. La **tabella Stato**, sette righe, approvata (**A**) e scritta, coi due campi di oggi del degrado letti nel
    codice (`crates/kernel/src/degradation.rs`); le registrate sul grafico dell'occupazione GPU e su chi
    costruisce l'esportazione OTLP; la seconda passata su ADR e tracciabilità, chiesta dal proprietario, ha
    trovato altre righe candidate, che si scrivono solo dopo l'approvazione.
12. Le cinque righe candidate dello Stato (8–12) approvate (**A**) e scritte; la riga 18 della Chat guadagna la
    fonte «Run persistenti, ripresa e cancellazione», letta nella spec del kernel come annullamento (SP-4) e non
    come cancellazione dallo storico.
13. La **tabella Permessi**, dieci righe, con le due passate fatte prima della presentazione, approvata (**A**) e
    scritta; la correzione del «chi» della riga 12 della Chat — i preset sono del 4, riga «Modalità di permessi a
    più livelli» — approvata insieme, con richiamo datato.
14. La **tabella Passi**, quattordici righe, con le due passate fatte prima della presentazione e i tipi di record e
    le specie di dettaglio letti in `crates/kernel/src/record.rs`, approvata (**A**) e scritta.
15. La **tabella Attività**, sedici righe, con le due passate fatte prima della presentazione, approvata (**A**) e
    scritta.
16. Alla domanda «non stiamo complicando le cose?» la risposta onesta — sì, sui moduli lontani — e la decisione 12;
    la decisione 13 sull'Ambito, letta in ADR-0016 (una tripla di lettura vale «lì, e solo lì») e nel follow-up di
    ADR-0024; la tabella corta dei tredici moduli scritta; la **§1 chiusa**.
17. Chiusura della seconda ripresa: stato riletto coi comandi, `GATE GREEN` rilanciato, il puntatore della §6 del
    compendio riscritto — le conferme date, la §1 chiusa, si riparte dalla sezione 2 — i vicoli ciechi di questa
    ripresa scritti qui sotto, la memoria dell'agente aggiornata; nessun codice toccato.
18. **Alla terza ripresa, il 2026-09-08:** stato riletto coi comandi, nessuna divergenza, `check-docs.sh` OK; la
    **sezione 2** presentata con le fonti lette nel codice — `platform`, `daemon`, le sei porte — e nel leggerle la
    proposta scritta («consegnato al daemon e non letto dal kernel») è **caduta**: `SaveLayout` arriva dentro il kernel
    (§5 del 2) e nessuna delle sei porte tiene un pacchetto opaco. Le due domande delegate dal proprietario («decidi
    secondo la skill») e decise: fuori dal registro (14), una settima porta (15); la §2 scritta qui sotto.
19. La **sezione 3** presentata — i nove pezzi in ordine, le sezioni del 2 che cambiano, il «non costruisce», le decisioni
    16–19 del coordinatore. Alla domanda del proprietario *«tutto questo è integrato e studiato nell'architettura, schemi ER
    ecc.?»* la risposta **verificata nei file** e non a memoria — la tabella *«Cosa è già studiato, e dove»* nella §3 — con
    le due divergenze trovate in design/09. **Approvata, A**, con le due registrate.
20. Chiusura della terza ripresa: il **mandato** del proprietario per la sessione nuova (decisione 16) scritto nel prossimo
    passo; il puntatore della §6 del compendio riscritto; `GATE GREEN` rilanciato; la memoria dell'agente aggiornata; nessun
    codice toccato.
21. **Alla quarta ripresa, il 2026-09-08 — la passata sui diagrammi, aperta e non chiusa:** stato riletto coi comandi, nessuna
    divergenza; i nove file di `docs/design/` letti diagramma per diagramma (`grep -c mermaid docs/design/*.md` li conta, una riga
    per blocco), col disegno della knowledge base e la consegna del 2, e per ciascuno misurato che cosa le decisioni di questa
    stella polare cambiano — la tabella *«I diagrammi, uno per uno»* nel prossimo passo. La registrata della terza ripresa sulla
    **policy VRAM** portata al proprietario come domanda 1 e **delegata**: decisa **A**, la proiezione del giornale (decisione 17).
    La **sezione 1** della passata — design/09 e le sue conseguenze in ADR-0022, ADR-0006, design/02 e design/05 — **presentata** a
    parole, con la tabella dei cambiamenti e il diagramma nuovo reso in chat dal suo sorgente mermaid; la sessione si è chiusa
    **sulla domanda A/B, senza risposta**: niente è scritto nei file di design.
22. **Alla quinta ripresa, il 2026-09-08:** stato riletto coi comandi, nessuna divergenza, `check-docs.sh` OK e `GATE GREEN`
    all'apertura; la sezione 1 ripresentata a parole e col diagramma reso dal sorgente identico. Alla domanda del proprietario
    *«stai tenendo conto di scalabilità, di quanto verrà e di quanto già esiste? è lo schema a modificare logiche, ADR e codice se
    è più corretto»* la risposta verificata nel codice — `set_policy` scrive intento ed esito, `is_granted` e `degradation_now`
    rileggono il giornale, `build_the_arbiter` riparte da `Remote` — e nei disegni: sette righe su nove reggevano, **due no** — il
    nodo «giornale» senza «guide approvate (col 13)», decise dal rimando del 2026-09-05 in ADR-0009 con lo stesso criterio che
    aveva messo «(col 6)» negli altri nodi; e la cella «Chi lo raggiunge» della configurazione con la sola settima porta, dove i
    profili li legge il daemon e li consegna (ADR-0034). **Approvata A con le due correzioni**, e la **decisione 18**; scritta —
    design/09, ADR-0022, ADR-0006, design/02, design/05, `CLAUDE.md`, questo file, il puntatore della §6 — coi fine-riga di
    ciascun file rimisurati; `check-docs.sh` OK, commit e push.
23. **Alla quinta ripresa, prima di chiudere:** la sezione 2 aperta con design/07 — letti i tre file della sezione, le righe
    G9–G14, `degradation.rs` (i due campi, e il doc che dichiara le fonti che mancano), `record.rs` (`EffectClass` senza variante
    «non dichiarata», `RecordV1` senza passo padre), le suite di conformità esistenti, ADR-0019 e ADR-0033; presentato coi tre
    controlli della decisione 18 e i due diagrammi resi dal sorgente identico; **approvato A** e scritto, col rimando in testa ad
    ADR-0019 — lo schema ha corretto l'ADR: la lista degli eventi non è chiusa, la GUI resta viva a GPU satura. Poi la consegna,
    chiesta dal proprietario; `GATE GREEN` e `check-docs.sh` OK alla chiusura.
24. **Alla sesta ripresa, il 2026-09-08:** ripresa con `session-resume` — stato riletto coi comandi, nessuna divergenza,
    `check-docs.sh` OK e `GATE GREEN` all'apertura; design/03 letto contro `record.rs`, `reconcile.rs`, `ports/journal.rs`,
    le due implementazioni del giornale, `Arbiter::set_policy`, ADR-0007, ADR-0008, design/09 e la §5 del 2, e trovato che
    la consegna diceva «quattro tipi di record» dove sono **sei**; presentato coi tre controlli della decisione 18 e i tre
    diagrammi resi dal sorgente identico; **approvato A e A** — design/03, e il richiamo alla tabella Passi — sotto la
    condizione del proprietario, e scritto coi fine-riga conservati; la registrata sui due passi per invocazione, con la
    decisione 23 del coordinatore; poi la consegna, chiesta dal proprietario con `session-handoff`; `check-docs.sh` OK e
    `GATE GREEN` alla chiusura; nessun codice toccato.
25. **Alla settima ripresa, il 2026-09-08:** ripresa con `session-resume` — stato riletto coi comandi, nessuna divergenza,
    `check-docs.sh` OK e `GATE GREEN` all'apertura; design/08 letto contro `crates/kernel/src/lib.rs`, le suite
    `journal_contract` e `reactor_contract` coi gemelli `_real` di `platform`, `dying_gui.rs`, le cinque campagne del passo
    «DST campaigns» di `gate.sh`, la §8.2 e la §8.4 della spec, la §11 del compendio; presentato coi tre controlli della
    decisione 18 e il diagramma reso dal sorgente identico; **approvato A** e scritto coi fine-riga conservati (CRLF); due
    divergenze registrate, del proprietario.
26. Chiusura della settima ripresa, chiesta dal proprietario con `session-handoff`: la domanda sulla lettera E della §8.2
    posta in forma A/B e lasciata **senza risposta** per la sessione nuova; il materiale della sezione 3 letto — `record.rs`
    (i sei campi di `RecordV1` coi loro indici, i sei `RecordKind`, le tre specie di `Detail`), la tabella `redb` del
    giornale in `platform`, i sei record congelati, ADR-0024, la §2.2 e la §4.2 del disegno della knowledge base, design/09
    e design/01 — e scritto nel prossimo passo coi percorsi; stato riletto coi comandi, `check-docs.sh` OK e `GATE GREEN`
    alla chiusura; il puntatore della §6 del compendio; la memoria dell'agente aggiornata; nessun codice toccato.
27. **All'ottava ripresa, il 2026-09-08:** ripresa con `session-resume` — stato riletto coi comandi, nessuna divergenza,
    `check-docs.sh` OK e `GATE GREEN` all'apertura; la domanda sulla lettera E riverificata contro la spec, ADR-0039 e la
    roadmap, e trovato che il 7 viveva in **tre** righe della spec e non in una; nessun worker vero nel codice
    (`grep -rln 'impl Worker for' crates/*/src` rende solo la prosa della porta). Il proprietario ha **delegato** («decidi
    secondo la skill»): decisa **A** e scritta — le tre righe a 12, il richiamo datato in «Sulla E» — coi fine-riga CRLF
    conservati; la decisione 19 e la 25, la registrata chiusa, il puntatore della §6; `check-docs.sh` OK, commit e push.
28. **All'ottava ripresa, la sezione 3 comincia:** il materiale riverificato coi comandi — `record.rs`, la tabella `redb`, i sei
    byte congelati, gli scrittori di produzione, `prune`, la porta `filesystem`, ADR-0024, il disegno della knowledge base; il
    primo disegno nuovo presentato coi tre controlli della decisione 18 e i due `erDiagram` resi in chat dal sorgente identico;
    il proprietario ha **delegato** («decidi secondo la skill»): decisa **A** (decisione 20) — design/10 scritto, la riga di
    `README.md`, «nove file» tolto dal compendio, le righe di questo file; un commento falso in `platform/src/journal.rs`
    registrato per il piano; `check-docs.sh` OK, commit e push.
29. Chiusura dell'ottava ripresa, chiesta dal proprietario con `session-handoff`: stato riletto coi comandi, `check-docs.sh` OK
    e `GATE GREEN` alla chiusura; in questo file lo stato in una riga, «Da sapere subito», la tabella dello stato, i vicoli
    ciechi, il prossimo passo; il puntatore della §6 del compendio e l'intestazione; la memoria dell'agente aggiornata; gli
    script di modifica dello scratchpad cancellati; nessun codice toccato.

## Le decisioni del proprietario, una per domanda

| # | Domanda | Risposta |
|---|---|---|
| 0 | il perimetro: stella polare di tutta la GUI, o allargare il 2? | **A, «che rispetti la skill»** — il wireframe è la stella polare, in un documento suo; ogni vista porta il numero del sotto-progetto che la costruisce; il 2 costruisce la **cornice** (le viste, i moduli mobili, la Home con l'anello che a parole dice «niente ancora») più la sua fetta di oggi. Scartata B, allargare il 2 fino ad anello, rete e grafo veri: disegna cose che nessuno produce fino al 3, 6 e 7, ed era già caduta come domanda 12 del [disegno della knowledge base](2026-09-04-knowledge-base-design.md) |
| 1 | cosa conta come «artefatto»: le chat e le run stanno nell'anello? | **A — no.** Nell'anello solo **file** prodotti dalle run — documenti, codice, asset 3D, catture, esportazioni — per data (ADR-0008 e ADR-0018: l'artefatto è un file sul disco, riferito dal giornale). Le chat e le run hanno una tessera loro, Attività. La rete al centro ha **tutto**: artefatti e file della knowledge base messi dal proprietario |
| 2 | cosa succede «a fuoco» | **A** — la rete al centro è un modulo come gli altri: «mettere a fuoco» è aprirlo a pagina intera con lo stesso gesto delle tessere, e a pagina intera è il grafo della knowledge base già deciso (§4.3 del disegno della knowledge base). Scartata B, uno zoom speciale della Home: un secondo modo di aprire, fatto per il solo centro |
| 3 | cosa sono le tessere | **A** — ogni tessera è un **modulo in piccolo, vivo**, che si usa sul posto e si apre a pagina intera; quali esistono lo dice la lista G9–G18 di [`spikes/GUI-REQUISITI.md`](../../../spikes/GUI-REQUISITI.md) più i quattro pilastri; ogni tessera porta il numero del sotto-progetto che la riempie; una tessera senza dati lo dice a parole. Scartata B, un pulsante per pilastro: la Home diventa un menu |
| 4 | dove vive la disposizione dei moduli | **A** — nel core, nell'archivio di configurazione di ADR-0022, e **il 2 ne costruisce il pezzo minimo**: un archivio con una voce sola, la disposizione; il core la manda alla GUI al collegamento, la GUI manda «salva disposizione». È la decisione 12 del [disegno dei gesti](2026-09-03-riconoscimento-gesti-design.md), registrata e ora presa. Salvarla nel browser della GUI è escluso da I1. Scartata B, rimandare l'archivio: si riordina e si perde tutto al riavvio |
| 5 | una modalità piccola sempre in primo piano | **A** — entra come terza vista, **Compatta**: si disegna ora, la costruisce il **10**, perché «sempre in primo piano» e la scorciatoia globale sono integrazione con l'OS (riga «Overlay/finestra fluttuante → GUI + L3» di tracciabilità). Zero codice nel 2 |
| 6 | la modularità: griglia, pannelli agganciabili, tela libera | **2, pannelli agganciabili**, a condizione che «batta davvero la 3»: `dockview-core` 8.2.0, usato **diretto** e non con l'adattatore Vue (ADR-0030 preferisce le librerie agnostiche); dipendenza nuova, in due passi. La prova è nello spike del guscio, sezione *«Lo spike di accettazione»*. Scartate: la griglia a tessere, che non sa fare schede e pannelli divisi per Lavoro; la tela libera, che sovrappone, salva in pixel e fa faticare la tastiera — resta la strada di riserva se la prova fallisce |
| 7 | la chat nella Home di default | **B** — no. La chat vive in Lavoro; in Home si aggiunge dal cassetto «+ moduli». «Agentic OS, non chatbot» |
| 8 | il modulo Passi nel 2 | **A** — nasce col 2, minimo: mostra i passi che il giornale ha oggi, cioè le invocazioni del registro con intento ed esito; col 3 cresce ai passi delle run. Chiude la divergenza con le due righe di tracciabilità «Replay dei trace» e «Osservabilità e tracing locale → GUI minima». Costo: un messaggio IPC con la lista dei passi, e un pannello |
| 9 | il PC «come Cowork» | «lavorare sui tuoi file e lanciare comandi va bene, per ora»: deciso nei meccanismi (ADR-0016, ADR-0024, ADR-0025) e visibile col 5. Guidare schermo, mouse e tastiera resta **solo un nome** — righe «Automazione OS → L3» e «Screenshot e comprensione dello schermo → L3 + Conversazione» — senza definizione: registrata, non presa |
| 10 | **alla ripresa, 2026-09-07** — quanto dettaglio porta un wireframe: il proprietario ha notato che nella barra della chat di Lavoro mancano il contesto della run, la modalità di esecuzione e il tasto «+ allegati», e che «se dovessimo mettere tutte le info mancanti ce ne sarebbero un botto» | **A** — un wireframe è una **mappa**: dove sta ogni modulo e chi lo costruisce; Lavoro e Compatta col grafo sono **approvati come mappa**. Tutto ciò che un modulo mostra, e i comandi che ha, va nel **catalogo dei moduli** (sezione 1 delle sezioni che mancano): una tabella per modulo con la **fonte** di ogni riga — G, ADR, riga di tracciabilità — e il sotto-progetto che la costruisce. Le tre cose notate hanno fonte (G11 · ADR-0016 · «Allegati in chat» di tracciabilità) ed entrano nella riga Chat, nella **barra della chat, per run**; la striscia sempre visibile resta il riassunto. Scartata B, ridisegnare i wireframe con tutto dentro: disegna oggi cose che arrivano col 3, 5, 6 e 13 e si rifanno col design system — la stessa ragione della B della domanda 0 |
| 11 | **alla seconda ripresa dello stesso giorno** — il proprietario ha aggiunto il **modello selezionato** alle cose che mancano nella barra della chat, e ha chiesto quando e dove si scrivono le informazioni che i wireframe non portano, per non crescere feature su feature | **A** — il metodo è la decisione 10, e il momento è la sezione 1, prima del disegno e del piano. Il modello entra nella riga Chat come **indicatore** del modello in uso, con fonte: il record di routing di ADR-0011 e la riga «Indicatore di stato modello» di tracciabilità, casa GUI. Un **selettore** a mano per chat non ha fonte, e tracciabilità ha già «Selettore di modello per compito ✅ §3 · profili»: la domanda «posso anche sceglierlo a mano, per run?» è registrata, chiusore il **3** col disegno del modulo Chat. Scartata B, decidere il selettore adesso: una riga G nuova e la riapertura di come si sceglie il routing, decisione strutturale che non si prende a fine giornata |
| 12 | **alla seconda ripresa** — la profondità del catalogo: «non stiamo complicando le cose?» | **A** — sì, sui moduli lontani. Le tabelle piene restano per i moduli del 2 e del 3 già scritti (Chat, Stato, Permessi, Passi, Attività); gli altri **tredici** entrano in **una tabella corta**: chi li costruisce, ciò che ADR e requisiti impongono di mostrare, le fonti — niente comandi, regole o dedotti, che scrive il sotto-progetto col suo disegno (sezione 6). Riletti i doppioni: le righe che tornano in più tabelle sono **facce diverse** dello stesso fatto, una per modulo, e restano; i doppioni veri stavano nella proposta di Ambito, non scritta. Il paragrafo dei criteri resta dov'è già scritto e si fa una volta per la tabella corta. Scartata B, tredici tabelle piene: circa seicento righe che i sotto-progetti riscriverebbero — la ragione della B della domanda 10 |
| 13 | **alla seconda ripresa** — l'Ambito: «che cos'è? vorrei che funzionasse simil Claude Desktop; ha interazione diretta con i permessi?» | l'ambito è la **cartella di lavoro della run** più la copia di sicurezza di ogni file prima che l'agente lo tocchi (ADR-0024): non un albero del disco, non il diff. **Simil Claude Desktop, deciso**: la cartella si sceglie all'avvio della run e si vede sempre nell'intestazione della chat; se ne aggiungono altre durante la run, e ogni aggiunta è una dichiarazione di ambito; pannello o riga lo decide il 5. Coi permessi, letto negli ADR: sono **due meccanismi distinti** — la tripla dice cosa l'agente **può fare** su un percorso, e una tripla di lettura vale «lì, e solo lì» (ADR-0016, tabella delle triple); l'ambito dice **dove c'è la copia di sicurezza** (ADR-0024, che non nomina i permessi). Si incontrano in un punto: la richiesta di permesso su un file, dove la GUI dice se il percorso è dentro l'ambito o fuori, **prima** che l'agente scriva (G18, follow-up di ADR-0024). Registrata: se il preset «auto-approva sicuri» approvi le letture ovunque o solo dentro l'ambito |
| 14 | **alla terza ripresa, 2026-09-08** — se «salva disposizione» passi dal registro delle funzioni con una tripla, o sia una scrittura fuori dal registro | **delegata al coordinatore, «decidi secondo la skill»: fuori dal registro.** ADR-0038 regola 5 tiene fuori lo spostare un pannello; salvarlo è lo stesso fatto reso durevole, e una tripla chiederebbe il permesso di sistemare le proprie finestre a ogni avvio (ADR-0016: un'approvazione vale quella tripla, quella sessione). §2 |
| 15 | **alla terza ripresa** — dove la disposizione si conserva, letto che `SaveLayout` arriva dentro il kernel (§5 del 2) e che nessuna delle sei porte tiene un pacchetto opaco: una **settima porta**, o il giornale forzato | **delegata al coordinatore: la settima porta.** Scartato il giornale — ADR-0018 tiene per sempre «la parte piccola», e un pacchetto di pannelli decine di volte al giorno non lo è — e, già alla domanda 4, il non salvare. Costo: la §3.1 della spec passa da sei a sette famiglie con richiamo datato, compito del piano. §2 |
| 16 | **alla terza ripresa** — la sezione 3 (**A**, con le due registrate), e la domanda *«tutto questo è integrato e studiato nell'architettura, schemi ER ecc.?»* | **Il mandato per la sessione nuova, con un agente nuovo:** aggiornare i diagrammi esistenti, **crearli dove mancano** — ER compresi — e **correggere la progettazione** dove un diagramma mostra un errore. Viene **prima** delle sezioni 4–6; una correzione a una sezione approvata torna al proprietario in forma A/B, con richiamo datato. La risposta verificata sta nella §3, tabella *«Cosa è già studiato, e dove»* |
| 17 | **alla quarta ripresa, 2026-09-08** — dove vive la policy VRAM: la registrata della terza ripresa, portata come domanda 1 della passata sui diagrammi. A: la corrente è la **proiezione del giornale** e il profilo dà il default; B: l'archivio di configurazione di ADR-0022, come design/09 diceva | **delegata al coordinatore, «decidi secondo la skill»: A.** Letto nel codice: `Arbiter::set_policy` in `crates/kernel/src/arbiter/mod.rs` scrive già ogni transizione come intento ed esito, con la policy per nome nel `reason`; `is_granted` e `degradation_now` rileggono già il giornale così; e design/09 dice da solo che solo il giornale è autorevole — un secondo archivio per un fatto che il giornale ha già sarebbe una casa doppia. Il profilo dà il **default** (Remote, ADR-0006); il daemon rilegge l'ultima transizione all'avvio, **compito del piano del 2** (`build_the_arbiter` riparte da `Remote`). Costo: un rimando in testa ad ADR-0006 — «determinato dal profilo» diventa «il profilo dà il default, il giornale il corrente» — l'etichetta della freccia e una riga in design/02, una riga in design/05. 🔶 **Dedotto, lo decide il disegno del 2:** la transizione guadagna un **dettaglio tipizzato**, sul precedente di `PermissionDetail`, perché la rilettura non cerchi una stringa; oggi non porta dettaglio. Scartata B: il default nel profilo e il corrente altrove, due case per un fatto |
| 18 | **alla quinta ripresa, 2026-09-08** — la sezione 1 della passata (design/09 e le sue conseguenze), e la domanda *«stai tenendo conto di scalabilità, di quanto verrà e di quanto già esiste? è lo schema a modificare le logiche, gli ADR e il codice esistenti se è più corretto di essi»* | **A, con le due correzioni** trovate rispondendo: «guide approvate (col 13)» nel nodo «giornale», e le **due vie** nella cella «Chi lo raggiunge» della configurazione — i profili consegnati dal daemon (ADR-0034), la disposizione dalla settima porta. ⛔ **E il metodo vale per tutti gli studi, i brainstorming e i diagrammi futuri:** ogni proposta controlla esplicitamente che cosa **esiste già** (codice, ADR, disegni), che cosa **arriva** (la roadmap) e se **regge crescendo**, e lo dice a parole prima dell'A/B; se lo schema è più corretto di una logica, di un ADR o del codice, si correggono **quelli** — l'ADR col richiamo datato, il codice come compito del piano, sempre in forma A/B, mai in silenzio. Scritto in `CLAUDE.md`, «Come si lavora qui» |
| 19 | **all'ottava ripresa, 2026-09-08** — la lettera E della §8.2 della spec: il primo worker vero è del 7 (la spec, 2026-08-08) o del 12 (ADR-0039 e la roadmap, 2026-09-03)? A: la spec dice 12, col richiamo datato; B: la spec resta com'è e la voce resta registrata | **delegata al coordinatore, «decidi secondo la skill»: A.** I tre controlli: **esiste** — nessun worker vero nel codice (`grep -rln 'impl Worker for' crates/*/src` rende solo la prosa della porta), la spec porta il 7 in **tre** righe (la tabella delle lettere di §8.2, la riga `process` di §8.2.2, la riga Q4 di §8.4) e la §8.2.1 dice da sola che *«se la roadmap cambia, la condizione resta vera e il numero si aggiorna»*; **arriva** — ADR-0039 e la roadmap dicono 12 dal 2026-09-03, e la Voce riusa; **regge crescendo** — se il 7 o il 6 arrivassero prima, la condizione resta e il numero si aggiorna di nuovo per la stessa regola, e il richiamo lo dice. Costo: tre numeri e un richiamo datato in «Sulla E»; correggerne una riga sola avrebbe lasciato due a mentire. Scartata B: due documenti approvati che si contraddicono senza nominarsi (gotcha #59), e una registrata che nessuno chiude |
| 20 | **all'ottava ripresa, 2026-09-08** — la sezione 3, primo disegno: il modello dei dati durevoli in un file nuovo, `docs/design/10-modello-dei-dati-durevoli.md`, con due `erDiagram` (A), o dentro design/03 e design/09 (B) | **delegata al coordinatore, «decidi secondo la skill»: A.** I tre controlli, detti prima della domanda: **esiste** — sei campi con indice, sei `RecordKind`, tre `Detail`, la tabella `redb` del giornale, cinque scrittori di produzione, `prune` che cancella, la porta `filesystem` con `CheckpointId` e senza implementazione vera; nessun `erDiagram` in `docs/`; **arriva** — col 2 l'invocazione e la disposizione, col 3 run e sub-run, col 5 ambito e checkpoint, col 6 la cartella e l'indice della knowledge base, col 13 le guide approvate (forma dedotta); **regge crescendo** — una specie nuova è un indice nuovo di `Detail` (ADR-0036, regola 3), un archivio nuovo è una porta o una tabella sua, e un'entità costruita passa dal secondo diagramma al primo con richiamo datato, regola scritta nel file. Presentato a parole e col diagramma reso in chat dal sorgente identico (decisione 21); scritto design/10, la riga di `README.md` (il cancello conta file e righe), «nove file» tolto dalla §12 del compendio. Un difetto trovato disegnando e **registrato**: un commento di `crates/platform/src/journal.rs` dice che `boundary.rs` scrive byte che non sono un `Record`, ed è falso — codice, compito del piano (decisione 18). Scartata B: un modello dei dati spezzato in tre case diverge in silenzio (gotcha #68) |

## Ciò che il repo diceva già, letto per decidere

| Funzionalità chiesta dal proprietario | Decisa? | Dove | Codice oggi | Arriva con |
|---|---|---|---|---|
| azioni automatiche a un'ora fissa | sì, come **trigger**: pianificazione, cambiamento di file, fine di un'altra run | ADR-0009; riga «Scheduling ✅» di [tracciabilità](../../tracciabilita.md) | solo l'orologio del reattore (`now`, `wall_time`, `wait_until`); nessun tipo `Trigger` (`grep -rl --include='*.rs' Trigger crates/` rende zero) | il meccanismo col **13**; la funzione per l'utente col **4** |
| sub-agenti e workflow | sì: un sub-agente è una **sub-run**, «proiezione ristretta della stessa struttura, col proprio segmento di giornale» ([design/03](../../design/03-run-durevoli.md)); la gerarchia run → sub-run → passo è ADR-0011; il piano nello stato durevole è ADR-0008 | righe «Orchestrazione e sub-agenti», «Planning», «Agenti in parallelo isolati» di tracciabilità | run e passo non esistono come tipi: c'è solo `StepId` (`grep -rnE 'RunId' crates/*/src` rende zero) | run e sub-run col **3**, piano e coda col **4**, isolamento su disco col **5** |
| il PC come Cowork: file e comandi | sì: permessi a tripla, checkpoint sugli ambiti dichiarati, nessun comando sotto il livello 2 | ADR-0016, ADR-0024, ADR-0025 | le porte `filesystem` e `process` nel kernel; nessuna capacità sopra | il **5** |
| il PC come Cowork: schermo, mouse, tastiera | **no**: solo il nome «Automazione OS» | tracciabilità, sezioni 7 e 8; nessun documento nomina mouse o tastiera (`grep -rnE 'mouse' docs --include='*.md'` fuori da questo file rende zero) | nulla | il **10**, senza disegno |

E le tre cose del repo che hanno **deciso la forma**, verificate coi comandi:

| Cosa | Dove | Che cosa ha deciso qui |
|---|---|---|
| «il sotto-progetto 2 costruisce pannelli e menu che si muovono con **qualunque puntatore**; la mano è un puntatore in più» | §4.1 del [disegno dei gesti](2026-09-03-riconoscimento-gesti-design.md), e la riga 12 di «Perché quest'ordine» in [roadmap](../../roadmap.md): `grep -n 'pannelli mobili' docs/roadmap.md` | i moduli mobili sono **già del 2**; le sezioni approvate del 2 non li nominavano — divergenza chiusa dal perimetro A |
| il pannello della knowledge base: «disegna il grafo raggruppato per router; filtra per specie, cartella, etichetta; cerca sui nomi; al click mostra il nodo e le funzioni del registro; orfani e collegamenti rotti; non tocca file, non tiene stato» | §4.3 del [disegno della knowledge base](2026-09-04-knowledge-base-design.md), decisioni 6, 9, 10, 17 | il nucleo a pagina intera **è** quel pannello, nato col 6; qui non si ridisegna, si colloca |
| «se la posizione dei pannelli sopravvive a un riavvio: configurazione, archivio di ADR-0022, che non esiste: la chiude chi lo costruisce» | decisione 12 del disegno dei gesti: `grep -n 'posizione dei pannelli' docs/superpowers/specs/2026-09-03-riconoscimento-gesti-design.md` | presa oggi: il 2 costruisce l'archivio minimo (domanda 4) |
| «L'interfaccia deve mostrare **cosa è coperto** dall'ambito attivo, prima che l'agente inizi a scrivere»; «un limite di dimensione oltre il quale un file viene escluso dal checkpoint **con avviso**» | follow-up di [ADR-0024](../../adr/0024-checkpoint-del-filesystem-ad-ambiti-dichiarati.md) | il modulo Ambito di Lavoro, con «fuori ambito: non coperto» e «file grande: escluso, con avviso» |
| «la leva non è la GUI ma la **frequenza di aggiornamento** decisa dal core»; «la webview esegue solo contenuto locale nostro»; G13 dal primo giorno | follow-up di [ADR-0027](../../adr/0027-stack-della-gui.md) | la difesa contro tante chat che scrivono insieme; la provenienza in ogni pezzo di flusso |
| «Replay dei trace 🔶 → GUI minima», «Osservabilità e tracing locale 🔶 → GUI minima» | tracciabilità, sezioni 4 e 8: `grep -n 'GUI minima' docs/tracciabilita.md` | il modulo Passi nasce col 2 (domanda 8) |

## Lo stato dell'arte verificato, e il comando

Fonte primaria: il registro npm interrogato il 2026-09-07 col comando sotto; la documentazione ufficiale
di `dockview` letta lo stesso giorno alle pagine nominate. ⚠️ **Casa unica provvisoria**: quando il
disegno sarà scritto queste righe passano in [`riferimenti.md`](../../riferimenti.md), e qui resta il
rimando — è la regola di `CLAUDE.md` alla chiusura di una voce con una fonte.

```
python - <<'EOF'
import json, urllib.request, urllib.parse
def npm(p):
    d = json.load(urllib.request.urlopen("https://registry.npmjs.org/" + urllib.parse.quote(p, safe="@")))
    v = d["dist-tags"]["latest"]; vv = d["versions"][v]
    return v, d["time"][v][:10], vv.get("license"), (vv.get("peerDependencies") or {}).get("vue", "-")
def dl(p):
    return json.load(urllib.request.urlopen("https://api.npmjs.org/downloads/point/last-week/" + urllib.parse.quote(p, safe="@")))["downloads"]
for p in ["dockview", "dockview-core", "dockview-vue", "splitpanes", "gridstack", "grid-layout-plus", "interactjs", "sigma", "graphology", "d3-force", "cytoscape", "@vue-flow/core", "force-graph", "three", "pixi.js", "markdown-it", "marked", "shiki", "codemirror", "dompurify"]:
    print(p, *npm(p), dl(p))
EOF
```

| Pacchetto | Versione | Pubblicata | Licenza | Vue richiesto | Download/settimana | Per che cosa |
|---|---|---|---|---|---|---|
| `dockview-core` | 8.2.0 | 2026-08-19 | MIT | — | 303 488 | **scelto**: il motore dei moduli, usato diretto |
| `dockview` | 8.2.0 | 2026-08-19 | MIT | — | 215 513 | lo stesso, col pacchetto ombrello |
| `dockview-vue` | 8.2.0 | 2026-08-19 | MIT | ^3.4.0 | 6 105 | l'adattatore Vue: **non scelto**, usato da pochi |
| `splitpanes` | 4.1.2 | 2026-05-26 | MIT | ^3.2.0 | 144 877 | solo pannelli divisi: non basta |
| `gridstack` | 13.2.0 | 2026-08-20 | MIT | — | 499 310 | la griglia a tessere, strada 1, scartata |
| `grid-layout-plus` | 1.1.1 | 2025-10-13 | MIT | ^3.0.0 | 77 895 | idem, versione Vue |
| `interactjs` | 1.10.28 | 2026-08-01 | MIT | — | 591 557 | la tela libera, strada 3, **di riserva** |
| `sigma` | 3.0.3 | 2026-04-30 | MIT | — | 240 975 | grafo su WebGL: candidato per il 6, non scelto qui |
| `graphology` | 0.26.0 | 2025-01-26 | MIT | — | 1 405 123 | la struttura del grafo sotto `sigma` |
| `d3-force` | 3.0.0 | 2021-06-05 | ISC | — | 16 164 124 | la fisica della rete viva: candidato, non scelto qui |
| `cytoscape` | 3.34.2 | 2026-08-25 | MIT | — | 14 670 550 | grafo: candidato per il 6 |
| `@vue-flow/core` | 1.48.2 | 2026-01-28 | MIT | ^3.3.0 | 476 176 | grafo a nodi Vue: candidato per il 6 |
| `force-graph` | 1.51.4 | 2026-04-16 | MIT | — | 595 590 | grafo a forze su canvas: candidato |
| `three` | 0.185.1 | 2026-07-01 | MIT | — | 14 025 392 | il viewer 3D, ADR-0030 |
| `pixi.js` | 8.20.1 | 2026-08-26 | MIT | — | 920 303 | la rete viva su WebGL: candidato, non scelto qui |
| `markdown-it` | 15.0.1 | 2026-08-27 | MIT | — | 27 048 598 | il renderer di markdown: decisione aperta della §9 del 2 |
| `marked` | 18.0.11 | 2026-08-24 | MIT | — | 66 978 414 | idem |
| `shiki` | 4.4.3 | 2026-08-10 | MIT | — | 21 210 699 | colore del codice nei blocchi |
| `codemirror` | 6.0.2 | 2025-06-19 | MIT | — | 7 109 920 | l'editor, ADR-0030 |
| `dompurify` | 3.4.15 | 2026-09-06 | MPL-2.0 OR Apache-2.0 | — | 45 528 659 | **non serve** se il testo non fidato si rende come testo e mai come HTML, §6a del 2 |

**`dockview` 8.x, letto su dockview.dev il 2026-09-07** — pagine `docs/overview/licence`,
`blog/dockview-enterprise`, `docs/core/groups/floatingGroups`, `docs/core/groups/maximizedGroups`,
`docs/core/groups/popoutGroups`, `docs/core/state/save`, `docs/core/locked`, `docs/advanced/accessibility`,
`docs/advanced/keyboard`, `docs/core/panels/move`, `docs/core/panels/tabs`, `docs/core/dnd/thirdParty`:

| Fatto | Conseguenza qui |
|---|---|
| dalla 8.0.0 due forme: il core resta **MIT** e gratis, `dockview-enterprise` è a pagamento; «nothing you rely on today has been taken away» | si usa il core; niente di enterprise entra nel disegno |
| **gratis**: gruppi galleggianti (quanti si vuole, con layout annidato, tenuti dentro la finestra, con un gancio `transformFloatingGroupDrag` per lo scatto a griglia o l'allineamento); massimizza e ripristina; finestre popout; salva e ripristina il layout (`toJSON`); blocco del layout e dei singoli gruppi; tocco e penna; drag esterno e librerie terze; schede e presa personalizzate; gruppi senza intestazione; `moveTo` programmatico; ruoli ARIA, annunci allo screen reader, indicatori di focus, navigazione di focus programmatica; CSP stretta | è ciò con cui la 2 «batte la 3»: pannelli liberi, pagina intera, finestra a parte, mano come puntatore, presa grande disegnata da noi, nucleo bloccato, salvataggio in JSON |
| **a pagamento**: navigazione spaziale da tastiera e aggancio da tastiera; guide e bussola durante il trascinamento; cronologia annulla/ripeti; gruppi ai bordi che si nascondono; schede appuntate, multi-riga, menu contestuali | G20 chiede la tastiera: le scorciatoie per spostare un pannello **si scrivono noi** sopra `moveTo`, dichiarato; lo scatto a griglia si scrive noi sul gancio; annulla/ripeti non è chiesto |
| v8 è additiva: ogni novità è opt-in, un solo cambio di comportamento (le dimensioni riportate ai pannelli escludono l'intestazione) | «novità non è maturità» regge: la 8.x non rompe la 7.x; le versioni si riverificano il giorno dello spike |

## I wireframe, e il loro stato

Disegnati in chat con lo strumento inline, poi salvati come SVG autonomi. Sono a bassa fedeltà: box,
etichette, il numero del sotto-progetto che riempie ogni modulo, e il tratteggio per «modulo mobile».
Colori e forme si decidono dopo, nel design system in tre momenti del 2.

| Wireframe | File | Stato | Verificato · dedotto · assunto |
|---|---|---|---|
| **Home** | [home.svg](2026-09-07-direzione-gui-wireframes/home.svg) | ✅ **approvata**, con la chat nel cassetto (domanda 7) | verificato: le tessere da G9–G18, il pannello knowledge base dal suo disegno, le funzioni di `dockview`; dedotto: la striscia «sempre visibile» come gruppo bloccato di `dockview`, da provare nello spike; assunto: l'aspetto Jarvis lo danno colori e forme |
| **Lavoro** | [lavoro.svg](2026-09-07-direzione-gui-wireframes/lavoro.svg) | ✅ **approvata come mappa alla ripresa del 2026-09-07** — diceva *«⏳ presentata: A su Passi, nessuna modifica chiesta, approvazione da confermare»*; il contesto della run, la modalità di esecuzione e «+ allegati» entrano nella riga Chat del catalogo, nella barra della chat, per run (decisione 10) | verificato: ogni riquadro ha una riga G o un ADR (G4, G5, G7, G13, G14, G18, ADR-0014, ADR-0016, ADR-0017, ADR-0024); dedotto: la richiesta di permesso in riga invece che in finestra — nel 2 resta la finestra della §6a, in riga è la forma del 3; assunto: niente |
| **Compatta e nucleo a pagina intera** | [compatta-e-grafo.svg](2026-09-07-direzione-gui-wireframes/compatta-e-grafo.svg) | ✅ **approvata come mappa alla ripresa del 2026-09-07** — diceva *«⏳ presentata, NON approvata: la sessione si è chiusa prima della risposta»*; ciò che Compatta e il nucleo a pagina intera mostrano va nel catalogo, righe del 10 e del 6 (decisione 10) | verificato: il pannello dalla §4.3 del disegno della knowledge base, decisioni 6, 9, 10; gli stati ascolto/pensiero/parlato da tracciabilità; V9 per la notifica; l'indicatore acceso dal core, ADR-0039; dedotto: trascinare un nodo è la funzione «sposta» del registro, quindi chiede permesso; se Compatta sia una finestra staccata di `dockview` o la finestra principale rimpicciolita lo decide il 10; assunto: niente |

Il quarto disegno, Lavoro, contiene la **quarta vista** implicita: un modulo a pagina intera è la stessa
cosa per tutti i moduli, quindi non ha un wireframe suo.

## Il modello della GUI, com'è stato approvato in chat

| Pezzo | Forma | Da dove |
|---|---|---|
| **modulo** | un **tipo** registrato nella SPA — Chat, Stato, Attività, Ambito, Diff, Anteprima, Terminale, Passi, Sensori, Costi, Permessi, Knowledge base, Asset 3D, Voce e gesti, Nucleo — e i suoi **esemplari**: un pannello di `dockview` con parametri, per esempio il numero della run. Un tipo, tanti esemplari; i tipi sono pochi, gli esemplari quanti si vuole | domanda 3; il modello di `dockview` |
| **regola unica** | ogni modulo si usa sul posto, in piccolo, **oppure** si apre a pagina intera; vale anche per il nucleo (domanda 2) | domande 2 e 3 |
| **vista** | un layout salvato con un nome: JSON di `dockview`; tre arrivano con l'app — **Home**, **Lavoro**, **Compatta** — e il proprietario può salvarne altre; costa un nome e una lista | domande 5 e 6, e la risposta su come cresce |
| **Home** | il **nucleo** al centro, bloccato: l'anello degli artefatti recenti per data, e dentro la rete viva di tutti i file; intorno le tessere agganciate; sopra la barra con le viste, la ricerca, il chip del core, e la fascia che compare solo se il core manca o il timbro è sbagliato; sotto la striscia sempre visibile e il cassetto «+ moduli» | il wireframe Home |
| **Lavoro** | Attività e Ambito a sinistra; la chat a schede al centro, ogni scheda una run, «stacca» per un pannello libero o un'altra finestra; Diff, Anteprima e Terminale a destra; Passi e Sensori in basso | il wireframe Lavoro |
| **Compatta** | una finestrella sempre in primo piano: la rete viva senza anello come presenza (ascolto, pensiero, parlato, fermo), l'ultima notifica, microfono e telecamera accesi dal core, la scorciatoia globale, «grande» per tornare al nucleo | il wireframe Compatta, da confermare |
| **striscia sempre visibile** | in ogni vista: degrado, permessi, contesto, costo e tetto, attese, telecamera, microfono — la lista «deve mostrare sempre» G9–G14 più i due indicatori di percezione | G9–G14; ADR-0039 |
| **ricerca** | sui nomi degli artefatti, dalla barra: colora i trovati nell'anello e nella rete; a pagina intera è la ricerca del pannello knowledge base | domanda 1; §4.3 del disegno della knowledge base |
| **come cresce** | più chat = più esemplari come schede, affiancati o in un'altra finestra; la Home non si allunga: Attività è l'indice di tutte le run, l'anello mostra gli ultimi file e il nucleo a pagina intera li ha tutti; una funzione nuova = un tipo nuovo registrato, che compare nel cassetto col suo numero; le viste sono JSON, niente da ridisegnare | la risposta al proprietario sul come regge |
| **dove si rompe, e cosa lo tiene** | tante chat che scrivono insieme mangiano CPU, e P3 era già stretto: `dockview` disegna solo i pannelli visibili («render modes»), la frequenza la decide il core (ADR-0027), e — dedotto, per il 3 — la GUI dice al core quali run guarda e il core manda solo quelle; un layout salvato che punta a una run sparita: il pannello lo dice a parole e si chiude | ADR-0027; la tabella delle licenze di `dockview` |
| **la disposizione** | vive nel core, archivio di configurazione di ADR-0022, che il 2 costruisce con una voce sola; il core la manda al collegamento, la GUI manda «salva disposizione» | domanda 4 |
| **la mano** | i moduli si muovono con qualunque puntatore; il 12 aggiunge il pinch: `dockview` supporta tocco e penna e le librerie terze di trascinamento | §4.1 del disegno dei gesti; la doc di `dockview` |
| **cosa mostra davvero il 2** | la cornice con `dockview`; Home col nucleo che a parole dice «niente ancora», Stato e Permessi vivi; Lavoro con la chat del core finto, la finestra di permesso, Passi con le invocazioni del registro; le altre tessere dicono a parole chi le riempie; Compatta non esiste ancora | domande 0 e 8 |

## Le sezioni approvate del disegno — il merito com'è stato approvato

### §1 — Il catalogo dei moduli · chiusa il 2026-09-07: cinque tabelle piene e una corta (decisione 12)

La regola, dalla decisione 10: un wireframe è una **mappa**; tutto ciò che un modulo mostra, e i comandi
che ha, sta qui, **una tabella per modulo**. Ogni riga porta la **fonte** — una riga G di
[`spikes/GUI-REQUISITI.md`](../../../spikes/GUI-REQUISITI.md), un ADR, una riga di
[tracciabilità](../../tracciabilita.md) — e il **sotto-progetto che costruisce la riga**, coi numeri della
[roadmap](../../roadmap.md); il «dove» dentro il modulo quando conta. Una riga **senza fonte non entra**:
prima diventa una riga G. Ciò che non si può decidere oggi va nelle «Registrate, non prese» con chi lo
chiude. Il catalogo dice **cosa** e **chi**, non come appare: ogni sotto-progetto disegna il proprio modulo
quando arriva, e una funzione nuova è una riga nuova, non un wireframe nuovo. Un modulo il cui
sotto-progetto non è chiuso mostra a parole chi lo riempie.

#### Chat · approvata il 2026-09-07

**Tipo** registrato nella SPA; **esemplare**: una scheda per run. Costruito dal **2** — la cornice, il flusso
del core finto, la finestra di permesso — e dal **3**, la chat vera. Messaggi IPC oggi: `Token`,
`PermissionRequired`, `Approve`; col 3 i messaggi della run.

| # | Dove | Cosa mostra o fa | Fonte | Chi | Verificato · dedotto |
|---|---|---|---|---|---|
| 1 | flusso | testo in markdown, blocchi di codice, un token alla volta | G4 · `Token` | 2 col core finto, 3 vero | verificato |
| 2 | flusso | ogni pezzo con la provenienza; il non fidato si rende come testo, mai HTML, e nessun link si apre da solo | G13 · ADR-0014 · §6a del 2 | 2 | verificato |
| 3 | flusso | la richiesta di permesso, la tripla a parole: finestra nel 2, **in riga** dal 3 | ADR-0016 · ADR-0038 · `PermissionRequired`, `Approve` | 2, 3 | verificato; «in riga» dedotto |
| 4 | flusso | «la run aspetta te», con notifica | G14 · V9 | 3 | verificato |
| 5 | flusso | l'esito di un fallback: avviso nel flusso se la catena cede su qualità o costo, errore se cede sui vincoli dei dati | ADR-0012 | 3 | verificato |
| 6 | flusso | `InCoda` e `Rifiutata` come due esiti diversi, resi diversi | G15 · ADR-0012 | 3 | verificato |
| 7 | flusso | un passo in dubbio dopo un crash, effetto irripetibile: domanda all'utente, nessun replay | ADR-0007 | 3 | verificato |
| 8 | flusso | un messaggio potato dalla ritenzione: si vede che c'era, con impronta e dimensione, mai indistinguibile da uno mai registrato | ADR-0018 | 3 | fonte verificata; il «chi» dedotto |
| 9 | flusso | il primo uso di uno strumento MCP: descrizione integrale e impronta all'approvazione; se cambia, sospeso col diff | ADR-0015 · righe «MCP» e «Difesa da tool poisoning» di tracciabilità | 4 | verificato |
| 10 | barra, per run | la casella di scrittura e «invia» | G4 · §1 del 2: «casella di scrittura → il 3» | 3 | verificato |
| 11 | barra, per run | il contesto della run: occupazione per categoria | G11 · ADR-0010 · riga «Indicatore di riempimento contesto» | 3, col dato dal 13 | verificato, decisione 10 |
| 12 | barra, per run | la modalità di esecuzione: i tre preset | ADR-0016 · riga «Modalità di permessi a più livelli» | 4 i preset, 3 la mostra per run | verificato, decisione 10; per run o globale nelle registrate. ✅ **Richiamo del 2026-09-07:** il «chi» diceva 3; la riga di tracciabilità assegna i preset al 4, corretto con l'approvazione del proprietario |
| 13 | barra, per run | il modello in uso, dal record di routing | ADR-0011 · riga «Indicatore di stato modello» | 3 | verificato, decisione 11; il selettore a mano nelle registrate |
| 14 | barra, per run | «+ allegati»: file e immagini, marcati non fidati | righe «Allegati in chat» e «Input immagini e vision» · ADR-0014 | 3 | verificato; il rapporto con «aggiungi al contesto» nelle registrate |
| 15 | barra, per run | il microfono: dettatura e push-to-talk, gli stati ascolto, pensiero, parlato | righe «Push-to-talk e dettatura» e «Stati di ascolto/pensiero/parlato» · ADR-0011 | 8 | fonte verificata; il «dove» dedotto |
| 16 | barra, per run | le guide attive per questa run: skill e profilo | ADR-0009 · righe «Skills» e «System prompt, personas e profili» | 13 il registro, 3 la mostra | fonte verificata; il «dove» dedotto |
| 17 | barra, per run | gli strumenti di questa run: server MCP attivi e sospesi | ADR-0003 · ADR-0019 · righe «MCP» e «Tool calling» | 4 | fonte verificata; il «dove» dedotto |
| 18 | comando | ferma la risposta; il costo dello stream interrotto resta nel giornale | riga «HITL: interruzione e steering» · riga «Run persistenti, ripresa e cancellazione» — la «cancellazione» è l'annullamento di una run o di una richiesta in corso, SP-4 della spec del kernel · ADR-0011 | 4 | verificato; fonte allargata il 2026-09-07 dopo l'approvazione, contenuto invariato |
| 19 | comando | «+ nuova run»: una chat è una run | ADR-0011, corollario | 3 | verificato; «chiede la cartella» nelle registrate |
| 20 | comando | «stacca» la scheda in un pannello libero o in un'altra finestra; manipolazione della GUI, non passa dal registro | `dockview` · ADR-0038 | 2 | verificato |
| 21 | comando | fork e branching, modifica e rigenerazione, ricerca nello storico, template e prompt salvati, esportazione: una riga ciascuno quando il 3 li disegna | le cinque righe di tracciabilità, casa Conversazione | 3 | verificato |
| 22 | scheda | una scheda è una run, col numero e lo stato; dove si vedono le sub-run lo disegna il 3 | ADR-0011 · righe «Sessioni multiple» e «Orchestrazione e sub-agenti» | 3 | verificato; le sub-run dedotte |
| 23 | scheda | costo e token di questa run | ADR-0011 | 3 | fonte verificata; il «dove» dedotto |
| 24 | flusso | senza una run, la chat lo dice a parole invece di restare vuota | §6a del 2, «i quattro stati della connessione» | 2 | ⚠️ **aggiunta dopo l'approvazione**, scrivendo, dalla §6a già approvata: il proprietario può toglierla |

**Non entra, per decisione già presa:** nessun comando «compatta» — la ricomposizione della proiezione è
continua e proattiva (ADR-0010): il contesto si vede (riga 11), non si comanda; nessuna pulizia del testo
non fidato — si marca, non si sana (ADR-0014).

**Esaminate e senza fonte oggi**, quindi fuori finché non diventano una riga G: rinominare una run;
cancellare una chat dallo storico (il giornale è append-only, ADR-0007: sarebbe una decisione, non una riga; la «cancellazione» di tracciabilità è un'altra cosa, l'annullamento di una run in corso, riga 18); «pensa di
più»; un interruttore «modalità piano» nella barra (il piano è del 4 come capacità, non come interruttore);
mettere in coda un messaggio mentre il modello scrive; fissare o archiviare una chat.

Debiti dichiarati: le righe del 4, dell'8 e del 13 nascono a parole nel 2 e nel 3 — «chi le riempie» — come
dice la regola; la Chat del 2 non ha la casella di scrittura (riga 10, il 3). 🔶 Dedotto, da confermare da
chi costruisce: «in riga» per la richiesta di permesso dal 3 (riga 3); il «chi» della riga 8; il «dove»
delle righe 15, 16, 17 e 23; le sub-run della riga 22.

Controllo sui cinque criteri: fonti lette il 2026-09-07 nel compendio (§5 intera), in
`spikes/GUI-REQUISITI.md` e in tracciabilità, i dedotti marcati; stessa forma della decisione 10 e numeri
della roadmap; le voci aperte nelle registrate; nessuna dipendenza da scegliere qui; solo righe con una
fonte oggi, niente disegno in pixel.

#### Stato · approvata il 2026-09-07

**Tipo** registrato nella SPA; **esemplare**: uno. È la tessera «Stato» della Home, viva nel 2; la striscia
sempre visibile ne è il riassunto (decisione 4 del coordinatore), il modulo è l'intero. Costruito dal **2**.
Messaggi IPC del disegno del 2, nomi provvisori (§4 del 2): `Degradation`, `Policy`, `Accepted`, `Verdict`.

| # | Cosa mostra | Fonte | Chi | Verificato · dedotto |
|---|---|---|---|---|
| 1 | il degrado corrente: i due campi di oggi — `vram_exhausted` e `routing_degraded`, in `crates/kernel/src/degradation.rs` — ognuno con la causa; cresce con chi porta le cause che ADR-0019 nomina: connettività, arbitro GPU, salute dei provider (3), permessi, strumenti sospesi (4); e la telecamera (12, ADR-0039) | G9 · ADR-0019 · `Degradation` · ADR-0039 · righe «Degrado esplicito quando manca la rete» e «Comportamento offline» | 2, poi 3, 4, 12 | verificato, campi letti nel codice |
| 2 | la policy VRAM attiva e il budget allocato sul totale; il totale è il budget allocabile: tutto meno la quota audio e la quota di presentazione | ADR-0006 · ADR-0005 · ADR-0033 · `Policy` · righe «Budget VRAM esplicito» e «Policy differenziata remoto vs locale» | 2 | verificato; le due quote sottratte visibili: dedotto |
| 3 | «protetto quanto il tuo account di sistema», come valore da `Accepted`, non scritta fissa | G16 · ADR-0023 · `Accepted` | 2 | verificato; la cifratura reale è «sede da assegnare» in tracciabilità |
| 4 | una riga di evento per l'ultimo `Verdict`, solo quando ne arriva uno; `InCoda` e `Rifiutata` distinti | G15 · ADR-0012 · `Verdict` · §6a del 2 | 2; il mittente di `Request` arriva col 7 | verificato |
| 5 | il profilo «riservato» attivo e ciò che spegne: avvio automatico, voce always-on, telecamera | ADR-0023 · ADR-0039 | 3, col gestore dei segreti (riga «Gestione segreti e credenziali») | fonte verificata; il «chi» dedotto |
| 6 | la transizione di policy, quando è offerta: gli effetti osservabili prima di accettarla — eviction, ricarica, notifica | ADR-0006 · riga «Swap coordinato» | 9 | fonte verificata; il «chi» dedotto |
| 7 | l'occupazione della GPU nel tempo, come grafico | G8 · ADR-0005 | 9 | fonte verificata; se vive qui o nel modulo Modelli locali: nelle registrate |
| 8 | chi tiene la VRAM adesso: le concessioni attive per titolare — i worker, la quota audio, la quota di presentazione della GUI stessa | ADR-0005 · ADR-0033 · righe «Semaforo unico delle risorse GPU» e «Budget di VRAM riservata all'audio» | 7 o 9, chi porta il primo worker che tiene VRAM | fonte verificata; il «chi» dedotto; serve un messaggio IPC nuovo, dedotto |
| 9 | i livelli di confinamento disponibili su questa macchina; se manca il livello 2, l'esecuzione di codice non parte | ADR-0025 · ADR-0019 · righe «Sandboxing ed esecuzione» e «Permessi e sandbox policy», L-5 | 5 | fonte verificata; se qui o nel modulo Permessi: dedotto |
| 10 | dopo un riavvio: la riconciliazione del giornale, e quanti passi restano in dubbio | ADR-0007 · ADR-0018 · riga «Run persistenti, ripresa e cancellazione» | 3 | fonte verificata; il «dove» dedotto |
| 11 | la telemetria: nessuna lascia la macchina per default; se l'esportazione è accesa, verso dove | ADR-0017 · V25 · riga «Telemetria locale e «no telemetry» garantito» | chi costruisce l'esportazione: nelle registrate, del proprietario | fonte verificata; il «chi» dedotto |
| 12 | la versione del programma, e se c'è un aggiornamento | riga «Packaging e aggiornamenti» | 10 | fonte verificata; se qui o nel modulo Impostazioni: dedotto |

**Nessun comando:** il cambio di policy è una funzione del registro e sta nel modulo Impostazioni (§5 e §6a
del 2, ADR-0038): Stato mostra, non comanda.

**Esaminate e senza fonte oggi:** l'uso di CPU e RAM (M1–M5 sono misure dello spike, non righe della GUI); un
tasto «riavvia il core» (la fascia ha «riprova», che ricollega, §6a del 2); l'elenco dei worker (ADR-0004 e
ADR-0028 li descrivono, nessuna riga dice di mostrarli: entrano solo come titolari di VRAM, riga 8). Il registro
degli eventi non è qui: è il modulo Passi (riga «Osservabilità e tracing locale → GUI minima»).

Debiti dichiarati: le righe del 3, del 5, del 7, del 9, del 10 e del 12 nascono a parole nel 2. 🔶 Dedotto, da
confermare da chi costruisce: le quote sottratte della riga 2; il «chi» delle righe 5, 6, 8 e 11; la casa delle
righe 7, 9, 10 e 12; il messaggio IPC della riga 8. ✅ Le righe 8–12 vengono dalla seconda passata su ADR e
tracciabilità chiesta dal proprietario, approvate il 2026-09-07 prima di essere scritte.

Controllo sui cinque criteri: fonti lette il 2026-09-07 — la §5 del compendio, le righe G, tracciabilità, la
§4 e la §6a del 2, e i due campi nel codice; stessa forma della tabella Chat; le voci aperte nelle registrate;
nessuna dipendenza da scegliere; solo righe con una fonte oggi.

#### Permessi · approvata il 2026-09-07

**Tipo** registrato nella SPA; **esemplare**: uno. È la tessera «Permessi» della Home, viva nel 2 — la tripla del
registro, oggi una, e la finestra di conferma; la voce «permessi» della striscia ne è il riassunto. Costruito dal
**2**, poi dal 3 (le run) e dal 4 (i preset, i server MCP). Messaggi IPC del disegno del 2: `PermissionRequired`,
`Approve`; la lista delle triple concesse chiede un messaggio nuovo (riga 1).

| # | Cosa mostra o fa | Fonte | Chi | Verificato · dedotto |
|---|---|---|---|---|
| 1 | le triple concesse nella sessione — strumento, risorsa, operazione — ognuna con chi l'ha chiesta, quando, per quale run | G10 · V21 · ADR-0016 · ADR-0038 | 2 la tripla del registro, 3 per run | fonte verificata; il messaggio IPC con la lista, dal core (I1): dedotto |
| 2 | quanto dura un permesso: quella tripla, quella sessione; finché il 3 non costruisce il confine di sessione la GUI non lo promette | ADR-0016 · V21 · §5 del 2, letto nel codice: `is_granted` rilegge tutto il giornale | 2 dichiara, 3 costruisce | verificato |
| 3 | la richiesta in attesa, e quante ce ne sono per run: la tripla a parole di tutti i giorni, la classe dell'effetto, chi la invoca, il livello di confinamento richiesto e se c'è | ADR-0016 · ADR-0038 · ADR-0007 · ADR-0025 · `PermissionRequired` · riga «HITL: approvazioni» | 2 il click, 3 l'agente, 5 il livello, 8 la voce, 12 il gesto | verificato; il livello qui o in Stato (riga 9 di Stato): dedotto |
| 4 | il preset attivo — chiede sempre, auto-approva sicuri (default), autonomo — e cosa ognuno lascia passare | ADR-0016 · riga «Modalità di permessi a più livelli» | 4 | verificato; per run o globale nelle registrate |
| 5 | i vincoli sui dati del profilo: ritenzione, provider esclusi, solo locale; e l'escalation automatica quando un contenuto è passato dal gestore dei segreti | ADR-0016 · ADR-0012 · ADR-0023 · riga «Zero-Data-Retention selettivo» | 3 | fonte verificata; qui o in Stato: dedotto |
| 6 | i server MCP e i loro strumenti: la descrizione approvata con l'impronta; sospeso col diff se cambia | ADR-0003 · ADR-0015 · righe «MCP» e «Difesa da tool poisoning» | 4 | verificato |
| 7 | regola: si chiede uguale anche se la richiesta nasce da contenuto non fidato o da un evento di percezione — informano, non autorizzano | ADR-0014 · ADR-0038 | 2 in poi | verificato |
| 8 | regola: un effetto irripetibile chiede conferma a chiunque lo invochi; per default la conferma non è gestuale, e quali funzioni siano gestuali lo decide il 12 | ADR-0038 · ADR-0007 · riga «Approvazione comandi distruttivi» | 2, 12 | verificato |
| 9 | comando: consenti o rifiuta; nel 2 la finestra con la trappola di focus, dal 3 anche in riga nella chat | ADR-0016 · `Approve` · §6a del 2 · G20 | 2, 3 | verificato; «in riga» dedotto |
| 10 | comando: revoca un server MCP | ADR-0003, «revocabile» | 4 | verificato |

**Non entra, per decisione già presa:** spostare pannelli e menu non chiede permesso — è presentazione, non passa
dal registro (ADR-0038).

**Esaminate e senza fonte oggi:** revocare una singola tripla già concessa (nel codice non c'è); «ricorda per
sempre» (contro «un'approvazione non si estende», ADR-0016); una lista di regole scritta a mano; una scadenza a
tempo di un permesso.

Debiti dichiarati: le righe del 3, del 4, del 5, dell'8 e del 12 nascono a parole nel 2; nel 2 la lista delle
triple è una. 🔶 Dedotto, da confermare da chi costruisce: il messaggio IPC della riga 1; la casa del livello
(riga 3) e della riga 5; «in riga» della riga 9.

Controllo sui cinque criteri: fonti lette il 2026-09-07 — la §5 del compendio, le righe G, V21, tracciabilità, la
§5 e la §6a del 2 e `is_granted` nel codice; stessa forma delle tabelle Chat e Stato; le voci aperte nelle
registrate; nessuna dipendenza da scegliere; solo righe con una fonte oggi. La seconda passata su ADR e
tracciabilità, fatta prima della presentazione, ha portato la correzione del «chi» della riga 12 della Chat.

#### Passi · approvata il 2026-09-07

**Tipo** registrato nella SPA; **esemplare**: uno, o uno per run quando il 3 lo vorrà. È il pannello in basso di
Lavoro. Costruito dal **2** — le invocazioni del registro, gli unici passi che il giornale ha prima del 3
(decisione 5 del coordinatore, domanda 8) — e dal **3**, i passi delle run. Messaggio IPC: la lista dei passi, nuovo,
già nel costo della domanda 8. Nel codice il giornale ha oggi quattro tipi di record — `Intent`, `Outcome`, `Note`,
`Verdict` — e tre specie di dettaglio — `Routing`, `Permission`, `Verdict` — in `crates/kernel/src/record.rs`; il 2
aggiunge `Invocation` (§5 del 2). ⚠️ **RICHIAMO DEL 2026-09-08, sesta ripresa: i tipi di record sono SEI, non quattro** —
`Routing` e `Permission` sono varianti proprie di `RecordKind` (indici 4 e 5), non note con dettaglio; le tre specie di
dettaglio reggono. Trovato scrivendo design/03 (decisione 18); approvato dal proprietario (A). Le righe della tabella non
cambiano.

| # | Cosa mostra o fa | Fonte | Chi | Verificato · dedotto |
|---|---|---|---|---|
| 1 | la lista dei passi, dal core: nel 2 le invocazioni del registro — funzione, invocatore, argomento, classe dell'effetto, esito; dal 3 i passi delle run | domanda 8 · decisione 5 del coordinatore · §5 del 2 · ADR-0038 · righe «Replay dei trace» e «Osservabilità e tracing locale» | 2, 3 | verificato |
| 2 | ogni passo com'è nel giornale: intento prima, esito dopo, le note, i verdetti; un intento senza esito è **in dubbio** e si vede così, con la classe dell'effetto | ADR-0007 · `RecordKind` in `crates/kernel/src/record.rs` | 2 | verificato, letto nel codice |
| 3 | il dettaglio del passo secondo la specie: routing, permesso, verdetto oggi; l'invocazione col 2 | `Detail` in `crates/kernel/src/record.rs` · §5 del 2 | 2 | verificato, letto nel codice |
| 4 | per un passo di modello: il record di routing risolto — modello, destinazione, provider, parametri, vincoli, catena di riserva valutata, tentativi, esito — con token e costo; un ritentativo non è un passo nuovo; uno stream interrotto ha comunque il suo costo | ADR-0011 · righe «Contabilità token e costi» e «Cronologia e riproducibilità» | 3 | verificato |
| 5 | la gerarchia: passo dentro run dentro run padre; i costi sono aggregazioni della stessa gerarchia | ADR-0011 · riga «Analisi dei costi per run e per sub-agente» | 3 | verificato |
| 6 | i verdetti dei sensori sul passo, schema compreso; una correzione è un passo nuovo | ADR-0009 · ADR-0013 | 3; il modulo Sensori è del 4 | verificato |
| 7 | per un passo che tocca file: la versione precedente, conservata e riferita dal passo, da cui «ripristina dal checkpoint» | ADR-0024 · G18 · righe «Checkpoint e rollback» e «Undo/checkpoint del filesystem» | 5 | verificato |
| 8 | il passaggio esplicito da contenuto non fidato a istruzione, giornalato, come evento del passo | ADR-0014 | 3 | fonte verificata; il «dove» dedotto |
| 9 | il payload potato: impronta e dimensione al posto del testo, distinto da «mai registrato»; un passo in dubbio non si pota | ADR-0018 | 3 | fonte verificata; il «chi» dedotto |
| 10 | la sostituzione di un parametro consegnato, come passo giornalato | ADR-0034 | chi porta la prima sostituzione | fonte verificata; il «chi» dedotto |
| 11 | un gesto di comando come passo nella run aperta, con l'invocatore «gesto» | ADR-0039 · ADR-0038 | 12 | verificato |
| 12 | comando: il replay — scorrere i passi di una run nell'ordine, intento ed esito; non è la ripresa del core, che è riconciliazione | riga «Replay dei trace» · ADR-0017 · ADR-0007 | 2, 3 | verificato |
| 13 | comando: esporta il trace — solo via OTLP, opt-in, verso una destinazione scelta dall'utente; nulla esce per default | ADR-0017 · V25 | chi costruisce l'esportazione: nelle registrate | fonte verificata; il «chi» dedotto |
| 14 | regola: Passi è una proiezione del giornale, non uno stato suo — si rilegge dal core; i campi della proiezione seguono il vocabolario GenAI di OpenTelemetry, le scritte restano in `locales/it.json` | ADR-0017 · I1 · G1 · G21 | 2 | verificato |

**Non entra, per decisione già presa:** modificare o cancellare un passo — il giornale è append-only (ADR-0007).
Andare dal passo alla sua run, al diff o al checkpoint è presentazione (ADR-0038): la disegna chi costruisce, non
è una riga.

**Esaminate e senza fonte oggi:** rieseguire un passo a mano (la riesecuzione è la riconciliazione del core,
ADR-0007; il «rigenera» della chat è un'altra cosa, riga 21 della Chat); un grafico dei tempi per passo (G8 è
costo e occupazione, non tempi); i filtri per specie e per run sono presentazione.

Debiti dichiarati: le righe del 3, del 4, del 5 e del 12 nascono a parole nel 2, e nel 2 la lista è delle sole
invocazioni. 🔶 Dedotto, da confermare da chi costruisce: il «dove» della riga 8; il «chi» delle righe 9, 10 e 13.

Controllo sui cinque criteri: fonti lette il 2026-09-07 — la §5 del compendio, le righe G, V25, tracciabilità, la
§5 del 2 e `record.rs` nel codice; stessa forma delle tabelle precedenti; le voci aperte nelle registrate; nessuna
dipendenza da scegliere; solo righe con una fonte oggi. Seconda passata su ADR e tracciabilità fatta prima della
presentazione: da lì le righe 8–11, 13 e 14.

#### Attività · approvata il 2026-09-07

**Tipo** registrato nella SPA; **esemplare**: uno. È la tessera «Attività» della Home e il pannello in alto a sinistra
di Lavoro: l'indice di tutte le run (domanda 1). Costruito dal **3** (le run), dal **4** (piano, coda, trigger come
funzione) e dal **13** (il meccanismo dei trigger); nel 2 la tessera dice a parole «arriva col 3». Messaggi IPC: quelli
delle run, del 3.

| # | Cosa mostra o fa | Fonte | Chi | Verificato · dedotto |
|---|---|---|---|---|
| 1 | l'indice di tutte le run: numero, stato, quando; una chat è una run | ADR-0011, corollario · domanda 1 · riga «Sessioni multiple» | 3 | verificato |
| 2 | le sub-run sotto la run padre; dal 5 il loro isolamento su disco | ADR-0011 · [design/03](../../design/03-run-durevoli.md) · righe «Orchestrazione e sub-agenti» e «Agenti in parallelo isolati» | 3, 5 | verificato |
| 3 | lo stato di ogni run: in corso; aspetta te, con notifica; finita; annullata; in dubbio dopo un riavvio | G14 · V9 · ADR-0007 · righe «Run persistenti, ripresa e cancellazione» e «Notifica «l'agente ha bisogno di te»» | 3; la notifica di sistema col 10 (riga «Notifiche → L3») | verificato; i nomi degli stati li fissa il 3: dedotto |
| 4 | se la richiesta di una run è in coda dall'arbitro: l'avviso di conflitto e la stima d'attesa; in coda e rifiutata distinti | G15 · V4 · ADR-0012 · riga «Avviso di conflitto e stima d'attesa» | 3 | verificato |
| 5 | la coda e le priorità delle run | riga «Coda e priorità delle run» · ADR-0005, le corsie | 4 | verificato |
| 6 | le run partite da un trigger e non dall'utente — pianificazione, cambiamento di file, fine di un'altra run, hook del ciclo di vita — con la causa | ADR-0009 · righe «Scheduling», «File watching e awareness del progetto» e «Hook sul ciclo di vita» | 13 il meccanismo, 4 la funzione, 6 la politica dei file | verificato |
| 7 | il piano di una run e lo stato dei suoi passi | ADR-0008 · riga «Planning e decomposizione dei task» | 4 | verificato |
| 8 | l'avanzamento e la notifica dei job lunghi | riga «Progress e notifiche per job lunghi → GUI minima» · V9 | 2 il meccanismo, 7 il primo job | fonte verificata; qui o nella striscia: dedotto |
| 9 | il costo per run e per sub-run | ADR-0011 · riga «Analisi dei costi per run e per sub-agente» | 3 | verificato; anche nel modulo Costi |
| 10 | la cartella o l'ambito della run | la registrata «ambito per progetto o per run» | 3 | dedotto, segue la registrata |
| 11 | comando: «+ nuova run»; la cartella: registrata | ADR-0011 | 3 | verificato |
| 12 | comando: apri la run — la sua chat come scheda, esemplare del tipo Chat | domanda 3, la regola unica del modello | 2 l'apertura, 3 | verificato |
| 13 | comando: ferma o annulla una run; riprendi una run che aspetta | righe «Run persistenti, ripresa e cancellazione» e «HITL: interruzione e steering» · ADR-0007 | 3, 4 | verificato |
| 14 | comando: fork — una run nuova da un punto di un'altra | riga «Fork e branching» | 3 | verificato |
| 15 | comando: cerca nello storico; esporta | righe «Ricerca nello storico» e «Esportazione conversazioni» | 3 | verificato; stessa funzione della Chat (riga 21), qui sull'indice |
| 16 | regola: Attività è un indice, proiezione del giornale; la Home non si allunga — l'anello mostra i file, Attività le run | il modello della GUI, «come cresce» · I1 · ADR-0008 | 2 | verificato |

**Non entra, per decisione già presa:** cancellare una run dallo storico — il giornale è append-only (ADR-0007),
come per la Chat.

**Esaminate e senza fonte oggi:** appuntare o archiviare una run; ordinare a mano; etichette; cartelle di run
(l'ambito come progetto è già registrata, chiusore il 3).

Debiti dichiarati: nel 2 la tessera dice a parole «arriva col 3»; le righe del 4, del 5, del 6, del 7, del 10 e
del 13 nascono a parole nel 3. 🔶 Dedotto, da confermare da chi costruisce: i nomi degli stati (riga 3); la casa
della riga 8; la riga 10, che segue la registrata.

Controllo sui cinque criteri: fonti lette il 2026-09-07 — la §5 del compendio, le righe G, V4, V9, tracciabilità,
il modello della GUI; stessa forma delle tabelle precedenti; le voci aperte nelle registrate; nessuna dipendenza da
scegliere; solo righe con una fonte oggi. Seconda passata su ADR e tracciabilità fatta prima della presentazione:
da lì le righe 4, 5, 6, 8 e 14.

#### Gli altri tredici moduli, in una tabella corta · approvata il 2026-09-07 (decisione 12)

Qui ogni modulo ha una riga: chi lo costruisce, ciò che ADR e requisiti gli **impongono** di mostrare, le fonti.
Niente comandi, niente regole, niente dedotti: la tabella piena la scrive il sotto-progetto col suo disegno, quando
arriva (sezione 6), e quel giorno sostituisce la riga con richiamo datato. La **cornice** — la barra delle viste, la
ricerca, il chip del core, la fascia, la striscia, il cassetto «+ moduli» — non è un modulo: la descrive «Il modello
della GUI».

| Modulo | Chi | Ciò che le fonti impongono di mostrare | Fonti |
|---|---|---|---|
| **Ambito** | 5 | la cartella di lavoro della run, scelta all'avvio e sempre visibile nell'intestazione della chat; le cartelle aggiunte durante la run, ogni aggiunta una dichiarazione di ambito; cosa copre il checkpoint e cosa no, **prima** che l'agente scriva; il file troppo grande escluso, con avviso. Simil Claude Desktop: decisione 13; pannello o riga lo decide il 5. Coi permessi: la richiesta di scrittura su un file dice se il percorso è dentro l'ambito (copia di sicurezza) o fuori (nessuna) | ADR-0024 e il suo follow-up · G18 · ADR-0016 · righe «Checkpoint e rollback» e «Undo/checkpoint del filesystem» |
| **Diff** | 5 | le modifiche a un file, approvabili o rifiutabili; la versione prima, conservata, e «ripristina dal checkpoint»; il renderer è una libreria da scegliere il giorno del 5 col comando sul registro npm — CodeMirror, già scelto come editor da ADR-0030, porta una vista diff e merge, da verificare quel giorno | G5 · ADR-0024 · ADR-0030 |
| **Anteprima** | 3 | artifacts e canvas con anteprima viva; il contenuto non fidato resta segnato; la webview esegue solo contenuto locale nostro | G7 · G13 · ADR-0014 · ADR-0027 e il suo follow-up |
| **Terminale** | 5 | i comandi eseguiti dall'agente e il loro output, col livello di confinamento — mai sotto il 2 per codice e comandi; l'output è contenuto non fidato | ADR-0025 · ADR-0014 · righe «Sandboxing ed esecuzione» e «Permessi e sandbox policy» |
| **Sensori** | 4 | i sensori registrati, con la classe di costo; i verdetti nel giornale; l'anello di miglioramento — la ricorrenza rilevata e la **proposta**, che l'utente approva; il canary di esfiltrazione come verdetto che blocca | ADR-0009 · ADR-0013 · ADR-0016 |
| **Costi** | 3 | costo corrente e distanza dal tetto; costo per run e per sub-run come aggregazioni della gerarchia; i grafici di costo; gli avvisi e i tetti di spesa | G12 · G8 · V8 · ADR-0011 · righe «Avvisi e tetti di spesa» e «Analisi dei costi per run e per sub-agente» |
| **Knowledge base e Nucleo a pagina intera** | 6 | il pannello della §4.3 del disegno della knowledge base: il grafo per router, i filtri per specie, cartella ed etichetta, la ricerca sui nomi, il nodo con le funzioni del registro, orfani e collegamenti rotti; a pagina intera è il nucleo (domanda 2); nell'anello solo file, per data (domanda 1) | [disegno della knowledge base](2026-09-04-knowledge-base-design.md), §4.3 · ADR-0008 · ADR-0018 · ADR-0038 |
| **Asset 3D** | 7 | il viewer 3D — rotazione, zoom, materiali — entro la quota di presentazione, con concessione ordinaria oltre (`Request`, `Verdict`); la coda dei job e il progresso; l'export della mesh | G6 · ADR-0033 · ADR-0030 · righe «Coda dei job di generazione», «Progress e notifiche per job lunghi» e «Export mesh (GLB/OBJ/PLY)» |
| **Voce e gesti** | 8, 12 | gli stati ascolto, pensiero, parlato; microfono e telecamera accesi dal core, con indicatore; la mano disegnata dai 21 punti, niente video; solo la wake word apre una run; «riservato» spegne voce always-on e telecamera | ADR-0039 · ADR-0023 · ADR-0011 · righe «Stati di ascolto/pensiero/parlato», «Wake word», «Push-to-talk e dettatura» e «Controlli di privacy del microfono» |
| **Backup** | 11 | cosa il backup **non** contiene, al momento del backup — indici ed embedding, pesi dei modelli, e mai i segreti; il ripristino | G17 · ADR-0022 · righe «Backup ed export dei dati» e «Backup della KB indipendente dall'app» |
| **Checkpoint** | 5 | le versioni conservate per passo e il ripristino: vivono dentro Diff e Ambito; se sia un modulo a sé lo decide il 5 | ADR-0024 · righe «Checkpoint e rollback» e «Undo/checkpoint del filesystem» |
| **Modelli locali** | 9 | catalogo e download; caricamento su richiesta, pre-caricamento, scarico per inattività; il tetto ai modelli residenti; l'indicatore di stato modello; il grafico dell'occupazione GPU, se vive qui (registrata) | ADR-0005 · ADR-0006 · righe «Catalogo e download modelli locali», «Caricamento su richiesta e pre-caricamento», «Scarico per inattività (TTL)», «Tetto ai modelli residenti» e «Indicatore di stato modello» |
| **Impostazioni** | 2, poi 3 e 10 | nel 2 il cambio di policy VRAM, funzione del registro con la sua tripla; poi le preferenze di provider, la telemetria opt-in e la sua destinazione, il profilo «riservato», l'avvio automatico, la lingua | §5 e §6a del 2 · ADR-0038 · ADR-0006 · ADR-0017 · ADR-0023 · G21 · righe «Preferenze di provider (OpenRouter)» e «Avvio automatico e daemon in background» |

Controllo sui cinque criteri, per la tabella corta: fonti lette il 2026-09-07 — la §5 del compendio, le righe G,
tracciabilità, e ADR-0016 e ADR-0024 aperti per la decisione 13; stessa forma delle tabelle piene ma senza comandi,
regole e dedotti, per proporzione (decisione 12); le voci aperte nelle registrate; nessuna dipendenza scelta qui —
CodeMirror è già di ADR-0030, il renderer del diff si verifica il giorno del 5; niente disegno in pixel.

### §2 — Viste e disposizione · approvata il 2026-09-08, con le decisioni 14 e 15 delegate

La **disposizione** — quale vista è aperta, per ogni vista dove stanno i pannelli, e le viste che il proprietario salva
con un nome (domanda 6): il JSON che `dockview` produce con `toJSON()`, più il nome della vista attiva — è stato di
**presentazione** che deve sopravvivere a un riavvio. I1 vieta alla GUI di conservarla da sé: la conserva il **core**.
Fonti: la domanda 4 di questa stella polare, la decisione 12 del disegno dei gesti, I1, la riga «configurazione, guide,
profili» di ADR-0022 — non cifrato, nel backup, permanente. Nessuna riga G la chiede.

⛔ **Ciò che il codice ha detto scrivendo, e che la proposta non sapeva.** La riga 2 delle sezioni che mancano diceva
*«l'archivio in `platform`, consegnato al daemon e non letto dal kernel»*, e **non regge**: `SaveLayout` arriva **dentro
il kernel** — la §5 del 2, approvata, mette il ciclo che ascolta la GUI in un'attività del kernel perché la DST possa
muoverla con `DyingGui` — e il kernel tocca il mondo solo dalle **sei porte** che la §3.1 della spec dichiara
**esaustive** e che il simulatore sostituisce tutte. Nessuna delle sei tiene un pacchetto opaco, letto in
`crates/kernel/src/ports/`: `filesystem` è per gli ambiti di checkpoint, pretende che ogni scrittura conservi prima la
versione precedente **su un passo**, e la sua implementazione vera è del 5; `journal` tiene per sempre solo «la parte
piccola» (ADR-0018) e pota i payload; le altre quattro sono tempo, rete, worker e la GUI stessa. Un daemon che scrive
fuori dal kernel non vede mai passare il messaggio; una chiusura o una cella consegnata all'attività sarebbe una porta
**non nominata** — gotcha #17 dalla porta di servizio, che è ciò contro cui la tabella di `ports/mod.rs` esiste.
Quindi: una **settima porta** (decisione 15), o il giornale piegato.

| # | Pezzo | Forma | La prova |
|---|---|---|---|
| 1 | cos'è «la disposizione» per il core | un pacchetto solo, **opaco**: byte che il core conserva e restituisce, mai apre. Se `dockview` cambia formato, il core non cambia | gli si danno byte che non sono JSON: tornano identici |
| 2 | **la settima porta** (decisione 15) | una famiglia nuova in `kernel::ports`, nome inglese nel disegno scritto: **due operazioni** — tenere dei byte sotto una chiave, ridarli — e **una chiave sola** oggi, un enum chiuso con la variante della disposizione. Non nomina file né percorsi (I3). ⛔ **Non è configurazione del kernel:** un valore su cui il kernel *decide* è consegnato (ADR-0034); qui il kernel *custodisce* ciò che la GUI gli affida e non lo legge mai per decidere — sta scritto nel doc della porta. La riga nella tabella di `ports/mod.rs` e nella §3.1 della spec: **sei → sette**, con richiamo datato, compito del piano | la finta in `crates/kernel/tests/ports_are_implementable.rs`; la suite di conformità sulle due implementazioni coi bugiardi, come `journal_contract` |
| 3 | l'implementazione vera | un modulo nuovo di `platform`: `redb` (ADR-0032) sul `FileBackend` che il giornale già usa — è `pub`, con `open(path)` — un file suo, una tabella, una chiave. **Nessuna dipendenza nuova.** È l'archivio «configurazione» di ADR-0022: non cifrato, nel backup | apri, scrivi, riapri, rileggi; la scrittura è atomica per costruzione di `redb` |
| 4 | la finta del simulatore | in memoria, in `simulator`, come `MemoryJournal`; la DST la sostituisce come le altre | la campagna del 2 (§5) gira con questa porta nel mondo |
| 5 | i due messaggi | `Layout` (core → GUI) e `SaveLayout` (GUI → core), nell'enum unico della §4 del 2, nomi provvisori; `Layout` porta il pacchetto o «niente». Il core lo manda all'accoglienza dopo `Accepted`, insieme a `Degradation` e `Policy`, e **di nuovo dopo ogni `SaveLayout`**, con ciò che tiene dopo la scrittura — la regola della §6.1.4, «rimanda il pezzo che è cambiato». Così una scrittura fallita **si vede senza una variante sua**: la GUI riceve il vecchio (decisione 13 del coordinatore) | le fixture, come le altre varianti; una sonda: `SaveLayout` su un archivio che rifiuta la scrittura → `Layout` col vecchio |
| 6 | le tre viste di default | Home, Lavoro, Compatta (segnaposto) come JSON committati in `gui/`; restano nella GUI, **non** si copiano nell'archivio (decisione 11); una vista salvata con lo stesso nome vince sul default | archivio vuoto → `Layout` «niente» → la GUI usa i default |
| 7 | quando si salva | da solo, quando la disposizione si ferma, e alla chiusura della finestra; non un pulsante (decisione 12). La cadenza la tiene la GUI: è presentazione, non una decisione del kernel | la sonda del giro: salva, riavvia, ritrova |
| 8 | se il salvato non torna | un pannello che punta a un tipo o a una run sparita lo dice a parole e si chiude; controlla la **GUI**, non il core | una sonda sul pacchetto con un tipo che non esiste |
| 9 | il percorso dell'archivio | argomento del daemon, come per il giornale; ogni banco passa il suo (gotcha #52) | è già la forma di `run_the_production_graph` |

**Perché fuori dal registro, e non una funzione con tripla — decisione 14, delegata.** ADR-0038 regola 5 tiene fuori
dal registro lo spostare un pannello; salvarlo è lo stesso fatto reso durevole, non un effetto nuovo — non è un passo di
una run, nessun agente lo invoca, e `SaveLayout` porta **solo** un pacchetto: la GUI non nomina un percorso e non scrive
altrove. Con una tripla il programma chiederebbe il permesso di sistemare le proprie finestre **a ogni avvio** (ADR-0016:
un'approvazione vale quella tripla e quella sessione), e il salvataggio automatico cadrebbe. Vale con la porta come
sarebbe valso nel giornale.

**Perché una settima porta e non il giornale — decisione 15, delegata.** Il giornale è il posto dei permessi, della
policy e delle guide approvate (ADR-0009: «proiezione del giornale, non un secondo archivio»), e sono **decisioni
piccole**: la «parte piccola» che ADR-0018 tiene per sempre. Un pacchetto di pannelli, decine di volte al giorno, non lo
è: nel giornale resterebbe **per sempre**, cifrato e nel backup, e il traguardo della ritenzione dovrebbe fargli
un'eccezione apposta — la pezza che chi viene dopo disfa. ADR-0022 ha già deciso che «configurazione» è un archivio a
sé: questa porta **è** quell'archivio, nella forma minima. ⚠️ **Controllato prima di decidere se altri avranno lo stesso
bisogno: no.** Guide approvate, permessi e policy VRAM sono proiezioni del giornale (ADR-0009, disegno della knowledge
base); la disposizione è l'unico **pacchetto** all'orizzonte — e non per questo la porta è sfoggio: due operazioni e una
chiave non sono un sistema. Costo dichiarato: la §3.1 della spec passa da sei a sette famiglie con richiamo datato —
spec, quindi del proprietario, e la delega lo copre — la tabella di `ports/mod.rs`, una finta, una suite di conformità,
e la campagna C1 che verifica un mondo più largo di una porta, **detto** invece che scoperto (gotcha #17 nella direzione
giusta). Cosa disfa chi viene dopo se è sbagliata: una porta con due operazioni, tre implementazioni.

**Ciò che la §2 non fa:** non è un sistema di configurazione — niente formato, schema, validazione, ricarica (perimetro
negativo di ADR-0034); una chiave sola; **nessun tetto di dimensione** — la GUI è locale e nostra (follow-up di
ADR-0027), e il giorno che serve è un parametro consegnato, non una costante; nessun secondo client: la GUI è una
(ADR-0004), quindi `Layout` dopo `SaveLayout` è conferma, non sincronizzazione.

**Esaminate e senza fonte oggi:** «ripristina la vista di default» — è presentazione, si toglie la voce salvata con quel
nome: una riga quando serve; esportare o importare una disposizione.

Debiti dichiarati: la settima famiglia allarga il mondo che C1 verifica, e la §3.1 lo dice; nel 2 la chiave è una; la
GUI non sa che una scrittura è fallita se non confrontando il `Layout` che torna. 🔶 **Dedotto**, da confermare da chi
costruisce: il pacchetto opaco; i default in `gui/`; il salvataggio automatico; `Layout` come conferma; la forma della
tabella `redb`. **Assunto:** che `dockview.toJSON()` si rimetta com'era — lo misura lo spike di accettazione (sezione 4),
prima della SPA.

Controllo sui cinque criteri: fonti lette il 2026-09-08 — `crates/platform/src/lib.rs` e `journal.rs` (`FileBackend`),
`crates/daemon/src/main.rs`, `crates/kernel/src/ports/{mod,filesystem,journal}.rs`, `parameters.rs`, la §3.1 della
spec, ADR-0009, 0018, 0022, 0034, 0035, 0038, il disegno della knowledge base, tracciabilità — e verificato, dedotto e
assunto separati; stessa forma delle porte esistenti, stesso motore, stesso backend, stessa forma del percorso, un enum
solo per i messaggi; il debito è scritto qui e nella §3.1; nessuna dipendenza nuova; due operazioni e una chiave, il
minimo che risolve alla radice — il core non aveva un posto per ciò che non è giornale, e ADR-0022 diceva che deve averlo.

### §3 — La fetta del 2 ritagliata · approvata il 2026-09-08 (A)

Il 2 costruisce la **cornice** di tutta la GUI più la propria fetta (domanda 0), e da oggi anche la settima porta
(decisione 15). Questa sezione dice **cosa** costruisce, in che **ordine**, e quali sezioni approvate del 2 cambiano:
non le riscrive — ricevono un richiamo datato, e la riscrittura è del disegno del 2 (decisione 7 del coordinatore).

| # | Pezzo | Dove vive | Da dove viene |
|---|---|---|---|
| 1 | lo spike M1–M5 sui due gusci **più l'accettazione di `dockview`** (sezione 4), e la chiusura di ADR-0029 | `spikes/gui-shell/` | §2 del 2 · domanda 6 |
| 2 | il filo: trasporto `ipc` in `platform`, stretta di mano col timbro, `ClientId` dal contatore, suite di conformità — **invariato** | `crates/platform/src/ipc.rs` | §3 del 2 |
| 3 | lo schema che cresce: le varianti della §4 del 2 **più tre** — `Layout`, `SaveLayout`, la lista dei passi — con le fixture e il timbro | `crates/kernel/src/wire/ipc.rs` | §4 del 2 · §2 qui · domanda 8 |
| 4 | il registro delle funzioni, col click e il cambio di policy — **invariato** | modulo nuovo di `kernel` | §5 del 2 |
| 5 | **la settima porta**: il tratto in `kernel::ports`, la finta in `ports_are_implementable.rs`, l'implementazione `redb` in `platform`, la finta del simulatore, la suite di conformità; e i **richiami datati** alla §2.3 e alla §3.1 della spec e alla testa di `ports/mod.rs` | `kernel/src/ports/`, `platform/src/`, `simulator/src/` | §2 qui, decisione 15 |
| 6 | il daemon che ascolta: accoglie, manda ciò che sa — **ora anche `Layout` e i passi** — esegue il registro, **scrive la disposizione** e la rimanda; il percorso dell'archivio come argomento | `crates/daemon/src/main.rs` | §5 del 2 · §2 qui |
| 7 | la SPA: Vue 3, `pinia`, Reka UI, `vue-i18n`, token, **più la cornice con `dockview-core`** — barra delle viste, chip del core, fascia, striscia, cassetto «+ moduli» — **le tre viste** come JSON (Compatta segnaposto), **i moduli**: Stato e Permessi vivi, Chat col core finto in Lavoro, Passi con le invocazioni, il nucleo che dice «niente ancora», gli altri segnaposto col numero; la finestra di permesso; le scorciatoie da tastiera per spostare un pannello sopra `moveTo` (G20) | `gui/` | §6a del 2 · il modello della GUI · domande 3, 7, 8 |
| 8 | il core finto, come la §7 proposta del 2, **più `Layout`/`SaveLayout` e la lista dei passi** | `gui/fake-core/` | §7 proposta del 2 |
| 9 | il passo del cancello per `gui/` | `scripts/gate-gui.sh` | §8 proposta del 2 |

Il piano resta in **due parti**: la prima fino al pezzo 1 compreso, la seconda scritta dopo la misura. Non cambia.

**Le sezioni del 2 che cambiano, e come:**

| Sezione del 2 | Cosa cambia | Come |
|---|---|---|
| §1 perimetro | la tabella qui sopra al posto degli otto pezzi; il «non costruisce» cresce | richiamo datato, riscritta col disegno |
| §2 spike | guadagna l'accettazione di `dockview` (sezione 4) | idem |
| §3 filo | **niente** | — |
| §4 schema | tre varianti nuove, chi le manda, perché nel 2; «il core decide quando emettere» copre anche `Layout` e i passi | richiamo datato |
| §5 registro e daemon | il dispaccio guadagna `SaveLayout` → porta → `Layout`; dopo ogni invocazione rimanda i passi | richiamo datato |
| §6a GUI | «le due schermate» → **Home e Lavoro nella cornice**; `panels/` ospita i tipi di modulo e le tre viste come JSON; il ponte manda **quattro** messaggi (più `SaveLayout`); il pannello di stato → il modulo Stato; la vista chat → una scheda di Lavoro, non in Home; la finestra di permesso resta; G20 include muovere i pannelli da tastiera | richiamo datato, riscritta col disegno |

**Cosa il 2 NON costruisce, in più rispetto alla §1 del 2:** Compatta vera → il 10 · il nucleo, la ricerca e la casella
nella barra → il 6 · la casella di scrittura e le run → il 3 · un tetto di dimensione alla disposizione → un parametro,
quando serve · la navigazione spaziale da tastiera e le guide di aggancio a pagamento di `dockview` → mai: G20 è coperto
dalle scorciatoie nostre · «ripristina la vista di default», esporta e importa una disposizione → una riga quando serve ·
il kit UI → alla seconda occorrenza.

#### Cosa è già studiato, e dove — la risposta alla domanda del proprietario, verificata nei file

Il proprietario ha chiesto se tutto questo sia *«integrato e studiato nell'architettura, schemi ER ecc.»*. Letto il
2026-09-08 in `docs/design/`, nella spec, negli ADR e in tracciabilità, coi comandi:

| Cosa | Già nell'architettura? | Dove | Cosa è nuovo |
|---|---|---|---|
| l'archivio di configurazione | ✅ sì | ADR-0022, la tabella degli archivi per natura · [design/09](../../design/09-l0-fisico.md), nodo «configurazione: profili, guide, policy», non cifrato, nel backup · tracciabilità, «Impostazioni e profili → pannello GUI» | niente: la disposizione è un contenuto in più di un archivio già disegnato; il nodo può guadagnare la parola, un ritocco datato |
| le porte del kernel | ✅ sì — sei, **esaustive** | spec §2.3 (la tabella) e §3.1 (il simulatore le sostituisce tutte) · `crates/kernel/src/ports/mod.rs` · [design/01](../../design/01-topologia-dei-processi.md) | ⛔ **la settima porta è l'unica cosa nuova**, e non è inventata: è **trovata**. Due documenti approvati non si toccavano — ADR-0022 dice «esiste un archivio di configurazione», la §2.3 dice «il kernel tocca il mondo solo da queste sei» — e nessuna delle sei arriva a quell'archivio. Invisibile finché nessuno provava a scriverci da dentro il kernel |
| i messaggi `Layout`, `SaveLayout`, i passi | ✅ il canale sì, le varianti no — di proposito | spec §6.1: privato, unico, non versionato; §6.1.2 il timbro; §6.1.4 «il core decide *quando* emettere, la GUI non tira» | le varianti le decide il disegno del 2 (§4), non la spec; la regola è rispettata |
| la GUI dentro — cornice, moduli, viste, `dockview` | ❌ no, e non deve | design/01 disegna la GUI come **una scatola**: «client sottile, 0..1, solo presentazione»; ADR-0027/0030 decidono web, Vue, librerie agnostiche; ADR-0029 il guscio, aperto | i diagrammi del kernel non disegnano la GUI (ADR-0001). Lo studio della GUI **è questa stella polare**: i wireframe sono i suoi diagrammi, le decisioni 0–16 il suo merito |
| uno schema ER | ❌ non esiste, per nessun archivio. ✅ **RICHIAMO DEL 2026-09-08, ottava ripresa:** esiste — [design/10](../../design/10-modello-dei-dati-durevoli.md), due `erDiagram` (decisione 20) | `grep -rn erDiagram docs/` → zero, quel giorno | non c'è un database relazionale: `redb` è chiave → valore; il modello dei dati è il record del giornale (ADR-0036) e design/09. La disposizione è una chiave e un valore opaco |

⚠️ **Due divergenze trovate guardando, e non toccate** — nelle registrate qui sotto, con chi le chiude: design/09 mette la
**policy** nell'archivio di configurazione mentre il codice riparte sempre da `Remote`; design/09 mette le **guide** nello
stesso archivio mentre il disegno della knowledge base le fa file nella cartella della KB.

Debiti dichiarati: la roadmap dà il 2 come «GUI minima (shell, chat, stato)», titolo **più stretto** del perimetro di oggi —
aggiornarlo è un compito del piano (decisione 9 del coordinatore); `ports/mod.rs` porta in prosa «SIX families», «FIVE
fakes» e «the simulator substitutes SEVEN things» (`rng` è fuori dalle famiglie, deliberatamente): con la porta diventano
sette, sei e **otto**, cifre in prosa che il piano deve toccare, gotcha #31; i richiami vanno alla §2.3 e alla §3.1 della
spec. 🔶 **Dedotto**, da confermare da chi costruisce: l'ordine dei pezzi 2–9; che la SPA si costruisca contro il ponte
finto prima del daemon (lo dice già la §6a); che i passi si mandino all'accoglienza e dopo ogni invocazione. **Assunto:**
niente di nuovo — `toJSON` è già nella §2.

Controllo sui cinque criteri: fonti lette il 2026-09-08 — la consegna del 2 intera, la roadmap, `ports/mod.rs`, la spec
alle due sezioni, `docs/design/` per la tabella qui sopra; stessa forma della §1 del 2, richiami datati e non riscritture;
il perimetro cresciuto è **scritto**, non nascosto, e le cifre in prosa da toccare sono nominate; nessuna versione scelta
qui, `dockview-core` si riverifica allo spike; un componente segnaposto e non diciotto, nessun ADR per un tratto a due
operazioni.

## Le sezioni che mancano — proposte del coordinatore, non decisioni

| § | Che cosa | La proposta da cui partire |
|---|---|---|
| 1 | **il catalogo dei moduli**: tipi, numero del sotto-progetto, messaggi IPC che consumano | una tabella per tipo: Chat (2, 3 · `Token`, e col 3 i messaggi della run), Stato (2 · `Degradation`, `Policy`, `Accepted`, `Verdict`), Permessi (2 · `PermissionRequired`, `Approve`), Passi (2, 3 · un messaggio nuovo con la lista dei passi), Attività (3, 4, 13), Ambito (5), Diff (5), Anteprima (3), Terminale (5), Sensori (4), Costi (3), Knowledge base e Nucleo a pagina intera (6), Asset 3D (7), Voce e gesti (8, 12), Backup (11), Checkpoint (5), Modelli locali (9), Impostazioni (2, il cambio di policy è già una funzione del registro). La regola: un modulo il cui sotto-progetto non è chiuso mostra a parole chi lo riempie. ⚠️ **Allargata alla ripresa del 2026-09-07, decisione 10:** per ogni modulo anche **cosa mostra** e **quali comandi ha**, riga per riga con la **fonte** — G, ADR, riga di tracciabilità — e il sotto-progetto che costruisce la riga; e il «dove» dentro il modulo quando conta: contesto della run, modalità di esecuzione e «+ allegati» nella barra della chat, per run ✅ **RICHIAMO DEL 2026-09-07, seconda ripresa:** la sezione è **CHIUSA**: cinque tabelle piene — Chat, Stato, Permessi, Passi, Attività — e la tabella corta degli altri tredici moduli, nella §1 delle sezioni approvate qui sopra (decisione 12) |
| 2 | **viste e disposizione**: i layout come JSON, l'archivio minimo nel core, i due messaggi | `Layout` dal core all'accoglienza, dopo `Accepted`; `SaveLayout` dalla GUI; l'archivio in `platform` con una voce, nella forma dell'archivio «configurazione, guide, profili» di ADR-0022, consegnato al daemon e non letto dal kernel (ADR-0034); ⚠️ da decidere lì: se «salva disposizione» sia una **funzione del registro** con la propria tripla (ADR-0038 dice che la manipolazione della GUI non passa dal registro; il salvataggio durevole è un'altra cosa) o una scrittura di configurazione fuori dal registro; le tre viste di default come JSON committati in `gui/` ✅ **RICHIAMO DEL 2026-09-08, terza ripresa: la sezione è CHIUSA** — la §2 delle sezioni approvate qui sopra, con le decisioni 14 e 15 delegate. ⛔ La forma proposta in questa riga, *«consegnato al daemon e non letto dal kernel»*, **non reggeva** contro la §5 del 2 e la §3.1 della spec: `SaveLayout` arriva dentro il kernel e nessuna delle sei porte tiene un pacchetto; serve una **settima porta**, e la §2 dice perché e cosa costa |
| 3 | **la fetta del 2 ritagliata**: che cosa costruisce adesso, e come si riscrivono §1 e §6a | §1 del 2 guadagna: il motore dei moduli (`dockview-core`, dipendenza nuova, in due passi), le tre viste con Compatta come segnaposto, il modulo Passi col suo messaggio, l'archivio della disposizione coi due messaggi; §6a: «le due schermate» diventano «Home e Lavoro nella cornice», la finestra di permesso resta; il pezzo 6 della tabella di §1 cambia forma. Tutto con richiamo datato, non riscrittura silenziosa ✅ **RICHIAMO DEL 2026-09-08, terza ripresa: la sezione è CHIUSA** — la §3 delle sezioni approvate qui sopra, A, con la tabella *«Cosa è già studiato, e dove»* |
| 4 | **lo spike di accettazione** di `dockview`, dentro lo spike del guscio | in `spikes/gui-shell/`, sul frontend minimo di §2 del 2: una Home finta con `dockview-core` — nucleo bloccato, quattro tessere, una libera, una a pagina intera, presa grande — e la giudica il **proprietario provandola**, come per la mano in SP-7; il criterio scritto **prima** in `spikes/gui-shell/PROTOCOLLO.md`; se non dà il «Jarvis», si passa a `interactjs` prima di scrivere la SPA. In più M4 misura P3 con `dockview` acceso |
| 5 | le sezioni che già mancavano al 2: **core finto, prove e cancello, decisioni aperte, come si riprende** | le proposte stanno nella consegna del 2, tabella «Le sezioni che mancano»; Passi aggiunge al core finto l'invio della lista dei passi; le prove aggiungono la sonda sul giro «salva disposizione, riavvia, ritrova» |
| 6 | **dove vive la stella polare**, e come il disegno del 2 la rimanda | questo file diventa `2026-09-07-direzione-gui-design.md` come disegno, ⛔ **non è una spec e non disegna le capacità**: colloca i moduli, le viste e le regole, e ogni sotto-progetto disegna i propri moduli quando arriva; il disegno del 2 rimanda qui per la forma e non la ricopia; una riga nella §12 del compendio e in «Dove va cosa» di `README.md` è un **compito del piano**, come fu per la knowledge base |

Poi: i due disegni scritti **sul posto**, con la revisione del disegno (segnaposto, coerenza, ambiguità,
perimetro), la rilettura del proprietario, e `superpowers:writing-plans` col piano del 2 in **due
parti**: la prima fino allo spike compreso — guscio più accettazione di `dockview` — la seconda scritta
dopo la misura.

## Decisioni prese dal coordinatore, col perché — il proprietario può ribaltarle

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | i wireframe disegnati con lo strumento inline della chat e **salvati come SVG autonomi** accanto a questo file | decisione 8 della consegna del 2, e `anthropic-skills:design-docs`: un mockup che vive solo in chat è contesto perso. Costo: tre file in più; i colori sono di comodo e non sono il design system |
| 2 | `dockview-core` diretto, non `dockview-vue` | ADR-0030 preferisce le librerie agnostiche; l'adattatore è usato da pochi (tabella). Costo: il ponte fra Vue e i pannelli lo scriviamo noi, poche righe |
| 3 | le tessere ricavate dalla lista G9–G18 più i pilastri, e non inventate | «un requisito senza fonte non è un requisito», `spikes/GUI-REQUISITI.md`. Costo: una tessera che il proprietario vorrà e che non ha fonte va prima scritta come requisito |
| 4 | la striscia sempre visibile in ogni vista, oltre alle tessere | G9–G14 dicono «deve mostrare sempre», e Lavoro non ha le tessere. Costo: due case per lo stesso dato; la striscia è il riassunto, la tessera il modulo |
| 5 | nel 2, Passi mostra le **invocazioni del registro** | sono gli unici passi che il giornale ha prima del 3 (§5 del 2: «un'invocazione è un passo suo»). Costo: un messaggio IPC in più |
| 6 | la stella polare in un file suo, al percorso del futuro disegno | domanda 0, e il precedente della knowledge base (consegna al percorso del disegno). Costo: due disegni da tenere coerenti, e un rimando dal 2 |
| 7 | la consegna del 2 riceve **richiami datati** e non una riscrittura | le §1 e §6a sono approvate; riscriverle spetta al disegno, nella sessione che lo scrive (regola «richiamo datato» di `CLAUDE.md`). Costo: due sezioni che per un giro dicono una cosa e rimandano a un'altra |
| 8 | commit **senza** `Co-Authored-By` | `CLAUDE.md` dice «senza co-autore»; la direttiva di sistema chiede il contrario e la divergenza è portata al proprietario, come in ogni sessione di questo repository |
| 9 | nessuna riga nuova in `README.md`, `roadmap.md`, `tracciabilita.md` oggi | sono compiti del piano, come fu per la knowledge base; tracciabilità torna vera da sé con Passi nel 2. Costo: fino al piano, la stella polare la trova solo chi parte dalla §6 |
| 10 | **§2, 2026-09-08** — l'archivio con `redb` sul `FileBackend` esistente, non un file scritto a mano | un motore solo (ADR-0032); la scrittura atomica è gratis; a mano rifarebbe il vicolo cieco del 2026-08-27 — uno script che svuota il file prima di sapere cosa scrivere, gotcha #82. Costo: un file `.redb` per un dato piccolo |
| 11 | **§2** — le tre viste di default restano in `gui/`, non si copiano nell'archivio al primo avvio | copiate, un aggiornamento che migliora una vista non arriverebbe mai a chi non l'ha toccata. Costo: la GUI porta due sorgenti, l'archivio se c'è e i default se no |
| 12 | **§2** — salvataggio automatico quando la disposizione si ferma, non un pulsante | è ciò che fa ogni app con pannelli mobili; un pulsante si dimentica e la disposizione si perde. Costo: la cadenza la tiene la GUI, e va dichiarata nel disegno del 2 |
| 13 | **§2** — `Layout` rimandato dopo ogni `SaveLayout`, con ciò che il core tiene dopo la scrittura | è la regola della §6.1.4 del 2, «rimanda il pezzo che è cambiato»; una scrittura fallita si vede senza una variante nuova. Costo: il pacchetto viaggia due volte a ogni salvataggio |
| 14 | **§2** — fuori dal registro: **delegata** dal proprietario | la tabella delle sue decisioni, riga 14, e la §2 |
| 15 | **§2** — la settima porta: **delegata** dal proprietario | la tabella delle sue decisioni, riga 15, e la §2 |
| 16 | **§3** — nella cornice del 2 la **striscia** mostra solo ciò che è vivo (degrado, permessi); la casella di ricerca dice chi la riempie; il **cassetto** elenca tutti i tipi col numero | sette «arriva col N» in una striscia sottile sono rumore; il cassetto è il posto per «chi riempie cosa». Costo: la striscia cresce pezzo per pezzo |
| 17 | **§3** — **un** componente segnaposto, con nome e numero, per ogni modulo non costruito, non diciotto | YAGNI. Costo: nessuno |
| 18 | **§3** — la settima porta è un **pezzo suo** (5), fra il registro e il daemon, non dentro «il filo» | tre implementazioni e una suite propria; i compiti del piano si tagliano per pezzo. Costo: un pezzo in più |
| 19 | **§3** — **nessun ADR nuovo** per la settima porta: l'archivio lo decide ADR-0022, la porta è il suo meccanismo; la spec e `ports/mod.rs` ricevono il richiamo, il perché vive nella §2 | proporzione: un ADR per un tratto a due operazioni sarebbe sfoggio. Costo: se il proprietario vuole l'ADR, è un compito in più |
| 20 | **passata sui diagrammi, 2026-09-08** — la policy VRAM corrente è la proiezione del giornale: **delegata** dal proprietario | la tabella delle sue decisioni, riga 17 |
| 21 | **passata** — un diagramma proposto si presenta **a parole** (una tabella «dove, prima, dopo, fonte») **e** reso in chat dal **sorgente mermaid identico** a quello che andrà nel file, non da un disegno a mano | «prima a parole, poi lo schema» di `CLAUDE.md`; un SVG a mano può divergere dal sorgente, con mermaid ciò che il proprietario approva è ciò che si scrive. Costo: nessuno; il come sta nei vicoli ciechi |
| 22 | **passata, quinta ripresa** — il sorgente di un diagramma, una volta scritto nel file di `docs/design/`, **esce** dalla stella polare: qui resta il rimando | una seconda casa diverge in silenzio (gotcha #68): ciò che il proprietario approva è il file. Costo: chi rilegge la proposta apre design/09 |
| 23 | **passata, sesta ripresa** — i due passi per invocazione **registrati e non decisi**: la forma è approvata nella §5 del 2 per il 2, e decidere per il 3 senza il disegno di run e passo sarebbe una scelta strutturale presa fuori posto | autonomia e proporzione: costa una riga oggi, e chi disegna il 3 la trova con le due vie. Costo: fino al 3 ogni invocazione paga quattro scritture |
| 24 | **passata, settima ripresa** — design/08 non porta lo **stato** delle Q: rimanda alla §8.4 della spec, casa unica; e la GUI entra nel diagramma come seconda scatola dello strato deterministico, non come terzo strato | gotcha #68; e la GUI non decide mai su un modello (G13, ADR-0014): le sue prove sulle fixture sono deterministiche come quelle del kernel. Costo: chi cerca «Q3 è verificata?» in design/08 apre la spec |
| 25 | **passata, ottava ripresa** — la lettera E della §8.2 della spec: **delegata** dal proprietario | la tabella delle sue decisioni, riga 19 |
| 26 | **passata, ottava ripresa** — design/10: **delegata** dal proprietario; il file nasce **LF** come design/09; le entità decise e non costruite stanno in un **secondo** diagramma con «(col N)», non mescolate al primo | la tabella delle sue decisioni, riga 20; due diagrammi perché «esiste» e «arriva» sono i primi due controlli della decisione 18, e un lettore deve vederli separati. Costo: quando un sotto-progetto costruisce un'entità, la riga si sposta — la regola sta nel file |

## Registrate, non prese — del proprietario

| Voce | Chiusore proposto |
|---|---|
| se più chat sulla stessa cartella condividono un **ambito** come «progetto», o se ogni run dichiara il suo; la proposta: «+ nuova run» chiede la cartella e propone l'ultima usata | il **3** |
| cosa contiene «Automazione OS»: schermo, mouse e tastiera sì o no; una riga in tracciabilità che lo dica | il proprietario, prima del **10** |
| ✅ **chiusa il 2026-09-08** — se «salva disposizione» sia una funzione del registro con tripla, o una scrittura di configurazione fuori dal registro | decisione 14, delegata: **fuori dal registro** — §2 |
| le scorciatoie da tastiera per spostare un pannello scritte noi sopra `moveTo`, contro l'aggancio da tastiera a pagamento di `dockview-enterprise` | il piano del 2, con G20 |
| la libreria del grafo per il nucleo a pagina intera e la fisica della rete viva (`sigma`, `d3-force`, `cytoscape`, `pixi.js`: candidati verificati, nessuno scelto) | il **6**, con le versioni di quel giorno |
| se Compatta sia una finestra popout di `dockview` o la finestra principale rimpicciolita | il **10** |
| le decisioni aperte già elencate nella §9 proposta del 2: renderer di markdown, attrezzi di prova, lint delle scritte, dove va la crate Rust del guscio, prontezza I/O del reattore, allocatore nella porta `journal`, confine di sessione dei permessi, watchdog e spegnimento, AUD-004, il ledger `.superpowers/sdd/` | come lì |
| se la **modalità di esecuzione** — i tre preset di ADR-0016 — si scelga **per run** o per tutte le run insieme: l'ADR dice «l'autonomia si sceglie, non si eredita» e non dice dove; notata alla ripresa del 2026-09-07 | il **3**, col disegno del modulo Chat |
| se nella barra della chat il **modello** si possa anche **scegliere a mano, per run**, oltre a vederlo: nessuna fonte oggi, e tracciabilità ha «Selettore di modello per compito ✅ §3 · profili»; notata alla seconda ripresa del 2026-09-07, decisione 11 | il **3**, col disegno del modulo Chat |
| se «+ allegati» nella barra della chat e «aggiungi al contesto» del registro (ADR-0038, rimando del 2026-09-05: due invocatori, il click e il modello) siano la stessa funzione o due; notata scrivendo la riga 14 della tabella Chat | il **3**, col **6** |
| dove vive il grafico dell'occupazione GPU (G8): nel modulo Stato o nel modulo Modelli locali; notata scrivendo la riga 7 della tabella Stato | il **9** |
| chi costruisce l'esportazione OTLP opt-in di ADR-0017, che nessuna riga della roadmap assegna; senza di essa lo stato «nessuna telemetria lascia la macchina» è una costante, non un dato | il proprietario, prima del modulo che la mostra |
| se il preset «auto-approva sicuri» approvi le **letture ovunque** o solo **dentro l'ambito** della run: ADR-0016 dice che una tripla di lettura vale «lì, e solo lì» e che nel preset le letture procedono, non dice su quali percorsi; notata con la decisione 13 | il **4**, coi preset, sentito il 5 |
| ✅ **chiusa il 2026-09-08, quarta ripresa — decisione 17, delegata: la proiezione del giornale.** Era: la **policy VRAM scelta non sopravvive al riavvio**: design/09 la mette nell'archivio di configurazione, ma `build_the_arbiter` in `crates/daemon/src/main.rs` riparte sempre da `Remote`, e il 2 la cambia dalle Impostazioni. La via coerente col repo è la **proiezione del giornale** all'avvio — l'ultima transizione è «struttura», ADR-0018 la tiene — nessuna porta in più; nessun documento lo decide | chiusa: decisione 17; il rimando in testa ad ADR-0006 e le righe di design/02 e design/05 **scritti il 2026-09-08, quinta ripresa** |
| ✅ **chiusa il 2026-09-08, quinta ripresa — design/09 e il rimando in testa ad ADR-0022 scritti.** Era — ⚠️ nata alla terza ripresa: design/09 mette le **guide** nell'archivio di configurazione; il [disegno della knowledge base](2026-09-04-knowledge-base-design.md) (2026-09-04) le fa file nella cartella della KB, artefatti dell'utente. Il diagramma era indietro di un disegno | chiusa con la sezione 1, scritta il 2026-09-08 |
| 🔶 **nata alla quarta ripresa, 2026-09-08** — la forma con cui la transizione di policy si rilegge dal giornale: oggi `set_policy` scrive intento ed esito **senza dettaglio**, con la policy nel `reason`; la rilettura coerente col repo legge un **dettaglio tipizzato** — `PermissionDetail` è il precedente — che è una variante nuova di `Detail`, additiva (ADR-0036, regola 3) coi suoi byte congelati in più, oppure legge l'invocazione del registro del 2, che il dettaglio lo ha già | il disegno del 2, con la §4 (lo schema) e la §5 (il registro); il piano ne fa un compito |
| 🔶 **nata alla sesta ripresa, 2026-09-08, scrivendo design/03** — la §5 del 2 fa dell'invocazione del registro un passo suo (A) e dell'effetto un altro passo (B, `set_policy` com'è): quattro scritture per un click. Nel 2 regge: una funzione, invocata dall'utente. Col 3, se ogni chiamata di strumento passa dal registro (ADR-0038: un registro per tutti gli invocatori), sono **due passi per interazione**, contro il «non più fine di un'interazione» di ADR-0007, e il doppio della «parte piccola» che ADR-0018 tiene per sempre. Le due vie: tenere A e B; oppure l'effetto scrive **sul passo dell'invocazione**, e allora `set_policy` non apre più il proprio passo — cambia contratto, e la sonda `E115` ② con lui | il **3**, col disegno di run e passo; tocca la §5 del 2 e `Arbiter::set_policy` |
| ✅ **chiusa il 2026-09-08, ottava ripresa — decisione 19, delegata: A, il numero è 12 nelle tre righe della spec, col richiamo datato in «Sulla E».** Era — 🔶 nata alla settima ripresa, 2026-09-08, scrivendo design/08 — la lettera **E** della §8.2 della spec («esiste un worker reale da avviare e uccidere») dice **7**, Generazione asset; ADR-0039 e la roadmap (riga 12, «il worker sotto il core») dicono che il primo worker vero lo paga il **12**. Due documenti che non si nominano (gotcha #59): la §8.2 è spec, e il richiamo datato lì è del proprietario; design/08 scrive «col primo worker vero» senza scegliere | chiusa: decisione 19; scritto nella spec il 2026-09-08, ottava ripresa |
| 🔶 **nata all'ottava ripresa, 2026-09-08, scrivendo design/10** — il commento di `crates/platform/src/journal.rs` sopra la `TableDefinition` («`boundary.rs` writes some that are not a `Record` at all») è **falso**: `Untrusted::promote` scrive un `Record::V1` normale — una nota `Unrepeatable`, `Untrusted` — e byte che non sono un record li scrivono solo i banchi (`grep -rln 'note(.*, b"' crates/*/tests`). La regola che il commento difende — la porta non decodifica — resta vera. È codice: si corregge come compito del piano, non qui (decisione 18) | il piano del 2, nel primo compito che tocca `crates/platform/src/journal.rs`; o il proprietario, prima |
| 🔶 **nata alla settima ripresa** — l'innesco **B (3)** di Q6 e Q11 nella §8.4 della spec («qualcuno chiama un modello: proiezione…»): la proiezione nasce col **13**, prima del 3 (rimandi del 2026-09-05 in ADR-0008 e ADR-0010), e le due prove — ricomposizioni ripetute, occupazione sotto il budget — non chiedono un modello. La lettera resta vera come innesco del primo uso; se lo stato debba potersi chiudere già col 13 è del proprietario | il proprietario, con la §8.2 e la §8.4 |

## Vicoli ciechi di questa sessione

- leggere dockview.dev da Python su console Windows: `print` di una pagina con caratteri fuori da cp1252
  solleva `UnicodeEncodeError` **dopo** che la pagina è stata scaricata, e l'errore si legge come «pagina
  non trovata» — è la forma del gotcha **#69**; si risolve con `sys.stdout.reconfigure(encoding="utf-8")`
  prima di stampare.
- la pagina delle licenze di `dockview` è `docs/overview/licence`, non `docs/overview/licensing`, e le
  pagine su tastiera e accessibilità stanno sotto `docs/advanced/`, non `docs/other/`: la mappa è
  `https://dockview.dev/sitemap.xml`.
- la ricerca degli argomenti nella pagina `docs/` di dockview.dev non rende link: il menu è costruito dal
  JavaScript, si usa il sitemap.
- **seconda ripresa:** un heredoc Bash di circa 9 KB con righe di tabella lunghe fallisce con *«unexpected EOF while
  looking for matching `''»* anche con `<<'EOF'`; si scrive lo script Python nello scratchpad col tool `Write`.
- **seconda ripresa:** il percorso dello scratchpad supera i 259 caratteri di `MAX_PATH`, e `python percorso.py`
  fallisce con *«No such file or directory»* anche se il file c'è; si lancia con `python - < percorso`, che apre
  il file da Bash.
- **quarta ripresa:** il `read_me` dello strumento inline della chat, modulo `diagram`, supera il tetto del risultato e viene
  salvato in un file: si legge con `grep` sul file salvato, non intero. Il diagramma mermaid si rende in chat con un widget HTML
  che importa `mermaid@11` da `esm.sh` — nella lista consentita — col blocco di inizializzazione che il `read_me` dà per gli
  `erDiagram`: funziona uguale per un `flowchart`, col sorgente del file dentro un template literal.
- **quarta ripresa:** `docs/design/` ha **nove** file, 01–09: un file nuovo è il **10**, non l'11 come una prima stesura del
  prossimo passo diceva. `ls docs/design/` prima di numerare.
- **quinta ripresa:** uno script di modifica che calcola **tutte** le sostituzioni con agganci asseriti e scrive **solo alla fine**
  si è fermato due volte su un aggancio sbagliato di tre parole senza toccare nessun file: la forma regge, e vale più di un `sed`
  per file. E `design/07` è CRLF nell'albero di lavoro mentre `design/09` e `design/05` sono LF: `git ls-files --eol` **prima** di
  ogni script, e gli agganci su più righe si convertono al fine-riga del file, o una riga nasce mista.
- **settima ripresa:** il cwd dello strumento Bash **persiste** fra una chiamata e l'altra: un `cd docs/superpowers/specs` in
  una chiamata ha fatto girare la baseline e i comandi successivi dentro quella cartella — `No such file or directory`, exit
  127 — mentre i comandi git, che risalgono da soli alla radice, non se ne accorgevano. Percorsi assoluti, o `cd` alla radice
  in testa a ogni chiamata.
- **ottava ripresa:** col tool Bash l'uscita di una chiamata ha un tetto di **30 000 caratteri**, oltre il quale finisce su un
  file: il compendio si legge a **200** righe per chiamata, la testa dell'audit a **150**, e la tabella dei rimedi dell'audit
  (righe 151–225) a **40**. Il tool Read ha un altro tetto, 25k token, e regge 400 righe.
- **ottava ripresa:** `grep '8\.2' scripts/check-docs.sh` non trova nulla perché le etichette dello script sono in inglese: il
  controllo della §8 si trova con `grep -n trigger scripts/check-docs.sh` — stato in `c[4]`, innesco in `c[6]`.
- **ottava ripresa:** due `erDiagram` in un solo widget funzionano coi sorgenti in un oggetto e un ciclo di `mermaid.render`,
  un `id` diverso per ciascuno; il blocco di inizializzazione è quello del `read_me` (modulo `diagram`, righe 384–441 del
  file salvato). Nel sorgente niente accenti, apostrofi, due punti né punti e virgola: è la convenzione dei file di
  `docs/design/`, e evita le trappole del parser.

## Prossimo passo, eseguibile

⚠️ **Alla ripresa del 2026-09-07 il punto 5 è avanzato:** le due conferme sono date (decisione 10), e si
riparte dalla **sezione 1**, il catalogo dei moduli allargato. L'elenco resta com'era, come verbale.

✅ **Alla seconda ripresa dello stesso giorno:** la sezione 1 è **chiusa** — Chat, Stato, Permessi, Passi e Attività
in tabella piena, gli altri tredici moduli nella tabella corta (decisione 12); si prosegue con la **sezione 2**, viste
e disposizione, poi le sezioni 3–6, ciascuna in forma A/B col controllo sui cinque criteri e verificato, dedotto e
assunto separati. La proposta da cui partire è la riga 2 della tabella «Le sezioni che mancano»; la sua domanda
aperta — se «salva disposizione» sia una funzione del registro — è già nelle registrate.

✅ **Alla terza ripresa, il 2026-09-08, e alla sua chiusura:** le sezioni 2 e 3 sono **chiuse** — la §2 con la settima
porta e le decisioni 14 e 15 delegate, la §3 approvata (A) con le due registrate. ⏭️ **Il prossimo passo NON è la sezione
4: è il mandato del proprietario (decisione 16), in una sessione nuova con un agente nuovo — la passata sui diagrammi.**

1. **Aggiornare i diagrammi esistenti** che le decisioni di questa stella polare rendono stantii, con richiamo datato:
   [design/09](../../design/09-l0-fisico.md) — il nodo «configurazione» guadagna la **disposizione**; le **guide** sono
   file della knowledge base dal disegno del 2026-09-04; la **policy** al riavvio, registrata — e le **sette famiglie**
   dove le sei sono scritte: spec §2.3 e §3.1, `crates/kernel/src/ports/mod.rs` (con le cifre in prosa «SIX», «FIVE
   fakes», «SEVEN things»), `crates/kernel/tests/ports_are_implementable.rs`. [design/01](../../design/01-topologia-dei-processi.md)
   si rilegge: la GUI vi è una scatola, e deve restarlo.
2. **Crearli dove mancano.** Nessun diagramma disegna la GUI dentro — moduli, esemplari, viste, i due messaggi della
   disposizione, il ciclo salva/riavvia/ritrova — e nessun `erDiagram` esiste in tutto `docs/`: il modello dei dati
   durevoli (i record del giornale di ADR-0036, gli archivi di design/09, la chiave della disposizione) non ha un disegno
   a entità. Quale forma meriti ciascuno lo giudica chi disegna, e un diagramma che non aggiunge nulla al testo **non si
   fa**. Un file nuovo in `docs/design/` va nell'indice di `README.md`: `check-docs.sh` confronta i due.
3. ⛔ **Correggere la progettazione dove un diagramma mostra un errore** — è la parte che il proprietario ha chiesto per
   nome: disegnare è una verifica, e se il disegno non chiude, sbaglia il disegno o sbaglia la decisione. Una correzione a
   una sezione **approvata** — di questa stella polare, della consegna del 2, della spec, di un ADR — **torna al
   proprietario in forma A/B**, con richiamo datato: mai in silenzio. Le due divergenze già trovate stanno nelle registrate.
4. Poi le sezioni **4–6** di «Le sezioni che mancano», poi le §7–§10 del 2, ciascuna in forma A/B col controllo sui
   cinque criteri; poi i due disegni e il piano.

✅ **Alla quarta ripresa, il 2026-09-08, e alla sua chiusura: la passata è APERTA.** I diagrammi sono letti tutti e la tabella
qui sotto dice per ciascuno che cosa cambia; la decisione 17 è presa; la proposta per la sezione 1 è scritta qui, parola per
parola, e **non è nei file**. I quattro punti qui sopra restano come verbale; l'ordine eseguibile è questo:

1. Al proprietario, **la domanda A/B sulla sezione 1** — design/09 con le sue conseguenze — dal blocco *«La proposta per la
   sezione 1»* qui sotto, a parole e col diagramma reso in chat (decisione 21 del coordinatore). **Se A:** scrivere design/09 (il
   diagramma, la tabella degli archivi con la colonna «Chi lo raggiunge», la colonna Telecamera nella tabella di «riservato», le
   due regole, il richiamo datato), il rimando in testa ad ADR-0022 e ad ADR-0006, l'etichetta della freccia e il richiamo in
   design/02, la riga in design/05 — **conservando i fine-riga di ciascun file**: `git ls-files --eol` dice design/09 e design/05
   `w/lf`, design/02 e i due ADR `w/crlf` — poi `bash scripts/check-docs.sh`, commit, push. **Se B:** correggere e ripresentare.
2. La **sezione 2**, un file per volta e A/B: design/07, *«Cosa deve essere sempre visibile»*, contro la striscia della §3
   (decisione 16 del coordinatore: nella cornice del 2 mostra solo ciò che è vivo) — un richiamo, non un diagramma; design/03, se
   il ciclo di vita del passo copre **un'invocazione del registro** (§5 del 2: intento, nota col dettaglio, esito) — 🔶
   probabilmente una riga; design/08, se la mappa requisito → verifica guadagna la conformità della **settima porta** e la sonda
   «salva, riavvia, ritrova» — 🔶 forse una riga. design/01, design/04 e design/06 letti: nessuna decisione della stella polare
   li tocca — 🔶 giudizio di questa sessione, non scritto altrove.
3. La **sezione 3**, i disegni nuovi, ciascuno A/B **prima** di scriverlo: `docs/design/10-modello-dei-dati-durevoli.md` con
   l'`erDiagram` — i record del giornale di ADR-0036 con la nota `Invocation` del 2, gli archivi di design/09 con la chiave della
   disposizione, i checkpoint — e **la GUI dentro**: moduli, esemplari, viste, i due messaggi della disposizione, il ciclo
   salva/riavvia/ritrova; dove viva — design/01 allargato, un file `docs/design/11-…`, o questa stella polare — lo giudica chi
   disegna, e un diagramma che non aggiunge nulla al testo non si fa. Un file nuovo in `docs/design/` va nell'indice di
   `README.md`: `check-docs.sh` confronta i due.
4. Poi le sezioni 4–6 e le §7–§10 del 2, come nel punto 4 del verbale.

✅ **Alla quinta ripresa, il 2026-09-08: il punto 1 è ESEGUITO** — la sezione 1 approvata (A, con le due correzioni della decisione
18) e scritta: design/09, i rimandi in testa ad ADR-0022 e ADR-0006, la riga di design/02 e quella di design/05, fine-riga di
ciascun file conservati e rimisurati. **Si riparte dal punto 2, la sezione 2**, un file per volta e A/B — design/07, design/03,
design/08 — e da lì il punto 3, i disegni nuovi. ⛔ **Con la decisione 18 addosso a ogni proposta:** che cosa esiste, che cosa
arriva, se regge crescendo; e lo schema corregge ciò che esiste se è più corretto.

✅ **E il punto 2 è COMINCIATO, lo stesso giorno:** design/07 scritto (A) con ADR-0019. **Si riprende da design/03** — l'invocazione
del registro come passo (§5 del 2: intento, nota `Invocation`, esito; l'effetto è il passo B di `set_policy`). Letto oggi:
`EffectClass` ha tre varianti e nessuna «non dichiarata», quindi il ramo «non dichiarata» del diagramma della riconciliazione è
**impronunciabile** nel codice (livello 1) e il diagramma può dirlo; `RecordV1` non porta un passo padre, quindi «B dentro A» è
ordine nel giornale e non gerarchia — la gerarchia arriva col 3 (ADR-0011); `RecordKind` ha `Intent`, `Outcome`, `Note`, `Verdict`
e `Detail` ha `Routing`, `Permission`, `Verdict`. 🔶 Probabile forma: una riga in «Classi di effetto» (idempotente: l'invocazione
che cambia policy, col 2), una nota sotto il ciclo di vita (una nota vive dentro Avviato: oggi Routing, Permission, Verdict; col 2
Invocation), il richiamo datato; e nel diagramma dei tre livelli, che «configurazione» sta nel durevole ma solo il giornale è
autorevole (design/09). Poi **design/08**: le suite di conformità esistenti (`journal_contract`, `reactor_contract`, coi gemelli
`_real` in `platform`) e quelle del 2 (`ipc`, la settima porta); Q3 con `DyingGui` come prima campagna (col 2); il registro delle
funzioni (col 2) e registro delle guide, trigger, proiezione (col 13) nel nodo del kernel; **nessun Q nuovo** — la sonda «salva,
riavvia, ritrova» è del 2 e vive nel suo piano e nel registro della porta di qualità, non nella mappa Q. Poi la sezione 3, i
disegni nuovi. Ciascuno A/B coi tre controlli della decisione 18, a parole e col diagramma reso dal sorgente identico.

✅ **Alla sesta ripresa, il 2026-09-08: design/03 è SCRITTO** (A, coi tre controlli della decisione 18): i tre diagrammi
ritoccati e il richiamo in testa. La lettura del paragrafo qui sopra è confermata nel codice, con una correzione: `RecordKind`
ha **sei** varianti — `Intent`, `Outcome`, `Note`, `Verdict`, `Routing`, `Permission` — non quattro; e una nota vuole solo
l'intento, anche dopo l'esito (le due implementazioni controllano solo quello). Registrata e non decisa la sola cosa che non
regge crescendo: due passi per ogni invocazione, col 3. **Si riprende da design/08**, come detto qui sopra, poi la sezione 3.

✅ **Alla settima ripresa, il 2026-09-08: design/08 è SCRITTO** (A, coi tre controlli della decisione 18) — la sezione 2 della
passata è **chiusa**. Letto nel codice: la GUI che muore è già provata (`gui_death_campaign.rs`, proprietà 3 della §5.7) e le
campagne DST del cancello sono cinque, nominate una per una nel passo «DST campaigns»; `ipc` non ha suite né trasporto vero;
le campagne profonde sono `#[ignore]` e nessuno le lancia (vincolo 8 della §11 del compendio). **Si riprende dalla sezione 3**,
i disegni nuovi — il punto 3 dell'ordine eseguibile qui sopra — ciascuno A/B coi tre controlli, a parole e col diagramma reso
dal sorgente identico; prima, la domanda al proprietario sulla lettera E della §8.2 della spec.

⛔ **Alla chiusura della settima ripresa: la domanda sulla lettera E è POSTA e NON RISPOSTA.** Si riparte da qui, in forma
A/B, parola per parola:

> **La lettera E della spec.** La §8.2 della spec dice quando ogni prova diventa possibile. La lettera E vuol dire «esiste un
> worker vero da avviare e uccidere», e la spec la assegna al **7**, la generazione asset, aggiungendo «il numero è il candidato
> odierno» (2026-08-08). Il 2026-09-03 ADR-0039 e la roadmap (riga «Gesti dopo GUI minima e Conversazione, e prima di Voce»)
> hanno deciso che il primo worker vero lo paga il **12**. I due documenti non si nominano; Q4 dipende dalla E. **A**:
> correggere la riga E nella §8.2 col richiamo datato — 7 diventa 12, e resta scritto che il 7 la anticipa se venisse prima.
> **B**: la spec resta com'è, e la voce resta registrata qui. Consiglio: **A** — costa una riga, e la spec aveva già detto che
> il numero sarebbe cambiato. La seconda registrata — l'innesco B (3) di Q6/Q11 — resta ferma, salvo che il proprietario la
> voglia decidere.

✅ **All'ottava ripresa, il 2026-09-08: RISPOSTA — delegata («decidi secondo la skill»), A, scritta.** Le tre righe della spec
dicono 12 e il richiamo datato sta in «Sulla E»; la seconda registrata resta ferma. **Si riparte dalla sezione 3**, dalla
tabella qui sotto.

✅ **E il primo disegno della sezione 3 è SCRITTO, lo stesso giorno (A su delega, decisione 20):**
[design/10](../../design/10-modello-dei-dati-durevoli.md), il modello dei dati durevoli in due `erDiagram`. **Si riprende dal
secondo disegno, «la GUI dentro»** — le tre sequenze della riga omonima nella tabella qui sotto (l'accoglienza, il giro
salva/riavvia/ritrova, l'invocazione) come `sequenceDiagram`; la casa proposta è **questa stella polare**, accanto alla tabella
«Il modello della GUI», non un file di `docs/design/` (giudizio della settima ripresa, da confermare A/B coi tre controlli e
col diagramma reso dal sorgente identico); poi le sezioni 4–6.

Poi la **sezione 3**, i disegni nuovi, ciascuno A/B coi tre controlli della decisione 18, a parole e col diagramma reso dal
sorgente identico (decisione 21). ⛔ **Il materiale è già letto, e sta qui perché non si rilegga da capo** — si riverifica
coi comandi, non si cita:

| Per | Che cosa esiste, letto il 2026-09-08 | Dove |
|---|---|---|
| l'`erDiagram` di `docs/design/10-modello-dei-dati-durevoli.md` | `RecordV1` ha **sei** campi con indice: `kind` (0), `effect` (1), `trust` (2), `payload` (3, byte), `reason` (4), `detail` (5, `Option<Detail>`, facoltativo, indice nuovo); `RecordKind` sei varianti — `Intent`, `Outcome`, `Note`, `Verdict`, `Routing`, `Permission`; `Detail` tre specie — `Verdict` (`passed`, `spent_millis`), `Routing` (`model`, `evaluated`, `degraded`), `Permission` (`tool`, `resource`, `write`); `Record` è l'enum di versione con la sola `V1` | `crates/kernel/src/record.rs`: `grep -n -A1 '#\[n(' crates/kernel/src/record.rs` |
| idem, l'archivio | la tabella `redb` del giornale: chiave `u64` progressiva → (`step: u64`, `kind: u8`, i byte del record); `FileJournal` porta `next_key` | `crates/platform/src/journal.rs`, la riga `const RECORDS: TableDefinition` |
| idem, i byte congelati | sei record, uno per `RecordKind`, più la mappa | `ls crates/kernel/tests/frozen/` |
| idem, ciò che arriva | col 2 la specie `Invocation` (funzione, invocatore, argomento) e la chiave della disposizione (enum chiuso, una variante, valore opaco); col 3 run e sub-run — oggi non esiste `RunId`, solo `StepId`; col 4 il piano; col 5 la versione conservata dal checkpoint, riferita dal passo, dentro un ambito dichiarato; col 6 la cartella della knowledge base e il suo indice — nodi router · gruppo · foglia · skill · guida-modello · cattura; attributi percorso · specie · etichetta · provenienza · ultima modifica; frecce router → gruppo → foglia → skill → router; col 13 le guide approvate | §5 del 2 · §2 qui sopra · ADR-0024, Decision 1–2 · disegno della knowledge base §2.2 e §4.2 · design/09 e design/03 |
| «la GUI dentro» | design/01 disegna la GUI come una scatola, e la §3 qui sopra dice che deve restarlo; il modello è nella tabella «Il modello della GUI», e le tre sequenze — l'accoglienza (`Hello` → `Accepted` → `Degradation`, `Policy`, `Layout`), il giro salva/riavvia/ritrova (`SaveLayout` → settima porta → `Layout`, e il vecchio se la scrittura fallisce), l'invocazione (`Invoke` → `PermissionRequired` → `Approve` → `Invoke` → `Policy`) — non hanno un diagramma. 🔶 **Giudizio di chi ha letto, non deciso:** la casa è **questa stella polare**, come `sequenceDiagram` accanto alla tabella del modello — non un file di `docs/design/`, che è la struttura del kernel, né design/01 | §4, §5, §6a del 2 · §2 qui sopra · la tabella «Cosa è già studiato, e dove» |
| il cancello | un file nuovo in `docs/design/` va nell'indice di `README.md`: `check-docs.sh` conta i file e le righe della tabella che linkano `design/`, e devono coincidere; e ogni riga `Q` di design/08 deve avere un metodo (V30) | `scripts/check-docs.sh`, le righe `d_file=` e `d_idx=` |

Poi le sezioni 4–6 e le §7–§10 del 2, come nel punto 4 del verbale.

#### I diagrammi, uno per uno — letti il 2026-09-08

| File | Diagrammi | Che cosa cambia | Sezione |
|---|---|---|---|
| [design/09](../../design/09-l0-fisico.md) | 3 flowchart, 1 stateDiagram | «Gli archivi»: la disposizione, le guide, la policy, i permessi e le transizioni nel giornale, l'indice della mappa; la colonna «Chi lo raggiunge»; Telecamera in «riservato»; due regole | **1**, ✅ scritta il 2026-09-08 |
| [ADR-0022](../../adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md) | tabelle | rimando in testa: le guide sono file della knowledge base; la politica della riga non cambia | 1, ✅ scritta |
| [ADR-0006](../../adr/0006-due-policy-vram-come-oggetti-distinti.md) | — | rimando in testa: il profilo dà il default, il giornale il corrente (decisione 17) | 1, ✅ scritta |
| [design/02](../../design/02-arbitrato-gpu.md) | 1 flowchart, 1 stateDiagram | l'etichetta «cambio di profilo di configurazione» e la frase «determinata dal profilo» (righe 235 e 249 al 2026-09-08) | 1, ✅ scritta |
| [design/05](../../design/05-gateway-inferenza.md) | 3 flowchart | la riga «Policy VRAM · chi la cambia: il profilo di configurazione» (riga 153 al 2026-09-08) | 1, ✅ scritta |
| [design/07](../../design/07-osservabilita-e-degrado.md) | 2 flowchart | «Cosa deve essere sempre visibile» contro la striscia della §3: un richiamo | 2, ✅ scritta il 2026-09-08 — e non un richiamo solo: sette fonti del degrado, la GUI viva a GPU satura, le proiezioni a classi, «Dove» e «Chi» |
| [ADR-0019](../../adr/0019-lo-stato-di-degrado-e-un-oggetto-osservabile.md) | tabella | rimando in testa: la lista degli eventi è aperta — il fallback dichiarato di ADR-0012 c'è già nel codice, la telecamera arriva con ADR-0039 — e a GPU satura resta viva anche la GUI (ADR-0033) | 2, ✅ scritta |
| [design/03](../../design/03-run-durevoli.md) | 2 flowchart, 1 stateDiagram | l'invocazione del registro come passo; il giornale separato dagli altri archivi, coi «(col N)»; le note nel ciclo di vita; il ramo «non dichiarata» → «non leggibile» | 2, ✅ scritta il 2026-09-08, sesta ripresa |
| [design/08](../../design/08-strategia-di-test.md) | 1 flowchart | la GUI nello strato deterministico (col 2); le suite di conformità e la regola che le fa crescere; le sei campagne DST; la riga Q3; lo stato delle Q lasciato alla §8.4 della spec; il «ciclo lungo» della DST profonda che non esiste; ADR-0026 | 2, ✅ scritta il 2026-09-08, settima ripresa |
| [design/01](../../design/01-topologia-dei-processi.md) | 1 flowchart, 1 stateDiagram | la GUI è una scatola e resta tale; la GUI dentro è un disegno nuovo | 3 |
| [design/04](../../design/04-anelli-e-sensori.md) · [design/06](../../design/06-permessi-e-confine-dei-dati.md) | 2 · 3 | 🔶 nulla, giudizio di questa sessione | — |
| [design/10](../../design/10-modello-dei-dati-durevoli.md) | 2 erDiagram | **nuovo**: il giornale a entità com'è nel codice — passo, voce, record, dettaglio, coi due «kind» — e ciò che è deciso per sotto-progetto, con «(col N)»; chi scrive oggi ogni specie; le regole; i comandi | 3, ✅ scritto il 2026-09-08, ottava ripresa |
| tutto `docs/` | — | nessun diagramma a entità: `grep -rE '^\s*erDiagram' docs/` è vuoto. ✅ **RICHIAMO DEL 2026-09-08, ottava ripresa:** ora ne restituisce due, in design/10 | 3 |

#### La proposta per la sezione 1 — presentata il 2026-09-08, NON approvata, NON scritta

✅ **RICHIAMO DEL 2026-09-08, quinta ripresa: APPROVATA (A) E SCRITTA, con due correzioni** — decisione 18: nel nodo «giornale»
anche «guide approvate (col 13)»; nella cella «Chi lo raggiunge» della configurazione **due vie**, i profili consegnati dal daemon
(ADR-0034) e la disposizione dalla settima porta. ⛔ **Il sorgente del diagramma vive ora in [design/09](../../design/09-l0-fisico.md)
e non più qui:** il blocco mermaid che stava sotto è tolto, perché una seconda casa diverge (gotcha #68). Le tabelle restano come
verbale di ciò che fu presentato; il titolo resta com'era.

A parole, cosa cambia — la tabella presentata al proprietario:

| Dove | Prima | Dopo | Fonte |
|---|---|---|---|
| nodo «configurazione» | profili, guide, policy | profili, **disposizione dei pannelli** (col 2) | stella polare §2 |
| nodo «artefatti» | file prodotti | + la **cartella della knowledge base**: router, foglie, **guide**, catture (col 6) | disegno della knowledge base §2.2 |
| nodo «giornale» | run, passi, routing, verdetti, costi | + **permessi, transizioni di policy**, invocazioni del registro (col 2) | `permission.rs` · `set_policy` · §5 del 2 |
| nodo «indici» | embedding, RAG | + l'**indice della mappa** (col 6), rigenerabile | disegno della knowledge base §2.2 |
| tabella degli archivi | quattro colonne | + colonna **«Chi lo raggiunge»**: giornale → porta `journal`, `redb` in `platform`; configurazione → la **settima porta** (col 2); artefatti → porta `filesystem` (ambiti e checkpoint; la vera col 5); segreti → la crate `secrets`, unico punto di lettura, vuota oggi per decisione; indici → la capacità (6) li costruisce e li rigenera, l'indice della mappa lo tiene il core e lo manda alla GUI via `ipc`; pesi → gestione dedicata (9) | spec §2.3 · stella polare §2 · ADR-0023 |
| tabella «riservato» | avvio automatico ❌, voce ❌ | + **telecamera ❌** | ADR-0039, rimando ad ADR-0023 |
| regole in fondo | — | + la disposizione è un **pacchetto opaco**; + la **policy non è configurazione** — i testi qui sotto | decisioni 14, 15 e 17 |
| ADR-0022 | riga «configurazione, guide, profili» | rimando datato in testa: le guide sono file della knowledge base; la politica della riga — in chiaro, nel backup, permanente — non cambia | disegno della knowledge base |
| ADR-0006 · design/02 · design/05 | «determinata dal profilo di configurazione» | rimando: il profilo dà il default, il giornale il corrente; in design/02 l'etichetta della freccia fra le due policy diventa «transizione esplicita — dal 2 una funzione del registro» | decisione 17 |

Il diagramma nuovo sta in design/09 — vedi il richiamo in testa a questo blocco.

Le due regole da aggiungere a *«Regole che i diagrammi non esprimono»* di design/09: **(1)** la disposizione dei pannelli è un
pacchetto opaco — il core la custodisce dalla settima porta e la restituisce alla GUI, non la legge mai per decidere: non è un
parametro consegnato (ADR-0034), è ciò che la GUI gli affida (decisioni 14 e 15); **(2)** la policy VRAM corrente non è
configurazione — è la proiezione del giornale, l'ultima transizione che `Arbiter::set_policy` scrive come intento ed esito; il
profilo dà il default (ADR-0006, rimando del 2026-09-08); il daemon la rilegge all'avvio, compito del piano del 2.

I due rimandi vanno in testa all'ADR, sotto «Deciders», nella forma del rimando di ADR-0009 (blocco citato, data, grassetto sul
fatto, «nessuna riga superata»). **ADR-0022:** la riga «configurazione, guide, profili» delle due tabelle — le guide sono file
della cartella della knowledge base, artefatti dell'utente, dal disegno del 2026-09-04 (§2.2); la configurazione contiene i
profili e, col 2, la disposizione dei pannelli, raggiunta dal kernel da una settima porta (stella polare §2); la politica della
riga non cambia. **ADR-0006:** «uno solo è attivo, determinato dal profilo di configurazione corrente» — il profilo dà il default,
e la policy corrente è la proiezione del giornale (decisione 17 della stella polare). In design/02 la frase «la policy attiva è
determinata dal profilo di configurazione» riceve il richiamo datato con lo stesso contenuto; in design/05 la riga «Policy VRAM ·
chi la cambia» dice «una transizione esplicita — dal 2 una funzione del registro; la corrente è la proiezione del giornale».

Le skill della sessione nuova: quelle dell'elenco qui sotto, più `anthropic-skills:design-docs`, che governa i diagrammi.
L'elenco numerato qui sotto resta com'era, come verbale.

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è il commit di questa
   chiusura o uno successivo.
2. La lettura obbligatoria di `CLAUDE.md`: il compendio per intero, a blocchi, e la testa dell'audit del
   2026-08-27.
3. **Questo file per intero**, poi la [consegna del 2](2026-09-06-sottoprogetto-2-gui-minima-design.md)
   per intero, coi richiami datati. Poi le skill: `anthropic-skills:decision-principles`,
   `anthropic-skills:session-resume` su questo file, `anthropic-skills:dev-discipline`,
   `anthropic-skills:dev-communication`, `superpowers:brainstorming` — percorso architetturale, si
   riprende dalle **conferme sospese**.
4. Rilanciare i comandi della tabella dello stato; riverificare le versioni della tabella npm il giorno
   in cui si sceglie una dipendenza, non prima.
5. Al proprietario, una per volta e in forma A/B: la conferma del wireframe di **Lavoro** com'è, poi di
   **Compatta e grafo** com'è; poi le sezioni 1–6 della tabella *«Le sezioni che mancano»*, ciascuna col
   controllo esplicito sui cinque criteri e con verificato, dedotto e assunto separati, chiedendo il sì
   dopo ogni sezione; poi le §7–§10 del 2.
6. Alla chiusura: i due disegni scritti sul posto, o le consegne aggiornate se il proprietario ferma
   prima; il testo di questa consegna in `docs/archivio/consegna-brainstorming-direzione-gui.md` e
   quello del 2 in `docs/archivio/consegna-brainstorming-sottoprogetto-2.md`, parola per parola coi link
   riscritti; il puntatore della §6 del compendio mosso; `bash scripts/check-docs.sh`,
   `bash scripts/gate.sh`, commit, push.

## Come tornare operativi

```bash
git fetch --all --prune && git status -sb && git log --oneline -3
bash scripts/check-docs.sh
bash scripts/gate.sh
```
