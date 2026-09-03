# Sotto-progetto 1 · Traguardo 1 — scheletro e porta di qualità

> **Per chi esegue:** SKILL RICHIESTA — usa `superpowers:subagent-driven-development`
> (consigliata) o `superpowers:executing-plans` per eseguire questo piano compito per
> compito. I passi usano le caselle (`- [ ]`) per il tracciamento.

**Obiettivo:** creare il workspace Rust con le cinque crate e rendere **eseguibile e provata
in due direzioni** l'intera porta di qualità, **senza scrivere una riga di logica di
prodotto**.

**Architettura:** un workspace Cargo alla radice con `crates/{kernel,platform,secrets,simulator,daemon}`.
`kernel` e `simulator` sono `#![no_std]` + `alloc` + `#![forbid(unsafe_code)]` e non
dipendono da nessuna crate del progetto; `platform`, `secrets` e `daemon` usano `std` e sono
il posto dove l'I/O deve vivere. Accanto al codice nascono i controlli automatici: test di
compilazione fallita, allow-list sui due grafi delle dipendenze, cancello senza OS.

**Stack:** Rust (`rustc` 1.95.0, `cargo` 1.95.0) · `bincode` 2.0.1 appuntato a `2` ·
`unty` 0.0.4 · `minicbor` 2.3.0 · `trybuild` 1 · bersaglio del cancello
`x86_64-unknown-none`.

**Spec di riferimento:** [`2026-08-06-sottoprogetto-1-kernel.md`](../specs/2026-08-06-sottoprogetto-1-kernel.md)
§0–§8. Il *perché* di ogni vincolo sta lì; qui c'è solo il *come*.

---

## Errata — 2026-08-08, dopo l'esecuzione

> ⛔ **Da leggere prima dei compiti.** Il piano **non si riscrive**: è il registro di ciò che
> fu osservato mentre lo si eseguiva, e riscriverlo falsificherebbe la storia. Ma è anche il
> documento che si riapre per i traguardi successivi, quindi non può restare muto su quattro
> punti in cui **detta una cosa e il repository ne contiene un'altra**.

| # | Dove | Cosa non torna | Perché |
|---|---|---|---|
| **E1** | **ovunque** — nomi di file, di funzioni e messaggi d'uscita | ⛔ **il piano detta identificatori italiani; il codice eseguito è in inglese** | contraddice la **§1.0** della spec — *«Codice: interamente in inglese: nomi di crate, moduli, tipi, funzioni, commenti nel sorgente»* — e **vince la spec**, per «spec prima del codice». Il piano non aveva l'autorità per derogarvi, e la deroga non era nemmeno dichiarata: era implicita nei nomi. I nomi veri sono nella tabella sotto |
| **E2** | **Task 2 · Step 2** | ⛔ **il banco vuoto NON fallisce: esce _verde_** | il passo prevede un rosso «perché la cartella `tests/compile_fail/` non esiste ancora». Misurato su `trybuild` 1.0.120: un **glob** che non pesca niente **non è un errore** — stampa un avviso giallo, lascia i fallimenti a zero e **esce 0**. Un percorso *letterale* inesistente invece diventa rosso, e l'asimmetria non si ricostruisce leggendo `t.compile_fail(...)`. ⚠️ Il piano il gotcha #26 lo aveva perfino **nominato**, nella riga sotto — *«se passasse trovando zero casi…»* — ma lo dava per non accaduto. È il caso che si è verificato. Rimedio in esercizio: `compile_fail.rs` conta i `.rs` **prima** di chiamare `trybuild`, e senza numero atteso (§8.6.2) |
| **E3** | **Task 6 · Step 3** | ⛔ **la porta diventa rossa, ma non per la ragione scritta** | il passo dice che togliendo `#![no_std]` da `crates/kernel/src/lib.rs` il caso «ora **compila**», e che `trybuild` lo segnala. Misurato: il caso **non compila lo stesso**. Ogni caso **ridichiara** il proprio `#![no_std]`, quindi `E0433` scatta comunque; `trybuild` cade per **mismatch dello `.stderr`**, perché il kernel che linka `std` fa **sparire la riga dell'allocatore** — *«no global memory allocator found»* — dall'output reale. E i casi rossi sono **due**, non uno: `std_in_kernel.rs` **e** `hashmap_in_kernel.rs`, cioè i due il cui oracolo portava quella riga. Il rosso c'è; l'affermazione sul **meccanismo** è falsa |
| **E4** | **Definizione di «fatto» · condizione 2** | ⛔ **non era sufficiente, e lo si è scoperto misurando** | «i quattro casi di `tests/compile_fail/` passano» non prova ciò che la condizione promette. I quattro **ridichiarano ciascuno i propri attributi** e non nominano mai `kernel::`: provano che il meccanismo **morde dove è dichiarato**, non che sia dichiarato **nel kernel**. Tolto `#![forbid(unsafe_code)]` da `crates/kernel/src/lib.rs` e scritto un `unsafe` **vero**, la porta restava **verde su cinque controlli su cinque**. Da qui `scripts/gate-attributes.sh` e la riga di catalogo che lo enumera (§7.4.2) |

### E1 — i nomi veri

| Il piano scrive | Nel repository è |
|---|---|
| `crates/kernel/tests/compile_fail/std_nel_kernel.rs` | `std_in_kernel.rs` |
| `crates/kernel/tests/compile_fail/unsafe_nel_kernel.rs` | `unsafe_in_kernel.rs` |
| `crates/kernel/tests/compile_fail/allow_unsafe_scavalca.rs` | `allow_overrides_forbid.rs` |
| `crates/kernel/tests/compile_fail/hashmap_nel_kernel.rs` | `hashmap_in_kernel.rs` |
| `crates/kernel/tests/dipendenze_utilizzabili.rs` | `dependencies_usable.rs` |
| `crates/platform/tests/contro_sonde.rs` | `counter_probes.rs` |
| `PORTA VERDE.` · `PORTA ROSSA` | `GATE GREEN.` · `GATE RED` |
| `I3 violato` · `grafo di build cambiato` | `I3 violated` · `build graph changed` |
| `OK — nessuna incoerenza.` | `OK — no inconsistencies.` |
| `.github/workflows/porta.yml` — `name: porta di qualità`, job `porta` | `.github/workflows/quality-gate.yml` — `name: quality gate`, job `gate`. ⚠️ **Allineato il 2026-08-09, non il giorno dell'esecuzione:** era l'ultimo residuo italiano nel codice, ed è stato lasciato indietro. ⛔ Cambiarlo era **gratis solo finché il workflow non fosse mai stato eseguito** — committato il 2026-08-08, ramo mai pushato, quindi nessuna regola di protezione del ramo poteva ancora riferirsi a quei nomi. Dopo la prima corsa non lo sarebbe più stato |

📌 **La tabella elenca solo ciò che il piano _scrive_.** Il cancello sugli attributi —
`scripts/gate-attributes.sh` — **non compare nel piano**: è nato dopo, dal difetto di **E4**.
E i messaggi di `check-docs.sh` che il piano non cita restano fuori di qui: dove sono citati,
cioè nella spec §8.5.4 e §8.6, sono stati **rimisurati**, non tradotti a tavolino.

⛔ **Le parole che `check-docs.sh` _cerca_ dentro i documenti restano italiane**, e non è
un'incoerenza: `verificato qui`, `parziale`, `rimandato`, `non controllato` e l'intestazione
`Difende` sono **contenuto della documentazione**, non messaggi dello script. Tradurle
renderebbe rosse o, peggio, **vacue** le asserzioni 2, 3, 4 e 6 di §8.6.1.

📌 **E2 ed E3 sono la stessa lezione, ed è quella del gotcha #17:** una sonda che guarda solo
il **codice d'uscita** non distingue «è scattato per il mio guasto» da «è scattato per un
altro». Entrambe le previsioni sbagliate del piano danno l'uscita **attesa** — verde in E2,
rosso in E3 — e solo il **messaggio** dice che il meccanismo non è quello descritto.

⛔ **E3 ha una trappola operativa, e sta proprio dove il passo la incontra.** Il rosso di
`trybuild` invita a *«bless it by rerunning with `TRYBUILD=overwrite`»*. Farlo qui
**cancellerebbe l'oracolo**: riscriverebbe i due `.stderr` sull'output di un kernel che linka
`std`, e la suite passerebbe per sempre — è il **gotcha #25**, e il vincolo globale 6 lo
vieta. Il rimedio è **rimettere `#![no_std]`**, non benedire l'output.

---

## Perché questo traguardo non produce niente di visibile, ed è deliberato

Un cancello costruito **dopo** la logica è un cancello che nessuno ha mai visto fallire, e
la §7.1.1 dice che allora non è un cancello. Su uno scheletro vuoto ogni controllo si prova
in **due** direzioni — che scatti dove deve, e che **non** scatti dove non deve — al costo di
poche righe. Dopo, la seconda direzione diventa cara e si smette di provarla.

⛔ **Cosa questo traguardo NON contiene, e non è una dimenticanza:** nessuna porta, nessun
tratto, nessun esecutore, nessun record durevole. In particolare **non si scrive nessun
record del giornale**: il vincolo 14 della §11 fa entrare i **byte congelati** nel
repository *al primo record scritto*, e quel primo record appartiene al Traguardo 3.
Scriverne uno qui congelerebbe un formato che la §4.9 non ha ancora messo alla prova.

---

## Vincoli globali

Valgono per **ogni** compito di questo piano. Sono decisioni già prese: i numeri e i nomi
sono copiati alla lettera dalla spec, non ricavati.

| # | Vincolo | Da |
|---|---|---|
| 1 | Le crate sono **cinque**: `kernel` · `platform` · `secrets` · `simulator` · `daemon`. **`kernel` non dipende da nessuna crate del progetto** | §1.2 |
| 2 | `kernel` e `simulator`: `#![no_std]` + `alloc` + `#![forbid(unsafe_code)]`. ⛔ **`forbid`, non `deny`** — `deny` è scavalcabile da un `#[allow]` locale | §1.4 · ADR-0026 |
| 3 | Il manifesto **appunta `bincode` a `2`**, con la ragione scritta accanto: la `3.0.0` esiste, è l'ultima pubblicata, e il suo intero sorgente è un `compile_error!` | §6.1.1 · gotcha #22 |
| 4 | `rustup target add x86_64-unknown-none` è un **prerequisito dell'ambiente**, o la porta è rossa per il motivo sbagliato | §7.3.2 |
| 5 | Il `clippy.toml` di `spikes/rust/` **non sale**: a livello di workspace scatterebbe addosso a `platform`, che *deve* chiamare l'orologio e il filesystem | §7.4.4 |
| 6 | ⛔ Gli `.stderr` di `trybuild` **si leggono, non si rigenerano in blocco**. Rigenerarli cancella l'oracolo e la suite passa per sempre | §7.1.4 · gotcha #25 |
| 7 | Ogni controllo entra **solo** se difende un `V`, un'`I` o un `Q` nominato, **si è visto scattare**, e **si è visto restare verde** dove la cosa è lecita | §7.1.1 |
| 8 | Cadenza: livello 1 **a ogni compilazione** — non «gira»: *è* il compilatore. Livello 2 **a ogni commit** | §7.5.1 |
| 9 | Nessuna decisione del kernel legge un parametro che non le è stato consegnato. In questo sotto-progetto i default sono **letterali in `daemon`** | §2.8 · ADR-0034 |
| 10 | ⛔ `std::collections::HashMap` non si usa in `kernel` e `simulator`: `RandomState` è seminato **per processo** e l'ordine di iterazione non è riproducibile. `BTreeMap`, o un hasher fissato | gotcha #12 |

### Due scelte che il piano prende, con la ragione

La spec non le fissa, e vanno prese ora perché costano zero adesso e crescono dopo.

| Scelta | Perché ora |
|---|---|
| **edition `2024`** su tutte e cinque le crate | è l'edizione corrente per `rustc` 1.95, e cambiarla dopo tocca ogni file. Gli spike usano `2021` e **restano** dove sono: non salgono come membri |
| un **`rust-toolchain.toml`** alla radice, che dichiara la versione **e il bersaglio** `x86_64-unknown-none` | il vincolo 4 chiede che il bersaglio sia installato. Dichiararlo qui lo rende automatico su una macchina pulita, invece di lasciarlo a una riga di README che nessuno legge — ed è la differenza fra «la porta è rossa» e «la porta è rossa per il motivo sbagliato» |

---

## Struttura dei file

```
Cargo.toml                     ← workspace: i cinque membri, e spikes/ ESCLUSO
rust-toolchain.toml            ← versione + bersaglio del cancello
crates/
  kernel/Cargo.toml            ← le tre voci spedite, bincode appuntato a "2"
  kernel/src/lib.rs            ← no_std + alloc + forbid(unsafe_code), e nient'altro
  kernel/tests/compile_fail.rs ← il banco trybuild
  kernel/tests/compile_fail/   ← un caso + un .stderr per ogni regola di livello 1
  platform/Cargo.toml
  platform/src/lib.rs          ← usa std E unsafe: è la contro-sonda vivente
  secrets/Cargo.toml
  secrets/src/lib.rs
  simulator/Cargo.toml         ← dipende SOLO da kernel
  simulator/src/lib.rs         ← no_std + alloc + forbid(unsafe_code)
  daemon/Cargo.toml            ← dipende da kernel, platform, secrets. NON da simulator
  daemon/src/main.rs
scripts/
  gate-no-os.sh                ← il cancello senza OS
  gate-deps.sh                 ← l'allow-list sui due grafi
  gate.sh                      ← il livello 2 completo, uno solo da lanciare
  check-docs.sh                ← esiste già, entra nel livello 2
```

⛔ **`spikes/` non è un membro del workspace.** `spikes/rust/` è a sua volta un workspace
annidato e porta un `clippy.toml` che il vincolo 5 vieta di far salire. Restano compilabili
dove sono; la §2.5 dice riga per riga cosa si **copia** dentro `crates/kernel/`, e quella
copia appartiene ai traguardi successivi.

---

## I traguardi successivi, per sapere cosa non c'è qui

Ciascuno avrà il proprio piano, scritto quando si arriva.

| # | Traguardo | Deliverable |
|---|---|---|
| **1** | **scheletro e porta di qualità** | ← **questo piano** |
| 2 | il substrato iniettabile | tempo, casualità, I/O, scheduling; l'esecutore in `kernel`; le sei famiglie di porte come tratti |
| 3 | giornale e formato durevole | la porta `journal` a byte, il record come enum di versione, **i byte congelati** |
| 4 | il simulatore DST | tempo virtuale, iniezione dei guasti, la campagna, i semi |
| 5 | arbitro GPU | ammissione, corsie, ciclo della concessione, le due policy |
| 6 | gli altri meccanismi | gateway, sensori, permessi, degrado, il canale worker |

---

## Task 1: Il workspace e le cinque crate

**Files:**
- Create: `Cargo.toml`
- Create: `rust-toolchain.toml`
- Create: `crates/kernel/Cargo.toml`, `crates/kernel/src/lib.rs`
- Create: `crates/platform/Cargo.toml`, `crates/platform/src/lib.rs`
- Create: `crates/secrets/Cargo.toml`, `crates/secrets/src/lib.rs`
- Create: `crates/simulator/Cargo.toml`, `crates/simulator/src/lib.rs`
- Create: `crates/daemon/Cargo.toml`, `crates/daemon/src/main.rs`

**Interfaces:**
- Consumes: nulla — è il primo compito.
- Produces: i cinque nomi di crate (`kernel`, `platform`, `secrets`, `simulator`, `daemon`)
  e i loro percorsi sotto `crates/`. Ogni compito successivo vi si appoggia. Nessuna
  funzione pubblica: le `lib.rs` sono deliberatamente vuote.

- [ ] **Step 1: Scrivere il manifesto del workspace, con `spikes/` escluso**

`Cargo.toml`:

```toml
[workspace]
resolver = "3"
members = [
    "crates/kernel",
    "crates/platform",
    "crates/secrets",
    "crates/simulator",
    "crates/daemon",
]
# spikes/ NON è un membro: spikes/rust/ è un workspace annidato e porta un clippy.toml
# che a livello di workspace scatterebbe addosso a platform, che DEVE chiamare l'orologio
# e il filesystem. Vincolo 5 della §11, deciso in §7.4.4.
exclude = ["spikes"]

[workspace.package]
edition = "2024"
version = "0.0.0"
publish = false
```

⛔ **Non aggiungere `[workspace.lints]`.** Ereditare `forbid(unsafe_code)` a livello di
workspace è la «semplificazione» che il Task 2 esiste per intercettare: `platform` non
compila senza `unsafe`.

- [ ] **Step 2: Dichiarare la toolchain e il bersaglio del cancello**

`rust-toolchain.toml`:

```toml
[toolchain]
channel = "1.95.0"
components = ["rustfmt", "clippy"]
# Il bersaglio del cancello senza OS (§7.3.2). Dichiararlo qui lo installa da solo su una
# macchina pulita: senza, la porta è rossa per il motivo sbagliato — vincolo 4 della §11.
targets = ["x86_64-unknown-none"]
```

- [ ] **Step 3: Creare `kernel`, che è la crate vincolata**

`crates/kernel/Cargo.toml`:

```toml
[package]
name = "kernel"
edition.workspace = true
version.workspace = true
publish.workspace = true

# ⛔ kernel non dipende da NESSUNA crate del progetto: §1.2, ed è la riga che rende I3
# verificabile guardando il manifesto invece di ispezionare il codice.
# Le dipendenze esterne arrivano nel Task 5, e ciascuna è una voce dell'allow-list di
# ADR-0031 con la propria giustificazione scritta.
[dependencies]

[dev-dependencies]
trybuild = "1"
```

`crates/kernel/src/lib.rs`:

```rust
//! Il kernel: logica, decisioni, tratti dichiarati. Nessuna chiamata all'OS.
//!
//! Questa crate non contiene ancora nulla: il Traguardo 1 costruisce lo scheletro e la
//! porta di qualità, e la logica arriva dai traguardi successivi. Gli attributi qui
//! sotto NON sono decorazione — sono tre delle regole di livello 1 della §7.4.1, e i
//! loro test negativi vivono in `tests/compile_fail/`.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;
```

- [ ] **Step 4: Creare `simulator`, vincolato come `kernel`**

`crates/simulator/Cargo.toml`:

```toml
[package]
name = "simulator"
edition.workspace = true
version.workspace = true
publish.workspace = true

[dependencies]
# ⚠️ simulator non aggiunge voci proprie all'allow-list, ma il suo grafo NON è vuoto:
# eredita per intero quello di kernel. §7.3.1.
kernel = { path = "../kernel" }
```

`crates/simulator/src/lib.rs`:

```rust
//! Il simulatore DST: implementazioni finte dei tratti che il kernel dichiara.
//!
//! Vincolata come `kernel`, e per la stessa ragione: una corsa deterministica non può
//! contenere una sorgente di non determinismo, e `HashMap` è la più insidiosa perché non
//! compare in nessun elenco di «chiamate OS» — gotcha #12.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;
```

- [ ] **Step 5: Creare `platform`, che è la contro-sonda vivente**

`crates/platform/Cargo.toml`:

```toml
[package]
name = "platform"
edition.workspace = true
version.workspace = true
publish.workspace = true

[dependencies]
kernel = { path = "../kernel" }
```

`crates/platform/src/lib.rs`:

```rust
//! Le implementazioni reali dei tratti dichiarati dal kernel: filesystem, orologio, rete,
//! processi, confinamento.
//!
//! ⛔ Questa crate USA `std` e USERÀ `unsafe` per la FFI, ed è deliberato: è il posto dove
//! l'I/O deve vivere (ADR-0031, perimetro). Le funzioni qui sotto esistono come
//! CONTRO-SONDE — provano che i divieti del kernel non scattano dove non devono, che è la
//! direzione che si dimentica (§7.1.1 regola 3, gotcha #24). Non cancellarle finché non
//! esiste codice reale che dimostri le stesse due cose.

/// Contro-sonda di `no_std`: `platform` nomina `std::fs` e **compila**.
pub fn contro_sonda_std_compila() -> bool {
    core::mem::size_of::<std::fs::File>() > 0
}

/// Contro-sonda di `forbid(unsafe_code)`: `platform` usa `unsafe` e **compila**.
///
/// Se qualcuno dichiarasse i divieti a livello di workspace, questa funzione smetterebbe
/// di compilare — ed è esattamente ciò che la contro-sonda deve intercettare.
pub fn contro_sonda_unsafe_compila() -> usize {
    let x: u8 = 42;
    let p = &raw const x;
    // SAFETY: `p` deriva da un riferimento a `x`, vivo per tutta la funzione.
    unsafe { *p as usize }
}
```

- [ ] **Step 6: Creare `secrets` e `daemon`**

`crates/secrets/Cargo.toml`:

```toml
[package]
name = "secrets"
edition.workspace = true
version.workspace = true
publish.workspace = true

[dependencies]
kernel = { path = "../kernel" }
```

`crates/secrets/src/lib.rs`:

```rust
//! L'unico punto che tocca il portachiavi dell'OS.
//!
//! È una crate separata da `platform` per una ragione sola: V34 chiede che «un solo punto
//! legge le credenziali» sia verificabile **staticamente**, e in Rust la granularità
//! verificabile è la crate. Dentro `platform` sarebbe una regola fra moduli, cioè una
//! convenzione. È il motivo per cui le crate sono cinque e non quattro (§1.2).
```

`crates/daemon/Cargo.toml`:

```toml
[package]
name = "daemon"
edition.workspace = true
version.workspace = true
publish.workspace = true

[dependencies]
kernel = { path = "../kernel" }
platform = { path = "../platform" }
secrets = { path = "../secrets" }
# ⛔ NON dipende da `simulator`. Il daemon è il cablaggio DI PRODUZIONE: monta `platform`.
# In simulazione il cablaggio lo fa il banco di prova, che riceve i parametri risolti —
# ADR-0034, e §1.2 corretta il 2026-08-08 dopo che tabella e grafo si contraddicevano.
```

`crates/daemon/src/main.rs`:

```rust
//! Il cablaggio di produzione: monta `platform`, avvia l'esecutore, ospita il server IPC,
//! e **produce i parametri risolti** che consegna al kernel (§2.8, ADR-0034).
//!
//! In questo sotto-progetto i default sono **letterali qui dentro**, non letti da un
//! archivio: vincolo 11 della §11.

fn main() {
    println!("daemon: scheletro. Nessuna logica in questo traguardo.");
}
```

- [ ] **Step 7: Compilare tutto, e verificare che il verso delle dipendenze sia quello dichiarato**

```bash
cargo build --workspace
```

Atteso: **cinque** crate compilate, zero warning.

```bash
cargo tree -p kernel -e normal
```

Atteso: `kernel v0.0.0` **e nient'altro** — nessuna dipendenza, ed è il vincolo 1.

```bash
cargo tree -p daemon -e normal --depth 1
```

Atteso: `daemon` con `kernel`, `platform`, `secrets`. ⛔ **`simulator` NON deve comparire.**

- [ ] **Step 8: Commit**

```bash
git add Cargo.toml rust-toolchain.toml crates/ && git commit -m "feat(workspace): le cinque crate, e spikes resta fuori"
```

---

## Task 2: `no_std` e `forbid(unsafe_code)`, provati in due direzioni

**Files:**
- Create: `crates/kernel/tests/compile_fail.rs`
- Create: `crates/kernel/tests/compile_fail/std_nel_kernel.rs`
- Create: `crates/kernel/tests/compile_fail/unsafe_nel_kernel.rs`
- Create: `crates/kernel/tests/compile_fail/allow_unsafe_scavalca.rs`
- Create: `crates/kernel/tests/compile_fail/hashmap_nel_kernel.rs`
- Create: i quattro `.stderr` corrispondenti — **generati una volta, poi letti**

**Interfaces:**
- Consumes: la crate `kernel` del Task 1, con i suoi tre attributi.
- Produces: il banco `trybuild`, che ogni regola di livello 1 successiva estende con un
  caso e il suo `.stderr`. Il file `compile_fail.rs` è il punto di ingresso e resta uno solo.

- [ ] **Step 1: Scrivere il banco che fallisce perché non ci sono casi**

`crates/kernel/tests/compile_fail.rs`:

```rust
//! I test di compilazione fallita: le regole di livello 1, viste scattare.
//!
//! ⛔ GOTCHA #25 — GLI `.stderr` NON SI RIGENERANO IN BLOCCO.
//! `trybuild` offre `TRYBUILD=overwrite` per riscriverli tutti sull'output corrente.
//! Serve quando i messaggi del compilatore cambiano legittimamente. Usato senza leggerli,
//! ogni caso diventa «l'errore atteso è quello che è uscito» e la suite passa per sempre.
//! La rigenerazione è un atto deliberato e **si legge nel diff**.
//!
//! ⚠️ Un test di compilazione fallita ha forza di livello 1 e visibilità di livello 2
//! (§7.1.3): cancellarlo NON riapre la violazione, la rende invisibile.

#[test]
fn le_regole_di_livello_1_non_compilano() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
}
```

- [ ] **Step 2: Lanciarlo e vedere che non prova niente**

```bash
cargo test -p kernel --test compile_fail
```

Atteso: **fallisce**, perché la cartella `tests/compile_fail/` non esiste ancora.
⚠️ Se passasse trovando zero casi, sarebbe il gotcha #26 applicato al banco: un controllo
che non trova niente da controllare **esce verde**. Verificare che il messaggio nomini il
percorso mancante.

- [ ] **Step 3: Scrivere i quattro casi negativi**

`crates/kernel/tests/compile_fail/std_nel_kernel.rs`:

```rust
// Regola: il kernel non NOMINA `std`. Meccanismo: `#![no_std]`. Forza: compilatore, E0433.
// Difende: I3 · V28 · V29 — §7.4.1 blocco A.
#![no_std]

fn legge_un_file() {
    let _ = std::fs::read_to_string("/etc/passwd");
}

fn main() {}
```

`crates/kernel/tests/compile_fail/unsafe_nel_kernel.rs`:

```rust
// Regola: niente `unsafe` nel kernel. Meccanismo: `#![forbid(unsafe_code)]`.
// Forza: compilatore. Difende: ADR-0026 vincolo 2 — §7.4.1 blocco A.
#![no_std]
#![forbid(unsafe_code)]

fn deferenzia() -> u8 {
    let x: u8 = 1;
    let p = &raw const x;
    unsafe { *p }
}

fn main() {}
```

`crates/kernel/tests/compile_fail/allow_unsafe_scavalca.rs`:

```rust
// ⛔ È IL CASO CHE GIUSTIFICA `forbid` INVECE DI `deny`, e senza di lui il vincolo 2 della
// §11 è una preferenza stilistica. Con `deny`, questo file COMPILA. Con `forbid`, il
// compilatore rifiuta l'`#[allow]` stesso: E0453.
#![no_std]
#![forbid(unsafe_code)]

#[allow(unsafe_code)]
fn deferenzia() -> u8 {
    let x: u8 = 1;
    let p = &raw const x;
    unsafe { *p }
}

fn main() {}
```

`crates/kernel/tests/compile_fail/hashmap_nel_kernel.rs`:

```rust
// Regola: `HashMap` non è nominabile nel kernel. Meccanismo: conseguenza GRATUITA di
// `no_std` — `HashMap` vive in `std`, non in `alloc`. Forza: compilatore, E0433.
// Difende: V29 · gotcha #12 — `RandomState` è seminato per processo, e l'ordine di
// iterazione non è riproducibile fra esecuzioni.
#![no_std]

extern crate alloc;

fn conta() {
    let _m: std::collections::HashMap<u8, u8> = std::collections::HashMap::new();
}

fn main() {}
```

- [ ] **Step 4: Generare gli `.stderr` UNA VOLTA, e leggerli**

```bash
TRYBUILD=overwrite cargo test -p kernel --test compile_fail
```

⛔ **Questa è l'unica volta in cui `overwrite` è lecito in questo piano.** Subito dopo,
aprire i quattro file e verificare **a occhio** che ciascuno contenga il codice d'errore
giusto:

| File | Deve contenere |
|---|---|
| `std_nel_kernel.stderr` | `E0433` — *failed to resolve: use of unresolved module or unlinked crate `std`* |
| `unsafe_nel_kernel.stderr` | *usage of an `unsafe` block* — con `forbid` |
| `allow_unsafe_scavalca.stderr` | **`E0453`** — *allow(unsafe_code) incompatible with previous forbid* |
| `hashmap_nel_kernel.stderr` | `E0433` sul percorso `std::collections::HashMap` |

⚠️ Se un `.stderr` contiene un errore **diverso** da quello atteso, il caso fallisce per il
motivo sbagliato: è il gotcha #9, e va corretto il caso, non l'oracolo.

- [ ] **Step 5: Rilanciare, senza `overwrite`, e vedere il verde**

```bash
cargo test -p kernel --test compile_fail
```

Atteso: **4 casi, 4 passati**.

- [ ] **Step 6: Provare la contro-sonda — le stesse cose sono lecite in `platform`**

`crates/platform/tests/contro_sonde.rs`:

```rust
//! Le contro-sonde di §7.1.1 regola 3: i divieti del kernel NON scattano dove non devono.
//! Senza queste, una regola troppo larga passerebbe per una regola che funziona — è la
//! direzione che si dimentica, e in M-3 la sonda decisiva è stata proprio questa.

#[test]
fn platform_nomina_std_e_compila() {
    assert!(platform::contro_sonda_std_compila());
}

#[test]
fn platform_usa_unsafe_e_compila() {
    assert_eq!(platform::contro_sonda_unsafe_compila(), 42);
}
```

- [ ] **Step 7: Lanciare le contro-sonde**

```bash
cargo test -p platform --test contro_sonde
```

Atteso: **2 passati**. Il fatto stesso che compilino è la prova.

- [ ] **Step 8: Commit**

```bash
git add crates/kernel/tests crates/platform/tests && git commit -m "test(porta): livello 1 provato in due direzioni — no_std, forbid, HashMap"
```

---

## Task 3: Il manifesto delle dipendenze, con `bincode` appuntato a `2`

**Files:**
- Modify: `crates/kernel/Cargo.toml`
- Create: `crates/kernel/tests/dipendenze_utilizzabili.rs`

**Interfaces:**
- Consumes: la crate `kernel` del Task 1.
- Produces: le tre voci **spedite** (`bincode`, `unty`, `minicbor`) disponibili al kernel, e
  il grafo di build a sette voci che il Task 5 misura.

- [ ] **Step 1: Scrivere il test che dimostra che le dipendenze sono utilizzabili in `no_std`**

`crates/kernel/tests/dipendenze_utilizzabili.rs`:

```rust
//! Che una versione esista non vuol dire che funzioni — gotcha #22.
//! `cargo add bincode` risolve alla 3.0.0, il cui INTERO SORGENTE è un `compile_error!`.
//! Questo test non prova la logica: prova che le voci spedite si compilano e si usano.
//!
//! ⛔ QUESTO FILE NON DICHIARA `#![no_std]`, E NON È UNA DIMENTICANZA.
//! Un test di integrazione è una crate a sé, e il banco di `#[test]` ha bisogno di `std`
//! per girare: con `#![no_std]` qui, il file non collega e fallisce per il motivo
//! sbagliato — gotcha #9. La prova che le dipendenze reggono **senza sistema operativo**
//! non è questo test: è `scripts/gate-no-os.sh` (Task 4), che compila `kernel` per
//! `x86_64-unknown-none`. Quello è il meccanismo; questo è solo il round-trip.

#[test]
fn bincode_2_fa_round_trip_in_no_std() {
    let atteso: u32 = 4096;
    let byte: Vec<u8> =
        bincode::encode_to_vec(atteso, bincode::config::standard()).expect("codifica");
    assert!(!byte.is_empty());
    let (letto, consumati): (u32, usize) =
        bincode::decode_from_slice(&byte, bincode::config::standard()).expect("decodifica");
    assert_eq!(letto, atteso);
    // I byte consumati pareggiano la lunghezza: è la regola che il gotcha #34 impone sul
    // canale a frame, e vale la pena esercitarla da subito.
    assert_eq!(consumati, byte.len());
}

#[test]
fn minicbor_fa_round_trip_in_no_std() {
    let atteso: u32 = 4096;
    let mut byte: Vec<u8> = Vec::new();
    minicbor::encode(atteso, &mut byte).expect("codifica");
    let letto: u32 = minicbor::decode(&byte).expect("decodifica");
    assert_eq!(letto, atteso);
}
```

- [ ] **Step 2: Lanciarlo e vederlo fallire**

```bash
cargo test -p kernel --test dipendenze_utilizzabili
```

Atteso: **FAIL** con *use of unresolved module or unlinked crate `bincode`* — le
dipendenze non sono ancora dichiarate.

- [ ] **Step 3: Dichiarare le tre voci spedite, ciascuna con la propria giustificazione**

Sostituire il blocco `[dependencies]` di `crates/kernel/Cargo.toml`:

```toml
# ⛔ OGNI VOCE QUI È UNA RIGA DELLA LISTA DI ADR-0031, e aggiungerne una è un atto
# deliberato e rivedibile. La giustificazione si scrive accanto, non altrove: la sede
# unica della lista è la §7.3.1 della spec, e questo manifesto la rispecchia.
# Il controllo che le verifica è scripts/gate-deps.sh, sui DUE grafi.
[dependencies]

# Serializza lo schema del canale privato `ipc` (I4). Raggiunge: NULLA — compila per un
# bersaglio senza OS, e nel grafo non compare nessuna sorgente di casualità.
# ⛔ APPUNTATO A "2" E NON A "3": la 3.0.0 è l'ultima pubblicata e il suo intero sorgente
# è `compile_error!("https://xkcd.com/2347/")`, un segnaposto contro l'occupazione del
# nome. Senza questo pin, il prossimo aggiornamento «sistema» il vincolo e rompe la build.
# Gotcha #22, vincolo 3 della §11.
bincode = { version = "2", default-features = false, features = ["alloc", "derive"] }

# ⚠️ `unty` 0.0.4 NON si dichiara qui, ed è deliberato: arriva come dipendenza di
# `bincode`, e dichiararla direttamente creerebbe una voce inutilizzata. È comunque IN
# LISTA in scripts/gate-deps.sh, perché il controllo misura il grafo TRANSITIVO — il
# pericolo arriva di rimbalzo, ed è la ragione per cui ADR-0031 non guarda le sole
# dipendenze dirette.

# Codifica il record durevole del giornale, per indice esplicito (§4.9, ADR-0036), E lo
# schema del canale verso i worker (§6.10, ADR-0037). Una voce sola per due artefatti: il
# canale riusa una voce già spedita, e la lista NON cresce.
minicbor = { version = "2.3.0", default-features = false, features = ["alloc", "derive"] }
```

⚠️ **`default-features = false` non è cosmesi:** con le feature predefinite il grafo si
allarga, e `cargo tree` risolve le feature mentre `cargo metadata` no — gotcha #23,
misurato: undici crate segnalate contro le due reali.

- [ ] **Step 4: Lanciare il test e vederlo passare**

```bash
cargo test -p kernel --test dipendenze_utilizzabili
```

Atteso: **2 passati**.

- [ ] **Step 5: Verificare che il grafo sia quello che la spec dichiara**

```bash
cargo tree -p kernel -e normal,no-proc-macro
```

Atteso: `kernel`, `bincode`, `unty`, `minicbor` — **tre voci spedite**, e nient'altro.

```bash
cargo tree -p kernel -e no-dev
```

Atteso: le tre di sopra **più** le sette di build: `bincode_derive`, `virtue`,
`minicbor-derive`, `syn`, `quote`, `proc-macro2`, `unicode-ident`.
⚠️ `syn` è la voce da guardare in faccia: `bincode_derive` usa `virtue` apposta per
evitarlo, e `minicbor-derive` lo tira dentro. Non viola V29 a runtime — è la ragione per
cui la classe «di build» esiste — ma **è superficie di supply chain**.

- [ ] **Step 6: Commit**

```bash
git add crates/kernel && git commit -m "feat(kernel): le tre voci spedite, con bincode appuntato a 2"
```

---

## Task 4: Il cancello senza OS

**Files:**
- Create: `scripts/gate-no-os.sh`

**Interfaces:**
- Consumes: le crate `kernel` e `simulator` del Task 1, con le dipendenze del Task 3.
- Produces: `scripts/gate-no-os.sh`, che il Task 6 richiama dentro `scripts/gate.sh`.
  Uscita 0 = passa, 1 = fallisce.

- [ ] **Step 1: Scrivere il cancello**

`scripts/gate-no-os.sh`:

```bash
#!/usr/bin/env bash
# Il cancello senza OS -- §7.3.2 della spec del sotto-progetto 1.
#
# SI AGGIUNGE alla allow-list, non la sostituisce. I due falliscono in modo
# complementare: la lista ENUMERA e nomina il colpevole ("X unty <- kernel -> bincode
# -> unty"); il cancello PROVA e coglie una crate GIA' IN LISTA che raggiunge l'OS per
# una via non prevista -- l'unificazione delle feature -- ma dice solo "no" senza dire
# chi. La lista e' la diagnosi, il cancello e' la prova.
#
# BERSAGLIO: x86_64-unknown-none, e non e' un dettaglio. Deve differire dal bersaglio
# reale in UNA SOLA dimensione, l'assenza dell'OS. thumbv7em-none-eabihf ne differisce
# per quattro (arch, puntatore, atomici a 64 bit) ed e' una sorgente di rossi per il
# motivo sbagliato -- gotcha #9 applicato al bersaglio.
#
# ⛔ NON aggiungere --workspace. Il comando nomina le DUE crate vincolate, e non e' una
# comodita': con --workspace il cancello fallisce su `platform` con "can't find crate
# for std", cioe' motivo giusto e crate sbagliata. E' la sonda B3, che non esisteva.
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1

BERSAGLIO=x86_64-unknown-none

echo "== cancello senza OS -- $BERSAGLIO =="

if ! rustup target list --installed | grep -qx "$BERSAGLIO"; then
  echo "  ✗ bersaglio $BERSAGLIO non installato."
  echo "    rustup target add $BERSAGLIO   (o affidati a rust-toolchain.toml)"
  echo "    Senza, la porta sarebbe rossa per il motivo sbagliato -- vincolo 4 della §11."
  exit 1
fi

if cargo build -p kernel -p simulator --target "$BERSAGLIO" 2>&1; then
  echo "  ✓ kernel e simulator compilano senza sistema operativo"
  exit 0
else
  echo "  ✗ kernel o simulator NON compilano per $BERSAGLIO."
  echo "    Il cancello non dice chi l'ha tirata dentro: guarda l'uscita di"
  echo "    scripts/gate-deps.sh, che nomina il rimbalzo."
  exit 1
fi
```

- [ ] **Step 2: Sonda B1 — lo stato pulito passa**

```bash
bash scripts/gate-no-os.sh
```

Atteso: `✓ kernel e simulator compilano senza sistema operativo`, uscita **0**.

- [ ] **Step 3: Sonda B2 — il cancello scatta su una dipendenza che tocca l'OS**

Aggiungere **temporaneamente** a `crates/kernel/Cargo.toml`, sotto `[dependencies]`:

```toml
getrandom = "0.2"
```

Poi:

```bash
bash scripts/gate-no-os.sh
```

Atteso: **uscita 1**, con `target is not supported` nell'output. È la sonda B2, misurata
il 2026-08-07 su entrambi i bersagli con lo **stesso** messaggio.

⛔ **Rimuovere la riga subito dopo**, e verificare che il cancello torni verde:

```bash
bash scripts/gate-no-os.sh
```

Atteso: uscita **0**.

- [ ] **Step 4: Sonda B3 — la contro-sonda che quasi non si scrive**

Verificare **a mano**, senza modificare lo script, che aggiungere `--workspace` lo
romperebbe per la ragione sbagliata:

```bash
cargo build --workspace --target x86_64-unknown-none
```

Atteso: **fallisce** su `platform` con `can't find crate for std` — motivo giusto, crate
sbagliata. Questo comando **non** entra nello script: è la prova che il comando scritto
nomina le due crate vincolate di proposito. Annotarne l'esito nel messaggio di commit.

- [ ] **Step 5: Commit**

```bash
git add scripts/gate-no-os.sh && git commit -m "feat(porta): cancello senza OS su x86_64-unknown-none, sonde B1 B2 B3"
```

---

## Task 5: L'allow-list sui due grafi

**Files:**
- Create: `scripts/gate-deps.sh`

**Interfaces:**
- Consumes: le tre voci spedite del Task 3 e le sette di build che ne discendono.
- Produces: `scripts/gate-deps.sh`. Due liste, due comandi, **due errori diversi con due
  rimedi diversi**. Uscita 0 = passa, 1 = fallisce.

- [ ] **Step 1: Scrivere il controllo, con le due liste e la guardia di non-vacuità**

`scripts/gate-deps.sh`:

```bash
#!/usr/bin/env bash
# L'allow-list sul grafo transitivo di kernel e simulator -- ADR-0031, §7.3.1.
#
# DUE GRAFI, DUE ERRORI, DUE RIMEDI OPPOSTI. Non e' completezza: unificarli insegna il
# riflesso "aggiungi alla lista" ANCHE per una violazione di I3, dove aggiungere E' la
# violazione. E un controllo che guarda solo cio' che spedisce lascia passare in silenzio
# proprio l'evento che ADR-0031 dice di rivedere.
#
#   spedita   -> `cargo tree -e normal,no-proc-macro` -> errore "I3 violato"
#                RIMEDIO: togliere la dipendenza. Aggiungerla alla lista NON e' un rimedio.
#   di build  -> il complemento fra `-e no-dev` e la riga sopra
#                RIMEDIO: valutare e aggiungere alla lista, con giustificazione.
#   di sviluppo -> esclusa, e l'esclusione e' PROVATA (vedi la guardia sotto).
#
# ⚠️ `cargo tree` e non `cargo metadata`: il secondo non risolve le feature. Misurato,
# gotcha #23: undici crate segnalate contro le due reali, cioe' 5x di sovra-segnalazione.
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1

fallimenti=0
segnala() { echo "  ✗ $*"; fallimenti=$((fallimenti + 1)); }

# --- Le due liste. Sede unica: la §7.3.1 della spec; questo file la rispecchia. ---
SPEDITE="bincode
kernel
minicbor
simulator
unty"

DI_BUILD="bincode_derive
minicbor-derive
proc-macro2
quote
syn
unicode-ident
virtue"

nomi() { sed 's/^[^a-zA-Z0-9_-]*//' | awk '{print $1}' | grep -E '^[a-z0-9_-]+$' | sort -u; }

for crate in kernel simulator; do
  echo "== $crate: grafo SPEDITO =="
  spedito=$(cargo tree -p "$crate" -e normal,no-proc-macro --prefix none 2>/dev/null | nomi)
  intrusi=$(comm -23 <(printf '%s\n' "$spedito") <(printf '%s\n' "$SPEDITE" | sort -u))
  if [ -n "$intrusi" ]; then
    for i in $intrusi; do
      segnala "I3 violato -- $crate spedisce '$i', che non e' in lista."
      echo "      ⛔ RIMEDIO: TOGLIERE la dipendenza. Aggiungerla alla lista non e' un rimedio."
      echo "      Da dove arriva:"
      cargo tree -p "$crate" -e normal,no-proc-macro -i "$i" 2>/dev/null | sed 's/^/        /'
    done
  fi

  echo "== $crate: grafo DI BUILD =="
  completo=$(cargo tree -p "$crate" -e no-dev --prefix none 2>/dev/null | nomi)
  build=$(comm -13 <(printf '%s\n' "$spedito") <(printf '%s\n' "$completo"))
  nuove=$(comm -23 <(printf '%s\n' "$build") <(printf '%s\n' "$DI_BUILD" | sort -u))
  if [ -n "$nuove" ]; then
    for n in $nuove; do
      segnala "grafo di build cambiato -- '$n' non e' in lista."
      echo "      ✅ RIMEDIO: valutarla e AGGIUNGERLA alla lista, con la giustificazione."
      echo "      E' l'evento da rivedere che ADR-0031 dichiara fra le proprie Negative."
    done
  fi

  # Guardia di non-vacuita': se i due grafi COINCIDONO il filtro non distingue niente,
  # ed e' la condizione esatta in cui M-3 e' stata ingannata (§7.2.3). Non passa in
  # silenzio: il controllo lo SEGNALA.
  if [ "$spedito" = "$completo" ]; then
    segnala "$crate: grafo spedito e grafo completo COINCIDONO -- il filtro non sta distinguendo niente."
    echo "      Non e' 'la lista e' corta': e' 'l'interrogazione era stretta'."
  fi
done

echo
if [ "$fallimenti" -eq 0 ]; then
  echo "OK -- i due grafi corrispondono alle due liste."
else
  echo "$fallimenti violazioni. Leggi il RIMEDIO: NON e' lo stesso per i due grafi."
  exit 1
fi
```

- [ ] **Step 2: Sonda N1 — lo stato pulito passa**

```bash
bash scripts/gate-deps.sh
```

Atteso: `OK -- i due grafi corrispondono alle due liste.`, uscita **0**.

⚠️ Se compare la riga della guardia di non-vacuità, il filtro non sta distinguendo: il
grafo di build **deve** essere più grande di quello spedito, perché le sette voci derive
ci sono.

- [ ] **Step 3: Sonda N2 — una crate spedita fuori lista fa scattare «I3 violato»**

Aggiungere **temporaneamente** a `crates/kernel/Cargo.toml`:

```toml
libc = "0.2"
```

```bash
bash scripts/gate-deps.sh
```

Atteso: **uscita 1**, con `I3 violato -- kernel spedisce 'libc'`, il rimedio **«TOGLIERE»**,
e la catena che nomina il rimbalzo. ⛔ Rimuovere la riga e riverificare il verde.

- [ ] **Step 4: Sonda N3 — una crate di build fuori lista dà l'ALTRO errore**

Aggiungere **temporaneamente** a `crates/kernel/Cargo.toml`:

```toml
[build-dependencies]
cc = "1"
```

```bash
bash scripts/gate-deps.sh
```

Atteso: **uscita 1**, con `grafo di build cambiato -- 'cc' non e' in lista` e il rimedio
**«AGGIUNGERLA»**. ⛔ **Il messaggio deve essere diverso da quello di N2.** Se i due
errori si assomigliano, il controllo insegna il rimedio sbagliato — ed è il motivo per cui
i grafi sono due. Rimuovere e riverificare.

- [ ] **Step 5: Sonda N4 — la contro-sonda, ed è quella decisiva**

Aggiungere **temporaneamente** a `crates/platform/Cargo.toml`:

```toml
getrandom = "0.2"
```

```bash
bash scripts/gate-deps.sh
```

Atteso: **uscita 0, verde.** `platform` **non** è vincolata da ADR-0031 — è il posto dove
l'I/O deve vivere — e una regola che scattasse anche lì sarebbe una regola troppo larga
che passa per una regola che funziona. In M-3 la sonda decisiva è stata **N4**, non N1.
⛔ Rimuovere la riga.

- [ ] **Step 6: Commit**

```bash
git add scripts/gate-deps.sh && git commit -m "feat(porta): allow-list sui due grafi, con rimedi opposti — sonde N1..N4"
```

---

## Task 6: La porta di livello 2, in un comando solo

**Files:**
- Create: `scripts/gate.sh`
- Create: `.github/workflows/porta.yml`

**Interfaces:**
- Consumes: `scripts/gate-no-os.sh` (Task 4), `scripts/gate-deps.sh` (Task 5),
  `scripts/check-docs.sh` (già esistente), e i test dei Task 2 e 3.
- Produces: `scripts/gate.sh`, il comando unico della cadenza «a ogni commit».

- [ ] **Step 1: Scrivere la porta**

`scripts/gate.sh`:

```bash
#!/usr/bin/env bash
# La porta di qualita', livello 2 -- §7.5.1 della spec del sotto-progetto 1.
#
# CADENZA: a ogni commit. Il livello 1 NON e' qui, e non e' una dimenticanza: le voci di
# livello 1 non "girano" mai -- SONO il compilatore. Se il codice compila, quelle regole
# valgono, e non esiste un modo di saltarle o di rimandarle a stasera.
#
# ⛔ Un rosso di questa porta significa sempre "invariante violata", mai "stile
# discutibile". `clippy` gira come igiene del codice ma NON ha voce qui: nessun V dipende
# da lui, e la regola 1 del criterio di ammissione (§7.1.1) dice che allora non entra.
# Il livello 3 del catalogo e' VUOTO, ed e' una decisione (§7.4.3).
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1

fallimenti=0
esegui() {
  echo
  echo "######## $1"
  shift
  if "$@"; then :; else fallimenti=$((fallimenti + 1)); fi
}

esegui "compilazione del workspace"        cargo build --workspace
esegui "test a esempi e compile-fail"      cargo test --workspace
esegui "cancello senza OS"                 bash scripts/gate-no-os.sh
esegui "allow-list sui due grafi"          bash scripts/gate-deps.sh
esegui "coerenza della documentazione"     bash scripts/check-docs.sh

echo
if [ "$fallimenti" -eq 0 ]; then
  echo "PORTA VERDE."
else
  echo "PORTA ROSSA -- $fallimenti controlli falliti."
  exit 1
fi
```

- [ ] **Step 2: Lanciare la porta intera**

```bash
bash scripts/gate.sh
```

Atteso: `PORTA VERDE.`, uscita **0**.

- [ ] **Step 3: Provare che la porta è capace di diventare rossa**

Rompere **temporaneamente** una cosa sola — togliere `#![no_std]` da
`crates/kernel/src/lib.rs`:

```bash
bash scripts/gate.sh
```

Atteso: **PORTA ROSSA**, e il fallimento è nei test di compilazione fallita: il caso
`std_nel_kernel.rs` ora **compila**, quindi `trybuild` lo segnala.
⚠️ È la guardia di non-vacuità applicata alla porta stessa: una porta che non si è mai
vista diventare rossa non è una porta. ⛔ Ripristinare la riga e riverificare il verde.

- [ ] **Step 4: Scrivere il flusso di CI**

`.github/workflows/porta.yml`:

```yaml
name: porta di qualità

on:
  push:
    branches: ["**"]
  pull_request:

jobs:
  porta:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      # rust-toolchain.toml dichiara versione e bersaglio: rustup li installa da solo,
      # ed è il motivo per cui il vincolo 4 della §11 non richiede un passo qui.
      - run: rustup show
      - run: bash scripts/gate.sh
```

- [ ] **Step 5: Commit**

```bash
git add scripts/gate.sh .github/workflows/porta.yml && git commit -m "feat(porta): livello 2 in un comando solo, e la CI lo lancia"
```

---

## Task 7: Il registro dei controlli, allineato al catalogo

**Files:**
- Create: `docs/porta-di-qualita.md`
- Modify: `docs/README.md`

**Interfaces:**
- Consumes: tutti i controlli dei Task 2–6.
- Produces: la tabella che mappa **ogni riga del catalogo §7.4 al file che la implementa**.
  È ciò che permette al prossimo traguardo di sapere dove aggiungere un controllo.

- [ ] **Step 1: Scrivere il registro**

`docs/porta-di-qualita.md`:

```markdown
# La porta di qualità — dove vive ogni controllo

> Questo file non decide niente. Il catalogo è la
> [§7.4 della spec](superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md); qui c'è
> soltanto la mappa fra ogni riga del catalogo e il file che la implementa.
>
> ⛔ **Un controllo nuovo entra prima nel catalogo, poi qui.** L'ordine inverso è il
> gotcha #36: una sezione decide un meccanismo, lo scrive nella propria tabella, e il
> catalogo resta indietro — è già successo due volte.

**Un comando solo:** `bash scripts/gate.sh`

## Livello 1 — il compilatore

| Regola del catalogo | Dove | Caso negativo |
|---|---|---|
| `#![no_std]` su `kernel` e `simulator` | `crates/kernel/src/lib.rs`, `crates/simulator/src/lib.rs` | `tests/compile_fail/std_nel_kernel.rs` |
| `#![forbid(unsafe_code)]` sulle stesse | idem | `tests/compile_fail/unsafe_nel_kernel.rs` · `allow_unsafe_scavalca.rs` |
| `HashMap` non nominabile | conseguenza gratuita di `no_std` | `tests/compile_fail/hashmap_nel_kernel.rs` |

**Contro-sonde:** `crates/platform/tests/contro_sonde.rs` — `platform` nomina `std` e usa
`unsafe`, e **compila**. Sono la direzione che si dimentica (§7.1.1 regola 3).

## Livello 2 — controlli esterni

| Regola del catalogo | Dove | Sonde viste scattare |
|---|---|---|
| allow-list, grafo **spedito** | `scripts/gate-deps.sh` | N1 · N2 |
| allow-list, grafo **di build** | idem, e l'errore è **diverso** | N3 |
| cancello senza OS | `scripts/gate-no-os.sh` | B1 · B2 |
| coerenza della documentazione | `scripts/check-docs.sh` | S1…S8, §8.6.3 e §8.5.4 |

**Contro-sonde:** N4 — `getrandom` in `platform` e il controllo **resta verde**. B3 —
`--workspace` fallirebbe su `platform` per il motivo giusto e la crate sbagliata.

## Livello 3 — vuoto, e non è una svista

`clippy` gira come igiene del codice ma **non ha voce nella porta**: nessun V dipende da
lui. Un rosso della porta deve significare sempre «invariante violata», mai «stile
discutibile», o si impara a ignorarlo.

## Cosa la porta NON controlla, in questo traguardo

| | |
|---|---|
| i **byte congelati** del record durevole | non esiste ancora nessun record. Entrano al **primo** record scritto — vincolo 14 della §11, Traguardo 3 |
| la **campagna DST** | non esiste ancora il simulatore. Traguardo 4 |
| i **test di contratto** fra porta finta e porta vera | non esistono ancora le porte. Traguardo 2 |
```

- [ ] **Step 2: Aggiungere la voce all'indice**

In `docs/README.md`, nella tabella dei documenti, aggiungere questa riga:

```markdown
| [La porta di qualità](porta-di-qualita.md) | dove vive ogni controllo della porta, mappato riga per riga sul catalogo §7.4. Un comando solo: `bash scripts/gate.sh` |
```

⚠️ Se la tabella di `README.md` ha un numero di colonne diverso da due, adeguare la riga:
`check-docs.sh` non lo verifica, ma una tabella sfasata si legge male e nessuno la corregge.

- [ ] **Step 3: Verificare che l'audit resti verde**

```bash
bash scripts/check-docs.sh
```

Atteso: `OK — nessuna incoerenza.` ⚠️ Il controllo dei link interni è quello che coglierebbe
un rimando sbagliato dentro il file nuovo.

- [ ] **Step 4: Commit**

```bash
git add docs/porta-di-qualita.md docs/README.md && git commit -m "docs: il registro dei controlli, mappato sul catalogo §7.4"
```

---

## Task 8: Chiusura del traguardo, e allineamento dei documenti di stato

**Files:**
- Modify: `docs/COMPENDIO.md` (§4 e §6)
- Modify: `docs/HANDOFF.md`
- Modify: `CLAUDE.md`

**Interfaces:**
- Consumes: tutto ciò che i Task 1–7 hanno prodotto.
- Produces: lo stato allineato. È l'ultimo passo, e non è opzionale.

- [ ] **Step 1: Lanciare la porta intera, un'ultima volta**

```bash
bash scripts/gate.sh
```

Atteso: `PORTA VERDE.` ⛔ Se non lo è, il traguardo **non è chiuso**: si corregge, non si
dichiara.

- [ ] **Step 2: Aggiornare il compendio**

Nella **§6** — «Dove siamo, e cosa viene dopo» — sostituire *«Zero righe di codice del
prodotto»* con lo stato reale: il workspace esiste, le cinque crate esistono e sono vuote,
la porta di qualità è **eseguibile e provata in due direzioni**, e il prossimo passo è il
**Traguardo 2 — il substrato iniettabile**.

Nella **§4** — lo stack — aggiungere la riga della **edition `2024`** e del
`rust-toolchain.toml`, che sono le due scelte prese da questo piano.

- [ ] **Step 3: Aggiornare l'handoff**

In `docs/HANDOFF.md`, il punto di ripresa e la sezione «Prima cosa da fare»: il piano è
eseguito, il codice è cominciato, e la §2.5 diventa la mappa del Traguardo 2.

⚠️ Aggiungere un gotcha **solo se ne è emerso uno vero** durante l'esecuzione. Un piano che
va liscio non ne produce; inventarne uno diluisce quelli che contano.

- [ ] **Step 4: Aggiornare `CLAUDE.md`**

La riga *«Oggi l'unico codice è in [`spikes/`](spikes/), e sono **prove**»* **non è più
vera**. Sostituire il paragrafo con:

```markdown
⚠️ **Questo non è un repository di sola documentazione.** Il codice del prodotto si
scrive **qui**, e vive in [`crates/`](crates/): cinque crate, con `kernel` e `simulator`
in `no_std`. Gli spike in [`spikes/`](spikes/) restano **prove**, fuori dal workspace.
La porta di qualità si lancia con un comando solo — `bash scripts/gate.sh` — e la mappa
dei controlli è in [`docs/porta-di-qualita.md`](docs/porta-di-qualita.md).
Lo stato corrente e il prossimo passo stanno nella **§6 del compendio** — non qui, o si
disallineano.
```

- [ ] **Step 5: Audit e commit**

```bash
bash scripts/check-docs.sh
```

Atteso: `OK — nessuna incoerenza.`

```bash
git add -A && git commit -m "docs: traguardo 1 chiuso — scheletro e porta di qualità" && git push
```

---

## Definizione di «fatto» per questo traguardo

Tutte e sei, o il traguardo non è chiuso.

| # | Condizione | Come si verifica |
|---|---|---|
| 1 | Le cinque crate esistono, `kernel` non dipende da nessuna crate del progetto | `cargo tree -p kernel -e normal` mostra `kernel` e le sole tre voci spedite |
| 2 | `kernel` e `simulator` sono `no_std` + `alloc` + **`forbid`** | i quattro casi di `tests/compile_fail/` passano, `allow_unsafe_scavalca` compreso |
| 3 | Ogni controllo si è visto **scattare** | sonde B1 B2 · N1 N2 N3, ciascuna eseguita e il messaggio letto |
| 4 | Ogni controllo si è visto **restare verde** dove la cosa è lecita | contro-sonde B3 · N4 · `platform/tests/contro_sonde.rs` |
| 5 | La porta gira in un comando solo, e la si è vista diventare **rossa** | `bash scripts/gate.sh` verde; poi rotta di proposito, rossa; poi ripristinata, verde |
| 6 | I documenti di stato non mentono | `bash scripts/check-docs.sh` verde, e §6 del compendio non dice più «zero righe di codice» |

⛔ **Cosa NON è nella definizione di fatto, e non va aggiunto:** che il kernel faccia
qualcosa. Non fa niente, ed è il punto.
