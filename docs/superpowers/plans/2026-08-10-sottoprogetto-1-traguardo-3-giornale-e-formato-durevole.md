# Sotto-progetto 1 · Traguardo 3 — il giornale e il formato durevole

## ⛔ Errata — 2026-08-10, dall'esecuzione del Task 1

> Il piano è un'**ipotesi**, e l'esecuzione la misura. Queste voci sono le divergenze trovate
> eseguendo i compiti: il testo dei compiti **non è stato toccato**, perché un piano riscritto a
> posteriori cancella la prova che l'attesa era sbagliata. Chi esegue un compito legge **prima**
> questa sezione, poi il compito. **Ventisei voci in tre passate:** le prime sette dal **Task 1**;
> le **quattordici** da **E8** in giù dai **Task 4 e 5**, eseguiti come un compito solo; **E22**
> da un passo preliminare al **Task 6**, e **E23–E26** dal Task 6 stesso. ⛔ **E19 ed E22 non
> sono divergenze ma DECISIONI**, prese dal coordinatore eseguendo: si leggono per prime fra
> quelle, perché cambiano una **firma pubblica** o il contratto di una porta condivisa, e il
> proprietario deve poterle **ribaltare vedendole** invece di scoprirle. ⛔ **E25 non è né l'una
> né l'altra: è una domanda RIPORTATA e non decisa** — la firma di `replay` — perché cambiarla
> tocca la porta, la conformità e due implementazioni, e il piano stesso (decisione **D6**) dice
> che quel momento è questo. Il coordinatore l'ha riservata a sé.

| # | Dove | Il piano dice | Misurato |
|---|---|---|---|
| **E1** | Task 1, Step 3 — doc di `EffectClass` | il sorgente dettato porta la parola italiana `` `irripetibile` `` in un commento | ⛔ la **§1.0** vieta l'italiano nel sorgente, e il piano **non ha l'autorità di derogarvi**: è la stessa specie della prima voce dell'errata del Traguardo 1. L'identificatore inglese esiste già ed è `Unrepeatable`. Corretto nel sorgente |
| **E2** | Task 1, Step 7 | *«`record_without_version.rs` passa da `error` a `ok`»* | **direzione invertita**. La base è `ok` — il caso fallisce a compilare come deve — e con l'`impl` temporaneo diventa **`error`**, con `Expected test case to fail to compile, but it succeeded.` ⚠️ La conclusione che il piano cerca **regge lo stesso, e in meglio**: la parola è `error` e non `mismatch`, quindi `TRYBUILD=overwrite` non può spegnere il caso e **non serve un secondo caso** (gotcha #42) |
| **E3** | Task 1, Step 3 — decisione **D3** | D3 impone *«scriverlo esplicito anche se è il default»* per la codifica ad array | il `record.rs` **dettato non porta `#[cbor(array)]`** su nessuno dei due tipi: la decisione non è onorata dal sorgente che la cita. Aggiunto su `RecordV1` e `Record`, e **misurato** che i byte non cambiano — `82 00 81 84 00 01 00 40` prima e dopo |
| **E4** | Task 1, Step 4 | attende `test result: ok. 4 passed` | **criterio di chiusura stantio**: il banco dettato lascia campi non riletti (vedi E7), e con le sonde che mancavano i test sono **10**. Un conteggio atteso invecchia come ogni altra cifra — gotcha #31 applicato a un criterio di uscita |
| **E5** | Task 1, Step 3 — doc di `RecordV1` | *«the indices follow **four** rules»* | ne **elenca tre**. Corretto a tre; le **sei** regole di §4.9.2 citate nel doc di modulo restano giuste — sono un altro insieme |
| **E6** | Task 1, Step 1 | la riga `assert_eq!(bytes[0], 0x82, "…");` dettata | **non è `rustfmt`-clean** (gli argomenti superano `fn_call_width` 60) mentre il repository lo è: `cargo fmt --all -- --check` esce 0 su tutto il resto. Riformattata |
| **E7** | Task 1, Step 1 e Step 3 | il banco dettato e il `RecordV1` dettato | **due lacune di specie 2** — il controllo manca del tutto. (a) nessuno dei quattro test dettati **rilegge un `kind`**: un `decode` che rispondesse sempre `Intent` li lascia tutti verdi, sul campo che il sorgente stesso chiama *«the whole write-ahead protocol rests on telling them apart»*. (b) `RecordV1` **deriva `Debug`** mentre porta un `payload` che il campo `trust` dichiara possibilmente non fidato: `boundary.rs` scrive `Debug` a mano per `Untrusted` esattamente per non farlo. Chiuse entrambe con sonda |
| **E8** | Task 4 e Task 5, interi | due compiti, due commit | ⛔ **eseguiti separatamente non funzionano, ed è stato verificato prima di cominciare.** Il Task 4 scrive una suite che chiama `replay()`, che la porta guadagna **solo** al Task 5: chiudere il Task 4 col proprio Step 4 significa committare un albero che **non compila**, contro il vincolo globale 8. E il `git add` dello Step 5 del Task 5 **non nomina** `crates/kernel/tests/journal_contract.rs`: alla lettera, l'artefatto del Task 4 non sarebbe **mai** entrato nel repository. **Fusi in un compito solo con un commit solo.** ⚠️ Una precisazione, perché il pre-controllo l'aveva detta più forte del vero: il Task 4 **ha** uno step di commit (Step 4); ciò che non ha è uno step di **porta** |
| **E9** | Task 4, Step 1 — le cinque promesse | `read_back` è provata **solo** dopo un `intent` | ⛔ **lacuna di specie 2, e proprio quella che l'esecutore del Task 3 aveva chiesto guardando avanti.** La suite non fissava da nessuna parte cosa `read_back` risponda dopo che sono stati scritti intento **ed** esito — che è l'unico caso in cui la scelta è osservabile, perché su un passo col solo intento il primo record e l'ultimo **sono lo stesso record**. Senza, al **Task 8** una tabella `redb` chiavata sul passo risponde l'**esito** e **nulla diventa rosso**. Aggiunta come **promessa 2**, col proprio messaggio |
| **E10** | Task 4, Step 2 — i tre bugiardi | tre bugiardi per cinque promesse | ⛔ **due promesse non si vedevano MAI fallire** — quella su `Missing` e quella su `OutOfOrder` — perché la suite muore alla **prima** promessa violata e nessun bugiardo ci arrivava. ⚠️ `SilentJournal` **viola** anche la promessa sull'esito senza intento (il suo `outcome` risponde `Ok(())`) e muore sulla promessa 1 molto prima: è la forma esatta di un controllo che **sembra** coperto. Gotcha **#14**. Portati a **sei**, uno per promessa, rotti in **sei modi diversi** (gotcha #45), e la corrispondenza uno-a-uno è stata **misurata** neutralizzando una promessa alla volta |
| **E11** | Task 4, Step 1 — promessa su `prune` | `assert!(journal.prune(step).is_err())` | ⚠️ **passa per la ragione sbagliata**: `MemoryJournal` rifiuta **ogni** potatura (decisione D7), quindi soddisfa la riga senza mai consultare se il passo sia in dubbio. Famiglia del gotcha **#30**. ⛔ **Non forzata**: la metà che discrimina è un passo **non** in dubbio la cui potatura dev'essere accettata, e non è scrivibile finché `prune` non è implementata — arriva col **Task 11**. Dichiarata sul posto e aperta come **voce nel registro**, non come nota (gotcha #36) |
| **E12** | Task 4, Step 1 — promessa su `replay` | confronta le **sole identità** dei passi, attese `[1, 2, 1]` | ⛔ **la promessa era VACUA contro il proprio bugiardo, e l'ha detto la misura.** `1, 2, 1` **è un palindromo**: `ShuffledJournal`, che rovescia il giornale, rende le stesse tre identità nelle stesse tre posizioni e **passava la suite intera**. È il difetto di specie 1 nella forma più cara — una sonda che attacca il caso invece del meccanismo. Chiusa confrontando i **record**, byte compresi |
| **E13** | Task 4, Step 1 — promessa 1 | rilegge con `.expect("read_back must find it")` | ⛔ **la via A6 scattava senza saper dire di essere A6.** Un giornale che non scrive risponde `Missing`, muore sull'`expect` e **non arriva** all'`assert_eq!`: il payload non nomina nessuna promessa, e il test negativo riportava *«ha sparato, ma NON sulla promessa 1»*. Promessa 1 è violata in **due** modi — byte sbagliati, o niente — e ora entrambi portano le sue parole |
| **E14** | Task 4, Step 2 — i test negativi | `assert_eq!(caught.as_deref(), Some(READ_BACK_MESSAGE))` | ⛔ **non può passare, per cinque casi su sei.** `assert_eq!` con messaggio personalizzato produce `` assertion `left == right` failed: <messaggio> `` più i due valori: il payload non è **mai** uguale alla costante. Solo l'unico `assert!` della suite corrisponderebbe esatto. Sostituito con `contains` — che è il modo di `reactor_contract.rs` — più il vincolo, ora scritto, che **nessun messaggio sia sottostringa di un altro** |
| **E15** | Task 5, Step 3 | attende `test result: ok. 4 passed` | **criterio di chiusura stantio**, la stessa specie di **E4**: con sei bugiardi e la finta onesta i test erano **7**, e sono **8** dalla decisione **E19**, che ne porta un settimo. ⚠️ **Questa cella è invecchiata una volta mentre la si scriveva** — diceva 7 — ed è la dimostrazione del gotcha **#31** dentro la riga che lo cita. `cargo test --workspace`: **25 target, 101 test** |
| **E16** | Task 5, elenco dei file e `git add` | `ports/journal.rs`, `simulator/journal.rs`, la spec | **incompleto in due punti.** L'arrivo di `replay` rende **false** tre frasi scritte altrove — il doc di modulo della porta (*«`replay` non è qui»*), il doc di `MemoryJournal` (*«nulla chiede quell'ordine oggi»*) e un commento in `crates/simulator/tests/memory_journal.rs` (*«`replay` non esiste ancora»*). Il terzo file **non è nell'elenco**. Corrette tutte e tre con richiamo datato: una frase di stato disallineata mente con autorevolezza. ⛔ **E un quarto file manca, questo per una ragione che non è di documentazione: un metodo aggiunto a un trait rompe OGNI implementatore.** `crates/kernel/tests/boundary_promotion.rs` ne ha **due** — `RecordingJournal` e `RefusingJournal` — e senza `replay` il workspace **non compila**: `E0046`, e la porta era **rossa** fra lo Step 2 e lo Step 5 del Task 5. È il costo ricorrente della decisione **D6**, e va messo in conto ogni volta che una porta cresce |
| **E17** | Task 4, Step 3 | attende un errore, `no method named replay found` | **misurati sette**: un `E0599` — quello atteso — e **sei** `E0407`, *«method `replay` is not a member of trait `Journal`»*, uno per bugiardo. La conclusione regge: il rosso è quello giusto e per la ragione giusta |
| **E18** | Task 4, Step 1 — il sorgente dettato | tre righe della suite | **non sono `rustfmt`-clean** mentre il resto del workspace lo è: due catene `.expect()` superano `fn_call_width` e una costante di messaggio va a capo dove `rustfmt` non vuole. Riformattate col solo `rustfmt --edition 2024` **sul file nuovo** — non `cargo fmt --all`, che riscriverebbe file interi e ne normalizzerebbe i fine-riga (vincolo globale 5). È la **seconda occorrenza** di **E6** |
| **E19** | ⛔ **una decisione presa eseguendo, e non dal piano** — nessun compito la assegnava | il piano lascia `intent` **senza guardia** e la questione aperta nel registro | ⛔ **Decisa il 2026-08-10 dal coordinatore, in revisione: un secondo `intent` sullo stesso passo è RIFIUTATO.** ⚠️ **È un'aggiunta al contratto di una porta condivisa**, ed è scritta qui perché il proprietario possa **ribaltarla vedendola** invece di scoprirla. Le ragioni: ADR-0007 dice *«l'intento di **ogni** passo»*, uno per passo, quindi un secondo è **fuori dal modello**; è la metà **simmetrica** della promessa su `outcome`, V6 tenuta dalla **porta** e non dalla diligenza del chiamante; YAGNI non si applica, perché su una porta dichiarata in anticipo i chiamanti sono vuoti **per costruzione** (gotcha #46); e costa **una riga** adesso contro due implementazioni e un archivio dopo. ⛔ **Cercato dove fosse già stata valutata** (gotcha #32): la voce del registro diceva *«non è una decisione presa, è un comportamento mai interrogato»* — mai valutata, mai scartata. **Opzioni scartate:** «ultimo che vince» contraddice la promessa 2; «accettare in silenzio» è ciò che c'era, e regge solo **per accidente del disegno della chiave** — chi chiavasse sul passo divergerebbe senza che nulla diventi rosso. ⛔ **Nessuna variante d'errore nuova:** `OutOfOrder` si **allarga**, perché la porta dichiara il proprio tipo d'errore *«deliberatamente povero»*. Eseguita come **promessa 6** della conformità, col bugiardo `UnguardedIntentJournal`; la sonda del simulatore **cambia oggetto** e guadagna la propria contro-sonda; la voce del registro si **chiude** con data e ragione |
| **E20** | ⛔ Task 7, Step 5 — la lista delle vie di `boundary.rs` | dettato come lavoro del **Task 7** | ⛔ **in parte già consumato, e si dice adesso perché chi lo eseguirà non lo rifaccia** — gotcha **#49** scoperto **prima** invece che eseguendo. La via **A6** è chiusa qui: la suite esiste e coglie `SilentJournal`, quindi la riga di `boundary.rs` porta già il proprio `✅ CLOSED`, e con essa l'intestazione della lista — che diceva *«ciò che NON è coperto»* con due voci chiuse dentro — e il capoverso di chiusura, che diceva *«il residuo è dichiarato, non risolto»* quando una l'ha risolta. ⚠️ **Al Task 7 resta la sua metà vera**: la via **A4**, che si chiude con l'etichetta di fiducia **nel record scritto da `promote`**, e la domanda aperta su `promote` che la decisione **D5** risponde. ⚠️ La metà compendio era già stata fatta col commit precedente, e le due si contraddicevano fra loro per un giro |
| **E21** | Task 4, Step 1 — promessa su `replay`, seconda lettura | payload `first`, `second`, `third` | ⚠️ **una seconda simmetria latente, oltre a quella di E12**: le **lunghezze** sono `5, 6, 5`, palindrome anch'esse. Innocua finché si confrontano i byte, ma il blocco tornerebbe vacuo alla prima riscrittura che confronti **dimensioni** — che è *esattamente* il modo in cui è diventato vacuo la prima volta. Terzo payload allungato a `third and last`: lunghezze `5, 6, 14` |
| **E22** | ⛔ **una decisione presa eseguendo, e non dal piano** — `Record::encode`, passo preliminare al Task 6 | il Task 1 lascia la **questione aperta** accanto alla funzione, e ogni compito successivo detta `.encode().expect("encode")` | ⛔ **Decisa il 2026-08-10 dal coordinatore: la firma diventa `pub fn encode(&self) -> Vec<u8>`.** ⚠️ **È il cambio di una firma pubblica**, ed è scritta qui perché il proprietario possa **ribaltarla vedendola**. Le ragioni: il repository ha **già** questa posizione, presa per `Ipc::accept` e scritta nella §6 del compendio — *«un `Result` che non può mai essere `Err` è superficie morta»*; al **Task 7** `promote` dovrà chiamarlo, e un `.expect` che non può sparare **dentro il codice del confine dei dati non fidati** è debito e non prudenza; i chiamanti sono **pochi** oggi e saranno molti dopo, quindi l'edit costa il minimo adesso. ⚠️ **Questa cella diceva «due», e contandoli erano UNO** — `crates/kernel/tests/record_shape.rs` con **nove** siti; `compile_fail/record_without_version.rs` era stato contato per errore e **non è un chiamante**, perché nomina `RecordV1::encode`, il metodo inerente che quel caso esiste per provare **assente**. ✅ L'errore va **a favore** dell'argomento. Corretto il 2026-08-10, gotcha **#31**. ⛔ **`RecordError` resta** — `decode` ne ha bisogno davvero — e il suo doc si **restringe** a «decoding», datato invece che riscritto. ⛔ **La misura del Task 1 non è stata cancellata ma promossa a evidenza:** `Vec<u8>` come `Write` di `minicbor` ha `Error = Infallible`, e le altre due strade (`Message`, `Custom`) hanno due soli produttori nella 2.3.0 — `SystemTime` e un `Path` non-UTF-8 — nessuno dei due nel grafo di questo tipo. ⚠️ **E il caso impossibile è contenuto, non ignorato:** il `Result` si scarta senza `expect` — che sposterebbe il ramo morto di un livello invece di toglierlo — perché un encoder che si fermasse a metà lascerebbe byte **troncati o vuoti**, e `Record::decode` risponde `Malformed` a **entrambi** (già tenuto da `bytes_that_are_not_a_record_decode_to_malformed`), quindi la riconciliazione legge `SuspendAndAsk` e il sistema **si ferma invece di indovinare**. Sonde di `record_shape.rs` rilanciate: **10 verdi**, `cargo test --workspace --no-fail-fast` **25 target, 101 test**, invariati |
| **E23** | Task 6, Step 3 — doc di `Resolution` | il sorgente dettato porta le parole italiane `` `verificabile` ``, `` `idempotente` ``, `` `irripetibile` `` nei commenti | ⛔ **la §1.0 vieta l'italiano nel sorgente**, ed è la **terza** occorrenza di **E1** dentro lo stesso piano: il piano **non ha l'autorità di derogarvi**. Gli identificatori inglesi esistono già — `EffectClass::Verifiable`, `Idempotent`, `Unrepeatable` — e sono quelli usati. Gotcha **#40** |
| **E24** | Task 6, Step 3 — `steps_in_doubt` dettata | il ramo `Err(_)` fa `open.push(..)` senza guardare se quel passo ci sia già | ⛔ **Misurato prima di decidere, e il numero è la voce:** intento valido per il passo 5, poi un record **indecifrabile** per il passo 5, e l'insieme risponde `[{5, RunAgain}, {5, SuspendAndAsk}]` — **lo stesso passo due volte**. Una funzione che si chiama *«i passi in dubbio»* e rende lo stesso passo due volte **non è un insieme**, e un chiamante lo sospenderebbe due volte. ⚠️ **E non è il solo produttore del doppione:** anche il caso di **E25** — un `outcome` col `kind` sbagliato — ne produceva uno. **Rimedio deliberato e scritto:** l'insieme si mantiene con `enter`/`leave` chiavati sul passo — `enter` **sostituisce sul posto** la risoluzione di un passo già in dubbio senza spostarlo, `leave` lo toglie. Tre regole scritte e ciascuna con la propria sonda: **al più una voce per passo** · **un record indecifrabile vince** su ciò che è stato letto prima, perché non dice nulla — nemmeno che il passo si sia chiuso — e ADR-0007 dice che un dubbio non risolvibile **ferma** · un passo che **rientra** in dubbio **conserva il posto** che aveva preso, perché non ha smesso di essere in dubbio, è cambiata solo la risposta. ⚠️ **Il caso gemello, guardato come chiesto:** un record indecifrabile **dopo** un esito valido **non** duplicava — l'esito lo aveva già tolto — ma **rimette il passo in dubbio**, ed è la risposta giusta per la stessa ragione. Sonde: `a_step_is_in_doubt_at_most_once_however_many_records_it_carries`, `a_step_that_re_enters_doubt_keeps_the_place_it_first_took`, `an_unreadable_record_after_a_readable_outcome_puts_the_step_back_in_doubt` |
| **E25** | ⛔ Task 6, Step 6 — la **domanda 2**, che il compito dichiara la più importante | *«serviva distinguere `intent` da `outcome` senza decodificare? Se sì, la porta sta restituendo troppo poco»* | ⛔ **Misurato, e la risposta è «no» alla lettera e «sì» nella sostanza.** Alla lettera: `Vec<(StepId, Vec<u8>)>` è **bastato** e `steps_in_doubt` non si è contorta. ⛔ **E la terza domanda — la copia dei byte — aveva una risposta scritta prima della misura, che era falsa:** «una volta sola», ragionata sui tipi. Misurati i puntatori su un payload da 4096 B, le allocazioni sono **tre** — la sorgente, il clone che `replay` consegna (record intero, 4106 B) e il payload che `decode` materializza fuori da quel buffer — e la riconciliazione **lo butta subito**, perché legge solo `kind` ed `effect`. ⚠️ Non è una ragione per cambiare `replay`: prestare i byte si scontra con le durate di una transazione `redb` al Task 8, e il rimedio sarebbe una decodifica che si ferma all'intestazione, che nessuna misura chiede oggi. Gotcha **#15** rivolto a chi scriveva. Nella sostanza: `replay` non dice **quale delle sue due operazioni** ha scritto ciascuna voce, mentre il giornale **lo sa** — `MemoryJournal` tiene un `EntryKind` interno e `JournalError::OutOfOrder` è **definito** in termini delle due operazioni. Quindi la riconciliazione ricostruisce quella distinzione da una **seconda verità indipendente**, il campo `kind` del record. ⚠️ **Misurate entrambe le direzioni del disaccordo, e falliscono in modo diverso:** un record scritto con `intent()` il cui `kind` dice `Outcome` → il passo **non è riportato**, cioè **un dubbio vero sparisce in silenzio**, che è l'unico fallimento che ADR-0007 esiste per impedire; un record scritto con `outcome()` il cui `kind` dice `Intent` → il passo è riportato benché concluso, e **prima** del rimedio di E24 era riportato **due volte**. ⚠️ **Non è un difetto oggi**: nessun codice del kernel scrive ancora un record — `promote` lo guadagna al Task 7 — quindi le due verità non possono divergere se non scrivendo un record che contraddice la chiamata. ⛔ **La firma NON è stata cambiata, ed è deliberato:** chiuderla significa che `replay` restituisca l'operazione, e ciò tocca la **porta**, la **conformità** e **due** implementazioni — un contratto condiviso, quindi **riportato al coordinatore invece che preso scrivendo un consumatore**. ⛔ **E la conseguenza che verrebbe con esso è scritta adesso perché non si scopra dopo:** se l'autorità passa alla porta, il campo `kind` del record diventa **ridondante** — e `record.rs` lo chiama *«il campo su cui poggia l'intero protocollo write-ahead»*. O se ne va, ed è un cambio di **formato** di cui i byte congelati del Task 10 sono la scadenza, o resta come **riscontro incrociato che qualcuno controlla davvero**. Una ridondanza che nessuno controlla è ciò che ADR-0036 ha rifiutato tenendo **un** oracolo invece di due. Dichiarata come questione aperta in `crates/kernel/src/reconcile.rs` |
| **E26** | Task 6, Step 4 — e le due divergenze meccaniche del sorgente dettato | attende `test result: ok. 4 passed` | **criterio di chiusura stantio**, la **terza** occorrenza di **E4** ed **E15**: i test sono **9**, perché quattro sonde mancavano (giornale vuoto, ordine, insieme, gemella dell'indecifrabile). `cargo test --workspace --no-fail-fast`: **26 target, 110 test**. ⚠️ **E il banco dettato non compilava per due ragioni indipendenti dalla logica:** l'aiutante chiama `.encode().expect("encode")`, che **E22** ha appena tolto; e l'ordine dei `use` non è `rustfmt`-clean — `kernel::reconcile` viene **prima** di `kernel::record` (`n` < `r` al quinto carattere) — che è la **terza** occorrenza di **E6**. Riformattato col solo `rustfmt --edition 2024` sui due file nuovi, mai `cargo fmt --all` (vincolo globale 5) |

---

> **Per chi esegue:** SKILL RICHIESTA — usa `superpowers:subagent-driven-development`
> (consigliata) o `superpowers:executing-plans` per eseguire questo piano compito per
> compito. I passi usano le caselle (`- [ ]`) per il tracciamento.

**Obiettivo:** dare al kernel un **record durevole che dichiara la propria versione**, un
giornale che lo scrive davvero in due implementazioni, la **riconciliazione** che lo rilegge
dopo un crash, e i **byte congelati** che ne sorvegliano l'evoluzione.

**Architettura:** il record vive in `kernel` e la porta `journal` scambia **byte** — la
codifica è proprietà del kernel (ADR-0036). Due implementazioni: il doppio in memoria in
`simulator` e `redb` col backend nostro in `platform`. Una **suite di conformità** in una
copia sola dice cosa entrambe promettono; tre **bugiardi** provano che sa fallire. I byte
congelati entrano **per ultimi**, quando il formato è già stato esercitato.

**Stack:** Rust (`rustc` 1.95.0, `cargo` 1.95.0), edition 2024 · `kernel` e `simulator`
`#![no_std]` + `alloc` + `#![forbid(unsafe_code)]` · `minicbor` 2.3.0 **già nella lista** di
ADR-0031 · `redb` 4.1.0 in `platform`, che ADR-0031 **non** vincola.

**Spec di riferimento:**
[`2026-08-06-sottoprogetto-1-kernel.md`](../specs/2026-08-06-sottoprogetto-1-kernel.md) —
**§4** per intero e **§4.9** in particolare, più §7.4.1 e §7.4.2 per le righe di catalogo che
questo traguardo implementa. Il *perché* sta lì e negli ADR **0007**, **0018** e **0036**;
qui c'è solo il *come*.

---

## ⛔ Le sette decisioni che questo piano prende, con la ragione

La spec non le fissa. Sono scritte qui perché una scelta non dichiarata è una scelta che il
prossimo rifà da capo — o peggio, ribalta senza sapere che era stata fatta.

| # | Decisione | Ragione |
|---|---|---|
| **D1** | l'ordine è **formato → consumatore → seconda implementazione → congelamento**, e i byte congelati sono **l'ultimo compito** | i byte congelati non si rigenerano mai (vincolo 14 §11). Congelarli prima che un consumatore reale abbia esercitato il formato significa congelare la forma sbagliata — il difetto del Task 11 del Traguardo 2, dove il piano dettava una porta che compilava e non era implementabile |
| **D2** | il vincolo 14 si legge **«nello stesso traguardo che introduce il record»**, non «nel primo commit che ne scrive uno» | in questo traguardo non esiste nessun archivio di produzione, nessun utente e nessun dato irriproducibile: i record che si scrivono sono di test. La finestra che il vincolo protegge si chiude quando il sistema comincia a **conservare**, e questo traguardo finisce prima. ⚠️ Dichiarata perché è un'**interpretazione**, non una lettura ovvia |
| **D3** | codifica **ad array**, che è il default di `minicbor` — **non** a mappa | ⛔ **misurato in ADR-0036, e la prima stesura di questo piano stava per sbagliarlo**: array **27 byte (+4 %)**, mappa **33 (+27 %)**, posizionale 26. L'ADR nota che la stima precedente *«prezzava la mappa invece dell'array»*. Scriverlo **esplicito** anche se è il default: un default non dichiarato è un default che qualcuno cambia |
| **D4** | l'**etichetta di fiducia** è un campo del record dal primo giorno | è la via **A4** di `crates/kernel/src/boundary.rs`, che dichiara il prezzo del non farlo: *«retrofitted later only by migrating the one irreproducible archive»*. Un campo che si **calcola dopo** costa un indice nuovo; un campo che porta **informazione che altrimenti si perde** non si aggiunge più |
| **D5** | una **promozione non è un passo proprio**: è una nota sul passo del chiamante | ADR-0007 fissa la granularità — *«un passo è un'interazione con il mondo esterno»* — e una promozione non tocca nulla fuori. Chiude la domanda che `promote` dichiara aperta, e toglie il difetto che ogni promozione lascia oggi: un passo con intento e senza esito, **in dubbio per sempre e mai potabile** |
| **D6** | la porta guadagna **`replay()`**, e la firma si decide **scrivendo la riconciliazione** | in questo progetto **una porta cresce quando arriva il suo primo consumatore**: è la regola con cui `Wakeup::EventReady` fu tolta al Task 4 del Traguardo 2 (E8, E9) e con cui la §6 del compendio tiene aperta la crescita di `reactor`. Decidere la firma prima del consumatore è rifare il Task 11 |
| **D7** | ⛔ la **ritenzione** — il record potato con impronta e dimensione — **resta fuori** | l'impronta pretende una funzione di hash, e nel kernel sarebbe una **voce nuova nella lista di ADR-0031**: un atto deliberato che richiede una misura che nessuno ha fatto. Resta dentro la sola regola che **non** ha bisogno di impronta: *`prune` di un passo in dubbio è rifiutato*. Dichiarata, non dimenticata |

---

## Cosa questo traguardo NON contiene, e non è una dimenticanza

| | Chi lo chiude |
|---|---|
| il backend **cadente** e l'iniezione di guasti fra intento ed esito | **Traguardo 4** — la §3.3 assegna *«caduta fra intento ed esito»* alla campagna, e il backend cadente è il **livello 2** di crash (§4.6). Qui nascono le implementazioni; là si rompono |
| ⚠️ `replay()` **carica tutto in memoria** | il primo consumatore che misuri un giornale grande. Il rimedio noto è un **checkpoint**, e progettarlo ora fisserebbe un meccanismo che nessuna misura ha toccato |
| il **record potato** con impronta e dimensione | **D7** — il traguardo che porta la ritenzione, con la decisione sull'impronta |
| la **durabilità attraverso il riavvio del processo** in conformità | ⛔ **non ci va mai**: è una proprietà della sola implementazione reale, e pretenderla in conformità renderebbe **rossa la finta, che è corretta**. Gotcha **#44**, e questa volta è dichiarato prima invece che scoperto eseguendo. Vive come test della sola `platform` |
| le vie **A1/A2**, **A5** e **A7** di `promote` | ⛔ **nessuno, e sono dichiarate non chiudibili** in `boundary.rs`. Questo traguardo ne chiude **due** delle sette — **A6** con la conformità, **A4** con l'etichetta nel record — contro l'una sola chiusa oggi |
| la **ricomposizione della proiezione** | §0.4 regola **C**: non ha consumatore finché nessuno chiama un modello |
| ⛔ **il resto del modello dello stato durevole di §4.4** — obiettivo, vincoli, piano, decisioni con il motivo, fatti con la provenienza, artefatti come riferimenti | il traguardo che porta il primo consumatore. ⚠️ **Va detto perché è una scelta e non una svista:** la §4.4 elenca cosa il giornale conterrà, e il record di questo traguardo ne porta **quattro campi** — tipo, classe, etichetta, payload. Gli altri non hanno **nessuno che li scriva**: senza capacità L2 non esiste un obiettivo, né un piano, né una decisione da registrare. ✅ E aggiungerli dopo **costa un indice nuovo ciascuno**, che è esattamente il caso che la regola 3 di §4.9.2 dichiara ✅ misurato — a differenza dell'etichetta di fiducia, che porta informazione che **si perderebbe** (decisione D4). È la distinzione fra le due categorie, applicata |

---

## Le quattro specie di difetto di piano, da tenere davanti eseguendo

Questo piano è un'**ipotesi**. Nel Traguardo 2 il difetto è stato nel piano molto più spesso
che nel codice, in quattro forme distinte — e ciascuna si coglie con una domanda che non
coglie le altre tre:

| # | Specie | La domanda che la trova |
|---|---|---|
| 1 | la **sonda è sbagliata** — vacua, o attacca il caso invece del meccanismo | si coglie **rileggendo** e confrontandola con ciò che dovrebbe far scattare |
| 2 | la **sonda manca** | *per ogni artefatto che il compito produce, quale controllo lo esercita?* Non si vede leggendo: non c'è niente da leggere |
| 3 | l'**artefatto è sbagliato**, e compila | si vede **solo** scrivendone un'implementazione **da fuori dalla crate** |
| 4 | il **compito è già eseguito** — gotcha #49 | *prima di eseguire, ciò che questo compito detta di produrre esiste già?* |

⛔ **E il banco con cui misuri sbaglia _verso l'attesa_** — gotcha **#48**, nove esiti
credibili e falsi in due sessioni. Prova che la mutazione **si sia applicata**, compila in un
passo **separato** dall'eseguire, e per ogni mutazione su un valore **provane due**.

---

## Vincoli globali

| # | Vincolo |
|---|---|
| 1 | ⛔ **codice in inglese, documentazione in italiano** — §1.0 della spec. Vale per nomi di tipi, funzioni, messaggi d'uscita e **commenti nel sorgente** |
| 2 | `kernel` e `simulator` restano `#![no_std]` + `alloc` + `#![forbid(unsafe_code)]` |
| 3 | ⛔ **nessuna voce nuova nella lista di ADR-0031.** `minicbor` è già spedito, `redb` vive in `platform` che la lista non vincola. Se un compito ne chiedesse una, **si ferma e si chiede** |
| 4 | `crates/kernel/tests/compile_fail.rs` **non si modifica**: la sua guardia conta i casi e non ha un numero atteso, e il banco usa un **glob** — un caso nuovo entra da solo |
| 5 | ⛔ **i fine-riga sono misti per file.** Nessuno strumento che riscriva un file intero |
| 6 | ogni regola nuova porta **due** sonde — che scatti dove deve, e che **non** scatti dove non deve (gotcha #24) — e un caso in `tests/compile_fail/` col suo `.stderr`, **da leggere e non da rigenerare in blocco** |
| 7 | ⛔ **un controllo nuovo entra prima nel catalogo §7.4 della spec, poi nel registro.** L'ordine inverso è il gotcha **#36**, successo tre volte |
| 8 | alla chiusura di ogni compito: `bash scripts/gate.sh` deve stampare `GATE GREEN`, poi si committa |

---

## Struttura dei file

| File | Responsabilità |
|---|---|
| `crates/kernel/src/record.rs` | **nuovo** — il record durevole: l'enum di versione, i campi con indice, la classe dell'effetto, l'etichetta di fiducia. La codifica vive qui (§4.9.3) |
| `crates/kernel/src/ports/journal.rs` | **modificato** — `replay()`, e il doc che smette di dire «milestone 3 arriva» |
| `crates/kernel/src/boundary.rs` | **modificato** — `promote` scrive un record vero e diventa una nota sul passo del chiamante (D5) |
| `crates/kernel/src/reconcile.rs` | **nuovo** — la riconciliazione: da `replay()` all'insieme dei passi in dubbio |
| `crates/simulator/src/journal.rs` | **nuovo** — il doppio in memoria |
| `crates/platform/src/journal.rs` | **nuovo** — `redb` col `StorageBackend` nostro |
| `crates/kernel/tests/journal_contract.rs` | **nuovo** — la suite di conformità e i tre bugiardi, in **una copia sola** |
| `crates/platform/tests/journal_contract_real.rs` | **nuovo** — `include!` della suite, contro l'implementazione vera |
| `crates/kernel/tests/frozen_bytes.rs` | **nuovo** — l'oracolo dei byte congelati |
| `crates/kernel/tests/frozen/record_v1.cbor` | **nuovo** — i byte, che **non si rigenerano** |
| `crates/kernel/tests/frozen/record_v1.map` | **nuovo** — la mappa `indice → nome → valore atteso` |

---

# Parte A — il formato, e le due cose che lo provano

## Task 1: Il record durevole — l'enum di versione e i campi con indice

**Files:**
- Create: `crates/kernel/src/record.rs`
- Modify: `crates/kernel/src/lib.rs`
- Test: `crates/kernel/tests/record_shape.rs`

⛔ **La regola 1 di ADR-0036 in una riga:** *«un record senza versione» non deve essere
esprimibile*. Si ottiene facendo del record un **enum di versione**, e l'enum non è cerimonia
di tipi: `minicbor` codifica un enum come array a due elementi — indice della variante, poi
valore — quindi **la versione finisce nei byte**. Una passata YAGNI che togliesse l'enum a
una variante sola toglierebbe un byte dal formato, non un livello di indirezione. Scritto nel
sorgente perché non venga tolto.

- [ ] **Step 1: Scrivere il test che fallisce**

Crea `crates/kernel/tests/record_shape.rs`:

```rust
//! Counter-probes for the durable record (§4.9). The probe that must FIRE lives in
//! `tests/compile_fail/record_without_version.rs`; these are the other direction, the one
//! that is forgotten (§7.1.1, rule 3).

use kernel::record::{EffectClass, Record, RecordKind, RecordV1, Trust};

#[test]
fn a_record_round_trips_through_its_own_encoding() {
    let original = Record::V1(RecordV1 {
        kind: RecordKind::Intent,
        effect: EffectClass::Idempotent,
        trust: Trust::Instruction,
        payload: b"why this step exists".to_vec(),
    });

    let bytes = original.encode().expect("encode");
    let read = Record::decode(&bytes).expect("decode");

    assert_eq!(read, original);
}

#[test]
fn the_version_is_in_the_bytes_and_not_only_in_the_type() {
    // `minicbor` encodes an enum as a two-element array: variant index, then value. So the
    // version travels WITH the record and a reader that has never seen the type can still
    // tell which version it is holding. Measured here rather than assumed: the first byte
    // of a two-element array is 0x82.
    let bytes = Record::V1(RecordV1 {
        kind: RecordKind::Intent,
        effect: EffectClass::Idempotent,
        trust: Trust::Instruction,
        payload: Vec::new(),
    })
    .encode()
    .expect("encode");

    assert_eq!(bytes[0], 0x82, "the record must encode as a 2-element array");
    assert_eq!(bytes[1], 0x00, "the second item must be the version index");
}

#[test]
fn a_payload_is_a_byte_string_and_not_an_array_of_numbers() {
    // Gotcha #35: without the byte-string annotation `minicbor` encodes a `Vec<u8>` as an
    // ARRAY OF NUMBERS. It compiles, it round-trips, and it costs 1.91x — measured on 4096 B:
    // 7813 against 4101. The annotation is load-bearing, so a test holds it.
    let small = Record::V1(RecordV1 {
        kind: RecordKind::Intent,
        effect: EffectClass::Idempotent,
        trust: Trust::Instruction,
        payload: vec![0xAA; 64],
    })
    .encode()
    .expect("encode");

    // 64 bytes as a byte string cost 64 + 2 of header. As an array of numbers each value
    // above 0x17 costs TWO bytes, so the array form could not fit under 100.
    assert!(
        small.len() < 100,
        "payload encoded as an array of numbers, not a byte string: {} bytes",
        small.len()
    );
}

#[test]
fn the_two_record_kinds_are_distinguishable_in_the_bytes() {
    let intent = Record::V1(RecordV1 {
        kind: RecordKind::Intent,
        effect: EffectClass::Idempotent,
        trust: Trust::Instruction,
        payload: Vec::new(),
    })
    .encode()
    .expect("encode");

    let outcome = Record::V1(RecordV1 {
        kind: RecordKind::Outcome,
        effect: EffectClass::Idempotent,
        trust: Trust::Instruction,
        payload: Vec::new(),
    })
    .encode()
    .expect("encode");

    assert_ne!(intent, outcome);
}
```

- [ ] **Step 2: Lanciare il test e verificare che fallisca**

```bash
cargo test -p kernel --test record_shape
```

Atteso: **FALLISCE** con `error[E0432]: unresolved import kernel::record`.

⚠️ Se fallisse per un motivo diverso, **fermarsi**: un test negativo che fallisce per la
ragione sbagliata è il gotcha #14.

- [ ] **Step 3: Scrivere il record**

Crea `crates/kernel/src/record.rs`:

```rust
//! The durable record (§4.9). ⛔ EVERY DURABLE RECORD DECLARES ITS OWN VERSION, AND ITS
//! FIELDS ARE IDENTIFIED BY EXPLICIT INDEX — ADR-0036, and the six rules are in §4.9.2.
//!
//! ⛔ THE ENCODING LIVES HERE, IN `kernel`, and the `journal` port exchanges BYTES. Three
//! reasons, from §4.9.3: the data model is the kernel's property (§4.4); with bytes on the
//! port the SIMULATOR EXCHANGES BYTES TOO, so the DST campaign really exercises encoding and
//! decoding instead of going around them; and the measured cost is small.
//!
//! ⛔ ARRAY ENCODING, NOT MAP, AND IT IS WRITTEN OUT EVEN THOUGH IT IS THE DEFAULT.
//! Measured in ADR-0036: array 27 bytes (+4 %), map 33 (+27 %), positional 26. The ADR notes
//! that the earlier estimate "priced the map instead of the array" — so the number that
//! decided this is the array one. A default nobody wrote down is a default somebody changes.

use alloc::vec::Vec;
use minicbor::{Decode, Encode};

/// Is this the INTENTION of a step or its OUTCOME? The whole write-ahead protocol rests on
/// telling them apart: a step with an intent and no outcome is IN DOUBT (§4.2), and the
/// doubt is what makes recovery possible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cbor(index_only)]
pub enum RecordKind {
    #[n(0)]
    Intent,
    #[n(1)]
    Outcome,
}

/// How an effect may be reconciled after a crash (ADR-0007).
///
/// ⛔ THE CLASS IS A MANDATORY FIELD OF THE RECORD, and that is the point: §7.4.4 raised V5
/// to the compiler precisely so that "an effect without a declared class" IS NOT
/// EXPRESSIBLE. A defaulted class would put the decision back where the risk is — the
/// forgetfulness of whoever writes.
///
/// ⚠️ The `irripetibile` default of ADR-0007 is NOT gone: it lives where it is actually
/// useful, on records READ BACK from a journal written before the class existed. Under
/// ADR-0036 that is not a special case — it is the ordinary case of a field absent in an
/// earlier version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cbor(index_only)]
pub enum EffectClass {
    /// Ask the world what happened, then finish or re-plan.
    #[n(0)]
    Verifiable,
    /// Just run it again.
    #[n(1)]
    Idempotent,
    /// ⛔ Suspend and ask the user. Also what an undeclared class means.
    #[n(2)]
    Unrepeatable,
}

/// Whether the payload of this record crossed the untrusted boundary (I6, ADR-0014).
///
/// ⛔ THIS FIELD IS WHY IT IS HERE ON DAY ONE, and the reason is written where it was found:
/// road A4 of `crate::boundary`. Write external text into the journal, read it back as raw
/// bytes, and it comes out indistinguishable from an instruction — BYTES CARRY NO LABELS.
/// The record is where a label can live, and `boundary.rs` prices the alternative exactly:
/// "retrofitted later only by migrating the one irreproducible archive".
///
/// ⚠️ AND THE LIMIT IS THE TOKEN'S LIMIT, declared rather than discovered later: this proves
/// PROVENANCE, NOT CORRECTNESS (§6.3.2). Whoever writes a record can label it wrongly. What
/// it buys is that a reader can no longer LOSE the distinction, which is a different thing
/// from making it impossible to lie about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
#[cbor(index_only)]
pub enum Trust {
    /// The payload may be used as an instruction.
    #[n(0)]
    Instruction,
    /// ⛔ The payload came from outside and stays outside (V20). Reading it back yields
    /// `Untrusted`, never a `String` that somebody may hand to the instruction channel.
    #[n(1)]
    Untrusted,
}

/// Version 1 of the durable record.
///
/// ⛔ EVERY FIELD CARRIES AN EXPLICIT INDEX, and the indices follow four rules that no
/// compiler enforces (§4.9.2): a new field is OPTIONAL and takes a NEW index; an index is
/// RETIRED AND NEVER REUSED — the gap stays; a non-additive change opens a NEW VERSION.
/// What holds them is the frozen bytes of `tests/frozen_bytes.rs`, a level 2 check.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct RecordV1 {
    #[n(0)]
    pub kind: RecordKind,
    #[n(1)]
    pub effect: EffectClass,
    #[n(2)]
    pub trust: Trust,
    /// ⛔ THE BYTE-STRING ANNOTATION IS LOAD-BEARING, not decoration. Without it `minicbor`
    /// encodes a `Vec<u8>` as an ARRAY OF NUMBERS: it compiles, it round-trips, and it costs
    /// 1.91x — measured on 4096 B, 7813 against 4101. Gotcha #35.
    #[n(3)]
    #[cbor(with = "minicbor::bytes")]
    pub payload: Vec<u8>,
}

/// The durable record. ⛔ A RECORD WITHOUT A VERSION IS NOT EXPRESSIBLE — rule 1 of §4.9.2,
/// held at level 1 by the type itself.
///
/// ⚠️ ONE VARIANT TODAY, AND IT IS NOT CEREMONY — written down because a YAGNI pass would
/// remove it and would be wrong. `minicbor` encodes an enum as a two-element array: variant
/// index, then value. So the version TRAVELS IN THE BYTES. Removing the enum would not
/// remove a level of indirection, it would remove a byte from the format — and that byte is
/// the whole of rule 1. Contrast with `Wakeup`, deleted at milestone 2 (errata E9): that one
/// wrapped a value and bought no error anywhere; this one is written to the archive.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum Record {
    #[n(0)]
    V1(#[n(0)] RecordV1),
}

/// What can go wrong encoding or decoding a record.
///
/// ⚠️ Deliberately poor, and for the reason `JournalError` is: a rich error invites the
/// kernel to branch on the reason, and there is exactly one thing to do with a record that
/// will not decode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordError {
    /// The bytes are not a record of any version this build knows.
    Malformed,
}

impl Record {
    /// Encodes to the bytes the `journal` port exchanges.
    pub fn encode(&self) -> Result<Vec<u8>, RecordError> {
        let mut bytes = Vec::new();
        minicbor::encode(self, &mut bytes).map_err(|_| RecordError::Malformed)?;
        Ok(bytes)
    }

    /// Decodes from the bytes the `journal` port hands back.
    pub fn decode(bytes: &[u8]) -> Result<Self, RecordError> {
        minicbor::decode(bytes).map_err(|_| RecordError::Malformed)
    }
}
```

Aggiungi in `crates/kernel/src/lib.rs`, accanto agli altri `pub mod`:

```rust
pub mod record;
```

- [ ] **Step 4: Lanciare il test e verificare che passi**

```bash
cargo test -p kernel --test record_shape
```

Atteso: `test result: ok. 4 passed`.

⚠️ Se `the_version_is_in_the_bytes_and_not_only_in_the_type` fallisce sui byte attesi,
**non aggiustare l'asserzione al valore osservato**: leggi i byte veri e capisci quale forma
`minicbor` ha prodotto. Un'asserzione allineata all'osservazione è il gotcha #15, e qui
costerebbe il formato.

- [ ] **Step 5: Il caso negativo — un record senza versione non compila**

Crea `crates/kernel/tests/compile_fail/record_without_version.rs`:

```rust
//! Catalogue §7.4.1 block C, row `Q14 · §4.9` — a durable record WITHOUT a version is not
//! expressible. The type is a version enum, so the inner value cannot stand alone where a
//! record is expected.

fn main() {
    let inner = kernel::record::RecordV1 {
        kind: kernel::record::RecordKind::Intent,
        effect: kernel::record::EffectClass::Idempotent,
        trust: kernel::record::Trust::Instruction,
        payload: alloc_free_payload(),
    };

    // The bare V1 body is NOT a record: only `Record::V1(..)` is.
    let _bytes = inner.encode();
}

fn alloc_free_payload() -> Vec<u8> {
    Vec::new()
}
```

- [ ] **Step 6: Generare l'oracolo, e leggerlo**

```bash
TRYBUILD=overwrite cargo test -p kernel --test compile_fail
```

⛔ **Poi apri `crates/kernel/tests/compile_fail/record_without_version.stderr` e leggilo.**
Deve nominare `encode` come metodo che non esiste su `RecordV1`. Se nomina altro, il caso
sta provando qualcos'altro — gotcha #25.

- [ ] **Step 7: Provare la regola in negativo**

Aggiungi temporaneamente a `crates/kernel/src/record.rs`:

```rust
impl RecordV1 {
    pub fn encode(&self) -> Result<Vec<u8>, RecordError> {
        let mut bytes = Vec::new();
        minicbor::encode(self, &mut bytes).map_err(|_| RecordError::Malformed)?;
        Ok(bytes)
    }
}
```

```bash
cargo test -p kernel --test compile_fail
```

Atteso: `record_without_version.rs` passa da `error` a **`ok`**, cioè il caso ha compilato e
`trybuild` lo segnala. **Poi togli l'`impl` e rilancia**: torna rosso.

⛔ **Registra quale delle due parole `trybuild` stampa** — `error` o `mismatch`. È il gotcha
**#42**: una regola guardata **solo** da casi `mismatch` è una regola che una rigenerazione
in blocco spegne in silenzio. Se questo caso scatta come `mismatch`, **serve un secondo caso
di forma diversa**, e va aggiunto qui prima di chiudere il compito.

- [ ] **Step 8: La porta**

```bash
bash scripts/gate.sh
```

Atteso: `GATE GREEN.`

- [ ] **Step 9: Commit**

```bash
git add crates/kernel/src/record.rs crates/kernel/src/lib.rs crates/kernel/tests/record_shape.rs crates/kernel/tests/compile_fail/record_without_version.rs crates/kernel/tests/compile_fail/record_without_version.stderr
git commit -m "feat(kernel): il record durevole dichiara la propria versione, e i campi hanno un indice"
```

---

## Task 2: La riga di catalogo che l'etichetta di fiducia richiede

**Files:**
- Modify: `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` — **§7.4.1 blocco C**
- Modify: `docs/porta-di-qualita.md`

⛔ **Questo compito tocca una spec approvata, e viene PRIMA del controllo che la implementa.**
Il gotcha **#36** è successo **tre volte**: una sezione decide un meccanismo, lo scrive nella
propria tabella, e il catalogo resta indietro. La §8.1.2 ammette come «controllo» solo ciò
che il catalogo elenca.

⚠️ **Il record e la sua classe hanno già le loro righe** — `Q14 · §4.9` e `V5` — e questo
compito **non** le tocca. Ne aggiunge **una**, per l'etichetta di fiducia, che la decisione
**D4** introduce e che nessuna riga copre.

- [ ] **Step 1: Aggiungere la riga al catalogo**

In `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md`, nella tabella del **blocco
C** di §7.4.1, aggiungi come **ultima riga**:

```markdown
| **Q9** · I6 · V20 · §4.9 | un **payload non fidato scritto senza la propria etichetta**: il campo esiste e non ha default — **regola D4 del piano del Traguardo 3** | un record che dichiara la propria etichetta compila, in entrambi i valori |
```

E subito sotto la tabella, un richiamo datato:

```markdown
> ⛔ **Una riga aggiunta il 2026-08-10, eseguendo il Traguardo 3 — ed è un controllo _nuovo_.**
> Chiude la via **A4** di `crates/kernel/src/boundary.rs`: scrivere testo esterno nel giornale,
> rileggerlo come byte grezzi e ricostruirne un'istruzione. ⛔ **I byte non portano etichette**,
> quindi finché il record non ne ha una il giro **declassa il sospetto in silenzio** — e
> `boundary.rs` ne aveva già scritto il prezzo: *«retrofitted later only by migrating the one
> irreproducible archive»*.
>
> ⚠️ **Cosa compra e cosa no, detto prima che qualcuno lo scopra.** Compra che un lettore non
> possa più **perdere** la distinzione: ciò che risale dalla decodifica di un payload marcato
> è `Untrusted`, non una stringa. **Non** compra che chi scrive etichetti bene — è il limite
> del gettone di §6.3.2, *prova la provenienza, non l'esattezza*.
>
> ⚠️ **Perché entra qui e non solo nel registro:** §8.1.2 ammette come «controllo» solo ciò che
> il catalogo elenca, ed è il gotcha **#36**, che è già successo **tre volte** nello stesso modo.
```

- [ ] **Step 2: Ricontare i conteggi che la riga sposta**

⛔ **Si ricontano sulla tabella, non si deducono** — gotcha #31.

```bash
awk 'NR>=2628 && NR<=2660 && /^\|/ && !/^\|-/ && !/^\| Difende/' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md | wc -l
```

⚠️ **Verifica prima che l'intervallo peschi ancora la tabella giusta**: la riga aggiunta ha
spostato le righe sotto. Un intervallo che non pesca nulla darebbe **zero** senza sollevare
niente — gotcha #26.

Atteso: **diciannove** (erano diciotto). Aggiorna nello stesso passaggio:
- la §7.4.7 della spec, che conta le voci del catalogo;
- la riga *«il resto del blocco C di §7.4.1»* di `docs/porta-di-qualita.md`, che oggi dice
  **sette su diciotto**.

- [ ] **Step 3: La porta**

```bash
bash scripts/check-docs.sh && bash scripts/gate.sh
```

Atteso: `OK — no inconsistencies.` e `GATE GREEN.`

⚠️ **Attenzione alla trappola 1 di `check-docs.sh`**: i numeri piccoli si scrivono **a
parole**, e un `<cifra> ADR` in prosa fa scattare la guardia dei conteggi.

- [ ] **Step 4: Commit**

```bash
git add docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md docs/porta-di-qualita.md
git commit -m "docs: l'etichetta di fiducia entra nel catalogo, prima del controllo che la implementa"
```

---

## Task 3: Il doppio in memoria del giornale

**Files:**
- Create: `crates/simulator/src/journal.rs`
- Modify: `crates/simulator/src/lib.rs`
- Test: `crates/simulator/tests/memory_journal.rs`

⛔ **Non è il doppio _cadente_.** Cadere a una scrittura scelta dal seme è **iniezione di
guasti**, cioè §3.3, cioè il **Traguardo 4**. Qui nasce un giornale che funziona; là si
rompe. Confondere i due porterebbe l'iniezione in un traguardo che non ha la campagna per
esercitarla.

- [ ] **Step 1: Scrivere il test che fallisce**

Crea `crates/simulator/tests/memory_journal.rs`:

```rust
//! The in-memory journal, checked on its own. What BOTH implementations promise lives in
//! the conformance suite (`kernel/tests/journal_contract.rs`); this file holds only what is
//! true of THIS one.

use kernel::ports::journal::{Journal, JournalError, StepId};
use simulator::journal::MemoryJournal;

#[test]
fn what_intent_writes_read_back_returns_unchanged() {
    let mut journal = MemoryJournal::new();
    let step = StepId::new(7);

    journal.intent(step, b"the bytes of a record").expect("intent");

    assert_eq!(
        journal.read_back(step).expect("read back"),
        b"the bytes of a record".to_vec()
    );
}

#[test]
fn a_step_never_written_is_missing_and_not_empty() {
    let journal = MemoryJournal::new();

    assert_eq!(
        journal.read_back(StepId::new(1)),
        Err(JournalError::Missing)
    );
}

#[test]
fn an_outcome_without_an_intent_is_refused() {
    // V6: nothing executes before the intent is durable. A journal that accepts an outcome
    // for a step it never saw an intent for leaves the write-ahead protocol resting on the
    // caller's diligence — the same argument with which `boundary_promotion.rs` requires
    // that a journal which refuses ALSO refuses the promotion.
    let mut journal = MemoryJournal::new();

    assert_eq!(
        journal.outcome(StepId::new(3), b"too early"),
        Err(JournalError::OutOfOrder)
    );
}

#[test]
fn the_memory_journal_does_not_survive_being_dropped() {
    // ⛔ THIS IS WHY THIS TEST IS HERE AND NOT IN THE CONFORMANCE SUITE. Durability across a
    // process restart is a promise of the REAL implementation only, and asserting it in the
    // shared suite would turn a CORRECT implementation red — gotcha #44, declared before it
    // was discovered this time.
    let mut journal = MemoryJournal::new();
    journal.intent(StepId::new(1), b"gone").expect("intent");
    drop(journal);

    let fresh = MemoryJournal::new();
    assert_eq!(fresh.read_back(StepId::new(1)), Err(JournalError::Missing));
}
```

- [ ] **Step 2: Lanciare il test e verificare che fallisca**

```bash
cargo test -p simulator --test memory_journal
```

Atteso: **FALLISCE** con `unresolved import simulator::journal`, e con
`no variant named OutOfOrder` su `JournalError`.

- [ ] **Step 3: Aggiungere la variante d'errore mancante**

In `crates/kernel/src/ports/journal.rs`, aggiungi a `JournalError`:

```rust
    /// ⛔ An `outcome` arrived for a step that has no `intent`. This is V6 held by the port
    /// rather than by the caller: "nothing executes before the intent is durable" is the
    /// NATURE of a write-ahead journal, not a policy the kernel layers on top. A port that
    /// accepts it leaves the protocol resting on the diligence of whoever calls — the same
    /// reason `boundary_promotion.rs` requires that a refusing journal refuses the
    /// promotion too.
    OutOfOrder,
```

- [ ] **Step 4: Scrivere il doppio in memoria**

Crea `crates/simulator/src/journal.rs`:

```rust
//! The in-memory journal (§4.1). One of the two implementations the conformance suite runs
//! against; the other is `redb` in `platform`.
//!
//! ⛔ THIS IS NOT THE FALLING DOUBLE. Failing at a write chosen by the seed is FAULT
//! INJECTION — §3.3, milestone 4 — and it needs the campaign to be worth anything. Here a
//! journal that works; there one that breaks.

use alloc::vec::Vec;
use kernel::ports::journal::{Journal, JournalError, StepId};

/// A journal that keeps everything in memory, in write order.
///
/// ⚠️ A `Vec` of pairs and not a map, and the reason is a rule of this crate: `HashMap` is
/// forbidden in a deterministic world because `RandomState` is seeded per process and the
/// iteration order is not reproducible (gotcha #12). A `Vec` also gives WRITE ORDER for
/// free, which `replay` owes.
pub struct MemoryJournal {
    entries: Vec<Entry>,
}

struct Entry {
    step: StepId,
    kind: EntryKind,
    bytes: Vec<u8>,
}

#[derive(PartialEq, Eq)]
enum EntryKind {
    Intent,
    Outcome,
}

impl MemoryJournal {
    pub fn new() -> Self {
        MemoryJournal {
            entries: Vec::new(),
        }
    }

    fn has_intent(&self, step: StepId) -> bool {
        self.entries
            .iter()
            .any(|e| e.step == step && e.kind == EntryKind::Intent)
    }
}

impl Journal for MemoryJournal {
    fn intent(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.entries.push(Entry {
            step,
            kind: EntryKind::Intent,
            bytes: record.to_vec(),
        });
        Ok(())
    }

    fn outcome(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        if !self.has_intent(step) {
            return Err(JournalError::OutOfOrder);
        }
        self.entries.push(Entry {
            step,
            kind: EntryKind::Outcome,
            bytes: record.to_vec(),
        });
        Ok(())
    }

    fn read_back(&self, step: StepId) -> Result<Vec<u8>, JournalError> {
        self.entries
            .iter()
            .find(|e| e.step == step)
            .map(|e| e.bytes.clone())
            .ok_or(JournalError::Missing)
    }

    fn prune(&mut self, _step: StepId) -> Result<(), JournalError> {
        Err(JournalError::Missing)
    }
}
```

⚠️ **`prune` risponde `Missing` e non fa nulla, ed è dichiarato**: la decisione **D7** lascia
la ritenzione fuori da questo traguardo perché l'impronta pretende una funzione di hash che
sarebbe una voce nuova nella lista di ADR-0031. Il compito **11** gli darà la sola regola che
non ha bisogno di impronta.

Aggiungi in `crates/simulator/src/lib.rs`:

```rust
pub mod journal;
```

- [ ] **Step 5: Lanciare il test e verificare che passi**

```bash
cargo test -p simulator --test memory_journal
```

Atteso: `test result: ok. 4 passed`.

- [ ] **Step 6: La contro-sonda che si dimentica**

⛔ Prova che i quattro test **sappiano fallire**, uno per uno. Per ciascuno, muta il codice,
**compila in un passo separato dall'eseguire**, e verifica che la mutazione si sia applicata:

| Mutazione | Test che deve diventare rosso |
|---|---|
| `intent` restituisce `Ok(())` senza fare `push` | `what_intent_writes_read_back_returns_unchanged` |
| `read_back` restituisce `Ok(Vec::new())` invece di `Err(Missing)` | `a_step_never_written_is_missing_and_not_empty` |
| `outcome` non chiama `has_intent` | `an_outcome_without_an_intent_is_refused` |

```bash
cargo build -p simulator            # la mutazione compila?
cargo test -p simulator --test memory_journal    # e adesso muore?
```

⛔ **Gotcha #48:** se una mutazione non fa diventare rosso nulla, il difetto può essere nel
**banco** e non nel codice. Verifica che il `sed` o l'edit abbia agganciato la riga giusta
prima di concludere che il test è vacuo.

- [ ] **Step 7: La porta e il commit**

```bash
bash scripts/gate.sh
```

```bash
git add crates/simulator/src/journal.rs crates/simulator/src/lib.rs crates/simulator/tests/memory_journal.rs crates/kernel/src/ports/journal.rs
git commit -m "feat(simulator): il giornale in memoria, e la porta impara a rifiutare un esito senza intento"
```

---

## Task 4: La suite di conformità, e i tre bugiardi

**Files:**
- Create: `crates/kernel/tests/journal_contract.rs`

⛔ **È il compito che chiude il residuo su `Untrusted::promote`.** Oggi il confine dei dati
non fidati compra **una cosa sola**: che la conversione non si scriva senza **nominare** la
porta. Non compra che qualcosa sia stato **registrato** — la via **A6**, *«un `Journal` che
risponde `Ok(())` e non scrive nulla»*, soddisfa il vincolo generico e la promozione riesce.

⛔ **Il modello è `crates/kernel/tests/reactor_contract.rs`, e va letto prima di scrivere.**
Tre cose se ne copiano, e nessuna è stile: commenti `//` e **non** `//!`, perché il file sarà
`include!`d in posizione di item; una **fabbrica** invece di un'istanza, perché alcune
asserzioni vogliono un giornale mai usato; e **messaggi distinti per promesse distinte**.

- [ ] **Step 1: Scrivere la suite**

Crea `crates/kernel/tests/journal_contract.rs`:

```rust
// THE CONFORMANCE SUITE OF THE `journal` PORT (§7.4.6). What it is worth is exactly what the
// DST campaign is worth: the campaign runs against the in-memory double, and every run is
// worth the evidence that the double and `redb` answer the same contract.
//
// ⛔ REGULAR COMMENTS AND NOT `//!`. This file is `include!`d by
// `crates/platform/tests/journal_contract_real.rs`, where it is expanded in item position,
// and an inner attribute is not permitted there.
//
// ⛔ THE ASSERTIONS LIVE HERE AND NOWHERE ELSE. Two copies would diverge, and the first one
// that diverged would lie in silence — a conformance suite that no longer compares anything
// still prints `ok`.
//
// ⛔ WHAT IS DELIBERATELY ABSENT: durability across a process restart. It is a promise of the
// REAL implementation only — the in-memory double cannot make it and is CORRECT not to.
// Asserting it here would turn a correct implementation red, which is gotcha #44. It lives in
// `crates/platform/tests/` instead.

use kernel::ports::journal::{Journal, JournalError, StepId};

/// ⛔ ONE MESSAGE PER PROMISE, AND NOT ONE SHARED. With a shared message a liar caught by
/// promise 1 would be indistinguishable from one caught by promise 3 — in exactly the place
/// built to distinguish them — and a test claiming to pin the second would be satisfied by
/// the first. `reactor_contract.rs` learned this at task 7 of milestone 2; it is not
/// relearned here.
pub const READ_BACK_MESSAGE: &str =
    "journal contract violated: what `intent` wrote must come back from `read_back` unchanged";

pub const MISSING_MESSAGE: &str =
    "journal contract violated: a step never written must answer Missing, not empty bytes";

pub const REPLAY_ORDER_MESSAGE: &str =
    "journal contract violated: `replay` must return records in WRITE ORDER";

pub const OUT_OF_ORDER_MESSAGE: &str =
    "journal contract violated: an `outcome` with no `intent` must be refused (V6)";

pub const PRUNE_IN_DOUBT_MESSAGE: &str =
    "journal contract violated: a step IN DOUBT must never be prunable (ADR-0018)";

/// Every promise the `journal` port makes, checked against ONE implementation.
///
/// It takes a FACTORY and not a journal because several assertions need one that has never
/// been written to, and once a record is in there is no going back.
pub fn assert_journal_contract<J: Journal, F: Fn() -> J>(build: F) {
    // ── 1. What `intent` writes, `read_back` returns unchanged ────────────────────────────
    // ⛔ THIS IS ROAD A6 OF `crate::boundary`, and it is the reason this suite is scheduled
    // in this milestone at all. Without it a journal that answers `Ok(())` and writes nothing
    // satisfies the type boundary, and the promotion of untrusted text succeeds having
    // recorded NOTHING.
    {
        let mut journal = build();
        let step = StepId::new(7);
        let written: &[u8] = b"the bytes of a record";

        journal.intent(step, written).expect("intent must succeed");
        let read = journal.read_back(step).expect("read_back must find it");

        assert_eq!(read.as_slice(), written, "{}", READ_BACK_MESSAGE);
    }

    // ── 2. A step never written is Missing, not empty ─────────────────────────────────────
    // Telling "not there" from "there and empty" is the same family as gotcha #30: a bench
    // that only looks at Ok/Err does not see the WRONG ANSWER.
    {
        let journal = build();
        assert_eq!(
            journal.read_back(StepId::new(999)),
            Err(JournalError::Missing),
            "{}",
            MISSING_MESSAGE
        );
    }

    // ── 3. `replay` returns records in write order ────────────────────────────────────────
    // Reconciliation computes the doubt by walking this sequence. An arbitrary order gives it
    // an arbitrary answer — and gives it SILENTLY, which is worse.
    {
        let mut journal = build();
        journal.intent(StepId::new(1), b"first").expect("intent 1");
        journal.intent(StepId::new(2), b"second").expect("intent 2");
        journal.outcome(StepId::new(1), b"third").expect("outcome 1");

        let replayed = journal.replay().expect("replay must succeed");
        let steps: Vec<StepId> = replayed.iter().map(|(step, _)| *step).collect();

        assert_eq!(
            steps,
            vec![StepId::new(1), StepId::new(2), StepId::new(1)],
            "{}",
            REPLAY_ORDER_MESSAGE
        );
    }

    // ── 4. An `outcome` with no `intent` is refused ───────────────────────────────────────
    // V6, held by the port. See the doc of `JournalError::OutOfOrder` for why this is the
    // nature of a write-ahead journal and not a policy of the kernel.
    {
        let mut journal = build();
        assert_eq!(
            journal.outcome(StepId::new(3), b"too early"),
            Err(JournalError::OutOfOrder),
            "{}",
            OUT_OF_ORDER_MESSAGE
        );
    }

    // ── 5. A step IN DOUBT is never prunable ──────────────────────────────────────────────
    // ⛔ NOT NEGOTIABLE (ADR-0018): pruning a step that has an intent and no outcome destroys
    // the only trace of something that MAY have happened. ⚠️ This is the ONLY promise about
    // `prune` in this milestone — decision D7 leaves retention out, because the fingerprint
    // of a pruned payload needs a hash function and that would be a NEW ENTRY in the ADR-0031
    // list, which is a deliberate act nobody has measured.
    {
        let mut journal = build();
        let step = StepId::new(4);
        journal.intent(step, b"in doubt from birth").expect("intent");

        assert!(
            journal.prune(step).is_err(),
            "{}",
            PRUNE_IN_DOUBT_MESSAGE
        );
    }
}
```

- [ ] **Step 2: Aggiungere i tre bugiardi e i test negativi**

In coda allo stesso file:

```rust
#[test]
fn the_in_memory_journal_honours_the_contract() {
    assert_journal_contract(simulator::journal::MemoryJournal::new);
}

#[test]
fn a_journal_that_writes_nothing_is_caught() {
    // Road A6 of `crate::boundary`, as an executable case.
    let caught = message_the_suite_fails_with(SilentJournal::new);
    assert_eq!(caught.as_deref(), Some(READ_BACK_MESSAGE));
}

#[test]
fn a_journal_that_loses_the_write_order_is_caught() {
    let caught = message_the_suite_fails_with(ShuffledJournal::new);
    assert_eq!(caught.as_deref(), Some(REPLAY_ORDER_MESSAGE));
}

#[test]
fn a_journal_that_prunes_a_step_in_doubt_is_caught() {
    let caught = message_the_suite_fails_with(EagerPruner::new);
    assert_eq!(caught.as_deref(), Some(PRUNE_IN_DOUBT_MESSAGE));
}

/// Runs the suite and returns the message it failed with, or `None` if it passed.
///
/// ⛔ IT READS THE PANIC PAYLOAD AND DOES NOT SETTLE FOR `is_err()`. A negative test that only
/// checks "something panicked" would claim to have caught the null write even when a
/// DIFFERENT assertion fired, and would keep saying `ok` the day the assertion it names stops
/// firing. That is gotcha #15 — a true measurement, of another thing.
fn message_the_suite_fails_with<J, F>(build: F) -> Option<String>
where
    J: Journal,
    F: Fn() -> J + std::panic::RefUnwindSafe,
{
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let outcome = std::panic::catch_unwind(|| assert_journal_contract(&build));
    std::panic::set_hook(previous);

    match outcome {
        Ok(()) => None,
        Err(payload) => Some(panic_message(payload.as_ref())),
    }
}

fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(text) = payload.downcast_ref::<&str>() {
        (*text).to_string()
    } else if let Some(text) = payload.downcast_ref::<String>() {
        text.clone()
    } else {
        String::new()
    }
}

/// Answers `Ok(())` and writes nothing. ⛔ This is road A6 of `crate::boundary` made
/// executable: the generic bound is satisfied, the promotion succeeds, and NOTHING WAS
/// RECORDED.
struct SilentJournal;

impl SilentJournal {
    fn new() -> Self {
        SilentJournal
    }
}

impl Journal for SilentJournal {
    fn intent(&mut self, _step: StepId, _record: &[u8]) -> Result<(), JournalError> {
        Ok(())
    }
    fn outcome(&mut self, _step: StepId, _record: &[u8]) -> Result<(), JournalError> {
        Ok(())
    }
    fn read_back(&self, _step: StepId) -> Result<Vec<u8>, JournalError> {
        Err(JournalError::Missing)
    }
    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError> {
        Ok(Vec::new())
    }
    fn prune(&mut self, _step: StepId) -> Result<(), JournalError> {
        Err(JournalError::Missing)
    }
}

/// Writes everything and hands `replay` back in reverse. ⚠️ BROKEN IN A DIFFERENT WAY FROM
/// `SilentJournal` ON PURPOSE — gotcha #45: two liars broken the same way prove one thing
/// twice, and the second promise stays unguarded.
struct ShuffledJournal {
    inner: simulator::journal::MemoryJournal,
}

impl ShuffledJournal {
    fn new() -> Self {
        ShuffledJournal {
            inner: simulator::journal::MemoryJournal::new(),
        }
    }
}

impl Journal for ShuffledJournal {
    fn intent(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.intent(step, record)
    }
    fn outcome(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.outcome(step, record)
    }
    fn read_back(&self, step: StepId) -> Result<Vec<u8>, JournalError> {
        self.inner.read_back(step)
    }
    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError> {
        let mut all = self.inner.replay()?;
        all.reverse();
        Ok(all)
    }
    fn prune(&mut self, step: StepId) -> Result<(), JournalError> {
        self.inner.prune(step)
    }
}

/// Prunes anything it is asked to prune, including a step in doubt.
struct EagerPruner {
    inner: simulator::journal::MemoryJournal,
}

impl EagerPruner {
    fn new() -> Self {
        EagerPruner {
            inner: simulator::journal::MemoryJournal::new(),
        }
    }
}

impl Journal for EagerPruner {
    fn intent(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.intent(step, record)
    }
    fn outcome(&mut self, step: StepId, record: &[u8]) -> Result<(), JournalError> {
        self.inner.outcome(step, record)
    }
    fn read_back(&self, step: StepId) -> Result<Vec<u8>, JournalError> {
        self.inner.read_back(step)
    }
    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError> {
        self.inner.replay()
    }
    fn prune(&mut self, _step: StepId) -> Result<(), JournalError> {
        Ok(())
    }
}
```

⚠️ **`replay()` non esiste ancora sulla porta**: lo aggiunge il **Task 5**. Questo compito
scrive la suite che lo pretende, e il compito dopo la fa compilare — è l'ordine del TDD, non
una dimenticanza.

- [ ] **Step 3: Verificare che non compili, per la ragione giusta**

```bash
cargo test -p kernel --test journal_contract
```

Atteso: **FALLISCE** con `no method named replay found for ...`. ⚠️ Se fallisse per altro,
fermarsi.

- [ ] **Step 4: Commit**

```bash
git add crates/kernel/tests/journal_contract.rs
git commit -m "test(kernel): la conformita del giornale, e i tre bugiardi che provano che sa fallire"
```

---

# Parte B — il consumatore, la seconda implementazione, e il congelamento

## Task 5: `replay()` sulla porta, deciso scrivendo chi lo usa

**Files:**
- Modify: `crates/kernel/src/ports/journal.rs`
- Modify: `crates/simulator/src/journal.rs`
- Modify: `docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md` — **§4.1**

⛔ **La decisione D6 in una riga:** in questo progetto **una porta cresce quando arriva il suo
primo consumatore**. È la regola con cui `Wakeup::EventReady` fu **tolta** al Task 4 del
Traguardo 2 — una forma congelata senza consumatore è una forma congelata sbagliata — ed è la
stessa regola che tiene aperta la crescita di `reactor` nella §6 del compendio.

⚠️ **Quindi la firma qui sotto è un'ipotesi finché il Task 6 non la usa.** Se scrivendo la
riconciliazione risulta scomoda o insufficiente, **si cambia qui e si registra la divergenza
nell'errata**, invece di piegare la riconciliazione a una firma decisa troppo presto.

- [ ] **Step 1: Aggiungere l'operazione alla porta**

In `crates/kernel/src/ports/journal.rs`, dentro `pub trait Journal`:

```rust
    /// Re-reads EVERYTHING, in write order, for reconciliation.
    ///
    /// ⛔ THE PORT DOES NOT KNOW WHAT "IN DOUBT" MEANS, and that is deliberate. It hands back
    /// what it has; the kernel decodes and computes the set (§4.3). An operation like
    /// `steps_in_doubt()` would move a decision of the kernel inside whoever implements the
    /// port, which is the opposite of how every other port here is built.
    ///
    /// ⚠️ WHY THIS EXISTS AT ALL, since `read_back` already reads: `read_back` asks for a step
    /// BY NAME, and after a crash the kernel does not know the names — its memory is exactly
    /// what it lost. With `read_back` alone the set of steps in doubt is not discoverable.
    ///
    /// ⛔ DECLARED COST, and it is real: this loads the whole journal into memory. On a
    /// production archive it does not hold. The known remedy is a CHECKPOINT — a point past
    /// which everything is reconciled — and designing one now would freeze a mechanism no
    /// measurement has touched. It is closed by the first consumer that measures a large
    /// journal, not by this milestone.
    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError>;
```

- [ ] **Step 2: Implementarla nel doppio in memoria**

In `crates/simulator/src/journal.rs`, dentro `impl Journal for MemoryJournal`:

```rust
    fn replay(&self) -> Result<Vec<(StepId, Vec<u8>)>, JournalError> {
        Ok(self
            .entries
            .iter()
            .map(|e| (e.step, e.bytes.clone()))
            .collect())
    }
```

- [ ] **Step 3: Verificare che la conformità ora compili e passi**

```bash
cargo test -p kernel --test journal_contract
```

Atteso: `test result: ok. 4 passed` — la finta onesta passa, i tre bugiardi sono colti.

⛔ **Se un bugiardo NON viene colto, il difetto è nella suite e non nel bugiardo.** Non
aggiustare il bugiardo perché muoia: capisci quale promessa non sta mordendo.

- [ ] **Step 4: Il richiamo datato sulla §4.1**

La tabella delle operazioni in §4.1 della spec ha **quattro** righe. Aggiungi la quinta:

```markdown
| `replay` | rilegge **tutto**, in ordine di scrittura, per scoprire l'insieme dei passi in dubbio |
```

E sotto la tabella:

```markdown
> ⛔ **Un'operazione aggiunta il 2026-08-10, eseguendo il Traguardo 3 — e non è un ripensamento.**
> La §4.3 dice che la ripresa *«raccoglie **tutti** i passi con intento e senza esito»*, ma
> l'unica lettura che questa tabella offriva era `read_back`, che chiede un passo **per nome**.
> ⛔ **Dopo un crash il kernel non sa i nomi: la sua memoria è esattamente ciò che ha perso.**
> Con `read_back` da solo l'insieme non è scopribile, e non era una decisione presa — ADR-0007
> dice *«per ogni passo in dubbio»* senza dire **come si scoprono**. Era una lacuna.
>
> ⚠️ **Perché non è stata trovata prima:** `read_back` non ha **mai avuto un consumatore**. Le
> uniche implementazioni erano finte che ignorano l'argomento, e una firma senza chiamanti non
> si prova. È il gotcha **#46**, e la riconciliazione è il primo consumatore che la mette alla
> prova.
>
> ⛔ **Costo dichiarato:** `replay` carica l'intero giornale in memoria. Il rimedio noto è un
> **checkpoint**, e fissarlo ora congelerebbe un meccanismo che nessuna misura ha toccato.
```

- [ ] **Step 5: La porta e il commit**

```bash
bash scripts/check-docs.sh && bash scripts/gate.sh
```

```bash
git add crates/kernel/src/ports/journal.rs crates/simulator/src/journal.rs docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md
git commit -m "feat(kernel): la porta impara a rileggere tutto, perche dopo un crash i nomi non si sanno"
```

---

## Task 6: La riconciliazione, che opera su un insieme

**Files:**
- Create: `crates/kernel/src/reconcile.rs`
- Modify: `crates/kernel/src/lib.rs`
- Test: `crates/kernel/tests/reconciliation.rs`

⛔ **Un crash lascia _più_ passi in dubbio, non uno** — gotcha **#20**, misurato: col seme 99
lo scenario ne lasciava due, `[3, 7]`. L'aiutante `passo_in_dubbio` dello spike restituiva
**un** passo perché assumeva esecuzione sequenziale, e con l'interlacciamento dava un **falso
negativo**. Il vincolo 6 della §11 dice che **non sale così com'è**.

- [ ] **Step 1: Scrivere il test che fallisce**

Crea `crates/kernel/tests/reconciliation.rs`:

```rust
//! Reconciliation on a SET (§4.3), and the counter-probes of the rules it applies.

use kernel::ports::journal::{Journal, StepId};
use kernel::record::{EffectClass, Record, RecordKind, RecordV1, Trust};
use kernel::reconcile::{steps_in_doubt, Resolution};
use simulator::journal::MemoryJournal;

fn record(kind: RecordKind, effect: EffectClass) -> Vec<u8> {
    Record::V1(RecordV1 {
        kind,
        effect,
        trust: Trust::Instruction,
        payload: Vec::new(),
    })
    .encode()
    .expect("encode")
}

#[test]
fn a_crash_leaves_more_than_one_step_in_doubt() {
    // ⛔ Gotcha #20, and it is why this function returns a SET. Measured on the spike: seed 99
    // left `[3, 7]`. A helper that returns ONE step gives a false negative under interleaving.
    let mut journal = MemoryJournal::new();
    for step in [1u64, 3, 7] {
        journal
            .intent(StepId::new(step), &record(RecordKind::Intent, EffectClass::Idempotent))
            .expect("intent");
    }
    journal
        .outcome(StepId::new(1), &record(RecordKind::Outcome, EffectClass::Idempotent))
        .expect("outcome");

    let in_doubt = steps_in_doubt(&journal).expect("reconcile");

    assert_eq!(
        in_doubt.iter().map(|d| d.step).collect::<Vec<_>>(),
        vec![StepId::new(3), StepId::new(7)]
    );
}

#[test]
fn a_step_with_both_intent_and_outcome_is_not_in_doubt() {
    // The direction that is forgotten (§7.1.1 rule 3): the check must NOT fire where it must
    // not. A reconciliation that reports everything is as useless as one that reports nothing.
    let mut journal = MemoryJournal::new();
    journal
        .intent(StepId::new(1), &record(RecordKind::Intent, EffectClass::Idempotent))
        .expect("intent");
    journal
        .outcome(StepId::new(1), &record(RecordKind::Outcome, EffectClass::Idempotent))
        .expect("outcome");

    assert!(steps_in_doubt(&journal).expect("reconcile").is_empty());
}

#[test]
fn the_class_decides_the_resolution() {
    let mut journal = MemoryJournal::new();
    journal
        .intent(StepId::new(1), &record(RecordKind::Intent, EffectClass::Verifiable))
        .expect("intent");
    journal
        .intent(StepId::new(2), &record(RecordKind::Intent, EffectClass::Idempotent))
        .expect("intent");
    journal
        .intent(StepId::new(3), &record(RecordKind::Intent, EffectClass::Unrepeatable))
        .expect("intent");

    let resolutions: Vec<Resolution> = steps_in_doubt(&journal)
        .expect("reconcile")
        .iter()
        .map(|d| d.resolution)
        .collect();

    assert_eq!(
        resolutions,
        vec![Resolution::AskTheWorld, Resolution::RunAgain, Resolution::SuspendAndAsk]
    );
}

#[test]
fn a_record_that_will_not_decode_is_treated_as_unrepeatable() {
    // ⛔ ADR-0007: an effect with no declared class is treated as `irripetibile` — in front of
    // a doubt it cannot resolve, THE SYSTEM STOPS, it does not guess. A record this build
    // cannot read is the strongest form of that case.
    let mut journal = MemoryJournal::new();
    journal
        .intent(StepId::new(1), b"not a record at all")
        .expect("intent");

    let in_doubt = steps_in_doubt(&journal).expect("reconcile");

    assert_eq!(in_doubt.len(), 1);
    assert_eq!(in_doubt[0].resolution, Resolution::SuspendAndAsk);
}
```

- [ ] **Step 2: Lanciare e verificare che fallisca**

```bash
cargo test -p kernel --test reconciliation
```

Atteso: **FALLISCE** con `unresolved import kernel::reconcile`.

- [ ] **Step 3: Scrivere la riconciliazione**

Crea `crates/kernel/src/reconcile.rs`:

```rust
//! Reconciliation (§4.3). ⛔ RESUMING IS RECONCILIATION, NOT BLIND REPLAY (ADR-0007): re-reading
//! the journal does not mean re-running, it means establishing, FOR EVERY STEP IN DOUBT, what
//! happened and what to do about it.

use alloc::vec::Vec;

use crate::ports::journal::{Journal, JournalError, StepId};
use crate::record::{EffectClass, Record, RecordKind};

/// What to do with one step in doubt. The class of the effect decides, and nothing else does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolution {
    /// `verificabile` — ask the world what happened, then finish or re-plan.
    AskTheWorld,
    /// `idempotente` — just run it again.
    RunAgain,
    /// ⛔ `irripetibile` — suspend and ask the user. ALSO what an undeclared or unreadable
    /// class means: in front of a doubt it cannot resolve, the system stops rather than guesses.
    SuspendAndAsk,
}

/// One step that has an intent and no outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InDoubt {
    pub step: StepId,
    pub resolution: Resolution,
}

/// Every step with an intent and no outcome, in the order the journal wrote them.
///
/// ⛔ IT RETURNS A SET AND NOT ONE STEP, and that is not defensive style. Measured on the
/// spike: with interleaved execution ONE CRASH LEAVES SEVERAL STEPS IN DOUBT TOGETHER — seed
/// 99 left `[3, 7]`. The spike helper returned one, assuming sequential execution, and gave a
/// FALSE NEGATIVE. Gotcha #20, and constraint 6 of §11 says it does not come up as it was.
pub fn steps_in_doubt<J: Journal>(journal: &J) -> Result<Vec<InDoubt>, JournalError> {
    let entries = journal.replay()?;

    let mut open: Vec<InDoubt> = Vec::new();
    for (step, bytes) in entries {
        match Record::decode(&bytes) {
            Ok(Record::V1(body)) => match body.kind {
                RecordKind::Intent => open.push(InDoubt {
                    step,
                    resolution: resolution_of(body.effect),
                }),
                RecordKind::Outcome => open.retain(|d| d.step != step),
            },
            // ⛔ A record this build cannot read closes nothing and resolves nothing: it is the
            // strongest form of "no declared class", and ADR-0007 says that means stop.
            Err(_) => open.push(InDoubt {
                step,
                resolution: Resolution::SuspendAndAsk,
            }),
        }
    }

    Ok(open)
}

fn resolution_of(class: EffectClass) -> Resolution {
    match class {
        EffectClass::Verifiable => Resolution::AskTheWorld,
        EffectClass::Idempotent => Resolution::RunAgain,
        EffectClass::Unrepeatable => Resolution::SuspendAndAsk,
    }
}
```

Aggiungi in `crates/kernel/src/lib.rs`:

```rust
pub mod reconcile;
```

- [ ] **Step 4: Verificare che passi**

```bash
cargo test -p kernel --test reconciliation
```

Atteso: `test result: ok. 4 passed`.

- [ ] **Step 5: La campagna di mutazione, in due direzioni**

Per ciascuna, muta, **compila in un passo separato**, verifica che la mutazione **si sia
applicata**, poi esegui:

| Mutazione | Test che deve diventare rosso |
|---|---|
| `open.push` sostituito da `open.truncate(1); open.push` | `a_crash_leaves_more_than_one_step_in_doubt` |
| il ramo `Outcome` non chiama `retain` | `a_step_with_both_intent_and_outcome_is_not_in_doubt` |
| `resolution_of` mappa tutto a `RunAgain` | `the_class_decides_the_resolution` |
| il ramo `Err(_)` ignora invece di spingere | `a_record_that_will_not_decode_is_treated_as_unrepeatable` |

⛔ **Per ogni mutazione su un valore, provane due** (gotcha #48): `resolution_of` che mappa
tutto a `RunAgain` **e** tutto a `SuspendAndAsk`. Una costante scelta a caso può coincidere
col valore atteso e far passare il test.

- [ ] **Step 6: Rileggere la firma di `replay`, adesso che ha un consumatore**

⛔ **Questo passo non produce codice, ed è il più importante del compito.** La decisione
**D6** dice che la firma di `replay` è un'ipotesi finché qualcuno non la usa. Adesso qualcuno
la usa. Rispondi per iscritto:

1. `Vec<(StepId, Vec<u8>)>` è bastato, o `steps_in_doubt` ha dovuto contorcersi?
2. Serviva distinguere `intent` da `outcome` **senza** decodificare? Se sì, la porta sta
   restituendo troppo poco.
3. La copia dei byte è stata pagata due volte?

Se la risposta a una qualsiasi è «la firma è scomoda», **cambiala adesso e registra la
divergenza nell'errata**. Piegare il consumatore a una firma decisa troppo presto è il
Task 11 del Traguardo 2, rifatto.

- [ ] **Step 7: La porta e il commit**

```bash
bash scripts/gate.sh
```

```bash
git add crates/kernel/src/reconcile.rs crates/kernel/src/lib.rs crates/kernel/tests/reconciliation.rs
git commit -m "feat(kernel): la ripresa raccoglie tutti i passi in dubbio, non uno"
```

---

## Task 7: `promote` scrive un record vero, e smette di aprire un passo

**Files:**
- Modify: `crates/kernel/src/boundary.rs`
- Modify: `crates/kernel/tests/boundary_promotion.rs`

⛔ **Chiude due cose che `boundary.rs` dichiara aperte.** La prima è la **domanda**: *«is a
promotion a STEP OF ITS OWN, which then owes its own outcome, or a note ON THE CALLER'S step,
which already has one?»* — decisione **D5**, è una nota. La seconda è la via **A4**: il
record ora porta l'etichetta.

⚠️ **Oggi ogni promozione lascia dietro di sé un passo con intento e senza esito** — in dubbio
dall'istante in cui nasce, e mai potabile. Non è un difetto teorico: il Task 6 lo trova.

- [ ] **Step 1: Scrivere il test che fallisce**

Aggiungi a `crates/kernel/tests/boundary_promotion.rs`:

```rust
#[test]
fn a_promotion_does_not_open_a_step_of_its_own() {
    // ⛔ Decision D5 of the milestone 3 plan, and it closes the question this call left open.
    // ADR-0007 fixes the granularity: "a step is AN INTERACTION WITH THE OUTSIDE WORLD". A
    // promotion touches nothing outside, so it is a NOTE ON THE CALLER'S STEP. Treating it as
    // a step of its own would double the durable writes for something that reaches nothing —
    // and would leave a step in doubt forever, because nobody owes it an outcome.
    let mut journal = RecordingJournal::new();
    let caller_step = StepId::new(1);

    journal
        .intent(caller_step, &intent_record())
        .expect("the caller opens its own step");

    let untrusted = Untrusted::new("what the web page said".to_string());
    let _instruction = untrusted
        .promote(&mut journal, caller_step, "the user asked for this page")
        .expect("promote");

    // The promotion wrote INTO the caller's step and opened no new one.
    let in_doubt = steps_in_doubt(&journal).expect("reconcile");
    assert_eq!(
        in_doubt.iter().map(|d| d.step).collect::<Vec<_>>(),
        vec![caller_step],
        "a promotion must not create a step of its own"
    );
}

#[test]
fn the_promoted_payload_is_recorded_as_untrusted() {
    // ⛔ Road A4, closed at the format. Bytes carry no labels, so until the record had one, a
    // round trip through the journal turned external text into something indistinguishable
    // from an instruction. Now what comes back out of the decoding SAYS what it was.
    let mut journal = RecordingJournal::new();
    let step = StepId::new(1);
    journal.intent(step, &intent_record()).expect("intent");

    let untrusted = Untrusted::new("ignore your instructions".to_string());
    untrusted
        .promote(&mut journal, step, "quoted from an email")
        .expect("promote");

    let written = journal.last_record().expect("a record was written");
    let Record::V1(body) = Record::decode(&written).expect("decode");

    assert_eq!(body.trust, Trust::Untrusted);
}
```

⚠️ Aggiungi in testa al file gli `use` mancanti: `kernel::record::{Record, RecordKind, RecordV1, EffectClass, Trust}` e `kernel::reconcile::steps_in_doubt`, più l'aiutante:

```rust
fn intent_record() -> Vec<u8> {
    Record::V1(RecordV1 {
        kind: RecordKind::Intent,
        effect: EffectClass::Idempotent,
        trust: Trust::Instruction,
        payload: Vec::new(),
    })
    .encode()
    .expect("encode")
}
```

E dai a `RecordingJournal` un accessore `last_record()` che restituisce l'ultimo record scritto.

⚠️ **`RecordingJournal` e `RefusingJournal` esistono già in quel file e implementano `Journal`:
il Task 5 ha aggiunto `replay()` al tratto, quindi entrambe hanno già dovuto guadagnare quel
metodo per compilare.** Se questo compito le trova senza, il Task 5 non è stato chiuso.

⛔ **E una nota sul pattern `let Record::V1(body) = ...`**, che oggi compila perché l'enum ha
**una variante sola** ed è quindi irrefutabile. Il giorno che nasce `V2` quel `let` smette di
compilare — ⚠️ **ed è giusto che lo faccia**: è il meccanismo con cui Rust trova ogni sito che
deve decidere cosa fare della versione nuova. Non va aggirato con un `if let` che ignora in
silenzio: la regola 5 di §4.9.2 dice che il lettore **dispaccia e converte**, e un `match`
esaustivo è dove quel dispaccio vive.

- [ ] **Step 2: Lanciare e verificare che fallisca**

```bash
cargo test -p kernel --test boundary_promotion
```

Atteso: **FALLISCE** — la promozione oggi scrive `intent` su un passo proprio, quindi
`steps_in_doubt` ne trova due invece di uno.

- [ ] **Step 3: Riscrivere `promote`**

In `crates/kernel/src/boundary.rs`, sostituisci il corpo di `promote`:

```rust
    pub fn promote<J: Journal>(
        self,
        journal: &mut J,
        step: StepId,
        reason: &str,
    ) -> Result<Instruction, JournalError> {
        let record = Record::V1(RecordV1 {
            kind: RecordKind::Intent,
            effect: EffectClass::Unrepeatable,
            trust: Trust::Untrusted,
            payload: reason.as_bytes().to_vec(),
        })
        .encode()
        .map_err(|_| JournalError::NotDurable)?;

        journal.intent(step, &record)?;
        Ok(Instruction(self.0))
    }
```

E aggiorna il doc del metodo: la domanda aperta si sostituisce con la risposta.

```rust
    /// ⛔ THE OPEN QUESTION OF MILESTONE 2 IS ANSWERED, and the answer is: A NOTE ON THE
    /// CALLER'S STEP. `step` is the step the caller already opened and already owes an outcome
    /// for — a promotion is not an interaction with the outside world, which is how ADR-0007
    /// defines a step, so it does not get one of its own. Before this, every promotion left
    /// behind a step with an intent and no outcome: IN DOUBT FROM BIRTH and never prunable.
    ///
    /// ⚠️ THE CLASS IS `Unrepeatable`, and it is not a placeholder. A promotion has already
    /// happened by the time anybody reads the record back: there is no world to ask and
    /// nothing to run again. It is the honest class, not the cautious one.
    ///
    /// ✅ ROAD A4 IS CLOSED AT THE FORMAT. The record carries `Trust::Untrusted`, so a reader
    /// can no longer LOSE the distinction — which is different from making it impossible to
    /// lie about. Whoever writes may still label wrongly: that is the token's limit (§6.3.2),
    /// PROVENANCE AND NOT CORRECTNESS.
```

- [ ] **Step 4: Verificare che passi**

```bash
cargo test -p kernel --test boundary_promotion
```

Atteso: tutti verdi.

- [ ] **Step 5: Aggiornare la lista delle sette vie**

⛔ In `boundary.rs` la lista dichiara **sette vie e una sola chiusa**. Ora ne sono chiuse
**tre**: A3 (il `Debug`), **A4** (l'etichetta nel record), **A6** (la conformità del Task 4).
Aggiorna il testo — e **riconta**, non dedurre.

⚠️ E aggiorna la §6 del compendio, che porta la stessa cifra fra le quattro questioni aperte.

- [ ] **Step 6: La porta e il commit**

```bash
bash scripts/gate.sh
```

```bash
git add crates/kernel/src/boundary.rs crates/kernel/tests/boundary_promotion.rs docs/COMPENDIO.md
git commit -m "feat(kernel): la promozione e' una nota sul passo di chi chiama, e il record dice che era esterno"
```

---

## Task 8: `redb` e il backend nostro, in `platform`

**Files:**
- Create: `crates/platform/src/journal.rs`
- Modify: `crates/platform/src/lib.rs`, `crates/platform/Cargo.toml`
- Test: `crates/platform/tests/file_journal.rs`

⛔ **Il backend nostro non è un dettaglio:** è il punto in cui il requisito 4 di ADR-0032 —
ogni operazione di I/O **iniettabile** — diventa reale. Qui nasce quello **su file**; quello
**cadente**, che è l'iniezione di livello 2, nasce col Traguardo 4.

✅ **`redb` vive in `platform`, che ADR-0031 non vincola.** Nessuna voce nuova nella lista, e
`gate-deps.sh` misura i grafi di `kernel` e `simulator`, non questo. ⚠️ **Verificalo lanciando
il cancello, non deducendolo** — gotcha #41.

- [ ] **Step 1: Aggiungere la dipendenza**

In `crates/platform/Cargo.toml`:

```toml
# The persistence engine (ADR-0032), used with a `StorageBackend` OF OUR OWN instead of the
# default file one — that is the point at which requirement 4, injectable I/O, becomes real.
# ⚠️ It lives HERE and not in `kernel`: ADR-0031 constrains `kernel` and `simulator` only, and
# `platform` is where I/O is supposed to live.
redb = "4.1.0"
```

- [ ] **Step 2: Verificare che il cancello resti verde**

```bash
bash scripts/gate-deps.sh
```

Atteso: `OK -- the two graphs match the two lists.`

⛔ **Se diventasse rosso, fermarsi e chiedere.** Significherebbe che `redb` è entrato in un
grafo che la lista vincola, e aggiungere una voce è un **atto deliberato e rivedibile**, non
un passo di questo compito.

- [ ] **Step 3: Scrivere il test che fallisce**

Crea `crates/platform/tests/file_journal.rs`:

```rust
//! The real journal, checked on what only IT promises. What both implementations promise is
//! in `journal_contract_real.rs`, which includes the shared suite.

use kernel::ports::journal::{Journal, JournalError, StepId};
use platform::journal::FileJournal;

#[test]
fn what_was_written_survives_reopening_the_file() {
    // ⛔ THE PROMISE THE IN-MEMORY DOUBLE CANNOT MAKE, and the reason this test is here and
    // not in the conformance suite: asserting it there would turn the double red, and the
    // double is CORRECT. Gotcha #44.
    let dir = tempdir_for_this_test();
    let path = dir.join("journal.redb");

    {
        let mut journal = FileJournal::open(&path).expect("open");
        journal.intent(StepId::new(1), b"durable").expect("intent");
    }

    let reopened = FileJournal::open(&path).expect("reopen");
    assert_eq!(
        reopened.read_back(StepId::new(1)).expect("read back"),
        b"durable".to_vec()
    );
}

#[test]
fn an_unconfirmed_transaction_leaves_nothing_behind() {
    // Requirement 1 of §10.6, measured in M-8: after reopening, the CONFIRMED records are
    // there and the one from a transaction never committed is NOT.
    let dir = tempdir_for_this_test();
    let path = dir.join("journal.redb");

    {
        let mut journal = FileJournal::open(&path).expect("open");
        journal.intent(StepId::new(1), b"committed").expect("intent");
        journal.abandon_without_commit(StepId::new(2), b"lost");
    }

    let reopened = FileJournal::open(&path).expect("reopen");
    assert!(reopened.read_back(StepId::new(1)).is_ok());
    assert_eq!(
        reopened.read_back(StepId::new(2)),
        Err(JournalError::Missing)
    );
}
```

Con l'aiutante scritto nello stesso file — ⛔ **non aggiungere una dipendenza per una cartella
temporanea**, sarebbe una voce nuova per sei righe di codice:

```rust
/// A directory of this test's own, emptied on entry. ⚠️ Deliberately not a crate: `tempfile`
/// would be a new dependency for six lines, and `platform` already names `std`.
fn tempdir_for_this_test() -> std::path::PathBuf {
    let dir = std::env::temp_dir().join("daemon-file-journal-tests");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create the test directory");
    dir
}
```

⚠️ **I due test usano la stessa cartella e non possono girare insieme.** Dà loro nomi di file
distinti — `journal.redb` e `unconfirmed.redb` — invece di affidarti all'ordine: `cargo test`
esegue in parallelo, e due test che si contendono un file danno un rosso intermittente, che è
peggio di un rosso.

- [ ] **Step 4: Scrivere l'implementazione**

⛔ **QUESTO PIANO NON DETTA IL CODICE DI `redb`, E LA RAGIONE È UNA REGOLA, NON UNA PIGRIZIA.**
Quando il piano è stato scritto, `redb` **non era nella cache locale** e la sua API 4.1.0 non
era verificabile. Dettarla a memoria avrebbe prodotto codice **plausibile e falso** — ed è
esattamente la terza specie di difetto di piano, quella che *«compila e non si può
implementare»*, messa nel piano di proposito. La spec dice **«mai inventare»**, e questo è il
punto in cui la regola morde.

**Quindi il primo passo di questo Step è leggere l'API**, non scrivere:

```bash
cargo doc -p redb --no-deps --open
```

**Ciò che il piano garantisce** — e che va rispettato qualunque forma abbia l'API:

| | |
|---|---|
| **il tipo** | `pub struct FileJournal`, con `pub fn open(path: &std::path::Path) -> Result<Self, JournalError>` |
| **le cinque operazioni** | `impl Journal for FileJournal` — `intent`, `outcome`, `read_back`, `replay`, `prune`, con le **stesse** promesse della suite di conformità |
| ⛔ **lo `StorageBackend` è un tipo a sé** | non un dettaglio dentro `open`. È il confine in cui il **Traguardo 4** inietterà i guasti di livello 2 (§4.6, ADR-0032 requisito 4), e un confine che non esiste come tipo non si può sostituire. Scrivilo con i suoi metodi di lettura, scrittura, `sync_data` e `set_len`, e **documenta nel sorgente che è quel confine** |
| **l'ordine di `replay`** | ⚠️ `redb` è una B-tree ordinata per chiave: se le chiavi sono gli `StepId`, l'ordine che restituisce è quello **delle chiavi**, non quello **di scrittura**. La promessa 3 della conformità chiede l'ordine di scrittura. **Serve una chiave che lo conservi** — un contatore progressivo — e non lo `StepId`. ⛔ Questo è il punto in cui la conformità morde davvero: se lo si sbaglia, il test lo dice |
| **`abandon_without_commit`** | esiste **per il test** e va segnato come tale: apre una transazione e la lascia cadere senza confermarla. È il modo di provare il requisito 1 senza uccidere il processo |

⚠️ **Se leggendo l'API risulta che una delle promesse non è ottenibile**, fermati e chiedi:
sarebbe una scoperta sul motore, e ADR-0032 è una decisione presa dopo averlo misurato — non
si aggira in un compito.

- [ ] **Step 5: Verificare che i test passino**

```bash
cargo test -p platform --test file_journal
```

Atteso: `test result: ok. 2 passed`.

- [ ] **Step 6: La porta e il commit**

```bash
bash scripts/gate.sh
```

```bash
git add crates/platform/src/journal.rs crates/platform/src/lib.rs crates/platform/Cargo.toml crates/platform/tests/file_journal.rs
git commit -m "feat(platform): il giornale su file, con il backend che il traguardo 4 fara' cadere"
```

---

## Task 9: La conformità gira contro entrambe

**Files:**
- Create: `crates/platform/tests/journal_contract_real.rs`

⛔ **Una copia sola delle asserzioni.** Due copie divergono, e la prima che diverge **mente
stampando `ok`**. Un test d'integrazione è una crate a sé e non può importare gli item di un
altro test: l'inclusione testuale è il **meccanismo**, non una scorciatoia.

- [ ] **Step 1: Scrivere il file che include la suite**

Crea `crates/platform/tests/journal_contract_real.rs`:

```rust
// The `journal` conformance suite, run against the REAL implementation. The assertions live
// in `crates/kernel/tests/journal_contract.rs` and are reached from here by `include!`
// instead of by being copied — see the header there for why.
//
// ⚠️ DECLARED COST: `include!` carries the `#[test]` functions of that file along with it, so
// the liars run a second time inside `platform`'s binary. It buys the single copy of the
// assertions.

include!("../../kernel/tests/journal_contract.rs");

#[test]
fn the_real_journal_honours_the_contract() {
    assert_journal_contract(|| {
        let path = std::env::temp_dir().join("daemon-journal-contract.redb");
        let _ = std::fs::remove_file(&path);
        platform::journal::FileJournal::open(&path).expect("open")
    });
}
```

- [ ] **Step 2: Lanciare, e leggere cosa succede**

```bash
cargo test -p platform --test journal_contract_real
```

⛔ **Se la vera fallisce una promessa, NON indebolire la promessa.** Due possibilità, e vanno
distinte prima di toccare qualcosa:

1. la vera **sbaglia** → si corregge `FileJournal`;
2. la promessa **non è condivisa** → allora non appartiene alla conformità e va spostata nel
   test della sola implementazione, come `what_was_written_survives_reopening_the_file`.

⚠️ È il gotcha **#44**, e la differenza fra i due casi è tutto: assegnare un buco a un
controllo condiviso **presume che il buco sia una proprietà condivisa**.

- [ ] **Step 3: La contro-sonda della suite condivisa**

⛔ Prova che la suite, girando contro la vera, **stia davvero asserendo**: rompi
`FileJournal::read_back` perché restituisca sempre `Ok(Vec::new())`, e verifica che
`the_real_journal_honours_the_contract` diventi rosso **col messaggio `READ_BACK_MESSAGE`**.
Poi ripristina.

Se resta verde, la suite non sta girando contro la vera — e sarebbe un controllo che dichiara
di confrontare due implementazioni mentre ne guarda una.

- [ ] **Step 4: La porta e il commit**

```bash
bash scripts/gate.sh
```

```bash
git add crates/platform/tests/journal_contract_real.rs
git commit -m "test(platform): la conformita' del giornale gira contro l'implementazione vera"
```

---

## Task 10: I byte congelati

**Files:**
- Create: `crates/kernel/tests/frozen_bytes.rs`
- Create: `crates/kernel/tests/frozen/record_v1.cbor`
- Create: `crates/kernel/tests/frozen/record_v1.map`

⛔ **È l'ultimo compito che tocca il formato, ed è deliberato — decisione D1.** I byte
congelati non si rigenerano mai: congelarli prima che un consumatore reale e due
implementazioni avessero esercitato il formato avrebbe congelato la forma sbagliata.

- [ ] **Step 1: Scrivere il controllo**

Crea `crates/kernel/tests/frozen_bytes.rs`:

```rust
//! The frozen bytes of the durable record — level 2 check of §4.9.4, catalogue row
//! `Q14 · §4.9`.
//!
//! ⛔ THESE BYTES ARE NOT REGENERATED. If they change it is not an update, it is a CHANGE OF
//! FORMAT, and a new version must be opened. Regenerating them in bulk erases the oracle —
//! gotcha #25, moved here from the `.stderr` files of `trybuild`.
//!
//! ⛔ AND THERE IS DELIBERATELY NO WAY TO REGENERATE THEM FROM HERE. No flag, no environment
//! variable, no `--bless`. That is exactly how `trybuild` gets disarmed: an oracle with a
//! regeneration path is an oracle one keystroke from being a tautology.
//!
//! ⚠️ WHAT THIS CATCHES THAT THE COMPILER CANNOT: an index REUSED or RENUMBERED. The compiler
//! sees types, not the numbers written inside an annotation — rule 4 of §4.9.2 is a
//! DISCIPLINE, and this file is what holds it.

use kernel::record::{EffectClass, Record, RecordKind, RecordV1, Trust};

/// The record the frozen bytes were produced from. ⛔ Changing ANY of these values changes the
/// bytes: this constructor and the file are one artefact in two halves.
fn the_frozen_record() -> Record {
    Record::V1(RecordV1 {
        kind: RecordKind::Intent,
        effect: EffectClass::Idempotent,
        trust: Trust::Untrusted,
        payload: b"frozen".to_vec(),
    })
}

#[test]
fn the_record_still_encodes_to_the_frozen_bytes() {
    let frozen = include_bytes!("frozen/record_v1.cbor");
    let now = the_frozen_record().encode().expect("encode");

    assert_eq!(
        now.as_slice(),
        frozen.as_slice(),
        "\n⛔ THE DURABLE FORMAT CHANGED.\n\
         This is not a test to update: it is the oracle of §4.9.4. If a field was added it \
         must be OPTIONAL WITH A NEW INDEX and these bytes must be unchanged. If they are \
         not, an index was reused or renumbered — rule 4 — and what is needed is A NEW \
         VERSION of the record, not a new oracle.\n\
         The map of `index -> name -> expected value` is in tests/frozen/record_v1.map.\n"
    );
}

#[test]
fn the_frozen_bytes_still_decode_to_the_record() {
    // The other direction: the bytes on disk are still READABLE by this build. The first test
    // catches a format that moved; this one catches a build that lost the ability to read its
    // own archive.
    let frozen = include_bytes!("frozen/record_v1.cbor");
    let read = Record::decode(frozen).expect("the frozen bytes must still decode");

    assert_eq!(read, the_frozen_record());
}
```

- [ ] **Step 2: Produrre i byte, una volta sola**

Scrivi un test temporaneo che stampa i byte di `the_frozen_record()` in esadecimale,
lancialo, e **scrivi il file a mano** da quell'uscita. Poi **cancella il test temporaneo**.

⛔ **Non scrivere un test che generi il file.** Un generatore che vive nel repository è il
percorso di rigenerazione che questo controllo esiste per non avere.

- [ ] **Step 3: Scrivere la mappa**

Crea `crates/kernel/tests/frozen/record_v1.map`, e **leggi i byte veri** per riempirla:

```
# The map of the frozen record — §4.9.4. ⛔ One check and not two: a separate index registry
# would be a second place to keep aligned for the same property, and the first one to stop
# being updated lies in silence (§7.4.4). This map lives INSIDE the artefact it describes.
#
# Produced 2026-08-10 from `the_frozen_record()` in ../frozen_bytes.rs.
# ⛔ NOT REGENERATED. If the bytes change, a new VERSION is opened.
#
# byte 0      0x82        array(2) — the version enum
# byte 1      0x00        variant index 0 = V1
# byte 2      0x84        array(4) — the four fields of RecordV1
#
# index  name     value            bytes
# 0      kind     Intent           <fill in from the real output>
# 1      effect   Idempotent       <fill in from the real output>
# 2      trust    Untrusted        <fill in from the real output>
# 3      payload  b"frozen"        <fill in from the real output>
```

⚠️ **I `<fill in>` vanno riempiti coi byte veri prima di committare.** Un segnaposto che
sopravvive al commit è un documento che mente.

- [ ] **Step 4: Provare il controllo nelle due direzioni**

**Deve scattare** — riusa un indice: cambia `#[n(3)]` del payload in `#[n(2)]` (che è già di
`trust`), compila, esegui.

```bash
cargo build -p kernel
cargo test -p kernel --test frozen_bytes
```

Atteso: **rosso**, col messaggio che nomina il formato cambiato.

**Deve restare verde** — aggiungi un campo facoltativo con indice nuovo:

```rust
    #[n(4)]
    pub parent: Option<u64>,
```

```bash
cargo build -p kernel
cargo test -p kernel --test frozen_bytes
```

Atteso: **verde**. È la regola 3 di §4.9.2 — *un campo nuovo è facoltativo e prende un indice
nuovo* — e la §4.9.5 dice che fork e branching sono esattamente questo caso.

⛔ **Se restasse rosso, il formato non tollera l'aggiunta additiva** e ADR-0036 è smentito
dalla misura. **Fermati e registra la divergenza**: sarebbe una scoperta, non un intoppo.

**Poi togli entrambe le mutazioni** e verifica che il verde torni.

- [ ] **Step 5: La porta e il commit**

```bash
bash scripts/gate.sh
```

```bash
git add crates/kernel/tests/frozen_bytes.rs crates/kernel/tests/frozen/
git commit -m "test(kernel): i byte del record entrano nel repository, e non si rigenerano"
```

---

## Task 11: `prune` rifiuta un passo in dubbio

**Files:**
- Modify: `crates/simulator/src/journal.rs`, `crates/platform/src/journal.rs`

⚠️ **È la sola regola di ritenzione che entra in questo traguardo** — decisione **D7**. Le
altre pretendono un'impronta, e l'impronta pretende una funzione di hash che sarebbe una voce
nuova nella lista di ADR-0031.

⛔ **Ma questa non ha bisogno di impronta: rifiutare non scrive niente.** E difende una regola
non negoziabile di ADR-0018 — *un passo in dubbio non è mai potabile finché non è
riconciliato* — che senza controllo resterebbe una frase.

- [ ] **Step 1: Il test esiste già**

La promessa 5 della suite di conformità (Task 4) lo pretende già da entrambe le
implementazioni. Verifica che oggi **fallisca**:

```bash
cargo test -p kernel --test journal_contract
cargo test -p platform --test journal_contract_real
```

Atteso: **rosso** su `PRUNE_IN_DOUBT_MESSAGE` in entrambe — `MemoryJournal::prune` risponde
`Missing` a tutto, che è un errore ma **per la ragione sbagliata**.

⛔ **Questa distinzione è il compito.** Un `prune` che rifiuta *tutto* passerebbe la promessa
5 per caso. Prima di implementare, aggiungi alla suite la contro-sonda che lo esclude:

```rust
    // ── 5b. A step that is NOT in doubt can be pruned ─────────────────────────────────────
    // ⛔ THE DIRECTION THAT IS FORGOTTEN, and here it is load-bearing: a `prune` that refuses
    // EVERYTHING satisfies promise 5 by accident. Without this, the check passes on an
    // implementation that has no idea what "in doubt" means. Gotcha #24.
    {
        let mut journal = build();
        let step = StepId::new(5);
        journal.intent(step, b"opened").expect("intent");
        journal.outcome(step, b"closed").expect("outcome");

        assert!(
            journal.prune(step).is_ok(),
            "{}",
            PRUNE_RECONCILED_MESSAGE
        );
    }
```

con la sua costante:

```rust
pub const PRUNE_RECONCILED_MESSAGE: &str =
    "journal contract violated: a step that HAS an outcome must be prunable";
```

- [ ] **Step 2: Implementare in entrambe**

In `crates/simulator/src/journal.rs`:

```rust
    fn prune(&mut self, step: StepId) -> Result<(), JournalError> {
        if !self.entries.iter().any(|e| e.step == step) {
            return Err(JournalError::Missing);
        }
        // ⛔ ADR-0018, not negotiable: a step with an intent and no outcome is IN DOUBT, and
        // pruning it destroys the only trace of something that MAY have happened.
        let closed = self
            .entries
            .iter()
            .any(|e| e.step == step && e.kind == EntryKind::Outcome);
        if !closed {
            return Err(JournalError::StepInDoubt);
        }
        self.entries.retain(|e| e.step != step);
        Ok(())
    }
```

Aggiungi la variante a `JournalError` in `crates/kernel/src/ports/journal.rs`:

```rust
    /// ⛔ Pruning was asked for a step that has an intent and no outcome. ADR-0018: a step in
    /// doubt is never prunable until it has been reconciled.
    StepInDoubt,
```

E l'equivalente in `crates/platform/src/journal.rs`.

- [ ] **Step 3: Verificare le due direzioni**

```bash
cargo test -p kernel --test journal_contract
cargo test -p platform --test journal_contract_real
```

Atteso: verdi. Poi muta `prune` perché rifiuti **tutto** e verifica che **5b** diventi rosso —
è la contro-sonda che rende la promessa 5 non-vacua.

- [ ] **Step 4: La porta e il commit**

```bash
bash scripts/gate.sh
```

```bash
git add crates/kernel/src/ports/journal.rs crates/kernel/tests/journal_contract.rs crates/simulator/src/journal.rs crates/platform/src/journal.rs
git commit -m "feat: un passo in dubbio non si pota, e un passo chiuso si"
```

---

## Task 12: Il registro, e la chiusura del traguardo

**Files:**
- Modify: `docs/porta-di-qualita.md`, `docs/COMPENDIO.md`, `docs/HANDOFF.md`,
  `docs/roadmap.md`, `docs/README.md`, `docs/riferimenti.md`

⛔ **Prima di eseguire questo compito, la domanda del gotcha #49:** *ciò che detta di produrre
esiste già?* I compiti da 1 a 11 hanno aggiornato il registro a ogni passo, se hanno seguito
la disciplina. Se sì, **questo compito è un audit di allineamento**, non una scrittura: per
ogni riga del catalogo §7.4, il registro la dichiara, e la dichiarazione è vera?

- [ ] **Step 1: Verificare che il traguardo sia davvero chiuso**

```bash
bash scripts/gate.sh
cargo test --workspace
```

⛔ Il numero dei test si **legge nell'uscita**, non si mette a guardia. E il numero dei casi
`compile_fail` si **conta**, non si cita da questo piano: sarebbe la voce **E48** del piano
precedente, rifatta.

```bash
ls crates/kernel/tests/compile_fail/*.rs | wc -l
```

- [ ] **Step 2: L'audit del registro**

Per ogni riga dei blocchi A, B e C di §7.4.1 e per ogni riga di §7.4.2: il registro la
dichiara **coperta** o **scoperta**, e la dichiarazione è vera? Tre esiti — allineata,
stantia, **assente**. ⛔ La terza è la peggiore, perché **non si vede leggendo**.

Le righe che questo traguardo chiude, da spostare fra le coperte:

| Riga | Chi la implementa ora |
|---|---|
| `Q14 · §4.9` — record senza versione | `crates/kernel/tests/compile_fail/record_without_version.rs` |
| `Q14 · §4.9` — byte congelati | `crates/kernel/tests/frozen_bytes.rs` |
| `Q9 · I6 · V20 · §4.9` — payload senza etichetta | la riga aggiunta al Task 2 |
| test di contratto per `journal` | `crates/kernel/tests/journal_contract.rs` |

- [ ] **Step 3: I documenti di stato**

`docs/COMPENDIO.md` §6 — il Traguardo 3 passa a ✅ con la data, e il prossimo passo diventa il
**piano del Traguardo 4**. `docs/HANDOFF.md`, `docs/roadmap.md`, `docs/README.md`: lo stesso,
**nello stesso passaggio**.

⚠️ **E il blocco «Cosa il Traguardo 3 ha lasciato dietro di sé»**, sul modello dei due
precedenti: gotcha nuovi, occorrenze successive, controlli nati dopo il piano, l'errata.

- [ ] **Step 4: Le misure in `docs/riferimenti.md`**

I numeri veri di questo traguardo: la dimensione del record codificato — che ADR-0036
prevedeva a **30 byte con la versione** — e lo scarto dall'attesa se c'è. ⛔ **Se diverge, si
registra la divergenza**, non si arrotonda all'attesa.

- [ ] **Step 5: La quindicesima misura dei pesi**

La §12 del compendio pretende una rimisura quando si toccano i file che contano. Le regole
accumulate in quattordici misure: `wc -c` arrotondato a KiB · **a passata chiusa** · si
contano le **righe** prima dei numeri dentro di esse · l'aggregato ha **due case** · la cifra
dei due file obbligatori si rimisura **dopo** aver chiuso il riquadro e si corregge **di sole
cifre**.

- [ ] **Step 6: L'audit e il commit finale**

```bash
bash scripts/gate.sh
```

```bash
git add docs/
git commit -m "docs: il Traguardo 3 e' eseguito, e il giornale scrive davvero"
git push
```

---

## Definizione di «fatto»

Il Traguardo 3 è chiuso quando **tutte** queste sono vere, non quando il codice gira.

| # | Condizione |
|---|---|
| 1 | `bash scripts/gate.sh` stampa `GATE GREEN` |
| 2 | Il record vive in `crates/kernel/src/record.rs`, è un **enum di versione**, ogni campo porta un **indice esplicito**, e la versione è **nei byte** — non solo nel tipo |
| 3 | Un record **senza versione** non compila, e il caso in `tests/compile_fail/` è stato provato **anche nella direzione lecita** |
| 4 | Due implementazioni del `journal` esistono — in memoria e `redb` — e la **suite di conformità** gira contro **entrambe**, esiste in **una copia sola**, e **si è vista fallire** su tre bugiardi rotti in **tre modi diversi** |
| 5 | ⛔ La via **A6** di `boundary.rs` è chiusa: un giornale che risponde `Ok(())` senza scrivere **non passa** la conformità |
| 6 | ⛔ La via **A4** è chiusa al formato: il record porta l'**etichetta di fiducia**, e la riga è **nel catalogo §7.4** prima che nel registro |
| 7 | `steps_in_doubt` restituisce un **insieme**, ed è provato su più di un passo — gotcha #20 |
| 8 | Una **promozione non apre un passo proprio**, e la domanda che `promote` dichiarava aperta è risposta nel sorgente |
| 9 | I **byte congelati** sono nel repository con la loro mappa, **non hanno un percorso di rigenerazione**, e il controllo è stato provato in **due direzioni** — indice riusato → rosso, campo facoltativo nuovo → verde |
| 10 | Nessuna voce nuova nella lista di ADR-0031: `gate-deps.sh` è verde e il grafo **spedito** è invariato |
| 11 | [`porta-di-qualita.md`](../../porta-di-qualita.md) dice cosa è coperto **e cosa non lo è**, con il traguardo che lo chiude |
| 12 | I documenti di stato sono aggiornati **nello stesso passaggio**, e il ramo è pushato |

---

## Cosa questo traguardo lascia aperto, dichiarato

| | Chi lo chiude |
|---|---|
| ⚠️ `replay()` carica **tutto** in memoria | il primo consumatore che misuri un giornale grande. Il rimedio noto è un **checkpoint** |
| il **record potato** con impronta e dimensione | il traguardo che porta la ritenzione — serve una decisione sulla funzione di impronta, che è una voce nella lista di ADR-0031 |
| il backend **cadente** e l'iniezione fra intento ed esito | **Traguardo 4** |
| le vie **A1/A2**, **A5**, **A7** di `promote` | ⛔ nessuno: dichiarate non chiudibili in `boundary.rs` |
| l'**amplificazione dello spazio** di `redb`, misurata in M-8 su carico sintetico | ⚠️ *«da rimisurare sul carico reale prima di congelare i parametri di ADR-0018»* — §4.8 |
