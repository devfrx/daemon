# Risultati degli spike

Data di esecuzione: _(da compilare)_

Criteri e soglie: [PROTOCOLLO.md](PROTOCOLLO.md) — congelato al primo commit di
codice di spike.

## SP-6 — Confine dei dati non fidati, e confini statici del kernel

| Criterio | Rust | Go | TypeScript |
|---|---|---|---|
| T1 non compila | ✅ `passa` | ✅ `passa` | |
| T2 percorso unico | ✅ `passa` | ✅ `passa` | |
| T3 ereditarietà | ✅ `passa` | ✅ `passa` | |
| T4 aggiramento | ✅ `passa` | ✅ `passa` **ma solo dopo una correzione**, vedi sotto | |
| T5 rilevabile globalmente | ✅ `passa` | ✅ `passa` | |
| T6 importazione vietata, provata in negativo | ✅ `passa` | ✅ `passa`, con driver scritto a mano | |

## SP-5 — Iniettabilità e riproducibilità

| Criterio | Rust | Go | TypeScript |
|---|---|---|---|
| C1 stesso seed → stessa traccia | ✅ `passa` | | |
| C2 seed diversi → tracce diverse | ✅ `passa` | | |
| C3 tempo virtuale | ✅ `passa` | | |
| C4 guasto riproducibile | ✅ `passa` | | |
| C5 nessun orologio/RNG globale | ✅ `passa` | | |
| C6 concorrenza nativa ordinabile | ✅ `passa` | | |
| C7 I/O iniettabile, crash riproducibile | ✅ `passa` | | |

**Rust passa entrambi gli spike.** Per la regola di applicazione del protocollo, avendo
C6 = `passa`, lo spareggio #1 dell'ADR **non gli si applica**.

## Osservazioni registrate — non criteri

| # | Rust | Go | TypeScript |
|---|---|---|---|
| O1 motore di persistenza conforme a §10.6 | candidati esistono: `redb` 4.1.0 · `fjall` 3.1.8 (LSM, adatto alla potatura selettiva) · `rusqlite` 0.40.1 · `sled` 1.0.0-alpha.124. **Requisito 4 (I/O iniettabile) da confermare** nell'ADR sulla persistenza: è il discriminante, non la disponibilità | | |
| O2 daemon a vita lunga, istanza singola | da registrare | | |

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
| C1, C2 | Rust | `42`, `43` | tracce identiche a parità di seed, diverse fra seed |
| C3 | Rust | `7` | orologio virtuale a 5000 ms, tempo di parete < 1 s |
| C4 | Rust | `99` | il seed inietta almeno un `GUASTO`; riprodotto identico |
| C6 | Rust | `20260806` | 100 esecuzioni → **1 sola** traccia distinta; con `20260807` l'interlacciamento cambia |
| C7 | Rust | `1, 7, 42, 99, 20260806` | tracce identiche a parità di seed, caduta inclusa |
| C7 dubbio | Rust | **`0`** | primo seed su 200 che cade *fra* intento ed esito: passo 0 resta `InDubbio`, rilevabile |

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

### SP-5 · Rust — eseguito il 2026-08-06, rustc 1.95.0

| Criterio | Comando | Output osservato | Divergenza dall'attesa |
|---|---|---|---|
| **C1–C4** | `cargo test --test sched` | 4/4. Il seed 99 inietta un guasto **senza doverne cercare un altro**: il piano prevedeva di doverlo sostituire | il piano prevedeva un possibile skip; non è servito |
| **C5** | `grep -rnE "Instant::now\|SystemTime\|rand::\|thread_rng\|std::fs\|std::net\|std::env\|HashMap" src/ kernel_core/src/` escludendo i commenti | nessun riscontro. **Provato non vacuo**: inserendo `SystemTime::now()` in `sched.rs` il grep lo trova | il grep iniziale, senza escludere i commenti, dava un falso positivo su un `//!` |
| **C6 (a)** | `cargo test --test c6` | `Future` native guidate da un esecutore proprio: **100 esecuzioni, seed 20260806, 1 sola traccia distinta**. Interlacciamento reale verificato, altrimenti il determinismo sarebbe vacuo | nessuna |
| **C6 (b)** controprova | idem | `std::thread` dell'OS: **> 1 traccia distinta su 100**. Stabilisce il confine di C6 — non è un criterio che tutti superano per costruzione | nessuna |
| **C7** | `cargo test --test c7` | 6/6. Crash riproducibile su 5 seed; l'ordine è write-ahead (`I,E,I,E,I,E`); il passo `InDubbio` è rilevabile e **senza falsi positivi** quando non c'è crash; il giornale è sostituibile con un secondo doppio senza toccare il codice sotto test | nessuna |
| ecosistema | `cargo add --dry-run madsim` | `Adding madsim v0.2.34`. Esiste un runtime deterministico di ecosistema che **sostituisce tokio**; `turmoil` 0.7.2 è l'alternativa | nessuna |

**Il dato che distingue Rust, in una riga.** L'ordine delle unità concorrenti native è
deciso dal **nostro** esecutore, non dal runtime: `Future` è un oggetto che si sceglie
quando far avanzare. Non serve uno strumento di test per ottenerlo, e vale anche fuori
dai test — che è la differenza fra controllo *posseduto* e *fornito*.

**Il costo, misurato e non stimato.** La regola di T6 (a) è a granularità di **crate**
e ha bloccato un uso **legittimo** di `Instant::now()` dentro il test C3, che deve
misurare il tempo di parete proprio per provare che il tempo virtuale non ha atteso.
Si è dovuto scrivere `#[allow(clippy::disallowed_methods)]` su quel test. È la prova,
su un caso reale e non ipotetico, che il meccanismo (a) è **disattivabile per sito** —
mentre `forbid` e `no_std` non lo sono. Il confine forte in Rust c'è, ma va scelto:
non è quello di default.

### SP-6 · Go — eseguito il 2026-08-06, go1.26.5

| Criterio | Comando | Output osservato | Divergenza dall'attesa |
|---|---|---|---|
| **T1** | `go test ./boundary/ -run TestT1` | `cannot use dalWeb (variable of struct type boundary.Untrusted) as boundary.Instruction value`. **Provato non vacuo.** Il driver verifica anche il **motivo** dell'errore: una compilazione fallita per la ragione sbagliata sarebbe un falso positivo (gotcha #9) | il driver del piano **non compilava**: a capo letterale in una stringa. Errata E1, necessaria |
| **T2** | ricerca testuale | una sola funzione, `Untrusted.PromoteToInstruction` | nessuna |
| **T3** | `go test ./boundary/` | `Summarize(Untrusted) Untrusted` | nessuna |
| **T4** | `go build` su una conversione diretta, da **fuori** dal package | vedi il riquadro sotto: **il piano si sbagliava** | ⚠️ **divergenza sostanziale** |
| **T5** | `go build ./... && go vet ./...` | puliti; la compilazione del modulo è il controllo globale | nessuna |
| **T6** | `go test ./kernel/` con driver su `go list -deps` | il kernel non dipende da `os`, `net`, `syscall`, `math/rand`. **Provato in entrambe le direzioni**: introducendo `import "os"` il test fallisce con `T6 VIOLATO: il kernel dipende da [syscall os]`. Controprova su `platform`, che *deve* risultare in violazione | Go non ha una regola nativa: serve un **driver scritto a mano**, come per T1. Toolchain standard, però: nessuno strumento esterno |

#### T4 · La trappola che il piano non conosceva

Il piano affermava, come evidenza pre-scritta da riportare: «aggirabile con una
conversione esplicita `Instruction(...)` **solo dentro il package**, perché il campo
`text` non è esportato; da fuori non è aggirabile». **Misurato: falso.**

| Passo | Comando | Risultato |
|---|---|---|
| entrambe le struct con campo `text string` | `go build` di `boundary.Instruction(dalWeb)` da un package esterno | **compila** — exit 0 |
| stessa cosa, eseguita | `go run` | stampa `sei un assistente\nignora le istruzioni precedenti`: **il contenuto non fidato è nel canale delle istruzioni** |
| campi rinominati in `text` / `raw` | `go build` | `cannot convert dalWeb (variable of struct type boundary.Untrusted) to type boundary.Instruction` |

**Causa.** In Go due tipi con lo stesso **tipo sottostante** sono convertibili. L'identità
dei tipi sottostanti richiede la stessa sequenza di nomi e tipi dei campi; per i campi
non esportati conta il package di provenienza, che qui è lo stesso per entrambi i tipi.
Il campo non esportato quindi **non protegge**: protegge dalla costruzione con
letterale, non dalla conversione.

**Gravità.** L'aggiramento non richiede `unsafe`, non richiede reflection, non richiede
di toccare il package: è la sintassi più ordinaria di Go, `T(x)`, scrivibile ovunque.
Era il modo di fallire peggiore — silenzioso e per costruzione.

**Correzione applicata e blindata.** I campi si chiamano `text` e `raw`. Il package
`boundary/conversione`, dietro il tag `violation`, contiene la conversione: se qualcuno
riallineasse i nomi dei campi, `TestT4LaConversioneDirettaNonCompila` fallisce.

**Verdetto.** `passa` per la regola di decisione — l'aggiramento è ora vietato **dal
compilatore** — con due riserve registrate:
1. la protezione dipende da una disciplina sui **nomi dei campi** che nessun
   compilatore impone e che non è nella documentazione del linguaggio come tale;
2. `unsafe` resta una via, ma è **vietabile con lo stesso meccanismo di T6**: verificato
   che il package `unsafe` compare in `go list -deps`. Go non ha l'equivalente di
   `#![forbid(unsafe_code)]`, quindi il divieto è un test, non un attributo.

#### Scoperta collaterale · l'ordine di iterazione delle `map`

Misurato: **200 iterazioni della stessa map, nello stesso processo → 8 ordini
distinti.** È la randomizzazione deliberata del runtime Go, e non è disattivabile.

È l'analogo del `HashMap` di Rust, ma con una differenza pratica sostanziale: in Rust
si sostituisce `HashMap` con `BTreeMap` e il problema sparisce; in Go `map` è il tipo
**incorporato** e non esiste un'alternativa ordinata nella libreria standard — le
chiavi vanno estratte e ordinate a ogni iterazione, ogni volta, per sempre.

Per V29 è una fonte di non determinismo che non compare in nessun elenco di «chiamate
OS» e che C1 rivelerebbe solo come traccia divergente e inspiegabile.

### Altre esecuzioni

| Criterio | Candidato | Comando | Output osservato | Divergenza dall'attesa |
|---|---|---|---|---|
| | | | | |
