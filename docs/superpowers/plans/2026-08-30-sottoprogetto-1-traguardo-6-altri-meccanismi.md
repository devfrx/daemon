# Traguardo 6 — gli altri meccanismi: il piano

> **Per chi esegue:** SOTTO-SKILL OBBLIGATORIA — `superpowers:subagent-driven-development`,
> un subagente fresco per compito con revisione fra uno e l'altro. È la modalità scelta dal
> proprietario e ha portato tutti i compiti eseguiti finora. I passi usano le caselle
> (`- [ ]`) per il tracciamento.

**Obiettivo.** Costruire gli ultimi meccanismi del kernel — la concessione che torna,
lo schema del canale worker, lo schema `ipc`, il contratto del sensore, il decisore del
gateway col suo gettone, il permesso, lo stato di degrado — e chiudere con essi le righe di
catalogo che li aspettano.

**Architettura.** Nessun meccanismo nuovo di trasporto: il traguardo costruisce **forme e
logica dentro `kernel`**, più una finta di `ipc` in `simulator`. `platform` non guadagna
niente — il trasporto vero di `ipc` e `process` è scaglionato da §0.2 e §0.4 riga §1. Le
decisioni di forma sono già prese dal
[disegno](../specs/2026-08-28-sottoprogetto-1-traguardo-6-altri-meccanismi-design.md); questo
piano le traduce in passi.

**Stack.** Rust edition 2024, toolchain appuntata `1.95.0`, `kernel` e `simulator` in
`no_std` + `alloc` + `forbid(unsafe_code)`. `minicbor` 2.3.0 per il giornale e per il canale
worker; il formato dello schema `ipc` lo decide il **compito 3bis**. La porta di qualità è
`bash scripts/gate.sh`, e deve stampare `GATE GREEN` a **ogni commit**.

---

## ▶️ A che punto è QUESTO PIANO — casa unica, e si aggiorna scrivendo

✅ **IL PIANO È FINITO IL 2026-08-30 — dieci compiti in cinque parti.**
⚠️ **RICHIAMO DEL 2026-08-30:** qui stava *«IL PIANO NON È FINITO. Non si esegue finché non lo
è»*, e la ragione che portava resta vera e vale la pena tenerla scritta: al Traguardo 5 il
disegno aveva **dimenticato una condizione di chiusura** e a rimediare fu chi scriveva il piano;
un piano eseguito a metà scrittura non ha nessuno che faccia quel controllo. ✅ **Quel controllo
è stato fatto:** la §7.2 del disegno nasce completa, e il pre-controllo di ogni parte ha trovato
qualcosa — **sedici** voci, e le ultime cinque hanno cambiato il prezzo dei compiti che stavano
per essere scritti.

⛔ **UN COSTO DICHIARATO, e chi esegue lo deve sapere prima:** i compiti **8** e **9** lasciano
**vuoti dei corpi di sonda**, con scritto **che cosa leggere** per riempirli invece del codice.
Non è un segnaposto: la firma di `Arbiter::new` e la risposta di `Arbiter::release` **cambiano
dentro questo stesso piano**, col compito **1**, e dettarle qui sarebbe dettare uno stato che il
traguardo sta cambiando — il gotcha **#57**, *una previsione citata come una misura*. È il
precedente del Task 8 del Traguardo 3, dove il piano **rifiutò di dettare l'API di `redb`**
perché dettarla a memoria produce *«codice plausibile e falso»*.

| Parte | Compiti | Stato |
|---|---|---|
| **A** — la concessione che torna | 1 | ✅ **ESEGUITA** — `GATE GREEN` a ogni commit del compito |
| **B** — il filo | 3, 3bis | ✅ **ESEGUITA** il 2026-08-31 — entrambi i compiti, `GATE GREEN` a ogni commit. Il **3bis** era una misura, e il proprietario ha deciso **C-1** lo stesso giorno: `bincode` 2.0.1 resta |
| **C** — lo schema `ipc` | 4 | ✅ **ESEGUITA** il 2026-08-31 — `GATE GREEN` a ogni commit. Il pre-controllo aveva reso **cinque** voci, `E32`–`E36`, **due bloccanti**; la **terza revisione** è tornata pulita |
| **D** — i meccanismi | 5, 6, 7, 8 | ▶️ **IN CORSO** — il **compito 5 è ESEGUITO**, `GATE GREEN` a ciascuno dei tre commit. ⚠️ **RICHIAMO DEL 2026-08-31: qui stava una DATA, ed era il 2026-09-01 mentre `git log` data tutti e tre i commit al 2026-08-31.** **Tolta e non ricorretta**, che è la riga di [`CLAUDE.md`](../../../CLAUDE.md) sulle cifre in più case applicata a una data: *«ogni riga porta il proprio commit, e il commit È la sua data»*, `git show -s --format=%ad <hash>` — e un comando non marcisce. ⛔ **I VERBALI NON SONO TOCCATI:** `E46` ed `E53` portano quella data dentro il racconto di ciò che quella sessione credeva, ed è onesto che ce la portino. Il pre-controllo aveva reso **cinque** voci, `E44`–`E48`, **due bloccanti**; eseguendo ne sono uscite altre **tre**, `E49`–`E51`. ⚠️ **E il compito 5 porta un RESIDUO DI METODO, `E53`: è stato eseguito in sessione e NON ha avuto il ciclo di revisione indipendente** che questo piano prescrive in testa. Chi riprende lo eredita, e non si chiude eseguendo il compito 6. Scritta il 2026-08-30 — pre-controllo P-12…P-15, voce chiusa dalla **D22**, e i quattro compiti. ✅ **E IL 2026-08-31 IL COMPITO 5 HA AVUTO LA SUA REVISIONE**, da una sessione fresca e non da un subagente: **tre rilievi**, `E54`, `E55` ed `E56`, di cui **due mutanti vivi** sull'intero workspace. Il residuo `E53` è **ridotto e non chiuso**, e dice che cosa manca. ⛔ **E IL PRE-CONTROLLO DEL COMPITO 6 È FATTO lo stesso giorno, PRIMA di dispacciarlo: `E57`–`E62`, DUE bloccanti** — una sonda che non può passare perché scrive una nota su un passo che nessuno ha aperto (`E45` alla seconda occorrenza) e due che non compilano. ⚠️ **Il compito NON è stato dispacciato:** il piano prescrive subagent-driven e i sotto-agenti restano non autorizzati. ⛔ **E IL 2026-08-31 IL COMPITO 5 HA AVUTO LA SUA SECONDA PASSATA DI REVISIONE, che ha trovato ALTRE DUE COSE: `E65` — QUATTRO mutanti vivi sull'intero workspace, di cui uno fa attraversare a contenuto non fidato il confine di I6 dentro il formato durevole — ed `E66`, tre affermazioni di data della specie che la passata precedente aveva deciso di togliere, sopravvissute perché quel censimento non arrivò mai al sorgente.** ⚠️ **QUINDI `E53` È RIDOTTA UNA SECONDA VOLTA E ANCORA NON CHIUSA, e la ragione è la regola stessa:** *«si rivede finché una passata non torna pulita»*, e questa non è tornata pulita. Il compito 5 ha ora **due** giri dove gli altri ne hanno avuti da tre a cinque, ed **entrambi** hanno trovato difetti veri — il secondo su un perimetro più largo, perché il primo aveva scritto codice. ✅ **E il prodotto del primo giro REGGE, misurato invece che assunto:** la guardia di crescita su `Detail` dà `` error[E0004] `` aggiungendo una variante, e le due sonde nuove di `reconciliation.rs` uccidono entrambe le mutazioni dell'arm `Verdict`. ⛔ **E IL TERZO GIRO È STATO FATTO LO STESSO GIORNO — il primo da un SOTTO-AGENTE, perché il proprietario li ha autorizzati, che è la condizione che `E53` registrava come mancante.** Ha reso **sei** rilievi, `E67`–`E72`, tutti **riverificati** prima di agire: due mutanti vivi (`spent_millis`, e l'annotazione `#[cbor(default)]` dichiarata portante e inerte), la testa di `frozen_bytes.rs` e la mappa dei byte con i conteggi del compito 5 mai aggiornati, una sonda col nome del proprio fratello, un ciclo che cammina tre varianti su quattro sotto la parola *«every»*, e un'asserzione dominata con la giustificazione misurata al contrario. ⚠️ **E UNO DEI SEI ERA PREZZATO PIÙ GRANDE DEL DIFETTO** — `E69`, dove il rapporto dava **due** nomi scambiati e ne era sbagliato **uno**: gotcha **#65** applicato al rapporto di un sotto-agente, colto leggendo la coppia gemella invece del rapporto. ⛔ **QUINDI `E53` È RIDOTTA UNA TERZA VOLTA E ANCORA NON CHIUSA:** tre giri, **tutti e tre** con difetti veri, e la regola resta *«finché una passata non torna pulita»* |
| **E** — la prova e la chiusura | 9, 10 | ✅ **scritta** il 2026-08-30 — e il pre-controllo ha trovato **P-16**, un meccanismo che la mappa dei file non ospitava |

📌 **Gli hash non stanno in questa tabella:** una lista di commit si allunga a ogni ondata
di revisione e va riscritta in **ogni** sito che la porta — quella del compito 3 era già
indietro, e viveva in due siti. Al suo posto sta il comando, che rende i commit che **nominano**
un compito: `git log --format='%h %s' | grep -E 'compito 3'`, col numero della colonna «Compiti».

✅ **La D è stata sbarrata e sbloccata lo stesso giorno, e il verbale è P-11.** I compiti **5**,
**6** e **7** devono mettere dati **strutturati e nostri** dentro un record durevole, e
`RecordV1` non aveva una casella per farlo. ⛔ **La risposta non era nessuna delle due che
sembravano esserci**, e a trovarla è stata una **misura**: un campo facoltativo da solo lascia
un lettore vecchio leggere il record **e perderne la sostanza in silenzio**, quindi la specie
nuova sta nel `kind` — che fa **fermare** quel lettore — e il dettaglio in un campo nuovo, fuori
dal `payload`. Sono la **D20** e la **D21**.

⚠️ **Il compito 2 non c'è, e non è un buco:** il timbro di build è **uscito** dal perimetro
alla §3.4 del disegno e diventa una non-costruzione dichiarata. La numerazione della §1.4 è
tenuta com'è invece di essere compattata, perché il disegno vi rimanda per numero.

### ▶️ Come si esegue un compito di questo piano

⛔ **QUALE compito venga dopo NON è scritto qui:** vive nella **§6 del
[`COMPENDIO.md`](../../COMPENDIO.md)**, in un posto solo, come già fanno
[`HANDOFF.md`](../../HANDOFF.md) e [`README.md`](../../README.md). È la regola del 2026-08-18,
che il piano del Traguardo 5 scrive per esteso: *«il puntatore al prossimo passo NON si ricopia
negli altri documenti»*. ⚠️ **Ciò che resta qui è la POSIZIONE del piano** — la tabella delle
parti qui sopra, che si aggiorna eseguendo — e **come** si esegue, che è un'altra cosa dal
**quale**.

⛔ **Il compito 2 NON esiste**, e non è un salto: il timbro di build è uscito dal perimetro
alla §3.4 del disegno.

La modalità resta quella scelta dal proprietario: `superpowers:subagent-driven-development` — **un
subagente fresco per compito**, con revisione fra uno e l'altro.

⛔ **E PRIMA DI DISPACCIARLO, il pre-controllo delle QUATTRO DOMANDE di `CLAUDE.md`**, che ha
trovato almeno un difetto reale in **tutti** i compiti dispacciati finora, senza una sola
eccezione — più le quattro righe che quell'elenco **non** coglie, prima fra tutte: *un compito
scritto prima si legge contro il codice di **ADESSO**, non contro il piano*.

📌 **Che cosa i cicli dei compiti 3 e 3bis hanno insegnato, e serve a chi viene dopo:** il
pre-controllo del compito 3 ha prodotto **cinque** voci prima di dispacciare, e la bloccante non
era nel meccanismo ma nel **dato** — una sonda **corretta** che il proprio ingresso rendeva
**vacua** (gotcha **#92**). Le revisioni che sono seguite hanno trovato una **via d'errore che
nessuna sonda raggiungeva** — mutata, l'intero workspace restava verde **cifra per cifra** — e
poi, ondata dopo ondata, **affermazioni false nella prosa scritta per chiuderne un'altra**: è la
specie che si è ripetuta.
📌 *Il prodotto regge alla prima passata; le frasi no.*

⛔ **RICHIAMO DEL 2026-08-31 — QUESTA SEZIONE PORTAVA IL PUNTATORE, ED È TOLTO E NON
RIALLINEATO.** Si intitolava *«Il prossimo passo, in forma eseguibile»* e apriva con *«ESEGUIRE
IL COMPITO 3BIS»*: falso **il minuto dopo** che il 3bis è girato, e infatti il commit che ha
chiuso quel compito ha aggiornato la §6 e **non** questa riga. ⚠️ **Ed era la settima volta:**
il 2026-08-30 la stessa riga era già passata per *«Scrivere la Parte D»* nudo, *«i compiti sono
FERMI su una voce del proprietario»*, *«scrivere i quattro compiti»*, *«6, 7 e 8»*, *«7 e 8»* e
*«scrivere la Parte E»*. ⛔ **Riallinearla comprerebbe un giro e nient'altro** — è il corollario
del gotcha **#68**: *il rimedio a una seconda casa è toglierla*. La voce **E16** dell'errata la
registrava come nota, e si chiude lì.

📌 **I dieci compiti, e l'ordine è quello della §1.4 del disegno** — ciascuno legge quello che
la sua riga nomina **e nient'altro**:

| # | Compito | Commit | Che cosa porta |
|---|---|---|---|
| **1** | `E30` + `R6` + `E21` | **tre** (D1) | l'identità dell'arbitro, `release` a tre risposte, la concessione che torna dalla porta `process` |
| ~~2~~ | ~~il timbro di build~~ | — | ⛔ **uscito dal perimetro** — §3.4 del disegno, non-costruzione dichiarata |
| **3** | §6.10, la metà che codifica | **due** | l'inquadratura condivisa e lo schema del canale worker — chiude il **vincolo 15** |
| **3bis** | la misura **C-1** | uno | e ⛔ **si ferma prima di decidere** se la misura chiede un cambio di formato (D12) |
| **4** | §6.1, lo schema `ipc` | uno | la busta e i due messaggi, nel formato che il 3bis ha deciso |
| **5** | §6.4, il sensore | **tre** | il contratto, **il campo `detail`** (D24) e l'anello che giornala |
| **6** | §6.2 e §6.3 | **due** | il decisore, il gettone di conformità (`Q13`) e il record di routing risolto |
| **7** | §6.6, il permesso | uno | la tripla, e la proiezione che risponde a una domanda invece di rendere una lista |
| **8** | §6.7, il degrado | uno | derivato e **ricalcolato**; ⛔ non tocca il formato |
| **9** | `E152` | **due** | la riconciliazione alla disconnessione (**P-16**) e le due proprietà di §5.7 che mancano |
| **10** | la chiusura | uno | ⛔ **è un AUDIT e non una scrittura**, e il verbale va in una **§8 del disegno** |

⛔ **Tre cose che governano l'esecuzione, e stanno qui perché nessun compito da solo le vede.**

| | |
|---|---|
| **il formato lo apre il 5, non il 6** | il campo `detail` si aggiunge **una volta sola** e lo paga il compito 5 (**D24**); i compiti 6 e 7 aggiungono la **propria** variante di `RecordKind`, la propria specie di `Detail` e il **proprio** record congelato — e nient'altro del formato |
| **i record congelati diventano tre in più** | uno per variante nuova (**D21**, **D22**), e ⛔ **lo scarto NON diventa rosso da solo**: l'array di `frozen_bytes.rs` è scritto a mano — **P-12** |
| **ogni compito RIMISURA la propria baseline** | **D5**. Quella di partenza sta scritta **una volta sola**, sotto la tabella delle decisioni |

✅ **RICHIAMO DEL 2026-08-30 — LA MISURA CHE QUESTA RIGA ASSEGNAVA AL COMPITO 6 È FATTA, ED È
STATA ANTICIPATA QUI.** Diceva: *«il PRIMO PASSO del compito 6 è una misura, non una scrittura …
la composizione è dedotta dalle due misure separate»*. ⛔ **Anticiparla non è stato uno zelo:** la
Parte D si scrive **sopra** quella deduzione, e la §4.3 del disegno prescrive di misurarla
*«prima di scrivere»* — non prima di eseguire. Il verbale, le cinque domande e i byte stanno in
**P-15**; la composizione **regge**, e la misura ha portato un fatto che la deduzione non aveva.

📌 **Ciò che ciascun compito chiude, secondo il disegno — e si riconta sulla §7.4, non si cita:**
il **5** chiude `V10` (blocco C, livello 1) e porta a ✅ anche `V14` e `Q10`, che oggi lo dicono
senza averlo; il **6** chiude `Q13` (blocco B); il **7** e l'**8** non chiudono righe proprie.
⛔ **E quattro righe NON vanno marcate ✅ a fine traguardo** — `V11`, `V21`, `V27`, `Q18` — che è
la **condizione 12**, l'unica negativa: chiudere troppo è un modo di fallire la Definizione di
«fatto», non di superarla.

⚠️ **Le decisioni già prese che governano la Parte D** sono la **D20** e la **D21** (la specie
nuova di record), la **D17** (i derive dove i tipi vivono) e la **D5** (ogni compito rimisura la
propria baseline). Si leggono nella tabella delle decisioni, non si ridecidono.

**Lo stato del repository alla consegna, misurato il 2026-08-30 e da RIMISURARE, non da citare:**
ramo `spec/sottoprogetto-1-kernel` **allineato a `origin`** (zero avanti, zero dietro), albero
**pulito**, **nessuno stash**, nessun file non tracciato · `bash scripts/gate.sh` → **`GATE
GREEN`** · `cargo test --locked --workspace --no-fail-fast` → **37 bersagli, 267 passate, 0
fallite, 2 ignorate** · `bash scripts/check-docs.sh` → `OK`.
⛔ **RICHIAMO DEL 2026-08-31, voce `E21` — qui stava *«Nessun rosso, né ereditato né
introdotto, e la ragione è che le tre parti scritte finora NON HANNO TOCCATO UNA RIGA DI
PRODOTTO»*, col comando che lo verificava e la clausola *«se nomina qualcosa, la frase è
falsa»*.** Il compito 1 ha toccato il prodotto, e quel comando **nomina i suoi file**: la frase
si è falsificata da sé. **TOLTA e non riallineata** — era una quantificazione universale su una
popolazione che ogni compito muove, cioè la radice **R1**. ⚠️ **E la tabella delle parti, tre
righe più su, era già aggiornata:** una correzione che attraversa una casa e non l'altra, gotcha
**#85**. ⛔ **Ciò che resta è la regola, che non invecchia: ogni compito RIMISURA la propria
baseline** (**D5**), e quella di partenza è la riga datata qui sopra.

⚠️ **Una cifra dei file toccati NON sta qui, ed è il rimedio e non una svista:** la prima stesura
di questa riga diceva *«nomina un solo file, questo»*, ed era vera finché non è stata scritta —
la chiusura della sessione tocca anche [`HANDOFF.md`](../../HANDOFF.md) e
[`COMPENDIO.md`](../../COMPENDIO.md). Corretta **prima** del commit: è la radice **R1**
dell'audit, colta dentro la riga che la produce.

⛔ **Ciò che chi riprende deve sapere sul compito 3bis:** è la **misura C-1**, e pretende una
ricerca **odierna** — *`bincode` è ancora dichiarato non mantenuto? esiste un'alternativa
mantenuta il cui pari TypeScript abbia un lettore?* La §3.5 del disegno dice alla lettera che
deciderlo a memoria sarebbe il gotcha **#48**. La fonte va tracciata in
[`riferimenti.md`](../../riferimenti.md) con la data.
⚠️ **RICHIAMO DEL 2026-08-30:** questa riga diceva *«prima di scriverlo»*, ed è diventata falsa
lo stesso giorno — il compito **è scritto**. Resta perché ciò che dice non è *quando* si scrive
ma *che cosa pretende*, e chi lo **esegue** ne ha lo stesso bisogno di chi lo scriveva. La
disciplina per esteso sta nel compito, non qui.

---

## ⚠️ L'errata di questo piano — si legge PRIMA di ogni compito, non una volta sola

⛔ **Nasce vuota, e non resterà vuota.** Il pre-controllo ha trovato un difetto reale in
**tutti** i compiti dispacciati finora, senza una sola eccezione: quando ne trovi uno, si
scrive **qui**, con il proprio numero, prima di eseguirlo. Un piano è un'ipotesi.

| # | Voce |
|---|---|
| **E1** | ⛔ **La sonda del Passo 1 del commit 1a NON COMPILA, e il rosso che ottiene non e' quello che il Passo 2 attende.** Scrive `Parameters::new(64, Mib(8192), ArbiterId::new(7))`, ma `Mib` e' `pub struct Mib(u64)` col **campo privato** e un banco d'integrazione sta **fuori dalla crate**. ✅ **Misurato con una sonda usa-e-getta scritta da fuori, compilata e cancellata nella stessa corsa:** `` error[E0423]: cannot initialize a tuple struct which contains private fields ``, col suggerimento del compilatore *«you might have meant to use the `new` associated function»*. ⛔ **Il costo non e' tipografico:** il Passo 2 attende `E0433`/`E0412` su `ArbiterId` e ne otterrebbe **due**, uno dei quali leggibile come «il rosso atteso»; e il **Passo 7**, che pretende la sonda verde dopo aver costruito `ArbiterId`, resterebbe **rosso** — il compito non tocca mai `Mib`. E' il precedente di `E144` al Task 12 del Traguardo 5: *il compito cosi' com'e' scritto non puo' passare*. 📌 **Rimedio: `Mib::new(8_192)`**, che e' la forma che il commit 1b usa gia' (`Mib::new(4_096)`) e l'unica che tutto il banco usa — un'incoerenza interna al compito, non una decisione. |
| **E2** | ⛔ **Il censimento delle finte del Passo 4 del commit 1c e' sbagliato in ENTRAMBI i termini, e il comando che detta e' CIECO a tre file su cinque.** Il passo dice *«sono **sei**, in tre file»* e detta `grep -rn "impl Worker for\|impl Process for" crates/ --include=*.rs`. ✅ **Rilanciato:** quel pattern rende **cinque** `impl` veri piu' **tre righe di commento**, e **non trova nessuna** delle finte dei casi `compile_fail` — che il passo pero' enumera. La ragione e' scritta **dentro quei file**, alla riga 5: usano il percorso **pienamente qualificato**, `impl kernel::ports::process::Worker for FakeWorker`. ✅ **Il censimento vero, col comando che lo rifa':** `grep -rnE "^impl (kernel::ports::process::)?(Worker\|Process) for" crates/ --include=*.rs` rende **undici implementazioni in cinque file** — `ports_are_implementable.rs` (2), `worker_tokens.rs` (3), e **due ciascuno** in `instructing_after_the_kill.rs`, `reading_twice_from_one_receipt.rs` e `reading_without_a_receipt.rs`. ⛔ **E la sua stessa enumerazione non torna con la propria cifra:** cinque nominate piu' *«le tre finte dentro i casi `compile_fail`»* fa **otto**, non sei. 📌 E' il gotcha **#70** in entrambe le forme — *il `grep` restituisce candidate, non case* — commesso nel passo che detta il `grep`, e la voce **P-5** di questo stesso pre-controllo lo aveva gia' colto su un'altra riga. |
| **E3** | ⛔ **Il commit 1c e' PIU' GRANDE di come il Passo 4 lo prezza, e la differenza e' strutturale: ogni `Worker` deve ora PORTARE la concessione da `start` a `kill`.** `Killed` porta `grant: Grant`, quindi `kill(self) -> Killed` deve produrne uno; ma `Grant` **non e' falsificabile** — non ha costruttore pubblico, `tests/compile_fail/grant_has_no_constructor.rs` lo pinza, e non deriva ne' `Copy` ne' `Clone`. ✅ **Misurato leggendo le cinque implementazioni:** **ogni** `Process::start` oggi **scarta** la concessione (`_grant: Grant`) e **nessuna** struct di worker ha un campo per essa — `FakeWorker { next }`, `ScriptedWorker { next_id, streams, dead }`. ⛔ **Quindi *«aggiorna le finte»* non e' un aggiornamento:** ciascuna delle **undici** implementazioni di `E2` cambia forma — il worker guadagna un campo `Grant`, e lo `start` smette di ignorare il proprio argomento. ⚖️ **Il disegno REGGE ed e' la parte da non fraintendere:** e' proprio questo che rende necessario `Started::Rejected`, perche' un avvio fallito non ha un worker a cui affidare la concessione. A essere sotto-prezzato e' il **lavoro**, non la forma — gotcha **#65** nella direzione che costa di piu', la stessa che **P-4** aveva dichiarato per `E21`. |
| **E4** | ⛔ **Il commit 1b rende ROSSI DUE ORACOLI della campagna DST, e nessuno dei suoi otto passi nomina la campagna.** `crates/simulator/tests/arbiter_campaign.rs:400` distingue *«restituita»* da *«gia' riscossa»* sul **ramo `Ok` contro il ramo `Err`** — ed e' esattamente la distinzione che il commit 1b **sposta dentro `Ok`**. Dopo il cambiamento `Ok(_)` inghiotte anche `Released::AlreadyCollected`, il contatore `Observed::already_collected` resta **0** su ogni seme, e **il `match` continua a compilare**. ✅ **I due lettori, misurati e non dedotti:** la guardia di non-vacuita' alla riga **590** (*«no holder ever overran its window: the sweep is not being exercised from this side»*) e il **primo dei due testimoni** di `property_5_expiry_frees_the_budget_under_the_scenario` alla riga **705**. ⚖️ **La notizia buona, detta per non gonfiare la voce: sono `assert!`, quindi si va ROSSI e non vacui** — `scripts/gate.sh` lo coglie. ⛔ **Ma il rimedio non e' meccanico**, e va deciso invece che improvvisato sul rosso: i tre rami diventano `Ok(Released::Now(_))`, `Ok(Released::AlreadyCollected)` e un `Err` che in questo scenario — **un** arbitro, concessioni proprie — non e' piu' raggiungibile. ⚠️ **E il doc di `already_collected` diventa FALSO:** dice *«Hand-backs that found the grant ALREADY OFF THE BOOKS -- `ReleaseError::UnknownGrant`»*. E' la radice **R1**, creata dal compito stesso, e la chiude il passo 5 della disciplina dell'audit: *ricontà i conteggi che il tuo rimedio ha reso stantii*. 📌 **Il censimento dei chiamanti di `release`, che il compito non da':** `grep -rn "\.release(" crates/ --include=*.rs` rende **dieci siti in due file**, nove in `arbiter_admission.rs` e uno qui. |
| **E5** | ⚠️ **REGISTRATA E NON PRESA, ed e' del proprietario: il `#[must_use]` che il commit 1c mette su `Started` e su `Killed` non e' tenuto da niente.** Toglierlo lascia l'intero workspace verde, cioe' e' un **mutante vivo** dal giorno in cui nasce. ⛔ **E non e' decorativo:** `process.start(grant, desc);` scartato **lascia cadere una concessione** che nessuno puo' ricostruire, che e' precisamente il difetto per cui `R6` esiste — quindi un `#[must_use]` che sparisse in silenzio **riaprirebbe la voce che questo compito chiude**. 📌 **La forma con cui questo repository tiene una proprieta' di livello 1 e' un caso `compile_fail`**, cioe' una **riga di catalogo nuova** in §7.4, che e' **spec**: vincolo globale 7. Stesso trattamento di `PL-1` e di `K-1`/`B-1`, stessa ragione (gotcha **#36**: una nota si legge e si dimentica, una voce aperta no). ⚖️ **Il precedente gemello e' gia' aperto:** `E47` ① del Traguardo 5 e' la voce sul `#[must_use]` **mancante** di `promote` — questa e' la stessa domanda dal lato opposto. |
| **E6** | ⚠️ **«I quarantadue siti» di `Parameters::new` sono TRENTANOVE.** `grep -rn` ne rende 42, ma **tre sono prosa** — `parameters_have_no_default.rs:18` e `:23`, `trust_has_no_default.rs:45`. 📌 E' il gotcha **#70** dentro la voce **P-4**, che il gotcha #70 lo cita: *il `grep` restituisce candidate, non case, e ogni riga che rende si legge intera*. ⚖️ **Non cambia il lavoro**, perche' i tre non compilano comunque nulla; cambia il numero che chi esegue si aspetta di veder cadere. |
| **E7** | ⚠️ **I «nove casi `compile_fail`» ne toccano SETTE, e la previsione di P-4 sui `.stderr` e' falsa a META'.** P-4 dice *«due di essi nominano `Parameters` nel proprio `.stderr` — `parameters_have_no_default` e `two_policies_at_once`»*. ✅ **Misurato eseguendo:** a cambiare e' stato **soltanto** `parameters_have_no_default.stderr`; `two_policies_at_once.stderr` e' rimasto `ok` **invariato**, perche' la sua `note` cita la firma di `Arbiter::new` e non quella di `Parameters::new`. ⛔ **La direzione dello sbaglio e' quella buona** — il lavoro era **piu' piccolo** di come il pre-controllo lo prezzava — ed e' il gotcha **#65** nel verso che di solito non si guarda. |
| **E8** | ⛔ **LA MUTAZIONE DI CONTROLLO DEL PASSO 7 DI 1c NON UCCIDEVA, E LA CAUSA NON ERA LA MUTAZIONE: ERA IL BANCO.** La prima lettura di questa voce diceva che il passo era **sotto-specificato** — *«un `Grant` di un secondo arbitro» e' ambiguo, perche' un secondo arbitro costruito dallo stesso aiutante ha lo stesso `ArbiterId`* — e prescriveva di mutare con un identificativo diverso. ✅ **La revisione di qualita' ha misurato che la diagnosi era rovesciata:** a tenere viva la mutazione era il **letterale `ArbiterId::new(1)` condiviso** da `a_real_grant` e da `an_arbiter_and_a_real_grant` in `crates/kernel/tests/worker_tokens.rs`. Dato all'aiutante usa-e-getta un `ArbiterId::new(2)`, **la mutazione dettata dal piano uccide** — `left: Err(UnknownGrant)`, `right: Ok(Now(Mib(1024)))`. ⛔ **Il piano aveva ragione e il banco no**, ed e' il gotcha **#65** applicato a una **mutazione**: si prezza leggendo il codice che la mutazione tocca, non la sua descrizione. 📌 **La regola generale, che vale oltre il caso:** una mutazione che sostituisce un valore con un altro **della stessa classe di equivalenza** non e' una mutazione — e qui la classe la definisce `issuer`, cioe' il meccanismo che questo stesso compito ha costruito. |
| **E9** | ⚠️ **Il commit 1b rende rosso un `.stderr` che nessun passo nomina — `grant_has_no_constructor`.** `Grant` guadagna il campo privato `issuer`, quindi la `note` passa da *«private field `id`»* a *«private fields `id` and `issuer`»*. ⚖️ **L'oracolo non si spegne, si RAFFORZA:** ora pinza anche la privatezza di `issuer`, che e' il campo su cui poggia la risposta nuova di `release`. Resta che il commit 1b **non ha nessun passo per i `.stderr`** — li ha soltanto 1a e 1c — quindi il rosso arriva a chi esegue senza un'istruzione che lo aspetti. |
| **E10** | ⛔ **IL COSTO DEI DOC E' LA PARTE PIU' GRANDE CHE IL COMPITO NON NOMINA, ed e' misurabile.** Il diff di `crates/kernel/src/arbiter/mod.rs` e' di **284 righe**, e sono **109 di commento contro 29 di codice**. ✅ **Sei blocchi diventano falsi e vanno riscritti col richiamo datato**, non quattro come la prima stesura di questa voce diceva — il conto lo rifa' `git diff <base>..<head> \| grep -c "^+.*2026-08-30"`. Fra essi **uno era falso gia' da prima**: il doc di `Grant` attribuiva a `id` la distinzione fra concessioni proprie e altrui, che e' di `issuer`. ⛔ **E due frasi sono state rese false in file che il compito NON elenca**, trovate solo dalla revisione: `arbiter_admission.rs` e la campagna DST. 📌 **La lezione, e vale per ogni compito futuro di questo traguardo:** il censimento che serve non e' *«chi CHIAMA la funzione che cambio»* ma *«chi la NOMINA»* — `grep -rn "\.release("` rende 14 righe, `grep -rnE "release\|UnknownGrant\|AlreadyCollected"` ne rende **131**, e le frasi false stavano nella differenza. |
| **E11** | ⚠️ **REGISTRATA E NON PRESA — le due sonde di 1c provano che torna UNA concessione, non che torna QUELLA.** ✅ **Il perimetro e' stato ristretto due volte con la misura, e il residuo e' molto piu' piccolo di come la prima stesura lo dichiarava.** Prima: *«il buco e' un limite di `Grant`, che non deriva `PartialEq` e non espone `id`»* — **falso**, ed era una diagnosi del coordinatore, non dell'esecutore: cambiando **un solo letterale** (`E8`) tre mutazioni su quattro sono passate da sopravvivere a morire. ⛔ **Ciò che resta aperto e' una riga sola:** una finta che sostituisse una concessione **dello stesso arbitro** passerebbe ancora, perche' per `release` due arbitri con la stessa identita' **sono un arbitro solo**. ⛔ **Dichiarata e non pinzata** (gotcha **#73**): pinzarla pretenderebbe di distinguere due concessioni dello stesso emittente, cioe' di cambiare `Grant`, che e' il tipo che `E30`/`R6` hanno appena fissato. 📌 **La forma che la chiuderebbe e' la suite di conformita' di §6.10**, che nasce col canale worker vero — non qui. La dichiarazione con la misura vive accanto alle due sonde in `crates/kernel/tests/worker_tokens.rs`. |
| **E12** | ⚠️ **REGISTRATA E NON PRESA — `release` non e' tenuto da nessuna sonda in DUE punti, e il primo non e' quello che sembra.** ⛔ **Il CONFINE `now == expires_at` E' TENUTO**, misurato: mutando `collect_expired` da `expires_at <= now` a `< now` muoiono **due** sonde, `a_grant_is_collected_at_the_instant_its_window_closes` in `crates/kernel/tests/arbiter_admission.rs` e `a_permanent_grant_survives_to_the_last_instant_of_the_axis_and_is_swept_at_it` **nel `mod tests` di `crates/daemon/src/main.rs`** — e la seconda vive in un'altra crate, il che e' la meta' che una lettura frettolosa sbaglia. ⛔ **A non essere tenuto e' `release` A quel confine**, che e' un corollario di quelle due piu' la sonda a `5_001`: **rifiutato di proposito** invece che dimenticato. ⛔ **Il buco vero e' l'altro: i due lati della GRAZIA di una revoca.** Nessun banco rilascia una concessione sotto revoca — la campagna DST passa a `LocalPolicy` **dopo** `executor.run()` — quindi quella coppia di valori, che il doc di `ReleaseError` dichiarava misurata, oggi non la tiene nulla. ⚖️ **Il chiusore e' il compito che dara' alla revoca un chiamante**, non questo. |
| **E13** | ⚠️ **GUARDATA E NON PRESA, con la ragione — `crates/kernel/src/arbiter/mod.rs`, il doc di `collect_expired`.** Dice *«both comparisons are `>`»*, mentre il confronto su `expires_at` e' scritto `if held.expires_at <= now { return false; }` dentro una chiusura a due clausole: logicamente la stessa cosa, tipograficamente no. ⛔ **NON e' la stessa specie della citazione tolta nello stesso commit** — quella fingeva un `retain(…)` **verbatim** che non esisteva; questa enuncia una **proprieta'** (*le due finestre sono semiaperte*), e la proprieta' e' **vera e misurata**: mutando `<=` in `<` muoiono due sonde. ⚖️ **Lasciata di proposito:** riscrivere prosa **non falsa** e' cio' che il gotcha **#76** ha misurato costare piu' di quanto compri. ⚠️ **L'attrito che resta e' reale e va detto:** chi cerca col `grep` un `>` su `expires_at` non lo trova. Se il prossimo che tocca quel doc vuole scioglierlo, sappia che non sta correggendo un errore. |
| **E14** | ⚠️ **TRE file di `tests/compile_fail/` portano la riga da 123 colonne SENZA la nota che gli altri quattro hanno, e l'esclusione e' MISURATA.** La nota che dichiara *perche'* la riga non si spezza sta in `admission_has_no_is_granted.rs`, `admission_reads_cold_start.rs`, `admission_without_profile.rs` e `two_policies_at_once.rs`. **Manca** in `instructing_after_the_kill.rs`, `reading_twice_from_one_receipt.rs` e `reading_without_a_receipt.rs`. ✅ **Perche', verificato:** quei tre sono lunghi **120-125 righe**, quindi una nota in **fondo** starebbe un centinaio di righe dopo cio' che spiega — e non verrebbe letta — mentre una in **testa** sposterebbe i riferimenti d'oracolo che stanno a 119-124, cioe' disferebbe proprio cio' che la riga lunga compra (precedente **AUD-042/045**). ⛔ **Dichiararli costerebbe una rigenerazione di `.stderr` per file**, che e' piu' di cio' che comprano. ⚠️ **Chi accorcia una di quelle righe rompe un oracolo e non trovera' nessun commento a dirglielo.** |
| **E15** | ⚠️ **DUE DOMANDE DI FORMA PER IL PROPRIETARIO, registrate e non prese, nessuna delle quali e' un difetto.** ① **`Arbiter::id` e' un SECONDO IDIOMA per leggere un parametro consegnato:** `total_vram` si legge come `self.parameters.total_vram()`, mentre `arbiter_id` viene **sollevato in un campo** dell'arbitro. La giustificazione scritta accanto (*un valore che non cambia mai*) varrebbe identica per `total_vram`, che sollevato non e'; e `Parameters` e' `Copy` con `arbiter_id()` `const`, quindi il campo si potrebbe togliere. ⚖️ **Non e' un difetto** — il doc dichiara che e' una copia, e `new` ne e' l'unico lettore, verificato — ma sono **due modi di dire una cosa** dentro lo stesso tipo. ② **`Started::Rejected` si legge come una contraddizione:** il tipo risponde *«che cosa ha fatto l'avvio»*, e una delle due risposte e' *«non e' mai partito»*. `Admission` non ha il problema perche' nomina l'**atto**, non l'esito. ⛔ **Il nome e' dettato dal piano**, quindi la domanda e' del proprietario e non dell'esecutore. |
| **E16** | ⚠️ **IL PUNTATORE DEL PROSSIMO PASSO HA DUE CASE, ed e' preesistente a questo traguardo.** La §6 del compendio dichiara di sé di essere *«la casa unica»* del `⏭️`, e il censimento `grep -rn "⏭️" docs/ CLAUDE.md` conferma che ogni riga viva fuori dal compendio **nomina** la §6 invece di riscriverla. ⛔ **Ma questo piano ne porta una propria**, la sezione *«Il prossimo passo, in forma eseguibile»* della sua testa, che dice **quale compito** eseguire — e va tenuta in passo con la §6 a mano, perche' nessun controllo le confronta. ⚖️ **Le due non sono la stessa cosa** — la §6 punta il passo del **progetto**, questa la posizione **dentro il piano** — ed e' la ragione per cui non e' stata tolta. ⚠️ **Registrata perche' il prossimo che chiude un compito le aggiorni ENTRAMBE**, e perche' se il proprietario vuole una casa sola sappia che questa e' la seconda. ✅ **CHIUSA il 2026-08-31 — decisione del proprietario: la seconda casa è TOLTA.** ⛔ **E la ragione per cui questa voce non l'aveva tolta è FALSIFICATA, non ignorata:** diceva che *«le due non sono la stessa cosa — la §6 punta il passo del progetto, questa la posizione dentro il piano»*, ma la §6 scrive **il numero del compito** — *«IL PROSSIMO PASSO È IL COMPITO 4»* — cioè esattamente la posizione dentro il piano: le due case dicevano la stessa cosa nelle **stesse unità**. ⚠️ **E la difesa che questa voce proponeva — *«il prossimo che chiude un compito le aggiorni ENTRAMBE»* — ha ceduto alla PRIMA occasione:** il commit del 3bis ha aggiornato la §6 e lasciato ferma la sezione, in questo stesso file. 📌 *Registrare una seconda casa non la difende; solo toglierla la difende.* ⛔ **RICHIAMO DEL 2026-08-31: questa riga chiudeva con *«corollario del gotcha #68»*, e la lezione ha ricevuto un NUMERO PROPRIO per decisione del proprietario — è il gotcha **#94** di [`HANDOFF.md`](../../HANDOFF.md), che ne è la casa unica.** Ciò che il #68 non diceva è che fra *riallineare* e *togliere* esiste una **terza via che sembra un rimedio e non lo è**: annotare il duplicato e delegarne il riallineamento a chi verrà — che è esattamente ciò che questa voce aveva fatto. |
| **E17** | ⛔ **BLOCCANTE — LA SONDA `the_byte_string_annotation_is_measured_and_not_asserted` E' VACUA COSI' COM'E' DETTATA, e la sua mutazione non uccide niente.** Il corpo che il Passo 1 di 3b detta e' `alloc_vec(&[0u8; 4096])`, e **tutti gli zeri stanno nel range a un byte del CBOR** (0..=23): un array di numeri e una stringa di byte costano allora **lo stesso**. ✅ **Misurato su una sonda usa-e-getta scritta da fuori la crate, compilata, eseguita e cancellata nella stessa corsa** — non dedotto: con `[0u8; 4096]` il corpo e' **4102 byte con l'annotazione e 4102 senza**, rapporto **1,00x**, e la forma incorniciata (**4106**) sta sotto il limite `< 4096 + 64` **in entrambi i casi**. ⛔ **Quindi la prima riga della tabella delle mutazioni del Passo 7 uccide ZERO sonde, non «quella sola».** ✅ **Con byte reali il divario c'e' ed e' quello della §6.10.4:** `(0..4096).map(|i| (i % 256) as u8)` da' **4102 contro 7814**, rapporto **1,90x** — che riproduce il *7813 contro 4101* di quella sezione a meno del byte di busta. 📌 **Rimedio: il corpo della sonda porta byte fuori dal range a un byte**, e la ragione si scrive accanto, perche' e' precisamente cio' che rende la sonda non vacua. E' il precedente di `E144` al Task 12 del Traguardo 5: *il compito cosi' com'e' scritto non puo' passare* — qui in una forma peggiore, perche' **passerebbe verde**. |
| **E18** | ⛔ **LA PRIMA MUTAZIONE DI 3a NE UCCIDE DUE, non «quella sola», e la seconda morte e' legittima.** ✅ **Misurato eseguendo le cinque sonde contro le due varianti del modulo**, non dedotto: `to_le_bytes` in entrambi i siti lascia verdi `a_framed_body_comes_back_exactly`, `a_truncated_frame_is_refused` e `bytes_shorter_than_the_prefix_are_refused`, e fa rossi `the_declared_length_is_four_bytes_big_endian` **e `a_frame_with_a_tail_is_refused`**. ⛔ **La causa non e' la simmetria fra i due siti** — che e' l'unico caso che la clausola del piano nomina, e il round-trip **sopravvive**, quindi la proprieta' per cui la clausola esiste **regge**. E' che la sonda della coda porta un **letterale big-endian**: `00 00 00 01` letto little-endian vale **16 777 216**, quindi il corpo da tre byte e' *troncato* e non *con una coda*, e la risposta passa da `TrailingBytes` a `Incomplete`. 📌 **Rimedio: la colonna «Deve uccidere» della prima riga diventa «`the_declared_length_is_four_bytes_big_endian` e `a_frame_with_a_tail_is_refused`, e NON il round-trip».** ⚠️ **Senza questa voce chi esegue si ferma su un rosso che il piano gli ordina di trattare come un difetto**, ed e' il caso opposto: la mutazione fa esattamente il proprio lavoro. |
| **E19** | ⛔ **IL BLOCCO DI `crates/kernel/src/wire/worker.rs` NON PORTA NESSUN `use`, e nessuno dei nomi che adopera e' in scope.** Mancano `Encode`, `Decode`, `Vec`, `Mib`, `framing`, `WireError` e `minicbor` — il tipo, le due funzioni e i due derive li usano tutti. ⚠️ **`crates/kernel/src/framing.rs` invece il proprio `use alloc::vec::Vec;` ce l'ha**, quindi lo scarto e' fra i due blocchi dello stesso compito e non una convenzione del piano. E' il precedente del Task 9 del Traguardo 5, dove *«tutti gli import mancavano»* era una delle due voci **bloccanti per costruzione**. 📌 **Non cambia il disegno, cambia se il Passo 4 compila.** |
| **E20** | ⚠️ **Il Passo 6 di 3b attende «`6 passed` e `5 passed`»: le sonde di `crates/kernel/tests/worker_wire.rs` sono CINQUE.** Ricontate sul blocco che il Passo 1 detta: `a_fragment_survives_the_round_trip`, `a_vram_peak_survives_the_round_trip`, `the_byte_string_annotation_is_measured_and_not_asserted`, `a_frame_with_a_tail_does_not_decode`, `junk_inside_the_declared_length_does_not_decode`. Quelle di `framing.rs` sono cinque pure, e li' il Passo 5 di 3a dice **cinque**: la cifra sbagliata e' una sola. ⚖️ **Non cambia il lavoro**, cambia il numero che chi esegue si aspetta di vedere — e un'attesa sbagliata di uno si legge come una sonda che non ha compilato. |
| **E21** | ⛔ **LA TESTA DI QUESTO PIANO DICHIARAVA «le tre parti scritte finora NON HANNO TOCCATO UNA RIGA DI PRODOTTO», e il comando che vi stava accanto per verificarlo la smentiva.** Trovata **riprendendo il 2026-08-31**, non eseguendo: il compito 1 ha toccato il prodotto, e `git diff --name-only a70d563..HEAD -- crates/ scripts/ Cargo.lock` nomina i suoi file — mentre la clausola scritta li' diceva *«se nomina qualcosa, la frase e' falsa»*. ⚠️ **E la tabella delle parti, tre righe piu' su, era gia' aggiornata a «Parte A ESEGUITA»:** una correzione che attraversa una casa e non l'altra, gotcha **#85**, dentro la sezione che si dichiara casa unica dello stato del piano. ✅ **TOLTA e non riallineata**, col richiamo datato: era una quantificazione universale su una popolazione che **ogni compito** muove, cioe' la radice **R1** dell'audit. Cio' che resta e' la regola — *ogni compito rimisura la propria baseline* (**D5**). |
| **E22** | ⚠️ **Il Passo 2 del commit 3b attende `error[E0432]` e il rosso vero e' `error[E0433]`, piu' un `E0599` che il compito non nomina.** Misurato eseguendo. ⚖️ **Non cambia il lavoro**, cambia il codice d'errore che chi esegue confronta col passo — e un rosso diverso da quello atteso si legge come *«ho sbagliato qualcosa»* invece che come *«il passo ha sbagliato la previsione»*. E' la specie di `E7` al compito 1. |
| **E23** | ⚠️ **Il Passo 4 del commit 3b detta `crates/kernel/src/wire/mod.rs` e `wire/worker.rs` ma NON dice di dichiarare `pub mod wire;` in `lib.rs`** — che il Passo 4 del commit **3a** invece dice per `framing`. Senza, i due file non entrano nella crate e il banco non compila. 📌 Non e' una decisione: e' una riga che manca al passo, ed e' la stessa specie di **E19**. |
| **E24** | ⚖️ **L'aiutante `alloc_vec` che il Passo 1 di 3b nomina NON e' stato scritto, ed e' una decisione presa eseguendo.** Il compito lo introduce con *«scrivilo come preferisci — ma **non** dargli un doc che prometta qualcosa»*; scritto, avrebbe avuto **un solo** chiamante e avrebbe incapsulato `to_vec()`, cioe' sarebbe stato l'**aiutante che non tiene niente** contro cui il compito stesso mette in guardia (precedente: il Task 8 del Traguardo 5). ⛔ **Dichiarata e non taciuta**, perche' un nome che il piano detta e il codice non porta si legge come una dimenticanza. |
| **E25** | ⚠️ **Il piano e' INTERNAMENTE INCOERENTE sulla forma dell'annotazione di `minicbor`, e la divergenza e' fra due suoi passi.** Il Passo 3 di 3b prescrive per `Mib` *«la forma che `record.rs` usa gia', non una che questo piano ricorda»*; il Passo 4 detta poi per `Fragment` la forma **combinata** `#[cbor(n(0), with = "minicbor::bytes")]`, che `record.rs` **non** usa — li' sono **due attributi su due righe**, `#[n(3)]` e `#[cbor(with = ...)]`. ✅ **Entrambe compilano**, misurato nel pre-controllo su una sonda usa-e-getta scritta da fuori la crate. ⚖️ **Risolta usando la forma di `record.rs` in entrambi i siti**, cioe' obbedendo alla regola del Passo 3 invece che al letterale del Passo 4: una convenzione sola batte due modi di dire la stessa cosa. |
| **E26** | ⛔ **IL PROTOCOLLO DELLE MUTAZIONI DELLA `D7` HA UN ORACOLO VACUO, e vale per OGNI compito futuro di questo piano.** Il Passo 6 di 3a detta `git diff --stat <file>` per verificare che la revoca sia completa, con la nota *«deve essere VUOTO»*. ⛔ **Su un file che git non traccia ancora — e `framing.rs` non lo e' fino al commit del Passo 8 — quel comando e' vuoto SEMPRE**, revoca riuscita o no: e' verde per la ragione sbagliata, cioe' precisamente la specie di oracolo che questo repository chiama vacuo. 📌 **La forma che tiene e' `cmp` contro la copia byte-esatta presa prima**, che non dipende dallo stato dell'indice. Usata in tutte le mutazioni di questo compito. |
| **E27** | ⛔ **LE CINQUE SONDE CHE IL COMPITO DETTA PER `worker_wire.rs` NON RAGGIUNGONO IL RAMO D'ERRORE DI `FromWorker::decode`, e nessun passo lo nota.** Trovato dalla revisione, non dall'esecuzione. Misurato: sostituito `.map_err(\|_\| WireError::Malformed)?` con `.expect("decode")` — cioe' *il kernel va in panico su un frame malformato che arriva da un worker* — l'intero workspace resta **verde, identico alla baseline cifra per cifra**. ⛔ **E la meta' che rende il rilievo bloccante:** il Passo 4 fa poggiare `let _ = minicbor::encode(...)` sul precedente di `Record::encode` e ordina di **rileggerlo**; riletto, quel doc non dice *«l'errore e' irraggiungibile, quindi si scarta»* ma che il caso impossibile e' **contenuto**, e il contenimento e' una sonda **nominata** su **due** ingressi. ✅ Chiuse con `an_empty_body_in_an_honest_envelope_does_not_decode` e `a_truncated_body_in_an_honest_envelope_does_not_decode` — **due `#[test]` separati**, perche' con uno solo il secondo ingresso non sarebbe mai esercitato (gotcha **#14**). Sotto la mutazione muoiono **quelle due e nessun'altra**, riprodotto dal coordinatore. |
| **E28** | ⛔ **LA PROSA SCRITTA PER CHIUDERE IL GOTCHA #31 NE HA COMMESSO UNO, e il rilievo che la coglieva sottocontava a sua volta.** La passata che ha tolto le cifre duplicate ha scritto che §6.10.4 *«e' la loro casa»*: le cifre `7813`/`4101`/`1,91x` vivono in **piu'** case, e §6.10.4 e' solo una. ⚠️ **La revisione ne nominava due e l'affermazione stava in due siti e non uno**, il secondo in `crates/kernel/tests/worker_wire.rs`: chiuderne uno solo sarebbe stato il difetto stesso che il rilievo denunciava. ✅ **Chiusa TOGLIENDO** e non riallineando a un numerale — un conteggio dentro un doc e' cio' che era appena marcito — e la passata ha messo **quindici** righe contro **dodici** tolte: e' il gotcha **#76** applicato una seconda volta. |
| **E29** | ⛔ **IL CENSIMENTO DELLE CASE DI C-1 È PIÙ GRANDE DI COME I `Files:` DEL 3BIS LO PREZZANO, ED È LA RADICE `R1` DENTRO IL COMPITO CHE DECIDE.** Il compito nomina quattro case — `crates/kernel/Cargo.toml`, [`riferimenti.md`](../../riferimenti.md), [`porta-di-qualita.md`](../../porta-di-qualita.md) e il `Cargo.lock` del caso B — ma la frase *«si decide al Traguardo 6»* vive anche altrove, e il verdetto la rende stantia **comunque vada**. ✅ **Misurato col comando e non ricordato:** `grep -rniE 'bincode\|§6\.1\.1\|3bis\|RUSTSEC' crates/ scripts/ docs/*.md`, e ogni riga restituita letta **intera** (gotcha **#70**). ⛔ **Tre case vive che il compito NON nomina, stantie in ENTRAMBI i casi:** la riga *«schema IPC»* della **§4 del compendio**, che scrive *«registrato il 2026-08-18, si decide al Traguardo 6»*; il **gotcha #64** di [`HANDOFF.md`](../../HANDOFF.md), che chiude con *«⛔ APERTA — registrata, non decisa»*; e la **chiusa della sezione C-1** di `riferimenti.md`, *«si decide allora, mentre la scelta è ancora libera»* — quel file è fra i `Files:`, ma per **aggiungere le fonti**, e né il passo né il criterio di chiusura nominano l'affermazione che ci sta già. ⚠️ **Due case in più SOLO nel caso B**, dichiarate perché non si confondano con le prime tre: il doc di modulo di `crates/kernel/src/ports/ipc.rs` (*«Milestone 6 brings the SCHEMA -- `bincode` in `kernel`»*) e la riga `ipc` della tabella di **ADR-0037** in §5 del compendio (*«§6.1.1 confermata, non riaperta»*) restano **vere** nel caso A. ⚖️ **Due che NON si toccano, guardate in faccia invece che saltate:** la scheda **C-1** di [`audit-2026-08-11.md`](../../audit-2026-08-11.md) e il riquadro su M-11 di `HANDOFF.md` sono **verbali datati** — 55ª misura, *dentro un verbale datato un'affermazione regge*. ⚠️ **E `porta-di-qualita.md` non ha NESSUNA occorrenza di `C-1`:** i `Files:` dicono *«Modify … la voce»*, ma la riga è **nuova** e non esistente — precedente **E154** del Traguardo 5, *l'elenco dei file di un compito è un'affermazione come le altre*. 📌 **Nessuna di queste riapre una decisione: ALLARGANO il perimetro**, ed è il gotcha **#65** nella direzione che costa di più. E la manutenzione di [`CLAUDE.md`](../../../CLAUDE.md) lo diceva già — *alla chiusura di ogni voce si aggiornano `COMPENDIO.md` e `HANDOFF.md`* — che è precisamente ciò che i `Files:` non riflettono. |
| **E30** | ⛔ **E29 HA CENSITO CINQUE CASE E NE MANCAVA UNA, ED È UN DIFETTO DEL COORDINATORE E NON DEL PIANO: il puntatore `⏭️` della §6 del compendio.** È la casa che quel documento dichiara **unica** per *«stato e prossimo passo»*, diceva *«IL PROSSIMO PASSO È ESEGUIRE IL COMPITO 3BIS»*, ed è falsa **il minuto dopo** che il compito gira — cioè stantia comunque vada il verdetto, come le altre tre. A trovarla è stato **chi eseguiva**, non il pre-controllo. ⛔ **E la causa è misurata, non supposta: il censimento LA AVEVA RESTITUITA.** `grep -rn '6\.1\.1' docs/*.md` rende quella riga, e chi scriveva E29 l'ha letta come *«il §6 parla del prossimo passo»* invece che come **una casa da correggere**. 📌 È il gotcha **#70** nella sua forma dichiarata — *il `grep` restituisce candidate, non case, e ogni riga che rende si legge INTERA* — commesso dentro la voce che il **#70** lo cita. ⚠️ **E c'è una seconda metà da dire invece che tacere:** E29 scrive come propria misura un comando **unificato** su `crates/ scripts/ docs/*.md`, mentre i comandi realmente lanciati furono **due e separati**, e quello sui documenti era **sensibile alle maiuscole** — la riga scrive `3BIS`, quindi il termine `3bis` non la pescava e a pescarla è stato il solo `6.1.1`. 📌 *Un comando scritto in un verbale è un'affermazione come le altre, e va lanciato nella forma in cui lo si scrive* — gotcha **#65** applicato alla propria riproduzione. ✅ **Corretta dal compito**: la §6 dichiara ora il 3bis eseguito, l'esito **caso B**, e il prossimo passo come **decisione del proprietario**. |
| **E31** | ⛔ **IL COMPITO 4 HA DUE RAMI E LA MISURA DEL 3BIS NON ENTRA IN NESSUNO DEI DUE: il candidato vivo è un TERZO ramo, ed è quello che il piano non poteva prevedere.** La tabella del compito 4 offre *«`bincode` (il ramo che il manifesto ha oggi)»* e *«`minicbor` (se il 3bis lo ha scelto)»*, cioè **tenere il formato** oppure **cambiarlo**. ⚖️ **Il candidato che il 3bis ha trovato non è né l'uno né l'altro:** `bincode-next` è un **fork che dichiara lo stesso formato sul filo**, quindi cambierebbero il **nome della crate** e il **percorso del derive** — e con essi la lista di **§7.3.1** e la riga di `scripts/gate-deps.sh` sul grafo transitivo — mentre **il formato e il pari resterebbero dove sono**, `bincode-ts` immutato dal 2025-07-17. 📌 **La conseguenza per chi esegue, e non è una rifinitura:** se il proprietario sceglie quel ramo, il compito 4 va **riscritto**, non ramificato — la sua colonna `bincode` detta `#[derive(bincode::Encode, bincode::Decode)]` e `bincode::encode_to_vec(...)`, cioè **nomi**, e un fork wire-compatibile cambia i nomi lasciando in piedi tutto il resto. ⚠️ **Registrata e non presa:** quale ramo sia è la **D12**, del proprietario, e riscrivere il compito 4 prima che la scelta ci sia sarebbe prenderla per omissione — che è precisamente ciò che la **D4** vieta. ⛔ **E non contraddice la D18:** *«il derive di `bincode` è DA VERIFICARE»* resta vero, e su un fork lo è due volte. ✅ **CHIUSA il 2026-08-31, e non perché fosse sbagliata: il suo condizionale non è mai scattato.** Il proprietario ha deciso che `bincode` 2.0.1 **resta**, quindi il compito 4 va sul ramo che il piano ha già scritto e **non** si riscrive. ⚠️ **La voce RESTA e non si cancella**, perché ciò che insegna sopravvive alla propria occasione: *quando un piano offre due rami, la misura può rispondere con un terzo* — e a coglierlo fu la lettura del compito 4 contro l'esito del 3bis, non il pre-controllo. |
| **E32** | ⛔ **BLOCCANTE — IL BANCO DETTATO NON RAGGIUNGE IL RAMO D'ERRORE DI `decode`, ED È LA MUTAZIONE `W9` RIFATTA SUL CANALE NUOVO.** Le quattro sonde dettate sono `a_grant_request_survives_the_round_trip`, `a_verdict_survives_the_round_trip`, `a_message_with_a_tail_does_not_decode` e `junk_inside_the_declared_length_does_not_decode`, e **nessuna** arriva al `map_err(|_| WireError::Malformed)`: i due giri completi **decodificano**, la coda **fuori** dalla busta è respinta da `framing::unframe` **prima** che il decodificatore giri, e la coda **dentro** la lunghezza dichiarata è presa dal confronto fra byte consumati e lunghezza del corpo, che è un controllo **diverso**. ⛔ **Non è una lacuna teorica: è la stessa che la revisione del compito 3 ha MISURATO il 2026-08-31** — mutazione `W9`, che lasciava **l'intero workspace verde** — e ha chiuso con **due** sonde nuove in `crates/kernel/tests/worker_wire.rs`, `an_empty_body_in_an_honest_envelope_does_not_decode` e `a_truncated_body_in_an_honest_envelope_does_not_decode`, il cui commento scrive per esteso che quel `map_err` **non era raggiunto da nessuna sonda del workspace**. ⚠️ **Il piano non poteva conoscerle: è stato scritto il 2026-08-30 e quelle sonde sono nate il giorno DOPO.** È la regola **5** di [`CLAUDE.md`](../../../CLAUDE.md) — *un compito scritto prima si legge contro il codice di ADESSO, non contro il piano* — nella sua forma più cara, perché il banco gemello è **il modello che il compito dice di seguire**. 📌 **Rimedio: `ipc_wire.rs` nasce con SEI sonde e non quattro**, le due nuove modellate sulle gemelle e come **due `#[test]` separati** — gotcha **#14**, una sonda sola si ferma alla prima asserzione fallita e non esercita mai il secondo ingresso, e il gemello lo scrive accanto a sé. ⚠️ **E la tabella delle mutazioni cresce di una riga:** quella che rompe il ramo d'errore deve uccidere **le due nuove e nient'altro**; quale forma prenda su questo formato si sceglie **leggendo il codice scritto**, e si scrive quale — come il Passo 7 già prescrive per la mutazione del discriminante. |
| **E33** | ⛔ **BLOCCANTE — I TRE TIPI NUOVI NON PORTANO NESSUN DERIVE, E LE SONDE DETTATE NE PRETENDONO QUATTRO.** Il blocco del Passo 5 detta `pub struct GrantRequest`, `pub enum Verdict` e `pub enum IpcMessage` **senza una riga `#[derive(...)]`**, e il Passo 4 copre **solo i quattro tipi condivisi** — `Mib`, `ComputeClass`, `Preemption`, `Millis` — che è ciò che il suo titolo dice e ciò che la tabella di **P-10** elenca. ⛔ **Così com'è dettato non compila:** `encode` e `decode` pretendono `bincode::Encode` e `bincode::Decode`, e `assert_eq!(IpcMessage::decode(&bytes), Ok(message))` pretende `Debug` e `PartialEq`. 📌 **La forma giusta è quella del gemello:** `crates/kernel/src/wire/worker.rs` scrive `#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]` su `FromWorker`. ✅ **E LA CONVIVENZA DEI DUE DERIVE — CHE IL PASSO 4 AFFERMA SENZA MISURARLA — È ORA MISURATA, il 2026-08-31**, con una patch usa-e-getta applicata e revocata **byte-esatta** nella stessa corsa: aggiunti `bincode::Encode, bincode::Decode` accanto a quelli di `minicbor` su `Mib` — che porta `#[cbor(array)]` e `#[n(0)]` — e su `ComputeClass`, `Preemption` e `Millis`, `cargo check --locked -p kernel` esce **0**. ⛔ **Quindi il derive di `bincode` regge su un ENUM e in `no_std`**, che è la metà della **D18** che un ricordo non distingue. ⚠️ **E tre fatti letti nel sorgente vendorizzato invece che ricordati** — `ls -d ~/.cargo/registry/src/*/bincode-2.0.1`: `pub trait Decode<Context>: Sized` porta un parametro, ma `decode_from_slice` lo **fissa a `()`** e restituisce già i byte consumati (`Result<(D, usize), DecodeError>`); `encode_to_vec` prende il valore **per valore**, e regge su `&self` perché `impl<T> Encode for &T` esiste (`enc/impls.rs:484`); e `encode_to_vec` vive in `features/impl_alloc.rs`, cioè dietro la feature `alloc` che il manifesto **ha già**. |
| **E34** | ⛔ **IL PASSO 8 ① PREZZA UNA CASA CHE ERA DIVENTATA FALSA IN UN ALTRO MODO, E LA FALSITÀ NON ERA DEL COMPITO: ERA UN RESIDUO DELLA DECISIONE C-1.** `crates/kernel/src/ports/ipc.rs` portava un richiamo datato **2026-08-31** scritto dal 3bis — *«THE FORMAT NAMED ABOVE IS NO LONGER SETTLED … The MEASUREMENT is done; the CHOICE is not … the open entry in docs/porta-di-qualita.md»* — e **tutte e tre le clausole erano false**: la scelta c'è, la voce è chiusa, il formato è di nuovo fissato. ⛔ **Il commit che decide C-1 (`4f2a859`) tocca cinque case e non questa**, quindi il sorgente ha mentito per un giorno. ✅ **Chiusa dal commit `b90ea9b`**, col precedente che il manifesto di questa crate ha ricevuto lo stesso giorno: l'**intestazione** si corregge perché è la cornice che si legge (gotcha **#31**), il **corpo** resta verbatim perché è un verbale, e la decisione si appende sotto. Censimento col `grep` e ogni riga letta **intera** (gotcha **#70**): era l'unica casa rimasta. ⚠️ **E il Passo 8 ① va letto con questo in mano:** dice *«il formato è quello che il **3bis** ha deciso»*, e il 3bis **non ha deciso** — si è fermato prima, caso B — a decidere è stato il **proprietario**. Il richiamo che il compito deve ancora scrivere è quindi sulla sola **metà del timbro**: la metà del formato è già chiusa. 📌 **Il compito chiede MENO del necessario in una direzione e PIÙ nell'altra**, gotcha **#65** in entrambi i versi dentro lo stesso passo. |
| **E35** | ⚠️ **I `Files:` PREZZANO `crates/kernel/src/wire/mod.rs` COME «il `pub mod ipc;`», E QUEL FILE PORTA UNA RIGA CHE IL COMPITO RENDE FALSA.** La riga 7 dice *«`ipc` IS NOT HERE YET: it arrives with task 4, in the format that task 3bis decides»*, e diventa falsa **nello stesso commit** che aggiunge il modulo. È il precedente **E154** del Traguardo 5 — *l'elenco dei file di un compito è un'affermazione come le altre, e si legge contro il codice* — ed è esattamente la lezione di **P-7** che la testa della Parte C invoca per giustificare il commit unico: *la prosa che il codice rende falsa deve atterrare nello stesso commit del fatto che la smentisce*. 📌 **Il rimedio è una riga TOLTA, non riscritta:** ciò che il modulo contiene lo dicono i suoi `pub mod`, che non possono marcire — forma di **AUD-046**. |
| **E36** | ⚠️ **LA SONDA DETTATA SCRIVE `good[4..]` DOVE ESISTE UNA COSTANTE ESPORTATA, E IL GEMELLO LA USA.** `kernel::framing` espone `pub const LENGTH_WIDTH: usize = 4`, e `crates/kernel/tests/worker_wire.rs` scrive `let body = &good[LENGTH_WIDTH..];` in **due** sonde; la riga d'import dettata, `use kernel::framing::WireError;`, diverge pure dal gemello, che fa `use kernel::framing::{self, LENGTH_WIDTH, WireError};` e chiama `framing::frame(...)` invece di qualificarlo per intero. ⛔ **Non è tipografia:** se la busta cambiasse larghezza il gemello resterebbe verde e questo banco andrebbe rosso **per la ragione sbagliata** — un letterale che nessun controllo lega al valore che copia, cioè la specie che [`CLAUDE.md`](../../../CLAUDE.md) chiude scrivendo *il comando e non il numero*. |
| **E37** | ⚠️ **REGISTRATA, NON PRESA — `G5` E `G-5` VIVONO NELLO STESSO DOCUMENTO, E LI DISTINGUE UN TRATTINO.** Le mutazioni di questo canale si chiamano `G1`…`G5` e stanno nella tabella del compito 4 di [`porta-di-qualita.md`](../../porta-di-qualita.md); nello **stesso file** vive il finding **`G-5`** dell'audit del 2026-08-11 — quello che ha reso `--locked` un **ingresso** del cancello invece che un suo effetto. ⛔ **E la nota che li disambigua fu scritta quando la collisione NON era ancora concreta, misurato e non supposto:** a `8269971` quel file porta già la nota *«non sono il finding `G-5`, che porta il trattino»* e **quattro** righe `G`; la quinta nasce a `1de8f37`, il giorno dopo. Il comando che lo rifa: `for c in 8269971 1de8f37; do git show $c:docs/porta-di-qualita.md \| grep -c '^\| \*\*G'; done`. ⚖️ **Registrata e non presa:** rinominare le mutazioni toccherebbe **ogni** cella più il rapporto, e la nota disambigua **ancora correttamente** — il difetto è latente, non attivo. 📌 **Il chiusore è la crescita: se il canale guadagna altre mutazioni, vale una lettera diversa.** Decisione del proprietario. |
| **E38** | ⚠️ **REGISTRATA, NON PRESA — IL DOC DI `GrantRequest` È MOLTO PIÙ DENSO DI PROSA DEL GEMELLO, E IL CANDIDATO A ESSERE TOLTO È IL BLOCCO SUI DERIVE.** ⛔ **Nessuna cifra qui, i due comandi:** `grep -c '^[[:space:]]*//' crates/kernel/src/wire/ipc.rs crates/kernel/src/wire/worker.rs` per le righe di commento e `grep -cE '^[[:space:]]*[^/[:space:]]' crates/kernel/src/wire/ipc.rs crates/kernel/src/wire/worker.rs` per quelle di codice — le cifre invecchierebbero al primo commit che tocca i due file, il rapporto fra loro no. ⛔ **Il candidato è il blocco sui derive**, metà del quale documenta **perché `Debug` e `PartialEq` ci sono** — cosa che il compilatore già dice. ⚖️ **Registrata e non presa, con la ragione:** quel blocco è il **verbale di una decisione aperta del proprietario** — `Eq` e `Clone` senza consumatore — e si accorcerà **da sé** quando il proprietario risponderà; sfoltirlo adesso sarebbe editare prosa che sta per cambiare, cioè pagarla due volte. |
| **E39** | ⚠️ **REGISTRATA — IL LETTERALE `[0x00, 0x00, 0x00, 0x00]` È LA SPECIE CHE `E36` HA CHIUSO, E LA CASA DEL RIMEDIO È IL BANCO GEMELLO.** `crates/kernel/tests/ipc_wire.rs` scrive `assert_eq!(bytes, [0x00, 0x00, 0x00, 0x00]);` — i quattro byte del prefisso di lunghezza, a mano — mentre **lo stesso file**, poche righe sopra e poche righe sotto, usa già la costante esportata `kernel::framing::LENGTH_WIDTH` che `E36` vi ha messo: `grep -n 'LENGTH_WIDTH' crates/kernel/tests/ipc_wire.rs`. ⛔ **Ma `crates/kernel/tests/worker_wire.rs` porta la riga IDENTICA**, ed è **fuori** dai file di questo compito — `grep -rn '0x00, 0x00, 0x00, 0x00' crates/kernel/tests/` le rende **entrambe**. Correggerne una sola lascerebbe i due banchi divergenti sulla **stessa** asserzione, che è il difetto che `E28` ha già pagato una volta in questo stesso traguardo. ⚖️ **Registrata, chiusore: chi tocca il gemello** — i due si correggono **insieme** o non si correggono. |
| **E40** | ⚠️ **REGISTRATA E NON PRESA, ed è del proprietario — IL RESIDUO `M1` DEL COMPITO 3: `crates/kernel/tests/worker_wire.rs` DICHIARA UN'ESCLUSIVITÀ CHE NON HA.** Il commento della sonda `the_byte_string_annotation_is_measured_and_not_asserted` chiude con *«the figures are not copied here, and this comment is the house of the ones below (gotcha #31)»* — cioè *«la casa è questa»* — e la casa **non è una**: il **4106** incorniciato e i due rapporti **1,00x** e **1,90x** vivono anche nella voce **`E17`** di questa stessa errata, che li ha misurati. ✅ **Censito col `grep` il 2026-08-31 e non dedotto** — `git grep -n '4106\|7818\|1\.90x\|1,90x' -- .` — con **ogni riga letta intera**, perché il `grep` rende candidate e non case (gotcha **#70**): dei quattro file che nomina **due sono candidate**, [`riferimenti.md`](../../riferimenti.md) e il piano del [Traguardo 3](2026-08-10-sottoprogetto-1-traguardo-3-giornale-e-formato-durevole.md), che portano un **altro** `4106` — la taglia di un `RecordV1` intero, misura di un'altra famiglia. Le case vere sono **due**: il commento ed `E17`. ⛔ **Non è un difetto di prodotto** — nessun codice cambia e nessuna sonda si muove — ed è l'ultima esclusività rimasta della famiglia che il compito 3 ha chiuso: il lato **documento** di `M1` è già chiuso, e la cella `W4` di [`porta-di-qualita.md`](../../porta-di-qualita.md) è oggi un **rimando** — *«le cifre non stanno qui»* — che punta proprio a quel commento. ⚖️ **Le due opzioni, ENTRAMBE scritte perché la scelta non è dell'esecutore:** **A)** togliere quelle poche parole, e una **sottrazione** non può introdurre un'affermazione nuova perché ne toglie una; **B)** dichiararla voce aperta e lasciare il commento com'è. ⛔ **Nessuna delle due è presa qui, e il residuo resta APERTO.** 📌 **Perché questa voce nasce adesso, a compito chiuso:** finora la scelta viveva **solo** nel ledger locale di `superpowers:subagent-driven-development`, che git **ignora** — quindi su un'altra macchina chi riprende avrebbe trovato una frase falsa **senza nessuna nota accanto**. È la specie che `E43` registra per intero. |
| **E41** | ⚠️ **REGISTRATA E NON PRESA — LA RIGA DEI FINE-RIGA DI [`CLAUDE.md`](../../../CLAUDE.md) HA IL SOGGETTO SBAGLIATO, E QUEL FILE È L'UNICA LETTURA OBBLIGATORIA.** La riga *«I fine-riga sono misti per file»* della tabella *«Come si lavora qui»* prescrive *«Chi **scrive** uno strumento che tocca file conserva i fine-riga di quel file»*, e l'unico strumento che nomina è `sed -i`. ⛔ **Ma `cargo fmt` non lo si scrive, lo si LANCIA** — e ha fatto scattare la trappola **due volte, con nome e data**: il **2026-08-21** su `crates/kernel/tests/ports_are_implementable.rs`, uno dei **quattro** file `i/crlf`, dove `git diff --stat` dichiarò righe che nessuno aveva toccato (il verbale sta in [`porta-di-qualita.md`](../../porta-di-qualita.md) e nella §6 del [compendio](../../COMPENDIO.md)); e il **2026-08-31**, dentro il **compito 4** di questo piano, su `crates/kernel/src/time.rs` — **senza danno nel diff**, perché quel file è `i/lf` e `core.autocrlf` ha assorbito la conversione. ⛔ **Nessun cumulativo qui, per costruzione:** quante volte la trappola sia scattata in tutto lo si ricava dai verbali — `git grep -n 'sed -i\|cargo fmt' -- docs` — e un numeratore scritto in questa cella invecchierebbe alla prossima (gotcha **#31**). Ciò che non invecchia è il **soggetto**, e il soggetto è sbagliato. ⚠️ **E la forma *«`cargo fmt` normalizza i fine-riga»* NON è in [`HANDOFF.md`](../../HANDOFF.md):** `grep -c 'cargo fmt' docs/HANDOFF.md` dà **zero**, misurato — vive **solo** in [`porta-di-qualita.md`](../../porta-di-qualita.md), che **non è lettura obbligatoria**. Quindi la regola che ogni sessione legge non copre il caso più frequente, e quella che lo copre non la legge nessuno per obbligo. 📌 **Il rimedio è UNA PAROLA nel soggetto** — *scrive* → *scrive o lancia* — non un paragrafo nuovo: la prosa che chiude il buco esiste già altrove (*«la regola vale per QUALUNQUE strumento che riscriva un file»*). ⚖️ **Non presa perché tocca il contratto d'ingresso di ogni sessione futura**, cioè il modo di lavorare: è del proprietario. |
| **E42** | ⛔ **SOMMARE LE RIGHE `test result:` DELL'USCITA DI `bash scripts/gate.sh` NON DÀ LA BASELINE: CONTA TRE BERSAGLI DUE VOLTE.** Il cancello ha un **settimo passo che non è un settimo controllo**, e i suoi commenti lo dichiarano: rilancia `dst_campaign`, `arbiter_campaign` e `engine_crash_consistency` con `--nocapture` per il **solo** scopo di stampare il **tempo di parete**, che il vincolo **7** pretende a ogni corsa — le loro asserzioni sono già girate nel `cargo test --locked --workspace` di un passo più su. ⛔ **Quindi la quaterna sommata dal cancello è SEMPRE più alta di quella vera, e lo scarto è esattamente quei tre bersagli**, con le loro passate e le loro ignorate. ✅ **Misurato il 2026-08-31 lanciando entrambi**, e ⛔ **le due quaterne NON sono scritte qui: sono scritti i due comandi.** È la scelta che [`CLAUDE.md`](../../../CLAUDE.md) prescrive — *un numero misurato non si scrive, si scrive il comando* — e qui compra più del solito, perché una quaterna invecchia al **primo compito** che aggiunge una sonda, mentre la relazione *«il cancello ne conta tre in più»* regge finché regge la lista del settimo passo. 📌 **I due comandi, provati:** la baseline canonica è quella della **D5**, `cargo test --locked --workspace --no-fail-fast \| awk '/^test result:/{n++;p+=$4;f+=$6;g+=$8} END{print n" · "p" · "f" · "g}'`; la stessa `awk` su `bash scripts/gate.sh` rende la quaterna gonfiata, e ⚠️ **la pipe si mangia la riga `GATE GREEN`** — il verdetto del cancello è il suo codice d'uscita, non questo conto. ⚖️ **Ci è cascato il controllore di questa sessione**, e una quaterna sbagliata scritta in un verbale è precisamente la radice **R1** — la stessa che `E21` ha pagato in testa a questo piano. |
| **E43** | ⛔ **IL LEDGER DI `superpowers:subagent-driven-development` NON RAGGIUNGE `origin`, E LO STATO DI CHI RIPRENDE DA UN'ALTRA MACCHINA NON STA LÌ.** `.superpowers/sdd/progress.md` — con i pacchetti di revisione e i rapporti dei compiti — è **ignorato da git**: `git check-ignore -v .superpowers/sdd/progress.md` risponde `.superpowers/sdd/.gitignore:1:*`. La skill lo prescrive come *«mappa di recupero»* e dice di controllarlo alla ripresa; ma [`AVVIO-CHAT.md`](../../AVVIO-CHAT.md) dichiara **due macchine**, e sull'altra quel file **non esiste**. ✅ **NESSUN PUNTATORE ROTTO, misurato e non supposto:** `git grep -n 'progress\.md\|\.superpowers' -- docs CLAUDE.md`, **ogni riga letta intera** — nessuna manda a leggerlo: tre lo dichiarano **esplicitamente** non tracciato o ignorato (il gotcha **#80** di [`HANDOFF.md`](../../HANDOFF.md), la riga **5** della §10 del [compendio](../../COMPENDIO.md), la riga dei perimetri di [`audit-2026-08-27.md`](../../audit-2026-08-27.md)) e due lo citano come **verbale** nell'errata del [Traguardo 5](2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu.md). ⛔ **Il buco non è un puntatore: è che nessun documento tracciato dice DOVE lo stato vive davvero** — e chi arriva dall'altra macchina lo scopre per assenza. ✅ **Vive in quattro case, e nessuna di esse è il ledger:** il **prossimo passo** nella **§6 del [compendio](../../COMPENDIO.md)**, casa unica; la **posizione dentro il piano** nella tabella delle parti in testa a questo file; le **sonde e le mutazioni** in [`porta-di-qualita.md`](../../porta-di-qualita.md); le **divergenze** in questa errata. La baseline non è fra esse ed è deliberato: la **D5** dice di **rimisurarla**, non di leggerla. ⛔ **E IL BRIEF DI UN COMPITO SI RIGENERA, non si recupera:** è un estratto di questo piano. ⚠️ **Lo script `task-brief` della skill NON funziona qui, misurato:** cerca intestazioni `^#+[ \t]+Task[ \t]+N` e questo piano scrive `### Compito N`, quindi su questo file rende **zero righe**. ✅ **La forma provata il 2026-08-31 su tutti e dieci i compiti**, con `n` uguale al numero della colonna *«Compiti»* della tabella delle parti: `awk -v n=4 '/^#+[ \t]+Compito[ \t]+[0-9]+/{t=($0 ~ ("^#+[ \t]+Compito[ \t]+" n "([^0-9a-z]\|$)"))} t' <questo piano>`. 📌 **`([^0-9a-z]\|$)` non è tipografia:** senza di esso `n=3` inghiottirebbe anche il **3bis**, e la guardia sui blocchi di codice che lo script originale porta qui non serve — verificato confrontando le due forme su tutti e dieci, uscite **identiche**. |
| **E44** | ⛔ **`crates/kernel/src/sensor.rs` COSÌ COM'È DETTATO NON COMPILA, E IL COMMIT 5c NE NOMINA UNDICI COSE CHE LA LISTA `use` NON HA.** Il commit **5a** detta in testa `use alloc::vec::Vec;`, `use crate::boundary::Untrusted;` e `use crate::time::Millis;`, e **nessun passo successivo tocca quella lista**; ma `run_the_ring` nomina `Journal`, `JournalError`, `StepId`, `Record`, `RecordV1`, `RecordKind`, `EffectClass`, `Trust`, `Detail`, `VerdictDetail` e `String` — e `kernel` è `#![no_std]` con `extern crate alloc`, quindi **`String` non è nel prelude** e va importato come fa già `record.rs` (`use alloc::string::String;`). ✅ **MISURATO, non dedotto:** il file dettato è stato scritto, `lib.rs` ha ricevuto il `pub mod sensor;` dopo una copia byte-esatta, e `cargo build --locked -p kernel` ha reso **19 errori** — `E0405` su `Journal`, `E0425` su `StepId` e `JournalError`, `E0433` su `Trust`, `String`, `Record`, `RecordKind`, `EffectClass` e `Detail`, `E0422` su `RecordV1` e `VerdictDetail` — poi `lib.rs` è stato **ripristinato dalla copia** e `sensor.rs` cancellato, con `git status --porcelain` **vuoto** e i CR di `lib.rs` invariati a **0**. ⛔ **È il precedente del Task 9 del Traguardo 5**, dove *«tutti gli import mancavano»* fu una delle due voci bloccanti per costruzione. ⚠️ **E `use alloc::vec::Vec;` è NON USATO in entrambi i commit** — misurato nella stessa corsa, `warning: unused import: alloc::vec::Vec` — perché né il 5a né il 5c nominano mai `Vec`: `to_vec()` è un metodo inerente della slice e il tipo non compare. ⛔ **Non rende rosso il cancello**, che non passa `-D warnings` — verificato leggendo `scripts/gate.sh` — ma il progetto tiene `cargo build --locked --workspace` a **zero avvisi**, quindi lasciarlo è una regressione della qualità dichiarata e non un dettaglio. 📌 **Rimedio: la lista `use` si scrive al commit 5c contro ciò che il file nomina davvero**, e `Vec` esce. |
| **E45** | ⛔ **DUE SONDE SU TRE DI `tests/sensor_ring.rs` NON POSSONO PASSARE: L'ANELLO SCRIVE UNA NOTA SU UN PASSO CHE NESSUNO HA APERTO.** `run_the_ring` scrive il verdetto con `journal.note(step, &record)?`, e il contratto della porta lo vieta a chiare lettere — `crates/kernel/src/ports/journal.rs`, doc di `note`: *«THE WRITE-AHEAD DISCIPLINE APPLIES: a note for a step with NO INTENT is `OutOfOrder`. A note is an annotation UPON something, and a step nobody opened is not something.»* — e `MemoryJournal::note` lo **fa rispettare**, con `if !self.has_intent(step) { return Err(JournalError::OutOfOrder); }`. Le tre sonde dettate partono tutte da `MemoryJournal::new()`, cioè da un archivio **vuoto**, e chiamano `run_the_ring(.., StepId::new(1), StepId::new(2), ..)`. ✅ **MISURATO da fuori la crate**, con una sonda usa-e-getta compilata, eseguita e cancellata nella stessa corsa: `note` su un giornale vuoto risponde **`Err(OutOfOrder)`**. Quindi il `?` propaga e `.expect` fa **panicare** `a_passing_sensor_writes_a_verdict_and_opens_nothing` e `a_failing_verdict_opens_a_new_step_and_carries_the_detail`; passa la sola `an_inferential_sensor_is_refused_by_the_tight_ring`, che **ritorna prima** di `note` — e passerebbe anche se l'anello fosse rotto in ogni altro modo. ⛔ **È la forma di `E144` al Task 12 del Traguardo 5:** *il compito così com'è scritto non può passare*. ⚠️ **E IL COSTO NON È LA RIGA DI SETUP: SONO GLI ORACOLI.** Aprire il passo 1 con un `intent` prima di chiamare l'anello — la via coerente con ADR-0007, perché un artefatto giudicato appartiene a un passo che **esiste** e chi lo ha aperto è il chiamante, non l'anello — mette un record in più nell'archivio: `written.len()` passa da **1 a 2** nella prima sonda e da **2 a 3** nella seconda, gli indici `written[0]`/`written[1]` **slittano**, e l'ultima asserzione della terza sonda, `assert!(records(&journal).is_empty())`, diventa **falsa** e va riformulata come *«nessun record oltre l'intento»* — altrimenti si perde esattamente la metà che il suo stesso commento dichiara di comprare. ⚖️ **La seconda via — che sia l'anello ad aprire il passo — è registrata e scartata sul merito**, non sul costo: darebbe a `run_the_ring` la facoltà di aprire passi, cioè gli farebbe fare ciò che il suo stesso doc attribuisce al chiamante quando rifiuta di allocare `next`. 📌 **E una conseguenza di merito da dichiarare accanto al codice:** `run_the_ring` risponde `Err(OutOfOrder)` su un passo mai aperto, e il suo doc dettato **non lo dice**. |
| **E46** | ⚠️ **RICHIAMO DEL 2026-09-01 — QUESTA VOCE DICEVA IL FALSO, ED È CORRETTA INVECE CHE CANCELLATA.** Diceva: *«la cifra di `P-13` nella prosa della Parte D è stantia»*, perché `grep -rn 'RecordV1 {' crates/ --include=*.rs` rende **39** righe contro le *«ventisei»* che la Parte D prezza. ⛔ **Ma 39 è il `grep` GREZZO, e `P-13` dichiara di averlo già scartato:** quella sezione porta il comando **filtrato** — che toglie i commenti, la definizione della struttura, l'`impl fmt::Debug`, la stringa di formato, il tipo di ritorno e le due righe in cui il censimento conta **sé stesso** — e la **D20** porta il proprio richiamo del 2026-08-30, *«qui stavano trentanove siti in undici file … riletto intero è ventisei in nove»*. ✅ **RIMISURATO IL 2026-09-01 col comando filtrato di `P-13`: 26 siti in 9 file, di cui 2 `compile_fail`.** La cifra **regge esattamente**, e regge anche la scomposizione delle tredici righe escluse: i **commenti di doc** che mostrano un record a cinque campi sono **sei**, ricontati. ⛔ **Il difetto era di chi ha scritto questa voce, e ha un nome:** la Parte D è stata prezzata leggendo la **prosa** invece di `P-13`, che è la casa del conteggio — gotcha **#65**, *un rapporto si prezza leggendo il codice*, nella direzione che chiede **più** del necessario. 📌 **Ed è la ragione per cui la voce resta invece di sparire:** una voce d'errata cancellata non insegna nulla a chi rifarà lo stesso conto, e questo piano ha già pagato due volte per un numerale letto nella casa sbagliata. ✅ **Ciò che era vero e resta vero:** il numerale *«il 34° caso `compile_fail`»* **regge**, ricontato col comando che il compito stesso ordina — `ls crates/kernel/tests/compile_fail/*.rs` ne dava **33** prima del commit 5a. |
| **E47** | ✅ **`Detail` SENZA `#[cbor(array)]` METTE SUL FILO GLI STESSI BYTE DEL GEMELLO CHE CE L'HA — MISURATO, e la voce esiste perché la domanda tocca l'ARTEFATTO IRREVERSIBILE.** Il Passo 2 del commit 5b detta `Detail` con `#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]` e **nessun** attributo `#[cbor(..)]`, mentre `Record` — l'unico altro enum-con-dati della crate, e quello che **avvolge** questo — porta `#[cbor(array)]` esplicito. Se il default divergesse, il quarto record congelato nascerebbe in una forma che nessuno ha scelto, e i byte congelati **non si rigenerano** (ADR-0036). ✅ **Misurato su `minicbor` 2.3.0, la versione del manifesto, in un progetto usa-e-getta fuori dal repository:** le due forme rendono **`82 00 81 82 f4 07`** entrambe, sei byte, **identiche**. ⛔ **Quindi non è un difetto sui byte e la scelta è libera**; ciò che resta è che i due soli enum-con-dati della crate porterebbero **due forme diverse** per lo stesso comportamento, ed è il genere di asimmetria che qualcuno *«sana»* dopo, senza sapere che era stata misurata. 📌 **Registrata con entrambe le opzioni e nessuna presa** — aggiungere l'attributo per simmetria, o lasciarlo e scrivere accanto che il default è già quello: è del proprietario. 📌 **E la misura si porta dietro l'inquadratura che il Passo 8 dovrà leggere:** `82 00 81` è la busta della variante, `82 f4 07` l'array a due del `VerdictDetail`, con **`f4`** per `passed: false` e **`07`** per `spent_millis: 7`. |
| **E48** | ⛔ **`core.autocrlf` VALE `false` SU QUESTA MACCHINA, E I DOCUMENTI LO DANNO `true`.** Il [compendio](../../COMPENDIO.md) lo scrive come misura — *«`core.autocrlf` vale `true`»* — e l'[audit del 2026-08-27](../../audit-2026-08-27.md) apre le proprie trappole con *«`core.autocrlf` è `true` su questa macchina»*. ✅ **Rimisurato il 2026-09-01 riprendendo da un `git fetch`:** `git config core.autocrlf` risponde **`false`**, e `git ls-files --eol` dà **4** file `w/crlf` contro **239** `w/lf`, cioè l'albero di lavoro **coincide** con l'indice — i quattro `w/crlf` sono esattamente i quattro `i/crlf`. ⚠️ **Nessuno dei due documenti mentiva:** entrambi dicono *«su questa macchina»*, e [`AVVIO-CHAT.md`](../../AVVIO-CHAT.md) dichiara che le macchine sono **due** — la stessa lacuna che **E43** ha trovato per il ledger, e per la stessa causa: un fatto d'**ambiente** scritto senza dire **di quale** ambiente. ⛔ **La conseguenza per chi esegue è concreta e va saputa prima**, perché ogni compito di questo piano ordina di rimisurare i fine-riga: qui la conversione in checkout **non avviene**, quindi un file che nasce LF resta LF e i CR contati prima e dopo una riscrittura devono coincidere **a zero** per tutto tranne quei quattro file. ✅ **E il piano stesso ne porta la traccia, verificata invece che supposta:** il messaggio del commit `42f5c69` chiude con *«fine-riga del piano conservati (CR == righe, 4603)»*, mentre qui lo stesso file rende **0 CR su 4603 righe** ed è `i/lf w/lf`. 📌 **Registrata e non presa:** se e dove i documenti debbano qualificare **quale** macchina hanno misurato è del proprietario, e tocca il contratto d'ingresso — stessa ragione di **E41**. |
| **E49** | ⛔ **DUE DIFETTI DI `frozen_bytes.rs` CHE IL COMPITO NON PREVEDE, trovati eseguendo il commit 5b, e il secondo è di merito.** **①** La guardia di non-vacuità della mappa portava la costante **3** scritta a mano — `assert_eq!(sections.len(), 3, "…there are three frozen files")` — quindi il quarto record la faceva rossa per la ragione sbagliata. ✅ Ora legge `the_frozen_records().len()`, e con ciò tiene **entrambe** le direzioni: misurato togliendo il quarto record dalla lista, va rossa anche di là. **②** ⛔ **`the_three_frozen_records_are_distinguishable_in_the_bytes` pretendeva che TUTTI i record congelati avessero la STESSA LUNGHEZZA e differissero solo nei byte 4..7** — e il quarto, che per la **D21** deve portare `Some`, ne ha **27** contro 21 e differisce anche all'intestazione d'array (`85` → `86`) e in tutta la coda. **Asserire l'uguale lunghezza fra arità diverse sarebbe asserire che la D21 non è avvenuta.** ✅ **Il rimedio è ristretto e la parte che conta è ciò che NON si tocca:** la proprietà *«la differenza sta dove la mappa dice»* vale ora sulle coppie di **uguale arità**, mentre l'`assert_ne!` — *«due record congelati che codificano uguale ne pinzerebbero uno solo»* — resta su **tutte** le coppie, arità comprese. Ciò che si perde è il **dove** della differenza, che fra due forme diverse non significa niente. ⚠️ **E il nome contava i propri soggetti:** rinominato `the_frozen_records_are_distinguishable_in_the_bytes`, sul precedente di `E40` del Traguardo 5 — censito col `grep`, nessun'altra casa. 📌 **La classe è quella del gotcha #29:** una partizione scritta quando i membri erano omogenei lascia scoperto il primo membro di forma diversa, e nulla lo segnala perché l'asserzione *sembra* generale. |
| **E50** | ⛔ **`crates/simulator/tests/dst_campaign.rs` PORTA UN `match` ESAUSTIVO SU `RecordKind` CHE LA MAPPA DEI FILE DEL COMPITO NON NOMINA, e non è un literal: è un ORACOLO che dichiara indipendenza.** `expected_doubt` modella la riconciliazione **senza leggere l'implementazione**, e lo dichiara di sé in quattro paragrafi; la variante nuova lo rende `error[E0004]`. ⛔ **Scrivervi l'arm vuoto che `reconcile` scrive sarebbe stato il difetto che quel file si vieta:** *«da quel momento l'oracolo concorderebbe con l'implementazione PER COSTRUZIONE — anche quando l'implementazione è sbagliata»*. ✅ **Risolto con la regola che il file stesso enuncia per il caso irraggiungibile** — quella del secondo intento: un verdetto **non può** entrare in quella traccia, perché niente lì esegue un sensore, quindi l'arm **fallisce rumorosamente** invece di essere assorbito. Se un giorno lo scenario crescerà un sensore, *«il rosso è una decisione che viene chiesta, non un difetto che viene riportato»*. 📌 **E la lezione per i compiti 6 e 7, che aprono altre due varianti:** l'elenco `Modify:` di un compito che tocca `RecordKind` deve contenere **anche** questo file, e non per un literal — gotcha **#65**, un elenco di file è un'affermazione come le altre. |
| **E51** | ⛔ **LA MUTAZIONE `M3` ERA UN MUTANTE VIVO, E IL COMPITO LO AVEVA PREVISTO COME POSSIBILITÀ: lo era davvero.** Spostato il controllo del costo dichiarato **dopo** `observe`, l'intero workspace resta **verde** — misurato il 2026-09-01, mutazione revocata da copia. La ragione, letta e non supposta: le tre sonde guardavano soltanto ciò che veniva **scritto**, e un sensore che gira e viene poi scartato non scrive niente lo stesso. ⚠️ **Ma il doc di `run_the_ring` dice che quell'ordine *«IS V11»***, cioè un'affermazione tenuta da nulla. ✅ **CHIUSO CON UNA SONDA IN PIÙ e non con un'asserzione più larga**, che è ciò che il compito prescrive: `an_inferential_sensor_is_never_run_at_all` usa un sensore che **registra di essere stato eseguito**, e asserisce in **due** direzioni — l'inferenziale non gira, il computazionale sì, senza la quale la prima riga sarebbe verde per un anello che non esegue niente. Sotto `M3` diventa rossa, rimisurato. ⛔ **E la sonda NON congela nessuna decisione aperta**, che è la condizione del gotcha **#73**: §6.4.1 rifiuta un inferenziale dall'anello stretto perché eseguirlo *«turns every step into two inferences»*, quindi *«un inferenziale non viene eseguito»* **è** V11 e non una scelta che qualcuno debba ancora fare. ⚠️ **VOCE APERTA REGISTRATA E NON PRESA, come il compito ordina:** le sonde di `crates/kernel/tests/sensor_ring.rs` e le due di `reconciliation.rs` **non hanno una riga di catalogo propria** — solo `V10` ce l'ha. La §7.4 è **spec**, vincolo globale 7, stesso trattamento di `PL-1`, `K-1`/`B-1` e delle sonde del Traguardo 5, stessa ragione (gotcha **#36**). |
| **E52** | ✅ **IL CRITERIO DI CHIUSURA PRETENDE «i SEI commenti di doc che mostravano un record a cinque campi portano il proprio richiamo datato», E LA MISURA DICE CHE NESSUNO DI ESSI DIVENTA FALSO.** Il numerale **regge** — censite il 2026-09-01, le occorrenze in commento sono **otto**, meno le **due** righe in cui il sorgente cita il comando che le conta: **sei**, esattamente ciò che `P-13` scompone. ⛔ **Ma la loro NATURA non è quella che il compito assume.** Guardate una per una (gotcha **#70**), **cinque** sono citazioni dentro un **verbale datato** — l'uscita che una misura produsse quel giorno, con la data a poche righe: `boundary.rs` due volte (2026-08-18, finding **P-1**; 2026-08-28, finding **AUD-050**), `record.rs` due volte (le stesse due date), e il caso `promote_reason_is_not_runtime_text.rs` (2026-08-18, *«measured before the fix»*). ⛔ **Un verbale non invecchia, ed è la distinzione della 55ª misura del compendio** — *dentro un verbale datato una cifra regge, dentro una voce aperta mente* — la stessa per cui l'audit del 2026-08-27 tolse `APERTO` da cinquantotto schede e **lasciò** le quindici che portavano `✅ CORRETTO — commit`. ✅ **E la sesta non è datata ma resta VERA per un'altra ragione:** il doc di `Record` scrive `V1(RecordV1 { .. payload: <N bytes> })`, con l'ellissi al posto dei campi, e ciò che quella riga afferma — il `Debug` derivato delega all'impl interno e il payload resta chiuso — è vero con cinque campi come con sei. ⚖️ **Quindi NON è stato toccato niente, ed è una scelta e non un'omissione:** appendere un richiamo datato a un verbale per dire che il mondo è cambiato **dopo** che il verbale fu scritto è ciò che `CLAUDE.md` chiama ricorreggere invece di togliere, e allungherebbe cinque blocchi senza rendere vera una sola frase che oggi sia falsa. ⚠️ **La più vicina al confine è dichiarata invece che taciuta:** la citazione di **AUD-050** in `record.rs` è l'unica che mostra **tutti** i campi **per nome** senza ellissi, quindi è l'unica che un lettore potrebbe copiare come *«la forma del `Debug` di oggi»*. Regge perché la sua riga d'apertura è `DATED RECALL, 2026-08-28 — FINDING AUD-050` e ciò che segue è dichiarato **misurato quel giorno**; se qualcuno decidesse che non basta, il rimedio è l'ellissi e non un richiamo — ed è del **proprietario**, perché tocca un verbale. |
| **E53** | ⛔ **IL COMPITO 5 È STATO ESEGUITO IN SESSIONE E NON SUBAGENT-DRIVEN. ✅ IL CICLO DI REVISIONE È STATO FATTO IL 2026-08-31, DA UNA SESSIONE FRESCA E NON DA UN SOTTO-AGENTE, E HA TROVATO TRE RILIEVI DI CUI DUE MUTANTI VIVI.** ⚠️ **La voce NON è cancellata e il residuo NON è azzerato: è RIDOTTO, e la differenza va detta.** L'intestazione di questo piano prescrive *«un subagente fresco per compito con revisione fra uno e l'altro»*; i sotto-agenti erano disabilitati il giorno dell'esecuzione e **lo erano ancora** il giorno della revisione — a una seconda domanda diretta è stato risposto *«decidi secondo i criteri»*, che è **letteralmente ciò che questa voce aveva già registrato non essere il permesso**. ✅ **CHE COSA COMPRA UNA SESSIONE FRESCA, e non è poco:** chi ha rivisto **non aveva scritto quel codice**, che è la proprietà che questa voce dichiarava comprare i rilievi — *«un revisore fresco legge il prodotto senza sapere che cosa l'autore intendeva»*. E li ha comprati: `E54`, `E55` ed `E56`, due dei quali sono **mutanti vivi sull'intero workspace** che le cinque mutazioni dettate non potevano vedere, perché **nessuna delle cinque li toccava**. ⛔ **CHE COSA NON COMPRA, misurato e non temuto:** chi ha rivisto aveva letto la §6 del compendio e l'intestazione del piano **prima** di aprire il codice, quindi conosceva già `E51`, `E52` e il gotcha **#95**. La contaminazione è sui rilievi **già trovati** e non su quelli da trovare — ma è reale, ed è meno indipendenza di quella che un subagente ha per costruzione. E **un solo giro**, non *«si rivede finché una passata non torna pulita»*. 📌 **IL RESIDUO CHE RESTA, per chi riprende:** il compito 5 ha avuto **una** revisione dove gli altri compiti di questo traguardo ne hanno avute da tre a cinque, e quella revisione ha trovato **tre** difetti reali al primo giro — che è la ragione per cui la regola dice *«finché una passata non torna pulita»*. ⛔ **Non si chiude eseguendo il compito 6**, e non si chiude nemmeno con questa riga: si chiude con **un secondo giro**, sul perimetro che ora è più largo perché la revisione stessa ha scritto codice. |
| **E54** | ✅ **CORRETTO — `Detail` È IL QUARTO ENUM SUL FILO E NON AVEVA NESSUNA GUARDIA DI CRESCITA: UNA VARIANTE NUOVA ENTRAVA NEL FORMATO DUREVOLE SENZA CHE NIENTE DIVENTASSE ROSSO.** ⛔ **Misurato nelle due direzioni il 2026-08-31, non dedotto.** Aggiunta a `Detail` una seconda variante `#[n(1)]`, `cargo test --locked --workspace --no-fail-fast` ha dato **41 bersagli, 297 passate, 0 fallite, 2 ignorate** — **identico alla baseline cifra per cifra**; la stessa aggiunta a `RecordKind` dà `` error[E0004]: non-exhaustive patterns ``. ⛔ **Perché non è cosmetico:** gli indici di `Detail` sono **sul filo** e non si ritirano mai (regola 4 di §4.9.2), e `frozen_bytes.rs` dichiara di sé che *«una variante aggiunta a `RecordKind` FERMA LA COMPILAZIONE DI QUESTO FILE»* e che *«una variante nuova di questi enum è un CAMBIO DI FORMATO»*. Quella garanzia era vera di **tre** enum e affermata per il formato: è la forma del difetto che il Task 10 del Traguardo 3 pagò — *«un record solo fissa tre indici di variante su otto»*. ✅ **Il rimedio è nella forma che il file già usa** — `match` esaustivo più asserzione di non-vacuità — e **non** più forte dei tre fratelli: estendere l'arm senza congelare un record compila ancora, che è il **limite dichiarato** che gli altri tre portano. Renderlo più forte da solo sarebbe una seconda convenzione per una proprietà sola (§7.4.4). ⚠️ **E il nome del test contava i propri soggetti:** `every_variant_of_the_three_enums_…` → `every_variant_of_the_wire_enums_…`, **rinominato e non riallineato a quattro**, sul precedente che quello stesso file aveva già posto per `the_three_frozen_records_…`. |
| **E55** | ✅ **DECISA DAL PROPRIETARIO IL 2026-08-31 E CHIUSA: LA CLASSE LA CONSEGNA IL CHIAMANTE.** `run_the_ring` guadagna `correction_effect: EffectClass`, e la sonda `the_class_of_the_corrective_step_is_the_one_the_caller_delivered` la pinza su **due** valori, **nessuno dei quali è il letterale vecchio** — un anello che ignorasse l'argomento fallirebbe su entrambi. ⛔ **La mutazione che era viva ora muore**, col proprio messaggio. ⚖️ **La ragione che decide non è la lettera di ADR-0007 ma la COERENZA: è ciò che quella stessa funzione fa una riga sopra** — *«`next` È CONSEGNATO E NON ALLOCATO … inventarne uno qui prenderebbe quella decisione scrivendola»* — ed è la forma di ADR-0034. L'anello non sa che cosa farà la correzione; chi lo sa è il chiamante. ⚠️ **Costo dichiarato:** un sesto parametro, e i quattro siti di chiamata esistenti lo passano. **Nessun conflitto coi compiti 6–10**, verificato: nominano `run_the_ring` una volta sola e come *scrittore di record*, mai la sua firma. ⛔ **Il verbale di com'era, perché è la parte che insegna:** ⛔ **Misurato il 2026-08-31:** `EffectClass::Idempotent` → `Unrepeatable` nel record di feedback di `run_the_ring` lascia l'intero workspace **verde**, **41 bersagli, 297 passate, 0 fallite, 2 ignorate**, identico alla baseline. **Non era fra le cinque mutazioni dettate** — `M1`…`M5` toccano il record del verdetto, l'apertura del passo, il costo, `passed` e il `payload`, mai la classe — quindi nessuno l'aveva mai eseguita, e il criterio *«ogni mutazione che non ha ucciso niente è dichiarata»* non poteva morderla. ⛔ **E L'ASIMMETRIA È LA FORMA CHE LA REVISIONE DEL COMPITO 4 AVEVA GIÀ TROVATO, in un posto nuovo:** il `Verifiable` del record di verdetto — che `reconcile` **provabilmente non legge mai**, e il codice lo scrive — porta **cinque righe** di giustificazione; l'`Idempotent` del feedback — che `reconcile` **legge**, `RecordKind::Intent => enter(.., resolution_of(body.effect))`, e che decide la riconciliazione di **ogni** passo correttivo che l'anello apre — non ne porta **nessuna**. Il campo che si rifiuta è argomentato, quello a cui si obbedisce no. ⚖️ **IL MERITO, che è la decisione:** l'anello dichiara *«riesegui»* per un passo il cui effetto **nessuno conosce ancora** — la correzione non è scritta — e quel record è l'**unico** intento di quel passo (`E19` rifiuta il secondo), quindi la classe è fissata lì e per sempre. ADR-0007 decide il contrario per esattamente quella situazione: *«Un effetto senza classe dichiarata è trattato come `irripetibile`: davanti a un dubbio non risolvibile il sistema si ferma, non indovina»*. ⚠️ **E non era un difetto di esecuzione:** il piano detta `Idempotent` al Passo 3 del commit 5c — era una decisione del **piano** che nessuno aveva prezzata contro l'ADR. |
| **E56** | ✅ **CORRETTO — DUE COMMENTI DI SORGENTE DICEVANO AL PRESENTE CHE `frozen_bytes.rs` CONGELA *«TRE»* RECORD, E QUESTO COMPITO NE HA FATTO IL QUARTO.** ⛔ **Le case sono state censite col `grep` e guardate in faccia una per una** (#70), non corrette dove il rilievo è stato trovato: `crates/kernel/src/record.rs` — *«It freezes THREE records, which between them pin all EIGHT variant indices of the three enums above»* — e `crates/kernel/tests/record_shape.rs` — *«It freezes THREE records, because the three enums have EIGHT variants between them»*. ⚠️ **Ed entrambe stanno DENTRO un capoverso che si dichiara datato**, il che è precisamente la distinzione della **55ª misura**: *dentro un verbale datato una cifra regge, dentro un'affermazione al presente mente*. Quei due capoversi datano ciò che **dicevano prima**, e poi affermano il conteggio **al presente** — quindi cadono dal lato che mente. ✅ **Rimedio: il richiamo datato, e il conteggio TOLTO invece che riallineato a quattro** — la misura degli otto rossi resta attribuita al 2026-08-10, dove è vera, e *«quanti siano adesso»* rimanda a `the_frozen_records()`, che è la casa unica e che il file stesso già usa da questo compito al posto del letterale `3`. ⚠️ **E il criterio di chiusura del compito non poteva coglierla:** pretende il richiamo datato sui *«commenti di doc che mostravano un record a CINQUE CAMPI»*, che è un'altra popolazione. È la classe **R1** dell'audit, e la disciplina che la coglie è il passo **5** di quel rapporto — *ricontà i conteggi che il tuo rimedio ha reso stantii*. |
| **E57** | ⛔ **BLOCCANTE — LA SONDA `the_dispatch_journals_the_RESOLVED_decision_and_not_a_reference_to_it` NON PUÒ PASSARE, ED È `E45` ALLA SECONDA OCCORRENZA.** `dispatch` scrive con `journal.note(step, &record)`, e `MemoryJournal::note` risponde `Err(JournalError::OutOfOrder)` quando `!self.has_intent(step)` — letto in `crates/simulator/src/journal.rs`, non dedotto. La sonda dettata parte da `MemoryJournal::new()` e **non apre mai** `StepId::new(1)`, quindi `.expect("dispatch")` va in panico. ⛔ **E l'oracolo non si aggiusta aprendo il passo e basta:** la sonda asserisce `entries.len() == 1` e legge `entries[0]`, cioè dichiara di **non** aspettarsi l'intento; con il passo aperto diventano **2** e il record di routing si sposta a `entries[1]`. Vanno riscritti l'aiutante di apertura, il conteggio e l'indice — è il costo che al compito 5 non era la riga di setup ma gli **oracoli**. ⚠️ **Non è una svista di chi ha scritto il piano:** il compito 6 è stato scritto il 2026-08-30, e `E45` è nata **eseguendo** il compito 5 il giorno dopo. È la **domanda 5** di [`CLAUDE.md`](../../../CLAUDE.md) — *il contratto cresce sotto il piano* — e la prova che la disciplina di rileggere un compito contro il codice di **adesso** paga anche quando il piano è di ieri. |
| **E58** | ⛔ **BLOCCANTE — DUE SONDE DETTATE NON COMPILANO: `assert_eq!` SU UN `Result` IL CUI `Ok` NON È `PartialEq`.** `Conforming` è dettato con `#[derive(Debug)]` e nient'altro, mentre `a_data_constraint_with_no_candidate_FAILS_CLOSED` e `an_empty_chain_fails_closed_too` confrontano `resolve(..)` con `Err(..)`. ⛔ **Misurato il 2026-08-31 con `rustc --edition 2024` su una riproduzione minima fuori dal repository**, cancellata nella stessa corsa: `` error[E0369]: binary operation `==` cannot be applied to type `Result<Conforming, GatewayError>` ``, con la nota *«an implementation of `PartialEq` might be missing for `Conforming`»*. ⛔ **E IL RIMEDIO NON È AGGIUNGERE `PartialEq`**, che è la strada che il compilatore stesso suggerisce: è il **precedente del pre-controllo del piano del Traguardo 5** — *«`Admission` non può derivare `Debug` né `PartialEq`, perché `Grant` non li ha e non deve averli, quindi ogni sonda dell'arbitro confronta con `matches!` e `let … else`»* — ed è anche la regola *«no caller, no item»* che **questo stesso compito** invoca due paragrafi più su per rifiutare un getter a `Conforming`. Un derive che esiste per il solo banco è la stessa cosa. |
| **E59** | ⛔ **IL CODICE DETTATO E IL DOC DETTATO DELLO STESSO CAMPO DICONO DUE COSE DIVERSE, E NESSUNA SONDA LI DISTINGUE — sull'ARTEFATTO IRREVERSIBILE.** `resolve` calcola `let evaluated = chain.len() as u32;`, mentre il doc di `RoutingDetail::evaluated` dice *«How many candidates the filter WALKED»*. ⛔ **Sono grandezze diverse perché `resolve` usa `.find()`, che si ferma al primo che combacia:** su una catena di cinque il cui primo conforme è il primo, ne cammina **uno** e il campo scriverebbe **cinque**. ⚠️ **E LO SCENARIO DELLA SONDA È DEGENERE, che è la ragione per cui il difetto non si vedrebbe:** l'unica sonda che legge `evaluated` usa `chain = [REMOTE_DEAR]`, un elemento, dove *«camminati»* e *«lunghezza»* valgono **1** entrambi. È la forma del rilievo ② della revisione del Task 8 del Traguardo 5 — *«`ceiling`, `allocated`, `asked` e `needed` valevano tutti lo stesso numero, quattro grandezze che coincidono e nessuna asserzione che possa distinguerle»*. ⛔ **E non è prosa: il campo entra nel QUINTO RECORD CONGELATO**, che non si rigenera. ⚠️ **Quale delle due letture sia giusta è una decisione**, non un allineamento: ADR-0011 chiede *«la catena di riserva VALUTATA»*, il che tira verso *«camminati»*; ma un conteggio che dipende da dove `.find()` si ferma è meno stabile di uno che dice quanti ne erano stati offerti. **Si decide leggendo l'ADR, e poi codice e doc dicono la stessa cosa** — e la sonda usa una catena di **almeno tre** con l'esito **non** in ultima posizione, o resta cieca comunque. |
| **E60** | ⚠️ **IL COMPITO NON COMPILA FINCHÉ NON ESTENDE IL `match` ESAUSTIVO SU `Detail`, E IL PIANO NON LO DICE — perché quella guardia è nata OGGI.** Il Passo 3 del commit **6b** nomina soltanto *«l'array a mano di `frozen_bytes.rs` … esteso a CINQUE varianti»*, che è `the_frozen_records()`. Dal 2026-08-31 quello stesso file porta anche un `match detail { Detail::Verdict(_) => {} }` — il rimedio di **`E54`** — quindi `Detail::Routing` lo rende `` error[E0004]: non-exhaustive patterns ``. ✅ **Non è un difetto: è la guardia che fa il proprio lavoro**, ed è esattamente il rosso che `E54` esiste per produrre. Va scritto nel compito perché chi esegue **atterri sulla riga giusta** invece di leggerlo come un imprevisto. ⚠️ **Il `match kind` accanto si comporta allo stesso modo** per `RecordKind::Routing`, e quello c'era già. 📌 **È la domanda 5 una seconda volta nello stesso pre-controllo**, e dice qualcosa che vale oltre il caso: un compito va riletto contro il codice di adesso anche quando *chi lo rilegge* è chi ha cambiato quel codice. |
| **E61** | ⚠️ **`Constraint::NoRetention` NON È NOMINATO DA NESSUNA SONDA DETTATA: la sua riga di `satisfied_by` nasce mutante viva.** Contate sulle sonde del Passo 3: `Constraint::LocalOnly` **due** volte, `Constraint::PriceCeiling` **due**, `NoRetention` **zero**. Quindi l'arm `Constraint::NoRetention => !candidate.retains` non è esercitato da niente, e la mutazione che ne toglie la negazione — cioè che fa passare esattamente i candidati che **trattengono i dati** — sopravviverebbe all'intero workspace. ⛔ **E la classe di quel vincolo è `Data`, cioè quella che ADR-0012 fa fallire CHIUSO:** il mutante non è un dettaglio di copertura, è la via per cui una richiesta che non deve lasciare tracce presso il provider ne troverebbe uno che le trattiene. **La più cara delle due direzioni** del vincolo è proprio quella scoperta. 📌 **Rimedio: una sonda in più, non un'asserzione più larga** — il campo `retains` esiste già su `Candidate` e i due candidati di banco lo portano già con valori **opposti** (`LOCAL_CHEAP` a `false`, `REMOTE_DEAR` a `true`), quindi costa una sonda e nessun dato nuovo. |
| **E62** | ⚠️ **REGISTRATA, NON PRESA — «`dispatch` CONSUMA IL GETTONE, QUINDI UNA RISOLUZIONE DISPACCIA UNA VOLTA SOLA» È UN'AFFERMAZIONE CHE NIENTE TIENE.** Il doc dettato di `dispatch` lo promette e cita il precedente di `Process::start`, che consuma un `Grant`; ma il compito costruisce **due** casi `compile_fail` — il candidato non filtrato e il gettone non coniabile — e **nessuno** per il secondo dispaccio. ✅ **E il precedente dice che cosa farne, invece di lasciarlo aperto:** per `Grant` il caso gemello — *«un secondo `start` con lo stesso `Grant` è `E0382`»* — è **misurato e non preso**, ed è una voce aperta del Traguardo 5, perché una riga di catalogo nuova è **spec** (vincolo globale 7). ⛔ **Quindi la via coerente è dichiararlo accanto al codice**, non costruirlo e non tacerlo: oggi il compito fa la terza cosa. ⚠️ **E il tipo lo regge già** — `Conforming` non deriva `Copy` né `Clone`, quindi la proprietà **esiste**; a mancare è chi la dica. |
| **E63** | ⛔ **LA LISTA `Files:` DEL COMPITO 6 NON NOMINA `crates/simulator/tests/dst_campaign.rs`, E IL COMPITO LO RENDE ROSSO.** Quel file porta a riga 412 un `match` **esaustivo** su `RecordKind` — l'oracolo dei passi in dubbio della campagna — e il suo arm `RecordKind::Verdict` è un `panic!` deliberato, non un arm vuoto. `RecordKind::Routing` lo fa diventare `` error[E0004] ``. ⚠️ **E LA VOCE CHE LO DICEVA ESISTE GIÀ: `E50`**, scritta dal compito 5 — *«l'elenco dei file di un compito che tocca `RecordKind` deve nominare anche `crates/simulator/tests/dst_campaign.rs`»* — ed è **nominata nella §6 del compendio proprio per il compito 6**. La lista non è stata aggiornata: la voce c'era, il rimedio no. ⛔ **E la decisione che ne segue NON è meccanica, ed è la ragione per cui quell'arm è un `panic!`:** il suo commento dice che scrivere l'arm vuoto *«farebbe accordare questo oracolo con l'implementazione PER COSTRUZIONE su un caso che non ha mai visto»*, e che *«il giorno in cui lo scenario cresce un sensore, il rosso è una decisione che viene chiesta, non un difetto che viene segnalato»*. Il compito 6 **non** fa crescere un sensore in quello scenario: aggiunge una specie che quello scenario non produce. **Quindi il rosso va chiuso con un secondo `panic!`, non con un arm vuoto** — o l'oracolo smette di essere indipendente. 📌 *Un elenco di file è un'affermazione come le altre, e si legge contro il codice* — gotcha **#65**, la stessa forma di `E154` del Traguardo 5. |
| **E64** | ✅ **CORRETTO — IL DIFETTO DEL GOTCHA #97 ESISTEVA GIÀ NEL PRODOTTO DEL COMPITO 5, E LA PRIMA PASSATA DI REVISIONE NON L'AVEVA VISTO.** Il messaggio del `panic!` di `dst_campaign.rs` portava *«and this ⟨diciotto spazi⟩ oracle»*: una continuazione di riga collassata, **identica** a quella trovata nel rimedio del revisore poche ore prima. ⛔ **Trovato censendo la RADICE invece che l'occorrenza** — passo 3 della disciplina dell'audit — con `grep -rnE` sui letterali che portano una corsa di tre o più spazi: **una sola occorrenza in tutto `crates/`**, e la seconda era già stata chiusa da `488cae0`. **Due letterali su due scritti da script avevano il difetto.** ✅ **E il rimedio ha PROVATO la prescrizione del #97 invece di ripeterla:** riscritto con lo **strumento di modifica** e non con uno script, la continuazione **sopravvive** — due righe da 92 e 98 caratteri — e il censimento torna **vuoto**. `cargo fmt --all --check` pulito, fine-riga immutati. ⚠️ **E la lezione sulla revisione, che è la parte scomoda:** questa occorrenza stava nel perimetro della prima passata e non è stata vista, perché quella passata cercava **mutanti** e **conteggi**, non **letterali**. È la ragione concreta per cui `E53` chiede una passata che torni pulita e non una sola. |
| **E65** | ⛔ **SECONDA PASSATA DI REVISIONE DEL COMPITO 5 — QUATTRO MUTANTI VIVI SULL'INTERO WORKSPACE, E LA RADICE È UNA SOLA.** `run_the_ring` scrive **due** record; il banco teneva per intero quello del **verdetto** e dell'altro — l'**intento del passo correttivo** — soltanto `kind` e `payload`, e nel ramo che **passa** il verdetto era tenuto dal solo `kind`. Misurate una per volta, ciascuna revocata da copia byte-esatta, ciascuna **41 bersagli · 298 passate · 0 fallite**, identico alla baseline: ① `trust: Trust::Untrusted` del record di feedback portato a `Trust::Instruction`; ② `passed` congelato a `false`; ③ `effect: Verifiable` del verdetto portato a `Unrepeatable`; ④ il `reason` del verdetto sostituito. ⛔ **LA PRIMA È LA PIÙ GRAVE, ED È I6:** il payload di quel record è il **dettaglio del sensore**, che ha letto un artefatto `Untrusted`, e ADR-0014 rende l'etichetta **ereditaria** — marcarlo `Instruction` fa attraversare al contenuto esterno il confine delle istruzioni, **dentro il formato durevole**. ⚠️ **E la forma del buco vale più del difetto:** la riga che asserisce **esattamente quel campo** sul record del verdetto stava tredici righe sopra — il banco teneva **uno dei due** record che la stessa funzione scrive. ⛔ **LA SECONDA HA UN DATO CHE LA SPIEGA: `passed: true` NON ESISTEVA IN TUTTO IL WORKSPACE.** Censito: `frozen_bytes.rs`, `reconciliation.rs`, `record_shape.rs` e la sonda negativa portano **tutti** `passed: false`, e l'unico sito che lo **calcola** è `sensor.rs` — quindi la costante era indistinguibile dal calcolo. È *«un controllo si prova in due direzioni»* di [`CLAUDE.md`](../../../CLAUDE.md) su un campo a due valori, sull'**artefatto irreversibile**. ✅ **CHIUSE ALLARGANDO LE DUE SONDE ESISTENTI, non aggiungendone di nuove** — il conteggio resta **298**, e l'invarianza è il dato: mancavano asserzioni, non casi. Rimisurate dopo: ciascuna delle quattro uccide **una** sonda e una sola, cioè sono **strette** (il vicolo cieco dell'audit). ⚖️ **③ e ④ sono PINZATE e non dichiarate**, che è il confine del Task 10 del Traguardo 5 — *un doc che AFFERMA un valore riceve una sonda; si lascia dichiarato solo ciò che una decisione APERTA può ancora cambiare* (#73) — e qui nessuna è aperta: `reconcile` non legge **mai** la classe di un verdetto, **misurato** mutandone l'arm nelle due direzioni (`enter` → due sonde rosse, `leave` → una). |
| **E66** | ⚠️ **LA DIVERGENZA DI DATA ERA CENSITA IN UN FILE SOLO, E TRE AFFERMAZIONI DELLA STESSA SPECIE VIVEVANO IN `crates/`.** La revisione del 2026-08-31 registrò che due celle di [`porta-di-qualita.md`](../../porta-di-qualita.md) datano una misura del compito 5 al **2026-09-01** mentre `git log` data **tutti** i commit al 2026-08-31, e decise: le affermazioni di **STATO** si tolgono — *«per quella domanda l'autorità è il commit»* — i **verbali** di misura restano. ⛔ **Il censimento non è mai arrivato al sorgente:** `grep -rn '2026-09-' crates/` rendeva **sei** righe, di cui **tre** sono esattamente affermazioni di stato — *«IT MOVED AGAIN ON»*, *«THE NAME SAID … UNTIL»*, *«`detail` … arrived on»* — cioè la classe che quella decisione toglie. È la radice **R1** dell'audit e il suo passo **3**: *correggi alla radice, non dove l'hai trovato*, col precedente **AUD-049**. ✅ **Tolte tutte e tre, e non riallineate a 2026-08-31:** al loro posto c'è l'**evento** — *«quando è arrivato l'indice 5»*, *«finché è arrivato il quarto record congelato»* — che è la cura di **AUD-007** applicata a una data, *un elenco invecchia, una regola no*. Le **tre** restanti sono verbali di misura (*«Measured on»*, *«were tried on»*) e **restano**, come la decisione prescrive. ⚠️ **E l'asimmetria è dichiarata dove nasce:** in `record_shape.rs` la clausola gemella data l'indice 4 al **2026-08-10**, che è **giusto** e resta, quindi accanto sta scritto perché una porti la data e l'altra no. ⛔ **Verificato che gli orari NON spiegano la data:** i commit stanno fra le **19:39** e le **21:56** del 2026-08-31, non a cavallo della mezzanotte. |
| **E67** | ⛔ **TERZA PASSATA DI REVISIONE — `spent_millis` ERA UN MUTANTE VIVO, ED È IL GOTCHA #98 UNA SECONDA VOLTA SUL CAMPO ACCANTO.** `run_the_ring` scrive `spent_millis: verdict.spent.get()`; sostituito con la **costante 7**, l'intero workspace resta verde — `41 · 298 · 0`, identico alla baseline. ⛔ **La causa è la stessa che `E65` ② aveva misurato per `passed`, e il censimento la dà esatta:** `ScriptedSensor` restituiva il letterale `Millis::new(7)`, quindi **7 era l'unico costo che attraversasse la conversione in tutto il workspace** — e il quarto record congelato, `reconciliation.rs` e `record_shape.rs` portano `7` come letterale, senza passare dall'anello. Un campo di cui il workspace produce **un valore solo** è una costante per ogni controllo che esiste. ⚠️ **E l'ironia sta nel file stesso:** un secondo valore esisteva già — `WatchfulSensor` con `Millis::new(1)` — ma il suo record non viene mai riletto. ✅ **CHIUSO DANDO A `ScriptedSensor` UN `spent` SCELTO DAL TEST, con valori DIVERSI nelle due sonde** — `3` in quella che passa, `7` in quella che fallisce — che è il gotcha **#48** e la forma che questo stesso banco già usa per la classe del passo correttivo. Rimisurato **nelle due direzioni**: la costante `7` e la costante `3` uccidono **una** sonda ciascuna. ⛔ **NON è un gotcha nuovo:** è il **#98** alla seconda occorrenza, lo stesso giorno e sul campo adiacente — e la riga del #98 lo registra, perché una classe che riproduce due volte in un giorno vale più della sua prima misura. |
| **E68** | ⚠️ **UN'ASSERZIONE DOMINATA, E LA SUA GIUSTIFICAZIONE MISURAVA AL CONTRARIO.** Il commento di `a_passing_sensor_writes_a_verdict_and_opens_nothing` diceva che *«un anello che scrivesse il verdetto sul passo SUCCESSIVO soddisferebbe ogni riga sopra tranne questa»*. ⛔ **Misurato: non ne soddisfa nessuna.** Con `note(next, ..)` la sonda muore sull'`.expect` **alla chiamata**, prima di ogni asserzione, perché `MemoryJournal::note` rifiuta un passo senza intento — `41 · 294 · 4 fallite`, col panico `the ring: OutOfOrder`. È la guardia di `E45` che arriva per prima. ⚠️ **E l'asserzione è davvero dominata:** la voce 0 è l'intento che il test ha scritto da sé e la voce 1 è già pinzata tre righe sopra, quindi nessuna mutazione singola dell'anello può farla fallire per prima. ✅ **LASCIATA, e dichiarata invece che cancellata:** costa una riga e dice la proprietà nella forma in cui un lettore la cerca; ciò che non deve fare è **rivendicare un potere discriminante che non ha**, ed è quello che è stato tolto. |
| **E69** | ⚠️ **UNA SONDA PORTAVA IL NOME DEL PROPRIO FRATELLO.** `a_verdict_leaves_a_closed_step_closed` costruisce **due intenti e nessun esito** — nel suo banco non esiste **nessun** passo chiuso — e asserisce che il dubbio **e le sue risoluzioni** escano come sono entrati. Il passo chiuso lo costruisce l'**altra** sonda, `a_verdict_does_not_put_a_step_in_doubt`, che asserisce l'insieme vuoto. ⛔ **E il rilievo del revisore era di UNA TACCA PIÙ GRANDE del difetto** — diceva *«gli scenari scambiati rispetto ai nomi»*, cioè entrambi sbagliati — mentre la coppia di `Note`, che porta i nomi giusti, mappa scenario→nome **allo stesso modo**: quindi `a_verdict_does_not_put_a_step_in_doubt` è **corretto** e il nome sbagliato è **uno solo**. Gotcha **#65**, nella direzione che costa di più, verificato leggendo la coppia gemella invece che il rapporto. ✅ **Rinominata `a_verdict_leaves_the_doubt_and_its_resolution_exactly_as_it_found_them`**, parola per parola come la gemella di `Note`, e **censita sulle tre case** che portavano il nome vecchio — fra cui `crates/kernel/src/reconcile.rs`, che la cita **per nome** nel doc del proprio arm. |
| **E70** | ⛔ **LA TESTA DI `frozen_bytes.rs` E QUELLA DI `record_v1.map` AFFERMAVANO AL PRESENTE CONTEGGI CHE IL COMPITO 5 AVEVA RESO FALSI — ed è `E56` nelle due case che il suo censimento non ha raggiunto.** `E56` aveva chiuso `record.rs` e `record_shape.rs`; restavano *«THREE RECORDS AND NOT ONE»*, *«EIGHT variants — RecordKind three»*, *«Three records are the FEWEST that cover all eight»*, *«any pair of the three files differs only inside bytes 4, 5 and 6»*, *«ONE CONSTRUCTOR for the three frozen records»*, *«The three frozen records»*, *«this function and the three files are ONE artefact in four pieces»*, *«ONE MAP FOR THREE FILES»* e l'intero capoverso gemello della mappa. ✅ **Misurato e non dedotto:** i record sono **quattro** (21 · 21 · 21 · 27 byte), le varianti **nove** — `RecordKind` 4, `EffectClass` 3, `Trust` 2 — e le coppie che **includono** il verdetto differiscono anche al byte **3** (`85` → `86`) oltre che nella coda, mentre le tre di pari arità differiscono davvero solo a 4, 5 e 6. ⛔ **E il fatto che pesa è DOVE stavano:** nello **stesso commit** quel file ha rinominato due sonde scrivendo la regola *«a name that counts its own subjects is a count like any other»*, e il codice porta già l'`if left.len() != right.len() { continue; }` con la propria spiegazione — **le sonde erano state corrette, il capoverso che le giustifica no**. ✅ **Tolti i numerali e non riallineati**, sostituiti dalla **regola** — *l'insieme copre ogni variante e non può essere più piccolo del più largo degli enum* — e dal rimando a `the_frozen_records()`, il cui **tipo di ritorno** porta il conto che il compilatore controlla. Anche il messaggio dell'`assert!` è stato corretto, **con lo strumento di modifica** perché porta una continuazione di riga (gotcha **#97**). |
| **E71** | ⚠️ **`every_record_kind_survives_the_round_trip_…` CAMMINAVA TRE VARIANTI SU QUATTRO, E LA PRIMA PAROLA DEL NOME È *«EVERY»*.** ⛔ **Non era un buco di copertura, misurato:** il giro di andata-e-ritorno del verdetto è tenuto da `frozen_bytes.rs`, e una collisione d'indice è impossibile al **livello 1** — `` error: duplicate index numbers `` dalla derive. Il difetto è che il nome **afferma** ciò che il ciclo non fa. ✅ **Chiuso aggiungendo `RecordKind::Verdict` al ciclo — un elemento d'array — invece di indebolire il nome**, che è più caro. ⛔ **E il blocco a coppie sotto resta a TRE deliberatamente:** che il quarto sia distinguibile nei byte lo tiene `frozen_bytes.rs` su **ogni** coppia, e asserirlo anche qui sarebbe una seconda casa per una proprietà (§7.4.4). ⚠️ **Rinominate anche `the_three_record_kinds_are_distinguishable_in_the_bytes` e la coda `…_and_the_three_differ_in_the_bytes`**, per la regola che quel file si è dato da sé; ⛔ **`every_effect_class_…_the_three_differ` NON è toccata**, perché `EffectClass` ha davvero tre varianti — ricontate. |
| **E72** | ⚠️ **`#[cbor(default)]` È DICHIARATO PORTANTE IN DUE DOCUMENTI ED È INERTE, MISURATO.** La regola 1 in testa a `frozen_bytes.rs` dice *«A FIELD ADDED TO `RecordV1` MUST BE `Option<..>` WITH `#[cbor(default)]`»* e il doc di `detail` la ripete. ⛔ **Tolta l'annotazione, l'intero workspace resta verde** — `41 · 298 · 0` — **inclusa** la direzione di compatibilità all'indietro: i tre `.cbor` da 21 byte continuano a decodificare a `detail: None`, perché `minicbor` legge già un campo `Option` mancante come `None`. A portare la regola è l'**`Option`**, non l'annotazione. ⚖️ **Il rimedio è correggere la REGOLA e non inventare una sonda per un'annotazione inerte**, ed è la scelta che il revisore aveva esplicitamente lasciato aperta: l'annotazione **resta** — cintura e bretelle non costano nulla su un artefatto che non si rigenera — ma i due doc ora dicono **quale metà difende**, così che un campo futuro che arrivasse senza non venga letto come una regola violata. ⚠️ **Il confronto che rende visibile la differenza:** `#[cbor(with = "minicbor::bytes")]` su `payload` è dichiarato portante **ed è pinzato** da una sonda propria — lì la seconda metà c'è. |
---

## Il pre-controllo del piano — che cosa il disegno dice e il codice smentisce

⛔ **Fatto il 2026-08-30 leggendo il disegno contro il codice di quel giorno**, come il
gotcha **#58** prescrive per un disegno e il **#65** per qualunque documento che prezza
lavoro. Quattro voci, tutte **misurate**.

### P-1 — Il disegno sbagliava la RAGIONE del perimetro, non il perimetro

Il richiamo del 2026-08-29 in §1.2 attribuiva la non-costruzione del trasporto reale alla metà
di **prontezza** della porta `reactor`, che non ha un produttore. **Falso, misurato:** le due
porte sono **a interrogazione** per costruzione — `Ipc::accept` rende `Option<ClientId>` senza
attendere, `Ipc::receive` e `Worker::read_next` rendono `Ok(None)` come risposta ordinaria, e
il doc di `receive` scrive che senza di essa *«the core could not poll this port at all»*.

✅ **Chiuso il 2026-08-30, prima che questo piano esistesse:** la §1.2 porta il richiamo
datato, la **voce 5** del disegno è chiusa, e le due celle della §7.4.6 della spec passano a
**❌ scaglionata**. **Il perimetro non cambia.** Terza occorrenza del **#58**, in una forma
nuova: il documento *aveva* letto codice — `reactor.rs`, vero in ogni parola — e non le due
porte che stava prezzando.

### P-2 — La deduzione della §4.3 è MISURATA, e la risposta ha una terza parte che il disegno non nominava

La §4.3 dichiara: *«che aggiungere una variante lasci i byte congelati identici è una
DEDUZIONE, non una misura. Si misura prima di scrivere»*. **Misurato il 2026-08-30**
aggiungendo `#[n(3)] Routing` a `RecordKind`, e revocato da copia byte-esatta con `git diff` a
zero righe:

| Domanda | Esito misurato |
|---|---|
| i byte congelati restano identici? | ✅ **sì** — `#[cbor(index_only)]` codifica l'indice nudo, quindi `00`/`01`/`02` non si muovono. `frozen_bytes` **6 su 6**, `record_shape` **12 su 12**, nessun `.cbor` toccato |
| è gratis? | ⛔ **no, e il no è una GARANZIA:** non compila finché **due** match esaustivi non decidono — `crates/kernel/src/reconcile.rs:90`, dove la riconciliazione deve dire che cosa la variante nuova significhi per il dubbio, e `crates/kernel/tests/frozen_bytes.rs:224`, l'oracolo stesso |
| il nuovo indice è **pinzato**? | ⛔ **NO, ed è il limite che il banco DICHIARA di avere.** Riprodotto: estendendo l'`arm` senza estendere l'**array a mano** `[RecordKind::Intent, RecordKind::Outcome, RecordKind::Note]`, tutto **compila e resta verde**, e il nuovo indice è tenuto da **nulla** — esattamente la condizione che quel test esiste per impedire sugli altri otto |

📌 **Conseguenza sul compito 6, e non è una rifinitura:** chi aggiunge la variante deve
estendere **anche l'array** e congelare un **quarto record** che la porti, o il traguardo
consegna un indice di filo difeso da niente. Il compilatore **non** lo dice.

### P-3 — La deduzione della §5.1 su `check-docs.sh` è MISURATA, e regge

La §5.1 dichiara *«NON MISURATO: che `check-docs.sh` … non possa verificare che un ✅ nomini un
controllo esistente»*. **Misurato:** lo script ha **undici** passi, e nessuno confronta uno
stato con l'esistenza del controllo che nomina — il più vicino è
`== §8: every V and every Q has a state, and the deferred ones have their trigger ==`, che
verifica l'**innesco** dei rimandati e non il **referente** dei ✅. Il comando che rifà la
misura, invece della cifra:

```bash
grep -nE '^echo "== ' scripts/check-docs.sh
```

⚖️ **La deduzione diventa misura, e la conclusione del disegno non cambia:** `V10`, `V14` e
`Q10` portano ✅ senza controllo e nessun cancello lo dice. Resta **voce 4** del proprietario
se lo script debba imparare a dirlo.

### P-4 — ⛔ Il costo di `E21` è più grande di come la §2.4 lo prezza, e la differenza è nei casi `compile_fail`

La §2.4 dice che `Parameters` guadagna un campo e *«tocca la radice di composizione e il
banco»*, e conta **tre** casi `compile_fail` toccati — i tre della porta `process`.
**Misurato: sono nove, e i tre nominati sono un sottoinsieme.**

`Parameters::new` è **posizionale**, quindi un terzo campo rompe **ogni** sito. Il censimento,
col comando che lo rifà:

```bash
grep -rn "Parameters::new" crates/ --include=*.rs | wc -l     # i siti
grep -rln "Parameters::new" crates/ --include=*.rs | wc -l    # i file
grep -rln "Parameters::new" crates/kernel/tests/compile_fail/*.rs
```

⛔ **Perché i nove casi `compile_fail` sono la parte che fa male, e non i quarantadue siti
ordinari:** un sito ordinario che non compila è un rosso che si legge e si corregge. Un caso
`compile_fail` che smette di fallire **per la ragione che asserisce** e comincia a fallire per
**arità sbagliata** diventa un `mismatch`: l'oracolo che quel caso *è* si spegne, e il rimedio
è rileggere il proprio `.stderr` **uno per uno, mai rigenerarli in blocco** — vincolo 10 della
§11, gotcha **#25**.

⚠️ **Due di essi nominano `Parameters` nel proprio `.stderr`** — `parameters_have_no_default`
e `two_policies_at_once` — quindi sono i due il cui testo atteso può cambiare davvero; gli
altri sette cadrebbero **prima** di arrivare all'errore che asseriscono.

📌 **Non è una ragione per spostare `ArbiterId` fuori da `Parameters`:** ADR-0034 lo colloca lì
e §6.1.3 vieta di generarlo, quindi il costo si **paga e si dichiara**. È il gotcha **#65**
nella direzione che il disegno prezza **meno**: qui il lavoro è più grande di come è scritto.

### P-5 — ⛔ La §11 del compendio prezza il vincolo 15 con un comando che NON riproduce la sua stessa clausola

Fatto scrivendo la Parte B, il 2026-08-30. La riga del **vincolo 15** della §11 di
[`COMPENDIO.md`](../../COMPENDIO.md) — che è la riga che il **compito 3 chiude** — scrive:

> *«`grep -rn minicbor crates/kernel/src/` lo trova **solo** in `record.rs`, cioè sul
> **giornale**»*

**Rilanciato: rende DUE file, non uno.**

```bash
grep -rn minicbor crates/kernel/src/ | cut -d: -f1 | sort -u
```

| File | Che cosa è |
|---|---|
| `crates/kernel/src/record.rs` | il codificatore vero — **tredici** riscontri |
| `crates/kernel/src/ports/process.rs` | **un doc di modulo**, che dichiara la non-costruzione: *«NOT the wire format (§6.10.3: `minicbor`, the port exchanges BYTES …)»* |

⛔ **La sostanza regge, la clausola no, e la differenza è la parte che conta.** L'unico uso
**come codice** è `record.rs`; ma la clausola è scritta **accanto al comando**, e il comando la
smentisce — che è precisamente ciò contro cui il rimedio di **AUD-007** aveva sostituito un
numerale con un comando.

⚠️ **Ed era falsa dalla nascita**, misurato e non dedotto: la riga di `process.rs` arriva da
`ff41eea`, cioè dal **Traguardo 2**, mentre la riga della §11 è stata scritta il 2026-08-27 da
`ebbbdac`. È il gotcha **#70** — *ogni riga che il censimento restituisce si legge intera* —
commesso dentro il rimedio che il gotcha **#68** prescriveva. Il disegno di questo traguardo la
domanda se l'era invece posta: la §1.2 dichiara per esteso che i riscontri di `Permission` e
`degrad` *«sono prosa nei commenti, guardati uno per uno»*.

📌 **NON si corregge adesso, e la ragione è il perimetro:** il **compito 3 onora il vincolo 15**,
quindi quella riga esce comunque dalla tabella *«cosa resta davanti»* alla chiusura. Riscriverla
ora significherebbe scriverla due volte. ⛔ **Entra invece nel criterio di chiusura del compito
3**, perché una riga che sparisce non è una riga corretta.

### P-6 — ⛔ Il disegno non dice DOVE vive lo schema del canale worker, e la mappa dei file non ha un posto per esso

La §3 del disegno decide la **meccanica** — l'inquadratura, il riuso della forma di `record.rs`,
`Frame` che resta opaco — e **nessuna sua riga dice dove il corpo del frame sia definito**. Ma la
condizione **6** della Definizione di «fatto» pretende che *«l'annotazione di stringa di byte sia
**sul canale worker**»*, e un'annotazione ha bisogno di un **campo**, cioè di un tipo.

**Misurato il 2026-08-30:**

```bash
ls crates/kernel/src/            # nessun framing.rs, nessun wire/
```

⛔ **E la mappa dei file di questo piano assegna `crates/kernel/src/wire/mod.rs` al compito 4.**
Il compito 3 arriva prima e ne ha bisogno: come è scritta, la mappa gli lascia il tipo senza una
casa.

✅ **Chiuso dalla decisione D8**, che non inventa un posto ma applica il precedente che il disegno
stesso cita: §6.10.3 dice *«la porta scambia byte, non messaggi tipizzati, **come `journal` dopo
ADR-0036**»*, e dopo ADR-0036 lo schema del giornale vive in `crates/kernel/src/record.rs`, fuori
dalla porta. La mappa è **corretta col richiamo datato**, non riscritta in silenzio.

### P-7 — ⚠️ Il doc di modulo di `ports/process.rs` diventa falso col commit 3b, e il compito deve saperlo prima

`crates/kernel/src/ports/process.rs`, doc di modulo, sotto *«What milestone 2 builds, and what it
does not»*:

> *«NOT the implementation (milestone 6), **NOT the wire format** (§6.10.3: `minicbor`, the port
> exchanges BYTES, every frame declares its own length and decoding checks the bytes consumed),
> and NOT the negative tests of §6.10.5 rows 1-4»*

Delle tre clausole, la terza è **già** falsa dal Task 11 del Traguardo 5 — e infatti quel paragrafo
porta già il proprio **richiamo del 2026-08-21**. La **seconda** diventa falsa col commit **3b**.

📌 **Sta qui e non solo nel compito perché è il passo 5 della disciplina dell'audit** —
*ricontà ciò che il tuo rimedio ha reso stantio* — e perché la casa del paragrafo è un file che
il compito 3 non toccherebbe altrimenti: con questa forma dello schema (D8) il `Frame` **non
cambia**, quindi nulla obbligherebbe a passare di lì. È esattamente il modo in cui una frase
sopravvive al fatto che la smentisce.

### P-8 — ⛔ `Admission` non è un tipo di filo, e metterlo sul filo conierebbe concessioni dai byte

La §6.2 del disegno mette in tabella *«core → gui: **esito dell'ammissione**, a tre vie — in
codice `Admission`»*. **Preso alla lettera non è implementabile**, e le ragioni misurate sono
tre, indipendenti:

| | Misurato in `crates/kernel/src/arbiter/mod.rs` |
|---|---|
| 1 | `Admission::Granted(Grant)`, e `Grant { id: GrantId }` con `GrantId` **privato** — un tipo che non esce dal modulo |
| 2 | `Grant` **non porta nessun derive**, e `Admission` **non deriva `Debug` né `PartialEq`** *deliberatamente*, col perché scritto accanto: darglieli significherebbe darli a `Grant` per comodità del banco |
| 3 | ⛔ **la terza è quella che conta:** un `Grant` **decodificabile** è una concessione **coniata dai byte**. §5.6 tiene che l'unico sito che ne conia una sia `Arbiter::issue`, e `crates/kernel/tests/compile_fail/grant_has_no_constructor.rs` esiste per renderlo impronunciabile da fuori. Sarebbe **AUD-050 rifatto sul gettone più forte del progetto** — *una guardia vale quanto il suo costruttore* |

✅ **E la lettura giusta della §6.2 è PIÙ PICCOLA della falsità apparente** — gotcha **#65**
nella direzione che costa meno. Quella cella **identifica il concetto**: l'*«esito a tre vie»*
di [ADR-0033](../../adr/0033-gpu-della-gui-quota-di-presentazione.md) è ciò che il codice chiama
`Admission`. Non prescrive il tipo che viaggia.

📌 **E il filo non ne ha bisogno, per una ragione che ADR-0033 scrive da sé:** la concessione è
**stato del core** (I1). Alla gui serve il **verdetto**, non il gettone — e un verdetto non è un
gettone. Chiuso dalla **D15**.

### P-9 — ⛔ `ResourceProfile` non è decodificabile, e il difetto è UN campo

La §6.2 dice *«richiesta di concessione ordinaria, **col profilo di risorsa dichiarato**»*.
Misurato in `crates/kernel/src/arbiter/resource.rs`:

```rust
pub struct ResourceProfile {
    pub name: &'static str,   // ⛔ questo
    pub reserved_vram: Mib,
    pub compute_class: ComputeClass,
    pub preemption: Preemption,
}
```

⛔ **Un `&'static str` non si produce da byte in arrivo**, se non **leakando** — che è la via
**A3** dichiarata aperta in `crates/kernel/src/boundary.rs` — e ciò che vi finirebbe è **testo
scelto dalla gui**, cioè contenuto **non fidato** (ADR-0014) dentro un campo di un tipo che
l'arbitro usa per **decidere**. Non è un fastidio di lifetime: è I6.

⛔ **E il secondo corno è chiuso pure lui, misurato:** portare il **nome** e risolverlo non è una
via, perché **non esiste nessun registro nome → profilo**.

```bash
grep -rn "ResourceProfile {" crates/ --include=*.rs
```

rende **due costanti in `crates/daemon/src/main.rs`** e **due aiutanti di banco**. Niente mappa
un nome su un profilo, e costruirne una sarebbe un meccanismo che nessuna riga scritta chiede.

✅ **Chiuso dalla D16, e il rimedio RINFORZA il confine invece di indebolirlo:** la richiesta
porta i **tre campi rappresentabili** e il **nome lo mette il core**. ADR-0005 dice che *«la
riserva è **dichiarata dal richiedente** e verificata dall'arbitro»* — cioè esattamente questa
ripartizione — e nessun testo non fidato raggiunge mai un tipo di decisione.

### P-10 — ⚠️ Il raggio dei derive è più grande di come la §6.8 lo prezza

La §6.8 elenca fra i costi *«`kernel` guadagna due moduli»*. **I moduli sono il costo piccolo.**
I tipi che devono guadagnare i derive del formato **vivono fuori da `wire/`**:

| Tipo | Dove | Chi lo porta sul filo |
|---|---|---|
| `Mib` | `crates/kernel/src/arbiter/resource.rs` | compito 3 (`minicbor`) **e** compito 4 |
| `ComputeClass` | idem | compito 4 |
| `Preemption` | idem | compito 4 |
| `Millis` | `crates/kernel/src/time.rs` | compito 4, dentro `Preemption::After(Millis)` |

⚠️ **E `Mib` li porta DUE VOLTE se il 3bis conferma `bincode`**, perché il compito 3 gli ha già
dato quelli di `minicbor`: un tipo solo, due formati, due insiemi di attributi.

⛔ **QUESTO NON È UN ARGOMENTO PER IL 3BIS, e scriverlo serve proprio a impedire che lo
diventi.** *«Con `minicbor` anche su `ipc` basterebbe un insieme di derive»* è un argomento di
**simmetria e di comodità**, e [ADR-0037](../../adr/0037-criterio-del-pari-per-il-formato-dei-canali.md)
li rifiuta per nome: il criterio là è il **pari**, misurato. Il costo si **registra**, non decide.

### P-11 — ⛔ SBARRA LA PARTE D: tre compiti su quattro devono mettere dati STRUTTURATI E NOSTRI in un record durevole, e `RecordV1` non ha una casella per farlo

Trovato scrivendo la Parte D, il 2026-08-30, e **non è una lacuna di dettaglio**: tocca l'unico
artefatto del progetto che non si corregge.

**I tre compiti, e la riga che li obbliga:**

| Compito | La riga | Che cosa deve entrare nel giornale |
|---|---|---|
| **5** — sensore | §6.4.1: il costo **speso** *«nel verdetto, misurato, **entra nel giornale**»*; e `V14`: l'anello *«apre un passo nuovo»* e vi porta il **dettaglio** | verdetto + dettaglio + costo speso |
| **6** — decisore | §4.3 del disegno: il **record di routing risolto** è giornalato col passo, e *«la disciplina di §4.9 si applica per intero»* | il record di ADR-0011: modello, destinazione, provider, parametri, vincoli, catena di riserva, tentativi, esito |
| **7** — permesso | §6.6: *«un permesso concesso è un **fatto giornalato**»*, e *«quali permessi sono attivi ora»* è una **proiezione del giornale** | la tripla concessa, che la proiezione rilegge |

**E il record non ha dove metterli, misurato in `crates/kernel/src/record.rs`:**

`RecordV1` ha cinque campi, e le **due** caselle di contenuto sono **già assegnate da una
decisione scritta**:

| Campo | Che cosa il suo doc dice di sé |
|---|---|
| `#[n(3)] payload: Vec<u8>` | *«somebody else's and may be anything»* — è il posto del contenuto **esterno**, e *«anything that may have come from outside belongs HERE and nowhere else»* |
| `#[n(4)] reason: String` | *«ours and is always UTF-8»* — testo **nostro**, e *«the asymmetry is the point rather than an accident of typing»* |

⛔ **Nessuna delle due è una casella per dati STRUTTURATI e NOSTRI**, e infilare il CBOR di un
record di routing dentro `payload` **riaprirebbe il difetto che il 2026-08-10 fu chiuso
separando `reason`**: fino a quel giorno `Untrusted::promote` metteva la propria giustificazione
nel `payload` etichettandola `Trust::Untrusted`, e il doc di `reason` scrive che quel record
*«avrebbe portato un'affermazione falsa nell'unico campo il cui mestiere è essere vero»*.
⚠️ **Il precedente vivo dice l'altra metà:** `Arbiter::set_policy` scrive `Trust::Instruction`
col **payload vuoto** e la ragione in `reason`. Byte nostri dentro `payload` **non li scrive
nessuno**.

**E la seconda metà è il `kind`, misurata anch'essa:**

```bash
grep -rn "RecordKind::" crates/kernel/src/    # chi COSTRUISCE e chi FA MATCH
```

✅ Confermato che i match esaustivi sono **due e solo due** — `crates/kernel/src/reconcile.rs`
(righe 91, 92 e 114) e `crates/kernel/tests/frozen_bytes.rs:224`; `arbiter/mod.rs` e
`boundary.rs` **costruiscono**, non decidono. ⚠️ E il doc di `RecordKind` **ha già misurato la
quarta variante**, col nome `Amend`: *«never reaches any bench, because `crate::reconcile`
matches this enum exhaustively and the LIBRARY stops with `E0004`»*.

⛔ **Perché questo FERMA il piano invece di essere una voce d'errata.** Il formato durevole è
l'**unico artefatto irreversibile** del progetto — la quarta proprietà della §3 del
[compendio](../../COMPENDIO.md), *«una finestra che si chiude alla prima riga di codice che
scrive un record»*, e quella riga è stata scritta il 2026-08-10 — e
[ADR-0036](../../adr/0036-evoluzione-del-formato-durevole-del-giornale.md) dice che se i byte
congelati cambiano *«non è un aggiornamento, è un **cambio di formato**»*. Un piano che
dettasse questa scelta la **delegherebbe a un subagente**, ed è la sola specie di decisione che
non si delega.

**Le due forme coerenti, con ciò che è misurato e ciò che è dedotto:**

| | Forma | Stato dell'evidenza |
|---|---|---|
| **α** | **una variante nuova di `RecordKind` per specie** (`Routing`, poi il permesso, poi forse il verdetto), e la struttura codificata **nel `payload`** | ⛔ è la direzione che il **disegno** prende (§4.3) e che **P-2** ha misurato per la variante: byte congelati identici, non compila finché i due match non decidono, indice **non pinzato** finché non nasce un record congelato nuovo. ⚠️ **Ma contraddice il doc di `payload`**, e nessuno ci ha mai messo byte nostri |
| **β** | **un campo facoltativo nuovo su `RecordV1`, a indice libero**, che porta un **enum versionato** del dettaglio strutturato; `RecordKind` **non si tocca** | ✅ la **regola 3 di §4.9.2** lo prevede — *«un campo nuovo è facoltativo e prende un indice nuovo»* — quindi **non è un cambio di formato**; e il Task 10 del Traguardo 3 ha **misurato** che un campo facoltativo a indice libero lascia i byte **identici** finché è `None` (gotcha **#54**). ⚠️ **DEDOTTO e non misurato per QUESTO caso:** che regga con un enum annidato all'indice 5 di `RecordV1` |

⚖️ **Non è presa qui, ed è del proprietario.** La differenza fra le due non è di eleganza: la α
mette byte nostri in una casella che dichiara di essere di qualcun altro e allunga un enum
`index_only` **una volta per specie**; la β lascia `RecordKind` alle tre risposte che dà al
dubbio e concentra l'evoluzione in **un** posto disciplinato. ⛔ **E la β non è la mia da
prendere nemmeno se sembra migliore**, perché il disegno approvato ha già scelto la α alla §4.3:
cambiarla è un **richiamo datato su una sezione approvata**, non un ritocco di piano.

⚠️ **Ciò che NON è in dubbio, e va detto per non far sembrare il buco più grande di quanto sia:**
i compiti **8** (il degrado, che si **ricalcola** e non si scrive) e **9** (la campagna) non
toccano il formato; e il compito 5 può essere scritto **fino al contratto del sensore** — è
l'**anello** che giornala, non il tratto.

#### ✅ P-11 È CHIUSA IL 2026-08-30, E LA MISURA HA DATO UNA TERZA RISPOSTA

⛔ **Il proprietario ha rimandato la scelta ai criteri, e il primo criterio è la correttezza
verificata: la β non era misurata per questo caso, quindi si è misurata prima di scegliere.**
Sonda usa-e-getta in `crates/kernel/tests/`, con una **struttura specchio** di `RecordV1` — così
nessuno dei trentanove siti di costruzione è stato toccato — compilata, letta e **cancellata
nella stessa corsa**. `record.rs` verificato **byte-identico** con `cmp` contro una copia presa
prima, albero pulito, baseline invariata a **37 bersagli, 267 passate, 0 fallite, 2 ignorate**.

| Domanda | Esito misurato |
|---|---|
| il campo facoltativo a indice 5 con un **enum annidato versionato** è additivo? | ✅ **sì** — `None` dà **byte identici** al tipo a cinque campi, `85 00 01 00 43 01 02 03 63 77 68 79` in entrambi. Il `None` in coda viene **troncato**, non scritto |
| e con `Some`? | ✅ da 12 a **18** byte: l'intestazione d'array passa `85` → `86` e in coda arriva `82 00 81 82 07 09` — cioè il dettaglio porta la **propria busta di versione**, la stessa forma di `Record` |
| un record **vecchio** a cinque campi si legge col tipo a sei? | ✅ **sì** |
| ⛔ un record **nuovo** con `Some` si legge col tipo a **cinque**? | ⛔ **SÌ, E QUESTA È LA NOTIZIA:** un build che non conosce il campo decodifica il record **con successo** e perde il dettaglio **in silenzio** |

⛔ **La quarta riga decide, e non era in nessuna delle due opzioni.** Sotto la **α** un build che
non conosce la variante nuova di `RecordKind` **non decodifica affatto** — `RecordError::Malformed`,
e la riconciliazione risponde `SuspendAndAsk`: *si ferma invece di indovinare*, ed è la
direzione che il doc di `RecordKind` dichiara **sicura**. Sotto la **β nuda**, lo stesso build
legge il record, lo crede intero, e **butta la sostanza**. Per un record di **permesso** significa
che la proiezione *«quali permessi sono attivi ora»* risponde con una verità parziale senza che
nulla lo dica: è il **degrado silenzioso** che ADR-0005 e ADR-0019 vietano.

📌 **E dalla misura esce anche il perché la regola 3 non basta da sola:** *«un campo nuovo è
facoltativo»* è pensata per un'**aggiunta** — un campo la cui assenza non cambia che cosa il
record è. Ciò che i compiti 5, 6 e 7 devono scrivere non è un'aggiunta a una specie esistente: è
una **specie nuova**. Portarla in un campo facoltativo usa il meccanismo contro la sua ragione,
e la misura mostra esattamente come.

✅ **La scelta è quindi la COMPOSIZIONE, ed è la D20** — variante nuova di `RecordKind` **e**
dettaglio in un campo facoltativo nuovo: la variante fa **fermare** il lettore vecchio, il campo
tiene i byte **fuori dal `payload`**, che resta *«di qualcun altro»*.

⚠️ **Ciò che NON è misurato, dichiarato:** la **composizione** dei due — un record con `kind`
nuovo **e** `detail: Some` — è **dedotta** dalle due misure, che toccano punti indipendenti della
codifica (l'indice della variante nel campo 0, il campo in coda). Si misura **come primo passo
del compito 6**, prima di scrivere: è la stessa disciplina che questa voce ha appena applicato a
sé stessa.

### P-12 — ⛔ La D20 dice TRE specie, la conclusione di P-2 fu misurata su UNA, e la D21 ha portato il singolare

La **D20** scrive nella propria clausola di costo che il campo si paga una volta sola *«perché
le **tre specie** condividono il campo»*, e la sua intestazione dice che **una specie nuova è
una variante nuova di `RecordKind` più il campo**. Tre specie sono quindi **tre** varianti
nuove. La **D21** dice *«il **quarto** record congelato»* — **uno**.

⛔ **Un record congelato porta UN `kind`**, quindi tre varianti nuove ne pretendono **tre**.

📌 **Da dove viene il singolare, e non è una svista di battitura:** viene da **P-2**, che misurò
aggiungendo **una** variante — `#[n(3)] Routing` — e la cui conclusione è corretta *sotto quella
premessa*: *«chi aggiunge la variante deve … congelare un quarto record che la porti»*. La D20 ha
poi cambiato la premessa da una specie a tre, e **nessuno ha riprezzato P-2 contro la premessa
nuova**. È il gotcha **#31** nella forma che costa di più — una conclusione vera resta scritta
mentre l'ipotesi da cui pendeva si è mossa — e il gotcha **#59** applicato a **due decisioni
dello stesso piano, scritte lo stesso giorno**, nessuna delle quali nomina l'altra.

⛔ **E LO SCARTO NON DIVENTA ROSSO, per la ragione che P-2 ha misurato lei stessa.** L'array di
`crates/kernel/tests/frozen_bytes.rs` è **scritto a mano**:

```rust
for kind in [RecordKind::Intent, RecordKind::Outcome, RecordKind::Note] {
    match kind { RecordKind::Intent | RecordKind::Outcome | RecordKind::Note => {} }
    assert!(kinds.contains(&kind),
        "no frozen record carries {kind:?}: its wire index is held by nothing");
}
```

Il `match` esaustivo ferma la **compilazione** alla prima variante nuova, e questo si vede; ma
estendere l'`arm` **senza** estendere l'array lascia tutto **verde**, ed è il limite che quel
banco **dichiara di avere** — *«extending the arm without extending the array above still
compiles»*. ⚠️ **Quindi con tre varianti e un solo record congelato il traguardo consegna DUE
indici di filo tenuti da nulla**, e nessun cancello lo dice: è la stessa condizione che il Task
10 del Traguardo 3 misurò — *«un record solo fissa tre indici di variante su otto … cinque indici
su otto sarebbero rimasti tenuti da nulla»* — e la cui cura fu congelare **tre** record, *«il
minimo che li copra tutti»*.

✅ **La metà meccanica si chiude qui e NON torna al proprietario**, perché la direzione è già sua:
la **D21** si riscrive come una **regola** invece che come un numerale — *un record congelato
nuovo per **ogni** variante nuova* — che è la cura che questo repository applica ai numeratori
(gotcha **#68**), e che resta vera qualunque sia il conto. ⚖️ È anche la forma che l'audit del
2026-08-27 chiama *«un'esecuzione lasciata a metà da un disegno approvato si completa, non si
ripropone al proprietario»*.

⚖️ **La metà che NON si chiude qui, ed è del proprietario: QUANTE varianti apre la Parte D.**
Vedi la voce aperta in fondo a questo pre-controllo.

⛔ **E NESSUN GOTCHA NUOVO, che è una decisione e non una dimenticanza.** Il **#31** dice già
questo alla lettera — *«una stima di costo prezzata sulla variante sbagliata sopravvive, perché
viene **citata** invece che **rifatta**»* — e sia P-12 sia **P-13** ne sono occorrenze: la
conclusione di P-2 citata sotto una premessa nuova, e il `grep` grezzo citato invece che riletto.
📌 *Un gotcha che non insegna niente diluisce quelli che insegnano*, e la §9 è l'elenco che si
rilegge per primo.

### P-13 — ⛔ Il censimento della D20 è il `grep` GREZZO, e lo scarto cade nelle due direzioni

La **D20** prezza il campo nuovo così: *«rompe i **trentanove** siti di costruzione di `RecordV1`
in **undici** file, **tre** dei quali sono casi `compile_fail`»*. Il numero viene dal comando che
il sorgente stesso nomina — `grep -rn 'RecordV1 {' crates/ --include=*.rs`, citato in
`crates/kernel/src/record.rs` e in `crates/kernel/src/boundary.rs`. **Rilette intere le trentanove
righe** (gotcha **#70**), i siti di costruzione sono **ventisei in nove file**, e i casi
`compile_fail` che ne costruiscono uno sono **due**, non tre.

```bash
grep -rn 'RecordV1 {' crates/ --include=*.rs \
 | grep -v '^[^:]*:[0-9]*: *//' | grep -v 'pub struct RecordV1 {' \
 | grep -v 'impl fmt::Debug for RecordV1 {' | grep -v 'RecordV1 {{' \
 | grep -v '"V1(RecordV1 {' | grep -v -- '-> RecordV1 {'
```

⛔ **Il terzo caso `compile_fail` non esiste:** in `promote_reason_is_not_runtime_text.rs` la riga
che il `grep` restituisce è **dentro un commento** — la riproduzione del reperto di P-1 — e quel
file non costruisce nessun `RecordV1`.

⛔ **Ma le tredici righe escluse NON sono lavoro zero, ed è la direzione che il numerale nasconde
invece di gonfiare** (gotcha **#65**, nelle **due** direzioni sulla stessa cifra):

| Che cos'è | Quante | Che cosa il campo nuovo le fa |
|---|---|---|
| la **definizione** della struttura | 1 | è il sito che il campo aggiunge |
| l'`impl fmt::Debug` e la sua **stringa di formato** | 2 | ⛔ obbligano a una decisione — **P-14** |
| un **tipo di ritorno** (`fn last_record(..) -> RecordV1`) | 1 | niente |
| l'**oracolo** del `Debug` in `tests/record_shape.rs` | 1 | ⛔ verde o rosso a seconda di **P-14** |
| **commenti di doc** che mostrano un record a cinque campi | 6 | ⚠️ diventano **falsi in silenzio**, ed è la specie che la radice **R1** produce |
| la **riga del comando** che il sorgente cita per contarli | 2 | ⚠️ il censimento conta **sé stesso**, come il `grep` delle voci aperte del Traguardo 5 |

📌 **La conseguenza sul compito 6 è che il lavoro non è «ventisei sostituzioni»:** sono ventisei
literal **più** una decisione **più** sei commenti di doc da riscrivere o da datare. ⚠️ **Il
numerale della D20 è TOLTO e non riallineato a ventisei**, perché il comando qui sopra lo rifà e
non marcisce.

### P-14 — ⛔ Il campo nuovo obbliga a una decisione sul `Debug` scritto a mano, e i due `.stderr` si comportano DIVERSAMENTE

**Il `Debug` di `RecordV1` è scritto a mano e stampa cinque campi**, e il suo oracolo è un
`assert_eq!` sulla **stringa esatta** in `crates/kernel/tests/record_shape.rs`:

```
"V1(RecordV1 { kind: Intent, effect: Idempotent, trust: Untrusted, "
"payload: <24 bytes>, reason: \"why this step exists\" })"
```

| Se il `Debug` … | L'oracolo | Che cosa si compra, e che cosa si paga |
|---|---|---|
| **non** stampa `detail` | resta **verde** | ⛔ `RecordV1` guadagna un **secondo campo nascosto** senza che nessuno l'abbia deciso, e la metà *«gli altri quattro campi devono restare leggibili»* — che il banco dichiara essere *«la metà che ci si dimentica»* — smette di valere per il campo nuovo |
| **stampa** `detail` | va **ROSSO** | la stringa attesa si riscrive **a mano**, e il dettaglio strutturato finisce in ogni `{:?}` che raggiunge un log |

⛔ **E non è una scelta di cosmetica: è I6.** Il doc dell'indice 3 dice che *«anything that may
have come from outside belongs HERE and nowhere else in this struct. Putting untrusted content at
any other index would print it in the first `{:?}` that reaches a log»*. Il campo `detail` porta
byte **nostri** per costruzione (D20), quindi stamparlo è legittimo — **ma la garanzia è di
disciplina, non di tipo**, esattamente come per `reason`, per cui **AUD-050** ha misurato che un
literal scritto da qualunque crate ci mette dentro una `String` di esecuzione.

⚠️ **E i due casi `compile_fail` che costruiscono un `RecordV1` NON si comportano allo stesso
modo**, misurato leggendo i loro `.stderr`:

| Caso | L'oracolo cita | Che cosa gli fa il campo nuovo |
|---|---|---|
| `record_without_version` | **la riga 15**, `inner.encode()` — *dentro* il literal | ⛔ `detail: None` la porta a **16**, e il `.stderr` va corretto **a mano**. Il file lo dichiara già di sé: la stessa cosa successe il 2026-08-10 con l'indice 4, *«corrected BY HAND to match. Not regenerated»* |
| `record_without_trust_label` | **la riga 7**, l'apertura del literal | ✅ non si muove — **ma il campo va aggiunto lo stesso**, o l'errore diventa `missing fields «trust» and «detail»` e il caso **smette di essere un caso sull'etichetta di fiducia**. Il file lo scrive già: *«A negative case that fires for a second reason is a case that stops proving the first»* |

📌 **Vincolo 10 della §11 e gotcha #25:** i due `.stderr` si rileggono **uno per uno**, e la corsa
che li produce si lancia **senza** `TRYBUILD=overwrite`. È la **D2** applicata a due file che la
D2 non nominava.

### P-15 — ✅ La composizione è MISURATA, e la deduzione regge — anticipata dal compito 6 al pre-controllo

La **P-11** chiude dichiarando: *«la **composizione** dei due — un record con `kind` nuovo **e**
`detail: Some` — è **dedotta** dalle due misure … si misura come primo passo del compito 6»*.
⛔ **È stata misurata QUI invece che lì**, e la ragione è la stessa che P-11 dà a sé stessa: la
Parte D si scrive **sopra** quella deduzione, e un piano scritto su un'ipotesi non misurata detta
a un subagente ciò che nessuno ha verificato. La §4.3 del disegno lo prescrive con parole proprie
— *«si misura **prima di scrivere**, in due direzioni»*.

**Come, e la disciplina è quella della D7:** banco usa-e-getta in `crates/kernel/tests/` con una
**struttura specchio** di `RecordV1` e dei tre enum `index_only` — così nessuno dei siti di
costruzione è stato toccato — compilato, letto e **cancellato nella stessa corsa**; albero di
lavoro verificato **pulito** con `git status --porcelain` dopo la cancellazione.

⛔ **E lo specchio è stato provato PRIMA di credergli, che è la misura senza la quale le altre non
provano niente:** i tre file congelati veri sono stati **ricostruiti byte per byte** dallo
specchio.

| # | Domanda | Esito misurato il 2026-08-30 |
|---|---|---|
| 1 | lo specchio **è** il record? | ✅ i tre `.cbor` ricostruiti identici — `820081850001014666726f7a656e6666726f7a656e` e i due fratelli |
| 2 | variante nuova **e** campo nuovo insieme muovono i byte esistenti? | ✅ **no** — `kind` vecchio e `detail: None` danno i **21 byte identici** su tutti e tre |
| 3 | il **campo da solo** ferma il lettore vecchio? | ⛔ **no** — decodifica `Ok`, `kind=Intent`, e il dettaglio **sparisce in silenzio**. È la quarta riga di P-11, riprodotta sul record vero |
| 4 | la **variante** ferma il lettore vecchio? | ✅ **sì** — `unknown enum variant 3 at position 4`, e `Record::decode` mappa **ogni** errore di `minicbor` su `RecordError::Malformed` (`record.rs:423`), che `reconcile.rs:119` risolve in `SuspendAndAsk` |
| 5 | la **composizione intera**? | ✅ 40 byte, il lettore vecchio si ferma con lo stesso errore, e il tipo nuovo rilegge `kind` **e** dettaglio |

📌 **E la misura porta un fatto che la deduzione non aveva:** l'intestazione d'array passa da `85`
a `86` **solo** quando `detail` è `Some`. Quindi, una volta che il campo esiste sul tipo vero, i
**tre** record congelati di oggi tengono **da soli** la metà additiva — vengono ricodificati dal
tipo nuovo con `detail: None` e devono restare a 21 byte — e il record congelato nuovo non deve
portarne una seconda copia: gli tocca **l'altra** metà, che è l'indice della variante **più** la
posizione del campo. È la ragione per cui la **D21** pretende `Some` e non `None`, ora misurata
invece che argomentata.

### P-16 — ⛔ La terza proprietà di §5.7 pretende un meccanismo che NON ESISTE, e la mappa dei file non ha un posto per esso

Trovato scrivendo la Parte E, il 2026-08-30. **È P-6 nella stessa forma**, un compito più
avanti: il disegno dice **che cosa** si inietta e non dice **chi** risponde.

La §5.7 della spec chiede: *«la GUI muore tenendo una concessione discrezionale → **la somma
torna alla linea di base**»*, ed è la voce **`E152`**, il cui chiusore dichiarato è questo
traguardo. [ADR-0033](../../adr/0033-gpu-della-gui-quota-di-presentazione.md) nomina il
meccanismo: *«se la GUI muore tenendo una concessione ordinaria, il core se ne accorge dalla
**disconnessione IPC** e riconcilia»*.

⛔ **Nessuno riconcilia, misurato:** non esiste nulla che leghi una concessione a un client, e
`Arbiter` non conosce i client — la sua API pubblica è `set_policy`, `allocated`, `admit`,
`queued`, `promote`, `revoking`, `release`.

```bash
grep -rniE "ClientId" crates/kernel/src/ --include=*.rs
```

⚠️ **E la mappa dei file dà al compito 9 UN SOLO file** — `crates/simulator/src/ipc.rs`, *«la
finta gui guidata dal seme»* — cioè il **generatore del guasto** e non ciò che vi risponde. Un
compito scritto contro quella mappa costruirebbe l'iniezione e troverebbe la proprietà **falsa**,
oppure — peggio — la terrebbe **dentro il banco**, che è una proprietà del sistema tenuta dal
test che dovrebbe misurarla.

✅ **Chiuso dalla D27:** nasce `crates/kernel/src/client.rs` con `ClientGrants` e `on_disconnect`.

⛔ **E la cosa da NON confondere, perché qualcuno lo farà: questo non è il ciclo di
orchestrazione.** `E50` ed `E51`/`E100` aspettano *«chi costruirà il primo ciclo di
orchestrazione»*, cioè chi decide **quando** chiamare `promote` rispetto ad `admit`.
`on_disconnect` non decide niente del genere: risponde a **un** evento con **un** rilascio. ⚖️ Le
due voci restano **aperte**, e il doc del modulo nuovo lo dichiara — altrimenti il primo che le
rilegge le crede chiuse da un file che si chiama come loro.


### ⚖️ La voce che questo pre-controllo apre per il proprietario

⛔ **QUANTE varianti di `RecordKind` apre la Parte D, e quindi quanti record congelati nascono.**
La **D20** dice *«tre specie»*; ciascuna riga che le impone è scritta, ma **quale delle tre
pretenda davvero una variante propria non lo è**:

| Specie | La riga che la impone | Serve una variante propria? |
|---|---|---|
| **record di routing** (compito 6) | ADR-0011: giornalato **col passo** | ⛔ **sì, e sembra forzato:** non apre né chiude un dubbio, quindi è la forma del `Note`; ma metterlo *dentro* `Note` fa perdere la sostanza in silenzio a un lettore vecchio — la misura 3 di **P-15** |
| **permesso concesso** (compito 7) | §6.6: *«un fatto giornalato»*, e la proiezione lo **rilegge** | ⛔ **sì, per la stessa ragione**, e in più la proiezione deve **trovarlo** fra gli altri record |
| **verdetto di sensore** (compito 5) | §6.4.1: il costo speso *«entra nel giornale»*; `V14`: *«un passo nuovo, giornalato»* | ⚠️ **NON forzato:** *«un passo nuovo»* sono un `Intent` e un `Outcome`, che esistono già. Se il verdetto viaggi come record proprio o dentro l'esito del passo nuovo **non è scritto da nessuna parte** |

⛔ **Perché è del proprietario e non mia:** il formato durevole è l'**unico artefatto
irreversibile** del progetto, ADR-0036 dice che se i byte congelati cambiano *«non è un
aggiornamento, è un cambio di formato»*, e P-11 si è fermata **davanti a questa stessa porta** per
una domanda più piccola. ⚖️ **E la differenza fra due e tre varianti non è di stile:** ogni
variante è un indice di filo che **non si ritira mai** (regola 4 di §4.9.2) e un record congelato
in più che **non si rigenera**.

⚠️ **Ciò che NON dipende dalla risposta, e si scrive comunque:** il **compito 8** non tocca il
formato (P-11 lo dichiara), la **forma** del campo `detail` è la D20 in ogni caso, il
**contratto** del sensore del compito 5 è indipendente, e le decisioni **P-13** e **P-14** si
pagano **una volta sola** qualunque sia il conto delle varianti.

---

#### ✅ LA VOCE È CHIUSA IL 2026-08-30 — TRE VARIANTI, E A DECIDERE SONO STATI DUE CRITERI SU CINQUE

⛔ **Il proprietario ha rimandato ai criteri di `decision-principles`, con l'accettazione
condizionata che quella skill definisce:** *«vale finché la condizione regge — se eseguendo
scopri che la strada raccomandata li viola, l'accettazione decade»*. I criteri sono stati
applicati **in ordine di precedenza**, e la risposta non è arrivata dal primo.

| | Criterio | Che cosa dice **su questa** scelta |
|---|---|---|
| 1 | **correttezza verificata** | ⛔ **decisivo contro la variante generica.** La sicurezza della **A** è **misurata** — P-15, riga 4: il lettore vecchio si ferma con `unknown enum variant 3 at position 4`. Quella della variante generica poggia su una **deduzione** che nessuno ha misurato: che una *specie* sconosciuta **dentro** il `detail` faccia fallire la decodifica allo stesso modo. Stesso meccanismo, sì — ma *«plausibile»* non è *«misurato»*, ed è la prima cosa che questo criterio rifiuta |
| 2 | **coerenza** | ⛔ **decisivo, e il progetto ha GIÀ RISPOSTO a questa domanda dentro QUESTO enum.** Al Task 7 del Traguardo 3 la promozione *«non è un secondo intento, è una **TERZA COSA**»*, e la risposta fu **una variante nuova di `RecordKind`** — `Note` — non un discriminante nascosto in un campo. Una specie nella busta e una dentro il `detail` sarebbero **due modi** di fare la stessa cosa, che è ciò che questo criterio vieta per nome |
| 3 | **zero debito futuro** | tre indici che non si ritirano mai sono un costo **dichiarato e deliberato** (regola 4 di §4.9.2, ADR-0036), non debito. La variante generica invece porta debito **silenzioso**: una specie aggiunta domani **non costringe nessun lettore a passarle davanti**, perché il `match` esaustivo su `RecordKind` non cresce più |
| 4 | **stato dell'arte** | non discrimina: il comportamento su cui poggia tutto è di `minicbor` 2.3.0, ed è stato **misurato oggi** |
| 5 | **proporzione** | ⚠️ **è il criterio che ha quasi salvato lo scaglionamento del verdetto**, e la ragione per cui non lo salva sta sotto. Nessuna delle tre varianti è speculativa: ciascuna ha una **riga scritta** che la impone, e nessuna serve *«un caso d'uso che nessuno ha chiesto»* |

⛔ **La terza riga era quella che il pre-controllo dava per NON forzata, e a forzarla è la §6.4.2
della spec**, letta di nuovo invece che ricordata: *«l'anello raccoglie il verdetto, apre un passo
nuovo (V14) **e vi porta il dettaglio**»*. Il dettaglio deve dunque avere una casa **in questo
traguardo**, e le case possibili sono due e cadono entrambe:

| Via | Perché cade |
|---|---|
| il dettaglio in `reason` | `reason` è *«ours and is always UTF-8»*, e il verdetto è **strutturato** — `(verdetto, dettaglio, costo speso)`, tre componenti. E per un sensore **inferenziale** il dettaglio è uscita di modello, cioè `payload` con `Trust::Untrusted`, non `reason` |
| il dettaglio nell'`Intent` del passo nuovo | il dubbio si risolverebbe bene, **ma il verdetto sparirebbe in silenzio** per un lettore vecchio — misura **3** di P-15. È la stessa perdita che rende necessaria la variante per il permesso, e la stessa che ADR-0005 e ADR-0019 chiamano **degrado silenzioso** |

⚠️ **E QUESTA È UNA LETTURA, NON UNA MISURA, e va detto invece che nascosto dietro le altre
quattro righe che lo sono.** Che *«vi porta il dettaglio»* obblighi a un record proprio dipende da
come si legge quella frase della §6.4.2. ⛔ **Se il proprietario la legge altrimenti, la terza
variante decade e torna la via dello scaglionamento** — `V14` e `Q10` andrebbero allora riletti
per verificare che si chiudano davvero senza il dettaglio strutturato, e il *costo speso* di
§6.4.1 resterebbe senza casa con un innesco dichiarato. **Le prime due varianti non dipendono da
questa lettura.**

📌 **E il fatto che le tre siano indistinguibili alla RICONCILIAZIONE non è un difetto, che è
l'obiezione da rispondere prima che qualcuno la faccia.** Tutte e tre sono *«né apre né chiude un
dubbio»*, quindi in `crates/kernel/src/reconcile.rs` prendono **tre arm vuoti** come `Note`. Ma il
dubbio riguarda **gli effetti**, non i fatti registrati **su** un passo: a distinguerle sono i
**loro** consumatori — la proiezione dei permessi del compito 7 e la contabilità del compito 6 —
e per quelli la discriminazione a livello 1 è ciò che evita di decodificare il `detail` di **ogni**
record per sapere se interessa.

---

## Le decisioni prese da questo piano

⛔ **Sono decisioni del piano, non del disegno, e chi esegue può ribaltarle** portando la
misura che le smentisce — è ciò per cui esiste l'errata.

| | Decisione | Perché |
|---|---|---|
| **D1** | ⛔ **Il compito 1 si consegna in TRE commit, ciascuno `GATE GREEN`** — l'identità, poi `release`, poi la porta `process` | il vincolo globale 8 vuole la porta verde a ogni commit, e le tre parti hanno raggi diversi: la prima tocca quarantadue siti, la terza nove `.stderr`. Un commit solo renderebbe irriconoscibile quale dei tre ha rotto cosa. ⚠️ **Resta UN compito e UN dispaccio:** il disegno dimostra in §2.2 che `E30` senza `E21` rende **rossa** una sonda esistente, quindi separarli in due compiti consegnerebbe un rosso |
| **D2** | i tre `.stderr` della porta `process` e i nove di `Parameters` si rileggono **uno per uno**, e la corsa che li produce si lancia **senza** `TRYBUILD=overwrite` | vincolo 10 della §11 e gotcha **#25**: un `.stderr` rigenerato in blocco è un oracolo che si è riscritto da solo per tornare verde |
| **D3** | ⛔ **`ArbiterId` NON è `pub`-costruibile da un letterale di tupla:** campo privato più `ArbiterId::new(u64)` pubblico | è un **parametro consegnato** (ADR-0034), quindi `daemon` deve poterlo costruire; ma la forma `ArbiterId(0)` da qualunque crate rifarebbe il difetto che **AUD-050** ha misurato su `RecordV1` — una guardia vale quanto il suo costruttore |
| **D4** | l'ordine dei compiti è **quello della §1.4 del disegno**, e il **3bis** resta prima del **4** | scrivere lo schema `ipc` in `bincode` **è** la decisione C-1 presa per omissione (§3.5). Invertirli la prende senza accorgersene |
| **D5** | ogni compito **rimisura la propria baseline** con `cargo test --locked --workspace --no-fail-fast` e non cita quella scritta qui | una baseline citata invecchia a ogni compito — gotcha **#31**. Quella di partenza sta scritta **una volta sola**, qui sotto |
| **D6** | ⛔ **il compito 6 congela un QUARTO record**, non solo una variante | P-2: senza, il nuovo indice di filo è tenuto da nulla, e il compilatore non lo dice. ⚠️ **RICHIAMO DEL 2026-08-30: questa riga non diceva CHE COSA il quarto record debba portare, e la chiusura di P-11 ha misurato che con `detail: None` non pinzerebbe niente. La precisa la D21**, che la sostituisce nel merito — la riga resta perché il *quando* (il compito 6) non è cambiato |
| **D7** | le mutazioni si provano **una alla volta**, si compila in un passo **separato** dall'eseguire, e si revoca **ripristinando da una copia presa prima** | gotcha **#48**, la trappola più frequente del progetto: una revoca che deve *cercare* può fallire e lasciare il file mutato — successo al Task 8 del Traguardo 5, sette misure buttate |
| **D8** | ⛔ **lo schema del canale worker vive in `crates/kernel/src/wire/worker.rs`, e `wire/mod.rs` nasce al compito 3 — non al 4** | P-6: il disegno non lo colloca, ma cita il precedente che risponde. §6.10.3 dice *«la porta scambia byte … **come `journal` dopo ADR-0036**»*, e lì lo schema vive in `record.rs`, **fuori dalla porta**. Mettere il corpo dentro `ports/process.rs` rifarebbe la mescolanza che ADR-0036 ha tolto al giornale. ⚠️ **Non è un modo nuovo:** è quello che il progetto usa già |
| **D9** | ⛔ **il corpo del canale worker porta UNA direzione sola — worker → core — e DUE varianti**, `Fragment(Vec<u8>)` e `VramPeak(Mib)` | sono le **sole** che qualcosa di scritto impone: §6.10.4 misura l'annotazione **su un frammento audio da 4096 B** e dichiara che *«il campo che questo canale fa entrare nel giornale è il **picco di VRAM** di §5.2.2: arriva dal worker»*. ⛔ **La direzione core → worker non è imposta da nessuna riga, quindi NON si costruisce** — è la stessa postura della §6.1 del disegno (*«il meccanismo è dovuto per iscritto, il vocabolario no»*), e la non-costruzione porta il **proprio innesco**, come la condizione **9** pretende |
| **D10** | ⛔ **l'inquadratura è di QUATTRO byte, big-endian**, decisa qui **una volta sola** | §6.3 lo prescrive alla lettera: *«la larghezza non si decide qui … si decide una volta sola, al compito 3»*. **Big-endian perché il criterio è il PARI** (ADR-0037): `DataView.getUint32(0)` in TypeScript e `struct.unpack(">I", …)` in Python sono le forme **senza bandiera**, e un ordine che il pari deve ricordarsi di girare è un difetto che compila. **Quattro e non due** perché un frammento di flusso non è limitato a 64 KiB; **non otto** perché spenderebbe quattro byte a frame su una portata che nessun canale raggiunge |
| **D11** | `WireError` nasce col commit 3a con le varianti che **`framing` produce**, e guadagna `Malformed` al **3b**, quando il suo produttore esiste | è la regola *«no caller, no item»* che `ProcessError` porta già scritta. Dichiarare al 3a una variante che nessuno produce sarebbe una promessa tenuta da niente — e il commit 3a deve essere `GATE GREEN` da solo (vincolo globale 8). ⚠️ **Quante siano lo dice il compito**, non questa cella: una cifra qui invecchierebbe al primo commit che ne aggiunge una, ed è il gotcha **#31** |
| **D12** | ⛔ **il compito 3bis MISURA e si ferma PRIMA di decidere**, se la misura chiede un cambio di formato | §6.1.1 è **spec**: riaprirla è del proprietario, vincolo globale 7. ⚠️ **La simmetria fra i due casi è solo apparente:** se la misura dice *«`bincode` è ancora l'unica via»*, il compito **registra** e prosegue, perché non tocca nessuna sezione; se dice *«esiste un'alternativa mantenuta col lettore del pari»*, si ferma. Precedenti: **AUD-004**, **AUD-036**, **AUD-044**, tutti fermatisi prima di decidere |
| **D13** | le **fonti** di C-1 vanno in [`riferimenti.md`](../../riferimenti.md), e **non è la convenzione nuova di `E146`** | `E146` riguarda le **misure interne**, che dal Traguardo 5 vivono in [`porta-di-qualita.md`](../../porta-di-qualita.md) accanto al controllo che difendono, e la §7.4 del disegno conferma che la chiusura non tocca quel file. Un **advisory** e lo stato di manutenzione di una crate sono l'altra cosa: la §12 del compendio chiama `riferimenti.md` *«la provenienza di ciò che non abbiamo dedotto noi, con le date»*. ⛔ Senza questa riga le due regole si leggono come un conflitto, e chi esegue ne sceglierebbe una a caso |
| **D14** | ⛔ **il corpo di `ipc` è UN enum solo, `IpcMessage`, con due varianti — una per direzione — e la direzione è DOCUMENTATA, non tipizzata** | la §6.7 dice che l'enumerazione *«la esercitano i DUE messaggi, non uno: con un tipo solo il discriminante non sarebbe provato»*. **Due** enum da una variante ciascuno lascerebbero **entrambi** i discriminanti non provati, cioè il difetto in doppia copia. ⚠️ **E tipizzare la direzione non compra niente ALLA PORTA:** `send` prende `&[u8]` e `receive` rende `Vec<u8>` — il confine non vede nessun tipo, quindi la garanzia esisterebbe solo fra siti del kernel che **oggi non esistono**. Il costo si dichiara accanto al tipo |
| **D15** | ⛔ **il core → gui porta un verdetto a TRE VIE SENZA IL GETTONE** — `Granted` è una variante **unitaria**, `Queued` pure, `Refused` porta i **due numeri** | P-8. ⚠️ **E l'asimmetria fra le tre non è arbitraria:** i due numeri di `Refused` sono l'unica cosa che il filo deve trasportare perché `design/02` vuole *«perché non entra, e l'alternativa praticabile»* e ADR-0020 vieta al kernel di suggerirla — *l'interfaccia costruisce l'alternativa, il kernel le passa il materiale*, quindi la gui **è** il consumatore scritto di quei due numeri. `Queued` invece resta unitaria: `TicketId::get()` è portante *«per un chiamante che ha accodato DUE richieste»*, e la gui ne ha una. Guadagnerà il biglietto col secondo consumatore, non prima |
| **D16** | ⛔ **la richiesta porta i TRE campi rappresentabili del profilo — `reserved_vram`, `compute_class`, `preemption` — e il NOME lo mette il core** | P-9, e non è un ripiego: ADR-0005 dice che *«la riserva è **dichiarata dal richiedente** e verificata dall'arbitro»*, cioè questa ripartizione esatta; e il nome è la parte che, venendo da fuori, sarebbe **testo non fidato dentro un tipo di decisione** (ADR-0014, I6). ⚠️ **Il costo dichiarato:** il core sceglie **un** profilo per la gui, quindi la gui non può chiederne uno arbitrario — ed è ciò che ADR-0033 descrive, un consumatore solo, il viewer 3D oltre la quota |
| **D17** | i derive del formato si aggiungono **dove i tipi vivono**, mai in tipi specchio dentro `wire/` | un tipo specchio è **una seconda definizione dello schema**, ed è la cosa che ADR-0037 rifiuta per il decodificatore del pari — con la differenza che lì sbaglia in silenzio *fuori*, qui sbaglierebbe in silenzio *dentro*: due definizioni si allineano finché qualcuno se ne ricorda, e nulla diventa rosso quando smette. ⚠️ Il prezzo è P-10, ed è dichiarato invece che evitato |
| **D18** | ⛔ **il compito 4 NON dà per scontata l'API del formato** | verificato nel repository: le due chiamate di `bincode` — `encode_to_vec(v, config::standard())` e `decode_from_slice(&b, config) -> (T, usize)` — sono esercitate da `crates/kernel/tests/dependencies_usable.rs`, che le prova **girando**; **il derive no**, nessuna riga del workspace lo usa. Si verifica con una **sonda usa-e-getta** compilata e cancellata nella stessa corsa, o leggendo la sorgente vendorizzata. È il precedente del Task 8 del Traguardo 3, dove il piano **rifiutò di dettare l'API di `redb`**: dettarla a memoria produce codice *plausibile e falso* |
| **D19** | ⚠️ **il compito 4 non chiude NESSUNA riga di catalogo, e va scritto** | misurato: nessuna riga di §7.4.1 o §7.4.2 nomina §6.1 — `awk '/^#### 7\.4\.1/{f=1} /^#### 7\.4\.3/{f=0} f' <spec> \| grep '§6\.1'` non rende niente. Ciò che il compito 4 produce è il **meccanismo** che rende chiudibile `E152` al compito **9**, e il gettone `Q13` è del compito **6**. ⛔ **Un compito che non muove un numeratore è quello su cui si è più tentati di scrivere che l'ha mosso**, ed è la specie di affermazione che la radice **R1** produce |
| **D20** | ⛔ **UNA SPECIE NUOVA DI RECORD È UNA VARIANTE NUOVA DI `RecordKind` _PIÙ_ UN CAMPO FACOLTATIVO NUOVO PER IL SUO DETTAGLIO** — mai l'una senza l'altro, e mai byte nostri nel `payload` | è la chiusura di **P-11**, misurata. **La variante** serve perché un lettore che non la conosce **non decodifica** e la riconciliazione risponde `SuspendAndAsk` — si ferma invece di indovinare; **il campo** serve perché il `payload` è *«di qualcun altro»* per decisione scritta, e infilarci il nostro CBOR riaprirebbe il difetto chiuso il 2026-08-10 separando `reason`. ⛔ **E il campo da solo non basta, misurato:** un record con `Some` si legge **anche** col tipo che non conosce il campo, che perde la sostanza **in silenzio**. ⚠️ **Il costo, dichiarato:** un campo nuovo rompe **ogni** sito di costruzione di `RecordV1`, e i casi `compile_fail` che ne costruiscono uno hanno i `.stderr` da rileggere uno per uno (vincolo 10 della §11) — è **P-4 in un'altra casa**, e si paga **una volta sola** perché le tre specie condividono il campo. ⛔ **RICHIAMO DEL 2026-08-30: qui stavano *«trentanove siti in undici file, tre dei quali `compile_fail`»*, ed erano il `grep` GREZZO.** Riletto intero è **ventisei in nove**, i `compile_fail` sono **due**, e le tredici righe escluse non sono lavoro zero — **P-13**. **Le cifre sono TOLTE e non riallineate:** il comando che le rifà vive in P-13, in una casa sola |
| **D21** | ⛔ **NASCE UN RECORD CONGELATO NUOVO PER OGNI VARIANTE NUOVA DI `RecordKind`, e ciascuno porta ENTRAMBE le cose insieme** — il proprio `kind` nuovo **e** `detail: Some` | un record congelato con `detail: None` non pinza **niente** dell'indice nuovo, perché un `None` in coda **non viene scritto**: misurato. È la terza riga di **P-2** — *«il nuovo indice non è pinzato»* — che vale **due volte** qui. ⛔ **RICHIAMO DEL 2026-08-30: questa riga diceva *«il QUARTO record congelato»*, al singolare, e la D20 accanto dichiara TRE specie.** Il singolare veniva da **P-2**, che misurò su **una** variante sola; con tre e un record solo il traguardo consegna **due** indici di filo tenuti da nulla, e **nessun cancello lo dice** — l'array di `frozen_bytes.rs` è scritto a mano e il banco dichiara di sé quel limite. **P-12**. 📌 **Regola e non numerale**, perché resta vera qualunque sia il conto (gotcha **#68**). ✅ **E la metà additiva NON è affar loro, misurato in P-15:** i **tre** record di oggi la tengono da soli — ricodificati dal tipo nuovo con `detail: None` devono restare a 21 byte — quindi al record nuovo tocca **l'altra** metà. ⚠️ Sostituisce la **D6**, che diceva *«un quarto record»* senza dire **che cosa** dovesse portare |

| **D22** | ⛔ **LA PARTE D APRE TRE VARIANTI DI `RecordKind` — `Routing`, `Permission`, `Verdict` — E CON ESSE NASCONO TRE RECORD CONGELATI**, il quarto, il quinto e il sesto | è la chiusura della voce che **P-12** apre, decisa dal proprietario **rimandando ai criteri** e presa applicandoli in ordine. **Due** hanno deciso: la **correttezza verificata**, perché la sicurezza di questa forma è quella che **P-15 ha misurato** mentre l'alternativa poggia su una deduzione; e la **coerenza**, perché il progetto ha già risposto a questa domanda **dentro questo enum** — `Note` nacque variante propria al Traguardo 3, non discriminante nascosto in un campo. ⚠️ **La terza variante dipende da una LETTURA e non da una misura**, dichiarata nel verbale: §6.4.2 dice che l'anello *«apre un passo nuovo e **vi porta il dettaglio**»*. Il verbale, i cinque criteri e le due vie scartate stanno in fondo al pre-controllo |
| **D23** | ⛔ **le tre varianti prendono TRE ARM VUOTI in `reconcile.rs`, come `Note`**, e la frase che lo giustifica si scrive accanto | il dubbio di ADR-0007 riguarda **gli effetti** — un passo con intento e senza esito — non i **fatti registrati su** un passo. Nessuna delle tre apre o chiude un dubbio. ⛔ **E l'arm vuoto NON è una scorciatoia:** il doc di `Note` argomenta già il proprio, e le due altre risposte *«furono MISURATE prima»*. Chi scrive i tre arm ripete quella misura per la propria variante invece di ereditarne la conclusione — è il **#65** applicato a un precedente |

| **D24** | ⛔ **il campo `detail` lo paga il COMPITO 5, e i compiti 6 e 7 ereditano** | il costo che **P-13** e **P-14** misurano — i ventisei literal, la decisione sul `Debug`, i due `.stderr` — è il costo del **campo**, non della **specie**. Pagarlo tre volte sarebbe impossibile (il campo si aggiunge una volta) e dividerlo lo renderebbe irriconoscibile: il compito 5 è il primo che ne ha bisogno, quindi è il suo. ⚠️ **Conseguenza dichiarata:** il commit 5b è il più largo del traguardo, e tocca l'artefatto irreversibile |
| **D25** | ⛔ **il `Debug` scritto a mano STAMPA `detail`, e l'oracolo di `record_shape.rs` va rosso e si riscrive a mano** | è la scelta che **P-14** dichiara non presa da nessuna riga. Il campo porta byte **nostri** per costruzione (D20), quindi stamparlo non apre nessuna strada di §A3; **non** stamparlo darebbe a `RecordV1` un **secondo campo nascosto** che nessuno ha deciso di nascondere, contro la metà che il banco chiama *«quella che ci si dimentica»* — *«a `Debug` that hid everything would … leave a failed `assert_eq!` on a record saying nothing at all»*. ⚠️ **Il costo si scrive accanto al tipo:** la garanzia che `detail` sia sempre nostro è di **disciplina** e non di tipo, perché `RecordV1` è `pub` coi campi `pub` — **AUD-050**, registrata e non presa |
| **D26** | ⛔ **`detail` è un TIPO — `Option<Detail>`, un enum a indici espliciti — e non byte opachi** | ADR-0036 regola 6 vuole la **codifica in `kernel`**; byte opachi pretenderebbero una **seconda** decodifica che nessuno può fare senza sapere il `kind` fuori banda, cioè il problema del `payload` spostato in una scatola nuova. ✅ **E la sicurezza è la stessa misurata in P-15:** una variante sconosciuta **non decodifica**, `Record::decode` la mappa su `Malformed` e la riconciliazione risponde `SuspendAndAsk`. ⚠️ **Il costo, dichiarato:** `kind` e specie del `detail` sono **due verità indipendenti** — la forma di `E25` in una casa nuova — e a tenerle in passo è **una funzione di costruzione per specie**, non il compilatore |

| **D27** | ⛔ **la riconciliazione alla disconnessione nasce in `crates/kernel/src/client.rs`, e NON dentro `Arbiter`** | è la chiusura di **P-16**. L'arbitro conosce riserve, corsie e concessioni, e **niente** dei client: dargli un `ClientId` metterebbe una nozione della porta `ipc` dentro il tipo che ADR-0005 tiene sulla **risorsa**. ⛔ **E il doc del modulo dichiara che NON è il ciclo di orchestrazione**, o il primo che rilegge `E50` ed `E51` le crede chiuse da un file che si chiama come loro: quel ciclo decide **quando** `promote` gira rispetto ad `admit`, questo risponde a **un** evento con **un** rilascio |

**La baseline di partenza, misurata il 2026-08-30 e da NON citare nei compiti:**
`bash scripts/gate.sh` → `GATE GREEN` · `cargo test --locked --workspace --no-fail-fast` →
**37 bersagli, 267 passate, 0 fallite, 2 ignorate**.

---

## La mappa dei file

⛔ **Nessun file di `platform`.** Il trasporto vero è scaglionato: è la voce 5, chiusa
dichiarandolo.

| File | Chi lo tocca | Responsabilità |
|---|---|---|
| `crates/kernel/src/arbiter/mod.rs` | compito 1 | `ArbiterId`, `Released`, `release` |
| `crates/kernel/src/parameters.rs` | compito 1 | il terzo campo consegnato |
| `crates/kernel/src/ports/process.rs` | compiti 1, 3 | `Started`, `Killed`; e al compito 3 **solo il doc di modulo**, non il `Frame` |
| `crates/kernel/src/framing.rs` | **creato** dal compito 3 | l'inquadratura: lunghezza dichiarata a larghezza fissa, condivisa dai due canali privati, e `WireError` |
| `crates/kernel/src/wire/mod.rs` | **creato** dal compito 3 | la cartella dei due schemi: dichiara i due figli e nient'altro |
| `crates/kernel/src/wire/worker.rs` | **creato** dal compito 3 | lo schema del canale worker: `FromWorker`, coi suoi `encode`/`decode` |
| `crates/kernel/src/wire/ipc.rs` | **creato** dal compito 4 | lo schema `ipc`: l'enumerazione dei messaggi |
| `crates/kernel/src/ports/ipc.rs` | compito 4 | **solo due richiami datati**: il formato e il timbro nel doc di modulo, e la frase di `ClientId` che si **ri-punta** (§6.5) |
| `crates/kernel/src/arbiter/resource.rs` | compiti 3, 4 | i derive di filo su `Mib` (3), `ComputeClass` e `Preemption` (4) — **D17**, e il raggio è **P-10** |
| `crates/kernel/src/time.rs` | compito 4 | il derive di filo su `Millis`, che `Preemption::After` trascina |
| `crates/kernel/src/sensor.rs` | **creato** dal compito 5 | il contratto del sensore di ADR-0009 |
| `crates/kernel/src/gateway/mod.rs` | **creato** dal compito 6 | il decisore, il filtro dei vincoli e il gettone di conformità |
| `crates/kernel/src/permission.rs` | **creato** dal compito 7 | la tripla, e la proiezione dal giornale |
| `crates/kernel/src/degradation.rs` | **creato** dal compito 8 | lo stato di degrado, **ricalcolato** e non cacheato |
| `crates/kernel/src/record.rs` | compito 6 | la variante `RecordKind` del record di routing |
| `crates/kernel/tests/frozen/` | compito 6 | il **quarto** record congelato (D6) |
| `crates/simulator/src/ipc.rs` | **creato** dal compito 9 | la finta gui guidata dal seme, sul precedente di `CrashingJournal` |
| `crates/kernel/src/client.rs` | **creato** dal compito 9 | ⛔ **riga aggiunta il 2026-08-30, finding P-16:** `ClientGrants` e `on_disconnect` — la riconciliazione che §5.7 riga 3 pretende e che la mappa non ospitava |
| `crates/kernel/tests/frozen/` | compiti 5, **6** e **7** | ⛔ **riga corretta il 2026-08-30:** diceva *«il quarto record congelato (D6)»* e il compito 6, ed erano **due** cose sbagliate — i record nuovi sono **uno per variante** (D21, D22), quindi **tre**, e il primo lo fa il compito **5** |
| `crates/daemon/src/main.rs` | compiti 1, 6, 7, 8 | i default letterali dei parametri nuovi |

📌 **Perché file separati e non un modulo solo:** il progetto già lo fa così — `arbiter` è
una cartella con tre file dal Task 8 del Traguardo 5, e le porte sono un file per famiglia.
Un file per responsabilità è la convenzione, non una scelta di questo piano.

⚠️ **RICHIAMO DEL 2026-08-30, scrivendo la Parte B — tre celle di questa mappa erano sbagliate,
e la causa è la stessa: `wire/mod.rs` era assegnato al compito 4.** Lo schema del **canale
worker** non aveva nessuna casa (P-6), quindi il compito 3 avrebbe dovuto inventarne una mentre
la eseguiva; e la cella di `ports/process.rs` prometteva *«il `Frame` che diventa codificato»*,
che con la forma della **D8** non succede — il `Frame` **resta identico**, e ciò che il compito 3
tocca lì è il **doc di modulo** reso falso (P-7). ⛔ **Corretta e non riscritta in silenzio:** una
mappa dei file è un'affermazione come le altre, ed `E154` del Traguardo 5 nasce esattamente da
un elenco di file che non nominava due lavori veri — gotcha **#65** applicato all'intestazione.

---

## Parte A — la concessione che torna

### Compito 1: `E30` + `R6` + `E21` — l'identità dell'arbitro, `Released`, `Started` e `Killed`

**Files:**
- Modify: `crates/kernel/src/arbiter/mod.rs` — `ArbiterId`, `Released`, `release`
- Modify: `crates/kernel/src/parameters.rs` — il terzo campo
- Modify: `crates/kernel/src/ports/process.rs` — `Started`, `Killed`, le due firme
- Modify: `crates/daemon/src/main.rs` — i default letterali
- Modify: i quarantadue siti di `Parameters::new` (comando in P-4)
- Test: `crates/kernel/tests/arbiter_admission.rs`, `crates/kernel/tests/worker_tokens.rs`,
  `crates/kernel/tests/ports_are_implementable.rs`,
  `crates/simulator/tests/arbiter_campaign.rs`
- Test: i nove `.stderr` di `crates/kernel/tests/compile_fail/` (elenco in P-4)

⛔ **Leggi P-4 prima di cominciare.** Il costo vero di questo compito sta lì, non nella §2.4
del disegno.

#### Commit 1a — l'identità (`E21`)

- [ ] **Passo 1: scrivi la sonda che fallisce**

In `crates/kernel/tests/parameters_delivered.rs`, in fondo:

```rust
#[test]
fn the_arbiter_identity_is_delivered_and_not_invented() {
    // ⛔ THE POINT IS THE ABSENCE OF A DEFAULT. §6.1.3 forbids the kernel to MINT an
    // identifier, and ADR-0034 says a decision reads only what it was handed: an arbiter
    // that chose its own id would be doing both.
    let parameters = Parameters::new(64, Mib(8192), ArbiterId::new(7));
    assert_eq!(parameters.arbiter_id(), ArbiterId::new(7));
}
```

- [ ] **Passo 2: lancia la sonda e verifica che NON COMPILI**

```bash
cargo test --locked -p kernel --test parameters_delivered 2>&1 | head -20
```

Atteso: `error[E0433]` o `error[E0412]` su `ArbiterId` — il tipo non esiste. ⛔ **Se compila,
fermati e scrivi una voce d'errata:** significa che qualcuno l'ha già costruito, ed è il
gotcha **#49**.

- [ ] **Passo 3: scrivi `ArbiterId` in `crates/kernel/src/arbiter/mod.rs`**

Accanto a `GrantId`, e **sopra** `Grant`:

```rust
/// The identity of one arbiter, DELIVERED and never minted.
///
/// ⛔ IT EXISTS FOR ONE QUESTION: `release` must be able to tell "a grant I issued and have
/// already swept" from "a grant of ANOTHER arbiter". Without it `held.remove` answers `None`
/// to both, and the two need opposite answers -- see `Released` and `ReleaseError`.
///
/// ⛔ DELIVERED, per ADR-0034: it travels in `Parameters`, and §6.1.3 forbids the kernel to
/// mint an identifier. Nothing here generates one.
///
/// ⚠️ THE FIELD IS PRIVATE AND THE CONSTRUCTOR IS NOT, and the asymmetry is the whole of it:
/// `daemon` has to build one, so a tuple literal from any crate would give the same forgery
/// `RecordV1` gave in AUD-050 -- a guard is worth exactly what its constructor is worth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArbiterId(u64);

impl ArbiterId {
    /// The identity in. It is a value the composition root hands down, not a choice.
    pub const fn new(value: u64) -> Self {
        ArbiterId(value)
    }
}
```

- [ ] **Passo 4: aggiungi il campo a `Parameters`**

In `crates/kernel/src/parameters.rs`, il campo e il suo lettore, **positionally last**:

```rust
pub struct Parameters {
    executor_turn_limit: u64,
    total_vram: Mib,
    arbiter_id: ArbiterId,
}
```

```rust
    pub const fn new(executor_turn_limit: u64, total_vram: Mib, arbiter_id: ArbiterId) -> Self {
        Parameters {
            executor_turn_limit,
            total_vram,
            arbiter_id,
        }
    }

    /// Which arbiter these parameters belong to.
    ///
    /// ⛔ DELIVERED, never invented: the kernel has no way to produce one, and §6.1.3 says
    /// it must not. Two arbiters built from the same value ARE the same arbiter as far as
    /// `release` is concerned, and that is the caller's statement to make, not ours.
    pub const fn arbiter_id(self) -> ArbiterId {
        self.arbiter_id
    }
```

⚠️ **Aggiungi l'`use` di `ArbiterId`** in `parameters.rs`; il modulo `arbiter` lo esporta già.

- [ ] **Passo 5: fai compilare i quarantadue siti**

⛔ **Uno per uno, e NON con una sostituzione globale.** I nove casi `compile_fail` vanno
guardati in faccia: sette di essi cadrebbero per **arità** prima di arrivare all'errore che
asseriscono, che è un `mismatch` e non il loro oracolo.

```bash
cargo build --locked --workspace --tests 2>&1 | grep -E "^error" | head -50
```

Per i siti di prova il valore è indifferente e si usa `ArbiterId::new(1)`; in
`crates/daemon/src/main.rs` è un **default letterale**, come `executor_turn_limit` e
`total_vram` già sono (vincolo 11 della §11).

- [ ] **Passo 6: rileggi i nove `.stderr`, uno per uno**

```bash
cargo test --locked -p kernel --test compile_fail 2>&1 | tail -40
```

⛔ **Mai `TRYBUILD=overwrite`** (D2). Per ogni caso che dà `mismatch`, apri il `.stderr`,
leggi che cosa è cambiato, e correggi **a mano** solo ciò che è cambiato davvero. Se il testo
atteso non cambia ma il **numero di riga** sì, è perché hai spostato righe nel `.rs`: è la
stessa trappola che il rimedio di **AUD-042/045** ha misurato — tre righe di commento
spostarono il difetto dalla riga 34 alla 37 e resero `mismatch` l'oracolo che il paragrafo
esisteva per proteggere.

- [ ] **Passo 7: lancia la sonda e verifica che passi**

```bash
cargo test --locked -p kernel --test parameters_delivered 2>&1 | tail -5
```

Atteso: `test result: ok.` con **una** sonda in più di prima.

- [ ] **Passo 8: il cancello, e la baseline rimisurata**

```bash
bash scripts/gate.sh
```

Atteso: `GATE GREEN`.

```bash
cargo test --locked --workspace --no-fail-fast 2>&1 | grep -E "^(running|test result:)" | awk '/^running/{t++} /^test result:/{p+=$4; f+=$6; i+=$8} END{print "bersagli="t"  passate="p"  fallite="f"  ignorate="i}'
```

- [ ] **Passo 9: commit**

```bash
git add -A && git commit -m "traguardo 6 (compito 1a): l'arbitro guadagna un'identita' CONSEGNATA -- senza, release non sa distinguere la propria concessione gia' spazzata da quella di un altro"
```

#### Commit 1b — `release` risponde tre cose (`E30`)

- [ ] **Passo 1: scrivi le due sonde che falliscono**

In `crates/kernel/tests/arbiter_admission.rs`, in fondo. ⛔ **Sono due perché le vie sono
due e falliscono diversamente** — una direzione tenuta da una mutazione è tenuta da niente
(gotcha **#72**):

```rust
#[test]
fn a_grant_of_this_arbiter_released_after_its_window_is_not_an_error() {
    // ⛔ THIS IS THE DECISION OF 2026-08-28: release NEVER answers `Err` to a grant of its
    // own. An expired window is not a failure of the release -- the sweep simply got there
    // first -- and the caller learns that from `AlreadyCollected`, not from an error.
    let mut arbiter = arbiter(ArbiterId::new(1), TOTAL);
    let Admission::Granted(grant) = arbiter.admit(
        &profile("short-lived", 4_096, ComputeClass::Batch),
        Millis::new(5_000),
        Monotonic::ORIGIN,
    ) else {
        panic!("4096 of 16384 fits");
    };

    let released = arbiter.release(grant, Monotonic::from_millis(5_001));

    assert_eq!(released, Ok(Released::AlreadyCollected));
}

/// The counter-probe, and it is the direction that is skipped: inside the window the release
/// says WHAT CAME BACK. Without it, "always answer AlreadyCollected" stays green.
#[test]
fn a_grant_released_inside_its_window_reports_what_came_back() {
    let mut arbiter = arbiter(ArbiterId::new(1), TOTAL);
    let Admission::Granted(grant) = arbiter.admit(
        &profile("short-lived", 4_096, ComputeClass::Batch),
        Millis::new(5_000),
        Monotonic::ORIGIN,
    ) else {
        panic!("4096 of 16384 fits");
    };

    let released = arbiter.release(grant, Monotonic::from_millis(4_999));

    assert_eq!(released, Ok(Released::Now(Mib::new(4_096))));
}
```

⛔ **L'aiutante `arbiter` di quel banco guadagna un primo argomento, ed è il punto del passo
5.** Oggi è `fn arbiter(total: Mib) -> Arbiter` e costruisce
`Parameters::new(TURN_LIMIT, total)`; diventa `fn arbiter(id: ArbiterId, total: Mib) -> Arbiter`.
⚠️ **Se gli dai un identificativo fisso dentro l'aiutante invece che come argomento**, la sonda
`a_grant_released_on_the_wrong_arbiter_…` costruisce **due arbitri con la stessa identità** e
smette di provare ciò che il suo nome dice — verde, e vuota. È il difetto che il passo 5 di
questo commit esiste per cogliere.

⚠️ **Gli altri nomi sono quelli del banco, verificati:** `profile(name, vram, lane)` a **tre**
argomenti, le costanti `TURN_LIMIT`, `TOTAL` e `LONG`, e gli idiomi `Mib::new`, `Millis::new`,
`Monotonic::ORIGIN`, `Monotonic::from_millis`. **Non inventarne di nuovi.**

- [ ] **Passo 2: lancia e verifica che NON COMPILI**

```bash
cargo test --locked -p kernel --test arbiter_admission 2>&1 | head -20
```

Atteso: `error[E0433]` su `Released` — il tipo non esiste.

- [ ] **Passo 3: scrivi `Released` e cambia `release`**

In `crates/kernel/src/arbiter/mod.rs`, accanto a `ReleaseError`:

```rust
/// What handing a grant back actually did.
///
/// ⛔ TWO ANSWERS AND NOT A `bool`, because the caller has something to do with the
/// difference: `Now` says this many MiB came back to the budget in this call, and
/// `AlreadyCollected` says the sweep had already taken them -- the books are the same either
/// way, and only the first is a change the caller caused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Released {
    /// Taken back now. The `Mib` is what returned to the budget.
    Now(Mib),
    /// The sweep had it already -- the declared window closed, or the grace of a revocation
    /// ran out. NOT an error: the reservation is not held by anyone either way.
    AlreadyCollected,
}
```

E la funzione, che ora **consulta l'identità**:

```rust
    /// Hands a grant back.
    ///
    /// ⛔ DECISION OF 2026-08-28: a grant THIS arbiter issued is never an `Err`. Its window
    /// may have closed and its grace may have run out; in both cases the sweep took the
    /// reservation back and the answer is `AlreadyCollected`. Only the grant this arbiter
    /// NEVER ISSUED is a caller defect, and that one stays an error.
    ///
    /// ⚠️ `UnknownGrant` NOW MEANS ONE THING, where it used to mean three. The two causes
    /// that left it are the two above; what remains is a grant minted by another arbiter,
    /// which `Parameters::arbiter_id` is what lets us see.
    pub fn release(&mut self, grant: Grant, now: Monotonic) -> Result<Released, ReleaseError> {
        if grant.issuer != self.id {
            return Err(ReleaseError::UnknownGrant);
        }
        self.collect_expired(now);
        match self.held.remove(&grant.id) {
            Some(held) => Ok(Released::Now(held.reserved)),
            None => Ok(Released::AlreadyCollected),
        }
    }
```

⛔ **`Grant` guadagna il campo `issuer: ArbiterId`**, scritto da `issue` e da nessun altro; e
`Arbiter` guadagna `id: ArbiterId`, letto da `Parameters` nel proprio `new`. Il campo di
`Grant` resta **privato**, come `id`.

- [ ] **Passo 4: lancia le due sonde e verifica che passino**

```bash
cargo test --locked -p kernel --test arbiter_admission 2>&1 | tail -5
```

- [ ] **Passo 5: verifica che la sonda del Traguardo 5 sia ANCORA VERDE, e non cancellata**

⛔ **È la condizione 5 della Definizione di «fatto», e il disegno la nomina per esteso:**
chiuderla cancellandola sarebbe *«cancellare una sonda per prendere una decisione»*, il gotcha
**#73** al contrario.

```bash
cargo test --locked -p kernel --test arbiter_admission a_grant_released_on_the_wrong_arbiter_is_an_error_and_not_a_silent_credit -- --exact 2>&1 | tail -5
```

Atteso: `1 passed`. ⚠️ **Quella sonda costruisce due arbitri:** ora devono ricevere **due
`ArbiterId` diversi**, o non prova più niente. Se il banco gliene dà uno solo, correggilo e
scrivi una voce d'errata — è il difetto che questo passo esiste per cogliere.

- [ ] **Passo 6: la mutazione di controllo, e sono DUE**

⛔ **D7: una alla volta, compilando in un passo separato dall'eseguire, revocando da copia.**

| Mutazione | Deve uccidere |
|---|---|
| `if grant.issuer != self.id` → `if false` | `a_grant_released_on_the_wrong_arbiter_is_an_error_and_not_a_silent_credit` |
| `Ok(Released::AlreadyCollected)` → `Err(ReleaseError::UnknownGrant)` | `a_grant_of_this_arbiter_released_after_its_window_is_not_an_error` |

```bash
cp crates/kernel/src/arbiter/mod.rs "$SCRATCH/mod.rs.orig"
# muta, poi:
cargo build --locked -p kernel --tests
cargo test --locked -p kernel --test arbiter_admission 2>&1 | tail -5
cp "$SCRATCH/mod.rs.orig" crates/kernel/src/arbiter/mod.rs
git diff --stat crates/kernel/src/arbiter/mod.rs   # deve essere VUOTO
```

- [ ] **Passo 7: il cancello**

```bash
bash scripts/gate.sh
```

- [ ] **Passo 8: commit**

```bash
git add -A && git commit -m "traguardo 6 (compito 1b): release non risponde piu' Err a una concessione PROPRIA -- UnknownGrant significa ora una cosa sola, e le altre due cause diventano AlreadyCollected"
```

#### Commit 1c — la porta `process` restituisce la concessione (`R6`)

- [ ] **Passo 1: scrivi le due sonde che falliscono**

In `crates/kernel/tests/worker_tokens.rs`. ⛔ **Due, e la seconda è quella che il disegno
chiama *«la metà che mancava»*** — l'avvio **fallito**, che non era discusso da nessuna parte:

⛔ **`a_real_grant()` costruisce l'arbitro e lo BUTTA, quindi non serve a queste due sonde:**
loro devono **rilasciare** sull'arbitro che ha emesso. Aggiungi accanto a esso un aiutante che
restituisce entrambi, e **non** cambiare `a_real_grant`, che le altre quattro sonde usano:

```rust
/// The arbiter AND the grant it issued. ⛔ `a_real_grant` throws the arbiter away, which is
/// right for the probes about token SHAPE; these two are about the reservation coming home,
/// so they need the books that hold it.
fn an_arbiter_and_a_real_grant() -> (Arbiter, Grant) {
    let mut arbiter = Arbiter::new(
        Parameters::new(10_000, Mib::new(16_384), ArbiterId::new(1)),
        VramPolicy::Remote(RemotePolicy),
    );
    let Admission::Granted(grant) = arbiter.admit(
        &ResourceProfile {
            name: "asr-realtime",
            reserved_vram: Mib::new(1_024),
            compute_class: ComputeClass::Realtime,
            preemption: Preemption::Never,
        },
        Millis::new(1_000_000),
        Monotonic::ORIGIN,
    ) else {
        panic!("1024 of 16384 fits");
    };
    (arbiter, grant)
}

#[test]
fn a_worker_that_is_killed_gives_the_grant_back() {
    let (mut arbiter, grant) = an_arbiter_and_a_real_grant();
    let Started::Running(worker) =
        FakeProcess.start(grant, WorkerDescriptor::new(b"asr.exe".to_vec()))
    else {
        panic!("the fake starts every worker it is asked for");
    };

    let killed = worker.kill();

    // ⛔ THE GRANT IS OUTSIDE EVERY `Result`, and this assertion is why: the reservation is a
    // fact of the BOOKS, not of the worker's health. `kill` is always lawful (§5.3 point 4),
    // so a worker that died badly still owes its reservation back.
    assert!(killed.outcome.is_ok());
    assert_eq!(
        arbiter.release(killed.grant, Monotonic::ORIGIN),
        Ok(Released::Now(Mib::new(1_024)))
    );
}

#[test]
fn a_start_that_fails_gives_the_grant_back_by_name() {
    // ⛔ THIS VIA WAS NOT DISCUSSED ANYWHERE before the milestone 6 design measured it: today
    // `start` takes the grant BY VALUE and drops it on `Err`, and nothing can rebuild it --
    // `GrantId` is private and `grant_has_no_constructor.rs` pins that. The reservation then
    // sat in the books for the whole declared window, and only the sweep got it back.
    let (mut arbiter, grant) = an_arbiter_and_a_real_grant();

    let Started::Rejected { grant, error } =
        FailingProcess.start(grant, WorkerDescriptor::new(b"asr.exe".to_vec()))
    else {
        panic!("FailingProcess refuses every start, so this must be the rejected arm");
    };

    assert_eq!(error, ProcessError::StartFailed);
    assert_eq!(
        arbiter.release(grant, Monotonic::ORIGIN),
        Ok(Released::Now(Mib::new(1_024)))
    );
}
```

⚠️ **`FakeProcess` e `FailingProcess` sono `struct` unitarie** e si usano **senza `::new()`** —
`FakeProcess.start(…)`. `FailingProcess` e la sonda `a_spawn_that_does_not_happen_is_start_failed`
nascono dal rimedio di **AUD-051**: quella sonda asserisce `outcome.err()`, quindi **cambia con
la firma** e va riscritta sul ramo `Started::Rejected` — è un lavoro del compito, non una
sorpresa.

- [ ] **Passo 2: lancia e verifica che NON COMPILI**

```bash
cargo test --locked -p kernel --test worker_tokens 2>&1 | head -20
```

Atteso: `error[E0433]` su `Started`.

- [ ] **Passo 3: scrivi `Started` e `Killed`, e cambia le due firme**

In `crates/kernel/src/ports/process.rs`:

```rust
/// What starting a worker did. ⛔ NOT a `Result`, and the shape is `Admission`'s.
///
/// ⛔ THE REJECTED ARM CARRIES THE GRANT BACK BY NAME. `start` consumes it, so before this
/// type a failed start dropped a reservation nobody could rebuild -- `GrantId` is private and
/// `tests/compile_fail/grant_has_no_constructor.rs` pins it -- and the books held it for the
/// whole declared window. The sweep was the only way back.
///
/// ⚠️ WHY NOT `Result<H, (Grant, ProcessError)>`: no error in this repository carries the
/// value it consumed, measured with
/// `grep -rnE "Result<[^,]+, *\([A-Z]" crates/ --include=*.rs`, which returns nothing. The
/// shape this project uses for "several outcomes, each carrying what belongs to it" is
/// `Admission`. A second idiom would be a second way to say one thing.
#[must_use]
pub enum Started<H> {
    /// The worker is alive, and the grant is now its.
    Running(H),
    /// It never started. The grant comes back, and so does the reason.
    Rejected { grant: Grant, error: ProcessError },
}

/// What killing a worker did.
///
/// ⛔ A STRUCT AND NOT AN ENUM, because there are not two states: there is ONE state with two
/// facts. The grant comes back whatever happened, and `outcome` says whether the kill itself
/// went cleanly.
///
/// ⛔ THE GRANT SITS OUTSIDE EVERY `Result`, and that is the teaching part: it comes back even
/// on the arm where the worker died badly. `kill` is ALWAYS LAWFUL (§5.3 point 4), and a
/// reservation is a fact of the books, not of the process's health.
#[must_use]
pub struct Killed {
    /// The reservation, back to whoever will hand it to the arbiter.
    pub grant: Grant,
    /// Whether the kill itself succeeded.
    pub outcome: Result<(), ProcessError>,
}
```

E le due firme, dentro i tratti:

```rust
    /// Kills the worker, and it is ALWAYS lawful (§5.3, point 4).
    ///
    /// ⛔ CONSUMES the `Worker`: instructing it after the kill does not compile.
    /// ⛔ AND IT RETURNS THE GRANT, outside the `Result` -- see `Killed`.
    fn kill(self) -> Killed;
```

```rust
    /// Starts a worker.
    ///
    /// ⛔ Takes the GRANT as an argument: whoever writes "start the worker" without one
    /// does not compile. This is the half of I2 that belongs to the compiler; the other
    /// half -- that `process` is the only port towards processes -- rests on a level 2
    /// check and is therefore deletable. Declared, not hidden (§5.6).
    ///
    /// ⛔ IT RETURNS `Started` AND NOT A `Result`, so the grant of a failed start has a way
    /// home -- see `Started::Rejected`.
    fn start(&mut self, grant: Grant, descriptor: WorkerDescriptor) -> Started<Self::Handle>;
```

⛔ **`Grant` deve essere raggiungibile da `ports::process`** per nome, e non ri-esportato: il
piano del Traguardo 5 decise *«`Grant` non è ri-esportato da `ports::process`»*. Usa il
percorso pieno `crate::arbiter::Grant`.

- [ ] **Passo 4: aggiorna le finte dei banchi**

Sono **sei**, in tre file, e il comando che le trova è quello di P-4 ristretto a `impl`:

```bash
grep -rn "impl Worker for\|impl Process for" crates/ --include=*.rs
```

`FakeWorker` e `FakeProcess` e `FailingProcess` in `worker_tokens.rs`; `ScriptedWorker` e
`SpawningProcess` in `ports_are_implementable.rs`; più le tre finte dentro i casi
`compile_fail`. ⛔ **`SpawningProcess::start` non deve diventare infallibile per comodità:** il
suo doc dichiara che cosa compra, e cambiarlo in silenzio è la specie di difetto che
**AUD-054** ha misurato. ⛔ **E `a_spawn_that_does_not_happen_is_start_failed` va RISCRITTA, non
cancellata:** asserisce `outcome.err()`, che con `Started` non esiste più; la forma nuova
smonta `Started::Rejected { error, .. }` e tiene l'**uguaglianza** sulla variante, che è ciò
che il suo doc dichiara di comprare contro un `is_err()`.

- [ ] **Passo 5: rileggi i tre `.stderr` della porta, uno per uno**

```bash
cargo test --locked -p kernel --test compile_fail 2>&1 | tail -40
```

⛔ **Mai in blocco** (D2). I tre sono `instructing_after_the_kill`, `reading_without_a_receipt`
e `reading_twice_from_one_receipt`; il primo asserisce `E0382` **sul `Worker` mosso da
`kill`**, e `kill` ora restituisce `Killed` invece di `Result<(), _>`: verifica che l'errore
sia ancora quello, e **non** che il caso semplicemente fallisca.

- [ ] **Passo 6: le due sonde passano**

```bash
cargo test --locked -p kernel --test worker_tokens 2>&1 | tail -5
```

- [ ] **Passo 7: la mutazione di controllo**

| Mutazione | Deve uccidere |
|---|---|
| in `Started::Rejected`, non restituire il `grant` ma un `Grant` di un secondo arbitro | `a_start_that_fails_gives_the_grant_back_by_name` — l'asserzione su `release` diventa `Err(UnknownGrant)` |

⚠️ **Se questa mutazione NON uccide**, la sonda sta provando l'arità e non l'identità: è la
stessa distinzione che il registro dichiara per `SingleReceipt::new`.

- [ ] **Passo 8: il cancello e la baseline**

```bash
bash scripts/gate.sh
```

- [ ] **Passo 9: commit**

```bash
git add -A && git commit -m "traguardo 6 (compito 1c): start e kill restituiscono la concessione -- e la via dell'avvio FALLITO non era discussa da nessuna parte"
```

#### Criterio di chiusura del compito 1

- [ ] `GATE GREEN` a tutti e tre i commit
- [ ] `a_grant_released_on_the_wrong_arbiter_is_an_error_and_not_a_silent_credit` **verde e
      non cancellata**, coi due arbitri che ricevono due `ArbiterId` diversi
- [ ] i nove `.stderr` riletti **uno per uno**, nessuno rigenerato in blocco
- [ ] `ReleaseError` ha **ancora una sola variante**
- [ ] le voci `E30`, `R6` ed `E21` della tabella unica di
      [`porta-di-qualita.md`](../../porta-di-qualita.md) sono marcate chiuse, col commit

---

## Parte B — il filo

⛔ **Due compiti, e l'ordine fra loro non è negoziabile.** Il **3** costruisce l'inquadratura e
lo schema del canale **worker**, il cui formato ADR-0037 ha già misurato e chiuso; il **3bis**
decide con quale formato si scriverà lo schema **`ipc`**, e sta **prima** del compito 4 perché
scriverlo in `bincode` **è** la decisione, presa per omissione (§3.5 del disegno, **D4**).

⚠️ **I due non condividono nessun file**, misurato mentre si scriveva questa parte: il 3 tocca
`framing.rs`, `wire/`, `lib.rs` e un doc di `ports/process.rs`; il 3bis tocca
`crates/kernel/Cargo.toml`, [`riferimenti.md`](../../riferimenti.md) e — solo nel caso B — il
`Cargo.lock`. Restano comunque **due dispacci separati**, perché la specie di lavoro è diversa:
uno scrive codice, l'altro fa una **ricerca**.

### Compito 3: §6.10 — l'inquadratura e lo schema del canale worker (vincolo 15)

**Files:**
- Create: `crates/kernel/src/framing.rs` — la busta e `WireError`
- Create: `crates/kernel/src/wire/mod.rs` — la cartella dei due schemi
- Create: `crates/kernel/src/wire/worker.rs` — `FromWorker`, `encode`, `decode`
- Modify: `crates/kernel/src/lib.rs` — i due `pub mod` nuovi
- Modify: `crates/kernel/src/arbiter/resource.rs` — `Mib` guadagna i due derive (Passo 4 di 3b)
- Modify: `crates/kernel/src/ports/process.rs` — **solo il doc di modulo** (P-7)
- Test: `crates/kernel/tests/framing.rs`, `crates/kernel/tests/worker_wire.rs` — **creati**
- Modify: [`porta-di-qualita.md`](../../porta-di-qualita.md) — la riga `Q4 · I5 · §6.10`

⛔ **Leggi P-5, P-6 e P-7 prima di cominciare.** Il primo dice che la riga della §11 che stai per
chiudere porta una clausola falsa; il secondo perché lo schema ha una casa che il disegno non gli
dava; il terzo quale frase il tuo commit rende stantia.

⛔ **E i due banchi vivono in `tests/`, cioè FUORI dalla crate, non in un `mod tests`.** È la
terza domanda del pre-controllo di [`../../../CLAUDE.md`](../../../CLAUDE.md): un artefatto
sbagliato compila, e a coglierlo è solo scriverne un'implementazione da fuori. `framing::frame`
e `FromWorker` sono `pub`: se non lo fossero abbastanza, questi banchi non compilerebbero.

#### Commit 3a — l'inquadratura, e la larghezza si decide UNA volta sola

- [ ] **Passo 1: scrivi le sonde che falliscono**

In `crates/kernel/tests/framing.rs`, file nuovo:

```rust
//! The envelope shared by the two private channels. ⛔ THE PROBES LIVE OUTSIDE THE CRATE on
//! purpose: what they hold is that the envelope is USABLE from outside, which is the only
//! form of "the boundary is real" this repository accepts (milestone 3, task 8).

use kernel::framing::{self, WireError};

#[test]
fn a_framed_body_comes_back_exactly() {
    let body = [1u8, 2, 3, 4, 5];
    let framed = framing::frame(&body).expect("frame");
    assert_eq!(framing::unframe(&framed), Ok(&body[..]));
}

#[test]
fn the_declared_length_is_four_bytes_big_endian() {
    // ⛔ THE BYTE ORDER IS AN ASSERTION AND NOT A COMMENT. ADR-0037 chooses a wire format on
    // what the PEER can read: `DataView.getUint32(0)` in TypeScript and `struct.unpack(">I")`
    // in Python are the forms that need no flag. Flipped to little-endian, nothing else in
    // this workspace would go red -- both peers live outside it.
    let framed = framing::frame(&[0xAA]).expect("frame");
    assert_eq!(framed, [0x00, 0x00, 0x00, 0x01, 0xAA]);
}

#[test]
fn a_truncated_frame_is_refused() {
    // Declares five, carries two.
    let bytes = [0x00, 0x00, 0x00, 0x05, 0x01, 0x02];
    assert_eq!(framing::unframe(&bytes), Err(WireError::Incomplete));
}

#[test]
fn bytes_shorter_than_the_prefix_are_refused() {
    // ⚠️ NOT the same failure as the one above, and it is worth its own probe: here there is
    // no declared length AT ALL, so the code path that reads it must not be reached.
    assert_eq!(framing::unframe(&[0x00, 0x00, 0x00]), Err(WireError::Incomplete));
}

#[test]
fn a_frame_with_a_tail_is_refused() {
    // Declares one, carries three. ⛔ THIS IS THE HALF A CBOR DECODER CANNOT SEE: it stops at
    // the first complete element and ignores what follows (gotcha #34, measured in §6.10.4).
    let bytes = [0x00, 0x00, 0x00, 0x01, 0x01, 0x02, 0x03];
    assert_eq!(framing::unframe(&bytes), Err(WireError::TrailingBytes));
}
```

- [ ] **Passo 2: lancia il banco e verifica che NON COMPILI**

```bash
cargo test --locked -p kernel --test framing 2>&1 | head -20
```

Atteso: `error[E0432]` — `kernel::framing` non esiste. ⛔ **Se compila, fermati e scrivi una
voce d'errata:** qualcuno l'ha già costruito, ed è il gotcha **#49**.

- [ ] **Passo 3: scrivi `crates/kernel/src/framing.rs`**

```rust
//! The envelope of the two private channels: a declared length, then the body.
//!
//! ⛔ WHAT IS SHARED IS NEITHER THE TRANSPORT NOR THE SCHEMA -- it is envelope bytes.
//! ADR-0035 reads I4 as "one transport and one schema PER PRIVATE CHANNEL", and ADR-0037
//! distrusts arguments of SYMMETRY between the two channels. Neither is contradicted: both
//! peers must read a length prefix whatever the body format is, so the problem here is
//! LITERALLY the same one and not a symmetric one. §6.3 of the milestone 6 design says so,
//! and whether that reading holds is open item 8 for the owner.
//!
//! ⛔ THE WIDTH IS DECIDED HERE AND NOWHERE ELSE (§6.3). Four bytes, big-endian.

use alloc::vec::Vec;

/// The width of the declared length, in bytes.
const LENGTH_WIDTH: usize = 4;

/// The longest body this envelope can declare.
///
/// ⚠️ THE GUARD ABOVE IT IS DECLARED AND NOT EXERCISED, and saying so is the point: reaching
/// it needs a body of four gibibytes, so no probe in this repository can produce one. What is
/// held instead is the WIDTH, by `the_declared_length_is_four_bytes_big_endian`: while the
/// prefix is four bytes this constant cannot be anything else.
pub const MAX_BODY_LEN: usize = u32::MAX as usize;

/// What can go wrong reading an envelope.
///
/// ⚠️ NO VARIANT CARRIES A PAYLOAD, and that is the shape of the project rather than an
/// omission: no error in this repository carries the value it consumed. The caller that wants
/// the numbers has the bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireError {
    /// Fewer bytes than the frame declares -- the prefix itself may be missing.
    Incomplete,
    /// More bytes than the frame declares.
    TrailingBytes,
    /// The body is longer than a declared length can express. See `MAX_BODY_LEN`.
    TooLong,
}

/// Wraps a body in its envelope.
pub fn frame(body: &[u8]) -> Result<Vec<u8>, WireError> {
    if body.len() > MAX_BODY_LEN {
        return Err(WireError::TooLong);
    }
    let mut bytes = Vec::with_capacity(LENGTH_WIDTH + body.len());
    bytes.extend_from_slice(&(body.len() as u32).to_be_bytes());
    bytes.extend_from_slice(body);
    Ok(bytes)
}

/// Reads a body out of its envelope.
///
/// ⛔ THE TWO FAILURES ARE NOT THE SAME FAULT, and §3.2 of the design puts them in one table:
/// a TAIL is caught by a decoder that checks its own position, a TRUNCATION is caught by
/// NOTHING BUT a declared length -- the tail is not there, and the CBOR can be complete all
/// the same. That is why this function exists on top of the body decoder and not instead of
/// it.
pub fn unframe(bytes: &[u8]) -> Result<&[u8], WireError> {
    if bytes.len() < LENGTH_WIDTH {
        return Err(WireError::Incomplete);
    }
    let (prefix, body) = bytes.split_at(LENGTH_WIDTH);
    let mut declared = [0u8; LENGTH_WIDTH];
    declared.copy_from_slice(prefix);
    let declared = u32::from_be_bytes(declared) as usize;
    if body.len() < declared {
        return Err(WireError::Incomplete);
    }
    if body.len() > declared {
        return Err(WireError::TrailingBytes);
    }
    Ok(body)
}
```

- [ ] **Passo 4: dichiara il modulo in `crates/kernel/src/lib.rs`**

Un `pub mod framing;` accanto agli altri. ⛔ **Non toccare il paragrafo in testa al file:** dice
già che *«ciò che questa crate contiene è la lista dei `pub mod`»*, ed è la forma che il finding
**AUD-046** ha messo lì proprio perché non invecchiasse. Aggiungere una riga al riassunto lo
riaprirebbe.

- [ ] **Passo 5: lancia le sonde e verifica che passino**

```bash
cargo test --locked -p kernel --test framing 2>&1 | tail -5
```

Atteso: `5 passed`.

- [ ] **Passo 6: le mutazioni di controllo, e sono TRE**

⛔ **D7: una alla volta, compilando in un passo separato dall'eseguire, revocando da copia.**

| Mutazione | Deve uccidere |
|---|---|
| `to_be_bytes` → `to_le_bytes` (in **entrambi** i siti, o non compila il round-trip) | `the_declared_length_is_four_bytes_big_endian`, **e nient'altro** |
| `if body.len() < declared` → `if false` | `a_truncated_frame_is_refused` |
| `if body.len() > declared` → `if false` | `a_frame_with_a_tail_is_refused` |

⚠️ **La prima ha un oracolo in più della propria riga: deve uccidere QUELLA SOLA.** Se uccide
anche il round-trip, la sonda dell'ordine non sta provando l'ordine ma la simmetria fra i due
siti — e l'ordine tornerebbe indifendibile appena qualcuno li cambia insieme, che è esattamente
il caso che conta.

```bash
cp crates/kernel/src/framing.rs "$SCRATCH/framing.rs.orig"
# muta, poi:
cargo build --locked -p kernel --tests
cargo test --locked -p kernel --test framing 2>&1 | tail -8
cp "$SCRATCH/framing.rs.orig" crates/kernel/src/framing.rs
git diff --stat crates/kernel/src/framing.rs   # deve essere VUOTO
```

- [ ] **Passo 7: il cancello**

```bash
bash scripts/gate.sh
```

⚠️ **`cargo fmt --all --check` non è un passo del cancello** (§7.4.3), e questo compito crea file
nuovi: lanciatelo a mano, o la deriva arriva col commit del prodotto come al Task 12 del
Traguardo 5.

- [ ] **Passo 8: commit**

```bash
git add -A && git commit -m "traguardo 6 (compito 3a): la busta dei due canali privati -- quattro byte big-endian, e la lunghezza dichiarata prende il TRONCAMENTO che nessun decodificatore CBOR puo' vedere"
```

#### Commit 3b — lo schema del canale worker, e la sola direzione che qualcosa impone

- [ ] **Passo 1: scrivi le sonde che falliscono**

In `crates/kernel/tests/worker_wire.rs`, file nuovo:

```rust
//! The schema of the channel towards the workers. ⛔ OUTSIDE THE CRATE, like `framing.rs`.

use kernel::arbiter::Mib;
use kernel::framing::WireError;
use kernel::wire::worker::FromWorker;

#[test]
fn a_fragment_survives_the_round_trip() {
    let message = FromWorker::Fragment(alloc_vec(&[9, 8, 7]));
    let bytes = message.encode().expect("encode");
    assert_eq!(FromWorker::decode(&bytes), Ok(message));
}

#[test]
fn a_vram_peak_survives_the_round_trip() {
    let message = FromWorker::VramPeak(Mib::new(1536));
    let bytes = message.encode().expect("encode");
    assert_eq!(FromWorker::decode(&bytes), Ok(message));
}

#[test]
fn the_byte_string_annotation_is_measured_and_not_asserted() {
    // ⛔ READING THE ATTRIBUTE IN THE SOURCE PROVES NOTHING -- what the annotation buys is a
    // SIZE, and §6.10.4 measured it: a 4096 B audio fragment costs 4101 bytes as a byte
    // string and 7813 as an array of numbers, i.e. 1.91x. Both compile, both round-trip,
    // both are correct; one costs double the traffic in silence.
    //
    // ⚠️ THE ASSERTION IS A BOUND AND NOT AN EQUALITY, and that is deliberate: an exact
    // number would go red the day the envelope or the variant index changes by a byte, i.e.
    // where the promise is KEPT (gotcha #24, the precedent is PL-1 and its `0600`). Write the
    // exact value you measure in this comment, dated, and leave the bound in the code.
    let body = alloc_vec(&[0u8; 4096]);
    let bytes = FromWorker::Fragment(body).encode().expect("encode");
    assert!(bytes.len() < 4096 + 64, "encoded {} bytes", bytes.len());
}

#[test]
fn a_frame_with_a_tail_does_not_decode() {
    let mut bytes = FromWorker::VramPeak(Mib::new(1)).encode().expect("encode");
    bytes.push(0xFF);
    assert_eq!(FromWorker::decode(&bytes), Err(WireError::TrailingBytes));
}

#[test]
fn junk_inside_the_declared_length_does_not_decode() {
    // ⛔ THE OTHER HALF, AND IT IS A DIFFERENT CHECK: here the ENVELOPE is honest -- the
    // declared length matches the body exactly -- and it is the body that carries a complete
    // CBOR element followed by a byte. `unframe` cannot see it; `position() != len()` can.
    // Remove either check and one of these two probes survives on its own merits.
    let good = FromWorker::VramPeak(Mib::new(1)).encode().expect("encode");
    let body = &good[4..];
    let mut junked = body.to_vec();
    junked.push(0xFF);
    let bytes = kernel::framing::frame(&junked).expect("frame");
    assert_eq!(FromWorker::decode(&bytes), Err(WireError::Malformed));
}
```

⚠️ **`alloc_vec` è un aiutante del banco**, non un tipo del kernel: una `Vec<u8>` costruita dal
test. Scrivilo come preferisci — ma **non** dargli un doc che prometta qualcosa, o è l'aiutante
che non teneva niente del Task 8 del Traguardo 5.

- [ ] **Passo 2: lancia il banco e verifica che NON COMPILI**

```bash
cargo test --locked -p kernel --test worker_wire 2>&1 | head -20
```

Atteso: `error[E0432]` su `kernel::wire`.

- [ ] **Passo 3: `Mib` guadagna i due derive**

In `crates/kernel/src/arbiter/resource.rs`, sul tipo `Mib`, con l'indice esplicito sul campo —
**la forma che `record.rs` usa già**, non una che questo piano ricorda:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Encode, Decode)]
pub struct Mib(#[n(0)] u64);
```

⛔ **È il primo derive di `minicbor` fuori da `record.rs`, e il costo va dichiarato accanto al
tipo:** `Mib` diventa un tipo di **filo** oltre che di decisione. La via che lo evitava —
mettere un `u64` nudo nel messaggio — è **scartata sul merito**: `Mib` esiste perché scambiare
MiB e millisecondi non compili, e un intero nudo che rientra dal filo è precisamente il caso che
i quattro `compile_fail` della §5.1 esistono per togliere.
⚠️ **Non dare per scontato `#[cbor(transparent)]`:** può esistere in `minicbor` 2.3.0 e questo
piano **non l'ha verificato**. Usa la forma sopra, che il repository esercita da tre traguardi, e
se vuoi l'altra **misurane i byte** prima — è la stessa disciplina con cui il Task 8 del
Traguardo 3 rifiutò di dettare l'API di `redb` a memoria.

- [ ] **Passo 4: scrivi `crates/kernel/src/wire/mod.rs` e `wire/worker.rs`**

`wire/mod.rs`:

```rust
//! The schemas of the two private channels, one file each.
//!
//! ⛔ SHARING A FOLDER IS NOT SHARING A SCHEMA -- ADR-0035, rule 2. The two schemas are
//! distinct and so are the two formats, and ADR-0037 measured why: the peers differ. What
//! they do share is the envelope, and it lives in `crate::framing`.
//!
//! ⚠️ `ipc` IS NOT HERE YET: it arrives with task 4, in the format that task 3bis decides.

pub mod worker;
```

`wire/worker.rs`:

```rust
//! The schema of the channel towards the workers (§6.10, ADR-0037).
//!
//! ⛔ THIS TAKES THE MECHANICS OF `record.rs` AND NOT ITS DISCIPLINE, and §6.10.3 says it in
//! as many words: no version enum, no register of retired indices, NO FROZEN BYTES. I4 gives
//! up versioning, and what stands in its place is the build stamp of §6.1.2 -- which this
//! milestone deliberately does NOT build (§3.4). Until it exists, NOTHING REFUSES A STALE
//! PEER, and the trigger is the first real worker process (§0.2).
//!
//! ⛔ ONE DIRECTION ONLY, worker -> core, AND THE OTHER IS A DECLARED NON-CONSTRUCTION.
//! Nothing written imposes a core -> worker message today: `instruct_one` and
//! `instruct_stream` take an opaque `Frame` and no production caller exists. §6.10.4 imposes
//! exactly these two -- it measures the annotation ON AN AUDIO FRAGMENT and names the VRAM
//! peak as the field this channel puts into the journal. Inventing a downward vocabulary now
//! would freeze it against an imaginary consumer -- gotcha #46 from the wrong side, the same
//! reason §3.4 gives for the stamp. The trigger is the same one.
```

Il tipo, con le due varianti di **D9**:

```rust
/// What a worker sends up.
///
/// ⛔ EVERY BYTE THAT RISES IS COVERED BY A RECEIPT (§6.10.1). This enum says what is INSIDE
/// a frame; it never says that a frame may arrive unsolicited -- that one is a FAULT, and the
/// port already has the word for it, `ProcessError::UnsolicitedFrame`.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum FromWorker {
    /// One fragment of an instructed answer -- an audio chunk, a piece of a stream.
    ///
    /// ⛔ THE BYTE-STRING ANNOTATION IS LOAD-BEARING, not decoration, and the same sentence
    /// sits on `RecordV1::payload`. Without it `minicbor` writes AN ARRAY OF NUMBERS:
    /// measured in §6.10.4 on a 4096 B audio fragment, 7813 bytes against 4101, 1.91x. It
    /// compiles, it round-trips, and it is correct -- it costs double the traffic in silence,
    /// which is why the probe that holds it asserts a SIZE and not the attribute.
    #[n(0)]
    Fragment(#[cbor(n(0), with = "minicbor::bytes")] Vec<u8>),

    /// The VRAM peak the work actually reached (§5.2.2).
    ///
    /// ⚠️ IT IS THE ONE FIELD THIS CHANNEL PUTS INTO THE JOURNAL, and there it is subject to
    /// §4.9 -- optional, new index. Here it is not: this schema has no version enum at all.
    #[n(1)]
    VramPeak(#[n(0)] Mib),
}
```

E le due funzioni, che **passano dalla busta**:

```rust
impl FromWorker {
    /// Encodes the message and wraps it in its envelope.
    pub fn encode(&self) -> Result<Vec<u8>, WireError> {
        let mut body = Vec::new();
        let _ = minicbor::encode(self, &mut body);
        framing::frame(&body)
    }

    /// Reads a message out of an envelope.
    ///
    /// ⛔ TWO CHECKS AND NOT ONE, and they catch different faults: `unframe` catches a frame
    /// whose length does not match, `position() != body.len()` catches a body that carries a
    /// complete element AND SOMETHING AFTER IT. A CBOR decoder stops at the first complete
    /// element; the second check is the line `Record::decode` already carries, and the reason
    /// is written there -- finding AUD-047.
    pub fn decode(bytes: &[u8]) -> Result<Self, WireError> {
        let body = framing::unframe(bytes)?;
        let mut decoder = minicbor::Decoder::new(body);
        let message = decoder.decode().map_err(|_| WireError::Malformed)?;
        if decoder.position() != body.len() {
            return Err(WireError::Malformed);
        }
        Ok(message)
    }
}
```

⚠️ **`let _ = minicbor::encode(...)` non è pigrizia**, ed è la stessa riga di `Record::encode`:
il doc di quel metodo spiega perché l'errore è irraggiungibile scrivendo su una `Vec<u8>`.
Rileggilo invece di ricopiarne la ragione qui.

- [ ] **Passo 5: `WireError` guadagna `Malformed` (D11)**

In `crates/kernel/src/framing.rs`. ⛔ **Adesso e non al 3a:** ora ha un produttore.

```rust
    /// The body did not decode as a message of this channel, or it carried a complete
    /// element followed by something else. ⚠️ PRODUCED BY THE SCHEMAS, NOT BY THIS MODULE:
    /// the envelope knows how many bytes there are, never what they mean.
    Malformed,
```

- [ ] **Passo 6: lancia i due banchi**

```bash
cargo test --locked -p kernel --test worker_wire --test framing 2>&1 | tail -8
```

Atteso: `6 passed` e `5 passed`. ⛔ **Scrivi nel commento della terza sonda il numero vero che
hai misurato**, con la data — la sonda resta un limite, il numero misurato è un fatto.

- [ ] **Passo 7: le mutazioni di controllo, e sono TRE**

| Mutazione | Deve uccidere |
|---|---|
| togli `#[cbor(n(0), with = "minicbor::bytes")]` dal `Fragment` | `the_byte_string_annotation_is_measured_and_not_asserted`, **e nient'altro** |
| togli il controllo `decoder.position() != body.len()` | `junk_inside_the_declared_length_does_not_decode`, **e non** `a_frame_with_a_tail_does_not_decode` |
| in `encode`, sostituisci `framing::frame(&body)` con `Ok(body)` | `a_frame_with_a_tail_does_not_decode` e i due round-trip |

⛔ **La seconda riga è l'oracolo che conta, e la sua colonna «e non» è metà dell'asserzione:**
se togliendo `position()` cadessero **entrambe**, le due sonde non starebbero provando due
guasti ma uno solo, e uno dei due controlli sarebbe dominato — è il gotcha **#45** applicato
alle mutazioni, e la §3.2 del disegno afferma proprio che i due guasti hanno **prenditori
diversi**. Se cadono entrambe, **fermati e scrivi una voce d'errata**: a essere sbagliata è la
tabella della §3.2, non la tua sonda.

- [ ] **Passo 8: il richiamo datato su `ports/process.rs` (P-7)**

Il paragrafo *«What milestone 2 builds, and what it does not»* dice **`NOT the wire format`**, e
da questo commit è falso. ⛔ **Si riscrive col proprio richiamo, non si accorcia:** è la forma
che il gotcha **#76** dichiara e il suo limite — a cambiare è il **fatto**. Nomina il file che
ora lo porta, `crate::wire::worker`, e lascia in piedi le due clausole che restano vere.

- [ ] **Passo 9: il registro, e il conteggio si RICONTA**

In [`porta-di-qualita.md`](../../porta-di-qualita.md):
- la riga *«i **byte consumati** pari alla lunghezza dichiarata dal frame — non esiste ancora il
  canale verso i worker. Traguardo 6»* di *«Cosa la porta NON controlla»* esce dalle scoperte,
  **col richiamo datato** e non cancellata;
- la riga di catalogo `Q4 · I5 · §6.10` del livello 2 guadagna la propria cella, con le due
  direzioni e la mutazione che le tiene.

⛔ **Il numeratore del livello 2 si riconta SUL CATALOGO e non per sottrazione**, delimitando per
intestazione — è la forma che il registro usa già e che il gotcha **#26** prescrive:

```bash
awk '/^#### 7\.4\.2/{f=1} /^#### 7\.4\.3/{f=0} f' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md | grep -c '^| '
```

⚠️ **E il conto deve tornare**: coperte + parziali + scoperte = il totale che il comando dà meno
l'intestazione. Se non torna, **vince il conteggio** e la divergenza è una voce d'errata.

- [ ] **Passo 10: il cancello, `fmt`, e il cancello dei documenti**

```bash
cargo fmt --all --check
bash scripts/gate.sh
bash scripts/check-docs.sh
```

- [ ] **Passo 11: commit**

```bash
git add -A && git commit -m "traguardo 6 (compito 3b): lo schema del canale worker, e l'annotazione di stringa di byte e' tenuta da una MISURA e non dall'attributo -- la direzione core to worker resta una non-costruzione dichiarata"
```

#### Criterio di chiusura del compito 3

- [ ] `GATE GREEN` a entrambi i commit
- [ ] il **vincolo 15** della §11 è onorato in tutte e tre le clausole — lunghezza dichiarata,
      byte consumati verificati, annotazione **sul canale worker** (condizione 6)
- [ ] la riga di catalogo `Q4 · I5 · §6.10` è chiusa **nelle due direzioni**, e il registro la
      porta col conteggio **ricontato**
- [ ] ⛔ la clausola falsa di **P-5** non sopravvive: la riga del vincolo 15 nella §11 del
      [compendio](../../COMPENDIO.md) esce dalla tabella *«cosa resta davanti»*, e non ci si
      limita a spostarla — una riga che sparisce non è una riga corretta
- [ ] la **non-costruzione** della direzione core → worker porta il **proprio innesco**
      (condizione 9), scritto accanto al codice e non solo qui
- [ ] il paragrafo di `ports/process.rs` porta il **richiamo datato** (P-7)
- [ ] nessun byte congelato è nato: §6.10.3 lo vieta, e il banco `frozen_bytes` ha ancora **gli
      stessi file** — `git status --porcelain crates/kernel/tests/frozen/` è **vuoto**

### Compito 3bis: la misura C-1, e la decisione su §6.1.1

**Files:**
- Modify: `crates/kernel/Cargo.toml` — il richiamo datato sulla nota C-1
- Modify: [`riferimenti.md`](../../riferimenti.md) — le fonti, con la data (**D13**)
- Modify: [`porta-di-qualita.md`](../../porta-di-qualita.md) — la voce, col chiusore
- Modify (**solo nel caso B**): `Cargo.lock`

⛔ **Questo compito è una RICERCA, e il suo prodotto è una misura datata.** Non scrive codice.
La §3.5 del disegno dice alla lettera che deciderlo a memoria sarebbe il gotcha **#48**.

⛔ **E c'è un argomento che si rifiuta per nome.** La §8 del [compendio](../../COMPENDIO.md)
vieta di *«riaprire §6.1.1 tanto ora c'è `minicbor` nel kernel»*: fu **tentato il 2026-08-08 e
la misura diede torto** — i due canali hanno **pari diversi**. Se durante questo compito ti
viene in mente quell'argomento, è **già stato scartato**, e riaprirlo pretende una misura nuova
sul **pari**, non una simmetria. L'unico argomento vivo è **C-1**, che è di specie diversa:
riguarda la libreria **dalla nostra parte**.

- [ ] **Passo 1: rileggi il finding contro il codice di ADESSO**

È il passo 1 della disciplina dell'audit. La nota C-1 vive nel manifesto, non in una tabella:

```bash
grep -n "C-1" -A20 crates/kernel/Cargo.toml
grep -rn "bincode" crates/kernel/src/ crates/kernel/tests/
```

⚠️ **La nota afferma *«ZERO production uses»*: verificala, non citarla.** Se il secondo comando
mostra un uso di produzione, la finestra che la nota dice aperta **si è già chiusa**, e questo
compito cambia di specie — scrivi una voce d'errata prima di proseguire.

- [ ] **Passo 2: la misura, oggi, da fonti primarie**

Due domande, e sono **due**: ADR-0037 chiede del **pari**, C-1 chiede di **noi**.

| | Domanda | Che cosa la risponde |
|---|---|---|
| **A** | `bincode` è **ancora** dichiarato non mantenuto? | l'advisory **RUSTSEC-2025-0141** com'è oggi, il repository upstream (ultimo commit, ultima release), la pagina della crate |
| **B** | esiste un'alternativa **mantenuta** il cui pari **TypeScript** abbia un lettore conforme? | la crate candidata **e** il suo lettore TypeScript, ciascuno con la propria evidenza di manutenzione |

⛔ **La B non si risponde con una lista di nomi.** È la forma di **M-11**: un candidato conta
solo se **il pari lo legge**, e la §8 del compendio vieta di rifare M-1…M-11 — ma questa è una
misura **nuova**, non una da rifare. ⚠️ **E il lettore TypeScript ha un precedente scomodo che va
riletto prima di fidarsi di una promessa di pacchetto:** `bincode-ts` 1.0.0 fu misurato con
**entrambi i punti d'ingresso pubblicati rotti su Node 24**, e funzionò dietro un bundler. La
fragilità è dichiarata in §6.10.6 della spec: un pacchetto che *esiste* non è un pacchetto che
*legge*.

⚠️ **Novità non è maturità**, e il criterio non è «l'ultima uscita»: è ciò che oggi è **corrente
e mantenuto**.

- [ ] **Passo 3: traccia le fonti in `riferimenti.md`, con la data**

⛔ **D13, e non è la convenzione nuova di `E146`:** le **misure interne** restano in
[`porta-di-qualita.md`](../../porta-di-qualita.md), le **fonti esterne** vanno qui — è ciò che
la §12 del compendio chiama *«la provenienza di ciò che non abbiamo dedotto noi, con le date»*.
Ogni riga porta **l'indirizzo**, la **data di consultazione** e **che cosa dice**, non una
parafrasi.

- [ ] **Passo 4: il verdetto, e dove si ferma**

| Caso | La misura dice | Che cosa fa questo compito |
|---|---|---|
| **A** | `bincode` resta l'unica via — nessuna alternativa mantenuta col lettore del pari, oppure l'advisory è caduto | **decide**: §6.1.1 resta com'è. Il manifesto riceve il **richiamo datato** con la misura, la voce entra nel registro col proprio chiusore, e il compito 4 procede |
| **B** | esiste un'alternativa mantenuta il cui pari TypeScript ha un lettore conforme | ⛔ **SI FERMA PRIMA DI DECIDERE.** §6.1.1 è **spec**: riaprirla è del proprietario, vincolo globale 7. Scrivi la misura, le **due** opzioni coi costi rimisurati, e portala |

⛔ **La differenza fra i due casi non è la gravità, è che cosa toccano:** il caso A **non tocca
nessuna sezione**, quindi decidere è dovuto; il caso B ne tocca una approvata. È la forma di
**AUD-004**, **AUD-036** e **AUD-044**, tutti fermatisi lì. ⚠️ **Fermarsi non è rimandare:** la
misura è **fatta** ed è il prodotto del compito; ciò che manca è la scelta, che non è
dell'agente.

- [ ] **Passo 5: il caso B, se e solo se il proprietario decide un cambio**

⛔ **Una dipendenza si aggiunge in DUE passi**, e il cancello passa `--locked` a **tutti** i suoi
siti `cargo` — quindi il `Cargo.lock` è un **ingresso**:

```bash
# fuori dal cancello, SENZA --locked
cargo build --workspace
# poi manifesto e lockfile insieme, in UN commit
git add crates/kernel/Cargo.toml Cargo.lock
```

Toccare il manifesto da solo lascia il cancello **rosso**. È il finding **G-5**, ed è il punto e
non il prezzo: ADR-0031 chiama l'aggiunta di una voce *«un atto deliberato e rivedibile»*.
⚠️ **E una voce nuova nella lista di ADR-0031 pretende la propria giustificazione scritta
accanto**, più la riga corrispondente in `scripts/gate-deps.sh` sul grafo **transitivo** — la
voce vecchia esce solo se nessuno la usa più.

- [ ] **Passo 6: il cancello e il cancello dei documenti**

```bash
bash scripts/gate.sh
bash scripts/check-docs.sh
```

- [ ] **Passo 7: commit**

```bash
git add -A && git commit -m "traguardo 6 (compito 3bis): la misura C-1 e' rifatta oggi invece che ricordata -- e la decisione su 6.1.1 e' quella che la misura sostiene, non quella dell'omissione"
```

#### Criterio di chiusura del compito 3bis

- [ ] la condizione **7** della Definizione di «fatto» è soddisfatta: la decisione C-1 è presa
      con una **misura odierna**
- [ ] le fonti stanno in [`riferimenti.md`](../../riferimenti.md) **con la data di
      consultazione**, e non sono parafrasate
- [ ] la nota C-1 di `crates/kernel/Cargo.toml` porta il **richiamo datato** — ⛔ **anche nel
      caso A**: una nota che dice *«decide at milestone 6»* è falsa il giorno dopo che il
      traguardo ha deciso
- [ ] la voce entra nella **tabella unica** delle voci aperte del traguardo (condizione 11) se
      resta aperta, o esce col commit che la chiude
- [ ] nel **caso B**: manifesto e `Cargo.lock` in **un** commit, e `bash scripts/gate.sh` verde
      **dopo** — non prima
- [ ] ⛔ nessuna riga di schema `ipc` è stata scritta: il compito **4** viene dopo, ed è la
      ragione per cui questo compito esiste

---

## Parte C — lo schema `ipc`

⛔ **Un compito solo, e la scelta va detta perché contraddice il precedente della D1.** Il
compito 1 si consegna in tre commit perché le sue tre parti hanno **raggi diversi**; qui il
cambiamento è **uno**, e la prosa che il codice rende falsa deve atterrare **nello stesso
commit** del fatto che la smentisce — è la lezione di **P-7**, e spezzarla in due commit la
rimetterebbe in piedi per la durata di uno.

### Compito 4: §6.1 — la busta porta due messaggi, nel formato che il 3bis ha deciso

**Files:**
- Create: `crates/kernel/src/wire/ipc.rs` — `IpcMessage`, `encode`, `decode`
- Modify: `crates/kernel/src/wire/mod.rs` — il `pub mod ipc;`
- Modify: `crates/kernel/src/arbiter/resource.rs` — i derive su `ComputeClass` e `Preemption`
- Modify: `crates/kernel/src/time.rs` — il derive su `Millis`
- Modify: `crates/kernel/src/ports/ipc.rs` — **due** richiami datati, e nient'altro
- Test: `crates/kernel/tests/ipc_wire.rs` — **creato**
- Modify: [`porta-di-qualita.md`](../../porta-di-qualita.md)

⛔ **Leggi P-8, P-9 e P-10 prima di cominciare.** Due celle della §6.2 del disegno **non sono
implementabili alla lettera**, e la ragione non è un fastidio di tipi: metterle sul filo
conierebbe concessioni dai byte e farebbe entrare testo non fidato in un tipo di decisione.

- [ ] **Passo 0: leggi il verdetto del compito 3bis, e se non c'è FERMATI**

⛔ **Il formato non si decide qui.** Scrivere questo schema in `bincode` senza che il 3bis sia
girato **è** la decisione C-1, presa per omissione — D4, e la §3.5 del disegno lo dice alla
lettera. Se il 3bis non è stato eseguito, questo compito non è dispacciabile.

```bash
grep -n "C-1" -A6 crates/kernel/Cargo.toml     # deve portare il richiamo datato del 3bis
```

📌 **Da qui in poi il compito ha DUE rami, e cambiano tre righe in tutto:** i derive sul tipo, la
chiamata che codifica e quella che decodifica. Tutto il resto — la busta, l'enumerazione, le
sonde, le mutazioni — è **identico**, ed è il motivo per cui questo compito si è potuto scrivere
prima che il 3bis girasse.

| | `bincode` (il ramo che il manifesto ha oggi) | `minicbor` (se il 3bis lo ha scelto) |
|---|---|---|
| derive | `#[derive(bincode::Encode, bincode::Decode)]` — ⛔ **da VERIFICARE, D18** | `#[derive(minicbor::Encode, minicbor::Decode)]` con gli indici, come `record.rs` |
| codifica | `bincode::encode_to_vec(self, bincode::config::standard())` ✅ **verificata in repo** | `minicbor::encode(self, &mut body)` ✅ verificata |
| byte consumati | il `usize` che `decode_from_slice` **restituisce già** ✅ verificata | `decoder.position()`, come `Record::decode` ✅ verificata |

- [ ] **Passo 1: verifica il derive del formato, non ricordarlo (D18)**

Nessuna riga del workspace usa il derive di `bincode`: `dependencies_usable.rs` prova le due
**funzioni** su un `u32`, non gli attributi. Scrivi una **sonda usa-e-getta**, compilala,
leggi l'esito, **cancellala nella stessa corsa** — è la forma che questo repository usa per
P-2 e per il quinto caso `compile_fail`. In alternativa, leggi la sorgente vendorizzata:

```bash
ls -d ~/.cargo/registry/src/*/bincode-2.0.1
```

⚠️ **Ciò che va verificato, e non è solo «il derive esiste»:** che si applichi a un **enum**,
quale sia il **percorso** dell'attributo, e se il tratto porti parametri generici che la firma
di `decode` deve nominare. Un derive che compila su una struct e non su un enum è esattamente il
genere di cosa che un ricordo non distingue.

- [ ] **Passo 2: scrivi le sonde che falliscono**

In `crates/kernel/tests/ipc_wire.rs`, file nuovo:

```rust
//! The schema of the `ipc` channel. ⛔ OUTSIDE THE CRATE, like `framing` and `worker_wire`.

use kernel::arbiter::{ComputeClass, Mib, Preemption};
use kernel::framing::WireError;
use kernel::time::Millis;
use kernel::wire::ipc::{GrantRequest, IpcMessage, Verdict};

fn a_request() -> GrantRequest {
    GrantRequest {
        reserved_vram: Mib::new(2048),
        compute_class: ComputeClass::Interactive,
        preemption: Preemption::After(Millis::new(500)),
    }
}

#[test]
fn a_grant_request_survives_the_round_trip() {
    let message = IpcMessage::Request(a_request());
    let bytes = message.encode().expect("encode");
    assert_eq!(IpcMessage::decode(&bytes), Ok(message));
}

#[test]
fn a_verdict_survives_the_round_trip() {
    // ⛔ THIS IS THE PROBE THAT EXERCISES THE DISCRIMINANT, and it is why §6.7 asks for TWO
    // messages rather than one: with a single message type the tag never varies, and a bug in
    // how it is written or read would be invisible. Same shape as the journal freezing THREE
    // records instead of one.
    let message = IpcMessage::Verdict(Verdict::Refused {
        asked: Mib::new(4096),
        ceiling: Mib::new(1024),
    });
    let bytes = message.encode().expect("encode");
    assert_eq!(IpcMessage::decode(&bytes), Ok(message));
}

#[test]
fn a_message_with_a_tail_does_not_decode() {
    let mut bytes = IpcMessage::Verdict(Verdict::Granted).encode().expect("encode");
    bytes.push(0xFF);
    assert_eq!(IpcMessage::decode(&bytes), Err(WireError::TrailingBytes));
}

#[test]
fn junk_inside_the_declared_length_does_not_decode() {
    // ⛔ THE ENVELOPE IS HONEST HERE and the body is not -- the other half of the pair, and it
    // is a DIFFERENT check. See the same probe in `worker_wire.rs`.
    let good = IpcMessage::Verdict(Verdict::Granted).encode().expect("encode");
    let mut junked = good[4..].to_vec();
    junked.push(0xFF);
    let bytes = kernel::framing::frame(&junked).expect("frame");
    assert_eq!(IpcMessage::decode(&bytes), Err(WireError::Malformed));
}
```

✅ **I nomi di questa sonda sono VERIFICATI nel sorgente, non ricordati** — `ComputeClass` ha
`Realtime`, `Interactive`, `Batch`, e `Millis::new` esiste:

```bash
grep -n "pub enum ComputeClass" -A20 crates/kernel/src/arbiter/resource.rs
grep -n "impl Millis" -A8 crates/kernel/src/time.rs
```

⚠️ **Rilanciali comunque prima di scrivere:** è il passo 1 della disciplina dell'audit, e il
rimedio di **AUD-036** ha già tradotto `interattivo` in `interactive` in `design/02` — cioè
questa famiglia di nomi si è mossa una volta.

- [ ] **Passo 3: lancia il banco e verifica che NON COMPILI**

```bash
cargo test --locked -p kernel --test ipc_wire 2>&1 | head -20
```

Atteso: `error[E0432]` su `kernel::wire::ipc`.

- [ ] **Passo 4: i derive sui tipi condivisi (D17, P-10)**

Su `Mib`, `ComputeClass` e `Preemption` in `crates/kernel/src/arbiter/resource.rs`, e su
`Millis` in `crates/kernel/src/time.rs`. ⛔ **Accanto a ciascuno va scritto PERCHÉ ce l'ha**, e
la frase è la stessa: *questo tipo attraversa un canale privato*. Un derive senza una ragione
accanto è ciò che il Task 11 del Traguardo 5 ha passato una revisione intera a potare.

⚠️ **`Mib` può finire con DUE insiemi di attributi** — quelli di `minicbor` dal compito 3 e
quelli del formato di `ipc` — e non è un difetto: è il costo di ADR-0037, che ha scelto **due
formati misurati** per due pari diversi. ⛔ **Non «sanarlo»:** la §8 del compendio vieta di
riaprire §6.1.1 per simmetria, e questa sarebbe la stessa mossa travestita da pulizia.

- [ ] **Passo 5: scrivi `crates/kernel/src/wire/ipc.rs`**

Il doc di modulo, e le tre cose che deve dire:

```rust
//! The schema of the `ipc` channel: the envelope of `crate::framing` carrying ONE enum.
//!
//! ⛔ ONE ENUM FOR BOTH DIRECTIONS, AND THE DIRECTION IS DOCUMENTED RATHER THAN TYPED. Two
//! enums of one variant each would leave BOTH discriminants unexercised, which is the very
//! thing §6.7 asks two messages for. And typing the direction would buy nothing at the port:
//! `send` takes `&[u8]` and `receive` returns `Vec<u8>`, so the boundary sees no type at all.
//! ⚠️ THE COST, stated: nothing stops a caller from encoding a `Verdict` and sending it UP.
//! Today there is no such caller -- the transport is staged out (open item 5) -- and the day
//! there is one, the guard that pays for itself is on the composition side, not here.
//!
//! ⛔ THE SCHEMA MINTS NO IDENTIFIERS BECAUSE IT CARRIES NONE, and saying it that way is the
//! point (§6.5). Writing "§6.1.3 is satisfied" would be green having compared empty sets. A
//! grant request is not a step of a run: it writes no record and carries neither `StepId` nor
//! `RunId`. The first message that carries an identifier is where the rule becomes real, and
//! where its probe is born.
//!
//! ⛔ NO VERSION ENUM, NO RETIRED-INDEX REGISTER, NO FROZEN BYTES -- I4 renounces versioning
//! (§6.4). What stands in its place is the BUILD STAMP of §6.1.2, WHICH THIS MILESTONE DOES
//! NOT BUILD (§3.4). Until it exists, NOTHING REFUSES A STALE GUI, and today that costs
//! nothing because there is no gui to refuse -- `grep -rn "impl Ipc" crates/` returns a bench
//! fake. The trigger is milestone 2 of the subproject, the one that brings the shell.
//!
//! ⛔ AND THE REVOCATION core -> gui IS A DECLARED NON-CONSTRUCTION. ADR-0033 names it -- "the
//! gui stops rendering the 3D and says so" -- and it is the first message this vocabulary will
//! gain. It is not here because no written line demands it today and because §5.7 row 3 speaks
//! of a gui that DIES, not one that is asked. ⚠️ THE COST IS REAL: until then a discretionary
//! grant is preemptible IN THE BOOKS and the gui never hears about it. Open item 7.
```

E il tipo, con la forma della **D14**, della **D15** e della **D16**:

```rust
/// What the gui asks for: an ordinary grant beyond the presentation quota (ADR-0033).
///
/// ⛔ IT IS NOT A `ResourceProfile`, AND THE MISSING FIELD IS THE REASON. `ResourceProfile`
/// carries `name: &'static str`, which cannot be produced from arriving bytes without leaking
/// -- and what would be leaked is text CHOSEN BY THE GUI, i.e. untrusted content (ADR-0014)
/// inside a type the arbiter DECIDES with. The split here is the one ADR-0005 already
/// describes: THE REQUESTER DECLARES THE RESERVATION, and the core names the profile.
pub struct GrantRequest {
    pub reserved_vram: Mib,
    pub compute_class: ComputeClass,
    pub preemption: Preemption,
}

/// The three-way outcome, WITHOUT the grant.
///
/// ⛔ `Granted` IS A UNIT VARIANT AND CARRIES NO `Grant`, which is the whole of this type. A
/// decodable `Grant` would be a capability MINTED FROM BYTES: §5.6 holds that the only site
/// that mints one is `Arbiter::issue`, and `tests/compile_fail/grant_has_no_constructor.rs`
/// exists to make it unspeakable from outside. It would be AUD-050 done again on the
/// strongest token in the project -- a guard is worth exactly what its constructor is worth.
/// ⚠️ AND THE GUI DOES NOT NEED ONE: ADR-0033 says the grant is STATE OF THE CORE (I1). What
/// crosses is the verdict.
///
/// ⚠️ `Refused` CARRIES TWO NUMBERS AND `Queued` CARRIES NOTHING, and the asymmetry is
/// argued: design/02 wants "why it does not fit and the workable alternative", ADR-0020
/// forbids the kernel to suggest one, so THE INTERFACE BUILDS IT AND THE KERNEL HANDS OVER
/// THE MATERIAL -- the gui is the written consumer of those two. A ticket, by contrast, is
/// load-bearing only for a caller with TWO requests outstanding, and the gui has one.
pub enum Verdict {
    Granted,
    Queued,
    Refused { asked: Mib, ceiling: Mib },
}

/// One message on the `ipc` wire.
pub enum IpcMessage {
    /// gui -> core.
    Request(GrantRequest),
    /// core -> gui.
    Verdict(Verdict),
}
```

E le due funzioni, **identiche nella forma a quelle di `wire::worker`**: `encode` codifica e
poi chiama `framing::frame`; `decode` chiama `framing::unframe`, decodifica, e **verifica i byte
consumati** contro la lunghezza del corpo. ⛔ **Sono due controlli e non uno**, e il doc lo deve
dire: la busta prende il **troncamento**, il conteggio dei consumati prende la **coda dentro** la
lunghezza dichiarata.

- [ ] **Passo 6: lancia il banco e verifica che passi**

```bash
cargo test --locked -p kernel --test ipc_wire 2>&1 | tail -5
```

- [ ] **Passo 7: le mutazioni di controllo, e sono TRE**

⛔ **D7: una alla volta, compilando in un passo separato dall'eseguire, revocando da copia.**

| Mutazione | Deve uccidere |
|---|---|
| in `encode`, `framing::frame(&body)` → `Ok(body)` | `a_message_with_a_tail_does_not_decode` e i due round-trip |
| ⛔ l'encoder scrive **sempre** il discriminante della prima variante | `a_verdict_survives_the_round_trip`, **e NON** `a_grant_request_survives_the_round_trip` |
| togli il confronto fra byte consumati e lunghezza del corpo | `junk_inside_the_declared_length_does_not_decode`, **e NON** `a_message_with_a_tail_does_not_decode` |

⛔ **Le due colonne «e NON» sono metà dell'asserzione, e la seconda riga è l'oracolo di questo
compito:** è la sola prova che il corpo è davvero **un'enumerazione** e non un tipo solo travestito.
Se quella mutazione uccide **entrambi** i round-trip, il discriminante non sta distinguendo
niente — fermati e scrivi una voce d'errata.
⚠️ **Come si scrive la seconda mutazione dipende dal formato** e questo piano non la detta: con
un derive il tag lo scrive la macro, quindi la via è **scambiare i corpi delle due varianti**
nel punto in cui si costruiscono, oppure codificare a mano il tag sbagliato. Sceglila leggendo
il codice che hai scritto, e **scrivi quale hai usato**.

- [ ] **Passo 8: i due richiami datati su `crates/kernel/src/ports/ipc.rs` (P-7 nella sua forma qui)**

⛔ **Due, e sono di specie diversa.**

① Il doc di modulo dice: *«Milestone 6 brings the SCHEMA — **`bincode` in `kernel`** … and the
**BUILD STAMP** of §6.1.2»*. **Entrambe le metà cambiano**, e la seconda più della prima: il
formato è quello che il 3bis ha deciso, e **il timbro NON arriva** — §3.4 lo toglie dal
perimetro e ne fa una non-costruzione dichiarata. ⚠️ **È una scadenza in prosa**, gotcha **#77**,
e questa è la corsa in cui scade: nulla è mai diventato rosso per lei.

② La frase di `ClientId`: *«Whoever implements this port in milestone 6 draws from THAT
counter»*. ⛔ **Si RI-PUNTA, non si toglie**, e lo prescrive la §6.5 del disegno: invecchia nel
**soggetto** e non nell'affermazione — gotcha **#87** — perché il Traguardo 6 **non implementa
la porta** (voce 5, il trasporto è scaglionato). Toglierla lascerebbe scoperto il difetto che
esiste per impedire: **due contatori identici che divergono senza che nulla lo segnali**.

- [ ] **Passo 9: il registro, e ciò che NON si scrive**

In [`porta-di-qualita.md`](../../porta-di-qualita.md): la sezione del compito, con le sonde, le
tre mutazioni e il **loro esito misurato**.

⛔ **E ciò che NON si scrive è la parte che va letta due volte (D19): questo compito non chiude
nessuna riga di catalogo.** Nessuna riga di §7.4 nomina §6.1. Non muovere nessun numeratore, e
**scrivi che non si muove** — la riga della campagna DST resta `PARZIALE` fino al compito 9, e
il gettone `Q13` fino al compito 6.

⚖️ **Una voce si registra e non si prende:** un caso `compile_fail` che tenga *«un verdetto non
può portare un `Grant`»* sarebbe una **riga di catalogo nuova**, cioè §7.4, cioè **spec** —
vincolo globale 7. Oggi quella proprietà è tenuta dal **fatto che `Verdict::Granted` è unitario**
e dal doc accanto; è **livello 1 per costruzione, non per un caso negativo**, ed è una
distinzione da scrivere invece che da lasciare intendere.

- [ ] **Passo 10: il cancello, `fmt`, e il cancello dei documenti**

```bash
cargo fmt --all --check
bash scripts/gate.sh
bash scripts/check-docs.sh
```

- [ ] **Passo 11: commit**

```bash
git add -A && git commit -m "traguardo 6 (compito 4): lo schema ipc -- il verdetto NON porta il gettone, e la richiesta non porta il nome del profilo: due celle della 6.2 non erano implementabili alla lettera"
```

#### Criterio di chiusura del compito 4

- [ ] `GATE GREEN`, e la baseline **rimisurata** col comando (D5)
- [ ] il corpo è **un enum con due varianti**, e la mutazione del discriminante ne uccide
      **una sola** — è la prova che §6.7 chiede
- [ ] ⛔ **nessun `Grant` è raggiungibile dal filo**: `grep -rn "Grant" crates/kernel/src/wire/`
      non deve restituire **niente** fuori dai commenti, e i commenti dicono **perché**
- [ ] ⛔ **nessun `&'static str` e nessuna stringa nello schema:** un campo di testo che arriva
      dalla gui è contenuto **non fidato**, e `Untrusted` è il solo tipo che può portarlo
      (ADR-0014). Se un messaggio futuro ne avrà bisogno, quello è il tipo, non `String`
- [ ] i **due richiami datati** di `ports/ipc.rs` ci sono, e il secondo **ri-punta** invece di
      togliere (§6.5)
- [ ] le **due** non-costruzioni — il **timbro** e la **revoca verso la gui** — portano ciascuna
      il proprio **innesco**, scritto accanto al codice (condizione 9)
- [ ] nessun byte congelato è nato: §6.4 lo vieta, e `git status --porcelain
      crates/kernel/tests/frozen/` è **vuoto**
- [ ] ⚠️ il registro **non** dichiara chiusa nessuna riga di catalogo (D19)

## Parte D — i meccanismi

⛔ **Tre dei quattro compiti toccano il formato durevole, e il campo `detail` si aggiunge UNA
VOLTA SOLA.** Il costo che **P-13** e **P-14** misurano — ventisei siti di costruzione, la
decisione sul `Debug` scritto a mano, i due `.stderr` da rileggere — è il costo del **campo**,
non della specie: lo paga **il primo compito che ne ha bisogno**, e i compiti 6 e 7 ereditano un
campo che c'è già e aggiungono soltanto la **propria** variante e il **proprio** record
congelato. È la **D24**.

⚠️ **Il compito 8 non tocca il formato**, e lo dichiara P-11: il degrado si **ricalcola**, non si
scrive.

### Compito 5: §6.4 — il contratto del sensore, il campo `detail`, e l'anello che giornala

**Files:**
- Create: `crates/kernel/src/sensor.rs` — `Sensor`, `Verdict`, `VerdictOutcome`, `CostClass`, `VerdictDetail`
- Modify: `crates/kernel/src/lib.rs` — il `pub mod sensor;`
- Modify: `crates/kernel/src/record.rs` — `Detail` all'indice **5**, `RecordKind::Verdict`, e il `Debug` scritto a mano
- Modify: `crates/kernel/src/reconcile.rs` — l'arm della variante nuova (**D23**)
- Modify: `crates/kernel/tests/frozen_bytes.rs` — l'array a mano, e il **quarto** record congelato
- Create: `crates/kernel/tests/frozen/record_v1_verdict.cbor` — ⛔ **irreversibile**
- Modify: `crates/kernel/tests/frozen/record_v1.map` — la sezione del quarto record
- Modify: `crates/kernel/tests/record_shape.rs` — i literal, e l'oracolo del `Debug`
- Modify: `crates/kernel/tests/reconciliation.rs`, `crates/kernel/tests/boundary_promotion.rs`, `crates/simulator/tests/dst_campaign.rs`, `crates/kernel/src/boundary.rs`, `crates/kernel/src/arbiter/mod.rs` — i literal
- Modify: `crates/kernel/tests/compile_fail/record_without_version.rs` + `.stderr`, `crates/kernel/tests/compile_fail/record_without_trust_label.rs` — **P-14**
- Create: `crates/kernel/tests/compile_fail/sensor_modifies_the_artefact.rs` + `.stderr` — il **34°** caso, `V10`
- Create: `crates/kernel/tests/sensor_ring.rs` — le sonde di `V14` e `Q10`
- Modify: [`porta-di-qualita.md`](../../porta-di-qualita.md)

✅ **E un file che NON si tocca, verificato invece che supposto:** `crates/kernel/tests/compile_fail.rs` raccoglie i casi con un **glob** — `t.compile_fail("tests/compile_fail/*.rs")` — quindi un caso nuovo entra da solo e non c'è nessun elenco da allungare. ⚠️ **La guardia di non-vacuità di quel file conta i `.rs` della cartella e pretende che siano più di zero**, senza numero atteso: un conteggio fisso *«diventerebbe rosso il giorno in cui il banco cresce per una ragione legittima»*, e questo compito lo fa crescere.

⛔ **Leggi P-12, P-13, P-14, P-15 e la D22 prima di cominciare**, e la **D24** qui sopra. Questo
compito tocca l'**unico artefatto irreversibile** del progetto: i byte congelati **non si
rigenerano**, e se cambiano non è un aggiornamento ma un **cambio di formato** (ADR-0036).

⛔ **E il numeratore «34°» si RICONTA, non si cita:** era 33 il 2026-08-30 e le Parti A, B e C
non ne aggiungono nessuno — misurato — ma questo compito potrebbe non essere il primo a girare.

```bash
ls crates/kernel/tests/compile_fail/*.rs | wc -l
```

#### Commit 5a — il contratto, e il 34° caso `compile_fail` (`V10`)

⛔ **Nessuna riga di formato in questo commit.** Il contratto è additivo puro: un modulo nuovo,
un caso `compile_fail` nuovo, e nient'altro. Il commit deve essere `GATE GREEN` **da solo**
(vincolo globale 8), e separarlo dal formato è ciò che rende leggibile quale dei due ha rotto
cosa (precedente della **D1**).

- [ ] **Passo 1: scrivi il caso `compile_fail`, e MISURA quale errore esce**

⛔ **Le vie sono DUE e non è ovvio quale delle due la riga di catalogo compri.** La riga
§7.4.1 blocco C dice *«un sensore che **modifica** l'artefatto — §6.4.2 lo consegna per
riferimento immutabile»*, con contro-sonda *«osservarlo e restituire un verdetto compila»*.

| Via | Che cosa prova | Errore atteso |
|---|---|---|
| **corpo** — un `Sensor` ben firmato il cui `observe` assegna a `*artefact` | che il sensore **non può modificare ciò che gli è stato consegnato** | `error[E0594]` — *cannot assign to `*artefact`, which is behind a `&` reference* |
| **firma** — un `impl Sensor` che dichiara `observe(&self, artefact: &mut Untrusted)` | che un sensore che **vuole** modificare non è **dichiarabile** | `error[E0053]` — *method has an incompatible type for trait* |

⛔ **Si scrive la via del CORPO**, perché è quella che la riga di catalogo formula — *«lo
consegna per riferimento immutabile»* parla della **consegna**, non della dichiarazione — e la
via della firma si registra nel commento del caso come **seconda strada che lo stesso meccanismo
chiude**. ⚠️ **Misura entrambe prima di scegliere il testo atteso**, invece di dedurre il codice
d'errore: è la stessa disciplina che il Task 10 del Traguardo 3 applicò alle due mutazioni
dettate, che **non compilavano** per una ragione diversa da quella prevista.

`crates/kernel/tests/compile_fail/sensor_modifies_the_artefact.rs`:

```rust
//! Catalogue §7.4.1 block C, row `V10` — a sensor that MODIFIES the artefact does not compile.
//! §6.4.2 hands it over by immutable reference, so the trait method cannot reach through it.

use kernel::boundary::Untrusted;
use kernel::sensor::{CostClass, Sensor, Verdict, VerdictOutcome};
use kernel::time::Millis;

struct MeddlingSensor;

impl Sensor for MeddlingSensor {
    fn declared_cost(&self) -> CostClass {
        CostClass::Computational
    }

    fn observe(&self, artefact: &Untrusted) -> Verdict {
        // A sensor that rewrites what it was handed. This is the whole case.
        *artefact = Untrusted::new("something else".into());

        Verdict {
            outcome: VerdictOutcome::Pass,
            detail: Untrusted::new(String::new()),
            spent: Millis::new(0),
        }
    }
}

fn main() {}

// ⛔ THE COUNTER-PROBE IS NOT HERE, AND IT ALREADY HAS A HOME: the catalogue row's "observing it
// and returning a verdict compiles" is `a_passing_sensor_writes_a_verdict_and_opens_nothing` in
// `tests/sensor_ring.rs`, which implements the trait and runs it. A copy here would be gotcha
// #49.
//
// ⛔ THE SECOND ROAD THE SAME MECHANISM SHUTS, measured rather than assumed: an `impl` that
// DECLARES `observe(&self, artefact: &mut Untrusted)` does not match the trait and fails with
// `error[E0053]`. It is not the case written here because the catalogue row is about the
// HANDING OVER, not about what an implementor may declare — but whoever widens this row should
// know the road exists and is already shut.
//
// ⛔ Names `kernel::` and declares no attributes of its own — gotcha #39.
//
// ⛔ THE NOTE IS DOWN HERE ON PURPOSE: the oracle quotes the line of the assignment, so a
// paragraph added at the top would move the code and break it. Whoever writes here appends.
```

- [ ] **Passo 2: fai girare il caso e leggi il `.stderr`, senza rigenerarlo in blocco**

```bash
cargo test --locked -p kernel --test compile_fail 2>&1 | tail -40
```

⛔ **Aspettati un ROSSO che dice «expected compilation to fail» oppure che il modulo `sensor`
non esiste** — il tratto non è ancora scritto. È il rosso di partenza, e va **riprodotto e
letto** prima di scrivere l'implementazione: è il passo 2 della disciplina dell'audit.

⚠️ **Il `.stderr` si scrive a mano dall'uscita vera.** ⛔ **Mai** `TRYBUILD=overwrite`: vincolo
10 della §11, gotcha **#25**.

- [ ] **Passo 3: scrivi il contratto**

`crates/kernel/src/sensor.rs`, file nuovo:

```rust
//! The sensor contract (§6.4, ADR-0009), and the ring that carries a verdict back.
//!
//! ⛔ THE CONTRACT IS DELIBERATELY POOR, and that is a decision and not an omission: ADR-0009
//! writes it `(artefact) -> (verdict, detail, cost)` and says a minimal contract can be widened
//! while a rich and wrong one cannot. RK-5 is already accepted: it gets revisited after the
//! SECOND real sensor in different areas, and if it does not stretch it BREAKS rather than
//! bends.

use alloc::vec::Vec;

use crate::boundary::Untrusted;
use crate::time::Millis;

/// What a sensor costs BEFORE it runs (§6.4.1). ⛔ IT IS A CLASS AND NOT A NUMBER, and the
/// reason is what V11 actually asks: "inferential sensors stay OUT of the tight ring". That is
/// a partition, not a threshold — a number would invite a cutoff nobody has chosen.
///
/// ⚠️ THE SECOND VARIANT HAS NO IMPLEMENTOR TODAY, and that is written rather than hidden:
/// §8.3 keeps V11 at `parziale` precisely because no inferential sensor exists (trigger C4).
/// What exists here is the MECHANISM that will admit or refuse one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CostClass {
    /// Runs on the CPU and answers from the artefact alone — schema validation, a linter, a
    /// test. Admitted to the tight ring.
    Computational,
    /// Calls a model. ⛔ REFUSED BY THE TIGHT RING (V11), and the refusal is the point: an
    /// inferential sensor in the tight loop turns every step into two inferences.
    Inferential,
}

/// Whether the artefact passed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerdictOutcome {
    Pass,
    /// ⛔ A NEGATIVE VERDICT IS NOT AN ERROR (ADR-0013): it is an ordinary answer that re-enters
    /// the ring as feedback, and the correction is A NEW STEP. Modelling it as an `Err` would
    /// make schema non-conformance an exception, which is the thing ADR-0013 exists to refuse.
    Fail,
}

/// What a sensor answers. The triple of ADR-0009, with the two costs of §6.4.1 kept apart:
/// the DECLARED one is on the trait and is read before running, the SPENT one is here.
///
/// ⛔ `detail` IS `Untrusted` AND THAT IS FORCED, not defensive. ADR-0014 makes the label
/// HEREDITARY — "extracting, summarising, translating or concatenating still produces untrusted
/// content" — and a detail is computed FROM the artefact. For an inferential sensor it is model
/// output outright. So it travels in the record's `payload`, which is the box whose doc says it
/// holds "somebody else's" bytes, and never in `reason`.
///
/// ⚠️ WHAT THE SPENT COST IS AND IS NOT: it is what the SENSOR reports, not what the ring
/// measured. A sensor that lies about it is not caught here — sensors are kernel-side code and
/// the content comes from capacities (ADR-0009). The limit is declared rather than defended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
    pub outcome: VerdictOutcome,
    pub detail: Untrusted,
    pub spent: Millis,
}

/// The contract. ⛔ `observe` TAKES THE ARTEFACT BY SHARED REFERENCE AND THAT IS `V10` — a
/// sensor observes and changes nothing; correcting is ring 1's job (§6.4.2). The negative case
/// is `tests/compile_fail/sensor_modifies_the_artefact.rs`.
pub trait Sensor {
    /// Read BEFORE running, and it decides admission to the tight ring (§6.4.1, V11).
    fn declared_cost(&self) -> CostClass;

    /// Observe, and answer. ⛔ THE ARTEFACT IS `&`, NEVER `&mut`.
    fn observe(&self, artefact: &Untrusted) -> Verdict;
}
```

- [ ] **Passo 4: dichiara il modulo**

In `crates/kernel/src/lib.rs`, in coda alla lista dei `pub mod`, **nell'ordine in cui i moduli
sono già scritti** e senza toccare il paragrafo di testa:

```rust
pub mod sensor;
```

⚠️ **Il doc di testa di `lib.rs` NON si tocca:** dichiara di sé che *«what this crate holds is
the list of `pub mod` below»* proprio perché una prosa riassuntiva invecchia — è il rimedio di
**AUD-046**, e riaprirlo sarebbe rifare il difetto che quel finding ha chiuso.

- [ ] **Passo 5: rileggi il `.stderr` e chiudi il commit**

```bash
cargo test --locked -p kernel --test compile_fail 2>&1 | tail -20
bash scripts/gate.sh
cargo test --locked --workspace --no-fail-fast     # rimisura, non citare (D5)
```

- [ ] **Passo 6: commit**

```bash
git add crates/kernel/src/sensor.rs crates/kernel/src/lib.rs \
        crates/kernel/tests/compile_fail/sensor_modifies_the_artefact.rs \
        crates/kernel/tests/compile_fail/sensor_modifies_the_artefact.stderr
git commit -m "feat(sensor): il contratto di ADR-0009, e V10 diventa un caso compile_fail"
```

#### Commit 5b — il campo `detail`, la variante `Verdict`, e il quarto record congelato

⛔ **QUESTO COMMIT TOCCA L'ARTEFATTO IRREVERSIBILE.** Leggi **P-13**, **P-14** e **P-15** prima
del primo carattere, e la testa di `crates/kernel/tests/frozen_bytes.rs`.

- [ ] **Passo 1: prendi la copia byte-esatta dei file che toccherai (D7)**

⛔ **Prima di qualunque modifica**, e non `git checkout --` dopo: il gotcha **#48** è la trappola
più frequente del progetto, e il vicolo cieco dell'audit del 2026-08-27 mostra che una scrittura
che fallisce a metà **tronca**.

```bash
cp crates/kernel/src/record.rs /tmp/record.rs.orig
tr -cd '\r' < crates/kernel/src/record.rs | wc -c    # il conto dei CR PRIMA
```

- [ ] **Passo 2: aggiungi il tipo del dettaglio, e la specie del verdetto**

In `crates/kernel/src/record.rs`, **dopo** `RecordKind` e **prima** di `RecordV1`:

```rust
/// The structured detail a record of OUR OWN SPECIES carries (D20). ⛔ IT IS A TYPE AND NOT
/// OPAQUE BYTES, and the reason is ADR-0036 rule 6: the encoding lives in `kernel`. Opaque
/// bytes here would need a second decode nobody could perform without knowing the `kind` out of
/// band, which is the `payload` problem moved into a new box.
///
/// ⛔ AN UNKNOWN VARIANT DOES NOT DECODE, and that is what makes the field safe. Measured on
/// 2026-08-30 (P-15): `minicbor` answers `unknown enum variant N`, `Record::decode` maps every
/// error to `RecordError::Malformed`, and `reconcile` resolves that to `SuspendAndAsk`. A build
/// that does not know a species STOPS instead of guessing.
///
/// ⚠️ IT PAIRS WITH `RecordKind` AND NOTHING AT LEVEL 1 HOLDS THE PAIR — declared, not defended.
/// A `RecordV1` with `kind: Verdict` and a `Detail::Routing` is constructible, because `RecordV1`
/// is `pub` with public fields (finding AUD-050, registered and not taken). What holds the pair
/// is that ONE function per species builds the record, the way `Arbiter::issue` is the only
/// place that mints a `Grant` (§5.6). It is the shape of E25 in a new place, and it is written
/// down rather than discovered.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum Detail {
    /// A sensor verdict upon the step's artefact (§6.4).
    #[n(0)]
    Verdict(#[n(0)] VerdictDetail),
}

/// The structured half of a verdict (§6.4.1). ⛔ THE DETAIL TEXT IS NOT HERE: it is untrusted by
/// inheritance (ADR-0014) and travels in the record's `payload`, under the `trust` label that
/// exists to say so. What lives here is what is OURS and structured — the outcome, and the cost
/// the sensor reports having spent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cbor(array)]
pub struct VerdictDetail {
    /// `false` is `VerdictOutcome::Fail`. ⚠️ A `bool` AND NOT THE ENUM, and the asymmetry is
    /// deliberate: `sensor::VerdictOutcome` is a kernel type free to grow a third answer, while
    /// this one is on the WIRE and an index here never retires (rule 4 of §4.9.2). The day the
    /// enum grows, this field becomes a new optional index and the `bool` retires — which is
    /// exactly the discipline, and it is cheaper than reserving indices for answers nobody has.
    #[n(0)]
    pub passed: bool,
    #[n(1)]
    pub spent_millis: u64,
}
```

⚠️ **`Millis` NON entra qui**, ed è una scelta con un costo: portarlo sul filo gli darebbe i
derive del formato, cioè **P-10** in una casa nuova, e `Millis` è già un tipo che il compito 4
porta su `bincode`. Un `u64` col nome che dice l'unità è il minimo che regge; la conversione sta
nella funzione di costruzione, in un posto solo.

- [ ] **Passo 3: aggiungi la variante e il campo**

In `RecordKind`, **in coda**, con l'indice **3** — ⛔ **un indice non si riusa mai** (regola 4):

```rust
    /// ⛔ A SENSOR VERDICT UPON THE STEP'S ARTEFACT (§6.4). Like `Note` it neither opens a doubt
    /// nor closes one — the doubt of ADR-0007 is about EFFECTS reaching the world, and a verdict
    /// is a fact recorded ABOUT a step, not an effect of it. ⚠️ AND THE EMPTY ARM IN
    /// `reconcile` WAS RE-MEASURED FOR THIS VARIANT rather than inherited from `Note`'s: see
    /// the arm itself.
    #[n(3)]
    Verdict,
```

In `RecordV1`, **in coda**, all'indice **5**:

```rust
    /// ⛔ OUR OWN STRUCTURED DATA, AND THE THIRD CONTENT BOX (D20). `payload` is somebody
    /// else's and `reason` is our prose; this is our STRUCTURE, and putting it in either of the
    /// other two was measured to be wrong — putting CBOR in `payload` reopens the defect that
    /// splitting `reason` shut on 2026-08-10, and `reason` is text.
    ///
    /// ⛔ OPTIONAL, WITH `#[cbor(default)]`, AT A NEW INDEX — rule 3 of §4.9.2, and the exemption
    /// `reason` used is SPENT: `tests/frozen_bytes.rs` exists, so this is how every field added
    /// to V1 arrives from now on.
    ///
    /// ✅ ADDITIVE, MEASURED IN BOTH DIRECTIONS on 2026-08-30 (P-15): with `None` the three
    /// frozen records encode to the SAME 21 BYTES — `minicbor` truncates a trailing `None`
    /// instead of writing `null` — and with `Some` the array header moves `85` -> `86`.
    ///
    /// ⛔ AND THE FIELD ALONE IS NOT ENOUGH, WHICH IS WHY THE `kind` GROWS WITH IT (D20): a build
    /// that does not know this index decodes a record carrying it and LOSES THE SUBSTANCE IN
    /// SILENCE — measured. The new `kind` is what makes that build stop.
    #[n(5)]
    #[cbor(default)]
    pub detail: Option<Detail>,
```

- [ ] **Passo 4: decidi il `Debug`, e la decisione è già presa — SÌ, lo stampa**

⛔ **P-14 dice che le due vie sono entrambe difendibili e che nessuna riga le sceglie. La sceglie
questo piano, ed è la D25:** il `Debug` scritto a mano **stampa `detail`**, e l'oracolo di
`tests/record_shape.rs` va **rosso** e si riscrive a mano.

**Perché**, e sono i criteri in ordine: il campo porta byte **nostri** per costruzione, quindi
stamparlo non apre nessuna strada di §A3; e **non** stamparlo darebbe a `RecordV1` un secondo
campo nascosto **senza che nessuno l'abbia deciso**, contro la metà che il banco dichiara essere
*«la metà che ci si dimentica»* — *«a `Debug` that hid everything would pass the assertion above
and leave a failed `assert_eq!` on a record saying nothing at all»*.

```rust
            "RecordV1 {{ kind: {:?}, effect: {:?}, trust: {:?}, payload: <{} bytes>, \
             reason: {:?}, detail: {:?} }}",
            self.kind,
            self.effect,
            self.trust,
            self.payload.len(),
            self.reason,
            self.detail
```

⚠️ **E il costo si scrive accanto al tipo**, non solo qui: un dettaglio strutturato finisce in
ogni `{:?}` che raggiunge un log, e la garanzia che sia sempre **nostro** è di **disciplina** —
`RecordV1` è `pub` coi campi `pub` (AUD-050).

- [ ] **Passo 5: fai il giro dei siti di costruzione, e il censimento si RILANCIA**

⛔ **Non fidarti del numero di P-13:** rilancia il comando, e **leggi intera** ogni riga che
restituisce (gotcha **#70**).

```bash
grep -rn 'RecordV1 {' crates/ --include=*.rs
```

Ogni **literal** guadagna `detail: None`, tranne quelli che il passo 7 nomina. ⚠️ **E le righe
che NON sono literal non si toccano a caso:** i **sei commenti di doc** che mostrano un record a
cinque campi diventano falsi — si riscrivono col **richiamo datato**, che è la convenzione qui, e
non si allineano in silenzio.

- [ ] **Passo 6: i due casi `compile_fail`, uno per volta (P-14)**

| Caso | Che cosa fare |
|---|---|
| `record_without_trust_label.rs` | aggiungi `detail: None` al literal. ✅ Il `.stderr` cita la **riga 7**, l'apertura del literal, che **non si muove** — verificalo, non assumerlo. ⛔ Senza il campo l'errore diventa `missing fields «trust» and «detail»` e il caso smette di essere un caso sull'etichetta di fiducia |
| `record_without_version.rs` | aggiungi `detail: None`. ⛔ Il `.stderr` cita `inner.encode()` alla **riga 15**, **dentro** il literal: la riga si sposta a **16**, e il `.stderr` va corretto **a mano**. Il file dichiara già di sé che la stessa cosa successe il 2026-08-10 |

```bash
cargo test --locked -p kernel --test compile_fail 2>&1 | tail -40
```

⛔ **Mai `TRYBUILD=overwrite`.**

- [ ] **Passo 7: il QUARTO record congelato — irreversibile**

⛔ **Un record congelato nuovo per ogni variante nuova (D21), e porta ENTRAMBE le cose:** `kind:
Verdict` **e** `detail: Some(..)`. Con `detail: None` non pinzerebbe **niente** dell'indice 5,
perché un `None` in coda non viene scritto — misurato.

⚠️ **La fabbrica `record(kind, effect, trust)` di `frozen_bytes.rs` non basta più**, perché non
sa mettere un `detail`. ⛔ **Non aggiungere una seconda fabbrica:** il file dichiara *«ONE
constructor for the three frozen records … a second constructor would be a second place to keep
aligned»*. Si **allarga** quella che c'è, con `detail` fra i parametri, e i tre siti esistenti
passano `None`.

```rust
fn record(
    kind: RecordKind,
    effect: EffectClass,
    trust: Trust,
    detail: Option<Detail>,
) -> Record {
    Record::V1(RecordV1 {
        kind,
        effect,
        trust,
        payload: FROZEN_PAYLOAD.to_vec(),
        reason: String::from(FROZEN_REASON),
        detail,
    })
}
```

Il quarto record, accanto ai tre:

```rust
        (
            "record_v1_verdict.cbor",
            VERDICT_BYTES,
            record(
                RecordKind::Verdict,
                EffectClass::Verifiable,
                Trust::Untrusted,
                Some(Detail::Verdict(VerdictDetail {
                    passed: false,
                    spent_millis: 7,
                })),
            ),
        ),
```

⚠️ **`passed: false` e non `true`, e non è indifferente:** `false` codifica `f4` e `true` `f5`,
quindi il byte **esiste** in entrambi i casi — ma un verdetto **negativo** è quello che
l'anello fa rientrare, cioè il caso per cui la specie esiste. ⛔ **E `spent_millis: 7` non è
zero:** uno zero codifica `00`, che è anche l'indice di variante di mezza tabella, e un byte che
somiglia a troppe cose rende la mappa più difficile da leggere di quanto debba essere.

⛔ **L'array a mano va esteso, e questa è la riga che NIENTE fa diventare rossa** — P-12, e il
banco lo dichiara di sé:

```rust
    for kind in [
        RecordKind::Intent,
        RecordKind::Outcome,
        RecordKind::Note,
        RecordKind::Verdict,
    ] {
        match kind {
            RecordKind::Intent
            | RecordKind::Outcome
            | RecordKind::Note
            | RecordKind::Verdict => {}
        }
```

- [ ] **Passo 8: i byte si producono da una sonda usa-e-getta e si scrivono A MANO**

⛔ **Nessun percorso di rigenerazione**, come al Task 10 del Traguardo 3: una sonda usa-e-getta
stampa i byte, tu li scrivi nel `.cbor` e nella mappa, e la sonda si **cancella nella stessa
corsa**.

⚠️ **La mappa `record_v1.map` è RILETTA dal banco:** offset e byte di ogni riga devono
ricostruire il `.cbor` esattamente, quindi un segnaposto non può sopravvivere al commit
(gotcha **#43**). La colonna di prosa è dichiarata **non verificata** dentro la mappa stessa.

⚠️ **E la sezione nuova della mappa va scritta con l'inquadratura giusta:** `82 00 81` è la busta
di versione, poi `86` — l'array a **sei** elementi, non più `85` — e in coda la busta del
dettaglio. **Leggi l'uscita**, non dedurre gli offset.

- [ ] **Passo 9: l'arm di `reconcile`, e si RIMISURA invece di ereditare (D23)**

In `crates/kernel/src/reconcile.rs`, dentro il `match body.kind`:

```rust
                // ⛔ A VERDICT NEITHER OPENS A DOUBT NOR CLOSES ONE, and the empty arm was
                // MEASURED for this variant rather than inherited from `Note`'s. The doubt of
                // ADR-0007 is about an EFFECT that may or may not have reached the world; a
                // verdict is a fact recorded ABOUT a step's artefact, and the step it names
                // already owes its own outcome. Both other answers were tried: `enter` makes a
                // finished step re-open forever, `leave` closes a doubt no effect resolved —
                // which is the silent loss ADR-0007 exists to prevent.
                RecordKind::Verdict => {}
```

⚠️ **Provalo:** metti `enter` al posto dell'arm vuoto e guarda che cosa diventa rosso, poi
`leave`, poi revoca **da copia**. Se **nessuna** delle due muove niente, la sonda che dovrebbe
tenerlo manca — ed è un mutante vivo da dichiarare, non da ignorare (gotcha **#73**).

- [ ] **Passo 10: rimisura i fine-riga, e chiudi**

```bash
tr -cd '\r' < crates/kernel/src/record.rs | wc -c    # deve valere quanto il passo 1
git ls-files --eol crates/kernel/tests/frozen/record_v1_verdict.cbor
bash scripts/gate.sh
cargo test --locked --workspace --no-fail-fast
```

⛔ **Il `.cbor` è BINARIO:** verifica che `.gitattributes` non lo normalizzi. I tre esistenti
sono il precedente — se non hanno una riga, questo non ne ha bisogno; se ce l'hanno, questo la
prende uguale.

- [ ] **Passo 11: commit**

```bash
git add crates/kernel/src/record.rs crates/kernel/src/reconcile.rs \
        crates/kernel/tests/ crates/simulator/tests/ crates/kernel/src/boundary.rs \
        crates/kernel/src/arbiter/mod.rs
git commit -m "feat(record): il campo detail all'indice 5, la variante Verdict, e il quarto record congelato"
```

#### Commit 5c — l'anello che giornala (`V14`, `Q10`)

- [ ] **Passo 1: scrivi le sonde che falliscono**

`crates/kernel/tests/sensor_ring.rs`, file nuovo — ⛔ **da FUORI la crate**, che è la terza
domanda del pre-controllo di `CLAUDE.md`:

```rust
//! The ring: it collects a verdict, journals it, and a NEGATIVE one opens a new step (V14, Q10).

use kernel::boundary::Untrusted;
use kernel::ports::journal::{Journal, StepId};
use kernel::record::{Detail, Record, RecordKind};
use kernel::sensor::{run_the_ring, CostClass, Sensor, Verdict, VerdictOutcome};
use kernel::time::Millis;
use simulator::journal::MemoryJournal;

/// A sensor whose verdict the TEST chooses — §6.4.2 asks for exactly this double.
struct ScriptedSensor {
    cost: CostClass,
    outcome: VerdictOutcome,
}

impl Sensor for ScriptedSensor {
    fn declared_cost(&self) -> CostClass {
        self.cost
    }

    fn observe(&self, _artefact: &Untrusted) -> Verdict {
        Verdict {
            outcome: self.outcome,
            detail: Untrusted::new("field `name` is missing".into()),
            spent: Millis::new(7),
        }
    }
}

fn records(journal: &MemoryJournal) -> Vec<(StepId, Record)> {
    journal
        .replay()
        .expect("replay")
        .into_iter()
        .map(|(step, bytes)| (step, Record::decode(&bytes).expect("decode")))
        .collect()
}

#[test]
fn a_passing_sensor_writes_a_verdict_and_opens_nothing() {
    // ⛔ THIS IS ALSO THE COUNTER-PROBE OF CATALOGUE ROW V10 — "observing it and returning a
    // verdict compiles". It lives here and not beside the compile_fail case: gotcha #49.
    let mut journal = MemoryJournal::new();
    let sensor = ScriptedSensor {
        cost: CostClass::Computational,
        outcome: VerdictOutcome::Pass,
    };

    let opened = run_the_ring(
        &sensor,
        &Untrusted::new("the artefact".into()),
        StepId::new(1),
        StepId::new(2),
        &mut journal,
    )
    .expect("the ring");

    assert_eq!(opened, None);
    let written = records(&journal);
    assert_eq!(written.len(), 1);
    let (step, Record::V1(body)) = &written[0];
    assert_eq!(*step, StepId::new(1));
    assert_eq!(body.kind, RecordKind::Verdict);
}

#[test]
fn a_failing_verdict_opens_a_new_step_and_carries_the_detail() {
    // V14: "a negative verdict re-entering the ring is a NEW STEP, journalled" -- and Q10:
    // it re-enters WITHOUT HUMAN INTERVENTION, which is why nothing here asks anybody anything.
    let mut journal = MemoryJournal::new();
    let sensor = ScriptedSensor {
        cost: CostClass::Computational,
        outcome: VerdictOutcome::Fail,
    };

    let opened = run_the_ring(
        &sensor,
        &Untrusted::new("the artefact".into()),
        StepId::new(1),
        StepId::new(2),
        &mut journal,
    )
    .expect("the ring");

    assert_eq!(opened, Some(StepId::new(2)));

    let written = records(&journal);
    assert_eq!(written.len(), 2);

    // The verdict, upon the step that was judged.
    let (step, Record::V1(verdict)) = &written[0];
    assert_eq!(*step, StepId::new(1));
    assert_eq!(verdict.kind, RecordKind::Verdict);
    // ⛔ THE ASSERTION IS ON THE ARCHIVE, NOT ON THE RETURN VALUE, and that is what keeps it from
    // being vacuous -- the same choice task 9 of milestone 5 made for the policy transition.
    let Some(Detail::Verdict(detail)) = &verdict.detail else {
        panic!("the verdict record carries no structured detail");
    };
    assert!(!detail.passed);
    assert_eq!(detail.spent_millis, 7);
    // The untrusted half travelled in the payload, under the label that says so.
    assert_eq!(verdict.payload, b"field `name` is missing");

    // The new step's intent, carrying the feedback.
    let (next, Record::V1(intent)) = &written[1];
    assert_eq!(*next, StepId::new(2));
    assert_eq!(intent.kind, RecordKind::Intent);
    assert_eq!(intent.payload, b"field `name` is missing");
}

#[test]
fn an_inferential_sensor_is_refused_by_the_tight_ring() {
    // V11's first half: the DECLARED cost decides admission, and it is read BEFORE running.
    // ⚠️ The row stays `parziale` all the same -- its second half has no subject while no
    // inferential sensor exists (§8.3, trigger C4). Condition 12 of the design: do NOT mark it.
    let mut journal = MemoryJournal::new();
    let sensor = ScriptedSensor {
        cost: CostClass::Inferential,
        outcome: VerdictOutcome::Fail,
    };

    let opened = run_the_ring(
        &sensor,
        &Untrusted::new("the artefact".into()),
        StepId::new(1),
        StepId::new(2),
        &mut journal,
    )
    .expect("the ring");

    assert_eq!(opened, None);
    // ⛔ AND NOTHING WAS WRITTEN, which is the half that would be missed: a ring that refused to
    // OPEN a step but journalled the verdict anyway would pass the assertion above.
    assert!(records(&journal).is_empty());
}
```

⚠️ **`MemoryJournal::new` e il percorso `simulator::journal` sono da VERIFICARE, non da
ricordare** — è la stessa regola che la D18 impone al derive di `bincode`:

```bash
grep -rn "pub struct MemoryJournal" -A6 crates/simulator/src/
grep -n "simulator" crates/kernel/Cargo.toml
```

- [ ] **Passo 2: fai girare, e leggi il rosso**

```bash
cargo test --locked -p kernel --test sensor_ring 2>&1 | tail -30
```

⛔ **Aspettati che `run_the_ring` non esista.** È il rosso di partenza.

- [ ] **Passo 3: scrivi l'anello**

In coda a `crates/kernel/src/sensor.rs`:

```rust
/// Runs one sensor over one artefact and carries the answer back into the journal.
///
/// ⛔ A FREE FUNCTION THAT TAKES THE PORT, like `reconcile::steps_in_doubt` — the project already
/// has this shape for "read the journal and derive", and this one writes as well. A struct
/// holding the journal would give the ring state, and I5 keeps state in one place.
///
/// ⛔ `next` IS DELIVERED AND NOT ALLOCATED, and that is not laziness: `StepId` HAS NO ALLOCATOR,
/// `ports/journal.rs` says so beside the type, and whether one arrives is the owner's — registered
/// and not taken since 2026-08-21. Inventing one here would take that decision by writing it.
///
/// Returns the id of the step it opened, or `None` when nothing was opened — either the verdict
/// passed, or the sensor was refused by the tight ring.
pub fn run_the_ring<S: Sensor, J: Journal>(
    sensor: &S,
    artefact: &Untrusted,
    step: StepId,
    next: StepId,
    journal: &mut J,
) -> Result<Option<StepId>, JournalError> {
    // ⛔ THE DECLARED COST IS READ BEFORE `observe` IS CALLED, and that ordering IS V11: a cost
    // that came back with the verdict would arrive after the expense (§6.4.1). Nothing is
    // journalled on this road — a sensor that never ran produced no verdict, and writing one
    // would be the record of an event that did not happen.
    if sensor.declared_cost() == CostClass::Inferential {
        return Ok(None);
    }

    let verdict = sensor.observe(artefact);

    // The verdict, upon the step whose artefact was judged. ⛔ `Verifiable` AND NOT
    // `Unrepeatable`: the class describes how a DOUBT about this record's effect would be
    // reconciled, and a verdict has no effect on the world — re-running the sensor over the same
    // artefact answers the same thing. ⚠️ It is never actually reconciled, because a `Verdict`
    // record opens no doubt (see `reconcile`); the field is mandatory and must still be true.
    let record = Record::V1(RecordV1 {
        kind: RecordKind::Verdict,
        effect: EffectClass::Verifiable,
        trust: Trust::Untrusted,
        payload: verdict.detail.as_str().as_bytes().to_vec(),
        reason: String::from("a sensor judged the artefact of this step"),
        detail: Some(Detail::Verdict(VerdictDetail {
            passed: verdict.outcome == VerdictOutcome::Pass,
            spent_millis: verdict.spent.get(),
        })),
    })
    .encode();
    journal.note(step, &record)?;

    if verdict.outcome == VerdictOutcome::Pass {
        return Ok(None);
    }

    // ⛔ A NEGATIVE VERDICT RE-ENTERS AS A NEW STEP (V14), AND NOBODY IS ASKED (Q10). The intent
    // carries the same untrusted detail as the feedback the next attempt has to answer.
    let feedback = Record::V1(RecordV1 {
        kind: RecordKind::Intent,
        effect: EffectClass::Idempotent,
        trust: Trust::Untrusted,
        payload: verdict.detail.as_str().as_bytes().to_vec(),
        reason: String::from("a sensor verdict re-entered the ring as a new step"),
        detail: None,
    })
    .encode();
    journal.intent(next, &feedback)?;

    Ok(Some(next))
}
```

⚠️ **`Millis::get()` è da VERIFICARE**, come tutto il resto: se il getter non c'è, il rimedio è
la stessa domanda che `StepId::get` si pose al Traguardo 3 — *serve a un chiamante che esiste* —
e va aggiunto **con quel chiamante**, non «per completezza».

- [ ] **Passo 4: le mutazioni, una alla volta e revocate da copia (D7)**

| # | Mutazione | Che cosa deve diventare rosso |
|---|---|---|
| M1 | l'anello **non** scrive il record di verdetto | `a_passing_sensor_…` e `a_failing_verdict_…` |
| M2 | l'anello apre il passo nuovo **anche** su `Pass` | `a_passing_sensor_writes_a_verdict_and_opens_nothing` |
| M3 | il controllo del costo dichiarato è **dopo** `observe` | ⚠️ **misura che cosa succede davvero:** se **niente** diventa rosso, l'ordinamento che il doc chiama *«IS V11»* è tenuto da nulla, ed è un mutante vivo da dichiarare |
| M4 | `passed` è sempre `true` | `a_failing_verdict_…`, sull'**archivio** |
| M5 | il `payload` del passo nuovo è vuoto | `a_failing_verdict_…`, ultima asserzione |

⛔ **Se una mutazione non uccide nulla, il rimedio è una sonda in più, non un'asserzione più
larga** — e se la sonda che la ucciderebbe congelasse una decisione aperta, si **dichiara** invece
di pinzarla (gotcha **#73**).

- [ ] **Passo 5: il registro, e le righe di catalogo**

In [`porta-di-qualita.md`](../../porta-di-qualita.md): la riga `V10` del blocco C passa a
**coperta**, col caso `sensor_modifies_the_artefact.rs` nella colonna *«deve scattare»* e
`a_passing_sensor_writes_a_verdict_and_opens_nothing` in quella *«deve restare verde»*.

⛔ **E QUATTRO righe NON si toccano** — `V11`, `V21`, `V27`, `Q18` — che è la **condizione 12**
del disegno, l'unica negativa. ⚠️ **`V11` in particolare è la tentazione di questo compito:** il
meccanismo di ammissione all'anello stretto **esiste** dopo il commit 5c e ha la sua sonda, ma la
seconda metà della riga *«gli inferenziali restano fuori»* non ha soggetto finché nessun sensore
inferenziale esiste. **Il meccanismo si costruisce, la riga non si muove.**

⚠️ **Le sonde di questo compito non hanno una riga di catalogo propria**, tranne `V10`. Si
**registra** come voce aperta e non si prende: la §7.4 è **spec**, vincolo globale 7 — stesso
trattamento di `PL-1`, `K-1`/`B-1` e delle sonde del Traguardo 5, stessa ragione (gotcha #36).

- [ ] **Passo 6: commit**

```bash
bash scripts/gate.sh
cargo test --locked --workspace --no-fail-fast
git add crates/kernel/src/sensor.rs crates/kernel/tests/sensor_ring.rs docs/porta-di-qualita.md
git commit -m "feat(sensor): l'anello giornala il verdetto e un verdetto negativo apre un passo nuovo (V14, Q10)"
```

#### Criterio di chiusura del compito 5

- [ ] `bash scripts/gate.sh` → `GATE GREEN` **a ciascuno dei tre commit**, non solo all'ultimo
- [ ] i casi `compile_fail` sono **uno in più**, e il numero si riconta col comando
- [ ] i record congelati sono **quattro**, e `record_v1.map` li ricostruisce tutti e quattro
- [ ] ⛔ i **tre** `.cbor` vecchi sono **byte-identici** — `git status --porcelain crates/kernel/tests/frozen/` nomina **solo** i file nuovi e la mappa
- [ ] l'array a mano di `frozen_bytes.rs` porta **quattro** varianti, e la sua `assert!` è stata **vista scattare** togliendo il quarto record
- [ ] i due `.stderr` toccati sono stati **riletti a mano**, e la corsa che li ha prodotti è girata **senza** `TRYBUILD=overwrite`
- [ ] il `Debug` stampa `detail`, e l'oracolo di `record_shape.rs` è stato visto **rosso** prima di essere riscritto
- [ ] i **sei** commenti di doc che mostravano un record a cinque campi portano il proprio **richiamo datato**
- [ ] ⚠️ `V11`, `V21`, `V27` e `Q18` **non** sono state marcate ✅ — condizione 12
- [ ] i fine-riga di ogni file toccato sono stati **rimisurati**, non supposti
- [ ] ogni mutazione che non ha ucciso niente è **dichiarata**, non taciuta

### Compito 6: §6.2 e §6.3 — il decisore, il gettone di conformità, e il record risolto

**Files:**
- Create: `crates/kernel/src/gateway/mod.rs` — `Candidate`, `Constraint`, `ConstraintClass`, `Conforming`, `GatewayError`, `resolve`, `dispatch`
- Modify: `crates/kernel/src/lib.rs` — il `pub mod gateway;`
- Modify: `crates/kernel/src/record.rs` — `RecordKind::Routing`, `Detail::Routing`, `RoutingDetail`
- Modify: `crates/kernel/src/reconcile.rs` — l'arm della variante nuova (**D23**)
- Modify: `crates/kernel/tests/frozen_bytes.rs` — l'array a mano, e il **quinto** record congelato
- Create: `crates/kernel/tests/frozen/record_v1_routing.cbor` — ⛔ **irreversibile**
- Modify: `crates/kernel/tests/frozen/record_v1.map` — la sezione del quinto record
- Create: `crates/kernel/tests/compile_fail/dispatching_an_unfiltered_candidate.rs` + `.stderr` — `Q13`
- Create: `crates/kernel/tests/compile_fail/conforming_has_no_constructor.rs` + `.stderr`
- Create: `crates/kernel/tests/gateway_decisor.rs` — le sonde del filtro e del record
- Modify: [`porta-di-qualita.md`](../../porta-di-qualita.md)

⛔ **Il campo `detail` c'è già: lo ha pagato il compito 5 (D24).** Questo compito aggiunge la
**propria** variante di `RecordKind`, la **propria** variante di `Detail` e il **proprio** record
congelato — e **nient'altro** del formato. ⚠️ **Se il compito 5 non è girato, questo non è
dispacciabile:** `Detail` non esiste.

```bash
grep -n "pub enum Detail" crates/kernel/src/record.rs     # deve rendere qualcosa
```

⛔ **E il numeratore dei record congelati si RICONTA, non si cita** — il compito 5 ne ha fatto uno
e questo il successivo, ma l'ordine di esecuzione non è garantito:

```bash
ls crates/kernel/tests/frozen/*.cbor
```

#### Commit 6a — il filtro dei vincoli, e il gettone che non si falsifica (`Q13`)

- [ ] **Passo 1: scrivi i due casi `compile_fail`, e MISURA i due errori**

⛔ **Il gettone ha DUE metà, e la riga di catalogo ne nomina UNA.** Il blocco B dice *«eseguire
una richiesta ← una prova di conformità | candidato non filtrato → **non compila** |
filtrato → compila»*. La seconda metà — che il gettone non si **coni** — è il precedente di
`grant_has_no_constructor.rs`, che il blocco B porta per `Grant` sotto la **stessa** riga.

`crates/kernel/tests/compile_fail/dispatching_an_unfiltered_candidate.rs`:

```rust
//! Catalogue §7.4.1 block B, row `Q13` — a candidate that has NOT been through the constraint
//! filter is not expressible as the argument of an execution. It is not that dispatching it is
//! forbidden: it cannot be SAID.

use kernel::gateway::{dispatch, Candidate};
use kernel::ports::journal::StepId;
use simulator::journal::MemoryJournal;

fn main() {
    let mut journal = MemoryJournal::new();

    let unfiltered = Candidate {
        model: "a-model",
        local: true,
        retains: false,
        price: 0,
    };

    // The whole case: `dispatch` wants a `Conforming`, and a `Candidate` is not one.
    let _ = dispatch(unfiltered, StepId::new(1), &mut journal);
}

// ⛔ IT REPORTS BY THE ORACLE AND NOT BY COMPILING, which is the WEAKER shape (gotcha #42), so
// the pair matters: `conforming_has_no_constructor.rs` beside it fires the strong way. Whoever
// widens this row reads both.
//
// ⛔ Names `kernel::` and declares no attributes of its own — gotcha #39.
```

`crates/kernel/tests/compile_fail/conforming_has_no_constructor.rs`:

```rust
//! The other half of block B row `Q13`: the token cannot be FORGED. `Conforming`'s fields are
//! private and its module is `kernel::gateway`, so from out here there is no way to build one.
//! ⛔ THE MINTER IS `resolve`, AND IT IS THE ONLY ONE — the same shape as `Arbiter::issue`
//! for `Grant` (§5.6), and for the same reason: a token whose producer lives INSIDE the crate
//! that defines it is not forgeable (§4.1 of the design).
//!
//! ⚠️ THE ERROR TEXT IS NOT GUESSED: `grant_has_no_constructor.stderr` shows that this shape of
//! error carries NO code — bare "cannot construct ... with struct literal syntax due to private
//! fields" — and that fact was itself a correction of a guess (gotcha #15). Measure it here too
//! rather than copying that file's oracle: the two types have a different number of fields, and
//! the `note:` line names them.
fn main() {
    let _forged = kernel::gateway::Conforming {};
}

// ⚠️ THE DECLARED LIMIT, and it is the same one `grant_has_no_constructor.rs` declares: trybuild
// compiles its cases as SEPARATE CRATES, so what is proved is the direction FROM OUTSIDE.
// Nothing here stops a `pub(crate)` constructor tomorrow — that would be a new catalogue row,
// and the catalogue is spec.
```

- [ ] **Passo 2: fai girare e leggi i due rossi**

```bash
cargo test --locked -p kernel --test compile_fail 2>&1 | tail -40
```

⛔ **Aspettati che il modulo `gateway` non esista.** È il rosso di partenza, e va **letto** prima
di scrivere: passo 2 della disciplina dell'audit. ⛔ **Mai `TRYBUILD=overwrite`.**

- [ ] **Passo 3: scrivi le sonde del filtro, e le tre uscite sono TRE**

`crates/kernel/tests/gateway_decisor.rs`, file nuovo — ⛔ **da FUORI la crate**:

```rust
//! The gateway decisor (§6.2): the chain, the two classes of constraint, and what each does when
//! the chain runs out. ⛔ NO MODEL IS CALLED HERE, and that is ADR-0020 in practice: the decisor
//! is verifiable with no provider in existence.

use kernel::gateway::{dispatch, resolve, Candidate, Constraint, GatewayError};
use kernel::ports::journal::{Journal, StepId};
use kernel::record::{Detail, Record, RecordKind};
use simulator::journal::MemoryJournal;

const LOCAL_CHEAP: Candidate = Candidate {
    model: "local-small",
    local: true,
    retains: false,
    price: 1,
};

const REMOTE_DEAR: Candidate = Candidate {
    model: "remote-large",
    local: false,
    retains: true,
    price: 100,
};

#[test]
fn a_conforming_candidate_is_chosen_and_nothing_is_degraded() {
    let chain = [LOCAL_CHEAP, REMOTE_DEAR];
    let resolved = resolve(&chain, &[Constraint::LocalOnly]).expect("resolve");
    assert!(!resolved.was_degraded());
}

#[test]
fn a_data_constraint_with_no_candidate_FAILS_CLOSED() {
    // ⛔ ADR-0012: constraints on DATA AND CONFIDENTIALITY fail closed at chain exhaustion.
    // There is no degraded road here, and the absence of one is the assertion.
    let chain = [REMOTE_DEAR];
    assert_eq!(
        resolve(&chain, &[Constraint::LocalOnly]),
        Err(GatewayError::NoConformingCandidate)
    );
}

#[test]
fn a_quality_constraint_with_no_candidate_DEGRADES_AND_SAYS_SO() {
    // ⛔ The other class: quality and cost proceed, DECLARING it. The two directions of the same
    // row, and they must not be the same test -- a single one could not tell them apart.
    let chain = [REMOTE_DEAR];
    let resolved = resolve(&chain, &[Constraint::PriceCeiling(10)]).expect("resolve");
    assert!(resolved.was_degraded());
}

#[test]
fn an_empty_chain_fails_closed_too() {
    assert_eq!(
        resolve(&[], &[]),
        Err(GatewayError::NoConformingCandidate)
    );
}

#[test]
fn the_dispatch_journals_the_RESOLVED_decision_and_not_a_reference_to_it() {
    // ⛔ ADR-0011: the record holds the RESOLVED decision, "not a reference to the configuration
    // -- re-reading today's configuration does not say what happened yesterday". So the model
    // NAME is in the record, and never an index into the chain.
    let mut journal = MemoryJournal::new();
    let chain = [REMOTE_DEAR];
    let resolved = resolve(&chain, &[Constraint::PriceCeiling(10)]).expect("resolve");

    dispatch(resolved, StepId::new(1), &mut journal).expect("dispatch");

    let entries = journal.replay().expect("replay");
    assert_eq!(entries.len(), 1);
    let (step, bytes) = &entries[0];
    assert_eq!(*step, StepId::new(1));
    let Record::V1(body) = Record::decode(bytes).expect("decode");
    assert_eq!(body.kind, RecordKind::Routing);
    let Some(Detail::Routing(routing)) = &body.detail else {
        panic!("the routing record carries no structured detail");
    };
    assert_eq!(routing.model, "remote-large");
    assert_eq!(routing.evaluated, 1);
    assert!(routing.degraded);
}
```

⚠️ **`Candidate` come `const` pretende che i suoi campi siano tutti `const`-costruibili**, e un
`&'static str` lo è. ⛔ **Se il compilatore lo rifiuta, NON cambiare il tipo del campo per far
passare il banco:** `&'static str` è la scelta di **P-9** — un nome che viene da fuori sarebbe
testo non fidato in un tipo di decisione — e il rimedio giusto è una funzione di banco, non un
`String`.

- [ ] **Passo 4: fai girare e leggi il rosso**

```bash
cargo test --locked -p kernel --test gateway_decisor 2>&1 | tail -30
```

- [ ] **Passo 5: scrivi il decisore**

`crates/kernel/src/gateway/mod.rs`, file nuovo:

```rust
//! The gateway decisor (§6.2) and the proof of conformance (§6.3).
//!
//! ⛔ NO MODEL IS INVOKED FROM HERE, AND NONE EVER WILL BE (ADR-0020): the kernel routes, filters
//! and journals; the provider adapters are staged out of this milestone by rule C of §0.4. What
//! that buys is written in ADR-0020 itself — the kernel is testable end to end with no model in
//! existence, and this file is where that stops being a slogan.
//!
//! ⛔ THE CHAIN IS DELIVERED PER CALL AND NOT HELD IN `Parameters`, and the choice is written
//! rather than left to be inferred. ADR-0034 forbids the kernel to READ a parameter it was not
//! handed; an argument IS being handed one. `Parameters` is the shape for what is fixed at
//! CONSTRUCTION, and a candidate chain is derived per request from policy and request (ADR-0011).
//! ⚠️ THE NEIGHBOURING OPEN VOICE IS `E94`, which asks the mirror question about the arbiter's
//! policy — it is the owner's and it is open; this line does not answer it.

use alloc::string::String;
use alloc::vec::Vec;

use crate::ports::journal::{Journal, JournalError, StepId};
use crate::record::{Detail, EffectClass, Record, RecordKind, RecordV1, RoutingDetail, Trust};

/// One candidate of the chain. ⛔ THE NAME IS `&'static str` AND THAT IS `I6`: a name arriving
/// from outside would be untrusted text inside a type the kernel DECIDES with (ADR-0014). It is
/// the same reasoning P-9 applied to `ResourceProfile`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Candidate {
    pub model: &'static str,
    /// Runs on this machine.
    pub local: bool,
    /// The provider keeps the data.
    pub retains: bool,
    pub price: u64,
}

/// ⛔ THE TWO CLASSES OF ADR-0012, AND THEY FAIL DIFFERENTLY. It is not a taxonomy: it is the
/// only thing that decides what happens when the chain runs out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintClass {
    /// Data and confidentiality. ⛔ FAIL CLOSED: an error, no fallback.
    Data,
    /// Quality and cost. ⛔ DECLARED DEGRADATION: it proceeds, saying so.
    Quality,
}

/// What a request demands of a candidate.
///
/// ⛔ A CANDIDATE THAT VIOLATES A CONSTRAINT IS NOT A FALLBACK: IT IS A DIFFERENT REQUEST
/// (ADR-0012), discarded before evaluation. That is why the filter runs first and the choice
/// second, rather than scoring everything and picking a winner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Constraint {
    /// The request may not leave this machine.
    LocalOnly,
    /// The provider may not keep the data.
    NoRetention,
    /// ⚠️ QUALITY CLASS: exceeding it degrades, it does not refuse.
    PriceCeiling(u64),
}

impl Constraint {
    pub fn class(self) -> ConstraintClass {
        match self {
            Constraint::LocalOnly | Constraint::NoRetention => ConstraintClass::Data,
            Constraint::PriceCeiling(_) => ConstraintClass::Quality,
        }
    }

    fn satisfied_by(self, candidate: &Candidate) -> bool {
        match self {
            Constraint::LocalOnly => candidate.local,
            Constraint::NoRetention => !candidate.retains,
            Constraint::PriceCeiling(ceiling) => candidate.price <= ceiling,
        }
    }
}

/// ⛔ THE PROOF OF CONFORMANCE (§6.3.1). It cannot be forged: every field is private and the only
/// place that builds one is `resolve`, below. That makes `Q13` a PROPERTY and not a check — an
/// unfiltered candidate is not EXPRESSIBLE as the argument of `dispatch`.
///
/// ⚠️ AND THE LIMIT IS §6.3.2, repeated here because it is the half that gets forgotten: A TOKEN
/// PROVES PROVENANCE, NOT CORRECTNESS. If `resolve` has a defect it mints wrong tokens and the
/// compiler says nothing. It removes ONE class of error — "we forgot to filter" — not two.
///
/// ⛔ IT CARRIES THE WHOLE RESOLVED DECISION AND HAS NO GETTER FOR IT, which is deliberate: the
/// only consumer is `dispatch`, in this module, and a public getter would exist for nobody
/// (the "no caller, no item" rule `ProcessError` already carries).
#[derive(Debug)]
pub struct Conforming {
    model: &'static str,
    evaluated: u32,
    degraded: bool,
}

impl Conforming {
    /// ⛔ DEGRADATION IS DECLARED, NEVER SILENT (ADR-0012, ADR-0019). This is the one thing a
    /// caller can ask, and it exists because a caller that cannot tell would have no way to say
    /// so to the user — which is the whole of "si dichiara prima, non si fallisce dopo".
    pub fn was_degraded(&self) -> bool {
        self.degraded
    }
}

/// What can go wrong. ⛔ DELIBERATELY POOR, like `JournalError`: one variant, and it means the
/// one thing that has no road onwards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GatewayError {
    /// ⛔ FAIL CLOSED (ADR-0012): no candidate satisfies the DATA constraints. There is no
    /// degraded road for this class, and its absence is the guarantee.
    NoConformingCandidate,
}

/// Walks the chain and mints the proof for the first candidate that conforms.
///
/// ⛔ THE TWO CLASSES ARE READ IN A FIXED ORDER, AND THE ORDER IS THE DECISION: the data
/// constraints are a filter — a candidate that fails one is not in the running at all — and the
/// quality constraints are a PREFERENCE among those that are left. Reading them the other way
/// round would let a price ceiling keep a request on a machine its data was not allowed to
/// leave, which is exactly the silent failure ADR-0012 sorts the classes to prevent.
pub fn resolve(
    chain: &[Candidate],
    constraints: &[Constraint],
) -> Result<Conforming, GatewayError> {
    let evaluated = chain.len() as u32;

    let admissible = |candidate: &Candidate| {
        constraints
            .iter()
            .filter(|c| c.class() == ConstraintClass::Data)
            .all(|c| c.satisfied_by(candidate))
    };

    let preferred = |candidate: &Candidate| {
        constraints
            .iter()
            .filter(|c| c.class() == ConstraintClass::Quality)
            .all(|c| c.satisfied_by(candidate))
    };

    // First choice: everything satisfied, nothing degraded.
    if let Some(candidate) = chain.iter().find(|c| admissible(c) && preferred(c)) {
        return Ok(Conforming {
            model: candidate.model,
            evaluated,
            degraded: false,
        });
    }

    // Second choice: the data constraints hold and a quality one does not. ⛔ THIS ROAD PROCEEDS,
    // and `degraded` is how it says so.
    if let Some(candidate) = chain.iter().find(|c| admissible(c)) {
        return Ok(Conforming {
            model: candidate.model,
            evaluated,
            degraded: true,
        });
    }

    // ⛔ AND THERE IS NO THIRD ROAD. An empty chain arrives here too, and it is the same answer
    // for the same reason: nothing conforms.
    Err(GatewayError::NoConformingCandidate)
}

/// Writes the RESOLVED routing record upon the step (ADR-0011) — and it CONSUMES the proof, so
/// one resolution dispatches once. Same shape as `Process::start` consuming a `Grant`.
///
/// ⛔ WHAT IS NOT HERE, AND IT IS STAGED RATHER THAN MISSING: the call to a provider. The
/// adapters are rule C of §0.4 — there is no provider to call — and the trigger is written here
/// rather than in prose elsewhere: THE FIRST PROVIDER ADAPTER. ⚠️ A deadline written in prose
/// has nothing that makes it fire (gotcha #77), so this one is not a promise: what this function
/// does today is the whole of what it claims to do.
pub fn dispatch<J: Journal>(
    token: Conforming,
    step: StepId,
    journal: &mut J,
) -> Result<(), JournalError> {
    // ⛔ THE RECORD CAN ONLY BE BUILT FROM THE TOKEN, and that is the point: what gets journalled
    // cannot disagree with what was filtered, because there is nothing else to build it from.
    let record = Record::V1(RecordV1 {
        kind: RecordKind::Routing,
        effect: EffectClass::Idempotent,
        trust: Trust::Instruction,
        payload: Vec::new(),
        reason: String::from("the gateway resolved the routing for this step"),
        detail: Some(Detail::Routing(RoutingDetail {
            model: String::from(token.model),
            evaluated: token.evaluated,
            degraded: token.degraded,
        })),
    })
    .encode();

    journal.note(step, &record)
}
```

⚠️ **`Trust::Instruction` col payload VUOTO, e il precedente è `Arbiter::set_policy`**, che fa
esattamente questo: nessun byte esterno entra in questo record, quindi l'etichetta è **vera** e
non decorativa. Il modello **nome** viene dalla catena consegnata, cioè da noi.

- [ ] **Passo 6: dichiara il modulo**

In `crates/kernel/src/lib.rs`, in coda: `pub mod gateway;` — ⛔ **e il doc di testa non si tocca**
(AUD-046).

#### Commit 6b — il record risolto entra nel formato, e il quinto record congelato

⛔ **Rileggi il commit 5b prima:** questo commit rifà la sua **seconda metà** — la variante, il
record congelato, l'array a mano — e **non** la prima, perché il campo c'è già (D24).

- [ ] **Passo 1: la variante e la specie del dettaglio**

In `RecordKind`, in coda, indice **4** — ⛔ **un indice non si riusa mai**:

```rust
    /// ⛔ THE RESOLVED ROUTING OF A STEP (ADR-0011), journalled WITH the step. Like `Note` and
    /// `Verdict` it neither opens a doubt nor closes one: the doubt is about an EFFECT, and a
    /// routing record says what was DECIDED, not what reached the world. The effect of the step
    /// is still owed by the step's own outcome.
    #[n(4)]
    Routing,
```

In `Detail`, in coda, indice **1**:

```rust
    /// The resolved routing of the step (§6.2, ADR-0011).
    #[n(1)]
    Routing(#[n(0)] RoutingDetail),
```

E la struttura, accanto a `VerdictDetail`:

```rust
/// The RESOLVED routing decision (ADR-0011). ⛔ THE MODEL NAME AND NOT AN INDEX INTO THE CHAIN,
/// and the ADR says why in one line: the record "holds the RESOLVED decision, not a reference to
/// the configuration — re-reading today's configuration does not say what happened yesterday".
/// An index would be exactly that reference.
///
/// ⚠️ IT IS A `String` HERE AND A `&'static str` IN `gateway::Candidate`, and the asymmetry is
/// the point rather than an oversight: a name on the WIRE has to be decodable, and P-9 measured
/// that a `&'static str` is not producible from arriving bytes without leaking. The conversion
/// happens in `dispatch`, in one place.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(array)]
pub struct RoutingDetail {
    #[n(0)]
    pub model: String,
    /// How many candidates the filter walked. ⚠️ NOT the chain's length as configured TODAY —
    /// what was evaluated THEN, which is the same distinction the ADR draws for the model.
    #[n(1)]
    pub evaluated: u32,
    /// ⛔ A QUALITY CONSTRAINT WAS RELAXED, AND IT WAS DECLARED (ADR-0012). A degradation that
    /// did not reach the record would be exactly the silent one ADR-0019 exists to forbid.
    #[n(2)]
    pub degraded: bool,
}
```

- [ ] **Passo 2: l'arm di `reconcile`, RIMISURATO (D23)**

```rust
                // ⛔ A ROUTING RECORD NEITHER OPENS A DOUBT NOR CLOSES ONE, and it was measured
                // for THIS variant rather than inherited: `enter` would leave every routed step
                // in doubt for ever, and `leave` would close a doubt that no outcome resolved —
                // which is the silent loss of a real doubt, the one failure ADR-0007 exists to
                // prevent.
                RecordKind::Routing => {}
```

⚠️ **Provalo con le due mutazioni**, come al compito 5, e se nessuna muove niente **dichiaralo**.

- [ ] **Passo 3: il quinto record congelato**

```rust
        (
            "record_v1_routing.cbor",
            ROUTING_BYTES,
            record(
                RecordKind::Routing,
                EffectClass::Idempotent,
                Trust::Instruction,
                Some(Detail::Routing(RoutingDetail {
                    model: String::from("frozen"),
                    evaluated: 2,
                    degraded: true,
                })),
            ),
        ),
```

⚠️ **`model: "frozen"` e non un nome vero:** i tre record congelati usano già `FROZEN_PAYLOAD` e
`FROZEN_REASON` per la stessa ragione — un valore che si riconosce a occhio nella mappa, e che non
somiglia a un dato reale. ⛔ **E `degraded: true` con `evaluated: 2`**, perché `false` e `0`
codificano byte che somigliano a mezza tabella di indici.

⛔ **L'array a mano di `frozen_bytes.rs` va esteso a CINQUE varianti**, e questa è di nuovo la riga
che **niente** fa diventare rossa — P-12.

- [ ] **Passo 4: i byte a mano, e la mappa**

Come al compito 5: sonda usa-e-getta, byte scritti **a mano**, cancellazione nella stessa corsa,
mappa **riletta dal banco**. ⛔ **Nessun percorso di rigenerazione.**

- [ ] **Passo 5: il registro**

La riga `Q13` del blocco B passa a **coperta**: `dispatching_an_unfiltered_candidate.rs` nella
colonna *«deve scattare»* e `a_conforming_candidate_is_chosen_and_nothing_is_degraded` in quella
*«deve restare verde»*. ⚠️ **E `conforming_has_no_constructor.rs` va nella stessa riga**, come
`grant_has_no_constructor.rs` vive sotto la riga della concessione: sono le **due metà** dello
stesso gettone.

⛔ **E gli SCRITTORI DI RECORD sono adesso QUATTRO, e va scritto perché la voce aperta cresce.**
Erano `Untrusted::promote` e `Arbiter::set_policy` quando il gotcha **#77** trovò che *«l'aiutante
nasce col SECONDO scrittore»* era scaduto senza che nulla diventasse rosso; il compito 5 ha
aggiunto `run_the_ring` e questo aggiunge `dispatch`. ⚖️ **La decisione del proprietario del
2026-08-10 regge — *«ciascuno degli scrittori ha la propria sonda»*** — e questo compito la
onora; ma la voce *«se le due funzioni debbano condividere un aiutante»* va **riaggiornata a
quattro** nella tabella unica di [`porta-di-qualita.md`](../../porta-di-qualita.md), col richiamo
datato. ⛔ **Non si costruisce l'aiutante:** è del proprietario, ed era già registrata e non presa.

- [ ] **Passo 6: commit**

```bash
bash scripts/gate.sh
cargo test --locked --workspace --no-fail-fast
git add crates/kernel/ docs/porta-di-qualita.md
git commit -m "feat(gateway): il record di routing risolto entra nel formato, e il quinto record congelato"
```

#### Criterio di chiusura del compito 6

- [ ] `bash scripts/gate.sh` → `GATE GREEN` a **entrambi** i commit
- [ ] `Q13` è **coperta** nel registro, con **entrambi** i casi `compile_fail`
- [ ] i due `.stderr` nuovi sono stati **letti dall'uscita vera**, non copiati da `grant_has_no_constructor.stderr`
- [ ] i record congelati sono **cinque**, la mappa li ricostruisce tutti, e i **quattro** vecchi sono byte-identici
- [ ] l'array a mano porta **cinque** varianti, e l'`assert!` è stata **vista scattare**
- [ ] le tre uscite del filtro hanno **tre** sonde distinte — conforme, fallimento chiuso, degrado dichiarato — e non una sola con tre asserzioni
- [ ] ⛔ nessun modello è invocato da nessuna parte: `grep -rn "provider\|adapter" crates/kernel/src/gateway/` non nomina nessuna chiamata
- [ ] la voce dell'**aiutante degli scrittori** è aggiornata a **quattro**, col richiamo datato, e **non** è stata chiusa
- [ ] i fine-riga di ogni file toccato sono stati **rimisurati**

### Compito 7: §6.6 — il permesso è una tripla, e «quali sono attivi ora» è una PROIEZIONE

**Files:**
- Create: `crates/kernel/src/permission.rs` — `Permission`, `Operation`, `grant`, `is_granted`
- Modify: `crates/kernel/src/lib.rs` — il `pub mod permission;`
- Modify: `crates/kernel/src/record.rs` — `RecordKind::Permission`, `Detail::Permission`, `PermissionDetail`
- Modify: `crates/kernel/src/reconcile.rs` — l'arm della variante nuova (**D23**)
- Modify: `crates/kernel/tests/frozen_bytes.rs` — l'array a mano, e il **sesto** record congelato
- Create: `crates/kernel/tests/frozen/record_v1_permission.cbor` — ⛔ **irreversibile**
- Modify: `crates/kernel/tests/frozen/record_v1.map`
- Create: `crates/kernel/tests/permission_triple.rs`
- Modify: [`porta-di-qualita.md`](../../porta-di-qualita.md)

⛔ **Il campo `detail` c'è già (D24).** Questo compito aggiunge la propria variante, la propria
specie di dettaglio e il proprio record congelato.

- [ ] **Passo 1: scrivi le sonde, e le direzioni sono TRE perché i componenti sono tre**

⛔ **Questa è la sonda che la §6.6 esiste per pretendere** — *«forma `(strumento × risorsa ×
operazione)` — **mai «lo strumento»**»* — e una sola asserzione non la prova: bisogna far variare
**ciascuno** dei tre componenti, uno per volta, contro una tripla concessa.

`crates/kernel/tests/permission_triple.rs`, file nuovo — ⛔ **da FUORI la crate**:

```rust
//! §6.6 — a permission is a TRIPLE, and "which permissions are active now" is a PROJECTION of
//! the journal, never a second archive (gotcha #7 applied to permissions).

use kernel::permission::{grant, is_granted, Operation, Permission};
use kernel::ports::journal::StepId;
use simulator::journal::MemoryJournal;

const READ_A: Permission = Permission {
    tool: "file",
    resource: "/a",
    operation: Operation::Read,
};

#[test]
fn a_granted_triple_is_granted() {
    let mut journal = MemoryJournal::new();
    grant(&mut journal, StepId::new(1), &READ_A).expect("grant");
    assert!(is_granted(&journal, &READ_A).expect("project"));
}

#[test]
fn a_different_OPERATION_is_not_covered() {
    let mut journal = MemoryJournal::new();
    grant(&mut journal, StepId::new(1), &READ_A).expect("grant");
    let write_a = Permission {
        operation: Operation::Write,
        ..READ_A
    };
    assert!(!is_granted(&journal, &write_a).expect("project"));
}

#[test]
fn a_different_RESOURCE_is_not_covered() {
    let mut journal = MemoryJournal::new();
    grant(&mut journal, StepId::new(1), &READ_A).expect("grant");
    let read_b = Permission {
        resource: "/b",
        ..READ_A
    };
    assert!(!is_granted(&journal, &read_b).expect("project"));
}

#[test]
fn a_different_TOOL_is_not_covered() {
    // ⛔ THIS IS THE ONE §6.6 NAMES OUT LOUD -- "never «the tool»". Granting `(file, /a, read)`
    // must not grant `(net, /a, read)`, and the three probes together are what makes the triple
    // a triple rather than a struct with three fields nobody compares.
    let mut journal = MemoryJournal::new();
    grant(&mut journal, StepId::new(1), &READ_A).expect("grant");
    let net_a = Permission {
        tool: "net",
        ..READ_A
    };
    assert!(!is_granted(&journal, &net_a).expect("project"));
}

#[test]
fn nothing_is_granted_on_an_empty_journal() {
    // ⛔ THE NON-VACUITY PROBE, and it is not decoration: without it a projection that answered
    // `true` to everything would pass three of the four probes above -- no, it would pass ONE,
    // and a projection that answered `false` to everything would pass THREE. That asymmetry is
    // the reason this one exists.
    let journal = MemoryJournal::new();
    assert!(!is_granted(&journal, &READ_A).expect("project"));
}
```

- [ ] **Passo 2: fai girare e leggi il rosso**

```bash
cargo test --locked -p kernel --test permission_triple 2>&1 | tail -30
```

- [ ] **Passo 3: scrivi la forma e la proiezione**

`crates/kernel/src/permission.rs`, file nuovo:

```rust
//! The permission as a triple (§6.6), and the projection that answers "which are active now".

use alloc::string::String;
use alloc::vec::Vec;

use crate::ports::journal::{Journal, JournalError, StepId};
use crate::record::{
    Detail, EffectClass, PermissionDetail, Record, RecordKind, RecordV1, Trust,
};

/// What may be done to the resource. ⛔ TWO VARIANTS AND NOT THREE, and the limit is declared
/// with its trigger: ADR-0016's default preset separates "reads, tests and builds proceed" from
/// "writes, commands and network egress ask", so READ and WRITE are the partition that preset
/// needs. A third operation arrives with THE FIRST TOOL THAT NEEDS ONE — and there are no tools
/// yet (rule C of §0.4, the mediator is staged).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Read,
    Write,
}

/// ⛔ A PERMISSION IS A TRIPLE, NEVER "THE TOOL" (§6.6). `(file, ~/x, read)` and not "the
/// filesystem"; `(net, a-host, egress)` and not "the internet".
///
/// ⛔ THE TWO NAMES ARE `&'static str` AND THAT IS `I6`, the same reasoning as
/// `gateway::Candidate` and P-9: a tool or resource name arriving from outside would be
/// untrusted text inside a type the kernel DECIDES with (ADR-0014). What comes back from the
/// journal is compared, never returned — see `is_granted`.
///
/// ⚠️ WHAT `V21` ASKS AND THIS DOES NOT ANSWER: "a permission holds for the granted triple AND
/// for the CURRENT SESSION". There is no session here — no interface, no mediator, no approval
/// cycle (rule C) — so the row stays `parziale` with its trigger, and this type holds the FORM
/// half only. Marking it would be the comfortable box §8.1's mandatory trigger exists to refuse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Permission {
    pub tool: &'static str,
    pub resource: &'static str,
    pub operation: Operation,
}

/// Records the grant. ⛔ A GRANTED PERMISSION IS A JOURNALLED FACT (§6.6), and from that one line
/// follows the thing that would otherwise cost a subsystem: there is NO second archive of active
/// permissions. It is gotcha #7 applied to permissions.
pub fn grant<J: Journal>(
    journal: &mut J,
    step: StepId,
    permission: &Permission,
) -> Result<(), JournalError> {
    let record = Record::V1(RecordV1 {
        kind: RecordKind::Permission,
        effect: EffectClass::Idempotent,
        trust: Trust::Instruction,
        payload: Vec::new(),
        reason: String::from("a permission was granted for this triple"),
        detail: Some(Detail::Permission(PermissionDetail {
            tool: String::from(permission.tool),
            resource: String::from(permission.resource),
            write: permission.operation == Operation::Write,
        })),
    })
    .encode();

    journal.note(step, &record)
}

/// ⛔ THE PROJECTION, AND IT RETURNS A `bool` RATHER THAN A LIST — which is the decision that
/// keeps `I6` intact. A list would have to hand back DECODED names, i.e. `String`s built from
/// bytes, and those would then be compared or displayed as if they were the kernel's own
/// vocabulary. Answering a QUESTION instead means the decoded values never leave this function.
///
/// ⛔ IT RE-READS AND DERIVES, like `reconcile::steps_in_doubt`, and it is a FREE FUNCTION taking
/// the port for the same reason: the project already has this shape, and a struct holding the
/// journal would give the projection state (I1, I5).
///
/// ⚠️ NOTHING REVOKES YET, and it is declared rather than left to be discovered: a permission
/// recorded is a permission held for ever by this function. Revocation is part of the mediator
/// and the approval cycle, staged by rule C — and the trigger is THE FIRST REVOCATION, which
/// will be a record of its own species and a new arm here.
pub fn is_granted<J: Journal>(
    journal: &J,
    wanted: &Permission,
) -> Result<bool, JournalError> {
    for (_, bytes) in journal.replay()? {
        // ⛔ A RECORD THIS BUILD CANNOT READ IS NOT AN ANSWER OF `false`. Skipping it would let a
        // newer permission species look like "not granted", which is the silent partial truth
        // P-15 measured and D20 exists to prevent — so a malformed record STOPS the projection.
        let Ok(Record::V1(body)) = Record::decode(&bytes) else {
            return Err(JournalError::Malformed);
        };
        if body.kind != RecordKind::Permission {
            continue;
        }
        let Some(Detail::Permission(held)) = &body.detail else {
            continue;
        };
        if held.tool == wanted.tool
            && held.resource == wanted.resource
            && held.write == (wanted.operation == Operation::Write)
        {
            return Ok(true);
        }
    }
    Ok(false)
}
```

⛔ **`JournalError::Malformed` è DA VERIFICARE, e se non esiste la decisione NON è inventarla.**
`JournalError` è dichiarato *«deliberatamente povero»* e ha quattro varianti; aggiungerne una
quinta è un atto che va argomentato come lo fu `StepInDoubt` — *«non allarga `OutOfOrder` perché
quello è definito da V6 mentre questa è ADR-0018»*. ⚠️ **Se la variante giusta non c'è**, le vie
sono due e vanno **misurate** prima di scegliere: aggiungerla con la propria ragione, oppure far
tornare alla proiezione un errore **proprio** — e la seconda ha il precedente di
`FileJournal::open`, che *«non restituisce `JournalError`»* perché nessuna delle sue varianti
significa ciò che serviva.

```bash
grep -n "pub enum JournalError" -A 30 crates/kernel/src/ports/journal.rs
```

- [ ] **Passo 4: la variante, la specie e il sesto record congelato**

`RecordKind`, indice **5**:

```rust
    /// ⛔ A PERMISSION GRANTED FOR A TRIPLE (§6.6). Like the three before it, it neither opens a
    /// doubt nor closes one — and the empty arm was measured for this variant too (D23).
    #[n(5)]
    Permission,
```

`Detail`, indice **2**, e la struttura:

```rust
/// The granted triple, on the wire (§6.6).
///
/// ⛔ `write: bool` AND NOT AN `Operation` ENUM, and it is the SAME decision `VerdictDetail`
/// made for its outcome — read that doc, it carries the whole argument. In one line: an enum
/// here would be a FOURTH `index_only` enum on the wire, whose variant indices `frozen_bytes.rs`
/// would then have to pin ONE PER FROZEN RECORD. A `bool` costs one byte, pins itself, and the
/// day a third operation exists it RETIRES in favour of a new optional index — which is rule 3
/// of §4.9.2 doing exactly what it is for.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cbor(array)]
pub struct PermissionDetail {
    #[n(0)]
    pub tool: String,
    #[n(1)]
    pub resource: String,
    #[n(2)]
    pub write: bool,
}
```

Il record congelato, con l'array a mano portato a **sei** varianti:

```rust
        (
            "record_v1_permission.cbor",
            PERMISSION_BYTES,
            record(
                RecordKind::Permission,
                EffectClass::Unrepeatable,
                Trust::Instruction,
                Some(Detail::Permission(PermissionDetail {
                    tool: String::from("frozen"),
                    resource: String::from("frozen"),
                    write: true,
                })),
            ),
        ),
```

- [ ] **Passo 5: il registro, e le due righe che NON si toccano**

⛔ **`V21` resta `⚠️ parziale`** — la metà *«e per la sessione corrente»* non ha soggetto — e non
si marca. È la **condizione 12**. ⚠️ **E la riga di `V21` va riletta, non solo lasciata stare:**
il suo innesco esiste già ed è quello giusto; se non lo fosse, correggerlo è §8, cioè **spec** —
si **registra** e non si prende (vincolo globale 7).

- [ ] **Passo 6: commit**

```bash
bash scripts/gate.sh && cargo test --locked --workspace --no-fail-fast
git add crates/kernel/ docs/porta-di-qualita.md
git commit -m "feat(permission): la tripla, e quali permessi sono attivi ora e' una proiezione del giornale"
```

#### Criterio di chiusura del compito 7

- [ ] le **tre** direzioni della tripla hanno **tre** sonde distinte, più quella del giornale vuoto
- [ ] la proiezione **non restituisce** nessun nome decodificato — `grep -n "String" crates/kernel/src/permission.rs` non ne trova nessuno in un tipo di ritorno
- [ ] i record congelati sono **sei**, la mappa li ricostruisce, e i **cinque** vecchi sono byte-identici
- [ ] l'array a mano porta **sei** varianti, e l'`assert!` è stata **vista scattare**
- [ ] ⚠️ `V21` **non** è stata marcata ✅
- [ ] la scelta su `JournalError` è stata **misurata** e argomentata, non improvvisata

---

### Compito 8: §6.7 — lo stato di degrado si RICALCOLA, e non si cachea

**Files:**
- Create: `crates/kernel/src/degradation.rs` — `Degradation`, `degradation_now`
- Modify: `crates/kernel/src/lib.rs` — il `pub mod degradation;`
- Create: `crates/kernel/tests/degradation_state.rs`
- Modify: [`porta-di-qualita.md`](../../porta-di-qualita.md)

✅ **QUESTO COMPITO NON TOCCA IL FORMATO**, e P-11 lo dichiara: il degrado si **ricalcola**, non
si scrive. Nessuna variante, nessun record congelato, nessun `.stderr`.

⛔ **E porta con sé una DIVERGENZA DICHIARATA dal disegno, che chi esegue deve conoscere prima:**
ADR-0019 e §6.7 dicono che il core *«**mantiene** uno stato di degrado corrente, **alimentato
dagli eventi**»*, e quelle parole si leggono **anche** come mantenimento incrementale. Il disegno
legge *«mantiene»* come *«espone»*, non come *«cachea»* — §5.2 — e **se la lettura del
proprietario è l'altra, la scelta è sua**. Il compito costruisce la lettura del disegno e la
dichiara accanto al tipo.

- [ ] **Passo 1: scrivi le sonde, e la terza è quella che conta**

`crates/kernel/tests/degradation_state.rs`, file nuovo:

```rust
//! §6.7 — the degradation state is DERIVED and recomputed, never authoritative of itself.

use kernel::arbiter::{Arbiter, ComputeClass, Mib, Preemption, ResourceProfile, WorkDescriptor};
use kernel::degradation::degradation_now;
use kernel::gateway::{dispatch, resolve, Candidate, Constraint};
use kernel::parameters::Parameters;
use kernel::ports::journal::StepId;
use simulator::journal::MemoryJournal;

#[test]
fn an_idle_machine_declares_nothing() {
    let journal = MemoryJournal::new();
    let arbiter = /* an arbiter with room to spare — build it the way `arbiter_admission.rs`
                     does, and READ that file rather than guessing the constructor */;
    let state = degradation_now(&arbiter, &journal).expect("derive");
    assert!(!state.vram_exhausted);
    assert!(!state.routing_degraded);
}

#[test]
fn a_degraded_routing_shows_up_in_the_state() {
    // ⛔ ADR-0012 says a quality constraint relaxed is a DECLARED degradation. This is where the
    // declaration becomes observable: without it, "declared" means "written in a record nobody
    // reads", which is the silent degradation ADR-0019 forbids.
    let mut journal = MemoryJournal::new();
    let chain = [Candidate { model: "remote", local: false, retains: true, price: 100 }];
    let resolved = resolve(&chain, &[Constraint::PriceCeiling(10)]).expect("resolve");
    dispatch(resolved, StepId::new(1), &mut journal).expect("dispatch");

    let arbiter = /* the same idle arbiter */;
    let state = degradation_now(&arbiter, &journal).expect("derive");
    assert!(state.routing_degraded);
}

#[test]
fn it_is_RECOMPUTED_and_not_cached() {
    // ⛔ THIS IS THE PROBE WITHOUT WHICH "recomputed" IS A CLAIM HELD BY NOTHING. Ask once, change
    // the world, ask again: a cached answer would repeat itself, and the whole reason §6.7
    // recomputes is that a cache makes "never authoritative of itself" a matter of discipline
    // instead of construction.
    let mut journal = MemoryJournal::new();
    let arbiter = /* the same idle arbiter */;

    let before = degradation_now(&arbiter, &journal).expect("derive");
    assert!(!before.routing_degraded);

    let chain = [Candidate { model: "remote", local: false, retains: true, price: 100 }];
    let resolved = resolve(&chain, &[Constraint::PriceCeiling(10)]).expect("resolve");
    dispatch(resolved, StepId::new(1), &mut journal).expect("dispatch");

    let after = degradation_now(&arbiter, &journal).expect("derive");
    assert!(after.routing_degraded);
}
```

⛔ **I tre `/* … */` sono SEGNAPOSTO E VANNO RIEMPITI LEGGENDO `arbiter_admission.rs`**, non
ricordando la firma di `Arbiter::new`. ⚠️ **Non è una svista del piano:** la firma cambia col
**compito 1**, che aggiunge `ArbiterId` a `Parameters` — dettarla qui sarebbe dettare uno stato
che il traguardo stesso sta cambiando, cioè il gotcha **#57**, *una previsione citata come una
misura*.

- [ ] **Passo 2: scrivi il derivato**

`crates/kernel/src/degradation.rs`, file nuovo:

```rust
//! The degradation state (§6.7, ADR-0019): DERIVED, recomputable, never authoritative of itself.

use crate::arbiter::Arbiter;
use crate::ports::journal::{Journal, JournalError};
use crate::record::{Detail, Record, RecordKind};

/// What is degraded right now. ⛔ THE SELECTION CRITERION IS §7.5's, and it is the reason this
/// struct is short: what is shown is what CHANGES WHAT THE USER CAN DO, not every internal
/// variation. "An interface that signals everything is indistinguishable from one that signals
/// nothing."
///
/// ⛔ TWO INPUTS OF ADR-0019 HAVE NO SOURCE IN THIS MILESTONE, and they are declared rather than
/// faked: CONNECTIVITY (§7 — `network` has no real implementation, §8.2.2) and PROVIDER HEALTH
/// (§6.2 — the adapters are rule C). Their triggers are their implementations, and no field
/// stands here waiting for them: a field that is always `false` reads as "fine" rather than as
/// "unknown", which is the falsest of the two.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Degradation {
    /// The arbiter has nothing left to admit. ⛔ IT IS AN INPUT NAMED BY ADR-0019 ITSELF, and
    /// §5 added a revocable consumer to it (ADR-0033) — "the 3D viewer is paused during a
    /// render" is exactly a condition that changes what the user can do.
    pub vram_exhausted: bool,
    /// The last routing resolved relaxed a quality constraint (ADR-0012). ⛔ DECLARED, not
    /// silent — and this field is where the declaration stops being private to the record.
    pub routing_degraded: bool,
}

/// Derives the state from the world as it is NOW.
///
/// ⛔ NO CACHE, AND THAT IS THE WHOLE DESIGN. It follows `reconcile::steps_in_doubt`: read, derive,
/// answer. It makes "never authoritative of itself" true BY CONSTRUCTION rather than by
/// discipline — a cache is bought the day a measurement asks for one, which is the same formula
/// the journal's checkpoint carries.
///
/// ⚠️ DECLARED DIVERGENCE, and the owner may read it the other way: ADR-0019 and §6.7 say the
/// core "MAINTAINS a current degradation state, FED BY EVENTS", and those words also read as
/// incremental maintenance. This reads "maintains" as "exposes". §5.2 of the milestone design
/// makes the same reading and says out loud that the other one is the owner's to choose.
pub fn degradation_now<J: Journal>(
    arbiter: &Arbiter,
    journal: &J,
) -> Result<Degradation, JournalError> {
    // ⛔ THE LAST ROUTING AND NOT ANY ROUTING: a degradation that happened and was then resolved
    // is not the state NOW, and "ever degraded" would be a fact about history rather than about
    // what the user can do — which is the §7.5 criterion this file is built on.
    let mut routing_degraded = false;
    for (_, bytes) in journal.replay()? {
        let Ok(Record::V1(body)) = Record::decode(&bytes) else {
            continue;
        };
        if body.kind != RecordKind::Routing {
            continue;
        }
        if let Some(Detail::Routing(routing)) = &body.detail {
            routing_degraded = routing.degraded;
        }
    }

    Ok(Degradation {
        // ⛔ THE COMPARISON IS AGAINST WHAT THE ARBITER WAS HANDED, not against a number chosen
        // here: `total_vram` is a DELIVERED parameter (ADR-0034), and reading the GPU would be an
        // OS call I3 forbids.
        vram_exhausted: arbiter.allocated() >= arbiter.ceiling(),
        routing_degraded,
    })
}
```

⛔ **`Arbiter::ceiling()` PROBABILMENTE NON ESISTE, e questo è il punto del passo.** L'API
pubblica misurata il 2026-08-30 è `set_policy`, `allocated`, `admit`, `queued`, `promote`,
`revoking`, `release` — **niente tetto**. Le vie sono due e vanno **misurate e argomentate**, non
scelte per comodità:

| Via | Che cosa costa |
|---|---|
| `Arbiter` guadagna `ceiling()` | un getter nuovo su un tipo che il Traguardo 5 ha chiuso — e il precedente di `StepId::get` dice che un getter torna **col chiamante che lo pretende**, che qui esiste |
| `degradation_now` riceve anche i `Parameters` | nessuna modifica all'arbitro, ma il chiamante deve tenere in passo due valori che l'arbitro già tiene insieme — cioè **due verità indipendenti**, la forma di `E25` |

📌 **La prima ha il precedente e il chiamante; si scriva quella, e il perché accanto al getter.**
⚠️ **E se `allocated()` e il tetto non fossero confrontabili senza un terzo valore** — le quote
permanenti sottratte da ADR-0033 — **fermati e dichiaralo** invece di inventare la formula: la
§5.1 dell'arbitro ha già una **voce aperta** su quali siano i tre addendi, ed è del proprietario.

- [ ] **Passo 3: le mutazioni**

| # | Mutazione | Che cosa deve diventare rosso |
|---|---|---|
| M1 | `routing_degraded` è sempre `false` | `a_degraded_routing_shows_up_in_the_state` e `it_is_RECOMPUTED_…` |
| M2 | il ciclo si ferma al **primo** record di routing invece che all'ultimo | ⚠️ **niente, con queste sonde** — e allora la frase *«l'ultimo e non uno qualsiasi»* è un mutante vivo: o si aggiunge la sonda con **due** dispacci, o si **dichiara** |
| M3 | `vram_exhausted` è sempre `false` | una sonda con l'arbitro pieno, che il passo 1 **non ha scritto** — ⛔ **scrivila**, o metà del tipo è tenuta da nulla |

⛔ **M3 è un buco del piano trovato scrivendolo, e sta scritto invece che corretto in silenzio:**
le sonde del passo 1 provano `routing_degraded` in tre modi e `vram_exhausted` in **nessuno**.
Chi esegue aggiunge la quarta sonda — un arbitro saturo — prima di chiudere.

- [ ] **Passo 4: il registro, e le due righe che NON si toccano**

⛔ **`V27` e `Q18` restano `⚠️ parziale`.** `V27` perché *«che l'**interfaccia** lo dichiari
prima»* non ha soggetto; `Q18` perché il metodo assegnato è DST con iniezione del guasto di rete e
**`network` non ha implementazione reale**. È la **condizione 12**, e sono due delle quattro righe
che il disegno vieta esplicitamente di marcare.

- [ ] **Passo 5: commit**

```bash
bash scripts/gate.sh && cargo test --locked --workspace --no-fail-fast
git add crates/kernel/ docs/porta-di-qualita.md
git commit -m "feat(degradation): lo stato di degrado si ricalcola dal mondo, e non si cachea"
```

#### Criterio di chiusura del compito 8

- [ ] la sonda *«ricalcolato e non cacheato»* esiste e **cambia il mondo fra le due domande**
- [ ] `vram_exhausted` ha la propria sonda — il buco **M3** è chiuso
- [ ] i due ingressi senza sorgente — connettività e salute dei provider — sono **dichiarati** accanto al tipo, e **non** hanno un campo che dica `false`
- [ ] la divergenza dichiarata su *«mantiene»* è scritta accanto alla funzione
- [ ] ⚠️ `V27` e `Q18` **non** sono state marcate ✅
- [ ] la scelta fra il getter e i `Parameters` è **argomentata** accanto al codice, non solo fatta
- [ ] ogni mutazione che non ha ucciso niente è **dichiarata**

## Parte E — la prova e la chiusura

⛔ **La Parte E non aggiunge meccanismi: li mette sotto prova e chiude.** Un'eccezione, ed è
P-16: la terza proprietà di §5.7 pretende una **riconciliazione alla disconnessione** che oggi
non esiste e che la mappa dei file non ospita.

### Compito 9: `E152` — le due proprietà di §5.7 che mancano, iniettate su `process` e `ipc`

**Files:**
- Create: `crates/kernel/src/client.rs` — `ClientGrants`, `on_disconnect` (**P-16**)
- Modify: `crates/kernel/src/lib.rs` — il `pub mod client;`
- Create: `crates/simulator/src/ipc.rs` — la finta gui guidata dal seme
- Modify: `crates/simulator/src/lib.rs` — il `pub mod ipc;`
- Create: `crates/simulator/tests/gui_death_campaign.rs` — la proprietà **3**
- Modify: `crates/simulator/tests/arbiter_campaign.rs` — la proprietà **2**, o un banco nuovo se il file cresce troppo
- Modify: [`porta-di-qualita.md`](../../porta-di-qualita.md)

⛔ **Non è dispacciabile prima dei compiti 3 e 4:** le due proprietà si iniettano su `process` e
`ipc`, e lo schema `ipc` nasce col 4. Lo dice la §1.4 del disegno.

- [ ] **Passo 0: riconta quali proprietà mancano, non fidarti di «due»**

⛔ **`E152` dice *«§5.7 elenca cinque proprietà e la campagna dell'arbitro ne tiene tre»*, e il
conteggio si rifà sul codice** — è la regola 1 della §7.3 del disegno, e il precedente è il Task
13 del Traguardo 5, dove *«se il conteggio vero diverge, vince il conteggio»*.

```bash
grep -n "property_" crates/simulator/tests/arbiter_campaign.rs
```

📌 **Le cinque, e le loro porte**, dalla §5.7 della spec: la somma non supera il budget
(`reactor`) · nessun processo è attivo senza concessione valida (**`process`**) · la gui muore
tenendo una concessione discrezionale (**`ipc`**) · una transizione di policy interrotta lascia
un passo riconciliabile (`journal`) · una concessione scaduta non resta allocata (`reactor`).

#### Commit 9a — la riconciliazione alla disconnessione (**P-16**)

- [ ] **Passo 1: scrivi le sonde**

`crates/kernel/tests/client_grants.rs`, file nuovo:

```rust
//! ADR-0033: "if the GUI dies holding an ordinary grant, the core notices from the IPC
//! DISCONNECTION and reconciles". This is that reconciliation, seen from outside the crate.

#[test]
fn a_disconnected_client_gives_its_grant_back() {
    // baseline -> admit for the client -> disconnect -> the sum is back to baseline.
    // ⛔ THE ASSERTION IS ON `allocated()`, not on a bookkeeping flag: the property of §5.7 is
    // about THE SUM, and a flag could be right while the sum was wrong.
}

#[test]
fn a_disconnect_gives_back_only_that_client_s_grants() {
    // ⛔ THE HALF THAT GETS FORGOTTEN (§7.1.1 rule 3): a reconciliation that released EVERYTHING
    // would pass the probe above. Two clients, one dies, the other keeps its reservation.
}

#[test]
fn a_disconnect_of_a_client_that_holds_nothing_changes_nothing() {
    // ⛔ AND IT MUST NOT BE AN ERROR: a client may die before it ever asked, and treating that as
    // a fault would make an ordinary event look like a defect -- the shape of `ReleaseError`,
    // whose `UnknownGrant` §5.6 spent a whole open voice on.
}
```

⚠️ **I corpi sono da scrivere leggendo `arbiter_admission.rs`**, non ricordando le firme: il
compito **1** cambia `Arbiter::new` e `Parameters`, e dettarle qui sarebbe il gotcha **#57**.

- [ ] **Passo 2: scrivi il registro delle concessioni dei client**

`crates/kernel/src/client.rs`, file nuovo:

```rust
//! Which grant belongs to which IPC client, and what happens when one disconnects.
//!
//! ⛔ WHY THIS EXISTS AT ALL, and it is the finding P-16: §5.7 asks that the sum return to
//! baseline when a GUI dies holding a discretionary grant, and ADR-0033 names the mechanism —
//! "the core notices from the IPC DISCONNECTION and reconciles". Nothing performed that
//! reconciliation, and no file in the plan's map had a place for it.
//!
//! ⛔ THIS IS NOT THE ORCHESTRATION LOOP, and the distinction matters because `E50` and `E51`
//! wait for one. That loop decides WHEN `promote` runs relative to `admit`; this decides nothing
//! of the sort — it answers one event with one release. The two open voices stay open.
//!
//! ⛔ WHY IT IS NOT INSIDE `Arbiter`: the arbiter knows reservations, lanes and grants, and
//! nothing about clients. Giving it a `ClientId` would put a notion of the `ipc` port inside the
//! type that ADR-0005 keeps about RESOURCE — and I3's shape of argument applies within the crate
//! too: a boundary is worth what it refuses to know.
```

⛔ **La forma: `ClientGrants` tiene le coppie e `on_disconnect` le restituisce all'arbitro.** Una
concessione **si consuma** rilasciandola (`Arbiter::release` prende `Grant` per valore), quindi la
struttura deve poterla **estrarre**, non solo leggerla. ⚠️ **E `release` risponde tre cose dal
compito 1 (`E30`):** l'esito di una concessione **scaduta** non è un errore, e chi scrive
`on_disconnect` deve **leggere la forma che il compito 1 ha consegnato**, non quella di oggi.

- [ ] **Passo 3: le mutazioni**

| # | Mutazione | Che cosa deve diventare rosso |
|---|---|---|
| M1 | `on_disconnect` non rilascia niente | `a_disconnected_client_gives_its_grant_back` |
| M2 | `on_disconnect` rilascia **tutte** le concessioni | `a_disconnect_gives_back_only_that_client_s_grants` |
| M3 | un client sconosciuto è un errore | `a_disconnect_of_a_client_that_holds_nothing_changes_nothing` |

#### Commit 9b — la finta gui guidata dal seme, e le due campagne

- [ ] **Passo 1: la finta gui, sul precedente di `CrashingJournal`**

`crates/simulator/src/ipc.rs`, file nuovo. ⛔ **Si legge `crates/simulator/src/journal.rs`
prima**, perché la forma è quella: un guasto **permanente** deciso dal seme, non riprendibile —
*«un giornale che rifiuta una volta e poi riparte modella un disco cattivo, non un crash»*, e per
una gui vale identico: un client che muore non torna.

⚠️ **E il punto di morte si estrae da un generatore DIVERSO da quello dell'interlacciamento, con
seme derivato** — è la decisione 2 del piano del Traguardo 4: due `SeededRng` costruiti dallo
stesso numero danno la **stessa** sequenza, e la campagna esplorerebbe una **diagonale** dello
spazio invece dello spazio.

- [ ] **Passo 2: la campagna della proprietà 3, e la NON-VACUITÀ è obbligatoria**

⛔ **§5.7.1 lo pretende alla lettera, ed è il gotcha #17 nella sua forma esatta:** *«iniettare un
kill dove il codice non arriva è una prova vacua che sembra un successo. Si conta **prima**
quante operazioni compie davvero quel percorso, si inietta **dentro** quel numero, e si
**verifica che il guasto sia scattato** — non solo che il test sia passato.»*

📌 **E la finta gui deve TENERE DAVVERO una concessione prima di morire**, che è ciò che la §6.2
del disegno dice di sé: senza la richiesta e l'esito il seme ucciderebbe un client che non ha mai
chiesto nulla, e il confronto sarebbe **fra insiemi vuoti** — la lezione che il Traguardo 4 ha
imparato **tre volte**, ogni volta dopo che la precedente era stata chiusa.

Gli **oracoli** sono due, e servono entrambi:

| Oracolo | Che cosa coglie |
|---|---|
| il guasto è **scattato** — il conteggio dei punti di morte raggiunti è maggiore di zero | una campagna che non uccide mai nessuno |
| c'era **qualcosa da verificare** — in almeno un mondo la gui ha ottenuto `Granted` prima di morire | una campagna che uccide client che non hanno mai chiesto nulla |

⛔ **Il secondo è quello che il Traguardo 4 ha dovuto aggiungere tre volte.** Senza, *«la somma
torna alla linea di base»* è verde perché **non se n'era mai mossa**.

⚠️ **E la costante di non-vacuità è un RILEVATORE DI CAMBIAMENTO**, sul precedente di
`EXPECTED_OUTCOMES` e `EXPECTED_DOUBT_SETS`: il giorno in cui diventa rossa si **rimisura lo
spazio e si riscelgono i numeri**, non si edita la costante finché la barra torna verde — che
sarebbe il gotcha **#25**. La frase si scrive **accanto** alla costante.

- [ ] **Passo 3: la campagna della proprietà 2, su `process`**

*«Nessun processo è `Attiva` senza concessione valida»* — iniezione: **kill in istanti
arbitrari**. ⚠️ **La porta `process` ha già i suoi gettoni e i quattro casi `compile_fail` dal
Task 11 del Traguardo 5**, quindi ciò che manca non è la forma: è l'iniezione. Si legge
`crates/kernel/tests/worker_tokens.rs` prima di scrivere.

- [ ] **Passo 4: la campagna di mutazione, una alla volta e revocata da copia (D7)**

⛔ **E la regola del Traguardo 4 vale qui:** quando due mutazioni uccidono la **stessa**
asserzione, prima di concludere che la sonda non distingue i due difetti (gotcha **#55**) si
cerca **una terza mutazione che lasci passare la prima asserzione** — se esiste, le due non erano
in competizione ma su assi diversi.

- [ ] **Passo 5: il registro, e `E152`**

La riga di livello 2 passa da **PARZIALE** a coperta, con le **cinque** proprietà nominate una
per una. ⛔ **E il numeratore si riconta**, non si deduce da *«tre più due»*.

⚠️ **`E152` si chiude nella tabella unica** di [`porta-di-qualita.md`](../../porta-di-qualita.md),
col commit che la chiude — non si ricopia altrove.

#### Criterio di chiusura del compito 9

- [ ] le **cinque** proprietà di §5.7 hanno ciascuna la propria sonda, ricontate sul file
- [ ] i **due** oracoli di non-vacuità esistono, e il secondo è stato **visto rosso** su una campagna che uccide client senza concessione
- [ ] la costante di non-vacuità porta accanto la frase che dice **come si rimedia** a un suo rosso
- [ ] `on_disconnect` ha le **tre** sonde, e la seconda — *«solo quelle di quel client»* — è stata vista rossa con M2
- [ ] ⛔ `E50` ed `E51` **non** sono state chiuse: `client.rs` non è il ciclo di orchestrazione, e il suo doc lo dichiara
- [ ] `E152` è chiusa nella tabella unica, col commit

---

### Compito 10: la chiusura — è un AUDIT, non una scrittura

**Files:**
- Modify: il [disegno](../specs/2026-08-28-sottoprogetto-1-traguardo-6-altri-meccanismi-design.md) — una **§8** nuova, col verbale (decisione **B** della §7.1)
- Modify: [`porta-di-qualita.md`](../../porta-di-qualita.md), [`COMPENDIO.md`](../../COMPENDIO.md), [`HANDOFF.md`](../../HANDOFF.md), [`roadmap.md`](../../roadmap.md), [`README.md`](../../README.md)

⛔ **IL VERBALE VA NEL DISEGNO E NON NEL COMPENDIO**, ed è la decisione **B** della §7.1: dal
2026-08-28 `check-docs.sh` impone un **tetto al compendio** — il passo
`== compendium size ceiling ==` è nel cancello — quindi un verbale scritto lì compete con quel
tetto.

⛔ **E SI PARTE DAI NUMERI, NON DALLE FRASI.** È il gotcha **#49**, che al Traguardo 3 e al
Traguardo 5 si è presentato **due volte**, ogni volta con **gran parte del compito già
eseguita**. La prima domanda è *«è già fatto?»*, non *«come lo faccio?»*.

- [ ] **Passo 1: rifai OGNI conteggio col comando, prima di leggere che cosa i documenti ne dicono**

```bash
bash scripts/gate.sh
cargo test --locked --workspace --no-fail-fast
ls crates/kernel/tests/compile_fail/*.rs | wc -l
ls crates/kernel/tests/frozen/*.cbor
awk '/^#### 7\.4\.1/{f=1} /^#### 7\.4\.3/{f=0} f' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md | grep -c '^|'
git diff --name-only <primo-commit-del-traguardo>..HEAD -- crates/ scripts/ Cargo.lock
```

⚠️ **Se un conteggio diverge da ciò che un documento scrive, VINCE IL CONTEGGIO**, e la
divergenza si **registra** invece di essere appianata.

- [ ] **Passo 2: rileggi la §7.2 del disegno CONTRO IL CODICE, una condizione per volta**

⛔ **Mai contro sé stessa.** Una condizione può risultare **scritta troppo larga**, ed è successo
alla **4** del Traguardo 4. Le dodici sono nella §7.2 e **non si ricopiano qui**: si aprono lì.

📌 **Le due che hanno più probabilità di sorprendere, dette in anticipo:**

| | Perché |
|---|---|
| la **3** — *«`V10`, `V14` e `Q10` hanno un controllo che ESISTE»* | la definizione di §8.1 pretende **visto scattare** *e* **visto restare verde**. Un controllo che esiste e che nessuno ha visto fallire non la soddisfa |
| la **12**, negativa | è la sola che si fallisce **facendo troppo**. Si verifica leggendo le quattro righe in §8.3/§8.4 e controllando che portino ancora `⚠️ parziale` |

- [ ] **Passo 3: le voci aperte in UNA tabella sola, con la colonna di chi le chiude**

⛔ **È la condizione 11, e al Traguardo 5 il disegno l'aveva DIMENTICATA** — a rimediare fu chi
scriveva il piano, col Task 13. Qui nasce completa, e va **eseguita**: le voci che questo
traguardo lascia aperte — le otto del disegno, quelle che i compiti hanno registrato, e quelle
**ereditate** dal Traguardo 5 che restano — in **una** tabella, con **chi le chiude**.

⚠️ **E la colonna porta la notizia:** al Traguardo 5 si scoprì che per alcune il chiusore **non**
era il proprietario, e sparse fra i riquadri si leggevano tutte come *«aspetta il proprietario»*.

⛔ **Il metodo è il `grep`, non la memoria**, e ogni riga che restituisce si legge **intera**
(gotcha **#70**). Il blocco di comandi che il Traguardo 5 ha lasciato in
[`porta-di-qualita.md`](../../porta-di-qualita.md) è il precedente, e **dichiara di sé che il
filtro non basta**: ne restituì ventitré, di cui undici già chiuse, e ne **mancava cinque** che
si trovarono leggendo la §6 del compendio.

- [ ] **Passo 4: scrivi la §8 del disegno, con la tabella delle smentite**

⛔ **La tabella *«dove il disegno è stato smentito dall'esecuzione»* è la parte che vale**, ed è
la forma che al Traguardo 4 produsse *«la condizione 4 era scritta troppo larga»*. Il
pre-controllo di questo piano ne ha già **cinque** candidate — P-1, P-5, P-6, P-8, P-9 — e i
compiti ne aggiungeranno.

📌 **E le cose che il disegno ha detto e che l'esecuzione ha CONFERMATO vanno scritte pure**, in
una riga sola ciascuna: un verbale che elenca solo le smentite fa sembrare il disegno peggiore di
com'era, ed è la stessa disonestà del verbale che elenca solo i successi.

- [ ] **Passo 5: la manutenzione, nello stesso passaggio**

`CLAUDE.md` la elenca: [`COMPENDIO.md`](../../COMPENDIO.md), [`roadmap.md`](../../roadmap.md),
[`README.md`](../../README.md), lo stato degli spike, [`HANDOFF.md`](../../HANDOFF.md) se
emergono gotcha nuovi, e `CLAUDE.md` se cambia il modo di lavorare.

⛔ **Ciò che la chiusura NON fa**, dalla §7.4 del disegno: non chiude il **sotto-progetto 1** —
resta la §8 di [`tracciabilita.md`](../../tracciabilita.md), che si aggiorna alla chiusura del
sotto-progetto — non chiude le **voci del proprietario**, e **non tocca**
[`riferimenti.md`](../../riferimenti.md), perché `E146` è *registrata e non presa* e cominciare la
convenzione nuova a metà produrrebbe **due** convenzioni invece di una.

⚠️ **E il compendio §6 va toccato con la mano leggera:** il suo tetto è nel cancello, e la §6
dichiara di sé che *«a che punto sia NON è scritto qui»*. Ciò che cambia è la **tabella dei sei
traguardi** e il **prossimo passo** — non un racconto.

#### Criterio di chiusura del compito 10

- [ ] tutti i conteggi sono stati **rifatti col comando** e le divergenze **registrate**
- [ ] le **dodici** condizioni della §7.2 sono state rilette **contro il codice**, una per volta
- [ ] ⛔ la condizione **12** è stata verificata **leggendo le quattro righe**, non ricordandole
- [ ] le voci aperte stanno in **una** tabella sola, con la colonna di chi le chiude, e il censimento è stato fatto **col `grep`** e riletto intero
- [ ] la **§8** del disegno esiste, con la tabella delle smentite **e** quella delle conferme
- [ ] `check-docs.sh` è **verde**, compreso il tetto del compendio
- [ ] ⛔ ciò che si è trovato **già eseguito** è scritto come tale, non rieseguito
