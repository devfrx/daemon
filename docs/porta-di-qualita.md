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
| **blocco C** · `Q9 · I6 · V20 · §4.9` — un payload non fidato **senza la propria etichetta** | `crates/kernel/src/record.rs` — `RecordV1::trust` non è un `Option`, non porta `#[cbor(default)]`, e `Trust` non implementa `Default` | `record_without_trust_label.rs` |
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
| un payload non fidato senza la propria etichetta | **`error`** | no |

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
| `crates/kernel/tests/boundary_promotion.rs` | **blocco C** · `Q9 · I6 · V20`, **entrambe** — la promozione dichiarata è la contro-sonda della regola A e della regola B — **e blocco B** · `V19` | otto test |
| `crates/kernel/tests/parameters_delivered.rs` | le **due** righe **blocco C** · `V29 · §2.8 · ADR-0034` | quattro test |
| `crates/kernel/tests/record_shape.rs` | **blocco C** · `Q14 · §4.9` **e** `Q9 · I6 · V20 · §4.9` — la contro-sonda dell'etichetta è `every_trust_label_survives_the_round_trip_and_the_two_differ_in_the_bytes`, che scrive **entrambi** i valori e ne confronta i byte | dieci test |

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
cartella `crates/kernel/tests/compile_fail/` ne contiene oggi **quattordici** in tutto —
**quattro** dal Traguardo 1 e **dieci** dal Traguardo 2.

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
| **R6** | ⛔ **la contro-sonda che conta, e la si sarebbe dimenticata:** rompendo l'avanzamento dell'orologio di parete del `VirtualReactor`, la conformità **resta verde** e scatta **solo** `crates/simulator/tests/virtual_clock.rs`. È la prova che la suite condivisa non impone alla vera un comportamento della finta — se lo facesse, renderebbe rossa un'implementazione **corretta**. Gotcha **#44** |

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

| Riga del catalogo | Perché non c'è ancora |
|---|---|
| il **resto** del blocco **B** di §7.4.1 — i **gettoni** | ⚠️ **Non più interamente scoperto, dal 2026-08-09:** **una riga su cinque** è implementata — `promuovere testo a istruzione ← la porta journal` (V19), da `crates/kernel/tests/compile_fail/promote_without_journal.rs`, che nomina quella riga di catalogo nella propria intestazione. Degli **altri quattro**, **due** li emettono l'arbitro (§5.6) e il filtro dei vincoli (§6.3) — **Traguardi 5 e 6**; gli altri **due**, il `Worker` e la **ricevuta**, li emette già `crates/kernel/src/ports/process.rs`, e restano scoperti per la ragione della riga di §6.10.5 più sotto: senza `Grant` non si ottiene un `Worker`. ⚠️ **Corretto il 2026-08-10:** diceva che li emettevano **tutti e quattro** l'arbitro e il filtro dei vincoli, e per due era falso — `Process::start` restituisce il `Worker` e `instruct_one`/`instruct_stream` le ricevute, tutti e tre spediti da questo traguardo. ⛔ Un costruttore di `Grant` dietro una feature di test **è stato valutato e scartato**: creerebbe il secondo modo di ottenere una concessione che §5.6 esiste per togliere dal compilatore |
| il resto del blocco **C** di §7.4.1 | **nove righe su diciannove** sono implementate (sopra). ⚠️ **Ricontate una seconda volta il 2026-08-10**, eseguendo il Task 2 del Traguardo 3: diceva *«sette su diciotto»*, e **sbagliava di due nel numeratore, non di uno** — il Task 1 aveva consegnato `record_without_version.rs` senza scriverne la riga qui, e il Task 2 ne ha aggiunta un'altra insieme alla propria riga di catalogo. ⛔ **È la stessa specie di prima e va detta così:** il denominatore lo muove chi tocca il catalogo e se ne accorge; **il numeratore lo muove chi scrive un caso**, che il catalogo non lo apre nemmeno. Delle due, la seconda è quella che invecchia in silenzio. ⚠️ **Ricontate il 2026-08-10:** diceva *«sei su diciassette»*, sbagliato in **entrambi** i termini — e prima ancora *«tre su sedici»*. Il numero giusto **esisteva già** in testa alla sezione «Livello 1»: era stato rimisurato e scritto in **uno solo dei due posti dello stesso file**, e il denominatore era rimasto indietro perché la riga della **regola B** entrò nel catalogo lo stesso giorno. Le **altre dieci** non cambiano — i due termini erano bassi di uno insieme — e nominano tipi che nascono coi **Traguardi 3, 5 e 6**. ⚠️ **E la loro descrizione era stretta, corretta lo stesso giorno:** diceva *«nominano tipi dell'arbitro, del giornale e del canale worker»*, e **due delle dieci non vi rientrano** — `V5`, un effetto senza classe dichiarata, e `V10`, un sensore che modifica l'artefatto. Chi le ricontasse **dalla descrizione** ne troverebbe otto: è lo stesso difetto della riga qui accanto un livello più sotto, e si ricontano **sul catalogo**, che è l'unico posto che le enumera. ⚠️ **E `V5` merita una parola, perché il Traguardo 3 l'ha resa ingannevole:** il tipo `EffectClass` **esiste** da `crates/kernel/src/record.rs` ed è un campo obbligatorio del record, ma **nessun caso lo esercita** — un tipo che esiste non è un controllo che scatta, e la riga resta fra le scoperte |
| i test di contratto per le **altre cinque** famiglie di porte | ✅ **`reactor` è coperta** dal Task 7 — sonde R1…R6 di livello 2. Restano `journal`, `filesystem`, `network`, `process` e `ipc`: le loro implementazioni nascono coi **Traguardi 3, 5 e 6**, e la suite di ciascuna nasce con esse |
| **due residui dichiarati dentro `SystemReactor`**, e stanno qui perché un registro che li tacesse mentirebbe | ⛔ sostituire `Some(self.now())` con `Some(deadline)` in `wait_until` **non fa scattare nulla**: la conformità non può coglierlo perché **sulla finta le due espressioni coincidono**, e distinguerle sulla vera richiederebbe l'overshoot dello sleep del sistema operativo, che nessuna piattaforma garantisce — un controllo verde per fortuna e rosso per sfortuna, cioè peggio di uno assente (gotcha #24). ⚠️ E `R5` prova che `wall_time()` non è **ferma**, non che sia **esatta**: l'esattezza vorrebbe una seconda sorgente di tempo |
| i **byte congelati** del record durevole | ⚠️ **Il soggetto era sbagliato, corretto il 2026-08-10:** diceva *«non esiste ancora nessun record»*, e il record **esiste** dal Task 1 del Traguardo 3 — `crates/kernel/src/record.rs`, con i suoi byte già misurati (`82 00 81 84 00 01 00 40` a payload vuoto). A non esistere è `crates/kernel/tests/frozen_bytes.rs`, e finché non esiste le **tre regole sugli indici** di §4.9.2 sono una convenzione e non un controllo: misurato al Task 1 che spostare una variante su un indice **libero** lascia l'intero banco verde, perché il `derive` rinumera codifica e decodifica insieme. Vincolo 14 della §11 del [compendio](COMPENDIO.md), **Task 10** di questo traguardo |
| la **campagna DST**, e l'elenco versionato dei **semi** di V31 | ⚠️ **Il soggetto era sbagliato, corretto il 2026-08-09:** diceva *«non esiste ancora il simulatore»*, e la crate `simulator` **esiste** e spedisce `SeededRng` e `VirtualReactor` — `crates/kernel/tests/executor_determinism.rs` gira già **C1 e C2** su di essi. A non esistere è la **campagna**: molti semi, guasti iniettati, e l'elenco versionato dei semi. Traguardo 4 |
| i **byte consumati** pari alla lunghezza dichiarata dal frame | non esiste ancora il canale verso i worker. Traguardo 6 |
| le **righe 1–4 di §6.10.5** — i casi negativi della porta `process` | ⛔ **Scaglionate, e la ragione è strutturale e non di fretta.** Tutte e quattro pretendono di **ottenere** un `Worker`; un `Worker` lo restituisce solo `start(grant, ..)`; e nessuno emette concessioni prima del **Traguardo 5**. Scriverle oggi darebbe quattro casi che falliscono perché manca la `Grant`, cioè **verdi per il motivo sbagliato** (gotcha #24) — una regola provata in una direzione sola non è ammissibile (§7.1.1 regola 3). ⚠️ Un costruttore di `Grant` dietro una feature di test resta **scartato** per la ragione già scritta nella riga del blocco **B** qui sopra. Quel che le sostituisce intanto è `ports_are_implementable.rs`, che le firme le esercita **in entrambe le direzioni** con una finta costruita direttamente dal test |
| solo `secrets` raggiunge il **portachiavi** | nessuno script lo verifica oggi: `gate-deps.sh` guarda i grafi di `kernel` e `simulator`, non quelli di `platform` e `secrets` |
| un solo **punto di uscita verso la rete** | la lista delle crate autorizzate è **vuota**, e una lista vuota passa sempre. Il catalogo lo dichiara già: è l'unica voce provata in una direzione sola, e si completa nel sotto-progetto che accende la rete |
