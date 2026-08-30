# Traguardo 6 — gli altri meccanismi: il disegno

✅ **QUESTO DISEGNO È COMPLETO DAL 2026-08-30.** Tutte le sezioni sono **approvate** dal
proprietario, una per volta, e la **§7** fissa la Definizione di «fatto» e chi la verifica. Chi
riprende ha un disegno intero da tradurre in piano.

⚠️ **RICHIAMO DEL 2026-08-30:** questo riquadro diceva *«QUESTO DISEGNO È IN CORSO»*, e prima
ancora *«1–5»*, *«le 6 e 7»* e *«due da aprire»*. ⛔ **Il conteggio delle sezioni è TOLTO e non
portato a sette:** a disegno completo non c'è più niente da contare, e la §6 del
[compendio](../../COMPENDIO.md) vi **rimanda** invece di tenerne una seconda copia (gotcha
**#68**).

⛔ **E «completo» NON significa «si passa al piano»:** quella condizione è scritta altrove e non
è di questo documento — vedi la §7.4 e le voci che questo disegno apre.
✅ **RICHIAMO DEL 2026-08-30 — e la condizione è SODDISFATTA:** l'ultima voce che sbarrava, la
**5**, è chiusa lo stesso giorno; la frase qui sopra resta perché dice **dove** vive la
condizione, non che sia aperta. ⛔ **E chiudendola si è misurato che la §1.2 sbagliava la
RAGIONE del perimetro, non il perimetro:** il richiamo sta lì, e si legge **prima** della §1.2.

⚠️ **Non è una spec.** Come i disegni dei Traguardi 4 e 5, fissa il **perimetro**, le **forme**
che la §6 della spec descrive a parole, e per ogni artefatto **il controllo che lo esercita**.
La spec resta l'autorità; dove questo disegno diverge, lo **dichiara**.

📌 **Metodo.** Ogni decisione qui è stata presa leggendo il **codice di adesso**, non la spec
che lo descrive. Dove un'affermazione è una **deduzione** e non una misura, è scritto.

---

## 1. Perimetro e ordine — ✅ approvata

### 1.1 Il traguardo NON si spacca

Le sei parti della §6 sono in gran parte indipendenti fra loro, e questo **non** è un criterio
di spacco in questo repository. Il precedente è il **Traguardo 2**, che in un traguardo solo ha
portato i due tempi, `Rng`, i parametri consegnati, `Reactor`, l'esecutore, l'orologio virtuale,
il reattore reale con la conformità, il cablaggio di produzione, il confine dei tipi e le porte
`filesystem`, `network`, `process`, `ipc`. Sono in `crates/kernel/src/` adesso: `filesystem` non
ha nulla a che vedere con `rng`, ed erano più eterogenei di questi.

L'indipendenza fra i pezzi è quindi un criterio di **ordine dei compiti dentro il piano**, non
di spacco. Spaccare introdurrebbe un settimo traguardo in quattro tabelle — `roadmap.md`,
`README.md`, `COMPENDIO.md`, `tracciabilita.md` — cioè la classe di difetto del gotcha **#68**,
per comprare qualcosa che il precedente dice non serva.

### 1.2 Il perimetro, misurato contro il codice

| § | Cosa | Stato **misurato** il 2026-08-28 |
|---|---|---|
| 6.1 | porta `ipc` e schema | porta **c'è** (T2). Schema e **timbro di build** no |
| 6.2 | decisore del gateway | **niente** |
| 6.3 | gettone di conformità | **niente**; gli altri tre gettoni esistono |
| 6.4 | contratto del sensore | **niente** |
| 6.5 | confine dei tipi | ⛔ **GIÀ ESEGUITO** — `Untrusted::promote` riceve `journal: &mut J`, dal Traguardo 3 Task 7 |
| 6.6 | permesso come tripla | **niente** |
| 6.7 | stato di degrado | **niente** |
| 6.10 | canale worker | firme e quattro casi `compile_fail` **ci sono** (T5 Task 11); manca **la metà che codifica** |

⛔ **RICHIAMO DEL 2026-08-29 — QUESTA TABELLA PREZZAVA DUE RIGHE MENO DI QUANTO LA SPEC
CHIEDA, e la divergenza ha DUE case e non una.** La §7.4.6 della spec mette in colonna
*«Implementazione reale in questo sotto-progetto»* un ✅ sia per `ipc` (*«named pipe»*) sia per
`process` (*«avvio, dialogo e uccisione veri»*), e la §0.4 riga §1 tiene dentro l'*«IPC lato
core»* lasciando fuori **solo** il processo `gui`. Nel codice non c'è né l'una né l'altro:

```
grep -rn "impl Ipc" crates/ --include=*.rs ; ls crates/platform/src/
```

⛔ **RICHIAMO DEL 2026-08-30 — LA CONCLUSIONE REGGE, L'ARGOMENTO NO, ed era proprio quello che
questa sezione dichiarava di aver misurato.** Qui stava: *«un trasporto vero pretende la metà di
**prontezza** della porta `reactor`, che non ha un produttore … costruire la pipe oggi
significherebbe congelare quella forma»*, con accanto la clausola onesta che per `process` fosse
una **deduzione** da *«va misurata prima di finire in un piano»*. ✅ **Misurata: è falsa per
entrambe.** Le due porte sono **a interrogazione** per costruzione, e lo dicono di sé:

| Porta | Firma | Ciò che il sorgente dichiara |
|---|---|---|
| `ipc` | `accept(&mut self) -> Option<ClientId>` | la finta: *«`accept` NEVER BLOCKS, so "nobody is waiting" has to be an ordinary answer rather than a wait»* |
| `ipc` | `receive(..) -> Result<Option<Vec<u8>>, IpcError>` | *«`Ok(None)` IS NOT AN ERROR … or the core could not **poll** this port at all»* |
| `process` | `read_next(..) -> Result<Option<Frame>, ProcessError>` | stessa forma |

⛔ **La parola *«poll»* è della porta, non di chi la prezza:** un trasporto che interroga non ha
bisogno di nessuna sveglia, quindi la prontezza mancante **non è ciò che tiene fuori il
trasporto**. ✅ **Ciò che lo tiene fuori era già scritto in due posti, ed è più solido:** la
**§0.2** — *«nessuna interfaccia grafica»* (sotto-progetto 2, [ADR-0029](../../adr/0029-guscio-della-gui.md)
è `Proposed`) e *«nessun worker Python»* — e la **§0.4 riga §1**, che fa entrare lo **schema**
IPC lato core e scaglia il **processo `gui`**. La §7.4.6 lo scriveva già nella **propria terza
colonna**: *«non esistono worker da avviare (§0.2)»*, *«non esiste una GUI dall'altro capo»*.

📌 **Che i fatti su `reactor` restino veri è la parte da non confondere:** la prontezza **non ha
davvero** un produttore, e `WaitOutcome { DeadlineReached, EventReady }` è stato **tolto** per
quella ragione. Sono veri e non sostengono questa conclusione — che è la forma esatta del
**#65**: *un rapporto è un piano, e si prezza leggendo il codice*, qui applicato al richiamo di un
disegno. Chi lo scrisse aveva letto una **guardia** (`reactor.rs`) e non le **due porte** che
stava prezzando.

✅ **LA VOCE 5 È CHIUSA IL 2026-08-30, e la correzione è più PICCOLA di come questa sezione la
prezzava** — gotcha **#65** nella direzione che costa di più. Le due celle passano a **❌
scaglionata**, che è la parola che quella colonna **già usa** per `filesystem` e `network`:
nessuno stato nuovo, e l'innesco è la **sezione che scaglia** invece di una condizione da
inventare. ⛔ **L'innesco che questa sezione proponeva — *«la prima sorgente di eventi esterni
sulla porta `reactor`»* — NON è stato scritto:** la §8.1 lo pretende obbligatorio, quindi in una
sezione normativa sarebbe stato difeso da nulla e **sbagliato**, cioè peggio del ✅ che
sostituisce. Il richiamo datato vive nella §7.4.6 della spec, che ne è la casa.

I comandi che rifanno la misura, invece delle cifre:

```
grep -rniE "trait Sensor|struct Verdict|enum Verdict" crates/ --include=*.rs
grep -rn "BuildStamp" crates/
grep -n "pub fn promote" -A6 crates/kernel/src/boundary.rs
```

⚠️ **I riscontri di `Permission` e `degrad` che un `grep` ingenuo restituisce sono prosa nei
commenti**, guardati uno per uno — non tipi. È il gotcha **#70**.

### 1.3 Le regole di scaglionamento, per riga

Ogni riga porta la propria regola di §0.3, come la §0.4 pretende.

| § | Regola | Cosa resta fuori |
|---|---|---|
| 6.1 | **A** — §0.4 riga §1: senza confine di processo non c'è nulla da simulare | il processo `gui` |
| 6.2 | **A** — Q13 è una proprietà su *qualunque* catena | gli adattatori dei provider reali (**C**) |
| 6.3 | **A** — è il meccanismo che rende Q13 una proprietà | — |
| 6.4 | **A** — Q10 si verifica con un doppio | i sensori reali (**C**), RK-5 |
| 6.6 | **B** — forma e registrazione | mediatore, preset, ciclo MCP, canary (**C**) |
| 6.7 | **A** — Q18 è DST: lo stato va dichiarato *prima* del primo fallimento | proiezione trace, OTLP (**C**) |
| 6.10 | vincolo **15** della §11, l'unico ancora aperto per metà | la conformità contro un worker vero (§7.4.6) |

### 1.4 L'ordine dei compiti, e cosa impone ciascuna posizione

| | Compito | Cosa lo mette lì |
|---|---|---|
| 1 | **`E30` + `R6` + `E21` insieme** | sono **la stessa riga di codice**, e `E21` è la precondizione di `E30` — §2 |
| 2 | ~~il timbro di build~~ | ⛔ **uscito**: diventa una non-costruzione dichiarata — §3.4 |
| 3 | **§6.10, la metà che codifica** | chiude il vincolo 15 |
| 3bis | **la misura C-1 e la decisione su §6.1.1** | ⬆️ **prima che una riga di schema esista** — §3.5 |
| 4 | **§6.1, lo schema `ipc`** | nel formato che 3bis ha deciso; identificativi **progressivi del giornale**, mai generati (§6.1.3) |
| 5 | **§6.4, il contratto del sensore** | §6.2 ne ha bisogno: *«schema non conforme = verdetto di sensore»* |
| 6 | **§6.2 + §6.3, decisore e gettone** | il gettone lo **emette il filtro dei vincoli**, che è dentro il decisore |
| 7 | **§6.6, il permesso** | indipendente, ma §6.7 lo consuma |
| 8 | **§6.7, il degrado** | **derivato**: arbitro (c'è), salute dei provider (6.2), permessi (6.6) |
| 9 | **`E152`** | le due proprietà di §5.7 mancanti si iniettano **in `process` e `ipc`**: esistono solo dopo 3 e 4 |
| 10 | **la chiusura** | come al Traguardo 3 e al 5: è un **audit**, non una scrittura |

---

## 2. `E30`, `R6` ed `E21` — le forme — ✅ approvata

### 2.1 Il problema, misurato

Il `Grant` è un gettone usa-e-getta. Tre cose lo consumano: `Arbiter::release`,
`Process::start` che riesce, e **`Process::start` che fallisce**.

| Via | Chi restituisce la riserva |
|---|---|
| worker avviato e finito | ⛔ **nessuno** — è `R6` |
| **avvio fallito** | ⛔ **nessuno**, e non era discusso da nessuna parte |

⛔ **La seconda non era nota.** Censita prima di dichiararla:

```
grep -rniE "start.{0,40}fail.{0,80}grant|grant.{0,80}start.{0,30}fail" crates/ --include=*.rs
```

`start` prende il `Grant` **per valore** e su `Err` lo lascia cadere; ricostruirlo è impossibile
— `GrantId` è privato e `crates/kernel/tests/compile_fail/grant_has_no_constructor.rs` prova che
`Grant` non ha costruttore. La riserva resta nei libri per **l'intera finestra dichiarata**
(`Held::expires_at`, scritto da `Arbiter::issue`), e la recupera solo la spazzata.

### 2.2 `E30` non è implementabile senza `E21`

La decisione di merito del 2026-08-28 (`9a18f36`) dice: concessione **propria** mai `Err`,
concessione **altrui** ancora errore. Per rispondere così, `release` deve sapere *«questa è
mia?»*.

Oggi non può. `Arbiter` non ha nessun campo d'identità, e `next_grant: 0` sta a
[`crates/kernel/src/arbiter/mod.rs:447`](../../../crates/kernel/src/arbiter/mod.rs): **ogni
arbitro parte da zero**, quindi due arbitri coniano lo stesso `GrantId`. La domanda che
`release` sa porre è *«è nei miei libri?»*, e `held.remove` risponde `None` sia alla propria già
spazzata sia a quella di un altro.

⛔ **E la conseguenza è più forte di *«servirebbe anche quella»*:**
`crates/kernel/tests/arbiter_admission.rs` asserisce
`second.release(grant, …).is_err()` nella sonda
`a_grant_released_on_the_wrong_arbiter_is_an_error_and_not_a_silent_credit`. Implementare `E30`
senza `E21` la rende **rossa**, e chiuderla significherebbe **cancellare una sonda per prendere
una decisione** — la formula del gotcha **#73** al contrario.

⚠️ **Una formulazione precedente diceva *«credito silenzioso»* ed è imprecisa:**
`AlreadyCollected` non restituisce nessun `Mib` e non tocca il budget. Il danno è la perdita
della protezione, non un accredito.

**Due vie per evitare l'identità, scartate:**

| Via | Perché cade |
|---|---|
| il **contatore** come discriminante (`id >= next_grant` ⇒ mai emessa da me) | non è **sana**: un arbitro più giovane accredita in silenzio la concessione di uno più vecchio |
| un **insieme delle già-spazzate** accanto a `held` | distingue i tre casi ma **cresce senza limite**, e potarlo rimette l'ambiguità: sposta il problema |

Resta l'identità, ed è ciò che `E21` già dice. ADR-0034 la colloca da sé: è un **parametro
consegnato**, accanto a `total_vram` in `Parameters`; il kernel non lo legge e non lo genera —
§6.1.3 vieta esplicitamente di generare identificativi.

### 2.3 Le forme

```rust
// arbiter — E30 + E21
pub struct ArbiterId(u64);           // consegnato in `Parameters` (ADR-0034)

pub enum Released {
    Now(Mib),                        // ripresa adesso: questo è tornato nel budget
    AlreadyCollected,                // la spazzata l'aveva già presa — finestra o grazia
}

pub fn release(&mut self, grant: Grant, now: Monotonic) -> Result<Released, ReleaseError>
// `UnknownGrant` significa ora UNA cosa: mai emessa da questo arbitro — difetto del chiamante
```

```rust
// porta process — R6, e la metà che mancava
#[must_use]
pub enum Started<H> {                // la forma di `Admission`, non un `Result`
    Running(H),
    Rejected { grant: Grant, error: ProcessError },   // la concessione torna PER NOME
}

fn start(&mut self, grant: Grant, descriptor: WorkerDescriptor) -> Started<Self::Handle>;

#[must_use]
pub struct Killed {
    pub grant: Grant,                // FUORI da ogni Result: torna sempre
    pub outcome: Result<(), ProcessError>,
}

fn kill(self) -> Killed;
```

⛔ **`release` tiene il `Result`, e non per gusto:** la decisione del 2026-08-28 lo scrive alla
lettera nel sorgente — *«Only the grant this arbiter never issued is a caller defect, and that
one stays an error»*. Collassare `UnknownGrant` dentro l'enum contraddirebbe una decisione presa.

⛔ **`start` e `kill` NON usano l'idioma «l'errore trasporta il valore».** Censito:

```
grep -rnE "Result<[^,]+, *\([A-Z]" crates/ --include=*.rs
```

non restituisce **niente**: nessun errore di questo repository trasporta il valore consumato. La
forma che il progetto usa per *«più esiti, ciascuno che porta ciò che gli spetta»* è `Admission`
— un `#[must_use] enum` con campi nominati. Introdurre l'idioma Rust standard sarebbe stato un
**secondo modo** per una cosa che il progetto fa già in un modo.

`Started` è generico dove `Admission` non lo è, e non è un modo nuovo: `Process::start`
restituisce il tipo associato `Self::Handle`, quindi il parametro è imposto.

`Killed` è una **struct** e non un enum perché non ci sono due stati: c'è **uno** stato con due
fatti. Il `Grant` fuori da ogni `Result` insegna *«torna comunque»*, e serve proprio sul ramo in
cui il worker è morto male — `kill` è *«sempre lecito»* (§5.3 punto 4), e la riserva è un fatto
dei **libri**, non della salute del processo.

### 2.4 I costi

| Costo | |
|---|---|
| `Parameters` guadagna un campo | tocca la radice di composizione e il banco. È il tipo che §2.8 pinza con **due** righe di catalogo — che sono **spec** |
| `start` e `kill` cambiano firma | i chiamanti sono banchi; nessun consumatore di produzione. Censimento: `grep -rn "\.release(\|\.kill(\|\.start(" crates/ --include=*.rs` |
| ⛔ **tre casi `compile_fail` toccati** | `instructing_after_the_kill.rs`, `reading_without_a_receipt.rs`, `reading_twice_from_one_receipt.rs`. I loro `.stderr` vanno **riletti uno per uno, mai rigenerati in blocco** — vincolo 10 della §11, gotcha **#25** |
| `ReleaseError` resta a **una** variante | e non è la forma ① scartata da `9a18f36`: lì una variante nuova lasciava tre cause in due caselle. Qui le cause **diventano** tre risposte |

---

## 3. §6.10 — il canale worker — ✅ approvata

### 3.1 Le due regole di §6.10.4 esistono già

La spec le presenta come regole nuove uscite da una misura. Il repository le ha **già risolte
entrambe**, nel giornale:

| Regola §6.10.4 | Dove è già risolta |
|---|---|
| ogni `Vec<u8>` porta l'annotazione di stringa di byte | `crates/kernel/src/record.rs` — `#[cbor(with = "minicbor::bytes")]`, col commento che la dichiara **portante e non decorativa** |
| la decodifica verifica i byte consumati | `Record::decode` costruisce un `Decoder` esplicito e controlla `decoder.position() != bytes.len()`, **un'uguaglianza e non un `>=`**. È il finding **AUD-047**, `8e25913` |

Quindi il canale worker **non progetta un secondo codificatore**: riusa la forma di `record.rs`.

⛔ **Ma prende la meccanica, non la disciplina**, e §6.10.3 lo dice alla lettera: niente enum di
versione, niente registro di indici ritirati, **niente byte congelati**.

### 3.2 La lunghezza dichiarata non è ridondante

| Guasto | Chi lo prende |
|---|---|
| coda dopo l'ultimo elemento | `position() == len()` |
| **frame troncato** | **solo** una lunghezza dichiarata — la coda non c'è, e il CBOR può essere completo lo stesso |

§6.10.5 riga 5 li nomina **entrambi**. ⚠️ **Che serva la lunghezza per il troncamento è
ragionamento, non misura:** il requisito è scritto, la giustificazione è di questo disegno.

La lunghezza sta **fuori** dal corpo CBOR, a larghezza fissa: dentro sarebbe un punto fisso,
perché un intero CBOR occupa 1, 2, 3, 5 o 9 byte a seconda del valore.

### 3.3 `Frame` resta opaco

§6.10.3: la porta scambia **byte**, non messaggi tipizzati, come `journal` dopo ADR-0036. Il
codificatore vive in `kernel`.

### 3.4 ⛔ Il timbro di build: la derivazione **non si disegna qui**

**Nessun punto** della spec o degli ADR dice come si calcoli il valore. Verificato:

```
grep -rn "timbro" docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md docs/adr/*.md docs/design/*.md
```

⚠️ Una delle occorrenze dentro §7.4 è un **falso positivo** — *«un timbro monotono»* parla di
`Monotonic` — colta leggendo la riga intera (gotcha **#70**).

Quattro derivazioni valutate, **nessuna regge oggi**:

| Via | Perché cade |
|---|---|
| costante a mano | marcisce — gotcha **#31** |
| `CARGO_PKG_VERSION` | **vacuo proprio quando serve**: uno schema che cambia fra due release porta la stessa versione |
| hash del commit | pretende un `build.rs` che **non esiste** (`find crates -name build.rs` → nulla) ed entra nel grafo di build che ADR-0031 governa |
| tabella dello schema a `const` | riporta la sincronizzazione a mano un livello più in là |

⛔ **Ma la ragione del rinvio non è che siano imperfette: è che il timbro non ha un
consumatore.** Non esiste una GUI (sotto-progetto 2) né un processo worker (§0.2). Disegnarne la
derivazione congelerebbe un meccanismo contro un consumatore immaginario — il gotcha **#46** dal
verso sbagliato, cioè **la stessa ragione** per cui `E30` e `R6` non furono costruite al
Traguardo 5.

✅ **E l'asimmetria col gettone di conformità è misurata, non asserita:** `Q13` ha una **riga di
catalogo** scritta e non chiusa; il timbro **non ne ha nessuna**. Rinviare l'uno e costruire
l'altro non è incoerenza — uno è dovuto per iscritto, l'altro no.

Il traguardo fa ciò che §6.1.4 prescrive per un caso gemello: **non costruisce, ma non
preclude**.

⚠️ **Costo dichiarato:** formalizzare il rinvio vuole una **riga C in §0.4**, e §0.3 dice che
*«un pezzo scaglionato senza una riga C esplicita è un errore di questa sezione»*. La §0.4 è
**spec** — vincolo globale 7, quindi **del proprietario**.

### 3.5 ⛔ La decisione C-1 è datata a questo traguardo, e sposta l'ordine

`crates/kernel/Cargo.toml` la porta scritta, alla lettera:

> «measured 2026-08-18, this crate has **ZERO production uses** of `bincode` … The `ipc` schema
> is Milestone 6. It is a window that closes on its own … **DECIDE AT MILESTONE 6, while the
> choice is still free**»

`bincode` 2.0.1 è coperto da RUSTSEC-2025-0141, *«Bincode is unmaintained»*, categoria `INFO`. Il
manifesto nomina la causa: ADR-0037 chiede se il **pari** ha un lettore mantenuto, M-1 chiedeva
se il **grafo** è accettabile, e **nessuno dei due** chiede se la libreria dalla **nostra** parte
è ancora mantenuta.

⛔ **Scrivere lo schema `ipc` in `bincode` _è_ la decisione, presa per omissione.** Va inserita
**prima** — è il compito **3bis** di §1.4.

⛔ **E non si riapre per simmetria.** La §8 del compendio lo vieta per nome: *«riaprire §6.1.1
tanto ora c'è `minicbor` nel kernel»* è stato **tentato il 2026-08-08 e la misura ha dato
torto** — i due canali hanno pari diversi. L'argomento vivo è **solo** C-1, che è di specie
diversa, e pretende una **misura nuova e odierna**: `bincode` è ancora non mantenuto? esiste
un'alternativa mantenuta il cui pari TypeScript abbia un lettore? ⚠️ **Quella misura è un
compito**, non una cosa da decidere a memoria — sarebbe il gotcha **#48**.

---

## 4. §6.2 e §6.3 — il decisore e il gettone — ✅ approvata

### 4.1 I gettoni non hanno tutti la stessa forza

| Gettone | Costruttore pubblico | Cosa prova |
|---|---|---|
| `Grant` | ⛔ **nessuno** — solo `Arbiter::issue`, privata | non falsificabile; `grant_has_no_constructor.rs` lo pinza |
| `Instruction` | ✅ `Instruction::new(String)` | guardia sulla **sola direzione pericolosa**: `Untrusted → Instruction` |
| `SingleReceipt` · `StreamReceipt` | ✅ `new(id: u64)` | l'**arità**, non l'autenticità — dichiarato accanto al costruttore |

📌 **La regola che li spiega tutti e tre, e che non stava scritta da nessuna parte: un gettone è
non falsificabile esattamente quando il suo produttore vive DENTRO la crate che lo definisce.**
Le ricevute hanno un costruttore pubblico perché chi le produce è `platform`, **fuori**: senza,
la porta non sarebbe implementabile — gotcha **#46**, e il sorgente lo dichiara.

⛔ **Il produttore della prova di conformità è il filtro dei vincoli, che è logica di `kernel`**
(ADR-0020). Dentro la crate. Quindi prende la forma di **`Grant`**: nessun costruttore pubblico,
coniata da una funzione privata del filtro.

### 4.2 La riga di catalogo che il traguardo chiude è **una**, ed esiste già

Nel blocco B della §7.4 — cercata per **intestazione** e non per posizione, perché lì la colonna
«Difende» è la **terza** (trappola 3 di `check-docs.sh`):

| Per fare questo | va consegnato | Difende | Sonda: *deve scattare* | Contro-sonda: *deve restare verde* |
|---|---|---|---|---|
| eseguire una richiesta | una **prova di conformità** | **Q13** | candidato non filtrato → **non compila** | filtrato → compila |

Delle cinque righe del blocco dei gettoni, **quattro sono già chiuse**. Come al Traguardo 5: il
traguardo **non crea** righe di catalogo, ne **chiude** una già scritta.

### 4.3 Il costo trovato, dichiarato come deduzione

Il decisore produce il **record di routing risolto**, e ADR-0011 lo vuole **giornalato col
passo**. Quindi, a differenza del canale worker, qui la disciplina di §4.9 si applica **per
intero**.

E `RecordKind` è uno dei tre enum `index_only` i cui indici sono pinzati dai **byte congelati**.

⚠️ **Che aggiungere una variante lasci i byte congelati identici è una DEDUZIONE, non una
misura.** La regola 3 di §4.9 parla di **campi**, non di varianti di enum. **Si misura prima di
scrivere**, in due direzioni — è la forma del gotcha **#54**.

---

## 5. §6.4, §6.6, §6.7 — sensore, permesso, degrado — ✅ approvata

### 5.1 ⛔ Tre righe dicono ✅ e non hanno un controllo

§8.1 definisce `✅ verificato qui` come *«esiste un controllo **in perimetro**, visto scattare e
visto restare verde»*. Tre righe lo portano, e i controlli che nominano **non esistono**:

| Sigla | Il controllo che nomina | Nel codice |
|---|---|---|
| **V10** | *«§7.4.1 C riga V10 — un sensore che modifica l'artefatto non compila»* | ⛔ **33 casi `compile_fail`, nessuno su un sensore** |
| **V14** | *«test a esempi con sensore finto e verdetto scelto dal test»* | ⛔ nessun sensore finto |
| **Q10** | *«test a esempi con sensore finto»* | ⛔ idem |

📌 **È il gotcha #49 ROVESCIATO.** La §6.5 *sembrava* lavoro ed era già fatta; queste *sembrano*
fatte e sono lavoro intero. La domanda del pre-controllo dà la risposta sbagliata in **entrambe**
le direzioni se ci si ferma alla marca.

⚠️ **Se siano le righe a essere sbagliate o la notazione di §8 a essere in avanti è del
proprietario**: §8 è spec, vincolo globale 7. La conseguenza sul piano non dipende da quale delle
due: **i tre controlli si costruiscono comunque**.

⚠️ **NON MISURATO, dichiarato come tale:** che `check-docs.sh` verifichi l'innesco di
`parziale`/`rimandato` ma **non** possa verificare che un ✅ nomini un controllo esistente. È una
deduzione da come §8.6 è descritta, non una lettura dello script — **va misurata** prima di
costruirci un rimedio.

### 5.2 Le decisioni

**§6.4 — il sensore.** Contratto di ADR-0009, deliberatamente povero, **portato** senza
ridiscuterlo. Il «costo» sono **due** cose, e la spec l'ha già deciso: *dichiarato* (statico, nel
registro, decide l'ammissione all'anello stretto — è `V11`) e *speso* (nel verdetto, misurato,
entra nel giornale). L'artefatto va **per riferimento immutabile**: è il 34° caso `compile_fail`.
Il sensore finto prova che **l'anello** funziona, non che il **contratto** regga sensori veri —
RK-5, già accettato.

**§6.6 — il permesso.** Forma `(strumento × risorsa × operazione)`. *«Quali permessi sono attivi
ora»* è una **proiezione del giornale**, e il progetto ha già un modo:
`reconcile::steps_in_doubt` è una **funzione libera** che prende la porta per riferimento,
rilegge e deriva. La proiezione dei permessi prende quella forma. La registrazione è un fatto
giornalato → stessa conseguenza di §4.9 del record di routing (§4.3).

**§6.7 — il degrado: si RICALCOLA, non si cachea.** Segue `steps_in_doubt`, e rende *«mai
autorevole di per sé»* vero **per costruzione** invece che per disciplina. Una cache si compra il
giorno in cui una misura la chiede — la formula del checkpoint del giornale.

⚠️ **DIVERGENZA DICHIARATA.** ADR-0019 e §6.7 dicono che il core *«**mantiene** uno stato di
degrado corrente, **alimentato dagli eventi**»*, e quelle parole si leggono anche come
mantenimento **incrementale**. Questo disegno legge «mantiene» come «espone», non come «cachea».
Se la lettura del proprietario è l'altra, la scelta è sua.

### 5.3 Cosa chiude e cosa **non** chiude

| | |
|---|---|
| chiude | **una** riga di catalogo: `V10`, blocco C, livello 1 |
| porta a ✅ | `V10`, `V14`, `Q10` — che oggi lo dicono senza averlo |
| **resta `⚠️ parziale`** | `V11` (innesco C4), `V21` (C4), `V27` (A2), `Q18` (B3) |

⛔ **Nessuna di queste quattro va marcata ✅ a fine traguardo:** le loro seconde metà pretendono
un sensore inferenziale, il mediatore, l'interfaccia e una `network` vera, tutti fuori per regola
C o B. Marcarle sarebbe la casella comoda che l'innesco obbligatorio della §8.1 esiste per
impedire.

---

## 6. §6.1 — lo schema `ipc` — ✅ approvata

### 6.1 Il perimetro: la busta e due messaggi

⛔ **Il vocabolario NON si progetta qui oltre il minimo, ed è la stessa postura della §3.4** —
ma non per simmetria con essa: per una ragione più forte e misurabile. Lo schema ha un
consumatore **scritto e non chiuso**, il timbro no.

| | Fatto | Dove |
|---|---|---|
| 1 | la terza proprietà di §5.7 si inietta **sulla porta `ipc`** — *«la GUI muore tenendo una concessione discrezionale → la somma torna alla linea di base»*, requisito **Q3 esteso** | §5.7 della spec |
| 2 | quella riga di catalogo di livello 2 è **PARZIALE**, ed è la voce **`E152`**, il cui chiusore dichiarato è **questo traguardo** | tabella unica di [`porta-di-qualita.md`](../../porta-di-qualita.md) |
| 3 | la **suite di conformità** di `ipc` è rimandata, e la ragione scritta è *«non esiste una GUI dall'altro capo»* | §7.4.6 della spec |

Quindi: il **meccanismo** è dovuto per iscritto, il **vocabolario** no.

### 6.2 I due messaggi non sono stati scelti

⛔ **C'è UNO scambio che ADR-0033 già fissa e che §5.7 già pretende, e sono lo stesso.**

| Direzione | Messaggio | Chi lo impone |
|---|---|---|
| gui → core | **richiesta di concessione ordinaria**, col profilo di risorsa dichiarato | [ADR-0033](../../adr/0033-gpu-della-gui-quota-di-presentazione.md), consumatore **3**: *«viewer 3D oltre la quota → concessione ordinaria, richiesta via IPC»* |
| core → gui | **esito dell'ammissione**, a tre vie | ADR-0033: *«esito a tre vie»* — in codice `Admission` |
| *(nessun messaggio)* | la **disconnessione** | §5.7 riga 3, iniettata sulla porta `ipc` |

📌 **Uno per direzione, e sono ciò che rende NON VACUA la campagna del compito 9.** Perché
quella proprietà dica qualcosa, la finta gui deve **tenere davvero** una concessione
discrezionale prima di morire: senza la richiesta e l'esito il seme ucciderebbe un client che
non ha mai chiesto nulla, e il confronto sarebbe fra insiemi vuoti — la lezione che il
Traguardo 4 ha imparato tre volte.

⛔ **Ciò che NON entra, dichiarato invece che dimenticato: la REVOCA core → gui.** ADR-0033 la
nomina — *«la GUI smette di renderizzare il 3D e lo dichiara»* — ed è il primo messaggio che il
vocabolario guadagnerà. Non entra perché nessuna riga scritta la pretende oggi, e perché §5.7
riga 3 parla di una GUI che **muore**, non di una a cui si chiede indietro. ⚠️ **Il costo è
reale:** fino ad allora una concessione discrezionale è prelazionabile **nei libri** e la GUI
non lo sente mai. Voce **7** per il proprietario.

### 6.3 La busta

```
[ lunghezza dichiarata : larghezza fissa ] [ corpo codificato ]
```

| Scelta | Perché |
|---|---|
| la lunghezza sta **fuori** dal corpo, a larghezza fissa | dentro sarebbe un punto fisso: un intero codificato occupa 1, 2, 3, 5 o 9 byte secondo il valore. È la forma della §3.2, già approvata per il canale worker |
| ⛔ **l'inquadratura è UN modulo solo in `kernel`, condiviso dai due canali privati** | è **indipendente dal formato**: entrambi i pari devono leggere un prefisso di lunghezza comunque. Nasce al compito **3** e il compito **4** la riusa |
| ⛔ **il corpo è un'ENUMERAZIONE di messaggi**, mai una struttura sola | è ciò che rende vera la promessa *«non precludere»* della §3.4 senza costruire niente: **il timbro di build diventa un tipo di messaggio nuovo**, e aggiungerne uno non tocca l'inquadratura |

⚠️ **La terza riga parla dell'INQUADRATURA, non della stabilità dei byte**, e la distinzione va
tenuta: qui **non ci sono byte congelati** (§6.4), quindi non esiste la domanda che la §4.3 pone
per `RecordKind`. Ciò che si compra è che un tipo di messaggio nuovo non obblighi a ridisegnare
la busta.

⛔ **E il controargomento va detto perché qualcuno lo troverà.**
[ADR-0037](../../adr/0037-criterio-del-pari-per-il-formato-dei-canali.md) diffida degli
argomenti **di simmetria** fra i due canali privati, e
[ADR-0035](../../adr/0035-porta-verso-i-worker-e-lettura-di-i4.md) dice *«un
meccanismo di trasporto e uno schema **per canale privato**»*. **La lettura di questo disegno è
che la condivisione regge:** ciò che si condivide non è né il trasporto né lo schema — sono
**byte di busta** — e la ragione non è la simmetria ma che il problema è **letteralmente lo
stesso**. I due schemi restano distinti e i due formati pure. ⚠️ **Se la lettura del proprietario
è l'altra, la scelta è sua** — voce **8**, sul precedente della divergenza dichiarata della §5.2.

📌 **La larghezza in byte non si decide qui**, e non è una lacuna: si decide **una volta sola**,
al compito 3, dove l'inquadratura nasce. Deciderla anche qui sarebbe il gotcha **#68**.

### 6.4 La disciplina di evoluzione, e il buco dichiarato

⛔ **`ipc` NON prende la disciplina della §4.9.** I4 rinuncia al versionamento, quindi **niente**
enum di versione, **niente** registro di indici ritirati, **niente byte congelati**. È ciò che la
§3 ha già deciso per il canale worker; lì la ragione era §6.10.3, qui è I4 direttamente.

⚠️ **Ma il meccanismo che SOSTITUISCE il versionamento è il timbro di build, ed è rinviato dalla
§3.4.** Va scritto senza addolcirlo:

> **Finché il timbro non esiste, nulla rifiuta una GUI stantia.** I4 rinuncia al versionamento
> *a condizione* che il timbro faccia il suo lavoro; oggi la condizione non è soddisfatta.

✅ **E oggi non costa nulla, misurato e non dedotto** — non esiste nessuna GUI da rifiutare:

```
grep -rn "impl Ipc" crates/ --include=*.rs
```

restituisce **solo** la finta di un banco. Il buco si paga al sotto-progetto 2, ed è lo stesso
innesco della voce **5**.

### 6.5 Gli identificativi — la regola non ha un sito, oggi

§6.1.3 dice che gli identificativi dello schema sono i **progressivi del giornale**, mai
generati. Misurato contro il vocabolario della §6.2:

```
grep -nE 'pub fn admit|pub fn set_policy' crates/kernel/src/arbiter/mod.rs
```

Solo `set_policy` riceve un `Journal`; **`admit` prende `(profile, valid_for, now)` e
restituisce `Admission`**. Una richiesta di concessione non è un passo di una run, non scrive
record, e non porta né `StepId` né `RunId`.

⛔ **Quindi la regola è soddisfatta A VUOTO, e dichiararlo è il punto.** Scrivere *«§6.1.3
rispettato»* sarebbe verde avendo confrontato insiemi vuoti — la forma che questo repository si
è impegnato a non ripetere. La formulazione onesta:

> Lo schema **non conia** identificativi perché oggi **non ne porta nessuno**. Nessun controllo
> può esercitare la regola: il primo messaggio che porterà un identificativo è il sito dove la
> regola diventa reale, e dove nasce la sua sonda.

✅ **E l'allocatore di `StepId` resta non costruito**, com'era già registrato in
`crates/kernel/src/ports/journal.rs`. ⚠️ La frase del doc di `ClientId` — *«chi implementa questa
porta al Traguardo 6 attinge da QUEL contatore»* — invecchia **nel soggetto** e non
nell'affermazione (gotcha **#87**): si **ri-punta** all'implementatore reale con richiamo datato,
non si toglie. Toglierla lascerebbe scoperto il difetto che esiste per impedire — due contatori
identici che divergono senza che nulla lo segnali.

### 6.6 Cosa NON cambia

| | |
|---|---|
| le tre firme della porta `ipc` | ⛔ **invariate**. `send`/`receive` continuano a scambiare `&[u8]`/`Vec<u8>` e **non** guadagnano un newtype come il `Frame` di `process`: il doc della porta dichiara le proprie firme *«aperte a un argomento misurato»*, e qui non c'è nessuna misura che lo chieda |
| la questione aperta di `accept` | ⛔ **resta aperta**, e questa sezione non la chiude: il suo prezzo è **la firma**, non una variante in più, ed è scritto sulla porta |
| `IpcError` | **due varianti**, invariate. `MalformedMessage` acquista finalmente un produttore: oggi il suo doc promette *«i byte consumati non uguagliano la lunghezza dichiarata»* e nessun codice lo produce |

### 6.7 Il controllo che esercita ciascun artefatto

| Artefatto | Deve scattare | Deve restare verde |
|---|---|---|
| lunghezza dichiarata | frame **troncato** → `MalformedMessage` | frame intero → decodifica |
| byte consumati uguali alla lunghezza | **coda** dopo l'ultimo elemento → `MalformedMessage` | esatto → decodifica |
| il corpo è un'enumerazione | ⛔ **lo esercitano i DUE messaggi**, non uno: con un tipo solo il discriminante non sarebbe provato — stessa forma per cui i byte congelati del giornale sono **tre** record e non uno | |
| §5.7 riga 3 · **Q3 esteso** | la campagna DST del compito **9** | la somma torna alla linea di base |
| «il core decide quando emettere» | ⛔ **nessuna sonda nuova**: lo tiene la **forma della porta** — non c'è una terza operazione, e il doc di `crates/kernel/src/ports/ipc.rs` lo argomenta già | |
| §6.1.3, non coniare | ⛔ **nessun controllo, e dichiarato**: non c'è un sito (§6.5) | |

⛔ **E LA FINTA CHE SERVE AL COMPITO 9 NON ESISTE — misurato, e la prima stesura di questa
sezione affermava il contrario.** Il doc di `crates/kernel/src/ports/ipc.rs` dice che il banco
scrive una finta gui *«including the client that DIES WHEN THE SEED DECIDES»*, e ripeterlo era il
gotcha **#65**: un doc di modulo è un'affermazione come le altre.

```
grep -rn '\.dies(' crates/ --include=*.rs
ls crates/simulator/src/
```

`FakeGui` vive **solo** in `crates/kernel/tests/ports_are_implementable.rs`, e `dies` è chiamata
da **due test in modo esplicito** — nessun seme la muove. `crates/simulator/src/` porta
`MemoryJournal`, `CrashingJournal`, `VirtualReactor` e `SeededRng`, e **nessuna finta di `ipc`**.
📌 **Quindi il compito 9 COSTRUISCE una finta gui in `simulator`**, guidata dal seme, sul
precedente di `CrashingJournal`; ciò che `FakeGui` offre è **la forma**, non l'artefatto.

### 6.8 I costi

| Costo | |
|---|---|
| `kernel` guadagna **due** moduli | l'inquadratura (compito 3) e lo schema `ipc` (compito 4) |
| `simulator` guadagna una **finta di `ipc`** | e il compito 9 è più grande di quanto la §1.4 lasci intendere — §6.7 |
| `platform` **non** guadagna niente | nessun trasporto: è la voce **5** |
| se il compito **3bis** scartasse `bincode` | la lista di ADR-0031 cambia, ed è un **atto deliberato in due passi** — manifesto e `Cargo.lock` insieme, fuori dal cancello (finding **G-5**) |
| il **timbro** resta un buco dichiarato | e con esso la condizione di I4, non soddisfatta (§6.4) |
| la **revoca** verso la GUI non esiste | una concessione discrezionale è prelazionabile nei libri e la GUI non lo sente (§6.2) |

## 7. La chiusura — ✅ approvata

### 7.1 Tre decisioni prima delle condizioni

⛔ **I due precedenti NON concordano su chi verifica la chiusura, e la §7 deve scegliere.**
Misurato leggendo i due disegni invece che ricordandoli:

| | Traguardo 4 | Traguardo 5 |
|---|---|---|
| dove stanno le condizioni | §0.3 *«Definizione di "fatto"»* | §0.3, stessa forma |
| chi le verifica | una **§12 aggiunta al disegno** il giorno della chiusura, che le rilegge una per una **contro il codice** | il **Task 13 del piano**, un audit; il verbale è finito in [`porta-di-qualita.md`](../../porta-di-qualita.md) e nel compendio, e il disegno **non ha** una sezione di chiusura |
| cosa ha prodotto | *«la condizione 4 era scritta troppo larga»*, e una tabella **«dove il disegno è stato smentito dall'esecuzione»** | *«gran parte era già eseguita»*, e l'unica cosa che mancava era **la condizione che il piano aveva aggiunto** |

📌 **Il dato che decide è quello in basso a destra.** Il Task 13 non ha trovato scarti nei
riconteggi: a mancare era la **condizione 8**, *«le voci aperte in una tabella sola»* — **la sola
che il piano aggiungeva alle sette del disegno**. Il disegno del Traguardo 5 aveva dimenticato una
condizione, e a rimediare è stato chi scriveva il piano. ⛔ **Questa sezione esiste per non
ripeterlo:** la Definizione di «fatto» di questo traguardo nasce **completa qui**.

| | Decisione | Perché |
|---|---|---|
| **A** | la Definizione di «fatto» vive **qui**, non in una §1.5 | nei due precedenti sta in §0.3, dentro il perimetro. Qui la §1 è **approvata**, e il disegno è comunque completo prima del piano: spostarla non compra nulla e costerebbe un richiamo su una sezione chiusa. ⚠️ **La deviazione è nominata**, perché chi confronta i tre disegni non la legga come una dimenticanza |
| **B** | ⛔ **il verbale di chiusura si scrive QUI, come la §12 del Traguardo 4 — non nel compendio** | è la forma che ha prodotto *«la condizione 4 era scritta troppo larga»* e la tabella delle smentite: un verbale accanto alla cosa che giudica. ✅ **E c'è una ragione che al Traguardo 4 non esisteva:** dal 2026-08-28 `check-docs.sh` impone un **tetto al compendio** — misurato, il passo `== compendium size ceiling ==` è nel cancello — quindi un verbale scritto lì compete con quel tetto |
| **C** | la chiusura è un **compito del piano**, il **10** della §1.4, eseguito subagent-driven con revisione | come al Traguardo 5. La **B** dice *dove scrive*, la **C** *chi scrive*: non sono la stessa scelta, e i due precedenti le mescolavano |

### 7.2 La Definizione di «fatto»

⚠️ **Nessuna condizione porta un numerale che il traguardo può muovere**, e non è pedanteria: la
condizione 4 del Traguardo 5 diceva *«le **dodici** righe di catalogo»*, e il Task 13 ha dovuto
ricontarle. Qui le condizioni **nominano la sezione**, e il conteggio lo fa chi chiude.

| # | Condizione |
|---|---|
| 1 | `bash scripts/gate.sh` → `GATE GREEN`, e la baseline **rimisurata** col comando, non citata |
| 2 | le righe di catalogo che le §§**2–6** nominano sono **chiuse o dichiarate**, nessuna a metà — ricontate sulla §7.4 della spec, e *se il conteggio vero diverge, vince il conteggio* |
| 3 | ⛔ **`V10`, `V14` e `Q10` hanno un controllo che ESISTE** — oggi portano ✅ senza averlo (§5.1) — e ciascuno è stato **visto scattare** e **visto restare verde**, che è la definizione di §8.1 |
| 4 | **`E152` è chiusa**: le due proprietà di §5.7 che mancavano sono iniettate su `process` e `ipc`, ciascuna con la propria contro-sonda, e la **finta gui di `simulator` esiste** (§6.7) |
| 5 | **`E30`, `R6` ed `E21` sono chiuse insieme**, nelle forme della §2.3, e la sonda `a_grant_released_on_the_wrong_arbiter_is_an_error_and_not_a_silent_credit` è **ancora verde** — non cancellata (§2.2) |
| 6 | il **vincolo 15** della §11 è onorato: il frame dichiara la propria lunghezza, la decodifica verifica i byte consumati, e l'annotazione di stringa di byte è **sul canale worker** |
| 7 | la decisione **C-1** è presa con una **misura odierna** (§3.5); se il formato cambia, manifesto e `Cargo.lock` sono committati **insieme** e fuori dal cancello (finding **G-5**) |
| 8 | i casi `compile_fail` nuovi e i **tre toccati** dalla §2.4 hanno il proprio `.stderr` **letto uno per uno**, mai rigenerato in blocco — vincolo 10 della §11 |
| 9 | ⛔ **ogni non-costruzione dichiarata porta il proprio innesco**: il timbro di build (§3.4), il trasporto vero (voce **5**), la revoca verso la gui (§6.2) |
| 10 | il registro [`porta-di-qualita.md`](../../porta-di-qualita.md) è riallineato, coi **conteggi ricontati** e non dedotti |
| 11 | ⛔ **le voci che il traguardo lascia aperte stanno in UNA tabella sola, con la colonna di chi le chiude** — ed è la condizione che al Traguardo 5 il disegno aveva dimenticato |
| 12 | ⛔ **CONDIZIONE NEGATIVA:** `V11`, `V21`, `V27` e `Q18` restano `⚠️ parziale` e **non** sono marcate ✅ (§5.3) |

📌 **La 12 non ha precedenti nei due disegni, ed è deliberata.** Le loro Definizioni potevano
essere soddisfatte **marcando più del dovuto**, e nessuna delle due lo vietava: chiudere troppo è
un modo di **fallire** questa Definizione, non di superarla. È l'innesco obbligatorio della §8.1
letto dal verso che quella sezione dichiara di temere — *«`parziale` non diventi la casella
comoda»*.

### 7.3 Chi verifica, e come

⛔ **La chiusura è un AUDIT e non una scrittura** — gotcha **#49**, che al Traguardo 3 e al
Traguardo 5 si è presentato **due volte**, ogni volta con gran parte del compito **già eseguita**.

| | Regola per chi chiude |
|---|---|
| 1 | **si parte dai numeri, non dalle frasi**: ogni conteggio si rifà col comando prima di leggere ciò che i documenti ne dicono |
| 2 | **la prima domanda è *«è già fatto?»***, non *«come lo faccio?»* |
| 3 | ciò che si trova già eseguito si **riconosce**, non si riesegue — e si scrive che lo era |
| 4 | la §7.2 si rilegge **contro il codice**, mai contro sé stessa: una condizione può risultare **scritta troppo larga**, ed è successo alla 4 del Traguardo 4 |
| 5 | il verbale finisce in una **§8 di questo disegno** — decisione **B** — con la tabella *«dove il disegno è stato smentito dall'esecuzione»* |

### 7.4 Cosa la chiusura NON fa

| | |
|---|---|
| non chiude il **sotto-progetto 1** | resta la §8 di [`tracciabilita.md`](../../tracciabilita.md), che si aggiorna alla chiusura del sotto-progetto e non di un traguardo |
| non chiude le **voci del proprietario** | le otto che questo disegno apre, e quelle ereditate dal Traguardo 5 |
| non tocca [`riferimenti.md`](../../riferimenti.md) | la voce **E146** è *registrata e non presa*: cominciare la convenzione nuova a metà produrrebbe **due** convenzioni invece di una |

## Cosa questo disegno ha misurato, e che non era scritto da nessuna parte

⛔ **È la parte che costa di più riscoprire.** Ognuna è stata verificata con un comando.

| | Scoperta |
|---|---|
| 1 | la **§6.5 è già eseguita** — il traguardo è più piccolo di come la spec lo fa sembrare |
| 2 | **`V10`, `V14`, `Q10` sono ✅ senza controllo** — il traguardo è più grande di come §8 lo fa sembrare |
| 3 | un **avvio fallito non restituisce la concessione**, e non era discusso in nessun documento né sorgente |
| 4 | **`E30` pretende `E21`**, e le due righe adiacenti della tabella unica non si nominavano |
| 5 | la **decisione C-1 è datata a questo traguardo** e vive nel **manifesto**, non nella tabella delle voci aperte |
| 6 | le **due regole di §6.10.4 esistono già** in `record.rs` |
| 7 | **nessun errore di questo repository trasporta il valore consumato** — la forma del progetto è `Admission` |
| 8 | il **timbro di build non ha una riga di catalogo**, mentre `Q13` sì |
| 9 | **la regola dei gettoni**: non falsificabile ⟺ produttore dentro la crate |
| 10 | la metà di **prontezza** della porta `reactor` **non ha un produttore**, e la porta ha già **tolto** la forma che la porterebbe. ⛔ **RICHIAMO DEL 2026-08-30:** qui seguiva *«— è ciò che tiene fuori il trasporto vero di `ipc` e `process`»*, ed è **falso, misurato**: le due porte sono a **interrogazione** e non chiedono nessuna sveglia. A tenerle fuori sono §0.2 e §0.4 riga §1 — vedi il richiamo in §1.2. La scoperta **resta**, la conseguenza cade |
| 11 | `admit` **non riceve un `Journal`** — solo `set_policy` lo fa — quindi lo scambio minimo di §6.2 non porta nessun identificativo, e §6.1.3 è soddisfatta **a vuoto** |
| 12 | **`ADR-0033` fissa già lo scambio minimo**, e coincide con ciò che §5.7 riga 3 pretende: il vocabolario non è stato scelto |
| 13 | ⛔ **la finta gui che il compito 9 richiede NON esiste**: `FakeGui` vive nel banco di `kernel` e muore per chiamata esplicita di due test, e `crates/simulator/src/` non ne ha nessuna — la prima stesura della §6 affermava il contrario, ripetendo un doc di modulo (gotcha **#65**) |
| 14 | ⛔ **i due precedenti di chiusura NON concordano su chi verifica**: il Traguardo 4 aggiunge una sezione al proprio disegno, il Traguardo 5 lo fa dal piano e scrive altrove — la §7.1 sceglie |
| 15 | al Traguardo 5 **il disegno aveva dimenticato una condizione di chiusura**, e a rimediare fu il piano: i riconteggi del Task 13 non produssero **nessuno** scarto, e a mancare era la sola condizione che il piano aggiungeva |

📌 **E il gotcha #88, nato stamattina, si è confermato TRE volte in questa sola sessione:** un
censimento per pattern ha restituito **meno** case di quante ce ne fossero — la frase spezzata a
capo nel compendio, la colonna «Difende» non in prima posizione nel catalogo, e le righe di §8.3
scritte `| V10 |` **senza grassetto** mentre il filtro pretendeva `**V10**`.

---

## Le voci che questo disegno apre per il proprietario

Nessuna è un difetto oggi. Tutte sono **registrate e non prese**.

| | Voce | Perché è sua |
|---|---|---|
| 1 | la **riga C in §0.4** che formalizza il rinvio del timbro di build | §0.4 è spec — vincolo globale 7 |
| 2 | se `V10`/`V14`/`Q10` siano righe **sbagliate** o notazione **in avanti** di §8 | §8 è spec |
| 3 | la lettura di *«mantiene»* in ADR-0019: **espone** o **cachea** | tocca un ADR `Accepted` |
| 4 | se `check-docs.sh` possa confrontare un ✅ con l'esistenza del controllo | sarebbe una riga di catalogo nuova |
| 5 | ~~le due righe di **§7.4.6** passano da ✅ a ⏳ con innesco *«la prima sorgente di eventi esterni sul `reactor`»*~~ — ✅ **CHIUSA IL 2026-08-30, e non nella forma qui scritta:** passano a **❌ scaglionata**, la parola che quella colonna già usa, e l'**innesco proposto era sbagliato** (§1.2) | era §7.4.6, **spec** — vincolo globale 7. ⛔ **Chiusa dal proprietario**, che ha visto la misura prima della scrittura |
| 6 | se il buco del **timbro** (§6.4) debba avere una **riga C in §0.4**, come il rinvio della §3.4 | §0.4 è spec |
| 7 | se la **revoca** verso la gui vada nel Traguardo 6 dopo tutto, dato che ADR-0033 la nomina e nessun innesco la tiene | è un allargamento di perimetro |
| 8 | se l'**inquadratura condivisa** fra i due canali privati (§6.3) sia economia o erosione della lettura di *«singolo»* di ADR-0035 | tocca la lettura di un ADR `Accepted`, come la voce **3** |

---

## Il prossimo passo

✅ **Il disegno è completo.** Ciò che manca non è più una sezione: è la **condizione per passare
al piano**, e non è scritta qui.

✅ **QUELLA CONDIZIONE È CHIUSA IL 2026-08-30, e non era quella che sembrava.** Due righe di
[`AVVIO-CHAT.md`](../../AVVIO-CHAT.md) si contraddicevano — *«le voci aperte NON lo sbarrano»*
contro *«MAI prima che le voci aperte siano chiuse»* — e `CLAUDE.md` si schierava con la seconda.
⛔ **A cadere è stata la seconda, e non per preferenza: era INSODDISFACIBILE.** Fra le voci
aperte ce ne sono con chiusore *«il traguardo della ritenzione»* e *«nessuno finché nessuna
misura lo chiede»*, quindi sotto la lettera nessun piano avrebbe mai più potuto essere scritto;
e i piani del **Traguardo 4** (2026-08-11) e del **Traguardo 5** (2026-08-18) furono scritti
**dopo** che quelle voci erano state raccolte. Gotcha **#89**.

📌 **La regola vera la enunciava già la raccolta stessa:** le voci si **SANNO** prima di
scrivere. A **sbarrare** è la colonna *«Chi la chiude»* di
[`porta-di-qualita.md`](../../porta-di-qualita.md) — e sotto quella regola le voci che nominano
questo traguardo sono `E30`, `R6` ed `E152`, **tutte e tre portate dalle §§2, 6 e 7**.

✅ **E L'ULTIMO SBARRAMENTO È CADUTO IL 2026-08-30: NON RESTA NULLA FRA QUESTO DISEGNO E IL
PIANO.** Era la **voce 5** — finché §7.4.6 diceva ✅ per il trasporto di `ipc` e `process`, un
piano scritto contro quella spec risultava **mancante di due compiti**. Le due celle passano a
**❌ scaglionata** col richiamo datato, e la spec smette di pretendere un trasporto che §0.2 e
§0.4 riga §1 scaglionano.

⛔ **E chiudendola si è misurato che questo disegno sbagliava la RAGIONE, non il perimetro:** il
richiamo in §1.2 la attribuiva alla prontezza mancante del `reactor`, e le due porte sono **a
interrogazione**. Il perimetro regge intatto; l'argomento è **riscritto**, non appeso. Chi
scrive il piano legge quel richiamo **prima** della §1.2, o riderivarebbe la ragione sbagliata.

⛔ **Che cosa venga dopo NON è scritto qui, ed è deliberato:** è uno **stato**, e la sua casa
unica è la §6 del [compendio](../../COMPENDIO.md). ⚠️ **La prima stesura di questo blocco lo
scriveva lo stesso, con un `⏭️`** — cioè apriva una seconda casa del puntatore dentro il
documento che chiude l'ultimo sbarramento. Gotcha **#68**, tolto prima del commit.

📌 **Ciò che questo disegno consegna a chi scriverà il piano**, che è suo e non un puntatore: i
**dieci compiti** ordinati dalla §1.4, la Definizione di «fatto» della §7.2, e le voci il cui
chiusore è **questo traguardo** — `E30`, `R6`, `E152` — portate dalle §§2, 6 e 7 e da **chiudere
nel piano**. ⛔ **Le altre voci aperte non sbarrano: si SANNO** (gotcha **#89**).

⚠️ **RICHIAMO DEL 2026-08-30:** questo blocco diceva *«Presentare la sezione 7»*, poi
*«RESTA UN SOLO SBARRAMENTO»*.
