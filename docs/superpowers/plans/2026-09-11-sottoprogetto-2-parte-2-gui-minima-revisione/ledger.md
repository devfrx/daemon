# Registro delle correzioni — revisione del piano intero, 2026-09-15 (sessione 12)

Fonte: 9 rapporti su 11 (R1 parziale: compiti 1–2; R3 parziale: compito 7; R5 completo: 11–12; R2, R4, R7, R9a, R9b, R10 completi).
Mancano: compito 13 (R6), compiti 15–17 (R8), compito 3 (R1), compito 8 (R3) — perimetri da rivedere ancora.
Ogni rilievo qui è stato riletto dal coordinatore; quelli marcati ✔ sono stati rimisurati anche da lui.
**Stato al 2026-09-15 (sessione 13, tredicesima chiusura):** testa e compiti 1–10 applicati (✅) — l'8 nelle sole due correzioni note, la sua revisione in profondità resta da fare; tutto il resto ⬜. **Sessione 14 (2026-09-15):** compiti 11 e 12 applicati (✅). **Sessione 15 (2026-09-15):** compito 13 applicato (✅) — con **D89** nuova (il dock segue lo store), R7-7 applicata al 15, e la cascata sul `Frame.vue` del 14 registrata come riga del 14. **Sessione 15, ondata 11 (2026-09-15):** compito 14 applicato (✅) — resta all'8 la sua metà di R9b-5. **Ondata 12 (2026-09-15):** compiti 15, 16 e 17 applicati (✅) in una ondata sola. **Ondata 13 (2026-09-15):** le tre righe note del 3 applicate (✅); resta ⬜ la sola metà dell'8 di R9b-5, con la sua revisione in profondità.

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
| D89 | il dock SEGUE lo store: `createDock` osserva `view` e `arrivals` di `useLayout` e mostra la vista da sé (`show()` azzera la baseline del `settle`); lo store conta gli **arrivi** che non sono l'eco del proprio `SaveLayout` (decisione 13: il core risponde con ciò che tiene — i byte si confrontano con quelli mandati); `Frame.switchTo` è `layout.view = view`. Trovato scrivendo D80: il `Layout` dell'accoglienza arrivava **dopo** l'unico `apply` e non veniva mai mostrato; e il `settle` dopo uno `switchTo` copiava la vista spedita nell'archivio (decisione 11) | coordinatore, ondata del 13 (R6 caduto) |

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

### Compito 3 — NON RIVISTO IN PROFONDITÀ (R1 caduto): le righe note sono applicate, la revisione in profondità resta da fare
- R10-16 l'etichetta della sola voce che ne mancava (`*.bin`: binari, niente fine-riga) ✅ · D75 `<data>` nei due `DATED RECALL` di `wire/ipc.rs` e nel criterio ✅ · D76 «sub-project 2» nei sei punti dei commenti dettati; i due «milestone 6» restano, perché sono il Traguardo 6 del SP1 ✅ · Attrezzo: `patch_c3.py`

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
- R9b-5, la metà dell'8: il Passo 12(a) scrive «✅ **chiusa il <data>, compito 8 del piano della parte 2**» in coda alla cella «Chiusore» della riga *«la forma con cui la transizione di policy si rilegge»* delle registrate della stella (la forma del Passo 15 del 14: ancora presa dal file) — con la revisione in profondità dell'8 ⬜
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
- R5-11 `git add Cargo.toml .gitignore crates/kernel/src/serving.rs gui/fake-core` in testa al Passo 9, **in scena e non `-N`** (la misura nella riga R5-11 del compito 11), col `grep -c target/` → 0 come seconda direzione di D38; il criterio delle mutazioni lo nomina ✅
- R5-2/R3-16 `let clock = SharedClock(&reactor);` prima dell'esecutore, `serve(&core, &clock, …)` e `the_faucet(&core, &clock, …)`, il commento «LOAD-BEARING» con E0716/E0597 (misurato dal revisore col modello compilato) ✅
- R5-3 `IpcMessage::decode(&buffer[..next])` nel pari, col commento del 9 (rimisurato: `decode` comincia con `framing::unframe`) ✅
- R5-4 il pari legge fino a un **predicato** (`type Until = fn(&[IpcMessage]) -> bool`) e digita la parola **dopo** l'accoglienza tenendo lui il `Sender` (`then_types: Option<(Sender<String>, &'static str)>`, a `heard.len() >= WELCOME`); `degrade` asserisce `[false, true]` sulle due `Degradation`; `verdict` idem ✅
- R5-5 criterio sulla metà non-test con l'`awk` fino a `#[cfg(test)]` → 0, e la direzione opposta sul file intero → più di zero ✅
- R5-6 `a_keyboard()` rende anche il `Sender`; la sonda negativa lo tiene vivo (`drop(hand)` dopo la corsa) e il commento dice che `Disconnected` è il caso **facile**; G1 resta «timeout» e dice perché morde ✅
- R5-7 accoglienza: `report.allocated == Mib::new(1_024 + 768)` e `report.total == TOTAL_VRAM` (`PolicyReport` è `Copy`); G2 dice che sono queste righe a coglierlo ✅
- R5-8/R9a-1 **Passo 9-bis** con la tabella delle ancore: il richiamo D41+P-68 (e D22, R3-18) in coda all'ultima riga del capoverso 🔶 dedotto (`grep -c -F` → 1 misurato oggi sulla riga 375); criterio `grep -c 'RICHIAMO DEL <data>, compito 12'` → 3; Files riscritto; via la data fissa `2026-09-14` (D75) ✅
- R5-10 (D83) `cp Cargo.lock gui/fake-core/Cargo.lock` prima del primo `cargo build`, il perché (rimisurato: la radice appunta `redb = "4.1.0"` con un caret, il lockfile 4.1.0), lo script del Passo 10 che confronta le crate comuni → `[]`, e il criterio ✅
- R5-12 `use kernel::wire::ipc::build_stamp;` dentro `mod tests`, tolto dall'`use` di testa (usato solo dalle sonde: verificato sul modello) ✅
- R5-14 Consumes riscritto dagli `use` del Passo 5: `take_frame` dal 2; `build_stamp`, `stamp_set`, `BuildStamp`, `Call` dal 3; da oggi anche `Parameters`, `Reactor`, `WallTime`, `Verdict`; via `Grant` e `Detail` ✅
- R5-16 (D88) i due richiami sulle righe «la disposizione» e «la lista dei passi» della §7 nel Passo 9-bis → provate nel kernel dal compito 7 ✅
- R5-17 `cargo clean` + due `time (… cargo test --locked)` nel Passo 10, e l'Atteso che manda la cifra al 15 con la data ✅
- R5-19 due `python - <<'EOF'` con `newline=""` e `assert` per `Cargo.toml` e `.gitignore`, `git ls-files --eol` e `git diff --stat` dopo ✅
- R5-20 `grep -rl … | wc -l` → 4, e il finto → 1 ✅
- R5-21 P-63: `grep -n 'non si carica da' docs/riferimenti.md` (rimisurato: la forma vecchia → 0 righe, la nuova → 1) ✅
- R5-22 «a TOOL with a lockfile of its own, under the root's `rust-toolchain.toml`» nel commento dettato e nello script (rimisurato: `rustup show active-toolchain` da una sottocartella → «overridden by … rust-toolchain.toml») ✅
- R3-18 (D88) D22 dentro il richiamo del capoverso 🔶 (la forma «o nel richiamo D41 del 12» che il rilievo ammetteva) ✅
- D76 «sub-project 2 design» ×2, «the sub-project 2 DST campaign (task 10)», «the sub-project 2 campaign (task 10)», «in this sub-project (ADR-0007)» ✅
- **Coerenza col 9 corretto** (decisione 72, non era nel registro): il pari del 12 è dichiarato «nella forma del 9» ma copiato **prima** di R4-6 e R4-7 — entrano la scadenza di parete di 5 s sul `connect` e il budget `WITH_A_PEER` (100_001) al posto di 4 000/8 000, con `WELCOME` = 5 riletto sul 7; la sonda dell'accoglienza mappa i **primi cinque** (`take(WELCOME)`), perché una `read` può portare anche il primo `Token` ✅
- ⚠️ **Non compilato:** il modulo delle sonde è riscritto sul modello letto, non eseguito (il compito 2 e il 7 non esistono ancora nel repo): chi esegue il 12 lo compila per primo e ogni rosso è una voce d'errata, come dice «Come si esegue» — il revisore R5 aveva compilato la forma vecchia
- Attrezzo: `patch_c12.py` accanto a questo file (tocca anche P-63 e questo registro)

### Compito 13 — NON RIVISTO IN PROFONDITÀ (R6 caduto): le righe note sono applicate, la revisione in profondità resta da fare
- R9b-11 (D80) `LayoutPack { view; layouts: Partial<Record<ViewName, SerializedDockview>> }`; `settle(layout)` **fonde** la vista aperta nel pacchetto tenuto (`saved`); `apply(api, view: ViewName, pack)` legge `pack?.layouts[view] ?? VIEWS[view]`; `unpack` legge le sole tre viste e rifiuta la forma vecchia `{view, layout}`; Interfaces riscritto; la sonda del merge in `stores.test.ts` e la sonda D80 in `frame.test.ts` (salvata sotto `home` → `view = "work"` mostra la spedita di Lavoro → `view = "home"` la salvata, e **zero** `SaveLayout`), con l'oracolo sugli **id dei pannelli** e non sul JSON canonico (E4: la taglia del viewport cambia le misure) ✅
- R9b-12 (D81) `beforeunload` salva solo se `!same(last, now)` e sposta `last`; il doc di `createDock` dichiara che il pannello attivo **è** disposizione (misurato sulla 8.2.0 dello spike, R9b-12; se la 8.3.1 differisce è errata); sonda nelle due direzioni in `frame.test.ts` — chiusa intatta → 0; chiusa dopo il `close()` di un pannello → 1, e l'evento bufferizzato non la ripete ✅
- ⛔ **C13-1 (coordinatore, scrivendo D80 — R6 caduto): il `Layout` dell'accoglienza non veniva MAI mostrato.** `createDock` chiamava `apply` una volta, in `onMounted`, cioè **prima** del `Hello` di `main.ts`; `layout.receive` aggiornava `state` e `view` e nessuno riapplicava. Compilava e passava le sonde. Rimedio, **D89**: il dock **segue lo store** — `watch([view, arrivals])` in `createDock`, `show()` che azzera la baseline; lo store conta gli arrivi che **non** sono l'eco del proprio `SaveLayout` (decisione 13: il core risponde con ciò che tiene; i byte si confrontano con quelli mandati); `Frame.switchTo` diventa `layout.view = view`; la sonda dell'eco in `stores.test.ts`; la sonda D80 di `frame.test.ts` parte da un pacchetto ricevuto **dopo** `createDock` ✅
- R10-9 Files: `views.test.ts` (che nessun passo dettava) → `generate-views.test.ts`, col perché ✅
- R10-10 Produces: `BigTab` (consumato e riscritto dal 14), `buildStamp` (D52), `type Phase`; la riga «ricensito» porta la data della seconda ricensione ✅
- R7-7 applicata **al 15** (la riga del 15, in questa ondata): «(compito **13**, Passo 12 — qui stava «14»)» ✅
- D75: la sola data dettata nel codice del 13 era «Richiamo del 2026-09-15, P-95» nel commento di `BigTab.ts` — tolta: in un file che nasce non c'è nulla da richiamare, resta il puntatore «P-95 of the part-2 plan, found at the pre-check of task 14»; le altre date del 13 sono fatti (P-2, E2, E4) e restano ✅
- D76 «sub-project 2» negli otto punti dei commenti dettati (`tokens.css` ×2, `Placeholder.vue`, `Strip.vue`, `registry.ts`, `dock.ts`, `main.ts` ×2) — non era in questa lista: applicata per coerenza coi compiti 5–12 ✅
- ⚠️ Il capoverso sotto il Passo 17 diceva «nessuna delle sue sonde monta una griglia»: ora `describe("the dock")` la monta e lo dice di sé — se il Passo 3 misura rosso, è quel `describe` a passare al revisore nel browser; due criteri di chiusura nuovi (D80/D89 e D81) ✅
- ⚠️ **Non compilato:** `layout.ts`, `dock.ts`, `Frame.vue` e le sonde sono riscritti sul modello letto (il compito 11 non esiste nel repo, e `dockview` sotto `jsdom` lo misura il Passo 3): chi esegue il 13 li compila per primi, e ogni rosso è una voce d'errata — come il modulo delle sonde del 12
- Attrezzo: `patch_c13.py` accanto a questo file (tocca anche la tabella D, la posizione, la riga del 15 e questo registro)
- R7-6 vale anche qui (stesso criterio `gui/src/**/*.test.ts`): applicata dall'ondata del 14 (`patch_c14.py`) ✅

### Compito 14
- ⚠️ **cascata di D80/D89 dal 13 corretto (decisione 78):** il `Frame.vue` ridettato al Passo 10 ridiffato contro il 13 di adesso — via `apply` e `unpack`, `switchTo` è `layout.view = view` col commento di D89; `api` resta, perché `moveActive` lo usa; la riga di prosa lo dice ✅
- R7-1 `String(…attrGet(…) ?? "")` ×2 e la riga sul tipo (`string | number | null` nel `.d.mts` spedito) ✅
- R7-2 `render` legge `stream.current?.text` al fotogramma; sonda «renders the text as it stands when the frame runs» con due `Token` prima di `await frame()` e `<p>primo secondo</p>` ✅
- R7-3 criterio: `deliverAll` porta anche `StaleBuild`, per costruzione — un `deliver("Accepted")` subito dopo ✅
- R7-4 criterio: la finestra si apre **al click** (la richiesta della fixture è già in `core.pending`), via il secondo `deliver("PermissionRequired")` ✅
- R7-5 «le **quattro** righe … fino a `reason the shape comes before the palette. */`» col `grep -c -F` → 1 prima di scrivere ✅
- R7-6 `grep -rcE … gui/src --include='*.test.ts' | grep -v ':0$'` → niente, col perché misurato — **anche nel 13**, che aveva lo stesso criterio ✅
- R7-8 `git add src/tokens/tokens.css` **prima** della mutazione (la forma di R5-11) e il capoverso riscritto: «si rilancia il primo script» non reggeva ✅
- R7-9 Files: `--stop`, il capoverso in testa, le regole della linguetta in coda ✅
- R7-10 `modules.test.ts`: `drawn(name)` monta il renderer e legge l'HTML — un costruito non contiene «placeholder», un non costruito porta `class="placeholder"`; `placeholderParams` nell'import ✅
- R7-12 criterio: il solo `git log … | wc -l` → 1 ✅
- R7-13 script `types or typings or exports`; prosa `.d.cts` di primo livello e `.d.mts` sotto `exports`, `typings` di `axe-core` ✅
- R7-14 richiami sulla riga 3 di Permessi e sulla riga 12 di Passi (costanti nel 2), nel Passo 15 riscritto ✅
- R7-26 «anche nei `grep`, con la data» nel Passo 15 e nel criterio ✅
- R10-8 Files: la stella polare (LF) con l'elenco dei richiami ✅
- R9b-4/R9a-13 (D87) un richiamo per modulo costruito — Chat, Stato, Permessi, Passi sul capoverso «Costruito dal **2**», Impostazioni sulla riga della corta — col nome del sorgente; la riga D87 del piano dice che la forma è questa e perché ✅
- R9b-5 la metà del 14: «✅ chiusa il <data>, compito 14 del piano della parte 2» sulla riga delle scorciatoie nelle registrate, nel Passo 15; **la metà dell'8** (la forma della transizione) è una riga sotto l'8 ✅
- ⚠️ **Passo 15 riscritto intero (decisione 77):** nove richiami, ancore prese DAL FILE per sezione e inizio di riga (`section`/`one`/`row`/`paragraph`), `assert` sul numero di righe invariato; i cinque comandi e l'Atteso 3/5/1/0/niente ✅
- D75: nulla da fare — le date dettate nel 14 sono misure (P-86, la 15.0.2) e `<data>` sta già nel Passo 15 ✅ · D76 «sub-project 2» nei tredici punti dei commenti dettati, con `assert` che nessun `milestone` sopravviva nel 14 ✅
- ⚠️ **Non compilato:** `Frame.vue`, `Chat.vue`, `markdown.ts` e le sonde toccate sono riscritti sul modello letto; chi esegue il 14 li compila per primi, ogni rosso è errata — come il 12 e il 13
- Attrezzo: `patch_c14.py` accanto a questo file (tocca anche il criterio del 13, la riga D87 e questo registro)

### Compito 15 — NON RIVISTO IN PROFONDITÀ (R8 caduto): le righe note sono applicate, la revisione in profondità resta da fare
- R10-1 `gate.sh` e il flusso sono `i/lf w/crlf` (rimisurato oggi: CR 97 = righe 97, 16 = 16): Files, Passo 1, Passo 8 (la riga `run` entra CRLF, Atteso CR = righe, `git ls-files --eol`), Passo 9 (il blocco `setup-node` convertito, Atteso CR = righe), Passo 10 (anche il commento entra CRLF) ✅
- R7-7 «(compito 13, Passo 12)» ✅ (applicata nell'ondata del 13)
- R7-11 il commento di `eslint.config.js` senza cifra («every `.vue` file here but `ViewBar`»), e il richiamo su P-99 col comando che conta (tredici, dodici a una parola; anche P-99 fuori dal 15) ✅
- R9a-12 (D88) il secondo richiamo del Passo 11, sulla riga «le scritte» della §8 (trovata per prefisso, in coda alla cella); Files, il titolo del passo e il criterio (`grep -c 'compito 15 del piano della parte 2'` → 2) ✅
- R9a-15 P-104: «§8» nel titolo e nella prima riga, «lo scrive il 15, Passo 11» nell'ultimo capoverso, col richiamo ✅
- R5-17 i due tempi del finto (`<data>`, `<tempo>` ×2) nel commento di `gate-gui.sh` (Passo 6), rimisurati al Passo 10 con `cargo clean` e due `time`, e il criterio `grep -c '<tempo>\|<data>'` su entrambi gli script ✅
- Attrezzo: `patch_c151617.py` accanto a questo file

### Compito 16 — NON RIVISTO IN PROFONDITÀ (R8 caduto): le righe note sono applicate, la revisione in profondità resta da fare
- R10-2 `gate.sh`, il flusso e l'audit sono `i/lf w/crlf` (rimisurato oggi), solo `gate-gui.sh` LF: Files, Passo 1 (Atteso), Passo 2 (blocco e riga vuota convertiti a CRLF, Atteso CR = righe), Passo 5 (il flusso scritto coi fine-riga del file da Python, via il `cp`), criterio ✅
- R9a-11 (D88) il richiamo in coda al capoverso «Ciò che la §8 non fa» del disegno del 2, in un sotto-passo del Passo 6 (ancora per frase contenuta, fine del capoverso), Files, `git add`, criterio (`grep -c 'compito 16 del piano della parte 2'` → 1) ✅
- D83 `cargo audit --file gui/fake-core/Cargo.lock` in `gate-gui.sh` subito dopo il `cargo test` del finto, prima del `cd gui` (Passo 4), le due direzioni anche sul lockfile del finto (Passo 3), Interfaces, Passo 1 (`grep -c 'cargo audit' scripts/gate-gui.sh`), criterio ✅
- Attrezzo: `patch_c151617.py`

### Compito 17 — NON RIVISTO IN PROFONDITÀ (R8 caduto): le righe note sono applicate, la revisione in profondità resta da fare
- R9b-15 criterio della roadmap: `grep -F 'parte-2-gui-minima.md' | sed 's/«[^»]*»//g' | grep -cE …` → 0, col perché ✅
- R9b-14 Passo 10: `grep -c 'dal compito 14 del piano della parte 2 (P-89, D56)'` → 1, col perché ✅
- R9b-3 (D86) Passo 5: la riga 2 della roadmap cambia titolo sul perimetro della §3; criterio `grep -c 'GUI minima (shell, chat, stato)'` → 0; il Passo 1 e il Passo 5 cercano la riga del piano per nome del file (R10-19 l'aveva già cambiata: «in scrittura dal 2026-09-11» non c'è più) ✅
- R9b-6 Passo 8-bis: la riga «codice e spec non toccati» della tabella dello stato e «Il prossimo passo» della stella, coi richiami ✅
- R9b-13 (D87, seconda metà) Passo 8-bis: i tre capoversi 🔶 confermati (sequenze → 7 con la precisazione `Stage`; §2 → 5, 7, 13; §3 → il piano, 11–13, 9, 7), ancore per frase contenuta contate oggi (una ciascuna); gli altri 🔶 restano, perché parlano di altri sotto-progetti ✅
- R9b-2 (D85) Passo 8-ter: `design/10` (rimisurato oggi `i/lf w/lf`, CR 0) — le due entità nel primo `erDiagram` coi campi letti da `record.rs`, via dal secondo, il richiamo sulla riga della tabella, `check-docs.sh`; quattro comandi nel criterio ✅
- R9a-16 Definizione di «fatto»: `git diff --name-only 42b50d8..HEAD -- …` contro l'unione delle liste Files, e `git diff --stat 42b50d8..HEAD -- docs/adr/` vuoto ✅
- D84 Passo 8: il processore a riposo (compito 9, Passo 14) in `riferimenti.md` con comando, sessanta secondi, numero senza soglia e data; anche i due tempi del finto (R5-17) ✅
- Files: la stella e `design/10`; le due liste `git ls-files --eol` (Passo 1 e 11) e l'Atteso; «otto file» → «i file della lista *Files*» (tre posti) ✅
- Attrezzo: `patch_c151617.py`

## Voci P nuove (P-117…): una per rilievo «fatto» confermato, raggruppate per compito; i «prosa» stanno nella tabella della dodicesima chiusura
