# Traguardo 4 — il simulatore DST: piano di implementazione

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** dare al simulatore la sola cosa che gli manca — **rompersi** — e provare che il kernel si riconcili e che il motore lasci un archivio recuperabile, su uno spazio di semi invece che su un caso scelto a mano.

**Architecture:** due campagne con **soggetti diversi**. Il livello 1 sostituisce l'intera porta `journal` con un doppio che cade a una scrittura scelta dal seme, e misura la **riconciliazione del kernel**: gira `no_std`, senza I/O, in microsecondi. Il livello 2 sostituisce il `redb::StorageBackend` sotto `FileJournal` con uno che cade a un'operazione scelta, e misura la **coerenza dopo crash del motore**: gira in un banco di prova di `platform`, con I/O vero, su poche decine di punti. Entrambe girano a ogni commit perché sono test, e `cargo test` è già il secondo controllo del cancello.

**Tech Stack:** Rust 1.95.0 (edition 2024, `rust-toolchain.toml`) · `kernel` e `simulator` in `#![no_std]` + `alloc` + `#![forbid(unsafe_code)]` · `redb` 4.1.0 in `platform` · `minicbor` 2.3.0 per il record durevole.

**Il disegno che questo piano traduce:** [Traguardo 4 — il disegno](../specs/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst-design.md). ⛔ **Si legge prima**, e in particolare la §0.1 (il perimetro), la §2.2 (dove vive il backend cadente) e la **§11**, che è il richiamo in cui il disegno stesso è stato corretto dal codice.

---

## ⛔ Errata — cosa questo piano detta e il codice smentisce

**Si legge prima del compito, non dopo.** ⛔ **Il piano non si riscrive:** resta il registro di ciò che fu deciso, e le divergenze si appendono qui con la loro misura. **Sessantasei voci in otto passate** — otto dal **Task 1**, otto dal **Task 2**, dieci dal **Task 3**, dieci dal **Task 4**, nove dal **Task 5**, otto dal **Task 6**, otto dal **Task 7**, cinque dal **Task 8**; ⛔ **dodici sono DECISIONI** (**E2**, **E6**, **E11**, **E17**, **E28**, **E32**, **E33**, **E38**, **E47**, **E48**, **E54**, **E55**), e il proprietario può ribaltarle vedendole. ⛔ **E quattro voci del Task 5 erano difetti del TASK 6 e del TASK 7 trovati prima di scriverli** — **E39**, **E40**, **E43**, **E44** — tutte confermate eseguendo. ⛔ **La voce più importante di tutte è E52: due righe dei DOCUMENTI DI STATO dicevano il falso su come si chiude il gotcha #51, e lo dicevano dal brainstorming.** ⚠️ Entrambe le cifre sono **contate col `grep` sulla tabella**, non a memoria. ⚠️ Questa cifra si riconta **sulle righe** e non a memoria: alla passata del Task 3 è stata scritta *«venticinque, nove dal Task 3»* e poi ne sono state appese **dieci** — il gotcha **#31** commesso dentro il capoverso che lo dichiara, e colto **prima** del commit solo perché il riconteggio è stato fatto sulla tabella.

| # | Voce |
|---|---|
| **E1** | ⛔ **Le sonde del Task 1 sono DIECI, non otto, e la nona è l'unica che tenga una frase dettata dal piano stesso.** Il doc di `may_write` dichiara che *«the counter moves only on an `Ok`»* — una scrittura rifiutata dal protocollo write-ahead non ha raggiunto l'archivio, quindi non consuma una posizione. **Nessuna delle otto sonde dettate fa mai fallire una scrittura interna**, quindi la mutazione `self.writes += 1` incondizionato **sopravvive a tutte e otto**. Aggiunta `a_write_the_protocol_refuses_does_not_consume_a_crash_position`, e la sua non-vacuità è provata **togliendo il blocco** e non un'asserzione (gotcha **#55**): con la mutazione attiva e il corpo svuotato le altre nove restano **verdi**. È il gotcha **#45** — un'eccezione scritta in un commento è indistinguibile da una dimenticanza finché una mutazione non prova che il sistema la difende — e non è cosmetica: il punto di caduta si estrae contro **quante scritture lo scenario compie davvero**, quindi un contatore che avanza su scritture rifiutate fa cadere il giornale **prima** del punto estratto, e con un punto vicino alla fine **non lo fa cadere affatto** (gotcha **#17**) |
| **E2** | ⛔ **DECISIONE — `prune` è rifiutata dopo la caduta, e il piano la detta come delega pura.** È l'**unica operazione mutante** che non passa dalla guardia, mentre il doc dettato promette *«every later write is refused too»* e il limite dichiarato nomina le **sole letture**: un processo morto potava ancora. ⛔ **Gata su `self.fallen` e NON su `may_write()`**, che armerebbe la caduta contando la potatura — e la differenza è **misurata**, non argomentata: le due mutazioni uccidono asserzioni **diverse**, la riga 82 e la riga 68, quindi la sonda distingue i due difetti. 📌 **La classe di difetto vale oltre il caso, ed è il gotcha #29 spostato dalle invarianti a un limite dichiarato:** una partizione scritta in un doc — *«le scritture»* contro *«le letture»* — lascia scoperto il membro che **non appartiene a nessuna delle due**, e nulla lo segnala perché il doc **sembra** esaustivo |
| **E3** | ⛔ **Il Task 1 rende FALSO il doc di modulo di `crates/simulator/src/journal.rs`**, che dice *«THIS IS NOT THE FALLING DOUBLE … Here a journal that works; there one that breaks»* — e il cadente è ora **in quel file**. ⚠️ **Non è ipotetico che qualcuno ci si appoggi:** il [disegno del Traguardo 4](../specs/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst-design.md) e [`riferimenti.md`](../../riferimenti.md) **citano quella frase come prova che il cadente non esiste**. Chiuso con un **richiamo datato** in testa, non cancellando |
| **E4** | ⛔ **`cargo fmt --all -- --check` era pulito su tutto il workspace e il Task 1 lo sporcava su due file** — non è un rosso del cancello, che non chiama `fmt`, ma è una regressione contro un invariante di fatto. ⛔ **E il rimedio ovvio era quello sbagliato:** `cargo fmt` ordina `use crate::…` **prima** di `use alloc::…`, il contrario di `crates/kernel/src/{boundary,executor,reconcile}.rs`. Rimedio giusto: una **riga vuota** dopo il gruppo `alloc`, che porta rustfmt a uscita 0 **e** allinea alla convenzione esistente. ⚠️ Misurato prima di lanciarlo: `cargo fmt --all` tocca **esattamente i due file del compito** e nessun altro, quindi **G8** non è in gioco — i quattro sorgenti CRLF del repository sono già puliti e rustfmt conserva i fine-riga del file |
| **E5** | **`different_seeds_choose_different_points` era molto più debole del proprio scopo.** `seen.len() > 1` passa contro un generatore che estrae **due sole** posizioni su otto — compreso uno che **non estrae mai l'ultima scrittura**, che è il caso in cui l'archivio sopravvissuto è più pieno e l'insieme dei passi in dubbio è **massimo**, cioè precisamente ciò che `C7b` misura. Rinominata **`every_write_of_the_scenario_can_be_the_one_that_falls`** e portata a copertura piena, con l'accoppiamento dichiarato: regge perché i semi sono molti più delle scritture, **non** per costruzione di `below` |
| **E6** | ⛔ **DECISIONE — `the_same_seed_chooses_the_same_write` non può fallire, e si DICHIARA invece di toglierla.** `from_seed` è una funzione pura dei suoi due argomenti: nessuna implementazione potrebbe renderla rossa, e la determinatezza vera — che la **sequenza** si ripeta — la tiene `crates/simulator/tests/seeded_rng.rs`. Resta come **enunciato nominato** di ciò che il tipo promette, dichiarato tale invece che contato come copertura. È la postura del gotcha **#44**: la conformità del reattore tiene una riga che prova la **sola chiamabilità** e lo scrive, invece del `let _ = …` che sembra copertura |
| **E7** | **Quattro righe di doc che il piano non aveva, e ciascuna chiude una via per sbagliare da fuori.** `from_seed(_, 0)` **viola il proprio doc** — `RngExt::below` risponde `0` per un limite di `0`, e `0` non è dentro `0..0`, che è vuoto: il punto non arriverebbe mai, cioè la vacuità che il capoverso sopra invoca. Clausola nel doc più `debug_assert!` · *«how many writes the scenario REALLY performs»* non diceva **in quale corsa** — quella **senza crash**, che è a questo che serve `without_crash()` · ⛔ la decisione **D4**, il seme **derivato**, viveva **solo nel chiamante** e il cablaggio ovvio è quello sbagliato: gotcha **#36**, una nota che vive nel chiamante è letta e dimenticata · il locale si chiamava `outcome` e dentro `fn outcome` **ombreggiava il nome della funzione** |
| **E8** | ⚠️ **L'atteso della mutazione A invecchia con l'insieme che enumera.** Il piano elenca **quattro** sonde rosse; con la nona sono **cinque**, con la decima sono **sei**. Misurate, e i sei nomi sono esatti. 📌 Un criterio di chiusura che **enumera nomi** è una cifra dentro una frase (gotcha **#31**): si riconta eseguendo, non si cita |
| **E9** | ⛔ **Il criterio dello Step 2 del Task 2 è sbagliato, e produce un falso stop.** Diceva *«Atteso: non compila»* e aggiungeva *«se compila e passa al primo colpo, fermarsi»* — ma quel compito è **tutto banco** e non nomina nulla che non esista già: **compilare e passare al primo colpo è l'esito atteso**. Misurato: ha compilato e passato **senza un solo errore**. ⚠️ Il vero oracolo di un compito di solo banco è la **mutazione**, non il rosso iniziale, e un'istruzione che ferma su un esito corretto è peggio di nessuna istruzione. 📌 La stessa forma torna al **Task 3** — *«Atteso: non compila, `E0599` … oppure verde»* — mentre il piano stesso ammette due righe sotto che `StepId::get()` esiste |
| **E10** | **`Resolution`, `CRASH_SEED_OFFSET` e `crash_seed` sono dettati nel Task 2 e lì sono codice morto**: nessuna delle due sonde chiama `from_seed`, quindi sarebbero un `unused_imports` e un `dead_code` a ogni compilazione per un compito intero. Spostati al **Task 3**, dove vive il chiamante. ⛔ **Conseguenza da portare avanti, ed è una lacuna che questa voce apre:** il Task 3 li dà per esistenti e ora **deve definirli lui**, o è `E0425: cannot find function crash_seed`. ⚠️ La decisione che portano — il seme del punto di caduta **derivato e diverso** da quello dell'interlacciamento — non si perde: sta sul doc di `CrashingJournal::from_seed`, dove il chiamante la legge (**E7**) |
| **E11** | ⛔ **DECISIONE — `C7a` guadagna il proprio oracolo di non-vacuità, perché era verde su un archivio VUOTO.** Misurato: girando lo scenario con `falling_at(0)` sui cinquanta semi, `writes_done() == 0`, traccia vuota, `steps_in_doubt() == []` — l'asserzione era soddisfatta da una corsa che **non aveva scritto niente**. ⛔ **Il difetto non è il buco ma l'asimmetria:** il piano gli risponde con una mutazione **una tantum**, mentre allo stesso identico buco su `C7b` dà un oracolo **permanente** — `has_fallen()`, il cui doc dice *«senza di esso, "questa corsa non ha lasciato dubbi" e "il crash non è mai scattato" sono lo stesso verde»*. Qui la frase vale parola per parola con *«lo scenario non ha scritto niente»*. Una mutazione registrata in un documento è una **nota**, e una nota si legge e si dimentica (gotcha **#36**). Ora `assert_eq!(journal.writes_done(), WRITES_PER_RUN)` dentro il ciclo |
| **E12** | **Il pin fissa una RELAZIONE e non un numero, e una relazione è soddisfatta da zero.** Con `ACTIVITIES` o `STEPS` a zero la costante vale **0**, entrambe le asserzioni leggono `0 == 0` e restano **verdi** — e il Task 3 passerebbe quello zero a `from_seed`, la cui unica guardia è un `debug_assert!`, **compilato via in release**. Chiuso con `assert!(WRITES_PER_RUN > 0)` nel pin |
| **E13** | ⛔ **Lo scenario era committato senza che nulla dicesse che la sua proprietà più importante è tenuta altrove.** Nessuna sonda di quel file va rossa se le tre attività girano **una dopo l'altra**: il conteggio delle scritture è lo stesso, e senza crash non c'è dubbio in nessuno dei due casi. Aggiunto un rimando sul doc di `run`, nell'idioma che il repository usa dappertutto. ⛔ **E il nome che quel rimando cita diventa un VINCOLO sul Task 3, non un suggerimento:** se quella sonda nascerà con un nome diverso, il rimando diventa una **bugia silenziosa** — la specie esatta che le altre correzioni di questa passata tolgono. ✅ La cifra citata è **misurata e non ereditata**: sul controfattuale sequenziale il massimo insieme in dubbio scende da **tre** a **uno**, su tutti e cinquanta i semi |
| **E14** | ⚠️ **Tre cifre in prosa (gotcha #31), e una era DENTRO IL TESTO CORRETTIVO.** Il doc di `run` diceva *«3 activities x 4 steps»* e *«5000 VIRTUAL milliseconds»*, ripetendo in prosa tre costanti del file; il doc di `Yield` diceva *«twenty lines»* per un blocco di **ventidue**. ⛔ E il capoverso dettato per chiudere **E13** conteneva *«if the **three** activities run one after the other»* — la stessa specie, quattro righe dalla correzione che la elimina. Colta da chi eseguiva, non da chi dettava |
| **E15** | **Il tempo di parete che il Task 4 dovrà usare non è quello che l'harness stampa.** `cargo test` dice `finished in 0.00s`, sotto la propria risoluzione. Misurato sul binario: **0,0324 ms per seme** nella forma di `c7a`, **0,0246** nella forma di `C7b` — meno, perché col crash lo scenario si ferma prima — con un pavimento di **~8 ms** per il binario nudo e **~80 ms** per `cargo test -p simulator --test dst_campaign`. ⚠️ Una prima misura diceva *«0,04 ms»* e *«pavimento ~50 ms»*: il pavimento era un **artefatto dello strumento**, il `fork` della shell contato insieme al binario. ⛔ **E il difetto d'uso, che è quello che conta:** il costo per seme è **per ciclo**, non per campagna. Il Task 3 aggiunge **due** cicli da duecento semi e `c7a` è un terzo da cinquanta: un tetto scritto `N × costo` sbaglia **per il numero di cicli**. La formula è `pavimento + Σ_cicli (N × costo_del_ciclo)` |
| **E16** | ⛔ **Due cose misurate che vincolano il Task 3.** ✅ Il confronto **ordinato** di `C7b` — `assert_eq!(found, expected)` e non un insieme — è **verde su duecento semi**, e la ragione è strutturale: fra la scrittura sul giornale e il `push` sulla traccia **non c'è alcun `await`**, quindi archivio e traccia sono in lockstep. **Non indebolirlo a un confronto insiemistico «per prudenza»**: è gratis, ed è la difesa contro la classe del palindromo che in questo repository è già costata tre sonde vacue. ⛔ E la **mutazione C del Task 3 è mal-scopata**: sostituisce `without_crash()` nel solo sito di `c7b_…`, ma `a_crash_leaves_more_than_one_step_in_doubt_on_at_least_one_seed` ha **il proprio** sito con `from_seed` e resta **verde** dove il piano la annuncia rossa — misurato, `best = 3`. Va applicata a **entrambi** i siti |

| **E17** | ⛔ **DECISIONE, e il difetto era bloccante: `C7b` non aveva la propria non-vacuità, ed era quella di `C7a` REIMPORTATA un compito dopo averla chiusa.** L'asserzione sul conteggio dei crash prova che **il guasto è scattato**; non prova che **la riconciliazione sia stata esercitata**. Le due si separano, misurate in due modi: con un giornale che cade alla scrittura **zero** su ogni seme, `C7b` è **verde** e **duecento confronti su duecento** sono `[] == []`; con `ACTIVITIES = 1` è **verde** e l'insieme massimo è uno. E già senza mutazioni **sei semi su duecento** confrontano due insiemi vuoti sui propri meriti. Chiuso con `assert!(largest > 1)`, e ⛔ **provato dal discrimine giusto**: entrambe le mutazioni sparano sulla **riga nuova**, non su quella del conteggio |
| **E18** | **`assert!(crashes > 0)` dettato dal piano è troppo debole, e l'uguaglianza non è pedanteria.** Il punto si estrae in `0..WRITES_PER_RUN` e lo scenario compie esattamente `WRITES_PER_RUN` scritture quando nulla cade: **ogni** seme deve raggiungere il proprio punto, e un solo seme che non cadesse significherebbe che lo scenario ha scritto **meno** del numero contro cui il punto è stato estratto — il no-op silenzioso del gotcha **#17**. `> 0` lascerebbe passare centonovantanove su duecento. Rafforzato ad `assert_eq!(crashes, CAMPAIGN_SEEDS)` e verificato su **cinquantamila** semi, zero eccezioni |
| **E19** | Il piano chiamava `steps_in_doubt` **due volte per seme** — una per l'insieme, una per il ciclo sulla risoluzione — facendo girare `replay()` il doppio per niente. Legata **una volta sola**; nessuna ragione trovata per le due chiamate |
| **E20** | ⛔ **La mutazione C del piano è mal-scopata, e la conferma è una misura.** Sostituisce `without_crash()` nel solo sito di `c7b_…`, ma `a_crash_leaves_more_than_one_step_in_doubt_on_at_least_one_seed` ha **il proprio** sito di chiamata: eseguita alla lettera resta **`ok`** dove il piano la annuncia rossa. Applicata a **entrambi** i siti, `left: 0 / right: 200` |
| **E21** | ⛔ **Lo Step 5 chiedeva di «leggere l'uscita» di numeri che nulla stampava.** `crashes` e il massimo erano locali e solo asseriti, quindi `--nocapture` non avrebbe mostrato niente e la misura per i Task 4 e 8 non sarebbe esistita. Due `println!`, nell'idioma di `measure_and_print_the_interleaving` |
| **E22** | ⚠️ **Quattro capoversi al futuro, resi stantii dal commit stesso che li conteneva** — *«what **will let task 3** assert the RESOLUTION»*, *«that is what **will keep** `C7b` from being a tautology»*, *«held **one task away**»*, *«which **arrives at task 3**, so **until then**…»*. ⛔ E uno sbagliava anche la **relazione**, non solo il tempo: chiamava la sonda dell'interlacciamento *«di `c7b`»* mentre è un `#[test]` **fratello**. Chiusi con richiami datati |
| **E23** | **La premessa dell'uguaglianza di E18 non era tenuta per intero.** È sana solo perché il conteggio delle scritture **non dipende dal seme**, e a tenerlo era `C7a` su un letterale nudo `0..50u64` mentre la campagna ne spazza duecento: **centocinquanta semi su duecento** avevano il conteggio non fissato, e il pin ne usa uno solo. Non era un difetto vivo — zero deviazioni su cinquantamila semi — era una **premessa non sorvegliata**, e il giorno in cui il Task 4 alza i semi il sostegno sarebbe sceso **in silenzio**. Ora entrambe le campagne spazzano `CAMPAIGN_SEEDS` |
| **E24** | **La guardia `contains` di `expected_doubt` era assicurazione morta DERIVATA DALL'IMPLEMENTAZIONE**, e stava quattro capoversi sotto la rivendicazione d'indipendenza dell'oracolo: *«rispecchia ciò che `enter` fa dall'altra parte»*. Oggi gratis perché irraggiungibile; il giorno in cui lo scenario crescesse fino a un secondo intento, l'oracolo concorderebbe con l'implementazione **per costruzione, anche sbagliata** — smetterebbe di essere indipendente **esattamente dove comincia a essere esercitato**. Convertita in **invariante asserito**, col prezzo dichiarato: lo scenario non può più crescere dentro quel caso senza un rosso |
| **E25** | ⚠️ **Una previsione del coordinatore smentita dalla misura, registrata perché il metodo conta più dell'esito.** La mutazione `ACTIVITIES = 1` era attesa produrre collaterali sul pin e su `C7a`, perché `WRITES_PER_RUN` scende da ventiquattro a otto. **Non ne ha prodotti:** la costante è **derivata** — `(ACTIVITIES * STEPS * 2)` — quindi si ricalcola e le due asserzioni confrontano otto con otto. 📌 È **letteralmente il caso che il commento del pin dichiara** (*«fissa una RELAZIONE, non un numero»*, **E12**), e chi ha eseguito lo ha visto perché ha misurato invece di fidarsi. ⚠️ **E lo stesso vale in senso opposto per una revisione:** tre degli otto rilievi che una revisione attribuiva al Task 4 sono **falsi** — il suo Step 3 non cancella `c7b_…`, quindi l'import di `Resolution` non resta inutilizzato e nessun nome viene rimosso. Un rilievo di un revisore si legge contro il piano, come un compito si legge contro il codice |
| **E26** | ⛔ **Il Task 4 è stantio sotto il codice del Task 3, e cinque cose sono misurate.** ⛔ **Il tetto è tarato in `--release` mentre `gate.sh` gira in `debug`:** **4,37 µs** contro **18,8 µs**, cioè **4,3×** — un tetto di un secondo sfora di più di quattro volte dove il cancello lo paga. Le due misure servono entrambe, ma **quella che decide è la `debug`**. ⛔ `campaign()` **duplica il corpo di `c7b_…` meno il ciclo sulla risoluzione e meno la non-vacuità di E17**, e `the_short_campaign_runs_on_every_commit` diventa *«la campagna che gira davvero»*: la **più debole** delle due finirebbe nel cancello. ⛔ Reintroduce `assert!(crashes > 0)` etichettandolo *«la stessa non-vacuità di `C7b`»*, che dopo **E17** e **E18** non lo è più. ⚠️ Introduce `SHORT_CAMPAIGN_SEEDS` ignorando che **`CAMPAIGN_SEEDS` esiste già**. ⚠️ E la regola *«il più grande multiplo di cento sotto il tetto»* massimizza un numero che oltre un certo punto **non compra punti di caduta** — tutti e ventiquattro sono già colpiti da quattro a tredici volte e l'insieme in dubbio **satura a tre**, che è il suo tetto strutturale `= ACTIVITIES`. Compra **solo diversità di interlacciamento**, e il piano deve dirlo invece di massimizzare |

| **E27** | ⛔ **Il tetto del tempo di parete era tarato in `--release` mentre `gate.sh` gira in `debug`.** Misurato: **18,3 µs** contro **4,4**, cioè **4,13×** — un tetto di un secondo sfora di più di quattro volte dove il cancello lo paga. ⚠️ **Servono entrambe le misure e per ragioni diverse:** la `release` per il confronto storico, la **`debug` perché è quella che decide**, e il piano ne teneva una sola |
| **E28** | ⛔ **DECISIONE — `campaign()` diventa il CORPO di `C7b`, e il piano lo metteva accanto.** Dettato così ne **duplicava il corpo meno il ciclo sulla risoluzione e meno i due oracoli di E17/E18**, e la sua sonda sarebbe diventata *«la campagna che gira davvero»*: **la più debole delle due sarebbe finita nel cancello**, con l'altra a girare in parallelo sullo stesso scenario. Ora `C7b` **è** la campagna breve — un ciclo solo a ogni commit |
| **E29** | ⛔ Il piano **reintroduce `assert!(crashes > 0)`** etichettandolo *«la stessa non-vacuità di `C7b`»*. Dopo **E17** e **E18** non lo è più: sono **due** oracoli, e il conteggio da solo è soddisfatto da una campagna che confronta insiemi vuoti |
| **E30** | Il piano introduce `SHORT_CAMPAIGN_SEEDS` ignorando che **`CAMPAIGN_SEEDS` esisteva già** dal Task 3 e lo usavano due cicli. Rinominata, una sola |
| **E31** | ⛔ **Il modello di costo del coordinatore era sbagliato del 37 %, e l'ha smentito una misura di chi eseguiva.** `2 × semi × costo_per_corsa` assume due cicli di costo uguale: non lo sono, perché una corsa che **cade** si ferma al proprio punto (**18,0 µs**) mentre `C7a` arriva in fondo (**31,6 µs**). Il binario costa `semi × 49,6 µs` e il tetto vero cade a **~19 000** semi, non a ~26 000 |
| **E32** | ⛔ **DECISIONE — il numero non si massimizza, e la regola del piano lo faceva.** *«Il più grande multiplo di cento sotto il tetto»* insegue una cifra che **satura**: i punti di caduta sono tutti e ventiquattro coperti già a duecento semi, e l'insieme in dubbio massimo vale **tre** perché il suo tetto è **strutturale** e uguale ad `ACTIVITIES`. Il criterio vero è **la chiusura dello spazio degli esiti**: gli insiemi in dubbio distinti che questa campagna può mai confrontare sono **centonove**, l'ultimo compare al seme **1038**, e ventimila semi in più non ne producono nemmeno uno. Scelti **duemila** — **1,7×** oltre il peggiore dei sei punti di chiusura osservati — che costano **~110 ms**, l'**11 %** del tetto. ⚠️ **Un tetto è un vincolo, non un bersaglio** |
| **E33** | ⛔ **DECISIONE, e presa solo DOPO aver misurato il rischio di falso positivo.** Il criterio di **E32** era l'unico numero portante del file che **nulla teneva**: cresca `ACTIVITIES`, lo spazio si allarga, duemila semi non lo chiudono più e **resta tutto verde mentre la giustificazione muore** — lo stesso guasto che `WRITES_PER_RUN` ha un test per impedire, un piano sopra. ⛔ **Ma una guardia che scatta dove non deve è peggio di una assente** (gotcha **#24**), e il dubbio era se centonove fosse proprietà dello **scenario** o di **quali semi** lo campionano. Misurato su **sei** costanti di mescolamento diverse: **centonove tutte e sei**, e a muoversi è solo *quando* lo spazio si chiude — semi 539, 610, 697, 802, 1038, 1166. Guardia adottata, provata in **due direzioni** |
| **E34** | ⛔ **Il file si contraddiceva a ottantacinque righe di distanza.** Dichiarava *«CONSTRAINT 7 OF §11 IS DISCHARGED HERE»* e, poco sopra, *«the gate does not yet collect it»*. Il vincolo ha **due** metà — il numero fissato e versionato, **e** il tempo di parete stampato *«così l'appesantimento diventa visibile»* — e questo compito ne assolve **una**: la seconda è del **Task 9**. ⚠️ Il costo di lasciarlo: l'audit di chiusura leggerebbe il vincolo come chiuso e il Task 9 come **ridondante** |
| **E35** | ✅ **La divergenza con M-2 è chiusa, e non era una divergenza.** I **25,8 µs** citati come *«il pavimento noto»* non sono confrontabili con nulla che esista oggi, per **tre** ragioni indipendenti: il prototipo **non è nel repository** (M-2 è del 2026-08-07, il workspace nasce l'08 e l'esecutore il 09) · l'esecutore era **un altro**, e l'argomento con cui `executor_determinism.rs` squalifica la cifra dell'**interlacciamento** dello spike squalifica anche quella del **costo**, dove nessuno l'aveva applicato · il protocollo era **un colpo solo** e non una media a caldo, e il rapporto fra le due forme è ~4×. ⚠️ **E la premessa con cui il rilievo fu aperto era sbagliata:** lo scenario di M-2 **il giornale ce l'aveva**, e a farlo leggere altrimenti è la formula *«scenario minimo»* di `design/08`. ✅ La conclusione che la cifra sosteneva **regge ed era per difetto**. Richiami datati scritti; le **tre citazioni nella spec** sono **registrate e non prese** |
| **E36** | ⛔ **Le trappole che la Parte 2 erediterebbe, misurate prima di dispacciarla.** ⛔ **Il Task 6 ha UN SOLO oracolo di non-vacuità**, e il livello 2 è esposto proprio lì: il punto si estrae fra **tutte e sei** le operazioni di `StorageBackend`, **`read` e `len` comprese**, quindi se i punti cadessero tutti su letture l'archivio riaperto sarebbe identico a quello scritto, il confronto a prefisso sarebbe **banalmente vero** e il conteggio degli scatti resterebbe verde. **Il secondo oracolo di livello 2 è: almeno un'iniezione ha prodotto un archivio riaperto strettamente più corto di quanto scritto.** ⚠️ E il confine che risponde davvero al **requisito 4 di ADR-0032** è quello fra **`write` e `sync_data`** — `CountingBackend` tiene già i due contatori separati — mentre un punto estratto **uniformemente** sulle sei operazioni può non visitarlo mai. ⚠️ Infine, due cose del livello 1 **non traslano**: la coppia breve/profonda con `#[ignore]` (il livello 2 è «decine di semi» con I/O vero) e la frase sul confronto **ordinato**, vera solo perché fra scrittura e traccia non c'è `await` — `redb` **riordina**, e la §0.2 del disegno dice **insieme** |

| **E37** | **Il contenuto dettato del Task 5 non compila**, e non per un nome mancante: `let backend = …` **oscura la funzione `backend`** nel namespace dei valori, e il secondo uso nello stesso test la richiama — `E0618: expected function, found CrashingBackend`. Il legame locale è stato rinominato; la funzione conserva il nome, che i passi successivi usano |
| **E38** | ⛔ **DECISIONE — la misura dello Step 3 diventa PERMANENTE, e cancellarla come il piano detta lascerebbe un backend cadente che NIENTE prova cada.** L'unica sonda dettata lo gira a `u64::MAX`, cioè **non cadente**: `may_serve` non prende mai il ramo della caduta, e `fallen`, `syncs` e `falls_at` non sono letti da nessuna asserzione. ⛔ **E il compilatore non può segnalarlo:** i tre campi sono letti da `backend()` stessa per costruire l'oggetto, quindi `dead_code` **tace** — misurato, contro la previsione del coordinatore che si aspettava tre avvisi. Non c'è nessun segnale: solo la sonda mancante. Al suo posto tre sonde permanenti, con `OPERATIONS_TO_OPEN` **misurata** |
| **E39** | ⛔ **BLOCCANTE PER IL TASK 6, trovato scrivendone la sonda in anticipo e misurandola: `syncs > 0` NON è l'oracolo che chiude il gotcha #51.** Sei sync su sette nascono **prima che esista un record** — `create_with_backend` nudo ne fa **sei** in diciannove operazioni — quindi un conteggio assoluto è soddisfatto da un motore che **non sincronizza nessuna scrittura**. ⛔ **Provato:** con `set_durability(Durability::None)` la sonda dettata dal Task 6 resta **VERDE**, mentre un oracolo a **delta** va rosso. La decomposizione è misurata e non riportata — 19/6 → 23/7 → 29/8 → 36/9, cioè **un sync per scrittura**. L'oracolo giusto è `syncs dopo la scrittura > syncs dopo l'apertura`, e vive dove le scritture **riescono**: accanto alla caduta il contatore delle operazioni si muoverebbe con lui e sparerebbe per primo |
| **E40** | ⛔ **Lo scenario di livello 2 SATURA a cinquantotto operazioni** — ventitré per aprire, ventitré per le tre scritture, dodici per il `Drop` — quindi da `falls_at = 58` la caduta **non scatta più** e la corsa è indistinguibile da una senza iniezione. ⚠️ **Il ciclo `23..63` del Task 6 fa scattare trentacinque punti su quaranta e passa lo stesso**, perché il suo oracolo è `fired > 0`; e la campagna profonda del Task 7, a **ottocento** punti, ne fa scattare **sempre e solo trentacinque**: venti volte il costo, **zero** stati in più. Scritto nel doc di `CrashingBackend`, dove il Task 7 sceglierà il proprio intervallo, invece che in una sonda che si cancella |
| **E41** | **Due asserzioni su cinque della sonda centrale non potevano parlare** (gotcha **#55**): `fallen == false` è implicata dall'asserzione sul contatore, che spara sempre prima — l'insieme dei casi «apertura riuscita **e** caduta scattata» è **vuoto**, misurato su ottanta punti tenendo il journal **vivo** — e `syncs > 0` era mascherata dalla stessa accoppiata, perché `sync_data` passa da `may_serve` e ogni calo della sincronizzazione muove anche il conteggio delle operazioni. ⚠️ **La prima misura aveva dato l'esito opposto**, ed era un artefatto: `with_backend(..).is_ok()` **distrugge il journal dentro la stessa istruzione**, e il `Drop` spende dodici operazioni che superano il punto d'iniezione |
| **E42** | **Il doc della costante diceva *«ONE CLEAN OPENING»* senza dire di un archivio VUOTO:** riaprire un archivio **popolato** costa **diciotto**, e su otto sessioni successive 18/19/19/27/18/18/18/18. ⚠️ **E il rosso che quella costante produce era asimmetrico:** troppo alta dà *«l'apertura non è costata quanto misurato»*, perfetto; troppo **bassa** dà *«l'archivio non si apre»*, che è un rosso per il **motivo sbagliato** (gotcha **#9**) — ed è il caso in cui cadrebbe un `redb` la cui apertura costasse **di più**. L'`expect` nomina ora la costante. ✅ **E il rifiuto di allentare a `>=` è ora misurato:** togliendo la guardia da ciascuna delle cinque operazioni, **cinque su cinque** vengono colte e **quattro su cinque solo dal contatore** |
| **E43** | ⚠️ **La riga di mutazione che il Task 6 detta non compila:** `set_durability` vuole `&mut self` — il piano lascia `let transaction` — e restituisce un `Result`, quindi `unused_must_use` |
| **E44** | ⚠️ **Il `catch_unwind` che il Task 6 detta è scritto per un panico che non arriva.** Misurato su `falls_at` da 0 a 59: il `Drop` di `Database` su un backend caduto **non va mai in panico** — ingoia l'errore in silenzio. ⚠️ E il suo `Drop` esegue una **transazione di scrittura intera** sotto `if !thread::panicking()`, quindi chi conterà le operazioni dopo un crash deve sapere che ne costa dodici |
| **E45** | ✅ **Una misura migliore di quella che il piano annota, per il Task 6:** a `falls_at = 28` l'`intent` risponde `Err(NotDurable)` **mentre il record è durevole** — la riapertura lo trova. È il caso più affilato per l'asserzione sul prefisso, meglio del `falls_at = 30` annotato, perché separa *«la porta ha detto no»* da *«l'archivio non ha il record»* |

| **E46** | Il Task 6 **ridichiara `const OPERATIONS_TO_OPEN`**, che il Task 5 ha già scritto col valore misurato: è **`E0428`**, il nome definito due volte |
| **E47** | ⛔ **DECISIONE — il ciclo si ferma alla SATURAZIONE, e il suo oracolo è un'UGUAGLIANZA e non `> 0`.** Il piano detta `OPERATIONS_TO_OPEN + 40` con `fired > 0`: in quell'intervallo **cinque punti su quaranta non scattano** e la sonda passa lo stesso, cioè cinque corse indistinguibili da corse senza iniezione. Fermandosi a cinquantotto ogni punto scatta e l'uguaglianza lo pretende. ✅ **E la conferma è arrivata gratis, dalla mutazione di un altro controllo:** sotto `Durability::None` `fired` scende da trentacinque a **ventotto**, perché senza durabilità `redb` fa **meno I/O** e la saturazione scende da 58 a **~51** — con `> 0` la sonda sarebbe rimasta **verde** |
| **E48** | ⛔ **DECISIONE — il ciclo guadagna un TERZO e un QUARTO oracolo, e il quarto è ciò che lo rende un SECONDO TESTIMONE del #51.** Misurato: con la sola coppia dettata dal piano, e con la saturazione ricalcolata a cinquantuno, `a_crashed_archive_reopens_in_a_coherent_state` è **interamente verde** sotto `Durability::None` — cioè **non contribuisce nulla** alla chiusura del #51, che poggerebbe su **una sonda sola**. ⛔ **`partial > 0`** — quanti punti restituiscono **alcuni ma non tutti** i record — è **rosso** lì, perché senza durabilità la scala di sopravvivenza **collassa a zero-o-tutto**: ventidue punti con zero record, sei con tre, **nessuno** con uno o due. Ed è un rilevatore **indipendente dalla costante fragile**. ⚠️ Più **`truncated < points`**, che è la direzione opposta e mancava: oggi guardavamo solo la deriva **verso il basso**, e se il costo dello scenario **crescesse** il ciclo smetterebbe **in silenzio** di raggiungere la coda restando verde. 📌 E le due chiudono anche una **misura in prosa** — la scala descritta in un commento — che sarebbe invecchiata (**#31**) |
| **E49** | ⚠️ **Il messaggio incolpava il motore per un panico del BANCO.** Con l'iniezione **dentro l'apertura** a panicare è `.expect("open")`, ma l'asserzione riferiva *«the engine panicked on the way down, which it was measured never to do»* — **diagnosi falsa**, esibita dalla mutazione che l'intervallo scende a zero. ⛔ È il gotcha **#9** già pagato a **E42** e corretto nella sonda sorella, **reintrodotto nell'aiutante nuovo**: un rimedio non trasla da solo al codice scritto dopo |
| **E50** | ⚠️ **Il messaggio dell'intervallo nominava una causa su due.** Sotto `Durability::None` il rosso recitava *«the range has drifted past saturation»* mentre **niente era andato alla deriva**: a cambiare era il **pattern di I/O del motore**, cioè la causa per cui quel file esiste. Ora nomina entrambi i candidati, e nell'ordine giusto |
| **E51** | ⚠️ **`.ok()` buttava la parola del motore**, e *«the archive did not reopen»* copriva **due guasti diversi** — l'apertura o il `replay` — contro la convenzione che `file_journal.rs` codifica già (*«un `match`, così i due esiti sbagliati si distinguono»*). ⚠️ **E il secondo `catch_unwind` che il piano detta attorno alla riapertura era sparito dal BRIEF del coordinatore, non dal piano**: registrato dove va. Rimesso, con una ragione **più forte** di quella dettata — quel lato legge un archivio lasciato **lacero** dalla caduta, cioè il posto dove un decodificatore inciampa più facilmente |
| **E52** | ⛔ **DUE RIGHE DEI DOCUMENTI DI STATO erano false su come si chiude il #51, e lo erano dal brainstorming.** Il compendio e l'handoff dicevano *«con `Durability::None` `redb` non chiama `sync_data`, quindi una campagna che pretende «è scattato almeno una volta» diventa rossa»*: sotto quella mutazione `redb` lo chiama **sette volte all'apertura**, e quella forma è **l'oracolo cieco per eccellenza**. ⚠️ Il compendio si **contraddiceva**: la §6 diceva già la cosa giusta. 📌 Il difetto non era il numero ma la **previsione** — quelle celle furono scritte quando il backend cadente **non esisteva**, gotcha **#57** |
| **E53** | ✅ **Il perimetro di ciò che la chiusura del #51 compra è stato SCRITTO invece che sottinteso**, e la parte che non compra è più lunga di quella che compra. Sta in [`riferimenti.md`](../../riferimenti.md) |

| **E54** | ⛔ **DECISIONE — la campagna breve di livello 2 È `a_crashed_archive_reopens_in_a_coherent_state`, e il piano ne dettava una SECONDA, più debole.** Stesso scenario, stesso aiutante, stessa asserzione sul prefisso, ma **un** oracolo invece di cinque — e la sua sonda si sarebbe chiamata *«the short campaign runs on every commit»*, cioè **la più debole sarebbe finita a rappresentare la campagna**. È **E28 ripetuto al livello 2**, e il gotcha **#49**. Ora i cinque controlli vivono in **un corpo solo** che le due profondità chiamano |
| **E55** | ⛔ **DECISIONE — la campagna PROFONDA approfondisce lo SCENARIO, non lo spazzamento.** Il piano detta `DEEP = SHORT × 20` sull'intervallo: misurato, `23..823` sono **ottocento** punti e ne scattano **sempre trentacinque**, perché oltre la saturazione la corsa è indistinguibile da una senza iniezione. ✅ **La profondità invece compra stati nuovi UNO PER RECORD**, e la metrica che lo dimostra non era chiesta da nessuno: i **pioli** — quante lunghezze di prefisso distinte tornano nello spazzamento — sono `record + 1` su `record + 1` **a ogni profondità misurata** (4/4, 11/11, 21/21, 31/31, 41/41). Scelti **trenta** record: 197 punti, 1,35 s, sotto i 3,9 s che la profonda di livello 1 già spende sullo stesso ciclo lungo |
| **E56** | ⛔ **La saturazione NON è lineare, e dedurla sarebbe stato sbagliato.** Misurata: **1** record → 41 operazioni · **3** → 58 · **10** → 100 · **20** → 160 · **30** → 220 · **50** → 340. Il **primo** record costa sei operazioni, il **secondo** sette, il **terzo dieci**, il quarto e il quinto sei, poi sei fisse; e il `Drop` ne costa dodici **tranne a due record, dove ne costa quattordici**. Estrapolare 220 da 58 avrebbe sbagliato |
| **E57** | ⛔ **Il tetto era di nuovo in `--release` mentre `gate.sh` gira in `debug`** — terza volta in questo piano, dopo **E27**. Misurato al livello 2 il fattore è **dodici**: 3,5 ms per iniezione in debug contro 0,29 in release. Il binario intero sta a **0,145 s**, sette volte dentro il tetto |
| **E58** | ⚠️ **Una mutazione ha smentito un commento scritto prima di misurarla, e la smentita vale più del commento.** Era dichiarato che accoppiare una saturazione con la profondità sbagliata fosse *«un no-op silenzioso, ogni asserzione resta verde»*: **è falso**, spara `truncated < points`. ⛔ Quell'oracolo era entrato al Task 6 per la direzione opposta — uno scenario più **caro** — e coglie anche una saturazione troppo **bassa**: 📌 **le due direzioni dello stesso errore sono colte da due oracoli diversi**, e nessuno dei due lo sapeva |
| **E59** | ✅ **`partial > 0` è testimone del gotcha #51 a DUE profondità:** con la durabilità tolta la scala collassa a zero-o-tutto **anche a trenta record**, su centosessantatré punti. Non è un accidente dello scenario piccolo |
| **E60** | ⚠️ **Quattro attese scritte prima della misura, e tutte corrette invece che appianate:** il costo per iniezione (previsto ~1,7 ms, misurato **3,5**), la profondità del collasso sotto `Durability::None` (previsti 313 punti, misurati **163**), la linearità della saturazione (**non** è lineare), e il no-op di **E58**. 📌 Quattro su quattro sbagliate è il dato: su questo file **nessuna previsione ha retto** |
| **E61** | ✅ **La seconda metà del vincolo 7 è ora PRODOTTA** — il tempo di parete si stampa a ogni corsa di entrambe le campagne di livello 2 — e resta al **Task 9** il passo del cancello che la **raccoglie**. Il commento che dichiarava il vincolo assolto *«in parte»* è aggiornato di conseguenza |

| **E62** | ⛔ **L'elenco dei semi era scritto per il SOLO livello 1, e al livello 2 «un seme» non esiste.** Il ciclo di livello 2 è **esaustivo e deterministico** — percorre ogni punto d'iniezione dall'apertura alla saturazione — quindi non c'è niente da estrarre: a identificare un caso sono il **punto d'iniezione** e il **numero di record**. Il file dettato avrebbe versionato la cosa sbagliata, e un lettore avrebbe cercato un seme che non c'è |
| **E63** | ⛔ **Un identificatore non basta a sé stesso, e le due campagne si rompono in modi diversi.** Al livello 1 il seme è **l'indice del ciclo**: abbassare `SHORT_CAMPAIGN_SEEDS` **scarta le voci più alte senza un rosso**, e passare a un intervallo con offset o passo le rende tutte ambigue. Al livello 2 le costanti che rendono interpretabile una voce dipendono da **`redb` 4.1.0**, perché i conteggi di operazioni sono proprietà del **suo layout interno** e non del nostro codice. Entrambe le cose ora sono scritte nel file, in una tabella propria |
| **E64** | ⛔ **Il comando di riproduzione dettato non riproduce un caso solo:** `cargo test … -- --nocapture` lancia **l'intera campagna**, e nessuna delle due prende filtri. Dichiarato invece che aggiustato — si scrive una **sonda usa-e-getta** che chiami direttamente lo scenario — perché comprare un meccanismo di filtro sarebbe superficie che nessuno ha chiesto |
| **E65** | ⚠️ **La riga iniziale diceva *«nessun difetto trovato»*, che è impreciso in un modo che conta:** nessuna **campagna** è fallita, ma il traguardo di difetti ne ha trovati molti — nel **piano** e nei **banchi**, col pre-controllo e con le mutazioni. È la distinzione che dà valore all'elenco, e ora è scritta |
| **E66** | ⚠️ **Il file non nominava la propria voce aperta:** che non abbia un chiudente — nessuno script verifica che una voce nomini un test esistente — è una decisione **registrata e non presa**, del proprietario, e va detta **dentro** il file oltre che nel compendio |

⛔ **E una cosa che questa passata ha deciso di NON fare, registrata perché non venga riscoperta.** Lo scenario giornalato del Task 2 avrebbe potuto guadagnare una sonda propria sull'**interlacciamento**, sulla falsariga di `non_vacuity_the_interleaving_is_real`. Non l'ha guadagnata: il **Task 3** tiene già quella proprietà con `a_crash_leaves_more_than_one_step_in_doubt_on_at_least_one_seed`, che è rossa se lo scenario non interlaccia. Aggiungerla sarebbe stato il gotcha **#49** — duplicare invece di verificare. ⚠️ **Il prezzo, dichiarato:** se quella sonda va rossa, la diagnosi ha **due** candidati — nessun interlacciamento, oppure riconciliazione sbagliata — e il pre-controllo del Task 3 deve provarla **non vacua**.

---

## Prima di eseguire ogni compito

⛔ **Un piano è un'ipotesi.** Il pre-controllo di ogni compito, **prima** di dispacciarlo, ha trovato almeno un difetto reale in dodici compiti su dodici del Traguardo 3. Si fanno **quattro domande**, e ciascuna coglie ciò che le altre tre non colgono:

| | Il difetto | Che cosa lo coglie |
|---|---|---|
| 1 | la **sonda è sbagliata** — vacua, o attacca il caso invece del meccanismo | **rileggere** |
| 2 | la **sonda manca** | *per ogni artefatto che il compito produce, quale controllo lo esercita?* |
| 3 | l'**artefatto è sbagliato**, e compila | **solo** scriverne un'implementazione **da fuori dalla crate** |
| 4 | il **compito è già eseguito** | *ciò che detta di produrre esiste già?* |

⛔ **E la quinta, che non sta nell'elenco: il contratto cresce sotto il piano.** Un compito scritto oggi si legge **contro il codice di quel giorno**, non contro questo file. ⚠️ **È già successo scrivendo questo piano:** due affermazioni del disegno erano sbagliate e le ha trovate il codice, non il ragionamento — §11 del disegno.

⛔ **E il banco sbaglia verso l'attesa** (gotcha #48). Per ogni mutazione: provare che **si sia applicata**, compilare in un passo **separato** dall'eseguire, e per ogni mutazione su un valore **provarne due**.

## Vincoli globali

| # | Vincolo |
|---|---|
| **G1** | ⛔ **Codice in inglese, documentazione in italiano** — §1.0 della spec. Nomi di crate, moduli, tipi, funzioni, messaggi d'uscita e **commenti nel sorgente** in inglese |
| **G2** | ⛔ **I byte congelati non si toccano.** `crates/kernel/tests/frozen/` e `crates/kernel/src/record.rs` restano invariati: nessun campo nuovo, nessun indice nuovo, nessuna variante nuova nei tre enum `index_only` |
| **G3** | ⛔ **`bash scripts/gate.sh` deve dare `GATE GREEN` a ogni commit.** Un compito che chiuderebbe con la porta rossa non si chiude: si fonde col successivo |
| **G4** | **TDD.** Prima la sonda che fallisce, poi il minimo che la fa passare. E la sonda si **vede fallire**, non si presume |
| **G5** | ⛔ **Un controllo si prova in due direzioni** (gotcha #24): che scatti dove deve, **e che non scatti dove non deve** |
| **G6** | **Si committa e si pusha alla fine di ogni compito**, senza chiedere e **senza co-autore** |
| **G7** | ⛔ **`HashMap` è vietato** in `kernel` e `simulator` (gotcha #12): `BTreeMap`, o un `Vec` |
| **G8** | ⛔ **I fine-riga sono misti per file.** Nessuno strumento riscrive un file che non stia già modificando |

## Decisioni prese da questo piano

Sono decisioni che la spec e il disegno non fissano, e che costavano zero a prendere qui. **Il proprietario può ribaltarle vedendole.**

| # | Decisione | Perché |
|---|---|---|
| **D1** | `CrashingJournal` **avvolge** `MemoryJournal` invece di duplicarlo | duplicare il doppio in memoria creerebbe due verità da tenere allineate, e la sola differenza è il punto di caduta |
| **D2** | la caduta risponde `JournalError::NotDurable`, e **da lì in poi ogni scrittura è rifiutata** | un giornale che rifiuta una volta e poi riprende modella un disco cattivo, non un crash. È ciò che fa fermare **tutte** le attività interlacciate e non solo quella che ha toccato il confine |
| **D3** | la riapertura si modella con `into_survivor(self) -> MemoryJournal`; le letture **delegano**, ed è un **limite dichiarato** | la campagna non usa mai le letture del tipo cadente: chiama `into_survivor`. Le letture ci sono perché `Journal` le richiede |
| **D4** | il punto di caduta si estrae da un generatore **diverso** da quello dell'interlacciamento, con seme **derivato** | due `SeededRng` costruiti dallo **stesso** numero producono la **stessa** sequenza: il punto di caduta sarebbe una funzione della prima mescolata, e la campagna esplorerebbe una **diagonale** dello spazio invece dello spazio |
| **D5** | il numero di semi si **misura**, non si sceglie | il vincolo 7 della §11 lo vuole *«fissato e versionato»*, non *«grande»*. Il criterio è un tetto dichiarato di tempo di parete |
| **D6** | la campagna è **un test**; il cancello **non guadagna un settimo controllo** | `cargo test --workspace` è già il secondo controllo, quindi la cadenza *«a ogni commit»* del vincolo 8 è già imposta. Il cancello guadagna un **passo di stampa** per il tempo di parete, che non è un controllo |
| **D7** | il **livello 1 prima del livello 2** | il livello 1 non ha I/O, chiude `C7a` e `C7b` — che sono la ragione per cui il traguardo esiste — e il livello 2 poggia sul vocabolario che il livello 1 stabilisce |

## La struttura dei file

| File | Responsabilità |
|---|---|
| `crates/simulator/src/journal.rs` — **modifica** | guadagna `CrashingJournal` accanto a `MemoryJournal`, che resta intatto |
| `crates/simulator/tests/crashing_journal.rs` — **nuovo** | le promesse del **solo** tipo cadente. Quelle di ogni giornale restano nella conformità |
| `crates/simulator/tests/dst_campaign.rs` — **nuovo** | lo scenario giornalato, `C7a`, `C7b`, la campagna breve e quella profonda |
| `crates/platform/tests/engine_crash_consistency.rs` — **nuovo** | `CrashingBackend` e la campagna di livello 2 |
| `crates/platform/tests/file_journal.rs` — **modifica** | data il commento che rimandava al Traguardo 4 |
| `docs/semi-dst.md` — **nuovo** | l'elenco versionato dei semi, e la regola per cui non è un oracolo |
| `scripts/gate.sh` — **modifica** | il passo di stampa del tempo di parete |
| `docs/porta-di-qualita.md` — **modifica** | una riga per ogni controllo nuovo, e i conteggi **ricontati** |

---

# Parte 1 — il livello 1: la porta che cade

## Task 1: `CrashingJournal`

**Files:**
- Modify: `crates/simulator/src/journal.rs`
- Test: `crates/simulator/tests/crashing_journal.rs` (create)

- [ ] **Step 1: scrivere le sonde che falliscono**

Crea `crates/simulator/tests/crashing_journal.rs`:

```rust
//! `CrashingJournal`: what only IT promises. What EVERY journal promises is the conformance
//! suite's business — `crates/kernel/tests/journal_contract.rs` — and this type is not held to
//! it, deliberately: a journal that stops working is a LIAR by construction, and gotcha #50
//! says a fake may break a contract when the test speaks about the breaking.

use kernel::ports::journal::{Journal, JournalError, StepId};
use simulator::journal::CrashingJournal;

const WRITES: u64 = 8;

#[test]
fn it_falls_at_the_write_it_was_told_to_fall_at() {
    // ⛔ NOT "it falls somewhere": at THE write. The number is handed in rather than drawn, so
    // this probe does not depend on the generator — that is `the_same_seed_chooses_the_same_
    // write`'s job.
    let mut journal = CrashingJournal::falling_at(2);

    assert_eq!(journal.intent(StepId::new(1), b"one"), Ok(()));
    assert_eq!(journal.outcome(StepId::new(1), b"one done"), Ok(()));
    assert_eq!(
        journal.intent(StepId::new(2), b"two"),
        Err(JournalError::NotDurable)
    );
}

#[test]
fn after_the_fall_every_later_write_is_refused_too() {
    // ⛔ THE DIFFERENCE BETWEEN A CRASH AND A BAD DISK, and it is decision D2. A journal that
    // refused once and then worked again would let the other interleaved activities carry on
    // writing after the process was supposed to be gone.
    let mut journal = CrashingJournal::falling_at(0);

    assert_eq!(
        journal.intent(StepId::new(1), b"one"),
        Err(JournalError::NotDurable)
    );
    assert_eq!(
        journal.intent(StepId::new(2), b"two"),
        Err(JournalError::NotDurable)
    );
    assert_eq!(
        journal.outcome(StepId::new(2), b"two done"),
        Err(JournalError::NotDurable)
    );
    assert_eq!(
        journal.note(StepId::new(2), b"a note"),
        Err(JournalError::NotDurable)
    );
}

#[test]
fn what_was_written_before_the_fall_survives() {
    // The archive the reconciliation will read after the restart.
    let mut journal = CrashingJournal::falling_at(1);
    assert_eq!(journal.intent(StepId::new(1), b"one"), Ok(()));
    assert_eq!(
        journal.outcome(StepId::new(1), b"one done"),
        Err(JournalError::NotDurable)
    );

    let survivor = journal.into_survivor();
    assert_eq!(
        survivor.replay().expect("replay"),
        vec![(StepId::new(1), b"one".to_vec())]
    );
}

#[test]
fn a_journal_told_not_to_crash_never_falls() {
    // ⛔ THE OTHER DIRECTION (rule 3 of §7.1.1): a control that fires where it must not is
    // worse than one that is absent. C7a rests entirely on this one.
    let mut journal = CrashingJournal::without_crash();
    for step in 0..64u64 {
        assert_eq!(journal.intent(StepId::new(step), b"i"), Ok(()), "step {step}");
        assert_eq!(journal.outcome(StepId::new(step), b"o"), Ok(()), "step {step}");
    }
    assert!(!journal.has_fallen());
    assert_eq!(journal.writes_done(), 128);
}

#[test]
fn the_same_seed_chooses_the_same_write() {
    let first = CrashingJournal::from_seed(99, WRITES);
    let second = CrashingJournal::from_seed(99, WRITES);
    assert_eq!(first.falls_at(), second.falls_at());
}

#[test]
fn the_drawn_point_lies_inside_the_writes_the_scenario_performs() {
    // ⛔ GOTCHA #17: injecting a fault where the code never arrives is a VACUOUS proof that
    // looks like a success. If the point could land past the last write, some seeds would
    // simply never crash and the campaign would report green for having done nothing.
    for seed in 0..500u64 {
        let point = CrashingJournal::from_seed(seed, WRITES).falls_at();
        assert!(point < WRITES, "seed {seed} drew {point}, outside 0..{WRITES}");
    }
}

#[test]
fn different_seeds_choose_different_points() {
    // ⛔ AND THE OTHER HALF OF #17: a point that never moves would make five hundred seeds one
    // single experiment repeated.
    let mut seen = std::collections::BTreeSet::new();
    for seed in 0..500u64 {
        seen.insert(CrashingJournal::from_seed(seed, WRITES).falls_at());
    }
    assert!(seen.len() > 1, "the point never moves: {} distinct", seen.len());
}

#[test]
fn has_fallen_says_no_until_it_falls() {
    // The campaign's non-vacuity oracle: without it, "the run produced no doubt" and "the
    // crash never fired" are the same green.
    let mut journal = CrashingJournal::falling_at(1);
    assert!(!journal.has_fallen());
    assert_eq!(journal.intent(StepId::new(1), b"one"), Ok(()));
    assert!(!journal.has_fallen());
    assert_eq!(
        journal.outcome(StepId::new(1), b"one done"),
        Err(JournalError::NotDurable)
    );
    assert!(journal.has_fallen());
}
```

- [ ] **Step 2: eseguirle e vederle fallire**

```bash
cargo test -p simulator --test crashing_journal
```

Atteso: **non compila**, con `E0432: unresolved import` su `simulator::journal::CrashingJournal`. ⛔ Se compila, il tipo esiste già: fermarsi e rileggere il compito (quarta domanda del pre-controllo).

- [ ] **Step 3: scrivere il tipo**

In `crates/simulator/src/journal.rs`, in coda al file, dopo `impl Journal for MemoryJournal`:

```rust
/// A journal that STOPS EXISTING at a write chosen by the seed — §3.3, and level 1 of the two
/// crash levels of ADR-0032.
///
/// ⛔ IT IS NOT AN ERROR CHANNEL, AND THAT DIFFERENCE IS THE WHOLE POINT. A journal that
/// answered `NotDurable` once and worked again afterwards would model A BAD DISK, not a crash:
/// a dead process does not come back. So the first refusal is PERMANENT, and every later write
/// is refused too — which is what makes all the interleaved activities of the campaign stop,
/// and not only the one that happened to touch the boundary.
///
/// ⚠️ DECLARED LIMIT, so this doc promises no more than it delivers: the READS delegate to the
/// surviving archive rather than refusing. The campaign never uses them — it calls
/// `into_survivor`, which models reopening the archive after the restart — and they are here
/// because `Journal` requires them.
///
/// ⚠️ IT IS NOT HELD TO THE CONFORMANCE SUITE, and that is deliberate rather than an omission:
/// this type is a LIAR by construction, and gotcha #50 says a fake may break a contract when
/// the test around it speaks about the breaking. Its own promises live in
/// `crates/simulator/tests/crashing_journal.rs`.
pub struct CrashingJournal {
    inner: MemoryJournal,
    falls_at: u64,
    writes: u64,
    fallen: bool,
}

impl CrashingJournal {
    /// Falls at the write with this index, counting from zero.
    pub const fn falling_at(write: u64) -> Self {
        CrashingJournal {
            inner: MemoryJournal::new(),
            falls_at: write,
            writes: 0,
            fallen: false,
        }
    }

    /// Falls at a write DRAWN from the seed, inside `0..expected_writes`.
    ///
    /// ⛔ `expected_writes` IS HOW MANY WRITES THE SCENARIO REALLY PERFORMS, counted rather
    /// than guessed. Gotcha #17: a point drawn past the last write never fires, and a campaign
    /// whose fault never arrives reports green for having done nothing.
    pub fn from_seed(seed: u64, expected_writes: u64) -> Self {
        let mut rng = SeededRng::new(seed);
        Self::falling_at(rng.below(expected_writes))
    }

    /// Never falls. It is what `C7a` — no crash, no false doubt — is measured against.
    pub const fn without_crash() -> Self {
        Self::falling_at(u64::MAX)
    }

    /// The write it will fall at.
    pub const fn falls_at(&self) -> u64 {
        self.falls_at
    }

    /// Whether it HAS fallen. ⛔ The campaign's non-vacuity oracle: without it, "this run left
    /// no doubt" and "the crash never fired" are the same green.
    pub const fn has_fallen(&self) -> bool {
        self.fallen
    }

    /// How many writes reached the archive.
    pub const fn writes_done(&self) -> u64 {
        self.writes
    }

    /// The archive that survived, as a journal that works. It models REOPENING after the
    /// restart, which is the only way the reconciliation ever meets a crashed archive.
    pub fn into_survivor(self) -> MemoryJournal {
        self.inner
    }

    /// Whether this write may proceed, MARKING the fall when it may not.
    ///
    /// ⚠️ It is asked BEFORE delegating, and the counter moves only on an `Ok` from the inner
    /// journal: a write the write-ahead protocol refuses (`OutOfOrder`) never reached storage,
    /// so it must not consume a position in the count the crash point is drawn against.
    fn may_write(&mut self) -> bool {
        if self.fallen {
            return false;
        }
        if self.writes == self.falls_at {
            self.fallen = true;
            return false;
        }
        true
    }
}

impl Journal for CrashingJournal {
    fn intent(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        if !self.may_write() {
            return Err(JournalError::NotDurable);
        }
        let outcome = self.inner.intent(step, record);
        if outcome.is_ok() {
            self.writes += 1;
        }
        outcome
    }

    fn outcome(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        if !self.may_write() {
            return Err(JournalError::NotDurable);
        }
        let outcome = self.inner.outcome(step, record);
        if outcome.is_ok() {
            self.writes += 1;
        }
        outcome
    }

    fn note(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        if !self.may_write() {
            return Err(JournalError::NotDurable);
        }
        let outcome = self.inner.note(step, record);
        if outcome.is_ok() {
            self.writes += 1;
        }
        outcome
    }

    fn read_back(&self, step: StepId) -> Result<Vec<u8>, JournalError> {
        self.inner.read_back(step)
    }

    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError> {
        self.inner.replay()
    }

    fn prune(&mut self, step: StepId) -> Result<(), JournalError> {
        self.inner.prune(step)
    }
}
```

E in testa allo stesso file, accanto agli `use` esistenti, aggiungi:

```rust
use crate::rng::SeededRng;
use kernel::rng::RngExt;
```

⚠️ **Servono entrambi i nomi:** `below` vive su `RngExt` e non su `Rng`, e un metodo raggiunto per blanket impl vuole il **tratto in scope**. È il costo dichiarato in `crates/kernel/src/rng.rs`.

- [ ] **Step 4: eseguirle e vederle passare**

```bash
cargo test -p simulator --test crashing_journal
```

Atteso: `test result: ok. 8 passed; 0 failed`.

- [ ] **Step 5: le due mutazioni, in due direzioni**

⛔ **Ciascuna si applica, si compila in un passo separato, e poi si esegue** — gotcha #48.

Mutazione A — `may_write` risponde sempre `true` (il giornale non cade mai):

```rust
    fn may_write(&mut self) -> bool {
        true
    }
```

```bash
cargo build -p simulator --tests
cargo test -p simulator --test crashing_journal
```

Atteso: **rosso** su `it_falls_at_the_write_it_was_told_to_fall_at`, `after_the_fall_every_later_write_is_refused_too`, `what_was_written_before_the_fall_survives` e `has_fallen_says_no_until_it_falls`.

Mutazione B — `may_write` risponde sempre `false` (cade sempre):

```rust
    fn may_write(&mut self) -> bool {
        false
    }
```

```bash
cargo build -p simulator --tests
cargo test -p simulator --test crashing_journal
```

Atteso: **rosso** su `a_journal_told_not_to_crash_never_falls`. ⛔ **È la direzione che si dimentica**, ed è quella su cui poggia `C7a`.

Ripristina il corpo vero e riesegui: verde.

- [ ] **Step 6: il cancello, e il commit**

```bash
bash scripts/gate.sh
```

Atteso: `GATE GREEN`.

```bash
git add crates/simulator/src/journal.rs crates/simulator/tests/crashing_journal.rs
git commit -m "feat(simulator): il giornale che cade, e la caduta non si riprende"
git push
```

---

## Task 2: lo scenario giornalato, e C7a

**Files:**
- Test: `crates/simulator/tests/dst_campaign.rs` (create)

- [ ] **Step 1: scrivere lo scenario e la sonda che fallisce**

Crea `crates/simulator/tests/dst_campaign.rs`:

```rust
//! The DST campaign — level 1 of the two crash levels (ADR-0032): the subject under test is
//! THE KERNEL'S RECONCILIATION, and nothing here touches a disk.
//!
//! ⚠️ C1, C2, C3 and non-vacuity are NOT here: they are permanent tests since milestone 2, in
//! `crates/kernel/tests/executor_determinism.rs`. This milestone brings the FAULT.

use core::cell::RefCell;

use kernel::executor::{Executor, Sleep};
use kernel::parameters::Parameters;
use kernel::ports::journal::{Journal, StepId};
use kernel::reconcile::{Resolution, steps_in_doubt};
use kernel::record::{EffectClass, Record, RecordKind, RecordV1, Trust};
use kernel::time::Monotonic;
use simulator::journal::CrashingJournal;
use simulator::reactor::VirtualReactor;
use simulator::rng::SeededRng;

const TURN_LIMIT: u64 = 10_000;
const ACTIVITIES: usize = 3;
const STEPS: usize = 4;

/// How many writes the scenario performs when nothing falls: two per step — the intent and the
/// outcome — which is the cost ADR-0007 accepts for the write-ahead discipline.
///
/// ⛔ IT IS PINNED BY A TEST rather than trusted, because the crash point is drawn BELOW this
/// number: were the scenario to perform fewer writes, the tail of the range would never fire
/// and those seeds would be silent no-ops. Gotcha #17.
const WRITES_PER_RUN: u64 = (ACTIVITIES * STEPS * 2) as u64;

/// ⛔ THE CRASH POINT IS DRAWN FROM A DIFFERENT GENERATOR THAN THE INTERLEAVING, and from a
/// seed DERIVED from this one rather than from the same number. Two `SeededRng` built from the
/// same seed produce the SAME sequence, so the crash point would be a function of the first
/// shuffle and the campaign would explore a DIAGONAL of the space instead of the space.
/// Decision D4 of the plan.
const CRASH_SEED_OFFSET: u64 = 0x9E37_79B9_7F4A_7C15;

fn crash_seed(seed: u64) -> u64 {
    seed ^ CRASH_SEED_OFFSET
}

/// A record of the shape every step of this scenario writes.
///
/// ⚠️ The class is `Idempotent` for every step, so the resolution the reconciliation must
/// answer is `RunAgain` — a single expected value, which is what lets the campaign assert the
/// RESOLUTION and not only the set.
fn record(kind: RecordKind) -> Vec<u8> {
    Record::V1(RecordV1 {
        kind,
        effect: EffectClass::Idempotent,
        trust: Trust::Instruction,
        payload: Vec::new(),
        reason: String::from("a step of the DST scenario"),
    })
    .encode()
}

/// What the scenario SUCCEEDED in writing, in order. It is the independent oracle of `C7b`.
///
/// ⛔ IT COMES FROM THE SCENARIO AND NOT FROM THE ARCHIVE, and that is what keeps `C7b` from
/// being a tautology: `steps_in_doubt` walks the DECODED archive, this walks what the activities
/// were told went through. A journal that dropped a record, or a decode that misread `kind`,
/// makes the two disagree.
type Trace = Vec<(u64, RecordKind)>;

/// The M-2 scenario, now journalled: 3 activities x 4 steps, each step writing its intent,
/// waiting 5000 VIRTUAL milliseconds, then writing its outcome.
fn run(seed: u64, journal: CrashingJournal) -> (CrashingJournal, Trace) {
    let journal = RefCell::new(journal);
    let trace: RefCell<Trace> = RefCell::new(Vec::new());
    let sleep = Sleep::new();
    let mut executor = Executor::new(
        SeededRng::new(seed),
        VirtualReactor::new(),
        Parameters::new(TURN_LIMIT),
        &sleep,
    );

    for activity in 0..ACTIVITIES {
        let journal = &journal;
        let trace = &trace;
        let sleep = &sleep;
        executor.spawn(async move {
            for step in 0..STEPS {
                let id = (activity * STEPS + step) as u64;

                // ⛔ AN ERROR FROM THE JOURNAL IS THE PROCESS DYING, not a case to handle.
                // The activity returns and writes nothing more — and since the journal refuses
                // everything after the first fall (decision D2), so do all the others.
                if journal
                    .borrow_mut()
                    .intent(StepId::new(id), &record(RecordKind::Intent))
                    .is_err()
                {
                    return;
                }
                trace.borrow_mut().push((id, RecordKind::Intent));

                // Suspend on a PORT: the reactor is the only thing that can bring this activity
                // back. §2.4.1.
                sleep.until(Monotonic::from_millis(((step as u64) + 1) * 5_000));
                Yield::once().await;

                if journal
                    .borrow_mut()
                    .outcome(StepId::new(id), &record(RecordKind::Outcome))
                    .is_err()
                {
                    return;
                }
                trace.borrow_mut().push((id, RecordKind::Outcome));
            }
        });
    }

    executor.run().expect("the scenario terminates");
    // ⛔ Dropped EXPLICITLY: the tasks hold boxed futures that borrow the two cells, and a boxed
    // trait object carries drop glue, so `into_inner` would not compile otherwise.
    drop(executor);
    (journal.into_inner(), trace.into_inner())
}

/// A future that returns `Pending` exactly once. It is how an activity hands control back to
/// the executor after declaring a suspension.
struct Yield(bool);

impl Yield {
    fn once() -> Self {
        Yield(false)
    }
}

impl core::future::Future for Yield {
    type Output = ();
    fn poll(
        mut self: core::pin::Pin<&mut Self>,
        _context: &mut core::task::Context<'_>,
    ) -> core::task::Poll<()> {
        if self.0 {
            core::task::Poll::Ready(())
        } else {
            self.0 = true;
            core::task::Poll::Pending
        }
    }
}

#[test]
fn the_scenario_really_writes_what_the_campaign_assumes() {
    // ⛔ GOTCHA #17, and it is asserted rather than commented: the crash point is drawn below
    // `WRITES_PER_RUN`, so if the scenario performed fewer writes the tail of the range would
    // be silent and those seeds would prove nothing while passing.
    let (journal, trace) = run(20_260_806, CrashingJournal::without_crash());
    assert_eq!(journal.writes_done(), WRITES_PER_RUN);
    assert_eq!(trace.len() as u64, WRITES_PER_RUN);
}

#[test]
fn c7a_without_a_crash_no_step_is_in_doubt() {
    // ⛔ NO FALSE POSITIVES. It is the half that is easy to skip, and the one that says the
    // doubt reported by C7b means something.
    for seed in 0..50u64 {
        let (journal, _) = run(seed, CrashingJournal::without_crash());
        let survivor = journal.into_survivor();
        assert_eq!(
            steps_in_doubt(&survivor).expect("replay"),
            Vec::new(),
            "seed {seed} left a doubt with no crash"
        );
    }
}
```

- [ ] **Step 2: eseguirle e vederle fallire**

```bash
cargo test -p simulator --test dst_campaign
```

Atteso: **non compila** — il file è nuovo, quindi il primo errore utile è la compilazione dello scenario. ⛔ Se compila e passa al primo colpo, **fermarsi**: significa che nulla di ciò che il compito dichiara di provare sta davvero girando, e va riletto lo scenario prima di andare avanti.

- [ ] **Step 3: far compilare, e vedere il verde**

Non c'è codice di prodotto da scrivere: questo compito è **tutto banco**. Correggi gli errori di compilazione che escono e riesegui:

```bash
cargo test -p simulator --test dst_campaign
```

Atteso: `test result: ok. 2 passed; 0 failed`.

- [ ] **Step 4: provare che `C7a` non sia vacua**

⛔ **Un verde su un giornale che non cade mai può voler dire due cose:** che la riconciliazione è giusta, o che lo scenario non scrive niente. Le si separa rompendo lo scenario, non la riconciliazione.

Mutazione — togli la scrittura dell'esito, lasciando l'intento:

```rust
                // if journal
                //     .borrow_mut()
                //     .outcome(StepId::new(id), &record(RecordKind::Outcome))
                //     .is_err()
                // {
                //     return;
                // }
                // trace.borrow_mut().push((id, RecordKind::Outcome));
```

```bash
cargo build -p simulator --tests
cargo test -p simulator --test dst_campaign
```

Atteso: **rosso su entrambe** — `the_scenario_really_writes_what_the_campaign_assumes` perché i conteggi scendono a metà, e `c7a_without_a_crash_no_step_is_in_doubt` perché **dodici** passi restano in dubbio.

Ripristina e riesegui: verde.

- [ ] **Step 5: il cancello, e il commit**

```bash
bash scripts/gate.sh
```

Atteso: `GATE GREEN`.

```bash
git add crates/simulator/tests/dst_campaign.rs
git commit -m "test(simulator): lo scenario scrive davvero, e senza crash nessun passo è in dubbio"
git push
```

---

## Task 3: C7b — il crash lascia **quell'insieme e non un altro**

**Files:**
- Modify: `crates/simulator/tests/dst_campaign.rs`

- [ ] **Step 1: scrivere l'oracolo indipendente e la sonda che fallisce**

Aggiungi in coda a `crates/simulator/tests/dst_campaign.rs`:

```rust
/// The steps left with an intent and no outcome, computed FROM THE SCENARIO'S TRACE.
///
/// ⛔ WHY THIS IS NOT A TAUTOLOGY, said here because it looks like one: the algorithm is the
/// same shape as `steps_in_doubt`, but the INPUT is not. This walks what the activities were
/// told went through; `steps_in_doubt` walks the bytes that came back out of the archive, after
/// decoding. A journal that lost a record, an encode that dropped a field, a decode that
/// misread `kind` — each makes the two disagree, and none of them would show if the expectation
/// were computed from the archive.
fn expected_doubt(trace: &Trace) -> Vec<u64> {
    let mut open: Vec<u64> = Vec::new();
    for (step, kind) in trace {
        match kind {
            RecordKind::Intent => {
                if !open.contains(step) {
                    open.push(*step);
                }
            }
            RecordKind::Outcome => open.retain(|s| s != step),
            RecordKind::Note => {}
        }
    }
    open
}

#[test]
fn c7b_a_crash_leaves_exactly_the_steps_the_scenario_left_open() {
    // ⛔ THE SET AND NOT ITS SIZE. Measured on the spike, seed 99 left `[3, 7]`: with
    // interleaved execution one crash leaves SEVERAL steps in doubt together, and a bench that
    // compared only how many would pass on the wrong ones. Gotcha #30, and #20.
    let mut crashes = 0usize;

    for seed in 0..200u64 {
        let (journal, trace) = run(seed, CrashingJournal::from_seed(crash_seed(seed), WRITES_PER_RUN));
        let fell = journal.has_fallen();
        let point = journal.falls_at();
        let survivor = journal.into_survivor();

        let expected = expected_doubt(&trace);
        let found: Vec<u64> = steps_in_doubt(&survivor)
            .expect("replay")
            .iter()
            .map(|d| d.step.get())
            .collect();

        assert_eq!(found, expected, "seed {seed}, crash at write {point}");

        // ⛔ EVERY STEP OF THIS SCENARIO DECLARES `Idempotent`, so the resolution is decided and
        // not incidental. Without this the campaign would hold WHICH steps are in doubt and say
        // nothing about WHAT TO DO with them, which is the half ADR-0007 exists for.
        for doubt in steps_in_doubt(&survivor).expect("replay") {
            assert_eq!(
                doubt.resolution,
                Resolution::RunAgain,
                "seed {seed}, step {}",
                doubt.step.get()
            );
        }

        if fell {
            crashes += 1;
        }
    }

    // ⛔ THE NON-VACUITY, AND IT IS THE POINT OF THE WHOLE TEST. Without it, two hundred seeds
    // that all failed to crash would pass this test in silence — a campaign reporting green for
    // having done nothing. Gotcha #17.
    assert!(crashes > 0, "no seed produced a crash: the campaign measured nothing");
}

#[test]
fn a_crash_leaves_more_than_one_step_in_doubt_on_at_least_one_seed() {
    // ⛔ FINDING 2 OF §3.6.1, held on the campaign rather than on a hand-built archive. The
    // in-tree probe `a_crash_leaves_more_than_one_step_in_doubt` in
    // `crates/kernel/tests/reconciliation.rs` builds the state BY HAND; this one gets there
    // through the executor, which is the only way to know the interleaving really produces it.
    let mut best = 0usize;
    for seed in 0..200u64 {
        let (journal, _) = run(seed, CrashingJournal::from_seed(crash_seed(seed), WRITES_PER_RUN));
        let survivor = journal.into_survivor();
        best = best.max(steps_in_doubt(&survivor).expect("replay").len());
    }
    assert!(best > 1, "no seed left more than one step in doubt: {best}");
}
```

- [ ] **Step 2: eseguirle e vederle fallire**

```bash
cargo test -p simulator --test dst_campaign
```

Atteso: **non compila**, `E0599: no method named get found for struct StepId` **oppure** verde. ⛔ **Se è verde, va provato in negativo prima di crederci** — vai allo Step 4 e torna qui.

⚠️ `StepId::get()` **esiste** dal Task 8 del Traguardo 3: se l'errore è quello, il compito è stantio e va riletto contro il codice.

- [ ] **Step 3: far passare**

Nessun codice di prodotto. Correggi la compilazione e riesegui:

```bash
cargo test -p simulator --test dst_campaign
```

Atteso: `test result: ok. 4 passed; 0 failed`.

- [ ] **Step 4: le tre mutazioni, e nessuna è sullo stesso pezzo**

⛔ **Ciascuna si applica, si compila in un passo separato, si esegue** — gotcha #48. E lo Step 3 del Task 9 del Traguardo 3 insegna che **una direzione sola proverebbe una promessa su quattro**.

Mutazione A — la riconciliazione non riporta niente. In `crates/kernel/src/reconcile.rs`, in `steps_in_doubt`, sostituisci il corpo del ramo `RecordKind::Intent`:

```rust
                RecordKind::Intent => {}
```

```bash
cargo build --workspace --tests
cargo test -p simulator --test dst_campaign
```

Atteso: **rosso** su `c7b_...` e su `a_crash_leaves_more_than_one_step_in_doubt_on_at_least_one_seed`. Ripristina.

Mutazione B — la risoluzione è sempre la stessa. In `crates/kernel/src/reconcile.rs`:

```rust
fn resolution_of(_class: EffectClass) -> Resolution {
    Resolution::SuspendAndAsk
}
```

```bash
cargo build --workspace --tests
cargo test -p simulator --test dst_campaign
```

Atteso: **rosso** sull'asserzione della risoluzione dentro `c7b_...`. ⛔ Senza questa mutazione, quel blocco potrebbe essere cancellato senza che nulla diventi rosso — gotcha **#45**. Ripristina.

Mutazione C — il crash non scatta mai. In `crates/simulator/tests/dst_campaign.rs`, sostituisci nella chiamata di `c7b_...`:

```rust
        let (journal, trace) = run(seed, CrashingJournal::without_crash());
```

```bash
cargo build -p simulator --tests
cargo test -p simulator --test dst_campaign
```

Atteso: **rosso** su `assert!(crashes > 0, ...)` con il messaggio *«no seed produced a crash»*, e **rosso** su `a_crash_leaves_more_than_one_step_in_doubt_on_at_least_one_seed`. ⛔ **È la mutazione che conta**: prova che il verde delle altre due non viene dall'assenza di guasti. Ripristina.

- [ ] **Step 5: registrare i numeri misurati, non attesi**

Esegui e **leggi l'uscita**:

```bash
cargo test -p simulator --test dst_campaign -- --nocapture
```

Annota nello scratchpad, per il Task 8: quanti semi su duecento hanno prodotto un crash, e il massimo di passi in dubbio visto. ⛔ **Non scriverli in un commento prima di averli letti** — gotcha #15.

- [ ] **Step 6: il cancello, e il commit**

```bash
bash scripts/gate.sh
```

Atteso: `GATE GREEN`.

```bash
git add crates/simulator/tests/dst_campaign.rs
git commit -m "test(simulator): il crash lascia quell'insieme e non un altro, e l'oracolo non viene dall'archivio"
git push
```

---

## Task 4: la campagna breve — il numero di semi **misurato**, e il tempo di parete

**Files:**
- Modify: `crates/simulator/tests/dst_campaign.rs`

- [ ] **Step 1: misurare il costo di un seme, prima di scegliere quanti**

⛔ **Decisione D5: il numero si misura, non si sceglie.** Aggiungi una sonda temporanea in coda al file:

```rust
#[test]
fn measure_the_cost_of_one_seed() {
    let started = std::time::Instant::now();
    for seed in 0..200u64 {
        let (journal, _) = run(seed, CrashingJournal::from_seed(crash_seed(seed), WRITES_PER_RUN));
        let _ = steps_in_doubt(&journal.into_survivor()).expect("replay");
    }
    let elapsed = started.elapsed();
    println!("200 seeds in {elapsed:?} — {:?} per seed", elapsed / 200);
}
```

```bash
cargo test -p simulator --test dst_campaign measure_the_cost_of_one_seed --release -- --nocapture
```

⚠️ **`--release`**, perché è il profilo con cui M-2 misurò i 25,8 µs e un numero preso in `debug` non è confrontabile con quello.

- [ ] **Step 2: scegliere il numero contro un tetto dichiarato**

Regola: **la campagna breve resta sotto un secondo di tempo di parete in `release`**, perché gira a ogni commit dentro `cargo test`.

Calcola `SHORT_CAMPAIGN_SEEDS` = il più grande multiplo di 100 che sta sotto il tetto, usando il costo per seme appena misurato. ⛔ **Scrivi il numero misurato nel commento**, non quello che ti aspettavi.

- [ ] **Step 3: sostituire la sonda temporanea con la campagna**

Cancella `measure_the_cost_of_one_seed` e aggiungi:

```rust
/// How many seeds the SHORT campaign explores. ⛔ FIXED AND VERSIONED — constraint 7 of §11 —
/// and MEASURED rather than chosen: at <PER-SEED COST MEASURED IN STEP 1> per seed in release,
/// this many stays under the declared ceiling of one second of wall time, which is what lets it
/// run inside `cargo test` on every commit.
const SHORT_CAMPAIGN_SEEDS: u64 = <NUMBER FROM STEP 2>;

/// The deep campaign. ⛔ `#[ignore]` rather than a shorter list: constraint 8 of §11 puts the
/// deep DST on a LONG CYCLE, and a campaign that made every commit slower would be turned off
/// by whoever waits for it.
const DEEP_CAMPAIGN_SEEDS: u64 = SHORT_CAMPAIGN_SEEDS * 100;

/// Runs `seeds` seeds and returns how many crashed, printing the wall time.
///
/// ⛔ THE WALL TIME IS PRINTED ON EVERY RUN — constraint 7 of §11 — and `println!` reaches the
/// terminal only under `--nocapture`, which is why `scripts/gate.sh` runs this binary a second
/// time with that flag. The assertions are the control; the printing is not.
fn campaign(seeds: u64) -> u64 {
    let started = std::time::Instant::now();
    let mut crashes = 0u64;

    for seed in 0..seeds {
        let (journal, trace) = run(seed, CrashingJournal::from_seed(crash_seed(seed), WRITES_PER_RUN));
        let fell = journal.has_fallen();
        let point = journal.falls_at();
        let survivor = journal.into_survivor();

        let expected = expected_doubt(&trace);
        let found: Vec<u64> = steps_in_doubt(&survivor)
            .expect("replay")
            .iter()
            .map(|d| d.step.get())
            .collect();
        assert_eq!(found, expected, "seed {seed}, crash at write {point}");

        if fell {
            crashes += 1;
        }
    }

    println!(
        "DST level 1: {seeds} seeds, {crashes} crashed, {:?} of wall time",
        started.elapsed()
    );
    crashes
}

#[test]
fn the_short_campaign_runs_on_every_commit() {
    let crashes = campaign(SHORT_CAMPAIGN_SEEDS);
    // ⛔ The same non-vacuity as C7b, on the campaign that actually runs: a campaign in which
    // nothing crashed is a campaign that measured nothing. Gotcha #17.
    assert!(crashes > 0, "no seed crashed in the short campaign");
}

#[test]
#[ignore = "the deep campaign belongs to the long cycle — constraint 8 of §11"]
fn the_deep_campaign() {
    let crashes = campaign(DEEP_CAMPAIGN_SEEDS);
    assert!(crashes > 0, "no seed crashed in the deep campaign");
}
```

⛔ **Sostituisci i due segnaposto `<...>` con i numeri misurati agli Step 1 e 2.** Un segnaposto sopravvissuto al commit è il gotcha **#43**.

- [ ] **Step 4: eseguire, e leggere il tempo stampato**

```bash
cargo test -p simulator --test dst_campaign -- --nocapture
```

Atteso: verde, e una riga `DST level 1: <N> seeds, <M> crashed, <T> of wall time`.

```bash
cargo test -p simulator --test dst_campaign -- --ignored --nocapture
```

Atteso: verde. ⚠️ Se la campagna profonda impiega più di qualche minuto, **abbassa `DEEP_CAMPAIGN_SEEDS` e registra il perché** invece di lasciarla in un tempo che nessuno aspetterà.

- [ ] **Step 5: provare che la campagna sia esercitata dal cancello**

```bash
cargo test --workspace 2>&1 | grep -c 'the_short_campaign_runs_on_every_commit'
```

Atteso: `1`. ⛔ Se è `0`, la campagna **non** gira a ogni commit e la decisione D6 è falsa: fermarsi.

- [ ] **Step 6: il cancello, e il commit**

```bash
bash scripts/gate.sh
```

Atteso: `GATE GREEN`.

```bash
git add crates/simulator/tests/dst_campaign.rs
git commit -m "test(simulator): la campagna breve, col numero di semi misurato invece che scelto"
git push
```

---

# Parte 2 — il livello 2: il motore che cade

## Task 5: `CrashingBackend`, e l'archivio che si riapre

**Files:**
- Test: `crates/platform/tests/engine_crash_consistency.rs` (create)

- [ ] **Step 1: scrivere il backend e la sonda che fallisce**

Crea `crates/platform/tests/engine_crash_consistency.rs`:

```rust
//! Level 2 of the two crash levels (ADR-0032, §4.6): the subject under test is not the kernel
//! but `redb` ITSELF, driven through the backend we control — *does the engine leave a
//! recoverable archive?*
//!
//! ⛔ IT LIVES IN A TEST BINARY AND NOT IN `platform/src/`, and that is the point rather than
//! tidiness: what task 8 of milestone 3 bought is that the `StorageBackend` boundary is
//! reachable FROM OUTSIDE THE CRATE (gotcha #46). A failing backend written inside `platform`
//! would prove nothing about that. It sits beside `CountingBackend` in
//! `crates/platform/tests/file_journal.rs`, whose comment named this milestone in advance.

use std::io;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use kernel::ports::journal::{Journal, StepId};
use platform::journal::FileJournal;
use redb::StorageBackend;

/// The bytes of the archive, held OUTSIDE the backend that serves them.
///
/// ⛔ THIS IS WHY `redb::InMemoryBackend` IS NOT USED, and it was measured rather than assumed:
/// that type is `InMemoryBackend(RwLock<Vec<u8>>)` with PRIVATE guards, so the bytes die with
/// the object and the archive CANNOT BE REOPENED — and reopening is the entire question level 2
/// asks. ADR-0032 measured with a backend of its own for the same reason.
type Archive = Arc<Mutex<Vec<u8>>>;

fn empty_archive() -> Archive {
    Arc::new(Mutex::new(Vec::new()))
}

/// A `redb::StorageBackend` that STOPS SERVING at an operation chosen by the caller.
///
/// ⚠️ `close` NEVER FAILS, and it is an exception with a reason: `redb` calls it exactly once
/// when the `Database` is dropped, and a failure there would fire during unwinding rather than
/// at the injection point the test is about.
#[derive(Debug)]
struct CrashingBackend {
    archive: Archive,
    falls_at: u64,
    operations: Arc<AtomicU64>,
    syncs: Arc<AtomicU64>,
    fallen: Arc<AtomicBool>,
}

/// What the test keeps after handing the backend over BY VALUE to `FileJournal::with_backend`.
#[derive(Clone)]
struct Handles {
    archive: Archive,
    operations: Arc<AtomicU64>,
    syncs: Arc<AtomicU64>,
    fallen: Arc<AtomicBool>,
}

fn backend(archive: &Archive, falls_at: u64) -> (CrashingBackend, Handles) {
    let handles = Handles {
        archive: Arc::clone(archive),
        operations: Arc::new(AtomicU64::new(0)),
        syncs: Arc::new(AtomicU64::new(0)),
        fallen: Arc::new(AtomicBool::new(false)),
    };
    let backend = CrashingBackend {
        archive: Arc::clone(archive),
        falls_at,
        operations: Arc::clone(&handles.operations),
        syncs: Arc::clone(&handles.syncs),
        fallen: Arc::clone(&handles.fallen),
    };
    (backend, handles)
}

impl CrashingBackend {
    /// Whether this operation may proceed, MARKING the fall when it may not. Once fallen, the
    /// backend never serves again — decision D2 applied at level 2.
    fn may_serve(&self) -> bool {
        if self.fallen.load(Ordering::Relaxed) {
            return false;
        }
        if self.operations.fetch_add(1, Ordering::Relaxed) == self.falls_at {
            self.fallen.store(true, Ordering::Relaxed);
            return false;
        }
        true
    }

    fn gone() -> io::Error {
        io::Error::new(io::ErrorKind::Other, "the process is gone")
    }
}

impl StorageBackend for CrashingBackend {
    fn len(&self) -> Result<u64, io::Error> {
        if !self.may_serve() {
            return Err(Self::gone());
        }
        Ok(self.archive.lock().expect("archive").len() as u64)
    }

    fn read(&self, offset: u64, out: &mut [u8]) -> Result<(), io::Error> {
        if !self.may_serve() {
            return Err(Self::gone());
        }
        let guard = self.archive.lock().expect("archive");
        let offset = usize::try_from(offset).map_err(|_| Self::gone())?;
        if offset + out.len() > guard.len() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "out of range"));
        }
        out.copy_from_slice(&guard[offset..offset + out.len()]);
        Ok(())
    }

    fn set_len(&self, len: u64) -> Result<(), io::Error> {
        if !self.may_serve() {
            return Err(Self::gone());
        }
        let len = usize::try_from(len).map_err(|_| Self::gone())?;
        // ⛔ ZERO-FILLED, because `redb`'s own trait says so: "New positions in the storage must
        // be initialized to zero". `Vec::resize` with 0 is exactly that, and getting it wrong
        // would corrupt the archive for a reason that has nothing to do with the injection.
        self.archive.lock().expect("archive").resize(len, 0);
        Ok(())
    }

    fn sync_data(&self) -> Result<(), io::Error> {
        self.syncs.fetch_add(1, Ordering::Relaxed);
        if !self.may_serve() {
            return Err(Self::gone());
        }
        Ok(())
    }

    fn write(&self, offset: u64, data: &[u8]) -> Result<(), io::Error> {
        if !self.may_serve() {
            return Err(Self::gone());
        }
        let mut guard = self.archive.lock().expect("archive");
        let offset = usize::try_from(offset).map_err(|_| Self::gone())?;
        if offset + data.len() > guard.len() {
            guard.resize(offset + data.len(), 0);
        }
        guard[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }

    fn close(&self) -> Result<(), io::Error> {
        Ok(())
    }
}

#[test]
fn without_a_crash_the_archive_reopens_with_everything_in_it() {
    // ⛔ THE OTHER DIRECTION FIRST (rule 3 of §7.1.1): if this failed, every red below would be
    // about the backend rather than about the injection.
    let archive = empty_archive();
    let (backend, handles) = backend(&archive, u64::MAX);

    {
        let mut journal = FileJournal::with_backend(backend).expect("open");
        journal.intent(StepId::new(1), b"one").expect("intent");
        journal.outcome(StepId::new(1), b"one done").expect("outcome");
    }

    let (reopened_backend, _) = backend(&handles.archive, u64::MAX);
    let reopened = FileJournal::with_backend(reopened_backend).expect("reopen");
    assert_eq!(
        reopened.replay().expect("replay"),
        vec![
            (StepId::new(1), b"one".to_vec()),
            (StepId::new(1), b"one done".to_vec()),
        ]
    );
}
```

- [ ] **Step 2: eseguirla e vederla fallire**

```bash
cargo test -p platform --test engine_crash_consistency
```

Atteso: **non compila**, oppure **rosso**. ⛔ Se è verde al primo colpo, rileggi lo Step 1: la sonda deve aver esercitato la scrittura, la riapertura **e** il `replay`.

- [ ] **Step 3: far passare**

Nessun codice di prodotto. Correggi la compilazione e riesegui.

```bash
cargo test -p platform --test engine_crash_consistency
```

Atteso: `test result: ok. 1 passed; 0 failed`.

- [ ] **Step 4: misurare che cosa fa `redb` alla caduta, invece di supporlo**

⛔ **Questa è una misura, non un'asserzione**, e va fatta prima di scrivere il test del crash: non è noto in anticipo se il `Drop` di `Database` su un backend caduto vada in panico, ignori l'errore, o scriva ancora.

Aggiungi una sonda **temporanea**:

```rust
#[test]
fn measure_what_the_engine_does_when_the_backend_falls() {
    let archive = empty_archive();
    let (backend, handles) = backend(&archive, 3);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut journal = FileJournal::with_backend(backend).expect("open");
        let first = journal.intent(StepId::new(1), b"one");
        let second = journal.outcome(StepId::new(1), b"one done");
        (format!("{first:?}"), format!("{second:?}"))
    }));
    println!("outcome: {result:?}");
    println!("operations reached: {}", handles.operations.load(Ordering::Relaxed));
    println!("syncs reached: {}", handles.syncs.load(Ordering::Relaxed));
    println!("fallen: {}", handles.fallen.load(Ordering::Relaxed));
    println!("archive bytes: {}", handles.archive.lock().expect("archive").len());
}
```

```bash
cargo test -p platform --test engine_crash_consistency measure_what_the_engine_does_when_the_backend_falls -- --nocapture
```

⛔ **Annota l'uscita nello scratchpad.** Se `FileJournal::with_backend` stesso cade perché l'apertura consuma già più di tre operazioni, **alza il punto di caduta** finché l'apertura riesce: il punto utile è quello **dopo** l'apertura, e sceglierlo altrove misurerebbe l'apertura invece della scrittura. È il gotcha **#17**.

Cancella la sonda temporanea quando hai i numeri.

- [ ] **Step 5: il cancello, e il commit**

```bash
bash scripts/gate.sh
```

Atteso: `GATE GREEN`.

```bash
git add crates/platform/tests/engine_crash_consistency.rs
git commit -m "test(platform): il backend cadente, scritto da fuori la crate, e l'archivio che si riapre"
git push
```

---

## Task 6: la coerenza dopo la caduta, e il gotcha #51 chiuso

**Files:**
- Modify: `crates/platform/tests/engine_crash_consistency.rs`

- [ ] **Step 1: scrivere le due sonde che falliscono**

Aggiungi in coda a `crates/platform/tests/engine_crash_consistency.rs`, usando come **punto di apertura** il numero misurato al Task 5 Step 4:

```rust
/// How many backend operations opening a journal costs. ⛔ MEASURED at task 5 step 4 and not
/// guessed: an injection point below this number crashes the OPEN, which measures the opening
/// instead of the writing — gotcha #17.
const OPERATIONS_TO_OPEN: u64 = <NUMBER MEASURED AT TASK 5 STEP 4>;

/// Writes two steps through a backend that falls at `falls_at`, then reopens the archive and
/// returns what came back — or `None` if the archive is unreadable, which is itself an answer.
fn crash_then_reopen(falls_at: u64) -> (Handles, Option<Vec<(StepId, Vec<u8>)>>) {
    let archive = empty_archive();
    let (crashing, handles) = backend(&archive, falls_at);

    // ⛔ The whole run is caught: a crashed engine is allowed to panic on the way down, and a
    // panic here is a legitimate outcome of the injection rather than a defect of the test.
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut journal = FileJournal::with_backend(crashing).expect("open");
        let _ = journal.intent(StepId::new(1), b"one");
        let _ = journal.outcome(StepId::new(1), b"one done");
        let _ = journal.intent(StepId::new(2), b"two");
    }));

    let (fresh, _) = backend(&handles.archive, u64::MAX);
    let reopened = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        FileJournal::with_backend(fresh)
            .ok()
            .and_then(|journal| journal.replay().ok())
    }))
    .unwrap_or(None);

    (handles, reopened)
}

#[test]
fn a_crashed_archive_reopens_in_a_coherent_state() {
    // ⛔ THE QUESTION LEVEL 2 ASKS, and the answer is not "everything survived": it is that
    // what comes back is a PREFIX of what was written — either the records confirmed before the
    // fall, or all of them, NEVER a partial record and never a scrambled one. ADR-0032 measured
    // 12 injection points, 12 reopenings, 12 coherent states; this holds it at every commit.
    let written: Vec<(StepId, Vec<u8>)> = vec![
        (StepId::new(1), b"one".to_vec()),
        (StepId::new(1), b"one done".to_vec()),
        (StepId::new(2), b"two".to_vec()),
    ];

    let mut fired = 0u64;
    for falls_at in OPERATIONS_TO_OPEN..OPERATIONS_TO_OPEN + 40 {
        let (handles, reopened) = crash_then_reopen(falls_at);
        if handles.fallen.load(Ordering::Relaxed) {
            fired += 1;
        }
        let Some(records) = reopened else {
            // An archive that will not reopen at all is a FAILURE of this promise, and it is
            // named rather than skipped.
            panic!("injection at {falls_at}: the archive did not reopen");
        };
        assert!(
            written.starts_with(&records),
            "injection at {falls_at}: what came back is not a prefix of what was written: {records:?}"
        );
    }

    // ⛔ THE NON-VACUITY, and it is the oracle rather than a statistic: forty injection points
    // that never fired would pass every assertion above. Gotcha #17.
    assert!(fired > 0, "no injection point fired: the campaign measured nothing");
}

#[test]
fn the_engine_really_syncs_and_that_is_what_closes_gotcha_51() {
    // ⛔ THE PROMISE NOBODY HELD UNTIL NOW. `FileJournal` promises a write survives the death of
    // the process, and putting `set_durability(Durability::None)` into it leaves ALL SIX tests
    // of `file_journal.rs` GREEN — because they reopen the file inside a LIVING process, so the
    // writes are in the operating system's hands either way. Gotcha #51.
    //
    // ⛔ WHAT MAKES IT OBSERVABLE IS NOT THE ARCHIVE, IT IS THE COUNT. With `Durability::None`
    // `redb` DOES NOT CALL `sync_data`, so a backend that COUNTS the calls says outright what no
    // reading of the archive can. The counter is the oracle — gotcha #54 applied to the
    // injection, and #17.
    let archive = empty_archive();
    let (crashing, handles) = backend(&archive, u64::MAX);

    {
        let mut journal = FileJournal::with_backend(crashing).expect("open");
        journal.intent(StepId::new(1), b"one").expect("intent");
    }

    assert!(
        handles.syncs.load(Ordering::Relaxed) > 0,
        "no sync_data reached the backend: the durability guarantee is not being asked for"
    );
}
```

⛔ **Sostituisci `<NUMBER MEASURED AT TASK 5 STEP 4>`.** Un segnaposto sopravvissuto al commit è il gotcha **#43**.

- [ ] **Step 2: eseguirle e vederle fallire**

```bash
cargo test -p platform --test engine_crash_consistency
```

⚠️ È possibile che siano **verdi al primo colpo**. In quel caso **non sono provate**: vai allo Step 4 e torna qui solo dopo aver visto entrambi i rossi.

- [ ] **Step 3: far passare**

Nessun codice di prodotto. Riesegui:

```bash
cargo test -p platform --test engine_crash_consistency
```

Atteso: `test result: ok. 3 passed; 0 failed`.

- [ ] **Step 4: la mutazione che chiude il #51, ed è su codice di produzione**

⛔ **Questa è la mutazione per cui esiste il compito**, e **il sorgente nomina già il punto**: in `crates/platform/src/journal.rs` c'è un commento che dice testualmente *«`redb` commits with `Durability::Immediate` unless told otherwise … `set_durability(Durability::None)` right here leaves ALL SIX tests of …»*. **«Right here» è il punto.** Inserisci lì, prima del commit della transazione:

```rust
        transaction.set_durability(redb::Durability::None);
```

```bash
cargo build --workspace --tests
cargo test -p platform --test engine_crash_consistency
```

Atteso: **rosso** su `the_engine_really_syncs_and_that_is_what_closes_gotcha_51`, col messaggio *«no sync_data reached the backend»*.

```bash
cargo test -p platform --test file_journal
```

Atteso: ⛔ **verde, tutti e sei.** È la conferma diretta del gotcha #51 — *«una garanzia sulla morte del processo non è osservabile da dentro il processo»* — e va **letta**, perché è ciò che dimostra che il controllo nuovo non era ridondante.

Ripristina e riesegui: verde.

- [ ] **Step 5: la seconda direzione — che il controllo non scatti dove non deve**

Mutazione — il backend conta i `sync_data` ma il punto di caduta è dentro l'apertura:

```rust
    for falls_at in 0..40 {
```

```bash
cargo build -p platform --tests
cargo test -p platform --test engine_crash_consistency
```

Atteso: **rosso** con *«the archive did not reopen»* su qualche punto basso. ⛔ **È la prova che `OPERATIONS_TO_OPEN` non è decorativo**: senza, la campagna misurerebbe l'apertura invece della scrittura. Ripristina.

- [ ] **Step 6: datare il commento che rimandava a qui**

In `crates/platform/tests/file_journal.rs`, dentro `the_storage_backend_is_substitutable_from_outside`, sostituisci:

```rust
    // written from OUTSIDE the crate, which is what this test is: `CountingBackend` lives here,
    // in a test binary, and `FileJournal` runs on it unchanged. Milestone 4 will put a FAILING
    // one in the same place (§4.6, ADR-0032 requirement 4); this one only counts.
```

con:

```rust
    // written from OUTSIDE the crate, which is what this test is: `CountingBackend` lives here,
    // in a test binary, and `FileJournal` runs on it unchanged. ✅ MILESTONE 4 PUT THE FAILING
    // ONE IN THE SAME PLACE on 2026-08-11 — `CrashingBackend` in
    // `crates/platform/tests/engine_crash_consistency.rs` — and this line is dated rather than
    // deleted, because the sentence it replaced is why that file went there instead of into
    // `src/`. This one only counts.
```

- [ ] **Step 7: il cancello, e il commit**

```bash
bash scripts/gate.sh
```

Atteso: `GATE GREEN`.

```bash
git add crates/platform/tests/engine_crash_consistency.rs crates/platform/tests/file_journal.rs
git commit -m "test(platform): l'archivio caduto si riapre coerente, e il conteggio dei sync chiude il #51"
git push
```

---

## Task 7: la campagna di livello 2, e il tempo di parete

**Files:**
- Modify: `crates/platform/tests/engine_crash_consistency.rs`

- [ ] **Step 1: misurare il costo di un punto d'iniezione**

Sonda **temporanea**:

```rust
#[test]
fn measure_the_cost_of_one_injection() {
    let started = std::time::Instant::now();
    for falls_at in OPERATIONS_TO_OPEN..OPERATIONS_TO_OPEN + 40 {
        let _ = crash_then_reopen(falls_at);
    }
    let elapsed = started.elapsed();
    println!("40 injections in {elapsed:?} — {:?} each", elapsed / 40);
}
```

```bash
cargo test -p platform --test engine_crash_consistency measure_the_cost_of_one_injection --release -- --nocapture
```

- [ ] **Step 2: scegliere il numero contro lo stesso tetto**

Regola: **la campagna breve di livello 2 resta sotto un secondo di tempo di parete in `release`**. Il numero di punti è quello che ci sta.

- [ ] **Step 3: sostituire la sonda temporanea con la campagna**

Cancella `measure_the_cost_of_one_injection` e aggiungi:

```rust
/// How many injection points the SHORT level-2 campaign explores. ⛔ FIXED AND VERSIONED —
/// constraint 7 of §11 — and MEASURED rather than chosen: at <COST MEASURED IN STEP 1> per
/// injection, this many stays under the declared ceiling of one second of wall time.
///
/// ⚠️ IT IS SMALLER THAN LEVEL 1 BY TWO ORDERS OF MAGNITUDE, and that is the design rather than
/// a shortfall: this campaign drives a real B-tree through a real archive, while level 1 runs
/// `no_std` code over a `Vec`. Two subjects, two costs — §2.1 of the design.
const SHORT_INJECTIONS: u64 = <NUMBER FROM STEP 2>;

const DEEP_INJECTIONS: u64 = SHORT_INJECTIONS * 20;

/// Walks `points` injection points and returns how many actually fired, printing the wall time.
fn injection_campaign(points: u64) -> u64 {
    let written: Vec<(StepId, Vec<u8>)> = vec![
        (StepId::new(1), b"one".to_vec()),
        (StepId::new(1), b"one done".to_vec()),
        (StepId::new(2), b"two".to_vec()),
    ];
    let started = std::time::Instant::now();
    let mut fired = 0u64;

    for falls_at in OPERATIONS_TO_OPEN..OPERATIONS_TO_OPEN + points {
        let (handles, reopened) = crash_then_reopen(falls_at);
        if handles.fallen.load(Ordering::Relaxed) {
            fired += 1;
        }
        let Some(records) = reopened else {
            panic!("injection at {falls_at}: the archive did not reopen");
        };
        assert!(
            written.starts_with(&records),
            "injection at {falls_at}: what came back is not a prefix of what was written: {records:?}"
        );
    }

    println!(
        "DST level 2: {points} injection points, {fired} fired, {:?} of wall time",
        started.elapsed()
    );
    fired
}

#[test]
fn the_short_injection_campaign_runs_on_every_commit() {
    let fired = injection_campaign(SHORT_INJECTIONS);
    assert!(fired > 0, "no injection point fired in the short campaign");
}

#[test]
#[ignore = "the deep campaign belongs to the long cycle — constraint 8 of §11"]
fn the_deep_injection_campaign() {
    let fired = injection_campaign(DEEP_INJECTIONS);
    assert!(fired > 0, "no injection point fired in the deep campaign");
}
```

⛔ **Sostituisci i due segnaposto `<...>` coi numeri misurati.**

- [ ] **Step 4: eseguire, e leggere il tempo stampato**

```bash
cargo test -p platform --test engine_crash_consistency -- --nocapture
cargo test -p platform --test engine_crash_consistency -- --ignored --nocapture
```

Atteso: verdi, con le righe `DST level 2: ...`.

- [ ] **Step 5: il cancello, e il commit**

```bash
bash scripts/gate.sh
```

Atteso: `GATE GREEN`.

```bash
git add crates/platform/tests/engine_crash_consistency.rs
git commit -m "test(platform): la campagna di livello 2, col numero di punti misurato"
git push
```

---

# Parte 3 — la chiusura

## Task 8: l'elenco dei semi, che non deve mentire

**Files:**
- Create: `docs/semi-dst.md`

- [ ] **Step 1: scrivere il file**

⛔ **Non c'è una sonda per questo artefatto, ed è deliberato** — §6 e §0.2 del disegno. Un elenco di semi che qualcosa *esercita* è un elenco che qualcuno leggerà come una rete di regressione, che è esattamente l'errore che la §3.4 della spec esiste per impedire.

Crea `docs/semi-dst.md`:

```markdown
# I semi della DST — e perché non sono un oracolo

**Creato il 2026-08-11**, col Traguardo 4.

## ⛔ La regola, prima dell'elenco

La §3.4 della [spec del sotto-progetto 1](superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md)
è categorica, e questa pagina esiste perché quella regola sopravviva al primo che leggerà la
tabella qui sotto:

> **Un seme riproduce un'esecuzione soltanto finché il codice non cambia.** Modificato il kernel,
> lo stesso seme esplora un cammino diverso.

| Cosa è permanente | Cosa non lo è |
|---|---|
| la **proprietà** verificata | il **cammino** che quella volta la violò |
| il seme come **punto di ripartenza per indagare oggi** | la garanzia che domani ritrovi lo stesso difetto |

⛔ **Quindi questo elenco non si rigioca automaticamente**, e la campagna non lo legge.
Rigiocare un seme il cui cammino è cambiato **non prova nulla e costa tempo** — sarebbe la stessa
classe di errore di «cifrato a riposo» dichiarato più forte di quanto sia.

⛔ **E ogni voce nomina il test permanente della propria proprietà.** Una riga senza quella colonna
è una riga che non protegge niente: il seme serve a **debuggare**, è la **proprietà** a proteggere.

## L'elenco

| Seme | Campagna | Cosa trovò | Il test permanente della proprietà |
|---|---|---|---|
| — | — | ⚠️ **Nessun difetto trovato alla chiusura del Traguardo 4.** La riga esiste per dire che l'elenco è **vuoto e non dimenticato** | — |

## Come si aggiunge una voce

1. La campagna fallisce su un seme. **Si annota il seme**, e si riproduce con
   `cargo test -p simulator --test dst_campaign -- --nocapture`.
2. Si trova la **proprietà** violata — non il cammino.
3. ⛔ **Si scrive un test che tenga quella proprietà**, e che fallisca prima della correzione.
4. Si corregge.
5. Si aggiunge la riga qui, **con il nome del test del punto 3**.

⚠️ Se il punto 3 non si riesce a fare, la voce **non si aggiunge**: un seme senza proprietà è
esattamente la falsa sicurezza che il riquadro in testa vieta.
```

- [ ] **Step 2: verificare che il collegamento risolva**

```bash
bash scripts/check-docs.sh
```

Atteso: `OK — no inconsistencies`. ⛔ Se esce `broken link`, il percorso relativo è sbagliato: `docs/semi-dst.md` sta in `docs/`, quindi la spec è `superpowers/specs/...` senza `../`.

- [ ] **Step 3: il cancello, e il commit**

```bash
bash scripts/gate.sh
```

Atteso: `GATE GREEN`.

```bash
git add docs/semi-dst.md
git commit -m "docs: l'elenco dei semi nasce vuoto, e con la regola per cui non è un oracolo"
git push
```

---

## Task 9: il cancello stampa il tempo di parete, e il registro incassa

**Files:**
- Modify: `scripts/gate.sh`
- Modify: `docs/porta-di-qualita.md`

- [ ] **Step 1: aggiungere il passo di stampa al cancello**

⛔ **Non è un settimo controllo** — decisione D6. Le asserzioni delle due campagne girano già dentro `cargo test --workspace`, che è il secondo controllo; questo passo esiste **solo** perché il vincolo 7 della §11 pretende che il tempo di parete si stampi a ogni corsa, e `cargo test` cattura l'uscita dei test che passano.

In `scripts/gate.sh`, dopo la riga `run "documentation consistency" ...`, aggiungi:

```bash

# ⛔ NOT A SEVENTH CHECK, and the count in the docs stays at six. The assertions of both DST
# campaigns already run inside `cargo test --workspace` above — that is the cadence constraint 8
# of §11 asks for. This runs them a SECOND time with `--nocapture` for one reason only:
# constraint 7 wants the WALL TIME printed on every run, and `cargo test` swallows the output of
# tests that pass. The cost is declared: the short campaigns run twice.
run "DST campaigns — wall time" bash -c '
  cargo test -p simulator --test dst_campaign -- --nocapture &&
  cargo test -p platform --test engine_crash_consistency -- --nocapture'
```

- [ ] **Step 2: eseguire il cancello e leggere l'uscita**

```bash
bash scripts/gate.sh
```

Atteso: `GATE GREEN`, e **due righe visibili** — `DST level 1: ...` e `DST level 2: ...`.

⛔ **Se le due righe non compaiono, il vincolo 7 non è soddisfatto** e il passo non serve a niente: fermarsi e capire perché.

- [ ] **Step 3: provare che il passo scatti anche in negativo**

Mutazione — rendi rossa una campagna. In `crates/simulator/tests/dst_campaign.rs`, dentro `the_short_campaign_runs_on_every_commit`:

```rust
    assert!(crashes > u64::MAX - 1, "forced red");
```

```bash
bash scripts/gate.sh
```

Atteso: `GATE RED -- 2 checks failed` — il secondo controllo **e** il passo di stampa. ⛔ Se ne fallisce solo uno, il passo di stampa non sta eseguendo ciò che dice. Ripristina.

- [ ] **Step 4: registrare i controlli nuovi**

In `docs/porta-di-qualita.md`, aggiungi le righe dei controlli nuovi seguendo il formato già in uso nel file — una riga per artefatto, con il file che la implementa, le sonde **per nome** e la contro-sonda.

Artefatti da registrare:

| Artefatto | File | Contro-sonda |
|---|---|---|
| `CrashingJournal` | `crates/simulator/tests/crashing_journal.rs` | `a_journal_told_not_to_crash_never_falls`, e la mutazione B del Task 1 |
| `C7a` | `crates/simulator/tests/dst_campaign.rs` | la mutazione del Task 2 Step 4 |
| `C7b` e la campagna di livello 1 | `crates/simulator/tests/dst_campaign.rs` | le tre mutazioni del Task 3 Step 4 |
| `CrashingBackend` e la campagna di livello 2 | `crates/platform/tests/engine_crash_consistency.rs` | le mutazioni del Task 6 Step 4 e Step 5 |
| il gotcha #51 chiuso | `the_engine_really_syncs_and_that_is_what_closes_gotcha_51` | `Durability::None` lascia i sei test di `file_journal.rs` verdi |

⛔ **E i conteggi del file si RICONTANO, non si incrementano** — gotcha #49, e la quinta questione aperta della §6 del compendio dice che quel file non è sorvegliato dalla guardia dei conteggi.

```bash
cargo test --workspace --no-fail-fast 2>&1 | grep 'test result'
```

Usa **questa uscita** per i numeri, non il ricordo.

- [ ] **Step 5: il cancello, e il commit**

```bash
bash scripts/gate.sh
```

Atteso: `GATE GREEN`, con le due righe di tempo di parete.

```bash
git add scripts/gate.sh docs/porta-di-qualita.md
git commit -m "chore: il cancello stampa il tempo di parete delle campagne, e il registro le incassa"
git push
```

---

## Task 10: la chiusura del traguardo nei documenti di stato

**Files:**
- Modify: `docs/COMPENDIO.md`, `docs/HANDOFF.md`, `docs/roadmap.md`, `docs/README.md`, `docs/riferimenti.md`
- Modify: `docs/superpowers/specs/2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst-design.md`

- [ ] **Step 1: ricontare prima di scrivere**

⛔ **Questo compito è un AUDIT, non una scrittura** — è la forma che il Task 12 del Traguardo 3 e il Task 13 del Traguardo 2 hanno preso entrambi, ed è il gotcha **#49**. La prima domanda non è *«cosa aggiungo?»* ma *«cosa è già stato scritto dai compiti che l'hanno prodotto?»*.

```bash
cargo test --workspace --no-fail-fast 2>&1 | tail -40
ls crates/kernel/tests/compile_fail/*.rs | wc -l
```

⛔ **Riconta:** i target, i test, i casi `compile_fail`. Nessuno di questi numeri si deduce dal numero precedente.

- [ ] **Step 2: rileggere la Definizione di «fatto» contro il repository**

⛔ **Il metro invecchia come l'oggetto misurato**, ed è successo in due traguardi su due. Rileggi le **otto** condizioni della Definizione di «fatto» qui sotto **una per una contro il codice**, e dove una è stantia **correggila in un'errata in testa a questo piano**, non nel testo — il testo è il registro di ciò che fu deciso.

- [ ] **Step 3: aggiornare i documenti di stato**

| File | Cosa |
|---|---|
| `docs/COMPENDIO.md` | §6: il traguardo eseguito, e **il prossimo passo — che vive in TRE punti di quella sezione**. §9 se sono nati gotcha nuovi. §12: la riga di `docs/semi-dst.md`, e la **ventunesima misura** |
| `docs/HANDOFF.md` | «Prima cosa da fare», la tabella dei sei traguardi, e il testo integrale dei gotcha nuovi |
| `docs/roadmap.md` | la riga del Traguardo 4, e il prossimo passo |
| `docs/README.md` | lo stato in testa, e la tabella delle specifiche |
| `docs/riferimenti.md` | ⛔ **le misure del traguardo, coi comandi**: il costo per seme, il costo per iniezione, i due numeri scelti, e ciò che la mutazione `Durability::None` ha mostrato |
| il **disegno** | un richiamo datato dove il codice l'ha smentito, se è successo di nuovo |

⛔ **La cifra dei semi e quella delle iniezioni vivono in più posti.** Cercale con un `grep` su **tutto il repository** prima di dichiararle aggiornate — sedicesima misura della §12.

- [ ] **Step 4: la misura dei pesi**

⛔ **Si scrive a passata CHIUSA**, cioè dopo l'ultima riga di documentazione — nona misura. Righe contate **partendo dall'elenco dei file citati** e non dalle righe presenti — quindicesima. `wc -c` **arrotondato** a KiB.

```bash
kib() { for f in "$@"; do b=$(wc -c < "$f"); printf '%4d KiB  %s\n' "$(( (b + 512) / 1024 ))" "$f"; done; }
```

- [ ] **Step 5: il cancello, e il commit**

```bash
bash scripts/gate.sh
```

Atteso: `GATE GREEN`.

```bash
git add -A
git commit -m "docs: il Traguardo 4 è chiuso, e il simulatore sa rompersi"
git push
```

---

## Definizione di «fatto»

⚠️ **Questa lista invecchia**, ed è successo in due traguardi su due. Si rilegge **contro il codice** al Task 10, non contro sé stessa.

| # | Condizione |
|---|---|
| **1** | `bash scripts/gate.sh` → `GATE GREEN`, e stampa **due** righe di tempo di parete |
| **2** | i **sette** artefatti della §0.2 del disegno esistono, ciascuno col controllo dichiarato nella sua riga |
| **3** | `C7a` e `C7b` girano dentro `cargo test --workspace`, quindi **a ogni commit** |
| **4** | ⛔ il gotcha **#51** è chiuso: `Durability::None` fa **rosso** — provato togliendo la durabilità davvero, e verificando che i sei test di `file_journal.rs` restino **verdi** |
| **5** | ⛔ ogni campagna è provata **in due direzioni**: che trovi il difetto che c'è, e che **non** ne dichiari uno che non c'è |
| **6** | [ADR-0032](../../adr/0032-motore-di-persistenza.md) porta il proprio rimando datato — ✅ **già fatto il 2026-08-11**, prima di questo piano |
| **7** | `docs/semi-dst.md` esiste, e la sua regola dice perché l'elenco non si rigioca |
| **8** | `docs/porta-di-qualita.md` ha una riga per ogni controllo nuovo, e i conteggi sono **ricontati** |
| **9** | ⛔ i **byte congelati** di `crates/kernel/tests/frozen/` sono **invariati** — `git diff --stat` non li nomina in nessuno dei dieci commit |
| **10** | ⛔ **nessuna delle quattro finte mancanti** — `filesystem`, `process`, `network`, `ipc` — è nata in questo traguardo |
