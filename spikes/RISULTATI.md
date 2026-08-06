# Risultati degli spike

Data di esecuzione: _(da compilare)_

Criteri e soglie: [PROTOCOLLO.md](PROTOCOLLO.md) — congelato al primo commit di
codice di spike.

## SP-6 — Confine dei dati non fidati, e confini statici del kernel

| Criterio | Rust | Go | TypeScript |
|---|---|---|---|
| T1 non compila | ✅ `passa` | | |
| T2 percorso unico | ✅ `passa` | | |
| T3 ereditarietà | ✅ `passa` | | |
| T4 aggiramento | ✅ `passa` | | |
| T5 rilevabile globalmente | ✅ `passa` | | |
| T6 importazione vietata, provata in negativo | ✅ `passa` | | |

## SP-5 — Iniettabilità e riproducibilità

| Criterio | Rust | Go | TypeScript |
|---|---|---|---|
| C1 stesso seed → stessa traccia | | | |
| C2 seed diversi → tracce diverse | | | |
| C3 tempo virtuale | | | |
| C4 guasto riproducibile | | | |
| C5 nessun orologio/RNG globale | | | |
| C6 concorrenza nativa ordinabile | | | |
| C7 I/O iniettabile, crash riproducibile | | | |

## Osservazioni registrate — non criteri

| # | Rust | Go | TypeScript |
|---|---|---|---|
| O1 motore di persistenza conforme a §10.6 | | | |
| O2 daemon a vita lunga, istanza singola | | | |

## Versioni degli strumenti

| Candidato | Comando | Output |
|---|---|---|
| Rust | `rustc --version` | `rustc 1.95.0 (59807616e 2026-04-14)` · `cargo 1.95.0` · `clippy 0.1.95` · `trybuild 1.0.120` |
| Go | `go version` | `go version go1.26.5 windows/amd64` |
| TypeScript | `npx tsc --version` | _(da compilare)_ |

## Seed usati

Un risultato senza seed non è valido.

| Criterio | Candidato | Seed | Note |
|---|---|---|---|
| | | | |

## Evidenze

Una riga per criterio e candidato: comando eseguito, output osservato, e **le
divergenze** rispetto a ciò che ci si aspettava. Una divergenza non registrata è un
risultato perso.

### SP-6 · Rust — eseguito il 2026-08-06, rustc 1.95.0

| Criterio | Comando | Output osservato | Divergenza dall'attesa |
|---|---|---|---|
| **T1** | `cargo test --test compile_fail` | `error[E0308]: mismatched types … expected &Instruction, found &Untrusted`. **Provato non vacuo**: rendendo compilabile la violazione il test passa a `FAILED`, ripristinandola torna `ok` | il piano prevedeva questo esito; confermato |
| **T2** | ricerca testuale su `src/` | una sola funzione, `Untrusted::promote_to_instruction`. I campi delle due struct non sono pubblici: nessun'altra via di costruzione dall'esterno | nessuna |
| **T3** | `cargo test --test boundary` | `summarize(&Untrusted) -> Untrusted`: la firma **impone** l'ereditarietà, non la raccomanda | nessuna |
| **T4** | `cargo build` con `#![forbid(unsafe_code)]` + `#[allow(unsafe_code)]` locale | `error[E0453]: allow(unsafe_code) incompatible with previous forbid`. **`forbid` non è scavalcabile per riga**, a differenza di `deny` | nessuna. Per la regola di decisione del protocollo è `passa`, non `parziale`: il divieto è del compilatore |
| **T5** | `cargo build` | la compilazione dell'intero progetto è essa stessa il controllo: non esiste un sito d'uso che si possa dimenticare di controllare | nessuna |
| **T6 (a)** lint | `cargo clippy -- -D clippy::disallowed_methods -D clippy::disallowed_types` | ferma `SystemTime::now`; **`cargo build` da solo NON la ferma**. Il divieto vive in `clippy.toml`, è configurabile e disattivabile con `#[allow]` | — |
| **T6 (b)** compilatore | `cargo build -p kernel_core` su una crate `#![no_std]` | `error[E0433]: cannot find module or crate 'std'`. Non è un lint: è ciò che il compilatore ha caricato. Provato in **entrambe** le direzioni | — |

**Nota strutturale su T6.** In Rust entrambi i meccanismi sono a **granularità di
crate**, non di modulo. Conseguenza architetturale, non dettaglio: il kernel dovrebbe
essere una crate propria, e il modulo di piattaforma una crate separata. È coerente con
I3, ma va detto perché vincola il layout dei sorgenti dal primo giorno.

**Scoperta collaterale, non cercata.** `std::collections::HashMap` è stata inserita fra
i tipi vietati: `RandomState` è seminato casualmente **per processo**, quindi l'ordine
di iterazione non è riproducibile fra esecuzioni. È una violazione di V29 che non
compare in nessun elenco di «chiamate OS» e che C1 scoprirebbe solo come traccia
divergente e inspiegabile. Vale per ogni candidato: va verificata anche su Go e
TypeScript.

### Altre esecuzioni

| Criterio | Candidato | Comando | Output osservato | Divergenza dall'attesa |
|---|---|---|---|---|
| | | | | |
