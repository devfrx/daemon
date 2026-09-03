# Traguardo 5 — l'arbitro GPU: il disegno

- **Data:** 2026-08-18
- **Stato:** disegno approvato, ✅ **piano scritto ed ESEGUITO** — tredici compiti su tredici, chiusi il 2026-08-25. ⚠️ **Richiamo del 2026-08-25:** questa riga diceva *«piano da scrivere»* ed era falsa dal **2026-08-18**, cioè dal giorno stesso in cui il disegno fu approvato — il piano fu scritto poche ore dopo. Nessuna passata l'aveva guardata perché è l'**intestazione**, e l'intestazione non è il contenuto: gotcha **#31**. Il corpo del documento resta com'è: è un **verbale**, e ciò che il traguardo ha davvero prodotto sta nella §6 del [compendio](../../COMPENDIO.md)
- **Sostituisce:** niente. **Precisa:** la [§5 della spec del sotto-progetto 1](2026-08-06-sottoprogetto-1-kernel.md), che descrive l'arbitro **finito**

> 🎯 **A cosa serve questo file.** La §5 della spec dice **cosa** è l'arbitro e con quali
> tipi. Questo documento dice **quanto ne costruisce il Traguardo 5, quali decisioni di forma
> restavano da prendere e quale controllo esercita ciascun artefatto** — cioè ciò che il piano
> dovrà tradurre in compiti.
>
> ⛔ **Non è una spec nuova e non ne apre una.** La §5 resta la fonte, insieme a
> [ADR-0005](../../adr/0005-arbitrato-gpu-su-due-dimensioni.md),
> [ADR-0006](../../adr/0006-due-policy-vram-come-oggetti-distinti.md),
> [ADR-0033](../../adr/0033-gpu-della-gui-quota-di-presentazione.md) e
> [design/02](../../design/02-arbitrato-gpu.md). Qui c'è lo scaglionamento, che la spec
> deliberatamente non fissa, e le forme che la spec descrive a parole.
>
> 📖 **Chi legge questo file:** chi scrive il piano del Traguardo 5, e chi lo esegue.

---

## 0. Il perimetro — cosa consegna il Traguardo 5

### 0.1 La decisione, e cosa l'ha presa

**Il Traguardo 5 costruisce l'arbitro intero, e prova ciò che si può provare senza un secondo
meccanismo.**

Il fatto che decide è stato ottenuto leggendo la §5.7 contro il codice di **oggi**. Delle
cinque proprietà che la DST deve verificare sull'arbitro, **tre** si iniettano su porte che
esistono — `reactor` dal Traguardo 2, `journal` dal Traguardo 4 — e **due** su porte che non
hanno implementazione: `process` e `ipc`, che sono il Traguardo 6.

⛔ **Ma «due non si possono fare» era una formulazione troppo grossa, ed è stata corretta prima
di scrivere questo documento.** Quelle due proprietà sono **due metà incollate**:

| Proprietà della §5.7 | Metà **arbitro** — entra qui | Metà **cablaggio** — Traguardo 6 |
|---|---|---|
| 2 — nessun lavoro attivo senza concessione valida | una concessione **rilasciata torna nel budget** | un worker ucciso **fa rilasciare** la sua concessione |
| 3 — la GUI muore tenendo una concessione discrezionale | *(la stessa metà: rilasciare restituisce la riserva)* | la **disconnessione IPC** è ciò che lo scatena |

📌 **Le due metà d'arbitro sono UNA SOLA, e va detto invece di contarne due.** All'arbitro non
serve sapere **chi** tiene una concessione: gli basta il proprio identificatore. Dargli un
concetto di «titolare» lo accoppierebbe a `ipc` per una cosa che il Traguardo 6 risolve con una
mappa dalla propria parte — e sarebbe un'astrazione costruita per un consumatore che non
esiste, cioè il gotcha **#46** preso dal verso sbagliato.

| Criterio | Cosa dice qui |
|---|---|
| **debiti futuri** | l'alternativa — tirare dentro `process` per chiudere la proprietà 2 — è **un secondo traguardo dentro il primo**: `process` implementata significa progettare il dialogo col worker e il formato sul filo (§6.10.3), che è il Traguardo 6 per intero |
| **coerenza** | il gotcha **#46** ha già scritto il rimedio per una porta senza implementazione, e non è implementarla: è **una finta in un banco**, che dà un chiamante a ciò che serve. La finta esiste, `crates/kernel/tests/ports_are_implementable.rs` |
| **perimetro dichiarato** | la §0.4 classifica l'arbitro **regola A, «tutta: ammissione, corsie, ciclo della concessione, revoca, due policy»**. Restringerlo sarebbe una decisione contro la spec, non una scelta di scaglionamento |
| ⛔ **non pigrizia** | dentro il proprio perimetro questa strada costa **di più** della lettura corta: la metà d'arbitro delle proprietà 2 e 3 è lavoro che «non si può fare oggi» avrebbe cancellato |

⚠️ **Il costo, dichiarato subito:** a fine Traguardo 5 due proprietà della §5.7 restano coperte
**a metà**. È accettabile **solo** perché la [§9](#9-cosa-non-entra-e-dove-va) dà a ciascuna
metà il proprio indirizzo.

### 0.2 Gli artefatti, e il controllo che esercita ciascuno

⛔ **La colonna di destra è obbligatoria, e non è documentazione.** È la seconda domanda del
pre-controllo — *per ogni artefatto che il compito produce, quale controllo lo esercita?* —
posta **a tempo di disegno** invece che a tempo di dispaccio, perché è la sola classe di
difetto che non si vede rileggendo: non c'è niente da leggere.

| # | Artefatto | Vive in | Chi lo esercita |
|---|---|---|---|
| **1** | `Mib` — la VRAM come tipo proprio, aritmetica saturante | `crates/kernel/src/arbiter/` | riga di catalogo **§7.4.1 C, `Q2 · §5.1`**, in **due** regole: non si passa per un `Millis`, e non esiste via `From` |
| **2** | `ComputeClass` — le tre corsie, con ordine **esplicito** | idem | una sonda che fissa l'ordine **per nome**; l'ordine non è derivato, quindi riordinare le varianti non lo muove |
| **3** | `Preemption` — `Never` \| `After(Millis)` | idem | il tipo stesso: un profilo non prelazionabile **non può portare** un tempo di grazia |
| **4** | `ResourceProfile` — ciò che l'arbitro riceve | idem | riga **§7.4.1 C, `V2`** — un'ammissione senza profilo non compila |
| **5** | `WorkDescriptor` — dove vive `cold_start` | idem | riga **§7.4.1 C, `Q8 · §5.2.1`**, in due direzioni: l'ammissione non lo raggiunge, un lettore fuori dall'ammissione sì |
| **6** | `Grant` — **spostato qui da `ports/process.rs`**, costruttore unico e privato | idem | riga **§7.4.1 B**, *avviare un worker ← una concessione*; e un caso `compile_fail` da fuori la crate |
| **7** | `Admission` — l'esito a tre vie | idem | riga **§7.4.1 C, `V4`**; il caso negativo nomina un `is_granted()` che non esiste — forma forte del gotcha **#42** |
| **8** | `Arbiter` — ammissione, code per corsia, rilascio, revoca, riscossione delle scadute | idem | la **campagna DST**, riga §7.4.2, con la sonda di §5.7.1 — *si rompe l'ammissione, la campagna fallisce e nomina il seme* |
| **9** | l'annidamento dello stato — `NonPreemptible` \| `Preemptible(…)` | idem | riga **§7.4.1 C, `I2 · §5.3`**: `Revoking` non ha **dove** stare su una concessione non prelazionabile |
| **10** | `VramPolicy` più `RemotePolicy` e `LocalPolicy` | idem | riga **§7.4.1 C, `V3`**, più sonde a esempi sulla decisione *si può fare spazio?* |
| **11** | la **transizione di policy** come passo giornalato | idem, e il giornale è una porta | la campagna di livello 1 col **giornale cadente** del Traguardo 4 |
| **12** | `total_vram` in `Parameters` | `crates/kernel/src/parameters.rs` | riga **§7.4.1 C, `V29 · §2.8 · ADR-0034`**, già in esercizio |
| **13** | il cablaggio in `daemon`: arbitro, giornale, **due concessioni permanenti** all'avvio | `crates/daemon/src/main.rs` | il test che già chiama `run_the_production_graph`, esteso |
| **14** | i **quattro casi negativi** di §6.10.5, righe 1–4 | `crates/kernel/tests/compile_fail/` e i banchi | erano **scaglionati** per assenza di `Grant`; il numero 6 li sblocca |

### 0.3 Definizione di «fatto»

| # | Condizione |
|---|---|
| 1 | `bash scripts/gate.sh` → `GATE GREEN` |
| 2 | le **tre** proprietà DST della §8 girano, ciascuna con la propria **contro-sonda** |
| 3 | la sonda di non-vacuità di §5.7.1 è **eseguita e registrata**: rotta l'ammissione, la campagna fallisce e **nomina il seme**; ripristinata, torna verde |
| 4 | le **dodici** righe di catalogo della §8 sono chiuse o dichiarate, nessuna a metà |
| 5 | `crates/kernel/tests/compile_fail/` porta i casi nuovi, ciascuno col proprio `.stderr` **letto** e non rigenerato in blocco |
| 6 | il registro [`porta-di-qualita.md`](../../porta-di-qualita.md) è riallineato, coi conteggi **ricontati** e non dedotti |
| 7 | ogni riga scoperta della §9 ha il proprio indirizzo scritto |

---

## 1. Cosa esiste già, misurato e non supposto

Misurato il 2026-08-18 sul codice a `3338808`.

| | |
|---|---|
| `Grant` **esiste** | `crates/kernel/src/ports/process.rs`, `pub struct Grant(());` — **senza costruttore pubblico, deliberatamente**. Il commento accanto dice che l'emittente arriva al Traguardo 5 |
| quindi `Process::start` è **implementabile e non chiamabile** | da nessuno, oggi. È la ragione per cui le righe 1–4 di §6.10.5 sono registrate come scaglionate |
| `Monotonic` **nomina già** l'arbitro | `crates/kernel/src/time.rs`: *«deadlines, **grant validity windows**, grace, timeouts»*. Il tipo è stato scritto guardando questo traguardo |
| `Parameters` porta **un solo campo** | `executor_turn_limit`. Aggiungerne uno rompe ogni chiamante, ed è attrito voluto — §2.8.5 |
| `RecordV1` ha l'**indice 5 libero** | misurato al Traguardo 3: un `Option` al 5 lascia i byte congelati identici con `None` e li allunga di uno con `Some` |
| `daemon` **non cabla nessun giornale** | `crates/daemon/src/main.rs` monta `SequentialRng`, `SystemReactor`, `Parameters` e `Sleep`. È il finding **A-7** |
| il **giornale cadente** esiste | `crates/simulator/src/journal.rs`, dal Traguardo 4 |
| l'arbitro **non esiste in nessuna forma** | nessun modulo, nessun tipo, nessuna riga. Il prototipo di **M-7** è uno spike, fuori dal workspace |

---

## 2. Dove vive l'arbitro

**L'arbitro è logica, non una porta.** Non ha un'implementazione vera e una finta: ce n'è una
sola, e in simulazione gira quella.

| | |
|---|---|
| **Decisione** | un **modulo nuovo di `kernel`**, accanto a `executor`, `reconcile` e `boundary`. **Non** una settima famiglia di porte |
| **Perché** | `crates/kernel/src/ports/mod.rs` dichiara le sei famiglie **esaustive**, e §3.1 pure. Una settima sarebbe una decisione strutturale che nessun ADR ha preso |
| **Conseguenza 1** | l'arbitro **non ha una suite di conformità**: una suite pretende due implementazioni da confrontare, e qui ce n'è una. Gotcha **#44** |
| **Conseguenza 2** | `simulator` **non guadagna una finta d'arbitro**. In campagna gira l'arbitro vero, ed è ciò che rende la campagna una prova sul prodotto invece che sulla sua imitazione — ADR-0020, ADR-0026 |

⛔ **La forma da non costruire, scritta perché è la tentazione naturale:** un tratto `Arbiter`
con due implementazioni «per poter iniettare guasti». I guasti si iniettano **dalle porte che
l'arbitro usa** — `reactor` e `journal` — non dentro di lui. Un tratto lì sarebbe un'astrazione
senza secondo implementatore.

---

## 3. `Grant`, e chi lo emette

Il Traguardo 5 deve dare a `Grant` un emittente. C'è un ostacolo, **misurato** e non dedotto.

In Rust un campo privato è visibile dal modulo che lo dichiara **e dai suoi figli**. Il modulo
`arbiter` sarebbe un **fratello** di `ports::process`. Riprodotto su una crate usa-e-getta
nello scratchpad:

```
error[E0423]: cannot initialize a tuple struct which contains private fields
```

È lo stesso errore che il commento di `Grant` cita per chi sta fuori dalla crate. **L'arbitro,
com'è messo il codice oggi, non può emettere la cosa che deve emettere.**

| | Uscita | Verdetto |
|---|---|---|
| **A** | rendere il campo `pub` | ⛔ **cade subito**: distrugge la sola cosa per cui il tipo esiste |
| **B** | un costruttore `pub(crate)` in `ports/process.rs` | ⚠️ regge, e **mente sulla proprietà** |
| **C** | **spostare `Grant` nel modulo `arbiter`** | ✅ **scelta** |

**Perché C e non B.** B costa una riga e lascia il tipo in casa di chi lo **consuma** invece
che di chi lo **emette**. Peggio: apre una via `pub(crate)`, cioè *chiunque dentro `kernel` può
fabbricare una concessione senza passare dall'ammissione*. Oggi lo farebbe un modulo solo;
domani non si sa, e nulla diventerebbe rosso. È la forma del gotcha **#67** — *una guardia vale
esattamente quanto il suo costruttore*.

Con **C** la regola è vera per costruzione: `Grant` ha **un** costruttore, privato nel modulo
dell'arbitro, e l'unica via d'uscita è l'ammissione.

| Costo di C | |
|---|---|
| un **rinomino** nei riferimenti | oggi i siti sono pochi — `ports/process.rs` e `ports_are_implementable.rs` — e `platform` non implementa ancora `Process`. Il costo cresce se si aspetta |
| `ports/process.rs` **importa** dal modulo `arbiter` | logica→logica dentro la stessa crate: non attraversa nessun confine di ADR-0031 |
| il commento di `Grant` va **riscritto col proprio richiamo datato** | dice *«§5.6, che arriva al Traguardo 5»* al futuro. Lasciarlo sarebbe il finding **A-2** rifatto |

⛔ **E non è la «feature di test» già scartata.** [`porta-di-qualita.md`](../../porta-di-qualita.md)
aveva valutato e respinto un costruttore di `Grant` dietro una feature di test, perché
*«creerebbe il secondo modo di ottenere una concessione»*. Qui non nasce un secondo modo: nasce
**il primo e unico**, ed è quello che §5.6 prescrive. Il banco passa dall'ammissione come tutti.

---

## 4. I parametri consegnati, e una divergenza dichiarata

I documenti scrivono la formula così:

```
budget allocabile = totale − quota audio − quota presentazione
```

e la §5.1 aggiunge: *«i tre addendi sono parametri consegnati»*.

### 4.1 Il difetto di prendere quella riga alla lettera

| Se l'arbitro **sottrae**… | …e le due quote **hanno** un titolare | la memoria è contata **due volte** |
|---|---|---|
| | …e le due quote **non** hanno un titolare | **I2 è falso** per quei due consumatori |

La seconda è precisamente il difetto che ADR-0033 esiste per chiudere — *«una quota sottratta
senza titolare lascia I2 falso»* — e ADR-0005 lo dice in una riga: *«la sottrazione **non è
un'esenzione»*, gotcha **#4**.

### 4.2 Cosa dicono davvero gli ADR

| Fonte | Parola |
|---|---|
| ADR-0033 §2 | *«Il core **richiede** all'avvio una **concessione** di presentazione permanente e non prelazionabile»* |
| design/02 | *«Il worker audio **detiene una concessione permanente e non prelazionabile** sulla quota riservata»* |

📌 **Le due quote non sono sottrazioni: sono due concessioni.** La formula **descrive
l'effetto**, non detta l'implementazione.

### 4.3 La decisione

| | |
|---|---|
| `Parameters` guadagna **un** campo: `total_vram: Mib` | è l'unico dei tre che una **decisione del kernel** legge — l'ammissione confronta la somma contro di lui |
| le due quote **non sono campi dell'arbitro** | sono la riserva di due profili che la **radice di composizione** dichiara: `daemon` in produzione, il **banco** in simulazione |
| all'avvio si chiedono **due concessioni permanenti** | l'arbitro non sa che si chiamano «audio» e «presentazione»: vede due concessioni permanenti come tutte le altre |

| Cosa ci guadagna il disegno | |
|---|---|
| **un'invariante sola invece di due** | *«la somma di tutte le concessioni non supera mai il totale»*, e non *«la somma delle ordinarie ≤ totale meno due quote»* |
| **I2 vero per costruzione** sui due consumatori | hanno un titolare perché **sono** concessioni |
| l'arbitro resta **cieco alle capacità** | ADR-0001: nessuna capacità ha accesso privilegiato. Cablare «audio» e «presentazione» dentro l'arbitro sarebbero due casi speciali in un meccanismo che deve essere paritario |
| la campagna può **variare le quote** senza toccare il kernel | in simulazione i profili li sceglie il banco: nessuna costante invisibile, gotcha **#28** |
| una configurazione impossibile **si vede** | se `total_vram` è più piccolo delle due quote, la seconda richiesta torna `Refused`. La forma a sottrazione avrebbe dato budget zero **in silenzio** |

⚠️ **La divergenza, registrata perché il proprietario possa ribaltarla vedendola.** La §5.1 dice
*«i tre addendi sono parametri consegnati»*, e qui se ne consegna **uno**. Lo spirito è
rispettato — l'arbitro non va a prendere nulla — la lettera no. Metterli tutti e tre in
`Parameters` produrrebbe due campi che **nessuna decisione del kernel legge**, cioè superficie
morta dentro il kernel: la stessa ragione per cui `Record::encode` ha perso il proprio `Result`.

---

## 5. Il modello della risorsa

| Asse | Rappresentazione | Perché |
|---|---|---|
| **VRAM** | `Mib(u64)` — tipo proprio, non un intero nudo | scambiare MiB con millisecondi **non deve compilare**. Stesso meccanismo di `Instruction`/`Untrusted` e di `Monotonic`/`WallTime` |
| **calcolo** | `ComputeClass` — **tre corsie ordinate**, non un numero | §5.1 |

**Aritmetica di `Mib`: saturante, e la direzione è quella giusta.** Un traboccamento nella somma
satura a `u64::MAX`, che è **maggiore** del totale: la richiesta viene **rifiutata**. Un
traboccamento che avvolge darebbe un numero più piccolo e produrrebbe sovra-ammissione, cioè Q2
che cede in silenzio. È la stessa forma della ragione già scritta accanto a
`Monotonic::saturating_add`.

⛔ **L'ordine delle corsie è esplicito, non derivato.** `Ord` derivato segue l'ordine di
**dichiarazione**: riordinare le varianti cambierebbe le priorità senza che nulla diventi rosso.
L'ordine vive in una chiave scritta, e una sonda lo fissa **per nome**. Togliere la trappola
batte sorvegliarla.

### 5.1 Il profilo di risorsa

| Campo | Tipo | Nota |
|---|---|---|
| `name` | `&'static str` | nominato e versionato. **Non `String`, e la ragione è P-1**: un nome scelto in fase di scrittura è un letterale nel binario, e testo di runtime lì non arriva |
| `reserved_vram` | `Mib` | la riserva **dichiarata** dal richiedente |
| `compute_class` | `ComputeClass` | la corsia |
| `preemption` | `Preemption` | ⬇️ |

⛔ **`preemptible` non è un booleano.** La §5.3 chiede che `InRevoca` sia **non
rappresentabile** per un profilo non prelazionabile — *«non costruibile»*, non «controllato a
runtime».

```rust
pub enum Preemption {
    Never,
    After(Millis),   // il tempo di grazia vive QUI dentro
}
```

Due stati illegali spariscono insieme: un profilo non prelazionabile **non può portare** un
tempo di grazia, e uno prelazionabile **non può esserne privo**.

E lo stato della concessione **annida** invece di appiattire:

```rust
enum Activity {
    NonPreemptible,                  // non ha DOVE mettere una revoca
    Preemptible(PreemptibleState),   // Running | Revoking { deadline }
}
```

⚠️ **`NonPreemptible` e non `Permanent`, e la differenza è reale:** un lavoro che non si può
interrompere finisce lo stesso e rilascia. La permanenza non è un tipo — è «nessuno chiama
rilascia».

### 5.2 `cold_start` vive fuori dall'ammissione

§5.2.1 lo vuole irraggiungibile dal percorso decisionale. Vive in `WorkDescriptor`, che
l'ammissione **non riceve**.

⚠️ **Questa decisione è stata presa due volte, e la prima era sbagliata.** La prima lettura era
*«non si costruisce, perché non ha consumatore»*. Il catalogo la smentisce: la riga
**`Q8 · §5.2.1`** ha come contro-sonda *«la proiezione di presentazione lo legge»*, e senza il
campo quella contro-sonda **non è scrivibile** — una riga provata in una direzione sola non è
ammissibile, §7.1.1 regola 3. La §5.2.1 il costo lo aveva già messo in conto: *«due strutture
invece di una»*.

📌 **Le strutture dell'arbitro sono `BTreeMap` e `Vec`.** Non è preferenza: `HashMap` vive in
`std`, che `kernel` non nomina, quindi il gotcha **#12** qui è chiuso dal compilatore e gratis.

---

## 6. Il ciclo della concessione

### 6.1 L'esito è a tre vie, e il compilatore lo impone

```rust
#[must_use]
pub enum Admission {
    Granted(Grant),
    Queued(TicketId),
    Refused { asked: Mib, ceiling: Mib },
}
```

⛔ **Non esistono `is_ok()`, `is_granted()`, né una conversione a booleano.** È così che `V4`
diventa una firma invece di una raccomandazione. Il caso negativo nomina un metodo che non
c'è — e se un giorno qualcuno lo aggiunge, il caso **inizia a compilare** e `trybuild` lo
riporta come `error` invece che come `mismatch`: forma forte del gotcha **#42**, che una
rigenerazione in blocco non disarma.

**`Refused` porta due numeri e non una frase.** design/02 vuole *«perché non entra, e
l'alternativa praticabile»*: l'alternativa la costruisce l'interfaccia, il kernel le dà il
materiale. Suggerire un profilo alternativo sarebbe logica L2 dentro il kernel.

### 6.2 L'arbitro non legge l'orologio

Ogni operazione che ha bisogno del tempo prende `now: Monotonic` **come argomento**.

| | |
|---|---|
| **la forma** | è quella di ADR-0034 — *consegnato, non letto*. L'arbitro è una funzione pura dei propri ingressi |
| **e una ragione meccanica** | `wait_until` è `&mut self`. Se l'arbitro tenesse un reattore, il reattore avrebbe **due proprietari** — lui e l'esecutore — e il prestito non passerebbe |
| **conseguenza** | in campagna il tempo lo governa il seme attraverso l'esecutore. Nessuna seconda sorgente di tempo virtuale |

⚠️ Su **`Monotonic`, mai `WallTime`**: un orologio che torna indietro non può scadere una
concessione. Il tipo lo impone già, con quattro casi `compile_fail` che esistono dal Traguardo 2.

### 6.3 La scadenza si riscuote quando qualcuno guarda

Una concessione `Granted` porta una scadenza. Prima di decidere, l'arbitro **riscuote** le
scadute.

⚠️ **Il limite, dichiarato:** fra due operazioni una concessione scaduta resta nei conti. Non
nega niente a nessuno — non c'è nessuno — e al primo che guarda è già liberata. **La proprietà
vale in ogni punto in cui è osservabile**, e la sonda va scritta così: *avanza l'orologio oltre
la scadenza, poi chiedi → concede*. La contro-sonda: *senza la riscossione, rifiuta*.

### 6.4 La coda è per corsia

§5.3.1 dice perché conta: la versione con l'ordine **per corsia** è quella che rende i numeri di
**M-7** un limite superiore ancora valido, quindi non si rimisurano. Una coda unica riordinata a
ogni rilascio invaliderebbe quella misura.

### 6.5 `Forzata` si ferma al confine

| Metà | Traguardo |
|---|---|
| la concessione torna nel budget alla scadenza della grazia | **5** |
| il processo viene davvero ucciso | **6**, con `process` |

### 6.6 Cosa l'arbitro non giornala qui

⬜ **Le concessioni non entrano nel giornale al Traguardo 5**, e non è pigrizia: §5.2.2 dice che
ciò che si giornala al rilascio è il **picco misurato**, e che *«il numero lo misura il worker e
risale dalla porta `process`»*. Nessun worker, nessun picco, niente da scrivere. Il campo nascerà
sotto la regola di §4.9 — facoltativo, indice nuovo, e l'**indice 5 è libero**, misurato.

---

## 7. Le due policy

### 7.1 Hanno qualcosa da fare, e sta dentro l'ammissione

Al Traguardo 5 non esistono modelli da sfrattare. La domanda era se le due policy fossero due
gusci vuoti. Non lo sono, e la differenza è **una decisione dentro il percorso di ammissione**:

> **Una richiesta non entra. Si può fare spazio?**

| Policy | Risposta |
|---|---|
| **REMOTA** *(default)* | **no**. Si accoda, o si rifiuta. Non si revoca niente per far posto |
| **LOCALE** | **sì**. Si revocano le concessioni prelazionabili delle corsie più basse finché entra |

📌 **E qui non serve nessun modello.** «Sfrattare un residente» **è** «revocare una concessione
prelazionabile» — un meccanismo che la §6 costruisce comunque. Le due policy si provano oggi con
concessioni sintetiche dichiarate dal banco: zero speculazione.

📌 **Ed è esattamente il punto in cui ADR-0006 diceva che sarebbe finito il condizionale.** Un
`if` sull'origine dell'inferenza piantato nel cuore dell'ammissione è la deriva invisibile che
quell'ADR rifiuta; due oggetti con la stessa interfaccia lo tengono in un posto solo.

### 7.2 Una sola attiva, e non è controllato: è il tipo

Il valore consegnato ne porta **una**. «Due policy attive» non è **esprimibile** — livello 1, ed
è ciò che la riga di catalogo di `V3` pretende.

### 7.3 La transizione è un passo giornalato

⛔ Cambiare policy ha effetti veri sul mondo — sfratti, ricariche — e **V6** dice che nulla si
esegue prima che l'intento sia durevole. Intento → effetti → esito. Una transizione interrotta a
metà lascia un passo **in dubbio**, riconciliabile come tutti gli altri (§4.3).

✅ È la **proprietà DST numero 4, provabile oggi**: il giornale cadente esiste dal Traguardo 4,
la riconciliazione dal Traguardo 3.

⚠️ **Conseguenza da dire:** l'arbitro guadagna così una dipendenza dal giornale, e
`crates/daemon/src/main.rs` oggi **non ne cabla nessuno** — finding **A-7**. Il Traguardo 5
glielo deve dare.

### 7.4 Cosa resta fuori

⬜ Il **contenuto** dello sfratto — quali modelli, in che ordine, la ricarica con avvio a freddo
visibile — è L2, e §0.2 lo esclude esplicitamente. Il Traguardo 5 costruisce **chi decide se
fare spazio**, non **cosa c'è da sfrattare**.

---

## 8. Cosa prova cosa — dodici righe di catalogo

⛔ **La notizia di questa sezione: il Traguardo 5 non crea righe di catalogo, ne chiude di già
scritte.** Contate sul catalogo §7.4 e non dedotte.

### 8.1 Livello 1 — blocco B, i gettoni

| Riga | Oggi | Col Traguardo 5 |
|---|---|---|
| avviare un worker ← una **concessione** | l'arbitro non esiste | ✅ **chiusa** |
| **parlare** a un worker ← l'oggetto `Worker` | scoperta: senza `Grant` non si ottiene un `Worker` | ✅ **sbloccata** |
| **leggere** ← una **ricevuta** | idem | ✅ **sbloccata** |

### 8.2 Livello 1 — blocco C, cosa non è esprimibile

| Difende | Il divieto | Come lo tiene |
|---|---|---|
| `Q2 · §5.1` | MiB assegnati a millisecondi | `Mib`, in **due** regole: non si passa l'uno per l'altro, e non esiste via `From`. La seconda nella forma forte del #42 |
| `V2` | un'ammissione **senza profilo** | il profilo è un argomento, non un `Option` |
| `V4` | l'esito trattato come **due vie** | `Admission` a tre varianti, e **nessun** `is_granted()` |
| `I2 · §5.3` | `InRevoca` per un profilo **non prelazionabile** | l'annidamento: `NonPreemptible` non ha dove metterlo |
| `Q8 · §5.2.1` | l'**ammissione legge `cold_start`** | il campo vive in `WorkDescriptor`, che l'ammissione non riceve |
| `V3` | una **seconda policy attiva** | il valore consegnato ne porta una |
| `I2 · §6.10` | istruire un worker **dopo `uccidi`** | ✅ **sbloccata** dal `Grant` |
| `I5 · §6.10` | leggere **due volte** dalla stessa ricevuta | ✅ **sbloccata** dal `Grant` |

### 8.3 Livello 2 — la campagna

La riga del catalogo esiste già e nomina `Q2 · I2 · V1` fra i propri difesi, con la sonda
scritta: *si rompe l'ammissione, la campagna fallisce e **nomina il seme***.

| # | Proprietà | Si inietta su | Contro-sonda |
|---|---|---|---|
| 1 | la somma di **tutte** le concessioni non supera mai il totale | `reactor` — interlacciamento | rotta l'ammissione, la campagna **fallisce e nomina il seme** |
| 4 | una **transizione di policy** interrotta lascia un passo **in dubbio** | `journal` — il cadente del Traguardo 4 | senza caduta, **nessun** passo in dubbio |
| 5 | una concessione **scaduta** non resta allocata | `reactor` — orologio virtuale | senza la riscossione, la stessa richiesta viene **rifiutata** |

⛔ **La non-vacuità qui è obbligatoria, non facoltativa** — §5.7.1. È la lezione che il
Traguardo 4 ha imparato **tre volte**: *«l'iniezione è avvenuta»* e *«c'era qualcosa da
verificare»* sono **due** affermazioni, e una campagna che tiene solo la prima è verde avendo
confrontato insiemi vuoti.

### 8.4 Le sonde a esempi

| Cosa | Perché non basta il compilatore |
|---|---|
| l'**ordine** delle corsie, per nome | la chiave è esplicita apposta; la sonda la fissa |
| **rilasciare** restituisce esattamente la riserva | è la metà d'arbitro delle proprietà 2 e 3 |
| la coda promuove **per corsia**, non FIFO globale | è ciò che tiene validi i numeri di M-7 |
| un `total_vram` più piccolo delle due quote → la seconda è **`Refused`** | la configurazione impossibile si vede invece di dare budget zero in silenzio |

---

## 9. Cosa non entra, e dove va

⛔ **Ogni riga ha un indirizzo.** Un arretrato anonimo è ciò che questa tabella esiste per non
lasciare.

| Non entra | Perché | Va a |
|---|---|---|
| chi **uccide davvero** il processo a grazia scaduta | serve `process` implementata | **Traguardo 6** |
| chi **rilascia** la concessione quando la GUI si disconnette | serve `ipc` implementata | **Traguardo 6** |
| il **picco misurato** nel giornale | lo misura il worker e risale da `process` (§5.2.2) | **Traguardo 6** |
| la riga 5 di §6.10.5 — byte consumati pari alla lunghezza dichiarata | è il formato sul filo | **Traguardo 6** |
| il **contenuto** dello sfratto | §0.2 esclude adattatori e worker | **L2** |
| la **taratura** dei profili reali | SP-1, SP-2: sono parametri, non impianto | **spike** |
| **M5**, il valore della quota di presentazione | richiede una GUI | **sotto-progetto 2** |
| la riga di guasto `Q22`, caduta durante la conservazione di un file | serve ambiti e checkpoint, e non è l'arbitro | **Traguardo 5/6** |

---

## 10. I costi accettati

| Costo | |
|---|---|
| **`Parameters::new` cambia firma e rompe ogni chiamante** | attrito voluto, §2.8.5: è ciò che impedisce a un parametro di rientrare come costante |
| **`Grant` si sposta di modulo** | un rinomino. Oggi i siti sono pochi e `platform` non implementa ancora `Process`; il costo cresce se si aspetta |
| **due strutture per il profilo invece di una** | il prezzo di rendere `cold_start` irraggiungibile. §5.2.1 lo aveva già messo in conto |
| **`daemon` deve cablare un giornale** | oggi non ne cabla nessuno, finding **A-7**. La transizione di policy è un passo giornalato |
| **la scadenza si riscuote pigramente** | fra due operazioni una scaduta resta nei conti. Dichiarato, e la proprietà è scritta per essere osservabile |
| **la contesa di calcolo resta indiretta** | ADR-0005 lo dichiarava già; questo traguardo non la migliora, e **SP-2** resta la sua verifica |
| **due proprietà della §5.7 restano coperte a metà** | mitigato dalla §9: ogni metà ha il proprio indirizzo |

---

## 11. Le misure di questo documento

Eseguite il **2026-08-18** · Windows 11 · toolchain `1.95.0` appuntata da `rust-toolchain.toml`
· repository a `3338808`, albero pulito.

| # | Domanda | Esito |
|---|---|---|
| **D5-1** | un modulo **fratello** può costruire `Grant`? | ⛔ **no** — `error[E0423]`, riprodotto su una crate usa-e-getta nello scratchpad, poi cancellata. È il fatto che decide la §3 |
| **D5-2** | quante righe di guasto della §3.3 eredita davvero il Traguardo 5? | **una**, più una condivisa. Contate sulla §7 del [disegno del Traguardo 4](2026-08-11-sottoprogetto-1-traguardo-4-simulatore-dst-design.md), che è la tabella che gli indirizzi li assegna: **1 · 1 · 7** |
| **D5-3** | quante righe di catalogo il Traguardo 5 **crea**? | **zero**. Ne chiude **dodici** già scritte — tre nel blocco B, otto nel blocco C, una di livello 2 |
| **D5-4** | la baseline prima di cominciare | `GATE GREEN` · `cargo test --workspace --no-fail-fast --locked` → **32 target, 194 passati, 0 falliti, 2 ignorati** · albero pulito |

---

## 12. Cosa questo documento lascia aperto

| | Chi la chiude |
|---|---|
| ⚠️ **`Grant` ha un solo costruttore, e nulla lo controlla da dentro la crate.** `trybuild` compila i casi come crate separate: prova la direzione **da fuori**, non che domani nessuno aggiunga un `pub(crate)` accanto | il **proprietario** — sarebbe una riga di catalogo nuova, vincolo globale 7. **Registrata, non presa** |
| ⚠️ **La contro-sonda di `Q8` dice *«la proiezione di presentazione lo legge»***, e la proiezione non esiste: a leggerlo sarà una **finta**. Prova la proprietà giusta — il campo è raggiungibile fuori dall'ammissione — con parole diverse da quelle della riga | il **proprietario**, se vuole riformulare la cella |
| ⛔ **La cifra «cinque delle nove righe di guasto»** vive sbagliata in [`COMPENDIO.md`](../../COMPENDIO.md) e [`roadmap.md`](../../roadmap.md); il vero è **una**, più una condivisa. Nasce da una tabella diversa: la §5.7 ne ha esattamente **cinque**, ma sono le proprietà che la DST verifica, non le righe di guasto | il **proprietario**: `CLAUDE.md` prescrive di **toglierla**, non di ricorreggerla — una cifra che vive in più documenti si toglie |
| ⚠️ **La divergenza su §5.1** — un parametro consegnato invece di tre | il **proprietario**, ribaltabile vedendola |
