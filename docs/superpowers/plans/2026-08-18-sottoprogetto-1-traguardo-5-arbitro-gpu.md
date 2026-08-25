# Traguardo 5 — l'arbitro GPU: il piano

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:subagent-driven-development`
> (raccomandata) o `superpowers:executing-plans` per eseguire questo piano **un compito per
> volta**, con revisione fra uno e l'altro. I passi usano caselle (`- [ ]`).

- **Data:** 2026-08-18
- **Traduce:** [il disegno del Traguardo 5](../specs/2026-08-18-sottoprogetto-1-traguardo-5-arbitro-gpu-design.md), dodici sezioni
- **Fonti:** §5 della [spec del sotto-progetto 1](../specs/2026-08-06-sottoprogetto-1-kernel.md) · ADR-0005 · ADR-0006 · ADR-0033 · ADR-0034 · [design/02](../../design/02-arbitrato-gpu.md)
- **Baseline misurata prima di scrivere:** `bash scripts/gate.sh` → `GATE GREEN`; `cargo test --workspace --no-fail-fast --locked` → **32 target · 194 passati · 0 falliti · 2 ignorati**; albero pulito a `b041620`

**Goal:** costruire l'arbitro GPU intero — ammissione, corsie, ciclo della concessione,
revoca, due policy — dentro `crates/kernel/src/arbiter/`, e provare ciò che si può provare
senza un secondo meccanismo.

**Architecture:** un **modulo nuovo di `kernel`**, non una settima famiglia di porte: le sei
sono dichiarate esaustive. L'arbitro è **logica pura dei propri ingressi** — non legge
l'orologio, non possiede un reattore, non possiede un giornale: riceve `now: Monotonic` come
argomento e il giornale per riferimento quando serve. I guasti si iniettano **dalle porte che
usa** (`reactor`, `journal`), mai dentro di lui.

**Tech Stack:** Rust `1.95.0` appuntata · edition 2024 · `kernel` è `#![no_std]` + `alloc` +
`#![forbid(unsafe_code)]` · strutture `BTreeMap` e `Vec` (§5.1) · `trybuild` per i casi di
compilazione fallita · nessuna dipendenza nuova.

---

## Vincoli globali

Valgono per **ogni** compito, e non si ripetono nei passi.

| # | Vincolo | Da |
|---|---|---|
| **1** | ⛔ **Codice in inglese, documentazione in italiano.** Crate, moduli, tipi, funzioni, messaggi d'uscita e **commenti nel sorgente** sono in inglese. Un riferimento al codice dentro un documento si scrive in inglese, col nome esatto del sorgente | §1.0 della spec · gotcha **#40** |
| **2** | ⛔ **Ogni compito chiude con `bash scripts/gate.sh` → `GATE GREEN`**, poi commit. Un compito che lascia la porta rossa non è finito | vincolo globale 8 dei piani precedenti |
| **3** | ⛔ **`kernel` non nomina `std`.** `HashMap` non è nominabile, ed è gratis: `BTreeMap` e `Vec` da `alloc` | §1.4 · gotcha **#12** |
| **4** | ⛔ **Nessun `#[allow]`.** Un `allow` è un divieto spento. Se serve, il codice è sbagliato | gotcha **#13** |
| **5** | ⛔ **Gli `.stderr` di `trybuild` non si rigenerano in blocco.** Ogni oracolo nuovo si **legge** nel diff. `TRYBUILD=overwrite` è un atto deliberato, mai un riflesso | gotcha **#25** |
| **6** | ⛔ **Ogni regola nuova si prova in DUE direzioni**: che scatti dove deve, **e che non scatti dove non deve** | gotcha **#24** · §7.1.1 regola 3 |
| **7** | ⛔ **La §7.4 della spec è SPEC.** Aggiungere una riga di catalogo è una decisione del **proprietario**: si **registra**, non si prende | vincolo globale 7 · §8.1.2 · gotcha **#36** |
| **8** | ⛔ **I byte congelati non si rigenerano.** Un campo nuovo di `RecordV1` è `Option` con `#[cbor(default)]` e prende un **indice nuovo**; l'indice **5 è libero**, misurato | ADR-0036 · §4.9 |
| **9** | ⛔ **Una dipendenza si aggiunge in due passi**, e questo traguardo **non ne aggiunge nessuna**. Se un compito credesse di averne bisogno, si ferma e lo riporta | ADR-0031 · finding **G-5** |
| **10** | ⛔ **I fine-riga sono misti per file, e si sa ESATTAMENTE quali.** Censiti sui blob committati il 2026-08-18: i file con `CR` nell'indice sono **quattro in tutto il repository**, tutti sorgenti Rust — `crates/kernel/src/ports/process.rs` (**291**), `crates/kernel/tests/ports_are_implementable.rs` (**971**), `crates/kernel/tests/reactor_contract.rs` (669), `crates/platform/src/reactor.rs` (123). ⛔ **E i primi DUE li tocca il Task 4 di questo piano.** Chi li tocca **conserva i loro fine-riga** e li rimisura dopo con `tr -cd '\r' \| wc -c`; mai `sed -i`. ⚠️ Sugli altri file la normalizzazione è assorbita da `core.autocrlf=true`, quindi **il diff non la mostra** — è precisamente su questi quattro che non lo è | gotcha **#48**, dodicesima forma · finding **G-5** |
| **11** | ⛔ **Le misure vivono nello scratchpad**, non nel repository, e si ripulisce dopo | `CLAUDE.md` |
| **12** | ⛔ **Prima di eseguire un compito si fanno le SETTE domande del pre-controllo** — l'elenco è in `CLAUDE.md`. Un difetto è stato trovato in **ventidue compiti su ventidue** | gotcha **#49**, **#58**, **#59**, **#65** |
| **13** | ⛔ **Un compito scritto prima si legge contro il codice di ADESSO**, non contro questo piano. Il contratto cresce sotto il piano | quinta domanda del pre-controllo |
| **14** | ⛔ **Una divergenza si REGISTRA nell'errata in testa a questo file**, non si appiana in silenzio | `CLAUDE.md` · gotcha **#15** |

---

## Errata — le voci che l'esecuzione produce

⛔ **Questo blocco nasce vuoto, e non è una formalità.** L'errata dei piani precedenti è
arrivata a **settantasette voci in nove passate** al Traguardo 3 e a **settanta in nove** al
Traguardo 4, con **ventuno decisioni** prese eseguendo. Il pre-controllo ha trovato almeno un
difetto reale in **ventidue compiti su ventidue**.

| Regola | |
|---|---|
| **dove si scrive** | qui, in coda a questa tabella, con la sigla `E<n>` e il compito che l'ha trovata |
| **cosa NON si fa** | riscrivere il corpo del compito. Il testo è il registro di **ciò che fu deciso**, non di ciò che è stato fatto |
| **quando si ferma** | se la divergenza tocca un **contratto di porta condivisa** o una **riga di catalogo §7.4**, si riporta al proprietario invece di deciderla (vincolo globale 7) |

| # | Compito | Voce |
|---|---|---|
| **E1** | Task 1 | ⚠️ **`resource.rs` apriva con `use crate::time::Millis;`, e nel Task 1 nessun codice lo nomina.** `Millis` compare per la prima volta come argomento in `Preemption::After(Millis)`, che è del Task 2 — quindi quella riga, scritta com'era, sarebbe stata un **unused import warning** a ogni build del Task 1. Risolto **omettendo** la riga: il tipo pieno `crate::time::Millis` compare solo nel Passo 6, in una mutazione **temporanea** (`impl From<Mib> for crate::time::Millis` e viceversa), che quindi non ha bisogno di un `use` nemmeno lei. Misurato: `cargo build --locked --workspace` **zero warning** con la riga omessa |
| **E2** | Task 1 | ⚠️ **Il Passo 6 prevedeva «gli altri restano `ok`», e non regge per la regola A.** Misurato con `impl From<Mib> for crate::time::Millis` (e, nell'altra direzione, `impl From<crate::time::Millis> for Mib`): il caso `no_conversion_from_*` della stessa direzione scatta `error` come atteso, ma il caso «passa l'uno per l'altro» della **stessa** direzione (`mib_as_millis.rs` o `millis_as_mib.rs`) passa da `ok` a **`mismatch`**, non resta `ok`. È la stessa specie già scritta in `docs/porta-di-qualita.md` per `Monotonic`/`WallTime` (gotcha #42, forma indiretta): `Mib` e `Millis` sono **valori posseduti**, quindi con l'`impl From` presente rustc appende un `help: call `.into()`` che l'oracolo non porta, e il confronto letterale fallisce. La regola B (`no_conversion_from_*`) resta comunque isolata da questo effetto — scatta `error` in entrambe le direzioni, indipendente dall'oracolo. Registrato in `docs/porta-di-qualita.md`, sezione «Come scattano le due direzioni», con la tabella della misura e i comandi |
| **E3** | Task 1 | ⛔ **Il comando di verifica del Passo 5 è VACUO, e il difetto è nel comando non nell'esito.** Il passo detta `git diff --stat crates/kernel/tests/compile_fail/` attendendo *«quattro file `.stderr` nuovi e nessun altro toccato»*: ma i quattro nascono **non tracciati**, e `git diff` non vede i file non tracciati — quindi quel comando risponde **vuoto** sia quando la rigenerazione si è comportata bene, sia quando ha riscritto un oracolo pre-esistente. ⚠️ Cioè la metà che il passo esiste per cogliere — *«se un `.stderr` pre-esistente compare nel diff»* — sarebbe stata invisibile con l'uscita vuota letta come conferma. Sostituito con `git status --porcelain`, che elenca i non tracciati (`??`) **e** i modificati (` M`) nella stessa uscita. ✅ Misurato: quattro `??` e nessun ` M` sotto `tests/compile_fail/`. 📌 È il gotcha **#14** applicato a un comando di verifica invece che a un test: un controllo che non può fallire non è un controllo |
| **E4** | Task 2 | ⚠️ **Il Passo 3 scriveva `After(Millis)` e `Option<Millis>` senza `use crate::time::Millis;`, e non avrebbe compilato (`E0412`).** È il punto in cui l'omissione di E1 torna legittimamente indietro: il Task 1 aveva omesso quella riga perché nessun codice nominava `Millis` allora; il Task 2 la reintroduce perché `Preemption::After(Millis)` la nomina davvero. Aggiunta `use crate::time::Millis;` in testa a `crates/kernel/src/arbiter/resource.rs`, sotto il commento di modulo. Registrarlo qui è ciò che impedisce a un lettore futuro di leggere E1 come un errore invece che come una scelta temporanea |
| **E5** | Task 3 | ⚠️ **`a_descriptor_names_the_profile_it_describes` non può fallire a runtime, qualunque cosa faccia il codice di produzione — il brief la scrive così ed è stata eseguita così, ma il registro deve dirlo.** Il test scrive lo stesso letterale (`"trellis2-512-lean"`) sia nel `ResourceProfile` sia nel `WorkDescriptor`, quindi `profile.name == descriptor.profile_name` confronta due copie della STESSA stringa e non due valori indipendenti prodotti da un percorso di produzione. La sua forza reale è al COMPILATORE: prova che le due forme esistano, portino quei nomi di campo e quei tipi, e siano costruibili da FUORI la crate (gotcha #46) — non che restino allineate a runtime. Nessuna modifica al test: inventare un secondo letterale o un lookup per farla mordere a runtime sarebbe costruire un meccanismo che il compito non chiede. Registrato in `docs/porta-di-qualita.md`, sezione «Livello 1 · `ResourceProfile` e `WorkDescriptor`», perché un lettore futuro non la conti fra le sonde che mordono a runtime |
| **E6** | Task 3 | ⚠️ **Il Passo 6 cita «R5 di questo piano» per la divergenza della contro-sonda di `Q8`, e non è R5.** `R5` — sia al pre-controllo (riga 129 di questo piano) sia nella tabella di chiusura (righe 4033 e 4113) — nomina una cosa diversa e già presente nel codice di oggi: `V4` e `I2 · §5.3`, le due celle di catalogo che nominano identificatori italiani (`Concessa`/`Rifiutata`/`InCoda`, `InRevoca`). La divergenza della contro-sonda di `Q8` — che il catalogo nomina una «proiezione di presentazione» inesistente — è una voce **distinta e senza sigla propria**, elencata alle righe 4036 e 4117 dello stesso piano. Citarla come R5 avrebbe indirizzato un lettore futuro sulla voce sbagliata. Registrata in `docs/porta-di-qualita.md` senza la sigla `R5`, descrivendo la divergenza per ciò che è |
| **E7** | Task 3 | ⚠️ **Il Passo 4 detta di SPOSTARE `cold_start` dentro `ResourceProfile`, ed è stato eseguito AGGIUNGENDOLO senza toglierlo da `WorkDescriptor`.** Uno spostamento letterale rompe anche `cold_start_is_readable_outside_the_decision_path` e l'aiutante `a_presentation_projection`, che leggono il campo su `WorkDescriptor`: andrebbero rossi per una ragione **estranea** a ciò che il passo misura, e il verde atteso non sarebbe più leggibile. Aggiungendolo, l'esperimento resta quello dettato — *«le sonde a esempi non coprono questa regola»* — e il costo è una chiamata da rattoppare. ✅ Misurato: workspace **verde** sotto la mutazione, mutazione provata applicata e poi revocata, **nessun residuo** (`cold_start` torna a comparire tre volte in `resource.rs`, due in prosa e una come campo di `WorkDescriptor`). 📌 Registrata perché il vincolo globale 14 non ha eccezioni per le divergenze che **non cambiano l'esito**: il precedente è **E3**, che registra uno scarto da un'AZIONE dettata e non da codice dettato |
| **E8** | Task 4 | ⚠️ **La baseline citata dal Passo 2 — «32 target, 194 passati» — è VECCHIA di tre compiti, ed è proprio la cifra contro cui si sarebbe confrontato.** Era la baseline **prima** del Task 1; i Task 1–3 hanno portato otto sonde nuove e un target. Rimisurata su `HEAD` (`0be4ec2`) **prima** di toccare qualunque cosa, con `cargo test --workspace --no-fail-fast --locked`: **33 target, 202 passati, 0 falliti, 2 ignorati**. Il confronto del Passo 2 è stato fatto contro **quella**, e il dopo coincide esatto — lo spostamento di modulo non ha fatto altro che spostare. 📌 È il gotcha **#31** dentro un piano invece che dentro un documento: un numeratore scritto una volta invecchia mentre il compito che lo cita aspetta il proprio turno, e un compito scritto prima **si legge contro il codice di adesso** (vincolo globale 13) |
| **E9** | Task 4 | ⛔ **Il Passo 2 attende un rosso da `cargo build --locked --workspace`, e quel comando NON PUÒ darlo: non compila i banchi d'integrazione.** Misurato: dopo lo spostamento di `Grant` il build esce **verde, exit 0**. Il rosso che prova la decisione **D8** c'è ed è esattamente dove il passo lo colloca, ma lo produce `cargo test`: `` error[E0603]: struct `Grant` is private `` su `crates/kernel/tests/ports_are_implementable.rs:52`, **unico** sito. ⚠️ **E la sigla non è quella che ci si aspetterebbe da un ri-export mancante:** non è un import non risolto (`E0432`) ma una **violazione di privacy**, perché il `use crate::arbiter::Grant;` dentro `ports::process` è privato e lascia il nome *visibile ma vietato* — la prova di D8 è **più forte** così, non più debole. Nessuna modifica al passo, che resta il registro di ciò che fu deciso |
| **E10** | Task 4 | ⛔ **`pub struct Grant { id: GrantId }` accende un warning, e il campo RESTA — decisione del proprietario, presa il 2026-08-19.** Misurato con `cargo build --locked --workspace`: `` warning: field `id` is never read `` su `crates/kernel/src/arbiter/mod.rs`, **una sola** (`GrantId` non è segnalato a sé), con `` note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default ``. **Le tre ragioni, perché stiano a verbale:** ① il campo **non è speculativo** — il suo lettore è il compito successivo e lo detta la decisione **D2** di questo piano: `release` consuma la concessione e deve poter rifiutare quella emessa da un **altro** arbitro (**D3**), che senza identità non è esprimibile; ② ciò che il commento di `Grant(())` rifiutò non era il campo ma `#[allow(dead_code)]`, cioè un divieto spento **in permanenza** — qui non si spegne niente, l'avviso si vede; ③ tornare a unit adesso e rimettere il campo al Task 5 cambierebbe la forma del tipo **due volte** e farebbe **rigenerare due volte** l'oracolo di `grant_has_no_constructor` — in questo repository rigenerare un oracolo è un atto deliberato, non la conseguenza di un ripensamento. ⛔ **Nessun `allow`, nessun lettore inventato, nessun getter.** Il cancello resta **verde**: non passa `-D warnings`. ⏳ **SCADENZA, ed è la parte falsificabile di questa riga: al Task 5 quell'avviso DEVE sparire**, perché `release` legge `id`. Chi esegue il Task 5 lo verifica con `cargo build --locked --workspace` e si aspetta **zero warning**. ⛔ **Se al Task 5 l'avviso è ancora lì, il campo non serviva e questa decisione era sbagliata** — non si mette a tacere, si toglie il campo |
| **E11** | Task 4 | ⛔ **Il comando di verifica del Passo 4 è VACUO, ed è la SECONDA occorrenza della specie già registrata in `E3`.** Il passo detta `git diff --stat crates/kernel/tests/compile_fail/` attendendo *«tre `.stderr` nuovi, nessun altro toccato»*: gli oracoli nuovi nascono **non tracciati**, e `git diff` non li vede — quindi il comando risponde **vuoto** sia che la rigenerazione si sia comportata bene, sia che abbia riscritto un oracolo pre-esistente, e la metà che il passo esiste per cogliere resta invisibile. Sostituito con `git status --porcelain crates/kernel/tests/compile_fail/`. ✅ Misurato: **sei `??`** (i tre `.rs` e i tre `.stderr`) e **nessun ` M`**. 📌 Che la stessa specie sia rientrata in un piano che la registrava già è il dato: `E3` correggeva l'esecuzione, non il **testo** dei passi successivi, che erano già scritti |
| **E12** | Task 4 | ⚠️ **Il commento dettato per `grant_has_no_constructor.rs` prometteva `` `error[E0422]`/`E0423` ``, e l'errore vero NON PORTA NESSUN CODICE.** Misurato leggendo l'oracolo generato: `` error: cannot construct `Grant` with struct literal syntax due to private fields ``, con `` note: private field `id` that was not provided `` — nessun `E0xxx`. Il commento è stato **corretto sulla misura** dentro il caso, a parità di righe perché l'oracolo cita `grant_has_no_constructor.rs:10` e uno spostamento lo avrebbe reso `mismatch`. ⚠️ **Il terzo caso è la controprova che il piano aveva ragione ad astenersi:** per `revoking_a_non_preemptible_grant.rs` il piano dichiara di **non** predire la sigla, e la misura è `` error[E0618]: expected function, found `Activity` `` — che dice *«questa cosa non è una funzione»* invece del *«`NonPreemptible` non porta campi»* che il passo si aspettava a parole: lo stesso fatto detto dall'altro lato. 📌 Gotcha **#15**, in un compito che lo cita: una sigla scritta prima della misura è un'ipotesi travestita da attesa |
| **E13** | Task 4 | ⛔ **La mutazione 2 del Passo 5 NON PUÒ rovesciare il proprio caso, ed è un difetto della mutazione e non dell'esito.** Il passo detta di rendere `pub` il campo di `Grant` attendendo `error`; ma il caso scrive `kernel::arbiter::Grant {}` **senza campo**, quindi resta fermo su `E0063` — e `GrantId` è un tipo **privato** che una crate esterna non può nominare comunque, cioè la strada è chiusa da **due** porte e la mutazione ne apre una sola. ✅ Misurato, applicata e provata entrata con `grep -c "pub id: GrantId"` → `1`: il caso passa da `ok` a **`mismatch`** (non a `error`, e nemmeno resta `ok`), perché il messaggio cambia in `` error[E0063]: missing field `id` in initializer of `Grant` ``. ✅ **Sostituita da una mutazione 2b che esercita davvero ciò che il caso difende** — `pub struct Grant {}`, il tipo svuotato dei campi, che fa compilare il letterale forgiato: misurata **`error`**, con gli altri due casi fermi su `ok`. ⚠️ **E la mutazione dettata non è stata inutile: ha misurato la SPECIE della guardia** — rotta per un verso scatta `mismatch`, cioè **dipende dall'oracolo** (gotcha **#42**), rotta per l'altro scatta `error`. È l'unica riga della tabella delle due direzioni che appartiene a entrambe le specie, ed è registrata in `docs/porta-di-qualita.md` |
| **E14** | Task 4 | ⛔ **`I2 · §5.3` aveva la seconda direzione tenuta da una MUTAZIONE, e una mutazione sparisce quando la revochi: colmata con una sonda permanente.** Il caso negativo tiene *«lo stato illegale non è pronunciabile»*; l'altra metà — *«quello legale SI COSTRUISCE»* — non era tenuta da niente che restasse, e il vincolo globale 6 la pretende (§7.1.1 regola 3). Scritta `a_revocation_is_constructible_on_the_preemptible_side`, che costruisce `Activity::Preemptible(PreemptibleState::Revoking { deadline })` e lo distingue **sia** da `Preemptible(Running)` **sia** da `NonPreemptible`. ⚠️ **`assert_ne!` è lecito qui e non viola R2:** quella restrizione è su `Admission`, che porta un `Grant` e quindi non ha né `Debug` né `PartialEq`; `Activity` li deriva entrambi. ✅ **Provata in negativo PRIMA di crederle, due mutazioni:** appiattendo `Activity` in `NonPreemptible(PreemptibleState)` diventa **rossa a compilazione** (`E0308` più `E0277`, perché la variante è ora una funzione); costruendo `Running` al posto di `Revoking { .. }` diventa **rossa a runtime** (`assertion left != right failed`). Non è decorazione |
| **E15** | Task 4 | ⚠️ **`crates/kernel/tests/arbiter_admission.rs` NASCE UN COMPITO PRIMA di dove il piano lo colloca.** La tabella dei file (riga 211) lo assegna ai Task **5, 6, 7**, e il Passo 1 del Task 5 lo elenca fra i `Create`. È nato qui perché la sonda di `E14` doveva esistere adesso, e ⛔ **non è stata messa in `crates/kernel/tests/arbiter_resource.rs`**: lì vive il vocabolario della **risorsa** — `Mib`, le corsie, il tempo di grazia — e `Activity` non è una risorsa, è ciò che una concessione **sta facendo**. ⛔ **CONSEGUENZA PER CHI ESEGUE IL TASK 5, ed è la quarta domanda del pre-controllo che scatterà da sé:** il suo *«Create»* è un **`Modify`**, e il commento di modulo dettato là va **fuso** con quello che c'è, non sovrascritto — altrimenti sparisce la sola cosa che tiene la seconda direzione di `I2 · §5.3`. L'avvertimento è scritto **dentro il file**, non solo qui, perché un piano lo si rilegge e un sorgente lo si apre |
| **E16** | Task 4 | ⛔ **`V4` era chiuso in `docs/porta-di-qualita.md` sulla direzione «compila» tenuta dalla mutazione 1 del Passo 5, e una mutazione sparisce quando la revochi — la stessa specie che `E14` aveva già colto per `I2 · §5.3` nella riga accanto, e il pre-controllo non l'aveva colta perché il difetto vive nel REGISTRO scritto a chiusura, non nel codice.** Trovato in revisione, non al Passo 5: `Mib::new` è `pub const fn`, quindi `Admission::Refused { asked, ceiling }` è costruibile da un banco d'integrazione, e un `match` che nomina tutte e tre le varianti compila da fuori la crate — esattamente la contro-sonda che il catalogo chiede per `V4`. ✅ **Aggiunta `an_admission_is_distinguishable_three_ways` a `crates/kernel/tests/arbiter_admission.rs`, sonda permanente**: costruisce `Refused` (l'unica via — `Granted` porta un `Grant` senza costruttore pubblico, `Queued` un `TicketId` dal campo privato) e la fa passare per il `match` esaustivo. ⚠️ **Provata in negativo sul CODICE DI PRODUZIONE, non sul test**: la variante `Queued` di `Admission` rinominata `Waiting` in `crates/kernel/src/arbiter/mod.rs` fa rosso a compilazione (`` error[E0599]: no variant or associated item named `Queued` found for enum `Admission` ``), mutazione provata entrata con `grep -c` e revocata con lo strumento di edit. ⛔ **Il morso è dichiarato SOLO a compilazione, e va detto invece che lasciato implicito**: nessun calcolo esiste prima di `admit` (Task 5), quindi i valori `Mib` che il `match` legge echeggiano solo quelli passati in costruzione — non un limite scoperto dopo, un fatto misurato e scritto subito. Registrato in `docs/porta-di-qualita.md`, riga `V4` e il riquadro delle due direzioni |
| **E17** | Task 4 | ⛔ **Il registro dichiarava `a_revocation_is_constructible_on_the_preemptible_side` «rossa a runtime» con una mutazione che editava il LETTERALE DEL TEST, non il codice di produzione — trovato in revisione, stessa specie di `E16`.** Le due `assert_ne!` di quel test confrontano varianti STRUTTURALMENTE distinte di `Activity` (`Preemptible(Revoking{..})` contro `Preemptible(Running)`, e contro `NonPreemptible`), e con la `derive(PartialEq)` che il tipo porta due varianti diverse non sono MAI uguali: nessuna mutazione del codice di produzione può farle fallire a runtime finché la derive resta. Il rosso che il registro citava veniva da `Running` scritto al posto di `Revoking { .. }` **dentro il test stesso** — prova che il letterale conta, non che la sonda sorvegli il codice, la stessa specie del gotcha #42. ✅ **Corretto il registro, non il test**: le due `assert_ne!` restano — non sono decorazione, documentano che `revoking` porta il valore atteso — ma la forza dichiarata ora è quella reale: la costruzione di `Activity::Preemptible(PreemptibleState::Revoking { deadline })`, scritta da fuori la crate, è ciò che chiude `I2 · §5.3`, sullo stesso principio già registrato da `E5` — *«non può fallire a runtime, qualunque cosa faccia il codice di produzione»*. Registrato in `docs/porta-di-qualita.md`, riquadro delle due direzioni sotto la tabella `V4`/`I2 · §5.3` |
| **E18** | Task 5 | ⚠️ **I chiamanti di `Parameters::new` sono DICIANNOVE in QUATTRO file, non «venti siti in sei».** Ricontati prima di cominciare, come il compito stesso impone: `grep -rno "Parameters::new(" crates/ --include=*.rs` → **diciannove occorrenze su diciassette righe** — `crates/daemon/src/main.rs` (1), `crates/kernel/tests/executor_determinism.rs` (10), `crates/kernel/tests/parameters_delivered.rs` (**7 occorrenze su 5 righe** — due righe ne portano due ciascuna), `crates/simulator/tests/dst_campaign.rs` (1). ⛔ **I due file `compile_fail` che il piano elenca fra i chiamanti NON lo sono:** `trust_has_no_default.rs` nomina `Parameters::new` **in un commento** e basta, quindi non è stato toccato; `parameters_have_no_default.rs` non la chiama, ma il suo **oracolo** ne cita la firma verbatim — vedi `E19`. 📌 Il «venti» del piano viene da `grep -rn … \| wc -l`, che conta **righe** e include le tre menzioni in prosa: contare le chiamate e contare le righe che le nominano sono due misure diverse, e il piano le confonde |
| **E19** | Task 5 | ⚠️ **`parameters_have_no_default.stderr` è andato `mismatch`, ed era PREVISTO — dal commento dentro il caso, scritto al Traguardo 2.** rustc chiude `E0599` con una nota che cita la firma di `Parameters::new` **verbatim**, quindi il secondo parametro arriva fino all'oracolo: è la frizione che §2.8.5 promette, che raggiunge anche il `.stderr`. Rigenerato **per la via documentata** — cancellato lo stantio, ri-eseguito, `diff -u` del vecchio contro quello in `wip/`, spostato **a mano** — e ⛔ **mai `TRYBUILD=overwrite`**, che avrebbe portato via gli altri ventisette (vincolo globale 5, gotcha #25). ✅ **Il diff letto è di due righe sole:** la firma e la sua sottolineatura di `^`. La regola che il caso difende scatta come `error` e non attraverso l'oracolo, quindi la rigenerazione **non disarma niente**. ⚠️ **E una misura collaterale che vale la pena scrivere:** quel file nell'albero di lavoro aveva `CRLF` e `admission_reads_cold_start.stderr` no; trybuild scrive `LF`. Con `core.autocrlf=true` i blob indicizzati sono `LF` in entrambi i casi, quindi il diff **di git** è di sole due righe — ma un `diff -u` sui file dell'albero mostrava **undici righe cambiate su undici**. Chi legge il diff di una rigenerazione lo legga con `--strip-trailing-cr`, o crederà a un cambiamento che non c'è (gotcha #48, per una volta senza danno) |
| **E20** | Task 5 | ⛔ **`Held` NASCE CON DUE CAMPI E NON CON QUATTRO, e il difetto del piano è che avverte per `lane` e non per `activity` — che è nella identica situazione.** Il Passo 4 detta `reserved`, `lane`, `activity`, `expires_at`, e avverte che `lane` «non ha lettori in questo compito», prescrivendo l'uscita: *«si sposta il campo al Task 6, dove nasce col proprio consumatore»*. ⚠️ **`activity` non ha lettori nemmeno lui** — il primo è la revoca, Task 7 — quindi con entrambi `cargo build` avrebbe dato **DUE** warning `dead_code`, e ⏳ **la scadenza di `E10` sarebbe stata falsificata dal compito stesso**: quella riga dice *«al Task 5 l'avviso DEVE sparire»*, e sarebbero rimasti due avvisi nuovi al posto di quello vecchio. ✅ **Risoluzione: la stessa uscita che il piano prescrive per `lane`, applicata a entrambi sugli stessi fatti** — `lane` al Task 6 con le code, `activity` al Task 7 con la revoca; nessun `#[allow]` (vincolo globale 4). ⛔ **Conseguenza sul corpo di `admit`, dichiarata perché non è un taglio estetico:** sparisce anche il blocco `activity: match profile.preemption { Never => NonPreemptible, After(_) => Preemptible(Running) }`, che esisteva **solo** per riempire quel campo — tutto il resto di `admit` è verbatim. ✅ Misurato a fine compito: `cargo build --locked --workspace` → **ZERO warning**. 📌 **E una seconda cosa del Passo 4, minore:** il blocco dice *«in coda a `crates/kernel/src/arbiter/mod.rs`»*, ma le sue tre `use` non possono stare in coda — sono state fuse in testa con la `use crate::time::Monotonic;` che c'era, che diventa `use crate::time::{Millis, Monotonic};`. È il gemello di `E4` |
| **E21** | Task 5 | ⚠️ **`a_grant_released_on_the_wrong_arbiter_is_an_error_and_not_a_silent_credit` prova MENO di ciò che il suo nome promette, e il limite è stato scritto invece che appianato.** La sonda costruisce un secondo arbitro **vuoto**, quindi prova *«non è nei miei libri»*, non *«distinguo le mie concessioni da quelle altrui»*: `GrantId` è un progressivo che riparte da zero per ogni `Arbiter`, quindi due arbitri che abbiano **entrambi** emesso concessioni condividono lo spazio degli id, e il secondo accrediterebbe la concessione del primo — cioè esattamente la sovra-ammissione «dalla porta di servizio» che il commento di `ReleaseError` dice di impedire. ⛔ **Il disegno NON è stato cambiato:** dare un'**identità** a `Arbiter` è una decisione del proprietario (vincolo globale 7 per analogia — non è una riga di catalogo, ma è una forma del contratto che il disegno ha fissato). Il limite è scritto **in inglese accanto a `ReleaseError`** nel sorgente e nella sezione del Task 5 di `docs/porta-di-qualita.md`, con ciò che protegge oggi: un processo ha **un** arbitro, e i diversi che esistono insieme esistono nei **banchi** |
| **E22** | Task 5 | ⚠️ **`total_vram` non aveva la propria sonda di livello 2, e la regola di §2.8.4 è sul COSTRUTTORE, non su un campo.** `crates/kernel/tests/parameters_delivered.rs` teneva *«il costruttore non sostituisce nulla per il valore che gli è consegnato»* solo per `executor_turn_limit`: un costruttore che lascia stare quel campo e mette un pavimento sull'altro avrebbe passato l'intero file. ✅ Aggiunta `the_constructor_substitutes_nothing_for_the_total_it_is_handed` — `Mib::ZERO` e `Mib::new(u64::MAX)` tornano indietro identici. ⚠️ **E una riga a `parameters_are_comparable_so_a_substitution_is_observable`:** due `Parameters` che differiscono **nel solo totale**, senza la quale una comparazione cieca al secondo campo avrebbe passato ogni sonda del file e sostituire un totale sarebbe stato **inosservabile** — che è precisamente ciò che la regola 4 di §2.8.2 ha bisogno sia esprimibile. Il file passa da **quattro** a **cinque** test |
| **E23** | Task 5 | ⛔ **`V2` NON È NEL BRIEF DEL TASK 5, e l'innesco che la chiedeva lo aveva scritto il TASK 3 — non nel piano, ma in `docs/porta-di-qualita.md`.** Quella riga dice: *«`V2` resta scoperta … e si chiude insieme a `Q8` allo stesso Task 5»*, e il piano stesso, al Task 3, scrive *«le righe di catalogo che questo compito PREPARA: `V2` … e `Q8 · §5.2.1` … si CHIUDONO al Task 5»*. Il Task 5 porta il caso di `Q8` e **nessun** caso per `V2`. ✅ **Scritto `crates/kernel/tests/compile_fail/admission_without_profile.rs`**, gemello di `executor_without_parameters.rs`: omettere il `&ResourceProfile` da `admit` è un errore di **arità** — misurato `` error[E0061]: this method takes 3 arguments but 2 arguments were supplied ``, con `` argument #1 of type `&ResourceProfile` is missing ``. ✅ **Provato non vacuo** togliendo il `profile` dalla firma di `admit`: il caso torna **`error`**, cioè indipendente dall'oracolo. La contro-sonda esisteva già ed è tutto `arbiter_admission.rs`, che `admit` lo chiama **col** profilo da fuori la crate. ⛔ **Non è una riga di catalogo NUOVA** — `V2` sta in §7.4.1 blocco C fra le scoperte da sempre — quindi il vincolo globale 7 non è toccato: qui si **copre** una riga, non se ne aggiunge una. 📌 **La specie del difetto è la sesta domanda del pre-controllo spostata di un posto:** un compito si legge anche contro gli **inneschi** che i compiti precedenti hanno lasciato nel registro, e nessuna delle sette domande guarda lì |
| **E24** | Task 5 | ⚠️ **La mutazione 1 del Passo 5 dice «il confronto col tetto» e i confronti col tetto sono DUE — misurato, e solo uno uccide la sonda attesa.** `admit` confronta due volte: `asked > ceiling` (*«più grande dell'intera macchina»*) e `self.allocated().saturating_add(asked) > ceiling` (*«la somma non sfora»*). ✅ **Mutando il secondo** — quello che il piano intende, perché è quello che nomina la sonda attesa — `the_sum_of_the_grants_never_exceeds_the_total` muore, e con lei `an_expired_grant_does_not_stay_allocated` e `a_grant_still_inside_its_window_is_not_collected`, che riempiono il totale **esatto** (7 passati, 3 falliti). ⛔ **Mutando il primo, la sonda attesa SOPRAVVIVE** e muoiono solo le altre due (8 passati, 2 falliti). 📌 **Misurate entrambe di proposito:** una campagna che si fosse fermata alla prima avrebbe concluso *«mutazione applicata, sonda morta, fatto»* senza sapere quale delle due guardie aveva toccato — ed è il difetto che il compito stesso chiama *«un banco di misura sbaglia verso l'attesa»*. Nessuna delle due guardie è vacua, ed è quello il risultato. Registrata in `docs/porta-di-qualita.md` come mutazione **1b** |
| **E25** | Task 5 | ⛔ **La mutazione di non-vacuità del Passo 6 dà `mismatch` e non `error`, e misura la SPECIE della guardia di `Q8 · §5.2.1`.** Rimettere `cold_start: Millis` su `ResourceProfile` **non** fa compilare `admission_reads_cold_start.rs`: il letterale del caso resta senza quel campo, quindi rustc passa da `` error[E0609]: no field `cold_start` `` a `` error[E0063]: missing field `cold_start` in initializer of `ResourceProfile` `` e trybuild dice **`mismatch`**. ⚠️ **Cioè la riga `Q8 · §5.2.1` si chiude con una guardia che DIPENDE DALL'ORACOLO** — stessa specie di `mib_as_millis.rs`, gotcha **#42** nella forma debole — e una rigenerazione in blocco la spegnerebbe in silenzio. Non è un difetto scoperto dopo ma il prezzo della forma di questo caso, e sta scritto **sia** nella sezione del Task 5 **sia** nella tabella «Come scattano le due direzioni», perché nessuno la conti fra le forti. ⚠️ **Precedente esatto: la mutazione 2 del Task 4** (`E13`), dove rendere `pub` il campo di `Grant` diede `mismatch` invece di `error` per la stessa ragione — il caso non porta il campo |
| **E26** | Task 5 | ⚠️ **DUE numeri del piano sono nati stantii, e sono la stessa specie in due posti.** ① Il Passo 5 attende *«PASS, otto test»* da `crates/kernel/tests/arbiter_admission.rs`, e ne ha **DIECI**: il file esiste dal Task 4 con due sonde dentro (`E15`), quindi otto nuove su due preesistenti. ② Il Passo 8 attende un conteggio *«non dedotto da 194 più otto»*, ma **194 è la baseline di prima del Task 1** — quattro compiti fa. ✅ Rimisurata su `HEAD` (`0a6f743`) **prima** di toccare qualunque cosa: **34 target, 204 passati, 0 falliti, 2 ignorati**; a fine compito **34 target, 213 passati, 0 falliti, 2 ignorati** — nove in più, le otto sonde dell'ammissione più quella di `E22`, e **nessun target nuovo** perché il file dell'ammissione c'era già. 📌 Stessa specie di `E8`, che aveva già colto lo stesso 194 al Task 4: `E8` correggeva l'esecuzione, non il **testo** dei passi successivi, che erano già scritti |
| **E27** | Task 5 | ⚠️ **Tre commenti portavano il tempo verbale del Task 4, ed è il finding A-2 dell'audit di questo progetto rifatto — riscritti, non lasciati in piedi.** ① Il commento di modulo di `crates/kernel/tests/arbiter_admission.rs` diceva *«CHI ESEGUE IL TASK 5 AGGIUNGE A QUESTO FILE»* al futuro: ora dice che il Task 5 lo ha fatto, e l'avvertimento resta per i Task 6 e 7. ② Il doc di `Grant` diceva *«NON ANCORA UN EMITTENTE … `admit` arriva al Task 5»*: `admit` è nello stesso file, venti righe sotto. ③ Il richiamo dentro lo stesso doc diceva *«nothing constructs one yet»*, ed è la frase che il richiamo stesso esiste per non lasciar marcire. ⛔ **La fusione del commento di modulo è il punto di `E15` e non una rifinitura:** sovrascriverlo avrebbe fatto sparire la sola cosa che tiene la seconda direzione di `I2 · §5.3` |
| **E28** | Task 5 | ⛔ **La guardia *«più grande dell'intera macchina»* è ANTICIPATORIA, e oggi nessuna sonda la tiene.** `if asked > ceiling { return Refused { asked, ceiling } }` (`crates/kernel/src/arbiter/mod.rs`) è **interamente sussunta** da quella sotto: se `asked > ceiling`, allora `allocated + asked > ceiling` è vero **a maggior ragione**, e il valore restituito è **identico**. ✅ **Misurato il 2026-08-19 cancellandola per intero**, `return` compreso — `cargo test --locked -p kernel --test arbiter_admission` → **11 passati, 0 falliti**, inclusa `a_request_larger_than_the_total_is_refused_and_not_queued`, la sola che la nomini; mutazione poi **revocata**, `git diff` vuoto. ⛔ **La guardia RESTA**: è dettata dal Passo 4 e al **Task 6** acquista senso, perché lì il ramo alternativo diventa `Queued` invece di `Refused` e le due guardie cominciano a rispondere **diverso**. Ciò che è stato corretto è il **registro**, che affermava *«nessuna delle due è vacua, ed è quello il risultato»* mentre la mutazione `>` → `>=` prova che morde **il confronto**, non che **la guardia** sia portante. 📌 Specie di `E17`: una guardia registrata per un morso che non ha |
| **E29** | Task 5 | ⛔ **Il confine della scadenza era un MUTANTE VIVO, e la coppia di sonde era presentata come *«le due direzioni complete»*.** `collect_expired` fa `retain(\|_, held\| held.expires_at > now)`; le due sonde della riscossione **scavalcano** il confine senza posarcisi — una guarda a `5_001`, l'altra a `4_999` — e a **`5_000` esatti** non chiedeva nessuno, quindi `>` mutato in `>=` sopravviveva all'intera suite **sulla funzione che quelle due esistono per tenere**. ⛔ **Sonda nuova, non dettata dal piano**: `a_grant_is_collected_at_the_instant_its_window_closes`, che si posa su `now == expires_at` e **dichiara** quale delle due semantiche è quella scelta — finestra **semiaperta**, `[inizio, scadenza)`. ✅ **Provata nelle due direzioni, misurando:** con la mutazione va **rossa** col proprio messaggio (*«at now == expires_at the window is already shut: [start, expiry)»*) ed è **la sola** su tutto il workspace — `cargo test --workspace --no-fail-fast --locked` non porta nessun altro rosso; col codice sano è **verde**. Mutazione applicata, provata entrata con `grep -c`, **revocata**, nessun residuo |
| **E30** | Task 5 | ⛔ **REGISTRATA, NON PRESA — `release` risponde `UnknownGrant` a una concessione PROPRIA che i libri non hanno più, e il nome della variante afferma il falso.** ⚠️ **ALLARGATA IL 2026-08-20 IN REVISIONE DEL TASK 7, E LA RIGA È RISCRITTA E NON AFFIANCATA (`E72`): LE CAUSE SONO TRE E NON DUE.** Dal Task 7 `collect_expired` ha **due** scadenze, quindi una concessione **chiesta indietro** la cui **grazia** è scaduta esce dai libri esattamente come una scaduta di finestra, e `release` su di essa risponde `UnknownGrant` per una **terza** ragione che nessuno dei due paragrafi originali nominava. ✅ **Misurato il 2026-08-20** su una sonda usa-e-getta cancellata subito dopo: chiesta indietro a `0` con grazia `500` e rilasciata a `500` esatti → `Err(UnknownGrant)`; la **stessa** rilasciata a `499` → `Ok(Mib(4096))` — essere in revoca **non** impedisce di restituire, solo la spazzata oltre la scadenza lo fa. Segue il testo originale della voce, che resta valido per le prime due cause. `release` chiama `collect_expired` **prima** di cercare, quindi una concessione con `expires_at <= now` è già stata tolta dai libri: `held.remove` dà `None` e si esce con `Err(ReleaseError::UnknownGrant)`, il cui doc diceva *«This arbiter never issued that grant»*. ✅ **Misurato su una sonda usa-e-getta fuori dal repository**: ammessa per `5_000` ms, rilasciata a `5_001` → `Err(UnknownGrant)`; a `4_999` → `Ok(Mib(4096))`; a `5_000` **esatti** → `Err(UnknownGrant)`. ⛔ **Entrambe le scelte che lo producono sono dettate dal Passo 4** — la riscossione prima della ricerca, e una variante sola — quindi **il disegno non è stato cambiato**: la conflazione è **dichiarata** accanto a `ReleaseError` nel sorgente e come voce aperta nel registro. ⚖️ **La scelta è del proprietario, coi due costi.** ① **Tenere una variante sola** costa che al Traguardo 6 `Worker::kill`, che restituisce la concessione a lavoro **finito** — cioè normalmente **dopo** la finestra — riceva un errore dove non è fallito niente; e l'unica uscita comoda, ignorare l'`Err`, riapre dall'altro lato proprio il credito silenzioso che la guardia esiste per chiudere. ② **Aggiungere `ReleaseError::Expired`** costa un braccio nuovo a ogni `match` fuori dalla crate (attrito voluto, è la forma di `V4`) **e** una riga in `release` che guardi la scadenza **prima** di riscuotere: cioè incrina *«l'arbitro riscuote prima di decidere»* come proprietà di **ogni** operazione, che è esattamente ciò che il Passo 4 comprava. ⛔ **E DAL 2026-08-20 LA SCELTA È PIÙ LARGA DI QUESTE DUE:** con **tre** cause, una variante `Expired` non le separa più — al Traguardo 6 *«il tuo rilascio è fallito»*, *«era già scaduta»* e *«te l'hanno tolta»* sono **tre** notizie diverse, e la terza è l'unica su cui un chiamante possa **agire**, perché essere prelazionati non è la stessa cosa che sopravvivere alla propria finestra. Quindi **anche la forma del rimedio** — una variante nuova, due, oppure un dato dentro la variante — è parte della decisione, e resta del proprietario. ⏳ **Oggi il difetto non morde**: `release` non ha **nessun** consumatore di produzione — tutti i suoi chiamanti sono **sonde**, tutte in `crates/kernel/tests/arbiter_admission.rs`. ⚠️ **LA CIFRA CHE QUESTA RIGA PORTAVA — *«due chiamanti»* — È TOLTA E NON RICORRETTA il 2026-08-20, IN SECONDA REVISIONE DEL TASK 7 (`E77`):** era vera quando il Task 5 la scrisse e il **Task 6 l'ha falsificata nel commit successivo**, dando alle sonde delle code i loro rilasci. Una cifra che vive in **due** documenti si toglie e non si riallinea (gotcha **#68**), e questa non serviva all'argomento: ciò che l'argomento usa è *«nessun consumatore di produzione»*, che è la metà che non marcisce. ⛔ **Il riconteggio che le era stato scritto accanto è tolto anche da qui il 2026-08-20, in terza revisione:** viveva in quattro documenti a un tempo, e adesso sta in **`E77`** e basta (`E85`) |
| **E31** | Task 5 | ⛔ **REGISTRATA, NON PRESA — `saturating_add` può produrre SOVRA-AMMISSIONE al limite superiore.** In `crates/kernel/src/arbiter/mod.rs`, `self.allocated().saturating_add(asked) > ceiling`: con `ceiling = Mib::new(u64::MAX)` e una concessione già a `u64::MAX`, la somma **satura** invece di superare il tetto, e la seconda concessione **passa**. ✅ **Misurato il 2026-08-19 su una sonda usa-e-getta fuori dal repository, e non dedotto** — un'evidenza scritta prima della misura è un'ipotesi: primo `admit` di `u64::MAX` su tetto `u64::MAX` → **`Granted`**, `allocated` = `Mib(18446744073709551615)`; secondo `admit` di **1 MiB** → **`Granted`**, `allocated` **invariato**. La somma vera sarebbe `2^64`. ⚖️ **La scelta è del proprietario.** ① **Lasciare `saturating_add`**: la configurazione è assurda e §5.1 dichiara già la sovra-ammissione come costo di un totale sbagliato — ma `crates/kernel/tests/parameters_delivered.rs` prova **apposta** che `Mib::new(u64::MAX)` arriva **intatto** fino al kernel, quindi il valore è raggiungibile per costruzione e non solo in teoria. ② **`checked_add` che rifiuti su `None`**: costa **una riga** e un ramo, e rende visibile una configurazione impossibile invece che silenziosa — al prezzo di un `Refused` che riporta `asked` e `ceiling` senza dire che il vero motivo è un traboccamento |
| **E32** | Task 5 | ⚠️ **REGISTRATA, NON PRESA — `parameters` e `arbiter` sono ora MUTUAMENTE DIPENDENTI.** `crates/kernel/src/parameters.rs` importa `crate::arbiter::Mib`, e `crates/kernel/src/arbiter/mod.rs` importa `crate::parameters::Parameters`. Legale in Rust — sono moduli, non crate — ed è **dettato dal piano**, che vuole `Arbiter::new(Parameters)` e `total_vram: Mib`. ✅ **Siti contati eseguendo** e non dedotti: **quattro file** devono nominare `kernel::arbiter` solo per **costruire dei `Parameters`**, senza ammettere niente — `crates/daemon/src/main.rs` (1 chiamata), `crates/kernel/tests/executor_determinism.rs` (10), `crates/kernel/tests/parameters_delivered.rs` (11), `crates/simulator/tests/dst_campaign.rs` (1), **23 chiamate in tutto**. ⚖️ **La scelta è del proprietario.** ① **Lasciarlo com'è**: costo zero oggi, e il prezzo è concettuale — i due moduli non sono più separabili in crate distinte, e chi legge `parameters.rs` deve sapere che esiste un arbitro. ② **Spostare `Mib` in un modulo neutro**, sotto `time` o accanto: rompe il ciclo, ma **cambia un percorso pubblico** — ✅ misurato: **sette** oracoli `.stderr` nominano oggi `arbiter`, e `kernel::arbiter::Mib` compare in `daemon`, in quattro banchi e nei casi `compile_fail` |
| **E33** | Task 5 | ⚠️ **Le voci `E18`…`E27` erano state scritte SOTTO la riga vuota che chiudeva la tabella**, quindi in Markdown non rendevano come righe di tabella ma come testo con dei pipe — invisibili a chi legge il piano renderizzato, che è **come lo legge il proprietario**. Verificato col `cat -A` del revisore e corretto togliendo quella riga vuota: le dieci voci sono ora **dentro** la tabella, sopra la riga vuota che la chiude, e queste dieci (`E28`…`E37`) con loro |
| **E34** | Task 5 | ⚠️ **Il doc di `Arbiter` dichiarava *«`BTreeMap` AND `Vec`»* e un `Vec` nella struct NON C'È** — i campi sono `Parameters`, `u64` e `BTreeMap`. Il testo era copiato dal piano, che pensava alle **code** del Task 6. Tolto, con un richiamo datato che dice **perché** torna quando arriveranno le corsie. 📌 È `E20` dall'altro lato: là un campo dettato dal piano e non nato perché non aveva lettore, qui una parola del piano rimasta in un commento a descrivere un campo che non c'è |
| **E35** | Task 5 | ⚠️ **Tre conteggi dei casi `compile_fail` in tre punti del registro, e a fine compito sembravano contraddirsi.** `docs/porta-di-qualita.md` diceva *«gli altri ventisei restano `ok`»* (mutazione 6), *«gli altri ventisette»* (rigenerazione dell'oracolo) e *«da ventisei a VENTOTTO»* (conteggi finali). Non erano in contraddizione ma **due istantanee** di un contenitore che cresceva **dentro lo stesso compito**: sono state **datate** invece che riallineate, perché una misura si registra col proprio momento. ✅ **Casi ricontati eseguendo** e non fidandosi di nessuna delle tre righe: `ls crates/kernel/tests/compile_fail/*.rs | wc -l` → **28** |
| **E36** | Task 5 | ⚠️ **Il commento di `crates/kernel/tests/compile_fail/admission_reads_cold_start.rs` accreditava la CHIAMATA dell'errore.** Diceva che il profilo è costruito *«e poi passato ad `admit` nello stesso `main`, e QUELLO è il punto»*: l'`E0609` nasce invece dal **letterale** e dall'**accesso al campo**, e la chiamata **non partecipa**. ✅ **Misurato cancellandola** su una crate usa-e-getta fuori dal repository — stesso `error[E0609]`, stessa nota coi quattro campi — e l'oracolo accanto nomina **un** errore e **una** riga, quella dell'accesso al campo. Ciò che la chiamata compra è un **accoppiamento alla firma** di grado `mismatch`, non di grado `error`. ⛔ **Riscritto a parità di righe**, perché l'oracolo pinza il **numero di riga** dell'accesso al campo: allungare il commento avrebbe reso il caso `mismatch` e costretto a rigenerare uno `.stderr` che non aveva ragione di cambiare |
| **E37** | Task 5 | ⚠️ **`assert_eq!(second.allocated(), Mib::ZERO, "no silent credit")` NON PUÒ FALLIRE** (`crates/kernel/tests/arbiter_admission.rs`). `release` può solo **rimuovere** dalla mappa, mai inserire, quindi `allocated()` su un arbitro nato vuoto è `Mib::ZERO` qualunque cosa faccia il codice di produzione: a tenere la sonda è la riga sopra, `is_err()`. ⛔ **Non è stata tolta** — dichiara l'intento che il nome porta, e torna a mordere il giorno in cui `release` avesse una via che **inserisce** — ma accanto è scritto **che cosa tiene davvero**. 📌 Stesso rimedio di `E17`, applicato a un'asserzione invece che a una sonda |
| **E38** | Task 5 | ⛔ **A INVECCHIARE non è solo il CONTEGGIO di una misura: è il QUALIFICATORE, e DATARE non lo salva.** Trovato nella **seconda** revisione del Task 5, dopo che le correzioni erano già committate. La tabella delle mutazioni di [`porta-di-qualita.md`](../../porta-di-qualita.md) porta una nota che dichiara *«i conteggi delle righe 1a–7 sono della suite di DIECI sonde … non sono state rimisurate, e il numero nuovo non è dedotto»* — corretta, e **cieca a metà del problema**: la riga **3** diceva *«rossa, e **sola**»* e la **1a** **nominava** le due sonde che cadevano con lei, e l'undicesima sonda (`a_grant_is_collected_at_the_instant_its_window_closes`) riempie anch'essa il tetto **esatto**. ⚠️ **Un conteggio stantio si vede; un *«e sola»* stantio si legge come una GARANZIA**, ed è precisamente l'esclusività la ragione per cui la riga 3 esiste. ✅ **Rimedio: le due celle sono state ACCORCIATE, non riallineate** — resta *«la sonda attesa muore»*, che regge; l'elenco e l'aggettivo se ne vanno, e la nota sotto la tabella dice **perché** e che l'esclusività *«va riconquistata rimisurando, non riscrivendo»*. ⛔ **Nessun numero nuovo è stato scritto**: quante sonde cadano oggi sotto 1a e 3 richiede due mutazioni che non sono state eseguite, e una cifra dedotta sarebbe il gotcha **#15**. 📌 È il gotcha **#31** su un **aggettivo** invece che su una cifra |
| **E39** | Task 5 | ⚠️ **I tre valori misurati di `release` sono documentazione SENZA GUARDIA, e la crate usa-e-getta che li ha prodotti è stata cancellata — dichiarato invece di lasciato implicito.** Il doc accanto a `ReleaseError` in `crates/kernel/src/arbiter/mod.rs` afferma come fatto misurato che il rilascio a `5_001` e a `5_000` dà `Err(UnknownGrant)` e a `4_999` dà `Ok(Mib(4096))`; **nessuna sonda del repository tiene nessuno dei tre**, quindi spostare `collect_expired` dopo la ricerca — una delle **due strade** che `E30` mette davanti al proprietario — non renderebbe rosso nulla e farebbe diventare quel paragrafo **falso in silenzio**. ⛔ **E fissarli con un test è stato SCARTATO sul merito, non per costo:** una sonda che asserisce `Err` a `5_001` congela esattamente il comportamento che `E30` lascia aperto, e il giorno in cui la variante `Expired` arrivasse andrebbe **rossa per aver avuto ragione** — una sonda che va cancellata per prendere una decisione è un voto contro il prenderla. ✅ La scelta e il suo costo sono scritti **accanto al tipo**, dove li legge chi lo tocca, e questa voce è il loro puntatore. 📌 Vale la coppia con `E30`: si chiude quando la decisione si chiude, non prima |
| **E40** | Task 6 | ⛔ **IL RAMO DELLA CODA ROMPE TRE SONDE DEL TASK 5, e il Passo 1 non le nomina.** Dando ad `admit` il ramo della coda, *«entra nella macchina ma non adesso»* smette di rispondere `Refused`: `the_sum_of_the_grants_never_exceeds_the_total`, `a_total_smaller_than_the_two_permanent_quotas_refuses_the_second_one` e `a_grant_still_inside_its_window_is_not_collected` andavano in panico dentro il proprio `let … else`. ✅ **Viste rosse prima di scrivere il codice di produzione, e per il motivo giusto** — **8 passate, 3 fallite** — poi riscritte. ⛔ **Nessuna annacquata:** in tutte e tre l'asserzione che porta la proprietà è su `allocated()`, cioè sul **numero**, e si è mossa solo quella sulla **variante**; i due numeri di `Refused` che la prima pinzava restano pinzati da `a_request_larger_than_the_total_is_refused_and_not_queued`. ⚠️ **Una è stata RINOMINATA** — `a_total_smaller_than_the_two_permanent_quotas_does_not_grant_the_second_one` — perché *«refuses»* nel nome oggi afferma il falso, e un nome che afferma il falso è peggio di una cifra stantia: sta nel **sorgente**. Il puntatore è stato riparato dov'era **vivo** (`docs/porta-di-qualita.md`) e lasciato dov'è un **verbale** (§6 di `COMPENDIO.md`, il corpo del Task 5 in questo piano, `.superpowers/sdd/progress.md`) |
| **E41** | Task 6 | ⛔ **REGISTRATA, NON PRESA — LA SONDA DELLE DUE QUOTE PERMANENTI HA PERSO LA PROPRIETÀ PER CUI ERA STATA SCRITTA, e chiuderla dentro l'arbitro è IMPOSSIBILE.** Fino al Task 5 una configurazione impossibile era **visibile**: la seconda quota permanente tornava `Refused` coi due numeri, ed era la sonda che pagava la divergenza del disegno dalla §5.1. Con le code torna **`Queued`**, e **nessuno la servirà mai** — rilasciare una concessione permanente è esattamente ciò che nessuno fa — cioè il degrado **silenzioso** che ADR-0005 e ADR-0019 vietano. ⛔ **L'arbitro non può ripararlo, e la ragione è già scritta nel sorgente accanto ad `Activity`:** *«Permanence is not a type -- it is nobody calls release»*, quindi l'arbitro non ha modo di distinguere un biglietto che **sarà** servito da uno che non lo sarà mai; una regola che non può valutare non è una regola che può imporre. ⏳ **IL CHIUSORE È IL TASK 10**, la radice di composizione: è **lei** a chiedere le due concessioni permanenti, e un `Queued` là è un fallimento d'avvio che può dire ad alta voce. Scritto accanto alla sonda e in `docs/porta-di-qualita.md`; qui è la voce che lo tiene aperto fino a quel compito |
| **E42** | Task 6 | ⚠️ **DUE COMMENTI DEL TASK 5 SONO DIVENTATI FALSI, e sono stati RISCRITTI e non estesi** — appendere una frase vera sotto una falsa lascia in piedi la falsa, che è il finding **A-2** dell'audit di questo progetto rifatto. ① Il doc di `Held` diceva che `lane` *«arriva con le code al task 6 — un biglietto deve sapere in che corsia aspetta»*: **falso**, e non nella data ma nella premessa. Una richiesta **in attesa** è un `Waiting`, che porta l'intero `ResourceProfile` e quindi la sua corsia, e `promote` percorre `queues` senza **mai** guardare `held`; `Held` è ciò che una **concessione** è, non ciò che un **biglietto** è. Il primo lettore di `lane` è `ask_back` al **Task 7**, lo stesso compito di `activity` — aggiungerlo ora accenderebbe il `dead_code` che `E10` ha stabilito di non spegnere. ⛔ **`Held` resta a DUE campi.** ② Il doc di `Grant` diceva che `Arbiter::admit` è *«the only function in this crate that builds one»*: vero per **un** compito. Riscritto, perché un'esclusività si legge come una **garanzia** (specie di `E38`): ciò che resta vero è che una concessione si **costruisce** in un posto solo, il privato `Arbiter::issue`, e che le due porte pubbliche passano entrambe di lì |
| **E43** | Task 6 | ✅ **`E28` SI CHIUDE, E L'ATTESA SI È AVVERATA — misurata, non dedotta.** Quella voce registrava che la guardia `asked > ceiling` era al Task 5 **interamente sussunta** — cancellata per intero, `return` compreso, **11 passate, 0 fallite** — e dichiarava che *«acquista senso al Task 6, quando il ramo alternativo diventa `Queued`»*. ✅ **La stessa identica mutazione, rieseguita sul codice del Task 6: `a_request_larger_than_the_total_is_refused_and_not_queued` muore, ed è SOLA — 18 passate, 1 fallita.** La guardia non è più sussunta, perché la seconda ora risponde **diverso**: senza la prima, una richiesta più grande dell'intera macchina verrebbe **accodata per sempre**. ⛔ **Eseguita perché `E28` la esigeva** e la tabella delle mutazioni del Passo 4 la ometteva: un'attesa scritta in un'errata è un debito di misura, non una previsione da citare |
| **E44** | Task 6 | ⚠️ **DUE DIVERGENZE NELLA TABELLA DELLE MUTAZIONI DEL PASSO 4, misurate e non appianate.** ① La mutazione **3** — via il controllo `saturating_add(asked) > ceiling` dentro `promote` — doveva uccidere `promote_with_no_room_freed_promotes_nothing` **e** `the_sum_of_the_grants_never_exceeds_the_total`: ✅ **la prima muore, la seconda NO**, e la ragione è leggibile nel codice di oggi — quella sonda **non chiama `promote`**, tiene l'invariante dal lato dell'**ammissione**, quindi una mutazione dentro `promote` non può toccarla. Muoiono invece **quattro** sonde delle code (**15 passate, 4 fallite**). ⛔ **La sonda non è stata allargata per far tornare l'attesa**: allargarla le cambierebbe il soggetto. ② La *«terza mutazione che lascia passare la prima»* che il piano indica — `promote` che promuove **una sola** voce e poi esce — doveva lasciare verde `promote_with_no_room_freed_promotes_nothing` e **rossa** `a_promoted_grant_is_a_grant_like_any_other`: ✅ **la prima resta verde, la seconda ANCHE**, perché ha **una** sola richiesta in attesa e *«una sola»* le basta. La **conclusione** del piano regge — le sonde stanno su assi diversi, e a provarlo sono le mutazioni **1**, **2** e **3c** — ma la sonda che il piano nominava per provarlo no |
| **E45** | Task 6 | ⚠️ **DIVERGENZA DAL CORPO DETTATO DI `promote`: le corsie NON sono un elenco scritto a mano.** Il Passo 3 detta `for lane in [ComputeClass::Realtime, ComputeClass::Interactive, ComputeClass::Batch]`; l'implementazione percorre `queues` con l'iterazione **propria** del `BTreeMap`. ⛔ **Due ragioni sul merito, e nessuna è di gusto.** ① Un elenco scritto lì è l'ordine delle corsie affermato una **seconda** volta, ed è esattamente la trappola per cui `resource.rs` ha rifiutato un `Ord` derivato: *«l'ordine, affermato UNA volta e in un posto solo»*, perché riordinare le varianti cambierebbe le priorità dell'arbitro e **niente andrebbe rosso**. ② Il **doc dettato dal piano stesso** dice *«`BTreeMap` ITERATES IN KEY ORDER … so "best lane first" costs nothing here. That is the coupling the probe `the_lane_order_is_pinned_by_name_and_realtime_comes_first` protects»*: con l'elenco quel paragrafo sarebbe **falso**, perché l'accoppiamento non esisterebbe. ✅ **Provato invece che affermato, mutazione 1b:** cambiata la **chiave** `ComputeClass::priority` (`Realtime => 2`, `Batch => 0`), `the_queue_promotes_by_lane_and_not_in_arrival_order` va **rossa** — l'ordine di `promote` viene davvero dalla chiave. ⚠️ **Il costo, dichiarato:** `issue` prende `&mut self`, quindi il ciclo non può tenere un prestito dentro `self.queues`, e le code sono **spostate fuori** con `core::mem::take` e rimesse alla fine. Per la durata del ciclo `self.queues` è **vuota**: è sano solo perché `allocated` e `issue` leggono e scrivono `held` e `next_grant` e nient'altro, ed è scritto **nel sorgente**, accanto al ciclo, invece che qui soltanto. ⛔ La firma, il tipo di ritorno e il doc restano **verbatim** |
| **E46** | Task 6 | ⚠️ **TRE SONDE OLTRE LE CINQUE DETTATE, e la terza è nata da un MUTANTE VIVO.** ① `promote_does_not_skip_ahead_to_a_smaller_request_behind_a_bigger_one`: il doc di `promote` **dichiara** che si ferma alla prima richiesta che non entra *dentro una corsia*, e nessuna delle cinque dettate lo esercitava — un `promote` che scavalcasse passava tutte e cinque. ② `promote_collects_the_expired_before_it_serves_the_queue`: *«l'arbitro riscuote prima di decidere»* è una proprietà di **ogni** operazione, e con `promote` sono **tre** di cui due coperte; ✅ tolta `self.collect_expired(now)` da `promote`, **nessuna** sonda moriva prima di questa. ③ `promote_serves_every_request_that_fits_and_not_just_the_first`: ✅ **misurato, non temuto** — un `promote` che si ferma dopo **una** promozione per corsia (mutazione **3c**) sopravviveva a **tutte e diciotto** le altre sonde, cioè il plurale del doc era tenuto da niente. Stessa specie e stesso rimedio di `E29`. ⚠️ **E due cifre stantie del piano, ricontate eseguendo:** il Passo 4 attende *«PASS, tredici test»* e il file ne ha **diciannove** (ne aveva **undici** all'inizio del compito, non otto); il Passo 1 apre con un secondo `use kernel::arbiter::Promotion;` a metà file, ed è stato **fuso** nell'importazione in testa, come le due del Task 5 |
| **E47** | Task 6 | ⚖️ **DUE COSE REGISTRATE E NON PRESE, perché la scelta è del proprietario.** ① **`promote` restituisce un `Vec<Promotion>` senza `#[must_use]`**, e `arbiter.promote(now);` da solo **compila**: le concessioni promosse finiscono nei libri senza che nessuno le tenga, la VRAM resta prenotata fino alla scadenza della finestra e il chiamante non ha niente da rilasciare. Costa **una riga** ed è la forma che `Admission` ha già; il motivo per cui non è stata presa è che è un **lint** e non una regola tenibile — produce un avviso e non un errore, quindi nessun caso `compile_fail` può pinzarlo, e aggiungerlo sarebbe superficie che **nessuna** delle due direzioni di §7.1.4 può provare. ② **`Admission::Queued` è RAGGIUNGIBILE A RUNTIME da oggi, e nessuna riga di catalogo lo chiedeva.** `V4` resta coperta esattamente come al Task 5 — i due casi negativi da una parte, `an_admission_is_distinguishable_three_ways` dall'altra — e ciò che il Task 6 aggiunge è **più** di quanto `V4` pretenda: la variante ora esce davvero da un percorso di produzione. ⛔ **Nessuna riga aggiunta al catalogo:** §7.4 è **spec**, e aggiungerne una è decisione del proprietario (vincolo globale **7**) |
| **E48** | Task 6, in revisione | ⛔ **UN MUTANTE VIVO NELL'INTERO WORKSPACE: `promote` non serviva MAI la corsia `Realtime`, e nessuna sonda se ne accorgeva.** ✅ **Misurato sul codice di `47941dd`, non dedotto:** aggiunto `if *lane_key == ComputeClass::Realtime { continue; }` al ciclo delle corsie di `promote`, il workspace resta **verde su tutti e 34 i target — 222 passate, 0 fallite, 2 ignorate**. La causa: `the_queue_promotes_by_lane_and_not_in_arrival_order` contrapponeva **`Interactive` a `Batch`** e basta, e l'unica richiesta `Realtime` del file veniva accodata e mai promossa. ⛔ **Il codice di produzione era GIUSTO: il difetto stava nella PROVA**, cioè nel prodotto principale del compito — *«prima la corsia migliore»* era provato sulla corsia **seconda**, che è proprio quella di cui parlano §5.3.1, M-7 e le quote permanenti di ADR-0033. ✅ **Rimedio: la sonda è stata RISCRITTA a tre corsie**, non affiancata da una seconda — le tre attese arrivano `Batch`, `Interactive`, `Realtime` ed escono rovesciate, con **esattamente due** che entrano nella stanza liberata, così l'ordine è pinzato su entrambe le posizioni servite **e** sulla terza lasciata in coda. ✅ **Rimisurata la stessa mutazione: rossa, e SOLA — 18 passate, 1 fallita.** ⚠️ **La taglia delle attese è scelta per non perdere niente:** con `2_048` ciascuna la sonda continua a morire sotto la **3** e la **3b** come prima, e le dieci mutazioni della campagna sono state **rimisurate tutte da capo** — nessuna cifra si è mossa |
| **E49** | Task 6, in revisione | ⛔ **PC-2 CHIEDEVA LA DIREZIONE NEGATIVA DELLO SCAVALCAMENTO E LA CAMPAGNA NON L'AVEVA MAI ESEGUITA.** Il requisito era che la sonda dello scavalcamento fosse *«provata in negativo da una mutazione di scavalcamento»*; al suo posto la registrazione metteva la mutazione **3** — via il controllo di capienza — che ne uccide **quattro** e quindi non isola niente. ⚠️ **E due punti la davano per fatta senza citare nessuna misura:** il doc della sonda in `crates/kernel/tests/arbiter_admission.rs` (*«un `promote` che scavalcasse passa ogni altra sonda di questo file»*) e la registrazione. ✅ **Adesso misurata, sul codice DOPO la riscrittura di `E48`** — il `break` sulla prima attesa che non entra sostituito da una **scansione** della prima che entra: `promote_does_not_skip_ahead_to_a_smaller_request_behind_a_bigger_one` **rossa, e SOLA — 18 passate, 1 fallita**. La frase resta perché è risultata **vera**, non perché era scritta. ⛔ **E RICONTANDO, LA TESI DI NON-RIDONDANZA ERA SBAGLIATA IN DUE MODI:** diceva *«la 3 ne uccide quattro … qui ce ne sono tre … nessuna delle quattro è ridondante»* — ① tre mutazioni isolanti nominate per **quattro** sonde; ② una delle tre, la **3c**, isola una sonda che fra le quattro **non c'è**. Il conto vero: la **1** (e la **1c**) isolano quella delle corsie, la **2** quella dell'ordine d'arrivo, la **3d** quella dello scavalcamento, e `promote_with_no_room_freed_promotes_nothing` **non è isolata da niente** — anzi muore **solo** dove muore anche quella dello scavalcamento (mutazioni 3, 3e, 4, 5b), cioè è **dominata**. ⚠️ **Cercata una mutazione che la isolasse — la `3e`, lo scarto silenzioso — e la previsione è stata SMENTITA dalla misura:** prevista sola, ne uccide **tre** (16 passate, 3 fallite). Registrato invece che appianato; la sonda **resta** perché dice un'altra cosa, ma la sua non-ridondanza è ora scritta come **non misurata** invece che affermata |
| **E50** | Task 6, in revisione | ⛔ **REGISTRATA, NON PRESA — LO SCAVALCAMENTO CHE `promote` RIFIUTA DENTRO UNA CORSIA, FRA CORSIE LO FA, con un'inversione di priorità sopra.** Il doc di `promote` giustificava la regola d'arresto dicendo che scavalcare *«lascerebbe una richiesta grande in una corsia affollata aspettare per sempre dietro le piccole»*: fra corsie il codice produce **esattamente quello**, perché una corsia che si ferma **cade sulla successiva**. ✅ **Misurato il 2026-08-19 su una sonda usa-e-getta fuori dal repository:** macchina `4_096` con `bulk` `3_072` e `small` `1_024` residenti, un'attesa **`Realtime`** da `4_096` accodata **prima** di un'attesa **`Batch`** da `1_024`; rilasciata `small`, `promote` promuove la **`Batch`** e lascia la `Realtime` in coda. ⛔ **Il comportamento NON è stato cambiato**, ed è dettato dal Passo 3 — il corpo detta il `break` interno e la caduta sulla corsia successiva — ed è **work-conserving**, quindi difendibile: l'alternativa tiene la macchina ferma per un'attesa che potrebbe non entrare mai. ⚖️ **Il difetto era che la conseguenza non stava scritta da nessuna parte:** §5.3, §5.3.1 e `design/02-arbitrato-gpu.md` non dicono niente sull'ordine **fra** corsie. Rimedio in due pezzi, nessuno dei quali tocca il comportamento: la frase del doc è **circoscritta** a *dentro una corsia* e dice per esteso che cosa succede fra corsie, e questa voce mette la scelta davanti al **proprietario**. ⛔ **AGGIUNTA IL 2026-08-19, seconda revisione: quella frase del doc NON È TENUTA DA NIENTE, e adesso lo DICE.** ✅ **Rimisurato di persona, non copiato dal rilievo:** fermata l'**intera passata** alla prima corsia la cui testa non entra — bandiera sul `break` interno, poi `break` sul ciclo esterno, cioè **nessuna caduta** sulla corsia successiva — **niente va rosso**: `arbiter_admission.rs` **19 passate, 0 fallite** e il workspace **34 target, 222 passate, 0 fallite, 2 ignorate** (campagna, riga **1d**). Mutante **vivo**, quindi il paragrafo diventa **falso in silenzio** se il Task 7 o il Task 10 cambiano l'ordine fra corsie. ⚖️ **Non pinzato apposta:** una sonda che congelasse la caduta congelerebbe **questa** scelta. Vedi `E53`. |
| **E51** | Task 6, in revisione | ⛔ **REGISTRATA, NON PRESA — `admit` NON CONSULTA MAI LA CODA, quindi un ritardatario la scavalca.** `admit` legge `held` e `parameters` e basta: se la stanza c'è quando viene chiamata, dice sì, qualunque cosa stia già aspettando. ✅ **Misurato il 2026-08-19 su una sonda usa-e-getta fuori dal repository:** macchina `4_096` piena, un biglietto **`Realtime`** da `4_096` in coda, rilasciato il residente; una **nuova** richiesta `Batch` da `4_096` che passa da `admit` è **`Granted` all'istante**, e il `promote` successivo torna **vuoto** col `Realtime` ancora in attesa. ⛔ **Conseguenza sulla registrazione, che è stata corretta:** *«l'ordine per corsia è ciò che tiene validi i numeri di M-7»* è vero **di `promote`** e non dell'arbitro — l'ordine d'**ammissione** lo sconfessa. ⚖️ **Perché non è chiusa qui:** farlo significherebbe un `admit` che **rifiuta stanza che esiste**, cioè una politica di scheduling che nessun ADR ha deciso; e *quando* si chiama `promote` rispetto ad `admit` è **orchestrazione**, cioè Task **10**. ⚠️ **E interagisce direttamente con `E41`:** una quota permanente accodata non solo non sarà mai servita, ma può vedersi passare davanti qualunque richiesta arrivata dopo. Stesso chiusore, il Task 10. ⛔ **AGGIUNTA IL 2026-08-19, seconda revisione: anche QUESTA frase del doc non è tenuta da niente, e adesso lo dice.** ✅ **Rimisurato di persona:** con `!self.queues.is_empty() \|\|` aggiunto alla seconda guardia di `admit` — il ritardatario si **accoda dietro** chi aspetta invece di scavalcarlo — **niente va rosso**: **19 passate, 0 fallite** nel file e **34 target, 222 passate, 0 fallite, 2 ignorate** nel workspace (campagna, riga **4b**). Mutante **vivo**, quindi il paragrafo diventa **falso in silenzio** il giorno in cui il Task 10 decidesse nell'altro senso. ⚖️ **Non pinzato apposta**, stessa ragione di `E39`. Vedi `E53`. |
| **E52** | Task 6, in revisione | ⚠️ **DUE PUNTI PICCOLI, entrambi su frasi che affermavano più di quanto tenessero.** ① **Il doc di `promote` nominava la sonda SBAGLIATA come custode dell'accoppiamento** — diceva *«è l'accoppiamento che la sonda `the_lane_order_is_pinned_by_name_and_realtime_comes_first` protegge»*, ma quella sonda pinza i valori di `priority()` e le tre relazioni `<`, e **resta verde** se `promote` smette del tutto di usare l'ordine della mappa. A tenere la **dipendenza** di `promote` dalla chiave è `the_queue_promotes_by_lane_and_not_in_arrival_order`, che è ciò che la mutazione **1b** dimostra davvero. ⛔ **La frase è verbatim dal Passo 3 e `E45` ci si appoggia**, quindi le due sonde sono **nominate entrambe** dicendo che cosa tiene ciascuna, invece di scambiarne una con l'altra. ② **`E47` ① affermava *«`arbiter.promote(now);` da solo compila»* senza citare nessuna misura**, dove `E30` ed `E31` dicono entrambe *«misurato su una sonda usa-e-getta fuori dal repository»*. ✅ **Misurato così, e nelle DUE direzioni:** com'è, quella riga compila e la crate chiamante resta verde **anche con `-D warnings`** (nessun lint scatta), e la promozione avviene davvero — dopo quella riga `allocated()` legge `Mib(4096)` e `queued()` legge `0`; con `#[must_use]` messo su `promote`, esce `warning: unused return value of Arbiter::promote that must be used` e la compilazione **riesce lo stesso, exit 0**. ⛔ **Ed è la misura ② che rende sano il resto del ragionamento di `E47` ①, che era corretto:** essendo un **avviso**, nessun caso `compile_fail` può pinzarlo, e un caso che dichiarasse `#![deny(unused_must_use)]` per farlo diventare errore proverebbe che **il lint morde dove è dichiarato**, non che il kernel dichiari l'attributo — gotcha **#39** esatto, la specie per cui è dovuto nascere `gate-attributes.sh`. La scelta resta del proprietario |
| **E53** | Task 6, in revisione | ⛔ **DUE FRASI DI DOC CHE NESSUNA SONDA TIENE, E DUE MUTANTI VIVI — DICHIARATE e non pinzate, ed è una decisione sul merito.** La passata precedente ha chiuso `E50` ed `E51` **scrivendo il comportamento in un doc**, e nessuno dei due paragrafi è tenuto da niente. ✅ **Rimisurati di persona il 2026-08-19, non copiati dal rilievo che li segnalava:** ① fermando l'**intera passata** di `promote` alla prima corsia la cui testa non entra — bandiera sul `break` interno, poi `break` sul ciclo esterno, cioè **nessuna caduta sulla corsia successiva** — `arbiter_admission.rs` dà **19 passate, 0 fallite** e il workspace **34 target, 222 passate, 0 fallite, 2 ignorate**; ② aggiungendo `!self.queues.is_empty() \|\|` alla seconda guardia di `admit` — il ritardatario si **accoda** invece di scavalcare — **le stesse identiche cifre**, 19/0 e 34 · 222 · 0 · 2. Entrambi **vivi**, e nessuna cifra diverge da quelle del rilievo. Ogni mutazione: entrata provata col conteggio delle occorrenze, **compilata in un passo separato** da quello che esegue, revocata da copia byte-esatta con `cmp` identico e `git diff` **vuoto**; zero CR prima e dopo. ⛔ **Il rimedio è DICHIARARLE non tenute, e pinzarle è stato scartato sul merito:** una sonda che congelasse la caduta fra corsie, o che asserisse `Granted` per il ritardatario, congelerebbe **esattamente** le due politiche che `E50` ed `E51` mettono davanti al proprietario, e *«una sonda che va cancellata per prendere una decisione è un voto contro il prenderla»* — è la ragione di `E39`, gotcha **#73**. ✅ **La dichiarazione sta in tre posti:** accanto ai due paragrafi **nel sorgente**, dove la legge chi li tocca; nelle due voci aperte di [`porta-di-qualita.md`](../../porta-di-qualita.md); e dentro `E50` ed `E51`, che sono le voci che il **proprietario** legge. ⚠️ **E le due mutazioni sono entrate nella campagna come `1d` e `4b`** — da tredici a **quindici** — perché il registro vuole **ogni** mutazione col proprio esito misurato, anche quando l'esito è *«non uccide niente, ed è voluto»* |
| **E54** | Task 6, in revisione | ⚠️ **UNA CELLA DEL TASK 5 AFFERMAVA IL CONTRARIO DELLA SEZIONE DEL TASK 6, ed era rimasta PENDENTE dopo che il proprio innesco era scattato.** In [`porta-di-qualita.md`](../../porta-di-qualita.md) la riga di `a_request_larger_than_the_total_is_refused_and_not_queued` diceva **al presente** che a consegnare i due numeri è *«la seconda guardia … misurato cancellando `asked > ceiling` per intero → 11 passati, 0 falliti»*, e chiudeva con ⏳ *«l'innesco è il Task 6»*. ⛔ **Il Task 6 quell'innesco l'ha fatto scattare:** la mutazione **5** e la voce `E43` misurano la stessa identica mutazione a **18 passate, 1 fallita** con quella sonda **sola**, e la sezione del Task 6 la chiama *«oggi l'unica custode di quella guardia»*. ✅ **Rimedio in due mosse, e nessuna cancella una misura:** la frase causale è **circoscritta al Task 5** e il conteggio `11 passati, 0 falliti` **resta**, perché è un'istantanea di allora (`E35` — una misura si registra col proprio momento); il marcatore ⏳ diventa un **rimando** a `E43` e alla sezione del Task 6. ⚠️ **La passata precedente aveva riparato due puntatori stantii in questo stesso file e lasciato in piedi l'unica cella che il Task 6 aveva reso FALSA**, ed è la specie del gotcha **#68**: un marcatore pendente il cui innesco è scattato si legge come ancora pendente |
| **E55** | Task 6, in revisione | ⚠️ **UN'ESCLUSIVITÀ LARGA NEL PARAGRAFO DELLA DOMINANZA, ed è la specie di `E38`.** Per giustificare che la sonda dominata `promote_with_no_room_freed_promotes_nothing` **resti**, il registro la diceva *«l'unica che chiami `promote` su una macchina esattamente piena senza che sia stato rilasciato niente»*: ⛔ **anche `promote_collects_the_expired_before_it_serves_the_queue` chiama `promote` senza nessun `release`**, ed entra con i libri a `4_096` — la stanza le arriva dalla **riscossione**. ✅ **Verificato sul sorgente e non ricordato:** sono le **sole due** del file a chiamare `promote` senza rilasciare. L'**intento** era esatto — *«l'unica a cui la macchina non regali stanza»* — l'esclusività **come scritta** no. ✅ **Rimedio: la frase è stata RISTRETTA a ciò che è vero** — *«su una macchina che non guadagna stanza in nessun modo, né da un rilascio né da una riscossione»* — e **non** riparata aggiungendo una cifra, perché un qualificatore stantio si legge come una **garanzia** mentre un conteggio stantio si può sempre ricontare (`E38`) |
| **E56** | Task 6, in revisione | ⛔ **IL COMMENTO DI MODULO DI `crates/kernel/tests/arbiter_admission.rs` ORDINA AL TASK 6 DI FONDERE, E IL TASK 6 NON L'HA FATTO.** Quel commento enumera i soggetti del file — *«Two things at task 4, and the ADMISSION itself from task 5: …»* — e tre paragrafi sotto dice che il Task 5 **fuse** invece di sovrascrivere e che *«the same still goes for tasks 6 and 7»*: il Task 6 ha aggiunto **otto** sonde su un soggetto **nuovo** — le code, `promote`, `queued` — senza toccarlo. ⚠️ **È una regola che non lega il documento che la ospita**, cioè il gotcha **#68**. ✅ **Rimedio: l'enumerazione è stata ESTESA** col soggetto del Task 6 — l'accodamento invece del rifiuto, l'ordine **per corsia** e dentro la corsia per arrivo, *«serve tutte quelle che entrano»*, la riscossione prima del servizio, e la promozione che è una concessione come le altre — ⛔ **niente è stato sovrascritto**, l'istruzione di fusione **resta viva per il Task 7**, e accanto sta il richiamo datato che dice che al Task 6 era stata saltata |
| **E57** | Task 7 | ⛔ **LE SONDE DI `ask_back` NON POSSONO STARE IN `tests/`, E IL RIMEDIO È IL PRIMO `#[cfg(test)]` DI `kernel`.** `ask_back` resta `pub(crate)` come il Passo 3 detta, e un `pub(crate)` **non si vede da un test d'integrazione**, che è una crate a sé: ✅ **misurato di persona e non dedotto** — una chiamata da `crates/kernel/tests/arbiter_admission.rs` esce `` error[E0624]: method `ask_back` is private ``, sito unico, e la sonda usa-e-getta è stata **revocata** con `cmp` identico. ✅ **E la fattibilità del modulo è stata misurata PRIMA di scriverlo**, perché `kernel` è `#![no_std]` e il banco di prova vuole `std`: un `#[cfg(test)] mod tests` usa-e-getta compila e gira **senza** `extern crate std`, `cargo test -p kernel --lib` → 1 passata. ⛔ **Dieci sonde su undici si spostano lì**, con la deviazione dichiarata accanto al modulo nella forma di `crates/platform/src/rng.rs`, che sta in `src/` per la stessa specie di ragione. ⚠️ **La undicesima NON si sposta:** `a_grant_that_is_neither_expired_nor_revoking_survives_the_sweep` non chiama `ask_back` e non ha bisogno di niente di privato — **solo la privatezza sposta una sonda**, e muoverla per compagnia spaccherebbe un soggetto su due file senza comprare niente. ⛔ **E la via d'uscita facile è stata rifiutata sul merito:** rendere `ask_back` `pub` per tenere le sonde in `tests/` pubblicherebbe un'operazione che non lo è — fare spazio è la **conseguenza** di una richiesta, mai una cosa che qualcuno chiede — ed è lo stesso argomento che tiene privata `collect_expired` e che ha rifiutato un costruttore `pub(crate)` per `Grant` (gotcha **#67**) |
| **E58** | Task 7 | ⛔ **`Held` GUADAGNA TRE CAMPI E NON UNO, e il piano ne annuncia uno solo.** Il Passo 3 avverte che `Held` *«guadagna il proprio tempo di grazia»*, ma il corpo dettato legge anche `held.activity` e `held.lane`, che il Task 5 rimandò (`E20`) e il Task 6 non portò (`E42`) proprio perché *«il primo lettore è `ask_back` al Task 7»*. Quindi i campi sono **`lane`, `activity` e `grace`**, e `issue` li riempie tutti e tre dal profilo — `activity` e `grace` **dallo stesso** `profile.preemption`, che è ciò che li tiene d'accordo: un `Held` prelazionabile senza grazia, o non prelazionabile con una, **non è costruibile da lì**. ⛔ **Il doc di `Held` è stato RISCRITTO e non affiancato:** diceva *«TWO FIELDS, AND THE OTHER TWO ARRIVE WITH THEIR OWN READERS»* e portava due richiami datati, tutti e tre ormai spesi — appendere una frase vera sotto una falsa lascia in piedi la falsa, che è il finding **A-2** dell'audit di questo progetto rifatto. ⚠️ **E `held_grace` NON È STATO SCRITTO.** Il piano lo chiama *«l'accessore»*, ma `Held` è una struct **privata nello stesso modulo** e `held.grace` è già raggiungibile: una funzione libera che restituisse un campo sarebbe superficie che nessuno compra (YAGNI, vincolo globale). Il campo si legge direttamente |
| **E59** | Task 7 | ⛔ **`a_non_preemptible_grant_is_never_asked_back` COME DETTATA PASSAVA PER LA RAGIONE SBAGLIATA — gotcha #74.** La sonda mette il residente in corsia `Realtime` e chiede sotto `Interactive`: la guardia `lane <= below → continue` lo scarta **da sola**, prima che la prelazionabilità sia guardata, quindi il meccanismo che il nome promette non gira **mai** e la sonda sarebbe verde anche con la guardia sull'attività cancellata. ✅ **Rimedio: il residente va in corsia `Batch`**, strettamente sotto quella che chiede, così a salvarlo può essere **solo** la guardia sulla grazia — e la mutazione **8** lo dimostra, uccidendola **da sola**. ⚠️ **Anche il nome del profilo è cambiato:** `"audio-reserved"` in corsia `Batch` è una frase falsa dentro un banco, ed è diventato `"batch-that-is-never-preempted"`. ⛔ **La contro-sonda per l'altra strada resta intatta:** `only_lanes_below_the_asking_one_are_asked_back` tiene il caso `Realtime` con un profilo **prelazionabile**, quindi le due direzioni ci sono entrambe e ciascuna esercita **una** guardia |
| **E60** | Task 7 | ⚠️ **LA SONDA DETTATA `a_grant_that_is_neither_expired_nor_revoking_survives_the_sweep` ASSERIVA `Refused`, E OGGI LA RISPOSTA È `Queued`.** Il Passo 1 scrive `assert!(matches!(refused, Admission::Refused { .. }))` su una richiesta di `4_096` con tetto `4_096`: non è *«più grande dell'intera macchina»*, quindi dal Task 6 *«entra nella macchina ma non in questo momento»* è un **biglietto**. È la stessa specie delle **tre** sonde che il Task 6 dovette riscrivere, e il compito è stato letto **contro il codice di adesso** e non contro il piano. ✅ **La sonda discrimina lo stesso, ed è misurato:** sotto la mutazione **5d** — la spazzata che riscuote una concessione `Running` — è la **sola** rossa del proprio file, 19 passate 1 fallita |
| **E61** | Task 7 | ⚠️ **`ask_back` RISCUOTE LO SCADUTO IN TESTA, E IL CORPO DETTATO NON LO FACEVA.** Il doc di `collect_expired` dichiara *«l'arbitro riscuote prima di decidere»* una **proprietà di ogni operazione** — è la ragione per cui è privata — e l'errata `E46` le diede una sonda per `promote` proprio su questo. Con `ask_back` le operazioni sono **quattro**, e il corpo dettato ne lasciava una fuori. ✅ **Verificato prima di aggiungerla che non cambi l'esito di nessuna sonda dettata**, e non supposto: tutte e cinque chiedono a `Monotonic::ORIGIN`, dove non c'è niente da riscuotere. La sonda è `ask_back_collects_the_expired_before_it_marks`, per analogia con `promote_collects_the_expired_before_it_serves_the_queue` e con la stessa costruzione — **nessun `release`** dentro, così l'unica cosa che può svuotare i libri è la riscossione. ✅ Mutazione **9**: tolta la riga, **rossa e sola**, 9 passate 1 fallita |
| **E62** | Task 7 | ⛔ **LE DUE GUARDIE DI `ask_back` DIVERGONO DAL CORPO DETTATO, E LA DIVERGENZA È UNA MISURA.** Il corpo del Passo 3 avvolge tutto in `if let Activity::Preemptible(PreemptibleState::Running)` e legge la grazia con `match held.grace { Some(g) => …, None => continue }`: siccome `issue` riempie `activity` e `grace` dallo **stesso** `profile.preemption`, le due guardie rispondono **entrambe** per il caso non prelazionabile e **si mascherano a vicenda**. ✅ **Misurato installando la forma dettata**, che è **verde** su tutte e trenta le sonde — le due forme sono equivalenti, quindi ciò che le distingue non è il comportamento ma cosa si riesce a **provare**: ① cancellando la guardia `Running` muore `asking_back_twice_does_not_buy_the_room_twice` e **solo lei**, 9 passate 1 fallita, mentre `a_non_preemptible_grant_is_never_asked_back` **resta verde**, salvata dal ramo `None`; ② trasformando `None => continue` in `None => now` **non muore niente**, 10 passate e 20 passate — **mutante vivo**, il ramo è **irraggiungibile**. ⛔ **Quindi sulla forma dettata la direzione *«una concessione non prelazionabile non si chiede indietro»* è tenuta da due guardie e provabile da nessuna delle due.** ✅ **La forma scritta separa le DOMANDE:** `matches!(… Revoking { .. }) → continue` chiede *«è già in uscita»*, `let Some(grace) = held.grace else { continue }` chiede *«si può chiedere indietro»* — `None` è la parola che `Preemption::grace` usa per *«questo profilo non si revoca mai»*, non un valore mancante. Ciascuna ha la propria sonda e il proprio uccisore **solo**: mutazioni **7** e **8**, 9/1 entrambe. ⚠️ **Il precedente è `ComputeClass::Ord` scritto a mano** — *«togliere la trappola batte sorvegliarla»* — e non è `ReleaseError`, dove chiudere il buco vorrebbe una decisione del proprietario: qui si tratta del corpo di una funzione **privata nata in questo compito**, senza contratto pubblico né riga di catalogo |
| **E63** | Task 7 | ⚠️ **L'ORDINE DELLE CORSIE NON È SCRITTO A MANO, e il corpo dettato lo scriveva.** Il Passo 3 percorre `for lane in [ComputeClass::Batch, ComputeClass::Interactive, ComputeClass::Realtime]`, con un commento che dice *«`BTreeMap` dà l'ordine delle chiavi, quindi questo lo rovescia»* — e la frase è **falsa**: `held` è chiavato per `GrantId`, non per corsia, quindi lì non c'è nessun ordine da rovesciare. ⛔ **Peggio, sarebbe l'ordine dichiarato una TERZA volta:** il doc di `promote` rifiuta esattamente quell'elenco — *«sarebbe quell'ordine dichiarato una seconda volta, cioè la trappola che `resource.rs` ha tolto rifiutando un `Ord` derivato»* — e una regola che non lega la funzione accanto è la specie del gotcha **#68**. ✅ **Scritto invece con un `BTreeSet<ComputeClass>` delle corsie che i libri contengono**, percorso `.rev()`: l'ordine viene dalla chiave `priority()` di §5.1 e da nient'altro, e le corsie vuote non si percorrono. ⚠️ **E «prima la corsia peggiore» era una frase tenuta da NIENTE:** è nata la sonda `asking_back_takes_the_worst_lane_first`, con le due prenotazioni **diverse** apposta perché `revoking()` conta e non nomina. ✅ Mutazione **10** — `.rev()` tolto: **rossa e sola**, 9/1. ⚖️ **L'ordine non è arbitrario e non va dal proprietario:** la tabella delle corsie di `design/02` dice `interattivo` *«servita prima di `batch`»* e `batch` *«può attendere indefinitamente»* |
| **E64** | Task 7 | ⛔ **LA MUTAZIONE 5 DELLA TABELLA NOMINA UNA SONDA CHE IL DIFETTO NON ATTRAVERSA, E CON LE SOLE SONDE DETTATE ERA UN MUTANTE VIVO.** La riga dice *«il ramo `Revoking` diventa `false`»* → `a_grant_that_is_neither_expired_nor_revoking_survives_the_sweep`, ma nello scenario di quella sonda **non c'è nessuna revoca**: il suo residente passa dal ramo `_`. ✅ **Misurato: muoiono DUE sonde, ed entrambe sono fra le NON dettate** — `a_grant_inside_its_grace_keeps_its_reservation` e `asking_back_twice_does_not_buy_the_room_twice` — 8 passate 2 fallite, con `arbiter_admission.rs` a **20 passate 0 fallite**. ⛔ **La causa è che nessuna sonda dettata SPAZZA dentro la grazia:** `allocated()` non riscuote — lo dichiara il suo doc — e `ask_back` neanche, quindi l'`assert_eq!(arbiter.allocated(), 4_096)` scritto subito dopo un `ask_back` non passa da **nessuna** spazzata e non può fallire. La sonda nuova fa spazzare un `admit` a `499` su una grazia che scade a `500`. ⚠️ **E i rami del `match` sono DUE, quindi le mutazioni sono due:** il ramo `Revoking` (righe **4**, **5**, **5c**) e il ramo `_` (righe **5b** e **5d**), ciascuna col proprio esito. ⛔ **Più il confine, che è la specie di `E29` sulla SECONDA scadenza:** le due sonde della grazia chiedono a `501` e a `499`, su `500` esatti non chiedeva nessuno, e `deadline > now` mutato in `>=` sarebbe sopravvissuto all'intera suite. Sonda `the_grace_runs_out_at_the_instant_of_its_deadline`, che **dichiara** anche quale semantica è la scelta — grazia semiaperta `[chiesta, scadenza)`, la stessa regola della finestra di validità e non una seconda. ✅ Mutazione **5c**: **rossa e sola**, 9/1 |
| **E65** | Task 7 | ⚠️ **LA DOMANDA DEL GOTCHA #66, MISURATA, E IL PIANO NOMINAVA LA SONDA SBAGLIATA.** Il Passo 4 chiede di mutare `expires_at <= now` in `expires_at < now` e di guardare `a_grant_still_inside_its_window_is_not_collected`, temendo che con la riscossione forzata dentro la stessa spazzata quella sonda smetta di discriminare. ✅ **Misurato: resta VERDE, e NON perché sia diventata vacua** — chiede a `4_999` su una finestra di `5_000`, e sotto quella mutazione fa la **stessa identica cosa**: quel confine non l'ha **mai** tenuto. ✅ **La sola morta è `a_grant_is_collected_at_the_instant_its_window_closes`** — la sonda che il Task 5 aggiunse con `E29` proprio per quel confine, e che il piano non conosce — `arbiter_admission.rs` **19 passate 1 fallita** e il bersaglio `lib` **10 passate 0 fallite**. ⛔ **Quindi la risposta alla domanda del #66 è: nessuna vacuità introdotta**, e il confine della finestra di validità resta tenuto da chi lo teneva già |
| **E66** | Task 7 | ⚠️ **LE IMPORTAZIONI DETTATE DAL PASSO 1 NON COMPILANO.** Il passo apre con `use kernel::arbiter::{Activity, PreemptibleState};` e `use kernel::time::Millis as Grace;`, e tutti e tre i nomi sono **già** in testa a `crates/kernel/tests/arbiter_admission.rs` dal Task 4: ✅ misurato, `` error[E0252]: the name `Activity` is defined multiple times ``. **Fuse invece che aggiunte** — l'alias `Grace` sparisce e l'aiutante scrive `Preemption::After(Millis::new(grace))` — che è la stessa istruzione che il commento di modulo di quel file impartisce a ogni compito dal Task 5. ⚠️ **E quel commento è stato ESTESO, non sovrascritto**, con il soggetto del Task 7 e con **dove** stanno le dieci sonde e perché: al Task 6 l'istruzione era stata saltata (`E56`), e restava viva per questo compito |
| **E67** | Task 7 | ⛔ **DUE AVVISI `dead_code` RESTANO IN PIEDI ALLA FINE DEL COMPITO — REGISTRATI PER IL PROPRIETARIO, NON SPENTI, ED È LA FORMA ESATTA DI `E10`.** ✅ Misurato con `cargo build --locked --workspace`: `` warning: fields `lane` and `grace` are never read `` e `` warning: method `ask_back` is never used ``, due sole, con `` note: `#[warn(dead_code)]` … on by default ``. ⛔ **La causa: `ask_back` è `pub(crate)` e il suo unico chiamante di produzione nasce al Task 8**, e un modulo `#[cfg(test)]` **non conta** per `dead_code` in una build non di test — quindi la *«finta in un banco»* che il Passo 3 indica come risposta del repository al gotcha **#46** dal verso sbagliato **esiste** (le dieci sonde) ma non fa tacere l'avviso. `lane` e `grace` cadono con lui: `activity` no, perché `revoking()` è `pub` e lo legge. ⛔ **Le vie d'uscita, tutte rifiutate sul merito:** un `#[allow]` è un divieto spento (gotcha **#13**, vincolo globale 4); un lettore inventato è superficie che nessuno compra; rendere `ask_back` `pub` pubblicherebbe un'operazione che non lo è e romperebbe la decisione del proprietario del Passo 3; dare a `ask_back` un chiamante di produzione **adesso** significherebbe prendere la decisione fra policy LOCALE e REMOTA, che è il Task 8. ⛔ **Il cancello resta VERDE — non passa `-D warnings` — e il compito NON è stato fermato** perché le tre ragioni di `E10` valgono parola per parola: il metodo non è speculativo, il suo lettore è **dettato dal compito successivo di questo stesso piano**, e niente viene messo a tacere. ⏳ **SCADENZA, ed è la parte falsificabile di questa riga: al Task 8 quei due avvisi DEVONO sparire.** Chi lo esegue lo verifica con `cargo build --locked --workspace` e si aspetta **zero warning**. ⛔ **Se al Task 8 sono ancora lì, `ask_back` non serviva e questa decisione era sbagliata** — non si mette a tacere, si toglie il metodo. ⚖️ **E se il proprietario preferisce non averli mai:** l'alternativa è **fondere il Task 7 col Task 8** in un solo commit, che è una decisione sua e non del compito che se ne accorge |
| **E68** | Task 7 | ⚠️ **TRE CIFRE STANTIE, E UNA DIVERGENZA SUI BERSAGLI CHE VA NELL'ALTRO VERSO.** ① Il Passo 4 attende *«PASS, diciannove test»* e `arbiter_admission.rs` ne aveva **già diciannove** prima del compito; ne ha **venti** dopo, perché dieci delle undici sonde del Task 7 vivono altrove (`E57`). ② La baseline in testa al piano — *«32 target · 194 passati»* — è di sei compiti fa: ✅ **rimisurata prima di cominciare**, `cargo test --workspace --no-fail-fast --locked` → **34 target · 222 passate · 0 fallite · 2 ignorate**, e a fine compito **34 · 233 · 0 · 2**. ③ ⛔ **E il pre-controllo avvertiva che un `mod tests` dentro `src` fa nascere un bersaglio di prova NUOVO, quindi che i target potessero muoversi: NON si sono mossi, e la divergenza si registra invece di appianarla** — `cargo test --workspace` eseguiva già `unittests src\lib.rs` per `kernel` con **zero** test, e oggi ne esegue **dieci**. Il bersaglio c'era, era vuoto. ✅ **Tutto ricontato eseguendo il binario, mai dedotto** (gotcha **#31**); `compile_fail` resta a **ventotto**, contati col comando, e nessun oracolo è stato rigenerato |

| **E69** | Task 7, in revisione | ⛔ **`ask_back` MARCAVA VITTIME ANCHE QUANDO IL BISOGNO NON SAREBBE STATO COPERTO, ED È CORRETTO — non registrato aperto, e la distinzione è il punto.** Il ciclo marcava finché `covered >= needed` **oppure** finché le corsie finivano, senza guardare prima se il recuperabile bastasse. ✅ **Riprodotto misurando, non ragionato:** macchina `8_192`, un `Batch` prelazionabile da `2_048` e un `Batch` **non** prelazionabile da `6_144`, `ask_back(4_096, Interactive, ORIGIN)` → `` left: Mib(2048), right: Mib(0) ``, bersaglio `lib` **11 passate, 1 fallita**. Il `Batch` era **condannato** — alla scadenza della grazia `collect_expired` lo toglie dai libri — e chi chiedeva **non si sedeva comunque**: lo stesso danno che il doc della funzione dichiara di evitare (*«evicts two jobs to seat one»*) preso dall'altra strada, uno sfrattato e nessuno seduto. ⛔ **Perché si corregge invece di registrarla aperta come `E50`/`E51`:** quelle sono **politiche** senza risposta giusta — chi va servito prima — e pinzarle sarebbe un voto contro il deciderle (gotcha **#73**); questa è un'operazione che **muta lo stato e riporta fallimento lasciando la mutazione lì**, cioè il degrado silenzioso che **ADR-0005** e **ADR-0019** vietano. ✅ **La forma: DUE PASSATE.** Una in **sola lettura** somma il recuperabile fra i candidati ammissibili, e **solo se copre `needed`** parte la seconda, che marca e si ferma appena il bisogno è coperto; se non copre, **non si marca nulla** e la risposta è `Mib::ZERO`. ⛔ **Nessuno stato nuovo, nessun tipo nuovo, e il criterio di ammissibilità è scritto UNA volta:** una `let askable = \|held: &Held\| -> Option<Millis>` che entrambe le passate interrogano — una copia verbatim del predicato sarebbe il difetto che questa correzione esiste per non introdurre. ⚠️ **CHIUSURA E NON METODO, E LA RAGIONE È UNA MISURA:** `ask_back` è `dead_code` fino al Task 8, e ciò di cui è l'unico chiamante muore con lui — ✅ misurato installandolo come `Held::askable_by`, `cargo build --locked --workspace` stampa **tre** avvisi, il terzo `` method `askable_by` is never used ``, cioè un avviso in più di quelli che il proprietario ha accettato (`E67`). Una chiusura è parte del corpo e non ne aggiunge nessuno. ✅ **Le dieci sonde preesistenti restano tutte verdi** sotto la forma nuova, verificato **eseguendo** e non a mano: `lib` **12 passate, 0 fallite**, `arbiter_admission.rs` **20, 0**. Sonda nuova `asking_back_marks_nothing_when_the_reclaimable_does_not_cover_the_need`; mutazioni nuove **12** (il controllo di capienza tolto → quella sonda, **rossa e SOLA**, 11/1) e **12c** (`reclaimable < needed` → `<=`, la seconda direzione: **7 passate, 5 fallite**, cadono tutte e cinque le sonde a incastro esatto) |
| **E70** | Task 7, in revisione | ⚖️ **DENTRO UNA CORSIA LA VITTIMA È LA PIÙ VECCHIA, E NESSUN DOCUMENTO LO DECIDE — DICHIARATA, NON PINZATA, e la ragione è la distinzione che `E69` stabilisce.** La passata che marca percorre `held`, che è chiavata per `GrantId`, quindi incontra le concessioni di una corsia **nell'ordine in cui sono state emesse** e prende la prima ammissibile: con un `Batch` da `4_096` e uno da `512` e un bisogno di `512` marca il `4_096`, sfrattando 4 GiB per liberarne mezzo. L'alternativa — **la più piccola che basta** — è altrettanto difendibile, e §5.3, §5.3.1 e design/02 tacciono: la tabella delle corsie decide l'ordine **fra** corsie, non **dentro** una. ⛔ **Quindi è una politica senza risposta giusta e va davanti al proprietario, non dentro una sonda:** una sonda che asserisse *«la più vecchia»* congelerebbe la scelta, e una sonda da cancellare per prendere una decisione è un voto contro il prenderla — è la forma di `E39`, `E50`, `E51`, `E53`. ✅ **MISURATO il 2026-08-20, e il mutante è VIVO:** con la passata che marca su `self.held.values_mut().rev()` — la **più recente** per prima — **niente va rosso**, `lib` **12 passate, 0 fallite** e `arbiter_admission.rs` **20, 0** (riga **13** della campagna). Il paragrafo sta nel doc di `ask_back` **col mutante vivo accanto**, così il giorno in cui qualcuno cambia l'ordine la frase diventa falsa **senza che nulla lo dica** — e questo è dichiarato invece che scoperto |
| **E71** | Task 7, in revisione | ⛔ **IL CONFINE `lane <= below` NON ERA TENUTO DA NESSUNA SONDA, ed è la specie di `E29` su una terza guardia.** Delle dieci sonde del `mod tests` **nessuna** metteva il residente nella **stessa** corsia dell'argomento `below`: otto `Batch` contro `below = Interactive`, una `Realtime` contro `Interactive`, una `Interactive`+`Batch` contro `below = Realtime` — tutte **strettamente** sopra o **strettamente** sotto, quindi il confine non veniva chiesto da nessuno. ✅ **Riprodotto misurando e non ragionato:** `lane <= below` mutato in `lane < below` è sopravvissuto all'**intero workspace** — `lib` **10 passate, 0 fallite**, `arbiter_admission.rs` **20, 0**, e ogni altro bersaglio verde. ⛔ **E il caso è quello che il Task 8 produce per PRIMO**, non una curiosità: l'ammissione chiede indietro **sotto la propria corsia**, quindi un pari nella corsia che chiede è la prima cosa che `ask_back` si vedrà passare, e sfrattare un pari per un pari è esattamente ciò che *«only lanes BELOW»* esclude. Sonda nuova `a_grant_in_the_asking_lane_itself_is_not_asked_back` — residente **prelazionabile** in `Interactive`, `below = Interactive`, che asserisce `Mib::ZERO` **e** `revoking() == 0`; riga nuova **2b** della campagna, `<=` → `<`: ✅ **rossa e SOLA**, 11/1 e 20/0 |
| **E72** | Task 7, in revisione | ⛔ **IL DOC DI `ReleaseError::UnknownGrant` DICEVA «DUE CAUSE» E DA QUESTO COMPITO SONO TRE — RISCRITTO IN TUTTI E TRE I PUNTI, NON AFFIANCATO.** `collect_expired` da oggi toglie dai libri anche una concessione la cui **grazia** è scaduta, quindi un `release` su di essa risponde `UnknownGrant` per una ragione che né il paragrafo del *«secondo limite dichiarato»*, né quello delle *«due cause conflate»*, né la riga della variante nominavano. ⛔ **Una frase di doc diventata falsa si RISCRIVE:** appenderne una vera sotto è il finding **A-2** dell'audit di questo progetto rifatto, e i tre punti portano ora un **richiamo datato** che dice cosa dicevano prima e perché il conteggio era giusto fino al Task 7. ✅ **La terza causa è MISURATA e non dedotta**, su una sonda usa-e-getta cancellata subito dopo: chiesta indietro a `0` con grazia `500`, rilasciata a `500` esatti → `Err(UnknownGrant)`, e a `499` → `Ok(Mib(4096))`. ⛔ **E NESSUNA SONDA LI PINZA, per la ragione di `E39`:** quei tre valori sono la base della decisione aperta `E30`, e una sonda che li congelasse andrebbe cancellata per prenderla. `E30` è stata **allargata** di conseguenza: con tre cause una sola variante `Expired` non le separa più, quindi la **forma** del rimedio è essa stessa parte della decisione del proprietario |
| **E73** | Task 7, in revisione | ⚠️ **UNA CELLA DEL REGISTRO ERA DIVENTATA FALSA, E LA CAMPAGNA È STATA RIMISURATA DA CAPO.** ① In [`porta-di-qualita.md`](../../porta-di-qualita.md) la cella di `promote_collects_the_expired_before_it_serves_the_queue` diceva **al presente** *«con `promote` le operazioni sono tre»*, mentre la sezione del Task 7 dice già **quattro**: due cifre al presente che si contraddicono nello stesso file. ✅ **Rimedio quello di `E54`: la frase è CIRCOSCRITTA al proprio compito** — *«al Task 6 … erano tre»* — e la misura di allora resta intatta, invece di essere riallineata. ⛔ **Un puntatore che vive in più punti si toglie o si data, non si ricorregge** (gotcha **#68**). ② ⛔ **E tutte le diciotto righe della campagna del Task 7 portavano i conteggi di una suite che non esiste più** — `10 / 20` — perché questa passata porta il `mod tests` da **dieci** a **dodici** sonde. Rimisurate **tutte da capo** invece che datate, che è ciò che la revisione del 2026-08-19 fece al Task 6 per la stessa ragione (gotcha **#31**). ⛔ **E la rimisura ha trovato una DOMINANZA NUOVA, scritta invece che taciuta:** la mutazione **8** — la guardia sulla grazia tolta — era *«rossa e SOLA»* e adesso ne uccide **due**, perché il residente non prelazionabile della sonda nuova di `E69` diventa recuperabile; quindi `a_non_preemptible_grant_is_never_asked_back` **non ha più un uccisore solo** ed è dominata da `asking_back_marks_nothing_when_the_reclaimable_does_not_cover_the_need`. Una candidata isolante è stata cercata e non trovata: le due sonde differiscono per la **presenza di un secondo candidato**, e nessuna mutazione della guardia distingue i due casi. Registrato come **intenzione** e non come fatto, esattamente come le tre dominate del Task 7. ③ ⚠️ **Le tre righe `D0`/`D1`/`D2` NON sono state rimisurate ma DATATE:** misurano la forma **dettata dal Passo 3**, che con le due passate non esiste più in una forma confrontabile; la conclusione che ne dipende (`E62`, le due guardie che si mascherano) resta valida e le guardie restano **tre e separate** dentro `askable` |
| **E74** | Task 7, in revisione | ⏳ **LA SCADENZA DI `E67` ORA STA ANCHE ACCANTO AL CODICE, che è la forma di `E10` e ciò che a `E67` mancava.** I due avvisi `` fields `lane` and `grace` are never read `` e `` method `ask_back` is never used `` **restano**, ed è una decisione del proprietario presa il 2026-08-20: sono la conseguenza di `ask_back` `pub(crate)` con le sonde in un `#[cfg(test)] mod`, che per `dead_code` non conta come lettore. ⛔ **Ma una scadenza che vive solo nell'errata è una scadenza che chi lavora nel sorgente non vede**, e `E10` al Task 4 ce l'aveva **accanto al campo**. Aggiunto quindi in coda al doc di `ask_back`, in inglese: al **Task 8** quei due avvisi **devono** sparire perché nasce il chiamante vero, e **se sono ancora lì il metodo non serviva e si toglie** — non si mette a tacere. Nessun `#[allow]`, nessun lettore inventato, nessun `pub` di comodo |
| **E75** | Task 7, in seconda revisione | ⛔ **LA PASSATA CHE MARCA SI FERMAVA ALLA PRIMA CORSIA SENZA CHE NIENTE LO DICESSE, ed è `E69` PRESA DA UNA TERZA STRADA.** La riscrittura in due passate ha lasciato nel corpo una **promessa** — *«It cannot run out of candidates before `covered` reaches `needed`: the pass above has already added up the same set»* — che **nessuna sonda teneva**. La passata di sola lettura somma **tutte** le corsie, quindi può promettere `6_144`; una passata che marcasse solo nella prima risponderebbe `2_048` **col titolare di quella corsia già condannato**: uno sfrattato e nessuno seduto, cioè il danno che `E69` esiste per togliere. ⛔ **Nessuna sonda ci arrivava, e la ragione è strutturale e non casuale:** sette avevano un residente solo, due ne avevano due nella **stessa** corsia, una esce dal ramo `reclaimable < needed` prima che una corsia sia percorsa, e l'unica a **due** corsie — `asking_back_takes_the_worst_lane_first` — chiede `1_024` contro un `Batch` da `2_048`, quindi la corsia peggiore le basta da sola. ✅ **Riprodotto misurando e non ragionato:** `lanes.iter().rev()` ridotto a `.take(1)`, applicato e **compilato in un passo separato**, è sopravvissuto all'**INTERO WORKSPACE** — **34 target, 235 passate, 0 fallite, 2 ignorate**. ⚖️ **E NON è una politica aperta**, quindi l'argomento di `E39`/`E70` — *«una sonda da cancellare per decidere è un voto contro il deciderla»* — **non si applica**: c'è una risposta giusta, ed è che la passata arrivi in fondo. ✅ **Sonda nuova `asking_back_crosses_into_the_next_lane_when_the_worst_one_is_not_enough`**: macchina `8_192`, `Interactive 4_096` e `Batch 2_048` entrambi prelazionabili, `below = Realtime`, `ask_back(4_096, …)` → **`6_144`** e `revoking() == 2`. ⚠️ **Pinza anche una seconda cosa che nessuno diceva: la marcatura può ECCEDERE** — `6_144` per un bisogno di `4_096` — cioè si ferma alla prima concessione che porta **oltre** la linea e non ne cerca una che ci **atterri**. Riga **14** della campagna: ✅ **rossa e SOLA, 12 passate 1 fallita** |
| **E76** | Task 7, in seconda revisione | ⚠️ **L'INSIEME DELLE CORSIE NON ERA QUELLO DEI CANDIDATI, quindi la frase *«la passata di sopra ha già sommato lo stesso insieme»* era una PROMESSA e non una costruzione.** `lanes` raccoglieva la corsia di **ogni** concessione, ammissibile o no, quindi una corsia priva di candidati veniva percorsa a vuoto. ⚠️ **QUESTA VOCE DICEVA *«misurato nelle DUE direzioni»* E CIÒ CHE ERA MISURATO ERA UNO SCENARIO, PRIMA E DOPO — riscritta in terza revisione, non affiancata** (`E84`). Le due direzioni ci sono ora davvero, con la stessa strumentazione — `lanes.len()` restituito da lì, su sonde usa-e-getta cancellate subito dopo: **toglie ciò che deve** — un residente `Realtime` che `below = Interactive` esclude più un candidato `Batch` → **`Mib(2)`** corsie prima del filtro e **`Mib(1)`** dopo — e **non toglie nient'altro** — `Interactive 4_096` e `Batch 2_048` entrambi candidati sotto `below = Realtime` → **`Mib(2)`** prima e **`Mib(2)`** dopo, cioè una corsia che ha un candidato non viene mai sottratta alla passata che marca. ⚖️ **Innocuo di per sé, e NON è per questo che si corregge:** un `filter(|held\| askable(held).is_some())` nella stessa catena rende i due insiemi **lo stesso insieme per costruzione**, cioè **toglie la distanza** che il commento avrebbe dovuto promettere via — il rimedio strutturale che questo repository preferisce alla sorveglianza (*«togliere la trappola batte sorvegliarla»*, il precedente di `ComputeClass::Ord`). ⛔ **E NON sostituisce la sonda di `E75`, e non è stato fatto al suo posto:** la struttura toglie la distanza fra i due insiemi, la sonda uccide il mutante che **salta una corsia**. Sono due difetti diversi e nessuno dei due copre l'altro — misurato, perché la riga **14** è rossa **anche** col filtro installato. ⚠️ **E che sia innocuo ha un prezzo, misurato e dichiarato dalla terza revisione: il filtro è un mutante vivo GARANTITO**, quindi la frase che compra sta al presente e non la tiene nessuna sonda (`E83`, riga **15**) |
| **E77** | Task 7, in seconda revisione | ⚠️ **DUE CIFRE STANTIE AL PRESENTE, E TUTTE E DUE SONO STATE TOLTE INVECE CHE RIALLINEATE.** ① Il doc dell'unica sonda del Task 7 rimasta in `crates/kernel/tests/arbiter_admission.rs` diceva *«the other ten probes of task 7»* quando erano **dodici**, e a dirlo era il commento di modulo **dello stesso file** trenta righe più su — che nel paragrafo immediatamente successivo **rivendicava di aver appena riscritto quel numero**. ⛔ **Due cifre al presente che si contraddicono in un file solo è il gotcha #31; una rivendicazione di completezza falsificata dal file che la ospita è il #68, e pesa di più.** La copia è **tolta**, la cifra vive in **un posto solo** — il commento di modulo, portato a **tredici** — e la sonda ci **rimanda**. ② ⛔ **E il `grep` fatto PRIMA di correggerla ne ha trovata un'altra che nessuno cercava — «una casa trovata non è tutte le case».** Il doc di `ReleaseError` diceva *«`release` has TWO callers in this repository»*: ✅ ricontati col comando, **nove** siti in **otto** sonde. Era vera al Task 5 (commit `681798e`) e il **Task 6 l'ha falsificata nel commit successivo**. **Tolta e non ingrandita**, in tutti e due i posti in cui viveva — il sorgente e `E30` — perché l'argomento ha bisogno di *«nessun consumatore di produzione»*, che è la metà che non marcisce. ⛔ **E IL RICONTEGGIO SCRITTO AL SUO POSTO È FINITO A SUA VOLTA IN QUATTRO DOCUMENTI**, sorgente compreso, sotto un titolo che dichiarava di non rimpiazzare la cifra con una più grande: dalla terza revisione **vive in questa voce e basta** — il *«nove siti in otto sonde»* di qui sopra — e il sorgente, `E30` e il registro ci **rimandano** invece di ripeterlo (`E85`) |
| **E78** | Task 7, in seconda revisione | ⚠️ **IL DOC DI `Held::grace` DICEVA *«see the two guards there»* E LE GUARDIE SONO TRE — RISCRITTO, non affiancato.** La prima ondata del 2026-08-20 ha portato la guardia sulla **corsia** dentro il criterio di ammissibilità, e da quel momento questa frase e la chiusura `askable` — che dice *«THREE QUESTIONS AND NOT ONE, AND THEY STAY THREE»* — erano due cifre al presente che si contraddicevano nello stesso file (gotcha **#31**). ✅ **Verificato leggendo il codice e non il commento:** `askable` ha **tre** uscite — corsia, *«è già in uscita»*, grazia — e la campagna ha una mutazione **per ciascuna**: la **2**/**2b**/**2c** per la corsia, la **7** per la seconda, la **8** per la terza. Il richiamo datato dice che cosa la frase diceva prima e da quando è falsa |
| **E79** | Task 7, in seconda revisione | ⚠️ **UN'ASSERZIONE CHE NON PUÒ FALLIRE — TENUTA, non cancellata, ed è la specie di `E37`.** In `asking_back_marks_nothing_when_the_reclaimable_does_not_cover_the_need`, l'`assert_eq!(arbiter.allocated(), Mib::new(8_192))` è vera per **qualunque** implementazione di `ask_back`: quella funzione non toglie **mai** dai libri — solo `collect_expired` lo fa, e allo `ORIGIN` con finestre `LONG` non c'è niente da riscuotere. ✅ **Riprodotto misurando e non dedotto**, isolandola (le due asserzioni sopra tolte) e rieseguendo tre mutazioni: resta **verde** sotto la **12**, che è il suo **uccisore solo** — 13 passate 0 fallite, dove a piena forza è 12/1 — sotto la **8** (12/1, e a morire è un'altra) e sotto la **1** forte (**6/7** isolata **e** 6/7 a piena forza, e questa sonda non è fra le morte). ⚠️ **QUESTA RIGA DICEVA `9/4` PER LA MUTAZIONE 1, ED È RISCRITTA E NON AFFIANCATA:** `9/4` è la forma **debole** che `E80②` ripudia nello stesso commit — il `retain` in coda, saltato dall'uscita anticipata — mentre la riga **1** della campagna, sempre nello stesso commit, dice **6/7**. Rimisurata il 2026-08-20 in terza revisione (`E81`). ⛔ **Il rimedio di questo repository NON è cancellarla:** dichiara l'intento che il **nome** porta — che i libri non si sono mossi. ⛔ **MA LA RAGIONE SCRITTA ACCANTO ERA FALSA, ED È RISCRITTA (`E82`):** questa voce concludeva che *«a tenere la sonda sono le due asserzioni sopra»* e che la terza *«tornerà a mordere il giorno in cui `ask_back` avesse una via che tocca i libri»* — al futuro. **Morde oggi:** sotto le mutazioni **5b** e **5d**, che toccano `collect_expired`, le prime due passano e questa è **l'unica che scatta**. L'isolamento qui sopra campionava solo mutazioni di `ask_back`, cioè la classe sotto cui non può fallire |
| **E80** | Task 7, in seconda revisione | ⚠️ **LA CAMPAGNA È STATA RIMISURATA DA CAPO PER LA SECONDA VOLTA NELLO STESSO GIORNO, e la rimisura ha corretto una MUTAZIONE oltre ai numeri.** ① La suite passa da **dodici** a **tredici** sonde nel bersaglio `lib`, quindi tutte le venti righe portavano di nuovo i conteggi di una suite che non esiste più (gotcha **#31**); rimisurate **tutte**, mai dedotte. Le tre righe **`D`** restano l'unica eccezione, perché misurano la forma **dettata dal Passo 3** che con le due passate non esiste più in versione confrontabile (`E35`). ⚠️ **QUESTA RIGA DICEVA CHE SONO «DATATE» E CHE ERA *«✅ verificato che la data ci sia»*, E NON PORTANO UNA DATA: riscritta in terza revisione, non affiancata** (`E86①`). Portano *«Misura del Task 7, non rimisurata»*, cioè un riferimento al **compito**. ✅ **Il contenuto è sano e anzi migliore di una data, ed è per questo che si corregge la RIVENDICAZIONE e non le celle:** le due ondate del 2026-08-20 hanno la stessa data e il Task 7 no, quindi il riferimento al compito le distingue dove una data non lo farebbe. ② ⛔ **La riga 1 era una mutazione più DEBOLE di quella che la sua cella descrive:** il `retain` era stato messo solo **in coda**, dove l'uscita anticipata `return covered` lo saltava — misurata così dà **9/4** invece di **6/7**. Rifatta con `break 'lanes`, perché la riscossione avvenga **sempre**. Una mutazione che non fa ciò che la sua riga dice è un mutante vivo travestito da misura. ③ ✅ **La riga 13 (`values_mut().rev()`) è stata RIMISURATA contro il codice di oggi e non citata** — era stata scritta **prima** della riscrittura in due passate — ed è **ancora un mutante vivo**: `lib` **13 passate, 0 fallite**, `arbiter_admission.rs` **20, 0**. La politica resta aperta (`E70`). ④ ⛔ **UNA PERDITA DI ESCLUSIVITÀ, RIMEDIATA NELLO STESSO GIRO:** la sonda nuova di `E75` muore anche sotto la **10**, quindi `asking_back_takes_the_worst_lane_first` ha perso l'uccisore solo — e **non** era per questo dominata, il suo insieme non è contenuto in nessun altro. ✅ **Candidata isolante cercata e TROVATA — riga 10b**, *«una vittima più grande del bisogno viene saltata»*: **rossa e SOLA, 12 passate 1 fallita**. Le due sonde condividono lo scenario e differiscono per la **taglia del bisogno**, non per l'ordine — quindi nessuna mutazione dell'**ordine** poteva separarle, e una della **scelta della vittima** sì; in più è l'unica riga che eserciti *«marcare può eccedere»*. Bilancio **a chiusura di questa ondata**: **nove** sonde su **quattordici** con uccisore solo (erano otto su tredici), campagna a **venticinque** righe — la terza ne ha aggiunta una, la **15** (`E83`) |
| **E81** | Task 7, in terza revisione | ⚠️ **QUATTRO CONTEGGI DI UNA SUITE CHE NON ESISTE PIÙ, IN SORGENTE VIVO, E UNO DI ESSI FALSO DUE VOLTE.** L'annotazione accanto alla terza asserzione di `asking_back_marks_nothing_when_the_reclaimable_does_not_cover_the_need` portava `12+0`, `11+1`, `11+1` e `8+4`: **sommano tutti a dodici**, mentre il bersaglio `lib` di `kernel` ha **tredici** sonde. Contraddicevano la riga **12** della campagna e `E79` **nello stesso commit**, e il marchio *«MEASURED on 2026-08-20»* non disambiguava niente, perché **entrambe** le ondate portano quella data. ⛔ **E l'`8/4` era la mutazione 1 nella forma DEBOLE che `E80②` ripudia nello stesso commit** — il `retain` messo solo in coda, che l'uscita anticipata saltava — quindi era falso per la **suite** e per la **mutazione**. ✅ **Rimisurati tutti e quattro il 2026-08-20 invece che circoscritti, perché qui la rimisura è possibile:** isolata la terza asserzione (le due sopra tolte), la sonda resta verde sotto la **12** — **13 passate, 0 fallite**, a piena forza **12/1** — sotto la **8** — **12/1**, e a morire è un'altra — e sotto la **1** forte — **6/7 isolata e 6/7 a piena forza**, identiche, perché lì la sonda non muore comunque. ⚠️ **La circoscrizione resta il rimedio delle sole righe `D`**, dove la forma misurata non esiste più; qui esiste, e una misura rifatta batte una datata. 📌 **Il conteggio nuovo è comunque circoscritto al proprio momento** con la forma che il file già usa cento righe più su — *«against the suite OF THAT MOMENT, thirteen probes here and twenty in `tests/arbiter_admission.rs`»* — così la prossima sonda lo invecchia **visibilmente**. Corretto anche il `9/4` che `E79` portava per la stessa mutazione debole |
| **E82** | Task 7, in terza revisione | ⛔ **LA TESI SCRITTA ACCANTO A QUELL'ASSERZIONE ERA FALSA, E LA CAMPAGNA DELLO STESSO COMMIT LO PROVAVA.** Il commento concludeva *«SO WHAT HOLDS THIS PROBE IS THE TWO ABOVE»* e dava la terza asserzione per vacua, *«che torna a mordere il giorno in cui `ask_back` avesse una via che tocca i libri»* — **al futuro**. ⛔ **Morde oggi.** ✅ **Riprodotto misurando, e la misura guarda una cosa che nessuna delle precedenti guardava: QUALE asserzione va in panico**, non se la sonda sia rossa. La sonda muore sotto **5b**, **5d**, **8**, **11**, **12**, e non per la stessa via: sotto **8** e **12** scatta la **prima**, la risposta (`Mib(8192)` e `Mib(2048)` contro `Mib(0)`); sotto **11** la **seconda**, lo stato (`1` contro `0`); sotto **5b** e **5d** — le due che mutano `collect_expired` invece di `ask_back` — le prime due **passano** e scatta **solo la terza**: `left: Mib(0), right: Mib(8192)` sotto la **5b** (`lib` **4/9**, `arbiter_admission` **7/13**) e `left: Mib(6144), right: Mib(8192)` sotto la **5d** (`lib` **5/8**, `arbiter_admission` **19/1**). Su quelle due righe è **questa** asserzione a uccidere la sonda, da sola. 📌 **LA CAUSA VALE OLTRE IL CASO:** la misura d'isolamento citata a sostegno campionava **solo** mutazioni di `ask_back`, cioè precisamente la classe sotto cui quell'asserzione non può fallire, e non ha mai eseguito le due righe in cui è portante — **un'esclusività misurata su un campione parziale si legge come una garanzia**. ✅ **Il rimedio scelto — tenerla — resta giusto, ed era la RAGIONE a essere sbagliata, e sbagliata per DIFETTO:** l'argomento vero non ha bisogno di un giorno futuro, perché la spazzata che `ask_back` chiama per prima è già una via che tocca i libri. Riscritte la frase nel sorgente, quella del registro e `E79` |
| **E83** | Task 7, in terza revisione | ⚖️ **IL `filter` SULL'AMMISSIBILITÀ IN `lanes` È UN MUTANTE VIVO GARANTITO — DICHIARATO ACCANTO ALLA FRASE, come `E70` fa col proprio.** Il filtro non ha comportamento: le corsie che toglie non contengono niente che il ciclo interno marcherebbe, perché quel ciclo richiede `askable` per conto suo. ✅ **MISURATO il 2026-08-20 e non dedotto:** con quella sola riga tolta — l'altro `filter`, quello della passata di sola lettura, intatto — l'**intero workspace** resta verde, **34 target, 236 passate, 0 fallite, 2 ignorate**, identico alla baseline misurata lo stesso giorno con lo stesso comando. ⛔ **Il filtro RESTA:** è ciò che rende *«the set IS the same set»* vero **per costruzione** e non per promessa (`E76`), e toglierlo rimetterebbe la distanza che quella frase dovrebbe altrimenti promettere via. ⚠️ **Il prezzo, dichiarato: quella frase sta al presente e non la tiene nessuna sonda**, quindi il giorno in cui qualcuno cancella la riga per peso morto diventa falsa **in silenzio** — è ciò che si compra comprando una proprietà per costruzione invece che con una sonda. ⛔ **E la ragione per cui si scrive è l'INCOERENZA prima ancora del mutante:** `E70` dichiara il proprio mutante vivo accanto alla frase, con la misura; questo no. Due frasi nello stesso file trattate in due modi sono la specie di difetto che questo repository chiama incoerenza. Riga **15** della campagna |
| **E84** | Task 7, in terza revisione | ⚠️ **`E76` DICEVA *«misurato nelle DUE direzioni»* E CIÒ CHE ERA MISURATO ERA UNO SCENARIO, PRIMA E DOPO — e il termine di casa non si usa per un'altra cosa.** *«Prima e dopo il filtro»* prova che il filtro **fa** qualcosa; *«nelle due direzioni»*, qui, vuol dire che un controllo **scatta dove deve e non scatta dove non deve**, ed è la seconda metà che si dimentica (regola di `CLAUDE.md`, e la ragione per cui esistono le righe **2c**, **3b** e **12c** di questa campagna). ✅ **La seconda direzione è stata MISURATA invece che la frase ristretta**, con la stessa strumentazione — `lanes.len()` restituito da lì, su due sonde usa-e-getta cancellate subito dopo: ① una corsia candidata più una esclusa — residente `Realtime`, `below = Interactive`, un candidato `Batch` — → **`Mib(2)` prima del filtro, `Mib(1)` dopo**; ② **due** corsie entrambe candidate — `Interactive 4_096` e `Batch 2_048`, `below = Realtime` — → **`Mib(2)` prima e `Mib(2)` dopo**, cioè il filtro non toglie **mai** una corsia che ha un candidato. `E76` è riscritta e non affiancata |
| **E85** | Task 7, in terza revisione | ⛔ **IL RICONTEGGIO CHE `E77` AVEVA MESSO AL POSTO DI UNA CIFRA STANTIA VIVEVA IN QUATTRO DOCUMENTI, SOTTO UN TITOLO CHE DICE *«AND IT IS NOT REPLACED BY A BIGGER NUMBER»*.** La cifra tolta era *«`release` has TWO callers»*; quella scritta al suo posto — i siti chiamanti ricontati — stava a un tempo nel doc di `ReleaseError`, in `E30`, in `E77` e nel registro. ⛔ **`CLAUDE.md` non fa eccezione per la cifra giusta: un puntatore o una cifra che vive in PIÙ documenti si TOGLIE, non si ricorregge**, e i documenti secondari **rimandano** (gotcha **#68**). Il riconteggio vive ora in **`E77`** e basta; il sorgente, `E30` e il registro ci rimandano. ⚠️ **Il rapporto del Task 7 lo porta ancora, ed è voluto:** è il verbale **datato** dell'ondata che lo misurò, non un documento che qualcuno consulta per sapere quanti siano i chiamanti — e riscrivere il verbale di un'ondata chiusa sarebbe riscrivere ciò che accadde |
| **E86** | Task 7, in terza revisione | ⚠️ **TRE MINORI, e il primo è una rivendicazione che il proprio oggetto smentisce.** ① `E80①` diceva *«✅ verificato che la data ci sia»* delle righe `D0`/`D1`/`D2`, che portano invece *«Misura del Task 7, non rimisurata»* — un riferimento al **compito**, non una data. ⛔ **Corretta la RIVENDICAZIONE e non le celle:** il contenuto è sano e anzi **migliore** di una data, perché le due ondate del 2026-08-20 ne condividono una e il Task 7 no — un riferimento al compito le distingue dove una data non lo farebbe. ② La cella della sonda del salto di corsia diceva *«nessuna corsia deve coprire il bisogno da sola»*, che al presente si legge come una proprietà dello **scenario** e non della regola: `Interactive 4_096` lo copre eccome, a non bastare è la corsia **peggiore**. ⛔ **Ristretta la frase invece di allargare lo scenario:** la lacuna è **nominale e non di copertura** — il mutante che deve morire muore, riga **14** rossa e sola — e la corsia migliore che basta da sola è anche la ragione per cui la risposta **eccede**. ③ In coda a questa tabella c'era una riga vuota **di troppo**, prima di quella che già la chiudeva: tolta senza toccare le voci, quindi `E33` non si ripete |
| **E87** | Task 8 | ⛔ **`enqueue` NON ESISTE, E IL CODICE DETTATO DAL PASSO 3 NON COMPILA.** Il ramo finisce con `return self.enqueue(profile, valid_for);` e in `crates/kernel/src/arbiter/mod.rs` non c'è nessun `enqueue`: l'accodamento è scritto **in linea** dentro `admit` dal Task 6 — il `TicketId`, l'incremento di `next_ticket`, l'`entry(profile.compute_class).or_default().push(Waiting { … })`, il `return Admission::Queued(ticket)`. ✅ **Estratto**, che è ciò che lo snippet assume, e dà all'accodamento una casa sola adesso che il ramo ha una seconda cosa davanti. ⛔ **L'estrazione non doveva cambiare NIENTE, e non è stato dato per scontato:** stessi campi, stesso ordine, stesso contatore, e il doc dell'`admit` che descriveva il ramo è **spostato** sulla funzione invece di essere riscritto. ✅ **Provato eseguendo:** `crates/kernel/tests/arbiter_admission.rs` resta a **venti** passate e il `#[cfg(test)] mod tests` del `lib` a **tredici**, gli stessi identici numeri misurati prima di toccare qualunque cosa. ✅ **E provato che il corpo estratto sia VIVO e non solo verde** — due mutazioni, righe **11a** e **11b** della campagna: senza l'incremento del contatore `arbiter_admission` va **18/2**, con ogni richiesta messa nella corsia `Batch` va **19/1**. 📌 La specie è la quinta domanda del pre-controllo: *«il contratto cresce sotto il piano»*, e qui il piano dava per esistente una funzione che un compito precedente non aveva scritto |
| **E88** | Task 8 | ⛔ **`each_policy_names_itself` COME DETTATA NON COMPILA: manca `MakeRoom` fra le importazioni.** `name()` vive **solo** sul tratto — ✅ verificato leggendo il sorgente e non ricordato: le quattro occorrenze di `fn name(` in `crates/kernel/src/` sono la dichiarazione nel tratto e i suoi **tre** impl, e nessun impl inerente esiste — quindi chiamarlo senza il tratto in scope è un errore. ✅ **Misurato togliendo `MakeRoom` dalla lista `use` a compito finito, e non predetto:** `` error[E0599]: no method named `name` found for enum `VramPolicy` in the current scope `` con `` help: items from traits can only be used if the trait is in scope ``, **quattro volte** — due per l'enum e due per il `&VramPolicy` che `policy()` restituisce. Mutazione revocata da copia byte-esatta, `cmp` identico. 📌 È la specie di `E4`, `E20` e `E46`: una lista `use` dettata dal piano che il codice del giorno non soddisfa |
| **E89** | Task 8 | ⛔ **IL CASO NEGATIVO DI `V3` DETTATO DAL PASSO 4 PROVA LA COSA SBAGLIATA, E LE DUE FORME SONO STATE MISURATE PRIMA DI SCEGLIERE.** Il piano scrive `VramPolicy::Remote(..) \| VramPolicy::Local(..)` e prova che `VramPolicy` non implementa **`BitOr`** — ✅ oracolo generato e letto: `` error[E0369]: no implementation for `VramPolicy \| VramPolicy` `` con `` note: `VramPolicy` does not implement `BitOr` ``. ⛔ **Ma la regola è *«il valore consegnato ne porta UNA sola»*, e la mutazione che la regola teme è un `Arbiter::new` che ACCETTI due policy.** ✅ **Misurate entrambe le forme nello stesso giro, con quella mutazione installata** (`pub const fn new(parameters, policy, _second)`), che è la domanda della **specie** del gotcha **#42**: la forma **`BitOr` resta `ok`** — non nomina `Arbiter::new`, quindi la regressione le è invisibile e il caso non scatta **mai** per la ragione per cui esiste; la forma **ad arità passa a `error`**, cioè comincia a compilare e `trybuild` lo dice **fuori dall'oracolo**. ✅ **Scelta la forma ad arità, `crates/kernel/tests/compile_fail/two_policies_at_once.rs`**, che è la stessa con cui `E23` chiuse `V2` (`admission_without_profile.rs`), e ⛔ **la forma dettata NON è stata tenuta accanto come seconda guardia**: un caso che non può scattare per la propria ragione è una guardia solo all'apparenza, e il compito chiede *«un caso buono invece di due deboli»*. ⚠️ **E il LIMITE è dichiarato e misurato, non dedotto:** il caso pinza l'**arità di `new`**, non l'assenza di ogni strada verso due policy — con un secondo costruttore `pub const fn new_with_two(parameters, a, _b)` il caso resta **`ok`** e l'intera suite resta verde. Stessa specie del limite che `admission_without_profile.rs` dichiara di sé. ⚠️ **Sotto la mutazione i tre casi preesistenti che chiamano `Arbiter::new` vanno `mismatch`**, ed è atteso: i loro oracoli citano la firma |
| **E90** | Task 8 | ⛔ **RICHIAMO DEL 2026-08-20 — QUESTA VOCE APRIVA CON *«SONO CINQUE IN QUATTRO FILE»*, E I FILE SONO CINQUE: la cifra è TOLTA da qui invece di essere riallineata.** Viveva in **quattro** case con due valori, contate col `grep` (`E109` le enumera), e `CLAUDE.md` dice di toglierla e non di ricorreggerla; la sua casa unica è **`E101`**, che l'ha misurata sul commit di partenza col `git grep` e che nomina proprio questa voce come portatrice dell'errore. ⚠️ **Riscritta e non affiancata, come `E91` due righe sotto lo stesso giorno:** due voci della stessa specie nella stessa tabella non possono avere due trattamenti (`E109`). ⚠️ **Ciò che questa voce tiene resta vero: il piano dice *«i banchi dei Task 5–7 più nessun altro»*, ed è falso in ENTRAMBI i termini** — è la specie di `E18`. Ricontati col comando prima di cominciare: `crates/kernel/src/arbiter/mod.rs` (l'aiutante del `mod tests`), `crates/kernel/tests/arbiter_admission.rs` (l'aiutante del banco) e **tre casi `compile_fail`** — `admission_has_no_is_granted.rs`, `admission_reads_cold_start.rs`, `admission_without_profile.rs`. ⛔ **E i tre casi hanno l'oracolo che pinza un NUMERO DI RIGA** (26, 27 e 41): lasciarli a un argomento avrebbe dato un `E0061` di arità che maschera l'errore che il caso esiste per provare (gotcha **#47**), e allungarli avrebbe reso l'oracolo `mismatch`. ✅ **Modificati a PARITÀ DI RIGHE** — la chiamata su quattro righe resta su quattro, con `Parameters::new(…)` su una riga sola e la policy sulla successiva: ✅ **ZERO oracoli rigenerati e ZERO righe di oracolo cambiate**, e le tre righe pinzate sono ancora quelle. ✅ **Verificato con `git status --porcelain` e non con `git diff`** — che i non tracciati non li vede, `E3` ed `E11` — l'uscita porta i tre `.rs` come ` M` e **nessun** `.stderr` modificato; l'unico `.stderr` nuovo è quello del caso di `V3`, **venti righe**, generato una volta e letto prima di installarlo. ✅ **Casi ricontati col comando:** `ls crates/kernel/tests/compile_fail/*.rs \| wc -l` → **29**. ✅ **E le due misure del coordinatore sono state riverificate e reggono:** nessun `.stderr` cita `Arbiter::new`, e i due che puntano dentro `src/arbiter/mod.rs` lo fanno **senza numero di riga**, perché `trybuild` normalizza i percorsi della crate — toccare `mod.rs` non sposta nessun oracolo |
| **E91** | Task 8 | ✅ **LA SCADENZA DI `E67`/`E74` È SCATTATA E SI È AVVERATA — misurata, non dedotta.** Quelle voci lasciavano due avvisi `dead_code` in piedi — `` fields `lane` and `grace` are never read `` e `` method `ask_back` is never used `` — accettati dal proprietario **con la scadenza a questo compito** e con la regola scritta anche accanto al codice: *«se al Task 8 sono ancora lì, il metodo non serviva e si toglie»*. ✅ **`cargo build --locked --workspace` → ZERO avvisi**, contro i **due** misurati sullo stesso comando prima di cominciare. La causa è esattamente quella prevista: `admit` sotto `LocalPolicy` è il chiamante di produzione di `ask_back`, e `ask_back` legge `lane` e `grace` dentro `askable`. ⛔ **Nessun `#[allow]`, nessun lettore inventato, nessun `pub` di comodo** — non ce n'è stato bisogno. ⛔ **RICHIAMO DEL 2026-08-20 — QUESTA VOCE DICEVA IL FALSO nella sua ultima frase, e la frase è RISCRITTA e non affiancata.** Diceva *«la scadenza è tolta anche dal doc di `ask_back`, e al suo posto c'è il fatto»*: il commit `4b89fea` **non contiene nessun hunk** che tocchi quella documentazione, e il testo era rimasto nell'albero **al presente**, in quattro punti — il chiamante dato per inesistente, il consumatore dato per un banco, i due avvisi dati per presenti, e l'ordine di **togliere** il metodo. ✅ **Tolto davvero nell'ondata di correzioni dello stesso giorno**, e il seguito è nella voce `E99` |
| **E92** | Task 8 | ⛔ **DUE DIVERGENZE NELLA TABELLA DELLE MUTAZIONI DEL PASSO 5, misurate e non appianate — e la seconda tocca proprio la regola che il passo cita.** ① **La mutazione 2 ne uccide DUE e non una.** Il passo dà `LocalPolicy::may_make_room` → `false` per `the_local_policy_asks_the_lower_lanes_back`: muore anche `under_the_local_policy_the_queued_request_is_served_past_the_grace`, **3 passate 2 fallite**. Cioè la regola *«quando una mutazione ne uccide due si cerca la terza che ne lasci passare una»* vale per **due** coppie e non per una — le due sonde REMOTE sotto la **1**, le due LOCAL sotto la **2**. ② ⛔ **La terza mutazione che il passo indica NON SEPARA NIENTE.** Il piano scrive che *«`promote` che ignora le corsie lascia verde `the_remote_policy_does_not_make_room_it_queues` e rossa `under_the_remote_policy_the_same_clock_advance_serves_nobody`»*: ✅ misurato con `queues.values_mut()` percorsa `.rev()`, **NESSUNA delle cinque muore — 5 passate, 0 fallite**. La ragione si legge nello scenario: c'è **una sola** richiesta in attesa, quindi un ordine fra corsie non decide niente. La riga uccide una sonda del **Task 6** in `arbiter_admission.rs` (19/1), che è dove appartiene. ✅ **Due separatori veri trovati al loro posto:** il controllo di capienza tolto da `promote` isola `under_the_remote_policy_…` (**4/1**, sull'asserzione `promoted.is_empty()`), e un `promote` che non promuove niente isola `under_the_local_policy_…` (**4/1**). ⛔ **Le sonde NON sono state allargate per far tornare l'attesa del piano** — allargarle cambierebbe il soggetto — e la **conclusione** del piano regge: le coppie stanno su assi diversi. A provarlo sono altre mutazioni. 📌 Stessa specie di `E44`, entrambi i punti |
| **E93** | Task 8 | ⚠️ **LE ASSERZIONI FINALI SU `allocated()` SONO DOMINATE DENTRO LA PROPRIA SONDA — DICHIARATO, e le asserzioni RESTANO.** Nelle sonde a scenario che la portano, l'`assert_eq!(arbiter.allocated(), …)` finale non decide **mai** un esito: sotto ogni riga della campagna un'asserzione **sopra** di essa scatta per prima. ⛔ **Non sono però vacue, e la differenza è misurata:** ✅ **isolate** — le asserzioni precedenti tolte — scattano sotto la mutazione che leva ad `admit` il controllo del tetto, `` left: Mib(8192), right: Mib(4096) ``. ✅ **Misurato per isolamento il 2026-08-20 e non ragionato**, ⚠️ **col campione NOMINATO**, perché un'esclusività misurata su un campione parziale si legge come una garanzia (`E82`): cinque mutazioni eseguite isolate — le due risposte delle policy, la spazzata che riscuote un `Running`, `promote` senza controllo di capienza, `admit` senza tetto. Sotto la spazzata le due sonde che muoiono lo fanno sul `panic!("queued")` del `let … else` e **non** sull'asserzione isolata. ⛔ **Il rimedio è quello di `E37` e `E79`: tenerle**, perché dichiarano l'intento che il **nome** della sonda porta — i libri non si sono mossi — con la ragione scritta **una volta sola** accanto alla prima e le altre che ci rimandano invece di ripeterla (gotcha **#68**). ⛔ **RICHIAMO DEL 2026-08-20, SECONDA ONDATA — QUESTA VOCE CONTAVA *«LE TRE»* E LE ASSERZIONI SU `allocated()` NEL BANCO SONO SEI.** ✅ Ricontate col comando e non a memoria: `grep -c "arbiter.allocated()" crates/kernel/tests/arbiter_policy.rs` → **6**, e **4** sul banco di `4b89fea` — le originali erano già **quattro** prima dell'ondata e le nuove sono **due**, quindi *«le tre più la quarta, quella della sonda nuova»* ne lasciava fuori due. ✅ **E la distinzione che conta non è il numero ma QUALE, misurata riga per riga:** le **cinque** asserzioni **finali** sono dominate — in ogni riga che uccide la loro sonda scatta prima un `revoking()`, un `matches!(outcome, Queued(_))`, un `promoted.len()` o il `panic!("queued")` del `let … else` — mentre la **sesta**, la **precondizione** `assert_eq!(allocated(), Mib::new(3_072), "PARTLY full")` di `a_partly_full_machine_…`, **NON è dominata**: è la sola asserzione su `allocated()` dell'intero banco che decide un esito, sotto la riga **12** (`left: Mib(1024)`). Voce `E108` |
| **E94** | Task 8 | ⚖️ **REGISTRATA, NON PRESA — LA POLICY È UN SECONDO VALORE CONSEGNATO, E LA §2.8.2 NE PARLA AL SINGOLARE.** La regola 1 di §2.8.2 dice che il kernel *«riceve alla costruzione **un valore** che porta i parametri risolti»*; la conseguenza gratuita che fonda `V3`, nella stessa sezione, è scritta *«se **il valore consegnato** porta **una** policy, "due policy attive" non è rappresentabile»*; e ADR-0034 elenca *«quale policy VRAM è attiva»* fra i parametri di **quel** valore, accanto al budget della GPU. Il piano consegna invece la policy come **secondo argomento** di `Arbiter::new`, accanto a `Parameters`. ⚖️ **Non è un indebolimento e il compito NON ha cambiato il disegno:** l'arbitro riceve comunque la policy **alla costruzione**, non legge nessuna configurazione, e `VramPolicy` essendo un enum ne porta una sola comunque — la forza di `V3` è identica. Ciò che diverge è il **testo**: la §2.8.2 andrebbe letta *«i valori consegnati»*. ⚖️ **La scelta è del proprietario, coi due costi.** ① **Lasciarlo com'è** costa una frase di spec che non descrive più esattamente il codice, e un secondo canale di consegna che il prossimo parametro potrebbe imitare. ② **Spostare la policy dentro `Parameters`** è il testo alla lettera, e costa un campo su un tipo che la **§7.4.1 blocco C** pinza con due righe proprie, la firma di `Parameters::new` e i suoi **diciannove** chiamanti contati da `E18`, più l'oracolo di `parameters_have_no_default.stderr` che quella firma la cita verbatim. ⛔ **La §2.8 è spec e il vincolo globale 7 vale**, quindi qui si registra. La divergenza è scritta anche **accanto a `Arbiter::new`** nel sorgente, dove la legge chi la tocca. 📌 È la SESTA domanda del pre-controllo — un ADR letto contro i propri fratelli — con ADR-0034 nel ruolo del fratello che ADR-0006 non nomina |
| **E95** | Task 8 | ⚠️ **DUE CIFRE STANTIE, RICONTATE ESEGUENDO.** ① La baseline in testa a questo piano — *«32 target · 194 passati»* — è di **otto** compiti fa, ed è la stessa che `E8`, `E26` e `E68` hanno già dovuto rimisurare: `cargo test --workspace --no-fail-fast --locked` prima di toccare qualunque cosa dà **34 target · 236 passate · 0 fallite · 2 ignorate**, e a fine compito **35 · 241 · 0 · 2** — un bersaglio nuovo, `tests/arbiter_policy.rs`, e cinque sonde. ⛔ **Nessun numero dedotto**: i conteggi per file sono stati letti sul binario — `arbiter_admission.rs` **20/0** prima e dopo, `mod tests` del `lib` **13/0** prima e dopo, `arbiter_policy.rs` **5/0**. ② Il Passo 5 attende *«PASS, cinque test»* dal file nuovo, e questa volta la cifra è **giusta**: il file nasce in questo compito e nessun compito precedente ci ha messo niente. 📌 Registrata anche quando la cifra torna, perché il vincolo globale 14 non ha eccezioni per le divergenze che **non** cambiano l'esito — il precedente è `E7` |
| **E96** | Task 8 | ⛔ **LA DOMANDA DEL GOTCHA #66, MISURATA: IL RAMO NUOVO NON È TENUTO DA NIENTE FUORI DAL PROPRIO BANCO, E NIENTE È DIVENTATO VACUO.** Il ramo che accoda è attraversato da parecchie sonde dei Task 6 e 7 — ogni esito `Queued` ci passa — quindi la domanda è doppia: ① qualcuna di quelle sonde ha smesso di provare ciò che provava? ② qualcuna tiene il ramo nuovo? ✅ **Alla ① la risposta è no, e la ragione è strutturale prima che misurata:** i due aiutanti condivisi consegnano `VramPolicy::Remote(RemotePolicy)` — il **default** di ADR-0006, quello che non fa spazio — quindi sotto quelle sonde `admit` fa esattamente ciò che faceva, riga per riga. Con `Local` avrebbe cominciato a **marcare**, cambiando in silenzio il soggetto di una dozzina di sonde che parlano d'altro. ✅ **Alla ② la risposta è no, e questa è misurata:** con la domanda tolta da `admit` in **entrambe** le direzioni — nessuno fa spazio, oppure tutti lo fanno — `arbiter_admission.rs` resta **20 passate, 0 fallite** e il `mod tests` del `lib` **13, 0**. Il ramo è tenuto **solo** da `tests/arbiter_policy.rs`. ⛔ **Non è un difetto ed è il motivo per cui quel banco esiste**, ma va scritto: chi cancellasse il banco delle policy non troverebbe nessun altro rosso. ✅ **E l'estrazione di `enqueue` sta dall'altra parte:** quella **è** tenuta dalle sonde preesistenti, righe **11a** e **11b**. Dichiarato nel commento di modulo di `crates/kernel/tests/arbiter_admission.rs`, che dal Task 6 impone a ogni compito di estendere l'enumerazione dei propri soggetti (`E56`, `E66`) — questo compito non vi aggiunge sonde ma **cambia ciò che il file prova**, ed è la prima volta che quell'istruzione va onorata in questa forma |
| **E97** | Task 8, in revisione | ⛔ **I DUE ARGOMENTI CHE `admit` CALCOLA PER `ask_back` NON ERANO TENUTI DA NIENTE, E LO SCENARIO DELLE CINQUE SONDE È DEGENERE.** Nel banco del compito il totale, il residente e il richiedente valgono **tutti 4_096**, quindi `ceiling == allocated() == asked == needed`: le quattro grandezze **coincidono** e nessuna asserzione può distinguerle. La campagna non aveva né una mutazione su `needed` né una sulla corsia. ✅ **DUE MUTANTI MISURATI VIVI il 2026-08-20, prima di scrivere una riga di rimedio** (gotcha **#45**), ciascuno compilato in un passo separato da quello che esegue e revocato da copia byte-esatta: ① `let needed = asked;` — `ask_back` usa il primo argomento come **soglia** (`if reclaimable < needed { return Mib::ZERO }`), quindi `LocalPolicy` degraderebbe a `Remote` **in silenzio** ogni volta che la macchina è solo PARZIALMENTE piena: **workspace 241 passate, 0 fallite**. ② `ask_back(needed, ComputeClass::Realtime, now)` — `askable` scarta con `held.lane <= below`, quindi una corsia **migliore** ALLARGA le vittime e l'arbitro sfratterebbe un pari `Interactive` per un `Interactive`: **workspace 241 passate, 0 fallite**. ⚠️ **`a_grant_in_the_asking_lane_itself_is_not_asked_back` non vede il secondo**: chiama `ask_back` **direttamente** con corsie esplicite, quindi tiene il **confine** e non il **cablaggio** — il meccanismo era tenuto bene dal Task 7, a mancare era la parte nuova di questo compito. ✅ **DUE SONDE NUOVE, e ciascun mutante muore con un uccisore SOLO NELL'INTERO WORKSPACE (242 passate, 1 fallita):** `a_partly_full_machine_asks_back_the_need_and_not_the_whole_request` — macchina 4_096, un `Realtime` da 2_048 e un `Batch` da 1_024, richiedente `Interactive` da 2_048, quindi le quattro grandezze diventano `4_096`, `3_072`, `2_048` e `1_024` — e `the_admission_asks_back_below_its_own_lane_and_spares_a_peer` — un `Realtime` da 1_024 e un **pari** `Interactive` da 2_048, che nessuno deve sfrattare. Righe **14** e **15** della campagna |
| **E98** | Task 8, in revisione | ⛔ **DIVERGENZA DAL RIMEDIO CHIESTO DALLA REVISIONE, e NON dal difetto: il rilievo pretendeva *«una sesta sonda… uccide entrambi i mutanti con uno scenario solo»*, e UNO SCENARIO SOLO NON PUÒ FARLO.** Il difetto era giusto — entrambi i mutanti erano vivi, misurato — ma la **forma** del rimedio no, e la ragione sta nel codice e non in un'opinione. ⛔ Uccidere `needed → asked` richiede `needed <= reclaimable < asked`, cioè che l'insieme sano dei candidati **copra** il bisogno. ⛔ Uccidere la corsia fissa richiede che l'esito cambi allargando quell'insieme; ma la passata che marca percorre le corsie **peggiore per prima** ed esce **appena `covered >= needed`**, e allargare `below` a una corsia migliore aggiunge solo candidati in corsie **migliori**, che quella passata raggiunge per ultimi. L'esito cambia quindi **solo se** l'insieme sano NON copre il bisogno — esattamente il contrario della prima condizione. ✅ **Misurato e non solo argomentato:** sotto il mutante `needed → asked` la sonda della corsia resta **verde**, e sotto il mutante della corsia resta verde quella del bisogno — **6 passate e 1 fallita** in entrambi i casi, con l'unica rossa che è ogni volta l'altra. ✅ **Scritte quindi DUE sonde e non una**, che è anche la forma che questo repository usa altrove — una sonda dice una cosa sola — e che dà a ciascuno dei due cablaggi la propria sonda: la **14** è *«scatta dove deve»* per lo **scarto**, la **15** *«non scatta dove non deve»* per la **corsia**. ⚠️ **RICHIAMO DEL 2026-08-20, SECONDA ONDATA — QUESTA CHIUSA DICEVA *«le due direzioni della regola nuova»*, al singolare, e le regole sono DUE.** Le due sonde non sono le due direzioni di un solo controllo: tengono due **argomenti** diversi, ciascuno col proprio verso opposto **altrove** — per lo scarto in `asking_back_marks_nothing_when_the_reclaimable_does_not_cover_the_need`, per la corsia in `the_local_policy_asks_the_lower_lanes_back` (sul cablaggio) e in `only_lanes_below_the_asking_one_are_asked_back` (dentro il meccanismo). Il **vincolo 6 è soddisfatto**: a comprimere due regole in una era la frase, non il banco. Voce `E110` |
| **E99** | Task 8, in revisione | ⛔ **LA SCADENZA DI `E67`/`E74` ERA RIMASTA SCRITTA ACCANTO AL CODICE, QUESTO COMMIT L'AVEVA RESA FALSA, E DUE DOCUMENTI LA DAVANO PER TOLTA.** Nell'albero il testo era ancora lì **al presente**, in quattro punti del doc di `ask_back`: *«THAT CALLER IS TASK 8 AND DOES NOT EXIST YET»*; *«the consumer is a BENCH»*; *«`cargo build` today prints TWO `dead_code` warnings … AT TASK 8 THOSE TWO WARNINGS MUST BE GONE … IF THEY ARE STILL THERE, THIS METHOD WAS NOT NEEDED AND IT IS REMOVED»*; e la **motivazione** della scelta chiusura-invece-di-metodo, la cui premessa (*«`ask_back` non ha chiamanti di produzione, quindi è `dead_code`, e ciò di cui è l'unico chiamante è morto con lui»*) era **scaduta lo stesso giorno**. ⛔ **Chi arriva dopo vi legge, al presente, l'ordine di TOGLIERE il metodo appena diventato la spina dorsale di `LocalPolicy`** — *«un documento che mente con autorevolezza»*, non una rifinitura. ⛔ **E NON ERA UNA DIMENTICANZA: il rapporto del compito lo dava per fatto** (*«la scadenza è stata **tolta** dal doc di `ask_back` e sostituita col fatto»*) **e la stessa frase chiudeva la voce `E91`** — due affermazioni false, **registrate** invece che corrette in silenzio: un rapporto è un'affermazione come le altre. ✅ **Riscritte e non affiancate** (2026-08-20): il doc di `ask_back` porta ora il **fatto** — il chiamante è `Arbiter::admit`, nel ramo che non fa entrare la richiesta, subito dopo che `may_make_room` ha risposto sì, e i due avvisi sono caduti al Task 8 — col rimando a `E91`. ✅ **E la premessa scaduta è stata RIMISURATA invece che dedotta:** con un aiutante privato di `impl Held` raggiungibile **solo** da dentro `ask_back`, `cargo build --locked --workspace` stampa **ZERO** avvisi, perché `admit` arriva ad `ask_back` e niente dietro di lui è più morto. Il commento dichiara quindi che la scelta della chiusura **oggi non è tenuta da nessuna misura**, invece di citarne una che non vale più. ✅ **Cercate tutte le case col `grep` e guardate in faccia una per una:** `docs/porta-di-qualita.md` ne aveva **due** — la sezione del Task 7 e quella del Task 8 — corrette entrambe con un richiamo datato; `docs/HANDOFF.md` **nessuna**. ⚠️ **`docs/COMPENDIO.md` è stato guardato e NON toccato, deliberatamente:** la sua §6 è ferma al Task 7 — dice ancora *«il prossimo passo è il Task 8»* perché il commit `docs(consegna)` non c'è ancora — e riallineare due frasi dentro una sezione ferma a un compito prima la lascerebbe **più** contraddittoria, non meno. Quella sezione si muove **come un blocco**, alla consegna (gotcha **#68**) |
| **E100** | Task 8, in revisione | ⛔ **L'INVERSIONE DI PRIORITÀ DI `E41`/`E51`/`E53` PASSA DA TEORICA A RAGGIUNGIBILE IN PRODUZIONE CON QUESTO COMMIT, e viveva SOLO nel rapporto — che è effimero.** È il gotcha **#36**: uno scarto fra codice e registro si scrive come **voce aperta**, mai come nota. `admit` riscuote e concede subito se c'è posto, **senza guardare le code**; fino al Task 7 `ask_back` non aveva chiamanti di produzione, quindi **nessuna revoca avveniva mai** fuori da una sonda e la stanza liberata da una revoca **non esisteva**. Da questo commit sì. ✅ **MISURATO NELLE DUE DIREZIONI il 2026-08-20**, su una sonda usa-e-getta cancellata subito dopo. **LOCAL:** residente `Batch` da 4_096 chiesto indietro per un biglietto `Interactive` in coda, poi a `501` una richiesta `Batch` **nuova** da 4_096 → **`Granted`**, il `promote` successivo torna **vuoto**, `queued()` è ancora **1** — il ritardatario si è seduto sulla stanza chiesta per un altro. **REMOTE**, che è esattamente il mondo prima di questo compito visto che nessuno veniva mai chiesto indietro: lo stesso ritardatario è **`Queued`** e `queued()` è **2** — non si era liberato niente, e non c'era niente da prendere. ⛔ **NON PINZATO, E DICHIARATO: nessuna sonda lo tiene** — la forma di `E50`/`E51`/`E53`, perché una sonda che va cancellata per prendere una decisione è un voto contro il prenderla. ⚖️ **Il chiusore resta il TASK 10** ed è ancora una decisione di **orchestrazione**: ciò che il Task 8 cambia è solo che il costo di lasciarla aperta si paga ora **in produzione** e non sulla carta. Scritto accanto ad `admit` **e** nella sezione del Task 8 di `docs/porta-di-qualita.md` |
| **E101** | Task 8, in revisione | ⚠️ **«CINQUE SITI IN QUATTRO FILE»: I FILE SONO CINQUE, E L'ERRORE VENIVA DAL BRIEF DEL COMPITO.** La voce `E90` e la sezione del Task 8 citavano il proprio comando come se la cifra fosse **misurata**, e non lo era: veniva dal brief, dove era già sbagliata. ✅ **Ricontati sul commit di partenza e non dedotti** — `git grep` di `Arbiter::new` su `dd99e50`, tolte le righe di commento: `crates/kernel/src/arbiter/mod.rs`, `crates/kernel/tests/arbiter_admission.rs` e i **tre** casi `compile_fail`, **uno per file** — cinque siti in **CINQUE** file. ⛔ **È il gotcha #31 preso dal verso in cui il numero non invecchia ma nasce già falso:** una cifra ereditata e ripresentata come propria non diventa vera per essere stata copiata con cura |
| **E102** | Task 8, in revisione | ⛔ **IL CONTEGGIO DELLA CAMPAGNA DIVERGEVA FRA DUE DOCUMENTI E NON TORNAVA IN NESSUNO DEI DUE, quindi È TOLTO dalla prosa e non riallineato.** `docs/porta-di-qualita.md` diceva *«TREDICI righe»*; il rapporto del compito diceva *«sedici»*. ⛔ `CLAUDE.md` è esplicito — *«un puntatore o una cifra che vive in PIÙ documenti si TOGLIE, non si ricorregge»* — e la ragione si vede qui: riallinearla la rimetterebbe nello stato in cui la sola difesa è la diligenza di chi aggiunge una riga, e quella diligenza non ha retto. ✅ **La tabella sotto È la misura**, e un rimando non può marcire (gotcha **#68**). ⚠️ **Tolta anche la seconda copia della stessa cifra**, nella riga di `E93` che diceva *«in tutte e tredici le righe»* |
| **E103** | Task 8, in revisione | ⚠️ **`V3` È CONTATA PARZIALE E NON COPERTA, e il numeratore del blocco C torna a QUATTORDICI SU DICIANNOVE.** Il compito l'aveva fatta entrare fra le coperte mentre **metà** della sua contro-sonda di catalogo — *«e la transizione resta un passo giornalato (§5.4)»* — è del **Task 9**. ⛔ **La ragione è la coerenza col precedente dello STESSO FILE, che aveva già applicato la regola opposta due volte:** `Q8 · §5.2.1` al Task 3 (*«non entra nel numeratore delle coperte: una riga parziale non è una riga chiusa»*) e la riga del blocco B. ✅ **Ricontato e non dedotto**, sull'enumerazione della cella che le elenca: coperte **quattordici**, parziale **una** (`V3`), scoperte **quattro** (`V5`, `V10` e le due righe di §6.10) — 14 + 1 + 4 = 19, e il conto torna. ✅ **Corretto in TUTTE le sue case, cercate col `grep`:** il richiamo in testa a `docs/porta-di-qualita.md`, la riga di catalogo, il titolo e l'apertura della sezione del Task 8, la riga della contro-sonda e la cella dei riconteggi. Residuo di *«quindici»* nel repository: **zero**, verificato col `grep`. ⏳ **INNESCO SCRITTO, perché chi lo chiude sappia di doverlo muovere: la seconda metà di `V3` è del TASK 9**, ed è quel compito a portare il numeratore a quindici |
| **E104** | Task 8, in revisione | ⛔ **LE DUE SONDE NUOVE CAMBIANO IL DENOMINATORE DELLA CAMPAGNA, quindi TUTTE LE RIGHE SONO STATE RIMISURATE SUL BANCO DI SETTE invece di essere riportate.** È il gotcha **#31** nella sua forma più economica da commettere: le celle dicevano *«banco 3/2»* su un banco che ora è di **sette**, e riportarle avrebbe voluto dire scrivere una cifra vera per un file che non esiste più. ⚠️ **`arbiter_admission` e `lib` NON sono stati rimisurati, ed è una scelta dichiarata e non una scorciatoia:** aggiungere sonde a `tests/arbiter_policy.rs` non può cambiare ciò che un altro bersaglio esegue. ⛔ **DUE ESITI SONO CAMBIATI DAVVERO, E SONO UN COSTO DELLE SONDE NUOVE:** la riga **7** (`revoking()` sempre 0) uccideva **una sola** sonda e ora ne uccide **due**; la riga **8** (`revoking()` conta ogni concessione) ne uccideva **una** e ora ne uccide **tre**. Le sonde nuove leggono `revoking()` anche loro, quindi `the_local_policy_asks_the_lower_lanes_back` e `the_remote_policy_does_not_make_room_it_queues` **hanno perso il proprio uccisore solo**: si passa da **cinque su cinque** a **cinque su sette**. Nessuna sonda è diventata più debole — è la CAMPAGNA a separare di meno. ⚠️ **E UNA SONDA È ORA DOMINATA DENTRO LA CAMPAGNA, dichiarato invece che lasciato scoprire** (forma di `E37`, `E79`, `E93`): `the_local_policy_asks_the_lower_lanes_back` muore sotto **2, 3a, 7, 12, 13**, e `a_partly_full_machine_…` muore sotto **quelle stesse più 8 e 14** — ⚠️ **richiamo del 2026-08-20, seconda ondata: diceva *«più 8, 13 e 14»* e la 13 era già nell'insieme di partenza**, quindi l'insieme più grande ha **sette** righe e non otto (`E110`). ⛔ **Non si cancella:** la campagna è un **campione** e non una dimostrazione, e la sonda dominata è la coppia esatta di `the_remote_policy_…` sullo **stesso scenario**, che è la forma con cui ADR-0006 chiede che le due policy si distinguano. ⚠️ **Altre righe si sono mosse senza cambiare significato:** la **2** e la **3a** ne uccidono **tre** invece di due, e la **12** e la **13** ne uccidono **sei**. ⛔ **RICHIAMO DEL 2026-08-20, SECONDA ONDATA — LA 12 NON APPARTENEVA A QUESTO ELENCO, E METTERCELA HA ARCHIVIATO UNA DIVERGENZA INVECE DI REGISTRARLA.** La sua cella dichiarava un *«prima»* di **3/2** che non poteva essere vero — questa ondata non tocca **nessuna** riga eseguibile e i test d'integrazione non condividono stato, quindi da tre passate non si scende a una aggiungendo sonde — e la cifra **nuova** (1/6) era invece giusta. ✅ **Rimisurato e non dedotto: 1/4 sul banco di cinque di `4b89fea`**, con la **13** accanto usata come controllo (stesso insieme di morte, e il suo *«prima»* di 1/4 regge). ⛔ **Il precedente di casa è `E81`, che per questa identica specie rimisurò E dichiarò:** dove la misura diverge dall'atteso si **registra la divergenza**, e questa voce l'aveva messa fra le righe *«mosse senza cambiare significato»*. Voce `E106` |
| **E105** | Task 8, in seconda revisione | ⛔ **L'AIUTANTE `never_preemptible` NATO CON LA PRIMA ONDATA NON TENEVA NIENTE, ED È UN MUTANTE VIVO GARANTITO CHE QUELL'ONDATA SI ERA PORTATA DENTRO DA SÉ.** Il suo doc dichiarava che le due sonde nuove *«ne hanno bisogno: un residente che NON può essere chiesto indietro è ciò che lascia la macchina solo parzialmente recuperabile»*, e in **nessuna** delle due il residente veniva guardato per la prelazionabilità: sta in corsia `Realtime` col richiedente in `Interactive`, quindi `askable` lo scarta sul **primo** dei tre controlli — `if held.lane <= below` — e `held.grace`, l'unica cosa che `Preemption::Never` cambia, **non viene mai letto**. ✅ **MISURATO PRIMA DI SCEGLIERE e non ragionato:** con `Preemption::After(Millis::new(500))` al posto di `Preemption::Never` dentro l'aiutante il banco resta **7 passate, 0 fallite**; nessun altro lo usava. ⛔ **Tolto, e i due residenti resi `preemptible`:** cadono insieme una duplicazione verbatim (l'aiutante era `preemptible` a meno di un campo), una frase falsa e un mutante vivo, e i due residenti restano al riparo per la sola ragione vera — la **corsia** — che è già il motivo per cui il pari di `the_admission_asks_back_below_…` è deliberatamente prelazionabile. ⚠️ **E l'attribuzione in linea era sbagliata a sua volta:** il commento diceva che a rendere lo scarto (`1_024`) minore della richiesta (`2_048`) fosse la prelazionabilità del residente, mentre a renderlo tale è che la macchina è **parzialmente libera** (`allocated` 3_072 su un tetto di 4_096). ⛔ **UNA TESI SCRITTA PRIMA DELLA MISURA È CADUTA, e si registra invece di tacerla:** l'attesa era che l'aiutante fosse **dannoso** e non solo inerte — che con `Never` una mutazione del confine da `held.lane <= below` a `held.lane < below` restasse invisibile. ✅ Misurata: quella mutazione uccide `the_admission_asks_back_below_its_own_lane_and_spares_a_peer` **anche** con l'aiutante in piedi (banco **6/1**), perché a diventare recuperabile è il **pari** e non il residente `Realtime`, ed è uccisa pure nel `lib` da `a_grant_in_the_asking_lane_itself_is_not_asked_back` (12/1). Inerte, non dannoso. ⛔ **E il banco è cambiato, quindi la CAMPAGNA È STATA RIESEGUITA TUTTA per la seconda volta in un giorno** (`E104` per il precedente): **tutti gli ingressi**, contati col comando sulla tabella, e **nessuna cella si è mossa** rispetto a quelle registrate — stessi conteggi e stessi insiemi di morte, con la **14** e la **15** ancora sole nell'intero workspace a **242 passate 1 fallita**. È la prova misurata che la rimozione non ha comportamento, e non un argomento sul perché non dovrebbe averlo. ⚠️ **`Preemption::Never` non resta in questo banco e non è una perdita:** è esercitata in `crates/kernel/tests/arbiter_admission.rs`, nel `#[cfg(test)] mod tests` del `lib`, in `crates/kernel/tests/arbiter_resource.rs` e in due casi `compile_fail` |
| **E106** | Task 8, in seconda revisione | ⛔ **LA CELLA DELLA MUTAZIONE 12 DICHIARAVA UN «PRIMA» CHE NON POTEVA ESSERE VERO, E `E104` L'AVEVA ARCHIVIATO INVECE DI REGISTRARLO.** La cella passava da *«banco 3/2»* a *«banco 1/6»*, e `E104` metteva quel salto fra le *«righe che si sono mosse senza cambiare significato»*, accanto alla **13** che va da 1/4 a 1/6. ⛔ **Ma la prima ondata non tocca NESSUNA riga eseguibile** — `git diff 4b89fea..ea0cc09 -- crates/kernel/src/arbiter/mod.rs` cambia **zero** righe che non siano commento, verificato col comando — **e i test d'integrazione non condividono stato**: aggiungere due sonde può solo aggiungere morte, quindi da **tre** passate si arriva al massimo a tre, mai a una. ✅ **Rimisurato sul banco di cinque di `4b89fea` e non dedotto: 1/4.** Muoiono `the_remote_policy_…` e `the_local_policy_…` sull'asserzione `matches!(outcome, Admission::Queued(_))` — la spazzata riscuote il residente `Running` **prima** del secondo `admit`, quindi la richiesta entra — e le due `under_the_…` sul `panic!("queued")` del `let … else`; sopravvive la sola `each_policy_names_itself`. ✅ **Controllo incrociato sulla riga 13**, che ha lo stesso insieme di morte: il suo *«prima»* di **1/4** è stato rimisurato e **regge**, il che isola la 12 come l'unica cifra sbagliata. ⛔ **E UN COROLLARIO DELLA STESSA CELLA ERA FALSO:** diceva che le morte muoiono sul `panic!` o sull'asserzione precedente *«e non su quella di `allocated()`»*, mentre sotto la 12 `a_partly_full_machine_…` muore **proprio** sull'`assert_eq!(arbiter.allocated(), Mib::new(3_072), "PARTLY full: 3_072 of 4_096")` — `left: Mib(1024)` — perché la spazzata svuota i libri già al secondo `admit`. ✅ **La riga 13 è stata rimisurata anche sul banco di sette per lo stesso corollario, e lì regge:** `a_partly_full_machine_…` muore sul `matches!(outcome, Queued(_))` e la sua asserzione di `allocated()`, che sta prima, passa. 📌 **Il precedente di casa è `E81`**, che per questa identica specie rimisurò **e** dichiarò: dove la misura diverge dall'atteso si **registra la divergenza**, e archiviarla fra le righe *«senza cambiare significato»* è il rovescio esatto della regola |
| **E107** | Task 8, in seconda revisione | ⛔ **IL RAPPORTO DICHIARAVA RISCRITTE DUE FRASI E NE AVEVA RISCRITTA UNA, E LA FRASE INCRIMINATA ERA ANCORA LÌ VERBATIM.** La §12.1 di `.superpowers/sdd/task-8-report.md` scriveva che la §4 dello stesso rapporto diceva il falso e che *«entrambe»* — la §4 e la voce `E91` — erano state *«riscritte con richiamo datato invece che corrette in silenzio: un rapporto è un'affermazione come le altre»*. ✅ `E91` **è** stata riscritta, verificato. ⛔ **La §4 no:** portava ancora, senza richiamo e senza data, *«La scadenza è stata **tolta** dal doc di `ask_back` e sostituita col fatto (chi è il chiamante)»* — la **stessa** frase che la §12.1 cita per condannarla, duecento righe più giù. ⛔ **È il difetto che l'ondata esisteva per chiudere, ripetuto dall'ondata stessa, e su una frase che essa cita testualmente.** ✅ **Riprodotto col `grep` prima di correggere:** la ricerca della frase dà **due** righe, la **89** (l'affermazione, al presente) e la **309** (la citazione che la condanna). ✅ Riscritta col proprio richiamo datato. 📌 **Registrata come dato sul METODO e non su chi l'ha scritta: è la quinta volta in due compiti che una correzione dichiarata non è nel diff**, e il rimedio non è un'esortazione ma un passo — **prima di dichiarare fatta una correzione si rilegge il diff** |
| **E108** | Task 8, in seconda revisione | ⚠️ **DUE CONTEGGI DEL BANCO SBAGLIATI NELLA STESSA SEZIONE, ENTRAMBI RICONTATI COL COMANDO.** ① **La cella della mutazione 3a diceva *«le tre sonde LOCAL»*, e le sonde che girano uno scenario `LocalPolicy` sono QUATTRO** — `grep -n "VramPolicy::Local(LocalPolicy)"` sul banco dà quattro corpi di sonda più le due letture di `each_policy_names_itself`, che non ammette niente. ✅ Il conteggio **4/3 è giusto** ed è l'etichetta a non esserlo: la quarta, `the_admission_asks_back_below_its_own_lane_and_spares_a_peer`, **sopravvive e ha ragione di sopravvivere**, perché asserisce `revoking() == 0`, che è anche ciò che *«nessuno fa spazio»* produce. ② **`E93` contava *«le tre asserzioni»* su `allocated()` più *«la quarta, quella della sonda nuova»*, e nel banco ce ne sono SEI:** `grep -c "arbiter.allocated()"` → **6** oggi e **4** su `4b89fea`, cioè le originali erano già quattro e le nuove sono due. ✅ **E la distinzione utile non è il numero ma QUALE, misurata riga per riga:** le **cinque** asserzioni **finali** sono dominate — in ogni riga che uccide la loro sonda scatta prima un `revoking()`, un `matches!(outcome, Queued(_))`, un `promoted.len()` o il `panic!("queued")` — mentre la **sesta**, la **precondizione** di `a_partly_full_machine_…`, **non è dominata**: è la sola asserzione su `allocated()` dell'intero banco che decide un esito, sotto la riga **12**. 📌 Entrambi i conteggi vivevano in due documenti; sono stati **rimisurati** e non riportati |
| **E109** | Task 8, in seconda revisione | ⛔ **UNA CIFRA IN QUATTRO CASE CON DUE VALORI, E UNA MISURA IN TRE CASE: RIDOTTE, NON RIALLINEATE.** ① `E90` apriva ancora con *«CINQUE IN QUATTRO FILE»* e senza rimando a `E101`, che quella voce la nomina come portatrice dell'errore, mentre `E91` **due righe sotto** era stata riscritta nello stesso commit: due trattamenti della stessa specie nella stessa tabella. ⛔ **Le case sono QUATTRO e non tre, contate col `grep` e non a occhio:** `E90` (valore 4), `E101` (5), la sezione del Task 8 di `docs/porta-di-qualita.md` (5) e la **riga di baseline della §1 del rapporto** (4) — è quest'ultima che la revisione non aveva visto. ✅ **La cifra è TOLTA da `E90`**, che ora rimanda a `E101`, unica casa della misura; **tolta anche dalla riga di baseline della §1**, che la portava accanto al proprio comando, cioè presentata come misurata mentre veniva dal brief. ② **La misura di `E100`** — il ritardatario a `501`, `Granted` contro `Queued`, `queued()` 1 contro 2 — era scritta **verbatim in tre documenti vivi**: il sorgente, il registro e l'errata. Il rimedio chiesto ne prevedeva **due**. ⛔ **La copia tolta è quella nel SORGENTE**, e la scelta non è arbitraria: è precisamente la copia che va stantia — `E99` ha dovuto riscrivere nello **stesso file e lo stesso giorno** una scadenza rimasta al presente — quindi il commento porta ora il **fatto** e il rimando, e le cifre restano in `E100` e nella sezione del Task 8 di `docs/porta-di-qualita.md`. 📌 `CLAUDE.md`: *«un puntatore o una cifra che vive in PIÙ documenti si TOGLIE, non si ricorregge»* |
| **E110** | Task 8, in seconda revisione | ⚠️ **TRE DIFETTI DI PROSA CHE NESSUNA MISURA SMENTIVA PERCHÉ NESSUNA MISURA LI TOCCAVA.** ① **`E104` scriveva la dominanza come *«quelle stesse più 8, 13 e 14»*, e la 13 era già nell'insieme di partenza** (`2, 3a, 7, 12, 13`): l'insieme più grande ha **sette** righe e non otto. ② **Le sonde 14 e 15 erano presentate come *«le due direzioni della regola nuova»*, al singolare, e le regole sono DUE** — lo **scarto** e la **corsia**, un cablaggio ciascuna, ciascuna con la propria direzione opposta **altrove**: per lo scarto in `asking_back_marks_nothing_when_the_reclaimable_does_not_cover_the_need`, per la corsia in `the_local_policy_asks_the_lower_lanes_back` (sul cablaggio) e in `only_lanes_below_the_asking_one_are_asked_back` (dentro il meccanismo). Il **vincolo 6 è soddisfatto**: a comprimere due regole in una era la frase e non il banco. ③ **E una terza, trovata leggendo la sezione intorno alle prime due:** l'apertura della campagna diceva *«le ultime due righe esistono per una misura di isolamento e non per uccidere»*, e le ultime due sono ora la **14** e la **15**, che sono uccisori soli nell'intero workspace. La prima ondata ha aggiunto due righe **in coda** lasciando in testa un **puntatore posizionale** — la forma di gotcha **#31** che invecchia più in fretta di una cifra, perché non c'è nessun numero da riconoscere come stantio. Le righe di isolamento si **nominano** (la 12 e la 13), non si contano dal fondo |
| **E111** | Task 8, in seconda revisione | ⛔ **LO STRUMENTO DI MISURA HA SBAGLIATO, E A COGLIERLO È STATO IL `cmp` DEL PROTOCOLLO E NON CHI LO GUIDAVA.** La revoca di ogni mutazione è stata scritta la prima volta come **sostituzione inversa** (NEW → OLD) con un `assert` di unicità sulla stringa NEW, invece che come **ripristino da copia byte-esatta**. ⛔ Sulla riga **2** — `LocalPolicy::may_make_room` da `true` a `false` — dopo la mutazione il file contiene **due** corpi `false`, quello di `RemotePolicy` e quello appena scritto: l'`assert` è scattato, la revoca **non è avvenuta**, e le **sette** righe eseguite dopo hanno girato con `policy.rs` ancora mutato, cioè impilate su una mutazione che nessuno aveva dichiarato. ✅ **Nessuna cifra sbagliata è entrata nei documenti:** il `cmp` con la copia pristine ha stampato *«differ: char 2262, line 52»* a ogni riga, le sette misure sono state **buttate** e rieseguite con la revoca fatta da `cp` della copia pristine più `cmp`. 📌 **Il protocollo lo diceva già** — *«revocata da copia byte-esatta con `cmp` identico»* — e l'errore è stato implementarne una versione più debole credendola equivalente: una revoca che deve **cercare** ciò che ha scritto può fallire, un ripristino no. ⚠️ **Registrata perché è un dato sul METODO:** senza il `cmp` in fondo al giro, sette celle false sarebbero entrate in un registro che dichiara di misurarle |
| **E112** | Task 8, in terza revisione | ⛔ **QUATTRO AFFERMAZIONI FALSE IN PROSA, TOLTE E NON RIALLINEATE:** ogni ondata di questi due compiti ha aperto rilievi nuovi **nella prosa scritta per chiuderne di vecchi**, e una frase cancellata non può essere falsa. ① `E97` descriveva i due residenti delle sonde nuove come **non prelazionabili**, e dopo `a53907e` sono `preemptible` entrambi (`crates/kernel/tests/arbiter_policy.rs`). La seconda ondata aveva corretto il doc dell'aiutante, i due commenti in linea e le due celle del registro, e aveva lasciato **l'errata** — l'unico documento a cui si chiede *cosa prova* una sonda; `E105` la nomina due righe sotto e non la guarda. ✅ **Le due qualificazioni sono tolte**, non riscritte: a tenere i due residenti fuori dai recuperabili è la **corsia**, già scritta dove serve. ② ⛔ **`E90` ed `E109` dichiaravano *«unica casa `E101`»* mentre la cifra dei siti di `Arbiter::new` viveva anche in [`porta-di-qualita.md`](../../porta-di-qualita.md), **due volte nella stessa sezione**, e l'enumerazione di `E109` la elencava senza toglierla. ✅ **Tolta dal registro, che ora rimanda**, invece di riscrivere tre copie con lo stesso numero: la rivendicazione di unicità è diventata **vera** e non è stata cancellata. ③ ⛔ **IL CONTEGGIO DELLA CAMPAGNA ERA RIENTRATO CON DUE VALORI DOPO CHE `E102` L'AVEVA TOLTO**, e in disaccordo dentro un file solo: la sezione del Task 8 del registro lo portava giusto in una riga e sbagliato in due, e l'errata lo divideva fra `E104` e `E105`. ✅ **Ricontata col comando sulla tabella prima di toccarla** — che porta anche `3a`/`3b` e `11a`/`11b`, quindi l'etichetta dell'ultima riga **non è** il numero delle righe, ed è da lì che veniva il valore sbagliato. ⛔ **La cifra è tolta dalla prosa in tutte le case e non riallineata**: la tabella **è** la misura, e un conteggio accanto a una tabella che si può contare è superficie che marcisce. ④ ⚠️ Nel rapporto del compito le righe **2 · 3a · 8** stavano sotto *«insiemi identici»*: condividono il **conteggio**, non l'**insieme** — la **8** uccide `the_remote_policy_…` e le due sonde nuove, le altre due `the_local_policy_…`, `under_the_local_policy_…` e `a_partly_full_machine_…`. ✅ Riga **ristretta** alla coppia, non spiegata. ⛔ **Né codice né sonde toccati**, e nessuna misura rifatta: il cancello è la sola prova che questa passata produce |
| **E113** | Task 9, eseguendo | ⛔ **`now` ESCE DALLA FIRMA DI `set_policy`, e il piano la detta con `now: Monotonic` più un `let _ = now;` dentro.** Il piano dichiara che il segnaposto non deve sopravvivere e lascia la scelta a chi esegue; la scelta è **togliere**. ✅ **Misurato sul sorgente e non dedotto:** `collect_expired` è chiamato in testa a **quattro** operazioni — `admit`, `promote`, `ask_back`, `release` — cioè quelle che toccano i libri, e **non** da `allocated()`, `queued()` e `revoking()`, il cui doc lo dichiara. `set_policy` non legge né scrive `held` né `queues`, quindi non ha scadute da riscuotere e segue quel precedente. ⛔ **E un parametro ignorato è la superficie morta** che questa crate ha tolto a `Record::encode` e rifiutato a `Ipc::accept`. ⚠️ **Il giorno in cui la transizione toccherà i libri l'argomento torna, e torna come ERRORE DI COMPILAZIONE** a ogni sito di chiamata — il primo di produzione nasce al **Task 10** — non come regressione silenziosa |
| **E114** | Task 9, eseguendo | ⛔ **IL DOC DI `Arbiter::policy()` PORTAVA UNA PREVISIONE, E QUESTO COMPITO L'HA MISURATA FALSA.** Diceva *«Task 9's journalled transition is the production reader»*, scritta al **Task 8** su codice che non esisteva: è il gotcha **#57**. `set_policy` legge `self.policy`, il **campo**, perché dentro l'`impl` passare dal proprio getter non compra nessuna proprietà e sarebbe **codice peggiore scritto per far tornare una frase**. ⛔ **La frase è TOLTA e non affiancata da una smentita** (gotcha **#76**), col richiamo datato `2026-08-20`, sul precedente del finding **A-7** dell'audit — *l'argomento regge, l'evidenza no*. Ciò che compra `policy()` restano i **banchi**, e da oggi sono tre |
| **E115** | Task 9, eseguendo | ⛔ **DUE DIVERGENZE MISURATE NELLA CAMPAGNA DI MUTAZIONE DETTATA.** ① La mutazione **4** — *«`set_policy` non scrive nulla»* — uccide **quattro** sonde e non le tre che la tabella prevede: la quarta è `a_transition_cut_between_intent_and_outcome_leaves_the_step_in_doubt`. ② La mutazione **2** — *«assegnare `self.policy` prima di scrivere l'intento»* — **non compila** nella forma ingenua: `VramPolicy` non deriva `Copy` né `Clone`, quindi l'assegnazione **muove** il valore e `policy.name()` sotto è `` error[E0382] ``. Rifatta nella forma che compila. 📌 **E il secondo è un fatto di livello 1 che nessuno aveva scritto:** l'ordine *intento prima dell'effetto* è in parte tenuto dal **compilatore**, non solo dalle sonde |
| **E116** | Task 9, eseguendo | ⚠️ **LA VERIFICA DEI FINE-RIGA DEL BRIEF NON ERA ESEGUIBILE ALLA LETTERA, ED È UN DIFETTO DEL COORDINATORE.** Il brief pretendeva che dopo il compito `tr -cd '\r' < docs/porta-di-qualita.md \| wc -c` desse **2820**, il valore di prima. ⛔ In un file **CRLF integrale** i `CR` sono il **numero di righe**, e il compito **aggiunge righe**: quell'oracolo non poteva essere verde. 📌 **L'invariante vera è più forte e non invecchia:** `CR == LF` e **zero LF nudi**, cioè il file è integralmente CRLF, più il conto del diff che torna. ⚠️ Vale oltre il caso: **una cifra assoluta non è un oracolo su un file che cresce**; lo è una relazione fra due misure dello stesso file |
| **E117** | Task 9, in ondata di correzioni | ⛔ **IL TITOLO DELLA CAMPAGNA NON DICE «NESSUN MUTANTE VIVO», E IL RIFIUTO È DELIBERATO.** La consegna dell'ondata chiedeva che, chiuse le due mutazioni sopravvissute, la frase *«nessun mutante vivo»* tornasse **vera**. Non lo torna: è un **universale sullo spazio delle mutazioni**, e una campagna è un **campione**. Il titolo legge *«NOVE MUTAZIONI, NOVE UCCISE»*, col motivo accanto — vero dei nove, e falsificabile contandoli. 📌 Stessa specie del gotcha **#31** quarta forma: *un aggettivo non si data, si toglie* |
| **E118** | Task 9, in ondata di correzioni | ⛔ **TOLTA ANCHE LA FRASE *«la cifra vive in un posto solo»*, oltre alla casa di troppo.** Il rilievo chiedeva di togliere il numeratore duplicato e lasciare l'affermazione; misurato, `grep -n 'righe su diciannove'` dà **tre** righe, e due sono **richiami datati** dei Task 5 e 8 che portano la cifra **di allora**. ⚠️ Un richiamo datato **non è una casa** — descrive uno stato passato, e riallinearlo sarebbe la nona misura della §12, *«una misura vera di un momento sbagliato è indistinguibile da una misura falsa»* — ma l'universale sarebbe stato **falso al primo `grep`** di chi non lo sa. Tolto l'universale, tenuta la casa unica della cifra viva |
| **E119** | Task 10, pre-controllo | ⛔ **`assert_eq!(run_the_production_graph(&…), Ok(()))` NON COMPILA.** `platform::journal::OpenError` deriva il **solo `Debug`**, quindi `StartupError` non può derivare `PartialEq`. Il Passo 1 lo prevedeva e diceva di passare ad `assert!`; ⚠️ **con una precisazione che il piano non fa:** il `Debug` va **dentro il messaggio** (`assert!(outcome.is_ok(), "…: {outcome:?}")`), o un rosso non dice quale ramo ha sparato |
| **E120** | Task 10, pre-controllo | ⚠️ **`crates/daemon/Cargo.toml` NON va toccato.** Il piano lo elenca fra i file da modificare *«se `kernel` non è già fra le dipendenze dirette — si verifica, non si assume»*: verificato, `kernel`, `platform` e `secrets` ci sono **già**. Il manifesto non compare nel diff, e il vincolo globale 9 regge |
| **E121** | Task 10, decisione del proprietario | ⛔ **LA CHIUSURA DI `E41` È PIÙ GRANDE DI COME IL PIANO LA SCRIVE.** Il Passo 3 la chiude con un'asserzione dentro un test; `E41` chiede *«un fallimento d'avvio che può dire ad alta voce»*. `StartupError` guadagna la variante `ReservedQuota { name }`, il cablaggio propaga con `?`, e il ramo d'errore ha **due sonde permanenti** — `Queued` a `Mib::new(1_500)`, `Refused` a `Mib::new(500)` — perché le vie sono **due** e falliscono diversamente (gotcha **#65**). ⛔ **E non la mutazione 2 del Passo 5:** una direzione tenuta da una mutazione è tenuta da **niente**, gotcha **#72** |
| **E122** | Task 10, decisione del proprietario | ⛔ **`E50`, `E51` ed `E100` NON SI CHIUDONO QUI, e il loro chiusore smette di essere un numero di compito.** Le tre voci dicevano *«chiusore il Task 10»*. Letto contro il compito: il Task 10 **non costruisce nessun ciclo di orchestrazione** — monta il grafo e lancia l'esecutore **una volta**, senza attività — quindi il posto in cui si sceglie fra `admit` e `promote` **non esiste ancora**. È il gotcha **#46** dal verso opposto. Il chiusore è **chi costruirà il primo ciclo**, e la forma non nomina un traguardo perché un traguardo invecchia |
| **E123** | Task 10, in esecuzione | ⛔ **`FOR_EVER` NON È LETTERALMENTE «MAI», e il commento dettato dal piano lo affermava.** ✅ Misurato: `Monotonic::ORIGIN.saturating_add(FOR_EVER)` satura **a** `u64::MAX` e `collect_expired` confronta `expires_at <= now`, quindi una spazzata all'ultimo millisecondo rappresentabile **riscuote entrambe le quote** — `allocated()` torna `Mib(0)` invece di `Mib(1792)`. Commento **riscritto**, e il confine pinzato nelle **due** direzioni: dentro a `u64::MAX - 1`, fuori a `u64::MAX` |
| **E124** | Task 10, in esecuzione | ⚠️ **`main` con un ramo unico `{error:?}` lascia DUE avvisi `field 0 is never read`.** `#[derive(Debug)]` **non conta come lettura** per l'analisi del codice morto. Nessun `#[allow]` (vincolo globale 4) e nessun payload svuotato: **un ramo per variante**, che i payload li legge e dà all'operatore tre frasi diverse |
| **E125** | Task 10, in esecuzione | ⚠️ **La funzione parametrizzata è `run_the_graph(parameters, journal_path)` e non `build_the_arbiter` da sola.** È l'unica forma in cui le due sonde di `E41` tengono il **cablaggio** e non solo la traduzione di `Admission`: misurato, con la forma minima le mutazioni **1** e **10** della campagna restavano vive |
| **E126** | Task 10, in esecuzione | ⛔ **IL REGISTRO DEL TASK 9 DICEVA IL FALSO, ed è stato riscritto.** Dichiarava che il chiamante di produzione di `Arbiter::set_policy` *«nasce al Task 10 col `daemon`»*: il grafo **costruisce** l'arbitro con `VramPolicy::Remote(RemotePolicy)` e non **transita** mai. Una transizione all'avvio sarebbe uno scambio che nessuno chiede, quindi **non è stata inventata** per far tornare la frase (gotcha **#57** più **#76**) |
| **E127** | Task 10, difetto del COORDINATORE | ⛔ **L'ADDENDUM VIETAVA DI TOCCARE `crates/kernel/src/arbiter/`, e lì dentro c'erano TRE frasi rese false dalla decisione dello stesso giorno.** Il divieto proteggeva i paragrafi che dichiarano i mutanti vivi di `E50`/`E51`, e ha coperto anche i puntatori *«il chiusore è il Task 10»*. L'esecutore si è **fermato e l'ha riportato** invece di aggirarlo, il che è la condotta giusta. Corretto dal coordinatore in un commit a parte, e da lì è nato il gotcha **#78** |
| **E128** | Task 10, difetto del COORDINATORE | ⛔ **UN'ESCLUSIVITÀ FALSA DETTATA NEL BRIEF DI UN'ONDATA, e copiata nel sorgente.** Il brief diceva che `Preemption::Never` è *«l'unico posto in cui vive, nel codice di produzione, la parola di ADR-0033»*: censita, quella parola vive in almeno **sei** siti di produzione, due dei quali dentro `daemon` stesso. La formulazione vera è più corta — *il solo sito che **sceglie** `Preemption::Never`* — ed è stata verificata da un revisore contando, non leggendo. 📌 **Un brief è un'affermazione come le altre**, gotcha **#65** applicato a chi coordina |
| **E129** | Task 10, in revisione | ⚠️ **QUATTRO CELLE DELLA CAMPAGNA DAVANO UN MUTANTE VIVO A `255 passate, 0 fallite`**, e `255` è il verde **con** la sonda presente — cioè uno stato in cui quella mutazione non può essere viva. La misura vera è **254 a sonda assente**. ✅ **L'ondata l'ha segnalata e NON presa**, perché non l'aveva rimisurata (gotcha **#15**), e la condotta è giusta; il coordinatore l'ha chiusa con la cifra presa dalla tabella di misura della prima ondata. ⚠️ **E il richiamo diceva *«questa cifra e le tre sotto»*:** le due **sopra** avevano lo stesso difetto e sono state qualificate in una passata successiva |
| **E130** | prima del Task 11, chiudendo il finding **P-2** | ⛔ **IL CODICE DETTATO DAL PASSO 1 DEL TASK 11 RIPETE UNA RAGIONE FALSA, E IL TITOLO DEL COMPITO PURE.** L'intestazione di `worker_tokens.rs` che il piano detta dice *«Until milestone 5 they could not be written at all: all four need to OBTAIN a `Worker`, a `Worker` comes only from `start(grant, ..)`, and nobody issued grants»*, e il titolo del compito dice *«i quattro casi … che il `Grant` sblocca»*. ✅ **Misurato il 2026-08-21 su una sonda usa-e-getta scritta da fuori la crate:** `impl Worker for W` **senza nominare `Grant`** compila e passa, quindi un `Worker` viene dall'**implementare il tratto** — e `crates/kernel/tests/ports_are_implementable.rs` lo fa dal Traguardo 2. Ciò che una concessione sblocca è **una** cosa: chiamare `Process::start`, che ha **zero** chiamanti in tutto il workspace. ⛔ **Chi esegue il Task 11 NON copia quell'intestazione**: la ragione vera è che manca la direzione *«deve scattare»* (§7.1.1 regola 3), e il verbale sta in [`porta-di-qualita.md`](../../porta-di-qualita.md), sezione «P-2». Da qui il gotcha **#79** |
| **E131** | prima del Task 11, pre-controllo del coordinatore | ⛔ **Il comando di verifica del Passo 4 è VACUO: `git diff --stat` non vede i file non tracciati**, e un oracolo nuovo nasce così — risponde vuoto sia se la rigenerazione va bene sia se riscrive un oracolo pre-esistente, cioè proprio la metà che il passo esiste per cogliere. ✅ **Rimedio: `git status --porcelain`**, che mostra insieme i quattro `??` nuovi e che nessuno dei 29 oracoli pre-esistenti sia andato in ` M`. Terza occorrenza di questa specie in questo piano, dopo `E3` ed `E11` |
| **E132** | prima del Task 11, pre-controllo del coordinatore | ⛔ **Il Passo 5 NON DISCRIMINA NIENTE: cancellare uno dei quattro `#[test]` indipendenti di `worker_tokens.rs` non rende rosso nulla**, per costruzione — sono test a sé in una crate d'integrazione, e il criterio del passo scatta su tutti e quattro allo stesso modo (gotcha **#14** applicato a un comando di verifica). ✅ **Rimedio: la seconda direzione vera è una MUTAZIONE DELLA FINTA**, non la cancellazione, revocata da una copia presa prima e mai con `git checkout --` (gotcha **#48**) |
| **E133** | prima del Task 11, pre-controllo del coordinatore | ⛔ **Il commento dettato per `one_grant_starts_one_worker` afferma una copertura che NON ESISTE.** Diceva che un secondo `start` avrebbe voluto un secondo `Grant` tenuto da un caso `compile_fail`: misurato contro i 29 casi pre-esistenti, **nessuno** lo tiene, e `assert!(first.is_ok())` non può fallire perché la finta risponde sempre `Ok` — un'asserzione vacua (forma di `E5`). ✅ **A tenere «una concessione avvia un worker solo» è il compilatore**: `Process::start` prende `Grant` per valore e `Grant` non deriva `Clone` |
| **E134** | prima del Task 11, pre-controllo del coordinatore | ⛔ **Il preambolo che il Passo 3 detta di ripetere in tutti e quattro i casi è MORTO in `talking_without_the_handle.rs`**, che non usa né `a_real_grant` né `FakeWorker` né `FakeProcess` né `a_started_worker`: ripeterlo vi lascerebbe avvisi `dead_code`, e il vincolo globale 4 vieta `#[allow]`. ✅ **Rimedio: quel caso si scrive in linea**, con percorsi `kernel::` completi e zero aiutanti — gli altri tre ripetono davvero il preambolo, ed è la duplicazione dichiarata dal piano |
| **E135** | prima del Task 11, pre-controllo del coordinatore | ⚠️ **Due delle quattro cifre del vincolo globale 10 sono STANTIE.** Rimisurato su `HEAD`: `crates/kernel/src/ports/process.rs` è a **266** `CR` (il vincolo dice 291) e `crates/kernel/tests/ports_are_implementable.rs` a **986** (dice 971); le altre due — `reactor_contract.rs` a 669, `reactor.rs` a 123 — reggono. 📌 **L'invariante che non invecchia è la RELAZIONE `CR == LF`, non la cifra assoluta** — voce `E116` |
| **E136** | Task 11, in esecuzione | ⛔ **IL PREAMBOLO DETTATO DAL PASSO 3 NON COMPILA COME È SCRITTO.** Il passo detta di scriverlo *«con `kernel::` al posto degli `use`»*: un percorso qualificato nell'intestazione di un `impl` non porta il tratto in scope per la sintassi a punto, quindi `FakeProcess.start(..)` dà `E0599`. ✅ **Rimedio misurato eseguendo:** `use kernel::ports::process::{Process as _, Worker as _};` nei tre casi che portano il preambolo — `instructing_after_the_kill.rs:18`, `reading_without_a_receipt.rs:18`, `reading_twice_from_one_receipt.rs:19` |
| **E137** | Task 11, decisione del proprietario | ⛔ **IL RILIEVO `plan-mandated` SULLA DUPLICAZIONE VERBATIM DEL PREAMBOLO È DECISO, NON SOLO REGISTRATO: IL PROPRIETARIO HA DECISO DI TENERLA.** Il Passo 3 impone di ripetere **verbatim** in tre dei quattro casi negativi il preambolo `a_real_grant`/`FakeWorker`/`FakeProcess`/`a_started_worker` — **93 righe per copia, esatte e identiche nelle tre** (rimisurato: da `use kernel::ports::process::{Process as _, Worker as _};` alla chiusura di `a_started_worker`, prima di `fn main`). La revisione l'ha marcata `plan-mandated` e Importante, ed è la condotta giusta: **la paternità del piano non si dà il voto da sola**. ⚠️ **Misurato, «non evitabile» non è del tutto vero**: un `include!` con la sorgente fuori dal glob `tests/compile_fail/*.rs` la collasserebbe a una copia. ✅ **Decisione: si tiene, e non per il costo.** Qui il debito della duplicazione **non esiste per costruzione**: una firma di `Worker` che cambia rompe la compilazione di tutte e tre le copie, uno spostamento di riga rompe i numeri citati nell'oracolo `.stderr` — non una promessa, è come `trybuild` confronta. ⛔ **E l'alternativa AGGIUNGE un meccanismo invece di toglierne uno**: un file fuori dal glob per sempre, o `trybuild` lo compila come un caso in più — e costringerebbe a rigenerare i quattro oracoli, atto deliberato per il vincolo globale 5, mai un riflesso. 📌 **Precedente di casa**: `Yield` è duplicato parola per parola fra `executor_determinism.rs` e `dst_campaign.rs`, stessa ragione — il codice di test non attraversa i confini di crate |
| **E138** | Task 11, ri-revisione | ⛔ **IL COMMENTO CHE IL PASSO 3 DETTA PER `reading_without_a_receipt.rs` AFFERMA LA GARANZIA CHE LA PORTA STESSA SMENTISCE.** Diceva che `read_one` per valore rende impossibile «ask for an answer nobody asked a question for»; il blocco `THE DECLARED LIMIT THIS COSTS` accanto a `SingleReceipt::new` in `crates/kernel/src/ports/process.rs` — **nominato e non numerato**, perché un numero di riga si sposta a ogni commento aggiunto sopra, e questo compito lo ha già spostato di sei — chiama quel limite il più importante del file — `new` è `pub`, e il caso prova solo `E0061`, l'**arità**, non l'autenticità della ricevuta. ✅ **Rimedio: le righe 1-2 riscritte**, stesso numero di righe — l'oracolo `.stderr` cita `:114:20`. 📌 Vincolo globale 14: si registra anche se l'esito del test (`E0061`) non cambia |
| **E139** | Task 11, difetto del COORDINATORE | ⛔ **LA SEZIONE A7 DEL BRIEF DEL TASK 11 VIETAVA DI MODIFICARE `crates/kernel/tests/ports_are_implementable.rs` — *«si RILEGGE, non si modifica … Non serve toccarlo»* — E DENTRO QUEL FILE C'ERANO FRASI CHE IL COMPITO STESSO STAVA PER RENDERE FALSE.** La causa è di specie nota: la passata che ha chiuso il finding **P-2**, la mattina stessa, ha tolto da quel file e da `crates/kernel/src/ports/process.rs` la **ragione** falsa dello scaglionamento e ha lasciato in piedi lo **stato** — *«scaglionati, registrati come non coperti in `docs/porta-di-qualita.md`»* — che il Task 11 ha cambiato poche ore dopo. Un divieto di toccare un file, e dentro quel file frasi rese false dalla decisione dello stesso giorno: è **`E127`** che si ripete, un compito più tardi. 📌 **Un brief è un'affermazione come le altre** (gotcha **#65**), e la regola che ne esce vale oltre il caso: **un censimento che toglie una RAGIONE deve chiedersi se lascia in piedi uno STATO che sta per cambiare.** ✅ **Corretto dalla settima ondata**, che ha tolto le due frasi di stato e messo al loro posto il puntatore al caso `compile_fail` che ora tiene quella direzione, ciascuna col proprio richiamo datato. ⚠️ **E un effetto collaterale che si dichiara invece di lasciarlo scoprire:** quella passata cambia il **numero di righe** di `process.rs` (`266 → 272`). ✅ **Quello della voce `E138` qui sopra non è stato riallineato ma TOLTO:** il blocco è ora **nominato** — `THE DECLARED LIMIT THIS COSTS` — che è il rimedio misurato dalla quinta ondata, perché un numero di riga si sposta a ogni commento aggiunto sopra e questo compito lo ha già spostato due volte. ⚠️ **Restano `:74`, `:95` e `:286`** nel corpo di questo piano e in [`riferimenti.md`](../../riferimenti.md), **non riallineati**: sono verbali datati scritti **prima** di questo compito, erano **già fuori posto prima** che cominciasse, e riallinearli sarebbe la seconda casa che il gotcha **#68** vieta. ⛔ **RICHIAMO DEL 2026-08-24, NONA ONDATA: qui la deriva era QUANTIFICATA — *«i puntatori per numero di riga che lo citano scalano di sei»* — e la quantificazione è TOLTA, non corretta** (gotcha **#76**: una cifra sbagliata accanto a un fatto che regge si toglie). Chi la leggeva concludeva che i tre fossero sei righe fuori posto e, aggiungendone sei, sarebbe atterrato **più lontano di prima**. ⚠️ **E `:74` non è fuori posto di un numero di righe:** cita *«nothing formats a grant»*, che `process.rs` **non contiene più** — `Grant` è emigrato in `crates/kernel/src/arbiter/mod.rs` al Task 4, e la frase **è ancora lì**, spezzata su due righe da un a-capo del commento. 📌 **Il che corregge anche il rilievo che ha aperto questa voce**, che la dava per sparita da tutto `crates/` sulla scorta di un `grep` a riga singola: il `grep` dice dove guardare, mai che cosa c'è (gotcha **#70**). Se una cifra la si vuole, si **riconta eseguendo** e si **data** |
| **E140** | Task 11, nona ondata | ⛔ **UNA DIVERGENZA CHE IL REGISTRO DICHIARAVA «NON REGISTRATA DA NESSUNA PARTE», E LO ERA DAVVERO.** La cella di catalogo di `I2 · §6.10` in [`porta-di-qualita.md`](../../porta-di-qualita.md) constatava che il catalogo scrive `uccidi` mentre il codice si chiama `kill`, e si fermava lì: **descriveva** la lacuna invece di aprirla, che è la forma che il gotcha **#36** chiama *«una nota si legge e si dimentica»*. ✅ **Registrata qui, e misurata invece che ripresa:** `grep -c "uccidi"` su questo piano dava **0**; la spec scrive `uccidi` in §7.4.1 blocco C (*«istruire un worker dopo `uccidi`»*), in §6.10.2 (`Worker::uccidi(self)`) e in §6.10.5, e il sorgente scrive `fn kill(self)` in `crates/kernel/src/ports/process.rs`. ⛔ **Chiuderla è del PROPRIETARIO**, perché tocca la §7.4 che è spec — vincolo globale 7, stesso trattamento di `R5`. ⚠️ **E NON è un allargamento di `R5`, verificato invece che dedotto:** `R5` è intestata *«due celle del catalogo nominano identificatori italiani **che questo traguardo fa esistere**»* ed è **correttamente circoscritta** — `Worker::kill` esiste dal **Traguardo 2**, quindi il suo numeratore non è falso e la voce non va toccata. Questa vive **accanto** a `R5`, non dentro |
| **E141** | Task 11, nona ondata — difetto del COORDINATORE | ⛔ **L'ORACOLO DEL CENSIMENTO DEI FINE-RIGA DATO ALL'ONDATA ERA STANTIO, E NON POTEVA ESSERE VERDE.** Il brief dettava `git ls-files --eol` *«atteso invariato: **140** `i/lf w/crlf`, **75** `i/lf w/lf`, **4** `i/crlf w/crlf`, **3** `i/-text w/-text`»*; misurato all'apertura dell'ondata e di nuovo alla chiusura: **140 · 84 · 4 · 3**. Il **75** è il valore di **prima del Task 11**, e quel compito ha aggiunto **nove** file tracciati — i quattro `crates/kernel/tests/compile_fail/*.rs` delle righe di §6.10.5, i loro quattro `.stderr` e `crates/kernel/tests/worker_tokens.rs`, tutti `i/lf w/lf`: `75 + 9 = 84`. 📌 **È la specie di `E116` un compito più tardi:** una **cifra assoluta** non è un oracolo su un artefatto che cresce; lo è una **relazione** fra due misure dello stesso artefatto. ✅ **L'invariante che regge, e che sostituisce la quaterna:** le tre classi che una passata di sola prosa non può toccare — `i/lf w/crlf`, `i/crlf w/crlf`, `i/-text w/-text` — **non si muovono** (140, 4, 3), e `i/lf w/lf` cresce **solo** dei file tracciati che quella passata aggiunge. La nona ondata non ne aggiunge nessuno, quindi resta **84**, ed è ciò che è stato verificato |
| **E142** | Task 12, pre-controllo del coordinatore | ⛔ **IL PASSO 1 CHIAMA `set_policy` CON QUATTRO ARGOMENTI E LA FIRMA NE PRENDE TRE — `error[E0061]`.** Il quarto è `Monotonic::from_millis(100_000)`. ⚠️ **La divergenza era già registrata, e nel posto giusto:** il doc sopra la firma in `crates/kernel/src/arbiter/mod.rs` dice testualmente *«⛔ IT TAKES NO `now`, AND THAT DIVERGES FROM THE PLAN»*, voce `E113` — il piano è l'unico dei due a non saperlo. ✅ **Rimedio: il quarto argomento è tolto.** 📌 È la **quinta cosa** dell'elenco di `CLAUDE.md` — *il contratto cresce sotto il piano* — nella forma più semplice che abbia: un compito scritto prima si legge contro il codice di **adesso** |
| **E143** | Task 12, pre-controllo del coordinatore | ⛔ **`#[derive(Default)]` SU `Observed` NON COMPILA: `error[E0277]: the trait bound `Mib: Default` is not satisfied`.** ✅ **Rimedio: un `impl Default` scritto a mano**, con `peak: Mib::ZERO`. ⛔ **E l'alternativa è stata scartata sul merito, non per costo:** aggiungere `Default` a `Mib` cambierebbe un **tipo spedito** per la comodità di un banco — è ciò che `ports::process` ha rifiutato a `Grant` per `Debug` — e `Mib::ZERO` esiste proprio perché *«having it named keeps `Mib::new(0)` out of the arithmetic»*. Il `Default` di `Observed` porta la ragione scritta accanto |
| **E144** | Task 12, in esecuzione — difetto del PIANO, e decisione del proprietario | ⛔ **LO SCENARIO DETTATO DAL PASSO 1 PRODUCE UN SOLO ESITO DISTINTO, E IL PASSO 2 DEL MEDESIMO COMPITO ASSERISCE CHE NON DEVE.** ✅ **Riprodotto prima di cambiarlo** (gotcha **#14**), corretti i soli `E142` ed `E143`: **1** esito distinto su **200, 500, 1 000, 2 000 e 20 000** semi, sempre `(granted 4, queued 8, refused 0, peak 6144)`; `the_campaign_sweeps_more_than_one_world` **ROSSA**, con la propria diagnosi. ⛔ **La causa, letta nel codice:** le tre attività sono **identiche in tutto ciò che i libri vedono**, quindi permutarle cambia **chi** viene servito e non **quanti** — e per di più `now` veniva dall'**indice del ciclo**, sotto `RemotePolicy` `may_make_room()` è `false` così `ask_back` non parte mai, nessuno **rilascia**, e la transizione di policy avviene **dopo** `executor.run()`. ⛔ **E il commento di modulo dettato dichiarava un meccanismo che non c'era:** diceva che i guasti vengono *«dal `reactor` per l'interlacciamento e l'orologio virtuale»*, e **l'orologio non veniva mai letto** — l'interlacciamento poi viene da `SeededRng` e non dal reattore. ✅ **Decisione del proprietario, presa: lo scenario si riscrive DENTRO il compito**, perché è un banco di prova e non un contratto di porta né una riga di §7.4 — precedente `E45`, un corpo dettato disatteso sul merito. Lo scenario di oggi: `now` dall'**orologio virtuale** attraverso `SharedClock`, **quattro parti diverse** in taglia, corsia, finestra e ritmo, **pause diverse** dopo una concessione e dopo un accodamento — così il calendario di una parte segue le risposte che ha ricevuto — e le concessioni **tornano**, con `promote` che serve la coda. **Misurato dopo: 7 esiti distinti**, l'ultimo nuovo al seme **38**, stabile fino a **20 000** semi e su **quattro** campionamenti diversi. Il verbale sta nella sezione «La campagna DST dell'arbitro» di [`porta-di-qualita.md`](../../porta-di-qualita.md) |
| **E145** | Task 12, pre-controllo del coordinatore | ⛔ **IL DOC DETTATO PER `RESERVATION` AFFERMA CHE LA CAMPAGNA *«esercita davvero il rifiuto e l'accodamento»*, E IL RIFIUTO NON È RAGGIUNGIBILE.** `Admission::Refused` pretende `asked > ceiling`, cioè `3072 > 8192`; misurato sullo scenario dettato: `refused = 0` su **20 000** semi. ✅ **Rimedio, e non è la sola cancellazione della frase:** nello scenario di oggi nessun profilo di `PARTIES` chiede più di `TOTAL`, la frase **dice questo** invece di quello, e `the_scenario_really_makes_the_admission_decide` asserisce `refused == 0` — così un profilo che crescesse o un tetto che calasse diventano **rossi** invece che silenziosi. ⛔ **Il ramo NON è stato inventato in questa campagna** aggiungendo una parte più grande della macchina: sarebbe una **costante** e non un mondo, e il ramo lo tiene già `a_request_larger_than_the_total_is_refused_and_not_queued` su uno stato costruito |
| **E146** | Task 12, decisione del proprietario | ⛔ **`docs/riferimenti.md` STA FRA I `Modify` DEL COMPITO E NON VA TOCCATO.** Misurato: **zero** commit su **63** dell'esecuzione del Traguardo 5 lo toccano, contro **41** su `docs/porta-di-qualita.md`; l'ultimo che lo ha mosso è `26a529c`, del 2026-08-18, cioè il commit che **crea** questo piano. ⛔ Scegliere fra *«spostare le misure lì»* e *«cambiare la regola»* è **una decisione del proprietario, registrata e non presa**. ✅ **Le misure di questo compito vanno in [`porta-di-qualita.md`](../../porta-di-qualita.md)**, accanto alla sonda che difendono, come hanno fatto tutti gli undici compiti precedenti |
| **E147** | Task 12, pre-controllo del coordinatore | ⛔ **`property_1` PORTA DUE ASSERZIONI CHE NON POSSONO FALLIRE.** `assert!(observed.peak <= TOTAL)` legge un valore che dentro `run` è **già** stato confrontato con `TOTAL`, e `assert_eq!(worlds, SHORT_CAMPAIGN_SEEDS)` conta i giri di un `for` su quell'intervallo. È la forma di `E5`. ✅ **Rimedio: le due sono tolte, e al loro posto c'è la NON-VACUITÀ DI `property_1`** — che è un'affermazione vera e falsificabile: su almeno un seme i libri arrivano a **meno della più piccola riserva** dal tetto, cioè il tetto è stato davvero vincolante da qualche parte. ⚠️ Senza, una passata in cui la macchina ha sempre spazio di avanzo resterebbe verde per un arbitro che ammette fino al doppio del totale. Misurato: `highest = Mib(8192)` su `Mib(8192)`, e la mutazione `M2` la rende rossa col messaggio *«the books never came within Mib(1) of Mib(8192)»* |
| **E148** | Task 12, pre-controllo del coordinatore | ⛔ **LA SOGLIA DI `property_5` È ASSUNTA DAL PIANO E VA MISURATA.** `simultaneous = TOTAL/RESERVATION` fa **2** in divisione intera, e presuppone **una** riserva per tutti: nello scenario di oggi le taglie sono quattro e quella formula non ha un valore da calcolare. ⛔ **E l'oracolo che ne discende si CONFONDE non appena qualcuno rilascia:** *«più concessioni di quante ne stiano insieme»* è soddisfatto dai **rilasci** e non più dalla scadenza — misurato, `released > 0` su **2000 semi su 2000** e `granted` fra **5** e **8**. ✅ **Rimedio: due testimoni DIRETTI**, e sono due affermazioni diverse. `already_collected` è il lato di chi tiene la concessione — torna a renderla e l'arbitro non ce l'ha più; `room_from_expiry` è il lato di chi la chiede — una richiesta che **non entrava nei libri letti nello stesso respiro** viene concessa, e `admit` può farlo solo perché riscuote prima di decidere. ⚠️ Lo scatto è esatto perché fra la lettura di `allocated()` e la chiamata ad `admit` **non c'è nessun `.await`**, e §2.4.1 fa dell'await l'unico punto in cui un'altra attività può correre. La mutazione `M6` uccide il **secondo** lasciando **verde il primo**: sono due |
| **E149** | Task 12, pre-controllo del coordinatore | ⚠️ **IL PASSO 4 CHIEDE DI RIFARE LA MISURA DELLO SPAZIO DEGLI ESITI *«con tre costanti di mescolamento diverse in `crash_point`»*, E `crash_point` NON ENTRA IN QUELLA MISURA:** le passate che contano gli esiti girano con `CrashingJournal::without_crash()`, e il punto di caduta influenza la sola `property_4`. ✅ **Fatte tutte e due, ciascuna dove morde.** L'invarianza del conteggio è misurata sul **campionamento dei semi** — `seed` stesso e `seed * K >> 33` per tre `K` diversi: **7 esiti distinti in tutti e quattro**. Le tre costanti di mescolamento sono usate in `crash_point`: `crashes` **2000/2000** in tutte e tre, `doubted` **999**, **1003**, **1003**. 📌 È lo scopo del gotcha **#24** — provare che il numero appartenga allo **scenario** e non al campione — applicato alla variabile che quel numero davvero muove |
| **E150** | Task 12, pre-controllo del coordinatore | ⚠️ **`scripts/gate.sh` NON STA NELL'INTESTAZIONE `Files:` DEL COMPITO E IL SUO `git add` INCLUDE `scripts`.** Il Passo 6 prevede di aggiungervi la campagna *«se non compare»*. ✅ **Misurato invece che dedotto**, col banco già scritto e la riga non ancora aggiunta: il cancello esce **`GATE GREEN`** e la sua uscita contiene le quattro righe `DST arbiter` **zero volte**, perché il settimo passo nomina i propri bersagli uno per uno. La riga è stata aggiunta. ⛔ **Fine-riga misurati prima e dopo**, che è ciò che il vincolo pretende su un file **interamente CRLF**: l'invariante è `CR == LF == numero di righe`, e regge prima e dopo. ⛔ **RICHIAMO DEL 2026-08-25, PRIMA ONDATA DI CORREZIONI: qui c'erano DUE CIFRE ASSOLUTE — *«`CR 67 → 79`, `LF 67 → 79`»* — e il `79` era FALSO. Sono TOLTE e non riallineate.** Una cifra assoluta non è un oracolo su un artefatto che cresce: `gate.sh` è cresciuto **nello stesso commit** che la scriveva, mentre la relazione resta vera mentre il file cresce — è la forma che il gotcha **#31** premia. ✅ **Rimisurata a `HEAD` il 2026-08-25**: `tr -cd '\r' < scripts/gate.sh \| wc -c` e `tr -cd '\n' < scripts/gate.sh \| wc -c` danno lo **stesso** numero, pari alle righe del file. ⚠️ **E la coda di questa voce va letta stretta, perché oggi le due metà sembrerebbero contraddirsi:** ciò che è stato misurato tradurre è `grep -c $'\r'`; `tr` sotto REDIREZIONE legge i byte, rimisurato oggi sullo stesso file, ed è la forma che `CLAUDE.md` prescrive. ⚠️ **E il documento committato contraddiceva il proprio rapporto d'esecuzione**, che portava il valore giusto: fra i due sopravvive quello committato, e per questo si corregge qui. ⚠️ **E i tre `grep`/`sed` di Git Bash su questa macchina TRADUCONO i fine-riga in modalità testo** — `grep -c $'\r'` risponde **0** su un file che `od -c` mostra come `\r\n` — quindi la modifica è stata fatta con uno script Python in **modalità binaria**, e la misura con `python`, non con `tr`/`wc` |
| **E151** | Task 12, in esecuzione — MUTANTE VIVO, per il proprietario | ⛔ **`Arbiter::release` RISCUOTE LE SCADUTE PRIMA DI CERCARE LA CONCESSIONE, E NIENTE IN TUTTO IL WORKSPACE TIENE QUELLA RIGA.** Mutazione `M9` della campagna di non-vacuità: tolta la chiamata `self.collect_expired(now)` da `release`, l'intero workspace resta **VERDE** — **37 bersagli, 264 passate, 0 fallite, 2 ignorate**. ⚠️ **La ragione, letta nel codice e non supposta:** ogni turno di ogni parte chiama `admit`, che riscuote per primo, quindi quando un ritardatario torna la spazzata l'ha già fatta qualcun altro, e il testimone `already_collected` resta `> 0` lo stesso. ⛔ **RICHIAMO DEL 2026-08-25, PRIMA ONDATA DI CORREZIONI — QUESTA VOCE DICEVA *«il doc di `release` non promette quella riscossione, quindi non c'è nemmeno una frase resa falsa»*, E LA SECONDA METÀ È MISURATA FALSA; e diceva che non si chiude *«per una scelta di perimetro»*, che è la ragione SBAGLIATA.** Si **riscrive** e non si affianca, perché a essere sbagliato è il **fatto**. ✅ **La premessa stretta regge** — il doc di `release` parla del solo consumo del `Grant` — **ma la conclusione no, ed è misurata su una sonda usa-e-getta dello scratchpad cancellata subito dopo:** una concessione da `4_096` ammessa per `5_000 ms` e rilasciata a `5_001` risponde `Err(UnknownGrant)` a `HEAD` e **`Ok(Mib(4096))`** sotto `M9`. Le frasi che quel salto rende false si **cercano** — `grep -rnE "collects before it\|COLLECTS THE EXPIRED" crates/ --include=*.rs` — e ogni riga che restituisce si legge **intera**: quelle che parlano di `release` o di *«ogni operazione»* diventano false, quelle che restano **vere** parlano di `admit`. ⚖️ **NON CHIUSO, E LA RAGIONE È `E30` — GIÀ SCRITTA NEL SORGENTE, non il perimetro `Files:`.** Il blocco *«AND NO PROBE PINS THOSE THREE VALUES»* sul doc di `ReleaseError` dichiara che una sonda che asserisse `Err` a `5_001` congelerebbe la decisione che `E30` mette davanti al proprietario — *«a probe that must be deleted to take a decision is a vote against taking it»* — e nomina **questo** mutante come costo dichiarato: *«moving `collect_expired` after the lookup … turns nothing red, and this paragraph would become false in silence»*. ⛔ **Quindi il mutante non è una scoperta nuova ma il costo che quel blocco aveva già messo per iscritto**, e chi chiudesse questa voce aggiungendo la sonda prenderebbe `E30` cancellando un paragrafo invece di deciderlo: la decisione resta **aperta e del proprietario**, sul precedente dei mutanti vivi dichiarati accanto ad `admit` e a `promote` |
| **E152** | Task 12, prima ondata di correzioni — prescrizione del brief NON eseguita | ⛔ **IL PASSO 6 CHIEDE CHE *«la riga di livello 2 passa a COPERTA»*, E IL MASSIMO SCRIVIBILE OGGI È *«PARZIALE»*.** Il compito non ha mosso nessuna riga di copertura e non ha registrato la divergenza; a `HEAD` la cella di [`porta-di-qualita.md`](../../porta-di-qualita.md) diceva ancora *«A non esistere è la campagna … Traguardo 4»*, mentre il **medesimo file** dedica una sezione intera alla campagna che esiste. ✅ **La sonda che la cella di catalogo pretende è ora ESEGUITA E DATATA** — *«si rompe l'ammissione: la campagna fallisce e nomina il seme (§5.7.1)»*: col confronto di `admit` fra l'allocato e il tetto reso `false`, la campagna esce rossa con `seed 0: allocated Mib(10240) exceeds the total Mib(8192)` e, sulla sonda a seme fisso, con `seed 20260818: allocated Mib(9216) exceeds the total Mib(8192)`. ⛔ **PARZIALE E NON COPERTA, e la ragione sta nella colonna «Difende» della riga stessa:** la §5.7 elenca **cinque** proprietà e la campagna dell'arbitro ne tiene **tre** — le altre due si iniettano sulle porte `process` e `ipc`, che nascono al **Traguardo 6**. ⏳ L'innesco è scritto accanto alla riga. ⛔ **Il numeratore delle coperte NON si muove** — ricontato sul catalogo §7.4.2 delimitando per intestazione: **sette** coperte su **tredici**, **due** parziali, **quattro** scoperte — perché una riga parziale non è una riga chiusa, che è il trattamento già dato a `Q8 · §5.2.1`, a `V3` e alla riga del blocco **B**. ⚠️ **Nessuna riga di catalogo è stata aggiunta**: la §7.4 è spec, vincolo globale 7 |

⛔ **`E16` ed `E17` condividono un unico soggetto, ed è il caso citarlo una volta sola: una
direzione tenuta da una mutazione revocata, e una sonda registrata per un morso che non ha.**
Entrambi sono stati trovati in revisione del Task 4 già chiuso, non al Passo 5: il difetto vive
nel testo del **registro** scritto a chiusura, non nel codice né nel piano.

---

## Il pre-controllo di QUESTO piano — sette cose trovate leggendo il disegno contro il codice

⛔ **Il disegno è un'ipotesi come tutto il resto, e il gotcha #58 dice che si legge contro il
codice di oggi, banchi di prova compresi.** Sette voci, tutte misurate sul sorgente a
`b041620`. Nessuna riapre una decisione: cadono evidenze e collocazioni, non scelte.

### R1 — ⛔ `WorkDescriptor` e `WorkerDescriptor` distano UNA LETTERA, e vivono nella stessa crate

`crates/kernel/src/ports/process.rs:95` dichiara già `pub struct WorkerDescriptor(Vec<u8>)` —
*che cosa avviare*, byte opachi per l'OS. Il disegno chiama `WorkDescriptor` una cosa
**diversa** — *dove vive `cold_start`* — e non nomina la collisione.

| | |
|---|---|
| **Rischio meccanico** | basso: moduli diversi, nessun conflitto di compilazione finché non si importano entrambi **non qualificati** nello stesso file |
| **Rischio umano** | alto, e ha un precedente pagato: la collisione `record`/`boundary` del Traguardo 3 ha riscritto **due oracoli pre-esistenti**, ed è registrata in `riferimenti.md` come permanente |
| **Decisione del piano** | si tiene il nome del disegno, `WorkDescriptor`. ⛔ **E nessun file importa i due non qualificati insieme**: dove servono entrambi si scrive `ports::process::WorkerDescriptor` per esteso |
| ⚠️ **Registrata** | se il proprietario preferisce un nome distante — `PresentationDescriptor`, `WorkDescription` — è una sua decisione, e costa un rinomino di un tipo che nasce qui |

### R2 — ⛔ `Admission` NON può derivare `Debug` né `PartialEq`, perché `Grant` non li ha

`process.rs:74` è esplicito: *«NO `Debug` EITHER … nothing formats a grant»*. `Admission`
porta un `Grant` nella variante `Granted`, quindi non può derivarli senza darli a `Grant` —
cioè senza aggiungere un derive **per comodità di banco**, che è precisamente ciò che quel
commento ha rifiutato.

**Conseguenza sulle sonde, e va saputa prima di scriverle:** niente `assert_eq!` su un
`Admission`. Si usa `matches!` e il pattern matching con `let … else`, poi si confrontano i
`Mib`, che `Debug` e `PartialEq` ce l'hanno.

### R3 — ⚠️ La riga di catalogo `Q2 · §5.1` è UNA, e il disegno la vuole tenuta da DUE regole

Il catalogo §7.4.1 C ha una riga sola — *«MiB assegnati a millisecondi»*, contro-sonda
*«ciascuno con sé stesso»* — mentre §8.2 del disegno la vuole tenuta *«in due regole: non si
passa l'uno per l'altro, e non esiste via `From`»*.

| | |
|---|---|
| **Due casi per una riga È ammissibile** | precedente locale: la riga `Q9 · I6 · V20 · §4.9` ha `record_without_trust_label.rs` **e** `trust_has_no_default.rs`, e la spec scrive *«due casi per una riga sola, perché le metà sono due»* |
| ⛔ **Ma il precedente più vicino è l'opposto** | per `Monotonic`/`WallTime` la regola *«non esiste una via `From`»* ha preso una **riga propria**, perché prima *«era scritta in un commento del sorgente, cioè era un'intenzione»* |
| **Decisione del piano** | si scrivono **tutti e tre** i casi (le due direzioni più la via `From`), perché costano poco e la seconda direzione è quella che su `V29 · §2.1` lasciava la porta verde |
| ⛔ **Registrata, non presa** | la riga di catalogo è **una** e formulata in **una direzione**; la via `From` non ne ha una propria. Allargarla o sdoppiarla è spec, vincolo globale 7 |

### R4 — ⚠️ Il disegno diverge dalla §5.2 e NON lo dichiara

La §5.2 elenca **due** campi — `preemptible: booleano` e `release_grace: durata` — il disegno
ne fa **uno**, `Preemption::Never | After(Millis)`.

La divergenza è **giusta nel merito**: la §5.3 punto 3 pretende che `InRevoca` sia *«non
rappresentabile»* per un profilo non prelazionabile, e con un booleano non lo è — resta un
controllo a runtime. L'enum fa sparire **due** stati illegali insieme. Ma il disegno dichiara
la divergenza sulla §5.1 (un parametro invece di tre) e **tace su questa**, che è della stessa
specie.

**Registrata qui perché il proprietario possa ribaltarla vedendola.**

### R5 — ⚠️ Due celle del catalogo nominano identificatori ITALIANI che questo traguardo fa esistere

| Cella | Dice | Diventerà |
|---|---|---|
| `V4` | *«distinguere `Concessa`, `Rifiutata` e `InCoda` compila»* | `Admission::Granted` · `Refused` · `Queued` |
| `I2 · §5.3` | *«`InRevoca` per un profilo non prelazionabile»* | `Activity::Preemptible(PreemptibleState::Revoking { .. })` |

Oggi sono **prosa**: nominano concetti che non esistono nel sorgente. Dal giorno in cui i tipi
esistono diventano **riferimenti al codice scritti in italiano**, che la §1.0 vieta.

⛔ **Registrata, non presa:** il catalogo è spec.

### R6 — ⛔ `Process::start` CONSUMA il `Grant`, e anche `Arbiter::release` lo consumerà

`process.rs:286` — `fn start(&mut self, grant: Grant, …)`, per valore. Se anche il rilascio
consuma la concessione, al **Traguardo 6** chi avvia un worker **non ha più nulla da
rilasciare**, e la metà di cablaggio della proprietà 2 (§0.1 del disegno) non ha una strada.

| | |
|---|---|
| **Non è un difetto oggi** | nessuno chiama `start`: non c'è implementazione di `process` |
| **La via naturale, dichiarata perché il Traguardo 6 non la riscopra** | `Worker::kill(self)` restituisce la concessione — `-> Result<Grant, ProcessError>` — così l'uccisione **è** il rilascio, che è esattamente ciò che la §0.1 del disegno vuole cablare |
| ⛔ **Non si costruisce ora** | cambiare `kill` oggi sarebbe un'astrazione per un consumatore che non esiste, gotcha **#46** dal verso sbagliato. Si **dichiara**, accanto a `Grant` |

### R7 — ⛔ Cablare un giornale in `daemon` fa scrivere un file vero al test che già esiste

`crates/daemon/src/main.rs` ha un test — `the_production_graph_assembles_and_the_executor_runs_to_completion`
— che chiama `run_the_production_graph()`. Dandogli un `FileJournal`, quel test comincia a
creare un archivio su disco.

⛔ **Gotcha #52, misurato al Traguardo 3:** un percorso fisso in una cartella condivisa è un
difetto **mascherato da Windows** — `remove_dir_all` fallisce a file aperto, quindi il rosso
esce **su Linux**, che è il secondo sistema del progetto.

**Rimedio dettato dal Task 10, ed è il pattern già in uso in `platform`:** il percorso è un
**argomento** della funzione di cablaggio, `main` passa il letterale di default (§11 vincolo
11), e il test passa una cartella **privata per call site** dal `line!()`, con prefisso
diverso da quello dei due banchi di `platform` — un numero di riga è unico dentro **un** file
solo.

---

## Le decisioni prese da questo piano

⛔ **Il disegno lascia alle parole ciò che il piano deve tradurre in firme.** Otto decisioni,
e la prima governa le altre. **Sono tutte ribaltabili da chi esegue**, purché la divergenza
finisca nell'errata.

| # | Decisione | Perché |
|---|---|---|
| **D1** | ⛔ **`Arbiter::new` prende `Parameters`, non un `Mib` nudo** | è la forma che `Executor::new` già ha, ed è la riga di catalogo `V29 · §2.8 · ADR-0034` — *«costruire una decisione senza i parametri consegnati»*. Un `Mib` nudo farebbe leggere il totale alla radice di composizione e lo consegnerebbe **fuori** dal meccanismo che ADR-0034 esiste per imporre |
| **D2** | **`Grant` porta un `GrantId` privato, e `release` lo CONSUMA** | consumare è livello 1: un doppio rilascio **non compila**. La conseguenza sul Traguardo 6 è **R6**, dichiarata |
| **D3** | **`release` restituisce `Result<Mib, ReleaseError>`** | il `Mib` è la riserva che torna nel budget, ed è ciò che la sonda del disegno §8.4 pretende di vedere. L'`Err` **è raggiungibile** — una concessione emessa da un arbitro e rilasciata su un altro — quindi non è un `Result` che non può mai essere `Err` |
| **D4** | ⛔ **`ComputeClass` implementa `Ord` A MANO, da una chiave esplicita** | `Ord` derivato segue l'ordine di **dichiarazione**: riordinare le varianti cambierebbe le priorità **senza che nulla diventi rosso**. Togliere la trappola batte sorvegliarla |
| **D5** | **La riscossione delle scadute è PRIVATA e gira in testa a ogni operazione** | *«prima di decidere, l'arbitro riscuote»*. Una `collect` pubblica sarebbe un secondo modo di far avanzare lo stato, cioè una via che nessuna sonda copre |
| **D6** | **La transizione di policy prende il giornale PER RIFERIMENTO**, non lo possiede | stessa ragione meccanica del reattore in §6.2 del disegno: un giornale posseduto dall'arbitro avrebbe due proprietari il giorno in cui il chiamante ne ha bisogno |
| **D7** | ⛔ **Il modulo `arbiter` è una CARTELLA con tre file** | `resource.rs` (i tipi della risorsa), `policy.rs` (le due policy), `mod.rs` (l'arbitro e il ciclo della concessione). Un file solo sarebbe il più grande di `kernel` e mescolerebbe tre responsabilità |
| **D8** | ⛔ **`Grant` NON è ri-esportato da `ports::process`** | un ri-export darebbe due percorsi allo stesso tipo e lo farebbe leggere come proprietà della porta, mentre il disegno lo sposta proprio perché appartiene a **chi lo emette** |

---

## La struttura dei file

| File | Responsabilità | Compito |
|---|---|---|
| `crates/kernel/src/arbiter/mod.rs` | `Arbiter`, `Grant`, `GrantId`, `Admission`, `TicketId`, `Activity`, `ReleaseError` — il ciclo della concessione | 1, 4, 5, 6, 7, 9 |
| `crates/kernel/src/arbiter/resource.rs` | `Mib`, `ComputeClass`, `Preemption`, `ResourceProfile`, `WorkDescriptor` — il modello della risorsa | 1, 2, 3 |
| `crates/kernel/src/arbiter/policy.rs` | `VramPolicy`, `RemotePolicy`, `LocalPolicy` — *si può fare spazio?* | 8 |
| `crates/kernel/src/lib.rs` | dichiara `pub mod arbiter;` | 1 |
| `crates/kernel/src/parameters.rs` | guadagna `total_vram` | 5 |
| `crates/kernel/src/ports/process.rs` | perde `Grant`, lo importa | 4 |
| `crates/kernel/tests/arbiter_resource.rs` | sonde a esempi sui tipi della risorsa | 1, 2, 3 |
| `crates/kernel/tests/arbiter_admission.rs` | sonde a esempi sull'ammissione, la coda, le scadenze, la revoca | 5, 6, 7 |
| `crates/kernel/tests/arbiter_policy.rs` | sonde a esempi sulle due policy e sulla transizione giornalata | 8, 9 |
| `crates/kernel/tests/compile_fail/*.rs` | i casi di livello 1 | 1, 3, 4, 8, 11 |
| `crates/kernel/tests/ports_are_implementable.rs` | l'import di `Grant` si sposta | 4 |
| `crates/daemon/src/main.rs` | arbitro, giornale, due concessioni permanenti | 10 |
| `crates/simulator/tests/arbiter_campaign.rs` | la campagna DST — proprietà 1, 4, 5 | 12 |
| `docs/porta-di-qualita.md` | il registro, riallineato a ogni compito | tutti |

---

# Parte 1 — il vocabolario della risorsa

Tre compiti, e nessuno di essi conosce l'arbitro. Producono i tipi con cui l'arbitro
parlerà, e ciascuno prepara o chiude una riga di catalogo di livello 1 **già scritta**.

---

### Task 1: il modulo `arbiter` e `Mib`

**Files:**
- Create: `crates/kernel/src/arbiter/mod.rs`
- Create: `crates/kernel/src/arbiter/resource.rs`
- Modify: `crates/kernel/src/lib.rs`
- Create: `crates/kernel/tests/arbiter_resource.rs`
- Create: `crates/kernel/tests/compile_fail/mib_as_millis.rs`
- Create: `crates/kernel/tests/compile_fail/millis_as_mib.rs`
- Create: `crates/kernel/tests/compile_fail/no_conversion_from_mib_to_millis.rs`
- Create: `crates/kernel/tests/compile_fail/no_conversion_from_millis_to_mib.rs`
- Modify: `docs/porta-di-qualita.md`

**Interfaces:**
- Consumes: `kernel::time::Millis`, che esiste dal Traguardo 2.
- Produces: `kernel::arbiter::Mib` — `Mib::ZERO`, `Mib::new(u64) -> Mib`,
  `Mib::get(self) -> u64`, `Mib::saturating_add(self, Mib) -> Mib`,
  `Mib::saturating_sub(self, Mib) -> Mib`. Tutti `const`.
  Derive: `Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash`.

⛔ **La riga di catalogo che questo compito chiude:** §7.4.1 blocco C, `Q2 · §5.1` — *«MiB
assegnati a millisecondi»*, contro-sonda *«ciascuno con sé stesso»*.

- [ ] **Passo 1: scrivere i quattro casi negativi, che oggi devono già fallire**

`crates/kernel/tests/compile_fail/mib_as_millis.rs`:

```rust
// Rule A, one direction: VRAM is not a duration. §5.1 -- "swapping MiB for milliseconds
// MUST NOT COMPILE", the same mechanism that separates `Instruction` from `Untrusted`.
fn takes_a_duration(_value: kernel::time::Millis) {}

fn main() {
    takes_a_duration(kernel::arbiter::Mib::new(4096));
}
```

`crates/kernel/tests/compile_fail/millis_as_mib.rs`:

```rust
// Rule A, the OTHER direction. Neither type is the stricter one -- a memory size and a
// duration each go wrong in their own way -- which is why both directions have a case
// here, exactly as `Monotonic`/`WallTime` do. A guard written in one direction only left
// the gate GREEN on the dangerous side once already (the widened `V29` row of §7.4.1).
fn takes_a_size(_value: kernel::arbiter::Mib) {}

fn main() {
    takes_a_size(kernel::time::Millis::new(4096));
}
```

`crates/kernel/tests/compile_fail/no_conversion_from_mib_to_millis.rs`:

```rust
// Rule B: no `From`/`Into` PATH exists between the two. The day somebody writes the impl
// this case starts COMPILING, and trybuild reports that outright as `error` instead of
// through its oracle -- so a bulk regeneration of the `.stderr` files cannot disarm it.
// Gotcha #42, strong form.
fn main() {
    let size = kernel::arbiter::Mib::new(4096);
    let _duration: kernel::time::Millis = size.into();
}
```

`crates/kernel/tests/compile_fail/no_conversion_from_millis_to_mib.rs`:

```rust
// Rule B, the other direction. Same argument, written once above.
fn main() {
    let duration = kernel::time::Millis::new(4096);
    let _size: kernel::arbiter::Mib = duration.into();
}
```

- [ ] **Passo 2: eseguirli e verificare che falliscano per il motivo GIUSTO**

Run: `cargo test --locked -p kernel --test compile_fail`

Expected: **FAIL**. ⛔ E il rosso va **letto**: i quattro casi devono fallire perché
`kernel::arbiter` **non esiste** (`E0433`), non perché la regola morde. È il rosso della
partenza, non quello che si cerca — la sonda vera arriva al passo 5.

- [ ] **Passo 3: scrivere `Mib`**

`crates/kernel/src/arbiter/resource.rs`:

```rust
//! The resource model of §5.1: VRAM as a type of its own, and compute as ORDERED LANES
//! rather than a number.

use crate::time::Millis;

/// VRAM, in whole MiB.
///
/// ⛔ A TYPE OF ITS OWN AND NOT A BARE INTEGER, for the reason §5.1 gives in one line:
/// swapping MiB for milliseconds MUST NOT COMPILE. It is the same mechanism that separates
/// `Instruction` from `Untrusted` and `Monotonic` from `WallTime`, and it is held by four
/// cases in `tests/compile_fail/` -- two for "neither is passable for the other", two for
/// "no `From` path exists".
///
/// ⛔ WHOLE MiB, AND THE QUANTISATION IS THE POINT. The resource is quantised; an integer
/// removes every question about rounding, and a rounding question inside a deterministic
/// decision path is debt (§5.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mib(u64);

impl Mib {
    /// No VRAM at all. It is the identity of `saturating_add` and the floor of
    /// `saturating_sub`, and having it named keeps `Mib::new(0)` out of the arithmetic.
    pub const ZERO: Mib = Mib(0);

    pub const fn new(value: u64) -> Self {
        Mib(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }

    /// Saturating and NOT wrapping, and the DIRECTION is what makes this safe rather than
    /// merely defined.
    ///
    /// ⛔ An overflow saturates to `u64::MAX`, which is GREATER than any ceiling, so the
    /// request is REFUSED. A wrapping add would yield a SMALLER number and produce
    /// over-admission -- Q2 giving way in silence, which is the one failure the whole
    /// arbiter exists to prevent. It is the same argument already written beside
    /// `Monotonic::saturating_add`, landing on the same side.
    pub const fn saturating_add(self, other: Mib) -> Self {
        Mib(self.0.saturating_add(other.0))
    }

    /// Saturating to zero. A budget cannot go negative, and a wrapping subtraction would
    /// yield an enormous free budget -- the same over-admission by the other road.
    pub const fn saturating_sub(self, other: Mib) -> Self {
        Mib(self.0.saturating_sub(other.0))
    }
}
```

`crates/kernel/src/arbiter/mod.rs`:

```rust
//! The GPU arbiter (§5, ADR-0005, ADR-0006, ADR-0033).
//!
//! ⛔ IT IS LOGIC, NOT A PORT, and the distinction is structural rather than tidy.
//! `crate::ports` declares SIX families and §3.1 calls that list EXHAUSTIVE; a seventh
//! would be a decision no ADR has taken. So the arbiter has no real implementation and no
//! fake: there is ONE, and in simulation that one runs. That is what makes the DST
//! campaign a proof about the product instead of about its imitation (ADR-0020).
//!
//! ⛔ AND THE SHAPE NOT TO BUILD, written because it is the natural temptation: a trait
//! `Arbiter` with two implementations "so faults can be injected". Faults are injected
//! FROM THE PORTS THE ARBITER USES -- `reactor` and `journal` -- never inside it. A trait
//! here would be an abstraction with no second implementor.
//!
//! ⛔ THE ARBITER NEVER READS THE CLOCK. Every operation that needs time takes
//! `now: Monotonic` as an ARGUMENT: the shape of ADR-0034, and a mechanical reason on top
//! of it -- `Reactor::wait_until` takes `&mut self`, so an arbiter that owned a reactor
//! would give it two owners, itself and the executor, and the borrow would not pass.

pub mod resource;

pub use resource::Mib;
```

E in `crates/kernel/src/lib.rs`, dopo `pub mod reconcile;`:

```rust
pub mod arbiter;
```

- [ ] **Passo 4: le sonde a esempi sull'aritmetica**

`crates/kernel/tests/arbiter_resource.rs`:

```rust
//! What the compiler cannot hold about the resource model: the DIRECTION in which the
//! arithmetic saturates, and the explicit lane order.

use kernel::arbiter::Mib;

/// ⛔ THE DIRECTION IS THE ASSERTION, not the fact that it does not panic. A wrapping add
/// would give a SMALLER number than the ceiling and admit a request that does not fit.
///
/// ⚠️ TWO VALUES AND NOT ONE (gotcha #48): a single pair can agree with the mutation by
/// accident. The second pair overflows by a different amount.
#[test]
fn an_overflowing_sum_saturates_upwards_so_a_request_is_refused() {
    let ceiling = Mib::new(16_384);

    let first = Mib::new(u64::MAX).saturating_add(Mib::new(1));
    assert_eq!(first, Mib::new(u64::MAX));
    assert!(
        first > ceiling,
        "a wrapped sum would land BELOW the ceiling and be admitted"
    );

    let second = Mib::new(u64::MAX - 5).saturating_add(Mib::new(9));
    assert_eq!(second, Mib::new(u64::MAX));
    assert!(second > ceiling);
}

/// The floor, and its own failure: a wrapped subtraction would give some 18 quintillion
/// MiB of free budget -- over-admission by the other road.
#[test]
fn a_subtraction_below_zero_saturates_to_zero_and_not_to_an_enormous_budget() {
    assert_eq!(Mib::new(3).saturating_sub(Mib::new(4)), Mib::ZERO);
    assert_eq!(Mib::ZERO.saturating_sub(Mib::new(1)), Mib::ZERO);
}

/// The ordinary path, so the two probes above are not the only thing this type is held by.
#[test]
fn the_ordinary_arithmetic_is_exact() {
    assert_eq!(Mib::new(4096).saturating_add(Mib::new(2048)), Mib::new(6144));
    assert_eq!(Mib::new(4096).saturating_sub(Mib::new(2048)), Mib::new(2048));
    assert_eq!(Mib::new(4096).get(), 4096);
}
```

- [ ] **Passo 5: la porta, e i quattro oracoli LETTI**

Run: `cargo test --locked -p kernel --test compile_fail`
Expected: **FAIL**, perché i quattro `.stderr` non esistono ancora.

Poi, **una volta sola e deliberatamente** (vincolo globale 5):

```bash
TRYBUILD=overwrite cargo test --locked -p kernel --test compile_fail
```

Run: `git diff --stat crates/kernel/tests/compile_fail/`
Expected: **quattro** file `.stderr` nuovi e **nessun altro toccato**. ⛔ Se un `.stderr`
pre-esistente compare nel diff, la rigenerazione ha riscritto un oracolo che non le
apparteneva: si ripristina **quel** file e si indaga.

⛔ **E i quattro oracoli si LEGGONO**, non si guardano: `mib_as_millis` e `millis_as_mib`
devono dire **`E0308` mismatched types**; i due `no_conversion_*` devono dire **`E0277`**,
il tratto `Into` non soddisfatto. Un oracolo che dicesse `E0433` significherebbe che il
modulo non si risolve e che il caso fallisce **per il motivo sbagliato** (gotcha #24).

- [ ] **Passo 6: la seconda direzione — che i due casi `From` scattino come `error`**

⛔ **È la misura che rende la regola B diversa da un'intenzione**, e va **eseguita**, non
dedotta. Aggiungere temporaneamente in fondo a `crates/kernel/src/arbiter/resource.rs`:

```rust
impl From<Mib> for Millis {
    fn from(value: Mib) -> Self {
        Millis::new(value.get())
    }
}
```

Run: `cargo test --locked -p kernel --test compile_fail 2>&1 | grep -E "error|mismatch|ok"`
Expected: `no_conversion_from_mib_to_millis.rs` riportato come **`error`** — cioè **ha
compilato** — mentre gli altri restano `ok`.

⛔ **E prima di crederci, provare che la mutazione si sia APPLICATA:**
`grep -c "impl From<Mib> for Millis" crates/kernel/src/arbiter/resource.rs` → `1`. Un verde
da una mutazione che non è entrata è la vacuità che si sta cacciando (gotcha #48).

Ripetere per l'altra direzione con `impl From<Millis> for Mib`, poi **revocare**.

⚠️ **Il ripristino si fa con lo strumento di edit, mai con `git checkout --`**: questo
compito sta scrivendo lo stesso file, e `git checkout` cancellerebbe il lavoro non
committato — gotcha **#48**, dodicesima forma.

- [ ] **Passo 7: il cancello, il registro, il commit**

Run: `bash scripts/gate.sh`
Expected: `GATE GREEN`.

Poi in [`docs/porta-di-qualita.md`](../../porta-di-qualita.md):
- la riga `Q2 · §5.1` passa da **scoperta** a **coperta**, coi quattro casi per nome e la
  misura del passo 6 riportata coi comandi;
- ⛔ si **registra come voce aperta** che la riga di catalogo è **una** e formulata in **una
  direzione**, mentre i casi sono quattro e le regole due (**R3** di questo piano) —
  registrata, non presa, perché §7.4 è spec (vincolo globale 7);
- i conteggi dei casi `compile_fail` si **ricontano** —
  `ls crates/kernel/tests/compile_fail/*.rs | wc -l` — e non si deducono da diciotto più
  quattro. Gotcha **#31**.

```bash
git add crates/kernel/src/arbiter crates/kernel/src/lib.rs crates/kernel/tests/arbiter_resource.rs crates/kernel/tests/compile_fail docs/porta-di-qualita.md
git commit -m "feat(arbiter): il modulo nasce, e Mib non e un intero nudo"
```

---

### Task 2: `ComputeClass` e `Preemption`

**Files:**
- Modify: `crates/kernel/src/arbiter/resource.rs`
- Modify: `crates/kernel/src/arbiter/mod.rs`
- Modify: `crates/kernel/tests/arbiter_resource.rs`
- Modify: `docs/porta-di-qualita.md`

**Interfaces:**
- Consumes: `Mib` del Task 1, `kernel::time::Millis`.
- Produces: `ComputeClass::{Realtime, Interactive, Batch}` con `priority(self) -> u8` e
  `Ord` **scritto a mano**; `Preemption::{Never, After(Millis)}` con
  `grace(self) -> Option<Millis>`.

⚠️ **Nessuna riga di catalogo si chiude qui, e va detto invece di lasciarlo dedurre.**
L'ordine delle corsie è un **valore**, non una forma: il compilatore non ha niente da
rifiutare, quindi lo tiene una **sonda a esempi**.

- [ ] **Passo 1: la sonda che fissa l'ordine PER NOME, e che oggi non compila**

In `crates/kernel/tests/arbiter_resource.rs`, in coda:

```rust
use kernel::arbiter::{ComputeClass, Preemption};
use kernel::time::Millis;

/// ⛔ THE ORDER IS FIXED BY NAME, and what that buys is worth stating. `Ord` DERIVED
/// follows DECLARATION order, so reordering the variants -- a tidy-up, a rename, an
/// alphabetisation -- would silently change the arbiter's priorities and NOTHING WOULD GO
/// RED. The order lives in an explicit key (`priority`), and this probe pins it by name.
///
/// ⚠️ SO REORDERING THE VARIANTS LEAVES THIS PROBE GREEN, deliberately: the trap has been
/// REMOVED rather than watched. What turns it red is changing the key -- which is the only
/// place the order is stated.
#[test]
fn the_lane_order_is_pinned_by_name_and_realtime_comes_first() {
    assert!(ComputeClass::Realtime < ComputeClass::Interactive);
    assert!(ComputeClass::Interactive < ComputeClass::Batch);
    assert!(ComputeClass::Realtime < ComputeClass::Batch);

    // The key itself, so a reader does not have to infer it from three inequalities.
    assert_eq!(ComputeClass::Realtime.priority(), 0);
    assert_eq!(ComputeClass::Interactive.priority(), 1);
    assert_eq!(ComputeClass::Batch.priority(), 2);
}

/// ⛔ THE COUNTER-PROBE OF THE ONE ABOVE, and it is the half that is easy to skip: the
/// ordering has to be TOTAL, so a `BTreeMap` keyed on a lane gets one bucket per lane and
/// not two that compare equal.
#[test]
fn the_three_lanes_are_distinct_and_the_ordering_is_total() {
    let lanes = [
        ComputeClass::Realtime,
        ComputeClass::Interactive,
        ComputeClass::Batch,
    ];
    for (index, left) in lanes.iter().enumerate() {
        for (other, right) in lanes.iter().enumerate() {
            assert_eq!(index == other, left == right);
            assert_eq!(index.cmp(&other), left.cmp(right));
        }
    }
}

/// ⛔ WHAT THE TYPE MAKES UNSAYABLE, and it is TWO illegal states and not one: a
/// non-preemptible profile CANNOT CARRY a grace time, and a preemptible one CANNOT LACK
/// one. A boolean plus a separate duration expresses both.
#[test]
fn a_grace_time_exists_exactly_when_the_profile_is_preemptible() {
    assert_eq!(Preemption::Never.grace(), None);
    assert_eq!(
        Preemption::After(Millis::new(250)).grace(),
        Some(Millis::new(250))
    );
}
```

- [ ] **Passo 2: eseguirla e vederla fallire**

Run: `cargo test --locked -p kernel --test arbiter_resource`
Expected: **FAIL** con `E0432`/`E0433` — `ComputeClass` e `Preemption` non esistono.

- [ ] **Passo 3: scrivere i due tipi**

In coda a `crates/kernel/src/arbiter/resource.rs`:

```rust
/// The three compute lanes of §5.1 and design/02. NOT a number: contention on compute is
/// governed by ORDER plus a "reduce your footprint" signal, never by an amount.
///
/// ⛔ `Ord` IS WRITTEN BY HAND, FROM AN EXPLICIT KEY, and that is the decision rather than
/// ceremony. A DERIVED `Ord` follows the order in which the variants are DECLARED, so
/// reordering them would change the arbiter's priorities and NOTHING WOULD GO RED.
/// Removing the trap beats watching it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComputeClass {
    /// Wake word, VAD, STT, TTS. Never preempted, and its VRAM is held by a PERMANENT
    /// GRANT rather than subtracted from the budget -- a subtraction without a holder
    /// leaves I2 false for that consumer (ADR-0033, gotcha #4).
    Realtime,
    /// Chat and the foreground agent. Served before `Batch`.
    Interactive,
    /// 3D render, indexing, background runs. May wait indefinitely.
    Batch,
}

impl ComputeClass {
    /// The order, stated ONCE and in one place. Lower is served first.
    pub const fn priority(self) -> u8 {
        match self {
            ComputeClass::Realtime => 0,
            ComputeClass::Interactive => 1,
            ComputeClass::Batch => 2,
        }
    }
}

impl PartialOrd for ComputeClass {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ComputeClass {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.priority().cmp(&other.priority())
    }
}

/// Whether the arbiter may take the resource back, and -- when it may -- how long the
/// holder gets to hand it over.
///
/// ⛔ NOT A BOOLEAN, AND THE GRACE TIME LIVES INSIDE THE VARIANT. §5.3 point 3 wants
/// `Revoking` to be NOT REPRESENTABLE for a non-preemptible profile -- "not constructible",
/// not "checked at runtime". A boolean cannot do that. This enum makes TWO illegal states
/// disappear together: a non-preemptible profile that carries a grace time, and a
/// preemptible one that has none.
///
/// ⚠️ DIVERGENCE FROM THE LETTER OF §5.2, DECLARED. That table lists TWO fields --
/// `preemptible: boolean` and `release_grace: duration` -- and this is ONE. The spirit of
/// §5.3 point 3 is what forces it; the letter of §5.2 is what it costs. Registered in the
/// errata of the milestone 5 plan so the owner can overturn it seeing it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Preemption {
    /// The arbiter never takes it back. ⚠️ NOT "permanent": a job that cannot be
    /// interrupted still FINISHES and releases. Permanence is not a type -- it is "nobody
    /// calls release".
    Never,
    /// The arbiter may take it back, and the holder gets this long to comply.
    After(Millis),
}

impl Preemption {
    /// The grace time, when there is one. `None` is not a missing value: it is the
    /// statement that this profile is never revoked.
    pub const fn grace(self) -> Option<Millis> {
        match self {
            Preemption::Never => None,
            Preemption::After(grace) => Some(grace),
        }
    }
}
```

E in `crates/kernel/src/arbiter/mod.rs`:

```rust
pub use resource::{ComputeClass, Mib, Preemption};
```

- [ ] **Passo 4: verde, poi le DUE mutazioni**

Run: `cargo test --locked -p kernel --test arbiter_resource`
Expected: **PASS**, sei test.

**Mutazione che deve UCCIDERE:** nella chiave, `Realtime => 2` e `Batch => 0`.
Expected: **FAIL** su `the_lane_order_is_pinned_by_name_and_realtime_comes_first`.

⛔ **Contro-mutazione, ed è la direzione che si dimentica:** **riordinare le varianti**
dell'enum — `Batch`, `Interactive`, `Realtime` — lasciando la chiave intatta.
Expected: **PASS**, tutti e sei. È ciò che dimostra che la trappola è stata **tolta** e non
sorvegliata; con un `Ord` derivato quella stessa mutazione sarebbe passata **rovesciando le
priorità dell'arbitro** senza un rosso.

Revocare entrambe con lo strumento di edit, mai con `git checkout --`.

- [ ] **Passo 5: il cancello, il registro, il commit**

Run: `bash scripts/gate.sh` → `GATE GREEN`.

In [`docs/porta-di-qualita.md`](../../porta-di-qualita.md) si registrano le tre sonde nuove
e **entrambe** le mutazioni del passo 4, e si dichiara che `ComputeClass` **non ha una riga
di catalogo**: è un valore, non una forma.

```bash
git add crates/kernel/src/arbiter crates/kernel/tests/arbiter_resource.rs docs/porta-di-qualita.md
git commit -m "feat(arbiter): tre corsie con l'ordine scritto, e il tempo di grazia dentro la variante"
```

---

### Task 3: `ResourceProfile` e `WorkDescriptor`

**Files:**
- Modify: `crates/kernel/src/arbiter/resource.rs`
- Modify: `crates/kernel/src/arbiter/mod.rs`
- Modify: `crates/kernel/tests/arbiter_resource.rs`
- Create: `crates/kernel/tests/compile_fail/admission_reads_cold_start.rs`
- Modify: `docs/porta-di-qualita.md`

**Interfaces:**
- Consumes: `Mib`, `ComputeClass`, `Preemption`.
- Produces:
  ```rust
  pub struct ResourceProfile {
      pub name: &'static str,
      pub reserved_vram: Mib,
      pub compute_class: ComputeClass,
      pub preemption: Preemption,
  }
  pub struct WorkDescriptor {
      pub profile_name: &'static str,
      pub cold_start: Millis,
  }
  ```

⛔ **Le righe di catalogo che questo compito PREPARA:** `V2` — *«un'ammissione senza profilo
di risorsa»* — e `Q8 · §5.2.1` — *«l'ammissione legge `cold_start`»*. ⚠️ **Si CHIUDONO al
Task 5**, quando `admit` esiste: oggi il caso negativo non ha una funzione di ammissione da
nominare. Va detto invece di dichiararle chiuse qui.

- [ ] **Passo 1: le sonde delle DUE direzioni di `Q8`**

In coda a `crates/kernel/tests/arbiter_resource.rs`:

```rust
use kernel::arbiter::{ResourceProfile, WorkDescriptor};

/// ⛔ THE OTHER HALF OF `Q8 · §5.2.1`, and without it the row is proved in one direction
/// only, which §7.1.1 rule 3 does not admit. The rule is "the admission cannot reach
/// `cold_start`"; the counter-probe is "somebody OUTSIDE the admission can".
///
/// ⚠️ THE CATALOGUE NAMES THAT SOMEBODY "the presentation projection", AND IT DOES NOT
/// EXIST. So the reader here is a FAKE, and it proves the right property -- the field is
/// reachable outside the decision path -- with words different from the row's. Registered
/// in §12 of the milestone 5 design as the owner's to reword.
fn a_presentation_projection(descriptor: &WorkDescriptor) -> Millis {
    descriptor.cold_start
}

#[test]
fn cold_start_is_readable_outside_the_decision_path() {
    let descriptor = WorkDescriptor {
        profile_name: "trellis2-512-lean",
        cold_start: Millis::new(9_000),
    };
    assert_eq!(a_presentation_projection(&descriptor), Millis::new(9_000));
}

/// ⛔ THE TWO STRUCTURES ARE TIED BY A NAME AND BY NOTHING ELSE, and §5.2.1 priced that
/// exactly: "two structures instead of one, and one more place to keep them aligned". This
/// probe is that place, and it is a probe rather than a type because a shared type would
/// put `cold_start` back within reach of the admission.
#[test]
fn a_descriptor_names_the_profile_it_describes() {
    let profile = ResourceProfile {
        name: "trellis2-512-lean",
        reserved_vram: Mib::new(6_144),
        compute_class: ComputeClass::Batch,
        preemption: Preemption::After(Millis::new(500)),
    };
    let descriptor = WorkDescriptor {
        profile_name: "trellis2-512-lean",
        cold_start: Millis::new(9_000),
    };
    assert_eq!(profile.name, descriptor.profile_name);
}
```

- [ ] **Passo 2: eseguirla e vederla fallire**

Run: `cargo test --locked -p kernel --test arbiter_resource`
Expected: **FAIL**, `ResourceProfile` e `WorkDescriptor` non esistono.

- [ ] **Passo 3: scrivere le due strutture**

In coda a `crates/kernel/src/arbiter/resource.rs`:

```rust
/// What the arbiter RECEIVES in order to decide (§5.2). Named and versioned: design/02
/// makes the version part of the NAME -- `trellis2-512-lean`, `trellis2-1024` -- because a
/// kind of work does not produce a number but a CURVE, and the useful points of that curve
/// become distinct named profiles.
///
/// ⛔ `name` IS `&'static str` AND NOT `String`, and the reason is finding P-1, closed on
/// 2026-08-18: a profile name is chosen when the code is written, so it is a literal in the
/// binary, and runtime text -- which is where untrusted content lives -- cannot get here at
/// all. A `String` would buy nothing and reopen a road that was measured shut.
///
/// ⛔ AND `cold_start` IS NOT HERE. §5.2.1: design/02 says it is "used to warn the user, not
/// to decide", which was a written rule and therefore a recommendation. It lives in
/// `WorkDescriptor`, which the admission does not receive, so a decision that wanted to read
/// it HAS NO WAY.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceProfile {
    pub name: &'static str,
    /// The reservation DECLARED by the requester, not measured after the fact. A
    /// systematically wrong reservation is a defect of the PROFILE and not an incident
    /// (ADR-0005).
    pub reserved_vram: Mib,
    pub compute_class: ComputeClass,
    pub preemption: Preemption,
}

/// What goes to the PRESENTATION side, and never to the admission (§5.2.1).
///
/// ⚠️ IT IS TIED TO ITS PROFILE BY A NAME AND BY NOTHING ELSE. §5.2.1 accepted the cost in
/// those words -- "two structures instead of one, and one more place to keep them aligned"
/// -- and a shared type would put `cold_start` back within reach of the decision, which is
/// the whole thing this split exists to prevent.
///
/// ⚠️ NOT TO BE CONFUSED WITH `crate::ports::process::WorkerDescriptor`, which is one letter
/// away and is a different thing: that one is WHAT TO START, opaque bytes for the OS. No
/// file imports both unqualified.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkDescriptor {
    pub profile_name: &'static str,
    /// How long the work takes to become useful when it starts cold. ⛔ FOR WARNING THE
    /// USER, NEVER FOR DECIDING.
    pub cold_start: Millis,
}
```

E in `crates/kernel/src/arbiter/mod.rs`:

```rust
pub use resource::{ComputeClass, Mib, Preemption, ResourceProfile, WorkDescriptor};
```

- [ ] **Passo 4: verde, e la mutazione che NON viene colta — che è il punto**

Run: `cargo test --locked -p kernel --test arbiter_resource`
Expected: **PASS**, otto test.

Mutazione: spostare `cold_start` dentro `ResourceProfile`.
Run: `cargo test --workspace --no-fail-fast --locked`
Expected: **VERDE**. ⛔ **Ed è il risultato che serve, non un fallimento del compito:** dice
che le sonde a esempi **non** coprono questa regola e che serve il caso di livello 1 del
passo 5. Registrarlo, poi revocare la mutazione.

- [ ] **Passo 5: il caso negativo, scritto ora e ancora PARZIALE**

`crates/kernel/tests/compile_fail/admission_reads_cold_start.rs`:

```rust
// `Q8 · §5.2.1`: the profile the arbiter receives HAS NO `cold_start`, so a decision that
// wanted it has no way -- `E0609`.
//
// ⚠️ THIS CASE IS THE FIRST HALF. What the row forbids is not "a struct without a field",
// it is THE DECISION PATH reading it, and the decision path does not exist yet: `admit`
// arrives with task 5, and this case is rewritten there to name it. Until then the row is
// registered as PARTIALLY covered rather than closed -- a row proved in one direction only
// is not admissible (§7.1.1 rule 3).
fn main() {
    let profile = kernel::arbiter::ResourceProfile {
        name: "asr-realtime",
        reserved_vram: kernel::arbiter::Mib::new(1_024),
        compute_class: kernel::arbiter::ComputeClass::Realtime,
        preemption: kernel::arbiter::Preemption::Never,
    };
    let _warn_the_user = profile.cold_start;
}
```

Run: `cargo test --locked -p kernel --test compile_fail`
Expected: **FAIL** — manca l'oracolo. Generarlo con `TRYBUILD=overwrite` **una volta**, poi
**leggerlo**: deve dire **`E0609` no field `cold_start`**.

- [ ] **Passo 6: il cancello, il registro, il commit**

Run: `bash scripts/gate.sh` → `GATE GREEN`.

In [`docs/porta-di-qualita.md`](../../porta-di-qualita.md): `Q8 · §5.2.1` passa a
**parzialmente coperta** con l'**innesco scritto** — *«si chiude al compito che porta
`admit`»*, perché §8.1 pretende l'innesco per `parziale` — e la contro-sonda registrata
**come finta**, con la sua divergenza dalla cella del catalogo (**R5** di questo piano).

```bash
git add crates/kernel/src/arbiter crates/kernel/tests docs/porta-di-qualita.md
git commit -m "feat(arbiter): il profilo che l'arbitro riceve, e cold_start fuori dall'ammissione"
```

---

# Parte 2 — il ciclo della concessione

Quattro compiti. Il Task 4 porta i tipi, il Task 5 l'arbitro che li emette, il 6 le code,
il 7 la revoca. ⛔ **Da qui in poi ogni compito ha un consumatore vero**: il Task 4 è
l'ultimo che si può leggere senza l'arbitro.

---

### Task 4: `Grant` si sposta, `Admission` a tre vie, `Activity` annidata

**Files:**
- Modify: `crates/kernel/src/arbiter/mod.rs`
- Modify: `crates/kernel/src/ports/process.rs` (perde `Grant`, lo importa)
- Modify: `crates/kernel/tests/ports_are_implementable.rs` (cambia un import)
- Create: `crates/kernel/tests/compile_fail/admission_is_not_two_ways.rs`
- Create: `crates/kernel/tests/compile_fail/grant_has_no_constructor.rs`
- Create: `crates/kernel/tests/compile_fail/revoking_a_non_preemptible_grant.rs`
- Modify: `docs/porta-di-qualita.md`

**Interfaces:**
- Consumes: `Mib`, `ComputeClass`, `Preemption` dei Task 1–2; `kernel::time::Monotonic`.
- Produces:
  ```rust
  pub struct Grant { /* private */ }        // nessun costruttore pubblico
  pub struct TicketId(u64);                 // TicketId::get(self) -> u64
  #[must_use] pub enum Admission {
      Granted(Grant),
      Queued(TicketId),
      Refused { asked: Mib, ceiling: Mib },
  }
  pub enum Activity { NonPreemptible, Preemptible(PreemptibleState) }
  pub enum PreemptibleState { Running, Revoking { deadline: Monotonic } }
  ```

⛔ **Le righe di catalogo che questo compito tocca:** `V4` — *«l'esito trattato come due vie
invece di tre»* — si **chiude**; `I2 · §5.3` — *«`InRevoca` per un profilo non
prelazionabile»* — si **chiude**; la riga del blocco B *«avviare un worker ← una
concessione»* resta **aperta** fino al Task 5, perché finché nessuno emette concessioni la
sua contro-sonda — *«con → compila»* — non è scrivibile.

⛔ **E il pre-controllo di QUESTO compito, prima di dispacciarlo:** rileggere **R2** —
`Admission` non può derivare `Debug` né `PartialEq`, perché `Grant` non li ha e non deve
averli. Chi scrive le sonde usa `matches!` e `let … else`.

⛔ **E QUESTO È IL COMPITO CHE TOCCA I FILE CRLF, entrambi.** `ports/process.rs` porta **291
`CR`** nel blob committato e `ports_are_implementable.rs` ne porta **971**: sono due dei
**quattro** file del repository che ce li hanno, e su questi la normalizzazione **non** è
assorbita da `core.autocrlf`. Si misurano **prima**:

```bash
for f in crates/kernel/src/ports/process.rs crates/kernel/tests/ports_are_implementable.rs; do
  echo "$f $(tr -cd '\r' < "$f" | wc -c)"
done
```

e **dopo**, con lo stesso comando. ⛔ Se il conteggio cambia, lo strumento ha normalizzato in
silenzio e `git diff` dichiarerà righe che nessuno ha toccato: si ripristina da una **copia
byte-esatta** presa prima, mai con `git checkout --`, che cancellerebbe il lavoro non
committato di questo compito.

- [ ] **Passo 1: spostare `Grant`, e vedere il rosso che lo prova**

In `crates/kernel/src/arbiter/mod.rs`, aggiungere:

```rust
use crate::time::Monotonic;

/// A grant from the arbiter. THE ONLY WAY TO START A WORKER.
///
/// ⛔ IT LIVES HERE AND NOT IN `ports::process`, and the move was forced by a measured
/// fact rather than by tidiness. In Rust a private field is visible to the module that
/// declares it AND ITS CHILDREN; the arbiter module is a SIBLING of `ports::process`, so
/// with the type over there the arbiter COULD NOT CONSTRUCT THE THING IT EXISTS TO ISSUE
/// -- `error[E0423]`, measured on a throwaway crate (D5-1 of the milestone 5 design).
///
/// ⛔ AND THE CHEAP WAY OUT WAS REFUSED ON THE MERITS: a `pub(crate)` constructor left over
/// there costs one line and opens a road -- ANYONE INSIDE `kernel` could mint a grant
/// without passing the admission. Today that would be one module; tomorrow nobody knows,
/// and nothing would go red. A guard is worth exactly what its CONSTRUCTOR is worth
/// (gotcha #67).
///
/// ⛔ There is deliberately NO public constructor, and now there is also an ISSUER: the
/// admission. That is the whole of §5.6 -- whoever writes "start the worker" without one
/// DOES NOT COMPILE.
///
/// ⚠️ NO `Debug`, NO `Clone`, NO `Copy`, and each absence is load-bearing rather than
/// minimal. `Clone` would let one grant start two workers. `Debug` -- nothing formats a
/// grant, and the receipts of `ports::process` keep theirs only because `unwrap_err`
/// requires it. The consequence is on the BENCH and is written down so nobody discovers it:
/// `Admission` cannot derive `Debug` or `PartialEq` either, so probes match on it with
/// `matches!` and `let … else` instead of `assert_eq!`.
///
/// ⛔ DECLARED, FOR MILESTONE 6, so that milestone does not rediscover it: `Process::start`
/// CONSUMES the grant, and `Arbiter::release` consumes it too. Whoever starts a worker
/// therefore has nothing left to release. The natural way back is for `Worker::kill` to
/// HAND THE GRANT BACK -- killing IS the release -- and it is not built now because that
/// caller does not exist yet (gotcha #46 from the wrong side).
pub struct Grant {
    id: GrantId,
}

/// The identity of a grant, inside the arbiter that issued it.
///
/// ⚠️ NOT PUBLIC, and it does not need to be: the only thing outside this module ever does
/// with a grant is HAND IT BACK. The day something needs to name one, it comes back with
/// that caller -- the formula this repository already uses for `StepId::get` and
/// `CheckpointId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct GrantId(u64);

/// A place in a lane's queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(u64);

impl TicketId {
    /// The number back out. ⛔ LOAD-BEARING and not a convenience: a caller that queued two
    /// requests has nothing else to tell its two tickets apart, and `TicketId` is `Copy`
    /// precisely so it can be retained and compared -- the argument that kept
    /// `SingleReceipt::id` and removed `CheckpointId::get`, landing here on the keeping side.
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// What the admission answers. THREE WAYS, and the compiler makes the caller face all
/// three (§5.3 point 1, `V4`).
///
/// ⛔ THERE IS NO `is_ok()`, NO `is_granted()`, AND NO CONVERSION TO A BOOLEAN. That is how
/// `V4` becomes a SIGNATURE instead of a recommendation: "refused" and "queued" are
/// different answers that call for different behaviour, and a boolean would collapse them.
/// The negative case names a method that does not exist -- so the day somebody adds it the
/// case starts COMPILING and trybuild reports `error`, which no bulk regeneration disarms
/// (gotcha #42, strong form).
///
/// ⛔ `Refused` CARRIES TWO NUMBERS AND NOT A SENTENCE. design/02 wants "why it does not fit,
/// and the workable alternative": the alternative is built by the interface, the kernel
/// hands it the material. Suggesting another profile would be L2 logic inside the kernel
/// (ADR-0020).
///
/// ⚠️ NO `Debug` AND NO `PartialEq`, because `Granted` carries a `Grant` and that type has
/// neither, deliberately. Deriving them here would mean giving `Grant` a `Debug` FOR THE
/// CONVENIENCE OF THE BENCH -- exactly what `ports::process` refused. Probes match instead.
#[must_use]
pub enum Admission {
    /// It fits. The grant is the only way to start a worker.
    Granted(Grant),
    /// It does not fit now, and the request is waiting IN ITS OWN LANE (§5.3.1).
    Queued(TicketId),
    /// It does not fit and it never will under this budget.
    Refused { asked: Mib, ceiling: Mib },
}

/// What a held grant is doing. ⛔ IT NESTS RATHER THAN FLATTENS, and the nesting IS the
/// rule: §5.3 point 3 wants `Revoking` to be NOT REPRESENTABLE for a non-preemptible
/// profile. `NonPreemptible` HAS NOWHERE TO PUT ONE -- it is a unit variant -- so the
/// illegal state is not forbidden at runtime, it cannot be spelled.
///
/// ⚠️ `NonPreemptible` AND NOT `Permanent`, and the difference is real: a job that cannot
/// be interrupted still FINISHES and releases. Permanence is not a type -- it is "nobody
/// calls release", which is exactly how the two permanent grants of the composition root
/// stay held (§4.3 of the design).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Activity {
    NonPreemptible,
    Preemptible(PreemptibleState),
}

/// The two states only a preemptible grant can be in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreemptibleState {
    Running,
    /// The arbiter has asked for the resource back. `deadline` is on the MONOTONIC axis --
    /// never wall time: a clock that steps backwards cannot expire a grant (§5.3 point 2).
    Revoking { deadline: Monotonic },
}
```

E in `crates/kernel/src/ports/process.rs`: **togliere** la dichiarazione di `Grant` con tutto
il proprio commento, e mettere in cima al file, accanto agli altri `use`:

```rust
use crate::arbiter::Grant;
```

⛔ **Il commento di `Grant` non si copia e non si perde: si SPOSTA, riscritto col proprio
richiamo datato.** Diceva *«§5.6, che arriva al Traguardo 5»* al **futuro**, e lasciarlo così
sarebbe il finding **A-2** rifatto — una formulazione falsificata che sopravvive perché
nessuno rilegge il documento in cui vive.

⛔ **E `Grant` NON si ri-esporta da `ports::process`** (decisione **D8**): due percorsi allo
stesso tipo lo farebbero leggere come proprietà della porta, mentre lo si sposta proprio
perché appartiene a chi lo **emette**.

- [ ] **Passo 2: compilare, e leggere il rosso che si aspetta**

Run: `cargo build --locked --workspace`
Expected: **FAIL** in `crates/kernel/tests/ports_are_implementable.rs:52`, che importa
`Grant` da `kernel::ports::process`.

⛔ **È il rosso che PROVA la decisione D8:** senza ri-export, ogni sito che nominava `Grant`
attraverso la porta si presenta da solo. Con un ri-export nessuno si sarebbe presentato, e i
due percorsi sarebbero nati invisibili.

Correggere l'import di quel banco:

```rust
use kernel::arbiter::Grant;
use kernel::ports::process::{
    Frame, Process, ProcessError, SingleReceipt, StreamReceipt, Worker, WorkerDescriptor,
};
```

Run: `cargo test --locked --workspace`
Expected: **PASS**, e il conteggio identico alla baseline — **32 target, 194 passati**. ⛔ Uno
spostamento di modulo che cambia un conteggio di test ha fatto **altro** oltre a spostare.

- [ ] **Passo 3: i tre casi negativi**

`crates/kernel/tests/compile_fail/admission_is_not_two_ways.rs`:

```rust
// `V4`: the admission answers THREE ways. §5.3 point 1 -- "refused" and "queued" are
// DISTINCT outcomes, so whoever calls is obliged to tell them apart, and treating the
// answer as a yes/no does not compile: `E0004`, non-exhaustive patterns.
//
// ⚠️ IT NEEDS NO ARBITER, DELIBERATELY. This case is about the SHAPE of the answer, and the
// shape exists from the moment the enum does. The second half of the row -- that there is no
// `is_granted()` shortcut either -- needs a real admission and arrives with it.
fn two_ways(outcome: kernel::arbiter::Admission) -> bool {
    match outcome {
        kernel::arbiter::Admission::Granted(_) => true,
        kernel::arbiter::Admission::Refused { .. } => false,
    }
}

fn main() {
    let _ = two_ways;
}
```

`crates/kernel/tests/compile_fail/grant_has_no_constructor.rs`:

```rust
// The other half of the block B row "starting a worker <- a grant": the token cannot be
// FORGED. `Grant`'s only field is private and its module is `kernel::arbiter`, so from out
// here there is no way to build one -- `error[E0422]`/`E0423`.
//
// ⚠️ THE DECLARED LIMIT, so this case promises no more than it holds: trybuild compiles its
// cases as SEPARATE CRATES, so what is proved is the direction FROM OUTSIDE. Nothing here
// stops somebody adding a `pub(crate)` constructor tomorrow -- that would be a new
// catalogue row, and the catalogue is spec. Registered in §12 of the design.
fn main() {
    let _forged = kernel::arbiter::Grant {};
}
```

`crates/kernel/tests/compile_fail/revoking_a_non_preemptible_grant.rs`:

```rust
// `I2 · §5.3`: a non-preemptible grant has NOWHERE to put a revocation. §5.3 point 3 wants
// this "not constructible", not "checked at runtime" -- so the state is not forbidden, it
// cannot be spelled.
fn main() {
    let _impossible = kernel::arbiter::Activity::NonPreemptible(
        kernel::arbiter::PreemptibleState::Revoking {
            deadline: kernel::time::Monotonic::from_millis(1_000),
        },
    );
}
```

- [ ] **Passo 4: generare i tre oracoli e LEGGERLI**

Run: `cargo test --locked -p kernel --test compile_fail`
Expected: **FAIL**, tre oracoli mancanti.

`TRYBUILD=overwrite cargo test --locked -p kernel --test compile_fail`, **una volta**.

Run: `git diff --stat crates/kernel/tests/compile_fail/`
Expected: **tre** `.stderr` nuovi, nessun altro toccato.

⛔ **E i tre si leggono, uno per uno.** Attesi: **`E0004`** per il primo, motivi non esaustivi
con `Queued` non coperto; un errore di **privacy o di costruzione** per il secondo; e per il
terzo un errore che dice che **`NonPreemptible` non porta campi**.

⚠️ **Il codice esatto del terzo NON è predetto qui, ed è deliberato:** una sigla scritta
prima della misura è un'ipotesi travestita da attesa — gotcha **#15**. Si legge dall'uscita
vera e si riporta nel registro **quella**.

- [ ] **Passo 5: la seconda direzione dei tre casi**

⛔ **Tre mutazioni, una alla volta, ciascuna compilata ed eseguita a sé e poi revocata** — è
la forma della campagna con cui si è chiusa la decisione 1 dell'audit.

| # | Mutazione | Atteso |
|---|---|---|
| 1 | aggiungere al `match` del caso il ramo `Admission::Queued(_) => false` | `admission_is_not_two_ways.rs` → **`error`**, cioè ha compilato: è il caso che dimostra che il rosso viene dalla **terza via** e non da un errore di sintassi |
| 2 | rendere il campo di `Grant` `pub` | `grant_has_no_constructor.rs` → **`error`** |
| 3 | appiattire `Activity` in `NonPreemptible(PreemptibleState)` | `revoking_a_non_preemptible_grant.rs` → **`error`** |

⛔ **E per ciascuna si prova che sia ENTRATA prima di credere all'esito**, con un `grep -c`
sul sito mutato: un verde da una mutazione non applicata è la vacuità che si sta cacciando
(gotcha #48).

Revocare tutte e tre con lo strumento di edit.

- [ ] **Passo 6: il cancello, il registro, il commit**

Run: `bash scripts/gate.sh` → `GATE GREEN`.

Nel registro: `V4` e `I2 · §5.3` passano a **coperte**, coi casi per nome e le mutazioni del
passo 5. La riga B *«avviare un worker ← una concessione»* resta **parziale**, con l'innesco
scritto: *«si chiude al compito che porta `admit`, che è l'unico modo di ottenere la
contro-sonda»*.

```bash
git add crates/kernel/src crates/kernel/tests docs/porta-di-qualita.md
git commit -m "feat(arbiter): Grant si sposta da chi lo consuma a chi lo emette, e l'esito e a tre vie"
```

---

### Task 5: `total_vram` consegnato, e l'arbitro che ammette e rilascia

**Files:**
- Modify: `crates/kernel/src/parameters.rs`
- Modify: `crates/kernel/src/arbiter/mod.rs`
- Create: `crates/kernel/tests/arbiter_admission.rs`
- Modify: `crates/kernel/tests/compile_fail/admission_reads_cold_start.rs`
- Create: `crates/kernel/tests/compile_fail/admission_has_no_is_granted.rs`
- Modify: **tutti** i chiamanti di `Parameters::new` — venti siti in sei file, contati col
  `grep` il 2026-08-18: `crates/daemon/src/main.rs`,
  `crates/kernel/tests/executor_determinism.rs`, `crates/kernel/tests/parameters_delivered.rs`,
  `crates/simulator/tests/dst_campaign.rs`, e i due casi `compile_fail`
  `parameters_have_no_default.rs` e `trust_has_no_default.rs`
- Modify: `docs/porta-di-qualita.md`

**Interfaces:**
- Consumes: tutto il Parte 1, più `Grant`, `Admission`, `Activity` del Task 4.
- Produces:
  ```rust
  impl Parameters {
      pub const fn new(executor_turn_limit: u64, total_vram: Mib) -> Self;
      pub const fn total_vram(self) -> Mib;
  }
  pub enum ReleaseError { UnknownGrant }
  impl Arbiter {
      pub fn new(parameters: Parameters) -> Self;
      pub fn admit(&mut self, profile: &ResourceProfile, valid_for: Millis, now: Monotonic) -> Admission;
      pub fn release(&mut self, grant: Grant, now: Monotonic) -> Result<Mib, ReleaseError>;
      pub fn allocated(&self) -> Mib;
  }
  ```

⛔ **Il conteggio dei chiamanti si RICONTA prima di cominciare** — `grep -rn "Parameters::new" crates/ --include=*.rs | wc -l` — e non si prende da questa riga: il contratto cresce sotto il piano, quinta domanda del pre-controllo.

⛔ **La riga di catalogo che questo compito chiude:** il blocco B, *«avviare un worker ← una
concessione»*, che finalmente ha **entrambe** le direzioni. E `Q8 · §5.2.1` passa da parziale
a **chiusa**, perché il caso negativo può ora nominare `admit`.

- [ ] **Passo 1: le sonde dell'ammissione, che oggi non compilano**

`crates/kernel/tests/arbiter_admission.rs`:

```rust
//! What the compiler cannot hold about the admission: that the sum of every grant never
//! exceeds the total, that releasing gives back EXACTLY the reservation, and that an
//! expired grant does not stay allocated.
//!
//! ⚠️ THE PROBES MATCH RATHER THAN COMPARE, and it is not a style: `Admission` has no
//! `Debug` and no `PartialEq`, because `Grant` deliberately has neither. Giving them to it
//! for the convenience of this file is the trade `ports::process` refused.

use kernel::arbiter::{Admission, Arbiter, ComputeClass, Mib, Preemption, ResourceProfile};
use kernel::parameters::Parameters;
use kernel::time::{Millis, Monotonic};

const TURN_LIMIT: u64 = 10_000;
const TOTAL: Mib = Mib::new(16_384);

fn profile(name: &'static str, vram: u64, lane: ComputeClass) -> ResourceProfile {
    ResourceProfile {
        name,
        reserved_vram: Mib::new(vram),
        compute_class: lane,
        preemption: Preemption::Never,
    }
}

fn arbiter(total: Mib) -> Arbiter {
    Arbiter::new(Parameters::new(TURN_LIMIT, total))
}

/// The window every probe in this file uses when the value does not matter.
const LONG: Millis = Millis::new(1_000_000);

/// ⛔ THE ASSERTION IS THE NUMBER, NOT THE VARIANT. "It granted" is satisfied by an arbiter
/// that grants everything; what says the budget is real is that `allocated` MOVED BY THE
/// RESERVATION.
#[test]
fn a_grant_takes_exactly_its_reservation_out_of_the_budget() {
    let mut arbiter = arbiter(TOTAL);
    assert_eq!(arbiter.allocated(), Mib::ZERO);

    let outcome = arbiter.admit(
        &profile("asr-realtime", 1_024, ComputeClass::Realtime),
        LONG,
        Monotonic::ORIGIN,
    );
    assert!(matches!(outcome, Admission::Granted(_)));
    assert_eq!(arbiter.allocated(), Mib::new(1_024));
}

/// ⛔ THE OTHER HALF OF THE PROPERTY, and it is the arbiter half of §5.7 properties 2 and 3
/// -- the only half milestone 5 can hold, and it is ONE and not two: the arbiter does not
/// need to know WHO held a grant, only that releasing puts the reservation back.
#[test]
fn releasing_gives_back_exactly_the_reservation() {
    let mut arbiter = arbiter(TOTAL);
    let Admission::Granted(grant) = arbiter.admit(
        &profile("trellis2-512-lean", 6_144, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("6144 of 16384 fits");
    };
    assert_eq!(arbiter.allocated(), Mib::new(6_144));

    let returned = arbiter
        .release(grant, Monotonic::ORIGIN)
        .expect("the arbiter issued this grant");

    assert_eq!(returned, Mib::new(6_144));
    assert_eq!(arbiter.allocated(), Mib::ZERO);
}

/// ⛔ THE `Err` OF `release` IS REACHABLE, which is what keeps it from being the dead
/// surface this repository removed from `Record::encode` and refused to `Ipc::accept`. Two
/// arbiters, a grant from the first handed to the second.
#[test]
fn a_grant_released_on_the_wrong_arbiter_is_an_error_and_not_a_silent_credit() {
    let mut first = arbiter(TOTAL);
    let mut second = arbiter(TOTAL);

    let Admission::Granted(grant) = first.admit(
        &profile("asr-realtime", 1_024, ComputeClass::Realtime),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("1024 of 16384 fits");
    };

    assert!(second.release(grant, Monotonic::ORIGIN).is_err());
    assert_eq!(second.allocated(), Mib::ZERO, "no silent credit");
}

/// ⛔ THE INVARIANT, ASSERTED ON THE NUMBER: the sum of ALL grants never exceeds the total.
/// The third request does not fit and comes back `Refused` with the two numbers design/02
/// asks for.
#[test]
fn the_sum_of_the_grants_never_exceeds_the_total() {
    let mut arbiter = arbiter(Mib::new(8_192));
    for name in ["a", "b"] {
        let outcome = arbiter.admit(
            &profile(name, 4_096, ComputeClass::Batch),
            LONG,
            Monotonic::ORIGIN,
        );
        assert!(matches!(outcome, Admission::Granted(_)));
    }
    assert_eq!(arbiter.allocated(), Mib::new(8_192));

    let Admission::Refused { asked, ceiling } = arbiter.admit(
        &profile("c", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("the budget is full");
    };
    assert_eq!(asked, Mib::new(4_096));
    assert_eq!(ceiling, Mib::new(8_192));
    assert_eq!(arbiter.allocated(), Mib::new(8_192), "nothing was over-admitted");
}

/// ⛔ AN IMPOSSIBLE CONFIGURATION IS VISIBLE INSTEAD OF SILENT, and this is the probe that
/// pays for the design's divergence from §5.1. With the two quotas SUBTRACTED from the
/// total, a total smaller than their sum would give a budget of zero WITHOUT A WORD. As two
/// permanent grants, the second one comes back `Refused` and names both numbers.
#[test]
fn a_total_smaller_than_the_two_permanent_quotas_refuses_the_second_one() {
    let mut arbiter = arbiter(Mib::new(1_500));

    let audio = arbiter.admit(
        &profile("audio-reserved", 1_024, ComputeClass::Realtime),
        LONG,
        Monotonic::ORIGIN,
    );
    assert!(matches!(audio, Admission::Granted(_)));

    let Admission::Refused { asked, ceiling } = arbiter.admit(
        &profile("presentation-reserved", 1_024, ComputeClass::Realtime),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("1024 + 1024 does not fit in 1500");
    };
    assert_eq!(asked, Mib::new(1_024));
    assert_eq!(ceiling, Mib::new(1_500));
}

/// ⛔ THE LAZY COLLECTION, AND THE PROPERTY IS WRITTEN SO IT IS OBSERVABLE. Between two
/// operations an expired grant stays in the books -- it denies nothing to nobody, there IS
/// nobody -- and at the first one who looks it is already freed. §5.7 property 5.
#[test]
fn an_expired_grant_does_not_stay_allocated() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let outcome = arbiter.admit(
        &profile("short-lived", 4_096, ComputeClass::Batch),
        Millis::new(5_000),
        Monotonic::ORIGIN,
    );
    assert!(matches!(outcome, Admission::Granted(_)));

    // The same request, after the window: the first one is collected and this one fits.
    let after = arbiter.admit(
        &profile("the-next-one", 4_096, ComputeClass::Batch),
        Millis::new(5_000),
        Monotonic::from_millis(5_001),
    );
    assert!(
        matches!(after, Admission::Granted(_)),
        "without the collection this is Refused"
    );
    assert_eq!(arbiter.allocated(), Mib::new(4_096), "one grant, not two");
}

/// The counter-probe of the one above, and it is the direction that is skipped: a grant
/// that has NOT expired is not collected. Without this, "collect everything always" passes.
#[test]
fn a_grant_still_inside_its_window_is_not_collected() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let outcome = arbiter.admit(
        &profile("still-running", 4_096, ComputeClass::Batch),
        Millis::new(5_000),
        Monotonic::ORIGIN,
    );
    assert!(matches!(outcome, Admission::Granted(_)));

    let after = arbiter.admit(
        &profile("the-next-one", 4_096, ComputeClass::Batch),
        Millis::new(5_000),
        Monotonic::from_millis(4_999),
    );
    assert!(
        matches!(after, Admission::Refused { .. }),
        "the window has not closed yet"
    );
}

/// ⛔ A REQUEST BIGGER THAN THE WHOLE MACHINE IS `Refused` AND NEVER `Queued`: no release
/// will ever make room for it, and a ticket that can never be served is a leak that looks
/// like patience.
#[test]
fn a_request_larger_than_the_total_is_refused_and_not_queued() {
    let mut arbiter = arbiter(Mib::new(8_192));
    let Admission::Refused { asked, ceiling } = arbiter.admit(
        &profile("too-big", 32_768, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("32768 never fits in 8192");
    };
    assert_eq!(asked, Mib::new(32_768));
    assert_eq!(ceiling, Mib::new(8_192));
}
```

⚠️ **`TicketId` NON è importato in questo file al Task 5**, e non deve esserlo: `Queued` non
ha ancora un produttore, quindi un import ci sarebbe solo per far compilare una riga scritta
in anticipo. Entra al Task 6, col proprio consumatore.

- [ ] **Passo 2: eseguirle e vederle fallire**

Run: `cargo test --locked -p kernel --test arbiter_admission`
Expected: **FAIL** — `Arbiter` non esiste e `Parameters::new` prende un argomento solo.

- [ ] **Passo 3: `Parameters` guadagna il totale**

In `crates/kernel/src/parameters.rs`:

```rust
use crate::arbiter::Mib;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Parameters {
    executor_turn_limit: u64,
    total_vram: Mib,
}

impl Parameters {
    pub const fn new(executor_turn_limit: u64, total_vram: Mib) -> Self {
        Parameters {
            executor_turn_limit,
            total_vram,
        }
    }

    pub const fn executor_turn_limit(self) -> u64 {
        self.executor_turn_limit
    }

    /// How much VRAM the machine has, in whole MiB.
    ///
    /// ⛔ IT IS DELIVERED AND NOT ASKED FOR, and §5.1 spent a dated recall on exactly this:
    /// the formula for the allocatable budget appears identically in three documents and
    /// NONE of them said where `total` comes from. Querying the GPU is an OS call, which I3
    /// forbids the kernel, and none of the six port families supplies hardware capacity. So
    /// it is DECLARED, like the reservation of ADR-0005, and a systematic discrepancy is a
    /// defect of the PARAMETER rather than an accident.
    ///
    /// ⚠️ THE COST, DECLARED BY §5.1 ITSELF: a wrong total produces over-admission -- Q2
    /// giving way through a configuration error rather than a code one. The mitigation is
    /// the measured peak of §5.2.2, not an a-priori check that does not exist here.
    ///
    /// ⛔ IT IS THE ONLY ONE OF THE THREE ADDENDS THAT IS DELIVERED, and that is a declared
    /// divergence from the letter of §5.1 rather than an omission. The audio quota and the
    /// presentation quota are NOT subtracted here: they are the reservations of two
    /// PERMANENT GRANTS asked for by the composition root. A subtraction without a holder
    /// leaves I2 false for those two consumers -- "the subtraction is not an exemption",
    /// ADR-0005 and gotcha #4 -- and two fields no kernel decision reads would be dead
    /// surface inside the kernel.
    pub const fn total_vram(self) -> Mib {
        self.total_vram
    }
}
```

⛔ **E i venti siti chiamanti si aggiornano tutti**, con un totale **scritto sul posto** e
non un default: un default in `Parameters` è precisamente ciò che §2.8.2 regola 2 vieta.
Per i banchi il valore è un letterale del banco; per `daemon` è un letterale di `daemon`
(vincolo 11 di §11) — e il cablaggio vero del `daemon` è il Task 10.

- [ ] **Passo 4: l'arbitro**

In coda a `crates/kernel/src/arbiter/mod.rs`:

```rust
use alloc::collections::BTreeMap;

use crate::parameters::Parameters;
use crate::time::Millis;

/// What can go wrong when handing a grant back.
///
/// ⛔ ONE VARIANT, AND IT IS REACHABLE -- which is what keeps this `Result` from being the
/// dead surface this repository removed from `Record::encode`. Two arbiters can exist in
/// one process (a bench builds several), and a grant issued by one is meaningless to the
/// other. Crediting it silently would corrupt the budget of an arbiter that never issued
/// it, which is over-admission arriving by the back door.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseError {
    /// This arbiter never issued that grant.
    UnknownGrant,
}

/// What the arbiter remembers about a grant it has issued.
struct Held {
    reserved: Mib,
    lane: ComputeClass,
    activity: Activity,
    /// The validity window, on the MONOTONIC axis (§5.3 point 2).
    expires_at: Monotonic,
}

/// The GPU arbiter: admission on VRAM, lanes on compute (ADR-0005).
///
/// ⛔ `BTreeMap` AND `Vec`, AND IT IS NOT A PREFERENCE: `HashMap` lives in `std`, which this
/// crate does not name, so gotcha #12 -- iteration order seeded PER PROCESS, which V29
/// forbids -- is closed here by the compiler and for free (§5.1). It also closes M-6.
pub struct Arbiter {
    parameters: Parameters,
    next_grant: u64,
    held: BTreeMap<GrantId, Held>,
}

impl Arbiter {
    /// ⛔ IT TAKES `Parameters` AND NOT A BARE `Mib`. That is the shape `Executor::new`
    /// already has and the one the catalogue row `V29 · §2.8 · ADR-0034` names -- "building
    /// a decision without the delivered parameters". A bare number would have the
    /// composition root read the total and hand it over OUTSIDE the mechanism ADR-0034
    /// exists to impose.
    pub const fn new(parameters: Parameters) -> Self {
        Arbiter {
            parameters,
            next_grant: 0,
            held: BTreeMap::new(),
        }
    }

    /// How much VRAM is spoken for right now. ⛔ IT COLLECTS NOTHING: it reports the books
    /// as they are, so a probe can tell "the collection happened" from "the number looks
    /// right anyway".
    pub fn allocated(&self) -> Mib {
        self.held
            .values()
            .fold(Mib::ZERO, |sum, held| sum.saturating_add(held.reserved))
    }

    /// Admission (§5.3). THREE ways out, and the caller must face all three.
    ///
    /// ⛔ IT COLLECTS THE EXPIRED FIRST, and the declared limit of that is written where the
    /// property is: between two operations an expired grant stays in the books. It denies
    /// nothing to nobody -- there is nobody -- and at the first one who looks it is already
    /// freed. The property holds AT EVERY POINT WHERE IT IS OBSERVABLE, which is why the
    /// probe advances the clock and then ASKS.
    ///
    /// ⛔ AND IT DOES NOT RECEIVE A `WorkDescriptor`. `cold_start` is not reachable from
    /// here, and that is `Q8 · §5.2.1` held by the shape of this signature rather than by a
    /// rule in a document.
    pub fn admit(
        &mut self,
        profile: &ResourceProfile,
        valid_for: Millis,
        now: Monotonic,
    ) -> Admission {
        self.collect_expired(now);

        let ceiling = self.parameters.total_vram();
        let asked = profile.reserved_vram;

        if asked > ceiling {
            // ⛔ Bigger than the whole machine: no release will ever make room, so a ticket
            // here would be a leak that looks like patience.
            return Admission::Refused { asked, ceiling };
        }

        if self.allocated().saturating_add(asked) > ceiling {
            return Admission::Refused { asked, ceiling };
        }

        let id = GrantId(self.next_grant);
        self.next_grant += 1;
        self.held.insert(
            id,
            Held {
                reserved: asked,
                lane: profile.compute_class,
                activity: match profile.preemption {
                    Preemption::Never => Activity::NonPreemptible,
                    Preemption::After(_) => Activity::Preemptible(PreemptibleState::Running),
                },
                expires_at: now.saturating_add(valid_for),
            },
        );
        Admission::Granted(Grant { id })
    }

    /// Hands a grant back, and answers with the reservation that returned to the budget.
    ///
    /// ⛔ IT CONSUMES THE GRANT: releasing twice DOES NOT COMPILE, which is level 1 and
    /// cheaper than any runtime guard. The consequence for milestone 6 is written beside
    /// `Grant`.
    pub fn release(&mut self, grant: Grant, now: Monotonic) -> Result<Mib, ReleaseError> {
        self.collect_expired(now);
        match self.held.remove(&grant.id) {
            Some(held) => Ok(held.reserved),
            None => Err(ReleaseError::UnknownGrant),
        }
    }

    /// ⛔ PRIVATE, AND DELIBERATELY. A public `collect` would be a SECOND way of advancing
    /// this state -- one no probe covers and no caller has to reach -- while "the arbiter
    /// collects before it decides" is a property of every operation rather than a step
    /// somebody remembers to take.
    fn collect_expired(&mut self, now: Monotonic) {
        self.held.retain(|_, held| held.expires_at > now);
    }
}
```

⚠️ **`Held::lane` non ha lettori in questo compito**, e il Task 6 glieli dà. ⛔ Se il
compilatore lo segnala, **non si mette un `#[allow]`** (vincolo globale 4): si sposta il
campo al Task 6, dove nasce col proprio consumatore.

- [ ] **Passo 5: verde, e le mutazioni**

Run: `cargo test --locked -p kernel --test arbiter_admission`
Expected: **PASS**, otto test.

⛔ **Quattro mutazioni, una alla volta, ciascuna compilata ed eseguita a sé:**

| # | Mutazione | Sonda che deve morire |
|---|---|---|
| 1 | `>` diventa `>=` nel confronto col tetto | `the_sum_of_the_grants_never_exceeds_the_total` |
| 2 | `release` restituisce `Mib::ZERO` invece di `held.reserved` | `releasing_gives_back_exactly_the_reservation` |
| 3 | `collect_expired` diventa un corpo vuoto | `an_expired_grant_does_not_stay_allocated` |
| 4 | `collect_expired` usa `retain(\|_, _\| false)` | `a_grant_still_inside_its_window_is_not_collected` |

⛔ **Le mutazioni 3 e 4 vanno provate ENTRAMBE**, ed è la ragione per cui esistono due
sonde: la 3 sola sarebbe soddisfatta da «riscuoti sempre tutto», che è il difetto opposto e
non meno grave.

- [ ] **Passo 6: `admission_reads_cold_start.rs` nomina finalmente `admit`**

Riscrivere il corpo del caso:

```rust
// `Q8 · §5.2.1`: THE DECISION PATH cannot reach `cold_start`. `admit` receives a
// `ResourceProfile`, which has no such field -- `E0609`.
fn main() {
    let mut arbiter = kernel::arbiter::Arbiter::new(kernel::parameters::Parameters::new(
        10_000,
        kernel::arbiter::Mib::new(16_384),
    ));
    let profile = kernel::arbiter::ResourceProfile {
        name: "asr-realtime",
        reserved_vram: kernel::arbiter::Mib::new(1_024),
        compute_class: kernel::arbiter::ComputeClass::Realtime,
        preemption: kernel::arbiter::Preemption::Never,
    };
    let _decide_on_it = profile.cold_start;
    let _ = arbiter.admit(
        &profile,
        kernel::time::Millis::new(1_000),
        kernel::time::Monotonic::ORIGIN,
    );
}
```

Rigenerare **quell'oracolo solo** e leggerlo: `E0609`.

- [ ] **Passo 7: la seconda metà di `V4`, che aspettava un'ammissione vera**

`crates/kernel/tests/compile_fail/admission_has_no_is_granted.rs`:

```rust
// `V4`, second half: there is no boolean shortcut on the answer either. `Admission` has no
// `is_granted()`, no `is_ok()`, and no conversion to `bool`.
//
// ⛔ IT NAMES A METHOD THAT DOES NOT EXIST, ON PURPOSE. Today that is `E0599`. The day
// somebody adds it this case starts COMPILING and trybuild reports it as `error` rather than
// through its oracle -- gotcha #42, the shape a bulk regeneration cannot disarm. The first
// half, `admission_is_not_two_ways.rs`, fires as `E0004` and DOES rest on its oracle: the
// two halves are complementary and neither is redundant.
fn main() {
    let mut arbiter = kernel::arbiter::Arbiter::new(kernel::parameters::Parameters::new(
        10_000,
        kernel::arbiter::Mib::new(16_384),
    ));
    let outcome = arbiter.admit(
        &kernel::arbiter::ResourceProfile {
            name: "asr-realtime",
            reserved_vram: kernel::arbiter::Mib::new(1_024),
            compute_class: kernel::arbiter::ComputeClass::Realtime,
            preemption: kernel::arbiter::Preemption::Never,
        },
        kernel::time::Millis::new(1_000),
        kernel::time::Monotonic::ORIGIN,
    );
    if outcome.is_granted() {
        // nothing: the point is that this line must not compile
    }
}
```

Generare l'oracolo e leggerlo: **`E0599`**.

Poi la mutazione, e va **provata applicata**: aggiungere
`impl Admission { pub const fn is_granted(&self) -> bool { matches!(self, Admission::Granted(_)) } }`
→ il caso deve essere riportato come **`error`**. Revocarla.

- [ ] **Passo 8: il cancello, il registro, il commit**

Run: `bash scripts/gate.sh` → `GATE GREEN`.
Run: `cargo test --workspace --no-fail-fast --locked`
Expected: il conteggio **ricontato**, non dedotto da 194 più otto — gotcha **#31**.

Nel registro: la riga B *«avviare un worker ← una concessione»* passa a **coperta**, con la
contro-sonda che finalmente esiste; `Q8 · §5.2.1` passa a **coperta**; le otto sonde nuove e
le quattro mutazioni entrano nella tabella.

```bash
git add crates/kernel crates/daemon crates/simulator docs/porta-di-qualita.md
git commit -m "feat(arbiter): il totale consegnato, l'ammissione e il rilascio"
```

---

### Task 6: le code, e sono per corsia

**Files:**
- Modify: `crates/kernel/src/arbiter/mod.rs`
- Modify: `crates/kernel/tests/arbiter_admission.rs`
- Modify: `docs/porta-di-qualita.md`

**Interfaces:**
- Consumes: tutto il Task 5.
- Produces: `Admission::Queued(TicketId)` acquisisce un **produttore**, e
  ```rust
  pub struct Promotion { pub ticket: TicketId, pub grant: Grant }
  impl Arbiter {
      pub fn promote(&mut self, now: Monotonic) -> Vec<Promotion>;
      pub fn queued(&self) -> usize;
  }
  ```

⛔ **Perché per corsia e non FIFO globale, ed è una misura e non un gusto:** §5.3.1 dice che
i numeri di **M-7** restano validi **come limite superiore** proprio perché la versione
specificata tiene l'ordine **per corsia**. Una coda unica riordinata a ogni rilascio
invaliderebbe quella misura, e allora andrebbe rifatta.

- [ ] **Passo 1: le sonde della coda**

In coda a `crates/kernel/tests/arbiter_admission.rs`:

```rust
use kernel::arbiter::Promotion;

/// A request that does not fit NOW but could fit later is queued, not refused.
#[test]
fn a_request_that_fits_the_machine_but_not_the_moment_is_queued() {
    let mut arbiter = arbiter(Mib::new(8_192));
    let Admission::Granted(resident) = arbiter.admit(
        &profile("resident", 8_192, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine exactly");
    };

    let Admission::Queued(ticket) = arbiter.admit(
        &profile("waiting", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("4096 fits the machine, just not right now");
    };
    assert_eq!(arbiter.queued(), 1);

    let _ = arbiter
        .release(resident, Monotonic::ORIGIN)
        .expect("this arbiter issued it");
    let promoted = arbiter.promote(Monotonic::ORIGIN);

    assert_eq!(promoted.len(), 1);
    assert_eq!(promoted[0].ticket, ticket);
    assert_eq!(arbiter.allocated(), Mib::new(4_096));
    assert_eq!(arbiter.queued(), 0);
}

/// ⛔ THE ASSERTION THAT KEEPS M-7's NUMBERS VALID, and it is the whole reason the queue is
/// per lane. `Batch` arrived FIRST and `Interactive` is served first anyway; a global FIFO
/// would promote them in arrival order and this probe would go red.
#[test]
fn the_queue_promotes_by_lane_and_not_in_arrival_order() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let Admission::Granted(resident) = arbiter.admit(
        &profile("resident", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine exactly");
    };

    let Admission::Queued(batch) = arbiter.admit(
        &profile("batch-first", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("queued");
    };
    let Admission::Queued(interactive) = arbiter.admit(
        &profile("interactive-second", 4_096, ComputeClass::Interactive),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("queued");
    };
    assert_eq!(arbiter.queued(), 2);

    let _ = arbiter
        .release(resident, Monotonic::ORIGIN)
        .expect("this arbiter issued it");
    let promoted = arbiter.promote(Monotonic::ORIGIN);

    assert_eq!(promoted.len(), 1, "only one of the two fits");
    assert_eq!(
        promoted[0].ticket, interactive,
        "the lane decides, not the arrival order"
    );
    assert_ne!(promoted[0].ticket, batch);
}

/// Within ONE lane the order is arrival order, and this is what says the lane rule above is
/// not "any order at all".
#[test]
fn inside_one_lane_the_order_is_the_order_of_arrival() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let Admission::Granted(resident) = arbiter.admit(
        &profile("resident", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("full");
    };

    let Admission::Queued(first) = arbiter.admit(
        &profile("first", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("queued");
    };
    let Admission::Queued(second) = arbiter.admit(
        &profile("second", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("queued");
    };

    let _ = arbiter
        .release(resident, Monotonic::ORIGIN)
        .expect("issued here");
    let promoted = arbiter.promote(Monotonic::ORIGIN);
    assert_eq!(promoted.len(), 1);
    assert_eq!(promoted[0].ticket, first);
    assert_ne!(promoted[0].ticket, second);
}

/// ⛔ THE COUNTER-PROBE, and it is the one that says `promote` is not "grant everything in
/// the queue": with no room freed it promotes NOTHING and the books do not move.
#[test]
fn promote_with_no_room_freed_promotes_nothing() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let Admission::Granted(_resident) = arbiter.admit(
        &profile("resident", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("full");
    };
    let Admission::Queued(_) = arbiter.admit(
        &profile("waiting", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("queued");
    };

    let promoted = arbiter.promote(Monotonic::ORIGIN);
    assert!(promoted.is_empty());
    assert_eq!(arbiter.allocated(), Mib::new(4_096));
    assert_eq!(arbiter.queued(), 1, "the ticket is still waiting");
}

/// ⛔ AND A PROMOTION IS A GRANT LIKE ANY OTHER: what comes out of the queue can be released
/// and gives back exactly its reservation. Without this the queue could hand out grants the
/// books never learned about.
#[test]
fn a_promoted_grant_is_a_grant_like_any_other() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let Admission::Granted(resident) = arbiter.admit(
        &profile("resident", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("full");
    };
    let Admission::Queued(_) = arbiter.admit(
        &profile("waiting", 2_048, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("queued");
    };
    let _ = arbiter
        .release(resident, Monotonic::ORIGIN)
        .expect("issued here");

    let mut promoted: Vec<Promotion> = arbiter.promote(Monotonic::ORIGIN);
    assert_eq!(promoted.len(), 1);
    let promotion = promoted.remove(0);
    assert_eq!(arbiter.allocated(), Mib::new(2_048));

    let returned = arbiter
        .release(promotion.grant, Monotonic::ORIGIN)
        .expect("the promotion came from this arbiter");
    assert_eq!(returned, Mib::new(2_048));
    assert_eq!(arbiter.allocated(), Mib::ZERO);
}
```

- [ ] **Passo 2: eseguirle e vederle fallire**

Run: `cargo test --locked -p kernel --test arbiter_admission`
Expected: **FAIL** — `queued`, `promote` e `Promotion` non esistono.

- [ ] **Passo 3: la coda per corsia**

In `crates/kernel/src/arbiter/mod.rs`:

```rust
use alloc::vec::Vec;

/// A ticket that came out of the queue, with the grant it waited for.
///
/// ⚠️ IT IS A STRUCT AND NOT A TUPLE, and the reason is the same one design/02 gives for
/// `Refused`: whoever reads `promotion.ticket` should not have to remember which of two
/// unnamed slots held it. A tuple of two things one of which has no `Debug` is worse still.
pub struct Promotion {
    pub ticket: TicketId,
    pub grant: Grant,
}

/// A request waiting in its lane.
struct Waiting {
    ticket: TicketId,
    profile: ResourceProfile,
    valid_for: Millis,
}
```

E dentro `impl Arbiter`, i campi `next_ticket: u64` e
`queues: BTreeMap<ComputeClass, Vec<Waiting>>`, più:

```rust
    /// How many requests are waiting, across all lanes.
    pub fn queued(&self) -> usize {
        self.queues.values().map(Vec::len).sum()
    }

    /// Serves the queue with whatever room there is now, BEST LANE FIRST.
    ///
    /// ⛔ THE ORDER IS BY LANE AND NOT BY ARRIVAL, and it is a measurement rather than a
    /// taste: §5.3.1 says M-7's numbers stay valid AS AN UPPER BOUND precisely because the
    /// specified version keeps the order PER LANE. A single queue re-sorted on every
    /// release would invalidate that measurement, and it would have to be redone.
    ///
    /// ⛔ IT STOPS AT THE FIRST REQUEST THAT DOES NOT FIT, WITHIN A LANE, and does not skip
    /// ahead to a smaller one. Skipping is a scheduling policy nobody decided, and it would
    /// let a large request in a busy lane wait for ever behind small ones.
    ///
    /// ⚠️ `BTreeMap` ITERATES IN KEY ORDER, and `ComputeClass` orders by its explicit
    /// priority key -- so "best lane first" costs nothing here. That is the coupling the
    /// probe `the_lane_order_is_pinned_by_name_and_realtime_comes_first` protects.
    pub fn promote(&mut self, now: Monotonic) -> Vec<Promotion> {
        self.collect_expired(now);

        let mut promoted = Vec::new();
        let ceiling = self.parameters.total_vram();

        for lane in [
            ComputeClass::Realtime,
            ComputeClass::Interactive,
            ComputeClass::Batch,
        ] {
            loop {
                let Some(waiting) = self.queues.get(&lane).and_then(|queue| queue.first()) else {
                    break;
                };
                let asked = waiting.profile.reserved_vram;
                if self.allocated().saturating_add(asked) > ceiling {
                    break;
                }
                let waiting = self
                    .queues
                    .get_mut(&lane)
                    .expect("the lane was just read")
                    .remove(0);
                let grant = self.issue(&waiting.profile, waiting.valid_for, now);
                promoted.push(Promotion {
                    ticket: waiting.ticket,
                    grant,
                });
            }
        }
        promoted
    }
```

⛔ **E `admit` guadagna il ramo della coda**, che finalmente dà un produttore a `Queued`:
dopo il rifiuto per «più grande della macchina», se non entra **adesso** si accoda invece di
rifiutare. La costruzione del `Held` si estrae in un aiutante privato `issue`, che `admit` e
`promote` condividono — ⛔ **un secondo posto in cui si costruisce una concessione sarebbe il
secondo modo di ottenerne una**, cioè la cosa che §5.6 esiste per togliere.

- [ ] **Passo 4: verde, e le mutazioni**

Run: `cargo test --locked -p kernel --test arbiter_admission`
Expected: **PASS**, tredici test.

| # | Mutazione | Sonda che deve morire |
|---|---|---|
| 1 | in `promote`, iterare le corsie in ordine `Batch, Interactive, Realtime` | `the_queue_promotes_by_lane_and_not_in_arrival_order` |
| 2 | in `promote`, `remove(0)` diventa `pop()` | `inside_one_lane_the_order_is_the_order_of_arrival` |
| 3 | togliere il controllo `saturating_add(asked) > ceiling` dentro `promote` | `promote_with_no_room_freed_promotes_nothing` **e** `the_sum_of_the_grants_never_exceeds_the_total` |
| 4 | in `admit`, restituire `Refused` invece di accodare | `a_request_that_fits_the_machine_but_not_the_moment_is_queued` |

⛔ **La 3 uccide due sonde, e prima di concludere che non le distingua** si cerca **una terza
mutazione che lasci passare la prima** — la regola nata dal Task 3 del Traguardo 4. Qui c'è:
`promote` che promuove **una sola** voce e poi esce lascia verde
`promote_with_no_room_freed_promotes_nothing` e rossa
`a_promoted_grant_is_a_grant_like_any_other`. Le due sonde stanno su **assi diversi**.

- [ ] **Passo 5: il cancello, il registro, il commit**

Run: `bash scripts/gate.sh` → `GATE GREEN`.

```bash
git add crates/kernel docs/porta-di-qualita.md
git commit -m "feat(arbiter): le code sono per corsia, ed e cio che tiene validi i numeri di M-7"
```

---

### Task 7: la revoca, e la grazia che scade

**Files:**
- Modify: `crates/kernel/src/arbiter/mod.rs`
- Modify: `crates/kernel/tests/arbiter_admission.rs`
- Modify: `docs/porta-di-qualita.md`

**Interfaces:**
- Consumes: tutto il Task 6.
- Produces:
  ```rust
  impl Arbiter {
      pub fn revoking(&self) -> usize;
      pub(crate) fn ask_back(&mut self, needed: Mib, below: ComputeClass, now: Monotonic) -> Mib;
  }
  ```
  ⚠️ `ask_back` è `pub(crate)` perché il suo unico chiamante è l'ammissione sotto policy
  LOCALE, che nasce al Task 8. **Se al momento di eseguire questo compito non ha ancora un
  chiamante**, si ferma e lo si porta indietro: un metodo senza consumatore è il gotcha
  **#46** dal verso sbagliato, e la risposta di questo repository è la **finta in un banco**.

⛔ **Il pre-controllo di questo compito, prima di dispacciarlo:** *quali sonde passano
ATTRAVERSO il difetto che sto chiudendo, e quali di esse resteranno verdi senza più provare
nulla?* È il gotcha **#66**, e la candidata è `a_grant_still_inside_its_window_is_not_collected`
— con la riscossione forzata dentro `collect_expired`, quella sonda potrebbe smettere di
discriminare. **Si muta e si guarda**, prima di dichiarare il compito finito.

- [ ] **Passo 1: le sonde della revoca**

In coda a `crates/kernel/tests/arbiter_admission.rs`:

```rust
use kernel::arbiter::{Activity, PreemptibleState};
use kernel::time::Millis as Grace;

fn preemptible(name: &'static str, vram: u64, lane: ComputeClass, grace: u64) -> ResourceProfile {
    ResourceProfile {
        name,
        reserved_vram: Mib::new(vram),
        compute_class: lane,
        preemption: Preemption::After(Grace::new(grace)),
    }
}

/// ⛔ ASKING BACK MARKS, IT DOES NOT TAKE. The reservation stays in the books for the whole
/// grace period: §5.3 point 4 gives the holder that long to hand it over, and an arbiter
/// that freed the memory at once would be admitting a second consumer onto VRAM the first
/// one is still using.
#[test]
fn asking_a_grant_back_marks_it_and_does_not_free_it_yet() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let Admission::Granted(_batch) = arbiter.admit(
        &preemptible("batch-resident", 4_096, ComputeClass::Batch, 500),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine");
    };

    let asked_back = arbiter.ask_back(Mib::new(4_096), ComputeClass::Interactive, Monotonic::ORIGIN);

    assert_eq!(asked_back, Mib::new(4_096), "one grant covers the need");
    assert_eq!(arbiter.revoking(), 1);
    assert_eq!(
        arbiter.allocated(),
        Mib::new(4_096),
        "the memory is still the holder's until the grace runs out"
    );
}

/// ⛔ THE GRACE IS COLLECTED, AND IT IS THE ARBITER HALF OF `Forzata` (§6.5 of the design).
/// The other half -- actually killing the process -- needs `process` and is milestone 6.
#[test]
fn a_grace_that_ran_out_returns_the_reservation_to_the_budget() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let Admission::Granted(_batch) = arbiter.admit(
        &preemptible("batch-resident", 4_096, ComputeClass::Batch, 500),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine");
    };
    let _ = arbiter.ask_back(Mib::new(4_096), ComputeClass::Interactive, Monotonic::ORIGIN);

    // Still inside the grace: nothing is free yet.
    assert_eq!(arbiter.allocated(), Mib::new(4_096));
    // Past it: the first one who looks finds the budget back.
    let after = arbiter.admit(
        &profile("the-interactive-one", 4_096, ComputeClass::Interactive),
        LONG,
        Monotonic::from_millis(501),
    );
    assert!(matches!(after, Admission::Granted(_)));
    assert_eq!(arbiter.allocated(), Mib::new(4_096), "one grant, not two");
    assert_eq!(arbiter.revoking(), 0);
}

/// ⛔ A NON-PREEMPTIBLE GRANT IS NEVER ASKED BACK, and this is `I2 · §5.3` seen from the
/// runtime side -- the type already makes `Revoking` unspellable for it, this says the
/// arbiter does not even try.
#[test]
fn a_non_preemptible_grant_is_never_asked_back() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let Admission::Granted(_audio) = arbiter.admit(
        &profile("audio-reserved", 4_096, ComputeClass::Realtime),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine");
    };

    let asked_back = arbiter.ask_back(Mib::new(4_096), ComputeClass::Interactive, Monotonic::ORIGIN);

    assert_eq!(asked_back, Mib::ZERO, "nothing here can be taken back");
    assert_eq!(arbiter.revoking(), 0);
    assert_eq!(arbiter.allocated(), Mib::new(4_096));
}

/// ⛔ ONLY LOWER LANES, and it is the counter-probe of the one above by the other road: a
/// `Realtime` job is not evicted to make room for an `Interactive` one, no matter how
/// preemptible its profile says it is.
#[test]
fn only_lanes_below_the_asking_one_are_asked_back() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let Admission::Granted(_realtime) = arbiter.admit(
        &preemptible("realtime-resident", 4_096, ComputeClass::Realtime, 500),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine");
    };

    let asked_back = arbiter.ask_back(Mib::new(4_096), ComputeClass::Interactive, Monotonic::ORIGIN);

    assert_eq!(asked_back, Mib::ZERO, "Realtime is not below Interactive");
    assert_eq!(arbiter.revoking(), 0);
}

/// ⛔ IT STOPS WHEN IT HAS ENOUGH, and the assertion is on the NUMBER: an arbiter that
/// revoked everything preemptible would satisfy "it made room" and evict two jobs to seat
/// one.
#[test]
fn asking_back_stops_as_soon_as_the_need_is_covered() {
    let mut arbiter = arbiter(Mib::new(8_192));
    for name in ["batch-a", "batch-b"] {
        let outcome = arbiter.admit(
            &preemptible(name, 4_096, ComputeClass::Batch, 500),
            LONG,
            Monotonic::ORIGIN,
        );
        assert!(matches!(outcome, Admission::Granted(_)));
    }

    let asked_back = arbiter.ask_back(Mib::new(4_096), ComputeClass::Interactive, Monotonic::ORIGIN);

    assert_eq!(asked_back, Mib::new(4_096));
    assert_eq!(arbiter.revoking(), 1, "one was enough");
}

/// ⛔ THE PROBE THAT PROTECTS THE ONE FROM TASK 5 FROM BECOMING VACUOUS (gotcha #66): with
/// forced reclamation now living inside the same sweep, a grant that is neither expired nor
/// revoking must still survive it.
#[test]
fn a_grant_that_is_neither_expired_nor_revoking_survives_the_sweep() {
    let mut arbiter = arbiter(Mib::new(4_096));
    let Admission::Granted(_resident) = arbiter.admit(
        &preemptible("resident", 4_096, ComputeClass::Batch, 500),
        Millis::new(5_000),
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine");
    };

    let refused = arbiter.admit(
        &profile("late-comer", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::from_millis(4_999),
    );
    assert!(
        matches!(refused, Admission::Refused { .. }),
        "nothing has expired and nothing was asked back"
    );
    assert_eq!(arbiter.allocated(), Mib::new(4_096));
}
```

- [ ] **Passo 2: eseguirle e vederle fallire**

Run: `cargo test --locked -p kernel --test arbiter_admission`
Expected: **FAIL** — `ask_back` e `revoking` non esistono.

- [ ] **Passo 3: la revoca**

In `impl Arbiter`:

```rust
    /// How many grants have been asked back and have not handed over yet.
    pub fn revoking(&self) -> usize {
        self.held
            .values()
            .filter(|held| {
                matches!(
                    held.activity,
                    Activity::Preemptible(PreemptibleState::Revoking { .. })
                )
            })
            .count()
    }

    /// Asks back enough preemptible grants FROM LANES BELOW `below` to cover `needed`, and
    /// answers with how much was actually asked back.
    ///
    /// ⛔ IT MARKS, IT DOES NOT TAKE. The reservation stays in the books for the whole grace
    /// period: §5.3 point 4 gives the holder that long, and freeing the memory at once would
    /// seat a second consumer on VRAM the first is still using. The forced reclamation
    /// happens in the sweep, when the grace has run out.
    ///
    /// ⛔ IT STOPS AS SOON AS THE NEED IS COVERED. "It made room" is satisfied by revoking
    /// everything, which evicts two jobs to seat one.
    ///
    /// ⚠️ `pub(crate)` BECAUSE ITS ONLY CALLER IS THE ADMISSION UNDER THE LOCAL POLICY. It is
    /// not a public operation: making room is a consequence of a request, never a thing
    /// somebody asks for.
    pub(crate) fn ask_back(
        &mut self,
        needed: Mib,
        below: ComputeClass,
        now: Monotonic,
    ) -> Mib {
        let mut covered = Mib::ZERO;
        // ⛔ Worst lane first: the cheapest thing to interrupt goes first. `BTreeMap` gives
        // key order, so this reverses it.
        for lane in [
            ComputeClass::Batch,
            ComputeClass::Interactive,
            ComputeClass::Realtime,
        ] {
            if lane <= below {
                // Not BELOW the asking lane: a Realtime job is not evicted for an
                // Interactive one, however preemptible its profile says it is.
                continue;
            }
            for held in self.held.values_mut() {
                if covered >= needed {
                    return covered;
                }
                if held.lane != lane {
                    continue;
                }
                if let Activity::Preemptible(PreemptibleState::Running) = held.activity {
                    let deadline = match held_grace(held) {
                        Some(grace) => now.saturating_add(grace),
                        None => continue,
                    };
                    held.activity =
                        Activity::Preemptible(PreemptibleState::Revoking { deadline });
                    covered = covered.saturating_add(held.reserved);
                }
            }
        }
        covered
    }
```

⚠️ **`Held` guadagna il proprio tempo di grazia**, perché `Preemption` vive nel profilo e il
profilo non si conserva: si aggiunge `grace: Option<Millis>` a `Held`, riempito da `issue`
con `profile.preemption.grace()`, e `held_grace` è l'accessore. ⛔ **Conservare il profilo
intero sarebbe più comodo e più caro:** metterebbe `name` e `compute_class` in due posti, e
un `&'static str` ritenuto dentro l'arbitro è uno stato che nessuna decisione legge.

E `collect_expired` diventa la **riscossione**, che copre **due** scadenze:

```rust
    /// ⛔ TWO DEADLINES, ONE SWEEP, and they are genuinely different things: `expires_at` is
    /// the validity window the requester declared, `deadline` is the grace a revocation
    /// gave. Both are on the MONOTONIC axis, never wall time (§5.3 point 2).
    fn collect_expired(&mut self, now: Monotonic) {
        self.held.retain(|_, held| {
            if held.expires_at <= now {
                return false;
            }
            match held.activity {
                Activity::Preemptible(PreemptibleState::Revoking { deadline }) => deadline > now,
                _ => true,
            }
        });
    }
```

- [ ] **Passo 4: verde, le mutazioni, e la caccia alla VACUITÀ**

Run: `cargo test --locked -p kernel --test arbiter_admission`
Expected: **PASS**, diciannove test.

| # | Mutazione | Sonda che deve morire |
|---|---|---|
| 1 | `ask_back` libera subito invece di marcare | `asking_a_grant_back_marks_it_and_does_not_free_it_yet` |
| 2 | togliere il `continue` su `lane <= below` | `only_lanes_below_the_asking_one_are_asked_back` |
| 3 | togliere il `return covered` anticipato | `asking_back_stops_as_soon_as_the_need_is_covered` |
| 4 | nel nuovo `collect_expired`, `deadline > now` diventa `true` | `a_grace_that_ran_out_returns_the_reservation_to_the_budget` |
| 5 | nel nuovo `collect_expired`, il ramo `Revoking` diventa `false` | `a_grant_that_is_neither_expired_nor_revoking_survives_the_sweep` |

⛔ **E poi la domanda del gotcha #66, che nessuna delle cinque pone:** la sonda del Task 5
`a_grant_still_inside_its_window_is_not_collected` **passava attraverso** un
`collect_expired` che aveva un ramo solo. Si **muta** — `expires_at <= now` in
`expires_at < now` — e si guarda:

- se va **rossa**, discrimina ancora e resta com'è;
- se resta **verde**, è diventata **vacua** e va riscritta, non lasciata. Una vacuità non si
  vede, un rosso sì.

Registrare l'esito **misurato**, quale che sia.

- [ ] **Passo 5: il cancello, il registro, il commit**

Run: `bash scripts/gate.sh` → `GATE GREEN`.

```bash
git add crates/kernel docs/porta-di-qualita.md
git commit -m "feat(arbiter): chiedere indietro marca e non prende, e la grazia che scade si riscuote"
```

---

# Parte 3 — le due policy

Due compiti. Il Task 8 dà alle policy **qualcosa da fare**, il Task 9 rende la transizione un
passo del giornale — che è la **proprietà DST numero 4**, l'unica delle cinque che si inietta
su una porta con due implementazioni vere.

---

### Task 8: `VramPolicy` — due oggetti, e la decisione sta dentro l'ammissione

**Files:**
- Create: `crates/kernel/src/arbiter/policy.rs`
- Modify: `crates/kernel/src/arbiter/mod.rs`
- Create: `crates/kernel/tests/arbiter_policy.rs`
- Create: `crates/kernel/tests/compile_fail/two_policies_at_once.rs`
- Modify: `docs/porta-di-qualita.md`

**Interfaces:**
- Consumes: tutto il Parte 2, in particolare `Arbiter::ask_back`.
- Produces:
  ```rust
  pub trait MakeRoom { fn may_make_room(&self) -> bool; fn name(&self) -> &'static str; }
  pub struct RemotePolicy;
  pub struct LocalPolicy;
  pub enum VramPolicy { Remote(RemotePolicy), Local(LocalPolicy) }
  impl Arbiter { pub fn new(parameters: Parameters, policy: VramPolicy) -> Self; pub fn policy(&self) -> &VramPolicy; }
  ```

⛔ **La riga di catalogo che questo compito chiude:** `V3` — *«una seconda policy attiva: il
valore consegnato ne porta una sola»*.

⛔ **E `Arbiter::new` cambia firma di nuovo.** È il secondo compito che la tocca, e va detto:
i chiamanti sono i banchi dei Task 5–7 più nessun altro, perché `daemon` cabla l'arbitro solo
al Task 10. **Si ricontano col `grep` prima di cominciare.**

- [ ] **Passo 1: le sonde delle due policy**

`crates/kernel/tests/arbiter_policy.rs`:

```rust
//! The two VRAM policies (ADR-0006), and what tells them apart is ONE DECISION INSIDE THE
//! ADMISSION PATH: "a request does not fit. Can room be made?"
//!
//! ⛔ AND NO MODEL IS NEEDED FOR THAT, which is what makes this provable at milestone 5.
//! "Evicting a resident" IS "revoking a preemptible grant" -- a mechanism task 7 built
//! anyway -- so the two policies are exercised with synthetic grants declared by the bench.
//! Zero speculation.

use kernel::arbiter::{
    Admission, Arbiter, ComputeClass, LocalPolicy, Mib, Preemption, RemotePolicy,
    ResourceProfile, VramPolicy,
};
use kernel::parameters::Parameters;
use kernel::time::{Millis, Monotonic};

const TURN_LIMIT: u64 = 10_000;
const LONG: Millis = Millis::new(1_000_000);

fn preemptible(name: &'static str, vram: u64, lane: ComputeClass) -> ResourceProfile {
    ResourceProfile {
        name,
        reserved_vram: Mib::new(vram),
        compute_class: lane,
        preemption: Preemption::After(Millis::new(500)),
    }
}

fn arbiter(total: u64, policy: VramPolicy) -> Arbiter {
    Arbiter::new(Parameters::new(TURN_LIMIT, Mib::new(total)), policy)
}

/// ⛔ THE DEFAULT, AND IT IS NOT A DETAIL: ADR-0006 makes REMOTE the default, and reopening
/// that turns a coordinated swap from an exception into the normal case.
#[test]
fn the_remote_policy_does_not_make_room_it_queues() {
    let mut arbiter = arbiter(4_096, VramPolicy::Remote(RemotePolicy));
    let Admission::Granted(_resident) = arbiter.admit(
        &preemptible("resident", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine");
    };

    let outcome = arbiter.admit(
        &preemptible("newcomer", 4_096, ComputeClass::Interactive),
        LONG,
        Monotonic::ORIGIN,
    );

    assert!(matches!(outcome, Admission::Queued(_)));
    assert_eq!(arbiter.revoking(), 0, "REMOTE revokes nothing to make room");
    assert_eq!(arbiter.allocated(), Mib::new(4_096));
}

/// ⛔ THE OTHER OBJECT, AND THE SAME CALL SITE. This is the difference ADR-0006 says must
/// NOT be an `if` planted in the middle of the admission: two objects with one interface
/// keep it in one place, where a conditional would drift invisibly.
#[test]
fn the_local_policy_asks_the_lower_lanes_back() {
    let mut arbiter = arbiter(4_096, VramPolicy::Local(LocalPolicy));
    let Admission::Granted(_resident) = arbiter.admit(
        &preemptible("resident", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine");
    };

    let outcome = arbiter.admit(
        &preemptible("newcomer", 4_096, ComputeClass::Interactive),
        LONG,
        Monotonic::ORIGIN,
    );

    // ⛔ STILL QUEUED, and that is the honest answer: the room is not free until the holder
    // hands it over. What LOCAL changed is that somebody was ASKED.
    assert!(matches!(outcome, Admission::Queued(_)));
    assert_eq!(arbiter.revoking(), 1, "LOCAL asked the Batch resident back");
    assert_eq!(arbiter.allocated(), Mib::new(4_096), "nothing freed yet");
}

/// ⛔ AND WHAT THE ASKING BUYS, END TO END: past the grace the queued request is served.
/// Without this the probe above proves a marking that leads nowhere.
#[test]
fn under_the_local_policy_the_queued_request_is_served_past_the_grace() {
    let mut arbiter = arbiter(4_096, VramPolicy::Local(LocalPolicy));
    let Admission::Granted(_resident) = arbiter.admit(
        &preemptible("resident", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine");
    };
    let Admission::Queued(ticket) = arbiter.admit(
        &preemptible("newcomer", 4_096, ComputeClass::Interactive),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("queued");
    };

    let promoted = arbiter.promote(Monotonic::from_millis(501));

    assert_eq!(promoted.len(), 1);
    assert_eq!(promoted[0].ticket, ticket);
    assert_eq!(arbiter.allocated(), Mib::new(4_096), "one grant, not two");
}

/// The counter-probe of the one above: under REMOTE the same clock advance serves NOBODY,
/// because nobody was ever asked back.
#[test]
fn under_the_remote_policy_the_same_clock_advance_serves_nobody() {
    let mut arbiter = arbiter(4_096, VramPolicy::Remote(RemotePolicy));
    let Admission::Granted(_resident) = arbiter.admit(
        &preemptible("resident", 4_096, ComputeClass::Batch),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("it fills the machine");
    };
    let Admission::Queued(_) = arbiter.admit(
        &preemptible("newcomer", 4_096, ComputeClass::Interactive),
        LONG,
        Monotonic::ORIGIN,
    ) else {
        panic!("queued");
    };

    let promoted = arbiter.promote(Monotonic::from_millis(501));

    assert!(promoted.is_empty(), "REMOTE asked nobody, so nothing came free");
    assert_eq!(arbiter.allocated(), Mib::new(4_096));
}

/// The name, so a journalled transition has something true to write down.
#[test]
fn each_policy_names_itself() {
    assert_eq!(VramPolicy::Remote(RemotePolicy).name(), "remote");
    assert_eq!(VramPolicy::Local(LocalPolicy).name(), "local");
}
```

- [ ] **Passo 2: eseguirle e vederle fallire**

Run: `cargo test --locked -p kernel --test arbiter_policy`
Expected: **FAIL** — il modulo `policy` non esiste.

- [ ] **Passo 3: le due policy**

`crates/kernel/src/arbiter/policy.rs`:

```rust
//! The two VRAM policies of ADR-0006. TWO OBJECTS WITH ONE INTERFACE, one active at a time,
//! chosen by the configuration profile -- NOT two arms of a conditional.
//!
//! ⛔ THE REASON IS IN ADR-0006 AND IT IS ABOUT DRIFT: an `if` on the origin of the
//! inference, planted in the middle of the admission, spreads invisibly as the admission
//! grows. Duplication between two objects is VISIBLE and BOUNDED; the drift of a conditional
//! is invisible and diffuse.
//!
//! ⛔ AND AT MILESTONE 5 THEY ARE NOT EMPTY SHELLS, which was the open question. The
//! difference is ONE DECISION inside the admission path -- "a request does not fit. Can room
//! be made?" -- and no model is needed to answer it: evicting a resident IS revoking a
//! preemptible grant, a mechanism §6 built anyway.

/// What the admission asks the active policy.
///
/// ⛔ ONE QUESTION AND NOT A FAMILY OF THEM. A rich interface here would invite the
/// admission to branch on the policy in several places, which is the conditional ADR-0006
/// refuses, arriving by another road.
pub trait MakeRoom {
    /// A request does not fit. May the arbiter take resources back to seat it?
    fn may_make_room(&self) -> bool;

    /// What this policy is called, for the journalled transition of §5.4.
    fn name(&self) -> &'static str;
}

/// The DEFAULT (ADR-0006): OpenRouter, VRAM free. Nothing is revoked to make room -- the
/// request queues, or it is refused.
pub struct RemotePolicy;

impl MakeRoom for RemotePolicy {
    fn may_make_room(&self) -> bool {
        false
    }

    fn name(&self) -> &'static str {
        "remote"
    }
}

/// Local inference: the machine's VRAM is the working set, so seating a new request may mean
/// taking resources back from preemptible grants in lower lanes.
pub struct LocalPolicy;

impl MakeRoom for LocalPolicy {
    fn may_make_room(&self) -> bool {
        true
    }

    fn name(&self) -> &'static str {
        "local"
    }
}

/// The policy the arbiter was BUILT WITH. ⛔ ONE, AND THE TYPE SAYS SO: "two active
/// policies" is NOT EXPRESSIBLE, which is `V3` at level 1 rather than a test at level 2
/// (§7.4.1 C, and §5.4 said this rule would rise to the compiler).
pub enum VramPolicy {
    Remote(RemotePolicy),
    Local(LocalPolicy),
}

impl MakeRoom for VramPolicy {
    fn may_make_room(&self) -> bool {
        match self {
            VramPolicy::Remote(policy) => policy.may_make_room(),
            VramPolicy::Local(policy) => policy.may_make_room(),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            VramPolicy::Remote(policy) => policy.name(),
            VramPolicy::Local(policy) => policy.name(),
        }
    }
}
```

E in `Arbiter::admit`, il ramo che oggi accoda:

```rust
        if self.allocated().saturating_add(asked) > ceiling {
            // ⛔ THE ONE PLACE THE TWO POLICIES DIFFER. ADR-0006 says exactly this is where
            // a conditional would have been planted, and why it is not one.
            if self.policy.may_make_room() {
                let needed = self
                    .allocated()
                    .saturating_add(asked)
                    .saturating_sub(ceiling);
                let _asked_back = self.ask_back(needed, profile.compute_class, now);
            }
            return self.enqueue(profile, valid_for);
        }
```

- [ ] **Passo 4: il caso negativo di `V3`**

`crates/kernel/tests/compile_fail/two_policies_at_once.rs`:

```rust
// `V3`: the delivered value carries ONE policy. "Two active at once" is not expressible --
// `VramPolicy` is an enum, so an arbiter cannot be handed both.
fn main() {
    let _both = kernel::arbiter::VramPolicy::Remote(kernel::arbiter::RemotePolicy)
        | kernel::arbiter::VramPolicy::Local(kernel::arbiter::LocalPolicy);
}
```

Generare l'oracolo **una volta** e **leggerlo**: deve dire che `VramPolicy` non implementa
`BitOr` — cioè che due policy non si sommano. ⚠️ **La sigla esatta si legge dall'uscita
vera**, non si predice qui.

- [ ] **Passo 5: verde, e le mutazioni**

Run: `cargo test --locked -p kernel --test arbiter_policy`
Expected: **PASS**, cinque test.

| # | Mutazione | Sonda che deve morire |
|---|---|---|
| 1 | `RemotePolicy::may_make_room` restituisce `true` | `the_remote_policy_does_not_make_room_it_queues` e `under_the_remote_policy_the_same_clock_advance_serves_nobody` |
| 2 | `LocalPolicy::may_make_room` restituisce `false` | `the_local_policy_asks_the_lower_lanes_back` |
| 3 | togliere del tutto la chiamata a `may_make_room` in `admit` | come la 2 |

⛔ **La 1 uccide due sonde: si cerca la terza mutazione che ne lasci passare una**, come
prescrive la regola nata al Traguardo 4. C'è: `promote` che ignora le corsie lascia verde
`the_remote_policy_does_not_make_room_it_queues` e rossa
`under_the_remote_policy_the_same_clock_advance_serves_nobody`. Assi diversi, non
competizione.

- [ ] **Passo 6: il cancello, il registro, il commit**

Run: `bash scripts/gate.sh` → `GATE GREEN`.

```bash
git add crates/kernel docs/porta-di-qualita.md
git commit -m "feat(arbiter): due policy come due oggetti, e la domanda che le distingue sta dentro l'ammissione"
```

---

### Task 9: la transizione di policy è un passo giornalato

**Files:**
- Modify: `crates/kernel/src/arbiter/mod.rs`
- Modify: `crates/kernel/tests/arbiter_policy.rs`
- Modify: `docs/porta-di-qualita.md`

**Interfaces:**
- Consumes: il Task 8; `kernel::ports::journal::{Journal, StepId}`;
  `kernel::record::{Record, RecordV1, RecordKind, EffectClass, Trust}`.
- Produces:
  ```rust
  impl Arbiter {
      pub fn set_policy<J: Journal>(
          &mut self,
          policy: VramPolicy,
          step: StepId,
          journal: &mut J,
          now: Monotonic,
      ) -> Result<(), JournalError>;
  }
  ```

⛔ **È la proprietà DST numero 4**, e questo compito è ciò che la rende scrivibile al Task 12.

⛔ **Il pre-controllo, prima di dispacciarlo:** *in quale altro stato del mondo questa
asserzione resterebbe verde?* Una sonda che chiede solo *«dopo la transizione la policy è
l'altra»* è verde anche con **zero record scritti**. Le asserzioni qui si fanno sull'**archivio**.

- [ ] **Passo 1: le sonde della transizione**

In coda a `crates/kernel/tests/arbiter_policy.rs`:

```rust
use kernel::ports::journal::{Journal, StepId};
use kernel::record::{EffectClass, Record, RecordKind, Trust};
use kernel::reconcile::{Resolution, steps_in_doubt};
use simulator::journal::{CrashingJournal, MemoryJournal};

/// ⛔ THE ASSERTION IS ON THE ARCHIVE, NOT ON THE POLICY. "After the transition the policy is
/// the other one" is green with ZERO records written, and V6 is exactly the claim that
/// nothing happens before the intent is durable.
#[test]
fn a_policy_transition_writes_its_intent_before_its_outcome() {
    let mut journal = MemoryJournal::new();
    let mut arbiter = arbiter(4_096, VramPolicy::Remote(RemotePolicy));

    arbiter
        .set_policy(
            VramPolicy::Local(LocalPolicy),
            StepId::new(1),
            &mut journal,
            Monotonic::ORIGIN,
        )
        .expect("the journal accepts");

    assert_eq!(arbiter.policy().name(), "local");

    let entries = journal.replay().expect("the archive reads back");
    assert_eq!(entries.len(), 2, "an intent AND an outcome");

    let kinds: Vec<RecordKind> = entries
        .iter()
        .map(|(_, bytes)| match Record::decode(bytes).expect("our own bytes") {
            Record::V1(record) => record.kind,
        })
        .collect();
    assert_eq!(kinds, vec![RecordKind::Intent, RecordKind::Outcome]);
}

/// ⛔ THE HALF THAT V6 IS ACTUALLY ABOUT: a journal that refuses the intent means the
/// transition DOES NOT HAPPEN. Without this the write-ahead is decoration.
#[test]
fn a_refused_intent_leaves_the_policy_where_it_was() {
    let mut journal = CrashingJournal::falling_at(0);
    let mut arbiter = arbiter(4_096, VramPolicy::Remote(RemotePolicy));

    let outcome = arbiter.set_policy(
        VramPolicy::Local(LocalPolicy),
        StepId::new(1),
        &mut journal,
        Monotonic::ORIGIN,
    );

    assert!(outcome.is_err());
    assert_eq!(
        arbiter.policy().name(),
        "remote",
        "nothing executes before the intent is durable"
    );
}

/// ⛔ AND A TRANSITION CUT IN HALF LEAVES A RECONCILABLE STEP -- which is DST property 4,
/// asserted here on ONE constructed state so that the campaign of task 12 has a shape to
/// look for rather than a hope.
#[test]
fn a_transition_cut_between_intent_and_outcome_leaves_the_step_in_doubt() {
    let mut journal = CrashingJournal::falling_at(1);
    let mut arbiter = arbiter(4_096, VramPolicy::Remote(RemotePolicy));

    let outcome = arbiter.set_policy(
        VramPolicy::Local(LocalPolicy),
        StepId::new(7),
        &mut journal,
        Monotonic::ORIGIN,
    );
    assert!(outcome.is_err(), "the outcome never reached the archive");

    let survivor = journal.into_survivor();
    let doubts = steps_in_doubt(&survivor).expect("the archive reads back");

    assert_eq!(doubts.len(), 1);
    assert_eq!(doubts[0].step, StepId::new(7));
    assert_eq!(
        doubts[0].resolution,
        Resolution::RunAgain,
        "a policy transition is idempotent: re-running converges"
    );
}

/// The counter-probe of the one above, and it is the direction that is skipped: WITHOUT a
/// crash, no step is in doubt. Otherwise "there is a doubt" would be satisfied by an arbiter
/// that never writes an outcome at all.
#[test]
fn without_a_crash_a_transition_leaves_no_step_in_doubt() {
    let mut journal = MemoryJournal::new();
    let mut arbiter = arbiter(4_096, VramPolicy::Remote(RemotePolicy));

    arbiter
        .set_policy(
            VramPolicy::Local(LocalPolicy),
            StepId::new(7),
            &mut journal,
            Monotonic::ORIGIN,
        )
        .expect("the journal accepts");

    assert!(steps_in_doubt(&journal).expect("reads back").is_empty());
}
```

⚠️ **Questo banco guadagna una dipendenza di sviluppo da `simulator`**, che è dove vivono
`MemoryJournal` e `CrashingJournal`. ⛔ **Si verifica che sia già dichiarata** in
`crates/kernel/Cargo.toml` sotto `[dev-dependencies]` — se non lo è, **è una dipendenza
nuova** e vale il vincolo globale 9: manifesto **e** lockfile, in due passi, committati
insieme. ⚠️ Se invece `kernel` non può dipendere da `simulator` nemmeno in `dev`, queste
quattro sonde si spostano in `crates/simulator/tests/`, e **si registra la divergenza**.

- [ ] **Passo 2: eseguirle e vederle fallire**

Run: `cargo test --locked -p kernel --test arbiter_policy`
Expected: **FAIL** — `set_policy` non esiste.

- [ ] **Passo 3: la transizione**

In `impl Arbiter`:

```rust
    /// Swaps the active policy, AS A JOURNALLED STEP (§5.4).
    ///
    /// ⛔ INTENT, THEN THE EFFECT, THEN THE OUTCOME -- and the order is V6 rather than
    /// tidiness. Changing policy has real effects on the world (evictions, reloads), and
    /// nothing executes before the intent is DURABLE. A transition cut in half leaves a step
    /// IN DOUBT, reconcilable like every other (§4.3).
    ///
    /// ⛔ THE JOURNAL COMES BY REFERENCE AND IS NOT OWNED, for the mechanical reason already
    /// written about the reactor: an arbiter that owned one would give it two owners the day
    /// a caller needs it too, and the borrow would not pass.
    ///
    /// ⛔ `EffectClass::Idempotent`, AND IT IS ARGUED RATHER THAN PICKED. ADR-0007 treats an
    /// effect with no declared class as `Unrepeatable`, so the choice has to be earned:
    /// "make the active policy be X" CONVERGES when re-run, which is what `Idempotent`
    /// means. ⚠️ THE DECLARED LIMIT: what milestone 5 does here is swap an object. When the
    /// CONTENT of an eviction arrives (L2), this class has to be looked at again -- a reload
    /// is not free to repeat.
    ///
    /// ⚠️ `Trust::Instruction`, and the payload is EMPTY: no external byte reaches this
    /// record. The label is about the payload (`Trust`'s own doc), and an empty payload that
    /// came from nowhere is ours.
    pub fn set_policy<J: Journal>(
        &mut self,
        policy: VramPolicy,
        step: StepId,
        journal: &mut J,
        now: Monotonic,
    ) -> Result<(), JournalError> {
        let _ = now;
        journal.intent(step, &transition_record(RecordKind::Intent, policy.name()))?;
        self.policy = policy;
        journal.outcome(
            step,
            &transition_record(RecordKind::Outcome, self.policy.name()),
        )
    }
```

e, accanto:

```rust
/// The durable record of a policy transition.
///
/// ⚠️ `reason` CARRIES THE NAME OF THE POLICY, and that is why `MakeRoom::name` exists: a
/// record that said only "policy transition" would make the two directions
/// indistinguishable in the archive, and the archive is the only thing that survives.
fn transition_record(kind: RecordKind, policy: &'static str) -> Vec<u8> {
    Record::V1(RecordV1 {
        kind,
        effect: EffectClass::Idempotent,
        trust: Trust::Instruction,
        payload: Vec::new(),
        reason: String::from(policy),
    })
    .encode()
}
```

⚠️ **`now` non è letto da `set_policy`**, e il `let _ = now;` è un segnaposto che **non deve
sopravvivere**: o l'argomento serve — e allora lo si usa — o **si toglie dalla firma**. ⛔ Un
parametro ignorato è la superficie morta che questo repository ha tolto a `Record::encode` e
rifiutato a `Ipc::accept`. **Chi esegue decide, e registra la scelta.**

- [ ] **Passo 4: verde, e le mutazioni**

Run: `cargo test --locked -p kernel --test arbiter_policy`
Expected: **PASS**, nove test.

| # | Mutazione | Sonda che deve morire |
|---|---|---|
| 1 | scambiare l'ordine — prima `outcome`, poi `intent` | `a_policy_transition_writes_its_intent_before_its_outcome` (e la porta risponde `OutOfOrder`) |
| 2 | assegnare `self.policy` **prima** di scrivere l'intento | `a_refused_intent_leaves_the_policy_where_it_was` |
| 3 | `EffectClass::Idempotent` diventa `Unrepeatable` | `a_transition_cut_between_intent_and_outcome_leaves_the_step_in_doubt` — la risoluzione diventa `SuspendAndAsk` |
| 4 | `set_policy` non scrive **nulla** e si limita ad assegnare | tre sonde su quattro |

⛔ **La 4 ne uccide tre, quindi si cerca la mutazione che ne lasci passare due:** scrivere
l'intento e **non** l'esito lascia verde
`a_refused_intent_leaves_the_policy_where_it_was` e rossa
`without_a_crash_a_transition_leaves_no_step_in_doubt`. Le sonde stanno su assi diversi.

- [ ] **Passo 5: il cancello, il registro, il commit**

Run: `bash scripts/gate.sh` → `GATE GREEN`.

```bash
git add crates/kernel docs/porta-di-qualita.md
git commit -m "feat(arbiter): la transizione di policy e un passo giornalato, intento prima dell'effetto"
```

---

# Parte 4 — il cablaggio, e ciò che il `Grant` sblocca

---

### Task 10: `daemon` cabla l'arbitro, il giornale e le due concessioni permanenti

**Files:**
- Modify: `crates/daemon/src/main.rs`
- Modify: `crates/daemon/Cargo.toml` (se `kernel` non è già fra le dipendenze dirette — **si verifica, non si assume**)
- Modify: `docs/porta-di-qualita.md`

**Interfaces:**
- Consumes: `Arbiter`, `VramPolicy`, `ResourceProfile`, `Parameters`;
  `platform::journal::{FileJournal, OpenError}`.
- Produces: `run_the_production_graph(journal_path: &Path) -> Result<(), StartupError>`.

⛔ **Il pre-controllo di questo compito, ed è **R7** di questo piano:** dando un
`FileJournal` a `daemon`, il test che già esiste comincia a **scrivere un file vero**. Un
percorso fisso in una cartella condivisa è il gotcha **#52** — su Windows la cancellazione
fallisce a file aperto, quindi il rosso esce **su Linux**.

- [ ] **Passo 1: il percorso diventa un argomento, e il test ne prende uno privato**

In `crates/daemon/src/main.rs`, la firma:

```rust
fn run_the_production_graph(journal_path: &Path) -> Result<(), StartupError>
```

e nel modulo di test:

```rust
    /// ⛔ A DIRECTORY OF ITS OWN PER CALL SITE, from `line!()`, and it is not caution: a
    /// fixed path in a shared directory is gotcha #52, measured at milestone 3. Windows
    /// refuses to delete a file that is open, so the removal FAILS SILENTLY there and the
    /// red comes out on Linux -- the project's second system.
    ///
    /// ⚠️ AND THE PREFIX IS DIFFERENT from the two benches of `platform`, because a line
    /// number is unique inside ONE file only and the binaries run together.
    fn private_dir_for_line(line: u32) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("daemon-production-graph-{line}"));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("a fresh directory for this call site");
        dir
    }

    #[test]
    fn the_production_graph_assembles_and_the_executor_runs_to_completion() {
        let dir = private_dir_for_line(line!());
        assert_eq!(run_the_production_graph(&dir.join("journal.redb")), Ok(()));
    }
```

⚠️ **`assert_eq!` pretende `PartialEq` e `Debug` su `StartupError`.** Se `OpenError` non li
ha, **si misura** invece di supporre: `grep -n "derive" crates/platform/src/journal.rs`. Dove
manca, la sonda passa a `assert!(… .is_ok())` e **la divergenza si registra**.

- [ ] **Passo 2: eseguire, e vedere il rosso**

Run: `cargo test --locked -p daemon`
Expected: **FAIL** — la firma non prende un percorso e `StartupError` non esiste.

- [ ] **Passo 3: il cablaggio**

```rust
use std::path::Path;

use kernel::arbiter::{
    Arbiter, ComputeClass, Mib, Preemption, RemotePolicy, ResourceProfile, VramPolicy,
};
use kernel::executor::{Executor, RunError, Sleep};
use kernel::parameters::Parameters;
use kernel::time::{Millis, Monotonic};
use platform::journal::{FileJournal, OpenError};
use platform::reactor::SystemReactor;
use platform::rng::SequentialRng;

/// How much VRAM this machine has. ⛔ A LITERAL RIGHT HERE, constraint 11 of §11: the
/// parameter store arrives later, and until it does the value has to be chosen SOMEWHERE.
/// What makes that acceptable is that it is WRITTEN DOWN -- the same number inside the
/// kernel would appear in no list and could not be made to vary at all (gotcha #28).
const TOTAL_VRAM: Mib = Mib::new(16_384);

/// The audio quota, and the presentation quota of ADR-0033.
///
/// ⛔ THEY ARE NOT SUBTRACTIONS, THEY ARE TWO PERMANENT GRANTS, and the difference is I2. A
/// quota subtracted from the budget WITHOUT A HOLDER leaves I2 false for that consumer --
/// "the subtraction is not an exemption" (ADR-0005, gotcha #4) -- whereas a grant HAS a
/// holder by construction. ADR-0033 says it in those words: "the core REQUESTS a permanent,
/// non-preemptible presentation grant at start-up".
const AUDIO_QUOTA: Mib = Mib::new(1_024);
const PRESENTATION_QUOTA: Mib = Mib::new(768);

/// ⛔ "PERMANENT" IS NOT A TYPE -- it is "nobody calls release". The window is saturated on
/// purpose: `Monotonic::saturating_add` does not wrap, so a deadline this far out never
/// arrives, and there is no special case inside the arbiter for a grant that never expires.
const FOR_EVER: Millis = Millis::new(u64::MAX);

#[derive(Debug)]
enum StartupError {
    Journal(OpenError),
    Run(RunError),
}

fn run_the_production_graph(journal_path: &Path) -> Result<(), StartupError> {
    let _journal = FileJournal::open(journal_path).map_err(StartupError::Journal)?;

    let mut arbiter = Arbiter::new(
        Parameters::new(EXECUTOR_TURN_LIMIT, TOTAL_VRAM),
        // ⛔ REMOTE is the default of ADR-0006, and reopening that turns a coordinated swap
        // from an exception into the normal case.
        VramPolicy::Remote(RemotePolicy),
    );

    // ⛔ THE ARBITER DOES NOT KNOW THESE ARE CALLED "audio" AND "presentation". It sees two
    // permanent grants like any other -- which is ADR-0001: no capability has privileged
    // access. Wiring the two names inside the arbiter would be two special cases in a
    // mechanism that has to be even-handed.
    let _audio = arbiter.admit(
        &ResourceProfile {
            name: "audio-reserved",
            reserved_vram: AUDIO_QUOTA,
            compute_class: ComputeClass::Realtime,
            preemption: Preemption::Never,
        },
        FOR_EVER,
        Monotonic::ORIGIN,
    );
    let _presentation = arbiter.admit(
        &ResourceProfile {
            name: "presentation-reserved",
            reserved_vram: PRESENTATION_QUOTA,
            compute_class: ComputeClass::Realtime,
            preemption: Preemption::Never,
        },
        FOR_EVER,
        Monotonic::ORIGIN,
    );

    let sleep = Sleep::new();
    let mut executor = Executor::new(
        SequentialRng::new(),
        SystemReactor::new(),
        Parameters::new(EXECUTOR_TURN_LIMIT, TOTAL_VRAM),
        &sleep,
    );

    executor.run().map_err(StartupError::Run)
}
```

⚠️ **`_journal` e i due `_`-prefissi:** il giornale e le due concessioni **non hanno ancora un
consumatore** in questo binario, ed è **il fatto che il compito consegna**, non un
segnaposto. ⛔ Se il compilatore li segnala, **non si mette un `#[allow]`** (vincolo globale
4): si trasforma il residuo in un'**asserzione**, cioè si aggiunge al test la verifica che le
due ammissioni siano `Granted` — che è comunque la cosa giusta da provare.

- [ ] **Passo 4: la sonda che dice che le due quote sono DAVVERO due concessioni**

Nel modulo di test di `daemon`, in aggiunta:

```rust
    /// ⛔ WHAT THIS BUYS THAT THE ASSEMBLY TEST DOES NOT: that the two quotas are HELD, not
    /// subtracted. An arbiter that had merely lowered its ceiling would pass the test above
    /// and leave I2 false for the two consumers -- gotcha #4, and it is the whole reason the
    /// design diverges from the letter of §5.1.
    ///
    /// ⚠️ IT REBUILDS THE ARBITER RATHER THAN REACHING INTO THE WIRING, and the cost is
    /// declared: the two constants are shared, the CALL is not, so a wiring that stopped
    /// asking for the two grants would leave this green. What holds THAT is the assertion
    /// added inside `run_the_production_graph` at step 3.
    #[test]
    fn the_two_reserved_quotas_are_held_by_grants_and_not_subtracted() {
        let mut arbiter = Arbiter::new(
            Parameters::new(EXECUTOR_TURN_LIMIT, TOTAL_VRAM),
            VramPolicy::Remote(RemotePolicy),
        );
        for (name, quota) in [
            ("audio-reserved", AUDIO_QUOTA),
            ("presentation-reserved", PRESENTATION_QUOTA),
        ] {
            let outcome = arbiter.admit(
                &ResourceProfile {
                    name,
                    reserved_vram: quota,
                    compute_class: ComputeClass::Realtime,
                    preemption: Preemption::Never,
                },
                FOR_EVER,
                Monotonic::ORIGIN,
            );
            assert!(matches!(outcome, kernel::arbiter::Admission::Granted(_)));
        }
        assert_eq!(
            arbiter.allocated(),
            AUDIO_QUOTA.saturating_add(PRESENTATION_QUOTA),
            "the quotas are SPOKEN FOR, which is what a subtraction would not show"
        );
    }
```

- [ ] **Passo 5: verde, le mutazioni, e la prova sui fine-riga**

Run: `cargo test --locked -p daemon`
Expected: **PASS**.

| # | Mutazione | Sonda che deve morire |
|---|---|---|
| 1 | togliere le due `admit` dal cablaggio | l'asserzione aggiunta al passo 3 |
| 2 | `TOTAL_VRAM` a `Mib::new(1_000)` | le due concessioni non entrano entrambe |

⛔ **E i fine-riga di `crates/daemon/src/main.rs` si misurano PRIMA e DOPO** —
`tr -cd '\r' < crates/daemon/src/main.rs | wc -c` — perché questo compito riscrive gran parte
del file. Un conteggio che cambia è una normalizzazione silenziosa, e `git diff`
dichiarerebbe righe cambiate che nessuno ha toccato.

- [ ] **Passo 6: il cancello, il registro, il commit**

Run: `bash scripts/gate.sh` → `GATE GREEN`.
Run: `git status --short`
Expected: **solo** i file che questo compito tocca. ⛔ Se compare un `journal.redb`, il test
sta scrivendo **dentro il repository** invece che nella cartella temporanea: si ferma e si
corregge, e si aggiunge la ragione al registro.

```bash
git add crates/daemon docs/porta-di-qualita.md
git commit -m "feat(daemon): il grafo di produzione monta l'arbitro, il giornale e le due concessioni permanenti"
```

---

### Task 11: i quattro casi di §6.10.5, che il `Grant` sblocca

**Files:**
- Create: `crates/kernel/tests/compile_fail/talking_without_the_handle.rs`
- Create: `crates/kernel/tests/compile_fail/instructing_after_the_kill.rs`
- Create: `crates/kernel/tests/compile_fail/reading_without_a_receipt.rs`
- Create: `crates/kernel/tests/compile_fail/reading_twice_from_one_receipt.rs`
- Create: `crates/kernel/tests/worker_tokens.rs` — le **contro-sonde**
- Modify: `docs/porta-di-qualita.md`

**Interfaces:**
- Consumes: `Grant` emesso da `Arbiter::admit`; la porta `process` così com'è.
- Produces: nessun tipo nuovo. ⛔ **Questo compito non aggiunge una riga di prodotto**: chiude
  quattro righe che aspettavano un emittente di concessioni dal Traguardo 2.

⛔ **Le righe che chiude:** blocco B *«parlare a un worker ← l'oggetto `Worker`»* e
*«leggere ← una ricevuta»*; blocco C `I2 · §6.10` e `I5 · §6.10`. Sono le **quattro righe di
§6.10.5** registrate come **scaglionate** in
[`porta-di-qualita.md`](../../porta-di-qualita.md).

⛔ **Il pre-controllo, ed è la quarta domanda:** *ciò che questo compito detta di produrre
esiste già?* Si **rilegge** `crates/kernel/tests/ports_are_implementable.rs`, che ha già una
finta di `Process` e una di `Worker` e le esercita **in entrambe le direzioni**. Ciò che lì
**non** c'è è una concessione **vera** — la finta di `Process::start` prende un `_grant` e non
lo usa. Se al momento di eseguire questo compito quel banco avesse già le contro-sonde,
**questo compito è già eseguito** e si riporta indietro invece di duplicare (gotcha **#49**).

- [ ] **Passo 1: le contro-sonde, con una concessione VERA**

`crates/kernel/tests/worker_tokens.rs`:

```rust
//! The counter-probes of the four rows of §6.10.5 -- the "must stay green" half. Until
//! milestone 5 they could not be written at all: all four need to OBTAIN a `Worker`, a
//! `Worker` comes only from `start(grant, ..)`, and nobody issued grants.
//!
//! ⛔ AND THE GRANT HERE IS A REAL ONE, from a real admission. A test-only constructor was
//! weighed and refused in `docs/porta-di-qualita.md`: it would create the SECOND way of
//! obtaining a grant that §5.6 exists to take away from the compiler. The bench goes through
//! the admission like everybody else.

use kernel::arbiter::{
    Admission, Arbiter, ComputeClass, Grant, Mib, Preemption, RemotePolicy, ResourceProfile,
    VramPolicy,
};
use kernel::parameters::Parameters;
use kernel::ports::process::{
    Frame, Process, ProcessError, SingleReceipt, StreamReceipt, Worker, WorkerDescriptor,
};
use kernel::time::{Millis, Monotonic};

/// A grant obtained the only way there is.
fn a_real_grant() -> Grant {
    let mut arbiter = Arbiter::new(
        Parameters::new(10_000, Mib::new(16_384)),
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
    grant
}

/// The smallest worker that answers. ⚠️ IT IS A FAKE AND IT IS ALLOWED TO BE POOR: what this
/// file tests is the SHAPE OF THE TOKENS, not a worker channel -- that is the conformance
/// suite, and it needs two implementations and a real worker (milestone 6).
struct FakeWorker {
    next: u64,
}

impl Worker for FakeWorker {
    fn instruct_one(&mut self, _frame: Frame) -> Result<SingleReceipt, ProcessError> {
        self.next += 1;
        Ok(SingleReceipt::new(self.next))
    }

    fn instruct_stream(&mut self, _frame: Frame) -> Result<StreamReceipt, ProcessError> {
        self.next += 1;
        Ok(StreamReceipt::new(self.next))
    }

    fn read_one(&mut self, receipt: SingleReceipt) -> Result<Frame, ProcessError> {
        Ok(Frame::new(receipt.id().to_le_bytes().to_vec()))
    }

    fn read_next(&mut self, _receipt: &mut StreamReceipt) -> Result<Option<Frame>, ProcessError> {
        Ok(None)
    }

    fn close(&mut self, _receipt: StreamReceipt) -> Result<(), ProcessError> {
        Ok(())
    }

    fn kill(self) -> Result<(), ProcessError> {
        Ok(())
    }
}

struct FakeProcess;

impl Process for FakeProcess {
    type Handle = FakeWorker;

    fn start(
        &mut self,
        _grant: Grant,
        _descriptor: WorkerDescriptor,
    ) -> Result<Self::Handle, ProcessError> {
        Ok(FakeWorker { next: 0 })
    }
}

fn a_started_worker() -> FakeWorker {
    FakeProcess
        .start(a_real_grant(), WorkerDescriptor::new(b"asr.exe".to_vec()))
        .expect("the fake always starts")
}

/// §6.10.5 row 1, counter-probe: WITH the handle, talking compiles and works.
#[test]
fn with_the_handle_the_worker_can_be_instructed() {
    let mut worker = a_started_worker();
    let receipt = worker
        .instruct_one(Frame::new(b"hello".to_vec()))
        .expect("the fake answers");
    assert_eq!(receipt.id(), 1);
}

/// §6.10.5 row 2, counter-probe: instructing BEFORE the kill compiles.
#[test]
fn instructing_before_the_kill_compiles() {
    let mut worker = a_started_worker();
    let _ = worker
        .instruct_one(Frame::new(b"hello".to_vec()))
        .expect("answered");
    worker.kill().expect("killing is always lawful");
}

/// §6.10.5 rows 3 and 4, counter-probe: reading ONCE, with the receipt, compiles.
#[test]
fn reading_once_with_the_receipt_compiles() {
    let mut worker = a_started_worker();
    let receipt = worker
        .instruct_one(Frame::new(b"hello".to_vec()))
        .expect("answered");
    let expected = receipt.id();
    let answer = worker.read_one(receipt).expect("answered");
    assert_eq!(answer.as_bytes(), &expected.to_le_bytes());
}

/// ⛔ AND THE ONE THAT SAYS THE GRANT IS SPENT: `start` CONSUMES it, so one grant starts one
/// worker. This is the runtime half of what `Grant`'s missing `Clone` holds at level 1.
#[test]
fn one_grant_starts_one_worker() {
    let grant = a_real_grant();
    let first = FakeProcess.start(grant, WorkerDescriptor::new(b"asr.exe".to_vec()));
    assert!(first.is_ok());
    // A second `start` would need a second grant: the first has been moved. That is held by
    // `compile_fail/`, not here -- a moved value cannot be written in a test that compiles.
}
```

- [ ] **Passo 2: eseguirle**

Run: `cargo test --locked -p kernel --test worker_tokens`
Expected: **PASS**, quattro test. ⛔ Se falliscono per un motivo diverso da un'asserzione —
per esempio perché `Grant` non è raggiungibile da fuori — si **ferma e si riporta**: sarebbe
un difetto del Task 4, non di questo.

- [ ] **Passo 3: i quattro casi negativi**

⛔ **Ciascuno dei quattro ripete VERBATIM il preambolo** — `a_real_grant`, `FakeWorker`,
`FakeProcess`, `a_started_worker` — dal file del passo 1, con `kernel::` al posto degli `use`.
⚠️ **La duplicazione è dichiarata e non evitabile:** `trybuild` compila ogni caso come una
**crate a sé**, e il codice di test non attraversa i confini di crate — la stessa ragione per
cui `Yield` è duplicato parola per parola fra `executor_determinism.rs` e `dst_campaign.rs`.

I quattro `main`:

`talking_without_the_handle.rs` — riga 1:

```rust
// §6.10.5 row 1: you talk to a worker ONLY with the object the start returned. What you have
// BEFORE starting -- the grant and the descriptor -- is not it.
fn main() {
    let descriptor = WorkerDescriptor::new(b"asr.exe".to_vec());
    let _ = descriptor.instruct_one(Frame::new(b"hello".to_vec()));
}
```

`instructing_after_the_kill.rs` — riga 2, e chiude `I2 · §6.10`:

```rust
// §6.10.5 row 2: `kill` CONSUMES the worker, so instructing it afterwards is a use after
// move -- `E0382`. I2 held by the compiler rather than by discipline.
fn main() {
    let mut worker = a_started_worker();
    worker.kill().expect("killing is always lawful");
    let _ = worker.instruct_one(Frame::new(b"too late".to_vec()));
}
```

`reading_without_a_receipt.rs` — riga 3:

```rust
// §6.10.5 row 3: reading demands a receipt. `read_one` takes one BY VALUE, so there is no
// way to ask for an answer nobody asked a question for.
fn main() {
    let mut worker = a_started_worker();
    let _ = worker.read_one();
}
```

`reading_twice_from_one_receipt.rs` — riga 4, e chiude `I5 · §6.10`:

```rust
// §6.10.5 row 4: a single receipt is CONSUMED by the read. Reading twice is a use after
// move -- `E0382` -- which is what makes "every byte that comes back is covered by a
// receipt" a shape rather than a promise.
fn main() {
    let mut worker = a_started_worker();
    let receipt = worker
        .instruct_one(Frame::new(b"hello".to_vec()))
        .expect("answered");
    let _first = worker.read_one(receipt);
    let _second = worker.read_one(receipt);
}
```

- [ ] **Passo 4: gli oracoli, letti**

`cargo test --locked -p kernel --test compile_fail` → **FAIL**, quattro oracoli mancanti.
`TRYBUILD=overwrite …`, **una volta**, poi `git diff --stat`: **quattro** `.stderr` nuovi.

⛔ **Letti uno per uno:** i due «dopo l'uccisione» e «due volte» devono dire **`E0382`**, uso
dopo lo spostamento; «senza la maniglia» **`E0599`**; «senza ricevuta» **`E0061`**, numero di
argomenti sbagliato.

⚠️ **Se uno dei quattro dicesse altro, il caso è sbagliato e non l'oracolo.** Un caso che
fallisce per il motivo sbagliato è **verde per il motivo sbagliato** dall'altra parte —
gotcha **#24**.

- [ ] **Passo 5: la seconda direzione**

⛔ **Le contro-sonde del passo 1 SONO la seconda direzione**, ed è il motivo per cui vengono
prima. Ma va provato che siano **portanti**: si cancellano i quattro test di
`worker_tokens.rs`, uno alla volta, e si guarda **che cosa** diventa rosso. ⚠️ Se cancellarne
uno **non** rende rosso nulla oltre al proprio conteggio, quel test **non prova nulla che gli
altri non provino già**, e va detto nel registro invece che lasciato.

- [ ] **Passo 6: il cancello, il registro, il commit**

Run: `bash scripts/gate.sh` → `GATE GREEN`.

Nel registro: le **quattro** righe di §6.10.5 escono dalla lista delle **scaglionate**, e la
riga del blocco B che diceva *«due su cinque scoperte perché senza `Grant` non si ottiene un
`Worker`»* si **riconta sul catalogo**, non si deduce.

```bash
git add crates/kernel/tests docs/porta-di-qualita.md
git commit -m "test(process): le quattro righe di 6.10.5 escono dallo scaglionamento"
```

---

# Parte 5 — la campagna, e la chiusura

---

### Task 12: la campagna DST dell'arbitro

**Files:**
- Create: `crates/simulator/tests/arbiter_campaign.rs`
- Modify: `docs/porta-di-qualita.md`
- Modify: `docs/riferimenti.md` (le misure di questo compito)
- Modify: `docs/semi-dst.md` **se e solo se** la campagna trova un difetto

**Interfaces:**
- Consumes: tutto ciò che precede, più `simulator::rng::SeededRng`,
  `simulator::reactor::VirtualReactor`, `simulator::journal::CrashingJournal`,
  `kernel::executor::{Executor, Sleep}`.
- Produces: nessun tipo. Produce **la prova**.

⛔ **La riga di catalogo che questo compito chiude:** quella di livello 2, §7.4.2, che nomina
`Q2 · I2 · V1` fra i propri difesi e porta già scritta la sonda — *«si rompe l'ammissione, la
campagna fallisce e nomina il seme»*.

⛔ **E la lezione che il Traguardo 4 ha imparato TRE VOLTE, ogni volta dopo aver chiuso la
precedente:** *«l'iniezione è avvenuta»* e *«c'era qualcosa da verificare»* sono **due**
affermazioni, e una campagna che tiene solo la prima è **verde avendo confrontato insiemi
vuoti**. Questa campagna porta **due oracoli distinti**, e il discrimine fra loro va
**provato**: le mutazioni che svuotano la campagna sparano sul secondo, quelle che spengono
il guasto sul primo.

- [ ] **Passo 1: lo scenario, e i due oracoli di non-vacuità**

`crates/simulator/tests/arbiter_campaign.rs`:

```rust
//! The DST campaign of the arbiter (§5.7). Three of the five properties: the two that need
//! `process` and `ipc` are milestone 6, and their ARBITER HALF -- "releasing gives back the
//! reservation" -- is held by `crates/kernel/tests/arbiter_admission.rs` instead.
//!
//! ⛔ THE ARBITER UNDER TEST IS THE REAL ONE. There is no fake: the arbiter is logic, not a
//! port, so in simulation the shipped object runs. The faults come from the ports it USES --
//! `reactor` for the interleaving and the virtual clock, `journal` for the crash.
//!
//! ⛔ AND NON-VACUITY IS MANDATORY HERE, §5.7.1 says so in those words. Two claims, two
//! oracles: THE INJECTION FIRED, and THERE WAS SOMETHING TO VERIFY.

use core::cell::RefCell;

use kernel::arbiter::{
    Admission, Arbiter, ComputeClass, LocalPolicy, Mib, Preemption, RemotePolicy,
    ResourceProfile, VramPolicy,
};
use kernel::executor::{Executor, Sleep};
use kernel::parameters::Parameters;
use kernel::ports::journal::StepId;
use kernel::reconcile::{Resolution, steps_in_doubt};
use kernel::time::{Millis, Monotonic};
use simulator::journal::CrashingJournal;
use simulator::reactor::VirtualReactor;
use simulator::rng::SeededRng;

const TURN_LIMIT: u64 = 10_000;
const ACTIVITIES: usize = 3;
const REQUESTS: usize = 4;
const TOTAL: Mib = Mib::new(8_192);

/// The reservation of every synthetic request. ⛔ IT IS CHOSEN SO THAT NOT EVERYTHING FITS:
/// three activities asking 3072 of 8192 means at most two are resident, so the campaign
/// really exercises refusal and queueing. A size that always fitted would sweep a world in
/// which the admission has nothing to decide -- gotcha #17.
const RESERVATION: Mib = Mib::new(3_072);

/// How many seeds the SHORT campaign sweeps. ⛔ FIXED AND VERSIONED WITH THIS FILE, never
/// drawn from the clock or from an environment variable: constraint 7 of §11, so two runs of
/// the gate sweep the same seeds.
///
/// ⚠️ THE NUMBER IS A STARTING POINT AND NOT A MEASURE, and saying so is the point. The
/// criterion this repository uses is NOT "the largest round number under the ceiling" -- that
/// chases a figure that SATURATES -- it is the CLOSURE OF THE OUTCOME SPACE: sweep until no
/// new distinct outcome appears, then take a margin. Step 4 measures it and this constant is
/// rewritten with what came out, together with the table that justifies it. A number left at
/// 2000 because the milestone 4 campaign used 2000 would be an inherited hypothesis.
const SHORT_CAMPAIGN_SEEDS: u64 = 2_000;

/// The step the policy transition is journalled under.
const TRANSITION_STEP: u64 = 900;

fn profile(name: &'static str, lane: ComputeClass) -> ResourceProfile {
    ResourceProfile {
        name,
        reserved_vram: RESERVATION,
        compute_class: lane,
        preemption: Preemption::After(Millis::new(500)),
    }
}

/// What one run of the scenario observed. ⛔ IT IS THE INDEPENDENT ORACLE, and it comes from
/// the ACTIVITIES rather than from the arbiter: the properties below walk the arbiter's own
/// books, this walks what the activities were told happened. An arbiter that lied about its
/// own totals would make the two disagree.
#[derive(Debug, Default, PartialEq, Eq)]
struct Observed {
    granted: usize,
    queued: usize,
    refused: usize,
    /// The largest `allocated()` seen at ANY point at which it was observable.
    peak: Mib,
}

/// The scenario: `ACTIVITIES` interleaved activities, each asking for a grant, sleeping on
/// the VIRTUAL clock, then releasing it -- `REQUESTS` times over.
///
/// ⛔ EVERY ACTIVITY OBSERVES `allocated()` AFTER ITS OWN OPERATION, and that is what makes
/// property 1 an assertion about EVERY POINT AT WHICH IT IS OBSERVABLE rather than about the
/// end of the run. A check only at the end is green for an arbiter that over-admits and then
/// tidies up.
fn run(seed: u64, journal: CrashingJournal) -> (CrashingJournal, Observed) {
    let arbiter = RefCell::new(Arbiter::new(
        Parameters::new(TURN_LIMIT, TOTAL),
        VramPolicy::Remote(RemotePolicy),
    ));
    let observed = RefCell::new(Observed::default());
    let journal = RefCell::new(journal);
    let sleep = Sleep::new();
    let mut executor = Executor::new(
        SeededRng::new(seed),
        VirtualReactor::new(),
        Parameters::new(TURN_LIMIT, TOTAL),
        &sleep,
    );

    for activity in 0..ACTIVITIES {
        let arbiter = &arbiter;
        let observed = &observed;
        let sleep = &sleep;
        executor.spawn(async move {
            for request in 0..REQUESTS {
                let now = Monotonic::from_millis(((request as u64) + 1) * 1_000);
                let lane = match activity % 3 {
                    0 => ComputeClass::Realtime,
                    1 => ComputeClass::Interactive,
                    _ => ComputeClass::Batch,
                };
                let outcome =
                    arbiter
                        .borrow_mut()
                        .admit(&profile("synthetic", lane), Millis::new(2_000), now);

                {
                    let mut observed = observed.borrow_mut();
                    match outcome {
                        Admission::Granted(_) => observed.granted += 1,
                        Admission::Queued(_) => observed.queued += 1,
                        Admission::Refused { .. } => observed.refused += 1,
                    }
                    let allocated = arbiter.borrow().allocated();
                    // ⛔ PROPERTY 1, ASSERTED WHERE IT IS OBSERVABLE. The seed is in the
                    // message: §5.7.1 wants the campaign to NAME THE SEED when it fails.
                    assert!(
                        allocated <= TOTAL,
                        "seed {seed}: allocated {allocated:?} exceeds the total {TOTAL:?}"
                    );
                    if allocated > observed.peak {
                        observed.peak = allocated;
                    }
                }

                sleep.until(Monotonic::from_millis(((request as u64) + 1) * 1_000));
                Yield::once().await;
            }
        });
    }

    executor.run().expect("the scenario terminates");
    drop(executor);

    // ⛔ THE JOURNALLED TRANSITION HAPPENS AFTER THE INTERLEAVING, and deliberately: it is a
    // decision of the composition root, not of an activity, and property 4 is about the
    // CRASH and not about the concurrency.
    let _ = arbiter.borrow_mut().set_policy(
        VramPolicy::Local(LocalPolicy),
        StepId::new(TRANSITION_STEP),
        &mut *journal.borrow_mut(),
        Monotonic::from_millis(100_000),
    );

    (journal.into_inner(), observed.into_inner())
}

/// A future that returns `Pending` exactly once.
///
/// ⚠️ DUPLICATED WORD FOR WORD from `dst_campaign.rs`, and it is declared rather than left to
/// be discovered: TEST CODE DOES NOT CROSS CRATE BOUNDARIES, so there is no place both
/// benches could reach. Unifying them would put a test helper on the wire of a shipped crate.
struct Yield(bool);

impl Yield {
    fn once() -> Self {
        Yield(false)
    }
}

impl core::future::Future for Yield {
    type Output = ();
    fn poll(
        mut self: core::pin::Pin<&mut Self>,
        _context: &mut core::task::Context<'_>,
    ) -> core::task::Poll<()> {
        if self.0 {
            core::task::Poll::Ready(())
        } else {
            self.0 = true;
            core::task::Poll::Pending
        }
    }
}
```

- [ ] **Passo 2: i due oracoli di non-vacuità, prima delle proprietà**

In coda allo stesso file:

```rust
/// ⛔ ORACLE ONE -- THE SCENARIO REALLY DOES WHAT THE CAMPAIGN ASSUMES. Without it, a
/// scenario whose requests all fitted would sweep a world where the admission never says no,
/// and every property below would be green having decided nothing. Gotcha #17.
#[test]
fn the_scenario_really_makes_the_admission_decide() {
    let (_journal, observed) = run(20_260_818, CrashingJournal::without_crash());

    assert_eq!(
        observed.granted + observed.queued + observed.refused,
        ACTIVITIES * REQUESTS,
        "every request got exactly one answer"
    );
    assert!(observed.granted > 0, "nothing was ever granted");
    assert!(
        observed.queued + observed.refused > 0,
        "everything fitted: the admission never had to say no, so this campaign proves nothing"
    );
    assert!(
        observed.peak > Mib::ZERO,
        "the books never moved: the arbiter was never asked anything"
    );
}

/// ⛔ ORACLE TWO -- THERE WAS SOMETHING TO VERIFY, and it is a DIFFERENT claim from oracle
/// one. This is the lesson milestone 4 learned three times, each time AFTER closing the
/// previous one: a campaign that holds only "the injection fired" is green having compared
/// empty sets.
///
/// ⚠️ WHAT IT MEASURES IS THE OUTCOME SPACE: how many DISTINCT `Observed` the sweep produces.
/// One distinct outcome across two thousand seeds means the interleaving changes nothing and
/// the campaign is one run repeated.
#[test]
fn the_campaign_sweeps_more_than_one_world() {
    let mut distinct = std::collections::BTreeSet::new();
    for seed in 0..SHORT_CAMPAIGN_SEEDS {
        let (_journal, observed) = run(seed, CrashingJournal::without_crash());
        distinct.insert((
            observed.granted,
            observed.queued,
            observed.refused,
            observed.peak.get(),
        ));
    }
    println!("arbiter campaign: {} distinct outcomes over {SHORT_CAMPAIGN_SEEDS} seeds", distinct.len());
    assert!(
        distinct.len() > 1,
        "every seed produced the SAME outcome: the interleaving is not reaching the arbiter, \
         so this campaign is one run repeated {SHORT_CAMPAIGN_SEEDS} times"
    );
}
```

- [ ] **Passo 3: le tre proprietà**

```rust
/// ⛔ PROPERTY 1 (§5.7): the sum of ALL grants never exceeds the total, at every point at
/// which it is observable. The assertion lives INSIDE `run`, fired after every operation of
/// every activity; this test is what sweeps the seeds and reports which one broke.
#[test]
fn property_1_the_sum_never_exceeds_the_total_on_any_seed() {
    let mut worlds = 0usize;
    for seed in 0..SHORT_CAMPAIGN_SEEDS {
        let (_journal, observed) = run(seed, CrashingJournal::without_crash());
        assert!(observed.peak <= TOTAL, "seed {seed}: peak {:?}", observed.peak);
        worlds += 1;
    }
    assert_eq!(worlds as u64, SHORT_CAMPAIGN_SEEDS);
}

/// ⛔ PROPERTY 5 (§5.7): an expired grant does not stay allocated. The injection is the
/// VIRTUAL CLOCK: the window is 2000 ms and the requests are 1000 ms apart, so grants expire
/// UNDER the scenario rather than in a constructed state.
///
/// ⚠️ THE ORACLE IS THAT SOMETHING WAS GRANTED AFTER THE BOOKS WERE FULL, which cannot happen
/// unless a collection took place. Asserting only "the peak is within the total" would be
/// green for an arbiter that granted once and then refused for ever.
#[test]
fn property_5_expiry_frees_the_budget_under_the_scenario() {
    let mut seeds_where_expiry_mattered = 0usize;
    for seed in 0..SHORT_CAMPAIGN_SEEDS {
        let (_journal, observed) = run(seed, CrashingJournal::without_crash());
        // More grants than the machine could hold at once means the budget was recycled.
        let simultaneous = TOTAL.get() / RESERVATION.get();
        if observed.granted as u64 > simultaneous {
            seeds_where_expiry_mattered += 1;
        }
    }
    assert!(
        seeds_where_expiry_mattered > 0,
        "on no seed were more grants issued than fit at once: nothing was ever recycled, so \
         the expiry is not being exercised at all"
    );
    println!("arbiter campaign: expiry recycled the budget on {seeds_where_expiry_mattered} seeds");
}

/// ⛔ PROPERTY 4 (§5.7): a policy transition cut in half leaves a step IN DOUBT, reconcilable
/// like every other. The injection is the `journal` port -- the crashing journal of milestone
/// 4 -- and the fall point is drawn from a generator DERIVED from the seed.
///
/// ⛔ TWO ORACLES AND NOT ONE: `crashes` says the injection fired, `doubted` says there was
/// something to compare. A run that fell BEFORE the intent leaves no doubt and no bug -- and
/// a campaign that counted only the falls would be green having compared empty sets.
#[test]
fn property_4_a_severed_transition_leaves_a_reconcilable_step() {
    let mut crashes = 0usize;
    let mut doubted = 0usize;

    for seed in 0..SHORT_CAMPAIGN_SEEDS {
        // ⛔ The fall point comes from a DERIVED seed and not from the same generator as the
        // interleaving: two `SeededRng` built from the same number give the SAME sequence, so
        // the campaign would explore a DIAGONAL of the space instead of the space (decision
        // D2 of the milestone 4 plan).
        let falls_at = crash_point(seed);
        let (journal, _observed) = run(seed, CrashingJournal::falling_at(falls_at));
        if !journal.has_fallen() {
            continue;
        }
        crashes += 1;

        let survivor = journal.into_survivor();
        let doubts = steps_in_doubt(&survivor).expect("the archive reads back");
        for doubt in &doubts {
            assert_eq!(
                doubt.step,
                StepId::new(TRANSITION_STEP),
                "seed {seed}: a step other than the transition is in doubt"
            );
            assert_eq!(
                doubt.resolution,
                Resolution::RunAgain,
                "seed {seed}: a policy transition is idempotent"
            );
        }
        if !doubts.is_empty() {
            doubted += 1;
        }
    }

    assert!(crashes > 0, "no seed crashed: the injection never fired");
    assert!(
        doubted > 0,
        "every crash fell before the intent: {crashes} crashes and ZERO doubts, so the \
         comparison was between empty sets on every one of them"
    );
    println!("arbiter campaign: {crashes} crashes, {doubted} with a step in doubt");
}

/// The fall point for a seed. ⛔ TWO WRITES PER TRANSITION -- the intent and the outcome -- so
/// the interesting point is 1: fell after the intent, before the outcome.
fn crash_point(seed: u64) -> u64 {
    // A derived mixing, so this generator is not the interleaving one.
    (seed.wrapping_mul(0xBF58_476D_1CE4_E5B9) >> 33) % 2
}
```

- [ ] **Passo 4: MISURARE il numero di semi invece di ereditarlo**

⛔ **Il criterio non è «il più grande multiplo di cento sotto il tetto»**, che insegue una
cifra che **satura**: è la **chiusura dello spazio degli esiti**. Nello scratchpad, con una
sonda usa-e-getta:

```
per seeds in 200, 500, 1000, 2000, 20000:
  quanti esiti distinti · a quale seme compare l'ultimo nuovo · quanti semi cadono · tempo di parete
```

Poi si sceglie un numero che **chiude lo spazio con margine**, e si scrive **la tabella**
accanto alla costante — come `SHORT_CAMPAIGN_SEEDS` di `dst_campaign.rs`.

⛔ **E si prova che il conteggio sia proprietà dello SCENARIO e non dei semi che lo
campionano** (gotcha #24): si rifà la misura con **tre costanti di mescolamento diverse** in
`crash_point`. Se il numero di esiti distinti è lo stesso, l'asserzione è un controllo; se
cambia, è una scommessa e va scritta come tale.

⛔ **E il budget di tempo si misura in `debug`**, non in `--release`: `gate.sh` lancia
`cargo test --workspace` senza `--release`, e al Traguardo 4 il fattore misurato fra i due era
**4,3**.

- [ ] **Passo 5: la NON-VACUITÀ di §5.7.1, eseguita e registrata**

⛔ **È la condizione 3 della definizione di «fatto», e non è facoltativa.**

| Passo | Atteso |
|---|---|
| si rompe deliberatamente l'ammissione — in `admit`, togliere il confronto `allocated().saturating_add(asked) > ceiling` | `property_1_the_sum_never_exceeds_the_total_on_any_seed` **fallisce**, e il messaggio **nomina il seme** |
| si ripristina | torna **verde** |

⛔ **E il messaggio si LEGGE**: deve contenere il numero del seme e i due valori. Un rosso
che dicesse solo *«assertion failed»* non soddisfa §5.7.1, che chiede alla campagna di
**nominare il seme**.

⛔ **Poi le altre tre mutazioni, ciascuna sul proprio oracolo**, per provare che i due oracoli
di non-vacuità **discriminano**:

| Mutazione | Chi deve sparare |
|---|---|
| `RESERVATION` a `Mib::new(1)` — tutto entra sempre | `the_scenario_really_makes_the_admission_decide` |
| `crash_point` restituisce sempre `0` — si cade prima dell'intento | il secondo oracolo di `property_4`, *«ZERO doubts»* |
| `CrashingJournal::without_crash()` al posto di quello cadente | il primo oracolo di `property_4`, *«no seed crashed»* |

⚠️ **Se una mutazione ne uccide due, si cerca la terza che ne lasci passare una** prima di
concludere che le sonde non distinguono i due difetti — gotcha **#55**, e la regola nata al
Task 3 del Traguardo 4.

- [ ] **Passo 6: il cancello, il tempo di parete, il commit**

Run: `bash scripts/gate.sh` → `GATE GREEN`.

⛔ **Il tempo di parete della campagna si STAMPA a ogni corsa** — vincolo 7 di §11 — con la
riga `println!` degli oracoli. E il cancello lo raccoglie: `scripts/gate.sh` lancia le
campagne con `--nocapture` proprio perché *«una campagna che non stampa niente è una
tentazione»*. ⛔ **Si verifica che la riga nuova compaia davvero nell'uscita del cancello**, e
se non compare **si aggiunge la campagna dell'arbitro all'elenco** dello script — che è una
modifica a `gate.sh`, quindi si **misura** che i fine-riga di quel file non cambino.

In [`docs/riferimenti.md`](../../riferimenti.md): la tabella dei semi, le tre costanti di
mescolamento, il tempo di parete, e i comandi. In
[`docs/porta-di-qualita.md`](../../porta-di-qualita.md): la riga di livello 2 passa a
**coperta**, con la sonda di non-vacuità **eseguita e datata**.

```bash
git add crates/simulator/tests docs/porta-di-qualita.md docs/riferimenti.md scripts
git commit -m "test(dst): la campagna dell'arbitro, tre proprieta e due oracoli di non-vacuita"
```

---

### Task 13: la chiusura, che è un AUDIT e non una scrittura

**Files:**
- Modify: `docs/COMPENDIO.md` (§6, §12)
- Modify: `docs/HANDOFF.md`
- Modify: `docs/roadmap.md`
- Modify: `docs/README.md`
- Modify: `docs/porta-di-qualita.md`
- Modify: `docs/riferimenti.md`
- Modify: `CLAUDE.md` **solo se** cambia il modo di lavorare

⛔ **Questo compito è un AUDIT, ed è il gotcha #49 alla terza occasione.** Ai Traguardi 2 e 3
il compito di chiusura dettava di **aggiungere** righe che i compiti precedenti avevano già
scritto a ogni passo: a mancare non era l'aggiunta, era il **riconteggio**. **Si parte dai
numeri, non dalle frasi.**

- [ ] **Passo 1: ricontare, col comando e non a memoria**

```bash
bash scripts/gate.sh
cargo test --workspace --no-fail-fast --locked 2>&1 | tail -5
ls crates/kernel/tests/compile_fail/*.rs | wc -l
grep -c "fn " crates/kernel/tests/arbiter_admission.rs
git status --short
```

⛔ **Ogni conteggio che questo traguardo ha mosso si riconta sul sorgente**, mai per
sottrazione dal valore precedente: gotcha **#31**, che su questi documenti è il difetto più
frequente del progetto.

- [ ] **Passo 2: le dodici righe di catalogo, contate SUL CATALOGO**

⛔ **Il disegno dice dodici. Si ricontano**, e la fonte è la §7.4 della spec — non questo
piano, non il disegno, non il registro:

| Blocco | Righe attese |
|---|---|
| §7.4.1 B, i gettoni | 3 — avviare · parlare · leggere |
| §7.4.1 C | 8 — `Q2 · §5.1` · `V2` · `V4` · `I2 · §5.3` · `Q8 · §5.2.1` · `V3` · `I2 · §6.10` · `I5 · §6.10` |
| §7.4.2, livello 2 | 1 — la campagna |

⚠️ **Se il conteggio vero diverge da dodici, VINCE IL CONTEGGIO** e la divergenza si scrive
nell'errata. Una cifra scritta prima della misura è un'ipotesi.

- [ ] **Passo 3: le voci aperte, raccolte in UN posto**

⛔ **Si raccolgono nel registro, in una tabella sola**, e non in sette riquadri sparsi:
quattro voci aperte sullo stesso oggetto sono il modo in cui una smette di esserlo senza che
nessuno l'abbia chiusa.

| Voce | Chi la chiude |
|---|---|
| **R1** — `WorkDescriptor` e `WorkerDescriptor` distano una lettera | il **proprietario**: un rinomino |
| **R3** — la riga `Q2 · §5.1` è una e in una direzione, i casi sono quattro | il **proprietario**: §7.4 è spec |
| **R4** — `Preemption` diverge dalla §5.2, che elenca due campi | il **proprietario** |
| **R5** — due celle del catalogo nominano identificatori **italiani** che ora esistono in inglese | il **proprietario**: §1.0 contro §7.4 |
| **R6** — `start` e `release` consumano entrambi il `Grant` | il **Traguardo 6**, e la via è scritta accanto a `Grant` |
| `Grant` ha un solo costruttore e **nulla lo controlla da dentro la crate** | il **proprietario**: riga di catalogo nuova |
| la contro-sonda di `Q8` nomina una **proiezione che non esiste** | il **proprietario**: riformulare la cella |
| la divergenza su §5.1 — un parametro consegnato invece di tre | il **proprietario** |
| le **dieci sonde permanenti** dell'audit, ancora senza riga di catalogo | il **proprietario**, dal 2026-08-18 |

- [ ] **Passo 4: il compendio, e il puntatore che vive in un posto solo**

In [`docs/COMPENDIO.md`](../../COMPENDIO.md):
- la **§6** incassa il traguardo, e il `⏭️` passa al **Traguardo 6**;
- la **§4** guadagna l'arbitro nello stack **solo se** ha prodotto una decisione di stack —
  altrimenti non si tocca;
- la **§12** riceve la propria misura, in **byte LF** e `int(n/1024 + 0.5)`, scritta **a
  passata chiusa** e con le righe contate **partendo dall'elenco dei file citati**;
- la riga del **disegno del Traguardo 5** si rilegge: diceva *«piano da scrivere»*.

⛔ **E il puntatore al prossimo passo NON si ricopia negli altri documenti.** `roadmap.md` e
`README.md` tengono lo **stato per traguardo** nelle proprie tabelle e **rimandano** alla §6
per il puntatore — è la regola del 2026-08-18, e la ragione per cui quella riga ha marcito
tre volte.

⛔ **Prima di dichiarare finito, il censimento:** `grep -rn "⏭️" docs/ CLAUDE.md` — e **ogni
riga trovata si legge INTERA**, senza troncarla. Gotcha **#70**, entrambe le forme: il `grep`
trova le **candidate**, non le case, e una riga scartata dopo centoventi caratteri è una casa
che il censimento **non ha guardato**.

- [ ] **Passo 5: `check-docs.sh`, poi il cancello, poi il commit**

```bash
bash scripts/check-docs.sh
bash scripts/gate.sh
```

Expected: verde ed `GATE GREEN`.

⚠️ **Le trappole di `check-docs.sh` si rileggono prima**, §10 del compendio: i conteggi
`<cifra> ADR`, la numerazione `####` per le sotto-sotto-sezioni, e le **due tabelle lette per
posizione** — §7.4 e §8.3/§8.4. ⛔ **Nessuna rinumerazione di sezioni**: lo script legge §7.4
e §8 **per posizione**.

```bash
git add docs CLAUDE.md
git commit -m "docs(traguardo-5): l'arbitro e eseguito, e i conteggi sono stati ricontati"
git push
```

---

## La definizione di «fatto» del Traguardo 5

⛔ **Sette condizioni, dalla §0.3 del disegno, più una che questo piano aggiunge.** ⚠️ **E un
criterio di chiusura invecchia come tutto il resto** — al Traguardo 2 la Definizione di
«fatto» pretendeva *«otto casi `compile_fail`»* dove erano quattordici — quindi si **rilegge
contro il codice** prima di usarla, non solo si applica.

| # | Condizione |
|---|---|
| 1 | `bash scripts/gate.sh` → `GATE GREEN` |
| 2 | le **tre** proprietà DST girano, ciascuna con la propria contro-sonda |
| 3 | la sonda di non-vacuità di §5.7.1 è **eseguita e registrata**: rotta l'ammissione la campagna fallisce e **nomina il seme**; ripristinata, torna verde |
| 4 | le **dodici** righe di catalogo sono chiuse **o dichiarate**, nessuna a metà — e ricontate **sul catalogo** |
| 5 | ogni caso in `crates/kernel/tests/compile_fail/` porta il proprio `.stderr` **letto** e non rigenerato in blocco |
| 6 | il registro è riallineato, coi conteggi **ricontati** |
| 7 | ogni riga scoperta della §9 del disegno ha il proprio indirizzo scritto |
| **8** | ⛔ **aggiunta da questo piano:** le voci aperte stanno in **una** tabella sola del registro, non sparse fra i riquadri dei compiti |

---

## Cosa questo piano lascia aperto

⚠️ **Nessuna di queste è un difetto oggi, e per ciascuna è scritto perché.** Sono le sette
voci del pre-controllo più le quattro della §12 del disegno, raccolte qui perché chi esegue
le abbia **prima** di scrivere e non trovandole.

| | Perché non è un difetto oggi | Chi la chiude |
|---|---|---|
| **R1** `WorkDescriptor` / `WorkerDescriptor` | moduli diversi, nessun conflitto di compilazione | il **proprietario**, con un rinomino |
| **R3** la riga `Q2 · §5.1` è una e in una direzione | i quattro casi esistono e mordono; a mancare è la **riga**, non il controllo | il **proprietario** — §7.4 è spec |
| **R4** `Preemption` contro la §5.2 | lo spirito di §5.3 punto 3 lo impone; la lettera di §5.2 è il prezzo | il **proprietario** |
| **R5** due celle del catalogo in italiano | oggi sono **prosa**; dal Task 4 diventano riferimenti al codice | il **proprietario** — §1.0 contro §7.4 |
| **R6** `start` e `release` consumano entrambi il `Grant` | nessuno chiama `start`: `process` non ha implementazione | il **Traguardo 6** |
| **R7** il giornale in `daemon` scrive un file vero | il rimedio è dettato dal Task 10 e ha un precedente in `platform` | chiuso dal Task 10 |
| `Grant` ha un solo costruttore, e nulla lo controlla **da dentro** la crate | `trybuild` prova la direzione da fuori, che è quella che conta oggi | il **proprietario** — riga di catalogo nuova |
| la contro-sonda di `Q8` nomina una proiezione inesistente | prova la proprietà giusta con parole diverse dalla cella | il **proprietario** |
| la divergenza su §5.1, un parametro invece di tre | le due quote **sono** due concessioni, quindi hanno un titolare | il **proprietario** |
| le **due metà** delle proprietà 2 e 3 della §5.7 | ciascuna metà ha il proprio indirizzo nella §9 del disegno | il **Traguardo 6** |
| la classe d'effetto della transizione di policy è `Idempotent` | oggi la transizione scambia un oggetto | **L2**, quando arriva il contenuto dello sfratto |
| [`semi-dst.md`](../../semi-dst.md) **non ha un chiudente** | eredità del Traguardo 4, non di questo | il **proprietario** |

---

## Se questo piano si rivela sbagliato

⛔ **Lo sarà, in almeno un compito su uno.** Il pre-controllo ha trovato un difetto in
**ventidue compiti su ventidue** fra i Traguardi 3 e 4, e questo piano è stato scritto **prima**
che una riga di arbitro esistesse.

| Cosa fare | Cosa non fare |
|---|---|
| **misurare**, decidere sul merito, e scrivere la voce nell'**errata in testa a questo file** | riscrivere il corpo del compito: il testo è il registro di ciò che fu deciso |
| se la divergenza cambia un **contratto di porta condivisa** o una **riga di catalogo**, **fermarsi e riportarla** | prenderla da soli: sono decisioni del proprietario, vincolo globale 7 |
| se un compito **è già eseguito**, dirlo e passare oltre | eseguirlo alla lettera duplicando — gotcha **#49** |

---

## Consegna dell'esecuzione

**Il piano è scritto.** Due modi di eseguirlo:

1. **Subagent-driven** *(raccomandata, ed è la modalità scelta dal proprietario)* — un
   subagente fresco per compito, con revisione fra uno e l'altro. Ha portato **dodici compiti
   su dodici** al Traguardo 3 e **dieci su dieci** al Traguardo 4.
2. **Inline** — i compiti si eseguono nella sessione corrente, con checkpoint di revisione.
