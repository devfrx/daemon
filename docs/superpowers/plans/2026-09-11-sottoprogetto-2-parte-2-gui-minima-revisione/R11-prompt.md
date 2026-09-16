# R11 — Compito 3: lo schema che cresce — le varianti nuove, i gemelli del filo, le fixture e il timbro di build

Rapporto: `<scratchpad>/review/R11-report.md`. Vincoli: `constraints.md` **col suo richiamo del 2026-09-16 in testa** (letto prima di
questo). Data del rapporto: 2026-09-16.

## Perimetro
`## Compito 3:` del piano — per intero, per intestazione (`awk '/^## Compito 3:/{f=1} /^## Compito 4:/{exit} f' <piano>`), a blocchi
di 200 righe. Era il terzo perimetro di R1, che è morto dopo il 2: **nessuno l'ha mai riletto in profondità**; tre sole righe note
sono applicate (registro `ledger.md`, sezione «Compito 3»).

## Sezioni dei disegni assegnate (ritrova la riga col `grep -n` sull'intestazione prima di leggere)
- `<disegno2>`: `### §4 — Lo schema` (`grep -n '^### §4' <disegno2>`), le due tabelle; la riga «lo schema» della tabella degli
  artefatti della §8.
- `<stella>`: la **sequenza 1** di `### La GUI dentro — le tre sequenze`; la tabella di `### §3` righe 2 e 3.
- Il codice di **oggi**, per intero: `crates/kernel/src/wire/ipc.rs` e `crates/kernel/tests/ipc_wire.rs` (esiste già:
  `ls crates/kernel/tests/`; il 3 lo **modifica**, Files dice `Modify`); il timbro di build in `crates/kernel/src/ports/ipc.rs`
  (`grep -rn 'BUILD STAMP\|build_stamp' crates/kernel/src/`).

## Su che cosa insistere, oltre alle sei dimensioni
- (il paragrafo di R1 sul compito 3, testuale) L'enum `IpcMessage` **di oggi** — varianti e indici (`grep -n '#\[n(' crates/kernel/src/wire/ipc.rs`
  o la forma che usa): le varianti nuove prendono indici **nuovi** e nessun indice si riusa; ogni tipo con campi ha il **gemello** (D11) e i
  tipi chiusi passano tali e quali — controlla contro i tipi veri del kernel (`Trust`, `Permission`, `VramPolicy`, `Mib`, `StepId`, `ClientId`, …:
  esistono, con quei campi?). Il generatore delle fixture scrive `.bin`, `.json`, `ipc_v1.map` (D35) in `gui/schema/fixtures/` (D12, P-18): la
  cartella e i nomi dei file coincidono con ciò che i compiti **11** e **13** leggono (scheletro, *Interfaces* di 11 e 13; **D52**: l'ultima riga
  di `ipc_v1.map` porta il timbro in esadecimale — il generatore dettato lo scrive davvero così?). `build_stamp`: com'è calcolato oggi e come
  cambia. I record congelati oggi sono **sei** (P-4: `ls crates/kernel/tests/frozen/`); chi lancia `ipc_wire.rs` nel cancello? `StepSummary`
  a **tre** campi (D56). Ogni «Atteso» su un file futuro: sul **modello** estratto dal piano.
- ⛔ **COMPILA il modello.** Il codice Rust dettato dal 3 tocca `wire/ipc.rs` e `tests/ipc_wire.rs`: nella tua cartella di prova
  (`<scratchpad>/review/probe-R11/ws/`) copia il workspace — `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `crates/` senza `target/`
  (le crate ereditano `edition.workspace`: senza la radice non compilano) — applica ai file copiati le modifiche che i Passi dettano
  (i sorgenti sono **CRLF**: Python con `newline=""`, mai `sed -i`), e LÀ `cargo build --locked` e `cargo test --locked -p kernel --test ipc_wire`
  con `CARGO_TARGET_DIR=C:\Users\zagor\AppData\Local\Temp\probe-R11-target` (percorso corto: lo scratchpad è lungo). Mai nel repo. Ogni
  rosso è un rilievo `fatto`, `sì`, col messaggio intero del compilatore. Se un Passo detta solo un frammento e la ricomposizione è
  ambigua, di' che cosa hai scelto.
- Le **tre righe applicate dal registro**: R10-16 (`*.bin` binari in Files), D75 (`<data>` nei due `DATED RECALL` di `wire/ipc.rs` e nel
  criterio), D76 («sub-project 2» nei sei commenti dettati; i due «milestone 6» restano perché sono il Traguardo 6 del SP1). Verificale:
  nel testo dettato `grep -c 'DATED RECALL, <data>'` → 2? nessun «milestone 2» sopravvive nel 3 (`awk` sul compito, poi `grep -ci 'milestone 2'` → 0)?
- Le **P** che nominano il 3 — almeno P-4, P-16, P-17, P-18, P-62 e quelle che `grep -n 'compito 3\b\|del 3\b\|il 3\b' <scratchpad>/review/p-titles.md`
  e la lettura delle voci trovano: rimisurale tutte.
- **Consumatori**: il blocco *Interfaces* del 3 nomina chi lo usa: `grep -n` di ogni nome prodotto nel piano e confronto firma per firma,
  in particolare i gemelli TypeScript dell'11 (`src/schema/messages.ts` nel Passo dell'11 di **oggi**; `probe-R5/gui-ts5/src/schema/` della
  sessione vecchia è una forma **vecchia**, non un oracolo) e il `decode` che il 7, il 9 e il 12 chiamano.
- **Numeri di compito**: ogni «il 7», «il 9», «l'11», «il 12», «il 13», «il 14» nel 3 contro la tabella della posizione (scheletro [B]).
