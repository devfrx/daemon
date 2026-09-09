# Archivio — la cronaca della consegna sulla direzione della GUI, dal 2026-09-07 al 2026-09-09

⛔ **Non è una lettura obbligatoria.** È il **verbale**, tenuto **parola per parola**, della cronaca delle undici riprese
del brainstorming sulla direzione della GUI — la stella polare — com'era scritta nella consegna alla chiusura dell'undicesima
ripresa, il **2026-09-09**: le sezioni «Stato in una riga», «Da sapere subito», «Stato del repo alla chiusura», «Fatto in
questa sessione» e «Prossimo passo, eseguibile». Spostato qui il 2026-09-09, dodicesima ripresa, per il **mandato del
proprietario** (decisione 26 della stella polare: sfoltire la lettura d'apertura, ogni taglio in A/B — questo è il primo,
approvato **A**), con la regola di `CLAUDE.md`: *un documento vivo porta ciò che è vero adesso, e un verbale va in archivio*.
Il precedente è [quella della knowledge base](consegna-brainstorming-knowledge-base.md); ⚠️ a differenza di quella, qui il
disegno **non è ancora scritto**: quando lo sarà, sul posto, il resto della consegna va in coda a **questo** file, parola per
parola — è il viaggio che la consegna stessa prevede in testa.

⚠️ **Ciò che è scritto qui era vero il giorno in cui fu scritto.** Il documento vivo è
[`../superpowers/specs/2026-09-07-direzione-gui-design.md`](../superpowers/specs/2026-09-07-direzione-gui-design.md) — le
decisioni, le sezioni approvate, le registrate, i vicoli ciechi e il solo prossimo passo vivo stanno **lì**, e non sono
copiati qui; il prossimo passo sta nella §6 di [`../COMPENDIO.md`](../COMPENDIO.md), in un posto solo.

⚠️ **Una sola cosa è cambiata rispetto al testo consegnato:** i percorsi relativi dei collegamenti, riscritti per questa
cartella. Fra «Fatto in questa sessione» e «Prossimo passo, eseguibile» la consegna porta le sezioni di merito, che restano
nel documento vivo: qui il salto è segnato da una riga fra parentesi quadre.

📌 **La misura del giorno del taglio, coi comandi che la rifanno** — `wc -c <file>` e lo snippet `tiktoken` di `CLAUDE.md`
(`cl100k_base`, limite inferiore): la stella polare pesava **222 315 byte** e **71 886 token** prima del taglio; il dopo lo
dice il comando, non questa riga.

---

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
delega, decisione 20)**: si riprende dal secondo disegno, «la GUI dentro». ✅ **La nona ripresa, lo stesso giorno, ha SCRITTO il secondo disegno su delega (decisioni 21–23):** le tre sequenze come `sequenceDiagram` in questo file, sotto «Il modello della GUI», e due correzioni alla consegna del 2 (§3, §4, §5, §6a) col richiamo datato; **la sezione 3 e la passata sui diagrammi sono CHIUSE**, si riprende dalle sezioni 4–6. ✅ **La decima ripresa, lo stesso giorno, ha PRESENTATO la sezione 4** — lo spike di accettazione di `dockview`, sette mosse giudicate dal proprietario più le misure — **approvata A (decisione 24) e NON scritta**: si scrive nella sessione nuova dal blocco «La proposta per la sezione 4» del prossimo passo; la domanda del proprietario sulla telecamera ha aperto una **mossa 8**, la mano di SP-7 come puntatore, proposta in forma A/B e **senza risposta**. ✅ **L'undicesima ripresa, il 2026-09-09, ha DECISO la mossa 8 — A, dal proprietario (decisione 25) — e ha SCRITTO la §4** con otto mosse e la riga Q4, col richiamo datato nella §2 del 2; il puntatore della §6 del compendio è sfoltito della cronaca delle riprese, archiviata: si riprende dalla **sezione 5**, poi dalla 6. ⛔ **Alla chiusura dell'undicesima ripresa la §7 del 2 — il core finto — è presentata in forma A/B e SENZA risposta, e il proprietario ha dato il mandato di SFOLTIRE la lettura d'apertura prima di ripartire (decisione 26): la sessione occupa ~350 000 token appena comincia.**

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

✅ **La nona ripresa, il 2026-09-08, ha scritto «la GUI dentro»** — le tre sequenze (l'accoglienza, il giro salva/riavvia/ritrova,
l'invocazione) come `sequenceDiagram` in questo file, sotto la tabella «Il modello della GUI» — su delega («decidi secondo la
skill»), decisioni 21–23. Disegnare ha trovato **due buchi in sezioni approvate del 2**, corretti col richiamo datato nella consegna
del 2: il permesso non aveva un passo su cui posarsi — `Approve` porta ora anche l'invocazione, §4, §5 e §6a — e «il core chiude»
non è un'operazione della porta `ipc` — il core non ascolta più il client rifiutato, la GUI esce, §3 e §5. ⛔ **La sezione 3 e la
passata sui diagrammi (decisione 16) sono CHIUSE: si riprende dalle sezioni 4–6** di «Le sezioni che mancano», poi le §7–§10 del 2,
poi i due disegni e il piano. `check-docs.sh` OK e `GATE GREEN` all'apertura e alla chiusura; la chiusura è chiesta dal proprietario con `session-handoff` («prima dimmi quanto manca e dove ci troviamo»), e la risposta sta nel prossimo passo; la consegna è questo file; nessun codice toccato.

✅ **La decima ripresa, il 2026-09-08, ha PRESENTATO la sezione 4 e NON l'ha scritta.** Ripresa con `session-resume`: stato riletto
coi comandi, nessuna divergenza — `spikes/gui-shell/` e `gui/` non esistono, com'è giusto — `check-docs.sh` OK e `GATE GREEN`
all'apertura. La sezione 4 — lo spike di accettazione di `dockview` dentro lo spike del guscio: **sette mosse** giudicate dal
proprietario provandole, il protocollo congelato prima, le misure (la mossa 7 come confronto di JSON, M4 con `dockview` acceso, Q3
per il popout), la riserva `interactjs` costruita solo su un no — presentata coi tre controlli della decisione 18 e i cinque criteri,
**approvata A** (decisione 24). Alla domanda del proprietario *«ma si fa anche la prova con la telecamera per spostarle con
mediapipe?»* la risposta verificata alla fonte — `dockview` ha `dndStrategy: 'pointer'`, e il disegno dei gesti §4.1 fa del
«qualunque puntatore» un compito del 2 — è una **mossa 8** più una riga **Q4**, proposte in forma A/B col consiglio A e **senza
risposta**. ⛔ **Niente è scritto nei file di merito:** la sezione sta parola per parola nel prossimo passo, blocco «La proposta per la
sezione 4»; la §2 del 2 non è toccata; i tre fatti letti oggi su dockview.dev stanno nella tabella «`dockview` 8.x». Chiusura chiesta
dal proprietario con `session-handoff` («la si scrive nella prossima sessione, non perdiamo eventuali findings/gotcha e decisioni»):
`check-docs.sh` OK e `GATE GREEN` alla chiusura; la consegna è questo file; nessun codice toccato. **Si riprende dalla risposta sulla
mossa 8, poi si scrive la §4, poi le sezioni 5 e 6.**

✅ **L'undicesima ripresa, il 2026-09-09, ha SCRITTO la §4.** Ripresa con `session-resume`: stato riletto coi comandi, nessuna
divergenza, `check-docs.sh` OK e `GATE GREEN` all'apertura. La mossa 8 — la mano di SP-7 come puntatore — decisa **A** dal
proprietario (decisione 25): la §4 sta nelle sezioni approvate con **otto mosse** e la riga **Q4**, col limite di `'pointer'`
sulla mossa 4 dichiarato (decisione 34), il richiamo datato nella §2 del 2 e la riga 4 di «Le sezioni che mancano» che vi
rimanda; le tabelle della proposta sono uscite dal prossimo passo (decisione 22). Il puntatore della §6 del compendio è
**sfoltito** della cronaca delle riprese — archiviata parola per parola in `docs/archivio/stato-storico.md`, con l'intestazione —
e dice lo stato di adesso (decisione 35). ⏭️ **Si riprende dalla sezione 5** — core finto, prove e cancello, decisioni aperte,
come si riprende: le §7–§10 del 2 — poi dalla 6; il come sta nel prossimo passo. Nessun codice toccato.

⛔ **Alla chiusura dell'undicesima ripresa, chiesta dal proprietario con `session-handoff`: la §7 del 2 — il core finto — è
PRESENTATA in forma A/B (reuse contro imitazione, consiglio A) e NON ha risposta**, e il proprietario ha dato un **mandato** per
la sessione nuova (decisione 26, con le sue parole): **prima di partire**, dare una pulita e un ordine alla documentazione di
ripresa e a tutto ciò che è obbligatorio leggere, e ridimensionarli dove serve, perché appena la sessione comincia ne occupa
**~350 000 token**. ⏭️ **Il prossimo passo NON è la §7: è quel mandato**, poi la §7 dalla domanda lasciata aperta, poi §8–§10 e
la sezione 6. La lettura d'apertura è **misurata coi comandi** nel prossimo passo — byte e token, blocco per blocco — e lì sta
l'ordine eseguibile. `check-docs.sh` OK e `GATE GREEN` alla chiusura; nessun codice toccato.

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
| i commit di questa sessione | `git log --oneline 664265a..HEAD` | i commit del 2026-09-07: la consegna, il punto fermo della prima ripresa, le tabelle e le decisioni della seconda, la sua chiusura, e la terza ripresa del 2026-09-08: la §2, la §3 con la sua chiusura; la quarta ripresa dello stesso giorno: l'apertura della passata sui diagrammi, la sua chiusura; la quinta ripresa: la sezione 1 scritta, design/07 con ADR-0019, la sua chiusura; la sesta ripresa: design/03 e la sua chiusura; la settima ripresa: design/08 e la sua chiusura; l'ottava ripresa: la lettera E nella spec, design/10 e la sua chiusura; la nona ripresa: «la GUI dentro», le tre sequenze coi richiami alla consegna del 2, e la sua chiusura; la decima ripresa: la sezione 4 presentata e approvata, non scritta — la sua chiusura; l'undicesima ripresa del 2026-09-09: la mossa 8 decisa A e la §4 scritta, il richiamo nella §2 del 2, il puntatore della §6 del compendio sfoltito e la cronaca archiviata — e la sua chiusura |
| codice e spec non toccati | `git diff --stat 664265a..HEAD -- crates/ scripts/ spikes/ .github/ Cargo.lock Cargo.toml rust-toolchain.toml docs/superpowers/specs/2026-08-06-kernel-design.md docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` | nulla, tranne la spec del sotto-progetto 1: tre righe e un richiamo nella §8.2, l'ottava ripresa su delega (decisione 19) |
| cancello | `bash scripts/gate.sh` | `GATE GREEN` — rilanciato su `664265a` prima di scrivere i documenti della consegna, e di nuovo alla chiusura della seconda ripresa del 2026-09-07 e della quarta ripresa del 2026-09-08, e all'apertura e alla chiusura della quinta, e all'apertura e alla chiusura dell'ottava, e all'apertura e alla chiusura della nona e della decima, e all'apertura e alla chiusura dell'undicesima; `check-docs.sh` a ogni commit. Si rilancia, non si cita |
| documenti | `bash scripts/check-docs.sh` | `OK` |
| fine-riga | `git ls-files --eol docs/COMPENDIO.md docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md docs/superpowers/specs/2026-09-07-direzione-gui-design.md docs/superpowers/specs/2026-09-07-direzione-gui-wireframes/*.svg docs/design/10-modello-dei-dati-durevoli.md` | il compendio `i/lf w/crlf`, gli altri `i/lf w/lf` — su questa macchina: un clone nuovo con `core.autocrlf=true` li mostra `w/crlf`, e non è una divergenza |
| i wireframe esistono | `ls docs/superpowers/specs/2026-09-07-direzione-gui-wireframes/` | `compatta-e-grafo.svg  home.svg  lavoro.svg` |
| margine del compendio | `wc -c docs/COMPENDIO.md` contro `grep -n '^ceiling=' scripts/check-docs.sh` | positivo. ⚠️ **Decima ripresa, 2026-09-08:** sceso sotto i **tremila byte** prima di questa chiusura, e ogni chiusura ne mangia qualche centinaio: alla prossima, **prima** di aggiungere una riga, la cronaca delle riprese esce dal puntatore della §6 — vive già qui, in «Da sapere subito» e nel «Fatto» — e nel compendio resta lo stato di adesso (gotcha #100). ✅ **Undicesima ripresa, 2026-09-09: FATTO** — misurato all'apertura, col comando di questa riga, **1694 byte** di margine; la cronaca è uscita dal puntatore e dall'intestazione, archiviata in `docs/archivio/stato-storico.md`, e il margine lo dà il comando |

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
30. **Alla nona ripresa, il 2026-09-08:** ripresa con `session-resume` — stato riletto coi comandi, nessuna divergenza,
    `check-docs.sh` OK e `GATE GREEN` all'apertura; il materiale di «la GUI dentro» riverificato nel codice — il tratto `Ipc`
    (tre operazioni, nessuna chiusura), `IpcMessage` (due varianti), il daemon senza attività e senza `Ipc`, `permission::grant`
    (una nota su un passo aperto da altri) e `is_granted`, `Arbiter::set_policy`, `degradation_now`, `ClientGrants::on_disconnect`,
    `DyingGui` e `gui_death_campaign.rs`, design/01, la §6.1.2 e la §6.1.4 della spec; nessun `sequenceDiagram` in `docs/`
    (`grep -rn '^\s*sequenceDiagram' docs/`). Le tre sequenze presentate coi tre controlli della decisione 18 e rese in chat dal
    sorgente identico (decisione 21 del coordinatore); disegnare ha trovato **due buchi** nella consegna del 2 — il permesso senza
    un passo, «il core chiude» senza un'operazione — portati in forma A/B insieme alla casa dei diagrammi.
31. Il proprietario ha **delegato** («decidi secondo la skill»): decise **A, A, A** (decisioni 21, 22, 23) — le sequenze scritte in
    questo file sotto «Il modello della GUI» dal sorgente identico a quello reso; i richiami datati nella consegna del 2, §3, §4, §5
    e §6a, più la cifra dei record congelati (sei, non quattro) nella riga «il giornale» della §5, la stessa correzione della sesta
    ripresa nell'altra casa; la registrata sull'archivio della disposizione che non si apre all'avvio; le righe di questo file; il
    puntatore della §6 del compendio; fine-riga di ciascun file conservati e rimisurati; `check-docs.sh` OK, commit e push. Alla
    domanda del proprietario *«tutti i diagrammi di ora e delle sessioni precedenti dove sono salvati?»* la risposta coi comandi, in
    chat: le case sono `docs/design/`, le due spec del kernel, questo file e i tre SVG dei wireframe — i comandi stanno nella riga
    «tutto `docs/`» della tabella «I diagrammi, uno per uno».
32. Chiusura della nona ripresa, chiesta dal proprietario con `session-handoff` («prima dimmi quanto manca e dove ci troviamo»):
    la risposta in chat e nel prossimo passo — la passata sui diagrammi è chiusa; restano le sezioni 4–6, le §7–§10 del 2, i due
    disegni, il piano in due parti, poi l'esecuzione; stato riletto coi comandi, `check-docs.sh` OK e `GATE GREEN` alla chiusura;
    in questo file «Da sapere subito», la tabella dello stato e questo punto; la memoria dell'agente aggiornata; scratchpad
    pulito; nessun codice toccato.
33. **Alla decima ripresa, il 2026-09-08:** ripresa con `session-resume` — stato riletto coi comandi, nessuna divergenza
    (`spikes/gui-shell/` e `gui/` non esistono, com'è giusto), `check-docs.sh` OK e `GATE GREEN` all'apertura; il materiale della
    sezione 4 riverificato — `spikes/gesti/PROTOCOLLO.md` e la sezione SP-7 di `spikes/RISULTATI.md` (il precedente: criteri prima,
    congelati al primo commit, il giudizio del proprietario con le sue parole), la regola «parziale» di `spikes/PROTOCOLLO.md`, P3
    «stretto» in `spikes/GUI-REQUISITI.md`, ADR-0029 con M1–M5, le righe 2, 6, 10 e 12 della roadmap, `.gitignore` — e letto alla
    fonte su dockview.dev: i popout aprono una finestra nuova su `popout.html` legata con `window.opener`; `toJSON`/`fromJSON` con
    l'evento `onDidLayoutChange`. La **sezione 4** presentata a parole, con la tabella delle sette mosse, le misure, la regola di
    decisione, ciò che non prova, ciò che cambia altrove, i tre controlli della decisione 18 e i cinque criteri, con verificato,
    dedotto e assunto separati; **approvata A** (decisione 24).
34. Alla domanda del proprietario *«A, ma si fa anche la prova con la telecamera per spostarle con mediapipe?»* la risposta
    verificata alla fonte — la pagina «Drag and drop strategy» di dockview.dev: `dndStrategy` `'auto'` usa il drag nativo HTML5 per
    il mouse, `'pointer'` guida ogni ingresso con eventi del puntatore, che una mano finta può creare — e nel disegno dei gesti (§4.1:
    il 2 costruisce pannelli che si muovono con qualunque puntatore): una **mossa 8** — la pinza dal worker e dal relay di SP-7 com'è,
    tradotta in eventi del puntatore, `dockview` in `'pointer'`, provata nel browser — e una riga **Q4** per guscio, proposte in forma
    A/B col consiglio A; **senza risposta**. Poi la chiusura, chiesta dal proprietario con `session-handoff` («la si scrive nella
    prossima sessione, non perdiamo eventuali findings/gotcha e decisioni»): le righe di questo file — lo stato in una riga, «Da sapere
    subito», la tabella dello stato, le decisioni 24 e 30–33, i tre fatti nella tabella di `dockview`, il richiamo sulla riga 4 delle
    sezioni che mancano, i vicoli ciechi, il prossimo passo col blocco della sezione 4 parola per parola — il puntatore della §6 del
    compendio e la sua intestazione; `check-docs.sh` OK e `GATE GREEN` alla chiusura; fine-riga conservati (LF qui, CRLF del
    compendio); la memoria dell'agente aggiornata; scratchpad pulito; nessun codice toccato.
35. **All'undicesima ripresa, il 2026-09-09:** ripresa con `session-resume` — stato riletto coi comandi, nessuna divergenza
    (`spikes/gui-shell/` e `gui/` non esistono; il solo diff della spec del sotto-progetto 1 è la lettera E dell'ottava ripresa),
    `check-docs.sh` OK e `GATE GREEN` all'apertura; i pezzi che la mossa 8 nomina verificati con `ls` — `s2_worker.py`, `relay/` con
    `page.html`, `PROTOCOLLO.md` in `spikes/gesti/`; O7 e i 114 ms in `spikes/RISULTATI.md` — e la riga «qualunque puntatore» del
    disegno dei gesti, §4.1. La domanda A/B sulla mossa 8 posta al proprietario a parole di tutti i giorni: **A** (decisione 25). La
    **§4** scritta nelle sezioni approvate con otto mosse e la riga Q4, dal blocco della decima ripresa, più il limite di `'pointer'`
    sulla mossa 4 (decisione 34); il richiamo datato nella §2 del 2; la riga 4 di «Le sezioni che mancano» rimanda alla §4; le
    tabelle del blocco tolte dal prossimo passo (decisione 22); il puntatore della §6 del compendio e la sua intestazione riscritti
    allo stato di adesso e sfoltiti della cronaca delle riprese, archiviata parola per parola (decisione 35); fine-riga conservati e
    rimisurati; `check-docs.sh` OK, commit e push.
36. Chiusura dell'undicesima ripresa, chiesta dal proprietario con `session-handoff` (le sue parole nella decisione 26): la
    **§7** del 2 presentata in forma A/B — il core finto che **riusa** l'attività del kernel su porte in memoria contro un copione
    che **imita** il daemon, consiglio A — e lasciata **senza risposta**, parola per parola nel prossimo passo; il mandato dello
    sfoltimento scritto come **primo passo** della sessione nuova, con la lettura d'apertura misurata coi comandi, in byte e in
    token, blocco per blocco; stato riletto coi comandi, `check-docs.sh` OK e `GATE GREEN` alla chiusura; il puntatore della §6
    del compendio e la sua intestazione; la memoria dell'agente aggiornata; scratchpad pulito; nessun codice toccato.

*[Qui la consegna porta «Le decisioni del proprietario, una per domanda», «Ciò che il repo diceva già, letto per decidere»,
«Lo stato dell'arte verificato, e il comando», «I wireframe, e il loro stato», «Il modello della GUI» con le tre sequenze,
«Le sezioni approvate del disegno», «Le sezioni che mancano», «Decisioni prese dal coordinatore», «Registrate, non prese» e
«Vicoli ciechi di questa sessione»: restano nel documento vivo.]*

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
   [design/09](../design/09-l0-fisico.md) — il nodo «configurazione» guadagna la **disposizione**; le **guide** sono
   file della knowledge base dal disegno del 2026-09-04; la **policy** al riavvio, registrata — e le **sette famiglie**
   dove le sei sono scritte: spec §2.3 e §3.1, `crates/kernel/src/ports/mod.rs` (con le cifre in prosa «SIX», «FIVE
   fakes», «SEVEN things»), `crates/kernel/tests/ports_are_implementable.rs`. [design/01](../design/01-topologia-dei-processi.md)
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
[design/10](../design/10-modello-dei-dati-durevoli.md), il modello dei dati durevoli in due `erDiagram`. **Si riprende dal
secondo disegno, «la GUI dentro»** — le tre sequenze della riga omonima nella tabella qui sotto (l'accoglienza, il giro
salva/riavvia/ritrova, l'invocazione) come `sequenceDiagram`; la casa proposta è **questa stella polare**, accanto alla tabella
«Il modello della GUI», non un file di `docs/design/` (giudizio della settima ripresa, da confermare A/B coi tre controlli e
col diagramma reso dal sorgente identico); poi le sezioni 4–6.

✅ **Alla nona ripresa, il 2026-09-08: il secondo disegno è SCRITTO (A su delega, decisioni 21–23)** — le tre sequenze nella
sottosezione «La GUI dentro», sotto «Il modello della GUI», dal sorgente identico a quello reso in chat; i richiami datati nella
consegna del 2 (§3, §4, §5, §6a). ⛔ **La sezione 3 e la passata sui diagrammi (decisione 16) sono CHIUSE.** ⏭️ **Si riprende dalle
sezioni 4–6 di «Le sezioni che mancano»** — lo spike di accettazione di `dockview` (4); core finto, prove e cancello, decisioni
aperte, come si riprende (5); dove vive la stella polare (6) — ciascuna in forma A/B coi tre controlli della decisione 18; poi le
§7–§10 del 2; poi i due disegni scritti sul posto e il piano in due parti. Restano del proprietario le registrate: l'innesco B (3)
di Q6/Q11, e le due nate disegnando — i due passi per invocazione, col 3; l'archivio della disposizione che non si apre, del
disegno del 2. La struttura moduli/esemplari/viste resta a parole nella tabella «Il modello della GUI»: un disegno si fa solo se il
proprietario lo chiede (decisione 29 del coordinatore). **Quanto manca, in passi:** tre sezioni della stella polare — la 4, la 5 (che
sono le §7–§10 del 2) e la 6 — una per volta e A/B; poi i due disegni scritti sul posto, con le due consegne archiviate parola per
parola; poi il piano del 2 in due parti, la prima fino allo spike compreso; poi l'esecuzione, subagent-driven, in sessioni nuove; in
parallelo AUD-004, l'ADR del proprietario che sbarra il 13.

✅ **Alla decima ripresa, il 2026-09-08: la sezione 4 è PRESENTATA e APPROVATA (A, decisione 24) e NON È SCRITTA** — il proprietario ha
chiuso la sessione con `session-handoff` («la si scrive nella prossima sessione, non perdiamo eventuali findings/gotcha e decisioni»).
⏭️ **L'ordine eseguibile della sessione nuova:**

1. Al proprietario, **la domanda A/B sulla mossa 8** — la mano di SP-7 come puntatore — dal blocco *«La proposta per la sezione 4»*
   qui sotto, parola per parola; consiglio **A**.
2. **Scrivere la §4** in «Le sezioni approvate del disegno», dopo la §3, dal blocco qui sotto — sette mosse, o otto se la risposta
   è A — nella forma delle §1–§3: le tabelle, «Cosa non prova», «Cosa cambia altrove», i tre controlli della decisione 18, i cinque
   criteri con verificato, dedotto e assunto separati; il **richiamo datato** nella §2 del 2 (il frontend minimo porta `dockview-core`,
   chat e scena sono due tessere, nascono Q3 e Q4, il protocollo porta le mosse); la riga 4 di «Le sezioni che mancano» rimanda alla §4;
   le tabelle del blocco **escono** dal prossimo passo e resta il rimando (decisione 22). `git ls-files --eol` prima di scrivere:
   questo file e la consegna del 2 sono `w/lf`. Poi `bash scripts/check-docs.sh`, commit, push.
3. La **sezione 5** — core finto, prove e cancello, decisioni aperte, come si riprende: sono le §7–§10 del 2, e le proposte da cui
   partire stanno nella tabella «Le sezioni che mancano» della consegna del 2; Passi aggiunge al core finto la lista dei passi, le
   prove aggiungono «salva, riavvia, ritrova» e le mosse dello spike. Poi la **sezione 6**, dove vive la stella polare. Ciascuna A/B
   coi tre controlli della decisione 18.
4. Poi i due disegni scritti sul posto, con le due consegne archiviate parola per parola; poi il piano del 2 in due parti.

⚠️ **Il margine del compendio è piccolo** (il comando nella tabella dello stato): alla prossima chiusura la cronaca delle riprese
esce dal puntatore della §6 e resta qui, **prima** di aggiungere una riga.

✅ **All'undicesima ripresa, il 2026-09-09: i punti 1 e 2 sono ESEGUITI, e il margine anche.** La mossa 8 decisa **A** dal
proprietario (decisione 25); la §4 scritta nelle sezioni approvate con otto mosse e la riga Q4, più il limite di `'pointer'` sulla
mossa 4 (decisione 34); il richiamo datato nella §2 del 2; la riga 4 di «Le sezioni che mancano» rimanda alla §4; le tabelle del
blocco qui sotto sono uscite (decisione 22). La cronaca delle riprese è uscita dal puntatore della §6 del compendio e dalla sua
intestazione, archiviata parola per parola in `docs/archivio/stato-storico.md` (decisione 35). ⏭️ **Si riprende dal punto 3: la
sezione 5** — core finto, prove e cancello, decisioni aperte, come si riprende, cioè le §7–§10 del 2, dalle proposte della tabella
«Le sezioni che mancano» della consegna del 2, con la lista dei passi nel core finto e, nelle prove, «salva, riavvia, ritrova» e le
otto mosse dello spike — poi la **sezione 6**, dove vive la stella polare; ciascuna A/B coi tre controlli della decisione 18; poi il
punto 4.

⛔ **Alla chiusura dell'undicesima ripresa, il 2026-09-09, chiesta dal proprietario con `session-handoff`: la §7 è PRESENTATA e
SENZA risposta, e il proprietario ha dato il mandato dello SFOLTIMENTO (decisione 26).** ⏭️ **L'ordine eseguibile della sessione
nuova:**

1. ⛔ **Prima di tutto, il mandato: sfoltire la lettura d'apertura.** Misurata alla chiusura, il 2026-09-09, coi comandi qui sotto:
   la lettura obbligatoria — `CLAUDE.md`, il compendio intero, la testa dell'audit fino a «Dettaglio», questo file intero, la
   consegna del 2 — pesa **536 KB** e **174 387 token** `cl100k` (limite inferiore: su italiano con emoji Claude conta di più), e la
   sessione ne occupa **~350 000** appena comincia (misura del proprietario, che comprende le skill e il prompt di sistema, che
   nessun comando qui misura). Dove pesa, in byte, blocco per blocco:

   | Documento | Peso | Ciò che è cronaca, o chiuso | Dove va |
   |---|---|---|---|
   | `docs/COMPENDIO.md` | 182 701 B · 59 534 token | la §6 è **104 078 B**, di cui **93 878** il riquadro «Le voci ancora aperte»: blocchi tenuti parola per parola perché nominano decisioni del proprietario — il **debito dichiarato** dello sfoltimento del 2026-08-28, che prevedeva la consolidazione in una tabella sola *«da presentare al proprietario una per una»* | la consolidazione: **voce per voce** col proprietario, in A/B; il racconto in `docs/archivio/stato-storico.md`; il tetto di `check-docs.sh` scende con lo sfoltimento (gotcha #100) |
   | questo file | 207 902 B · 67 280 token | cronaca: «Da sapere subito» **11 991**, «Fatto in questa sessione» **19 424**, «Prossimo passo, eseguibile» **30 947**; merito: «Le sezioni approvate» 59 911, le due tabelle delle decisioni 31 178, «Il modello della GUI» 13 174 | la cronaca in `docs/archivio/consegna-brainstorming-direzione-gui.md` (o un file gemello), parola per parola coi link riscritti; qui restano lo stato in poche righe, le decisioni, le sezioni approvate, le registrate, i vicoli ciechi ancora utili e il solo prossimo passo **vivo** |
   | `docs/audit-2026-08-27.md`, righe 1–450 | 82 907 B · 27 475 token | «Stato dei rimedi» **36 121** e «I 73 finding» **19 591**: tutti chiusi (`awk -F'\|' 'NF>4{gsub(/^ +\| +$/,"",$5); print $5}' docs/audit-2026-08-27.md \| grep -c aperto` → 0) | la lettura d'apertura si restringe a «Come si concludono quelli aperti» e alla tabella delle voci senza numero AUD; è una riga di `CLAUDE.md`, quindi del **proprietario** |
   | `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` | 46 757 B · 15 078 token | poco: le sezioni approvate sono merito | resta; i richiami datati si rileggono quando il disegno del 2 si scrive |
   | `CLAUDE.md` | 15 914 B · 5 020 token | i richiami lunghi nella testa e nelle tabelle | è il modo di lavorare: ogni riga tolta è del **proprietario**, A/B |

   I comandi che rifanno la misura — byte: `wc -c CLAUDE.md docs/COMPENDIO.md docs/superpowers/specs/2026-09-07-direzione-gui-design.md
   docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` e `sed -n '1,450p' docs/audit-2026-08-27.md | wc -c`;
   token: lo snippet `tiktoken` di `CLAUDE.md`, esteso ai cinque file (`tiktoken` è installato su questa macchina); per blocco:
   `awk '/^## /{if(h!="")printf "%7d  %s\n", b, h; h=$0; b=0} {b+=length($0)+1} END{printf "%7d  %s\n", b, h}' <file>`.
   **Il metodo, e le regole che non si negoziano:** si misura di nuovo prima di tagliare; per ogni documento una proposta A/B —
   che cosa esce, dove va, che cosa resta — e si scrive solo dopo il sì; **niente si cancella**: un verbale va in `docs/archivio/`
   parola per parola con la data e i link riscritti (regola di `CLAUDE.md`); nessuna decisione del proprietario si riassume
   d'iniziativa; i puntatori restano in una casa sola (gotcha #68); `bash scripts/check-docs.sh` dopo ogni file e i fine-riga
   rimisurati (`git ls-files --eol`: questo file e la consegna del 2 `w/lf`, il compendio e l'archivio `w/crlf`); la memoria
   dell'agente sui blocchi di lettura si aggiorna quando le taglie cambiano.
2. Al proprietario, **la domanda A/B sulla §7**, dal blocco *«La proposta per la sezione 5 — §7, il core finto»* qui sotto,
   parola per parola; consiglio **A**. Se A o B: scrivere la §7 nella consegna del 2, sotto «Le sezioni approvate del disegno»,
   nella forma delle §3–§6a (Pezzo · Forma · La prova), la riga 7 di «Le sezioni che mancano» della consegna col richiamo;
   `check-docs.sh`, commit, push.
3. La **§8** — prove e cancello: `scripts/gate-gui.sh` (il core finto con `cargo test --locked --manifest-path`, poi `npm ci`,
   `npm run build`, `npm test`) da una riga `run` di `gate.sh`; la CI con `actions/setup-node` appuntato; `.gitignore` per `gui/`
   e per le cartelle di build dello spike; la tabella artefatto → controllo dalle colonne «prova» delle §3–§6a più la sonda
   del core finto contro il trasporto vero da un thread, le prove della SPA sulle fixture, la sonda «salva, riavvia, ritrova»,
   la suite di conformità della settima porta, la campagna DST del 2 (§5), le otto mosse dello spike. Poi, come domanda a sé,
   la registrata della nona ripresa: **l'archivio della disposizione che non si apre all'avvio** — A: il core parte e lo
   dichiara con un terzo stato di `Layout`, «non disponibile», e ogni `SaveLayout` riceve lo stesso (richiamo datato sulla riga
   5 della §2 di questo file, che oggi dice «il pacchetto o niente»); B: il core si ferma come per il giornale (`StartupError`).
   Consiglio A, ADR-0019: si dichiara prima, non si fallisce dopo, e un archivio cosmetico non ferma il core.
4. La **§9** — le decisioni aperte col chiusore: quelle della riga 9 della consegna del 2, più le registrate di questo file che
   dicono «il disegno del 2», più il commento falso di `crates/platform/src/journal.rs` (piano del 2). Poi la **§10**, come si
   riprende, sul precedente dei disegni dei gesti e della knowledge base. Poi la **sezione 6** di questo file, dove vive la stella
   polare. Ciascuna A/B coi tre controlli della decisione 18.
5. Poi i due disegni scritti sul posto, con le due consegne archiviate parola per parola; poi il piano del 2 in due parti.

#### La proposta per la sezione 5 — §7, il core finto · presentata il 2026-09-09, undicesima ripresa, SENZA risposta

**A parole.** Un programma piccolo in `gui/fake-core/` che finge di essere il core, così la GUI si costruisce e si prova prima
che il daemon vero faccia tutto e prima che esista un modello. Parla sul filo vero con lo schema vero. Già approvato (§1 e la
riga 7 della consegna del 2, la §3 di questo file): fuori dal workspace Cargo (`gui` in `exclude`), il suo `Cargo.lock`
committato perché lo usa il cancello, token a tempo come lo spike (2000 in dieci secondi, testo non fidato, markdown), `Invoke`
→ `PermissionRequired` la prima volta, `Layout`/`SaveLayout` e la lista dei passi.

**La domanda: quanto è finto?**

> **A — riusa.** Il finto fa girare l'attività **vera** del kernel che ascolta la GUI — quella che il daemon farà girare, e che
> il 2 costruisce come attività del kernel perché la DST la muova con `DyingGui` (§5 del 2) — su porte in memoria: il giornale
> del simulatore, la finta del simulatore per la settima porta, un arbitro vero, e il trasporto vero di `platform`. Finto è solo
> un **rubinetto**: un'attività che manda i token a tempo e, su un comando di una parola da stdin, un `Verdict` o un record di
> routing degradato nel giornale in memoria. Tutto il resto — `Hello`/`Accepted`/`StaleBuild`, `Degradation`, `Policy`,
> `Layout`/`SaveLayout`, `Invoke`/`PermissionRequired`/`Approve`, la lista dei passi — è il codice vero.
> **B — imita.** Un copione a sé che parla lo schema e il filo e scrive a mano ogni risposta del daemon (la riga 7 della consegna
> del 2 com'è scritta).
> Consiglio: **A** — il modo del repo è la logica vera su porte sostituite, cioè il simulatore; B è una seconda copia del
> dispaccio, e il giorno che il daemon cambia il finto diverge senza che nulla diventi rosso.

**Il costo di A, dichiarato:** il finto dipende anche da `simulator` per percorso; l'attività del kernel deve potersi costruire da
fuori, cosa che la campagna DST della §5 già pretende; il rubinetto condivide il trasporto con l'attività attraverso una `RefCell`,
il modo dell'esecutore; la disposizione vive finché il finto non riparte — la persistenza vera è del daemon, provata dalla sonda
della §8.

**I tre controlli (decisione 18).** *Esiste:* l'emettitore di `spikes/gui-ipc/src/bin/core.rs` — righe JSON, tre canali,
sopravvive alla GUI che muore e accetta una riconnessione; `IpcMessage` con due varianti e otto sonde di andata e ritorno e di
rifiuto in `crates/kernel/tests/ipc_wire.rs`; `FakeGui` in `crates/kernel/tests/ports_are_implementable.rs` e `DyingGui` in
`crates/simulator/src/ipc.rs`; il dispaccio di `gui_death_campaign.rs` scritto **dentro il banco** (`accept`/`receive`/`send` in
riga), quindi nessuna attività «servi la GUI» esiste ancora nel kernel: la costruisce il pezzo 6 della §3; nessun trasporto in
`platform`; `gui/` non esiste; il manifesto di radice esclude solo `spikes`. *Arriva:* col 3 il core vero produce i token e il
rubinetto perde quel compito, ma resta per ogni modulo il cui produttore arriva dopo; col 12 il gesto come invocatore, stessa
attività. *Regge crescendo:* una variante nuova è un ramo del `match` nell'attività vera, e con A il finto la segue gratis; con
B è una risposta in più scritta a mano.

**Controllo sui cinque criteri.** Verificato il 2026-09-09: i file sopra, letti. 🔶 **Dedotto:** che un'attività e un rubinetto
possano condividere la porta con una `RefCell` — `Executor::spawn` prende future con vita `'a`, e la campagna dell'arbitro fa
già girare più attività su stato condiviso; che il valore di `Accepted` sia consegnato al finto come al daemon. **Assunto:**
niente. Debito scritto: il rubinetto è codice finto fuori dal prodotto; il suo «degrada» è un meccanismo vero con una causa finta.

> La §7 così? **A:** riusa — l'attività vera su porte in memoria, più il rubinetto. **B:** imita — un copione a sé. Consiglio: **A**.

#### La proposta per la sezione 4 — presentata e approvata (A) il 2026-09-08, decima ripresa; SCRITTA il 2026-09-09

✅ **RICHIAMO DEL 2026-09-09, undicesima ripresa: la sezione è SCRITTA** — la §4 delle sezioni approvate qui sopra, con otto mosse
e la riga Q4. ⛔ **Le tabelle e la prosa che stavano qui sono TOLTE** (decisione 22 del coordinatore): il merito vive nella §4, e
una seconda casa diverge (gotcha #68). Restano, come verbale, le due domande poste e le risposte.

> La sezione 4 va bene così? **A:** sì — si scrive nella stella polare, col richiamo nella §2 del 2, e si passa alla sezione 5.
> **B:** cambia qualcosa. Consiglio: **A**.

Il proprietario, il 2026-09-08: *«A, ma si fa anche la prova con la telecamera per spostarle con mediapipe?»* (decisione 24). Da lì
la **mossa 8** — la pinza dal worker e dal relay di SP-7 com'è, tradotta in eventi del puntatore, `dockview` in
`dndStrategy: 'pointer'`, provata nel browser — e la riga **Q4**:

> Aggiungere la mossa 8 e la riga Q4 alla sezione 4? **A:** sì, così: la sezione si scrive con otto mosse. **B:** no, la mano resta
> tutta al 12: la sezione si scrive con sette mosse. Consiglio: **A**.

Il proprietario, il 2026-09-09: **A** (decisione 25).

Poi la **sezione 3**, i disegni nuovi, ciascuno A/B coi tre controlli della decisione 18, a parole e col diagramma reso dal
sorgente identico (decisione 21). ⛔ **Il materiale è già letto, e sta qui perché non si rilegga da capo** — si riverifica
coi comandi, non si cita:

| Per | Che cosa esiste, letto il 2026-09-08 | Dove |
|---|---|---|
| l'`erDiagram` di `docs/design/10-modello-dei-dati-durevoli.md` | `RecordV1` ha **sei** campi con indice: `kind` (0), `effect` (1), `trust` (2), `payload` (3, byte), `reason` (4), `detail` (5, `Option<Detail>`, facoltativo, indice nuovo); `RecordKind` sei varianti — `Intent`, `Outcome`, `Note`, `Verdict`, `Routing`, `Permission`; `Detail` tre specie — `Verdict` (`passed`, `spent_millis`), `Routing` (`model`, `evaluated`, `degraded`), `Permission` (`tool`, `resource`, `write`); `Record` è l'enum di versione con la sola `V1` | `crates/kernel/src/record.rs`: `grep -n -A1 '#\[n(' crates/kernel/src/record.rs` |
| idem, l'archivio | la tabella `redb` del giornale: chiave `u64` progressiva → (`step: u64`, `kind: u8`, i byte del record); `FileJournal` porta `next_key` | `crates/platform/src/journal.rs`, la riga `const RECORDS: TableDefinition` |
| idem, i byte congelati | sei record, uno per `RecordKind`, più la mappa | `ls crates/kernel/tests/frozen/` |
| idem, ciò che arriva | col 2 la specie `Invocation` (funzione, invocatore, argomento) e la chiave della disposizione (enum chiuso, una variante, valore opaco); col 3 run e sub-run — oggi non esiste `RunId`, solo `StepId`; col 4 il piano; col 5 la versione conservata dal checkpoint, riferita dal passo, dentro un ambito dichiarato; col 6 la cartella della knowledge base e il suo indice — nodi router · gruppo · foglia · skill · guida-modello · cattura; attributi percorso · specie · etichetta · provenienza · ultima modifica; frecce router → gruppo → foglia → skill → router; col 13 le guide approvate | §5 del 2 · §2 qui sopra · ADR-0024, Decision 1–2 · disegno della knowledge base §2.2 e §4.2 · design/09 e design/03 |
| «la GUI dentro» | design/01 disegna la GUI come una scatola, e la §3 qui sopra dice che deve restarlo; il modello è nella tabella «Il modello della GUI», e le tre sequenze — l'accoglienza (`Hello` → `Accepted` → `Degradation`, `Policy`, `Layout`), il giro salva/riavvia/ritrova (`SaveLayout` → settima porta → `Layout`, e il vecchio se la scrittura fallisce), l'invocazione (`Invoke` → `PermissionRequired` → `Approve` → `Invoke` → `Policy`) — non hanno un diagramma. 🔶 **Giudizio di chi ha letto, non deciso:** la casa è **questa stella polare**, come `sequenceDiagram` accanto alla tabella del modello — non un file di `docs/design/`, che è la struttura del kernel, né design/01. ✅ **RICHIAMO DEL 2026-09-08, nona ripresa: deciso A (decisione 23) e scritto** | §4, §5, §6a del 2 · §2 qui sopra · la tabella «Cosa è già studiato, e dove» |
| il cancello | un file nuovo in `docs/design/` va nell'indice di `README.md`: `check-docs.sh` conta i file e le righe della tabella che linkano `design/`, e devono coincidere; e ogni riga `Q` di design/08 deve avere un metodo (V30) | `scripts/check-docs.sh`, le righe `d_file=` e `d_idx=` |

Poi le sezioni 4–6 e le §7–§10 del 2, come nel punto 4 del verbale.

#### I diagrammi, uno per uno — letti il 2026-09-08

| File | Diagrammi | Che cosa cambia | Sezione |
|---|---|---|---|
| [design/09](../design/09-l0-fisico.md) | 3 flowchart, 1 stateDiagram | «Gli archivi»: la disposizione, le guide, la policy, i permessi e le transizioni nel giornale, l'indice della mappa; la colonna «Chi lo raggiunge»; Telecamera in «riservato»; due regole | **1**, ✅ scritta il 2026-09-08 |
| [ADR-0022](../adr/0022-layout-dei-dati-per-natura-e-backup-dichiarato.md) | tabelle | rimando in testa: le guide sono file della knowledge base; la politica della riga non cambia | 1, ✅ scritta |
| [ADR-0006](../adr/0006-due-policy-vram-come-oggetti-distinti.md) | — | rimando in testa: il profilo dà il default, il giornale il corrente (decisione 17) | 1, ✅ scritta |
| [design/02](../design/02-arbitrato-gpu.md) | 1 flowchart, 1 stateDiagram | l'etichetta «cambio di profilo di configurazione» e la frase «determinata dal profilo» (righe 235 e 249 al 2026-09-08) | 1, ✅ scritta |
| [design/05](../design/05-gateway-inferenza.md) | 3 flowchart | la riga «Policy VRAM · chi la cambia: il profilo di configurazione» (riga 153 al 2026-09-08) | 1, ✅ scritta |
| [design/07](../design/07-osservabilita-e-degrado.md) | 2 flowchart | «Cosa deve essere sempre visibile» contro la striscia della §3: un richiamo | 2, ✅ scritta il 2026-09-08 — e non un richiamo solo: sette fonti del degrado, la GUI viva a GPU satura, le proiezioni a classi, «Dove» e «Chi» |
| [ADR-0019](../adr/0019-lo-stato-di-degrado-e-un-oggetto-osservabile.md) | tabella | rimando in testa: la lista degli eventi è aperta — il fallback dichiarato di ADR-0012 c'è già nel codice, la telecamera arriva con ADR-0039 — e a GPU satura resta viva anche la GUI (ADR-0033) | 2, ✅ scritta |
| [design/03](../design/03-run-durevoli.md) | 2 flowchart, 1 stateDiagram | l'invocazione del registro come passo; il giornale separato dagli altri archivi, coi «(col N)»; le note nel ciclo di vita; il ramo «non dichiarata» → «non leggibile» | 2, ✅ scritta il 2026-09-08, sesta ripresa |
| [design/08](../design/08-strategia-di-test.md) | 1 flowchart | la GUI nello strato deterministico (col 2); le suite di conformità e la regola che le fa crescere; le sei campagne DST; la riga Q3; lo stato delle Q lasciato alla §8.4 della spec; il «ciclo lungo» della DST profonda che non esiste; ADR-0026 | 2, ✅ scritta il 2026-09-08, settima ripresa |
| [design/01](../design/01-topologia-dei-processi.md) | 1 flowchart, 1 stateDiagram | la GUI è una scatola e resta tale; la GUI dentro è un disegno nuovo | 3 |
| [design/04](../design/04-anelli-e-sensori.md) · [design/06](../design/06-permessi-e-confine-dei-dati.md) | 2 · 3 | 🔶 nulla, giudizio di questa sessione | — |
| [design/10](../design/10-modello-dei-dati-durevoli.md) | 2 erDiagram | **nuovo**: il giornale a entità com'è nel codice — passo, voce, record, dettaglio, coi due «kind» — e ciò che è deciso per sotto-progetto, con «(col N)»; chi scrive oggi ogni specie; le regole; i comandi | 3, ✅ scritto il 2026-09-08, ottava ripresa |
| questo file, «La GUI dentro» | 3 sequenceDiagram | **nuovo**: l'accoglienza, il giro salva/riavvia/ritrova, l'invocazione — il protocollo core ↔ GUI con la GUI come una scatola sola; due correzioni alla consegna del 2 trovate disegnando | 3, ✅ scritto il 2026-09-08, nona ripresa |
| tutto `docs/` | — | nessun diagramma a entità: `grep -rE '^\s*erDiagram' docs/` è vuoto. ✅ **RICHIAMO DEL 2026-09-08, ottava ripresa:** ora ne restituisce due, in design/10; ✅ **e alla nona** i `sequenceDiagram` sono tre, in questo file. Dove vive ogni diagramma lo dice il comando `grep -rlE '^\s*(flowchart|stateDiagram|erDiagram|sequenceDiagram)' docs`, e i tre wireframe SVG `ls docs/superpowers/specs/2026-09-07-direzione-gui-wireframes/` | 3 |

#### La proposta per la sezione 1 — presentata il 2026-09-08, NON approvata, NON scritta

✅ **RICHIAMO DEL 2026-09-08, quinta ripresa: APPROVATA (A) E SCRITTA, con due correzioni** — decisione 18: nel nodo «giornale»
anche «guide approvate (col 13)»; nella cella «Chi lo raggiunge» della configurazione **due vie**, i profili consegnati dal daemon
(ADR-0034) e la disposizione dalla settima porta. ⛔ **Il sorgente del diagramma vive ora in [design/09](../design/09-l0-fisico.md)
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
3. **Questo file per intero**, poi la [consegna del 2](../superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md)
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

## La dodicesima ripresa, 2026-09-09 — la cronaca, scritta alla chiusura

⚠️ **Dal taglio 1 in poi la cronaca di ogni ripresa si scrive qui**, non nel documento vivo (decisione 27 del proprietario,
36 del coordinatore). Ciò che è vero adesso sta nella stella polare.

1. Ripresa con `anthropic-skills:decision-principles` e `anthropic-skills:session-resume`, più `dev-discipline` e
   `dev-communication`: `git fetch`, stato riletto coi comandi, nessuna divergenza dalla consegna dell'undicesima ripresa;
   `check-docs.sh` OK e `GATE GREEN` all'apertura. La lettura obbligatoria letta a blocchi in parallelo (quattordici chiamate).
2. Il mandato (decisione 26) come primo passo: la lettura d'apertura **rimisurata** coi comandi del prossimo passo — 551 293
   byte e 179 219 token `cl100k` sui cinque file — e il peso per blocco; l'ordine proposto: stella polare, audit, compendio,
   `CLAUDE.md`, la consegna del 2 com'è.
3. **Taglio 1, A (decisione 27):** «Stato in una riga», «Da sapere subito», la tabella dello stato e «Fatto in questa
   sessione» com'erano, e il «Prossimo passo» intero, in questo file coi link riscritti; nel vivo lo stato in poche righe, i
   tre paragrafi vivi, la tabella coi comandi (più la riga dei diagrammi), il solo prossimo passo vivo. Commit `95c9857`.
4. **Taglio 2, A (decisione 28):** la lettura dell'audit ristretta alla tabella delle voci senza numero AUD e alla disciplina
   in cinque passi (4 806 byte contro 82 907); le tre case dell'istruzione riscritte col richiamo datato e i testi vecchi in
   `docs/archivio/lettura-di-apertura-storico.md`; la cella «R1 — APERTA» dell'audit e la sua testa corrette. Commit `b5414a7`.
5. **Taglio 3, A (decisione 29):** ogni voce del riquadro «Le voci ancora aperte» della §6 censita una per una contro le
   tabelle uniche del registro (T5: 34 righe, 5 chiuse; T6: 29), la tabella dell'audit e la tabella del 2026-08-10 — 32
   righe in chat, due orfane (la regola di rimisura del Task 9; i pesi a mano di `AVVIO-CHAT.md`) — e il racconto in
   `docs/archivio/stato-storico.md` col richiamo nella sua testa e nel registro; nella §6 gli indici coi due comandi `awk`,
   la tabella com'era più le due righe, il ritratto pieno; il tetto di `check-docs.sh` da 188 416 a 111 616. Il cancello ha
   colto due link doppiati e uno vuoto prima del commit. Commit `0b20318`.
6. Rimisurato dopo i tre tagli: 324 341 byte e 105 267 token `cl100k` (il comando: lo snippet `tiktoken` di `CLAUDE.md`
   esteso ai cinque file, l'audit ridotto ai due pezzi). La memoria dell'agente sui blocchi di lettura aggiornata.
7. La **domanda 4** — riscrivere il puntatore «Il prossimo passo» della §6 allo stato di oggi — posta in A/B col consiglio A
   e lasciata **senza risposta**: il proprietario ha chiesto la chiusura con `session-handoff` («appena concludi»).
8. Chiusura: la stella polare (stato in una riga, «Da sapere subito», la tabella dello stato, le decisioni 27–29 del
   proprietario e 36–42 del coordinatore, due registrate, i vicoli ciechi, il prossimo passo col blocco della domanda 4 e
   l'abbozzo della domanda 5), questa cronaca, il richiamo nel puntatore della §6 del compendio; `check-docs.sh` OK e il
   cancello rilanciato; scratchpad pulito; nessun codice toccato.

## La tredicesima ripresa, 2026-09-09 — la cronaca, scritta man mano

⚠️ **Dal taglio 1 in poi la cronaca di ogni ripresa si scrive qui**, non nel documento vivo (decisione 27 del proprietario,
36 del coordinatore). Ciò che è vero adesso sta nella stella polare.

1. Ripresa con `anthropic-skills:decision-principles` e `anthropic-skills:session-resume`, più `dev-discipline` e
   `dev-communication`: `git fetch`, stato riletto coi comandi della tabella dello stato; nessuna divergenza dalla consegna
   della dodicesima ripresa, tranne la misura della lettura d'apertura, cresciuta col commit di chiusura stesso (la stella
   polare da 157 266 a 167 456 byte). `check-docs.sh` OK e `GATE GREEN` all'apertura. Letti il compendio intero, i due pezzi
   dell'audit e le sole parti di stato della stella polare, a blocchi misurati prima coi byte per blocco, nessuna chiamata
   traboccata.
2. La lettura d'apertura rimisurata coi comandi del punto 1 del prossimo passo: 335 806 byte e 108 738 token `cl100k` sui
   cinque pezzi.
3. La **domanda 4** riposta al proprietario com'era, con la misura del giorno — il puntatore, dal titolo alla sottosezione
   seguente: 7 195 byte, `awk '/^### Il prossimo passo/{s=1} s&&/^### Le voci ancora aperte/{s=0} s{b+=length($0)+1} END{print b}'
   docs/COMPENDIO.md`. **Risposta A (decisione 30):** il puntatore riscritto allo stato di oggi (4 231 byte con lo stesso
   comando), il testo com'era in `docs/archivio/stato-storico.md` coi link riscritti; l'intestazione del compendio nomina il
   taglio; la riga del comando nella tabella dello stato della stella polare; decisione 43 del coordinatore (il comando dei
   finding chiusi resta nella §6, fuori dal taglio) e una registrata nuova (quanto della stella polare si legga all'apertura).
   Qui sotto, com'erano, il paragrafo di «Da sapere subito» e il blocco della proposta.
4. **Taglio 5, A (decisione 31):** misurati prima i sette pezzi di cronaca di `CLAUDE.md` — 2 173 byte e 753 token su
   5 019, contro la stima di ~2 000 token della dodicesima ripresa: divergenza detta al proprietario prima della domanda, e il
   consiglio A dato per coerenza (la regola di `CLAUDE.md` sui verbali) e non per i token. I sette pezzi in
   `docs/archivio/lettura-di-apertura-storico.md` con la data e dove stavano; `CLAUDE.md` da 15 892 a 14 412 byte, ogni regola
   col suo perché e il rimando «il verbale in archivio» (decisione 44 del coordinatore); il puntatore della §6 del compendio
   e la sua intestazione aggiornati allo stato del mandato. Qui sotto, com'erano, il paragrafo di «Da sapere subito» dopo il
   taglio 4 e il paragrafo della domanda 5.
5. **La domanda su `AVVIO-CHAT.md`, B (decisione 32):** il proprietario non lo incolla più; il messaggio resta com'è (370
   righe e 25 577 byte con l'`awk` della §12 del compendio) e non è più lettura d'apertura; la voce dei pesi a mano nella
   tabella del 2026-08-10 della §6 del compendio e la registrata della dodicesima ripresa chiuse con la decisione; il
   richiamo in testa ad `AVVIO-CHAT.md` registrato al proprietario. **Il mandato (decisione 26) è eseguito.** Rimisurato
   dopo i cinque tagli: 334 300 byte e 108 137 token `cl100k` — i tagli 4 e 5 hanno reso ~600 token netti, perché la stella
   polare è cresciuta di ~900 con le righe delle decisioni e dello stato (decisione 45 del coordinatore: il punto 1 del
   prossimo passo, la misura datata e il metodo, esce qui sotto e resta una riga coi comandi). Qui sotto, com'erano, il
   paragrafo di «Da sapere subito» dopo il taglio 5, il paragrafo della domanda su `AVVIO-CHAT.md` e il punto 1.
6. La **§7 del 2** riposta al proprietario parola per parola dal blocco *«La proposta per la sezione 5»* della stella
   polare, con la verifica che il codice è intatto da `664265a`; lette prima le §1–§6a della consegna del 2, le sue sezioni
   che mancano, le §2–§3 della stella polare e le sue sezioni che mancano. **Senza risposta:** il proprietario ha chiesto la
   chiusura con `session-handoff` («continuiamo nella prossima sessione»).
7. Chiusura: la stella polare (stato in una riga, «Da sapere subito», il prossimo passo con l'ordine della sessione nuova e
   le letture che bastano, il richiamo sotto il blocco della §7, quattro vicoli ciechi), questa cronaca; `check-docs.sh` OK
   e il cancello rilanciato e letto dal log (`GATE GREEN`); scratchpad pulito; la memoria dell'agente aggiornata; nessun
   codice toccato. I commit della ripresa: `b0901e4` (taglio 4), `33b9b78` (taglio 5), `fa84516` (la chiusura del mandato),
   e il commit di questa chiusura.

### Il paragrafo di «Da sapere subito» com'era alla chiusura della dodicesima ripresa

✅ **La dodicesima ripresa, il 2026-09-09, ha eseguito TRE tagli del mandato, tutti approvati A (decisioni 27–29):** la
cronaca di questo file in archivio; la lettura dell'audit ristretta a due pezzi, con le tre case dell'istruzione riscritte
(`CLAUDE.md`, la §6 del compendio, la voce 3 del messaggio di `AVVIO-CHAT.md`) e i testi vecchi in
`docs/archivio/lettura-di-apertura-storico.md`; il riquadro delle voci aperte della §6 del compendio ridotto agli indici dopo
il censimento voce per voce (32 righe in chat, due orfane), il racconto in `docs/archivio/stato-storico.md`, il tetto di
`check-docs.sh` sceso. La lettura d'apertura, rimisurata coi comandi del punto 1 del prossimo passo: da 551 293 a 324 341 byte,
da 179 219 a 105 267 token `cl100k` — il dopo lo rifà il comando. ⛔ **Alla chiusura, chiesta dal proprietario con
`session-handoff` («appena concludi»), la domanda 4 — il puntatore della §6 — è POSTA e SENZA risposta:** sta nel prossimo
passo, parola per parola. `check-docs.sh` OK e `GATE GREEN` all'apertura; alla chiusura `check-docs.sh` OK e `GATE GREEN`
rilanciato e letto dal log. Nessun codice toccato; toccato `scripts/check-docs.sh`, il tetto.

### La proposta per il taglio 4, com'era — posta alla dodicesima ripresa, chiusa A alla tredicesima

#### La proposta per il taglio 4 — il puntatore «Il prossimo passo» della §6 · posta il 2026-09-09, dodicesima ripresa, SENZA risposta

> **Domanda 4 — il puntatore «Il prossimo passo» della §6** (6,8 KB). Oggi è una catena di ✅ su cose chiuse: sotto-progetto 1,
> piano dei gesti, knowledge base, sezioni 1–4, passata sui diagrammi, più il paragrafo sul racconto del Traguardo 6 uscito.
>
> - **A:** lo riscrivo allo stato di oggi, in poche righe: cosa è chiuso (con la data e il link, senza il racconto), il mandato in
>   corso, e il prossimo passo vivo (la §7 del 2, poi §8–§10, la sezione 6, i due disegni, il piano in due parti, ADR-0029 con
>   M1–M5, AUD-004 in parallelo). Il testo com'è va in `docs/archivio/stato-storico.md`, parola per parola. Da 6,8 KB a ~2,5 KB.
> - **B:** resta com'è.
>
> **Consiglio: A.** È l'unico posto dove vive il prossimo passo: più è corto, meno invecchia.

### Il paragrafo di «Da sapere subito» com'era dopo il taglio 4

✅ **Il mandato alla tredicesima ripresa, il 2026-09-09: QUATTRO tagli eseguiti, tutti approvati A (decisioni 27–30):** la
cronaca di questo file in archivio; la lettura dell'audit ristretta a due pezzi; il riquadro delle voci aperte della §6 del
compendio ridotto agli indici; il puntatore «Il prossimo passo» della §6 riscritto allo stato di oggi — da 7 195 a 4 231 byte,
il comando nella tabella dello stato — e il testo com'era in `docs/archivio/stato-storico.md`. Resta `CLAUDE.md`, la domanda
5; poi `AVVIO-CHAT.md`, se il proprietario lo incolla ancora. La lettura d'apertura, rimisurata all'apertura coi comandi del
punto 1 del prossimo passo: 335 806 byte e 108 738 token `cl100k` — il dopo lo rifà il comando. `check-docs.sh` OK e
`GATE GREEN` all'apertura. Nessun codice toccato. Il paragrafo che stava qui, sulla dodicesima ripresa, è in archivio.

### La domanda 5 com'era, nel prossimo passo — abbozzata alla dodicesima ripresa, chiusa A alla tredicesima

⏭️ **La domanda 5, da porre adesso che la 4 è chiusa — `CLAUDE.md` (15,9 KB):** i verbali dentro la testa e dentro le righe delle tabelle — le
sei volte del gotcha #31 sul peso della lettura, il richiamo del 2026-08-30 sulla riga di `writing-plans`, le storie dei fine-riga
e di G-5, il capoverso del 2026-08-28 sotto le quattro domande — escono in `docs/archivio/lettura-di-apertura-storico.md`; ogni
**regola** resta, con una riga di perché. Stima: a ~10 KB. È il documento del proprietario: ogni riga tolta è sua, A/B. Il guadagno
è piccolo (~2 000 token) e va detto. Poi, se il proprietario incolla ancora il messaggio di `AVVIO-CHAT.md` (26 224 byte, comando
nella §12 del compendio), lo stesso taglio anche lì; se non lo incolla più, resta com'è e si dice.

### Il paragrafo di «Da sapere subito» com'era dopo il taglio 5

✅ **Il mandato alla tredicesima ripresa, il 2026-09-09: CINQUE tagli eseguiti, tutti approvati A (decisioni 27–31):** la
cronaca di questo file in archivio; la lettura dell'audit ristretta a due pezzi; il riquadro delle voci aperte della §6 del
compendio ridotto agli indici; il puntatore «Il prossimo passo» della §6 riscritto allo stato di oggi — da 7 195 a 4 231 byte,
il comando nella tabella dello stato — col testo com'era in `docs/archivio/stato-storico.md`; `CLAUDE.md` sfoltito dei sette
verbali — da 15 892 a 14 412 byte, `wc -c` — che stanno in `docs/archivio/lettura-di-apertura-storico.md` con la data e dove
stavano, ogni regola rimasta col suo perché (misurato prima di chiedere: 2 173 byte e 753 token, contro la stima di ~2 000
token della dodicesima ripresa — la divergenza è nella decisione 31). Resta la domanda su `AVVIO-CHAT.md`: se il proprietario
lo incolla ancora, lo stesso taglio anche lì; se no, il mandato si chiude. La lettura d'apertura, rimisurata all'apertura coi
comandi del punto 1 del prossimo passo: 335 806 byte e 108 738 token `cl100k` — il dopo lo rifà il comando. `check-docs.sh`
OK e `GATE GREEN` all'apertura. Nessun codice toccato. I paragrafi che stavano qui — sulla dodicesima ripresa, e dopo il
taglio 4 — sono in archivio.

### La domanda su `AVVIO-CHAT.md` com'era, nel prossimo passo — posta e chiusa B alla tredicesima ripresa

⏭️ **La domanda su `AVVIO-CHAT.md`, da porre adesso che la 5 è chiusa:** il messaggio fra le due recinzioni — il comando nella
§12 del compendio lo misura — porta ancora pesi scritti a mano e verbali (la registrata della dodicesima ripresa, e la riga della
tabella del 2026-08-10 della §6 del compendio). Prima si chiede se il proprietario lo incolla ancora all'inizio delle chat: se
sì, si misura e si propone lo stesso taglio in A/B — i verbali in `docs/archivio/lettura-di-apertura-storico.md`, i pesi
sostituiti dai comandi; se no, resta com'è, lo si dice qui, e il mandato si chiude.

### Il punto 1 del prossimo passo com'era — la misura datata dell'undicesima ripresa e il metodo

1. ⛔ **Prima di tutto, il mandato: sfoltire la lettura d'apertura.** Misurata alla chiusura, il 2026-09-09, coi comandi qui sotto:
   la lettura obbligatoria — `CLAUDE.md`, il compendio intero, la testa dell'audit fino a «Dettaglio», questo file intero, la
   consegna del 2 — pesa **536 KB** e **174 387 token** `cl100k` (limite inferiore: su italiano con emoji Claude conta di più), e la
   sessione ne occupa **~350 000** appena comincia (misura del proprietario, che comprende le skill e il prompt di sistema, che
   nessun comando qui misura). Dove pesa, in byte, blocco per blocco:

   | Documento | Peso | Ciò che è cronaca, o chiuso | Dove va |
   |---|---|---|---|
   | `docs/COMPENDIO.md` | 182 701 B · 59 534 token | la §6 è **104 078 B**, di cui **93 878** il riquadro «Le voci ancora aperte»: blocchi tenuti parola per parola perché nominano decisioni del proprietario — il **debito dichiarato** dello sfoltimento del 2026-08-28, che prevedeva la consolidazione in una tabella sola *«da presentare al proprietario una per una»* | la consolidazione: **voce per voce** col proprietario, in A/B; il racconto in `docs/archivio/stato-storico.md`; il tetto di `check-docs.sh` scende con lo sfoltimento (gotcha #100) |
   | questo file | 207 902 B · 67 280 token | cronaca: «Da sapere subito» **11 991**, «Fatto in questa sessione» **19 424**, «Prossimo passo, eseguibile» **30 947**; merito: «Le sezioni approvate» 59 911, le due tabelle delle decisioni 31 178, «Il modello della GUI» 13 174 | la cronaca in `docs/archivio/consegna-brainstorming-direzione-gui.md` (o un file gemello), parola per parola coi link riscritti; qui restano lo stato in poche righe, le decisioni, le sezioni approvate, le registrate, i vicoli ciechi ancora utili e il solo prossimo passo **vivo** |
   | `docs/audit-2026-08-27.md`, righe 1–450 | 82 907 B · 27 475 token | «Stato dei rimedi» **36 121** e «I 73 finding» **19 591**: tutti chiusi (`awk -F'\|' 'NF>4{gsub(/^ +\| +$/,"",$5); print $5}' docs/audit-2026-08-27.md \| grep -c aperto` → 0) | la lettura d'apertura si restringe a «Come si concludono quelli aperti» e alla tabella delle voci senza numero AUD; è una riga di `CLAUDE.md`, quindi del **proprietario** |
   | `docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` | 46 757 B · 15 078 token | poco: le sezioni approvate sono merito | resta; i richiami datati si rileggono quando il disegno del 2 si scrive |
   | `CLAUDE.md` | 15 914 B · 5 020 token | i richiami lunghi nella testa e nelle tabelle | è il modo di lavorare: ogni riga tolta è del **proprietario**, A/B |

   I comandi che rifanno la misura — byte: `wc -c CLAUDE.md docs/COMPENDIO.md docs/superpowers/specs/2026-09-07-direzione-gui-design.md
   docs/superpowers/specs/2026-09-06-sottoprogetto-2-gui-minima-design.md` e `sed -n '1,450p' docs/audit-2026-08-27.md | wc -c`;
   token: lo snippet `tiktoken` di `CLAUDE.md`, esteso ai cinque file (`tiktoken` è installato su questa macchina); per blocco:
   `awk '/^## /{if(h!="")printf "%7d  %s\n", b, h; h=$0; b=0} {b+=length($0)+1} END{printf "%7d  %s\n", b, h}' <file>`.
   **Il metodo, e le regole che non si negoziano:** si misura di nuovo prima di tagliare; per ogni documento una proposta A/B —
   che cosa esce, dove va, che cosa resta — e si scrive solo dopo il sì; **niente si cancella**: un verbale va in `docs/archivio/`
   parola per parola con la data e i link riscritti (regola di `CLAUDE.md`); nessuna decisione del proprietario si riassume
   d'iniziativa; i puntatori restano in una casa sola (gotcha #68); `bash scripts/check-docs.sh` dopo ogni file e i fine-riga
   rimisurati (`git ls-files --eol`: questo file e la consegna del 2 `w/lf`, il compendio e l'archivio `w/crlf`); la memoria
   dell'agente sui blocchi di lettura si aggiorna quando le taglie cambiano.

## La quattordicesima ripresa, 2026-09-09 — la cronaca, scritta man mano

Aperta con `/anthropic-skills:decision-principles /anthropic-skills:session-resume`, senza il messaggio di `AVVIO-CHAT.md`
(decisione 32). Ripresa coi comandi: `git fetch --all --prune` e `git status -sb` (`## main...origin/main`, niente sotto),
nessuno stash, nessuna operazione a metà; `check-docs.sh` OK e `GATE GREEN` all'apertura, letto dal log; la tabella dello stato
di questo file rilanciata riga per riga, nessuna divergenza — `git diff --stat 664265a..HEAD` sul codice dà la spec del
sotto-progetto 1 e `scripts/check-docs.sh`, com'è scritto; i fatti del codice su cui poggia la proposta riletti **prima** della
domanda: `gui/` non esiste, `exclude = ["spikes"]` nel manifesto di radice, nessun `impl Ipc for` fuori dai commenti in `crates/`,
`Executor::spawn` prende future con vita `'a`, `executor_determinism.rs` fa girare più attività su una `RefCell`,
`gui_death_campaign.rs` sta in `crates/simulator/tests/`. La lettura d'apertura com'è scritta in `CLAUDE.md` — il compendio
intero, i due pezzi dell'audit — più le parti di stato di questo file, «La GUI dentro», la §3, e la consegna del 2 intera; i
blocchi misurati prima con l'`awk` per byte, due chiamate traboccate (la memoria dell'agente ne porta la taglia).

La domanda sulla §7 riposta al proprietario a parole semplici, A/B col consiglio A; la risposta: **A**. Fatto nello stesso
commit: la §7 scritta nella consegna del 2 sotto «Le sezioni approvate del disegno», nella forma delle §3–§6a (Pezzo · Forma ·
La prova), coi tre controlli della decisione 18 e il controllo sui cinque criteri riletti nel codice; la riga 7 di «Le sezioni che
mancano» della consegna e la riga 5 di questo file col richiamo; la decisione 33 nella tabella; «Stato in una riga», «Da sapere
subito» e il prossimo passo di questo file allo stato di oggi — si riprende dalla **§8**; il punto 1 del puntatore della §6 del
compendio e la sua intestazione. Trovato scrivendo, e registrato come dedotto nella §7: `build_the_arbiter` e `reserve` vivono in
`crates/daemon/src/main.rs`, un binario, quindi il finto non può importarli — dove spostarli lo decide il piano. Nessun codice
toccato. I testi com'erano stanno qui sotto.

### Il paragrafo di «Da sapere subito» com'era alla chiusura della tredicesima ripresa

⛔ **Il prossimo passo è la §7 del 2**, dalla domanda A/B lasciata aperta: il come, coi comandi, sta in «Prossimo passo,
eseguibile». ⛔ **Alla chiusura della tredicesima ripresa (2026-09-09) la domanda è stata RIPOSTA al proprietario parola per
parola, con la verifica che il codice è intatto da `664265a`, e resta SENZA risposta:** il proprietario ha chiesto la chiusura
con `session-handoff` («continuiamo nella prossima sessione»).

### Le due frasi di «Stato in una riga» com'erano alla chiusura della tredicesima ripresa (frammenti)

**Mancano la sezione 5** — le §7–§10 del 2: core finto, prove e cancello,
decisioni aperte, come si riprende — **e la 6**

⛔ **La §7 del 2 è stata RIPOSTA in forma A/B alla chiusura della tredicesima ripresa, chiesta dal proprietario con
`session-handoff`, e resta SENZA risposta**: si riprende da lì, dal blocco *«La proposta per la sezione 5»* del prossimo
passo.

### Il paragrafo del prossimo passo com'era, e il suo punto 2

⛔ **Alla chiusura della tredicesima ripresa (2026-09-09), chiesta dal proprietario con `session-handoff`, la domanda sulla §7 è
stata RIPOSTA parola per parola e resta SENZA risposta.** ⏭️ **L'ordine della sessione nuova:** la risposta A o B al blocco *«La
proposta per la sezione 5»* qui sotto; poi la §7 scritta nella consegna del 2 sotto «Le sezioni approvate del disegno», nella
forma delle §3–§6a (Pezzo · Forma · La prova), e la riga 7 di «Le sezioni che mancano» della consegna col richiamo;
`check-docs.sh`, commit, push; poi il punto 3 (la §8), il 4 (§9, §10 e la sezione 6), il 5.

2. Al proprietario, **la domanda A/B sulla §7**, dal blocco *«La proposta per la sezione 5 — §7, il core finto»* qui sotto,
   parola per parola; consiglio **A**. Se A o B: scrivere la §7 nella consegna del 2, sotto «Le sezioni approvate del disegno»,
   nella forma delle §3–§6a (Pezzo · Forma · La prova), la riga 7 di «Le sezioni che mancano» della consegna col richiamo;
   `check-docs.sh`, commit, push.

### La proposta per la sezione 5 com'era — posta all'undicesima ripresa, riposta alla tredicesima, chiusa A alla quattordicesima

#### La proposta per la sezione 5 — §7, il core finto · presentata il 2026-09-09, undicesima ripresa, SENZA risposta

✅ **Richiamo del 2026-09-09, tredicesima ripresa:** riposta al proprietario com'è, parola per parola, dopo la verifica che il
codice è intatto da `664265a` (`git diff --stat 664265a..HEAD` su `crates/`, `scripts/`, `spikes/`: solo il tetto di
`check-docs.sh`); senza risposta alla chiusura, chiesta con `session-handoff`.

**A parole.** Un programma piccolo in `gui/fake-core/` che finge di essere il core, così la GUI si costruisce e si prova prima
che il daemon vero faccia tutto e prima che esista un modello. Parla sul filo vero con lo schema vero. Già approvato (§1 e la
riga 7 della consegna del 2, la §3 di questo file): fuori dal workspace Cargo (`gui` in `exclude`), il suo `Cargo.lock`
committato perché lo usa il cancello, token a tempo come lo spike (2000 in dieci secondi, testo non fidato, markdown), `Invoke`
→ `PermissionRequired` la prima volta, `Layout`/`SaveLayout` e la lista dei passi.

**La domanda: quanto è finto?**

> **A — riusa.** Il finto fa girare l'attività **vera** del kernel che ascolta la GUI — quella che il daemon farà girare, e che
> il 2 costruisce come attività del kernel perché la DST la muova con `DyingGui` (§5 del 2) — su porte in memoria: il giornale
> del simulatore, la finta del simulatore per la settima porta, un arbitro vero, e il trasporto vero di `platform`. Finto è solo
> un **rubinetto**: un'attività che manda i token a tempo e, su un comando di una parola da stdin, un `Verdict` o un record di
> routing degradato nel giornale in memoria. Tutto il resto — `Hello`/`Accepted`/`StaleBuild`, `Degradation`, `Policy`,
> `Layout`/`SaveLayout`, `Invoke`/`PermissionRequired`/`Approve`, la lista dei passi — è il codice vero.
> **B — imita.** Un copione a sé che parla lo schema e il filo e scrive a mano ogni risposta del daemon (la riga 7 della consegna
> del 2 com'è scritta).
> Consiglio: **A** — il modo del repo è la logica vera su porte sostituite, cioè il simulatore; B è una seconda copia del
> dispaccio, e il giorno che il daemon cambia il finto diverge senza che nulla diventi rosso.

**Il costo di A, dichiarato:** il finto dipende anche da `simulator` per percorso; l'attività del kernel deve potersi costruire da
fuori, cosa che la campagna DST della §5 già pretende; il rubinetto condivide il trasporto con l'attività attraverso una `RefCell`,
il modo dell'esecutore; la disposizione vive finché il finto non riparte — la persistenza vera è del daemon, provata dalla sonda
della §8.

**I tre controlli (decisione 18).** *Esiste:* l'emettitore di `spikes/gui-ipc/src/bin/core.rs` — righe JSON, tre canali,
sopravvive alla GUI che muore e accetta una riconnessione; `IpcMessage` con due varianti e otto sonde di andata e ritorno e di
rifiuto in `crates/kernel/tests/ipc_wire.rs`; `FakeGui` in `crates/kernel/tests/ports_are_implementable.rs` e `DyingGui` in
`crates/simulator/src/ipc.rs`; il dispaccio di `gui_death_campaign.rs` scritto **dentro il banco** (`accept`/`receive`/`send` in
riga), quindi nessuna attività «servi la GUI» esiste ancora nel kernel: la costruisce il pezzo 6 della §3; nessun trasporto in
`platform`; `gui/` non esiste; il manifesto di radice esclude solo `spikes`. *Arriva:* col 3 il core vero produce i token e il
rubinetto perde quel compito, ma resta per ogni modulo il cui produttore arriva dopo; col 12 il gesto come invocatore, stessa
attività. *Regge crescendo:* una variante nuova è un ramo del `match` nell'attività vera, e con A il finto la segue gratis; con
B è una risposta in più scritta a mano.

**Controllo sui cinque criteri.** Verificato il 2026-09-09: i file sopra, letti. 🔶 **Dedotto:** che un'attività e un rubinetto
possano condividere la porta con una `RefCell` — `Executor::spawn` prende future con vita `'a`, e la campagna dell'arbitro fa
già girare più attività su stato condiviso; che il valore di `Accepted` sia consegnato al finto come al daemon. **Assunto:**
niente. Debito scritto: il rubinetto è codice finto fuori dal prodotto; il suo «degrada» è un meccanismo vero con una causa finta.

> La §7 così? **A:** riusa — l'attività vera su porte in memoria, più il rubinetto. **B:** imita — un copione a sé. Consiglio: **A**.

### Seguito della quattordicesima ripresa, lo stesso giorno — la §8

Dopo il commit della §7 (`8fd8431`), la §8 presentata in chat a parole e con lo schema, coi cinque criteri e con verificato,
dedotto e assunto separati, e due domande A/B: il cancello unico col passo web dentro contro una CI web a parte (consiglio A), e
l'archivio della disposizione che non si apre all'avvio — la registrata della nona ripresa — parte e dichiara contro fermarsi
(consiglio A). Letti prima: `scripts/gate.sh`, la CI, `.gitignore`, «Cosa la porta NON controlla» e la sezione della sonda S3
in `porta-di-qualita.md`, le tabelle «artefatto → controllo» dei disegni dei gesti e della knowledge base, la §2 e la §4 di
questo file; alla fonte, la pagina di `actions/setup-node` (legge la versione da `package.json`, consiglia `v7`) e quella di
npm (`engine-strict`, default falso; se `npm ci` la onori non lo dice). Il proprietario ha risposto con la formula della
delega, «decidi secondo la skill»: decisioni 34 e 35, A e A, coi perché nella §8 e nelle decisioni 46–50 del coordinatore.
Fatto nello stesso commit: la §8 scritta nella consegna del 2; la riga 8 di «Le sezioni che mancano» della consegna e la riga
5 di questo file col richiamo; il richiamo datato sulla riga 5 della §2; la registrata chiusa; le tabelle delle decisioni; lo
stato e il prossimo passo di questo file — si riprende dalla §9; il puntatore della §6 del compendio. Nessun codice toccato.
I testi com'erano stanno qui sotto.

### Il punto 3 del prossimo passo com'era — la §8 proposta, chiusa A alla quattordicesima

3. La **§8** — prove e cancello: `scripts/gate-gui.sh` (il core finto con `cargo test --locked --manifest-path`, poi `npm ci`,
   `npm run build`, `npm test`) da una riga `run` di `gate.sh`; la CI con `actions/setup-node` appuntato; `.gitignore` per `gui/`
   e per le cartelle di build dello spike; la tabella artefatto → controllo dalle colonne «prova» delle §3–§6a più la sonda
   del core finto contro il trasporto vero da un thread, le prove della SPA sulle fixture, la sonda «salva, riavvia, ritrova»,
   la suite di conformità della settima porta, la campagna DST del 2 (§5), le otto mosse dello spike. Poi, come domanda a sé,
   la registrata della nona ripresa: **l'archivio della disposizione che non si apre all'avvio** — A: il core parte e lo
   dichiara con un terzo stato di `Layout`, «non disponibile», e ogni `SaveLayout` riceve lo stesso (richiamo datato sulla riga
   5 della §2 di questo file, che oggi dice «il pacchetto o niente»); B: il core si ferma come per il giornale (`StartupError`).
   Consiglio A, ADR-0019: si dichiara prima, non si fallisce dopo, e un archivio cosmetico non ferma il core.

### I paragrafi di stato com'erano dopo la §7, prima della §8 (frammenti)

**Manca il resto della sezione 5** — le §8–§10 del 2: prove e cancello, decisioni aperte, come si riprende; la §7, il core
finto, è **scritta** il 2026-09-09 (decisione 33, A) — **e la 6**

✅ **La §7 del 2 ha avuto risposta A alla quattordicesima ripresa (2026-09-09, decisione 33) ed è scritta nella consegna
del 2**: si riprende dalla **§8**, punto 3 del prossimo passo.

⛔ **Il prossimo passo è la §8 del 2** — prove e cancello — dal punto 3 di «Prossimo passo, eseguibile». ✅ **Alla
quattordicesima ripresa (2026-09-09) il proprietario ha risposto A alla §7** (decisione 33): la §7 è scritta nella consegna del
2, sotto «Le sezioni approvate del disegno», nella forma delle §3–§6a;

✅ **Alla quattordicesima ripresa (2026-09-09) la domanda sulla §7 ha avuto risposta A (decisione 33), e la §7 è scritta**
nella consegna del 2 sotto «Le sezioni approvate del disegno», nella forma delle §3–§6a, con la riga 7 di «Le sezioni che
mancano» della consegna col richiamo; il blocco della proposta com'era è nella cronaca in archivio. ⏭️ **L'ordine da qui:** il
punto 3 (la §8), poi il 4 (§9, §10 e la sezione 6), poi il 5.
