# Registro delle correzioni — revisione del piano intero, 2026-09-15 (sessione 12)

Fonte: 9 rapporti su 11 (R1 parziale: compiti 1–2; R3 parziale: compito 7; R5 completo: 11–12; R2, R4, R7, R9a, R9b, R10 completi).
Mancano: nulla — i compiti 15–17 sono stati rivisti da R8 il 2026-09-16; il compito 3 è rivisto da R11, l'8 da R12 e il 13 da R6, tutti il 2026-09-16.
Ogni rilievo qui è stato riletto dal coordinatore; quelli marcati ✔ sono stati rimisurati anche da lui.
**Ondata 17 (2026-09-16):** la revisione in profondità di **15, 16 e 17** è FATTA (R8, un revisore solo su Opus 5: ~436k token, 132 chiamate, ~43 minuti) e i suoi **ventiquattro** rilievi confermati sono applicati (✅) con `patch_c151617b.py`, insieme al **criterio di chiusura del compito 7**, che non esisteva (R8-21), e a **D91**; il rapporto è `R8-report.md` accanto. Le due misure che il piano ora porta — l'analizzatore TypeScript e `ignoreText` — sono state rifatte dal coordinatore sulla catena `eslint` che R8 aveva installato, nelle due direzioni. Nel registro non resta nessun ⬜. **Stato al 2026-09-15 (sessione 13, tredicesima chiusura):** testa e compiti 1–10 applicati (✅) — l'8 nelle sole due correzioni note, la sua revisione in profondità resta da fare; tutto il resto ⬜. **Sessione 14 (2026-09-15):** compiti 11 e 12 applicati (✅). **Sessione 15 (2026-09-15):** compito 13 applicato (✅) — con **D89** nuova (il dock segue lo store), R7-7 applicata al 15, e la cascata sul `Frame.vue` del 14 registrata come riga del 14. **Sessione 15, ondata 11 (2026-09-15):** compito 14 applicato (✅) — resta all'8 la sua metà di R9b-5. **Ondata 12 (2026-09-15):** compiti 15, 16 e 17 applicati (✅) in una ondata sola. **Ondata 13 (2026-09-15):** le tre righe note del 3 applicate (✅); resta ⬜ la sola metà dell'8 di R9b-5, con la sua revisione in profondità. **Ondata 14 (2026-09-16):** la revisione in profondità del 3 è FATTA (R11, un revisore solo su Opus 5: 283k token, 69 comandi, 22 minuti) e i suoi tredici rilievi sono applicati (✅) con `patch_c3b.py`; il rapporto è `R11-report.md` accanto; resta ⬜ la sola metà dell'8 di R9b-5, con la revisione in profondità dell'8. **Ondata 15 (2026-09-16):** la revisione in profondità dell'8 è FATTA (R12, un revisore solo su Opus 5: ~297k token, 93 chiamate, 98 comandi, ~29 minuti) e i suoi tredici rilievi confermati sono applicati (✅) con `patch_c8b.py`, insieme alla metà dell'8 di R9b-5 col testo che R12 propone; il rapporto è `R12-report.md` accanto; le due misure che il piano ora porta (R12-1, R12-2) sono state rifatte dal coordinatore sul modello compilato di R12 con la sonda nella forma nuova. Nel registro non resta nessun ⬜. **Ondata 16 (2026-09-16):** la revisione in profondità del 13 è FATTA (R6, un revisore solo su Opus 5: ~401k token, 125 chiamate, ~40 minuti) e i suoi diciassette rilievi confermati sono applicati (✅) con `patch_c13b.py`, con due voci del coordinatore dalla sua tabella di copertura, il richiamo in P-97, i riallineamenti nel 14 e nel 15, **D90** e una riga nelle voci aperte; il rapporto è `R6-report.md` accanto; l'ondata intera è stata applicata al modello compilato di R6 e misurata prima di essere dettata. Nel registro non resta nessun ⬜.

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

### Compito 3 — RIVISTO IN PROFONDITÀ da R11 il 2026-09-16 (Opus 5; R1 era caduto dopo il 2): tredici rilievi, tutti applicati
- R10-16 l'etichetta della sola voce che ne mancava (`*.bin`: binari, niente fine-riga) ✅ · D75 `<data>` nei due `DATED RECALL` di `wire/ipc.rs` e nel criterio ✅ · D76 «sub-project 2» nei cinque punti dei commenti dettati (il registro diceva «sei»: contati da R11); i due «milestone 6» restano, perché sono il Traguardo 6 del SP1 ✅ · Attrezzo: `patch_c3.py`
- R11-1 Passo 1: `grep -cE '^#\[test\]'` — due `#[test]` del file stanno dentro commenti (9 contro 7, rimisurato dal coordinatore) ✅ · R11-2 Passo 2: `use crate::time::Millis;` con la ragione (`E0433` nella copia) ✅ · R11-3 Passo 5: `altered[0] = IpcMessage::Hello(build_stamp())` (`E0423`: `BuildStamp(0)` da fuori la crate; nessun `new`, come il Passo 2 decide) ✅ · R11-4 Passo 5: la riga `use` senza `BuildStamp`, `DegradationReport`, `PolicyReport`, `StepSummary` (zero avvisi, provato da R11) ✅ · R11-5 Passo 6: `cat … | tr -cd '\r' | wc -c` con la controprova (la redirezione col glob rendeva 0 sempre) ✅ · R11-6 Passo 7: la mappa confrontata sull'ultima riga con `format!("stamp {:#018x}", build_stamp().get())`, la terza metà nella prosa, e **G9** nel Passo 8 come seconda direzione (con G8 attiva prima nulla andava rosso; D52 consegna la mappa alla SPA) ✅ · R11-7 Passo 9 (b): «The variants added today put» — l'ultima casa di P-35, in inglese; `grep -ci eleven` accanto a `grep -c undici` nella cura di P-35 ✅ · R11-8 Passo 3: l'eccezione dichiarata nel doc di `stamp_set` (`routing_degraded` al default, P-34 vince) ✅ · R11-9 Passo 6: il commento non attribuisce più una regola a `gate.sh` ✅ · R11-10 Passo 8: G7 toglie l'**ultima** variante (`Verdict`, `[13]`, `13-verdict.bin` e `.json`) e non `Steps` in mezzo — la via (b), nessuno slittamento ✅ · R11-11 testa: via il numerale «dieci tipi», il comando al suo posto ✅ · R11-12 Passo 9 (a): la terza metà del richiamo («returns a bench fake» è falso dal compito 2) e `sub-project 7` (D76) ✅ · R11-13 Passo 10: «il sotto-progetto 7» nel messaggio di commit ✅ · Passo 10: il criterio conta anche G9 ✅ · Le P-117… per questi rilievi restano da scrivere (passo 5 della quindicesima chiusura) · Attrezzo: `patch_c3b.py`

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

### Compito 8 — RIVISTO IN PROFONDITÀ da R12 il 2026-09-16 (Opus 5; R3 era caduto dopo il 7): tredici rilievi confermati, tutti applicati, e la metà di R9b-5
- R9b-5, la metà dell'8: il Passo 12 riscritto intero nella forma del Passo 15 del 14 — UN solo script Python per i due disegni, `section`/`row`, ancore dal file, `assert` sul numero di righe — col sotto-passo che scrive «✅ **chiusa il <data>, compito 8 del piano della parte 2**» in coda alla cella «Chiusore proposto» (l'ultima delle due colonne di quella tabella) della riga *«la forma con cui la transizione di policy si rilegge»*; Files, e il criterio `grep -c` → 1 ✅
- R12-1 Passo 4: la prima sonda nella forma della gemella `a_permission_does_not_put_a_step_in_doubt` — intento, nota `Policy`, esito, **una seconda nota dopo l'esito** — perché `MemoryJournal::note` rifiuta un passo senza intento (`OutOfOrder`, la sonda panicava sull'`expect`) ✅ · R12-2 Passo 3: il commento dettato riporta le risposte **misurate** (`enter`: `RunAgain` contro `SuspendAndAsk`, e il passo finito riaperto dalla nota dopo l'esito; `leave`: `[]` contro il dubbio aperto; `arbiter_policy` verde sotto entrambe), rimisurate dal coordinatore sul modello con la sonda di R12-1 ✅ · R12-3 Passo 7: la prima sonda passa per `.map(|policy| policy.name())` come le tre sorelle (`VramPolicy` senza `derive`, `E0369`/`E0277`), e nessun `derive` si aggiunge ✅ · R12-4 Passi 4, 5 e 7: `steps_in_doubt` e `policy_now` chiamate **nude** come le gemelle, `policy_now` entra nell'`use kernel::arbiter::{…}` del banco (i banchi importano elementi, non moduli: `E0433` ×7) ✅ · R12-5 Passo 6: l'`use crate::record::{…}` di `arbiter/mod.rs` dettato per esteso con `PolicyDetail` (`E0422`) ✅ · R12-6 Files, Passo 8 e Passo 10: il `match detail` di `frozen_bytes.rs` — il quinto sito, su `Detail` — riceve `Detail::Policy(_) => {}` (`E0004`); il richiamo in P-45 ✅ · R12-7 Passi 8(a) e 10: l'`use kernel::record::{…}` di `record_shape.rs` e di `frozen_bytes.rs` riceve `PolicyDetail` ✅ · R12-8 Passo 3: il *Trova* è la seconda riga intera del capoverso (`matches each. Each writer carries its OWN`), il *Sostituisci* la spezza attorno al richiamo — il blocco «intero» col `/// ` d'apertura non esisteva nel file ✅ · R12-9 Files: via «e **D1** (**D25**)», resta la riga 8 della posizione ✅ · R12-10 Passo 12: riscritto nella forma del Passo 15 del 14 (sopra), coi tre `grep -c` nel criterio di chiusura ✅ · R12-11 Passo 9: la citazione della mappa spezzata su due righe → il `grep -F` sulla frase che sta su una riga ✅ · R12-12 Passo 5(a): via il numerale «tre righe», resta l'ancora ✅ · R12-13 Files e Passo 12: il richiamo sulla riga «il giornale» della §5 del 2 (*«il passo B di `set_policy` com'è»*) ✅ · R12-14…R12-21 NON RIPRODOTTO: nulla da fare, e il coordinatore non le ricerca · Le P-117… per questi rilievi restano da scrivere · Attrezzo: `patch_c8b.py`
- R10-12 (D77) E nude nella prosa: `E66` ed `E112` (lezione delle date) ed `E50` (l'oracolo) col nome del piano del Traguardo 6 ✅ · D75/D76: nulla da fare — i richiami dettati dell'8 portano già `<data>`, e «milestone 5 task 9» è il Traguardo 5 del SP1 ✅ (verificate da R12-20: reggono)

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

### Compito 13 — RIVISTO IN PROFONDITÀ da R6 il 2026-09-16 (Opus 5; il primo R6 era caduto): diciassette rilievi confermati, tutti applicati, e due del coordinatore
- R9b-11 (D80) `LayoutPack { view; layouts: Partial<Record<ViewName, SerializedDockview>> }`; `settle(layout)` **fonde** la vista aperta nel pacchetto tenuto (`saved`); `apply(api, view: ViewName, pack)` legge `pack?.layouts[view] ?? VIEWS[view]`; `unpack` legge le sole tre viste e rifiuta la forma vecchia `{view, layout}`; Interfaces riscritto; la sonda del merge in `stores.test.ts` e la sonda D80 in `frame.test.ts` (salvata sotto `home` → `view = "work"` mostra la spedita di Lavoro → `view = "home"` la salvata, e **zero** `SaveLayout`), con l'oracolo sugli **id dei pannelli** e non sul JSON canonico (E4: la taglia del viewport cambia le misure) ✅
- R9b-12 (D81) `beforeunload` salva solo se `!same(last, now)` e sposta `last`; il doc di `createDock` dichiara che il pannello attivo **è** disposizione (misurato sulla 8.2.0 dello spike, R9b-12; se la 8.3.1 differisce è errata); sonda nelle due direzioni in `frame.test.ts` — chiusa intatta → 0; chiusa dopo il `close()` di un pannello → 1, e l'evento bufferizzato non la ripete ✅
- ⛔ **C13-1 (coordinatore, scrivendo D80 — R6 caduto): il `Layout` dell'accoglienza non veniva MAI mostrato.** `createDock` chiamava `apply` una volta, in `onMounted`, cioè **prima** del `Hello` di `main.ts`; `layout.receive` aggiornava `state` e `view` e nessuno riapplicava. Compilava e passava le sonde. Rimedio, **D89**: il dock **segue lo store** — `watch([view, arrivals])` in `createDock`, `show()` che azzera la baseline; lo store conta gli arrivi che **non** sono l'eco del proprio `SaveLayout` (decisione 13: il core risponde con ciò che tiene; i byte si confrontano con quelli mandati); `Frame.switchTo` diventa `layout.view = view`; la sonda dell'eco in `stores.test.ts`; la sonda D80 di `frame.test.ts` parte da un pacchetto ricevuto **dopo** `createDock` ✅
- R10-9 Files: `views.test.ts` (che nessun passo dettava) → `generate-views.test.ts`, col perché ✅
- R10-10 Produces: `BigTab` (consumato e riscritto dal 14), `buildStamp` (D52), `type Phase`; la riga «ricensito» porta la data della seconda ricensione ✅
- R7-7 applicata **al 15** (la riga del 15, in questa ondata): «(compito **13**, Passo 12 — qui stava «14»)» ✅
- D75: la sola data dettata nel codice del 13 era «Richiamo del 2026-09-15, P-95» nel commento di `BigTab.ts` — tolta: in un file che nasce non c'è nulla da richiamare, resta il puntatore «P-95 of the part-2 plan, found at the pre-check of task 14»; le altre date del 13 sono fatti (P-2, E2, E4) e restano ✅
- D76 «sub-project 2» negli otto punti dei commenti dettati (`tokens.css` ×2, `Placeholder.vue`, `Strip.vue`, `registry.ts`, `dock.ts`, `main.ts` ×2) — non era in questa lista: applicata per coerenza coi compiti 5–12 ✅
- ⚠️ Il capoverso sotto il Passo 17 diceva «nessuna delle sue sonde monta una griglia»: ora `describe("the dock")` la monta e lo dice di sé — se il Passo 3 misura rosso, è quel `describe` a passare al revisore nel browser; due criteri di chiusura nuovi (D80/D89 e D81) ✅
- ✅ **Compilati il 2026-09-16:** R6 ha ricostruito il modello dell'11 e del 13 (`C:\Users\zagor\AppData\Local\Temp\probe-R6\gui`, percorso corto perché lo scratchpad supera MAX_PATH per `npm` e `vite`) e il coordinatore vi ha applicato l'ondata 16 intera prima di dettarla: `vue-tsc` zero errori, 23 sonde verdi e 1 saltata, il generatore verde con tre viste senza `"missing"`, `vite build` col timbro decimale nel bundle, `npm run dev` che serve la pagina col timbro in `/@vite/env` e zero errori in console; le due sonde nuove uccise ciascuna dalla propria mutazione (qui stava «Non compilato … chi esegue il 13 li compila per primi»)
- Attrezzo: `patch_c13.py` accanto a questo file (tocca anche la tabella D, la posizione, la riga del 15 e questo registro)
- R7-6 vale anche qui (stesso criterio `gui/src/**/*.test.ts`): applicata dall'ondata del 14 (`patch_c14.py`) ✅
- R6-1 Passo 2 e Files: `@types/node` **24.13.5** fra le `devDependencies` (la major segue Node: `npm view @types/node@24 version | tail -1`, non `latest`, che indica la 22) e `"types": ["vite/client", "node"]` in `gui/tsconfig.json` con Python — senza, `npm run build` esce 2 con otto `TS2307`/`TS2591` (sei `import` da `node:` nel 13, i primi del piano) ✅ · R6-2 Passo 13: `setActivePinia(createPinia())` in un `beforeAll` di `generate-views.test.ts`, col perché (`VueContent` monta ogni pannello senza pinia, e qui non c'è una root app) ✅ · R6-3 Passo 6 riscritto (**D90**): il timbro lo legge `vite.config.ts` in Node e lo consegna con `define`; `stamp.ts` lo dichiara e non importa più la mappa; Files, Interfaces, e il criterio che lo prova nei tre mondi (bundle, `vitest`, `/@vite/env` in sviluppo) — misurato dal coordinatore sul modello di R6 e nel browser: pagina aperta, zero errori in console ✅ · R6-4 Passo 5: le diciotto chiavi `modules.*` dettate per esteso coi nomi della §1 («Knowledge base» per la riga «Knowledge base e Nucleo a pagina intera»); il capoverso del Passo 12 rimanda al 5; la riga *Consumes* del 15 dice «Passo 5» ✅ · R6-5 Passo 13: `node --input-type=module -e "import * as v from 'vitest'; …"` ✅ · R6-6 Passo 18: `git add src/frame/Band.vue` prima della mutazione — la forma di R5-11 e del Passo 3 del 14, non il Python che R6 proponeva (coerenza col repo) — e l'Atteso che dice perché ✅ · R6-7 Passo 3: «Passo 17» nelle due celle e nel capoverso 📌 ✅ · R6-8 Passo 3: la terza riga della tabella, `gui/src/jsdom-setup.ts` (Create) montato da `setupFiles`, il richiamo che dice che cosa la sonda misura davvero; il richiamo in P-97 sull'attribuzione ✅ · R6-9 Passo 13: il capoverso di `views/index.ts` dice che l'`as` è un'asserzione portante che coglie l'incompatibilità grossolana, e che la prima sonda di `frame.test.ts` copre il resto ✅ · R6-10 Passo 11: il commento di `BigTab.ts` dichiara la trappola per il 14 invece di affermare che i due eventi sono fermati ✅ · R6-11 Passo 4: il capoverso di `tokens.css` già nella forma di D53 (il testo che il 14 dettava); il Passo 3 del 14 lo **verifica** e non lo riscrive, la riga *Files* del 14 perde «il capoverso in testa» ✅ · R6-12 Passo 1: «tre righe `/gui/`», e da chi ✅ · R6-13 criterio: `grep -rc … --include='*.ts' --include='*.vue' | grep -v ':0$'` → niente ✅ · R6-14 Passo 12: `Placeholder.vue` dice «niente ancora» sul nucleo (`knowledge`, riga Home de «Il modello della GUI») e ancora chi lo riempie; la chiave `placeholder.nucleus` al Passo 5; la sonda in `frame.test.ts` e il criterio del browser — misurato nelle due direzioni ✅ · R6-15 Passo 9: «the opening paragraph of §2» ✅ · R6-16 Passo 15: il velo (`.drawer-overlay`: `fixed`, `inset: 0`, `rgb(0 0 0 / 0.45)`, `z-index: 100`) e `z-index: 101` sul cassetto; via la classe `drawer-open` senza regola — visto nel browser ✅ · R6-17 Passo 14: `isBuilt` nel registro (Interfaces) e `!isBuilt(panel.id) &&` in `apply`, col perché; la terza sonda di `describe("the dock")`; il criterio `grep -c '"missing"'` sulle viste → 0 — misurato nelle due direzioni ✅ · R6-18…R6-26 NON RIPRODOTTO: nulla da fare, e il coordinatore non le ricerca · Attrezzo: `patch_c13b.py`
- ⛔ **C13-2 (coordinatore, dalla tabella di copertura di R6): «il resto spento» della §6a non è prodotto da nessun passo** — registrata nella tabella *«Le voci aperte che questo piano SA, e non chiude»* con l'A/B per il proprietario (consiglio A: lo stato vuoto di oggi) ✅
- Le P-117… per questi rilievi restano da scrivere (decisione 94)

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
- R6-11 (2026-09-16, dalla revisione in profondità del 13): il capoverso in testa a `tokens.css` lo scrive già il 13 nella forma di D53; il Passo 3 lo verifica col `grep -c -F` e non lo riscrive, e la riga *Files* perde «il capoverso in testa (Passo 3)» (`patch_c13b.py`) ✅

### Compito 15 — RIVISTO IN PROFONDITÀ da R8 il 2026-09-16 (Opus 5): ventiquattro rilievi confermati su tre compiti, tutti applicati
- R10-1 `gate.sh` e il flusso sono `i/lf w/crlf` (rimisurato oggi: CR 97 = righe 97, 16 = 16): Files, Passo 1, Passo 8 (la riga `run` entra CRLF, Atteso CR = righe, `git ls-files --eol`), Passo 9 (il blocco `setup-node` convertito, Atteso CR = righe), Passo 10 (anche il commento entra CRLF) ✅
- R7-7 «(compito 13, Passo 12)» ✅ (applicata nell'ondata del 13)
- R7-11 il commento di `eslint.config.js` senza cifra («every `.vue` file here but `ViewBar`»), e il richiamo su P-99 col comando che conta (tredici, dodici a una parola; anche P-99 fuori dal 15) ✅
- R9a-12 (D88) il secondo richiamo del Passo 11, sulla riga «le scritte» della §8 (trovata per prefisso, in coda alla cella); Files, il titolo del passo e il criterio (`grep -c 'compito 15 del piano della parte 2'` → 2) ✅
- R9a-15 P-104: «§8» nel titolo e nella prima riga, «lo scrive il 15, Passo 11» nell'ultimo capoverso, col richiamo ✅
- R5-17 i due tempi del finto (`<data>`, `<tempo>` ×2) nel commento di `gate-gui.sh` (Passo 6), rimisurati al Passo 10 con `cargo clean` e due `time`, e il criterio `grep -c '<tempo>\|<data>'` su entrambi gli script ✅
- Attrezzo: `patch_c151617.py` accanto a questo file
- R6-4 (2026-09-16, dalla revisione in profondità del 13): la riga *Consumes* dice «compito 13, Passo 5» — le chiavi `modules.*` sono dettate per esteso al Passo 5, non al 12 (`patch_c13b.py`) ✅

- R8-1 (bloccante) Passi 2 e 3: `@typescript-eslint/parser` **8.70.0** fra le devDependencies, nel dizionario del `npm view` e nella frase delle versioni; `import tsParser` e il blocco `harness/ts-in-vue` (`files: ["**/*.vue"]`, `languageOptions.parserOptions.parser`) **prima** di `harness/settings`, col commento; il capoverso «Nessun `@typescript-eslint/parser`» riscritto (resta vero per i `.ts`, falso per i `.vue`); la riga 4 della tabella del Passo 4 dice che senza il blocco rende 1 e l'eccezione è **vacua**; una riga nel criterio ✅
- R8-2 (bloccante) **D91**: `"@intlify/vue-i18n/no-raw-text": ["error", { ignoreText: [":", "—"] }]` col commento, e il capoverso del Passo 4 che dichiara la prima mutazione **contro-prova** di `ignoreText` — misurato dal coordinatore sulla catena di R8: sette errori senza la lista, zero con, e `{{ a }}: ciao — {{ a }}` ancora rosso ✅
- R8-3 (bloccante) Passi 9 e 11: `export SCRATCH="$(cygpath -w /tmp)"` e `os.path.join(os.environ["SCRATCH"], …)` nei tre siti che Python legge, `import io, os`, col perché accanto ✅
- R8-4 (bloccante) Passo 1: `time bash scripts/gate.sh > /dev/null 2>&1` in coda al blocco, e il Passo 10 dice che il terzo `<tempo>` viene da lì ✅
- R8-5 Interfaces *Consumes*: `Chat.vue` torna al compito **14**, le chiavi restano al 13 Passo 5 ✅
- R8-6 (bloccante) Passo 3: il commento dice `(task 13 too, step 5)` ✅
- R8-7 Passo 12: la mutazione che porta il cancello a **`GATE RED`**, revocata, e la riga nel criterio ✅
- R8-8 Passo 1: le otto righe di `.gitignore` vengono da **tre** commit (`01694e3`, `8fc9696`, `d5eb0b8`), col perché il comando di P-102 ne indicava uno ✅
- R8-9 **P-101**: «Passo 4» → «Passo 2», e la conclusione ristretta ai `.ts` ✅
- R8-10 Passo 11: il **terzo** richiamo, sulla riga `scripts/gate-gui.sh` della §8 (la catena si allunga a cinque e poi a sette comandi), con la sua ancora e il suo `assert` ✅
- Attrezzo: `patch_c151617b.py`

### Compito 16 — RIVISTO IN PROFONDITÀ da R8 il 2026-09-16 (Opus 5): ventiquattro rilievi confermati su tre compiti, tutti applicati
- R10-2 `gate.sh`, il flusso e l'audit sono `i/lf w/crlf` (rimisurato oggi), solo `gate-gui.sh` LF: Files, Passo 1 (Atteso), Passo 2 (blocco e riga vuota convertiti a CRLF, Atteso CR = righe), Passo 5 (il flusso scritto coi fine-riga del file da Python, via il `cp`), criterio ✅
- R9a-11 (D88) il richiamo in coda al capoverso «Ciò che la §8 non fa» del disegno del 2, in un sotto-passo del Passo 6 (ancora per frase contenuta, fine del capoverso), Files, `git add`, criterio (`grep -c 'compito 16 del piano della parte 2'` → 1) ✅
- D83 `cargo audit --file gui/fake-core/Cargo.lock` in `gate-gui.sh` subito dopo il `cargo test` del finto, prima del `cd gui` (Passo 4), le due direzioni anche sul lockfile del finto (Passo 3), Interfaces, Passo 1 (`grep -c 'cargo audit' scripts/gate-gui.sh`), criterio ✅
- Attrezzo: `patch_c151617.py`

- R8-11 (bloccante) criterio: `grep -cE '^[^#]*audit-level' scripts/gate-gui.sh` → 0 — il `grep` di prima rendeva **1** sul commento che il Passo 4 scrive; provato nelle due direzioni ✅
- R8-12 criterio: `tail -2` più `grep -n 'npm run lint\|npm audit'` (due righe, `lint` prima), perché `tail -3` non arriva a `npm run lint` ✅
- R8-13 (bloccante) Passi 2, 5 e 6: i cinque siti `/tmp` che Python legge passano da `cygpath` ✅
- R8-14 Passo 2: i due tempi diventano `<tempo>` misurati al Passo 3, e il commento distingue la **prima** corsa (clona ~45 MB) dal fetch incrementale; l'argomento di D69 non cambia ✅
- R8-15 Passo 6: il richiamo di X-1 chiude anche il numerale — la sonda `#[cfg(unix)]` è **una**, col comando ✅
- R8-16 Passo 1: «più di zero» per `gate-gui` (la riga `run` più il commento del Passo 10 del 15), «uno» per `setup-node` ✅
- R8-17 criterio: il divieto di `-n` si conta su **due** file, perché da D83 i siti sono due ✅
- Attrezzo: `patch_c151617b.py`

### Compito 17 — RIVISTO IN PROFONDITÀ da R8 il 2026-09-16 (Opus 5): ventiquattro rilievi confermati su tre compiti, tutti applicati
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

- R8-18 (bloccante) criterio: `grep -c '^⏭️ \*\*IL PROSSIMO PASSO'` → 1 al posto dell'`awk` su `⏭️`, che rende **tre** righe e non può mai renderne una; e il Passo 3 riscrive **anche l'elenco numerato** sotto il puntatore ✅
- R8-19 (bloccante) Passo 9: `ls crates/kernel/tests/frozen/*.cbor | wc -l`, perché senza `*.cbor` conta anche `record_v1.map` e la Definizione di «fatto» direbbe nove per otto ✅
- R8-20 (bloccante) Passo 1: `grep -nE '^(#### |\*\*)Criterio di chiusura'` — l'ancora `^\*\*` ne vedeva **dieci**, perché i compiti 1–6 usano `#### ` — e il numerale «sedici» **tolto** ✅
- R8-21 il **compito 7** riceve il proprio criterio di chiusura, nella forma dei fratelli (sonde, `Core::ipc` che non esiste, il ramo `Request` che non chiama l'arbitro, la funzione registrata, i sette richiami, i `compile_fail`, i fine-riga, il cancello): era l'unico dei diciassette senza ✅
- R8-22 Passo 1 e Passo 5: le righe della roadmap sono **tre**, e la riga del piano si trova con `grep -n '^| \[Sotto-progetto 2 · parte 2'`, che ne rende una ✅
- R8-23 Passo 8-bis e criterio: `assert DATE != "<data>"` in testa allo script, e **un** comando per tutte le case (`grep -rn '<data>\|<tempo>' docs/ --include='*.md'`, escluso il piano, che le detta) ✅
- R8-26 **Passo 8-quater**: il richiamo sulla riga «Codice di prodotto» della §10 del disegno del 2 — il gemello di quella che il Passo 8-bis corregge nella stella — e la riga *Files*; la §10 resta il verbale della sessione dei disegni e **non** diventa il diario ✅
- R8-24 e R8-25 NON RIPRODOTTO: nulla da fare, e il coordinatore non le ricerca (R8-25 dice che la §6 della stella nomina «Dove va cosa» mentre D14 della parte 1 e il precedente vero dicono «Specifiche»: il piano ha ragione)
- Attrezzo: `patch_c151617b.py`

## Voci P nuove (P-117…): una per rilievo «fatto» confermato, raggruppate per compito; i «prosa» stanno nella tabella della dodicesima chiusura. ⚠️ **RICHIAMO DEL 2026-09-16, decisione 94 della diciassettesima chiusura del piano:** la forma — una P per rilievo, o una per compito che rimanda al rapporto e a questo registro — è del **proprietario, in A/B**, prima che una P nuova si scriva; i rilievi «fatto» confermati si contano col `grep -cE` scritto in quella chiusura (centotrenta il 2026-09-16, R6 e R8 mancanti), non con un `awk` per colonna, che sbaglia sulle celle con una barra escapata
