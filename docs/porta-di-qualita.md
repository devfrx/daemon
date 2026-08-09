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

Le tre righe del **blocco A** di §7.4.1, e **tre righe del blocco C** dal Traguardo 2. Il
blocco **B** (i gettoni) non è ancora implementato: i suoi gettoni li emettono l'arbitro e
il filtro dei vincoli, che nascono coi Traguardi 5 e 6.

| Regola del catalogo | Dove è dichiarata | Caso negativo |
|---|---|---|
| `#![no_std]` su `kernel` e `simulator` | `crates/kernel/src/lib.rs` · `crates/simulator/src/lib.rs` | `crates/kernel/tests/compile_fail/std_in_kernel.rs` |
| `#![forbid(unsafe_code)]` sulle stesse | idem | `crates/kernel/tests/compile_fail/unsafe_in_kernel.rs` · `allow_overrides_forbid.rs` |
| `HashMap` non nominabile | conseguenza gratuita di `no_std` | `crates/kernel/tests/compile_fail/hashmap_in_kernel.rs` |
| **blocco C** · `V29 · §2.1` — i due tempi non si scambiano, **in nessuna delle due direzioni** | `crates/kernel/src/time.rs` — `Monotonic` e `WallTime` sono due tipi distinti | `monotonic_as_wall.rs` · `wall_as_monotonic.rs` |
| **blocco C** · `V29 · §2.1` — **nessuna via `From`/`Into`** fra i due tempi | idem: nessuna conversione è dichiarata, e il divieto non è più un commento | `no_conversion_from_monotonic_to_wall.rs` · `no_conversion_from_wall_to_monotonic.rs` |
| **blocco C** · `V29 · §2.2` — la **riduzione** di `below` non è sovrascrivibile | `crates/kernel/src/rng.rs` — `below` vive su `RngExt`, con `impl<R: Rng> RngExt for R {}` | `override_below.rs` |
| **blocco C** · `V29 · §2.8` — il kernel **non nomina un default** | `crates/kernel/src/parameters.rs` — nessun `impl Default`, e `new` pretende ogni campo | `parameters_have_no_default.rs` |

⛔ **Le due direzioni non sono simmetriche nel modo di scattare, e la differenza conta —
gotcha #42.** `trybuild` stampa **`error`** quando un caso ha compilato e **`mismatch`**
quando l'uscita non combacia con l'oracolo:

| Regola | Scatta come | Dipende dall'oracolo? |
|---|---|---|
| i due tempi non si scambiano | `mismatch` | **sì** — una rigenerazione in blocco la spegnerebbe in silenzio |
| nessuna via `From`/`Into` | **`error`** | no |
| `below` non sovrascrivibile | **`error`** | no |

Misurato: col solo caso «passa l'uno per l'altro», aggiungere `impl From<WallTime> for
Monotonic` lasciava la porta **verde su sei controlli su sei**. La riga `From`/`Into` esiste
perché quella era la direzione **pericolosa** — una decisione che dipende dal wall time — e
perché una regola guardata solo da casi `mismatch` non è guardata abbastanza.

**Contro-sonde delle righe nuove:** `crates/kernel/tests/time_types.rs` (sette) ·
`crates/simulator/tests/seeded_rng.rs` (otto). ⛔ **I cinque casi nuovi nominano `kernel::`
e non ridichiarano attributi propri**, a differenza dei quattro del Traguardo 1: è il rimedio
al gotcha **#39**, e significa che i loro oracoli sono accoppiati alla **superficie pubblica
del kernel**. Un cambio di firma li rende rossi, ed è corretto che lo faccia.

⛔ **La colonna «Caso negativo» prova il meccanismo, non la dichiarazione — e il registro
non deve lasciar credere altro.** I quattro casi di `crates/kernel/tests/compile_fail/`
**ridichiarano ciascuno i propri attributi** e non nominano mai `kernel::`. Provano che
`#![no_std]` e `#![forbid(unsafe_code)]` **mordono dove sono dichiarati**; non provano che
siano dichiarati nel kernel. Tolto `#![forbid(unsafe_code)]` da `crates/kernel/src/lib.rs`
e scritto un `unsafe` vero, quei casi restano **verdi**.

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

Le sonde, per nome:

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

⛔ **La riga del build script è la sesta voce di livello 2, ed è entrata il 2026-08-09 su una
lacuna _misurata_.** Un `crates/kernel/build.rs` che chiama `SystemTime::now()`,
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

**Contro-sonde, per esteso:** N4 · B3 · le due contro-sonde di `gate-attributes.sh` —
`platform`, `secrets` e `daemon` non dichiarano nessuno dei tre attributi e **restano
verdi**, e `crates/platform/build.rs` **resta verde** anch'esso, perché non sono nella lista
dei vincolati e un controllo che scattasse anche lì sarebbe rosso per il motivo sbagliato
(gotcha #24): `platform` è **il posto dove l'I/O deve vivere**. Più `build = false`, che il
build script lo **disattiva** e va distinto da `build = "gen.rs"`, che lo dichiara. E, al
livello sopra, `crates/platform/tests/counter_probes.rs`.

## Livello 3 — vuoto, e non è una svista

`clippy` gira come igiene del codice ma **non ha voce nella porta**: nessun V dipende da
lui. Un rosso della porta deve significare sempre «invariante violata», mai «stile
discutibile», o si impara a ignorarlo.

## Cosa la porta NON controlla, in questo traguardo

Righe del catalogo §7.4 che oggi **nessun file implementa**. Stanno qui perché un registro
che le omettesse lascerebbe credere che siano coperte.

| Riga del catalogo | Perché non c'è ancora |
|---|---|
| il blocco **B** di §7.4.1 — i **gettoni** | i gettoni li emettono l'arbitro (§5.6) e il filtro dei vincoli (§6.3): **Traguardi 5 e 6**. ⛔ Un costruttore di `Grant` dietro una feature di test **è stato valutato e scartato**: creerebbe il secondo modo di ottenere una concessione che §5.6 esiste per togliere dal compilatore |
| il resto del blocco **C** di §7.4.1 | tre righe su sedici sono implementate (sopra). Le altre nominano tipi dell'arbitro, del giornale e del canale worker, che nascono coi Traguardi 3, 5 e 6 |
| i **test di contratto** fra porta finta e porta vera | ⏭️ **Traguardo 2, Task 7.** La porta `reactor` esiste, la finta pure (`VirtualReactor`), la vera no. ⛔ **Due cose che quella suite deve coprire, o partono non provate:** il ramo `deadline <= now → None` di `VirtualReactor::wait_until` — che **nessun test esercita**, perché l'esecutore non lo raggiunge più da quando promuove i dormienti scaduti — e `VirtualReactor::wall_time()`, che **nessuno legge**. Sono contratto della porta, e §7.4.6 chiama questa suite *«la più importante: la validità della DST poggia lì»* |
| i **byte congelati** del record durevole | non esiste ancora nessun record. Entrano al **primo** record scritto — vincolo 14 della §11 del [compendio](COMPENDIO.md), Traguardo 3 |
| la **campagna DST**, e l'elenco versionato dei **semi** di V31 | non esiste ancora il simulatore. Traguardo 4 |
| i **byte consumati** pari alla lunghezza dichiarata dal frame | non esiste ancora il canale verso i worker. Traguardo 6 |
| solo `secrets` raggiunge il **portachiavi** | nessuno script lo verifica oggi: `gate-deps.sh` guarda i grafi di `kernel` e `simulator`, non quelli di `platform` e `secrets` |
| un solo **punto di uscita verso la rete** | la lista delle crate autorizzate è **vuota**, e una lista vuota passa sempre. Il catalogo lo dichiara già: è l'unica voce provata in una direzione sola, e si completa nel sotto-progetto che accende la rete |
