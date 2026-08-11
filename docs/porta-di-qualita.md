# La porta di qualità — dove vive ogni controllo

> Questo file non decide niente. Il catalogo è la
> [§7.4 della spec](superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md); qui c'è
> soltanto la mappa fra ogni riga del catalogo e il file che la implementa.
>
> ⛔ **Un controllo nuovo entra prima nel catalogo, poi qui.** L'ordine inverso è il
> gotcha #36: una sezione decide un meccanismo, lo scrive nella propria tabella, e il
> catalogo resta indietro — è già successo due volte.

**Un comando solo:** `bash scripts/gate.sh`

| # | Cosa lancia | |
|---|---|---|
| 1 | `cargo build --workspace` | rende **visibile** il livello 1 |
| 2 | `cargo test --workspace` | banco `compile_fail`, contro-sonde, round-trip delle voci spedite |
| 3 | `bash scripts/gate-no-os.sh` | livello 2 |
| 4 | `bash scripts/gate-deps.sh` | livello 2 |
| 5 | `bash scripts/gate-attributes.sh` | livello 2 |
| 6 | `bash scripts/check-docs.sh` | livello 2 |

⚠️ **I primi due non «sono» il livello 1.** Il livello 1 *è* il compilatore e vale a ogni
compilazione, senza che nessuno lo lanci; `gate.sh` compila perché una porta che non
compila non prova niente. La stessa distinzione è scritta in testa a `scripts/gate.sh`.

La CI lancia lo stesso identico comando: `.github/workflows/quality-gate.yml` — `name: quality
gate`, job `gate`. ⚠️ **Rinominato il 2026-08-09, ed era l'ultimo residuo italiano nel codice:**
il workflow non era **mai stato eseguito** — committato lo stesso giorno, ramo non ancora
pushato — quindi nessuna regola di protezione del ramo poteva ancora riferirsi al nome vecchio.
Dopo la prima corsa non sarebbe più stato gratis.

## Livello 1 — il compilatore

Le tre righe del **blocco A** di §7.4.1, **nove righe del blocco C** e **una del blocco B** —
otto dal Traguardo 2, **due dal Traguardo 3**. ⚠️ **I conteggi si ricontano sulla tabella qui
sotto, mai si deducono** — gotcha **#31**: questa riga ne ha già portati **tre** sbagliati, e il
terzo è del **2026-08-10**: diceva *«sette righe del blocco C … queste ultime otto dal Traguardo
2»*, e il Task 1 del Traguardo 3 aveva già consegnato `record_without_version.rs` **senza
scrivere qui la propria riga**. Un numeratore invecchia anche quando nessuno tocca il
denominatore. Del blocco **B**
(i gettoni) è coperta **una riga su cinque**, `promuovere testo a istruzione ← la porta
journal` (V19); perché gli altri quattro non lo siano lo dice la riga del blocco **B** in
«Cosa la porta NON controlla», e da oggi **lo dice in un posto solo**.

| Regola del catalogo | Dove è dichiarata | Caso negativo |
|---|---|---|
| `#![no_std]` su `kernel` e `simulator` | `crates/kernel/src/lib.rs` · `crates/simulator/src/lib.rs` | `crates/kernel/tests/compile_fail/std_in_kernel.rs` |
| `#![forbid(unsafe_code)]` sulle stesse | idem | `crates/kernel/tests/compile_fail/unsafe_in_kernel.rs` · `allow_overrides_forbid.rs` |
| `HashMap` non nominabile | conseguenza gratuita di `no_std` | `crates/kernel/tests/compile_fail/hashmap_in_kernel.rs` |
| **blocco C** · `V29 · §2.1` — i due tempi non si scambiano, **in nessuna delle due direzioni** | `crates/kernel/src/time.rs` — `Monotonic` e `WallTime` sono due tipi distinti | `monotonic_as_wall.rs` · `wall_as_monotonic.rs` |
| **blocco C** · `V29 · §2.1` — **nessuna via `From`/`Into`** fra i due tempi | idem: nessuna conversione è dichiarata, e il divieto non è più un commento | `no_conversion_from_monotonic_to_wall.rs` · `no_conversion_from_wall_to_monotonic.rs` |
| **blocco C** · `V29 · §2.2` — la **riduzione** di `below` non è sovrascrivibile | `crates/kernel/src/rng.rs` — `below` vive su `RngExt`, con `impl<R: Rng> RngExt for R {}` | `override_below.rs` |
| **blocco C** · `V29 · §2.8` — il kernel **non nomina un default** | `crates/kernel/src/parameters.rs` — nessun `impl Default`, e `new` pretende ogni campo | `parameters_have_no_default.rs` |
| **blocco C** · `V29 · §2.8 · ADR-0034` — una decisione **senza i parametri consegnati** | `crates/kernel/src/executor.rs` — `Executor::new` prende `Parameters` **per posizione**, quindi ometterli è un errore di arità e non un valore messo di default in silenzio | `executor_without_parameters.rs` |
| **blocco C** · `Q9 · I6 · V20` — `Untrusted` **dove è attesa** un'`Instruction`, **regola A** | `crates/kernel/src/boundary.rs` — `Instruction` e `Untrusted` sono due tipi distinti | `untrusted_as_instruction.rs` |
| **blocco C** · `Q9 · I6 · V20` — **nessuna via `From`/`Into`** da `Untrusted` a `Instruction`, **regola B** | idem: nessuna conversione è dichiarata, e l'unica strada ammessa è `promote`, che pretende il giornale | `no_conversion_from_untrusted_to_instruction.rs` |
| **blocco C** · `Q14 · §4.9` — un record durevole **senza versione** | `crates/kernel/src/record.rs` — `Record` è un enum di versione, e l'`encode` inerente vive su di esso, non sul corpo `RecordV1` | `record_without_version.rs` |
| **blocco C** · `Q9 · I6 · V20 · §4.9` — un payload non fidato **senza la propria etichetta** | `crates/kernel/src/record.rs` — `RecordV1::trust` non è un `Option` e `Trust` non implementa `Default`. ⛔ **Le metà sono due e i casi sono due**, come per i due tempi: il primo tiene *«il campo esiste»* (`E0063`), il secondo *«e non ha default»* (`E0277`) | `record_without_trust_label.rs` · `trust_has_no_default.rs` |
| **blocco B** · `V19` — **promuovere testo a istruzione ← la porta `journal`** | `crates/kernel/src/boundary.rs` — `Untrusted::promote` pretende il giornale come **argomento**, e la registrazione fallita fa fallire la promozione | `promote_without_journal.rs` |

⛔ **La regola uscita dalla riga della regola B, e vale oltre il caso** (gotcha **#36**, terza
occorrenza): quando un compito implementa un controllo che il catalogo non ha, il posto dove
dichiararlo è **il registro**, e la dichiarazione va scritta come **voce aperta**, non come
nota. Una nota si legge e si dimentica; una voce aperta la §6 del
[compendio](COMPENDIO.md) la porta finché qualcuno la chiude.

#### Come scattano le due direzioni, e perché la differenza conta

⛔ **Le due direzioni non sono simmetriche nel modo di scattare, e la differenza conta —
gotcha #42.** `trybuild` stampa **`error`** quando un caso ha compilato e **`mismatch`**
quando l'uscita non combacia con l'oracolo:

| Regola | Scatta come | Dipende dall'oracolo? |
|---|---|---|
| i due tempi non si scambiano | `mismatch` | **sì** — una rigenerazione in blocco la spegnerebbe in silenzio |
| nessuna via `From`/`Into` fra i due tempi | **`error`** | no |
| `below` non sovrascrivibile | **`error`** | no |
| l'esecutore senza i parametri consegnati | **`error`** | no |
| `Untrusted` dove è attesa un'`Instruction` — **regola A** | ⛔ **niente: resta `ok`** | no, ed è **peggio** — vedi sotto |
| nessuna via `From`/`Into` da `Untrusted` a `Instruction` — **regola B** | **`error`** | no |
| un record durevole senza versione | **`error`** | no |
| un payload non fidato senza la propria etichetta — **il campo esiste** | **`error`** | no |
| la stessa riga, **e non ha default** — `trust_has_no_default.rs` | **`error`** | no |

⛔ **La riga della regola A è la divergenza, e si registra invece di allinearla all'attesa.**
Il gotcha #42 prevedeva un `mismatch` — rustc che aggiunge righe di `help: call Into::into`
che l'oracolo non porta — e **su questa coppia non succede**. Misurato: con
`impl From<Untrusted> for Instruction` presente, `untrusted_as_instruction.rs` resta **`ok`**,
perché lì lo scarto è fra **riferimenti** (`&Untrusted` contro `&Instruction`) e quell'impl non
dà nessun `&Untrusted: Into<&Instruction>`, quindi rustc non ha suggerimenti da appendere. Sui
due tempi lo scarto è fra **valori posseduti**, e lì il suggerimento compare. Quella guardia
non è «disarmabile da una rigenerazione»: è **cieca fin dall'inizio**, e su I6 il caso della
regola B non è una rifinitura — senza, l'`impl From` lascia la porta **verde** con il confine
già caduto.

Misurato: col solo caso «passa l'uno per l'altro», aggiungere `impl From<WallTime> for
Monotonic` lasciava la porta **verde su sei controlli su sei**. La riga `From`/`Into` esiste
perché quella era la direzione **pericolosa** — una decisione che dipende dal wall time — e
perché una regola guardata solo da casi `mismatch` non è guardata abbastanza.

#### Le contro-sonde delle righe nuove

Per file — la direzione che si dimentica (§7.1.1 regola 3):

| File | Righe che difende | |
|---|---|---|
| `crates/kernel/tests/time_types.rs` | **blocco C** · `V29 · §2.1`, entrambe | sette test |
| `crates/simulator/tests/seeded_rng.rs` | **blocco C** · `V29 · §2.2` | otto test |
| `crates/kernel/tests/boundary_promotion.rs` | **blocco C** · `Q9 · I6 · V20`, **entrambe** — la promozione dichiarata è la contro-sonda della regola A e della regola B — **e blocco B** · `V19` | ⚠️ **quindici** test — ricontati **sul binario** il 2026-08-10 chiudendo il traguardo: la cella diceva **otto**, ed era ferma a prima che il Task 7 vi portasse le sonde della nota, dell'accordo fra `kind` e operazione, e le due in cui la sonda dettata è stata **divisa**. Gotcha **#31** |
| `crates/kernel/tests/parameters_delivered.rs` | le **due** righe **blocco C** · `V29 · §2.8 · ADR-0034` | quattro test |
| `crates/kernel/tests/record_shape.rs` | **blocco C** · `Q14 · §4.9` **e** `Q9 · I6 · V20 · §4.9` — la contro-sonda dell'etichetta è `every_trust_label_survives_the_round_trip_and_the_two_differ_in_the_bytes`, che scrive **entrambi** i valori e ne confronta i byte | ⚠️ **dodici** test — ricontati **sul binario** il 2026-08-10 chiudendo il traguardo: la cella diceva **dieci**, e i due mancanti sono `the_reason_survives_the_round_trip_and_travels_beside_the_payload` e `an_empty_record_is_nine_bytes_and_the_inner_array_holds_five`, arrivati col **Task 7** — verificati col `diff` fra i due commit invece che dedotti. Gotcha **#31** |

⛔ **`boundary_promotion.rs` è la contro-sonda di due blocchi insieme**, e ciascuno dei suoi
test dice una cosa sola: la promozione dichiarata **compila ed è registrata**, col proprio
passo e la propria ragione — contare i record non basta, un `promote` che scrivesse il passo
sbagliato o una ragione vuota lascerebbe il conteggio a uno (gotcha #30); un giornale che
**rifiuta rifiuta anche la promozione**, altrimenti V19 poggerebbe sulla diligenza del
chiamante; `summarize` restituisce `Untrusted`, e a provarlo è **l'annotazione**, cioè il
compilatore, non un'asserzione; `summarize` conta **caratteri e non byte** — misurato: con
fixture di solo ASCII gli altri test restavano verdi mentre una fetta di byte moriva sul primo
taglio dentro un carattere multi-byte; e il `Debug` di `Untrusted` **non stampa il contenuto**,
che chiudeva una via d'uscita dal confine — con `Debug` derivato,
`Instruction::new(format!("{:?}", untrusted))` portava il testo attraverso intatto.

⛔ **`parameters_delivered.rs` colma un buco del registro, non solo del codice.** Fino a oggi
qui era nominato il **solo** caso `compile_fail` di quelle due righe e **nessuna contro-sonda**,
mentre la regola 3 di §7.1.1 ne pretende due per voce. Il file pinza che il valore **porti** i
parametri risolti, che due valori diversi si distinguano **e** che due uguali non denuncino una
sostituzione mai avvenuta (gotcha #24), e che **nessun ripiego viva dentro il costruttore** —
la via d'ingresso più economica per un default, che il compilatore **non** può vietare e che
§2.8.4 dichiara come limite.

#### I casi di `compile_fail`, e cosa provano davvero

⛔ **I dieci casi nuovi nominano `kernel::` e non ridichiarano attributi propri**, a differenza
dei quattro del Traguardo 1: è il rimedio al gotcha **#39**, e significa che i loro oracoli
sono accoppiati alla **superficie pubblica del kernel**. Un cambio di firma li rende rossi, ed
è corretto che lo faccia. ⚠️ **Ricontati il 2026-08-09:** questa riga diceva *«cinque»*, e la
cartella `crates/kernel/tests/compile_fail/` ne conteneva allora **quattordici** in tutto —
**quattro** dal Traguardo 1 e **dieci** dal Traguardo 2. ⚠️ **Ricontati una seconda volta il
2026-08-10, chiudendo il traguardo, e col comando invece che a memoria** —
`ls crates/kernel/tests/compile_fail/*.rs | wc -l` → **diciassette**: i quattordici più **tre**
dal Traguardo 3, `record_without_version.rs` dal Task 1 e la coppia
`record_without_trust_label.rs` · `trust_has_no_default.rs` dal Task 2. ⛔ **Erano già tre al
commit precedente, e nessuno aveva toccato questa cifra:** è il gotcha **#31** sul numeratore di
un contenitore che cresce, e chi lo muove è chi scrive un caso — che questa riga non la apre
nemmeno. La frase *«i dieci casi nuovi»* qui sopra resta **vera del Traguardo 2**, e vale identica
per i tre del Traguardo 3, che nominano `kernel::` allo stesso modo.

⛔ **La colonna «Caso negativo» prova il meccanismo, non la dichiarazione — e il registro
non deve lasciar credere altro.** I **quattro casi del Traguardo 1** — `std_in_kernel.rs`,
`unsafe_in_kernel.rs`, `allow_overrides_forbid.rs` e `hashmap_in_kernel.rs` —
**ridichiarano ciascuno i propri attributi** e non nominano mai `kernel::`. Provano che
`#![no_std]` e `#![forbid(unsafe_code)]` **mordono dove sono dichiarati**; non provano che
siano dichiarati nel kernel. Tolto `#![forbid(unsafe_code)]` da `crates/kernel/src/lib.rs`
e scritto un `unsafe` vero, quei casi restano **verdi**. ⚠️ **Il limite riguarda quei quattro e
non la cartella:** i dieci del Traguardo 2 fanno l'opposto, e attribuire anche a loro questa
debolezza direbbe che il gotcha #39 è ancora aperto.

A sorvegliare la **presenza** degli attributi è `scripts/gate-attributes.sh`, che è di
**livello 2**: un controllo esterno, quindi cancellabile. La riga di `forbid` è di ramo
**1b** — sostiene la validità dei blocchi A, B e C — e poggia quindi su un controllo più
debole di quello che difende. È dichiarato, non nascosto.

**Contro-sonde:** `crates/platform/tests/counter_probes.rs` — `platform` nomina `std` e usa
`unsafe`, e **compila**. Sono la direzione che si dimentica (§7.1.1 regola 3).

**Guardia di non-vacuità del banco:** `crates/kernel/tests/compile_fail.rs` conta i `.rs`
prima di chiamare `trybuild`. Senza, il banco **vuoto** usciva **verde**: un glob che non
pesca niente non è un errore per `trybuild`, che stampa un avviso e lascia i fallimenti a
zero. Misurato, non dedotto.

## Livello 2 — controlli esterni

Ogni voce porta **due** direzioni, per la regola 3 di §7.1.1: quella che deve scattare e
quella che deve restare verde.

| Regola del catalogo | Dove | Deve scattare | Deve restare verde |
|---|---|---|---|
| allow-list, grafo **spedito** | `scripts/gate-deps.sh` | N2 · **N5** | N1 · **N4** |
| allow-list, grafo **di build** | idem, e l'errore è **diverso** | N3 | N1 |
| cancello senza OS su `x86_64-unknown-none` | `scripts/gate-no-os.sh` | B2 · **B4** | B1 · **B3** |
| le crate vincolate **dichiarano davvero** i propri attributi | `scripts/gate-attributes.sh` | `forbid` tolto · `deny` al posto di `forbid` · attributi tolti a `simulator` · file atteso assente · lista dei vincolati vuota | stato pulito · `platform`, `secrets` e `daemon` |
| le crate vincolate **non hanno un build script** | idem, e l'errore è **diverso** | `crates/kernel/build.rs` · `crates/simulator/build.rs` · `build = "gen.rs"` nel manifesto · manifesto assente | `crates/platform/build.rs` · `build = false` |
| coerenza della documentazione | `scripts/check-docs.sh` | S1…S6c · S7 · S7b · S7c · S7d | C0 · C5 · **C6** |
| i **test di contratto** fra porta finta e porta vera — porta `reactor` | `crates/kernel/tests/reactor_contract.rs`, incluso da `crates/platform/tests/reactor_contract_real.rs` | **R3** · **R4** · R5 | R1 · R2 · **R6** |
| i **test di contratto** — porta `journal`, **due implementazioni su due** | `crates/kernel/tests/journal_contract.rs`, incluso da `crates/platform/tests/journal_contract_real.rs`. ✅ **Ricontata il 2026-08-10 col Task 9:** questa riga diceva *«una implementazione su due»* ed era giusta, perché la suite girava contro la sola finta. Ora gira contro **entrambe a ogni commit** — `MemoryJournal` e `platform::journal::FileJournal` — **otto promesse su otto verdi** su ciascuna: *«misurato una volta con un file usa-e-getta»* è diventato *«tenuto»*, ed è l'unica differenza che questa riga esisteva per non lasciar sfumare. ⛔ **E la colonna «deve scattare» aveva una riga ASSENTE, trovata il 2026-08-10 partendo dall'elenco dei bugiardi invece che dalla colonna:** **J13** esisteva nella tabella delle sonde qui sotto dal Task 11 e **non era mai entrato qui**, quindi da questa riga il bugiardo della **7b** — l'unico che sbaglia dicendo **no** — risultava inesistente. È la specie che non si vede leggendo, perché ciò che manca non si legge; a trovarla è stato il movimento inverso, e i bugiardi sono **nove**. ⚠️ Le promesse verdi su ciascuna implementazione sono oggi **nove**, non otto: la cifra qui sopra è quella del Task 9 e resta perché descrive **quella** misura | **J2** · **J3** · **J4** · **J5** · **J6** · **J7** · **J8** · **J10** · **J13** | J1 · **J9** · **J11** · **J12** |
| ⛔ i **byte congelati** del record durevole — §4.9.4, riga di catalogo `Q14 · §4.9` | `crates/kernel/tests/frozen_bytes.rs`, più `tests/frozen/` — **tre** `.cbor` e **una** mappa. ⛔ **Non si rigenerano, e non c'è nessun percorso per farlo:** niente flag, niente variabile d'ambiente, niente `--bless` — è così che `trybuild` si disarma (gotcha **#25**). I byte sono stati **scritti a mano** dall'uscita di una sonda usa-e-getta, cancellata nello stesso commit | **F1** · **F2** · **F3** | **F4** · **F5** · **F6** |

#### Le sonde, per nome

| | |
|---|---|
| **N1** | lo stato pulito passa — è il verde di partenza, non una violazione colta |
| **N2** | una crate **spedita** fuori lista → `I3 violated`, e il rimedio è **TOGLIERE** |
| **N3** | una crate **di build** fuori lista → l'altro messaggio, e il rimedio è **AGGIUNGERLA**. Sono due grafi proprio perché i rimedi sono opposti |
| **N4** | `getrandom` in `platform`, dove ADR-0031 lo ammette: **resta verde**. È la sonda che di solito si dimentica |
| **N5** | un nome di crate con la **maiuscola**: prima usciva **verde**, ed era un falso negativo su I3. Corretto allargando la classe di caratteri del filtro, con la ragione scritta accanto alla classe |
| **B1** | `kernel` e `simulator` compilano per il bersaglio senza OS |
| **B2** | `getrandom` in `kernel` → `target is not supported` |
| **B3** | contro-sonda: con `--workspace` il cancello fallirebbe su `platform` con `can't find crate for std` — **motivo giusto, crate sbagliata**. Per questo il comando nomina `-p kernel -p simulator` |
| **B4** | il bersaglio non installato → uscita 1 e messaggio corretto. ⚠️ Vedi sotto: è la via *offline* |
| **S1…S7d · C0 · C5 · C6** | **diciotto su diciotto**, con ripristino byte-identico della spec: le tredici del 2026-08-07 (§8.6.3) più le quattro di `S7` e la contro-sonda `C6`, aggiunte chiudendo la §7.1.1 |
| **R1** | la **finta** onora il contratto — il verde di partenza dal lato `simulator` |
| **R2** | la **vera** onora il contratto: la stessa funzione, gli stessi assert, l'altra implementazione. È ciò che la suite esiste per comprare |
| **R3** | `NullAdvanceLiar`, che risponde `Some` a una scadenza **pari** all'istante corrente → la suite scatta. ⛔ E il test **legge il payload del panic**: verifica *quale* asserzione ha sparato, perché un `is_err()` nudo direbbe «ho colto il null advance» anche se a scattare fosse stata un'altra — una misura vera di un'altra cosa, gotcha #15 |
| **R4** | `PastDeadlineLiar`, che onora `deadline == now` e mente su `deadline < now` → scatta sul **solo** caso 2b. ⛔ È la sonda che prova che il caso 2b **non è vacuo**: prima che esistesse, cancellare l'intero blocco 2b lasciava la porta **verde**, e con essa la metà `<` del ramo priva di guardia. Gotcha **#45** |
| **R5** | la plausibilità di `wall_time()` sulla **vera** — `crates/platform/tests/wall_clock_plausibility.rs`: un istante posteriore a una data fissa del passato. Coglie l'orologio fermo a **zero o all'epoca**, che era la mutazione sopravvissuta alla prima stesura |
| **J1** | il **doppio in memoria** onora tutte e **nove** le promesse — il verde di partenza. ✅ **La promessa 7 non è più soddisfatta per la ragione sbagliata:** la voce aperta più sotto è **chiusa** dal Task 11, e la metà che discrimina è la **7b**. ⚠️ **Questa riga ha detto «tutte e sei», poi «tutte e otto»**, ed erano cifre stantie nella stessa riga: le promesse sono **nove in dieci blocchi** dal 2026-08-10 — 7b è la **seconda direzione** della 7, non una regola in più. Ricontate ogni volta sul sorgente, gotcha **#31** |
| **J2** | `SilentJournal`, che risponde `Ok(())` e non scrive → scatta la **promessa 1**. ⛔ È la via **A6** di `boundary.rs` resa eseguibile, ed è la ragione per cui questa suite esiste in questo traguardo |
| **J3** | `LastWriteWinsJournal`, che rilegge l'**ultimo** record del passo invece del primo → scatta la **promessa 2**. ⛔ È la forma che una tabella chiavata sul passo ha **per natura**, cioè quella che avrà `redb`: è la promessa che la seconda implementazione **non incontra da sola** |
| **J4** | `EmptyInsteadOfMissingJournal`, che riporta l'**assenza** come lettura riuscita di **niente** → scatta la **promessa 3** |
| **J5** | `ShuffledJournal`, che restituisce il giornale **rovesciato** → scatta la **promessa 4** |
| **J6** | `PermissiveJournal`, che accetta un esito **senza intento** → scatta la **promessa 5** |
| **J7** | `UnguardedIntentJournal`, che accetta un **secondo intento** sullo stesso passo → scatta la **promessa 6**. ⛔ **Rotto per assenza e non per menzogna:** ha il proprio archivio e la guardia su `outcome`, e gli manca solo quella su `intent` — è ciò che era `MemoryJournal` fino al 2026-08-10. Non lava un rifiuto altrui come fa `PermissiveJournal`, o sarebbe lo stesso difetto scritto due volte (gotcha #45) |
| **J8** | `EagerPruner`, che pota un passo **in dubbio** → scatta la **promessa 7**. ⚠️ Dal 2026-08-10 la promessa che uccide ha **due** asserzioni: l'intento nudo, e l'intento **con una nota** — una nota non è un esito, e senza il secondo caso una nota archiviata come esito rendeva potabile un passo in dubbio con l'intero workspace verde (misurato, mutazione `M12` del Task 11) |
| **J10** | `DiscardedNoteJournal`, che **controlla la nota e non ne conserva nulla** → scatta la **promessa 8**. ⛔ **Rotto in un ottavo modo** (gotcha #45): fa la verifica, risponde `Ok(())` e non scrive — *valida e poi butta*. `SilentJournal` è il vicino più prossimo e **non è lo stesso**: quello non verifica niente e muore sulla promessa 1, sei promesse prima. ⛔ **Ed è la forma che una implementazione vera prende davvero:** `note` è l'operazione più nuova della porta, quindi la più probabile da lasciare `Ok(())` mentre il resto è scritto bene — e ciò che smette di arrivare all'archivio è **il contenuto non fidato con l'etichetta che dice che lo era**, cioè la via **A6** puntata sulla via **A4**. ⚠️ **Sta qui e non fra J8 e J9** perché gli identificatori di questo registro **non si spostano**: J9 era già la contro-sonda quando questo bugiardo è nato |
| **J9** | ⛔ **la contro-sonda, ed è di due pezzi.** (a) La **durabilità attraverso la caduta del processo** sta **fuori** dalla suite — `the_memory_journal_does_not_survive_being_dropped` vive in `memory_journal.rs` — perché pretenderla in conformità renderebbe rossa la finta, che è **corretta** (gotcha **#44**). (b) La **mutazione di controllo**: cambiato un **solo commento**, tutti e **undici** i test restano verdi e `cargo test --workspace --no-fail-fast` dà **zero** rossi. Senza (b) la tabella qui sopra non prova niente — gotcha **#48**. ⚠️ **La cifra ha detto «otto» e poi «dieci»**: rimisurata il 2026-08-10 con la promessa 8, la sonda delle sottostringhe e il bugiardo della 7b |
| **J13** | ⛔ **`AlwaysInDoubtJournal`, che rifiuta OGNI potatura con la parola giusta → scatta la promessa 7b**, ed è ciò che rende non-vacua la **7**. ⛔ **Rotto in un nono modo** (gotcha #45): è **l'unico bugiardo del registro che sbaglia dicendo NO** — non distrugge niente e non mente, semplicemente non pone mai la domanda. `EagerPruner` è il suo **opposto** e non il suo gemello. ⛔ **Risponde `StepInDoubt` e non `Missing`, e la scelta è il controllo:** con `Missing` morirebbe sulla promessa **7**, sei righe prima, e la 7b resterebbe non provata mentre un test afferma il contrario. ⚠️ **Ed è ciò che erano ENTRAMBE le implementazioni fino al 2026-08-10**, a meno della parola |
| **J11** | ⛔ **il vincolo che rende sicuro `contains`, e fino al 2026-08-10 era solo dichiarato.** `no_promise_message_is_a_substring_of_another` — nessuno dei **nove** messaggi è sottostringa di un altro, e nessuno è vuoto. ⚠️ **Erano otto fino al Task 11**, e il conteggio delle coppie è stato **strumentato invece che calcolato**: **72 coppie ordinate**, erano 56. I due messaggi di `prune` sono la coppia più vicina che l'insieme abbia mai avuto — cominciano entrambi con *«a step »*. Senza, un bugiardo colto sulla promessa **sbagliata** soddisferebbe comunque il test che nomina quella giusta, e la suite direbbe `ok` indicando il posto sbagliato. ⚠️ **Misurato in due direzioni**, e la prima è quella che insegna: rendere un messaggio un semplice **prefisso** di un altro **non** lo fa scattare — giustamente, perché `contains` non è ingannato da un prefisso condiviso — mentre una **vera** inclusione (`MISSING_MESSAGE` = `"a `note` upon an open step must be"`) lo fa scattare **da solo**, e un messaggio **vuoto** anche |
| **R6** | ⛔ **la contro-sonda che conta, e la si sarebbe dimenticata:** rompendo l'avanzamento dell'orologio di parete del `VirtualReactor`, la conformità **resta verde** e scatta **solo** `crates/simulator/tests/virtual_clock.rs`. È la prova che la suite condivisa non impone alla vera un comportamento della finta — se lo facesse, renderebbe rossa un'implementazione **corretta**. Gotcha **#44** |
| **J12** | la **vera** onora il contratto — `the_real_journal_honours_the_contract` in `crates/platform/tests/journal_contract_real.rs`, la stessa funzione e gli stessi assert di **J1**, l'altra implementazione. È ciò che la suite esiste per comprare, ed è l'analogo di **R2**. ⚠️ **Dal Task 11 le promesse sono nove**, e `FileJournal` le onora tutte: `prune` è la prima operazione che gli è costata un **cambio di archivio** invece di una riga — vedi la voce del Task 11 più sotto. ⛔ **E la non-vacuità è misurata in TRE direzioni, non una**, perché una sola proverebbe **una promessa su otto**: rotta `FileJournal::read_back` perché risponda sempre `Ok(Vec::new())` muore sulla **promessa 1** col messaggio `READ_BACK_MESSAGE`; tolta la guardia sul **secondo intento** muore sulla **promessa 6** col `SECOND_INTENT_MESSAGE`, cioè **dopo** aver superato le cinque precedenti sui propri meriti; rovesciato `replay` muore sulla **promessa 4** col `REPLAY_ORDER_MESSAGE`. La **mutazione di controllo** — un solo commento dentro `FileJournal` — lascia **28 target e 144 test verdi**. ⚠️ **Le quattro cifre di questo periodo sono del Task 9 e non sono state rifatte:** *«una promessa su otto»*, `28 target`, `144 test`, e le due qui sotto. Oggi le promesse sono **nove** e il workspace è a **29 target e 152 test**, ricontati il 2026-08-10 — ma il rosso **non** è stato riprodotto contro il codice di adesso, quindi le cifre restano quelle misurate, con la data accanto. ⛔ **E la separazione fra i due lati è misurata anch'essa:** con `FileJournal` rotta, `kernel --test journal_contract` resta **10 su 10 verde** e dentro il binario di `platform` restano verdi i **dieci** test inclusi, rosso il solo J12 — cifre del **Task 9**; oggi quei binari portano **undici** e **dodici** test. Se cadessero insieme, i due lati non sarebbero due |
| **F1** | una **rinumerazione fra indici esistenti** — `kind` 0 ↔ `effect` 1 → **rosso**, tre sonde su sei, col messaggio che nomina il formato cambiato. ⚠️ **La mutazione che il piano dettava** — `payload` 3 → 2, già di `trust` — **non compila**: `error: duplicate index numbers`, quindi il controllo non si sarebbe **mai visto scattare**, e proprio sull'unico oracolo che non si rigenera |
| **F2** | un campo spostato su un indice **libero** — `payload` 3 → 7 → **rosso**. Due mutazioni per la stessa regola, e in due forme diverse, com'è il contro-verso del gotcha **#48** |
| **F3** | ⛔ **le OTTO varianti dei tre enum `index_only` rinumerate UNA PER UNA** su un indice libero → **otto rossi su otto**. È ciò che rende la copertura reale invece di apparente: un record congelato solo ne fisserebbe **tre** — `RecordKind` ha 3 varianti, `EffectClass` 3, `Trust` 2 — e le altre cinque resterebbero tenute da **nulla**, come il Task 1 aveva misurato |
| **F4** | ✅ un campo **facoltativo** con `#[cbor(default)]` su un indice **libero** → **verde**: i byte congelati non si muovono. È la regola 3 di §4.9.2, e ADR-0036 è confermato dalla misura invece che citato |
| **F5** | ⛔ **la metà che rende F4 non vacua:** `Some(9)` invece di `None` → **22 byte** invece di 21, con `86` e il valore in fondo. Senza di essa, *«i byte non si sono mossi»* sarebbe compatibile con un campo che sul filo **non arriva mai**, e il verde non proverebbe l'additività. Gotcha **#54** |
| **F6** | **controllo**: una parola di un commento di `record.rs` → **6 passed**, zero rossi |

#### Il build script — entrato su una lacuna misurata

⛔ **La riga del build script è entrata il 2026-08-09 su una lacuna _misurata_, e allora le
voci di livello 2 erano sei.** ⚠️ **Legata alla data e non alla posizione, il 2026-08-10:**
diceva *«è la sesta voce di livello 2»*, vero quando fu scritto e falso da quando la riga dei
test di contratto è entrata poche ore dopo — nella tabella di oggi, che ne ha **sette**, il
build script sta **quinto**. È la terza forma del gotcha **#31**: un'affermazione vera legata
al **contenitore** invece che a ciò che fu misurato, e il contenitore cresce senza avvisare. Un `crates/kernel/build.rs` che chiama `SystemTime::now()`,
`fs::metadata()` e `env::var()` e inietta il risultato con `cargo:rustc-env` lasciava la porta
**verde su sei controlli su sei**: `build` e `test` lo compilano perché è il mestiere di un
build script, `gate-no-os.sh` lo compila **per l'host anche con `--target`** e **lo esegue**,
`gate-deps.sh` non vede nodi nuovi se lo script non ha dipendenze proprie, `gate-attributes.sh`
leggeva solo `src/lib.rs`, e `check-docs.sh` non guarda il codice. Difende **`I3` e `V29`
direttamente** — non è di ramo 1b: `cargo:rustc-env` più `env!()` cuoce nel kernel un valore
letto dal mondo alla build, che è il gotcha #28. **Il rimedio è TOGLIERE**, come per il grafo
spedito, e il messaggio dello script lo dice insieme al perché — un controllo che sembra
pedanteria viene aggirato. Vive dentro `gate-attributes.sh` perché è **il punto cieco di quello
script**: `build.rs` ha attributi propri e il `forbid` di `lib.rs` non lo raggiunge.

#### Tre note sui limiti dichiarati di questi controlli

📌 **`gate-attributes.sh` è un controllo di testo, e non va promosso.** Cerca gli attributi
con `grep` ancorato a inizio riga: prova che il divieto sia **dichiarato**, non che nel
kernel non ci sia `unsafe`. Quella la prova il compilatore, ed è proprio ciò che questo
controllo tiene in piedi. Costo dichiarato nello script stesso: l'ancora chiude il caso
`// #![forbid(...)]`, che è quello reale, ma un attributo sepolto in un commento di blocco
`/* … */` sfugge ancora — chiuderlo richiederebbe un parser, cioè un rimedio più fragile
del buco.

⚠️ **B4 è la via _offline_, non una rete di sicurezza — e la differenza va detta.**
`rustup target list --installed` **riconcilia `rust-toolchain.toml` prima di rispondere**:
il manifesto dichiara `targets = ["x86_64-unknown-none"]`, quindi se il bersaglio manca
l'atto stesso di interrogarlo lo **reinstalla**, e la risposta è «c'è». Con la rete
disponibile la guardia **non può scattare**. Scatta quando la riconciliazione fallisce,
cioè **senza rete** — verificato: uscita 1 e messaggio corretto. È la via che rende
utilizzabile una macchina isolata, non un controllo che sorveglia la macchina connessa.

**Contro-sonde:** la colonna «deve restare verde» qui sopra, più
`crates/platform/tests/counter_probes.rs` al livello 1. ⚠️ Il punto che si dimentica è **perché**
`platform`, `secrets` e `daemon` restano verdi: non sono fra le crate vincolate, e un controllo
che scattasse anche lì sarebbe rosso per il motivo sbagliato (gotcha #24) — `platform` è **il
posto dove l'I/O deve vivere**.

#### Gli altri test che «cargo test --workspace» porta

**Cosa difende ciascuno.** ⛔ Stanno qui
perché il gotcha **#36** ha una forma pura e silenziosa: chi scrive un controllo lo considera
«scritto» e non lo riporta nel registro, e da fuori resta **indistinguibile da uno che non
esiste**. Nessuno di questi è una riga del catalogo — sono controlli di livello 2 che
sostengono righe del catalogo, o che tengono in piedi ciò che le righe presuppongono.

| File | Che cosa difende |
|---|---|
| `crates/kernel/tests/executor_determinism.rs` (dieci test) | **C1, C2 e C3 sull'esecutore _spedito_**, non su quello dello spike: **cento** corse allo stesso seme danno una traccia sola, **duecento** semi distinti non ne danno una sola, e il tempo virtuale **non attende** — l'orologio si ferma a 20 000 ms dove il sequenziale arriverebbe a 60 000. Più le sonde di **non-vacuità**: che l'interfoliazione sia reale, che un blocco diventi **errore e non attesa infinita**, che un reattore che non avanza sia **errore e non giro a vuoto**, che un'attesa già scaduta svegli subito senza muovere l'orologio, che una richiesta di sospensione **non si erediti** fra attività, e che un rideposito perpetuo di una scadenza passata **termini comunque** |
| `crates/kernel/tests/ports_are_implementable.rs` (**tredici** test) | il rimedio al gotcha **#46**: una **finta** per `Filesystem`, una per `Network`, due per `process` — `Worker` e `Process` — e una per `Ipc`, con chiamate che le esercitano in entrambe le direzioni. **Cinque finte per quattro famiglie**, ed è la copertura di **tutte** le porte dichiarate senza implementazione. È ciò che tiene in vita `Path::as_bytes()`, `Endpoint::as_bytes()` e il `Clone` su `Path` contro una passata YAGNI — su un tratto dichiarato **in anticipo** i chiamanti sono vuoti per costruzione, e il criterio non distingue il morto dalla sola porta d'ingresso di chi verrà — e prova che quelle firme siano **implementabili fuori dalla crate**, dove la privacy di modulo di una tuple-struct le renderebbe inutilizzabili. ⚠️ **Non** è una suite di conformità: quella pretende due implementazioni da confrontare |
| `crates/simulator/tests/memory_journal.rs` (**quindici** test — ⚠️ ricontati **sul binario** il 2026-08-10 col Task 11: la riga diceva **undici** e la cifra era già stantia prima di lui, gotcha **#31**) | il **doppio in memoria** del giornale (§4.1): che l'intento riletto torni intatto, che un passo mai scritto sia **`Missing` e non vuoto**, che un esito **senza** intento sia rifiutato **e** uno **dopo** il proprio intento accettato — le due direzioni, e la seconda mancava — che il rifiuto guardi **quale** passo, che `read_back` risponda con l'**intento** e non con l'esito, che ogni passo rilegga **il proprio** primo record, che un **secondo intento** sullo stesso passo sia **rifiutato senza scrivere** e che uno su un passo **diverso** resti **accettato** — le due direzioni della guardia decisa il 2026-08-10 — e che `prune` **rifiuti un passo in dubbio senza potarlo** — ⚠️ questa riga diceva *«rifiuti senza potare»* e descriveva la non-implementazione, superata dal Task 11 — e che risponda **`Missing`** a un passo mai scritto, che è la **terza** risposta e la sola che la conformità non tiene (voce aperta 2). ⚠️ **Non** è la suite di conformità, e dal 2026-08-10 la distinzione non è più «quella non esiste» ma **quella sta altrove**: `journal_contract.rs` porta ciò che **entrambe** le implementazioni promettono, questo file ciò che è vero **di questa sola** — l'ordine *dentro* un passo, il secondo intento, e che il giornale non sopravviva alla propria caduta |
| `crates/simulator/tests/crashing_journal.rs` (**dieci** test — ⚠️ il piano ne dettava **otto**, e le due arrivate dal pre-controllo sono la scrittura rifiutata e la potatura dopo la caduta) | ⛔ **il giornale che CADE** (§3.3, livello 1 dei due livelli di crash di ADR-0032) — ciò che promette **solo lui**, ed è per costruzione un bugiardo, quindi la conformità non lo tiene (gotcha **#50**): che cada **alla** scrittura dichiarata e non «da qualche parte», che dopo la caduta **ogni** scrittura successiva sia rifiutata — è la differenza fra un **crash** e un **disco cattivo**, decisione D2, ed è la permanenza a fermare *tutte* le attività interlacciate — che ciò che era stato scritto **sopravviva** e si rilegga da `into_survivor`, che `has_fallen` dica **no** finché non cade, e che un giornale a cui è stato detto di **non** cadere non cada mai in centoventotto scritture: quest'ultima è la direzione che si dimentica, e ⛔ **`C7a` poggia interamente su di lei** — misurato, è l'unica sonda **verde** sotto la mutazione «cade sempre» e **rossa** sotto «non cade mai». Più le due che il piano non chiedeva: che una scrittura **rifiutata dal protocollo** non consumi una posizione del conteggio — senza, il punto estratto scivola e con un punto vicino alla fine il guasto **non scatta**, gotcha **#17** — e che `prune`, **unica operazione mutante fuori dalle tre contate**, sia rifiutata dopo la caduta **senza armarla né consumarla**. ⚠️ **Una sonda è dichiarata non falsificabile invece che tolta:** `the_same_seed_chooses_the_same_write` non può fallire perché `from_seed` è pura, e la determinatezza vera la tiene `seeded_rng.rs` — postura del gotcha **#44** |
| `crates/simulator/tests/dst_campaign.rs` (**quattro** test — ⚠️ erano **due** al Task 2, e le due nuove sono `C7b` e la sonda dell'interlacciamento; ricontati **sul binario**) | ⛔ **la campagna DST di livello 1** (§3.3, ADR-0032): il soggetto sotto esame è la **riconciliazione del kernel**, e nulla tocca un disco. Consegna lo **scenario giornalato** — quello di M-2, tre attività per quattro passi, ora con intento ed esito attraverso la porta — la **traccia**, che sarà l'oracolo **indipendente** di `C7b` perché viene da ciò che le attività hanno saputo essere passato e non dall'archivio, e **`C7a`**: senza crash, **nessun** passo in dubbio, su cinquanta semi. Più il pin che **fissa** `WRITES_PER_RUN = 24`, senza il quale il punto di caduta si estrarrebbe contro un numero non verificato (gotcha **#17**). ⛔ **E l'oracolo di non-vacuità di `C7a`, che il piano non chiedeva ed è la riga che conta:** *«nessun passo è in dubbio»* e *«lo scenario non ha scritto niente»* erano **lo stesso verde** — misurato con un giornale che cade alla scrittura zero. Ora il ciclo pretende `writes_done() == WRITES_PER_RUN` **prima** di guardare l'insieme, ed è il gemello di `has_fallen()` sull'altra metà della campagna. ⚠️ **Ciò che questo file NON tiene, ed è dichiarato sul doc di `run`:** che lo scenario **interlacci**. Nessuna sua sonda va rossa se le tre attività girano una dopo l'altra; la proprietà la tiene `a_crash_leaves_more_than_one_step_in_doubt_on_at_least_one_seed`, un compito più in là — e il **nome** è un vincolo, perché il rimando lo cita. ✅ **E dal Task 3 quella sonda esiste, in questo stesso file**, insieme a **`C7b`**: il crash lascia **quell'insieme e non un altro**, confrontato con l'oracolo `expected_doubt` che viene dalla **traccia** e non dall'archivio — la sola ragione per cui `C7b` non è una tautologia, e ⛔ **misurata**: rompendo l'oracolo, il confronto va rosso con `[3]` dall'archivio contro dodici passi dalla traccia. Più il **confronto ordinato** invece che insiemistico, che morde davvero — un `replay` ordinato per passo dà `left: [0, 4]` contro `right: [4, 0]`, ed è il difetto che una tabella `redb` chiavata sul passo produrrebbe da sola. ⛔ **E `C7b` ha DUE oracoli di non-vacuità e non uno, perché provano cose diverse:** che **ogni** seme raggiunga il proprio punto di caduta — uguaglianza e non `> 0`, perché un seme che non cade significa che lo scenario ha scritto meno del numero contro cui il punto è estratto — e che **almeno un seme lasci più di un passo in dubbio**, senza cui la campagna confronterebbe insiemi vuoti restando verde. ✅ **Dal Task 4 `C7b` È la campagna breve**: il corpo per-seme vive in `campaign(seeds)`, che la campagna profonda riusa sotto `#[ignore]`, e non esiste un secondo ciclo più debole accanto — ⛔ il piano ne dettava uno, e sarebbe stato **quello** a finire nel cancello. ⛔ **E il numero di semi ha una TERZA guardia, che è il criterio con cui è stato scelto:** gli insiemi in dubbio distinti che questo scenario può produrre sono **centonove**, e la campagna pretende di vederli **tutti** — `EXPECTED_DOUBT_SETS`. Non è una proprietà ma un **rilevatore di cambiamento sulla forma dello scenario**, nella postura dei byte congelati, e ⛔ **è stato adottato solo dopo aver misurato che non scattasse dove non deve**: sei costanti di mescolamento diverse danno centonove tutte e sei, quindi il conteggio è dello **scenario** e non dei semi. Provato in due direzioni — a cinquecento semi ne vede centocinque e scatta |
| `crates/kernel/tests/reconciliation.rs` (⚠️ **undici** test — ricontati **sul binario** il 2026-08-10 chiudendo il traguardo: la cella diceva **nove**, e le due arrivate dopo sono del **Task 7**, la nota che non apre un dubbio e la nota che non tocca quello del chiamante. Gotcha **#31**) | la **riconciliazione** (§4.3, ADR-0007) — il primo consumatore di `replay()`: che un crash lasci **più** passi in dubbio e non uno (gotcha **#20**, `[3, 7]` col seme 99), che un passo con intento **ed** esito **non** sia in dubbio (la direzione che si dimentica), che la **classe decida** la risoluzione sui tre valori, e che un record indecifrabile valga `SuspendAndAsk`. ⚠️ **Quattro sonde che il compito non chiedeva:** il giornale **vuoto** — il primo avvio, che nessuna sonda dettata incontrava — l'**ordine di scrittura** scritto `7, 3, 1` perché quella dettata attendeva `[3, 7]`, che è ordine di scrittura **e** ordine numerico insieme, e le **due dell'insieme**: al più una voce per passo, e un passo che rientra **conserva il posto**. ⛔ **Ciò che questo file NON tiene, ed è dichiarato in `reconcile.rs`:** che il `kind` del record concordi con l'operazione che l'ha scritto. ⚠️ **Questo rimando diceva *«vedi la voce aperta in fondo»* e puntava a una voce che non è più aperta:** la questione delle **due verità** è **chiusa dal proprietario** il 2026-08-10 — come **decisione** e non come garanzia — e a valle esiste `the_promotion_writes_through_note_and_the_record_says_note`, che l'accordo lo fissa per **l'unico scrittore che esiste** |
| `crates/platform/tests/file_journal.rs` (**sei** test) | ciò che **solo** il giornale su file promette (§4.1, ADR-0032), e che pretenderlo in conformità renderebbe rossa la finta — gotcha **#44**: che una scrittura **sopravviva alla riapertura**, che una transazione **mai confermata non lasci nulla** (requisito 1 di §10.6), che il contatore delle chiavi **riprenda dall'archivio** invece che da zero — altrimenti la seconda sessione **sovrascrive** la prima in silenzio — che la guardia sul **secondo intento** regga **attraverso una riapertura**, perché legge l'archivio e non un campo della sessione, e che il **lucchetto** rifiuti un secondo giornale sullo stesso file **mentre il primo è aperto** (l'altra direzione la tiene la prima sonda: chiuso il primo, la riapertura riesce). ⛔ **E la sesta è la prova che il confine è reale:** `CountingBackend` è una **seconda implementazione di `redb::StorageBackend` scritta da fuori la crate`**, `FileJournal` ci gira sopra invariato, e i contatori dicono che l'I/O **passa davvero di lì** — senza quell'asserzione un giornale che accettasse il backend e scrivesse altrove resterebbe verde. È il rimedio al gotcha **#46** applicato al confine su cui il **Traguardo 4** inietterà i guasti di livello 2. ⚠️ **Non** è la suite di conformità: quella sta in `journal_contract.rs` e gira contro **entrambe** dalla riga qui sotto |
| `crates/platform/tests/journal_contract_real.rs` (**un** test proprio, ⚠️ **dodici** eseguiti — ricontati **sul binario** il 2026-08-10 chiudendo il traguardo: la cella diceva **undici**, ferma a prima che il Task 11 portasse il nono bugiardo. Gotcha **#31**) | ⛔ **la conformità della porta `journal` contro l'implementazione VERA**, e il file è corto perché **le asserzioni non si ripetono**: `include!("../../kernel/tests/journal_contract.rs")` le raggiunge testualmente, come `reactor_contract_real.rs` fa per `reactor`. Due copie divergerebbero e **la prima che diverge mente stampando `ok`**. ⚠️ **Costo dichiarato e non nascosto:** l'inclusione porta con sé anche i `#[test]` del file incluso, quindi la finta, i **nove bugiardi** e la sonda delle sottostringhe **girano una seconda volta** dentro il binario di `platform` — **dodici** test in tutto, di cui **uno solo** tocca il disco. ⚠️ **Questa frase diceva «otto bugiardi» e «undici test»**, ed erano le cifre del Task 9: `AlwaysInDoubtJournal` è entrato col Task 11 e nessuno le ha mosse. Gotcha **#31**. ⛔ **E la fabbrica dà un file NUOVO a ogni chiamata — nove per corsa — invece di cancellarne uno fisso:** su Windows la cancellazione **fallisce in silenzio** se il file è ancora aperto e la fabbrica riaprirebbe **i dati vecchi** (gotcha **#52**), `FileJournal` tiene un **lucchetto esclusivo**, e la promessa 4 conta l'**intero** archivio. La numerazione passa da un `AtomicU64` perché `assert_journal_contract` prende **`Fn`**, non `FnMut`. La cartella è **una per call site**, dal `line!()`, con un **prefisso diverso** da quello di `file_journal.rs`: i due binari girano insieme e un numero di riga è unico dentro **un** file solo. Sonda **J12** |
| `crates/platform/tests/engine_crash_consistency.rs` (**tre** test al Task 5) | ⛔ **il LIVELLO 2 dei due livelli di crash** (ADR-0032, §4.6): il soggetto sotto esame **non è il kernel** ma **`redb` stesso**, guidato attraverso un `StorageBackend` che cade a un'operazione scelta. ⛔ **Vive in un banco di prova e non in `platform/src/`, ed è il punto:** ciò che il Task 8 del Traguardo 3 comprò è che quel confine sia raggiungibile **da fuori la crate** (gotcha **#46**), e un backend cadente scritto **dentro** `platform` non proverebbe nulla su quello. Tiene: che senza caduta l'archivio **si riapra con tutto dentro** — la direzione che si dimentica, messa **per prima**, perché se cadesse ogni rosso successivo parlerebbe del backend invece che dell'iniezione — che il backend **cada all'operazione dichiarata** e non prima, e che la caduta sia **permanente**. ⛔ **E l'oracolo che chiuderà il gotcha #51 è un DELTA e non un conteggio:** sei sync su sette nascono **prima che esista un record**, quindi *«`sync_data` è stato chiamato»* è soddisfatto da un motore che **non sincronizza nessuna scrittura** — misurato, con `Durability::None` la forma assoluta resta **verde** e quella a delta va rossa. ⚠️ **Ciò che questo file NON tiene, ed è deliberato:** la **coerenza dopo la riapertura**, che è il soggetto del compito successivo e a cui va lasciato il proprio rosso da cui partire |
| `crates/kernel/tests/dependencies_usable.rs` (due test) | che le voci **spedite** dell'allow-list **compilino e facciano round-trip** — gotcha #22, `cargo add bincode` risolve a una versione il cui intero sorgente è un `compile_error!`. E per `bincode` i **byte consumati** sono pari alla lunghezza dichiarata, che è la regola imposta dal gotcha **#34**: un decodificatore che si ferma al primo elemento completo e ignora la coda «ha decodificato» senza provare niente |

#### Le finte delle porte — cosa hanno colto, e cosa hanno potato

⛔ **`ports_are_implementable.rs` ha smesso di confermare e ha colto un difetto — il
2026-08-09, sulla porta `process`.** La porta **come il piano la dettava non era
implementabile**: `instruct_one` deve **restituire** un `SingleReceipt` il cui unico campo è
`pub(crate)`, e la privacy di un campo di struct è **di modulo** — da fuori dalla crate quel
valore non si poteva costruire. È il gotcha **#46** nella forma peggiore: non «non riesco a
**leggere** un campo» ma «non riesco a **produrre** il valore di ritorno». Il rimedio — `new` e
`id` su entrambe le ricevute — porta la ragione accanto a sé in
`crates/kernel/src/ports/process.rs`. ⚠️ `Grant` resta **senza costruttore**, ed è l'opposto
deliberato: §5.6 la vuole inedificabile.

⚠️ **E `Grant` diverge dal piano, misurato.** Il campo con nome `reserved_mib: u64` costava un
`#[allow(dead_code)]` — qui un `allow` è un divieto spento (gotcha **#13**) — e anticipava un
pezzo del modello dell'arbitro, che è del Traguardo 5. Il campo **unitario privato** dà la
garanzia **identica** a costo zero: da fuori `Grant(())` è `E0423`, senza warning e senza
`allow`. Via anche `Debug`: nessuno formatta una concessione. ⚠️ Sulle **ricevute** `Debug` si
tiene, lo pretende `unwrap_err`. ⛔ E la misura ha una trappola che vale oltre il caso: **gli
errori di rustc si mascherano fra passate** — col costruttore assente usciva `E0599`, e sul
letterale **nessun errore**, perché l'`E0451` lo emette la passata di **privacy**, che non gira
se il type-check si è già fermato. Gotcha **#47**.

⛔ **`Clone` potato da `WorkerDescriptor` e da `Frame`, e la contro-sonda è ciò che rende la
potatura difendibile.** Tolti da questi due: **verde, zero warning**. Tolto da `Path` come
contro-sonda: **rosso**, `E0277` · `E0308` · `E0599`. Su `Path` e su `Endpoint` `Clone` è
**portante** — `declare_scope` consegna un **prestito** che l'implementazione deve trattenere —
mentre `WorkerDescriptor` e `Frame` attraversano la porta **per valore**. ⚠️ «Non implementabile
oggi» e «un chiamante lo vorrà domani» sono due forme diverse, e a distinguerle è **la finta**.

⛔ **La porta `ipc` ha usato la stessa finta per il servizio opposto — il 2026-08-10 — e la
differenza fra i due esiti è la parte da tenere.** Su `process` la finta ha **colto un
difetto**; su `ipc` ha compilato al primo colpo, e ciò che ha comprato è stata una
**sottrazione**. Scritta **prima** del sorgente, non ha usato nessuna delle quattro voci che il
piano dettava per `ClientId` — misurate **una per una**, togliendo e ricompilando:

| Voce del piano | Misura | Esito |
|---|---|---|
| `ClientId::get()` | tolto → `build` e `test --workspace` **verdi, zero warning** | ⛔ **cancellato** |
| `Hash` | idem | ⛔ **cancellato** |
| `PartialOrd`/`Ord` | idem | ⛔ **cancellato** |
| `PartialEq`/`Eq` | **contro-sonda** — tolto → **rosso**, `E0369` («`==` cannot be applied») | ✅ **tenuto** |
| `Copy` | **contro-sonda** — tolto → **rosso**, `E0382`: **ventitré**, su **otto** siti di dichiarazione | ✅ **tenuto** |
| `Debug` | **contro-sonda** — tolto → **rosso**, `E0277`, lo pretende ogni `assert_eq!` | ✅ **tenuto** |
| `Clone` | **non è una scelta**: lo pretende `Copy` — tolto da solo, `kernel` **non compila** (`E0277`) | ✅ **tenuto dal compilatore** |

⛔ **Le contro-sonde non sono cerimonia: sono ciò che regge l'argomento della prima riga.**
`get()` è cancellato *perché* un'implementazione **conserva** un identificativo `Copy` e lo
confronta con `==`, come `InMemoryFilesystem` fa con `CheckpointId` — ma se `PartialEq` non
fosse davvero esercitato quel «perché» poggerebbe sul nulla, che è esattamente il modo in cui
`SingleReceipt::id` era rimasto in vita senza copertura. ⚠️ L'argomento **a favore** di `Ord` era
reale — il **#12** vieta `HashMap` e spinge su `BTreeMap` — e cade su un criterio preciso:
`Ord` **non blocca** chi implementa da fuori (una tabella più `==` basta) e si aggiunge dopo in
una riga. L'eccezione del **#46** copre *«non implementabile oggi»*, non *«comodo domani»*.
`Hash` è il peggiore dei tre: **abilita la cosa vietata**.

#### Le passate di mutazione, e le righe che contano

⛔ **Ogni finta porta la propria passata. La prosa sotto le tabelle è solo per le mutazioni
uccise da _un test solo_:** sono le uniche che dicono qualcosa che la tabella non dice già.

**`process` — dodici mutazioni, dodici uccise.**

| | Mutazione | Chi la uccide |
|---|---|---|
| M1 | ricevuta ignota → `Ok(None)` invece di `UnsolicitedFrame` | `..._refuses_where_it_must` |
| M2 | `read_one` ignora la morte del worker | `..._refuses_where_it_must` |
| M3 | il flusso non si esaurisce mai | `..._can_be_implemented_and_called` · `..._correlated_...` |
| M4 | `instruct_one` accetta un frame vuoto | `..._refuses_where_it_must` |
| M5 | `close` ammette una ricevuta mai aperta | `..._refuses_where_it_must` |
| M6a | `read_one` → costante **7** | `..._can_be_implemented_and_called` · `..._correlated_...` |
| M6b | `read_one` → costante **1** (la scelta **avversaria**) | ⛔ **solo** `..._correlated_...` |
| M7 | `read_next` prende il flusso 0 invece di cercarlo | `..._correlated_...` · `..._refuses_where_it_must` |
| M8 | `close` rimuove il flusso 0 invece di cercarlo | `..._correlated_...` · `..._refuses_where_it_must` |
| M9 | **de-correlazione totale**, zero `receipt.id()` nella finta | `..._correlated_...` · `..._refuses_where_it_must` |
| M10 | `WorkerDescriptor::new` perde un byte | `..._start_is_not_callable` |
| M11 | `kill()` **acquista** una guardia di liveness | `killing_a_worker_consumes_it` |

⛔ **M6b è la riga che conta, e si vede solo provando _due_ valori.** Il registro aveva
dichiarato che «`read_one` su costante» uccideva: vero per la costante **7**, **falso** per la
costante **1**, che coincideva con l'id atteso — **verde su 8 test su 8**, correlazione persa e
nessuno se ne accorgeva. ⛔ E il fondo era più basso: con tre mutazioni combinate la finta
conteneva **zero** occorrenze di `receipt.id()` e la suite restava verde. La causa non era la
costante ma **la forma della suite**, che non teneva mai due ricevute aperte insieme. Rimedio:
`answers_are_correlated_to_the_receipt_that_asked`, con lunghezze **diverse** e chiudendo il
flusso in posizione **1** — a budget uguale, o chiudendo la posizione 0, la mutazione
sopravviveva. Gotcha **#15**.

⛔ **M11 è nata sopravvivendo.** Il file spendeva quattro righe a dichiarare che `kill` la
guardia di liveness **non ce l'ha, di proposito** (§5.3 punto 4) — e **niente lo teneva**:
aggiungendola, i nove test restavano verdi. Una riga — uccidere un worker **già morto** — la
rende rossa. ⚠️ La mutazione è nata da una **rifinitura di stile**: estratto l'aiutante
`alive()`, l'unico punto che non lo chiama è diventato **visibile**, e visibile ha fatto
chiedere se fosse provato. Un principio che non si può controllare è un'intenzione.

**`ipc` — quattordici mutazioni, quattordici uccise.**

| | Mutazione | Chi la uccide |
|---|---|---|
| M1 | `accept` riusa l'identificativo | `..._delivered_to_the_client_they_name` · `a_dead_client_...` |
| M2 | `live()` ignora la morte del client | `..._refuses_where_it_must` · `a_dead_client_...` |
| M3 | `live()` smette di guardare le identità (prende il client 0) | i tre sopra |
| M4 | `receive` risponde `Ok(None)` a un client ignoto | `..._refuses_where_it_must` |
| M5 | `receive` risponde `Err` invece di `Ok(None)` a vuoto | tre test |
| M6 | `send` consegna sempre al client 0 | `..._delivered_to_the_client_they_name` |
| M7 | `accept` consegna un client che nessuno ha chiesto | `..._can_be_implemented_and_called` |
| M8a | `accept` conia la costante **7** | `..._delivered_...` · `a_dead_client_...` |
| M8b | `accept` conia la costante **1** (la scelta **avversaria**: è l'id reale del primo) | idem |
| M9 | `receive` non consuma il messaggio | `..._can_be_implemented_...` · `..._refuses_...` |
| M10 | il controllo sul messaggio malformato sparisce | `..._refuses_where_it_must` |
| M11 | morire non uccide | `..._refuses_...` · `a_dead_client_...` |
| M12 | ⛔ la morte è **contagiosa**: ne muore uno, muoiono tutti | ⛔ **solo** `a_dead_client_...` |
| M13 | ⛔ `accept` **ricicla** l'identificativo di un client morto | ⛔ **solo** `a_dead_client_...` |

⛔ **M12 e M13 esistono perché la prima passata ha colto il gotcha #45 dentro il lavoro di
oggi.** Con le prime dodici, `a_dead_client_does_not_take_the_port_with_it` non era **l'unico
uccisore di nessuna**: cancellandolo, tutte e dodici morivano lo stesso — cioè il test che porta
la proprietà per cui la porta esiste, **la gui è sacrificabile**, era cancellabile lasciando la
porta **verde**. Le due mutazioni che lo isolano sono quelle che nessun altro test poteva
vedere: che la morte di un client **non contagi** gli altri, e che una gui che si riconnette sia
un **client nuovo** e non erediti l'identificativo del cadavere — altrimenti un messaggio
accodato per il morto verrebbe consegnato al nuovo arrivato, che per I1 nasce **senza stato
proprio** e non potrebbe accorgersene.

⛔ **E una questione resta dichiarata e aperta nel sorgente di `ipc`, sul modello di
`network`.** `accept` non può fallire, e per i due modi di fallire che il vocabolario conosce è
**coerente**: `Disconnected` è un'affermazione **su un `ClientId`**, e `accept` è l'unico metodo
che un `ClientId` **non lo prende**; e non decodifica niente, quindi nemmeno `MalformedMessage`
lo raggiunge. ⚠️ Ma un **ascoltatore** rotto — non un client — oggi **non ha parola** in questo
vocabolario, e arriverebbe come `None`, cioè un **valore sbagliato** invece di un errore
(gotcha #30). ⛔ **Ed è _anche_ un'asimmetria fra le firme:** `receive` restituisce
`Result<Option<Vec<u8>>, IpcError>`, dove «niente di pronto» e «rotto» sono già distinti;
l'argomento contro il `Result` confuta solo `Result<ClientId, IpcError>`, non la forma con
l'`Option` dentro, che il metodo lì sotto **già usa**.

| | |
|---|---|
| **il prezzo vero** | ⛔ aggiungere una terza variante domani **non chiuderebbe niente**: non c'è dove restituirla. Chiudere il residuo costa **la firma**, non l'enum |
| **perché la firma resta** | oggi `IpcError` non ha **nessuna** variante che `accept` possa restituire: un `Result` che non può mai essere `Err` è **superficie morta**, esattamente ciò che questa porta ha appena potato in tre derive e un accessore |

**`memory_journal` — diciotto passate: una di controllo, sedici uccise, una viva e dichiarata.**
⚠️ **Il ritratto è del 2026-08-09 e va letto con la data**: la decisione sul secondo intento ne ha
spostate tre — **M7a**, **M13** e le due nuove **M14** e **M15** — e la tabella che le riconta è
in fondo alla voce chiusa, non qui. Le altre quattordici reggono parola per parola.
⛔ **M0 va letta per prima:** cambia **solo un commento**, e nessun test diventa rosso. Senza di
lei la tabella non prova niente (gotcha #48).

| | Mutazione | Chi la uccide |
|---|---|---|
| M0 | *controllo* — cambia solo un commento | ⛔ **nessuno, ed è il punto** |
| M1 | `intent` risponde `Ok(())` senza scrivere | sei test |
| M2a · M2b | lettura mancante → `Ok(vuoto)` · → `NotDurable` | `a_step_never_written...` · `..._does_not_survive_being_dropped` |
| M3 | `outcome` salta la guardia | `..._without_an_intent_is_refused` · `..._belongs_to_another_step` |
| M4 | `has_intent` ignora **quale** passo | ⛔ **solo** `..._belongs_to_another_step` |
| M5 | `has_intent` ignora il **tipo** di voce | ⛔ **nessuno, e non è una lacuna** — sotto |
| M6a · M6b | `read_back` dà l'**ultimo** (`.rev()` · `.last()`) | `..._the_intent_and_not_the_outcome` · `each_step...` · `a_second_intent...` |
| M7a | gli **intenti** scritti in testa | ⛔ **solo** `a_second_intent...` — ⚠️ **rimisurata il 2026-08-10**, chiudendo la voce del secondo intento: oggi la uccide la **conformità**, promessa 4 |
| M7b | gli **esiti** scritti in testa | `..._the_intent_and_not_the_outcome` · `each_step...` |
| M8 | `read_back` ignora l'argomento `step` | ⛔ **solo** `each_step...` |
| M9a · M9b · M10 | `prune` risponde `Ok` · risponde `NotDurable` · **pota lo stesso** | `prune_refuses_and_leaves_the_record_where_it_was` — ⚠️ **rimisurate il 2026-08-10 col Task 11: ora ne uccidono TRE ciascuna**, perché la conformità pretende la parola esatta (`StepInDoubt`) e non più un `is_err()` |
| M11 | stato **globale di processo** (`static AtomicBool`) | `..._does_not_survive_being_dropped` |
| M12 | `outcome` rifiuta **sempre** | `..._after_its_intent_is_accepted` · `..._the_intent_and_not_the_outcome` · `each_step...` |
| M13 | `intent` **sovrascrive** l'intento già presente | ⛔ **solo** `a_second_intent...` — ⚠️ **irraggiungibile dal 2026-08-10**: la guardia risponde prima. La forma viva è **M14**, sotto |

⛔ **M5 non è uccisa da nessuno, e non va chiusa: distingue uno stato irraggiungibile.** Il
primo record di un passo può essere **solo** un intento, perché `outcome` esige `has_intent`;
quindi «esiste una voce per questo passo» ed «esiste un **intento** per questo passo» sono la
stessa affermazione.

⛔ **E LA PREVISIONE SCRITTA QUI ERA SBAGLIATA, misurata il 2026-08-10.** Diceva:
*«l'equivalenza cade il giorno in cui `prune` rimuoverà voci in modo selettivo — il compito 11
di questo traguardo»*. Il compito 11 è stato eseguito, `prune` rimuove voci in modo selettivo, e
**M5 sopravvive ancora all'intero workspace**. La ragione è precisa e la previsione non l'aveva
vista: `prune` non chiede `has_intent`, chiede **se esiste un ESITO** — una domanda che
`has_intent` non pone e non porrà. L'equivalenza cadrà con `has_outcome`, che nessuno ha ancora
scritto. ⚠️ La frase è **sostituita dalla misura** invece che cancellata: era plausibile, e chi
legge il prossimo `prune` la penserà di nuovo. Gotcha **#15** rivolto a chi scriveva.

✅ **LA QUESTIONE DEL SECONDO INTENTO È CHIUSA IL 2026-08-10, e la voce resta qui invece di
essere cancellata** — una voce aperta che sparisce non si distingue da una dimenticata.

**Com'era.** *Se un secondo intento sullo stesso passo debba essere accettato.* `intent` **non
aveva guardia**: lo accettava in silenzio, e `read_back` rispondeva col **primo** dei due. Non
era una decisione presa, era un comportamento mai interrogato. ⚠️ La misura che l'ha scoperta:
con gli intenti scritti in testa (**M7a**) tutti e nove i test di allora restavano **verdi**, e
il testimone era di **tre chiamate senza nessun esito** — `intent(1,"p0"); intent(1,"p1");
read_back(1)`, che rispondeva `"p0"` e col rovesciamento `"p1"`.

⛔ **La decisione: un secondo intento è RIFIUTATO.** Presa dal **coordinatore in revisione**,
non dal piano, e dichiarata come tale nell'errata perché il proprietario possa **ribaltarla
vedendola**. Le ragioni, in ordine:

| | |
|---|---|
| **il modello** | ADR-0007 dice *«l'intento di **ogni** passo»* — uno per passo. Un secondo è **fuori dal modello**, non un caso da disciplinare |
| **la simmetria** | è la metà mancante di *«un esito senza intento è rifiutato»*: V6 tenuta dalla **porta** invece che dalla diligenza del chiamante. La porta lo faceva già in una direzione sola |
| **YAGNI non si applica** | su una porta dichiarata in anticipo i chiamanti sono vuoti **per costruzione**, quindi il criterio non distingue il morto dal portante — gotcha **#46** |
| **il costo** | una riga adesso — `has_intent` c'era già — contro due implementazioni e un archivio dopo |
| **mai scartata prima** | ⛔ cercato dove fosse già stata valutata (gotcha #32): questa stessa voce diceva *«non è una decisione presa»*. Mai valutata, mai scartata |

⛔ **E perché la sede è la conformità e non `journal.rs`.** La promessa **2** costringe già la
seconda implementazione a conservare **più record per passo**, quindi a chiavare più fine
dell'identità del passo — e con una chiave così *«il primo intento vince»* cade da sé. ⚠️ **Ma
è un accordo per accidente del disegno della chiave, non per contratto:** chi chiavasse **sul
passo**, che è la scelta naturale, divergerebbe dalla finta **senza che nulla diventi rosso**.
È il caso per cui la suite esiste.

⚠️ **Nessuna variante d'errore nuova: `OutOfOrder` si allarga.** La porta dichiara il proprio
tipo d'errore *«deliberatamente povero: un tipo ricco invita il kernel a diramare sulla
ragione»*, e *«un'operazione è arrivata fuori ordine per questo passo»* copre entrambe le
direzioni. Il doc della variante lo dice.

⚠️ **E la decisione ha spostato due mutazioni, misurate di nuovo invece che dedotte** — un
uccisore che sparisce in silenzio è il pericolo vero:

| Mutazione | Chi la uccideva | Chi la uccide **ora** |
|---|---|---|
| **M7a** — gli intenti scritti in testa | ⛔ **solo** `a_second_intent...`, che con la guardia **non può più esistere** in quella forma | ✅ la **conformità**, promessa 4 — l'ordine di scrittura di `replay` **fra** i passi, che allora non esisteva. Tre test. **La mutazione ha cambiato padrone, non è sopravvissuta** |
| **M13** — `intent` sovrascrive l'intento già presente | ⛔ solo `a_second_intent...` | **irraggiungibile**: la guardia risponde `OutOfOrder` prima. La forma viva è **M14**, la guardia **tolta**, uccisa dalla conformità (promessa 6) e da `a_second_intent_on_the_same_step_is_refused` |
| **M15** — la guardia scritta `!entries.is_empty()` invece di `has_intent(step)` | *nuova* | **sei test**: `each_step_reads_back_its_own_first_record` e `an_intent_for_another_step_is_still_accepted` qui, più **cinque** in conformità, che muoiono sul **setup** della promessa 4. ⚠️ **La prima stesura di questa riga diceva «solo la contro-sonda nuova», ed era falsa**: scritta prima della misura, corretta dopo |

**`journal_contract` — nove promesse in dieci blocchi, nove bugiardi, e la corrispondenza è
stata _misurata_ su otto.** ⚠️ **Questa riga diceva «otto promesse, otto bugiardi», ricontate sul
sorgente il 2026-08-10 chiudendo il traguardo:** i messaggi di promessa sono **nove** —
`READ_BACK` · `READ_BACK_IS_THE_INTENT` · `MISSING` · `REPLAY_ORDER` · `OUT_OF_ORDER` ·
`SECOND_INTENT` · `PRUNE_IN_DOUBT` · `PRUNE_RECONCILED` · `NOTE` — e i bugiardi altrettanti.
Gotcha **#31**. ⛔ **E la cifra della campagna NON è stata alzata per simmetria, perché sarebbe
stata un'ipotesi:** la tabella qui sotto è la passata presa **dopo la promessa 8**, quando i
bugiardi erano otto, e la nona — la **7b** — è stata provata non-vacua **in un altro modo**, con
la mutazione `M14b` della campagna di `prune`, che toglie il **blocco intero** invece di
neutralizzare una promessa. Le due misure dicono la stessa cosa con due strumenti, e chi rifà la
passata a nove otterrà **otto verdi** per riga dove qui ne stanno sette.
⛔ Neutralizzando **una** promessa alla volta — avvolgendone i blocchi in `if false` — cade
**esattamente** il test del suo bugiardo e nessun altro, **otto volte su otto**. È la prova che
nessuna promessa è decorativa e che nessun bugiardo muore sulla promessa di un altro.
⚠️ **Rimisurata per intero il 2026-08-10 due volte:** la prima dopo l'arrivo della promessa 6, la
seconda dopo la promessa 8. Un banco che non si rifà quando l'insieme cambia è un banco che
riporta l'esito di ieri. ⚠️ **E la promessa 8 ha DUE blocchi**, non uno — il passo mai aperto sta
in un giornale suo — quindi neutralizzarla ne spegne due; scritto qui perché chi rifà la misura
con uno strumento che cerca un blocco solo otterrebbe un falso «sopravvissuta».

⚠️ **La tabella qui sotto misura la suite contro la FINTA, e dal Task 9 c'è la metà gemella
sull'implementazione VERA:** sta nella sonda **J12**, si rompe `FileJournal` invece di
neutralizzare un blocco, e vale su **tre** promesse diverse più una mutazione di controllo. Le
due misure non si sostituiscono — questa dice che nessuna promessa è decorativa, quella dice che
la suite sta davvero girando contro `redb` e non contro una descrizione di `redb`.

| Promessa neutralizzata | Chi cade | E nessun altro |
|---|---|---|
| 1 · `read_back` rende ciò che `intent` ha scritto | `SilentJournal` (**J2**) | 7 verdi |
| 2 · dopo l'esito, `read_back` rende ancora l'**intento** | `LastWriteWinsJournal` (**J3**) | 7 verdi |
| 3 · un passo mai scritto è `Missing`, non vuoto | `EmptyInsteadOfMissingJournal` (**J4**) | 7 verdi |
| 4 · `replay` in ordine di **scrittura** | `ShuffledJournal` (**J5**) | 7 verdi |
| 5 · un esito **senza** intento è rifiutato | `PermissiveJournal` (**J6**) | 7 verdi |
| 6 · un **secondo intento** sullo stesso passo è rifiutato | `UnguardedIntentJournal` (**J7**) | 7 verdi |
| 7 · un passo **in dubbio** non è potabile | `EagerPruner` (**J8**) | 7 verdi |
| 8 · una **nota** su un passo aperto è conservata e non spodesta l'intento | `DiscardedNoteJournal` (**J10**) | 7 verdi |
| ⚠️ **7b** · un passo **riconciliato** è potabile — *fuori da questa passata, e si dice invece di lasciarla sembrare assente* | `AlwaysInDoubtJournal` (**J13**) | ⛔ misurata dalla mutazione **`M14b`** della campagna di `prune`, che toglie il **blocco intero**: il bugiardo risponde *«THE SUITE IS VACUOUS ON promise 7b»* in **entrambi** i binari. La forma è diversa **per necessità** — gotcha **#55**: le tre asserzioni del blocco portano un messaggio solo, quindi neutralizzarne una lascia il rosso in piedi e non prova niente |

⛔ **E due difetti reali sono stati colti dalla misura, non dalla rilettura** — entrambi nella
suite come il piano la dettava:

| | Il difetto | Come si è visto |
|---|---|---|
| **a** | la promessa **4** confrontava le **sole identità** dei passi, e la sequenza dettata `1, 2, 1` **è un palindromo**: un `replay` rovesciato rende le stesse tre identità nelle stesse tre posizioni | `ShuffledJournal` **passava la suite intera** — «la suite è vacua sulla promessa 4». Chiuso confrontando i **record**, byte compresi: `first, second, third` rovesciato è `third, second, first` |
| **b** | la promessa **1** rileggeva con un `.expect("read_back must find it")`, e la via **A6** è proprio il caso in cui `read_back` **non trova**: la suite scattava con un messaggio che **non nomina nessuna promessa** | `a_journal_that_writes_nothing_is_caught` riportava «ha sparato, ma NON sulla promessa 1» — la suite coglieva A6 e non sapeva dirlo. Chiuso mettendo il messaggio della promessa anche sull'`expect` |

✅ **LA QUESTIONE DELLA PROMESSA 7 È CHIUSA IL 2026-08-10 DAL TASK 11, e la voce resta qui invece
di essere cancellata** — una voce aperta che sparisce non si distingue da una dimenticata.
`prune` ha imparato a rifiutare **un passo in dubbio** invece di rifiutare tutto, la promessa
**7b** è la metà che discrimina, e `AlwaysInDoubtJournal` (**J13**) è il bugiardo che prova che
morde. ⛔ **E lo Step 1 del compito attendeva un ROSSO che non c'era:** la partenza era **verde**
in entrambe, esattamente per la ragione che questo capoverso descriveva. ⛔ **Al suo posto due
voci nuove restano aperte, misurate e non supposte** — la terza risposta di `prune` e la
distinzione di ADR-0018 — e stanno **sotto**.

**Com'era.** ⚠️ **Questo capoverso diceva «la
promessa 6» e il numero era stantio** — `prune` è la **settima** da quando il secondo intento si è
preso la sesta; corretto il 2026-08-10 ricontando sul sorgente, gotcha **#31**. L'asserzione chiede che `prune`
**rifiuti**, e `MemoryJournal` rifiuta **tutto** (decisione D7) — quindi la supera senza mai
consultare se il passo sia in dubbio. Un giornale che rifiutasse ogni potatura a caso è qui
**indistinguibile** da uno che rifiuta *questa* perché è in dubbio: è la famiglia del gotcha
**#30**. ⛔ **Non è stata forzata**, e la ragione è precisa: la metà che discrimina è un passo
**non** in dubbio la cui potatura dev'essere **accettata**, e non si può scrivere finché `prune`
non è implementata da nessuna delle due parti. Arriva col **Task 11** di questo traguardo, dove
`prune` impara a rifiutare *un passo in dubbio* invece di rifiutare tutto. Fino ad allora **J8**
prova che la promessa sa scattare, e nient'altro prova che sappia distinguere. ⚠️ **Anche questo
identificatore era sbagliato e diceva J7**, che è il bugiardo del secondo intento: il potatore
avido è **J8**.

⛔ **VOCE APERTA 1 — la regola di ADR-0018 che ENTRAMBE le implementazioni violano, e che il Task
11 dichiara invece di chiudere** (gotcha **#36**: una nota si legge e si dimentica). ADR-0018
pretende che *«un payload assente e uno mai registrato non siano indistinguibili»*. **Misurato il
2026-08-10, non argomentato:** dopo la potatura, un passo potato e un passo **mai scritto**
rispondono **entrambi `Err(Missing)`** a `read_back`, sono **entrambi assenti** da `replay`, e una
**seconda** `prune` risponde `Err(Missing)` a tutti e due. Indistinguibili in **tre** modi, su
tutte e due. ⛔ **Non è stata chiusa, e la ragione è la decisione D7:** la distinzione piena vuole
l'**impronta** e la **dimensione** che ADR-0018 chiede a un record potato, l'impronta vuole una
funzione di hash, e nel kernel quella è una **voce nuova nella lista di ADR-0031** — un atto
deliberato che nessuna misura ha preparato. ⚠️ **E la via che sembrava non costarla è stata
cercata e MISURATA:** lasciare la voce e svuotare il payload **funziona** — `Ok([])` contro
`Err(Missing)`, distinguibili, conformità verde — ma `steps_in_doubt` risponde allora
**`SuspendAndAsk`** su un passo riconciliato e potato, perché byte vuoti sono **indecifrabili** e
un record indecifrabile rimette il passo in dubbio. Il sistema si fermerebbe su **ogni** passo
potato, **a ogni ripresa**. Una traccia che serva dev'essere leggibile dalla riconciliazione,
cioè una decisione di **formato** — e i byte congelati del Task 10 la rendono un atto deliberato.
**Chi la chiude:** il traguardo che porta la ritenzione, **insieme** alla decisione sull'impronta.
Il limite è scritto anche accanto al codice, in tutte e due le implementazioni e nel blocco 7b.

⚠️ **VOCE APERTA 2 — la terza risposta di `prune` non è tenuta da nessuna promessa.** Le risposte
sono **tre**: `Missing` per un passo mai scritto, `StepInDoubt` per uno aperto, `Ok` per uno
riconciliato. Le promesse **7** e **7b** tengono le ultime due **attraverso entrambe** le
implementazioni; la prima è tenuta **solo** per il doppio in memoria, in
`crates/simulator/tests/memory_journal.rs`. **Misurato:** togliere la guardia `Missing` a
`FileJournal` (mutazione `M10` del Task 11) lascia l'**intero workspace verde**, quindi le due
potrebbero divergere in silenzio. ⛔ **Non è un buco aperto dal Task 11** — prima di lui entrambe
rifiutavano **ogni** potatura con `Missing` — e chiuderlo costa una promessa col proprio bugiardo,
che nessuna misura chiede oggi. **Chi la chiude:** il primo consumatore di `prune`, cioè la
spazzata di ritenzione.

**`reconciliation` — sedici mutazioni più una di controllo, e tre sonde isolate da una propria;
misurate il 2026-08-10, col Task 6.** ⛔ **La data mancava, ed era l'unica campagna del file a non
averla:** le cifre qui sotto — *«nove sonde su nove»*, *«`9 passed`»* — sono vere **di quel
giorno**, e il file ha ora **undici** sonde, le due che il **Task 7** vi ha aggiunto. La campagna
**non è stata rilanciata** su quelle due: si dichiara invece di allinearla, perché una cifra
rialzata per simmetria sarebbe un'ipotesi travestita da misura (gotcha **#15**).
⚠️ Applicazione verificata **per siti**, compilazione in un passo **separato** dall'esecuzione,
conteggio con `--no-fail-fast` (gotcha **#48**). **Sedici su sedici applicate**, nessuna
incompilabile; **nove sonde su nove** muoiono sotto almeno una mutazione, quindi nessuna è vacua.

| Mutazione | Chi cade |
|---|---|
| il ramo `Intent` tronca l'insieme a **1**, e a **0** | il crash a più passi, la classe, l'ordine — e con `0` anche il posto |
| il ramo `Outcome` non toglie nulla, e mette in dubbio invece di togliere | il passo con intento **ed** esito, il crash a più passi, l'ordine |
| `resolution_of` mappa tutto a `RunAgain`, a `SuspendAndAsk`, ad `AskTheWorld` | ⛔ **la prima ne uccide UNA, le altre due ne uccidono DUE**: gli altri test scrivono `Idempotent`, quindi la costante che coincide col valore atteso **nasconde metà del difetto**. È il contro-verso del #48 guadagnato di nuovo |
| il ramo `Err(_)` ignora, e risolve a `RunAgain` | il record indecifrabile, l'insieme, il posto, la gemella dopo l'esito |
| l'insieme **rovesciato** | l'ordine **e** il crash a più passi |
| ✅ l'insieme **ordinato per passo** | ⛔ **solo l'ordine**, e nient'altro: è la prova che la sonda dettata teneva l'ordine **per accidente**. ⚠️ Costa **due file**, perché `StepId` non deriva `Ord` di proposito |
| `enter` spinge sempre, e `enter` toglie-e-rispinge in coda | la prima uccide insieme e posto; ✅ **la seconda solo il posto**, ed è l'alternativa che un lettore sceglierebbe |
| `enter` tiene la **prima** risoluzione invece dell'ultima | insieme e posto |
| ✅ `MemoryJournal::replay` rifiuta un giornale **vuoto**, e ne perde la **prima** voce | ⛔ la prima uccide **solo** il giornale vuoto — ogni altra sonda scrive qualcosa prima, quindi nessuna incontra il primo avvio; la seconda ne uccide quattro |
| **controllo**: cambiata **una parola di un commento** | ✅ **nulla**, `9 passed` |

**`boundary` e l'arm `Note` — nove mutazioni più due di controllo, misurate il 2026-08-10.**
⚠️ Compilazione in un passo **separato** dall'esecuzione e conteggio con `--no-fail-fast` (gotcha
**#48**); ripristino **byte-identico** dei file, fine-riga compresi (vincolo globale 5).

| Mutazione | Chi cade |
|---|---|
| l'arm `Note` letto come un **intento** | **5**: le due sonde della nota, e tre della promozione |
| l'arm `Note` letto come un **esito** | **4** — ⚠️ `a_note_does_not_put_a_step_in_doubt` **non** cade qui, e la ragione è nel suo scenario: finisce con un esito, quindi un `leave` in più non cambia nulla. Delle due sonde della nota **una sola** vede entrambe le direzioni, ed è scritto invece che taciuto |
| `payload` e `reason` **scambiati** — il difetto che il piano dettava | **4**, fra cui quella dell'etichetta e quella del `Debug` |
| `trust: Instruction` invece di `Untrusted` | ⛔ **una sola**, ed è la prova che l'etichetta non poggia su nessun'altra asserzione |
| il record dice `Intent`, scritto comunque con `note()` | **4**, fra cui la sonda dell'accordo |
| il record dice `Outcome`, scritto comunque con `note()` | **4**, le stesse |
| ✅ `promote` scrive con **`outcome()`** invece di `note()` | ⛔ **prima: NESSUNA.** È l'opzione **F** scartata dal proprietario, e nulla la teneva. Dopo `OperationSpy`: **una sola**, la sua |
| `promote` scrive con **`intent()`** | **9** — il rifiuto della porta cade su tutto |
| ✅ `RecordKind::Note` spostata su un indice **libero** (2 → 7) | ⛔ **nulla**, ed era **atteso e dichiarato**: il derive rinumera codifica e decodifica insieme, quindi nessun andata-e-ritorno lo vede. È la stessa misura che il doc di `record_shape.rs` porta per gli altri campi. ✅ **Rimisurata il 2026-08-10 col Task 10: ora è ROSSA**, e la tengono i byte congelati — sonda **F3**, che ripete la stessa rinumerazione su **tutte e otto** le varianti |
| **controllo**: una parola di un commento in `reconcile.rs` | ✅ **nulla** |
| **controllo**: una parola di un commento in `boundary_promotion.rs` | ✅ **nulla** |

**`MemoryJournal::note` — quattro mutazioni.**

| Mutazione | Chi cade |
|---|---|
| la guardia `has_intent` **tolta** | **4**, fra cui la conformità |
| la guardia dimentica **quale** passo (`!entries.is_empty()`) | ⛔ **una sola**, `a_note_is_refused_when_the_intent_belongs_to_another_step` — è il gemello del buco misurato su `outcome`, e senza quella sonda la mutazione sopravvive |
| la nota **scartata** (il bugiardo, messo nel giornale vero) | **8** |
| ⛔ la nota archiviata come `EntryKind::Intent` | ⛔ **NULLA, e la sonda scritta per tenerla è stata TOLTA.** La variante interna è **inosservabile** da fuori: `note` rifiuta un passo senza intento, quindi un passo di sole note non è costruibile attraverso la porta e `has_intent` non può diventare vero per una nota sola. La sonda che lo affermava è stata cancellata e la misura registrata al suo posto — una sonda il cui commento rivendica un difetto che non vede insegna che il difetto è coperto, ed è peggio di un difetto scoperto che qualcuno conosce. Gotcha **#15** |

**`prune` su ENTRAMBE le implementazioni — quindici mutazioni più una di controllo, misurate il
2026-08-10 col Task 11.** Applicazione verificata dal banco stesso (rifiuta se il modello non è
unico nel file), **compilazione in un passo separato** dall'esecuzione, `--no-fail-fast`,
ripristino byte-identico.

| Mutazione | Chi cade |
|---|---|
| `M1` la guardia `Missing` tolta (simulatore) | **2** |
| `M2` *chiuso* = **una voce qualsiasi** (una nota conta) | **4**, conformità compresa |
| `M3` la guardia di ADR-0018 tolta · `M4` rifiuto giusto, **parola sbagliata** | **4** ciascuna |
| ⛔ `M5` risponde `Ok` e **non pota niente** · `M9` idem su `redb` | ⛔ **NESSUNO al primo giro** — la 7b guardava **solo il valore di ritorno**, famiglia del gotcha #30 e specie di **E42**. ✅ Chiusa con *«qualcosa è successo»*: ora **3** e **2** |
| ⛔ `M6` risponde `Ok` e pota **l'INTERO giornale** | ⛔ **NESSUNO al primo giro** — *«pota il passo 5»* era libero di distruggere ogni altro passo. ✅ Chiusa con un passo **spettatore**: ora **3** |
| `M7` qualsiasi voce conta come chiusa (`redb`) · `M8` parola sbagliata (`redb`) | **2** ciascuna |
| ⚠️ `M10` la guardia `Missing` tolta a `redb` | ⚠️ **NESSUNO, e resta dichiarata** — voce aperta 2 qui sopra |
| `M11` un **esito** archiviato come nota | **2** |
| ⛔ `M12` una **nota** archiviata come **esito** | ⛔ **NESSUNO al primo giro**: un passo con intento **e una nota** diventava potabile **mentre è in dubbio**, l'unica cosa che ADR-0018 vieta, e il difetto che il byte del `kind` esiste per impedire. ✅ Chiusa dando alla promessa 7 il caso **intento + nota**: ora **2** |
| ✅ `M13` **la contro-sonda della contro-sonda** — `prune` rifiuta **tutto**, con la parola giusta | **4**, e `the_in_memory_journal_honours_the_contract` muore col `PRUNE_RECONCILED_MESSAGE`. È ciò che rende la promessa 7 non-vacua |
| ⚠️ `M14` la sola asserzione (a) della 7b neutralizzata | ⚠️ **il bugiardo resta VERDE, e giustamente** — la (b) lo coglie con lo stesso messaggio. ⛔ **Ma allora non prova più la non-vacuità**, quindi rifatta come `M14b` |
| ✅ `M14b` il **blocco 7b intero** tolto | `a_journal_that_calls_every_step_in_doubt_is_caught` → **«THE SUITE IS VACUOUS ON promise 7b»**, in **entrambi** i binari |
| la **mutazione di controllo** — un commento in `MemoryJournal::prune` | ⛔ **nessuno, ed è il punto** |

⛔ **Quattro sopravvissute su quindici al primo giro, e tre erano difetti veri** — la lezione è
che una contro-sonda **nasce non provata** (gotcha #45, quarta volta in questo piano) e che
`M14` insegna una forma nuova: **una mutazione che deve far scattare qualcosa può fallire per aver
colpito troppo poco**, e il verde che ne esce si legge come una prova. È il gotcha **#54** girato.

⚠️ **E una sonda non muore MAI da sola, dichiarato invece che taciuto:**
`a_step_is_in_doubt_at_most_once_however_many_records_it_carries` cade solo insieme a
`a_step_that_re_enters_doubt_keeps_the_place_it_first_took`, la cui asserzione confronta il
**vettore intero** e vede quindi anche un doppione. Resta perché porta lo **scenario** — un passo,
due record, il secondo illeggibile — non perché veda un difetto che nessun'altra vede.

✅ **E LA QUESTIONE DELLE DUE VERITÀ È CHIUSA IL 2026-08-10 — DAL PROPRIETARIO — e la voce resta
qui invece di essere cancellata:** una voce aperta che sparisce non si distingue da una
dimenticata.

**Com'era.** *Il `kind` del record e l'operazione della porta sono due verità indipendenti, e
`replay()` ne restituisce una sola.* La riconciliazione si fida del **campo**; il giornale conosce
l'**operazione** — `MemoryJournal` tiene un `EntryKind` interno e `JournalError::OutOfOrder` è
definito sulle due operazioni. Misurate le due direzioni: `intent()` con un record che dice
`Outcome` fa **sparire in silenzio un dubbio vero**; `outcome()` con un record che dice `Intent`
rimette in dubbio un passo concluso.

⛔ **La decisione: `replay()` NON cambia e il `kind` RESTA nel record.** Presa dal **proprietario**,
non dal piano. Le ragioni, in ordine:

| | |
|---|---|
| **è semantica del kernel** | distinguere intento da esito è una decisione del kernel, e spostarla nella porta contraddice il doc di `replay` stesso — *«un'operazione come `steps_in_doubt()` sposterebbe una decisione del kernel dentro chi implementa la porta»* |
| **la forma durevole è del kernel** | ADR-0036, ed è la stessa regola per cui la porta scambia **byte** |
| **il costo evitato** | l'alternativa tocca **porta, conformità e due implementazioni**, e renderebbe **ridondante** il campo che `record.rs` chiama *«quello su cui poggia l'intero protocollo write-ahead»* |

⛔ **E il disaccordo si chiude da CHI SCRIVE, che oggi è una funzione sola.** `Untrusted::promote`
è l'unico codice del kernel che scrive un record, quindi non si costruisce un aiutante per un
chiamante solo: si mette una **sonda** che fissi l'accordo —
`the_promotion_writes_through_note_and_the_record_says_note` — e l'aiutante nasce col **secondo**
scrittore, quando avrà due siti per cui essere giusto.

⚠️ **E la sonda è nata sbagliata, corretta dalla misura e non dalla rilettura.** La prima stesura
asseriva **solo** il `kind` e dichiarava la metà della porta *«tenuta per costruzione, perché
`note` è l'unica operazione che ammette un passo già aperto senza chiuderlo»*. **Falso, e
misurato:** `outcome` lo ammette anch'esso, quindi un `promote` riscritto per chiamare `outcome()`
— cioè l'**opzione F**, quella che il proprietario ha esaminato e **scartato** — lasciava
**l'intero workspace verde**. Chiusa con una spia conforme, `OperationSpy`, che delega tutto a
`MemoryJournal` e in più annota **quale metodo** è stato chiamato: ora quella mutazione uccide
**una sola** sonda, la sua.

⛔ **Ciò che NON è comprato, detto per intero:** nessuna regola di livello 1 impedisce a uno
scrittore futuro di chiamare `outcome()` con un record che dice `Intent`. La sonda copre
**l'unico scrittore che esiste**. È una voce **chiusa come decisione**, non come garanzia.

⚠️ **E una riga di questo registro va riletta quando quella questione si chiude, non prima:**
la voce di `V5` fra le scoperte dice *«il tipo `EffectClass` esiste ma nessun caso lo esercita»*.
Dal Task 6 la **riconciliazione** lo esercita su tutti e tre i valori, e tratta un record senza
classe leggibile come `Unrepeatable`, che è la frase di ADR-0007. ⛔ **Ma la riga di catalogo di
`V5` è di livello 1** — *«un effetto senza classe dichiarata non è esprimibile»* — e un test di
comportamento **non la copre**: sono due proprietà diverse con lo stesso nome. Scritto qui perché
chi riconta non muova il numeratore per la ragione sbagliata, e perché spostarlo richiederebbe il
**catalogo §7.4**, che è una modifica alla spec che nessuno ha deciso (vincolo globale 7).

⚠️ **Il banco di misura ha prodotto _nove_ esiti credibili e falsi in due sessioni** — gotcha
**#48**, col testo integrale in [`HANDOFF.md`](HANDOFF.md). ⛔ Gli ultimi due sono della revisione
di oggi e portano una forma nuova: **si erano corretti due strumenti gemelli, ma uno solo** — il
gemello non corretto ha riscritto di nuovo i fine-riga alla corsa successiva — e una
**rinomina** ha reso stantie due ancore di mutazione, che sono tornate «zero siti» invece che un
esito. 📌 Una cosa sola resta qui, perché
non è un inganno del banco ma un **fatto del repository**: i fine-riga sono **misti per file**
(`process.rs` e `ports_are_implementable.rs` in CRLF, `mod.rs` e `network.rs` in LF). Non c'è
una convenzione da seguire, c'è **un file da non cambiare** — uno strumento che ne riscrive uno
intero in LF produce un `git diff` di seicento righe che nessuno ha toccato.

#### `CrashingJournal` — cinque mutazioni, cinque uccise, e una coppia che prova di essere due difetti

**Misurate il 2026-08-11, chiudendo il Task 1 del Traguardo 4.** ⚠️ **Rifatte da zero dopo
un'interruzione:** la prima passata era stata eseguita e il suo verbale è andato perso, e un
commit senza il proprio verbale è un numero senza misura — rifarle costa meno che fidarsene.
Applicazione provata con `grep` **dopo** l'edit, compilazione in un passo **separato**
dall'esecuzione, ripristino con `git checkout --` e albero verificato vuoto fra una e l'altra
(gotcha **#48**).

| | Mutazione | Chi la uccide |
|---|---|---|
| A | `may_write` risponde sempre `true` — non cade mai | **sei** sonde. ⚠️ Il piano ne elencava **quattro**: il criterio enumerava nomi e l'insieme è cresciuto sotto di lui |
| B | `may_write` risponde sempre `false` — cade sempre | sei sonde, e la sola che conta è `a_journal_told_not_to_crash_never_falls` |
| C | il contatore avanza **anche su una scrittura rifiutata** | ⛔ **solo** `a_write_the_protocol_refuses_does_not_consume_a_crash_position`, riga 127 |
| D | la guardia di `prune` **tolta** | ⛔ solo `after_the_fall_pruning_is_refused_too_but_it_never_causes_the_fall`, riga **82** |
| D′ | la guardia di `prune` passa da `may_write()` invece che da `fallen` | ⛔ la **stessa** sonda, riga **68** |

⛔ **A e B insieme sono la prova in due direzioni, e senza la coppia una delle due sonde
sembrerebbe copertura senza esserlo.** `a_journal_told_not_to_crash_never_falls` è **verde
sotto A e rossa sotto B**: è l'unica in quella direzione, ed è quella su cui `C7a` poggerà. Il
suo specchio esatto è `after_the_fall_every_later_write_is_refused_too`, **rossa sotto A e verde
sotto B** — asserisce solo rifiuti, quindi un giornale che rifiuta sempre la soddisfa. Entrambe
sono sonde **a senso unico**, e ciascuna è la controparte dell'altra.

⛔ **D e D′ uccidono asserzioni DIVERSE, ed è la sola cosa che quella coppia esisteva per
scoprire.** Righe 82 e 68, con due messaggi di panico distinti: la sonda **distingue** *«un
processo morto pota ancora»* da *«la potatura arma la caduta»*, quindi il gotcha **#55** — più
asserzioni che condividono un esito, e una mutazione mirata a una colta da un'altra — **non si
applica qui**. ⚠️ **E una divergenza dall'atteso, registrata invece che appianata:** D′ era
attesa uccidere la riga 69 o le 70-74, e uccide la **68** — al momento della prima potatura il
contatore è già pari al punto di caduta, quindi `may_write()` rifiuta **in tronco** e il test
muore prima di poter osservare l'armamento. L'armamento avviene lo stesso; nessuna asserzione
arriva a vederlo. L'esito è **più forte** dell'atteso, e diverso.

⚠️ **Una seconda divergenza, sul modo in cui D uccide.** Il panico è `Err(Missing)` contro
`Err(NotDurable)` e non `Ok(())` contro `Err(NotDurable)`: la seconda potatura raggiunge il
giornale interno, che risponde *«manca»* perché la prima l'aveva già rimosso. L'uccisione resta
valida — l'asserzione confronta la **variante** e non l'esito, che è la lezione del gotcha
**#30** — ma il messaggio non dice *«la potatura è riuscita»*.

⛔ **E la non-vacuità della sonda nuova è provata togliendo IL BLOCCO, non un'asserzione**
(gotcha **#55** al rovescio). Con la mutazione C attiva e il corpo di
`a_write_the_protocol_refuses_does_not_consume_a_crash_position` racchiuso in un commento a
blocco, il banco torna **verde**: le altre nove sonde sono **cieche** a C, e la frase *«the
counter moves only on an `Ok`»* la tiene lei sola.

📌 **E il conteggio delle posizioni di caduta è stato verificato per una via che non passa da
Rust**, perché la cifra scritta nel commento della sonda era stata **citata** da una revisione
prima che qualcuno la rimisurasse (gotcha **#53**). Ricalcolando xorshift64 fuori dal
repository, sui cinquecento semi e con otto scritture, le **otto** posizioni escono tutte:
`{61, 69, 66, 62, 63, 57, 58, 64}`, minimo **57** e massimo **69**. La cifra era vera.

#### La campagna di livello 1 — quattro mutazioni, e una riga nuova che si è messa davanti alle altre

**Misurate il 2026-08-11, chiudendo il Task 2 del Traguardo 4.** Applicazione provata dopo
l'edit, compilazione in un passo **separato** dall'esecuzione, ripristino verificato col
`git diff` contro il commit precedente e non a `grep` (gotcha **#48**).

| | Mutazione | Sonda rossa | Asserzione che ha sparato |
|---|---|---|---|
| A | lo scenario smette di scrivere l'**esito** | **entrambe** | il pin su `writes_done` · `c7a` sull'oracolo nuovo — `left: 12, right: 24` |
| C | il giornale cade alla scrittura **zero** | solo `c7a` | ⛔ **l'oracolo nuovo**, *«nothing was written to be in doubt about»* — `left: 0, right: 24` |
| D | il giornale cade alla scrittura **cinque** | solo `c7a` | l'oracolo nuovo — `left: 5, right: 24` |
| E | l'esito **si scrive**, ma il record dice `Intent` | solo `c7a` | ⛔ **l'asserzione dell'insieme**, dodici passi in dubbio, tutti `RunAgain` |

⛔ **La mutazione E non era prevista, e senza di lei la passata avrebbe concluso il falso.** Le
mutazioni C e D uccidono **la stessa** asserzione — quella nuova — e la lettura ovvia è il
gotcha **#55**: due difetti che una sonda non distingue. ⚠️ **La lettura ovvia è sbagliata**, e a
dirlo è E: la riga nuova non oscura quella dell'insieme, la **domina** su un asse diverso.
Sono due mansioni disgiunte — la nuova giudica **lo scenario** (*ha scritto?*), quella
dell'insieme giudica **la riconciliazione** (*è d'accordo?*) — e la seconda è raggiungibile
esattamente quando lo scenario è intatto, che è il soggetto dichiarato di `C7a`. L'ordine delle
due è quello giusto. 📌 **La regola che ne esce:** quando due mutazioni uccidono la stessa
asserzione, prima di concludere che la sonda non distingue si cerca **una terza mutazione che
lasci passare la prima asserzione**. Se esiste, le due non erano in competizione.

⚠️ **E una regressione di copertura, registrata invece che taciuta:** prima dell'oracolo nuovo,
la mutazione A uccideva `c7a` **sull'insieme**, elencando dodici dubbi; ora la uccide sulla riga
nuova. Nessun rosso è andato perso, ma il **punto d'impatto si è spostato** — è il gotcha #48
nella forma in cui una riga aggiunta si mette davanti, e la sola ragione per cui si è visto è
che si è rilanciata la campagna dopo una rifinitura invece che solo dopo un cambiamento di
comportamento.

✅ **E il controfattuale sequenziale è stato costruito e misurato, non argomentato**, perché la
decisione di **non** fissare l'interlacciamento in questo file dipende da esso: commentando la
sospensione — così che ogni attività finisca al primo poll — il massimo insieme in dubbio scende
da **tre** a **uno** su tutti e cinquanta i semi. Quindi la sonda del compito successivo va
davvero rossa su uno scenario sequenziale, e una sonda in più qui sarebbe stata duplicazione
(gotcha **#49**).

#### `C7b` — sei mutazioni, e due esistono perché la non-vacuità dichiarata non era quella vera

**Misurate il 2026-08-11, chiudendo il Task 3 del Traguardo 4.** Applicazione provata dopo
l'edit, compilazione in un passo **separato**, ripristino verificato e verde riconfermato fra una
mutazione e l'altra (gotcha **#48**).

| | Mutazione | Sonda rossa | Asserzione che ha sparato |
|---|---|---|---|
| A | `RecordKind::Intent => {}` in `reconcile.rs` — la riconciliazione non riporta niente | `C7b` e la sonda dell'interlacciamento | l'**insieme**, `left: []` contro `right: [3]` |
| B | `resolution_of` risponde sempre `SuspendAndAsk` | solo `C7b` | ⛔ **la risoluzione**, `SuspendAndAsk` contro `RunAgain` |
| C | nessun crash, su **entrambi** i siti di chiamata | entrambe | il conteggio, `left: 0` contro `right: 200` |
| D | `expected_doubt` non chiude più il dubbio sull'esito | solo `C7b` | l'**insieme**, `left: [3]` dall'archivio contro dodici passi dalla traccia |
| E | il giornale cade alla scrittura **zero** su ogni seme | `C7b` e la fratella | ⛔ **la non-vacuità nuova**, non il conteggio |
| F | `ACTIVITIES = 1` — nessun interlacciamento | `C7b` e la fratella | ⛔ **la non-vacuità nuova** |

⛔ **Le mutazioni E ed F sono la ragione per cui `C7b` ha un secondo oracolo, e il difetto che
hanno scoperto era stato chiuso un compito prima su `C7a`.** L'asserzione sul conteggio dei crash
prova che **il guasto è scattato**; non prova che la **riconciliazione sia stata esercitata**. Con
un giornale che cade al primo byte, duecento semi su duecento cadono davvero **e duecento
confronti su duecento sono `[] == []`**: la campagna è verde e non ha verificato nulla. Con una
sola attività, ogni crash lascia al più un passo. ⚠️ E già senza mutazioni **sei semi su
duecento** confrontano due insiemi vuoti sui propri meriti. 📌 **La forma generale, che è la
notizia:** *«l'iniezione è avvenuta»* e *«c'era qualcosa da verificare»* sono **due** affermazioni,
e un solo oracolo ne copre una. Una campagna ne vuole due.

⛔ **E il discrimine è stato provato, non assunto:** sotto E ed F spara la **riga nuova** e non
quella del conteggio, che è ciò che dice che la riga nuova difende quello per cui è nata. Sotto C
— nessun crash — spara ancora **il conteggio**, ed è la risposta giusta: senza guasto non c'è
dubbio, e il difetto è che l'iniezione non è arrivata.

⚠️ **Una previsione del coordinatore smentita dalla misura, registrata:** la mutazione F era
attesa produrre collaterali sul pin e su `C7a`, perché `WRITES_PER_RUN` scende da ventiquattro a
otto. Non ne ha prodotti — la costante è **derivata**, quindi si ricalcola e le asserzioni
confrontano otto con otto. È il caso che il commento del pin dichiara: *«fissa una relazione, non
un numero»*.

📌 **E le righe si sono mosse senza che le asserzioni si muovessero**, ricontrollato rilanciando
A, B, C e D dopo le correzioni: nessuna è finita davanti a un'asserzione nuova. È il contro-verso
del difetto che il Task 2 aveva registrato, dove una riga aggiunta si era messa davanti.

#### I byte congelati — tredici mutazioni, e due dettate dal piano non compilavano

**Misurate il 2026-08-10, chiudendo il Task 10.** ⚠️ Applicazione provata **per byte**,
compilazione in un passo **separato** dall'esecuzione, ripristino **byte-identico** dei sorgenti
(gotcha **#48**, vincolo globale 5). ⛔ **L'oggetto è l'unico artefatto del progetto che non si
corregge:** se i byte cambiano non è un aggiornamento, è un cambio di formato.

| Mutazione | Chi cade (su **6** sonde) |
|---|---|
| ⛔ **dettata dal piano** — `payload` `#[n(3)]` → `#[n(2)]`, già di `trust` | ⛔ **non compila**: `duplicate index numbers`. Il controllo non si sarebbe mai visto scattare |
| ✅ `kind` 0 ↔ `effect` 1, scambio fra indici esistenti | **3** — encode, decode, e la sonda degli offset |
| ✅ `payload` 3 → 7, indice libero | **3** — le stesse |
| ⛔ **le otto varianti rinumerate una per una** su un indice libero | **2 ciascuna** — encode e decode — e ⛔ **otto volte su otto**. È la sonda **F3**, e il motivo per cui i record congelati sono tre |
| ⛔ **dettata dal piano** — campo facoltativo a `#[n(4)]`, già di `reason` | ⛔ **non compila**, stesso errore. ⚠️ E il piano avrebbe letto quel rosso come *«ADR-0036 smentito dalla misura»*: un errore di compilazione scambiato per una scoperta sull'architettura |
| ✅ campo facoltativo a `#[n(5)]`, indice libero, con `#[cbor(default)]` | ✅ **nessuna**, ed è la risposta voluta: i byte congelati non si muovono |
| ⛔ lo stesso campo valorizzato — `Some(9)` invece di `None` | **21 → 22 byte**, `85` → `86` e `09` in fondo: il campo **arriva sul filo**, quindi il verde qui sopra significa qualcosa. Senza questa misura sarebbe stato vacuo — gotcha **#54** |
| una **variante nuova** di `RecordKind` (`#[n(3)] Amend`) | ⛔ **la LIBRERIA non compila** — `E0004` in `crate::reconcile` — quindi il livello 1 arriva **prima** dell'oracolo e le sonde non lo vedono mai. Misurato invece che dedotto |
| **controllo**: una parola di un commento di `record.rs` | ✅ **nulla**, `6 passed` |

📌 **E la mappa non è prosa.** `record_v1.map` è **riletta** dal banco: le colonne `offset` e
`hex` di ogni riga devono ricostruire il `.cbor` byte per byte, quindi un `<fill in>` non si
analizza e un byte trascritto male non corrisponde — il gotcha **#43** chiuso da un controllo
invece che da un avviso. ⚠️ **La colonna di prosa è dichiarata NON verificata dentro la mappa
stessa:** un documento controllato a metà che non dice quale metà è peggio di uno che nessuno
controlla.

#### Due moduli di test vivono in `src/`

📌 **Due moduli di test vivono in `src/` invece che in `tests/`, e la deviazione è dichiarata
in entrambi i file.** Non è una scorciatoia: in un caso non è nemmeno una scelta.

| Dove | Che cosa difende, e perché sta in `src/` |
|---|---|
| `crates/daemon/src/main.rs` (un test) | che il **grafo di produzione si monti e giri**: il cablaggio, non il dimensionamento del limite di turni — misurato in due direzioni, con il limite a `0` il test resta **verde** perché senza attività il corpo non gira mai. Sta in `src/` perché la funzione sotto test è **privata in un target `bin`**, e nessun test d'integrazione può linkare un binario |
| `crates/platform/src/rng.rs` (cinque test) | `SequentialRng`: che le estrazioni siano 0, 1, 2 e così via, che `new` e `default` siano **lo stesso** generatore, che `below` percorra gli indici a turno **a limite costante**, il limite dichiarato quando il limite **cambia**, e che il contatore **avvolga invece di traboccare**. Sta in `src/` perché l'ultimo costruisce `SequentialRng(u64::MAX)` **col campo privato**, irraggiungibile da una crate a parte senza 2^64 estrazioni o un costruttore che esisterebbe solo per il test |

## Livello 3 — vuoto, e non è una svista

`clippy` gira come igiene del codice ma **non ha voce nella porta**: nessun V dipende da
lui. Un rosso della porta deve significare sempre «invariante violata», mai «stile
discutibile», o si impara a ignorarlo.

⛔ **`cargo fmt --all --check` è stato valutato il 2026-08-09, e resta fuori per lo stesso
argomento.** La deriva era reale — **una sola** in tutto il workspace, il letterale di struct
di `Parameters::new`, ereditata dal Task 3 — ed è stata corretta **nel file** (commit
`c1aebcc`) invece che sorvegliata dalla porta. La regola 1 di §7.1.1 la rifiuta su
**entrambi** i rami, ed è la stessa coppia di domande con cui quella sezione caccia `clippy`:

| Ramo | La domanda | Su `rustfmt` |
|---|---|---|
| **1a** | difende un `V`, un'`I` o un `Q` nominato? | **no** — nessuna proprietà del sistema dipende da dove va a capo un letterale di struct |
| **1b** | quale riga del catalogo smette di essere **vera**, se lo cancelli? | **nessuna** — stesso esito che su `clippy`, e la §7.4.3 regge parola per parola |

⚠️ **E non sarebbe comunque una decisione da prendere qui.** *«Il livello 3 del catalogo è
vuoto»* sta fra le voci **non rilitigabili** della §7 del [compendio](COMPENDIO.md):
aggiungere un passo di stile alla porta richiederebbe un **ADR che superi la §7.4.3**, non un
commit. ⚠️ `rustfmt` non è nemmeno un lint, ma sulla scala di §7.1.2 si comporta da livello
**3**: si cancella, e si aggira per singolo elemento con una riga — `#[rustfmt::skip]`.

📌 **Il costo accettato, ed è reale:** `cargo fmt --all --check` resta un segnale che
**nessuno fa rispettare**. Oggi è verde su tutto il workspace, e a tenerlo tale è l'igiene di
chi scrive — non un controllo. Chi lo trova rosso lo corregge nel file, come qui.

## Cosa la porta NON controlla, in questo traguardo

Righe del catalogo §7.4 che oggi **nessun file implementa** — o che lo sono **in parte**, e
allora la riga dice **quale** parte. Stanno qui perché un registro che le omettesse lascerebbe
credere che siano coperte, e una riga che dicesse «scoperta» dove qualcosa c'è mentirebbe
nell'altro verso.

⚠️ **Riletta riga per riga il 2026-08-10, chiudendo il Traguardo 3, contro la condizione 11
della Definizione di «fatto» del piano** — *«dice cosa è coperto **e** cosa non lo è, con il
traguardo che lo chiude»*. **Dieci righe, e otto nominano il proprio.** Le **due** che non lo
fanno si dichiarano qui invece di lasciarle sembrare sviste: i **due residui di
`SystemReactor`** non nominano nessuno **perché nessuno li chiude con questi mezzi** — la
conformità non può coglierli e distinguerli sulla vera vorrebbe una garanzia che nessuna
piattaforma offre, quindi il «chi» è **vuoto per misura**, non per dimenticanza; il
**portachiavi** invece non ha un chiudente scritto da nessuna parte, ed è una **lacuna del
registro** e non della porta — nominarne uno adesso sarebbe inventarlo, quindi resta scritto
che manca.

| Riga del catalogo | Perché non c'è ancora |
|---|---|
| il **resto** del blocco **B** di §7.4.1 — i **gettoni** | ⚠️ **Non più interamente scoperto, dal 2026-08-09:** **una riga su cinque** è implementata — `promuovere testo a istruzione ← la porta journal` (V19), da `crates/kernel/tests/compile_fail/promote_without_journal.rs`, che nomina quella riga di catalogo nella propria intestazione. Degli **altri quattro**, **due** li emettono l'arbitro (§5.6) e il filtro dei vincoli (§6.3) — **Traguardi 5 e 6**; gli altri **due**, il `Worker` e la **ricevuta**, li emette già `crates/kernel/src/ports/process.rs`, e restano scoperti per la ragione della riga di §6.10.5 più sotto: senza `Grant` non si ottiene un `Worker`. ⚠️ **Corretto il 2026-08-10:** diceva che li emettevano **tutti e quattro** l'arbitro e il filtro dei vincoli, e per due era falso — `Process::start` restituisce il `Worker` e `instruct_one`/`instruct_stream` le ricevute, tutti e tre spediti da questo traguardo. ⛔ Un costruttore di `Grant` dietro una feature di test **è stato valutato e scartato**: creerebbe il secondo modo di ottenere una concessione che §5.6 esiste per togliere dal compilatore |
| il resto del blocco **C** di §7.4.1 | **nove righe su diciannove** sono implementate (sopra). ⚠️ **Ricontate una seconda volta il 2026-08-10**, eseguendo il Task 2 del Traguardo 3: diceva *«sette su diciotto»*, e **sbagliava di due nel numeratore, non di uno** — il Task 1 aveva consegnato `record_without_version.rs` senza scriverne la riga qui, e il Task 2 ne ha aggiunta un'altra insieme alla propria riga di catalogo. ⛔ **È la stessa specie di prima e va detta così:** il denominatore lo muove chi tocca il catalogo e se ne accorge; **il numeratore lo muove chi scrive un caso**, che il catalogo non lo apre nemmeno. Delle due, la seconda è quella che invecchia in silenzio. ⚠️ **Ricontate il 2026-08-10:** diceva *«sei su diciassette»*, sbagliato in **entrambi** i termini — e prima ancora *«tre su sedici»*. Il numero giusto **esisteva già** in testa alla sezione «Livello 1»: era stato rimisurato e scritto in **uno solo dei due posti dello stesso file**, e il denominatore era rimasto indietro perché la riga della **regola B** entrò nel catalogo lo stesso giorno. Le **altre dieci** non cambiano — i due termini erano bassi di uno insieme — e nominano tipi che nascono coi **Traguardi 3, 5 e 6**. ⚠️ **E la loro descrizione era stretta, corretta lo stesso giorno:** diceva *«nominano tipi dell'arbitro, del giornale e del canale worker»*, e **due delle dieci non vi rientrano** — `V5`, un effetto senza classe dichiarata, e `V10`, un sensore che modifica l'artefatto. Chi le ricontasse **dalla descrizione** ne troverebbe otto: è lo stesso difetto della riga qui accanto un livello più sotto, e si ricontano **sul catalogo**, che è l'unico posto che le enumera. ⚠️ **E `V5` merita una parola, perché il Traguardo 3 l'ha resa ingannevole:** il tipo `EffectClass` **esiste** da `crates/kernel/src/record.rs` ed è un campo obbligatorio del record, ma **nessun caso lo esercita** — un tipo che esiste non è un controllo che scatta, e la riga resta fra le scoperte |
| i test di contratto per le **altre quattro** famiglie di porte | ✅ **`reactor` è coperta** dal Task 7 del Traguardo 2 — sonde R1…R6. ⚠️ **Ricontate il 2026-08-10:** questa riga diceva *«le altre **cinque**»* e contava `journal` fra le scoperte, e dal Task 4/5 di questo traguardo non lo è più — `crates/kernel/tests/journal_contract.rs`, sonde **J1…J8**. ⛔ **Ma è coperta a metà, e la metà va detta:** una suite di conformità vale la prova che **due** implementazioni rispondono lo stesso. ⚠️ **Ricontata il 2026-08-10 col Task 8:** la seconda implementazione **esiste** — `platform::journal::FileJournal` su `redb` — e la suite le gira contro **verde**, misurato con un file usa-e-getta; ma il file che la esegue **dentro** il repository era il **Task 9**, quindi finché non c'era, ciò che il repository comprava era la via **A6** di `boundary.rs` e otto promesse **scritte in una copia sola**, non l'accordo fra due. ⛔ *«Misurato una volta»* non è *«tenuto a ogni commit»*, ed è esattamente la differenza che questo registro esiste per non lasciar sfumare. ✅ **Ricontata una terza volta il 2026-08-10, col Task 9, e la mezza copertura è CHIUSA:** `crates/platform/tests/journal_contract_real.rs` esiste, e l'accordo fra le due implementazioni è ora **tenuto a ogni commit** — sonda **J12**, con tre contro-sonde che la provano non vacua su **tre promesse diverse** e una mutazione di controllo che non muove nulla. ✅ **Ricontata una QUARTA volta il 2026-08-10, col Task 11, e anche la promessa 7 è chiusa:** questa riga diceva *«ciò che resta scoperto di `journal` non è più il numero delle implementazioni ma la promessa 7, che entrambe soddisfano per la ragione sbagliata finché `prune` non è implementata»* — `prune` è implementata su entrambe, la promessa **7b** è la metà che discrimina, e le sonde sono **J1…J13**. ⚠️ **Ciò che resta scoperto è ora dichiarato in due voci aperte** — la distinzione di ADR-0018 fra potato e mai registrato, e la terza risposta di `prune` — non più in una promessa che passa senza guardare. Restano scoperte `filesystem`, `network`, `process` e `ipc`: le loro implementazioni nascono coi **Traguardi 5 e 6**, e la suite di ciascuna nasce con esse |
| **due residui dichiarati dentro `SystemReactor`**, e stanno qui perché un registro che li tacesse mentirebbe | ⛔ sostituire `Some(self.now())` con `Some(deadline)` in `wait_until` **non fa scattare nulla**: la conformità non può coglierlo perché **sulla finta le due espressioni coincidono**, e distinguerle sulla vera richiederebbe l'overshoot dello sleep del sistema operativo, che nessuna piattaforma garantisce — un controllo verde per fortuna e rosso per sfortuna, cioè peggio di uno assente (gotcha #24). ⚠️ E `R5` prova che `wall_time()` non è **ferma**, non che sia **esatta**: l'esattezza vorrebbe una seconda sorgente di tempo |
| ~~i **byte congelati** del record durevole~~ ✅ **CHIUSA il 2026-08-10, col Task 10** | ⚠️ **Resta scritta perché la storia della riga è il dato.** Diceva prima *«non esiste ancora nessun record»* — falso dal Task 1 — e poi *«a non esistere è `crates/kernel/tests/frozen_bytes.rs`, e finché non esiste le tre regole sugli indici di §4.9.2 sono una convenzione e non un controllo»*, che era vero e ora non lo è: il file esiste, congela **tre** record, e le **otto** varianti sono state rinumerate una per una con **otto rossi su otto** — sonde **F1…F6** nel livello 2. ⛔ **Ciò che resta scoperto è UNA sola cosa, e va detta:** l'oracolo è un file che qualcuno può riscrivere, e la difesa è che la riscrittura **si legge nel diff** — non che sia impossibile. È la stessa forza e la stessa debolezza del gotcha **#25**, dichiarata da ADR-0036 fin dall'inizio. Vincolo 14 della §11 del [compendio](COMPENDIO.md) |
| la **campagna DST**, e l'elenco versionato dei **semi** di V31 | ⚠️ **Il soggetto era sbagliato, corretto il 2026-08-09:** diceva *«non esiste ancora il simulatore»*, e la crate `simulator` **esiste** e spedisce `SeededRng` e `VirtualReactor` — `crates/kernel/tests/executor_determinism.rs` gira già **C1 e C2** su di essi. A non esistere è la **campagna**: molti semi, guasti iniettati, e l'elenco versionato dei semi. Traguardo 4 |
| i **byte consumati** pari alla lunghezza dichiarata dal frame | non esiste ancora il canale verso i worker. Traguardo 6 |
| le **righe 1–4 di §6.10.5** — i casi negativi della porta `process` | ⛔ **Scaglionate, e la ragione è strutturale e non di fretta.** Tutte e quattro pretendono di **ottenere** un `Worker`; un `Worker` lo restituisce solo `start(grant, ..)`; e nessuno emette concessioni prima del **Traguardo 5**. Scriverle oggi darebbe quattro casi che falliscono perché manca la `Grant`, cioè **verdi per il motivo sbagliato** (gotcha #24) — una regola provata in una direzione sola non è ammissibile (§7.1.1 regola 3). ⚠️ Un costruttore di `Grant` dietro una feature di test resta **scartato** per la ragione già scritta nella riga del blocco **B** qui sopra. Quel che le sostituisce intanto è `ports_are_implementable.rs`, che le firme le esercita **in entrambe le direzioni** con una finta costruita direttamente dal test |
| solo `secrets` raggiunge il **portachiavi** | nessuno script lo verifica oggi: `gate-deps.sh` guarda i grafi di `kernel` e `simulator`, non quelli di `platform` e `secrets` |
| un solo **punto di uscita verso la rete** | la lista delle crate autorizzate è **vuota**, e una lista vuota passa sempre. Il catalogo lo dichiara già: è l'unica voce provata in una direzione sola, e si completa nel sotto-progetto che accende la rete |
