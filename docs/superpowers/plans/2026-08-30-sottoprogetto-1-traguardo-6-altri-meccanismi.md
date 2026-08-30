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

## ▶️ A che punto è QUESTO PIANO — casa unica, e si aggiorna scrivendo

⛔ **IL PIANO NON È FINITO. Non si esegue finché non lo è**, e il perché è la §7.1 del
disegno: al Traguardo 5 il disegno aveva **dimenticato una condizione di chiusura** e a
rimediare fu chi scriveva il piano. Un piano eseguito a metà scrittura non ha nessuno che
faccia quel controllo.

| Parte | Compiti | Stato |
|---|---|---|
| **A** — la concessione che torna | 1 | ✅ **scritta** il 2026-08-30 |
| **B** — il filo | 3, 3bis | ✅ **scritta** il 2026-08-30 |
| **C** — lo schema `ipc` | 4 | ✅ **scritta** il 2026-08-30 |
| **D** — i meccanismi | 5, 6, 7, 8 | ⛔ **SBARRATA il 2026-08-30 — P-11**, e aspetta il proprietario |
| **E** — la prova e la chiusura | 9, 10 | ⬜ da scrivere |

⛔ **Perché la D è sbarrata, in tre righe.** I compiti **5**, **6** e **7** devono mettere dati
**strutturati e nostri** dentro un record durevole, e `RecordV1` non ha una casella per farlo:
le sue due caselle di contenuto sono assegnate da una decisione scritta — `payload` è *«di
qualcun altro»*, `reason` è *«testo nostro»*. Le forme coerenti sono **due**, il disegno ne ha
scelta una alla §4.3 senza discutere l'altra, e il formato durevole è **l'unico artefatto
irreversibile del progetto**. ⛔ **Non si delega a un compito**, e non si sceglie scrivendo il
piano: il dettaglio, con le misure delle due forme, è in **P-11**.

⚠️ **Il compito 2 non c'è, e non è un buco:** il timbro di build è **uscito** dal perimetro
alla §3.4 del disegno e diventa una non-costruzione dichiarata. La numerazione della §1.4 è
tenuta com'è invece di essere compattata, perché il disegno vi rimanda per numero.

⛔ **Ciò che chi riprende deve sapere sul compito 3bis:** è la **misura C-1**, e pretende una
ricerca **odierna** — *`bincode` è ancora dichiarato non mantenuto? esiste un'alternativa
mantenuta il cui pari TypeScript abbia un lettore?* La §3.5 del disegno dice alla lettera che
deciderlo a memoria sarebbe il gotcha **#48**. La fonte va tracciata in
[`riferimenti.md`](../../riferimenti.md) con la data.
⚠️ **RICHIAMO DEL 2026-08-30:** questa riga diceva *«prima di scriverlo»*, ed è diventata falsa
lo stesso giorno — il compito **è scritto**. Resta perché ciò che dice non è *quando* si scrive
ma *che cosa pretende*, e chi lo **esegue** ne ha lo stesso bisogno di chi lo scriveva. La
disciplina per esteso sta nel compito, non qui.

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

### P-5 — ⛔ La §11 del compendio prezza il vincolo 15 con un comando che NON riproduce la sua stessa clausola

Fatto scrivendo la Parte B, il 2026-08-30. La riga del **vincolo 15** della §11 di
[`COMPENDIO.md`](../../COMPENDIO.md) — che è la riga che il **compito 3 chiude** — scrive:

> *«`grep -rn minicbor crates/kernel/src/` lo trova **solo** in `record.rs`, cioè sul
> **giornale**»*

**Rilanciato: rende DUE file, non uno.**

```bash
grep -rn minicbor crates/kernel/src/ | cut -d: -f1 | sort -u
```

| File | Che cosa è |
|---|---|
| `crates/kernel/src/record.rs` | il codificatore vero — **tredici** riscontri |
| `crates/kernel/src/ports/process.rs` | **un doc di modulo**, che dichiara la non-costruzione: *«NOT the wire format (§6.10.3: `minicbor`, the port exchanges BYTES …)»* |

⛔ **La sostanza regge, la clausola no, e la differenza è la parte che conta.** L'unico uso
**come codice** è `record.rs`; ma la clausola è scritta **accanto al comando**, e il comando la
smentisce — che è precisamente ciò contro cui il rimedio di **AUD-007** aveva sostituito un
numerale con un comando.

⚠️ **Ed era falsa dalla nascita**, misurato e non dedotto: la riga di `process.rs` arriva da
`ff41eea`, cioè dal **Traguardo 2**, mentre la riga della §11 è stata scritta il 2026-08-27 da
`ebbbdac`. È il gotcha **#70** — *ogni riga che il censimento restituisce si legge intera* —
commesso dentro il rimedio che il gotcha **#68** prescriveva. Il disegno di questo traguardo la
domanda se l'era invece posta: la §1.2 dichiara per esteso che i riscontri di `Permission` e
`degrad` *«sono prosa nei commenti, guardati uno per uno»*.

📌 **NON si corregge adesso, e la ragione è il perimetro:** il **compito 3 onora il vincolo 15**,
quindi quella riga esce comunque dalla tabella *«cosa resta davanti»* alla chiusura. Riscriverla
ora significherebbe scriverla due volte. ⛔ **Entra invece nel criterio di chiusura del compito
3**, perché una riga che sparisce non è una riga corretta.

### P-6 — ⛔ Il disegno non dice DOVE vive lo schema del canale worker, e la mappa dei file non ha un posto per esso

La §3 del disegno decide la **meccanica** — l'inquadratura, il riuso della forma di `record.rs`,
`Frame` che resta opaco — e **nessuna sua riga dice dove il corpo del frame sia definito**. Ma la
condizione **6** della Definizione di «fatto» pretende che *«l'annotazione di stringa di byte sia
**sul canale worker**»*, e un'annotazione ha bisogno di un **campo**, cioè di un tipo.

**Misurato il 2026-08-30:**

```bash
ls crates/kernel/src/            # nessun framing.rs, nessun wire/
```

⛔ **E la mappa dei file di questo piano assegna `crates/kernel/src/wire/mod.rs` al compito 4.**
Il compito 3 arriva prima e ne ha bisogno: come è scritta, la mappa gli lascia il tipo senza una
casa.

✅ **Chiuso dalla decisione D8**, che non inventa un posto ma applica il precedente che il disegno
stesso cita: §6.10.3 dice *«la porta scambia byte, non messaggi tipizzati, **come `journal` dopo
ADR-0036**»*, e dopo ADR-0036 lo schema del giornale vive in `crates/kernel/src/record.rs`, fuori
dalla porta. La mappa è **corretta col richiamo datato**, non riscritta in silenzio.

### P-7 — ⚠️ Il doc di modulo di `ports/process.rs` diventa falso col commit 3b, e il compito deve saperlo prima

`crates/kernel/src/ports/process.rs`, doc di modulo, sotto *«What milestone 2 builds, and what it
does not»*:

> *«NOT the implementation (milestone 6), **NOT the wire format** (§6.10.3: `minicbor`, the port
> exchanges BYTES, every frame declares its own length and decoding checks the bytes consumed),
> and NOT the negative tests of §6.10.5 rows 1-4»*

Delle tre clausole, la terza è **già** falsa dal Task 11 del Traguardo 5 — e infatti quel paragrafo
porta già il proprio **richiamo del 2026-08-21**. La **seconda** diventa falsa col commit **3b**.

📌 **Sta qui e non solo nel compito perché è il passo 5 della disciplina dell'audit** —
*ricontà ciò che il tuo rimedio ha reso stantio* — e perché la casa del paragrafo è un file che
il compito 3 non toccherebbe altrimenti: con questa forma dello schema (D8) il `Frame` **non
cambia**, quindi nulla obbligherebbe a passare di lì. È esattamente il modo in cui una frase
sopravvive al fatto che la smentisce.

### P-8 — ⛔ `Admission` non è un tipo di filo, e metterlo sul filo conierebbe concessioni dai byte

La §6.2 del disegno mette in tabella *«core → gui: **esito dell'ammissione**, a tre vie — in
codice `Admission`»*. **Preso alla lettera non è implementabile**, e le ragioni misurate sono
tre, indipendenti:

| | Misurato in `crates/kernel/src/arbiter/mod.rs` |
|---|---|
| 1 | `Admission::Granted(Grant)`, e `Grant { id: GrantId }` con `GrantId` **privato** — un tipo che non esce dal modulo |
| 2 | `Grant` **non porta nessun derive**, e `Admission` **non deriva `Debug` né `PartialEq`** *deliberatamente*, col perché scritto accanto: darglieli significherebbe darli a `Grant` per comodità del banco |
| 3 | ⛔ **la terza è quella che conta:** un `Grant` **decodificabile** è una concessione **coniata dai byte**. §5.6 tiene che l'unico sito che ne conia una sia `Arbiter::issue`, e `crates/kernel/tests/compile_fail/grant_has_no_constructor.rs` esiste per renderlo impronunciabile da fuori. Sarebbe **AUD-050 rifatto sul gettone più forte del progetto** — *una guardia vale quanto il suo costruttore* |

✅ **E la lettura giusta della §6.2 è PIÙ PICCOLA della falsità apparente** — gotcha **#65**
nella direzione che costa meno. Quella cella **identifica il concetto**: l'*«esito a tre vie»*
di [ADR-0033](../../adr/0033-gpu-della-gui-quota-di-presentazione.md) è ciò che il codice chiama
`Admission`. Non prescrive il tipo che viaggia.

📌 **E il filo non ne ha bisogno, per una ragione che ADR-0033 scrive da sé:** la concessione è
**stato del core** (I1). Alla gui serve il **verdetto**, non il gettone — e un verdetto non è un
gettone. Chiuso dalla **D15**.

### P-9 — ⛔ `ResourceProfile` non è decodificabile, e il difetto è UN campo

La §6.2 dice *«richiesta di concessione ordinaria, **col profilo di risorsa dichiarato**»*.
Misurato in `crates/kernel/src/arbiter/resource.rs`:

```rust
pub struct ResourceProfile {
    pub name: &'static str,   // ⛔ questo
    pub reserved_vram: Mib,
    pub compute_class: ComputeClass,
    pub preemption: Preemption,
}
```

⛔ **Un `&'static str` non si produce da byte in arrivo**, se non **leakando** — che è la via
**A3** dichiarata aperta in `crates/kernel/src/boundary.rs` — e ciò che vi finirebbe è **testo
scelto dalla gui**, cioè contenuto **non fidato** (ADR-0014) dentro un campo di un tipo che
l'arbitro usa per **decidere**. Non è un fastidio di lifetime: è I6.

⛔ **E il secondo corno è chiuso pure lui, misurato:** portare il **nome** e risolverlo non è una
via, perché **non esiste nessun registro nome → profilo**.

```bash
grep -rn "ResourceProfile {" crates/ --include=*.rs
```

rende **due costanti in `crates/daemon/src/main.rs`** e **due aiutanti di banco**. Niente mappa
un nome su un profilo, e costruirne una sarebbe un meccanismo che nessuna riga scritta chiede.

✅ **Chiuso dalla D16, e il rimedio RINFORZA il confine invece di indebolirlo:** la richiesta
porta i **tre campi rappresentabili** e il **nome lo mette il core**. ADR-0005 dice che *«la
riserva è **dichiarata dal richiedente** e verificata dall'arbitro»* — cioè esattamente questa
ripartizione — e nessun testo non fidato raggiunge mai un tipo di decisione.

### P-10 — ⚠️ Il raggio dei derive è più grande di come la §6.8 lo prezza

La §6.8 elenca fra i costi *«`kernel` guadagna due moduli»*. **I moduli sono il costo piccolo.**
I tipi che devono guadagnare i derive del formato **vivono fuori da `wire/`**:

| Tipo | Dove | Chi lo porta sul filo |
|---|---|---|
| `Mib` | `crates/kernel/src/arbiter/resource.rs` | compito 3 (`minicbor`) **e** compito 4 |
| `ComputeClass` | idem | compito 4 |
| `Preemption` | idem | compito 4 |
| `Millis` | `crates/kernel/src/time.rs` | compito 4, dentro `Preemption::After(Millis)` |

⚠️ **E `Mib` li porta DUE VOLTE se il 3bis conferma `bincode`**, perché il compito 3 gli ha già
dato quelli di `minicbor`: un tipo solo, due formati, due insiemi di attributi.

⛔ **QUESTO NON È UN ARGOMENTO PER IL 3BIS, e scriverlo serve proprio a impedire che lo
diventi.** *«Con `minicbor` anche su `ipc` basterebbe un insieme di derive»* è un argomento di
**simmetria e di comodità**, e [ADR-0037](../../adr/0037-criterio-del-pari-per-il-formato-dei-canali.md)
li rifiuta per nome: il criterio là è il **pari**, misurato. Il costo si **registra**, non decide.

### P-11 — ⛔ SBARRA LA PARTE D: tre compiti su quattro devono mettere dati STRUTTURATI E NOSTRI in un record durevole, e `RecordV1` non ha una casella per farlo

Trovato scrivendo la Parte D, il 2026-08-30, e **non è una lacuna di dettaglio**: tocca l'unico
artefatto del progetto che non si corregge.

**I tre compiti, e la riga che li obbliga:**

| Compito | La riga | Che cosa deve entrare nel giornale |
|---|---|---|
| **5** — sensore | §6.4.1: il costo **speso** *«nel verdetto, misurato, **entra nel giornale**»*; e `V14`: l'anello *«apre un passo nuovo»* e vi porta il **dettaglio** | verdetto + dettaglio + costo speso |
| **6** — decisore | §4.3 del disegno: il **record di routing risolto** è giornalato col passo, e *«la disciplina di §4.9 si applica per intero»* | il record di ADR-0011: modello, destinazione, provider, parametri, vincoli, catena di riserva, tentativi, esito |
| **7** — permesso | §6.6: *«un permesso concesso è un **fatto giornalato**»*, e *«quali permessi sono attivi ora»* è una **proiezione del giornale** | la tripla concessa, che la proiezione rilegge |

**E il record non ha dove metterli, misurato in `crates/kernel/src/record.rs`:**

`RecordV1` ha cinque campi, e le **due** caselle di contenuto sono **già assegnate da una
decisione scritta**:

| Campo | Che cosa il suo doc dice di sé |
|---|---|
| `#[n(3)] payload: Vec<u8>` | *«somebody else's and may be anything»* — è il posto del contenuto **esterno**, e *«anything that may have come from outside belongs HERE and nowhere else»* |
| `#[n(4)] reason: String` | *«ours and is always UTF-8»* — testo **nostro**, e *«the asymmetry is the point rather than an accident of typing»* |

⛔ **Nessuna delle due è una casella per dati STRUTTURATI e NOSTRI**, e infilare il CBOR di un
record di routing dentro `payload` **riaprirebbe il difetto che il 2026-08-10 fu chiuso
separando `reason`**: fino a quel giorno `Untrusted::promote` metteva la propria giustificazione
nel `payload` etichettandola `Trust::Untrusted`, e il doc di `reason` scrive che quel record
*«avrebbe portato un'affermazione falsa nell'unico campo il cui mestiere è essere vero»*.
⚠️ **Il precedente vivo dice l'altra metà:** `Arbiter::set_policy` scrive `Trust::Instruction`
col **payload vuoto** e la ragione in `reason`. Byte nostri dentro `payload` **non li scrive
nessuno**.

**E la seconda metà è il `kind`, misurata anch'essa:**

```bash
grep -rn "RecordKind::" crates/kernel/src/    # chi COSTRUISCE e chi FA MATCH
```

✅ Confermato che i match esaustivi sono **due e solo due** — `crates/kernel/src/reconcile.rs`
(righe 91, 92 e 114) e `crates/kernel/tests/frozen_bytes.rs:224`; `arbiter/mod.rs` e
`boundary.rs` **costruiscono**, non decidono. ⚠️ E il doc di `RecordKind` **ha già misurato la
quarta variante**, col nome `Amend`: *«never reaches any bench, because `crate::reconcile`
matches this enum exhaustively and the LIBRARY stops with `E0004`»*.

⛔ **Perché questo FERMA il piano invece di essere una voce d'errata.** Il formato durevole è
l'**unico artefatto irreversibile** del progetto — la quarta proprietà della §3 del
[compendio](../../COMPENDIO.md), *«una finestra che si chiude alla prima riga di codice che
scrive un record»*, e quella riga è stata scritta il 2026-08-10 — e
[ADR-0036](../../adr/0036-evoluzione-del-formato-durevole-del-giornale.md) dice che se i byte
congelati cambiano *«non è un aggiornamento, è un **cambio di formato**»*. Un piano che
dettasse questa scelta la **delegherebbe a un subagente**, ed è la sola specie di decisione che
non si delega.

**Le due forme coerenti, con ciò che è misurato e ciò che è dedotto:**

| | Forma | Stato dell'evidenza |
|---|---|---|
| **α** | **una variante nuova di `RecordKind` per specie** (`Routing`, poi il permesso, poi forse il verdetto), e la struttura codificata **nel `payload`** | ⛔ è la direzione che il **disegno** prende (§4.3) e che **P-2** ha misurato per la variante: byte congelati identici, non compila finché i due match non decidono, indice **non pinzato** finché non nasce un record congelato nuovo. ⚠️ **Ma contraddice il doc di `payload`**, e nessuno ci ha mai messo byte nostri |
| **β** | **un campo facoltativo nuovo su `RecordV1`, a indice libero**, che porta un **enum versionato** del dettaglio strutturato; `RecordKind` **non si tocca** | ✅ la **regola 3 di §4.9.2** lo prevede — *«un campo nuovo è facoltativo e prende un indice nuovo»* — quindi **non è un cambio di formato**; e il Task 10 del Traguardo 3 ha **misurato** che un campo facoltativo a indice libero lascia i byte **identici** finché è `None` (gotcha **#54**). ⚠️ **DEDOTTO e non misurato per QUESTO caso:** che regga con un enum annidato all'indice 5 di `RecordV1` |

⚖️ **Non è presa qui, ed è del proprietario.** La differenza fra le due non è di eleganza: la α
mette byte nostri in una casella che dichiara di essere di qualcun altro e allunga un enum
`index_only` **una volta per specie**; la β lascia `RecordKind` alle tre risposte che dà al
dubbio e concentra l'evoluzione in **un** posto disciplinato. ⛔ **E la β non è la mia da
prendere nemmeno se sembra migliore**, perché il disegno approvato ha già scelto la α alla §4.3:
cambiarla è un **richiamo datato su una sezione approvata**, non un ritocco di piano.

⚠️ **Ciò che NON è in dubbio, e va detto per non far sembrare il buco più grande di quanto sia:**
i compiti **8** (il degrado, che si **ricalcola** e non si scrive) e **9** (la campagna) non
toccano il formato; e il compito 5 può essere scritto **fino al contratto del sensore** — è
l'**anello** che giornala, non il tratto.

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
| **D8** | ⛔ **lo schema del canale worker vive in `crates/kernel/src/wire/worker.rs`, e `wire/mod.rs` nasce al compito 3 — non al 4** | P-6: il disegno non lo colloca, ma cita il precedente che risponde. §6.10.3 dice *«la porta scambia byte … **come `journal` dopo ADR-0036**»*, e lì lo schema vive in `record.rs`, **fuori dalla porta**. Mettere il corpo dentro `ports/process.rs` rifarebbe la mescolanza che ADR-0036 ha tolto al giornale. ⚠️ **Non è un modo nuovo:** è quello che il progetto usa già |
| **D9** | ⛔ **il corpo del canale worker porta UNA direzione sola — worker → core — e DUE varianti**, `Fragment(Vec<u8>)` e `VramPeak(Mib)` | sono le **sole** che qualcosa di scritto impone: §6.10.4 misura l'annotazione **su un frammento audio da 4096 B** e dichiara che *«il campo che questo canale fa entrare nel giornale è il **picco di VRAM** di §5.2.2: arriva dal worker»*. ⛔ **La direzione core → worker non è imposta da nessuna riga, quindi NON si costruisce** — è la stessa postura della §6.1 del disegno (*«il meccanismo è dovuto per iscritto, il vocabolario no»*), e la non-costruzione porta il **proprio innesco**, come la condizione **9** pretende |
| **D10** | ⛔ **l'inquadratura è di QUATTRO byte, big-endian**, decisa qui **una volta sola** | §6.3 lo prescrive alla lettera: *«la larghezza non si decide qui … si decide una volta sola, al compito 3»*. **Big-endian perché il criterio è il PARI** (ADR-0037): `DataView.getUint32(0)` in TypeScript e `struct.unpack(">I", …)` in Python sono le forme **senza bandiera**, e un ordine che il pari deve ricordarsi di girare è un difetto che compila. **Quattro e non due** perché un frammento di flusso non è limitato a 64 KiB; **non otto** perché spenderebbe quattro byte a frame su una portata che nessun canale raggiunge |
| **D11** | `WireError` nasce col commit 3a con le varianti che **`framing` produce**, e guadagna `Malformed` al **3b**, quando il suo produttore esiste | è la regola *«no caller, no item»* che `ProcessError` porta già scritta. Dichiarare al 3a una variante che nessuno produce sarebbe una promessa tenuta da niente — e il commit 3a deve essere `GATE GREEN` da solo (vincolo globale 8). ⚠️ **Quante siano lo dice il compito**, non questa cella: una cifra qui invecchierebbe al primo commit che ne aggiunge una, ed è il gotcha **#31** |
| **D12** | ⛔ **il compito 3bis MISURA e si ferma PRIMA di decidere**, se la misura chiede un cambio di formato | §6.1.1 è **spec**: riaprirla è del proprietario, vincolo globale 7. ⚠️ **La simmetria fra i due casi è solo apparente:** se la misura dice *«`bincode` è ancora l'unica via»*, il compito **registra** e prosegue, perché non tocca nessuna sezione; se dice *«esiste un'alternativa mantenuta col lettore del pari»*, si ferma. Precedenti: **AUD-004**, **AUD-036**, **AUD-044**, tutti fermatisi prima di decidere |
| **D13** | le **fonti** di C-1 vanno in [`riferimenti.md`](../../riferimenti.md), e **non è la convenzione nuova di `E146`** | `E146` riguarda le **misure interne**, che dal Traguardo 5 vivono in [`porta-di-qualita.md`](../../porta-di-qualita.md) accanto al controllo che difendono, e la §7.4 del disegno conferma che la chiusura non tocca quel file. Un **advisory** e lo stato di manutenzione di una crate sono l'altra cosa: la §12 del compendio chiama `riferimenti.md` *«la provenienza di ciò che non abbiamo dedotto noi, con le date»*. ⛔ Senza questa riga le due regole si leggono come un conflitto, e chi esegue ne sceglierebbe una a caso |
| **D14** | ⛔ **il corpo di `ipc` è UN enum solo, `IpcMessage`, con due varianti — una per direzione — e la direzione è DOCUMENTATA, non tipizzata** | la §6.7 dice che l'enumerazione *«la esercitano i DUE messaggi, non uno: con un tipo solo il discriminante non sarebbe provato»*. **Due** enum da una variante ciascuno lascerebbero **entrambi** i discriminanti non provati, cioè il difetto in doppia copia. ⚠️ **E tipizzare la direzione non compra niente ALLA PORTA:** `send` prende `&[u8]` e `receive` rende `Vec<u8>` — il confine non vede nessun tipo, quindi la garanzia esisterebbe solo fra siti del kernel che **oggi non esistono**. Il costo si dichiara accanto al tipo |
| **D15** | ⛔ **il core → gui porta un verdetto a TRE VIE SENZA IL GETTONE** — `Granted` è una variante **unitaria**, `Queued` pure, `Refused` porta i **due numeri** | P-8. ⚠️ **E l'asimmetria fra le tre non è arbitraria:** i due numeri di `Refused` sono l'unica cosa che il filo deve trasportare perché `design/02` vuole *«perché non entra, e l'alternativa praticabile»* e ADR-0020 vieta al kernel di suggerirla — *l'interfaccia costruisce l'alternativa, il kernel le passa il materiale*, quindi la gui **è** il consumatore scritto di quei due numeri. `Queued` invece resta unitaria: `TicketId::get()` è portante *«per un chiamante che ha accodato DUE richieste»*, e la gui ne ha una. Guadagnerà il biglietto col secondo consumatore, non prima |
| **D16** | ⛔ **la richiesta porta i TRE campi rappresentabili del profilo — `reserved_vram`, `compute_class`, `preemption` — e il NOME lo mette il core** | P-9, e non è un ripiego: ADR-0005 dice che *«la riserva è **dichiarata dal richiedente** e verificata dall'arbitro»*, cioè questa ripartizione esatta; e il nome è la parte che, venendo da fuori, sarebbe **testo non fidato dentro un tipo di decisione** (ADR-0014, I6). ⚠️ **Il costo dichiarato:** il core sceglie **un** profilo per la gui, quindi la gui non può chiederne uno arbitrario — ed è ciò che ADR-0033 descrive, un consumatore solo, il viewer 3D oltre la quota |
| **D17** | i derive del formato si aggiungono **dove i tipi vivono**, mai in tipi specchio dentro `wire/` | un tipo specchio è **una seconda definizione dello schema**, ed è la cosa che ADR-0037 rifiuta per il decodificatore del pari — con la differenza che lì sbaglia in silenzio *fuori*, qui sbaglierebbe in silenzio *dentro*: due definizioni si allineano finché qualcuno se ne ricorda, e nulla diventa rosso quando smette. ⚠️ Il prezzo è P-10, ed è dichiarato invece che evitato |
| **D18** | ⛔ **il compito 4 NON dà per scontata l'API del formato** | verificato nel repository: le due chiamate di `bincode` — `encode_to_vec(v, config::standard())` e `decode_from_slice(&b, config) -> (T, usize)` — sono esercitate da `crates/kernel/tests/dependencies_usable.rs`, che le prova **girando**; **il derive no**, nessuna riga del workspace lo usa. Si verifica con una **sonda usa-e-getta** compilata e cancellata nella stessa corsa, o leggendo la sorgente vendorizzata. È il precedente del Task 8 del Traguardo 3, dove il piano **rifiutò di dettare l'API di `redb`**: dettarla a memoria produce codice *plausibile e falso* |
| **D19** | ⚠️ **il compito 4 non chiude NESSUNA riga di catalogo, e va scritto** | misurato: nessuna riga di §7.4.1 o §7.4.2 nomina §6.1 — `awk '/^#### 7\.4\.1/{f=1} /^#### 7\.4\.3/{f=0} f' <spec> \| grep '§6\.1'` non rende niente. Ciò che il compito 4 produce è il **meccanismo** che rende chiudibile `E152` al compito **9**, e il gettone `Q13` è del compito **6**. ⛔ **Un compito che non muove un numeratore è quello su cui si è più tentati di scrivere che l'ha mosso**, ed è la specie di affermazione che la radice **R1** produce |

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
| `crates/kernel/src/ports/process.rs` | compiti 1, 3 | `Started`, `Killed`; e al compito 3 **solo il doc di modulo**, non il `Frame` |
| `crates/kernel/src/framing.rs` | **creato** dal compito 3 | l'inquadratura: lunghezza dichiarata a larghezza fissa, condivisa dai due canali privati, e `WireError` |
| `crates/kernel/src/wire/mod.rs` | **creato** dal compito 3 | la cartella dei due schemi: dichiara i due figli e nient'altro |
| `crates/kernel/src/wire/worker.rs` | **creato** dal compito 3 | lo schema del canale worker: `FromWorker`, coi suoi `encode`/`decode` |
| `crates/kernel/src/wire/ipc.rs` | **creato** dal compito 4 | lo schema `ipc`: l'enumerazione dei messaggi |
| `crates/kernel/src/ports/ipc.rs` | compito 4 | **solo due richiami datati**: il formato e il timbro nel doc di modulo, e la frase di `ClientId` che si **ri-punta** (§6.5) |
| `crates/kernel/src/arbiter/resource.rs` | compiti 3, 4 | i derive di filo su `Mib` (3), `ComputeClass` e `Preemption` (4) — **D17**, e il raggio è **P-10** |
| `crates/kernel/src/time.rs` | compito 4 | il derive di filo su `Millis`, che `Preemption::After` trascina |
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

⚠️ **RICHIAMO DEL 2026-08-30, scrivendo la Parte B — tre celle di questa mappa erano sbagliate,
e la causa è la stessa: `wire/mod.rs` era assegnato al compito 4.** Lo schema del **canale
worker** non aveva nessuna casa (P-6), quindi il compito 3 avrebbe dovuto inventarne una mentre
la eseguiva; e la cella di `ports/process.rs` prometteva *«il `Frame` che diventa codificato»*,
che con la forma della **D8** non succede — il `Frame` **resta identico**, e ciò che il compito 3
tocca lì è il **doc di modulo** reso falso (P-7). ⛔ **Corretta e non riscritta in silenzio:** una
mappa dei file è un'affermazione come le altre, ed `E154` del Traguardo 5 nasce esattamente da
un elenco di file che non nominava due lavori veri — gotcha **#65** applicato all'intestazione.

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

---

## Parte B — il filo

⛔ **Due compiti, e l'ordine fra loro non è negoziabile.** Il **3** costruisce l'inquadratura e
lo schema del canale **worker**, il cui formato ADR-0037 ha già misurato e chiuso; il **3bis**
decide con quale formato si scriverà lo schema **`ipc`**, e sta **prima** del compito 4 perché
scriverlo in `bincode` **è** la decisione, presa per omissione (§3.5 del disegno, **D4**).

⚠️ **I due non condividono nessun file**, misurato mentre si scriveva questa parte: il 3 tocca
`framing.rs`, `wire/`, `lib.rs` e un doc di `ports/process.rs`; il 3bis tocca
`crates/kernel/Cargo.toml`, [`riferimenti.md`](../../riferimenti.md) e — solo nel caso B — il
`Cargo.lock`. Restano comunque **due dispacci separati**, perché la specie di lavoro è diversa:
uno scrive codice, l'altro fa una **ricerca**.

### Compito 3: §6.10 — l'inquadratura e lo schema del canale worker (vincolo 15)

**Files:**
- Create: `crates/kernel/src/framing.rs` — la busta e `WireError`
- Create: `crates/kernel/src/wire/mod.rs` — la cartella dei due schemi
- Create: `crates/kernel/src/wire/worker.rs` — `FromWorker`, `encode`, `decode`
- Modify: `crates/kernel/src/lib.rs` — i due `pub mod` nuovi
- Modify: `crates/kernel/src/arbiter/resource.rs` — `Mib` guadagna i due derive (Passo 4 di 3b)
- Modify: `crates/kernel/src/ports/process.rs` — **solo il doc di modulo** (P-7)
- Test: `crates/kernel/tests/framing.rs`, `crates/kernel/tests/worker_wire.rs` — **creati**
- Modify: [`porta-di-qualita.md`](../../porta-di-qualita.md) — la riga `Q4 · I5 · §6.10`

⛔ **Leggi P-5, P-6 e P-7 prima di cominciare.** Il primo dice che la riga della §11 che stai per
chiudere porta una clausola falsa; il secondo perché lo schema ha una casa che il disegno non gli
dava; il terzo quale frase il tuo commit rende stantia.

⛔ **E i due banchi vivono in `tests/`, cioè FUORI dalla crate, non in un `mod tests`.** È la
terza domanda del pre-controllo di [`../../../CLAUDE.md`](../../../CLAUDE.md): un artefatto
sbagliato compila, e a coglierlo è solo scriverne un'implementazione da fuori. `framing::frame`
e `FromWorker` sono `pub`: se non lo fossero abbastanza, questi banchi non compilerebbero.

#### Commit 3a — l'inquadratura, e la larghezza si decide UNA volta sola

- [ ] **Passo 1: scrivi le sonde che falliscono**

In `crates/kernel/tests/framing.rs`, file nuovo:

```rust
//! The envelope shared by the two private channels. ⛔ THE PROBES LIVE OUTSIDE THE CRATE on
//! purpose: what they hold is that the envelope is USABLE from outside, which is the only
//! form of "the boundary is real" this repository accepts (milestone 3, task 8).

use kernel::framing::{self, WireError};

#[test]
fn a_framed_body_comes_back_exactly() {
    let body = [1u8, 2, 3, 4, 5];
    let framed = framing::frame(&body).expect("frame");
    assert_eq!(framing::unframe(&framed), Ok(&body[..]));
}

#[test]
fn the_declared_length_is_four_bytes_big_endian() {
    // ⛔ THE BYTE ORDER IS AN ASSERTION AND NOT A COMMENT. ADR-0037 chooses a wire format on
    // what the PEER can read: `DataView.getUint32(0)` in TypeScript and `struct.unpack(">I")`
    // in Python are the forms that need no flag. Flipped to little-endian, nothing else in
    // this workspace would go red -- both peers live outside it.
    let framed = framing::frame(&[0xAA]).expect("frame");
    assert_eq!(framed, [0x00, 0x00, 0x00, 0x01, 0xAA]);
}

#[test]
fn a_truncated_frame_is_refused() {
    // Declares five, carries two.
    let bytes = [0x00, 0x00, 0x00, 0x05, 0x01, 0x02];
    assert_eq!(framing::unframe(&bytes), Err(WireError::Incomplete));
}

#[test]
fn bytes_shorter_than_the_prefix_are_refused() {
    // ⚠️ NOT the same failure as the one above, and it is worth its own probe: here there is
    // no declared length AT ALL, so the code path that reads it must not be reached.
    assert_eq!(framing::unframe(&[0x00, 0x00, 0x00]), Err(WireError::Incomplete));
}

#[test]
fn a_frame_with_a_tail_is_refused() {
    // Declares one, carries three. ⛔ THIS IS THE HALF A CBOR DECODER CANNOT SEE: it stops at
    // the first complete element and ignores what follows (gotcha #34, measured in §6.10.4).
    let bytes = [0x00, 0x00, 0x00, 0x01, 0x01, 0x02, 0x03];
    assert_eq!(framing::unframe(&bytes), Err(WireError::TrailingBytes));
}
```

- [ ] **Passo 2: lancia il banco e verifica che NON COMPILI**

```bash
cargo test --locked -p kernel --test framing 2>&1 | head -20
```

Atteso: `error[E0432]` — `kernel::framing` non esiste. ⛔ **Se compila, fermati e scrivi una
voce d'errata:** qualcuno l'ha già costruito, ed è il gotcha **#49**.

- [ ] **Passo 3: scrivi `crates/kernel/src/framing.rs`**

```rust
//! The envelope of the two private channels: a declared length, then the body.
//!
//! ⛔ WHAT IS SHARED IS NEITHER THE TRANSPORT NOR THE SCHEMA -- it is envelope bytes.
//! ADR-0035 reads I4 as "one transport and one schema PER PRIVATE CHANNEL", and ADR-0037
//! distrusts arguments of SYMMETRY between the two channels. Neither is contradicted: both
//! peers must read a length prefix whatever the body format is, so the problem here is
//! LITERALLY the same one and not a symmetric one. §6.3 of the milestone 6 design says so,
//! and whether that reading holds is open item 8 for the owner.
//!
//! ⛔ THE WIDTH IS DECIDED HERE AND NOWHERE ELSE (§6.3). Four bytes, big-endian.

use alloc::vec::Vec;

/// The width of the declared length, in bytes.
const LENGTH_WIDTH: usize = 4;

/// The longest body this envelope can declare.
///
/// ⚠️ THE GUARD ABOVE IT IS DECLARED AND NOT EXERCISED, and saying so is the point: reaching
/// it needs a body of four gibibytes, so no probe in this repository can produce one. What is
/// held instead is the WIDTH, by `the_declared_length_is_four_bytes_big_endian`: while the
/// prefix is four bytes this constant cannot be anything else.
pub const MAX_BODY_LEN: usize = u32::MAX as usize;

/// What can go wrong reading an envelope.
///
/// ⚠️ NO VARIANT CARRIES A PAYLOAD, and that is the shape of the project rather than an
/// omission: no error in this repository carries the value it consumed. The caller that wants
/// the numbers has the bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireError {
    /// Fewer bytes than the frame declares -- the prefix itself may be missing.
    Incomplete,
    /// More bytes than the frame declares.
    TrailingBytes,
    /// The body is longer than a declared length can express. See `MAX_BODY_LEN`.
    TooLong,
}

/// Wraps a body in its envelope.
pub fn frame(body: &[u8]) -> Result<Vec<u8>, WireError> {
    if body.len() > MAX_BODY_LEN {
        return Err(WireError::TooLong);
    }
    let mut bytes = Vec::with_capacity(LENGTH_WIDTH + body.len());
    bytes.extend_from_slice(&(body.len() as u32).to_be_bytes());
    bytes.extend_from_slice(body);
    Ok(bytes)
}

/// Reads a body out of its envelope.
///
/// ⛔ THE TWO FAILURES ARE NOT THE SAME FAULT, and §3.2 of the design puts them in one table:
/// a TAIL is caught by a decoder that checks its own position, a TRUNCATION is caught by
/// NOTHING BUT a declared length -- the tail is not there, and the CBOR can be complete all
/// the same. That is why this function exists on top of the body decoder and not instead of
/// it.
pub fn unframe(bytes: &[u8]) -> Result<&[u8], WireError> {
    if bytes.len() < LENGTH_WIDTH {
        return Err(WireError::Incomplete);
    }
    let (prefix, body) = bytes.split_at(LENGTH_WIDTH);
    let mut declared = [0u8; LENGTH_WIDTH];
    declared.copy_from_slice(prefix);
    let declared = u32::from_be_bytes(declared) as usize;
    if body.len() < declared {
        return Err(WireError::Incomplete);
    }
    if body.len() > declared {
        return Err(WireError::TrailingBytes);
    }
    Ok(body)
}
```

- [ ] **Passo 4: dichiara il modulo in `crates/kernel/src/lib.rs`**

Un `pub mod framing;` accanto agli altri. ⛔ **Non toccare il paragrafo in testa al file:** dice
già che *«ciò che questa crate contiene è la lista dei `pub mod`»*, ed è la forma che il finding
**AUD-046** ha messo lì proprio perché non invecchiasse. Aggiungere una riga al riassunto lo
riaprirebbe.

- [ ] **Passo 5: lancia le sonde e verifica che passino**

```bash
cargo test --locked -p kernel --test framing 2>&1 | tail -5
```

Atteso: `5 passed`.

- [ ] **Passo 6: le mutazioni di controllo, e sono TRE**

⛔ **D7: una alla volta, compilando in un passo separato dall'eseguire, revocando da copia.**

| Mutazione | Deve uccidere |
|---|---|
| `to_be_bytes` → `to_le_bytes` (in **entrambi** i siti, o non compila il round-trip) | `the_declared_length_is_four_bytes_big_endian`, **e nient'altro** |
| `if body.len() < declared` → `if false` | `a_truncated_frame_is_refused` |
| `if body.len() > declared` → `if false` | `a_frame_with_a_tail_is_refused` |

⚠️ **La prima ha un oracolo in più della propria riga: deve uccidere QUELLA SOLA.** Se uccide
anche il round-trip, la sonda dell'ordine non sta provando l'ordine ma la simmetria fra i due
siti — e l'ordine tornerebbe indifendibile appena qualcuno li cambia insieme, che è esattamente
il caso che conta.

```bash
cp crates/kernel/src/framing.rs "$SCRATCH/framing.rs.orig"
# muta, poi:
cargo build --locked -p kernel --tests
cargo test --locked -p kernel --test framing 2>&1 | tail -8
cp "$SCRATCH/framing.rs.orig" crates/kernel/src/framing.rs
git diff --stat crates/kernel/src/framing.rs   # deve essere VUOTO
```

- [ ] **Passo 7: il cancello**

```bash
bash scripts/gate.sh
```

⚠️ **`cargo fmt --all --check` non è un passo del cancello** (§7.4.3), e questo compito crea file
nuovi: lanciatelo a mano, o la deriva arriva col commit del prodotto come al Task 12 del
Traguardo 5.

- [ ] **Passo 8: commit**

```bash
git add -A && git commit -m "traguardo 6 (compito 3a): la busta dei due canali privati -- quattro byte big-endian, e la lunghezza dichiarata prende il TRONCAMENTO che nessun decodificatore CBOR puo' vedere"
```

#### Commit 3b — lo schema del canale worker, e la sola direzione che qualcosa impone

- [ ] **Passo 1: scrivi le sonde che falliscono**

In `crates/kernel/tests/worker_wire.rs`, file nuovo:

```rust
//! The schema of the channel towards the workers. ⛔ OUTSIDE THE CRATE, like `framing.rs`.

use kernel::arbiter::Mib;
use kernel::framing::WireError;
use kernel::wire::worker::FromWorker;

#[test]
fn a_fragment_survives_the_round_trip() {
    let message = FromWorker::Fragment(alloc_vec(&[9, 8, 7]));
    let bytes = message.encode().expect("encode");
    assert_eq!(FromWorker::decode(&bytes), Ok(message));
}

#[test]
fn a_vram_peak_survives_the_round_trip() {
    let message = FromWorker::VramPeak(Mib::new(1536));
    let bytes = message.encode().expect("encode");
    assert_eq!(FromWorker::decode(&bytes), Ok(message));
}

#[test]
fn the_byte_string_annotation_is_measured_and_not_asserted() {
    // ⛔ READING THE ATTRIBUTE IN THE SOURCE PROVES NOTHING -- what the annotation buys is a
    // SIZE, and §6.10.4 measured it: a 4096 B audio fragment costs 4101 bytes as a byte
    // string and 7813 as an array of numbers, i.e. 1.91x. Both compile, both round-trip,
    // both are correct; one costs double the traffic in silence.
    //
    // ⚠️ THE ASSERTION IS A BOUND AND NOT AN EQUALITY, and that is deliberate: an exact
    // number would go red the day the envelope or the variant index changes by a byte, i.e.
    // where the promise is KEPT (gotcha #24, the precedent is PL-1 and its `0600`). Write the
    // exact value you measure in this comment, dated, and leave the bound in the code.
    let body = alloc_vec(&[0u8; 4096]);
    let bytes = FromWorker::Fragment(body).encode().expect("encode");
    assert!(bytes.len() < 4096 + 64, "encoded {} bytes", bytes.len());
}

#[test]
fn a_frame_with_a_tail_does_not_decode() {
    let mut bytes = FromWorker::VramPeak(Mib::new(1)).encode().expect("encode");
    bytes.push(0xFF);
    assert_eq!(FromWorker::decode(&bytes), Err(WireError::TrailingBytes));
}

#[test]
fn junk_inside_the_declared_length_does_not_decode() {
    // ⛔ THE OTHER HALF, AND IT IS A DIFFERENT CHECK: here the ENVELOPE is honest -- the
    // declared length matches the body exactly -- and it is the body that carries a complete
    // CBOR element followed by a byte. `unframe` cannot see it; `position() != len()` can.
    // Remove either check and one of these two probes survives on its own merits.
    let good = FromWorker::VramPeak(Mib::new(1)).encode().expect("encode");
    let body = &good[4..];
    let mut junked = body.to_vec();
    junked.push(0xFF);
    let bytes = kernel::framing::frame(&junked).expect("frame");
    assert_eq!(FromWorker::decode(&bytes), Err(WireError::Malformed));
}
```

⚠️ **`alloc_vec` è un aiutante del banco**, non un tipo del kernel: una `Vec<u8>` costruita dal
test. Scrivilo come preferisci — ma **non** dargli un doc che prometta qualcosa, o è l'aiutante
che non teneva niente del Task 8 del Traguardo 5.

- [ ] **Passo 2: lancia il banco e verifica che NON COMPILI**

```bash
cargo test --locked -p kernel --test worker_wire 2>&1 | head -20
```

Atteso: `error[E0432]` su `kernel::wire`.

- [ ] **Passo 3: `Mib` guadagna i due derive**

In `crates/kernel/src/arbiter/resource.rs`, sul tipo `Mib`, con l'indice esplicito sul campo —
**la forma che `record.rs` usa già**, non una che questo piano ricorda:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Encode, Decode)]
pub struct Mib(#[n(0)] u64);
```

⛔ **È il primo derive di `minicbor` fuori da `record.rs`, e il costo va dichiarato accanto al
tipo:** `Mib` diventa un tipo di **filo** oltre che di decisione. La via che lo evitava —
mettere un `u64` nudo nel messaggio — è **scartata sul merito**: `Mib` esiste perché scambiare
MiB e millisecondi non compili, e un intero nudo che rientra dal filo è precisamente il caso che
i quattro `compile_fail` della §5.1 esistono per togliere.
⚠️ **Non dare per scontato `#[cbor(transparent)]`:** può esistere in `minicbor` 2.3.0 e questo
piano **non l'ha verificato**. Usa la forma sopra, che il repository esercita da tre traguardi, e
se vuoi l'altra **misurane i byte** prima — è la stessa disciplina con cui il Task 8 del
Traguardo 3 rifiutò di dettare l'API di `redb` a memoria.

- [ ] **Passo 4: scrivi `crates/kernel/src/wire/mod.rs` e `wire/worker.rs`**

`wire/mod.rs`:

```rust
//! The schemas of the two private channels, one file each.
//!
//! ⛔ SHARING A FOLDER IS NOT SHARING A SCHEMA -- ADR-0035, rule 2. The two schemas are
//! distinct and so are the two formats, and ADR-0037 measured why: the peers differ. What
//! they do share is the envelope, and it lives in `crate::framing`.
//!
//! ⚠️ `ipc` IS NOT HERE YET: it arrives with task 4, in the format that task 3bis decides.

pub mod worker;
```

`wire/worker.rs`:

```rust
//! The schema of the channel towards the workers (§6.10, ADR-0037).
//!
//! ⛔ THIS TAKES THE MECHANICS OF `record.rs` AND NOT ITS DISCIPLINE, and §6.10.3 says it in
//! as many words: no version enum, no register of retired indices, NO FROZEN BYTES. I4 gives
//! up versioning, and what stands in its place is the build stamp of §6.1.2 -- which this
//! milestone deliberately does NOT build (§3.4). Until it exists, NOTHING REFUSES A STALE
//! PEER, and the trigger is the first real worker process (§0.2).
//!
//! ⛔ ONE DIRECTION ONLY, worker -> core, AND THE OTHER IS A DECLARED NON-CONSTRUCTION.
//! Nothing written imposes a core -> worker message today: `instruct_one` and
//! `instruct_stream` take an opaque `Frame` and no production caller exists. §6.10.4 imposes
//! exactly these two -- it measures the annotation ON AN AUDIO FRAGMENT and names the VRAM
//! peak as the field this channel puts into the journal. Inventing a downward vocabulary now
//! would freeze it against an imaginary consumer -- gotcha #46 from the wrong side, the same
//! reason §3.4 gives for the stamp. The trigger is the same one.
```

Il tipo, con le due varianti di **D9**:

```rust
/// What a worker sends up.
///
/// ⛔ EVERY BYTE THAT RISES IS COVERED BY A RECEIPT (§6.10.1). This enum says what is INSIDE
/// a frame; it never says that a frame may arrive unsolicited -- that one is a FAULT, and the
/// port already has the word for it, `ProcessError::UnsolicitedFrame`.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum FromWorker {
    /// One fragment of an instructed answer -- an audio chunk, a piece of a stream.
    ///
    /// ⛔ THE BYTE-STRING ANNOTATION IS LOAD-BEARING, not decoration, and the same sentence
    /// sits on `RecordV1::payload`. Without it `minicbor` writes AN ARRAY OF NUMBERS:
    /// measured in §6.10.4 on a 4096 B audio fragment, 7813 bytes against 4101, 1.91x. It
    /// compiles, it round-trips, and it is correct -- it costs double the traffic in silence,
    /// which is why the probe that holds it asserts a SIZE and not the attribute.
    #[n(0)]
    Fragment(#[cbor(n(0), with = "minicbor::bytes")] Vec<u8>),

    /// The VRAM peak the work actually reached (§5.2.2).
    ///
    /// ⚠️ IT IS THE ONE FIELD THIS CHANNEL PUTS INTO THE JOURNAL, and there it is subject to
    /// §4.9 -- optional, new index. Here it is not: this schema has no version enum at all.
    #[n(1)]
    VramPeak(#[n(0)] Mib),
}
```

E le due funzioni, che **passano dalla busta**:

```rust
impl FromWorker {
    /// Encodes the message and wraps it in its envelope.
    pub fn encode(&self) -> Result<Vec<u8>, WireError> {
        let mut body = Vec::new();
        let _ = minicbor::encode(self, &mut body);
        framing::frame(&body)
    }

    /// Reads a message out of an envelope.
    ///
    /// ⛔ TWO CHECKS AND NOT ONE, and they catch different faults: `unframe` catches a frame
    /// whose length does not match, `position() != body.len()` catches a body that carries a
    /// complete element AND SOMETHING AFTER IT. A CBOR decoder stops at the first complete
    /// element; the second check is the line `Record::decode` already carries, and the reason
    /// is written there -- finding AUD-047.
    pub fn decode(bytes: &[u8]) -> Result<Self, WireError> {
        let body = framing::unframe(bytes)?;
        let mut decoder = minicbor::Decoder::new(body);
        let message = decoder.decode().map_err(|_| WireError::Malformed)?;
        if decoder.position() != body.len() {
            return Err(WireError::Malformed);
        }
        Ok(message)
    }
}
```

⚠️ **`let _ = minicbor::encode(...)` non è pigrizia**, ed è la stessa riga di `Record::encode`:
il doc di quel metodo spiega perché l'errore è irraggiungibile scrivendo su una `Vec<u8>`.
Rileggilo invece di ricopiarne la ragione qui.

- [ ] **Passo 5: `WireError` guadagna `Malformed` (D11)**

In `crates/kernel/src/framing.rs`. ⛔ **Adesso e non al 3a:** ora ha un produttore.

```rust
    /// The body did not decode as a message of this channel, or it carried a complete
    /// element followed by something else. ⚠️ PRODUCED BY THE SCHEMAS, NOT BY THIS MODULE:
    /// the envelope knows how many bytes there are, never what they mean.
    Malformed,
```

- [ ] **Passo 6: lancia i due banchi**

```bash
cargo test --locked -p kernel --test worker_wire --test framing 2>&1 | tail -8
```

Atteso: `6 passed` e `5 passed`. ⛔ **Scrivi nel commento della terza sonda il numero vero che
hai misurato**, con la data — la sonda resta un limite, il numero misurato è un fatto.

- [ ] **Passo 7: le mutazioni di controllo, e sono TRE**

| Mutazione | Deve uccidere |
|---|---|
| togli `#[cbor(n(0), with = "minicbor::bytes")]` dal `Fragment` | `the_byte_string_annotation_is_measured_and_not_asserted`, **e nient'altro** |
| togli il controllo `decoder.position() != body.len()` | `junk_inside_the_declared_length_does_not_decode`, **e non** `a_frame_with_a_tail_does_not_decode` |
| in `encode`, sostituisci `framing::frame(&body)` con `Ok(body)` | `a_frame_with_a_tail_does_not_decode` e i due round-trip |

⛔ **La seconda riga è l'oracolo che conta, e la sua colonna «e non» è metà dell'asserzione:**
se togliendo `position()` cadessero **entrambe**, le due sonde non starebbero provando due
guasti ma uno solo, e uno dei due controlli sarebbe dominato — è il gotcha **#45** applicato
alle mutazioni, e la §3.2 del disegno afferma proprio che i due guasti hanno **prenditori
diversi**. Se cadono entrambe, **fermati e scrivi una voce d'errata**: a essere sbagliata è la
tabella della §3.2, non la tua sonda.

- [ ] **Passo 8: il richiamo datato su `ports/process.rs` (P-7)**

Il paragrafo *«What milestone 2 builds, and what it does not»* dice **`NOT the wire format`**, e
da questo commit è falso. ⛔ **Si riscrive col proprio richiamo, non si accorcia:** è la forma
che il gotcha **#76** dichiara e il suo limite — a cambiare è il **fatto**. Nomina il file che
ora lo porta, `crate::wire::worker`, e lascia in piedi le due clausole che restano vere.

- [ ] **Passo 9: il registro, e il conteggio si RICONTA**

In [`porta-di-qualita.md`](../../porta-di-qualita.md):
- la riga *«i **byte consumati** pari alla lunghezza dichiarata dal frame — non esiste ancora il
  canale verso i worker. Traguardo 6»* di *«Cosa la porta NON controlla»* esce dalle scoperte,
  **col richiamo datato** e non cancellata;
- la riga di catalogo `Q4 · I5 · §6.10` del livello 2 guadagna la propria cella, con le due
  direzioni e la mutazione che le tiene.

⛔ **Il numeratore del livello 2 si riconta SUL CATALOGO e non per sottrazione**, delimitando per
intestazione — è la forma che il registro usa già e che il gotcha **#26** prescrive:

```bash
awk '/^#### 7\.4\.2/{f=1} /^#### 7\.4\.3/{f=0} f' docs/superpowers/specs/2026-08-06-sottoprogetto-1-kernel.md | grep -c '^| '
```

⚠️ **E il conto deve tornare**: coperte + parziali + scoperte = il totale che il comando dà meno
l'intestazione. Se non torna, **vince il conteggio** e la divergenza è una voce d'errata.

- [ ] **Passo 10: il cancello, `fmt`, e il cancello dei documenti**

```bash
cargo fmt --all --check
bash scripts/gate.sh
bash scripts/check-docs.sh
```

- [ ] **Passo 11: commit**

```bash
git add -A && git commit -m "traguardo 6 (compito 3b): lo schema del canale worker, e l'annotazione di stringa di byte e' tenuta da una MISURA e non dall'attributo -- la direzione core to worker resta una non-costruzione dichiarata"
```

#### Criterio di chiusura del compito 3

- [ ] `GATE GREEN` a entrambi i commit
- [ ] il **vincolo 15** della §11 è onorato in tutte e tre le clausole — lunghezza dichiarata,
      byte consumati verificati, annotazione **sul canale worker** (condizione 6)
- [ ] la riga di catalogo `Q4 · I5 · §6.10` è chiusa **nelle due direzioni**, e il registro la
      porta col conteggio **ricontato**
- [ ] ⛔ la clausola falsa di **P-5** non sopravvive: la riga del vincolo 15 nella §11 del
      [compendio](../../COMPENDIO.md) esce dalla tabella *«cosa resta davanti»*, e non ci si
      limita a spostarla — una riga che sparisce non è una riga corretta
- [ ] la **non-costruzione** della direzione core → worker porta il **proprio innesco**
      (condizione 9), scritto accanto al codice e non solo qui
- [ ] il paragrafo di `ports/process.rs` porta il **richiamo datato** (P-7)
- [ ] nessun byte congelato è nato: §6.10.3 lo vieta, e il banco `frozen_bytes` ha ancora **gli
      stessi file** — `git status --porcelain crates/kernel/tests/frozen/` è **vuoto**

### Compito 3bis: la misura C-1, e la decisione su §6.1.1

**Files:**
- Modify: `crates/kernel/Cargo.toml` — il richiamo datato sulla nota C-1
- Modify: [`riferimenti.md`](../../riferimenti.md) — le fonti, con la data (**D13**)
- Modify: [`porta-di-qualita.md`](../../porta-di-qualita.md) — la voce, col chiusore
- Modify (**solo nel caso B**): `Cargo.lock`

⛔ **Questo compito è una RICERCA, e il suo prodotto è una misura datata.** Non scrive codice.
La §3.5 del disegno dice alla lettera che deciderlo a memoria sarebbe il gotcha **#48**.

⛔ **E c'è un argomento che si rifiuta per nome.** La §8 del [compendio](../../COMPENDIO.md)
vieta di *«riaprire §6.1.1 tanto ora c'è `minicbor` nel kernel»*: fu **tentato il 2026-08-08 e
la misura diede torto** — i due canali hanno **pari diversi**. Se durante questo compito ti
viene in mente quell'argomento, è **già stato scartato**, e riaprirlo pretende una misura nuova
sul **pari**, non una simmetria. L'unico argomento vivo è **C-1**, che è di specie diversa:
riguarda la libreria **dalla nostra parte**.

- [ ] **Passo 1: rileggi il finding contro il codice di ADESSO**

È il passo 1 della disciplina dell'audit. La nota C-1 vive nel manifesto, non in una tabella:

```bash
grep -n "C-1" -A20 crates/kernel/Cargo.toml
grep -rn "bincode" crates/kernel/src/ crates/kernel/tests/
```

⚠️ **La nota afferma *«ZERO production uses»*: verificala, non citarla.** Se il secondo comando
mostra un uso di produzione, la finestra che la nota dice aperta **si è già chiusa**, e questo
compito cambia di specie — scrivi una voce d'errata prima di proseguire.

- [ ] **Passo 2: la misura, oggi, da fonti primarie**

Due domande, e sono **due**: ADR-0037 chiede del **pari**, C-1 chiede di **noi**.

| | Domanda | Che cosa la risponde |
|---|---|---|
| **A** | `bincode` è **ancora** dichiarato non mantenuto? | l'advisory **RUSTSEC-2025-0141** com'è oggi, il repository upstream (ultimo commit, ultima release), la pagina della crate |
| **B** | esiste un'alternativa **mantenuta** il cui pari **TypeScript** abbia un lettore conforme? | la crate candidata **e** il suo lettore TypeScript, ciascuno con la propria evidenza di manutenzione |

⛔ **La B non si risponde con una lista di nomi.** È la forma di **M-11**: un candidato conta
solo se **il pari lo legge**, e la §8 del compendio vieta di rifare M-1…M-11 — ma questa è una
misura **nuova**, non una da rifare. ⚠️ **E il lettore TypeScript ha un precedente scomodo che va
riletto prima di fidarsi di una promessa di pacchetto:** `bincode-ts` 1.0.0 fu misurato con
**entrambi i punti d'ingresso pubblicati rotti su Node 24**, e funzionò dietro un bundler. La
fragilità è dichiarata in §6.10.6 della spec: un pacchetto che *esiste* non è un pacchetto che
*legge*.

⚠️ **Novità non è maturità**, e il criterio non è «l'ultima uscita»: è ciò che oggi è **corrente
e mantenuto**.

- [ ] **Passo 3: traccia le fonti in `riferimenti.md`, con la data**

⛔ **D13, e non è la convenzione nuova di `E146`:** le **misure interne** restano in
[`porta-di-qualita.md`](../../porta-di-qualita.md), le **fonti esterne** vanno qui — è ciò che
la §12 del compendio chiama *«la provenienza di ciò che non abbiamo dedotto noi, con le date»*.
Ogni riga porta **l'indirizzo**, la **data di consultazione** e **che cosa dice**, non una
parafrasi.

- [ ] **Passo 4: il verdetto, e dove si ferma**

| Caso | La misura dice | Che cosa fa questo compito |
|---|---|---|
| **A** | `bincode` resta l'unica via — nessuna alternativa mantenuta col lettore del pari, oppure l'advisory è caduto | **decide**: §6.1.1 resta com'è. Il manifesto riceve il **richiamo datato** con la misura, la voce entra nel registro col proprio chiusore, e il compito 4 procede |
| **B** | esiste un'alternativa mantenuta il cui pari TypeScript ha un lettore conforme | ⛔ **SI FERMA PRIMA DI DECIDERE.** §6.1.1 è **spec**: riaprirla è del proprietario, vincolo globale 7. Scrivi la misura, le **due** opzioni coi costi rimisurati, e portala |

⛔ **La differenza fra i due casi non è la gravità, è che cosa toccano:** il caso A **non tocca
nessuna sezione**, quindi decidere è dovuto; il caso B ne tocca una approvata. È la forma di
**AUD-004**, **AUD-036** e **AUD-044**, tutti fermatisi lì. ⚠️ **Fermarsi non è rimandare:** la
misura è **fatta** ed è il prodotto del compito; ciò che manca è la scelta, che non è
dell'agente.

- [ ] **Passo 5: il caso B, se e solo se il proprietario decide un cambio**

⛔ **Una dipendenza si aggiunge in DUE passi**, e il cancello passa `--locked` a **tutti** i suoi
siti `cargo` — quindi il `Cargo.lock` è un **ingresso**:

```bash
# fuori dal cancello, SENZA --locked
cargo build --workspace
# poi manifesto e lockfile insieme, in UN commit
git add crates/kernel/Cargo.toml Cargo.lock
```

Toccare il manifesto da solo lascia il cancello **rosso**. È il finding **G-5**, ed è il punto e
non il prezzo: ADR-0031 chiama l'aggiunta di una voce *«un atto deliberato e rivedibile»*.
⚠️ **E una voce nuova nella lista di ADR-0031 pretende la propria giustificazione scritta
accanto**, più la riga corrispondente in `scripts/gate-deps.sh` sul grafo **transitivo** — la
voce vecchia esce solo se nessuno la usa più.

- [ ] **Passo 6: il cancello e il cancello dei documenti**

```bash
bash scripts/gate.sh
bash scripts/check-docs.sh
```

- [ ] **Passo 7: commit**

```bash
git add -A && git commit -m "traguardo 6 (compito 3bis): la misura C-1 e' rifatta oggi invece che ricordata -- e la decisione su 6.1.1 e' quella che la misura sostiene, non quella dell'omissione"
```

#### Criterio di chiusura del compito 3bis

- [ ] la condizione **7** della Definizione di «fatto» è soddisfatta: la decisione C-1 è presa
      con una **misura odierna**
- [ ] le fonti stanno in [`riferimenti.md`](../../riferimenti.md) **con la data di
      consultazione**, e non sono parafrasate
- [ ] la nota C-1 di `crates/kernel/Cargo.toml` porta il **richiamo datato** — ⛔ **anche nel
      caso A**: una nota che dice *«decide at milestone 6»* è falsa il giorno dopo che il
      traguardo ha deciso
- [ ] la voce entra nella **tabella unica** delle voci aperte del traguardo (condizione 11) se
      resta aperta, o esce col commit che la chiude
- [ ] nel **caso B**: manifesto e `Cargo.lock` in **un** commit, e `bash scripts/gate.sh` verde
      **dopo** — non prima
- [ ] ⛔ nessuna riga di schema `ipc` è stata scritta: il compito **4** viene dopo, ed è la
      ragione per cui questo compito esiste

---

## Parte C — lo schema `ipc`

⛔ **Un compito solo, e la scelta va detta perché contraddice il precedente della D1.** Il
compito 1 si consegna in tre commit perché le sue tre parti hanno **raggi diversi**; qui il
cambiamento è **uno**, e la prosa che il codice rende falsa deve atterrare **nello stesso
commit** del fatto che la smentisce — è la lezione di **P-7**, e spezzarla in due commit la
rimetterebbe in piedi per la durata di uno.

### Compito 4: §6.1 — la busta porta due messaggi, nel formato che il 3bis ha deciso

**Files:**
- Create: `crates/kernel/src/wire/ipc.rs` — `IpcMessage`, `encode`, `decode`
- Modify: `crates/kernel/src/wire/mod.rs` — il `pub mod ipc;`
- Modify: `crates/kernel/src/arbiter/resource.rs` — i derive su `ComputeClass` e `Preemption`
- Modify: `crates/kernel/src/time.rs` — il derive su `Millis`
- Modify: `crates/kernel/src/ports/ipc.rs` — **due** richiami datati, e nient'altro
- Test: `crates/kernel/tests/ipc_wire.rs` — **creato**
- Modify: [`porta-di-qualita.md`](../../porta-di-qualita.md)

⛔ **Leggi P-8, P-9 e P-10 prima di cominciare.** Due celle della §6.2 del disegno **non sono
implementabili alla lettera**, e la ragione non è un fastidio di tipi: metterle sul filo
conierebbe concessioni dai byte e farebbe entrare testo non fidato in un tipo di decisione.

- [ ] **Passo 0: leggi il verdetto del compito 3bis, e se non c'è FERMATI**

⛔ **Il formato non si decide qui.** Scrivere questo schema in `bincode` senza che il 3bis sia
girato **è** la decisione C-1, presa per omissione — D4, e la §3.5 del disegno lo dice alla
lettera. Se il 3bis non è stato eseguito, questo compito non è dispacciabile.

```bash
grep -n "C-1" -A6 crates/kernel/Cargo.toml     # deve portare il richiamo datato del 3bis
```

📌 **Da qui in poi il compito ha DUE rami, e cambiano tre righe in tutto:** i derive sul tipo, la
chiamata che codifica e quella che decodifica. Tutto il resto — la busta, l'enumerazione, le
sonde, le mutazioni — è **identico**, ed è il motivo per cui questo compito si è potuto scrivere
prima che il 3bis girasse.

| | `bincode` (il ramo che il manifesto ha oggi) | `minicbor` (se il 3bis lo ha scelto) |
|---|---|---|
| derive | `#[derive(bincode::Encode, bincode::Decode)]` — ⛔ **da VERIFICARE, D18** | `#[derive(minicbor::Encode, minicbor::Decode)]` con gli indici, come `record.rs` |
| codifica | `bincode::encode_to_vec(self, bincode::config::standard())` ✅ **verificata in repo** | `minicbor::encode(self, &mut body)` ✅ verificata |
| byte consumati | il `usize` che `decode_from_slice` **restituisce già** ✅ verificata | `decoder.position()`, come `Record::decode` ✅ verificata |

- [ ] **Passo 1: verifica il derive del formato, non ricordarlo (D18)**

Nessuna riga del workspace usa il derive di `bincode`: `dependencies_usable.rs` prova le due
**funzioni** su un `u32`, non gli attributi. Scrivi una **sonda usa-e-getta**, compilala,
leggi l'esito, **cancellala nella stessa corsa** — è la forma che questo repository usa per
P-2 e per il quinto caso `compile_fail`. In alternativa, leggi la sorgente vendorizzata:

```bash
ls -d ~/.cargo/registry/src/*/bincode-2.0.1
```

⚠️ **Ciò che va verificato, e non è solo «il derive esiste»:** che si applichi a un **enum**,
quale sia il **percorso** dell'attributo, e se il tratto porti parametri generici che la firma
di `decode` deve nominare. Un derive che compila su una struct e non su un enum è esattamente il
genere di cosa che un ricordo non distingue.

- [ ] **Passo 2: scrivi le sonde che falliscono**

In `crates/kernel/tests/ipc_wire.rs`, file nuovo:

```rust
//! The schema of the `ipc` channel. ⛔ OUTSIDE THE CRATE, like `framing` and `worker_wire`.

use kernel::arbiter::{ComputeClass, Mib, Preemption};
use kernel::framing::WireError;
use kernel::time::Millis;
use kernel::wire::ipc::{GrantRequest, IpcMessage, Verdict};

fn a_request() -> GrantRequest {
    GrantRequest {
        reserved_vram: Mib::new(2048),
        compute_class: ComputeClass::Interactive,
        preemption: Preemption::After(Millis::new(500)),
    }
}

#[test]
fn a_grant_request_survives_the_round_trip() {
    let message = IpcMessage::Request(a_request());
    let bytes = message.encode().expect("encode");
    assert_eq!(IpcMessage::decode(&bytes), Ok(message));
}

#[test]
fn a_verdict_survives_the_round_trip() {
    // ⛔ THIS IS THE PROBE THAT EXERCISES THE DISCRIMINANT, and it is why §6.7 asks for TWO
    // messages rather than one: with a single message type the tag never varies, and a bug in
    // how it is written or read would be invisible. Same shape as the journal freezing THREE
    // records instead of one.
    let message = IpcMessage::Verdict(Verdict::Refused {
        asked: Mib::new(4096),
        ceiling: Mib::new(1024),
    });
    let bytes = message.encode().expect("encode");
    assert_eq!(IpcMessage::decode(&bytes), Ok(message));
}

#[test]
fn a_message_with_a_tail_does_not_decode() {
    let mut bytes = IpcMessage::Verdict(Verdict::Granted).encode().expect("encode");
    bytes.push(0xFF);
    assert_eq!(IpcMessage::decode(&bytes), Err(WireError::TrailingBytes));
}

#[test]
fn junk_inside_the_declared_length_does_not_decode() {
    // ⛔ THE ENVELOPE IS HONEST HERE and the body is not -- the other half of the pair, and it
    // is a DIFFERENT check. See the same probe in `worker_wire.rs`.
    let good = IpcMessage::Verdict(Verdict::Granted).encode().expect("encode");
    let mut junked = good[4..].to_vec();
    junked.push(0xFF);
    let bytes = kernel::framing::frame(&junked).expect("frame");
    assert_eq!(IpcMessage::decode(&bytes), Err(WireError::Malformed));
}
```

✅ **I nomi di questa sonda sono VERIFICATI nel sorgente, non ricordati** — `ComputeClass` ha
`Realtime`, `Interactive`, `Batch`, e `Millis::new` esiste:

```bash
grep -n "pub enum ComputeClass" -A20 crates/kernel/src/arbiter/resource.rs
grep -n "impl Millis" -A8 crates/kernel/src/time.rs
```

⚠️ **Rilanciali comunque prima di scrivere:** è il passo 1 della disciplina dell'audit, e il
rimedio di **AUD-036** ha già tradotto `interattivo` in `interactive` in `design/02` — cioè
questa famiglia di nomi si è mossa una volta.

- [ ] **Passo 3: lancia il banco e verifica che NON COMPILI**

```bash
cargo test --locked -p kernel --test ipc_wire 2>&1 | head -20
```

Atteso: `error[E0432]` su `kernel::wire::ipc`.

- [ ] **Passo 4: i derive sui tipi condivisi (D17, P-10)**

Su `Mib`, `ComputeClass` e `Preemption` in `crates/kernel/src/arbiter/resource.rs`, e su
`Millis` in `crates/kernel/src/time.rs`. ⛔ **Accanto a ciascuno va scritto PERCHÉ ce l'ha**, e
la frase è la stessa: *questo tipo attraversa un canale privato*. Un derive senza una ragione
accanto è ciò che il Task 11 del Traguardo 5 ha passato una revisione intera a potare.

⚠️ **`Mib` può finire con DUE insiemi di attributi** — quelli di `minicbor` dal compito 3 e
quelli del formato di `ipc` — e non è un difetto: è il costo di ADR-0037, che ha scelto **due
formati misurati** per due pari diversi. ⛔ **Non «sanarlo»:** la §8 del compendio vieta di
riaprire §6.1.1 per simmetria, e questa sarebbe la stessa mossa travestita da pulizia.

- [ ] **Passo 5: scrivi `crates/kernel/src/wire/ipc.rs`**

Il doc di modulo, e le tre cose che deve dire:

```rust
//! The schema of the `ipc` channel: the envelope of `crate::framing` carrying ONE enum.
//!
//! ⛔ ONE ENUM FOR BOTH DIRECTIONS, AND THE DIRECTION IS DOCUMENTED RATHER THAN TYPED. Two
//! enums of one variant each would leave BOTH discriminants unexercised, which is the very
//! thing §6.7 asks two messages for. And typing the direction would buy nothing at the port:
//! `send` takes `&[u8]` and `receive` returns `Vec<u8>`, so the boundary sees no type at all.
//! ⚠️ THE COST, stated: nothing stops a caller from encoding a `Verdict` and sending it UP.
//! Today there is no such caller -- the transport is staged out (open item 5) -- and the day
//! there is one, the guard that pays for itself is on the composition side, not here.
//!
//! ⛔ THE SCHEMA MINTS NO IDENTIFIERS BECAUSE IT CARRIES NONE, and saying it that way is the
//! point (§6.5). Writing "§6.1.3 is satisfied" would be green having compared empty sets. A
//! grant request is not a step of a run: it writes no record and carries neither `StepId` nor
//! `RunId`. The first message that carries an identifier is where the rule becomes real, and
//! where its probe is born.
//!
//! ⛔ NO VERSION ENUM, NO RETIRED-INDEX REGISTER, NO FROZEN BYTES -- I4 renounces versioning
//! (§6.4). What stands in its place is the BUILD STAMP of §6.1.2, WHICH THIS MILESTONE DOES
//! NOT BUILD (§3.4). Until it exists, NOTHING REFUSES A STALE GUI, and today that costs
//! nothing because there is no gui to refuse -- `grep -rn "impl Ipc" crates/` returns a bench
//! fake. The trigger is milestone 2 of the subproject, the one that brings the shell.
//!
//! ⛔ AND THE REVOCATION core -> gui IS A DECLARED NON-CONSTRUCTION. ADR-0033 names it -- "the
//! gui stops rendering the 3D and says so" -- and it is the first message this vocabulary will
//! gain. It is not here because no written line demands it today and because §5.7 row 3 speaks
//! of a gui that DIES, not one that is asked. ⚠️ THE COST IS REAL: until then a discretionary
//! grant is preemptible IN THE BOOKS and the gui never hears about it. Open item 7.
```

E il tipo, con la forma della **D14**, della **D15** e della **D16**:

```rust
/// What the gui asks for: an ordinary grant beyond the presentation quota (ADR-0033).
///
/// ⛔ IT IS NOT A `ResourceProfile`, AND THE MISSING FIELD IS THE REASON. `ResourceProfile`
/// carries `name: &'static str`, which cannot be produced from arriving bytes without leaking
/// -- and what would be leaked is text CHOSEN BY THE GUI, i.e. untrusted content (ADR-0014)
/// inside a type the arbiter DECIDES with. The split here is the one ADR-0005 already
/// describes: THE REQUESTER DECLARES THE RESERVATION, and the core names the profile.
pub struct GrantRequest {
    pub reserved_vram: Mib,
    pub compute_class: ComputeClass,
    pub preemption: Preemption,
}

/// The three-way outcome, WITHOUT the grant.
///
/// ⛔ `Granted` IS A UNIT VARIANT AND CARRIES NO `Grant`, which is the whole of this type. A
/// decodable `Grant` would be a capability MINTED FROM BYTES: §5.6 holds that the only site
/// that mints one is `Arbiter::issue`, and `tests/compile_fail/grant_has_no_constructor.rs`
/// exists to make it unspeakable from outside. It would be AUD-050 done again on the
/// strongest token in the project -- a guard is worth exactly what its constructor is worth.
/// ⚠️ AND THE GUI DOES NOT NEED ONE: ADR-0033 says the grant is STATE OF THE CORE (I1). What
/// crosses is the verdict.
///
/// ⚠️ `Refused` CARRIES TWO NUMBERS AND `Queued` CARRIES NOTHING, and the asymmetry is
/// argued: design/02 wants "why it does not fit and the workable alternative", ADR-0020
/// forbids the kernel to suggest one, so THE INTERFACE BUILDS IT AND THE KERNEL HANDS OVER
/// THE MATERIAL -- the gui is the written consumer of those two. A ticket, by contrast, is
/// load-bearing only for a caller with TWO requests outstanding, and the gui has one.
pub enum Verdict {
    Granted,
    Queued,
    Refused { asked: Mib, ceiling: Mib },
}

/// One message on the `ipc` wire.
pub enum IpcMessage {
    /// gui -> core.
    Request(GrantRequest),
    /// core -> gui.
    Verdict(Verdict),
}
```

E le due funzioni, **identiche nella forma a quelle di `wire::worker`**: `encode` codifica e
poi chiama `framing::frame`; `decode` chiama `framing::unframe`, decodifica, e **verifica i byte
consumati** contro la lunghezza del corpo. ⛔ **Sono due controlli e non uno**, e il doc lo deve
dire: la busta prende il **troncamento**, il conteggio dei consumati prende la **coda dentro** la
lunghezza dichiarata.

- [ ] **Passo 6: lancia il banco e verifica che passi**

```bash
cargo test --locked -p kernel --test ipc_wire 2>&1 | tail -5
```

- [ ] **Passo 7: le mutazioni di controllo, e sono TRE**

⛔ **D7: una alla volta, compilando in un passo separato dall'eseguire, revocando da copia.**

| Mutazione | Deve uccidere |
|---|---|
| in `encode`, `framing::frame(&body)` → `Ok(body)` | `a_message_with_a_tail_does_not_decode` e i due round-trip |
| ⛔ l'encoder scrive **sempre** il discriminante della prima variante | `a_verdict_survives_the_round_trip`, **e NON** `a_grant_request_survives_the_round_trip` |
| togli il confronto fra byte consumati e lunghezza del corpo | `junk_inside_the_declared_length_does_not_decode`, **e NON** `a_message_with_a_tail_does_not_decode` |

⛔ **Le due colonne «e NON» sono metà dell'asserzione, e la seconda riga è l'oracolo di questo
compito:** è la sola prova che il corpo è davvero **un'enumerazione** e non un tipo solo travestito.
Se quella mutazione uccide **entrambi** i round-trip, il discriminante non sta distinguendo
niente — fermati e scrivi una voce d'errata.
⚠️ **Come si scrive la seconda mutazione dipende dal formato** e questo piano non la detta: con
un derive il tag lo scrive la macro, quindi la via è **scambiare i corpi delle due varianti**
nel punto in cui si costruiscono, oppure codificare a mano il tag sbagliato. Sceglila leggendo
il codice che hai scritto, e **scrivi quale hai usato**.

- [ ] **Passo 8: i due richiami datati su `crates/kernel/src/ports/ipc.rs` (P-7 nella sua forma qui)**

⛔ **Due, e sono di specie diversa.**

① Il doc di modulo dice: *«Milestone 6 brings the SCHEMA — **`bincode` in `kernel`** … and the
**BUILD STAMP** of §6.1.2»*. **Entrambe le metà cambiano**, e la seconda più della prima: il
formato è quello che il 3bis ha deciso, e **il timbro NON arriva** — §3.4 lo toglie dal
perimetro e ne fa una non-costruzione dichiarata. ⚠️ **È una scadenza in prosa**, gotcha **#77**,
e questa è la corsa in cui scade: nulla è mai diventato rosso per lei.

② La frase di `ClientId`: *«Whoever implements this port in milestone 6 draws from THAT
counter»*. ⛔ **Si RI-PUNTA, non si toglie**, e lo prescrive la §6.5 del disegno: invecchia nel
**soggetto** e non nell'affermazione — gotcha **#87** — perché il Traguardo 6 **non implementa
la porta** (voce 5, il trasporto è scaglionato). Toglierla lascerebbe scoperto il difetto che
esiste per impedire: **due contatori identici che divergono senza che nulla lo segnali**.

- [ ] **Passo 9: il registro, e ciò che NON si scrive**

In [`porta-di-qualita.md`](../../porta-di-qualita.md): la sezione del compito, con le sonde, le
tre mutazioni e il **loro esito misurato**.

⛔ **E ciò che NON si scrive è la parte che va letta due volte (D19): questo compito non chiude
nessuna riga di catalogo.** Nessuna riga di §7.4 nomina §6.1. Non muovere nessun numeratore, e
**scrivi che non si muove** — la riga della campagna DST resta `PARZIALE` fino al compito 9, e
il gettone `Q13` fino al compito 6.

⚖️ **Una voce si registra e non si prende:** un caso `compile_fail` che tenga *«un verdetto non
può portare un `Grant`»* sarebbe una **riga di catalogo nuova**, cioè §7.4, cioè **spec** —
vincolo globale 7. Oggi quella proprietà è tenuta dal **fatto che `Verdict::Granted` è unitario**
e dal doc accanto; è **livello 1 per costruzione, non per un caso negativo**, ed è una
distinzione da scrivere invece che da lasciare intendere.

- [ ] **Passo 10: il cancello, `fmt`, e il cancello dei documenti**

```bash
cargo fmt --all --check
bash scripts/gate.sh
bash scripts/check-docs.sh
```

- [ ] **Passo 11: commit**

```bash
git add -A && git commit -m "traguardo 6 (compito 4): lo schema ipc -- il verdetto NON porta il gettone, e la richiesta non porta il nome del profilo: due celle della 6.2 non erano implementabili alla lettera"
```

#### Criterio di chiusura del compito 4

- [ ] `GATE GREEN`, e la baseline **rimisurata** col comando (D5)
- [ ] il corpo è **un enum con due varianti**, e la mutazione del discriminante ne uccide
      **una sola** — è la prova che §6.7 chiede
- [ ] ⛔ **nessun `Grant` è raggiungibile dal filo**: `grep -rn "Grant" crates/kernel/src/wire/`
      non deve restituire **niente** fuori dai commenti, e i commenti dicono **perché**
- [ ] ⛔ **nessun `&'static str` e nessuna stringa nello schema:** un campo di testo che arriva
      dalla gui è contenuto **non fidato**, e `Untrusted` è il solo tipo che può portarlo
      (ADR-0014). Se un messaggio futuro ne avrà bisogno, quello è il tipo, non `String`
- [ ] i **due richiami datati** di `ports/ipc.rs` ci sono, e il secondo **ri-punta** invece di
      togliere (§6.5)
- [ ] le **due** non-costruzioni — il **timbro** e la **revoca verso la gui** — portano ciascuna
      il proprio **innesco**, scritto accanto al codice (condizione 9)
- [ ] nessun byte congelato è nato: §6.4 lo vieta, e `git status --porcelain
      crates/kernel/tests/frozen/` è **vuoto**
- [ ] ⚠️ il registro **non** dichiara chiusa nessuna riga di catalogo (D19)
