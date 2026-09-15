# Registro delle correzioni — revisione del piano intero, 2026-09-15 (sessione 12)

Fonte: 9 rapporti su 11 (R1 parziale: compiti 1–2; R3 parziale: compito 7; R5 completo: 11–12; R2, R4, R7, R9a, R9b, R10 completi).
Mancano: compito 13 (R6), compiti 15–17 (R8), compito 3 (R1), compito 8 (R3) — perimetri da rivedere ancora.
Ogni rilievo qui è stato riletto dal coordinatore; quelli marcati ✔ sono stati rimisurati anche da lui.
**Stato al 2026-09-15 (sessione 13, tredicesima chiusura):** testa e compiti 1–10 applicati (✅) — l'8 nelle sole due correzioni note, la sua revisione in profondità resta da fare; tutto il resto ⬜. **Sessione 14 (2026-09-15):** compito 11 applicato (✅), con una riga R5-11 nuova sul 12.

## Le decisioni nuove (righe D da scrivere)

| D | Decisione | Da |
|---|---|---|
| D75 | ogni richiamo datato che un compito DETTA porta `<data>` (il giorno dell'esecuzione), mai la data di scrittura del piano; i criteri `grep -c … 2026-09-1x` passano a `<data>`; compiti 1–6 e 13 riallineati ai 8, 9, 14–17 | R1-6, R2-6, R9a-18 |
| D76 | nei commenti di codice dettati, il sotto-progetto 2 si nomina «sub-project 2, task N» — mai «milestone 2 task N», che nel codice di oggi significa il Traguardo 2 del sotto-progetto 1 (`crates/kernel/src/executor.rs`, `ports/ipc.rs`, `ports/mod.rs`); «milestone N» resta il Traguardo N del SP1 | R4-14 ✔ |
| D77 | una `E` di un altro piano si cita col nome del piano, come le `D` (P-112): nella prosa del piano e nel diario; i commenti di codice seguono i sorgenti (numeri nudi) | R10-12 |
| D78 | il trasporto `ipc` su Windows NON legge in modo non bloccante: `read` con `PIPE_NOWAIT` rende `Ok(0)` a pari vivo e a pari sparito (misurato: 4 sonde rosse su 9, `interprocess::os::windows::misc::downgrade_eof` + `std` che classifica `ERROR_NO_DATA` come `BrokenPipe`). Forma: `accept` non bloccante (misurato `WouldBlock`), flussi BLOCCANTI, un thread lettore per client che spinge i byte letti in un canale `mpsc`; `receive` svuota il canale e incornicia; l'EOF vero lo dà la `read` bloccante sul thread (canale chiuso → `Disconnected`); `send` scrive bloccante sulla metà di scrittura. Costo dichiarato: un thread per client (0..1 nel 2), `send` può bloccare a tubo pieno (limite scritto nel doc). Richiamo `<data>` sulla riga 🔶 dedotto della §3 e sul Passo 1(a) del 2 | R1-7 ✔ (meccanismo alla fonte), R1-15 |
| D79 | `typescript` si appunta a **5.9.3** e non alla 7.0.2: `vue-tsc` 3.3.11 risolve `typescript/lib/tsc`, che la 7 non esporta (`ERR_PACKAGE_PATH_NOT_EXPORTED`, misurato installando il modello dell'11); con la 5.9.3 build e 9 sonde verdi. P-2 riceve il richiamo | R5-1 |
| D80 | la disposizione salvata è PER VISTA: `LayoutPack { view: ViewName; layouts: Partial<Record<ViewName, SerializedDockview>> }`; `apply` legge `layouts[view] ?? shipped`; `settle` fonde la vista corrente; due sonde (salva sotto `home`, `switchTo("work")` → la spedita di Lavoro; torna → la salvata) | R9b-11 ✔ |
| D81 | `beforeunload` salva solo se `!same(last, api.toJSON())` (decisione 11 del coordinatore della stella: i default non si copiano nell'archivio) | R9b-12 ✔ |
| D82 | la suite di conformità di `ipc` gira SOLO sul trasporto vero (nessun bugiardo nel 2: ogni promessa vuole un pari che scrive byte) e vive in `crates/kernel/tests/contract/ipc.rs` — NON un bersaglio di prova (niente bersaglio a zero test, niente avviso `unused_macros`, niente `#[allow]`) — inclusa da `crates/platform/tests/ipc_contract_real.rs`; la riga 2 della posizione perde «coi bugiardi»; richiamo `<data>` sulle righe «la suite di conformità» della §3 e «il trasporto `ipc`» della §8 | R1-9, R1-13, R9a-4 |
| D83 | il lockfile del core finto si SEMINA dalla radice (`cp Cargo.lock gui/fake-core/Cargo.lock` prima del primo `cargo build`) e un criterio del 12 confronta le versioni delle crate comuni; `cargo audit --file gui/fake-core/Cargo.lock` entra in `gate-gui.sh` (compito 16) | R5-10 |
| D84 | il processore a riposo del daemon si MISURA al compito 9 (comando, 60 s, senza soglia) e il 17 lo porta in `riferimenti.md` con comando e data — decisione 41 del proprietario | R9a-5, R9b-7 ✔ |
| D85 | `docs/design/10-modello-dei-dati-durevoli.md` si aggiorna al 17: `INVOCATION_DETAIL` passa al primo `erDiagram` col richiamo (regola scritta nel file, decisione 20), `POLICY_DETAIL` (specie 4) entra | R9b-2 ✔ |
| D86 | la riga 2 della roadmap cambia TITOLO al 17 sul perimetro della §3 della stella (debito dichiarato della §3) | R9b-3 ✔ |
| D87 | il 14 scrive un richiamo `<data>` per OGNI modulo costruito nelle tabelle della §1 della stella (regola della §6: chi costruisce un modulo mette il richiamo nella sua riga) e nella riga Impostazioni della corta; il 17 scrive i ✅ sui 🔶 dedotti della stella confermati dal piano | R9a-13, R9b-4, R9b-13 |
| D88 | i richiami che le decisioni D del piano rendono dovuti nei DUE disegni si scrivono dal compito che esegue la decisione: §3 (D9, D10, D78, D82) dal 2; §4 riga `Request, Verdict` (D5) dal 7; §8 «decodificata dai byte» e §2 «la SPA parla bincode» (D36) dall'11; §7 righe «la disposizione», «la lista dei passi», «senza copiarla», dedotto `Accepted` (D41, D22, P-70) dal 12; §8 «Ciò che la §8 non fa» (X-1/X-3) dal 16; §8 «le scritte» (D63/D65) dal 15; decisioni 6, 45, 47, 51, 52, 55 del coordinatore della stella (D2, D71, D66, D3, D4, D63–D65) dal compito che le esegue; D2 e D4 riallineate (13 e 11) | R9a-3, R9a-7, R9a-8, R9a-10, R9a-11, R9a-12, R9b-8, R9b-9, R9b-10, R3-18, R5-15, R5-16 |

## Per compito — rilievi, rimedio deciso, stato (⬜ da applicare · ✅ applicato)

### Testa del piano (posizione, D, voci aperte, diario)
- R10-3 riga 7 posizione «al 8» → «al 9» ✅
- R10-4/R9b-8/R5-35 D2 «compito 12» → «compito 14, in `moveActive.ts`» (la nota vive lì; richiamo datato) ✅
- R10-5/R9b-10 D4 «al compito 10» → «al compito 11» ✅
- R10-6 D12 «dopo il 10» → «dopo l'11», «sette compiti» → togliere il numerale ✅
- R10-7/R5-9 Interfaces 11 `Fixture { file, kind, value }` → `{ file, message: IpcMessage }`; P-75 richiamo «applicata in revisione» ✅
- R10-12 (D77) E nude in D26, D29, P-49, prosa del 8, diario → col nome del piano ✅
- R10-13 diario: `grep -n '<the version'` → `awk` fermato al diario (nel diario nuovo) ✅
- R10-14 comandi con `\|` nelle celle: forma `[|]`; trappola nel diario ✅
- R10-19 roadmap riga: «in scrittura dal 2026-09-11» → «scritto il 2026-09-15, rivisto il <data della chiusura>» — fuori dai compiti ✅
- R4-15 secondo segnaposto `<la versione …>` nel 9 → `interprocess = "2.4.4"` tale e quale al 2 ✅
- R10-15 conteggio posizione: solo nota ✅ (nessuna modifica)

### Compito 1
- R1-1, R1-2 ancore vere su due righe (`registered, not taken.` / `with nothing to report it.`) ✅
- R1-3 citazione con «in milestone 6» ✅
- R1-4 alias `alloc_vec_of_steps` + `#[allow]` → `Vec<u64>` ✅
- R1-5 consumatori «2, 7, 9, 10 e 12» ✅
- R1-6 (D75) `<data>` ✅

### Compito 2
- R1-7 (D78) trasporto: Passo 8 riscritto (thread lettore per client, canale, `split`), Passo 1(a) richiamo, suite adattata; cascata su 9 e 12 (i pari leggono bloccanti: già così) ✅
- R1-8 Passo 9-bis: i tre richiami P-23 (`ports/mod.rs` ×2, `ports_are_implementable.rs` riga 1) con Trova unici; CR/eol/add/criterio ✅
- R1-9/R1-13/R9a-4 (D82) suite in `tests/contract/ipc.rs`, solo trasporto vero; riga 2 posizione; richiami §3/§8 ✅
- R1-10 `send`: `WouldBlock` → con D78 la scrittura è bloccante; doc del limite; sonda `send` a pari sparito ✅
- R1-11 consumatori «9 e 12»; `declared_len` interno ✅
- R1-12/R4-16 `Cargo.lock` esce LF da cargo: atteso `i/lf w/lf`, CR 0 (anche nel 9) ✅
- R1-14 citazione di `platform/src/lib.rs` alla lettera; ancora sotto AUD-022; `pub mod ipc;` dopo `pub mod rng;` ✅
- R1-15 sonda pari sparito con baseline pari vivo → `Ok(None)` ✅
- R9a-3 (D88) richiami §3: `send` (D10), `receive` (D9+D10), 🔶 dedotto (D78) ✅
- R10-16 etichette eol mancanti in Files (framing.rs) ✅

### Compito 3 — NON RIVISTO IN PROFONDITÀ (R1 caduto): resta da fare
- R10-16 etichette eol ⬜ · D75 `<data>` ⬜ · D76 «sub-project 2» ⬜

### Compito 4
- R2-1 Trova della spec: ultima riga del riquadro `> come \`process\` in §2.3.1.` ✅
- R2-2 Trova riga 37 «FIVE fakes, because» (spezzata) ✅
- R2-3 tabella dei tocchi: righe 29, 32, 43 dette; testo del richiamo del tocco 5 ✅
- R2-4 Passo 1: baseline `ports_are_implementable` 14 passati, `grep -c` → 11 ✅
- R2-5 C3: `E0053` + tre `E0308` ✅
- R2-6 (D75) `<data>` ✅
- R2-7 consumatori «5, 7, 9, 10 e 12» ✅

### Compito 5
- R2-8 criterio: `grep -c 'fn with_backend'` → 0 ✅
- R2-9 `use redb::…` senza `ReadableTable` (la mutazione lo aggiunge) ✅
- R2-10 citazione «not written here as a fixed set» ✅
- R9a-6/R9b-1 (D14 allargata): il commento falso di `journal.rs` riga 63 riscritto al vero nello stesso tocco; criterio «due regioni»; voce 10 della §9 ✅ `<data>` ✅

### Compito 6
- R2-12 Files + Passo 8-bis: `record_shape.rs` (braccio + array + la quarta nota), `dst_campaign.rs` (braccio `panic!`); richiamo in P-33 **e in P-45** (il cui ✅ era una promessa) ✅
- R2-13 Passo 8: `InvocationDetail` nell'`use` di `frozen_bytes.rs`, braccio `Detail::Invocation(_) => {}`, voce nell'array `for kind in […]` ✅
- R2-14 `record(RecordV1::intent, EffectClass::Idempotent)` / `outcome`; `InvocationDetail` nell'`use` di `reconciliation.rs` ✅
- R2-15 `an_invocation_note()` con `EffectClass::Unrepeatable`; seconda nota dopo l'`outcome`; criterio: `enter` → rosse entrambe, `leave` → rossa la seconda ✅
- R2-16 «sei» test del registro ✅
- R2-17 criterio D16: `grep -c 'use crate::arbiter\|crate::arbiter::'` → 0 ✅
- R2-18 Passo 1: `grep -c '^\s*#\[n('` → 29; indici con `-B1` ✅
- R2-19 Passo 1: `grep -rn 'match .*kind\b'` → 4, `match detail` → 1 ✅
- R2-20 Atteso eol: sei crlf + disegno lf ✅
- R2-21 consumatori «7 e 10 (14 per nome)»; `Operation` del banco ✅
- R2-22 Passo 1: baseline reconciliation 18 ✅
- R2-6 (D75) `<data>` ✅
- D76 «sub-project 2 design» nei sei commenti dettati del 6 (cinque più uno maiuscolo) — non era in questa lista: applicata per coerenza col compito 5 (patch_c45.py) ✅
- Attrezzo: `patch_c6.py` accanto a questo file, modello per i compiti 7–17 (fetta del compito, ancore asserite, scrittura atomica)
- R10-16 etichetta eol di `crates/kernel/tests/frozen/record_v1.map` in Files — la riga R10-16 nomina anche il 6 e la lista del 6 non la portava: applicata con `patch_c7.py` ✅

### Compito 7
- R3-1 alternanza chiusa con `\b` / Atteso 4 righe ✅
- R3-2 pattern `pub \(const \)\?fn policy` ✅
- R3-3 Atteso `registry.rs` `i/lf w/lf` — assorbito da R3-4: `registry.rs` esce dal comando `git ls-files --eol` del Passo 1, e l'Atteso lo dice ✅
- R3-4 togliere le due «Modify» (registry.rs, wire/ipc.rs); Passo 4 → verbale «già applicato nei compiti 3 e 6» ✅
- R3-5 compile_fail: sette chiamate su una riga; `parameters_have_no_default.stderr` va `mismatch` (atteso), rigenerato per la via del file (mai `TRYBUILD=overwrite`), letto; Passo 2 «cinquanta chiamate in ventuno file» ✅
- R3-6 «le righe 7 e 9» ✅
- R3-7 «(task 9)» ✅
- R3-8 «task 12»; «task 10's campaign, D34 took it» ✅
- R3-9 doc: DST del 10 su un filo proprio dietro `RefCell` (D32) ✅
- R3-10 «THE SECOND COPY … third and fourth with 9 and 10, fifth with 12» ✅
- R3-11 citazione di client.rs con le lineette ✅
- R3-12 richiamo D22 e 🔶 dedotto: il rifiutato esce subito dalla tabella ✅
- R3-13 undicesima sonda `a_word_the_dispatch_does_not_know_is_refused_without_a_word` ✅
- R3-14 riga «NON prova» che nomina il 5 e il 9 ✅
- R3-17 Files: stella polare (LF) riga 2 Stato (D20) ✅
- R3-18 (D88) richiamo D22 in §7 dedotto ✅
- R9a-9 custodia che rifiuta `keep` nel banco + sonda `Layout(Package(primo))` ✅
- R9a-10 (D88) richiamo D5 sulla riga `Request, Verdict` della §4 ✅
- R10-16 etichette eol (lib.rs, parameters.rs, executor.rs) ✅
- D76 «sub-project 2» nei commenti (quattordici posti; «milestone 2 of sub-project 1» e «milestone 6» restano, perché sono Traguardi del SP1) ✅ · D75 `<data>`: il Passo 8 lo portava già; la cella D21 riceve «compito 7» e il Passo 9 il criterio sui richiami ✅
- Attrezzo: `patch_c7.py` accanto a questo file

### Compito 8 — NON RIVISTO IN PROFONDITÀ (R3 caduto): resta da fare
- R10-12 (D77) E nude nella prosa: `E66` ed `E112` (lezione delle date) ed `E50` (l'oracolo) col nome del piano del Traguardo 6 ✅ · D75/D76: nulla da fare — i richiami dettati dell'8 portano già `<data>`, e «milestone 5 task 9» è il Traguardo 5 del SP1 ✅ (la revisione in profondità resta da fare)

### Compito 9
- R4-1 Passo 7: la chiamata di `main` a due argomenti; «rosso su main e su tests» ✅
- R4-2 Passo 8: `decode(&buffer[..next])`, via il capoverso del dubbio ✅
- R4-3 Passo 14 → 7-bis (dev-dependency prima del pari); il numero 14 libero è preso dalla misura del processore (D84) ✅
- R4-4 `Millis::new(0)` ×5, via la copertura ✅
- R4-5 richiamo sul doc di `run_the_production_graph` (SO THAT A TEST CAN CALL IT) ✅
- R4-6 `connect` con scadenza di parete (5 s) e panico che nomina la riga; budget largo `WITH_A_PEER` per ogni sonda con un pari (Passi 10, 11, 12) ✅
- R4-7 baseline a limite 0 e **senza pari** (a zero giri un pari non fa in tempo a collegarsi, R4-6) ✅
- R4-8/R9a-2 i quattro corpi `/* … */` dettati (con `WELCOME` e `WITH_A_PEER`, che nessun passo definiva; il secondo core con `FILE_FLAG_FIRST_PIPE_INSTANCE` letto nel sorgente di `interprocess`) ✅
- R4-9 Passo 13 `Disconnected`: coperto dal 7 e dal 10, dichiarato; criterio «Passi 10–12»; richiamo §8 ✅
- R4-10 «Passo 12», «le quattro sonde delle tre decisioni» ✅
- R4-11 doc di `SOCKET_NAME`: il finto lega lo STESSO nome (D45) ✅
- R4-12 «task 10 of this plan … keeps it local (D34)»; «the milestone 2 campaign» → «the sub-project 2 campaign (task 10)» ✅
- R4-13 «the assembled shell, which this plan does not build (P-53)» ✅
- R4-14 (D76) ✅
- R4-15 segnaposto → `interprocess = "2.4.4"` — era già applicato nell'ondata 1 (testa), qui solo verificato ✅
- R4-16 `Cargo.lock` → `i/lf w/lf` ✅
- R4-17 «The doc of `run_the_production_graph` says…» ✅
- R4-18 «in coda alla cella La prova» ✅
- R3-15 = R4-4 ✅
- R9a-10 richiamo §8 riga «il limite di giri e Disconnected» ✅
- R5-3 = R4-2 ✅
- D84 misura CPU a riposo (Passo nuovo + criterio) ✅
- R10-16 etichetta eol `Cargo.lock` ✅
- Attrezzo: `patch_c89.py` accanto a questo file (compiti 8 e 9); ⚠️ trappola: il blocco spostato (Passo 14 → 7-bis) si sposta PRIMA di inserire un nuovo «Passo 14», o l'ancora `Passo 15` porta via anche quello — scattato una volta, senza scrivere

### Compito 10
- R4-19/R10-18 blocco Files (4 righe) ✅
- R4-20 sonda-premio: `Wire::watched_for_ever(client)` (= `dies(client, u64::MAX)`), gemello di `without_crash`, dettato nel Passo 3 e usato nella tabella del Passo 5 ✅
- R4-21 Passo 1 + `grep fn replay -A2` ✅
- R4-22 «i due `match` della campagna» (anche in P-61) ✅
- R4-23/R10-11 Consumes: via `Journal`/`Custody` non importati; + `RngExt`, `MemoryJournal` ✅
- R4-24 via il `tee` su `$SCRATCH` (il log lo tiene chi esegue nello scratchpad) ✅
- R4-25/R9a-14 richiamo §5 su UNA riga, ancora = riga intera `| l'attività | …` ✅
- R4-26 «quattro righe: la struct e i suoi tre metodi» (anche P-60) ✅
- R4-27 Passo 7: rimisurare i costi del settimo passo e riscrivere la riga con `<data>` ✅
- D76 ✅
- Attrezzo: `patch_c10.py` accanto a questo file

### Compito 11
- R5-1 (D79) `typescript` 5.9.3 nel Passo 3 con la ragione accanto, nel dizionario del Passo 2 e nella frase del Passo 5; **P-2 riceve il richiamo** (D79 lo prometteva e nessuna riga della testa lo portava); l'Atteso del Passo 2 rimisurato il 2026-09-15 con la 5.9.3 e `@vitejs/plugin-vue` nel dizionario: `jsdom` resta il più stretto su ogni ramo ✅
- R5-9 = R10-7 — già applicata nell'ondata 1 (testa), qui solo verificata: `grep -c 'interface Fixture { file: string; message: IpcMessage }'` → 1 ✅
- R5-11 ⚠️ **NON `git add -N`: `git add gui .gitignore` (Passo 4) e `git add gui` (Passo 13)** — misurato il 2026-09-15 in un repository di prova: con *intent-to-add* `git diff --stat` mostra il file intero anche a revoca fatta (mai «vuoto») e `git checkout --` lo **svuota**; messo in scena, il diff a zero e il ripristino dall'indice reggono. Il Passo 4 lo spiega, il criterio lo nomina; **il 12 resta da fare** nella stessa forma (riga aggiunta sotto) ✅
- R5-13 «i compiti 13 e 14», col perché (il 12 è Rust) ✅
- R5-15 (D88) Files + **Passo 13-bis** con la tabella delle due ancore (§8 la riga `la SPA, schema/`; §2 la frase `la SPA parla bincode` del richiamo del 2026-09-10, `grep -c -F` → 1), il `git add` del Passo 14, il messaggio di commit e il criterio `grep -c` → 2 ✅
- R5-18 Passo 2: il `for … npm view … version dist-tags` sulle sei del manifesto e la regola di lettura del vincolo 8 (major no; minor/patch solo se l'appuntata non si installa, con errata) ✅
- R5-28 verificato: «si aggiorna Node PRIMA di proseguire» è nel Passo 2 — nulla da fare ✅
- D76 «sub-project 2 design» nei due commenti dettati (`fixtures.ts`, `bridge.ts`) — non era in questa lista: applicata per coerenza coi compiti 5–10 ✅ · D79 e D88 nella lista *Read* ✅
- Attrezzo: `patch_c11.py` accanto a questo file (tocca anche P-2 e questo registro)

### Compito 12
- R5-11 `git add gui/fake-core` prima delle mutazioni del Passo 9 — **in scena, non `-N`**: la misura è nella riga R5-11 del compito 11 ⬜
- R5-2/R3-16 `let clock = SharedClock(&reactor);` prima dell'esecutore; riga «load-bearing» ⬜
- R5-3 `decode(&buffer[..next])` ⬜
- R5-4 la parola dopo l'accoglienza (pari che tiene il `Sender`, manda a `heard.len() >= 5`); `degrade` sulla seconda `Degradation` ⬜
- R5-5 criterio sulla metà non-test + direzione opposta ⬜
- R5-6 canale tenuto vivo nella sonda negativa; commento; G1 atteso «timeout» ⬜
- R5-7 accoglienza: asserire `report.allocated == Mib::new(1_024 + 768)` e `total` ⬜
- R5-8/R9a-1 Passo nuovo: richiamo D41 in §7 (ancora «senza copiarla: dove spostarla lo decide il piano»), criterio `grep -c 'D41'` ⬜
- R5-10 (D83) `cp Cargo.lock`; criterio versioni comuni ⬜
- R5-12 `build_stamp` dentro `mod tests` ⬜
- R5-14 Consumes riscritto dal codice ⬜
- R5-16 (D88) richiami §7 «la disposizione»/«la lista dei passi» → provate nel 7 ⬜
- R5-17 `time` a freddo e a caldo; il numero al 15 nel commento di `gate-gui.sh` ⬜
- R5-19 due `python -` con `newline=""` e `assert` per `Cargo.toml` e `.gitignore` ⬜
- R5-20 `grep -rl … | wc -l` → 4 ⬜
- R5-21 P-63 comando `grep -n 'non si carica da'` ⬜
- R5-22 «under the root's rust-toolchain.toml» ⬜
- R3-18 (D88) richiamo D22 in §7 ⬜
- D76 ⬜

### Compito 13 — NON RIVISTO IN PROFONDITÀ (R6 caduto): resta da fare
- R9b-11 (D80) LayoutPack per vista ⬜ · R9b-12 (D81) ⬜ · R10-9 Files `generate-views.test.ts` ⬜ · R10-10 Produces: `BigTab`, `buildStamp`, `Phase` ⬜ · R7-7 (chiavi `modules.*` sono del 13: correggere il 15) ⬜ · D75 `2026-09-15` → `<data>` ⬜

### Compito 14
- R7-1 `String(…attrGet(…) ?? "")` ×2 + riga sul tipo ⬜
- R7-2 render legge `stream.current?.text` al fotogramma; sonda con due `Token` prima di `await frame()` ⬜
- R7-3 criterio: dopo `deliverAll()` un `deliver("Accepted")` e la frase su `StaleBuild` ⬜
- R7-4 criterio: la finestra si apre al click (pending dalla fixture) ⬜
- R7-5 «le QUATTRO righe … fino a `reason the shape comes before the palette. */`» + `grep -c -F` ⬜
- R7-6 `grep -rcE … --include='*.test.ts' | grep -v ':0$'` → niente ⬜
- R7-8 «si rifà a mano la modifica, o `git add` prima» ⬜
- R7-9 Files: `--stop`, capoverso, regole della linguetta ⬜
- R7-10 seconda direzione di `modules.test.ts` (placeholderParams) o commento ⬜
- R7-12 `git log … | wc -l` → 1 solo ⬜
- R7-13 script `types or typings`; prosa `.d.cts`/`.d.mts` ⬜
- R7-14 richiamo del Passo 15 esteso alla riga 3 di Permessi e alla riga 12 di Passi (costanti nel 2) ⬜
- R7-26 «anche nel grep» con la data ⬜
- R10-8 Files: stella polare (LF) ⬜
- R9b-4/R9a-13 (D87) richiami per modulo costruito nella §1 della stella ⬜
- R9b-5 «✅ chiusa» sulle due registrate (scorciatoie da tastiera; forma della transizione — quest'ultima dal 8) ⬜
- D75/D76 ⬜

### Compito 15 — NON RIVISTO IN PROFONDITÀ (R8 caduto): resta da fare
- R10-1 eol: `gate.sh` e il flusso sono `i/lf w/crlf` (Files, Passo 1, Passo 8/9 CRLF) ⬜ · R7-7 «(compito 13, Passo 12)» ⬜ · R7-11 «tredici .vue, dodici a una parola» (anche P-99) ⬜ · R9a-12 (D88) richiamo «le scritte» §8 ⬜ · R9a-15 P-104 «§8» e «lo scrive il 15» ⬜ · R5-17 numero del tempo del finto ⬜

### Compito 16 — NON RIVISTO IN PROFONDITÀ (R8 caduto): resta da fare
- R10-2 eol: `gate.sh`, flusso, audit `i/lf w/crlf`; solo `gate-gui.sh` LF; blocco CRLF; `cp` con fine-riga del file ⬜ · R9a-11 (D88) richiamo «Ciò che la §8 non fa» ⬜ · D83 `cargo audit --file gui/fake-core/Cargo.lock` ⬜

### Compito 17 — NON RIVISTO IN PROFONDITÀ (R8 caduto): resta da fare
- R9b-15 criterio roadmap: comando ancorato alla riga e depurato delle citazioni ⬜ · R9b-14 Passo 10: `grep -c 'dal compito 14 del piano della parte 2 (P-89, D56)'` → 1 ⬜ · R9b-3 (D86) titolo roadmap ⬜ · R9b-6 stella: riga «codice e spec non toccati» + «Il prossimo passo» ⬜ · R9b-13 (D87) ✅ sui 🔶 dedotti ⬜ · R9b-2 (D85) design/10 ⬜ · R9a-16 Definizione di fatto: `git diff --name-only 42b50d8..HEAD` contro le liste Files; `docs/adr/` vuoto ⬜ · D84 CPU in riferimenti.md ⬜

## Voci P nuove (P-117…): una per rilievo «fatto» confermato, raggruppate per compito; i «prosa» stanno nella tabella della dodicesima chiusura
