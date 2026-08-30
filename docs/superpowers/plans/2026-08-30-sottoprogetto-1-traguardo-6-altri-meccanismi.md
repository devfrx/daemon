# Traguardo 6 — gli altri meccanismi: il piano

> **Per chi esegue:** SOTTO-SKILL OBBLIGATORIA — `superpowers:subagent-driven-development`,
> un subagente fresco per compito con revisione fra uno e l'altro. È la modalità scelta dal
> proprietario e ha portato tutti i compiti eseguiti finora. I passi usano le caselle
> (`- [ ]`) per il tracciamento.

**Obiettivo.** Costruire gli ultimi meccanismi del kernel — la concessione che torna,
lo schema del canale worker, lo schema `ipc`, il contratto del sensore, il decisore del
gateway col suo gettone, il permesso, lo stato di degrado — e chiudere con essi le righe di
catalogo che li aspettano.

**Architettura.** Nessun meccanismo nuovo di trasporto: il traguardo costruisce **forme e
logica dentro `kernel`**, più una finta di `ipc` in `simulator`. `platform` non guadagna
niente — il trasporto vero di `ipc` e `process` è scaglionato da §0.2 e §0.4 riga §1. Le
decisioni di forma sono già prese dal
[disegno](../specs/2026-08-28-sottoprogetto-1-traguardo-6-altri-meccanismi-design.md); questo
piano le traduce in passi.

**Stack.** Rust edition 2024, toolchain appuntata `1.95.0`, `kernel` e `simulator` in
`no_std` + `alloc` + `forbid(unsafe_code)`. `minicbor` 2.3.0 per il giornale e per il canale
worker; il formato dello schema `ipc` lo decide il **compito 3bis**. La porta di qualità è
`bash scripts/gate.sh`, e deve stampare `GATE GREEN` a **ogni commit**.

---

## ⚠️ L'errata di questo piano — si legge PRIMA di ogni compito, non una volta sola

⛔ **Nasce vuota, e non resterà vuota.** Il pre-controllo ha trovato un difetto reale in
**tutti** i compiti dispacciati finora, senza una sola eccezione: quando ne trovi uno, si
scrive **qui**, con il proprio numero, prima di eseguirlo. Un piano è un'ipotesi.

| # | Voce |
|---|---|
| — | *(nessuna ancora)* |

---

## Il pre-controllo del piano — che cosa il disegno dice e il codice smentisce

⛔ **Fatto il 2026-08-30 leggendo il disegno contro il codice di quel giorno**, come il
gotcha **#58** prescrive per un disegno e il **#65** per qualunque documento che prezza
lavoro. Quattro voci, tutte **misurate**.

### P-1 — Il disegno sbagliava la RAGIONE del perimetro, non il perimetro

Il richiamo del 2026-08-29 in §1.2 attribuiva la non-costruzione del trasporto reale alla metà
di **prontezza** della porta `reactor`, che non ha un produttore. **Falso, misurato:** le due
porte sono **a interrogazione** per costruzione — `Ipc::accept` rende `Option<ClientId>` senza
attendere, `Ipc::receive` e `Worker::read_next` rendono `Ok(None)` come risposta ordinaria, e
il doc di `receive` scrive che senza di essa *«the core could not poll this port at all»*.

✅ **Chiuso il 2026-08-30, prima che questo piano esistesse:** la §1.2 porta il richiamo
datato, la **voce 5** del disegno è chiusa, e le due celle della §7.4.6 della spec passano a
**❌ scaglionata**. **Il perimetro non cambia.** Terza occorrenza del **#58**, in una forma
nuova: il documento *aveva* letto codice — `reactor.rs`, vero in ogni parola — e non le due
porte che stava prezzando.

### P-2 — La deduzione della §4.3 è MISURATA, e la risposta ha una terza parte che il disegno non nominava

La §4.3 dichiara: *«che aggiungere una variante lasci i byte congelati identici è una
DEDUZIONE, non una misura. Si misura prima di scrivere»*. **Misurato il 2026-08-30**
aggiungendo `#[n(3)] Routing` a `RecordKind`, e revocato da copia byte-esatta con `git diff` a
zero righe:

| Domanda | Esito misurato |
|---|---|
| i byte congelati restano identici? | ✅ **sì** — `#[cbor(index_only)]` codifica l'indice nudo, quindi `00`/`01`/`02` non si muovono. `frozen_bytes` **6 su 6**, `record_shape` **12 su 12**, nessun `.cbor` toccato |
| è gratis? | ⛔ **no, e il no è una GARANZIA:** non compila finché **due** match esaustivi non decidono — `crates/kernel/src/reconcile.rs:90`, dove la riconciliazione deve dire che cosa la variante nuova significhi per il dubbio, e `crates/kernel/tests/frozen_bytes.rs:224`, l'oracolo stesso |
| il nuovo indice è **pinzato**? | ⛔ **NO, ed è il limite che il banco DICHIARA di avere.** Riprodotto: estendendo l'`arm` senza estendere l'**array a mano** `[RecordKind::Intent, RecordKind::Outcome, RecordKind::Note]`, tutto **compila e resta verde**, e il nuovo indice è tenuto da **nulla** — esattamente la condizione che quel test esiste per impedire sugli altri otto |

📌 **Conseguenza sul compito 6, e non è una rifinitura:** chi aggiunge la variante deve
estendere **anche l'array** e congelare un **quarto record** che la porti, o il traguardo
consegna un indice di filo difeso da niente. Il compilatore **non** lo dice.

### P-3 — La deduzione della §5.1 su `check-docs.sh` è MISURATA, e regge

La §5.1 dichiara *«NON MISURATO: che `check-docs.sh` … non possa verificare che un ✅ nomini un
controllo esistente»*. **Misurato:** lo script ha **undici** passi, e nessuno confronta uno
stato con l'esistenza del controllo che nomina — il più vicino è
`== §8: every V and every Q has a state, and the deferred ones have their trigger ==`, che
verifica l'**innesco** dei rimandati e non il **referente** dei ✅. Il comando che rifà la
misura, invece della cifra:

```bash
grep -nE '^echo "== ' scripts/check-docs.sh
```

⚖️ **La deduzione diventa misura, e la conclusione del disegno non cambia:** `V10`, `V14` e
`Q10` portano ✅ senza controllo e nessun cancello lo dice. Resta **voce 4** del proprietario
se lo script debba imparare a dirlo.

### P-4 — ⛔ Il costo di `E21` è più grande di come la §2.4 lo prezza, e la differenza è nei casi `compile_fail`

La §2.4 dice che `Parameters` guadagna un campo e *«tocca la radice di composizione e il
banco»*, e conta **tre** casi `compile_fail` toccati — i tre della porta `process`.
**Misurato: sono nove, e i tre nominati sono un sottoinsieme.**

`Parameters::new` è **posizionale**, quindi un terzo campo rompe **ogni** sito. Il censimento,
col comando che lo rifà:

```bash
grep -rn "Parameters::new" crates/ --include=*.rs | wc -l     # i siti
grep -rln "Parameters::new" crates/ --include=*.rs | wc -l    # i file
grep -rln "Parameters::new" crates/kernel/tests/compile_fail/*.rs
```

⛔ **Perché i nove casi `compile_fail` sono la parte che fa male, e non i quarantadue siti
ordinari:** un sito ordinario che non compila è un rosso che si legge e si corregge. Un caso
`compile_fail` che smette di fallire **per la ragione che asserisce** e comincia a fallire per
**arità sbagliata** diventa un `mismatch`: l'oracolo che quel caso *è* si spegne, e il rimedio
è rileggere il proprio `.stderr` **uno per uno, mai rigenerarli in blocco** — vincolo 10 della
§11, gotcha **#25**.

⚠️ **Due di essi nominano `Parameters` nel proprio `.stderr`** — `parameters_have_no_default`
e `two_policies_at_once` — quindi sono i due il cui testo atteso può cambiare davvero; gli
altri sette cadrebbero **prima** di arrivare all'errore che asseriscono.

📌 **Non è una ragione per spostare `ArbiterId` fuori da `Parameters`:** ADR-0034 lo colloca lì
e §6.1.3 vieta di generarlo, quindi il costo si **paga e si dichiara**. È il gotcha **#65**
nella direzione che il disegno prezza **meno**: qui il lavoro è più grande di come è scritto.

---

## Le decisioni prese da questo piano

⛔ **Sono decisioni del piano, non del disegno, e chi esegue può ribaltarle** portando la
misura che le smentisce — è ciò per cui esiste l'errata.

| | Decisione | Perché |
|---|---|---|
| **D1** | ⛔ **Il compito 1 si consegna in TRE commit, ciascuno `GATE GREEN`** — l'identità, poi `release`, poi la porta `process` | il vincolo globale 8 vuole la porta verde a ogni commit, e le tre parti hanno raggi diversi: la prima tocca quarantadue siti, la terza nove `.stderr`. Un commit solo renderebbe irriconoscibile quale dei tre ha rotto cosa. ⚠️ **Resta UN compito e UN dispaccio:** il disegno dimostra in §2.2 che `E30` senza `E21` rende **rossa** una sonda esistente, quindi separarli in due compiti consegnerebbe un rosso |
| **D2** | i tre `.stderr` della porta `process` e i nove di `Parameters` si rileggono **uno per uno**, e la corsa che li produce si lancia **senza** `TRYBUILD=overwrite` | vincolo 10 della §11 e gotcha **#25**: un `.stderr` rigenerato in blocco è un oracolo che si è riscritto da solo per tornare verde |
| **D3** | ⛔ **`ArbiterId` NON è `pub`-costruibile da un letterale di tupla:** campo privato più `ArbiterId::new(u64)` pubblico | è un **parametro consegnato** (ADR-0034), quindi `daemon` deve poterlo costruire; ma la forma `ArbiterId(0)` da qualunque crate rifarebbe il difetto che **AUD-050** ha misurato su `RecordV1` — una guardia vale quanto il suo costruttore |
| **D4** | l'ordine dei compiti è **quello della §1.4 del disegno**, e il **3bis** resta prima del **4** | scrivere lo schema `ipc` in `bincode` **è** la decisione C-1 presa per omissione (§3.5). Invertirli la prende senza accorgersene |
| **D5** | ogni compito **rimisura la propria baseline** con `cargo test --locked --workspace --no-fail-fast` e non cita quella scritta qui | una baseline citata invecchia a ogni compito — gotcha **#31**. Quella di partenza sta scritta **una volta sola**, qui sotto |
| **D6** | ⛔ **il compito 6 congela un QUARTO record**, non solo una variante | P-2: senza, il nuovo indice di filo è tenuto da nulla, e il compilatore non lo dice |
| **D7** | le mutazioni si provano **una alla volta**, si compila in un passo **separato** dall'eseguire, e si revoca **ripristinando da una copia presa prima** | gotcha **#48**, la trappola più frequente del progetto: una revoca che deve *cercare* può fallire e lasciare il file mutato — successo al Task 8 del Traguardo 5, sette misure buttate |

**La baseline di partenza, misurata il 2026-08-30 e da NON citare nei compiti:**
`bash scripts/gate.sh` → `GATE GREEN` · `cargo test --locked --workspace --no-fail-fast` →
**37 bersagli, 267 passate, 0 fallite, 2 ignorate**.

---

## La mappa dei file

⛔ **Nessun file di `platform`.** Il trasporto vero è scaglionato: è la voce 5, chiusa
dichiarandolo.

| File | Chi lo tocca | Responsabilità |
|---|---|---|
| `crates/kernel/src/arbiter/mod.rs` | compito 1 | `ArbiterId`, `Released`, `release` |
| `crates/kernel/src/parameters.rs` | compito 1 | il terzo campo consegnato |
| `crates/kernel/src/ports/process.rs` | compiti 1, 3 | `Started`, `Killed`, e il `Frame` che diventa codificato |
| `crates/kernel/src/framing.rs` | **creato** dal compito 3 | l'inquadratura: lunghezza dichiarata a larghezza fissa, condivisa dai due canali privati |
| `crates/kernel/src/wire/mod.rs` | **creato** dal compito 4 | lo schema `ipc`: l'enumerazione dei messaggi |
| `crates/kernel/src/sensor.rs` | **creato** dal compito 5 | il contratto del sensore di ADR-0009 |
| `crates/kernel/src/gateway/mod.rs` | **creato** dal compito 6 | il decisore, il filtro dei vincoli e il gettone di conformità |
| `crates/kernel/src/permission.rs` | **creato** dal compito 7 | la tripla, e la proiezione dal giornale |
| `crates/kernel/src/degradation.rs` | **creato** dal compito 8 | lo stato di degrado, **ricalcolato** e non cacheato |
| `crates/kernel/src/record.rs` | compito 6 | la variante `RecordKind` del record di routing |
| `crates/kernel/tests/frozen/` | compito 6 | il **quarto** record congelato (D6) |
| `crates/simulator/src/ipc.rs` | **creato** dal compito 9 | la finta gui guidata dal seme, sul precedente di `CrashingJournal` |
| `crates/daemon/src/main.rs` | compiti 1, 6, 7, 8 | i default letterali dei parametri nuovi |

📌 **Perché file separati e non un modulo solo:** il progetto già lo fa così — `arbiter` è
una cartella con tre file dal Task 8 del Traguardo 5, e le porte sono un file per famiglia.
Un file per responsabilità è la convenzione, non una scelta di questo piano.

---

## Parte A — la concessione che torna

### Compito 1: `E30` + `R6` + `E21` — l'identità dell'arbitro, `Released`, `Started` e `Killed`

**Files:**
- Modify: `crates/kernel/src/arbiter/mod.rs` — `ArbiterId`, `Released`, `release`
- Modify: `crates/kernel/src/parameters.rs` — il terzo campo
- Modify: `crates/kernel/src/ports/process.rs` — `Started`, `Killed`, le due firme
- Modify: `crates/daemon/src/main.rs` — i default letterali
- Modify: i quarantadue siti di `Parameters::new` (comando in P-4)
- Test: `crates/kernel/tests/arbiter_admission.rs`, `crates/kernel/tests/worker_tokens.rs`,
  `crates/kernel/tests/ports_are_implementable.rs`,
  `crates/simulator/tests/arbiter_campaign.rs`
- Test: i nove `.stderr` di `crates/kernel/tests/compile_fail/` (elenco in P-4)

⛔ **Leggi P-4 prima di cominciare.** Il costo vero di questo compito sta lì, non nella §2.4
del disegno.

#### Commit 1a — l'identità (`E21`)

- [ ] **Passo 1: scrivi la sonda che fallisce**

In `crates/kernel/tests/parameters_delivered.rs`, in fondo:

```rust
#[test]
fn the_arbiter_identity_is_delivered_and_not_invented() {
    // ⛔ THE POINT IS THE ABSENCE OF A DEFAULT. §6.1.3 forbids the kernel to MINT an
    // identifier, and ADR-0034 says a decision reads only what it was handed: an arbiter
    // that chose its own id would be doing both.
    let parameters = Parameters::new(64, Mib(8192), ArbiterId::new(7));
    assert_eq!(parameters.arbiter_id(), ArbiterId::new(7));
}
```

- [ ] **Passo 2: lancia la sonda e verifica che NON COMPILI**

```bash
cargo test --locked -p kernel --test parameters_delivered 2>&1 | head -20
```

Atteso: `error[E0433]` o `error[E0412]` su `ArbiterId` — il tipo non esiste. ⛔ **Se compila,
fermati e scrivi una voce d'errata:** significa che qualcuno l'ha già costruito, ed è il
gotcha **#49**.

- [ ] **Passo 3: scrivi `ArbiterId` in `crates/kernel/src/arbiter/mod.rs`**

Accanto a `GrantId`, e **sopra** `Grant`:

```rust
/// The identity of one arbiter, DELIVERED and never minted.
///
/// ⛔ IT EXISTS FOR ONE QUESTION: `release` must be able to tell "a grant I issued and have
/// already swept" from "a grant of ANOTHER arbiter". Without it `held.remove` answers `None`
/// to both, and the two need opposite answers -- see `Released` and `ReleaseError`.
///
/// ⛔ DELIVERED, per ADR-0034: it travels in `Parameters`, and §6.1.3 forbids the kernel to
/// mint an identifier. Nothing here generates one.
///
/// ⚠️ THE FIELD IS PRIVATE AND THE CONSTRUCTOR IS NOT, and the asymmetry is the whole of it:
/// `daemon` has to build one, so a tuple literal from any crate would give the same forgery
/// `RecordV1` gave in AUD-050 -- a guard is worth exactly what its constructor is worth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArbiterId(u64);

impl ArbiterId {
    /// The identity in. It is a value the composition root hands down, not a choice.
    pub const fn new(value: u64) -> Self {
        ArbiterId(value)
    }
}
```

- [ ] **Passo 4: aggiungi il campo a `Parameters`**

In `crates/kernel/src/parameters.rs`, il campo e il suo lettore, **positionally last**:

```rust
pub struct Parameters {
    executor_turn_limit: u64,
    total_vram: Mib,
    arbiter_id: ArbiterId,
}
```

```rust
    pub const fn new(executor_turn_limit: u64, total_vram: Mib, arbiter_id: ArbiterId) -> Self {
        Parameters {
            executor_turn_limit,
            total_vram,
            arbiter_id,
        }
    }

    /// Which arbiter these parameters belong to.
    ///
    /// ⛔ DELIVERED, never invented: the kernel has no way to produce one, and §6.1.3 says
    /// it must not. Two arbiters built from the same value ARE the same arbiter as far as
    /// `release` is concerned, and that is the caller's statement to make, not ours.
    pub const fn arbiter_id(self) -> ArbiterId {
        self.arbiter_id
    }
```

⚠️ **Aggiungi l'`use` di `ArbiterId`** in `parameters.rs`; il modulo `arbiter` lo esporta già.

- [ ] **Passo 5: fai compilare i quarantadue siti**

⛔ **Uno per uno, e NON con una sostituzione globale.** I nove casi `compile_fail` vanno
guardati in faccia: sette di essi cadrebbero per **arità** prima di arrivare all'errore che
asseriscono, che è un `mismatch` e non il loro oracolo.

```bash
cargo build --locked --workspace --tests 2>&1 | grep -E "^error" | head -50
```

Per i siti di prova il valore è indifferente e si usa `ArbiterId::new(1)`; in
`crates/daemon/src/main.rs` è un **default letterale**, come `executor_turn_limit` e
`total_vram` già sono (vincolo 11 della §11).

- [ ] **Passo 6: rileggi i nove `.stderr`, uno per uno**

```bash
cargo test --locked -p kernel --test compile_fail 2>&1 | tail -40
```

⛔ **Mai `TRYBUILD=overwrite`** (D2). Per ogni caso che dà `mismatch`, apri il `.stderr`,
leggi che cosa è cambiato, e correggi **a mano** solo ciò che è cambiato davvero. Se il testo
atteso non cambia ma il **numero di riga** sì, è perché hai spostato righe nel `.rs`: è la
stessa trappola che il rimedio di **AUD-042/045** ha misurato — tre righe di commento
spostarono il difetto dalla riga 34 alla 37 e resero `mismatch` l'oracolo che il paragrafo
esisteva per proteggere.

- [ ] **Passo 7: lancia la sonda e verifica che passi**

```bash
cargo test --locked -p kernel --test parameters_delivered 2>&1 | tail -5
```

Atteso: `test result: ok.` con **una** sonda in più di prima.

- [ ] **Passo 8: il cancello, e la baseline rimisurata**

```bash
bash scripts/gate.sh
```

Atteso: `GATE GREEN`.

```bash
cargo test --locked --workspace --no-fail-fast 2>&1 | grep -E "^(running|test result:)" | awk '/^running/{t++} /^test result:/{p+=$4; f+=$6; i+=$8} END{print "bersagli="t"  passate="p"  fallite="f"  ignorate="i}'
```

- [ ] **Passo 9: commit**

```bash
git add -A && git commit -m "traguardo 6 (compito 1a): l'arbitro guadagna un'identita' CONSEGNATA -- senza, release non sa distinguere la propria concessione gia' spazzata da quella di un altro"
```

#### Commit 1b — `release` risponde tre cose (`E30`)

- [ ] **Passo 1: scrivi le due sonde che falliscono**

In `crates/kernel/tests/arbiter_admission.rs`, in fondo. ⛔ **Sono due perché le vie sono
due e falliscono diversamente** — una direzione tenuta da una mutazione è tenuta da niente
(gotcha **#72**):

```rust
#[test]
fn a_grant_of_this_arbiter_released_after_its_window_is_not_an_error() {
    // ⛔ THIS IS THE DECISION OF 2026-08-28: release NEVER answers `Err` to a grant of its
    // own. An expired window is not a failure of the release -- the sweep simply got there
    // first -- and the caller learns that from `AlreadyCollected`, not from an error.
    let mut arbiter = arbiter(ArbiterId::new(1), TOTAL);
    let Admission::Granted(grant) = arbiter.admit(
        &profile("short-lived", 4_096, ComputeClass::Batch),
        Millis::new(5_000),
        Monotonic::ORIGIN,
    ) else {
        panic!("4096 of 16384 fits");
    };

    let released = arbiter.release(grant, Monotonic::from_millis(5_001));

    assert_eq!(released, Ok(Released::AlreadyCollected));
}

/// The counter-probe, and it is the direction that is skipped: inside the window the release
/// says WHAT CAME BACK. Without it, "always answer AlreadyCollected" stays green.
#[test]
fn a_grant_released_inside_its_window_reports_what_came_back() {
    let mut arbiter = arbiter(ArbiterId::new(1), TOTAL);
    let Admission::Granted(grant) = arbiter.admit(
        &profile("short-lived", 4_096, ComputeClass::Batch),
        Millis::new(5_000),
        Monotonic::ORIGIN,
    ) else {
        panic!("4096 of 16384 fits");
    };

    let released = arbiter.release(grant, Monotonic::from_millis(4_999));

    assert_eq!(released, Ok(Released::Now(Mib::new(4_096))));
}
```

⛔ **L'aiutante `arbiter` di quel banco guadagna un primo argomento, ed è il punto del passo
5.** Oggi è `fn arbiter(total: Mib) -> Arbiter` e costruisce
`Parameters::new(TURN_LIMIT, total)`; diventa `fn arbiter(id: ArbiterId, total: Mib) -> Arbiter`.
⚠️ **Se gli dai un identificativo fisso dentro l'aiutante invece che come argomento**, la sonda
`a_grant_released_on_the_wrong_arbiter_…` costruisce **due arbitri con la stessa identità** e
smette di provare ciò che il suo nome dice — verde, e vuota. È il difetto che il passo 5 di
questo commit esiste per cogliere.

⚠️ **Gli altri nomi sono quelli del banco, verificati:** `profile(name, vram, lane)` a **tre**
argomenti, le costanti `TURN_LIMIT`, `TOTAL` e `LONG`, e gli idiomi `Mib::new`, `Millis::new`,
`Monotonic::ORIGIN`, `Monotonic::from_millis`. **Non inventarne di nuovi.**

- [ ] **Passo 2: lancia e verifica che NON COMPILI**

```bash
cargo test --locked -p kernel --test arbiter_admission 2>&1 | head -20
```

Atteso: `error[E0433]` su `Released` — il tipo non esiste.

- [ ] **Passo 3: scrivi `Released` e cambia `release`**

In `crates/kernel/src/arbiter/mod.rs`, accanto a `ReleaseError`:

```rust
/// What handing a grant back actually did.
///
/// ⛔ TWO ANSWERS AND NOT A `bool`, because the caller has something to do with the
/// difference: `Now` says this many MiB came back to the budget in this call, and
/// `AlreadyCollected` says the sweep had already taken them -- the books are the same either
/// way, and only the first is a change the caller caused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Released {
    /// Taken back now. The `Mib` is what returned to the budget.
    Now(Mib),
    /// The sweep had it already -- the declared window closed, or the grace of a revocation
    /// ran out. NOT an error: the reservation is not held by anyone either way.
    AlreadyCollected,
}
```

E la funzione, che ora **consulta l'identità**:

```rust
    /// Hands a grant back.
    ///
    /// ⛔ DECISION OF 2026-08-28: a grant THIS arbiter issued is never an `Err`. Its window
    /// may have closed and its grace may have run out; in both cases the sweep took the
    /// reservation back and the answer is `AlreadyCollected`. Only the grant this arbiter
    /// NEVER ISSUED is a caller defect, and that one stays an error.
    ///
    /// ⚠️ `UnknownGrant` NOW MEANS ONE THING, where it used to mean three. The two causes
    /// that left it are the two above; what remains is a grant minted by another arbiter,
    /// which `Parameters::arbiter_id` is what lets us see.
    pub fn release(&mut self, grant: Grant, now: Monotonic) -> Result<Released, ReleaseError> {
        if grant.issuer != self.id {
            return Err(ReleaseError::UnknownGrant);
        }
        self.collect_expired(now);
        match self.held.remove(&grant.id) {
            Some(held) => Ok(Released::Now(held.reserved)),
            None => Ok(Released::AlreadyCollected),
        }
    }
```

⛔ **`Grant` guadagna il campo `issuer: ArbiterId`**, scritto da `issue` e da nessun altro; e
`Arbiter` guadagna `id: ArbiterId`, letto da `Parameters` nel proprio `new`. Il campo di
`Grant` resta **privato**, come `id`.

- [ ] **Passo 4: lancia le due sonde e verifica che passino**

```bash
cargo test --locked -p kernel --test arbiter_admission 2>&1 | tail -5
```

- [ ] **Passo 5: verifica che la sonda del Traguardo 5 sia ANCORA VERDE, e non cancellata**

⛔ **È la condizione 5 della Definizione di «fatto», e il disegno la nomina per esteso:**
chiuderla cancellandola sarebbe *«cancellare una sonda per prendere una decisione»*, il gotcha
**#73** al contrario.

```bash
cargo test --locked -p kernel --test arbiter_admission a_grant_released_on_the_wrong_arbiter_is_an_error_and_not_a_silent_credit -- --exact 2>&1 | tail -5
```

Atteso: `1 passed`. ⚠️ **Quella sonda costruisce due arbitri:** ora devono ricevere **due
`ArbiterId` diversi**, o non prova più niente. Se il banco gliene dà uno solo, correggilo e
scrivi una voce d'errata — è il difetto che questo passo esiste per cogliere.

- [ ] **Passo 6: la mutazione di controllo, e sono DUE**

⛔ **D7: una alla volta, compilando in un passo separato dall'eseguire, revocando da copia.**

| Mutazione | Deve uccidere |
|---|---|
| `if grant.issuer != self.id` → `if false` | `a_grant_released_on_the_wrong_arbiter_is_an_error_and_not_a_silent_credit` |
| `Ok(Released::AlreadyCollected)` → `Err(ReleaseError::UnknownGrant)` | `a_grant_of_this_arbiter_released_after_its_window_is_not_an_error` |

```bash
cp crates/kernel/src/arbiter/mod.rs "$SCRATCH/mod.rs.orig"
# muta, poi:
cargo build --locked -p kernel --tests
cargo test --locked -p kernel --test arbiter_admission 2>&1 | tail -5
cp "$SCRATCH/mod.rs.orig" crates/kernel/src/arbiter/mod.rs
git diff --stat crates/kernel/src/arbiter/mod.rs   # deve essere VUOTO
```

- [ ] **Passo 7: il cancello**

```bash
bash scripts/gate.sh
```

- [ ] **Passo 8: commit**

```bash
git add -A && git commit -m "traguardo 6 (compito 1b): release non risponde piu' Err a una concessione PROPRIA -- UnknownGrant significa ora una cosa sola, e le altre due cause diventano AlreadyCollected"
```

#### Commit 1c — la porta `process` restituisce la concessione (`R6`)

- [ ] **Passo 1: scrivi le due sonde che falliscono**

In `crates/kernel/tests/worker_tokens.rs`. ⛔ **Due, e la seconda è quella che il disegno
chiama *«la metà che mancava»*** — l'avvio **fallito**, che non era discusso da nessuna parte:

⛔ **`a_real_grant()` costruisce l'arbitro e lo BUTTA, quindi non serve a queste due sonde:**
loro devono **rilasciare** sull'arbitro che ha emesso. Aggiungi accanto a esso un aiutante che
restituisce entrambi, e **non** cambiare `a_real_grant`, che le altre quattro sonde usano:

```rust
/// The arbiter AND the grant it issued. ⛔ `a_real_grant` throws the arbiter away, which is
/// right for the probes about token SHAPE; these two are about the reservation coming home,
/// so they need the books that hold it.
fn an_arbiter_and_a_real_grant() -> (Arbiter, Grant) {
    let mut arbiter = Arbiter::new(
        Parameters::new(10_000, Mib::new(16_384), ArbiterId::new(1)),
        VramPolicy::Remote(RemotePolicy),
    );
    let Admission::Granted(grant) = arbiter.admit(
        &ResourceProfile {
            name: "asr-realtime",
            reserved_vram: Mib::new(1_024),
            compute_class: ComputeClass::Realtime,
            preemption: Preemption::Never,
        },
        Millis::new(1_000_000),
        Monotonic::ORIGIN,
    ) else {
        panic!("1024 of 16384 fits");
    };
    (arbiter, grant)
}

#[test]
fn a_worker_that_is_killed_gives_the_grant_back() {
    let (mut arbiter, grant) = an_arbiter_and_a_real_grant();
    let Started::Running(worker) =
        FakeProcess.start(grant, WorkerDescriptor::new(b"asr.exe".to_vec()))
    else {
        panic!("the fake starts every worker it is asked for");
    };

    let killed = worker.kill();

    // ⛔ THE GRANT IS OUTSIDE EVERY `Result`, and this assertion is why: the reservation is a
    // fact of the BOOKS, not of the worker's health. `kill` is always lawful (§5.3 point 4),
    // so a worker that died badly still owes its reservation back.
    assert!(killed.outcome.is_ok());
    assert_eq!(
        arbiter.release(killed.grant, Monotonic::ORIGIN),
        Ok(Released::Now(Mib::new(1_024)))
    );
}

#[test]
fn a_start_that_fails_gives_the_grant_back_by_name() {
    // ⛔ THIS VIA WAS NOT DISCUSSED ANYWHERE before the milestone 6 design measured it: today
    // `start` takes the grant BY VALUE and drops it on `Err`, and nothing can rebuild it --
    // `GrantId` is private and `grant_has_no_constructor.rs` pins that. The reservation then
    // sat in the books for the whole declared window, and only the sweep got it back.
    let (mut arbiter, grant) = an_arbiter_and_a_real_grant();

    let Started::Rejected { grant, error } =
        FailingProcess.start(grant, WorkerDescriptor::new(b"asr.exe".to_vec()))
    else {
        panic!("FailingProcess refuses every start, so this must be the rejected arm");
    };

    assert_eq!(error, ProcessError::StartFailed);
    assert_eq!(
        arbiter.release(grant, Monotonic::ORIGIN),
        Ok(Released::Now(Mib::new(1_024)))
    );
}
```

⚠️ **`FakeProcess` e `FailingProcess` sono `struct` unitarie** e si usano **senza `::new()`** —
`FakeProcess.start(…)`. `FailingProcess` e la sonda `a_spawn_that_does_not_happen_is_start_failed`
nascono dal rimedio di **AUD-051**: quella sonda asserisce `outcome.err()`, quindi **cambia con
la firma** e va riscritta sul ramo `Started::Rejected` — è un lavoro del compito, non una
sorpresa.

- [ ] **Passo 2: lancia e verifica che NON COMPILI**

```bash
cargo test --locked -p kernel --test worker_tokens 2>&1 | head -20
```

Atteso: `error[E0433]` su `Started`.

- [ ] **Passo 3: scrivi `Started` e `Killed`, e cambia le due firme**

In `crates/kernel/src/ports/process.rs`:

```rust
/// What starting a worker did. ⛔ NOT a `Result`, and the shape is `Admission`'s.
///
/// ⛔ THE REJECTED ARM CARRIES THE GRANT BACK BY NAME. `start` consumes it, so before this
/// type a failed start dropped a reservation nobody could rebuild -- `GrantId` is private and
/// `tests/compile_fail/grant_has_no_constructor.rs` pins it -- and the books held it for the
/// whole declared window. The sweep was the only way back.
///
/// ⚠️ WHY NOT `Result<H, (Grant, ProcessError)>`: no error in this repository carries the
/// value it consumed, measured with
/// `grep -rnE "Result<[^,]+, *\([A-Z]" crates/ --include=*.rs`, which returns nothing. The
/// shape this project uses for "several outcomes, each carrying what belongs to it" is
/// `Admission`. A second idiom would be a second way to say one thing.
#[must_use]
pub enum Started<H> {
    /// The worker is alive, and the grant is now its.
    Running(H),
    /// It never started. The grant comes back, and so does the reason.
    Rejected { grant: Grant, error: ProcessError },
}

/// What killing a worker did.
///
/// ⛔ A STRUCT AND NOT AN ENUM, because there are not two states: there is ONE state with two
/// facts. The grant comes back whatever happened, and `outcome` says whether the kill itself
/// went cleanly.
///
/// ⛔ THE GRANT SITS OUTSIDE EVERY `Result`, and that is the teaching part: it comes back even
/// on the arm where the worker died badly. `kill` is ALWAYS LAWFUL (§5.3 point 4), and a
/// reservation is a fact of the books, not of the process's health.
#[must_use]
pub struct Killed {
    /// The reservation, back to whoever will hand it to the arbiter.
    pub grant: Grant,
    /// Whether the kill itself succeeded.
    pub outcome: Result<(), ProcessError>,
}
```

E le due firme, dentro i tratti:

```rust
    /// Kills the worker, and it is ALWAYS lawful (§5.3, point 4).
    ///
    /// ⛔ CONSUMES the `Worker`: instructing it after the kill does not compile.
    /// ⛔ AND IT RETURNS THE GRANT, outside the `Result` -- see `Killed`.
    fn kill(self) -> Killed;
```

```rust
    /// Starts a worker.
    ///
    /// ⛔ Takes the GRANT as an argument: whoever writes "start the worker" without one
    /// does not compile. This is the half of I2 that belongs to the compiler; the other
    /// half -- that `process` is the only port towards processes -- rests on a level 2
    /// check and is therefore deletable. Declared, not hidden (§5.6).
    ///
    /// ⛔ IT RETURNS `Started` AND NOT A `Result`, so the grant of a failed start has a way
    /// home -- see `Started::Rejected`.
    fn start(&mut self, grant: Grant, descriptor: WorkerDescriptor) -> Started<Self::Handle>;
```

⛔ **`Grant` deve essere raggiungibile da `ports::process`** per nome, e non ri-esportato: il
piano del Traguardo 5 decise *«`Grant` non è ri-esportato da `ports::process`»*. Usa il
percorso pieno `crate::arbiter::Grant`.

- [ ] **Passo 4: aggiorna le finte dei banchi**

Sono **sei**, in tre file, e il comando che le trova è quello di P-4 ristretto a `impl`:

```bash
grep -rn "impl Worker for\|impl Process for" crates/ --include=*.rs
```

`FakeWorker` e `FakeProcess` e `FailingProcess` in `worker_tokens.rs`; `ScriptedWorker` e
`SpawningProcess` in `ports_are_implementable.rs`; più le tre finte dentro i casi
`compile_fail`. ⛔ **`SpawningProcess::start` non deve diventare infallibile per comodità:** il
suo doc dichiara che cosa compra, e cambiarlo in silenzio è la specie di difetto che
**AUD-054** ha misurato. ⛔ **E `a_spawn_that_does_not_happen_is_start_failed` va RISCRITTA, non
cancellata:** asserisce `outcome.err()`, che con `Started` non esiste più; la forma nuova
smonta `Started::Rejected { error, .. }` e tiene l'**uguaglianza** sulla variante, che è ciò
che il suo doc dichiara di comprare contro un `is_err()`.

- [ ] **Passo 5: rileggi i tre `.stderr` della porta, uno per uno**

```bash
cargo test --locked -p kernel --test compile_fail 2>&1 | tail -40
```

⛔ **Mai in blocco** (D2). I tre sono `instructing_after_the_kill`, `reading_without_a_receipt`
e `reading_twice_from_one_receipt`; il primo asserisce `E0382` **sul `Worker` mosso da
`kill`**, e `kill` ora restituisce `Killed` invece di `Result<(), _>`: verifica che l'errore
sia ancora quello, e **non** che il caso semplicemente fallisca.

- [ ] **Passo 6: le due sonde passano**

```bash
cargo test --locked -p kernel --test worker_tokens 2>&1 | tail -5
```

- [ ] **Passo 7: la mutazione di controllo**

| Mutazione | Deve uccidere |
|---|---|
| in `Started::Rejected`, non restituire il `grant` ma un `Grant` di un secondo arbitro | `a_start_that_fails_gives_the_grant_back_by_name` — l'asserzione su `release` diventa `Err(UnknownGrant)` |

⚠️ **Se questa mutazione NON uccide**, la sonda sta provando l'arità e non l'identità: è la
stessa distinzione che il registro dichiara per `SingleReceipt::new`.

- [ ] **Passo 8: il cancello e la baseline**

```bash
bash scripts/gate.sh
```

- [ ] **Passo 9: commit**

```bash
git add -A && git commit -m "traguardo 6 (compito 1c): start e kill restituiscono la concessione -- e la via dell'avvio FALLITO non era discussa da nessuna parte"
```

#### Criterio di chiusura del compito 1

- [ ] `GATE GREEN` a tutti e tre i commit
- [ ] `a_grant_released_on_the_wrong_arbiter_is_an_error_and_not_a_silent_credit` **verde e
      non cancellata**, coi due arbitri che ricevono due `ArbiterId` diversi
- [ ] i nove `.stderr` riletti **uno per uno**, nessuno rigenerato in blocco
- [ ] `ReleaseError` ha **ancora una sola variante**
- [ ] le voci `E30`, `R6` ed `E21` della tabella unica di
      [`porta-di-qualita.md`](../../porta-di-qualita.md) sono marcate chiuse, col commit
