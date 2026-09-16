# R12 — Compito 8: la specie `Policy` del giornale, e la policy riletta dal giornale

Rapporto: `<scratchpad>/review/R12-report.md`. Vincoli: `constraints.md` **col suo richiamo del 2026-09-16 in testa** (letto prima di
questo). Data del rapporto: 2026-09-16.

## Perimetro
`## Compito 8:` del piano — per intero, per intestazione (`awk '/^## Compito 8:/{f=1} /^## Compito 9:/{exit} f' <piano>`), a blocchi
di 200 righe. Era il secondo perimetro di R3, morto dopo il 7: **nessuno l'ha mai riletto in profondità**; due sole righe note sono
applicate (registro `ledger.md`, sezione «Compito 8»), e una riga resta ⬜: la metà dell'8 di R9b-5, qui sotto.

## Sezioni dei disegni assegnate (ritrova la riga col `grep -n` prima di leggere)
- `<disegno2>`: `### §5` (`grep -n '^### §5' <disegno2>`) — la policy e la sua rilettura.
- `<stella>`: la tabella delle decisioni **registrate**, la riga che comincia con
  `| 🔶 **nata alla quarta ripresa, 2026-09-08** — la forma con cui la transizione di policy si rilegge dal giornale`
  (`grep -n -F 'la forma con cui la transizione di policy si rilegge' <stella>` → **una** riga, il 2026-09-16 la 963).
- `docs/adr/0006-*.md`: le prime 40 righe (il rimando datato in testa: la policy corrente come proiezione del giornale).
- Il codice di **oggi**: il file del record e delle specie (`grep -rln 'enum RecordKind' crates/kernel/src/` lo dice; gli indici con
  `grep -n '#\[n(' <quel file>`), `crates/kernel/src/arbiter/` (`set_policy`, `MakeRoom`), `crates/kernel/tests/frozen/` (sei `.cbor` e la
  mappa `record_v1.map`) e `crates/kernel/tests/frozen_bytes.rs`.

## Su che cosa insistere, oltre alle sei dimensioni
- (il paragrafo di R3 sul compito 8, testuale) ⛔ **Gli indici**: `RecordKind::Policy` all'indice **7** e `Detail::Policy` all'indice **4**
  (D26): quali indici sono presi **oggi**, e il **6** ne prende uno prima (`Invocation`): i tre compiti 6, 8 e il codice di oggi si contendono
  gli stessi numeri? `PolicyDetail { local: bool }` e `RecordV1::policy`; l'**ottavo** record congelato — la numerazione dei file in
  `crates/kernel/tests/frozen/` e la mappa (il 6 ne aggiunge uno: il nome del file dell'ottavo non collide con quello del settimo?).
  `policy_now(journal) -> Option<VramPolicy>` (D27); `Arbiter::set_policy` **oggi** — firma, che cosa scrive nel giornale, `MakeRoom::name()`
  esiste? (`grep -n 'pub fn set_policy\|fn name' crates/kernel/src/arbiter/*.rs`). P-46: il compito 8 rende rosso il banco del 7 e la cura
  è nell'8 — la sequenza è dettata in un ordine che lascia il cancello verde **a ogni commit** (vincolo 13)? I `match` esaustivi su
  `RecordKind` (P-45: quattro) — il compito 8 li tocca tutti? La mutazione «misurata accanto» delle due sonde: è scritta con l'esito atteso?
- ⛔ **COMPILA il modello.** Nella tua cartella di prova (`<scratchpad>/review/probe-R12/ws/`) copia il workspace — `Cargo.toml`,
  `Cargo.lock`, `rust-toolchain.toml`, `crates/` senza `target/` (le crate ereditano `edition.workspace`) — applica ai file copiati le
  modifiche che i Passi dettano (sorgenti **CRLF**: Python con `newline=""`, mai `sed -i`), e LÀ `cargo build --locked` e
  `cargo test --locked -p kernel` con `CARGO_TARGET_DIR=C:\Users\zagor\AppData\Local\Temp\probe-R12-target`. Mai nel repo. Il settimo record
  congelato del 6 non esiste ancora: se l'ottavo dell'8 lo presuppone (numerazione, `frozen_bytes.rs`, il `match` su `Detail`), modella anche
  ciò che il 6 detta (scheletro, *Interfaces* del 6, e `awk` sul compito 6 per i blocchi) o dichiara il limite. Ogni rosso è un rilievo
  `fatto`, `sì`, col messaggio intero. ⚠️ I byte congelati esistenti **non cambiano** (ADR-0036): se una modifica dettata li muove, è un
  rilievo che blocca.
- ⛔ **La metà dell'8 di R9b-5, l'unica riga ⬜ del registro** — il coordinatore la scriverà col tuo rapporto. Il Passo 12(a) dell'8 deve
  scrivere «✅ **chiusa il <data>, compito 8 del piano della parte 2**» in coda alla cella «Chiusore» della riga della stella citata sopra,
  **nella forma del Passo 15 del 14** (`awk '/^## Compito 14:/{f=1} /^## Compito 15:/{exit} f' <piano>`, poi cerca `Passo 15`: le ancore prese dal
  file per sezione e inizio di riga, l'`assert` sul numero di righe invariato, il criterio `grep -c`). Verifica: la riga esiste ed è unica;
  conta le colonne di quella tabella e di' **quale** cella è «Chiusore» (l'intestazione della tabella); il Passo 12(a) di oggi dice già
  qualcosa di questa riga? Proponi nel rapporto il **testo esatto** del sotto-passo — ancora, script Python nella forma del Passo 15 del 14,
  la riga nei Files e il criterio — così il coordinatore lo applica senza reinventarlo.
- Le **due righe applicate** (R10-12 / D77: `E66`, `E112` ed `E50` col nome del piano del Traguardo 6): `grep -n 'E66\|E112\|E50'` nel
  compito → ciascuna porta il nome del piano? D75 e D76: i richiami dettati portano `<data>`, e «milestone 5 task 9» è il Traguardo 5 del
  SP1 e resta — è così?
- Le **P** che nominano l'8 — almeno P-44, P-45, P-46 e quelle che `p-titles.md` e la lettura delle voci trovano: rimisurale. **D25, D26,
  D27**: ciascuna ha il suo riflesso nei Passi.
- **Consumatori**: il 9, il 10 e il 12 consumano dall'8 (`policy_now`, `PolicyDetail`, `RecordKind::Policy`, `Detail::Policy`): `grep -n` nel
  piano, firma per firma.
- **Numeri di compito**: ogni «il 6», «il 7», «il 9», «il 10», «il 12» nell'8 contro la tabella della posizione (D25 ha spostato di uno dal
  9 al 16).
