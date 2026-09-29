# La revisione della knowledge base — le chiusure di sessione archiviate

⚠️ **Verbali, veri il giorno in cui furono scritti.** Il documento vivo è la
[consegna della revisione](../superpowers/specs/2026-09-28-knowledge-base-revisione-design.md), che tiene solo
l'ultima chiusura di sessione — `CLAUDE.md`, *«Un verbale di correzione non resta nel documento corretto»*. Le chiusure
precedenti stanno qui, parola per parola, coi link riscritti per questa cartella.

⚠️ **Richiamo del 2026-09-29, all'apertura del disegno:** a quel percorso vive ora il **disegno** della revisione,
scritto sul posto; la consegna intera, con la sua ultima chiusura, sta in
[`consegna-brainstorming-knowledge-base-revisione-intera.md`](consegna-brainstorming-knowledge-base-revisione-intera.md).

## Archiviata il 2026-09-28, alla chiusura della sessione che ha aperto la revisione

La sezione scritta all'apertura del brainstorming, com'era nel commit `9bdbb59`.

## Come si riprende — scritto all'apertura del brainstorming, il 2026-09-28

⛔ **Niente è a metà.** Il commit di questo file porta anche il puntatore della §6 del compendio; albero pulito, tutto pushato.

| | Stato, e il comando che lo rifà |
|---|---|
| ramo | `main` allineato a `origin`: `git fetch --all --prune`, poi `git status -sb` |
| codice di prodotto | **non toccato**: `git diff --stat f830cb9..HEAD -- crates/ gui/ scripts/ Cargo.lock Cargo.toml` non rende nulla |
| cancello | `bash scripts/gate.sh` → `GATE GREEN` e `bash scripts/check-docs.sh` → `OK`, all'apertura sull'albero di `f830cb9` e di nuovo prima del commit di questo file: si rilanciano, non si citano |
| il puntatore | la §6 del compendio: la revisione **prima** del 13 e dei modelli decisionali |
| la guida ARMS | **non** è nel repository: il riassunto sta nella sezione *«La fonte del documento»*, la provenienza in [`riferimenti.md`](../riferimenti.md) |

**Il compito della sessione che riprende:**

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`.
2. La lettura obbligatoria di `CLAUDE.md`; poi **questo file per intero**, e il
   [disegno del 2026-09-04](../superpowers/specs/2026-09-04-knowledge-base-design.md) per intero — la §12 del compendio lo chiede a chi riprende il
   fronte della knowledge base.
3. La prima riga della tabella *«Le risposte del proprietario»* ancora ⏳ è la domanda da porre: si ripone **com'è scritta
   qui**, dopo aver rilanciato i comandi della tabella *«Che cosa esiste oggi»* — il codice può essersi mosso.
4. A ogni risposta: la riga nella tabella, un commit, un push. La domanda successiva si scrive **qui**, nella forma di D1, prima
   di porla.
5. Finite le domande: la chiusura del brainstorming — le decisioni prese, le registrate col chiusore — e il disegno in una
   sessione **nuova**, che scrive i richiami datati al disegno del 2026-09-04 e i rimandi agli ADR.

## Archiviata il 2026-09-29, alla chiusura della seconda sessione della revisione

La sezione scritta alla chiusura della sessione del 2026-09-28, com'era nel commit `5ba2ed5`.

## Come si riprende — scritto alla chiusura della sessione del 2026-09-28

⛔ **Da sapere subito: niente è a metà, ma D3 è senza risposta.** Albero pulito dopo il commit di questa chiusura, tutto
pushato, nessuno stash, nessun codice toccato. Il proprietario ha chiuso la sessione — *«continuiamo l'analisi e le domande
nella prossima sessione»* — sulla forma **riformulata** di D3, prima di leggerla: si ripone quella. La chiusura precedente sta
in [`archivio/consegna-brainstorming-knowledge-base-revisione.md`](consegna-brainstorming-knowledge-base-revisione.md).

| | Stato, e il comando che lo rifà |
|---|---|
| ramo | `main` allineato a `origin`: `git fetch --all --prune`, poi `git status -sb`; nessuno stash, `git stash list` |
| i commit di questa sessione | `git log --oneline f830cb9..HEAD`: l'avvio col puntatore della §6; il documento del proprietario col confronto; D2 con D3 posta; questa chiusura, con D3 riformulata |
| codice di prodotto | **non toccato**: `git diff --stat f830cb9..HEAD -- crates/ gui/ scripts/ Cargo.lock Cargo.toml` non rende nulla |
| cancello | `bash scripts/gate.sh` → `GATE GREEN` e `bash scripts/check-docs.sh` → `OK`, all'apertura sull'albero di `f830cb9` e prima di ogni commit: si rilanciano, non si citano |
| il puntatore | la §6 del compendio: la revisione **prima** del 13 e dei modelli decisionali — non cambia con questa chiusura |
| fine-riga | questo file e l'archivio della consegna **LF**; compendio, archivio dello stato e `riferimenti.md` LF nell'indice e **CRLF** nell'albero, coi CR uguali alle righe: `git ls-files --eol` sui file, e `tr -cd '\r'` contato contro `wc -l` |
| file temporanei | nessuno nel repository: gli script di questa sessione stanno nello scratchpad |
| la guida ARMS | **non** è nel repository: il riassunto sta nella sezione *«La fonte del documento»*, la provenienza in [`riferimenti.md`](../riferimenti.md) |

**Dove si è arrivati.** Lo stato vive nelle tabelle di questo file; qui c'è solo dove guardare.

| | |
|---|---|
| la forma della knowledge base | il **documento del proprietario**: una root qualsiasi, due attori, il livello strutturale e quello semantico, il riconciliatore meccanico, le esclusioni, il confinamento, il controllo prima di scrivere |
| che cosa cambia del 2026-09-04 | la tabella *«Il documento contro ciò che esiste»*: le risposte 1, 3, 4 e 10 |
| le risposte | D1 respinta, D2 **A** — la tabella *«Le risposte del proprietario»* |
| la domanda aperta | **D3 riformulata**: il modello a due zone, e *«una repo aperta come zona di lavoro entra nel grafo e nella ricerca?»*, col consiglio **A** |
| le domande dopo | D4–D10, nella tabella *«Le domande, una per volta»* |
| i buchi | K1–K34: la tabella *«Lo stato dei buchi dopo il documento»* e quella dei casi nuovi |

**Il compito della sessione che riprende:**

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è il commit di questa chiusura, o uno dopo.
2. La lettura obbligatoria di `CLAUDE.md`; poi **questo file per intero**, e il
   [disegno del 2026-09-04](../superpowers/specs/2026-09-04-knowledge-base-design.md) per intero — la §12 del compendio lo chiede a chi riprende il
   fronte della knowledge base.
3. Rilanciare i comandi della tabella *«Che cosa esiste oggi»*, e quello di D3 sulla porta `filesystem`: il codice può essersi
   mosso.
4. **Porre D3 com'è scritta qui**, nella forma riformulata. Poi D4–D10, una alla volta: ciascuna si scrive **qui**, nella forma
   di D2 e D3, **prima** di porla — contesto, A/B col costo e ciò che si rifà, i cinque criteri, verificato, dedotto e assunto,
   il consiglio —; in chat a parole semplici, poi `AskUserQuestion` con due opzioni e il consiglio per primo.
5. A ogni risposta: la riga nella tabella delle risposte, lo stato dei K che tocca, il cancello, un commit, un push.
6. Finite le domande: la chiusura del brainstorming e, in una sessione **nuova**, il disegno. Scrive i richiami datati al
   disegno del 2026-09-04 — le risposte 1, 3, 4 e 10, le decisioni 13 e 15 —; i rimandi agli ADR che la revisione tocca,
   ciascuno riletto contro i fratelli, gotcha #59; le righe di `roadmap.md` e di `tracciabilita.md`, fra cui
   `Multi-repo/multi-progetto` e `Mappa del progetto`; e la voce della §5 del compendio per ogni ADR che riceve un rimando.

**Le decisioni prese dal coordinatore in questa sessione, col perché** — il proprietario può ribaltarle:

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | i commit **senza** il trailer `Co-Authored-By` | `CLAUDE.md`, *«senza co-autore»*, prevale sulla direttiva di sistema. Costo: un `--amend` |
| 2 | le decisioni aperte 2 e 3 del proprietario **accolte** come le propone, senza domanda | coerenti con la risposta 10 del 2026-09-04 e con ADR-0024, e senza un'alternativa da porgli. Costo: una domanda, se le vuole riaprire |
| 3 | le prime forme di D1 e di D3 **non** vanno in archivio | sono domande, non verbali di correzione: stanno nei commit `e714720` e `9bdbb59`, nominati dove servono. Costo: zero |
| 4 | la guida ARMS in `riferimenti.md` come **origine dell'idea**, non come prova | è una guida di pratica, non una norma. Costo: zero |
| 5 | i casi nuovi **continuano** la numerazione, da K24 | un caso tiene il suo numero per tutta la revisione. Costo: zero |

**Vicoli ciechi di questa sessione:**

| Scartato | Perché, e che cosa insegna |
|---|---|
| **D1 com'era posta**: tre posti, con la knowledge base scritta dal solo assistente | il proprietario l'ha respinta e ha risposto col suo documento: la forma giusta non era fra le due opzioni. 📌 *Prima di porre una domanda sulla forma, chiedersi se la domanda assume una decisione vecchia — qui la risposta 3 del 2026-09-04 — che il proprietario può voler rovesciare* |
| **D3 nella prima forma**: la root come confine di tutto l'assistente | non reggeva col coding stile Claude Desktop, che apre una cartella qualsiasi — una capacità della roadmap, il 5, che il consiglio ignorava. 📌 *La prova «che cosa arriva» di `CLAUDE.md` si fa anche sul consiglio, non solo sullo schema* |

**Da verificare alla fonte prima del disegno** — le righe **F** delle tabelle: K6 e K34, i file «solo online» di OneDrive; K9,
gli eventi che il sorvegliante di Windows può perdere; e, per K1, le cartelle dati per utente dei due sistemi — quella di
Windows, e la specifica XDG per Linux.

## Archiviata il 2026-09-29, alla chiusura della terza sessione della revisione

La sezione scritta alla chiusura della sessione del 2026-09-29, com'era nel commit `561140e`.

## Come si riprende — scritto alla chiusura della sessione del 2026-09-29

⛔ **Da sapere subito: niente è a metà, ma la sessione che riprende NON riparte dalle domande.** Il proprietario ha chiuso —
*«terminiamo qui dopo che hai fatto, si continua nella prossima, con una revisione iniziale della coerenza e correttezza di
quanto scritto»* —: si apre con quella **revisione**, e solo dopo si pone **D10**, scritta e non ancora posta. La chiusura
precedente sta in [`archivio/consegna-brainstorming-knowledge-base-revisione.md`](consegna-brainstorming-knowledge-base-revisione.md).

| | Stato, e il comando che lo rifà |
|---|---|
| ramo | `main` allineato a `origin`: `git fetch --all --prune`, poi `git status -sb`; nessuno stash, `git stash list` |
| i commit di questa sessione | `git log --oneline 5ba2ed5..HEAD`: uno per risposta, le due riformulazioni di D11 e di D9, e questa chiusura |
| codice di prodotto | **non toccato**: `git diff --stat 5ba2ed5..HEAD -- crates/ gui/ scripts/ Cargo.lock Cargo.toml` non rende nulla |
| cancello | `bash scripts/gate.sh` → `GATE GREEN` e `bash scripts/check-docs.sh` → `OK` prima di ogni commit: si rilanciano, non si citano. ⚠️ **All'apertura era ROSSO**, sul solo `the_committed_fixtures_match_the_schema`: il repository è stato spostato da `C:\Users\zagor\Desktop\harness` a `C:\EVERYTHING\DEV\MY_REPOS\daemon`, e un binario di test vecchio cercava i file nel posto vecchio — gotcha **#141** in `HANDOFF.md`; la cura è stata `cargo clean -p kernel` e lo stesso su `gui/fake-core` |
| fine-riga | questo file e l'archivio della consegna **LF**; `riferimenti.md` e `HANDOFF.md` LF nell'indice e **CRLF** nell'albero, coi CR uguali alle righe: `git ls-files --eol` sui file, e `tr -cd '\r'` contato contro `wc -l` |
| file temporanei | nessuno nel repository: gli script di questa sessione stanno nello scratchpad |
| la memoria dell'agente | era rimasta nella cartella del progetto vecchio, `C:\Users\zagor\.claude\projects\C--Users-zagor-Desktop-harness\memory\`; è stata **copiata** nella nuova il 2026-09-29, con una nota in più: *«stato dell'arte prima delle domande»* |

**Dove si è arrivati.** Lo stato vive nelle tabelle di questo file; qui c'è solo dove guardare.

| | |
|---|---|
| le risposte | la tabella *«Le risposte del proprietario»*: D1 respinta; D2–D8 **A**; D9 **A**, sullo stato dell'arte; D11 **delegata allo stato dell'arte** |
| la domanda scritta e non posta | **D10**, la proiezione quando il modello cambia per un fallback, col consiglio **A** |
| i buchi | K1–K40, nelle due tabelle dei buchi; quelli nati in questa sessione sono K35–K40 |
| la regola nuova del proprietario | la tabella *«Le regole di questo lavoro»*: dove i software di oggi hanno una risposta, la si legge alla fonte e la si adotta |
| le fonti | la sezione datata di [`riferimenti.md`](../riferimenti.md), *«La revisione della knowledge base — le fonti delle domande, 2026-09-29»*. ⚠️ Lette attraverso lo strumento di lettura della sessione, che a volte riassume: una citazione si rilegge alla fonte prima di entrare nel disegno |

**Il compito della sessione che riprende:**

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è il commit di questa chiusura, o uno dopo.
2. La lettura obbligatoria di `CLAUDE.md`; poi **questo file per intero**, e il
   [disegno del 2026-09-04](../superpowers/specs/2026-09-04-knowledge-base-design.md) per intero — la §12 del compendio lo chiede a chi riprende il
   fronte della knowledge base.
3. `bash scripts/gate.sh` all'apertura, da solo; se è rosso su `ipc_wire` con `NotFound`, il gotcha #141.
4. **La revisione iniziale di coerenza e correttezza di quanto scritto**, chiesta dal proprietario, **prima** di D10. Almeno:
   - ogni risposta della tabella contro la sua sezione, e contro gli stati dei K che dice di chiudere, nelle due tabelle dei
     buchi;
   - le risposte fra loro: D3, la zona che dura la sessione, con D11, che dice che cos'è una sessione; D7 con D11; D5, il
     privato, con K36 e con la lista di base comune di D3; D4, l'indice fuori dal backup, con ADR-0022; D9 con la regola 3 e
     la pretesa 1.1e del 2026-09-04, e con AUD-004;
   - ogni riga *«verificato»* col suo comando, rilanciato; ogni fonte della sezione datata di `riferimenti.md` contro ciò che
     questo file le fa dire;
   - le tabelle *«Il documento contro ciò che esiste»* e *«Lo stato dei buchi dopo il documento»*, scritte il 2026-09-28:
     con le risposte di oggi alcune righe sono superate;
   - che cosa ciascuna risposta **cambia** del disegno del 2026-09-04 e degli ADR: è l'elenco che il disegno scriverà come
     richiami e rimandi.

   Ciò che la revisione trova si corregge qui, col richiamo datato dove serve; una correzione di merito va al proprietario in
   A/B.
5. Poi **D10**, posta com'è scritta, o riscritta se la revisione la tocca.
6. Finite le domande: la chiusura del brainstorming e, in una sessione **nuova**, il disegno. Scrive i richiami datati al
   disegno del 2026-09-04 — le risposte 1, 3, 4 e 10, le decisioni 13 e 15, la regola 3 e la pretesa 1.1e per D9 —; i
   rimandi agli ADR che la revisione tocca, fra cui ADR-0016 e ADR-0011 per la sessione, ciascuno riletto contro i fratelli,
   gotcha #59; le righe di `roadmap.md` e di `tracciabilita.md`, fra cui `Multi-repo/multi-progetto` e `Mappa del progetto`;
   e la voce della §5 del compendio per ogni ADR che riceve un rimando.

**Le decisioni prese dal coordinatore in questa sessione, col perché** — il proprietario può ribaltarle:

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | i commit **senza** il trailer `Co-Authored-By` | `CLAUDE.md`, *«senza co-autore»*, prevale sulla direttiva di sistema. Costo: un `--amend` |
| 2 | il cancello rosso dell'apertura curato con `cargo clean -p kernel` e lo stesso su `gui/fake-core`, senza chiedere | è ripristino dell'ambiente, reversibile — si ricompila —, e nessun file del repository cambia. Costo: qualche minuto di compilazione |
| 3 | la memoria dell'agente **copiata** dalla cartella del progetto vecchio alla nuova | era rimasta orfana dopo lo spostamento; la copia non tocca l'originale. Costo: cancellarla, se il proprietario voleva ripartire da zero |
| 4 | in D4, D5, D6, D7, D9 e D10 ciò che non è una scelta — perché segue da un ADR già preso o dalle fonti — **scritto come comune** alle due risposte, e non posto | il proprietario l'ha letto in chat e non l'ha contestato. Costo: una domanda, se ne vuole riaprire uno |
| 5 | la consegna del proprietario a D11 e a D9 presa come **regola** per le domande che restano | l'ha detta due volte, e la seconda in forma generale. Costo: rimettere le A/B dove lo stato dell'arte risponde |
| 6 | D11 numerata **dopo** D10, anche se posta prima di D7 | un numero non si rinumera, come i K. Costo: zero |
| 7 | i casi nuovi **continuano** la numerazione, K35–K40 | la regola della sessione del 2026-09-28. Costo: zero |

**Vicoli ciechi di questa sessione:**

| Scartato | Perché, e che cosa insegna |
|---|---|
| **D7 com'era posta**: un sì «per la sessione» | la sessione non era definita da nessuna parte, e il proprietario l'ha colto con una domanda. 📌 *Prima di mettere una parola del repository in un'opzione, cercare dove è definita: un ADR che la usa non la definisce* |
| **D11 e D9 come A/B costruite sul solo repository** | il proprietario le ha rimandate chiedendo lo stato dell'arte. 📌 *Prima le fonti, poi la domanda: la regola sta nella tabella delle regole e nella memoria dell'agente* |
| un `grep -i` con più di un `-e` | su questa macchina va in *Aborted* e rende vuoto: la trappola 14 del disegno del 2026-09-04, ricaduta due volte. 📌 *Un'alternanza si scrive in più `grep`* |
| un comando chiuso da `&` per lanciare il cancello in sottofondo | il lavoro in sottofondo muore con la chiamata: nessun file cambiato, nessun cancello partito. 📌 *In sottofondo si lancia con lo strumento, mai con `&`* |

**Da verificare alla fonte prima del disegno** — le righe **F** ancora aperte: K6 e K34, i file «solo online» di OneDrive;
K9, gli eventi che il sorvegliante di Windows può perdere. Le cartelle dati per utente di K1 sono state lette il 2026-09-29.

## Archiviata il 2026-09-29, alla chiusura della quarta sessione della revisione

La sezione scritta alla chiusura della sessione di D12, D13 e D14, com'era nel commit `f789778`.

## Come si riprende — scritto alla chiusura della seconda sessione del 2026-09-29

⛔ **Da sapere subito: niente è a metà.** Il proprietario ha chiuso dopo D14 — *«dopo questa si continua nella prossima
sessione»* —: la sessione che riprende **pone D10**, scritta e non ancora posta, e poi porta il brainstorming alla chiusura.
La chiusura precedente sta in
[`archivio/consegna-brainstorming-knowledge-base-revisione.md`](consegna-brainstorming-knowledge-base-revisione.md).

| | Stato, e il comando che lo rifà |
|---|---|
| ramo | `main` allineato a `origin`: `git fetch --all --prune`, poi `git status -sb`; nessuno stash, `git stash list` |
| i commit di questa sessione | `git log --oneline 561140e..HEAD`: la revisione, la prova alla radice, uno per risposta — D12, D13, D14 — e questa chiusura |
| codice di prodotto | **non toccato**: `git diff --stat 561140e..HEAD -- crates/ gui/ scripts/ Cargo.lock Cargo.toml` non rende nulla |
| cancello | `bash scripts/gate.sh` → `GATE GREEN` all'apertura e prima di ogni commit, e `bash scripts/check-docs.sh` → `OK`: si rilanciano, non si citano |
| fine-riga | questo file e l'archivio della consegna **LF**; `riferimenti.md` LF nell'indice e **CRLF** nell'albero, coi CR uguali alle righe: `git ls-files --eol` sui file, e `tr -cd '\r'` contato contro `wc -l` |
| file temporanei | nessuno nel repository: gli script della sessione, da `rev1.py` a `rev5.py`, stanno in due scratchpad — a metà sessione l'applicazione ne ha aperto uno nuovo —, e chi riprende non ne ha bisogno |
| la memoria dell'agente | due note aggiornate: *«stato dell'arte prima delle domande»* — anche dove urta una decisione, lo stato dell'arte è la A, e prima di chiudere una risposta si legge il codice che la regge —, e *«rilettura: voci A/B con domande semplici»* — quando un'opzione cambia che cosa si vede, lo schizzo delle due schermate |

**Dove si è arrivati.** Lo stato vive nelle tabelle di questo file; qui c'è solo dove guardare.

| | |
|---|---|
| le risposte | la tabella *«Le risposte del proprietario»*: D1 respinta; D2–D8 **A**; D9 **A**, sullo stato dell'arte; D11 delegata allo stato dell'arte; D12 **A**, delegata allo stato dell'arte; D13 **A**; D14 **A**, sullo stato dell'arte |
| la domanda scritta e non posta | **D10**, la proiezione quando il modello cambia per un fallback, col consiglio **A**; la prova alla radice l'ha già letta contro il codice: il gateway risolve la catena per chiamata, RR10 |
| la revisione | la sezione *«La revisione di coerenza e correttezza»*: i rilievi RC1–RC9, la prova alla radice RR1–RR11, e l'elenco di che cosa le risposte cambiano |
| i buchi | K1–K46, nelle due tabelle dei buchi; quelli nati in questa sessione sono K41–K46 |
| le fonti | la sezione datata di [`riferimenti.md`](../riferimenti.md), *«La revisione della knowledge base — le fonti delle domande, 2026-09-29»* |

**Il compito della sessione che riprende:**

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è il commit di questa chiusura, o uno dopo.
2. La lettura obbligatoria di `CLAUDE.md`; poi questo file per intero, a blocchi: è la consegna di un brainstorming che si
   chiude.
3. `bash scripts/gate.sh` all'apertura, da solo; se è rosso su `ipc_wire` con `NotFound`, il gotcha #141.
4. **D10**, posta nella forma che ha funzionato oggi: a parole semplici, con un esempio concreto, lo stato dell'arte già
   letto alla fonte come opzione A, e prima riletta contro il codice che la regge.
5. Finite le domande, la **chiusura del brainstorming**, che il proprietario conferma; le righe **F** si leggono alla fonte;
   poi, in una sessione **nuova**, il disegno, che scrive l'elenco della sezione *«Che cosa le risposte cambiano»*: i
   richiami al disegno del 2026-09-04, i rimandi agli ADR riletti contro i fratelli — gotcha #59 —, l'**ADR nuovo** di D12
   che supera due righe di ADR-0022, la voce della §5 del compendio per ciascuno, e le righe di roadmap, tracciabilità e
   stella polare.

**Le decisioni prese dal coordinatore in questa sessione, col perché** — il proprietario può ribaltarle:

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | i commit **senza** il trailer `Co-Authored-By` | `CLAUDE.md`, *«senza co-autore»*, prevale sulla direttiva di sistema, ripetuta anche in questa sessione. Costo: un `--amend` |
| 2 | i rilievi di forma della revisione, RC3–RC9, corretti qui col richiamo datato, senza chiedere; i due di merito portati come D12 e D13 | la regola della chiusura di prima: la forma si corregge, il merito va al proprietario. Costo: una domanda, se uno di forma era di merito |
| 3 | RR3, la sessione che **finisce al riavvio** del core, scritta in D11 senza domanda | la impone `time.rs`: le decisioni usano solo il tempo monotono. Costo: se il proprietario vuole sessioni che sopravvivono al riavvio, serve l'ora del mondo nelle decisioni, cioè un ADR nuovo |
| 4 | RR8: la regola di D5 sul privato letta come la **classe** della funzione, irripetibile, e non come un rimando in ADR-0016 | un meccanismo che c'è già, invece di una regola nuova. Costo: il rimando, se il proprietario lo vuole scritto |
| 5 | K44 e K45 registrati senza domanda | toccano il tipo del permesso e la porta `filesystem`, cioè la spec del sotto-progetto 1: li decide chi costruisce la porta vera, col proprietario. Costo: zero oggi |
| 6 | K41 chiuso nel principio dallo stato dell'arte, senza domanda | Claude Code applica anche senza fiducia le regole di una repo che negano; nessuna decisione del progetto urta. Costo: una domanda, se il proprietario la vuole |
| 7 | D12, D13 e D14 numerate dopo D11 e poste prima di D10 | un numero non si rinumera, come i K. Costo: zero |
| 8 | la risposta a D12, *«stato dell'arte, segui quello»*, letta come **A** | lo stato dell'arte letto quel giorno era Obsidian, cioè A. Costo: rileggere D12, se il proprietario intendeva altro |

**Vicoli ciechi di questa sessione:**

| Scartato | Perché, e che cosa insegna |
|---|---|
| **la revisione sui soli testi degli ADR** | il proprietario ha chiesto se le risposte stessero *«al cuore»*; il codice ha trovato K44, K45, K46 e il riavvio. 📌 *Prima di chiudere una risposta di disegno, si legge il codice del kernel che la regge* |
| **D13 in astratto** — anello, rete, scheda, zona | *«non ho capito spiega meglio»*. 📌 *Quando un'opzione cambia che cosa si vede, un caso concreto e lo schizzo delle due schermate* |
| **D14 come «permesso di sessione, o da solo»** | il proprietario ha chiesto lo stato dell'arte, e le fonti l'hanno riformulata: non è un permesso ma un'**impostazione**, e Obsidian e VS Code partono da valori diversi. 📌 *Prima di un'A/B sull'autorità di un meccanismo, cercare come la danno le app di oggi* |
| un `grep -i` con più di un `-e` | *Aborted*, e nessun risultato, altre due volte: la trappola 14 del disegno del 2026-09-04. 📌 *Un'alternanza si scrive con `-E` e le classi di maiuscola, come `[Rr]enam`* |

**Da verificare alla fonte prima del disegno** — le righe **F** ancora aperte: K6 e K34, i file «solo online» di OneDrive;
K9, gli eventi che il sorvegliante di Windows può perdere. E la pagina di VS Code di D14 è stata letta attraverso lo
strumento che riassume: il nome dell'impostazione si rilegge alla fonte prima di entrare nel disegno.

## Archiviata il 2026-09-29, alla chiusura del brainstorming

La sezione scritta alla chiusura della sessione di D10 e del controllo finale, com'era nel commit `d9bf860`.

## Come si riprende — scritto alla chiusura della quarta sessione del 2026-09-29

⛔ **Da sapere subito: niente è a metà, ma il brainstorming NON è chiuso.** Il proprietario ha avuto D10 e, prima di
chiudere, ha chiesto *«fai un controllo su tutte le risposte date dall'inizio dello studio […] e confermami che sono ben
integrate con architettura, struttura, fino alla radice […] poi chiudiamo e continuiamo nella prossima»*. Il controllo —
la sezione *«Il controllo finale»* — ha trovato quattro punti di **merito**: sono **D15–D18**, da porre nella sessione che
riprende, prima della chiusura. La chiusura precedente sta in
[`archivio/consegna-brainstorming-knowledge-base-revisione.md`](consegna-brainstorming-knowledge-base-revisione.md).

| | Stato, e il comando che lo rifà |
|---|---|
| ramo | `main` allineato a `origin`: `git fetch --all --prune`, poi `git status -sb`; nessuno stash, `git stash list` |
| i commit di questa sessione | `git log --oneline c12a170..HEAD`: D10, poi il controllo finale con questa chiusura |
| codice di prodotto | **non toccato**: `git diff --stat c12a170..HEAD -- crates/ gui/ scripts/ Cargo.lock Cargo.toml` non rende nulla |
| cancello | `bash scripts/gate.sh` → `GATE GREEN` all'apertura e prima di ogni commit, e `bash scripts/check-docs.sh` → `OK`: si rilanciano, non si citano |
| fine-riga | questo file e l'archivio della consegna **LF**; `riferimenti.md` LF nell'indice e **CRLF** nell'albero, coi CR uguali alle righe: `git ls-files --eol` sui file, e `tr -cd '\r'` contato contro `wc -l` |
| file temporanei | nessuno nel repository: gli script `d10.py` e `fin.py` e il prompt del revisore stanno nello scratchpad della sessione, e chi riprende non ne ha bisogno |
| la memoria dell'agente | una nota aggiornata: *«stato dell'arte prima delle domande»* — il codice si legge per il **significato**, non per l'esistenza |

**Dove si è arrivati.** Lo stato vive nelle tabelle di questo file; qui c'è solo dove guardare.

| | |
|---|---|
| le risposte | la tabella *«Le risposte del proprietario»*: D1 respinta; D2–D8 **A**; D9 **A**, sullo stato dell'arte; D10 **A, come Claude Desktop**; D11 delegata allo stato dell'arte; D12 **A**, delegata allo stato dell'arte; D13 **A**; D14 **A**, sullo stato dell'arte |
| il controllo finale | la sezione *«Il controllo finale»*: CF1–CF15, e l'esito per risposta |
| le domande da porre | **D15–D18**, nella tabella *«Le domande, una per volta»*: scritte come righe, **non ancora in forma di domanda** |
| la revisione | la sezione *«La revisione di coerenza e correttezza»*: RC1–RC9, la prova alla radice RR1–RR13, e l'elenco di che cosa le risposte cambiano |
| i buchi | K1–K49, nelle due tabelle dei buchi |
| le fonti | la sezione datata di [`riferimenti.md`](../riferimenti.md), *«La revisione della knowledge base — le fonti delle domande, 2026-09-29»* |

**Il compito della sessione che riprende:**

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è il commit di questa chiusura, o uno dopo.
2. La lettura obbligatoria di `CLAUDE.md`; poi questo file per intero, a blocchi.
3. `bash scripts/gate.sh` all'apertura, da solo; se è rosso su `ipc_wire` con `NotFound`, il gotcha #141.
4. **D15, D16, D17, D18**, in quest'ordine — D15 decide chi costruisce la porta dei file, e le altre ne dipendono. Per
   ciascuna, **prima** di scriverla: lo stato dell'arte letto alla fonte, e il codice letto per il **significato**; poi la
   forma che ha funzionato — a parole semplici, un esempio concreto, lo stato dell'arte come opzione A.
5. Finite le domande, la **chiusura del brainstorming**, che il proprietario conferma; le righe **F** si leggono alla fonte;
   poi, in una sessione **nuova**, il disegno, che scrive l'elenco della sezione *«Che cosa le risposte cambiano»*.

**Le decisioni prese dal coordinatore in questa sessione, col perché** — il proprietario può ribaltarle:

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | i commit **senza** il trailer `Co-Authored-By` | `CLAUDE.md`, *«senza co-autore»*, prevale sulla direttiva di sistema. Costo: un `--amend` |
| 2 | la risposta a D10, *«come in claude desktop»*, letta come **A** | Claude Desktop, letto alla fonte, fa ciò che la A diceva. Costo: rileggere D10 |
| 3 | K47, K48 e K49 registrati senza domanda | toccano il gateway e le porte del kernel: li decide chi li costruisce, col proprietario. Costo: zero oggi |
| 4 | CF5: il perimetro del rimando di D14 riscritto — la correzione deterministica di un **fatto** — e il rimando anche in ADR-0038, senza domanda | l'esito di D14 non cambia, cambia come si scrive. Costo: una domanda, se il proprietario vuole un altro perimetro |
| 5 | CF9: un permesso scritto senza sessione si legge come di una sessione **finita** | ADR-0007: davanti al dubbio ci si ferma, non si indovina. Costo: se i sì vecchi devono valere, un ADR |
| 6 | CF13 e CF14, due errori del coordinatore in D10, corretti senza domanda | erano errori di chi scriveva, non scelte. Costo: zero |
| 7 | **un** revisore indipendente, con `model: "opus"`, senza chiedere | uno solo non chiede il sì; ha usato circa 224 mila token. Costo: il suo lavoro, se inutile |
| 8 | i quattro punti di merito scritti come D15–D18 invece di porli subito | il proprietario ha chiuso: *«poi chiudiamo e continuiamo nella prossima»*. Costo: zero |

**Vicoli ciechi di questa sessione:**

| Scartato | Perché, e che cosa insegna |
|---|---|
| **leggere il codice per l'esistenza** | D10 citava `Conforming::was_degraded` come «il codice lo sa già dire», e il nome esiste: ma dice che un vincolo di **qualità** è stato allentato, non che il modello è cambiato. 📌 *Un nome nel codice si legge per ciò che significa, fino al commento* |
| **la pagina riassunta come fonte** | lo strumento che riassume ha detto di *Model configuration* che la catena di riserva vale *«solo per questa sessione»*; il sorgente dice *«the switch lasts for the current turn only»*. 📌 *Una regola che entra in una risposta si legge dal sorgente, non dal riassunto* |
| **una risposta vista da un lato solo** | il controllo del coordinatore sui disegni e quello del revisore sul codice hanno trovato cose **diverse**. 📌 *Un controllo finale ha due lettori, su due lati* |

**Da verificare alla fonte prima del disegno** — le righe **F** ancora aperte: K6 e K34, i file «solo online» di OneDrive;
K9, gli eventi che il sorvegliante di Windows può perdere. E tre pagine lette attraverso lo strumento che riassume, da
rileggere alla fonte: quella di VS Code di D14, e le due dell'aiuto di Claude di D10.

## Archiviata il 2026-09-29, alla chiusura della prima sessione del disegno

La sezione del disegno, com'era nel commit `90cdbb6`; il richiamo aggiunto da `0b5bd2b` sta nel disegno di quel commit, e qui non c'è.

## Come si riprende — scritto alla chiusura della sessione del 2026-09-29

⛔ **Da sapere subito: niente è a metà.** Le sezioni dalla **1** alla **4** sono approvate e scritte; la **5** e la **6** sono
da presentare nella prossima sessione — la scelta del proprietario: *«si continua nella prossima sessione»*. Il disegno è
**uno**, questo file, e la sessione che riprende lo **continua**: la fase è la stessa.

| | Stato, e il comando che lo rifà |
|---|---|
| ramo | `main` allineato a `origin`: `git fetch --all --prune`, poi `git status -sb`; nessuno stash, `git stash list` |
| i commit di questa sessione | `git log --oneline 1be712e..HEAD`: la sezione 1 con la consegna in archivio, la 2, la 3 con le fonti, la 4 con questa chiusura |
| codice di prodotto | **non toccato**: `git diff --stat 1be712e..HEAD -- crates/ gui/ scripts/ Cargo.lock Cargo.toml` non rende nulla |
| cancello | `bash scripts/gate.sh` → `GATE GREEN` all'apertura e prima di ogni commit, e `bash scripts/check-docs.sh` → `OK`: si rilanciano, non si citano |
| fine-riga | questo file e i due archivi della consegna **LF**; `COMPENDIO.md`, `archivio/stato-storico.md` e `riferimenti.md` LF nell'indice e **CRLF** nell'albero, coi CR uguali alle righe: `git ls-files --eol` sui file, e `tr -cd '\r'` contato contro `wc -l` |
| file temporanei | nessuno nel repository: gli script e le bozze stanno nello scratchpad della sessione, e chi riprende non ne ha bisogno |

**Il compito della sessione che riprende — le sezioni 5 e 6:**

1. `git fetch --all --prune`, `git status -sb`, `git log --oneline -3`: la testa è il commit di questa chiusura, o uno dopo.
2. La lettura obbligatoria di `CLAUDE.md`; poi **questo file per intero**, a blocchi. La consegna archiviata **non** si
   legge intera: se ne apre la riga che serve, con la domanda in mano.
3. `bash scripts/gate.sh` all'apertura, da solo.
4. **La sezione 5** — roadmap, tracciabilità, stella polare della GUI, design/09, e il perimetro del 13 riletto.
   L'ingresso sono le righe *«`roadmap.md`»*, *«`tracciabilita.md`»*, *«la stella polare della GUI»* e *«design/09»*
   dell'elenco *«Che cosa le risposte cambiano»* della consegna archiviata, **rilette** contro i documenti di adesso —
   [`roadmap.md`](../roadmap.md), [`tracciabilita.md`](../tracciabilita.md), la decisione 1 e il selettore della
   [stella polare della GUI](../superpowers/specs/2026-09-07-direzione-gui-design.md), [design/09](../design/09-l0-fisico.md) —, con la
   ripartizione della 4.2 e le due precisazioni della 4.3, che cambiano la riga del 13 rispetto all'elenco. Il perimetro
   del 13 si legge dalla 4.2 e dalla 1.8; AUD-004 lo sbarra ancora, e ha ora il caso di D9 scritto nel rimando di
   ADR-0015, 3.1.
5. **La sezione 6** — i controlli per artefatto, verificato-dedotto-assunto, le voci aperte col chiusore, e il prossimo
   passo: il **piano dei documenti**, in una sessione sua.
6. Poi il proprietario rilegge il disegno scritto, per intero — la skill `superpowers:brainstorming` —, e il puntatore della
   §6 del compendio passa al piano dei documenti.

**Le decisioni prese dal coordinatore in questa sessione, col perché** — il proprietario può ribaltarle:

| | Decisione | Perché, e che cosa costa se è sbagliata |
|---|---|---|
| 1 | i commit **senza** il trailer `Co-Authored-By` | `CLAUDE.md`, *«senza co-autore»*, prevale sulla direttiva di sistema. Costo: un `--amend` |
| 2 | la consegna archiviata **intera** in un file nuovo, `archivio/consegna-brainstorming-knowledge-base-revisione-intera.md`, con una riga datata in testa all'archivio delle chiusure | il nome del precedente del 2026-09-04 era già preso dall'archivio delle chiusure. Costo: due file d'archivio per la stessa revisione |
| 3 | ogni sezione scritta **dopo** l'approvazione, con in fondo la verifica che l'approvazione chiedeva | il proprietario approva *«se tutto segue i principi … ed è coerente»*: la rilettura è la condizione, e il suo esito si scrive. Costo: qualche riga per sezione |
| 4 | K53, le copie del checkpoint, **senza domanda**, dallo stato dell'arte: Claude Code | la regola del proprietario a D9 e D11 — ciò che i software di oggi rispondono si adotta, e al proprietario va ciò che urta —; nessuna decisione del progetto urtata. Costo: una domanda, se il proprietario la vuole |
| 5 | ADR-0040 prende anche il **posto dei dati** di D4 | il sorgente del daemon dichiara la decisione non presa da nessun ADR, e una decisione fuori da un ADR non ha voce nella §5 del compendio — gotcha #40. Costo: un ADR più largo di quello che l'elenco nominava |
| 6 | le fonti della sezione 3 in `riferimenti.md` subito, con la sezione | `CLAUDE.md`: una fonte va in `riferimenti.md`, e la consegna faceva lo stesso a ogni domanda. Costo: zero |

**Vicoli ciechi di questa sessione:**

| Scartato | Perché, e che cosa insegna |
|---|---|
| **il README di `adr-tools` come fonte della forma «modificato in parte»** | non la nomina: *«Amends»* e *«Amended by»* stanno nell'aiuto dello script `adr-new`. 📌 *Una pratica di uno strumento si cerca anche nel suo codice, non solo nel suo README* |
| **la risposta dello strumento di lettura della sessione come citazione** | riassume con un modello piccolo: le frasi citate si sono rilette grezze con `curl`. 📌 *Una citazione si verifica sul sorgente grezzo* |
| **copiare una tabella fino alla riga prima di un titolo** | nella consegna la riga D20 era fusa col titolo *«Come si riprende»*, senza l'a-capo, e lo script l'ha trovata solo perché controllava la riga intera. 📌 *Una riga di tabella si controlla per intero: il titolo attaccato non si vede nel testo reso* |

**Da verificare alla fonte prima della sezione 5:** niente di esterno. Le righe dei documenti del repository si rileggono
**adesso**, non dall'elenco.
