# Traguardo 6 — gli altri meccanismi: il disegno

⛔ **QUESTO DISEGNO È IN CORSO, e non è una formalità dirlo.** Le sezioni **1–5 sono
approvate** dal proprietario, una per volta; le **6 e 7 non sono state nemmeno presentate**.
Chi riprende **non** ha un disegno completo da tradurre in piano: ha cinque sezioni chiuse e
due da aprire. Trattarlo come finito produrrebbe un piano che salta lo schema `ipc` e la
chiusura.

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

## 6. §6.1 — lo schema `ipc` — ⬜ **DA PRESENTARE**

Non è stata né discussa né approvata. Ciò che è già fissato altrove e la vincola:

- il **formato** lo decide il compito **3bis** (§3.5), non questa sezione;
- gli **identificativi sono i progressivi del giornale**, mai generati (§6.1.3);
- il **timbro di build** è rinviato (§3.4), e la forma non deve precluderlo;
- §6.1.4: il core decide **quando** emettere, la GUI non tira — *«non si costruisce ora, ma la
  forma della porta non deve precluderlo»*.

## 7. La chiusura — ⬜ **DA PRESENTARE**

Al Traguardo 3 e al 5 la chiusura è stata un **audit** e non una scrittura, e **gran parte era
già eseguita** (gotcha #49, due volte). Questa sezione deve fissare le **condizioni di
chiusura** e chi le verifica.

---

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

---

## Il prossimo passo

⛔ **Presentare la sezione 6**, poi la 7. Poi il disegno è completo, si scrive il documento
finale, e **solo allora** si invoca `superpowers:writing-plans`.

⚠️ **Non si passa al piano con questo disegno così com'è:** due sezioni su sette non esistono, e
una di esse — lo schema `ipc` — è il compito 4 dell'ordine.
